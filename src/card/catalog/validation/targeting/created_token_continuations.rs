// Continuations whose producer guarantees battlefield object identities.
//
// Token creation and battlefield-only queries provide a fact ordinary binding
// validation cannot infer. Preserve it through a sequence so those objects
// can gain noncopiable abilities alongside other instructions.

fn validate_battlefield_binding_continuation(
    effect: EffectDef,
    battlefield_binding: Binding,
    targets: &[AbilityTargetDef],
    triggering_object_zone: Option<ZoneKind>,
) -> Result<(), GrantedAbilityValidationError> {
    match effect {
        EffectDef::Sequence(effects) => {
            for effect in effects {
                validate_battlefield_binding_continuation(
                    *effect,
                    battlefield_binding,
                    targets,
                    triggering_object_zone,
                )?;
            }
            Ok(())
        }
        EffectDef::Apply {
            recipient:
                EffectRecipientDef(EffectRecipientSetDef::Objects(ObjectSetDef::Binding(binding))),
            effect,
            duration,
        } if binding == battlefield_binding => {
            validate_applied_effect_shapes(
                EffectRecipientDef::objects(ObjectSetDef::Binding(binding)),
                effect,
                targets,
                false,
            )?;
            if duration_is_valid_for_applied_effect(duration, effect) {
                Ok(())
            } else {
                Err(GrantedAbilityValidationError::UnsupportedResolvingAppliedEffect)
            }
        }
        _ => validate_effect_target_shapes(effect, targets, triggering_object_zone),
    }
}

#[test]
fn ability_grants_to_bound_queries_require_battlefield_only_sources() {
    const BINDING: Binding = crate::Binding!("artifacts");
    const HASTE: EffectDef = EffectDef::Apply {
        recipient: EffectRecipientDef::objects(ObjectSetDef::Binding(BINDING)),
        effect: AppliedEffectDef::add_ability(&crate::card::abilities::haste()),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    };
    for zones in [
        &[ZoneKind::Battlefield][..],
        &[ZoneKind::Graveyard][..],
        &[ZoneKind::Battlefield, ZoneKind::Graveyard][..],
    ] {
        let effect = EffectDef::BindObjects(crate::card::BindObjectsDef {
            source: crate::card::ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::Query(
                ObjectQueryDef::new(ObjectPredicateDef::HasType(crate::card::CardType::Artifact), zones),
            )),
            binding: BINDING,
            then: &HASTE,
        });
        assert_eq!(validate_effect_target_shapes(effect, &[], None).is_ok(),
            zones == [ZoneKind::Battlefield]);
    }
}
