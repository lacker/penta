impl Game {
    /// Resolves an ordinary activated clause by its printed order. Legacy
    /// aggregate definitions fall back to their historic primary identity;
    /// migrated multi-ability cards retain the exact clause chosen by the
    /// action (for example, Factory's animate and pump abilities).
    #[cfg(test)]
    pub(super) fn activated_ability_origin(
        &self,
        permanent: &Permanent,
        index: usize,
    ) -> AbilityOrigin {
        let mut activated_index = 0;
        self.find_effective_ability(permanent, |effective| {
            if !effective.ability.is_executable()
                || !matches!(
                    effective.ability.definition,
                    DeclarativeAbilityDef::Activated(_)
                )
            {
                return false;
            }
            let matches = activated_index == index;
            activated_index += 1;
            matches
        })
        .map_or_else(
            || match Self::effective_rules_source(permanent) {
                ObjectCharacteristics::Card { definition, part } => AbilityOrigin::Printed {
                    definition,
                    part,
                    ability: AbilityId::PRIMARY,
                },
                ObjectCharacteristics::Token { part, .. } => AbilityOrigin::Token {
                    part,
                    ability: AbilityId::PRIMARY,
                },
                ObjectCharacteristics::Emblem { .. } => AbilityOrigin::Emblem {
                    ability: AbilityId::PRIMARY,
                },
                ObjectCharacteristics::FaceDown { .. } => AbilityOrigin::FaceDown {
                    ability: AbilityId::PRIMARY,
                },
            },
            |effective| effective.origin,
        )
    }

    pub(super) fn permanent_types(&self, permanent: &Permanent) -> Option<CardTypeSet> {
        let mut types = self.effective_rules(permanent)?.types();
        if let Some(copy) = permanent.active_copy_values() {
            types = types.union(copy.added_types);
        }
        let mut operations = permanent
            .resolved_continuous_effects
            .iter()
            .filter(|effect| self.resolved_continuous_effect_is_active(effect))
            .filter_map(|effect| match effect.kind {
                ResolvedContinuousEffectKind::CardTypes(operation) => {
                    Some((effect.timestamp, effect.component_order, operation))
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        if let Some(timestamp) = permanent.reconfigured_timestamp {
            operations.push((
                timestamp,
                u16::MAX,
                SetOperationDef::Remove(CardTypeSet::single(CardType::Creature)),
            ));
        }
        // Impending (CR 702.175a): a permanent cast for its impending cost is
        // not a creature until its last time counter is removed. Structural
        // like reconfigure above rather than a clause the card writes,
        // because it is a consequence of how the spell was paid for -- and
        // because it has to be true the instant the permanent arrives, with
        // no window in which anything could see a creature.
        if permanent.cast_alternative == Some(crate::card::AlternativeCastKindDef::Impending)
            && permanent.counters(crate::card::CounterKind::Time) > 0
        {
            operations.push((
                permanent.timestamp,
                u16::MAX,
                SetOperationDef::Remove(CardTypeSet::single(CardType::Creature)),
            ));
        }
        // Bestow (CR 702.103c): while a permanent cast for its bestow cost
        // stays attached it is an Aura enchantment and not a creature. The
        // same structural shape as impending above, and for the same reason.
        if Self::is_bestowed_aura(permanent) {
            operations.push((
                permanent.timestamp,
                u16::MAX,
                SetOperationDef::Remove(CardTypeSet::single(CardType::Creature)),
            ));
        }
        if let Some(_pass) = StaticSetCharacteristicLayerGuard::enter() {
            let result = self.visit_static_applied_effects(permanent, |applied| {
                if let AppliedEffectDef::Characteristic(CharacteristicOperationDef::CardTypes(
                    operation,
                )) = applied.effect
                {
                    operations.push((applied.timestamp, applied.component_order, operation));
                }
                ControlFlow::Continue(())
            });
            debug_assert!(result.is_continue());
        }
        operations.sort_by_key(|(timestamp, order, _)| (*timestamp, *order));
        for (_, _, operation) in operations {
            types = Self::apply_card_type_operation(types, operation);
        }
        Some(types)
    }

    /// Card types below live static type-changing effects. The land-type
    /// dependency walk uses this for its `HasType(Land)` recipient gate:
    /// supported static card-type effects only add Creature, so they cannot
    /// change that answer, and re-entering their full traversal here would
    /// recursively rebuild the Blood Moon source set.
    pub(super) fn permanent_types_below_static_effects(
        &self,
        permanent: &Permanent,
    ) -> Option<CardTypeSet> {
        let _pass = StaticSetCharacteristicLayerGuard::enter();
        self.permanent_types(permanent)
    }

    fn apply_card_type_operation(
        current: CardTypeSet,
        operation: SetOperationDef<CardTypeSet>,
    ) -> CardTypeSet {
        match operation {
            SetOperationDef::Add(types) => current.union(types),
            SetOperationDef::Set(types) => types,
            SetOperationDef::Remove(removed) => CardType::ALL
                .into_iter()
                .filter(|card_type| current.contains(*card_type) && !removed.contains(*card_type))
                .fold(CardTypeSet::empty(), CardTypeSet::with),
        }
    }

    /// The colours a permanent actually is after layer-5 operations.
    pub(super) fn effective_colors(&self, permanent: &Permanent, rules: &CardRules) -> [bool; 5] {
        let mut colors = rules.color_set();
        let mut operations = permanent
            .resolved_continuous_effects
            .iter()
            .filter(|effect| self.resolved_continuous_effect_is_active(effect))
            .filter_map(|effect| match effect.kind {
                ResolvedContinuousEffectKind::Colors(operation) => {
                    Some((effect.timestamp, effect.component_order, operation))
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        if let Some(_pass) = StaticSetCharacteristicLayerGuard::enter() {
            let result = self.visit_static_applied_effects(permanent, |applied| {
                if let AppliedEffectDef::Characteristic(CharacteristicOperationDef::Colors(
                    operation,
                )) = applied.effect
                {
                    operations.push((applied.timestamp, applied.component_order, operation));
                }
                ControlFlow::Continue(())
            });
            debug_assert!(result.is_continue());
        }
        operations.sort_by_key(|(timestamp, order, _)| (*timestamp, *order));
        for (_, _, operation) in operations {
            colors = Self::apply_color_operation(colors, operation);
        }
        colors.to_flags()
    }

    pub(super) fn apply_color_operation(
        current: ColorSet,
        operation: SetOperationDef<ColorSet>,
    ) -> ColorSet {
        let (included, excluded) = match operation {
            SetOperationDef::Add(added) => (Some(added), None),
            SetOperationDef::Remove(removed) => (None, Some(removed)),
            SetOperationDef::Set(set) => return set,
        };
        [
            ManaColor::White,
            ManaColor::Blue,
            ManaColor::Black,
            ManaColor::Red,
            ManaColor::Green,
        ]
        .into_iter()
        .filter(|color| {
            (current.contains(*color) || included.is_some_and(|set| set.contains(*color)))
                && !excluded.is_some_and(|set| set.contains(*color))
        })
        .fold(ColorSet::empty(), ColorSet::with)
    }

    /// Every colour question about a permanent goes through here, so a
    /// repainted one answers the same way to protection, to Aura legality,
    /// and to anything else that asks.
    pub(super) fn permanent_colors(&self, permanent: &Permanent) -> [bool; 5] {
        let Some(rules) = self.effective_rules(permanent) else {
            return [false; 5];
        };
        self.effective_colors(permanent, &rules)
    }

    /// Whether one source characteristic snapshot matches any protection
    /// quality the permanent currently has. The protection holder is the
    /// predicate source: qualities such as "the chosen player" read the
    /// choice recorded on that permanent, while the object being tested is
    /// the spell, ability source, blocker, Aura, or damage source.
    pub(super) fn is_protected_from_characteristics(
        &self,
        permanent: &Permanent,
        source: &TriggerEventObject,
        source_is_spell: bool,
    ) -> bool {
        self.find_effective_ability(permanent, |effective| {
            if !effective.ability.is_executable() {
                return false;
            }
            let DeclarativeAbilityDef::Keyword(KeywordAbility::ProtectionFrom(predicate)) =
                effective.ability.definition
            else {
                return false;
            };
            self.trigger_object_matches_for_controller(
                *predicate,
                source,
                permanent.card.id,
                source_is_spell,
                Some(permanent.controller),
            )
        })
        .is_some()
    }

    /// Every quality at once, for the sources that are whole objects rather
    /// than a bare color set.
    pub(super) fn is_protected_from_object(
        &self,
        permanent: &Permanent,
        source: GameObjectId,
        source_is_spell: bool,
    ) -> bool {
        self.protection_source_characteristics(source)
            .is_some_and(|characteristics| {
                self.is_protected_from_characteristics(
                    permanent,
                    &characteristics,
                    source_is_spell,
                )
            })
    }

    /// Characteristics for a protection source wherever the relevant event
    /// can leave it. Stack abilities borrow their source presentation, while
    /// a prospective spell still in hand uses its printed stack form.
    fn protection_source_characteristics(
        &self,
        source: GameObjectId,
    ) -> Option<TriggerEventObject> {
        if let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
        {
            return Some(self.trigger_event_object(permanent));
        }
        if let Some(stack) = self.stack.iter().find(|stack| stack.id == source) {
            return self.stack_object_event_object(stack);
        }
        if let Some(retired) = self.retired_objects.get(&source) {
            return match retired {
                RetiredObject::Permanent { permanent, .. } => {
                    Some(self.trigger_event_object(permanent))
                }
                RetiredObject::Stack(stack) => self.stack_object_event_object(stack),
                RetiredObject::Card(card) => self.printed_trigger_event_object(
                    source,
                    card.definition,
                    card.owner,
                    &CharacteristicContext::Graveyard,
                ),
            };
        }
        let (zone, card) = self.card_in_nonbattlefield_zone(source)?;
        let context = match zone {
            ZoneKind::Library => CharacteristicContext::Library,
            ZoneKind::Hand => CharacteristicContext::Hand,
            ZoneKind::Graveyard => CharacteristicContext::Graveyard,
            ZoneKind::Exile => CharacteristicContext::Exile,
            ZoneKind::Battlefield | ZoneKind::Stack | ZoneKind::Command => return None,
        };
        self.printed_trigger_event_object(source, card.definition, card.owner, &context)
    }

    /// The subtypes of any object, wherever it is. The companion of
    /// [`Self::object_colors`], and read the same way.
    #[cfg(test)]
    pub(super) fn object_subtypes(&self, object: GameObjectId) -> Vec<&'static str> {
        if let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == object)
        {
            return self.effective_subtypes(permanent).into_owned();
        }
        if let Some(stack) = self.stack.iter().find(|stack| stack.id == object) {
            return self
                .stack_trigger_event_object(stack)
                .map(|event| event.subtypes.into_owned())
                .unwrap_or_default();
        }
        match self.retired_objects.get(&object) {
            Some(RetiredObject::Permanent { permanent, .. }) => {
                self.effective_subtypes(permanent).into_owned()
            }
            Some(RetiredObject::Stack(stack)) => self
                .stack_trigger_event_object(stack)
                .map(|event| event.subtypes.into_owned())
                .unwrap_or_default(),
            Some(RetiredObject::Card(_)) | None => self
                .object_definition(object)
                .and_then(|definition| self.catalog.get(definition))
                .map(|definition| definition.rules.subtypes().to_vec())
                .unwrap_or_default(),
        }
    }

    pub(super) fn permanent_can_be_targeted_by(
        &self,
        permanent: &Permanent,
        controller: PlayerId,
        source: GameObjectId,
        source_is_spell: bool,
    ) -> bool {
        !(self.is_protected_from_object(permanent, source, source_is_spell)
            || self.permanent_has_executable_keyword(permanent, KeywordAbility::Shroud)
            || permanent.controller != controller
                && self.permanent_has_executable_keyword(permanent, KeywordAbility::Hexproof)
            || self.cannot_become_enchanted(permanent) && self.source_attaches_itself(source))
    }

    /// Whether the object doing the targeting is an Aura spell: one whose
    /// spell clause attaches the permanent the spell becomes. "Can't be
    /// enchanted" is a targeting restriction for those and nothing else, so a
    /// Shock aimed at the same permanent is unaffected.
    pub(super) fn source_attaches_itself(&self, source: GameObjectId) -> bool {
        let Some(definition) = self
            .object_definition(source)
            .and_then(|definition| self.catalog.get(definition))
        else {
            return false;
        };
        definition.parts.iter().any(|part| {
            part.rules.ability_clauses().iter().any(|ability| {
                ability.is_executable()
                    && matches!(ability.definition, DeclarativeAbilityDef::Spell(_))
                    && ability
                        .declarative_effect()
                        .is_some_and(Self::effect_attaches)
            })
        })
    }

    pub(super) fn effect_attaches(effect: EffectDef) -> bool {
        match effect {
            EffectDef::Attach { .. } => true,
            EffectDef::Sequence(effects) => effects.iter().copied().any(Self::effect_attaches),
            EffectDef::May { effect, .. } => Self::effect_attaches(*effect),
            _ => false,
        }
    }

    /// The card definition behind a game object, wherever it currently is.
    pub(super) fn object_definition(&self, object: GameObjectId) -> Option<CardDefinitionId> {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == object)
            .and_then(|permanent| permanent.card.definition.card_definition())
            .or_else(|| {
                self.stack
                    .iter()
                    .find(|stack| stack.id == object)
                    .and_then(|stack| stack.presentation().card_definition())
            })
            .or_else(|| {
                self.card_in_nonbattlefield_zone(object)
                    .map(|(_, card)| card.definition)
            })
            .or_else(|| {
                self.players
                    .iter()
                    .flat_map(|player| player.outside_game.iter())
                    .find(|card| card.id == object)
                    .map(|card| card.definition)
            })
            .or_else(|| match self.retired_objects.get(&object) {
                Some(RetiredObject::Card(card)) => Some(card.definition),
                Some(RetiredObject::Permanent { permanent, .. }) => {
                    permanent.card.definition.card_definition()
                }
                Some(RetiredObject::Stack(stack)) => stack.presentation().card_definition(),
                None => None,
            })
    }

    /// The expansion a game object's card was first printed in. A token has
    /// no printing, so it belongs to no expansion.
    pub(super) fn object_debut_set(&self, object: GameObjectId) -> Option<CardSet> {
        let definition = self.object_definition(object)?;
        self.catalog.get(definition).map(|card| card.debut_set)
    }

    pub(super) fn object_colors(&self, object: GameObjectId) -> [bool; 5] {
        if let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == object)
        {
            return self.permanent_colors(permanent);
        }
        if let Some(stack) = self.stack.iter().find(|stack| stack.id == object) {
            return stack.colors.map_or_else(
                || {
                    self.stack_trigger_event_object(stack)
                        .map_or([false; 5], |event| event.colors)
                },
                ColorSet::to_flags,
            );
        }
        if let Some(retired) = self.retired_objects.get(&object) {
            return match retired {
                RetiredObject::Permanent { permanent, .. } => self.permanent_colors(permanent),
                RetiredObject::Stack(stack) => self
                    .stack_trigger_event_object(stack)
                    .map_or([false; 5], |event| event.colors),
                RetiredObject::Card(card) => self
                    .catalog
                    .get(card.definition)
                    .map_or([false; 5], |definition| definition.rules.colors()),
            };
        }
        self.card_in_nonbattlefield_zone(object)
            .map(|(_, card)| card)
            .or_else(|| {
                self.players
                    .iter()
                    .flat_map(|player| player.outside_game.iter())
                    .find(|card| card.id == object)
            })
            .and_then(|card| self.catalog.get(card.definition))
            .map_or([false; 5], |definition| definition.rules.colors())
    }
    pub(super) fn combat_is_protected(&self, blocker: &Permanent, attacker: &Permanent) -> bool {
        self.is_protected_from_characteristics(attacker, &self.trigger_event_object(blocker), false)
    }
}
