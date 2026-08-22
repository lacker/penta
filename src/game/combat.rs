use super::{
    Action, AppliedRuleDef, AttackDefender, CardType, CombatDamageAssignment, CombatDamageStage,
    CommittedTriggerEvent, ControlFlow, CounterKind, DeclarativeAbilityDef, EffectDef, Game,
    GameEvent, GameObjectId, KeywordAbility, Permanent, PlayerId, Target,
};

mod assignment;
mod damage_delivery;

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
                    && permanent.detained_until_turn_of.is_none()
                    && self.can_attack_with_moat(permanent, moat_active)
            })
            .flat_map(|permanent| {
                defenders
                    .iter()
                    .copied()
                    .filter(|defender| self.attack_restrictions_allow(permanent, *defender))
                    .map(|defender| Action::DeclareAttacker {
                        attacker: permanent.card.id,
                        defender,
                    })
            })
            .collect()
    }

    fn attack_restrictions_allow(&self, attacker: &Permanent, defender: AttackDefender) -> bool {
        let AttackDefender::Player(defender) = defender else {
            return true;
        };
        self.resolved_attack_restrictions
            .iter()
            .filter(|restriction| {
                restriction.affected_player == defender
                    && self.continuous_effect_expiration_is_active(
                        restriction.expiration,
                        restriction.source.object,
                    )
            })
            .all(|restriction| {
                self.trigger_object_matches(
                    restriction.allowed_attacker,
                    &self.trigger_event_object(attacker),
                    restriction.source.object,
                    false,
                )
            })
    }

    #[cfg(test)]
    pub(super) fn can_attack(&self, permanent: &Permanent) -> bool {
        let moat_active = self.count_behavior(super::CardBehavior::Moat) > 0;
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
        if self.permanent_has_executable_keyword(permanent, KeywordAbility::Defender)
            && !self.has_applied_rule(permanent, AppliedRuleDef::MayAttackDespiteDefender)
        {
            return false;
        }
        if moat_active && !flying {
            return false;
        }
        if !self.attack_restrictions_met(permanent) || self.cannot_attack(permanent) {
            return false;
        }
        self.permanent_has_executable_keyword(permanent, KeywordAbility::Haste)
            || self.has_applied_rule(permanent, AppliedRuleDef::MayAttackAsThoughHasty)
            || self.turns_started[permanent.controller.index()] > permanent.entered_controller_turn
    }

    /// Whether every "can't attack unless ..." clause this creature prints is
    /// currently satisfied. The query carries its own controller relation, so
    /// "defending player" is read as the attacker's opponent -- which is the
    /// only defending player there is in a two-player game.
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

    pub(super) fn declare_attacker(&mut self, attacker: GameObjectId, defender: AttackDefender) {
        let vigilance = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == attacker)
            .is_some_and(|permanent| {
                self.permanent_has_executable_keyword(permanent, KeywordAbility::Vigilance)
            });
        let turns_started = self.turns_started;
        if let Some(permanent) = self
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == attacker)
        {
            permanent.attacking = true;
            permanent.attack_defender = Some(defender);
            permanent.attacked_this_turn = true;
            permanent.attacks_this_turn = permanent.attacks_this_turn.saturating_add(1);
            permanent.last_attacked_turn = Some((
                permanent.controller,
                turns_started[permanent.controller.index()],
            ));
            if !vigilance {
                // Tapping is part of the single CR 508.1 declaration. Commit
                // the state now for later attacker legality, but defer its
                // trigger event until every attacker has been declared.
                permanent.tapped = true;
            }
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
        // Declaration size and attack number are facts of this event, not
        // mutable conditions to recheck while placing the trigger.
        let declaration_size = u8::try_from(attackers.len()).unwrap_or(u8::MAX);
        let listeners = self.battlefield_trigger_listeners();
        let mut events = attackers
            .iter()
            .filter_map(|attacker| {
                self.battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == *attacker && permanent.tapped)
                    .map(|permanent| CommittedTriggerEvent::Tapped {
                        object: self.trigger_event_object(permanent),
                        for_mana: false,
                    })
            })
            .collect::<Vec<_>>();
        events.extend(attackers.iter().filter_map(|attacker| {
            self.battlefield
                .iter()
                .find(|permanent| permanent.card.id == *attacker)
                .map(|permanent| {
                    let defending_player = match Self::combat_defender(permanent) {
                        AttackDefender::Player(player) => player,
                        AttackDefender::Planeswalker(walker) => self
                            .battlefield
                            .iter()
                            .find(|candidate| candidate.card.id == walker)
                            .map_or(permanent.controller.opponent(), |walker| walker.controller),
                    };
                    CommittedTriggerEvent::Attacks {
                        object: self.trigger_event_object(permanent),
                        declaration_size,
                        attack_number: permanent.attacks_this_turn,
                        defending_player,
                    }
                })
        }));
        // And the declaration itself, once: "whenever you attack" is one
        // trigger however many creatures were declared (CR 508.1).
        events.push(CommittedTriggerEvent::AttackersDeclared {
            attackers: attackers
                .iter()
                .filter_map(|attacker| {
                    self.battlefield
                        .iter()
                        .find(|permanent| permanent.card.id == *attacker)
                        .map(|permanent| self.trigger_event_object(permanent))
                })
                .collect(),
        });
        self.capture_battlefield_trigger_batch_from_snapshot(&listeners, &events);
    }

    /// Whether a static effect on `blocker` narrows what it may block, and
    /// this attacker is outside that. The other direction -- an attacker
    /// forbidding a blocker -- is `blocking_is_prevented`.
    pub(super) fn blocker_may_only_block(&self, blocker: &Permanent, attacker: &Permanent) -> bool {
        // Asked while blockers are declared, which is outside static
        // resolution, so this reads the real current numbers: "power 2 or
        // greater" has to see a creature a Crusade has pumped.
        let characteristics = self.targeting_event_object(attacker);
        let mut restricted = false;
        let _ = self.visit_applied_rules(blocker, |applied| {
            if let AppliedRuleDef::CanBlockOnly(predicate) = applied.rule
                && !self.trigger_object_matches(predicate, &characteristics, applied.source, false)
            {
                restricted = true;
                return ControlFlow::Break(());
            }
            ControlFlow::Continue(())
        });
        restricted
    }

    /// Whether a static effect on `attacker` forbids `blocker` from blocking
    /// it, as Juggernaut forbids Walls.
    pub(super) fn blocking_is_prevented(&self, attacker: &Permanent, blocker: &Permanent) -> bool {
        let characteristics = self.trigger_event_object(blocker);
        let mut prevented = false;
        let result = self.visit_applied_rules(attacker, |applied| {
            if let AppliedRuleDef::CannotBeBlockedBy(predicate) = applied.rule
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

    /// Whether a static or resolved rule on `attacker` requires `blocker` to
    /// block it, as Lure requires every creature that can. Read from the
    /// attacker for the same reason the prohibition above is: the printed
    /// text sits on the creature being blocked, not on the ones doing it.
    fn must_be_blocked_by(&self, attacker: &Permanent, blocker: &Permanent) -> bool {
        let characteristics = self.trigger_event_object(blocker);
        let mut required = false;
        let _ = self.visit_applied_rules(attacker, |applied| {
            if let AppliedRuleDef::MustBeBlockedBy(predicate) = applied.rule
                && self.trigger_object_matches(predicate, &characteristics, applied.source, false)
            {
                required = true;
                return ControlFlow::Break(());
            }
            ControlFlow::Continue(())
        });
        required
    }

    /// Whether a continuous effect currently stops anything blocking this
    /// permanent. Asked afresh, so a resolved rule stops when its duration
    /// expires and a static one when its source leaves.
    fn cannot_be_blocked(&self, permanent: &Permanent) -> bool {
        self.has_applied_rule(permanent, AppliedRuleDef::CannotBeBlocked)
    }

    /// Whether a continuous effect from anywhere forbids this creature from
    /// attacking. The printed "can't attack unless ..." clause is a separate
    /// question a creature asks about itself.
    pub(super) fn cannot_attack(&self, permanent: &Permanent) -> bool {
        self.has_applied_rule(permanent, AppliedRuleDef::CannotAttack)
    }

    /// Whether this creature may still be declared as a blocker.
    ///
    /// Every creature may block one attacker; a rule can raise that. The
    /// allowance counts attackers rather than declarations, which is what
    /// makes a band cost one blocker no matter how many creatures are in it.
    fn has_blocks_left(&self, permanent: &Permanent) -> bool {
        let mut allowance = 1_usize;
        let _ = self.visit_applied_rules(permanent, |applied| {
            if let AppliedRuleDef::MayBlockAdditionalCreatures(extra) = applied.rule {
                allowance = if extra == u8::MAX {
                    usize::MAX
                } else {
                    allowance.saturating_add(usize::from(extra))
                };
            }
            ControlFlow::Continue(())
        });
        // Spent per band rather than per attacker: a band is one block
        // however many creatures are in it. A lone attacker counts as its own
        // band, so it is named by its own id.
        let mut spent: Vec<_> = permanent
            .blocking
            .iter()
            .map(|attacker| {
                self.battlefield
                    .iter()
                    .find(|other| other.card.id == *attacker)
                    .and_then(|other| other.attacking_band)
                    .map_or((None, Some(*attacker)), |band| (Some(band), None))
            })
            .collect();
        spent.sort_unstable();
        spent.dedup();
        spent.len() < allowance
    }

    pub(super) fn cannot_block(&self, permanent: &Permanent) -> bool {
        if permanent.detained_until_turn_of.is_some() {
            return true;
        }
        // Unleash: the counter is what stops it blocking, so a creature that
        // declined the counter blocks as normal and one that took it never
        // does again.
        if permanent.counters(CounterKind::PlusOnePlusOne) > 0
            && self.permanent_has_executable_keyword(permanent, KeywordAbility::Unleash)
        {
            return true;
        }
        self.has_applied_rule(permanent, AppliedRuleDef::CannotBlock)
    }

    /// The blocks this player may declare, after combat requirements have
    /// taken the alternatives away.
    ///
    /// CR 509.1c asks for the maximum possible number of requirements to be
    /// obeyed without violating a restriction. A creature that is able to
    /// block a must-be-blocked attacker therefore has no other legal
    /// assignment: every block it makes elsewhere obeys one requirement
    /// fewer. Two such attackers leave it a choice between them, because it
    /// can only block one either way.
    pub(super) fn blocker_actions(&self, player: PlayerId) -> Vec<Action> {
        let available = self.available_blocker_actions(player);
        let required: Vec<GameObjectId> = available
            .iter()
            .filter_map(|action| self.required_block(action).map(|(blocker, _)| blocker))
            .collect();
        if required.is_empty() {
            return available;
        }
        available
            .into_iter()
            .filter(|action| match action {
                Action::DeclareBlocker { blocker, .. } => {
                    !required.contains(blocker) || self.required_block(action).is_some()
                }
                _ => true,
            })
            .collect()
    }

    /// The blocker and attacker of a declaration a requirement compels.
    fn required_block(&self, action: &Action) -> Option<(GameObjectId, GameObjectId)> {
        let Action::DeclareBlocker { blocker, attacker } = action else {
            return None;
        };
        let find = |id: GameObjectId| {
            self.battlefield
                .iter()
                .find(|permanent| permanent.card.id == id)
        };
        let (blocker_permanent, attacker_permanent) = (find(*blocker)?, find(*attacker)?);
        let required = self.must_be_blocked_by(attacker_permanent, blocker_permanent)
            || self.has_applied_rule(
                blocker_permanent,
                AppliedRuleDef::MustBlockEachAttackerIfAble,
            );
        required.then_some((*blocker, *attacker))
    }

    /// Whether a requirement is still unmet, which is what stops the
    /// defending player from finishing the declaration.
    pub(super) fn block_requirement_outstanding(&self, available: &[Action]) -> bool {
        available
            .iter()
            .any(|action| self.required_block(action).is_some())
    }

    /// Whether some menacing attacker is blocked by exactly one creature.
    ///
    /// CR 702.110a is a constraint on the finished declaration, not on any
    /// single block: the first blocker is perfectly legal and only becomes
    /// illegal by being the last. So it is checked where the declaration
    /// ends rather than where each block is offered.
    pub(super) fn menace_is_unsatisfied(&self, player: PlayerId) -> bool {
        self.battlefield
            .iter()
            .filter(|permanent| {
                permanent.attacking
                    && self.permanent_has_executable_keyword(permanent, KeywordAbility::Menace)
            })
            .any(|menacing| {
                self.battlefield
                    .iter()
                    .filter(|candidate| {
                        candidate.controller == player && candidate.is_blocking(menacing.card.id)
                    })
                    .count()
                    == 1
            })
    }

    /// Whether this blocker could legally be declared against this one
    /// attacker, ignoring how many blocks it has left. A band asks this of
    /// every one of its members, because blocking a band blocks all of them.
    fn blocker_may_block(&self, blocker_permanent: &Permanent, attacker: GameObjectId) -> bool {
        let Some(attacker_permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == attacker)
        else {
            return false;
        };
        if blocker_permanent.is_blocking(attacker) {
            // A second declaration against the same attacker would spend one
            // of the blocker's allowance on a block it already has.
            return false;
        }
        if self.landwalk_beats(attacker_permanent, attacker_permanent.controller.opponent())
            || self.cannot_be_blocked(attacker_permanent)
            || self.blocking_is_prevented(attacker_permanent, blocker_permanent)
            || self.blocker_may_only_block(blocker_permanent, attacker_permanent)
            || self.combat_is_protected(blocker_permanent, attacker_permanent)
        {
            return false;
        }
        if self.has_flying(attacker_permanent)
            && !self.has_flying(blocker_permanent)
            && !self.permanent_has_executable_keyword(blocker_permanent, KeywordAbility::Reach)
        {
            return false;
        }
        if self.permanent_has_executable_keyword(attacker_permanent, KeywordAbility::Intimidate)
            && !self.is_artifact_permanent(blocker_permanent)
        {
            let shares_color = self
                .permanent_colors(attacker_permanent)
                .into_iter()
                .zip(self.permanent_colors(blocker_permanent))
                .any(|(attacker, blocker)| attacker && blocker);
            if !shares_color {
                return false;
            }
        }
        true
    }

    /// The attacking bands, each named by its lowest-numbered member. A lone
    /// attacker is a group of one, so this is every attacker when nobody has
    /// banded.
    fn attacking_groups(&self) -> Vec<(GameObjectId, Vec<GameObjectId>)> {
        self.battlefield
            .iter()
            .filter(|permanent| permanent.attacking)
            .filter_map(|permanent| {
                let group = self.band_group(permanent.card.id);
                let representative = group.iter().copied().min()?;
                (representative == permanent.card.id).then_some((representative, group))
            })
            .collect()
    }

    fn available_blocker_actions(&self, player: PlayerId) -> Vec<Action> {
        let blockers: Vec<_> = self
            .battlefield
            .iter()
            .filter(|permanent| {
                permanent.controller == player
                    && !permanent.tapped
                    && self.has_blocks_left(permanent)
                    && self.power(permanent).is_some()
                    && !self.cannot_block(permanent)
            })
            .map(|permanent| permanent.card.id)
            .collect();
        // One offer per band rather than per creature: blocking any member
        // blocks the whole band, so naming a different member would be the
        // same declaration.
        let groups = self.attacking_groups();
        blockers
            .into_iter()
            .flat_map(|blocker| {
                let blocker_permanent = self
                    .battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == blocker)
                    .expect("blocker is on the battlefield");
                groups.iter().filter_map(move |(attacker, group)| {
                    group
                        .iter()
                        .all(|member| self.blocker_may_block(blocker_permanent, *member))
                        .then_some(Action::DeclareBlocker {
                            blocker,
                            attacker: *attacker,
                        })
                })
            })
            .collect()
    }

    pub(super) fn declare_blocker(&mut self, blocker: GameObjectId, attacker: GameObjectId) {
        // A band is blocked as a group: one declaration against any member
        // puts the blocker in front of every creature in the band.
        let band = self.band_group(attacker);
        if let Some(permanent) = self
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == blocker)
        {
            for member in &band {
                if !permanent.blocking.contains(member) {
                    permanent.blocking.push(*member);
                }
            }
        }
        for member in band {
            if !self.combat_blocked_attackers.contains(&member) {
                self.combat_blocked_attackers.push(member);
            }
        }
    }

    pub(super) fn finish_declaring_blockers(&mut self) {
        self.blockers_declared = true;
        self.priority = self.active_player;
        self.consecutive_passes = 0;
        let blocked = self
            .battlefield
            .iter()
            .flat_map(|permanent| permanent.blocking.iter().copied())
            .collect::<Vec<_>>();
        for permanent in &mut self.battlefield {
            permanent.blocked = blocked.contains(&permanent.card.id);
            // The declaration is the moment blocker status is established.
            // After this, attackers can leave and empty the list above; the
            // creatures that blocked them are still blocking creatures.
            permanent.blocking_this_combat |= permanent.is_blocking_anything();
        }
        let assignments = self
            .battlefield
            .iter()
            .flat_map(|permanent| {
                permanent
                    .blocking
                    .iter()
                    .map(|attacker| (permanent.card.id, *attacker))
            })
            .collect::<Vec<_>>();
        if !assignments.is_empty() {
            self.events.push(GameEvent::BlockDeclared {
                player: self.active_player.opponent(),
                assignments,
            });
        }
        // Blocking is one declaration. Freeze its listeners and every
        // object-local event before a triggered-mana ability can mutate the
        // battlefield while the declaration is being published.
        let listeners = self.battlefield_trigger_listeners();
        let mut trigger_events = self.becomes_blocked_trigger_events(&blocked);
        trigger_events.extend(self.blocking_relationship_trigger_events());
        trigger_events.extend(self.unblocked_attacker_trigger_events(&blocked));
        self.capture_battlefield_trigger_batch_from_snapshot(&listeners, &trigger_events);
    }

    /// CR 509.1h leaves an attacker nobody blocked unblocked, which is what
    /// these clauses read. It can only be answered once blocking is done.
    fn unblocked_attacker_trigger_events(
        &self,
        blocked: &[GameObjectId],
    ) -> Vec<CommittedTriggerEvent> {
        self.battlefield
            .iter()
            .filter(|permanent| permanent.attacking && !blocked.contains(&permanent.card.id))
            .map(|permanent| CommittedTriggerEvent::AttacksAndIsNotBlocked {
                object: self.trigger_event_object(permanent),
            })
            .collect()
    }

    /// One event per ordered pair of a blocker and what it blocks, so a
    /// clause printed on either creature reads the other as the triggering
    /// object. "Blocks or becomes blocked by" is one clause, not two.
    fn blocking_relationship_trigger_events(&self) -> Vec<CommittedTriggerEvent> {
        let pairs = self
            .battlefield
            .iter()
            .flat_map(|permanent| {
                permanent
                    .blocking
                    .iter()
                    .map(|attacker| (permanent.card.id, *attacker))
            })
            .collect::<Vec<_>>();
        let mut events = Vec::with_capacity(pairs.len().saturating_mul(2));
        for (blocker, attacker) in pairs {
            let Some((blocker, attacker)) = self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == blocker)
                .map(|permanent| self.trigger_event_object(permanent))
                .zip(
                    self.battlefield
                        .iter()
                        .find(|permanent| permanent.card.id == attacker)
                        .map(|permanent| self.trigger_event_object(permanent)),
                )
            else {
                continue;
            };
            for (creature, other) in [(blocker.clone(), attacker.clone()), (attacker, blocker)] {
                events.push(CommittedTriggerEvent::BlocksOrBecomesBlocked { creature, other });
            }
        }
        events
    }

    /// CR 509.1h. Each attacker becomes blocked once, however many creatures
    /// blocked it, so the event fires per attacker and carries the count the
    /// rampage-style clauses are written against.
    fn becomes_blocked_trigger_events(
        &self,
        blocked: &[GameObjectId],
    ) -> Vec<CommittedTriggerEvent> {
        let mut attackers = blocked.to_vec();
        attackers.sort_unstable();
        attackers.dedup();
        attackers
            .into_iter()
            .filter_map(|attacker| {
                let blockers = blocked.iter().filter(|id| **id == attacker).count();
                self.battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == attacker)
                    .map(|permanent| CommittedTriggerEvent::BecomesBlocked {
                        object: self.trigger_event_object(permanent),
                        blockers_beyond_first: u16::try_from(blockers.saturating_sub(1))
                            .unwrap_or(u16::MAX),
                    })
            })
            .collect()
    }

    pub(super) fn start_combat_damage(&mut self) {
        // Tests and a few internal procedures can construct combat directly,
        // so also capture live blocking relationships here. During an ordinary
        // game, `declare_blocker` recorded them before either player received
        // priority and they therefore survive a blocker leaving the field.
        let newly_blocked = self
            .battlefield
            .iter()
            .flat_map(|permanent| permanent.blocking.iter().copied())
            .collect::<Vec<_>>();
        for attacker in newly_blocked {
            if !self.combat_blocked_attackers.contains(&attacker) {
                self.combat_blocked_attackers.push(attacker);
            }
        }
        // Blocker status, from the same directly constructed relationships.
        for permanent in &mut self.battlefield {
            permanent.blocking_this_combat |= permanent.is_blocking_anything();
        }

        let strike_wave_combatants = self
            .battlefield
            .iter()
            .filter(|permanent| permanent.attacking || permanent.is_blocking_this_combat())
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
        if self.has_applied_rule(permanent, AppliedRuleDef::AssignsNoCombatDamage) {
            return false;
        }
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
        self.pending_combat_assignments = self
            .battlefield
            .iter()
            .filter(|permanent| {
                // A creature blocking more than one attacker divides its
                // damage too, so blockers join attackers in the queue.
                (permanent.attacking || permanent.is_blocking_anything())
                    && self.deals_damage_in_current_combat_step(permanent)
            })
            // Ask exactly when there is a real choice. One blocker and no
            // trample leaves a single legal distribution and no question; one
            // blocker plus trample is a genuine decision about how much to
            // spill past it.
            .filter(|permanent| self.combat_assignment_actions(permanent.card.id).len() > 1)
            .map(|permanent| permanent.card.id)
            .collect();
        if self.pending_combat_assignments.is_empty() {
            self.deal_combat_damage();
        }
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
        source: GameObjectId,
        assignments: Vec<CombatDamageAssignment>,
    ) {
        if let Some(permanent) = self
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == source)
        {
            permanent.combat_damage_assignment = assignments;
        }
        self.pending_combat_assignments.remove(0);
        if self.pending_combat_assignments.is_empty() {
            self.deal_combat_damage();
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
            let attacker_deals_damage =
                self.deals_damage_in_current_combat_step(&self.battlefield[attacker_index]);
            let blockers: Vec<_> = self
                .battlefield
                .iter()
                .filter(|permanent| permanent.is_blocking(attacker_id))
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
            } else if !blockers.is_empty() && attacker_deals_damage {
                self.deal_attacker_combat_damage(attacker_id, attacker_index, &blockers);
            }
        }
        self.deal_blocker_combat_damage();
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
