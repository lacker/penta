use super::{
    AbilityCostDef, AbilityOrigin, AlternativeCastKindDef, BTreeMap, BattlefieldExitCompletion,
    CREATURE_TYPES, CardDefinition, CardDefinitionId, CardType, CardTypeSet, CastChoices,
    CastSignature, CastSourceZone, CommittedTriggerEvent, DecisionContinuation, DecisionOption,
    DecisionPreference, DecisionVisibility, DecisionZone, DeclarativeAbilityDef, EntryCompletion,
    Game, GameEvent, GameObjectId, Mana, ManaActivationChoices, ManaColor, ManaPaymentPurpose,
    PendingBattlefieldEntry, Permanent, PlayActionKind, PlayOptionDef, PlayOptionId, PlayerId,
    StackObject, StackObjectKind, Target, ZoneKind, ZoneMoveCause, ZonePlacement, remove_card,
};
mod signature_validation;

use crate::card::{
    BattlefieldEntryScalarChoiceDef, CardSet, ScalarChoiceListDef, SpellLifeCostDef, SpendModeDef,
};

impl Game {
    pub(super) fn play_land(
        &mut self,
        player: PlayerId,
        card_id: GameObjectId,
        option_id: PlayOptionId,
    ) {
        // A land is ordinarily played from hand; a permission can also
        // offer one out of its owner's graveyard, or off the top of their
        // library.
        let state = &self.players[player.index()];
        let from = if state.hand.iter().any(|card| card.id == card_id) {
            ZoneKind::Hand
        } else if state.graveyard.iter().any(|card| card.id == card_id) {
            ZoneKind::Graveyard
        } else {
            ZoneKind::Library
        };
        let definition_id = match from {
            ZoneKind::Graveyard => &state.graveyard,
            ZoneKind::Library => &state.library,
            _ => &state.hand,
        }
        .iter()
        .find(|card| card.id == card_id)
        .map(|card| card.definition)
        .expect("legal land action references a card in a playable zone");
        let definition = self
            .catalog
            .get(definition_id)
            .expect("legal land action references a cataloged card");
        let option = definition
            .play_option(option_id)
            .filter(|option| option.action == PlayActionKind::PlayLand)
            .expect("legal land action references a land play option");
        let presented = match &option.form {
            crate::card::SpellForm::Part(part) => *part,
            crate::card::SpellForm::Combined(_) => {
                unreachable!("a land play option presents exactly one card part")
            }
        };
        definition
            .part(presented)
            .filter(|part| part.rules.has_type(CardType::Land))
            .expect("land play option references a land part");
        let state = &mut self.players[player.index()];
        let source_zone = match from {
            ZoneKind::Graveyard => &mut state.graveyard,
            ZoneKind::Library => &mut state.library,
            _ => &mut state.hand,
        };
        let card = remove_card(source_zone, card_id)
            .expect("legal land action references a card in a playable zone");
        self.players[player.index()].land_played_this_turn = true;
        self.consecutive_passes = 0;
        let permanent =
            Permanent::entering(card, presented, player, self.turns_started[player.index()]);
        self.enqueue_battlefield_entry(PendingBattlefieldEntry {
            permanent,
            from,
            completion: EntryCompletion::LandPlayed { player },
            redirected_to: None,
        });
    }

    pub(super) fn creature_type_choices(&self, player: PlayerId) -> Vec<String> {
        let mut counts = CREATURE_TYPES
            .iter()
            .map(|creature_type| ((*creature_type).into(), 0))
            .collect::<BTreeMap<String, usize>>();
        for card in &self.players[player.index()].hand {
            let Some(definition) = self.catalog.get(card.definition) else {
                continue;
            };
            for part in &definition.parts {
                if part.rules.has_type(CardType::Creature) {
                    for subtype in part.rules.subtypes() {
                        if let Some(count) = counts.get_mut(*subtype) {
                            *count += 1;
                        }
                    }
                }
            }
        }
        let mut choices = counts.into_iter().collect::<Vec<_>>();
        choices.sort_by(|(left_name, left_count), (right_name, right_count)| {
            right_count
                .cmp(left_count)
                .then_with(|| left_name.cmp(right_name))
        });
        choices.into_iter().map(|(name, _)| name).collect()
    }

    /// "You may have this enter as a copy of ...": the copy is picked as the
    /// permanent enters, and entering as itself is always an option.
    pub(super) fn queue_entry_copy_choice(
        &mut self,
        player: PlayerId,
        choices: Vec<GameObjectId>,
        added_types: CardTypeSet,
        retain_printed_subtypes: bool,
        added_abilities: Vec<super::CopiableAbility>,
    ) {
        let mut options = vec![DecisionOption {
            id: 0,
            label: "Enter as itself".into(),
            card: None,
            members: Vec::new(),
            ability_text: None,
            zone: DecisionZone::None,
        }];
        options.extend(choices.iter().enumerate().filter_map(|(index, id)| {
            let permanent = self
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == *id)?;
            let presentation = Self::effective_rules_source(permanent);
            Some(DecisionOption {
                id: u32::try_from(index + 1).unwrap_or(u32::MAX),
                label: self.presentation_name(presentation).map_or_else(
                    || "Copy an unknown permanent".into(),
                    |name| format!("Enter as a copy of {name}"),
                ),
                card: Some((*id, presentation)),
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::Battlefield,
            })
        }));
        self.queue_decision(
            player,
            "Choose what this permanent enters as",
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            1..=1,
            false,
            options,
            DecisionContinuation::BattlefieldEntryCopy {
                choices,
                added_types,
                retain_printed_subtypes,
                added_abilities,
            },
        );
    }

    pub(super) fn queue_entry_scalar_choice(
        &mut self,
        player: PlayerId,
        context: super::ReplacementEffectContext,
        choice: BattlefieldEntryScalarChoiceDef,
    ) {
        let (prompt, choices) = self.entry_scalar_choices(player, choice);
        let options = choices
            .iter()
            .enumerate()
            .map(|(index, value)| DecisionOption {
                id: u32::try_from(index).unwrap_or(u32::MAX),
                label: value.clone(),
                card: None,
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::None,
            })
            .collect();
        self.queue_decision(
            player,
            prompt,
            DecisionVisibility::Public,
            DecisionPreference::Neutral,
            1..=1,
            false,
            options,
            DecisionContinuation::BattlefieldEntryScalarChoice {
                context,
                choice,
                choices,
            },
        );
    }

    pub(super) fn entry_scalar_choices(
        &self,
        player: PlayerId,
        choice: BattlefieldEntryScalarChoiceDef,
    ) -> (&'static str, Vec<String>) {
        let (prompt, mut choices, fallback) = match choice.list {
            ScalarChoiceListDef::CardNames | ScalarChoiceListDef::NonlandCardNames => {
                let nonland_only = choice.list == ScalarChoiceListDef::NonlandCardNames;
                let mut names = self
                    .catalog
                    .definitions()
                    .into_iter()
                    .filter(|definition| definition.debut_set != CardSet::Token)
                    .flat_map(|definition| definition.parts.iter())
                    // A split card is nameable half by half, so the land test
                    // belongs to the part rather than to the whole card.
                    .filter(|part| !nonland_only || !part.rules.types().contains(CardType::Land))
                    .map(|part| part.name.clone())
                    .collect::<Vec<_>>();
                names.sort();
                names.dedup();
                (
                    if nonland_only {
                        "Choose a nonland card name"
                    } else {
                        "Choose a card name"
                    },
                    names,
                    "Black Lotus",
                )
            }
            ScalarChoiceListDef::CreatureTypes => (
                "Choose a creature type",
                self.creature_type_choices(player),
                "Human",
            ),
        };
        // A deliberately tiny catalog must not strand an entry procedure.
        if choices.is_empty() {
            choices.push(fallback.into());
        }
        (prompt, choices)
    }

    pub(super) fn activate_mana_source(
        &mut self,
        player: PlayerId,
        source: GameObjectId,
        ability: AbilityOrigin,
        color: ManaColor,
        choices: ManaActivationChoices,
    ) {
        let activation = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
            .and_then(|permanent| self.mana_ability_activation(permanent, ability, color, choices))
            .expect("legal mana action references a mana source");
        let produced_mana = Self::mana_for_activation(&activation);
        // Counted for the same reason an ordinary activation is: a printed
        // "only once each turn" is read off this tally when the ability is
        // next offered.
        if let Some(permanent) = self
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == source)
        {
            match permanent
                .activations_this_turn
                .iter_mut()
                .find(|(origin, _)| *origin == ability)
            {
                Some((_, count)) => *count = count.saturating_add(1),
                None => permanent.activations_this_turn.push((ability, 1)),
            }
        }
        for cost in activation.costs.as_slice() {
            match cost {
                AbilityCostDef::TapSource => {
                    // The tap transition carries its purpose, so ordinary
                    // tap triggers and mana-tap triggers scan one event.
                    let _ = self.tap_permanent_for_mana(source);
                }
                // The open-ended removal never arrives: enumeration sized it
                // before the activation was built. The two sacrifices and the
                // exile are deferred to the batch below, so that a Goblin
                // sacrificing itself leaves the battlefield once.
                AbilityCostDef::SacrificeSource
                | AbilityCostDef::ReturnSourceToHand
                | AbilityCostDef::ExileSource
                | AbilityCostDef::SacrificePermanent { .. }
                | AbilityCostDef::SacrificePermanents { .. }
                | AbilityCostDef::RemoveAnyNumberOfCountersFromSource(_) => {}
                AbilityCostDef::RemoveCountersFromSource { kind, amount } => {
                    self.battlefield
                        .iter_mut()
                        .find(|permanent| permanent.card.id == source)
                        .expect("a legal mana activation has its source")
                        .remove_counters(*kind, *amount);
                }
                AbilityCostDef::PayLife(amount) => {
                    self.lose_life(player, *amount);
                }
                AbilityCostDef::Mana(cost) => {
                    // Out of the pool, never by planning: the mana this
                    // ability is about to make is not available to pay for
                    // making it.
                    let _ = self.pay_player_cost(player, *cost, 0);
                }
                AbilityCostDef::DiscardSource
                | AbilityCostDef::UntapSource
                | AbilityCostDef::SacrificeObject(_)
                | AbilityCostDef::Loyalty(_)
                | AbilityCostDef::ExileCardsFromGraveyard { .. }
                | AbilityCostDef::DiscardCards(_)
                | AbilityCostDef::DiscardCardMatching(_)
                | AbilityCostDef::DiscardCardsAtRandom(_)
                | AbilityCostDef::ReturnUnblockedAttackerToHand
                | AbilityCostDef::TapPermanent { .. }
                | AbilityCostDef::Special(_) => {
                    unreachable!("unsupported mana-ability costs are not enumerated")
                }
            }
        }
        if activation.costs.contains(&AbilityCostDef::ExileSource) {
            self.exile_permanent(source);
        } else {
            // The source's own sacrifice and a named permanent's are the same
            // exit, so they go in one batch. Skirk Prospector sacrificing
            // itself names its own id here, and the batch holds it once.
            let mut sacrificed = Vec::new();
            if activation.costs.contains(&AbilityCostDef::SacrificeSource) {
                sacrificed.push(source);
            }
            if let Some(chosen) = activation.cost_object
                && !sacrificed.contains(&chosen)
            {
                sacrificed.push(chosen);
            }
            if !sacrificed.is_empty() {
                self.move_permanents_to_graveyard_then(
                    &sacrificed,
                    Some(BattlefieldExitCompletion::CompleteManaAbility {
                        player,
                        activation,
                        produced_mana,
                    }),
                );
                return;
            }
        }
        self.complete_mana_ability(player, &activation, produced_mana);
    }

    /// Which alternative cast, if any, the chosen play option and paid costs
    /// amount to. Read while the card is still in the zone it is cast from,
    /// which is why it takes the player rather than the stack object.
    /// How a spell still on the stack was cast, if it was cast some
    /// alternative way. Read off the signature rather than from a permanent,
    /// because a spell that has not resolved has no permanent yet.
    /// Whether this player could cast a sorcery right now (CR 307.1): their
    /// own main phase, with an empty stack.
    pub(super) fn sorcery_speed_available(&self, player: PlayerId) -> bool {
        player == self.active_player && self.step.is_main() && self.stack.is_empty()
    }

    /// Which alternative a spell was cast with, read off the spell object.
    ///
    /// A "when you cast this spell, if it was kicked" trigger asks while the
    /// spell is still on the stack. The spell's own resolution asks after it
    /// has left, so the retired record answers there -- the signature is the
    /// same either way.
    pub(super) fn stack_object_cast_with(
        &self,
        object: GameObjectId,
    ) -> Option<AlternativeCastKindDef> {
        let stack_object = self
            .stack
            .iter()
            .find(|candidate| candidate.id == object)
            .or_else(|| match self.retired_objects.get(&object) {
                Some(super::RetiredObject::Stack(retired)) => Some(retired.as_ref()),
                Some(super::RetiredObject::Card(_) | super::RetiredObject::Permanent { .. })
                | None => None,
            })?;
        let signature = stack_object.signature.as_ref()?;
        let definition = self
            .catalog
            .get(stack_object.card.definition.card_definition()?)?;
        let option = definition.play_option(signature.play_option())?;
        self.selected_alternative_kind(definition, option, object, signature.costs())
    }

    fn cast_alternative_kind(
        &self,
        player: PlayerId,
        card_id: GameObjectId,
        signature: &CastSignature,
    ) -> Option<AlternativeCastKindDef> {
        self.players[player.index()]
            .hand
            .iter()
            .chain(&self.players[player.index()].graveyard)
            .find(|card| card.id == card_id)
            .and_then(|card| self.catalog.get(card.definition))
            .and_then(|definition| {
                definition
                    .play_option(signature.play_option())
                    .map(|option| (definition, option))
            })
            .and_then(|(definition, option)| {
                self.selected_alternative_kind(definition, option, card_id, signature.costs())
            })
    }

    pub(super) fn complete_mana_ability(
        &mut self,
        player: PlayerId,
        activation: &super::ManaAbilityActivation,
        produced_mana: Vec<Mana>,
    ) {
        self.add_mana(player, produced_mana);
        if activation.effect.damage_to_controller > 0 {
            self.damage_target_from(
                Some(activation.source),
                Some(Target::Player(player)),
                activation.effect.damage_to_controller,
            );
        }
        // "If there are no mining counters on this land, sacrifice it."
        // Checked here because a mana ability resolves without the stack:
        // the land is gone by the time anyone could respond, and a counter
        // removed by anything other than this ability leaves it alone.
        if let Some(kind) = activation.effect.sacrifice_source_when_out_of
            && self.battlefield.iter().any(|permanent| {
                permanent.card.id == activation.source && permanent.counters(kind) == 0
            })
        {
            self.move_permanents_to_graveyard(&[activation.source]);
        }
        self.consecutive_passes = 0;
        self.events.push(GameEvent::ManaAdded {
            player,
            source: activation.source,
        });
    }

    /// Whether the chosen modes suit the play option: the right number, in
    /// ascending order, without repeats unless the card allows them, and all
    /// of them actually executable.
    /// The spell's own "as an additional cost to cast this spell, pay N
    /// life", if it prints one. An alternative cost replaces the mana cost
    /// rather than the additional one, so this is read whichever way the
    /// spell is being cast.
    pub(super) fn spell_life_cost(
        definition: &CardDefinition,
        option: &PlayOptionDef,
    ) -> Option<SpellLifeCostDef> {
        let (_, ability) = Self::spell_ability(definition, option)?;
        let DeclarativeAbilityDef::Spell(spell) = ability.definition else {
            return None;
        };
        spell.life_cost()
    }

    /// How much life a cast of this spell for `x` actually pays.
    pub(super) fn spell_life_payment(
        definition: &CardDefinition,
        option: &PlayOptionDef,
        x: u16,
    ) -> u16 {
        Self::spell_life_cost(definition, option).map_or(0, |cost| {
            if cost.amount_is_x {
                x
            } else {
                u16::from(cost.amount)
            }
        })
    }

    /// The largest X a "pay X life" cost can be paid at. A player may pay
    /// life only down to zero (CR 118.4), so their life total is the bound;
    /// paying none is always available.
    pub(super) fn maximum_x_for_life(&self, player: PlayerId) -> u16 {
        u16::try_from(self.players[player.index()].life.max(0)).unwrap_or(u16::MAX)
    }

    /// The life a cast owes for the way it was paid for: what the chosen
    /// alternative names, plus what the spell prints as an additional cost.
    fn cast_life_payment(
        &self,
        definition: CardDefinitionId,
        stack_object: &StackObject,
        x: u16,
    ) -> u16 {
        self.selected_alternative_life(definition, stack_object)
            + self
                .catalog
                .get(definition)
                .and_then(|definition| {
                    let option = stack_object
                        .signature
                        .as_ref()
                        .and_then(|signature| definition.play_option(signature.play_option()))?;
                    Some(Self::spell_life_payment(definition, option, x))
                })
                .unwrap_or(0)
    }

    /// The life a spell cast off the top of a library owes, read while it
    /// is still up there.
    fn library_top_life_for_cast(
        &self,
        player: PlayerId,
        card_id: GameObjectId,
        choices: &CastChoices,
    ) -> u16 {
        self.players[player.index()]
            .library
            .last()
            .filter(|top| top.id == card_id)
            .and_then(|top| {
                let definition = self.catalog.get(top.definition)?;
                let option = definition.play_option(choices.play_option())?;
                self.library_top_life_cost(top, player, option)
            })
            .unwrap_or(0)
    }

    pub(super) fn cast_spell(
        &mut self,
        player: PlayerId,
        card_id: GameObjectId,
        choices: &CastChoices,
        sacrifices: &[GameObjectId],
    ) {
        let (signature, cost, _behavior, source_zone) = self
            .validated_cast_signature(player, card_id, choices)
            .expect("validated casting choices remain valid while paying costs");
        let targets = signature.iter_targets().copied().collect::<Vec<_>>();
        let x = signature.x();
        let alternative_kind = self.cast_alternative_kind(player, card_id, &signature);
        // Both say the same thing about where the card goes afterwards:
        // exiled rather than buried, wherever it would otherwise have gone.
        let cast_via_flashback = matches!(
            alternative_kind,
            Some(AlternativeCastKindDef::Flashback | AlternativeCastKindDef::WithoutPayingManaCost)
        );
        let cast_face_down = alternative_kind == Some(AlternativeCastKindDef::FaceDown);
        let energy = self.exile_energy_cost(card_id, player).unwrap_or(0);
        // Read while the card is still on the library, which is the only
        // place the permission reaching it can be found.
        let library_top_life = if source_zone == CastSourceZone::LibraryTop {
            self.library_top_life_for_cast(player, card_id, choices)
        } else {
            0
        };
        let card = match source_zone {
            CastSourceZone::Hand => remove_card(&mut self.players[player.index()].hand, card_id),
            CastSourceZone::Graveyard => {
                // Cast out of a graveyard is a card leaving it, which the
                // clauses that ask about the turn have to see.
                self.note_card_left_graveyard(player);
                remove_card(&mut self.players[player.index()].graveyard, card_id)
            }
            CastSourceZone::Exile => {
                self.consume_exile_play_permission(card_id);
                // The card is in its owner's exile, which need not be the
                // exile of the player casting it.
                remove_card(&mut self.players[0].exile, card_id)
                    .or_else(|| remove_card(&mut self.players[1].exile, card_id))
            }
            CastSourceZone::LibraryTop => {
                remove_card(&mut self.players[player.index()].library, card_id)
            }
        }
        .expect("legal cast action references a card in its validated source zone");
        // Every outstanding grant applies to the same next sorcery, whatever
        // its timing, so consume them together based on the form actually cast.
        let cast_is_sorcery = self
            .catalog
            .get(card.definition)
            .and_then(|definition| {
                let option = definition.play_option(signature.play_option())?;
                Self::play_option_types(definition, option)
            })
            .is_some_and(|types| types.contains(CardType::Sorcery));
        if cast_is_sorcery {
            self.sorcery_flash_grants[player.index()] = 0;
        }
        // A spell is first proposed on the stack, then mana abilities may be
        // activated and costs are paid. The operation cannot fail after the
        // validated signature above, so keeping the provisional object local
        // gives mana spend riders a concrete destination without exposing a
        // half-paid spell to priority or trigger placement.
        let (card, _zone_change) = self.zone_change_card(card);
        let stack_id = card.id;
        let definition = card.definition;
        let frozen_spell_ability = self.frozen_spell_payload(definition, &signature);
        // Read now, because nothing afterwards can tell: by resolution the
        // step has usually moved on, and the stack is empty again.
        let cast_at_instant_speed = !self.sorcery_speed_available(player);
        let mut stack_object = StackObject {
            id: stack_id,
            kind: StackObjectKind::Spell,
            card: card.into(),
            source: None,
            ability: frozen_spell_ability,
            controller: player,
            signature: Some(signature),
            chosen_permanents: Vec::new(),
            applied_effects: Vec::new(),
            text_changes: Vec::new(),
            colors: None,
            cast_via_flashback,
            cast_at_instant_speed,
            cast_from_zone: Some(source_zone),
            cast_face_down,
            colors_of_mana_spent: crate::card::ColorSet::empty(),
            is_copy: false,
        };
        let payment_purpose = ManaPaymentPurpose::Spell {
            object: stack_id,
            definition,
            controller: player,
            form: stack_object
                .signature
                .as_ref()
                .expect("a spell has a cast signature")
                .form()
                .clone(),
        };
        // Life named by the chosen alternative, by the spell's own additional
        // cost, or by the permission that let it be cast off a library, is
        // paid alongside its mana, before the spell is finished on the stack.
        let life = self.cast_life_payment(definition, &stack_object, x) + library_top_life;
        if life > 0 {
            self.lose_life(player, life);
        }
        // Read before the permission is consumed above; spent here, where
        // every other cost for this cast is paid.
        if energy > 0 {
            self.spend_energy(player, energy);
        }
        self.activate_mana_for_cost_avoiding_for(player, cost, x, None, &payment_purpose);
        let spent_mana = self.pay_player_cost_for(player, cost, x, &payment_purpose);
        Self::apply_spent_mana_to_spell(&mut stack_object, &spent_mana);
        // Recorded whether or not this spell counts them: what paid for a
        // spell is a fact about the cast, and a clause that asks later has
        // nothing else to read it from.
        for mana in &spent_mana {
            if mana.color != ManaColor::Colorless {
                stack_object.colors_of_mana_spent =
                    stack_object.colors_of_mana_spent.with(mana.color);
            }
        }
        self.continue_spell_cast(stack_object, targets, sacrifices.to_vec());
    }

    /// The life the chosen alternative names, if the cast selected one.
    fn selected_alternative_life(&self, definition: CardDefinitionId, object: &StackObject) -> u16 {
        let Some(signature) = object.signature.as_ref() else {
            return 0;
        };
        let Some(selected) = signature.costs().alternative() else {
            return 0;
        };
        let Some(card) = self.catalog.get(definition) else {
            return 0;
        };
        let Some(option) = card.play_option(signature.play_option()) else {
            return 0;
        };
        Self::alternative_cast_clause(card, option, selected)
            .and_then(|(_, ability, _)| match ability.definition {
                DeclarativeAbilityDef::AlternativeCast(alternative) => Some(alternative.life),
                _ => None,
            })
            .unwrap_or(0)
    }

    /// How this spell's additional cost spends what it named. A cast that
    /// selected an alternative reads that alternative's cost; everything else
    /// reads the spell's own.
    fn spell_additional_cost_spend(&self, object: &StackObject) -> SpendModeDef {
        let Some(signature) = object.signature.as_ref() else {
            return SpendModeDef::ByZone;
        };
        let Some(card_definition) = object.card.definition.card_definition() else {
            return SpendModeDef::ByZone;
        };
        let Some(definition) = self.catalog.get(card_definition) else {
            return SpendModeDef::ByZone;
        };
        let Some(option) = definition.play_option(signature.play_option()) else {
            return SpendModeDef::ByZone;
        };
        if let Some(selected) = signature.costs().alternative()
            && let Some((_, ability, _)) =
                Self::alternative_cast_ability(definition, option, selected)
            && let DeclarativeAbilityDef::AlternativeCast(alternative) = ability.definition
            && let Some(cost) = alternative.additional_cost
        {
            return cost.spend;
        }
        definition
            .rules
            .ability_clauses()
            .iter()
            .find_map(|ability| match ability.definition {
                DeclarativeAbilityDef::Spell(spell) if ability.is_executable() => {
                    spell.additional_cost()
                }
                _ => None,
            })
            .map_or(SpendModeDef::ByZone, |cost| cost.spend)
    }

    pub(super) fn continue_spell_cast(
        &mut self,
        stack_object: StackObject,
        targets: Vec<Target>,
        mut remaining_sacrifices: Vec<GameObjectId>,
    ) {
        // The same list carries every object an additional cost spends. What
        // spending means is the cost's own business: by default the object's
        // zone decides, but a cost can say otherwise -- the free-spell cycle
        // returns its lands to hand rather than losing them.
        let spend = self.spell_additional_cost_spend(&stack_object);
        if spend == SpendModeDef::ReturnToHand {
            for spent in remaining_sacrifices.drain(..) {
                self.move_target_to_zone(
                    Target::Permanent(spent),
                    ZoneKind::Hand,
                    ZoneMoveCause::Effect {
                        controller: stack_object.controller,
                    },
                    None,
                    ZonePlacement::Top,
                );
            }
        }
        // One move for the whole cost, however many cards it spends out of a
        // graveyard, which is what "one or more cards" reads.
        let mut exiled_from_graveyard = Vec::new();
        while let Some(&spent) = remaining_sacrifices.first() {
            if self
                .battlefield
                .iter()
                .any(|permanent| permanent.card.id == spent)
            {
                break;
            }
            remaining_sacrifices.remove(0);
            let owner = stack_object.controller;
            if let Some(card) = remove_card(&mut self.players[owner.index()].graveyard, spent) {
                let (card, _zone_change) = self.zone_change_card(card);
                self.players[owner.index()].exile.push(card.clone());
                exiled_from_graveyard.push(card);
            } else if let Some(card) = remove_card(&mut self.players[owner.index()].hand, spent) {
                let definition = card.definition;
                let (card, _zone_change) = self.zone_change_card(card);
                let moved = card.id;
                if spend == SpendModeDef::Exile {
                    self.players[owner.index()].exile.push(card);
                } else {
                    self.put_card_into_graveyard(owner, card);
                    self.events.push(GameEvent::CardsDiscarded {
                        player: owner,
                        cards: vec![(moved, definition)],
                    });
                }
            }
        }
        if !exiled_from_graveyard.is_empty() {
            self.capture_cards_exiled(&exiled_from_graveyard, ZoneKind::Graveyard);
            self.note_card_left_graveyard(stack_object.controller);
        }
        if !remaining_sacrifices.is_empty() {
            let sacrificed = remaining_sacrifices.remove(0);
            self.move_permanents_to_graveyard_then(
                &[sacrificed],
                Some(BattlefieldExitCompletion::CompleteSpellCast {
                    object: Box::new(stack_object),
                    targets,
                    remaining_sacrifices,
                }),
            );
            return;
        }

        let player = stack_object.controller;
        let stack_id = stack_object.id;
        let definition = stack_object
            .card
            .definition
            .card_definition()
            .expect("a cast spell is backed by a card definition");
        let cast_event = self
            .stack_trigger_event_object(&stack_object)
            .expect("a cast spell has locked characteristics");
        self.stack.push(stack_object);
        self.consecutive_passes = 0;
        self.spells_cast_this_turn[player.index()] =
            self.spells_cast_this_turn[player.index()].saturating_add(1);
        // Kept for the targeting triggers below, which run after the cast
        // event has taken the list.
        let crime_targets = targets.clone();
        let mut targeted = Vec::new();
        for target in &targets {
            if let Target::Permanent(id) | Target::Card(id) = target
                && !targeted.contains(id)
            {
                targeted.push(*id);
            }
        }
        self.events.push(GameEvent::SpellCast {
            player,
            card: stack_id,
            definition,
            targets,
        });
        self.capture_battlefield_triggers(&CommittedTriggerEvent::SpellCast {
            object: cast_event.clone(),
        });
        self.capture_crime_triggers(player, &crime_targets);
        // "Whenever this becomes the target of a spell" fires here, where the
        // targets are locked in -- once per targeting spell however many of
        // its slots name the same permanent (CR 115.7c).
        for target in targeted {
            self.capture_battlefield_triggers(&CommittedTriggerEvent::BecameTargetOfSpell {
                target,
                object: cast_event.clone(),
            });
        }
        // The spell's own cast clause, which no battlefield listener carries.
        self.capture_own_cast_triggers(stack_id);
    }
}
