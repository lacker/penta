use super::EffectBindingValue;
use std::collections::BTreeMap;

/// Slots are allocated by one resolution and never leave that scope.
/// The name map is authoritative at persistence boundaries.
#[derive(Clone, Debug, Default)]
pub(super) struct ScopedBindingValues {
    slots: BTreeMap<String, usize>,
    values: Vec<EffectBindingValue>,
}

impl ScopedBindingValues {
    pub(super) fn get(&self, name: &str) -> Option<&EffectBindingValue> {
        self.slots.get(name).map(|slot| &self.values[*slot])
    }

    pub(super) fn insert(&mut self, name: String, value: EffectBindingValue) {
        if let Some(slot) = self.slots.get(&name) {
            self.values[*slot] = value;
        } else {
            self.slots.insert(name, self.values.len());
            self.values.push(value);
        }
    }

    pub(super) fn values(&self) -> impl Iterator<Item = &EffectBindingValue> {
        self.slots.values().map(|slot| &self.values[*slot])
    }

    pub(super) fn values_mut(&mut self) -> impl Iterator<Item = &mut EffectBindingValue> {
        self.values.iter_mut()
    }

    pub(super) fn into_named_values(self) -> BTreeMap<String, EffectBindingValue> {
        self.slots
            .into_iter()
            .map(|(name, slot)| (name, self.values[slot].clone()))
            .collect()
    }

    pub(super) fn from_named_values(values: BTreeMap<String, EffectBindingValue>) -> Self {
        let mut scoped = Self::default();
        for (name, value) in values {
            scoped.insert(name, value);
        }
        scoped
    }
}

impl PartialEq for ScopedBindingValues {
    fn eq(&self, other: &Self) -> bool {
        self.slots.len() == other.slots.len()
            && self
                .slots
                .keys()
                .all(|name| self.get(name) == other.get(name))
    }
}
impl Eq for ScopedBindingValues {}

#[cfg(test)]
mod scoped_binding_tests {
    use super::super::*;

    #[test]
    fn scoped_bindings_are_independent_and_restore_by_name() {
        let name = crate::Binding!("local_test_result");
        let mut first = EffectResolutionContext::empty();
        let mut second = EffectResolutionContext::empty();
        first.bind_single_object(name, Some(Target::Permanent(GameObjectId(1))));
        second.bind_single_object(name, Some(Target::Permanent(GameObjectId(2))));
        first.bind_object_group(crate::Binding!("another_local_name"), Vec::new());
        let restored = EffectResolutionContext::from_bindings(
            TriggerContext::empty(),
            None,
            Vec::new(),
            first.bindings(),
        );
        assert_eq!(
            restored.single_object(name),
            Some(Target::Permanent(GameObjectId(1)))
        );
        assert_eq!(
            second.single_object(name),
            Some(Target::Permanent(GameObjectId(2)))
        );
        assert_eq!(restored.bindings(), first.bindings());
        assert_eq!(
            restored, first,
            "scope equality must ignore slot allocation order"
        );
        assert_eq!(
            restored.bindings().keys().cloned().collect::<Vec<_>>(),
            ["another_local_name", "local_test_result"]
        );
    }
}
