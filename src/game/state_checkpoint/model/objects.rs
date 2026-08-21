use serde::{Deserialize, Serialize};

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
        definition: u16,
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
/// rooted in a printed or custom card creator. Declarative creators select an
/// effect-tree node; custom creators select one entry from the card layer's
/// creator-owned token registry. No `CardRules` or function pointer crosses
/// the checkpoint boundary.
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
    Custom {
        creator: Box<AbilityLocator>,
        token_index: usize,
    },
}

impl TokenCharacteristicsLocator {
    pub(in crate::game::state_checkpoint) fn creator(&self) -> &AbilityLocator {
        match self {
            Self::EffectPath { creator, .. } | Self::Custom { creator, .. } => creator,
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
        definition: u16,
        part_id: u8,
    },
    Token {
        token: TokenCharacteristicsLocator,
        part_id: u8,
    },
    Emblem {
        emblem: EmblemCharacteristicsLocator,
    },
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(in crate::game::state_checkpoint) enum ObjectKindSnapshot {
    Card { definition: u16 },
    Token,
    Emblem,
    Ability,
}
