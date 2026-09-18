// Dash's alternative cost, conditional haste, and delayed return.
//
// Included textually into `abilities.rs`, so the imports here are the parent
// module's.

/// The delayed half of dash: the creature goes home at the beginning of the
/// next end step, whoever's it is.
static DASH_RETURNS_IT: AbilityDef = AbilityDef::triggered(
    "At the beginning of the next end step, return this creature to its owner's hand.",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::End,
        player: PlayerRelation::Any,
    },
    EffectDef::move_to_zone(
        EffectRecipientDef::Source,
        ZoneKind::Hand,
        ZonePlacement::Top,
    ),
);

static WAS_DASHED: TriggerConditionDef =
    TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Dash);

static DASH_HASTE: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::Source,
    effect: AppliedEffectDef::add_ability(&HASTE),
};

static HASTE: AbilityDef = haste();

/// Dash (CR 702.109a), expanded into its alternative cost, conditional haste,
/// and delayed-return setup. Use [`crate::ability_list!`] to include the entire
/// mechanic in a card declaration.
#[must_use]
pub const fn dash(costs: &'static [CostDef], text: &'static str) -> [AbilityDef; 3] {
    [
        AbilityDef::alternative_cast(
            costs,
            AlternativeCastKindDef::Dash,
            Some(text),
            EffectDef::None,
        ),
        AbilityDef::static_ability(
            "This creature has haste as long as it was dashed.",
            EffectDef::IfCondition {
                condition: &WAS_DASHED,
                then: &DASH_HASTE,
            },
        ),
        AbilityDef::triggered_if(
            "When this creature enters, if it was dashed, return it to its owner's hand at the \
             beginning of the next end step.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &WAS_DASHED,
            EffectDef::InstallTrigger(InstalledTriggerDef::once(&DASH_RETURNS_IT)),
        ),
    ]
}
