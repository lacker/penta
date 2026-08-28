//! Taking control of a permanent through resolving and static effects.
//!
//! Resolving effects record what ends them -- cleanup or a source remaining in
//! the required state. Static attachment effects are instead reconciled live
//! before priority, using the attachment timestamp to select the newest one.

use super::{
    ControlDurationDef, DeclarativeAbilityDef, EffectDef, EffectRecipientDef,
    EffectResolutionContext, Game, GameObjectId, PlayerId, PlayerRefDef, ScopedEffect, StackObject,
    Target,
};

#[derive(Clone, Copy)]
struct StaticControlClaim {
    source: GameObjectId,
    target: GameObjectId,
    controller: PlayerId,
    timestamp: super::ContinuousEffectTimestamp,
}

impl Game {
    const fn is_attached_static_control(effect: EffectDef) -> bool {
        matches!(
            effect,
            EffectDef::GainControl {
                object: EffectRecipientDef::AttachedPermanent,
                controller: PlayerRefDef::EffectController,
                duration: ControlDurationDef::WhileSourceRemains {
                    while_tapped: false,
                },
            }
        )
    }

    fn attached_static_control_claims(&self) -> Vec<StaticControlClaim> {
        let mut claims = Vec::new();
        for source in &self.battlefield {
            let Some(target) = source.attached_to else {
                continue;
            };
            if !self
                .battlefield
                .iter()
                .any(|permanent| permanent.card.id == target)
            {
                continue;
            }
            self.for_each_effective_ability(source, |effective| {
                if matches!(
                    effective.ability.definition,
                    DeclarativeAbilityDef::Static(_)
                ) && effective
                    .ability
                    .declarative_effect()
                    .is_some_and(Self::is_attached_static_control)
                {
                    claims.push(StaticControlClaim {
                        source: source.card.id,
                        target,
                        controller: source.controller,
                        timestamp: source.timestamp,
                    });
                }
            });
        }
        claims.sort_by_key(|claim| claim.timestamp);
        claims
    }

    /// Reconciles layer-2 control effects supplied by static abilities. These
    /// never resolve and never use the stack: the newest applicable Aura
    /// controls what it is attached to for exactly as long as that ability
    /// remains applicable to that attachment.
    pub(super) fn reconcile_static_control_changes(&mut self) {
        let claims = self.attached_static_control_claims();
        let mut winners: Vec<StaticControlClaim> = Vec::new();
        for claim in claims {
            if let Some(current) = winners
                .iter_mut()
                .find(|current| current.target == claim.target)
            {
                *current = claim;
            } else {
                winners.push(claim);
            }
        }

        for permanent in &mut self.battlefield {
            if !permanent.control_requires_source_attached {
                continue;
            }
            let Some(source) = permanent.control_source else {
                debug_assert!(false, "an attachment-held control change has a source");
                permanent.control_requires_source_attached = false;
                continue;
            };
            let still_winning = winners
                .iter()
                .any(|claim| claim.source == source && claim.target == permanent.card.id);
            if still_winning {
                continue;
            }
            permanent.control_source = None;
            permanent.control_requires_source_tapped = false;
            permanent.control_requires_source_attached = false;
            if let Some(previous) = permanent.control_reverts_to.take() {
                permanent.controller = previous;
                permanent.suspend_haste = false;
                permanent.entered_controller_turn = self.turns_started[previous.index()];
            }
        }

        for claim in winners {
            let Some(index) = self
                .battlefield
                .iter()
                .position(|permanent| permanent.card.id == claim.target)
            else {
                continue;
            };
            let held_by_another_effect = self.battlefield[index]
                .control_source
                .is_some_and(|source| source != claim.source)
                || (self.battlefield[index].control_source.is_none()
                    && self.battlefield[index].control_reverts_to.is_some());
            if held_by_another_effect
                || (self.battlefield[index].controller != claim.controller
                    && self.cannot_change_controller(&self.battlefield[index]))
            {
                continue;
            }
            let permanent = &mut self.battlefield[index];
            if permanent.controller != claim.controller {
                permanent
                    .control_reverts_to
                    .get_or_insert(permanent.controller);
                permanent.controller = claim.controller;
                permanent.suspend_haste = false;
                permanent.entered_controller_turn = self.turns_started[claim.controller.index()];
            }
            permanent.control_source = Some(claim.source);
            permanent.control_requires_source_tapped = false;
            permanent.control_requires_source_attached = true;
        }
    }

    /// The shared body of both control-change durations.
    pub(super) fn take_control_of(
        &mut self,
        recipient: EffectRecipientDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
        duration: ControlDurationDef,
        controller: PlayerId,
    ) {
        let holder = match duration {
            ControlDurationDef::UntilEndOfTurn | ControlDurationDef::Indefinitely => None,
            ControlDurationDef::WhileSourceRemains { while_tapped } => {
                Some((object.source.unwrap_or(object.id), while_tapped))
            }
        };
        for target in self.effect_recipients(recipient, object, context, scoped) {
            let Target::Permanent(id) = target else {
                continue;
            };
            let Some(index) = self
                .battlefield
                .iter()
                .position(|permanent| permanent.card.id == id)
            else {
                continue;
            };
            if self.battlefield[index].controller == controller
                || self.cannot_change_controller(&self.battlefield[index])
            {
                continue;
            }
            let permanent = &mut self.battlefield[index];
            // Only the first change records where control came from, so
            // passing a permanent around and back still returns it to whoever
            // had it before the turn started. An indefinite change records
            // nothing: there is nothing for cleanup to give back, and an
            // earlier turn-scoped change over the same permanent still ends
            // the way it was going to.
            if duration != ControlDurationDef::Indefinitely {
                permanent
                    .control_reverts_to
                    .get_or_insert(permanent.controller);
            }
            permanent.controller = controller;
            permanent.suspend_haste = false;
            permanent.control_source = holder.map(|(id, _)| id);
            permanent.control_requires_source_tapped = holder.is_some_and(|(_, tapped)| tapped);
            permanent.control_requires_source_attached = false;
            // It has not been under its new controller's control since their
            // turn began, so it is summoning sick unless something grants
            // haste. This is why the cards that steal a creature almost always
            // grant it too.
            permanent.entered_controller_turn = self.turns_started[controller.index()];
        }
    }
}

impl Game {
    /// Swap who controls two permanents. Both controllers are read before
    /// either moves: doing it as two ordinary control changes would let the
    /// first one change the answer the second needs.
    pub(super) fn exchange_control_of(
        &mut self,
        first: EffectRecipientDef,
        second: EffectRecipientDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> bool {
        let one = self.single_permanent_recipient(first, object, context, scoped);
        let other = self.single_permanent_recipient(second, object, context, scoped);
        let (Some(one), Some(other)) = (one, other) else {
            return false;
        };
        if one == other {
            return false;
        }
        let controllers = [one, other].map(|id| {
            self.battlefield
                .iter()
                .find(|permanent| permanent.card.id == id)
                .map(|permanent| permanent.controller)
        });
        let ([Some(one_controller), Some(other_controller)], false) = (
            controllers,
            [one, other].iter().any(|id| {
                self.battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == *id)
                    .is_some_and(|permanent| self.cannot_change_controller(permanent))
            }),
        ) else {
            return false;
        };
        if one_controller == other_controller {
            return false;
        }
        for (id, controller) in [(one, other_controller), (other, one_controller)] {
            let turns_started = self.turns_started[controller.index()];
            let Some(permanent) = self
                .battlefield
                .iter_mut()
                .find(|permanent| permanent.card.id == id)
            else {
                continue;
            };
            permanent.controller = controller;
            permanent.suspend_haste = false;
            // An exchange lasts indefinitely, so nothing is recorded for
            // cleanup to give back. It is still a new controller who has not
            // had it since their turn began, which is what makes it
            // summoning sick.
            permanent.entered_controller_turn = turns_started;
        }
        true
    }

    fn single_permanent_recipient(
        &self,
        recipient: EffectRecipientDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> Option<GameObjectId> {
        let mut found = self
            .effect_recipients(recipient, object, context, scoped)
            .into_iter()
            .filter_map(|target| match target {
                Target::Permanent(id) => Some(id),
                Target::Card(_) | Target::Player(_) | Target::Spell(_) => None,
            });
        let first = found.next()?;
        found.next().is_none().then_some(first)
    }
}
