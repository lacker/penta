// Long because the effect vocabulary is wide, not because the function
// does several things: every arm is one variant walked the same way.
fn collect_program_ability_grants(
    program: AbilityProgramDef,
    grants: &mut Vec<&AbilityDef>,
    tokens: &mut Vec<TokenCharacteristics>,
    emblems: &mut Vec<EmblemCharacteristics>,
) {
    match program {
        AbilityProgramDef::Effects(effect) => {
            collect_ability_grants(effect, grants, tokens, emblems);
        }
        AbilityProgramDef::Replacement(effect) => {
            collect_replacement_ability_grants(effect, grants, tokens, emblems);
        }
    }
}

fn collect_ability_grants(
    effect: EffectDef,
    grants: &mut Vec<&AbilityDef>,
    tokens: &mut Vec<TokenCharacteristics>,
    emblems: &mut Vec<EmblemCharacteristics>,
) {
    match effect {
        EffectDef::InstallTrigger(trigger) => {
            collect_program_ability_grants(
                trigger.ability.effect.definition,
                grants,
                tokens,
                emblems,
            );
        }
        EffectDef::CreateOngoingEffect(ongoing) => {
            collect_program_ability_grants(
                ongoing.ability.effect.definition,
                grants,
                tokens,
                emblems,
            );
        }
        EffectDef::ConditionalStatic(conditional) => {
            collect_applied_ability_grants(conditional.then.effect, grants);
        }
        EffectDef::StaticApply { effect, .. }
        | EffectDef::Apply { effect, .. }
        | EffectDef::DealDamageAndApply {
            applied: effect, ..
        } => {
            collect_applied_ability_grants(effect, grants);
        }
        EffectDef::CreateToken { token, copy, .. } => match copy {
            Some(copy) => grants.extend(
                copy.exceptions
                    .added_abilities
                    .iter()
                    .filter_map(|addition| match addition {
                        CopyAbilityDef::This => None,
                        CopyAbilityDef::Ability(ability) => Some(*ability),
                    }),
            ),
            None => tokens.push(token),
        },
        EffectDef::CreateAttachedToken { token, .. } => {
            tokens.push(token);
        }
        EffectDef::CreateEmblem { emblem } => emblems.push(emblem),
        EffectDef::BecomeCopyOf { exceptions, .. } => grants.extend(
            exceptions
                .added_abilities
                .iter()
                .filter_map(|addition| match addition {
                    CopyAbilityDef::This => None,
                    CopyAbilityDef::Ability(ability) => Some(*ability),
                }),
        ),
        _ => {}
    }
    for child in crate::card::child_effects(effect) {
        collect_ability_grants(child, grants, tokens, emblems);
    }
}

fn collect_replacement_ability_grants(
    effect: ReplacementEffectDef,
    grants: &mut Vec<&AbilityDef>,
    tokens: &mut Vec<TokenCharacteristics>,
    emblems: &mut Vec<EmblemCharacteristics>,
) {
    match effect {
        ReplacementEffectDef::Sequence(effects) => {
            for effect in effects {
                collect_replacement_ability_grants(*effect, grants, tokens, emblems);
            }
        }
        ReplacementEffectDef::Perform(effect) => {
            collect_ability_grants(*effect, grants, tokens, emblems);
        }
        ReplacementEffectDef::Conditional {
            if_true, if_false, ..
        } => {
            for effect in if_true.iter().chain(if_false.iter()) {
                collect_replacement_ability_grants(*effect, grants, tokens, emblems);
            }
        }
        ReplacementEffectDef::PayOr {
            if_paid,
            if_declined,
            ..
        } => {
            for effect in if_paid.iter().chain(if_declined.iter()) {
                collect_replacement_ability_grants(*effect, grants, tokens, emblems);
            }
        }
        ReplacementEffectDef::ReplaceEventWithNothing
        | ReplacementEffectDef::MoveToZone(_)
        | ReplacementEffectDef::ModifyBattlefieldEntry(_)
        | ReplacementEffectDef::PlaceCountersOnMovedObject { .. }
        | ReplacementEffectDef::MultiplyEventAmount(_)
        | ReplacementEffectDef::AddToEventAmount(_)
        | ReplacementEffectDef::Choose(_)
        | ReplacementEffectDef::LookAtHand(_) => {}
        ReplacementEffectDef::CopyEntering { exceptions, .. } => grants.extend(
            exceptions
                .added_abilities
                .iter()
                .filter_map(|addition| match addition {
                    CopyAbilityDef::This => None,
                    CopyAbilityDef::Ability(ability) => Some(*ability),
                }),
        ),
    }
}

fn collect_applied_ability_grants(effect: AppliedEffectDef, grants: &mut Vec<&AbilityDef>) {
    match effect {
        AppliedEffectDef::Composite(effects) => {
            for effect in effects {
                collect_applied_ability_grants(*effect, grants);
            }
        }
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
            AbilityOperationDef::Add(ability),
        )) => grants.push(ability),
        AppliedEffectDef::Rule(_) | AppliedEffectDef::Characteristic(_) => {}
    }
}

fn program_ability_grant_sites(program: AbilityProgramDef) -> usize {
    match program {
        AbilityProgramDef::Effects(effect) => ability_grant_sites(effect),
        AbilityProgramDef::Replacement(effect) => replacement_ability_grant_sites(effect),
    }
}

fn ability_grant_sites(effect: EffectDef) -> usize {
    let direct = match effect {
        EffectDef::InstallTrigger(trigger) => {
            program_ability_grant_sites(trigger.ability.effect.definition)
        }
        EffectDef::CreateOngoingEffect(ongoing) => {
            program_ability_grant_sites(ongoing.ability.effect.definition)
        }
        EffectDef::IfFormat {
            then, otherwise, ..
        } => return ability_grant_sites(*then).max(ability_grant_sites(*otherwise)),
        EffectDef::ConditionalStatic(conditional) => {
            applied_ability_grant_sites(conditional.then.effect)
        }
        EffectDef::StaticApply { effect, .. }
        | EffectDef::Apply { effect, .. }
        | EffectDef::DealDamageAndApply {
            applied: effect, ..
        } => applied_ability_grant_sites(effect),
        EffectDef::BecomeCopyOf { exceptions, .. } => exceptions.added_abilities.len(),
        EffectDef::CreateToken {
            copy: Some(copy), ..
        } => copy.exceptions.added_abilities.len(),
        _ => 0,
    };
    crate::card::child_effects(effect)
        .into_iter()
        .map(ability_grant_sites)
        .fold(direct, usize::saturating_add)
}

fn replacement_ability_grant_sites(effect: ReplacementEffectDef) -> usize {
    match effect {
        ReplacementEffectDef::Sequence(effects) => effects
            .iter()
            .map(|effect| replacement_ability_grant_sites(*effect))
            .fold(0, usize::saturating_add),
        ReplacementEffectDef::Perform(effect) => ability_grant_sites(*effect),
        ReplacementEffectDef::Conditional {
            if_true, if_false, ..
        } => if_true
            .iter()
            .chain(if_false.iter())
            .map(|effect| replacement_ability_grant_sites(*effect))
            .fold(0, usize::saturating_add),
        ReplacementEffectDef::PayOr {
            if_paid,
            if_declined,
            ..
        } => if_paid
            .iter()
            .chain(if_declined.iter())
            .map(|effect| replacement_ability_grant_sites(*effect))
            .fold(0, usize::saturating_add),
        ReplacementEffectDef::ReplaceEventWithNothing
        | ReplacementEffectDef::MoveToZone(_)
        | ReplacementEffectDef::ModifyBattlefieldEntry(_)
        | ReplacementEffectDef::PlaceCountersOnMovedObject { .. }
        | ReplacementEffectDef::MultiplyEventAmount(_)
        | ReplacementEffectDef::AddToEventAmount(_)
        | ReplacementEffectDef::Choose(_)
        | ReplacementEffectDef::LookAtHand(_) => 0,
        ReplacementEffectDef::CopyEntering { exceptions, .. } => {
            exceptions.added_abilities.len()
        }
    }
}

fn applied_ability_grant_sites(effect: AppliedEffectDef) -> usize {
    match effect {
        AppliedEffectDef::Composite(effects) => effects
            .iter()
            .map(|effect| applied_ability_grant_sites(*effect))
            .fold(0, usize::saturating_add),
        AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
            AbilityOperationDef::Add(_),
        )) => 1,
        AppliedEffectDef::Rule(_) | AppliedEffectDef::Characteristic(_) => 0,
    }
}
