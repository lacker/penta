#[cfg(test)]
pub(in crate::card::catalog) fn validate_ability_targets(
    targets: &[AbilityTargetDef],
    effect: EffectDef,
) -> Result<(), GrantedAbilityValidationError> {
    if targets.len() > usize::from(u8::MAX) + 1 {
        return Err(GrantedAbilityValidationError::TooManyTargets {
            count: targets.len(),
        });
    }
    for (position, definition) in targets.iter().enumerate() {
        let target = TargetIndex::from_index(position)
            .expect("the target count was validated before assigning positional indices");
        if definition.minimum > definition.maximum {
            return Err(GrantedAbilityValidationError::InvalidTargetBounds {
                target,
                minimum: definition.minimum,
                maximum: definition.maximum,
            });
        }
    }
    let bindings = BindingRegistry::default();
    validate_effect_references(
        effect,
        targets.len(),
        BindingScope::empty(&bindings),
    )?;
    validate_effect_target_shapes(effect, targets, None)
}

#[cfg(test)]
pub(in crate::card::catalog) fn validate_replacement_ability_targets(
    targets: &[AbilityTargetDef],
    effect: ReplacementEffectDef,
) -> Result<(), GrantedAbilityValidationError> {
    validate_target_definitions(targets)?;
    let bindings = BindingRegistry::default();
    validate_replacement_effect_target_references(
        effect,
        targets.len(),
        BindingScope::empty(&bindings),
    )?;
    validate_replacement_effect_target_shapes(effect, targets)
}

pub(super) fn validate_ability_program_targets(
    targets: &[AbilityTargetDef],
    program: AbilityProgramDef,
    trigger_event: Option<TriggerEventDef>,
    chosen_cost_card_binding: Option<Binding>,
) -> Result<(), GrantedAbilityValidationError> {
    validate_target_definitions(targets)?;
    let bindings = BindingRegistry::default();
    let scope = chosen_cost_card_binding.map_or_else(
        || Ok(BindingScope::empty(&bindings)),
        |binding| BindingScope::empty(&bindings).with_object(binding),
    )?;
    validate_program_references(program, targets.len(), scope)?;
    validate_program_target_shapes(program, targets, trigger_event)
}

pub(super) fn validate_ability_trigger_event(
    event: TriggerEventDef,
    target_count: usize,
) -> Result<(), GrantedAbilityValidationError> {
    let bindings = BindingRegistry::default();
    validate_trigger_event_references(
        event,
        target_count,
        BindingScope::empty(&bindings),
    )
}

fn validate_program_references(
    program: AbilityProgramDef,
    target_count: usize,
    scope: BindingScope<'_>,
) -> Result<(), GrantedAbilityValidationError> {
    match program {
        AbilityProgramDef::Effects(effect) => {
            validate_effect_references(effect, target_count, scope)
        }
        AbilityProgramDef::Replacement(effect) => {
            validate_replacement_effect_target_references(effect, target_count, scope)
        }
    }
}

fn validate_target_definitions(
    targets: &[AbilityTargetDef],
) -> Result<(), GrantedAbilityValidationError> {
    if targets.len() > usize::from(u8::MAX) + 1 {
        return Err(GrantedAbilityValidationError::TooManyTargets {
            count: targets.len(),
        });
    }
    for (position, definition) in targets.iter().enumerate() {
        let target = TargetIndex::from_index(position)
            .expect("the target count was validated before assigning positional indices");
        if definition.minimum > definition.maximum {
            return Err(GrantedAbilityValidationError::InvalidTargetBounds {
                target,
                minimum: definition.minimum,
                maximum: definition.maximum,
            });
        }
        if definition
            .exact_count
            .is_some_and(|value| !cast_target_count_value_supported(value))
        {
            return Err(
                GrantedAbilityValidationError::UnsupportedEffectProgramContext {
                    context: "cast target count",
                    operation: "unsupported ValueDef",
                },
            );
        }
    }
    Ok(())
}

fn cast_target_count_value_supported(value: ValueDef) -> bool {
    match value {
        ValueDef::Constant(_) | ValueDef::ChosenX | ValueDef::AdditionalCostPayments(_) => true,
        ValueDef::Negate(value) => cast_target_count_value_supported(*value),
        ValueDef::Scaled(scaled) => cast_target_count_value_supported(scaled.value),
        ValueDef::Sum(sum) => {
            cast_target_count_value_supported(sum.left)
                && cast_target_count_value_supported(sum.right)
        }
        ValueDef::IfAdditionalCostPaid(conditional) => {
            cast_target_count_value_supported(conditional.if_paid)
                && cast_target_count_value_supported(conditional.otherwise)
        }
        ValueDef::Halved(halved) => cast_target_count_value_supported(halved.value),
        ValueDef::Quotient(quotient) => {
            cast_target_count_value_supported(quotient.numerator)
                && cast_target_count_value_supported(quotient.denominator)
        }
        _ => false,
    }
}

#[derive(Default)]
struct BindingRegistry {
    labels: std::cell::RefCell<Vec<&'static str>>,
    declared_labels: std::cell::RefCell<Vec<&'static str>>,
    next_parent: std::cell::Cell<u8>,
    parent_reads: std::cell::Cell<u64>,
    binding_reads: std::cell::Cell<u64>,
    chosen_name_reads: std::cell::Cell<u64>,
}

#[derive(Clone, Copy)]
struct BindingScope<'registry> {
    objects: u64,
    object_sets: u64,
    escaping_object_sets: u64,
    parent_object: Option<u8>,
    parent_object_set: Option<u8>,
    bindings: &'registry BindingRegistry,
}

impl<'registry> BindingScope<'registry> {
    fn empty(bindings: &'registry BindingRegistry) -> Self {
        Self {
            objects: 0,
            object_sets: 0,
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
            return Err(GrantedAbilityValidationError::UnsupportedEffectProgramContext {
                context: "binding",
                operation: "an empty binding label",
            });
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
            return Err(GrantedAbilityValidationError::UnsupportedEffectProgramContext {
                context: "binding",
                operation: "more than 64 distinct bindings in one ability",
            });
        }
        let bit = 1_u64 << bindings.len();
        bindings.push(label);
        Ok(Some(bit))
    }

    fn next_parent(self) -> Result<u8, GrantedAbilityValidationError> {
        let next = self.bindings.next_parent.get();
        if next == 64 {
            return Err(GrantedAbilityValidationError::UnsupportedEffectProgramContext {
                context: "parent binding",
                operation: "more than 64 nested parent bindings in one ability",
            });
        }
        self.bindings.next_parent.set(next + 1);
        Ok(next)
    }

    fn declare_binding(
        self,
        binding: Binding,
    ) -> Result<u64, GrantedAbilityValidationError> {
        let Some(label) = binding.label() else {
            return Err(GrantedAbilityValidationError::UnsupportedEffectProgramContext {
                context: "binding",
                operation: "a durable binding cannot use ParentBinding",
            });
        };
        if self.bindings.declared_labels.borrow().contains(&label) {
            return Err(GrantedAbilityValidationError::BindingAlreadyDeclared {
                binding,
            });
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

    fn chosen_name_read_count(self) -> u64 {
        self.bindings.chosen_name_reads.get()
    }

    fn mark_chosen_name_read(self) {
        self.bindings
            .chosen_name_reads
            .set(self.bindings.chosen_name_reads.get() + 1);
    }

    fn with_object(
        self,
        binding: Binding,
    ) -> Result<Self, GrantedAbilityValidationError> {
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

    fn with_object_set(
        self,
        binding: Binding,
    ) -> Result<Self, GrantedAbilityValidationError> {
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
                return Err(GrantedAbilityValidationError::ObjectBindingReferenceOutOfScope {
                    binding,
                });
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
                return Err(GrantedAbilityValidationError::ObjectSetBindingReferenceOutOfScope {
                    binding,
                });
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

fn validate_target_index(
    target: TargetIndex,
    target_count: usize,
) -> Result<(), GrantedAbilityValidationError> {
    if target.index() < target_count {
        Ok(())
    } else {
        Err(GrantedAbilityValidationError::TargetReferenceOutOfBounds {
            target,
            target_count,
        })
    }
}

pub(super) fn validate_ability_cost_target_references(
    costs: &[AbilityCostDef],
    targets: &[AbilityTargetDef],
) -> Result<(), GrantedAbilityValidationError> {
    for cost in costs {
        if let AbilityCostDef::ManaValueOfTarget { target, .. } = cost {
            validate_target_shape(*target, targets, RecipientExpectation::Object, true)?;
        }
    }
    Ok(())
}

fn validate_object_reference(
    reference: ObjectRefDef,
    target_count: usize,
    scope: BindingScope<'_>,
) -> Result<(), GrantedAbilityValidationError> {
    match reference {
        ObjectRefDef::Target(target) | ObjectRefDef::SourceOfTargetedStackObject(target) => {
            validate_target_index(target, target_count)
        }
        ObjectRefDef::Binding(binding) => scope.validate_object_reference(binding),
        ObjectRefDef::Source
        | ObjectRefDef::CreatingSource
        | ObjectRefDef::ZoneChangeSuccessor(_)
        | ObjectRefDef::ZoneChangeResultOfTriggeringObject
        | ObjectRefDef::AbilityGrantSource
        | ObjectRefDef::ResolvingObject
        | ObjectRefDef::AdditionalCostObject(_)
        | ObjectRefDef::AttachedToSource
        | ObjectRefDef::TriggeringObject
        | ObjectRefDef::DamagedObject => Ok(()),
    }
}

fn validate_player_reference(
    reference: PlayerRefDef,
    target_count: usize,
    scope: BindingScope<'_>,
) -> Result<(), GrantedAbilityValidationError> {
    match reference {
        PlayerRefDef::Target(target) => validate_target_index(target, target_count),
        PlayerRefDef::ControllerOf(reference)
        | PlayerRefDef::OpponentOf(reference)
        | PlayerRefDef::OwnerOf(reference) => {
            validate_object_reference(reference, target_count, scope)
        }
        PlayerRefDef::EffectController
        | PlayerRefDef::EnchantedPlayer
        | PlayerRefDef::EventPlayer
        | PlayerRefDef::Opponent => Ok(()),
    }
}

fn validate_payment_references(
    payment: EffectPaymentDef,
    target_count: usize,
    scope: BindingScope<'_>,
) -> Result<(), GrantedAbilityValidationError> {
    validate_single_payment_payer(payment.payer)?;
    validate_player_set(payment.payer, target_count, scope)?;
    if let EffectPaymentCostDef::GenericMana(amount) = payment.cost {
        validate_value_target_references(amount, target_count, scope)?;
    }
    Ok(())
}

fn validate_single_payment_payer(
    players: PlayerSetDef,
) -> Result<(), GrantedAbilityValidationError> {
    if matches!(
        players,
        PlayerSetDef::All | PlayerSetDef::Related(PlayerRelation::Any)
    ) {
        Err(GrantedAbilityValidationError::InvalidPaymentPayer { players })
    } else {
        Ok(())
    }
}

fn validate_damage_matcher_references(
    matcher: DamageEventMatcherDef,
    target_count: usize,
    scope: BindingScope<'_>,
) -> Result<(), GrantedAbilityValidationError> {
    match matcher.source {
        DamageSourceMatcherDef::Object(reference) | DamageSourceMatcherDef::Except(reference) => {
            validate_object_reference(reference, target_count, scope)?;
        }
        DamageSourceMatcherDef::Any
        | DamageSourceMatcherDef::Group(_)
        | DamageSourceMatcherDef::AffectedObject
        | DamageSourceMatcherDef::Matching(_) => {}
    }
    match matcher.recipient {
        DamageRecipientMatcherDef::Recipients(recipient) => {
            validate_recipient_target_references(recipient, target_count, scope)
        }
        DamageRecipientMatcherDef::PlayerAndCreaturesControlledBy(player) => {
            validate_player_reference(player, target_count, scope)
        }
        // A predicate names no target and no player, so there is nothing
        // here to check against the clause's declared slots.
        DamageRecipientMatcherDef::MatchingObject(_)
        | DamageRecipientMatcherDef::Any
        | DamageRecipientMatcherDef::AffectedObject
        | DamageRecipientMatcherDef::PlayerOrPlaneswalker => Ok(()),
    }
}


fn validate_player_set(
    players: PlayerSetDef,
    target_count: usize,
    scope: BindingScope<'_>,
) -> Result<(), GrantedAbilityValidationError> {
    match players {
        PlayerSetDef::One(reference) => validate_player_reference(reference, target_count, scope),
        PlayerSetDef::LegalTargets(target) => validate_target_index(target, target_count),
        PlayerSetDef::All | PlayerSetDef::Related(_) => Ok(()),
    }
}

fn validate_query(
    query: ObjectQueryDef,
    target_count: usize,
    scope: BindingScope<'_>,
) -> Result<(), GrantedAbilityValidationError> {
    if let Some(controller) = query.controller {
        validate_player_set(controller, target_count, scope)?;
    }
    if let Some(owner) = query.owner {
        validate_player_set(owner, target_count, scope)?;
    }
    if let Some(related_player) = query.related_player {
        validate_player_set(related_player, target_count, scope)?;
    }
    if let Some(relative) = query.relative_position {
        let reference = match relative {
            ZoneRelativePositionDef::Above(reference)
            | ZoneRelativePositionDef::Below(reference) => reference,
        };
        validate_object_reference(reference, target_count, scope)?;
    }
    Ok(())
}

fn validate_condition(
    condition: ConditionDef,
    target_count: usize,
    scope: BindingScope<'_>,
) -> Result<(), GrantedAbilityValidationError> {
    match condition {
        ConditionDef::Exists(query) => validate_query(query, target_count, scope),
        ConditionDef::ObjectCount(counting) => validate_query(counting.query, target_count, scope),
        // A turn count names nothing and reads nothing but the game.
        ConditionDef::ControllerTurnsTakenAtMost(_) => Ok(()),
        ConditionDef::All(conditions) => conditions
            .iter()
            .try_for_each(|condition| validate_condition(*condition, target_count, scope)),
    }
}

fn validate_trigger_condition(
    condition: TriggerConditionDef,
    target_count: usize,
    scope: BindingScope<'_>,
) -> Result<(), GrantedAbilityValidationError> {
    match condition {
        TriggerConditionDef::All(conditions) | TriggerConditionDef::AnyOf(conditions) => conditions
            .iter()
            .copied()
            .try_for_each(|condition| validate_trigger_condition(condition, target_count, scope)),
        TriggerConditionDef::Not(condition) => {
            validate_trigger_condition(*condition, target_count, scope)
        }
        TriggerConditionDef::ObjectCount { query, .. } => {
            validate_query(query, target_count, scope)
        }
        TriggerConditionDef::ObjectSetCount(condition) => {
            validate_object_set_target_references(*condition.objects, target_count, scope)?;
            condition.predicate.filter.map_or(Ok(()), |filter| {
                validate_object_predicate_references(filter.predicate(), target_count, scope)
            })
        }
        TriggerConditionDef::TargetMatches { slot, .. } => {
            validate_target_index(slot, target_count)
        }
        // The binding has to have been introduced by an enclosing choice:
        // a condition reading one that was never saved names nothing.
        TriggerConditionDef::BoundObjectMatches { binding, .. } => {
            validate_object_reference(ObjectRefDef::Binding(binding), target_count, scope)
        }
        TriggerConditionDef::ControllerHadPermanentLeaveThisTurn
        | TriggerConditionDef::ControllerHadCardLeaveGraveyardThisTurn
        | TriggerConditionDef::ControllerHasCitysBlessing
        | TriggerConditionDef::ControllerGainedLifeThisTurn
        | TriggerConditionDef::CreatureDiedThisTurn
        | TriggerConditionDef::SourceArrivedSinceControllersLastUpkeep
        | TriggerConditionDef::SacrificedObjectMatches(_)
        | TriggerConditionDef::SourceOnBattlefield
        | TriggerConditionDef::SourceInZone(_)
        | TriggerConditionDef::SourceUntapped
        | TriggerConditionDef::SourceIsPaired
        | TriggerConditionDef::ActivePlayer(_)
        | TriggerConditionDef::SpellsCastThisTurn { .. }
        | TriggerConditionDef::SpellsCastLastTurn { .. }
        | TriggerConditionDef::ControlsGreatestPowerCreature
        | TriggerConditionDef::SourceMatches { .. }
        | TriggerConditionDef::AttachedPermanentMatches { .. }
        | TriggerConditionDef::SourceCounters { .. }
        | TriggerConditionDef::SourceCastWith(_)
        | TriggerConditionDef::SourcePaidAdditionalCost(_)
        | TriggerConditionDef::SourceCastFrom(_)
        | TriggerConditionDef::SourceWasCast
        | TriggerConditionDef::SourceCastAtInstantSpeed
        | TriggerConditionDef::SourceLoyalty { .. }
        | TriggerConditionDef::SourceActivationsThisTurn { .. }
        | TriggerConditionDef::SourceResolutionsThisTurn { .. }
        | TriggerConditionDef::SourceDealtDamageToOpponentThisTurn
        | TriggerConditionDef::OpponentWasDealtDamageThisTurn
        | TriggerConditionDef::SourceIsTapped
        | TriggerConditionDef::SourceIsUntapped
        | TriggerConditionDef::ControllerLifeAtMost(_)
        | TriggerConditionDef::PlayerHasMostLife(_)
        | TriggerConditionDef::ControllerLifeAtMostHalfStartingLife => Ok(()),
        TriggerConditionDef::ValueComparison(values) => {
            validate_value_target_references(values.left, target_count, scope)?;
            validate_value_target_references(values.right, target_count, scope)
        }
    }
}

fn validate_recipient_target_references(
    recipient: EffectRecipientDef,
    target_count: usize,
    scope: BindingScope<'_>,
) -> Result<(), GrantedAbilityValidationError> {
    match recipient.0 {
        EffectRecipientSetDef::LegalTargets(target) => validate_target_index(target, target_count),
        EffectRecipientSetDef::Objects(objects) => {
            validate_object_set_target_references(objects, target_count, scope)
        }
        EffectRecipientSetDef::Players(players)
        | EffectRecipientSetDef::PlayersAndCreaturesTheyControl(players) => {
            validate_player_set(players, target_count, scope)
        }
        // The attacker is named the same way any other object is; what it
        // is attacking is read off the declaration rather than authored.
        EffectRecipientSetDef::DefenderOf(reference) => {
            validate_object_reference(reference, target_count, scope)
        }
    }
}

fn validate_object_set_target_references(
    objects: ObjectSetDef,
    target_count: usize,
    scope: BindingScope<'_>,
) -> Result<(), GrantedAbilityValidationError> {
    match objects {
        ObjectSetDef::One(reference)
        | ObjectSetDef::PermanentsTargetedBy(reference)
        | ObjectSetDef::LegalAttachmentHosts(reference)
        | ObjectSetDef::TokensCreatedBy(reference) => {
            validate_object_reference(reference, target_count, scope)
        }
        ObjectSetDef::Binding(binding)
        | ObjectSetDef::ZoneChangeSuccessorsOfBinding(binding)
        | ObjectSetDef::MatchingBinding { binding, .. } => {
            scope.validate_object_set_reference(binding)
        }
        ObjectSetDef::Matching { objects, object } => {
            validate_object_set_target_references(*objects, target_count, scope)?;
            validate_object_predicate_references(object.predicate(), target_count, scope)
        }
        ObjectSetDef::ExceptObject { objects, object } => {
            validate_object_set_target_references(*objects, target_count, scope)?;
            validate_object_reference(object, target_count, scope)
        }
        ObjectSetDef::LegalTargets(target) => {
            validate_target_index(target, target_count)
        }
        ObjectSetDef::Query(query) => validate_query(query, target_count, scope),
        ObjectSetDef::PlayerAttachments(query) => {
            validate_object_predicate_references(query.object, target_count, scope)
        }
        // The pile is named by which permanent exiled the cards, so there is
        // no player or target reference in it to validate.
        ObjectSetDef::LinkedExiles => Ok(()),
        ObjectSetDef::BottomOfGraveyard(player)
            | ObjectSetDef::CardsDrawnThisTurnInHand(player)
            | ObjectSetDef::PermanentsControlledBy(player)
            | ObjectSetDef::TopOfGraveyardMatching { player, .. } => {
            validate_player_reference(player, target_count, scope)
        }
    }
}

#[allow(
    clippy::too_many_lines,
    reason = "exhaustive value traversal keeps every reference-bearing variant visible"
)]
fn validate_value_target_references(
    value: ValueDef,
    target_count: usize,
    scope: BindingScope<'_>,
) -> Result<(), GrantedAbilityValidationError> {
    match value {
        ValueDef::Negate(value) => validate_value_target_references(*value, target_count, scope),
        ValueDef::Scaled(scaled) => {
            validate_value_target_references(scaled.value, target_count, scope)
        }
        ValueDef::Halved(halved) => {
            validate_value_target_references(halved.value, target_count, scope)
        }
        ValueDef::Quotient(quotient) => {
            validate_value_target_references(quotient.numerator, target_count, scope)?;
            validate_value_target_references(quotient.denominator, target_count, scope)
        }
        ValueDef::Sum(sum) => {
            validate_value_target_references(sum.left, target_count, scope)?;
            validate_value_target_references(sum.right, target_count, scope)
        }
        ValueDef::IfAdditionalCostPaid(condition) => validate_value_pair_target_references(
            condition.if_paid,
            condition.otherwise,
            target_count,
            scope,
        ),
        ValueDef::IfControllerLifeAtMost(condition) => validate_value_pair_target_references(
            condition.then,
            condition.otherwise,
            target_count,
            scope,
        ),
        ValueDef::IfCreatureDiedThisTurn(condition) => validate_value_pair_target_references(
            condition.then,
            condition.otherwise,
            target_count,
            scope,
        ),
        ValueDef::IfCondition(condition) => {
            validate_trigger_condition(*condition.condition, target_count, scope)?;
            validate_value_pair_target_references(
                condition.then,
                condition.otherwise,
                target_count,
                scope,
            )
        }
        ValueDef::IfSourceMatches(condition) => {
            validate_object_predicate_references(condition.object, target_count, scope)?;
            validate_value_pair_target_references(
                condition.then,
                condition.otherwise,
                target_count,
                scope,
            )
        }
        ValueDef::IfTargetMatches(condition) => {
            validate_target_index(condition.slot, target_count)?;
            validate_value_pair_target_references(
                condition.then,
                condition.otherwise,
                target_count,
                scope,
            )
        }
        ValueDef::IfMatchingObjectCount(condition) => {
            validate_query(condition.query, target_count, scope)?;
            validate_value_pair_target_references(
                condition.then,
                condition.otherwise,
                target_count,
                scope,
            )
        }
        ValueDef::AggregateObjectValues(aggregate) => {
            validate_object_set_target_references(aggregate.objects, target_count, scope)
        }
        ValueDef::AggregatePlayerObjectCounts(aggregate) => {
            validate_player_set(aggregate.players, target_count, scope)?;
            validate_query(aggregate.query, target_count, scope)
        }
        ValueDef::CountMatchingObjects(query)
        | ValueDef::AnyMatchingObject(query)
        | ValueDef::DistinctNamesAmong(query) => {
            validate_query(*query, target_count, scope)
        }
        ValueDef::CountMatchingPlayerAttachments(query) => {
            validate_object_predicate_references(query.object, target_count, scope)
        }
        ValueDef::CountObjects(objects) | ValueDef::CardTypesAmongObjects(objects) => {
            validate_object_set_target_references(*objects, target_count, scope)
        }
        ValueDef::TargetPower(target)
        | ValueDef::TargetToughness(target)
        | ValueDef::TargetLibrarySize(target)
        | ValueDef::TargetManaValue(target) => validate_target_index(target, target_count),
        // Whatever the amount reads has to be nameable where it is read, the
        // same as any other object reference in the program.
        ValueDef::ObjectPower(reference) | ValueDef::ObjectManaValue(reference) => {
            validate_object_reference(reference, target_count, scope)
        }
        ValueDef::CountersOnObject(counted) => {
            validate_object_reference(counted.object, target_count, scope)
        }
        ValueDef::BoundObjectCount(binding) => scope.validate_object_set_reference(binding),
        ValueDef::CountSpellsCastThisTurn(_)
        | ValueDef::Constant(_)
        | ValueDef::ChosenX
        | ValueDef::PlayerCounters { .. }
        | ValueDef::SacrificedManaValue
        | ValueDef::SourceCastX
        | ValueDef::SourcePower
        | ValueDef::AffectedManaValue
        | ValueDef::AffectedColorCount
        | ValueDef::TriggeringObjectPower
        | ValueDef::TriggeringObjectToughness
        | ValueDef::LifeTotal(_)
        | ValueDef::StartingLifeTotal
        | ValueDef::SourceToughness
        | ValueDef::TriggerEventAmount
        | ValueDef::DamageEventAmount
        | ValueDef::CardsInHandAbove { .. }
        | ValueDef::DamageTakenThisTurn { .. }
        | ValueDef::CountersOnSource(_)
        | ValueDef::CardsDrawnThisTurn(_)
        | ValueDef::LandsPlayedThisTurn(_)
        | ValueDef::LifeGainedThisTurn(_)
        | ValueDef::DevotionTo(_)
        | ValueDef::BasicLandTypesControlled(_)
        | ValueDef::LibrarySize(_)
        | ValueDef::SpellsCastThisGame(_)
        | ValueDef::ColorsOfManaSpent
        | ValueDef::PaidAmount
        | ValueDef::MatchedCount
        | ValueDef::MatchedCardTypes
        | ValueDef::MatchedManaValue
        | ValueDef::SpellsCastBeforeThisTurn
        | ValueDef::AdditionalCostPayments(_)
        | ValueDef::CreaturesDiedThisTurn
        | ValueDef::OpponentsWhoLostLifeThisTurn
        | ValueDef::CardTypesAmongGraveyards(_)
        | ValueDef::IfCardTypesAmongGraveyards(_)
        // This reads the share assigned to the target currently being
        // affected; the surrounding recipient carries the slot reference.
        | ValueDef::DistinctTargets
        | ValueDef::DividedAmongTargets
        | ValueDef::ResolvedRecipientCount => Ok(()),
    }
}

fn validate_value_pair_target_references(
    left: ValueDef,
    right: ValueDef,
    target_count: usize,
    scope: BindingScope<'_>,
) -> Result<(), GrantedAbilityValidationError> {
    validate_value_target_references(left, target_count, scope)?;
    validate_value_target_references(right, target_count, scope)
}

fn validate_applied_effect_target_references(
    effect: AppliedEffectDef,
    target_count: usize,
    scope: BindingScope<'_>,
) -> Result<(), GrantedAbilityValidationError> {
    match effect {
        AppliedEffectDef::Composite(effects) => {
            for effect in effects {
                validate_applied_effect_target_references(*effect, target_count, scope)?;
            }
            Ok(())
        }
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
            PowerToughnessOperationDef::SetBase { power, toughness }
            | PowerToughnessOperationDef::Modify { power, toughness },
        )) => {
            validate_value_target_references(power, target_count, scope)?;
            validate_value_target_references(toughness, target_count, scope)
        }
        AppliedEffectDef::Rule(AppliedRuleDef::PreventDamage(matcher)) => {
            validate_damage_matcher_references(matcher, target_count, scope)
        }
        AppliedEffectDef::Rule(AppliedRuleDef::AttackRestriction(restriction)) => {
            validate_object_predicate_references(
                restriction.attacker,
                target_count,
                scope,
            )
        }
        AppliedEffectDef::Rule(AppliedRuleDef::BlockRestriction(restriction)) => {
            match restriction {
                BlockRestrictionDef::Pair { counterpart, .. } => match counterpart {
                    BlockRestrictionMatchDef::Any => Ok(()),
                    BlockRestrictionMatchDef::Matching(predicate)
                    | BlockRestrictionMatchDef::Except(predicate) => {
                        validate_object_predicate_references(predicate, target_count, scope)
                    }
                },
                BlockRestrictionDef::MinimumBlockers(_) => Ok(()),
            }
        }
        AppliedEffectDef::Rule(AppliedRuleDef::RedirectDamageFromTo {
            source,
            destination,
        }) => {
            validate_object_reference(source, target_count, scope)?;
            validate_object_reference(destination, target_count, scope)
        }
        // A granted ability introduces its own target scope and is validated
        // separately when the grant tree is traversed.
        AppliedEffectDef::Rule(_) | AppliedEffectDef::Characteristic(_) => Ok(()),
    }
}

fn validate_object_predicate_references(
    predicate: ObjectPredicateDef,
    target_count: usize,
    scope: BindingScope<'_>,
) -> Result<(), GrantedAbilityValidationError> {
    match predicate {
        ObjectPredicateDef::All(predicates) | ObjectPredicateDef::AnyOf(predicates) => {
            for predicate in predicates {
                validate_object_predicate_references(*predicate, target_count, scope)?;
            }
            Ok(())
        }
        ObjectPredicateDef::Not(predicate) | ObjectPredicateDef::AttachedTo(predicate) => {
            validate_object_predicate_references(*predicate, target_count, scope)
        }
        ObjectPredicateDef::ManaValueEqualTo(value)
        | ObjectPredicateDef::ManaValueAtMostValue(value)
        | ObjectPredicateDef::ToughnessLessThan(value)
        | ObjectPredicateDef::PowerGreaterThan(value)
        | ObjectPredicateDef::ToughnessGreaterThan(value)
        | ObjectPredicateDef::PowerLessThan(value) => {
            validate_value_target_references(value, target_count, scope)
        }
        ObjectPredicateDef::NameEquals(name) => {
            validate_card_name_references(name, target_count, scope)
        }
        ObjectPredicateDef::NameIn(names) => {
            validate_card_name_set_references(names, target_count, scope)
        }
        _ => Ok(()),
    }
}

fn validate_card_name_references(
    name: CardNameDef,
    target_count: usize,
    scope: BindingScope<'_>,
) -> Result<(), GrantedAbilityValidationError> {
    match name {
        CardNameDef::Object(reference) => validate_object_reference(reference, target_count, scope),
        CardNameDef::EffectChoice => {
            scope.mark_chosen_name_read();
            Ok(())
        }
        CardNameDef::Literal(_) | CardNameDef::SourceChoice => Ok(()),
    }
}

fn validate_card_name_set_references(
    names: CardNameSetDef,
    target_count: usize,
    scope: BindingScope<'_>,
) -> Result<(), GrantedAbilityValidationError> {
    match names {
        CardNameSetDef::NamesOf(objects)
        | CardNameSetDef::NamesAppearingAtLeast { objects, .. } => {
            validate_object_set_target_references(*objects, target_count, scope)
        }
        CardNameSetDef::BasicLandNames => Ok(()),
    }
}
