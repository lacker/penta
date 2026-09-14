//! Values evaluated from a permanent's prospective battlefield entry.

use super::super::{CastContext, Game, Permanent, Target, TriggerContext, ValueDef};
use crate::card::{ObjectSetDef, ZoneKind};

pub(super) fn entry_value(
    game: &Game,
    permanent: &Permanent,
    value: ValueDef,
    from: Option<ZoneKind>,
) -> Option<i32> {
    match value {
        ValueDef::Constant(value) => Some(value),
        ValueDef::SourceCastX => Some(i32::from(permanent.cast.as_ref().map_or(0, |cast| cast.x))),
        ValueDef::ColorsOfManaSpent => Some(i32::from(
            permanent
                .cast
                .as_ref()
                .map_or(0, CastContext::colors_spent_count),
        )),
        ValueDef::AdditionalCostPayments(index) => Some(i32::from(
            permanent
                .cast
                .as_ref()
                .and_then(|cast| cast.additional_costs.get(index.index()))
                .copied()
                .unwrap_or_default(),
        )),
        ValueDef::CountObjects(objects) => {
            Some(i32::try_from(entry_objects(game, permanent, *objects)?.len()).unwrap_or(i32::MAX))
        }
        ValueDef::CardTypesAmongObjects(objects) => {
            Some(game.card_types_among_targets(&entry_objects(game, permanent, *objects)?))
        }
        ValueDef::CountMatchingObjects(query) => {
            let objects = game.objects_matching_query(
                *query,
                permanent.controller,
                permanent.card.id,
                TriggerContext::empty(),
            );
            // An entry is still prospective: the detached card remains part of
            // its old zone for this count until the move commits (CR 614.12).
            let include_arriving = from.is_some_and(|zone| {
                query.zones.contains(&zone)
                    && query.relative_position.is_none()
                    && !objects.contains(&Target::Card(permanent.card.id))
                    && permanent.card.clone().into_card().is_some_and(|card| {
                        game.query_player_constraints_match(
                            None,
                            card.owner,
                            *query,
                            (permanent.controller, card.id),
                            TriggerContext::empty(),
                            None,
                        ) && game.card_object_matches(query.object, &card, zone, card.id)
                    })
            });
            Some(i32::try_from(objects.len() + usize::from(include_arriving)).unwrap_or(i32::MAX))
        }
        ValueDef::IfAdditionalCostPaid(conditional) => {
            let paid = permanent
                .cast
                .as_ref()
                .and_then(|cast| cast.additional_costs.get(conditional.cost.index()))
                .copied()
                .unwrap_or_default();
            entry_value(
                game,
                permanent,
                if paid > 0 {
                    conditional.if_paid
                } else {
                    conditional.otherwise
                },
                from,
            )
        }
        ValueDef::Negate(value) => entry_value(game, permanent, *value, from)?.checked_neg(),
        ValueDef::Scaled(scaled) => {
            entry_value(game, permanent, scaled.value, from)?.checked_mul(scaled.factor)
        }
        ValueDef::Sum(sum) => entry_value(game, permanent, sum.left, from)?
            .checked_add(entry_value(game, permanent, sum.right, from)?),
        ValueDef::Halved(halved) => {
            Some(halved.apply(entry_value(game, permanent, halved.value, from)?))
        }
        ValueDef::Quotient(quotient) => Some(quotient.apply(
            entry_value(game, permanent, quotient.numerator, from)?,
            entry_value(game, permanent, quotient.denominator, from)?,
        )),
        _ => None,
    }
}

fn entry_objects(game: &Game, permanent: &Permanent, objects: ObjectSetDef) -> Option<Vec<Target>> {
    match objects {
        ObjectSetDef::LinkedExiles => Some(
            game.linked_exile_ids_with_cast(permanent.card.id, permanent.cast.as_ref())
                .into_iter()
                .filter(|id| game.card_in_nonbattlefield_zone(*id).is_some())
                .map(Target::Card)
                .collect(),
        ),
        ObjectSetDef::Matching {
            objects,
            object: predicate,
        } => Some(
            entry_objects(game, permanent, *objects)?
                .into_iter()
                .filter(|target| {
                    game.bound_object_matches(*target, predicate.predicate(), permanent.card.id)
                })
                .collect(),
        ),
        _ => None,
    }
}
