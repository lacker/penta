use super::{AbilityLocator, AbilityOrigin};

pub(in crate::game::state_checkpoint) fn ability_locator_matches_origin(
    locator: &AbilityLocator,
    origin: AbilityOrigin,
) -> bool {
    match (locator, origin) {
        (
            AbilityLocator::DynamicGrant {
                granting: source, ..
            },
            origin,
        ) => ability_locator_matches_origin(source, origin),
        (
            AbilityLocator::Card {
                definition,
                part_id,
                ability_id,
                ..
            },
            AbilityOrigin::Printed {
                definition: expected_definition,
                part,
                ability,
            },
        ) => *definition == expected_definition && *part_id == part.0 && *ability_id == ability.0,
        (
            AbilityLocator::Card {
                definition,
                part_id,
                ability_id,
                ..
            },
            AbilityOrigin::Granted {
                source_definition,
                source_part,
                source_ability,
                ..
            },
        ) => {
            *definition == source_definition
                && *part_id == source_part.0
                && *ability_id == source_ability.0
        }
        (
            AbilityLocator::Token {
                part_id,
                ability_id,
                ..
            },
            AbilityOrigin::Token { part, ability },
        ) => *part_id == part.0 && *ability_id == ability.0,
        (
            AbilityLocator::Token {
                part_id,
                ability_id,
                ..
            },
            AbilityOrigin::TokenGranted {
                source_part,
                source_ability,
                ..
            },
        ) => *part_id == source_part.0 && *ability_id == source_ability.0,
        (AbilityLocator::Emblem { ability_id, .. }, AbilityOrigin::Emblem { ability }) => {
            *ability_id == ability.0
        }
        (
            AbilityLocator::Emblem { ability_id, .. },
            AbilityOrigin::EmblemGranted { source_ability, .. },
        ) => *ability_id == source_ability.0,
        _ => false,
    }
}

pub(super) fn with_nested(locator: AbilityLocator, nested: Vec<usize>) -> AbilityLocator {
    match locator {
        AbilityLocator::DynamicGrant {
            granting: source,
            definition,
        } => AbilityLocator::DynamicGrant {
            granting: source,
            definition: Box::new(with_nested(*definition, nested)),
        },
        AbilityLocator::Card {
            definition,
            part_id,
            ability_id,
            ..
        } => AbilityLocator::Card {
            definition,
            part_id,
            ability_id,
            nested,
        },
        AbilityLocator::Token {
            token,
            part_id,
            ability_id,
            ..
        } => AbilityLocator::Token {
            token,
            part_id,
            ability_id,
            nested,
        },
        AbilityLocator::Emblem {
            emblem, ability_id, ..
        } => AbilityLocator::Emblem {
            emblem,
            ability_id,
            nested,
        },
    }
}
