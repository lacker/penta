use crate::card::{
    AbilityCostDef, AbilityDef, AbilityOperationDef, AbilityProcedureDef, AppliedEffectDef,
    AppliedRuleDef, AttackDefenderScopeDef, AttackRestrictionDef, BlockRestrictionDef,
    BlockRestrictionMatchDef, CardType, CharacteristicOperationDef, CostModificationDef,
    DamageEventMatcherDef, DamageRecipientMatcherDef, DamageSourceMatcherDef,
    DeclarativeAbilityDef, EffectDef, EffectRecipientDef, EffectRecipientSetDef,
    ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef, PlayerRefDef, PlayerRelation,
    PlayerSetDef, PowerToughnessOperationDef, SetOperationDef, TriggerConditionDef, ValueDef,
    ZoneKind,
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
        | DeclarativeAbilityDef::OptionalAdditionalCost(_)
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
        | DeclarativeAbilityDef::OptionalAdditionalCost(_)
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
        EffectDef::ModifyCost(CostModificationDef::AbilityIncrease {
            permanent: matcher, ..
        }) if position == StaticPosition::Root
            && source_zones == [ZoneKind::Battlefield]
            && static_object_predicate_supported(matcher) =>
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
        EffectDef::ModifyCost(CostModificationDef::SpellIncrease { spell, caster, .. })
            if position == StaticPosition::Root
                && source_zones == [ZoneKind::Battlefield]
                && static_object_predicate_supported(spell)
                && static_player_relation_supported(caster) =>
        {
            Ok(())
        }
        EffectDef::ModifyCost(CostModificationDef::SpellReduction {
            spell,
            caster,
            amount,
        }) if position == StaticPosition::Root
            && source_zones == [ZoneKind::Battlefield]
            && static_object_predicate_supported(spell)
            && static_player_relation_supported(caster)
            && static_cost_reduction_value_supported(amount) =>
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
        // A static clause names one kind of thing or the other; the mixed
        // recipient exists for a resolving damage clause.
        EffectRecipientSetDef::PlayersAndCreaturesTheyControl(_) => {
            Err("StaticApply with a mixed player-and-creature recipient")
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
            | AppliedRuleDef::MaySpendManaAsAnyColorForCreatureAbilities
            | AppliedRuleDef::MayPlayAdditionalLands(_)
            | AppliedRuleDef::NoMaximumHandSize
            | AppliedRuleDef::DoublesTokensCreated
            | AppliedRuleDef::WinsInsteadOfDrawingFromEmptyLibrary,
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
        // Static animation is deliberately narrower than resolving
        // characteristic changes. A direct source/attachment recipient
        // cannot feed back into its own selection; a group query must avoid
        // reading the characteristics it supplies.
        CharacteristicOperationDef::CardTypes(SetOperationDef::Add(types)) => {
            types == crate::card::CardTypeSet::single(CardType::Creature)
                && static_type_animation_query_supported(recipient)
        }
        CharacteristicOperationDef::Colors(_) => static_animation_query_supported(recipient),
        CharacteristicOperationDef::CreatureTypes(_) | CharacteristicOperationDef::Subtypes(_) => {
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
        | AppliedRuleDef::MaySpendManaAsAnyColorForCreatureAbilities
        | AppliedRuleDef::MayPlayAdditionalLands(_)
        | AppliedRuleDef::NoMaximumHandSize
        | AppliedRuleDef::DoublesTokensCreated
        | AppliedRuleDef::WinsInsteadOfDrawingFromEmptyLibrary
        | AppliedRuleDef::CannotPlay(_)
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
        EffectDef::StaticApply { .. }
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
        | PlayerSetDef::One(PlayerRefDef::EffectController | PlayerRefDef::Opponent) => true,
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
            | ObjectRefDef::AbilityGrantSource
            | ObjectRefDef::Target(_)
            | ObjectRefDef::SourceOfTargetedStackObject(_)
            | ObjectRefDef::TriggeringObject
            | ObjectRefDef::DamagedObject,
        )
        | ObjectSetDef::Binding(_)
        | ObjectSetDef::MatchingBinding { .. }
        | ObjectSetDef::PermanentsTargetedBy(_)
        | ObjectSetDef::LinkedExiles(_)
        | ObjectSetDef::CardsDrawnThisTurnInHand(_)
        | ObjectSetDef::BottomOfGraveyard(_)
        | ObjectSetDef::SharingNameWith(_)
        | ObjectSetDef::SharingNameWithBinding { .. }
        | ObjectSetDef::TopOfGraveyardMatching { .. } => false,
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

/// Which predicates a static animation's own query may read.
///
/// The rule is stratification, not a list of favourites. A static animation
/// may ask whether an object is already a creature because CR 613.6 pins a
/// compound effect's recipient set when its layer-4 component starts; its
/// later components do not reselect after the effect supplies Creature. It
/// still may not ask about colour, which an animation can repaint without an
/// earlier component pinning the selection. Everything else below is another
/// card type, a non-land subtype, attachment, or which object is the source.
/// A basic land subtype remains excluded because layer-4 operations supply it.
/// The two `Game::static_*animation_predicate_is_supported` methods are the
/// runtime's copies of this list; both sides are meant to say the same thing.
fn static_animation_predicate_supported(predicate: ObjectPredicateDef, creature: bool) -> bool {
    match predicate {
        ObjectPredicateDef::Subtype(name) => !crate::card::BasicLandType::ALL
            .iter()
            .any(|land_type| land_type.subtype() == name),
        ObjectPredicateDef::Any
        | ObjectPredicateDef::Source
        | ObjectPredicateDef::AttachedToSource
        | ObjectPredicateDef::HasAnyBasicLandType(_)
        | ObjectPredicateDef::HasType(
            CardType::Land | CardType::Enchantment | CardType::Artifact,
        ) => true,
        ObjectPredicateDef::HasType(CardType::Creature) => creature,
        ObjectPredicateDef::All(predicates) | ObjectPredicateDef::AnyOf(predicates) => predicates
            .iter()
            .copied()
            .all(|predicate| static_animation_predicate_supported(predicate, creature)),
        ObjectPredicateDef::Not(predicate) => {
            static_animation_predicate_supported(*predicate, creature)
        }
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
        ObjectPredicateDef::ControlledBy(relation) | ObjectPredicateDef::OwnedBy(relation) => {
            static_player_relation_supported(relation)
        }
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
        | ObjectPredicateDef::Supertype(_)
        | ObjectPredicateDef::DebutSet(_)
        | ObjectPredicateDef::SharesNameWithSource
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
        // A comparison of the object with itself reads nothing the walk
        // supplies, so there is no value to gate on.
        | ObjectPredicateDef::ToughnessGreaterThanItsPower
        | ObjectPredicateDef::EnteredThisTurn
        | ObjectPredicateDef::AttackedDuringControllersLastTurn => true,
    }
}

fn static_trigger_condition_supported(condition: TriggerConditionDef) -> bool {
    match condition {
        TriggerConditionDef::All(conditions) | TriggerConditionDef::AnyOf(conditions) => conditions
            .iter()
            .copied()
            .all(static_trigger_condition_supported),
        TriggerConditionDef::Not(condition) => static_trigger_condition_supported(*condition),
        TriggerConditionDef::ObjectCount { query, .. } => static_query_supported(query),
        TriggerConditionDef::ActivePlayer(relation)
        | TriggerConditionDef::SpellsCastThisTurn {
            player: relation, ..
        }
        | TriggerConditionDef::SpellsCastLastTurn {
            player: relation, ..
        } => static_player_relation_supported(relation),
        TriggerConditionDef::SourceActivationsThisTurn { .. }
        // Both count something about a resolution, which a static walk is
        // not one of.
        | TriggerConditionDef::SourceResolutionsThisTurn { .. }
        | TriggerConditionDef::TargetMatches { .. }
        // And this reads a binding, which only a resolution has.
        | TriggerConditionDef::BoundObjectMatches { .. }
        | TriggerConditionDef::ControlsGreatestPowerCreature => false,
        TriggerConditionDef::SourceMatches { object }
        | TriggerConditionDef::LinkedExilesMatch { object }
        | TriggerConditionDef::AttachedPermanentMatches { object } => {
            static_object_predicate_supported(object)
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
        | TriggerConditionDef::SourceCounters { .. }
        | TriggerConditionDef::SourceCastWith(_)
        | TriggerConditionDef::SourceCastFrom(_)
        | TriggerConditionDef::SourceCastAtInstantSpeed
        | TriggerConditionDef::ValueComparison(_)
        | TriggerConditionDef::SourceLoyalty { .. }
        | TriggerConditionDef::SourceDealtDamageToOpponentThisTurn
        | TriggerConditionDef::SourceIsTapped
        | TriggerConditionDef::SourceIsUntapped
        | TriggerConditionDef::ControllerLifeAtMost(_)
        | TriggerConditionDef::ControllerLifeAtMostHalfStartingLife => true,
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

// A table that only grows: one line per effect, and nothing to factor.
#[allow(clippy::too_many_lines)]
const fn effect_operation_name(effect: EffectDef) -> &'static str {
    match effect {
        EffectDef::None => "None",
        EffectDef::Sequence(_) => "Sequence",
        EffectDef::Randomized { .. } => "Randomized",
        EffectDef::Choose(_) | EffectDef::ChooseCardName { .. } => "Choose",
        EffectDef::ForEachInBinding { .. } => "ForEachInBinding",
        EffectDef::BindMatching { .. } => "BindMatching",
        EffectDef::PayOr(_) => "PayOr",
        EffectDef::SplitIntoPiles(_) => "SplitIntoPiles",
        EffectDef::PreventDamage { .. } => "PreventDamage",
        EffectDef::AddMana(_) => "AddMana",
        EffectDef::AddPoisonCounters { .. } => "AddPoisonCounters",
        EffectDef::AddEnergyCounters { .. } => "AddEnergyCounters",
        EffectDef::DealDamage { .. } => "DealDamage",
        EffectDef::DealDamageFrom { .. } => "DealDamageFrom",
        EffectDef::DealDamageAndApply { .. } => "DealDamageAndApply",
        EffectDef::GainLife { .. } => "GainLife",
        EffectDef::DrawCards { .. } => "DrawCards",
        EffectDef::ShuffleLibrary { .. } => "ShuffleLibrary",
        EffectDef::BuryGraveyard { .. } => "BuryGraveyard",
        EffectDef::EmptyManaPool { .. } => "EmptyManaPool",
        EffectDef::Discard { .. } => "Discard",
        EffectDef::DiscardCards { .. } => "DiscardCards",
        EffectDef::LoseLife { .. } => "LoseLife",
        EffectDef::LoseTheGame { .. } => "LoseTheGame",
        EffectDef::WinTheGame { .. } => "WinTheGame",
        EffectDef::Tap { .. } => "Tap",
        EffectDef::RemoveFromCombat { .. } => "RemoveFromCombat",
        EffectDef::Untap { .. } => "Untap",
        EffectDef::Saddle { .. } => "Saddle",
        EffectDef::CreateToken { .. } => "CreateToken",
        EffectDef::CreateAttachedToken { .. } => "CreateAttachedToken",
        EffectDef::CreateTokenCopyOf { .. } => "CreateTokenCopyOf",
        EffectDef::Endure { .. } => "Endure",
        EffectDef::CreateMyriadTokens => "CreateMyriadTokens",
        EffectDef::Attach { .. }
        | EffectDef::AttachToSource { .. }
        | EffectDef::PhaseOut { .. }
        | EffectDef::ReturnAttached { .. } => "Attach",
        EffectDef::Reconfigure { .. } => "Reconfigure",
        EffectDef::Unattach { .. } => "Unattach",
        EffectDef::PairWithSource { .. } => "PairWithSource",
        EffectDef::Destroy { .. } => "Destroy",
        EffectDef::Detain { .. } => "Detain",
        EffectDef::Regenerate { .. } => "Regenerate",
        EffectDef::Sacrifice { .. } => "Sacrifice",
        EffectDef::SimultaneousChoose(_) => "SimultaneousChoose",
        EffectDef::SacrificeOfChoice { .. } => "SacrificeOfChoice",
        EffectDef::ExileTopOfLibraryToPlay { .. } => "ExileTopOfLibraryToPlay",
        EffectDef::ExileAtRandomFromGraveyardToPlay { .. } => "ExileAtRandomFromGraveyardToPlay",
        EffectDef::ExileTopAndMayCast { .. } => "ExileTopAndMayCast",
        EffectDef::MayCastTargetWithoutPaying { .. } => "MayCastTargetWithoutPaying",
        EffectDef::Mill { .. } => "Mill",
        EffectDef::SearchZonesAndExileRest { .. } => "SearchZonesAndExileRest",
        EffectDef::MillUntil { .. } => "MillUntil",
        EffectDef::ExileFromTopUntil { .. } => "ExileFromTopUntil",
        EffectDef::ManifestDread { .. } => "ManifestDread",
        EffectDef::Cascade => "Cascade",
        EffectDef::Proliferate => "Proliferate",
        EffectDef::Explore { .. } => "Explore",
        EffectDef::LookAtHand { .. } => "LookAtHand",
        EffectDef::LookAtRandomCardInHand { .. } => "LookAtRandomCardInHand",
        EffectDef::RevealHand { .. } => "RevealHand",
        EffectDef::RevealAtRandomFromHand { .. } => "RevealAtRandomFromHand",
        EffectDef::LookAtTopAndSelect { .. } => "LookAtTopAndSelect",
        EffectDef::SearchZone { .. } => "SearchZone",
        EffectDef::ChooseCards { .. } => "ChooseCards",
        EffectDef::ReplaceNextDrawThisTurn { .. } => "ReplaceNextDrawThisTurn",
        EffectDef::IfFormat { .. } => "IfFormat",
        EffectDef::Counter { .. } | EffectDef::CopyResolvingSpell { .. } => "Counter",
        EffectDef::ReturnSpellToHand { .. } => "ReturnSpellToHand",
        EffectDef::PutSpellIntoOwnersLibrary { .. } => "PutSpellIntoOwnersLibrary",
        EffectDef::DrainLife { .. } => "DrainLife",
        EffectDef::AddManaEqualTo { .. } => "AddManaEqualTo",
        EffectDef::AddCounters { .. } => "AddCounters",
        EffectDef::RemoveCounters { .. } => "RemoveCounters",
        EffectDef::DoubleCounters { .. } => "DoubleCounters",
        EffectDef::RemoveAllCounters { .. } => "RemoveAllCounters",
        EffectDef::SkipNextUntapSteps { .. } => "SkipNextUntapSteps",
        EffectDef::ChangeTextBasicLandType { .. } => "ChangeTextBasicLandType",
        EffectDef::ChooseColor { .. } => "ChooseColor",
        EffectDef::BecomeCopyOf { .. } => "BecomeCopyOf",
        EffectDef::PutSourceOntoBattlefieldAttacking => "PutSourceOntoBattlefieldAttacking",
        EffectDef::BecomeMonarch { .. } => "BecomeMonarch",
        EffectDef::VoteForPermanentToExile { .. } => "VoteForPermanentToExile",
        EffectDef::DamageCannotBePreventedThisTurn => "DamageCannotBePreventedThisTurn",
        EffectDef::GrantFlashToNextSorcery => "GrantFlashToNextSorcery",
        EffectDef::May { .. } => "May",
        EffectDef::ExileLinkedToSource { .. } => "ExileLinkedToSource",
        EffectDef::MayPlayWithoutPaying { .. } => "MayPlayWithoutPaying",
        EffectDef::ExileGrantingOwnerPlay { .. } => "ExileGrantingOwnerPlay",
        EffectDef::ReturnLinkedExiles { .. } => "ReturnLinkedExiles",
        EffectDef::GainControl { .. } | EffectDef::ExchangeControl { .. } => "GainControl",
        EffectDef::IfCondition { .. } => "IfCondition",
        EffectDef::InstallTrigger(_) => "InstallTrigger",
        EffectDef::CreateOngoingEffect(_) => "CreateOngoingEffect",
        EffectDef::CannotBeForcedToSacrifice => "CannotBeForcedToSacrifice",
        EffectDef::CannotBeForcedToDiscard => "CannotBeForcedToDiscard",
        EffectDef::GainClassLevel { .. } => "GainClassLevel",
        EffectDef::SubstituteBasicLandTypeUntilEndOfTurn { .. } => {
            "SubstituteBasicLandTypeUntilEndOfTurn"
        }
        EffectDef::ReduceGenericCostBy(_) => "ReduceGenericCostBy",
        EffectDef::ModifyCost(_) => "ModifyCost",
        EffectDef::CannotAttackUnless(_) => "CannotAttackUnless",
        EffectDef::CannotAttackIf(_) => "CannotAttackIf",
        EffectDef::LandwalkCanBeBlocked(_) => "LandwalkCanBeBlocked",
        EffectDef::ScheduleTurnPhases(_) => "ScheduleTurnPhases",
        EffectDef::TakeExtraTurn { .. } => "TakeExtraTurn",
        EffectDef::CreateEmblem { .. } => "CreateEmblem",
        EffectDef::PutOntoBattlefieldThen { .. } => "PutOntoBattlefieldThen",
        EffectDef::ReturnWithHasteAndFinality { .. } => "ReturnWithHasteAndFinality",
        EffectDef::Transform { .. } => "Transform",
        EffectDef::PutIntoLibraryBeneathTop { .. } => "PutIntoLibraryBeneathTop",
        EffectDef::MoveToZone { .. } => "MoveToZone",
        EffectDef::StaticApply { .. } => "StaticApply",
        EffectDef::Apply { .. } => "Apply",
        EffectDef::Special(_) => "Special",
    }
}

include!("program_context/static_values.rs");
