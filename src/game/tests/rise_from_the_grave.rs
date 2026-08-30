//! Reanimating something and saying what it now is.
//!
//! A permanent that enters the battlefield is a new object with a new
//! identity. The characteristic effect follows the move by explicitly
//! resolving the graveyard card's zone-change successor.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.battlefield.clear();
    game.players[PlayerId::One.index()].graveyard.clear();
    game.players[PlayerId::Two.index()].graveyard.clear();
    game
}

/// The spell in hand, mana to cast it, and `corpse` in `owner`'s graveyard.
fn board(corpse: CardDefinitionId, owner: PlayerId) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready();
    let body = card(20_000, corpse, owner);
    let body_id = body.id;
    game.players[owner.index()].graveyard.push(body);
    let spell = card(10_000, cards::RISE_FROM_THE_GRAVE, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.black = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 4;
    (game, spell_id, body_id)
}

fn cast_at(game: &mut Game, spell: GameObjectId, body: GameObjectId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == spell
                    && choices
                        .targets()
                        .iter()
                        .any(|selection| selection.targets().contains(&Target::Card(body)))
            }
            _ => false,
        })
        .expect("the corpse can be named");
    game.apply(PlayerId::One, action).expect("it is cast");
    drain_pending(game);
}

fn reanimated(game: &Game, definition: CardDefinitionId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == definition)
        .expect("it came back")
}

/// A green bear comes back as a black green Zombie Bear under the caster.
#[test]
fn it_adds_black_and_zombie_without_removing_what_was_there() {
    let (mut game, spell, body) = board(cards::GRIZZLY_BEARS, PlayerId::Two);
    cast_at(&mut game, spell, body);

    let permanent = reanimated(&game, cards::GRIZZLY_BEARS);
    let id = permanent.card.id;
    assert_eq!(
        permanent.controller,
        PlayerId::One,
        "under your control, not its owner's",
    );

    let colors = game.permanent_colors(permanent);
    let black = ManaColor::Black.color_index().expect("black is a colour");
    let green = ManaColor::Green.color_index().expect("green is a colour");
    assert!(colors[black], "black was added");
    assert!(colors[green], "and green is still there");

    let subtypes = game.object_subtypes(id);
    assert!(subtypes.contains(&"Zombie"), "a Zombie now");
    assert!(subtypes.contains(&"Bear"), "and still a Bear");
}

/// The object it lands on is a new one, so the following characteristic
/// effect must resolve the move's successor rather than reuse the old ID.
#[test]
fn the_permanent_is_not_the_card_that_was_in_the_graveyard() {
    let (mut game, spell, body) = board(cards::GRIZZLY_BEARS, PlayerId::Two);
    cast_at(&mut game, spell, body);

    assert_ne!(
        reanimated(&game, cards::GRIZZLY_BEARS).card.id,
        body,
        "a fresh identity on arrival",
    );
}

/// It reaches your own graveyard too, and an already-black creature simply
/// stays black.
#[test]
fn it_recurs_from_your_own_graveyard() {
    let (mut game, spell, body) = board(cards::SEDGE_TROLL, PlayerId::One);
    cast_at(&mut game, spell, body);

    let permanent = reanimated(&game, cards::SEDGE_TROLL);
    let id = permanent.card.id;
    assert_eq!(permanent.controller, PlayerId::One);
    assert!(
        game.permanent_colors(permanent)
            [ManaColor::Black.color_index().expect("black is a colour")],
        "still black",
    );
    assert!(game.object_subtypes(id).contains(&"Zombie"));
}
