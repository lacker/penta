impl Game {
    /// Whether an ordered layer-4 stream actually executes a Set operation.
    /// A chosen substitution is conditional: a basic Plains is untouched by
    /// an Island-to-Forest choice and therefore keeps its rules text. Earlier
    /// Add/Remove operations still feed that condition in timestamp order.
    fn land_type_set_operation_applies(
        &self,
        affected: &Permanent,
        operations: &[(ContinuousEffectTimestamp, u16, LandTypeOperation)],
    ) -> bool {
        let mut current = self.basic_land_types_before_layer_operations(affected);
        for (_, _, operation) in operations {
            match operation {
                LandTypeOperation::SetTo(_) | LandTypeOperation::SetToChosen(_) => return true,
                LandTypeOperation::Substitute { from, .. } => {
                    if current[from.index()] {
                        return true;
                    }
                }
                LandTypeOperation::Add(types) => {
                    for (present, added) in current.iter_mut().zip(types) {
                        *present |= added;
                    }
                }
                LandTypeOperation::AddChosen(chosen) => current[chosen.index()] = true,
                LandTypeOperation::Remove(types) => {
                    for (present, removed) in current.iter_mut().zip(types) {
                        *present &= !removed;
                    }
                }
            }
        }
        false
    }

    fn basic_land_types_before_layer_operations(
        &self,
        permanent: &Permanent,
    ) -> [bool; BasicLandType::ALL.len()] {
        let mut present = [false; BasicLandType::ALL.len()];
        let Some(rules) = self.effective_rules(permanent) else {
            return present;
        };
        let text_words = self.text_word_map_for_permanent(permanent);
        for subtype in rules
            .subtypes()
            .iter()
            .copied()
            .chain(self.retained_printed_subtypes(permanent).iter().copied())
        {
            let Some(land_type) = BasicLandType::from_subtype(subtype) else {
                continue;
            };
            let land_type = text_words.basic_land_type(land_type);
            present[land_type.index()] = true;
        }
        present
    }
}
