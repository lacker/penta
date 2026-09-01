use super::{
    AbilitySourceRef, ArrivalAttachment, BattlefieldArrival, BattlefieldExitCompletion, CardPartId,
    CopiableAbility, CounteredSpellZone, DamageAssignment, DeclarativeAbilityDef, EffectDef,
    EffectResolutionContext, Game, Permanent, ResolvedOngoingEffect, SacrificeDeclined,
    SacrificeFollowup, ScopedEffect, StackAbilityResolver, StackObject, Target, ZoneKind,
    ZoneMoveCause, ZonePlacement,
};
use crate::card::ArrivalAttachmentDef;
mod attachment;
mod bound_outputs;
mod damage;
mod exile_to_play;
mod hand_and_library;
mod installed_triggers;
mod linked_exiles;
mod mana;
mod move_to_zone;
mod object_collections;
mod permanent_state;
mod player_state;
mod prevention;
mod tapping;
mod tokens;

impl Game {
    #[allow(clippy::too_many_lines)]
    pub(super) fn resolve_effect_def(
        &mut self,
        scoped: ScopedEffect,
        object: &StackObject,
        context: impl Into<EffectResolutionContext>,
    ) {
        let context = context.into();
        match scoped.effect {
            EffectDef::Sequence(effects) => {
                self.resolve_effects_in_order(
                    effects
                        .iter()
                        .map(|effect| scoped.with_effect(*effect))
                        .collect(),
                    object,
                    context,
                );
            }
            EffectDef::ContinueReplacedDraw => {
                if let Some(draw) = context.replaced_draw {
                    let _ = self.continue_draw_card(draw.player, draw.applied);
                }
            }
            EffectDef::BindOutput { .. } => {
                self.resolve_bound_output_effect(scoped, object, context);
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
            EffectDef::Choose(definition) => {
                self.queue_effect_choice(definition, object, context, scoped);
            }
            EffectDef::ChooseExact(definition) => {
                self.queue_exact_effect_choice(definition, object, context, scoped);
            }
            EffectDef::ChooseCardsFromCollection(definition) => {
                self.queue_collection_card_choice(definition, object, context, scoped);
            }
            EffectDef::BindObjects(_)
            | EffectDef::IfNoObjects(_)
            | EffectDef::ClassifyObjects(_)
            | EffectDef::RevealAndClassifyCards(_)
            | EffectDef::CombineObjects(_)
            | EffectDef::RandomizeObjectOrder(_)
            | EffectDef::RevealObjects(_)
            | EffectDef::MoveObjects(_)
            | EffectDef::PutObjectsOntoBattlefieldFaceDown(_) => {
                self.resolve_object_collection_effect(scoped, object, context);
            }
            EffectDef::ChooseObjectOrder(definition) => {
                self.queue_choose_object_order(definition, object, context, scoped);
            }
            EffectDef::Mill { .. } => {
                let _ = self.resolve_mill_effect(scoped, object, context);
            }
            EffectDef::MillUntil(_) => {
                let _ = self.resolve_mill_until_effect(scoped, object, context);
            }
            EffectDef::SelectAtRandomFromZone { .. } => {
                let _ = self.resolve_random_zone_selection_effect(scoped, object, context);
            }
            EffectDef::RevealAtRandomFromHand { .. } => {
                let _ = self.resolve_random_hand_reveal_effect(scoped, object, context);
            }
            EffectDef::LookAtObjects(definition) => {
                self.queue_look_at_objects(definition, object, context, scoped);
            }
            EffectDef::PartitionGroup(definition) => {
                self.queue_partition_group(definition, object, context, scoped);
            }
            EffectDef::ChooseGroup(definition) => {
                self.queue_choose_group(definition, object, context, scoped);
            }
            EffectDef::ChooseOneOfEach(definition) => {
                self.queue_choose_one_of_each(definition, object, context, scoped);
            }
            EffectDef::ChooseForEachPlayer(definition) => {
                self.queue_choices_for_each_player(definition, object, context, scoped);
            }
            EffectDef::ForEachInBinding {
                objects,
                binding,
                effect,
            } => {
                self.resolve_for_each_in_binding(
                    objects,
                    binding,
                    0,
                    scoped.with_effect(*effect),
                    object,
                    context,
                );
            }
            EffectDef::PayOr(definition) => {
                // "You may pay {1}{G} if this permanent is attached to a
                // creature you control": asked before the offer, so a false
                // answer takes the other branch rather than offering a
                // payment that would buy nothing.
                if definition.condition.is_some_and(|condition| {
                    !self.trigger_condition_holds(
                        condition,
                        object.source.unwrap_or(object.id),
                        object.controller,
                        context.trigger,
                        object.ability.as_ref().map(|ability| ability.origin),
                        Some((object, scoped, &context)),
                    )
                }) {
                    if let Some(otherwise) = definition.otherwise {
                        self.resolve_effect_def(scoped.with_effect(*otherwise), object, context);
                    }
                    return;
                }
                let payers =
                    self.effect_players(definition.payment.payer, object, &context, scoped);
                let [player] = payers.as_slice() else {
                    if let Some(otherwise) = definition.otherwise {
                        self.resolve_effect_def(scoped.with_effect(*otherwise), object, context);
                    }
                    return;
                };
                let payment =
                    self.resolved_effect_payment(definition.payment.cost, object, &context, scoped);
                self.queue_pay_or(
                    *player,
                    payment,
                    definition.visibility,
                    scoped,
                    object,
                    context,
                    definition.if_paid.map(|effect| scoped.with_effect(*effect)),
                    definition
                        .otherwise
                        .map(|effect| scoped.with_effect(*effect)),
                );
            }
            EffectDef::AddMana(_) | EffectDef::AddManaEqualTo { .. } => {
                self.resolve_mana_effect(scoped, object, &context);
            }
            EffectDef::DrainLife { recipient, amount } => {
                let amount = self
                    .effect_value(amount, object, &context, scoped)
                    .max(0)
                    .try_into()
                    .unwrap_or(u16::MAX);
                let targets = self.effect_recipients(recipient, object, &context, scoped);
                let available = targets
                    .iter()
                    .copied()
                    .map(|target| (target, self.drainable_from(target)))
                    .collect::<Vec<_>>();
                let assignments = targets
                    .into_iter()
                    .map(|target| DamageAssignment {
                        source: Some(object.id),
                        target: Some(target),
                        amount,
                        combat: false,
                    })
                    .collect();
                let gained = self
                    .deal_damage_simultaneously(assignments)
                    .recipients
                    .into_iter()
                    .map(|outcome| {
                        let available = available
                            .iter()
                            .find_map(|(target, amount)| {
                                (*target == outcome.recipient).then_some(*amount)
                            })
                            .unwrap_or(0);
                        outcome.amount.min(available)
                    })
                    .fold(0_u16, u16::saturating_add);
                self.gain_life(object.controller, gained);
            }
            EffectDef::DealDamage { recipient, amount } => {
                self.deal_effect_damage(recipient, amount, object, &context, scoped);
            }
            EffectDef::DealDamageSimultaneously(assignments) => {
                self.deal_simultaneous_effect_damage(assignments, object, &context, scoped);
            }
            EffectDef::DealDamageFrom {
                source,
                recipient,
                amount,
            } => {
                self.deal_effect_damage_from(source, recipient, amount, object, &context, scoped);
            }
            EffectDef::DealDamageAndApply {
                recipient,
                amount,
                applied,
                duration,
            } => {
                let damaged = self.deal_effect_damage(recipient, amount, object, &context, scoped);
                self.apply_effect_to_targets(&damaged, applied, duration, object, &context, scoped);
            }
            EffectDef::Fight {
                first,
                second,
                excess,
            } => {
                self.fight(first, second, excess, object, &context, scoped);
            }
            EffectDef::GainLife { .. }
            | EffectDef::LoseLife { .. }
            | EffectDef::SetLifeTotal { .. }
            | EffectDef::AddPlayerCounters { .. }
            | EffectDef::EmptyManaPool { .. }
            | EffectDef::WinTheGame { .. }
            | EffectDef::LoseTheGame { .. } => {
                self.resolve_player_state_effect(scoped, object, &context);
            }
            EffectDef::AddCounters { .. }
            | EffectDef::ChooseCounterKind { .. }
            | EffectDef::ChooseEffect { .. }
            | EffectDef::ModifyCounters { .. }
            | EffectDef::DoubleCounters { .. }
            | EffectDef::RemoveCounters { .. }
            | EffectDef::RemoveAllCounters { .. }
            | EffectDef::PhaseOut { .. }
            | EffectDef::SubstituteBasicLandTypeUntilEndOfTurn { .. }
            | EffectDef::SkipNextUntapSteps { .. } => {
                self.resolve_permanent_state_effect(scoped, object, &context);
            }
            EffectDef::DrawCards { .. }
            | EffectDef::ShuffleLibrary { .. }
            | EffectDef::BuryGraveyard { .. }
            | EffectDef::Discard { .. }
            | EffectDef::DiscardCards { .. }
            | EffectDef::ExileTopOfLibraryToPlay { .. }
            | EffectDef::ExileOneFromEachZone(_)
            | EffectDef::MillWhileMatching(_)
            | EffectDef::ExileTopAndMayCast { .. }
            | EffectDef::ExileFromTopUntil { .. }
            | EffectDef::Cascade
            | EffectDef::LookAtHand { .. }
            | EffectDef::LookAtRandomCardInHand { .. }
            | EffectDef::RevealHand { .. }
            | EffectDef::SearchZone { .. }
            | EffectDef::ChooseCards { .. }
            | EffectDef::ReplaceNextDrawThisTurn { .. } => {
                self.resolve_hand_and_library_effect(scoped, object, &context);
            }
            EffectDef::Proliferate => self.offer_proliferate(object),
            EffectDef::Explore { object: recipient } => {
                for target in self.effect_recipients(recipient, object, &context, scoped) {
                    if let Target::Permanent(creature) = target {
                        self.explore(creature);
                    }
                }
            }
            EffectDef::Tap { .. } | EffectDef::Untap { .. } | EffectDef::Saddle { .. } => {
                self.resolve_tap_effect(scoped, object, &context);
            }
            EffectDef::RemoveFromCombat { object: recipient } => {
                for target in self.effect_recipients(recipient, object, &context, scoped) {
                    if let Target::Permanent(permanent) = target {
                        self.remove_permanent_from_combat(permanent);
                    }
                }
            }
            EffectDef::Regenerate { object: recipient } => {
                for target in self.effect_recipients(recipient, object, &context, scoped) {
                    if let Target::Permanent(permanent) = target {
                        self.add_regeneration_shield(permanent);
                    }
                }
            }
            EffectDef::CreateToken { .. } | EffectDef::CreateAttachedToken { .. } => {
                self.resolve_token_effect(scoped, object, &context);
            }
            EffectDef::PreventDamage { .. } => {
                self.resolve_prevention_effect(scoped, object, &context);
            }
            EffectDef::Destroy {
                object: recipient,
                can_regenerate,
                then,
            } => {
                let permanents = self
                    .effect_recipients(recipient, object, &context, scoped)
                    .into_iter()
                    .filter_map(|target| match target {
                        Target::Permanent(permanent) => Some(permanent),
                        Target::Player(_) | Target::Card(_) | Target::Spell(_) => None,
                    })
                    .collect::<Vec<_>>();
                let completion = then.map(|follow_up| BattlefieldExitCompletion::DestroyFollowup {
                    candidates: permanents.clone(),
                    binding: follow_up.binding,
                    object: Box::new(object.clone()),
                    context,
                    effect: scoped.with_effect(*follow_up.effect),
                });
                self.destroy_permanents_then(&permanents, can_regenerate, completion);
            }
            // Audit: whoever controls a named permanent is treated as the
            // player sacrificing it, which is right for "each player
            // sacrifices" and wrong for "sacrifice it". CR 701.17a says a
            // player cannot sacrifice what they do not control, so a Sneak
            // Attack creature an opponent has taken should survive the end
            // step -- "you sacrifice the creature only if you still control
            // it" -- and here it does not. Fixing it needs the effect to
            // carry whose sacrifice it is; the recipient shape alone cannot
            // tell a delayed "sacrifice it" from a board-wide one.
            EffectDef::Sacrifice { object: recipient } => {
                let permanents = self
                    .effect_recipients(recipient, object, &context, scoped)
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
                self.sacrifice_permanents(&permanents);
            }
            EffectDef::SacrificeOfChoice {
                count,
                amount: sacrificed_amount,
                player: recipient,
                object: predicate,
                then,
                otherwise,
                optional,
            } => {
                let source = object.source.unwrap_or(object.id);
                for target in self.effect_recipients(recipient, object, &context, scoped) {
                    let Target::Player(player) = target else {
                        continue;
                    };
                    // A prohibition on being made to sacrifice reaches an
                    // offer as readily as an order: "if that spell or ability
                    // gives you the option to sacrifice a permanent, you
                    // can't take that option" (Tamiyo, Collector of Tales).
                    // Nothing was sacrificed, so what is left is the branch
                    // for having sacrificed nothing.
                    if !self.can_be_forced_to_sacrifice(player, object.controller) {
                        if let Some(effect) = otherwise {
                            self.resolve_effect_def(
                                scoped.with_effect(*effect),
                                object,
                                context.clone(),
                            );
                        }
                        continue;
                    }
                    let followup = then.map(|effect| SacrificeFollowup {
                        amount: sacrificed_amount,
                        object: Box::new(object.clone()),
                        context: context.clone(),
                        effect: scoped.with_effect(*effect),
                    });
                    let declined = otherwise.map(|effect| SacrificeDeclined {
                        object: Box::new(object.clone()),
                        context: context.clone(),
                        effect: scoped.with_effect(*effect),
                    });
                    let count =
                        usize::try_from(self.effect_value(count, object, &context, scoped).max(0))
                            .unwrap_or(0);
                    self.queue_chosen_sacrifice(
                        player, predicate, count, source, followup, declined, optional,
                    );
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
                let card = self.unbacked_emblem_object(emblem, controller);
                let mut emblem = Permanent::entering(
                    card,
                    CardPartId::PRIMARY,
                    controller,
                    self.turns_started[controller.index()],
                    self.turn,
                );
                emblem.timestamp = self.allocate_continuous_effect_timestamp();
                emblem.emblem_source = object.ability_origin();
                self.emblems.push(emblem);
            }
            EffectDef::CreateOngoingEffect(ongoing) => {
                let Some(frozen) = object.ability.as_ref() else {
                    return;
                };
                let expiration = Self::continuous_effect_expiration(
                    ongoing.duration,
                    object.controller,
                    self.turns_started[object.controller.index()],
                );
                let affected = ongoing.affected.map_or_else(
                    || vec![None],
                    |recipient| {
                        self.effect_recipients(recipient, object, &context, scoped)
                            .into_iter()
                            .map(Some)
                            .collect()
                    },
                );
                for affected in affected {
                    let mut frozen_context = context.clone();
                    if let (Some(binding), Some(affected)) = (ongoing.binding, affected) {
                        frozen_context.bind_single_object(binding, Some(affected));
                    }
                    let effect_object = self.allocate_object_id();
                    self.ongoing_effects.push(ResolvedOngoingEffect {
                        source: AbilitySourceRef {
                            object: effect_object,
                            // The nested ability is structurally located
                            // beneath the resolving clause, so its root
                            // provenance remains the clause that created it.
                            ability: frozen.origin,
                        },
                        owner: object.card.owner,
                        controller: object.controller,
                        presentation: frozen.presentation,
                        ability: *ongoing.ability,
                        context: frozen_context,
                        expiration,
                    });
                }
            }
            EffectDef::PutOntoBattlefieldThen {
                object: recipient,
                binding,
                counters,
                then,
            } => {
                self.put_onto_battlefield_then(
                    recipient, binding, counters, then, object, context, scoped,
                );
            }
            EffectDef::Transform { object: recipient } => {
                for target in self.effect_recipients(recipient, object, &context, scoped) {
                    if let Target::Permanent(id) = target {
                        self.transform_permanent(id);
                    }
                }
            }
            EffectDef::ScheduleTurnPhases(phases) => {
                self.schedule_turn_phases(phases);
            }
            EffectDef::TakeExtraTurn { player: recipient } => {
                let players = self
                    .effect_recipients(recipient, object, &context, scoped)
                    .into_iter()
                    .filter_map(|target| match target {
                        Target::Player(player) => Some(player),
                        Target::Card(_) | Target::Permanent(_) | Target::Spell(_) => None,
                    });
                self.schedule_extra_turns(players);
            }
            EffectDef::SearchZonesAndExileRest {
                player: recipient,
                zones,
                count,
            } => {
                for target in self.effect_recipients(recipient, object, &context, scoped) {
                    if let Target::Player(player) = target {
                        self.queue_search_zones_and_exile_rest(player, zones, usize::from(count));
                    }
                }
            }
            EffectDef::PutIntoLibraryBeneathTop {
                object: recipient,
                depth,
            } => {
                let depth = self
                    .effect_value(depth, object, &context, scoped)
                    .max(0)
                    .try_into()
                    .unwrap_or(usize::MAX);
                for target in self.effect_recipients(recipient, object, &context, scoped) {
                    let (Target::Permanent(id) | Target::Card(id)) = target else {
                        continue;
                    };
                    let _ = self.move_target_to_zone(
                        target,
                        ZoneKind::Library,
                        ZoneMoveCause::Effect {
                            controller: object.controller,
                        },
                        None,
                        ZonePlacement::Top,
                    );
                    // Leaving the battlefield makes a new object, so the card
                    // now sitting on top of the library is the successor
                    // rather than the permanent that was targeted.
                    let moved = self.successors.get(&id).copied().unwrap_or(id);
                    self.sink_library_card(moved, depth);
                }
            }
            EffectDef::PutSourceOntoBattlefieldAttacking => {
                self.put_ninja_onto_the_battlefield(object);
            }
            EffectDef::VoteForPermanentToExile { object: predicate } => {
                self.queue_permanent_vote(
                    object.controller,
                    predicate,
                    object.source.unwrap_or(object.id),
                );
            }
            EffectDef::GainClassLevel { level } => {
                self.raise_class_level(object.source.unwrap_or(object.id), level);
            }
            EffectDef::BecomeMonarch { player } => {
                if let Some(player) = self.player_reference(player, object, &context, scoped) {
                    self.set_monarch(player);
                }
            }
            EffectDef::DamageCannotBePreventedThisTurn => {
                self.damage_cannot_be_prevented_this_turn = true;
            }
            EffectDef::Endure {
                object: recipient,
                amount,
            } => {
                let amount =
                    u16::try_from(self.effect_value(amount, object, &context, scoped).max(0))
                        .unwrap_or(u16::MAX);
                for target in self.effect_recipients(recipient, object, &context, scoped) {
                    if let Target::Permanent(permanent) = target {
                        self.queue_endure(object.controller, permanent, amount);
                    }
                }
            }
            EffectDef::May {
                player: recipient,
                effect,
            } => {
                for target in self.effect_recipients(recipient, object, &context, scoped) {
                    if let Target::Player(player) = target {
                        self.queue_optional_effect(
                            player,
                            object,
                            context.clone(),
                            scoped.with_effect(*effect),
                        );
                    }
                }
            }
            EffectDef::MayCastTargetWithoutPaying {
                object: recipient,
                ability,
            } => {
                if let Some(Target::Card(card)) = self
                    .effect_recipients(recipient, object, &context, scoped)
                    .into_iter()
                    .next()
                {
                    self.offer_granted_cast(object.controller, card, ability);
                }
            }
            EffectDef::ExileLinkedToSource { .. }
            | EffectDef::PermitLookAtExiled { .. }
            | EffectDef::ExileGrantingOwnerPlay { .. }
            | EffectDef::ExileGrantingControllerPlayThisTurn { .. }
            | EffectDef::MayPlayWithoutPaying { .. }
            | EffectDef::ReturnLinkedExiles { .. } => {
                self.resolve_linked_exile_effect(scoped, object, &context);
            }
            EffectDef::PermitCastFromGraveyardThisTurn { object: recipient } => {
                let controller = object.controller;
                for target in self.effect_recipients(recipient, object, &context, scoped) {
                    if let Target::Card(card) = target {
                        self.permit_graveyard_cast_this_turn(card, controller);
                    }
                }
            }
            EffectDef::Detain { object: recipient } => {
                let controller = object.controller;
                let created = self.turns_started[controller.index()];
                for target in self.effect_recipients(recipient, object, &context, scoped) {
                    if let Target::Permanent(id) = target
                        && let Some(permanent) = self
                            .battlefield
                            .iter_mut()
                            .find(|permanent| permanent.card.id == id)
                    {
                        permanent.detained_until_turn_of = Some((controller, created));
                    }
                }
            }
            EffectDef::GainControl {
                object: recipient,
                controller,
                duration,
            } => {
                // Who receives it is read where the effect resolves, so
                // "an opponent gains control" hands it to the one player
                // the effect's controller is not.
                let Some(receiver) = self.player_reference(controller, object, &context, scoped)
                else {
                    return;
                };
                self.take_control_of(recipient, object, &context, scoped, duration, receiver);
            }
            EffectDef::ExchangeControl {
                first,
                second,
                otherwise,
            } => {
                if !self.exchange_control_of(first, second, object, &context, scoped)
                    && let Some(otherwise) = otherwise
                {
                    self.resolve_effect_def(scoped.with_effect(*otherwise), object, context);
                }
            }
            effect @ (EffectDef::IfCondition { .. } | EffectDef::IfElseCondition { .. }) => {
                let conditional = effect
                    .conditional()
                    .expect("conditional variants expose their shared shape");
                let condition_holds = self.trigger_condition_holds(
                    conditional.condition,
                    object.source.unwrap_or(object.id),
                    object.controller,
                    context.trigger,
                    object.ability.as_ref().map(|ability| ability.origin),
                    Some((object, scoped, &context)),
                );
                if let Some(branch) = conditional.branch(condition_holds) {
                    self.resolve_effect_def(scoped.with_effect(*branch), object, context);
                }
            }
            EffectDef::InstallTrigger(installed) => {
                let Some(source_ability) = object.ability.as_ref().map(|frozen| frozen.origin)
                else {
                    return;
                };
                self.install_trigger_from(installed, scoped, object, context, source_ability);
            }
            EffectDef::ChooseCardName {
                chooser,
                nonland_only,
                matched_in,
                zone,
                binding,
                then,
            } => {
                if let Some(player) = self.player_reference(chooser, object, &context, scoped)
                    && let Some(searched) =
                        self.player_reference(matched_in, object, &context, scoped)
                {
                    self.queue_card_name_choice(
                        player,
                        nonland_only,
                        searched,
                        zone,
                        binding,
                        object.clone(),
                        context,
                        scoped.with_effect(*then),
                    );
                }
            }
            EffectDef::CopyStackObject(copy) => {
                let copies = self
                    .effect_value(copy.count, object, &context, scoped)
                    .max(0)
                    .try_into()
                    .unwrap_or(u16::MAX);
                let Some(player) = self.player_reference(copy.controller, object, &context, scoped)
                else {
                    return;
                };
                for target in self.effect_recipients(copy.object, object, &context, scoped) {
                    if let Target::Spell(id) = target
                        && let Some(stack_object) = (id == object.id)
                            .then(|| object.clone())
                            .or_else(|| self.stack.iter().find(|item| item.id == id).cloned())
                            // A spell that has left the stack is copied from
                            // its last known information: countering a storm
                            // spell in response to its own trigger does not
                            // take the copies with it.
                            .or_else(|| self.retired_stack_object(id))
                        && copies > 0
                    {
                        self.queue_copy_decision_chain(
                            player,
                            stack_object,
                            copy.colors,
                            copy.retarget,
                            "the copy",
                            copies,
                        );
                    }
                }
            }
            EffectDef::ChangeStackTargets(change) => {
                let Some(chooser) = self.player_reference(change.chooser, object, &context, scoped)
                else {
                    return;
                };
                let replacement = match change.change {
                    crate::card::StackTargetChangeDef::ChooseNew { .. } => None,
                    crate::card::StackTargetChangeDef::ReplaceOneWith(recipient) => self
                        .effect_recipients(recipient, object, &context, scoped)
                        .into_iter()
                        .next(),
                };
                for target in self.effect_recipients(change.object, object, &context, scoped) {
                    if let Target::Spell(id) = target
                        && let Some(stack_object) = self
                            .stack
                            .iter()
                            .find(|candidate| candidate.id == id)
                            .cloned()
                    {
                        self.queue_stack_target_change(
                            chooser,
                            &stack_object,
                            change.change,
                            replacement,
                        );
                    }
                }
            }
            EffectDef::PutSpellIntoOwnersLibrary { object: recipient } => {
                for target in self.effect_recipients(recipient, object, &context, scoped) {
                    if let Target::Spell(spell) = target {
                        self.queue_spell_library_end_choice(spell);
                    }
                }
            }
            EffectDef::Counter {
                object: recipient,
                zone,
                placement,
            } => {
                let zone = match zone {
                    ZoneKind::Exile => CounteredSpellZone::Exile,
                    ZoneKind::Hand => CounteredSpellZone::Hand,
                    ZoneKind::Library => CounteredSpellZone::Library(placement),
                    _ => CounteredSpellZone::Graveyard,
                };
                for target in self.effect_recipients(recipient, object, &context, scoped) {
                    if let Target::Spell(spell) = target {
                        self.counter_spell_into(spell, zone);
                    }
                }
            }
            EffectDef::ChooseColor {
                object: recipient,
                operation,
                duration,
            } => {
                // Resolved before the question is asked: targets are already
                // chosen, and a group is whatever it is at this moment.
                let targets = self.effect_recipients(recipient, object, &context, scoped);
                if !targets.is_empty() {
                    self.queue_color_choice(
                        object.controller,
                        Box::new(object.clone()),
                        context.clone(),
                        scoped,
                        targets,
                        operation,
                        duration,
                    );
                }
            }
            EffectDef::ChangeTextBasicLandType { object: recipient } => {
                if let Some(target) = self
                    .effect_recipients(recipient, object, &context, scoped)
                    .into_iter()
                    .next()
                {
                    self.queue_basic_land_type_text_change(object.controller, target);
                }
            }
            EffectDef::BecomeCopyOf {
                object: recipient,
                copier,
                exceptions,
                duration,
            } => {
                let Some(target) = self
                    .effect_recipients(recipient, object, &context, scoped)
                    .into_iter()
                    .next()
                else {
                    return;
                };
                // "This becomes a copy" names nothing, so the source is what
                // becomes one; a clause that points elsewhere says so.
                let becomes = match copier {
                    None => object.source,
                    Some(copier) => match self
                        .effect_recipients(copier, object, &context, scoped)
                        .into_iter()
                        .next()
                    {
                        Some(Target::Permanent(id)) => Some(id),
                        _ => None,
                    },
                };
                let Some(mut copy) = self.copiable_values_of(target) else {
                    return;
                };
                if let Some(stats) = exceptions.base_power_toughness {
                    copy.base_power_toughness = Some(stats);
                }
                if let Some(colors) = exceptions.colors {
                    copy.colors = Some(colors);
                }
                copy.added_creature_types
                    .extend(exceptions.added_creature_types.named);
                copy.added_types = copy.added_types.union(exceptions.added_types);
                copy.no_mana_cost |= exceptions.no_mana_cost;
                if let Some(payload) = &object.ability {
                    copy.added_abilities
                        .extend(exceptions.added_abilities.iter().filter_map(|added| {
                            Some(CopiableAbility {
                                origin: payload.origin,
                                definition: match added {
                                    crate::card::CopyAbilityDef::This => {
                                        *payload.definition.as_deref()?
                                    }
                                    crate::card::CopyAbilityDef::Ability(ability) => **ability,
                                },
                            })
                        }));
                }
                let expiration = duration.map(|duration| {
                    Self::continuous_effect_expiration(
                        duration,
                        object.controller,
                        self.turns_started[object.controller.index()],
                    )
                });
                if let Some(becomes) = becomes
                    && let Some(permanent) = self
                        .battlefield
                        .iter_mut()
                        .find(|permanent| permanent.card.id == becomes)
                {
                    permanent.copy_effect = Some(copy);
                    permanent.copy_expiration = expiration;
                }
            }
            EffectDef::Apply {
                recipient,
                effect,
                duration,
            } => self.resolve_applied_effect(recipient, effect, duration, object, &context, scoped),
            effect @ (EffectDef::MoveToZone { .. } | EffectDef::WithBattlefieldArrival { .. }) => {
                self.resolve_move_to_zone_effect(effect, object, &context, scoped);
            }
            EffectDef::WithZoneMoveResult {
                effect,
                binding,
                then,
            } => self.resolve_zone_move_result(effect, binding, then, object, context, scoped),
            EffectDef::Attach { object: recipient } => {
                self.resolve_attach_effect(recipient, false, object, &context, scoped);
            }
            EffectDef::AttachToSource { object: recipient } => {
                self.resolve_attach_effect(recipient, true, object, &context, scoped);
            }
            EffectDef::PairWithSource { object: recipient } => {
                self.resolve_pair_with_source(recipient, object, &context, scoped);
            }
            EffectDef::Reconfigure { object: recipient } => {
                self.resolve_reconfigure_effect(recipient, object, &context, scoped);
            }
            EffectDef::Unattach { object: recipient } => {
                self.resolve_unattach_effect(recipient, object, &context, scoped);
            }
            EffectDef::None
            | EffectDef::CreateMyriadTokens
            | EffectDef::CannotBeForcedToSacrifice
            | EffectDef::CannotBeForcedToDiscard
            | EffectDef::ReduceGenericCostBy(_)
            | EffectDef::ModifyCost(_)
            | EffectDef::LandwalkCanBeBlocked(_)
            | EffectDef::CannotAttackUnless(_)
            | EffectDef::CannotAttackIf(_)
            | EffectDef::ConditionalStatic(_)
            | EffectDef::StaticApply { .. }
            | EffectDef::Special(_) => {
                // Myriad performs no mutation in a two-player game because
                // there is no opponent other than the defending player.
                // The remaining entries are execution seams until a
                // supported card needs their concrete rules procedure.
            }
        }
    }

    pub(super) fn resolve_for_each_in_binding(
        &mut self,
        objects: crate::ObjectSetBindingIndex,
        binding: crate::ObjectBindingIndex,
        mut next: usize,
        effect: ScopedEffect,
        object: &StackObject,
        context: EffectResolutionContext,
    ) {
        let members = context.object_group(objects).to_vec();
        let mut later_procedures = std::mem::take(&mut self.pending_procedures);
        while let Some(member) = members.get(next).copied() {
            next += 1;
            let mut iteration = context.clone();
            iteration.bind_single_object(binding, Some(member));
            self.resolve_effect_def(effect, object, iteration);
            if !self.pending_decisions.is_empty()
                || !self.pending_events.is_empty()
                || !self.pending_procedures.is_empty()
            {
                if next < members.len() {
                    self.pending_procedures
                        .push_back(super::PendingProcedure::ForEachInBinding {
                            objects,
                            binding,
                            next,
                            effect,
                            object: Box::new(object.clone()),
                            context,
                        });
                }
                self.pending_procedures.append(&mut later_procedures);
                return;
            }
        }
        self.pending_procedures.append(&mut later_procedures);
    }
}
