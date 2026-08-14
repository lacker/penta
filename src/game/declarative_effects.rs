use super::{
    AbilitySourceRef, AddManaEffectDef, CardPartId, CharacteristicSource, CopiableAbility, CostDef,
    CounteredSpellZone, DeclarativeAbilityDef, DelayedTrigger, DiscardSelectionDef,
    DrawReplacement, EffectDef, EffectRecipientDef, FloatingTrigger, Game, GameResult, Mana,
    ManaPool, ManaSelectionDef, ManaSource, Permanent, SacrificeFollowup, ScopedEffect,
    StackObject, Target, TriggerCapture, TriggerContext, ValueDef, WinReason, ZoneKind,
    ZoneMoveCause, public_cards,
};

mod prevention;

impl Game {
    #[allow(clippy::too_many_lines)]
    pub(super) fn resolve_effect_def(
        &mut self,
        scoped: ScopedEffect,
        object: &StackObject,
        context: TriggerContext,
    ) {
        match scoped.effect {
            EffectDef::Sequence(effects) => {
                self.resolve_effects_in_order(
                    effects
                        .iter()
                        .map(|effect| scoped.with_effect(*effect))
                        .collect(),
                    object,
                    context,
                    None,
                );
            }
            EffectDef::Randomized {
                likelihood,
                on_success,
                on_failure,
            } => {
                let branch = if self.rng.sample_probability(likelihood.value()) {
                    on_success
                } else {
                    on_failure
                };
                self.resolve_effect_def(scoped.with_effect(*branch), object, context);
            }
            EffectDef::ChoosePermanent {
                choice,
                chooser,
                object: predicate,
                controller,
                then,
            } => {
                let choosers = self.effect_recipients(chooser, object, context, scoped);
                for chooser in choosers {
                    if let Target::Player(chooser) = chooser {
                        self.queue_permanent_effect_choice(
                            choice,
                            chooser,
                            predicate,
                            controller,
                            object,
                            context,
                            scoped.with_effect(*then),
                        );
                    }
                }
            }
            EffectDef::AddMana(AddManaEffectDef {
                mana: ManaSelectionDef::One(kind),
                amount,
                restrictions,
                spend_effects,
                damage_to_controller,
            }) => {
                let color = kind;
                let source = object
                    .source
                    .zip(object.ability_origin())
                    .map(|(object, ability)| ManaSource { object, ability });
                let mana = Mana {
                    color,
                    source,
                    restrictions,
                    spend_effects,
                };
                self.add_mana(
                    object.controller,
                    std::iter::repeat_n(mana, usize::from(amount)),
                );
                if damage_to_controller > 0 {
                    self.damage_target_from(
                        object.source.or(Some(object.id)),
                        Some(Target::Player(object.controller)),
                        damage_to_controller,
                    );
                }
            }
            EffectDef::DrainLife { recipient, amount } => {
                let amount = self
                    .effect_value(amount, object, context, scoped)
                    .max(0)
                    .try_into()
                    .unwrap_or(u16::MAX);
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    let available = self.drainable_from(target);
                    let dealt = self.damage_target_from(Some(object.id), Some(target), amount);
                    self.gain_life(object.controller, dealt.min(available));
                }
            }
            EffectDef::DealDamage { recipient, amount } => {
                // A divided total is chosen per target when the spell is
                // cast, so each one takes its own share rather than the same
                // amount as everyone else.
                let divided = matches!(amount, ValueDef::DividedAmongTargets);
                let shared = if divided {
                    0
                } else {
                    self.effect_value(amount, object, context, scoped)
                        .max(0)
                        .try_into()
                        .unwrap_or(u16::MAX)
                };
                let slot = match recipient {
                    EffectRecipientDef::Target(target) => Some(scoped.target_slot(target)),
                    _ => None,
                };
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    let amount = if divided {
                        slot.and_then(|slot| Self::divided_share(object, slot, target))
                            .unwrap_or(0)
                    } else {
                        shared
                    };
                    if amount == 0 && divided {
                        continue;
                    }
                    self.damage_target_from(
                        object.source.or(Some(object.id)),
                        Some(target),
                        amount,
                    );
                }
            }
            EffectDef::GainLife { recipient, amount } => {
                let amount = self
                    .effect_value(amount, object, context, scoped)
                    .max(0)
                    .try_into()
                    .unwrap_or(u16::MAX);
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Player(player) = target {
                        self.gain_life(player, amount);
                    }
                }
            }
            EffectDef::DrawCards { recipient, amount } => {
                let amount = self
                    .effect_value(amount, object, context, scoped)
                    .max(0)
                    .try_into()
                    .unwrap_or(u16::MAX);
                let mut players = self
                    .effect_recipients(recipient, object, context, scoped)
                    .into_iter()
                    .filter_map(|target| match target {
                        Target::Player(player) => Some(player),
                        Target::Card(_) | Target::Permanent(_) | Target::Spell(_) => None,
                    })
                    .collect::<Vec<_>>();
                // CR 121.2c: when multiple players draw, the active player
                // performs every individual draw first, followed by the
                // nonactive player. This order belongs to drawing rather than
                // to the general `EachPlayer` recipient.
                players.sort_by_key(|player| (*player != self.active_player, player.index()));
                for player in players {
                    self.draw_cards(player, amount);
                }
            }
            EffectDef::ShuffleLibrary { player: recipient } => {
                let mut players = self
                    .effect_recipients(recipient, object, context, scoped)
                    .into_iter()
                    .filter_map(|target| match target {
                        Target::Player(player) => Some(player),
                        Target::Card(_) | Target::Permanent(_) | Target::Spell(_) => None,
                    })
                    .collect::<Vec<_>>();
                players.sort_by_key(|player| (*player != self.active_player, player.index()));
                for player in players {
                    self.rng.shuffle(&mut self.players[player.index()].library);
                }
            }
            EffectDef::EmptyManaPool { player: recipient } => {
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Player(player) = target {
                        self.players[player.index()].mana_pool = ManaPool::default();
                        self.players[player.index()].mana.clear();
                    }
                }
            }
            EffectDef::Discard {
                recipient,
                amount,
                selection: DiscardSelectionDef::RecipientChooses,
            } => {
                let amount = self.effect_value(amount, object, context, scoped).max(0);
                let cause = ZoneMoveCause::Effect {
                    controller: object.controller,
                };
                let players = self
                    .effect_recipients(recipient, object, context, scoped)
                    .into_iter()
                    .filter_map(|target| match target {
                        Target::Player(player) => Some(player),
                        Target::Card(_) | Target::Permanent(_) | Target::Spell(_) => None,
                    })
                    .collect();
                self.queue_effect_discards(players, amount, cause);
            }
            EffectDef::Discard {
                recipient,
                amount,
                selection: DiscardSelectionDef::Random,
            } => {
                let amount = self
                    .effect_value(amount, object, context, scoped)
                    .max(0)
                    .try_into()
                    .unwrap_or(u16::MAX);
                let cause = ZoneMoveCause::Effect {
                    controller: object.controller,
                };
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Player(player) = target {
                        self.discard_random(player, amount, cause);
                    }
                }
            }
            EffectDef::LoseLife { recipient, amount } => {
                let amount = self
                    .effect_value(amount, object, context, scoped)
                    .max(0)
                    .try_into()
                    .unwrap_or(u16::MAX);
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Player(player) = target {
                        self.lose_life(player, amount);
                    }
                }
            }
            EffectDef::Tap { object: recipient } => {
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Permanent(permanent) = target {
                        let _ = self.tap_permanent(permanent);
                    }
                }
            }
            EffectDef::Regenerate { object: recipient } => {
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Permanent(permanent) = target {
                        self.add_regeneration_shield(permanent);
                    }
                }
            }
            EffectDef::CreateToken { token, count } => {
                for _ in 0..self.effect_value(count, object, context, scoped).max(0) {
                    self.create_token(object.controller, token);
                }
            }
            EffectDef::Untap { object: recipient } => {
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Permanent(id) = target
                        && let Some(permanent) = self
                            .battlefield
                            .iter_mut()
                            .find(|candidate| candidate.card.id == id)
                    {
                        permanent.tapped = false;
                    }
                }
            }
            EffectDef::PreventNextDamage { .. }
            | EffectDef::ChooseDamageSource { .. }
            | EffectDef::PreventNextDamageFromSource { .. }
            | EffectDef::PreventAllDamageThisTurn { .. }
            | EffectDef::PreventAllCombatDamageThisTurn
            | EffectDef::PreventCombatDamageThisTurn { .. }
            | EffectDef::PreventCombatDamageDealtByThisTurn { .. }
            | EffectDef::PreventDamageToPlayerAndControlledCreaturesThisTurn { .. }
            | EffectDef::PreventAllCombatDamageExceptSourceThisTurn { .. } => {
                self.resolve_prevention_effect(scoped, object, context);
            }
            EffectDef::Destroy {
                object: recipient,
                can_regenerate,
            } => {
                let permanents = self
                    .effect_recipients(recipient, object, context, scoped)
                    .into_iter()
                    .filter_map(|target| match target {
                        Target::Permanent(permanent) => Some(permanent),
                        Target::Player(_) | Target::Card(_) | Target::Spell(_) => None,
                    })
                    .collect::<Vec<_>>();
                self.destroy_permanents(&permanents, can_regenerate);
            }
            EffectDef::Sacrifice { object: recipient } => {
                let permanents = self
                    .effect_recipients(recipient, object, context, scoped)
                    .into_iter()
                    .filter_map(|target| match target {
                        Target::Permanent(permanent) => Some(permanent),
                        Target::Player(_) | Target::Card(_) | Target::Spell(_) => None,
                    })
                    .filter(|permanent| {
                        self.permanent_controller(*permanent)
                            .is_none_or(|controller| {
                                self.can_be_forced_to_sacrifice(controller, object.controller)
                            })
                    })
                    .collect::<Vec<_>>();
                self.move_permanents_to_graveyard(&permanents);
            }
            EffectDef::DestroyOfChoice {
                player: recipient,
                object: predicate,
                can_regenerate,
            } => {
                let source = object.source.unwrap_or(object.id);
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Player(player) = target {
                        self.queue_chosen_destruction(player, predicate, source, can_regenerate);
                    }
                }
            }
            EffectDef::SacrificeOfChoice {
                player: recipient,
                object: predicate,
                then,
                optional,
            } => {
                let source = object.source.unwrap_or(object.id);
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    let Target::Player(player) = target else {
                        continue;
                    };
                    // A prohibition on being forced to sacrifice does not
                    // reach an offer the player is free to decline.
                    if !optional && !self.can_be_forced_to_sacrifice(player, object.controller) {
                        continue;
                    }
                    let followup = then.map(|effect| SacrificeFollowup {
                        object: Box::new(object.clone()),
                        context,
                        effect: scoped.with_effect(*effect),
                    });
                    self.queue_chosen_sacrifice(player, predicate, source, followup, optional);
                }
            }
            EffectDef::SplitPermanentsAndSacrificeAPile { player: recipient } => {
                let splitter = object.controller;
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Player(player) = target {
                        self.queue_pile_split(splitter, player);
                    }
                }
            }
            EffectDef::Mill {
                player: recipient,
                amount,
            } => {
                let count = self.effect_value(amount, object, context, scoped).max(0);
                let Ok(count) = usize::try_from(count) else {
                    return;
                };
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Player(player) = target {
                        let milled = self.take_top_of_library(player, count);
                        self.bury_cards(player, milled);
                    }
                }
            }
            EffectDef::RevealAndSplitIntoPiles {
                count,
                rest,
                placement,
            } => {
                let count = self.effect_value(count, object, context, scoped).max(0);
                let Ok(count) = usize::try_from(count) else {
                    return;
                };
                self.queue_revealed_pile_split(object.controller, count, rest, placement);
            }
            EffectDef::LookAtHand { player: recipient } => {
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Player(seen) = target {
                        self.last_seen_hands[object.controller.index()] =
                            Some((seen, public_cards(&self.players[seen.index()].hand)));
                    }
                }
            }
            EffectDef::LookAtTopAndMayTake {
                player: recipient,
                object: predicate,
            } => {
                let source = object.source.unwrap_or(object.id);
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Player(player) = target {
                        self.queue_top_card_offer(player, predicate, source);
                    }
                }
            }
            EffectDef::LookAtTopAndSelect {
                player: recipient,
                selection,
            } => {
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Player(player) = target {
                        self.queue_top_card_selection(player, selection, object, context, scoped);
                    }
                }
            }
            EffectDef::SearchZone {
                player: recipient,
                source: source_zone,
                object: predicate,
                minimum,
                maximum,
                reveal,
                destination,
                placement,
                shuffle,
            } => {
                let source = object.source.unwrap_or(object.id);
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Player(player) = target {
                        self.queue_zone_search(
                            player,
                            source_zone,
                            predicate,
                            minimum,
                            maximum,
                            reveal,
                            destination,
                            placement,
                            shuffle,
                            source,
                            object.controller,
                        );
                    }
                }
            }
            EffectDef::ChooseCards {
                player: recipient,
                sources,
                object: predicate,
                minimum,
                maximum,
                reveal,
                destination,
                placement,
            } => {
                let source = object.source.unwrap_or(object.id);
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Player(player) = target {
                        self.queue_owned_card_choice(
                            player,
                            sources,
                            predicate,
                            minimum,
                            maximum,
                            reveal,
                            destination,
                            placement,
                            source,
                            object.controller,
                        );
                    }
                }
            }
            EffectDef::ReplaceNextDrawThisTurn {
                player: recipient,
                effect,
            } => {
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Player(player) = target {
                        self.draw_replacements[player.index()].push_back(DrawReplacement {
                            object: Box::new(object.clone()),
                            context,
                            effect: scoped.with_effect(*effect),
                        });
                    }
                }
            }
            EffectDef::IfFormat {
                format,
                then,
                otherwise,
            } => {
                let effect = if self.format == format {
                    then
                } else {
                    otherwise
                };
                self.resolve_effect_def(scoped.with_effect(*effect), object, context);
            }
            EffectDef::CreateEmblem { emblem } => {
                let controller = object.controller;
                let card =
                    self.unbacked_object(emblem, controller, CharacteristicSource::Ability(emblem));
                let mut emblem = Permanent::entering(
                    card,
                    CardPartId::PRIMARY,
                    controller,
                    self.turns_started[controller.index()],
                );
                emblem.timestamp = self.allocate_continuous_effect_timestamp();
                emblem.emblem_source = object.ability_origin();
                self.emblems.push(emblem);
            }
            EffectDef::LoseTheGame { player: recipient } => {
                let mut losers = self
                    .effect_recipients(recipient, object, context, scoped)
                    .into_iter()
                    .filter_map(|target| match target {
                        Target::Player(player) => Some(player),
                        Target::Card(_) | Target::Permanent(_) | Target::Spell(_) => None,
                    })
                    .collect::<Vec<_>>();
                losers.sort_unstable();
                losers.dedup();
                match losers.as_slice() {
                    [loser] => self.finish(GameResult::Winner {
                        winner: loser.opponent(),
                        reason: WinReason::OpponentLostToAnEffect,
                    }),
                    [_, _] => self.finish(GameResult::Draw),
                    [] => {}
                    _ => unreachable!("a two-player game has at most two losers"),
                }
            }
            EffectDef::Transform { object: recipient } => {
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Permanent(id) = target {
                        self.transform_permanent(id);
                    }
                }
            }
            EffectDef::AdditionalCombatPhase => {
                self.additional_combat_phases = self.additional_combat_phases.saturating_add(1);
            }
            EffectDef::TakeExtraTurn { player: recipient } => {
                let players = self
                    .effect_recipients(recipient, object, context, scoped)
                    .into_iter()
                    .filter_map(|target| match target {
                        Target::Player(player) => Some(player),
                        Target::Card(_) | Target::Permanent(_) | Target::Spell(_) => None,
                    });
                self.schedule_extra_turns(players);
            }
            EffectDef::CannotCastNoncreatureSpellsThisTurn { player: recipient } => {
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Player(player) = target {
                        self.noncreature_casts_locked[player.index()] = true;
                    }
                }
            }
            EffectDef::GrantFlashToNextSorcery => {
                let grants = &mut self.sorcery_flash_grants[object.controller.index()];
                *grants = grants.saturating_add(1);
            }
            EffectDef::May {
                player: recipient,
                effect,
            } => {
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Player(player) = target {
                        self.queue_optional_effect(
                            player,
                            object,
                            context,
                            scoped.with_effect(*effect),
                        );
                    }
                }
            }
            EffectDef::ExileLinkedToSource { object: recipient } => {
                let source = object.source.unwrap_or(object.id);
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    let exiled = match target {
                        Target::Permanent(id) => self.exile_permanent_returning_card(id),
                        Target::Card(id) => self.exile_card_returning_card(id),
                        Target::Player(_) | Target::Spell(_) => None,
                    };
                    if let Some(exiled) = exiled {
                        self.linked_exiles.push((source, exiled));
                    }
                }
            }
            EffectDef::ReturnLinkedExiles { zone, grant } => {
                let source = object.source.unwrap_or(object.id);
                let returning = self
                    .linked_exiles
                    .iter()
                    .filter(|(exiled_by, _)| *exiled_by == source)
                    .map(|(_, card)| *card)
                    .collect::<Vec<_>>();
                self.linked_exiles
                    .retain(|(exiled_by, _)| *exiled_by != source);
                for card in returning {
                    self.return_exiled_card(card, zone, grant);
                }
            }
            EffectDef::GainControlWhileSourceRemains {
                object: recipient,
                while_tapped,
            } => {
                let source = object.source.unwrap_or(object.id);
                self.take_control_of(
                    recipient,
                    object,
                    context,
                    scoped,
                    Some((source, while_tapped)),
                );
            }
            EffectDef::GainControlThisTurn { object: recipient } => {
                self.take_control_of(recipient, object, context, scoped, None);
            }
            EffectDef::CannotRegenerateThisTurn { object: recipient } => {
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Permanent(id) = target
                        && let Some(permanent) = self
                            .battlefield
                            .iter_mut()
                            .find(|permanent| permanent.card.id == id)
                    {
                        permanent.cannot_regenerate_this_turn = true;
                    }
                }
            }
            EffectDef::MakeUnblockableThisTurn { object: recipient } => {
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Permanent(id) = target
                        && let Some(permanent) = self
                            .battlefield
                            .iter_mut()
                            .find(|permanent| permanent.card.id == id)
                    {
                        permanent.unblockable_this_turn = true;
                    }
                }
            }
            EffectDef::TriggerUntilYourNextTurn { ability } => {
                let DeclarativeAbilityDef::Triggered(definition) = ability.definition else {
                    return;
                };
                let Some(frozen) = object.ability.as_ref() else {
                    return;
                };
                self.floating_triggers.push(FloatingTrigger {
                    event: definition.event,
                    capture: TriggerCapture {
                        source: AbilitySourceRef {
                            object: object.source.unwrap_or(object.id),
                            ability: frozen.origin,
                        },
                        definition: frozen.presentation_definition,
                        owner: object.card.owner,
                        controller: object.controller,
                        text: ability.text,
                        target_defs: definition.targets,
                        effect: ability.effect.definition,
                        resolver: Self::ability_resolver(frozen.origin, ability),
                        context: TriggerContext::empty(),
                        condition: definition.condition,
                    },
                    until_turn_of: object.controller,
                    created_after_turns: self.turns_started[object.controller.index()],
                });
            }
            EffectDef::IfCondition { condition, then } => {
                if self.trigger_condition_holds(
                    condition,
                    object.source.unwrap_or(object.id),
                    object.controller,
                    context,
                    object.ability.as_ref().map(|ability| ability.origin),
                    Some((object, scoped)),
                ) {
                    self.resolve_effect_def(scoped.with_effect(*then), object, context);
                }
            }
            EffectDef::AtNextStep {
                step,
                player,
                effect,
            } => {
                self.delayed_triggers.push(DelayedTrigger {
                    object: Box::new(object.clone()),
                    context,
                    step,
                    player,
                    effect: scoped.with_effect(*effect),
                });
            }
            EffectDef::AddManaEqualTo { color, amount } => {
                let amount = self
                    .effect_value(amount, object, context, scoped)
                    .max(0)
                    .try_into()
                    .unwrap_or(u16::MAX);
                self.add_unrestricted_mana(object.controller, color, amount);
            }
            EffectDef::Counter {
                object: recipient,
                zone,
            } => {
                let zone = if zone == ZoneKind::Exile {
                    CounteredSpellZone::Exile
                } else {
                    CounteredSpellZone::Graveyard
                };
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Spell(spell) = target {
                        self.counter_spell_into(spell, zone);
                    }
                }
            }
            EffectDef::CounterUnlessPaid {
                object: recipient,
                amount,
                zone,
            } => {
                let amount = self
                    .effect_value(amount, object, context, scoped)
                    .max(0)
                    .try_into()
                    .unwrap_or(u16::MAX);
                let zone = if zone == ZoneKind::Exile {
                    CounteredSpellZone::Exile
                } else {
                    CounteredSpellZone::Graveyard
                };
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Spell(spell) = target {
                        self.queue_counter_unless_paid(spell, amount, zone);
                    }
                }
            }
            EffectDef::AddCounters {
                object: recipient,
                kind,
                amount,
            } => {
                let amount = self
                    .effect_value(amount, object, context, scoped)
                    .max(0)
                    .try_into()
                    .unwrap_or(u16::MAX);
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    if let Target::Permanent(permanent) = target
                        && let Some(permanent) = self
                            .battlefield
                            .iter_mut()
                            .find(|candidate| candidate.card.id == permanent)
                    {
                        permanent.add_counters(kind, amount);
                    }
                }
            }
            EffectDef::ChangeTextBasicLandType { object: recipient } => {
                if let Some(target) = self
                    .effect_recipients(recipient, object, context, scoped)
                    .into_iter()
                    .next()
                {
                    self.queue_basic_land_type_text_change(object.controller, target);
                }
            }
            EffectDef::BecomeCopyOf {
                object: recipient,
                retain_source_ability,
            } => {
                let Some(Target::Permanent(target)) = self
                    .effect_recipients(recipient, object, context, scoped)
                    .into_iter()
                    .next()
                else {
                    return;
                };
                let Some(mut copy) = self
                    .battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == target)
                    .map(Self::copiable_characteristics)
                else {
                    return;
                };
                if retain_source_ability
                    && let Some(payload) = &object.ability
                    && let Some(definition) = payload.definition.as_deref()
                {
                    copy.added_abilities.push(CopiableAbility {
                        origin: payload.origin,
                        definition: *definition,
                    });
                }
                if let Some(source) = object.source
                    && let Some(permanent) = self
                        .battlefield
                        .iter_mut()
                        .find(|permanent| permanent.card.id == source)
                {
                    permanent.copy_effect = Some(copy);
                }
            }
            EffectDef::OptionalPayment { payment, if_paid } => {
                let Some(player) = self.payment_player(object.controller, context, payment) else {
                    return;
                };
                let [CostDef::Mana(cost)] = payment.costs else {
                    return;
                };
                self.queue_optional_mana_payment(
                    player,
                    *cost,
                    object,
                    context,
                    scoped.with_effect(*if_paid),
                );
            }
            EffectDef::UnlessPaid { cost, otherwise } => {
                self.queue_mana_payment_or_else(
                    object.controller,
                    cost,
                    object,
                    context,
                    scoped.with_effect(*otherwise),
                );
            }
            EffectDef::Apply {
                recipient,
                effect,
                duration,
            } => self.resolve_applied_effect(recipient, effect, duration, object, context, scoped),
            EffectDef::MoveToZone {
                object: recipient,
                zone,
                controller,
                placement,
            } => {
                let arriving_controller = controller.map(|relation| {
                    if self.player_relation_matches(
                        object.controller,
                        relation,
                        object.controller,
                        context,
                    ) {
                        object.controller
                    } else {
                        object.controller.opponent()
                    }
                });
                for target in self.effect_recipients(recipient, object, context, scoped) {
                    self.move_target_to_zone(
                        target,
                        zone,
                        ZoneMoveCause::Effect {
                            controller: object.controller,
                        },
                        arriving_controller,
                        placement,
                    );
                }
            }
            // An Aura attaches as its spell becomes a permanent, which is
            // handled where the permanent enters rather than here.
            EffectDef::Attach { .. }
            | EffectDef::None
            | EffectDef::Replacement(_)
            | EffectDef::AddMana(AddManaEffectDef {
                mana: ManaSelectionDef::Choice(_),
                ..
            })
            | EffectDef::CannotBeForcedToSacrifice
            | EffectDef::ReduceGenericCostBy(_)
            | EffectDef::PlayersCantPlay(_)
            | EffectDef::LandwalkCanBeBlocked(_)
            | EffectDef::CannotAttackUnless(_)
            | EffectDef::MultiplyEventAmount(_)
            | EffectDef::ChooseCardName { .. }
            | EffectDef::ChoosePlayer { .. }
            | EffectDef::CopyPermanentAsItEnters { .. }
            | EffectDef::ChooseCreatureType { .. }
            | EffectDef::Special(_) => {
                // Choice-bearing mana and the remaining declarative effect
                // families are execution seams until a supported card needs
                // their concrete rules procedure.
            }
        }
    }
}
