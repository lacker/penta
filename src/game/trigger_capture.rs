use crate::CharacteristicContext;
use crate::card::{
    AbilityPredicateDef, AttackDeclarationRangeDef, AttackEventMatcherDef,
    BattlefieldEntryChoiceDestinationDef, DamageSourceGroupDef, ZoneChangeObservationDef,
};

use super::{
    AbilityDef, AbilityId, AbilityOrigin, AbilityProcedureDef, AbilitySourceRef, AddManaEffectDef,
    BattlefieldTriggerListener, CardDefinitionId, CardPartId, CardType, CommittedTriggerEvent,
    DamageEventMatcherDef, DamageKindDef, DamageRecipientMatcherDef, DamageSourceMatcherDef,
    DeclarativeAbilityDef, EffectDef, EffectRecipientSetDef, EffectResolutionContext,
    EffectiveAbility, FrozenActivatedAbility, Game, GameEvent, GameObjectId, GrantId,
    InstalledTriggerLifetime, KeywordAbility, Mana, ManaSelectionDef, ManaSource,
    ObjectCharacteristics, ObjectPredicateDef, ObjectRefDef, ObjectSetDef, PendingTrigger,
    Permanent, PlayerId, PlayerRefDef, PlayerRelation, PlayerSetDef, RetiredObject, ScopedEffect,
    StackAbilityResolver, TapPurposeDef, Target, TriggerCapture, TriggerContext, TriggerEventDef,
    TriggerEventObject, ZoneKind,
};

mod exile;
mod graveyard;
include!("trigger_capture/drawing.rs");

impl Game {
    pub(super) fn capture_battlefield_triggers(&mut self, event: &CommittedTriggerEvent) {
        let listeners = self.battlefield_trigger_listeners();
        self.capture_battlefield_triggers_from_snapshot(&listeners, event);
    }

    /// "Whenever one or more counters are put on this permanent." One event
    /// per permanent the placement touched, and one per placement rather
    /// than one per counter: two at once is one event carrying two.
    pub(in crate::game) fn capture_counters_placed(
        &mut self,
        permanents: &[GameObjectId],
        kind: crate::card::CounterKind,
        amount: u16,
    ) {
        if amount == 0 {
            return;
        }
        for permanent in permanents {
            let Some(object) = self
                .battlefield
                .iter()
                .find(|candidate| candidate.card.id == *permanent)
                .map(|candidate| self.trigger_event_object(candidate))
            else {
                continue;
            };
            self.capture_battlefield_triggers(&CommittedTriggerEvent::CountersPlaced {
                object,
                kind,
                amount,
            });
        }
    }

    /// "Whenever this becomes the target of a spell or ability", for the
    /// ability half. Raised once the ability is on the stack, which is where
    /// its targets are locked in, and once per targeting ability however many
    /// of its slots name the same permanent (CR 115.7c) -- the same rule the
    /// cast side follows.
    pub(super) fn capture_ability_targeting_triggers(&mut self, ability: GameObjectId) {
        let Some(object) = self.stack.iter().find(|object| object.id == ability) else {
            return;
        };
        let Some(event) = self.stack_object_event_object(object) else {
            return;
        };
        let mut targeted = Vec::new();
        let mut targeted_players = Vec::new();
        for target in object
            .ability
            .as_ref()
            .into_iter()
            .flat_map(|payload| payload.targets.iter())
            .flat_map(crate::TargetSelection::targets)
        {
            match target {
                Target::Permanent(id) | Target::Card(id) if !targeted.contains(id) => {
                    targeted.push(*id);
                }
                Target::Player(player) if !targeted_players.contains(player) => {
                    targeted_players.push(*player);
                }
                _ => {}
            }
        }
        for target in targeted {
            self.capture_battlefield_triggers(&CommittedTriggerEvent::BecameTargetOfAbility {
                target,
                object: event.clone(),
            });
        }
        for player in targeted_players {
            self.capture_battlefield_triggers(&CommittedTriggerEvent::PlayerBecameTarget {
                player,
                object: event.clone(),
            });
        }
    }

    /// "When you cycle this card" (CR 702.29b), raised as the cycling ability
    /// is activated. Only the cycled card can carry the clause, so its own
    /// printed abilities are the entire listener list -- there is no zone to
    /// scan. The card is read in the graveyard the discard cost has already
    /// put it in, which is also the object the trigger names.
    pub(super) fn capture_cycling_triggers(&mut self, cycled: GameObjectId, player: PlayerId) {
        let Some((_zone, card)) = self.card_in_nonbattlefield_zone(cycled) else {
            return;
        };
        let card = card.clone();
        let Some(object) = self.printed_trigger_event_object(
            cycled,
            card.definition,
            player,
            &CharacteristicContext::Graveyard,
        ) else {
            return;
        };
        let mut listeners = Vec::new();
        self.for_each_printed_card_ability(&card, &CharacteristicContext::Graveyard, |effective| {
            let ability = effective.ability;
            let DeclarativeAbilityDef::Triggered(definition) = ability.definition else {
                return;
            };
            if !ability.is_executable()
                || definition.event != TriggerEventDef::Cycled
                || definition.procedure != AbilityProcedureDef::Shared
            {
                return;
            }
            listeners.push(BattlefieldTriggerListener {
                event: definition.event,
                uses_stack: true,
                trigger_limit: definition.trigger_limit,
                installed: None,
                capture: TriggerCapture {
                    source: AbilitySourceRef {
                        object: cycled,
                        ability: effective.origin,
                    },
                    presentation: Self::ability_presentation(
                        effective.origin,
                        ObjectCharacteristics::card(card.definition, CardPartId::PRIMARY),
                    ),
                    owner: card.owner,
                    controller: player,
                    text: ability.text,
                    target_defs: definition.targets.to_vec(),
                    targets: Vec::new(),
                    effect: ability.declarative_effect().unwrap_or(EffectDef::None),
                    resolver: Self::ability_resolver(effective.origin, &ability),
                    context: TriggerContext::empty().into(),
                    condition: definition.condition,
                    modes: definition.modes,
                    x: 0,
                },
            });
        });
        if listeners.is_empty() {
            return;
        }
        self.capture_battlefield_triggers_from_snapshot(
            &listeners,
            &CommittedTriggerEvent::Cycled { object },
        );
    }

    /// A spell's own "when you cast this spell" clause, raised as it is put on
    /// the stack. Storm is the case: the ability belongs to the spell rather
    /// than to anything on the battlefield, so the ordinary listener scan
    /// never sees it, and the spell it copies is still on the stack beneath
    /// the trigger when that trigger resolves.
    pub(super) fn capture_own_cast_triggers(&mut self, spell: GameObjectId) {
        let Some(cast) = self.stack.iter().find(|object| object.id == spell).cloned() else {
            return;
        };
        let Some(object) = self.stack_object_event_object(&cast) else {
            return;
        };
        let Some(card) = cast.card.clone().into_card() else {
            return;
        };
        let Some(signature) = cast.signature.as_ref() else {
            return;
        };
        let Some(cast_from) = cast.cast_from_zone else {
            return;
        };
        let context = CharacteristicContext::Stack {
            form: signature.form().clone(),
        };
        let mut listeners = Vec::new();
        self.for_each_printed_card_ability(&card, &context, |effective| {
            let ability = effective.ability;
            let DeclarativeAbilityDef::Triggered(definition) = ability.definition else {
                return;
            };
            let watches_this_cast = match definition.event {
                TriggerEventDef::SpellCast {
                    object: ObjectPredicateDef::Source,
                    from,
                } => from.is_none_or(|zone| cast_from.zone() == zone),
                _ => false,
            };
            if !ability.is_executable()
                || !watches_this_cast
                || definition.procedure != AbilityProcedureDef::Shared
            {
                return;
            }
            listeners.push(BattlefieldTriggerListener {
                event: definition.event,
                uses_stack: true,
                trigger_limit: definition.trigger_limit,
                installed: None,
                capture: TriggerCapture {
                    source: AbilitySourceRef {
                        object: spell,
                        ability: effective.origin,
                    },
                    presentation: Self::ability_presentation(
                        effective.origin,
                        ObjectCharacteristics::card(card.definition, CardPartId::PRIMARY),
                    ),
                    owner: card.owner,
                    controller: cast.controller,
                    text: ability.text,
                    target_defs: definition.targets.to_vec(),
                    targets: Vec::new(),
                    effect: ability.declarative_effect().unwrap_or(EffectDef::None),
                    resolver: Self::ability_resolver(effective.origin, &ability),
                    context: TriggerContext::empty().into(),
                    condition: definition.condition,
                    modes: definition.modes,
                    x: 0,
                },
            });
        });
        if listeners.is_empty() {
            return;
        }
        self.capture_battlefield_triggers_from_snapshot(
            &listeners,
            &CommittedTriggerEvent::SpellCast {
                object,
                from: cast_from,
            },
        );
    }

    /// How many times this object's copy of one ability has triggered this
    /// turn, for the abilities that cap themselves.
    pub(super) fn triggers_this_turn(&self, source: AbilitySourceRef) -> u8 {
        self.battlefield
            .iter()
            .chain(self.emblems.iter())
            .find(|permanent| permanent.card.id == source.object)
            .and_then(|permanent| {
                permanent
                    .triggers_this_turn
                    .iter()
                    .find(|(origin, _)| *origin == source.ability)
            })
            .map_or(0, |(_, count)| *count)
    }

    fn record_trigger_this_turn(&mut self, source: AbilitySourceRef) {
        let Some(permanent) = self
            .battlefield
            .iter_mut()
            .chain(self.emblems.iter_mut())
            .find(|permanent| permanent.card.id == source.object)
        else {
            return;
        };
        if let Some(entry) = permanent
            .triggers_this_turn
            .iter_mut()
            .find(|(origin, _)| *origin == source.ability)
        {
            entry.1 = entry.1.saturating_add(1);
        } else {
            permanent.triggers_this_turn.push((source.ability, 1));
        }
    }

    pub(super) fn battlefield_trigger_listeners(&self) -> Vec<BattlefieldTriggerListener> {
        let mut listeners = Vec::new();
        // Emblems listen alongside the battlefield, the way they already sit
        // in every other sweep. Nothing about an emblem is in a zone, so the
        // battlefield an emblem's clause names is the one its abilities were
        // written against rather than a place it can be found.
        for permanent in self.battlefield.iter().chain(self.emblems.iter()) {
            self.for_each_effective_ability(permanent, |effective| {
                let ability = effective.ability;
                if !ability.is_executable() {
                    return;
                }
                let (definition, uses_stack) = match ability.definition {
                    DeclarativeAbilityDef::TriggeredMana(definition) => {
                        if ability.declarative_effect().is_none() {
                            return;
                        }
                        (definition, false)
                    }
                    DeclarativeAbilityDef::Triggered(definition) => (definition, true),
                    DeclarativeAbilityDef::Spell(_)
                    | DeclarativeAbilityDef::ActivatedMana(_)
                    | DeclarativeAbilityDef::Activated(_)
                    | DeclarativeAbilityDef::Static(_)
                    | DeclarativeAbilityDef::Replacement(_)
                    | DeclarativeAbilityDef::AlternativeCast(_)
                    | DeclarativeAbilityDef::OptionalAdditionalCost(_)
                    | DeclarativeAbilityDef::SpecialAction(_)
                    | DeclarativeAbilityDef::Pregame(_)
                    | DeclarativeAbilityDef::Keyword(_)
                    | DeclarativeAbilityDef::DeckConstruction(_)
                    | DeclarativeAbilityDef::Legacy => return,
                };
                // Compatibility procedures execute elsewhere, so admitting
                // them here would manufacture a duplicate trigger.
                if definition.procedure != AbilityProcedureDef::Shared {
                    return;
                }
                if !definition.source_zones.contains(&ZoneKind::Battlefield) {
                    return;
                }
                let source = AbilitySourceRef {
                    object: permanent.card.id,
                    ability: effective.origin,
                };
                listeners.push(BattlefieldTriggerListener {
                    event: definition.event,
                    uses_stack,
                    trigger_limit: definition.trigger_limit,
                    installed: None,
                    capture: TriggerCapture {
                        source,
                        presentation: Self::ability_presentation(
                            effective.origin,
                            Self::effective_rules_source(permanent),
                        ),
                        owner: permanent.card.owner,
                        controller: permanent.controller,
                        text: ability.text,
                        target_defs: definition.targets.to_vec(),
                        targets: Vec::new(),
                        effect: ability.declarative_effect().unwrap_or(EffectDef::None),
                        resolver: Self::ability_resolver(effective.origin, &ability),
                        context: TriggerContext::empty().into(),
                        condition: definition.condition,
                        modes: definition.modes,
                        x: 0,
                    },
                });
            });
        }
        self.extend_with_graveyard_trigger_listeners(&mut listeners);
        self.extend_with_exile_trigger_listeners(&mut listeners);
        // Installed triggers listen the same way, minus a permanent to hang
        // on; they are appended last so a permanent's own triggers keep the
        // relative order they had before any existed.
        listeners.extend(self.installed_triggers.iter().map(|installed| {
            BattlefieldTriggerListener {
                trigger_limit: None,
                event: installed.event,
                uses_stack: true,
                installed: Some(installed.id),
                capture: installed.capture.clone(),
            }
        }));
        listeners
    }

    pub(super) fn capture_battlefield_triggers_from_snapshot(
        &mut self,
        listeners: &[BattlefieldTriggerListener],
        event: &CommittedTriggerEvent,
    ) {
        self.capture_battlefield_trigger_batch_from_snapshot(
            listeners,
            std::slice::from_ref(event),
        );
    }

    /// Determine every match and intervening-if result for one atomic event
    /// batch before any triggered-mana ability can mutate the game. Attack
    /// declarations and simultaneous exits both publish more than one
    /// object-local event, but all of those facts belong to one rules event.
    pub(super) fn capture_battlefield_trigger_batch_from_snapshot(
        &mut self,
        listeners: &[BattlefieldTriggerListener],
        events: &[CommittedTriggerEvent],
    ) {
        self.capture_battlefield_trigger_batch_with_mana_resolver(
            listeners,
            events,
            |game, capture| {
                game.resolve_triggered_mana_effect(
                    capture.source,
                    capture.controller,
                    capture.effect,
                    &capture.context,
                );
            },
        );
    }

    fn batch_trigger_context(
        &self,
        listener: &BattlefieldTriggerListener,
        event: &CommittedTriggerEvent,
        events: &[CommittedTriggerEvent],
        matched_damage_recipients: &mut Vec<(AbilitySourceRef, Option<u32>, Target)>,
    ) -> Option<TriggerContext> {
        let mut context = event.context();
        let (
            TriggerEventDef::DamageDealt(DamageEventMatcherDef {
                source: DamageSourceMatcherDef::Any,
                ..
            }),
            CommittedTriggerEvent::DamageDealt { recipient, .. },
        ) = (listener.event, event)
        else {
            return Some(context);
        };
        let occurrence = (listener.capture.source, listener.installed, *recipient);
        if matched_damage_recipients.contains(&occurrence) {
            return None;
        }
        matched_damage_recipients.push(occurrence);
        let amount = events
            .iter()
            .filter(|candidate| {
                matches!(
                    candidate,
                    CommittedTriggerEvent::DamageDealt {
                        recipient: candidate_recipient,
                        ..
                    } if candidate_recipient == recipient
                ) && self.trigger_event_matches_for_controller(
                    listener.event,
                    candidate,
                    listener.capture.source.object,
                    Some(listener.capture.controller),
                )
            })
            .filter_map(|candidate| match candidate {
                CommittedTriggerEvent::DamageDealt { amount, .. } => Some(*amount),
                _ => None,
            })
            .fold(0_u16, u16::saturating_add);
        context.amount = Some(i32::from(amount));
        Some(context)
    }

    pub(super) fn capture_battlefield_trigger_batch_with_mana_resolver(
        &mut self,
        listeners: &[BattlefieldTriggerListener],
        events: &[CommittedTriggerEvent],
        mut resolve_mana: impl FnMut(&mut Self, TriggerCapture),
    ) {
        let mut consumed_once = Vec::new();
        let mut matched = Vec::new();
        // One simultaneous damage event can contain assignments from several
        // sources to the same recipient. A clause concerned only with what
        // was dealt damage sees one occurrence for that recipient, carrying
        // the total amount, rather than one occurrence per source.
        let mut matched_damage_recipients = Vec::new();
        // "Triggers only once each turn" counts the triggering rather than
        // the resolution, and one batch can offer a capped ability several
        // matching events, so the count has to rise inside this loop as
        // well as be read from the turn so far.
        let mut limited = Vec::new();
        for event in events {
            for listener in listeners {
                if !self.trigger_event_matches_for_controller(
                    listener.event,
                    event,
                    listener.capture.source.object,
                    Some(listener.capture.controller),
                ) {
                    continue;
                }
                let Some(trigger_context) = self.batch_trigger_context(
                    listener,
                    event,
                    events,
                    &mut matched_damage_recipients,
                ) else {
                    continue;
                };
                if let Some(limit) = listener.trigger_limit {
                    let source = listener.capture.source;
                    let already = self.triggers_this_turn(source);
                    let in_batch = limited.iter().filter(|counted| **counted == source).count();
                    if usize::from(already).saturating_add(in_batch) >= usize::from(limit) {
                        continue;
                    }
                    limited.push(source);
                }
                if let Some(id) = listener.installed
                    && self
                        .installed_triggers
                        .iter()
                        .find(|installed| installed.id == id)
                        .is_some_and(|installed| {
                            matches!(installed.lifetime, InstalledTriggerLifetime::Once)
                        })
                {
                    if consumed_once.contains(&id) {
                        continue;
                    }
                    // A once-only listener is consumed by the first matching
                    // event even when its intervening-if condition is false.
                    consumed_once.push(id);
                }
                let mut capture = listener.capture.clone();
                // Keep installer bindings and targets; only the committed
                // event-local context changes for this match.
                capture.context.trigger = trigger_context;
                if let Some(object) = Self::zone_change_event_object(listener.event, event) {
                    capture.context.trigger.object = Some(object.id);
                    capture.context.trigger.object_controller = Some(object.controller);
                }
                let condition_holds = self.trigger_capture_condition_holds(&capture);
                // "That ability triggers an additional time" is not a second
                // ability but the same one again, so the extra instances are
                // exact copies of this match and are ordered beside it.
                for _ in 0..self.additional_trigger_copies(event, &capture) {
                    matched.push((listener.uses_stack, capture.clone(), condition_holds));
                }
                matched.push((listener.uses_stack, capture, condition_holds));
            }
        }

        self.installed_triggers
            .retain(|installed| !consumed_once.contains(&installed.id));
        for source in limited {
            self.record_trigger_this_turn(source);
        }

        // Record ordinary triggers first, using the precomputed condition.
        // Any triggers caused while a triggered-mana ability resolves are
        // therefore later in the pending stream than the event that caused it.
        for (uses_stack, capture, condition_holds) in &matched {
            if *uses_stack && *condition_holds {
                self.capture_trigger_prechecked(capture);
            }
        }
        for (uses_stack, capture, condition_holds) in matched {
            if !uses_stack && condition_holds {
                resolve_mana(self, capture);
            }
        }
    }

    /// How many extra times this match triggers, under every
    /// "triggers an additional time" rule its controller has.
    ///
    /// Only an arrival on the battlefield is doubled; every printed clause
    /// of this shape names one. Each rule counts once, so two of them are
    /// two extra triggers rather than four.
    fn additional_trigger_copies(
        &self,
        event: &CommittedTriggerEvent,
        capture: &TriggerCapture,
    ) -> u8 {
        let CommittedTriggerEvent::ZoneChanged {
            after: Some(object),
            to: ZoneKind::Battlefield,
            ..
        } = event
        else {
            return 0;
        };
        // A trigger whose source has left the battlefield -- or never was a
        // permanent -- is nobody's "triggered ability of a permanent you
        // control", however the event reads.
        let Some(source) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == capture.source.object)
        else {
            return 0;
        };
        let source_object = self.trigger_event_object(source);
        let controller = capture.controller;
        let mut copies = 0_u8;
        self.visit_player_static_rules(controller, |rule| {
            let crate::card::AppliedRuleDef::TriggersAnAdditionalTime(doubling) = rule else {
                return;
            };
            if self.trigger_object_matches_for_controller(
                doubling.entering,
                object,
                capture.source.object,
                false,
                Some(controller),
            ) && self.trigger_object_matches_for_controller(
                doubling.permanent,
                &source_object,
                capture.source.object,
                false,
                Some(controller),
            ) {
                copies = copies.saturating_add(1);
            }
        });
        copies
    }

    pub(super) fn capture_custom_source_triggers(
        &mut self,
        source: &Permanent,
        abilities: &[EffectiveAbility],
        event: &CommittedTriggerEvent,
    ) {
        let triggers = abilities
            .iter()
            .filter_map(|effective| match effective.ability.definition {
                DeclarativeAbilityDef::Triggered(definition)
                    if effective.ability.is_executable()
                        && definition.procedure == AbilityProcedureDef::Legacy
                        && effective.ability.custom_behavior().is_some()
                        && definition.source_zones.contains(&ZoneKind::Battlefield)
                        && self.trigger_event_matches_for_controller(
                            definition.event,
                            event,
                            source.card.id,
                            Some(source.controller),
                        ) =>
                {
                    Some((
                        effective.origin,
                        effective.ability.text,
                        definition.targets,
                        effective
                            .ability
                            .declarative_effect()
                            .unwrap_or(EffectDef::None),
                        Self::ability_resolver(effective.origin, &effective.ability),
                    ))
                }
                DeclarativeAbilityDef::Spell(_)
                | DeclarativeAbilityDef::ActivatedMana(_)
                | DeclarativeAbilityDef::TriggeredMana(_)
                | DeclarativeAbilityDef::Activated(_)
                | DeclarativeAbilityDef::Triggered(_)
                | DeclarativeAbilityDef::Static(_)
                | DeclarativeAbilityDef::Replacement(_)
                | DeclarativeAbilityDef::AlternativeCast(_)
                | DeclarativeAbilityDef::OptionalAdditionalCost(_)
                | DeclarativeAbilityDef::SpecialAction(_)
                | DeclarativeAbilityDef::Pregame(_)
                | DeclarativeAbilityDef::Keyword(_)
                | DeclarativeAbilityDef::DeckConstruction(_)
                | DeclarativeAbilityDef::Legacy => None,
            })
            .collect::<Vec<_>>();
        for (ability, text, targets, effect, resolver) in triggers {
            self.capture_trigger(&TriggerCapture {
                source: AbilitySourceRef {
                    object: source.card.id,
                    ability,
                },
                presentation: Self::ability_presentation(
                    ability,
                    Self::effective_rules_source(source),
                ),
                owner: source.card.owner,
                controller: source.controller,
                text,
                target_defs: targets.to_vec(),
                targets: Vec::new(),
                effect,
                resolver,
                context: event.context().into(),
                // A legacy custom trigger states its own condition inside its
                // behavior rather than declaring one here, and prints no
                // modes.
                condition: None,
                modes: None,
                x: 0,
            });
        }
    }

    // Long only because every event definition pairs with its committed event.
    fn trigger_event_player_reference(
        &self,
        reference: PlayerRefDef,
        ability_source: GameObjectId,
        controller: Option<PlayerId>,
        event: &CommittedTriggerEvent,
    ) -> Option<PlayerId> {
        match reference {
            PlayerRefDef::EffectController => controller,
            PlayerRefDef::EnchantedPlayer => {
                self.current_or_last_known_enchanted_player(ability_source)
            }
            PlayerRefDef::Opponent => controller.map(PlayerId::opponent),
            PlayerRefDef::EventPlayer => event.context().event_player,
            PlayerRefDef::ControllerOf(reference) => self
                .trigger_event_object_reference(reference, ability_source, event)
                .and_then(|object| self.current_or_last_known_controller(object)),
            PlayerRefDef::OwnerOf(reference) => self
                .trigger_event_object_reference(reference, ability_source, event)
                .and_then(|object| self.current_or_last_known_owner(object)),
            PlayerRefDef::OpponentOf(reference) => self
                .trigger_event_object_reference(reference, ability_source, event)
                .and_then(|object| self.current_or_last_known_controller(object))
                .map(PlayerId::opponent),
            PlayerRefDef::Target(_) => None,
        }
    }

    /// Who controls an object, whether it is still on the battlefield or has
    /// left and is only remembered.
    pub(super) fn controller_of_object(&self, object: GameObjectId) -> Option<PlayerId> {
        self.battlefield
            .iter()
            .chain(self.emblems.iter())
            .find(|permanent| permanent.card.id == object)
            .map(|permanent| permanent.controller)
            .or_else(|| match self.retired_objects.get(&object) {
                Some(RetiredObject::Permanent { permanent, .. }) => Some(permanent.controller),
                Some(RetiredObject::Stack(object)) => Some(object.controller),
                Some(RetiredObject::Card(_)) | None => None,
            })
    }
}

include!("trigger_capture/event_matching.rs");
include!("trigger_capture/attack_matching.rs");
include!("trigger_capture/ability_resolver.rs");
include!("trigger_capture/damage_matching.rs");
include!("trigger_capture/triggered_mana.rs");
include!("trigger_capture/object_matching.rs");
include!("trigger_capture/procedure.rs");

#[cfg(test)]
mod emblem_trigger_limit_tests {
    use super::*;

    #[test]
    fn emblem_trigger_counts_are_read_from_the_emblem() {
        let mut game = crate::game::tests::ready_game();
        let owner = PlayerId::One;
        let card = game
            .unbacked_emblem_object(crate::EmblemCharacteristics::new("Test emblem", &[]), owner);
        let source = AbilitySourceRef {
            object: card.id,
            ability: AbilityOrigin::Emblem {
                ability: AbilityId::PRIMARY,
            },
        };
        game.emblems.push(Permanent::entering(
            card,
            CardPartId::PRIMARY,
            owner,
            game.turns_started[owner.index()],
            game.turn,
        ));

        assert_eq!(game.triggers_this_turn(source), 0);
        game.record_trigger_this_turn(source);
        assert_eq!(game.triggers_this_turn(source), 1);
    }
}
