fn validate_program_target_shapes(
    program: AbilityProgramDef,
    targets: &[AbilityTargetDef],
    trigger_event: Option<TriggerEventDef>,
) -> Result<(), GrantedAbilityValidationError> {
    let triggering_object_zone = trigger_event.and_then(trigger_event_object_zone);
    match program {
        AbilityProgramDef::Effects(effect) => {
            validate_effect_target_shapes(effect, targets, triggering_object_zone)
        }
        AbilityProgramDef::Replacement(effect) => {
            validate_replacement_effect_target_shapes(effect, targets)
        }
    }
}

fn target_matches_expectation(
    predicate: AbilityTargetPredicate,
    expected: RecipientExpectation,
) -> bool {
    if let AbilityTargetPredicate::IfAdditionalCostPaid {
        if_paid, otherwise, ..
    } = predicate
    {
        return target_matches_expectation(*if_paid, expected)
            && target_matches_expectation(*otherwise, expected);
    }
    if let AbilityTargetPredicate::AnyOf(predicates) = predicate {
        return !predicates.is_empty()
            && predicates
                .iter()
                .copied()
                .all(|predicate| target_matches_expectation(predicate, expected));
    }
    match expected {
        RecipientExpectation::Any => true,
        RecipientExpectation::Object => matches!(
            predicate,
            AbilityTargetPredicate::Object { .. }
                | AbilityTargetPredicate::ControlledByTargetOf { .. }
                | AbilityTargetPredicate::OwnedByTargetPlayer { .. }
        ),
        RecipientExpectation::Player => matches!(predicate, AbilityTargetPredicate::Player(_)),
    }
}

fn target_can_project(predicate: AbilityTargetPredicate, expected: RecipientExpectation) -> bool {
    if let AbilityTargetPredicate::IfAdditionalCostPaid {
        if_paid, otherwise, ..
    } = predicate
    {
        return target_can_project(*if_paid, expected)
            && target_can_project(*otherwise, expected);
    }
    if let AbilityTargetPredicate::AnyOf(predicates) = predicate {
        return !predicates.is_empty()
            && predicates
                .iter()
                .copied()
                .all(|predicate| target_can_project(predicate, expected));
    }
    match expected {
        RecipientExpectation::Any => true,
        RecipientExpectation::Object => !matches!(predicate, AbilityTargetPredicate::Player(_)),
        RecipientExpectation::Player => !matches!(
            predicate,
            AbilityTargetPredicate::Object { .. }
                | AbilityTargetPredicate::ControlledByTargetOf { .. }
                | AbilityTargetPredicate::OwnedByTargetPlayer { .. }
        ),
    }
}

fn validate_target_projection(
    target: TargetIndex,
    targets: &[AbilityTargetDef],
    expected: RecipientExpectation,
) -> Result<(), GrantedAbilityValidationError> {
    let Some(definition) = targets.get(target.index()) else {
        return Err(GrantedAbilityValidationError::TargetReferenceOutOfBounds {
            target,
            target_count: targets.len(),
        });
    };
    if target_can_project(definition.predicate, expected) {
        Ok(())
    } else {
        Err(GrantedAbilityValidationError::TargetReferenceKindMismatch {
            target,
            predicate: definition.predicate,
            expected: public_subject_kind(expected),
        })
    }
}

fn public_subject_kind(expected: RecipientExpectation) -> EffectSubjectKind {
    match expected {
        RecipientExpectation::Object => EffectSubjectKind::Object,
        RecipientExpectation::Player => EffectSubjectKind::Player,
        RecipientExpectation::Any => unreachable!("an any-target expectation never errors"),
    }
}

fn validate_target_shape(
    target: TargetIndex,
    targets: &[AbilityTargetDef],
    expected: RecipientExpectation,
    singular: bool,
) -> Result<(), GrantedAbilityValidationError> {
    let Some(definition) = targets.get(target.index()) else {
        return Err(GrantedAbilityValidationError::TargetReferenceOutOfBounds {
            target,
            target_count: targets.len(),
        });
    };
    if singular && definition.maximum > 1 {
        return Err(
            GrantedAbilityValidationError::TargetReferenceRequiresSingular {
                target,
                maximum: definition.maximum,
            },
        );
    }
    if !target_matches_expectation(definition.predicate, expected) {
        return Err(GrantedAbilityValidationError::TargetReferenceKindMismatch {
            target,
            predicate: definition.predicate,
            expected: public_subject_kind(expected),
        });
    }
    Ok(())
}

fn validate_object_reference_shape(
    reference: ObjectRefDef,
    targets: &[AbilityTargetDef],
) -> Result<(), GrantedAbilityValidationError> {
    if let ObjectRefDef::Target(target) = reference {
        validate_target_shape(target, targets, RecipientExpectation::Object, true)?;
    }
    Ok(())
}

fn validate_player_reference_shape(
    reference: PlayerRefDef,
    targets: &[AbilityTargetDef],
) -> Result<(), GrantedAbilityValidationError> {
    match reference {
        PlayerRefDef::Target(target) => {
            validate_target_shape(target, targets, RecipientExpectation::Player, true)
        }
        // The runtime can derive a controller from both halves of an any-target
        // slot: a player is their own controller, while an object has one.
        // "Each player other than its controller" reads a controller the
        // same way, so it admits the same targets.
        PlayerRefDef::ControllerOf(ObjectRefDef::Target(target))
        | PlayerRefDef::OpponentOf(ObjectRefDef::Target(target)) => {
            validate_target_shape(target, targets, RecipientExpectation::Any, true)
        }
        // Players have no owner, so this derived reference requires an
        // object-only target even though ControllerOf can also consume a
        // player directly. Merely projecting an any-target slot is not enough:
        // the selected member could still be a player and silently produce no
        // owner at resolution.
        PlayerRefDef::OwnerOf(ObjectRefDef::Target(target)) => {
            validate_target_shape(target, targets, RecipientExpectation::Object, true)
        }
        PlayerRefDef::ControllerOf(reference)
        | PlayerRefDef::OpponentOf(reference)
        | PlayerRefDef::OwnerOf(reference) => {
            validate_object_reference_shape(reference, targets)
        }
        PlayerRefDef::EffectController
        | PlayerRefDef::EnchantedPlayer
        | PlayerRefDef::EventPlayer
        | PlayerRefDef::Opponent => Ok(()),
    }
}

fn validate_player_set_shape(
    players: PlayerSetDef,
    targets: &[AbilityTargetDef],
) -> Result<(), GrantedAbilityValidationError> {
    match players {
        PlayerSetDef::One(reference) => validate_player_reference_shape(reference, targets)?,
        PlayerSetDef::LegalTargets(target) => {
            validate_target_projection(target, targets, RecipientExpectation::Player)?;
        }
        PlayerSetDef::All | PlayerSetDef::Related(_) => {}
    }
    Ok(())
}

fn validate_query_shape(
    query: ObjectQueryDef,
    targets: &[AbilityTargetDef],
) -> Result<(), GrantedAbilityValidationError> {
    for players in [query.controller, query.owner, query.related_player]
        .into_iter()
        .flatten()
    {
        validate_player_set_shape(players, targets)?;
    }
    if let Some(relative) = query.relative_position {
        let reference = match relative {
            ZoneRelativePositionDef::Above(reference)
            | ZoneRelativePositionDef::Below(reference) => reference,
        };
        validate_object_reference_shape(reference, targets)?;
    }
    validate_object_predicate_shape(query.object, targets)
}

fn validate_object_set_shape(
    objects: ObjectSetDef,
    targets: &[AbilityTargetDef],
) -> Result<(), GrantedAbilityValidationError> {
    match objects {
        ObjectSetDef::Union(sets) => sets
            .iter()
            .copied()
            .try_for_each(|objects| validate_object_set_shape(objects, targets)),
        ObjectSetDef::One(reference)
        | ObjectSetDef::PermanentsTargetedBy(reference)
        | ObjectSetDef::LegalAttachmentHosts(reference)
        | ObjectSetDef::TokensCreatedBy(reference) => {
            validate_object_reference_shape(reference, targets)
        }
        ObjectSetDef::Query(query) => validate_query_shape(query, targets),
        ObjectSetDef::Matching { objects, object } => {
            validate_object_set_shape(*objects, targets)?;
            validate_object_predicate_shape(object.predicate(), targets)
        }
        ObjectSetDef::ExceptObject { objects, object } => {
            validate_object_set_shape(*objects, targets)?;
            validate_object_reference_shape(object, targets)
        }
        ObjectSetDef::CardsDrawnThisTurnInHand(player)
        | ObjectSetDef::PermanentsControlledBy(player) => {
            validate_player_reference_shape(player, targets)
        }
        ObjectSetDef::PlayerAttachments(query) => {
            validate_object_predicate_shape(query.object, targets)
        }
        ObjectSetDef::LegalTargets(target) => {
            validate_target_projection(target, targets, RecipientExpectation::Object)
        }
        ObjectSetDef::Binding(_)

        | ObjectSetDef::ZoneChangeSuccessorsOfBinding(_)
        | ObjectSetDef::MatchingBinding { .. }
        | ObjectSetDef::LinkedExiles
        | ObjectSetDef::BottomOfGraveyard(_)
        | ObjectSetDef::TopOfGraveyardMatching { .. } => Ok(()),
    }
}

fn validate_recipient_shape(
    recipient: EffectRecipientDef,
    targets: &[AbilityTargetDef],
    expected: RecipientExpectation,
) -> Result<(), GrantedAbilityValidationError> {
    match recipient.0 {
        EffectRecipientSetDef::LegalTargets(target) => {
            validate_target_shape(target, targets, expected, false)
        }
        EffectRecipientSetDef::Objects(objects) => {
            if matches!(expected, RecipientExpectation::Player) {
                return Err(GrantedAbilityValidationError::EffectRecipientKindMismatch {
                    recipient,
                    expected: EffectSubjectKind::Player,
                });
            }
            validate_object_set_shape(objects, targets)
        }
        // Naming both kinds at once is neither an object recipient nor a
        // player one, so a clause expecting either alone refuses it.
        EffectRecipientSetDef::PlayersAndCreaturesTheyControl(players) => {
            if !matches!(expected, RecipientExpectation::Any) {
                return Err(GrantedAbilityValidationError::EffectRecipientKindMismatch {
                    recipient,
                    expected: EffectSubjectKind::Player,
                });
            }
            validate_player_set_shape(players, targets)
        }
        EffectRecipientSetDef::Players(players) => {
            if matches!(expected, RecipientExpectation::Object) {
                return Err(GrantedAbilityValidationError::EffectRecipientKindMismatch {
                    recipient,
                    expected: EffectSubjectKind::Object,
                });
            }
            validate_player_set_shape(players, targets)
        }
        // "The player or planeswalker it's attacking" is a player or an
        // object depending on the declaration, so only a clause that
        // accepts either may name it.
        EffectRecipientSetDef::DefenderOf(_) => {
            if matches!(expected, RecipientExpectation::Any) {
                Ok(())
            } else {
                Err(GrantedAbilityValidationError::EffectRecipientKindMismatch {
                    recipient,
                    expected: EffectSubjectKind::Player,
                })
            }
        }
    }
}

#[allow(clippy::too_many_lines)]
fn validate_value_shape(
    value: ValueDef,
    targets: &[AbilityTargetDef],
) -> Result<(), GrantedAbilityValidationError> {
    match value {
        ValueDef::Negate(value) => validate_value_shape(*value, targets),
        ValueDef::Scaled(value) => validate_value_shape(value.value, targets),
        ValueDef::Halved(value) => validate_value_shape(value.value, targets),
        ValueDef::Quotient(value) => {
            validate_value_pair_shape(value.numerator, value.denominator, targets)
        }
        ValueDef::Sum(value) => validate_value_pair_shape(value.left, value.right, targets),
        ValueDef::IfAdditionalCostPaid(value) => {
            validate_value_pair_shape(value.if_paid, value.otherwise, targets)
        }
        ValueDef::IfControllerLifeAtMost(value) => {
            validate_value_pair_shape(value.then, value.otherwise, targets)
        }
        ValueDef::IfCondition(value) => {
            validate_trigger_condition_shape(*value.condition, targets)?;
            validate_value_pair_shape(value.then, value.otherwise, targets)
        }
        ValueDef::IfCreatureDiedThisTurn(value) => {
            validate_value_pair_shape(value.then, value.otherwise, targets)
        }
        ValueDef::IfSourceMatches(value) => {
            validate_object_predicate_shape(value.object, targets)?;
            validate_value_pair_shape(value.then, value.otherwise, targets)
        }
        ValueDef::IfTargetMatches(value) => {
            validate_target_shape(value.slot, targets, RecipientExpectation::Object, false)?;
            validate_value_pair_shape(value.then, value.otherwise, targets)
        }
        ValueDef::IfMatchingObjectCount(value) => {
            validate_query_shape(value.query, targets)?;
            validate_value_pair_shape(value.then, value.otherwise, targets)
        }
        ValueDef::AggregateObjectValues(a) => validate_aggregate_shape(a.objects, targets),
        ValueDef::AggregatePlayerObjectCounts(aggregate) => {
            validate_player_set_shape(aggregate.players, targets)?;
            validate_query_shape(aggregate.query, targets)
        }
        ValueDef::CountMatchingObjects(query)
        | ValueDef::AnyMatchingObject(query)
        | ValueDef::DistinctNamesAmong(query) => validate_query_shape(*query, targets),
        ValueDef::CountMatchingPlayerAttachments(query) => {
            validate_object_predicate_shape(query.object, targets)
        }
        ValueDef::CountObjects(objects) => validate_object_set_shape(*objects, targets),
        ValueDef::TargetLibrarySize(target) => {
            validate_target_shape(target, targets, RecipientExpectation::Player, true)
        }
        ValueDef::TargetPower(target)
        | ValueDef::TargetToughness(target)
        | ValueDef::TargetManaValue(target) => {
            validate_target_shape(target, targets, RecipientExpectation::Object, true)
        }
        ValueDef::ObjectPower(reference) | ValueDef::ObjectManaValue(reference) => {
            validate_object_reference_shape(reference, targets)
        }
        ValueDef::CountersOnObject(counted) => {
            validate_object_reference_shape(counted.object, targets)
        }
        ValueDef::CountSpellsCastThisTurn(_)
        | ValueDef::CreaturesDiedThisTurn
        | ValueDef::OpponentsWhoLostLifeThisTurn
        | ValueDef::CardTypesAmongGraveyards(_)
        | ValueDef::IfCardTypesAmongGraveyards(_)
        | ValueDef::LifeTotal(_)
        | ValueDef::StartingLifeTotal
        | ValueDef::Constant(_)
        | ValueDef::ChosenX
        | ValueDef::SourceCastX
        | ValueDef::SourcePower
        | ValueDef::AffectedManaValue
        | ValueDef::AffectedColorCount
        | ValueDef::SourceToughness
        | ValueDef::TriggeringObjectPower
        | ValueDef::TriggeringObjectToughness
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
        | ValueDef::BoundObjectCount(_)
        | ValueDef::SpellsCastBeforeThisTurn
        | ValueDef::PlayerCounters { .. }
        | ValueDef::SacrificedManaValue
        | ValueDef::AdditionalCostPayments(_)
        | ValueDef::DistinctTargets
        | ValueDef::DividedAmongTargets
        | ValueDef::ResolvedRecipientCount => Ok(()),
        ValueDef::CardTypesAmongObjects(objects) => {
            validate_object_set_shape(*objects, targets)
        }
    }
}

fn validate_aggregate_shape(
    objects: ObjectSetDef,
    targets: &[AbilityTargetDef],
) -> Result<(), GrantedAbilityValidationError> {
    validate_object_set_shape(objects, targets)
}

fn validate_object_predicate_shape(
    predicate: ObjectPredicateDef,
    targets: &[AbilityTargetDef],
) -> Result<(), GrantedAbilityValidationError> {
    match predicate {
        ObjectPredicateDef::All(predicates) | ObjectPredicateDef::AnyOf(predicates) => {
            for predicate in predicates {
                validate_object_predicate_shape(*predicate, targets)?;
            }
            Ok(())
        }
        ObjectPredicateDef::Not(predicate) | ObjectPredicateDef::AttachedTo(predicate) => {
            validate_object_predicate_shape(*predicate, targets)
        }
        ObjectPredicateDef::ManaValueEqualTo(value)
        | ObjectPredicateDef::ManaValueAtMostValue(value)
        | ObjectPredicateDef::ToughnessLessThan(value)
        | ObjectPredicateDef::PowerGreaterThan(value)
        | ObjectPredicateDef::ToughnessGreaterThan(value)
        | ObjectPredicateDef::PowerLessThan(value) => validate_value_shape(value, targets),
        ObjectPredicateDef::NameEquals(name) => validate_card_name_shape(name, targets),
        ObjectPredicateDef::NameIn(names) => validate_card_name_set_shape(*names, targets),
        _ => Ok(()),
    }
}

fn validate_card_name_shape(
    name: CardNameDef,
    targets: &[AbilityTargetDef],
) -> Result<(), GrantedAbilityValidationError> {
    match name {
        CardNameDef::NameOf(reference) => validate_object_reference_shape(reference, targets),
        CardNameDef::Literal(_) | CardNameDef::Binding(_) => Ok(()),
    }
}

fn validate_card_name_set_shape(
    names: CardNameSetDef,
    targets: &[AbilityTargetDef],
) -> Result<(), GrantedAbilityValidationError> {
    match names {
        CardNameSetDef::Union(sets) => sets
            .iter()
            .copied()
            .try_for_each(|names| validate_card_name_set_shape(names, targets)),
        CardNameSetDef::NamesOf(objects)
        | CardNameSetDef::NamesAppearingAtLeast { objects, .. } => {
            validate_object_set_shape(*objects, targets)
        }
        CardNameSetDef::AllCardNames
        | CardNameSetDef::NonlandCardNames
        | CardNameSetDef::LandCardNames
        | CardNameSetDef::NonbasicLandCardNames
        | CardNameSetDef::CardNamesOtherThanBasicLands
        | CardNameSetDef::BasicLandNames => Ok(()),
    }
}

fn validate_damage_matcher_shape(
    matcher: DamageEventMatcherDef,
    targets: &[AbilityTargetDef],
) -> Result<(), GrantedAbilityValidationError> {
    match matcher.source {
        DamageSourceMatcherDef::Object(reference) | DamageSourceMatcherDef::Except(reference) => {
            validate_object_reference_shape(reference, targets)?;
        }
        DamageSourceMatcherDef::Matching(predicate) => {
            validate_object_predicate_shape(predicate, targets)?;
        }
        DamageSourceMatcherDef::Any
        | DamageSourceMatcherDef::Group(_)
        | DamageSourceMatcherDef::AffectedObject => {}
    }
    match matcher.recipient {
        DamageRecipientMatcherDef::Recipients(recipient) => {
            validate_recipient_shape(recipient, targets, RecipientExpectation::Any)
        }
        DamageRecipientMatcherDef::PlayerAndCreaturesControlledBy(player) => {
            validate_player_reference_shape(player, targets)
        }
        DamageRecipientMatcherDef::MatchingObject(_)
        | DamageRecipientMatcherDef::Any
        | DamageRecipientMatcherDef::AffectedObject
        | DamageRecipientMatcherDef::PlayerOrPlaneswalker => Ok(()),
    }
}

fn validate_condition_shape(
    condition: ConditionDef,
    targets: &[AbilityTargetDef],
) -> Result<(), GrantedAbilityValidationError> {
    match condition {
        ConditionDef::Exists(query) => validate_query_shape(query, targets),
        ConditionDef::ObjectCount(counting) => validate_query_shape(counting.query, targets),
        ConditionDef::ControllerTurnsTakenAtMost(_) => Ok(()),
        ConditionDef::All(conditions) => conditions
            .iter()
            .try_for_each(|condition| validate_condition_shape(*condition, targets)),
    }
}

fn validate_value_pair_shape(
    left: ValueDef,
    right: ValueDef,
    targets: &[AbilityTargetDef],
) -> Result<(), GrantedAbilityValidationError> {
    validate_value_shape(left, targets)?;
    validate_value_shape(right, targets)
}

fn validate_trigger_condition_shape(
    condition: TriggerConditionDef,
    targets: &[AbilityTargetDef],
) -> Result<(), GrantedAbilityValidationError> {
    match condition {
        TriggerConditionDef::All(conditions) | TriggerConditionDef::AnyOf(conditions) => conditions
            .iter()
            .copied()
            .try_for_each(|condition| validate_trigger_condition_shape(condition, targets)),
        TriggerConditionDef::Not(condition) => {
            validate_trigger_condition_shape(*condition, targets)
        }
        TriggerConditionDef::ObjectCount { query, .. } => validate_query_shape(query, targets),
        TriggerConditionDef::ObjectSetCount(condition) => {
            validate_object_set_shape(*condition.objects, targets)?;
            condition.predicate.filter.map_or(Ok(()), |filter| {
                validate_object_predicate_shape(filter.predicate(), targets)
            })
        }
        TriggerConditionDef::SourceMatches { object }
        | TriggerConditionDef::BoundObjectMatches { object, .. }
        | TriggerConditionDef::SacrificedObjectMatches(object)
        | TriggerConditionDef::AttachedPermanentMatches { object } => {
            validate_object_predicate_shape(object, targets)
        }
        TriggerConditionDef::TargetMatches { slot, object } => {
            validate_target_shape(slot, targets, RecipientExpectation::Object, false)?;
            validate_object_predicate_shape(object, targets)
        }
        TriggerConditionDef::ControllerHadPermanentLeaveThisTurn
        | TriggerConditionDef::ControllerHadCardLeaveGraveyardThisTurn
        | TriggerConditionDef::ControllerHasCitysBlessing
        | TriggerConditionDef::ControllerGainedLifeThisTurn
        | TriggerConditionDef::CreatureDiedThisTurn
        | TriggerConditionDef::SourceArrivedSinceControllersLastUpkeep
        | TriggerConditionDef::SourceOnBattlefield
        | TriggerConditionDef::SourceInZone(_)
        | TriggerConditionDef::SourceUntapped
        | TriggerConditionDef::SourceIsPaired
        | TriggerConditionDef::ActivePlayer(_)
        | TriggerConditionDef::SpellsCastThisTurn { .. }
        | TriggerConditionDef::SpellsCastLastTurn { .. }
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
        | TriggerConditionDef::ControllerLifeAtMostHalfStartingLife
        | TriggerConditionDef::ControlsGreatestPowerCreature
        | TriggerConditionDef::SourceCounters { .. } => Ok(()),
        TriggerConditionDef::ValueComparison(values) => {
            validate_value_shape(values.left, targets)?;
            validate_value_shape(values.right, targets)
        }
    }
}

fn validate_payment_shape(
    payment: EffectPaymentDef,
    targets: &[AbilityTargetDef],
) -> Result<(), GrantedAbilityValidationError> {
    validate_player_set_shape(payment.payer, targets)?;
    if let PlayerSetDef::LegalTargets(target) = payment.payer {
        validate_target_shape(target, targets, RecipientExpectation::Any, true)?;
        validate_target_projection(target, targets, RecipientExpectation::Player)?;
    }
    if let EffectPaymentCostDef::GenericMana(amount) = payment.cost {
        validate_value_shape(amount, targets)?;
    }
    Ok(())
}

fn applied_effect_adds_ability(effect: AppliedEffectDef) -> bool {
    match effect {
        AppliedEffectDef::Composite(effects) => {
            effects.iter().copied().any(applied_effect_adds_ability)
        }
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
            AbilityOperationDef::Add(_),
        )) => true,
        AppliedEffectDef::Characteristic(_) | AppliedEffectDef::Rule(_) => false,
    }
}

fn nonbattlefield_ability_grants_are_flashback(effect: AppliedEffectDef) -> bool {
    match effect {
        AppliedEffectDef::Composite(effects) => effects
            .iter()
            .copied()
            .all(nonbattlefield_ability_grants_are_flashback),
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
            AbilityOperationDef::Add(ability),
        )) => {
            matches!(
                    ability.definition,
                    DeclarativeAbilityDef::AlternativeCast(definition)
                        if definition.kind == AlternativeCastKindDef::Flashback
                )
        }
        AppliedEffectDef::Characteristic(_) | AppliedEffectDef::Rule(_) => true,
    }
}

fn nonbattlefield_ability_grants_are_suspend(effect: AppliedEffectDef) -> bool {
    match effect {
        AppliedEffectDef::Composite(effects) => effects
            .iter()
            .copied()
            .all(nonbattlefield_ability_grants_are_suspend),
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
            AbilityOperationDef::Add(ability),
        )) => {
            matches!(
                    ability.definition,
                    DeclarativeAbilityDef::Keyword(crate::card::KeywordAbility::Suspend(
                        crate::card::SuspendAbilityDef::Granted
                    ))
                )
        }
        AppliedEffectDef::Characteristic(_) | AppliedEffectDef::Rule(_) => true,
    }
}

/// Whole replacement abilities granted to a resolving permanent spell can
/// move onto the permanent it becomes. The event restriction is what keeps a
/// granted battlefield-only ability from being mistaken for stack behavior.
fn nonbattlefield_ability_grants_are_source_entry_replacements(
    effect: AppliedEffectDef,
) -> bool {
    match effect {
        AppliedEffectDef::Composite(effects) => effects
            .iter()
            .copied()
            .all(nonbattlefield_ability_grants_are_source_entry_replacements),
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
            AbilityOperationDef::Add(ability),
        )) => matches!(
            ability.definition,
            DeclarativeAbilityDef::Replacement(definition)
                if definition.event == ReplacementEventDef::SourceEntersBattlefield
        ),
        AppliedEffectDef::Characteristic(_) | AppliedEffectDef::Rule(_) => false,
    }
}

fn target_may_name_nonbattlefield_object(
    target: TargetIndex,
    targets: &[AbilityTargetDef],
) -> bool {
    targets.get(target.index()).is_none_or(|definition| {
        target_predicate_may_name_nonbattlefield_object(definition.predicate)
    })
}

fn recipient_may_name_nonbattlefield_object(
    recipient: EffectRecipientDef,
    targets: &[AbilityTargetDef],
    triggering_object_zone: Option<ZoneKind>,
) -> bool {
    match recipient.0 {
        EffectRecipientSetDef::Objects(ObjectSetDef::Union(sets)) => sets.iter().copied().any(
            |objects| {
                recipient_may_name_nonbattlefield_object(
                    EffectRecipientDef::objects(objects),
                    targets,
                    triggering_object_zone,
                )
            },
        ),
        EffectRecipientSetDef::LegalTargets(target)
        | EffectRecipientSetDef::Objects(
            ObjectSetDef::LegalTargets(target) | ObjectSetDef::One(ObjectRefDef::Target(target)),
        ) => target_may_name_nonbattlefield_object(target, targets),
        EffectRecipientSetDef::Objects(ObjectSetDef::Query(query)) => query
            .zones
            .iter()
            .any(|zone| *zone != ZoneKind::Battlefield),
        EffectRecipientSetDef::Objects(
            ObjectSetDef::One(
                ObjectRefDef::Binding(_)
                | ObjectRefDef::AdditionalCostObject(_)
                | ObjectRefDef::ZoneChangeSuccessor(_)
                | ObjectRefDef::ZoneChangeResultOfTriggeringObject,
            )
            | ObjectSetDef::Binding(_)

            | ObjectSetDef::ZoneChangeSuccessorsOfBinding(_)
            | ObjectSetDef::MatchingBinding { .. }
            | ObjectSetDef::Matching { .. }
            | ObjectSetDef::ExceptObject { .. }
            // A graveyard is not the battlefield, which is the whole point of
            // naming a card at either end of it.
            | ObjectSetDef::LinkedExiles
            | ObjectSetDef::CardsDrawnThisTurnInHand(_)
            | ObjectSetDef::BottomOfGraveyard(_)
            | ObjectSetDef::TopOfGraveyardMatching { .. },
        ) => true,
        EffectRecipientSetDef::Objects(ObjectSetDef::One(ObjectRefDef::TriggeringObject)) => {
            triggering_object_zone != Some(ZoneKind::Battlefield)
        }
        EffectRecipientSetDef::Objects(
            ObjectSetDef::One(
                ObjectRefDef::Source
                | ObjectRefDef::CreatingSource
                | ObjectRefDef::AbilityGrantSource
                | ObjectRefDef::ResolvingObject
                | ObjectRefDef::AttachedToSource
                // Damage reaches players and permanents, so what took it is
                // on the battlefield or is not an object at all.
                | ObjectRefDef::DamagedObject
                // The permanent behind a countered ability, which is on the
                // battlefield or nowhere.
                | ObjectRefDef::SourceOfTargetedStackObject(_),
            )
            | ObjectSetDef::PermanentsTargetedBy(_)
            | ObjectSetDef::PlayerAttachments(_)
            | ObjectSetDef::LegalAttachmentHosts(_)
            | ObjectSetDef::PermanentsControlledBy(_)
            | ObjectSetDef::TokensCreatedBy(_),
        )
        // Players and the creatures they control: nothing outside the
        // battlefield is named either way.
        | EffectRecipientSetDef::PlayersAndCreaturesTheyControl(_)
        // An attacker is on the battlefield, and so is anything it can be
        // declared against.
        | EffectRecipientSetDef::DefenderOf(_)
        | EffectRecipientSetDef::Players(_) => false,
    }
}

fn recipient_nonbattlefield_zones_support_flashback(
    recipient: EffectRecipientDef,
    targets: &[AbilityTargetDef],
    triggering_object_zone: Option<ZoneKind>,
) -> bool {
    match recipient.0 {
        EffectRecipientSetDef::Objects(ObjectSetDef::Union(sets)) => sets.iter().copied().all(
            |objects| {
                recipient_nonbattlefield_zones_support_flashback(
                    EffectRecipientDef::objects(objects),
                    targets,
                    triggering_object_zone,
                )
            },
        ),
        EffectRecipientSetDef::LegalTargets(target)
        | EffectRecipientSetDef::Objects(
            ObjectSetDef::LegalTargets(target) | ObjectSetDef::One(ObjectRefDef::Target(target)),
        ) => targets.get(target.index()).is_some_and(|definition| {
            target_predicate_zones_support_flashback(definition.predicate)
        }),
        EffectRecipientSetDef::Objects(ObjectSetDef::Query(query)) => {
            zones_support_flashback(query.zones)
        }
        EffectRecipientSetDef::Objects(ObjectSetDef::One(ObjectRefDef::TriggeringObject)) => {
            matches!(
                triggering_object_zone,
                Some(ZoneKind::Battlefield | ZoneKind::Graveyard)
            )
        }
        EffectRecipientSetDef::Objects(
            ObjectSetDef::One(
                ObjectRefDef::Binding(_)
                | ObjectRefDef::AdditionalCostObject(_),
            )
            | ObjectSetDef::Binding(_)

            | ObjectSetDef::ZoneChangeSuccessorsOfBinding(_)
            | ObjectSetDef::MatchingBinding { .. }
            | ObjectSetDef::Matching { .. }
            | ObjectSetDef::ExceptObject { .. }
            | ObjectSetDef::LinkedExiles
            | ObjectSetDef::CardsDrawnThisTurnInHand(_)
            | ObjectSetDef::BottomOfGraveyard(_)
            | ObjectSetDef::TopOfGraveyardMatching { .. },
        ) => false,
        EffectRecipientSetDef::Objects(
            ObjectSetDef::One(
                ObjectRefDef::Source
                | ObjectRefDef::CreatingSource
                | ObjectRefDef::ZoneChangeSuccessor(_)
                | ObjectRefDef::ZoneChangeResultOfTriggeringObject
                | ObjectRefDef::AbilityGrantSource
                | ObjectRefDef::ResolvingObject
                | ObjectRefDef::AttachedToSource
                | ObjectRefDef::SourceOfTargetedStackObject(_)
                // Damage reaches players and permanents, so what took it was
                // on the battlefield.
                | ObjectRefDef::DamagedObject,
            )
            | ObjectSetDef::PermanentsTargetedBy(_)
            | ObjectSetDef::PlayerAttachments(_)
            | ObjectSetDef::LegalAttachmentHosts(_)
            | ObjectSetDef::PermanentsControlledBy(_)
            | ObjectSetDef::TokensCreatedBy(_),
        )
        | EffectRecipientSetDef::PlayersAndCreaturesTheyControl(_)
        | EffectRecipientSetDef::DefenderOf(_)
        | EffectRecipientSetDef::Players(_) => true,
    }
}

/// Which player sets a static play rule may name. A resolving one may name
/// whoever the effect points at; a static one is read live wherever a play is
/// offered, so its recipient has to be a set that can be answered there.
fn static_play_rule_recipient_supported(recipient: EffectRecipientDef) -> bool {
    matches!(
        recipient.0,
        EffectRecipientSetDef::Players(
            PlayerSetDef::All
                | PlayerSetDef::Related(
                    PlayerRelation::Any
                        | PlayerRelation::You
                        | PlayerRelation::NotYou
                        | PlayerRelation::Opponent
                        | PlayerRelation::ActivePlayer
                        | PlayerRelation::NonactivePlayer
                        | PlayerRelation::EnchantedPlayer
                )
                | PlayerSetDef::One(PlayerRefDef::EffectController | PlayerRefDef::EnchantedPlayer)
        )
    )
}

fn validate_applied_effect_shapes(
    recipient: EffectRecipientDef,
    effect: AppliedEffectDef,
    targets: &[AbilityTargetDef],
    static_effect: bool,
) -> Result<(), GrantedAbilityValidationError> {
    match effect {
        AppliedEffectDef::Composite(effects) => {
            for effect in effects {
                validate_applied_effect_shapes(recipient, *effect, targets, static_effect)?;
            }
            Ok(())
        }
        // A grant over a whole graveyard names the player whose graveyard it
        // is, the same way the permissions below name a player. A timing
        // permission names its player for the same reason: no object has a
        // casting window of its own.
        AppliedEffectDef::Rule(AppliedRuleDef::GrantsAlternativeCastFromGraveyard {
            object,
            ..
        }) => {
            validate_recipient_shape(recipient, targets, RecipientExpectation::Player)?;
            validate_object_predicate_shape(object, targets)
        }
        AppliedEffectDef::Rule(AppliedRuleDef::MayCastAsThoughItHadFlash(permission)) => {
            validate_recipient_shape(recipient, targets, RecipientExpectation::Player)?;
            validate_object_predicate_shape(permission.object, targets)
        }
        AppliedEffectDef::Rule(AppliedRuleDef::MayPlayFromGraveyard(permission)) => {
            validate_recipient_shape(recipient, targets, RecipientExpectation::Player)?;
            validate_object_predicate_shape(permission.restriction.object, targets)?;
            Ok(())
        }
        AppliedEffectDef::Rule(AppliedRuleDef::TriggersAnAdditionalTime(doubling)) => {
            validate_recipient_shape(recipient, targets, RecipientExpectation::Player)?;
            validate_object_predicate_shape(doubling.entering, targets)?;
            validate_object_predicate_shape(doubling.permanent, targets)?;
            Ok(())
        }
        AppliedEffectDef::Rule(
            AppliedRuleDef::CannotPlay(restriction)
            | AppliedRuleDef::MayPlayFromTopOfLibrary { restriction, .. },
        ) => {
            validate_recipient_shape(recipient, targets, RecipientExpectation::Player)?;
            validate_object_predicate_shape(restriction.object, targets)?;
            if static_effect && !static_play_rule_recipient_supported(recipient) {
                return Err(
                    GrantedAbilityValidationError::UnsupportedStaticPlayerRecipient { recipient },
                );
            }
            Ok(())
        }
        AppliedEffectDef::Rule(AppliedRuleDef::PlayerRule(
            crate::card::PlayerRuleDef::LegendRuleDoesNotApplyTo(predicate),
        )) => validate_predicated_player_rule_shape(recipient, *predicate, targets),
        // Each names a player and carries nothing else.
        AppliedEffectDef::Rule(
            AppliedRuleDef::Ascend
            | AppliedRuleDef::MayLookAtTopOfLibrary
            | AppliedRuleDef::PlaysWithTopOfLibraryRevealed
            | AppliedRuleDef::MaySpendManaAsAnyColorForCreatureAbilities
            | AppliedRuleDef::MayPlayAdditionalLands(_)
            | AppliedRuleDef::MayPlayAnyNumberOfLands
            | AppliedRuleDef::CannotDrawMoreThanEachTurn(_)
            | AppliedRuleDef::RevealsDrawnCards
            | AppliedRuleDef::CannotGainLife
            | AppliedRuleDef::PlayerRule(_)
            | AppliedRuleDef::DoublesTokensCreated,
        ) => validate_recipient_shape(recipient, targets, RecipientExpectation::Player),
        // The cap names the players it applies to; the predicate picks out
        // which of their permanents it covers.
        AppliedEffectDef::Rule(AppliedRuleDef::UntapAtMostOne(predicate)) => {
            validate_recipient_shape(recipient, targets, RecipientExpectation::Player)?;
            validate_object_predicate_shape(predicate, targets)
        }
        // The protection names the player who has it and the quality it is
        // from, which is an ordinary object predicate read against whatever
        // would damage, target, or enchant them.
        AppliedEffectDef::Rule(AppliedRuleDef::PlayerProtectionFrom(quality)) => {
            validate_recipient_shape(recipient, targets, RecipientExpectation::Player)?;
            validate_object_predicate_shape(quality, targets)
        }
        AppliedEffectDef::Rule(AppliedRuleDef::AttackRestriction(restriction)) => {
            validate_attack_restriction_shape(recipient, restriction, targets)
        }
        AppliedEffectDef::Rule(AppliedRuleDef::BlockRestriction(restriction)) => {
            validate_block_restriction_shape(recipient, restriction, targets)
        }
        AppliedEffectDef::Rule(AppliedRuleDef::PreventDamage(matcher)) => {
            validate_recipient_shape(recipient, targets, RecipientExpectation::Object)?;
            validate_damage_matcher_shape(matcher, targets)
        }
        // A limit protects a player, so unlike prevention its recipient is
        // the player whose damage is capped.
        AppliedEffectDef::Rule(AppliedRuleDef::LimitDamage { matcher, .. }) => {
            validate_recipient_shape(recipient, targets, RecipientExpectation::Player)?;
            validate_damage_matcher_shape(matcher, targets)
        }
        AppliedEffectDef::Rule(AppliedRuleDef::RedirectDamageFromTo {
            source,
            destination,
        }) => {
            validate_recipient_shape(recipient, targets, RecipientExpectation::Player)?;
            validate_object_reference_shape(source, targets)?;
            validate_object_reference_shape(destination, targets)
        }
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
            PowerToughnessOperationDef::SetBase { power, toughness }
            | PowerToughnessOperationDef::Modify { power, toughness },
        )) => {
            validate_recipient_shape(recipient, targets, RecipientExpectation::Object)?;
            validate_value_shape(power, targets)?;
            validate_value_shape(toughness, targets)
        }
        AppliedEffectDef::Rule(_) | AppliedEffectDef::Characteristic(_) => {
            validate_recipient_shape(recipient, targets, RecipientExpectation::Object)
        }
    }
}

fn validate_predicated_player_rule_shape(
    recipient: EffectRecipientDef,
    predicate: ObjectPredicateDef,
    targets: &[AbilityTargetDef],
) -> Result<(), GrantedAbilityValidationError> {
    validate_recipient_shape(recipient, targets, RecipientExpectation::Player)?;
    validate_object_predicate_shape(predicate, targets)
}

fn validate_attack_restriction_shape(
    recipient: EffectRecipientDef,
    restriction: AttackRestrictionDef,
    targets: &[AbilityTargetDef],
) -> Result<(), GrantedAbilityValidationError> {
    let expectation = match restriction.defender {
        AttackDefenderScopeDef::Any => RecipientExpectation::Object,
        AttackDefenderScopeDef::AffectedPlayer
        | AttackDefenderScopeDef::AffectedPlayerOrPlaneswalker => RecipientExpectation::Player,
    };
    validate_recipient_shape(recipient, targets, expectation)?;
    validate_object_predicate_shape(restriction.attacker, targets)
}

fn validate_block_restriction_shape(
    recipient: EffectRecipientDef,
    restriction: BlockRestrictionDef,
    targets: &[AbilityTargetDef],
) -> Result<(), GrantedAbilityValidationError> {
    validate_recipient_shape(recipient, targets, RecipientExpectation::Object)?;
    match restriction {
        BlockRestrictionDef::Pair { counterpart, .. } => match counterpart {
            BlockRestrictionMatchDef::Any => Ok(()),
            BlockRestrictionMatchDef::Matching(predicate)
            | BlockRestrictionMatchDef::Except(predicate) => {
                validate_object_predicate_shape(predicate, targets)
            }
        },
        BlockRestrictionDef::MinimumBlockers(_) => Ok(()),
    }
}
