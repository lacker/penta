//! Attaching, detaching, and pairing.
//!
//! Auras, Equipment, Fortifications, soulbond, and reconfigure all answer the
//! same question -- what is attached to what -- so their procedures stay in
//! one focused module even though each remains a direct effect variant.

use super::super::{
    EffectRecipientDef, EffectResolutionContext, Game, ScopedEffect, StackObject, Target,
};

impl Game {
    pub(super) fn resolve_attach_effect(
        &mut self,
        recipient: EffectRecipientDef,
        onto_source: bool,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        let Some(source) = object.source else {
            return;
        };
        for target in self.effect_recipients(recipient, object, context, scoped) {
            let attached = match (onto_source, target) {
                (true, Target::Permanent(id)) => self.try_attach(id, source),
                (false, Target::Permanent(id)) => self.try_attach(source, id),
                (false, Target::Player(player)) => self.try_attach_to_player(source, player),
                (true, Target::Player(_) | Target::Card(_) | Target::Spell(_))
                | (false, Target::Card(_) | Target::Spell(_)) => false,
            };
            if attached && !onto_source {
                break;
            }
        }
    }

    pub(super) fn resolve_pair_with_source(
        &mut self,
        recipient: EffectRecipientDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        let Some(source) = object.source else {
            return;
        };
        let partner = self
            .effect_recipients(recipient, object, context, scoped)
            .into_iter()
            .find_map(|target| match target {
                Target::Permanent(id) => Some(id),
                _ => None,
            });
        if let Some(partner) = partner {
            self.pair_creatures(source, partner);
        }
    }

    pub(super) fn resolve_reconfigure_effect(
        &mut self,
        recipient: EffectRecipientDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        let Some(source) = object.source else {
            return;
        };
        let host = self
            .effect_recipients(recipient, object, context, scoped)
            .into_iter()
            .find_map(|target| match target {
                Target::Permanent(id) => Some(id),
                _ => None,
            });
        if let Some(host) = host {
            self.try_attach(source, host);
        } else {
            self.unattach(source);
        }
    }

    pub(super) fn resolve_unattach_effect(
        &mut self,
        recipient: EffectRecipientDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        for target in self.effect_recipients(recipient, object, context, scoped) {
            if let Target::Permanent(attachment) = target {
                self.unattach(attachment);
            }
        }
    }
}
