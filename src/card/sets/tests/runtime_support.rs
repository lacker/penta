mod conditions;
mod nested_definitions;
mod stack_effects;
mod static_effects;

pub(super) use conditions::*;
pub(super) use static_effects::shared_static_effect;

pub(super) use nested_definitions::*;
pub(super) use stack_effects::shared_stack_effect;

use crate::Game;
use crate::card::{
    ActivatedAbilityDef, AppliedRuleDef, BlockRestrictionMatchDef, CostModificationDef,
    ReplacementConditionDef, SpellAdditionalCostDef,
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
        | ObjectPredicateDef::PowerLessThan(_)
        | ObjectPredicateDef::ToughnessGreaterThanItsPower
        | ObjectPredicateDef::ToughnessGreaterThan(_)
        | ObjectPredicateDef::WasDealtDamageThisTurn
        | ObjectPredicateDef::DealtDamageThisTurn
        | ObjectPredicateDef::ControlledBy(_)
        | ObjectPredicateDef::OwnedBy(_)
        | ObjectPredicateDef::Supertype(_)
        | ObjectPredicateDef::DebutSet(_)
        | ObjectPredicateDef::SharesNameWithSource
        | ObjectPredicateDef::HasSourcesChosenScalar(_)
        | ObjectPredicateDef::TargetsObjectMatching(_)
        | ObjectPredicateDef::AttackingOrBlocking
        | ObjectPredicateDef::HasKeyword(_)
        | ObjectPredicateDef::HasCounter(_)
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
            | ObjectSetDef::Binding(_)
            | ObjectSetDef::MatchingBinding { .. }
            | ObjectSetDef::LinkedExiles(_)
            | ObjectSetDef::CardsDrawnThisTurnInHand(_)
            | ObjectSetDef::BottomOfGraveyard(_)
            | ObjectSetDef::LegalTargets(_)
            | ObjectSetDef::SharingNameWith(_)
            | ObjectSetDef::SharingNameWithBinding { .. }
            | ObjectSetDef::TopOfGraveyardMatching { .. },
        )
        // Both kinds at once is shared for the same reason each half is:
        // the players come from a relation, and the creatures they control
        // from the ordinary battlefield walk.
        | EffectRecipientSetDef::Players(_)
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
    [query.related_player, query.controller, query.owner]
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
            | KeywordAbility::Menace
            | KeywordAbility::Undying
            | KeywordAbility::Indestructible
            | KeywordAbility::Landwalk(_)
            | KeywordAbility::LegendaryLandwalk
            | KeywordAbility::AttacksEachCombatIfAble
            | KeywordAbility::Banding
            | KeywordAbility::BandsWithOther(_)
            | KeywordAbility::Unleash
            | KeywordAbility::ProtectionFrom(_)
            // The colourlessness is the card's printed colour set, so the
            // keyword itself has nothing left to execute.
            | KeywordAbility::Infect
            | KeywordAbility::Devoid
            | KeywordAbility::Compleated
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
        ManaSelectionDef::One(_) => true,
        // Both selections offer a choice of type; a combination divides the
        // amount across those types instead of picking one, and the runtime
        // enumerates every division for the same reason it enumerates every
        // colour.
        ManaSelectionDef::Choice(colors) | ManaSelectionDef::Combination(colors) => {
            choices_are_supported && !colors.is_empty()
        }
        // The same choice of type, over a list the runtime reads off the
        // permanent's own imprint rather than off the clause.
        ManaSelectionDef::ColorsOfLinkedExiles => choices_are_supported,
    };
    // "Where X is this creature's power" is resolved against the permanent
    // as the ability is offered, exactly as the counted forms above are, so
    // a printed amount of zero is the whole amount only when no value
    // replaces it.
    let amount_is_known = mana.amount > 0
        || matches!(
            mana.variable_amount,
            Some(ValueDef::CountersOnSource(_) | ValueDef::SourcePower)
        );
    selection_is_supported
        && amount_is_known
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
    duration: ResolvedEffectDurationDef,
) -> bool {
    // "For as long as this source remains tapped" is recorded against its
    // source rather than a deadline. Only the two effect families whose
    // runtime readers consult that source-tapped expiration belong here.
    let while_source_tapped = resolving_effect_supports_while_source_tapped(effect);
    // Long-lived effects must consist entirely of leaves the permanent can
    // store. Until-end-of-turn retains the nonbattlefield ability-grant case.
    let long_lived = resolving_effect_supports_long_duration(effect);
    let duration_is_supported = matches!(
        duration,
        // An end-of-combat effect expires one step earlier than an
        // end-of-turn one and is stored the same way, so whatever the shorter
        // duration can carry the longer one can too.
        ResolvedEffectDurationDef::UntilEndOfTurn | ResolvedEffectDurationDef::UntilEndOfCombat
    ) || duration == ResolvedEffectDurationDef::UntilYourNextUpkeep
        && long_lived
        || matches!(
            duration,
            ResolvedEffectDurationDef::UntilYourNextTurn | ResolvedEffectDurationDef::Permanent
        ) && long_lived
        || duration == ResolvedEffectDurationDef::WhileSourceTapped && while_source_tapped;
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
            | DeclarativeAbilityDef::Static(_)
            | DeclarativeAbilityDef::OptionalAdditionalCost(_)
            | DeclarativeAbilityDef::SpecialAction(_)
            | DeclarativeAbilityDef::Legacy => false,
        },
        // Stack-object rules use `AppliedStackEffect`; every other typed rule
        // is stored on a permanent with this Apply's timestamp and duration.
        AppliedEffectDef::Rule(AppliedRuleDef::CannotBeCountered) => false,
        AppliedEffectDef::Characteristic(_) | AppliedEffectDef::Rule(_) => true,
    }
}

pub(super) fn shared_activated_costs(source_zones: &[ZoneKind], costs: &[AbilityCostDef]) -> bool {
    let battlefield = source_zones == [ZoneKind::Battlefield];
    let hand = source_zones == [ZoneKind::Hand];
    let graveyard = source_zones == [ZoneKind::Graveyard];
    let sacrifice_choices = costs
        .iter()
        .filter(|cost| matches!(cost, AbilityCostDef::SacrificePermanent { .. }))
        .count();
    let fixed_sacrifices = costs
        .iter()
        .filter(|cost| matches!(cost, AbilityCostDef::SacrificeObject(_)))
        .count();
    let source_exit_costs = costs
        .iter()
        .filter(|cost| {
            matches!(
                cost,
                AbilityCostDef::SacrificeSource
                    | AbilityCostDef::ExileSource
                    | AbilityCostDef::ReturnSourceToHand
            )
        })
        .count();
    sacrifice_choices <= 1
        && fixed_sacrifices <= 1
        && source_exit_costs <= 1
        && costs.iter().all(|cost| match cost {
            // A variable X is offered one activation per affordable
            // value. More than one X in the same cost is not: nothing
            // enumerates a cost that charges X twice.
            AbilityCostDef::Mana(cost) => cost.x_multiplier <= 1,
            // The chosen object comes from the battlefield or from the
            // activating player's own graveyard, so only the predicate
            // needs checking.
            // The discard reads the payer's hand rather than the
            // battlefield, but the shape is the same: a permanent to activate
            // from and a predicate the shared walk can read.
            // The many-at-once form is paid by a decision rather than by
            // enumeration, which asks the same question of the same walk.
            AbilityCostDef::SacrificePermanent { object, .. }
            | AbilityCostDef::SacrificePermanents { object, .. }
            | AbilityCostDef::TapPermanent { object, .. }
            | AbilityCostDef::ExileCardsFromGraveyard { object, .. }
            | AbilityCostDef::DiscardCardMatching(object)
            | AbilityCostDef::ExileCardFromHand(object) => {
                battlefield && shared_object_predicate(*object)
            }
            // Exiling the source is the one cost a card can pay from its own
            // graveyard; the rest of these need a permanent to act on.
            AbilityCostDef::ExileSource => battlefield || graveyard,
            // A fixed object sacrifice is supported only when it names the
            // source whose activation is being checked.
            AbilityCostDef::SacrificeObject(
                ObjectRefDef::Source | ObjectRefDef::AbilityGrantSource,
            )
            | AbilityCostDef::TapSource
            | AbilityCostDef::UntapSource
            | AbilityCostDef::SacrificeSource
            // The source leaves the battlefield to pay either way; only
            // where it lands differs.
            | AbilityCostDef::ReturnSourceToHand
            | AbilityCostDef::RemoveCountersFromSource { .. }
            // Open-ended only in the declaration: one activation per size is
            // built by the mana path, which is why the caller also requires
            // the effect to be an AddMana.
            | AbilityCostDef::RemoveAnyNumberOfCountersFromSource(_)
            | AbilityCostDef::PayLife(_)
            | AbilityCostDef::Loyalty(_)
            // Nobody chooses which cards go, so a random discard needs no
            // decision procedure -- only a permanent to activate from.
            | AbilityCostDef::DiscardCardsAtRandom(_)
            // Crew and saddle name no predicate: what may pay is every other
            // untapped creature the payer controls, and the decision that
            // asks reads the battlefield directly.
            | AbilityCostDef::TapCreaturesWithTotalPower { .. } => battlefield,
            // Ninjutsu's cost joins the discard here: what it may return is
            // combat state rather than a predicate, and both are paid by a
            // card in hand.
            AbilityCostDef::DiscardSource
            | AbilityCostDef::ReturnUnblockedAttackerToHand => hand,
            AbilityCostDef::SacrificeObject(
                ObjectRefDef::ResolvingObject
                | ObjectRefDef::Binding(_)
                | ObjectRefDef::AttachedToSource
                | ObjectRefDef::Target(_)
                | ObjectRefDef::TriggeringObject
                | ObjectRefDef::SourceOfTargetedStackObject(_),
            )
            | AbilityCostDef::DiscardCards(_)
            | AbilityCostDef::Special(_) => false,
        })
}

/// Only one object is chosen, and only from the two places the casting
/// enumeration looks: the caster's own battlefield and graveyard.
fn shared_spell_additional_cost(cost: Option<SpellAdditionalCostDef>) -> bool {
    let Some(cost) = cost else {
        return true;
    };
    // Each way of paying has to be one the runtime can enumerate, and all of
    // them have to spend what they name the same way: the payment path reads
    // one spend mode for the whole cost, and picks the zone per object.
    cost.alternatives().into_iter().all(|alternative| {
        alternative.spend == cost.spend
            // A cost counted from something else has no printed number to
            // check: what makes it payable is the X the spell is cast for,
            // or how many modes were chosen.
            && (alternative.counted != crate::card::SpellAdditionalCostCountDef::Printed
                || alternative.count >= 1)
            && matches!(
                alternative.zone,
                ZoneKind::Battlefield | ZoneKind::Graveyard | ZoneKind::Hand
            )
            && shared_object_predicate(alternative.object)
    })
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
                    && effect == ReplacementEffectDef::MoveToZone(ZoneKind::Exile)
            }
            ReplacementEventDef::WouldGainLife(_) => {
                !definition.optional
                    && definition.condition.is_none()
                    && battlefield_only(definition.source_zones)
                    && matches!(effect, ReplacementEffectDef::MultiplyEventAmount(_))
            }
            ReplacementEventDef::WouldBeginTurn { .. } => {
                definition
                    .condition
                    .is_none_or(|condition| condition == ReplacementConditionDef::SourceTapped)
                    && battlefield_only(definition.source_zones)
                    && shared_begin_turn_replacement_effect(effect)
            }
            ReplacementEventDef::WouldDraw { .. } => {
                // A hand size is read where the draw instruction is, which
                // is the one condition this walk can answer.
                definition.condition.is_none_or(|condition| {
                    matches!(condition, ReplacementConditionDef::ControllerHandAtMost(_))
                }) && battlefield_only(definition.source_zones)
                    && shared_draw_replacement_effect(effect)
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
                modal.modes.iter().all(|mode| {
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
                    EffectDef::AddMana(_) => shared_mana_effect(effect, false),
                    // A triggered mana ability resolves without an offer to
                    // read an amount off, so this one stays outside.
                    EffectDef::AddManaEqualTo { .. }
                    | EffectDef::Randomized { .. }
                    | EffectDef::Choose(_)
                    | EffectDef::ChooseCardName { .. }
                    | EffectDef::BindMatching { .. }
                    | EffectDef::ForEachInBinding { .. }
                    | EffectDef::PayOr(_)
                    | EffectDef::SplitIntoPiles(_)
                    | EffectDef::PreventDamage { .. }
                    | EffectDef::May { .. }
                    | EffectDef::None
                    | EffectDef::DealDamage { .. }
                    | EffectDef::DealDamageFrom { .. }
                    | EffectDef::DealDamageAndApply { .. }
                    | EffectDef::DrainLife { .. }
                    | EffectDef::GainLife { .. }
                    | EffectDef::AddPoisonCounters { .. }
                    | EffectDef::AddEnergyCounters { .. }
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
                    | EffectDef::DestroyAtEndOfCombat { .. }
                    | EffectDef::SkipNextUntapSteps { .. }
                    | EffectDef::DoubleCounters { .. }
                    | EffectDef::RemoveAllCounters { .. }
                    | EffectDef::Untap { .. }
                    | EffectDef::Saddle { .. }
                    | EffectDef::Attach { .. }
                    | EffectDef::AttachToSource { .. }
                    | EffectDef::PhaseOut { .. }
                    | EffectDef::ReturnAttached { .. }
                    | EffectDef::Reconfigure { .. }
                    | EffectDef::Unattach { .. }
                    | EffectDef::CreateToken { .. }
                    | EffectDef::CreateAttachedToken { .. }
                    | EffectDef::CreateTokenCopyOf { .. }
                    | EffectDef::Endure { .. }
                    | EffectDef::CreateMyriadTokens
                    | EffectDef::Destroy { .. }
                    | EffectDef::Sacrifice { .. }
                    | EffectDef::SacrificeKeepingOnePerType { .. }
                    | EffectDef::SacrificeOfChoice { .. }
                    | EffectDef::ExileTopOfLibraryToPlay { .. }
                    | EffectDef::ExileAtRandomFromGraveyardToPlay { .. }
                    | EffectDef::ExileTopAndMayCast { .. }
                    | EffectDef::MayCastTargetWithoutPaying { .. }
                    | EffectDef::Mill { .. }
                    | EffectDef::SearchZonesAndExileRest { .. }
                    | EffectDef::MillUntil { .. }
                    | EffectDef::ExileFromTopUntil { .. }
                    | EffectDef::ManifestDread { .. }
                    | EffectDef::ReturnWithHasteAndFinality { .. }
                    | EffectDef::Cascade
                    | EffectDef::Proliferate
                    | EffectDef::Explore { .. }
                    | EffectDef::LookAtTopAndSelect { .. }
                    | EffectDef::LookAtHand { .. }
                    | EffectDef::LookAtRandomCardInHand { .. }
                    | EffectDef::RevealAtRandomFromHand { .. }
                    | EffectDef::RevealHand { .. }
                    | EffectDef::SearchZone { .. }
                    | EffectDef::ChooseCards { .. }
                    | EffectDef::ReplaceNextDrawThisTurn { .. }
                    | EffectDef::IfFormat { .. }
                    | EffectDef::Counter { .. }
                    | EffectDef::ReturnSpellToHand { .. }
                    | EffectDef::PutSpellIntoOwnersLibrary { .. }
                    | EffectDef::CopyResolvingSpell { .. }
                    | EffectDef::AddCounters { .. }
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
                    | EffectDef::GrantFlashToNextSorcery
                    | EffectDef::ExileLinkedToSource { .. }
                    | EffectDef::MayPlayWithoutPaying { .. }
                    | EffectDef::ExileGrantingOwnerPlay { .. }
                    | EffectDef::ReturnLinkedExiles { .. }
                    | EffectDef::Detain { .. }
                    | EffectDef::GainControl { .. }
                    | EffectDef::ExchangeControl { .. }
                    | EffectDef::InstallTrigger(_)
                    | EffectDef::IfCondition { .. }
                    | EffectDef::ReduceGenericCostBy(_)
                    | EffectDef::ModifyCost(_)
                    | EffectDef::LandwalkCanBeBlocked(_)
                    | EffectDef::CannotAttackUnless(_)
                    | EffectDef::CannotAttackIf(_)
                    | EffectDef::PutIntoLibraryBeneathTop { .. }
                    | EffectDef::MoveToZone { .. }
                    | EffectDef::StaticApply { .. }
                    | EffectDef::Apply { .. }
                    | EffectDef::PairWithSource { .. }
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
                    [ZoneKind::Battlefield | ZoneKind::Hand | ZoneKind::Graveyard]
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
            // A trigger listens from the battlefield, from a graveyard, or
            // -- for the one clause no single walk sees -- from both. A
            // permanent that dies is captured off a snapshot taken before it
            // left, when the graveyard walk cannot see it yet; a card
            // discarded or milled is captured after it lands, when the
            // battlefield walk never held it. Every other event is found
            // from whichever zone the card is in, so claiming both would be
            // an authoring mistake rather than a listener.
            (matches!(
                definition.source_zones,
                [ZoneKind::Battlefield | ZoneKind::Graveyard]
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
                    // One mode, because a trigger carries one program: what
                    // placement puts on the stack is the mode's own effect
                    // and targets.
                    modal.minimum == 1
                        && modal.maximum == 1
                        && !modal.may_repeat
                        && modal.additional_cost.is_none()
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
        DeclarativeAbilityDef::Static(definition) => {
            (effect == EffectDef::None
                && ability.coverage.status == ImplementationStatus::Complete
                && ability.coverage.explanation.is_some())
                || shared_static_effect(definition.source_zones, effect)
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
            // Plot is not a cast at all: the clause exists so the plot cost
            // has somewhere printed to live.
            | AlternativeCastKindDef::Plot
            | AlternativeCastKindDef::Escape
            | AlternativeCastKindDef::Retrace
            | AlternativeCastKindDef::Impending
            | AlternativeCastKindDef::Dash
            // Offspring changes only what the cast cost, which the arrival
            // trigger reads off the permanent afterwards.
            | AlternativeCastKindDef::Offspring
            | AlternativeCastKindDef::Warp
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
        },
        DeclarativeAbilityDef::OptionalAdditionalCost(_) => effect == EffectDef::None,
        DeclarativeAbilityDef::Keyword(keyword) => shared_keyword(keyword),
        DeclarativeAbilityDef::SpecialAction(_) | DeclarativeAbilityDef::Legacy => false,
    }
}
