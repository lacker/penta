//! Semantic identity is independent of an ability's text or owning module.

/// A compact global identity. Names are hashed during constant evaluation;
/// runtime abilities, payments, and events carry only the numeric value.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MechanicId(u64);

impl MechanicId {
    #[must_use]
    pub const fn from_name(name: &str) -> Self {
        let bytes = name.as_bytes();
        let mut hash = 0xcbf2_9ce4_8422_2325_u64;
        let mut index = 0;
        while index < bytes.len() {
            hash ^= bytes[index] as u64;
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
            index += 1;
        }
        Self(hash)
    }
}

impl std::fmt::Display for MechanicId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Checkpoints use hex strings, avoiding JSON's 53-bit number limit.
        write!(formatter, "{:016x}", self.0)
    }
}

/// Ability identity and payment purpose use the same semantic vocabulary.
pub type AbilityLabel = MechanicId;
