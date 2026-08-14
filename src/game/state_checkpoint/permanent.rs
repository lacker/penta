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
    let copy_effect = permanent.copy_effect.as_ref().map(|copy| {
        let added_abilities = copy
            .added_abilities
            .iter()
            .filter_map(|ability| {
                Some(CopiableAbilitySnapshot {
                    origin: ability_origin_snapshot(ability.origin),
                    ability: ability_locator(catalog, |candidate| {
                        *candidate == ability.definition
                    })?,
                })
            })
            .collect::<Vec<_>>();
        let complete = added_abilities.len() == copy.added_abilities.len();
        (
            CopiableCharacteristicsSnapshot {
                definition: copy.base.0.0,
                part_id: copy.base.1.0,
                added_types: CardType::ALL.map(|card_type| copy.added_types.contains(card_type)),
                added_abilities,
            },
            complete,
        )
    });
    let has_unlocated_copy_ability = copy_effect.as_ref().is_some_and(|(_, complete)| !complete);
    PermanentSnapshot {
        object_id: permanent.card.id.0,
        owner: permanent.card.owner.index(),
        timestamp: permanent.timestamp.0,
        entered_controller_turn: permanent.entered_controller_turn,
        detained_until_turn_of: permanent
            .detained_until_turn_of
            .map(|(player, turns)| (player.index(), turns)),
        destroy_at_end_of_combat: permanent.destroy_at_end_of_combat,
        skipped_untap_steps: permanent.skipped_untap_steps,
        control_reverts_to: permanent.control_reverts_to.map(PlayerId::index),
        control_source: permanent.control_source.map(|id| id.0),
        control_requires_source_tapped: permanent.control_requires_source_tapped,
        chosen_player: permanent.chosen_player.map(PlayerId::index),
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
        attacks_this_turn: permanent.attacks_this_turn,
        damage_sources: permanent.damage_sources.iter().map(|id| id.0).collect(),
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
        copy_effect: copy_effect.map(|(snapshot, _)| snapshot),
        copied_from: permanent
            .copied_from
            .map(|(definition, part)| CopiedFromSnapshot {
                definition: definition.0,
                part_id: part.0,
            }),
        text_changes: permanent
            .text_changes
            .iter()
            .map(|change| model::BasicLandTypeChangeSnapshot {
                from: basic_land_type_snapshot(change.from),
                to: basic_land_type_snapshot(change.to),
            })
            .collect(),
        has_dynamic_characteristics: has_unlocated_resolved_effect || has_unlocated_copy_ability,
    }
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
        definition: permanent.card.definition.0,
        presented_part_id: permanent.presented.0,
        controller: permanent.controller.index(),
        tapped: permanent.tapped,
        damage: permanent.damage,
        attacking: permanent.attacking,
        attack_defender: permanent.attack_defender.map(|defender| match defender {
            AttackDefender::Player(player) => AttackDefenderSnapshot::Player {
                seat: player.index(),
            },
            AttackDefender::Planeswalker(object) => AttackDefenderSnapshot::Planeswalker {
                object_id: object.0,
            },
        }),
        blocked: permanent.blocked,
        blocking: permanent.blocking.map(|id| id.0),
        activated_loyalty_this_turn: permanent.activated_loyalty_this_turn,
        chosen_creature_type: permanent.chosen_creature_type.clone(),
        chosen_card_name: permanent.chosen_card_name.clone(),
    }
}
