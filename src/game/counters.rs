use crate::card::CounterKind;

/// Sparse counter state shared by permanents, cards in other zones, and
/// players. Most objects carry no counters and the ones that do normally
/// carry only one kind, so storage follows the rules' named markers instead
/// of reserving a slot for every counter name the catalog knows about.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(super) struct Counters(Vec<(CounterKind, u16)>);

impl Counters {
    #[must_use]
    pub(super) const fn new() -> Self {
        Self(Vec::new())
    }

    #[must_use]
    pub(super) fn count(&self, kind: CounterKind) -> u16 {
        self.position(kind)
            .ok()
            .map_or(0, |position| self.0[position].1)
    }

    pub(super) fn add(&mut self, kind: CounterKind, amount: u16) {
        if amount == 0 {
            return;
        }
        match self.position(kind) {
            Ok(position) => {
                self.0[position].1 = self.0[position].1.saturating_add(amount);
            }
            Err(position) => self.0.insert(position, (kind, amount)),
        }
    }

    pub(super) fn remove(&mut self, kind: CounterKind, amount: u16) {
        let Ok(position) = self.position(kind) else {
            return;
        };
        let remaining = self.0[position].1.saturating_sub(amount);
        if remaining == 0 {
            self.0.remove(position);
        } else {
            self.0[position].1 = remaining;
        }
    }

    pub(super) fn set(&mut self, kind: CounterKind, amount: u16) {
        match (self.position(kind), amount) {
            (Ok(position), 0) => {
                self.0.remove(position);
            }
            (Ok(position), _) => self.0[position].1 = amount,
            (Err(position), 1..) => self.0.insert(position, (kind, amount)),
            (Err(_), 0) => {}
        }
    }

    pub(super) fn clear(&mut self) {
        self.0.clear();
    }

    #[must_use]
    pub(super) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(super) fn iter(&self) -> impl ExactSizeIterator<Item = (CounterKind, u16)> + '_ {
        self.0.iter().copied()
    }

    fn position(&self, kind: CounterKind) -> Result<usize, usize> {
        self.0.binary_search_by_key(&kind, |(held, _)| *held)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sparse_collection_adds_removes_and_drops_zero_entries() {
        let mut counters = Counters::new();
        counters.add(CounterKind::named("charge"), 2);
        counters.add(CounterKind::named("charge"), 3);
        counters.add(CounterKind::PlusOnePlusOne, 1);

        assert_eq!(counters.count(CounterKind::named("charge")), 5);
        assert_eq!(counters.count(CounterKind::PlusOnePlusOne), 1);
        assert_eq!(counters.iter().count(), 2);

        counters.remove(CounterKind::named("charge"), 5);
        assert_eq!(counters.count(CounterKind::named("charge")), 0);
        assert_eq!(counters.iter().count(), 1);
    }

    #[test]
    fn ordinary_counter_names_are_values_not_enum_variants() {
        let first = CounterKind::named("quest");
        let same_name = CounterKind::named("quest");
        let different = CounterKind::named("lore");

        assert_eq!(first, same_name);
        assert_ne!(first, different);
        assert_eq!(CounterKind::named("charge").name(), "charge");
    }

    #[test]
    fn power_toughness_and_keyword_families_carry_intrinsic_meaning() {
        assert_eq!(
            CounterKind::MinusZeroMinusTwo.power_toughness_bonus(),
            (0, -2)
        );
        assert_eq!(
            CounterKind::Indestructible.granted_keyword(),
            Some(crate::KeywordAbility::Indestructible)
        );
        for keyword in crate::KeywordCounter::ALL {
            assert!(
                crate::card::abilities::keyword_counter_ability(CounterKind::keyword(keyword))
                    .is_some(),
                "{keyword:?} has an intrinsic ability"
            );
        }
    }

    #[test]
    fn counter_keys_stay_compact_and_the_registry_has_no_collisions() {
        assert_eq!(std::mem::size_of::<CounterKind>(), 8);
        let mut known = CounterKind::KNOWN.to_vec();
        known.sort_unstable();
        known.dedup();
        assert_eq!(known.len(), CounterKind::KNOWN.len());
        for kind in CounterKind::KNOWN {
            assert_eq!(CounterKind::from_name(kind.name()), Some(kind));
        }
    }
}
