//! Attaching, detaching, and pairing.
//!
//! Auras, Equipment, Fortifications, soulbond, and reconfigure all answer the
//! same question -- what is attached to what -- so the whole family resolves
//! here rather than as six arms in the effect walk next door.

use super::super::{
    AttachmentDef, EffectResolutionContext, Game, ScopedEffect, StackObject, Target,
};

impl Game {
    pub(super) fn resolve_attachment_effect(
        &mut self,
        attachment: AttachmentDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) {
        match attachment {
            AttachmentDef::Attach { object: recipient }
            | AttachmentDef::AttachToSource { object: recipient } => {
                let onto_source = matches!(attachment, AttachmentDef::AttachToSource { .. });
                let Some(source) = object.source else {
                    return;
                };
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    let attached = match (onto_source, target) {
                        (true, Target::Permanent(id)) => self.try_attach(id, source),
                        (false, Target::Permanent(id)) => self.try_attach(source, id),
                        (false, Target::Player(player)) => {
                            self.try_attach_to_player(source, player)
                        }
                        (true, Target::Player(_) | Target::Card(_) | Target::Spell(_))
                        | (false, Target::Card(_) | Target::Spell(_)) => false,
                    };
                    if attached && !onto_source {
                        break;
                    }
                }
            }
            AttachmentDef::PairWithSource { object: recipient } => {
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
            AttachmentDef::Reconfigure { object: recipient } => {
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
            AttachmentDef::Unattach { object: recipient } => {
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Permanent(attachment) = target {
                        self.unattach(attachment);
                    }
                }
            }
        }
    }
}
