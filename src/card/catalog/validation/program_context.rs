use crate::card::{
    AbilityDef, AbilityOperationDef, AppliedEffectDef, AppliedRuleDef, CardType,
    CharacteristicOperationDef, DamageEventMatcherDef, DamageRecipientMatcherDef,
    DamageSourceMatcherDef, DeclarativeAbilityDef, EffectDef, EffectRecipientDef,
    EffectRecipientSetDef, ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef,
    PlayerRefDef, PlayerRelation, PlayerSetDef, PowerToughnessOperationDef, SetOperationDef,
    TriggerConditionDef, ValueDef, ZoneKind,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct EffectProgramContextError {
    pub context: &'static str,
    pub operation: &'static str,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum StaticPosition {
    Root,
    Traversed,
}

/// Checks the boundary between programs that resolve and programs that are
/// derived live from a static source. The two runtimes intentionally share
/// `EffectDef`, so the catalog has to reject leaves that the selected runtime
/// would otherwise ignore.
pub(super) fn validate_ability_effect_context(
    ability: &AbilityDef,
) -> Result<(), EffectProgramContextError> {
    let Some(effect) = ability.declarative_effect() else {
        // Metadata-only, custom, card-owned, and replacement programs do not
        // execute this ordinary effect tree through either runtime.
        return Ok(());
    };
    match ability.definition {
        DeclarativeAbilityDef::Static(definition) => validate_static_effect(
            effect,
            definition.source_zones,
            StaticPosition::Root,
            ability.coverage.explanation.is_some(),
        )
        .map_err(|operation| EffectProgramContextError {
            context: "static",
            operation,
        }),
        DeclarativeAbilityDef::Replacement(_) => Ok(()),
        DeclarativeAbilityDef::Spell(_)
        | DeclarativeAbilityDef::ActivatedMana(_)
        | DeclarativeAbilityDef::TriggeredMana(_)
        | DeclarativeAbilityDef::Activated(_)
        | DeclarativeAbilityDef::Triggered(_)
        | DeclarativeAbilityDef::AlternativeCast(_)
        | DeclarativeAbilityDef::SpecialAction(_)
        | DeclarativeAbilityDef::Keyword(_)
        | DeclarativeAbilityDef::Legacy => {
            validate_resolving_effect(effect, resolving_source_zones(ability)).map_err(
                |operation| EffectProgramContextError {
                    context: "resolving",
                    operation,
                },
            )
        }
    }
}

fn resolving_source_zones(ability: &AbilityDef) -> &'static [ZoneKind] {
    match ability.definition {
        DeclarativeAbilityDef::ActivatedMana(definition)
        | DeclarativeAbilityDef::Activated(definition) => definition.source_zones,
        DeclarativeAbilityDef::TriggeredMana(definition)
        | DeclarativeAbilityDef::Triggered(definition) => definition.source_zones,
        DeclarativeAbilityDef::SpecialAction(definition) => definition.source_zones,
        DeclarativeAbilityDef::Spell(_)
        | DeclarativeAbilityDef::AlternativeCast(_)
        | DeclarativeAbilityDef::Keyword(_)
        | DeclarativeAbilityDef::Legacy => &[ZoneKind::Stack],
        DeclarativeAbilityDef::Static(_) | DeclarativeAbilityDef::Replacement(_) => &[],
    }
}

fn validate_static_effect(
    effect: EffectDef,
    source_zones: &[ZoneKind],
    position: StaticPosition,
    has_external_enforcement_explanation: bool,
) -> Result<(), &'static str> {
    match effect {
        EffectDef::None
            if position == StaticPosition::Root && has_external_enforcement_explanation =>
        {
            Ok(())
        }
        EffectDef::Sequence(effects) => {
            if effects.is_empty() {
                return Err("empty Sequence");
            }
            for effect in effects {
                validate_static_effect(
                    *effect,
                    source_zones,
                    StaticPosition::Traversed,
                    has_external_enforcement_explanation,
                )?;
            }
            Ok(())
        }
        EffectDef::IfCondition { condition, then }
            if source_zones == [ZoneKind::Battlefield]
                && static_trigger_condition_supported(*condition) =>
        {
            validate_static_effect(
                *then,
                source_zones,
                StaticPosition::Traversed,
                has_external_enforcement_explanation,
            )
        }
        EffectDef::StaticApply { recipient, effect } => {
            validate_static_apply(source_zones, recipient, effect)
        }
        EffectDef::CannotBeForcedToSacrifice | EffectDef::LandwalkCanBeBlocked(_)
            if position == StaticPosition::Root && source_zones == [ZoneKind::Battlefield] =>
        {
            Ok(())
        }
        EffectDef::CannotAttackUnless(query)
            if position == StaticPosition::Root
                && source_zones == [ZoneKind::Battlefield]
                && query.zones == [ZoneKind::Battlefield]
                && static_query_supported(*query) =>
        {
            Ok(())
        }
        EffectDef::ReduceGenericCostBy(value)
            if position == StaticPosition::Root
                && source_zones == [ZoneKind::Hand]
                && static_cost_reduction_value_supported(value) =>
        {
            Ok(())
        }
        _ => Err(effect_operation_name(effect)),
    }
}

fn validate_static_apply(
    source_zones: &[ZoneKind],
    recipient: EffectRecipientDef,
    effect: AppliedEffectDef,
) -> Result<(), &'static str> {
    if source_zones == [ZoneKind::Stack] {
        return if recipient == EffectRecipientDef::Source
            && stack_static_applied_effect_supported(effect)
        {
            Ok(())
        } else {
            Err("StaticApply outside its supported stack-source shape")
        };
    }
    if source_zones != [ZoneKind::Battlefield] {
        return Err("StaticApply from unsupported source zones");
    }
    match recipient.0 {
        EffectRecipientSetDef::Players(players) => {
            if !static_player_set_supported(players) {
                return Err("StaticApply with an unavailable static player recipient");
            }
            if static_player_applied_effect_supported(effect) {
                Ok(())
            } else {
                Err("StaticApply with an unsupported player-facing effect")
            }
        }
        EffectRecipientSetDef::Objects(objects) => {
            if !static_object_set_supported(objects) {
                return Err("StaticApply with an unavailable static object recipient");
            }
            if static_object_applied_effect_supported(recipient, effect) {
                Ok(())
            } else {
                Err("StaticApply with an unsupported object-facing effect")
            }
        }
        EffectRecipientSetDef::LegalTargets(_) => Err("StaticApply with a target-scoped recipient"),
    }
}

fn stack_static_applied_effect_supported(effect: AppliedEffectDef) -> bool {
    match effect {
        AppliedEffectDef::Composite(effects) => {
            !effects.is_empty()
                && effects
                    .iter()
                    .copied()
                    .all(stack_static_applied_effect_supported)
        }
        AppliedEffectDef::Rule(AppliedRuleDef::CannotBeCountered) => true,
        AppliedEffectDef::Characteristic(_) | AppliedEffectDef::Rule(_) => false,
    }
}

fn static_player_applied_effect_supported(effect: AppliedEffectDef) -> bool {
    match effect {
        AppliedEffectDef::Composite(effects) => {
            !effects.is_empty()
                && effects
                    .iter()
                    .copied()
                    .all(static_player_applied_effect_supported)
        }
        AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(restriction)) => {
            static_object_predicate_supported(restriction.object)
        }
        AppliedEffectDef::Characteristic(_) | AppliedEffectDef::Rule(_) => false,
    }
}

fn static_object_applied_effect_supported(
    recipient: EffectRecipientDef,
    effect: AppliedEffectDef,
) -> bool {
    match effect {
        AppliedEffectDef::Composite(effects) => {
            !effects.is_empty()
                && effects
                    .iter()
                    .copied()
                    .all(|effect| static_object_applied_effect_supported(recipient, effect))
        }
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(_))
        | AppliedEffectDef::Rule(
            AppliedRuleDef::CannotBeEnchanted
            | AppliedRuleDef::CannotBecomeEnchanted
            | AppliedRuleDef::CannotAttack
            | AppliedRuleDef::CannotBeBlocked
            | AppliedRuleDef::CannotBlock
            | AppliedRuleDef::CannotChangeController
            | AppliedRuleDef::CannotRegenerate
            | AppliedRuleDef::DoesNotUntapDuringUntapStep
            | AppliedRuleDef::MayChooseNotToUntap
            | AppliedRuleDef::RemainsAttachedThroughProtection,
        ) => true,
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::BasicLandTypes(operation)) => {
            match operation {
                SetOperationDef::Add(types)
                | SetOperationDef::Remove(types)
                | SetOperationDef::Set(types) => !types.is_empty(),
            }
        }
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
            PowerToughnessOperationDef::SetBase { power, toughness }
            | PowerToughnessOperationDef::Modify { power, toughness },
        )) => {
            static_power_toughness_value_supported(power)
                && static_power_toughness_value_supported(toughness)
        }
        // Static animation is deliberately narrower than resolving
        // characteristic changes: it may add the creature card type, may
        // repaint color, and must use a query that cannot read anything those
        // operations supply. Static subtype changes remain outside this
        // stratified walk.
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::CardTypes(
            SetOperationDef::Add(types),
        )) => types.contains(CardType::Creature) && static_animation_query_supported(recipient),
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::Colors(
            SetOperationDef::Set(_),
        )) => static_animation_query_supported(recipient),
        AppliedEffectDef::Characteristic(
            CharacteristicOperationDef::CardTypes(
                SetOperationDef::Remove(_) | SetOperationDef::Set(_),
            )
            | CharacteristicOperationDef::Colors(
                SetOperationDef::Add(_) | SetOperationDef::Remove(_),
            )
            | CharacteristicOperationDef::CreatureTypes(_),
        )
        | AppliedEffectDef::Rule(
            AppliedRuleDef::CannotBeCountered
            | AppliedRuleDef::CannotPlay(_)
            | AppliedRuleDef::RedirectDamageFromTo { .. },
        ) => false,
        AppliedEffectDef::Rule(
            AppliedRuleDef::CannotBeBlockedBy(predicate) | AppliedRuleDef::CanBlockOnly(predicate),
        ) => static_object_predicate_supported(predicate),
        AppliedEffectDef::Rule(AppliedRuleDef::PreventDamage(matcher)) => {
            static_damage_matcher_supported(matcher)
        }
        AppliedEffectDef::Rule(AppliedRuleDef::RedirectPlayerDamageToThis(_)) => {
            matches!(
                recipient.object_reference(),
                Some(ObjectRefDef::Source | ObjectRefDef::AttachedToSource)
            ) || recipient.object_query().is_some()
        }
    }
}

#[allow(clippy::too_many_lines)]
fn validate_resolving_effect(
    effect: EffectDef,
    source_zones: &[ZoneKind],
) -> Result<(), &'static str> {
    match effect {
        EffectDef::Sequence(effects) => {
            if effects.is_empty() {
                return Err("empty Sequence");
            }
            for effect in effects {
                validate_resolving_effect(*effect, source_zones)?;
            }
            Ok(())
        }
        EffectDef::Randomized {
            on_success,
            on_failure,
            ..
        } => {
            validate_resolving_effect(*on_success, source_zones)?;
            validate_resolving_effect(*on_failure, source_zones)
        }
        EffectDef::Choose(choice) => validate_resolving_effect(*choice.then, source_zones),
        EffectDef::PayOr(payment) => {
            for effect in payment.if_paid.iter().chain(payment.otherwise.iter()) {
                validate_resolving_effect(**effect, source_zones)?;
            }
            Ok(())
        }
        EffectDef::SplitIntoPiles(partition) => {
            validate_resolving_effect(*partition.then, source_zones)
        }
        EffectDef::May { effect, .. }
        | EffectDef::ReplaceNextDrawThisTurn { effect, .. }
        | EffectDef::IfCondition { then: effect, .. } => {
            validate_resolving_effect(*effect, source_zones)
        }
        EffectDef::SacrificeOfChoice {
            then: Some(effect), ..
        } => validate_resolving_effect(*effect, source_zones),
        EffectDef::LookAtTopAndSelect { selection, .. } => {
            if let Some(effect) = selection.then {
                validate_resolving_effect(*effect, source_zones)?;
            }
            Ok(())
        }
        EffectDef::IfFormat {
            then, otherwise, ..
        } => {
            validate_resolving_effect(*then, source_zones)?;
            validate_resolving_effect(*otherwise, source_zones)
        }
        EffectDef::InstallTrigger(trigger) => {
            let Some(effect) = trigger.ability.declarative_effect() else {
                return Err("InstallTrigger with a non-declarative program");
            };
            validate_resolving_effect(effect, source_zones)
        }
        EffectDef::Apply {
            recipient, effect, ..
        } if recipient == EffectRecipientDef::Source
            && applied_effect_adds_ability(effect)
            && source_zones != [ZoneKind::Battlefield] =>
        {
            Err("Apply grants an ability to a nonbattlefield source")
        }
        EffectDef::StaticApply { .. }
        | EffectDef::CannotBeForcedToSacrifice
        | EffectDef::ReduceGenericCostBy(_)
        | EffectDef::CannotAttackUnless(_)
        | EffectDef::LandwalkCanBeBlocked(_) => Err(effect_operation_name(effect)),
        EffectDef::None
        | EffectDef::PreventDamage { .. }
        | EffectDef::AddMana(_)
        | EffectDef::AddPoisonCounters { .. }
        | EffectDef::DealDamage { .. }
        | EffectDef::GainLife { .. }
        | EffectDef::DrawCards { .. }
        | EffectDef::ShuffleLibrary { .. }
        | EffectDef::EmptyManaPool { .. }
        | EffectDef::Discard { .. }
        | EffectDef::DiscardCards { .. }
        | EffectDef::LoseLife { .. }
        | EffectDef::LoseTheGame { .. }
        | EffectDef::Tap { .. }
        | EffectDef::RemoveFromCombat { .. }
        | EffectDef::Untap { .. }
        | EffectDef::CreateToken { .. }
        | EffectDef::CreateAttachedToken { .. }
        | EffectDef::CreateTokenCopyOf { .. }
        | EffectDef::Attach { .. }
        | EffectDef::Reconfigure { .. }
        | EffectDef::Destroy { .. }
        | EffectDef::DestroyAtEndOfCombat { .. }
        | EffectDef::Detain { .. }
        | EffectDef::Regenerate { .. }
        | EffectDef::Sacrifice { .. }
        | EffectDef::SacrificeOfChoice { then: None, .. }
        | EffectDef::Mill { .. }
        | EffectDef::LookAtHand { .. }
        | EffectDef::SearchZone { .. }
        | EffectDef::ChooseCards { .. }
        | EffectDef::Counter { .. }
        | EffectDef::DrainLife { .. }
        | EffectDef::AddManaEqualTo { .. }
        | EffectDef::AddCounters { .. }
        | EffectDef::RemoveAllCounters { .. }
        | EffectDef::SkipNextUntapSteps { .. }
        | EffectDef::ChangeTextBasicLandType { .. }
        | EffectDef::BecomeCopyOf { .. }
        | EffectDef::GrantFlashToNextSorcery
        | EffectDef::ExileLinkedToSource { .. }
        | EffectDef::ReturnLinkedExiles { .. }
        | EffectDef::GainControl { .. }
        | EffectDef::ScheduleTurnPhases(_)
        | EffectDef::TakeExtraTurn { .. }
        | EffectDef::CreateEmblem { .. }
        | EffectDef::Transform { .. }
        | EffectDef::MoveToZone { .. }
        | EffectDef::Apply { .. }
        | EffectDef::Special(_) => Ok(()),
    }
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

fn static_player_set_supported(players: PlayerSetDef) -> bool {
    match players {
        PlayerSetDef::All | PlayerSetDef::One(PlayerRefDef::EffectController) => true,
        PlayerSetDef::Related(relation) => static_player_relation_supported(relation),
        PlayerSetDef::LegalTargets(_)
        | PlayerSetDef::One(
            PlayerRefDef::EventPlayer
            | PlayerRefDef::Target(_)
            | PlayerRefDef::ControllerOf(_)
            | PlayerRefDef::OwnerOf(_),
        ) => false,
    }
}

fn static_player_relation_supported(relation: PlayerRelation) -> bool {
    matches!(
        relation,
        PlayerRelation::Any
            | PlayerRelation::You
            | PlayerRelation::NotYou
            | PlayerRelation::Opponent
            | PlayerRelation::ActivePlayer
            | PlayerRelation::NonactivePlayer
    )
}

fn static_object_set_supported(objects: ObjectSetDef) -> bool {
    match objects {
        ObjectSetDef::One(ObjectRefDef::Source | ObjectRefDef::AttachedToSource) => true,
        ObjectSetDef::Query(query) => {
            query.zones == [ZoneKind::Battlefield] && static_query_supported(query)
        }
        ObjectSetDef::LegalTargets(_)
        | ObjectSetDef::One(
            ObjectRefDef::ResolvingObject
            | ObjectRefDef::Binding(_)
            | ObjectRefDef::Target(_)
            | ObjectRefDef::TriggeringObject,
        )
        | ObjectSetDef::Binding(_)
        | ObjectSetDef::SharingNameWith(_) => false,
    }
}

fn static_query_supported(query: ObjectQueryDef) -> bool {
    !query.zones.is_empty()
        && [query.related_player, query.controller, query.owner]
            .into_iter()
            .flatten()
            .all(static_player_set_supported)
        && static_object_predicate_supported(query.object)
}

fn static_animation_query_supported(recipient: EffectRecipientDef) -> bool {
    recipient.object_query().is_some_and(|query| {
        query.zones == [ZoneKind::Battlefield]
            && static_query_supported(query)
            && static_animation_predicate_supported(query.object)
    })
}

fn static_animation_predicate_supported(predicate: ObjectPredicateDef) -> bool {
    match predicate {
        ObjectPredicateDef::Any
        | ObjectPredicateDef::HasAnyBasicLandType(_)
        | ObjectPredicateDef::HasType(CardType::Land) => true,
        ObjectPredicateDef::All(predicates) | ObjectPredicateDef::AnyOf(predicates) => predicates
            .iter()
            .copied()
            .all(static_animation_predicate_supported),
        ObjectPredicateDef::Not(predicate) => static_animation_predicate_supported(*predicate),
        _ => false,
    }
}

fn static_object_predicate_supported(predicate: ObjectPredicateDef) -> bool {
    match predicate {
        ObjectPredicateDef::All(predicates) | ObjectPredicateDef::AnyOf(predicates) => predicates
            .iter()
            .copied()
            .all(static_object_predicate_supported),
        ObjectPredicateDef::Not(predicate) | ObjectPredicateDef::AttachedTo(predicate) => {
            static_object_predicate_supported(*predicate)
        }
        ObjectPredicateDef::ControlledBy(relation) => static_player_relation_supported(relation),
        ObjectPredicateDef::ManaValueEqualTo(value)
        | ObjectPredicateDef::ManaValueAtMostValue(value)
        | ObjectPredicateDef::ToughnessLessThan(value)
        | ObjectPredicateDef::PowerGreaterThan(value)
        | ObjectPredicateDef::ToughnessGreaterThan(value)
        | ObjectPredicateDef::PowerLessThan(value) => static_source_value_supported(value),
        ObjectPredicateDef::Special(_) => false,
        ObjectPredicateDef::Any
        | ObjectPredicateDef::Source
        | ObjectPredicateDef::Token
        | ObjectPredicateDef::Tapped
        | ObjectPredicateDef::HasType(_)
        | ObjectPredicateDef::HasAnyBasicLandType(_)
        | ObjectPredicateDef::Spell
        | ObjectPredicateDef::NoncreatureSpell
        | ObjectPredicateDef::Color(_)
        | ObjectPredicateDef::ColorCount(_)
        | ObjectPredicateDef::Subtype(_)
        | ObjectPredicateDef::ManaValueAtMost(_)
        | ObjectPredicateDef::PowerAtLeast(_)
        | ObjectPredicateDef::PowerExactly(_)
        | ObjectPredicateDef::ToughnessExactly(_)
        | ObjectPredicateDef::HasCounter(_)
        | ObjectPredicateDef::Supertype(_)
        | ObjectPredicateDef::DebutSet(_)
        | ObjectPredicateDef::SharesNameWithSource
        | ObjectPredicateDef::AttackingOrBlocking
        | ObjectPredicateDef::HasKeyword(_)
        | ObjectPredicateDef::HasNonManaActivatedAbility
        | ObjectPredicateDef::Attacking
        | ObjectPredicateDef::AttachedToSource
        | ObjectPredicateDef::Blocking
        | ObjectPredicateDef::BlockedBySource
        | ObjectPredicateDef::Enchanted
        | ObjectPredicateDef::AttackedThisTurn => true,
    }
}

fn static_source_value_supported(value: ValueDef) -> bool {
    matches!(
        value,
        ValueDef::Constant(_) | ValueDef::SourcePower | ValueDef::CountersOnSource(_)
    )
}

fn static_power_toughness_value_supported(value: ValueDef) -> bool {
    match value {
        ValueDef::Constant(_) => true,
        ValueDef::CountMatchingObjects(query) | ValueDef::AnyMatchingObject(query) => {
            static_query_supported(*query)
        }
        ValueDef::Scaled(scaled) => static_power_toughness_value_supported(scaled.value),
        ValueDef::ChosenX
        | ValueDef::SourcePower
        | ValueDef::SourceToughness
        | ValueDef::TriggeringObjectPower
        | ValueDef::TriggerEventAmount
        | ValueDef::CardsInHandAbove { .. }
        | ValueDef::Negate(_)
        | ValueDef::IfCreatureDiedThisTurn(_)
        | ValueDef::IfTargetMatches(_)
        | ValueDef::IfMatchingObjectCount(_)
        | ValueDef::CountersOnSource(_)
        | ValueDef::TargetPower(_)
        | ValueDef::TargetManaValue(_)
        | ValueDef::DividedAmongTargets => false,
    }
}

fn static_cost_reduction_value_supported(value: ValueDef) -> bool {
    match value {
        ValueDef::Constant(_) => true,
        ValueDef::CountMatchingObjects(query) => static_query_supported(*query),
        ValueDef::ChosenX
        | ValueDef::SourcePower
        | ValueDef::SourceToughness
        | ValueDef::TriggeringObjectPower
        | ValueDef::TriggerEventAmount
        | ValueDef::CardsInHandAbove { .. }
        | ValueDef::AnyMatchingObject(_)
        | ValueDef::Negate(_)
        | ValueDef::Scaled(_)
        | ValueDef::IfCreatureDiedThisTurn(_)
        | ValueDef::IfTargetMatches(_)
        | ValueDef::IfMatchingObjectCount(_)
        | ValueDef::CountersOnSource(_)
        | ValueDef::TargetPower(_)
        | ValueDef::TargetManaValue(_)
        | ValueDef::DividedAmongTargets => false,
    }
}

fn static_trigger_condition_supported(condition: TriggerConditionDef) -> bool {
    match condition {
        TriggerConditionDef::ObjectCount { query, .. } => static_query_supported(query),
        TriggerConditionDef::ActivePlayer(relation)
        | TriggerConditionDef::SpellsCastLastTurn {
            player: relation, ..
        } => static_player_relation_supported(relation),
        TriggerConditionDef::SourceActivationsThisTurn { .. }
        | TriggerConditionDef::TargetMatches { .. }
        | TriggerConditionDef::ControlsGreatestPowerCreature => false,
        TriggerConditionDef::AttachedPermanentMatches { object } => {
            static_object_predicate_supported(object)
        }
        TriggerConditionDef::SourceOnBattlefield
        | TriggerConditionDef::SourceUntapped
        | TriggerConditionDef::SourceCounters { .. }
        | TriggerConditionDef::SourceLoyalty { .. }
        | TriggerConditionDef::SourceDealtDamageToOpponentThisTurn
        | TriggerConditionDef::SourceIsTapped => true,
    }
}

fn static_damage_matcher_supported(matcher: DamageEventMatcherDef) -> bool {
    let source = match matcher.source {
        DamageSourceMatcherDef::Any
        | DamageSourceMatcherDef::Group(_)
        | DamageSourceMatcherDef::AffectedObject => true,
        DamageSourceMatcherDef::Object(reference) | DamageSourceMatcherDef::Except(reference) => {
            static_damage_object_reference_supported(reference)
        }
        DamageSourceMatcherDef::Matching(predicate) => static_object_predicate_supported(predicate),
    };
    let recipient = match matcher.recipient {
        DamageRecipientMatcherDef::Any | DamageRecipientMatcherDef::AffectedObject => true,
        DamageRecipientMatcherDef::Recipients(recipients) => recipients
            .object_reference()
            .is_some_and(static_damage_object_reference_supported),
        DamageRecipientMatcherDef::PlayerAndCreaturesControlledBy(_) => false,
    };
    source && recipient
}

fn static_damage_object_reference_supported(reference: ObjectRefDef) -> bool {
    matches!(
        reference,
        ObjectRefDef::Source | ObjectRefDef::AttachedToSource
    )
}

const fn effect_operation_name(effect: EffectDef) -> &'static str {
    match effect {
        EffectDef::None => "None",
        EffectDef::Sequence(_) => "Sequence",
        EffectDef::Randomized { .. } => "Randomized",
        EffectDef::Choose(_) => "Choose",
        EffectDef::PayOr(_) => "PayOr",
        EffectDef::SplitIntoPiles(_) => "SplitIntoPiles",
        EffectDef::PreventDamage { .. } => "PreventDamage",
        EffectDef::AddMana(_) => "AddMana",
        EffectDef::AddPoisonCounters { .. } => "AddPoisonCounters",
        EffectDef::DealDamage { .. } => "DealDamage",
        EffectDef::GainLife { .. } => "GainLife",
        EffectDef::DrawCards { .. } => "DrawCards",
        EffectDef::ShuffleLibrary { .. } => "ShuffleLibrary",
        EffectDef::EmptyManaPool { .. } => "EmptyManaPool",
        EffectDef::Discard { .. } => "Discard",
        EffectDef::DiscardCards { .. } => "DiscardCards",
        EffectDef::LoseLife { .. } => "LoseLife",
        EffectDef::LoseTheGame { .. } => "LoseTheGame",
        EffectDef::Tap { .. } => "Tap",
        EffectDef::RemoveFromCombat { .. } => "RemoveFromCombat",
        EffectDef::Untap { .. } => "Untap",
        EffectDef::CreateToken { .. } => "CreateToken",
        EffectDef::CreateAttachedToken { .. } => "CreateAttachedToken",
        EffectDef::CreateTokenCopyOf { .. } => "CreateTokenCopyOf",
        EffectDef::Attach { .. } => "Attach",
        EffectDef::Reconfigure { .. } => "Reconfigure",
        EffectDef::Destroy { .. } => "Destroy",
        EffectDef::DestroyAtEndOfCombat { .. } => "DestroyAtEndOfCombat",
        EffectDef::Detain { .. } => "Detain",
        EffectDef::Regenerate { .. } => "Regenerate",
        EffectDef::Sacrifice { .. } => "Sacrifice",
        EffectDef::SacrificeOfChoice { .. } => "SacrificeOfChoice",
        EffectDef::Mill { .. } => "Mill",
        EffectDef::LookAtHand { .. } => "LookAtHand",
        EffectDef::LookAtTopAndSelect { .. } => "LookAtTopAndSelect",
        EffectDef::SearchZone { .. } => "SearchZone",
        EffectDef::ChooseCards { .. } => "ChooseCards",
        EffectDef::ReplaceNextDrawThisTurn { .. } => "ReplaceNextDrawThisTurn",
        EffectDef::IfFormat { .. } => "IfFormat",
        EffectDef::Counter { .. } => "Counter",
        EffectDef::DrainLife { .. } => "DrainLife",
        EffectDef::AddManaEqualTo { .. } => "AddManaEqualTo",
        EffectDef::AddCounters { .. } => "AddCounters",
        EffectDef::RemoveAllCounters { .. } => "RemoveAllCounters",
        EffectDef::SkipNextUntapSteps { .. } => "SkipNextUntapSteps",
        EffectDef::ChangeTextBasicLandType { .. } => "ChangeTextBasicLandType",
        EffectDef::BecomeCopyOf { .. } => "BecomeCopyOf",
        EffectDef::GrantFlashToNextSorcery => "GrantFlashToNextSorcery",
        EffectDef::May { .. } => "May",
        EffectDef::ExileLinkedToSource { .. } => "ExileLinkedToSource",
        EffectDef::ReturnLinkedExiles { .. } => "ReturnLinkedExiles",
        EffectDef::GainControl { .. } => "GainControl",
        EffectDef::IfCondition { .. } => "IfCondition",
        EffectDef::InstallTrigger(_) => "InstallTrigger",
        EffectDef::CannotBeForcedToSacrifice => "CannotBeForcedToSacrifice",
        EffectDef::ReduceGenericCostBy(_) => "ReduceGenericCostBy",
        EffectDef::CannotAttackUnless(_) => "CannotAttackUnless",
        EffectDef::LandwalkCanBeBlocked(_) => "LandwalkCanBeBlocked",
        EffectDef::ScheduleTurnPhases(_) => "ScheduleTurnPhases",
        EffectDef::TakeExtraTurn { .. } => "TakeExtraTurn",
        EffectDef::CreateEmblem { .. } => "CreateEmblem",
        EffectDef::Transform { .. } => "Transform",
        EffectDef::MoveToZone { .. } => "MoveToZone",
        EffectDef::StaticApply { .. } => "StaticApply",
        EffectDef::Apply { .. } => "Apply",
        EffectDef::Special(_) => "Special",
    }
}
