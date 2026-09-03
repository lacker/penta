//! Object matching for replacement effects that watch zone moves externally.
//!
//! A nonbattlefield card does not have the live characteristics of a
//! permanent, but its physical kind and owner are stable on both sides of a
//! prospective move. Keep this evaluator restricted to those axes and let the
//! catalog runtime boundary reject every predicate it cannot answer.

use super::{Game, PlayerId, TriggerContext};
use crate::card::ObjectPredicateDef;

impl Game {
    pub(in crate::game) fn zone_move_object_matches(
        &self,
        predicate: ObjectPredicateDef,
        owner: PlayerId,
        is_token: bool,
        replacement_controller: PlayerId,
    ) -> bool {
        self.zone_move_object_match(predicate, owner, is_token, replacement_controller)
            .unwrap_or(false)
    }

    fn zone_move_object_match(
        &self,
        predicate: ObjectPredicateDef,
        owner: PlayerId,
        is_token: bool,
        replacement_controller: PlayerId,
    ) -> Option<bool> {
        match predicate {
            ObjectPredicateDef::Any => Some(true),
            ObjectPredicateDef::Token => Some(is_token),
            ObjectPredicateDef::OwnedBy(relation) => Some(self.player_relation_matches(
                owner,
                relation,
                replacement_controller,
                TriggerContext::empty(),
            )),
            ObjectPredicateDef::All(predicates) => {
                let mut matches = true;
                for predicate in predicates {
                    matches &= self.zone_move_object_match(
                        *predicate,
                        owner,
                        is_token,
                        replacement_controller,
                    )?;
                }
                Some(matches)
            }
            ObjectPredicateDef::AnyOf(predicates) => {
                let mut matches = false;
                for predicate in predicates {
                    matches |= self.zone_move_object_match(
                        *predicate,
                        owner,
                        is_token,
                        replacement_controller,
                    )?;
                }
                Some(matches)
            }
            ObjectPredicateDef::Not(predicate) => Some(!self.zone_move_object_match(
                *predicate,
                owner,
                is_token,
                replacement_controller,
            )?),
            _ => None,
        }
    }
}
