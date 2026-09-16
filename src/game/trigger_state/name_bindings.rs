impl EffectResolutionContext {
    pub(super) fn card_name_label(&self, label: &str) -> Option<String> {
        self.bindings
            .with(|bindings| match bindings.values.get(label) {
                Some(EffectBindingValue::CardName(name)) => Some(name.clone()),
                _ => None,
            })
    }

    pub(super) fn card_name(&self, binding: Binding) -> Option<String> {
        binding
            .label()
            .and_then(|label| self.card_name_label(label))
    }

    pub(super) fn bind_runtime_card_name(&mut self, binding: &RuntimeBinding, name: String) {
        let RuntimeBinding::Label(label) = binding else {
            unreachable!("catalog validation rejected ParentBinding for a card name")
        };
        self.bindings.with_mut(|bindings| {
            bindings
                .values
                .insert(label.clone(), EffectBindingValue::CardName(name));
        });
    }
}

impl EffectResolutionContext {
    pub(super) fn bound_value(&self, binding: Binding) -> Option<i32> {
        let label = binding.label()?;
        self.bindings
            .with(|bindings| match bindings.values.get(label) {
                Some(EffectBindingValue::Number(value)) => Some(*value),
                _ => None,
            })
    }

    pub(super) fn bind_value(&mut self, binding: Binding, value: i32) {
        let label = binding
            .label()
            .expect("validated number binding has a label");
        self.bindings.with_mut(|bindings| {
            bindings
                .values
                .insert(label.to_owned(), EffectBindingValue::Number(value));
        });
    }
}

impl EffectResolutionContext {
    pub(super) fn creature_type_label(&self, label: &str) -> Option<String> {
        self.bindings
            .with(|bindings| match bindings.values.get(label) {
                Some(EffectBindingValue::CreatureType(name)) => Some(name.clone()),
                _ => None,
            })
    }

    pub(super) fn creature_type(&self, binding: Binding) -> Option<String> {
        binding
            .label()
            .and_then(|label| self.creature_type_label(label))
    }

    pub(super) fn bind_runtime_creature_type(&mut self, binding: &RuntimeBinding, name: String) {
        let RuntimeBinding::Label(label) = binding else {
            unreachable!("catalog validation rejected ParentBinding for a card name")
        };
        self.bindings.with_mut(|bindings| {
            bindings
                .values
                .insert(label.clone(), EffectBindingValue::CreatureType(name));
        });
    }
}
