//! Two Arabian Nights lands that shoot at combat.
//!
//! Desert waits for the end-of-combat step, which is the whole card: it hits
//! something that already survived the damage rather than stopping it.
//! Island of Wak-Wak takes a flier's power off, which is the same base-power
//! setter Singing Tree wanted and the same reason it is only half of one.

use super::*;

/// A Serra Angel attacking player two, with `land` untapped on their side.
fn attacked_by_an_angel(land: CardDefinitionId) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    game.turns_started[PlayerId::Two.index()] = 5;
    game.active_player = PlayerId::One;
    let mut attacker = creature(10_000, cards::SERRA_ANGEL, PlayerId::One);
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);
    let defending_land = creature(10_001, land, PlayerId::Two);
    let land_id = defending_land.card.id;
    game.battlefield.push(defending_land);

    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.blockers_declared = true;
    game.priority = PlayerId::Two;
    (game, attacker_id, land_id)
}

fn shoot(game: &Game, land: GameObjectId) -> Option<Action> {
    game.legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => *source == land && !targets.is_empty(),
            _ => false,
        })
}

#[test]
fn the_desert_waits_for_the_end_of_combat_step() {
    let (mut game, _attacker, land) = attacked_by_an_angel(cards::DESERT);

    assert!(
        shoot(&game, land).is_none(),
        "the blockers step is too early"
    );

    game.step = Step::EndOfCombat;
    assert!(shoot(&game, land).is_some(), "and this is the window");
}

#[test]
fn the_desert_shoots_the_attacker() {
    let (mut game, attacker, land) = attacked_by_an_angel(cards::DESERT);
    game.step = Step::EndOfCombat;

    let action = shoot(&game, land).expect("an attacker to shoot");
    game.apply(PlayerId::Two, action)
        .expect("the ability activates");
    drain_pending(&mut game);

    let angel = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == attacker)
        .expect("a 4/4 survives one damage");
    assert_eq!(angel.damage, 1);
}

/// The window is a step, not a phase: once combat is over the land has
/// nothing to shoot and no window to shoot in.
#[test]
fn the_desert_is_shut_after_combat() {
    let (mut game, _attacker, land) = attacked_by_an_angel(cards::DESERT);
    game.step = Step::PostcombatMain;

    assert!(shoot(&game, land).is_none());
}

#[test]
fn the_island_takes_a_fliers_power_and_leaves_its_toughness() {
    let (mut game, attacker, land) = attacked_by_an_angel(cards::ISLAND_OF_WAK_WAK);

    let action = shoot(&game, land).expect("the Angel flies");
    game.apply(PlayerId::Two, action)
        .expect("the ability activates");
    drain_pending(&mut game);

    let angel = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == attacker)
        .expect("still there");
    assert_eq!(game.power(angel), Some(0));
    assert_eq!(game.toughness(angel), Some(4));
}

/// The control: it names fliers, so a ground creature is not a legal target.
#[test]
fn the_island_ignores_a_ground_creature() {
    let mut game = ready_game();
    game.turns_started[PlayerId::Two.index()] = 5;
    game.battlefield
        .push(creature(10_000, cards::SEDGE_TROLL, PlayerId::One));
    let land = creature(10_001, cards::ISLAND_OF_WAK_WAK, PlayerId::Two);
    let land_id = land.card.id;
    game.battlefield.push(land);
    game.priority = PlayerId::Two;

    assert!(shoot(&game, land_id).is_none());
}
