#[test]
fn reflexive_trigger_targets_have_their_own_namespace_and_shapes() {
    static TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
        ObjectPredicateDef::HasType(CardType::Creature),
    )];
    static TAP: AbilityDef = AbilityDef::triggered_with_targets(
        "When you do, tap target creature.",
        TriggerEventDef::Reflexive,
        &TARGETS,
        EffectDef::Tap {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        },
    );
    static MISSING_TARGET: AbilityDef = AbilityDef::triggered(
        "When you do, tap the creator's target.",
        TriggerEventDef::Reflexive,
        EffectDef::Tap {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        },
    );
    static WRONG_SHAPE: AbilityDef = AbilityDef::triggered_with_targets(
        "When you do, draw for a creature target.",
        TriggerEventDef::Reflexive,
        &TARGETS,
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(1),
        },
    );
    super::validate_ability_targets(&[], EffectDef::ReflexiveTrigger(&TAP))
        .expect("the creator does not declare the reflexive ability's targets");

    assert!(
        super::validate_ability_targets(&TARGETS, EffectDef::ReflexiveTrigger(&MISSING_TARGET))
            .is_err()
    );

    assert!(
        super::validate_ability_targets(&[], EffectDef::ReflexiveTrigger(&WRONG_SHAPE)).is_err()
    );
}

#[test]
fn reflexive_trigger_rejects_future_listeners_and_standalone_reflexive_clauses() {
    static LATER: AbilityDef = AbilityDef::triggered(
        "At the beginning of the end step, trigger.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::End,
            player: PlayerRelation::Any,
        },
        EffectDef::None,
    );
    assert!(super::validate_ability_targets(&[], EffectDef::ReflexiveTrigger(&LATER)).is_err());

    let reflexive = AbilityDef::triggered(
        "When you do, trigger.",
        TriggerEventDef::Reflexive,
        EffectDef::None,
    );
    assert!(matches!(
        error(definition_with_ability(reflexive)),
        CatalogError::UnsupportedTriggerEvent { .. }
    ));
}
