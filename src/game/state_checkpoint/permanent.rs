#![allow(clippy::wildcard_imports)]

use super::*;

#[allow(clippy::too_many_lines)]
pub(super) fn permanent_snapshot(
    catalog: &CardCatalog,
    permanent: &Permanent,
) -> PermanentSnapshot {
    let temporary_granted_abilities = permanent
        .temporary_granted_abilities
        .iter()
        .filter_map(|grant| {
            Some(TemporaryGrantedAbilitySnapshot {
                ability: ability_locator(catalog, |ability| *ability == grant.ability)?,
                source: grant.source.0,
                source_definition: grant.source_definition.0,
                source_part_id: grant.source_part.0,
                source_ability_id: grant.source_ability.0,
                grant_id: grant.grant.0,
                timestamp: grant.timestamp.0,
                order: grant.order,
                expiration: expiration_snapshot(grant.expiration),
            })
        })
        .collect::<Vec<_>>();
    let has_unlocated_grant =
        temporary_granted_abilities.len() != permanent.temporary_granted_abilities.len();
    let temporary_removed_abilities = permanent
        .temporary_removed_abilities
        .iter()
        .filter_map(|removal| {
            Some(TemporaryRemovedAbilitySnapshot {
                effect: applied_effect_locator(
                    catalog,
                    AppliedEffectDef::RemoveAbilities(removal.predicate),
                )?,
                timestamp: removal.timestamp.0,
                order: removal.order,
                expiration: expiration_snapshot(removal.expiration),
            })
        })
        .collect::<Vec<_>>();
    let has_unlocated_removal =
        temporary_removed_abilities.len() != permanent.temporary_removed_abilities.len();
    let licid_effects = permanent
        .licid_effects
        .iter()
        .filter_map(|effect| {
            Some(LicidEffectSnapshot {
                effect_id: effect.id.0,
                ender: effect.ender.index(),
                transform_action: ability_origin_snapshot(effect.transform_action),
                end: ability_locator(catalog, |candidate| *candidate == effect.end)?,
            })
        })
        .collect::<Vec<_>>();
    let has_unlocated_licid_effect = licid_effects.len() != permanent.licid_effects.len();
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
        power_bonus: permanent.power_bonus,
        toughness_bonus: permanent.toughness_bonus,
        unblockable_this_turn: permanent.unblockable_this_turn,
        cannot_block_this_turn: permanent.cannot_block_this_turn,
        detained_until_turn_of: permanent
            .detained_until_turn_of
            .map(|(player, turns)| (player.index(), turns)),
        combat_damage_prevented: permanent.combat_damage_prevented,
        combat_damage_dealt_by_prevented: permanent.combat_damage_dealt_by_prevented,
        cannot_regenerate_this_turn: permanent.cannot_regenerate_this_turn,
        control_layer_base: permanent.control_layer_base.map(PlayerId::index),
        control_until_end_of_turn: permanent
            .control_until_end_of_turn
            .iter()
            .map(|effect| UntilEndOfTurnControlSnapshot {
                timestamp: effect.timestamp.0,
                controller: effect.controller.index(),
            })
            .collect(),
        control_while_source_remains: permanent
            .control_while_source_remains
            .iter()
            .map(|effect| WhileSourceControlSnapshot {
                timestamp: effect.timestamp.0,
                controller: effect.controller.index(),
                source: effect.source.0,
                requires_source_tapped: effect.requires_source_tapped,
            })
            .collect(),
        chosen_player: permanent.chosen_player.map(PlayerId::index),
        destroy_at_end: permanent.destroy_at_end,
        counters: permanent.counters.to_vec(),
        attached_to: permanent.attached_to.map(|id| id.0),
        attachment_form: permanent.attachment_form.map(|form| match form {
            AttachmentForm::Bestowed { timestamp } => AttachmentFormSnapshot::Bestowed {
                timestamp: timestamp.0,
            },
            AttachmentForm::Reconfigured { timestamp } => AttachmentFormSnapshot::Reconfigured {
                timestamp: timestamp.0,
            },
            AttachmentForm::Licid => AttachmentFormSnapshot::Licid,
        }),
        licid_effects,
        reanimation_linked: permanent.reanimation_linked.map(|linked| linked.0),
        reanimation_effect: permanent.reanimation_effect.map(|effect| {
            ReanimationAttachmentEffectSnapshot {
                timestamp: effect.timestamp.0,
                aura: reanimation_aura_snapshot(effect.aura),
            }
        }),
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
        animation: permanent
            .animation
            .map(|animation| animation_snapshot(animation.definition)),
        animation_timestamp: permanent.animation.map(|animation| animation.timestamp.0),
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
        temporary_granted_abilities,
        temporary_removed_abilities,
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
        has_dynamic_characteristics: has_unlocated_grant
            || has_unlocated_removal
            || has_unlocated_copy_ability
            || has_unlocated_licid_effect,
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
