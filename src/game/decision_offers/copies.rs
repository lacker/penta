// Copying a stack object: the retarget decisions Fork, storm, and their
// relatives ask, and the chain that makes more than one copy.
//
// Split out of `decision_offers.rs` only to keep one file readable; these are
// ordinary members of the same `impl Game`. The paths and imports are the
// parent module's.

impl Game {
    pub(super) fn queue_stack_target_change(
        &mut self,
        chooser: PlayerId,
        stack_object: &StackObject,
        change: crate::card::StackTargetChangeDef,
        replacement: Option<Target>,
    ) {
        let original = stack_object.target_selections().to_vec();
        if original.is_empty() {
            return;
        }
        // Changing an existing object's targets does not change who controls
        // that object. Its printed "you control" restrictions and hexproof
        // checks therefore remain relative to its own controller, even when
        // another player is instructed to choose the new targets.
        let mut target_lists = self.copy_target_choices(stack_object, stack_object.controller);
        target_lists.retain(|candidate| match change {
            crate::card::StackTargetChangeDef::ChooseNew {
                optional,
                restriction,
            } => {
                let differences = target_differences(&original, candidate);
                (optional || !differences.is_empty())
                    && restriction.is_none_or(|restriction| {
                        differences
                            .iter()
                            .all(|(_, target)| self.target_matches(restriction, *target))
                    })
            }
            crate::card::StackTargetChangeDef::ReplaceOneWith(_) => replacement.is_some_and(
                |replacement| {
                    let differences = target_differences(&original, candidate);
                    differences.len() == 1 && differences[0].1 == replacement
                },
            ),
        });
        target_lists.sort_unstable_by_key(|targets| flatten_target_selections(targets));
        target_lists.dedup();
        if target_lists.is_empty() {
            return;
        }
        let options = target_lists
            .iter()
            .enumerate()
            .map(|(index, targets)| {
                let labels = flatten_target_selections(targets)
                    .iter()
                    .map(|target| self.target_label(chooser, *target))
                    .collect::<Vec<_>>()
                    .join(", ");
                DecisionOption {
                    id: u32::try_from(index).unwrap_or(u32::MAX),
                    label: if *targets == original {
                        "Keep current targets".into()
                    } else {
                        format!("Change targets to {labels}")
                    },
                    card: None,
                    members: Vec::new(),
                    ability_text: None,
                    zone: DecisionZone::None,
                }
            })
            .collect();
        self.queue_decision(
            chooser,
            "Choose new targets",
            DecisionVisibility::Private,
            DecisionPreference::Neutral,
            1..=1,
            false,
            options,
            DecisionContinuation::ChangeStackTargets {
                object: stack_object.id,
                target_lists,
            },
        );
    }

    pub(super) fn change_stack_targets(
        &mut self,
        object: GameObjectId,
        targets: &[TargetSelection],
    ) {
        let Some(index) = self.stack.iter().position(|candidate| candidate.id == object) else {
            return;
        };
        let old_targets = self.stack[index].declared_targets();
        if self.stack[index].replace_target_selections(targets).is_err() {
            return;
        }
        let new_targets = self.stack[index].declared_targets();
        let kind = self.stack[index].kind;
        let Some(event) = self.stack_object_event_object(&self.stack[index]) else {
            return;
        };
        let mut newly_targeted = Vec::new();
        let mut newly_targeted_players = Vec::new();
        for target in new_targets {
            if old_targets.contains(&target) {
                continue;
            }
            match target {
                Target::Permanent(id) | Target::Card(id) if !newly_targeted.contains(&id) => {
                    newly_targeted.push(id);
                }
                Target::Player(player) if !newly_targeted_players.contains(&player) => {
                    newly_targeted_players.push(player);
                }
                Target::Spell(_) | Target::Permanent(_) | Target::Card(_) | Target::Player(_) => {}
            }
        }
        for target in newly_targeted {
            let event = match kind {
                crate::game::StackObjectKind::Spell => {
                    crate::game::CommittedTriggerEvent::BecameTargetOfSpell {
                        target,
                        object: event.clone(),
                    }
                }
                crate::game::StackObjectKind::ActivatedAbility
                | crate::game::StackObjectKind::TriggeredAbility => {
                    crate::game::CommittedTriggerEvent::BecameTargetOfAbility {
                        target,
                        object: event.clone(),
                    }
                }
            };
            self.capture_battlefield_triggers(&event);
        }
        for player in newly_targeted_players {
            self.capture_battlefield_triggers(
                &crate::game::CommittedTriggerEvent::PlayerBecameTarget {
                    player,
                    object: event.clone(),
                },
            );
        }
    }

    /// The same, several times over. Each copy is targeted before the next is
    /// offered, which is what storm's "you may choose new targets for the
    /// copies" means: the copies are separate objects with separate choices.
    pub(super) fn queue_copy_decision_chain(
        &mut self,
        player: PlayerId,
        spell: StackObject,
        colors: Option<ColorSet>,
        retarget: bool,
        described: &str,
        copies: u16,
    ) {
        if copies == 0 {
            return;
        }
        let remaining = copies - 1;
        let original_selections = spell.signature.as_ref().map_or_else(
            || {
                spell
                    .ability
                    .as_ref()
                    .map(|ability| ability.targets.clone())
                    .unwrap_or_default()
            },
            |signature| signature.targets().to_vec(),
        );
        if !retarget {
            for _ in 0..copies {
                self.push_copy_with_colors(
                    spell.clone(),
                    player,
                    original_selections.clone(),
                    colors,
                );
            }
            return;
        }
        let target_lists = self.copy_target_choices(&spell, player);
        if original_selections.is_empty() {
            for _ in 0..copies {
                self.push_copy_with_colors(spell.clone(), player, Vec::new(), colors);
            }
            return;
        }
        let original_targets = spell.targets();
        let options = target_lists
            .iter()
            .enumerate()
            .map(|(index, targets)| DecisionOption {
                id: u32::try_from(index).unwrap_or(u32::MAX),
                label: if flatten_target_selections(targets) == original_targets {
                    "Keep original targets".into()
                } else {
                    let labels = flatten_target_selections(targets)
                        .iter()
                        .map(|target| self.target_label(player, *target))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("Copy with targets {labels}")
                },
                card: None,
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::None,
            })
            .collect();
        self.queue_decision(
            player,
            format!("Choose targets for {described}"),
            DecisionVisibility::Private,
            DecisionPreference::Neutral,
            1..=1,
            false,
            options,
            DecisionContinuation::CopyStackObject {
                colors,
                remaining,
                player,
                spell,
                target_lists,
            },
        );
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn copy_target_choices(
        &self,
        spell: &StackObject,
        player: PlayerId,
    ) -> Vec<Vec<TargetSelection>> {
        let original_selections = spell.signature.as_ref().map_or_else(
            || {
                spell
                    .ability
                    .as_ref()
                    .map(|ability| ability.targets.as_slice())
                    .unwrap_or_default()
            },
            |signature| signature.targets(),
        );
        if original_selections.is_empty() {
            return vec![Vec::new()];
        }
        let Some(card_definition) = spell.card.definition.card_definition() else {
            return self.copy_ability_target_choices(spell, player, original_selections);
        };
        let Some(definition) = self.catalog.get(card_definition) else {
            return vec![original_selections.to_vec()];
        };
        let Some(signature) = &spell.signature else {
            return self.copy_ability_target_choices(spell, player, original_selections);
        };
        let Some(option) = definition.play_option(signature.play_option()) else {
            return vec![signature.targets().to_vec()];
        };
        let declarative_slots = spell
            .ability
            .as_ref()
            .map(|ability| ability.target_defs.clone())
            .filter(|slots| !slots.is_empty())
            .or_else(|| {
                Self::spell_ability(definition, option).and_then(|(_, ability)| {
                    let DeclarativeAbilityDef::Spell(spell) = ability.definition else {
                        return None;
                    };
                    let spliced = self.spliced_clauses_of(signature.spliced())?;
                    Self::selected_spell_plan(spell, signature.modes(), &spliced)
                        .map(|plan| plan.target_defs)
                        .filter(|targets| !targets.is_empty())
                })
            });
        if let Some(slots) = declarative_slots {
            let context = spell
                .ability
                .as_ref()
                .map_or_else(TriggerContext::empty, |ability| ability.context.trigger);
            let mut choices = vec![Vec::new()];
            for (selection_index, original) in signature.targets().iter().enumerate() {
                let Some(slot) = slots.get(original.slot().index()) else {
                    return vec![signature.targets().to_vec()];
                };
                let original_candidates = Self::without_excluded_source(
                    slot,
                    spell.id,
                    self.ability_targets_matching_with_selections(
                        slot.predicate,
                        &signature.targets()[..selection_index],
                        player,
                        spell.id,
                        context,
                    ),
                );
                let mut combined = Vec::new();
                for prefix in &choices {
                    let candidates = Self::without_excluded_source(
                        slot,
                        spell.id,
                        self.ability_targets_matching_with_selections(
                            slot.predicate,
                            prefix,
                            player,
                            spell.id,
                            context,
                        ),
                    );
                    let replacements = target_replacements_preserving_unchanged(
                        original,
                        &candidates,
                        &original_candidates,
                    );
                    for replacement in &replacements {
                        let mut selected = prefix.clone();
                        selected.push(replacement.clone());
                        combined.push(selected);
                    }
                }
                choices = combined;
            }
            choices.sort_unstable_by_key(|targets| flatten_target_selections(targets));
            choices.dedup();
            return choices;
        }
        let slots = Self::target_slots_for(option, signature.modes());
        let mut choices = vec![Vec::new()];
        for original in signature.targets() {
            let Some(slot) = slots.iter().find(|slot| slot.id == original.slot()) else {
                return vec![signature.targets().to_vec()];
            };
            let candidates = self.targets_matching(slot.predicate);
            let replacements =
                target_replacements_preserving_unchanged(original, &candidates, &[]);
            let mut combined = Vec::new();
            for prefix in &choices {
                for replacement in &replacements {
                    let mut selected = prefix.clone();
                    selected.push(replacement.clone());
                    combined.push(selected);
                }
            }
            choices = combined;
        }
        choices.sort_unstable_by_key(|targets| flatten_target_selections(targets));
        choices.dedup();
        choices
    }

    fn copy_ability_target_choices(
        &self,
        object: &StackObject,
        player: PlayerId,
        original: &[TargetSelection],
    ) -> Vec<Vec<TargetSelection>> {
        let Some(payload) = object.ability.as_ref() else {
            return vec![original.to_vec()];
        };
        let mut choices = vec![Vec::new()];
        for (selection_index, selection) in original.iter().enumerate() {
            let Some(slot) = payload.target_defs.get(selection.slot().index()) else {
                return vec![original.to_vec()];
            };
            let source = object.source.unwrap_or(object.id);
            let original_candidates = Self::without_excluded_source(
                slot,
                source,
                self.ability_targets_matching_with_selections(
                    slot.predicate,
                    &original[..selection_index],
                    player,
                    source,
                    payload.context.trigger,
                ),
            );
            let mut combined = Vec::new();
            for prefix in &choices {
                let candidates = Self::without_excluded_source(
                    slot,
                    source,
                    self.ability_targets_matching_with_selections(
                        slot.predicate,
                        prefix,
                        player,
                        source,
                        payload.context.trigger,
                    ),
                );
                let replacements = target_replacements_preserving_unchanged(
                    selection,
                    &candidates,
                    &original_candidates,
                );
                for replacement in &replacements {
                    let mut selected = prefix.clone();
                    selected.push(replacement.clone());
                    combined.push(selected);
                }
            }
            choices = combined;
        }
        choices.sort_unstable_by_key(|targets| flatten_target_selections(targets));
        choices.dedup();
        choices
    }

    pub(super) fn push_copy(
        &mut self,
        spell: StackObject,
        player: PlayerId,
        targets: Vec<TargetSelection>,
    ) {
        self.push_copy_with_colors(spell, player, targets, None);
    }

    /// A copy effect may repaint what it copies, as Fork does. The override
    /// replaces the printed colours outright rather than adding to them.
    pub(super) fn push_copy_with_colors(
        &mut self,
        mut spell: StackObject,
        player: PlayerId,
        targets: Vec<TargetSelection>,
        colors: Option<ColorSet>,
    ) {
        spell.colors = colors;
        let copied_source = spell.source;
        match spell.kind {
            crate::game::StackObjectKind::Spell => {
                let definition = spell
                    .card
                    .definition
                    .card_definition()
                    .expect("a spell copy keeps its printed card definition");
                let card =
                    self.unbacked_object(definition, player, CharacteristicSource::Copy(definition));
                spell.id = card.id;
                spell.card = card.into();
                spell.source = None;
            }
            crate::game::StackObjectKind::ActivatedAbility
            | crate::game::StackObjectKind::TriggeredAbility => {
                let card = self.unbacked_ability_object(spell.presentation(), player);
                spell.id = card.id;
                spell.card = card;
                spell.source = copied_source;
            }
        }
        spell.controller = player;
        if let Some(ability) = &mut spell.ability {
            ability.targets.clone_from(&targets);
        }
        spell.signature = spell.signature.as_ref().map(|signature| {
            signature
                .copy_with_targets(targets)
                .expect("copy replacement retains target slots and cardinality")
        });
        // Effects attached by mana spent on the original spell are not
        // copiable values. The copy keeps printed static abilities through
        // its definition, but it was not paid for with that mana.
        spell.applied_effects.clear();
        // Text-changing effects are not copiable values.
        spell.text_changes.clear();
        // A copy was not cast and paid no costs. Keep copied casting choices
        // and payment-object references while clearing provenance and facts
        // about mana or life actually spent.
        spell.cast = spell.cast.as_ref().map(CastContext::for_spell_copy);
        spell.is_copy = true;
        // Published where the copy actually lands, so a clause reading "or
        // copy" sees the same object anything else on the stack would.
        let copied = (spell.kind == crate::game::StackObjectKind::Spell)
            .then(|| self.stack_trigger_event_object(&spell))
            .flatten();
        self.stack.push(spell);
        if let Some(object) = copied {
            self.capture_battlefield_triggers(&crate::game::CommittedTriggerEvent::SpellCopied {
                object,
            });
        }
    }
}

fn target_differences(
    original: &[TargetSelection],
    replacement: &[TargetSelection],
) -> Vec<(Target, Target)> {
    original
        .iter()
        .zip(replacement)
        .flat_map(|(original, replacement)| {
            original
                .targets()
                .iter()
                .copied()
                .zip(replacement.targets().iter().copied())
        })
        .filter(|(original, replacement)| original != replacement)
        .collect()
}

fn target_replacements_preserving_unchanged(
    original: &TargetSelection,
    legal: &[Target],
    legal_with_original_prefix: &[Target],
) -> Vec<TargetSelection> {
    let mut replacements = vec![Vec::new()];
    for original_target in original.targets() {
        let mut options = legal.to_vec();
        if !legal_with_original_prefix.contains(original_target) || legal.contains(original_target) {
            options.push(*original_target);
        }
        options.sort_unstable();
        options.dedup();
        let mut extended = Vec::new();
        for prefix in replacements {
            for target in options.iter().copied() {
                if !prefix.contains(&target) {
                    let mut selected = prefix.clone();
                    selected.push(target);
                    extended.push(selected);
                }
            }
        }
        replacements = extended;
    }
    replacements
        .into_iter()
        .filter_map(|targets| original.with_replaced_targets(targets))
        .collect()
}
