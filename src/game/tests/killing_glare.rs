//! A target predicate measured against the spell's own chosen X.
//!
//! Which creatures are legal targets depends on how much was paid, and the
//! spell has no stack object yet when that is asked -- so the enumerator,
//! which already walks one X at a time, says which X it is considering.
//! "Power X or less" is inclusive at the boundary.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.battlefield.clear();
    game.players[PlayerId::One.index()].hand.clear();
    game
}

/// The Glare in hand with `mana` black available, and one creature of each
/// listed definition under the opponent.
fn board(mana: u16, victims: &[CardDefinitionId]) -> (Game, GameObjectId, Vec<GameObjectId>) {
    let mut game = ready();
    let mut ids = Vec::new();
    for (index, definition) in victims.iter().enumerate() {
        let permanent = creature(
            10_000 + u32::try_from(index).expect("small"),
            *definition,
            PlayerId::Two,
        );
        ids.push(permanent.card.id);
        game.battlefield.push(permanent);
    }
    let glare = card(20_000, cards::KILLING_GLARE, PlayerId::One);
    let glare_id = glare.id;
    game.players[PlayerId::One.index()].hand.push(glare);
    game.players[PlayerId::One.index()].mana_pool.black = mana;
    (game, glare_id, ids)
}

/// Every creature the Glare could be aimed at when X is `x`.
fn targets_at(game: &Game, glare: GameObjectId, x: u16) -> Vec<GameObjectId> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, choices, .. } if card == glare && choices.x() == x => {
                choices.iter_targets().find_map(|target| match target {
                    Target::Permanent(id) => Some(*id),
                    _ => None,
                })
            }
            _ => None,
        })
        .collect()
}

/// A 2/2 is reachable at X of two and not at X of one: the boundary is
/// inclusive.
#[test]
fn the_power_limit_includes_x_itself() {
    let (game, glare, victims) = board(4, &[cards::GRIZZLY_BEARS]);
    let bear = victims[0];

    assert!(
        !targets_at(&game, glare, 1).contains(&bear),
        "a 2/2 is out of reach at one",
    );
    assert!(
        targets_at(&game, glare, 2).contains(&bear),
        "and in reach at two",
    );
}

/// A bigger creature stays out of reach until X catches up.
#[test]
fn a_bigger_creature_needs_a_bigger_x() {
    let (game, glare, victims) = board(6, &[cards::GRIZZLY_BEARS, cards::SERRA_ANGEL]);
    let (bear, angel) = (victims[0], victims[1]);

    let at_two = targets_at(&game, glare, 2);
    assert!(at_two.contains(&bear), "the 2/2 is reachable");
    assert!(!at_two.contains(&angel), "the 4/4 is not");

    assert!(
        targets_at(&game, glare, 4).contains(&angel),
        "until X reaches four",
    );
}

/// At X of zero it still kills something, which is what makes it playable
/// off one mana.
#[test]
fn x_of_zero_still_reaches_a_zero_power_creature() {
    let (game, glare, victims) = board(1, &[cards::WALL_OF_STONE, cards::GRIZZLY_BEARS]);
    let (wall, bear) = (victims[0], victims[1]);

    let at_zero = targets_at(&game, glare, 0);
    assert!(at_zero.contains(&wall), "a 0/8 has power zero");
    assert!(!at_zero.contains(&bear), "and a 2/2 does not");
}

/// Casting it really destroys what it named.
#[test]
fn it_destroys_the_creature_it_named() {
    let (mut game, glare, victims) = board(3, &[cards::GRIZZLY_BEARS]);
    let bear = victims[0];
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == glare
                    && choices.x() == 2
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Permanent(bear))
            }
            _ => false,
        })
        .expect("the bear is reachable at two");
    game.apply(PlayerId::One, action).expect("it is cast");
    drain_pending(&mut game);

    assert!(game.battlefield.is_empty(), "the bear was destroyed");
}
