use crate::CardCatalog;
use crate::card::DamageSourceMatcherDef;

use super::model_prevention::{
    DamagePreventionCapacitySnapshot, DamageRecipientMatcherSnapshot, DamageSourceGroupSnapshot,
    DamageSourceMatcherSnapshot, ResolvedDamagePreventionSnapshot, ResolvedDamageRedirectSnapshot,
};
use super::stack::object_reference_requires_hidden_rebinding;
use super::{
    AbilitySourceRef, ContinuousEffectTimestamp, Game, GameObjectId, PlayerId,
    RelationalSourceFilter, ResolvedDamagePrevention, ResolvedDamagePreventionCapacity,
    ResolvedDamageRecipientMatcher, ResolvedDamageRedirect, ResolvedDamageSourceMatcher, Target,
    ability_origin_from_snapshot, event, expiration_snapshot, parse_expiration,
    parse_snapshot_target, player_from_index, semantics, target_snapshot,
};
use crate::card::ValueDef;

pub(super) fn damage_redirect_snapshot(
    redirect: ResolvedDamageRedirect,
) -> ResolvedDamageRedirectSnapshot {
    ResolvedDamageRedirectSnapshot {
        player: redirect.player.index(),
        source: redirect.source.0,
        destination: redirect.destination.0,
        expiration: expiration_snapshot(redirect.expiration),
    }
}

pub(super) fn parse_damage_redirect(
    snapshot: &ResolvedDamageRedirectSnapshot,
) -> Result<ResolvedDamageRedirect, String> {
    Ok(ResolvedDamageRedirect {
        player: player_from_index(snapshot.player)?,
        source: GameObjectId(snapshot.source),
        destination: GameObjectId(snapshot.destination),
        expiration: parse_expiration(&snapshot.expiration)?,
    })
}

pub(super) const fn damage_redirect_referenced_object_ids(
    redirect: ResolvedDamageRedirect,
) -> [GameObjectId; 2] {
    [redirect.source, redirect.destination]
}

pub(super) fn damage_prevention_snapshot(
    game: &Game,
    viewer: PlayerId,
    prevention: ResolvedDamagePrevention,
) -> Option<ResolvedDamagePreventionSnapshot> {
    if damage_prevention_has_unrebindable_hidden_reference(game, viewer, prevention) {
        return None;
    }
    let catalog = &game.catalog;
    let matching_source = match prevention.source {
        ResolvedDamageSourceMatcher::Matching { predicate, .. } => Some(predicate),
        ResolvedDamageSourceMatcher::Any
        | ResolvedDamageSourceMatcher::Exact(_)
        | ResolvedDamageSourceMatcher::Except(_)
        | ResolvedDamageSourceMatcher::Group(_) => None,
    };
    let needs_definition =
        matching_source.is_some() || prevention.amount != ValueDef::DamageEventAmount;
    let definition = if needs_definition {
        Some(semantics::resolved_damage_prevention_locator(
            catalog,
            prevention.source_ability,
            matching_source,
            prevention.amount,
        )?)
    } else {
        None
    };
    let source = match prevention.source {
        ResolvedDamageSourceMatcher::Any => DamageSourceMatcherSnapshot::Any,
        ResolvedDamageSourceMatcher::Exact(object) => DamageSourceMatcherSnapshot::Exact {
            object_id: object.0,
        },
        ResolvedDamageSourceMatcher::Except(object) => DamageSourceMatcherSnapshot::Except {
            object_id: object.0,
        },
        ResolvedDamageSourceMatcher::Matching { relative_to, .. } => {
            DamageSourceMatcherSnapshot::Matching {
                relative_to: relative_to.0,
            }
        }
        ResolvedDamageSourceMatcher::Group(group) => DamageSourceMatcherSnapshot::Group {
            group: damage_source_group_snapshot(group),
        },
    };
    let recipient = match prevention.recipient {
        ResolvedDamageRecipientMatcher::Any => DamageRecipientMatcherSnapshot::Any,
        ResolvedDamageRecipientMatcher::Exact(target) => DamageRecipientMatcherSnapshot::Exact {
            target: target_snapshot(target),
        },
        ResolvedDamageRecipientMatcher::PlayerAndCreaturesControlledBy(player) => {
            DamageRecipientMatcherSnapshot::PlayerAndControlledCreatures {
                seat: player.index(),
            }
        }
    };
    Some(ResolvedDamagePreventionSnapshot {
        source,
        recipient,
        combat_only: prevention.combat_only,
        capacity: match prevention.capacity {
            ResolvedDamagePreventionCapacity::Amount(remaining) => {
                DamagePreventionCapacitySnapshot::Amount { remaining }
            }
            ResolvedDamagePreventionCapacity::Events(remaining) => {
                DamagePreventionCapacitySnapshot::Events { remaining }
            }
            ResolvedDamagePreventionCapacity::Unlimited => {
                DamagePreventionCapacitySnapshot::Unlimited
            }
        },
        definition,
        gain_life: prevention.gain_life.map(PlayerId::index),
        source_ability: event::ability_source_snapshot(prevention.source_ability),
        timestamp: prevention.timestamp.0,
        expiration: expiration_snapshot(prevention.expiration),
    })
}

pub(super) fn parse_damage_prevention(
    catalog: &CardCatalog,
    snapshot: &ResolvedDamagePreventionSnapshot,
) -> Result<ResolvedDamagePrevention, String> {
    let source_ability = AbilitySourceRef {
        object: GameObjectId(snapshot.source_ability.object),
        ability: ability_origin_from_snapshot(snapshot.source_ability.ability),
    };
    let authored = snapshot
        .definition
        .as_ref()
        .map(|definition| {
            semantics::catalog_damage_prevention(catalog, definition)
                .ok_or("damage-prevention locator is absent from this catalog")
        })
        .transpose()?;
    if let Some(definition) = &snapshot.definition
        && !semantics::ability_locator_matches_origin(&definition.ability, source_ability.ability)
    {
        return Err("damage-prevention locator disagrees with its source ability".into());
    }
    Ok(ResolvedDamagePrevention {
        source: match &snapshot.source {
            DamageSourceMatcherSnapshot::Any => ResolvedDamageSourceMatcher::Any,
            DamageSourceMatcherSnapshot::Exact { object_id } => {
                ResolvedDamageSourceMatcher::Exact(GameObjectId(*object_id))
            }
            DamageSourceMatcherSnapshot::Except { object_id } => {
                ResolvedDamageSourceMatcher::Except(GameObjectId(*object_id))
            }
            DamageSourceMatcherSnapshot::Matching { relative_to } => {
                let Some(authored) = authored else {
                    return Err("damage-prevention matcher has no authored definition".into());
                };
                let DamageSourceMatcherDef::Matching(predicate) = authored.matcher.source else {
                    return Err(
                        "damage-prevention matcher locator does not identify a predicate".into(),
                    );
                };
                ResolvedDamageSourceMatcher::Matching {
                    predicate,
                    relative_to: GameObjectId(*relative_to),
                }
            }
            DamageSourceMatcherSnapshot::Group { group } => {
                ResolvedDamageSourceMatcher::Group(parse_damage_source_group(*group))
            }
        },
        recipient: match snapshot.recipient {
            DamageRecipientMatcherSnapshot::Any => ResolvedDamageRecipientMatcher::Any,
            DamageRecipientMatcherSnapshot::Exact { target } => {
                ResolvedDamageRecipientMatcher::Exact(parse_snapshot_target(target))
            }
            DamageRecipientMatcherSnapshot::PlayerAndControlledCreatures { seat } => {
                ResolvedDamageRecipientMatcher::PlayerAndCreaturesControlledBy(player_from_index(
                    seat,
                )?)
            }
        },
        combat_only: snapshot.combat_only,
        capacity: match snapshot.capacity {
            DamagePreventionCapacitySnapshot::Amount { remaining } => {
                ResolvedDamagePreventionCapacity::Amount(remaining)
            }
            DamagePreventionCapacitySnapshot::Events { remaining } => {
                ResolvedDamagePreventionCapacity::Events(remaining)
            }
            DamagePreventionCapacitySnapshot::Unlimited => {
                ResolvedDamagePreventionCapacity::Unlimited
            }
        },
        amount: authored.map_or(ValueDef::DamageEventAmount, |prevention| prevention.amount),
        gain_life: snapshot.gain_life.map(player_from_index).transpose()?,
        source_ability,
        timestamp: ContinuousEffectTimestamp(snapshot.timestamp),
        expiration: parse_expiration(&snapshot.expiration)?,
    })
}

pub(super) fn damage_prevention_referenced_object_ids(
    prevention: ResolvedDamagePrevention,
) -> Vec<GameObjectId> {
    let mut objects = vec![prevention.source_ability.object];
    match prevention.source {
        ResolvedDamageSourceMatcher::Any | ResolvedDamageSourceMatcher::Group(_) => {}
        ResolvedDamageSourceMatcher::Exact(object)
        | ResolvedDamageSourceMatcher::Except(object)
        | ResolvedDamageSourceMatcher::Matching {
            relative_to: object,
            ..
        } => objects.push(object),
    }
    if let ResolvedDamageRecipientMatcher::Exact(target) = prevention.recipient
        && let Some(object) = target_object_id(target)
    {
        objects.push(object);
    }
    objects
}

pub(super) fn damage_prevention_has_unrebindable_hidden_reference(
    game: &Game,
    viewer: PlayerId,
    prevention: ResolvedDamagePrevention,
) -> bool {
    damage_prevention_referenced_object_ids(prevention)
        .into_iter()
        .any(|object| object_reference_requires_hidden_rebinding(game, viewer, object))
}

const fn damage_source_group_snapshot(group: RelationalSourceFilter) -> DamageSourceGroupSnapshot {
    match group {
        RelationalSourceFilter::CreaturesWithFlying => {
            DamageSourceGroupSnapshot::CreaturesWithFlying
        }
        RelationalSourceFilter::AttackingCreaturesWithoutFlying => {
            DamageSourceGroupSnapshot::AttackingCreaturesWithoutFlying
        }
        RelationalSourceFilter::Artifacts => DamageSourceGroupSnapshot::Artifacts,
        RelationalSourceFilter::UnblockedCreatures => DamageSourceGroupSnapshot::UnblockedCreatures,
    }
}

const fn parse_damage_source_group(group: DamageSourceGroupSnapshot) -> RelationalSourceFilter {
    match group {
        DamageSourceGroupSnapshot::CreaturesWithFlying => {
            RelationalSourceFilter::CreaturesWithFlying
        }
        DamageSourceGroupSnapshot::AttackingCreaturesWithoutFlying => {
            RelationalSourceFilter::AttackingCreaturesWithoutFlying
        }
        DamageSourceGroupSnapshot::Artifacts => RelationalSourceFilter::Artifacts,
        DamageSourceGroupSnapshot::UnblockedCreatures => RelationalSourceFilter::UnblockedCreatures,
    }
}

const fn target_object_id(target: Target) -> Option<GameObjectId> {
    match target {
        Target::Player(_) => None,
        Target::Card(object) | Target::Permanent(object) | Target::Spell(object) => Some(object),
    }
}
