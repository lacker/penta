use super::*;
use crate::card::CopyAbilityDef;

fn trigger_predicate_requires_live_battlefield(predicate: ObjectPredicateDef) -> bool {
    match predicate {
        ObjectPredicateDef::All(predicates) | ObjectPredicateDef::AnyOf(predicates) => predicates
            .iter()
            .copied()
            .any(trigger_predicate_requires_live_battlefield),
        ObjectPredicateDef::Not(predicate) => {
            trigger_predicate_requires_live_battlefield(*predicate)
        }
        ObjectPredicateDef::HasNonManaActivatedAbility => true,
        _ => false,
    }
}

#[allow(clippy::too_many_lines)]
pub(in super::super) fn shared_trigger_event(event: TriggerEventDef) -> bool {
    match event {
        // One ability, so it is only runnable if every way into it is.
        TriggerEventDef::AnyOf(events) => events.iter().copied().all(shared_trigger_event),
        // Both halves have to be runnable: the event it wraps, and the
        // condition the runtime asks where that event is matched.
        TriggerEventDef::While { event, condition } => {
            shared_trigger_event(*event) && shared_trigger_condition(*condition)
        }
        TriggerEventDef::ZoneChanged(matcher) => {
            const COMMITTED_TRANSITIONS: [(ZoneKind, ZoneKind); 12] = [
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
            let can_match_departure =
                COMMITTED_TRANSITIONS
                    .iter()
                    .any(|(actual_from, actual_to)| {
                        *actual_from == ZoneKind::Battlefield
                            && *actual_to != ZoneKind::Battlefield
                            && matcher.from.is_none_or(|expected| expected == *actual_from)
                            && matcher.to.is_none_or(|expected| expected == *actual_to)
                    });
            shared_object_predicate(matcher.object)
                && (!can_match_departure
                    || !trigger_predicate_requires_live_battlefield(matcher.object))
                && COMMITTED_TRANSITIONS
                    .iter()
                    .any(|(actual_from, actual_to)| {
                        matcher.from.is_none_or(|expected| expected == *actual_from)
                            && matcher.to.is_none_or(|expected| expected == *actual_to)
                    })
                && matcher.previously_damaged_by.is_none_or(|reference| {
                    matcher
                        .from
                        .is_none_or(|from| from == ZoneKind::Battlefield)
                        && matcher.to.is_none_or(|to| to == ZoneKind::Graveyard)
                        && matches!(
                            reference,
                            ObjectRefDef::Source
                                | ObjectRefDef::AttachedToSource
                                | ObjectRefDef::TriggeringObject
                        )
                })
        }
        TriggerEventDef::Tapped(matcher) => shared_object_predicate(matcher.object),
        // The batch is read where the tokens arrived, so the predicate is
        // asked of live battlefield permanents.
        TriggerEventDef::TokensCreated { token, .. } => shared_object_predicate(token),
        // The zones are printed constants and the owner a relation, so
        // there is nothing here that could read the board.
        TriggerEventDef::CardsExiled { zones, .. } => !zones.is_empty(),
        TriggerEventDef::AttackDeclared {
            attacker,
            declaration,
        } => {
            shared_object_predicate(attacker)
                && declaration.minimum > 0
                && declaration
                    .maximum
                    .is_none_or(|maximum| declaration.minimum <= maximum)
        }
        TriggerEventDef::Exerted(attacker) => shared_object_predicate(attacker),
        // Published from every site that sacrifices, before the permanent
        // leaves, so what it was is still readable.
        TriggerEventDef::Sacrificed { object, .. } => shared_object_predicate(object),
        TriggerEventDef::Attacks(matcher) => {
            shared_object_predicate(matcher.attacker)
                && matcher.declaration.minimum > 0
                && matcher
                    .declaration
                    .maximum
                    .is_none_or(|maximum| matcher.declaration.minimum <= maximum)
                && matcher.attack_number.is_none_or(|number| number > 0)
        }
        TriggerEventDef::BecomesBlocked(object)
        | TriggerEventDef::CountersPlaced { object, .. }
        | TriggerEventDef::CountersRemoved { object, .. }
        | TriggerEventDef::LastCounterRemoved { object, .. }
        | TriggerEventDef::Transforms(object) => {
            shared_object_predicate(object)
        }
        // Both read the spell rather than the battlefield, and a spell on
        // the stack is not a permanent the predicate can interrogate.
        TriggerEventDef::SpellCast { object, .. }
        | TriggerEventDef::SpellCopied(object)
        | TriggerEventDef::BecomesTargetOfSpell(object)
        | TriggerEventDef::BecomesTargetOfSpellOrAbility(object)
        | TriggerEventDef::YouOrYourPermanentBecomesTarget(object) => {
            shared_object_predicate(object) && !trigger_predicate_requires_live_battlefield(object)
        }
        // A crime names only the player who committed it; what was targeted
        // is not part of the event. Cycling names no object of its own: the
        // card that was cycled is the only thing that can be listening.
        TriggerEventDef::CommittedCrime(_)
        | TriggerEventDef::BecomesLevel(_)
        | TriggerEventDef::Cycled
        | TriggerEventDef::DoorUnlocked
        // The land that was played is on the battlefield by the time the
        // trigger is captured, so an ordinary predicate may read it.
        | TriggerEventDef::LandPlayed { .. }
        | TriggerEventDef::StepBegins { .. }
        | TriggerEventDef::LifeGained(_)
        | TriggerEventDef::Discarded(_)
        | TriggerEventDef::DiscardedCards(_)
        // The crown names only the player who received it, and there is
        // nothing else for a predicate to read.
        | TriggerEventDef::BecomesMonarch(_)
        | TriggerEventDef::DrewCard(_)
        | TriggerEventDef::StateCondition => true,
        TriggerEventDef::DamageDealt(matcher) => {
            let source = match matcher.source {
                DamageSourceMatcherDef::Matching(object) => {
                    shared_object_predicate(object)
                        && !trigger_predicate_requires_live_battlefield(object)
                }
                DamageSourceMatcherDef::Any | DamageSourceMatcherDef::Group(_) => true,
                DamageSourceMatcherDef::AffectedObject => false,
                DamageSourceMatcherDef::Object(reference)
                | DamageSourceMatcherDef::Except(reference) => matches!(
                    reference,
                    ObjectRefDef::Source
                        | ObjectRefDef::AttachedToSource
                        | ObjectRefDef::TriggeringObject
                ),
            };
            let recipient = match matcher.recipient {
                DamageRecipientMatcherDef::Recipients(EffectRecipientDef(
                    EffectRecipientSetDef::Objects(ObjectSetDef::One(reference)),
                )) => matches!(
                    reference,
                    ObjectRefDef::Source
                        | ObjectRefDef::AttachedToSource
                        | ObjectRefDef::TriggeringObject
                ),
                DamageRecipientMatcherDef::MatchingObject(predicate) => {
                    shared_object_predicate(predicate)
                }
                DamageRecipientMatcherDef::Any
                | DamageRecipientMatcherDef::PlayerOrPlaneswalker
                | DamageRecipientMatcherDef::Recipients(EffectRecipientDef(
                    EffectRecipientSetDef::Players(_),
                ))
                | DamageRecipientMatcherDef::PlayerAndCreaturesControlledBy(
                    PlayerRefDef::EffectController
                    | PlayerRefDef::EnchantedPlayer
                    | PlayerRefDef::EventPlayer
                    | PlayerRefDef::Opponent,
                ) => true,
                DamageRecipientMatcherDef::PlayerAndCreaturesControlledBy(
                    PlayerRefDef::ControllerOf(reference)
                    | PlayerRefDef::OpponentOf(reference)
                    | PlayerRefDef::OwnerOf(reference),
                ) => matches!(
                    reference,
                    ObjectRefDef::Source
                        | ObjectRefDef::AttachedToSource
                        | ObjectRefDef::TriggeringObject
                ),
                DamageRecipientMatcherDef::AffectedObject
                | DamageRecipientMatcherDef::Recipients(_)
                | DamageRecipientMatcherDef::PlayerAndCreaturesControlledBy(
                    PlayerRefDef::Target(_),
                ) => false,
            };
            source && recipient
        }
        TriggerEventDef::ObjectsDied { object: source }
        | TriggerEventDef::AttacksAndIsNotBlocked { attacker: source }
        | TriggerEventDef::UnblockedAttackersDeclared {
            attacker: source, ..
        }
        | TriggerEventDef::CombatDamageDealtToPlayers {
            sources: source, ..
        }
        | TriggerEventDef::Blocks { blocked: source }
        | TriggerEventDef::BecomesBlockedBy { blocker: source } => shared_object_predicate(source),
        TriggerEventDef::BlocksOrBecomesBlockedBy { creature, other } => {
            shared_object_predicate(creature) && shared_object_predicate(other)
        }
    }
}

/// The conditions an entry replacement may read, unwrapped through any
/// conjunction. Each leaf still has to be a battlefield query the shared
/// runtime can answer.
fn shared_entry_replacement_condition(condition: ConditionDef) -> bool {
    match condition {
        ConditionDef::Exists(query) => {
            query.zones == [ZoneKind::Battlefield] && shared_object_predicate(query.object)
        }
        ConditionDef::ObjectCount(counting) => {
            counting.query.zones == [ZoneKind::Battlefield]
                && shared_object_predicate(counting.query.object)
        }
        ConditionDef::All(conditions) => conditions
            .iter()
            .copied()
            .all(shared_entry_replacement_condition),
        // A turn count is read off the game rather than out of a zone, so
        // there is nothing about it for the entry walk to be unable to see.
        ConditionDef::ControllerTurnsTakenAtMost(_) => true,
    }
}

pub(super) fn shared_entry_replacement_effect(effect: ReplacementEffectDef) -> bool {
    match effect {
        ReplacementEffectDef::ModifyBattlefieldEntry(_)
        | ReplacementEffectDef::Choose(_)
        | ReplacementEffectDef::LookAtHand(_)
        // The entering card goes somewhere else instead. Only the zones a
        // card can actually be sent to are inside the boundary; a redirect
        // back onto the battlefield would be the entry it replaced.
        | ReplacementEffectDef::MoveToZone(
            ZoneKind::Graveyard | ZoneKind::Exile | ZoneKind::Hand | ZoneKind::Library,
        ) => true,
        ReplacementEffectDef::CopyEntering { exceptions, .. } => exceptions
            .added_abilities
            .iter()
            .all(|addition| matches!(addition, CopyAbilityDef::Ability(_))),
        ReplacementEffectDef::Sequence(effects) => {
            !effects.is_empty() && effects.iter().copied().all(shared_entry_replacement_effect)
        }
        ReplacementEffectDef::Conditional {
            condition,
            if_true,
            if_false,
        } => {
            let condition_is_supported = shared_entry_replacement_condition(condition);
            condition_is_supported
                && if_true.iter().copied().all(shared_entry_replacement_effect)
                && if_false
                    .iter()
                    .copied()
                    .all(shared_entry_replacement_effect)
        }
        ReplacementEffectDef::PayOr {
            payment,
            if_paid,
            if_declined,
        } => {
            !matches!(
                payment.payer,
                PlayerSetDef::All | PlayerSetDef::Related(PlayerRelation::Any)
            ) && if_paid.iter().copied().all(shared_entry_replacement_effect)
                && if_declined
                    .iter()
                    .copied()
                    .all(shared_entry_replacement_effect)
        }
        ReplacementEffectDef::ReplaceEventWithNothing
        | ReplacementEffectDef::MoveToZone(_)
        | ReplacementEffectDef::Perform(_)
        | ReplacementEffectDef::PlaceCountersOnMovedObject { .. }
        | ReplacementEffectDef::MultiplyEventAmount(_)
        // A draw's clause rather than an entry's.
        | ReplacementEffectDef::AddToEventAmount(_) => false,
    }
}

pub(in super::super) fn shared_begin_turn_replacement_effect(effect: ReplacementEffectDef) -> bool {
    match effect {
        ReplacementEffectDef::ReplaceEventWithNothing => true,
        ReplacementEffectDef::Perform(effect) => matches!(
            *effect,
            EffectDef::Untap {
                object: EffectRecipientDef::Source,
            }
        ),
        ReplacementEffectDef::Sequence(effects) => {
            !effects.is_empty()
                && effects
                    .iter()
                    .copied()
                    .all(shared_begin_turn_replacement_effect)
                && effects
                    .iter()
                    .any(|effect| matches!(effect, ReplacementEffectDef::ReplaceEventWithNothing))
        }
        ReplacementEffectDef::MoveToZone(_)
        | ReplacementEffectDef::ModifyBattlefieldEntry(_)
        | ReplacementEffectDef::PlaceCountersOnMovedObject { .. }
        | ReplacementEffectDef::MultiplyEventAmount(_)
        | ReplacementEffectDef::AddToEventAmount(_)
        | ReplacementEffectDef::Choose(_)
        | ReplacementEffectDef::LookAtHand(_)
        | ReplacementEffectDef::CopyEntering { .. }
        | ReplacementEffectDef::Conditional { .. }
        | ReplacementEffectDef::PayOr { .. } => false,
    }
}

pub(in super::super) fn shared_draw_replacement_effect(effect: ReplacementEffectDef) -> bool {
    // "You draw that many cards plus one instead": the instruction is
    // replaced by a larger one, counted where the instruction is rather
    // than once per card.
    if matches!(effect, ReplacementEffectDef::AddToEventAmount(_)) {
        return true;
    }
    let ReplacementEffectDef::Sequence(effects) = effect else {
        return false;
    };
    effects.len() == 2
        && effects
            .iter()
            .filter(|effect| **effect == ReplacementEffectDef::ReplaceEventWithNothing)
            .count()
            == 1
        && effects
            .iter()
            .filter_map(|effect| match effect {
                ReplacementEffectDef::Perform(effect) => Some(**effect),
                _ => None,
            })
            .all(shared_stack_effect)
        && effects
            .iter()
            .filter(|effect| matches!(effect, ReplacementEffectDef::Perform(_)))
            .count()
            == 1
}

/// The draw engine evaluates an amount-changing replacement once per
/// instruction and a replacement-with-effect once per card. Keep conditions
/// paired with the level where the runtime can answer them.
pub(in super::super) fn shared_draw_replacement_program(
    condition: Option<ReplacementConditionDef>,
    effect: ReplacementEffectDef,
) -> bool {
    let condition_supported = if matches!(effect, ReplacementEffectDef::AddToEventAmount(_)) {
        condition.is_none_or(|condition| {
            matches!(condition, ReplacementConditionDef::ControllerHandAtMost(_))
        })
    } else {
        condition
            .is_none_or(|condition| condition == ReplacementConditionDef::ControllerLibraryEmpty)
    };
    condition_supported && shared_draw_replacement_effect(effect)
}

pub(in super::super) fn shared_battlefield_exit_replacement_effect(
    effect: ReplacementEffectDef,
) -> bool {
    match effect {
        // Exile and library are the two destinations that answer "instead":
        // one takes the card out of the game, the other puts it back.
        ReplacementEffectDef::MoveToZone(zone) => {
            matches!(zone, ZoneKind::Exile | ZoneKind::Library)
        }
        // A mark the runtime carries with the card into its new zone. The
        // Sequence rule below is what requires a destination beside it.
        ReplacementEffectDef::PlaceCountersOnMovedObject { .. } => true,
        ReplacementEffectDef::Perform(effect) => matches!(
            *effect,
            EffectDef::TakeExtraTurn {
                player: EffectRecipientDef::Controller,
            } | EffectDef::ShuffleLibrary {
                player: EffectRecipientDef::Controller,
            }
        ),
        ReplacementEffectDef::Sequence(effects) => {
            !effects.is_empty()
                && effects
                    .iter()
                    .copied()
                    .all(shared_battlefield_exit_replacement_effect)
                && effects
                    .iter()
                    .any(|effect| matches!(effect, ReplacementEffectDef::MoveToZone(_)))
        }
        ReplacementEffectDef::ReplaceEventWithNothing
        | ReplacementEffectDef::ModifyBattlefieldEntry(_)
        | ReplacementEffectDef::MultiplyEventAmount(_)
        | ReplacementEffectDef::AddToEventAmount(_)
        | ReplacementEffectDef::Choose(_)
        | ReplacementEffectDef::LookAtHand(_)
        | ReplacementEffectDef::CopyEntering { .. }
        | ReplacementEffectDef::Conditional { .. }
        | ReplacementEffectDef::PayOr { .. } => false,
    }
}

pub(super) fn shared_replacement_event(event: ReplacementEventDef) -> bool {
    match event {
        ReplacementEventDef::SourceEntersBattlefield
        | ReplacementEventDef::WouldGainLife(_)
        | ReplacementEventDef::WouldDraw { .. }
        | ReplacementEventDef::WouldBeginTurn { .. } => true,
        ReplacementEventDef::ObjectEntersBattlefield { object, .. } => {
            shared_object_predicate(object)
        }
        ReplacementEventDef::WouldMove { cause, .. } => shared_zone_move_cause(cause),
        // Only graveyard placement funnels through one procedure the
        // replacement can sit in front of.
        ReplacementEventDef::AnyObjectWouldMove { to, .. } => to == ZoneKind::Graveyard,
        ReplacementEventDef::Special(_) => false,
    }
}

fn assert_nested_installed_ability(card_name: &str, ability: &AbilityDef) {
    assert!(
        shared_definition_ability(ability),
        "{card_name} installs a triggered ability outside the shared runtime boundary: {ability:?}",
    );
    assert_nested_program_abilities(card_name, ability.effect.definition);
}

pub(in super::super) fn assert_nested_program_abilities(
    card_name: &str,
    program: AbilityProgramDef,
) {
    match program {
        AbilityProgramDef::Effects(effect) => assert_nested_definition_abilities(card_name, effect),
        AbilityProgramDef::Replacement(effect) => {
            assert_nested_replacement_definition_abilities(card_name, effect);
        }
    }
}

pub(in super::super) fn assert_nested_definition_abilities(card_name: &str, effect: EffectDef) {
    match effect {
        EffectDef::InstallTrigger(trigger) => {
            assert_nested_installed_ability(card_name, trigger.ability);
        }
        EffectDef::StaticApply { effect, .. }
        | EffectDef::Apply { effect, .. }
        | EffectDef::DealDamageAndApply {
            applied: effect, ..
        } => {
            assert_nested_definition_applied_effect(card_name, effect);
        }
        EffectDef::BecomeCopyOf { exceptions, .. } => {
            for addition in exceptions.added_abilities {
                if let CopyAbilityDef::Ability(ability) = addition {
                    assert_nested_installed_ability(card_name, ability);
                }
            }
        }
        EffectDef::CreateToken {
            copy: Some(copy), ..
        } => {
            for addition in copy.exceptions.added_abilities {
                if let CopyAbilityDef::Ability(ability) = addition {
                    assert_nested_installed_ability(card_name, ability);
                }
            }
        }
        _ => {}
    }
    for child in crate::card::child_effects(effect) {
        assert_nested_definition_abilities(card_name, child);
    }
}

pub(in super::super) fn assert_nested_replacement_definition_abilities(
    card_name: &str,
    effect: ReplacementEffectDef,
) {
    match effect {
        ReplacementEffectDef::Sequence(effects) => {
            for effect in effects {
                assert_nested_replacement_definition_abilities(card_name, *effect);
            }
        }
        ReplacementEffectDef::Perform(effect) => {
            assert_nested_definition_abilities(card_name, *effect);
        }
        ReplacementEffectDef::Conditional {
            if_true, if_false, ..
        } => {
            for effect in if_true.iter().chain(if_false.iter()) {
                assert_nested_replacement_definition_abilities(card_name, *effect);
            }
        }
        ReplacementEffectDef::PayOr {
            if_paid,
            if_declined,
            ..
        } => {
            for effect in if_paid.iter().chain(if_declined.iter()) {
                assert_nested_replacement_definition_abilities(card_name, *effect);
            }
        }
        ReplacementEffectDef::ReplaceEventWithNothing
        | ReplacementEffectDef::MoveToZone(_)
        | ReplacementEffectDef::ModifyBattlefieldEntry(_)
        | ReplacementEffectDef::PlaceCountersOnMovedObject { .. }
        | ReplacementEffectDef::MultiplyEventAmount(_)
        | ReplacementEffectDef::AddToEventAmount(_)
        | ReplacementEffectDef::Choose(_)
        | ReplacementEffectDef::LookAtHand(_) => {}
        ReplacementEffectDef::CopyEntering { exceptions, .. } => {
            for addition in exceptions.added_abilities {
                if let CopyAbilityDef::Ability(ability) = addition {
                    assert_nested_installed_ability(card_name, ability);
                }
            }
        }
    }
}

pub(in super::super) fn assert_nested_definition_applied_effect(
    card_name: &str,
    effect: AppliedEffectDef,
) {
    match effect {
        AppliedEffectDef::Composite(effects) => {
            for effect in effects {
                assert_nested_definition_applied_effect(card_name, *effect);
            }
        }
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
            AbilityOperationDef::Add(ability),
        )) => {
            if ability.declarative_effect().is_some() {
                assert!(
                    shared_definition_ability(ability),
                    "{card_name} contains a nested shared declarative ability outside the shared runtime boundary: {ability:?}",
                );
            }
            assert_nested_program_abilities(card_name, ability.effect.definition);
        }
        AppliedEffectDef::Rule(_) | AppliedEffectDef::Characteristic(_) => {}
    }
}
