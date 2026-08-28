use super::super::{
    AbilitySourceRef, ApplicableBeginTurnReplacement, DecisionContinuation, DecisionOption,
    DecisionPreference, DecisionVisibility, DecisionZone, DeclarativeAbilityDef,
    DeferredBeginTurnEffect, EffectDef, Game, PlayerId, ReplacementConditionDef,
    ReplacementEffectDef, ReplacementEventDef, ScopedEffect, StackAbilityPayload,
    StackAbilityResolver, StackObject, StackObjectKind, TriggerContext, TurnKindDef, ZoneKind,
};

impl Game {
    fn take_next_turn_player(&mut self) -> (PlayerId, TurnKindDef) {
        if let Some(extra) = self.extra_turns.pop() {
            (extra, TurnKindDef::Extra)
        } else {
            let regular = self.next_regular_player;
            self.next_regular_player = regular.opponent();
            (regular, TurnKindDef::Regular)
        }
    }

    pub(in crate::game) fn start_next_turn(&mut self) {
        self.start_next_turn_with_deferred(Vec::new());
    }

    fn start_next_turn_with_deferred(&mut self, deferred: Vec<DeferredBeginTurnEffect>) {
        let (player, kind) = self.take_next_turn_player();
        self.continue_begin_turn(player, kind, Vec::new(), deferred);
    }

    fn continue_begin_turn(
        &mut self,
        player: PlayerId,
        kind: TurnKindDef,
        mut applied: Vec<AbilitySourceRef>,
        mut deferred: Vec<DeferredBeginTurnEffect>,
    ) {
        let replacements = self.applicable_begin_turn_replacements(player, kind, &applied);
        match replacements.as_slice() {
            [] => self.commit_next_turn(player, deferred),
            [replacement] if !replacement.optional => {
                applied.push(replacement.source);
                if Self::apply_begin_turn_replacement(replacement, &mut deferred) {
                    self.start_next_turn_with_deferred(deferred);
                } else {
                    self.continue_begin_turn(player, kind, applied, deferred);
                }
            }
            _ => self.queue_begin_turn_decision(player, kind, applied, replacements, deferred),
        }
    }

    fn applicable_begin_turn_replacements(
        &self,
        player: PlayerId,
        kind: TurnKindDef,
        applied: &[AbilitySourceRef],
    ) -> Vec<ApplicableBeginTurnReplacement> {
        let mut replacements = Vec::new();
        for permanent in &self.battlefield {
            self.for_each_effective_ability(permanent, |effective| {
                let ability = effective.ability;
                let DeclarativeAbilityDef::Replacement(definition) = ability.definition else {
                    return;
                };
                if !ability.is_executable() {
                    return;
                }
                let ReplacementEventDef::WouldBeginTurn {
                    player: relation,
                    kind: matching_kind,
                } = definition.event
                else {
                    return;
                };
                let Some(effect) = ability.declarative_replacement() else {
                    return;
                };
                let source = AbilitySourceRef {
                    object: permanent.card.id,
                    ability: effective.origin,
                };
                let condition_matches = match definition.condition {
                    None => true,
                    Some(ReplacementConditionDef::SourceTapped) => permanent.tapped,
                    Some(ReplacementConditionDef::CreatureDiedThisTurn) => {
                        self.creature_died_this_turn
                    }
                    // How a permanent's spell was paid for is a fact about
                    // the entry, so nothing about a turn beginning asks it.
                    // Hand and library sizes are likewise facts about draws.
                    Some(
                        ReplacementConditionDef::SourceCastWith(_)
                        | ReplacementConditionDef::SourceNotCastFrom(_)
                        | ReplacementConditionDef::ControllerHandAtMost(_)
                        | ReplacementConditionDef::ControllerLibraryEmpty,
                    ) => false,
                };
                if applied.contains(&source)
                    || !definition.source_zones.contains(&ZoneKind::Battlefield)
                    || !matching_kind.matches(kind)
                    || !self.player_relation_matches(
                        player,
                        relation,
                        permanent.controller,
                        TriggerContext {
                            event_player: Some(player),
                            ..TriggerContext::empty()
                        },
                    )
                    || !condition_matches
                    || !Self::begin_turn_replacement_effect_supported(effect)
                {
                    return;
                }
                replacements.push(ApplicableBeginTurnReplacement {
                    source,
                    controller: permanent.controller,
                    presentation: Self::ability_presentation(
                        effective.origin,
                        Self::effective_rules_source(permanent),
                    ),
                    text: ability.text,
                    optional: definition.optional,
                    effect,
                });
            });
        }
        replacements
    }

    fn queue_begin_turn_decision(
        &mut self,
        player: PlayerId,
        kind: TurnKindDef,
        applied: Vec<AbilitySourceRef>,
        replacements: Vec<ApplicableBeginTurnReplacement>,
        deferred: Vec<DeferredBeginTurnEffect>,
    ) {
        let can_begin = replacements.iter().all(|replacement| replacement.optional);
        let mut options = Vec::new();
        if can_begin {
            options.push(DecisionOption {
                id: 0,
                label: "Begin the turn".into(),
                card: None,
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::None,
            });
        }
        options.extend(replacements.iter().enumerate().map(|(index, replacement)| {
            let name = self
                .presentation_name(replacement.presentation)
                .unwrap_or_else(|| "the source".into());
            DecisionOption {
                id: u32::try_from(index + 1).expect("begin-turn replacement count fits u32"),
                label: format!("Apply {name}'s replacement effect"),
                card: Some((replacement.source.object, replacement.presentation)),
                members: Vec::new(),
                ability_text: Some(replacement.text.into()),
                zone: DecisionZone::Battlefield,
            }
        }));
        self.queue_decision(
            player,
            "A turn would begin",
            DecisionVisibility::Public,
            if can_begin {
                DecisionPreference::PreferOption(0)
            } else {
                DecisionPreference::Neutral
            },
            1..=1,
            false,
            options,
            DecisionContinuation::BeginTurn {
                player,
                kind,
                applied,
                replacements,
                deferred,
            },
        );
    }

    pub(in crate::game) fn choose_begin_turn(
        &mut self,
        player: PlayerId,
        kind: TurnKindDef,
        mut applied: Vec<AbilitySourceRef>,
        replacements: &[ApplicableBeginTurnReplacement],
        mut deferred: Vec<DeferredBeginTurnEffect>,
        option: u32,
    ) {
        if option == 0 && replacements.iter().all(|replacement| replacement.optional) {
            self.commit_next_turn(player, deferred);
            return;
        }
        let Some(index) = option
            .checked_sub(1)
            .and_then(|index| usize::try_from(index).ok())
        else {
            return;
        };
        let Some(replacement) = replacements.get(index).copied() else {
            return;
        };
        applied.push(replacement.source);
        if Self::apply_begin_turn_replacement(&replacement, &mut deferred) {
            self.start_next_turn_with_deferred(deferred);
        } else {
            self.continue_begin_turn(player, kind, applied, deferred);
        }
    }

    const fn begin_turn_replacement_effect_supported(effect: ReplacementEffectDef) -> bool {
        match effect {
            ReplacementEffectDef::ReplaceEventWithNothing => true,
            ReplacementEffectDef::Perform(effect) => matches!(
                *effect,
                EffectDef::Untap {
                    object: crate::card::EffectRecipientDef::Source,
                }
            ),
            ReplacementEffectDef::Sequence(effects) => {
                let mut index = 0;
                while index < effects.len() {
                    if !Self::begin_turn_replacement_effect_supported(effects[index]) {
                        return false;
                    }
                    index += 1;
                }
                true
            }
            ReplacementEffectDef::ModifyBattlefieldEntry(_)
            | ReplacementEffectDef::MoveToZone(_)
            | ReplacementEffectDef::Conditional { .. }
            | ReplacementEffectDef::PayOr { .. }
            | ReplacementEffectDef::PlaceCountersOnMovedObject { .. }
            | ReplacementEffectDef::MultiplyEventAmount(_)
            | ReplacementEffectDef::AddToEventAmount(_)
            | ReplacementEffectDef::Choose(_)
            | ReplacementEffectDef::LookAtHand(_)
            | ReplacementEffectDef::CopyEntering { .. } => false,
        }
    }

    fn apply_begin_turn_replacement(
        replacement: &ApplicableBeginTurnReplacement,
        deferred: &mut Vec<DeferredBeginTurnEffect>,
    ) -> bool {
        Self::apply_begin_turn_replacement_effect(replacement, replacement.effect, deferred)
    }

    fn apply_begin_turn_replacement_effect(
        replacement: &ApplicableBeginTurnReplacement,
        effect: ReplacementEffectDef,
        deferred: &mut Vec<DeferredBeginTurnEffect>,
    ) -> bool {
        match effect {
            ReplacementEffectDef::Sequence(effects) => {
                let mut replaced = false;
                for effect in effects {
                    replaced |=
                        Self::apply_begin_turn_replacement_effect(replacement, *effect, deferred);
                }
                replaced
            }
            ReplacementEffectDef::ReplaceEventWithNothing => true,
            ReplacementEffectDef::Perform(effect) => {
                deferred.push(DeferredBeginTurnEffect {
                    replacement: *replacement,
                    effect: *effect,
                });
                false
            }
            ReplacementEffectDef::ModifyBattlefieldEntry(_)
            | ReplacementEffectDef::MoveToZone(_)
            | ReplacementEffectDef::Conditional { .. }
            | ReplacementEffectDef::PayOr { .. }
            | ReplacementEffectDef::PlaceCountersOnMovedObject { .. }
            | ReplacementEffectDef::MultiplyEventAmount(_)
            | ReplacementEffectDef::AddToEventAmount(_)
            | ReplacementEffectDef::Choose(_)
            | ReplacementEffectDef::LookAtHand(_)
            | ReplacementEffectDef::CopyEntering { .. } => false,
        }
    }

    pub(super) fn perform_deferred_begin_turn_effects(
        &mut self,
        player: PlayerId,
        deferred: Vec<DeferredBeginTurnEffect>,
    ) {
        for deferred in deferred {
            self.perform_begin_turn_replacement_effect(
                &deferred.replacement,
                deferred.effect,
                player,
            );
        }
    }

    fn perform_begin_turn_replacement_effect(
        &mut self,
        replacement: &ApplicableBeginTurnReplacement,
        effect: EffectDef,
        player: PlayerId,
    ) {
        let Some(permanent) = self
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == replacement.source.object)
        else {
            return;
        };
        let object = StackObject {
            id: replacement.source.object,
            kind: StackObjectKind::TriggeredAbility,
            card: permanent.card.clone(),
            source: Some(replacement.source.object),
            ability: Some(StackAbilityPayload {
                origin: replacement.source.ability,
                definition: None,
                presentation: replacement.presentation,
                text: Some(replacement.text),
                target_defs: Vec::new(),
                targets: Vec::new(),
                context: TriggerContext {
                    event_player: Some(player),
                    ..TriggerContext::empty()
                }
                .into(),
                resolver: StackAbilityResolver::Declarative(ScopedEffect::primary(effect)),
                condition: None,
                mode_effects: Vec::new(),
                resolution_destination: None,
                x: 0,
                sacrificed_mana_value: 0,
            }),
            controller: replacement.controller,
            signature: None,
            chosen_permanents: Vec::new(),
            applied_effects: Vec::new(),
            text_changes: Vec::new(),
            colors: None,
            cast_via_flashback: false,
            cast_via_suspend: false,
            cast_at_instant_speed: false,
            cast_from_zone: None,
            face_down: None,
            colors_of_mana_spent: crate::card::ColorSet::empty(),
            phyrexian_symbols_paid_with_life: 0,
            is_copy: false,
        };
        self.resolve_effect_def(
            ScopedEffect::primary(effect),
            &object,
            TriggerContext {
                event_player: Some(player),
                ..TriggerContext::empty()
            },
        );
    }

    /// Queues extra turns in active-player/nonactive-player order. The turn
    /// queue is a stack, so this also makes the nonactive player's turn the
    /// first one taken when one effect gives both players an extra turn.
    pub(in crate::game) fn schedule_extra_turns(
        &mut self,
        players: impl IntoIterator<Item = PlayerId>,
    ) {
        let mut affected = [false; 2];
        for player in players {
            affected[player.index()] = true;
        }
        for player in [self.active_player, self.active_player.opponent()] {
            if affected[player.index()] {
                self.extra_turns.push(player);
            }
        }
    }
}
