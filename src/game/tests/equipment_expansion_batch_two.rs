//! The second ISD–DGM Equipment completion batch: live affected-color
//! values, two-sided block matching, subtype removal and attachment SBAs,
//! and an Equipment that unattaches itself before transforming.

use super::*;
use crate::ImplementationStatus;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game
}

fn permanent(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("the permanent remains on the battlefield")
}

fn attached_board(
    equipment: CardDefinitionId,
    host: CardDefinitionId,
) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready();
    let host = creature(10_100, host, PlayerId::One);
    let host_id = host.card.id;
    let mut equipment = creature(10_000, equipment, PlayerId::One);
    let equipment_id = equipment.card.id;
    equipment.attached_to = Some(host_id);
    game.battlefield.extend([equipment, host]);
    (game, equipment_id, host_id)
}

fn commit_equipped_block(equipped_attacks: bool, other: CardDefinitionId) -> (Game, GameObjectId) {
    let mut game = ready();
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;

    let attacker_controller = if equipped_attacks {
        PlayerId::One
    } else {
        PlayerId::Two
    };
    let blocker_controller = attacker_controller.opponent();
    let attacker_definition = if equipped_attacks {
        cards::GRIZZLY_BEARS
    } else {
        other
    };
    let blocker_definition = if equipped_attacks {
        other
    } else {
        cards::GRIZZLY_BEARS
    };

    let mut attacker = creature(10_100, attacker_definition, attacker_controller);
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(blocker_controller));
    let attacker_id = attacker.card.id;
    let mut blocker = creature(10_200, blocker_definition, blocker_controller);
    blocker.blocking = vec![attacker_id];
    let blocker_id = blocker.card.id;
    let host_id = if equipped_attacks {
        attacker_id
    } else {
        blocker_id
    };
    let other_id = if equipped_attacks {
        blocker_id
    } else {
        attacker_id
    };
    let mut stake = creature(
        10_000,
        cards::WOODEN_STAKE,
        host_id_player(equipped_attacks),
    );
    stake.attached_to = Some(host_id);
    game.battlefield.extend([stake, attacker, blocker]);
    game.finish_declaring_blockers();
    (game, other_id)
}

const fn host_id_player(equipped_attacks: bool) -> PlayerId {
    if equipped_attacks {
        PlayerId::One
    } else {
        PlayerId::Two
    }
}

#[test]
fn wooden_stake_matches_the_equipped_subject_on_both_sides_of_a_vampire_block() {
    for equipped_attacks in [true, false] {
        let (mut game, vampire) = commit_equipped_block(equipped_attacks, cards::STROMKIRK_NOBLE);
        game.battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == vampire)
            .expect("the Vampire is present before the trigger resolves")
            .regeneration_shields = 1;
        drain_pending(&mut game);
        assert!(
            game.battlefield
                .iter()
                .all(|permanent| permanent.card.id != vampire),
            "the opposing Vampire is destroyed without regeneration"
        );
    }

    let (mut game, nonvampire) = commit_equipped_block(true, cards::SEDGE_TROLL);
    drain_pending(&mut game);
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == nonvampire),
        "a non-Vampire on the other side of the block does not trigger the Stake"
    );
}

#[test]
fn civic_saber_counts_the_equipped_creatures_live_colors() {
    let (mut game, _saber, smiter) = attached_board(cards::CIVIC_SABER, cards::LOXODON_SMITER);
    assert_eq!(
        (
            game.power(permanent(&game, smiter)),
            game.toughness(permanent(&game, smiter))
        ),
        (Some(6), Some(4)),
        "the green-white creature receives +2/+0"
    );

    attach_constant_resolved_characteristics(
        &mut game,
        smiter,
        &[AppliedEffectDef::set_colors(ColorSet::from_colors(&[
            ManaColor::Red,
        ]))],
        ContinuousEffectExpiration::EndOfTurn,
    );
    assert_eq!(
        (
            game.power(permanent(&game, smiter)),
            game.toughness(permanent(&game, smiter))
        ),
        (Some(5), Some(4)),
        "the bonus follows a later color-changing effect"
    );
}

#[test]
fn haunted_plate_mail_animation_removes_equipment_and_unattaches() {
    let mut game = ready();
    let mut mail = creature(10_000, cards::HAUNTED_PLATE_MAIL, PlayerId::One);
    let mail_id = mail.card.id;
    let host = creature(10_100, cards::GRIZZLY_BEARS, PlayerId::Two);
    let host_id = host.card.id;
    mail.attached_to = Some(host_id);
    game.battlefield.extend([mail, host]);
    assert_eq!(game.power(permanent(&game, host_id)), Some(6));

    let animate = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility {
                    source,
                    targets,
                    ..
                } if *source == mail_id && targets.is_empty()
            )
        })
        .expect("the zero-cost animation is offered with no creatures controlled");
    game.apply(PlayerId::One, animate)
        .expect("the animation activates");
    drain_pending(&mut game);

    let animated = permanent(&game, mail_id);
    let types = game
        .permanent_types(animated)
        .expect("the animated permanent has effective types");
    assert!(types.contains(CardType::Artifact));
    assert!(types.contains(CardType::Creature));
    assert_eq!(game.effective_subtypes(animated).as_ref(), &["Spirit"]);
    assert_eq!(
        (game.power(animated), game.toughness(animated)),
        (Some(4), Some(4))
    );
    assert_eq!(animated.attached_to, None);
    assert_eq!(game.power(permanent(&game, host_id)), Some(2));
}

#[test]
fn creature_attachments_unattach_even_when_reconfigure_remains() {
    let mut game = ready();
    let sash = creature(10_000, cards::LION_SASH, PlayerId::One);
    let sash_id = sash.card.id;
    let host = creature(10_100, cards::GRIZZLY_BEARS, PlayerId::One);
    let host_id = host.card.id;
    game.battlefield.extend([sash, host]);

    assert!(
        game.try_attach(sash_id, host_id),
        "an unattached reconfigure creature may prospectively attach"
    );
    assert!(
        !game
            .permanent_types(permanent(&game, sash_id))
            .expect("Sash has types")
            .contains(CardType::Creature),
        "reconfigure removes Creature while attached"
    );

    attach_constant_resolved_characteristics(
        &mut game,
        sash_id,
        &[AppliedEffectDef::set_card_types(
            CardTypeSet::single(CardType::Artifact).with(CardType::Creature),
        )],
        ContinuousEffectExpiration::EndOfTurn,
    );
    game.check_state_based_actions();
    assert_eq!(
        permanent(&game, sash_id).attached_to,
        None,
        "a later effect making the attached Equipment a creature wins by timestamp and the SBA detaches it"
    );
}

#[test]
fn an_attached_aura_that_becomes_a_creature_is_not_left_attached() {
    let mut game = ready();
    let host = creature(10_100, cards::GRIZZLY_BEARS, PlayerId::One);
    let host_id = host.card.id;
    let mut aura = creature(10_000, cards::HOLY_STRENGTH, PlayerId::One);
    let aura_id = aura.card.id;
    aura.attached_to = Some(host_id);
    game.battlefield.extend([aura, host]);
    attach_constant_resolved_characteristics(
        &mut game,
        aura_id,
        &[AppliedEffectDef::add_card_types(CardTypeSet::single(
            CardType::Creature,
        ))],
        ContinuousEffectExpiration::EndOfTurn,
    );

    game.check_state_based_actions();
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != aura_id),
        "the creature Aura first becomes unattached and is then put into its owner's graveyard"
    );
}

#[test]
fn an_attached_permanent_that_loses_aura_unattaches_but_stays_on_the_battlefield() {
    let mut game = ready();
    let host = creature(10_100, cards::GRIZZLY_BEARS, PlayerId::One);
    let host_id = host.card.id;
    let mut aura = creature(10_000, cards::HOLY_STRENGTH, PlayerId::One);
    let aura_id = aura.card.id;
    aura.attached_to = Some(host_id);
    game.battlefield.extend([aura, host]);
    attach_constant_resolved_characteristics(
        &mut game,
        aura_id,
        &[AppliedEffectDef::remove_subtypes(&["Aura"])],
        ContinuousEffectExpiration::EndOfTurn,
    );

    game.check_state_based_actions();
    let former_aura = permanent(&game, aura_id);
    assert_eq!(former_aura.attached_to, None);
    assert!(
        !game.effective_subtypes(former_aura).contains(&"Aura"),
        "effective subtype removal determines the current attachment kind"
    );
}

#[test]
fn elbrus_unattaches_then_transforms_after_the_equipped_creature_hits_a_player() {
    let (mut game, elbrus, host) =
        attached_board(cards::ELBRUS_THE_BINDING_BLADE, cards::GRIZZLY_BEARS);
    game.damage_target_from_kind(Some(host), Some(Target::Player(PlayerId::Two)), 3, true);
    drain_pending(&mut game);

    let withengar = permanent(&game, elbrus);
    assert_eq!(withengar.presented, CardPartId(1));
    assert_eq!(withengar.attached_to, None);
    assert_eq!(
        (game.power(withengar), game.toughness(withengar)),
        (Some(13), Some(13))
    );
    for keyword in [
        KeywordAbility::Flying,
        KeywordAbility::Intimidate,
        KeywordAbility::Trample,
    ] {
        assert!(game.permanent_has_executable_keyword(withengar, keyword));
    }
    assert_eq!(game.power(permanent(&game, host)), Some(2));
}

#[test]
fn batch_two_coverage_is_complete_except_for_withengars_terminal_trigger() {
    let catalog = poc::catalog().expect("catalog builds");
    for definition in [
        cards::WOODEN_STAKE,
        cards::CIVIC_SABER,
        cards::HAUNTED_PLATE_MAIL,
    ] {
        let card = catalog.get(definition).expect("the card is cataloged");
        assert_eq!(
            card.implementation_status(),
            ImplementationStatus::Complete,
            "{} should be fully executable",
            card.name,
        );
    }
    let elbrus = catalog
        .get(cards::ELBRUS_THE_BINDING_BLADE)
        .expect("Elbrus is cataloged");
    assert_eq!(
        elbrus.implementation_status(),
        ImplementationStatus::Partial
    );
    let back = elbrus
        .part(CardPartId(1))
        .expect("Withengar is the back face");
    assert!(back.rules.ability_clauses().iter().any(|ability| {
        ability.text.starts_with("Whenever a player loses the game")
            && !ability.is_executable()
            && ability.coverage.explanation.is_some()
    }));
}
