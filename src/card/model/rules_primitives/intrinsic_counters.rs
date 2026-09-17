/// A named counter whose presence has a consequence defined by the game rules,
/// independent of the card or mechanic that created it.
///
/// This is deliberately narrower than counters referenced by named rules
/// mechanics. A time counter has meaning to suspend and an energy counter can
/// be paid by an effect, but neither does anything merely by being present.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum IntrinsicCounter {
    Loyalty,
    Lore,
    Finality,
    Stun,
    Poison,
    Hone,
    Shield,
}

impl IntrinsicCounter {
    pub const ALL: [Self; 7] = [
        Self::Loyalty,
        Self::Lore,
        Self::Finality,
        Self::Stun,
        Self::Poison,
        Self::Hone,
        Self::Shield,
    ];

    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Loyalty => "loyalty",
            Self::Lore => "lore",
            Self::Finality => "finality",
            Self::Stun => "stun",
            Self::Poison => "poison",
            Self::Hone => "hone",
            Self::Shield => "shield",
        }
    }

    pub(super) const fn from_name(name: CounterName) -> Option<Self> {
        let mut index = 0;
        while index < Self::ALL.len() {
            let candidate = Self::ALL[index];
            if counter_name_key(candidate.name()) == name.0 {
                return Some(candidate);
            }
            index += 1;
        }
        None
    }
}
