//! Cosmogrand Zenith: the second spell each turn pays again, and the choice
//! is between going wider and going taller.

use super::*;

/// The Zenith on the battlefield under Player One with a bear beside him,
/// and `hand` in hand with mana to spare.
fn staged(hand: &[CardDefinitionId]) -> (Game, GameObjectId, Vec<GameObjectId>) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let zenith = game
        .put_onto_battlefield(PlayerId::One, cards::COSMOGRAND_ZENITH)
        .expect("cataloged");
    game.put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    drain_pending(&mut game);
    let mut held = Vec::new();
    for definition in hand {
        let card = game
            .build_zone(PlayerId::One, &[*definition])
            .expect("cataloged")
            .into_iter()
            .next()
            .expect("one card");
        held.push(card.id);
        game.players[0].hand.push(card);
    }
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    for color in [ManaColor::Red, ManaColor::Green, ManaColor::Colorless] {
        game.add_unrestricted_mana(PlayerId::One, color, 6);
    }
    game.turns_started = [5, 5];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, zenith, held)
}

/// Answers decisions, preferring mode `mode` where one is offered.
fn settle(game: &mut Game, mode: usize) {
    for _ in 0..24 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = decision
                .options
                .get(mode)
                .or_else(|| decision.options.first())
                .map(|option| vec![option.id])
                .unwrap_or_default();
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .expect("the offered choice is legal");
            continue;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let priority = game.priority;
        if game.apply(priority, Action::PassPriority).is_err() {
            break;
        }
    }
    drain_pending(game);
}

fn cast(game: &mut Game, held: GameObjectId, mode: usize) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == held))
        .expect("it is castable");
    game.apply(PlayerId::One, action).expect("it is cast");
    settle(game, mode);
}

fn soldiers(game: &Game) -> usize {
    game.battlefield
        .iter()
        .filter(|permanent| {
            game.effective_subtypes(permanent)
                .contains(crate::card::Subtype::named("Soldier"))
        })
        .filter(|permanent| permanent.card.definition != cards::COSMOGRAND_ZENITH)
        .count()
}

fn counters_on(game: &Game, id: GameObjectId) -> u16 {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .map_or(0, |permanent| {
            permanent.counters(CounterKind::PlusOnePlusOne)
        })
}

/// The first spell does nothing; the second offers the choice.
#[test]
fn the_second_spell_makes_two_soldiers() {
    let (mut game, _, held) = staged(&[cards::LIGHTNING_BOLT, cards::GIANT_GROWTH]);

    cast(&mut game, held[0], 0);
    assert_eq!(soldiers(&game), 0, "the first spell pays nothing");

    cast(&mut game, held[1], 0);

    assert_eq!(soldiers(&game), 2, "and the second makes two Soldiers");
}

/// The other mode grows everything you have, the Zenith included.
#[test]
fn the_other_mode_grows_the_board() {
    let (mut game, zenith, held) = staged(&[cards::LIGHTNING_BOLT, cards::GIANT_GROWTH]);
    let bears = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::GRIZZLY_BEARS)
        .expect("the bear is there")
        .card
        .id;

    cast(&mut game, held[0], 1);
    cast(&mut game, held[1], 1);

    assert_eq!(counters_on(&game, zenith), 1, "the Zenith counts himself");
    assert_eq!(counters_on(&game, bears), 1, "and the bear beside him");
    assert_eq!(soldiers(&game), 0, "the other mode was not taken");
}

/// "Your second spell each turn": exactly the second, so the third pays
/// nothing.
#[test]
fn the_third_spell_pays_nothing() {
    let (mut game, _, held) = staged(&[
        cards::LIGHTNING_BOLT,
        cards::GIANT_GROWTH,
        cards::LIGHTNING_BOLT,
    ]);

    cast(&mut game, held[0], 0);
    cast(&mut game, held[1], 0);
    assert_eq!(soldiers(&game), 2);

    cast(&mut game, held[2], 0);

    assert_eq!(soldiers(&game), 2, "the third is not the second");
}

/// The count is per turn, and it is yours: their spells are not counted.
#[test]
fn their_spells_are_not_yours() {
    let (mut game, _, held) = staged(&[cards::GIANT_GROWTH]);
    let theirs = game
        .build_zone(PlayerId::Two, &[cards::LIGHTNING_BOLT])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let their_bolt = theirs.id;
    game.players[1].hand.push(theirs);
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Red, 1);
    game.priority = PlayerId::Two;
    let cast_theirs = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == their_bolt))
        .expect("they can cast it");
    game.apply(PlayerId::Two, cast_theirs).expect("it is cast");
    settle(&mut game, 0);
    game.priority = PlayerId::One;

    cast(&mut game, held[0], 0);

    assert_eq!(
        soldiers(&game),
        0,
        "their Bolt did not make your Growth the second spell",
    );
}

/// "The ability resolves before the spell that caused it to trigger.
/// Notably, if your second spell is a creature spell and you choose the
/// second mode, the resulting creature won't get a +1/+1 counter."
#[test]
fn the_creature_that_triggered_it_arrives_too_late_for_a_counter() {
    let (mut game, zenith, held) = staged(&[cards::LIGHTNING_BOLT, cards::LLANOWAR_ELVES]);

    cast(&mut game, held[0], 1);
    cast(&mut game, held[1], 1);

    assert_eq!(counters_on(&game, zenith), 1, "the Zenith grew");
    let elves = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::LLANOWAR_ELVES)
        .expect("the Elves resolved after the trigger did")
        .card
        .id;
    assert_eq!(
        counters_on(&game, elves),
        0,
        "and the spell that triggered it was still on the stack at the time",
    );
}

/// "It resolves even if that spell is countered or otherwise leaves the
/// stack without resolving."
#[test]
fn countering_the_second_spell_does_not_undo_the_trigger() {
    let (mut game, _zenith, held) = staged(&[cards::LIGHTNING_BOLT, cards::LIGHTNING_BOLT]);
    let counter = game
        .build_zone(PlayerId::Two, &[cards::COUNTERSPELL])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let counter_id = counter.id;
    game.players[1].hand.push(counter);
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Blue, 2);

    cast(&mut game, held[0], 0);
    let bolt = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == held[1]))
        .expect("the second Bolt is castable");
    game.apply(PlayerId::One, bolt).expect("it is cast");

    // The trigger goes on the stack above the Bolt and so resolves first,
    // which is the ruling: answer its mode, then counter the Bolt still
    // sitting underneath.
    let bolt_on_stack = game
        .stack
        .iter()
        .find(|object| object.card.definition == cards::LIGHTNING_BOLT)
        .expect("the Bolt is on the stack")
        .id;
    for _ in 0..12 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options: vec![decision.options[0].id],
                },
            )
            .expect("the mode is chosen");
            continue;
        }
        if let Some(answer) = game
            .legal_actions(PlayerId::Two)
            .into_iter()
            .find(|action| {
                matches!(action, Action::CastSpell { card, choices, .. }
                if *card == counter_id
                    && choices.targets().iter().any(|selection| {
                        selection.targets().contains(&Target::Spell(bolt_on_stack))
                    }))
            })
        {
            game.apply(PlayerId::Two, answer).expect("it is cast");
            break;
        }
        let priority = game.priority;
        if game.apply(priority, Action::PassPriority).is_err() {
            break;
        }
    }
    settle(&mut game, 0);

    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::LIGHTNING_BOLT),
        "the Bolt was countered",
    );
    assert_eq!(
        soldiers(&game),
        2,
        "and the trigger it caused paid out all the same",
    );
}

/// "It will count any spells you've cast this turn, which may include
/// Cosmogrand Zenith itself." The fixture puts him onto the battlefield, so
/// this is the one that casts him: he is the first spell, and the next one
/// is the second.
#[test]
fn he_counts_himself_when_he_was_cast() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let zenith = card(87_000, cards::COSMOGRAND_ZENITH, PlayerId::One);
    let zenith_id = zenith.id;
    game.players[0].hand.push(zenith);
    let bolt = card(87_001, cards::LIGHTNING_BOLT, PlayerId::One);
    let bolt_id = bolt.id;
    game.players[0].hand.push(bolt);
    for color in [ManaColor::White, ManaColor::Red, ManaColor::Colorless] {
        game.add_unrestricted_mana(PlayerId::One, color, 4);
    }
    game.turns_started = [5, 5];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;

    cast(&mut game, zenith_id, 0);
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == zenith_id
                || permanent.card.definition == cards::COSMOGRAND_ZENITH),
        "he resolved, and was the first spell of the turn",
    );
    assert_eq!(soldiers(&game), 0, "his own cast pays nothing");

    cast(&mut game, bolt_id, 0);

    assert_eq!(
        soldiers(&game),
        2,
        "the Bolt is the second spell because he was the first",
    );
}

/// "Each turn": the count starts again, so a spell left over from last turn
/// does not make this turn's first spell a second.
#[test]
fn the_count_starts_again_each_turn() {
    let (mut game, _, held) = staged(&[
        cards::LIGHTNING_BOLT,
        cards::GIANT_GROWTH,
        cards::LIGHTNING_BOLT,
    ]);

    cast(&mut game, held[0], 0);
    assert_eq!(soldiers(&game), 0, "one spell this turn");

    // Round to Player One's turn again.
    for _ in 0..2 {
        game.start_next_turn();
        drain_pending(&mut game);
    }
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    for color in [ManaColor::Red, ManaColor::Green, ManaColor::Colorless] {
        game.add_unrestricted_mana(PlayerId::One, color, 6);
    }

    cast(&mut game, held[1], 0);
    assert_eq!(
        soldiers(&game),
        0,
        "the first spell of the new turn is a first spell",
    );

    cast(&mut game, held[2], 0);
    assert_eq!(soldiers(&game), 2, "and the one after it is the second");
}

/// "Two 1/1 white Human Soldier creature tokens", and the mode is a choice
/// of one: taking the tokens puts no counters on anything.
#[test]
fn the_soldiers_are_one_one_white_humans_and_nothing_is_grown() {
    let (mut game, zenith, held) = staged(&[cards::LIGHTNING_BOLT, cards::GIANT_GROWTH]);

    cast(&mut game, held[0], 0);
    cast(&mut game, held[1], 0);

    let made: Vec<_> = game
        .battlefield
        .iter()
        .filter(|permanent| permanent.card.definition == ObjectKind::Token)
        .collect();
    assert_eq!(made.len(), 2, "two of them");
    for token in made {
        assert_eq!(
            (game.power(token), game.toughness(token)),
            (Some(1), Some(1)),
            "a 1/1",
        );
        assert_eq!(
            game.effective_colors(token, &game.effective_rules(token).expect("rules")),
            [true, false, false, false, false],
            "white and nothing else",
        );
        let subtypes = game.effective_subtypes(token);
        assert!(
            subtypes.contains(crate::card::Subtype::named("Human")),
            "a Human"
        );
        assert!(
            subtypes.contains(crate::card::Subtype::named("Soldier")),
            "and a Soldier"
        );
    }
    assert_eq!(
        counters_on(&game, zenith),
        0,
        "and choosing one mode is choosing not the other",
    );
}

/// "Put a +1/+1 counter on each creature *you* control": their board grows
/// by nothing, however much of it there is.
#[test]
fn the_counters_go_only_on_your_own_creatures() {
    let (mut game, zenith, held) = staged(&[cards::LIGHTNING_BOLT, cards::GIANT_GROWTH]);
    let theirs = game
        .put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("cataloged");
    drain_pending(&mut game);
    game.priority = PlayerId::One;

    cast(&mut game, held[0], 1);
    cast(&mut game, held[1], 1);

    assert_eq!(counters_on(&game, zenith), 1, "yours grew");
    assert_eq!(
        counters_on(&game, theirs),
        0,
        "and the Angel across the table did not",
    );
}
