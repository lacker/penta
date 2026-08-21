// Copying a spell: the decisions Fork and its relatives ask, the chain that
// makes more than one copy, and the retarget offer each copy carries.
//
// Split out of `decision_offers.rs` only to keep one file readable; these are
// ordinary members of the same `impl Game`. The paths and imports are the
// parent module's.

impl Game {
    pub(super) fn queue_chain_lightning_decision(&mut self, player: PlayerId, spell: StackObject) {
        // Without RR to spend there is nothing to decide, and a prompt whose
        // only answer is "no" is worse than no prompt at all.
        if !self.can_pay_cost(player, ManaCost::new(0, 2), 0) {
            return;
        }
        let mut targets = self.damage_targets();
        if let Some(target) = spell.first_target()
            && !targets.contains(&target)
        {
            targets.push(target);
        }
        let mut options = vec![DecisionOption {
            id: 0,
            label: "Don't copy Chain Lightning".into(),
            card: None,
            members: Vec::new(),
            ability_text: None,
            zone: DecisionZone::None,
        }];
        options.extend(
            targets
                .iter()
                .enumerate()
                .map(|(index, target)| DecisionOption {
                    id: u32::try_from(index + 1).unwrap_or(u32::MAX),
                    label: format!(
                        "Copy Chain Lightning → {}",
                        self.target_label(player, *target)
                    ),
                    card: None,
                    members: Vec::new(),
                    ability_text: None,
                    zone: DecisionZone::None,
                }),
        );
        self.queue_decision(
            player,
            "Copy Chain Lightning?",
            DecisionVisibility::Private,
            DecisionPreference::Neutral,
            1..=1,
            false,
            options,
            DecisionContinuation::ChainLightning {
                player,
                spell,
                targets,
            },
        );
    }

    pub(super) fn queue_fork_decision(&mut self, player: PlayerId, spell: StackObject) {
        self.queue_copy_decision(player, spell, Some(FORK_COPY_COLOR), "Fork's copy");
    }

    /// Offers a copy of `spell` under `player`, letting them retarget it. Fork
    /// repaints what it copies and a card copying itself does not, so the
    /// colours are the caller's to decide.
    pub(super) fn queue_copy_decision(
        &mut self,
        player: PlayerId,
        spell: StackObject,
        colors: Option<ColorSet>,
        described: &str,
    ) {
        self.queue_copy_decision_chain(player, spell, colors, described, 1);
    }

    /// The same, several times over. Each copy is targeted before the next is
    /// offered, which is what storm's "you may choose new targets for the
    /// copies" means: the copies are separate objects with separate choices.
    pub(super) fn queue_copy_decision_chain(
        &mut self,
        player: PlayerId,
        spell: StackObject,
        colors: Option<ColorSet>,
        described: &str,
        copies: u16,
    ) {
        if copies == 0 {
            return;
        }
        let remaining = copies - 1;
        let target_lists = self.copy_target_choices(&spell, player);
        if spell
            .signature
            .as_ref()
            .is_some_and(|signature| signature.targets().is_empty())
        {
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
            DecisionContinuation::Fork {
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
        let Some(signature) = &spell.signature else {
            return Vec::new();
        };
        if signature.targets().is_empty() {
            return vec![Vec::new()];
        }
        let Some(card_definition) = spell.card.definition.card_definition() else {
            return vec![signature.targets().to_vec()];
        };
        let Some(definition) = self.catalog.get(card_definition) else {
            return vec![signature.targets().to_vec()];
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
                    Self::selected_spell_plan(spell, signature.modes())
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
                let mut replacements = target_combinations(
                    &self.ability_targets_matching(slot.predicate, player, spell.id, context),
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
            return choices;
        }
        let slots = Self::target_slots_for(option, signature.modes());
        if Self::uses_legacy_behavior_targets(definition, option) {
            let Some(behavior) = Self::play_option_behavior(definition, option) else {
                return vec![signature.targets().to_vec()];
            };
            let mut choices = self
                .legal_target_lists(behavior, player, Some(signature.iter_targets().count()))
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
        let definition = spell
            .card
            .definition
            .card_definition()
            .expect("a spell copy keeps its printed card definition");
        let card = self.unbacked_object(definition, player, CharacteristicSource::Copy(definition));
        spell.id = card.id;
        spell.card = card.into();
        spell.source = None;
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
        spell.phyrexian_symbols_paid_with_life = 0;
        spell.is_copy = true;
        self.stack.push(spell);
    }
}
