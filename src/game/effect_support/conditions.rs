// Asking whether a condition holds: the intervening-ifs a trigger checks
// twice, the guards an effect branches on, and the two values such a
// condition compares.
//
// Split out of `effect_support.rs` only to keep one file readable; these are
// ordinary members of the same `impl Game`. The paths and imports are the
// parent module's.

impl Game {
    pub(super) fn source_object_set_count_condition_holds(
        &self,
        condition: crate::card::ObjectSetCountConditionDef,
        source: GameObjectId,
    ) -> bool {
        let objects = self.source_object_set_targets(*condition.objects, source);
        let count = condition.filter.map_or(objects.len(), |filter| {
            objects
                .into_iter()
                .filter(|target| {
                    self.bound_object_matches(*target, filter.predicate(), source)
                })
                .count()
        });
        compare(
            &count,
            condition.comparison,
            &usize::from(condition.amount),
        )
    }

    /// Whether one chosen target answers a characteristic predicate.
    ///
    /// What is read depends on what the slot named: a permanent on the
    /// battlefield, a spell on the stack, or a card in a zone that is not
    /// either -- "if it was a permanent card" asks that of a card in a
    /// graveyard, which is as public as the board.
    fn chosen_target_matches(
        &self,
        target: Target,
        predicate: ObjectPredicateDef,
        source: GameObjectId,
    ) -> bool {
        let matched = match target {
            Target::Permanent(id) => self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == id)
                .map(|permanent| self.trigger_event_object(permanent)),
            Target::Spell(id) => self
                .stack
                .iter()
                .find(|candidate| candidate.id == id)
                .and_then(|candidate| self.stack_trigger_event_object(candidate)),
            // A card that has already moved during this resolution is still
            // the card the activation named -- "when a creature card is
            // exiled this way" asks about the card that just left the
            // graveyard -- so follow the move rather than finding nothing.
            Target::Card(id) => {
                let id = if self.card_in_nonbattlefield_zone(id).is_some() {
                    id
                } else {
                    self.successors.get(&id).copied().unwrap_or(id)
                };
                return self.card_in_nonbattlefield_zone(id).is_some_and(|(zone, card)| {
                    self.card_object_matches(predicate, card, zone, source)
                });
            }
            // A player has no characteristics to read.
            Target::Player(_) => None,
        };
        matched.is_some_and(|matched| {
            self.trigger_object_matches(predicate, &matched, source, false)
        })
    }

    /// How many times this ability has been activated from this permanent so
    /// far this turn.
    /// One side of a two-value condition. The board-readable values a cost
    /// reduction already answers, plus the two a condition of this shape
    /// actually asks about: what a player is devoted to, and how much
    /// library they have left.
    fn condition_value(
        &self,
        value: crate::card::ValueDef,
        source: GameObjectId,
        controller: PlayerId,
        context: TriggerContext,
    ) -> i32 {
        match value {
            // Counted with the trigger's own context, so a query can name the
            // player the event happened to: "more creatures than they do" is
            // asked about the player whose upkeep began, not about whoever
            // controls the enchantment asking.
            crate::card::ValueDef::CountMatchingObjects(query) => i32::try_from(
                self.objects_matching_query(*query, controller, source, context)
                    .len(),
            )
            .unwrap_or(i32::MAX),
            // Counted the same way and from the same walk, over names rather
            // than over objects: "seven or more lands with different names"
            // is an intervening-if, asked before anything is resolving.
            crate::card::ValueDef::DistinctNamesAmong(query) => self.distinct_names_among(
                &self.objects_matching_query(*query, controller, source, context),
            ),
            crate::card::ValueDef::CountSpellsCastThisTurn(query) => {
                self.spells_cast_matching_this_turn(*query, controller, source, context)
            }
            crate::card::ValueDef::LifeTotal(relation) => [PlayerId::One, PlayerId::Two]
                .into_iter()
                .find(|player| {
                    self.player_relation_matches(*player, relation, controller, context)
                })
                .map_or(0, |player| i32::from(self.players[player.index()].life)),
            // "Activate only if this creature's power is 3 or greater": read
            // live off the source, so a pump that resolved in response is
            // part of the answer.
            crate::card::ValueDef::SourcePower => self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == source)
                .and_then(|permanent| self.power(permanent))
                .map_or(0, i32::from),
            // Read live: an intervening-if asking whether this was the
            // first land is asked after the land drop was counted.
            crate::card::ValueDef::LandsPlayedThisTurn(relation) => {
                [PlayerId::One, PlayerId::Two]
                    .into_iter()
                    .filter(|player| {
                        self.player_relation_matches(*player, relation, controller, context)
                    })
                    .map(|player| i32::from(self.players[player.index()].lands_played_this_turn))
                    .sum()
            }
            crate::card::ValueDef::DevotionTo(_)
            | crate::card::ValueDef::LibrarySize(_)
            | crate::card::ValueDef::SpellsCastThisGame(_)
            | crate::card::ValueDef::BasicLandTypesControlled(_) => {
                self.player_readable_value(value, controller)
            }
            crate::card::ValueDef::CountObjects(objects) => i32::try_from(
                self.source_object_set_targets(*objects, source).len(),
            )
            .unwrap_or(i32::MAX),
            crate::card::ValueDef::CardTypesAmongObjects(objects) => self
                .card_types_among_targets(&self.source_object_set_targets(*objects, source)),
            crate::card::ValueDef::CardTypesAmongGraveyards(player) => {
                self.card_types_among_graveyards(player, controller)
            }
            // "If defending player has more cards in hand than you" is two
            // hands compared, so both sides are read here rather than in the
            // layer walk that sizes a creature by one of them.
            crate::card::ValueDef::CardsInHandAbove { player, threshold } => {
                let counted = [PlayerId::One, PlayerId::Two]
                    .into_iter()
                    .filter(|candidate| {
                        self.player_relation_matches(
                            *candidate,
                            player,
                            controller,
                            crate::game::TriggerContext::empty(),
                        )
                    })
                    .map(|candidate| self.players[candidate.index()].hand.len())
                    .sum::<usize>();
                i32::try_from(counted.saturating_sub(usize::from(threshold))).unwrap_or(i32::MAX)
            }
            // A per-turn tally the game keeps, read the same way whether the
            // question comes from a resolving effect or from an
            // intervening-if that has no resolving object yet.
            crate::card::ValueDef::CardsDrawnThisTurn(relation) => [PlayerId::One, PlayerId::Two]
                .into_iter()
                .filter(|player| {
                    self.player_relation_matches(*player, relation, controller, context)
                })
                .map(|player| i32::from(self.cards_drawn_this_turn[player.index()]))
                .sum(),
            // The life tally beside it, read the same way and for the same
            // reason: "if you gained 3 or more life this turn" is asked
            // before anything is resolving.
            crate::card::ValueDef::LifeGainedThisTurn(relation) => [PlayerId::One, PlayerId::Two]
                .into_iter()
                .filter(|player| {
                    self.player_relation_matches(*player, relation, controller, context)
                })
                .map(|player| i32::from(self.life_gained_this_turn[player.index()]))
                .sum(),
            // The X its own spell was cast for, which the permanent recorded
            // as it arrived. An intervening "if X is 5 or more" asks about
            // that number, so it has to be readable here and not only where a
            // counter count is being handed to an entry replacement.
            crate::card::ValueDef::SourceCastX => self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == source)
                .map_or(0, |permanent| {
                    i32::from(permanent.cast.as_ref().map_or(0, |cast| cast.x))
                }),
            other => i32::from(self.cost_reduction_value(other, controller, source)),
        }
    }

    /// How many times this object's copy of one ability has resolved this
    /// turn, including the resolution asking: the count is recorded as a
    /// resolution begins, so "the first time" reads one.
    pub(super) fn ability_resolutions_this_turn(
        &self,
        source: GameObjectId,
        ability: AbilityOrigin,
    ) -> u8 {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
            .and_then(|permanent| {
                permanent
                    .resolutions_this_turn
                    .iter()
                    .find(|(origin, _)| *origin == ability)
            })
            .map_or(0, |(_, count)| *count)
    }

    pub(super) fn ability_activations_this_turn(
        &self,
        source: GameObjectId,
        ability: AbilityOrigin,
    ) -> u8 {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
            .and_then(|permanent| {
                permanent
                    .activations_this_turn
                    .iter()
                    .find(|(origin, _)| *origin == ability)
            })
            .map_or(0, |(_, count)| *count)
    }

    /// Whether a trigger's intervening-if condition holds right now. Rule
    /// 603.4 asks this when the ability would trigger and again as it
    /// resolves, so both call sites read the same board.
    #[allow(clippy::too_many_lines)]
    pub(super) fn trigger_condition_holds(
        &self,
        condition: &TriggerConditionDef,
        source: GameObjectId,
        controller: PlayerId,
        context: TriggerContext,
        ability: Option<AbilityOrigin>,
        object: Option<(&StackObject, ScopedEffect, &EffectResolutionContext)>,
    ) -> bool {
        let TriggerConditionDef::ObjectCount {
            query,
            comparison,
            amount,
        } = condition
        else {
            return match condition {
                TriggerConditionDef::All(conditions) => conditions.iter().all(|condition| {
                    self.trigger_condition_holds(
                        condition, source, controller, context, ability, object,
                    )
                }),
                TriggerConditionDef::AnyOf(conditions) => conditions.iter().any(|condition| {
                    self.trigger_condition_holds(
                        condition, source, controller, context, ability, object,
                    )
                }),
                TriggerConditionDef::Not(condition) => !self.trigger_condition_holds(
                    condition, source, controller, context, ability, object,
                ),
                // Both sides are read where the condition is checked, which
                // is the only way "X or more cards in your library" can be
                // said: neither amount is a printed number.
                TriggerConditionDef::ValueComparison(values) => {
                    let value = |value| {
                        object.map_or_else(
                            || self.condition_value(value, source, controller, context),
                            |(object, scoped, effect_context)| {
                                self.effect_value(value, object, effect_context, scoped)
                            },
                        )
                    };
                    let left = value(values.left);
                    let right = value(values.right);
                    compare(&left, values.comparison, &right)
                }
                TriggerConditionDef::SourceOnBattlefield => self
                    .battlefield
                    .iter()
                    .any(|permanent| permanent.card.id == source),
                TriggerConditionDef::SourceInZone(zone) => self
                    .card_in_nonbattlefield_zone(source)
                    .is_some_and(|(actual, _)| actual == *zone),
                // Names, not identities: a second copy of the named card is
                // still the named card, so the definitions are compared.
                TriggerConditionDef::BoundObjectsShareName { first, second } => {
                    let named = |objects: &&'static ObjectSetDef| {
                        object
                            .and_then(|(object, scoped, context)| {
                                self.effect_objects(**objects, object, context, scoped)
                                    .into_iter()
                                    .next()
                            })
                            .and_then(|target| match target {
                                Target::Permanent(id) | Target::Card(id) | Target::Spell(id) => {
                                    self.object_definition(id)
                                }
                                Target::Player(_) => None,
                            })
                    };
                    match (named(first), named(second)) {
                        (Some(first), Some(second)) => first == second,
                        _ => false,
                    }
                }
                // The card an earlier choice saved, read where it is now:
                // the clause that asks is the same resolution that chose it.
                TriggerConditionDef::BoundObjectMatches {
                    binding,
                    object: predicate,
                } => object.is_some_and(|(_, _, context): (_, _, &EffectResolutionContext)| {
                    context.single_object(*binding).is_some_and(|bound| {
                        self.bound_object_matches(bound, *predicate, source)
                    })
                }),
                // The permanent records the controller's turn count as it
                // arrived. By this upkeep that count has advanced once, so
                // "since the last upkeep" is exactly one turn ago -- and the
                // check stops being true afterwards, which is what keeps an
                // echo cost from coming due a second time.
                TriggerConditionDef::SourceArrivedSinceControllersLastUpkeep => self
                    .battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == source)
                    .is_some_and(|permanent| {
                        self.turns_started[permanent.controller.index()]
                            == permanent.entered_controller_turn.saturating_add(1)
                    }),
                TriggerConditionDef::SourceUntapped => self
                    .battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == source)
                    .is_some_and(|permanent| !permanent.tapped),
                TriggerConditionDef::ActivePlayer(relation) => {
                    self.player_relation_matches(self.active_player, *relation, controller, context)
                }
                // A tie counts, so this asks whether anybody is strictly
                // ahead of the players the relation names rather than
                // whether one of them is alone in front. A relation that
                // names nobody is false: there is no player to be ahead.
                TriggerConditionDef::PlayerHasMostLife(relation) => {
                    let best = [PlayerId::One, PlayerId::Two]
                        .into_iter()
                        .map(|player| self.players[player.index()].life)
                        .max();
                    let mut matching =
                        [PlayerId::One, PlayerId::Two].into_iter().filter(|player| {
                            self.player_relation_matches(*player, *relation, controller, context)
                        });
                    best.is_some_and(|best| {
                        matching.any(|player| self.players[player.index()].life >= best)
                    })
                }
                TriggerConditionDef::SpellsCastThisTurn {
                    quantifier,
                    player: relation,
                    comparison,
                    amount,
                } => {
                    let mut matching =
                        [PlayerId::One, PlayerId::Two].into_iter().filter(|player| {
                            self.player_relation_matches(*player, *relation, controller, context)
                        });
                    let satisfies = |player: PlayerId| {
                        compare(
                            &self.spells_cast_this_turn[player.index()],
                            *comparison,
                            &u16::from(*amount),
                        )
                    };
                    match quantifier {
                        QuantifierDef::Every => matching.all(satisfies),
                        QuantifierDef::Any => matching.any(satisfies),
                    }
                }
                TriggerConditionDef::SpellsCastLastTurn {
                    quantifier,
                    player: relation,
                    comparison,
                    amount,
                } => {
                    let mut matching =
                        [PlayerId::One, PlayerId::Two].into_iter().filter(|player| {
                            self.player_relation_matches(*player, *relation, controller, context)
                        });
                    let satisfies = |player: PlayerId| {
                        compare(
                            &self.spells_cast_last_turn[player.index()],
                            *comparison,
                            &u16::from(*amount),
                        )
                    };
                    match quantifier {
                        QuantifierDef::Every => matching.all(satisfies),
                        QuantifierDef::Any => matching.any(satisfies),
                    }
                }
                // A tie counts, so this asks whether anything is strictly
                // bigger rather than whether one creature is unique.
                TriggerConditionDef::ControlsGreatestPowerCreature => {
                    let mut best: Option<i16> = None;
                    let mut mine: Option<i16> = None;
                    for permanent in &self.battlefield {
                        let Some(power) = self.power(permanent) else {
                            continue;
                        };
                        best = Some(best.map_or(power, |seen: i16| seen.max(power)));
                        if permanent.controller == controller {
                            mine = Some(mine.map_or(power, |seen: i16| seen.max(power)));
                        }
                    }
                    match (mine, best) {
                        (Some(mine), Some(best)) => mine >= best,
                        _ => false,
                    }
                }
                // Follows the attachment rather than being frozen when the
                // Equipment moved, so the answer is about where it is now.
                TriggerConditionDef::ControllerHasCitysBlessing => {
                    self.citys_blessing[controller.index()]
                }
                TriggerConditionDef::ControllerGainedLifeThisTurn => {
                    self.life_gained_this_turn[controller.index()] > 0
                }
                TriggerConditionDef::ControllerHadPermanentLeaveThisTurn => {
                    self.permanent_left_battlefield_this_turn[controller.index()]
                }
                TriggerConditionDef::ControllerHadCardLeaveGraveyardThisTurn => {
                    self.card_left_graveyard_this_turn[controller.index()]
                }
                TriggerConditionDef::CreatureDiedThisTurn => self.creature_died_this_turn,
                // A dies-trigger asks about the permanent that died, which
                // is no longer there to look at. "If it was a creature" has
                // to read what it last was (CR 603.10); finding nothing and
                // answering no would be the wrong answer rather than a
                // deliberate one.
                TriggerConditionDef::ObjectSetCount(counting) => {
                    self.source_object_set_count_condition_holds(**counting, source)
                }
                TriggerConditionDef::SourceMatches { object: predicate } => {
                    if let Some(permanent) = self
                        .battlefield
                        .iter()
                        .find(|permanent| permanent.card.id == source)
                        .or_else(|| match self.retired_objects.get(&source) {
                            Some(crate::game::RetiredObject::Permanent { permanent, .. }) => {
                                Some(permanent.as_ref())
                            }
                            _ => None,
                        })
                    {
                        self.trigger_object_matches(
                            *predicate,
                            &self.trigger_event_object(permanent),
                            source,
                            false,
                        )
                    } else {
                        self.card_in_nonbattlefield_zone(source).is_some_and(|(zone, card)| {
                            self.card_object_matches(*predicate, card, zone, source)
                        })
                    }
                }
                TriggerConditionDef::AttachedPermanentMatches { object: predicate } => self
                    .current_or_last_known_attached_host(source)
                    .and_then(|host| {
                        self.battlefield
                            .iter()
                            .find(|permanent| permanent.card.id == host)
                    })
                    .is_some_and(|host| {
                        self.trigger_object_matches(
                            *predicate,
                            &self.trigger_event_object(host),
                            source,
                            false,
                        )
                    }),
                // Read live off the source, so a card whose counters change
                // during a turn answers differently each time it is asked --
                // and off what it last carried once it has left, the way
                // `SourceIsTapped` below does. An ability that has lost its
                // source uses last known information (CR 608.2h): Malcolm
                // killed in response still pays for the discard with the
                // four chorus counters he had when he was last there.
                TriggerConditionDef::SourceCounters {
                    kind,
                    comparison,
                    amount,
                } => compare(
                    &self.current_or_last_known_counters(source, *kind),
                    *comparison,
                    &u16::from(*amount),
                ),
                // A permanent remembers how it was cast; a spell still on the
                // stack carries the cast signature itself, which is where a
                // "when you cast this spell, if it was kicked" trigger has to
                // read it -- the permanent does not exist yet.
                // Read from the permanent, which remembers it, or from the
                // spell still on the stack when the question comes earlier.
                TriggerConditionDef::SourceCastAtInstantSpeed => self
                    .cast_context_for(source, object.map(|(resolving, _, _)| resolving))
                    .is_some_and(|cast| cast.at_instant_speed),
                TriggerConditionDef::SourceCastFrom(zone) => {
                    self.cast_context_for(source, object.map(|(resolving, _, _)| resolving))
                        .and_then(|cast| cast.source_zone)
                        .is_some_and(|from| from.zone() == *zone)
                }
                // Any recorded cast zone means it was cast; nothing else
                // sets one.
                TriggerConditionDef::SourceWasCast => self
                    .cast_context_for(source, object.map(|(resolving, _, _)| resolving))
                    .is_some_and(|cast| cast.was_cast()),
                // "If it was kicked" is a fact about how the source was cast
                // rather than about where it is now, so it survives the source:
                // a Thieving Skydiver killed with his arrival trigger still on
                // the stack was kicked when it resolves, and still steals.
                TriggerConditionDef::SourceCastWith(kind) => self
                    .cast_context_for(source, object.map(|(resolving, _, _)| resolving))
                    .is_some_and(|cast| cast.alternative == Some(*kind)),
                TriggerConditionDef::SourcePaidAdditionalCost(cost) => {
                    self.source_additional_cost_payments(source, *cost) > 0
                }
                TriggerConditionDef::SourceLoyalty { comparison, amount } => self
                    .battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == source)
                    .is_some_and(|permanent| {
                        compare(
                            &permanent.counters(CounterKind::Loyalty),
                            *comparison,
                            &u16::from(*amount),
                        )
                    }),
                // Counting the activation now resolving is what makes
                // "activated four or more times" true on the fourth one.
                TriggerConditionDef::SourceActivationsThisTurn { comparison, amount } => ability
                    .is_some_and(|origin| {
                        compare(
                            &self.ability_activations_this_turn(source, origin),
                            *comparison,
                            amount,
                        )
                    }),
                TriggerConditionDef::SourceResolutionsThisTurn { comparison, amount } => ability
                    .is_some_and(|origin| {
                        compare(
                            &self.ability_resolutions_this_turn(source, origin),
                            *comparison,
                            amount,
                        )
                    }),
                // Read now rather than when the ability was created, so a
                // delayed effect asks about the target as it is at that point.
                TriggerConditionDef::TargetMatches {
                    slot,
                    object: predicate,
                } => object.is_some_and(|(stack, scoped, _)| {
                    Self::chosen_targets(stack, scoped.target_slot(*slot))
                        .any(|target| self.chosen_target_matches(target, *predicate, source))
                }),
                TriggerConditionDef::SourceDealtDamageToOpponentThisTurn => self
                    .battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == source)
                    .is_some_and(|permanent| permanent.dealt_damage_to_opponent_this_turn),
                TriggerConditionDef::SourceIsPaired => self
                    .battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == source)
                    .is_some_and(|permanent| permanent.paired_with.is_some()),
                TriggerConditionDef::SourceIsTapped => self.current_or_last_known_tapped(source),
                TriggerConditionDef::SourceIsUntapped => !self.current_or_last_known_tapped(source),
                TriggerConditionDef::ControllerLifeAtMost(threshold) => {
                    i32::from(self.players[controller.index()].life) <= i32::from(*threshold)
                }
                // Rounded up, as every "half your starting life total" clause
                // is: at twenty the boundary is ten, and at an odd total it
                // is the higher half.
                TriggerConditionDef::ControllerLifeAtMostHalfStartingLife => {
                    let starting = i32::from(self.format.rules().starting_life);
                    i32::from(self.players[controller.index()].life) <= starting.div_euclid(2)
                }
                TriggerConditionDef::ObjectCount { .. } => {
                    unreachable!("the object-count arm is destructured above")
                }
            };
        };
        let mut count = 0;
        let result = self.visit_objects_matching_query_with_prospective(
            *query,
            controller,
            source,
            context,
            None,
            object,
            |_| {
                count += 1;
                ControlFlow::Continue(())
            },
        );
        debug_assert!(result.is_continue());
        compare(&i64::from(count), *comparison, &i64::from(*amount))
    }
}
