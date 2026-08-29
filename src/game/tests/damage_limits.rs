//! Damage that is capped rather than prevented.
//!
//! A limit has no capacity to spend and no follow-up: it applies to every
//! matching event for as long as its source is there. The two printed shapes
//! differ in what the cap depends on -- a flat number, or the recipient's
//! life when the damage would be dealt -- which is why the life-relative one
//! cannot be folded into the flat one.

use super::*;

fn burn(game: &mut Game, source: GameObjectId, amount: u16) -> i16 {
    let before = game.players[PlayerId::One.index()].life;
    game.damage_target_from(Some(source), Some(Target::Player(PlayerId::One)), amount);
    before - game.players[PlayerId::One.index()].life
}

/// Ali from Cairo under player one, with a burn source under player two.
fn ali_board() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::ALI_FROM_CAIRO, PlayerId::One));
    let burner = creature(10_001, cards::SOL_RING, PlayerId::Two);
    let burner_id = burner.card.id;
    game.battlefield.push(burner);
    (game, burner_id)
}

#[test]
fn ali_lets_ordinary_damage_through() {
    let (mut game, burner) = ali_board();
    assert_eq!(burn(&mut game, burner, 3), 3, "well above one life");
}

/// The cap depends on current life, so the same source dealing the same
/// amount is limited only once the life total is low enough.
#[test]
fn ali_stops_the_damage_at_one_life() {
    let (mut game, burner) = ali_board();
    game.players[PlayerId::One.index()].life = 4;

    assert_eq!(
        burn(&mut game, burner, 10),
        3,
        "down to one, and no further"
    );
    assert_eq!(game.players[PlayerId::One.index()].life, 1);
}

/// Already at one, nothing gets through at all.
#[test]
fn ali_holds_the_line_at_one() {
    let (mut game, burner) = ali_board();
    game.players[PlayerId::One.index()].life = 1;

    assert_eq!(burn(&mut game, burner, 5), 0);
    assert_eq!(game.players[PlayerId::One.index()].life, 1);
}

/// It protects its controller, not the other player.
#[test]
fn ali_does_not_protect_the_opponent() {
    let (mut game, _) = ali_board();
    game.players[PlayerId::Two.index()].life = 2;
    let ring = creature(10_002, cards::SOL_RING, PlayerId::One);
    let ring_id = ring.card.id;
    game.battlefield.push(ring);

    game.damage_target_from(Some(ring_id), Some(Target::Player(PlayerId::Two)), 5);

    assert!(
        game.players[PlayerId::Two.index()].life <= 0,
        "the other player has no such protection"
    );
}

/// Forethought Amulet caps at two, and only for instant and sorcery sources.
#[test]
fn the_amulet_caps_spell_damage_and_leaves_the_rest() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::FORETHOUGHT_AMULET, PlayerId::One));
    let permanent_source = creature(10_001, cards::SOL_RING, PlayerId::Two);
    let permanent_id = permanent_source.card.id;
    game.battlefield.push(permanent_source);

    assert_eq!(
        burn(&mut game, permanent_id, 5),
        5,
        "an artifact is neither an instant nor a sorcery"
    );

    // A real Lightning Bolt: the source has to be the spell on the stack for
    // its type to be readable at all.
    let bolt = card(10_002, cards::LIGHTNING_BOLT, PlayerId::Two);
    let bolt_id = bolt.id;
    game.players[PlayerId::Two.index()].hand.push(bolt);
    game.players[PlayerId::Two.index()].mana_pool.red = 1;
    game.priority = PlayerId::Two;

    let before = game.players[PlayerId::One.index()].life;
    let action = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == bolt_id
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Player(PlayerId::One))
            }
            _ => false,
        })
        .expect("the Bolt can be aimed at the other player");
    game.apply(PlayerId::Two, action).expect("the Bolt is cast");
    drain_pending(&mut game);

    assert_eq!(
        before - game.players[PlayerId::One.index()].life,
        2,
        "a Bolt's three becomes two"
    );
}
