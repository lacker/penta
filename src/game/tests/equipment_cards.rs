//! Five Equipment whose audit lines said the equip procedure was missing.
//!
//! It was not: equip, the attachment relation, and the Equipment host rules
//! were all built. What these pin is the equip activation itself -- sorcery
//! speed, your own creature -- and the two conditional bonuses, which read
//! the equipped creature's type live.

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

/// Puts the Equipment and a creature out, then equips.
enum EquipmentHost {
    Card(CardDefinitionId),
    Token(TokenCharacteristics),
}

impl From<CardDefinitionId> for EquipmentHost {
    fn from(definition: CardDefinitionId) -> Self {
        Self::Card(definition)
    }
}

impl From<TokenCharacteristics> for EquipmentHost {
    fn from(token: TokenCharacteristics) -> Self {
        Self::Token(token)
    }
}

fn equip_onto(
    equipment: CardDefinitionId,
    host: impl Into<EquipmentHost>,
) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready();
    let gear = creature(10_000, equipment, PlayerId::One);
    let gear_id = gear.card.id;
    game.battlefield.push(gear);
    let creature_permanent = match host.into() {
        EquipmentHost::Card(definition) => creature(10_100, definition, PlayerId::One),
        EquipmentHost::Token(token) => token_permanent(10_100, token, PlayerId::One),
    };
    let host_id = creature_permanent.card.id;
    game.battlefield.push(creature_permanent);
    game.players[PlayerId::One.index()].mana_pool.colorless = 6;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == gear_id),
        )
        .expect("equip is offered");
    game.apply(PlayerId::One, action).expect("legal");
    drain_pending(&mut game);
    (game, gear_id, host_id)
}

#[test]
fn riot_gear_and_kitesail_hand_out_their_printed_bonuses() {
    let (game, _, host) = equip_onto(cards::RIOT_GEAR, cards::GRIZZLY_BEARS);
    assert_eq!(stats(&game, host), (Some(3), Some(4)), "a 2/2 with +1/+2");

    let (game, _, host) = equip_onto(cards::KITESAIL, cards::GRIZZLY_BEARS);
    assert_eq!(stats(&game, host), (Some(3), Some(2)));
    assert!(
        game.permanent_has_executable_keyword(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == host)
                .expect("still there"),
            KeywordAbility::Flying,
        ),
        "and the flying comes with it",
    );
}

#[test]
fn the_hood_hands_out_intimidate() {
    let (game, _, host) = equip_onto(cards::EXECUTIONERS_HOOD, cards::GRIZZLY_BEARS);
    assert!(
        game.permanent_has_executable_keyword(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == host)
                .expect("still there"),
            KeywordAbility::Intimidate,
        )
    );
}

/// The Mattock's second +1/+1 reads the equipped creature's type, so it is
/// worth one more on a Human than on anything else.
#[test]
fn the_mattock_pays_a_human_twice() {
    let (game, _, human) = equip_onto(cards::HEAVY_MATTOCK, cards::ELITE_INQUISITOR);
    assert_eq!(stats(&game, human), (Some(4), Some(4)), "a 2/2 with +2/+2");

    let (game, _, other) = equip_onto(cards::HEAVY_MATTOCK, cards::GRIZZLY_BEARS);
    assert_eq!(stats(&game, other), (Some(3), Some(3)), "+1/+1 only");
}

/// The Bracers' size is unconditional and only the vigilance reads the type.
#[test]
fn the_bracers_split_their_two_clauses() {
    let vigilant = |game: &Game, id: GameObjectId| {
        game.permanent_has_executable_keyword(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == id)
                .expect("still there"),
            KeywordAbility::Vigilance,
        )
    };

    // A Human with no printed vigilance, so the grant is the only source.
    let (game, _, human) = equip_onto(
        cards::BLADED_BRACERS,
        tokens::creature(&["Human"], &[ManaColor::White], 1, 1),
    );
    assert_eq!(stats(&game, human), (Some(2), Some(2)));
    assert!(vigilant(&game, human), "a Human gets the vigilance");

    let (game, _, zombie) = equip_onto(
        cards::BLADED_BRACERS,
        tokens::creature(&["Zombie"], &[ManaColor::Black], 2, 2),
    );
    assert_eq!(
        stats(&game, zombie),
        (Some(3), Some(3)),
        "the size is unconditional",
    );
    assert!(!vigilant(&game, zombie), "but the vigilance is not");
}

/// Equip is sorcery-speed and aims at your own creature.
#[test]
fn equip_is_restricted_to_your_own_creatures_at_sorcery_speed() {
    let mut game = ready();
    let gear = creature(10_000, cards::RIOT_GEAR, PlayerId::One);
    let gear_id = gear.card.id;
    game.battlefield.push(gear);
    let theirs = creature(10_100, cards::GRIZZLY_BEARS, PlayerId::Two);
    let theirs_id = theirs.card.id;
    game.battlefield.push(theirs);
    game.players[PlayerId::One.index()].mana_pool.colorless = 4;

    assert!(
        !game.legal_actions(PlayerId::One).iter().any(|action| {
            matches!(action, Action::ActivateAbility { source, targets, .. }
                if *source == gear_id
                    && targets.iter().flat_map(crate::TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(theirs_id)))
        }),
        "equip names a creature you control",
    );

    game.battlefield
        .push(creature(10_101, cards::GRIZZLY_BEARS, PlayerId::One));
    assert!(
        game.legal_actions(PlayerId::One).iter().any(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == gear_id)
        ),
        "and one of yours is fine",
    );

    game.step = Step::DeclareBlockers;
    assert!(
        !game.legal_actions(PlayerId::One).iter().any(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == gear_id)
        ),
        "equip only as a sorcery",
    );
}

/// The Pike recounts your graveyard continuously, so a spell arriving later
/// grows the creature without re-equipping.
#[test]
fn the_pike_recounts_the_graveyard() {
    let (mut game, _, host) = equip_onto(cards::RUNECHANTERS_PIKE, cards::GRIZZLY_BEARS);
    assert_eq!(stats(&game, host), (Some(2), Some(2)), "an empty graveyard");
    assert!(
        game.permanent_has_executable_keyword(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == host)
                .expect("still there"),
            KeywordAbility::FirstStrike,
        ),
        "the first strike is unconditional",
    );

    // A creature card in the graveyard is neither an instant nor a sorcery.
    game.players[PlayerId::One.index()].graveyard.push(card(
        30_000,
        cards::GRIZZLY_BEARS,
        PlayerId::One,
    ));
    assert_eq!(stats(&game, host), (Some(2), Some(2)), "wrong card type");

    for index in 0..2 {
        game.players[PlayerId::One.index()].graveyard.push(card(
            30_100 + index,
            cards::LIGHTNING_BOLT,
            PlayerId::One,
        ));
    }
    assert_eq!(stats(&game, host), (Some(4), Some(2)), "two instants");

    // The opponent's graveyard is not yours.
    game.players[PlayerId::Two.index()].graveyard.push(card(
        30_200,
        cards::LIGHTNING_BOLT,
        PlayerId::Two,
    ));
    assert_eq!(stats(&game, host), (Some(4), Some(2)), "still two");
}

/// The Trident forces the attack, and unequipping releases it.
#[test]
fn the_trident_forces_the_attack_while_it_is_on() {
    let (mut game, gear, host) = equip_onto(cards::TORMENTORS_TRIDENT, cards::GRIZZLY_BEARS);
    assert_eq!(stats(&game, host), (Some(5), Some(2)));

    game.step = Step::DeclareAttackers;
    let must_attack = |game: &Game| {
        !game
            .legal_actions(PlayerId::One)
            .contains(&Action::FinishDeclaringAttackers)
    };
    assert!(must_attack(&game), "the requirement is on");

    let index = game
        .battlefield
        .iter()
        .position(|permanent| permanent.card.id == gear)
        .expect("still there");
    game.battlefield[index].attached_to = None;
    assert!(!must_attack(&game), "and off again once unequipped");
}

/// The Shield's extra block is one more than the usual one.
#[test]
fn the_shield_buys_one_extra_block() {
    let (mut game, _, host) = equip_onto(cards::VANGUARDS_SHIELD, cards::GRIZZLY_BEARS);
    assert_eq!(stats(&game, host), (Some(2), Some(5)));

    game.active_player = PlayerId::Two;
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.priority = PlayerId::One;
    let mut attackers = Vec::new();
    for index in 0..3 {
        let mut attacker = creature(20_000 + index, cards::GRIZZLY_BEARS, PlayerId::Two);
        attacker.attacking = true;
        attackers.push(attacker.card.id);
        game.battlefield.push(attacker);
    }

    let offered = |game: &Game| {
        attackers
            .iter()
            .filter(|attacker| {
                game.legal_actions(PlayerId::One)
                    .contains(&Action::DeclareBlocker {
                        blocker: host,
                        attacker: **attacker,
                    })
            })
            .count()
    };
    assert_eq!(offered(&game), 3, "all three are candidates to begin with");

    game.apply(
        PlayerId::One,
        Action::DeclareBlocker {
            blocker: host,
            attacker: attackers[0],
        },
    )
    .expect("the first block is legal");
    assert_eq!(offered(&game), 2, "the Shield buys a second block");

    game.apply(
        PlayerId::One,
        Action::DeclareBlocker {
            blocker: host,
            attacker: attackers[1],
        },
    )
    .expect("the second block is legal");
    assert_eq!(offered(&game), 0, "but not a third");
}

/// The Quiver grants two abilities rather than one with a choice: only one of
/// them names a Werewolf, and both cost the creature's own tap.
#[test]
fn the_quiver_grants_two_abilities_that_share_one_tap() {
    let (mut game, _, host) = equip_onto(cards::WOLFHUNTERS_QUIVER, cards::GRIZZLY_BEARS);
    let werewolf = creature(10_200, cards::GATSTAF_SHEPHERD, PlayerId::Two);
    let werewolf_id = werewolf.card.id;
    game.battlefield.push(werewolf);
    let bystander = creature(10_201, cards::AIR_ELEMENTAL, PlayerId::Two);
    let bystander_id = bystander.card.id;
    game.battlefield.push(bystander);
    // Summoning sickness would stop a tap ability regardless of the grant.
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }

    let aiming_at = |game: &Game, victim: GameObjectId| {
        game.legal_actions(PlayerId::One)
            .iter()
            .filter(|action| {
                matches!(action, Action::ActivateAbility { source, targets, .. }
                    if *source == host
                        && targets.iter().flat_map(crate::TargetSelection::targets)
                            .any(|target| *target == Target::Permanent(victim)))
            })
            .count()
    };
    assert_eq!(
        aiming_at(&game, werewolf_id),
        2,
        "a Werewolf is a legal target for both",
    );
    assert_eq!(
        aiming_at(&game, bystander_id),
        1,
        "and anything else only for the any-target one",
    );

    let volley = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, targets, .. }
                if *source == host
                    && targets.iter().flat_map(crate::TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(bystander_id)))
        })
        .expect("the any-target ability is offered");
    game.apply(PlayerId::One, volley).expect("legal");
    drain_pending(&mut game);

    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == bystander_id)
            .expect("a 4/4 survives one")
            .damage,
        1,
    );
    assert_eq!(
        aiming_at(&game, werewolf_id),
        0,
        "the tap is spent, so neither ability is offered again",
    );
}
