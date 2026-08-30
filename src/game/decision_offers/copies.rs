// Copying a stack object: the retarget decisions Fork, storm, and their
// relatives ask, and the chain that makes more than one copy.
//
// Split out of `decision_offers.rs` only to keep one file readable; these are
// ordinary members of the same `impl Game`. The paths and imports are the
// parent module's.

impl Game {
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
            for original in signature.targets() {
                let Some(slot) = slots.get(original.slot().index()) else {
                    return vec![signature.targets().to_vec()];
                };
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
                    let mut replacements = target_combinations(
                        &candidates,
                        original.targets().len(),
                    )
                    .into_iter()
                    .map(|targets| TargetSelection::new(original.slot(), targets))
                    .collect::<Vec<_>>();
                    // Copy effects may keep the original target even if it has
                    // since become illegal; normal resolution will then apply
                    // the usual target-legality rules to the copy.
                    replacements.push(original.clone());
                    replacements.sort_unstable_by_key(|selection| selection.targets().to_vec());
                    replacements.dedup();
                    for replacement in &replacements {
                        let mut selected = prefix.clone();
                        selected.push(replacement.clone());
                        combined.push(selected);
                    }
                }
                choices = combined;
            }
            return choices;
        }
        let slots = Self::target_slots_for(option, signature.modes());
        if Self::uses_legacy_behavior_targets(definition, option) {
            let Some(behavior) = Self::play_option_behavior(definition, option) else {
                return vec![signature.targets().to_vec()];
            };
            let mut choices = self
                .legal_target_lists(
                    behavior,
                    player,
                    Some(signature.iter_targets().count()),
                    spell.id,
                )
                .into_iter()
                .map(|targets| {
                    if targets.is_empty() {
                        Vec::new()
                    } else {
                        vec![TargetSelection::new(TargetSlotId(0), targets)]
                    }
                })
                .collect::<Vec<_>>();
            choices.push(signature.targets().to_vec());
            choices.sort_unstable_by_key(|targets| flatten_target_selections(targets));
            choices.dedup();
            return choices;
        }

        let mut choices = vec![Vec::new()];
        for original in signature.targets() {
            let Some(slot) = slots.iter().find(|slot| slot.id == original.slot()) else {
                return vec![signature.targets().to_vec()];
            };
            let mut replacements = target_combinations(
                &self.targets_matching(slot.predicate),
                original.targets().len(),
            )
            .into_iter()
            .map(|targets| TargetSelection::new(slot.id, targets))
            .collect::<Vec<_>>();
            replacements.push(original.clone());
            replacements.sort_unstable_by_key(|selection| selection.targets().to_vec());
            replacements.dedup();
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
        for selection in original {
            let Some(slot) = payload.target_defs.get(selection.slot().index()) else {
                return vec![original.to_vec()];
            };
            let mut combined = Vec::new();
            for prefix in &choices {
                let source = object.source.unwrap_or(object.id);
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
                let mut replacements =
                    target_combinations(&candidates, selection.targets().len())
                        .into_iter()
                        .map(|targets| TargetSelection::new(selection.slot(), targets))
                        .collect::<Vec<_>>();
                replacements.push(selection.clone());
                replacements.sort_unstable_by_key(|replacement| replacement.targets().to_vec());
                replacements.dedup();
                for replacement in &replacements {
                    let mut selected = prefix.clone();
                    selected.push(replacement.clone());
                    combined.push(selected);
                }
            }
            choices = combined;
        }
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
