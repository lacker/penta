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
        EffectDef::Choose(choice) => vec![*choice.then],
        EffectDef::PayOr(payment) => payment
            .if_paid
            .into_iter()
            .chain(payment.otherwise)
            .copied()
            .collect(),
        EffectDef::SplitIntoPiles(partition) => vec![*partition.then],
        EffectDef::ForEachInBinding { effect, .. }
        | EffectDef::May { effect, .. }
        | EffectDef::ReplaceNextDrawThisTurn { effect, .. }
        | EffectDef::IfCondition { then: effect, .. }
        | EffectDef::BindMatching { then: effect, .. }
        | EffectDef::SelectAtRandomFromZone { then: effect, .. }
        | EffectDef::ChooseCardName { then: effect, .. }
        | EffectDef::RevealAtRandomFromHand { then: effect, .. }
        | EffectDef::PutOntoBattlefieldThen { then: effect, .. }
        | EffectDef::ReturnWithHasteAndFinality { then: effect, .. } => vec![*effect],
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
        EffectDef::LookAtTopAndSelect { selection, .. } => {
            selection.then.into_iter().copied().collect()
        }
        EffectDef::CreateToken { created, .. } => {
            created.into_iter().map(|created| *created.then).collect()
        }
        EffectDef::SearchZone { then, .. }
        | EffectDef::Mill { then, .. }
        | EffectDef::ExileTopAndMayCast {
            otherwise: then, ..
        } => then.into_iter().copied().collect(),
        EffectDef::MillUntil(mill) => mill.then.into_iter().copied().collect(),
        EffectDef::Discard { then, .. } => then
            .into_iter()
            .map(|follow_up| *follow_up.effect)
            .collect(),
        EffectDef::ExchangeControl { otherwise, .. } => otherwise.into_iter().copied().collect(),

        EffectDef::AddCounters { .. }
        | EffectDef::AddMana(_)
        | EffectDef::AddManaEqualTo { .. }
        | EffectDef::GainClassLevel { .. }
        | EffectDef::AddPoisonCounters { .. }
        | EffectDef::AddEnergyCounters { .. }
        | EffectDef::Apply { .. }
        | EffectDef::Attach { .. }
        | EffectDef::AttachToSource { .. }
        | EffectDef::PhaseOut { .. }
        | EffectDef::ReturnAttached { .. }
        | EffectDef::PairWithSource { .. }
        | EffectDef::Reconfigure { .. }
        | EffectDef::Unattach { .. }
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
        | EffectDef::ReturnSpellToHand { .. }
        | EffectDef::Counter { .. }
        | EffectDef::CopyResolvingSpell { .. }
        | EffectDef::CreateEmblem { .. }
        | EffectDef::CreateOngoingEffect(_)
        | EffectDef::CreateAttachedToken { .. }
        | EffectDef::CreateTokenCopyOf { .. }
        | EffectDef::Endure { .. }
        | EffectDef::CreateMyriadTokens
        | EffectDef::DealDamage { .. }
        | EffectDef::DealDamageFrom { .. }
        | EffectDef::DealDamageAndApply { .. }
        | EffectDef::Destroy { then: None, .. }
        | EffectDef::Detain { .. }
        | EffectDef::DiscardCards { .. }
        | EffectDef::DrainLife { .. }
        | EffectDef::DrawCards { .. }
        | EffectDef::EmptyManaPool { .. }
        | EffectDef::ExileLinkedToSource { .. }
        | EffectDef::MayPlayWithoutPaying { .. }
        | EffectDef::ExileGrantingOwnerPlay { .. }
        | EffectDef::ExileGrantingControllerPlayThisTurn { .. }
        | EffectDef::GainControl { .. }
        | EffectDef::GainLife { .. }
        | EffectDef::GrantFlashToNextSorcery
        | EffectDef::SearchZonesAndExileRest { .. }
        | EffectDef::PutIntoLibraryBeneathTop { .. }
        | EffectDef::PutSourceOntoBattlefieldAttacking
        | EffectDef::VoteForPermanentToExile { .. }
        | EffectDef::BecomeMonarch { .. }
        | EffectDef::InstallTrigger(_)
        | EffectDef::LandwalkCanBeBlocked(_)
        | EffectDef::LookAtHand { .. }
        | EffectDef::LookAtRandomCardInHand { .. }
        | EffectDef::LoseLife { .. }
        | EffectDef::LoseTheGame { .. }
        | EffectDef::WinTheGame { .. }
        | EffectDef::ExileFromTopUntil { .. }
        | EffectDef::ManifestDread { .. }
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
        CreatedTokensDef, DestroyFollowUpDef, EffectRecipientDef, MillUntilDef, ObjectPredicateDef,
        TokenCharacteristics, ValueDef, ZoneKind,
    };
    use crate::ids::{ObjectSetBindingIndex, TargetIndex};

    static CHILD: EffectDef = EffectDef::None;
    static TOKEN: TokenCharacteristics =
        TokenCharacteristics::creature(&["Test"], &[], 1, 1).with_name("Walker Test");

    #[test]
    fn created_token_continuation_is_a_child() {
        let create = |created| EffectDef::CreateToken {
            token: TOKEN,
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
    fn mill_until_continuation_is_a_child() {
        static MILL_WITH_CHILD: MillUntilDef = MillUntilDef {
            player: EffectRecipientDef::Controller,
            object: ObjectPredicateDef::Any,
            matched_zone: ZoneKind::Graveyard,
            binding: None,
            then: Some(&CHILD),
        };
        static MILL_ALONE: MillUntilDef = MillUntilDef {
            then: None,
            ..MILL_WITH_CHILD
        };

        assert_eq!(
            child_effects(EffectDef::MillUntil(&MILL_WITH_CHILD)),
            vec![CHILD]
        );
        assert!(child_effects(EffectDef::MillUntil(&MILL_ALONE)).is_empty());
    }
}
