//! Game actions shared by resolving instructions and payment obligations.
//!
//! The program owns selection vocabulary and action identity. Its caller owns
//! the execution contract: resolution does as much as possible; payment must
//! validate the complete obligation before committing any of its actions.

use super::{
    ChoiceVisibilityDef, ControlDurationDef, CostDef, EffectDef, EffectRecipientDef,
    ObjectPredicateDef, ObjectQueryDef, ObjectSetDef, PlayerRefDef, PlayerRelation, PlayerSetDef,
    ValueDef, ZoneKind,
};
use crate::ids::Binding;

pub mod actions;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GameActionDef {
    Choose(GameActionChoiceDef),
    Sequence(&'static [GameActionDef]),
    /// Discard the named cards, preserving discard events and replacements.
    DiscardCards {
        object: EffectRecipientDef,
    },
    /// Each named permanent is sacrificed by its controller.
    Sacrifice {
        object: EffectRecipientDef,
    },
    /// The executing player sacrifices the named permanents they control.
    SacrificeYours {
        object: EffectRecipientDef,
    },
    GainControl {
        object: EffectRecipientDef,
        controller: PlayerRefDef,
        duration: ControlDurationDef,
    },
}

/// Select a computed number of objects before executing the nested action.
/// Effects select every available candidate when fewer than `amount` exist.
/// Costs require the full amount and reserve the complete selection first.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct GameActionChoiceDef {
    pub binding: Binding,
    pub chooser: PlayerRefDef,
    pub candidates: ObjectSetDef,
    pub amount: ValueDef,
    pub visibility: ChoiceVisibilityDef,
    pub then: &'static GameActionDef,
}

impl GameActionDef {
    /// Require this program as a complete resolving payment obligation.
    ///
    /// The surrounding payment procedure still validates which program shapes
    /// it can plan. Static card declarations can call this directly on an inline
    /// constructor or composition; no separately named program is necessary.
    #[must_use]
    pub const fn as_cost(&'static self) -> CostDef {
        CostDef::Perform(self)
    }

    /// Resolve this program, doing as much as possible.
    #[must_use]
    pub const fn as_effect(self) -> EffectDef {
        EffectDef::Perform(self)
    }

    /// Set the predicate of a query-based selection, retaining its zone,
    /// ownership, and control constraints.
    ///
    /// # Panics
    ///
    /// Panics unless this is a `Choose` action with query candidates.
    #[must_use]
    pub const fn matching(mut self, object: ObjectPredicateDef) -> Self {
        let ObjectSetDef::Query(query) = &mut self.choice_mut().candidates else {
            panic!("matching() requires query candidates");
        };
        query.object = object;
        self
    }

    /// Set a fixed or computed selection quantity.
    ///
    /// # Panics
    ///
    /// Panics unless this is a `Choose` action.
    #[must_use]
    pub const fn with_amount(mut self, amount: ValueDef) -> Self {
        self.choice_mut().amount = amount;
        self
    }

    /// Set who selects the objects, independently of the candidate set.
    ///
    /// # Panics
    ///
    /// Panics unless this is a `Choose` action.
    #[must_use]
    pub const fn with_chooser(mut self, chooser: PlayerRefDef) -> Self {
        self.choice_mut().chooser = chooser;
        self
    }

    /// Set who may observe a selection.
    ///
    /// # Panics
    ///
    /// Panics unless this is a `Choose` action.
    #[must_use]
    pub const fn with_visibility(mut self, visibility: ChoiceVisibilityDef) -> Self {
        self.choice_mut().visibility = visibility;
        self
    }

    const fn choice_mut(&mut self) -> &mut GameActionChoiceDef {
        let Self::Choose(choice) = self else {
            panic!("selection builders require a Choose action");
        };
        choice
    }

    /// The first payment slice supports independent exact object selections.
    /// Reject arbitrary branching, hidden-information-dependent planning, and
    /// unresolved external bindings before advertising a payable program.
    pub(crate) fn payment_choice(self) -> Option<GameActionChoiceDef> {
        let Self::Choose(choice) = self else {
            return None;
        };
        if choice.chooser != PlayerRefDef::EffectController {
            return None;
        }
        let ObjectSetDef::Query(query) = choice.candidates else {
            return None;
        };
        let bound = EffectRecipientDef::objects(ObjectSetDef::Binding(choice.binding));
        let you = PlayerSetDef::Related(PlayerRelation::You);
        let not_you = PlayerSetDef::Related(PlayerRelation::NotYou);
        let expected = match *choice.then {
            Self::DiscardCards { object } if object == bound => {
                ObjectQueryDef::owned_by(query.object, &[ZoneKind::Hand], you)
            }
            Self::SacrificeYours { object } | Self::Sacrifice { object } if object == bound => {
                ObjectQueryDef::controlled_by(query.object, &[ZoneKind::Battlefield], you)
            }
            Self::GainControl {
                object,
                controller: PlayerRefDef::EffectController,
                ..
            } if object == bound => {
                ObjectQueryDef::controlled_by(query.object, &[ZoneKind::Battlefield], not_you)
            }
            _ => return None,
        };
        (query == expected).then_some(choice)
    }

    pub(crate) fn payment_program_supported(self) -> bool {
        match self {
            Self::Sequence(actions) => {
                !actions.is_empty()
                    && actions
                        .iter()
                        .all(|action| action.payment_program_supported())
            }
            action => action.payment_choice().is_some(),
        }
    }
}
