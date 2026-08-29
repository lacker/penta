//! Dealing and publishing combat damage.
//!
//! Split out of the parent module for the source-size budget.

#![allow(clippy::wildcard_imports)]

use super::*;

impl Game {
    /// Test and card-local entry point for one combat assignment. Full combat
    /// collects every assignment first and uses the same batch primitive.
    #[cfg(test)]
    pub(in crate::game) fn deal_combat_damage_to_player(
        &mut self,
        attacker: GameObjectId,
        player: PlayerId,
        amount: u16,
    ) {
        self.deal_damage_simultaneously(vec![DamageAssignment {
            source: Some(attacker),
            target: Some(Target::Player(player)),
            amount,
            combat: true,
        }]);
    }

    /// How much life a drain can take from a recipient: what it had before
    /// the damage, which is all it can give however much is dealt.
    pub(in crate::game) fn drainable_from(&self, target: Target) -> u16 {
        match target {
            Target::Player(player) => self.players[player.index()].life.max(0).cast_unsigned(),
            Target::Permanent(id) => self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == id)
                .and_then(|permanent| {
                    if self
                        .permanent_types(permanent)
                        .is_some_and(|types| types.contains(CardType::Planeswalker))
                    {
                        return Some(permanent.counters(CounterKind::Loyalty));
                    }
                    self.toughness(permanent)
                        .map(|value| value.max(0).cast_unsigned())
                })
                .unwrap_or(0),
            Target::Card(_) | Target::Spell(_) => 0,
        }
    }

    /// A blocked attacker's part of the combat-damage event, divided among its
    /// blockers and whatever trample spills onto. Nothing is dealt here: the
    /// caller collects every attacker's and blocker's assignments before the
    /// shared simultaneous commit.
    pub(in crate::game) fn attacker_combat_damage_assignments(
        &self,
        attacker_id: GameObjectId,
        attacker_index: usize,
        blockers: &[GameObjectId],
    ) -> Vec<DamageAssignment> {
        let assignments = self.battlefield[attacker_index]
            .combat_damage_assignment
            .clone();
        let split = if assignments.is_empty() {
            self.default_damage_split(attacker_id, blockers)
        } else {
            assignments
                .into_iter()
                .map(|assignment| (assignment.recipient, assignment.amount))
                .collect()
        };
        split
            .into_iter()
            .map(|(recipient, amount)| DamageAssignment {
                source: Some(attacker_id),
                target: Some(recipient),
                amount,
                combat: true,
            })
            .collect()
    }

    /// Every blocker's combat damage, divided among the attackers it blocks.
    ///
    /// A pass of its own rather than part of each attacker's exchange: a
    /// creature blocking two attackers deals its power once between them, and
    /// running this inside the attacker loop would deal it once per attacker.
    pub(in crate::game) fn blocker_combat_damage_assignments(&self) -> Vec<DamageAssignment> {
        let blockers: Vec<_> = self
            .battlefield
            .iter()
            .filter(|permanent| {
                permanent.is_blocking_anything()
                    && self.deals_damage_in_current_combat_step(permanent)
            })
            .map(|permanent| {
                (
                    permanent.card.id,
                    permanent.combat_damage_assignment.clone(),
                )
            })
            .collect();
        let mut damage = Vec::new();
        for (blocker, assignments) in blockers {
            let split = if assignments.is_empty() {
                let recipients: Vec<_> = self
                    .combat_damage_recipients(blocker)
                    .into_iter()
                    .filter_map(|target| match target {
                        Target::Permanent(id) => Some(id),
                        Target::Player(_) | Target::Card(_) | Target::Spell(_) => None,
                    })
                    .collect();
                self.default_damage_split(blocker, &recipients)
            } else {
                assignments
                    .into_iter()
                    .map(|assignment| (assignment.recipient, assignment.amount))
                    .collect()
            };
            damage.extend(
                split
                    .into_iter()
                    .map(|(recipient, amount)| DamageAssignment {
                        source: Some(blocker),
                        target: Some(recipient),
                        amount,
                        combat: true,
                    }),
            );
        }
        damage
    }

    pub(in crate::game) fn combat_defender(attacker: &Permanent) -> AttackDefender {
        attacker
            .attack_defender
            .unwrap_or(AttackDefender::Player(attacker.controller.opponent()))
    }

    pub(in crate::game) fn combat_defender_target(&self, attacker: &Permanent) -> Option<Target> {
        match Self::combat_defender(attacker) {
            AttackDefender::Player(player) => Some(Target::Player(player)),
            AttackDefender::Planeswalker(id) => self
                .battlefield
                .iter()
                .find(|permanent| {
                    permanent.card.id == id
                        && permanent.controller != attacker.controller
                        && self
                            .permanent_types(permanent)
                            .is_some_and(|types| types.contains(CardType::Planeswalker))
                })
                .map(|permanent| Target::Permanent(permanent.card.id)),
        }
    }
}
