//! Game actions shared by resolving instructions and payment obligations.
//!
//! The program owns selection vocabulary and action identity. Its caller owns
//! the execution contract: resolution does as much as possible; payment must
//! validate the complete obligation before committing any of its actions.

use super::{
    ChoiceVisibilityDef, ControlDurationDef, EffectRecipientDef, ObjectPredicateDef,
    ObjectQueryDef, ObjectSetDef, PlayerRefDef, PlayerRelation, PlayerSetDef, ValueDef, ZoneKind,
};
use crate::ids::Binding;

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
    #[must_use]
    pub const fn choose_discard(object: ObjectPredicateDef, amount: ValueDef) -> Self {
        Self::Choose(GameActionChoiceDef {
            binding: crate::ids::ParentBinding,
            chooser: PlayerRefDef::EffectController,
            candidates: ObjectSetDef::Query(ObjectQueryDef::owned_by(
                object,
                &[ZoneKind::Hand],
                PlayerSetDef::Related(PlayerRelation::You),
            )),
            amount,
            visibility: ChoiceVisibilityDef::Private,
            then: &DISCARD_CHOSEN,
        })
    }

    #[must_use]
    pub const fn choose_sacrifice(object: ObjectPredicateDef, amount: ValueDef) -> Self {
        Self::Choose(GameActionChoiceDef {
            binding: crate::ids::ParentBinding,
            chooser: PlayerRefDef::EffectController,
            candidates: ObjectSetDef::Query(ObjectQueryDef::controlled_by(
                object,
                &[ZoneKind::Battlefield],
                PlayerSetDef::Related(PlayerRelation::You),
            )),
            amount,
            visibility: ChoiceVisibilityDef::Public,
            then: &SACRIFICE_CHOSEN,
        })
    }

    /// Gain control indefinitely; a separate instruction may later return it.
    #[must_use]
    pub const fn choose_gain_control(object: ObjectPredicateDef, amount: ValueDef) -> Self {
        Self::Choose(GameActionChoiceDef {
            binding: crate::ids::ParentBinding,
            chooser: PlayerRefDef::EffectController,
            candidates: ObjectSetDef::Query(ObjectQueryDef::controlled_by(
                object,
                &[ZoneKind::Battlefield],
                PlayerSetDef::Related(PlayerRelation::NotYou),
            )),
            amount,
            visibility: ChoiceVisibilityDef::Public,
            then: &GAIN_CONTROL_OF_CHOSEN,
        })
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

const DISCARD_CHOSEN: GameActionDef = GameActionDef::DiscardCards {
    object: EffectRecipientDef::objects(ObjectSetDef::Binding(crate::ids::ParentBinding)),
};
const SACRIFICE_CHOSEN: GameActionDef = GameActionDef::SacrificeYours {
    object: EffectRecipientDef::objects(ObjectSetDef::Binding(crate::ids::ParentBinding)),
};
const GAIN_CONTROL_OF_CHOSEN: GameActionDef = GameActionDef::GainControl {
    object: EffectRecipientDef::objects(ObjectSetDef::Binding(crate::ids::ParentBinding)),
    controller: PlayerRefDef::EffectController,
    duration: ControlDurationDef::Indefinitely,
};
