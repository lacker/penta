//! Rooms (CR 714).
//!
//! A Room is a split enchantment whose halves are doors. Casting one half
//! puts the enchantment onto the battlefield with that door unlocked; the
//! other stays shut until someone pays for it. What the permanent is at any
//! moment is the combination of its unlocked doors, which is why the doors,
//! the pair of them, and neither of them are all parts of the card.
//!
//! Unlocking is a special action: it uses no stack and cannot be responded
//! to, so a door opens between two priorities rather than during one.

use crate::card::{AbilityProcedureDef, CardStructure, DeclarativeAbilityDef, TriggerEventDef};
use crate::ids::{CardPartId, GameObjectId};

use super::{
    AbilityOrigin, AbilitySourceRef, Action, EffectDef, Game, ManaPaymentPurpose, PlayerId,
    TriggerCapture, TriggerContext,
};

impl Game {
    /// "Any time you have priority during a main phase of your turn and the
    /// stack is empty": the sorcery window, which is what the reminder text
    /// means by "as a sorcery".
    pub(super) fn add_unlock_door_actions(&self, player: PlayerId, actions: &mut Vec<Action>) {
        if player != self.active_player || !self.step.is_main() || !self.stack.is_empty() {
            return;
        }
        for permanent in &self.battlefield {
            if permanent.controller != player || permanent.face_down.is_some() {
                continue;
            }
            let Some(definition) = permanent.card.definition.card_definition() else {
                continue;
            };
            let Some(card) = self.catalog.get(definition) else {
                continue;
            };
            for door in card.locked_doors(permanent.presented) {
                let Some(cost) = card.part(door).and_then(|part| part.rules.mana_cost()) else {
                    continue;
                };
                if !self.can_pay_cost_for(player, cost, 0, &ManaPaymentPurpose::Other) {
                    continue;
                }
                actions.push(Action::UnlockDoor {
                    room: permanent.card.id,
                    door,
                });
            }
        }
    }

    pub(super) fn unlock_door(&mut self, player: PlayerId, room: GameObjectId, door: CardPartId) {
        let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == room)
        else {
            return;
        };
        if permanent.controller != player || permanent.face_down.is_some() {
            return;
        }
        let Some(definition) = permanent.card.definition.card_definition() else {
            return;
        };
        let presented = permanent.presented;
        let Some(card) = self.catalog.get(definition) else {
            return;
        };
        let Some(unlocked) = card.presentation_after_unlocking(presented, door) else {
            return;
        };
        let Some(cost) = card.part(door).and_then(|part| part.rules.mana_cost()) else {
            return;
        };
        if !self.can_pay_cost_for(player, cost, 0, &ManaPaymentPurpose::Other) {
            return;
        }
        self.activate_mana_for_cost(player, cost, 0);
        let _spent = self.pay_player_cost(player, cost, 0);
        // The permanent is looked up again: paying can resolve mana
        // abilities, and a Room that sacrificed itself for mana is no longer
        // there to open.
        let Some(permanent) = self
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == room)
        else {
            return;
        };
        if permanent.presented != presented {
            return;
        }
        permanent.presented = unlocked;
        self.capture_door_unlocked(room, door);
    }

    /// A Room that came off the stack arrives with the door you cast already
    /// open, and that opening is what its clause is about (CR 714.4c). It
    /// happens as part of entering rather than because of it, which is why an
    /// effect that doubles enter triggers leaves it alone.
    pub(super) fn capture_room_entry_unlock(&mut self, permanent: GameObjectId) {
        let unlocked = self
            .battlefield
            .iter()
            .find(|entered| entered.card.id == permanent)
            .filter(|entered| entered.face_down.is_none())
            .and_then(|entered| {
                let definition = entered.card.definition.card_definition()?;
                let card = self.catalog.get(definition)?;
                let CardStructure::Room { doors, .. } = &card.structure else {
                    return None;
                };
                doors
                    .contains(&entered.presented)
                    .then_some(entered.presented)
            });
        if let Some(door) = unlocked {
            self.capture_door_unlocked(permanent, door);
        }
    }

    /// Hands "when you unlock this door" to the door that just opened.
    ///
    /// Deliberately not an ordinary committed event. A Room with both doors
    /// open presents the pair of them, so a general sweep over its abilities
    /// would find the other door's copy of this clause too and open-nothing
    /// would fire it.
    pub(super) fn capture_door_unlocked(&mut self, room: GameObjectId, door: CardPartId) {
        let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == room)
        else {
            return;
        };
        let Some(definition) = permanent.card.definition.card_definition() else {
            return;
        };
        let owner = permanent.card.owner;
        let controller = permanent.controller;
        let Some(part) = self
            .catalog
            .get(definition)
            .and_then(|card| card.part(door))
        else {
            return;
        };
        let mut captures = Vec::new();
        for attached in part.rules.indexed_abilities() {
            let ability = attached.definition;
            let DeclarativeAbilityDef::Triggered(triggered) = ability.definition else {
                continue;
            };
            if triggered.event != TriggerEventDef::DoorUnlocked
                || triggered.procedure != AbilityProcedureDef::Shared
            {
                continue;
            }
            let origin = AbilityOrigin::Printed {
                definition,
                part: door,
                ability: attached.id,
            };
            captures.push(TriggerCapture {
                source: AbilitySourceRef {
                    object: room,
                    ability: origin,
                },
                presentation: Self::ability_presentation(
                    origin,
                    Self::effective_rules_source(permanent),
                ),
                owner,
                controller,
                text: ability.text,
                target_defs: triggered.targets.to_vec(),
                targets: Vec::new(),
                effect: ability.declarative_effect().unwrap_or(EffectDef::None),
                resolver: Self::ability_resolver(origin, &ability),
                context: TriggerContext::empty().into(),
                condition: triggered.condition,
                modes: triggered.modes,
                x: 0,
            });
        }
        for capture in &captures {
            self.capture_trigger(capture);
        }
    }
}
