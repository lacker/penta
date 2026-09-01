use serde::{Deserialize, Serialize};

use crate::CardDefinitionId;

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
    EffectPath {
        creator: Box<AbilityLocator>,
        effect_path: Vec<usize>,
    },
}

impl TokenCharacteristicsLocator {
    pub(in crate::game::state_checkpoint) fn creator(&self) -> &AbilityLocator {
        match self {
            Self::EffectPath { creator, .. } => creator,
        }
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
