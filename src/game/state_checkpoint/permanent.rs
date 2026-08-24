#![allow(clippy::wildcard_imports)]

use super::*;

#[allow(clippy::too_many_lines)]
pub(super) fn permanent_snapshot(
    catalog: &CardCatalog,
    permanent: &Permanent,
) -> PermanentSnapshot {
    let resolved_continuous_effects = permanent
        .resolved_continuous_effects
        .iter()
        .filter_map(|effect| resolved_continuous_effect_snapshot(catalog, effect))
        .collect::<Vec<_>>();
    let has_unlocated_resolved_effect =
        resolved_continuous_effects.len() != permanent.resolved_continuous_effects.len();
    let copy_effect = permanent
        .copy_effect
        .as_ref()
        .and_then(|copy| copiable_characteristics_snapshot(catalog, copy));
    let has_unlocated_copy_ability = permanent.copy_effect.is_some()
        && copy_effect.as_ref().is_none_or(|(_, complete)| !complete);
    let double_faced_token_copy = permanent
        .double_faced_token_copy
        .as_ref()
        .and_then(|faces| {
            let (front, front_complete) = copiable_characteristics_snapshot(catalog, &faces.front)?;
            let (back, back_complete) = copiable_characteristics_snapshot(catalog, &faces.back)?;
            Some((
                DoubleFacedCopiableCharacteristicsSnapshot {
                    modal: faces.kind == DoubleFacedKind::Modal,
                    front_part_id: faces.front_part.0,
                    back_part_id: faces.back_part.0,
                    front,
                    back,
                },
                front_complete && back_complete,
            ))
        });
    let has_unlocated_double_faced_copy = permanent.double_faced_token_copy.is_some()
        && double_faced_token_copy
            .as_ref()
            .is_none_or(|(_, complete)| !complete);
    let token_characteristics = permanent
        .token_characteristics
        .and_then(|token| token_characteristics_locator(catalog, token));
    let has_unlocated_token_characteristics =
        permanent.token_characteristics.is_some() && token_characteristics.is_none();
    let copied_from = permanent
        .copied_from
        .and_then(|characteristics| object_characteristics_snapshot(catalog, characteristics));
    let has_unlocated_copied_from = permanent.copied_from.is_some() && copied_from.is_none();
    let face_down = permanent
        .face_down
        .and_then(face_down_characteristics_snapshot);
    let has_unlocated_face_down = permanent.face_down.is_some() && face_down.is_none();
    PermanentSnapshot {
        object_id: permanent.card.id.0,
        owner: permanent.card.owner.index(),
        object_kind: object_kind_snapshot(permanent.card.definition),
        token_characteristics,
        double_faced_token_copy: double_faced_token_copy.map(|(snapshot, _)| snapshot),
        face_down,
        turn_up_for_mana_cost: permanent.turn_up_for_mana_cost,
        presented_part_id: permanent.presented.0,
        timestamp: permanent.timestamp.0,
        entered_controller_turn: permanent.entered_controller_turn,
        entered_turn: permanent.entered_turn,
        detained_until_turn_of: permanent
            .detained_until_turn_of
            .map(|(player, turns)| (player.index(), turns)),
        destroy_at_end_of_combat: permanent.destroy_at_end_of_combat,
        skipped_untap_steps: permanent.skipped_untap_steps,
        control_reverts_to: permanent.control_reverts_to.map(PlayerId::index),
        control_source: permanent.control_source.map(|id| id.0),
        control_requires_source_tapped: permanent.control_requires_source_tapped,
        control_requires_source_attached: permanent.control_requires_source_attached,
        chosen_player: permanent.chosen_player.map(PlayerId::index),
        cast_x: permanent.cast_x,
        cast_from_zone: permanent.cast_from_zone.map(|zone| zone.label().to_owned()),
        cast_alternative: permanent
            .cast_alternative
            .map(|kind| kind.label().to_owned()),
        destroy_at_end: permanent.destroy_at_end,
        counters: permanent.counters.to_vec(),
        attached_to: permanent.attached_to.map(|id| id.0),
        reconfigured_timestamp: permanent
            .reconfigured_timestamp
            .map(|timestamp| timestamp.0),
        exile_instead_of_dying: permanent.exile_instead_of_dying,
        combat_damage_assignment: permanent
            .combat_damage_assignment
            .iter()
            .map(|assignment| CombatDamageAssignmentSnapshot {
                recipient: target_snapshot(assignment.recipient),
                amount: assignment.amount,
            })
            .collect(),
        regeneration_shields: permanent.regeneration_shields,
        attacked_this_turn: permanent.attacked_this_turn,
        exerted: permanent.exerted,
        saddled: permanent.saddled,
        exhausted: permanent
            .exhausted
            .iter()
            .copied()
            .map(super::ability_origin_snapshot)
            .collect(),
        last_attacked_turn: permanent
            .last_attacked_turn
            .map(|(player, turns)| (player.index(), turns)),
        attacks_this_turn: permanent.attacks_this_turn,
        damage_sources: permanent.damage_sources.iter().map(|id| id.0).collect(),
        was_dealt_damage_this_turn: permanent.was_dealt_damage_this_turn,
        dealt_damage_this_turn: permanent.dealt_damage_this_turn,
        paired_with: permanent.paired_with.map(|id| id.0),
        dealt_damage_to_opponent_this_turn: permanent.dealt_damage_to_opponent_this_turn,
        deathtouch_damage: permanent.deathtouch_damage,
        created_by: permanent.created_by.map(|id| id.0),
        temporary_keywords: permanent
            .temporary_keywords
            .iter()
            .copied()
            .map(keyword_snapshot)
            .collect(),
        keywords_until_upkeep_of: permanent
            .keywords_until_upkeep_of
            .iter()
            .map(|(player, keyword)| UpkeepKeywordSnapshot {
                seat: player.index(),
                keyword: keyword_snapshot(*keyword),
            })
            .collect(),
        resolved_continuous_effects,
        activations_this_turn: permanent
            .activations_this_turn
            .iter()
            .map(|(origin, count)| AbilityActivationSnapshot {
                origin: ability_origin_snapshot(*origin),
                count: *count,
            })
            .collect(),
        triggers_this_turn: permanent
            .triggers_this_turn
            .iter()
            .map(|(origin, count)| AbilityActivationSnapshot {
                origin: ability_origin_snapshot(*origin),
                count: *count,
            })
            .collect(),
        resolutions_this_turn: permanent
            .resolutions_this_turn
            .iter()
            .map(|(origin, count)| AbilityActivationSnapshot {
                origin: ability_origin_snapshot(*origin),
                count: *count,
            })
            .collect(),
        cast_at_instant_speed: permanent.cast_at_instant_speed,
        became_aura: permanent.became_aura,
        copy_effect: copy_effect.map(|(snapshot, _)| snapshot),
        copy_expiration: permanent.copy_expiration.map(expiration_snapshot),
        copied_from: copied_from.map(|characteristics| CopiedFromSnapshot { characteristics }),
        text_changes: permanent
            .text_changes
            .iter()
            .map(|change| model::BasicLandTypeChangeSnapshot {
                from: basic_land_type_snapshot(change.from),
                to: basic_land_type_snapshot(change.to),
            })
            .collect(),
        has_dynamic_characteristics: has_unlocated_resolved_effect
            || has_unlocated_copy_ability
            || has_unlocated_double_faced_copy
            || has_unlocated_token_characteristics
            || has_unlocated_copied_from
            || has_unlocated_face_down,
    }
}

fn copiable_characteristics_snapshot(
    catalog: &CardCatalog,
    copy: &CopiableCharacteristics,
) -> Option<(CopiableCharacteristicsSnapshot, bool)> {
    if matches!(copy.base, ObjectCharacteristics::Emblem { .. }) {
        return None;
    }
    let base = object_characteristics_snapshot(catalog, copy.base)?;
    let added_abilities = copy
        .added_abilities
        .iter()
        .filter_map(|ability| {
            Some(CopiableAbilitySnapshot {
                origin: ability_origin_snapshot(ability.origin),
                ability: ability_locator_for_origin(catalog, ability.origin, |candidate| {
                    *candidate == ability.definition
                })?,
            })
        })
        .collect::<Vec<_>>();
    let complete = added_abilities.len() == copy.added_abilities.len();
    Some((
        CopiableCharacteristicsSnapshot {
            base,
            added_types: CardType::ALL.map(|card_type| copy.added_types.contains(card_type)),
            base_power_toughness: copy
                .base_power_toughness
                .map(|(power, toughness)| [power, toughness]),
            colors: copy.colors.map(crate::card::ColorSet::to_flags),
            added_creature_types: copy
                .added_creature_types
                .iter()
                .map(|creature_type| (*creature_type).to_owned())
                .collect(),
            no_mana_cost: copy.no_mana_cost,
            added_abilities,
            retain_printed_subtypes: copy.retain_printed_subtypes,
        },
        complete,
    ))
}

fn resolved_continuous_effect_snapshot(
    catalog: &CardCatalog,
    effect: &ResolvedContinuousEffect,
) -> Option<ResolvedContinuousEffectSnapshot> {
    Some(ResolvedContinuousEffectSnapshot {
        definition: resolved_applied_effect_locator(catalog, effect.source, effect.definition)?,
        source: event::ability_source_snapshot(effect.source),
        timestamp: effect.timestamp.0,
        component_order: effect.component_order,
        expiration: expiration_snapshot(effect.expiration),
        operation: resolved_operation_snapshot(effect.definition, &effect.kind)?,
    })
}

fn resolved_operation_snapshot(
    definition: AppliedEffectDef,
    kind: &ResolvedContinuousEffectKind,
) -> Option<ResolvedContinuousOperationSnapshot> {
    match (definition, kind) {
        (
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
                AbilityOperationDef::Add(expected),
            )),
            ResolvedContinuousEffectKind::Abilities(ResolvedAbilityOperation::Add {
                ability,
                grant,
            }),
        ) if *expected == *ability => {
            Some(ResolvedContinuousOperationSnapshot::AbilityAdd { grant_id: grant.0 })
        }
        (
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::Abilities(
                AbilityOperationDef::Remove(expected),
            )),
            ResolvedContinuousEffectKind::Abilities(ResolvedAbilityOperation::Remove(actual)),
        ) if expected == *actual => Some(ResolvedContinuousOperationSnapshot::AbilityRemove),
        (
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::BasicLandTypes(expected)),
            ResolvedContinuousEffectKind::BasicLandTypes(actual),
        ) => matching_set_operation(expected, *actual)
            .map(|operation| ResolvedContinuousOperationSnapshot::BasicLandTypes { operation }),
        (
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::CardTypes(expected)),
            ResolvedContinuousEffectKind::CardTypes(actual),
        ) => matching_set_operation(expected, *actual)
            .map(|operation| ResolvedContinuousOperationSnapshot::CardTypes { operation }),
        (
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::Colors(expected)),
            ResolvedContinuousEffectKind::Colors(actual),
        ) => matching_set_operation(expected, *actual)
            .map(|operation| ResolvedContinuousOperationSnapshot::Colors { operation }),
        (
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::CreatureTypes(expected)),
            ResolvedContinuousEffectKind::CreatureTypes(actual),
        ) => matching_set_operation(expected, *actual)
            .map(|operation| ResolvedContinuousOperationSnapshot::CreatureTypes { operation }),
        (
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::Subtypes(expected)),
            ResolvedContinuousEffectKind::Subtypes(actual),
        ) => matching_set_operation(expected, *actual)
            .map(|operation| ResolvedContinuousOperationSnapshot::Subtypes { operation }),
        (
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
                PowerToughnessOperationDef::SetBase { .. },
            )),
            ResolvedContinuousEffectKind::PowerToughness(
                ResolvedPowerToughnessOperation::SetBase { power, toughness },
            ),
        ) => Some(ResolvedContinuousOperationSnapshot::SetBasePowerToughness {
            power: *power,
            toughness: *toughness,
        }),
        (
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
                PowerToughnessOperationDef::SetBasePower(_),
            )),
            ResolvedContinuousEffectKind::PowerToughness(
                ResolvedPowerToughnessOperation::SetBasePower { power },
            ),
        ) => Some(ResolvedContinuousOperationSnapshot::SetBasePower { power: *power }),
        (AppliedEffectDef::Rule(expected), ResolvedContinuousEffectKind::Rule(actual))
            if expected == *actual =>
        {
            Some(ResolvedContinuousOperationSnapshot::Rule)
        }
        (
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
                PowerToughnessOperationDef::Modify { .. },
            )),
            ResolvedContinuousEffectKind::PowerToughness(ResolvedPowerToughnessOperation::Modify {
                power,
                toughness,
            }),
        ) => Some(ResolvedContinuousOperationSnapshot::ModifyPowerToughness {
            power: *power,
            toughness: *toughness,
        }),
        (
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
                PowerToughnessOperationDef::Switch,
            )),
            ResolvedContinuousEffectKind::PowerToughness(ResolvedPowerToughnessOperation::Switch),
        ) => Some(ResolvedContinuousOperationSnapshot::SwitchPowerToughness),
        _ => None,
    }
}

fn matching_set_operation<T: Copy + Eq>(
    expected: SetOperationDef<T>,
    actual: SetOperationDef<T>,
) -> Option<SetOperationSnapshot> {
    (expected == actual).then(|| set_operation_snapshot(&expected))
}

fn set_operation_snapshot<T>(operation: &SetOperationDef<T>) -> SetOperationSnapshot {
    match operation {
        SetOperationDef::Add(_) => SetOperationSnapshot::Add,
        SetOperationDef::Remove(_) => SetOperationSnapshot::Remove,
        SetOperationDef::Set(_) => SetOperationSnapshot::Set,
    }
}

pub(super) fn detached_permanent_snapshot(
    catalog: &CardCatalog,
    permanent: &Permanent,
) -> DetachedPermanentSnapshot {
    DetachedPermanentSnapshot {
        state: permanent_snapshot(catalog, permanent),
        controller: permanent.controller.index(),
        tapped: permanent.tapped,
        damage: permanent.damage,
        attacking: permanent.attacking,
        attacking_band: permanent.attacking_band,
        attack_defender: permanent.attack_defender.map(|defender| match defender {
            AttackDefender::Player(player) => AttackDefenderSnapshot::Player {
                seat: player.index(),
            },
            AttackDefender::Planeswalker(object) => AttackDefenderSnapshot::Planeswalker {
                object_id: object.0,
            },
        }),
        blocked: permanent.blocked,
        blocking: permanent.blocking.iter().map(|id| id.0).collect(),
        blocking_this_combat: permanent.blocking_this_combat.then_some(true),
        activated_loyalty_this_turn: permanent.activated_loyalty_this_turn,
        chosen_creature_type: permanent.chosen_creature_type.clone(),
        chosen_basic_land_type: permanent
            .chosen_basic_land_type
            .map(basic_land_type_snapshot),
        chosen_card_name: permanent.chosen_card_name.clone(),
    }
}
