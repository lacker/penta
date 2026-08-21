// Rebuilding copiable values, including both intrinsic faces of a copied
// double-faced token. Included textually by `wire.rs`.

pub(super) fn parse_copiable_characteristics(
    snapshot: &CopiableCharacteristicsSnapshot,
    catalog: &CardCatalog,
) -> Result<CopiableCharacteristics, String> {
    let base = object_characteristics_from_snapshot(catalog, &snapshot.base)
        .ok_or("checkpoint copy-effect characteristics are absent from this catalog")?;
    if matches!(base, ObjectCharacteristics::Emblem { .. }) {
        return Err("an emblem cannot supply copied characteristics".into());
    }
    let mut added_types = CardTypeSet::empty();
    for (card_type, present) in CardType::ALL.into_iter().zip(snapshot.added_types) {
        if present {
            added_types = added_types.with(card_type);
        }
    }
    Ok(CopiableCharacteristics {
        base,
        added_types,
        retain_printed_subtypes: snapshot.retain_printed_subtypes,
        added_abilities: snapshot
            .added_abilities
            .iter()
            .map(|ability| {
                let origin = ability_origin_from_snapshot(ability.origin);
                if !super::semantics::ability_locator_matches_origin(&ability.ability, origin) {
                    return Err(
                        "checkpoint copied ability locator disagrees with its origin".to_owned(),
                    );
                }
                Ok(CopiableAbility {
                    origin,
                    definition: catalog_ability(catalog, &ability.ability).ok_or_else(|| {
                        "checkpoint copied ability locator is absent from this catalog".to_owned()
                    })?,
                })
            })
            .collect::<Result<Vec<_>, String>>()?,
    })
}

fn parse_double_faced_copiable_characteristics(
    snapshot: &DoubleFacedCopiableCharacteristicsSnapshot,
    catalog: &CardCatalog,
) -> Result<DoubleFacedCopiableCharacteristics, String> {
    if snapshot.front_part_id == snapshot.back_part_id {
        return Err("checkpoint double-faced token uses the same ID for both faces".into());
    }
    Ok(DoubleFacedCopiableCharacteristics {
        kind: if snapshot.modal {
            DoubleFacedKind::Modal
        } else {
            DoubleFacedKind::Transforming
        },
        front_part: CardPartId(snapshot.front_part_id),
        back_part: CardPartId(snapshot.back_part_id),
        front: parse_copiable_characteristics(&snapshot.front, catalog)?,
        back: parse_copiable_characteristics(&snapshot.back, catalog)?,
    })
}
