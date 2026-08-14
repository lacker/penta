//! Which static abilities the shared runtime can execute.
//!
//! A static effect is read live off the battlefield rather than resolved, so
//! the questions here are different from the stack's: what may be applied
//! continuously, to whom, and for how long.

use super::*;

/// The remaining static effects that are not an `Apply`.
pub(in super::super) fn shared_static_non_apply_effect(
    source_zones: &[ZoneKind],
    effect: EffectDef,
) -> bool {
    match effect {
        // Both are read off the battlefield and neither carries anything
        // further to check: one names a land type, the other nothing at all.
        EffectDef::CannotBeForcedToSacrifice | EffectDef::LandwalkCanBeBlocked(_) => {
            battlefield_only(source_zones)
        }
        // Read while attackers are declared, over the battlefield, so only
        // the object predicate is left to check.
        EffectDef::CannotAttackUnless(query) => {
            battlefield_only(source_zones)
                && query.zones == [ZoneKind::Battlefield]
                && shared_object_predicate(query.object)
                && shared_static_query(*query)
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
pub(in super::super) fn shared_static_effect(source_zones: &[ZoneKind], effect: EffectDef) -> bool {
    match effect {
        EffectDef::CannotBeForcedToSacrifice
        | EffectDef::ReduceGenericCostBy(_)
        | EffectDef::LandwalkCanBeBlocked(_)
        | EffectDef::CannotAttackUnless(_)
        | EffectDef::Sequence(_) => shared_static_non_apply_effect(source_zones, effect),
        EffectDef::StaticApply { recipient, effect } => {
            let battlefield_recipient_is_supported = match recipient.0 {
                EffectRecipientSetDef::Objects(ObjectSetDef::One(
                    ObjectRefDef::Source | ObjectRefDef::AttachedToSource,
                ))
                | EffectRecipientSetDef::Players(
                    PlayerSetDef::All
                    | PlayerSetDef::One(PlayerRefDef::EffectController)
                    | PlayerSetDef::Related(_),
                ) => true,
                EffectRecipientSetDef::Objects(ObjectSetDef::Query(query)) => {
                    query.zones == [ZoneKind::Battlefield]
                        && shared_object_predicate(query.object)
                        && shared_static_query(query)
                }
                EffectRecipientSetDef::LegalTargets(_)
                | EffectRecipientSetDef::Objects(
                    ObjectSetDef::One(
                        ObjectRefDef::ResolvingObject
                        | ObjectRefDef::Binding(_)
                        | ObjectRefDef::Target(_)
                        | ObjectRefDef::TriggeringObject,
                    )
                    | ObjectSetDef::Binding(_)
                    | ObjectSetDef::LegalTargets(_)
                    | ObjectSetDef::SharingNameWith(_),
                )
                | EffectRecipientSetDef::Players(
                    PlayerSetDef::LegalTargets(_)
                    | PlayerSetDef::One(
                        PlayerRefDef::EventPlayer
                        | PlayerRefDef::Target(_)
                        | PlayerRefDef::ControllerOf(_)
                        | PlayerRefDef::OwnerOf(_),
                    ),
                ) => false,
            };
            let battlefield_effect_is_supported = shared_static_applied_effect(recipient, effect);
            let battlefield_effect = battlefield_only(source_zones)
                && battlefield_recipient_is_supported
                && battlefield_effect_is_supported;
            let stack_source_effect = source_zones == [ZoneKind::Stack]
                && recipient == EffectRecipientDef::Source
                && shared_cannot_be_countered_effect(effect);
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
        | EffectDef::Choose(_)
        | EffectDef::PayOr(_)
        | EffectDef::SplitIntoPiles(_)
        | EffectDef::PreventDamage { .. }
        | EffectDef::Apply { .. }
        | EffectDef::May { .. }
        | EffectDef::ExileLinkedToSource { .. }
        | EffectDef::ReturnLinkedExiles { .. }
        | EffectDef::Detain { .. }
        | EffectDef::GainControl { .. }
        | EffectDef::InstallTrigger(_)
        | EffectDef::None
        | EffectDef::AddMana(_)
        | EffectDef::AddManaEqualTo { .. }
        | EffectDef::DealDamage { .. }
        | EffectDef::DrainLife { .. }
        | EffectDef::GainLife { .. }
        | EffectDef::AddPoisonCounters { .. }
        | EffectDef::DrawCards { .. }
        | EffectDef::Discard { .. }
        | EffectDef::DiscardCards { .. }
        | EffectDef::ShuffleLibrary { .. }
        | EffectDef::EmptyManaPool { .. }
        | EffectDef::LoseLife { .. }
        | EffectDef::LoseTheGame { .. }
        | EffectDef::Regenerate { .. }
        | EffectDef::Tap { .. }
        | EffectDef::RemoveFromCombat { .. }
        | EffectDef::DestroyAtEndOfCombat { .. }
        | EffectDef::SkipNextUntapSteps { .. }
        | EffectDef::RemoveAllCounters { .. }
        | EffectDef::Untap { .. }
        | EffectDef::Attach { .. }
        | EffectDef::Reconfigure { .. }
        | EffectDef::CreateToken { .. }
        | EffectDef::CreateAttachedToken { .. }
        | EffectDef::CreateTokenCopyOf { .. }
        | EffectDef::Destroy { .. }
        | EffectDef::Sacrifice { .. }
        | EffectDef::SacrificeOfChoice { .. }
        | EffectDef::Mill { .. }
        | EffectDef::LookAtTopAndSelect { .. }
        | EffectDef::LookAtHand { .. }
        | EffectDef::SearchZone { .. }
        | EffectDef::ChooseCards { .. }
        | EffectDef::ReplaceNextDrawThisTurn { .. }
        | EffectDef::IfFormat { .. }
        | EffectDef::Counter { .. }
        | EffectDef::AddCounters { .. }
        | EffectDef::ChangeTextBasicLandType { .. }
        | EffectDef::BecomeCopyOf { .. }
        | EffectDef::MoveToZone { .. }
        | EffectDef::CreateEmblem { .. }
        | EffectDef::Transform { .. }
        | EffectDef::ScheduleTurnPhases(_)
        | EffectDef::TakeExtraTurn { .. }
        | EffectDef::Special(_) => false,
    }
}

fn shared_static_animation_query(recipient: EffectRecipientDef) -> bool {
    recipient.object_query().is_some_and(|query| {
        query.zones == [ZoneKind::Battlefield]
            && shared_static_query(query)
            && Game::static_animation_predicate_is_supported(query.object)
    })
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
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::BasicLandTypes(
            SetOperationDef::Add(land_types)
            | SetOperationDef::Remove(land_types)
            | SetOperationDef::Set(land_types),
        )) => !land_types.is_empty(),
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
            AbilityOperationDef::Add(ability),
        )) => shared_definition_ability(ability),
        // Static animation is deliberately narrower than resolving
        // characteristic changes: it may add the creature card type, may
        // repaint color, and must use a query that cannot read anything those
        // operations supply. Static subtype changes remain outside this
        // stratified walk.
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::CardTypes(
            SetOperationDef::Add(types),
        )) => types.contains(CardType::Creature) && shared_static_animation_query(recipient),
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::Colors(
            SetOperationDef::Set(_),
        )) => shared_static_animation_query(recipient),
        AppliedEffectDef::Characteristic(
            CharacteristicOperationDef::CardTypes(
                SetOperationDef::Remove(_) | SetOperationDef::Set(_),
            )
            | CharacteristicOperationDef::Colors(
                SetOperationDef::Add(_) | SetOperationDef::Remove(_),
            )
            | CharacteristicOperationDef::CreatureTypes(_),
        )
        | AppliedEffectDef::Rule(AppliedRuleDef::RedirectDamageFromTo { .. }) => false,
        // A blocking restriction is read off the ordinary static-effect walk
        // over the attacker, so a group recipient works exactly as a
        // self-applied one does: Bower Passage names every creature you
        // control rather than only itself. The other restriction keeps the
        // narrower list because no card applies it to a group.
        AppliedEffectDef::Rule(AppliedRuleDef::CannotBeBlockedBy(predicate)) => {
            (matches!(
                recipient.object_reference(),
                Some(ObjectRefDef::Source | ObjectRefDef::AttachedToSource)
            ) || recipient.object_query().is_some())
                && shared_object_predicate(predicate)
        }
        AppliedEffectDef::Rule(AppliedRuleDef::CanBlockOnly(predicate)) => {
            matches!(
                recipient.object_reference(),
                Some(ObjectRefDef::Source | ObjectRefDef::AttachedToSource)
            ) && shared_object_predicate(predicate)
        }
        // The redirection names a group rather than a predicate, and it can
        // only be aimed at a permanent whose controller it protects.
        AppliedEffectDef::Rule(AppliedRuleDef::RedirectPlayerDamageToThis(_)) => {
            matches!(
                recipient.object_reference(),
                Some(ObjectRefDef::Source | ObjectRefDef::AttachedToSource)
            ) || recipient.object_query().is_some()
        }
        AppliedEffectDef::Rule(AppliedRuleDef::PreventDamage(matcher)) => {
            let recipient_is_supported = matches!(
                recipient.object_reference(),
                Some(ObjectRefDef::Source | ObjectRefDef::AttachedToSource)
            );
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
        AppliedEffectDef::Rule(AppliedRuleDef::RemainsAttachedThroughProtection) => {
            recipient == EffectRecipientDef::Source
        }
        AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(restriction)) => {
            matches!(recipient.0, EffectRecipientSetDef::Players(_))
                && shared_object_predicate(restriction.object)
        }
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
            AbilityOperationDef::Remove(_),
        ))
        | AppliedEffectDef::Rule(_) => true,
    }
}

/// The values a static power/toughness bonus may be built from. They are the
/// ones the layer walk can evaluate without reading a resolving spell, and a
/// scale is allowed only over another such value.
fn static_stat_value(value: crate::card::ValueDef) -> bool {
    match value {
        crate::card::ValueDef::Constant(_)
        | crate::card::ValueDef::AnyMatchingObject(_)
        | crate::card::ValueDef::CountMatchingObjects(_) => true,
        crate::card::ValueDef::Scaled(scaled) => static_stat_value(scaled.value),
        _ => false,
    }
}
