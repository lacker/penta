use serde::{Deserialize, Serialize};

use crate::CardDefinitionId;

use super::{BasicLandTypeSnapshot, ManaColorSnapshot, PermanentSnapshot, TargetSnapshot};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::struct_excessive_bools)]
pub(in crate::game::state_checkpoint) struct DetachedPermanentSnapshot {
    pub(in crate::game::state_checkpoint) state: PermanentSnapshot,
    pub(in crate::game::state_checkpoint) controller: usize,
    pub(in crate::game::state_checkpoint) tapped: bool,
    pub(in crate::game::state_checkpoint) damage: u16,
    pub(in crate::game::state_checkpoint) attacking: bool,
    pub(in crate::game::state_checkpoint) attack_defender: Option<AttackDefenderSnapshot>,
    pub(in crate::game::state_checkpoint) blocked: bool,
    /// Every attacker this creature is blocking. A list because a band is
    /// blocked as a group and one creature may be allowed several blocks.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(in crate::game::state_checkpoint) blocking: Vec<u32>,
    /// Whether it blocked something that has since left combat, which the
    /// list above can no longer say. Absent from a payload written before the
    /// distinction existed, and from the ordinary case where the list answers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(in crate::game::state_checkpoint) blocking_this_combat: Option<bool>,
    /// The attacking band this creature is in, shared by every member.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(in crate::game::state_checkpoint) attacking_band: Option<u8>,
    pub(in crate::game::state_checkpoint) activated_loyalty_this_turn: bool,
    pub(in crate::game::state_checkpoint) chosen_creature_type: Option<String>,
    /// The basic land type this permanent was told to be as it entered.
    pub(in crate::game::state_checkpoint) chosen_basic_land_type: Option<BasicLandTypeSnapshot>,
    /// An ordered find-and-replace pair chosen as this permanent entered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(in crate::game::state_checkpoint) chosen_basic_land_type_substitution:
        Option<[BasicLandTypeSnapshot; 2]>,
    /// The color this permanent was told to remember as it entered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(in crate::game::state_checkpoint) chosen_color: Option<ManaColorSnapshot>,
    pub(in crate::game::state_checkpoint) chosen_card_name: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub(in crate::game::state_checkpoint) enum AttackDefenderSnapshot {
    Player { seat: usize },
    Planeswalker { object_id: u32 },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::game::state_checkpoint) struct CombatDamageAssignmentSnapshot {
    pub(in crate::game::state_checkpoint) recipient: TargetSnapshot,
    pub(in crate::game::state_checkpoint) amount: u16,
}

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
