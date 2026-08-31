use super::{
    BattlefieldExitCompletion, CardPartId, CopiableCharacteristics, CounterKind,
    DoubleFacedCopiableCharacteristics, EntryCompletion, Game, GameEvent, PendingBattlefieldEntry,
    PendingProcedure, Permanent, PlayerId, StackAbilityResolver, StackObject, StackObjectKind,
    Target, ZoneKind,
};
use crate::SpellResolutionDestinationDef;
use crate::card::{
    AbilityOperationDef, AppliedEffectDef, CharacteristicOperationDef, DeclarativeAbilityDef,
};

impl Game {
    pub(super) fn pass_priority(&mut self, _player: PlayerId) {
        self.consecutive_passes += 1;
        if self.consecutive_passes == 1 {
            self.priority = self.priority.opponent();
            return;
        }

        self.consecutive_passes = 0;
        if self.stack.is_empty() {
            self.advance_step();
        } else {
            self.resolve_stack_top();
            if self.result.is_none() {
                self.priority = self.active_player;
            }
        }
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn resolve_stack_top(&mut self) {
        let object = self
            .stack
            .pop()
            .expect("resolution is requested only for a nonempty stack");
        self.retire_stack_object(&object);
        match object.kind {
            StackObjectKind::ActivatedAbility | StackObjectKind::TriggeredAbility => {
                // Counted as the resolution begins rather than after it, so
                // an ability asking whether this is the first time it has
                // resolved this turn counts the one asking. A resolution
                // that suspends on a decision resumes at the finish, never
                // here, so nothing is counted twice.
                self.record_ability_resolution(&object);
                let pending_before = self.pending_decisions.len();
                let procedures_before = self.pending_procedures.len();
                let events_before = self.pending_events.len();
                let resolved = self.resolve_stack_ability(&object);
                if self.defer_stack_resolution(
                    pending_before,
                    procedures_before,
                    events_before,
                    &object,
                    resolved,
                ) {
                    return;
                }
                self.finish_stack_resolution(&object, resolved);
                return;
            }
            StackObjectKind::Spell => {}
        }
        let definition = object
            .card
            .definition
            .card_definition()
            .expect("a spell object is backed by a card definition");
        let spell_types = self
            .stack_spell_types(&object)
            .or_else(|| self.catalog.get(definition).map(|card| card.rules.types()))
            .unwrap_or_else(crate::card::CardTypeSet::empty);
        let aura_host = Self::aura_host_for(&object);
        let aura_player = Self::aura_player_for(&object);
        // A bestow spell whose host is gone is not countered: it loses the
        // Aura half and arrives as an enchantment creature (CR 702.103c),
        // which entering unattached is enough to make it.
        let bestowed = self.was_cast_for_bestow(&object);
        let aura_host = aura_host.filter(|_| !bestowed || !self.spell_fizzles(&object));
        let aura_fizzles = !bestowed
            && spell_types.is_permanent()
            && (aura_host.is_some() || aura_player.is_some())
            && self.spell_fizzles(&object);
        if spell_types.is_permanent() && !aura_fizzles {
            let chosen_player = match (aura_player, object.first_target()) {
                (None, Some(Target::Player(player))) => Some(player),
                _ => None,
            };
            let presented = object
                .signature
                .as_ref()
                .and_then(|signature| match signature.form() {
                    crate::card::SpellForm::Part(part) => Some(*part),
                    crate::card::SpellForm::Combined(parts) => parts.first().copied(),
                })
                .unwrap_or(CardPartId::PRIMARY);
            if object.is_copy {
                let cast = object.cast.clone();
                let copied_face = |part| CopiableCharacteristics {
                    base: crate::ObjectCharacteristics::card(definition, part),
                    added_types: crate::card::CardTypeSet::empty(),
                    added_abilities: Vec::new(),
                    retain_printed_subtypes: false,
                    base_power_toughness: None,
                    colors: object.colors,
                    added_creature_types: Vec::new(),
                    no_mana_cost: false,
                };
                let base = object.face_down.map_or_else(
                    || crate::ObjectCharacteristics::card(definition, presented),
                    crate::ObjectCharacteristics::face_down,
                );
                let double_faced = object
                    .face_down
                    .is_none()
                    .then(|| {
                        let definition = self.catalog.get(definition)?;
                        let crate::card::CardStructure::DoubleFaced { front, back, kind } =
                            definition.structure
                        else {
                            return None;
                        };
                        Some(DoubleFacedCopiableCharacteristics {
                            kind,
                            front_part: front,
                            back_part: back,
                            front: copied_face(front),
                            back: copied_face(back),
                        })
                    })
                    .flatten();
                self.create_token_copy_with_completion(
                    object.controller,
                    CopiableCharacteristics {
                        base,
                        added_types: crate::card::CardTypeSet::empty(),
                        added_abilities: Vec::new(),
                        retain_printed_subtypes: false,
                        base_power_toughness: None,
                        colors: object.colors,
                        added_creature_types: Vec::new(),
                        no_mana_cost: false,
                    },
                    double_faced,
                    presented,
                    EntryCompletion::SpellResolved {
                        card: object.id,
                        definition,
                    },
                    |permanent| {
                        permanent.chosen_player = chosen_player;
                        permanent.cast = cast;
                        permanent.attached_to = aura_host;
                        permanent.attached_player = aura_player;
                    },
                );
                return;
            }
            let mut permanent = Permanent::entering(
                object.card,
                presented,
                object.controller,
                self.turns_started[object.controller.index()],
                self.turn,
            );
            permanent.face_down = object.face_down;
            self.initialize_battlefield_entry(&mut permanent);
            let phyrexian_symbols_paid_with_life = object
                .cast
                .as_ref()
                .map_or(0, |cast| cast.phyrexian_symbols_paid_with_life);
            if phyrexian_symbols_paid_with_life > 0
                && self.effective_rules(&permanent).is_some_and(|rules| {
                    rules.has_executable_keyword(crate::card::KeywordAbility::Compleated)
                })
            {
                let loyalty = permanent
                    .counters(CounterKind::Loyalty)
                    .saturating_sub(phyrexian_symbols_paid_with_life.saturating_mul(2));
                permanent.set_counters(CounterKind::Loyalty, loyalty);
            }
            permanent.chosen_player = chosen_player;
            permanent.cast.clone_from(&object.cast);
            // "It gains haste until end of turn": an ability granted by the
            // mana that paid for a permanent spell keeps applying to the
            // permanent it becomes (CR 611.2c). Only keyword grants are
            // carried over, which is the only shape mana riders take.
            permanent.temporary_keywords.extend(
                object
                    .applied_effects
                    .iter()
                    .filter_map(|applied| Self::granted_keyword(applied.effect)),
            );
            // A whole ability rather than a keyword: what a graveyard
            // permission handed the spell it allowed ("if you do, it gains
            // ...") belongs to the permanent, and unlike the riders above it
            // has no expiration.
            for (granting, granted) in object
                .applied_effects
                .iter()
                .filter_map(|applied| Some((applied.granting?, applied.effect)))
                .collect::<Vec<_>>()
            {
                self.grant_resolved_ability_to_entering_permanent(
                    &mut permanent,
                    granting,
                    granted,
                );
            }
            if object.cast.as_ref().is_some_and(|cast| cast.via_suspend) {
                permanent.suspend_haste = true;
            }
            permanent.text_changes = object.text_changes;
            permanent.attached_to = aura_host;
            permanent.attached_player = aura_player;
            self.enqueue_battlefield_entry(PendingBattlefieldEntry {
                permanent,
                from: ZoneKind::Stack,
                completion: EntryCompletion::SpellResolved {
                    card: object.id,
                    definition,
                },
                redirected_to: None,
            });
            return;
        }
        let spell_fizzled = aura_fizzles || self.spell_fizzles(&object);
        if spell_fizzled {
            // 608.2b: a spell whose targets are all illegal on resolution does
            // nothing at all — a second Counterspell aimed at the same target
            // arrives to find it gone and goes to the graveyard spent.
            self.events.push(GameEvent::SpellFizzled {
                card: object.id,
                definition,
            });
        } else if object.ability.is_some() {
            let pending_before = self.pending_decisions.len();
            let procedures_before = self.pending_procedures.len();
            let events_before = self.pending_events.len();
            let _ = self.resolve_stack_ability(&object);
            if self.defer_stack_resolution(
                pending_before,
                procedures_before,
                events_before,
                &object,
                true,
            ) {
                return;
            }
        }
        self.finish_stack_resolution(&object, !spell_fizzled);
    }

    /// Records that one of a permanent's abilities is resolving, for the
    /// cards that count their own resolutions.
    fn record_ability_resolution(&mut self, object: &StackObject) {
        let Some(payload) = object.ability.as_ref() else {
            return;
        };
        let Some(source) = object.source else {
            return;
        };
        let origin = payload.origin;
        let Some(permanent) = self
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == source)
        else {
            return;
        };
        match permanent
            .resolutions_this_turn
            .iter_mut()
            .find(|(recorded, _)| *recorded == origin)
        {
            Some((_, count)) => *count = count.saturating_add(1),
            None => permanent.resolutions_this_turn.push((origin, 1)),
        }
    }

    pub(super) fn finish_stack_resolution(&mut self, object: &StackObject, resolved: bool) {
        let presentation = object.presentation();
        match object.kind {
            StackObjectKind::ActivatedAbility => {
                let source = object
                    .source
                    .expect("activated abilities remember their source");
                let event = if resolved {
                    GameEvent::AbilityResolved {
                        object: object.id,
                        source,
                        presentation,
                    }
                } else {
                    GameEvent::AbilityFizzled {
                        object: object.id,
                        source,
                        presentation,
                    }
                };
                self.events.push(event);
                return;
            }
            StackObjectKind::TriggeredAbility => {
                let source = object
                    .source
                    .expect("triggered abilities remember their source");
                let event = if resolved {
                    GameEvent::TriggeredAbilityResolved {
                        object: object.id,
                        source,
                        presentation,
                    }
                } else {
                    GameEvent::TriggeredAbilityFizzled {
                        object: object.id,
                        source,
                        presentation,
                    }
                };
                self.events.push(event);
                return;
            }
            StackObjectKind::Spell => {}
        }

        let definition = object
            .card
            .definition
            .card_definition()
            .expect("a spell object is backed by a card definition");

        let spell_types = self
            .stack_spell_types(object)
            .or_else(|| self.catalog.get(definition).map(|card| card.rules.types()))
            .unwrap_or_else(crate::card::CardTypeSet::empty);
        let aura_fizzles = !self.was_cast_for_bestow(object)
            && spell_types.is_permanent()
            && Self::aura_host_for(object).is_some()
            && self.spell_fizzles(object);
        let card_id = object.id;
        if !spell_types.is_permanent() || aura_fizzles {
            self.finish_spell_destination(object, resolved);
        }
        self.events.push(GameEvent::SpellResolved {
            card: card_id,
            definition,
        });
    }

    /// Whether this spell was cast for its bestow cost. CR 702.103c: such a
    /// spell is not countered when its target is gone -- it stops being an
    /// Aura spell and resolves as an enchantment creature instead.
    pub(super) fn was_cast_for_bestow(&self, object: &StackObject) -> bool {
        let Some(definition) = object.card.definition.card_definition() else {
            return false;
        };
        let Some(signature) = object.signature.as_ref() else {
            return false;
        };
        let Some(card) = self.catalog.get(definition) else {
            return false;
        };
        let Some(option) = card.play_option(signature.play_option()) else {
            return false;
        };
        self.selected_alternative_kind(card, option, object.id, signature.costs())
            == Some(crate::card::AlternativeCastKindDef::Bestow)
    }

    /// The keyword one applied effect grants, when granting a keyword is all
    /// it does. Reads through a composite, so a rider that grants two says
    /// the first of them and nothing else in the engine has to know it was
    /// compound.
    fn granted_keyword(effect: AppliedEffectDef) -> Option<crate::card::KeywordAbility> {
        match effect {
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
                AbilityOperationDef::Add(ability),
            )) => match ability.definition {
                DeclarativeAbilityDef::Keyword(keyword) => Some(keyword),
                _ => None,
            },
            AppliedEffectDef::Composite(effects) => {
                effects.iter().copied().find_map(Self::granted_keyword)
            }
            AppliedEffectDef::Characteristic(_) | AppliedEffectDef::Rule(_) => None,
        }
    }

    fn finish_spell_destination(&mut self, object: &StackObject, resolved: bool) {
        let owner = object.card.owner;
        let destination = if resolved {
            object
                .ability
                .as_ref()
                .and_then(|ability| ability.resolution_destination)
                .unwrap_or(SpellResolutionDestinationDef::Graveyard)
        } else {
            SpellResolutionDestinationDef::Graveyard
        };
        let destination = self.rebound_destination(object, destination);
        if object.is_copy {
            // A copy has no card to move, but "shuffle it into its owner's
            // library" still instructs its controller to shuffle.
            if destination == SpellResolutionDestinationDef::LibraryShuffled {
                self.rng.shuffle(&mut self.players[owner.index()].library);
            }
            return;
        }

        // Flashback replaces the move from the stack with exile, not the rest
        // of the resolution. In particular, White Sun's Zenith still shuffles
        // its owner's library, and a spell that already exiles itself still
        // gets its destination counters.
        let flashback_replaces_move = object.cast.as_ref().is_some_and(|cast| cast.via_flashback);
        let (mut card, _zone_change) = self.zone_change_card(
            object
                .card
                .clone()
                .into_card()
                .expect("a spell object is backed by a card"),
        );
        match destination {
            SpellResolutionDestinationDef::Graveyard if !flashback_replaces_move => {
                self.put_card_into_graveyard(owner, card);
            }
            SpellResolutionDestinationDef::Hand if !flashback_replaces_move => {
                self.players[owner.index()].hand.push(card);
            }
            // Flashback exiles the card wherever else it would have gone, so
            // a bought-back flashback spell is still exiled rather than
            // returned (CR 702.34a).
            SpellResolutionDestinationDef::Graveyard
            | SpellResolutionDestinationDef::Hand
            | SpellResolutionDestinationDef::Exile => {
                self.players[owner.index()].exile.push(card);
            }
            SpellResolutionDestinationDef::ExileOnAdventure => {
                // The exiled card is a new object, and it is that object its
                // owner may cast the creature half of later.
                self.permit_adventure_return(card.id, owner);
                self.players[owner.index()].exile.push(card);
            }
            SpellResolutionDestinationDef::ExileWithCounters(counters) => {
                for &(kind, amount) in counters {
                    card.add_counters(kind, amount);
                }
                self.players[owner.index()].exile.push(card);
            }
            SpellResolutionDestinationDef::Rebound => {}
            SpellResolutionDestinationDef::LibraryShuffled => {
                if flashback_replaces_move {
                    self.players[owner.index()].exile.push(card);
                } else {
                    self.players[owner.index()].library.push(card);
                }
                self.rng.shuffle(&mut self.players[owner.index()].library);
            }
        }
    }

    fn rebound_destination(
        &mut self,
        object: &StackObject,
        destination: SpellResolutionDestinationDef,
    ) -> SpellResolutionDestinationDef {
        if destination != SpellResolutionDestinationDef::Rebound {
            return destination;
        }
        // Rebound only applies to a successfully resolving physical spell its
        // caster cast from hand. It installs the delayed offer here, beside
        // the destination it replaces, so the keyword's two halves cannot
        // drift apart in card declarations.
        if !object.cast.as_ref().is_some_and(|cast| {
            cast.source_zone
                .is_some_and(|from| from.zone() == ZoneKind::Hand)
        }) || object.is_copy
        {
            return SpellResolutionDestinationDef::Graveyard;
        }
        let definition_id = object
            .card
            .definition
            .card_definition()
            .expect("a rebound spell is backed by a card definition");
        let definition = self
            .catalog
            .get(definition_id)
            .expect("a rebound spell's definition is cataloged");
        let signature = object
            .signature
            .as_ref()
            .expect("a rebound spell retains its cast signature");
        let option = definition
            .play_option(signature.play_option())
            .expect("a rebound spell retains its selected play option");
        let source_ability = Self::rebound_ability_origin(definition, option)
            .expect("a rebound destination comes from an executable rebound ability");
        let context = object
            .ability
            .as_ref()
            .expect("a rebound spell retains its frozen ability")
            .context
            .clone();
        self.install_trigger_from(
            crate::card::abilities::REBOUND_DELAYED_TRIGGER,
            super::ScopedEffect::primary(crate::card::EffectDef::None),
            object,
            context,
            source_ability,
        );
        SpellResolutionDestinationDef::Exile
    }

    fn defer_stack_resolution(
        &mut self,
        pending_before: usize,
        procedures_before: usize,
        events_before: usize,
        object: &StackObject,
        resolved: bool,
    ) -> bool {
        if self.defer_after_battlefield_exit(
            pending_before,
            BattlefieldExitCompletion::FinishStackResolution {
                object: Box::new(object.clone()),
                resolved,
            },
        ) {
            return true;
        }
        if self.pending_decisions.len() > pending_before
            || self.pending_procedures.len() > procedures_before
            || self.pending_events.len() > events_before
        {
            self.pending_procedures
                .push_back(PendingProcedure::FinishStackResolution {
                    object: Box::new(object.clone()),
                    resolved,
                });
            return true;
        }
        false
    }

    pub(super) fn resolve_stack_ability(&mut self, object: &StackObject) -> bool {
        if self.stack_ability_fizzles(object) {
            return false;
        }
        // Rule 603.4's second look. A condition that has stopped holding
        // since the ability triggered makes it do nothing at all, which is
        // reported the same way an ability with no legal target is.
        if let Some(ability) = object.ability.as_ref()
            && let Some(condition) = ability.condition
            && !self.trigger_condition_holds(
                condition,
                object.source.unwrap_or(object.id),
                object.controller,
                ability.context.trigger,
                Some(ability.origin),
                None,
            )
        {
            return false;
        }
        let (resolver, context, mode_effects) = object
            .ability
            .as_ref()
            .map(|ability| {
                (
                    ability.resolver,
                    ability.context.clone(),
                    ability.mode_effects.as_slice(),
                )
            })
            .expect("ability stack objects freeze their complete payload");
        match resolver {
            StackAbilityResolver::Prepared {
                reference: _,
                effect,
            } if self.prepared_engine.enabled() && mode_effects.is_empty() => {
                crate::prepared_engine::execute_effect(effect, self, object.controller);
            }
            StackAbilityResolver::Prepared { reference, .. } => {
                let mut effects = Vec::with_capacity(mode_effects.len() + 1);
                effects.push(reference);
                effects.extend_from_slice(mode_effects);
                self.resolve_effects_in_order(effects, object, context);
            }
            StackAbilityResolver::Declarative(effect)
            | StackAbilityResolver::DeclarativeIgnoringTargetFizzle(effect) => {
                let mut effects = Vec::with_capacity(mode_effects.len() + 1);
                effects.push(effect);
                effects.extend_from_slice(mode_effects);
                self.resolve_effects_in_order(effects, object, context);
            }
            StackAbilityResolver::CastOffer(alternative) => {
                if let (Some(card), Some(payload)) = (object.source, object.ability.as_ref()) {
                    debug_assert_eq!(
                        self.ability_for_origin(card, payload.origin)
                            .and_then(|ability| match ability.definition {
                                super::DeclarativeAbilityDef::AlternativeCast(definition) => {
                                    Some(definition.kind)
                                }
                                _ => None,
                            }),
                        Some(alternative),
                    );
                    self.queue_alternative_cast_offer(object.controller, card, payload.origin);
                }
            }
        }
        true
    }

    pub(super) fn stack_ability_fizzles(&self, object: &StackObject) -> bool {
        let Some(ability) = &object.ability else {
            return false;
        };
        if matches!(
            ability.resolver,
            StackAbilityResolver::DeclarativeIgnoringTargetFizzle(_)
        ) {
            return false;
        }
        let mut had_target = false;
        let mut has_legal_target = false;
        for selection in &ability.targets {
            if ability.target_defs.get(selection.slot().index()).is_none() {
                continue;
            }
            for target in selection.targets() {
                had_target = true;
                has_legal_target |=
                    self.stack_ability_target_is_legal(object, selection.slot(), *target);
            }
        }
        had_target && !has_legal_target
    }
}
