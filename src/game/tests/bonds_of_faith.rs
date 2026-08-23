//! Bonds of Faith's live Human/non-Human Aura branches.

use super::*;
use crate::ImplementationStatus;

fn power_toughness(game: &Game, id: GameObjectId) -> (Option<i16>, Option<i16>) {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("the creature is on the battlefield");
    (game.power(permanent), game.toughness(permanent))
}

fn can_block(game: &Game, blocker: GameObjectId) -> bool {
    game.legal_actions(PlayerId::One).iter().any(
        |action| matches!(action, Action::DeclareBlocker { blocker: actual, .. } if *actual == blocker),
    )
}

#[test]
fn bonds_of_faith_switches_live_between_its_two_branches() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;

    let human = creature(10_000, cards::AVACYNIAN_PRIEST, PlayerId::One);
    let human_id = human.card.id;
    game.battlefield.push(human);
    let bear = creature(10_001, cards::GRIZZLY_BEARS, PlayerId::One);
    let bear_id = bear.card.id;
    game.battlefield.push(bear);
    let mut bonds = creature(10_002, cards::BONDS_OF_FAITH, PlayerId::Two);
    let bonds_id = bonds.card.id;
    bonds.attached_to = Some(human_id);
    game.battlefield.push(bonds);

    assert_eq!(power_toughness(&game, human_id), (Some(3), Some(4)));
    assert!(
        game.can_attack(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == human_id)
                .expect("the Human is there")
        )
    );

    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == bonds_id)
        .expect("Bonds of Faith is there")
        .attached_to = Some(bear_id);

    assert_eq!(power_toughness(&game, human_id), (Some(1), Some(2)));
    assert_eq!(power_toughness(&game, bear_id), (Some(2), Some(2)));
    assert!(
        !game.can_attack(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == bear_id)
                .expect("the Bear is there")
        )
    );

    game.step = Step::DeclareBlockers;
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::One;
    let mut attacker = creature(10_003, cards::SEDGE_TROLL, PlayerId::Two);
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::One));
    game.battlefield.push(attacker);
    assert!(!can_block(&game, bear_id));

    game.battlefield
        .retain(|permanent| permanent.card.id != bonds_id);
    assert!(can_block(&game, bear_id));
}

#[test]
fn bonds_of_faith_is_fully_declarative_and_complete() {
    let catalog = poc::catalog().expect("catalog builds");
    let bonds = catalog
        .get(cards::BONDS_OF_FAITH)
        .expect("Bonds of Faith is cataloged");

    assert_eq!(
        bonds.rules.implementation_status(),
        ImplementationStatus::Complete
    );
    assert!(
        bonds
            .rules
            .ability_clauses()
            .iter()
            .all(|ability| ability.effect.execution == EffectExecutionDef::Declarative)
    );
}
