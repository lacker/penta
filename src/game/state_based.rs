use super::{
    AppliedRuleDef, CardSupertype, CardType, CounterKind, Game, GameObjectId, GameResult,
    ObjectKind, Permanent, PlayerId, TriggerEventDef, WinReason,
};

/// CR 704.5c. Ten is a rules constant, not a format setting: no supported
/// format changes it.
const LETHAL_POISON: u16 = 10;

/// CR 702.131b. Ten, like the poison total above, is a rules constant.
const ASCEND_THRESHOLD: usize = 10;

impl Game {
    /// Ascend (CR 702.131b): a player controlling a permanent with ascend and
    /// ten or more permanents gets the city's blessing. It is checked here
    /// rather than raised as a trigger because nothing may be done about it
    /// in between -- and once given it is never taken back, so dropping to
    /// nine afterwards changes nothing.
    fn grant_the_citys_blessing(&mut self) {
        for player in [PlayerId::One, PlayerId::Two] {
            if self.citys_blessing[player.index()] {
                continue;
            }
            let permanents = self
                .battlefield
                .iter()
                .filter(|permanent| permanent.controller == player)
                .count();
            if permanents >= ASCEND_THRESHOLD
                && self.player_rule_applies(player, AppliedRuleDef::Ascend)
            {
                self.citys_blessing[player.index()] = true;
            }
        }
    }

    pub(super) fn check_state_based_actions(&mut self) {
        self.end_expired_control_changes();
        self.reconcile_static_control_changes();
        if self.check_player_loss_conditions() {
            return;
        }
        self.grant_the_citys_blessing();
        self.annihilate_opposing_counters();
        self.unattach_illegal_non_aura_attachments();
        loop {
            self.unbestow_permanents_without_a_host();
            let battlefield_len = self.battlefield.len();
            let mut regenerate = Vec::new();
            let mut die = Vec::new();
            // The sweep below only reads, and asks the same land-type
            // questions of every permanent on the battlefield. It is dropped
            // by hand before the first mutation, because an answer must not
            // outlive the board it was given for.
            let land_types = self.hold_land_type_query_memo();
            for permanent in &self.battlefield {
                // 704.5m: an Aura attached to nothing, or to something that is
                // no longer a legal host, is put into its owner's graveyard.
                // 704.5p does the milder thing for Equipment: it comes loose
                // and stays where it is.
                if self.is_aura_permanent(permanent) {
                    let legal_attachment = match (permanent.attached_to, permanent.attached_player)
                    {
                        (Some(host), None) => self.is_legal_aura_host(permanent, host),
                        (None, Some(player)) => self.is_legal_aura_player(permanent, player),
                        (None, None) | (Some(_), Some(_)) => false,
                    };
                    if !legal_attachment {
                        die.push(permanent.card.id);
                        continue;
                    }
                }
                if self
                    .permanent_types(permanent)
                    .is_some_and(|types| types.contains(CardType::Planeswalker))
                    && permanent.counters(CounterKind::Loyalty) == 0
                {
                    die.push(permanent.card.id);
                    continue;
                }
                let Some(toughness) = self.toughness(permanent) else {
                    continue;
                };
                let zero_toughness = toughness <= 0;
                let lethal_damage = i32::from(permanent.damage) >= i32::from(toughness)
                    || (permanent.damage > 0 && permanent.deathtouch_damage);
                if zero_toughness {
                    die.push(permanent.card.id);
                    continue;
                }
                if !lethal_damage || self.has_indestructible(permanent) {
                    continue;
                }
                if permanent.regeneration_shields > 0
                    && !self.has_applied_rule(permanent, AppliedRuleDef::CannotRegenerate)
                {
                    regenerate.push(permanent.card.id);
                } else {
                    die.push(permanent.card.id);
                }
            }
            drop(land_types);
            for id in regenerate {
                self.regenerate_permanent(id);
            }
            self.move_permanents_to_graveyard(&die);
            // 704.5p again: the host that just died is a host no longer, and
            // the Equipment on it comes loose in this same check rather than
            // waiting for the next one. Auras with dead hosts are handled at
            // the top of this loop; without this an Equipment stays pointed
            // at a permanent that is not there any more for as long as it
            // takes a player to get priority again.
            self.unattach_illegal_non_aura_attachments();
            if !self.pending_decisions.is_empty()
                || !self.pending_events.is_empty()
                || !self.pending_procedures.is_empty()
            {
                return;
            }
            self.apply_legend_rule();
            self.apply_world_rule();
            self.apply_role_rule();
            self.sacrifice_completed_sagas();
            if self.battlefield.len() == battlefield_len {
                break;
            }
        }
        self.break_illegal_pairings();
        self.capture_state_triggers();
    }

    /// CR 704.5p-r: an Equipment on a noncreature, a Fortification on a
    /// nonland, a creature attached to anything, or an attached permanent
    /// that is no longer an Aura, Equipment, or Fortification becomes
    /// unattached. Unlike an Aura with an illegal host these stay on the
    /// battlefield, so this is deliberately separate from the Aura move to
    /// the graveyard below.
    /// CR 702.103c: a bestowed permanent whose enchanted creature is gone
    /// does not go with it. It comes unattached and becomes a creature
    /// instead, which is why this runs inside the loop and ahead of the Aura
    /// rule below: by the time that rule reads it, it is no longer an Aura.
    fn unbestow_permanents_without_a_host(&mut self) {
        let loose = self
            .battlefield
            .iter()
            .filter(|permanent| {
                Self::is_bestowed_aura(permanent)
                    && permanent
                        .attached_to
                        .is_none_or(|host| !self.is_legal_aura_host(permanent, host))
            })
            .map(|permanent| permanent.card.id)
            .collect::<Vec<_>>();
        for id in loose {
            self.unattach(id);
        }
    }

    fn unattach_illegal_non_aura_attachments(&mut self) {
        let loose = self
            .battlefield
            .iter()
            .filter(|permanent| {
                let Some(host) = permanent.attached_to else {
                    return false;
                };
                if self
                    .permanent_types(permanent)
                    .is_some_and(|types| types.contains(CardType::Creature))
                {
                    return true;
                }
                match self.attachment_kind(permanent) {
                    Some(super::attachments::AttachmentKind::Aura) => false,
                    Some(
                        super::attachments::AttachmentKind::Equipment
                        | super::attachments::AttachmentKind::Fortification,
                    ) => !self.is_legal_attachment_host(permanent, host),
                    None => true,
                }
            })
            .map(|permanent| permanent.card.id)
            .collect::<Vec<_>>();
        for id in loose {
            self.unattach(id);
        }
    }

    /// CR 122.3: a permanent with both +1/+1 and -1/-1 counters loses an
    /// equal number of each, so it never carries both at once.
    fn annihilate_opposing_counters(&mut self) {
        for permanent in &mut self.battlefield {
            let paired = permanent
                .counters(CounterKind::PlusOnePlusOne)
                .min(permanent.counters(CounterKind::MinusOneMinusOne));
            if paired > 0 {
                permanent.remove_counters(CounterKind::PlusOnePlusOne, paired);
                permanent.remove_counters(CounterKind::MinusOneMinusOne, paired);
            }
        }
    }

    /// CR 704.5a-b: zero life and trying to draw from an empty library are
    /// state-based loss conditions. Read both players and both conditions in
    /// one pass so simultaneous losses end the two-player game in a draw.
    fn check_player_loss_conditions(&mut self) -> bool {
        let tried_to_draw_from_empty = self
            .players
            .each_mut()
            .map(|player| std::mem::take(&mut player.tried_to_draw_from_empty_library));
        let poisoned = [
            self.players[0].counters.count(CounterKind::Poison) >= LETHAL_POISON,
            self.players[1].counters.count(CounterKind::Poison) >= LETHAL_POISON,
        ];
        let lost = [
            self.players[0].life <= 0 || tried_to_draw_from_empty[0] || poisoned[0],
            self.players[1].life <= 0 || tried_to_draw_from_empty[1] || poisoned[1],
        ];
        // Life is checked first because it is the ordinary case; poison is
        // last because a seat that is dead twice over is still just dead.
        let reason = |loser: PlayerId| {
            if tried_to_draw_from_empty[loser.index()] {
                WinReason::OpponentTriedToDrawFromEmptyLibrary
            } else if self.players[loser.index()].life <= 0 {
                WinReason::OpponentLostAllLife
            } else {
                WinReason::OpponentPoisoned
            }
        };

        let result = match lost {
            [true, true] => Some(GameResult::Draw),
            [true, false] => Some(GameResult::Winner {
                winner: PlayerId::Two,
                reason: reason(PlayerId::One),
            }),
            [false, true] => Some(GameResult::Winner {
                winner: PlayerId::One,
                reason: reason(PlayerId::Two),
            }),
            [false, false] => None,
        };
        if let Some(result) = result {
            self.finish(result);
            true
        } else {
            false
        }
    }

    /// CR 603.8: a state trigger triggers whenever its condition is true, and
    /// does not trigger again while it is already waiting or on the stack.
    /// State-based actions are checked whenever anything could have changed,
    /// which is exactly when such a condition could have become true.
    pub(super) fn capture_state_triggers(&mut self) {
        let listeners = self
            .battlefield_trigger_listeners()
            .into_iter()
            .filter(|listener| {
                listener.uses_stack && listener.event == TriggerEventDef::StateCondition
            })
            .filter(|listener| {
                let source = listener.capture.source;
                let waiting = self
                    .pending_triggers
                    .iter()
                    .any(|pending| pending.source == source);
                let on_stack = self.stack.iter().any(|object| {
                    object.source == Some(source.object)
                        && object
                            .ability
                            .as_ref()
                            .is_some_and(|ability| ability.origin == source.ability)
                });
                !waiting && !on_stack
            })
            .collect::<Vec<_>>();
        for listener in listeners {
            self.capture_trigger(&listener.capture);
        }
    }

    /// CR 704.5s: a player controlling two or more Role tokens attached to
    /// the same permanent keeps the newest and puts the rest into the
    /// graveyard. Unlike the legend rule this asks nobody -- the rules name
    /// which one stays -- and it is per host rather than per name, so the
    /// same player may hold Roles on several creatures at once.
    pub(super) fn apply_role_rule(&mut self) {
        loop {
            let mut extra: Option<GameObjectId> = None;
            'search: for permanent in &self.battlefield {
                let Some(host) = permanent.attached_to else {
                    continue;
                };
                if !self.is_role_token(permanent) {
                    continue;
                }
                for other in &self.battlefield {
                    if other.card.id == permanent.card.id
                        || other.controller != permanent.controller
                        || other.attached_to != Some(host)
                        || !self.is_role_token(other)
                    {
                        continue;
                    }
                    // The newest stays, and a token made later has the
                    // larger identity.
                    extra = Some(if permanent.card.id.0 > other.card.id.0 {
                        other.card.id
                    } else {
                        permanent.card.id
                    });
                    break 'search;
                }
            }
            let Some(extra) = extra else {
                return;
            };
            self.move_permanents_to_graveyard(&[extra]);
            if !self.pending_decisions.is_empty() || !self.pending_events.is_empty() {
                return;
            }
        }
    }

    /// Whether this permanent is a Role token: the rule is about tokens
    /// specifically, so an ordinary Aura that somehow had the subtype would
    /// not be one.
    fn is_role_token(&self, permanent: &Permanent) -> bool {
        permanent.card.definition == ObjectKind::Token
            && self.effective_subtypes(permanent).contains(&"Role")
    }

    /// CR 704.5k: of two or more World permanents, only the one that has had
    /// the supertype for the shortest time stays. Authored World permanents
    /// have it for their entire battlefield lifetime, whose timestamp is
    /// already recorded independently from object identity. Equal timestamps
    /// mean equal durations, so the tie clause puts every World permanent into
    /// its owner's graveyard in one simultaneous zone-change batch.
    fn apply_world_rule(&mut self) {
        let worlds = self
            .battlefield
            .iter()
            .filter(|permanent| {
                self.permanent_supertypes(permanent)
                    .is_some_and(|supertypes| supertypes.contains(CardSupertype::World))
            })
            .map(|permanent| (permanent.card.id, permanent.timestamp))
            .collect::<Vec<_>>();
        if worlds.len() < 2 {
            return;
        }

        let newest = worlds
            .iter()
            .map(|(_, timestamp)| *timestamp)
            .max()
            .expect("two World permanents have a newest timestamp");
        let newest_count = worlds
            .iter()
            .filter(|(_, timestamp)| *timestamp == newest)
            .count();
        let doomed = worlds
            .into_iter()
            .filter_map(|(object, timestamp)| {
                (newest_count > 1 || timestamp != newest).then_some(object)
            })
            .collect::<Vec<_>>();
        self.move_permanents_to_graveyard(&doomed);
    }

    /// The legend rule as a state-based action: a player controlling two or
    /// more legendary permanents with the same name keeps one and puts the
    /// rest into the graveyard.
    ///
    /// The controller chooses which one stays (CR 704.5j), and the choice is
    /// a real one: counters, attachments, damage, and tapped status all
    /// separate two permanents of one name. A Thespian's Stage that has
    /// copied its controller's own Dark Depths is the case the rule is played
    /// for — the copy carries no ice counters and is worth keeping even
    /// though paying for the ability left it tapped. The options are ordered
    /// so the first is the strictly better body, untapped over tapped and
    /// then newest, which is what an automated policy takes by preference.
    pub(super) fn apply_legend_rule(&mut self) {
        let Some((controller, mut candidates)) = self.legend_rule_group() else {
            return;
        };
        candidates.sort_by(|left, right| {
            let body = |id: GameObjectId| {
                self.battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == id)
                    .map_or((true, 0), |permanent| {
                        (permanent.tapped, permanent.card.id.0)
                    })
            };
            let (left_tapped, left_id) = body(*left);
            let (right_tapped, right_id) = body(*right);
            left_tapped.cmp(&right_tapped).then(right_id.cmp(&left_id))
        });
        self.queue_legend_rule_choice(controller, &candidates);
    }

    /// One controller's same-named legendary permanents, when they have more
    /// than one of them. The first group found is enough: answering it runs
    /// state-based actions again, which finds the next.
    fn legend_rule_group(&self) -> Option<(PlayerId, Vec<GameObjectId>)> {
        for permanent in &self.battlefield {
            if self.legend_rule_does_not_apply_to(permanent)
                || !self
                    .permanent_supertypes(permanent)
                    .is_some_and(|supertypes| supertypes.contains(CardSupertype::Legendary))
            {
                continue;
            }
            let Some(name) = self.effective_permanent_name(permanent) else {
                continue;
            };
            let group = self
                .battlefield
                .iter()
                .filter(|other| {
                    other.controller == permanent.controller
                        && !self.legend_rule_does_not_apply_to(other)
                        && self
                            .permanent_supertypes(other)
                            .is_some_and(|supertypes| supertypes.contains(CardSupertype::Legendary))
                        && self.effective_permanent_name(other).as_deref() == Some(name.as_ref())
                })
                .map(|other| other.card.id)
                .collect::<Vec<_>>();
            if group.len() > 1 {
                return Some((permanent.controller, group));
            }
        }
        None
    }
    /// "For as long as you control this creature" ends when that stops being
    /// true: the holder leaving the battlefield, or passing to someone else,
    /// both return the stolen permanent to whoever had it before.
    fn end_expired_control_changes(&mut self) {
        let expired = self
            .battlefield
            .iter()
            .filter_map(|permanent| {
                let holder = permanent.control_source?;
                let held = self
                    .battlefield
                    .iter()
                    .find(|candidate| candidate.card.id == holder)
                    .is_some_and(|candidate| {
                        if permanent.control_requires_source_attached {
                            candidate.attached_to == Some(permanent.card.id)
                        } else {
                            candidate.controller == permanent.controller
                                && (!permanent.control_requires_source_tapped || candidate.tapped)
                        }
                    });
                (!held).then_some(permanent.card.id)
            })
            .collect::<Vec<_>>();
        for id in expired {
            let Some(permanent) = self
                .battlefield
                .iter_mut()
                .find(|permanent| permanent.card.id == id)
            else {
                continue;
            };
            permanent.control_source = None;
            permanent.control_requires_source_tapped = false;
            permanent.control_requires_source_attached = false;
            if let Some(owner) = permanent.control_reverts_to.take() {
                permanent.controller = owner;
                permanent.suspend_haste = false;
                permanent.entered_controller_turn = self.turns_started[owner.index()];
            }
        }
    }
}
