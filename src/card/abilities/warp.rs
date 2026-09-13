// Included into abilities.rs; Warp owns its complete resolution program here.

static WARP_EXILES_IT: AbilityDef = AbilityDef::triggered(
    "At the beginning of the next end step, exile this permanent. Its owner may cast it \
     from exile on a later turn.",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::End,
        player: PlayerRelation::Any,
    },
    EffectDef::ExileGrantingOwnerPlay {
        object: EffectRecipientDef::Source,
        surcharge: ManaCost::new(0, 0),
        later_turn: true,
        cast_only: true,
    },
);

/// Warp's alternative cost includes its delayed trigger. The spell installs
/// it as it resolves, so removing the permanent's abilities cannot stop it.
#[must_use]
pub const fn warp(costs: &'static [CostDef], text: &'static str) -> AbilityDef {
    AbilityDef::alternative_cast(
        costs,
        AlternativeCastKindDef::Warp,
        Some(text),
        EffectDef::InstallTrigger(InstalledTriggerDef::once(&WARP_EXILES_IT)),
    )
}
