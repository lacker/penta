//! Exile operations that must keep the new object's identity.
//!
//! Split out of the parent module for the source-size budget. Linked exile
//! remembers which source moved a card; play permissions instead attach to
//! the new card created by the zone change. Both consume the shared movement
//! results after replacements and all members of the exit batch finish.

use super::super::{EffectResolutionContext, Game, ScopedEffect, StackObject, Target};
use crate::card::{EffectDef, EffectRecipientDef, ZoneKind, ZonePlacement};
use crate::game::{BattlefieldExitCause, BattlefieldExitCompletion, GameObjectId, ZoneMoveCause};

impl Game {
    /// Return a paired exile as the duration ends, before any player receives
    /// priority. Ability removal and trigger suppression cannot stop this.
    pub(in crate::game) fn return_exiles_whose_duration_ended(&mut self) {
        let mut returning = Vec::new();
        self.exile_returns.retain(|(source, card, zone)| {
            if self
                .battlefield
                .iter()
                .chain(&self.phased_out)
                .any(|permanent| permanent.card.id == *source)
            {
                true
            } else {
                returning.push((*source, *card, *zone));
                false
            }
        });
        self.linked_exiles.retain(|(source, card)| {
            !returning.iter().any(|(return_source, return_card, _)| {
                return_source == source && return_card == card
            })
        });
        if returning.is_empty() {
            return;
        }
        self.entering_together(|game| {
            for (_, card, zone) in returning {
                game.return_exiled_card(card, zone, None, None, false, None);
            }
        });
    }

    pub(in crate::game) fn matching_linked_exiles(
        &self,
        predicate: crate::card::ObjectPredicateDef,
        object: &StackObject,
    ) -> Vec<Target> {
        let source = object.source.unwrap_or(object.id);
        self.linked_exile_ids(source)
            .into_iter()
            .filter(|card| {
                self.card_in_nonbattlefield_zone(*card)
                    .is_some_and(|(zone, instance)| {
                        self.card_object_matches(predicate, instance, zone, source)
                    })
            })
            .map(Target::Card)
            .collect()
    }

    /// The declaration owns links and permissions; the ordinary movement machinery
    /// owns replacements, simultaneous battlefield exits, and new object identities.
    fn resolve_exile_effect(
        &mut self,
        recipient: EffectRecipientDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        let source = object.source.unwrap_or(object.id);
        if matches!(
            scoped.effect,
            EffectDef::ExileLinkedToSource {
                until_source_leaves: true,
                ..
            }
        ) && !self
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == source)
        {
            return;
        }
        let face_down = matches!(
            scoped.effect,
            EffectDef::ExileLinkedToSource {
                face_down: true,
                ..
            }
        );
        let mut origins = Vec::new();
        for origin in self
            .effect_recipients(recipient, object, context, scoped)
            .into_iter()
            .filter_map(|target| match target {
                Target::Permanent(id) => Some((id, ZoneKind::Battlefield)),
                Target::Card(id) => self
                    .card_in_nonbattlefield_zone(id)
                    .filter(|(zone, _)| *zone != ZoneKind::Exile)
                    .map(|(zone, _)| (id, zone)),
                _ => None,
            })
        {
            if !origins.contains(&origin) {
                origins.push(origin);
            }
        }
        let cards = origins
            .iter()
            .filter(|(_, zone)| *zone != ZoneKind::Battlefield)
            .map(|(card, _)| *card)
            .collect::<Vec<_>>();
        let pending_before = self.pending_decisions.len();
        self.exile_cards_returning_cards(
            &cards,
            face_down,
            ZoneMoveCause::Effect {
                controller: object.controller,
            },
        );
        let permanents = origins
            .iter()
            .filter(|(_, zone)| *zone == ZoneKind::Battlefield)
            .map(|(card, _)| (*card, BattlefieldExitCause::Other))
            .collect::<Vec<_>>();
        let completion = BattlefieldExitCompletion::ExileEffect {
            origins,
            object: Box::new(object.clone()),
            context: context.clone(),
            effect: scoped,
        };
        // A nonbattlefield replacement can redirect a commander to a hidden
        // zone and suspend its move. Preserve the effect's follow-up there too.
        if permanents.is_empty()
            && self.pending_decisions.len() > pending_before
            && self.defer_after_battlefield_exit(pending_before, completion.clone())
        {
            return;
        }
        self.move_permanents_to_zone_with_visibility_then(
            &permanents,
            ZoneKind::Exile,
            ZonePlacement::Top,
            face_down,
            Some(completion),
        );
    }

    pub(in crate::game) fn finish_exile_effect(
        &mut self,
        origins: &[(GameObjectId, ZoneKind)],
        object: &StackObject,
        context: EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        // Only the immediate successor in exile belongs to this instruction.
        // Canceled moves, redirected destinations, and departed exile objects
        // never receive a link or a play permission.
        let exiled = origins
            .iter()
            .filter_map(|(original, from)| {
                let successor = *self.successors.get(original)?;
                self.card_in_nonbattlefield_zone(successor)
                    .filter(|(zone, _)| *zone == ZoneKind::Exile)
                    .map(|(_, card)| (successor, card.owner, *from))
            })
            .collect::<Vec<_>>();
        match scoped.effect {
            EffectDef::ExileLinkedToSource {
                until_source_leaves,
                then,
                ..
            } => {
                let source = object.source.unwrap_or(object.id);
                for (card, _, from) in exiled {
                    if !self.linked_exiles.contains(&(source, card)) {
                        self.linked_exiles.push((source, card));
                        if until_source_leaves
                            && matches!(from, ZoneKind::Battlefield | ZoneKind::Hand)
                        {
                            self.exile_returns.push((source, card, from));
                        }
                    }
                }
                self.return_exiles_whose_duration_ended();
                if let Some(then) = then {
                    self.resolve_effect_def(scoped.with_effect(*then), object, context);
                }
            }
            EffectDef::ExileGrantingOwnerPlay {
                surcharge,
                later_turn,
                cast_only,
                ..
            } => {
                for (card, owner, _) in exiled {
                    self.permit_owner_play_while_exiled(
                        card, owner, surcharge, later_turn, cast_only,
                    );
                }
            }
            EffectDef::ExileGrantingControllerPlayThisTurn { .. } => {
                for (card, _, _) in exiled {
                    self.permit_cast_this_turn(card, object.controller);
                }
            }
            _ => unreachable!("only exile declarations install this completion"),
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
                object: recipient, ..
            }
            | EffectDef::ExileGrantingOwnerPlay {
                object: recipient, ..
            }
            | EffectDef::ExileGrantingControllerPlayThisTurn { object: recipient } => {
                self.resolve_exile_effect(recipient, object, context, scoped);
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
            if let Some(granted) = self
                .exile_play_permissions
                .iter_mut()
                .rev()
                .find(|granted| granted.card == card && granted.player == player)
            {
                granted.grants_haste = permission.grants_haste;
                granted.maximum_spell_mana_value = permission.maximum_spell_mana_value;
                granted.lands_may_be_played = !permission.cast_only;
            }
            if permission.duration == crate::card::FreePlayDurationDef::UntilEndOfTurn {
                continue;
            }
            self.offer_permitted_play(player, card, permission.mandatory, object, context, scoped);
        }
    }
}
