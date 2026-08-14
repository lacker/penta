use super::super::{
    AbilityTargetDef, AbilityTargetPredicate, CardBehavior, DividedTotal, Game, GameObjectId,
    PlayerId, Target, TargetSelection, TargetSlotDef, TargetSlotId, TriggerContext,
    positive_compositions, target_combinations,
};

impl Game {
    pub(in crate::game) fn legacy_target_selections(
        &self,
        behavior: CardBehavior,
        player: PlayerId,
    ) -> Vec<Vec<TargetSelection>> {
        self.legal_target_lists(behavior, player, None)
            .into_iter()
            .map(|targets| {
                if targets.is_empty() {
                    Vec::new()
                } else {
                    vec![TargetSelection::new(TargetSlotId(0), targets)]
                }
            })
            .collect()
    }

    pub(in crate::game) fn legal_target_selections(
        &self,
        slots: &[TargetSlotDef],
        x: u16,
    ) -> Vec<Vec<TargetSelection>> {
        let mut selections = vec![Vec::new()];
        for slot in slots {
            let candidates = self.targets_matching(slot.predicate);
            let mut choices = Vec::new();
            if let Some(total) = slot.divided_total {
                let total = match total {
                    DividedTotal::Fixed(total) => total,
                    DividedTotal::ChosenX => u8::try_from(x).unwrap_or(u8::MAX),
                };
                // Every chosen target takes at least one, so the number of
                // targets follows from how the total is split.
                for count in 1..=usize::from(total).min(candidates.len()) {
                    for targets in target_combinations(&candidates, count) {
                        for amounts in positive_compositions(total, count) {
                            choices.push(TargetSelection::divided(
                                slot.id,
                                targets.clone(),
                                amounts,
                            ));
                        }
                    }
                }
                let mut combined = Vec::new();
                for prefix in &selections {
                    for choice in &choices {
                        let mut selected = prefix.clone();
                        selected.push(choice.clone());
                        combined.push(selected);
                    }
                }
                selections = combined;
                continue;
            }
            for count in slot.minimum..=slot.maximum {
                choices.extend(
                    target_combinations(&candidates, usize::from(count))
                        .into_iter()
                        .map(|targets| TargetSelection::new(slot.id, targets)),
                );
            }
            let mut combined = Vec::new();
            for prefix in &selections {
                for choice in &choices {
                    let mut selected = prefix.clone();
                    selected.push(choice.clone());
                    combined.push(selected);
                }
            }
            selections = combined;
        }
        selections
    }

    pub(in crate::game) fn legal_ability_target_selections(
        &self,
        slots: &[AbilityTargetDef],
        controller: PlayerId,
        source: GameObjectId,
        context: TriggerContext,
        x: u16,
    ) -> Vec<Vec<TargetSelection>> {
        let mut selections = vec![Vec::new()];
        for (index, slot) in slots.iter().enumerate() {
            let id = TargetSlotId::from_index(index)
                .expect("validated ability targets fit the runtime slot space");
            // A slot that reads an earlier slot's choice has to be enumerated
            // once per prefix, because its candidates are different for each.
            if let AbilityTargetPredicate::ControlledByTargetOf {
                object,
                slot: other,
            } = slot.predicate
            {
                let other = TargetSlotId::from_index(other.index())
                    .expect("validated dependent target fits the runtime slot space");
                let mut combined = Vec::new();
                for prefix in &selections {
                    let candidates = prefix
                        .iter()
                        .find(|selection: &&TargetSelection| selection.slot() == other)
                        .and_then(|selection| selection.targets().first().copied())
                        .and_then(|target| match target {
                            Target::Player(player) => Some(player),
                            Target::Permanent(id) | Target::Card(id) | Target::Spell(id) => {
                                self.current_or_last_known_controller(id)
                            }
                        })
                        .map_or_else(Vec::new, |owner| {
                            self.battlefield
                                .iter()
                                .filter(|permanent| permanent.controller == owner)
                                .filter(|permanent| {
                                    self.trigger_object_matches(
                                        object,
                                        &self.trigger_event_object(permanent),
                                        source,
                                        false,
                                    ) && self
                                        .permanent_can_be_targeted_by(permanent, controller, source)
                                })
                                .map(|permanent| Target::Permanent(permanent.card.id))
                                .collect::<Vec<_>>()
                        });
                    for count in slot.minimum..=slot.maximum {
                        for targets in target_combinations(&candidates, usize::from(count)) {
                            let mut selected = prefix.clone();
                            selected.push(TargetSelection::new(id, targets));
                            combined.push(selected);
                        }
                    }
                }
                selections = combined;
                continue;
            }
            let candidates =
                self.ability_targets_matching(slot.predicate, controller, source, context);
            let mut choices = Vec::new();
            if let Some(total) = slot.divided_total {
                let total = match total {
                    DividedTotal::Fixed(total) => total,
                    DividedTotal::ChosenX => u8::try_from(x).unwrap_or(u8::MAX),
                };
                // Every chosen target takes at least one, so the number of
                // targets follows from how the total is split.
                for count in 1..=usize::from(total).min(candidates.len()) {
                    for targets in target_combinations(&candidates, count) {
                        for amounts in positive_compositions(total, count) {
                            choices.push(TargetSelection::divided(id, targets.clone(), amounts));
                        }
                    }
                }
                let mut combined = Vec::new();
                for prefix in &selections {
                    for choice in &choices {
                        let mut selected = prefix.clone();
                        selected.push(choice.clone());
                        combined.push(selected);
                    }
                }
                selections = combined;
                continue;
            }
            for count in slot.minimum..=slot.maximum {
                choices.extend(
                    target_combinations(&candidates, usize::from(count))
                        .into_iter()
                        .map(|targets| TargetSelection::new(id, targets)),
                );
            }
            let mut combined = Vec::new();
            for prefix in &selections {
                for choice in &choices {
                    let mut selected = prefix.clone();
                    selected.push(choice.clone());
                    combined.push(selected);
                }
            }
            selections = combined;
        }
        selections
    }
}
