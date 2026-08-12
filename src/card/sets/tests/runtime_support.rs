mod nested_definitions;
mod stack_effects;

pub(super) use nested_definitions::*;
pub(super) use stack_effects::shared_stack_effect;

use crate::card::ReplacementConditionDef;

use super::*;

pub(super) fn shared_object_predicate(predicate: ObjectPredicateDef) -> bool {
    match predicate {
        ObjectPredicateDef::All(predicates) | ObjectPredicateDef::AnyOf(predicates) => {
            predicates.iter().copied().all(shared_object_predicate)
        }
        ObjectPredicateDef::Not(predicate) => shared_object_predicate(*predicate),
        ObjectPredicateDef::Special(_) => false,
        ObjectPredicateDef::Any
        | ObjectPredicateDef::Source
        | ObjectPredicateDef::Token
        | ObjectPredicateDef::HasType(_)
        | ObjectPredicateDef::HasAnyBasicLandType(_)
        | ObjectPredicateDef::Spell
        | ObjectPredicateDef::NoncreatureSpell
        | ObjectPredicateDef::Color(_)
        | ObjectPredicateDef::ColorCount(_)
        | ObjectPredicateDef::Subtype(_)
        | ObjectPredicateDef::ManaValueAtMost(_)
        | ObjectPredicateDef::ManaValueEqualTo(_)
        | ObjectPredicateDef::ManaValueAtMostValue(_)
        | ObjectPredicateDef::PowerAtLeast(_)
        | ObjectPredicateDef::PowerExactly(_)
        | ObjectPredicateDef::ToughnessExactly(_)
        | ObjectPredicateDef::ToughnessLessThan(_)
        | ObjectPredicateDef::ControlledBy(_)
        | ObjectPredicateDef::Supertype(_)
        | ObjectPredicateDef::DebutSet(_)
        | ObjectPredicateDef::SharesNameWithSource
        | ObjectPredicateDef::AttackingOrBlocking
        | ObjectPredicateDef::HasKeyword(_)
        | ObjectPredicateDef::HasNonManaActivatedAbility
        | ObjectPredicateDef::Attacking
        | ObjectPredicateDef::AttackedThisTurn => true,
    }
}

pub(super) fn shared_effect_recipient(recipient: EffectRecipientDef) -> bool {
    match recipient {
        EffectRecipientDef::MatchingObjects { object, zones, .. } => {
            !zones.is_empty()
                && zones.iter().all(|zone| {
                    matches!(
                        zone,
                        ZoneKind::Battlefield
                            | ZoneKind::Stack
                            | ZoneKind::Library
                            | ZoneKind::Hand
                            | ZoneKind::Graveyard
                            | ZoneKind::Exile
                            | ZoneKind::Command
                    )
                })
                && shared_object_predicate(object)
        }
        // The sweep is over the battlefield, so only the predicate
        // needs checking.
        EffectRecipientDef::ObjectsControlledByTarget { object, .. }
        | EffectRecipientDef::ObjectsOwnedByTarget { object, .. } => {
            shared_object_predicate(object)
        }
        EffectRecipientDef::CardsOwnedByTarget { object, zones, .. } => {
            !zones.is_empty()
                && zones.iter().all(|zone| {
                    matches!(
                        zone,
                        ZoneKind::Library | ZoneKind::Hand | ZoneKind::Graveyard | ZoneKind::Exile
                    )
                })
                && shared_object_predicate(object)
        }
        EffectRecipientDef::ObjectsSharingNameWithTarget(_)
        | EffectRecipientDef::Source
        | EffectRecipientDef::ChosenPermanent(_)
        | EffectRecipientDef::AttachedPermanent
        | EffectRecipientDef::Controller
        | EffectRecipientDef::Opponent
        | EffectRecipientDef::EachPlayer
        | EffectRecipientDef::Target(_)
        | EffectRecipientDef::ControllerOfTarget(_)
        | EffectRecipientDef::TriggeringObject
        | EffectRecipientDef::ControllerOfTriggeringObject
        | EffectRecipientDef::EventPlayer => true,
    }
}

pub(super) fn shared_keyword(keyword: KeywordAbility) -> bool {
    matches!(
        keyword,
        KeywordAbility::Flying
            | KeywordAbility::Trample
            | KeywordAbility::Haste
            | KeywordAbility::FirstStrike
            | KeywordAbility::DoubleStrike
            | KeywordAbility::Vigilance
            | KeywordAbility::Defender
            | KeywordAbility::Deathtouch
            | KeywordAbility::Lifelink
            | KeywordAbility::Reach
            | KeywordAbility::Flash
            | KeywordAbility::Hexproof
            | KeywordAbility::Shroud
            | KeywordAbility::Intimidate
            | KeywordAbility::Undying
            | KeywordAbility::Indestructible
            | KeywordAbility::Mountainwalk
            | KeywordAbility::Forestwalk
            | KeywordAbility::AttacksEachCombatIfAble
            | KeywordAbility::ProtectionFrom(_)
    )
}

pub(super) fn shared_zone_move_cause(cause: ZoneMoveCauseDef) -> bool {
    matches!(
        cause,
        ZoneMoveCauseDef::Any
            | ZoneMoveCauseDef::EffectControlledBy(
                PlayerRelation::Any
                    | PlayerRelation::You
                    | PlayerRelation::Opponent
                    | PlayerRelation::ActivePlayer
                    | PlayerRelation::NonactivePlayer
            )
    )
}

pub(super) fn shared_cannot_be_countered_effect(effect: AppliedEffectDef) -> bool {
    match effect {
        AppliedEffectDef::Composite(effects) => {
            !effects.is_empty()
                && effects
                    .iter()
                    .copied()
                    .all(shared_cannot_be_countered_effect)
        }
        AppliedEffectDef::CannotBeCountered | AppliedEffectDef::CannotBeEnchanted => true,
        AppliedEffectDef::ModifyPowerToughness { .. }
        | AppliedEffectDef::DoesNotUntapDuringUntapStep
        | AppliedEffectDef::CannotBecomeEnchanted
        | AppliedEffectDef::CannotChangeController
        | AppliedEffectDef::CannotBeBlockedBy(_)
        | AppliedEffectDef::PreventDamageFrom(_)
        | AppliedEffectDef::AddLandTypes(_)
        | AppliedEffectDef::SetLandTypes(_)
        | AppliedEffectDef::RemoveAbilities(_)
        | AppliedEffectDef::Animate(_)
        | AppliedEffectDef::GrantAbility(_)
        | AppliedEffectDef::Special(_) => false,
    }
}

pub(super) fn shared_mana_effect(effect: EffectDef, choices_are_supported: bool) -> bool {
    let EffectDef::AddMana(mana) = effect else {
        return false;
    };
    let selection_is_supported = match mana.mana {
        ManaSelectionDef::One(_) => true,
        ManaSelectionDef::Choice(colors) => choices_are_supported && !colors.is_empty(),
    };
    selection_is_supported
        && mana.amount > 0
        && mana
            .restrictions
            .iter()
            .copied()
            .all(|restriction| match restriction {
                ManaRestrictionDef::CastSpell(object) => shared_object_predicate(object),
                ManaRestrictionDef::CastCreatureSpellOfChosenType => true,
                ManaRestrictionDef::ActivateAbility(_) | ManaRestrictionDef::Special(_) => false,
            })
        && mana.spend_effects.iter().copied().all(|effect| {
            let ManaSpendEffectDef::ApplyToPaidSpell(effect) = effect else {
                return false;
            };
            shared_cannot_be_countered_effect(effect)
        })
}

pub(super) fn shared_resolving_apply(
    recipient: EffectRecipientDef,
    effect: AppliedEffectDef,
    duration: EffectDurationDef,
) -> bool {
    // Resolved ability additions and removals share one duration-aware
    // operation path. Other applied effects still end with the turn.
    let ability_change = resolving_effect_is_only_ability_changes(effect);
    let duration_is_supported = duration == EffectDurationDef::UntilEndOfTurn
        || duration == EffectDurationDef::UntilYourNextUpkeep && ability_change
        || matches!(
            duration,
            EffectDurationDef::UntilYourNextTurn | EffectDurationDef::Permanent
        ) && ability_change;
    if !duration_is_supported || !shared_effect_recipient(recipient) {
        return false;
    }
    shared_resolving_applied_effect(effect)
}

fn resolving_effect_is_only_ability_changes(effect: AppliedEffectDef) -> bool {
    match effect {
        AppliedEffectDef::Composite(effects) => {
            !effects.is_empty()
                && effects
                    .iter()
                    .copied()
                    .all(resolving_effect_is_only_ability_changes)
        }
        AppliedEffectDef::GrantAbility(_) | AppliedEffectDef::RemoveAbilities(_) => true,
        AppliedEffectDef::CannotBeCountered
        | AppliedEffectDef::DoesNotUntapDuringUntapStep
        | AppliedEffectDef::CannotBeEnchanted
        | AppliedEffectDef::CannotBecomeEnchanted
        | AppliedEffectDef::CannotChangeController
        | AppliedEffectDef::CannotBeBlockedBy(_)
        | AppliedEffectDef::PreventDamageFrom(_)
        | AppliedEffectDef::AddLandTypes(_)
        | AppliedEffectDef::SetLandTypes(_)
        | AppliedEffectDef::Animate(_)
        | AppliedEffectDef::ModifyPowerToughness { .. }
        | AppliedEffectDef::Special(_) => false,
    }
}

pub(super) fn shared_resolving_applied_effect(effect: AppliedEffectDef) -> bool {
    match effect {
        AppliedEffectDef::Composite(effects) => {
            !effects.is_empty() && effects.iter().copied().all(shared_resolving_applied_effect)
        }
        // These operations are executed directly by the shared apply
        // path; animation reads the whole creature off the definition.
        AppliedEffectDef::Animate(_)
        | AppliedEffectDef::ModifyPowerToughness { .. }
        | AppliedEffectDef::RemoveAbilities(_) => true,
        AppliedEffectDef::GrantAbility(ability) => match ability.definition {
            DeclarativeAbilityDef::ActivatedMana(definition)
            | DeclarativeAbilityDef::Activated(definition) => {
                battlefield_only(definition.source_zones) && shared_definition_ability(ability)
            }
            DeclarativeAbilityDef::TriggeredMana(_)
            | DeclarativeAbilityDef::Triggered(_)
            | DeclarativeAbilityDef::Replacement(_)
            | DeclarativeAbilityDef::Keyword(_) => shared_definition_ability(ability),
            DeclarativeAbilityDef::AlternativeCast(definition) => {
                definition.kind == AlternativeCastKindDef::Flashback
                    && ability.declarative_effect() == Some(EffectDef::None)
            }
            DeclarativeAbilityDef::Spell(_)
            | DeclarativeAbilityDef::Static(_)
            | DeclarativeAbilityDef::SpecialAction(_)
            | DeclarativeAbilityDef::Legacy => false,
        },
        // A blocking restriction is continuous, not an until-end-of-turn
        // rider a spell hands out.
        AppliedEffectDef::CannotBeCountered
        | AppliedEffectDef::DoesNotUntapDuringUntapStep
        | AppliedEffectDef::CannotBeEnchanted
        | AppliedEffectDef::CannotBecomeEnchanted
        | AppliedEffectDef::CannotChangeController
        | AppliedEffectDef::CannotBeBlockedBy(_)
        | AppliedEffectDef::PreventDamageFrom(_)
        | AppliedEffectDef::AddLandTypes(_)
        | AppliedEffectDef::SetLandTypes(_)
        | AppliedEffectDef::Special(_) => false,
    }
}

pub(super) fn shared_activated_costs(source_zones: &[ZoneKind], costs: &[AbilityCostDef]) -> bool {
    let battlefield = source_zones == [ZoneKind::Battlefield];
    let hand = source_zones == [ZoneKind::Hand];
    let sacrifice_choices = costs
        .iter()
        .filter(|cost| matches!(cost, AbilityCostDef::SacrificePermanent { .. }))
        .count();
    let source_exit_costs = costs
        .iter()
        .filter(|cost| {
            matches!(
                cost,
                AbilityCostDef::SacrificeSource | AbilityCostDef::ExileSource
            )
        })
        .count();
    sacrifice_choices <= 1
        && source_exit_costs <= 1
        && costs.iter().all(|cost| match cost {
            // A variable X is offered one activation per affordable
            // value. More than one X in the same cost is not: nothing
            // enumerates a cost that charges X twice.
            AbilityCostDef::Mana(cost) => cost.x_multiplier <= 1,
            // The chosen object comes from the battlefield or from the
            // activating player's own graveyard, so only the predicate
            // needs checking.
            AbilityCostDef::SacrificePermanent { object, .. }
            | AbilityCostDef::ExileCardFromGraveyard(object) => {
                battlefield && shared_object_predicate(*object)
            }
            AbilityCostDef::TapSource
            | AbilityCostDef::SacrificeSource
            | AbilityCostDef::ExileSource
            | AbilityCostDef::RemoveCountersFromSource { .. }
            | AbilityCostDef::PayLife(_)
            | AbilityCostDef::Loyalty(_) => battlefield,
            AbilityCostDef::DiscardSource => hand,
            AbilityCostDef::UntapSource
            | AbilityCostDef::DiscardCards(_)
            | AbilityCostDef::Special(_) => false,
        })
}

/// The two static effects that are not an `Apply`: a prohibition read off
/// the battlefield, and a cost reduction read out of hand.
pub(super) fn shared_static_non_apply_effect(source_zones: &[ZoneKind], effect: EffectDef) -> bool {
    match effect {
        EffectDef::CannotBeForcedToSacrifice => battlefield_only(source_zones),
        // The prohibition is read off the battlefield while play options
        // are offered, and only against a card's printed shape.
        EffectDef::PlayersCantPlay(predicate) => {
            battlefield_only(source_zones) && shared_object_predicate(*predicate)
        }
        EffectDef::ReduceGenericCostBy(value) => {
            source_zones == [ZoneKind::Hand]
                && matches!(
                    value,
                    crate::card::ValueDef::Constant(_)
                        | crate::card::ValueDef::CountMatchingObjects(_)
                )
        }
        EffectDef::Sequence(effects) => {
            !effects.is_empty()
                && effects
                    .iter()
                    .copied()
                    .all(|effect| shared_static_effect(source_zones, effect))
        }
        _ => false,
    }
}

#[allow(clippy::too_many_lines)]
pub(super) fn shared_static_effect(source_zones: &[ZoneKind], effect: EffectDef) -> bool {
    match effect {
        EffectDef::CannotBeForcedToSacrifice
        | EffectDef::ReduceGenericCostBy(_)
        | EffectDef::PlayersCantPlay(_)
        | EffectDef::Sequence(_) => shared_static_non_apply_effect(source_zones, effect),
        EffectDef::Apply {
            recipient,
            effect,
            duration,
        } => {
            let battlefield_recipient_is_supported = match recipient {
                EffectRecipientDef::Source | EffectRecipientDef::AttachedPermanent => true,
                EffectRecipientDef::MatchingObjects { object, zones, .. } => {
                    zones == [ZoneKind::Battlefield] && shared_object_predicate(object)
                }
                EffectRecipientDef::Controller
                | EffectRecipientDef::Opponent
                | EffectRecipientDef::EachPlayer
                | EffectRecipientDef::ChosenPermanent(_)
                | EffectRecipientDef::Target(_)
                | EffectRecipientDef::ControllerOfTarget(_)
                | EffectRecipientDef::ObjectsControlledByTarget { .. }
                | EffectRecipientDef::ObjectsOwnedByTarget { .. }
                | EffectRecipientDef::CardsOwnedByTarget { .. }
                | EffectRecipientDef::ObjectsSharingNameWithTarget(_)
                | EffectRecipientDef::TriggeringObject
                | EffectRecipientDef::ControllerOfTriggeringObject
                | EffectRecipientDef::EventPlayer => false,
            };
            let battlefield_effect_is_supported = shared_static_applied_effect(recipient, effect);
            let battlefield_effect = battlefield_only(source_zones)
                && battlefield_recipient_is_supported
                && battlefield_effect_is_supported
                && matches!(
                    duration,
                    EffectDurationDef::WhileSourceRemainsInZone
                        | EffectDurationDef::UntilSourceLeavesZone
                );
            let stack_source_effect = source_zones == [ZoneKind::Stack]
                && recipient == EffectRecipientDef::Source
                && shared_cannot_be_countered_effect(effect)
                && duration == EffectDurationDef::WhileSourceRemainsInZone;
            battlefield_effect || stack_source_effect
        }
        EffectDef::IfCondition { condition, then } => {
            battlefield_only(source_zones)
                && shared_static_trigger_condition(*condition)
                && shared_static_effect(source_zones, *then)
        }
        // None of these is a static ability; all execute from the stack.
        EffectDef::GrantFlashToNextSorcery
        | EffectDef::Randomized { .. }
        | EffectDef::ChoosePermanent { .. }
        | EffectDef::May { .. }
        | EffectDef::ExileLinkedToSource { .. }
        | EffectDef::ReturnLinkedExiles { .. }
        | EffectDef::MakeUnblockableThisTurn { .. }
        | EffectDef::GainControlThisTurn { .. }
        | EffectDef::AtNextStep { .. }
        | EffectDef::TriggerUntilYourNextTurn { .. }
        | EffectDef::None
        | EffectDef::AddMana(_)
        | EffectDef::AddManaEqualTo { .. }
        | EffectDef::DealDamage { .. }
        | EffectDef::DrainLife { .. }
        | EffectDef::GainLife { .. }
        | EffectDef::DrawCards { .. }
        | EffectDef::Discard { .. }
        | EffectDef::ShuffleLibrary { .. }
        | EffectDef::EmptyManaPool { .. }
        | EffectDef::LoseLife { .. }
        | EffectDef::LoseTheGame { .. }
        | EffectDef::Tap { .. }
        | EffectDef::Untap { .. }
        | EffectDef::PreventCombatDamageThisTurn { .. }
        | EffectDef::PreventCombatDamageDealtByThisTurn { .. }
        | EffectDef::Attach { .. }
        | EffectDef::CreateToken { .. }
        | EffectDef::Destroy { .. }
        | EffectDef::Sacrifice { .. }
        | EffectDef::SacrificeOfChoice { .. }
        | EffectDef::DestroyOfChoice { .. }
        | EffectDef::SplitPermanentsAndSacrificeAPile { .. }
        | EffectDef::RevealAndSplitIntoPiles { .. }
        | EffectDef::Mill { .. }
        | EffectDef::LookAtTopAndMayTake { .. }
        | EffectDef::LookAtTopAndSelect { .. }
        | EffectDef::LookAtHand { .. }
        | EffectDef::SearchZone { .. }
        | EffectDef::ChooseCards { .. }
        | EffectDef::ReplaceNextDrawThisTurn { .. }
        | EffectDef::IfFormat { .. }
        | EffectDef::Counter { .. }
        | EffectDef::CounterUnlessPaid { .. }
        | EffectDef::AddCounters { .. }
        | EffectDef::ChangeTextBasicLandType { .. }
        | EffectDef::BecomeCopyOf { .. }
        | EffectDef::OptionalPayment { .. }
        | EffectDef::UnlessPaid { .. }
        | EffectDef::MultiplyEventAmount(_)
        | EffectDef::Replacement(_)
        | EffectDef::MoveToZone { .. }
        | EffectDef::ChooseCardName { .. }
        | EffectDef::ChoosePlayer { .. }
        | EffectDef::CopyPermanentAsItEnters { .. }
        | EffectDef::ChooseCreatureType { .. }
        | EffectDef::CreateEmblem { .. }
        | EffectDef::Transform { .. }
        | EffectDef::AdditionalCombatPhase
        | EffectDef::TakeExtraTurn { .. }
        | EffectDef::CannotCastNoncreatureSpellsThisTurn { .. }
        | EffectDef::Special(_) => false,
    }
}

pub(super) fn shared_static_applied_effect(
    recipient: EffectRecipientDef,
    effect: AppliedEffectDef,
) -> bool {
    match effect {
        AppliedEffectDef::Composite(effects) => {
            !effects.is_empty()
                && effects
                    .iter()
                    .copied()
                    .all(|effect| shared_static_applied_effect(recipient, effect))
        }
        AppliedEffectDef::ModifyPowerToughness { power, toughness } => {
            let supported = |value| {
                matches!(
                    value,
                    crate::card::ValueDef::Constant(_)
                        | crate::card::ValueDef::AnyMatchingObject(_)
                        | crate::card::ValueDef::CountMatchingObjects(_)
                )
            };
            supported(power) && supported(toughness)
        }
        AppliedEffectDef::AddLandTypes(land_types) | AppliedEffectDef::SetLandTypes(land_types) => {
            !land_types.is_empty()
        }
        AppliedEffectDef::GrantAbility(ability) => shared_definition_ability(ability),
        AppliedEffectDef::CannotBeBlockedBy(predicate)
        | AppliedEffectDef::PreventDamageFrom(predicate) => {
            matches!(
                recipient,
                EffectRecipientDef::Source | EffectRecipientDef::AttachedPermanent
            ) && shared_object_predicate(predicate)
        }
        AppliedEffectDef::DoesNotUntapDuringUntapStep
        | AppliedEffectDef::RemoveAbilities(_)
        | AppliedEffectDef::CannotBeCountered
        | AppliedEffectDef::CannotBeEnchanted
        | AppliedEffectDef::CannotBecomeEnchanted
        | AppliedEffectDef::CannotChangeController => true,
        // Only a resolving animation is supported; nothing reads one off
        // a static ability.
        AppliedEffectDef::Animate(_) | AppliedEffectDef::Special(_) => false,
    }
}

pub(super) fn shared_trigger_condition(condition: TriggerConditionDef) -> bool {
    match condition {
        TriggerConditionDef::ObjectCount { query, .. } => shared_object_predicate(query.object),
        TriggerConditionDef::TargetMatches { object, .. } => shared_object_predicate(object),
        TriggerConditionDef::SourceOnBattlefield
        | TriggerConditionDef::SourceUntapped
        | TriggerConditionDef::ActivePlayer(_)
        | TriggerConditionDef::SourceLoyalty { .. }
        | TriggerConditionDef::SourceActivationsThisTurn { .. }
        | TriggerConditionDef::SourceDealtDamageToOpponentThisTurn
        | TriggerConditionDef::SourceIsTapped
        | TriggerConditionDef::SpellsCastLastTurn { .. } => true,
    }
}

/// Static effects have a battlefield source but no captured trigger event,
/// resolving ability, or stack-target scope. Keep their condition boundary to
/// the source-state predicates that can be evaluated from exactly that input.
fn shared_static_trigger_condition(condition: TriggerConditionDef) -> bool {
    matches!(
        condition,
        TriggerConditionDef::SourceOnBattlefield | TriggerConditionDef::SourceUntapped
    )
}

pub(super) fn battlefield_only(zones: &[ZoneKind]) -> bool {
    zones == [ZoneKind::Battlefield]
}

#[allow(clippy::too_many_lines)]
pub(super) fn shared_definition_ability(ability: &AbilityDef) -> bool {
    let Some(effect) = ability.declarative_effect() else {
        return false;
    };
    match ability.definition {
        DeclarativeAbilityDef::Spell(definition) => {
            if let Some(modal) = definition.modal() {
                modal.modes.iter().all(|mode| {
                    mode.declarative_effect().is_none() || shared_definition_ability(mode)
                })
            } else {
                shared_stack_effect(effect)
            }
        }
        DeclarativeAbilityDef::ActivatedMana(definition) => {
            battlefield_only(definition.source_zones)
                && definition.procedure == AbilityProcedureDef::Shared
                && !definition.costs.as_slice().is_empty()
                && definition.costs.iter().all(|cost| {
                    matches!(
                        cost,
                        AbilityCostDef::TapSource
                            | AbilityCostDef::SacrificeSource
                            | AbilityCostDef::ExileSource
                            | AbilityCostDef::RemoveCountersFromSource { .. }
                            | AbilityCostDef::PayLife(_)
                    )
                })
                && definition
                    .costs
                    .iter()
                    .filter(|cost| {
                        matches!(
                            cost,
                            AbilityCostDef::SacrificeSource | AbilityCostDef::ExileSource
                        )
                    })
                    .count()
                    <= 1
                && shared_mana_effect(effect, true)
        }
        DeclarativeAbilityDef::TriggeredMana(definition) => {
            fn immediate_mana_effect(effect: EffectDef) -> bool {
                match effect {
                    EffectDef::Sequence(effects) => {
                        !effects.is_empty() && effects.iter().copied().all(immediate_mana_effect)
                    }
                    EffectDef::AddMana(_) => shared_mana_effect(effect, false),
                    // A mana ability's amount has to be knowable without
                    // resolving it, which this one is not.
                    EffectDef::AddManaEqualTo { .. }
                    | EffectDef::Randomized { .. }
                    | EffectDef::ChoosePermanent { .. }
                    | EffectDef::May { .. }
                    | EffectDef::None
                    | EffectDef::DealDamage { .. }
                    | EffectDef::DrainLife { .. }
                    | EffectDef::GainLife { .. }
                    | EffectDef::DrawCards { .. }
                    | EffectDef::Discard { .. }
                    | EffectDef::ShuffleLibrary { .. }
                    | EffectDef::EmptyManaPool { .. }
                    | EffectDef::LoseLife { .. }
                    | EffectDef::LoseTheGame { .. }
                    | EffectDef::Tap { .. }
                    | EffectDef::Untap { .. }
                    | EffectDef::PreventCombatDamageThisTurn { .. }
                    | EffectDef::PreventCombatDamageDealtByThisTurn { .. }
                    | EffectDef::Attach { .. }
                    | EffectDef::CreateToken { .. }
                    | EffectDef::Destroy { .. }
                    | EffectDef::Sacrifice { .. }
                    | EffectDef::SacrificeOfChoice { .. }
                    | EffectDef::DestroyOfChoice { .. }
                    | EffectDef::SplitPermanentsAndSacrificeAPile { .. }
                    | EffectDef::RevealAndSplitIntoPiles { .. }
                    | EffectDef::Mill { .. }
                    | EffectDef::LookAtTopAndMayTake { .. }
                    | EffectDef::LookAtTopAndSelect { .. }
                    | EffectDef::LookAtHand { .. }
                    | EffectDef::SearchZone { .. }
                    | EffectDef::ChooseCards { .. }
                    | EffectDef::ReplaceNextDrawThisTurn { .. }
                    | EffectDef::IfFormat { .. }
                    | EffectDef::Counter { .. }
                    | EffectDef::CounterUnlessPaid { .. }
                    | EffectDef::AddCounters { .. }
                    | EffectDef::ChangeTextBasicLandType { .. }
                    | EffectDef::BecomeCopyOf { .. }
                    | EffectDef::OptionalPayment { .. }
                    | EffectDef::UnlessPaid { .. }
                    | EffectDef::CannotBeForcedToSacrifice
                    | EffectDef::CreateEmblem { .. }
                    | EffectDef::Transform { .. }
                    | EffectDef::AdditionalCombatPhase
                    | EffectDef::TakeExtraTurn { .. }
                    | EffectDef::CannotCastNoncreatureSpellsThisTurn { .. }
                    | EffectDef::GrantFlashToNextSorcery
                    | EffectDef::ExileLinkedToSource { .. }
                    | EffectDef::ReturnLinkedExiles { .. }
                    | EffectDef::MakeUnblockableThisTurn { .. }
                    | EffectDef::GainControlThisTurn { .. }
                    | EffectDef::AtNextStep { .. }
                    | EffectDef::IfCondition { .. }
                    | EffectDef::TriggerUntilYourNextTurn { .. }
                    | EffectDef::ReduceGenericCostBy(_)
                    | EffectDef::PlayersCantPlay(_)
                    | EffectDef::MultiplyEventAmount(_)
                    | EffectDef::Replacement(_)
                    | EffectDef::MoveToZone { .. }
                    | EffectDef::ChooseCardName { .. }
                    | EffectDef::ChoosePlayer { .. }
                    | EffectDef::CopyPermanentAsItEnters { .. }
                    | EffectDef::ChooseCreatureType { .. }
                    | EffectDef::Apply { .. }
                    | EffectDef::Special(_) => false,
                }
            }
            definition.condition.is_none()
                && definition.procedure == AbilityProcedureDef::Shared
                && battlefield_only(definition.source_zones)
                && shared_trigger_event(definition.event)
                && immediate_mana_effect(effect)
        }
        DeclarativeAbilityDef::Activated(definition) => {
            matches!(
                    definition.source_zones,
                    [ZoneKind::Battlefield | ZoneKind::Hand]
                ) && definition.procedure == AbilityProcedureDef::Shared
                    && shared_activated_costs(definition.source_zones, definition.costs.as_slice())
                    // An activation enumerates its targets once for every
                    // affordable X, so a slot dividing X has no enumeration
                    // to live in yet.
                    && definition
                        .targets
                        .iter()
                        .all(|slot| slot.divided_total.is_none())
                    && shared_stack_effect(effect)
        }
        DeclarativeAbilityDef::Triggered(definition) => {
            // A state trigger is nothing but its condition: without one it
            // would trigger on every state-based check forever.
            let condition_is_required = definition.event != TriggerEventDef::StateCondition
                || definition.condition.is_some();
            battlefield_only(definition.source_zones)
                && definition.procedure == AbilityProcedureDef::Shared
                && shared_trigger_event(definition.event)
                && condition_is_required
                && definition
                    .condition
                    .is_none_or(|condition| shared_trigger_condition(*condition))
                && shared_stack_effect(effect)
        }
        DeclarativeAbilityDef::Static(definition) => {
            (effect == EffectDef::None
                && ability.coverage.status == ImplementationStatus::Complete
                && ability.coverage.explanation.is_some())
                || shared_static_effect(definition.source_zones, effect)
        }
        DeclarativeAbilityDef::Replacement(definition) => match definition.event {
            ReplacementEventDef::SourceEntersBattlefield
            | ReplacementEventDef::ObjectEntersBattlefield { .. } => {
                !definition.optional
                    && definition.condition.is_none()
                    && battlefield_only(definition.source_zones)
                    && shared_replacement_event(definition.event)
                    && matches!(effect, EffectDef::Replacement(effect) if shared_entry_replacement_effect(effect))
            }
            ReplacementEventDef::EntersBattlefield => {
                !definition.optional
                    && definition.condition.is_none()
                    && battlefield_only(definition.source_zones)
                    && matches!(
                        effect,
                        EffectDef::ChooseCreatureType {
                            object: EffectRecipientDef::Source,
                        } | EffectDef::ChooseCardName {
                            object: EffectRecipientDef::Source,
                        } | EffectDef::ChoosePlayer {
                            object: EffectRecipientDef::Source,
                            ..
                        } | EffectDef::CopyPermanentAsItEnters { .. }
                    )
            }
            ReplacementEventDef::WouldMove { from, to, cause } => {
                !definition.optional
                    && definition.condition.is_none()
                    && definition.source_zones == [from]
                    && ((from == ZoneKind::Hand
                        && to == ZoneKind::Graveyard
                        && shared_zone_move_cause(cause)
                        && effect
                            == EffectDef::MoveToZone {
                                object: EffectRecipientDef::Source,
                                zone: ZoneKind::Battlefield,
                                controller: None,
                                placement: ZonePlacement::Top,
                            })
                        || (from == ZoneKind::Battlefield
                            && to == ZoneKind::Graveyard
                            && cause == ZoneMoveCauseDef::Any
                            && matches!(
                                effect,
                                EffectDef::Replacement(effect)
                                    if shared_battlefield_exit_replacement_effect(effect)
                            )))
            }
            ReplacementEventDef::AnyObjectWouldMove { .. } => {
                !definition.optional
                    && definition.condition.is_none()
                    && battlefield_only(definition.source_zones)
                    && shared_replacement_event(definition.event)
                    && effect
                        == EffectDef::MoveToZone {
                            object: EffectRecipientDef::Source,
                            zone: ZoneKind::Exile,
                            controller: None,
                            placement: ZonePlacement::Top,
                        }
            }
            ReplacementEventDef::WouldGainLife(_) => {
                !definition.optional
                    && definition.condition.is_none()
                    && battlefield_only(definition.source_zones)
                    && matches!(effect, EffectDef::MultiplyEventAmount(_))
            }
            ReplacementEventDef::WouldBeginTurn { .. } => {
                definition
                    .condition
                    .is_none_or(|condition| condition == ReplacementConditionDef::SourceTapped)
                    && battlefield_only(definition.source_zones)
                    && matches!(
                        effect,
                        EffectDef::Replacement(effect)
                            if shared_begin_turn_replacement_effect(effect)
                    )
            }
            ReplacementEventDef::Special(_) => false,
        },
        DeclarativeAbilityDef::AlternativeCast(definition) => match definition.kind {
            // Both are permission to cast rather than effects of their
            // own; the card's spell clause does the work.
            AlternativeCastKindDef::Flashback | AlternativeCastKindDef::Miracle => {
                effect == EffectDef::None
            }
            AlternativeCastKindDef::Overload => shared_stack_effect(effect),
        },
        DeclarativeAbilityDef::Keyword(keyword) => shared_keyword(keyword),
        DeclarativeAbilityDef::SpecialAction(_) | DeclarativeAbilityDef::Legacy => false,
    }
}
