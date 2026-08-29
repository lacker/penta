//! Preparation and atomic commitment of simultaneous damage events.

use super::{
    CardType, CommittedTriggerEvent, CounterKind, DamageAssignment, DamageAssignmentOutcome,
    DamageEventOutcome, DamageRecipientOutcome, Game, KeywordAbility, PlayerId, PreparedDamage,
    PreparedDamageSource, ProspectiveDamage, Target,
};

impl Game {
    /// Resolves one damage event containing every supplied assignment.
    /// Sources, recipients, powers supplied by callers, keyword results, and
    /// lethal thresholds are all frozen before any damage result changes the
    /// game. This is the common commit point for combat damage, fight, and
    /// declarative instructions that explicitly deal damage simultaneously.
    pub(in crate::game) fn deal_damage_simultaneously(
        &mut self,
        assignments: Vec<DamageAssignment>,
    ) -> DamageEventOutcome {
        let mut deferred_life_gains = Vec::new();
        let mut assigned_to_players = [0_u16; 2];
        let prepared = assignments
            .into_iter()
            .filter_map(|assignment| {
                self.prepare_damage_assignment(
                    assignment,
                    &mut assigned_to_players,
                    &mut deferred_life_gains,
                )
            })
            .collect::<Vec<_>>();
        let recipients = self.damage_recipient_outcomes(&prepared);

        self.commit_prepared_damage(&prepared, deferred_life_gains);

        DamageEventOutcome {
            assignments: prepared
                .into_iter()
                .map(|damage| DamageAssignmentOutcome {
                    source: damage.source,
                    recipient: damage.target,
                    amount: damage.amount,
                })
                .collect(),
            recipients,
        }
    }

    fn prepare_damage_assignment(
        &mut self,
        assignment: DamageAssignment,
        assigned_to_players: &mut [u16; 2],
        deferred_life_gains: &mut Vec<(PlayerId, u16)>,
    ) -> Option<PreparedDamage> {
        // CR 614.9: redirection applies before prevention. Freeze where this
        // assignment lands before any result in the event is applied.
        let target = self.redirected_damage_target(assignment.source, assignment.target)?;
        if matches!(target, Target::Card(_) | Target::Spell(_)) {
            return None;
        }
        let source_object = assignment
            .source
            .and_then(|source| self.damage_source_event_object(source));
        let source_is_spell = assignment
            .source
            .is_some_and(|source| self.damage_source_is_spell(source));
        let recipient_object = match target {
            Target::Permanent(id) => self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == id)
                .map(|permanent| self.trigger_event_object(permanent)),
            Target::Player(_) | Target::Card(_) | Target::Spell(_) => None,
        };
        if matches!(target, Target::Permanent(_)) && recipient_object.is_none() {
            return None;
        }
        let event = ProspectiveDamage {
            source: assignment.source,
            source_object: source_object.as_ref(),
            source_is_spell,
            target: Some(target),
            recipient_object: recipient_object.as_ref(),
            combat: assignment.combat,
        };
        let preventable = !self.damage_cannot_be_prevented_this_turn
            && !self.combat_damage_cannot_be_prevented(assignment.source, assignment.combat);
        let mut amount = if preventable {
            self.apply_resolved_damage_prevention(event, assignment.amount, deferred_life_gains)
        } else {
            assignment.amount
        };
        let already_assigned = match target {
            Target::Player(player) => assigned_to_players[player.index()],
            Target::Permanent(_) | Target::Card(_) | Target::Spell(_) => 0,
        };
        amount = self.apply_damage_limits(event, amount, already_assigned);
        if preventable
            && (self.static_damage_is_prevented(event)
                || self.protection_stops_damage(Some(target), assignment.source, source_is_spell))
        {
            amount = 0;
        }
        if amount == 0 {
            return None;
        }
        if let Target::Player(player) = target {
            assigned_to_players[player.index()] =
                assigned_to_players[player.index()].saturating_add(amount);
        }
        let source_has_keyword = |keyword: KeywordAbility| {
            source_object.as_ref().is_some_and(|source| {
                keyword
                    .simple_index()
                    .is_some_and(|index| source.keywords & (1 << index) != 0)
            })
        };
        let lifelink_controller = source_object.as_ref().and_then(|source| {
            source_has_keyword(KeywordAbility::Lifelink).then_some(source.controller)
        });
        let has_deathtouch = source_has_keyword(KeywordAbility::Deathtouch);
        let has_infect = source_has_keyword(KeywordAbility::Infect);

        Some(PreparedDamage {
            source: assignment.source,
            source_properties: PreparedDamageSource {
                object: source_object,
                is_spell: source_is_spell,
                has_deathtouch,
                has_infect,
                lifelink_controller,
            },
            target,
            recipient_object,
            amount,
            combat: assignment.combat,
        })
    }

    fn damage_recipient_outcomes(
        &self,
        prepared: &[PreparedDamage],
    ) -> Vec<DamageRecipientOutcome> {
        // CR 120.10 asks whether the recipient was dealt excess damage by
        // all sources in this event together. Keep this aggregate separate
        // from the source-specific assignment outcomes required by CR 120.9.
        let mut recipients = Vec::<DamageRecipientOutcome>::new();
        for damage in prepared {
            if let Some(outcome) = recipients
                .iter_mut()
                .find(|outcome| outcome.recipient == damage.target)
            {
                outcome.amount = outcome.amount.saturating_add(damage.amount);
            } else {
                recipients.push(DamageRecipientOutcome {
                    recipient: damage.target,
                    amount: damage.amount,
                    excess: 0,
                });
            }
        }
        for outcome in &mut recipients {
            let Target::Permanent(id) = outcome.recipient else {
                continue;
            };
            let damages_creature = prepared.iter().any(|damage| {
                damage.target == outcome.recipient
                    && damage
                        .recipient_object
                        .as_ref()
                        .is_some_and(|object| object.types.contains(CardType::Creature))
            });
            if !damages_creature {
                continue;
            }
            let has_deathtouch = prepared.iter().any(|damage| {
                damage.target == outcome.recipient && damage.source_properties.has_deathtouch
            });
            let lethal = if has_deathtouch {
                self.lethal_damage(id).min(1)
            } else {
                self.lethal_damage(id)
            };
            outcome.excess = outcome.amount.saturating_sub(lethal);
        }
        recipients
    }

    fn commit_prepared_damage(
        &mut self,
        prepared: &[PreparedDamage],
        mut deferred_life_gains: Vec<(PlayerId, u16)>,
    ) {
        // Only now do the results change life totals, counters, and marks.
        for damage in prepared {
            match damage.target {
                Target::Player(player) if damage.source_properties.has_infect => {
                    self.add_player_counters(player, CounterKind::Poison, damage.amount);
                }
                Target::Player(player) => {
                    self.deal_damage_to_player(player, damage.amount, damage.source, damage.combat);
                }
                Target::Permanent(id)
                    if damage.source_properties.has_infect
                        && damage
                            .recipient_object
                            .as_ref()
                            .is_some_and(|object| object.types.contains(CardType::Creature)) =>
                {
                    self.deal_infect_damage_to_creature(id, damage.amount, damage.source);
                }
                Target::Permanent(id) => {
                    self.deal_damage_to_permanent(
                        id,
                        damage.amount,
                        damage.source,
                        damage.source_properties.has_deathtouch,
                    );
                }
                Target::Card(_) | Target::Spell(_) => unreachable!("filtered before commit"),
            }
            if damage.combat {
                self.record_combat_damage_to_player(
                    Some(damage.target),
                    damage.source_properties.object.as_ref(),
                );
            }
        }

        deferred_life_gains.extend(prepared.iter().filter_map(|damage| {
            damage
                .source_properties
                .lifelink_controller
                .map(|controller| (controller, damage.amount))
        }));
        for (player, amount) in deferred_life_gains {
            self.gain_life(player, amount);
        }

        // Publish only after the full event is committed, so every trigger
        // observes one post-damage state rather than an intermediate state.
        for damage in prepared {
            self.capture_battlefield_triggers(&CommittedTriggerEvent::DamageDealt {
                source: damage.source_properties.object.clone(),
                source_is_spell: damage.source_properties.is_spell,
                recipient: damage.target,
                recipient_object: damage.recipient_object.clone(),
                amount: damage.amount,
                combat: damage.combat,
            });
        }
    }
}
