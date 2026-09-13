//! Face-down permanents: casting one, and turning it face up.
//!
//! A face-down permanent's body lives in `characteristics`; this module owns
//! the one action that changes the state. Turning a permanent face up is a
//! special action (CR 702.37b): it uses no stack, nobody may respond to it,
//! and it is available even though the permanent has no abilities at all
//! while it is face down.

use super::{Action, Game, GameObjectId, PlayerId};

impl Game {
    /// The spell's own abilities after its face-down characteristics replace
    /// the printed face. External effects are evaluated by their own lanes.
    pub(super) fn for_each_stack_spell_ability(
        &self,
        object: &super::StackObject,
        mut visitor: impl FnMut(super::EffectiveAbility),
    ) {
        if let Some(face_down) = object.face_down {
            for attached in face_down.rules().indexed_abilities() {
                visitor(super::EffectiveAbility {
                    origin: crate::AbilityOrigin::FaceDown {
                        ability: attached.id,
                    },
                    ability: attached.definition,
                });
            }
        } else if let Some(signature) = &object.signature
            && let Some(card) = object.card.clone().into_card()
        {
            self.for_each_printed_card_ability(
                &card,
                &super::CharacteristicContext::Stack {
                    form: signature.form().clone(),
                },
                visitor,
            );
        }
    }

    /// A face-down card in exile has no characteristics (unlike a face-down
    /// spell or permanent, whose creating mechanism supplies a body).
    pub(super) fn face_down_exiled_event_object(
        id: GameObjectId,
        owner: PlayerId,
    ) -> super::TriggerEventObject {
        super::TriggerEventObject {
            id,
            token: false,
            has_adventure: false,
            types: super::CardTypeSet::empty(),
            controller: owner,
            colors: [false; 5],
            subtypes: std::borrow::Cow::Borrowed(&[]),
            mana_value: 0,
            power: None,
            toughness: None,
            supertypes: [false; super::CardSupertype::COUNT],
            attacking_or_blocking: false,
            keywords: 0,
            attacking: false,
            tapped: false,
            attacked_this_turn: false,
            saddled: false,
            attacked_during_controllers_last_turn: false,
        }
    }

    /// The morph cost printed on the physical card under a permanent, if it
    /// has one. Read off `card.definition` rather than the presented rules,
    /// which while face down are the body's and carry nothing.
    pub(super) fn printed_morph_cost(
        &self,
        permanent: &super::Permanent,
    ) -> Option<&'static [crate::CostDef]> {
        self.catalog
            .get(permanent.card.definition.card_definition()?)?
            .part(permanent.presented)?
            .rules
            .morph_cost()
    }

    /// What turning this permanent face up costs, or `None` when nothing
    /// can. A morph-like object pays the special cost its card prints; a
    /// Manifest- or Cloak-like object pays the card's own mana cost, and only
    /// if the card under it is a creature card (CR 701.34c, 701.58c).
    pub(super) fn face_up_cost(&self, permanent: &super::Permanent) -> Option<Vec<crate::CostDef>> {
        if self
            .catalog
            .get(permanent.card.definition.card_definition()?)?
            .rules
            .implementation_status()
            == crate::ImplementationStatus::Unsupported
        {
            return None;
        }
        if let Some(cost) = self.printed_morph_cost(permanent) {
            return Some(cost.to_vec());
        }
        if !permanent.turn_up_for_mana_cost {
            return None;
        }
        let part = self
            .catalog
            .get(permanent.card.definition.card_definition()?)?
            .part(permanent.presented)?;
        part.rules
            .has_type(crate::card::CardType::Creature)
            .then(|| {
                part.rules
                    .mana_cost()
                    .map(|mana| vec![crate::CostDef::Mana(mana)])
            })
            .flatten()
    }

    pub(super) fn add_face_up_actions(&self, player: PlayerId, actions: &mut Vec<Action>) {
        for permanent in self
            .battlefield
            .iter()
            .filter(|permanent| permanent.face_down.is_some() && permanent.controller == player)
        {
            if self.can_pay_special_action(
                player,
                permanent.card.id,
                super::special_action_payments::PaidSpecialAction::TurnFaceUp,
            ) {
                actions.push(Action::TurnFaceUp {
                    permanent: permanent.card.id,
                });
            }
        }
    }

    pub(super) fn turn_face_up(&mut self, player: PlayerId, permanent: GameObjectId) {
        self.begin_special_action_payment(
            player,
            permanent,
            super::special_action_payments::PaidSpecialAction::TurnFaceUp,
        );
    }

    pub(in crate::game) fn finish_turn_face_up(&mut self, permanent: GameObjectId) {
        // Turning face up is not a zone change and creates no new object, so
        // the permanent keeps its identity, its counters, and its damage. It
        // simply stops presenting the body.
        if let Some(target) = self
            .battlefield
            .iter_mut()
            .find(|candidate| candidate.card.id == permanent)
        {
            target.face_down = None;
        }
        // Nothing in the staged tranche triggers on a permanent being
        // turned face up, so there is no event to raise yet; the state
        // change alone is what the morph cards here need.
        let _ = permanent;
    }
}
