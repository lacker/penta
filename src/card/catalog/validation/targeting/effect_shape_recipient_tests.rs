use super::*;
use crate::card::{PlayActionMatcherDef, PlayRestrictionDef, ResolvedEffectDurationDef};

const PLAYER_TARGET: AbilityTargetDef =
    AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(PlayerRelation::Any));
const OBJECT_TARGET: AbilityTargetDef =
    AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::Any,
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    });
const ANY_TARGET: AbilityTargetDef =
    AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget);

fn cannot_play() -> AppliedEffectDef {
    AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(PlayRestrictionDef::new(
        PlayActionMatcherDef::CastSpell,
        ObjectPredicateDef::NoncreatureSpell,
    )))
}

#[test]
fn object_and_player_effects_reject_opposite_typed_recipients() {
    assert_eq!(
        validate_ability_targets(
            &[],
            EffectDef::Tap {
                object: EffectRecipientDef::Controller,
            },
        ),
        Err(GrantedAbilityValidationError::EffectRecipientKindMismatch {
            recipient: Box::new(EffectRecipientDef::Controller),
            expected: EffectSubjectKind::Object,
        }),
    );
    assert_eq!(
        validate_ability_targets(
            &[],
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Source,
                amount: ValueDef::Constant(1),
            },
        ),
        Err(GrantedAbilityValidationError::EffectRecipientKindMismatch {
            recipient: Box::new(EffectRecipientDef::Source),
            expected: EffectSubjectKind::Player,
        }),
    );
    assert_eq!(
        validate_ability_targets(
            &[],
            EffectDef::ExchangeControl {
                first: EffectRecipientDef::Source,
                second: EffectRecipientDef::Controller,
                otherwise: None,
            },
        ),
        Err(GrantedAbilityValidationError::EffectRecipientKindMismatch {
            recipient: Box::new(EffectRecipientDef::Controller),
            expected: EffectSubjectKind::Object,
        }),
        "both sides of an exchange are validated",
    );
}

#[test]
fn target_slots_must_contain_the_subject_kind_an_effect_reads() {
    assert_eq!(
        validate_ability_targets(
            &[PLAYER_TARGET],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
        Err(GrantedAbilityValidationError::TargetReferenceKindMismatch {
            target: TargetIndex::PRIMARY,
            predicate: PLAYER_TARGET.predicate,
            expected: EffectSubjectKind::Object,
        }),
    );
    assert_eq!(
        validate_ability_targets(
            &[OBJECT_TARGET],
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ),
        Err(GrantedAbilityValidationError::TargetReferenceKindMismatch {
            target: TargetIndex::PRIMARY,
            predicate: OBJECT_TARGET.predicate,
            expected: EffectSubjectKind::Player,
        }),
    );
}

#[test]
fn typed_projections_make_mixed_target_filtering_explicit() {
    let effects = Box::leak(Box::new([
        EffectDef::Tap {
            object: EffectRecipientDef::target_objects(TargetIndex::PRIMARY),
        },
        EffectDef::Apply {
            recipient: EffectRecipientDef::target_players(TargetIndex::PRIMARY),
            effect: cannot_play(),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    ]));
    validate_ability_targets(&[ANY_TARGET], EffectDef::Sequence(effects))
        .expect("typed projections retain both halves of an any-target slot");
}

#[test]
fn raw_target_references_require_at_most_one_selected_target() {
    let targets = [AbilityTargetDef::up_to(OBJECT_TARGET.predicate, 2)];
    assert_eq!(
        validate_ability_targets(
            &targets,
            EffectDef::Tap {
                object: EffectRecipientDef::object(ObjectRefDef::Target(TargetIndex::PRIMARY,)),
            },
        ),
        Err(
            GrantedAbilityValidationError::TargetReferenceRequiresSingular {
                target: TargetIndex::PRIMARY,
                maximum: 2,
            },
        ),
    );
}

#[test]
fn derived_controller_accepts_mixed_targets_but_owner_requires_an_object() {
    validate_ability_targets(
        &[ANY_TARGET],
        EffectDef::GainLife {
            recipient: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                ObjectRefDef::Target(TargetIndex::PRIMARY),
            )),
            amount: ValueDef::Constant(1),
        },
    )
    .expect("a player is its own controller, so either half is meaningful");

    validate_ability_targets(
        &[OBJECT_TARGET],
        EffectDef::GainLife {
            recipient: EffectRecipientDef::player(PlayerRefDef::OwnerOf(ObjectRefDef::Target(
                TargetIndex::PRIMARY,
            ))),
            amount: ValueDef::Constant(1),
        },
    )
    .expect("an object target always has an owner");

    for target in [ANY_TARGET, PLAYER_TARGET] {
        assert_eq!(
            validate_ability_targets(
                &[target],
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::player(PlayerRefDef::OwnerOf(
                        ObjectRefDef::Target(TargetIndex::PRIMARY),
                    )),
                    amount: ValueDef::Constant(1),
                },
            ),
            Err(GrantedAbilityValidationError::TargetReferenceKindMismatch {
                target: TargetIndex::PRIMARY,
                predicate: target.predicate,
                expected: EffectSubjectKind::Object,
            }),
        );
    }
}

#[test]
fn static_player_rules_reject_event_only_selectors() {
    let recipient = EffectRecipientDef::player(PlayerRefDef::EventPlayer);
    assert_eq!(
        validate_ability_targets(
            &[],
            EffectDef::StaticApply {
                recipient,
                effect: cannot_play(),
            },
        ),
        Err(
            GrantedAbilityValidationError::UnsupportedStaticPlayerRecipient {
                recipient: Box::new(recipient)
            },
        ),
    );
}
