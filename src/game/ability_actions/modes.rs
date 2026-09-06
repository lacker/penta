impl Game {
    /// Every way of answering an activated ability's "choose one --". An
    /// ability that prints no modes has exactly one answer: choose none.
    pub(super) fn activated_mode_selections(definition: &ActivatedAbilityDef) -> Vec<Vec<ModeId>> {
        let Some(modal) = definition.modes else {
            return vec![Vec::new()];
        };
        let implemented = modal
            .modes
            .iter()
            .enumerate()
            .filter_map(|(index, _)| ModeId::from_index(index))
            .collect::<Vec<_>>();
        mode_id_selections(
            &implemented,
            usize::from(modal.minimum),
            usize::from(modal.maximum),
            modal.may_repeat,
        )
    }

    /// The targets and mode effects an activation with these modes carries.
    /// The ability's own targets come first, then each chosen mode's, which
    /// is the same flattening a modal spell uses.
    pub(super) fn selected_activated_plan(
        definition: &ActivatedAbilityDef,
        selected_modes: &[ModeId],
    ) -> Option<SelectedSpellPlan> {
        let Some(modal) = definition.modes else {
            return selected_modes.is_empty().then(|| SelectedSpellPlan {
                target_defs: definition.targets.to_vec(),
                mode_effects: Vec::new(),
            });
        };
        let mut target_defs = definition.targets.to_vec();
        let mut selected = selected_modes.to_vec();
        selected.sort_by_key(|mode| mode.index());
        let mut mode_effects = Vec::with_capacity(selected.len());
        for selected in selected {
            let mode = modal.modes.get(selected.index())?;
            let effect = mode.declarative_effect()?;
            let DeclarativeAbilityDef::Spell(mode_spell) = mode.definition else {
                return None;
            };
            let target_base = target_defs.len();
            target_defs.extend_from_slice(mode_spell.targets());
            mode_effects.push(ScopedEffect::at(effect, target_base));
        }
        Some(SelectedSpellPlan {
            target_defs,
            mode_effects,
        })
    }
}
