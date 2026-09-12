impl Game {
    fn prepared_supplies_land_type_effect(&self, source: &Permanent) -> Option<bool> {
        if source.face_down.is_none()
            && source.active_copy_values().is_none()
            && let ObjectKind::Card(definition) = source.card.definition
            && let Some(program) = self
                .prepared_engine
                .static_program(definition, source.presented)
        {
            return Some(program.supplies_land_type_effect());
        }
        let program = self.prepared_static_program(Self::effective_rules_source(source))?;
        Some(
            program.supplies_land_type_effect()
                || source.active_copy_values().into_iter().any(|copy| {
                    copy.added_abilities.iter().any(|ability| {
                        let ability = ability.definition;
                        matches!(ability.definition, DeclarativeAbilityDef::Static(_))
                            && ability
                                .declarative_effect()
                                .is_some_and(Self::effect_contains_land_type_operation)
                    })
                }),
        )
    }
}

impl Game {
    pub(super) fn land_type_effect_sources<'a>(
        &'a self,
        prospective: Option<&'a Permanent>,
    ) -> Vec<(&'a Permanent, ContinuousEffectTimestamp)> {
        let mut sources = if let Some(sources) = self.prepared_land_type_sources() {
            sources
                .indices()
                .iter()
                .map(|index| {
                    let source = &self.battlefield[*index];
                    (source, source.timestamp)
                })
                .collect()
        } else {
            self.battlefield
                .iter()
                .filter(|source| self.supplies_land_type_effect(source))
                .map(|source| (source, source.timestamp))
                .collect::<Vec<_>>()
        };
        if let Some(prospective) = prospective
            && self.supplies_land_type_effect_uncached(prospective)
            && !sources
                .iter()
                .any(|(source, _)| source.card.id == prospective.card.id)
        {
            sources.push((prospective, self.prospective_continuous_effect_timestamp()));
        }
        sources
    }
}
