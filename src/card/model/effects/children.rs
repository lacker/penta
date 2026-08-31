//! The recursive edges in the declarative effect tree.

use super::EffectDef;

/// Kept as one exhaustive match so adding a recursive effect variant cannot
/// silently classify it as a leaf and make validators or semantic locators
/// skip its branch.
#[allow(clippy::too_many_lines)]
pub(crate) fn child_effects(effect: EffectDef) -> Vec<EffectDef> {
    match effect {
        EffectDef::Sequence(effects) => effects.to_vec(),
        EffectDef::Randomized {
            on_success,
            on_failure,
            ..
        } => vec![*on_success, *on_failure],
        EffectDef::MillWhileMatching(mill) => vec![*mill.body, *mill.on_match],
        EffectDef::Choose(choice) => vec![*choice.then],
        EffectDef::ChooseCardsFromCollection(choice) => vec![*choice.then],
        EffectDef::LookAtObjects(definition) => vec![*definition.then],
        EffectDef::ChooseObjectOrder(definition) => vec![*definition.then],
        EffectDef::ClassifyObjects(definition) => vec![*definition.then],
        EffectDef::RevealAndClassifyCards(definition) => vec![*definition.then],
        EffectDef::CombineObjects(definition) => vec![*definition.then],
        EffectDef::ChooseOneOfEach(definition) => vec![*definition.then],
        EffectDef::ChooseGroup(definition) => vec![*definition.then],
        EffectDef::BindObjects(definition) => vec![*definition.then],
        EffectDef::IfNoObjects(definition) => {
            vec![*definition.if_empty, *definition.otherwise]
        }
        EffectDef::PartitionGroup(definition) => vec![*definition.then],
        EffectDef::RandomizeObjectOrder(definition) => vec![*definition.then],
        EffectDef::RevealObjects(definition) => vec![*definition.then],
        EffectDef::MoveObjects(definition) => vec![*definition.then],
        EffectDef::PutObjectsOntoBattlefieldFaceDown(definition) => vec![*definition.then],
        EffectDef::PayOr(payment) => payment
            .if_paid
            .into_iter()
            .chain(payment.otherwise)
            .copied()
            .collect(),
        EffectDef::BindOutput { effect, .. }
        | EffectDef::ForEachInBinding { effect, .. }
        | EffectDef::May { effect, .. }
        | EffectDef::ChooseCounterKind { then: effect, .. }
        | EffectDef::ReplaceNextDrawThisTurn { effect, .. }
        | EffectDef::ChooseCardName { then: effect, .. }
        | EffectDef::PutOntoBattlefieldThen { then: effect, .. }
        | EffectDef::WithBattlefieldArrival { effect, .. }
        | EffectDef::PermitLookAtExiled { then: effect, .. }
        | EffectDef::ExileLinkedToSource {
            then: Some(effect), ..
        } => vec![*effect],
        effect @ (EffectDef::IfCondition { .. } | EffectDef::IfElseCondition { .. }) => {
            let conditional = effect
                .conditional()
                .expect("conditional variants expose their shared shape");
            std::iter::once(*conditional.then)
                .chain(conditional.otherwise.copied())
                .collect()
        }
        EffectDef::WithZoneMoveResult { effect, then, .. } => vec![*effect, *then],
        EffectDef::ChooseEffect { choices, .. } => {
            choices.iter().map(|choice| choice.effect).collect()
        }
        EffectDef::Destroy {
            then: Some(follow_up),
            ..
        } => vec![*follow_up.effect],
        EffectDef::IfFormat {
            then, otherwise, ..
        } => vec![*then, *otherwise],
        EffectDef::SacrificeOfChoice {
            then, otherwise, ..
        } => then.into_iter().chain(otherwise).copied().collect(),
        EffectDef::CreateToken { created, .. } => {
            created.into_iter().map(|created| *created.then).collect()
        }
        EffectDef::SearchZone { then, .. }
        | EffectDef::ExileTopAndMayCast {
            otherwise: then, ..
        } => then.into_iter().copied().collect(),
        EffectDef::Discard { then, .. } => then
            .into_iter()
            .map(|follow_up| *follow_up.effect)
            .collect(),
        EffectDef::ExchangeControl { otherwise, .. } => otherwise.into_iter().copied().collect(),
        EffectDef::Fight { excess, .. } => excess
            .into_iter()
            .map(|continuation| *continuation.then)
            .collect(),

        // A distributed look runs nothing after a card lands, so like every
        // other leaf below it has no child effect to walk.
        EffectDef::AddCounters { .. }
        | EffectDef::AddMana(_)
        | EffectDef::AddManaEqualTo { .. }
        | EffectDef::SelectAtRandomFromZone { .. }
        | EffectDef::RevealAtRandomFromHand { .. }
        | EffectDef::ContinueReplacedDraw
        | EffectDef::Mill { .. }
        | EffectDef::MillUntil(_)
        | EffectDef::GainClassLevel { .. }
        | EffectDef::AddPlayerCounters { .. }
        | EffectDef::Apply { .. }
        | EffectDef::Attach { .. }
        | EffectDef::AttachToSource { .. }
        | EffectDef::PairWithSource { .. }
        | EffectDef::Reconfigure { .. }
        | EffectDef::Unattach { .. }
        | EffectDef::PhaseOut { .. }
        | EffectDef::BecomeCopyOf { .. }
        | EffectDef::CannotAttackUnless(_)
        | EffectDef::CannotAttackIf(_)
        | EffectDef::CannotBeForcedToSacrifice
        | EffectDef::CannotBeForcedToDiscard
        | EffectDef::ChooseColor { .. }
        | EffectDef::ChangeTextBasicLandType { .. }
        | EffectDef::SubstituteBasicLandTypeUntilEndOfTurn { .. }
        | EffectDef::ChooseCards { .. }
        | EffectDef::PutSpellIntoOwnersLibrary { .. }
        | EffectDef::Counter { .. }
        | EffectDef::CopyStackObject(_)
        | EffectDef::ChangeStackTargets(_)
        | EffectDef::CreateEmblem { .. }
        | EffectDef::CreateOngoingEffect(_)
        | EffectDef::CreateAttachedToken { .. }
        | EffectDef::Endure { .. }
        | EffectDef::CreateMyriadTokens
        | EffectDef::DealDamage { .. }
        | EffectDef::DealDamageSimultaneously(_)
        | EffectDef::DealDamageFrom { .. }
        | EffectDef::DealDamageAndApply { .. }
        | EffectDef::Destroy { then: None, .. }
        | EffectDef::Detain { .. }
        | EffectDef::DiscardCards { .. }
        | EffectDef::DrainLife { .. }
        | EffectDef::DrawCards { .. }
        | EffectDef::EmptyManaPool { .. }
        | EffectDef::ExileLinkedToSource { then: None, .. }
        | EffectDef::MayPlayWithoutPaying { .. }
        | EffectDef::ExileGrantingOwnerPlay { .. }
        | EffectDef::ExileGrantingControllerPlayThisTurn { .. }
        | EffectDef::GainControl { .. }
        | EffectDef::GainLife { .. }
        | EffectDef::SetLifeTotal { .. }
        | EffectDef::SearchZonesAndExileRest { .. }
        | EffectDef::PutIntoLibraryBeneathTop { .. }
        | EffectDef::PutSourceOntoBattlefieldAttacking
        | EffectDef::VoteForPermanentToExile { .. }
        | EffectDef::BecomeMonarch { .. }
        | EffectDef::InstallTrigger(_)
        | EffectDef::LandwalkCanBeBlocked(_)
        | EffectDef::LookAtHand { .. }
        | EffectDef::ExileOneFromEachZone { .. }
        | EffectDef::PermitCastFromGraveyardThisTurn { .. }
        | EffectDef::LookAtRandomCardInHand { .. }
        | EffectDef::LoseLife { .. }
        | EffectDef::LoseTheGame { .. }
        | EffectDef::WinTheGame { .. }
        | EffectDef::ExileFromTopUntil { .. }
        | EffectDef::Cascade
        | EffectDef::ExileTopOfLibraryToPlay { .. }
        | EffectDef::MayCastTargetWithoutPaying { .. }
        | EffectDef::MoveToZone { .. }
        | EffectDef::RevealHand { .. }
        | EffectDef::RemoveFromCombat { .. }
        | EffectDef::None
        | EffectDef::DamageCannotBePreventedThisTurn
        | EffectDef::PreventDamage { .. }
        | EffectDef::ReduceGenericCostBy(_)
        | EffectDef::ModifyCost(_)
        | EffectDef::Regenerate { .. }
        | EffectDef::DoubleCounters { .. }
        | EffectDef::ModifyCounters { .. }
        | EffectDef::RemoveAllCounters { .. }
        | EffectDef::RemoveCounters { .. }
        | EffectDef::Explore { .. }
        | EffectDef::Proliferate
        | EffectDef::ReturnLinkedExiles { .. }
        | EffectDef::Sacrifice { .. }
        | EffectDef::ScheduleTurnPhases(_)
        | EffectDef::BuryGraveyard { .. }
        | EffectDef::ShuffleLibrary { .. }
        | EffectDef::SkipNextUntapSteps { .. }
        | EffectDef::Special(_)
        | EffectDef::ConditionalStatic(_)
        | EffectDef::StaticApply { .. }
        | EffectDef::TakeExtraTurn { .. }
        | EffectDef::Tap { .. }
        | EffectDef::Transform { .. }
        | EffectDef::Saddle { .. }
        | EffectDef::Untap { .. } => Vec::new(),
        EffectDef::SimultaneousChoose(definition) => vec![*definition.then],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{
        BattlefieldEntryModificationDef, CreatedTokensDef, DestroyFollowUpDef, EffectRecipientDef,
        MillUntilDef, ObjectPredicateDef, TokenCharacteristics, ValueDef, ZoneKind, ZonePlacement,
    };
    use crate::ids::{ObjectSetBindingIndex, TargetIndex};

    static CHILD: EffectDef = EffectDef::None;
    static TOKEN: TokenCharacteristics =
        TokenCharacteristics::creature(&["Test"], &[], 1, 1).with_name("Walker Test");

    #[test]
    fn created_token_continuation_is_a_child() {
        let create = |created| EffectDef::CreateToken {
            token: TOKEN,
            copy: None,
            controller: None,
            count: ValueDef::Constant(1),
            tapped: false,
            attacking: false,
            counters: None,
            created,
        };

        assert_eq!(
            child_effects(create(Some(CreatedTokensDef {
                binding: ObjectSetBindingIndex::PRIMARY,
                then: &CHILD,
            }))),
            vec![CHILD],
        );
        assert!(child_effects(create(None)).is_empty());
    }

    #[test]
    fn battlefield_arrival_wraps_a_zone_move() {
        static MOVE: EffectDef = EffectDef::MoveToZone {
            object: EffectRecipientDef::Source,
            zone: ZoneKind::Battlefield,
            placement: ZonePlacement::Top,
        };
        let wrapped = EffectDef::WithBattlefieldArrival {
            effect: &MOVE,
            arrival: super::super::BattlefieldArrivalDef {
                modifications: &[BattlefieldEntryModificationDef::Tapped],
                ..super::super::BattlefieldArrivalDef::DEFAULT
            },
        };

        assert_eq!(child_effects(wrapped), vec![MOVE]);
    }

    #[test]
    fn zone_move_result_exposes_the_move_and_follow_up() {
        static MOVE: EffectDef = EffectDef::MoveToZone {
            object: EffectRecipientDef::Source,
            zone: ZoneKind::Battlefield,
            placement: ZonePlacement::Top,
        };
        let wrapped = EffectDef::WithZoneMoveResult {
            effect: &MOVE,
            binding: ObjectSetBindingIndex::PRIMARY,
            then: &CHILD,
        };

        assert_eq!(child_effects(wrapped), vec![MOVE, CHILD]);
    }

    #[test]
    fn destroy_continuation_is_a_child() {
        let destroy = |then| EffectDef::Destroy {
            object: EffectRecipientDef::Source,
            can_regenerate: true,
            then,
        };

        assert_eq!(
            child_effects(destroy(Some(DestroyFollowUpDef {
                binding: ObjectSetBindingIndex::PRIMARY,
                effect: &CHILD,
            }))),
            vec![CHILD],
        );
        assert!(child_effects(destroy(None)).is_empty());
    }

    #[test]
    fn impossible_exchange_continuation_is_a_child() {
        assert_eq!(
            child_effects(EffectDef::ExchangeControl {
                first: EffectRecipientDef::Source,
                second: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                otherwise: Some(&CHILD),
            }),
            vec![CHILD],
        );
    }

    #[test]
    fn mill_until_is_a_leaf() {
        static MILL: MillUntilDef = MillUntilDef {
            player: EffectRecipientDef::Controller,
            object: ObjectPredicateDef::Any,
            matched_zone: ZoneKind::Graveyard,
        };
        assert!(child_effects(EffectDef::MillUntil(&MILL)).is_empty());
    }
}
