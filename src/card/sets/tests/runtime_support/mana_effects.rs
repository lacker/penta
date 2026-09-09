//! Runtime support for mana effects and their bounded choices.

use super::*;

pub(in super::super) fn shared_mana_effect(effect: EffectDef, choices_are_supported: bool) -> bool {
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
        ManaSelectionDef::ChoiceOfBundles(bundles) => {
            choices_are_supported
                && !bundles.is_empty()
                && bundles.iter().all(|bundle| bundle.total() > 0)
                && mana.also.is_none()
                && mana.variable_amount.is_none()
                && mana.amount_override.is_none()
        }
        // A combination enumerates every division across the available types.
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
    let amount_is_known = matches!(mana.mana, ManaSelectionDef::ChoiceOfBundles(_))
        || mana.amount > 0
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
                ManaRestrictionDef::CastCreatureSpellOfChosenType
                | ManaRestrictionDef::CumulativeUpkeep => true,
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
