mod abilities;
mod ability_analysis;
mod actions;
mod casting;
mod combat;
mod decisions;
mod spell_profile;
mod targets;

use self::spell_profile::DeclarativeSpellProfile;
use super::Policy;
use crate::card::{
    AbilityCostDef, AbilityTargetDef, AbilityTargetPredicate, AlternativeCastKindDef,
    AppliedEffectDef, BasicLandType, CardBehavior, CardCatalog, CardSupertype, CardType,
    CardTypeSet, CharacteristicOperationDef, DeclarativeAbilityDef, EffectDef, EffectRecipientDef,
    ObjectPredicateDef, PlayerRelation, PowerToughnessOperationDef, SetOperationDef, SpellForm,
    ValueDef, ZoneKind,
};
use crate::game::{
    DecisionObservation, DecisionOption, DecisionPreference, DecisionZone, PlayerObservation,
    StackObjectKind, StackObservation, Step,
};
use crate::{
    AbilityOrigin, Action, AttackDefender, CardDefinitionId, CastChoices, GameObjectId,
    ObjectCharacteristics, PlayerId, Target,
};

/// A deterministic baseline that applies simple card- and combat-aware rules.
#[derive(Clone, Debug)]
pub struct HandcraftedPolicy {
    catalog: CardCatalog,
    mulligans_taken: u8,
}

impl HandcraftedPolicy {
    #[must_use]
    pub fn new(catalog: CardCatalog) -> Self {
        Self {
            catalog,
            mulligans_taken: 0,
        }
    }
}

impl Policy for HandcraftedPolicy {
    fn choose_action(&mut self, observation: &PlayerObservation) -> Option<Action> {
        if let Some(decision) = observation.decision.as_ref() {
            return self.choose_decision(observation, decision);
        }
        let action = observation
            .legal_actions
            .iter()
            .max_by_key(|action| self.score_action(observation, action))
            .cloned();
        if matches!(action, Some(Action::TakeMulligan)) {
            self.mulligans_taken += 1;
        } else if matches!(action, Some(Action::KeepHand)) {
            self.mulligans_taken = 0;
        }
        action
    }
}
