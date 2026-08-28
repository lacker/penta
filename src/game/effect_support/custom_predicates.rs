impl Game {
    pub(super) fn ability_target_uses_custom_predicate(predicate: AbilityTargetPredicate) -> bool {
        match predicate {
            AbilityTargetPredicate::AnyTarget
            | AbilityTargetPredicate::PlayerOrPlaneswalker(_)
            | AbilityTargetPredicate::ControlledByTargetOf { .. }
            | AbilityTargetPredicate::OwnedByTargetPlayer { .. }
            | AbilityTargetPredicate::PlayerWithMoreObjectsThanChooser { .. }
            | AbilityTargetPredicate::Player(_) => false,
            AbilityTargetPredicate::AnyOf(predicates) => predicates
                .iter()
                .copied()
                .any(Self::ability_target_uses_custom_predicate),
            AbilityTargetPredicate::Object { object, .. }
            | AbilityTargetPredicate::StackObject { object, .. } => {
                Self::object_predicate_uses_custom_predicate(object)
            }
        }
    }

    pub(super) fn object_predicate_uses_custom_predicate(predicate: ObjectPredicateDef) -> bool {
        match predicate {
            ObjectPredicateDef::Special(_) => true,
            ObjectPredicateDef::All(predicates) | ObjectPredicateDef::AnyOf(predicates) => {
                predicates
                    .iter()
                    .any(|predicate| Self::object_predicate_uses_custom_predicate(*predicate))
            }
            ObjectPredicateDef::Not(predicate) | ObjectPredicateDef::AttachedTo(predicate) => {
                Self::object_predicate_uses_custom_predicate(*predicate)
            }
            ObjectPredicateDef::Any
            | ObjectPredicateDef::Source
            | ObjectPredicateDef::Token
            | ObjectPredicateDef::HasType(_)
            | ObjectPredicateDef::HasAnyBasicLandType(_)
            | ObjectPredicateDef::Spell
            | ObjectPredicateDef::HasSourcesChosenScalar(_)
            | ObjectPredicateDef::TargetsObjectMatching(_)
            | ObjectPredicateDef::NoncreatureSpell
            | ObjectPredicateDef::Color(_)
            | ObjectPredicateDef::ColorCount(_)
            | ObjectPredicateDef::Subtype(_)
            | ObjectPredicateDef::Named(_)
        | ObjectPredicateDef::HasChosenName
            | ObjectPredicateDef::ManaValueAtMost(_)
            | ObjectPredicateDef::ManaValueEqualTo(_)
            | ObjectPredicateDef::ManaValueAtMostValue(_)
            | ObjectPredicateDef::PowerAtLeast(_)
            | ObjectPredicateDef::PowerExactly(_)
            | ObjectPredicateDef::ToughnessExactly(_)
        | ObjectPredicateDef::TotalPowerAndToughnessAtMost(_)
            | ObjectPredicateDef::ToughnessLessThan(_)
            | ObjectPredicateDef::PowerGreaterThan(_)
            | ObjectPredicateDef::PowerLessThan(_)
            | ObjectPredicateDef::ToughnessGreaterThanItsPower
            | ObjectPredicateDef::ToughnessGreaterThan(_)
            | ObjectPredicateDef::ControlledBy(_)
            | ObjectPredicateDef::OwnedBy(_)
            | ObjectPredicateDef::Supertype(_)
            | ObjectPredicateDef::DebutSet(_)
            | ObjectPredicateDef::HasName(_)
            | ObjectPredicateDef::AttackingOrBlocking
            | ObjectPredicateDef::Tapped
        | ObjectPredicateDef::WasDealtDamageThisTurn
        | ObjectPredicateDef::DealtDamageThisTurn
            | ObjectPredicateDef::Attacking
            | ObjectPredicateDef::Saddled
            | ObjectPredicateDef::Blocking
            | ObjectPredicateDef::BlockedBySource
            | ObjectPredicateDef::BlockingSource
            | ObjectPredicateDef::BandedWithSource
        | ObjectPredicateDef::Unpaired
        | ObjectPredicateDef::PairedWithSource
            | ObjectPredicateDef::Enchanted
            | ObjectPredicateDef::AttachedToSource
            | ObjectPredicateDef::AttackedThisTurn
            | ObjectPredicateDef::CameUnderControlThisTurn
            | ObjectPredicateDef::EnteredThisTurn
            | ObjectPredicateDef::AttackedDuringControllersLastTurn
            | ObjectPredicateDef::HasKeyword(_)
            | ObjectPredicateDef::HasAbility(_)
            | ObjectPredicateDef::HasCounter(_)
            | ObjectPredicateDef::HasAnyCounter
            | ObjectPredicateDef::CounterCount { .. }
            | ObjectPredicateDef::HasNonManaActivatedAbility => false,
        }
    }
}

/// One comparison, so a condition reads the same however it is counted.
pub(super) fn compare<T: Ord>(left: &T, comparison: ComparisonDef, right: &T) -> bool {
    match comparison {
        ComparisonDef::Less => left < right,
        ComparisonDef::LessOrEqual => left <= right,
        ComparisonDef::Equal => left == right,
        ComparisonDef::GreaterOrEqual => left >= right,
        ComparisonDef::Greater => left > right,
    }
}

#[cfg(test)]
mod tests {
    use super::compare;
    use crate::ComparisonDef;

    #[test]
    fn comparisons_follow_their_ordering_semantics() {
        assert!(compare(&1, ComparisonDef::Less, &2));
        assert!(compare(&2, ComparisonDef::LessOrEqual, &2));
        assert!(compare(&2, ComparisonDef::Equal, &2));
        assert!(compare(&2, ComparisonDef::GreaterOrEqual, &2));
        assert!(compare(&3, ComparisonDef::Greater, &2));

        assert!(!compare(&2, ComparisonDef::Less, &2));
        assert!(!compare(&3, ComparisonDef::LessOrEqual, &2));
        assert!(!compare(&3, ComparisonDef::Equal, &2));
        assert!(!compare(&1, ComparisonDef::GreaterOrEqual, &2));
        assert!(!compare(&2, ComparisonDef::Greater, &2));
    }
}
