use crate::card::{
    ChooseDef, ChooseExactDef, EffectDef, EffectRecipientDef, EffectRecipientSetDef,
    ObjectChoiceBindingDef, ObjectSetDef, ZoneKind,
};
use crate::{GameObjectId, ObjectSetBindingIndex};

use super::decision_offers::effect_choice_visibility;
use super::{
    CardPartId, DecisionContinuation, DecisionOption, DecisionPreference, DecisionZone,
    EffectResolutionContext, Game, ObjectCharacteristics, ScopedEffect, StackObject, Target,
};

pub(super) struct EffectChoiceDecisionState {
    pub(super) chooser: super::PlayerId,
    pub(super) candidates: Vec<Target>,
    pub(super) minimum: usize,
    pub(super) maximum: usize,
    pub(super) options: Vec<DecisionOption>,
    pub(super) preference: DecisionPreference,
}

impl Game {
    /// Evaluates a computed exact cardinality once, then uses the ordinary
    /// bounded object-choice machinery for the decision and continuation.
    pub(super) fn queue_exact_effect_choice(
        &mut self,
        definition: ChooseExactDef,
        object: &StackObject,
        context: EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        let amount = usize::try_from(
            self.effect_value(definition.amount, object, &context, scoped)
                .max(0),
        )
        .unwrap_or(usize::MAX);
        self.queue_effect_choice(
            Self::fixed_effect_choice(definition, amount),
            object,
            context,
            scoped,
        );
    }

    pub(super) const fn fixed_effect_choice(
        definition: ChooseExactDef,
        amount: usize,
    ) -> ChooseDef {
        ChooseDef {
            binding: ObjectChoiceBindingDef::Objects(definition.binding),
            unchosen: None,
            chooser: definition.chooser,
            candidates: definition.candidates,
            exclude: definition.exclude,
            minimum: amount,
            maximum: amount,
            visibility: definition.visibility,
            then: definition.then,
        }
    }

    pub(super) fn exact_effect_choice_decision_state(
        &self,
        definition: ChooseExactDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> Option<(ChooseDef, EffectChoiceDecisionState)> {
        let amount = usize::try_from(
            self.effect_value(definition.amount, object, context, scoped)
                .max(0),
        )
        .unwrap_or(usize::MAX);
        let fixed = Self::fixed_effect_choice(definition, amount);
        let state = self.effect_choice_decision_state(fixed, object, context, scoped)?;
        Some((fixed, state))
    }

    /// Offers one generic bounded, non-targeting object choice and resumes its
    /// nested effect with the selected object or group in the resolution
    /// context. Candidate `Target` values are retained beside the observation
    /// so choosing a spell cannot later be reconstructed as a permanent.
    pub(super) fn queue_effect_choice(
        &mut self,
        definition: ChooseDef,
        object: &StackObject,
        context: EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        let Some(state) = self.effect_choice_decision_state(definition, object, &context, scoped)
        else {
            return;
        };

        // A required choice with nothing to choose is not made at all, and
        // what follows it reads the object that would have been chosen: a
        // Currency Converter with an empty bank puts no card anywhere and
        // so pays out neither half of "if it's a land ... if it's a nonland".
        if definition.minimum > 0 && state.candidates.is_empty() {
            return;
        }

        // A mandatory instruction has no decision when every legal object is
        // forced, unless its binding preserves resolution order and at least
        // two objects still need ordering. An optional instruction still asks,
        // because declining is itself the player's choice even with one
        // candidate -- but not with none, where there is nothing to decline
        // and the only legal answer is the empty one.
        if effect_choice_resolves_automatically(definition, state.candidates.len()) {
            let mut context = context;
            if let Some(unchosen) = definition.unchosen {
                context.bind_object_group(unchosen, Vec::new());
            }
            Self::bind_effect_choice(&mut context, definition.binding, state.candidates);
            self.resolve_effect_def(scoped.with_effect(*definition.then), object, context);
            return;
        }

        self.queue_decision(
            state.chooser,
            effect_choice_prompt(*definition.then, definition.binding),
            effect_choice_visibility(definition.visibility),
            state.preference,
            state.minimum..=state.maximum,
            false,
            state.options,
            DecisionContinuation::ChooseForEffect {
                definition: scoped,
                binding: definition.binding,
                object: Box::new(object.clone()),
                context,
                candidates: state.candidates,
                effect: scoped.with_effect(*definition.then),
            },
        );
        if matches!(
            definition.binding,
            ObjectChoiceBindingDef::OrderedObjects(_)
        ) {
            self.pending_decisions
                .last_mut()
                .expect("the effect choice was just queued")
                .observation
                .order_semantics = Some(super::DecisionOrderSemantics::Resolution);
        }
    }

    pub(super) fn effect_choice_decision_state(
        &self,
        definition: ChooseDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> Option<EffectChoiceDecisionState> {
        let chooser = self.effect_player_reference(definition.chooser, object, context, scoped)?;
        let excluded = definition.exclude.and_then(|reference| {
            self.effect_object_reference_id(reference, object, context, scoped)
        });
        let mut candidates = self.effect_objects(definition.candidates, object, context, scoped);
        candidates.retain(|candidate| target_object_id(*candidate) != excluded);
        let minimum = definition.minimum.min(candidates.len());
        let maximum = definition.maximum.min(candidates.len()).max(minimum);
        let options = candidates
            .iter()
            .copied()
            .enumerate()
            .map(|(index, candidate)| self.effect_target_option(index, candidate))
            .collect();
        let preference = if effect_removes_binding(*definition.then, definition.binding) {
            if candidates.iter().all(|candidate| {
                matches!(candidate, Target::Permanent(id)
                    if self.permanent_controller(*id) == Some(chooser))
            }) {
                DecisionPreference::LowerCardValue
            } else {
                DecisionPreference::RemovalChoice
            }
        } else {
            DecisionPreference::Neutral
        };
        Some(EffectChoiceDecisionState {
            chooser,
            candidates,
            minimum,
            maximum,
            options,
            preference,
        })
    }

    pub(super) fn bind_effect_choice(
        context: &mut EffectResolutionContext,
        binding: ObjectChoiceBindingDef,
        selected: Vec<Target>,
    ) {
        match binding {
            ObjectChoiceBindingDef::Object(binding) => {
                context.bind_single_object(binding, selected.first().copied());
            }
            ObjectChoiceBindingDef::Objects(binding)
            | ObjectChoiceBindingDef::OrderedObjects(binding) => {
                context.bind_object_group(binding, selected);
            }
        }
    }

    pub(super) fn effect_target_option(&self, index: usize, target: Target) -> DecisionOption {
        let (label, card, zone) = match target {
            Target::Permanent(id) => self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == id)
                .map_or_else(
                    || ("Unknown permanent".into(), None, DecisionZone::Battlefield),
                    |permanent| {
                        (
                            self.effective_permanent_name(permanent).map_or_else(
                                || "Unknown permanent".into(),
                                std::borrow::Cow::into_owned,
                            ),
                            Some((id, Self::effective_rules_source(permanent))),
                            DecisionZone::Battlefield,
                        )
                    },
                ),
            Target::Spell(id) => self
                .stack
                .iter()
                .find(|candidate| candidate.id == id)
                .map_or_else(
                    || ("Unknown spell".into(), None, DecisionZone::Stack),
                    |candidate| {
                        let characteristics = candidate.presentation();
                        (
                            match characteristics {
                                ObjectCharacteristics::Card { definition, part } => self
                                    .catalog
                                    .get(definition)
                                    .and_then(|card| card.part(part))
                                    .map_or_else(
                                        || "Unknown spell".into(),
                                        |part| part.name.clone(),
                                    ),
                                ObjectCharacteristics::Token { token, part } => {
                                    token.part(part).map_or_else(
                                        || "Unknown spell".into(),
                                        |part| part.name().into_owned(),
                                    )
                                }
                                ObjectCharacteristics::Emblem { emblem } => emblem.name().into(),
                                ObjectCharacteristics::FaceDown { face_down } => {
                                    face_down.display_name().into()
                                }
                            },
                            Some((id, characteristics)),
                            DecisionZone::Stack,
                        )
                    },
                ),
            Target::Card(id) => self.card_in_nonbattlefield_zone(id).map_or_else(
                || ("Unknown card".into(), None, DecisionZone::None),
                |(zone, card)| {
                    (
                        self.catalog.get(card.definition).map_or_else(
                            || "Unknown card".into(),
                            |definition| definition.name.clone(),
                        ),
                        Some((
                            id,
                            ObjectCharacteristics::card(card.definition, CardPartId::PRIMARY),
                        )),
                        decision_zone(zone),
                    )
                },
            ),
            Target::Player(player) => (
                format!("Player {}", player.index() + 1),
                None,
                DecisionZone::None,
            ),
        };
        DecisionOption {
            id: u32::try_from(index).unwrap_or(u32::MAX),
            label,
            card,
            members: Vec::new(),
            ability_text: None,
            zone,
        }
    }

    pub(super) fn effect_target_card(
        &self,
        target: Target,
    ) -> Option<(GameObjectId, ObjectCharacteristics)> {
        match target {
            Target::Permanent(id) => self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == id)
                .map(|permanent| (id, Self::effective_rules_source(permanent))),
            Target::Spell(id) => self
                .stack
                .iter()
                .find(|candidate| candidate.id == id)
                .map(|candidate| (id, candidate.presentation())),
            Target::Card(id) => self.card_in_nonbattlefield_zone(id).map(|(_, card)| {
                (
                    id,
                    ObjectCharacteristics::card(card.definition, CardPartId::PRIMARY),
                )
            }),
            Target::Player(_) => None,
        }
    }
}

pub(super) fn effect_choice_resolves_automatically(
    definition: ChooseDef,
    candidate_count: usize,
) -> bool {
    let ordering_matters = matches!(
        definition.binding,
        ObjectChoiceBindingDef::OrderedObjects(_)
    ) && candidate_count > 1;
    candidate_count == 0
        || (!ordering_matters && definition.minimum > 0 && candidate_count <= definition.minimum)
}

pub(super) fn effect_choice_prompt(
    effect: EffectDef,
    binding: ObjectChoiceBindingDef,
) -> &'static str {
    let (ObjectChoiceBindingDef::Objects(binding)
    | ObjectChoiceBindingDef::OrderedObjects(binding)) = binding
    else {
        return "Choose objects";
    };
    match effect {
        EffectDef::MoveToZone {
            object,
            zone,
            placement,
        } if recipient_uses_binding(object, ObjectChoiceBindingDef::Objects(binding)) => {
            match (zone, placement) {
                (ZoneKind::Hand, _) => "Put a card into your hand",
                (ZoneKind::Library, crate::card::ZonePlacement::Top) => {
                    "Put a card on top of your library"
                }
                (ZoneKind::Library, crate::card::ZonePlacement::Bottom) => {
                    "Put a card on the bottom of your library"
                }
                (ZoneKind::Graveyard, _) => "Put a card into your graveyard",
                (ZoneKind::Exile, _) => "Exile a card",
                _ => "Choose objects",
            }
        }
        EffectDef::MoveObjects(definition)
            if definition.input == ObjectSetDef::Binding(binding) =>
        {
            match (definition.zone, definition.placement) {
                (ZoneKind::Hand, _) => "Put a card into your hand",
                (ZoneKind::Library, crate::card::ZonePlacement::Top) => {
                    "Put a card on top of your library"
                }
                (ZoneKind::Library, crate::card::ZonePlacement::Bottom) => {
                    "Put a card on the bottom of your library"
                }
                (ZoneKind::Graveyard, _) => "Put a card into your graveyard",
                (ZoneKind::Exile, _) => "Exile a card",
                _ => "Choose objects",
            }
        }
        _ => crate::card::child_effects(effect)
            .into_iter()
            .find_map(|child| {
                let prompt = effect_choice_prompt(child, ObjectChoiceBindingDef::Objects(binding));
                (prompt != "Choose objects").then_some(prompt)
            })
            .unwrap_or("Choose objects"),
    }
}

fn target_object_id(target: Target) -> Option<GameObjectId> {
    match target {
        Target::Player(_) => None,
        Target::Card(id) | Target::Permanent(id) | Target::Spell(id) => Some(id),
    }
}

const fn decision_zone(zone: ZoneKind) -> DecisionZone {
    match zone {
        ZoneKind::Library => DecisionZone::Library,
        ZoneKind::Hand => DecisionZone::Hand,
        ZoneKind::Battlefield => DecisionZone::Battlefield,
        ZoneKind::Graveyard => DecisionZone::Graveyard,
        ZoneKind::Stack => DecisionZone::Stack,
        ZoneKind::Exile => DecisionZone::Exile,
        ZoneKind::Command => DecisionZone::Command,
    }
}

fn recipient_uses_binding(recipient: EffectRecipientDef, binding: ObjectChoiceBindingDef) -> bool {
    match (recipient.0, binding) {
        (
            EffectRecipientSetDef::Objects(ObjectSetDef::One(crate::card::ObjectRefDef::Binding(
                recipient,
            ))),
            ObjectChoiceBindingDef::Object(binding),
        ) => recipient == binding,
        (
            EffectRecipientSetDef::Objects(ObjectSetDef::Binding(recipient)),
            ObjectChoiceBindingDef::Objects(binding)
            | ObjectChoiceBindingDef::OrderedObjects(binding),
        ) => recipient == binding,
        _ => false,
    }
}

pub(super) fn effect_removes_binding(effect: EffectDef, binding: ObjectChoiceBindingDef) -> bool {
    match effect {
        EffectDef::Destroy { object, .. }
        | EffectDef::Sacrifice { object }
        | EffectDef::DiscardCards { object }
        | EffectDef::MoveToZone {
            object,
            zone: ZoneKind::Graveyard | ZoneKind::Exile,
            ..
        } => recipient_uses_binding(object, binding),
        EffectDef::Sequence(effects) => effects
            .iter()
            .copied()
            .any(|effect| effect_removes_binding(effect, binding)),
        EffectDef::Randomized {
            on_success,
            on_failure,
            ..
        } => {
            effect_removes_binding(*on_success, binding)
                || effect_removes_binding(*on_failure, binding)
        }
        EffectDef::Choose(definition) => effect_removes_binding(*definition.then, binding),
        EffectDef::ChooseForEachPlayer(definition) => {
            effect_removes_binding(*definition.then, binding)
        }
        EffectDef::PayOr(definition) => {
            definition
                .if_paid
                .is_some_and(|effect| effect_removes_binding(*effect, binding))
                || definition
                    .otherwise
                    .is_some_and(|effect| effect_removes_binding(*effect, binding))
        }
        EffectDef::May { effect, .. } | EffectDef::ReplaceNextDrawThisTurn { effect, .. } => {
            effect_removes_binding(*effect, binding)
        }
        effect @ (EffectDef::IfCondition { .. } | EffectDef::IfElseCondition { .. }) => {
            let conditional = effect
                .conditional()
                .expect("conditional variants expose their shared shape");
            effect_removes_binding(*conditional.then, binding)
                || conditional
                    .otherwise
                    .is_some_and(|otherwise| effect_removes_binding(*otherwise, binding))
        }
        EffectDef::InstallTrigger(installed) => installed
            .ability
            .declarative_effect()
            .is_some_and(|effect| effect_removes_binding(effect, binding)),
        EffectDef::IfFormat {
            then, otherwise, ..
        } => effect_removes_binding(*then, binding) || effect_removes_binding(*otherwise, binding),
        EffectDef::SacrificeOfChoice {
            then: Some(effect), ..
        } => effect_removes_binding(*effect, binding),
        _ => false,
    }
}

pub(super) fn effect_moves_group_to_hand(
    effect: EffectDef,
    binding: ObjectSetBindingIndex,
) -> bool {
    effect_matches_group_operation(effect, binding, GroupOperation::MoveToHand)
}

pub(super) fn effect_sacrifices_group(effect: EffectDef, binding: ObjectSetBindingIndex) -> bool {
    effect_matches_group_operation(effect, binding, GroupOperation::Sacrifice)
}

#[derive(Clone, Copy)]
enum GroupOperation {
    MoveToHand,
    Sacrifice,
}

fn effect_matches_group_operation(
    effect: EffectDef,
    binding: ObjectSetBindingIndex,
    operation: GroupOperation,
) -> bool {
    let recipient_matches = |recipient: EffectRecipientDef| {
        recipient_uses_binding(recipient, ObjectChoiceBindingDef::Objects(binding))
    };
    match effect {
        EffectDef::MoveToZone {
            object,
            zone: ZoneKind::Hand,
            ..
        } if matches!(operation, GroupOperation::MoveToHand) => recipient_matches(object),
        EffectDef::MoveObjects(definition)
            if matches!(operation, GroupOperation::MoveToHand)
                && definition.zone == ZoneKind::Hand =>
        {
            definition.input == ObjectSetDef::Binding(binding)
        }
        EffectDef::Sacrifice { object } if matches!(operation, GroupOperation::Sacrifice) => {
            recipient_matches(object)
        }
        EffectDef::InstallTrigger(installed) => installed
            .ability
            .declarative_effect()
            .is_some_and(|effect| effect_matches_group_operation(effect, binding, operation)),
        _ => crate::card::child_effects(effect)
            .into_iter()
            .any(|child| effect_matches_group_operation(child, binding, operation)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{ChoiceVisibilityDef, PlayerRefDef};

    fn required_pair(binding: ObjectChoiceBindingDef) -> ChooseDef {
        ChooseDef {
            binding,
            unchosen: None,
            chooser: PlayerRefDef::EffectController,
            candidates: ObjectSetDef::Binding(ObjectSetBindingIndex::PRIMARY),
            exclude: None,
            minimum: 2,
            maximum: 2,
            visibility: ChoiceVisibilityDef::Private,
            then: &EffectDef::None,
        }
    }

    #[test]
    fn a_required_ordered_pair_still_asks_for_its_order() {
        assert!(effect_choice_resolves_automatically(
            required_pair(ObjectChoiceBindingDef::Objects(
                ObjectSetBindingIndex::PRIMARY,
            )),
            2,
        ));
        assert!(!effect_choice_resolves_automatically(
            required_pair(ObjectChoiceBindingDef::OrderedObjects(
                ObjectSetBindingIndex::PRIMARY,
            )),
            2,
        ));
    }
}
