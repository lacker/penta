// Reusable trigger shapes for creatures dying.
//
// Included textually into `abilities.rs`, so these constructors share its
// model imports and remain in the public `card::abilities` namespace.

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
