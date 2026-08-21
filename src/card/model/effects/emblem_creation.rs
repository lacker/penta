use super::super::{AbilityDef, EffectDef, EmblemCharacteristics};

impl EffectDef {
    /// Creates an emblem with the given display name and abilities.
    #[must_use]
    pub const fn create_emblem(name: &'static str, abilities: &'static [AbilityDef]) -> Self {
        Self::CreateEmblem {
            emblem: EmblemCharacteristics::new(name, abilities),
        }
    }
}
