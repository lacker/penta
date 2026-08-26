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
    validate_effect_references(effect, targets.len(), BindingScope::EMPTY)?;
    validate_effect_target_shapes(effect, targets, None)
}

#[cfg(test)]
pub(in crate::card::catalog) fn validate_replacement_ability_targets(
    targets: &[AbilityTargetDef],
    effect: ReplacementEffectDef,
) -> Result<(), GrantedAbilityValidationError> {
    validate_target_definitions(targets)?;
    validate_replacement_effect_target_references(effect, targets.len(), BindingScope::EMPTY)?;
    validate_replacement_effect_target_shapes(effect, targets)
}

pub(super) fn validate_ability_program_targets(
    targets: &[AbilityTargetDef],
    program: AbilityProgramDef,
    trigger_event: Option<TriggerEventDef>,
) -> Result<(), GrantedAbilityValidationError> {
    validate_target_definitions(targets)?;
    validate_program_references(program, targets.len(), BindingScope::EMPTY)?;
    validate_program_target_shapes(program, targets, trigger_event)
}

pub(super) fn validate_ability_trigger_event(
    event: TriggerEventDef,
    target_count: usize,
) -> Result<(), GrantedAbilityValidationError> {
    validate_trigger_event_references(event, target_count, BindingScope::EMPTY)
}

fn validate_program_references(
    program: AbilityProgramDef,
    target_count: usize,
    scope: BindingScope,
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
    }
    Ok(())
}

#[derive(Clone, Copy)]
struct BindingScope {
    objects: u8,
    object_sets: u8,
}

impl BindingScope {
    const EMPTY: Self = Self {
        objects: 0,
        object_sets: 0,
    };

    fn with_object(
        self,
        binding: ObjectBindingIndex,
    ) -> Result<Self, GrantedAbilityValidationError> {
        let bit = 1 << binding.index();
        if self.objects & bit != 0 {
            Err(GrantedAbilityValidationError::ObjectBindingAlreadyInScope { binding })
        } else {
            Ok(Self {
                objects: self.objects | bit,
                ..self
            })
        }
    }

    fn with_object_set(
        self,
        binding: ObjectSetBindingIndex,
    ) -> Result<Self, GrantedAbilityValidationError> {
        let bit = 1 << binding.index();
        if self.object_sets & bit != 0 {
            Err(GrantedAbilityValidationError::ObjectSetBindingAlreadyInScope { binding })
        } else {
            Ok(Self {
                object_sets: self.object_sets | bit,
                ..self
            })
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

fn validate_object_reference(
    reference: ObjectRefDef,
    target_count: usize,
    scope: BindingScope,
) -> Result<(), GrantedAbilityValidationError> {
    match reference {
        ObjectRefDef::Target(target) | ObjectRefDef::SourceOfTargetedStackObject(target) => {
            validate_target_index(target, target_count)
        }
        ObjectRefDef::Binding(binding) => {
            if scope.objects & (1 << binding.index()) != 0 {
                Ok(())
            } else {
                Err(GrantedAbilityValidationError::ObjectBindingReferenceOutOfScope { binding })
            }
        }
        ObjectRefDef::Source
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
    scope: BindingScope,
) -> Result<(), GrantedAbilityValidationError> {
    match reference {
        PlayerRefDef::Target(target) => validate_target_index(target, target_count),
        PlayerRefDef::ControllerOf(reference) | PlayerRefDef::OwnerOf(reference) => {
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
    scope: BindingScope,
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
    scope: BindingScope,
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
    scope: BindingScope,
) -> Result<(), GrantedAbilityValidationError> {
    match players {
        PlayerSetDef::One(reference) => validate_player_reference(reference, target_count, scope),
        PlayerSetDef::LegalTargets(target) => validate_target_index(target, target_count),
        PlayerSetDef::All | PlayerSetDef::Related(_) => Ok(()),
    }
}

fn validate_pile_role(
    role: &'static str,
    players: PlayerSetDef,
) -> Result<(), GrantedAbilityValidationError> {
    if matches!(
        players,
        PlayerSetDef::All | PlayerSetDef::Related(PlayerRelation::Any)
    ) {
        Err(GrantedAbilityValidationError::InvalidPileRole { role, players })
    } else {
        Ok(())
    }
}

fn validate_query(
    query: ObjectQueryDef,
    target_count: usize,
    scope: BindingScope,
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
    scope: BindingScope,
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
    scope: BindingScope,
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
        | TriggerConditionDef::BoundObjectsShareName { .. }
        | TriggerConditionDef::SourceArrivedSinceControllersLastUpkeep
        | TriggerConditionDef::SourceOnBattlefield
        | TriggerConditionDef::SourceInZone(_)
        | TriggerConditionDef::SourceUntapped
        | TriggerConditionDef::SourceIsPaired
        | TriggerConditionDef::ActivePlayer(_)
        | TriggerConditionDef::SpellsCastThisTurn { .. }
        | TriggerConditionDef::SpellsCastLastTurn { .. }
        | TriggerConditionDef::ControlsGreatestPowerCreature
        | TriggerConditionDef::SourceMatches { .. }
        | TriggerConditionDef::LinkedExilesMatch { .. }
        | TriggerConditionDef::AttachedPermanentMatches { .. }
        | TriggerConditionDef::SourceCounters { .. }
        | TriggerConditionDef::SourceCastWith(_)
        | TriggerConditionDef::SourceCastFrom(_)
        | TriggerConditionDef::SourceWasCast
        | TriggerConditionDef::SourceCastAtInstantSpeed
        | TriggerConditionDef::SourceLoyalty { .. }
        | TriggerConditionDef::SourceActivationsThisTurn { .. }
        | TriggerConditionDef::SourceResolutionsThisTurn { .. }
        | TriggerConditionDef::SourceDealtDamageToOpponentThisTurn
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
    scope: BindingScope,
) -> Result<(), GrantedAbilityValidationError> {
    match recipient.0 {
        EffectRecipientSetDef::LegalTargets(target) => validate_target_index(target, target_count),
        EffectRecipientSetDef::Objects(
            ObjectSetDef::One(reference)
            | ObjectSetDef::PermanentsTargetedBy(reference)
            | ObjectSetDef::SharingNameWith(reference),
        ) => validate_object_reference(reference, target_count, scope),
        EffectRecipientSetDef::Objects(
            ObjectSetDef::Binding(binding) | ObjectSetDef::MatchingBinding { binding, .. },
        ) => {
            if scope.object_sets & (1 << binding.index()) != 0 {
                Ok(())
            } else {
                Err(GrantedAbilityValidationError::ObjectSetBindingReferenceOutOfScope { binding })
            }
        }
        EffectRecipientSetDef::Objects(ObjectSetDef::LegalTargets(target)) => {
            validate_target_index(target, target_count)
        }
        EffectRecipientSetDef::Objects(ObjectSetDef::Query(query)) => {
            validate_query(query, target_count, scope)
        }
        // The pile is named by which permanent exiled the cards, so there is
        // no player or target reference in it to validate.
        EffectRecipientSetDef::Objects(ObjectSetDef::LinkedExiles(_)) => Ok(()),
        EffectRecipientSetDef::Objects(
            ObjectSetDef::BottomOfGraveyard(player)
            | ObjectSetDef::CardsDrawnThisTurnInHand(player)
            | ObjectSetDef::SharingNameWithBinding { player, .. }
            | ObjectSetDef::TopOfGraveyardMatching { player, .. },
        ) => validate_player_reference(player, target_count, scope),
        EffectRecipientSetDef::Players(players)
        | EffectRecipientSetDef::PlayersAndCreaturesTheyControl(players) => {
            validate_player_set(players, target_count, scope)
        }
    }
}

fn validate_value_target_references(
    value: ValueDef,
    target_count: usize,
    scope: BindingScope,
) -> Result<(), GrantedAbilityValidationError> {
    match value {
        ValueDef::Negate(value) => {
            validate_value_target_references(*value, target_count, scope)
        }
        ValueDef::Scaled(scaled) => {
            validate_value_target_references(scaled.value, target_count, scope)
        }
        ValueDef::Halved(halved) => {
            validate_value_target_references(halved.value, target_count, scope)
        }
        ValueDef::Sum(sum) => {
            validate_value_target_references(sum.left, target_count, scope)?;
            validate_value_target_references(sum.right, target_count, scope)
        }
        ValueDef::IfControllerLifeAtMost(condition) => {
            validate_value_target_references(condition.then, target_count, scope)?;
            validate_value_target_references(condition.otherwise, target_count, scope)
        }
        ValueDef::IfCreatureDiedThisTurn(condition) => {
            validate_value_target_references(condition.then, target_count, scope)?;
            validate_value_target_references(condition.otherwise, target_count, scope)
        }
        ValueDef::IfTargetMatches(condition) => {
            validate_target_index(condition.slot, target_count)?;
            validate_value_target_references(condition.then, target_count, scope)?;
            validate_value_target_references(condition.otherwise, target_count, scope)
        }
        ValueDef::IfMatchingObjectCount(condition) => {
            validate_query(condition.query, target_count, scope)?;
            validate_value_target_references(condition.then, target_count, scope)?;
            validate_value_target_references(condition.otherwise, target_count, scope)
        }
        ValueDef::CountMatchingObjects(query)
        | ValueDef::AnyMatchingObject(query)
        | ValueDef::DistinctNamesAmong(query)
        | ValueDef::GreatestPowerAmong(query) => {
            validate_query(*query, target_count, scope)
        }
        ValueDef::CountMatchingPlayerAttachments(query) => {
            validate_object_predicate_references(query.object, target_count, scope)
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
        ValueDef::CountSpellsCastThisTurn(_)
        | ValueDef::Constant(_)
        | ValueDef::ChosenX
        | ValueDef::PlayerCounters { .. }
        | ValueDef::SacrificedManaValue
        | ValueDef::SourceCastX
        | ValueDef::SourcePower
        | ValueDef::AffectedManaValue
        | ValueDef::AffectedColorCount
        | ValueDef::TotalPowerOfLinkedExiles
        | ValueDef::TotalToughnessOfLinkedExiles
        | ValueDef::TriggeringObjectPower
        | ValueDef::TriggeringObjectToughness
        | ValueDef::LifeTotal(_)
        | ValueDef::SourceToughness
        | ValueDef::TriggerEventAmount
        | ValueDef::CardsInHandAbove { .. }
        | ValueDef::DamageTakenThisTurn { .. }
        | ValueDef::CountersOnSource(_)
        | ValueDef::CardsDrawnThisTurn(_)
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
        | ValueDef::BoundObjectCount(_)
        | ValueDef::SpellsCastBeforeThisTurn
        | ValueDef::TimesAdditionalCostPaid
        | ValueDef::CreaturesDiedThisTurn
        | ValueDef::OpponentsWhoLostLifeThisTurn
        | ValueDef::CardTypesAmongGraveyards(_)
        | ValueDef::IfCardTypesAmongGraveyards(_)
        // This reads the share assigned to the target currently being
        // affected; the surrounding recipient carries the slot reference.
        | ValueDef::DistinctTargets
        | ValueDef::DividedAmongTargets => Ok(()),
    }
}

fn validate_applied_effect_target_references(
    effect: AppliedEffectDef,
    target_count: usize,
    scope: BindingScope,
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
            match restriction.counterpart {
                BlockRestrictionMatchDef::Any => Ok(()),
                BlockRestrictionMatchDef::Matching(predicate)
                | BlockRestrictionMatchDef::Except(predicate) => {
                    validate_object_predicate_references(predicate, target_count, scope)
                }
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
    scope: BindingScope,
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
        _ => Ok(()),
    }
}

fn validate_resolving_applied_effect(
    recipient: EffectRecipientDef,
    effect: AppliedEffectDef,
) -> Result<(), GrantedAbilityValidationError> {
    match effect {
        AppliedEffectDef::Composite(effects) => {
            if effects.is_empty() {
                return Err(GrantedAbilityValidationError::UnsupportedResolvingAppliedEffect);
            }
            for effect in effects {
                validate_resolving_applied_effect(recipient, *effect)?;
            }
            Ok(())
        }
        AppliedEffectDef::Rule(
            AppliedRuleDef::CannotPlay(_)
            | AppliedRuleDef::MayPlayFromGraveyard(_)
            // A timing permission is aimed at a player the same way a
            // prohibition is: no object has a casting window of its own.
            | AppliedRuleDef::MayCastAsThoughItHadFlash(_)
            // A player's protection is likewise a rule about the player: an
            // object gets protection as a keyword instead.
            | AppliedRuleDef::PlayerProtectionFrom(_)
            | AppliedRuleDef::RedirectDamageFromTo { .. },
        ) => {
            if matches!(recipient.0, EffectRecipientSetDef::Objects(_)) {
                Err(GrantedAbilityValidationError::UnsupportedResolvingAppliedEffect)
            } else {
                Ok(())
            }
        }
        AppliedEffectDef::Rule(AppliedRuleDef::AttackRestriction(restriction)) => {
            if restriction
                .cost
                .is_some_and(|cost| cost.variable_x || cost.x_multiplier != 0)
            {
                return Err(GrantedAbilityValidationError::UnsupportedResolvingAppliedEffect);
            }
            let object_recipient = matches!(recipient.0, EffectRecipientSetDef::Objects(_));
            match restriction.defender {
                AttackDefenderScopeDef::Any if object_recipient => Ok(()),
                AttackDefenderScopeDef::AffectedPlayer
                | AttackDefenderScopeDef::AffectedPlayerOrPlaneswalker
                    if !object_recipient =>
                {
                    Ok(())
                }
                _ => Err(GrantedAbilityValidationError::UnsupportedResolvingAppliedEffect),
            }
        }
        AppliedEffectDef::Rule(AppliedRuleDef::BlockRestriction(restriction)) => {
            let object_recipient = matches!(
                recipient.0,
                EffectRecipientSetDef::Objects(_) | EffectRecipientSetDef::LegalTargets(_)
            );
            if restriction
                .cost
                .is_some_and(|cost| cost.variable_x || cost.x_multiplier != 0)
                || !object_recipient
            {
                Err(GrantedAbilityValidationError::UnsupportedResolvingAppliedEffect)
            } else {
                Ok(())
            }
        }
        AppliedEffectDef::Rule(
            AppliedRuleDef::CannotBeCountered | AppliedRuleDef::PreventDamage(_),
        ) => Err(GrantedAbilityValidationError::UnsupportedResolvingAppliedEffect),
        AppliedEffectDef::Rule(_) | AppliedEffectDef::Characteristic(_) => {
            if matches!(recipient.0, EffectRecipientSetDef::Players(_)) {
                Err(GrantedAbilityValidationError::UnsupportedResolvingAppliedEffect)
            } else {
                Ok(())
            }
        }
    }
}
