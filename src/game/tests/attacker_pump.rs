//! A pump aimed at an attacking creature. Two things are easy to get wrong
//! here: the window, since the declare-attackers step offers only
//! declarations until the attack is locked in, and the target filter, which
//! must exclude the creature sitting at home -- including the one whose
//! ability it is.

use super::*;

/// Infantry Veteran and a Grizzly Bears under player one, with the Bears
/// attacking and the attack already declared.
fn staged() -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    let mut veteran = creature(45_000, cards::INFANTRY_VETERAN, PlayerId::One);
    veteran.entered_controller_turn = 0;
    let veteran_id = veteran.card.id;
    game.battlefield.push(veteran);
    let mut bear = creature(45_001, cards::GRIZZLY_BEARS, PlayerId::One);
    bear.entered_controller_turn = 0;
    bear.attacking = true;
    bear.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let bear_id = bear.card.id;
    game.battlefield.push(bear);
    game.active_player = PlayerId::One;
    game.step = Step::DeclareAttackers;
    game.attackers_declared = true;
    game.priority = PlayerId::One;
    (game, veteran_id, bear_id)
}

fn power(game: &Game, id: GameObjectId) -> i16 {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("the creature is on the battlefield");
    game.power(permanent).expect("power")
}

fn targets(game: &Game, source: GameObjectId) -> Vec<Target> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::ActivateAbility {
                source: activated,
                targets,
                ..
            } if activated == source => Some(targets),
            _ => None,
        })
        .flatten()
        .flat_map(|selection| selection.targets().to_vec())
        .collect()
}

#[test]
fn only_the_attacker_may_be_pumped() {
    let (game, veteran, bear) = staged();
    let offered = targets(&game, veteran);
    assert!(
        offered.contains(&Target::Permanent(bear)),
        "the attacking Bears are a legal target"
    );
    assert!(
        !offered.contains(&Target::Permanent(veteran)),
        "the Veteran stayed home, so it cannot pump itself"
    );
}

#[test]
fn the_attacker_grows_for_the_turn() {
    let (mut game, veteran, bear) = staged();
    assert_eq!(power(&game, bear), 2, "a plain Grizzly Bears to start");

    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == veteran
                    && targets
                        .iter()
                        .flat_map(TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(bear))
            }
            _ => false,
        })
        .expect("the Bears are a legal target for the pump");
    game.apply(PlayerId::One, activation)
        .expect("the tap pays for it");
    pass_priority_pair(&mut game);

    assert_eq!(power(&game, bear), 3, "the attacker is a point bigger");
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == veteran && permanent.tapped),
        "and the Veteran is tapped"
    );
}
