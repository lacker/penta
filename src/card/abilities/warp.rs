// Warp's alternative cost and delayed exile with permission to play.
//
// Included textually into `abilities.rs`, so the imports here are the parent
// module's.

/// The delayed half of warp: the permanent is exiled at the beginning of
/// the next end step, and its owner may cast it from there afterwards.
static WARP_EXILES_IT: AbilityDef = AbilityDef::triggered(
    "At the beginning of the next end step, exile this permanent. You may cast it from exile on \
     a later turn.",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::End,
        player: PlayerRelation::Any,
    },
    EffectDef::ExileGrantingOwnerPlay {
        object: EffectRecipientDef::Source,
        // Nothing on top: the card is simply castable from where it now
        // sits.
        surcharge: ManaCost::new(0, 0),
    },
);

static WAS_WARPED: TriggerConditionDef =
    TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Warp);

/// Warp: an ordinary cast from hand for a different price, with what it
/// buys stated by the clause below.
#[must_use]
pub const fn warp(costs: &'static [CostDef], text: &'static str) -> AbilityDef {
    AbilityDef::alternative_cast(
        costs,
        AlternativeCastKindDef::Warp,
        Some(text),
        EffectDef::None,
    )
}

/// "Exile it at the beginning of the next end step, then you may cast it
/// from exile on a later turn." A delayed trigger set up as the warped
/// permanent arrives.
#[must_use]
pub const fn warped_exile() -> AbilityDef {
    AbilityDef::triggered_if(
        "When this permanent enters, if it was warped, exile it at the beginning of the next end \
         step. You may cast it from exile on a later turn.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::Source,
            None,
            Some(ZoneKind::Battlefield),
        ),
        &WAS_WARPED,
        EffectDef::InstallTrigger(InstalledTriggerDef::once(&WARP_EXILES_IT)),
    )
}
