//! What a single object predicate says about one characteristic.
//!
//! Each of these answers one question -- the colour a predicate demands, the
//! subtype it names, the controller it requires -- by walking the composites
//! the same way. They are separated from the label building they feed because
//! together they are most of a file.

use super::{ManaColor, ObjectPredicateDef, PlayerRelation};

pub(super) fn object_predicate_implies(
    predicate: ObjectPredicateDef,
    expected: ObjectPredicateDef,
) -> bool {
    if predicate == expected {
        return true;
    }
    match predicate {
        ObjectPredicateDef::All(predicates) => predicates
            .iter()
            .copied()
            .any(|predicate| object_predicate_implies(predicate, expected)),
        ObjectPredicateDef::AnyOf(predicates) => {
            !predicates.is_empty()
                && predicates
                    .iter()
                    .copied()
                    .all(|predicate| object_predicate_implies(predicate, expected))
        }
        ObjectPredicateDef::Ability
        | ObjectPredicateDef::ActivatedAbility
        | ObjectPredicateDef::TriggeredAbility
        | ObjectPredicateDef::DeclaredTargetCount { .. }
        | ObjectPredicateDef::HasDeclaredTarget(_)
        | ObjectPredicateDef::HasDeclaredPlayerTarget(_)
        | ObjectPredicateDef::Any
        | ObjectPredicateDef::Source
        | ObjectPredicateDef::Token
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
        | ObjectPredicateDef::AttachedTo(_)
        | ObjectPredicateDef::AttachedToSource
        | ObjectPredicateDef::AttackedThisTurn
        | ObjectPredicateDef::CameUnderControlThisTurn
        | ObjectPredicateDef::EnteredThisTurn
        | ObjectPredicateDef::AttackedDuringControllersLastTurn
        | ObjectPredicateDef::HasType(_)
        | ObjectPredicateDef::NameEquals(_)
        | ObjectPredicateDef::NameIn(_)
        | ObjectPredicateDef::Spell
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
        | ObjectPredicateDef::HasAnyBasicLandType(_)
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
        | ObjectPredicateDef::Not(_)
        | ObjectPredicateDef::Special(_) => false,
    }
}

pub(super) fn predicate_color(predicate: ObjectPredicateDef) -> Option<ManaColor> {
    match predicate {
        ObjectPredicateDef::Color(color) => Some(color),
        ObjectPredicateDef::All(predicates) => predicates.iter().copied().find_map(predicate_color),
        ObjectPredicateDef::Ability
        | ObjectPredicateDef::ActivatedAbility
        | ObjectPredicateDef::TriggeredAbility
        | ObjectPredicateDef::DeclaredTargetCount { .. }
        | ObjectPredicateDef::HasDeclaredTarget(_)
        | ObjectPredicateDef::HasDeclaredPlayerTarget(_)
        | ObjectPredicateDef::Any
        | ObjectPredicateDef::Source
        | ObjectPredicateDef::Token
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
        | ObjectPredicateDef::AttachedTo(_)
        | ObjectPredicateDef::AttachedToSource
        | ObjectPredicateDef::AttackedThisTurn
        | ObjectPredicateDef::CameUnderControlThisTurn
        | ObjectPredicateDef::EnteredThisTurn
        | ObjectPredicateDef::AttackedDuringControllersLastTurn
        | ObjectPredicateDef::HasType(_)
        | ObjectPredicateDef::NameEquals(_)
        | ObjectPredicateDef::NameIn(_)
        | ObjectPredicateDef::Spell
        | ObjectPredicateDef::NoncreatureSpell
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
        | ObjectPredicateDef::HasAnyBasicLandType(_)
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
        | ObjectPredicateDef::AnyOf(_)
        | ObjectPredicateDef::Not(_)
        | ObjectPredicateDef::Special(_) => None,
    }
}

pub(super) fn predicate_color_count(predicate: ObjectPredicateDef) -> Option<u8> {
    match predicate {
        ObjectPredicateDef::ColorCount(count) => Some(count),
        ObjectPredicateDef::All(predicates) => {
            predicates.iter().copied().find_map(predicate_color_count)
        }
        ObjectPredicateDef::Ability
        | ObjectPredicateDef::ActivatedAbility
        | ObjectPredicateDef::TriggeredAbility
        | ObjectPredicateDef::DeclaredTargetCount { .. }
        | ObjectPredicateDef::HasDeclaredTarget(_)
        | ObjectPredicateDef::HasDeclaredPlayerTarget(_)
        | ObjectPredicateDef::Any
        | ObjectPredicateDef::Source
        | ObjectPredicateDef::Token
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
        | ObjectPredicateDef::AttachedTo(_)
        | ObjectPredicateDef::AttachedToSource
        | ObjectPredicateDef::AttackedThisTurn
        | ObjectPredicateDef::CameUnderControlThisTurn
        | ObjectPredicateDef::EnteredThisTurn
        | ObjectPredicateDef::AttackedDuringControllersLastTurn
        | ObjectPredicateDef::HasType(_)
        | ObjectPredicateDef::NameEquals(_)
        | ObjectPredicateDef::NameIn(_)
        | ObjectPredicateDef::Spell
        | ObjectPredicateDef::NoncreatureSpell
        | ObjectPredicateDef::Color(_)
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
        | ObjectPredicateDef::HasAnyBasicLandType(_)
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
        | ObjectPredicateDef::AnyOf(_)
        | ObjectPredicateDef::Not(_)
        | ObjectPredicateDef::Special(_) => None,
    }
}

pub(super) fn predicate_subtype(predicate: ObjectPredicateDef) -> Option<&'static str> {
    match predicate {
        ObjectPredicateDef::Subtype(subtype) => Some(subtype),
        ObjectPredicateDef::All(predicates) => {
            predicates.iter().copied().find_map(predicate_subtype)
        }
        ObjectPredicateDef::Ability
        | ObjectPredicateDef::ActivatedAbility
        | ObjectPredicateDef::TriggeredAbility
        | ObjectPredicateDef::DeclaredTargetCount { .. }
        | ObjectPredicateDef::HasDeclaredTarget(_)
        | ObjectPredicateDef::HasDeclaredPlayerTarget(_)
        | ObjectPredicateDef::NameEquals(_)
        | ObjectPredicateDef::NameIn(_)
        | ObjectPredicateDef::Any
        | ObjectPredicateDef::Source
        | ObjectPredicateDef::Token
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
        | ObjectPredicateDef::AttachedTo(_)
        | ObjectPredicateDef::AttachedToSource
        | ObjectPredicateDef::AttackedThisTurn
        | ObjectPredicateDef::CameUnderControlThisTurn
        | ObjectPredicateDef::EnteredThisTurn
        | ObjectPredicateDef::AttackedDuringControllersLastTurn
        | ObjectPredicateDef::HasType(_)
        | ObjectPredicateDef::Spell
        | ObjectPredicateDef::NoncreatureSpell
        | ObjectPredicateDef::Color(_)
        | ObjectPredicateDef::ColorCount(_)
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
        | ObjectPredicateDef::HasAnyBasicLandType(_)
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
        | ObjectPredicateDef::AnyOf(_)
        | ObjectPredicateDef::Not(_)
        | ObjectPredicateDef::Special(_) => None,
    }
}

pub(super) fn predicate_negated_subtype(predicate: ObjectPredicateDef) -> Option<&'static str> {
    match predicate {
        ObjectPredicateDef::Not(inner) => match *inner {
            ObjectPredicateDef::Subtype(subtype) => Some(subtype),
            _ => None,
        },
        ObjectPredicateDef::All(predicates) => predicates
            .iter()
            .copied()
            .find_map(predicate_negated_subtype),
        ObjectPredicateDef::Ability
        | ObjectPredicateDef::ActivatedAbility
        | ObjectPredicateDef::TriggeredAbility
        | ObjectPredicateDef::DeclaredTargetCount { .. }
        | ObjectPredicateDef::HasDeclaredTarget(_)
        | ObjectPredicateDef::HasDeclaredPlayerTarget(_)
        | ObjectPredicateDef::Any
        | ObjectPredicateDef::Source
        | ObjectPredicateDef::Token
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
        | ObjectPredicateDef::AttachedTo(_)
        | ObjectPredicateDef::AttachedToSource
        | ObjectPredicateDef::AttackedThisTurn
        | ObjectPredicateDef::CameUnderControlThisTurn
        | ObjectPredicateDef::EnteredThisTurn
        | ObjectPredicateDef::AttackedDuringControllersLastTurn
        | ObjectPredicateDef::HasType(_)
        | ObjectPredicateDef::NameEquals(_)
        | ObjectPredicateDef::NameIn(_)
        | ObjectPredicateDef::Spell
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
        | ObjectPredicateDef::HasAnyBasicLandType(_)
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
        | ObjectPredicateDef::AnyOf(_)
        | ObjectPredicateDef::Special(_) => None,
    }
}

pub(super) fn predicate_power_at_least(predicate: ObjectPredicateDef) -> Option<i16> {
    match predicate {
        // An exact power is also a minimum, which is all this reports.
        ObjectPredicateDef::PowerAtLeast(power) | ObjectPredicateDef::PowerExactly(power) => {
            Some(power)
        }
        ObjectPredicateDef::All(predicates) => predicates
            .iter()
            .copied()
            .find_map(predicate_power_at_least),
        ObjectPredicateDef::Ability
        | ObjectPredicateDef::ActivatedAbility
        | ObjectPredicateDef::TriggeredAbility
        | ObjectPredicateDef::DeclaredTargetCount { .. }
        | ObjectPredicateDef::HasDeclaredTarget(_)
        | ObjectPredicateDef::HasDeclaredPlayerTarget(_)
        | ObjectPredicateDef::ToughnessExactly(_)
        | ObjectPredicateDef::TotalPowerAndToughnessAtMost(_)
        | ObjectPredicateDef::ToughnessLessThan(_)
        | ObjectPredicateDef::PowerGreaterThan(_)
        | ObjectPredicateDef::PowerLessThan(_)
        | ObjectPredicateDef::ToughnessGreaterThanItsPower
        | ObjectPredicateDef::ToughnessGreaterThan(_)
        | ObjectPredicateDef::Any
        | ObjectPredicateDef::Source
        | ObjectPredicateDef::Token
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
        | ObjectPredicateDef::AttachedTo(_)
        | ObjectPredicateDef::AttachedToSource
        | ObjectPredicateDef::AttackedThisTurn
        | ObjectPredicateDef::CameUnderControlThisTurn
        | ObjectPredicateDef::EnteredThisTurn
        | ObjectPredicateDef::AttackedDuringControllersLastTurn
        | ObjectPredicateDef::HasType(_)
        | ObjectPredicateDef::NameEquals(_)
        | ObjectPredicateDef::NameIn(_)
        | ObjectPredicateDef::Spell
        | ObjectPredicateDef::NoncreatureSpell
        | ObjectPredicateDef::Color(_)
        | ObjectPredicateDef::ColorCount(_)
        | ObjectPredicateDef::Subtype(_)
        | ObjectPredicateDef::ManaValueAtMost(_)
        | ObjectPredicateDef::GenericManaCostAtMost(_)
        | ObjectPredicateDef::ManaValueEqualTo(_)
        | ObjectPredicateDef::ManaValueAtMostValue(_)
        | ObjectPredicateDef::HasAnyBasicLandType(_)
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
        | ObjectPredicateDef::AnyOf(_)
        | ObjectPredicateDef::Not(_)
        | ObjectPredicateDef::Special(_) => None,
    }
}

pub(super) fn predicate_mana_value_at_most(predicate: ObjectPredicateDef) -> Option<u8> {
    match predicate {
        ObjectPredicateDef::ManaValueAtMost(value) => Some(value),
        ObjectPredicateDef::All(predicates) => predicates
            .iter()
            .copied()
            .find_map(predicate_mana_value_at_most),
        ObjectPredicateDef::Ability
        | ObjectPredicateDef::ActivatedAbility
        | ObjectPredicateDef::TriggeredAbility
        | ObjectPredicateDef::DeclaredTargetCount { .. }
        | ObjectPredicateDef::HasDeclaredTarget(_)
        | ObjectPredicateDef::HasDeclaredPlayerTarget(_)
        | ObjectPredicateDef::Any
        | ObjectPredicateDef::Source
        | ObjectPredicateDef::Token
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
        | ObjectPredicateDef::AttachedTo(_)
        | ObjectPredicateDef::AttachedToSource
        | ObjectPredicateDef::AttackedThisTurn
        | ObjectPredicateDef::CameUnderControlThisTurn
        | ObjectPredicateDef::EnteredThisTurn
        | ObjectPredicateDef::AttackedDuringControllersLastTurn
        | ObjectPredicateDef::HasType(_)
        | ObjectPredicateDef::NameEquals(_)
        | ObjectPredicateDef::NameIn(_)
        | ObjectPredicateDef::Spell
        | ObjectPredicateDef::NoncreatureSpell
        | ObjectPredicateDef::Color(_)
        | ObjectPredicateDef::ColorCount(_)
        | ObjectPredicateDef::Subtype(_)
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
        | ObjectPredicateDef::HasAnyBasicLandType(_)
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
        | ObjectPredicateDef::AnyOf(_)
        | ObjectPredicateDef::Not(_)
        | ObjectPredicateDef::GenericManaCostAtMost(_)
        | ObjectPredicateDef::Special(_) => None,
    }
}

pub(super) fn predicate_controller(predicate: ObjectPredicateDef) -> Option<PlayerRelation> {
    match predicate {
        ObjectPredicateDef::ControlledBy(controller) => Some(controller),
        ObjectPredicateDef::All(predicates) => {
            predicates.iter().copied().find_map(predicate_controller)
        }
        ObjectPredicateDef::Ability
        | ObjectPredicateDef::ActivatedAbility
        | ObjectPredicateDef::TriggeredAbility
        | ObjectPredicateDef::DeclaredTargetCount { .. }
        | ObjectPredicateDef::HasDeclaredTarget(_)
        | ObjectPredicateDef::HasDeclaredPlayerTarget(_)
        // Owning a card is not controlling it, so it names no controller.
        | ObjectPredicateDef::OwnedBy(_)
        | ObjectPredicateDef::Any
        | ObjectPredicateDef::Source
        | ObjectPredicateDef::Token
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
        | ObjectPredicateDef::AttachedTo(_)
        | ObjectPredicateDef::AttachedToSource
        | ObjectPredicateDef::AttackedThisTurn
        | ObjectPredicateDef::CameUnderControlThisTurn
        | ObjectPredicateDef::EnteredThisTurn
        | ObjectPredicateDef::AttackedDuringControllersLastTurn
        | ObjectPredicateDef::HasType(_)
        | ObjectPredicateDef::NameEquals(_)
        | ObjectPredicateDef::NameIn(_)
        | ObjectPredicateDef::Spell
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
        | ObjectPredicateDef::HasAnyBasicLandType(_)
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
        | ObjectPredicateDef::AnyOf(_)
        | ObjectPredicateDef::Not(_)
        | ObjectPredicateDef::Special(_) => None,
    }
}
