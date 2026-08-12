//! Bot policies and deterministic game-running utilities.

mod handcrafted;
mod random;
mod runner;

use crate::{Action, PlayerObservation};

pub use self::handcrafted::HandcraftedPolicy;
pub use self::random::RandomPolicy;
pub use self::runner::{PlayError, play_game};

/// Chooses one of the actions in a player's current observation.
pub trait Policy {
    fn choose_action(&mut self, observation: &PlayerObservation) -> Option<Action>;
}

#[cfg(test)]
mod tests {
    use super::HandcraftedPolicy;
    use crate::TargetIndex;
    use crate::card::{
        CostDef, EffectDef, EffectRecipientDef, ManaCost, ObjectPredicateDef, PaymentDef,
        PlayerRelation, TargetConditionDef, TurnStepDef, ValueDef,
    };

    static TARGET_CONDITION: TargetConditionDef = TargetConditionDef {
        slot: TargetIndex::PRIMARY,
        object: ObjectPredicateDef::Any,
        then: ValueDef::Constant(1),
        otherwise: ValueDef::Constant(0),
    };
    static CONDITIONAL_EFFECT: EffectDef = EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::IfTargetMatches(&TARGET_CONDITION),
    };
    static OPTIONAL_PAYMENT_COST: [CostDef; 1] = [CostDef::Mana(ManaCost::new(1, 0))];

    #[test]
    fn target_condition_search_descends_decision_effects() {
        let may = EffectDef::May(&CONDITIONAL_EFFECT);
        let optional_payment = EffectDef::OptionalPayment {
            payment: PaymentDef::new(PlayerRelation::You, &OPTIONAL_PAYMENT_COST),
            if_paid: &CONDITIONAL_EFFECT,
        };
        let delayed = EffectDef::AtNextStep {
            step: TurnStepDef::End,
            player: PlayerRelation::You,
            effect: &CONDITIONAL_EFFECT,
        };

        assert_eq!(
            HandcraftedPolicy::target_condition_in(may),
            Some(&TARGET_CONDITION),
        );
        assert_eq!(
            HandcraftedPolicy::target_condition_in(optional_payment),
            Some(&TARGET_CONDITION),
        );
        assert_eq!(
            HandcraftedPolicy::target_condition_in(delayed),
            Some(&TARGET_CONDITION),
        );
    }
}
