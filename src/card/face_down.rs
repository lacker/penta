//! Rules-owned characteristics for face-down spells and permanents.
//!
//! These constructors name mechanisms rather than identities. Morph,
//! manifest, and Illusionary Mask currently prescribe the same ordinary 2/2
//! body; disguise and cloak prescribe the same body with ward {2}. Keeping
//! separate constructors makes the creating rule explicit while sharing the
//! resulting characteristic values.

use super::{AbilityDef, AlternativeCastKindDef, FaceDownCharacteristics, abilities};

static WARD_TWO: [AbilityDef; 1] = [abilities::ward(
    2,
    "Ward {2} (Whenever this creature becomes the target of a spell or ability an opponent controls, counter it unless that player pays {2}.)",
)];

/// The fallback characteristics from CR 708.2a: a nameless, colorless 2/2
/// creature with no mana cost, subtypes, or abilities.
#[must_use]
pub const fn ordinary() -> FaceDownCharacteristics {
    FaceDownCharacteristics::creature("Face-down creature", &[], &[], 2, 2)
}

#[must_use]
pub const fn morph() -> FaceDownCharacteristics {
    ordinary()
}

/// Alternative-cast metadata for Morph. Keeping the mechanism label beside
/// its face-down values lets casting and checkpoint code recover both from
/// one declaration.
#[must_use]
pub const fn morph_cast() -> AlternativeCastKindDef {
    AlternativeCastKindDef::FaceDown {
        label: "Morph",
        characteristics: morph(),
    }
}

#[must_use]
pub const fn manifest() -> FaceDownCharacteristics {
    ordinary()
}

#[must_use]
pub const fn illusionary_mask() -> FaceDownCharacteristics {
    ordinary()
}

/// The 2/2 face-down creature with ward {2} prescribed by disguise.
#[must_use]
pub const fn disguise() -> FaceDownCharacteristics {
    FaceDownCharacteristics::creature("Face-down creature", &[], &[], 2, 2)
        .with_abilities(&WARD_TWO)
}

/// Alternative-cast metadata for Disguise.
#[must_use]
pub const fn disguise_cast() -> AlternativeCastKindDef {
    AlternativeCastKindDef::FaceDown {
        label: "Disguise",
        characteristics: disguise(),
    }
}

/// The 2/2 face-down creature with ward {2} prescribed by cloak.
#[must_use]
pub const fn cloak() -> FaceDownCharacteristics {
    disguise()
}
