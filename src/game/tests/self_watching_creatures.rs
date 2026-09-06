//! Two triggers a creature points at itself or at everything but itself. A
//! "whenever this blocks" clause needs the blocking creature to be the
//! ability's own source, and an "another creature enters" clause has to
//! miss its own arrival -- both of which look identical to a clause that
//! never fires.

use super::*;

/// Shield Sphere blocking a Grizzly Bears of player two's.
fn shield_blocks() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::Two.index()] = 5;
    let mut attacker = creature(93_000, cards::GRIZZLY_BEARS, PlayerId::Two);
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::One));
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);
    let mut sphere = creature(93_001, cards::SHIELD_SPHERE, PlayerId::One);
    sphere.entered_controller_turn = 0;
    sphere.blocking = vec![attacker_id];
    let sphere_id = sphere.card.id;
    game.battlefield.push(sphere);
    game.active_player = PlayerId::Two;
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.finish_declaring_blockers();
    for _ in 0..12 {
        drain_pending(&mut game);
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let holder = game.priority;
        if game.apply(holder, Action::PassPriority).is_err() {
            break;
        }
    }
    (game, sphere_id)
}

#[test]
fn blocking_wears_the_sphere_down() {
    let (game, sphere) = shield_blocks();
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == sphere)
        .expect("the Sphere is on the battlefield");
    assert_eq!(
        (
            game.power(permanent).expect("power"),
            game.toughness(permanent).expect("toughness")
        ),
        (0, 5),
        "one block, one counter, one less toughness"
    );
}

/// A Soul Warden of player one's, then `arrivals` creatures cast by
/// `caster`, returning the life player one ends on.
fn warden_life(arrivals: &[CardDefinitionId], caster: PlayerId) -> i16 {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[1].hand.clear();
    game.turns_started[caster.index()] = 5;
    game.active_player = caster;
    game.step = Step::PrecombatMain;
    game.priority = caster;
    let mut warden = creature(93_100, cards::SOUL_WARDEN, PlayerId::One);
    warden.entered_controller_turn = 0;
    game.battlefield.push(warden);
    for (index, definition) in arrivals.iter().enumerate() {
        let held = card(
            93_200 + u32::try_from(index).expect("a small fixture"),
            *definition,
            caster,
        );
        let held_id = held.id;
        game.players[caster.index()].hand.push(held);
        for color in [ManaColor::White, ManaColor::Green] {
            game.add_unrestricted_mana(caster, color, 2);
        }
        game.add_unrestricted_mana(caster, ManaColor::Colorless, 4);
        let cast = game
            .legal_actions(caster)
            .into_iter()
            .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == held_id))
            .expect("the creature is castable");
        game.apply(caster, cast).expect("the cast is legal");
        for _ in 0..8 {
            drain_pending(&mut game);
            if game.stack.is_empty() && game.pending_triggers.is_empty() {
                break;
            }
            let holder = game.priority;
            if game.apply(holder, Action::PassPriority).is_err() {
                break;
            }
        }
    }
    game.players[0].life
}

#[test]
fn the_warden_counts_creatures_from_either_side() {
    assert_eq!(
        warden_life(&[cards::GRIZZLY_BEARS], PlayerId::One),
        21,
        "my own creature is another creature"
    );
    assert_eq!(
        warden_life(&[cards::GRIZZLY_BEARS], PlayerId::Two),
        21,
        "and so is the opponent's"
    );
}

#[test]
fn a_second_warden_pays_for_the_first_but_not_itself() {
    assert_eq!(
        warden_life(&[cards::SOUL_WARDEN], PlayerId::One),
        21,
        "the Warden already out sees the new one arrive, and the new one \
         does not see itself"
    );
}
