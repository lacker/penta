//! Reconstruction at the boundaries sampled play almost never reaches.
//!
//! The broad audit walks hundreds of thousands of decision boundaries, but a
//! census of what it actually touched shows the tail it cannot reach: a
//! seven-mana pile-splitter, a planeswalker ultimate, an entering copy. Those
//! are exactly the continuations most likely to carry state a snapshot cannot
//! express, so each one is built here on purpose rather than waited for.

use super::super::*;
use super::{determinized, true_hidden_hypothesis};
use crate::game::DecisionContinuation;
use crate::game::tests::{card, creature, mana_ability_for, ready_game};
use crate::{Action, CardDefinitionId, CounterKind, ManaColor, TargetSelection};
use serde_json::Value;

/// The whole reconstruction contract at one boundary: the engine claims the
/// state is representable, the true hidden zones rebuild it, and so does a
/// hypothesis that disagrees with them. The last part is the point of the
/// exercise -- a search bot never knows the truth, so a reconstruction that
/// only works when handed the real hidden zones is not a reconstruction.
fn assert_reconstructs(game: &Game, label: &str) {
    let viewer = game
        .decision_player()
        .unwrap_or_else(|| panic!("{label}: the position must await an action"));
    let observation = game.observe(viewer);
    let actions = crate::protocol::protocol_actions(&observation);
    let wire = crate::protocol::observation_json_for_format(
        &game.catalog,
        game.format,
        &observation,
        game.in_pregame(),
        &actions,
    );
    assert_eq!(
        wire["checkpoint"]["hasDeferredState"],
        Value::Bool(false),
        "{label}: the engine deferred a state it is expected to represent",
    );

    let truth = true_hidden_hypothesis(game, viewer);
    for (kind, hidden) in [
        ("the true hidden zones", truth.clone()),
        (
            "a hypothesis that disagrees with the host",
            determinized(&truth, viewer),
        ),
    ] {
        let rebuilt = Game::from_observation_checkpoint(
            game.catalog.clone(),
            game.format,
            &wire,
            &hidden,
            4_242,
        )
        .unwrap_or_else(|error| panic!("{label}: {kind} failed to rebuild: {error}"));
        let rebuilt_observation = rebuilt.observe(viewer);
        let rebuilt_actions = crate::protocol::protocol_actions(&rebuilt_observation);
        assert_eq!(
            rebuilt_actions, actions,
            "{label}: {kind} changed the actions"
        );
        assert_eq!(
            crate::protocol::observation_json_for_format(
                &rebuilt.catalog,
                rebuilt.format,
                &rebuilt_observation,
                rebuilt.in_pregame(),
                &rebuilt_actions,
            ),
            wire,
            "{label}: {kind} changed the public observation",
        );
    }
}

fn rebuild_from_truth(game: &Game, seed: u64) -> Game {
    let viewer = game
        .decision_player()
        .expect("the position must await an action");
    let observation = game.observe(viewer);
    let actions = crate::protocol::protocol_actions(&observation);
    let wire = crate::protocol::observation_json_for_format(
        &game.catalog,
        game.format,
        &observation,
        game.in_pregame(),
        &actions,
    );
    Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &wire,
        &true_hidden_hypothesis(game, viewer),
        seed,
    )
    .expect("the typed decision state reconstructs")
}

/// `ready_game` clears the board and hands but keeps both libraries stocked,
/// which is what these positions want: a known board and real hidden zones to
/// hypothesize about.
fn staged_game() -> Game {
    ready_game()
}

/// The same board under the modern format, for the cards that only exist
/// there. Only the rules profile differs; reconstruction reads the format
/// from the same observation it reads everything else from.
fn staged_modern_game() -> Game {
    let mut game = ready_game();
    game.format = crate::Format::IsdRtrStandard;
    game
}

#[test]
fn a_begin_turn_replacement_with_a_deferred_effect_reconstructs_and_resumes() {
    let mut game = staged_game();
    game.step = crate::Step::Cleanup;
    game.extra_turns = vec![PlayerId::Two, PlayerId::Two];
    let mut vault = creature(10_000, crate::card::cards::TIME_VAULT, PlayerId::Two);
    vault.tapped = true;
    let vault_id = vault.card.id;
    game.battlefield.push(vault);

    game.start_next_turn();
    let pending = game
        .pending_decisions
        .first()
        .expect("Time Vault asks whether to replace the first extra turn");
    let replacement = pending
        .observation
        .options
        .iter()
        .find(|option| option.card.is_some_and(|(object, _)| object == vault_id))
        .expect("the Vault replacement is offered")
        .id;
    game.apply(
        PlayerId::Two,
        Action::ChooseDecision {
            decision: pending.observation.id,
            options: vec![replacement],
        },
    )
    .expect("the first extra turn is skipped");

    assert!(matches!(
        game.pending_decisions
            .first()
            .map(|pending| &pending.continuation),
        Some(DecisionContinuation::BeginTurn { deferred, .. }) if deferred.len() == 1
    ));
    assert_reconstructs(
        &game,
        "a prospective turn carrying Time Vault's deferred untap",
    );
    let mut rebuilt = rebuild_from_truth(&game, 4_243);

    for candidate in [&mut game, &mut rebuilt] {
        let pending = candidate
            .pending_decisions
            .first()
            .expect("the reconstructed turn choice remains pending");
        candidate
            .apply(
                PlayerId::Two,
                Action::ChooseDecision {
                    decision: pending.observation.id,
                    options: vec![0],
                },
            )
            .expect("the next extra turn begins");
    }

    assert_eq!(rebuilt.observe(PlayerId::One), game.observe(PlayerId::One));
    assert_eq!(rebuilt.observe(PlayerId::Two), game.observe(PlayerId::Two));
    assert_eq!(game.active_player, PlayerId::Two);
    assert!(
        !game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == vault_id)
            .expect("Time Vault remains on the battlefield")
            .tapped,
        "the carried effect untaps Time Vault before the accepted turn begins",
    );
}

#[test]
fn a_battlefield_exit_replacement_choice_explicitly_fails_closed() {
    let mut game = staged_game();
    game.battlefield.clear();
    game.put_onto_battlefield(PlayerId::Two, crate::card::cards::REST_IN_PEACE)
        .expect("Rest in Peace is cataloged");
    let nexus = game
        .put_onto_battlefield(PlayerId::One, crate::card::cards::UGINS_NEXUS)
        .expect("Ugin's Nexus is cataloged");
    game.pending_triggers.clear();
    game.move_permanents_to_graveyard(&[nexus]);

    assert!(matches!(
        game.pending_decisions
            .first()
            .map(|pending| &pending.continuation),
        Some(DecisionContinuation::BattlefieldExitReplacement { .. })
    ));
    let viewer = game
        .decision_player()
        .expect("the affected player orders the replacements");
    let observation = game.observe(viewer);
    let actions = crate::protocol::protocol_actions(&observation);
    let wire = crate::protocol::observation_json_for_format(
        &game.catalog,
        game.format,
        &observation,
        game.in_pregame(),
        &actions,
    );
    assert_eq!(wire["checkpoint"]["hasDeferredState"], Value::Bool(true));
    let error = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &wire,
        &true_hidden_hypothesis(&game, viewer),
        4_244,
    )
    .expect_err("the unencoded battlefield-exit completion graph must fail closed");
    assert!(
        error.contains("without stable catalog semantics"),
        "unexpected reconstruction error: {error}",
    );
}

/// Adds mana the way the engine does, so the itemized units and the public
/// aggregate pool stay in agreement. Writing `mana_pool` alone builds a game
/// the engine could never have reached, and reconstruction says so.
fn fill_mana(game: &mut Game, player: PlayerId, amount: u16) {
    for color in [
        ManaColor::White,
        ManaColor::Blue,
        ManaColor::Black,
        ManaColor::Red,
        ManaColor::Green,
        ManaColor::Colorless,
    ] {
        game.add_unrestricted_mana(player, color, amount);
    }
}

fn loyalty_action(
    source: GameObjectId,
    definition: CardDefinitionId,
    ability: u8,
    targets: Vec<TargetSelection>,
) -> Action {
    Action::ActivateAbility {
        source,
        ability: AbilityOrigin::Printed {
            definition,
            part: CardPartId::PRIMARY,
            ability: AbilityId(ability),
        },
        targets,
        cost_object: None,
        x: 0,
    }
}

fn resolve_top_of_stack(game: &mut Game) {
    for _ in 0..8 {
        if !game.pending_decisions.is_empty() || game.stack.is_empty() {
            return;
        }
        let priority = game.priority;
        game.apply(priority, Action::PassPriority)
            .expect("priority passes while the stack resolves");
    }
}

fn answer_with_first_option(game: &mut Game) {
    let pending = game
        .pending_decisions
        .first()
        .expect("a decision is pending");
    let player = pending.observation.player;
    let decision = pending.observation.id;
    let option = pending.observation.options[0].id;
    game.apply(
        player,
        Action::ChooseDecision {
            decision,
            options: vec![option],
        },
    )
    .expect("the first option is answerable");
}

/// Casts `source` at `target` by finding the action the seat is actually
/// offered, so target-slot bookkeeping stays the engine's business.
fn cast_targeting(game: &mut Game, player: PlayerId, source: GameObjectId, target: Target) {
    let action = game
        .legal_actions(player)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell { card, choices, .. }
                    if *card == source
                        && choices.iter_targets().copied().eq([target])
            )
        })
        .unwrap_or_else(|| panic!("{source:?} cannot be cast at {target:?}"));
    game.apply(player, action).expect("the cast is legal");
}

/// Answers the pending decision with the option standing for `object`. Entry
/// choices usually offer "as itself" first, and taking that default would walk
/// straight past the state the test exists to build.
fn answer_with_option_naming(game: &mut Game, object: GameObjectId) {
    let pending = game
        .pending_decisions
        .first()
        .expect("a decision is pending");
    let player = pending.observation.player;
    let decision = pending.observation.id;
    let option = pending
        .observation
        .options
        .iter()
        .find(|option| {
            option
                .card
                .is_some_and(|(candidate, _)| candidate == object)
        })
        .unwrap_or_else(|| panic!("{object:?} is not one of the offered options"))
        .id;
    game.apply(
        player,
        Action::ChooseDecision {
            decision,
            options: vec![option],
        },
    )
    .expect("the named option is answerable");
}

/// Liliana's +1 asks both players at once. Between the two answers the engine
/// holds one seat's committed discard while the other is still choosing, which
/// is a choice the waiting seat must not be able to read out of its own
/// checkpoint -- and which the host must still be able to hand back.
#[test]
fn a_multi_player_discard_reconstructs_while_one_choice_is_still_hidden() {
    let mut game = staged_modern_game();
    let walker_id = GameObjectId(10_000);
    let mut walker = creature(
        walker_id.0,
        crate::card::cards::LILIANA_OF_THE_VEIL,
        PlayerId::One,
    );
    walker.set_counters(CounterKind::Loyalty, 3);
    game.battlefield.push(walker);
    for (player, base) in [(PlayerId::One, 11_000), (PlayerId::Two, 12_000)] {
        for offset in 0..2 {
            let card = card(base + offset, crate::card::cards::MOUNTAIN, player);
            game.players[player.index()].hand.push(card);
        }
    }

    game.apply(
        PlayerId::One,
        loyalty_action(
            walker_id,
            crate::card::cards::LILIANA_OF_THE_VEIL,
            0,
            Vec::new(),
        ),
    )
    .expect("Liliana's plus ability activates");
    resolve_top_of_stack(&mut game);

    assert_eq!(
        game.decision_player(),
        Some(PlayerId::One),
        "the active player discards first"
    );
    assert_reconstructs(&game, "a multi-player discard before any choice");

    answer_with_first_option(&mut game);
    assert_eq!(
        game.decision_player(),
        Some(PlayerId::Two),
        "the opposing seat still owes a discard"
    );
    let chosen = matches!(
        game.pending_decisions[0].continuation,
        DecisionContinuation::DiscardForEffect { ref chosen, .. }
            if chosen.iter().any(|(player, cards)| *player == PlayerId::One && !cards.is_empty())
    );
    assert!(chosen, "the first seat's discard must already be recorded");
    assert_reconstructs(&game, "a multi-player discard holding a hidden choice");
}

/// Liliana's ultimate is the only card-owned pile program in the catalog and
/// it never fires in sampled play. Its two continuations carry a callback,
/// which the snapshot must address by registry key rather than by pointer.
#[test]
fn a_card_owned_pile_program_reconstructs_at_both_of_its_boundaries() {
    let mut game = staged_modern_game();
    let walker_id = GameObjectId(10_000);
    let mut walker = creature(
        walker_id.0,
        crate::card::cards::LILIANA_OF_THE_VEIL,
        PlayerId::One,
    );
    walker.set_counters(CounterKind::Loyalty, 6);
    game.battlefield.push(walker);
    for offset in 0..3 {
        game.battlefield.push(creature(
            12_000 + offset,
            crate::card::cards::WALKING_CORPSE,
            PlayerId::Two,
        ));
    }

    game.apply(
        PlayerId::One,
        loyalty_action(
            walker_id,
            crate::card::cards::LILIANA_OF_THE_VEIL,
            2,
            vec![TargetSelection::single(
                TargetSlotId(0),
                Target::Player(PlayerId::Two),
            )],
        ),
    )
    .expect("Liliana's ultimate activates");
    resolve_top_of_stack(&mut game);

    assert!(
        matches!(
            game.pending_decisions
                .first()
                .map(|pending| &pending.continuation),
            Some(DecisionContinuation::SeparateIntoPiles { .. })
        ),
        "the ultimate must be waiting on a pile split, not {:?}",
        game.pending_decisions
            .first()
            .map(|pending| &pending.continuation)
    );
    assert_reconstructs(&game, "a card-owned pile split");

    answer_with_first_option(&mut game);
    assert!(
        matches!(
            game.pending_decisions
                .first()
                .map(|pending| &pending.continuation),
            Some(DecisionContinuation::ChoosePile { .. })
        ),
        "splitting must lead to the pile choice"
    );
    assert_reconstructs(&game, "a card-owned pile choice");
}

/// A floating trigger belongs to no object: its source has already resolved,
/// and it watches the game until a named player's next turn. Sampled play
/// never installed one, and it is the clearest case of executable state with
/// nowhere obvious to live in a snapshot.
#[test]
fn a_floating_trigger_installed_by_a_resolved_ability_reconstructs() {
    let mut game = staged_modern_game();
    let walker_id = GameObjectId(10_000);
    let mut walker = creature(
        walker_id.0,
        crate::card::cards::JACE_ARCHITECT_OF_THOUGHT,
        PlayerId::One,
    );
    walker.set_counters(CounterKind::Loyalty, 4);
    game.battlefield.push(walker);

    game.apply(
        PlayerId::One,
        loyalty_action(
            walker_id,
            crate::card::cards::JACE_ARCHITECT_OF_THOUGHT,
            0,
            Vec::new(),
        ),
    )
    .expect("Jace's plus ability activates");
    resolve_top_of_stack(&mut game);

    assert_eq!(
        game.floating_triggers.len(),
        1,
        "the resolved ability must leave a floating trigger behind"
    );
    assert_reconstructs(&game, "a floating trigger watching the game");
}

/// The opposing seat splits cards it has just been shown, and those cards are
/// in no zone at all while it decides. Both halves of the program have to
/// survive a round trip, including the placement instructions for whichever
/// pile is not kept.
#[test]
fn a_revealed_pile_split_reconstructs_at_both_of_its_boundaries() {
    let mut game = staged_modern_game();
    let walker_id = GameObjectId(10_000);
    let mut walker = creature(
        walker_id.0,
        crate::card::cards::JACE_ARCHITECT_OF_THOUGHT,
        PlayerId::One,
    );
    walker.set_counters(CounterKind::Loyalty, 4);
    game.battlefield.push(walker);

    game.apply(
        PlayerId::One,
        loyalty_action(
            walker_id,
            crate::card::cards::JACE_ARCHITECT_OF_THOUGHT,
            1,
            Vec::new(),
        ),
    )
    .expect("Jace's minus ability activates");
    resolve_top_of_stack(&mut game);

    assert!(
        matches!(
            game.pending_decisions
                .first()
                .map(|pending| &pending.continuation),
            Some(DecisionContinuation::RevealedPileSplit { .. })
        ),
        "the ability must be waiting on a revealed pile split, not {:?}",
        game.pending_decisions
            .first()
            .map(|pending| &pending.continuation)
    );
    assert_reconstructs(&game, "a revealed pile split");

    answer_with_first_option(&mut game);
    assert!(
        matches!(
            game.pending_decisions
                .first()
                .map(|pending| &pending.continuation),
            Some(DecisionContinuation::RevealedPileChoice { .. })
        ),
        "splitting must lead to the pile choice"
    );
    assert_reconstructs(&game, "a revealed pile choice");
}

/// Copy Artifact chooses what to be as it enters, so the seat sees a
/// replacement decision whose options are permanents, and then a permanent
/// whose characteristics come from a frozen copy rather than its own card.
#[test]
fn an_entering_copy_reconstructs_while_choosing_and_after_it_has_copied() {
    let mut game = staged_game();
    game.battlefield.push(creature(
        12_000,
        crate::card::cards::ORNITHOPTER,
        PlayerId::Two,
    ));
    let copy = card(11_000, crate::card::cards::COPY_ARTIFACT, PlayerId::One);
    let copy_id = copy.id;
    game.players[PlayerId::One.index()].hand.push(copy);
    fill_mana(&mut game, PlayerId::One, 4);

    game.apply(
        PlayerId::One,
        Action::CastSpell {
            card: copy_id,
            choices: crate::CastChoices::default(),
            sacrifices: Vec::new(),
        },
    )
    .expect("Copy Artifact is castable");
    resolve_top_of_stack(&mut game);

    assert!(
        matches!(
            game.pending_decisions
                .first()
                .map(|pending| &pending.continuation),
            Some(DecisionContinuation::BattlefieldEntryCopy { .. })
        ),
        "entering must ask what to copy, not {:?}",
        game.pending_decisions
            .first()
            .map(|pending| &pending.continuation)
    );
    assert_reconstructs(&game, "a permanent choosing what to enter as");

    answer_with_option_naming(&mut game, GameObjectId(12_000));
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.copy_effect.is_some()),
        "the choice must leave a copied permanent behind"
    );
    assert_reconstructs(&game, "a permanent that entered as a copy");
}

/// A permanent that named a card as it entered carries that name for the rest
/// of the game. The name is free text rather than a catalog id, so it is the
/// one piece of permanent state a locator cannot address.
#[test]
fn a_permanent_that_named_a_card_reconstructs_while_naming_and_after() {
    let mut game = staged_modern_game();
    let needle = card(11_000, crate::card::cards::PITHING_NEEDLE, PlayerId::One);
    let needle_id = needle.id;
    game.players[PlayerId::One.index()].hand.push(needle);
    fill_mana(&mut game, PlayerId::One, 4);

    game.apply(
        PlayerId::One,
        Action::CastSpell {
            card: needle_id,
            choices: crate::CastChoices::default(),
            sacrifices: Vec::new(),
        },
    )
    .expect("Pithing Needle is castable");
    resolve_top_of_stack(&mut game);

    assert!(
        matches!(
            game.pending_decisions
                .first()
                .map(|pending| &pending.continuation),
            Some(DecisionContinuation::BattlefieldEntryCardName { .. })
        ),
        "entering must ask for a card name, not {:?}",
        game.pending_decisions
            .first()
            .map(|pending| &pending.continuation)
    );
    assert_reconstructs(&game, "a permanent choosing a card name");

    answer_with_first_option(&mut game);
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.chosen_card_name.is_some()),
        "the choice must leave a named permanent behind"
    );
    assert_reconstructs(&game, "a permanent holding a chosen card name");
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
    while game
        .pending_decisions
        .first()
        .is_some_and(|pending| matches!(pending.continuation, DecisionContinuation::Fork { .. }))
    {
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
