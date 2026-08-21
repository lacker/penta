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
        | ObjectRefDef::AttachedToSource
        | ObjectRefDef::TriggeringObject => Ok(()),
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
        PlayerRefDef::EffectController | PlayerRefDef::EventPlayer | PlayerRefDef::Opponent => {
            Ok(())
        }
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
        DamageRecipientMatcherDef::Any
        | DamageRecipientMatcherDef::AffectedObject
        | DamageRecipientMatcherDef::PlayerOrPlaneswalker => Ok(()),
    }
}

fn unsupported_trigger_event(event: TriggerEventDef) -> GrantedAbilityValidationError {
    GrantedAbilityValidationError::UnsupportedTriggerEvent { event }
}

fn validate_trigger_object_predicate(
    predicate: ObjectPredicateDef,
    event: TriggerEventDef,
    target_count: usize,
    scope: BindingScope,
) -> Result<(), GrantedAbilityValidationError> {
    match predicate {
        ObjectPredicateDef::All(predicates) | ObjectPredicateDef::AnyOf(predicates) => {
            for predicate in predicates {
                validate_trigger_object_predicate(*predicate, event, target_count, scope)?;
            }
            Ok(())
        }
        ObjectPredicateDef::Not(predicate) | ObjectPredicateDef::AttachedTo(predicate) => {
            validate_trigger_object_predicate(*predicate, event, target_count, scope)
        }
        ObjectPredicateDef::ManaValueEqualTo(value)
        | ObjectPredicateDef::ManaValueAtMostValue(value)
        | ObjectPredicateDef::ToughnessLessThan(value)
        | ObjectPredicateDef::PowerGreaterThan(value)
        | ObjectPredicateDef::ToughnessGreaterThan(value)
        | ObjectPredicateDef::PowerLessThan(value) => {
            validate_value_target_references(value, target_count, scope)?;
            if matches!(
                value,
                ValueDef::CreaturesDiedThisTurn
                    | ValueDef::CardTypesAmongGraveyards(_)
                    | ValueDef::IfCardTypesAmongGraveyards(_)
                    | ValueDef::Constant(_)
                    | ValueDef::ChosenX
                    | ValueDef::SourceCastX
                    | ValueDef::SourcePower
                    | ValueDef::AffectedManaValue
                    | ValueDef::AffectedColorCount
                    | ValueDef::TotalPowerOfLinkedExiles
                    | ValueDef::TotalToughnessOfLinkedExiles
                    | ValueDef::LifeTotal(_)
                    | ValueDef::SourceToughness
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
            ) {
                Ok(())
            } else {
                Err(unsupported_trigger_event(event))
            }
        }
        ObjectPredicateDef::ControlledBy(
            PlayerRelation::ChosenPlayer | PlayerRelation::EventPlayer,
        )
        | ObjectPredicateDef::Special(_) => Err(unsupported_trigger_event(event)),
        ObjectPredicateDef::Any
        | ObjectPredicateDef::Source
        | ObjectPredicateDef::Token
        | ObjectPredicateDef::Tapped
        | ObjectPredicateDef::WasDealtDamageThisTurn
        | ObjectPredicateDef::DealtDamageThisTurn
        | ObjectPredicateDef::HasType(_)
        | ObjectPredicateDef::HasAnyBasicLandType(_)
        | ObjectPredicateDef::Spell
        | ObjectPredicateDef::NoncreatureSpell
        | ObjectPredicateDef::Color(_)
        | ObjectPredicateDef::ColorCount(_)
        | ObjectPredicateDef::Subtype(_)
        | ObjectPredicateDef::Named(_)
        | ObjectPredicateDef::HasChosenName
        | ObjectPredicateDef::ManaValueAtMost(_)
        | ObjectPredicateDef::PowerAtLeast(_)
        | ObjectPredicateDef::PowerExactly(_)
        | ObjectPredicateDef::ToughnessExactly(_)
        | ObjectPredicateDef::TotalPowerAndToughnessAtMost(_)
        | ObjectPredicateDef::HasCounter(_)
        | ObjectPredicateDef::ControlledBy(_)
        | ObjectPredicateDef::OwnedBy(_)
        | ObjectPredicateDef::Supertype(_)
        | ObjectPredicateDef::DebutSet(_)
        | ObjectPredicateDef::SharesNameWithSource
        | ObjectPredicateDef::HasSourcesChosenScalar(_)
        | ObjectPredicateDef::TargetsObjectMatching(_)
        | ObjectPredicateDef::AttackingOrBlocking
        | ObjectPredicateDef::HasKeyword(_)
        | ObjectPredicateDef::HasNonManaActivatedAbility
        | ObjectPredicateDef::Attacking
        | ObjectPredicateDef::AttachedToSource
        | ObjectPredicateDef::Blocking
        | ObjectPredicateDef::BlockedBySource
        | ObjectPredicateDef::BlockingSource
        | ObjectPredicateDef::BandedWithSource
        | ObjectPredicateDef::Unpaired
        | ObjectPredicateDef::PairedWithSource
        | ObjectPredicateDef::Enchanted
        | ObjectPredicateDef::AttackedThisTurn
        | ObjectPredicateDef::CameUnderControlThisTurn
        | ObjectPredicateDef::AttackedDuringControllersLastTurn => Ok(()),
    }
}

fn trigger_predicate_requires_live_battlefield(predicate: ObjectPredicateDef) -> bool {
    match predicate {
        ObjectPredicateDef::All(predicates) | ObjectPredicateDef::AnyOf(predicates) => predicates
            .iter()
            .copied()
            .any(trigger_predicate_requires_live_battlefield),
        ObjectPredicateDef::Not(predicate) => {
            trigger_predicate_requires_live_battlefield(*predicate)
        }
        ObjectPredicateDef::HasNonManaActivatedAbility | ObjectPredicateDef::AttachedTo(_) => true,
        ObjectPredicateDef::Any
        | ObjectPredicateDef::Source
        | ObjectPredicateDef::Token
        | ObjectPredicateDef::Tapped
        | ObjectPredicateDef::WasDealtDamageThisTurn
        | ObjectPredicateDef::DealtDamageThisTurn
        | ObjectPredicateDef::HasType(_)
        | ObjectPredicateDef::HasAnyBasicLandType(_)
        | ObjectPredicateDef::Spell
        | ObjectPredicateDef::NoncreatureSpell
        | ObjectPredicateDef::Color(_)
        | ObjectPredicateDef::ColorCount(_)
        | ObjectPredicateDef::Subtype(_)
        | ObjectPredicateDef::Named(_)
        | ObjectPredicateDef::HasChosenName
        | ObjectPredicateDef::ManaValueAtMost(_)
        | ObjectPredicateDef::ManaValueEqualTo(_)
        | ObjectPredicateDef::ManaValueAtMostValue(_)
        | ObjectPredicateDef::PowerAtLeast(_)
        | ObjectPredicateDef::PowerExactly(_)
        | ObjectPredicateDef::ToughnessExactly(_)
        | ObjectPredicateDef::TotalPowerAndToughnessAtMost(_)
        | ObjectPredicateDef::ToughnessLessThan(_)
        | ObjectPredicateDef::PowerGreaterThan(_)
        | ObjectPredicateDef::ToughnessGreaterThan(_)
        | ObjectPredicateDef::PowerLessThan(_)
        | ObjectPredicateDef::HasCounter(_)
        | ObjectPredicateDef::ControlledBy(_)
        | ObjectPredicateDef::OwnedBy(_)
        | ObjectPredicateDef::Supertype(_)
        | ObjectPredicateDef::DebutSet(_)
        | ObjectPredicateDef::SharesNameWithSource
        | ObjectPredicateDef::HasSourcesChosenScalar(_)
        | ObjectPredicateDef::TargetsObjectMatching(_)
        | ObjectPredicateDef::AttackingOrBlocking
        | ObjectPredicateDef::HasKeyword(_)
        | ObjectPredicateDef::AttachedToSource
        | ObjectPredicateDef::Attacking
        | ObjectPredicateDef::Blocking
        | ObjectPredicateDef::BlockedBySource
        | ObjectPredicateDef::BlockingSource
        | ObjectPredicateDef::BandedWithSource
        | ObjectPredicateDef::Unpaired
        | ObjectPredicateDef::PairedWithSource
        | ObjectPredicateDef::Enchanted
        | ObjectPredicateDef::AttackedThisTurn
        | ObjectPredicateDef::CameUnderControlThisTurn
        | ObjectPredicateDef::AttackedDuringControllersLastTurn
        | ObjectPredicateDef::Special(_) => false,
    }
}

fn validate_trigger_object_reference(
    reference: ObjectRefDef,
    event: TriggerEventDef,
    target_count: usize,
    scope: BindingScope,
) -> Result<(), GrantedAbilityValidationError> {
    validate_object_reference(reference, target_count, scope)?;
    if matches!(
        reference,
        ObjectRefDef::Source | ObjectRefDef::AttachedToSource | ObjectRefDef::TriggeringObject
    ) {
        Ok(())
    } else {
        Err(unsupported_trigger_event(event))
    }
}

fn validate_trigger_player_reference(
    reference: PlayerRefDef,
    event: TriggerEventDef,
    target_count: usize,
    scope: BindingScope,
) -> Result<(), GrantedAbilityValidationError> {
    validate_player_reference(reference, target_count, scope)?;
    match reference {
        PlayerRefDef::EffectController | PlayerRefDef::EventPlayer | PlayerRefDef::Opponent => {
            Ok(())
        }
        PlayerRefDef::ControllerOf(reference) | PlayerRefDef::OwnerOf(reference) => {
            validate_trigger_object_reference(reference, event, target_count, scope)
        }
        PlayerRefDef::Target(_) => Err(unsupported_trigger_event(event)),
    }
}

fn validate_trigger_player_set(
    players: PlayerSetDef,
    event: TriggerEventDef,
    target_count: usize,
    scope: BindingScope,
) -> Result<(), GrantedAbilityValidationError> {
    match players {
        PlayerSetDef::All | PlayerSetDef::Related(_) => Ok(()),
        PlayerSetDef::LegalTargets(_) => Err(unsupported_trigger_event(event)),
        PlayerSetDef::One(reference) => {
            validate_trigger_player_reference(reference, event, target_count, scope)
        }
    }
}

fn validate_trigger_damage_matcher(
    matcher: DamageEventMatcherDef,
    event: TriggerEventDef,
    target_count: usize,
    scope: BindingScope,
) -> Result<(), GrantedAbilityValidationError> {
    match matcher.source {
        DamageSourceMatcherDef::Any | DamageSourceMatcherDef::Group(_) => {}
        // `AffectedObject` belongs to static prevention rules, whose applied
        // recipient is resolved outside an event. A triggered listener has no
        // such anchor and must name Source or another event reference.
        DamageSourceMatcherDef::AffectedObject => {
            return Err(unsupported_trigger_event(event));
        }
        DamageSourceMatcherDef::Matching(predicate) => {
            if trigger_predicate_requires_live_battlefield(predicate) {
                return Err(unsupported_trigger_event(event));
            }
            validate_trigger_object_predicate(predicate, event, target_count, scope)?;
        }
        DamageSourceMatcherDef::Object(reference) | DamageSourceMatcherDef::Except(reference) => {
            validate_trigger_object_reference(reference, event, target_count, scope)?;
        }
    }
    match matcher.recipient {
        DamageRecipientMatcherDef::Any | DamageRecipientMatcherDef::PlayerOrPlaneswalker => Ok(()),
        DamageRecipientMatcherDef::Recipients(EffectRecipientDef(
            EffectRecipientSetDef::Objects(ObjectSetDef::One(reference)),
        )) => validate_trigger_object_reference(reference, event, target_count, scope),
        DamageRecipientMatcherDef::Recipients(EffectRecipientDef(
            EffectRecipientSetDef::Players(players),
        )) => validate_trigger_player_set(players, event, target_count, scope),
        DamageRecipientMatcherDef::PlayerAndCreaturesControlledBy(player) => {
            validate_trigger_player_reference(player, event, target_count, scope)
        }
        DamageRecipientMatcherDef::AffectedObject | DamageRecipientMatcherDef::Recipients(_) => {
            Err(unsupported_trigger_event(event))
        }
    }
}

/// A range no declaration could satisfy: a clause that asks for none, or for
/// more than it will accept, is not a clause about attacking.
const fn declaration_range_is_empty(range: crate::card::AttackDeclarationRangeDef) -> bool {
    range.minimum == 0
        || match range.maximum {
            Some(maximum) => range.minimum > maximum,
            None => false,
        }
}

const COMMITTED_ZONE_TRANSITIONS: [(ZoneKind, ZoneKind); 9] = [
    (ZoneKind::Library, ZoneKind::Battlefield),
    (ZoneKind::Hand, ZoneKind::Battlefield),
    (ZoneKind::Graveyard, ZoneKind::Battlefield),
    (ZoneKind::Exile, ZoneKind::Battlefield),
    (ZoneKind::Stack, ZoneKind::Battlefield),
    (ZoneKind::Battlefield, ZoneKind::Graveyard),
    (ZoneKind::Battlefield, ZoneKind::Exile),
    (ZoneKind::Battlefield, ZoneKind::Hand),
    (ZoneKind::Battlefield, ZoneKind::Library),
];

fn validate_trigger_event_references(
    event: TriggerEventDef,
    target_count: usize,
    scope: BindingScope,
) -> Result<(), GrantedAbilityValidationError> {
    match event {
        // The ability is one ability, so every way into it has to be
        // independently valid.
        TriggerEventDef::AnyOf(events) => events
            .iter()
            .try_for_each(|event| validate_trigger_event_references(*event, target_count, scope)),
        TriggerEventDef::ZoneChanged(matcher) => {
            if !COMMITTED_ZONE_TRANSITIONS.iter().any(|(from, to)| {
                matcher.from.is_none_or(|expected| expected == *from)
                    && matcher.to.is_none_or(|expected| expected == *to)
            }) {
                return Err(unsupported_trigger_event(event));
            }
            let can_match_departure = COMMITTED_ZONE_TRANSITIONS.iter().any(|(from, to)| {
                *from == ZoneKind::Battlefield
                    && *to != ZoneKind::Battlefield
                    && matcher.from.is_none_or(|expected| expected == *from)
                    && matcher.to.is_none_or(|expected| expected == *to)
            });
            if can_match_departure && trigger_predicate_requires_live_battlefield(matcher.object) {
                return Err(unsupported_trigger_event(event));
            }
            validate_trigger_object_predicate(matcher.object, event, target_count, scope)?;
            if let Some(reference) = matcher.previously_damaged_by {
                if matcher
                    .from
                    .is_some_and(|from| from != ZoneKind::Battlefield)
                    || matcher.to.is_some_and(|to| to != ZoneKind::Graveyard)
                {
                    return Err(unsupported_trigger_event(event));
                }
                validate_trigger_object_reference(reference, event, target_count, scope)?;
            }
            Ok(())
        }
        TriggerEventDef::Tapped(matcher) => {
            validate_trigger_object_predicate(matcher.object, event, target_count, scope)
        }
        // The zones and the owner are printed constants, so there is no
        // reference in the clause to validate -- only that it names a zone
        // at all.
        TriggerEventDef::AttackDeclared {
            attacker,
            declaration,
        } => {
            if declaration_range_is_empty(declaration) {
                return Err(unsupported_trigger_event(event));
            }
            validate_trigger_object_predicate(attacker, event, target_count, scope)
        }
        TriggerEventDef::Attacks(matcher) => {
            if declaration_range_is_empty(matcher.declaration) || matcher.attack_number == Some(0) {
                return Err(unsupported_trigger_event(event));
            }
            validate_trigger_object_predicate(matcher.attacker, event, target_count, scope)
        }
        TriggerEventDef::SpellCast(predicate)
            if trigger_predicate_requires_live_battlefield(predicate) =>
        {
            Err(unsupported_trigger_event(event))
        }
        TriggerEventDef::AttacksAndIsNotBlocked {
            attacker: predicate,
        }
        | TriggerEventDef::BecomesBlocked(predicate)
        | TriggerEventDef::Blocks { blocked: predicate }
        | TriggerEventDef::BecomesBlockedBy { blocker: predicate }
        | TriggerEventDef::SpellCast(predicate)
        | TriggerEventDef::BecomesTargetOfSpell(predicate)
        | TriggerEventDef::BecomesTargetOfSpellOrAbility(predicate)
        | TriggerEventDef::CountersPlaced {
            object: predicate, ..
        }
        | TriggerEventDef::Transforms(predicate) => {
            validate_trigger_object_predicate(predicate, event, target_count, scope)
        }
        TriggerEventDef::BlocksOrBecomesBlockedBy { creature, other } => {
            validate_trigger_object_predicate(creature, event, target_count, scope)?;
            validate_trigger_object_predicate(other, event, target_count, scope)
        }
        TriggerEventDef::DamageDealt(matcher) => {
            validate_trigger_damage_matcher(matcher, event, target_count, scope)
        }
        // A clause that names no zone at all is not a clause about exiling.
        TriggerEventDef::LifeGained(PlayerRelation::ChosenPlayer)
        | TriggerEventDef::CardsExiled { zones: &[], .. } => Err(unsupported_trigger_event(event)),
        TriggerEventDef::CommittedCrime(_)
        | TriggerEventDef::BecomesLevel(_)
        | TriggerEventDef::Cycled
        | TriggerEventDef::DoorUnlocked
        | TriggerEventDef::StepBegins { .. }
        | TriggerEventDef::LifeGained(_)
        | TriggerEventDef::BecomesMonarch(_)
        | TriggerEventDef::DrewCard(_)
        | TriggerEventDef::Discarded(_)
        | TriggerEventDef::CardsExiled { .. }
        | TriggerEventDef::StateCondition => Ok(()),
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
        TriggerConditionDef::All(conditions) => conditions
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
        | TriggerConditionDef::ControlsGreatestPowerCreature
        | TriggerConditionDef::SourceMatches { .. }
        | TriggerConditionDef::AttachedPermanentMatches { .. }
        | TriggerConditionDef::SourceCounters { .. }
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
        | TriggerConditionDef::ControllerLifeAtMostHalfStartingLife => Ok(()),
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
        | ValueDef::GreatestPowerAmong(query) => {
            validate_query(*query, target_count, scope)
        }
        ValueDef::TargetPower(target)
        | ValueDef::TargetToughness(target)
        | ValueDef::TargetLibrarySize(target)
        | ValueDef::TargetManaValue(target) => validate_target_index(target, target_count),
        ValueDef::Constant(_)
        | ValueDef::ChosenX
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
        | ValueDef::DevotionTo(_)
        | ValueDef::LibrarySize(_)
        | ValueDef::ColorsOfManaSpent
        | ValueDef::PaidAmount
        | ValueDef::MatchedCount
        | ValueDef::MatchedCardTypes
        | ValueDef::BoundObjectCount(_)
        | ValueDef::SpellsCastBeforeThisTurn
        | ValueDef::CreaturesDiedThisTurn
        | ValueDef::CardTypesAmongGraveyards(_)
        | ValueDef::IfCardTypesAmongGraveyards(_)
        // This reads the share assigned to the target currently being
        // affected; the surrounding recipient carries the slot reference.
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
            | AppliedRuleDef::RedirectDamageFromTo { .. },
        ) => {
            if matches!(recipient.0, EffectRecipientSetDef::Objects(_)) {
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
