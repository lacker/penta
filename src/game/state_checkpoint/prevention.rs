use crate::CardCatalog;
use crate::card::DamageSourceMatcherDef;

use super::model_prevention::{
    DamagePreventionCapacitySnapshot, DamagePreventionCoverageSnapshot, DamagePreventionLocator,
    DamageRecipientMatcherSnapshot, DamageSourceGroupSnapshot, DamageSourceMatcherSnapshot,
    ResolvedDamagePreventionSnapshot, ResolvedDamageRedirectSnapshot,
};
use super::stack::object_reference_requires_hidden_rebinding;
use super::{
    AbilitySourceRef, ContinuousEffectTimestamp, Game, GameObjectId, PlayerId,
    RelationalSourceFilter, ResolvedDamagePrevention, ResolvedDamagePreventionCapacity,
    ResolvedDamagePreventionCoverage, ResolvedDamageRecipientMatcher, ResolvedDamageRedirect,
    ResolvedDamageSourceMatcher, Target, ability_origin_from_snapshot, event, expiration_snapshot,
    parse_expiration, parse_snapshot_target, player_from_index, semantics, target_snapshot,
};

pub(super) const fn damage_redirect_snapshot(
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
    snapshot: ResolvedDamageRedirectSnapshot,
) -> Result<ResolvedDamageRedirect, String> {
    Ok(ResolvedDamageRedirect {
        player: player_from_index(snapshot.player)?,
        source: GameObjectId(snapshot.source),
        destination: GameObjectId(snapshot.destination),
        expiration: parse_expiration(snapshot.expiration)?,
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
    let source = match prevention.source {
        ResolvedDamageSourceMatcher::Any => DamageSourceMatcherSnapshot::Any,
        ResolvedDamageSourceMatcher::Exact(object) => DamageSourceMatcherSnapshot::Exact {
            object_id: object.0,
        },
        ResolvedDamageSourceMatcher::Except(object) => DamageSourceMatcherSnapshot::Except {
            object_id: object.0,
        },
        ResolvedDamageSourceMatcher::Matching {
            predicate,
            relative_to,
        } => DamageSourceMatcherSnapshot::Matching {
            definition: semantics::resolved_damage_prevention_locator(
                catalog,
                prevention.source_ability,
                predicate,
            )?,
            relative_to: relative_to.0,
        },
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
        coverage: match prevention.coverage {
            ResolvedDamagePreventionCoverage::All => DamagePreventionCoverageSnapshot::All,
            ResolvedDamagePreventionCoverage::HalfRoundedDown => {
                DamagePreventionCoverageSnapshot::HalfRoundedDown
            }
        },
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
    Ok(ResolvedDamagePrevention {
        source: match &snapshot.source {
            DamageSourceMatcherSnapshot::Any => ResolvedDamageSourceMatcher::Any,
            DamageSourceMatcherSnapshot::Exact { object_id } => {
                ResolvedDamageSourceMatcher::Exact(GameObjectId(*object_id))
            }
            DamageSourceMatcherSnapshot::Except { object_id } => {
                ResolvedDamageSourceMatcher::Except(GameObjectId(*object_id))
            }
            DamageSourceMatcherSnapshot::Matching {
                definition,
                relative_to,
            } => {
                let authored = semantics::catalog_damage_prevention(catalog, definition)
                    .ok_or("damage-prevention matcher locator is absent from this catalog")?;
                let DamageSourceMatcherDef::Matching(predicate) = authored.matcher.source else {
                    return Err(
                        "damage-prevention matcher locator does not identify a predicate".into(),
                    );
                };
                let expected = semantics::resolved_damage_prevention_locator(
                    catalog,
                    source_ability,
                    predicate,
                )
                .ok_or(
                    "damage-prevention source ability does not contain its matcher definition",
                )?;
                if !same_locator(&expected, definition) {
                    return Err(
                        "damage-prevention matcher locator disagrees with its source ability"
                            .into(),
                    );
                }
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
        coverage: match snapshot.coverage {
            DamagePreventionCoverageSnapshot::All => ResolvedDamagePreventionCoverage::All,
            DamagePreventionCoverageSnapshot::HalfRoundedDown => {
                ResolvedDamagePreventionCoverage::HalfRoundedDown
            }
        },
        gain_life: snapshot.gain_life.map(player_from_index).transpose()?,
        source_ability,
        timestamp: ContinuousEffectTimestamp(snapshot.timestamp),
        expiration: parse_expiration(snapshot.expiration)?,
    })
}

fn same_locator(left: &DamagePreventionLocator, right: &DamagePreventionLocator) -> bool {
    left.effect_index == right.effect_index && left.ability == right.ability
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
