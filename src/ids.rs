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

/// Positional identity of one object binding retained while an effect
/// resolves.
///
/// Unlike a [`TargetIndex`], a binding is not part of the spell or ability's
/// stack payload: it is populated only while the effect program resolves and
/// is not subject to targeting restrictions or legality checks.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ObjectBindingIndex(u8);

impl ObjectBindingIndex {
    /// The first object binding in an ordinary single-binding effect.
    pub const PRIMARY: Self = Self(0);

    /// The number of independent object bindings one resolving effect can
    /// retain.
    pub(crate) const COUNT: usize = 8;

    /// Creates an authored object binding index within the supported binding
    /// space.
    ///
    /// # Panics
    ///
    /// Panics when `index` is not less than eight.
    #[must_use]
    pub const fn new(index: u8) -> Self {
        assert!(
            (index as usize) < Self::COUNT,
            "object binding index must be less than eight"
        );
        Self(index)
    }

    #[must_use]
    pub const fn index(self) -> usize {
        self.0 as usize
    }

    #[must_use]
    pub fn from_index(index: usize) -> Option<Self> {
        if index < Self::COUNT {
            u8::try_from(index).ok().map(Self)
        } else {
            None
        }
    }
}

/// Positional identity of one object-set binding retained while an effect
/// resolves.
///
/// Object-set bindings preserve a collection as one typed value rather than
/// assigning each member an [`ObjectBindingIndex`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ObjectSetBindingIndex(u8);

impl ObjectSetBindingIndex {
    /// The first object-set binding in an ordinary single-binding effect.
    pub const PRIMARY: Self = Self(0);

    /// The number of independent object-set bindings one resolving effect can
    /// retain.
    pub(crate) const COUNT: usize = 8;

    /// Creates an authored object-set binding index within the supported
    /// binding space.
    ///
    /// # Panics
    ///
    /// Panics when `index` is not less than eight.
    #[must_use]
    pub const fn new(index: u8) -> Self {
        assert!(
            (index as usize) < Self::COUNT,
            "object-set binding index must be less than eight"
        );
        Self(index)
    }

    #[must_use]
    pub const fn index(self) -> usize {
        self.0 as usize
    }

    #[must_use]
    pub fn from_index(index: usize) -> Option<Self> {
        if index < Self::COUNT {
            u8::try_from(index).ok().map(Self)
        } else {
            None
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
    use super::{CardDefinitionId, ObjectBindingIndex, ObjectSetBindingIndex};

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

    #[test]
    fn object_binding_indices_use_eight_bounded_slots() {
        assert_eq!(ObjectBindingIndex::COUNT, 8);
        assert_eq!(ObjectBindingIndex::PRIMARY.index(), 0);

        for index in 0..ObjectBindingIndex::COUNT {
            let binding = ObjectBindingIndex::from_index(index).expect("supported binding index");
            assert_eq!(binding.index(), index);
            assert_eq!(
                ObjectBindingIndex::new(u8::try_from(index).expect("binding index fits in u8")),
                binding
            );
        }

        assert_eq!(
            ObjectBindingIndex::from_index(ObjectBindingIndex::COUNT),
            None
        );
        assert_eq!(ObjectBindingIndex::from_index(usize::MAX), None);
    }

    #[test]
    fn object_set_binding_indices_use_eight_bounded_slots() {
        assert_eq!(ObjectSetBindingIndex::COUNT, 8);
        assert_eq!(ObjectSetBindingIndex::PRIMARY.index(), 0);

        for index in 0..ObjectSetBindingIndex::COUNT {
            let binding =
                ObjectSetBindingIndex::from_index(index).expect("supported binding index");
            assert_eq!(binding.index(), index);
            assert_eq!(
                ObjectSetBindingIndex::new(u8::try_from(index).expect("binding index fits in u8"),),
                binding
            );
        }

        assert_eq!(
            ObjectSetBindingIndex::from_index(ObjectSetBindingIndex::COUNT),
            None
        );
        assert_eq!(ObjectSetBindingIndex::from_index(usize::MAX), None);
    }
}
