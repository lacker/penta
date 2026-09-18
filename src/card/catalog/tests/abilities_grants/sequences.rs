#[test]
fn catalog_sequence_requires_at_least_two_effects() {
    static DRAW: EffectDef = EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    };
    static PUMP: EffectDef = EffectDef::StaticApply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::modify_power_toughness(
            ValueDef::Constant(1),
            ValueDef::Constant(1),
        ),
    };
    for (effect, static_ability) in [(DRAW, false), (PUMP, true)] {
        let single: &'static [EffectDef] = Box::leak(Box::new([effect]));
        let pair: &'static [EffectDef] = Box::leak(Box::new([effect, effect]));
        let ability = |effect| {
            if static_ability {
                AbilityDef::static_ability("A static program.", effect)
            } else {
                AbilityDef::spell("A resolving program.", effect)
            }
        };
        for effects in [&[][..], single] {
            assert_eq!(
                error(definition_with_ability(ability(EffectDef::Sequence(
                    effects
                )))),
                CatalogError::UnsupportedAbilityEffectProgramContext {
                    definition: CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000001"),
                    part: CardPartId::PRIMARY,
                    ability: AbilityId::PRIMARY,
                    context: "sequence",
                    operation: "requires at least two effects; use None or the single effect directly",
                },
            );
        }
        for valid in [EffectDef::None, effect, EffectDef::Sequence(pair)] {
            CardCatalog::new([definition_with_ability(ability(valid))])
                .expect("empty and single programs need no wrapper; two effects form a sequence");
        }
    }
}

#[test]
fn catalog_sequence_minimum_applies_inside_continuations_and_replacements() {
    static DRAW: EffectDef = EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    };
    static EMPTY: EffectDef = EffectDef::Sequence(&[]);
    static SINGLE: EffectDef = EffectDef::Sequence(&[DRAW]);
    let problem = GrantedAbilityValidationError::UnsupportedEffectProgramContext {
        context: "sequence",
        operation: "requires at least two effects; use None or the single effect directly",
    };
    for invalid in [&EMPTY, &SINGLE] {
        for effect in continuation_effects(invalid) {
            assert_eq!(
                super::validate_ability_targets(&[], effect),
                Err(problem.clone()),
                "every recursive continuation must reject a degenerate sequence",
            );
        }
        assert_eq!(
            super::validate_replacement_ability_targets(
                &[],
                ReplacementEffectDef::Perform(invalid),
            ),
            Err(problem.clone()),
            "ordinary effects embedded in replacement programs share the same invariant",
        );
    }
}
