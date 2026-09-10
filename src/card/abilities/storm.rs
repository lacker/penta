// Storm's source-cast trigger and spell-copy effect.
//
// Included textually into `abilities.rs`, so the imports here are the parent
// module's.

static STORM_COPY: CopyStackObjectDef = CopyStackObjectDef {
    object: EffectRecipientDef::Source,
    controller: PlayerRefDef::EffectController,
    count: ValueDef::SpellsCastBeforeThisTurn,
    retarget: true,
    colors: None,
};

/// Storm (CR 702.40): casting the spell triggers one copy for every spell
/// cast before it this turn. Each copy gets its own target choice.
#[must_use]
pub const fn storm() -> AbilityDef {
    AbilityDef::triggered(
        "Storm (When you cast this spell, copy it for each spell cast before it this turn. You may choose new targets for the copies.)",
        TriggerEventDef::spell_cast(ObjectPredicateDef::Source),
        EffectDef::CopyStackObject(&STORM_COPY),
    )
}
