mod nested_definitions;

pub(super) use nested_definitions::*;

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
        EffectRecipientDef::ObjectsSharingNameWithTarget(_)
        | EffectRecipientDef::Source
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
        | AppliedEffectDef::CannotBeEnchanted
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
        | AppliedEffectDef::CannotBeEnchanted
        | AppliedEffectDef::CannotBeBlockedBy(_)
        | AppliedEffectDef::PreventDamageFrom(_)
        | AppliedEffectDef::AddLandTypes(_)
        | AppliedEffectDef::SetLandTypes(_)
        | AppliedEffectDef::Special(_) => false,
    }
}

pub(super) fn shared_stack_effect(effect: EffectDef) -> bool {
    shared_stack_effect_at_position(effect, true)
}

/// A queued decision returns control to the decision procedure instead of
/// suspending its caller. It is therefore safe at the root of a resolving
/// effect (and may wrap a whole sequence), but not as one component of a
/// sequence whose remaining components would otherwise resolve first.
/// The effects whose whole procedure is a decision the shared runtime
/// asks for. Their callers have already established that a deferred
/// decision is allowed where they sit; this checks only their arguments.
pub(super) fn shared_decision_effect(effect: EffectDef) -> bool {
    match effect {
        // Both halves of the split are asked for, so only the player
        // needs checking.
        EffectDef::SplitPermanentsAndSacrificeAPile { player } => shared_effect_recipient(player),
        // The reveal, the split, and the choice are all asked for, and
        // the library is the resolving object's controller's own.
        EffectDef::RevealAndSplitIntoPiles { .. } => true,
        // Looking is private and the offer is the only visible part, and
        // a chosen destruction reaches only the chooser's own battlefield.
        EffectDef::LookAtTopAndMayTake { player, object }
        | EffectDef::DestroyOfChoice { player, object, .. } => {
            shared_effect_recipient(player) && shared_object_predicate(object)
        }
        _ => false,
    }
}

/// The chooser is a player and the choices are their own battlefield, so
/// only the predicate needs checking. The follow-up runs inside the
/// sacrifice's continuation, where a further deferred decision has
/// nowhere to resume.
pub(super) fn shared_sacrifice_of_choice(effect: EffectDef) -> bool {
    let EffectDef::SacrificeOfChoice {
        player,
        object,
        then,
        ..
    } = effect
    else {
        return false;
    };
    shared_effect_recipient(player)
        && shared_object_predicate(object)
        && then.is_none_or(|effect| shared_stack_effect_at_position(*effect, false))
}

pub(super) fn shared_stack_effect_at_position(
    effect: EffectDef,
    deferred_decision_allowed: bool,
) -> bool {
    match effect {
        EffectDef::Sequence(effects) => {
            !effects.is_empty()
                && effects
                    .iter()
                    .copied()
                    .all(|effect| shared_stack_effect_at_position(effect, false))
        }
        EffectDef::AddMana(_) => shared_mana_effect(effect, false),
        EffectDef::DealDamage { recipient, .. }
        | EffectDef::DrainLife { recipient, .. }
        | EffectDef::GainLife { recipient, .. }
        | EffectDef::DrawCards { recipient, .. }
        | EffectDef::Discard { recipient, .. }
        | EffectDef::LoseLife { recipient, .. }
        | EffectDef::Mill {
            player: recipient, ..
        }
        | EffectDef::CannotCastNoncreatureSpellsThisTurn { player: recipient }
        | EffectDef::LoseTheGame { player: recipient }
        | EffectDef::LookAtHand { player: recipient } => shared_effect_recipient(recipient),
        EffectDef::SacrificeOfChoice { .. } => shared_sacrifice_of_choice(effect),
        // The choice is asked of whoever controls the candidates, and the
        // candidates are their own battlefield, so only the player and
        // the predicate need checking.
        EffectDef::DestroyOfChoice { .. }
        | EffectDef::SplitPermanentsAndSacrificeAPile { .. }
        | EffectDef::RevealAndSplitIntoPiles { .. }
        | EffectDef::LookAtTopAndMayTake { .. } => {
            deferred_decision_allowed && shared_decision_effect(effect)
        }
        EffectDef::SearchLibrary {
            player,
            object,
            destination,
        } => {
            shared_effect_recipient(player)
                && shared_object_predicate(object)
                && matches!(destination, ZoneKind::Battlefield | ZoneKind::Hand)
        }
        // Only the two destinations the return path knows.
        EffectDef::ReturnLinkedExiles { zone, .. } => {
            matches!(zone, ZoneKind::Battlefield | ZoneKind::Hand)
        }
        EffectDef::Tap { object }
        | EffectDef::Untap { object }
        | EffectDef::PreventCombatDamageThisTurn { object }
        | EffectDef::Destroy { object, .. }
        | EffectDef::Sacrifice { object }
        | EffectDef::ExileLinkedToSource { object }
        | EffectDef::MakeUnblockableThisTurn { object }
        | EffectDef::GainControlThisTurn { object }
        | EffectDef::AddCounters { object, .. }
        | EffectDef::Attach { object }
        | EffectDef::ChangeTextBasicLandType { object }
        | EffectDef::BecomeCopyOf { object, .. } => shared_effect_recipient(object),
        // Only the two destinations counter_spell_into knows.
        EffectDef::Counter { object, zone } | EffectDef::CounterUnlessPaid { object, zone, .. } => {
            matches!(zone, ZoneKind::Graveyard | ZoneKind::Exile) && shared_effect_recipient(object)
        }
        // Neither needs a recipient: a token is created under the
        // resolving object's controller, and the flash grant is about its
        // controller's next spell.
        // The amount is computed when the effect resolves, so nothing has
        // to read it ahead of time the way a mana ability does.
        EffectDef::AddManaEqualTo { .. }
        | EffectDef::CreateToken { .. }
        | EffectDef::CreateEmblem { .. }
        | EffectDef::Transform { .. }
        | EffectDef::AdditionalCombatPhase
        | EffectDef::GrantFlashToNextSorcery => true,
        // Each of these asks a question and then runs an inner effect,
        // so the question has to be allowed here and the answer has to be
        // something the shared procedure can carry out.
        EffectDef::May(effect)
        | EffectDef::OptionalManaPayment { effect, .. }
        | EffectDef::UnlessPaid {
            otherwise: effect, ..
        } => deferred_decision_allowed && shared_stack_effect_at_position(*effect, true),
        // Scheduling creates a fresh resolution boundary. A decision may
        // therefore be the delayed effect's root even when scheduling it
        // is itself one component of a sequence.
        EffectDef::IfCondition { then: effect, .. } | EffectDef::AtNextStep { effect, .. } => {
            shared_stack_effect_at_position(*effect, true)
        }
        // Installing an ability is a resolution like any other; what it
        // installs has to be an ability the shared runtime can fire.
        EffectDef::TriggerUntilYourNextTurn { ability } => shared_definition_ability(ability),
        EffectDef::Apply {
            recipient,
            effect,
            duration,
        } => shared_resolving_apply(recipient, effect, duration),
        // Only the moves the runtime actually performs are inside the
        // boundary. A move to the stack or command zone is still a seam.
        EffectDef::MoveToZone { object, zone, .. } => {
            matches!(
                zone,
                ZoneKind::Battlefield
                    | ZoneKind::Hand
                    | ZoneKind::Graveyard
                    | ZoneKind::Exile
                    | ZoneKind::Library
            ) && shared_effect_recipient(object)
        }
        EffectDef::None
        | EffectDef::CannotBeForcedToSacrifice
        | EffectDef::ReduceGenericCostBy(_)
        | EffectDef::PlayersCantPlay(_)
        | EffectDef::MultiplyEventAmount(_)
        | EffectDef::Replacement(_)
        | EffectDef::ChooseCardName { .. }
        | EffectDef::ChoosePlayer { .. }
        | EffectDef::CopyPermanentAsItEnters { .. }
        | EffectDef::ChooseCreatureType { .. }
        | EffectDef::Special(_) => false,
    }
}

pub(super) fn shared_trigger_event(event: TriggerEventDef) -> bool {
    match event {
        TriggerEventDef::ZoneChanged { object, from, to } => {
            const COMMITTED_TRANSITIONS: [(ZoneKind, ZoneKind); 5] = [
                (ZoneKind::Hand, ZoneKind::Battlefield),
                (ZoneKind::Stack, ZoneKind::Battlefield),
                (ZoneKind::Battlefield, ZoneKind::Graveyard),
                (ZoneKind::Battlefield, ZoneKind::Exile),
                (ZoneKind::Battlefield, ZoneKind::Hand),
            ];
            shared_object_predicate(object)
                && COMMITTED_TRANSITIONS
                    .iter()
                    .any(|(actual_from, actual_to)| {
                        from.is_none_or(|expected| expected == *actual_from)
                            && to.is_none_or(|expected| expected == *actual_to)
                    })
        }
        TriggerEventDef::BecomesTapped(object)
        | TriggerEventDef::Attacks(object)
        | TriggerEventDef::AttacksFirstTimeThisTurn(object)
        | TriggerEventDef::TappedForMana(object)
        | TriggerEventDef::SpellCast(object) => shared_object_predicate(object),
        TriggerEventDef::StepBegins { .. }
        | TriggerEventDef::LifeGained(_)
        | TriggerEventDef::StateCondition
        | TriggerEventDef::TransformsIntoThisFace
        | TriggerEventDef::DamagedCreatureDied => true,
        // Only "whenever this creature is dealt damage" is committed; a
        // wider recipient has no event behind it yet.
        TriggerEventDef::DamageDealt { source, recipient } => {
            recipient == EffectRecipientDef::Source && source == ObjectPredicateDef::Any
        }
        TriggerEventDef::CombatDamageDealtToPlayer { source }
        | TriggerEventDef::CombatDamageDealtToSource { source }
        | TriggerEventDef::DamageDealtToPlayer { source, .. } => shared_object_predicate(source),
        TriggerEventDef::AbilityActivated(_)
        | TriggerEventDef::ManaAdded(_)
        | TriggerEventDef::Special(_) => false,
    }
}

pub(super) fn shared_activated_costs(source_zones: &[ZoneKind], costs: &[AbilityCostDef]) -> bool {
    let battlefield = source_zones == [ZoneKind::Battlefield];
    let hand = source_zones == [ZoneKind::Hand];
    let sacrifice_choices = costs
        .iter()
        .filter(|cost| matches!(cost, AbilityCostDef::SacrificePermanent { .. }))
        .count();
    sacrifice_choices <= 1
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
            | AbilityCostDef::RemoveCountersFromSource { .. }
            | AbilityCostDef::PayLife(_)
            | AbilityCostDef::Loyalty(_) => battlefield,
            AbilityCostDef::DiscardSource => hand,
            AbilityCostDef::UntapSource
            | AbilityCostDef::DiscardCards(_)
            | AbilityCostDef::ExileSource
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
                | EffectRecipientDef::Target(_)
                | EffectRecipientDef::ControllerOfTarget(_)
                | EffectRecipientDef::ObjectsControlledByTarget { .. }
                | EffectRecipientDef::ObjectsOwnedByTarget { .. }
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
        // None of these is a static ability; all execute from the stack.
        EffectDef::GrantFlashToNextSorcery
        | EffectDef::May(_)
        | EffectDef::ExileLinkedToSource { .. }
        | EffectDef::ReturnLinkedExiles { .. }
        | EffectDef::MakeUnblockableThisTurn { .. }
        | EffectDef::GainControlThisTurn { .. }
        | EffectDef::AtNextStep { .. }
        | EffectDef::IfCondition { .. }
        | EffectDef::TriggerUntilYourNextTurn { .. }
        | EffectDef::None
        | EffectDef::AddMana(_)
        | EffectDef::AddManaEqualTo { .. }
        | EffectDef::DealDamage { .. }
        | EffectDef::DrainLife { .. }
        | EffectDef::GainLife { .. }
        | EffectDef::DrawCards { .. }
        | EffectDef::Discard { .. }
        | EffectDef::LoseLife { .. }
        | EffectDef::LoseTheGame { .. }
        | EffectDef::Tap { .. }
        | EffectDef::Untap { .. }
        | EffectDef::PreventCombatDamageThisTurn { .. }
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
        | EffectDef::LookAtHand { .. }
        | EffectDef::SearchLibrary { .. }
        | EffectDef::Counter { .. }
        | EffectDef::CounterUnlessPaid { .. }
        | EffectDef::AddCounters { .. }
        | EffectDef::ChangeTextBasicLandType { .. }
        | EffectDef::BecomeCopyOf { .. }
        | EffectDef::OptionalManaPayment { .. }
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
            recipient == EffectRecipientDef::Source && shared_object_predicate(predicate)
        }
        AppliedEffectDef::RemoveAbilities(_)
        | AppliedEffectDef::CannotBeCountered
        | AppliedEffectDef::CannotBeEnchanted => true,
        // Only a resolving animation is supported; nothing reads one off
        // a static ability.
        AppliedEffectDef::Animate(_) | AppliedEffectDef::Special(_) => false,
    }
}

pub(super) fn shared_trigger_condition(condition: TriggerConditionDef) -> bool {
    match condition {
        TriggerConditionDef::ObjectCount { query, .. } => shared_object_predicate(query.object),
        TriggerConditionDef::TargetMatches { object, .. } => shared_object_predicate(object),
        TriggerConditionDef::ActivePlayer(_)
        | TriggerConditionDef::SourceLoyalty { .. }
        | TriggerConditionDef::SourceActivationsThisTurn { .. }
        | TriggerConditionDef::SourceDealtDamageToOpponentThisTurn
        | TriggerConditionDef::SpellsCastLastTurn { .. } => true,
    }
}

pub(super) fn shared_replacement_effect(effect: ReplacementEffectDef) -> bool {
    match effect {
        ReplacementEffectDef::None | ReplacementEffectDef::ModifyBattlefieldEntry(_) => true,
        ReplacementEffectDef::Sequence(effects) => {
            !effects.is_empty() && effects.iter().copied().all(shared_replacement_effect)
        }
        ReplacementEffectDef::Conditional {
            condition,
            if_true,
            if_false,
        } => {
            let condition_is_supported = match condition {
                ConditionDef::Exists(query) => {
                    query.zones == [ZoneKind::Battlefield] && shared_object_predicate(query.object)
                }
            };
            condition_is_supported
                && if_true.iter().copied().all(shared_replacement_effect)
                && if_false.iter().copied().all(shared_replacement_effect)
        }
        ReplacementEffectDef::OptionalPayment {
            payment,
            if_paid,
            if_declined,
        } => {
            let payable_life = payment.costs.iter().try_fold(0_u32, |total, cost| {
                let AbilityCostDef::PayLife(amount) = cost else {
                    return None;
                };
                total.checked_add(u32::from(*amount))
            });
            payment.payer != PlayerRelation::Any
                && !payment.costs.is_empty()
                && payable_life.is_some_and(|amount| amount > 0 && i16::try_from(amount).is_ok())
                && if_paid.iter().copied().all(shared_replacement_effect)
                && if_declined.iter().copied().all(shared_replacement_effect)
        }
    }
}

pub(super) fn shared_replacement_event(event: ReplacementEventDef) -> bool {
    match event {
        ReplacementEventDef::SourceEntersBattlefield
        | ReplacementEventDef::WouldGainLife(_)
        | ReplacementEventDef::EntersBattlefield => true,
        ReplacementEventDef::ObjectEntersBattlefield { object, .. } => {
            shared_object_predicate(object)
        }
        ReplacementEventDef::WouldMove { cause, .. } => shared_zone_move_cause(cause),
        // Only graveyard placement funnels through one procedure the
        // replacement can sit in front of.
        ReplacementEventDef::AnyObjectWouldMove { to } => to == ZoneKind::Graveyard,
        ReplacementEventDef::Special(_) => false,
    }
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
                            | AbilityCostDef::RemoveCountersFromSource { .. }
                            | AbilityCostDef::PayLife(_)
                    )
                })
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
                    | EffectDef::May(_)
                    | EffectDef::None
                    | EffectDef::DealDamage { .. }
                    | EffectDef::DrainLife { .. }
                    | EffectDef::GainLife { .. }
                    | EffectDef::DrawCards { .. }
                    | EffectDef::Discard { .. }
                    | EffectDef::LoseLife { .. }
                    | EffectDef::LoseTheGame { .. }
                    | EffectDef::Tap { .. }
                    | EffectDef::Untap { .. }
                    | EffectDef::PreventCombatDamageThisTurn { .. }
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
                    | EffectDef::LookAtHand { .. }
                    | EffectDef::SearchLibrary { .. }
                    | EffectDef::Counter { .. }
                    | EffectDef::CounterUnlessPaid { .. }
                    | EffectDef::AddCounters { .. }
                    | EffectDef::ChangeTextBasicLandType { .. }
                    | EffectDef::BecomeCopyOf { .. }
                    | EffectDef::OptionalManaPayment { .. }
                    | EffectDef::UnlessPaid { .. }
                    | EffectDef::CannotBeForcedToSacrifice
                    | EffectDef::CreateEmblem { .. }
                    | EffectDef::Transform { .. }
                    | EffectDef::AdditionalCombatPhase
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
                battlefield_only(definition.source_zones)
                    && shared_replacement_event(definition.event)
                    && matches!(effect, EffectDef::Replacement(effect) if shared_replacement_effect(effect))
            }
            ReplacementEventDef::EntersBattlefield => {
                battlefield_only(definition.source_zones)
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
                definition.source_zones == [from]
                    && from == ZoneKind::Hand
                    && to == ZoneKind::Graveyard
                    && shared_zone_move_cause(cause)
                    && effect
                        == EffectDef::MoveToZone {
                            object: EffectRecipientDef::Source,
                            zone: ZoneKind::Battlefield,
                            controller: None,
                            placement: ZonePlacement::Top,
                        }
            }
            ReplacementEventDef::AnyObjectWouldMove { .. } => {
                battlefield_only(definition.source_zones)
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
                battlefield_only(definition.source_zones)
                    && matches!(effect, EffectDef::MultiplyEventAmount(_))
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
