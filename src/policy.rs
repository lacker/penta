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
        AbilityDef, EffectDef, EffectRecipientDef, InstalledTriggerDef, ObjectPredicateDef,
        PayOrDef, PlayerRelation, TargetConditionDef, TriggerEventDef, TurnStepDef, ValueDef,
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
    static DELAYED_CONDITIONAL: AbilityDef = AbilityDef::triggered(
        "At the beginning of your next end step, apply the conditional effect.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::End,
            player: PlayerRelation::You,
        },
        CONDITIONAL_EFFECT,
    );
    #[test]
    fn target_condition_search_descends_decision_effects() {
        let may = EffectDef::May {
            player: EffectRecipientDef::Controller,
            effect: &CONDITIONAL_EFFECT,
        };
        let optional_payment = EffectDef::PayOr(PayOrDef::optional(
            &[crate::CostDef::Mana(crate::mana_cost!("{1}"))],
            &CONDITIONAL_EFFECT,
        ));
        let delayed = EffectDef::InstallTrigger(InstalledTriggerDef::once(&DELAYED_CONDITIONAL));

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
