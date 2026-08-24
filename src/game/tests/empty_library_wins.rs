//! The shared empty-library draw replacement used by Laboratory Maniac and
//! Jace, including its interaction with another replacement.

use super::*;

fn game_with(permanent: CardDefinitionId, library_cards: usize) -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[PlayerId::One.index()].library.clear();
    for offset in 0..library_cards {
        game.players[PlayerId::One.index()].library.push(card(
            85_000 + u32::try_from(offset).expect("a short library"),
            cards::GRIZZLY_BEARS,
            PlayerId::One,
        ));
    }
    game.put_onto_battlefield(PlayerId::One, permanent)
        .expect("cataloged");
    drain_pending(&mut game);
    game
}

/// Laboratory Maniac does not replace an ordinary draw. Once the library is
/// empty, the next draw is the one its static ability replaces with a win.
#[test]
fn laboratory_maniac_wins_only_when_the_library_is_empty() {
    let mut game = game_with(cards::LABORATORY_MANIAC, 1);

    assert!(game.draw_card(PlayerId::One).is_some());
    assert_eq!(game.result(), None, "a nonempty-library draw still happens");

    assert_eq!(game.draw_card(PlayerId::One), None);
    assert_eq!(
        game.result(),
        Some(GameResult::Winner {
            winner: PlayerId::One,
            reason: WinReason::WonByAnEffect,
        }),
    );
    assert!(
        !game.players[PlayerId::One.index()].tried_to_draw_from_empty_library,
        "the losing draw was replaced rather than attempted",
    );
}

/// Both effects replace the same prospective draw, so the affected player
/// chooses one. Choosing Island Sanctuary skips the draw without winning;
/// choosing Laboratory Maniac wins immediately.
#[test]
fn laboratory_maniac_competes_with_another_draw_replacement() {
    for choose_maniac in [false, true] {
        let mut game = game_with(cards::LABORATORY_MANIAC, 0);
        game.step = Step::Draw;
        game.active_player = PlayerId::One;
        game.put_onto_battlefield(PlayerId::One, cards::ISLAND_SANCTUARY)
            .expect("cataloged");
        drain_pending(&mut game);

        assert_eq!(game.draw_card(PlayerId::One), None);
        let decision = game.observe(PlayerId::One).decision.expect("a choice");
        assert_eq!(decision.options.len(), 2, "both replacements are offered");
        let selected = decision
            .options
            .iter()
            .find(|option| {
                option
                    .ability_text
                    .as_deref()
                    .is_some_and(|text| text.contains("win the game") == choose_maniac)
            })
            .expect("the requested replacement")
            .id;
        game.choose_decision(PlayerId::One, decision.id, &[selected]);

        if choose_maniac {
            assert_eq!(
                game.result(),
                Some(GameResult::Winner {
                    winner: PlayerId::One,
                    reason: WinReason::WonByAnEffect,
                }),
            );
        } else {
            assert_eq!(game.result(), None, "Sanctuary replaced the draw");
            assert_eq!(game.resolved_attack_restrictions.len(), 1);
        }
        assert!(
            !game.players[PlayerId::One.index()].tried_to_draw_from_empty_library,
            "either choice replaced the draw",
        );
    }
}
