use super::{Game, GameObjectId, PlayerId, RetiredObject, TriggerContext, ValueDef};
use crate::card::PlayerRelation;

impl Game {
    /// The values a predicate can read while matching, where the only context
    /// is the ability's source. Anything wider stays outside the boundary.
    pub(in crate::game) fn value_from_source(
        &self,
        value: ValueDef,
        source: GameObjectId,
    ) -> Option<i32> {
        match value {
            ValueDef::Constant(amount) => Some(amount),
            ValueDef::AggregateObjectValues(aggregate) => {
                let crate::card::ObjectSetDef::Query(query) = aggregate.objects else {
                    return None;
                };
                let controller = self.current_or_last_known_controller(source)?;
                let objects =
                    self.objects_matching_query(query, controller, source, TriggerContext::empty());
                Some(self.aggregate_object_values(objects, aggregate.select, aggregate.operation))
            }
            ValueDef::CountersOnSource(kind) => {
                Some(i32::from(self.current_or_last_known_counters(source, kind)))
            }
            // A spell is its own source, so its chosen X is right there --
            // by way of the retired record, because a spell leaves the stack
            // before its effect runs. An activated ability's source is the
            // permanent instead, and its X is not reachable from a predicate.
            ValueDef::ChosenX => self
                .stack
                .iter()
                .find(|object| object.id == source)
                .map(|object| i32::from(object.x()))
                .or_else(|| match self.retired_objects.get(&source) {
                    Some(RetiredObject::Stack(object)) => Some(i32::from(object.x())),
                    Some(RetiredObject::Card(_) | RetiredObject::Permanent { .. }) | None => None,
                })
                // A spell being cast has no stack object yet, and "target
                // creature with power X or less" has to be answered before
                // it gets one.
                .or_else(|| self.prospective_x.get().map(i32::from)),
            // Converge, reached the same way and for the same reason: the
            // spell is its own source, and it has left the stack by the time
            // its effect asks what paid for it.
            ValueDef::ColorsOfManaSpent => self
                .stack
                .iter()
                .find(|object| object.id == source)
                .map(|object| i32::from(object.colors_spent_count()))
                .or_else(|| match self.retired_objects.get(&source) {
                    Some(RetiredObject::Stack(object)) => {
                        Some(i32::from(object.colors_spent_count()))
                    }
                    Some(RetiredObject::Card(_) | RetiredObject::Permanent { .. }) | None => None,
                }),
            // Pumping the source widens the predicate while it is live. If
            // source and triggering object leave simultaneously, use the
            // same last-known power frozen for the rest of the trigger.
            ValueDef::SourcePower => self.current_or_last_known_power(source).map(i32::from),
            ValueDef::SourceToughness => {
                self.current_or_last_known_toughness(source).map(i32::from)
            }
            ValueDef::CardsInHandAbove { player, threshold } => {
                let controller = self.current_or_last_known_controller(source)?;
                let counted = if player == PlayerRelation::ControllerOfAttachedPermanent {
                    self.attached_host_controller_of(source)
                        .unwrap_or(controller)
                } else {
                    [PlayerId::One, PlayerId::Two]
                        .into_iter()
                        .find(|candidate| {
                            self.player_relation_matches(
                                *candidate,
                                player,
                                controller,
                                TriggerContext::empty(),
                            )
                        })
                        .unwrap_or(controller)
                };
                Some(
                    i32::try_from(
                        self.players[counted.index()]
                            .hand
                            .len()
                            .saturating_sub(usize::from(threshold)),
                    )
                    .unwrap_or(i32::MAX),
                )
            }
            // The X its own spell was cast for, which the permanent recorded
            // as it arrived. "Target artifact with mana value X or less" is
            // a predicate rather than an effect, and by the time it is asked
            // the spell is a permanent.
            ValueDef::SourceCastX => self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == source)
                .map(|permanent| i32::from(permanent.cast.as_ref().map_or(0, |cast| cast.x)))
                .or_else(|| match self.retired_objects.get(&source) {
                    Some(RetiredObject::Permanent { permanent, .. }) => {
                        Some(i32::from(permanent.cast.as_ref().map_or(0, |cast| cast.x)))
                    }
                    Some(RetiredObject::Card(_) | RetiredObject::Stack(_)) | None => None,
                }),
            ValueDef::AdditionalCostPayments(cost) => Some(i32::from(
                self.source_additional_cost_payments(source, cost),
            )),
            ValueDef::IfAdditionalCostPaid(conditional) => {
                let selected = if self.source_additional_cost_payments(source, conditional.cost) > 0
                {
                    conditional.if_paid
                } else {
                    conditional.otherwise
                };
                self.value_from_source(selected, source)
            }
            // "Power X or less" is said as "below X plus one", so a sum has
            // to be reachable from here as well as its parts.
            ValueDef::Sum(sum) => self
                .value_from_source(sum.left, source)
                .zip(self.value_from_source(sum.right, source))
                .map(|(left, right)| left.saturating_add(right)),
            _ => None,
        }
    }
}
