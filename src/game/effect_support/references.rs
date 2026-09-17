// Resolving the object and player references an effect names, from the
// resolving object's own context: target slots, bindings, player sets, and
// the recipients and object sets those add up to.
//
// Split out of `effect_support.rs` only to keep one file readable; these are
// ordinary members of the same `impl Game`. The paths and imports are the
// parent module's.

impl Game {
    pub(super) fn static_object_reference(
        &self,
        reference: ObjectRefDef,
        effect_source: GameObjectId,
    ) -> Option<GameObjectId> {
        match reference {
            ObjectRefDef::Source | ObjectRefDef::ResolvingObject => Some(effect_source),
            ObjectRefDef::AttachedToSource => {
                self.current_or_last_known_attached_host(effect_source)
            }
            ObjectRefDef::CreatingSource => self.creating_source_of(effect_source),
            ObjectRefDef::AbilityGrantSource
            | ObjectRefDef::ZoneChangeSuccessor(_)
            | ObjectRefDef::ZoneChangeResultOfTriggeringObject
            | ObjectRefDef::Binding(_)
            | ObjectRefDef::AdditionalCostObject(_)
            | ObjectRefDef::Target(_)
            | ObjectRefDef::SourceOfTargetedStackObject(_)
            | ObjectRefDef::TriggeringObject
            | ObjectRefDef::DamagedObject => None,
        }
    }

    fn raw_target_reference(
        slot: TargetIndex,
        object: &StackObject,
        scoped: ScopedEffect,
    ) -> Option<Target> {
        Self::chosen_targets(object, scoped.target_slot(slot)).next()
    }

    pub(in crate::game) fn object_reference_target(
        &self,
        reference: ObjectRefDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> Option<Target> {
        match reference {
            // The source is the exact game object from which the spell or
            // ability originated. If it has left its zone, keep naming that
            // retired incarnation for last-known-information reads; never
            // silently substitute the new object it became.
            ObjectRefDef::Source => object
                .source
                .and_then(|source| self.object_target_with_lki(source)),
            ObjectRefDef::CreatingSource => object
                .source
                .and_then(|source| self.creating_source_of(source))
                .and_then(|creator| self.live_object_target(creator)),
            ObjectRefDef::ZoneChangeSuccessor(reference) => self
                .object_reference_id(reference.exact(), object, context, scoped)
                .and_then(|referenced| self.zone_change_successor_target(referenced)),
            ObjectRefDef::ZoneChangeResultOfTriggeringObject => context
                .trigger
                .zone_change_result
                .and_then(|result| self.live_object_target(result)),
            // A granted ability freezes the exact object that supplied the
            // grant. Do not follow a zone-change successor here: this is the
            // last-known permanent the ability names even after sacrificing
            // it as a cost.
            ObjectRefDef::AbilityGrantSource => {
                object.ability_origin().and_then(|origin| match origin {
                    crate::AbilityOrigin::Granted { source, .. }
                    | crate::AbilityOrigin::TokenGranted { source, .. }
                    | crate::AbilityOrigin::EmblemGranted { source, .. } => {
                        Some(Target::Permanent(source))
                    }
                    _ => None,
                })
            }
            // The resolver has already removed this object from the live
            // stack, but effects such as the Chain cycle still name and copy
            // the spell that is currently resolving.
            ObjectRefDef::ResolvingObject => Some(Target::Spell(object.id)),
            ObjectRefDef::AdditionalCostObject(index) => object
                .chosen_permanents
                .get(index.index())
                .copied()
                .and_then(|paid| self.object_target_with_lki(paid)),
            ObjectRefDef::SourceOfTargetedStackObject(target) => self
                .targeted_stack_object_source(target, object, scoped)
                .map(Target::Permanent),
            ObjectRefDef::Binding(binding) => context.single_object(binding),
            ObjectRefDef::AttachedToSource => object
                .source
                .and_then(|source| self.current_or_last_known_attached_host(source))
                .map(|host| self.attached_host_target(host)),
            ObjectRefDef::Target(target) => {
                let slot = scoped.target_slot(target);
                Self::raw_target_reference(target, object, scoped)
                    .filter(|target| !matches!(target, Target::Player(_)))
                    .filter(|target| self.stack_ability_target_is_legal(object, slot, *target))
            }
            ObjectRefDef::TriggeringObject => context
                .trigger
                .object
                .and_then(|triggering| self.object_target_with_lki(triggering)),
            ObjectRefDef::DamagedObject => context
                .trigger
                .damaged_object
                .and_then(|damaged| self.object_target_with_lki(damaged)),
        }
    }

    /// What one printed payment actually costs, worked out where it is
    /// asked.
    ///
    /// Shared by the three places that ask -- a resolving `PayOr`, an entry
    /// replacement's "unless you pay", and the checkpoint that rebuilds a
    /// standing payment decision -- because all three are the same question
    /// about the same clause, and a cost that answered differently in one of
    /// them would be a different card there.
    pub(in crate::game) fn resolved_effect_costs(
        &self,
        costs: &[crate::CostDef],
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> crate::game::ResolvedEffectPayment {
        crate::game::ResolvedEffectPayment::all(
            costs
                .iter()
                .map(|cost| self.resolved_effect_payment(*cost, object, context, scoped))
                .collect(),
        )
    }

    pub(in crate::game) fn resolved_effect_payment(
        &self,
        cost: crate::card::CostDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> crate::game::ResolvedEffectPayment {
        use crate::card::CostDef as Cost;
        use crate::game::ResolvedEffectPayment as Resolved;
        let amount_of = |value| {
            u16::try_from(self.effect_value(value, object, context, scoped).max(0))
                .unwrap_or(u16::MAX)
        };
        match cost {
            Cost::Perform(program) => {
                self.resolve_action_payment(*program, object, context, scoped, 1)
            }
            Cost::All(costs) => self.resolved_effect_costs(costs, object, context, scoped),
            Cost::Parameter | Cost::Repeated { .. } | Cost::Choice(_) => {
                self.resolved_program_cost(cost, object, context, scoped, None, None)
            }
            Cost::Mana(cost) => Resolved::Mana(cost),
            Cost::GenericMana(amount) => Resolved::Mana(crate::ManaCost::new(amount_of(amount), 0)),
            Cost::ColoredMana { color, amount } => {
                Resolved::Mana(crate::ManaCost::of_color(color, amount_of(amount)))
            }
            Cost::ObjectManaCostReducedBy {
                object: recipient,
                generic,
            } => Resolved::Mana(
                self.object_mana_cost_reduced_by(*recipient, generic, object, context, scoped),
            ),
            Cost::PayLife(amount) => Resolved::Life(amount),
            Cost::Life(amount) => Resolved::Life(amount_of(amount)),
            Cost::Energy(amount) => Resolved::Energy(amount),
            Cost::MillCards(amount) => Resolved::Mill(amount),
            Cost::DiscardCards(amount) => Resolved::Discard(amount),
            Cost::SacrificePermanent {
                object: predicate,
                controller: crate::card::PlayerRelation::You,
            } => Resolved::SacrificePermanentMatching(predicate),
            Cost::SacrificeCreaturesWithTotalPower(total) => {
                Resolved::SacrificeCreaturesWithTotalPower(total)
            }
            Cost::MovePermanentMatching { object, zone } => {
                Resolved::MovePermanentMatching { object, zone }
            }
            Cost::ChosenGenericMana => Resolved::ChosenGenericMana,
            Cost::ChosenEnergy => Resolved::ChosenEnergy,
            Cost::RemoveAnyNumberOfCounters {
                object: recipient,
                kind,
            } => self
                .effect_recipients(*recipient, object, context, scoped)
                .into_iter()
                .find_map(|target| match target {
                    Target::Permanent(object) => {
                        Some(Resolved::RemoveAnyNumberOfCounters { object, kind })
                    }
                    Target::Player(_) | Target::Card(_) | Target::Spell(_) => None,
                })
                .unwrap_or(Resolved::RemoveAnyNumberOfCounters {
                    object: crate::GameObjectId(0),
                    kind,
                }),
            Cost::Discard {
                object: predicate,
                quantity: crate::card::CostQuantityDef::Fixed(1),
            } => Resolved::DiscardMatching(predicate),
            _ => unreachable!("unsupported resolving payment cost reached execution"),
        }
    }

    /// "Its mana cost reduced by {N}": the printed cost of whatever the
    /// reference names, less that much generic. Coloured pips stand: what
    /// the reduction touches is the generic part and nothing else.
    pub(in crate::game) fn object_mana_cost_reduced_by(
        &self,
        recipient: crate::card::EffectRecipientDef,
        generic: u16,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> crate::ManaCost {
        let Some(Target::Permanent(id)) = self
            .effect_recipients(recipient, object, context, scoped)
            .into_iter()
            .next()
        else {
            return crate::ManaCost::default();
        };
        let Some(mut cost) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .and_then(|permanent| self.effective_rules(permanent))
            .and_then(|rules| rules.mana_cost())
        else {
            return crate::ManaCost::default();
        };
        cost.generic = cost.generic.saturating_sub(generic);
        cost
    }

    pub(super) fn object_reference_id(
        &self,
        reference: ObjectRefDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> Option<GameObjectId> {
        match reference {
            ObjectRefDef::Source => object.source,
            ObjectRefDef::CreatingSource => object
                .source
                .and_then(|source| self.creating_source_of(source)),
            ObjectRefDef::ZoneChangeSuccessor(reference) => self
                .object_reference_id(reference.exact(), object, context, scoped)
                .and_then(|referenced| self.zone_change_successor_target(referenced))
                .and_then(|target| match target {
                    Target::Card(id) | Target::Permanent(id) | Target::Spell(id) => Some(id),
                    Target::Player(_) => None,
                }),
            // Preserve the exact event result even after this resolution has
            // moved it again. An enclosing ZoneChangeSuccessor reference
            // needs the retired identity in order to follow that one new
            // transition edge; a direct target lookup still requires it live.
            ObjectRefDef::ZoneChangeResultOfTriggeringObject => context.trigger.zone_change_result,
            ObjectRefDef::AbilityGrantSource => {
                object.ability_origin().and_then(|origin| match origin {
                    crate::AbilityOrigin::Granted { source, .. }
                    | crate::AbilityOrigin::TokenGranted { source, .. }
                    | crate::AbilityOrigin::EmblemGranted { source, .. } => Some(source),
                    _ => None,
                })
            }
            ObjectRefDef::ResolvingObject => Some(object.id),
            ObjectRefDef::AdditionalCostObject(index) => {
                object.chosen_permanents.get(index.index()).copied()
            }
            ObjectRefDef::Binding(binding) => {
                context
                    .single_object(binding)
                    .and_then(|target| match target {
                        Target::Card(id) | Target::Permanent(id) | Target::Spell(id) => Some(id),
                        Target::Player(_) => None,
                    })
            }
            ObjectRefDef::AttachedToSource => object
                .source
                .and_then(|source| self.current_or_last_known_attached_host(source)),
            ObjectRefDef::Target(target) => {
                let slot = scoped.target_slot(target);
                Self::raw_target_reference(target, object, scoped)
                    .filter(|target| self.stack_ability_target_is_legal(object, slot, *target))
                    .and_then(|target| match target {
                        Target::Card(id) | Target::Permanent(id) | Target::Spell(id) => Some(id),
                        Target::Player(_) => None,
                    })
            }
            ObjectRefDef::SourceOfTargetedStackObject(target) => {
                self.targeted_stack_object_source(target, object, scoped)
            }
            ObjectRefDef::TriggeringObject => context.trigger.object,
            ObjectRefDef::DamagedObject => context.trigger.damaged_object,
        }
    }

    /// The permanent a targeted stack ability came from. Read after the
    /// ability has left the stack -- which is when "destroy that permanent"
    /// asks -- so the retired record is the one that answers, and a targeted
    /// spell has no such source at all.
    fn targeted_stack_object_source(
        &self,
        target: crate::TargetIndex,
        object: &StackObject,
        scoped: ScopedEffect,
    ) -> Option<GameObjectId> {
        let Some(Target::Spell(id)) = Self::raw_target_reference(target, object, scoped) else {
            return None;
        };
        let source = self
            .stack
            .iter()
            .find(|candidate| candidate.id == id)
            .map_or_else(
                || self.retired_stack_object_source(id),
                |stack| stack.source,
            )?;
        self.battlefield
            .iter()
            .any(|permanent| permanent.card.id == source)
            .then_some(source)
    }

    pub(in crate::game) fn player_reference(
        &self,
        reference: PlayerRefDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> Option<PlayerId> {
        match reference {
            PlayerRefDef::CastBinding(binding) => self
                .cast_context_for(object.source.unwrap_or(object.id), Some(object))
                .and_then(|cast| cast.player_bindings.get(binding.label()?).copied()),
            PlayerRefDef::EffectController => Some(object.controller),
            PlayerRefDef::EnchantedPlayer => object
                .source
                .and_then(|source| self.current_or_last_known_enchanted_player(source)),
            PlayerRefDef::Opponent => Some(object.controller.opponent()),
            PlayerRefDef::EventPlayer => context.trigger.event_player,
            PlayerRefDef::Target(target) => {
                let slot = scoped.target_slot(target);
                Self::chosen_targets(object, slot)
                    .find(|target| self.stack_ability_target_is_legal(object, slot, *target))
                    .and_then(|target| match target {
                        Target::Player(player) => Some(player),
                        Target::Card(_) | Target::Permanent(_) | Target::Spell(_) => None,
                    })
            }
            // A direct object recipient still checks whether its target is
            // legal. Derived identity is different: a later instruction in
            // the same resolving effect may ask who controlled or owned an
            // object that an earlier instruction already moved. Preserve the
            // announced target here and answer from last-known information.
            PlayerRefDef::ControllerOf(ObjectRefDef::Target(target)) => {
                Self::raw_target_reference(target, object, scoped).and_then(|target| match target {
                    Target::Player(player) => Some(player),
                    Target::Card(id) | Target::Permanent(id) | Target::Spell(id) => {
                        self.current_or_last_known_controller(id)
                    }
                })
            }
            PlayerRefDef::OwnerOf(ObjectRefDef::Target(target)) => {
                Self::raw_target_reference(target, object, scoped).and_then(|target| match target {
                    Target::Card(id) | Target::Permanent(id) | Target::Spell(id) => {
                        self.current_or_last_known_owner(id)
                    }
                    Target::Player(_) => None,
                })
            }
            PlayerRefDef::ControllerOf(ObjectRefDef::TriggeringObject) => context
                .trigger
                .object
                .and_then(|triggering| self.current_or_last_known_controller(triggering))
                .or(context.trigger.object_controller),
            PlayerRefDef::ControllerOf(reference) => self
                .object_reference_id(reference, object, context, scoped)
                .and_then(|referenced| self.current_or_last_known_controller(referenced)),
            PlayerRefDef::OwnerOf(reference) => self
                .object_reference_id(reference, object, context, scoped)
                .and_then(|referenced| self.current_or_last_known_owner(referenced)),
            // "Each player other than its controller", which with two
            // players is the one who is not.
            PlayerRefDef::OpponentOf(reference) => self
                .player_reference(
                    PlayerRefDef::ControllerOf(reference),
                    object,
                    context,
                    scoped,
                )
                .map(PlayerId::opponent),
        }
    }

    fn players_in_set(
        &self,
        players: PlayerSetDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> Vec<PlayerId> {
        match players {
            PlayerSetDef::All => vec![object.controller, object.controller.opponent()],
            PlayerSetDef::One(reference) => self
                .player_reference(reference, object, context, scoped)
                .into_iter()
                .collect(),
            PlayerSetDef::LegalTargets(target) => {
                let slot = scoped.target_slot(target);
                Self::chosen_targets(object, slot)
                    .filter(|target| self.stack_ability_target_is_legal(object, slot, *target))
                    .filter_map(|target| match target {
                        Target::Player(player) => Some(player),
                        Target::Card(_) | Target::Permanent(_) | Target::Spell(_) => None,
                    })
                    .collect()
            }
            PlayerSetDef::Related(relation) => [object.controller, object.controller.opponent()]
                .into_iter()
                .filter(|candidate| {
                    self.player_relation_matches_for_source(
                        *candidate,
                        relation,
                        object.controller,
                        object.source.unwrap_or(object.id),
                        context.trigger,
                    )
                })
                .collect(),
        }
    }

    pub(super) fn effect_object_reference_id(
        &self,
        reference: ObjectRefDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> Option<GameObjectId> {
        self.object_reference_id(reference, object, context, scoped)
    }

    pub(super) fn effect_player_reference(
        &self,
        reference: PlayerRefDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> Option<PlayerId> {
        self.player_reference(reference, object, context, scoped)
    }

    pub(super) fn effect_players(
        &self,
        players: PlayerSetDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> Vec<PlayerId> {
        self.players_in_set(players, object, context, scoped)
    }

    /// What an attacking permanent is attacking, as a target: the defending
    /// player, or the planeswalker the attack was declared against (CR
    /// 506.3b). Nothing when it is not attacking, or has already left.
    fn attacked_defender_target(&self, attacker: GameObjectId) -> Option<Target> {
        // Last known information when the attacker has left (CR 608.2h):
        // Myr Battlesphere's own ruling is that a Battlesphere answered in
        // response still throws its Myr at what it was attacking, and only
        // the +X/+0 half is lost with the body.
        let defender = self.attack_defender_of(attacker).or_else(|| {
            match self.retired_objects.get(&attacker) {
                Some(crate::game::RetiredObject::Permanent { permanent, .. }) => {
                    permanent.attack_defender
                }
                _ => None,
            }
        });
        match defender? {
            crate::AttackDefender::Player(player) => Some(Target::Player(player)),
            crate::AttackDefender::Planeswalker(planeswalker) => {
                Some(Target::Permanent(planeswalker))
            }
        }
    }

    /// What an Aura's host is, as a target. Almost always a permanent; an
    /// Aura enchanting a card in a graveyard has a card instead, and
    /// "return enchanted creature card" has to be able to name it. A host
    /// that is nowhere at all is still reported as a permanent, which is
    /// what every last-known-information reader of this expects.
    fn attached_host_target(&self, host: GameObjectId) -> Target {
        if self
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == host)
        {
            return Target::Permanent(host);
        }
        if self.card_in_nonbattlefield_zone(host).is_some() {
            return Target::Card(host);
        }
        Target::Permanent(host)
    }

    pub(super) fn effect_recipients(
        &self,
        recipient: EffectRecipientDef,
        object: &StackObject,
        context: &EffectResolutionContext,
        scoped: ScopedEffect,
    ) -> Vec<Target> {
        match recipient.0 {
            EffectRecipientSetDef::LegalTargets(target) => {
                let slot = scoped.target_slot(target);
                Self::chosen_targets(object, slot)
                    .filter(|target| self.stack_ability_target_is_legal(object, slot, *target))
                    .collect()
            }
            EffectRecipientSetDef::Objects(objects) => {
                self.effect_objects(objects, object, context, scoped)
            }
            // The declaration decides which kind this is: a player, or the
            // planeswalker standing in front of them. An attacker that has
            // left combat -- or left the battlefield -- names nothing.
            EffectRecipientSetDef::DefenderOf(reference) => self
                .effect_object_reference_id(reference, object, context, scoped)
                .and_then(|attacker| self.attacked_defender_target(attacker))
                .into_iter()
                .collect(),
            EffectRecipientSetDef::Players(players) => self
                .players_in_set(players, object, context, scoped)
                .into_iter()
                .map(Target::Player)
                .collect(),
            // "Each opponent and each creature they control": both kinds in
            // one list, players first, which is the order the clause reads.
            EffectRecipientSetDef::PlayersAndCreaturesTheyControl(players) => {
                let players = self.players_in_set(players, object, context, scoped);
                let mut recipients = players
                    .iter()
                    .copied()
                    .map(Target::Player)
                    .collect::<Vec<_>>();
                recipients.extend(
                    self.battlefield
                        .iter()
                        .filter(|permanent| players.contains(&permanent.controller))
                        .filter(|permanent| {
                            self.permanent_types(permanent).is_some_and(|types| {
                                types.contains(crate::card::CardType::Creature)
                            })
                        })
                        .map(|permanent| Target::Permanent(permanent.card.id)),
                );
                recipients
            }
        }
    }

    /// Whether one member of a binding matches a predicate, wherever it is.
    /// A binding can hold battlefield permanents as readily as cards in a
    /// graveyard, so both are looked for.
    pub(super) fn bound_object_matches(
        &self,
        bound: Target,
        predicate: ObjectPredicateDef,
        source: GameObjectId,
    ) -> bool {
        let id = match bound {
            Target::Card(id) | Target::Permanent(id) | Target::Spell(id) => id,
            Target::Player(_) => return false,
        };
        if let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
        {
            return self.trigger_object_matches(
                predicate,
                &self.trigger_event_object(permanent),
                source,
                false,
            );
        }
        if let Some(stack_object) = self.stack.iter().find(|candidate| candidate.id == id) {
            return self
                .stack_trigger_event_object(stack_object)
                .is_some_and(|object| {
                    self.trigger_object_matches(
                        predicate,
                        &object,
                        source,
                        stack_object.kind == StackObjectKind::Spell,
                    )
                });
        }
        if let Some((zone, card)) = self.card_in_nonbattlefield_zone(id) {
            return self.card_object_matches(predicate, card, zone, source);
        }
        // Nowhere at all: a token that was sacrificed has ceased to exist,
        // and "if the sacrificed creature was a Hamster" is a question about
        // exactly that. Answered from last-known information (CR 608.2h),
        // which is where the amounts beside it are read from too.
        match self.retired_objects.get(&id) {
            Some(crate::game::RetiredObject::Permanent { permanent, .. }) => self
                .trigger_object_matches(
                    predicate,
                    &self.trigger_event_object(permanent),
                    source,
                    false,
                ),
            Some(crate::game::RetiredObject::Card(card)) => {
                self.card_object_matches(predicate, card, ZoneKind::Graveyard, source)
            }
            Some(crate::game::RetiredObject::Stack(stack_object)) => self
                .stack_trigger_event_object(stack_object)
                .is_some_and(|object| {
                    self.trigger_object_matches(
                        predicate,
                        &object,
                        source,
                        stack_object.kind == StackObjectKind::Spell,
                    )
                }),
            None => false,
        }
    }
}

include!("object_sets.rs");
