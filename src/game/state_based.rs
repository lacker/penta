use super::attachments::AttachmentKind;
use super::{
    AttachmentForm, CardSupertype, CardType, CounterKind, Game, GameObjectId, GameResult, PlayerId,
    TriggerEventDef, WinReason,
};

impl Game {
    pub(super) fn check_state_based_actions(&mut self) {
        self.close_stale_miracle_window();
        self.end_expired_control_changes();
        if self.check_player_loss_conditions() {
            return;
        }
        loop {
            self.reconcile_all_attachment_control();
            let battlefield_len = self.battlefield.len();
            let mut regenerate = Vec::new();
            let mut die = Vec::new();
            let mut detach = Vec::new();
            for permanent in &self.battlefield {
                let attachment_kind = self.attachment_kind(permanent);
                let attached_wrong_kind = permanent.attached_to.is_some()
                    && (attachment_kind.is_none()
                        || self
                            .permanent_types(permanent)
                            .is_some_and(|types| types.contains(CardType::Creature)));
                if attached_wrong_kind {
                    // 704.5p: a creature, or any other permanent that is not
                    // an Aura, Equipment, or Fortification, cannot stay
                    // attached. Reconfigure's type change has already removed
                    // Creature before this check.
                    detach.push(permanent.card.id);
                } else {
                    match attachment_kind {
                        Some(AttachmentKind::Aura)
                            if permanent.attached_to.is_none_or(|host| {
                                !self.is_legal_attachment_host(permanent, host, false)
                            }) =>
                        {
                            if matches!(
                                permanent.attachment_form,
                                Some(AttachmentForm::Bestowed { .. })
                            ) {
                                detach.push(permanent.card.id);
                            } else {
                                // 704.5m: an Aura attached to nothing, or to
                                // an illegal host, goes to its owner's graveyard.
                                die.push(permanent.card.id);
                                continue;
                            }
                        }
                        Some(AttachmentKind::Equipment | AttachmentKind::Fortification)
                            if permanent.attached_to.is_some_and(|host| {
                                !self.is_legal_attachment_host(permanent, host, false)
                            }) =>
                        {
                            // 704.5n: Equipment and Fortifications merely
                            // become unattached when their host is illegal.
                            detach.push(permanent.card.id);
                        }
                        _ => {}
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
                if permanent.regeneration_shields > 0 && !permanent.cannot_regenerate_this_turn {
                    regenerate.push(permanent.card.id);
                } else {
                    die.push(permanent.card.id);
                }
            }
            for id in regenerate {
                self.regenerate_permanent(id);
            }
            let detached = !detach.is_empty();
            for id in detach {
                self.unattach(id);
            }
            self.move_permanents_to_graveyard(&die);
            if !self.pending_decisions.is_empty()
                || !self.pending_events.is_empty()
                || !self.pending_procedures.is_empty()
            {
                return;
            }
            self.apply_legend_rule();
            if self.battlefield.len() == battlefield_len && !detached {
                break;
            }
        }
        self.capture_state_triggers();
    }

    /// CR 704.5a-b: zero life and trying to draw from an empty library are
    /// state-based loss conditions. Read both players and both conditions in
    /// one pass so simultaneous losses end the two-player game in a draw.
    fn check_player_loss_conditions(&mut self) -> bool {
        let tried_to_draw_from_empty = self
            .players
            .each_mut()
            .map(|player| std::mem::take(&mut player.tried_to_draw_from_empty_library));
        let lost = [
            self.players[0].life <= 0 || tried_to_draw_from_empty[0],
            self.players[1].life <= 0 || tried_to_draw_from_empty[1],
        ];

        let result = match lost {
            [true, true] => Some(GameResult::Draw),
            [true, false] => Some(GameResult::Winner {
                winner: PlayerId::Two,
                reason: if tried_to_draw_from_empty[0] {
                    WinReason::OpponentTriedToDrawFromEmptyLibrary
                } else {
                    WinReason::OpponentLostAllLife
                },
            }),
            [false, true] => Some(GameResult::Winner {
                winner: PlayerId::One,
                reason: if tried_to_draw_from_empty[1] {
                    WinReason::OpponentTriedToDrawFromEmptyLibrary
                } else {
                    WinReason::OpponentLostAllLife
                },
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

    /// The legend rule as a state-based action: a player controlling two or
    /// more legendary permanents with the same name keeps one and puts the
    /// rest into the graveyard. The rules let the controller choose; with
    /// identical names the copies differ only in tap and damage state, so the
    /// strictly best one — untapped over tapped, then newest — is kept
    /// without asking.
    pub(super) fn apply_legend_rule(&mut self) {
        loop {
            let mut extra: Option<GameObjectId> = None;
            'search: for permanent in &self.battlefield {
                if !self
                    .effective_rules(permanent)
                    .is_some_and(|rules| rules.has_supertype(CardSupertype::Legendary))
                {
                    continue;
                }
                let name_source = Self::effective_rules_source(permanent);
                for other in &self.battlefield {
                    if other.card.id == permanent.card.id
                        || other.controller != permanent.controller
                        || Self::effective_rules_source(other) != name_source
                    {
                        continue;
                    }
                    let permanent_wins = (!permanent.tapped && other.tapped)
                        || (permanent.tapped == other.tapped
                            && permanent.card.id.0 > other.card.id.0);
                    extra = Some(if permanent_wins {
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
    /// "For as long as you control this creature" ends when that stops being
    /// true: the holder leaving the battlefield, or passing to someone else,
    /// both return the stolen permanent to whoever had it before.
    fn end_expired_control_changes(&mut self) {
        let sources = self
            .battlefield
            .iter()
            .map(|permanent| (permanent.card.id, permanent.controller, permanent.tapped))
            .collect::<Vec<_>>();
        let mut changed = false;
        for permanent in &mut self.battlefield {
            let before = permanent.control_while_source_remains.len();
            permanent.control_while_source_remains.retain(|effect| {
                sources
                    .iter()
                    .find(|(id, _, _)| *id == effect.source)
                    .is_some_and(|(_, controller, tapped)| {
                        *controller == effect.controller
                            && (!effect.requires_source_tapped || *tapped)
                    })
            });
            changed |= permanent.control_while_source_remains.len() != before;
        }
        if changed {
            self.reconcile_all_control_layers();
        }
    }
}
