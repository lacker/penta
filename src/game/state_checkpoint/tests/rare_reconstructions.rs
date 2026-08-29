//! Reconstruction of objects and continuations caught mid-flight.
//!
//! Its sibling [`super::rare_states`] builds the payments and replacement
//! choices that have to fail closed; these are the states an ordinary game
//! passes through and a snapshot has to be able to express -- restricted
//! mana still unspent, a spell cast from a graveyard, a run of triggers
//! waiting to be ordered.

use super::super::*;
use super::rare_states::{
    answer_with_first_option, assert_reconstructs, cast_targeting, fill_mana, resolve_top_of_stack,
    staged_game, staged_modern_game,
};
use crate::card::cards;
use crate::game::DecisionContinuation;
use crate::game::tests::{
    activated_ability_for, card, cast_action, choose_decision_by_label, creature, mana_ability_for,
    pass_priority_pair, ready_game,
};
use crate::{Action, CounterKind, ManaColor, Target};

#[test]
fn a_mandatory_suspend_cast_offer_reconstructs() {
    let mut game = ready_game();
    let definition = game
        .catalog
        .find_by_name("Durkwood Baloth")
        .expect("Durkwood Baloth is cataloged");
    let mut baloth = card(10_900, definition, PlayerId::One);
    baloth.counters.add(CounterKind::named("time"), 1);
    let id = baloth.id;
    game.players[0].exile.push(baloth);

    game.remove_counters_from_object(Target::Card(id), CounterKind::named("time"), 1);
    for _ in 0..8 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        let player = game.priority;
        game.apply(player, Action::PassPriority)
            .expect("the Suspend trigger advances");
    }

    assert!(
        game.stack.is_empty(),
        "Suspend offer retained {} stack objects",
        game.stack.len()
    );
    assert!(game.pending_triggers.is_empty());

    let (_, rebuilt) =
        super::super::tests::rebuild_current_checkpoint(&game, PlayerId::One, 10_901);
    assert!(matches!(
        rebuilt.pending_decisions[0].continuation,
        DecisionContinuation::CastSuspended {
            card,
            ..
        } if card == id
    ));
    let actions = rebuilt.legal_actions(PlayerId::One);
    assert!(
        actions
            .iter()
            .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == id))
    );
    assert!(
        !actions
            .iter()
            .any(|action| matches!(action, Action::ChooseDecision { .. }))
    );
}

#[test]
fn granted_suspend_and_its_generated_trigger_reconstruct() {
    let mut game = ready_game();
    let jhoira_definition = game
        .catalog
        .find_by_name("Jhoira of the Ghitu")
        .expect("Jhoira is cataloged");
    let bolt_definition = game
        .catalog
        .find_by_name("Lightning Bolt")
        .expect("Lightning Bolt is cataloged");
    let jhoira = creature(10_910, jhoira_definition, PlayerId::One);
    let jhoira_id = jhoira.card.id;
    game.battlefield.push(jhoira);
    let bolt = card(10_911, bolt_definition, PlayerId::One);
    let bolt_id = bolt.id;
    game.players[0].hand.push(bolt);
    fill_mana(&mut game, PlayerId::One, 2);

    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility { source, cost_objects, .. }
                    if *source == jhoira_id && cost_objects == &[bolt_id]
            )
        })
        .expect("Jhoira can suspend the card");
    game.apply(PlayerId::One, activation).unwrap();
    resolve_top_of_stack(&mut game);

    let (_, mut rebuilt) =
        super::super::tests::rebuild_current_checkpoint(&game, PlayerId::One, 10_912);
    let suspended = rebuilt.players[0].exile[0].id;
    assert!(rebuilt.is_suspended(suspended));
    rebuilt.remove_counters_from_object(Target::Card(suspended), CounterKind::named("time"), 4);
    for _ in 0..8 {
        if !rebuilt.pending_decisions.is_empty() {
            break;
        }
        let player = rebuilt.priority;
        rebuilt
            .apply(player, Action::PassPriority)
            .expect("the granted Suspend trigger advances");
    }
    let (_, rebuilt) =
        super::super::tests::rebuild_current_checkpoint(&rebuilt, PlayerId::One, 10_913);
    assert!(matches!(
        rebuilt.pending_decisions[0].continuation,
        DecisionContinuation::CastSuspended { card, .. } if card == suspended
    ));
}

#[test]
fn chosen_counter_kind_reconstructs_before_the_operation_choice() {
    let mut game = ready_game();
    let clock = card(
        10_920,
        game.catalog
            .find_by_name("Clockspinning")
            .expect("Clockspinning is cataloged"),
        PlayerId::One,
    );
    let clock_id = clock.id;
    game.players[0].hand.push(clock);
    let mut target = creature(10_921, cards::SAVANNAH_LIONS, PlayerId::One);
    let target_id = target.card.id;
    target.add_counters(CounterKind::named("charge"), 2);
    game.battlefield.push(target);
    fill_mana(&mut game, PlayerId::One, 1);

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == clock_id
                    && choices
                        .iter_targets()
                        .copied()
                        .eq([Target::Permanent(target_id)])
            }
            _ => false,
        })
        .expect("Clockspinning can target the countered permanent");
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);
    choose_decision_by_label(&mut game, PlayerId::One, "Choose a charge counter");

    let (_, mut rebuilt) =
        super::super::tests::rebuild_current_checkpoint(&game, PlayerId::One, 10_922);
    choose_decision_by_label(
        &mut rebuilt,
        PlayerId::One,
        "Put another of the chosen counter",
    );
    assert_eq!(
        rebuilt
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == target_id)
            .expect("the target remains")
            .counters(CounterKind::named("charge")),
        3
    );
}

/// Mishra's Workshop pays for artifacts and nothing else. Unspent restricted
/// mana is the case where the public pool and the engine's units disagree in
/// meaning while agreeing in count, so the units have to travel.
#[test]
fn unspent_restricted_mana_reconstructs() {
    let mut game = staged_game();
    let workshop_id = GameObjectId(10_000);
    game.battlefield.push(creature(
        workshop_id.0,
        crate::card::cards::MISHRA_S_WORKSHOP,
        PlayerId::One,
    ));

    let ability = mana_ability_for(&game, workshop_id, ManaColor::Colorless);
    game.apply(
        PlayerId::One,
        Action::ActivateManaAbility {
            source: workshop_id,
            ability,
            color: ManaColor::Colorless,
            counters_removed: None,
            cost_object: None,
            combination: None,
            triggered_mana: None,
        },
    )
    .expect("the Workshop taps for mana");

    let restricted = game.players[PlayerId::One.index()]
        .mana
        .iter()
        .filter(|mana| !mana.restrictions.is_empty())
        .count();
    assert_eq!(restricted, 3, "the Workshop makes three restricted mana");
    assert_reconstructs(&game, "an unspent pool of restricted mana");
}

/// A flashback spell is on the stack with an alternative cost already paid and
/// a graveyard exile owed to it after it resolves. The stack object therefore
/// carries state its printed card does not.
#[test]
fn a_spell_cast_from_a_graveyard_reconstructs_while_it_is_on_the_stack() {
    let mut game = staged_modern_game();
    let spell = card(20_000, crate::card::cards::THINK_TWICE, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].graveyard.push(spell);
    fill_mana(&mut game, PlayerId::One, 4);

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell { card, choices, .. }
                    if *card == spell_id && choices.costs().alternative().is_some()
            )
        })
        .expect("Think Twice can be flashed back from the graveyard");
    game.apply(PlayerId::One, action)
        .expect("the flashback cast is legal");

    assert!(
        game.stack
            .last()
            .is_some_and(|object| object.cast_via_flashback),
        "the spell must be marked as cast via flashback"
    );
    assert_reconstructs(&game, "a flashback spell on the stack");
}

#[test]
fn a_spell_reconstructs_with_its_retired_additional_cost_object() {
    let mut game = staged_modern_game();
    let lunge = card(20_100, cards::CORPSE_LUNGE, PlayerId::One);
    let lunge_id = lunge.id;
    game.players[0].hand.push(lunge);
    let fodder = card(20_101, cards::SERRA_ANGEL, PlayerId::One);
    let fodder_id = fodder.id;
    game.players[0].graveyard.push(fodder);
    let victim = creature(20_102, cards::AIR_ELEMENTAL, PlayerId::Two);
    let victim_id = victim.card.id;
    game.battlefield.push(victim);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);

    game.apply(
        PlayerId::One,
        cast_action(
            lunge_id,
            vec![Target::Permanent(victim_id)],
            vec![fodder_id],
            0,
        ),
    )
    .expect("Corpse Lunge can spend the creature card");

    assert_reconstructs(
        &game,
        "a spell retaining its additional cost object's last-known information",
    );
}

#[test]
fn a_phyrexian_life_payment_reconstructs_and_is_not_copied() {
    let mut game = ready_game();
    let tamiyo = card(
        20_100,
        crate::card::cards::TAMIYO_COMPLEATED_SAGE,
        PlayerId::One,
    );
    let tamiyo_id = tamiyo.id;
    game.players[PlayerId::One.index()].hand.push(tamiyo);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell { card, choices, .. }
                    if *card == tamiyo_id && !choices.mana_payment().alternatives().is_empty()
            )
        })
        .expect("Tamiyo can be cast with life for the Phyrexian symbol");
    game.apply(PlayerId::One, action)
        .expect("the Phyrexian-life cast is legal");
    let original = game.stack.last().expect("Tamiyo is on the stack").clone();
    assert_eq!(original.phyrexian_symbols_paid_with_life, 1);
    assert_reconstructs(&game, "a Phyrexian-life spell on the stack");

    game.push_copy_with_colors(original, PlayerId::One, Vec::new(), None);
    assert!(
        game.stack
            .last()
            .is_some_and(|copy| copy.is_copy && copy.phyrexian_symbols_paid_with_life == 0),
        "a spell copy inherits copiable choices but not its original payment",
    );
}

/// Fork puts a copy of a spell on the stack and repaints it red. The copy is
/// backed by no card in any zone and its color no longer matches its printed
/// face, so both the copy flag and the override have to survive.
#[test]
fn a_copied_and_recolored_spell_reconstructs_on_the_stack() {
    let mut game = staged_game();
    let bolt = card(20_000, crate::card::cards::LIGHTNING_BOLT, PlayerId::One);
    let bolt_id = bolt.id;
    game.players[PlayerId::One.index()].hand.push(bolt);
    let fork = card(20_001, crate::card::cards::FORK, PlayerId::One);
    let fork_id = fork.id;
    game.players[PlayerId::One.index()].hand.push(fork);
    fill_mana(&mut game, PlayerId::One, 4);

    cast_targeting(
        &mut game,
        PlayerId::One,
        bolt_id,
        Target::Player(PlayerId::Two),
    );
    let spell_on_stack = game.stack.last().expect("the Bolt is on the stack").id;
    cast_targeting(
        &mut game,
        PlayerId::One,
        fork_id,
        Target::Spell(spell_on_stack),
    );
    resolve_top_of_stack(&mut game);
    while game.pending_decisions.first().is_some_and(|pending| {
        matches!(
            pending.continuation,
            DecisionContinuation::CopyStackObject { .. }
        )
    }) {
        answer_with_first_option(&mut game);
    }

    assert!(
        game.stack.iter().any(|object| object.is_copy),
        "Fork must have left a copy on the stack"
    );
    assert!(
        game.stack
            .iter()
            .any(|object| object.is_copy && object.colors.is_some()),
        "the copy must carry Fork's color override"
    );
    assert_reconstructs(&game, "a copied and recolored spell");
}

/// Triggers that have fired but not yet been placed live in the game itself,
/// with the context each captured when it fired. The broad audit never sees
/// this, because placement normally follows capture without a boundary in
/// between, so the `pendingTriggers` half of the snapshot is only exercised
/// from a position built on purpose.
#[test]
fn triggers_that_have_fired_but_not_yet_been_placed_reconstruct() {
    let mut game = staged_game();
    for id in 10_000..10_002 {
        let mut vault = creature(id, crate::card::cards::MANA_VAULT, PlayerId::One);
        vault.tapped = true;
        game.battlefield.push(vault);
    }
    game.step = crate::Step::Upkeep;
    game.handle_upkeep_triggers();

    assert_eq!(
        game.pending_triggers.len(),
        2,
        "both vaults must have captured a trigger"
    );
    assert_reconstructs(&game, "triggers captured but not yet placed");
}

/// Simultaneous triggers wait, unordered, in a decision that owns them. They
/// are on no stack and belong to no object, and each carries the context it
/// captured when it fired, so the ordering decision is where a snapshot has
/// the most to lose.
#[test]
fn simultaneous_triggers_waiting_to_be_ordered_reconstruct() {
    let mut game = staged_game();
    for id in 10_000..10_002 {
        let mut vault = creature(id, crate::card::cards::MANA_VAULT, PlayerId::One);
        vault.tapped = true;
        game.battlefield.push(vault);
    }
    game.step = crate::Step::Upkeep;
    game.handle_upkeep_triggers();
    game.finish_rules_procedure();

    assert!(
        matches!(
            game.pending_decisions
                .first()
                .map(|pending| &pending.continuation),
            Some(DecisionContinuation::TriggerOrder { .. })
        ),
        "two upkeep triggers must ask for an order, not {:?}",
        game.pending_decisions
            .first()
            .map(|pending| &pending.continuation)
    );
    assert_reconstructs(&game, "simultaneous triggers awaiting an order");
}

/// A text change rewrites a printed word for the rest of the game. It is
/// neither a characteristic nor an effect on a stack, and the permanent that
/// carries it has to come back reading the same way.
#[test]
fn an_indefinite_text_change_reconstructs_while_choosing_and_after() {
    let mut game = staged_game();
    let land_id = GameObjectId(12_000);
    game.battlefield.push(creature(
        land_id.0,
        crate::card::cards::PLATEAU,
        PlayerId::Two,
    ));
    let hack = card(11_000, crate::card::cards::MAGICAL_HACK, PlayerId::One);
    let hack_id = hack.id;
    game.players[PlayerId::One.index()].hand.push(hack);
    fill_mana(&mut game, PlayerId::One, 4);

    cast_targeting(
        &mut game,
        PlayerId::One,
        hack_id,
        Target::Permanent(land_id),
    );
    resolve_top_of_stack(&mut game);

    assert!(
        matches!(
            game.pending_decisions
                .first()
                .map(|pending| &pending.continuation),
            Some(DecisionContinuation::BasicLandTypeTextChange { .. })
        ),
        "Magical Hack must ask which word to rewrite, not {:?}",
        game.pending_decisions
            .first()
            .map(|pending| &pending.continuation)
    );
    assert_reconstructs(&game, "a text change choosing its words");

    answer_with_first_option(&mut game);
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| !permanent.text_changes.is_empty()),
        "the choice must leave a rewritten permanent behind"
    );
    assert_reconstructs(&game, "a permanent carrying an indefinite text change");
}

/// A phased-out permanent is public information, so it survives a checkpoint
/// the same way anything else on the board does -- and comes back phased
/// out rather than as an ordinary permanent.
#[test]
fn a_phased_out_permanent_reconstructs_as_phased_out() {
    let mut game = ready_game();
    game.battlefield.push(creature(
        10_000,
        crate::card::cards::BLACK_VISE,
        PlayerId::One,
    ));
    let vise = game.battlefield[0].card.id;
    game.phase_out(vise);
    assert!(game.battlefield.is_empty(), "it left the battlefield");

    let (_wire, rebuilt) = super::super::tests::rebuild_current_checkpoint(&game, PlayerId::One, 7);
    assert!(
        rebuilt.battlefield.is_empty(),
        "and the rebuilt game does not have it on the battlefield either",
    );
    assert_eq!(
        rebuilt
            .phased_out
            .iter()
            .map(|permanent| permanent.card.definition)
            .collect::<Vec<_>>(),
        vec![crate::card::cards::BLACK_VISE],
        "it came back waiting to phase in",
    );
}

/// A Dreadnought's entry cost is answered one creature at a time, and the
/// resolution that wants it is still in flight while it is asked -- so the
/// checkpoint has to carry that resolution along with how much is owed.
#[test]
fn a_run_of_sacrifices_reconstructs_mid_payment() {
    let mut game = ready_game();
    for index in 0..3 {
        game.battlefield.push(creature(
            10_010 + index,
            crate::card::cards::SERRA_ANGEL,
            PlayerId::One,
        ));
    }
    let dreadnought = card(
        10_000,
        crate::card::cards::PHYREXIAN_DREADNOUGHT,
        PlayerId::One,
    );
    let dreadnought_id = dreadnought.id;
    game.players[PlayerId::One.index()].hand.push(dreadnought);
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;
    game.priority = PlayerId::One;
    game.apply(
        PlayerId::One,
        crate::game::tests::cast_action(dreadnought_id, Vec::new(), Vec::new(), 0),
    )
    .expect("one mana casts it");
    crate::game::tests::pass_until_decision(&mut game);

    let offer = game
        .observe(PlayerId::One)
        .decision
        .expect("the payer is asked whether to pay");
    let pay = offer
        .options
        .iter()
        .find(|option| option.id != 0)
        .expect("paying is on offer")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: offer.id,
            options: vec![pay],
        },
    )
    .expect("paying is legal");

    let (_wire, rebuilt) =
        super::super::tests::rebuild_current_checkpoint(&game, PlayerId::One, 11);
    let remaining = match rebuilt
        .pending_decisions
        .first()
        .map(|pending| &pending.continuation)
    {
        Some(DecisionContinuation::SacrificeToTotalPower { remaining, .. }) => *remaining,
        other => panic!("the run of sacrifices came back as {other:?}"),
    };
    assert_eq!(remaining, 12, "and it still owes the whole twelve");
}

/// A trigger that divides a fixed total asks twice: which targets, and then
/// how much each takes. A game saved between those two questions has to come
/// back asking the second one, with the same splits on offer.
#[test]
fn a_pending_trigger_division_reconstructs_and_resumes() {
    let mut game = staged_game();
    game.battlefield.clear();
    game.battlefield
        .push(creature(30_000, cards::SERRA_ANGEL, PlayerId::Two));
    game.battlefield
        .push(creature(30_001, cards::SERRA_ANGEL, PlayerId::Two));
    game.put_onto_battlefield(PlayerId::One, cards::FURY)
        .expect("Fury is cataloged");

    let targets = loop {
        if let Some(decision) = game.observe(PlayerId::One).decision {
            break decision;
        }
        let player = game.priority;
        game.apply(player, Action::PassPriority)
            .expect("the enters trigger is waiting on its targets");
    };
    let both = targets
        .options
        .iter()
        .filter(|option| option.label == "Serra Angel")
        .map(|option| option.id)
        .collect::<Vec<_>>();
    assert_eq!(both.len(), 2);
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: targets.id,
            options: both,
        },
    )
    .expect("naming two targets is legal");

    assert!(
        game.observe(PlayerId::One)
            .decision
            .is_some_and(|decision| decision.prompt.contains("divide")),
        "the division is what is pending",
    );
    assert_reconstructs(&game, "a pending trigger division");
}

/// A vote half cast. The tally so far and who has yet to vote live only in
/// the continuation, so a snapshot that loses either would resume a
/// different election.
#[test]
fn a_vote_reconstructs_between_the_two_ballots() {
    let mut game = staged_game();
    game.battlefield.clear();
    game.battlefield
        .push(creature(31_000, cards::SERRA_ANGEL, PlayerId::Two));
    game.battlefield
        .push(creature(31_001, cards::GRIZZLY_BEARS, PlayerId::Two));
    fill_mana(&mut game, PlayerId::One, 3);
    let judgment = card(31_002, cards::COUNCILS_JUDGMENT, PlayerId::One);
    let judgment_id = judgment.id;
    game.players[PlayerId::One.index()].hand.push(judgment);

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == judgment_id))
        .expect("Council's Judgment is castable");
    game.apply(PlayerId::One, cast).expect("it is cast");
    let ballot = loop {
        if let Some(decision) = game.observe(PlayerId::One).decision {
            break decision;
        }
        let player = game.priority;
        game.apply(player, Action::PassPriority)
            .expect("the spell is waiting to be voted on");
    };
    let angel = ballot
        .options
        .iter()
        .find(|option| option.label == "Serra Angel")
        .expect("the Angel is on the ballot");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: ballot.id,
            options: vec![angel.id],
        },
    )
    .expect("the first vote is cast");

    assert!(
        game.observe(PlayerId::Two)
            .decision
            .is_some_and(|decision| decision.prompt.contains("Vote")),
        "the second ballot is what is pending",
    );
    assert_reconstructs(&game, "a vote with one ballot still to come");
}

/// A Doomsday search half answered. What the search looked at is fixed
/// before anybody answers, so a snapshot that lost it would exile a
/// different set of cards.
#[test]
fn a_multi_zone_search_reconstructs_before_it_is_answered() {
    let mut game = staged_game();
    game.battlefield.clear();
    game.players[PlayerId::One.index()].library.clear();
    game.players[PlayerId::One.index()].graveyard.clear();
    for id in 32_000..32_008 {
        game.players[PlayerId::One.index()].library.push(card(
            id,
            cards::GRIZZLY_BEARS,
            PlayerId::One,
        ));
    }
    game.players[PlayerId::One.index()].graveyard.push(card(
        32_020,
        cards::BLACK_LOTUS,
        PlayerId::One,
    ));
    fill_mana(&mut game, PlayerId::One, 3);
    let doomsday = card(32_030, cards::DOOMSDAY, PlayerId::One);
    let doomsday_id = doomsday.id;
    game.players[PlayerId::One.index()].hand.push(doomsday);

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == doomsday_id))
        .expect("Doomsday is castable");
    game.apply(PlayerId::One, cast).expect("it is cast");
    loop {
        if game.observe(PlayerId::One).decision.is_some() {
            break;
        }
        let player = game.priority;
        game.apply(player, Action::PassPriority)
            .expect("the spell is waiting on its search");
    }

    assert_reconstructs(&game, "a multi-zone search waiting to be answered");
}

/// A Breach choice caught before it is answered. What the creature arrives
/// carrying travels with the resolution rather than with the choice, so a
/// snapshot that lost it would put down a permanent with neither haste nor
/// the clause that takes it away again.
#[test]
fn a_choice_that_puts_a_permanent_onto_the_battlefield_reconstructs_unanswered() {
    let mut game = staged_game();
    game.battlefield.clear();
    game.players[PlayerId::One.index()].hand.clear();
    game.players[PlayerId::One.index()]
        .hand
        .push(card(33_000, cards::SERRA_ANGEL, PlayerId::One));
    let breach = card(33_001, cards::THROUGH_THE_BREACH, PlayerId::One);
    let breach_id = breach.id;
    game.players[PlayerId::One.index()].hand.push(breach);
    fill_mana(&mut game, PlayerId::One, 5);

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == breach_id))
        .expect("Through the Breach is castable");
    game.apply(PlayerId::One, cast).expect("it is cast");
    while game.observe(PlayerId::One).decision.is_none() {
        let player = game.priority;
        game.apply(player, Action::PassPriority)
            .expect("the spell is waiting to be answered");
    }

    assert!(
        matches!(
            game.pending_decisions
                .first()
                .map(|pending| &pending.continuation),
            Some(DecisionContinuation::ChooseCards {
                arrival: Some(_),
                ..
            })
        ),
        "the pending choice carries the resolution its arrival belongs to",
    );
    assert_reconstructs(&game, "a creature chosen for the battlefield, unanswered");
}

/// A run of colour choices caught between its two answers. The mana already
/// added is in the pool; what has to survive is how many are still owed and
/// what they will be made of.
#[test]
fn a_run_of_chosen_color_mana_reconstructs_between_answers() {
    let mut game = staged_game();
    game.battlefield.clear();
    game.players[PlayerId::Two.index()]
        .counters
        .set(CounterKind::named("charge"), 2);
    let mut suspended = card(98_765, cards::REALITY_STROBE, PlayerId::Two);
    suspended.add_counters(CounterKind::named("time"), 3);
    game.players[PlayerId::Two.index()].exile.push(suspended);
    let relic = game
        .put_onto_battlefield(PlayerId::One, cards::COALITION_RELIC)
        .expect("Coalition Relic is cataloged");
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == relic)
    {
        permanent
            .counters
            .set(crate::game::CounterKind::named("charge"), 3);
    }

    game.capture_battlefield_triggers(&crate::game::CommittedTriggerEvent::StepBegins {
        step: crate::card::TurnStepDef::PrecombatMain,
        player: PlayerId::One,
    });
    game.finish_rules_procedure();
    while game.observe(PlayerId::One).decision.is_none() {
        let player = game.priority;
        game.apply(player, Action::PassPriority)
            .expect("the trigger is waiting to be answered");
    }
    answer_with_first_option(&mut game);

    assert!(
        matches!(
            game.pending_decisions
                .first()
                .map(|pending| &pending.continuation),
            Some(DecisionContinuation::ChosenColorMana { remaining: 2, .. })
        ),
        "two of the three are still owed",
    );
    assert_reconstructs(&game, "a colour run with two answers still to come");
}

/// A standing offer to cast an exiled card is a decision whose answer is a
/// cast rather than a selection, so the rebuilt position has to carry both
/// the clause that made the offer and the card it points at.
#[test]
fn a_standing_cast_offer_reconstructs() {
    let mut game = staged_game();
    game.battlefield.clear();
    game.players[PlayerId::One.index()].library.clear();
    let chandra = game
        .put_onto_battlefield(PlayerId::One, cards::CHANDRA_TORCH_OF_DEFIANCE)
        .expect("Chandra is cataloged");
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == chandra)
    {
        permanent.entered_controller_turn = 0;
    }
    let bolt = game
        .build_zone(PlayerId::One, &[cards::LIGHTNING_BOLT])
        .expect("Lightning Bolt is cataloged")
        .into_iter()
        .next()
        .expect("one card");
    game.players[PlayerId::One.index()].library.push(bolt);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);

    let ability = activated_ability_for(&game, chandra, 0);
    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: chandra,
            ability,
            targets: Vec::new(),
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
            mana_payment: None,
        },
    )
    .expect("the loyalty ability activates");
    while game.pending_decisions.is_empty() && !game.stack.is_empty() {
        let player = game.priority;
        game.apply(player, Action::PassPriority)
            .expect("the ability is waiting to resolve");
    }

    assert!(
        matches!(
            game.pending_decisions
                .first()
                .map(|pending| &pending.continuation),
            Some(DecisionContinuation::MayCastExiled { .. })
        ),
        "the offer is what the position is waiting on",
    );
    assert_reconstructs(&game, "an offer to cast the exiled top card");
}
