//! Bodyguards: damage aimed at a player that lands on a creature instead.
//!
//! Redirection happens before anything else looks at the damage (CR 614.9),
//! so the shields and preventions downstream all answer the creature that
//! took it rather than the player it was aimed at. And the condition rides
//! on the recipient, so tapping the bodyguard turns it off.

use super::*;

fn damage_on(game: &Game, permanent: GameObjectId) -> u16 {
    game.battlefield
        .iter()
        .find(|candidate| candidate.card.id == permanent)
        .map_or(0, |candidate| candidate.damage)
}

/// A bodyguard for player one, with an attacker for player two.
fn guarded_by(bodyguard: CardDefinitionId) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    let guard = creature(10_000, bodyguard, PlayerId::One);
    let guard_id = guard.card.id;
    game.battlefield.push(guard);
    let mut attacker = creature(10_001, cards::SEDGE_TROLL, PlayerId::Two);
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::One));
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);
    (game, guard_id, attacker_id)
}

#[test]
fn the_bodyguard_takes_what_the_unblocked_attacker_deals() {
    let (mut game, guard, _) = guarded_by(cards::VETERAN_BODYGUARD);
    let before = game.players[PlayerId::One.index()].life;

    game.deal_combat_damage();

    assert_eq!(game.players[PlayerId::One.index()].life, before);
    assert_eq!(damage_on(&game, guard), 2, "the 2/2 hit the bodyguard");
}

/// Tapping it turns the redirection off, and the player takes the hit.
#[test]
fn a_tapped_bodyguard_guards_nothing() {
    let (mut game, guard, _) = guarded_by(cards::VETERAN_BODYGUARD);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == guard)
        .expect("still there")
        .tapped = true;
    let before = game.players[PlayerId::One.index()].life;

    game.deal_combat_damage();

    assert_eq!(game.players[PlayerId::One.index()].life, before - 2);
    assert_eq!(damage_on(&game, guard), 0);
}

/// A blocked attacker is not unblocked, so Veteran Bodyguard ignores it --
/// and the question is asked as the damage arrives, not when it attacked.
#[test]
fn a_blocked_attacker_is_not_redirected() {
    let (mut game, guard, attacker) = guarded_by(cards::VETERAN_BODYGUARD);
    let mut blocker = creature(10_002, cards::SEDGE_TROLL, PlayerId::One);
    blocker.blocking = vec![attacker];
    game.battlefield.push(blocker);

    game.deal_combat_damage();

    assert_eq!(
        damage_on(&game, guard),
        0,
        "the blocker took it, not the bodyguard"
    );
}

/// Martyrs of Korlis names artifacts, so an attacking creature walks past it
/// and an artifact's ability does not.
#[test]
fn martyrs_of_korlis_guards_against_artifacts_only() {
    let (mut game, martyrs, _) = guarded_by(cards::MARTYRS_OF_KORLIS);
    let before = game.players[PlayerId::One.index()].life;

    game.deal_combat_damage();
    assert_eq!(
        game.players[PlayerId::One.index()].life,
        before - 2,
        "a creature is not an artifact"
    );
    assert_eq!(damage_on(&game, martyrs), 0);

    let ring = creature(10_002, cards::SOL_RING, PlayerId::Two);
    let ring_id = ring.card.id;
    game.battlefield.push(ring);
    let before = game.players[PlayerId::One.index()].life;

    game.damage_target_from(Some(ring_id), Some(Target::Player(PlayerId::One)), 3);

    assert_eq!(game.players[PlayerId::One.index()].life, before);
    assert_eq!(damage_on(&game, martyrs), 3, "the artifact's damage landed");
}

/// It guards its own controller, not the other player.
#[test]
fn a_bodyguard_does_not_guard_the_opponent() {
    let (mut game, guard, _) = guarded_by(cards::MARTYRS_OF_KORLIS);
    let ring = creature(10_002, cards::SOL_RING, PlayerId::One);
    let ring_id = ring.card.id;
    game.battlefield.push(ring);
    let before = game.players[PlayerId::Two.index()].life;

    game.damage_target_from(Some(ring_id), Some(Target::Player(PlayerId::Two)), 3);

    assert_eq!(game.players[PlayerId::Two.index()].life, before - 3);
    assert_eq!(damage_on(&game, guard), 0);
}

/// Shimian Night Stalker names one attacker for the turn rather than a
/// group, so a second attacker still gets through.
#[test]
fn the_night_stalker_guards_against_the_attacker_it_named() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    let stalker = creature(10_000, cards::SHIMIAN_NIGHT_STALKER, PlayerId::One);
    let stalker_id = stalker.card.id;
    game.battlefield.push(stalker);
    game.players[PlayerId::One.index()].mana_pool.black = 1;

    let mut named = creature(10_001, cards::SEDGE_TROLL, PlayerId::Two);
    named.attacking = true;
    named.attack_defender = Some(AttackDefender::Player(PlayerId::One));
    let named_id = named.card.id;
    game.battlefield.push(named);
    // A different size, so which attacker was redirected is visible in the
    // numbers rather than being symmetric.
    let mut other = creature(10_002, cards::SERRA_ANGEL, PlayerId::Two);
    other.attacking = true;
    other.attack_defender = Some(AttackDefender::Player(PlayerId::One));
    game.battlefield.push(other);

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == stalker_id
                    && targets
                        .iter()
                        .flat_map(crate::casting::TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(named_id))
            }
            _ => false,
        })
        .expect("the Stalker can name an attacker");
    game.apply(PlayerId::One, action)
        .expect("the ability activates");
    drain_pending(&mut game);

    let before = game.players[PlayerId::One.index()].life;
    game.deal_combat_damage();

    assert_eq!(
        before - game.players[PlayerId::One.index()].life,
        4,
        "the 4/4 it did not name got through"
    );
    assert_eq!(
        damage_on(&game, stalker_id),
        2,
        "and the 2/2 it named hit the Stalker"
    );
}
