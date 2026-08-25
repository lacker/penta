//! Which static abilities the shared runtime can execute.
//!
//! A static effect is read live off the battlefield rather than resolved, so
//! the questions here are different from the stack's: what may be applied
//! continuously, to whom, and for how long.

use super::*;
use crate::ControlDurationDef;

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
        // Neither increase carries a value, so only the predicate -- and,
        // for the spell one, the caster relation -- needs checking.
        EffectDef::ModifyCost(CostModificationDef::AbilityIncrease { permanent, .. }) => {
            battlefield_only(source_zones) && shared_object_predicate(permanent)
        }
        EffectDef::ModifyCost(CostModificationDef::SourceAbilityIncrease { source, .. }) => {
            battlefield_only(source_zones) && shared_object_predicate(source)
        }
        // The discount beside it does carry a value, read off the board the
        // same way a spell discount's is.
        EffectDef::ModifyCost(CostModificationDef::AbilityReduction {
            permanent, amount, ..
        }) => {
            battlefield_only(source_zones)
                && shared_object_predicate(permanent)
                && matches!(
                    amount,
                    crate::card::ValueDef::Constant(_)
                        | crate::card::ValueDef::CountMatchingObjects(_)
                )
        }
        EffectDef::ModifyCost(CostModificationDef::SpellIncrease { spell, caster, .. }) => {
            battlefield_only(source_zones)
                && shared_object_predicate(spell)
                && matches!(
                    caster,
                    crate::card::PlayerRelation::Any
                        | crate::card::PlayerRelation::You
                        | crate::card::PlayerRelation::Opponent
                        // "Except during its controller's turn", which the
                        // relation matcher answers off the active player.
                        | crate::card::PlayerRelation::NonactivePlayer
                )
        }
        EffectDef::ModifyCost(CostModificationDef::SpellAlternative {
            spell,
            caster,
            zones,
            ..
        }) => shared_spell_alternative(source_zones, spell, caster, zones),
        // Read off a permanent rather than the card in hand, so the spell
        // predicate and the caster relation are what have to be shared.
        EffectDef::ModifyCost(CostModificationDef::SpellReduction {
            spell,
            caster,
            amount,
        }) => {
            battlefield_only(source_zones)
                && shared_object_predicate(spell)
                && matches!(
                    caster,
                    crate::card::PlayerRelation::Any
                        | crate::card::PlayerRelation::You
                        | crate::card::PlayerRelation::Opponent
                )
                && matches!(
                    amount,
                    crate::card::ValueDef::Constant(_)
                        | crate::card::ValueDef::CountMatchingObjects(_)
                )
        }
        EffectDef::ReduceGenericCostBy(value) => {
            source_zones == [ZoneKind::Hand]
                && matches!(
                    value,
                    crate::card::ValueDef::Constant(_)
                        | crate::card::ValueDef::CountMatchingObjects(_)
                        // Domain counts basic land types rather than
                        // permanents, and the planner reads it the same way
                        // it reads a count of lands.
                        | crate::card::ValueDef::BasicLandTypesControlled(_)
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
        | EffectDef::CannotBeForcedToDiscard
        | EffectDef::GainClassLevel { .. }
        | EffectDef::SubstituteBasicLandTypeUntilEndOfTurn { .. }
        | EffectDef::ReduceGenericCostBy(_)
        | EffectDef::ModifyCost(_)
        | EffectDef::LandwalkCanBeBlocked(_)
        | EffectDef::CannotAttackUnless(_)
        | EffectDef::CannotAttackIf(_)
        | EffectDef::GainControl { .. }
        | EffectDef::Sequence(_) => shared_static_non_apply_effect(source_zones, effect),
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
                    query.zones == [ZoneKind::Battlefield]
                        && shared_object_predicate(query.object)
                        && shared_static_query(query)
                }
                // A static clause names one kind of thing or the other.
                EffectRecipientSetDef::PlayersAndCreaturesTheyControl(_)
                | EffectRecipientSetDef::LegalTargets(_)
                | EffectRecipientSetDef::Objects(
                    ObjectSetDef::One(
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
                    | ObjectSetDef::LinkedExiles(_)
                    | ObjectSetDef::CardsDrawnThisTurnInHand(_)
                    | ObjectSetDef::BottomOfGraveyard(_)
                    | ObjectSetDef::LegalTargets(_)
                    | ObjectSetDef::PermanentsTargetedBy(_)
                    | ObjectSetDef::SharingNameWith(_)
                    | ObjectSetDef::SharingNameWithBinding { .. }
                    | ObjectSetDef::TopOfGraveyardMatching { .. },
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
            let battlefield_effect =
                matches!(source_zones, [ZoneKind::Battlefield | ZoneKind::Graveyard])
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
        EffectDef::DamageCannotBePreventedThisTurn
        | EffectDef::PutSourceOntoBattlefieldAttacking
        | EffectDef::BecomeMonarch { .. }
        | EffectDef::VoteForPermanentToExile { .. }
        | EffectDef::GrantFlashToNextSorcery
        | EffectDef::Randomized { .. }
        | EffectDef::Choose(_)
        | EffectDef::SimultaneousChoose(_)
        | EffectDef::ChooseCardName { .. }
        | EffectDef::BindMatching { .. }
        | EffectDef::SelectAtRandomFromZone { .. }
        | EffectDef::ForEachInBinding { .. }
        | EffectDef::PayOr(_)
        | EffectDef::SplitIntoPiles(_)
        | EffectDef::PreventDamage { .. }
        | EffectDef::Apply { .. }
        | EffectDef::May { .. }
        | EffectDef::ExileLinkedToSource { .. }
        | EffectDef::MayPlayWithoutPaying { .. }
        | EffectDef::ExileGrantingOwnerPlay { .. }
        | EffectDef::ExileGrantingControllerPlayThisTurn { .. }
        | EffectDef::ReturnLinkedExiles { .. }
        | EffectDef::Detain { .. }
        | EffectDef::ExchangeControl { .. }
        | EffectDef::InstallTrigger(_)
        | EffectDef::None
        | EffectDef::AddMana(_)
        | EffectDef::AddManaEqualTo { .. }
        | EffectDef::DealDamage { .. }
        | EffectDef::DealDamageFrom { .. }
        | EffectDef::DealDamageAndApply { .. }
        | EffectDef::DrainLife { .. }
        | EffectDef::GainLife { .. }
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
        | EffectDef::CreateTokenCopyOf { .. }
        | EffectDef::Endure { .. }
        | EffectDef::Destroy { .. }
        | EffectDef::Sacrifice { .. }
        | EffectDef::SacrificeOfChoice { .. }
        | EffectDef::ExileTopOfLibraryToPlay { .. }
        | EffectDef::ExileTopAndMayCast { .. }
        | EffectDef::MayCastTargetWithoutPaying { .. }
        | EffectDef::Mill { .. }
        | EffectDef::SearchZonesAndExileRest { .. }
        | EffectDef::MillUntil { .. }
        | EffectDef::ExileFromTopUntil { .. }
        | EffectDef::ManifestDread { .. }
        | EffectDef::Cascade
        | EffectDef::Proliferate
        | EffectDef::Explore { .. }
        | EffectDef::LookAtTopAndSelect { .. }
        | EffectDef::LookAtTopAndDistribute { .. }
        | EffectDef::LookAtHand { .. }
        | EffectDef::LookAtRandomCardInHand { .. }
        | EffectDef::RevealAtRandomFromHand { .. }
        | EffectDef::RevealHand { .. }
        | EffectDef::SearchZone { .. }
        | EffectDef::ChooseCards { .. }
        | EffectDef::ReplaceNextDrawThisTurn { .. }
        | EffectDef::IfFormat { .. }
        | EffectDef::Counter { .. }
        | EffectDef::PutSpellIntoOwnersLibrary { .. }
        | EffectDef::CopyResolvingSpell { .. }
        | EffectDef::CopyTargetSpell { .. }
        | EffectDef::AddCounters { .. }
        | EffectDef::RemoveCounters { .. }
        | EffectDef::ChangeTextBasicLandType { .. }
        | EffectDef::ChooseColor { .. }
        | EffectDef::BecomeCopyOf { .. }
        | EffectDef::PutIntoLibraryBeneathTop { .. }
        | EffectDef::MoveToZone { .. }
        | EffectDef::CreateEmblem { .. }
        | EffectDef::CreateOngoingEffect(_)
        | EffectDef::PutOntoBattlefieldThen { .. }
        | EffectDef::Transform { .. }
        | EffectDef::ScheduleTurnPhases(_)
        | EffectDef::TakeExtraTurn { .. }
        | EffectDef::CreateMyriadTokens
        | EffectDef::Special(_) => false,
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

fn shared_direct_characteristic_recipient(recipient: EffectRecipientDef) -> bool {
    matches!(
        recipient.object_reference(),
        Some(ObjectRefDef::Source | ObjectRefDef::AttachedToSource)
    )
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
            CharacteristicOperationDef::Colors(_) | CharacteristicOperationDef::Subtypes(_),
        ) => shared_static_animation_query(recipient),
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::CreatureTypes(_)) => {
            shared_direct_characteristic_recipient(recipient)
        }
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
        AppliedRuleDef::BlockRestriction(restriction) => {
            (matches!(
                recipient.object_reference(),
                Some(ObjectRefDef::Source | ObjectRefDef::AttachedToSource)
            ) || recipient.object_query().is_some())
                && match restriction.counterpart {
                    BlockRestrictionMatchDef::Any => true,
                    BlockRestrictionMatchDef::Matching(predicate)
                    | BlockRestrictionMatchDef::Except(predicate) => {
                        shared_object_predicate(predicate)
                    }
                }
                && restriction
                    .cost
                    .is_none_or(|cost| !cost.variable_x && cost.x_multiplier == 0)
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
        AppliedRuleDef::MayCastAsThoughItHadFlash(object) => {
            matches!(recipient.0, EffectRecipientSetDef::Players(_))
                && shared_object_predicate(object)
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
        // Read from the pile the source exiled as it entered, which the
        // static power-and-toughness layer can reach from that source.
        | crate::card::ValueDef::TotalPowerOfLinkedExiles
        | crate::card::ValueDef::TotalToughnessOfLinkedExiles
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
        crate::card::ValueDef::Scaled(scaled) => static_stat_value(scaled.value),
        crate::card::ValueDef::Halved(halved) => static_stat_value(halved.value),
        _ => false,
    }
}
