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
        let attachment = attachment.and_then(|attachment| match attachment {
            ArrivalAttachmentDef::SourceToArrival => {
                object.source.map(ArrivalAttachment::SourceToArrival)
            }
            ArrivalAttachmentDef::ArrivalToHost(reference) => self
                .object_reference_target(reference, object, context, scoped)
                .and_then(|target| match target {
                    Target::Permanent(host) => Some(ArrivalAttachment::ArrivalToHost(host)),
                    _ => None,
                }),
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
        // Resolved once rather than per target: "with a counter on it" reads
        // the same number for everything the clause moves.
        let arriving_counters = counters.map(|counters| {
            (
                counters.kind,
                u16::try_from(
                    self.effect_value(counters.amount, object, context, scoped)
                        .max(0),
                )
                .unwrap_or(u16::MAX),
            )
        });
        for target in self.effect_recipients(recipient, object, context, scoped) {
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
}
