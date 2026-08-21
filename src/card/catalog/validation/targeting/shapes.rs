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

fn trigger_event_object_zone(event: TriggerEventDef) -> Option<ZoneKind> {
    match event {
        // Every alternative has to agree, or the ability's targets would read
        // an object from one zone on one path and another zone on the next.
        TriggerEventDef::AnyOf(events) => {
            let mut zones = events.iter().map(|event| trigger_event_object_zone(*event));
            let first = zones.next()?;
            zones.all(|zone| zone == first).then_some(first)?
        }
        TriggerEventDef::ZoneChanged(matcher) => matcher.to,
        TriggerEventDef::Tapped(_)
        | TriggerEventDef::Attacks(_)
        | TriggerEventDef::AttackDeclared { .. }
        | TriggerEventDef::CardsExiled { .. }
        | TriggerEventDef::AttacksAndIsNotBlocked { .. }
        | TriggerEventDef::BecomesBlocked(_)
        | TriggerEventDef::BlocksOrBecomesBlockedBy { .. }
        | TriggerEventDef::Blocks { .. }
        | TriggerEventDef::BecomesBlockedBy { .. }
        | TriggerEventDef::CountersPlaced { .. }
        | TriggerEventDef::Transforms(_) => Some(ZoneKind::Battlefield),
        // Both name the spell rather than what it points at, and a spell
        // is on the stack.
        TriggerEventDef::SpellCast(_)
        | TriggerEventDef::BecomesTargetOfSpell(_)
        | TriggerEventDef::BecomesTargetOfSpellOrAbility(_) => {
            Some(ZoneKind::Stack)
        }
        // The cycled card is in the graveyard by the time the trigger goes
        // on the stack, but nothing reads it as an object, so it names no
        // zone at all.
        TriggerEventDef::CommittedCrime(_)
        | TriggerEventDef::BecomesLevel(_)
        | TriggerEventDef::Cycled
        | TriggerEventDef::DoorUnlocked
        | TriggerEventDef::StepBegins { .. }
        | TriggerEventDef::DamageDealt(_)
        | TriggerEventDef::StateCondition
        | TriggerEventDef::LifeGained(_)
        | TriggerEventDef::BecomesMonarch(_)
        | TriggerEventDef::DrewCard(_)
        // The card is already in a graveyard and nothing reads it, so the
        // event names no object at all.
        | TriggerEventDef::Discarded(_) => None,
    }
}

fn target_matches_expectation(
    predicate: AbilityTargetPredicate,
    expected: RecipientExpectation,
) -> bool {
    match expected {
        RecipientExpectation::Any => true,
        RecipientExpectation::Object => matches!(
            predicate,
            AbilityTargetPredicate::Object { .. }
                | AbilityTargetPredicate::StackObject { .. }
                | AbilityTargetPredicate::ControlledByTargetOf { .. }
        ),
        RecipientExpectation::Player => matches!(predicate, AbilityTargetPredicate::Player(_)),
    }
}

fn target_can_project(predicate: AbilityTargetPredicate, expected: RecipientExpectation) -> bool {
    match expected {
        RecipientExpectation::Any => true,
        RecipientExpectation::Object => !matches!(predicate, AbilityTargetPredicate::Player(_)),
        RecipientExpectation::Player => !matches!(
            predicate,
            AbilityTargetPredicate::Object { .. }
                | AbilityTargetPredicate::ControlledByTargetOf { .. }
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
        PlayerRefDef::ControllerOf(ObjectRefDef::Target(target)) => {
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
        PlayerRefDef::ControllerOf(reference) | PlayerRefDef::OwnerOf(reference) => {
            validate_object_reference_shape(reference, targets)
        }
        PlayerRefDef::EffectController | PlayerRefDef::EventPlayer | PlayerRefDef::Opponent => {
            Ok(())
        }
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
    validate_object_predicate_shape(query.object, targets)
}

fn validate_object_set_shape(
    objects: ObjectSetDef,
    targets: &[AbilityTargetDef],
) -> Result<(), GrantedAbilityValidationError> {
    match objects {
        ObjectSetDef::One(reference)
        | ObjectSetDef::PermanentsTargetedBy(reference)
        | ObjectSetDef::SharingNameWith(reference) => {
            validate_object_reference_shape(reference, targets)
        }
        ObjectSetDef::Query(query) => validate_query_shape(query, targets),
        ObjectSetDef::LegalTargets(target) => {
            validate_target_projection(target, targets, RecipientExpectation::Object)
        }
        ObjectSetDef::Binding(_)
                | ObjectSetDef::MatchingBinding { .. }
        | ObjectSetDef::LinkedExiles(_)
            | ObjectSetDef::BottomOfGraveyard(_)
        | ObjectSetDef::SharingNameWithBinding { .. }
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
    }
}

fn validate_value_shape(
    value: ValueDef,
    targets: &[AbilityTargetDef],
) -> Result<(), GrantedAbilityValidationError> {
    match value {
        ValueDef::Negate(value) => validate_value_shape(*value, targets),
        ValueDef::Scaled(value) => validate_value_shape(value.value, targets),
        ValueDef::Halved(value) => validate_value_shape(value.value, targets),
        ValueDef::Sum(value) => {
            validate_value_shape(value.left, targets)?;
            validate_value_shape(value.right, targets)
        }
        ValueDef::IfControllerLifeAtMost(value) => {
            validate_value_shape(value.then, targets)?;
            validate_value_shape(value.otherwise, targets)
        }
        ValueDef::IfCreatureDiedThisTurn(value) => {
            validate_value_shape(value.then, targets)?;
            validate_value_shape(value.otherwise, targets)
        }
        ValueDef::IfTargetMatches(value) => {
            validate_target_shape(value.slot, targets, RecipientExpectation::Object, false)?;
            validate_value_shape(value.then, targets)?;
            validate_value_shape(value.otherwise, targets)
        }
        ValueDef::IfMatchingObjectCount(value) => {
            validate_query_shape(value.query, targets)?;
            validate_value_shape(value.then, targets)?;
            validate_value_shape(value.otherwise, targets)
        }
        ValueDef::CountMatchingObjects(query)
        | ValueDef::AnyMatchingObject(query)
        | ValueDef::GreatestPowerAmong(query) => validate_query_shape(*query, targets),
        ValueDef::TargetLibrarySize(target) => {
            validate_target_shape(target, targets, RecipientExpectation::Player, true)
        }
        ValueDef::TargetPower(target)
        | ValueDef::TargetToughness(target)
        | ValueDef::TargetManaValue(target) => {
            validate_target_shape(target, targets, RecipientExpectation::Object, true)
        }
        ValueDef::CreaturesDiedThisTurn
        | ValueDef::CardTypesAmongGraveyards(_)
        | ValueDef::IfCardTypesAmongGraveyards(_)
        | ValueDef::LifeTotal(_)
        | ValueDef::Constant(_)
        | ValueDef::ChosenX
        | ValueDef::SourceCastX
        | ValueDef::SourcePower
        | ValueDef::AffectedManaValue
        | ValueDef::AffectedColorCount
        | ValueDef::TotalPowerOfLinkedExiles
        | ValueDef::TotalToughnessOfLinkedExiles
        | ValueDef::SourceToughness
        | ValueDef::TriggeringObjectPower
        | ValueDef::TriggeringObjectToughness
        | ValueDef::TriggerEventAmount
        | ValueDef::CardsInHandAbove { .. }
        | ValueDef::DamageTakenThisTurn { .. }
        | ValueDef::CountersOnSource(_)
        | ValueDef::CardsDrawnThisTurn(_)
        | ValueDef::DevotionTo(_)
        | ValueDef::LibrarySize(_)
        | ValueDef::ColorsOfManaSpent
        | ValueDef::PaidAmount
        | ValueDef::MatchedCount
        | ValueDef::MatchedCardTypes
        | ValueDef::BoundObjectCount(_)
        | ValueDef::SpellsCastBeforeThisTurn
        | ValueDef::DividedAmongTargets => Ok(()),
    }
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
        _ => Ok(()),
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
        DamageRecipientMatcherDef::Any
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
        ConditionDef::All(conditions) => conditions
            .iter()
            .try_for_each(|condition| validate_condition_shape(*condition, targets)),
    }
}

fn validate_trigger_condition_shape(
    condition: TriggerConditionDef,
    targets: &[AbilityTargetDef],
) -> Result<(), GrantedAbilityValidationError> {
    match condition {
        TriggerConditionDef::All(conditions) => conditions
            .iter()
            .copied()
            .try_for_each(|condition| validate_trigger_condition_shape(condition, targets)),
        TriggerConditionDef::Not(condition) => {
            validate_trigger_condition_shape(*condition, targets)
        }
        TriggerConditionDef::ObjectCount { query, .. } => validate_query_shape(query, targets),
        TriggerConditionDef::SourceMatches { object }
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
        | TriggerConditionDef::BoundObjectsShareName { .. }
        | TriggerConditionDef::SourceArrivedSinceControllersLastUpkeep
        | TriggerConditionDef::SourceOnBattlefield
        | TriggerConditionDef::SourceUntapped
        | TriggerConditionDef::SourceIsPaired
        | TriggerConditionDef::ActivePlayer(_)
        | TriggerConditionDef::SpellsCastThisTurn { .. }
        | TriggerConditionDef::SpellsCastLastTurn { .. }
        | TriggerConditionDef::SourceCastWith(_)
        | TriggerConditionDef::SourceCastFrom(_)
        | TriggerConditionDef::SourceCastAtInstantSpeed
        | TriggerConditionDef::ValueComparison(_)
        | TriggerConditionDef::SourceLoyalty { .. }
        | TriggerConditionDef::SourceActivationsThisTurn { .. }
        | TriggerConditionDef::SourceResolutionsThisTurn { .. }
        | TriggerConditionDef::SourceDealtDamageToOpponentThisTurn
        | TriggerConditionDef::SourceIsTapped
        | TriggerConditionDef::SourceIsUntapped
        | TriggerConditionDef::ControllerLifeAtMost(_)
        | TriggerConditionDef::ControllerLifeAtMostHalfStartingLife
        | TriggerConditionDef::ControlsGreatestPowerCreature
        | TriggerConditionDef::SourceCounters { .. } => Ok(()),
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

fn nonbattlefield_ability_grants_are_supported(effect: AppliedEffectDef) -> bool {
    match effect {
        AppliedEffectDef::Composite(effects) => effects
            .iter()
            .copied()
            .all(nonbattlefield_ability_grants_are_supported),
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
            AbilityOperationDef::Add(ability),
        )) => {
            ability.is_executable()
                && matches!(
                    ability.definition,
                    DeclarativeAbilityDef::AlternativeCast(definition)
                        if definition.kind == AlternativeCastKindDef::Flashback
                )
        }
        AppliedEffectDef::Characteristic(_) | AppliedEffectDef::Rule(_) => true,
    }
}

fn target_may_name_nonbattlefield_object(
    target: TargetIndex,
    targets: &[AbilityTargetDef],
) -> bool {
    targets.get(target.index()).is_none_or(|definition| {
        matches!(
            definition.predicate,
            AbilityTargetPredicate::Object { zones, .. }
                if zones.iter().any(|zone| *zone != ZoneKind::Battlefield)
        )
    })
}

fn recipient_may_name_nonbattlefield_object(
    recipient: EffectRecipientDef,
    targets: &[AbilityTargetDef],
    triggering_object_zone: Option<ZoneKind>,
) -> bool {
    match recipient.0 {
        EffectRecipientSetDef::LegalTargets(target)
        | EffectRecipientSetDef::Objects(
            ObjectSetDef::LegalTargets(target) | ObjectSetDef::One(ObjectRefDef::Target(target)),
        ) => target_may_name_nonbattlefield_object(target, targets),
        EffectRecipientSetDef::Objects(ObjectSetDef::Query(query)) => query
            .zones
            .iter()
            .any(|zone| *zone != ZoneKind::Battlefield),
        EffectRecipientSetDef::Objects(
            ObjectSetDef::One(ObjectRefDef::Binding(_))
            | ObjectSetDef::Binding(_)
            | ObjectSetDef::MatchingBinding { .. }
            // A graveyard is not the battlefield, which is the whole point of
            // naming a card at either end of it.
            | ObjectSetDef::LinkedExiles(_)
            | ObjectSetDef::BottomOfGraveyard(_)
            | ObjectSetDef::TopOfGraveyardMatching { .. },
        ) => true,
        EffectRecipientSetDef::Objects(ObjectSetDef::SharingNameWithBinding { zone, .. }) => {
            zone != ZoneKind::Battlefield
        }
        EffectRecipientSetDef::Objects(ObjectSetDef::One(ObjectRefDef::TriggeringObject)) => {
            triggering_object_zone != Some(ZoneKind::Battlefield)
        }
        EffectRecipientSetDef::Objects(
            ObjectSetDef::One(
                ObjectRefDef::Source
                | ObjectRefDef::AbilityGrantSource
                | ObjectRefDef::ResolvingObject
                | ObjectRefDef::AttachedToSource
                // The permanent behind a countered ability, which is on the
                // battlefield or nowhere.
                | ObjectRefDef::SourceOfTargetedStackObject(_),
            )
            | ObjectSetDef::PermanentsTargetedBy(_)
            | ObjectSetDef::SharingNameWith(_),
        )
        // Players and the creatures they control: nothing outside the
        // battlefield is named either way.
        | EffectRecipientSetDef::PlayersAndCreaturesTheyControl(_)
        | EffectRecipientSetDef::Players(_) => false,
    }
}

fn recipient_nonbattlefield_zones_support_flashback(
    recipient: EffectRecipientDef,
    targets: &[AbilityTargetDef],
    triggering_object_zone: Option<ZoneKind>,
) -> bool {
    let supported_zones = |zones: &[ZoneKind]| {
        zones
            .iter()
            .all(|zone| matches!(zone, ZoneKind::Battlefield | ZoneKind::Graveyard))
    };
    match recipient.0 {
        EffectRecipientSetDef::LegalTargets(target)
        | EffectRecipientSetDef::Objects(
            ObjectSetDef::LegalTargets(target) | ObjectSetDef::One(ObjectRefDef::Target(target)),
        ) => targets.get(target.index()).is_some_and(|definition| {
            matches!(
                definition.predicate,
                AbilityTargetPredicate::Object { zones, .. } if supported_zones(zones)
            )
        }),
        EffectRecipientSetDef::Objects(ObjectSetDef::Query(query)) => supported_zones(query.zones),
        EffectRecipientSetDef::Objects(ObjectSetDef::One(ObjectRefDef::TriggeringObject)) => {
            matches!(
                triggering_object_zone,
                Some(ZoneKind::Battlefield | ZoneKind::Graveyard)
            )
        }
        EffectRecipientSetDef::Objects(
            ObjectSetDef::One(ObjectRefDef::Binding(_))
            | ObjectSetDef::Binding(_)
            | ObjectSetDef::MatchingBinding { .. }
            | ObjectSetDef::LinkedExiles(_)
            | ObjectSetDef::BottomOfGraveyard(_)
            | ObjectSetDef::SharingNameWithBinding { .. }
            | ObjectSetDef::TopOfGraveyardMatching { .. },
        ) => false,
        EffectRecipientSetDef::Objects(
            ObjectSetDef::One(
                ObjectRefDef::Source
                | ObjectRefDef::AbilityGrantSource
                | ObjectRefDef::ResolvingObject
                | ObjectRefDef::AttachedToSource
                | ObjectRefDef::SourceOfTargetedStackObject(_),
            )
            | ObjectSetDef::PermanentsTargetedBy(_)
            | ObjectSetDef::SharingNameWith(_),
        )
        | EffectRecipientSetDef::PlayersAndCreaturesTheyControl(_)
        | EffectRecipientSetDef::Players(_) => true,
    }
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
        // A grant over a whole graveyard names the player whose graveyard
        // it is, the same way the permissions below name a player.
        AppliedEffectDef::Rule(AppliedRuleDef::GrantsAlternativeCastFromGraveyard {
            object,
            ..
        }) => {
            validate_recipient_shape(recipient, targets, RecipientExpectation::Player)?;
            validate_object_predicate_shape(object, targets)
        }
        AppliedEffectDef::Rule(
            AppliedRuleDef::CannotPlay(restriction)
            | AppliedRuleDef::MayPlayFromGraveyard(restriction)
            | AppliedRuleDef::MayPlayFromTopOfLibrary { restriction, .. },
        ) => {
            validate_recipient_shape(recipient, targets, RecipientExpectation::Player)?;
            validate_object_predicate_shape(restriction.object, targets)?;
            if static_effect
                && !matches!(
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
                            )
                            | PlayerSetDef::One(PlayerRefDef::EffectController)
                    )
                )
            {
                return Err(
                    GrantedAbilityValidationError::UnsupportedStaticPlayerRecipient { recipient },
                );
            }
            Ok(())
        }
        // Each names a player and carries nothing else.
        AppliedEffectDef::Rule(
            AppliedRuleDef::Ascend
            | AppliedRuleDef::MayLookAtTopOfLibrary
            | AppliedRuleDef::MaySpendManaAsAnyColorForCreatureAbilities
            | AppliedRuleDef::NoMaximumHandSize
            | AppliedRuleDef::WinsInsteadOfDrawingFromEmptyLibrary,
        ) => validate_recipient_shape(recipient, targets, RecipientExpectation::Player),
        // The cap names the players it applies to; the predicate picks out
        // which of their permanents it covers.
        AppliedEffectDef::Rule(AppliedRuleDef::UntapAtMostOne(predicate)) => {
            validate_recipient_shape(recipient, targets, RecipientExpectation::Player)?;
            validate_object_predicate_shape(predicate, targets)
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
