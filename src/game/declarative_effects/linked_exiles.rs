//! Exile operations that must keep the new object's identity.
//!
//! Split out of the parent module for the source-size budget. Linked exile
//! remembers which source moved a card; play permissions instead attach to
//! the new card created by the zone change. Both must happen during the move,
//! before an ordinary continuation loses the old object's identity.

use super::super::{EffectResolutionContext, Game, ScopedEffect, StackObject, Target};
use crate::card::{EffectDef, EffectRecipientDef};
use crate::game::GameObjectId;

impl Game {
    pub(super) fn resolve_linked_exile_effect(
        &mut self,
        scoped: ScopedEffect,
        object: &StackObject,
        context: &EffectResolutionContext,
    ) {
        match scoped.effect {
            EffectDef::ExileLinkedToSource {
                object: recipient,
                then,
            } => {
                let source = object.source.unwrap_or(object.id);
                for exiled in self.exile_effect_objects(recipient, object, context, scoped) {
                    self.linked_exiles.push((source, exiled));
                }
                if let Some(then) = then {
                    self.resolve_effect_def(scoped.with_effect(*then), object, context.clone());
                }
            }
            EffectDef::ExileGrantingOwnerPlay {
                object: recipient,
                surcharge,
            } => {
                for exiled in self.exile_effect_objects(recipient, object, context, scoped) {
                    // Its owner, not the exiler: what the clause hands back
                    // is the card's own player's ability to play it.
                    if let Some((_, instance)) = self.card_in_nonbattlefield_zone(exiled) {
                        let owner = instance.owner;
                        self.permit_owner_play_while_exiled(exiled, owner, surcharge);
                    }
                }
            }
            EffectDef::ExileGrantingControllerPlayThisTurn { object: recipient } => {
                for exiled in self.exile_effect_objects(recipient, object, context, scoped) {
                    self.permit_cast_this_turn(exiled, object.controller);
                }
            }
            EffectDef::MayPlayWithoutPaying { objects } => {
                self.permit_playing_without_paying(objects, object, context, scoped);
            }
            EffectDef::ReturnLinkedExiles {
                object: predicate,
                zone,
                grant,
                counters,
                arrival_effect,
                controller,
                transformed,
            } => {
                let source = object.source.unwrap_or(object.id);
                let returning = self
                    .linked_exiles
                    .iter()
                    .filter(|(exiled_by, _)| *exiled_by == source)
                    .map(|(_, card)| *card)
                    .filter(|card| {
                        self.card_in_nonbattlefield_zone(*card)
                            .is_some_and(|(zone, instance)| {
                                self.card_object_matches(predicate, instance, zone, source)
                            })
                    })
                    .collect::<Vec<_>>();
                // Only what comes back stops being linked: a pile the clause
                // did not name is still exiled with this source, which is
                // what "each creature card exiled with it" leaves behind.
                self.linked_exiles
                    .retain(|(exiled_by, card)| *exiled_by != source || !returning.contains(card));
                let counters = counters.map(|counters| {
                    (
                        counters.kind,
                        u16::try_from(
                            self.effect_value(counters.amount, object, context, scoped)
                                .max(0),
                        )
                        .unwrap_or(u16::MAX),
                    )
                });
                let arriving_controller = controller.map(|relation| {
                    if self.player_relation_matches(
                        object.controller,
                        relation,
                        object.controller,
                        context.trigger,
                    ) {
                        object.controller
                    } else {
                        object.controller.opponent()
                    }
                });
                for card in returning {
                    let arrived = self.return_exiled_card(
                        card,
                        zone,
                        grant,
                        arriving_controller,
                        transformed,
                        counters,
                    );
                    // Applied as the move happens: what arrives is a new
                    // object, so a later effect would have nothing to name.
                    if let (Some(effect), Some(arrived)) = (arrival_effect, arrived) {
                        self.apply_arrival_effect(arrived, *effect, object, context, scoped);
                    }
                }
            }
            _ => {}
        }
    }

    fn exile_effect_objects(
        &mut self,
        recipient: EffectRecipientDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> Vec<GameObjectId> {
        self.effect_recipients(recipient, object, context, scoped)
            .into_iter()
            .filter_map(|target| match target {
                Target::Permanent(id) => self.exile_permanent_returning_card(id),
                Target::Card(id) => self.exile_card_returning_card(id),
                Target::Player(_) | Target::Spell(_) => None,
            })
            .collect()
    }

    /// "You may play those cards without paying their mana costs." The
    /// permission lasts the turn it was granted on, which is the turn the
    /// ability resolved.
    fn permit_playing_without_paying(
        &mut self,
        objects: crate::card::ObjectSetDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        for target in self.effect_objects(objects, object, context, scoped) {
            if let Target::Card(card) = target {
                self.permit_free_play_this_turn(card, object.controller);
            }
        }
    }
}
