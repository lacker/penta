fn unsupported_trigger_event(event: TriggerEventDef) -> GrantedAbilityValidationError {
    GrantedAbilityValidationError::UnsupportedTriggerEvent { event }
}

/// The values a stat comparison in a trigger predicate may read. A list
/// rather than part of the walk above, which is a match over predicates.
fn trigger_stat_value_is_supported(value: ValueDef) -> bool {
    matches!(
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
                    | ValueDef::LifeTotal(_)
                    | ValueDef::SourceToughness
                    | ValueDef::CountersOnSource(_)
                    | ValueDef::CardsDrawnThisTurn(_)
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
    )
}

fn validate_trigger_object_predicate(
    predicate: ObjectPredicateDef,
    event: TriggerEventDef,
    target_count: usize,
    scope: BindingScope<'_>,
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
            if trigger_stat_value_is_supported(value) {
                Ok(())
            } else {
                Err(unsupported_trigger_event(event))
            }
        }
        ObjectPredicateDef::DeclaredTargetCount { minimum, maximum } => {
            if minimum <= maximum {
                Ok(())
            } else {
                Err(unsupported_trigger_event(event))
            }
        }
        ObjectPredicateDef::HasAbility(_)
        | ObjectPredicateDef::Ability
        | ObjectPredicateDef::ActivatedAbility
        | ObjectPredicateDef::TriggeredAbility
        | ObjectPredicateDef::HasDeclaredTarget(_)
        | ObjectPredicateDef::HasDeclaredPlayerTarget(_)
        | ObjectPredicateDef::ControlledBy(
            PlayerRelation::ChosenPlayer | PlayerRelation::EventPlayer,
        )
        // A trigger snapshot carries mana value, not the printed cost, so
        // the cost-shape reading has nothing to read here.
        | ObjectPredicateDef::GenericManaCostAtMost(_)
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
        | ObjectPredicateDef::HasAnyCounter
            | ObjectPredicateDef::CounterCount { .. }
        | ObjectPredicateDef::ControlledBy(_)
        | ObjectPredicateDef::OwnedBy(_)
        | ObjectPredicateDef::Supertype(_)
        | ObjectPredicateDef::DebutSet(_)
        | ObjectPredicateDef::HasName(_)
        | ObjectPredicateDef::HasSourcesChosenScalar(_)
        | ObjectPredicateDef::TargetsObjectMatching(_)
        | ObjectPredicateDef::AttackingOrBlocking
        | ObjectPredicateDef::HasKeyword(_)
        | ObjectPredicateDef::HasNonManaActivatedAbility
        | ObjectPredicateDef::Attacking
        | ObjectPredicateDef::Saddled
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
        | ObjectPredicateDef::EnteredThisTurn
        | ObjectPredicateDef::ToughnessGreaterThanItsPower
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
        | ObjectPredicateDef::Ability
        | ObjectPredicateDef::ActivatedAbility
        | ObjectPredicateDef::TriggeredAbility
        | ObjectPredicateDef::DeclaredTargetCount { .. }
        | ObjectPredicateDef::HasDeclaredTarget(_)
        | ObjectPredicateDef::HasDeclaredPlayerTarget(_)
        | ObjectPredicateDef::NoncreatureSpell
        | ObjectPredicateDef::Color(_)
        | ObjectPredicateDef::ColorCount(_)
        | ObjectPredicateDef::Subtype(_)
        | ObjectPredicateDef::Named(_)
        | ObjectPredicateDef::HasChosenName
        | ObjectPredicateDef::ManaValueAtMost(_)
        | ObjectPredicateDef::GenericManaCostAtMost(_)
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
        | ObjectPredicateDef::ToughnessGreaterThanItsPower
        | ObjectPredicateDef::HasCounter(_)
        | ObjectPredicateDef::HasAnyCounter
            | ObjectPredicateDef::CounterCount { .. }
        | ObjectPredicateDef::ControlledBy(_)
        | ObjectPredicateDef::OwnedBy(_)
        | ObjectPredicateDef::Supertype(_)
        | ObjectPredicateDef::DebutSet(_)
        | ObjectPredicateDef::HasName(_)
        | ObjectPredicateDef::HasSourcesChosenScalar(_)
        | ObjectPredicateDef::TargetsObjectMatching(_)
        | ObjectPredicateDef::AttackingOrBlocking
        | ObjectPredicateDef::HasKeyword(_)
        | ObjectPredicateDef::HasAbility(_)
        | ObjectPredicateDef::AttachedToSource
        | ObjectPredicateDef::Attacking
        | ObjectPredicateDef::Saddled
        | ObjectPredicateDef::Blocking
        | ObjectPredicateDef::BlockedBySource
        | ObjectPredicateDef::BlockingSource
        | ObjectPredicateDef::BandedWithSource
        | ObjectPredicateDef::Unpaired
        | ObjectPredicateDef::PairedWithSource
        | ObjectPredicateDef::Enchanted
        | ObjectPredicateDef::AttackedThisTurn
        | ObjectPredicateDef::CameUnderControlThisTurn
        | ObjectPredicateDef::EnteredThisTurn
        | ObjectPredicateDef::AttackedDuringControllersLastTurn
        | ObjectPredicateDef::Special(_) => false,
    }
}

fn validate_trigger_object_reference(
    reference: ObjectRefDef,
    event: TriggerEventDef,
    target_count: usize,
    scope: BindingScope<'_>,
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
    scope: BindingScope<'_>,
) -> Result<(), GrantedAbilityValidationError> {
    validate_player_reference(reference, target_count, scope)?;
    match reference {
        PlayerRefDef::EffectController
        | PlayerRefDef::EnchantedPlayer
        | PlayerRefDef::EventPlayer
        | PlayerRefDef::Opponent => Ok(()),
        PlayerRefDef::ControllerOf(reference)
        | PlayerRefDef::OpponentOf(reference)
        | PlayerRefDef::OwnerOf(reference) => {
            validate_trigger_object_reference(reference, event, target_count, scope)
        }
        PlayerRefDef::Target(_) => Err(unsupported_trigger_event(event)),
    }
}

fn validate_trigger_player_set(
    players: PlayerSetDef,
    event: TriggerEventDef,
    target_count: usize,
    scope: BindingScope<'_>,
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
    scope: BindingScope<'_>,
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
        // The same bar the source side clears: the damaged object is read
        // from its snapshot, so a predicate that needs it still standing on
        // the battlefield cannot answer.
        DamageRecipientMatcherDef::MatchingObject(predicate) => {
            if trigger_predicate_requires_live_battlefield(predicate) {
                return Err(unsupported_trigger_event(event));
            }
            validate_trigger_object_predicate(predicate, event, target_count, scope)
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

const COMMITTED_ZONE_TRANSITIONS: [(ZoneKind, ZoneKind); 12] = [
    (ZoneKind::Library, ZoneKind::Battlefield),
    (ZoneKind::Hand, ZoneKind::Battlefield),
    (ZoneKind::Graveyard, ZoneKind::Battlefield),
    (ZoneKind::Exile, ZoneKind::Battlefield),
    (ZoneKind::Stack, ZoneKind::Battlefield),
    (ZoneKind::Library, ZoneKind::Graveyard),
    (ZoneKind::Hand, ZoneKind::Graveyard),
    (ZoneKind::Exile, ZoneKind::Graveyard),
    (ZoneKind::Battlefield, ZoneKind::Graveyard),
    (ZoneKind::Battlefield, ZoneKind::Exile),
    (ZoneKind::Battlefield, ZoneKind::Hand),
    (ZoneKind::Battlefield, ZoneKind::Library),
];

/// The zone-change arm of the walk next door, which is long enough to read
/// on its own: which transitions are actually published, whether the
/// predicate can survive the departure it matches, and the optional "that
/// was dealt damage by" reference.
fn validate_zone_change_references(
    event: TriggerEventDef,
    matcher: ZoneChangeEventMatcherDef,
    target_count: usize,
    scope: BindingScope<'_>,
) -> Result<(), GrantedAbilityValidationError> {
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

fn validate_trigger_event_references(
    event: TriggerEventDef,
    target_count: usize,
    scope: BindingScope<'_>,
) -> Result<(), GrantedAbilityValidationError> {
    match event {
        // The ability is one ability, so every way into it has to be
        // independently valid.
        TriggerEventDef::AnyOf(events) => events
            .iter()
            .try_for_each(|event| validate_trigger_event_references(*event, target_count, scope)),
        // The condition is validated where every trigger condition is; what
        // this walk asks is only about the event it wraps.
        TriggerEventDef::While { event, .. } => {
            validate_trigger_event_references(*event, target_count, scope)
        }
        TriggerEventDef::ZoneChanged(matcher) => {
            validate_zone_change_references(event, matcher, target_count, scope)
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
        // Each names one object: the creature exerted, the source of the
        // "you may" that was accepted, the source of the clause that
        // sacrificed something, or the permanent sacrificed.
        TriggerEventDef::Exerted(object)
        | TriggerEventDef::OptionalEffectTaken(object)
        | TriggerEventDef::SacrificePerformed(object)
        | TriggerEventDef::Sacrificed { object, .. } => {
            validate_trigger_object_predicate(object, event, target_count, scope)
        }
        TriggerEventDef::Attacks(matcher) => {
            if declaration_range_is_empty(matcher.declaration) || matcher.attack_number == Some(0) {
                return Err(unsupported_trigger_event(event));
            }
            validate_trigger_object_predicate(matcher.attacker, event, target_count, scope)
        }
        TriggerEventDef::StackObject(matcher)
            if trigger_predicate_requires_live_battlefield(matcher.object) =>
        {
            Err(unsupported_trigger_event(event))
        }
        TriggerEventDef::StackObject(matcher) => {
            validate_trigger_object_predicate(matcher.object, event, target_count, scope)?;
            if let StackObjectEventDef::TargetSelection { target, .. } = matcher.event {
                validate_stack_target_filter(target, event, target_count, scope)?;
            }
            Ok(())
        }
        // The batch is read from last-known information, so a predicate
        // that needs the object still standing on the battlefield cannot
        // answer it -- the same bar the battlefield-to-graveyard zone change
        // beside it has to clear.
        TriggerEventDef::ObjectsDied { object }
            if trigger_predicate_requires_live_battlefield(object) =>
        {
            Err(unsupported_trigger_event(event))
        }
        TriggerEventDef::ObjectsDied { object: predicate }
        | TriggerEventDef::TokensCreated {
            token: predicate, ..
        }
        | TriggerEventDef::AttacksAndIsNotBlocked {
            attacker: predicate,
        }
        | TriggerEventDef::UnblockedAttackersDeclared {
            attacker: predicate,
            ..
        }
        | TriggerEventDef::CombatDamageDealtToPlayers {
            sources: predicate, ..
        }
        | TriggerEventDef::BecomesBlocked(predicate)
        | TriggerEventDef::Blocks { blocked: predicate }
        | TriggerEventDef::BecomesBlockedBy { blocker: predicate }
        | TriggerEventDef::CountersPlaced {
            object: predicate, ..
        }
        | TriggerEventDef::CountersRemoved { object: predicate, .. }
        | TriggerEventDef::LastCounterRemoved { object: predicate, .. }
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
        | TriggerEventDef::CumulativeUpkeepNotPaid
        | TriggerEventDef::BecomesLevel(_)
        | TriggerEventDef::Cycled
        | TriggerEventDef::DoorUnlocked
        | TriggerEventDef::StepBegins { .. }
        | TriggerEventDef::LandPlayed { .. }
        | TriggerEventDef::LifeGained(_)
        | TriggerEventDef::BecomesMonarch(_)
        | TriggerEventDef::DrewCard(_)
        | TriggerEventDef::Discarded(_)
        | TriggerEventDef::DiscardedCards(_)
        | TriggerEventDef::CardsExiled { .. }
        | TriggerEventDef::StateCondition => Ok(()),
    }
}

fn validate_stack_target_filter(
    filter: StackTargetFilterDef,
    event: TriggerEventDef,
    target_count: usize,
    scope: BindingScope<'_>,
) -> Result<(), GrantedAbilityValidationError> {
    match filter {
        StackTargetFilterDef::Player(_) => Ok(()),
        StackTargetFilterDef::Permanent(predicate)
        | StackTargetFilterDef::Card(predicate)
        | StackTargetFilterDef::Spell(predicate) => {
            validate_trigger_object_predicate(predicate, event, target_count, scope)
        }
        StackTargetFilterDef::AnyOf(filters) => filters
            .iter()
            .try_for_each(|filter| validate_stack_target_filter(*filter, event, target_count, scope)),
    }
}
