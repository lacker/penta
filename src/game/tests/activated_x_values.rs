//! An X announced at activation and then read by the effect it pays for.
//! An X that fails to reach the effect resolves to zero, which looks like a
//! pump that simply does nothing -- so the size is measured at two
//! different values rather than merely observed once.

use super::*;

/// Cackling Witch and a Grizzly Bears under player one, with `mana` spare.
fn staged(mana: u16) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    let mut witch = creature(91_000, cards::CACKLING_WITCH, PlayerId::One);
    witch.entered_controller_turn = 0;
    game.battlefield.push(witch);
    let mut bear = creature(91_001, cards::GRIZZLY_BEARS, PlayerId::One);
    bear.entered_controller_turn = 0;
    let bear_id = bear.card.id;
    game.battlefield.push(bear);
    game.players[0]
        .hand
        .push(card(91_010, cards::MOUNTAIN, PlayerId::One));
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, mana);
    (game, bear_id)
}

/// Activates the Witch for `x` at the Bears and returns the resulting size.
fn pumped(x: u16) -> (i16, i16) {
    let (mut game, bear) = staged(x);
    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source,
                targets,
                x: chosen,
                ..
            } => {
                *source == GameObjectId(91_000)
                    && *chosen == x
                    && targets
                        .iter()
                        .flat_map(TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(bear))
            }
            _ => false,
        })
        .unwrap_or_else(|| panic!("an activation for X of {x} at the Bears is offered"));
    game.apply(PlayerId::One, activation)
        .expect("the cost is payable");
    for _ in 0..12 {
        drain_pending(&mut game);
        if game.stack.is_empty() {
            break;
        }
        let holder = game.priority;
        if game.apply(holder, Action::PassPriority).is_err() {
            break;
        }
    }
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == bear)
        .expect("the Bears is on the battlefield");
    (
        game.power(permanent).expect("power"),
        game.toughness(permanent).expect("toughness"),
    )
}

#[test]
fn an_x_of_zero_leaves_the_bears_alone() {
    assert_eq!(pumped(0), (2, 2), "nothing paid, nothing gained");
}

#[test]
fn the_pump_is_the_size_of_the_x() {
    assert_eq!(pumped(3), (5, 2), "three more power and no toughness");
}

/// Cho-Manno opposite a Serra Angel, after one combat.
fn cho_manno_blocks() -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::Two.index()] = 5;
    let mut cho = creature(91_100, cards::CHO_MANNO_REVOLUTIONARY, PlayerId::One);
    cho.entered_controller_turn = 0;
    let cho_id = cho.card.id;
    game.battlefield.push(cho);
    let mut angel = creature(91_101, cards::SERRA_ANGEL, PlayerId::Two);
    angel.attacking = true;
    angel.attack_defender = Some(AttackDefender::Player(PlayerId::One));
    let angel_id = angel.card.id;
    game.battlefield.push(angel);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == cho_id)
        .expect("Cho-Manno is on the battlefield")
        .blocking = vec![angel_id];
    game.active_player = PlayerId::Two;
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.finish_declaring_blockers();
    drain_pending(&mut game);
    game.step = Step::CombatDamage;
    game.begin_combat_damage_assignment();
    take_default_combat_assignment(&mut game);
    game.check_state_based_actions();
    game
}

#[test]
fn cho_manno_blocks_an_angel_and_lives() {
    let game = cho_manno_blocks();
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == GameObjectId(91_100)),
        "four damage is prevented in full"
    );
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == GameObjectId(91_101))
            .expect("the Angel is on the battlefield")
            .damage,
        2,
        "and Cho-Manno still deals its own damage out"
    );
}
