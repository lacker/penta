//! Shared eligibility and whole-selection validation for semantic object costs.

use crate::card::{AggregateOperationDef, CostDef, CostQuantityDef, ObjectSetValueDef, ZoneKind};
use crate::game::{Game, GameObjectId, PlayerId};

impl Game {
    /// Source identity is the caller's source, not the candidate being tested.
    /// Casting excludes its announced spell separately; resolving costs may
    /// legitimately select their own source.
    pub(in crate::game) fn object_cost_candidates(
        &self,
        player: PlayerId,
        source: GameObjectId,
        cost: CostDef,
    ) -> Vec<GameObjectId> {
        let Some((predicate, from, _)) = cost.object_selection() else {
            return Vec::new();
        };
        if from == ZoneKind::Battlefield {
            return self
                .battlefield
                .iter()
                .filter(|permanent| {
                    permanent.controller == player
                        && (!matches!(cost, CostDef::Tap { .. }) || !permanent.tapped)
                        && self.trigger_object_matches(
                            predicate,
                            &self.trigger_event_object(permanent),
                            source,
                            false,
                        )
                })
                .map(|permanent| permanent.card.id)
                .collect();
        }
        let cards = match from {
            ZoneKind::Hand => &self.players[player.index()].hand,
            ZoneKind::Graveyard => &self.players[player.index()].graveyard,
            _ => return Vec::new(),
        };
        cards
            .iter()
            .filter(|card| self.card_object_matches(predicate, card, from, source))
            .map(|card| card.id)
            .collect()
    }

    pub(in crate::game) fn object_selection_is_valid(
        &self,
        candidates: &[GameObjectId],
        selected: &[GameObjectId],
        quantity: CostQuantityDef,
    ) -> bool {
        selected
            .iter()
            .enumerate()
            .all(|(index, id)| candidates.contains(id) && !selected[..index].contains(id))
            && match quantity {
                CostQuantityDef::ObjectSetValueAtLeast(requirement) => {
                    self.object_set_value(selected, requirement.value)
                        >= i32::from(requirement.minimum)
                }
                quantity => quantity
                    .fixed_value()
                    .is_some_and(|count| selected.len() == usize::from(count)),
            }
    }

    pub(in crate::game) fn object_selection_is_payable(
        &self,
        candidates: &[GameObjectId],
        quantity: CostQuantityDef,
    ) -> bool {
        let CostQuantityDef::ObjectSetValueAtLeast(requirement) = quantity else {
            return quantity
                .fixed_value()
                .is_some_and(|count| candidates.len() >= usize::from(count));
        };
        let maximum = match requirement.value {
            ObjectSetValueDef::Aggregate {
                operation: AggregateOperationDef::Minimum | AggregateOperationDef::Maximum,
                ..
            } => candidates
                .iter()
                .map(|id| self.object_set_value(&[*id], requirement.value))
                .max()
                .unwrap_or(0),
            ObjectSetValueDef::Aggregate {
                operation: AggregateOperationDef::Sum,
                ..
            } => candidates
                .iter()
                .map(|id| self.object_set_value(&[*id], requirement.value).max(0))
                .fold(0_i32, i32::saturating_add),
            ObjectSetValueDef::CardTypeCount => {
                self.object_set_value(candidates, requirement.value)
            }
        };
        maximum >= i32::from(requirement.minimum)
    }

    /// Commit one validated hand/graveyard action as a batch. Discard remains
    /// discard (with its replacements and events), not generic zone movement.
    pub(in crate::game) fn pay_object_card_cost(
        &mut self,
        player: PlayerId,
        cost: CostDef,
        selected: &[GameObjectId],
    ) -> Vec<GameObjectId> {
        match cost {
            CostDef::Discard { .. } => {
                self.discard_cards_with_cause(
                    player,
                    selected,
                    crate::game::ZoneMoveCause::Effect { controller: player },
                );
                Vec::new()
            }
            CostDef::Exile {
                from: ZoneKind::Hand | ZoneKind::Graveyard,
                ..
            } => {
                let (_, from, _) = cost.object_selection().expect("an object cost");
                let mut moved = Vec::new();
                for id in selected {
                    if let Some((card, ZoneKind::Exile)) = self.move_nonbattlefield_card(
                        *id,
                        from,
                        ZoneKind::Exile,
                        crate::game::ZoneMoveCause::Effect { controller: player },
                        None,
                        false,
                    ) {
                        moved.push(card);
                    }
                }
                if !moved.is_empty() {
                    self.capture_cards_exiled(&moved, from);
                }
                moved.into_iter().map(|card| card.id).collect()
            }
            _ => unreachable!("only hand/graveyard object costs use this commit path"),
        }
    }
}
