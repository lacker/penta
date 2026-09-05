impl Game {
    /// Applies one layer-4 basic-land-subtype operation. Set removes every
    /// existing land subtype before installing its result (CR 305.7).
    fn apply_basic_land_subtype_operation(
        subtypes: &mut Vec<&'static str>,
        operation: LandTypeOperation,
    ) {
        fn is_land_subtype(subtype: &str) -> bool {
            LAND_SUBTYPES.contains(&subtype)
        }

        match operation {
            LandTypeOperation::SetTo(_)
            | LandTypeOperation::SetToChosen(_)
            | LandTypeOperation::Substitute { .. } => {
                if let LandTypeOperation::Substitute { from, .. } = operation
                    && !subtypes
                        .iter()
                        .any(|subtype| BasicLandType::from_subtype(subtype) == Some(from))
                {
                    return;
                }
                let chosen = [match operation {
                    LandTypeOperation::SetToChosen(chosen) => chosen,
                    LandTypeOperation::Substitute { to, .. } => to,
                    _ => BasicLandType::Plains,
                }];
                let types = match operation {
                    LandTypeOperation::SetTo(mask) => BasicLandType::ALL
                        .into_iter()
                        .filter(|land_type| mask[land_type.index()])
                        .collect::<Vec<_>>(),
                    LandTypeOperation::SetToChosen(_) | LandTypeOperation::Substitute { .. } => {
                        chosen.into_iter().collect()
                    }
                    LandTypeOperation::Add(_) | LandTypeOperation::Remove(_) => unreachable!(),
                };
                let mut insertion = subtypes
                    .iter()
                    .position(|subtype| is_land_subtype(subtype))
                    .unwrap_or(0);
                subtypes.retain(|subtype| !is_land_subtype(subtype));
                insertion = insertion.min(subtypes.len());
                for land_type in types {
                    if subtypes
                        .iter()
                        .any(|subtype| BasicLandType::from_subtype(subtype) == Some(land_type))
                    {
                        continue;
                    }
                    subtypes.insert(insertion, land_type.subtype());
                    insertion += 1;
                }
            }
            LandTypeOperation::Add(types) => {
                let mut insertion = subtypes
                    .iter()
                    .position(|subtype| !is_land_subtype(subtype))
                    .unwrap_or(subtypes.len());
                for land_type in BasicLandType::ALL
                    .into_iter()
                    .filter(|land_type| types[land_type.index()])
                {
                    if subtypes
                        .iter()
                        .any(|subtype| BasicLandType::from_subtype(subtype) == Some(land_type))
                    {
                        continue;
                    }
                    subtypes.insert(insertion, land_type.subtype());
                    insertion += 1;
                }
            }
            LandTypeOperation::Remove(types) => {
                subtypes.retain(|subtype| {
                    BasicLandType::from_subtype(subtype)
                        .is_none_or(|land_type| !types[land_type.index()])
                });
            }
        }
    }
}
