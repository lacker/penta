//! Permanents that name a colour or a creature type as they enter, and then
//! read that name back for the rest of the game. Nothing about these cards is
//! decided until the board is already visible, so the same printed card is a
//! different card in every deck -- and the interesting half is the read, not
//! the choice: an anthem that has to recognise its tribe, a body that has to
//! become the colour it picked, and protection that has to turn away exactly
//! one fifth of the creatures across the table.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::One.index()] = 9;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game
}

/// Casts `definition` from hand with the mana already floating, answering the
/// entry choice with the option whose label names `choice`.
fn cast_choosing(game: &mut Game, definition: CardDefinitionId, choice: &str) {
    let held = card(74_000, definition, PlayerId::One);
    let held_id = held.id;
    game.players[0].hand.push(held);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 4);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 2);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 8);
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == held_id))
        .expect("a cast was offered");
    game.apply(PlayerId::One, cast).expect("it is cast");

    let mut answered = false;
    for _ in 0..8 {
        if let Some(decision) = game.observe(PlayerId::One).decision {
            let option = decision
                .options
                .iter()
                .find(|option| option.label.contains(choice))
                .unwrap_or_else(|| {
                    let labels: Vec<_> = decision.options.iter().map(|o| o.label.clone()).collect();
                    panic!("no option named {choice}; the offer was {labels:?}")
                })
                .id;
            game.apply(
                PlayerId::One,
                Action::ChooseDecision {
                    decision: decision.id,
                    options: vec![option],
                },
            )
            .expect("the choice is legal");
            answered = true;
        }
        if game.stack.is_empty() && answered {
            break;
        }
        let holder = game.priority;
        if game.apply(holder, Action::PassPriority).is_err() {
            break;
        }
    }
    assert!(answered, "the entry choice was offered");
    drain_pending(game);
}

fn stats(game: &Game, definition: CardDefinitionId) -> (i16, i16) {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == ObjectKind::Card(definition))
        .expect("the permanent is on the battlefield");
    let stats = game.creature_stats(permanent).expect("a creature");
    (stats.power, stats.toughness)
}

/// Shared Triumph names its tribe on the way in, and only that tribe grows.
#[test]
fn shared_triumph_pumps_the_named_tribe_and_leaves_the_rest_alone() {
    let mut game = ready();
    // Goblin King is a 2/2 whose own anthem passes itself over, so the
    // difference measured here is the Triumph's alone.
    game.battlefield
        .push(creature(10_000, cards::GOBLIN_KING, PlayerId::One));
    game.battlefield
        .push(creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One));

    assert_eq!(stats(&game, cards::GOBLIN_KING), (2, 2), "printed");
    assert_eq!(stats(&game, cards::SAVANNAH_LIONS), (2, 1), "printed");

    cast_choosing(&mut game, cards::SHARED_TRIUMPH, "Goblin");

    assert_eq!(
        stats(&game, cards::GOBLIN_KING),
        (3, 3),
        "the King is a Goblin, so the Triumph found it"
    );
    assert_eq!(
        stats(&game, cards::SAVANNAH_LIONS),
        (2, 1),
        "a Cat is not what was named"
    );
}

/// Alloy Golem arrives colourless and paints itself whichever colour it
/// named, which is the only reason to pay six for a 4/4.
#[test]
fn alloy_golem_becomes_the_color_it_named() {
    let mut game = ready();
    cast_choosing(&mut game, cards::ALLOY_GOLEM, "Blue");
    let golem = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == ObjectKind::Card(cards::ALLOY_GOLEM))
        .expect("the Golem resolved");
    assert_eq!(
        game.object_colors(golem.card.id),
        [false, true, false, false, false],
        "blue and nothing else, artifact or not"
    );
}
