//! "Can attack as though it didn't have defender."
//!
//! A permission, not an ability removal. The Wall keeps the keyword, so
//! anything reading "a creature with defender" still finds one -- and every
//! other reason it cannot attack is untouched, which is what summoning
//! sickness demonstrates.

use super::*;

fn wall_board(definition: CardDefinitionId) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    let wall = creature(10_000, definition, PlayerId::One);
    let wall_id = wall.card.id;
    game.battlefield.push(wall);
    (game, wall_id)
}

fn permanent(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("the permanent is on the battlefield")
}

fn can_attack(game: &Game, id: GameObjectId) -> bool {
    game.can_attack(permanent(game, id))
}

#[test]
fn the_aura_lets_its_wall_attack() {
    let (mut game, wall) = wall_board(cards::WALL_OF_STONE);
    assert!(!can_attack(&game, wall), "a Wall stays home on its own");

    let mut aura = creature(10_001, cards::ANIMATE_WALL, PlayerId::One);
    aura.attached_to = Some(wall);
    game.battlefield.push(aura);

    assert!(can_attack(&game, wall));
}

/// The keyword is still there, which is the difference between a permission
/// and losing the ability.
#[test]
fn the_wall_still_has_defender() {
    let (mut game, wall) = wall_board(cards::WALL_OF_STONE);
    let mut aura = creature(10_001, cards::ANIMATE_WALL, PlayerId::One);
    aura.attached_to = Some(wall);
    game.battlefield.push(aura);

    assert!(
        game.permanent_has_executable_keyword(permanent(&game, wall), KeywordAbility::Defender),
        "the permission does not take the keyword away"
    );
}

/// Everything else that stops an attack still does. A Wall that arrived this
/// turn is not going anywhere.
#[test]
fn the_permission_does_not_cure_summoning_sickness() {
    let (mut game, wall) = wall_board(cards::WALL_OF_STONE);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == wall)
        .expect("the Wall is there")
        .entered_controller_turn = game.turns_started[PlayerId::One.index()];
    let mut aura = creature(10_001, cards::ANIMATE_WALL, PlayerId::One);
    aura.attached_to = Some(wall);
    game.battlefield.push(aura);

    assert!(!can_attack(&game, wall));
}

/// The Aura is what grants it, so removing the Aura takes it back.
#[test]
fn detaching_the_aura_sits_the_wall_back_down() {
    let (mut game, wall) = wall_board(cards::WALL_OF_STONE);
    let mut aura = creature(10_001, cards::ANIMATE_WALL, PlayerId::One);
    let aura_id = aura.card.id;
    aura.attached_to = Some(wall);
    game.battlefield.push(aura);
    assert!(can_attack(&game, wall));

    game.battlefield
        .retain(|permanent| permanent.card.id != aura_id);

    assert!(!can_attack(&game, wall));
}

/// Wall of Wonder grants itself the same permission, and pays for it with
/// four toughness in the same effect.
#[test]
fn the_wall_of_wonder_charges_itself_up() {
    let (mut game, wall) = wall_board(cards::WALL_OF_WONDER);
    assert!(!can_attack(&game, wall));
    game.players[PlayerId::One.index()].mana_pool.blue = 2;
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == wall))
        .expect("the charge is affordable");
    game.apply(PlayerId::One, action)
        .expect("the ability is activated");
    drain_pending(&mut game);

    assert!(can_attack(&game, wall));
    assert_eq!(game.power(permanent(&game, wall)), Some(5));
    assert_eq!(game.toughness(permanent(&game, wall)), Some(1));
}

/// A trigger can hand the permission out too, and it ends with the turn like
/// any other resolved rule.
#[test]
fn a_spell_cast_can_free_the_cyclops_for_the_turn() {
    let (mut game, cyclops) = wall_board(cards::NIVIX_CYCLOPS);
    assert!(!can_attack(&game, cyclops), "defender holds it back");

    let bolt = card(10_001, cards::LIGHTNING_BOLT, PlayerId::One);
    let bolt_id = bolt.id;
    game.players[PlayerId::One.index()].hand.push(bolt);
    game.players[PlayerId::One.index()].mana_pool.red = 1;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == bolt_id))
        .expect("the Bolt can be cast");
    game.apply(PlayerId::One, action).expect("the Bolt is cast");
    drain_pending(&mut game);

    assert!(can_attack(&game, cyclops), "and the trigger frees it");
    assert_eq!(game.power(permanent(&game, cyclops)), Some(4));
    assert!(
        game.permanent_has_executable_keyword(permanent(&game, cyclops), KeywordAbility::Defender),
        "still a defender, still allowed to swing"
    );
}
