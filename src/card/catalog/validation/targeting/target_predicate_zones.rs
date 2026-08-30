// Zone questions shared by target-shape validation. Conditional predicates
// have to be safe on every cast branch because the surrounding effect shape
// is one declarative program.

fn target_predicate_may_name_nonbattlefield_object(
    predicate: AbilityTargetPredicate,
) -> bool {
    match predicate {
        AbilityTargetPredicate::IfAdditionalCostPaid {
            if_paid, otherwise, ..
        } => {
            target_predicate_may_name_nonbattlefield_object(*if_paid)
                || target_predicate_may_name_nonbattlefield_object(*otherwise)
        }
        AbilityTargetPredicate::AnyOf(predicates) => predicates
            .iter()
            .copied()
            .any(target_predicate_may_name_nonbattlefield_object),
        AbilityTargetPredicate::Object { zones, .. } => zones
            .iter()
            .any(|zone| *zone != ZoneKind::Battlefield),
        _ => false,
    }
}

fn target_predicate_zones_support_flashback(predicate: AbilityTargetPredicate) -> bool {
    match predicate {
        AbilityTargetPredicate::IfAdditionalCostPaid {
            if_paid, otherwise, ..
        } => {
            target_predicate_zones_support_flashback(*if_paid)
                && target_predicate_zones_support_flashback(*otherwise)
        }
        AbilityTargetPredicate::AnyOf(predicates) => {
            !predicates.is_empty()
                && predicates
                    .iter()
                    .copied()
                    .all(target_predicate_zones_support_flashback)
        }
        AbilityTargetPredicate::Object { zones, .. } => zones_support_flashback(zones),
        _ => false,
    }
}

fn zones_support_flashback(zones: &[ZoneKind]) -> bool {
    zones
        .iter()
        .all(|zone| matches!(zone, ZoneKind::Battlefield | ZoneKind::Graveyard))
}
