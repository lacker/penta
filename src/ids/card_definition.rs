use super::CardDefinitionKey;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt;
use std::num::NonZeroU32;
use std::sync::{LazyLock, RwLock};

include!(concat!(env!("OUT_DIR"), "/compiled_card_keys.rs"));

/// A compact reference in this engine process's definition namespace.
///
/// Built-in references are generated at compile time from natural keys. Custom
/// keys are interned on entry, never during a catalog lookup. Neither allocation
/// order nor the numeric representation is part of the persistence contract:
/// serialization and ordering always use the natural key.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct CardDefinitionId(NonZeroU32);

#[derive(Default)]
struct CustomKeys {
    indices: HashMap<CardDefinitionKey, usize>,
    keys: Vec<CardDefinitionKey>,
}

impl CustomKeys {
    fn intern(&mut self, key: CardDefinitionKey) -> usize {
        *self.indices.entry(key).or_insert_with(|| {
            let index = self.keys.len();
            self.keys.push(key);
            index
        })
    }
}

// Automatically allocated keys live for the engine process, just like the
// compiled key table. Game/catalog membership is checked by each catalog.
static CUSTOM_KEYS: LazyLock<RwLock<CustomKeys>> =
    LazyLock::new(|| RwLock::new(CustomKeys::default()));

impl CardDefinitionId {
    pub(crate) const fn from_compiled(index: u32) -> Self {
        assert!((index as usize) < COMPILED_KEYS.len());
        Self(NonZeroU32::new(index + 1).unwrap())
    }

    /// Parses and resolves a natural key at an input/definition boundary.
    ///
    /// # Panics
    /// Panics if the UUID is malformed or nil.
    #[must_use]
    pub fn from_uuid(key: &str) -> Self {
        Self::from(CardDefinitionKey::from_uuid(key))
    }

    #[must_use]
    pub fn try_from_uuid(key: &str) -> Option<Self> {
        CardDefinitionKey::try_from_uuid(key).map(Self::from)
    }

    /// The natural key, independent of this build's internal numbering.
    #[must_use]
    pub fn key(self) -> CardDefinitionKey {
        let index = self.0.get() as usize - 1;
        if index < COMPILED_KEYS.len() {
            COMPILED_KEYS[index]
        } else {
            CUSTOM_KEYS
                .read()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .keys[index - COMPILED_KEYS.len()]
        }
    }

    /// Returns the natural UUID used by wire consumers, never an internal ID.
    #[must_use]
    pub fn get(self) -> String {
        self.key().to_string()
    }

    /// Built-in programs use direct indexing; custom catalogs retain a sparse
    /// map so unrelated custom registrations cannot inflate their storage.
    #[inline]
    pub(crate) const fn compiled_index(self) -> Option<usize> {
        let index = self.0.get() as usize - 1;
        if index < COMPILED_KEYS.len() {
            Some(index)
        } else {
            None
        }
    }
}

impl From<CardDefinitionKey> for CardDefinitionId {
    fn from(key: CardDefinitionKey) -> Self {
        if let Ok(index) = COMPILED_KEYS.binary_search(&key) {
            return Self::from_compiled(u32::try_from(index).unwrap());
        }
        let index = CUSTOM_KEYS
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .intern(key);
        let raw = u32::try_from(COMPILED_KEYS.len() + index + 1)
            .expect("process card-definition namespace exhausted");
        Self(NonZeroU32::new(raw).unwrap())
    }
}

impl Ord for CardDefinitionId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // The generated table is sorted by natural key, preserving observable
        // order without resolving those keys during ordinary built-in play.
        if self.compiled_index().is_some() && other.compiled_index().is_some() {
            self.0.cmp(&other.0)
        } else {
            self.key().cmp(&other.key())
        }
    }
}

impl PartialOrd for CardDefinitionId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for CardDefinitionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.key().fmt(formatter)
    }
}

impl fmt::Debug for CardDefinitionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.key().fmt(formatter)
    }
}

impl Serialize for CardDefinitionId {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.key().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CardDefinitionId {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        CardDefinitionKey::deserialize(deserializer).map(Self::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiled_card_ids_round_trip_natural_keys() {
        for (index, key) in COMPILED_KEYS.iter().enumerate() {
            let id = CardDefinitionId::from(*key);
            assert_eq!(id.compiled_index(), Some(index));
            assert_eq!(id.key(), *key);
            assert_eq!(serde_json::to_value(id).unwrap(), key.to_string());
        }
        assert_eq!(std::mem::size_of::<CardDefinitionId>(), 4);
        assert_eq!(std::mem::size_of::<Option<CardDefinitionId>>(), 4);
    }

    #[test]
    fn custom_key_allocation_order_is_not_identity_or_ordering() {
        let low = CardDefinitionKey::from_uuid("00000000-0000-0000-0000-0000000ff001");
        let high = CardDefinitionKey::from_uuid("ffffffff-ffff-ffff-ffff-fffffffffff1");
        let high_id = CardDefinitionId::from(high);
        let low_id = CardDefinitionId::from(low);
        assert!(high_id.compiled_index().is_none());
        assert_eq!(CardDefinitionId::from(high), high_id);
        assert!(low_id < high_id);
        let encoded = serde_json::to_value([high_id, low_id]).unwrap();
        assert_eq!(
            encoded,
            serde_json::json!([high.to_string(), low.to_string()])
        );
        assert_eq!(
            serde_json::from_value::<[CardDefinitionId; 2]>(encoded).unwrap(),
            [high_id, low_id]
        );
        let mut first = CustomKeys::default();
        let mut second = CustomKeys::default();
        let first_low = first.intern(low);
        second.intern(high);
        let second_low = second.intern(low);
        assert_ne!(first_low, second_low);
        assert_eq!(first.keys[first_low], second.keys[second_low]);
    }
}
