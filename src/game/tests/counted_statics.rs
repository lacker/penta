//! Four statics whose answer is a battlefield count, or a composite applied
//! as one.
//!
//! Three read the board live: the Armor's bonus grows as enchantments arrive,
//! and the Jailbreaker's and the Thief's permissions come and go with the
//! Gate. The Keyrune is the odd one out -- its animation and its evasion are
//! one effect for one duration.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.battlefield.clear();
    game
}

fn stats(game: &Game, id: GameObjectId) -> (Option<i16>, Option<i16>) {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still there");
    (game.power(permanent), game.toughness(permanent))
}

/// The Armor counts itself, and every enchantment added after it.
#[test]
fn the_armor_recounts_your_enchantments() {
    let mut game = ready();
    let bear = creature(10_000, cards::GRIZZLY_BEARS, PlayerId::One);
    let bear_id = bear.card.id;
    game.battlefield.push(bear);
    let mut armor = creature(10_001, cards::ETHEREAL_ARMOR, PlayerId::One);
    armor.attached_to = Some(bear_id);
    game.battlefield.push(armor);

    assert_eq!(
        stats(&game, bear_id),
        (Some(3), Some(3)),
        "the Armor counts itself",
    );
    assert!(
        game.permanent_has_executable_keyword(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == bear_id)
                .expect("still there"),
            KeywordAbility::FirstStrike,
        ),
        "and the first strike is unconditional",
    );

    game.battlefield
        .push(creature(10_002, cards::INTANGIBLE_VIRTUE, PlayerId::One));
    assert_eq!(stats(&game, bear_id), (Some(4), Some(4)), "two now");

    // An opponent's enchantment is not yours.
    game.battlefield
        .push(creature(10_100, cards::INTANGIBLE_VIRTUE, PlayerId::Two));
    assert_eq!(stats(&game, bear_id), (Some(4), Some(4)), "still two");
}

/// The Jailbreaker keeps defender and gains permission only while a Gate is
/// out.
#[test]
fn the_jailbreaker_needs_a_gate_to_attack() {
    let mut game = ready();
    let ogre = creature(10_000, cards::OGRE_JAILBREAKER, PlayerId::One);
    let ogre_id = ogre.card.id;
    game.battlefield.push(ogre);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.step = Step::DeclareAttackers;

    let can_attack = |game: &Game| {
        game.legal_actions(PlayerId::One).iter().any(
            |action| matches!(action, Action::DeclareAttacker { attacker, .. } if *attacker == ogre_id),
        )
    };
    assert!(!can_attack(&game), "defender, and no Gate");

    game.put_onto_battlefield(PlayerId::One, cards::BOROS_GUILDGATE)
        .expect("cataloged");
    assert!(can_attack(&game), "a Gate unlocks it");

    assert!(
        game.permanent_has_executable_keyword(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == ogre_id)
                .expect("still there"),
            KeywordAbility::Defender,
        ),
        "it is a permission, not an ability removal",
    );

    let guildgate = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::BOROS_GUILDGATE)
        .expect("it is there")
        .card
        .id;
    game.battlefield
        .retain(|permanent| permanent.card.id != guildgate);
    assert!(!can_attack(&game), "and it locks again when the Gate goes");
}

fn blocked_by(game: &Game, attacker: GameObjectId, blocker: GameObjectId) -> bool {
    game.legal_actions(PlayerId::Two)
        .contains(&Action::DeclareBlocker { blocker, attacker })
}

/// Puts an attacker under a Way of the Thief opposite one blocker.
fn thief_attack() -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready();
    let mut attacker = creature(10_000, cards::GRIZZLY_BEARS, PlayerId::One);
    attacker.attacking = true;
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);
    let mut aura = creature(10_001, cards::WAY_OF_THE_THIEF, PlayerId::One);
    aura.attached_to = Some(attacker_id);
    game.battlefield.push(aura);
    let blocker = creature(10_100, cards::AIR_ELEMENTAL, PlayerId::Two);
    let blocker_id = blocker.card.id;
    game.battlefield.push(blocker);

    game.active_player = PlayerId::One;
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.priority = PlayerId::Two;
    (game, attacker_id, blocker_id)
}

/// The size is unconditional; only the evasion asks about the Gate.
#[test]
fn the_thief_needs_a_gate_only_for_the_evasion() {
    let (mut game, attacker, blocker) = thief_attack();
    assert_eq!(
        stats(&game, attacker),
        (Some(4), Some(4)),
        "a 2/2 with +2/+2, Gate or no Gate",
    );
    assert!(
        blocked_by(&game, attacker, blocker),
        "no Gate, so blockable"
    );

    game.put_onto_battlefield(PlayerId::One, cards::BOROS_GUILDGATE)
        .expect("cataloged");
    assert!(
        !blocked_by(&game, attacker, blocker),
        "a Gate makes it unblockable",
    );
    assert_eq!(stats(&game, attacker), (Some(4), Some(4)), "size unchanged");
}

/// The Keyrune's animation and its evasion are one effect for one duration.
#[test]
fn the_keyrune_animates_and_slips_through_together() {
    let mut game = ready();
    let keyrune = creature(10_000, cards::DIMIR_KEYRUNE, PlayerId::One);
    let keyrune_id = keyrune.card.id;
    game.battlefield.push(keyrune);
    game.players[PlayerId::One.index()].mana_pool.blue = 1;
    game.players[PlayerId::One.index()].mana_pool.black = 1;
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == keyrune_id))
        .expect("two mana covers it");
    game.apply(PlayerId::One, action).expect("legal");
    drain_pending(&mut game);

    assert_eq!(stats(&game, keyrune_id), (Some(2), Some(2)), "a 2/2 now");

    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == keyrune_id)
    {
        permanent.attacking = true;
    }
    let blocker = creature(10_100, cards::AIR_ELEMENTAL, PlayerId::Two);
    let blocker_id = blocker.card.id;
    game.battlefield.push(blocker);
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.priority = PlayerId::Two;

    assert!(
        !blocked_by(&game, keyrune_id, blocker_id),
        "and it cannot be blocked",
    );
}
