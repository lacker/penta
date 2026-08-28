//! Resumable decisions over frozen object collections and genuine pile groups.

use super::decision_permanent_choice::{
    EffectChoiceDecisionState, effect_choice_prompt, effect_moves_group_to_hand,
    effect_sacrifices_group,
};
use crate::card::{
    ChoiceVisibilityDef, ChooseCardsFromCollectionDef, ChooseGroupDef, ChooseObjectOrderDef,
    ChooseOneOfEachDef, CollectionInspectionDef, EffectDef, LookAtObjectsDef, PartitionGroupDef,
};

use super::{
    DecisionContinuation, DecisionOption, DecisionOrderSemantics, DecisionPreference,
    DecisionVisibility, DecisionZone, EffectResolutionContext, Game, ScopedEffect, StackObject,
    Target,
};

impl Game {
    pub(super) fn collection_card_choice_decision_state(
        &self,
        definition: ChooseCardsFromCollectionDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> Option<EffectChoiceDecisionState> {
        let chooser = self.effect_player_reference(definition.actor, object, context, scoped)?;
        let inspected =
            self.effect_object_collection(definition.source, object, context, scoped)?;
        let candidates = inspected
            .iter()
            .copied()
            .filter(|target| {
                self.effect_collection_target_matches(
                    definition.object,
                    *target,
                    object,
                    context,
                    scoped,
                )
            })
            .collect::<Vec<_>>();
        let minimum = definition.minimum.min(candidates.len());
        let maximum = definition.maximum.min(candidates.len()).max(minimum);
        let members = inspected
            .iter()
            .copied()
            .filter_map(|target| self.effect_target_card(target))
            .collect::<Vec<_>>();
        let mut options = candidates
            .iter()
            .copied()
            .enumerate()
            .map(|(index, candidate)| self.effect_target_option(index, candidate))
            .collect::<Vec<_>>();
        for option in &mut options {
            option.members.clone_from(&members);
        }
        if candidates.is_empty()
            && definition.inspection == CollectionInspectionDef::Look
            && !inspected.is_empty()
        {
            options.push(DecisionOption {
                id: 0,
                label: "Continue".into(),
                card: None,
                members,
                ability_text: None,
                zone: DecisionZone::None,
            });
        }
        Some(EffectChoiceDecisionState {
            chooser,
            candidates,
            minimum,
            maximum,
            options,
            preference: DecisionPreference::HigherCardValue,
        })
    }

    pub(super) fn queue_collection_card_choice(
        &mut self,
        definition: ChooseCardsFromCollectionDef,
        object: &StackObject,
        mut context: EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        let Some(inspected) =
            self.effect_object_collection(definition.source, object, &context, scoped)
        else {
            return;
        };
        if definition.inspection == CollectionInspectionDef::Reveal {
            self.reveal_effect_collection(&inspected);
        }
        context.bind_object_group(definition.chosen, Vec::new());
        context.bind_object_group(definition.remainder, inspected);
        let Some(state) =
            self.collection_card_choice_decision_state(definition, object, &context, scoped)
        else {
            return;
        };
        if context.object_group(definition.remainder).is_empty()
            || (state.candidates.is_empty()
                && definition.inspection == CollectionInspectionDef::Reveal)
        {
            self.resolve_effect_def(scoped.with_effect(*definition.then), object, context);
            return;
        }
        self.queue_decision(
            state.chooser,
            if state.candidates.is_empty() {
                "Continue"
            } else {
                effect_choice_prompt(
                    *definition.then,
                    crate::card::ObjectChoiceBindingDef::Objects(definition.chosen),
                )
            },
            match definition.inspection {
                CollectionInspectionDef::Look => DecisionVisibility::Private,
                CollectionInspectionDef::Reveal => DecisionVisibility::Public,
            },
            state.preference,
            state.minimum..=state.maximum,
            false,
            state.options,
            DecisionContinuation::ChooseForEffect {
                definition: scoped,
                binding: crate::card::ObjectChoiceBindingDef::Objects(definition.chosen),
                object: Box::new(object.clone()),
                context,
                candidates: state.candidates,
                effect: scoped.with_effect(*definition.then),
            },
        );
    }

    pub(super) fn queue_choose_object_order(
        &mut self,
        definition: ChooseObjectOrderDef,
        object: &StackObject,
        mut context: EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        let Some(actor) = self.effect_player_reference(definition.actor, object, &context, scoped)
        else {
            return;
        };
        let candidates = self.effect_objects(definition.input, object, &context, scoped);
        if candidates.len() <= 1 {
            context.bind_object_group(definition.ordered, candidates);
            self.resolve_effect_def(scoped.with_effect(*definition.then), object, context);
            return;
        }
        let options = candidates
            .iter()
            .copied()
            .enumerate()
            .map(|(index, target)| self.effect_target_option(index, target))
            .collect();
        let count = candidates.len();
        self.queue_decision(
            actor,
            Self::library_order_prompt(definition.placement),
            decision_visibility(definition.visibility),
            DecisionPreference::Neutral,
            count..=count,
            false,
            options,
            DecisionContinuation::ChooseObjectOrderForEffect {
                definition: scoped,
                candidates,
                object: Box::new(object.clone()),
                context,
                effect: scoped.with_effect(*definition.then),
            },
        );
        self.pending_decisions
            .last_mut()
            .expect("the arrangement was just queued")
            .observation
            .order_semantics = Some(DecisionOrderSemantics::Resolution);
    }

    pub(super) fn queue_look_at_objects(
        &mut self,
        definition: LookAtObjectsDef,
        object: &StackObject,
        context: EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        let Some(actor) = self.effect_player_reference(definition.actor, object, &context, scoped)
        else {
            return;
        };
        let Some(members) =
            self.effect_object_collection(definition.source, object, &context, scoped)
        else {
            return;
        };
        if members.is_empty() {
            self.resolve_effect_def(scoped.with_effect(*definition.then), object, context);
            return;
        }
        let shown = members
            .iter()
            .copied()
            .filter_map(|target| self.effect_target_card(target))
            .collect();
        self.queue_decision(
            actor,
            "Continue",
            decision_visibility(definition.visibility),
            DecisionPreference::Neutral,
            0..=0,
            false,
            vec![DecisionOption {
                id: 0,
                label: "Continue".into(),
                card: None,
                members: shown,
                ability_text: None,
                zone: DecisionZone::None,
            }],
            DecisionContinuation::LookAtObjectsForEffect {
                definition: scoped,
                object: Box::new(object.clone()),
                context,
                effect: scoped.with_effect(*definition.then),
            },
        );
    }

    pub(super) fn queue_partition_group(
        &mut self,
        definition: PartitionGroupDef,
        object: &StackObject,
        mut context: EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        let Some(actor) = self.effect_player_reference(definition.actor, object, &context, scoped)
        else {
            return;
        };
        let items = self.effect_objects(definition.input, object, &context, scoped);
        if items.is_empty() {
            context.bind_object_group(definition.first, Vec::new());
            context.bind_object_group(definition.second, Vec::new());
            self.resolve_effect_def(scoped.with_effect(*definition.then), object, context);
            return;
        }
        let options = items
            .iter()
            .copied()
            .enumerate()
            .map(|(index, target)| self.effect_target_option(index, target))
            .collect();
        self.queue_decision(
            actor,
            "Separate the objects into two piles",
            decision_visibility(definition.visibility),
            DecisionPreference::BalancedPartition,
            0..=items.len(),
            false,
            options,
            DecisionContinuation::PartitionGroupForEffect {
                definition: scoped,
                items,
                object: Box::new(object.clone()),
                context,
                effect: scoped.with_effect(*definition.then),
            },
        );
    }

    pub(super) fn queue_choose_group(
        &mut self,
        definition: ChooseGroupDef,
        object: &StackObject,
        mut context: EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        let Some(actor) = self.effect_player_reference(definition.actor, object, &context, scoped)
        else {
            return;
        };
        let first = self.effect_objects(definition.first, object, &context, scoped);
        let second = self.effect_objects(definition.second, object, &context, scoped);
        if first.is_empty() && second.is_empty() {
            context.bind_object_group(definition.chosen, Vec::new());
            context.bind_object_group(definition.unchosen, Vec::new());
            self.resolve_effect_def(scoped.with_effect(*definition.then), object, context);
            return;
        }
        let options = [first.as_slice(), second.as_slice()]
            .into_iter()
            .enumerate()
            .map(|(index, pile)| {
                let names = pile
                    .iter()
                    .copied()
                    .map(|target| self.effect_target_option(0, target).label)
                    .collect::<Vec<_>>();
                DecisionOption {
                    id: u32::try_from(index).unwrap_or(u32::MAX),
                    label: format!(
                        "Choose pile {} ({})",
                        index + 1,
                        if names.is_empty() {
                            "empty".into()
                        } else {
                            names.join(", ")
                        }
                    ),
                    card: None,
                    members: pile
                        .iter()
                        .copied()
                        .filter_map(|target| self.effect_target_card(target))
                        .collect(),
                    ability_text: None,
                    zone: DecisionZone::None,
                }
            })
            .collect();
        let effect = scoped.with_effect(*definition.then);
        let preference = if effect_moves_group_to_hand(effect.effect, definition.chosen) {
            DecisionPreference::HigherCardValue
        } else if effect_sacrifices_group(effect.effect, definition.chosen) {
            DecisionPreference::LowerCardValue
        } else {
            DecisionPreference::Neutral
        };
        self.queue_decision(
            actor,
            "Choose a pile",
            decision_visibility(definition.visibility),
            preference,
            1..=1,
            false,
            options,
            DecisionContinuation::ChooseGroupForEffect {
                definition: scoped,
                first,
                second,
                object: Box::new(object.clone()),
                context,
                effect: scoped.with_effect(*definition.then),
            },
        );
    }

    pub(super) fn queue_choose_one_of_each(
        &mut self,
        definition: ChooseOneOfEachDef,
        object: &StackObject,
        context: EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        let remaining = self.effect_objects(definition.input, object, &context, scoped);
        self.queue_next_one_of_each(scoped, 0, remaining, Vec::new(), object, context, false);
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn queue_next_one_of_each(
        &mut self,
        scoped: ScopedEffect,
        mut next: usize,
        remaining: Vec<Target>,
        chosen: Vec<Target>,
        object: &StackObject,
        mut context: EffectResolutionContext,
        resumed: bool,
    ) {
        let EffectDef::ChooseOneOfEach(definition) = scoped.effect else {
            return;
        };
        let Some(actor) = self.effect_player_reference(definition.actor, object, &context, scoped)
        else {
            return;
        };
        while let Some(predicate) = definition.predicates.get(next).copied() {
            let candidates = remaining
                .iter()
                .copied()
                .filter(|target| {
                    self.bound_object_matches(
                        *target,
                        predicate,
                        object.source.unwrap_or(object.id),
                    )
                })
                .collect::<Vec<_>>();
            if candidates.is_empty() {
                next += 1;
                continue;
            }
            let options = candidates
                .iter()
                .copied()
                .enumerate()
                .map(|(index, target)| self.effect_target_option(index, target))
                .collect();
            self.queue_decision(
                actor,
                Self::one_of_each_prompt(predicate),
                decision_visibility(definition.visibility),
                DecisionPreference::HigherCardValue,
                0..=1,
                false,
                options,
                DecisionContinuation::ChooseOneOfEachForEffect {
                    definition: scoped,
                    next,
                    candidates,
                    remaining,
                    chosen,
                    object: Box::new(object.clone()),
                    context,
                },
            );
            return;
        }
        context.bind_object_group(definition.chosen, chosen);
        context.bind_object_group(definition.remainder, remaining);
        let effect = scoped.with_effect(*definition.then);
        if resumed {
            self.resolve_nested_effect_before_later(effect, object, context);
        } else {
            self.resolve_effect_def(effect, object, context);
        }
    }

    pub(super) fn one_of_each_prompt(predicate: crate::card::ObjectPredicateDef) -> String {
        match predicate {
            crate::card::ObjectPredicateDef::HasType(card_type) => {
                let name = card_type.name().to_lowercase();
                let article = if name.starts_with(['a', 'e', 'i', 'o', 'u']) {
                    "an"
                } else {
                    "a"
                };
                format!("Put {article} {name} card from among them into your hand")
            }
            _ => "Choose a matching object".into(),
        }
    }
}

const fn decision_visibility(visibility: ChoiceVisibilityDef) -> DecisionVisibility {
    match visibility {
        ChoiceVisibilityDef::Public => DecisionVisibility::Public,
        ChoiceVisibilityDef::Private => DecisionVisibility::Private,
    }
}
