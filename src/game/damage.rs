use super::{
    AppliedEffectDef, CardType, CommittedTriggerEvent, ControlFlow, CounterKind, Game,
    GameObjectId, KeywordAbility, Permanent, PlayerId, PreventionShield, RetiredObject,
    ShieldCoverageDef, Target, TriggerEventObject,
};

impl Game {
    pub(super) fn damage_target(&mut self, target: Option<Target>, amount: u16) -> u16 {
        self.damage_target_from(None, target, amount)
    }

    /// Whether a static prevention on this permanent stops damage from this
    /// particular source. The source has to be a permanent, which is what
    /// "damage from artifact creatures" is about; damage from a spell is
    /// never prevented this way.
    pub(super) fn damage_is_prevented_from(
        &self,
        permanent: &Permanent,
        source: Option<GameObjectId>,
    ) -> bool {
        let Some(source) = source.and_then(|source| {
            self.battlefield
                .iter()
                .find(|candidate| candidate.card.id == source)
        }) else {
            return false;
        };
        let subject = self.trigger_event_object(source);
        self.visit_static_applied_effects(permanent, |applied| {
            if matches!(applied.effect, AppliedEffectDef::PreventDamageFrom(predicate)
                if self.trigger_object_matches(predicate, &subject, permanent.card.id, false))
            {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        })
        .is_break()
    }

    /// Applies any prevention shields covering this recipient, returning what
    /// is left to deal. `None` means all of it was prevented, so no damage
    /// event happens at all -- prevented damage was never dealt, so nothing
    /// that watches for damage should see it.
    fn spend_prevention_shields(
        &mut self,
        target: Option<Target>,
        amount: u16,
        source: Option<GameObjectId>,
    ) -> Option<u16> {
        let Some(target) = target else {
            return Some(amount);
        };
        // A shield naming a source ignores damage from anything else; a
        // shield naming none answers whatever arrives.
        let answers = |shield: &PreventionShield| {
            shield.recipient == target && shield.source.is_none_or(|named| Some(named) == source)
        };
        if amount == 0 || !self.prevention_shields.iter().any(answers) {
            return (amount > 0).then_some(amount);
        }
        let mut left = amount;
        let mut spent_named = Vec::new();
        let mut gained_life = Vec::new();
        for shield in &mut self.prevention_shields {
            if !answers(shield) || left == 0 {
                continue;
            }
            let Some(remaining) = shield.remaining.as_mut() else {
                // "Prevent all damage" is never spent; it simply holds. A
                // shield naming a source instead covers this one damage --
                // all of it, or the part its coverage names -- and is gone.
                shield.source?;
                let prevented = match shield.coverage {
                    ShieldCoverageDef::All => left,
                    ShieldCoverageDef::HalfRoundedDown => left / 2,
                };
                if shield.gain_life {
                    gained_life.push((target, prevented));
                }
                spent_named.push(shield.source);
                left -= prevented;
                break;
            };
            let spent = (*remaining).min(left);
            *remaining -= spent;
            left -= spent;
        }
        self.prevention_shields.retain(|shield| {
            shield.remaining != Some(0)
                && !(shield.source.is_some() && spent_named.contains(&shield.source))
        });
        // "You gain life equal to the damage prevented this way" reads the
        // amount actually stopped, so it is paid after the arithmetic above.
        for (recipient, prevented) in gained_life {
            if let Target::Player(player) = recipient
                && prevented > 0
            {
                self.gain_life(player, prevented);
            }
        }
        (left > 0).then_some(left)
    }

    fn relational_damage_is_prevented(
        &self,
        source: Option<GameObjectId>,
        target: Option<Target>,
        combat: bool,
    ) -> bool {
        self.relational_damage_preventions
            .iter()
            .any(|effect| match effect {
                super::RelationalDamagePrevention::ToPlayerAndControlledCreatures(player) => {
                    match target {
                        Some(Target::Player(recipient)) => recipient == *player,
                        Some(Target::Permanent(id)) => self
                            .battlefield
                            .iter()
                            .find(|permanent| permanent.card.id == id)
                            .is_some_and(|permanent| {
                                permanent.controller == *player
                                    && self
                                        .permanent_types(permanent)
                                        .is_some_and(|types| types.contains(CardType::Creature))
                            }),
                        Some(Target::Card(_) | Target::Spell(_)) | None => false,
                    }
                }
                super::RelationalDamagePrevention::FromAllExcept(exception) => {
                    combat && source != Some(*exception)
                }
            })
    }

    pub(super) fn damage_target_from(
        &mut self,
        source: Option<GameObjectId>,
        target: Option<Target>,
        amount: u16,
    ) -> u16 {
        self.damage_target_from_kind(source, target, amount, false)
    }

    pub(super) fn damage_target_from_kind(
        &mut self,
        source: Option<GameObjectId>,
        target: Option<Target>,
        amount: u16,
        combat: bool,
    ) -> u16 {
        let Some(amount) = self.spend_prevention_shields(target, amount, source) else {
            return 0;
        };
        let source_colors = source.map_or([false; 5], |source| self.object_colors(source));
        if self.relational_damage_is_prevented(source, target, combat)
            || target.is_some_and(|target| match target {
                Target::Permanent(id) => self
                    .battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == id)
                    .is_some_and(|permanent| {
                        self.is_protected_from_colors(permanent, source_colors)
                            || self.damage_is_prevented_from(permanent, source)
                    }),
                Target::Player(_) | Target::Card(_) | Target::Spell(_) => false,
            })
        {
            return 0;
        }
        let lifelink_controller = source.and_then(|source| {
            self.source_controller_with_keyword(source, KeywordAbility::Lifelink)
        });
        let has_deathtouch = source.is_some_and(|source| {
            self.source_controller_with_keyword(source, KeywordAbility::Deathtouch)
                .is_some()
        });
        let dealt_damage = match target {
            Some(Target::Player(player)) => {
                self.deal_damage(player, amount);
                if amount > 0
                    && let Some(damager) = source.and_then(|source| {
                        self.battlefield
                            .iter_mut()
                            .find(|permanent| permanent.card.id == source)
                    })
                    && damager.controller != player
                {
                    damager.dealt_damage_to_opponent_this_turn = true;
                }
                self.publish_damage_to_player(source, player, amount);
                true
            }
            Some(Target::Permanent(id)) => {
                if let Some(index) = self
                    .battlefield
                    .iter()
                    .position(|permanent| permanent.card.id == id)
                {
                    if self
                        .permanent_types(&self.battlefield[index])
                        .is_some_and(|types| types.contains(CardType::Planeswalker))
                    {
                        let remaining = self.battlefield[index]
                            .counters(CounterKind::Loyalty)
                            .saturating_sub(amount);
                        self.battlefield[index].set_counters(CounterKind::Loyalty, remaining);
                        true
                    } else {
                        let permanent = &mut self.battlefield[index];
                        permanent.damage = permanent.damage.saturating_add(amount);
                        if amount > 0 {
                            permanent.deathtouch_damage |= has_deathtouch;
                            if let Some(source) = source
                                && !permanent.damage_sources.contains(&source)
                            {
                                permanent.damage_sources.push(source);
                            }
                        }
                        true
                    }
                } else {
                    false
                }
            }
            Some(Target::Card(_) | Target::Spell(_)) | None => false,
        };
        if dealt_damage
            && amount > 0
            && let Some(controller) = lifelink_controller
        {
            self.gain_life(controller, amount);
        }
        if dealt_damage
            && amount > 0
            && let Some(source) = source
            && let Some(recipient) = target
            && let Some(source) = self.damage_source_event_object(source)
        {
            let event = CommittedTriggerEvent::DamageDealt {
                source,
                recipient,
                amount,
                combat,
            };
            self.capture_battlefield_triggers(&event);
        }
        if dealt_damage { amount } else { 0 }
    }

    pub(super) fn damage_source_event_object(
        &self,
        source: GameObjectId,
    ) -> Option<TriggerEventObject> {
        if let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
        {
            return Some(self.trigger_event_object(permanent));
        }
        if let Some(object) = self.stack.iter().find(|object| object.id == source) {
            return self.stack_trigger_event_object(object);
        }
        match self.retired_objects.get(&source) {
            Some(RetiredObject::Permanent { permanent, .. }) => {
                Some(self.trigger_event_object(permanent))
            }
            Some(RetiredObject::Stack(object)) => self.stack_trigger_event_object(object),
            Some(RetiredObject::Card(_)) | None => None,
        }
    }

    pub(super) fn damage_targets(&self) -> Vec<Target> {
        let mut targets = vec![Target::Player(PlayerId::One), Target::Player(PlayerId::Two)];
        targets.extend(
            self.battlefield
                .iter()
                .filter(|permanent| {
                    self.power(permanent).is_some()
                        || self
                            .permanent_types(permanent)
                            .is_some_and(|types| types.contains(CardType::Planeswalker))
                })
                .map(|permanent| Target::Permanent(permanent.card.id)),
        );
        targets
    }
}
