impl Game {
    pub(super) fn ability_resolver(
        origin: AbilityOrigin,
        ability: &AbilityDef,
    ) -> StackAbilityResolver {
        if let Some(resolver) = StackAbilityResolver::linked_cast_offer(ability) {
            return resolver;
        }
        if let Some(binding) = crate::card::ability_binding(origin, ability) {
            return StackAbilityResolver::CardOwned(binding.resolver());
        }
        if let Some(behavior) = ability.custom_behavior() {
            StackAbilityResolver::Custom(behavior)
        } else {
            let effect = match ability.declarative_effect() {
                Some(effect) => effect,
                None => EffectDef::None,
            };
            let scoped = ScopedEffect::primary(effect);
            match ability.definition {
                DeclarativeAbilityDef::Triggered(definition)
                    if definition.resolves_with_illegal_targets =>
                {
                    StackAbilityResolver::DeclarativeIgnoringTargetFizzle(scoped)
                }
                _ => StackAbilityResolver::Declarative(scoped),
            }
        }
    }

    pub(super) fn freeze_activated_ability(
        &self,
        permanent: &Permanent,
        origin: AbilityOrigin,
    ) -> FrozenActivatedAbility {
        let effective =
            self.find_effective_ability(permanent, |effective| effective.origin == origin);
        let fallback = Self::effective_rules_source(permanent);
        let presentation = Self::ability_presentation(origin, fallback);
        let text = effective.map(|effective| effective.ability.text);
        let definition = effective.map(|effective| Box::new(effective.ability));
        let (target_defs, resolver) = effective.map_or(
            (
                &[][..],
                StackAbilityResolver::Declarative(ScopedEffect::primary(EffectDef::None)),
            ),
            |effective| {
                let target_defs = match effective.ability.definition {
                    DeclarativeAbilityDef::Activated(definition) => definition.targets,
                    DeclarativeAbilityDef::Spell(_)
                    | DeclarativeAbilityDef::ActivatedMana(_)
                    | DeclarativeAbilityDef::TriggeredMana(_)
                    | DeclarativeAbilityDef::Triggered(_)
                    | DeclarativeAbilityDef::Static(_)
                    | DeclarativeAbilityDef::Replacement(_)
                    | DeclarativeAbilityDef::AlternativeCast(_)
                    | DeclarativeAbilityDef::OptionalAdditionalCost(_)
                    | DeclarativeAbilityDef::SpecialAction(_)
                    | DeclarativeAbilityDef::Keyword(_)
                    | DeclarativeAbilityDef::Legacy => &[],
                };
                (
                    target_defs,
                    Self::ability_resolver(effective.origin, &effective.ability),
                )
            },
        );
        FrozenActivatedAbility {
            origin,
            definition,
            presentation,
            text,
            target_defs: target_defs.to_vec(),
            resolver,
            // Both filled in by the activation, which is where X and the
            // modes are chosen.
            mode_effects: Vec::new(),
            x: 0,
        }
    }
}
