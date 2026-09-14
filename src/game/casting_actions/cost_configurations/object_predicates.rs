// Object costs whose matching condition reads the announced X.

impl Game {
    // An equality in a required conjunction bounds X by the chosen object's
    // mana value. Alternatives and negations do not establish that bound.
    fn cost_predicate_bounds_x(object: crate::card::ObjectPredicateDef) -> bool {
        use crate::card::{ObjectPredicateDef, ValueDef};
        match object {
            ObjectPredicateDef::ManaValueEqualTo(ValueDef::ChosenX) => true,
            ObjectPredicateDef::All(predicates) => predicates
                .iter()
                .copied()
                .any(Self::cost_predicate_bounds_x),
            _ => false,
        }
    }

    fn cost_card_matches_x(
        &self,
        predicate: crate::card::ObjectPredicateDef,
        card: &CardInstance,
        zone: ZoneKind,
        source: GameObjectId,
        x: u16,
    ) -> bool {
        use crate::card::{ObjectPredicateDef, ValueDef};
        let nested = |predicate| self.cost_card_matches_x(predicate, card, zone, source, x);
        match predicate {
            ObjectPredicateDef::All(predicates) => predicates.iter().copied().all(nested),
            ObjectPredicateDef::AnyOf(predicates) => predicates.iter().copied().any(nested),
            ObjectPredicateDef::Not(predicate) => !nested(*predicate),
            ObjectPredicateDef::ManaValueEqualTo(ValueDef::ChosenX) => self.card_object_matches(
                ObjectPredicateDef::ManaValueEqualTo(ValueDef::Constant(i32::from(x))),
                card,
                zone,
                source,
            ),
            _ => self.card_object_matches(predicate, card, zone, source),
        }
    }

    fn spell_object_additional_cost_payments_for_count(
        &self,
        cost: CostDef,
        required: usize,
        card: &CardInstance,
        player: PlayerId,
        x: u16,
    ) -> Vec<SpellAdditionalCostPayment> {
        let candidates = self.additional_cost_candidates_with_x(cost, card, player, Some(x));
        Self::object_combinations(&candidates, required)
            .into_iter()
            .map(|objects| SpellAdditionalCostPayment {
                objects: objects.into_iter().map(|object| (object, cost)).collect(),
                mana: ManaCost::default(),
                includes_mana_payment: false,
                life: 0,
                generic_reduction: None,
            })
            .collect()
    }

    fn additional_cost_candidates(
        &self,
        cost: CostDef,
        card: &CardInstance,
        player: PlayerId,
    ) -> Vec<GameObjectId> {
        self.additional_cost_candidates_with_x(cost, card, player, Some(0))
    }

    // None asks which objects can supply X equal to their own mana value;
    // Some checks payment against the spell's already announced X.
    fn additional_cost_candidates_with_x(
        &self,
        cost: CostDef,
        card: &CardInstance,
        player: PlayerId,
        x: Option<u16>,
    ) -> Vec<GameObjectId> {
        let (object, from) = match cost {
            CostDef::Sacrifice { object, .. } | CostDef::ReturnToHand { object, .. } => {
                (object, ZoneKind::Battlefield)
            }
            CostDef::Tap { object, .. } => (object, ZoneKind::Battlefield),
            CostDef::Discard { object, .. } => (object, ZoneKind::Hand),
            CostDef::Exile { object, from, .. } => (object, from),
            _ => return Vec::new(),
        };
        match from {
            ZoneKind::Battlefield => self
                .battlefield
                .iter()
                .filter(|permanent| {
                    permanent.controller == player
                        && (!matches!(cost, CostDef::Tap { .. }) || !permanent.tapped)
                        && self.trigger_object_matches(
                            object,
                            &self.trigger_event_object(permanent),
                            permanent.card.id,
                            false,
                        )
                })
                .map(|permanent| permanent.card.id)
                .collect(),
            // The same exclusion as hand below, for the same reason: escape
            // and flashback are cast from the graveyard, so by the time the
            // cost is paid the card is on the stack and not there to spend.
            // This is what "exile five other cards" means.
            ZoneKind::Graveyard => self.players[player.index()]
                .graveyard
                .iter()
                .filter(|held| {
                    held.id != card.id
                        && self.cost_card_matches_x(
                            object,
                            held,
                            ZoneKind::Graveyard,
                            held.id,
                            x.unwrap_or_else(|| {
                                self.current_or_last_known_mana_value(held.id).unwrap_or(0)
                            }),
                        )
                })
                .map(|held| held.id)
                .collect(),
            // The card paying the cost cannot be the spell itself: it has
            // already left hand by the time the cost is paid.
            ZoneKind::Hand => self.players[player.index()]
                .hand
                .iter()
                .filter(|held| {
                    held.id != card.id
                        && self.cost_card_matches_x(
                            object,
                            held,
                            ZoneKind::Hand,
                            held.id,
                            x.unwrap_or_else(|| {
                                self.current_or_last_known_mana_value(held.id).unwrap_or(0)
                            }),
                        )
                })
                .map(|held| held.id)
                .collect(),
            _ => Vec::new(),
        }
    }
}
