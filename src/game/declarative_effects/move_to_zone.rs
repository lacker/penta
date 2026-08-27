//! Moving objects between zones, and everything an arrival on the
//! battlefield carries with it.
//!
//! Split out of the parent module for the source-size budget: what belongs
//! here is one clause whose parameters all describe the same arrival.

use super::{
    ArrivalAttachment, ArrivalAttachmentDef, BattlefieldArrival, EffectResolutionContext, Game,
    ScopedEffect, StackObject, Target, ZoneKind, ZoneMoveCause, ZonePlacement,
};
use crate::card::{
    AppliedEffectDef, CounterKind, EffectRecipientDef, PlayerRelation, TokenCountersDef,
};

/// How a permanent this effect moves arrives, when it arrives at all.
/// "Under your control" and "attach this to it" both belong to the arrival:
/// what enters is a new object, so neither can wait for a later step.
fn battlefield_arrival(
    owner: crate::PlayerId,
    arriving_controller: Option<crate::PlayerId>,
    attachment: Option<ArrivalAttachment>,
    counters: Option<(CounterKind, u16)>,
    tapped: bool,
) -> Option<BattlefieldArrival> {
    if arriving_controller.is_none() && attachment.is_none() && counters.is_none() && !tapped {
        return None;
    }
    let controller = arriving_controller.unwrap_or(owner);
    let arrival = if tapped {
        BattlefieldArrival::tapped_under(controller)
    } else {
        BattlefieldArrival::under(controller)
    };
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
    pub(super) from: Option<ZoneKind>,
    pub(super) zone: ZoneKind,
    pub(super) controller: Option<PlayerRelation>,
    pub(super) placement: ZonePlacement,
    pub(super) arrival_effect: Option<&'static AppliedEffectDef>,
    pub(super) attachment: Option<ArrivalAttachmentDef>,
    pub(super) counters: Option<TokenCountersDef>,
    pub(super) tapped: bool,
}

impl Game {
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

    fn batch_exile_permanents(
        &mut self,
        recipients: &[Target],
        from: Option<ZoneKind>,
        zone: ZoneKind,
    ) -> bool {
        let batch =
            zone == ZoneKind::Exile && from.is_none_or(|from| from == ZoneKind::Battlefield);
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

    pub(super) fn resolve_move_to_zone(
        &mut self,
        clause: MoveToZoneClause,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        let MoveToZoneClause {
            recipient,
            from,
            zone,
            controller,
            placement,
            arrival_effect,
            attachment,
            counters,
            tapped,
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
        let batch_exile = self.batch_exile_permanents(&recipients, from, zone);
        for target in recipients {
            if batch_exile && matches!(target, Target::Permanent(_)) {
                continue;
            }
            let (actual_zone, owner) = match target {
                Target::Permanent(id) => self
                    .battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == id)
                    .map_or((None, None), |permanent| {
                        (Some(ZoneKind::Battlefield), Some(permanent.card.owner))
                    }),
                Target::Spell(id) => self
                    .stack
                    .iter()
                    .find(|candidate| candidate.id == id)
                    .map_or((None, None), |candidate| {
                        (Some(ZoneKind::Stack), Some(candidate.card.owner))
                    }),
                Target::Card(id) => self
                    .card_in_nonbattlefield_zone(id)
                    .map_or((None, None), |(zone, card)| (Some(zone), Some(card.owner))),
                Target::Player(_) => (None, None),
            };
            if from.is_some_and(|expected| actual_zone != Some(expected)) {
                continue;
            }
            // An Aura whose host is gone stays where it is; anything else
            // that attaches arrives bare.
            if lost_its_host && self.moving_card_is_an_aura(target) {
                continue;
            }
            let arrived = self.move_target_to_zone(
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
                    tapped,
                ),
                placement,
            );
            // Applied as the move happens: the identity a permanent gets on
            // arrival is not the one the card had in the graveyard it came
            // from, so a later effect would have nothing to name.
            if let (Some(effect), Some(arrived)) = (arrival_effect, arrived) {
                self.apply_arrival_effect(arrived, *effect, object, context, scoped);
            }
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
