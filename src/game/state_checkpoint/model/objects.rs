use serde::{Deserialize, Serialize};

use crate::CardDefinitionId;

use super::{BasicLandTypeSnapshot, ManaColorSnapshot};

/// A semantic path to an authored ability. Printed abilities start directly
/// from the card catalog; virtual-object abilities first rebuild their
/// creator-owned characteristics from the effect that creates them.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(
    tag = "source",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(in crate::game::state_checkpoint) enum AbilityLocator {
    Card {
        definition: CardDefinitionId,
        part_id: u8,
        ability_id: u8,
        nested: Vec<usize>,
    },
    Token {
        token: TokenCharacteristicsLocator,
        part_id: u8,
        ability_id: u8,
        nested: Vec<usize>,
    },
    Emblem {
        emblem: EmblemCharacteristicsLocator,
        ability_id: u8,
        nested: Vec<usize>,
    },
}

/// A durable path from a card-, token-, or emblem-owned ability to the token
/// characteristics it creates. Virtual-object creator chains are recursively
/// rooted in a printed card creator and select a declarative effect-tree node.
/// No `CardRules` or function pointer crosses the checkpoint boundary.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(
    tag = "source",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(in crate::game::state_checkpoint) enum TokenCharacteristicsLocator {
    EntryChoice {
        creator: Box<AbilityLocator>,
        choice_index: usize,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        colors: Option<[bool; 5]>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        basic_land_type_words: Option<[BasicLandTypeSnapshot; 5]>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        color_words: Option<[ManaColorSnapshot; 5]>,
    },
    EffectPath {
        creator: Box<AbilityLocator>,
        effect_path: Vec<usize>,
        /// A text-changing effect on the creating spell or ability bakes
        /// changed color words into the token's copiable values.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        colors: Option<[bool; 5]>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        basic_land_type_words: Option<[BasicLandTypeSnapshot; 5]>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        color_words: Option<[ManaColorSnapshot; 5]>,
    },
}

impl TokenCharacteristicsLocator {
    pub(in crate::game::state_checkpoint) fn creator(&self) -> &AbilityLocator {
        match self {
            Self::EffectPath { creator, .. } | Self::EntryChoice { creator, .. } => creator,
        }
    }

    pub(in crate::game::state_checkpoint) fn set_word_overrides(
        &mut self,
        new_colors: Option<[bool; 5]>,
        new_basic_land_type_words: Option<[BasicLandTypeSnapshot; 5]>,
        new_color_words: Option<[ManaColorSnapshot; 5]>,
    ) {
        let (colors, basic_land_type_words, color_words) = match self {
            Self::EntryChoice {
                colors,
                basic_land_type_words,
                color_words,
                ..
            }
            | Self::EffectPath {
                colors,
                basic_land_type_words,
                color_words,
                ..
            } => (colors, basic_land_type_words, color_words),
        };
        *colors = new_colors;
        *basic_land_type_words = new_basic_land_type_words;
        *color_words = new_color_words;
    }
}

/// A durable path from a card-, token-, or emblem-owned ability to the emblem
/// characteristics it creates. Only semantic creator/effect positions cross
/// the checkpoint boundary; `CardRules` and behavior pointers never do.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(
    tag = "source",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(in crate::game::state_checkpoint) enum EmblemCharacteristicsLocator {
    EffectPath {
        creator: Box<AbilityLocator>,
        effect_path: Vec<usize>,
    },
}

impl EmblemCharacteristicsLocator {
    pub(in crate::game::state_checkpoint) fn creator(&self) -> &AbilityLocator {
        match self {
            Self::EffectPath { creator, .. } => creator,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(in crate::game::state_checkpoint) enum ObjectCharacteristicsSnapshot {
    Card {
        definition: CardDefinitionId,
        part_id: u8,
    },
    Token {
        token: TokenCharacteristicsLocator,
        part_id: u8,
    },
    Emblem {
        emblem: EmblemCharacteristicsLocator,
    },
    FaceDown {
        face_down: FaceDownCharacteristicsSnapshot,
    },
}

/// Rule-defined face-down values supported by the current engine. The tags
/// describe characteristics rather than the mechanism that supplied them:
/// morph, manifest, and Illusionary Mask share `OrdinaryTwoTwo`, while
/// disguise and cloak share `WardTwoTwo`.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::game::state_checkpoint) enum FaceDownCharacteristicsSnapshot {
    OrdinaryTwoTwo,
    WardTwoTwo,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(in crate::game::state_checkpoint) enum ObjectKindSnapshot {
    Card { definition: CardDefinitionId },
    Token,
    Emblem,
    Ability,
}
