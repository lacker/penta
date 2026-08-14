use super::{
    AbilityDef, AbilityEffectExpiration, AbilityId, AbilityOrigin, AbilityTargetPredicate,
    AppliedEffectDef, CardPartId, CastSignature, ComparisonDef, ContinuousEffectTimestamp,
    ControlFlow, CounterKind, EffectDurationDef, EffectRecipientDef, Game, GameObjectId, GrantId,
    ObjectPredicateDef, ObjectQueryDef, Permanent, PlayerId, QuantifierDef, ResolvedAnimation,
    ScopedEffect, StackObject, StackObjectKind, Target, TargetIndex, TargetSelection, TargetSlotId,
    TemporaryAbilityGrant, TemporaryGrantedAbility, TemporaryRemovedAbilities, TriggerConditionDef,
    TriggerContext, ZoneKind,
};

mod recipients;

#[derive(Clone, Copy)]
struct ResolvedAppliedEffect<'a> {
    duration: EffectDurationDef,
    timestamp: ContinuousEffectTimestamp,
    object: &'a StackObject,
    context: TriggerContext,
    scoped: ScopedEffect,
}

impl Game {
    pub(super) fn resolve_applied_effect(
        &mut self,
        recipient: EffectRecipientDef,
        effect: AppliedEffectDef,
        duration: EffectDurationDef,
        object: &StackObject,
        context: TriggerContext,
        scoped: ScopedEffect,
    ) {
        let timestamp = self.allocate_continuous_effect_timestamp();
        let resolution = ResolvedAppliedEffect {
            duration,
            timestamp,
            object,
            context,
            scoped,
        };
        for target in self.effect_recipients(recipient, object, context, scoped) {
            self.apply_applied_effect_component(target, effect, resolution);
        }
        // Ability additions and removals can start or end an attachment's
        // layer-2 control effect. Re-derive it before a later instruction in
        // the same resolving sequence observes a controller.
        self.reconcile_all_control_layers();
        // Everything else lasts until cleanup. Keeping the duration explicit
        // here makes unsupported permanent/granted effects visible rather
        // than silently changing their lifetime.
        debug_assert!(matches!(
            duration,
            EffectDurationDef::UntilEndOfTurn
                | EffectDurationDef::Permanent
                | EffectDurationDef::UntilYourNextUpkeep
                | EffectDurationDef::UntilYourNextTurn
        ));
    }

    /// Where a granted ability lands: the supported nonbattlefield flashback
    /// case keeps its cleanup-bounded card grant, while a permanent records an
    /// ordered, duration-aware layer operation for every ability category.
    pub(super) fn apply_granted_ability(
        &mut self,
        target: Target,
        ability: &'static AbilityDef,
        duration: EffectDurationDef,
        timestamp: ContinuousEffectTimestamp,
        object: &StackObject,
    ) {
        match target {
            Target::Card(target) => {
                let grant = TemporaryAbilityGrant {
                    object: target,
                    ability: *ability,
                };
                if self.card_in_nonbattlefield_zone(target).is_some()
                    && !self.temporary_ability_grants.contains(&grant)
                {
                    self.temporary_ability_grants.push(grant);
                }
            }
            Target::Permanent(target) => {
                let source = object.source.unwrap_or(object.id);
                let origin = object.ability_origin().unwrap_or(AbilityOrigin::Printed {
                    definition: object.presentation_definition(),
                    part: CardPartId::PRIMARY,
                    ability: AbilityId::PRIMARY,
                });
                let (source_definition, source_part, source_ability) =
                    Self::ability_origin_components(origin, object.presentation_definition());
                let expiration = Self::ability_effect_expiration(
                    duration,
                    object.controller,
                    self.turns_started[object.controller.index()],
                );
                if let Some(permanent) = self
                    .battlefield
                    .iter_mut()
                    .find(|permanent| permanent.card.id == target)
                {
                    let order = u16::try_from(
                        permanent.temporary_granted_abilities.len()
                            + permanent.temporary_removed_abilities.len(),
                    )
                    .expect("one resolved effect creates at most 65,536 ability operations");
                    let grant = GrantId::from_index(permanent.temporary_granted_abilities.len())
                        .expect("one permanent has at most 256 resolved grants");
                    permanent
                        .temporary_granted_abilities
                        .push(TemporaryGrantedAbility {
                            ability: *ability,
                            source,
                            source_definition,
                            source_part,
                            source_ability,
                            grant,
                            timestamp,
                            order,
                            expiration,
                        });
                }
            }
            Target::Player(_) | Target::Spell(_) => {}
        }
    }

    pub(super) fn ability_effect_expiration(
        duration: EffectDurationDef,
        controller: PlayerId,
        turns_started: u32,
    ) -> AbilityEffectExpiration {
        match duration {
            EffectDurationDef::UntilEndOfTurn => AbilityEffectExpiration::EndOfTurn,
            EffectDurationDef::UntilYourNextUpkeep => AbilityEffectExpiration::UpkeepOf(controller),
            EffectDurationDef::UntilYourNextTurn => AbilityEffectExpiration::TurnOf {
                player: controller,
                turn: turns_started.saturating_add(1),
            },
            EffectDurationDef::Permanent => AbilityEffectExpiration::Never,
            EffectDurationDef::WhileSourceRemainsInZone
            | EffectDurationDef::UntilSourceLeavesZone => {
                unreachable!("a resolving effect cannot have a static duration")
            }
        }
    }

    /// The removal half of a resolved ability-layer operation, kept beside
    /// the dispatch rather than inside it.
    fn apply_removed_abilities(
        &mut self,
        target: Target,
        predicate: crate::card::AbilityPredicateDef,
        resolution: ResolvedAppliedEffect<'_>,
    ) {
        let Target::Permanent(target) = target else {
            return;
        };
        let expiration = Self::ability_effect_expiration(
            resolution.duration,
            resolution.object.controller,
            self.turns_started[resolution.object.controller.index()],
        );
        if let Some(permanent) = self
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == target)
        {
            let order = u16::try_from(
                permanent.temporary_granted_abilities.len()
                    + permanent.temporary_removed_abilities.len(),
            )
            .expect("one resolved effect creates at most 65,536 ability operations");
            permanent
                .temporary_removed_abilities
                .push(TemporaryRemovedAbilities {
                    predicate,
                    timestamp: resolution.timestamp,
                    order,
                    expiration,
                });
        }
    }

    fn apply_applied_effect_component(
        &mut self,
        target: Target,
        effect: AppliedEffectDef,
        resolution: ResolvedAppliedEffect<'_>,
    ) {
        match effect {
            AppliedEffectDef::Composite(effects) => {
                for effect in effects {
                    self.apply_applied_effect_component(target, *effect, resolution);
                }
            }
            AppliedEffectDef::GrantAbility(ability) => {
                self.apply_granted_ability(
                    target,
                    ability,
                    resolution.duration,
                    resolution.timestamp,
                    resolution.object,
                );
            }
            AppliedEffectDef::RemoveAbilities(predicate) => {
                self.apply_removed_abilities(target, predicate, resolution);
            }
            // A resolved prohibition is recorded on the permanent, the way
            // the other until-end-of-turn combat riders are; the printed
            // static form is read from the continuous layer instead.
            AppliedEffectDef::CannotBlock => {
                if let Target::Permanent(target) = target
                    && let Some(permanent) = self
                        .battlefield
                        .iter_mut()
                        .find(|permanent| permanent.card.id == target)
                {
                    permanent.cannot_block_this_turn = true;
                }
            }
            AppliedEffectDef::Animate(animation) => {
                if let Target::Permanent(target) = target
                    && let Some(permanent) = self
                        .battlefield
                        .iter_mut()
                        .find(|permanent| permanent.card.id == target)
                {
                    // A second animation overwrites the first. Its timestamp
                    // still orders the retained operation against dynamic
                    // attachment forms in layer 4.
                    permanent.animation = Some(ResolvedAnimation {
                        definition: animation,
                        timestamp: resolution.timestamp,
                    });
                }
            }
            AppliedEffectDef::ModifyPowerToughness { power, toughness } => {
                let Target::Permanent(target) = target else {
                    return;
                };
                let power = i16::try_from(
                    self.effect_value(
                        power,
                        resolution.object,
                        resolution.context,
                        resolution.scoped,
                    )
                    .clamp(i32::from(i16::MIN), i32::from(i16::MAX)),
                )
                .expect("the effect value was clamped to i16");
                let toughness = i16::try_from(
                    self.effect_value(
                        toughness,
                        resolution.object,
                        resolution.context,
                        resolution.scoped,
                    )
                    .clamp(i32::from(i16::MIN), i32::from(i16::MAX)),
                )
                .expect("the effect value was clamped to i16");
                if let Some(permanent) = self
                    .battlefield
                    .iter_mut()
                    .find(|permanent| permanent.card.id == target)
                {
                    permanent.power_bonus = permanent.power_bonus.saturating_add(power);
                    permanent.toughness_bonus = permanent.toughness_bonus.saturating_add(toughness);
                }
            }
            // Only the printed static form of "can't be blocked" exists; a
            // resolving one already has its own effect.
            AppliedEffectDef::CannotBeBlocked
            | AppliedEffectDef::CannotBeCountered
            | AppliedEffectDef::DoesNotUntapDuringUntapStep
            | AppliedEffectDef::MayChooseNotToUntap
            | AppliedEffectDef::CannotBeEnchanted
            | AppliedEffectDef::CannotBecomeEnchanted
            | AppliedEffectDef::CannotChangeController
            | AppliedEffectDef::RemainsAttachedThroughProtection
            | AppliedEffectDef::ControlBySourceController
            | AppliedEffectDef::CannotBeBlockedBy(_)
            | AppliedEffectDef::PreventDamageFrom(_)
            | AppliedEffectDef::PreventCombatDamage
            | AppliedEffectDef::AddLandTypes(_)
            | AppliedEffectDef::SetLandTypes(_)
            | AppliedEffectDef::Special(_) => {}
        }
    }

    pub(super) fn live_object_target(&self, object: GameObjectId) -> Option<Target> {
        if self
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == object)
        {
            return Some(Target::Permanent(object));
        }
        if self.stack.iter().any(|candidate| candidate.id == object) {
            return Some(Target::Spell(object));
        }
        self.card_in_nonbattlefield_zone(object)
            .is_some()
            .then_some(Target::Card(object))
    }

    /// The battlefield permanents a target-relative sweep names. Control and
    /// ownership pick out different sets the moment anything has changed
    /// hands: a stolen artifact goes home to its owner, not to whoever is
    /// holding it.
    pub(super) fn battlefield_sweep_for_target(
        &self,
        recipient: EffectRecipientDef,
        object: &StackObject,
        context: TriggerContext,
        scoped: ScopedEffect,
    ) -> Vec<Target> {
        let (predicate, player_source, by_owner) = match recipient {
            EffectRecipientDef::ObjectsControlledByTarget { object, slot } => {
                (object, EffectRecipientDef::ControllerOfTarget(slot), false)
            }
            EffectRecipientDef::ObjectsOwnedByTarget { object, slot } => {
                (object, EffectRecipientDef::Target(slot), true)
            }
            _ => return Vec::new(),
        };
        let Some(Target::Player(player)) = self
            .effect_recipients(player_source, object, context, scoped)
            .into_iter()
            .next()
        else {
            return Vec::new();
        };
        let source = object.source.unwrap_or(object.id);
        self.battlefield
            .iter()
            .filter(|permanent| {
                player
                    == if by_owner {
                        permanent.card.owner
                    } else {
                        permanent.controller
                    }
            })
            .filter(|permanent| {
                self.trigger_object_matches(
                    predicate,
                    &self.trigger_event_object(permanent),
                    source,
                    false,
                )
            })
            .map(|permanent| Target::Permanent(permanent.card.id))
            .collect()
    }

    fn cards_owned_by_target(
        &self,
        predicate: ObjectPredicateDef,
        zones: &[ZoneKind],
        slot: TargetIndex,
        object: &StackObject,
        scoped: ScopedEffect,
    ) -> Vec<Target> {
        let slot = scoped.target_slot(slot);
        let Some(Target::Player(player)) = Self::chosen_targets(object, slot)
            .find(|target| self.stack_ability_target_is_legal(object, slot, *target))
        else {
            return Vec::new();
        };
        let source = object.source.unwrap_or(object.id);
        zones
            .iter()
            .copied()
            .filter(|zone| {
                matches!(
                    zone,
                    ZoneKind::Library | ZoneKind::Hand | ZoneKind::Graveyard | ZoneKind::Exile
                )
            })
            .flat_map(|zone| {
                self.cards_in_zone(zone).filter_map(move |card| {
                    (card.owner == player
                        && self.card_object_matches(predicate, card, zone, source))
                    .then_some(Target::Card(card.id))
                })
            })
            .collect()
    }

    pub(super) fn effect_recipients(
        &self,
        recipient: EffectRecipientDef,
        object: &StackObject,
        context: TriggerContext,
        scoped: ScopedEffect,
    ) -> Vec<Target> {
        if let EffectRecipientDef::Target(target) = recipient {
            let slot = scoped.target_slot(target);
            return Self::chosen_targets(object, slot)
                .filter(|target| self.stack_ability_target_is_legal(object, slot, *target))
                .collect();
        }

        if let EffectRecipientDef::ChosenPermanent(choice) = recipient {
            return context
                .chosen_object(choice)
                .map(Target::Permanent)
                .into_iter()
                .collect();
        }

        // "Its controller" is read after the rest of the effect has already
        // run, by which point the target is often gone -- Ghost Quarter
        // destroys the land before its owner searches. So this reads the
        // chosen target without the legality filter and falls back to
        // last-known information.
        if let EffectRecipientDef::ControllerOfTarget(target) = recipient {
            let slot = scoped.target_slot(target);
            return Self::chosen_targets(object, slot)
                .find_map(|target| match target {
                    Target::Permanent(id) | Target::Card(id) | Target::Spell(id) => {
                        self.current_or_last_known_controller(id)
                    }
                    Target::Player(player) => Some(player),
                })
                .map(Target::Player)
                .into_iter()
                .collect();
        }

        // "Each creature that player controls" and "all artifacts target
        // player owns" both read a player off a target slot and then sweep the
        // battlefield, so neither is a plain target nor a relation to the
        // ability's own controller.
        if matches!(
            recipient,
            EffectRecipientDef::ObjectsControlledByTarget { .. }
                | EffectRecipientDef::ObjectsOwnedByTarget { .. }
        ) {
            return self.battlefield_sweep_for_target(recipient, object, context, scoped);
        }

        if let EffectRecipientDef::CardsOwnedByTarget {
            object: predicate,
            zones,
            slot,
        } = recipient
        {
            return self.cards_owned_by_target(predicate, zones, slot, object, scoped);
        }

        if let EffectRecipientDef::ObjectsSharingNameWithTarget(target) = recipient {
            return self.objects_sharing_name_with_target(scoped.target_slot(target), object);
        }

        if recipient == EffectRecipientDef::EachPlayer {
            return vec![
                Target::Player(object.controller),
                Target::Player(object.controller.opponent()),
            ];
        }

        let EffectRecipientDef::MatchingObjects {
            object: predicate,
            zones,
            controller,
        } = recipient
        else {
            return self.direct_effect_recipients(recipient, object, context);
        };

        self.objects_matching_query(
            ObjectQueryDef {
                object: predicate,
                zones,
                controller,
            },
            object.controller,
            object.source.unwrap_or(object.id),
            context,
        )
    }

    /// Whether a trigger's intervening-if condition holds right now. Rule
    /// 603.4 asks this when the ability would trigger and again as it
    /// resolves, so both call sites read the same board.
    /// How many times this ability has been activated from this permanent so
    /// far this turn.
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

    #[allow(clippy::too_many_lines)]
    pub(super) fn trigger_condition_holds(
        &self,
        condition: &TriggerConditionDef,
        source: GameObjectId,
        controller: PlayerId,
        context: TriggerContext,
        ability: Option<AbilityOrigin>,
        object: Option<(&StackObject, ScopedEffect)>,
    ) -> bool {
        let TriggerConditionDef::ObjectCount {
            query,
            comparison,
            amount,
        } = condition
        else {
            return match condition {
                TriggerConditionDef::SourceOnBattlefield => self
                    .battlefield
                    .iter()
                    .any(|permanent| permanent.card.id == source),
                TriggerConditionDef::SourceUntapped => self
                    .battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == source)
                    .is_some_and(|permanent| !permanent.tapped),
                TriggerConditionDef::ActivePlayer(relation) => {
                    self.player_relation_matches(self.active_player, *relation, controller, context)
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
                // Read now rather than when the ability was created, so a
                // delayed effect asks about the target as it is at that point.
                TriggerConditionDef::TargetMatches {
                    slot,
                    object: predicate,
                } => object.is_some_and(|(stack, scoped)| {
                    Self::chosen_targets(stack, scoped.target_slot(*slot)).any(|target| {
                        matches!(target, Target::Permanent(id)
                        if self
                            .battlefield
                            .iter()
                            .find(|permanent| permanent.card.id == id)
                            .is_some_and(|permanent| {
                                self.trigger_object_matches(
                                    *predicate,
                                    &self.trigger_event_object(permanent),
                                    source,
                                    false,
                                )
                            }))
                    })
                }),
                TriggerConditionDef::SourceDealtDamageToOpponentThisTurn => self
                    .battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == source)
                    .is_some_and(|permanent| permanent.dealt_damage_to_opponent_this_turn),
                TriggerConditionDef::SourceIsTapped => self.current_or_last_known_tapped(source),
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
            |_| {
                count += 1;
                ControlFlow::Continue(())
            },
        );
        debug_assert!(result.is_continue());
        compare(&i64::from(count), *comparison, &i64::from(*amount))
    }

    /// How much of a divided total one target takes, read off the selection
    /// frozen when the object was put on the stack.
    pub(super) fn divided_share(
        object: &StackObject,
        slot: TargetSlotId,
        target: Target,
    ) -> Option<u16> {
        object
            .signature
            .as_ref()
            .map(CastSignature::targets)
            .or_else(|| {
                object
                    .ability
                    .as_ref()
                    .map(|ability| ability.targets.as_slice())
            })?
            .iter()
            .find(|selection| selection.slot() == slot)?
            .amount_for(target)
    }

    /// The targets frozen into one slot when the object was put on the stack,
    /// before any legality check.
    pub(super) fn chosen_targets(
        object: &StackObject,
        slot: TargetSlotId,
    ) -> impl Iterator<Item = Target> {
        object
            .signature
            .as_ref()
            .map(CastSignature::targets)
            .or_else(|| {
                object
                    .ability
                    .as_ref()
                    .map(|ability| ability.targets.as_slice())
            })
            .and_then(|selections| selections.iter().find(|selection| selection.slot() == slot))
            .into_iter()
            .flat_map(TargetSelection::targets)
            .copied()
    }

    /// Finds objects using only zone, relation, and effective-characteristic
    /// predicates. Unlike target enumeration, this does not apply hexproof,
    /// protection, or any other targeting restriction.
    pub(super) fn objects_matching_query(
        &self,
        query: ObjectQueryDef,
        evaluation_controller: PlayerId,
        source: GameObjectId,
        context: TriggerContext,
    ) -> Vec<Target> {
        self.objects_matching_query_with_prospective(
            query,
            evaluation_controller,
            source,
            context,
            None,
        )
    }

    pub(super) fn objects_matching_query_with_prospective(
        &self,
        query: ObjectQueryDef,
        evaluation_controller: PlayerId,
        source: GameObjectId,
        context: TriggerContext,
        prospective: Option<&Permanent>,
    ) -> Vec<Target> {
        let mut recipients = Vec::new();
        let result = self.visit_objects_matching_query_with_prospective(
            query,
            evaluation_controller,
            source,
            context,
            prospective,
            |recipient| {
                recipients.push(recipient);
                ControlFlow::Continue(())
            },
        );
        debug_assert!(result.is_continue());
        recipients
    }

    pub(super) fn any_object_matches_query_with_prospective(
        &self,
        query: ObjectQueryDef,
        evaluation_controller: PlayerId,
        source: GameObjectId,
        context: TriggerContext,
        prospective: Option<&Permanent>,
    ) -> bool {
        self.visit_objects_matching_query_with_prospective(
            query,
            evaluation_controller,
            source,
            context,
            prospective,
            |_| ControlFlow::Break(()),
        )
        .is_break()
    }

    pub(super) fn visit_objects_matching_query_with_prospective(
        &self,
        query: ObjectQueryDef,
        evaluation_controller: PlayerId,
        source: GameObjectId,
        context: TriggerContext,
        prospective: Option<&Permanent>,
        mut visitor: impl FnMut(Target) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        if query.zones.contains(&ZoneKind::Battlefield) {
            for permanent in &self.battlefield {
                if !self.player_relation_matches(
                    permanent.controller,
                    query.controller,
                    evaluation_controller,
                    context,
                ) {
                    continue;
                }
                let characteristics = prospective.map_or_else(
                    || self.trigger_event_object(permanent),
                    |prospective| {
                        self.trigger_event_object_with_prospective(permanent, prospective)
                    },
                );
                if self.trigger_object_matches(query.object, &characteristics, source, false)
                    && visitor(Target::Permanent(permanent.card.id)).is_break()
                {
                    return ControlFlow::Break(());
                }
            }
        }
        if query.zones.contains(&ZoneKind::Stack) {
            for candidate in self.stack.iter() {
                if candidate.kind != StackObjectKind::Spell
                    || !self.player_relation_matches(
                        candidate.controller,
                        query.controller,
                        evaluation_controller,
                        context,
                    )
                {
                    continue;
                }
                let Some(characteristics) = self.stack_trigger_event_object(candidate) else {
                    continue;
                };
                if self.trigger_object_matches(query.object, &characteristics, source, true)
                    && visitor(Target::Spell(candidate.id)).is_break()
                {
                    return ControlFlow::Break(());
                }
            }
        }
        // The same card zones the target enumerator understands. Without this
        // a sweep over graveyards matched nothing and the clause was inert.
        for zone in [
            ZoneKind::Library,
            ZoneKind::Hand,
            ZoneKind::Graveyard,
            ZoneKind::Exile,
            ZoneKind::Command,
        ] {
            if !query.zones.contains(&zone) {
                continue;
            }
            for card in self.cards_in_zone(zone) {
                if self.player_relation_matches(
                    card.owner,
                    query.controller,
                    evaluation_controller,
                    context,
                ) && self.card_object_matches(query.object, card, zone, source)
                    && visitor(Target::Card(card.id)).is_break()
                {
                    return ControlFlow::Break(());
                }
            }
        }
        ControlFlow::Continue(())
    }

    pub(super) fn stack_ability_target_is_legal(
        &self,
        object: &StackObject,
        slot: TargetSlotId,
        target: Target,
    ) -> bool {
        let source = object.source.unwrap_or(object.id);
        let Some(ability) = &object.ability else {
            return true;
        };
        let Some(definition) = ability.target_defs.get(slot.index()) else {
            // Legacy custom actions can carry targets without a declarative
            // target slot. Their historic resolver remains authoritative.
            return true;
        };
        if Self::ability_target_uses_custom_predicate(definition.predicate) {
            // Custom activated handlers offered these targets before the
            // shared predicate vocabulary could express their full legality.
            // Preserve their prior zone-presence check until the named
            // predicate itself is migrated; treating `Special` as no matches
            // would incorrectly counter every such ability on resolution.
            return match target {
                Target::Player(_) => true,
                Target::Card(id) => self.card_in_nonbattlefield_zone(id).is_some(),
                Target::Permanent(id) => self
                    .battlefield
                    .iter()
                    .any(|permanent| permanent.card.id == id),
                Target::Spell(id) => self.stack.iter().any(|candidate| candidate.id == id),
            };
        }
        self.ability_targets_matching(
            definition.predicate,
            object.controller,
            source,
            ability.context,
        )
        .contains(&target)
    }

    pub(super) fn ability_target_uses_custom_predicate(predicate: AbilityTargetPredicate) -> bool {
        match predicate {
            AbilityTargetPredicate::AnyTarget
            | AbilityTargetPredicate::PlayerOrPlaneswalker(_)
            | AbilityTargetPredicate::ControlledByTargetOf { .. }
            | AbilityTargetPredicate::Player(_) => false,
            AbilityTargetPredicate::Object { object, .. } => {
                Self::object_predicate_uses_custom_predicate(object)
            }
        }
    }

    pub(super) fn object_predicate_uses_custom_predicate(predicate: ObjectPredicateDef) -> bool {
        match predicate {
            ObjectPredicateDef::Special(_) => true,
            ObjectPredicateDef::All(predicates) | ObjectPredicateDef::AnyOf(predicates) => {
                predicates
                    .iter()
                    .any(|predicate| Self::object_predicate_uses_custom_predicate(*predicate))
            }
            ObjectPredicateDef::Not(predicate) => {
                Self::object_predicate_uses_custom_predicate(*predicate)
            }
            ObjectPredicateDef::Any
            | ObjectPredicateDef::Source
            | ObjectPredicateDef::AttachedToSource
            | ObjectPredicateDef::Token
            | ObjectPredicateDef::HasType(_)
            | ObjectPredicateDef::HasAnyBasicLandType(_)
            | ObjectPredicateDef::Spell
            | ObjectPredicateDef::NoncreatureSpell
            | ObjectPredicateDef::Color(_)
            | ObjectPredicateDef::ColorCount(_)
            | ObjectPredicateDef::Subtype(_)
            | ObjectPredicateDef::ManaValueAtMost(_)
            | ObjectPredicateDef::ManaValueEqualTo(_)
            | ObjectPredicateDef::ManaValueAtMostValue(_)
            | ObjectPredicateDef::PowerAtLeast(_)
            | ObjectPredicateDef::PowerExactly(_)
            | ObjectPredicateDef::ToughnessExactly(_)
            | ObjectPredicateDef::ToughnessLessThan(_)
            | ObjectPredicateDef::PowerGreaterThan(_)
            | ObjectPredicateDef::ToughnessGreaterThan(_)
            | ObjectPredicateDef::ControlledBy(_)
            | ObjectPredicateDef::Supertype(_)
            | ObjectPredicateDef::DebutSet(_)
            | ObjectPredicateDef::SharesNameWithSource
            | ObjectPredicateDef::AttackingOrBlocking
            | ObjectPredicateDef::Tapped
            | ObjectPredicateDef::Attacking
            | ObjectPredicateDef::Blocking
            | ObjectPredicateDef::AttackedThisTurn
            | ObjectPredicateDef::HasKeyword(_)
            | ObjectPredicateDef::HasNonManaActivatedAbility => false,
        }
    }

    pub(super) fn first_legal_ability_target(&self, object: &StackObject) -> Option<Target> {
        object.ability.as_ref().and_then(|ability| {
            ability.targets.iter().find_map(|selection| {
                selection.targets().iter().copied().find(|target| {
                    self.stack_ability_target_is_legal(object, selection.slot(), *target)
                })
            })
        })
    }
}

/// One comparison, so a condition reads the same however it is counted.
pub(super) fn compare<T: Ord>(left: &T, comparison: ComparisonDef, right: &T) -> bool {
    match comparison {
        ComparisonDef::Less => left < right,
        ComparisonDef::LessOrEqual => left <= right,
        ComparisonDef::Equal => left == right,
        ComparisonDef::GreaterOrEqual => left >= right,
        ComparisonDef::Greater => left > right,
    }
}

#[cfg(test)]
mod tests {
    use super::compare;
    use crate::ComparisonDef;

    #[test]
    fn comparisons_follow_their_ordering_semantics() {
        assert!(compare(&1, ComparisonDef::Less, &2));
        assert!(compare(&2, ComparisonDef::LessOrEqual, &2));
        assert!(compare(&2, ComparisonDef::Equal, &2));
        assert!(compare(&2, ComparisonDef::GreaterOrEqual, &2));
        assert!(compare(&3, ComparisonDef::Greater, &2));

        assert!(!compare(&2, ComparisonDef::Less, &2));
        assert!(!compare(&3, ComparisonDef::LessOrEqual, &2));
        assert!(!compare(&3, ComparisonDef::Equal, &2));
        assert!(!compare(&1, ComparisonDef::GreaterOrEqual, &2));
        assert!(!compare(&2, ComparisonDef::Greater, &2));
    }
}
