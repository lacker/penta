impl Game {
    /// Set replaces the entire land family (CR 305.7), preserving other families.
    fn apply_basic_land_subtype_operation(subtypes: &mut SubtypeSet, operation: LandTypeOperation) {
        let mut selected = SubtypeSet::EMPTY;
        match operation {
            LandTypeOperation::SetTo(mask)
            | LandTypeOperation::Add(mask)
            | LandTypeOperation::Remove(mask) => {
                for land_type in BasicLandType::ALL {
                    if mask[land_type.index()] {
                        selected.insert(land_type.subtype_id());
                    }
                }
            }
            LandTypeOperation::SetToChosen(chosen) | LandTypeOperation::AddChosen(chosen) => {
                selected.insert(chosen.subtype_id());
            }
            LandTypeOperation::Substitute { from, to } => {
                if !subtypes.contains(from.subtype_id()) {
                    return;
                }
                selected.insert(to.subtype_id());
            }
        }
        *subtypes = match operation {
            LandTypeOperation::SetTo(_)
            | LandTypeOperation::SetToChosen(_)
            | LandTypeOperation::Substitute { .. } => subtypes
                .difference(const { SubtypeSet::family(SubtypeFamily::Land) })
                .union(selected),
            LandTypeOperation::Add(_) | LandTypeOperation::AddChosen(_) => subtypes.union(selected),
            LandTypeOperation::Remove(_) => subtypes.difference(selected),
        };
    }
}
