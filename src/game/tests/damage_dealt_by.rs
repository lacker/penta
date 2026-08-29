//! "Whenever this deals damage" uses the source axis of the canonical damage
//! event. Incoming-damage clauses use its recipient axis instead; the Aura
//! case names an attached host on the same event rather than adding another
//! direction-specific event kind.

use super::*;

#[test]
fn el_hajjaj_gains_life_equal_to_what_it_deals() {
    let mut game = ready_game();
    let hajjaj = creature(10_000, cards::EL_HAJJAJ, PlayerId::One);
    let hajjaj_id = hajjaj.card.id;
    game.battlefield.push(hajjaj);
    let victim = creature(10_001, cards::SEDGE_TROLL, PlayerId::Two);
    let victim_id = victim.card.id;
    game.battlefield.push(victim);

    game.damage_target_from(Some(hajjaj_id), Some(Target::Permanent(victim_id)), 3);
    drain_pending(&mut game);

    assert_eq!(
        game.players[PlayerId::One.index()].life,
        i16::from(rules::STARTING_LIFE) + 3,
        "three dealt, three gained"
    );
}

/// Damage dealt *to* it is not damage dealt *by* it, which is the whole
/// distinction the new event draws.
#[test]
fn el_hajjaj_gains_nothing_from_damage_it_takes() {
    let mut game = ready_game();
    let hajjaj = creature(10_000, cards::EL_HAJJAJ, PlayerId::One);
    let hajjaj_id = hajjaj.card.id;
    game.battlefield.push(hajjaj);
    let attacker = creature(10_001, cards::SEDGE_TROLL, PlayerId::Two);
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);

    game.damage_target_from(Some(attacker_id), Some(Target::Permanent(hajjaj_id)), 1);
    drain_pending(&mut game);

    assert_eq!(
        game.players[PlayerId::One.index()].life,
        i16::from(rules::STARTING_LIFE),
    );
}

/// Spirit Link watches its host, and the life goes to the Aura's controller
/// rather than the creature's -- which is why it is not lifelink.
#[test]
fn spirit_link_pays_the_auras_controller_not_the_creatures() {
    let mut game = ready_game();
    let troll = creature(10_000, cards::SEDGE_TROLL, PlayerId::Two);
    let troll_id = troll.card.id;
    game.battlefield.push(troll);
    let mut link = creature(10_001, cards::SPIRIT_LINK, PlayerId::One);
    link.attached_to = Some(troll_id);
    game.battlefield.push(link);
    let bystander = creature(10_002, cards::SAVANNAH_LIONS, PlayerId::One);
    let bystander_id = bystander.card.id;
    game.battlefield.push(bystander);

    game.damage_target_from(Some(troll_id), Some(Target::Permanent(bystander_id)), 2);
    drain_pending(&mut game);

    assert_eq!(
        game.players[PlayerId::One.index()].life,
        i16::from(rules::STARTING_LIFE) + 2,
        "the Aura's controller gains the life"
    );
    assert_eq!(
        game.players[PlayerId::Two.index()].life,
        i16::from(rules::STARTING_LIFE),
        "and the creature's controller gains none"
    );
}

/// The host is read live, so a creature the Aura is not on does not trigger.
#[test]
fn spirit_link_ignores_a_creature_it_is_not_attached_to() {
    let mut game = ready_game();
    let troll = creature(10_000, cards::SEDGE_TROLL, PlayerId::Two);
    let troll_id = troll.card.id;
    game.battlefield.push(troll);
    let other = creature(10_001, cards::SEDGE_TROLL, PlayerId::Two);
    let other_id = other.card.id;
    game.battlefield.push(other);
    let mut link = creature(10_002, cards::SPIRIT_LINK, PlayerId::One);
    link.attached_to = Some(troll_id);
    game.battlefield.push(link);

    game.damage_target_from(Some(other_id), Some(Target::Player(PlayerId::One)), 2);
    drain_pending(&mut game);

    assert_eq!(
        game.players[PlayerId::One.index()].life,
        i16::from(rules::STARTING_LIFE) - 2,
        "the unenchanted creature's damage gains nothing"
    );
}
