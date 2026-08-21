// Rebuilding a resolved continuous effect from its snapshot, and the set
// operations its layers carry.
//
// Split out of `wire.rs` only to keep one file readable; included textually,
// so the paths and imports here are the parent module's.

fn parse_resolved_continuous_effect(
    state: &ResolvedContinuousEffectSnapshot,
    catalog: &CardCatalog,
) -> Result<ResolvedContinuousEffect, String> {
    let source = AbilitySourceRef {
        object: GameObjectId(state.source.object),
        ability: ability_origin_from_snapshot(state.source.ability),
    };
    if !super::semantics::applied_effect_locator_matches_source(&state.definition, source) {
        return Err(
            "checkpoint resolved-effect locator disagrees with its source ability".to_owned(),
        );
    }
    let definition = catalog_applied_effect(catalog, &state.definition)
        .ok_or("checkpoint resolved-effect locator is absent from this catalog")?;
    Ok(ResolvedContinuousEffect {
        definition,
        source,
        timestamp: ContinuousEffectTimestamp(state.timestamp),
        component_order: state.component_order,
        expiration: parse_expiration(state.expiration)?,
        kind: parse_resolved_operation(definition, &state.operation)?,
    })
}

fn parse_resolved_operation(
    definition: AppliedEffectDef,
    state: &ResolvedContinuousOperationSnapshot,
) -> Result<ResolvedContinuousEffectKind, String> {
    let mismatch = || {
        Err("checkpoint resolved-effect operation does not match its authored locator".to_owned())
    };
    match (definition, state) {
        (
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
                AbilityOperationDef::Add(ability),
            )),
            ResolvedContinuousOperationSnapshot::AbilityAdd { grant_id },
        ) => Ok(ResolvedContinuousEffectKind::Abilities(
            ResolvedAbilityOperation::Add {
                ability: *ability,
                grant: GrantId(*grant_id),
            },
        )),
        (
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
                AbilityOperationDef::Remove(predicate),
            )),
            ResolvedContinuousOperationSnapshot::AbilityRemove,
        ) => Ok(ResolvedContinuousEffectKind::Abilities(
            ResolvedAbilityOperation::Remove(predicate),
        )),
        (
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::BasicLandTypes(value)),
            ResolvedContinuousOperationSnapshot::BasicLandTypes { operation },
        ) => Ok(ResolvedContinuousEffectKind::BasicLandTypes(
            parse_set_operation(value, *operation)?,
        )),
        (
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::CardTypes(value)),
            ResolvedContinuousOperationSnapshot::CardTypes { operation },
        ) => Ok(ResolvedContinuousEffectKind::CardTypes(
            parse_set_operation(value, *operation)?,
        )),
        (
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::Colors(value)),
            ResolvedContinuousOperationSnapshot::Colors { operation },
        ) => Ok(ResolvedContinuousEffectKind::Colors(parse_set_operation(
            value, *operation,
        )?)),
        (
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::CreatureTypes(value)),
            ResolvedContinuousOperationSnapshot::CreatureTypes { operation },
        ) => Ok(ResolvedContinuousEffectKind::CreatureTypes(
            parse_set_operation(value, *operation)?,
        )),
        (
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::Subtypes(value)),
            ResolvedContinuousOperationSnapshot::Subtypes { operation },
        ) => Ok(ResolvedContinuousEffectKind::Subtypes(parse_set_operation(
            value, *operation,
        )?)),
        (AppliedEffectDef::Rule(rule), ResolvedContinuousOperationSnapshot::Rule) => {
            Ok(ResolvedContinuousEffectKind::Rule(rule))
        }
        (
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
                PowerToughnessOperationDef::SetBase { .. },
            )),
            ResolvedContinuousOperationSnapshot::SetBasePowerToughness { power, toughness },
        ) => Ok(ResolvedContinuousEffectKind::PowerToughness(
            ResolvedPowerToughnessOperation::SetBase {
                power: *power,
                toughness: *toughness,
            },
        )),
        (
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
                PowerToughnessOperationDef::SetBasePower(_),
            )),
            ResolvedContinuousOperationSnapshot::SetBasePower { power },
        ) => Ok(ResolvedContinuousEffectKind::PowerToughness(
            ResolvedPowerToughnessOperation::SetBasePower { power: *power },
        )),
        (
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
                PowerToughnessOperationDef::Modify { .. },
            )),
            ResolvedContinuousOperationSnapshot::ModifyPowerToughness { power, toughness },
        ) => Ok(ResolvedContinuousEffectKind::PowerToughness(
            ResolvedPowerToughnessOperation::Modify {
                power: *power,
                toughness: *toughness,
            },
        )),
        (
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
                PowerToughnessOperationDef::Switch,
            )),
            ResolvedContinuousOperationSnapshot::SwitchPowerToughness,
        ) => Ok(ResolvedContinuousEffectKind::PowerToughness(
            ResolvedPowerToughnessOperation::Switch,
        )),
        _ => mismatch(),
    }
}

fn parse_set_operation<T: Copy>(
    definition: SetOperationDef<T>,
    state: SetOperationSnapshot,
) -> Result<SetOperationDef<T>, String> {
    match (definition, state) {
        (SetOperationDef::Add(value), SetOperationSnapshot::Add) => Ok(SetOperationDef::Add(value)),
        (SetOperationDef::Remove(value), SetOperationSnapshot::Remove) => {
            Ok(SetOperationDef::Remove(value))
        }
        (SetOperationDef::Set(value), SetOperationSnapshot::Set) => Ok(SetOperationDef::Set(value)),
        _ => Err("checkpoint set operation does not match its authored locator".into()),
    }
}
