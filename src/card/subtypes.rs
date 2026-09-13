//! Rules-owned subtype vocabulary and const-compiled sets.
//!
//! Card authors keep literal names. Constructors validate and compile them;
//! runtime characteristics use the same typed vocabulary in every zone.

use super::{CardType, CardTypeSet};

mod set;
pub(crate) use set::SubtypeList;
pub use set::SubtypeSet;

/// The subtype families in CR 205.3. Creature/Kindred and Instant/Sorcery
/// share families. Knowing a family does not imply its card type is supported.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SubtypeFamily {
    Artifact,
    Enchantment,
    Land,
    Planeswalker,
    Spell,
    Creature,
    Planar,
    Dungeon,
    Battle,
}

impl SubtypeFamily {
    pub const ALL: [Self; 9] = [
        Self::Artifact,
        Self::Enchantment,
        Self::Land,
        Self::Planeswalker,
        Self::Spell,
        Self::Creature,
        Self::Planar,
        Self::Dungeon,
        Self::Battle,
    ];

    const fn bit(self) -> u16 {
        1 << self as u16
    }

    #[must_use]
    pub const fn allowed_on(self, types: CardTypeSet) -> bool {
        match self {
            Self::Artifact => types.contains(CardType::Artifact),
            Self::Enchantment => types.contains(CardType::Enchantment),
            Self::Land => types.contains(CardType::Land),
            Self::Planeswalker => types.contains(CardType::Planeswalker),
            Self::Spell => types.contains(CardType::Instant) || types.contains(CardType::Sorcery),
            Self::Creature => {
                types.contains(CardType::Creature) || types.contains(CardType::Kindred)
            }
            Self::Planar | Self::Dungeon | Self::Battle => false,
        }
    }
}

macro_rules! subtype_vocabulary {
    ($($variant:ident => [$name:literal $(, $alias:literal)*], [$($family:ident),+];)*) => {
        /// One subtype identity. Numeric positions are local implementation
        /// details; declarations and persistence use names, never these indices.
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub enum Subtype { $($variant,)* }

        #[allow(non_upper_case_globals)]
        mod hashes {
            $(pub const $variant: u64 = super::name_hash($name);)*
        }

        impl Subtype {
            pub const ALL: &[Self] = &[$(Self::$variant,)*];

            #[must_use]
            pub const fn name(self) -> &'static str {
                match self { $(Self::$variant => $name,)* }
            }

            #[must_use]
            pub const fn in_family(self, family: SubtypeFamily) -> bool {
                let families = match self {
                    $(Self::$variant => 0 $(| SubtypeFamily::$family.bit())+,)*
                };
                families & family.bit() != 0
            }

            /// Parse exact rules names, accepting apostrophe spelling aliases.
            #[must_use]
            pub const fn from_name(name: &str) -> Option<Self> {
                match name_hash(name) {
                    $(hashes::$variant if same_name(name, $name) $(|| same_name(name, $alias))* => Some(Self::$variant),)*
                    _ => None,
                }
            }
        }
    };
}

include!("subtypes/vocabulary.rs");

// Hash dispatch checks every candidate's exact spelling before accepting it.
// Apostrophe aliases share a bucket; subtype IDs never use these hash values.
const fn name_hash(name: &str) -> u64 {
    let bytes = name.as_bytes();
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    let mut index = 0;
    while index < bytes.len() {
        let byte = if bytes[index] == 0xe2
            && index + 2 < bytes.len()
            && bytes[index + 1] == 0x80
            && bytes[index + 2] == 0x99
        {
            index += 2;
            b'\''
        } else {
            bytes[index]
        };
        hash = (hash ^ byte as u64).wrapping_mul(0x0000_0100_0000_01b3);
        index += 1;
    }
    hash
}

const fn same_name(left: &str, right: &str) -> bool {
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

impl Subtype {
    /// Validate a name while constructing a declaration. Static declarations
    /// run this at compile time; dynamic callers can use `from_name` to fail softly.
    ///
    /// # Panics
    /// Panics if the name is absent from the rules vocabulary.
    #[must_use]
    pub const fn named(name: &str) -> Self {
        match Self::from_name(name) {
            Some(subtype) => subtype,
            None => panic!("unknown subtype name"),
        }
    }

    #[must_use]
    pub const fn allowed_on(self, types: CardTypeSet) -> bool {
        let mut index = 0;
        while index < SubtypeFamily::ALL.len() {
            let family = SubtypeFamily::ALL[index];
            if self.in_family(family) && family.allowed_on(types) {
                return true;
            }
            index += 1;
        }
        false
    }
}

const fn family_count(family: SubtypeFamily) -> usize {
    let mut count = 0;
    let mut index = 0;
    while index < Subtype::ALL.len() {
        if Subtype::ALL[index].in_family(family) {
            count += 1;
        }
        index += 1;
    }
    count
}

const fn family_names<const N: usize>(family: SubtypeFamily) -> [&'static str; N] {
    let mut names = [""; N];
    let mut next = 0;
    let mut index = 0;
    while index < Subtype::ALL.len() {
        let subtype = Subtype::ALL[index];
        if subtype.in_family(family) {
            names[next] = subtype.name();
            next += 1;
        }
        index += 1;
    }
    assert!(next == N);
    names
}

/// Legal choices are independent of the loaded card catalog.
pub const CREATURE_TYPES: &[&str] =
    &family_names::<{ family_count(SubtypeFamily::Creature) }>(SubtypeFamily::Creature);
pub const LAND_SUBTYPES: &[&str] =
    &family_names::<{ family_count(SubtypeFamily::Land) }>(SubtypeFamily::Land);
pub const NONBASIC_LAND_SUBTYPES: &[&str] = &const {
    let mut names = [""; family_count(SubtypeFamily::Land) - 5];
    let mut next = 0;
    let mut index = 0;
    while index < Subtype::ALL.len() {
        let subtype = Subtype::ALL[index];
        if subtype.in_family(SubtypeFamily::Land)
            && !matches!(
                subtype,
                Subtype::Plains
                    | Subtype::Island
                    | Subtype::Swamp
                    | Subtype::Mountain
                    | Subtype::Forest
            )
        {
            names[next] = subtype.name();
            next += 1;
        }
        index += 1;
    }
    names
};

#[must_use]
pub fn creature_type_name(name: &str) -> Option<&'static str> {
    Subtype::from_name(name)
        .filter(|subtype| subtype.in_family(SubtypeFamily::Creature))
        .map(Subtype::name)
}

#[cfg(test)]
mod tests;
