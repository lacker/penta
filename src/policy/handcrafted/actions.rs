use super::{
    Action, BasicLandType, CardDefinitionId, CardSupertype, DecisionObservation, DecisionOption,
    DecisionZone, EffectDef, EffectRecipientDef, GameObjectId, HandcraftedPolicy,
    PlayerObservation, Step,
};

impl HandcraftedPolicy {
    pub(super) fn should_mulligan(&self, observation: &PlayerObservation) -> bool {
        if self.mulligans_taken >= 2 {
            return false;
        }
        let mana_sources = observation
            .hand
            .iter()
            .filter(|(_, definition)| self.is_mana_source(*definition))
            .count();
        !(2..=5).contains(&mana_sources)
    }

    /// Whether any of a card's abilities turns the permanent into a creature.
    /// A permanent that can do that is worth keeping untapped in combat: the
    /// creature it becomes cannot attack if its own mana paid for the change.
    pub(super) fn definition_animates_itself(&self, definition: CardDefinitionId) -> bool {
        self.catalog.get(definition).is_some_and(|card| {
            card.parts.iter().any(|part| {
                part.rules.ability_clauses().iter().any(|ability| {
                    matches!(
                        ability.declarative_effect(),
                        Some(EffectDef::Apply {
                            recipient: EffectRecipientDef::Source,
                            effect: crate::card::AppliedEffectDef::Animate(_),
                            ..
                        })
                    )
                })
            })
        })
    }

    pub(super) fn mana_action_score(
        &self,
        observation: &PlayerObservation,
        source: GameObjectId,
    ) -> i32 {
        let saving_an_attacker = observation.active_player == observation.viewer
            && matches!(
                observation.step,
                Step::BeginningOfCombat | Step::DeclareAttackers
            )
            && observation.mana_pools[observation.viewer.index()].total() == 0
            && observation.battlefield.iter().any(|permanent| {
                permanent.controller == observation.viewer
                    && !permanent.tapped
                    && permanent.power.is_none()
                    && self.definition_animates_itself(permanent.definition)
            });
        if saving_an_attacker
            && !Self::permanent_definition(observation, source)
                .is_some_and(|definition| self.definition_animates_itself(definition))
        {
            8_800
        } else {
            -100
        }
    }

    pub(super) fn score_land(&self, observation: &PlayerObservation, card: GameObjectId) -> i32 {
        let definition = Self::hand_definition(observation, card);
        // The legend rule bins a duplicate on arrival. Replacing a tapped copy
        // with a fresh one is fine; duplicating an untapped one wastes both
        // the card and the land drop.
        if definition
            .and_then(|id| self.catalog.get(id))
            .is_some_and(|card| card.rules.has_supertype(CardSupertype::Legendary))
            && observation.battlefield.iter().any(|permanent| {
                permanent.controller == observation.viewer
                    && Some(permanent.definition) == definition
                    && !permanent.tapped
            })
        {
            return 40;
        }
        let basic_land_type_count =
            definition
                .and_then(|id| self.catalog.get(id))
                .map_or(0, |card| {
                    BasicLandType::ALL
                        .into_iter()
                        .filter(|land_type| card.rules.has_subtype(land_type.subtype()))
                        .count()
                });
        if basic_land_type_count >= 2 {
            return 9_400;
        }
        if basic_land_type_count == 1 {
            return 9_300;
        }
        9_000
    }

    pub(super) fn score_untap(
        &self,
        observation: &PlayerObservation,
        permanents: &[GameObjectId],
    ) -> i32 {
        8_000
            + permanents
                .iter()
                .filter_map(|id| {
                    observation
                        .battlefield
                        .iter()
                        .find(|permanent| permanent.id == *id)
                })
                .map(|permanent| {
                    let card = self.card_value(permanent.definition);
                    let power = i32::from(permanent.power.unwrap_or(0).max(0));
                    card + power * 10
                })
                .sum::<i32>()
    }

    pub(super) fn score_action(&self, observation: &PlayerObservation, action: &Action) -> i32 {
        match action {
            Action::KeepHand => 10_000,
            Action::TakeMulligan if self.should_mulligan(observation) => 11_000,
            Action::TakeMulligan => -5_000,
            Action::BottomCards { cards } | Action::DiscardCards { cards } => {
                9_000
                    - cards
                        .iter()
                        .filter_map(|card| Self::hand_definition(observation, *card))
                        .map(|definition| self.card_value(definition))
                        .sum::<i32>()
            }
            Action::ChooseDecision { options, .. } => {
                let selected_value = observation.decision.as_ref().map_or(0, |decision| {
                    decision
                        .options
                        .iter()
                        .filter(|option| options.contains(&option.id))
                        .filter_map(|option| option.card)
                        .map(|(_, definition)| self.card_value(definition))
                        .sum::<i32>()
                });
                match observation
                    .decision
                    .as_ref()
                    .map(|decision| decision.preference)
                {
                    Some(crate::DecisionPreference::HigherCardValue) => 8_000 + selected_value,

                    Some(crate::DecisionPreference::LowerCardValue) => 8_000 - selected_value,
                    Some(crate::DecisionPreference::PreferOption(preferred)) => {
                        8_000 + i32::from(options.contains(&preferred))
                    }
                    Some(
                        crate::DecisionPreference::BalancedPartition
                        | crate::DecisionPreference::LinkedExileTargets
                        | crate::DecisionPreference::RemovalChoice
                        | crate::DecisionPreference::Neutral,
                    )
                    | None => 8_000,
                }
            }
            Action::CancelDecision { .. } => -1_000,
            Action::ChooseUntap { permanents } => self.score_untap(observation, permanents),
            Action::PlayLand { card, .. } => self.score_land(observation, *card),
            Action::ActivateManaAbility { source, .. } => {
                self.mana_action_score(observation, *source)
            }
            Action::PayLifeForMana => 5,
            Action::CastSpell { card, choices, .. } => self.score_cast(observation, *card, choices),
            Action::ActivateAbility {
                source,
                ability,
                targets,
                cost_object,
                x,
            } => self.score_ability(observation, *source, *ability, targets, *cost_object, *x),
            Action::DeclareAttacker { attacker, defender } => {
                self.score_attack(observation, *attacker)
                    + Self::defender_preference(observation, *attacker, *defender)
            }
            Action::DeclareBlocker { blocker, attacker } => {
                Self::score_block(observation, *blocker, *attacker)
            }
            Action::FinishDeclaringAttackers | Action::FinishDeclaringBlockers => 1_000,
            Action::AssignCombatDamage { assignments, .. } => {
                6_000 + Self::score_assignment(observation, assignments)
            }
            Action::PassPriority => 0,
            Action::Concede => i32::MIN,
        }
    }

    pub(super) fn linked_exile_target_score(
        &self,
        observation: &PlayerObservation,
        decision: &DecisionObservation,
        option: &DecisionOption,
    ) -> i32 {
        let Some((object, definition)) = option.card else {
            return -10_000;
        };
        let value = self.card_value(definition).max(1);
        match option.zone {
            DecisionZone::Battlefield => observation
                .battlefield
                .iter()
                .find(|permanent| permanent.id == object)
                .map_or(-value, |permanent| {
                    if permanent.controller == decision.player {
                        -value
                    } else {
                        value
                    }
                }),
            DecisionZone::Graveyard => {
                if observation.graveyards[decision.player.index()]
                    .iter()
                    .any(|(card, _)| *card == object)
                {
                    value
                } else {
                    -value
                }
            }
            DecisionZone::Hand
            | DecisionZone::Stack
            | DecisionZone::Library
            | DecisionZone::Exile
            | DecisionZone::Command
            | DecisionZone::DrawnThisStep
            | DecisionZone::None => -value,
        }
    }

    pub(super) fn battlefield_removal_choice_score(
        &self,
        observation: &PlayerObservation,
        decision: &DecisionObservation,
        option: &DecisionOption,
    ) -> i32 {
        let Some((object, definition)) = option.card else {
            return -10_000;
        };
        let value = self.card_value(definition).max(1);
        observation
            .battlefield
            .iter()
            .find(|permanent| permanent.id == object)
            .map_or(-value, |permanent| {
                if permanent.controller == decision.player {
                    -value
                } else {
                    value
                }
            })
    }
}
