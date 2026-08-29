//! Resolving an object query into the objects it matches.
//!
//! Split out of the parent module for the source-size budget.

#![allow(clippy::wildcard_imports)]

use super::*;
use crate::card::ZoneRelativePositionDef;

impl Game {
    fn ordered_zone_position(&self, object: GameObjectId) -> Option<(ZoneKind, PlayerId, usize)> {
        for player in [PlayerId::One, PlayerId::Two] {
            let zones = [
                (ZoneKind::Library, &self.players[player.index()].library),
                (ZoneKind::Graveyard, &self.players[player.index()].graveyard),
            ];
            for (zone, cards) in zones {
                if let Some(position) = cards.iter().position(|card| card.id == object) {
                    return Some((zone, player, position));
                }
            }
        }
        None
    }

    fn query_reference_object(
        &self,
        reference: ObjectRefDef,
        source: GameObjectId,
        context: TriggerContext,
        effect_context: Option<(&StackObject, ScopedEffect, &EffectResolutionContext)>,
    ) -> Option<GameObjectId> {
        match reference {
            ObjectRefDef::Source => Some(source),
            ObjectRefDef::TriggeringObject => context.object,
            ObjectRefDef::DamagedObject => context.damaged_object,
            _ => effect_context.and_then(|(object, scoped, resolution)| {
                self.object_reference_target(reference, object, resolution, scoped)
                    .and_then(|target| match target {
                        Target::Permanent(id) | Target::Card(id) | Target::Spell(id) => Some(id),
                        Target::Player(_) => None,
                    })
            }),
        }
    }

    fn query_relative_position_matches(
        &self,
        candidate: GameObjectId,
        relative: ZoneRelativePositionDef,
        source: GameObjectId,
        context: TriggerContext,
        effect_context: Option<(&StackObject, ScopedEffect, &EffectResolutionContext)>,
    ) -> bool {
        let reference = match relative {
            ZoneRelativePositionDef::Above(reference)
            | ZoneRelativePositionDef::Below(reference) => reference,
        };
        let Some(anchor) = self.query_reference_object(reference, source, context, effect_context)
        else {
            return false;
        };
        let (Some(candidate), Some(anchor)) = (
            self.ordered_zone_position(candidate),
            self.ordered_zone_position(anchor),
        ) else {
            return false;
        };
        if candidate.0 != anchor.0 || candidate.1 != anchor.1 {
            return false;
        }
        match relative {
            ZoneRelativePositionDef::Above(_) => candidate.2 > anchor.2,
            ZoneRelativePositionDef::Below(_) => candidate.2 < anchor.2,
        }
    }

    /// Finds objects using only zone, relation, and effective-characteristic
    /// predicates. Unlike target enumeration, this does not apply hexproof,
    /// protection, or any other targeting restriction.
    pub(in crate::game) fn objects_matching_query(
        &self,
        query: ObjectQueryDef,
        evaluation_controller: PlayerId,
        source: GameObjectId,
        context: TriggerContext,
    ) -> Vec<Target> {
        self.objects_matching_query_with_prospective(
            query,
            evaluation_controller,
            source,
            context,
            None,
        )
    }

    pub(in crate::game) fn objects_matching_effect_query(
        &self,
        query: ObjectQueryDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> Vec<Target> {
        self.objects_matching_query_with_context(
            query,
            object.controller,
            object.source.unwrap_or(object.id),
            context.trigger,
            None,
            Some((object, scoped, context)),
        )
    }

    pub(in crate::game) fn objects_matching_query_with_prospective(
        &self,
        query: ObjectQueryDef,
        evaluation_controller: PlayerId,
        source: GameObjectId,
        context: TriggerContext,
        prospective: Option<&Permanent>,
    ) -> Vec<Target> {
        self.objects_matching_query_with_context(
            query,
            evaluation_controller,
            source,
            context,
            prospective,
            None,
        )
    }

    fn objects_matching_query_with_context(
        &self,
        query: ObjectQueryDef,
        evaluation_controller: PlayerId,
        source: GameObjectId,
        context: TriggerContext,
        prospective: Option<&Permanent>,
        effect_context: Option<(&StackObject, ScopedEffect, &EffectResolutionContext)>,
    ) -> Vec<Target> {
        let mut recipients = Vec::new();
        let result = self.visit_objects_matching_query_with_context(
            query,
            evaluation_controller,
            source,
            context,
            prospective,
            effect_context,
            |recipient| {
                recipients.push(recipient);
                ControlFlow::Continue(())
            },
        );
        debug_assert!(result.is_continue());
        recipients
    }

    pub(in crate::game) fn any_object_matches_query_with_prospective(
        &self,
        query: ObjectQueryDef,
        evaluation_controller: PlayerId,
        source: GameObjectId,
        context: TriggerContext,
        prospective: Option<&Permanent>,
    ) -> bool {
        self.visit_objects_matching_query_with_context(
            query,
            evaluation_controller,
            source,
            context,
            prospective,
            None,
            |_| ControlFlow::Break(()),
        )
        .is_break()
    }

    #[allow(clippy::too_many_arguments)]
    pub(in crate::game) fn visit_objects_matching_query_with_prospective(
        &self,
        query: ObjectQueryDef,
        evaluation_controller: PlayerId,
        source: GameObjectId,
        context: TriggerContext,
        prospective: Option<&Permanent>,
        effect_context: Option<(&StackObject, ScopedEffect, &EffectResolutionContext)>,
        visitor: impl FnMut(Target) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        self.visit_objects_matching_query_with_context(
            query,
            evaluation_controller,
            source,
            context,
            prospective,
            effect_context,
            visitor,
        )
    }

    fn player_matches_set(
        &self,
        candidate: PlayerId,
        players: PlayerSetDef,
        query_origin: (PlayerId, GameObjectId),
        context: TriggerContext,
        effect_context: Option<(&StackObject, ScopedEffect, &EffectResolutionContext)>,
    ) -> bool {
        let (evaluation_controller, source) = query_origin;
        match players {
            PlayerSetDef::All => true,
            PlayerSetDef::Related(relation) => self.player_relation_matches_for_source(
                candidate,
                relation,
                evaluation_controller,
                source,
                context,
            ),
            PlayerSetDef::One(PlayerRefDef::EffectController) => candidate == evaluation_controller,
            PlayerSetDef::One(PlayerRefDef::EventPlayer) => context.event_player == Some(candidate),
            PlayerSetDef::LegalTargets(target) => {
                effect_context.is_some_and(|(object, scoped, resolution)| {
                    self.players_in_set(
                        PlayerSetDef::LegalTargets(target),
                        object,
                        resolution,
                        scoped,
                    )
                    .contains(&candidate)
                })
            }
            PlayerSetDef::One(reference) => {
                effect_context.is_some_and(|(object, scoped, resolution)| {
                    self.player_reference(reference, object, resolution, scoped) == Some(candidate)
                })
            }
        }
    }

    pub(in crate::game) fn query_player_constraints_match(
        &self,
        controller: Option<PlayerId>,
        owner: PlayerId,
        query: ObjectQueryDef,
        query_origin: (PlayerId, GameObjectId),
        context: TriggerContext,
        effect_context: Option<(&StackObject, ScopedEffect, &EffectResolutionContext)>,
    ) -> bool {
        query.related_player.is_none_or(|players| {
            self.player_matches_set(
                controller.unwrap_or(owner),
                players,
                query_origin,
                context,
                effect_context,
            )
        }) && query.controller.is_none_or(|players| {
            controller.is_some_and(|candidate| {
                self.player_matches_set(candidate, players, query_origin, context, effect_context)
            })
        }) && query.owner.is_none_or(|players| {
            self.player_matches_set(owner, players, query_origin, context, effect_context)
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn visit_objects_matching_query_with_context(
        &self,
        query: ObjectQueryDef,
        evaluation_controller: PlayerId,
        source: GameObjectId,
        context: TriggerContext,
        prospective: Option<&Permanent>,
        effect_context: Option<(&StackObject, ScopedEffect, &EffectResolutionContext)>,
        mut visitor: impl FnMut(Target) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        // "Other than that creature": read once, because every candidate is
        // asked the same question about the same target.
        let excluded = query
            .excluding_target
            .zip(effect_context)
            .map(|(target, (object, scoped, _))| {
                Self::chosen_targets(object, scoped.target_slot(target)).collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let mut visitor = |candidate: Target| {
            if excluded.contains(&candidate) {
                return ControlFlow::Continue(());
            }
            visitor(candidate)
        };
        if query.relative_position.is_none() && query.zones.contains(&ZoneKind::Battlefield) {
            for permanent in &self.battlefield {
                if !self.query_player_constraints_match(
                    Some(permanent.controller),
                    permanent.card.owner,
                    query,
                    (evaluation_controller, source),
                    context,
                    effect_context,
                ) {
                    continue;
                }
                let characteristics = prospective.map_or_else(
                    || self.trigger_event_object(permanent),
                    |prospective| {
                        self.trigger_event_object_with_prospective(permanent, prospective)
                    },
                );
                if self.trigger_object_matches(query.object, &characteristics, source, false)
                    && visitor(Target::Permanent(permanent.card.id)).is_break()
                {
                    return ControlFlow::Break(());
                }
            }
        }
        if query.relative_position.is_none() && query.zones.contains(&ZoneKind::Stack) {
            for candidate in self.stack.iter() {
                if candidate.kind != StackObjectKind::Spell
                    || !self.query_player_constraints_match(
                        Some(candidate.controller),
                        candidate.card.owner,
                        query,
                        (evaluation_controller, source),
                        context,
                        effect_context,
                    )
                {
                    continue;
                }
                let Some(characteristics) = self.stack_trigger_event_object(candidate) else {
                    continue;
                };
                if self.trigger_object_matches(query.object, &characteristics, source, true)
                    && visitor(Target::Spell(candidate.id)).is_break()
                {
                    return ControlFlow::Break(());
                }
            }
        }
        // The same card zones the target enumerator understands. Without this
        // a sweep over graveyards matched nothing and the clause was inert.
        for zone in [
            ZoneKind::Library,
            ZoneKind::Hand,
            ZoneKind::Graveyard,
            ZoneKind::Exile,
            ZoneKind::Command,
        ] {
            if !query.zones.contains(&zone) {
                continue;
            }
            for card in self.cards_in_zone(zone) {
                if !query.relative_position.is_none_or(|relative| {
                    self.query_relative_position_matches(
                        card.id,
                        relative,
                        source,
                        context,
                        effect_context,
                    )
                }) {
                    continue;
                }
                if self.query_player_constraints_match(
                    None,
                    card.owner,
                    query,
                    (evaluation_controller, source),
                    context,
                    effect_context,
                ) && self.card_object_matches(query.object, card, zone, source)
                    && visitor(Target::Card(card.id)).is_break()
                {
                    return ControlFlow::Break(());
                }
            }
        }
        ControlFlow::Continue(())
    }
}
