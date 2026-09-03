use serde::{Deserialize, Serialize};

use super::{CopiableAbilitySnapshot, ObjectCharacteristicsSnapshot};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::game::state_checkpoint) struct CopiableCharacteristicsSnapshot {
    pub(in crate::game::state_checkpoint) base: ObjectCharacteristicsSnapshot,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(in crate::game::state_checkpoint) name: Option<String>,
    pub(in crate::game::state_checkpoint) added_types: [bool; crate::card::CardType::COUNT],
    #[serde(default)]
    pub(in crate::game::state_checkpoint) added_supertypes:
        [bool; crate::card::CardSupertype::COUNT],
    #[serde(default)]
    pub(in crate::game::state_checkpoint) removed_supertypes:
        [bool; crate::card::CardSupertype::COUNT],
    pub(in crate::game::state_checkpoint) added_abilities: Vec<CopiableAbilitySnapshot>,
    /// Additive: a checkpoint written before a copy could keep its own
    /// subtypes restores without them, which is what every copy did then.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(in crate::game::state_checkpoint) retain_printed_subtypes: bool,
    /// "Except it's a 1/1", which is a copiable value of its own. Additive:
    /// a checkpoint written before it existed restores a copy with none.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(in crate::game::state_checkpoint) base_power_toughness: Option<[i16; 2]>,
    /// The other exceptions embalm and eternalize print. Additive for the
    /// same reason: a checkpoint written before they existed restores a copy
    /// that made none of them.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(in crate::game::state_checkpoint) colors: Option<[bool; 5]>,
    #[serde(default, skip_serializing_if = "<[String]>::is_empty")]
    pub(in crate::game::state_checkpoint) added_creature_types: Vec<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub(in crate::game::state_checkpoint) no_mana_cost: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::game::state_checkpoint) struct DoubleFacedCopiableCharacteristicsSnapshot {
    pub(in crate::game::state_checkpoint) modal: bool,
    pub(in crate::game::state_checkpoint) front_part_id: u8,
    pub(in crate::game::state_checkpoint) back_part_id: u8,
    pub(in crate::game::state_checkpoint) front: CopiableCharacteristicsSnapshot,
    pub(in crate::game::state_checkpoint) back: CopiableCharacteristicsSnapshot,
}
