//! Which static abilities the shared runtime can execute.
//!
//! A static effect is read live off the battlefield rather than resolved, so
//! the questions here are different from the stack's: what may be applied
//! continuously, to whom, and for how long.

use super::*;
use crate::{
    ControlDurationDef,
    card::{BlockRestrictionDef, PlayerRuleDef},
};

fn cast_source_zones_supported(zones: &[ZoneKind]) -> bool {
    !zones.is_empty()
        && zones.iter().all(|zone| {
            matches!(
                zone,
                ZoneKind::Library | ZoneKind::Hand | ZoneKind::Graveyard | ZoneKind::Exile
            )
        })
}

fn shared_spell_alternative(
    source_zones: &[ZoneKind],
    spell: ObjectPredicateDef,
    caster: PlayerRelation,
    zones: &[ZoneKind],
) -> bool {
    battlefield_only(source_zones)
        && cast_source_zones_supported(zones)
        && shared_object_predicate(spell)
        && matches!(
            caster,
            PlayerRelation::Any | PlayerRelation::You | PlayerRelation::Opponent
        )
}

fn shared_cost_modification(source_zones: &[ZoneKind], modification: CostModificationDef) -> bool {
    match modification {
        CostModificationDef::AbilityIncrease { permanent, .. } => {
            battlefield_only(source_zones) && shared_object_predicate(permanent)
        }
        CostModificationDef::SourceAbilityIncrease { source, .. } => {
            battlefield_only(source_zones) && shared_object_predicate(source)
        }
        CostModificationDef::AbilityReduction {
            permanent, amount, ..
        } => {
            battlefield_only(source_zones)
                && shared_object_predicate(permanent)
                && matches!(
                    amount,
                    crate::card::ValueDef::Constant(_)
                        | crate::card::ValueDef::CountMatchingObjects(_)
                )
        }
        CostModificationDef::Spell(modification) => {
            let allow_nonactive = matches!(modification.adjustment, CostAdjustmentDef::Add(_));
            let source_supported = match source_zones {
                [ZoneKind::Battlefield] => true,
                [ZoneKind::Stack] => {
                    modification.condition == SpellCostConditionDef::TargetsSource
                        && allow_nonactive
                }
                _ => false,
            };
            let amount_supported = match modification.adjustment {
                CostAdjustmentDef::Add(CostAmountDef::Mana(_)) => true,
                CostAdjustmentDef::Add(CostAmountDef::Generic(value))
                | CostAdjustmentDef::Subtract(CostAmountDef::Generic(value)) => {
                    shared_spell_cost_value(value)
                        || (source_zones == [ZoneKind::Battlefield]
                            && matches!(value, ValueDef::CountersOnSource(_)))
                }
                CostAdjustmentDef::Subtract(CostAmountDef::Mana(amount)) => {
                    amount.hybrid.iter().all(|count| *count == 0)
                        && amount.additional_flexible.iter().all(|count| *count == 0)
                        && !amount.variable_x
                        && amount.x_multiplier == 0
                }
            };
            source_supported
                && shared_object_predicate(modification.spell)
                && shared_cost_modifier_caster(modification.caster, allow_nonactive)
                && amount_supported
        }
        CostModificationDef::SpellAlternative {
            spell,
            caster,
            zones,
            ..
        } => shared_spell_alternative(source_zones, spell, caster, zones),
    }
}

fn shared_spell_cost_value(value: ValueDef) -> bool {
    match value {
        ValueDef::Constant(_) | ValueDef::DistinctTargets => true,
        ValueDef::CountMatchingObjects(query) => shared_static_query(*query),
        ValueDef::CountSpellsCastThisTurn(query) => {
            shared_object_predicate(query.spell) && shared_cost_modifier_caster(query.player, true)
        }
        ValueDef::BasicLandTypesControlled(relation) => shared_cost_modifier_caster(relation, true),
        _ => false,
    }
}

fn shared_source_cost_reduction_value(value: ValueDef) -> bool {
    match value {
        ValueDef::Constant(_) => true,
        ValueDef::CountMatchingObjects(query) => {
            shared_static_query(*query) && shared_object_predicate(query.object)
        }
        ValueDef::IfMatchingObjectCount(condition) => {
            shared_static_query(condition.query)
                && shared_object_predicate(condition.query.object)
                && shared_source_cost_reduction_value(condition.then)
                && shared_source_cost_reduction_value(condition.otherwise)
        }
        ValueDef::IfCreatureDiedThisTurn(branches) => {
            shared_source_cost_reduction_value(branches.then)
                && shared_source_cost_reduction_value(branches.otherwise)
        }
        ValueDef::BasicLandTypesControlled(relation) => shared_cost_modifier_caster(relation, true),
        ValueDef::Sum(sum) => {
            shared_source_cost_reduction_value(sum.left)
                && shared_source_cost_reduction_value(sum.right)
        }
        _ => false,
    }
}

fn shared_cost_modifier_caster(caster: PlayerRelation, allow_nonactive: bool) -> bool {
    matches!(
        caster,
        PlayerRelation::Any | PlayerRelation::You | PlayerRelation::Opponent
    ) || (allow_nonactive && caster == PlayerRelation::NonactivePlayer)
}

/// The remaining static effects that are not an `Apply`.
pub(in super::super) fn shared_static_non_apply_effect(
    source_zones: &[ZoneKind],
    effect: EffectDef,
) -> bool {
    match effect {
        // Both are read off the battlefield and neither carries anything
        // further to check: one names a land type, the other nothing at all.
        EffectDef::CannotBeForcedToSacrifice
        | EffectDef::CannotBeForcedToDiscard
        | EffectDef::GainClassLevel { .. }
        | EffectDef::SubstituteBasicLandTypeUntilEndOfTurn { .. }
        | EffectDef::LandwalkCanBeBlocked(_)
        | EffectDef::GainControl {
            object: EffectRecipientDef::AttachedPermanent,
            controller: PlayerRefDef::EffectController,
            duration:
                ControlDurationDef::WhileSourceRemains {
                    while_tapped: false,
                },
        } => battlefield_only(source_zones),
        // Read while attackers are declared, over the battlefield, so only
        // the object predicate is left to check.
        EffectDef::CannotAttackUnless(query) | EffectDef::CannotAttackIf(query) => {
            battlefield_only(source_zones)
                && query.zones == [ZoneKind::Battlefield]
                && shared_object_predicate(query.object)
                && shared_static_query(*query)
        }
        EffectDef::ModifyCost(modification) => shared_cost_modification(source_zones, modification),
        EffectDef::ReduceGenericCostBy(value) => {
            source_zones == [ZoneKind::Hand] && shared_source_cost_reduction_value(value)
        }
        EffectDef::Sequence(effects) => {
            !effects.is_empty()
                && effects
                    .iter()
                    .copied()
                    .all(|effect| shared_static_effect_at(source_zones, effect, false))
        }
        _ => false,
    }
}

#[allow(clippy::too_many_lines)]
pub(in super::super) fn shared_static_effect(source_zones: &[ZoneKind], effect: EffectDef) -> bool {
    shared_static_effect_at(source_zones, effect, true)
}

#[allow(clippy::too_many_lines)]
fn shared_static_effect_at(source_zones: &[ZoneKind], effect: EffectDef, root: bool) -> bool {
    match effect {
        EffectDef::CannotBeForcedToSacrifice
        | EffectDef::CannotBeForcedToDiscard
        | EffectDef::GainClassLevel { .. }
        | EffectDef::SubstituteBasicLandTypeUntilEndOfTurn { .. }
        | EffectDef::ReduceGenericCostBy(_)
        | EffectDef::ModifyCost(_)
        | EffectDef::LandwalkCanBeBlocked(_)
        | EffectDef::CannotAttackUnless(_)
        | EffectDef::CannotAttackIf(_)
        | EffectDef::GainControl { .. }
        | EffectDef::ChooseExact(_)
        | EffectDef::Sequence(_) => shared_static_non_apply_effect(source_zones, effect),
        EffectDef::ConditionalStatic(conditional) => {
            battlefield_only(source_zones)
                && shared_source_object_set(*conditional.condition.objects)
                && conditional
                    .condition
                    .predicate
                    .filter
                    .is_none_or(|filter| shared_object_predicate(filter.predicate()))
                && shared_static_effect_at(
                    source_zones,
                    EffectDef::StaticApply {
                        recipient: conditional.then.recipient,
                        effect: conditional.then.effect,
                    },
                    false,
                )
        }
        EffectDef::StaticApply { recipient, effect } => {
            let battlefield_recipient_is_supported = match recipient.0 {
                EffectRecipientSetDef::Objects(ObjectSetDef::One(
                    ObjectRefDef::Source | ObjectRefDef::AttachedToSource,
                ))
                | EffectRecipientSetDef::Players(
                    PlayerSetDef::All
                    | PlayerSetDef::One(
                        PlayerRefDef::EffectController
                        | PlayerRefDef::Opponent
                        | PlayerRefDef::EnchantedPlayer,
                    )
                    | PlayerSetDef::Related(_),
                ) => true,
                EffectRecipientSetDef::Objects(ObjectSetDef::Query(query)) => {
                    (query.zones == [ZoneKind::Battlefield]
                        && shared_object_predicate(query.object)
                        && shared_static_query(query))
                        || source_zones == [ZoneKind::Battlefield]
                            && shared_static_creature_type_effect(effect)
                            && shared_static_creature_type_query(recipient)
                }
                // A static clause names one kind of thing or the other, and
                // names it outright rather than by what it is attacking.
                EffectRecipientSetDef::PlayersAndCreaturesTheyControl(_)
                | EffectRecipientSetDef::DefenderOf(_)
                | EffectRecipientSetDef::LegalTargets(_)
                | EffectRecipientSetDef::Objects(
                    ObjectSetDef::One(
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
                    | ObjectSetDef::Matching { .. }
                    | ObjectSetDef::LinkedExiles
                    | ObjectSetDef::CardsDrawnThisTurnInHand(_)
                    | ObjectSetDef::PermanentsControlledBy(_)
                    | ObjectSetDef::BottomOfGraveyard(_)
                    | ObjectSetDef::LegalTargets(_)
                    | ObjectSetDef::PermanentsTargetedBy(_)
                    | ObjectSetDef::PlayerAttachments(_)
                    | ObjectSetDef::LegalAttachmentHosts(_)
                    | ObjectSetDef::ExceptObject { .. }
                    | ObjectSetDef::TokensCreatedBy(_)
                    | ObjectSetDef::TopOfGraveyardMatching { .. },
                )
                | EffectRecipientSetDef::Players(
                    PlayerSetDef::LegalTargets(_)
                    | PlayerSetDef::One(
                        PlayerRefDef::EventPlayer
                        | PlayerRefDef::Target(_)
                        | PlayerRefDef::ControllerOf(_)
                        | PlayerRefDef::OpponentOf(_)
                        | PlayerRefDef::OwnerOf(_),
                    ),
                ) => false,
            };
            let battlefield_effect_is_supported = shared_static_applied_effect(recipient, effect);
            let battlefield_effect =
                matches!(source_zones, [ZoneKind::Battlefield | ZoneKind::Graveyard])
                    && battlefield_recipient_is_supported
                    && battlefield_effect_is_supported;
            let stack_source_effect = source_zones == [ZoneKind::Stack]
                && recipient == EffectRecipientDef::Source
                && shared_cannot_be_countered_effect(effect);
            let battlefield_stack_effect = root
                && battlefield_only(source_zones)
                && recipient.object_query().is_some_and(|query| {
                    query.zones == [ZoneKind::Stack]
                        && shared_object_predicate(query.object)
                        && shared_static_query(query)
                })
                && shared_stack_uncounterability_effect(effect);
            // "As long as this isn't on the battlefield, it's a 1/1 Insect
            // creature": what a card says about itself, read by the card
            // view in whichever of its zones the clause names. The stack is
            // one of them -- the spell on its way in wears the clause too --
            // and only the battlefield, with its own layer walk, is not.
            let card_source_effect = !source_zones.is_empty()
                && source_zones.iter().all(|zone| {
                    matches!(
                        zone,
                        ZoneKind::Library
                            | ZoneKind::Hand
                            | ZoneKind::Graveyard
                            | ZoneKind::Stack
                            | ZoneKind::Exile
                            | ZoneKind::Command
                    )
                })
                && recipient == EffectRecipientDef::Source
                && shared_card_characteristics(effect);
            battlefield_effect
                || stack_source_effect
                || battlefield_stack_effect
                || card_source_effect
        }
        effect @ (EffectDef::IfCondition { .. } | EffectDef::IfElseCondition { .. }) => {
            let conditional = effect
                .conditional()
                .expect("conditional variants expose their shared shape");
            battlefield_only(source_zones)
                && shared_static_trigger_condition(*conditional.condition)
                && shared_static_effect_at(source_zones, *conditional.then, false)
                && conditional.otherwise.is_none_or(|otherwise| {
                    shared_static_effect_at(source_zones, *otherwise, false)
                })
        }
        // None of these is a static ability; all execute from the stack.
        EffectDef::BindOutput { .. }
        | EffectDef::DamageCannotBePreventedThisTurn
        | EffectDef::PutSourceOntoBattlefieldAttacking
        | EffectDef::BecomeMonarch { .. }
        | EffectDef::VoteForPermanentToExile { .. }
        | EffectDef::Randomized { .. }
        | EffectDef::Choose(_)
        | EffectDef::ChooseForEachPlayer(_)
        | EffectDef::ChooseCardName { .. }
        | EffectDef::SelectAtRandomFromZone { .. }
        | EffectDef::ForEachInBinding { .. }
        | EffectDef::PayOr(_)
        | EffectDef::PreventDamage { .. }
        | EffectDef::Apply { .. }
        | EffectDef::May { .. }
        | EffectDef::ExileLinkedToSource { .. }
        | EffectDef::MayPlayWithoutPaying { .. }
        | EffectDef::ExileGrantingOwnerPlay { .. }
        | EffectDef::ExileGrantingControllerPlayThisTurn { .. }
        | EffectDef::PermitCastFromGraveyardThisTurn { .. }
        | EffectDef::ReturnLinkedExiles { .. }
        | EffectDef::Detain { .. }
        | EffectDef::ExchangeControl { .. }
        | EffectDef::InstallTrigger(_)
        | EffectDef::ContinueReplacedDraw
        | EffectDef::None
        | EffectDef::AddMana(_)
        | EffectDef::AddManaEqualTo { .. }
        | EffectDef::DealDamage { .. }
        | EffectDef::DealDamageFrom { .. }
        | EffectDef::DealDamageAndApply { .. }
        | EffectDef::DrainLife { .. }
        | EffectDef::GainLife { .. }
        | EffectDef::SetLifeTotal { .. }
        | EffectDef::AddPlayerCounters { .. }
        | EffectDef::DrawCards { .. }
        | EffectDef::Discard { .. }
        | EffectDef::DiscardCards { .. }
        | EffectDef::ShuffleLibrary { .. }
        | EffectDef::BuryGraveyard { .. }
        | EffectDef::EmptyManaPool { .. }
        | EffectDef::LoseLife { .. }
        | EffectDef::LoseTheGame { .. }
        | EffectDef::WinTheGame { .. }
        | EffectDef::Regenerate { .. }
        | EffectDef::Tap { .. }
        | EffectDef::RemoveFromCombat { .. }
        | EffectDef::SkipNextUntapSteps { .. }
        | EffectDef::DoubleCounters { .. }
        | EffectDef::RemoveAllCounters { .. }
        | EffectDef::Untap { .. }
        | EffectDef::Saddle { .. }
        | EffectDef::Attach { .. }
        | EffectDef::AttachToSource { .. }
        | EffectDef::Reconfigure { .. }
        | EffectDef::Unattach { .. }
        | EffectDef::PairWithSource { .. }
        | EffectDef::PhaseOut { .. }
        | EffectDef::CreateToken { .. }
        | EffectDef::CreateAttachedToken { .. }
        | EffectDef::Endure { .. }
        | EffectDef::Destroy { .. }
        | EffectDef::Sacrifice { .. }
        | EffectDef::SacrificeYours { .. }
        | EffectDef::SacrificeOfChoice { .. }
        | EffectDef::ExileTopOfLibraryToPlay { .. }
        | EffectDef::ExileTopAndMayCast { .. }
        | EffectDef::MayCastTargetWithoutPaying { .. }
        | EffectDef::Mill { .. }
        | EffectDef::SearchZonesAndExileRest { .. }
        | EffectDef::MillUntil { .. }
        | EffectDef::ExileFromTopUntil { .. }
        | EffectDef::Cascade
        | EffectDef::Proliferate
        | EffectDef::Explore { .. }
        | EffectDef::LookAtHand { .. }
        | EffectDef::ChooseCardsFromCollection(_)
        | EffectDef::LookAtObjects(_)
        | EffectDef::ChooseObjectOrder(_)
        | EffectDef::ClassifyObjects(_)
        | EffectDef::RevealAndClassifyCards(_)
        | EffectDef::CombineObjects(_)
        | EffectDef::ChooseOneOfEach(_)
        | EffectDef::ChooseGroup(_)
        | EffectDef::BindObjects(_)
        | EffectDef::IfNoObjects(_)
        | EffectDef::PartitionGroup(_)
        | EffectDef::RandomizeObjectOrder(_)
        | EffectDef::RevealObjects(_)
        | EffectDef::MoveObjects(_)
        | EffectDef::PutObjectsOntoBattlefieldFaceDown(_)
        | EffectDef::PermitLookAtExiled { .. }
        | EffectDef::ExileOneFromEachZone(_)
        | EffectDef::MillWhileMatching(_)
        | EffectDef::LookAtRandomCardInHand { .. }
        | EffectDef::RevealAtRandomFromHand { .. }
        | EffectDef::RevealHand { .. }
        | EffectDef::SearchZone { .. }
        | EffectDef::ChooseCards { .. }
        | EffectDef::ReplaceNextDrawThisTurn { .. }
        | EffectDef::IfFormat { .. }
        | EffectDef::Counter { .. }
        | EffectDef::PutSpellIntoOwnersLibrary { .. }
        | EffectDef::CopyStackObject(_)
        | EffectDef::ChangeStackTargets(_)
        | EffectDef::AddCounters { .. }
        | EffectDef::ChooseCounterKind { .. }
        | EffectDef::ChooseEffect { .. }
        | EffectDef::ModifyCounters { .. }
        | EffectDef::RemoveCounters { .. }
        | EffectDef::ChangeTextBasicLandType { .. }
        | EffectDef::ChooseColor { .. }
        | EffectDef::BecomeCopyOf { .. }
        | EffectDef::PutIntoLibraryBeneathTop { .. }
        | EffectDef::MoveToZone { .. }
        | EffectDef::WithBattlefieldArrival { .. }
        | EffectDef::WithZoneMoveResult { .. }
        | EffectDef::CreateEmblem { .. }
        | EffectDef::CreateOngoingEffect(_)
        | EffectDef::PutOntoBattlefieldThen { .. }
        | EffectDef::Transform { .. }
        | EffectDef::ScheduleTurnPhases(_)
        | EffectDef::TakeExtraTurn { .. }
        | EffectDef::CreateMyriadTokens
        | EffectDef::DealDamageSimultaneously(_)
        | EffectDef::Fight { .. }
        | EffectDef::Special(_) => false,
    }
}

fn shared_stack_uncounterability_effect(effect: AppliedEffectDef) -> bool {
    match effect {
        AppliedEffectDef::Composite(effects) => {
            !effects.is_empty()
                && effects
                    .iter()
                    .copied()
                    .all(shared_stack_uncounterability_effect)
        }
        AppliedEffectDef::Rule(AppliedRuleDef::CannotBeCountered) => true,
        AppliedEffectDef::Characteristic(_) | AppliedEffectDef::Rule(_) => false,
    }
}

fn shared_static_creature_type_effect(effect: AppliedEffectDef) -> bool {
    match effect {
        AppliedEffectDef::Composite(effects) => {
            !effects.is_empty()
                && effects
                    .iter()
                    .copied()
                    .all(shared_static_creature_type_effect)
        }
        AppliedEffectDef::Characteristic(
            CharacteristicOperationDef::CreatureTypes(_)
            | CharacteristicOperationDef::AddChosenCreatureType
            | CharacteristicOperationDef::SetChosenCreatureType,
        ) => true,
        AppliedEffectDef::Characteristic(_) | AppliedEffectDef::Rule(_) => false,
    }
}

fn shared_static_animation_query(recipient: EffectRecipientDef) -> bool {
    shared_direct_characteristic_recipient(recipient)
        || recipient.object_query().is_some_and(|query| {
            query.zones == [ZoneKind::Battlefield]
                && shared_static_query(query)
                && Game::static_animation_predicate_is_supported(query.object)
        })
}

fn shared_static_type_animation_query(recipient: EffectRecipientDef) -> bool {
    shared_direct_characteristic_recipient(recipient)
        || recipient.object_query().is_some_and(|query| {
            query.zones == [ZoneKind::Battlefield]
                && shared_static_query(query)
                && Game::static_type_animation_predicate_is_supported(query.object)
        })
}

fn shared_static_creature_type_query(recipient: EffectRecipientDef) -> bool {
    shared_direct_characteristic_recipient(recipient)
        || recipient.object_query().is_some_and(|query| {
            shared_static_query(query)
                && Game::static_type_animation_predicate_is_supported(query.object)
        })
}

fn shared_direct_characteristic_recipient(recipient: EffectRecipientDef) -> bool {
    matches!(
        recipient.object_reference(),
        Some(ObjectRefDef::Source | ObjectRefDef::AttachedToSource)
    )
}

/// What a card may say about itself while it sits in a zone: types,
/// subtypes, and a printed body, which is all a card view carries.
fn shared_card_characteristics(effect: AppliedEffectDef) -> bool {
    match effect {
        AppliedEffectDef::Composite(effects) => {
            !effects.is_empty() && effects.iter().copied().all(shared_card_characteristics)
        }
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::CardTypes(
            SetOperationDef::Add(types),
        )) => !types.is_empty(),
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::Subtypes(
            SetOperationDef::Add(subtypes),
        )) => !subtypes.is_empty(),
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
            PowerToughnessOperationDef::SetBase {
                power: crate::card::ValueDef::Constant(_),
                toughness: crate::card::ValueDef::Constant(_),
            },
        )) => true,
        _ => false,
    }
}

pub(in super::super) fn shared_static_applied_effect(
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
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
            PowerToughnessOperationDef::SetBase { power, toughness }
            | PowerToughnessOperationDef::Modify { power, toughness },
        )) => static_stat_value(power) && static_stat_value(toughness),
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
            PowerToughnessOperationDef::SetBasePower(power)
            | PowerToughnessOperationDef::SetBaseToughness(power),
        )) => static_stat_value(power),
        // A characteristic-defining ability is answered by the shared
        // characteristics walk in every zone, so what it may read is what
        // any other shared static stat may read.
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
            PowerToughnessOperationDef::Define { power, toughness },
        )) => {
            (power.is_some() || toughness.is_some())
                && power.is_none_or(static_stat_value)
                && toughness.is_none_or(static_stat_value)
        }
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::BasicLandTypes(
            SetOperationDef::Add(land_types)
            | SetOperationDef::Remove(land_types)
            | SetOperationDef::Set(land_types),
        )) => !land_types.is_empty(),
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
            AbilityOperationDef::Add(ability),
        )) => shared_definition_ability(ability),
        // Static animation is deliberately narrower than resolving
        // characteristic changes. Direct source/attachment recipients cannot
        // feed back into their own selection; a group query must avoid
        // reading the characteristics it supplies.
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::CardTypes(
            SetOperationDef::Add(types),
        )) => {
            types != crate::card::CardTypeSet::EMPTY
                && (shared_direct_characteristic_recipient(recipient)
                    || types == crate::card::CardTypeSet::single(CardType::Creature)
                        && shared_static_type_animation_query(recipient))
        }
        AppliedEffectDef::Characteristic(
            CharacteristicOperationDef::Color(_)
            | CharacteristicOperationDef::Colors(_)
            | CharacteristicOperationDef::Subtypes(_),
        ) => shared_static_animation_query(recipient),
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::Supertypes(operation)) => {
            let supertypes = match operation {
                SetOperationDef::Add(supertypes)
                | SetOperationDef::Remove(supertypes)
                | SetOperationDef::Set(supertypes) => supertypes,
            };
            !supertypes.is_empty() && shared_static_animation_query(recipient)
        }
        AppliedEffectDef::Characteristic(
            CharacteristicOperationDef::CreatureTypes(_)
            | CharacteristicOperationDef::AddChosenCreatureType
            | CharacteristicOperationDef::SetChosenCreatureType,
        ) => shared_static_creature_type_query(recipient),
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::CardTypes(
            SetOperationDef::Remove(_) | SetOperationDef::Set(_),
        )) => false,
        // A blocking restriction is read off the ordinary static-effect walk
        // over whichever participant carries it, so a group recipient works
        // exactly as a self-applied one does.
        AppliedEffectDef::Rule(rule) => shared_static_applied_rule(recipient, rule),
        // A switch reads nothing, so there is no value to gate on.
        // A linked-exile grant hands out whatever the exiled creature cards
        // print, so there is nothing here to check ahead of time the way a
        // written-down grant is checked. Each ability it finds already passed
        // the catalog's own validation as an ability of its own card. "This
        // land is the chosen type" carries nothing to check either: which
        // type it is comes from the choice made on the way in.
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
            AbilityOperationDef::AddActivatedAbilitiesOfLinkedExiles(object),
        )) => shared_object_predicate(object),
        AppliedEffectDef::Characteristic(
            CharacteristicOperationDef::Abilities(AbilityOperationDef::Remove(_))
            | CharacteristicOperationDef::PowerToughness(PowerToughnessOperationDef::Switch)
            | CharacteristicOperationDef::ChosenBasicLandType,
        ) => true,
    }
}

/// The rules a static ability may hand to what it affects. Split from the
/// characteristic changes above because the two ask different questions: a
/// characteristic change has to be something the layer walk can evaluate,
/// while a rule has to be something the place that reads it knows to look
/// for.
fn shared_static_applied_rule(recipient: EffectRecipientDef, rule: AppliedRuleDef) -> bool {
    match rule {
        AppliedRuleDef::RedirectDamageFromTo { .. } => false,
        AppliedRuleDef::PlayerRule(PlayerRuleDef::LegendRuleDoesNotApplyTo(predicate)) => {
            matches!(recipient.0, EffectRecipientSetDef::Players(_))
                && shared_object_predicate(*predicate)
        }
        AppliedRuleDef::BlockRestriction(restriction) => {
            (matches!(
                recipient.object_reference(),
                Some(ObjectRefDef::Source | ObjectRefDef::AttachedToSource)
            ) || recipient.object_query().is_some())
                && match restriction {
                    BlockRestrictionDef::Pair {
                        counterpart, cost, ..
                    } => {
                        let counterpart_supported = match counterpart {
                            BlockRestrictionMatchDef::Any => true,
                            BlockRestrictionMatchDef::Matching(predicate)
                            | BlockRestrictionMatchDef::Except(predicate) => {
                                shared_object_predicate(predicate)
                            }
                        };
                        counterpart_supported
                            && cost.is_none_or(|cost| !cost.variable_x && cost.x_multiplier == 0)
                    }
                    BlockRestrictionDef::MinimumBlockers(required) => required > 1,
                }
        }
        // A requirement is read off the attacker on the same walk as the
        // prohibition above, but no card applies one to a group, so the
        // recipient list stays as narrow as the restriction's.
        AppliedRuleDef::MustBeBlockedBy(predicate) => {
            matches!(
                recipient.object_reference(),
                Some(ObjectRefDef::Source | ObjectRefDef::AttachedToSource)
            ) && shared_object_predicate(predicate)
        }
        // The redirection names a group rather than a predicate, and it can
        // only be aimed at a permanent whose controller it protects.
        AppliedRuleDef::RedirectPlayerDamageToThis(_) => {
            matches!(
                recipient.object_reference(),
                Some(ObjectRefDef::Source | ObjectRefDef::AttachedToSource)
            ) || recipient.object_query().is_some()
        }
        AppliedRuleDef::PreventDamage(matcher) => {
            // The shield is looked up on the permanent it was applied to, so a
            // query recipient installs one on each match and nothing has to
            // rewrite the matcher.
            let recipient_is_supported = matches!(
                recipient.object_reference(),
                Some(ObjectRefDef::Source | ObjectRefDef::AttachedToSource)
            ) || recipient.object_query().is_some();
            let matcher_is_supported = match (matcher.source, matcher.recipient) {
                (
                    DamageSourceMatcherDef::Matching(source),
                    DamageRecipientMatcherDef::AffectedObject,
                ) => shared_object_predicate(source),
                (DamageSourceMatcherDef::AffectedObject, DamageRecipientMatcherDef::Any)
                | (DamageSourceMatcherDef::Any, DamageRecipientMatcherDef::AffectedObject) => true,
                _ => false,
            };
            recipient_is_supported && matcher_is_supported
        }
        // Read only off the Aura whose attachment it is defending, which is
        // the source of the ability granting the protection.
        AppliedRuleDef::RemainsAttachedThroughProtection => recipient == EffectRecipientDef::Source,
        // Read where a graveyard cast is enumerated, by the same walk the
        // permissions below go through.
        AppliedRuleDef::GrantsAlternativeCastFromGraveyard { object, ability } => {
            matches!(recipient.0, EffectRecipientSetDef::Players(_))
                && shared_object_predicate(object)
                && matches!(
                    ability.definition,
                    DeclarativeAbilityDef::AlternativeCast(_)
                )
        }
        // The graveyard permission carries what bounds it as well as what it
        // names, and both halves are read where the play is offered.
        AppliedRuleDef::MayPlayFromGraveyard(permission) => {
            matches!(recipient.0, EffectRecipientSetDef::Players(_))
                && shared_object_predicate(permission.restriction.object)
        }
        AppliedRuleDef::CannotPlay(restriction)
        | AppliedRuleDef::MayPlayFromTopOfLibrary { restriction, .. } => {
            matches!(recipient.0, EffectRecipientSetDef::Players(_))
                && shared_object_predicate(restriction.object)
        }
        // Read where a cast is offered, against the card being cast, so what
        // it names has to be answerable there the same way.
        AppliedRuleDef::MayCastAsThoughItHadFlash(permission) => {
            matches!(recipient.0, EffectRecipientSetDef::Players(_))
                && shared_object_predicate(permission.object)
        }
        _ => true,
    }
}

/// The values a static power/toughness bonus may be built from. They are the
/// ones the layer walk can evaluate without reading a resolving spell, and a
/// scale or a halving is allowed only over another such value.
fn static_stat_value(value: crate::card::ValueDef) -> bool {
    match value {
        crate::card::ValueDef::Constant(_)
        | crate::card::ValueDef::AnyMatchingObject(_)
        // Read live from the static effect's own controller, the same way a
        // battlefield count is.
        | crate::card::ValueDef::CardsInHandAbove { .. }
        | crate::card::ValueDef::CountMatchingObjects(_)
        // Read from the affected object rather than from the effect's own
        // source, which the static power-and-toughness layer has in hand.
        | crate::card::ValueDef::AffectedManaValue
        | crate::card::ValueDef::AffectedColorCount
        // Read live from every graveyard, which the layer walk can reach
        // without a resolving spell in hand.
        | crate::card::ValueDef::CardTypesAmongGraveyards(_)
        // A tally the game keeps for the turn, reachable from the layer walk
        // for the same reason.
        | crate::card::ValueDef::CardsDrawnThisTurn(_)
        // Counters on the effect's own source: plain state the layer has.
        | crate::card::ValueDef::CountersOnSource(_)
        // Domain, read live off the lands on the battlefield the same way a
        // battlefield count is.
        | crate::card::ValueDef::BasicLandTypesControlled(_) => true,
        crate::card::ValueDef::CountObjects(objects)
        | crate::card::ValueDef::CardTypesAmongObjects(objects) => {
            shared_source_object_set(*objects)
        }
        crate::card::ValueDef::AggregateObjectValues(aggregate) => {
            shared_source_object_set(aggregate.objects)
        }
        crate::card::ValueDef::Scaled(scaled) => static_stat_value(scaled.value),
        crate::card::ValueDef::Halved(halved) => static_stat_value(halved.value),
        crate::card::ValueDef::Quotient(quotient) => {
            static_stat_value(quotient.numerator) && static_stat_value(quotient.denominator)
        }
        crate::card::ValueDef::IfSourceMatches(branches) => {
            shared_object_predicate(branches.object)
                && static_stat_value(branches.then)
                && static_stat_value(branches.otherwise)
        }
        // "That number plus 1" is one amount, not a printed body with a
        // count over it, so a sum of two readable amounts is readable.
        crate::card::ValueDef::Sum(sum) => {
            static_stat_value(sum.left) && static_stat_value(sum.right)
        }
        _ => false,
    }
}
