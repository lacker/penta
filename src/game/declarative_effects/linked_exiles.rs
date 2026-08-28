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
    pub(super) fn matching_linked_exiles(
        &self,
        predicate: crate::card::ObjectPredicateDef,
        object: &StackObject,
    ) -> Vec<Target> {
        let source = object.source.unwrap_or(object.id);
        self.linked_exiles
            .iter()
            .filter(|(exiled_by, _)| *exiled_by == source)
            .map(|(_, card)| *card)
            .filter(|card| {
                self.card_in_nonbattlefield_zone(*card)
                    .is_some_and(|(zone, instance)| {
                        self.card_object_matches(predicate, instance, zone, source)
                    })
            })
            .map(Target::Card)
            .collect()
    }

    /// Turns a linked exile face down. That is a fact about the exile rather
    /// than about who may play the card: what it buys the owner is a look at
    /// their own pile, and everybody else a count of it.
    fn hide_linked_exile(&mut self, exiled: GameObjectId, face_down: bool) {
        if !face_down {
            return;
        }
        if let Some((_, instance)) = self.card_in_nonbattlefield_zone(exiled) {
            let owner = instance.owner;
            // Nobody may look at a card exiled face down unless something
            // says so (CR 713.2), and a linked exile says nothing: the
            // hands Memory Jar puts away are hidden from both players until
            // the end step hands them back.
            self.hide_from_everyone_while_exiled(exiled, owner);
        }
    }

    /// One linked exile, kept apart from the match above only for its size.
    #[allow(clippy::too_many_arguments)]
    fn resolve_exile_linked_to_source(
        &mut self,
        recipient: EffectRecipientDef,
        face_down: bool,
        until_source_leaves: bool,
        then: Option<&'static EffectDef>,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        let source = object.source.unwrap_or(object.id);
        // "Exile it until this permanent leaves the battlefield" does
        // nothing once that permanent has already gone: there is no
        // later moment for what it took to come back (CR 610.3b). An
        // ordinary linked exile is not conditional that way -- what a
        // Parallax Wave takes on its way out stays taken.
        if until_source_leaves
            && !self
                .battlefield
                .iter()
                .any(|permanent| permanent.card.id == source)
        {
            return;
        }
        for exiled in self.exile_effect_objects(recipient, object, context, scoped) {
            self.linked_exiles.push((source, exiled));
            self.hide_linked_exile(exiled, face_down);
        }
        if let Some(then) = then {
            self.resolve_effect_def(scoped.with_effect(*then), object, context.clone());
        }
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn resolve_linked_exile_effect(
        &mut self,
        scoped: ScopedEffect,
        object: &StackObject,
        context: &EffectResolutionContext,
    ) {
        match scoped.effect {
            EffectDef::ExileLinkedToSource {
                object: recipient,
                face_down,
                until_source_leaves,
                then,
            } => {
                self.resolve_exile_linked_to_source(
                    recipient,
                    face_down,
                    until_source_leaves,
                    then,
                    object,
                    context,
                    scoped,
                );
            }
            EffectDef::PermitLookAtExiled {
                object: recipient,
                player,
                then,
            } => {
                let Some(player) = self.effect_player_reference(player, object, context, scoped)
                else {
                    return;
                };
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    let Target::Card(card) = target else {
                        continue;
                    };
                    let exiled = self
                        .card_in_nonbattlefield_zone(card)
                        .is_some_and(|(zone, _)| zone == crate::card::ZoneKind::Exile)
                        .then_some(card)
                        .or_else(|| self.successors.get(&card).copied());
                    if let Some(exiled) = exiled
                        && self
                            .card_in_nonbattlefield_zone(exiled)
                            .is_some_and(|(zone, _)| zone == crate::card::ZoneKind::Exile)
                    {
                        self.permit_look_while_exiled(exiled, player);
                    }
                }
                self.resolve_effect_def(scoped.with_effect(*then), object, context.clone());
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
            EffectDef::MayPlayWithoutPaying(permission) => {
                self.permit_playing_without_paying(permission, object, context, scoped);
            }
            EffectDef::ReturnLinkedExiles {
                object: predicate,
                zone,
                grant,
                counters,
                controller,
                transformed,
            } => {
                let source = object.source.unwrap_or(object.id);
                let returning = self
                    .matching_linked_exiles(predicate, object)
                    .into_iter()
                    .filter_map(|target| match target {
                        Target::Card(card) => Some(card),
                        _ => None,
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
                    self.return_exiled_card(
                        card,
                        zone,
                        grant,
                        arriving_controller,
                        transformed,
                        counters,
                    );
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
    /// Grants "you may play it without paying its mana cost" over a set of
    /// cards, for as long as the clause that printed it says.
    ///
    /// A permission that outlives its resolution is simply granted. One that
    /// does not is granted *and offered*: the offer is a standing decision,
    /// taken by playing the card while it waits and declined by answering
    /// it, and declining takes the permission straight back. A card with no
    /// legal play is never offered, and its permission goes back at once
    /// rather than lingering until the end of the turn.
    fn permit_playing_without_paying(
        &mut self,
        permission: crate::card::FreePlayDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        let player = object.controller;
        for target in self.effect_objects(permission.objects, object, context, scoped) {
            let Target::Card(card) = target else {
                continue;
            };
            self.permit_free_play_this_turn(card, player);
            if permission.grants_haste
                && let Some(granted) = self
                    .exile_play_permissions
                    .iter_mut()
                    .rev()
                    .find(|granted| granted.card == card && granted.player == player)
            {
                granted.grants_haste = true;
            }
            if permission.duration == crate::card::FreePlayDurationDef::UntilEndOfTurn {
                continue;
            }
            self.offer_permitted_play(player, card, permission.mandatory, object, context, scoped);
        }
    }
}
