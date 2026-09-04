impl BindingScope<'_> {
    fn with_known_binding_labels(
        self,
        bindings: &[Binding],
    ) -> Result<Self, GrantedAbilityValidationError> {
        for binding in bindings {
            let _ = self.binding_bit(*binding, true)?;
        }
        Ok(self)
    }

    fn with_declared_card_name(
        self,
        binding: Binding,
    ) -> Result<Self, GrantedAbilityValidationError> {
        let bit = self
            .binding_bit(binding, false)?
            .expect("the card-name output binding was declared while validating the effect");
        Ok(Self {
            card_names: self.card_names | bit,
            ..self
        })
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
