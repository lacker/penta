//! Moving objects between zones, and everything an arrival on the
//! battlefield carries with it.
//!
//! Split out of the parent module for the source-size budget: what belongs
//! here is one clause whose parameters all describe the same arrival.

use super::{
    ArrivalAttachment, ArrivalAttachmentDef, BattlefieldArrival, EffectDef,
    EffectResolutionContext, Game, ScopedEffect, StackObject, Target, ZoneKind, ZoneMoveCause,
    ZonePlacement,
};
use crate::card::{
    BattlefieldArrivalDef, CounterKind, EffectRecipientDef, PlayerRelation, TokenCountersDef,
};

/// How a permanent this effect moves arrives, when it arrives at all.
/// "Under your control" and "attach this to it" both belong to the arrival:
/// what enters is a new object, so neither can wait for a later step.
fn battlefield_arrival(
    owner: crate::PlayerId,
    arriving_controller: Option<crate::PlayerId>,
    attachment: Option<ArrivalAttachment>,
    counters: Option<(CounterKind, u16)>,
    modifications: &'static [crate::card::BattlefieldEntryModificationDef],
) -> Option<BattlefieldArrival> {
    if arriving_controller.is_none()
        && attachment.is_none()
        && counters.is_none()
        && modifications.is_empty()
    {
        return None;
    }
    let controller = arriving_controller.unwrap_or(owner);
    let arrival = BattlefieldArrival::under(controller).with_modifications(modifications);
    let arrival = match attachment {
        Some(ArrivalAttachment::SourceToArrival(source)) => arrival.attaching(source),
        Some(ArrivalAttachment::ArrivalToHost(host)) => arrival.attached_to(host),
        Some(ArrivalAttachment::ArrivalToPlayer(player)) => arrival.attached_to_player(player),
        None => arrival,
    };
    Some(arrival.with_counters(counters))
}

/// One authored "put this there" clause, gathered so the resolution takes an
/// arrival rather than six loose parameters.
#[derive(Clone, Copy)]
pub(super) struct MoveToZoneClause {
    pub(super) recipient: EffectRecipientDef,
    pub(super) zone: ZoneKind,
    pub(super) controller: Option<PlayerRelation>,
    pub(super) placement: ZonePlacement,
    pub(super) attachment: Option<ArrivalAttachmentDef>,
    pub(super) counters: Option<TokenCountersDef>,
    pub(super) modifications: &'static [crate::card::BattlefieldEntryModificationDef],
}

impl Game {
    pub(super) fn resolve_move_to_zone_effect(
        &mut self,
        effect: EffectDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        let (move_effect, arrival) = match effect {
            effect @ EffectDef::MoveToZone { .. } => (effect, BattlefieldArrivalDef::DEFAULT),
            EffectDef::WithBattlefieldArrival {
                effect: move_effect @ &EffectDef::MoveToZone { .. },
                arrival,
            } => (*move_effect, arrival),
            EffectDef::WithBattlefieldArrival { .. } => {
                unreachable!("battlefield arrival must wrap a zone move")
            }
            _ => unreachable!("move-to-zone resolver received another effect"),
        };
        let EffectDef::MoveToZone {
            object: recipient,
            zone,
            placement,
        } = move_effect
        else {
            unreachable!("move effect was checked above")
        };
        self.resolve_move_to_zone(
            MoveToZoneClause {
                recipient,
                zone,
                controller: arrival.controller,
                placement,
                attachment: arrival.attachment,
                counters: arrival.counters,
                modifications: arrival.modifications,
            },
            object,
            context,
            scoped,
        );
    }

    pub(super) fn resolve_zone_move_result(
        &mut self,
        effect: &'static EffectDef,
        binding: crate::ObjectSetBindingIndex,
        then: &'static EffectDef,
        object: &StackObject,
        mut context: EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        let move_recipient = match effect {
            EffectDef::MoveToZone {
                object: recipient, ..
            } => Some(*recipient),
            EffectDef::WithBattlefieldArrival { effect: inner, .. } => match **inner {
                EffectDef::MoveToZone {
                    object: recipient, ..
                } => Some(recipient),
                _ => unreachable!("battlefield arrival must wrap a zone move"),
            },
            _ => None,
        };
        if let Some(recipient) = move_recipient {
            let moved = self.effect_recipients(recipient, object, &context, scoped);
            context.bind_object_group(binding, moved);
            self.resolve_effects_in_order(
                vec![scoped.with_effect(*effect), scoped.with_effect(*then)],
                object,
                context,
            );
            return;
        }

        match effect {
            EffectDef::ReturnLinkedExiles {
                object: predicate, ..
            } => {
                let moved = self.matching_linked_exiles(*predicate, object);
                context.bind_object_group(binding, moved);
                self.resolve_effects_in_order(
                    vec![scoped.with_effect(*effect), scoped.with_effect(*then)],
                    object,
                    context,
                );
            }
            EffectDef::ChooseCards {
                player: recipient,
                sources,
                object: predicate,
                minimum,
                maximum,
                reveal,
                destination,
                placement,
            } => {
                let source = object.source.unwrap_or(object.id);
                let mut queued = false;
                for target in self.effect_recipients(*recipient, object, &context, scoped) {
                    if let Target::Player(player) = target {
                        queued |= self.queue_owned_card_choice(
                            player,
                            sources,
                            *predicate,
                            *minimum,
                            *maximum,
                            *reveal,
                            *destination,
                            *placement,
                            Some((object.clone(), context.clone(), scoped)),
                            source,
                            object.controller,
                        );
                    }
                }
                if !queued {
                    context.bind_object_group(binding, Vec::new());
                    self.resolve_effect_def(scoped.with_effect(*then), object, context);
                }
            }
            _ => unreachable!("zone-move result must wrap a zone-moving effect"),
        }
    }

    fn resolved_arrival_counters(
        &self,
        counters: Option<TokenCountersDef>,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> Option<(CounterKind, u16)> {
        counters.map(|counters| {
            (
                counters.kind,
                u16::try_from(
                    self.effect_value(counters.amount, object, context, scoped)
                        .max(0),
                )
                .unwrap_or(u16::MAX),
            )
        })
    }

    fn batch_exile_permanents(&mut self, recipients: &[Target], zone: ZoneKind) -> bool {
        let batch = zone == ZoneKind::Exile;
        if !batch {
            return false;
        }
        let permanents = recipients
            .iter()
            .filter_map(|target| match target {
                Target::Permanent(id) => Some(*id),
                Target::Card(_) | Target::Player(_) | Target::Spell(_) => None,
            })
            .collect::<Vec<_>>();
        self.exile_permanents(&permanents);
        true
    }

    fn move_simultaneous_library_batch(
        &mut self,
        recipients: &[Target],
        clause: MoveToZoneClause,
        attachment: Option<ArrivalAttachment>,
    ) -> bool {
        if clause.zone != ZoneKind::Library
            || clause.controller.is_some()
            || !clause.modifications.is_empty()
            || attachment.is_some()
            || clause.counters.is_some()
        {
            return false;
        }
        let Some(permanents) = recipients
            .iter()
            .map(|target| match target {
                Target::Permanent(id) => Some(*id),
                Target::Player(_) | Target::Card(_) | Target::Spell(_) => None,
            })
            .collect::<Option<Vec<_>>>()
        else {
            return false;
        };
        self.move_permanents_to_zone(&permanents, clause.zone, clause.placement);
        true
    }

    pub(super) fn resolve_move_to_zone(
        &mut self,
        clause: MoveToZoneClause,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        let MoveToZoneClause {
            recipient,
            zone,
            controller,
            placement,
            attachment,
            counters,
            modifications,
        } = clause;
        let attachment = attachment
            .and_then(|attachment| self.arrival_attachment(attachment, object, context, scoped));
        // A host that is gone is not the same answer for everything that
        // attaches. An Aura cannot enter unattached (CR 303.4f) and does not
        // enter at all; an Equipment simply arrives bare -- Sword of the
        // Meek's own ruling says so outright. Which it is depends on the card
        // being moved, so the decision waits for the loop below.
        let lost_its_host = clause.attachment.is_some() && attachment.is_none();
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
        // Resolved once rather than per target: "with a counter on it" reads
        // the same number for everything the clause moves.
        let arriving_counters = self.resolved_arrival_counters(counters, object, context, scoped);
        let recipients = self.effect_recipients(recipient, object, context, scoped);
        let batch_exile = self.batch_exile_permanents(&recipients, zone);
        // A library sweep is one simultaneous event, not a run of unrelated
        // one-object moves. Keep the whole prospective batch together so
        // replacement effects see the same battlefield and CR 401.4 can ask
        // each owner for the relative order at the instructed position.
        if self.move_simultaneous_library_batch(&recipients, clause, attachment) {
            return;
        }
        for target in recipients {
            if batch_exile && matches!(target, Target::Permanent(_)) {
                continue;
            }
            let owner = match target {
                Target::Permanent(id) => self
                    .battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == id)
                    .map(|permanent| permanent.card.owner),
                Target::Spell(id) => self
                    .stack
                    .iter()
                    .find(|candidate| candidate.id == id)
                    .map(|candidate| candidate.card.owner),
                Target::Card(id) => self
                    .card_in_nonbattlefield_zone(id)
                    .map(|(_, card)| card.owner),
                Target::Player(_) => None,
            };
            // An Aura whose host is gone stays where it is; anything else
            // that attaches arrives bare.
            if lost_its_host && self.moving_card_is_an_aura(target) {
                continue;
            }
            self.move_target_to_zone(
                target,
                zone,
                ZoneMoveCause::Effect {
                    controller: object.controller,
                },
                // "Under your control" and "attach this to it" both belong to
                // the arrival: a permanent that enters is a new object, so
                // neither can wait for a later step.
                battlefield_arrival(
                    owner.unwrap_or(object.controller),
                    arriving_controller,
                    attachment,
                    arriving_counters,
                    modifications,
                ),
                placement,
            );
        }
    }

    /// The attachment an arrival carries, or `None` when what it named is no
    /// longer there.
    fn arrival_attachment(
        &mut self,
        attachment: ArrivalAttachmentDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> Option<ArrivalAttachment> {
        match attachment {
            ArrivalAttachmentDef::SourceToArrival => {
                object.source.map(ArrivalAttachment::SourceToArrival)
            }
            ArrivalAttachmentDef::ArrivalToHost(reference) => self
                .object_reference_target(reference, object, context, scoped)
                .and_then(|target| match target {
                    Target::Permanent(host) => Some(ArrivalAttachment::ArrivalToHost(host)),
                    _ => None,
                }),
            ArrivalAttachmentDef::ArrivalToPlayer(reference) => self
                .player_reference(reference, object, context, scoped)
                .map(ArrivalAttachment::ArrivalToPlayer),
        }
    }

    /// Whether what is being moved is an Aura, which is the one thing that
    /// cannot arrive without the host its clause named.
    fn moving_card_is_an_aura(&self, target: Target) -> bool {
        if let Target::Permanent(id) = target {
            return self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == id)
                .is_some_and(|permanent| self.is_aura_permanent(permanent));
        }
        let definition = match target {
            Target::Card(id) => self
                .card_in_nonbattlefield_zone(id)
                .map(|(_, card)| card.definition),
            Target::Spell(id) => self
                .stack
                .iter()
                .find(|candidate| candidate.id == id)
                .and_then(|candidate| candidate.card.definition.card_definition()),
            Target::Permanent(_) | Target::Player(_) => None,
        };
        definition
            .and_then(|definition| self.catalog.get(definition))
            .is_some_and(|card| card.rules.has_subtype("Aura"))
    }
}
