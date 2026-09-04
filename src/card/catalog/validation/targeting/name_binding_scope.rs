impl BindingScope<'_> {
    fn with_card_name(self, binding: Binding) -> Result<Self, GrantedAbilityValidationError> {
        if binding == crate::ParentBinding {
            return Err(GrantedAbilityValidationError::UnsupportedEffectProgramContext {
                context: "card-name binding",
                operation: "a chosen card name requires a durable labeled binding",
            });
        }
        let bit = self.declare_binding(binding)?;
        if (self.objects | self.object_sets | self.card_names) & bit != 0 {
            Err(GrantedAbilityValidationError::BindingAlreadyDeclared { binding })
        } else {
            Ok(Self {
                card_names: self.card_names | bit,
                ..self
            })
        }
    }

    fn validate_card_name_binding_reference(
        self,
        binding: Binding,
    ) -> Result<(), GrantedAbilityValidationError> {
        let Some(bit) = self.binding_bit(binding, false)? else {
            // A name recorded by the source permanent is outside the lexical
            // effect-binding scope and is validated with that source.
            return Ok(());
        };
        if self.card_names & bit == 0 {
            return Err(GrantedAbilityValidationError::UnsupportedEffectProgramContext {
                context: "card-name binding",
                operation: "a binding declared for another value kind",
            });
        }
        self.bindings
            .binding_reads
            .set(self.bindings.binding_reads.get() | bit);
        Ok(())
    }
}
