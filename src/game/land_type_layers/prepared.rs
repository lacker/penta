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
                        ability.is_executable()
                            && matches!(ability.definition, DeclarativeAbilityDef::Static(_))
                            && ability
                                .declarative_effect()
                                .is_some_and(Self::effect_contains_land_type_operation)
                    })
                }),
        )
    }
}
