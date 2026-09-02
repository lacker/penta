// Splice, and the resolution plan a spliced spell resolves.
//
// Split from action enumeration because it answers what a cast turns into
// rather than which casts are on offer: which clauses the announced modes
// and the revealed splices contribute, in what order, and with whose
// targets. Included textually into `casting_actions.rs`, so the imports
// here are that module's.

impl Game {
    /// The clauses the cards spliced onto a spell contribute, in the order
    /// they were announced (CR 702.47a). A card is a legal splice only while
    /// it is in the caster's hand and prints both a splice cost and a spell
    /// clause to add.
    pub(in crate::game) fn spliced_spell_clauses(
        &self,
        player: PlayerId,
        spliced: &[GameObjectId],
    ) -> Option<Vec<AbilityDef>> {
        // In hand is where a splice is announced from, and the announcement
        // is what this checks; the clauses themselves are read the same way
        // wherever the cards are afterwards.
        if !spliced.iter().all(|card| {
            self.players[player.index()]
                .hand
                .iter()
                .any(|candidate| candidate.id == *card)
        }) {
            return None;
        }
        self.spliced_clauses_of(spliced)
    }

    /// The spell clauses of cards spliced onto a spell, looked up wherever
    /// those cards are: a copy of the spell carries the spliced text with it
    /// (CR 707.10), and the payload is rebuilt from the frozen signature
    /// rather than from a hand.
    pub(in crate::game) fn spliced_clauses_of(
        &self,
        spliced: &[GameObjectId],
    ) -> Option<Vec<AbilityDef>> {
        let mut clauses = Vec::with_capacity(spliced.len());
        for card in spliced {
            let (_, instance) = self.card_in_nonbattlefield_zone(*card)?;
            let definition = self.catalog.get(instance.definition)?;
            Self::splice_cost(definition)?;
            let option = definition.play_options.first()?;
            let (_, ability) = Self::spell_ability(definition, option)?;
            clauses.push(ability);
        }
        Some(clauses)
    }

    /// Every set of cards the caster may splice onto this spell, the empty
    /// one first (CR 702.47a). Only an Arcane spell may be spliced onto, and
    /// only a card in hand that prints a splice cost may be one of them.
    ///
    /// Enumerated over at most five eligible cards, which is far more than
    /// any real hand holds: the sets are a power set, and the bound is what
    /// keeps a pathological hand from becoming a pathological action list.
    fn splice_selections(
        &self,
        definition: &CardDefinition,
        player: PlayerId,
        cast: GameObjectId,
    ) -> Vec<Vec<GameObjectId>> {
        if !definition.rules.has_subtype("Arcane") {
            return vec![Vec::new()];
        }
        let eligible: Vec<GameObjectId> = self.players[player.index()]
            .hand
            .iter()
            // The spell being cast is on the stack by the time splice cards
            // are revealed, so a card can never be spliced onto itself
            // (CR 702.47a) however many splice clauses it prints.
            .filter(|held| held.id != cast)
            .filter(|held| {
                self.catalog
                    .get(held.definition)
                    .is_some_and(|definition| Self::splice_cost(definition).is_some())
            })
            .map(|held| held.id)
            .take(5)
            .collect();
        let mut selections = Vec::with_capacity(1 << eligible.len());
        for mask in 0..(1_u32 << eligible.len()) {
            selections.push(
                eligible
                    .iter()
                    .enumerate()
                    .filter(|(index, _)| mask & (1 << index) != 0)
                    .map(|(_, card)| *card)
                    .collect(),
            );
        }
        selections
    }

    /// What splicing all of these onto a spell costs together.
    pub(in crate::game) fn total_splice_cost(&self, spliced: &[GameObjectId]) -> ManaCost {
        spliced
            .iter()
            .filter_map(|card| self.card_in_nonbattlefield_zone(*card))
            .filter_map(|(_, instance)| self.catalog.get(instance.definition))
            .filter_map(Self::splice_cost)
            .fold(ManaCost::default(), add_mana_cost)
    }

    /// What splicing this card onto an Arcane spell costs, or `None` when it
    /// has no splice clause at all.
    pub(in crate::game) fn splice_cost(definition: &CardDefinition) -> Option<ManaCost> {
        definition
            .parts
            .iter()
            .flat_map(|part| part.rules.ability_clauses())
            .find_map(|ability| match ability.definition {
                DeclarativeAbilityDef::AlternativeCast(alternative)
                    if alternative.kind == AlternativeCastKindDef::Splice =>
                {
                    match alternative.mana_cost {
                        crate::card::AlternativeCastManaCostDef::Fixed(cost) => Some(cost),
                        crate::card::AlternativeCastManaCostDef::ThisCardManaCost => {
                            definition.rules.mana_cost()
                        }
                    }
                }
                _ => None,
            })
    }

    pub(super) fn selected_spell_plan(
        spell: crate::card::SpellAbilityDef,
        selected_modes: &[ModeId],
        spliced: &[AbilityDef],
    ) -> Option<SelectedSpellPlan> {
        let mut target_defs = spell.targets().to_vec();
        if target_defs.len() > usize::from(u8::MAX) + 1 {
            return None;
        }
        if spell.modal().is_none() {
            if !selected_modes.is_empty() {
                return None;
            }
            return Self::extend_plan_with_splices(target_defs, Vec::new(), spliced);
        }
        let mut selected = selected_modes.to_vec();
        selected.sort_by_key(|mode| mode.index());
        let mut mode_effects = Vec::with_capacity(selected.len());
        for selected in selected {
            let mode = spell.mode(selected)?;
            let effect = mode.declarative_effect()?;
            let DeclarativeAbilityDef::Spell(mode_spell) = mode.definition else {
                return None;
            };
            let target_base = target_defs.len();
            let target_count = mode_spell.targets().len();
            if target_base.checked_add(target_count)? > usize::from(u8::MAX) + 1 {
                return None;
            }
            target_defs.extend_from_slice(mode_spell.targets());
            mode_effects.push(ScopedEffect {
                effect,
                target_base,
            });
        }
        Self::extend_plan_with_splices(target_defs, mode_effects, spliced)
    }

    /// Adds each spliced clause's targets and effect to the plan, exactly
    /// the way a chosen mode adds its own: what is cast resolves the spell's
    /// instructions and then theirs, in the order they were announced.
    fn extend_plan_with_splices(
        mut target_defs: Vec<AbilityTargetDef>,
        mut mode_effects: Vec<ScopedEffect>,
        spliced: &[AbilityDef],
    ) -> Option<SelectedSpellPlan> {
        for clause in spliced {
            let DeclarativeAbilityDef::Spell(spell) = clause.definition else {
                return None;
            };
            let effect = clause.declarative_effect()?;
            let target_base = target_defs.len();
            if target_base.checked_add(spell.targets().len())? > usize::from(u8::MAX) + 1 {
                return None;
            }
            target_defs.extend_from_slice(spell.targets());
            mode_effects.push(ScopedEffect {
                effect,
                target_base,
            });
        }
        Some(SelectedSpellPlan {
            target_defs,
            mode_effects,
        })
    }
}
