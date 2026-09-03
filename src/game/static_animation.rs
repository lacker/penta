//! Recipient stratification for static characteristic transformations.
//!
//! "All Forests are 1/1 creatures that are still lands" has to keep applying
//! as Forests come and go. The unified characteristic IR represents that as
//! independent card-type and power/toughness operations, and the ordinary
//! continuous-effect layer walkers derive those operations live.
//!
//! The recipient vocabulary remains narrow: it may ask about land types, the
//! card types below the operation being assembled, subtypes, attachment,
//! which object is the source, and control. A layer-4 transformation may also
//! ask about Creature because CR 613.6 pins a compound animation's recipient
//! set when that component starts to apply; its later components do not
//! reselect after it has supplied that type itself. A colour-only
//! transformation gets no such exception. A basic land subtype is excluded
//! the other way because layer-4 operations supply those. `runtime_support`
//! uses this same boundary, so a card that needs more is blocked rather than
//! silently misread. `card::catalog::validation` keeps a matching list for the
//! catalog-time refusal; the two are meant to say the same thing.

use super::{BasicLandType, CardType, Game, ObjectPredicateDef};
use crate::card::{CardNameDef, CardNameSetDef, ObjectRefDef};

/// Whether a subtype name is one a static effect can itself supply. Basic
/// land subtypes are: the layer-4 basic-land-type operations set and remove
/// them, so a static animation asking about one could read what another
/// animation just wrote. Every other subtype is inert here.
fn subtype_is_supplied_by_a_static_effect(name: &str) -> bool {
    BasicLandType::ALL
        .iter()
        .any(|land_type| land_type.subtype() == name)
}

impl Game {
    fn static_animation_predicate_is_supported_with_creature(
        predicate: ObjectPredicateDef,
        creature: bool,
    ) -> bool {
        match predicate {
            ObjectPredicateDef::Subtype(name) => !subtype_is_supplied_by_a_static_effect(name),
            ObjectPredicateDef::NameEquals(
                CardNameDef::Literal(_)
                | CardNameDef::SourceChoice
                | CardNameDef::NameOf(ObjectRefDef::Source | ObjectRefDef::AttachedToSource),
            )
            | ObjectPredicateDef::NameIn(CardNameSetDef::BasicLandNames) => true,
            ObjectPredicateDef::Any
            | ObjectPredicateDef::Source
            | ObjectPredicateDef::AttachedToSource
            | ObjectPredicateDef::HasSourcesChosenScalar(_)
            | ObjectPredicateDef::HasAnyBasicLandType(_)
            | ObjectPredicateDef::HasType(
                CardType::Land | CardType::Enchantment | CardType::Artifact,
            ) => true,
            ObjectPredicateDef::HasType(CardType::Creature) => creature,
            ObjectPredicateDef::All(predicates) | ObjectPredicateDef::AnyOf(predicates) => {
                predicates.iter().copied().all(|predicate| {
                    Self::static_animation_predicate_is_supported_with_creature(predicate, creature)
                })
            }
            ObjectPredicateDef::Not(predicate) => {
                Self::static_animation_predicate_is_supported_with_creature(*predicate, creature)
            }
            _ => false,
        }
    }

    /// Whether a static colour transformation's recipient predicate stays
    /// inside the stratified vocabulary above.
    #[must_use]
    pub fn static_animation_predicate_is_supported(predicate: ObjectPredicateDef) -> bool {
        Self::static_animation_predicate_is_supported_with_creature(predicate, false)
    }

    /// The layer-4 variant may select noncreatures because CR 613.6 keeps that
    /// selection for the compound effect's later components.
    #[must_use]
    pub fn static_type_animation_predicate_is_supported(predicate: ObjectPredicateDef) -> bool {
        Self::static_animation_predicate_is_supported_with_creature(predicate, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static NONCREATURE_ARTIFACT: ObjectPredicateDef = ObjectPredicateDef::All(&[
        ObjectPredicateDef::AttachedToSource,
        ObjectPredicateDef::HasType(CardType::Artifact),
        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
    ]);

    #[test]
    fn only_a_type_layer_transformation_may_pin_a_noncreature_selection() {
        assert!(Game::static_type_animation_predicate_is_supported(
            NONCREATURE_ARTIFACT,
        ));
        assert!(
            !Game::static_animation_predicate_is_supported(NONCREATURE_ARTIFACT),
            "a later colour-only transformation has no layer-4 selection to keep",
        );
    }
}
