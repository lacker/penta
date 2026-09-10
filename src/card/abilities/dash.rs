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
    EffectDef::MoveToZone {
        object: EffectRecipientDef::Source,
        zone: ZoneKind::Hand,
        placement: ZonePlacement::Top,
    },
);

static WAS_DASHED: TriggerConditionDef =
    TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Dash);

static DASH_HASTE: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::Source,
    effect: AppliedEffectDef::add_ability(&HASTE),
};

static HASTE: AbilityDef = haste();

/// Dash (CR 702.109a): an ordinary cast from hand for a different price.
/// What it buys is stated by the two clauses below, the way evoke's
/// sacrifice is stated beside its own alternative cost.
#[must_use]
pub const fn dash(costs: &'static [CostDef], text: &'static str) -> AbilityDef {
    AbilityDef::alternative_cast(
        costs,
        AlternativeCastKindDef::Dash,
        Some(text),
        EffectDef::None,
    )
}

/// "If you do, it gains haste": read live off how the permanent was cast,
/// so a creature that arrived some other way has nothing.
#[must_use]
pub const fn dashed_haste() -> AbilityDef {
    AbilityDef::static_ability(
        "This creature has haste as long as it was dashed.",
        EffectDef::IfCondition {
            condition: &WAS_DASHED,
            then: &DASH_HASTE,
        },
    )
}

/// "And it's returned from the battlefield to its owner's hand at the
/// beginning of the next end step." A delayed trigger set up as the dashed
/// creature arrives, so a second dash later sets up a second return.
#[must_use]
pub const fn dashed_return() -> AbilityDef {
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
    )
}
