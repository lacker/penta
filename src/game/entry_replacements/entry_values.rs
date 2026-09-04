//! Values evaluated from a permanent's prospective battlefield entry.

use super::super::{CastContext, Game, Permanent, Target, TriggerContext, ValueDef};
use crate::card::ObjectSetDef;

pub(super) fn entry_value(game: &Game, permanent: &Permanent, value: ValueDef) -> Option<i32> {
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
        ValueDef::CountMatchingObjects(query) => Some(
            i32::try_from(
                game.objects_matching_query(
                    *query,
                    permanent.controller,
                    permanent.card.id,
                    TriggerContext::empty(),
                )
                .len(),
            )
            .unwrap_or(i32::MAX),
        ),
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
            )
        }
        ValueDef::Negate(value) => entry_value(game, permanent, *value)?.checked_neg(),
        ValueDef::Scaled(scaled) => {
            entry_value(game, permanent, scaled.value)?.checked_mul(scaled.factor)
        }
        ValueDef::Sum(sum) => entry_value(game, permanent, sum.left)?
            .checked_add(entry_value(game, permanent, sum.right)?),
        ValueDef::Halved(halved) => Some(halved.apply(entry_value(game, permanent, halved.value)?)),
        ValueDef::Quotient(quotient) => Some(quotient.apply(
            entry_value(game, permanent, quotient.numerator)?,
            entry_value(game, permanent, quotient.denominator)?,
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
