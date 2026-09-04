use std::fmt;
use std::num::NonZeroU64;

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

/// Stable identity of a card in the card catalog.
///
/// Values are positive integers no greater than [`Self::MAX`], so every ID is
/// represented exactly by a JavaScript `number`. Missing or hidden identity
/// is represented by the surrounding type rather than by a reserved value.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CardDefinitionId(NonZeroU64);

impl CardDefinitionId {
    /// Largest exactly representable ID the engine assigns.
    pub const MAX: u64 = (1_u64 << 52) - 1;

    /// Creates a JavaScript-safe card definition ID.
    ///
    /// # Panics
    ///
    /// Panics when `raw` is zero or exceeds [`Self::MAX`]. Use
    /// [`Self::try_new`] at untrusted input boundaries.
    #[must_use]
    pub const fn new(raw: u64) -> Self {
        assert!(raw > 0, "card definition IDs must be nonzero");
        assert!(
            raw <= Self::MAX,
            "card definition IDs must be JavaScript-safe"
        );
        Self(NonZeroU64::new(raw).expect("card definition ID was checked as nonzero"))
    }

    #[must_use]
    pub const fn try_new(raw: u64) -> Option<Self> {
        if raw == 0 || raw > Self::MAX {
            None
        } else {
            match NonZeroU64::new(raw) {
                Some(raw) => Some(Self(raw)),
                None => None,
            }
        }
    }

    #[must_use]
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

impl From<CardDefinitionId> for u64 {
    fn from(id: CardDefinitionId) -> Self {
        id.get()
    }
}

impl fmt::Display for CardDefinitionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(formatter)
    }
}

impl Serialize for CardDefinitionId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.get())
    }
}

impl<'de> Deserialize<'de> for CardDefinitionId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = u64::deserialize(deserializer)?;
        Self::try_new(raw).ok_or_else(|| {
            de::Error::custom(format_args!(
                "card definition ID must be between 1 and {}",
                Self::MAX
            ))
        })
    }
}

/// Identity of one logical rules component within a card definition.
///
/// Parts include faces of double-faced cards and halves of split cards. The
/// identifier is local to its [`CardDefinitionId`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CardPartId(pub u8);

impl CardPartId {
    /// The sole part of an ordinary card, or the primary/front part of a
    /// structured card.
    pub const PRIMARY: Self = Self(0);
}

/// Positional identity of one ability attached to a card part.
///
/// The same definition can create many independent ability objects during a
/// game. Those objects receive [`GameObjectId`]s; this identifier continues to
/// name the ability in the card's ordered rules definition. Reusable ability
/// definitions carry no identity of their own.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AbilityId(pub u8);

impl AbilityId {
    /// The first ability in an ordinary single-ability card part.
    pub const PRIMARY: Self = Self(0);

    #[must_use]
    pub const fn index(self) -> usize {
        self.0 as usize
    }

    #[must_use]
    pub fn from_index(index: usize) -> Option<Self> {
        u8::try_from(index).ok().map(Self)
    }
}

/// Identity of one ability-granting effect within an attached source ability.
///
/// Unlike [`AbilityId`], this is local to the effect tree of a single ability
/// clause. Keeping it separate lets a reusable, ID-free ability definition be
/// granted from more than one structural site without conflating provenance.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GrantId(pub u8);

impl GrantId {
    pub const PRIMARY: Self = Self(0);

    #[must_use]
    pub const fn index(self) -> usize {
        self.0 as usize
    }

    #[must_use]
    pub fn from_index(index: usize) -> Option<Self> {
        u8::try_from(index).ok().map(Self)
    }
}

/// Identity of one legal way to play a card, local to its card definition.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PlayOptionId(pub u8);

impl PlayOptionId {
    /// The ordinary play option synthesized for an unstructured card.
    pub const DEFAULT: Self = Self(0);
}

/// Positional identity of one rules-text mode, local to a play option.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ModeId(pub u8);

impl ModeId {
    #[must_use]
    pub const fn index(self) -> usize {
        self.0 as usize
    }

    #[must_use]
    pub fn from_index(index: usize) -> Option<Self> {
        u8::try_from(index).ok().map(Self)
    }
}

/// Positional reference to a target within one authored ability clause.
///
/// This is definition-local: instantiating modal branches remaps it to the
/// runtime [`TargetSlotId`] assigned to the resulting stack object. Split-card
/// presentation forms flatten the same way; executable combined-spell payload
/// composition is not yet supported.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TargetIndex(pub u8);

impl TargetIndex {
    pub const PRIMARY: Self = Self(0);

    #[must_use]
    pub const fn index(self) -> usize {
        self.0 as usize
    }

    #[must_use]
    pub fn from_index(index: usize) -> Option<Self> {
        u8::try_from(index).ok().map(Self)
    }
}

/// Positional reference to an object paid for a spell's additional cost.
///
/// The order is the order in which the cast's object costs were paid. Unlike
/// a target, this names the paid object itself and therefore remains useful
/// through last-known information after payment moves it to another zone.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AdditionalCostObjectIndex(pub u8);

impl AdditionalCostObjectIndex {
    pub const PRIMARY: Self = Self(0);

    #[must_use]
    pub const fn index(self) -> usize {
        self.0 as usize
    }

    #[must_use]
    pub fn from_index(index: usize) -> Option<Self> {
        u8::try_from(index).ok().map(Self)
    }
}

/// Positional reference to one optional additional-cost clause on a card.
///
/// Unlike [`AdditionalCostId`], this is authored against the ordered list of
/// optional additional costs rather than the ordered list of every ability.
/// Adding an unrelated printed clause therefore does not change which kicker
/// or other optional cost a declarative effect names.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AdditionalCostIndex(pub u8);

impl AdditionalCostIndex {
    pub const PRIMARY: Self = Self(0);
    pub const SECONDARY: Self = Self(1);

    #[must_use]
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

/// Authored identity of one value retained while an effect resolves.
///
/// Unlike a [`TargetIndex`], a binding is not part of the spell or ability's
/// target payload: it is populated only by the effect program and is not
/// subject to targeting restrictions or legality checks. The producer and
/// consumer determine whether the value is one object or an object set.
/// A compact reference to either a durable label or the direct lexical
/// parent's output. Labels are registered here once so the high-fanout effect
/// model carries only a compact identifier while declarations and diagnostics
/// retain their authored names.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Binding(u8);

const BINDING_LABELS: &[&str] = &[
    "atraxa_chosen",
    "atraxa_rest",
    "balance_creatures_kept",
    "balance_creatures_sacrificed",
    "balance_hand_discarded",
    "balance_hand_kept",
    "balance_lands_kept",
    "balance_lands_sacrificed",
    "branch_output",
    "cards",
    "conditional_cards",
    "consult_kicked_chosen",
    "consult_kicked_rest",
    "consult_normal_chosen",
    "consult_normal_rest",
    "delver_matching",
    "delver_other",
    "devourer_exiled",
    "devourer_top",
    "divine_reckoning_chosen",
    "divine_reckoning_destroyed",
    "domri_creature",
    "domri_noncreature",
    "empty_cards",
    "epic_experiment_castable",
    "epic_experiment_exiled",
    "epic_experiment_rest",
    "exiled_creature",
    "fact_chosen",
    "fact_first",
    "fact_second",
    "fact_unchosen",
    "grave_betrayal_card",
    "guild_feud_controller_chosen",
    "guild_feud_controller_rest",
    "guild_feud_opponent_chosen",
    "guild_feud_opponent_entered",
    "guild_feud_opponent_fighter",
    "guild_feud_opponent_rest",
    "haunted_fengraf_card",
    "healing_salve_target",
    "hideaway_hidden",
    "hideaway_rest",
    "intuition_chosen",
    "intuition_unchosen",
    "iteration_after_hand",
    "iteration_bottom",
    "iteration_exile",
    "iteration_hand",
    "jace_chosen",
    "jace_first",
    "jace_second",
    "jace_unchosen",
    "jarad_orders_graveyard",
    "karn_chosen",
    "karn_rest",
    "lair_bottom_cards",
    "lair_delved_cards",
    "liliana_chosen_pile",
    "liliana_first_pile",
    "liliana_second_pile",
    "liliana_spared_pile",
    "limited_resources_lands_kept",
    "limited_resources_lands_sacrificed",
    "manifest_dread_graveyard",
    "manifest_dread_permanent",
    "mercurial_chemister_discarded",
    "milled_card",
    "milled_cards",
    "nadu_land",
    "nadu_nonland",
    "object",
    "objects",
    "objects_2",
    "optional_card",
    "oracle_rest",
    "oracle_top",
    "outcome_owned_by_you",
    "paroxysm_land",
    "paroxysm_nonland",
    "produced_cards",
    "random_graveyard_card",
    "random_graveyard_cards",
    "release_sacrificed_permanents",
    "release_spared_permanents",
    "revealed_card",
    "revealed_cards",
    "scry_bottom",
    "scry_ordered_bottom",
    "scry_ordered_top",
    "scry_top",
    "sphinx_chosen",
    "sphinx_first",
    "sphinx_second",
    "sphinx_unchosen",
    "surveil_graveyard",
    "surveil_top",
    "suspended_card",
    "top_card_chosen",
    "top_card_remainder",
    "ugin_sacrificed_permanents",
    "ugin_spared_permanents",
    "uncovered_clues_chosen",
    "uncovered_clues_remainder",
    "wilderness_remainder",
    "wilds_land",
];

#[allow(non_upper_case_globals)]
pub const ParentBinding: Binding = Binding(u8::MAX);

impl Binding {
    #[doc(hidden)]
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn from_label(label: &'static str) -> Self {
        assert!(BINDING_LABELS.len() < u8::MAX as usize);
        let mut index = 0;
        while index < BINDING_LABELS.len() {
            if const_str_eq(BINDING_LABELS[index], label) {
                return Self(index as u8);
            }
            index += 1;
        }
        panic!("binding label is not registered in ids.rs")
    }

    #[must_use]
    pub const fn label(self) -> Option<&'static str> {
        if self.0 == u8::MAX {
            None
        } else {
            Some(BINDING_LABELS[self.0 as usize])
        }
    }
}

const fn const_str_eq(left: &str, right: &str) -> bool {
    let left = left.as_bytes();
    let right = right.as_bytes();
    if left.len() != right.len() {
        return false;
    }
    let mut index = 0;
    while index < left.len() {
        if left[index] != right[index] {
            return false;
        }
        index += 1;
    }
    true
}

impl fmt::Debug for Binding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.label() {
            Some(label) => formatter.debug_tuple("Binding").field(&label).finish(),
            None => formatter.write_str("ParentBinding"),
        }
    }
}

/// Identity of one independently chosen target slot on an instantiated spell
/// or ability. Slots are assigned in flattened target-clause order.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TargetSlotId(pub u8);

impl TargetSlotId {
    #[must_use]
    pub const fn index(self) -> usize {
        self.0 as usize
    }

    #[must_use]
    pub fn from_index(index: usize) -> Option<Self> {
        u8::try_from(index).ok().map(Self)
    }
}

/// Identity of an alternative cost choice, local to one play option.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AlternativeCostId(pub u8);

/// Identity of an additional cost choice, local to its card definition.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AdditionalCostId(pub u8);

/// Identity of a recipe that can combine two physical cards into one melded
/// game object. No supported format currently executes meld actions, but card
/// topology can refer to a recipe without conflating it with a card face.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MeldRecipeId(pub u16);

/// Identity of one physical piece of cardboard for the duration of a game.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PhysicalCardId(pub u32);

/// Identity of one rules object in its current zone.
///
/// A true zone change creates a new identity. Turning a card face up,
/// transforming it, or phasing it out does not.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GameObjectId(pub u32);

/// Compatibility name for callers written before physical cards and game
/// objects had separate identities. Prefer [`GameObjectId`] in new code.
///
/// This is the same type, not a distinct one, so nothing here stops a
/// protocol 1 caller from carrying an ID across a zone change — which is now
/// wrong. `#[deprecated]` is deliberately absent: on a `use` re-export the
/// attribute never reaches callers, and a `type` alias that would carry it
/// cannot be used as a constructor, so it would break every `CardInstanceId(n)`
/// in the wild. The migration pressure has to come from the changelog.
pub use GameObjectId as CardInstanceId;

/// Compatibility name for callers written before stack objects shared the
/// global game-object identity space. Prefer [`GameObjectId`] in new code.
///
/// The same caveat as [`CardInstanceId`] applies: this is an alias, not a
/// separate type, and it carries no compiler warning.
pub use GameObjectId as StackObjectId;

/// One of the two players in a game.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PlayerId {
    One,
    Two,
}

impl PlayerId {
    #[must_use]
    pub const fn opponent(self) -> Self {
        match self {
            Self::One => Self::Two,
            Self::Two => Self::One,
        }
    }

    #[must_use]
    pub const fn index(self) -> usize {
        match self {
            Self::One => 0,
            Self::Two => 1,
        }
    }
}

impl fmt::Display for PlayerId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::One => formatter.write_str("player one"),
            Self::Two => formatter.write_str("player two"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BINDING_LABELS, Binding, CardDefinitionId};

    #[test]
    fn binding_label_registry_is_sorted_unique_and_round_trips() {
        for labels in BINDING_LABELS.windows(2) {
            assert!(
                labels[0] < labels[1],
                "binding labels must be sorted and unique"
            );
        }
        for label in BINDING_LABELS {
            assert_eq!(Binding::from_label(label).label(), Some(*label));
        }
    }

    #[test]
    fn card_definition_ids_serde_as_exact_javascript_numbers() {
        let id = CardDefinitionId::new(1_u64 << 40);
        let encoded = serde_json::to_value(id).expect("ID serializes");
        assert_eq!(encoded.as_u64(), Some(id.get()));
        assert_eq!(
            serde_json::from_value::<CardDefinitionId>(encoded).expect("ID deserializes"),
            id,
        );
        assert!(serde_json::from_value::<CardDefinitionId>(serde_json::json!(0)).is_err());
        assert!(
            serde_json::from_value::<CardDefinitionId>(serde_json::json!(
                CardDefinitionId::MAX + 1
            ))
            .is_err()
        );
    }
}
