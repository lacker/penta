/// The lifetime of a continuous effect created by a resolving spell or
/// ability. Static effects use [`EffectDef::StaticApply`] instead: they are
/// derived live from the ability that creates them and have no stored
/// expiration.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ResolvedEffectDurationSetDef(u8);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ResolvedEffectDurationDef {
    Permanent,
    UntilEndOfTurn,
    /// Until the affected player next casts a spell matching the applied
    /// rule's spell predicate.
    UntilNextMatchingCast,
    /// Until the beginning of the resolving ability's controller's next
    /// upkeep, which outlives the cleanup that ends an until-end-of-turn
    /// effect.
    UntilYourNextUpkeep,
    /// Until the next turn of the effect's controller begins. The affected
    /// turn is captured when the resolving effect is created.
    UntilYourNextTurn,
    /// Until the current combat phase ends. Shorter than
    /// [`Self::UntilEndOfTurn`]: it expires as the end-of-combat step
    /// finishes rather than waiting for cleanup, so a creature pumped for one
    /// combat is back to its printed size in the postcombat main phase.
    UntilEndOfCombat,
    /// For as long as the effect's own source stays tapped. Unlike every
    /// other resolving duration this one has no deadline: the artifact that
    /// tapped to make it decides when it ends by untapping.
    WhileSourceTapped,
    /// "For as long as this creature remains on the battlefield." The same
    /// open-ended shape as [`Self::WhileSourceTapped`] with a weaker
    /// condition: the source has only to still be there. A source that
    /// leaves and returns is a new object, so what it left behind stays
    /// ended.
    WhileSourceRemains,
    /// Ends when any contained atomic duration ends. Authored cards normally
    /// construct this through [`Self::or`].
    AnyOf(ResolvedEffectDurationSetDef),
}

impl ResolvedEffectDurationDef {
    const fn bits(self) -> u8 {
        match self {
            Self::Permanent => 0,
            Self::UntilEndOfTurn => 1 << 0,
            Self::UntilNextMatchingCast => 1 << 1,
            Self::UntilYourNextUpkeep => 1 << 2,
            Self::UntilYourNextTurn => 1 << 3,
            Self::UntilEndOfCombat => 1 << 4,
            Self::WhileSourceTapped => 1 << 5,
            Self::WhileSourceRemains => 1 << 6,
            Self::AnyOf(set) => set.0,
        }
    }

    const fn from_bits(bits: u8) -> Self {
        match bits {
            0 => Self::Permanent,
            1 => Self::UntilEndOfTurn,
            2 => Self::UntilNextMatchingCast,
            4 => Self::UntilYourNextUpkeep,
            8 => Self::UntilYourNextTurn,
            16 => Self::UntilEndOfCombat,
            32 => Self::WhileSourceTapped,
            64 => Self::WhileSourceRemains,
            _ => Self::AnyOf(ResolvedEffectDurationSetDef(bits)),
        }
    }

    /// Ends when either duration ends. Chaining composes any number of
    /// independent expiration conditions without inventing compound cases.
    #[must_use]
    pub const fn or(self, other: Self) -> Self {
        Self::from_bits(self.bits() | other.bits())
    }

    #[must_use]
    pub const fn contains(self, duration: Self) -> bool {
        let duration = duration.bits();
        duration != 0 && self.bits() & duration == duration
    }
}
