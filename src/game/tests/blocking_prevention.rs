//! Prevention that names the damage's source by its relationship.
//!
//! "By creatures it's blocking" is read from the Wall, not from the
//! attacker: the attacker's own record does not name who is blocking it. And
//! "by enchanted creatures" is read off the battlefield, so an Aura arriving
//! or leaving changes the answer without the Wall being touched.

use super::*;

/// A big attacker for player one, blocked by `wall` for player two.
fn blocked_by(wall: CardDefinitionId) -> (Game, GameObjectId, GameObjectId) {
    blocked_by_attacker(cards::SERRA_ANGEL, wall)
}

fn blocked_by_attacker(
    attacker: CardDefinitionId,
    wall: CardDefinitionId,
) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    let mut attacker = creature(10_000, attacker, PlayerId::One);
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);

    let mut blocker = creature(10_001, wall, PlayerId::Two);
    blocker.blocking = vec![attacker_id];
    let blocker_id = blocker.card.id;
    game.battlefield.push(blocker);
    (game, attacker_id, blocker_id)
}

fn damage_on(game: &Game, permanent: GameObjectId) -> u16 {
    game.battlefield
        .iter()
        .find(|candidate| candidate.card.id == permanent)
        .map_or(0, |candidate| candidate.damage)
}

fn survives(game: &Game, permanent: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .any(|candidate| candidate.card.id == permanent)
}

#[test]
fn a_wall_takes_nothing_from_what_it_blocks() {
    let (mut game, _, wall) = blocked_by(cards::WALL_OF_VAPOR);
    game.deal_combat_damage();

    assert!(survives(&game, wall), "a 0/1 survives a 4/4 it blocked");
    assert_eq!(damage_on(&game, wall), 0);
}

/// The prevention names one relationship, not a blanket shield: damage from
/// something the Wall is not blocking still lands.
#[test]
fn damage_from_elsewhere_still_lands() {
    let (mut game, _, wall) = blocked_by(cards::WALL_OF_VAPOR);
    let other = creature(10_002, cards::SAVANNAH_LIONS, PlayerId::One);
    let other_id = other.card.id;
    game.battlefield.push(other);

    game.damage_target_from(Some(other_id), Some(Target::Permanent(wall)), 1);
    game.check_state_based_actions();

    assert!(
        !survives(&game, wall),
        "a creature it never blocked killed it"
    );
}

/// An ordinary blocker is not covered, which is what shows the effect is the
/// Wall's own rather than something about blocking.
#[test]
fn an_ordinary_blocker_takes_the_damage() {
    let (mut game, _, blocker) = blocked_by(cards::SAVANNAH_LIONS);
    game.deal_combat_damage();

    assert!(!survives(&game, blocker), "a 2/1 dies to a 4/4");
}

/// Wall of Putrid Flesh reads the battlefield: the same attacker is
/// prevented or not depending on whether an Aura is attached to it.
#[test]
fn an_aura_on_the_attacker_is_what_turns_the_prevention_on() {
    // A red attacker: the Wall's protection from white would answer a white
    // one before the prevention ever came up.
    let (mut game, attacker, wall) =
        blocked_by_attacker(cards::SHIVAN_DRAGON, cards::WALL_OF_PUTRID_FLESH);
    let mut aura = creature(10_002, cards::UNHOLY_STRENGTH, PlayerId::One);
    aura.attached_to = Some(attacker);
    game.battlefield.push(aura);
    game.check_state_based_actions();

    game.deal_combat_damage();
    assert!(
        survives(&game, wall),
        "an enchanted attacker cannot hurt it"
    );
    assert_eq!(damage_on(&game, wall), 0);

    let (mut game, _, wall) =
        blocked_by_attacker(cards::SHIVAN_DRAGON, cards::WALL_OF_PUTRID_FLESH);
    game.deal_combat_damage();
    assert!(
        !survives(&game, wall),
        "and an unenchanted one kills the 2/4"
    );
}

/// Enchanted Being names combat, so a burn spell from the same enchanted
/// creature still lands. That is the whole difference from Wall of Putrid
/// Flesh, which prevents all damage from one.
#[test]
fn enchanted_being_stops_combat_damage_only() {
    let (mut game, attacker, being) =
        blocked_by_attacker(cards::SHIVAN_DRAGON, cards::ENCHANTED_BEING);
    let mut aura = creature(10_002, cards::UNHOLY_STRENGTH, PlayerId::One);
    aura.attached_to = Some(attacker);
    game.battlefield.push(aura);
    game.check_state_based_actions();

    game.deal_combat_damage();
    assert_eq!(damage_on(&game, being), 0, "combat damage is prevented");

    game.damage_target_from(Some(attacker), Some(Target::Permanent(being)), 1);
    assert_eq!(
        damage_on(&game, being),
        1,
        "but an ability of the same creature still burns it"
    );
}

/// Demonic Torment prevents one direction. The enchanted creature deals
/// nothing, and still takes what its blocker deals back.
#[test]
fn demonic_torment_stops_only_what_its_host_deals() {
    let mut game = ready_game();
    let mut attacker = creature(10_000, cards::SHIVAN_DRAGON, PlayerId::One);
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);
    let mut torment = creature(10_001, cards::DEMONIC_TORMENT, PlayerId::Two);
    torment.attached_to = Some(attacker_id);
    game.battlefield.push(torment);
    let mut blocker = creature(10_002, cards::SERRA_ANGEL, PlayerId::Two);
    blocker.blocking = vec![attacker_id];
    let blocker_id = blocker.card.id;
    game.battlefield.push(blocker);
    game.check_state_based_actions();

    game.deal_combat_damage();

    assert_eq!(
        damage_on(&game, blocker_id),
        0,
        "the tormented creature deals nothing"
    );
    assert_eq!(
        damage_on(&game, attacker_id),
        4,
        "and still takes what the blocker deals"
    );
}

/// A shield printed on the creature itself rather than granted, and read the
/// same way: the Seraph takes nothing from combat at all.
#[test]
fn the_seraph_takes_no_combat_damage() {
    let (mut game, _, seraph) = blocked_by_attacker(cards::SEDGE_TROLL, cards::SERAPH_OF_THE_SWORD);
    game.deal_combat_damage();

    assert_eq!(damage_on(&game, seraph), 0);
    assert!(survives(&game, seraph));
}

/// Non-combat damage is a different event, and the Seraph's shield names
/// combat.
#[test]
fn the_seraph_still_takes_damage_from_a_spell() {
    let (mut game, _, seraph) = blocked_by_attacker(cards::SEDGE_TROLL, cards::SERAPH_OF_THE_SWORD);
    game.damage_target_from(None, Some(Target::Permanent(seraph)), 2);

    assert_eq!(damage_on(&game, seraph), 2);
}

/// The Transport's shield is narrower: it stops what its blockers deal and
/// nothing else, so a creature outside the block still connects.
#[test]
fn the_transport_only_stops_its_own_blockers() {
    // The Transport has to be the attacker: its shield names the creatures
    // blocking it, and nothing blocks a blocker.
    let (mut game, transport, _) =
        blocked_by_attacker(cards::ARMORED_TRANSPORT, cards::SEDGE_TROLL);
    game.deal_combat_damage();
    assert_eq!(damage_on(&game, transport), 0, "its blocker deals nothing");

    let other = creature(10_002, cards::SAVANNAH_LIONS, PlayerId::One);
    let other_id = other.card.id;
    game.battlefield.push(other);
    game.damage_target_from(Some(other_id), Some(Target::Permanent(transport)), 1);

    assert_eq!(
        damage_on(&game, transport),
        1,
        "a creature it never met still hits it"
    );
}

/// The other side of the same relationship, and the one that tells the
/// predicate apart from a blanket shield: a creature the Transport is
/// blocking is not a creature blocking the Transport, so its damage lands.
#[test]
fn the_transport_takes_damage_from_what_it_blocks() {
    let (mut game, _, transport) =
        blocked_by_attacker(cards::MONSS_GOBLIN_RAIDERS, cards::ARMORED_TRANSPORT);
    game.deal_combat_damage();

    // Its toughness is one, so the damage landing is the same fact as the
    // Transport not being there any more.
    assert!(
        !survives(&game, transport),
        "a blanket shield would have saved it"
    );
}

/// Defang stops everything its host would deal, not only combat damage, so
/// an activated ability is as harmless as an attack.
#[test]
fn defang_silences_its_host_entirely() {
    let mut game = ready_game();
    let troll = creature(10_000, cards::SEDGE_TROLL, PlayerId::One);
    let troll_id = troll.card.id;
    game.battlefield.push(troll);
    let victim = creature(10_001, cards::SERRA_ANGEL, PlayerId::Two);
    let victim_id = victim.card.id;
    game.battlefield.push(victim);

    game.damage_target_from(Some(troll_id), Some(Target::Permanent(victim_id)), 2);
    assert_eq!(damage_on(&game, victim_id), 2, "unarmed, it hits");

    let mut aura = creature(10_002, cards::DEFANG, PlayerId::Two);
    aura.attached_to = Some(troll_id);
    game.battlefield.push(aura);

    game.damage_target_from(Some(troll_id), Some(Target::Permanent(victim_id)), 2);
    assert_eq!(damage_on(&game, victim_id), 2, "and then it does not");
}

/// The two-sided Aura shield, which is two rules in one Apply: the host
/// neither deals combat damage nor takes it.
#[test]
fn ghostly_possession_shields_both_directions() {
    let (mut game, attacker, blocker) =
        blocked_by_attacker(cards::SEDGE_TROLL, cards::GRIZZLY_BEARS);
    let mut aura = creature(10_002, cards::GHOSTLY_POSSESSION, PlayerId::Two);
    aura.attached_to = Some(attacker);
    game.battlefield.push(aura);

    game.deal_combat_damage();

    assert_eq!(damage_on(&game, blocker), 0, "it dealt nothing");
    assert_eq!(damage_on(&game, attacker), 0, "and took nothing");
}
