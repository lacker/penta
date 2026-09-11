use super::names::{shared_card_name, shared_card_name_set};
use super::*;

pub(in crate::card::sets::tests) fn shared_object_predicate(predicate: ObjectPredicateDef) -> bool {
    match predicate {
        ObjectPredicateDef::All(predicates) | ObjectPredicateDef::AnyOf(predicates) => {
            predicates.iter().copied().all(shared_object_predicate)
        }
        ObjectPredicateDef::Not(predicate) | ObjectPredicateDef::AttachedTo(predicate) => {
            shared_object_predicate(*predicate)
        }
        ObjectPredicateDef::NameEquals(name) => shared_card_name(name),
        ObjectPredicateDef::NameIn(names) => shared_card_name_set(*names),
        ObjectPredicateDef::Special(_) => false,
        ObjectPredicateDef::Any
        | ObjectPredicateDef::Source
        | ObjectPredicateDef::Commander
        | ObjectPredicateDef::Token
        | ObjectPredicateDef::HasType(_)
        | ObjectPredicateDef::HasAnyBasicLandType(_)
        | ObjectPredicateDef::Spell
        | ObjectPredicateDef::Ability
        | ObjectPredicateDef::ActivatedAbility
        | ObjectPredicateDef::TriggeredAbility
        | ObjectPredicateDef::DeclaredTargetCount { .. }
        | ObjectPredicateDef::HasDeclaredTarget(_)
        | ObjectPredicateDef::HasDeclaredPlayerTarget(_)
        | ObjectPredicateDef::NoncreatureSpell
        | ObjectPredicateDef::Color(_)
        | ObjectPredicateDef::ColorCount(_)
        | ObjectPredicateDef::Subtype(_)
        | ObjectPredicateDef::ManaValueAtMost(_)
        | ObjectPredicateDef::GenericManaCostAtMost(_)
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
        | ObjectPredicateDef::WasDealtDamageThisTurn
        | ObjectPredicateDef::DealtDamageThisTurn
        | ObjectPredicateDef::ControlledBy(_)
        | ObjectPredicateDef::OwnedBy(_)
        | ObjectPredicateDef::Supertype(_)
        | ObjectPredicateDef::DebutSet(_)
        | ObjectPredicateDef::HasSourcesChosenScalar(_)
        | ObjectPredicateDef::TargetsObjectMatching(_)
        | ObjectPredicateDef::AttackingOrBlocking
        | ObjectPredicateDef::HasKeyword(_)
        | ObjectPredicateDef::HasAbility(_)
        | ObjectPredicateDef::HasCounter(_)
        | ObjectPredicateDef::HasAnyCounter
        | ObjectPredicateDef::CounterCount { .. }
        | ObjectPredicateDef::HasNonManaActivatedAbility
        | ObjectPredicateDef::Tapped
        | ObjectPredicateDef::Attacking
        | ObjectPredicateDef::Saddled
        | ObjectPredicateDef::Blocking
        | ObjectPredicateDef::AttachedToSource
        | ObjectPredicateDef::BlockedBySource
        | ObjectPredicateDef::BlockingSource
        | ObjectPredicateDef::BandedWithSource
        | ObjectPredicateDef::Unpaired
        | ObjectPredicateDef::PairedWithSource
        | ObjectPredicateDef::Enchanted
        | ObjectPredicateDef::AttackedThisTurn
        | ObjectPredicateDef::CameUnderControlThisTurn
        | ObjectPredicateDef::EnteredThisTurn
        | ObjectPredicateDef::AttackedDuringControllersLastTurn => true,
    }
}
