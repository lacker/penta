use crate::card::{
    AbilityCostDef, AbilityDef, AbilityOperationDef, AbilityProcedureDef, AppliedEffectDef,
    AppliedRuleDef, AttackDefenderScopeDef, AttackRestrictionDef, BlockRestrictionDef,
    BlockRestrictionMatchDef, CardType, CharacteristicOperationDef, CostAdjustmentDef,
    CostAmountDef, CostModificationDef, DamageEventMatcherDef, DamageRecipientMatcherDef,
    DamageSourceMatcherDef, DeclarativeAbilityDef, EffectDef, EffectRecipientDef,
    EffectRecipientSetDef, ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef,
    ObjectValueAggregateDef, ObjectValueDef, PlayerRefDef, PlayerRelation, PlayerSetDef,
    PowerToughnessOperationDef, ReplacementEffectDef, ReplacementEventDef, SetOperationDef,
    SpellCostConditionDef, TriggerConditionDef, ValueDef, ZoneKind,
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
        // Metadata-only and replacement programs do not execute this ordinary
        // effect tree through either runtime.
        return Ok(());
    };
    match ability.definition {
        DeclarativeAbilityDef::Static(definition) => {
            validate_static_effect(effect, definition.source_zones, StaticPosition::Root).map_err(
                |operation| EffectProgramContextError {
                    context: "static",
                    operation,
                },
            )
        }
        DeclarativeAbilityDef::Replacement(_) => Ok(()),
        DeclarativeAbilityDef::Spell(_)
        | DeclarativeAbilityDef::ActivatedMana(_)
        | DeclarativeAbilityDef::TriggeredMana(_)
        | DeclarativeAbilityDef::Activated(_)
        | DeclarativeAbilityDef::Triggered(_)
        | DeclarativeAbilityDef::AlternativeCast(_)
        | DeclarativeAbilityDef::OptionalAdditionalCost(_)
        | DeclarativeAbilityDef::SpecialAction(_)
        | DeclarativeAbilityDef::Pregame(_)
        | DeclarativeAbilityDef::Keyword(_)
        | DeclarativeAbilityDef::DeckConstruction(_) => {
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
        DeclarativeAbilityDef::Pregame(_) => &[ZoneKind::Hand],
        DeclarativeAbilityDef::Spell(_)
        | DeclarativeAbilityDef::AlternativeCast(_)
        | DeclarativeAbilityDef::OptionalAdditionalCost(_)
        | DeclarativeAbilityDef::Keyword(_)
        | DeclarativeAbilityDef::DeckConstruction(_) => &[ZoneKind::Stack],
        DeclarativeAbilityDef::Static(_) | DeclarativeAbilityDef::Replacement(_) => &[],
    }
}

#[allow(clippy::too_many_lines)]
fn validate_static_effect(
    effect: EffectDef,
    source_zones: &[ZoneKind],
    position: StaticPosition,
) -> Result<(), &'static str> {
    match effect {
        EffectDef::None if position == StaticPosition::Root => Ok(()),
        EffectDef::Sequence(effects) => {
            if effects.is_empty() {
                return Err("empty Sequence");
            }
            for effect in effects {
                validate_static_effect(*effect, source_zones, StaticPosition::Traversed)?;
            }
            Ok(())
        }
        effect @ (EffectDef::IfCondition { .. } | EffectDef::IfElseCondition { .. })
            if matches!(source_zones, [ZoneKind::Battlefield | ZoneKind::Graveyard]) =>
        {
            let conditional = effect
                .conditional()
                .expect("conditional variants expose their shared shape");
            if !static_trigger_condition_supported(*conditional.condition) {
                return Err(effect_operation_name(effect));
            }
            validate_static_effect(*conditional.then, source_zones, StaticPosition::Traversed)?;
            conditional.otherwise.map_or(Ok(()), |otherwise| {
                validate_static_effect(*otherwise, source_zones, StaticPosition::Traversed)
            })
        }
        EffectDef::ConditionalStatic(conditional)
            if matches!(source_zones, [ZoneKind::Battlefield | ZoneKind::Graveyard])
                && static_condition_object_set_supported(*conditional.condition.objects)
                && conditional
                    .condition
                    .filter
                    .is_none_or(|filter| static_object_predicate_supported(filter.predicate())) =>
        {
            validate_static_apply(
                source_zones,
                conditional.then.recipient,
                conditional.then.effect,
            )
        }
        EffectDef::StaticApply { recipient, effect } => {
            validate_static_apply(source_zones, recipient, effect)
        }
        EffectDef::GainControl {
            object: EffectRecipientDef::AttachedPermanent,
            controller: PlayerRefDef::EffectController,
            duration:
                crate::card::ControlDurationDef::WhileSourceRemains {
                    while_tapped: false,
                },
        } if position == StaticPosition::Root && source_zones == [ZoneKind::Battlefield] => Ok(()),
        // A prohibition holds wherever a static clause states it, including
        // one of several the same printed sentence states -- Tamiyo says
        // "discard cards or sacrifice permanents" in one breath.
        EffectDef::CannotBeForcedToSacrifice | EffectDef::CannotBeForcedToDiscard
            if source_zones == [ZoneKind::Battlefield] =>
        {
            Ok(())
        }
        EffectDef::LandwalkCanBeBlocked(_)
            if position == StaticPosition::Root && source_zones == [ZoneKind::Battlefield] =>
        {
            Ok(())
        }
        EffectDef::CannotAttackUnless(query) | EffectDef::CannotAttackIf(query)
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
        // The battlefield mirror: read off a permanent rather than the card
        // discounting itself, so it names the spells and their caster.
        // The increase carries a whole mana cost rather than a value, so
        // there is nothing here to check beyond the predicate and the player
        // relation the discount beside it also checks.
        EffectDef::ModifyCost(modification)
            if static_ability_increase_supported(modification, source_zones, position) =>
        {
            Ok(())
        }
        EffectDef::ModifyCost(CostModificationDef::AbilityReduction {
            permanent: matcher,
            amount,
            ..
        }) if position == StaticPosition::Root
            && source_zones == [ZoneKind::Battlefield]
            && static_object_predicate_supported(matcher)
            && static_cost_reduction_value_supported(amount) =>
        {
            Ok(())
        }
        EffectDef::ModifyCost(modification)
            if position == StaticPosition::Root
                && static_spell_cost_modification_supported(modification, source_zones) =>
        {
            Ok(())
        }
        _ => Err(effect_operation_name(effect)),
    }
}

fn static_ability_increase_supported(
    modification: CostModificationDef,
    source_zones: &[ZoneKind],
    position: StaticPosition,
) -> bool {
    let matcher = match modification {
        CostModificationDef::AbilityIncrease { permanent, .. } => permanent,
        CostModificationDef::SourceAbilityIncrease { source, .. } => source,
        _ => return false,
    };
    position == StaticPosition::Root
        && source_zones == [ZoneKind::Battlefield]
        && static_object_predicate_supported(matcher)
}

/// What a card may say about itself while it is in a zone: the pieces the
/// card view can carry, which are types, subtypes, and a printed body.
fn card_static_applied_effect_supported(effect: AppliedEffectDef) -> bool {
    match effect {
        AppliedEffectDef::Composite(effects) => {
            !effects.is_empty()
                && effects
                    .iter()
                    .copied()
                    .all(card_static_applied_effect_supported)
        }
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::CardTypes(
            SetOperationDef::Add(types),
        )) => !types.is_empty(),
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::Subtypes(
            SetOperationDef::Add(subtypes),
        )) => !subtypes.is_empty(),
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
            PowerToughnessOperationDef::SetBase {
                power: ValueDef::Constant(_),
                toughness: ValueDef::Constant(_),
            },
        )) => true,
        _ => false,
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
    // "As long as this isn't on the battlefield, it's a 1/1 Insect creature":
    // a clause about the card rather than about a permanent, read wherever
    // the card is asked about. Only what the card says about itself, and
    // only the characteristics a card-in-a-zone view can carry.
    if !source_zones.is_empty()
        && source_zones
            .iter()
            .all(|zone| matches!(zone, ZoneKind::Library | ZoneKind::Hand | ZoneKind::Exile))
        || source_zones.contains(&ZoneKind::Library) && source_zones.contains(&ZoneKind::Graveyard)
    {
        return if recipient == EffectRecipientDef::Source
            && card_static_applied_effect_supported(effect)
        {
            Ok(())
        } else {
            Err("StaticApply outside its supported card-source shape")
        };
    }
    if !matches!(source_zones, [ZoneKind::Battlefield | ZoneKind::Graveyard]) {
        return Err("StaticApply from unsupported source zones");
    }
    match recipient.0 {
        // A static clause names one kind of thing or the other; the mixed
        // recipient exists for a resolving damage clause.
        EffectRecipientSetDef::PlayersAndCreaturesTheyControl(_) => {
            Err("StaticApply with a mixed player-and-creature recipient")
        }
        // A static clause says what it affects; what a creature is attacking
        // is a fact about one combat, which a resolving clause reads.
        EffectRecipientSetDef::DefenderOf(_) => {
            Err("StaticApply with an attack-defender recipient")
        }
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
        AppliedEffectDef::Rule(
            AppliedRuleDef::CannotPlay(restriction)
            | AppliedRuleDef::MayPlayFromTopOfLibrary { restriction, .. },
        ) => static_object_predicate_supported(restriction.object),
        AppliedEffectDef::Rule(AppliedRuleDef::MayPlayFromGraveyard(permission)) => {
            static_object_predicate_supported(permission.restriction.object)
        }
        // Read where a cast is offered, against the card being cast, just
        // like the other player-facing play permissions above.
        AppliedEffectDef::Rule(AppliedRuleDef::MayCastAsThoughItHadFlash(permission)) => {
            static_object_predicate_supported(permission.object)
        }
        // Both predicates are read against an object the trigger walk
        // already has in hand: what arrived, and what carries the ability.
        AppliedEffectDef::Rule(AppliedRuleDef::TriggersAnAdditionalTime(doubling)) => {
            static_object_predicate_supported(doubling.entering)
                && static_object_predicate_supported(doubling.permanent)
        }
        // Read where a graveyard cast is enumerated, by the same walk that
        // answers the permissions above.
        AppliedEffectDef::Rule(AppliedRuleDef::GrantsAlternativeCastFromGraveyard {
            object,
            ability,
        }) => {
            static_object_predicate_supported(object)
                && matches!(
                    ability.definition,
                    crate::card::DeclarativeAbilityDef::AlternativeCast(_)
                )
        }
        // A damage limit protecting a player is read by its own walk over the
        // battlefield, since nothing about the damage event points back at
        // the permanent carrying the rule.
        AppliedEffectDef::Rule(AppliedRuleDef::LimitDamage { matcher, .. }) => {
            static_damage_matcher_supported(matcher)
        }
        // Read by the untap procedure, which walks the battlefield for the
        // same reason: the cap names a player, not a permanent.
        AppliedEffectDef::Rule(AppliedRuleDef::UntapAtMostOne(predicate)) => {
            static_object_predicate_supported(predicate)
        }
        AppliedEffectDef::Rule(AppliedRuleDef::AttackRestriction(restriction)) => {
            restriction.defender != AttackDefenderScopeDef::Any
                && static_attack_restriction_supported(restriction)
        }
        // Read by the cleanup step, by the same walk and for the same reason.
        // The colour permission is read the same way, from the mana payment
        // rather than the cleanup step.
        AppliedEffectDef::Rule(
            AppliedRuleDef::Ascend
            | AppliedRuleDef::MayLookAtTopOfLibrary
            // Read by whoever is being shown the game rather than by any
            // step of it: a public top card changes what an observation
            // says and nothing else.
            | AppliedRuleDef::PlaysWithTopOfLibraryRevealed
            | AppliedRuleDef::MaySpendManaAsAnyColorForCreatureAbilities
            | AppliedRuleDef::MayPlayAdditionalLands(_)
            | AppliedRuleDef::MayPlayAnyNumberOfLands
            | AppliedRuleDef::CannotDrawMoreThanEachTurn(_)
            | AppliedRuleDef::RevealsDrawnCards | AppliedRuleDef::CannotGainLife
            | AppliedRuleDef::PlayerRule(_)
            | AppliedRuleDef::DoublesTokensCreated,
        ) => true,
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
        AppliedEffectDef::Characteristic(operation) => {
            static_object_characteristic_supported(recipient, operation)
        }
        AppliedEffectDef::Rule(rule) => static_object_rule_supported(recipient, rule),
    }
}

/// Which characteristic operations a static walk can supply. Split from the
/// rules beside them for the source-size budget; the two halves answer
/// different questions and share only their recipient.
fn static_object_characteristic_supported(
    recipient: EffectRecipientDef,
    operation: CharacteristicOperationDef,
) -> bool {
    match operation {
        // A switch reads nothing, so there is no value to gate on and no way
        // for it to re-enter the characteristics walk. "This land is the
        // chosen type" reads only the choice its own source made, which is
        // the same story.
        CharacteristicOperationDef::PowerToughness(PowerToughnessOperationDef::Switch)
        | CharacteristicOperationDef::Abilities(_)
        | CharacteristicOperationDef::ChosenBasicLandType => true,
        CharacteristicOperationDef::BasicLandTypes(operation) => match operation {
            SetOperationDef::Add(types)
            | SetOperationDef::Remove(types)
            | SetOperationDef::Set(types) => !types.is_empty(),
        },
        CharacteristicOperationDef::PowerToughness(
            PowerToughnessOperationDef::SetBase { power, toughness }
            | PowerToughnessOperationDef::Modify { power, toughness },
        ) => {
            static_power_toughness_value_supported(power)
                && static_power_toughness_value_supported(toughness)
        }
        CharacteristicOperationDef::PowerToughness(
            PowerToughnessOperationDef::SetBasePower(power)
            | PowerToughnessOperationDef::SetBaseToughness(power),
        ) => static_power_toughness_value_supported(power),
        // A characteristic-defining ability defines its own source and
        // names at least one half; the amounts it may read are the same
        // ones any other static stat may read.
        CharacteristicOperationDef::PowerToughness(PowerToughnessOperationDef::Define {
            power,
            toughness,
        }) => {
            recipient == EffectRecipientDef::Source
                && (power.is_some() || toughness.is_some())
                && power.is_none_or(static_power_toughness_value_supported)
                && toughness.is_none_or(static_power_toughness_value_supported)
        }
        // Static animation is deliberately narrower than resolving
        // characteristic changes. A direct source/attachment recipient
        // cannot feed back into its own selection; a group query must avoid
        // reading the characteristics it supplies.
        CharacteristicOperationDef::CardTypes(SetOperationDef::Add(types)) => {
            types != crate::card::CardTypeSet::EMPTY
                && (static_direct_characteristic_recipient(recipient)
                    || types == crate::card::CardTypeSet::single(CardType::Creature)
                        && static_type_animation_query_supported(recipient))
        }
        CharacteristicOperationDef::Color(_)
        | CharacteristicOperationDef::Colors(_)
        | CharacteristicOperationDef::Subtypes(_) => static_animation_query_supported(recipient),
        CharacteristicOperationDef::CreatureTypes(_) => {
            static_direct_characteristic_recipient(recipient)
        }
        CharacteristicOperationDef::CardTypes(
            SetOperationDef::Remove(_) | SetOperationDef::Set(_),
        ) => false,
    }
}

/// Which rules a static walk can supply to an object.
fn static_object_rule_supported(recipient: EffectRecipientDef, rule: AppliedRuleDef) -> bool {
    match rule {
        AppliedRuleDef::AssignsNoCombatDamage
        | AppliedRuleDef::AssignsCombatDamageEqualToToughness
        // Read off the creature dealing the damage, which the damage walk
        // has in hand.
        | AppliedRuleDef::CombatDamageCannotBePrevented
        // Read off the creature being tapped to pay, which the crew walk
        // has in hand.
        | AppliedRuleDef::CrewsAsThoughPowerGreater(_)
        | AppliedRuleDef::CannotBeEnchanted
        | AppliedRuleDef::CannotBecomeEnchanted
        | AppliedRuleDef::CannotActivateAbilities
        | AppliedRuleDef::MayActivateLoyaltyAnyTime
        | AppliedRuleDef::MayAttackDespiteDefender
        | AppliedRuleDef::MayAttackAsThoughHasty
        | AppliedRuleDef::MustBlockEachAttackerIfAble
        | AppliedRuleDef::CannotChangeController
        | AppliedRuleDef::CannotRegenerate
        | AppliedRuleDef::DoesNotUntapDuringUntapStep
        | AppliedRuleDef::MayChooseNotToUntap
        | AppliedRuleDef::RemainsAttachedThroughProtection => true,
        AppliedRuleDef::AttackRestriction(restriction) => {
            restriction.defender == AttackDefenderScopeDef::Any
                && static_attack_restriction_supported(restriction)
        }
        AppliedRuleDef::BlockRestriction(restriction) => {
            static_block_restriction_supported(restriction)
        }
        // Zero extra blocks would be a rule that grants nothing.
        AppliedRuleDef::MayBlockAdditionalCreatures(extra) => extra > 0,
        // "Except by one or more creatures" is what every creature already
        // is, so a printed clause saying it would say nothing.
        AppliedRuleDef::CannotBeBlockedExceptByAtLeast(required) => required > 1,
        AppliedRuleDef::CannotBeCountered
        // Ascend belongs to a player, so nothing about an object reads it.
        | AppliedRuleDef::Ascend
        | AppliedRuleDef::MayLookAtTopOfLibrary
        | AppliedRuleDef::PlaysWithTopOfLibraryRevealed
        // A doubled trigger belongs to a player as well: what it reads is
        // who controls the permanent and who controls what arrived.
        | AppliedRuleDef::TriggersAnAdditionalTime(_)
        | AppliedRuleDef::MaySpendManaAsAnyColorForCreatureAbilities
        | AppliedRuleDef::MayPlayAdditionalLands(_)
        | AppliedRuleDef::MayPlayAnyNumberOfLands
        | AppliedRuleDef::CannotDrawMoreThanEachTurn(_)
        | AppliedRuleDef::RevealsDrawnCards
        | AppliedRuleDef::DoublesTokensCreated
        | AppliedRuleDef::CannotPlay(_)
        // A timing permission belongs to a player too, and so does a bar on
        // gaining life.
        | AppliedRuleDef::MayCastAsThoughItHadFlash(_)
        | AppliedRuleDef::CannotGainLife
        | AppliedRuleDef::PlayerProtectionFrom(_) | AppliedRuleDef::PlayerRule(_)
        // No printed static says "if it would die, exile it instead": every
        // card that says it is a resolving effect with a duration on it.
        | AppliedRuleDef::ExileInsteadOfDying
        | AppliedRuleDef::MayPlayFromGraveyard(_)
        | AppliedRuleDef::MayPlayFromTopOfLibrary { .. }
        | AppliedRuleDef::GrantsAlternativeCastFromGraveyard { .. }
        | AppliedRuleDef::UntapAtMostOne(_)
        | AppliedRuleDef::RedirectDamageFromTo { .. } => false,
        AppliedRuleDef::MustBeBlockedBy(predicate) => static_object_predicate_supported(predicate),
        AppliedRuleDef::PreventDamage(matcher) | AppliedRuleDef::LimitDamage { matcher, .. } => {
            static_damage_matcher_supported(matcher)
        }
        AppliedRuleDef::RedirectPlayerDamageToThis(_) => {
            matches!(
                recipient.object_reference(),
                Some(ObjectRefDef::Source | ObjectRefDef::AttachedToSource)
            ) || recipient.object_query().is_some()
        }
    }
}

fn static_attack_restriction_supported(restriction: AttackRestrictionDef) -> bool {
    static_object_predicate_supported(restriction.attacker)
        && restriction
            .cost
            .is_none_or(|cost| !cost.variable_x && cost.x_multiplier == 0)
}

fn static_block_restriction_supported(restriction: BlockRestrictionDef) -> bool {
    let counterpart_supported = match restriction.counterpart {
        BlockRestrictionMatchDef::Any => true,
        BlockRestrictionMatchDef::Matching(predicate)
        | BlockRestrictionMatchDef::Except(predicate) => {
            static_object_predicate_supported(predicate)
        }
    };
    counterpart_supported
        && restriction
            .cost
            .is_none_or(|cost| !cost.variable_x && cost.x_multiplier == 0)
}

fn validate_resolving_effect(
    effect: EffectDef,
    source_zones: &[ZoneKind],
) -> Result<(), &'static str> {
    match effect {
        EffectDef::Sequence([]) => Err("empty Sequence"),
        EffectDef::InstallTrigger(trigger) => {
            let Some(effect) = trigger.ability.declarative_effect() else {
                return Err("InstallTrigger with a non-declarative program");
            };
            validate_resolving_effect(effect, source_zones)
        }
        // A turn-scoped replacement has nowhere else to live: the card that
        // made it is gone by the time it applies, so the effect object is
        // its home rather than a permanent it could be granted to.
        EffectDef::CreateOngoingEffect(ongoing)
            if matches!(
                ongoing.ability.definition,
                DeclarativeAbilityDef::Replacement(_)
            ) =>
        {
            let DeclarativeAbilityDef::Replacement(definition) = ongoing.ability.definition else {
                unreachable!("the guard above matched a replacement")
            };
            let Some(effect) = ongoing.ability.declarative_replacement() else {
                return Err("CreateOngoingEffect with a non-declarative replacement");
            };
            if !matches!(
                (definition.event, effect),
                (
                    ReplacementEventDef::AnyObjectWouldMove { .. },
                    ReplacementEffectDef::MoveToZone(_),
                )
            ) {
                return Err("CreateOngoingEffect with an unsupported replacement");
            }
            Ok(())
        }
        EffectDef::CreateOngoingEffect(ongoing) => {
            let (definition, mana) = match ongoing.ability.definition {
                DeclarativeAbilityDef::Activated(definition) => (definition, false),
                DeclarativeAbilityDef::ActivatedMana(definition) => (definition, true),
                _ => return Err("CreateOngoingEffect with a non-activated ability"),
            };
            let Some(effect) = ongoing.ability.declarative_effect() else {
                return Err("CreateOngoingEffect with a non-declarative program");
            };
            if definition.procedure != AbilityProcedureDef::Shared
                || definition.source_zones != [ZoneKind::Command]
                || !definition.targets.is_empty()
                || definition.modes.is_some()
                || definition.activation_limit.is_some()
                || definition.any_player_may_activate
                || definition.condition.is_some()
                || definition.costs.as_slice().iter().any(|cost| {
                    if mana {
                        !matches!(cost, AbilityCostDef::PayLife(_))
                    } else {
                        !matches!(cost, AbilityCostDef::Mana(cost) if !cost.variable_x)
                    }
                })
                || ongoing.duration == crate::card::ResolvedEffectDurationDef::WhileSourceTapped
            {
                return Err("CreateOngoingEffect with an unsupported activated ability");
            }
            validate_resolving_effect(effect, &[ZoneKind::Command])
        }
        EffectDef::Apply {
            recipient, effect, ..
        } if recipient == EffectRecipientDef::Source
            && applied_effect_adds_ability(effect)
            && source_zones != [ZoneKind::Battlefield] =>
        {
            Err("Apply grants an ability to a nonbattlefield source")
        }
        EffectDef::ContinueReplacedDraw
        | EffectDef::ConditionalStatic(_)
        | EffectDef::StaticApply { .. }
        | EffectDef::CannotBeForcedToSacrifice
        | EffectDef::CannotBeForcedToDiscard
        | EffectDef::ReduceGenericCostBy(_)
        | EffectDef::ModifyCost(_)
        | EffectDef::CannotAttackUnless(_)
        | EffectDef::CannotAttackIf(_)
        | EffectDef::LandwalkCanBeBlocked(_) => Err(effect_operation_name(effect)),
        _ => {
            for child in crate::card::child_effects(effect) {
                validate_resolving_effect(child, source_zones)?;
            }
            Ok(())
        }
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
        PlayerSetDef::All
        | PlayerSetDef::One(
            PlayerRefDef::EffectController | PlayerRefDef::Opponent | PlayerRefDef::EnchantedPlayer,
        ) => true,
        PlayerSetDef::Related(relation) => static_player_relation_supported(relation),
        PlayerSetDef::LegalTargets(_)
        | PlayerSetDef::One(
            PlayerRefDef::EventPlayer
            | PlayerRefDef::Target(_)
            | PlayerRefDef::ControllerOf(_)
            | PlayerRefDef::OpponentOf(_)
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
            | PlayerRelation::ChosenPlayer
            | PlayerRelation::DefendingPlayer
            | PlayerRelation::EnchantedPlayer
    )
}

fn static_object_set_supported(objects: ObjectSetDef) -> bool {
    match objects {
        ObjectSetDef::One(ObjectRefDef::Source | ObjectRefDef::AttachedToSource)
        | ObjectSetDef::LinkedExiles => true,
        ObjectSetDef::Query(query) => {
            query.zones == [ZoneKind::Battlefield] && static_query_supported(query)
        }
        ObjectSetDef::LegalTargets(_)
        | ObjectSetDef::One(
            ObjectRefDef::ResolvingObject
            | ObjectRefDef::CreatingSource
            | ObjectRefDef::ZoneChangeSuccessor(_)
            | ObjectRefDef::ZoneChangeResultOfTriggeringObject
            | ObjectRefDef::Binding(_)
            | ObjectRefDef::AdditionalCostObject(_)
            | ObjectRefDef::AbilityGrantSource
            | ObjectRefDef::Target(_)
            | ObjectRefDef::SourceOfTargetedStackObject(_)
            | ObjectRefDef::TriggeringObject
            | ObjectRefDef::DamagedObject,
        )
        | ObjectSetDef::Binding(_)
        | ObjectSetDef::ZoneChangeSuccessorsOfBinding(_)
        | ObjectSetDef::MatchingBinding { .. }
        | ObjectSetDef::PermanentsTargetedBy(_)
        | ObjectSetDef::PlayerAttachments(_)
        | ObjectSetDef::LegalAttachmentHosts(_)
        | ObjectSetDef::CardsDrawnThisTurnInHand(_)
        | ObjectSetDef::PermanentsControlledBy(_)
        | ObjectSetDef::TokensCreatedBy(_)
        | ObjectSetDef::BottomOfGraveyard(_)
        | ObjectSetDef::SharingNameWith(_)
        | ObjectSetDef::SharingNameWithBinding { .. }
        | ObjectSetDef::TopOfGraveyardMatching { .. } => false,
        ObjectSetDef::Matching { objects, object } => {
            static_object_set_supported(*objects)
                && static_object_predicate_supported(object.predicate())
        }
    }
}

/// A condition may inspect objects outside the battlefield even though a
/// static apply can only modify battlefield objects. This preserves the
/// query vocabulary supported by the older `ObjectCount` condition while the
/// count and the applied operation stay separately composed.
fn static_condition_object_set_supported(objects: ObjectSetDef) -> bool {
    match objects {
        ObjectSetDef::Query(query) => static_query_supported(query),
        ObjectSetDef::Matching { objects, object } => {
            static_condition_object_set_supported(*objects)
                && static_object_predicate_supported(object.predicate())
        }
        _ => static_object_set_supported(objects),
    }
}

fn static_query_supported(query: ObjectQueryDef) -> bool {
    !query.zones.is_empty()
        && query.relative_position.is_none()
        && [query.related_player, query.controller, query.owner]
            .into_iter()
            .flatten()
            .all(static_player_set_supported)
        && static_object_predicate_supported(query.object)
}

fn static_animation_query_supported(recipient: EffectRecipientDef) -> bool {
    static_direct_characteristic_recipient(recipient)
        || recipient.object_query().is_some_and(|query| {
            query.zones == [ZoneKind::Battlefield]
                && static_query_supported(query)
                && static_animation_predicate_supported(query.object, false)
        })
}

fn static_type_animation_query_supported(recipient: EffectRecipientDef) -> bool {
    static_direct_characteristic_recipient(recipient)
        || recipient.object_query().is_some_and(|query| {
            query.zones == [ZoneKind::Battlefield]
                && static_query_supported(query)
                && static_animation_predicate_supported(query.object, true)
        })
}

fn static_direct_characteristic_recipient(recipient: EffectRecipientDef) -> bool {
    matches!(
        recipient.object_reference(),
        Some(ObjectRefDef::Source | ObjectRefDef::AttachedToSource)
    )
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
        DamageRecipientMatcherDef::MatchingObject(predicate) => {
            static_object_predicate_supported(predicate)
        }
        DamageRecipientMatcherDef::PlayerAndCreaturesControlledBy(_)
        | DamageRecipientMatcherDef::PlayerOrPlaneswalker => false,
    };
    source && recipient
}

fn static_damage_object_reference_supported(reference: ObjectRefDef) -> bool {
    matches!(
        reference,
        ObjectRefDef::Source | ObjectRefDef::AttachedToSource
    )
}

include!("program_context/effect_operation_names.rs");
include!("program_context/static_conditions.rs");
include!("program_context/static_predicates.rs");
include!("program_context/static_values.rs");
