// Static abilities whose sources are cards in graveyards.

#[derive(Clone, Copy)]
struct StaticEffectSource<'a> {
    permanent: &'a Permanent,
    zone: ZoneKind,
    timestamp: crate::game::ContinuousEffectTimestamp,
    check_layer_survival: bool,
}

impl<'a> StaticEffectSource<'a> {
    const fn battlefield(
        permanent: &'a Permanent,
        timestamp: crate::game::ContinuousEffectTimestamp,
    ) -> Self {
        Self {
            permanent,
            zone: ZoneKind::Battlefield,
            timestamp,
            check_layer_survival: true,
        }
    }

    const fn graveyard(permanent: &'a Permanent) -> Self {
        Self {
            permanent,
            zone: ZoneKind::Graveyard,
            timestamp: permanent.timestamp,
            check_layer_survival: false,
        }
    }
}

impl Game {
    fn graveyard_static_sources(&self) -> Vec<Permanent> {
        [PlayerId::One, PlayerId::Two]
            .into_iter()
            .flat_map(|owner| {
                self.players[owner.index()]
                    .graveyard
                    .iter()
                    .filter(|card| {
                        let mut supplies_graveyard_static = false;
                        self.for_each_printed_card_ability(
                            card,
                            &CharacteristicContext::Graveyard,
                            |effective| {
                                let ability = effective.ability;
                                supplies_graveyard_static |= ability.is_executable()
                                    && matches!(
                                        ability.definition,
                                        DeclarativeAbilityDef::Static(definition)
                                            if definition.source_zones.contains(&ZoneKind::Graveyard)
                                    )
                                    && ability.declarative_effect().is_some();
                            },
                        );
                        supplies_graveyard_static
                    })
                    .map(move |card| {
                        Permanent::entering(
                            card.clone(),
                            CardPartId::PRIMARY,
                            owner,
                            self.turns_started[owner.index()],
                            self.turn,
                        )
                    })
            })
            .collect()
    }

    fn visit_static_source_effects(
        &self,
        input: StaticEffectSource<'_>,
        affected: &Permanent,
        prospective: Option<&Permanent>,
        kind: StaticEffectKind,
        visitor: &mut impl FnMut(StaticAppliedEffect) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        let source = input.permanent;
        let Some(rules) = self.effective_rules(source) else {
            return ControlFlow::Continue(());
        };
        let source_presentation = Self::effective_rules_source(source);
        for attached in rules.indexed_abilities() {
            let ability = attached.definition;
            let DeclarativeAbilityDef::Static(definition) = ability.definition else {
                continue;
            };
            if !ability.is_executable() || !definition.source_zones.contains(&input.zone) {
                continue;
            }
            let origin = Self::authored_ability_origin(source_presentation, attached.id);
            if input.check_layer_survival
                && !self.ability_survives_resolved_operations(source, origin)
            {
                continue;
            }
            let Some(effect) = ability.declarative_effect() else {
                continue;
            };
            let mut traversal = StaticEffectTraversal {
                source,
                source_timestamp: input.timestamp,
                source_presentation,
                source_origin: origin,
                affected,
                prospective,
                next_grant: 0,
                next_component_order: 0,
            };
            if self
                .visit_static_effect(effect, &mut traversal, kind, visitor)
                .is_break()
            {
                return ControlFlow::Break(());
            }
        }
        ControlFlow::Continue(())
    }
}
