// Lexical effect-output scope and the independent card-part cost namespace.
// Included into references.rs to share its validation vocabulary.

#[derive(Default)]
struct BindingRegistry {
    labels: std::cell::RefCell<Vec<&'static str>>,
    declared_labels: std::cell::RefCell<Vec<&'static str>>,
    next_parent: std::cell::Cell<u8>,
    parent_reads: std::cell::Cell<u64>,
    binding_reads: std::cell::Cell<u64>,
}

#[derive(Clone, Copy)]
struct BindingScope<'registry> {
    cost_bindings: &'registry [Binding],
    objects: u64,
    object_sets: u64,
    card_names: u64,
    escaping_object_sets: u64,
    parent_object: Option<u8>,
    parent_object_set: Option<u8>,
    bindings: &'registry BindingRegistry,
}

impl<'registry> BindingScope<'registry> {
    fn empty(bindings: &'registry BindingRegistry) -> Self {
        Self {
            cost_bindings: &[],
            objects: 0,
            object_sets: 0,
            card_names: 0,
            escaping_object_sets: 0,
            parent_object: None,
            parent_object_set: None,
            bindings,
        }
    }

    fn binding_bit(
        self,
        binding: Binding,
        create: bool,
    ) -> Result<Option<u64>, GrantedAbilityValidationError> {
        let Some(label) = binding.label() else {
            return Ok(None);
        };
        if label.is_empty() {
            return Err(
                GrantedAbilityValidationError::UnsupportedEffectProgramContext {
                    context: "binding",
                    operation: "an empty binding label",
                },
            );
        }
        if let Some(index) = self
            .bindings
            .labels
            .borrow()
            .iter()
            .position(|bound| *bound == label)
        {
            return Ok(Some(1_u64 << index));
        }
        if !create {
            return Ok(None);
        }
        let mut bindings = self.bindings.labels.borrow_mut();
        if bindings.len() == u64::BITS as usize {
            return Err(
                GrantedAbilityValidationError::UnsupportedEffectProgramContext {
                    context: "binding",
                    operation: "more than 64 distinct bindings in one ability",
                },
            );
        }
        let bit = 1_u64 << bindings.len();
        bindings.push(label);
        Ok(Some(bit))
    }

    fn next_parent(self) -> Result<u8, GrantedAbilityValidationError> {
        let next = self.bindings.next_parent.get();
        if next == 64 {
            return Err(
                GrantedAbilityValidationError::UnsupportedEffectProgramContext {
                    context: "parent binding",
                    operation: "more than 64 nested parent bindings in one ability",
                },
            );
        }
        self.bindings.next_parent.set(next + 1);
        Ok(next)
    }

    fn declare_binding(self, binding: Binding) -> Result<u64, GrantedAbilityValidationError> {
        let Some(label) = binding.label() else {
            return Err(
                GrantedAbilityValidationError::UnsupportedEffectProgramContext {
                    context: "binding",
                    operation: "a durable binding cannot use ParentBinding",
                },
            );
        };
        if self.bindings.declared_labels.borrow().contains(&label) {
            return Err(GrantedAbilityValidationError::BindingAlreadyDeclared { binding });
        }
        let bit = self
            .binding_bit(binding, true)?
            .expect("declaring a binding assigns a label bit");
        self.bindings.declared_labels.borrow_mut().push(label);
        Ok(bit)
    }

    fn with_declared_object_set(
        self,
        binding: Binding,
    ) -> Result<Self, GrantedAbilityValidationError> {
        let bit = self
            .binding_bit(binding, false)?
            .expect("the effect output binding was declared while validating the effect");
        Ok(Self {
            object_sets: self.object_sets | bit,
            ..self
        })
    }

    fn with_escaping_object_sets(
        self,
        bindings: &[Binding],
    ) -> Result<Self, GrantedAbilityValidationError> {
        let mut escaping = self.escaping_object_sets;
        for binding in bindings {
            let Some(bit) = self.binding_bit(*binding, true)? else {
                continue;
            };
            escaping |= bit;
        }
        Ok(Self {
            escaping_object_sets: escaping,
            ..self
        })
    }

    fn object_set_may_escape(self, binding: Binding) -> bool {
        self.binding_bit(binding, false)
            .ok()
            .flatten()
            .is_some_and(|bit| self.escaping_object_sets & bit != 0)
    }

    fn parent_binding_was_read(self) -> bool {
        self.parent_object
            .or(self.parent_object_set)
            .is_some_and(|binding| self.bindings.parent_reads.get() & (1_u64 << binding) != 0)
    }

    fn binding_was_read(self, binding: Binding) -> bool {
        self.binding_bit(binding, false)
            .ok()
            .flatten()
            .is_some_and(|bit| self.bindings.binding_reads.get() & bit != 0)
    }

    fn with_object(self, binding: Binding) -> Result<Self, GrantedAbilityValidationError> {
        if binding == crate::ParentBinding {
            return Ok(Self {
                parent_object: Some(self.next_parent()?),
                parent_object_set: None,
                ..self
            });
        }
        let bit = self.declare_binding(binding)?;
        if (self.objects | self.object_sets) & bit != 0 {
            Err(GrantedAbilityValidationError::BindingAlreadyDeclared { binding })
        } else {
            Ok(Self {
                objects: self.objects | bit,
                ..self
            })
        }
    }

    fn with_object_set(self, binding: Binding) -> Result<Self, GrantedAbilityValidationError> {
        if binding == crate::ParentBinding {
            return Ok(Self {
                parent_object: None,
                parent_object_set: Some(self.next_parent()?),
                ..self
            });
        }
        let bit = self.declare_binding(binding)?;
        if self.objects & bit != 0 || self.object_sets & bit != 0 {
            Err(GrantedAbilityValidationError::BindingAlreadyDeclared { binding })
        } else {
            Ok(Self {
                object_sets: self.object_sets | bit,
                ..self
            })
        }
    }

    fn validate_object_reference(
        self,
        binding: Binding,
    ) -> Result<(), GrantedAbilityValidationError> {
        if binding == crate::ParentBinding {
            let Some(parent) = self.parent_object else {
                return Err(
                    GrantedAbilityValidationError::ObjectBindingReferenceOutOfScope { binding },
                );
            };
            self.bindings
                .parent_reads
                .set(self.bindings.parent_reads.get() | (1_u64 << parent));
            return Ok(());
        }
        if self
            .binding_bit(binding, false)?
            .is_some_and(|bit| self.objects & bit != 0)
        {
            let bit = self
                .binding_bit(binding, false)?
                .expect("the object binding was found in scope");
            self.bindings
                .binding_reads
                .set(self.bindings.binding_reads.get() | bit);
            Ok(())
        } else {
            Err(GrantedAbilityValidationError::ObjectBindingReferenceOutOfScope { binding })
        }
    }

    fn validate_object_set_reference(
        self,
        binding: Binding,
    ) -> Result<(), GrantedAbilityValidationError> {
        if binding == crate::ParentBinding {
            let Some(parent) = self.parent_object_set else {
                return Err(
                    GrantedAbilityValidationError::ObjectSetBindingReferenceOutOfScope { binding },
                );
            };
            self.bindings
                .parent_reads
                .set(self.bindings.parent_reads.get() | (1_u64 << parent));
            return Ok(());
        }
        if self
            .binding_bit(binding, false)?
            .is_some_and(|bit| self.object_sets & bit != 0)
        {
            let bit = self
                .binding_bit(binding, false)?
                .expect("the object-set binding was found in scope");
            self.bindings
                .binding_reads
                .set(self.bindings.binding_reads.get() | bit);
            Ok(())
        } else {
            Err(GrantedAbilityValidationError::ObjectSetBindingReferenceOutOfScope { binding })
        }
    }
}

include!("name_binding_scope.rs");
