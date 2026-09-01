use super::{
    AbilityCostDef, AbilityOrigin, CardDefinitionId, CardTypeSet, DecisionOption,
    DeclarativeAbilityDef, DeclarativeSpellProfile, EffectDef, EffectRecipientDef, GameObjectId,
    HandcraftedPolicy, ObjectCharacteristics, PlayerObservation, Step, Target,
};
use crate::{EffectRecipientSetDef, ObjectSetDef, PlayerRefDef};

impl HandcraftedPolicy {
    pub(super) fn activated_target_score(
        observation: &PlayerObservation,
        target: Target,
        declarative: Option<DeclarativeSpellProfile>,
    ) -> i32 {
        if let Some(amount) = declarative.and_then(|profile| profile.damage) {
            return Self::damage_target_score(observation, target, amount);
        }
        if declarative.is_some_and(|profile| {
            profile.has(DeclarativeSpellProfile::REMOVES | DeclarativeSpellProfile::TAPS)
        }) {
            return Self::removal_target_score(observation, target);
        }
        if declarative.is_some_and(|profile| profile.has(DeclarativeSpellProfile::APPLIES)) {
            return match target {
                Target::Permanent(id) => observation
                    .battlefield
                    .iter()
                    .find(|permanent| permanent.id == id)
                    .map_or(-10_000, |permanent| {
                        let useful_combat_window = observation.step == Step::DeclareBlockers
                            || (observation.step == Step::CombatDamage
                                && observation.regular_combat_damage_pending);
                        if permanent.controller == observation.viewer
                            && useful_combat_window
                            && (permanent.attacking || permanent.blocking_this_combat)
                        {
                            1_500
                        } else {
                            -10_000
                        }
                    }),
                Target::Player(_) | Target::Card(_) | Target::Spell(_) => -10_000,
            };
        }
        Self::target_score(observation, target)
    }

    pub(super) fn discard_source_cost(
        &self,
        source_definition: Option<CardDefinitionId>,
        origin: AbilityOrigin,
    ) -> i32 {
        let Some(source_definition) = source_definition else {
            return 0;
        };
        let AbilityOrigin::Printed {
            definition,
            part,
            ability,
        } = origin
        else {
            return 0;
        };
        if definition != source_definition {
            return 0;
        }
        self.catalog
            .get(definition)
            .and_then(|card| card.part(part))
            .and_then(|part| part.rules.ability(ability))
            .and_then(|ability| match ability.definition {
                DeclarativeAbilityDef::Activated(definition)
                    if definition.costs.contains(&AbilityCostDef::DiscardSource) =>
                {
                    Some(self.card_value(source_definition))
                }
                _ => None,
            })
            .unwrap_or(0)
    }

    /// The loyalty a printed activated ability charges, when `ability` really
    /// is a clause on `definition`. Reading it off the catalog keeps the
    /// policy from guessing at a planeswalker's cost.
    pub(super) fn loyalty_cost_of(
        &self,
        source_definition: Option<CardDefinitionId>,
        ability: AbilityOrigin,
    ) -> Option<i8> {
        source_definition.and_then(|definition| {
            let AbilityOrigin::Printed {
                definition: origin_definition,
                part,
                ability,
            } = ability
            else {
                return None;
            };
            let actual = (origin_definition == definition)
                .then(|| {
                    self.catalog
                        .get(definition)?
                        .part(part)?
                        .rules
                        .ability(ability)
                })
                .flatten()?;
            match actual.definition {
                DeclarativeAbilityDef::Activated(definition) => {
                    definition.costs.iter().find_map(|cost| match *cost {
                        AbilityCostDef::Loyalty(change) => Some(change),
                        _ => None,
                    })
                }
                _ => None,
            }
        })
    }

    /// A loyalty ability's own score, before the costs every ability pays.
    /// Fighting off an attacker beats pointing damage somewhere, which beats
    /// the card's own hint, which beats a plain continuous effect.
    pub(super) fn score_loyalty_ability(
        observation: &PlayerObservation,
        targets: &[crate::TargetSelection],
        declarative: Option<DeclarativeSpellProfile>,
        structural_hint_score: i32,
        target_score: i32,
        cost: i8,
    ) -> i32 {
        let fight_score = targets
            .iter()
            .flat_map(crate::TargetSelection::targets)
            .filter_map(|target| match target {
                Target::Permanent(id) => observation
                    .battlefield
                    .iter()
                    .find(|permanent| permanent.id == *id),
                Target::Player(_) | Target::Card(_) | Target::Spell(_) => None,
            })
            .collect::<Vec<_>>();
        let loyalty_score = if fight_score.len() == 2
            && fight_score[0].controller == observation.viewer
            && fight_score[1].controller == observation.viewer.opponent()
            && fight_score[0].power.unwrap_or(0)
                >= fight_score[1].toughness.unwrap_or(0)
                    - i16::try_from(fight_score[1].damage).unwrap_or(i16::MAX)
            && fight_score[1].power.unwrap_or(0)
                < fight_score[0].toughness.unwrap_or(0)
                    - i16::try_from(fight_score[0].damage).unwrap_or(i16::MAX)
        {
            8_000
        } else if declarative.is_some_and(|profile| profile.damage.is_some()) {
            7_200 + target_score
        } else if structural_hint_score != 0 {
            6_800 + structural_hint_score
        } else if declarative.is_some_and(|profile| profile.has(DeclarativeSpellProfile::APPLIES)) {
            5_200 + target_score
        } else {
            4_500 + target_score
        };
        loyalty_score + i32::from(cost) * 100
    }

    /// What one decision option is worth. An ordinary card option is worth
    /// that card even when `members` disclose other inspected cards. A pile
    /// option has no single `card`, so it is worth its members together.
    pub(super) fn option_value(&self, option: &DecisionOption) -> i32 {
        if let Some((_, characteristics)) = option.card {
            return self.characteristics_value(characteristics);
        }
        option
            .members
            .iter()
            .map(|(_, characteristics)| self.characteristics_value(*characteristics))
            .sum()
    }

    /// Splits the options into two piles of as near equal value as it can,
    /// both nonempty, and returns the ids of the first. Liliana's ultimate
    /// hands the choice of pile to the opponent, so the safe split is the even
    /// one. Exhaustive while the set is small enough to enumerate, and a
    /// largest-first greedy fill beyond that; both are deterministic.
    pub(super) fn balanced_partition(
        &self,
        options: &[&DecisionOption],
        minimum: usize,
        maximum: usize,
    ) -> Vec<u32> {
        if options.len() < 2 {
            return Vec::new();
        }
        // A pile of everything is not a split, and neither is a pile of
        // nothing, so the size stays inside the decision's own bounds and
        // short of the whole set.
        let lowest = minimum.max(1);
        let highest = maximum.min(options.len() - 1);
        if lowest > highest {
            return Vec::new();
        }
        let values: Vec<i32> = options
            .iter()
            .map(|option| self.option_value(option))
            .collect();
        let total: i32 = values.iter().sum();
        if options.len() <= 16 {
            let mut best: Option<(i32, Vec<u32>)> = None;
            for mask in 1u32..(1 << options.len()) - 1 {
                let size = mask.count_ones() as usize;
                if size < lowest || size > highest {
                    continue;
                }
                let mut sum = 0;
                let mut chosen = Vec::new();
                for (index, option) in options.iter().enumerate() {
                    if mask & (1 << index) != 0 {
                        sum += values[index];
                        chosen.push(option.id);
                    }
                }
                let gap = (total - 2 * sum).abs();
                // Ties break on the smaller pile and then on its ids, so the
                // same board always produces the same split.
                let key = (gap, chosen.len(), chosen.clone());
                if best.as_ref().is_none_or(|(best_gap, best_ids)| {
                    key < (*best_gap, best_ids.len(), best_ids.clone())
                }) {
                    best = Some((gap, chosen));
                }
            }
            return best.map(|(_, chosen)| chosen).unwrap_or_default();
        }
        let mut order: Vec<usize> = (0..options.len()).collect();
        order.sort_by_key(|index| (-values[*index], options[*index].id));
        let mut first = Vec::new();
        let mut first_sum = 0;
        let mut second_sum = 0;
        for index in order {
            if first_sum <= second_sum && first.len() < highest {
                first_sum += values[index];
                first.push(options[index].id);
            } else {
                second_sum += values[index];
            }
        }
        first.sort_unstable();
        first
    }

    fn target_player_partitioned_for_sacrifice(effect: EffectDef) -> Option<crate::TargetIndex> {
        let EffectDef::PartitionGroup(partition) = effect else {
            return None;
        };
        let ObjectSetDef::PermanentsControlledBy(PlayerRefDef::Target(target)) = partition.input
        else {
            return None;
        };
        let EffectDef::ChooseGroup(choice) = *partition.then else {
            return None;
        };
        if choice.first != ObjectSetDef::Binding(partition.first)
            || choice.second != ObjectSetDef::Binding(partition.second)
        {
            return None;
        }
        let EffectDef::Sacrifice { object } = *choice.then else {
            return None;
        };
        (object
            == EffectRecipientDef(EffectRecipientSetDef::Objects(ObjectSetDef::Binding(
                choice.chosen,
            ))))
        .then_some(target)
    }

    /// Score the reusable "partition, let the target choose, sacrifice that
    /// group" workflow from its declarative structure. Strategic meaning
    /// stays with the shared construct rather than card-local policy metadata.
    pub(super) fn structural_hint_score(
        &self,
        observation: &PlayerObservation,
        ability: AbilityOrigin,
        targets: &[crate::TargetSelection],
    ) -> i32 {
        let AbilityOrigin::Printed {
            definition,
            part,
            ability,
        } = ability
        else {
            return 0;
        };
        let Some(target) = self
            .catalog
            .get(definition)
            .and_then(|card| card.part(part))
            .and_then(|part| part.rules.ability(ability))
            .and_then(|ability| ability.declarative_effect())
            .and_then(Self::target_player_partitioned_for_sacrifice)
        else {
            return 0;
        };
        let Some(target) = crate::TargetSlotId::from_index(target.index()) else {
            return 0;
        };
        targets
            .iter()
            .find(|selection| selection.slot() == target)
            .into_iter()
            .flat_map(crate::TargetSelection::targets)
            .filter_map(|target| match target {
                Target::Player(player) => Some(player),
                Target::Card(_) | Target::Permanent(_) | Target::Spell(_) => None,
            })
            .map(|player| {
                let value = observation
                    .battlefield
                    .iter()
                    .filter(|permanent| permanent.controller == *player)
                    .map(|permanent| self.characteristics_value(permanent.characteristics))
                    .sum::<i32>()
                    / 2;
                if *player == observation.viewer {
                    -value
                } else {
                    500 + value
                }
            })
            .sum()
    }

    fn activated_source_profile(
        &self,
        observation: &PlayerObservation,
        source: GameObjectId,
        ability: AbilityOrigin,
    ) -> (Option<CardDefinitionId>, Option<DeclarativeSpellProfile>) {
        let Some(characteristics) =
            Self::permanent_characteristics(observation, source).or_else(|| {
                Self::hand_definition(observation, source).map(|definition| {
                    ObjectCharacteristics::card(definition, crate::CardPartId::PRIMARY)
                })
            })
        else {
            return (None, None);
        };
        (
            characteristics.card_definition(),
            self.declarative_activated_profile(characteristics, ability),
        )
    }

    pub(super) fn score_ability(
        &self,
        observation: &PlayerObservation,
        source: GameObjectId,
        ability: AbilityOrigin,
        targets: &[crate::TargetSelection],
        sacrifices: &[GameObjectId],
        x: u16,
    ) -> i32 {
        let (source_definition, declarative) =
            self.activated_source_profile(observation, source, ability);
        let global_destroy_types =
            declarative.map_or_else(CardTypeSet::empty, |profile| profile.global_destroy_types);
        let target = targets
            .iter()
            .flat_map(crate::TargetSelection::targets)
            .next()
            .copied();
        let target_score = targets
            .iter()
            .flat_map(crate::TargetSelection::targets)
            .copied()
            .map(|value| Self::activated_target_score(observation, value, declarative))
            .sum::<i32>();
        // Every object the cost spends is a cost, so a clause naming two
        // cards is scored as twice the loss rather than once.
        let sacrifice_cost = sacrifices
            .iter()
            .filter(|card| **card != source)
            .filter_map(|card| Self::permanent_characteristics(observation, *card))
            .map(|characteristics| self.characteristics_value(characteristics))
            .sum::<i32>();
        let discard_source_cost = self.discard_source_cost(source_definition, ability);
        let structural_hint_score = self.structural_hint_score(observation, ability, targets);
        let loyalty_cost = self.loyalty_cost_of(source_definition, ability);
        if let Some(cost) = loyalty_cost {
            return Self::score_loyalty_ability(
                observation,
                targets,
                declarative,
                structural_hint_score,
                target_score,
                cost,
            ) - sacrifice_cost
                - discard_source_cost;
        }
        let score = match () {
            () if declarative
                .is_some_and(|profile| profile.has(DeclarativeSpellProfile::EXTRA_TURN)) =>
            {
                8_300
            }
            () if !global_destroy_types.is_empty() => {
                self.global_destroy_score(observation, global_destroy_types)
            }
            () if declarative.is_some_and(|profile| {
                profile.has(DeclarativeSpellProfile::REMOVES | DeclarativeSpellProfile::TAPS)
            }) =>
            {
                7_200 + target_score
            }
            () if declarative.is_some_and(|profile| profile.cards_drawn.is_some()) => 6_500,
            () if self.ability_spends_mana_on_nothing(
                observation,
                source,
                ability,
                source_definition,
                declarative,
                target,
            ) =>
            {
                -100
            }
            // Eating your own board to pump only pays when it wins.
            () if source_definition.is_some_and(|definition| {
                self.sacrifice_pump_wins_now(observation, source, definition, ability)
            }) =>
            {
                10_000
            }
            () if declarative.is_some_and(|profile| {
                profile.taps_source && profile.has(DeclarativeSpellProfile::APPLIES)
            }) =>
            {
                5_200 + target_score + i32::from(x) * 100
            }
            () if declarative.is_some_and(|profile| profile.damage.is_some()) => {
                7_200 + target_score
            }
            () if declarative
                .is_some_and(|profile| profile.has(DeclarativeSpellProfile::APPLIES)) =>
            {
                5_200 + target_score
            }
            () if declarative.is_some() => 4_500 + target_score,
            () => -10_000,
        };
        if !sacrifices.is_empty()
            && let Some(amount) = declarative.and_then(|profile| profile.damage)
            && matches!(target, Some(Target::Player(player)) if player == observation.viewer.opponent())
            && observation.life_totals[observation.viewer.opponent().index()]
                > i16::try_from(amount).unwrap_or(i16::MAX)
        {
            return -1_000;
        }
        score + structural_hint_score - sacrifice_cost - discard_source_cost
    }
}
