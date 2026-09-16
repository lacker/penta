// Committing an arrival after its entry replacements finish.
impl Game {
    #[allow(clippy::too_many_lines)]
    pub(super) fn commit_battlefield_entry(&mut self, mut entry: PendingBattlefieldEntry) {
        if let Some(zone) = entry.redirected_to {
            self.commit_redirected_entry(entry, zone);
            return;
        }
        let prospective = entry.permanent.card.id;
        if entry.completion != EntryCompletion::Setup
            && let Some(card) = entry.permanent.card.clone().into_card()
        {
            let (card, _zone_change) = self.zone_change_card(card);
            entry.permanent.card = card.into();
        }
        // A permanent takes a fresh identity as it actually arrives, so
        // anything linked to it while the entry was still prospective has to
        // be re-pointed at the object that ended up on the battlefield.
        if prospective != entry.permanent.card.id {
            let arrived = entry.permanent.card.id;
            for (source, _) in &mut self.linked_exiles {
                if *source == prospective {
                    *source = arrived;
                }
            }
        }
        entry.permanent.timestamp = self.allocate_continuous_effect_timestamp();
        let permanent_id = entry.permanent.card.id;
        let face_down = entry.permanent.face_down.is_some();
        let definition = entry.permanent.card.definition.card_definition();
        let choose_defender = entry.permanent.attacking;
        let arriving_controller = entry.permanent.controller;
        self.battlefield.push(entry.permanent);
        self.create_prepared_spell(permanent_id);
        self.grant_enduring_stories();
        let mut arriving_attacker = None;
        if choose_defender {
            let is_creature = self.battlefield.last().is_some_and(|permanent| {
                self.permanent_types(permanent)
                    .is_some_and(|types| types.contains(crate::card::CardType::Creature))
            });
            self.battlefield.last_mut().expect("just entered").attacking = is_creature;
            if is_creature {
                arriving_attacker = Some(crate::AttackDefender::Player(
                    arriving_controller.opponent(),
                ));
                self.queue_arriving_attacker_defender(
                    arriving_controller,
                    arriving_controller.opponent(),
                    &[permanent_id],
                );
            }
        }

        if let EntryCompletion::AttachSource { source } = entry.completion {
            self.try_attach(source, permanent_id);
        }

        // The other direction: the Equipment is what arrived, and the host
        // was here all along.
        if let EntryCompletion::AttachToHost { host } = entry.completion {
            self.try_attach(permanent_id, host);
        }

        let attacking = match entry.completion {
            EntryCompletion::Attacking { defender } => Some(defender),
            EntryCompletion::SpellResolved { .. } if self.step.is_combat() => self
                .battlefield
                .iter()
                .find(|p| p.card.id == permanent_id && p.controller == self.active_player)
                .and_then(|p| p.cast.as_ref())
                .and_then(|cast| cast.sneak_defender),
            _ => None,
        };
        let attacking =
            arriving_attacker.or(attacking).filter(|defender| self.arriving_creature_can_attack(permanent_id, *defender));
        if let Some(defender) = attacking
            && let Some(permanent) = self
                .battlefield
                .iter_mut()
                .find(|permanent| permanent.card.id == permanent_id)
        {
            // It was never declared, so it does not count as having been
            // declared -- but everything else about it is an attacker.
            permanent.attacking = true;
            self.combat_had_attackers = true;
            permanent.attack_defender = Some(defender);
            permanent.attacked_this_turn = true;
            permanent.attacks_this_turn = permanent.attacks_this_turn.saturating_add(1);
        }

        if let EntryCompletion::LandPlayed { player } = entry.completion {
            self.events.push(GameEvent::LandPlayed {
                player,
                card: permanent_id,
                definition: definition.expect("a played land is a card"),
            });
        }

        let entered = self
            .battlefield
            .last()
            .expect("a committed battlefield entry is present");
        // What a trigger asks of an arriving permanent is what it is as it
        // arrives, static effects and all: "consider static abilities to
        // determine whether its power and toughness are both 1" is Sword of
        // the Meek's ruling, and a Crusade already on the battlefield is one
        // of them. The entry is committed by the time this is read, so the
        // widened view is safe here where it is not inside the layer walk.
        let entered_event = self.targeting_event_object(entered);
        let before_event = if prospective == permanent_id {
            None
        } else {
            match self.retired_objects.get(&prospective) {
                Some(RetiredObject::Stack(stack)) => self.stack_object_event_object(stack),
                Some(RetiredObject::Card(card)) => {
                    let context = match entry.from {
                        ZoneKind::Library => Some(CharacteristicContext::Library),
                        ZoneKind::Hand => Some(CharacteristicContext::Hand),
                        ZoneKind::Graveyard => Some(CharacteristicContext::Graveyard),
                        ZoneKind::Exile => Some(CharacteristicContext::Exile),
                        // A stack predecessor is represented by the arm above;
                        // the remaining zones do not hold cards that enter.
                        ZoneKind::Battlefield | ZoneKind::Stack | ZoneKind::Command => None,
                    };
                    context.and_then(|context| {
                        self.printed_trigger_event_object(
                            card.id,
                            card.definition,
                            card.owner,
                            &context,
                        )
                    })
                }
                Some(RetiredObject::Permanent { permanent, .. }) => {
                    Some(self.trigger_event_object(permanent))
                }
                None => None,
            }
        };
        // Raised before the entry below, since the play is what caused it:
        // a clause about playing a land reads the land that was played.
        if let EntryCompletion::LandPlayed { player } = entry.completion {
            self.capture_battlefield_triggers(&CommittedTriggerEvent::LandPlayed {
                player,
                object: entered_event.clone(),
            });
        }
        self.capture_entry_event(CommittedTriggerEvent::ZoneChanged {
            before: before_event,
            after: Some(entered_event),
            from: entry.from,
            to: ZoneKind::Battlefield,
            damage_sources: Vec::new(),
        });
        // A delayed trigger in a permanent spell's selected alternative
        // clause is installed by resolution, not by an enters ability. Use
        // the frozen spell payload, even if entry changes its abilities.
        if let EntryCompletion::SpellResolved { card, .. } = entry.completion {
            self.install_permanent_spell_resolution_trigger(card, permanent_id);
        }
        self.capture_room_entry_unlock(permanent_id);
        self.place_entry_lore_counter(permanent_id);
        if self.pregame.is_none() && self.restart_arrivals.is_none() {
            self.apply_legend_rule();
        }

        if let EntryCompletion::SpellResolved { card, definition } = entry.completion {
            self.events
                .push(GameEvent::spell_resolved(card, definition, face_down));
        }
    }
}
