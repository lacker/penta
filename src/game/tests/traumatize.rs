//! Milling half of a library that is not yours.
//!
//! The count is read off the target as the spell resolves, so it is neither
//! a fixed number nor your own library. Rounded down, which only an odd
//! library can show.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.battlefield.clear();
    for player in [PlayerId::One, PlayerId::Two] {
        game.players[player.index()].library.clear();
        game.players[player.index()].graveyard.clear();
    }
    game
}

/// `mine` and `theirs` cards in the two libraries, with the spell in hand.
fn board(mine: usize, theirs: usize) -> (Game, GameObjectId) {
    let mut game = ready();
    for (player, count) in [(PlayerId::One, mine), (PlayerId::Two, theirs)] {
        for index in 0..count {
            let id = 20_000
                + u32::try_from(index).expect("small")
                + 100 * u32::try_from(player.index()).expect("two seats");
            game.players[player.index()]
                .library
                .push(card(id, cards::GRIZZLY_BEARS, player));
        }
    }
    let spell = card(10_000, cards::TRAUMATIZE, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.blue = 2;
    game.players[PlayerId::One.index()].mana_pool.colorless = 3;
    (game, spell_id)
}

fn cast_at(game: &mut Game, spell: GameObjectId, victim: PlayerId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == spell
                    && choices
                        .targets()
                        .iter()
                        .any(|selection| selection.targets().contains(&Target::Player(victim)))
            }
            _ => false,
        })
        .expect("that player can be named");
    game.apply(PlayerId::One, action).expect("it is cast");
    drain_pending(game);
}

/// Seven cards is three, not four and not half of the caster's library.
#[test]
fn it_mills_half_the_targets_library_rounded_down() {
    let (mut game, spell) = board(20, 7);
    cast_at(&mut game, spell, PlayerId::Two);

    assert_eq!(
        game.players[PlayerId::Two.index()].library.len(),
        4,
        "three of seven went",
    );
    assert_eq!(game.players[PlayerId::Two.index()].graveyard.len(), 3);
    assert_eq!(
        game.players[PlayerId::One.index()].library.len(),
        20,
        "and the caster's own library is untouched",
    );
}

/// It can be aimed at yourself, and then it reads your library instead.
#[test]
fn it_reads_whichever_library_it_was_aimed_at() {
    let (mut game, spell) = board(9, 20);
    cast_at(&mut game, spell, PlayerId::One);

    assert_eq!(
        game.players[PlayerId::One.index()].library.len(),
        5,
        "four of nine went",
    );
    assert_eq!(
        game.players[PlayerId::Two.index()].library.len(),
        20,
        "and the other library is untouched",
    );
}

/// An empty library mills nothing rather than failing.
#[test]
fn an_empty_library_mills_nothing() {
    let (mut game, spell) = board(20, 0);
    cast_at(&mut game, spell, PlayerId::Two);

    assert!(game.players[PlayerId::Two.index()].graveyard.is_empty());
}
