//! Two Zombies that collect what they killed.
//!
//! "Dealt damage by this creature this turn" is not "died in combat with
//! it": the damage and the death can be far apart, and the creature that
//! died must have been damaged by *this* source rather than any. Both cards
//! read the corpse afterwards, one for its toughness and one to put it back.

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

/// The named Zombie under player one and a Wall of Stone under player two,
/// which is 0/8 so its toughness is unmistakable.
fn board(zombie: CardDefinitionId) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready();
    let source = creature(10_000, zombie, PlayerId::One);
    let source_id = source.card.id;
    game.battlefield.push(source);
    let wall = creature(10_001, cards::WALL_OF_STONE, PlayerId::Two);
    let wall_id = wall.card.id;
    game.battlefield.push(wall);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    (game, source_id, wall_id)
}

/// Marks the Wall as damaged by `source`, then kills it outright, so the
/// damage and the death are separate events.
fn damage_then_destroy(game: &mut Game, source: GameObjectId, victim: GameObjectId) {
    game.damage_target_from(Some(source), Some(Target::Permanent(victim)), 1);
    game.move_permanents_to_graveyard(&[victim]);
    drain_pending(game);
}

/// Eight toughness, read after the Wall is already in a graveyard.
#[test]
fn the_ghoul_gains_the_dead_creatures_toughness() {
    let (mut game, ghoul, wall) = board(cards::ABATTOIR_GHOUL);
    let before = game.players[PlayerId::One.index()].life;
    damage_then_destroy(&mut game, ghoul, wall);

    assert_eq!(
        game.players[PlayerId::One.index()].life,
        before + 8,
        "the Wall's eight toughness, not its zero power",
    );
}

/// A creature this Ghoul never touched is worth nothing.
#[test]
fn the_ghoul_ignores_a_creature_it_did_not_damage() {
    let (mut game, _ghoul, wall) = board(cards::ABATTOIR_GHOUL);
    let before = game.players[PlayerId::One.index()].life;
    let bystander = creature(10_002, cards::GRIZZLY_BEARS, PlayerId::Two);
    let bystander_id = bystander.card.id;
    game.battlefield.push(bystander);

    // Damaged by the Wall rather than by the Ghoul.
    game.damage_target_from(Some(wall), Some(Target::Permanent(bystander_id)), 1);
    game.move_permanents_to_graveyard(&[bystander_id]);
    drain_pending(&mut game);

    assert_eq!(
        game.players[PlayerId::One.index()].life,
        before,
        "somebody else's kill",
    );
}

/// The Slaver takes the corpse and repaints it.
#[test]
fn the_slaver_reanimates_what_it_killed_as_a_black_zombie() {
    let (mut game, slaver, wall) = board(cards::DREAD_SLAVER);
    damage_then_destroy(&mut game, slaver, wall);

    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::WALL_OF_STONE)
        .expect("the Wall came back");
    let id = permanent.card.id;
    assert_eq!(
        permanent.controller,
        PlayerId::One,
        "under the Slaver's controller, not the Wall's owner",
    );
    assert!(
        game.permanent_colors(permanent)
            [ManaColor::Black.color_index().expect("black is a colour")],
        "black now",
    );
    assert!(game.object_subtypes(id).contains(&"Zombie"), "a Zombie now");
    assert!(
        game.object_subtypes(id).contains(&"Wall"),
        "and still a Wall"
    );
}

/// Mortus Strider's printed "return it" finds the destination object recorded
/// by the death event; an ordinary source reference keeps naming the retired
/// permanent instead.
#[test]
fn a_dying_creature_that_returns_itself_finds_its_own_card() {
    let mut game = ready();
    let strider = creature(10_000, cards::MORTUS_STRIDER, PlayerId::One);
    let strider_id = strider.card.id;
    game.battlefield.push(strider);
    let before = game.players[PlayerId::One.index()].hand.len();

    game.move_permanents_to_graveyard(&[strider_id]);
    drain_pending(&mut game);

    assert_eq!(
        game.players[PlayerId::One.index()].hand.len(),
        before + 1,
        "it came back to hand",
    );
    assert!(
        game.players[PlayerId::One.index()].graveyard.is_empty(),
        "and did not stay in the graveyard",
    );
}

/// The exception crosses the death event only. Moving the card again makes
/// another new object, so the pending trigger cannot chase it into exile.
#[test]
fn a_return_trigger_does_not_follow_a_second_zone_change() {
    let mut game = ready();
    let strider = creature(10_000, cards::MORTUS_STRIDER, PlayerId::One);
    let strider_id = strider.card.id;
    game.battlefield.push(strider);
    let hand_before = game.players[PlayerId::One.index()].hand.len();

    game.move_permanents_to_graveyard(&[strider_id]);
    let graveyard_id = game.players[PlayerId::One.index()]
        .graveyard
        .iter()
        .find(|card| card.definition == cards::MORTUS_STRIDER)
        .expect("the death created a graveyard card")
        .id;
    game.move_target_to_zone(
        Target::Card(graveyard_id),
        ZoneKind::Exile,
        ZoneMoveCause::Effect {
            controller: PlayerId::Two,
        },
        None,
        ZonePlacement::Top,
    );
    drain_pending(&mut game);

    assert!(
        game.players[PlayerId::One.index()]
            .exile
            .iter()
            .any(|card| card.definition == cards::MORTUS_STRIDER),
        "the trigger did not follow the card's second move",
    );
    assert_eq!(
        game.players[PlayerId::One.index()].hand.len(),
        hand_before,
        "the card was not returned to hand",
    );
}
