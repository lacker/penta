//! Attacker eligibility, defender-scoped restrictions, and declaration costs.

use std::ops::ControlFlow;

use crate::card::{AttackDefenderScopeDef, AttackRestrictionDef};

use super::super::{
    Action, AppliedRuleDef, AttackDefender, CardType, DeclarativeAbilityDef, EffectDef, Game,
    KeywordAbility, ManaCost, Permanent, PlayerId,
};
use super::add_declaration_cost;

impl Game {
    pub(in crate::game) fn attacker_actions(&self, player: PlayerId) -> Vec<Action> {
        let defenders = self.attack_defenders(player);
        self.battlefield
            .iter()
            .filter(|permanent| {
                permanent.controller == player
                    && !permanent.tapped
                    && !permanent.attacking
                    && permanent.detained_until_turn_of.is_none()
                    && self.can_attack_base(permanent)
            })
            .flat_map(|permanent| {
                defenders.iter().copied().filter_map(move |defender| {
                    self.prospective_attack_is_affordable(permanent, defender)
                        .then_some(Action::DeclareAttacker {
                            attacker: permanent.card.id,
                            defender,
                        })
                })
            })
            .collect()
    }

    fn attack_defenders(&self, player: PlayerId) -> Vec<AttackDefender> {
        let mut defenders = vec![AttackDefender::Player(player.opponent())];
        defenders.extend(
            self.battlefield
                .iter()
                .filter(|permanent| {
                    permanent.controller == player.opponent()
                        && self
                            .permanent_types(permanent)
                            .is_some_and(|types| types.contains(CardType::Planeswalker))
                })
                .map(|permanent| AttackDefender::Planeswalker(permanent.card.id)),
        );
        defenders
    }

    /// The creature-facing restrictions independent of which defender is
    /// chosen. Pair predicates and declaration costs are checked separately.
    fn can_attack_base(&self, permanent: &Permanent) -> bool {
        if self.base_stats(permanent).is_none() {
            return false;
        }
        if self.permanent_has_executable_keyword(permanent, KeywordAbility::Defender)
            && !self.has_applied_rule(permanent, AppliedRuleDef::MayAttackDespiteDefender)
        {
            return false;
        }
        if !self.attack_restrictions_met(permanent) {
            return false;
        }
        self.permanent_has_executable_keyword(permanent, KeywordAbility::Haste)
            || self.has_applied_rule(permanent, AppliedRuleDef::MayAttackAsThoughHasty)
            || self.turns_started[permanent.controller.index()] > permanent.entered_controller_turn
    }

    /// Test and observation helper: whether at least one defender is legal,
    /// ignoring whether the controller currently has mana for an optional
    /// declaration cost.
    pub(in crate::game) fn can_attack(&self, permanent: &Permanent) -> bool {
        self.can_attack_base(permanent)
            && self
                .attack_defenders(permanent.controller)
                .into_iter()
                .any(|defender| self.attack_pair_cost(permanent, defender).is_some())
    }

    /// "Attacks each combat if able" never forces a player to pay a cost.
    /// A cost-free planeswalker attack can still make the requirement apply
    /// when attacking the defending player would be taxed.
    pub(in crate::game) fn must_attack_if_able(&self, permanent: &Permanent) -> bool {
        self.permanent_has_executable_keyword(permanent, KeywordAbility::AttacksEachCombatIfAble)
            && self.can_attack_base(permanent)
            && self
                .attack_defenders(permanent.controller)
                .into_iter()
                .any(|defender| {
                    self.attack_pair_cost(permanent, defender)
                        .is_some_and(|cost| cost == ManaCost::default())
                })
    }

    /// Whether every "can't attack unless ..." clause this creature prints
    /// is currently satisfied.
    fn attack_restrictions_met(&self, permanent: &Permanent) -> bool {
        let mut allowed = true;
        let _ = self.visit_effective_abilities(permanent, |effective| {
            if effective.ability.is_executable()
                && matches!(
                    effective.ability.definition,
                    DeclarativeAbilityDef::Static(_)
                )
                && let Some(effect) = effective.ability.declarative_effect()
                && match effect {
                    EffectDef::CannotAttackUnless(query) => !self.any_battlefield_object_matches(
                        query,
                        permanent.card.id,
                        permanent.controller,
                    ),
                    EffectDef::CannotAttackIf(query) => self.any_battlefield_object_matches(
                        query,
                        permanent.card.id,
                        permanent.controller,
                    ),
                    _ => false,
                }
            {
                allowed = false;
                return ControlFlow::Break(());
            }
            ControlFlow::Continue(())
        });
        allowed
    }

    fn prospective_attack_is_affordable(
        &self,
        permanent: &Permanent,
        defender: AttackDefender,
    ) -> bool {
        let Some(cost) =
            self.attack_declaration_cost(permanent.controller, Some((permanent, defender)))
        else {
            return false;
        };
        if cost == ManaCost::default() {
            return true;
        }
        let taps_to_attack =
            !self.permanent_has_executable_keyword(permanent, KeywordAbility::Vigilance);
        self.can_pay_declaration_cost(
            permanent.controller,
            cost,
            taps_to_attack.then_some(permanent.card.id),
        )
    }

    pub(in crate::game) fn attack_declaration_is_payable(&self, player: PlayerId) -> bool {
        let Some(cost) = self.attack_declaration_cost(player, None) else {
            return false;
        };
        cost == ManaCost::default() || self.can_pay_declaration_cost(player, cost, None)
    }

    pub(super) fn pay_attack_declaration_cost(&mut self, player: PlayerId) {
        let cost = self
            .attack_declaration_cost(player, None)
            .expect("a legal attack declaration remains legal while it is committed");
        if cost == ManaCost::default() {
            return;
        }
        self.activate_mana_for_cost(player, cost, 0);
        let _spent = self.pay_player_cost(player, cost, 0);
    }

    fn attack_declaration_cost(
        &self,
        player: PlayerId,
        prospective: Option<(&Permanent, AttackDefender)>,
    ) -> Option<ManaCost> {
        let mut total = ManaCost::default();
        for permanent in self
            .battlefield
            .iter()
            .filter(|permanent| permanent.controller == player && permanent.attacking)
        {
            let defender = permanent.attack_defender?;
            total = add_declaration_cost(total, self.attack_pair_cost(permanent, defender)?);
        }
        if let Some((permanent, defender)) = prospective {
            total = add_declaration_cost(total, self.attack_pair_cost(permanent, defender)?);
        }
        Some(total)
    }

    /// Returns `None` for a prohibited pairing and otherwise the total cost
    /// this one attacker contributes to the declaration.
    fn attack_pair_cost(&self, attacker: &Permanent, defender: AttackDefender) -> Option<ManaCost> {
        let characteristics = self.targeting_event_object(attacker);
        let mut total = ManaCost::default();
        let mut allowed = true;

        let _ = self.visit_applied_rules(attacker, |applied| {
            if let AppliedRuleDef::AttackRestriction(restriction) = applied.rule
                && restriction.defender == AttackDefenderScopeDef::Any
                && self.trigger_object_matches(
                    restriction.attacker,
                    &characteristics,
                    applied.source,
                    false,
                )
            {
                if let Some(cost) = restriction.cost {
                    total = add_declaration_cost(total, cost);
                } else {
                    allowed = false;
                    return ControlFlow::Break(());
                }
            }
            ControlFlow::Continue(())
        });
        if !allowed {
            return None;
        }

        let affected_player = match defender {
            AttackDefender::Player(player) => player,
            AttackDefender::Planeswalker(planeswalker) => self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == planeswalker)
                .map(|permanent| permanent.controller)?,
        };
        let _ = self.visit_attack_restrictions(affected_player, |applied| {
            let restriction = applied.restriction;
            if attack_scope_matches(restriction, defender)
                && self.trigger_object_matches(
                    restriction.attacker,
                    &characteristics,
                    applied.source,
                    false,
                )
            {
                if let Some(cost) = restriction.cost {
                    total = add_declaration_cost(total, cost);
                } else {
                    allowed = false;
                    return ControlFlow::Break(());
                }
            }
            ControlFlow::Continue(())
        });
        allowed.then_some(total)
    }
}

fn attack_scope_matches(restriction: AttackRestrictionDef, defender: AttackDefender) -> bool {
    match restriction.defender {
        AttackDefenderScopeDef::Any | AttackDefenderScopeDef::AffectedPlayerOrPlaneswalker => true,
        AttackDefenderScopeDef::AffectedPlayer => matches!(defender, AttackDefender::Player(_)),
    }
}
