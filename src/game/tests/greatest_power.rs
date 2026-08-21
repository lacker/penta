//! The greatest power among the creatures you control.
//!
//! Not a count and not a sum: one creature's size, chosen from the board, and
//! zero when there is nobody to ask. Each test puts a Wall of Stone beside
//! the big creature so that a reading of toughness, a count, or a total would
//! all give a different answer.

use super::*;
use crate::ImplementationStatus;

/// A 4/4 Serra Angel, a 0/8 Wall, and a 2/2 bear under player one, plus a
/// bigger creature the opponent controls that must not be counted.
fn board() -> Game {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    game.step = Step::PrecombatMain;
    game.battlefield.clear();
    game.battlefield
        .push(creature(10_000, cards::SERRA_ANGEL, PlayerId::One));
    game.battlefield
        .push(creature(10_001, cards::WALL_OF_STONE, PlayerId::One));
    game.battlefield
        .push(creature(10_002, cards::GRIZZLY_BEARS, PlayerId::One));
    game.battlefield
        .push(creature(10_003, cards::GHOULTREE, PlayerId::Two));
    game
}

/// Casts the named spell from hand at `target`, if any.
fn cast(game: &mut Game, definition: CardDefinitionId, at_opponent: bool) {
    let spell = card(20_000, definition, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.black = 1;
    game.players[PlayerId::One.index()].mana_pool.green = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 3;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == spell_id
                    && (!at_opponent
                        || choices.targets().iter().any(|selection| {
                            selection.targets().contains(&Target::Player(PlayerId::Two))
                        }))
            }
            _ => false,
        })
        .expect("the spell is castable");
    game.apply(PlayerId::One, action).expect("it is cast");
    drain_pending(game);
}

/// Four is the Angel's power: not the Wall's eight toughness, not three
/// creatures, and not their nine total power.
#[test]
fn essence_harvest_drains_the_biggest_power() {
    let mut game = board();
    cast(&mut game, cards::ESSENCE_HARVEST, true);

    assert_eq!(game.players[1].life, 16, "four lost");
    assert_eq!(game.players[0].life, 24, "and four gained");
}

/// The opponent's 10/10 is not one of yours.
#[test]
fn essence_harvest_ignores_creatures_you_do_not_control() {
    let mut game = board();
    game.battlefield
        .retain(|permanent| permanent.controller == PlayerId::Two);
    cast(&mut game, cards::ESSENCE_HARVEST, true);

    assert_eq!(game.players[1].life, 20, "no creatures of yours, no drain");
    assert_eq!(game.players[0].life, 20);
}

#[test]
fn fungal_sprouting_makes_one_saproling_per_greatest_power() {
    let mut game = board();
    cast(&mut game, cards::FUNGAL_SPROUTING, false);

    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| is_token_with(
                permanent,
                tokens::creature(&["Saproling"], &[ManaColor::Green], 1, 1)
            ))
            .count(),
        4,
        "the Angel's four, not the Wall's eight or the board's three",
    );
}

/// Garruk's draw ability was the one clause the value was missing.
#[test]
fn garruk_draws_the_greatest_power() {
    let mut game = board();
    let garruk = creature(10_010, cards::GARRUK_PRIMAL_HUNTER, PlayerId::One);
    let garruk_id = garruk.card.id;
    game.battlefield.push(garruk);
    for index in 0..8 {
        game.players[PlayerId::One.index()].library.push(card(
            30_000 + index,
            cards::GRIZZLY_BEARS,
            PlayerId::One,
        ));
    }
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == garruk_id)
        .expect("Garruk is there")
        .set_counters(CounterKind::Loyalty, 3);
    let before = game.players[PlayerId::One.index()].hand.len();

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility {
                source,
                ability: AbilityOrigin::Printed { ability, .. },
                ..
            } if *source == garruk_id && *ability == AbilityId(1))
        })
        .expect("the minus three is offered");
    game.apply(PlayerId::One, action).expect("it is activated");
    drain_pending(&mut game);

    assert_eq!(
        game.players[PlayerId::One.index()].hand.len(),
        before + 4,
        "four cards for the Angel's four power",
    );
}

#[test]
fn both_new_spells_report_complete_coverage() {
    let catalog = poc::catalog().expect("catalog builds");
    for definition in [
        cards::ESSENCE_HARVEST,
        cards::FUNGAL_SPROUTING,
        cards::GARRUK_PRIMAL_HUNTER,
    ] {
        let card = catalog.get(definition).expect("the card is cataloged");
        assert_eq!(
            card.rules.implementation_status(),
            ImplementationStatus::Complete,
            "{} should be fully executable",
            card.name,
        );
    }
}
