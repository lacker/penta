use super::{
    AbilityEffectExpiration, Action, AlternativeCastKindDef, CardBehavior, CardDefinitionId,
    CardType, CombatDamageStage, CommittedTriggerEvent, DecisionContinuation, DecisionOption,
    DecisionPreference, DecisionVisibility, DecisionZone, DeclarativeAbilityDef,
    DeferredBeginTurnEffect, EffectDef, Game, GameEvent, GameObjectId, GameResult, ManaPool,
    PendingProcedure, PlayerId, ReplacementEventDef, Step, TriggerContext, TurnStepDef,
    one_or_none,
};

mod begin_turn;

impl Game {
    fn skips_turn_based_untap(&self, permanent: &super::Permanent) -> bool {
        self.does_not_untap_during_untap_step(permanent)
    }

    pub(super) fn untap_actions(&self, player: PlayerId) -> Vec<Action> {
        let lands: Vec<_> = self
            .battlefield
            .iter()
            .filter(|permanent| {
                permanent.controller == player
                    && permanent.tapped
                    && !self.skips_turn_based_untap(permanent)
                    && self
                        .permanent_types(permanent)
                        .is_some_and(|types| types.contains(CardType::Land))
            })
            .map(|permanent| permanent.card.id)
            .collect();
        let creatures: Vec<_> = self
            .battlefield
            .iter()
            .filter(|permanent| {
                permanent.controller == player
                    && permanent.tapped
                    && !self.skips_turn_based_untap(permanent)
                    && self.power(permanent).is_some()
            })
            .map(|permanent| permanent.card.id)
            .collect();
        let land_choices = if self.winter_orb_active() {
            one_or_none(&lands)
        } else {
            vec![lands]
        };
        let creature_choices = if self.count_behavior(CardBehavior::Smoke) > 0 {
            one_or_none(&creatures)
        } else {
            vec![creatures]
        };
        let mut actions = Vec::new();
        for land in &land_choices {
            for creature in &creature_choices {
                let mut permanents = land.clone();
                permanents.extend(creature);
                permanents.sort_unstable();
                permanents.dedup();
                actions.push(Action::ChooseUntap { permanents });
            }
        }
        actions
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
                permanent.tapped = false;
            }
        }
        self.untap_pending = false;
        self.priority = self.active_player;
        self.handle_upkeep_triggers();
    }

    /// Commits every life gain in one place so replacement and triggered
    /// abilities observe spells, lifelink, and card-specific drains through
    /// the same event path. Gaining nothing is not a life-gain event.
    pub(super) fn gain_life(&mut self, player: PlayerId, amount: u16) {
        if amount == 0 {
            return;
        }
        let amount = amount.saturating_mul(self.life_gain_multiplier(player));
        self.players[player.index()].life = self.players[player.index()]
            .life
            .saturating_add(i16::try_from(amount).unwrap_or(i16::MAX));
        self.capture_battlefield_triggers(&CommittedTriggerEvent::LifeGained { player, amount });
    }

    /// How much a life gain is scaled by the replacement effects on the
    /// battlefield. CR 616.1 lets the affected player order these, but the
    /// order of pure multipliers cannot change their product.
    pub(super) fn life_gain_multiplier(&self, player: PlayerId) -> u16 {
        let mut multiplier = 1u16;
        for permanent in &self.battlefield {
            self.for_each_effective_ability(permanent, |effective| {
                let ability = effective.ability;
                let DeclarativeAbilityDef::Replacement(definition) = ability.definition else {
                    return;
                };
                let ReplacementEventDef::WouldGainLife(relation) = definition.event else {
                    return;
                };
                let Some(EffectDef::MultiplyEventAmount(factor)) = ability.declarative_effect()
                else {
                    return;
                };
                if ability.is_executable()
                    && self.player_relation_matches(
                        player,
                        relation,
                        permanent.controller,
                        TriggerContext::empty(),
                    )
                {
                    multiplier = multiplier.saturating_mul(u16::from(factor));
                }
            });
        }
        multiplier
    }

    /// Life loss that is not damage: no source deals it, nothing that
    /// triggers on damage sees it, and prevention does not apply.
    pub(super) fn lose_life(&mut self, player: PlayerId, amount: u16) {
        let amount_as_i16 = i16::try_from(amount).unwrap_or(i16::MAX);
        self.players[player.index()].life -= amount_as_i16;
        self.events.push(GameEvent::LifeLost { player, amount });
    }

    pub(super) fn deal_damage(&mut self, player: PlayerId, amount: u16) {
        let amount_as_i16 = i16::try_from(amount).unwrap_or(i16::MAX);
        self.players[player.index()].life -= amount_as_i16;
        self.events.push(GameEvent::DamageDealt { player, amount });
    }

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

    /// A miracle window belongs to one card sitting in hand. Once that card
    /// has been cast, discarded, or otherwise moved, there is nothing left to
    /// pay a miracle cost for.
    pub(super) fn close_stale_miracle_window(&mut self) {
        if let Some(card) = self.miracle_window
            && !self
                .players
                .iter()
                .any(|player| player.hand.iter().any(|held| held.id == card))
        {
            self.miracle_window = None;
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
                if !(self.turn == 1 && self.active_player == PlayerId::One) {
                    let _ = self.draw_card(self.active_player);
                    if !self.pending_decisions.is_empty() || !self.pending_events.is_empty() {
                        self.pending_procedures
                            .push_back(PendingProcedure::FinishStepAdvance);
                        return;
                    }
                }
            }
            Step::Draw => self.step = Step::PrecombatMain,
            Step::PrecombatMain => self.step = Step::BeginningOfCombat,
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
                self.clear_combat();
                // An extra combat phase replaces the move to the second main,
                // which still happens once the extra combats are spent.
                if self.additional_combat_phases > 0 {
                    self.additional_combat_phases -= 1;
                    self.step = Step::BeginningOfCombat;
                } else {
                    self.step = Step::PostcombatMain;
                }
            }
            Step::PostcombatMain => {
                self.step = Step::End;
                self.handle_end_step();
            }
            Step::End => {
                self.step = Step::Cleanup;
                self.cleanup();
                if !self.cleanup_pending {
                    return;
                }
            }
            Step::Cleanup => {
                self.start_next_turn();
                return;
            }
        }

        self.finish_step_advance();
    }

    fn finish_step_advance(&mut self) {
        if self.result.is_none() {
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

    fn commit_next_turn(&mut self, next_player: PlayerId, deferred: Vec<DeferredBeginTurnEffect>) {
        // CR 614.10b: an action coupled to a skipped turn is the first thing
        // that happens in the next turn that actually occurs. Do not expose it
        // while another prospective turn can still be replaced.
        self.perform_deferred_begin_turn_effects(next_player, deferred);
        self.turn += 1;
        self.active_player = next_player;
        self.turns_started[self.active_player.index()] += 1;
        let turns_started = self.turns_started;
        for permanent in &mut self.battlefield {
            permanent
                .temporary_granted_abilities
                .retain(|grant| match grant.expiration {
                    AbilityEffectExpiration::UpkeepOf(player) => player != self.active_player,
                    AbilityEffectExpiration::TurnOf { player, turn } => {
                        turns_started[player.index()] < turn
                    }
                    AbilityEffectExpiration::EndOfTurn | AbilityEffectExpiration::Never => true,
                });
            permanent
                .temporary_removed_abilities
                .retain(|removal| match removal.expiration {
                    AbilityEffectExpiration::UpkeepOf(player) => player != self.active_player,
                    AbilityEffectExpiration::TurnOf { player, turn } => {
                        turns_started[player.index()] < turn
                    }
                    AbilityEffectExpiration::EndOfTurn | AbilityEffectExpiration::Never => true,
                });
        }
        self.creature_died_this_turn = false;
        self.sorcery_flash_grants = [0; 2];
        self.additional_combat_phases = 0;
        self.noncreature_casts_locked = [false; 2];
        self.spells_cast_last_turn = self.spells_cast_this_turn;
        self.spells_cast_this_turn = [0; 2];
        self.cards_drawn_this_turn = [0; 2];
        self.drawn_this_turn = [Vec::new(), Vec::new()];
        self.miracle_window = None;
        self.step = Step::Upkeep;
        self.players[self.active_player.index()].land_played_this_turn = false;
        // "Until your next turn" means the one now beginning, not the one the
        // ability resolved during.
        let started = self.turns_started[self.active_player.index()];
        self.floating_triggers.retain(|floating| {
            floating.until_turn_of != self.active_player || started <= floating.created_after_turns
        });
        for permanent in &mut self.battlefield {
            permanent
                .keywords_until_upkeep_of
                .retain(|(player, _)| *player != self.active_player);
            // One loyalty ability per planeswalker per turn, so the allowance
            // returns as the turn does.
            permanent.activated_loyalty_this_turn = false;
        }
        let winter_orb = self.winter_orb_active();
        let smoke = self.count_behavior(CardBehavior::Smoke) > 0;
        let restricted_lands: Vec<_> = self
            .battlefield
            .iter()
            .filter(|permanent| {
                self.permanent_types(permanent)
                    .is_some_and(|types| types.contains(CardType::Land))
            })
            .map(|permanent| permanent.card.id)
            .collect();
        let restricted_creatures: Vec<_> = self
            .battlefield
            .iter()
            .filter(|permanent| self.power(permanent).is_some())
            .map(|permanent| permanent.card.id)
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
                let restricted = (winter_orb && restricted_lands.contains(&permanent.card.id))
                    || (smoke && restricted_creatures.contains(&permanent.card.id));
                if restricted && permanent.tapped && !untap_restricted.contains(&permanent.card.id)
                {
                    self.untap_pending = true;
                } else if !untap_restricted.contains(&permanent.card.id) {
                    permanent.tapped = false;
                }
            }
        }
        if !self.untap_pending {
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
        self.fire_delayed_triggers(TurnStepDef::Upkeep);
    }

    pub(super) fn handle_end_step(&mut self) {
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
        if self.players[self.active_player.index()].hand.len() > 7 {
            self.cleanup_pending = true;
        } else {
            self.complete_cleanup();
        }
    }

    pub(super) fn complete_cleanup(&mut self) {
        self.channel_active[self.active_player.index()] = false;
        self.finish_cleanup();
        self.empty_mana_pools();
        if self.result.is_none() {
            self.start_next_turn();
        }
    }

    pub(super) fn finish_cleanup(&mut self) {
        self.temporary_ability_grants.clear();
        for replacements in &mut self.draw_replacements {
            replacements.clear();
        }
        for permanent in &mut self.battlefield {
            permanent.damage = 0;
            permanent.exile_instead_of_dying = false;
            permanent.damage_sources.clear();
            permanent.deathtouch_damage = false;
            permanent.power_bonus = 0;
            permanent.toughness_bonus = 0;
            permanent.temporary_keywords.clear();
            permanent
                .temporary_granted_abilities
                .retain(|grant| grant.expiration != AbilityEffectExpiration::EndOfTurn);
            permanent
                .temporary_removed_abilities
                .retain(|removal| removal.expiration != AbilityEffectExpiration::EndOfTurn);
            if let Some(owner) = permanent.control_reverts_to.take() {
                permanent.controller = owner;
            }
            permanent.unblockable_this_turn = false;
            permanent.combat_damage_prevented = false;
            permanent.combat_damage_dealt_by_prevented = false;
            permanent.destroy_at_end = false;
            permanent.animation = None;
            permanent.activations_this_turn.clear();
            permanent.dealt_damage_to_opponent_this_turn = false;
            permanent.regeneration_shields = 0;
            permanent.attacked_this_turn = false;
            permanent.attacks_this_turn = 0;
        }
    }

    pub(super) fn clear_combat(&mut self) {
        for permanent in &mut self.battlefield {
            permanent.attacking = false;
            permanent.blocked = false;
            permanent.blocking = None;
            permanent.combat_damage_assignment.clear();
        }
        self.pending_combat_attackers.clear();
        self.combat_damage_stage = CombatDamageStage::NotStarted;
        self.combat_blocked_attackers.clear();
    }

    pub(super) fn winter_orb_active(&self) -> bool {
        self.battlefield.iter().any(|permanent| {
            !permanent.tapped && self.effective_behavior(permanent) == Some(CardBehavior::WinterOrb)
        })
    }

    pub(super) fn draw_card(&mut self, player: PlayerId) -> Option<GameObjectId> {
        if self.draw_replacements[player.index()].len() > 1 {
            self.queue_draw_replacement_choice(player);
            return None;
        }
        if let Some(replacement) = self.draw_replacements[player.index()].pop_front() {
            self.resolve_effect_def(replacement.effect, &replacement.object, replacement.context);
            return None;
        }
        let Some(card) = self.players[player.index()].library.pop() else {
            self.players[player.index()].tried_to_draw_from_empty_library = true;
            return None;
        };
        let (card, _zone_change) = self.zone_change_card(card);
        let card_id = card.id;
        let definition = card.definition;
        self.players[player.index()].hand.push(card);
        self.events.push(GameEvent::CardDrawn {
            player,
            card: card_id,
        });
        let drawn = &mut self.cards_drawn_this_turn[player.index()];
        *drawn = drawn.saturating_add(1);
        self.drawn_this_turn[player.index()].push(card_id);
        if self.cards_drawn_this_turn[player.index()] == 1 && self.has_miracle(definition) {
            self.queue_miracle_reveal(player, card_id);
        }
        Some(card_id)
    }

    fn queue_draw_replacement_choice(&mut self, player: PlayerId) {
        let replacements = self.draw_replacements[player.index()]
            .drain(..)
            .collect::<Vec<_>>();
        let options = replacements
            .iter()
            .enumerate()
            .map(|(index, replacement)| {
                let definition = replacement.object.presentation_definition();
                let name = self
                    .catalog
                    .get(definition)
                    .map_or("Draw replacement", |card| card.name.as_str());
                DecisionOption {
                    id: u32::try_from(index).unwrap_or(u32::MAX),
                    label: replacement
                        .object
                        .ability_text()
                        .map_or_else(|| name.to_string(), |text| format!("{name} — {text}")),
                    card: None,
                    members: Vec::new(),
                    ability_text: replacement.object.ability_text().map(str::to_owned),
                    zone: DecisionZone::None,
                }
            })
            .collect();
        self.queue_decision(
            player,
            "Choose which effect replaces this draw",
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            1..=1,
            false,
            options,
            DecisionContinuation::DrawReplacement {
                player,
                replacements,
            },
        );
    }

    /// Whether a card offers a miracle cost at all.
    pub(super) fn has_miracle(&self, definition: CardDefinitionId) -> bool {
        self.catalog.get(definition).is_some_and(|definition| {
            definition.parts.iter().any(|part| {
                part.rules.ability_clauses().iter().any(|ability| {
                    ability.is_executable()
                        && matches!(
                            ability.definition,
                            DeclarativeAbilityDef::AlternativeCast(alternative)
                                if alternative.kind == AlternativeCastKindDef::Miracle
                        )
                })
            })
        })
    }

    /// Offers the reveal that opens a miracle window. Revealing is the whole
    /// choice: whether to then pay the cost is the ordinary cast decision,
    /// and declining to cast simply lets the window close.
    pub(super) fn queue_miracle_reveal(&mut self, player: PlayerId, card: GameObjectId) {
        let name = self.players[player.index()]
            .hand
            .iter()
            .find(|held| held.id == card)
            .and_then(|held| self.catalog.get(held.definition))
            .map_or_else(
                || "that card".to_string(),
                |definition| definition.name.clone(),
            );
        self.queue_decision(
            player,
            format!("Reveal {name} for its miracle cost?"),
            DecisionVisibility::Private,
            DecisionPreference::Neutral,
            1..=1,
            false,
            vec![
                DecisionOption {
                    id: 0,
                    label: "Keep it hidden".into(),
                    card: None,
                    members: Vec::new(),
                    ability_text: None,
                    zone: DecisionZone::None,
                },
                DecisionOption {
                    id: 1,
                    label: format!("Reveal {name}"),
                    card: Some((card, CardDefinitionId(0))),
                    members: Vec::new(),
                    ability_text: None,
                    zone: DecisionZone::Hand,
                },
            ],
            DecisionContinuation::MiracleReveal { card },
        );
    }

    pub(super) fn draw_cards(&mut self, player: PlayerId, count: u16) {
        if count == 0 {
            return;
        }
        if !self.pending_decisions.is_empty()
            || !self.pending_events.is_empty()
            || !self.pending_procedures.is_empty()
        {
            self.pending_procedures
                .push_back(PendingProcedure::DrawCards {
                    player,
                    remaining: count,
                });
            return;
        }
        let mut remaining = count;
        while remaining > 0 {
            if self.result.is_some() {
                break;
            }
            remaining -= 1;
            let _ = self.draw_card(player);
            if !self.pending_decisions.is_empty()
                || !self.pending_events.is_empty()
                || !self.pending_procedures.is_empty()
            {
                if remaining > 0 {
                    self.pending_procedures
                        .push_back(PendingProcedure::DrawCards { player, remaining });
                }
                break;
            }
        }
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
                    custom_followup,
                } => self.resolve_effects_in_order(effects, &object, context, custom_followup),
                PendingProcedure::SylvanAfterDraw { player } => {
                    let candidates = self.sylvan_candidates(player);
                    let choices = candidates.len().min(2);
                    if choices > 0 {
                        self.queue_sylvan_select(player, candidates, choices);
                    }
                }
                PendingProcedure::SimultaneousDraws {
                    remaining,
                    next,
                    was_deferred,
                } => self.continue_simultaneous_draws(remaining, next, was_deferred),
                PendingProcedure::ShuffleLibrary { player } => {
                    self.rng.shuffle(&mut self.players[player.index()].library);
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
        context: TriggerContext,
        custom_followup: Option<CardBehavior>,
    ) {
        let mut later_procedures = std::mem::take(&mut self.pending_procedures);
        while !effects.is_empty() {
            let effect = effects.remove(0);
            self.resolve_effect_def(effect, object, context);
            if !self.pending_decisions.is_empty()
                || !self.pending_events.is_empty()
                || !self.pending_procedures.is_empty()
            {
                if !effects.is_empty() || custom_followup.is_some() {
                    self.pending_procedures
                        .push_back(PendingProcedure::ResolveEffects {
                            effects,
                            object: Box::new(object.clone()),
                            context,
                            custom_followup,
                        });
                }
                self.pending_procedures.append(&mut later_procedures);
                return;
            }
        }
        if let Some(behavior) = custom_followup {
            self.resolve_custom_spell_followup(object, behavior);
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
                let amount_as_i16 = i16::try_from(amount).unwrap_or(i16::MAX);
                self.players[player.index()].life -= amount_as_i16;
                self.events.push(GameEvent::ManaBurn { player, amount });
            }
        }
        self.check_state_based_actions();
    }

    /// Draws every card for the active player first, then every card for the
    /// other player. Each player's draws still happen one at a time so draw
    /// replacements can suspend the instruction. One spell can deck both
    /// players, so empty-library losses remain deferred until the complete
    /// simultaneous instruction finishes. Empty-library loss is recorded on
    /// each player and settled at the next state-based-action check.
    #[cfg(test)]
    pub(super) fn draw_cards_simultaneously(&mut self, counts: [u16; 2]) {
        let was_deferred = self.defer_empty_library_loss;
        self.defer_empty_library_loss = true;
        self.continue_simultaneous_draws(counts, self.active_player, was_deferred);
    }

    fn continue_simultaneous_draws(
        &mut self,
        mut remaining: [u16; 2],
        mut next: PlayerId,
        was_deferred: bool,
    ) {
        while remaining.iter().any(|count| *count > 0) && self.result.is_none() {
            let player = next;
            if remaining[player.index()] == 0 {
                next = player.opponent();
                continue;
            }
            remaining[player.index()] -= 1;
            let _ = self.draw_card(player);
            if remaining[player.index()] == 0 {
                next = player.opponent();
            }
            if !self.pending_decisions.is_empty()
                || !self.pending_events.is_empty()
                || !self.pending_procedures.is_empty()
            {
                self.pending_procedures
                    .push_back(PendingProcedure::SimultaneousDraws {
                        remaining,
                        next,
                        was_deferred,
                    });
                return;
            }
        }
        self.defer_empty_library_loss = was_deferred;
    }

    pub(super) fn finish(&mut self, result: GameResult) {
        self.result = Some(result);
        self.events.push(GameEvent::GameEnded { result });
    }
}
