use super::{
    Action, AppliedEffectDef, AttackDefender, CardBehavior, CardType, CombatDamageAssignment,
    CombatDamageStage, CommittedTriggerEvent, ControlFlow, CounterKind, Game, GameEvent,
    GameObjectId, KeywordAbility, Permanent, PlayerId, Target,
};

impl Game {
    pub(super) fn attacker_actions(&self, player: PlayerId, moat_active: bool) -> Vec<Action> {
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
        self.battlefield
            .iter()
            .filter(|permanent| {
                permanent.controller == player
                    && !permanent.tapped
                    && !permanent.attacking
                    && self.can_attack_with_moat(permanent, moat_active)
            })
            .flat_map(|permanent| {
                defenders
                    .iter()
                    .copied()
                    .map(|defender| Action::DeclareAttacker {
                        attacker: permanent.card.id,
                        defender,
                    })
            })
            .collect()
    }

    #[cfg(test)]
    pub(super) fn can_attack(&self, permanent: &Permanent) -> bool {
        let moat_active = self.count_behavior(CardBehavior::Moat) > 0;
        self.can_attack_with_moat(permanent, moat_active)
    }

    pub(super) fn can_attack_with_moat(&self, permanent: &Permanent, moat_active: bool) -> bool {
        if self.base_stats(permanent).is_none() {
            return false;
        }
        let flying = moat_active && self.has_flying(permanent);
        self.can_attack_creature(permanent, moat_active, flying)
    }

    pub(super) fn can_attack_creature(
        &self,
        permanent: &Permanent,
        moat_active: bool,
        flying: bool,
    ) -> bool {
        if self.permanent_has_executable_keyword(permanent, KeywordAbility::Defender) {
            return false;
        }
        if moat_active && !flying {
            return false;
        }
        self.permanent_has_executable_keyword(permanent, KeywordAbility::Haste)
            || self.turns_started[permanent.controller.index()] > permanent.entered_controller_turn
    }

    pub(super) fn declare_attacker(&mut self, attacker: GameObjectId, defender: AttackDefender) {
        let vigilance = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == attacker)
            .is_some_and(|permanent| {
                self.permanent_has_executable_keyword(permanent, KeywordAbility::Vigilance)
            });
        if let Some(permanent) = self
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == attacker)
        {
            permanent.attacking = true;
            permanent.attack_defender = Some(defender);
            permanent.attacked_this_turn = true;
            permanent.attacks_this_turn = permanent.attacks_this_turn.saturating_add(1);
        }
        if !vigilance {
            let _ = self.tap_permanent(attacker);
        }
    }

    pub(super) fn finish_declaring_attackers(&mut self) {
        self.attackers_declared = true;
        self.priority = self.active_player;
        self.consecutive_passes = 0;
        let attackers = self
            .battlefield
            .iter()
            .filter(|permanent| permanent.controller == self.active_player && permanent.attacking)
            .map(|permanent| permanent.card.id)
            .collect::<Vec<_>>();
        if attackers.is_empty() {
            return;
        }
        self.events.push(GameEvent::AttackDeclared {
            player: self.active_player,
            attackers: attackers.clone(),
        });
        // CR 508.2: the whole declaration happens at once, so every attacker
        // is already attacking by the time any of these triggers is captured.
        let events = attackers
            .iter()
            .filter_map(|attacker| {
                self.battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == *attacker)
                    .map(|permanent| CommittedTriggerEvent::Attacks {
                        object: self.trigger_event_object(permanent),
                    })
            })
            .collect::<Vec<_>>();
        for event in &events {
            self.capture_battlefield_triggers(event);
        }
    }

    /// Whether a static effect on `attacker` forbids `blocker` from blocking
    /// it, as Juggernaut forbids Walls.
    pub(super) fn blocking_is_prevented(&self, attacker: &Permanent, blocker: &Permanent) -> bool {
        let characteristics = self.trigger_event_object(blocker);
        let mut prevented = false;
        let result = self.visit_static_applied_effects(attacker, |applied| {
            if let AppliedEffectDef::CannotBeBlockedBy(predicate) = applied.effect
                && self.trigger_object_matches(predicate, &characteristics, applied.source, false)
            {
                prevented = true;
                return ControlFlow::Break(());
            }
            ControlFlow::Continue(())
        });
        debug_assert!(result.is_continue() || prevented);
        prevented
    }

    pub(super) fn blocker_actions(&self, player: PlayerId) -> Vec<Action> {
        let blockers: Vec<_> = self
            .battlefield
            .iter()
            .filter(|permanent| {
                permanent.controller == player
                    && !permanent.tapped
                    && permanent.blocking.is_none()
                    && self.power(permanent).is_some()
            })
            .map(|permanent| permanent.card.id)
            .collect();
        let attackers: Vec<_> = self
            .battlefield
            .iter()
            .filter(|permanent| permanent.attacking)
            .map(|permanent| {
                (
                    permanent.card.id,
                    self.has_flying(permanent),
                    (self.has_mountainwalk(permanent)
                        && self.controls_mountain(permanent.controller.opponent()))
                        || (self.has_forestwalk(permanent)
                            && self.controls_forest(permanent.controller.opponent())),
                    self.power(permanent).unwrap_or(0),
                )
            })
            .collect();
        blockers
            .into_iter()
            .flat_map(|blocker| {
                let blocker_permanent = self
                    .battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == blocker)
                    .expect("blocker is on the battlefield");
                let blocker_can_block_flying = self.has_flying(blocker_permanent)
                    || self
                        .permanent_has_executable_keyword(blocker_permanent, KeywordAbility::Reach);
                let ironclaw =
                    self.effective_behavior(blocker_permanent) == Some(CardBehavior::IronclawOrcs);
                attackers
                    .iter()
                    .filter_map(move |(attacker, flying, unblockable, power)| {
                        let attacker_permanent = self
                            .battlefield
                            .iter()
                            .find(|permanent| permanent.card.id == *attacker)
                            .expect("attacker is on the battlefield");
                        let intimidate = self.permanent_has_executable_keyword(
                            attacker_permanent,
                            KeywordAbility::Intimidate,
                        );
                        let shares_color = self
                            .effective_rules(attacker_permanent)
                            .zip(self.effective_rules(blocker_permanent))
                            .is_some_and(|(attacker, blocker)| {
                                attacker
                                    .colors()
                                    .into_iter()
                                    .zip(blocker.colors())
                                    .any(|(attacker, blocker)| attacker && blocker)
                            });
                        let can_block = !(*unblockable
                            || attacker_permanent.unblockable_this_turn
                            || self.blocking_is_prevented(attacker_permanent, blocker_permanent)
                            || *flying && !blocker_can_block_flying
                            || intimidate
                                && !self.is_artifact_permanent(blocker_permanent)
                                && !shares_color
                            || ironclaw && *power >= 2
                            || self.combat_is_protected(blocker_permanent, attacker_permanent));
                        can_block.then_some(Action::DeclareBlocker {
                            blocker,
                            attacker: *attacker,
                        })
                    })
            })
            .collect()
    }

    pub(super) fn declare_blocker(&mut self, blocker: GameObjectId, attacker: GameObjectId) {
        if let Some(permanent) = self
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == blocker)
        {
            permanent.blocking = Some(attacker);
        }
        if !self.combat_blocked_attackers.contains(&attacker) {
            self.combat_blocked_attackers.push(attacker);
        }
    }

    pub(super) fn finish_declaring_blockers(&mut self) {
        self.blockers_declared = true;
        self.priority = self.active_player;
        self.consecutive_passes = 0;
        let blocked = self
            .battlefield
            .iter()
            .filter_map(|permanent| permanent.blocking)
            .collect::<Vec<_>>();
        for permanent in &mut self.battlefield {
            permanent.blocked = blocked.contains(&permanent.card.id);
        }
        let assignments = self
            .battlefield
            .iter()
            .filter_map(|permanent| {
                permanent
                    .blocking
                    .map(|attacker| (permanent.card.id, attacker))
            })
            .collect::<Vec<_>>();
        if !assignments.is_empty() {
            self.events.push(GameEvent::BlockDeclared {
                player: self.active_player.opponent(),
                assignments,
            });
        }
    }

    pub(super) fn start_combat_damage(&mut self) {
        // Tests and a few internal procedures can construct combat directly,
        // so also capture live blocking relationships here. During an ordinary
        // game, `declare_blocker` recorded them before either player received
        // priority and they therefore survive a blocker leaving the field.
        let newly_blocked = self
            .battlefield
            .iter()
            .filter_map(|permanent| permanent.blocking)
            .collect::<Vec<_>>();
        for attacker in newly_blocked {
            if !self.combat_blocked_attackers.contains(&attacker) {
                self.combat_blocked_attackers.push(attacker);
            }
        }

        let strike_wave_combatants = self
            .battlefield
            .iter()
            .filter(|permanent| permanent.attacking || permanent.blocking.is_some())
            .filter(|permanent| {
                self.permanent_has_executable_keyword(permanent, KeywordAbility::FirstStrike)
                    || self
                        .permanent_has_executable_keyword(permanent, KeywordAbility::DoubleStrike)
            })
            .map(|permanent| permanent.card.id)
            .collect::<Vec<_>>();
        self.combat_damage_stage = if strike_wave_combatants.is_empty() {
            CombatDamageStage::Single
        } else {
            CombatDamageStage::FirstStrike {
                strike_wave_combatants,
            }
        };
        self.begin_combat_damage_assignment();
    }

    pub(super) fn begin_regular_combat_damage_after_first_strike(&mut self) {
        let CombatDamageStage::FirstStrike {
            strike_wave_combatants,
        } = &self.combat_damage_stage
        else {
            return;
        };
        self.combat_damage_stage = CombatDamageStage::RegularAfterFirstStrike {
            strike_wave_combatants: strike_wave_combatants.clone(),
        };
        self.begin_combat_damage_assignment();
    }

    pub(super) fn deals_damage_in_current_combat_step(&self, permanent: &Permanent) -> bool {
        match &self.combat_damage_stage {
            CombatDamageStage::NotStarted | CombatDamageStage::Single => true,
            CombatDamageStage::FirstStrike {
                strike_wave_combatants,
            } => strike_wave_combatants.contains(&permanent.card.id),
            CombatDamageStage::RegularAfterFirstStrike {
                strike_wave_combatants,
            } => {
                !strike_wave_combatants.contains(&permanent.card.id)
                    || self
                        .permanent_has_executable_keyword(permanent, KeywordAbility::DoubleStrike)
            }
        }
    }

    pub(super) fn begin_combat_damage_assignment(&mut self) {
        for permanent in &mut self.battlefield {
            permanent.combat_damage_assignment.clear();
        }
        self.pending_combat_attackers = self
            .battlefield
            .iter()
            .filter(|attacker| {
                attacker.attacking && self.deals_damage_in_current_combat_step(attacker)
            })
            // Ask exactly when there is a real choice. One blocker and no
            // trample leaves a single legal distribution and no question; one
            // blocker plus trample is a genuine decision about how much to
            // spill past it.
            .filter(|attacker| self.combat_assignment_actions(attacker.card.id).len() > 1)
            .map(|attacker| attacker.card.id)
            .collect();
        if self.pending_combat_attackers.is_empty() {
            self.deal_combat_damage();
        }
    }

    pub(super) fn combat_assignment_actions(&self, attacker_id: GameObjectId) -> Vec<Action> {
        let Some(attacker) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == attacker_id)
        else {
            return Vec::new();
        };
        let power = self.power(attacker).unwrap_or(0).max(0).cast_unsigned();
        let trample = self.has_trample(attacker);
        let mut recipients: Vec<_> = self
            .battlefield
            .iter()
            .filter(|permanent| permanent.blocking == Some(attacker_id))
            .map(|permanent| Target::Permanent(permanent.card.id))
            .collect();
        recipients.sort_unstable();
        let blocker_count = recipients.len();
        let defender_index = trample
            .then(|| self.combat_defender_target(attacker))
            .flatten()
            .map(|defender| {
                let index = recipients.len();
                recipients.push(defender);
                index
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
                // 510.1c: damage is assigned in an order, and a blocker only
                // gets any once every blocker ahead of it has lethal. Whatever
                // order the player picks, that leaves at most one blocker
                // holding a non-lethal share.
                if blockers()
                    .filter(|(id, amount)| {
                        *amount > 0 && *amount < self.lethal_damage_from(*id, attacker_id)
                    })
                    .count()
                    > 1
                {
                    return false;
                }
                // 510.1d: trample only spills once every blocker has lethal.
                let defender_damage = defender_index
                    .and_then(|index| amounts.get(index))
                    .copied()
                    .unwrap_or(0);
                if defender_damage == 0 {
                    return true;
                }
                blockers().all(|(id, amount)| amount >= self.lethal_damage_from(id, attacker_id))
            })
            .map(|amounts| Action::AssignCombatDamage {
                attacker: attacker_id,
                assignments: recipients
                    .iter()
                    .copied()
                    .zip(amounts)
                    .map(|(recipient, amount)| CombatDamageAssignment { recipient, amount })
                    .collect(),
            })
            .collect()
    }

    /// How an unassigned attacker spreads its damage: enough to kill each
    /// blocker in turn, then the remainder over the top when it can trample
    /// onto its defender, or onto the last blocker otherwise.
    pub(super) fn default_damage_split(
        &self,
        attacker_id: GameObjectId,
        blockers: &[GameObjectId],
    ) -> Vec<(Target, u16)> {
        let Some(attacker) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == attacker_id)
        else {
            return Vec::new();
        };
        let mut remaining = self.power(attacker).unwrap_or(0).max(0).cast_unsigned();
        let trample = self.has_trample(attacker);
        let mut split = Vec::with_capacity(blockers.len() + 1);
        for blocker in blockers {
            let amount = self
                .lethal_damage_from(*blocker, attacker_id)
                .min(remaining);
            remaining -= amount;
            split.push((Target::Permanent(*blocker), amount));
        }
        if remaining > 0 {
            if trample && let Some(defender) = self.combat_defender_target(attacker) {
                split.push((defender, remaining));
            } else if let Some(last) = split.last_mut() {
                last.1 += remaining;
            }
        }
        split
    }

    pub(super) fn lethal_damage(&self, permanent_id: GameObjectId) -> u16 {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == permanent_id)
            .map_or(0, |permanent| {
                self.toughness(permanent)
                    .unwrap_or(0)
                    .max(0)
                    .cast_unsigned()
                    .saturating_sub(permanent.damage)
            })
    }

    pub(super) fn lethal_damage_from(
        &self,
        permanent_id: GameObjectId,
        source: GameObjectId,
    ) -> u16 {
        let ordinary = self.lethal_damage(permanent_id);
        if ordinary > 0
            && self
                .source_controller_with_keyword(source, KeywordAbility::Deathtouch)
                .is_some()
        {
            1
        } else {
            ordinary
        }
    }

    pub(super) fn assign_combat_damage(
        &mut self,
        attacker: GameObjectId,
        assignments: Vec<CombatDamageAssignment>,
    ) {
        if let Some(permanent) = self
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == attacker)
        {
            permanent.combat_damage_assignment = assignments;
        }
        self.pending_combat_attackers.remove(0);
        if self.pending_combat_attackers.is_empty() {
            self.deal_combat_damage();
        }
    }

    /// Whether combat damage to this recipient is prevented for the turn.
    /// Only a permanent can carry the prevention; a player never does.
    pub(super) fn combat_damage_is_prevented_for(&self, recipient: Target) -> bool {
        matches!(recipient, Target::Permanent(id)
            if self
                .battlefield
                .iter()
                .any(|permanent| permanent.card.id == id && permanent.combat_damage_prevented))
    }

    /// How much life a drain can take from a recipient: what it had before
    /// the damage, which is all it can give however much is dealt.
    pub(super) fn drainable_from(&self, target: Target) -> u16 {
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

    /// Raises the event for damage a player took, whatever dealt it. Only a
    /// battlefield source can be recognised, which is what every trigger that
    /// reads this needs.
    pub(super) fn publish_damage_to_player(
        &mut self,
        source: Option<GameObjectId>,
        player: PlayerId,
        amount: u16,
    ) {
        if amount == 0 {
            return;
        }
        let Some(source) = source.and_then(|source| {
            self.battlefield
                .iter()
                .find(|permanent| permanent.card.id == source)
        }) else {
            return;
        };
        let event = CommittedTriggerEvent::DamageDealtToPlayer {
            object: self.trigger_event_object(source),
            player,
            amount,
        };
        self.capture_battlefield_triggers(&event);
    }

    /// Combat damage from an attacker to a player, which is the one kind of
    /// damage the "whenever this deals combat damage to a player" triggers
    /// listen for. Ordinary damage to a player carries no such event.
    /// Combat damage from an attacker to whatever it is attacking. A player
    /// also gets the "deals combat damage to a player" event; a planeswalker
    /// takes the damage as a permanent, which its loyalty counters absorb.
    pub(super) fn deal_combat_damage_to(
        &mut self,
        attacker: GameObjectId,
        defender: Target,
        amount: u16,
    ) {
        match defender {
            Target::Player(player) => self.deal_combat_damage_to_player(attacker, player, amount),
            Target::Permanent(_) | Target::Card(_) | Target::Spell(_) => {
                // Flagged as combat damage so a trigger that listens for it
                // arriving here, as Vraska's does, can tell it apart from an
                // ability's damage.
                self.damage_target_from_kind(Some(attacker), Some(defender), amount, true);
            }
        }
    }

    pub(super) fn deal_combat_damage_to_player(
        &mut self,
        attacker: GameObjectId,
        player: PlayerId,
        amount: u16,
    ) {
        self.damage_target_from(Some(attacker), Some(Target::Player(player)), amount);
        if amount == 0 {
            return;
        }
        let Some(source) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == attacker)
        else {
            return;
        };
        let event = CommittedTriggerEvent::CombatDamageDealtToPlayer {
            object: self.trigger_event_object(source),
            player,
            amount,
        };
        self.capture_battlefield_triggers(&event);
    }

    /// Combat damage between one blocked attacker and everything blocking it,
    /// in both directions.
    pub(super) fn exchange_blocked_combat_damage(
        &mut self,
        attacker_id: GameObjectId,
        attacker_index: usize,
        blockers: &[GameObjectId],
        attacker_deals_damage: bool,
    ) {
        let assignments = self.battlefield[attacker_index]
            .combat_damage_assignment
            .clone();
        if attacker_deals_damage {
            let split = if assignments.is_empty() {
                self.default_damage_split(attacker_id, blockers)
            } else {
                assignments
                    .into_iter()
                    .map(|assignment| (assignment.recipient, assignment.amount))
                    .collect()
            };
            for (recipient, amount) in split {
                if self.combat_damage_is_prevented_for(recipient) {
                    continue;
                }
                // Trample past a blocker is still combat damage to a player,
                // so it goes through the same path as an unblocked hit.
                if let Target::Player(player) = recipient {
                    self.deal_combat_damage_to_player(attacker_id, player, amount);
                } else {
                    self.damage_target_from(Some(attacker_id), Some(recipient), amount);
                }
            }
        }
        if self.combat_damage_is_prevented_for(Target::Permanent(attacker_id)) {
            return;
        }
        let return_damage = blockers
            .iter()
            .filter_map(|id| {
                self.battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == *id)
                    .filter(|permanent| {
                        self.deals_damage_in_current_combat_step(permanent)
                            && !permanent.combat_damage_prevented
                    })
                    .and_then(|permanent| self.power(permanent))
                    .map(|power| (*id, power.max(0).cast_unsigned()))
            })
            .collect::<Vec<_>>();
        for (blocker, amount) in return_damage {
            self.damage_target_from(Some(blocker), Some(Target::Permanent(attacker_id)), amount);
        }
    }

    pub(super) fn combat_defender(attacker: &Permanent) -> AttackDefender {
        attacker
            .attack_defender
            .unwrap_or(AttackDefender::Player(attacker.controller.opponent()))
    }

    pub(super) fn combat_defender_target(&self, attacker: &Permanent) -> Option<Target> {
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

    pub(super) fn deal_combat_damage(&mut self) {
        let attackers: Vec<_> = self
            .battlefield
            .iter()
            .filter(|permanent| permanent.attacking)
            .map(|permanent| permanent.card.id)
            .collect();
        for attacker_id in attackers {
            let Some(attacker_index) = self
                .battlefield
                .iter()
                .position(|permanent| permanent.card.id == attacker_id)
            else {
                continue;
            };
            let power = self
                .power(&self.battlefield[attacker_index])
                .unwrap_or(0)
                .max(0)
                .cast_unsigned();
            let attacker_deals_damage = self
                .deals_damage_in_current_combat_step(&self.battlefield[attacker_index])
                && !self.battlefield[attacker_index].combat_damage_prevented;
            let blockers: Vec<_> = self
                .battlefield
                .iter()
                .filter(|permanent| permanent.blocking == Some(attacker_id))
                .map(|permanent| permanent.card.id)
                .collect();
            if attacker_deals_damage && blockers.is_empty() {
                let was_blocked = self.combat_blocked_attackers.contains(&attacker_id);
                if was_blocked && !self.has_trample(&self.battlefield[attacker_index]) {
                    continue;
                }
                let Some(defender) = self.combat_defender_target(&self.battlefield[attacker_index])
                else {
                    continue;
                };
                self.deal_combat_damage_to(attacker_id, defender, power);
            } else if !blockers.is_empty() {
                self.exchange_blocked_combat_damage(
                    attacker_id,
                    attacker_index,
                    &blockers,
                    attacker_deals_damage,
                );
            }
        }
        self.check_state_based_actions();
    }
}

pub(super) fn damage_distributions(recipient_count: usize, total: u16) -> Vec<Vec<u16>> {
    if recipient_count == 0 {
        return (total == 0).then_some(Vec::new()).into_iter().collect();
    }
    let mut result = Vec::new();
    for amount in 0..=total {
        for mut tail in damage_distributions(recipient_count - 1, total - amount) {
            let mut distribution = vec![amount];
            distribution.append(&mut tail);
            result.push(distribution);
        }
    }
    result
}
