use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

/// Natural key of a card definition: its canonical printing UUID.
///
/// Catalogs derive their own dense indices from this key. Only the UUID is
/// serialized; no numeric allocation or historical ID registry is persistent.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CardDefinitionKey([u8; 16]);

impl CardDefinitionKey {
    /// Parses a canonical, lowercase, hyphenated UUID at a definition boundary.
    ///
    /// # Panics
    /// Panics for a malformed or nil UUID; use [`Self::try_from_uuid`] for input.
    #[must_use]
    pub const fn from_uuid(key: &str) -> Self {
        match Self::try_from_uuid(key) {
            Some(key) => key,
            None => panic!("card definition key must be a canonical UUID"),
        }
    }

    #[must_use]
    pub const fn try_from_uuid(key: &str) -> Option<Self> {
        let bytes = key.as_bytes();
        if bytes.len() != 36 {
            return None;
        }
        let mut value = 0_u128;
        let mut index = 0;
        while index < bytes.len() {
            let byte = bytes[index];
            if index == 8 || index == 13 || index == 18 || index == 23 {
                if byte != b'-' {
                    return None;
                }
            } else {
                let digit = match byte {
                    b'0'..=b'9' => byte - b'0',
                    b'a'..=b'f' => byte - b'a' + 10,
                    _ => return None,
                };
                value = (value << 4) | digit as u128;
            }
            index += 1;
        }
        if value == 0 {
            None
        } else {
            Some(Self(value.to_be_bytes()))
        }
    }

    /// Returns the natural key used in decks, observations and checkpoints.
    #[must_use]
    pub fn get(self) -> String {
        self.to_string()
    }
}

impl fmt::Display for CardDefinitionKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let raw = u128::from_be_bytes(self.0);
        write!(
            formatter,
            "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
            raw >> 96,
            (raw >> 80) & 0xffff,
            (raw >> 64) & 0xffff,
            (raw >> 48) & 0xffff,
            raw & 0xffff_ffff_ffff
        )
    }
}

impl fmt::Debug for CardDefinitionKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("CardDefinitionKey")
            .field(&self.to_string())
            .finish()
    }
}

impl Serialize for CardDefinitionKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for CardDefinitionKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let key = String::deserialize(deserializer)?;
        Self::try_from_uuid(&key)
            .ok_or_else(|| de::Error::custom("card definition key must be a canonical UUID"))
    }
}

mod card_definition;
pub use card_definition::CardDefinitionId;

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

/// Positional reference to an object paid for a spell or ability's object cost.
///
/// The order is the order in which the action's object costs were paid.
/// Unlike a target, this names the paid object itself and therefore remains
/// useful through last-known information after payment moves it to another
/// zone, or while a revealed object remains in its hand.
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

/// A natural binding name in its owning effect or card-part scope.
///
/// Declarations keep their authored names; runtime storage assigns private
/// slots within each resolution. Cost and effect bindings use separate scopes.
/// `ParentBinding` refers to the direct lexical parent's output.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Binding(&'static BindingName);

/// Compiler-owned storage emitted by `Binding!`; its address is never identity.
#[doc(hidden)]
#[derive(Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BindingName(&'static str);

impl BindingName {
    /// # Panics
    /// Panics if the authored name is empty.
    #[must_use]
    pub const fn new(label: &'static str) -> Self {
        assert!(!label.is_empty(), "binding names must not be empty");
        Self(label)
    }
}

#[allow(non_upper_case_globals)]
pub const ParentBinding: Binding = Binding(&BindingName(""));

impl Binding {
    /// References a compiler-owned name; equality compares names, not addresses.
    #[doc(hidden)]
    #[must_use]
    pub const fn named(name: &'static BindingName) -> Self {
        Self(name)
    }

    #[must_use]
    pub const fn label(self) -> Option<&'static str> {
        if self.0.0.is_empty() {
            None
        } else {
            Some(self.0.0)
        }
    }
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
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
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
    use super::{Binding, CardDefinitionId};

    #[test]
    fn binding_names_need_no_global_registration() {
        const LOCAL: Binding = crate::Binding!("a_new_local_name");
        assert_eq!(LOCAL.label(), Some("a_new_local_name"));
    }

    #[test]
    fn card_definition_keys_serialize_as_natural_uuids() {
        let key = "b13bf496-f3c0-4c13-8282-e7abfab6a198";
        let id = CardDefinitionId::from_uuid(key);
        let encoded = serde_json::to_value(id).unwrap();
        assert_eq!(encoded, key);
        assert_eq!(
            serde_json::from_value::<CardDefinitionId>(encoded).unwrap(),
            id
        );
        assert!(serde_json::from_value::<CardDefinitionId>(serde_json::json!(123)).is_err());
        assert!(CardDefinitionId::try_from_uuid("00000000-0000-0000-0000-000000000000").is_none());
        assert!(CardDefinitionId::try_from_uuid("not-a-uuid").is_none());
    }
}
