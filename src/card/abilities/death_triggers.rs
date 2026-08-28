// Reusable enters-the-battlefield and battlefield-to-graveyard trigger shapes.
//
// Included textually into `abilities.rs`, so these constructors share its
// model imports and remain in the public `card::abilities` namespace.

/// A source permanent's own enters-the-battlefield trigger.
#[must_use]
pub const fn enters_trigger(text: &'static str, effect: EffectDef) -> AbilityDef {
    enters_trigger_with_targets(text, &[], effect)
}

/// A targeted source permanent's own enters-the-battlefield trigger.
#[must_use]
pub const fn enters_trigger_with_targets(
    text: &'static str,
    targets: &'static [AbilityTargetDef],
    effect: EffectDef,
) -> AbilityDef {
    AbilityDef::triggered_with_targets(
        text,
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::Source,
            None,
            Some(ZoneKind::Battlefield),
        ),
        targets,
        effect,
    )
}

/// A source permanent's own battlefield-to-graveyard trigger ("dies" for a creature).
#[must_use]
pub const fn dies_trigger(text: &'static str, effect: EffectDef) -> AbilityDef {
    dies_trigger_matching(text, ObjectPredicateDef::Source, effect)
}

/// The targeted form of [`dies_trigger`].
#[must_use]
pub const fn dies_trigger_with_targets(
    text: &'static str,
    targets: &'static [AbilityTargetDef],
    effect: EffectDef,
) -> AbilityDef {
    dies_trigger_matching_with_targets(text, ObjectPredicateDef::Source, targets, effect)
}

/// A battlefield-to-graveyard trigger for any permanent matching `object`.
#[must_use]
pub const fn dies_trigger_matching(
    text: &'static str,
    object: ObjectPredicateDef,
    effect: EffectDef,
) -> AbilityDef {
    dies_trigger_matching_with_targets(text, object, &[], effect)
}

/// The targeted form of [`dies_trigger_matching`].
#[must_use]
pub const fn dies_trigger_matching_with_targets(
    text: &'static str,
    object: ObjectPredicateDef,
    targets: &'static [AbilityTargetDef],
    effect: EffectDef,
) -> AbilityDef {
    AbilityDef::triggered_with_targets(
        text,
        TriggerEventDef::zone_changed(
            object,
            Some(ZoneKind::Battlefield),
            Some(ZoneKind::Graveyard),
        ),
        targets,
        effect,
    )
}

/// A trigger for a creature dealt damage by this ability's source this turn
/// dying. Damage history and the source's last known information travel with
/// the battlefield-to-graveyard event, so the source may die in the same
/// batch as the creature it damaged.
#[must_use]
pub const fn creature_damaged_by_source_dies_trigger(
    text: &'static str,
    effect: EffectDef,
) -> AbilityDef {
    creature_damaged_by_source_dies_trigger_with_targets(text, &[], effect)
}

/// The targeted form of [`creature_damaged_by_source_dies_trigger`].
#[must_use]
pub const fn creature_damaged_by_source_dies_trigger_with_targets(
    text: &'static str,
    targets: &'static [AbilityTargetDef],
    effect: EffectDef,
) -> AbilityDef {
    AbilityDef::triggered_with_targets(
        text,
        TriggerEventDef::ZoneChanged(
            ZoneChangeEventMatcherDef::new(
                ObjectPredicateDef::HasType(CardType::Creature),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            )
            .previously_damaged_by(ObjectRefDef::Source),
        ),
        targets,
        effect,
    )
}
