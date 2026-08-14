//! Shared attachment relation and legality.
//!
//! Auras, Equipment, and Fortifications all use the same `attached_to`
//! relation. Their legal hosts and the consequence of becoming illegal differ,
//! which belongs here rather than being inferred independently by every
//! resolving effect and state-based action.

use super::{CardType, Game, GameObjectId, Permanent};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum AttachmentKind {
    Aura,
    Equipment,
    Fortification,
}

impl Game {
    /// The attachment category a permanent currently presents as.
    pub(super) fn attachment_kind(&self, permanent: &Permanent) -> Option<AttachmentKind> {
        if self.is_aura_permanent(permanent) {
            return Some(AttachmentKind::Aura);
        }
        let subtypes = self.effective_subtypes(permanent);
        if subtypes.contains(&"Equipment") {
            Some(AttachmentKind::Equipment)
        } else if subtypes.contains(&"Fortification") {
            Some(AttachmentKind::Fortification)
        } else {
            None
        }
    }

    pub(super) fn has_reconfigure(&self, permanent: &Permanent) -> bool {
        let mut found = false;
        self.for_each_effective_ability(permanent, |effective| {
            found |= effective
                .ability
                .declarative_effect()
                .is_some_and(Self::effect_is_reconfigure);
        });
        found
    }

    pub(super) const fn effect_is_reconfigure(effect: super::EffectDef) -> bool {
        matches!(effect, super::EffectDef::Reconfigure { .. })
    }

    /// Whether `attachment` may attach to `host` under its current
    /// characteristics. This intentionally does not impose the controller
    /// restriction of equip or fortify: that restriction is a property of
    /// those activated abilities' target declarations, while other effects
    /// may attach an Equipment or Fortification to an opponent's object.
    pub(super) fn is_legal_attachment_host(
        &self,
        attachment: &Permanent,
        host: GameObjectId,
    ) -> bool {
        match self.attachment_kind(attachment) {
            Some(AttachmentKind::Aura) => self.is_legal_aura_host(attachment, host),
            Some(AttachmentKind::Equipment) => self.battlefield.iter().any(|candidate| {
                candidate.card.id == host
                    && self
                        .permanent_types(candidate)
                        .is_some_and(|types| types.contains(CardType::Creature))
            }),
            Some(AttachmentKind::Fortification) => self.battlefield.iter().any(|candidate| {
                candidate.card.id == host
                    && self
                        .permanent_types(candidate)
                        .is_some_and(|types| types.contains(CardType::Land))
            }),
            None => false,
        }
    }

    /// Attach one existing permanent to an eligible battlefield object.
    /// Reattaching to the same object is a rules no-op. A successful change
    /// receives a new timestamp because attachment-dependent continuous
    /// effects can participate in timestamp order.
    pub(super) fn try_attach(&mut self, attachment: GameObjectId, host: GameObjectId) -> bool {
        let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == attachment)
            .cloned()
        else {
            return false;
        };
        if permanent.attached_to == Some(host) || !self.is_legal_attachment_host(&permanent, host) {
            return false;
        }
        let timestamp = self.allocate_continuous_effect_timestamp();
        let reconfigured = self.attachment_kind(&permanent) == Some(AttachmentKind::Equipment)
            && self.has_reconfigure(&permanent);
        let Some(permanent) = self
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == attachment)
        else {
            return false;
        };
        permanent.attached_to = Some(host);
        permanent.timestamp = timestamp;
        permanent.reconfigured_timestamp = reconfigured.then_some(timestamp);
        true
    }

    /// Detach an Equipment or Fortification without changing zones.
    pub(super) fn unattach(&mut self, attachment: GameObjectId) -> bool {
        let timestamp = self.allocate_continuous_effect_timestamp();
        let Some(permanent) = self
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == attachment)
        else {
            return false;
        };
        if permanent.attached_to.is_none() {
            return false;
        }
        permanent.attached_to = None;
        permanent.reconfigured_timestamp = None;
        permanent.timestamp = timestamp;
        true
    }
}
