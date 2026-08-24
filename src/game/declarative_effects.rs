use super::{
    AbilityProcedureDef, AbilitySourceRef, ArrivalAttachment, BattlefieldArrival,
    BattlefieldExitCompletion, CardPartId, CopiableAbility, CounteredSpellZone,
    DeclarativeAbilityDef, EffectDef, EffectResolutionContext, Game, InstalledTrigger,
    InstalledTriggerLifetime, Permanent, ResolvedOngoingEffect, SacrificeDeclined,
    SacrificeFollowup, ScopedEffect, StackAbilityResolver, StackObject, Target, TriggerCapture,
    ZoneKind, ZoneMoveCause, ZonePlacement,
};
use crate::card::{ArrivalAttachmentDef, InstalledTriggerLifetimeDef};
use move_to_zone::MoveToZoneClause;

mod damage;
mod hand_and_library;
mod linked_exiles;
mod mana;
mod move_to_zone;
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
            EffectDef::Choose(definition) => {
                self.queue_effect_choice(definition, object, context, scoped);
            }
            EffectDef::SimultaneousChoose(definition) => {
                self.queue_simultaneous_choice(definition, object, context, scoped);
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
            EffectDef::SplitIntoPiles(definition) => {
                self.queue_effect_pile_split(definition, object, context, scoped);
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
                for target in self.effect_recipients(recipient, object, &context, scoped) {
                    let available = self.drainable_from(target);
                    let dealt = self.damage_target_from(Some(object.id), Some(target), amount);
                    self.gain_life(object.controller, dealt.min(available));
                }
            }
            EffectDef::DealDamage { recipient, amount } => {
                self.deal_effect_damage(recipient, amount, object, &context, scoped);
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
            EffectDef::GainLife { .. }
            | EffectDef::LoseLife { .. }
            | EffectDef::AddEnergyCounters { .. }
            | EffectDef::AddPoisonCounters { .. }
            | EffectDef::EmptyManaPool { .. }
            | EffectDef::WinTheGame { .. }
            | EffectDef::LoseTheGame { .. } => {
                self.resolve_player_state_effect(scoped, object, &context);
            }
            EffectDef::AddCounters { .. }
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
            | EffectDef::Mill { .. }
            | EffectDef::ExileTopOfLibraryToPlay { .. }
            | EffectDef::ExileAtRandomFromGraveyardToPlay { .. }
            | EffectDef::ExileTopAndMayCast { .. }
            | EffectDef::MillUntil { .. }
            | EffectDef::ExileFromTopUntil { .. }
            | EffectDef::ManifestDread { .. }
            | EffectDef::Cascade
            | EffectDef::LookAtHand { .. }
            | EffectDef::LookAtRandomCardInHand { .. }
            | EffectDef::RevealAtRandomFromHand { .. }
            | EffectDef::RevealHand { .. }
            | EffectDef::LookAtTopAndSelect { .. }
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
            EffectDef::CreateToken { .. }
            | EffectDef::CreateAttachedToken { .. }
            | EffectDef::CreateTokenCopyOf { .. } => {
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
                    // A prohibition on being forced to sacrifice does not
                    // reach an offer the player is free to decline.
                    if !optional && !self.can_be_forced_to_sacrifice(player, object.controller) {
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
                then,
            } => {
                self.put_onto_battlefield_then(recipient, binding, then, object, context, scoped);
            }
            EffectDef::ReturnWithHasteAndFinality {
                object: recipient,
                binding,
                then,
            } => {
                self.return_with_haste_and_finality(
                    recipient, binding, then, object, context, scoped,
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
            EffectDef::GrantFlashToNextSorcery => {
                let grants = &mut self.sorcery_flash_grants[object.controller.index()];
                *grants = grants.saturating_add(1);
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
            | EffectDef::ExileGrantingOwnerPlay { .. }
            | EffectDef::MayPlayWithoutPaying { .. }
            | EffectDef::ReturnLinkedExiles { .. } => {
                self.resolve_linked_exile_effect(scoped, object, &context);
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
            EffectDef::IfCondition { condition, then } => {
                if self.trigger_condition_holds(
                    condition,
                    object.source.unwrap_or(object.id),
                    object.controller,
                    context.trigger,
                    object.ability.as_ref().map(|ability| ability.origin),
                    Some((object, scoped, &context)),
                ) {
                    self.resolve_effect_def(scoped.with_effect(*then), object, context);
                }
            }
            EffectDef::InstallTrigger(installed) => {
                let DeclarativeAbilityDef::Triggered(definition) = installed.ability.definition
                else {
                    return;
                };
                // Installed triggers use the ordinary pending-trigger and
                // stack paths. Declaring fresh targets would require a second
                // target namespace; until that exists they may only retain
                // the installing object's already-chosen target slots.
                if definition.procedure != AbilityProcedureDef::Shared
                    || !definition.targets.is_empty()
                {
                    return;
                }
                let Some(effect) = installed.ability.declarative_effect() else {
                    return;
                };
                let Some(frozen) = object.ability.as_ref() else {
                    return;
                };
                let lifetime = match installed.lifetime {
                    InstalledTriggerLifetimeDef::Once => InstalledTriggerLifetime::Once,
                    InstalledTriggerLifetimeDef::UntilNextTurn(player) => {
                        let Some(player) =
                            self.effect_player_reference(player, object, &context, scoped)
                        else {
                            return;
                        };
                        InstalledTriggerLifetime::UntilTurn {
                            player,
                            turn: self.turns_started[player.index()].saturating_add(1),
                        }
                    }
                };
                let id = self.next_installed_trigger_id;
                self.next_installed_trigger_id = self.next_installed_trigger_id.saturating_add(1);
                self.installed_triggers.push(InstalledTrigger {
                    id,
                    event: definition.event,
                    capture: TriggerCapture {
                        source: AbilitySourceRef {
                            object: object.source.unwrap_or(object.id),
                            ability: frozen.origin,
                        },
                        presentation: frozen.presentation,
                        owner: object.card.owner,
                        controller: object.controller,
                        text: installed.ability.text,
                        // The selections belong to the installing ability's
                        // lexical target namespace. They remain readable by
                        // the nested effect, but the installed ability does
                        // not target them again when it triggers.
                        target_defs: Vec::new(),
                        targets: frozen.targets.clone(),
                        effect,
                        resolver: StackAbilityResolver::Declarative(scoped.with_effect(effect)),
                        context,
                        condition: definition.condition,
                        // An installed trigger carries the effect it was
                        // installed with; nothing about it is modal.
                        modes: None,
                        x: frozen.x,
                    },
                    lifetime,
                });
            }
            EffectDef::BindMatching {
                objects,
                binding,
                then,
            } => {
                let bound = self.effect_objects(objects, object, &context, scoped);
                let mut context = context;
                context.bind_object_group(binding, bound);
                self.resolve_effect_def(scoped.with_effect(*then), object, context);
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
            EffectDef::CopyResolvingSpell { chooser, count } => {
                let copies = self
                    .effect_value(count, object, &context, scoped)
                    .max(0)
                    .try_into()
                    .unwrap_or(u16::MAX);
                // A triggered ability copies the spell it belongs to, which is
                // still on the stack beneath it; a spell copying itself is its
                // own object.
                let original = object.source.unwrap_or(object.id);
                if let Some(player) = self.player_reference(chooser, object, &context, scoped)
                    && let Some(spell) = self.stack.iter().find(|item| item.id == original).cloned()
                    && copies > 0
                {
                    self.queue_copy_decision_chain(player, spell, None, "the copy", copies);
                }
            }
            EffectDef::ReturnSpellToHand { object: recipient } => {
                for target in self.effect_recipients(recipient, object, &context, scoped) {
                    if let Target::Spell(spell) = target {
                        self.return_spell_to_hand(spell);
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
                retain_source_ability,
                added_types,
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
                copy.added_types = copy.added_types.union(added_types);
                if retain_source_ability
                    && let Some(payload) = &object.ability
                    && let Some(definition) = payload.definition.as_deref()
                {
                    copy.added_abilities.push(CopiableAbility {
                        origin: payload.origin,
                        definition: *definition,
                    });
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
            EffectDef::MoveToZone {
                object: recipient,
                zone,
                controller,
                placement,
                arrival_effect,
                attachment,
                counters,
            } => self.resolve_move_to_zone(
                MoveToZoneClause {
                    recipient,
                    zone,
                    controller,
                    placement,
                    arrival_effect,
                    attachment,
                    counters,
                },
                object,
                &context,
                scoped,
            ),
            // An Aura attaches as its spell becomes a permanent, so its own
            // clause has nothing left to do. Equip resolves this instead.
            // An Aura spell attaches itself to what it names; "attach it to
            // this creature" runs the other way, and the source is the host.
            EffectDef::Attach { object: recipient }
            | EffectDef::AttachToSource { object: recipient } => {
                let onto_source = matches!(scoped.effect, EffectDef::AttachToSource { .. });
                let Some(source) = object.source else {
                    return;
                };
                for target in self.effect_recipients(recipient, object, &context, scoped) {
                    let Target::Permanent(id) = target else {
                        continue;
                    };
                    let attached = if onto_source {
                        self.try_attach(id, source)
                    } else {
                        self.try_attach(source, id)
                    };
                    if attached && !onto_source {
                        break;
                    }
                }
            }
            EffectDef::ReturnAttached {
                object: recipient,
                attach_to,
            } => {
                let host = self
                    .effect_recipients(attach_to, object, &context, scoped)
                    .into_iter()
                    .find_map(|target| match target {
                        Target::Permanent(id) => Some(id),
                        _ => None,
                    });
                // An Aura with nothing to enchant never enters (CR 303.4f),
                // so a missing host leaves the card where it is.
                let Some(host) = host else {
                    return;
                };
                for target in self.effect_recipients(recipient, object, &context, scoped) {
                    let arrived = self.move_target_to_zone(
                        target,
                        ZoneKind::Battlefield,
                        ZoneMoveCause::Effect {
                            controller: object.controller,
                        },
                        Some(BattlefieldArrival::under(object.controller)),
                        crate::card::ZonePlacement::Top,
                    );
                    // The card that arrives is a new object, which is the
                    // whole reason this is one effect: an attach written
                    // afterwards would still be naming the old one.
                    if let Some(arrived) = arrived {
                        self.try_attach(arrived, host);
                    }
                }
            }
            EffectDef::PairWithSource { object: recipient } => {
                let Some(source) = object.source else {
                    return;
                };
                let partner = self
                    .effect_recipients(recipient, object, &context, scoped)
                    .into_iter()
                    .find_map(|target| match target {
                        Target::Permanent(id) => Some(id),
                        _ => None,
                    });
                if let Some(partner) = partner {
                    self.pair_creatures(source, partner);
                }
            }
            EffectDef::Reconfigure { object: recipient } => {
                let Some(source) = object.source else {
                    return;
                };
                let host = self
                    .effect_recipients(recipient, object, &context, scoped)
                    .into_iter()
                    .find_map(|target| match target {
                        Target::Permanent(id) => Some(id),
                        _ => None,
                    });
                if let Some(host) = host {
                    self.try_attach(source, host);
                } else {
                    self.unattach(source);
                }
            }
            EffectDef::Unattach { object: recipient } => {
                for target in self.effect_recipients(recipient, object, &context, scoped) {
                    if let Target::Permanent(attachment) = target {
                        self.unattach(attachment);
                    }
                }
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
