// Stable paths and clause provenance for suspended composed instructions.
// Included into semantics.rs to share its catalog-addressing vocabulary.

pub(super) fn scoped_effect_snapshot_in_catalog(
    catalog: &CardCatalog,
    root: &AbilityDef,
    effect: ScopedEffect,
) -> Option<ScopedEffectSnapshot> {
    let Some(origin) = effect.clause_origin else {
        return scoped_effect_snapshot(root, effect);
    };
    let locator = ability_locator_for_origin(catalog, origin, |ability| {
        scoped_effect_snapshot(ability, effect).is_some()
    })?;
    let ability = catalog_ability(catalog, &locator)?;
    let mut snapshot = scoped_effect_snapshot(&ability, effect)?;
    snapshot.clause_ability = Some(locator);
    snapshot.clause_origin = Some(super::ability_origin_snapshot(origin));
    Some(snapshot)
}

pub(super) fn scoped_effect_snapshot(
    ability: &AbilityDef,
    effect: ScopedEffect,
) -> Option<ScopedEffectSnapshot> {
    let mut path = Vec::new();
    let found = match ability.effect.definition {
        AbilityProgramDef::Effects(definition) => locate_effect(
            definition,
            crate::game::EffectLocalRules::default(),
            None,
            effect,
            &mut path,
        ),
        AbilityProgramDef::Replacement(replacement) => replacement_child_effects(replacement)
            .into_iter()
            .enumerate()
            .any(|(index, root)| {
                path.push(index);
                if locate_effect(
                    root,
                    crate::game::EffectLocalRules::default(),
                    None,
                    effect,
                    &mut path,
                ) {
                    true
                } else {
                    path.pop();
                    false
                }
            }),
    };
    if found {
        return Some(ScopedEffectSnapshot {
            clause_ability: None,
            clause_origin: None,
            ability_path: Vec::new(),
            path,
            target_base: effect.target_base,
        });
    }
    child_abilities(ability)
        .into_iter()
        .enumerate()
        .find_map(|(index, child)| {
            let mut snapshot = scoped_effect_snapshot(child, effect)?;
            snapshot.ability_path.insert(0, index);
            Some(snapshot)
        })
}

pub(super) fn catalog_scoped_effect(
    catalog: &CardCatalog,
    ability: &AbilityLocator,
    snapshot: &ScopedEffectSnapshot,
) -> Option<ScopedEffect> {
    let clause_origin = snapshot
        .clause_origin
        .map(super::ability_origin_from_snapshot);
    if snapshot.clause_ability.is_some() != clause_origin.is_some() {
        return None;
    }
    if let (Some(locator), Some(origin)) = (&snapshot.clause_ability, clause_origin)
        && !ability_locator_matches_origin(locator, origin)
    {
        return None;
    }
    if let Some(clause) = &snapshot.clause_ability {
        let same_part = matches!((ability, clause),
            (AbilityLocator::Card { definition: root, part_id: root_part, .. },
             AbilityLocator::Card { definition: child, part_id: child_part, .. })
            if root == child && root_part == child_part);
        if !same_part
            || !matches!(
                catalog_ability(catalog, ability)?.definition,
                DeclarativeAbilityDef::Spell(_)
            )
            || !matches!(
                catalog_ability(catalog, clause)?.definition,
                DeclarativeAbilityDef::Spell(_)
            )
        {
            return None;
        }
    }
    let mut ability =
        catalog_ability(catalog, snapshot.clause_ability.as_ref().unwrap_or(ability))?;
    for &index in &snapshot.ability_path {
        ability = **child_abilities(&ability).get(index)?;
    }
    let (mut effect, path) = match ability.effect.definition {
        AbilityProgramDef::Effects(effect) => (effect, snapshot.path.as_slice()),
        AbilityProgramDef::Replacement(replacement) => {
            let (&root, path) = snapshot.path.split_first()?;
            (*replacement_child_effects(replacement).get(root)?, path)
        }
    };
    let mut local_rules = crate::game::EffectLocalRules::default();
    let mut cost_parameter = None;
    for &index in path {
        if let EffectDef::WithCosts { costs, .. } = effect {
            cost_parameter = Some(costs);
        }
        if let EffectDef::WithRule { rule, .. } = effect {
            local_rules = local_rules.with(rule);
        }
        effect = *child_effects(effect).get(index)?;
    }
    Some(ScopedEffect {
        effect,
        target_base: snapshot.target_base,
        clause_origin,
        local_rules,
        cost_parameter,
    })
}
