impl Game {
    fn behold_cost_payments(
        &self,
        cost: CostDef,
        predicate: crate::card::ObjectPredicateDef,
        count: u8,
        choose_creature_type: bool,
        card: &CardInstance,
        player: PlayerId,
    ) -> Vec<SpellAdditionalCostPayment> {
        use crate::card::{ObjectPredicateDef, Subtype, SubtypeDef};
        let types = if choose_creature_type {
            crate::card::CREATURE_TYPES
                .iter()
                .map(|name| Subtype::from_name(name))
                .collect::<Vec<_>>()
        } else {
            vec![None]
        };
        types
            .into_iter()
            .flat_map(|chosen| {
                let predicates = [
                    predicate,
                    chosen.map_or(ObjectPredicateDef::Any, |kind| {
                        ObjectPredicateDef::Subtype(SubtypeDef::Fixed(kind))
                    }),
                ];
                // Each object is tested independently, without targeting or tapping it.
                let mut candidates = self
                    .battlefield
                    .iter()
                    .filter(|p| {
                        p.controller == player
                            && predicates.iter().all(|predicate| {
                                self.trigger_object_matches_for_controller(
                                    *predicate,
                                    &self.trigger_event_object(p),
                                    card.id,
                                    false,
                                    Some(player),
                                )
                            })
                    })
                    .map(|p| p.card.id)
                    .collect::<Vec<_>>();
                candidates.extend(
                    self.players[player.index()]
                        .hand
                        .iter()
                        .filter(|held| {
                            held.id != card.id
                                && predicates.iter().all(|predicate| {
                                    self.card_object_matches(
                                        *predicate,
                                        held,
                                        ZoneKind::Hand,
                                        card.id,
                                    )
                                })
                        })
                        .map(|held| held.id),
                );
                Self::object_combinations(&candidates, usize::from(count))
                    .into_iter()
                    .map(|objects| SpellAdditionalCostPayment {
                        objects: objects.into_iter().map(|id| (id, cost)).collect(),
                        chosen_creature_type: chosen,
                        ..SpellAdditionalCostPayment::free()
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}
