//! Shared application of copiable-value exceptions.

use super::super::{CopiableAbility, CopiableCharacteristics, StackObject};
use crate::card::{CopyAbilityDef, CopyExceptionsDef};

pub(super) fn apply_copy_exceptions(
    copy: &mut CopiableCharacteristics,
    exceptions: CopyExceptionsDef,
    object: &StackObject,
) {
    if let Some(name) = exceptions.name {
        copy.name = Some(name.to_owned());
    }
    if let Some(stats) = exceptions.base_power_toughness {
        copy.base_power_toughness = Some(stats);
    }
    if let Some(colors) = exceptions.colors {
        copy.colors = Some(colors);
    }
    copy.added_creature_types
        .extend(exceptions.added_creature_types.named);
    copy.added_types = copy.added_types.union(exceptions.added_types);
    for supertype in exceptions.added_supertypes {
        copy.added_supertypes[supertype.index()] = true;
        copy.removed_supertypes[supertype.index()] = false;
    }
    for supertype in exceptions.removed_supertypes {
        copy.removed_supertypes[supertype.index()] = true;
        copy.added_supertypes[supertype.index()] = false;
    }
    copy.no_mana_cost |= exceptions.no_mana_cost;
    if let Some(payload) = &object.ability {
        copy.added_abilities
            .extend(exceptions.added_abilities.iter().filter_map(|added| {
                Some(CopiableAbility {
                    origin: payload.origin,
                    definition: match added {
                        CopyAbilityDef::This => *payload.definition.as_deref()?,
                        CopyAbilityDef::Ability(ability) => **ability,
                    },
                })
            }));
    }
}
