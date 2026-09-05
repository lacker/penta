use super::{
    Action, CombatDamageStage, CommittedTriggerEvent, CounterKind, DeferredBeginTurnEffect,
    EffectDef, EffectResolutionContext, Game, GameEvent, GameObjectId, GameResult,
    InstalledTriggerLifetime, ManaPool, PendingProcedure, PlayerId, Step, TurnPhaseDef,
    TurnPhaseResume, TurnStepDef, one_or_none,
};

mod begin_turn;
mod drawing;
mod life_gain;

impl Game {
    pub(in crate::game) fn skips_turn_based_untap(&self, permanent: &super::Permanent) -> bool {
        permanent.skipped_untap_steps > 0 || self.does_not_untap_during_untap_step(permanent)
    }

    /// Spends one owed untap step for each of the active player's permanents.
    /// Called once the untap step is over, so the count is still readable
    /// while the step decides what untaps.
    fn spend_untap_skips(&mut self) {
        let active = self.active_player;
        for permanent in &mut self.battlefield {
            if permanent.controller == active {
                permanent.skipped_untap_steps = permanent.skipped_untap_steps.saturating_sub(1);
            }
        }
    }

    pub(super) fn untap_actions(&self, player: PlayerId) -> Vec<Action> {
        let untappable: Vec<GameObjectId> = self
            .battlefield
            .iter()
            .filter(|permanent| {
                permanent.controller == player
                    && permanent.tapped
                    && !self.skips_turn_based_untap(permanent)
            })
            .map(|permanent| permanent.card.id)
            .collect();
        // Each cap narrows its own group to one survivor, so a permanent
        // covered by two caps cannot let a second one through either.
        let groups: Vec<Vec<GameObjectId>> = self
            .untap_limits(player)
            .into_iter()
            .map(|limit| self.permanents_under_untap_limit(player, limit))
            .collect();
        let mut choices: Vec<Vec<GameObjectId>> = vec![untappable];
        for group in &groups {
            choices = choices
                .iter()
                .flat_map(|chosen| {
                    one_or_none(group).into_iter().map(move |kept| {
                        let mut next = chosen.clone();
                        next.retain(|id| !group.contains(id) || kept.contains(id));
                        next
                    })
                })
                .collect();
        }
        // A permanent that may choose not to untap turns its own untap into
        // an independent yes/no, so each combination of those choices is a
        // separate declaration.
        let optional = self
            .battlefield
            .iter()
            .filter(|permanent| {
                permanent.controller == player
                    && permanent.tapped
                    && self.may_choose_not_to_untap(permanent)
            })
            .map(|permanent| permanent.card.id)
            .collect::<Vec<_>>();
        let mut actions = Vec::new();
        for permanents in &choices {
            for skipped in Self::subsets_of(&optional) {
                let mut choice = permanents.clone();
                choice.retain(|id| !skipped.contains(id));
                if !actions.contains(&Action::ChooseUntap {
                    permanents: choice.clone(),
                }) {
                    actions.push(Action::ChooseUntap { permanents: choice });
                }
            }
        }
        actions
    }

    /// Every subset of a small set, for the independent untap choices above.
    /// The printed cards put one such permanent on the battlefield at a time;
    /// the bound keeps a pathological board from exploding the action list,
    /// and beyond it every optional permanent simply untaps.
    fn subsets_of(ids: &[GameObjectId]) -> Vec<Vec<GameObjectId>> {
        const MAXIMUM_INDEPENDENT_CHOICES: usize = 4;
        if ids.len() > MAXIMUM_INDEPENDENT_CHOICES {
            return vec![Vec::new()];
        }
        (0..1usize << ids.len())
            .map(|mask| {
                ids.iter()
                    .enumerate()
                    .filter(|(index, _)| mask & (1 << index) != 0)
                    .map(|(_, id)| *id)
                    .collect()
            })
            .collect()
    }

    pub(super) fn choose_untap(&mut self, player: PlayerId, selected: &[GameObjectId]) {
        let eligible = self
            .battlefield
            .iter()
            .filter(|permanent| {
                permanent.controller == player && !self.skips_turn_based_untap(permanent)
            })
            .map(|permanent| permanent.card.id)
            .collect::<Vec<_>>();
        for permanent in &mut self.battlefield {
            if eligible.contains(&permanent.card.id) && selected.contains(&permanent.card.id) {
                permanent.untap();
            }
        }
        self.untap_pending = false;
        self.priority = self.active_player;
        self.spend_untap_skips();
        self.handle_upkeep_triggers();
    }

    /// Adds one kind of counter to a player. Poison's state-based loss and
    /// energy's payment semantics read the same counter store independently.
    pub(super) fn add_player_counters(&mut self, player: PlayerId, kind: CounterKind, amount: u16) {
        if amount == 0 {
            return;
        }
        self.players[player.index()].counters.add(kind, amount);
    }

    /// Spends energy, all of it or none. A payment that cannot be made in
    /// full is not made at all, which is what makes "unless you pay {E}" a
    /// real choice rather than a partial one.
    pub(super) fn spend_energy(&mut self, player: PlayerId, amount: u16) -> bool {
        if self.players[player.index()]
            .counters
            .count(CounterKind::named("energy"))
            < amount
        {
            return false;
        }
        self.players[player.index()]
            .counters
            .remove(CounterKind::named("energy"), amount);
        true
    }

    /// Life loss that is not damage: no source deals it, nothing that
    /// triggers on damage sees it, and prevention does not apply.
    pub(super) fn lose_life(&mut self, player: PlayerId, amount: u16) {
        if amount == 0 || self.life_total_cannot_change(player) {
            return;
        }
        let amount_as_i16 = i16::try_from(amount).unwrap_or(i16::MAX);
        self.players[player.index()].life -= amount_as_i16;
        if amount > 0 {
            self.lost_life_this_turn[player.index()] = true;
        }
        self.events.push(GameEvent::LifeLost { player, amount });
    }

    pub(super) fn deal_damage(&mut self, player: PlayerId, amount: u16) {
        if !self.life_total_cannot_change(player) {
            let amount_as_i16 = i16::try_from(amount).unwrap_or(i16::MAX);
            self.players[player.index()].life -= amount_as_i16;
        }
        // Damage to a player is life lost, which is what the clauses reading
        // it ask about: how the life went is not part of the question.
        if amount > 0 && !self.life_total_cannot_change(player) {
            self.lost_life_this_turn[player.index()] = true;
        }
        self.events.push(GameEvent::DamageDealt { player, amount });
    }

    /// Every step a card can name, which is now all of them: Thawing
    /// Glaciers returns itself at the beginning of the cleanup step.
    pub(super) const fn turn_step_def(step: Step) -> TurnStepDef {
        match step {
            Step::Upkeep => TurnStepDef::Upkeep,
            Step::Draw => TurnStepDef::Draw,
            Step::PrecombatMain => TurnStepDef::PrecombatMain,
            Step::BeginningOfCombat => TurnStepDef::BeginningOfCombat,
            Step::DeclareAttackers => TurnStepDef::DeclareAttackers,
            Step::DeclareBlockers => TurnStepDef::DeclareBlockers,
            Step::CombatDamage => TurnStepDef::CombatDamage,
            Step::EndOfCombat => TurnStepDef::EndOfCombat,
            Step::PostcombatMain => TurnStepDef::PostcombatMain,
            Step::End => TurnStepDef::End,
            Step::Cleanup => TurnStepDef::Cleanup,
        }
    }

    pub(super) fn advance_step(&mut self) {
        if self.step.ends_phase() || self.format.rules().mana_empties_at_end_of_step {
            self.empty_mana_pools();
            if self.result.is_some() {
                return;
            }
        }

        match self.step {
            Step::Upkeep => {
                self.step = Step::Draw;
                self.draw_step_draw_taken[self.active_player.index()] = false;
                if !(self.turn == 1 && self.active_player == PlayerId::One) {
                    self.draw_instruction(self.active_player, 1);
                    if !self.pending_decisions.is_empty() || !self.pending_events.is_empty() {
                        self.pending_procedures
                            .push_back(PendingProcedure::FinishStepAdvance);
                        return;
                    }
                }
            }
            Step::Draw => {
                if self.advance_after_turn_phase(TurnPhaseResume::Step(Step::PrecombatMain)) {
                    return;
                }
            }
            Step::PrecombatMain => {
                if self.advance_after_turn_phase(TurnPhaseResume::Step(Step::BeginningOfCombat)) {
                    return;
                }
            }
            Step::BeginningOfCombat => {
                self.step = Step::DeclareAttackers;
                self.attackers_declared = false;
            }
            Step::DeclareAttackers => {
                self.step = Step::DeclareBlockers;
                self.blockers_declared = false;
            }
            Step::DeclareBlockers => {
                self.step = Step::CombatDamage;
                self.start_combat_damage();
            }
            Step::CombatDamage => self.advance_combat_damage_step(),
            Step::EndOfCombat => {
                self.expire_end_of_combat_effects();
                self.clear_combat();
                if self.advance_after_turn_phase(TurnPhaseResume::Step(Step::PostcombatMain)) {
                    return;
                }
            }
            Step::PostcombatMain => {
                if self.advance_after_turn_phase(TurnPhaseResume::Step(Step::End)) {
                    return;
                }
            }
            Step::End => {
                self.step = Step::Cleanup;
                self.cleanup();
                if !self.cleanup_pending {
                    return;
                }
            }
            Step::Cleanup => {
                if self.advance_after_turn_phase(TurnPhaseResume::NextTurn) {
                    return;
                }
            }
        }

        self.finish_step_advance();
    }

    /// Starts the next additional phase, or resumes the ordinary turn once
    /// the queue is empty. The first boundary displaced by a schedule is
    /// frozen so an inserted combat created after the postcombat main resumes
    /// at the end step rather than manufacturing another ordinary main phase.
    ///
    /// Returns `true` when progression began the next turn, whose startup path
    /// publishes its own step change.
    fn advance_after_turn_phase(&mut self, ordinary_resume: TurnPhaseResume) -> bool {
        let next = if let Some(phase) = self.turn_phase_queue.pop_front() {
            self.turn_phase_resume.get_or_insert(ordinary_resume);
            TurnPhaseResume::Step(match phase {
                TurnPhaseDef::Combat => Step::BeginningOfCombat,
                TurnPhaseDef::PostcombatMain => Step::PostcombatMain,
            })
        } else {
            self.turn_phase_resume.take().unwrap_or(ordinary_resume)
        };

        match next {
            TurnPhaseResume::Step(step) => {
                self.step = step;
                if step == Step::End {
                    self.handle_end_step();
                }
                false
            }
            TurnPhaseResume::NextTurn => {
                self.start_next_turn();
                true
            }
        }
    }

    pub(super) fn schedule_turn_phases(&mut self, phases: &[TurnPhaseDef]) {
        for phase in phases.iter().rev() {
            self.turn_phase_queue.push_front(*phase);
        }
    }

    fn finish_step_advance(&mut self) {
        if self.result.is_none() {
            // CR 714.2b: the lore counter goes on after the draw step, which
            // is as the precombat main phase begins -- before anything in
            // that phase, including the chapter it reads.
            if self.step == Step::PrecombatMain {
                let active = self.active_player;
                self.place_draw_step_lore_counters(active);
            }
            self.begin_step_triggers();
            self.priority = self.active_player;
            self.events.push(GameEvent::StepChanged {
                turn: self.turn,
                active_player: self.active_player,
                step: self.step,
            });
        }
    }

    pub(super) fn advance_combat_damage_step(&mut self) {
        if matches!(
            &self.combat_damage_stage,
            CombatDamageStage::FirstStrike { .. }
        ) {
            self.begin_regular_combat_damage_after_first_strike();
        } else {
            self.step = Step::EndOfCombat;
        }
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn commit_next_turn(
        &mut self,
        next_player: PlayerId,
        deferred: Vec<DeferredBeginTurnEffect>,
    ) {
        // CR 614.10b: an action coupled to a skipped turn is the first thing
        // that happens in the next turn that actually occurs. Do not expose it
        // while another prospective turn can still be replaced.
        self.perform_deferred_begin_turn_effects(next_player, deferred);
        self.turn += 1;
        self.active_player = next_player;
        self.turns_started[self.active_player.index()] += 1;
        let turns_started = self.turns_started;
        self.damage_preventions.retain(|prevention| {
            prevention
                .expiration
                .survives_turn_start(self.active_player, turns_started)
        });
        self.damage_redirects.retain(|redirect| {
            redirect
                .expiration
                .survives_turn_start(self.active_player, turns_started)
        });
        self.resolved_play_restrictions.retain(|restriction| {
            restriction
                .expiration
                .survives_turn_start(self.active_player, turns_started)
        });
        self.resolved_attack_restrictions.retain(|restriction| {
            restriction
                .expiration
                .survives_turn_start(self.active_player, turns_started)
        });
        self.resolved_play_permissions.retain(|permission| {
            permission
                .expiration
                .survives_turn_start(self.active_player, turns_started)
        });
        self.resolved_player_protections.retain(|protection| {
            protection
                .expiration
                .survives_turn_start(self.active_player, turns_started)
        });
        self.resolved_player_rules.retain(|rule| {
            rule.expiration
                .survives_turn_start(self.active_player, turns_started)
        });
        self.ongoing_effects.retain(|effect| {
            effect
                .expiration
                .survives_turn_start(self.active_player, turns_started)
        });
        for permanent in &mut self.battlefield {
            permanent.resolved_continuous_effects.retain(|effect| {
                effect
                    .expiration
                    .survives_turn_start(self.active_player, turns_started)
            });
            // A copy that named a duration ends with everything else that
            // did; one that named none has no expiration to check.
            if permanent.copy_expiration.is_some_and(|expiration| {
                !expiration.survives_turn_start(self.active_player, turns_started)
            }) {
                permanent.copy_effect = None;
                permanent.copy_expiration = None;
            }
        }
        // The bound a limited graveyard permission carries is per turn, so
        // the allowance returns as the turn does.
        self.graveyard_permission_uses.clear();
        self.creature_died_this_turn = false;
        self.damage_cannot_be_prevented_this_turn = false;
        self.creatures_died_this_turn = 0;
        self.turn_phase_queue.clear();
        self.turn_phase_resume = None;
        self.spells_cast_last_turn = self.spells_cast_this_turn;
        self.spells_cast_this_turn = [0; 2];
        self.spell_cast_history_this_turn.clear();
        self.cards_drawn_this_turn = [0; 2];
        self.life_gained_this_turn = [0; 2];
        self.lost_life_this_turn = [false; 2];
        self.permanent_left_battlefield_this_turn = [false; 2];
        self.card_left_graveyard_this_turn = [false; 2];
        self.drawn_this_turn = [Vec::new(), Vec::new()];
        self.step = Step::Upkeep;
        self.players[self.active_player.index()].lands_played_this_turn = 0;
        let started = self.turns_started[self.active_player.index()];
        let active = self.active_player;
        // The lifetime freezes both the referenced player and the exact turn
        // at installation, so extra turns and skipped turns have ordinary
        // turn-engine semantics rather than being re-evaluated later.
        self.installed_triggers
            .retain(|installed| match installed.lifetime {
                InstalledTriggerLifetime::Once => true,
                // The turn it was installed on is over: this runs as the
                // next one begins, whoever it belongs to.
                InstalledTriggerLifetime::ThisTurn { turn } => self.turn == turn,
                InstalledTriggerLifetime::UntilTurn { player, turn } => {
                    player != self.active_player || self.turns_started[player.index()] < turn
                }
            });
        for permanent in &mut self.battlefield {
            permanent
                .keywords_until_upkeep_of
                .retain(|(player, _)| *player != self.active_player);
            // Detain ends when its controller's next turn begins.
            if permanent
                .detained_until_turn_of
                .is_some_and(|(player, created)| player == active && started > created)
            {
                permanent.detained_until_turn_of = None;
            }
            // One loyalty ability per planeswalker per turn, so the allowance
            // returns as the turn does.
            permanent.activated_loyalty_this_turn = false;
            permanent.activations_this_turn.clear();
            permanent.triggers_this_turn.clear();
            permanent.resolutions_this_turn.clear();
            permanent.dealt_damage_to_opponent_this_turn = false;
            permanent.attacked_this_turn = false;
            permanent.exerted = false;
            permanent.saddled = false;
            permanent.attacks_this_turn = 0;
            permanent.damage_sources.clear();
            permanent.was_dealt_damage_this_turn = false;
            permanent.dealt_damage_this_turn = false;
        }
        // "Damage dealt to you this turn" resets with the turn, not with
        // cleanup: a spell cast in the postcombat main phase still reads what
        // combat did.
        self.damage_taken_this_turn = [0; 2];
        self.damage_taken_by_group_this_turn = [[0; crate::card::DamageSourceGroupDef::COUNT]; 2];
        // "During any turn you attacked with a Rogue" is about this turn,
        // so what was attacked with goes with it.
        self.attacked_subtypes_this_turn = [Vec::new(), Vec::new()];
        // "It phases in before its controller untaps": ahead of the untap
        // itself, so a permanent that comes back this turn untaps with the
        // rest of them.
        self.phase_in_for(self.active_player);
        // Everything a cap covers waits for the player to choose; everything
        // else untaps now.
        let capped: Vec<GameObjectId> = self
            .untap_limits(self.active_player)
            .into_iter()
            .flat_map(|limit| self.permanents_under_untap_limit(self.active_player, limit))
            .collect();
        let untap_restricted: Vec<_> = self
            .battlefield
            .iter()
            .filter(|permanent| self.skips_turn_based_untap(permanent))
            .map(|permanent| permanent.card.id)
            .collect();
        self.untap_pending = false;
        for permanent in &mut self.battlefield {
            if permanent.controller == self.active_player {
                if capped.contains(&permanent.card.id)
                    && permanent.tapped
                    && !untap_restricted.contains(&permanent.card.id)
                {
                    self.untap_pending = true;
                } else if !untap_restricted.contains(&permanent.card.id) {
                    permanent.untap();
                }
            }
        }
        // The untap is done, so anything that only had to survive it is
        // finished. Dropping these here rather than with the turn-start pass
        // above is what makes "until your next upkeep" cover the untap step
        // the way the words do.
        let active = self.active_player;
        self.damage_preventions
            .retain(|prevention| prevention.expiration.survives_untap_step(active));
        self.damage_redirects
            .retain(|redirect| redirect.expiration.survives_untap_step(active));
        self.resolved_play_restrictions
            .retain(|restriction| restriction.expiration.survives_untap_step(active));
        self.resolved_attack_restrictions
            .retain(|restriction| restriction.expiration.survives_untap_step(active));
        self.resolved_play_permissions
            .retain(|permission| permission.expiration.survives_untap_step(active));
        self.resolved_player_protections
            .retain(|protection| protection.expiration.survives_untap_step(active));
        self.resolved_player_rules
            .retain(|rule| rule.expiration.survives_untap_step(active));
        self.ongoing_effects
            .retain(|effect| effect.expiration.survives_untap_step(active));
        for permanent in &mut self.battlefield {
            permanent
                .resolved_continuous_effects
                .retain(|effect| effect.expiration.survives_untap_step(active));
        }
        if !self.untap_pending {
            self.spend_untap_skips();
            self.handle_upkeep_triggers();
        }
        if self.result.is_none() {
            self.priority = self.active_player;
            self.events.push(GameEvent::StepChanged {
                turn: self.turn,
                active_player: self.active_player,
                step: self.step,
            });
        }
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn handle_upkeep_triggers(&mut self) {
        let player = self.active_player;
        self.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
            step: TurnStepDef::Upkeep,
            player,
        });
    }

    /// Ends the continuous effects that last only for one combat. Combat can
    /// happen more than once in a turn, so this runs per combat phase rather
    /// than waiting for cleanup.
    fn expire_end_of_combat_effects(&mut self) {
        for permanent in &mut self.battlefield {
            permanent
                .resolved_continuous_effects
                .retain(|effect| effect.expiration.survives_end_of_combat());
        }
        self.damage_preventions
            .retain(|prevention| prevention.expiration.survives_end_of_combat());
        self.damage_redirects
            .retain(|redirect| redirect.expiration.survives_end_of_combat());
        self.resolved_play_restrictions
            .retain(|restriction| restriction.expiration.survives_end_of_combat());
        self.resolved_attack_restrictions
            .retain(|restriction| restriction.expiration.survives_end_of_combat());
        self.resolved_play_permissions
            .retain(|permission| permission.expiration.survives_end_of_combat());
        self.resolved_player_protections
            .retain(|protection| protection.expiration.survives_end_of_combat());
        self.resolved_player_rules
            .retain(|rule| rule.expiration.survives_end_of_combat());
        self.ongoing_effects
            .retain(|effect| effect.expiration.survives_end_of_combat());
    }

    pub(super) fn handle_end_step(&mut self) {
        self.monarch_draws_at_end_step();
        let doomed: Vec<_> = self
            .battlefield
            .iter()
            .filter(|permanent| permanent.destroy_at_end)
            .map(|permanent| permanent.card.id)
            .collect();
        for id in doomed {
            self.destroy_permanent(id);
        }
    }

    pub(super) fn cleanup(&mut self) {
        if self
            .maximum_hand_size(self.active_player)
            .is_some_and(|maximum| self.players[self.active_player.index()].hand.len() > maximum)
        {
            self.cleanup_pending = true;
        } else {
            self.complete_cleanup();
        }
    }

    pub(super) fn complete_cleanup(&mut self) {
        self.finish_cleanup();
        self.empty_mana_pools();
        if self.result.is_some() {
            return;
        }
        // CR 514.2-3: the turn-based actions happen first, and only then does
        // anything that triggered at the beginning of this step go on the
        // stack. When something does, the turn stops here and both players
        // get priority instead of the step ending silently.
        //
        // The extra cleanup step CR 514.3 grants afterwards is not modeled:
        // no supported card creates an effect during cleanup that a second
        // round would have to clear.
        self.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
            step: TurnStepDef::Cleanup,
            player: self.active_player,
        });
        if !self.pending_triggers.is_empty() {
            self.priority = self.active_player;
            self.events.push(GameEvent::StepChanged {
                turn: self.turn,
                active_player: self.active_player,
                step: self.step,
            });
            return;
        }
        if !self.advance_after_turn_phase(TurnPhaseResume::NextTurn) {
            self.finish_step_advance();
        }
    }

    pub(super) fn finish_cleanup(&mut self) {
        self.nonbattlefield_ability_grants
            .retain(|grant| grant.expiration.survives_cleanup());
        // These resolving permissions and restrictions last only until the
        // cleanup step. A later phase can still be inserted into this turn,
        // but it must not revive an expired Quicken or Aurelia's Fury effect.
        self.resolved_play_restrictions
            .retain(|restriction| restriction.expiration.survives_cleanup());
        self.resolved_attack_restrictions
            .retain(|restriction| restriction.expiration.survives_cleanup());
        self.resolved_play_permissions
            .retain(|permission| permission.expiration.survives_cleanup());
        self.resolved_player_protections
            .retain(|protection| protection.expiration.survives_cleanup());
        self.resolved_player_rules
            .retain(|rule| rule.expiration.survives_cleanup());
        self.ongoing_effects
            .retain(|effect| effect.expiration.survives_cleanup());
        self.damage_preventions
            .retain(|prevention| prevention.expiration.survives_cleanup());
        self.damage_redirects
            .retain(|redirect| redirect.expiration.survives_cleanup());
        for replacements in &mut self.draw_replacements {
            replacements.clear();
        }
        // A source-tapped effect survives cleanup only while its recorded
        // source is still present and tapped. Once spent, remove the ordered
        // component so tapping that source again cannot revive it.
        let still_tapped: Vec<_> = self
            .battlefield
            .iter()
            .filter(|permanent| permanent.tapped)
            .map(|permanent| permanent.card.id)
            .collect();
        // The same for a source that had only to remain: it is already inert
        // once its source has left, and dropping it here keeps a permanent
        // from carrying components nothing will ever read again.
        let still_present: Vec<_> = self
            .battlefield
            .iter()
            .map(|permanent| permanent.card.id)
            .collect();
        for permanent in &mut self.battlefield {
            permanent.damage = 0;
            permanent.exile_instead_of_dying = false;
            permanent.deathtouch_damage = false;
            permanent.temporary_keywords.clear();
            permanent
                .text_changes
                .retain(|change| change.expiration.survives_cleanup());
            permanent.resolved_continuous_effects.retain(|effect| {
                effect.expiration.survives_cleanup()
                    && (!effect.expiration.requires_source_tapped()
                        || still_tapped.contains(&effect.source.object))
                    && (!effect.expiration.requires_source_to_remain()
                        || still_present.contains(&effect.source.object))
            });
            if permanent
                .copy_expiration
                .is_some_and(|expiration| !expiration.survives_cleanup())
            {
                permanent.copy_effect = None;
                permanent.copy_expiration = None;
            }
            // A control change held by a permanent outlives the turn; only
            // the turn-scoped form is ended here.
            if permanent.control_source.is_none()
                && let Some(owner) = permanent.control_reverts_to.take()
            {
                permanent.controller = owner;
                permanent.suspend_haste = false;
            }
            permanent.destroy_at_end = false;
            permanent.regeneration_shields = 0;
        }
    }

    pub(super) fn clear_combat(&mut self) {
        for permanent in &mut self.battlefield {
            permanent.attacking = false;
            permanent.attacking_band = None;
            permanent.blocked = false;
            permanent.blocking.clear();
            permanent.blocking_this_combat = false;
            permanent.combat_damage_assignment.clear();
        }
        self.pending_combat_assignments.clear();
        self.combat_damage_stage = CombatDamageStage::NotStarted;
        self.combat_blocked_attackers.clear();
    }

    /// Resumes rules procedures in the order their interrupted operations
    /// require. Nothing here grants priority; the caller drains this queue
    /// before state-based actions and trigger placement.
    pub(super) fn continue_pending_procedures(&mut self) {
        while self.result.is_none()
            && self.pending_decisions.is_empty()
            && self.pending_events.is_empty()
        {
            let Some(procedure) = self.pending_procedures.pop_front() else {
                return;
            };
            let mut later_procedures = std::mem::take(&mut self.pending_procedures);
            match procedure {
                PendingProcedure::DrawCards { player, remaining } => {
                    self.draw_cards(player, remaining);
                }
                PendingProcedure::ResolveEffects {
                    effects,
                    object,
                    context,
                } => self.resolve_effects_in_order(effects, &object, context),
                PendingProcedure::ForEachInBinding {
                    objects,
                    binding,
                    next,
                    effect,
                    object,
                    context,
                } => self
                    .resolve_for_each_in_binding(objects, binding, next, effect, &object, context),
                PendingProcedure::SimultaneousDraws {
                    remaining,
                    next,
                    was_deferred,
                } => self.continue_simultaneous_draws(remaining, next, was_deferred),
                PendingProcedure::ShuffleLibrary { player } => {
                    self.rng.shuffle(&mut self.players[player.index()].library);
                }
                PendingProcedure::FinishStackResolution { object, resolved } => {
                    self.finish_stack_resolution(&object, resolved);
                }
                PendingProcedure::FinishStepAdvance => self.finish_step_advance(),
            }
            self.pending_procedures.append(&mut later_procedures);
        }
    }

    pub(super) fn resolve_effects_in_order(
        &mut self,
        mut effects: Vec<super::ScopedEffect>,
        object: &super::StackObject,
        context: impl Into<EffectResolutionContext>,
    ) {
        let mut context = context.into();
        let mut later_procedures = std::mem::take(&mut self.pending_procedures);
        while !effects.is_empty() {
            let effect = effects.remove(0);
            context = if let EffectDef::BindOutput { .. } = effect.effect {
                self.resolve_bound_output_effect(effect, object, context)
            } else {
                self.resolve_effect_def(effect, object, context.fork_resolution());
                context
            };
            if !self.pending_decisions.is_empty()
                || !self.pending_events.is_empty()
                || !self.pending_procedures.is_empty()
            {
                if !effects.is_empty() {
                    self.pending_procedures
                        .push_back(PendingProcedure::ResolveEffects {
                            effects,
                            object: Box::new(object.clone()),
                            context,
                        });
                }
                self.pending_procedures.append(&mut later_procedures);
                return;
            }
        }
        self.pending_procedures.append(&mut later_procedures);
    }

    pub(super) fn empty_mana_pools(&mut self) {
        let mana_burn = self.format.rules().mana_burn;
        for player in [PlayerId::One, PlayerId::Two] {
            let amount = self.players[player.index()].mana_pool.total();
            self.players[player.index()].mana_pool = ManaPool::default();
            self.players[player.index()].mana.clear();
            if mana_burn && amount > 0 {
                if !self.life_total_cannot_change(player) {
                    let amount_as_i16 = i16::try_from(amount).unwrap_or(i16::MAX);
                    self.players[player.index()].life -= amount_as_i16;
                }
                self.events.push(GameEvent::ManaBurn { player, amount });
            }
        }
        self.check_state_based_actions();
    }

    pub(super) fn finish(&mut self, result: GameResult) {
        self.result = Some(result);
        self.events.push(GameEvent::GameEnded { result });
    }
}
