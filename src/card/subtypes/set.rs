use super::{CardTypeSet, Subtype, SubtypeFamily};

/// A distinct set of rules-defined subtypes. Literal names are compiled by
/// const constructors; membership and family operations need no string scan.
///
/// ```
/// use penta::card::{Subtype, SubtypeSet};
/// const TYPES: SubtypeSet = SubtypeSet::from_names(&["Elf", "Druid", "Elf"]);
/// assert!(TYPES.contains(Subtype::Elf));
/// assert_eq!(TYPES.len(), 2);
/// ```
///
/// ```compile_fail
/// use penta::card::SubtypeSet;
/// const TYPES: SubtypeSet = SubtypeSet::from_names(&["Dargon"]);
/// ```
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct SubtypeSet([u64; Subtype::ALL.len().div_ceil(64)]);

impl SubtypeSet {
    pub const EMPTY: Self = Self([0; Subtype::ALL.len().div_ceil(64)]);

    /// # Panics
    /// Panics on an unknown subtype name; static declarations fail compilation.
    #[must_use]
    pub const fn from_names(names: &[&str]) -> Self {
        let mut set = Self::EMPTY;
        let mut index = 0;
        while index < names.len() {
            set.insert(Subtype::named(names[index]));
            index += 1;
        }
        set
    }

    #[must_use]
    const fn build_family(family: SubtypeFamily) -> Self {
        let mut set = Self::EMPTY;
        let mut index = 0;
        while index < Subtype::ALL.len() {
            let subtype = Subtype::ALL[index];
            if subtype.in_family(family) {
                set.insert(subtype);
            }
            index += 1;
        }
        set
    }

    #[must_use]
    pub const fn family(family: SubtypeFamily) -> Self {
        const FAMILIES: [SubtypeSet; SubtypeFamily::ALL.len()] = {
            let mut masks = [SubtypeSet::EMPTY; SubtypeFamily::ALL.len()];
            let mut index = 0;
            while index < masks.len() {
                masks[index] = SubtypeSet::build_family(SubtypeFamily::ALL[index]);
                index += 1;
            }
            masks
        };
        FAMILIES[family as usize]
    }

    #[must_use]
    pub const fn contains(self, subtype: Subtype) -> bool {
        let index = subtype as usize;
        self.0[index / 64] & (1 << (index % 64)) != 0
    }

    pub const fn insert(&mut self, subtype: Subtype) {
        let index = subtype as usize;
        self.0[index / 64] |= 1 << (index % 64);
    }

    pub const fn remove(&mut self, subtype: Subtype) {
        let index = subtype as usize;
        self.0[index / 64] &= !(1 << (index % 64));
    }

    #[must_use]
    pub const fn union(mut self, other: Self) -> Self {
        let mut index = 0;
        while index < self.0.len() {
            self.0[index] |= other.0[index];
            index += 1;
        }
        self
    }

    #[must_use]
    pub const fn intersection(mut self, other: Self) -> Self {
        let mut index = 0;
        while index < self.0.len() {
            self.0[index] &= other.0[index];
            index += 1;
        }
        self
    }

    #[must_use]
    pub const fn difference(mut self, other: Self) -> Self {
        let mut index = 0;
        while index < self.0.len() {
            self.0[index] &= !other.0[index];
            index += 1;
        }
        self
    }

    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.len() == 0
    }

    #[must_use]
    pub const fn len(self) -> usize {
        let mut count = 0;
        let mut index = 0;
        while index < self.0.len() {
            count += self.0[index].count_ones() as usize;
            index += 1;
        }
        count
    }

    /// Canonical vocabulary order, independent of insertion order.
    pub fn iter(self) -> impl Iterator<Item = Subtype> {
        self.0
            .into_iter()
            .enumerate()
            .flat_map(|(index, mut word)| {
                std::iter::from_fn(move || {
                    if word == 0 {
                        return None;
                    }
                    let bit = word.trailing_zeros() as usize;
                    word &= word - 1;
                    Some(Subtype::ALL[index * 64 + bit])
                })
            })
    }

    pub fn retain(&mut self, mut predicate: impl FnMut(Subtype) -> bool) {
        for subtype in self.iter() {
            if !predicate(subtype) {
                self.remove(subtype);
            }
        }
    }

    pub fn retain_for_card_types(&mut self, types: CardTypeSet) {
        let mut allowed = Self::EMPTY;
        for family in SubtypeFamily::ALL {
            if family.allowed_on(types) {
                allowed = allowed.union(Self::family(family));
            }
        }
        *self = self.intersection(allowed);
    }
}

impl Default for SubtypeSet {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl std::fmt::Debug for SubtypeSet {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_set()
            .entries(self.iter().map(Subtype::name))
            .finish()
    }
}

/// Authored order belongs to presentation; the derived mask belongs to rules
/// evaluation. Private fields ensure constructors cannot let them disagree.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct SubtypeList {
    names: &'static [&'static str],
    set: SubtypeSet,
}

impl SubtypeList {
    pub const fn new(names: &'static [&'static str]) -> Self {
        Self {
            names,
            set: SubtypeSet::from_names(names),
        }
    }
    pub const fn names(self) -> &'static [&'static str] {
        self.names
    }
    pub const fn set(self) -> SubtypeSet {
        self.set
    }
}
