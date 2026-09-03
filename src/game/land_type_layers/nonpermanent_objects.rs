#[derive(Clone)]
enum SubtypeLayerOperation {
    BasicLand(LandTypeOperation),
    Creature(SetOperationDef<CreatureTypeSetDef>),
    ChosenCreature {
        chosen: &'static str,
        replace: bool,
    },
    Named(SetOperationDef<&'static [&'static str]>),
    /// The same as adding named subtypes, over a list a copy carries rather
    /// than one a card printed. Owned because the copy's exceptions are
    /// interned per game rather than authored as a static slice.
    AddedNamed(Vec<&'static str>),
}

impl Game {
    fn static_source_chosen_creature_type(
        &self,
        source: crate::GameObjectId,
        prospective: Option<&Permanent>,
    ) -> Option<&'static str> {
        self.battlefield
            .iter()
            .find(|permanent| permanent.card.id == source)
            .or_else(|| prospective.filter(|permanent| permanent.card.id == source))
            .and_then(|permanent| permanent.chosen_creature_type.as_deref())
            .and_then(crate::card::creature_type_name)
    }

    fn collect_static_subtype_operation(
        &self,
        applied: &super::StaticAppliedEffect,
        prospective: Option<&Permanent>,
        operations: &mut Vec<(ContinuousEffectTimestamp, u16, SubtypeLayerOperation)>,
    ) {
        let operation = match applied.effect {
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::CreatureTypes(
                operation,
            )) => Some(SubtypeLayerOperation::Creature(operation)),
            AppliedEffectDef::Characteristic(CharacteristicOperationDef::Subtypes(operation)) => {
                Some(SubtypeLayerOperation::Named(operation))
            }
            AppliedEffectDef::Characteristic(
                operation @ (CharacteristicOperationDef::AddChosenCreatureType
                | CharacteristicOperationDef::SetChosenCreatureType),
            ) => self
                .static_source_chosen_creature_type(applied.source, prospective)
                .map(|chosen| SubtypeLayerOperation::ChosenCreature {
                    chosen,
                    replace: operation == CharacteristicOperationDef::SetChosenCreatureType,
                }),
            _ => None,
        };
        if let Some(operation) = operation {
            operations.push((applied.timestamp, applied.component_order, operation));
        }
    }

    /// Applies subtype-changing static effects from battlefield sources to a
    /// spell or card characteristic snapshot. Permanents use the ordinary
    /// layer walk; objects in other zones have no permanent state to carry
    /// that walk, but the authored `StaticApply` query is the same.
    pub(super) fn apply_static_subtype_effects_to_object(
        &self,
        object: &mut TriggerEventObject,
        context: &CharacteristicContext,
    ) {
        let Some(zone) = context.self_characteristic_zone() else {
            return;
        };
        let owner = self
            .stack
            .iter()
            .find(|candidate| candidate.id == object.id)
            .map(|candidate| candidate.card.owner)
            .or_else(|| {
                self.card_in_nonbattlefield_zone(object.id)
                    .map(|(_, card)| card.owner)
            })
            .unwrap_or(object.controller);
        let baseline = object.clone();
        let affected = StaticAffectedObject::Object {
            characteristics: &baseline,
            controller: (zone == ZoneKind::Stack).then_some(object.controller),
            owner,
            zone,
            is_spell: zone == ZoneKind::Stack,
        };
        let mut operations = Vec::new();
        let result = self.visit_battlefield_static_applied_effects_for_object(
            affected,
            StaticEffectKind::Subtypes,
            |applied| {
                self.collect_static_subtype_operation(&applied, None, &mut operations);
                ControlFlow::Continue(())
            },
        );
        debug_assert!(result.is_continue());
        operations.sort_by_key(|(timestamp, order, _)| (*timestamp, *order));
        Self::apply_subtype_operations(object.subtypes.to_mut(), operations);
    }
}
