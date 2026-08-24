//! Cards that read a graveyard, and the one that empties a library on
//! purpose.
//!
//! What these have in common is a zone nobody is looking at: how many card
//! types a graveyard holds, and whether a library still has anything left.

use super::*;

/// Delirium counts card types in your own graveyard, not both. Four types is
/// the line: below it the spell deals two, at it six.
#[test]
fn unholy_heat_deals_six_only_with_four_card_types_in_your_graveyard() {
    for (types, expected) in [(3_usize, 2_i16), (4, 6)] {
        let mut game = ready_game();
        game.battlefield.clear();
        game.players[PlayerId::One.index()].graveyard.clear();
        game.players[PlayerId::Two.index()].graveyard.clear();
        // A fifth type sitting in the opponent's graveyard must not count.
        game.players[PlayerId::Two.index()].graveyard.push(card(
            80_000,
            cards::BLACK_LOTUS,
            PlayerId::Two,
        ));

        for (index, definition) in [
            cards::GRIZZLY_BEARS,
            cards::LIGHTNING_BOLT,
            cards::FOREST,
            cards::PHYREXIAN_ARENA,
        ]
        .into_iter()
        .take(types)
        .enumerate()
        {
            game.players[PlayerId::One.index()].graveyard.push(card(
                80_100 + u32::try_from(index).expect("four cards fit"),
                definition,
                PlayerId::One,
            ));
        }

        let angel = creature(80_200, cards::SERRA_ANGEL, PlayerId::Two);
        let angel_id = angel.card.id;
        game.battlefield.push(angel);

        let heat = card(80_201, cards::UNHOLY_HEAT, PlayerId::One);
        let heat_id = heat.id;
        game.players[PlayerId::One.index()].hand.push(heat);
        game.players[PlayerId::One.index()].mana_pool.red = 1;
        game.apply(
            PlayerId::One,
            cast_action(heat_id, vec![Target::Permanent(angel_id)], Vec::new(), 0),
        )
        .expect("it can name a creature");
        drain_pending(&mut game);

        let dealt = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == angel_id)
            .map_or(6, |permanent| {
                i16::try_from(permanent.damage).unwrap_or(i16::MAX)
            });
        assert_eq!(
            dealt, expected,
            "{types} card types in your graveyard, and an artifact in theirs",
        );
    }
}

/// The static is the card: drawing from an empty library normally loses the
/// game, and with this Jace out it wins instead.
#[test]
fn jace_turns_an_empty_library_draw_into_a_win() {
    for jace_out in [false, true] {
        let mut game = ready_game();
        game.battlefield.clear();
        game.players[PlayerId::One.index()].library.clear();
        if jace_out {
            game.put_onto_battlefield(PlayerId::One, cards::JACE_WIELDER_OF_MYSTERIES)
                .expect("cataloged");
            drain_pending(&mut game);
        }

        game.draw_card(PlayerId::One);
        game.check_state_based_actions();

        assert_eq!(
            game.result(),
            Some(GameResult::Winner {
                winner: if jace_out {
                    PlayerId::One
                } else {
                    PlayerId::Two
                },
                reason: if jace_out {
                    WinReason::WonByAnEffect
                } else {
                    WinReason::OpponentTriedToDrawFromEmptyLibrary
                },
            }),
            "an empty draw with Jace out wins, and without him loses",
        );
    }
}

/// And the ultimate closes the same loop deliberately: seven cards out of a
/// seven-card library, then the win.
#[test]
fn jaces_ultimate_wins_once_it_has_emptied_the_library() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[PlayerId::One.index()].library.clear();
    for index in 0..7 {
        game.players[PlayerId::One.index()].library.push(card(
            81_000 + index,
            cards::GRIZZLY_BEARS,
            PlayerId::One,
        ));
    }
    let jace = game
        .put_onto_battlefield(PlayerId::One, cards::JACE_WIELDER_OF_MYSTERIES)
        .expect("cataloged");
    drain_pending(&mut game);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == jace)
        .expect("he entered")
        .add_counters(CounterKind::Loyalty, 4);

    // The third clause: the static, the plus, and then this one.
    let wanted = activated_ability_for(&game, jace, 1);
    let ultimate = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, ability, .. }
                if *source == jace && *ability == wanted)
        })
        .expect("eight loyalty pays for the ultimate");
    game.apply(PlayerId::One, ultimate).expect("it activates");
    drain_pending(&mut game);

    assert_eq!(
        game.players[PlayerId::One.index()].library.len(),
        0,
        "seven cards out of a seven-card library",
    );
    assert_eq!(
        game.result(),
        Some(GameResult::Winner {
            winner: PlayerId::One,
            reason: WinReason::WonByAnEffect,
        }),
        "and the ultimate's own check wins on the spot",
    );
}
