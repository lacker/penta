//! Core card-schema types grouped by rules concept.
//!
//! This private facade preserves the established paths used by the rest of
//! the card module while keeping each concept in a focused implementation
//! file.

mod ability;
mod ability_kinds;
mod characteristics;
mod composition;
mod costs;
mod effects;
mod emblem;
mod face_down;
mod identity;
mod inline_rules;
mod mana_cost;
mod payments;
mod prepared;
mod presentation;
mod presentation_predicates;
mod rules;
mod rules_primitives;
mod targeting;
mod token;

pub use ability::*;
pub use ability_kinds::*;
pub use characteristics::*;
pub use composition::*;
pub use costs::*;
pub use effects::*;
pub use emblem::*;
pub use face_down::*;
pub use identity::*;
pub use mana_cost::*;
pub use payments::*;
pub use prepared::*;
pub use presentation::*;
pub use rules::*;
pub use rules_primitives::*;
pub use targeting::*;
pub use token::*;

/// Builds a [`ManaCost`] from canonical braced symbols and validates the
/// literal at compile time.
///
/// ```
/// # use penta::{ManaCost, mana_cost};
/// const COST: ManaCost = mana_cost!("{2}{G}{G}");
/// assert_eq!(COST.generic, 2);
/// assert_eq!(COST.green, 2);
/// ```
///
/// ```compile_fail
/// # use penta::{ManaCost, mana_cost};
/// const COST: ManaCost = mana_cost!("2GG");
/// ```
#[macro_export]
macro_rules! mana_cost {
    ($symbols:literal) => {{
        const COST: $crate::ManaCost = match $crate::ManaCost::parse_symbols($symbols) {
            Ok(cost) => cost,
            Err(_) => panic!("invalid mana cost literal"),
        };
        COST
    }};
}

/// Builds a resolution-time conditional value without exposing the referenced
/// composite that keeps [`ValueDef`] compact.
#[macro_export]
macro_rules! value_if_condition {
    ($condition:expr, $then:expr, $otherwise:expr) => {
        $crate::ValueDef::IfCondition(&$crate::card::ConditionValueDef {
            condition: $condition,
            then: $then,
            otherwise: $otherwise,
        })
    };
}

#[cfg(test)]
mod tests;
