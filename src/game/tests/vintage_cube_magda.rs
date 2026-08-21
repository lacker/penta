//! Magda, Brazen Outlaw: every tap is a Treasure, and five Treasures are
//! whatever artifact the deck is built around.

use super::*;

/// Player One with Magda out since last turn, plus `others` beside her.
fn staged(others: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let magda = game
        .put_onto_battlefield(PlayerId::One, cards::MAGDA_BRAZEN_OUTLAW)
        .expect("cataloged");
    for definition in others {
        game.put_onto_battlefield(PlayerId::One, *definition)
            .expect("cataloged");
    }
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [2, 1];
    drain_pending(&mut game);
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, magda)
}

fn deciding(game: &Game) -> Option<PlayerId> {
    game.pending_decisions
        .first()
        .map(|pending| pending.observation.player)
}

fn settle(game: &mut Game) {
    for _ in 0..24 {
        if deciding(game).is_some() {
            return;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    game.check_state_based_actions();
}

fn treasures(game: &Game) -> usize {
    game.battlefield
        .iter()
        .filter(|permanent| is_token_with(permanent, tokens::treasure()))
        .count()
}

fn permanent_of(game: &Game, definition: CardDefinitionId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == definition)
        .expect("it is on the battlefield")
}

/// Puts `count` Treasures onto the battlefield without going through Magda.
fn give_treasures(game: &mut Game, count: usize) {
    for _ in 0..count {
        game.create_token(PlayerId::One, tokens::treasure());
    }
    drain_pending(game);
    game.priority = PlayerId::One;
}

/// "Other Dwarves you control get +1/+0": another Dwarf grows, Magda does
/// not.
#[test]
fn other_dwarves_get_bigger_and_she_does_not() {
    let (game, _magda) = staged(&[cards::DWARVEN_DEMOLITION_TEAM]);

    let other = permanent_of(&game, cards::DWARVEN_DEMOLITION_TEAM);
    assert_eq!(
        game.power(other),
        Some(2),
        "a 1/1 Dwarf is a 2/1 beside her"
    );
    let magda = permanent_of(&game, cards::MAGDA_BRAZEN_OUTLAW);
    assert_eq!(
        game.power(magda),
        Some(2),
        "and she is still the 2/1 she prints",
    );
}

/// A creature that is not a Dwarf gets nothing.
#[test]
fn a_nondwarf_gets_nothing() {
    let (game, _magda) = staged(&[cards::GRIZZLY_BEARS]);

    assert_eq!(
        game.power(permanent_of(&game, cards::GRIZZLY_BEARS)),
        Some(2),
        "a Grizzly Bears is not a Dwarf",
    );
}

/// Tapping a Dwarf makes a Treasure -- including tapping Magda herself.
#[test]
fn tapping_a_dwarf_makes_a_treasure() {
    let (mut game, magda) = staged(&[]);
    assert_eq!(treasures(&game), 0, "none yet");

    game.tap_permanent(magda);
    settle(&mut game);

    assert_eq!(treasures(&game), 1, "one Treasure for the tap");
}

/// It is any Dwarf you control, not just her.
#[test]
fn tapping_another_dwarf_makes_one_too() {
    let (mut game, _magda) = staged(&[cards::DWARVEN_DEMOLITION_TEAM]);
    let other = permanent_of(&game, cards::DWARVEN_DEMOLITION_TEAM).card.id;

    game.tap_permanent(other);
    settle(&mut game);

    assert_eq!(treasures(&game), 1, "the other Dwarf counts");
}

/// A nonDwarf tapping makes nothing.
#[test]
fn tapping_a_nondwarf_makes_nothing() {
    let (mut game, _magda) = staged(&[cards::GRIZZLY_BEARS]);
    let bears = permanent_of(&game, cards::GRIZZLY_BEARS).card.id;

    game.tap_permanent(bears);
    settle(&mut game);

    assert_eq!(treasures(&game), 0, "the clause names Dwarves");
}

/// Five Treasures find an artifact and put it onto the battlefield.
#[test]
fn five_treasures_fetch_an_artifact() {
    let (mut game, magda) = staged(&[]);
    give_treasures(&mut game, 5);
    let ring = game
        .build_zone(PlayerId::One, &[cards::SOL_RING])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    game.players[0].library.push(ring);

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == magda))
        .expect("five Treasures pay for it");
    game.apply(PlayerId::One, action).expect("it activates");
    // Paying five Treasures is itself a decision, and the search is the one
    // after it, so answer whatever is asked until the Ring is on the menu.
    for _ in 0..8 {
        settle(&mut game);
        let Some(seat) = deciding(&game) else { break };
        let decision = game.observe(seat).decision.expect("just checked");
        let wanted = decision.options.iter().find(|option| {
            option
                .card
                .is_some_and(|(_, found)| found.card_definition() == Some(cards::SOL_RING))
        });
        let options = wanted.map_or_else(
            || {
                decision
                    .options
                    .iter()
                    .take(decision.minimum)
                    .map(|option| option.id)
                    .collect()
            },
            |option| vec![option.id],
        );
        game.apply(
            seat,
            Action::ChooseDecision {
                decision: decision.id,
                options,
            },
        )
        .expect("the answer is legal");
    }
    settle(&mut game);

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::SOL_RING),
        "the Sol Ring is on the battlefield",
    );
    assert_eq!(treasures(&game), 0, "and all five Treasures paid for it");
}

/// Four is not five: the cost is paid in full or not at all.
#[test]
fn four_treasures_are_not_enough() {
    let (mut game, magda) = staged(&[]);
    give_treasures(&mut game, 4);

    assert!(
        !game.legal_actions(PlayerId::One).iter().any(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == magda)
        }),
        "a partial payment buys nothing",
    );
}

/// A Treasure is a mana of any colour, spent by sacrificing it.
#[test]
fn a_treasure_makes_one_mana_of_any_colour() {
    let (mut game, _magda) = staged(&[]);
    give_treasures(&mut game, 1);
    let treasure = game
        .battlefield
        .iter()
        .find(|permanent| is_token_with(permanent, tokens::treasure()))
        .expect("the Treasure is on the battlefield")
        .card
        .id;

    let colors = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::ActivateManaAbility { source, color, .. } if source == treasure => Some(color),
            _ => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(colors.len(), 5, "all five colours: {colors:?}");
}
