mod conditions;
mod costs;
mod nested_definitions;
mod stack_effects;
mod static_effects;

pub(super) use conditions::*;
pub(super) use costs::*;
pub(super) use static_effects::shared_static_effect;

pub(super) use nested_definitions::*;
pub(super) use stack_effects::shared_stack_effect;

use crate::Game;
use crate::card::{
    ActivatedAbilityDef, AppliedRuleDef, BlockRestrictionMatchDef, CostAdjustmentDef,
    CostAmountDef, CostModificationDef, ManaTypeDef, ReplacementConditionDef,
    SpellCostConditionDef, StackObjectEventDef, StackTargetFilterDef,
};

use super::*;

pub(super) fn shared_object_predicate(predicate: ObjectPredicateDef) -> bool {
    match predicate {
        ObjectPredicateDef::All(predicates) | ObjectPredicateDef::AnyOf(predicates) => {
            predicates.iter().copied().all(shared_object_predicate)
        }
        ObjectPredicateDef::Not(predicate) | ObjectPredicateDef::AttachedTo(predicate) => {
            shared_object_predicate(*predicate)
        }
        ObjectPredicateDef::HasName(
            ObjectRefDef::AbilityGrantSource
            | ObjectRefDef::CreatingSource
            | ObjectRefDef::ZoneChangeSuccessor(_)
            | ObjectRefDef::ZoneChangeResultOfTriggeringObject
            | ObjectRefDef::ResolvingObject
            | ObjectRefDef::AttachedToSource
            | ObjectRefDef::Target(_)
            | ObjectRefDef::TriggeringObject
            | ObjectRefDef::DamagedObject
            | ObjectRefDef::Binding(_)
            | ObjectRefDef::AdditionalCostObject(_)
            | ObjectRefDef::SourceOfTargetedStackObject(_),
        )
        | ObjectPredicateDef::Special(_) => false,
        ObjectPredicateDef::Any
        | ObjectPredicateDef::Source
        | ObjectPredicateDef::Token
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
        | ObjectPredicateDef::PowerLessThan(_)
        | ObjectPredicateDef::ToughnessGreaterThanItsPower
        | ObjectPredicateDef::ToughnessGreaterThan(_)
        | ObjectPredicateDef::WasDealtDamageThisTurn
        | ObjectPredicateDef::DealtDamageThisTurn
        | ObjectPredicateDef::ControlledBy(_)
        | ObjectPredicateDef::OwnedBy(_)
        | ObjectPredicateDef::Supertype(_)
        | ObjectPredicateDef::DebutSet(_)
        | ObjectPredicateDef::HasName(ObjectRefDef::Source)
        | ObjectPredicateDef::HasSourcesChosenScalar(_)
        | ObjectPredicateDef::TargetsObjectMatching(_)
        | ObjectPredicateDef::AttackingOrBlocking
        | ObjectPredicateDef::HasKeyword(_)
        | ObjectPredicateDef::HasAbility(_)
        | ObjectPredicateDef::HasCounter(_)
        | ObjectPredicateDef::HasAnyCounter
        | ObjectPredicateDef::CounterCount { .. }
        | ObjectPredicateDef::HasNonManaActivatedAbility
        | ObjectPredicateDef::Tapped
        | ObjectPredicateDef::Attacking
        | ObjectPredicateDef::Saddled
        | ObjectPredicateDef::Blocking
        | ObjectPredicateDef::AttachedToSource
        | ObjectPredicateDef::BlockedBySource
        | ObjectPredicateDef::BlockingSource
        | ObjectPredicateDef::BandedWithSource
        | ObjectPredicateDef::Unpaired
        | ObjectPredicateDef::PairedWithSource
        | ObjectPredicateDef::Enchanted
        | ObjectPredicateDef::AttackedThisTurn
        | ObjectPredicateDef::CameUnderControlThisTurn
        | ObjectPredicateDef::EnteredThisTurn
        | ObjectPredicateDef::AttackedDuringControllersLastTurn => true,
    }
}

pub(super) fn shared_effect_recipient(recipient: EffectRecipientDef) -> bool {
    match recipient.0 {
        EffectRecipientSetDef::Objects(ObjectSetDef::Query(query)) => {
            let ObjectQueryDef { object, zones, .. } = query;
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
        EffectRecipientSetDef::LegalTargets(_)
        | EffectRecipientSetDef::Objects(
            ObjectSetDef::One(_)
            | ObjectSetDef::PermanentsTargetedBy(_)
            | ObjectSetDef::PlayerAttachments(_)
            | ObjectSetDef::LegalAttachmentHosts(_)
            | ObjectSetDef::Binding(_)

            | ObjectSetDef::ZoneChangeSuccessorsOfBinding(_)
            | ObjectSetDef::MatchingBinding { .. }
            | ObjectSetDef::Matching { .. }
            | ObjectSetDef::LinkedExiles
            | ObjectSetDef::CardsDrawnThisTurnInHand(_)
            | ObjectSetDef::PermanentsControlledBy(_)
            | ObjectSetDef::BottomOfGraveyard(_)
            | ObjectSetDef::LegalTargets(_)
            | ObjectSetDef::SharingNameWith(_)
            | ObjectSetDef::SharingNameWithBinding { .. }
            | ObjectSetDef::TokensCreatedBy(_)
            | ObjectSetDef::TopOfGraveyardMatching { .. },
        )
        // Both kinds at once is shared for the same reason each half is:
        // the players come from a relation, and the creatures they control
        // from the ordinary battlefield walk. What an attacker is attacking
        // is shared too: the declaration recorded it, and the walk reads it
        // back the same way either kind of recipient is read.
        | EffectRecipientSetDef::Players(_)
        | EffectRecipientSetDef::DefenderOf(_)
        | EffectRecipientSetDef::PlayersAndCreaturesTheyControl(_) => true,
    }
}

fn shared_static_player_set(players: PlayerSetDef) -> bool {
    matches!(
        players,
        PlayerSetDef::All
            | PlayerSetDef::Related(_)
            | PlayerSetDef::One(PlayerRefDef::EffectController)
    )
}

fn shared_static_query(query: ObjectQueryDef) -> bool {
    query.relative_position.is_none()
        && [query.related_player, query.controller, query.owner]
            .into_iter()
            .flatten()
            .all(shared_static_player_set)
}

pub(super) fn shared_keyword(keyword: KeywordAbility) -> bool {
    matches!(
        keyword,
        KeywordAbility::Convoke
            | KeywordAbility::Delve
            | KeywordAbility::Improvise
            | KeywordAbility::Flying
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
            | KeywordAbility::Shadow
            | KeywordAbility::Menace
            | KeywordAbility::Flanking
            | KeywordAbility::Undying
            | KeywordAbility::Persist
            | KeywordAbility::Indestructible
            | KeywordAbility::Landwalk(_)
            | KeywordAbility::LegendaryLandwalk
            | KeywordAbility::AttacksEachCombatIfAble
            | KeywordAbility::AttacksPlayerEachCombatIfAble
            | KeywordAbility::Banding
            | KeywordAbility::BandsWithOther(_)
            | KeywordAbility::Unleash
            | KeywordAbility::ProtectionFrom(_)
            // The colourlessness is the card's printed colour set, so the
            // keyword itself has nothing left to execute.
            | KeywordAbility::Infect
            | KeywordAbility::Devoid
            | KeywordAbility::Compleated
            | KeywordAbility::Suspend(_)
            | KeywordAbility::Rebound
            | KeywordAbility::SplitSecond
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
        AppliedEffectDef::Rule(
            AppliedRuleDef::CannotBeCountered | AppliedRuleDef::CannotBeEnchanted,
        ) => true,
        AppliedEffectDef::Characteristic(_) | AppliedEffectDef::Rule(_) => false,
    }
}

/// Whether an applied effect does nothing but grant keyword abilities. Mana
/// riders that survive into the permanent a spell becomes are limited to
/// these: anything else would be an effect with nowhere to live once the
/// spell is gone.
pub(super) fn shared_granted_keyword_effect(effect: AppliedEffectDef) -> bool {
    match effect {
        AppliedEffectDef::Composite(effects) => {
            !effects.is_empty() && effects.iter().copied().all(shared_granted_keyword_effect)
        }
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
            AbilityOperationDef::Add(ability),
        )) => matches!(ability.definition, DeclarativeAbilityDef::Keyword(_)),
        AppliedEffectDef::Characteristic(_) | AppliedEffectDef::Rule(_) => false,
    }
}

pub(super) fn shared_mana_effect(effect: EffectDef, choices_are_supported: bool) -> bool {
    // "Add one for each counter on this creature" is read off the permanent
    // as the ability is offered, so the amount is known before activation
    // just as a printed one is.
    if let EffectDef::AddManaEqualTo { amount, .. } = effect {
        return matches!(
            amount,
            ValueDef::CountersOnSource(_)
                | ValueDef::PaidAmount
                | ValueDef::MatchedCount
                | ValueDef::BoundObjectCount(_)
                | ValueDef::SpellsCastBeforeThisTurn
                | ValueDef::CountMatchingObjects(_)
        );
    }
    let EffectDef::AddMana(mana) = effect else {
        return false;
    };
    let selection_is_supported = match mana.mana {
        ManaSelectionDef::One(ManaTypeDef::Fixed(_)) => true,
        ManaSelectionDef::One(ManaTypeDef::ChosenColor)
        | ManaSelectionDef::ColorsOfLinkedExiles => choices_are_supported,
        // Both selections offer a choice of type; a combination divides the
        // amount across those types instead of picking one, and the runtime
        // enumerates every division for the same reason it enumerates every
        // colour.
        ManaSelectionDef::Choice(types) | ManaSelectionDef::Combination(types) => {
            choices_are_supported
                && match types.source {
                    crate::card::ManaTypeSourceDef::Fixed(colors) => !colors.is_empty(),
                    crate::card::ManaTypeSourceDef::CouldBeProducedBy(
                        crate::card::ObjectSetDef::One(crate::card::ObjectRefDef::Source)
                        | crate::card::ObjectSetDef::Query(_),
                    ) => true,
                    crate::card::ManaTypeSourceDef::ProducedBy(_)
                    | crate::card::ManaTypeSourceDef::CouldBeProducedBy(_) => false,
                }
        }
    };
    // "Where X is this creature's power" and "where X is the number of
    // permanents you control matching a predicate" are resolved against the
    // permanent as the ability is offered, exactly as the counted forms above
    // are, so a printed amount of zero is the whole amount only when no value
    // replaces it.
    let amount_is_known = mana.amount > 0
        || matches!(
            mana.variable_amount,
            Some(
                ValueDef::CountersOnSource(_)
                    | ValueDef::SourcePower
                    | ValueDef::CountMatchingObjects(_)
            )
        );
    selection_is_supported
        && amount_is_known
        && mana
            .restrictions
            .iter()
            .copied()
            .all(|restriction| match restriction {
                ManaRestrictionDef::CastSpell(object)
                | ManaRestrictionDef::CannotCastSpell(object) => shared_object_predicate(object),
                ManaRestrictionDef::CastCreatureSpellOfChosenType => true,
                ManaRestrictionDef::ActivateAbility(_) | ManaRestrictionDef::Special(_) => false,
            })
        && mana
            .spend_effects
            .iter()
            .copied()
            .all(|effect| match effect {
                ManaSpendEffectDef::ApplyToPaidSpell(effect) => {
                    shared_cannot_be_countered_effect(effect)
                }
                // A conditional rider is read where the mana is spent, against
                // the spell it paid for, so what it asks has to be answerable
                // there -- and what it grants has to be a keyword, which is what
                // the permanent the spell becomes carries away with it.
                ManaSpendEffectDef::ApplyToPaidSpellMatching { object, effect } => {
                    shared_object_predicate(object) && shared_granted_keyword_effect(effect)
                }
                ManaSpendEffectDef::ApplyToPaidAbility(_) | ManaSpendEffectDef::Special(_) => false,
            })
}

pub(super) fn shared_resolving_apply(
    recipient: EffectRecipientDef,
    effect: AppliedEffectDef,
    duration: ResolvedEffectDurationDef,
) -> bool {
    // "For as long as this source remains tapped" is recorded against its
    // source rather than a deadline. Only the two effect families whose
    // runtime readers consult that source-tapped expiration belong here.
    let while_source_tapped = resolving_effect_supports_while_source_tapped(effect);
    // Long-lived effects must consist entirely of leaves the permanent can
    // store. Until-end-of-turn retains the nonbattlefield ability-grant case.
    let long_lived = resolving_effect_supports_long_duration(effect);
    let next_matching_cast = matches!(
        effect,
        AppliedEffectDef::Rule(AppliedRuleDef::MayCastAsThoughItHadFlash(_))
    );
    // While-source-remains is stored like an indefinite effect; only its live
    // reader asks whether the recorded source is still on the battlefield.
    let requires_long_lived = duration.contains(ResolvedEffectDurationDef::UntilYourNextUpkeep)
        || duration.contains(ResolvedEffectDurationDef::UntilYourNextTurn)
        || duration.contains(ResolvedEffectDurationDef::WhileSourceRemains);
    let duration_is_supported = if duration == ResolvedEffectDurationDef::Permanent {
        long_lived
    } else {
        (!requires_long_lived || long_lived)
            && (!duration.contains(ResolvedEffectDurationDef::WhileSourceTapped)
                || while_source_tapped)
            && (!duration.contains(ResolvedEffectDurationDef::UntilNextMatchingCast)
                || next_matching_cast)
    };
    if !duration_is_supported || !shared_effect_recipient(recipient) {
        return false;
    }
    shared_resolving_applied_effect(effect)
}

fn resolving_effect_supports_long_duration(effect: AppliedEffectDef) -> bool {
    match effect {
        AppliedEffectDef::Composite(effects) => {
            !effects.is_empty()
                && effects
                    .iter()
                    .copied()
                    .all(resolving_effect_supports_long_duration)
        }
        AppliedEffectDef::Characteristic(_) => true,
        AppliedEffectDef::Rule(rule) => rule != AppliedRuleDef::CannotBeCountered,
    }
}

fn resolving_effect_supports_while_source_tapped(effect: AppliedEffectDef) -> bool {
    match effect {
        AppliedEffectDef::Composite(effects) => {
            !effects.is_empty()
                && effects
                    .iter()
                    .copied()
                    .all(resolving_effect_supports_while_source_tapped)
        }
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(_))
        | AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep) => true,
        AppliedEffectDef::Characteristic(_) | AppliedEffectDef::Rule(_) => false,
    }
}

pub(super) fn shared_resolving_applied_effect(effect: AppliedEffectDef) -> bool {
    match effect {
        AppliedEffectDef::Composite(effects) => {
            !effects.is_empty() && effects.iter().copied().all(shared_resolving_applied_effect)
        }
        AppliedEffectDef::Rule(AppliedRuleDef::CannotBeCountered) => false,
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
            AbilityOperationDef::Add(ability),
        )) => match ability.definition {
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
            | DeclarativeAbilityDef::Pregame(_)
            | DeclarativeAbilityDef::Static(_)
            | DeclarativeAbilityDef::OptionalAdditionalCost(_)
            | DeclarativeAbilityDef::SpecialAction(_)
            | DeclarativeAbilityDef::DeckConstruction(_) => false,
        },
        AppliedEffectDef::Characteristic(_) | AppliedEffectDef::Rule(_) => true,
    }
}

pub(super) fn battlefield_only(zones: &[ZoneKind]) -> bool {
    zones == [ZoneKind::Battlefield]
}

#[allow(clippy::too_many_lines)]
pub(super) fn shared_definition_ability(ability: &AbilityDef) -> bool {
    if let DeclarativeAbilityDef::Replacement(definition) = ability.definition {
        let Some(effect) = ability.declarative_replacement() else {
            return false;
        };
        return match definition.event {
            ReplacementEventDef::SourceEntersBattlefield => {
                battlefield_only(definition.source_zones)
                    && shared_replacement_event(definition.event)
                    && shared_entry_replacement_effect(effect)
            }
            ReplacementEventDef::ObjectEntersBattlefield { .. } => {
                !definition.optional
                    && definition.condition.is_none()
                    && battlefield_only(definition.source_zones)
                    && shared_replacement_event(definition.event)
                    && shared_entry_replacement_effect(effect)
            }
            ReplacementEventDef::WouldMove { from, to, cause } => {
                // "From anywhere" watches from every zone the card can be
                // in, so the clause's own source zones say so rather than
                // naming the one zone it leaves.
                let zones_match = match from {
                    Some(from) => definition.source_zones == [from],
                    None => {
                        definition.source_zones.contains(&ZoneKind::Battlefield)
                            && definition.source_zones.contains(&ZoneKind::Stack)
                    }
                };
                !definition.optional
                    && definition.condition.is_none()
                    && zones_match
                    && ((from == Some(ZoneKind::Hand)
                        && to == ZoneKind::Graveyard
                        && shared_zone_move_cause(cause)
                        && effect == ReplacementEffectDef::MoveToZone(ZoneKind::Battlefield))
                        || (matches!(from, None | Some(ZoneKind::Battlefield))
                            && to == ZoneKind::Graveyard
                            && cause == ZoneMoveCauseDef::Any
                            && shared_battlefield_exit_replacement_effect(effect)))
            }
            ReplacementEventDef::AnyObjectWouldMove { .. } => {
                !definition.optional
                    && definition.condition.is_none()
                    && battlefield_only(definition.source_zones)
                    && shared_replacement_event(definition.event)
                    // The same program a battlefield exit is held to: this
                    // event is that one plus every other zone it can start
                    // in, and both paths read the destination and the mark
                    // the same way.
                    && shared_battlefield_exit_replacement_effect(effect)
            }
            ReplacementEventDef::WouldBeDestroyed { object } => {
                !definition.optional
                    && definition.condition.is_none()
                    && battlefield_only(definition.source_zones)
                    && shared_object_predicate(object)
                    && shared_destruction_replacement_effect(effect)
            }
            ReplacementEventDef::WouldGainLife(_) => {
                !definition.optional
                    && definition.condition.is_none()
                    && battlefield_only(definition.source_zones)
                    && matches!(
                        effect,
                        ReplacementEffectDef::MultiplyEventAmount(_)
                            | ReplacementEffectDef::AddToEventAmount(_)
                    )
            }
            ReplacementEventDef::WouldBeginTurn { .. } => {
                definition
                    .condition
                    .is_none_or(|condition| condition == ReplacementConditionDef::SourceTapped)
                    && battlefield_only(definition.source_zones)
                    && shared_begin_turn_replacement_effect(effect)
            }
            ReplacementEventDef::WouldDraw { .. } => {
                battlefield_only(definition.source_zones)
                    && shared_draw_replacement_program(definition.condition, effect)
            }
            ReplacementEventDef::Special(_) => false,
        };
    }
    let Some(effect) = ability.declarative_effect() else {
        return false;
    };
    match ability.definition {
        DeclarativeAbilityDef::Spell(definition) => {
            if let Some(modal) = definition.modal() {
                shared_spell_additional_cost(modal.escalate_cost)
                    && modal.modes.iter().all(|mode| {
                        mode.declarative_effect().is_none() || shared_definition_ability(mode)
                    })
            } else {
                // A clause that exists only to carry an additional cost has
                // nothing to do on resolution, which is why None is allowed
                // here and nowhere else.
                let cost = definition.additional_cost();
                // Life is spent rather than named, so a life cost selects
                // nothing and needs no shape beyond the amount itself. The
                // caster's choice of X is bounded where the cast is offered.
                shared_spell_additional_cost(cost)
                    && (shared_stack_effect(effect)
                        || (cost.is_some() && effect == EffectDef::None))
            }
        }
        DeclarativeAbilityDef::ActivatedMana(definition) => {
            fn is_bounded(definition: &ActivatedAbilityDef) -> bool {
                // A printed "only once each turn" bounds the ability just as
                // surely as a cost that spends the board does, which is what
                // lets Vivi Ornitier's {0} be a cost at all.
                definition.activation_limit.is_some()
                    || definition.costs.iter().any(|cost| {
                        matches!(
                            cost,
                            AbilityCostDef::TapSource
                                | AbilityCostDef::ExertSource
                                | AbilityCostDef::DiscardHand
                                | AbilityCostDef::SacrificeSource
                                | AbilityCostDef::ExileSource
                                // Sacrificing another permanent bounds the
                                // ability the same way spending the source
                                // does.
                                | AbilityCostDef::SacrificePermanent { .. }
                                | AbilityCostDef::ExileCardFromHand(_)
                        )
                    })
            }

            let battlefield = battlefield_only(definition.source_zones);
            let hand = definition.source_zones == [ZoneKind::Hand]
                && definition.costs.as_slice() == [AbilityCostDef::ExileSource]
                && definition.activation_limit.is_none()
                && definition.condition.is_none();
            let command = definition.source_zones == [ZoneKind::Command]
                && !definition.costs.as_slice().is_empty()
                && definition
                    .costs
                    .iter()
                    .all(|cost| matches!(cost, AbilityCostDef::PayLife(_)))
                && definition.activation_limit.is_none()
                && definition.condition.is_none();

            (battlefield || hand || command)
                && definition.procedure == AbilityProcedureDef::Shared
                && !definition.costs.as_slice().is_empty()
                && definition.costs.iter().all(|cost| {
                    if hand || command {
                        return true;
                    }
                    matches!(
                        cost,
                        AbilityCostDef::TapSource
                            // Exerting spends the source's next untap step,
                            // which the mana path pays where it pays the tap.
                            | AbilityCostDef::ExertSource
                            // Discarding a hand takes every card and asks
                            // nothing, which is what a mana ability needs of
                            // a cost: no window in which to choose.
                            | AbilityCostDef::DiscardHand
                            | AbilityCostDef::SacrificeSource
                            | AbilityCostDef::ExileSource
                            | AbilityCostDef::RemoveCountersFromSource { .. }
                            // The mana path is the one place that enumerates
                            // an open-ended removal into one activation per
                            // size, so this is where it belongs.
                            | AbilityCostDef::RemoveAnyNumberOfCountersFromSource(_)
                            // And the one place that enumerates a "sacrifice
                            // a <thing>" cost into one activation per
                            // candidate, for the same reason.
                            | AbilityCostDef::SacrificePermanent { .. }
                            | AbilityCostDef::ExileCardFromHand(_)
                            // A loyalty cost bounds the ability by the rule
                            // rather than by the board: one loyalty ability
                            // per planeswalker per turn, and the mana path
                            // asks that question where it pays.
                            | AbilityCostDef::Loyalty(_)
                            | AbilityCostDef::PayLife(_)
                    ) || matches!(
                        cost,
                        // Mana is paid out of the pool, so the ability also
                        // has to be bounded some other way; flexible mana
                        // symbols and {X} would need a choice the activation
                        // cannot carry.
                        AbilityCostDef::Mana(mana)
                            if !mana.variable_x
                                && mana.hybrid_total() == 0
                                && is_bounded(&definition)
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
                // "Activate only if you control a Swamp or a Forest" is read
                // where the activation is offered, so its shape has to be
                // one the runtime can actually evaluate.
                && definition.condition.is_none_or(|condition| {
                    battlefield && shared_trigger_condition(*condition)
                })
                && shared_mana_effect(effect, battlefield)
        }
        DeclarativeAbilityDef::TriggeredMana(definition) => {
            fn immediate_mana_effect(effect: EffectDef) -> bool {
                match effect {
                    EffectDef::Sequence(effects) => {
                        !effects.is_empty() && effects.iter().copied().all(immediate_mana_effect)
                    }
                    EffectDef::AddMana(mana) => {
                        mana.amount > 0
                            && mana.variable_amount.is_none()
                            && matches!(
                                mana.recipient,
                                PlayerRefDef::EffectController
                                    | PlayerRefDef::ControllerOf(ObjectRefDef::TriggeringObject)
                            )
                            && !matches!(mana.mana, ManaSelectionDef::ColorsOfLinkedExiles)
                    }
                    // A triggered mana ability resolves without an offer to
                    // read an amount off, so this one stays outside.
                    EffectDef::BindOutput { .. }
                    | EffectDef::ContinueReplacedDraw
                    | EffectDef::AddManaEqualTo { .. }
                    | EffectDef::Randomized { .. }
                    | EffectDef::Choose(_)
                    | EffectDef::ChooseExact(_)
                    | EffectDef::ChooseCardsFromCollection(_)
                    | EffectDef::LookAtObjects(_)
                    | EffectDef::ChooseObjectOrder(_)
                    | EffectDef::ClassifyObjects(_)
                    | EffectDef::RevealAndClassifyCards(_)
                    | EffectDef::CombineObjects(_)
                    | EffectDef::ChooseOneOfEach(_)
                    | EffectDef::ChooseGroup(_)
                    | EffectDef::BindObjects(_)
                    | EffectDef::PartitionGroup(_)
                    | EffectDef::RandomizeObjectOrder(_)
                    | EffectDef::RevealObjects(_)
                    | EffectDef::MoveObjects(_)
                    | EffectDef::ChooseForEachPlayer(_)
                    | EffectDef::ChooseCardName { .. }
                    | EffectDef::SelectAtRandomFromZone { .. }
                    | EffectDef::ForEachInBinding { .. }
                    | EffectDef::PayOr(_)
                    | EffectDef::PreventDamage { .. }
                    | EffectDef::May { .. }
                    | EffectDef::None
                    | EffectDef::DealDamage { .. }
                    | EffectDef::DealDamageSimultaneously(_)
                    | EffectDef::DealDamageFrom { .. }
                    | EffectDef::DealDamageAndApply { .. }
                    | EffectDef::Fight { .. }
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
                    | EffectDef::CreateMyriadTokens
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
                    | EffectDef::PutOntoBattlefieldThen { .. }
                    | EffectDef::Cascade
                    | EffectDef::Proliferate
                    | EffectDef::Explore { .. }
                    | EffectDef::IfNoObjects(_)
                    | EffectDef::PutObjectsOntoBattlefieldFaceDown(_)
                    | EffectDef::PermitLookAtExiled { .. }
                    | EffectDef::LookAtHand { .. }
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
                    | EffectDef::CannotBeForcedToSacrifice
                    | EffectDef::CannotBeForcedToDiscard
                    | EffectDef::GainClassLevel { .. }
                    | EffectDef::SubstituteBasicLandTypeUntilEndOfTurn { .. }
                    | EffectDef::CreateEmblem { .. }
                    | EffectDef::CreateOngoingEffect(_)
                    | EffectDef::Transform { .. }
                    | EffectDef::ScheduleTurnPhases(_)
                    | EffectDef::TakeExtraTurn { .. }
                    | EffectDef::PutSourceOntoBattlefieldAttacking
                    | EffectDef::BecomeMonarch { .. }
                    | EffectDef::VoteForPermanentToExile { .. }
                    | EffectDef::DamageCannotBePreventedThisTurn
                    | EffectDef::ExileLinkedToSource { .. }
                    | EffectDef::MayPlayWithoutPaying { .. }
                    | EffectDef::ExileGrantingOwnerPlay { .. }
                    | EffectDef::ExileGrantingControllerPlayThisTurn { .. }
                    | EffectDef::PermitCastFromGraveyardThisTurn { .. }
                    | EffectDef::ReturnLinkedExiles { .. }
                    | EffectDef::Detain { .. }
                    | EffectDef::GainControl { .. }
                    | EffectDef::ExchangeControl { .. }
                    | EffectDef::InstallTrigger(_)
                    | EffectDef::IfCondition { .. }
                    | EffectDef::IfElseCondition { .. }
                    | EffectDef::ReduceGenericCostBy(_)
                    | EffectDef::ModifyCost(_)
                    | EffectDef::LandwalkCanBeBlocked(_)
                    | EffectDef::CannotAttackUnless(_)
                    | EffectDef::CannotAttackIf(_)
                    | EffectDef::PutIntoLibraryBeneathTop { .. }
                    | EffectDef::MoveToZone { .. }
                    | EffectDef::WithBattlefieldArrival { .. }
                    | EffectDef::WithZoneMoveResult { .. }
                    | EffectDef::ConditionalStatic(_)
                    | EffectDef::StaticApply { .. }
                    | EffectDef::Apply { .. }
                    | EffectDef::Special(_) => false,
                }
            }
            definition.condition.is_none()
                && definition.procedure == AbilityProcedureDef::Shared
                && battlefield_only(definition.source_zones)
                && matches!(
                    definition.event,
                    TriggerEventDef::Tapped(matcher)
                        if matcher.purpose == crate::card::TapPurposeDef::Mana
                )
                && shared_trigger_event(definition.event)
                && immediate_mana_effect(effect)
        }
        DeclarativeAbilityDef::Activated(definition) => {
            matches!(
                    definition.source_zones,
                    [ZoneKind::Battlefield | ZoneKind::Hand | ZoneKind::Graveyard | ZoneKind::Exile]
                ) && definition.procedure == AbilityProcedureDef::Shared
                    && shared_activated_costs(definition.source_zones, definition.costs.as_slice())
                    // Only the mana path enumerates one activation per
                    // removable count, so an open-ended removal outside it
                    // would leave the size unanswered.
                    && (!definition.costs.iter().any(|cost| {
                        matches!(
                            cost,
                            AbilityCostDef::RemoveAnyNumberOfCountersFromSource(_)
                        )
                    }) || matches!(effect, EffectDef::AddMana(_)))
                    // Conservative rather than forced: activations now
                    // enumerate their targets per affordable X, so a divided
                    // slot would have somewhere to live. Nothing prints one
                    // on an activated ability yet, so nothing has exercised
                    // the division arithmetic on this path.
                    && definition
                        .targets
                        .iter()
                        .all(|slot| slot.divided_total.is_none())
                    && definition
                        .condition
                        .is_none_or(|condition| shared_trigger_condition(*condition))
                    && definition.modes.is_none_or(|modal| {
                        modal.modes.iter().all(|mode| {
                            mode.declarative_effect().is_none() || shared_definition_ability(mode)
                        })
                    })
                    // A purely modal ability does nothing of its own before
                    // its chosen modes, which is the one place an activated
                    // ability may resolve to nothing.
                    && (shared_stack_effect(effect)
                        || (definition.modes.is_some() && effect == EffectDef::None))
        }
        DeclarativeAbilityDef::Triggered(definition) => {
            // A state trigger is nothing but its condition: without one it
            // would trigger on every state-based check forever.
            let condition_is_required = definition.event != TriggerEventDef::StateCondition
                || definition.condition.is_some();
            // A trigger listens from the battlefield, graveyard, or exile,
            // or -- for the one clause no single walk sees -- from the first
            // two together. A permanent that dies is captured off a snapshot
            // taken before it left, when the graveyard walk cannot see it
            // yet; a card discarded or milled is captured after it lands,
            // when the battlefield walk never held it. Exile has its own
            // listener walk for Suspend and printed exile abilities. Every
            // other event is found from whichever one zone the card is in,
            // so claiming multiple zones would be an authoring mistake.
            (matches!(
                definition.source_zones,
                [ZoneKind::Battlefield | ZoneKind::Graveyard | ZoneKind::Exile]
            ) || matches!(
                (definition.source_zones, definition.event),
                (
                    [ZoneKind::Battlefield, ZoneKind::Graveyard],
                    TriggerEventDef::ZoneChanged(matcher),
                ) if matcher.from.is_none() && matcher.to == Some(ZoneKind::Graveyard)
            )) && definition.procedure == AbilityProcedureDef::Shared
                && shared_trigger_event(definition.event)
                && condition_is_required
                && definition
                    .condition
                    .is_none_or(|condition| shared_trigger_condition(*condition))
                && definition.modes.is_none_or(|modal| {
                    // At most one mode, because a trigger carries one
                    // program: what placement puts on the stack is the
                    // mode's own effect and targets. A minimum of zero is
                    // the printed "choose up to one", where declining
                    // leaves the trigger carrying nothing.
                    modal.minimum <= 1
                        && modal.maximum == 1
                        && !modal.may_repeat
                        && modal.escalate_cost.is_none()
                        && modal.conditional_maximum.is_none()
                        && modal.modes.iter().all(|mode| {
                            mode.declarative_effect().is_none() || shared_definition_ability(mode)
                        })
                })
                // A purely modal trigger does nothing of its own before the
                // mode it prints, exactly as a modal activated ability does.
                && (shared_stack_effect(effect)
                    || (definition.modes.is_some() && effect == EffectDef::None))
        }
        DeclarativeAbilityDef::Pregame(definition) => {
            definition
                .costs
                .iter()
                .all(|cost| matches!(cost, AbilityCostDef::ExileCardFromHand(_)))
                && definition.costs.len() <= 1
                && shared_stack_effect(effect)
        }
        DeclarativeAbilityDef::Static(definition) => {
            effect == EffectDef::None || shared_static_effect(definition.source_zones, effect)
        }
        DeclarativeAbilityDef::Replacement(_) => unreachable!("handled before ordinary effects"),
        DeclarativeAbilityDef::AlternativeCast(definition) => match definition.kind {
            // These are permissions to cast rather than effects of their
            // own. The card's spell clause does the work; for a face-down
            // cast nothing does, which is the point. Impending and dash
            // change only how the permanent arrives, and the card's own
            // clauses say what that change is.
            AlternativeCastKindDef::Flashback
            | AlternativeCastKindDef::WithoutPayingManaCost
            | AlternativeCastKindDef::Foretell
            // Rebound's free cast, which like the others above only says
            // what the cast costs and where it is taken from.
            | AlternativeCastKindDef::Rebound
            // Plot is not a cast at all: the clause exists so the plot cost
            // has somewhere printed to live.
            | AlternativeCastKindDef::Plot
            // Splice is not a cast either: the card stays in hand and its
            // clause is added to somebody else's spell.
            | AlternativeCastKindDef::Splice
            | AlternativeCastKindDef::Escape
            | AlternativeCastKindDef::Retrace
            | AlternativeCastKindDef::Impending
            | AlternativeCastKindDef::Dash
            // Offspring changes only what the cast cost, which the arrival
            // trigger reads off the permanent afterwards.
            | AlternativeCastKindDef::Offspring
            | AlternativeCastKindDef::Warp
            // Emerge says only what the cast costs and what is sacrificed
            // to reach it.
            | AlternativeCastKindDef::Emerge
            | AlternativeCastKindDef::Miracle
            | AlternativeCastKindDef::AlternativeCost
            | AlternativeCastKindDef::FaceDown { .. } => effect == EffectDef::None,
            // Overload carries the instructions the modified spell resolves
            // with, so it has to be an effect the shared runtime can execute.
            // Overload and bestow both carry the instructions the modified
            // spell resolves with -- "each" instead of "target" for one, an
            // attach for the other -- so both have to be effects the shared
            // runtime can execute.
            AlternativeCastKindDef::Overload | AlternativeCastKindDef::Bestow => {
                shared_stack_effect(effect)
            }
            // A kicker either replaces the instructions or does not. When it
            // does not, the spell resolves exactly as printed and being
            // kicked is only a fact its other clauses can read.
            AlternativeCastKindDef::Kicked => {
                effect == EffectDef::None || shared_stack_effect(effect)
            }
            // What a cast wears when another permanent supplied the cost.
            // No card prints it, so no card may claim it.
            AlternativeCastKindDef::Granted => false,
        },
        // Neither clause resolves anything: a cost clause has already been
        // paid where the spell was announced, and a deck-construction
        // permission is read while a deck is assembled and never while a
        // game runs. Both are shared exactly when they do nothing.
        DeclarativeAbilityDef::OptionalAdditionalCost(_)
        | DeclarativeAbilityDef::DeckConstruction(_) => effect == EffectDef::None,
        DeclarativeAbilityDef::Keyword(keyword) => shared_keyword(keyword),
        DeclarativeAbilityDef::SpecialAction(_) => false,
    }
}
