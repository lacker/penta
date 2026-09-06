//! Who divides combat damage, and among what.
//!
//! An attacker with several blockers divides its damage among them, and so
//! does a creature blocking several attackers -- which happens when a card
//! grants an extra block, or when one blocker takes a whole band. Banding
//! moves either of those choices to the controller of the creature with
//! banding, which is the only reason the two directions share a queue.

use super::super::{
    Action, BandingQuality, CombatDamageAssignment, Game, GameObjectId, KeywordAbility, PlayerId,
    Target,
};
use super::damage_distributions;
use crate::card::AppliedRuleDef;

impl Game {
    /// Who divides this creature's combat damage. CR 702.21: a creature with
    /// banding on the other side of the block takes that choice, so its
    /// controller decides rather than the creature's own. Any one creature
    /// with banding is enough.
    ///
    /// For an attacker that is a blocker with banding; for a blocker it is one
    /// of the attackers it is blocking, so the same rule reads in both
    /// directions from whichever creature is dealing the damage.
    pub(in crate::game) fn combat_damage_assigner(&self, source: GameObjectId) -> PlayerId {
        let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
        else {
            return self.active_player;
        };
        let opposition: Vec<_> = self
            .combat_damage_recipients(source)
            .into_iter()
            .filter_map(|target| match target {
                Target::Permanent(id) => Some(id),
                Target::Player(_) | Target::Card(_) | Target::Spell(_) => None,
            })
            .collect();

        // Plain banding: one creature with it on the other side is enough.
        let banded = opposition.iter().find_map(|id| {
            let other = self.battlefield.iter().find(|other| other.card.id == *id)?;
            self.permanent_has_executable_keyword(other, KeywordAbility::Banding)
                .then_some(other.controller)
        });
        if let Some(controller) = banded {
            return controller;
        }

        // "Bands with other" asks for two: the printed rule is at least two
        // creatures of the quality, one of which carries the ability. One
        // legendary creature alone does not take the choice.
        for quality in BandingQuality::ALL {
            let qualifying: Vec<_> = opposition
                .iter()
                .copied()
                .filter(|id| self.matches_banding_quality(*id, quality))
                .collect();
            if qualifying.len() >= 2
                && qualifying
                    .iter()
                    .any(|id| self.has_bands_with_other(*id, quality))
                && let Some(controller) = qualifying.first().and_then(|id| {
                    self.battlefield
                        .iter()
                        .find(|other| other.card.id == *id)
                        .map(|other| other.controller)
                })
            {
                return controller;
            }
        }
        permanent.controller
    }

    /// The creatures this one deals its combat damage to, in object order: an
    /// attacker's blockers, or a blocker's attackers.
    pub(in crate::game) fn combat_damage_recipients(&self, source: GameObjectId) -> Vec<Target> {
        let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
        else {
            return Vec::new();
        };
        let mut recipients: Vec<_> = if permanent.attacking {
            self.battlefield
                .iter()
                .filter(|other| other.is_blocking(source))
                .map(|other| Target::Permanent(other.card.id))
                .collect()
        } else {
            permanent
                .blocking
                .iter()
                .filter(|attacker| {
                    self.battlefield
                        .iter()
                        .any(|other| other.card.id == **attacker && other.attacking)
                })
                .map(|attacker| Target::Permanent(*attacker))
                .collect()
        };
        recipients.sort_unstable();
        recipients
    }

    pub(in crate::game) fn combat_assignment_actions(
        &self,
        source_id: GameObjectId,
    ) -> Vec<Action> {
        let Some(source) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source_id)
        else {
            return Vec::new();
        };
        let power = self
            .combat_assigned_power(source)
            .unwrap_or(0)
            .max(0)
            .cast_unsigned();
        // Trample spills past blockers onto what the creature is attacking. A
        // blocker is attacking nothing, so trample on one has nowhere to go.
        let trample = source.attacking && self.has_trample(source);
        let mut recipients = self.combat_damage_recipients(source_id);
        let blocker_count = recipients.len();
        let defender_index = trample
            .then(|| self.combat_defender_target(source))
            .flatten()
            .map(|defender| {
                let index = recipients.len();
                recipients.push(defender);
                index
            });

        // "As though it weren't blocked" is one extra division rather than a
        // change to the combat: everything on the defender and nothing on the
        // blockers, offered alongside the ordinary splits.
        let unblocked_assignment = (source.attacking
            && blocker_count > 0
            && self.has_applied_rule(
                source,
                AppliedRuleDef::MayAssignCombatDamageAsThoughUnblocked,
            ))
        .then(|| self.combat_defender_target(source))
        .flatten()
        .map(|defender| Action::AssignCombatDamage {
            attacker: source_id,
            assignments: recipients
                .iter()
                .take(blocker_count)
                .map(|recipient| CombatDamageAssignment {
                    recipient: *recipient,
                    amount: 0,
                })
                .chain(std::iter::once(CombatDamageAssignment {
                    recipient: defender,
                    amount: power,
                }))
                .collect(),
        });

        damage_distributions(recipients.len(), power)
            .into_iter()
            .filter(|amounts| {
                let blockers = || {
                    recipients
                        .iter()
                        .take(blocker_count)
                        .zip(amounts)
                        .filter_map(|(target, amount)| match target {
                            Target::Permanent(id) => Some((*id, *amount)),
                            Target::Player(_) | Target::Card(_) | Target::Spell(_) => None,
                        })
                };
                // CR 702.19b: trample only spills once every blocker has
                // lethal damage assigned. Without defender damage, current
                // CR 510.1c permits any division among the blockers.
                let defender_damage = defender_index
                    .and_then(|index| amounts.get(index))
                    .copied()
                    .unwrap_or(0);
                if defender_damage == 0 {
                    return true;
                }
                blockers().all(|(id, amount)| amount >= self.lethal_damage_from(id, source_id))
            })
            .map(|amounts| Action::AssignCombatDamage {
                attacker: source_id,
                assignments: recipients
                    .iter()
                    .copied()
                    .zip(amounts)
                    .map(|(recipient, amount)| CombatDamageAssignment { recipient, amount })
                    .collect(),
            })
            .chain(unblocked_assignment)
            .collect()
    }

    /// How an unassigned creature spreads its damage: enough to kill each
    /// recipient in turn, then the remainder over the top when it can trample
    /// onto its defender, or onto the last recipient otherwise.
    pub(in crate::game) fn default_damage_split(
        &self,
        source_id: GameObjectId,
        recipients: &[GameObjectId],
    ) -> Vec<(Target, u16)> {
        let Some(source) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source_id)
        else {
            return Vec::new();
        };
        let mut remaining = self
            .combat_assigned_power(source)
            .unwrap_or(0)
            .max(0)
            .cast_unsigned();
        let trample = source.attacking && self.has_trample(source);
        let mut split = Vec::with_capacity(recipients.len() + 1);
        for recipient in recipients {
            let amount = self
                .lethal_damage_from(*recipient, source_id)
                .min(remaining);
            remaining -= amount;
            split.push((Target::Permanent(*recipient), amount));
        }
        if remaining > 0 {
            if trample && let Some(defender) = self.combat_defender_target(source) {
                split.push((defender, remaining));
            } else if let Some(last) = split.last_mut() {
                last.1 += remaining;
            }
        }
        split
    }
}
