// Whether one committed event answers one printed trigger event. The whole
// question is one wide match, which is why it lives beside the capture logic
// rather than inside it; included textually into `trigger_capture.rs`, so the
// imports here are that module's.

impl Game {
    pub(super) fn zone_change_event_observation(
        definition: TriggerEventDef,
        event: &CommittedTriggerEvent,
    ) -> Option<ZoneChangeObservationDef> {
        let CommittedTriggerEvent::ZoneChanged {
            from,
            to,
            ..
        } = event
        else {
            return None;
        };
        let matcher = match definition {
            TriggerEventDef::ZoneChanged(matcher) => Some(matcher),
            TriggerEventDef::While { event: wrapped, .. } => {
                return Self::zone_change_event_observation(*wrapped, event);
            }
            TriggerEventDef::AnyOf(events) => events.iter().find_map(|candidate| {
                let TriggerEventDef::ZoneChanged(matcher) = candidate else {
                    return None;
                };
                (matcher.from.is_none_or(|expected| expected == *from)
                    && matcher.to.is_none_or(|expected| expected == *to))
                .then_some(*matcher)
            }),
            _ => None,
        }?;
        Some(matcher.observation)
    }

    fn zone_change_event_object(
        definition: TriggerEventDef,
        event: &CommittedTriggerEvent,
    ) -> Option<&TriggerEventObject> {
        let CommittedTriggerEvent::ZoneChanged { before, after, .. } = event else {
            return None;
        };
        match Self::zone_change_event_observation(definition, event)? {
            ZoneChangeObservationDef::Before => before.as_ref(),
            ZoneChangeObservationDef::After => after.as_ref(),
        }
    }

    fn stack_target_matches_for_controller(
        &self,
        filter: StackTargetFilterDef,
        target: Target,
        source: GameObjectId,
        controller: Option<PlayerId>,
        context: TriggerContext,
    ) -> bool {
        match filter {
            StackTargetFilterDef::Player(relation) => match target {
                Target::Player(player) => controller.is_some_and(|controller| {
                    self.player_relation_matches(player, relation, controller, context)
                }),
                Target::Permanent(_) | Target::Card(_) | Target::Spell(_) => false,
            },
            StackTargetFilterDef::Permanent(predicate) => match target {
                Target::Permanent(target) => self
                    .battlefield
                    .iter()
                    .find(|candidate| candidate.card.id == target)
                    .map(|candidate| self.trigger_event_object(candidate))
                    .is_some_and(|target| {
                        self.trigger_object_matches_for_controller(
                            predicate,
                            &target,
                            source,
                            false,
                            controller,
                        )
                    }),
                Target::Player(_) | Target::Card(_) | Target::Spell(_) => false,
            },
            StackTargetFilterDef::Card(predicate) => match target {
                Target::Card(target) => self
                    .card_in_nonbattlefield_zone(target)
                    .and_then(|(zone, card)| {
                        let context = match zone {
                            ZoneKind::Library => CharacteristicContext::Library,
                            ZoneKind::Hand => CharacteristicContext::Hand,
                            ZoneKind::Graveyard => CharacteristicContext::Graveyard,
                            ZoneKind::Exile => CharacteristicContext::Exile,
                            ZoneKind::Battlefield | ZoneKind::Stack | ZoneKind::Command => {
                                return None;
                            }
                        };
                        self.printed_trigger_event_object(
                            target,
                            card.definition,
                            card.owner,
                            &context,
                        )
                    })
                    .is_some_and(|target| {
                        self.trigger_object_matches_for_controller(
                            predicate,
                            &target,
                            source,
                            false,
                            controller,
                        )
                    }),
                Target::Player(_) | Target::Permanent(_) | Target::Spell(_) => false,
            },
            StackTargetFilterDef::Spell(predicate) => match target {
                Target::Spell(target) => self
                    .stack
                    .iter()
                    .find(|candidate| candidate.id == target)
                    .and_then(|candidate| self.stack_object_event_object(candidate))
                    .is_some_and(|target| {
                        self.trigger_object_matches_for_controller(
                            predicate,
                            &target,
                            source,
                            true,
                            controller,
                        )
                    }),
                Target::Player(_) | Target::Permanent(_) | Target::Card(_) => false,
            },
            StackTargetFilterDef::AnyOf(filters) => filters.iter().copied().any(|filter| {
                self.stack_target_matches_for_controller(
                    filter, target, source, controller, context,
                )
            }),
        }
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn trigger_event_matches_for_controller(
        &self,
        definition: TriggerEventDef,
        event: &CommittedTriggerEvent,
        source: GameObjectId,
        controller: Option<PlayerId>,
    ) -> bool {
        match (definition, event) {
            // One printed ability, several ways into the same matching path.
            (TriggerEventDef::AnyOf(events), _) => events.iter().any(|alternative| {
                self.trigger_event_matches_for_controller(*alternative, event, source, controller)
            }),
            // A printed "while ...". The condition belongs to the event, so
            // it is asked here, once, against the board as the event
            // happened -- and never again as the ability resolves.
            (
                TriggerEventDef::While {
                    event: wrapped,
                    condition,
                },
                _,
            ) => {
                self.trigger_event_matches_for_controller(*wrapped, event, source, controller)
                    && controller.is_some_and(|controller| {
                        self.trigger_condition_holds(
                            condition,
                            source,
                            controller,
                            event.context(),
                            None,
                            None,
                        )
                    })
            }
            (
                TriggerEventDef::ZoneChanged(matcher),
                CommittedTriggerEvent::ZoneChanged {
                    before,
                    after,
                    from: actual_from,
                    to: actual_to,
                    damage_sources,
                },
            ) => {
                matcher.from.is_none_or(|expected| expected == *actual_from)
                    && matcher.to.is_none_or(|expected| expected == *actual_to)
                    && matcher.previously_damaged_by.is_none_or(|reference| {
                        self.trigger_event_object_reference(reference, source, event)
                            .is_some_and(|source| damage_sources.contains(&source))
                    })
                    && match matcher.observation {
                        ZoneChangeObservationDef::Before => before.as_ref(),
                        ZoneChangeObservationDef::After => after.as_ref(),
                    }
                    .is_some_and(|object| {
                        self.trigger_object_matches_for_controller(
                            matcher.object,
                            object,
                            source,
                            false,
                            controller,
                        )
                    })
            }
            (
                TriggerEventDef::Tapped(matcher),
                CommittedTriggerEvent::Tapped { object, for_mana },
            ) => {
                (matcher.purpose == TapPurposeDef::Any || *for_mana)
                    && self.trigger_object_matches_for_controller(
                        matcher.object,
                        object,
                        source,
                        false,
                        controller,
                    )
            }
            (
                TriggerEventDef::CountersPlaced {
                    object: predicate,
                    kind,
                },
                CommittedTriggerEvent::CountersPlaced {
                    object,
                    kind: placed,
                    ..
                },
            ) => {
                kind == *placed
                    && self.trigger_object_matches_for_controller(
                        predicate, object, source, false, controller,
                    )
            }
            (
                TriggerEventDef::CountersRemoved {
                    object: predicate,
                    kind,
                },
                CommittedTriggerEvent::CountersRemoved {
                    object,
                    kind: removed,
                    ..
                },
            ) => {
                kind == *removed
                    && self.trigger_object_matches_for_controller(
                        predicate, object, source, false, controller,
                    )
            }
            (
                TriggerEventDef::LastCounterRemoved {
                    object: predicate,
                    kind,
                },
                CommittedTriggerEvent::CountersRemoved {
                    object,
                    kind: removed,
                    remaining,
                    ..
                },
            ) => {
                *remaining == 0
                    && kind == *removed
                    && self.trigger_object_matches_for_controller(
                        predicate, object, source, false, controller,
                    )
            }
            (
                TriggerEventDef::BecomesBlocked(predicate),
                CommittedTriggerEvent::BecomesBlocked { object, .. },
            )
            | (
                TriggerEventDef::AttacksAndIsNotBlocked {
                    attacker: predicate,
                },
                CommittedTriggerEvent::AttacksAndIsNotBlocked { object },
            )
            | (
                TriggerEventDef::Transforms(predicate),
                CommittedTriggerEvent::Transformed { object },
            )
            // Exert and "when you do" are both a choice made inside something
            // else, published so the reflexive half can watch for it.
            | (TriggerEventDef::Exerted(predicate), CommittedTriggerEvent::Exerted { object })
            | (
                TriggerEventDef::OptionalEffectTaken(predicate),
                CommittedTriggerEvent::OptionalEffectTaken { object },
            )
            // The compulsory spelling of the same thing: the predicate reads
            // the source that sacrificed, not what it took.
            | (
                TriggerEventDef::SacrificePerformed(predicate),
                CommittedTriggerEvent::SacrificePerformed { object, .. },
            ) => self.trigger_object_matches_for_controller(
                predicate, object, source, false, controller,
            ),
            (
                TriggerEventDef::StackObject(matcher),
                CommittedTriggerEvent::StackObject {
                    object,
                    kind,
                    event: committed,
                },
            ) => {
                let occurrence_matches = match (matcher.event, committed) {
                    (
                        StackObjectEventDef::Cast { from: expected },
                        CommittedStackObjectEvent::Cast { from },
                    ) => {
                        *kind == StackObjectKind::Spell
                            && expected.is_none_or(|zone| from.zone() == zone)
                    }
                    (StackObjectEventDef::Copied, CommittedStackObjectEvent::Copied) => {
                        *kind == StackObjectKind::Spell
                    }
                    (
                        StackObjectEventDef::TargetSelection {
                            target: expected_target,
                            ..
                        },
                        CommittedStackObjectEvent::TargetSelection { target },
                    ) => self.stack_target_matches_for_controller(
                        expected_target,
                        *target,
                        source,
                        controller,
                        event.context(),
                    ),
                    _ => false,
                };
                occurrence_matches
                    && self.trigger_object_matches_for_controller(
                        matcher.object,
                        object,
                        source,
                        *kind == StackObjectKind::Spell,
                        controller,
                    )
            }
            (
                TriggerEventDef::LandPlayed {
                    land: predicate,
                    player: relation,
                },
                CommittedTriggerEvent::LandPlayed { player, object },
            ) => {
                controller.is_some_and(|controller| {
                    self.player_relation_matches(
                        *player,
                        relation,
                        controller,
                        TriggerContext::empty(),
                    )
                }) && self.trigger_object_matches_for_controller(
                    predicate, object, source, false, controller,
                )
            }
            (
                TriggerEventDef::BlocksOrBecomesBlockedBy {
                    creature: subject,
                    other: predicate,
                },
                CommittedTriggerEvent::BlocksOrBecomesBlocked { creature, other },
            ) => {
                self.trigger_object_matches_for_controller(
                    subject, creature, source, false, controller,
                ) && self.trigger_object_matches_for_controller(
                    predicate, other, source, false, controller,
                )
            }
            // The one-directional halves distinguish the pair by its attacker.
            (
                TriggerEventDef::Blocks { blocked: predicate },
                CommittedTriggerEvent::BlocksOrBecomesBlocked { creature, other },
            ) => {
                creature.id == source
                    && !creature.attacking
                    && self.trigger_object_matches_for_controller(
                        predicate, other, source, false, controller,
                    )
            }
            (
                TriggerEventDef::BecomesBlockedBy { blocker: predicate },
                CommittedTriggerEvent::BlocksOrBecomesBlocked { creature, other },
            ) => {
                creature.id == source
                    && creature.attacking
                    && self.trigger_object_matches_for_controller(
                        predicate, other, source, false, controller,
                    )
            }
            (
                TriggerEventDef::CardsExiled { zones, owner },
                CommittedTriggerEvent::CardsExiled {
                    cards,
                    from,
                    owner: exiled_by,
                },
            ) => self.exile_move_matches(zones, owner, cards, *from, *exiled_by, controller),
            // One batch, one trigger: the predicate says which of the dead
            // count toward "one or more".
            (
                TriggerEventDef::ObjectsDied { object: predicate },
                CommittedTriggerEvent::ObjectsDied { objects },
            ) => objects.iter().any(|object| {
                self.trigger_object_matches_for_controller(
                    predicate, object, source, false, controller,
                )
            }),
            // The same shape for one instruction's worth of tokens: the
            // relation says whose they are, and the predicate says which of
            // them count toward "one or more".
            (
                TriggerEventDef::TokensCreated {
                    player,
                    token: predicate,
                },
                CommittedTriggerEvent::TokensCreated {
                    tokens,
                    controller: created_by,
                },
            ) => {
                controller.is_some_and(|controller| {
                    self.player_relation_matches(
                        *created_by,
                        player,
                        controller,
                        crate::game::TriggerContext::empty(),
                    )
                }) && tokens.iter().any(|token| {
                    self.trigger_object_matches_for_controller(
                        predicate, token, source, false, controller,
                    )
                })
            }
            // One batch, one trigger: the predicate says which of the
            // unblocked attackers count, and the relation says whether the
            // player they were aimed at is the one the clause names.
            (
                TriggerEventDef::UnblockedAttackersDeclared { attacker, defender },
                CommittedTriggerEvent::UnblockedAttackersDeclared {
                    attackers,
                    defending_player,
                },
            ) => {
                controller.is_some_and(|controller| {
                    self.player_relation_matches(
                        *defending_player,
                        defender,
                        controller,
                        crate::game::TriggerContext::empty(),
                    )
                }) && attackers.iter().any(|object| {
                    self.trigger_object_matches_for_controller(
                        attacker, object, source, false, controller,
                    )
                })
            }
            // One creature of theirs damaging one player they name is the
            // whole of "one or more ... to one or more": the batch is the
            // event, and the predicates say which of it counts.
            (
                TriggerEventDef::CombatDamageDealtToPlayers {
                    sources: wanted,
                    players: relation,
                },
                CommittedTriggerEvent::CombatDamageDealtToPlayers { sources, players },
            ) => {
                controller.is_some_and(|controller| {
                    players.iter().any(|player| {
                        self.player_relation_matches(
                            *player,
                            relation,
                            controller,
                            crate::game::TriggerContext::empty(),
                        )
                    })
                }) && sources.iter().any(|object| {
                    self.trigger_object_matches_for_controller(
                        wanted, object, source, false, controller,
                    )
                })
            }
            (
                TriggerEventDef::AttackDeclared {
                    attacker,
                    declaration,
                },
                CommittedTriggerEvent::AttackersDeclared { attackers },
            ) => self.attack_declaration_matches(
                attacker,
                declaration,
                attackers,
                source,
                controller,
            ),
            (
                TriggerEventDef::Attacks(matcher),
                CommittedTriggerEvent::Attacks {
                    object,
                    declaration_size,
                    attack_number,
                    defending_player,
                    attacked_a_planeswalker,
                },
            ) => self.attacker_matches(
                matcher,
                object,
                AttackEventFacts {
                    declaration_size: *declaration_size,
                    attack_number: *attack_number,
                    defending_player: *defending_player,
                    attacked_a_planeswalker: *attacked_a_planeswalker,
                },
                source,
                controller,
            ),
            (
                TriggerEventDef::DamageDealt(matcher),
                damage @ CommittedTriggerEvent::DamageDealt { .. },
            ) => self.damage_trigger_matches(matcher, damage, source, controller),
            // Only the Class carrying the clause can reach its own levels,
            // so the object is the whole of the match.
            (
                TriggerEventDef::BecomesLevel(wanted),
                CommittedTriggerEvent::BecameLevel { object, level },
            ) => *object == source && *level == wanted,
            (
                TriggerEventDef::DrewCard(matcher),
                event @ CommittedTriggerEvent::DrewCard { .. },
            ) => self.draw_trigger_matches(matcher, event, source, controller),
            // The card the discard put into the graveyard is carried for
            // the clause that goes on to name it; which card it was does not
            // narrow the trigger, which asks only whose discard it was.
            (
                TriggerEventDef::Discarded(relation),
                CommittedTriggerEvent::Discarded { player, .. },
            )
            | (
                TriggerEventDef::DiscardedCards(relation),
                CommittedTriggerEvent::CardsDiscarded { player },
            )
            | (
                TriggerEventDef::BecomesMonarch(relation),
                CommittedTriggerEvent::BecameMonarch { player },
            )
            | (
                TriggerEventDef::CommittedCrime(relation),
                CommittedTriggerEvent::CommittedCrime { player },
            )
            | (
                TriggerEventDef::LifeGained(relation),
                CommittedTriggerEvent::LifeGained { player, .. },
            ) => {
                let controller = controller.unwrap_or(*player);
                self.player_relation_matches(*player, relation, controller, event.context())
            }
            // The listener list for a cycled card holds only that card's own
            // clauses, so there is nothing further to match on: any card
            // whose ability reached here is the card that was cycled.
            (TriggerEventDef::Cycled, CommittedTriggerEvent::Cycled { object }) => {
                object.id == source
            }
            (
                TriggerEventDef::Sacrificed {
                    object: predicate,
                    player: relation,
                },
                CommittedTriggerEvent::Sacrificed { object, player },
            ) => {
                self.player_relation_matches(
                    *player,
                    relation,
                    controller.unwrap_or(*player),
                    TriggerContext::empty(),
                ) && self.trigger_object_matches_for_controller(
                    predicate, object, source, false, controller,
                )
            }
            (
                TriggerEventDef::StepBegins { step, player },
                CommittedTriggerEvent::StepBegins {
                    step: actual_step,
                    player: actual_player,
                },
            ) => {
                if step != *actual_step {
                    return false;
                }
                if player == PlayerRelation::ChosenPlayer {
                    return self.chosen_player_of(source) == Some(*actual_player);
                }
                if player == PlayerRelation::ControllerOfAttachedPermanent {
                    return self.attached_host_controller_of(source) == Some(*actual_player);
                }
                if player == PlayerRelation::EnchantedPlayer {
                    return self.current_or_last_known_enchanted_player(source)
                        == Some(*actual_player);
                }
                let controller = controller
                    .or_else(|| self.current_or_last_known_controller(source))
                    .unwrap_or(*actual_player);
                self.player_relation_matches(*actual_player, player, controller, event.context())
            }
            _ => false,
        }
    }
}
