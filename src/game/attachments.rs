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
        let subtypes = self.effective_subtypes(permanent);
        if subtypes.contains(&"Aura") {
            return Some(AttachmentKind::Aura);
        }
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
        self.is_legal_attachment_host_with_prospective_reconfigure(attachment, host, false)
    }

    fn is_legal_attachment_host_with_prospective_reconfigure(
        &self,
        attachment: &Permanent,
        host: GameObjectId,
        prospective_reconfigure: bool,
    ) -> bool {
        let kind = self.attachment_kind(attachment);
        let is_creature = self
            .permanent_types(attachment)
            .is_some_and(|types| types.contains(CardType::Creature));
        // An attached permanent that is also a creature must become
        // unattached. Reconfigure is the one prospective exception: attaching
        // it creates the timestamp that immediately removes Creature for as
        // long as it remains attached.
        if is_creature
            && !(prospective_reconfigure
                && kind == Some(AttachmentKind::Equipment)
                && self.has_reconfigure(attachment))
        {
            return false;
        }
        match kind {
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

    /// Soulbond two creatures. The relation is symmetric and recorded on
    /// both, and either already being paired makes this a no-op: soulbond
    /// pairs only unpaired creatures.
    pub(super) fn pair_creatures(&mut self, one: GameObjectId, other: GameObjectId) -> bool {
        if one == other {
            return false;
        }
        let both_free = [one, other].into_iter().all(|id| {
            self.battlefield
                .iter()
                .any(|permanent| permanent.card.id == id && permanent.paired_with.is_none())
        });
        if !both_free {
            return false;
        }
        for permanent in &mut self.battlefield {
            if permanent.card.id == one {
                permanent.paired_with = Some(other);
            } else if permanent.card.id == other {
                permanent.paired_with = Some(one);
            }
        }
        true
    }

    /// CR 702.94b: the pair lasts only while one player controls both, and
    /// only while both are creatures. Checked with the other state-based
    /// actions, so a partner leaving frees the survivor immediately.
    pub(super) fn break_illegal_pairings(&mut self) {
        let broken = self
            .battlefield
            .iter()
            .filter_map(|permanent| {
                let partner = permanent.paired_with?;
                let still_legal = self.battlefield.iter().any(|candidate| {
                    candidate.card.id == partner
                        && candidate.controller == permanent.controller
                        && candidate.paired_with == Some(permanent.card.id)
                        && self
                            .permanent_types(candidate)
                            .is_some_and(|types| types.contains(CardType::Creature))
                });
                (!still_legal).then_some(permanent.card.id)
            })
            .collect::<Vec<_>>();
        for permanent in &mut self.battlefield {
            if broken.contains(&permanent.card.id) {
                permanent.paired_with = None;
            }
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
        if permanent.attached_to == Some(host)
            || !self.is_legal_attachment_host_with_prospective_reconfigure(&permanent, host, true)
        {
            return false;
        }
        let timestamp = self.allocate_continuous_effect_timestamp();
        let reconfigured = self.attachment_kind(&permanent) == Some(AttachmentKind::Equipment)
            && self.has_reconfigure(&permanent);
        let became_aura = self.attachment_kind(&permanent) == Some(AttachmentKind::Aura);
        let Some(permanent) = self
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == attachment)
        else {
            return false;
        };
        permanent.attached_to = Some(host);
        // "It becomes an Aura" happens in the same resolution that attaches
        // it, so the two are recorded together and it stays one afterwards.
        permanent.became_aura = became_aura;
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
