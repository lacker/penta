//! Blocker eligibility, participant-scoped restrictions, and declaration costs.

use std::ops::ControlFlow;

use crate::card::{BlockRestrictionDef, BlockRestrictionMatchDef, BlockRestrictionSubjectDef};

use super::{
    super::{AppliedRuleDef, Game, GameObjectId, ManaCost, Permanent, PlayerId},
    add_declaration_cost,
};

impl Game {
    /// Whether the restrictions printed on either participant permit this
    /// blocker-attacker pairing. Declaration costs permit the pair here and
    /// are checked against the complete proposed declaration separately.
    pub(super) fn block_pair_is_allowed(&self, blocker: &Permanent, attacker: &Permanent) -> bool {
        !self.participant_prohibits_block(blocker, attacker, BlockRestrictionSubjectDef::Blocker)
            && !self.participant_prohibits_block(
                attacker,
                blocker,
                BlockRestrictionSubjectDef::Attacker,
            )
    }

    #[cfg(test)]
    pub(in crate::game) fn blocking_is_prevented(
        &self,
        attacker: &Permanent,
        blocker: &Permanent,
    ) -> bool {
        !self.block_pair_is_allowed(blocker, attacker)
    }

    fn participant_prohibits_block(
        &self,
        subject: &Permanent,
        counterpart: &Permanent,
        expected_subject: BlockRestrictionSubjectDef,
    ) -> bool {
        let characteristics = self.targeting_event_object(counterpart);
        let mut prohibited = false;
        let _ = self.visit_applied_rules(subject, |applied| {
            if let AppliedRuleDef::BlockRestriction(BlockRestrictionDef::Pair {
                subject,
                counterpart,
                cost,
            }) = applied.rule
                && subject == expected_subject
                && self.block_restriction_matches(counterpart, &characteristics, applied.source)
                && cost.is_none()
            {
                prohibited = true;
                return ControlFlow::Break(());
            }
            ControlFlow::Continue(())
        });
        prohibited
    }

    fn block_restriction_matches(
        &self,
        counterpart_match: BlockRestrictionMatchDef,
        counterpart: &super::super::TriggerEventObject,
        source: GameObjectId,
    ) -> bool {
        match counterpart_match {
            BlockRestrictionMatchDef::Any => true,
            BlockRestrictionMatchDef::Matching(predicate) => {
                self.trigger_object_matches(predicate, counterpart, source, false)
            }
            BlockRestrictionMatchDef::Except(predicate) => {
                !self.trigger_object_matches(predicate, counterpart, source, false)
            }
        }
    }

    pub(super) fn prospective_block_is_affordable(
        &self,
        player: PlayerId,
        blocker: &Permanent,
        attacker: GameObjectId,
    ) -> bool {
        let Some(cost) = self.block_declaration_cost(player, Some((blocker.card.id, attacker)))
        else {
            return false;
        };
        cost == ManaCost::default() || self.can_pay_declaration_cost(player, cost, None)
    }

    pub(in crate::game) fn block_declaration_is_payable(&self, player: PlayerId) -> bool {
        let Some(cost) = self.block_declaration_cost(player, None) else {
            return false;
        };
        cost == ManaCost::default() || self.can_pay_declaration_cost(player, cost, None)
    }

    /// A requirement never forces its player to pay a declaration cost. Once
    /// another chosen block already incurs a blocker-scoped cost, adding a
    /// cost-free relationship for that same blocker can still be required.
    pub(super) fn prospective_block_adds_no_cost(
        &self,
        player: PlayerId,
        blocker: GameObjectId,
        attacker: GameObjectId,
    ) -> bool {
        self.block_declaration_cost(player, None)
            .zip(self.block_declaration_cost(player, Some((blocker, attacker))))
            .is_some_and(|(current, prospective)| current == prospective)
    }

    pub(super) fn pay_block_declaration_cost(&mut self, player: PlayerId) {
        let cost = self
            .block_declaration_cost(player, None)
            .expect("a legal block declaration remains legal while it is committed");
        if cost == ManaCost::default() {
            return;
        }
        self.activate_mana_for_cost(player, cost, 0);
        let _spent = self.pay_player_cost(player, cost, 0);
    }

    /// Returns `None` if any relationship is prohibited and otherwise the
    /// declaration's total mana cost. A blocker-scoped rule contributes once
    /// per blocking creature; an attacker-scoped rule contributes once per
    /// blocker assigned to that attacker.
    fn block_declaration_cost(
        &self,
        player: PlayerId,
        prospective: Option<(GameObjectId, GameObjectId)>,
    ) -> Option<ManaCost> {
        let mut total = ManaCost::default();
        for blocker in self
            .battlefield
            .iter()
            .filter(|permanent| permanent.controller == player)
        {
            let attackers = self.blocked_attackers_for(blocker, prospective);
            if attackers.is_empty() {
                continue;
            }

            let mut allowed = true;
            let _ = self.visit_applied_rules(blocker, |applied| {
                let AppliedRuleDef::BlockRestriction(BlockRestrictionDef::Pair {
                    subject,
                    counterpart,
                    cost,
                }) = applied.rule
                else {
                    return ControlFlow::Continue(());
                };
                if subject != BlockRestrictionSubjectDef::Blocker
                    || !attackers.iter().any(|attacker| {
                        self.block_restriction_matches(
                            counterpart,
                            &self.targeting_event_object(attacker),
                            applied.source,
                        )
                    })
                {
                    return ControlFlow::Continue(());
                }
                let Some(cost) = cost else {
                    allowed = false;
                    return ControlFlow::Break(());
                };
                total = add_declaration_cost(total, cost);
                ControlFlow::Continue(())
            });
            if !allowed {
                return None;
            }

            let blocker_characteristics = self.targeting_event_object(blocker);
            for attacker in attackers {
                let _ = self.visit_applied_rules(attacker, |applied| {
                    let AppliedRuleDef::BlockRestriction(BlockRestrictionDef::Pair {
                        subject,
                        counterpart,
                        cost,
                    }) = applied.rule
                    else {
                        return ControlFlow::Continue(());
                    };
                    if subject != BlockRestrictionSubjectDef::Attacker
                        || !self.block_restriction_matches(
                            counterpart,
                            &blocker_characteristics,
                            applied.source,
                        )
                    {
                        return ControlFlow::Continue(());
                    }
                    let Some(cost) = cost else {
                        allowed = false;
                        return ControlFlow::Break(());
                    };
                    total = add_declaration_cost(total, cost);
                    ControlFlow::Continue(())
                });
                if !allowed {
                    return None;
                }
            }
        }
        Some(total)
    }

    fn blocked_attackers_for(
        &self,
        blocker: &Permanent,
        prospective: Option<(GameObjectId, GameObjectId)>,
    ) -> Vec<&Permanent> {
        let mut ids = blocker.blocking.clone();
        if let Some((prospective_blocker, attacker)) = prospective
            && prospective_blocker == blocker.card.id
        {
            ids.extend(self.band_group(attacker));
        }
        ids.sort_unstable();
        ids.dedup();
        ids.into_iter()
            .filter_map(|id| {
                self.battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == id)
            })
            .collect()
    }
}
