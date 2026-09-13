//! Resources owed by an enclosing cost. Mana production may change other
//! resources, but may not spend the same untapped state or destroy a later payer.
use super::super::{AbilityOrigin, CostDef, CounterKind, Game, GameObjectId, PlayerId};

#[derive(Clone, Debug)]
pub(in crate::game) enum PaymentReservation {
    Object(GameObjectId),
    Untapped(GameObjectId),
    Tapped(GameObjectId),
    Counters(GameObjectId, CounterKind, u16),
    Life(u16),
    Hand(usize),
    Library(usize),
    Quota {
        source: GameObjectId,
        cost: CostDef,
        excluded: Vec<GameObjectId>,
    },
}

impl Game {
    pub(in crate::game) fn activation_payment_reservations(
        source: GameObjectId,
        origin: AbilityOrigin,
        costs: &[CostDef],
        chosen: &[GameObjectId],
    ) -> Vec<PaymentReservation> {
        let mut excluded = costs
            .iter()
            .filter_map(|cost| match cost {
                CostDef::SacrificeObject(reference) => {
                    Self::activation_object_reference(*reference, source, origin)
                }
                CostDef::SacrificeSource | CostDef::ExileSource | CostDef::ReturnSourceToHand => {
                    Some(source)
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        excluded.extend_from_slice(chosen);
        let mut reserved = chosen
            .iter()
            .copied()
            .map(PaymentReservation::Object)
            .collect::<Vec<_>>();
        for cost in costs {
            match *cost {
                CostDef::TapSource => reserved.push(PaymentReservation::Untapped(source)),
                CostDef::UntapSource => reserved.push(PaymentReservation::Tapped(source)),
                CostDef::TapPermanents { count: 1, .. } => reserved.extend(chosen.iter().copied().map(PaymentReservation::Untapped)),
                CostDef::SacrificeSource | CostDef::ExileSource | CostDef::ReturnSourceToHand
                | CostDef::DiscardSource | CostDef::ExertSource => reserved.push(PaymentReservation::Object(source)),
                CostDef::SacrificeObject(reference) => {
                    if let Some(object) = Self::activation_object_reference(reference, source, origin) {
                        reserved.push(PaymentReservation::Object(object));
                    }
                }
                CostDef::RemoveCountersFromSource { kind, amount } => {
                    if let Some(PaymentReservation::Counters(_, _, required)) = reserved.iter_mut().find(|r|
                        matches!(r, PaymentReservation::Counters(object, counter, _) if *object == source && *counter == kind))
                    { *required = required.saturating_add(amount); }
                    else { reserved.push(PaymentReservation::Counters(source, kind, amount)); }
                }
                CostDef::Loyalty(change) => {
                    reserved.push(PaymentReservation::Object(source));
                    if change < 0 { reserved.push(PaymentReservation::Counters(source, CounterKind::Loyalty, u16::from(change.unsigned_abs()))); }
                }
                CostDef::DiscardCardsAtRandom(amount) => reserved.push(PaymentReservation::Hand(usize::from(amount))),
                CostDef::MillCards(amount) | CostDef::ExileTopCards(amount) => reserved.push(PaymentReservation::Library(usize::from(amount))),
                CostDef::SacrificePermanents { .. } | CostDef::TapPermanents { .. }
                | CostDef::TapCreaturesWithTotalPower { .. } => {
                    let mut excluded = excluded.clone();
                    if costs.contains(&CostDef::TapSource) && !matches!(cost, CostDef::SacrificePermanents { .. }) { excluded.push(source); }
                    reserved.push(PaymentReservation::Quota { source, cost: *cost, excluded });
                }
                _ => {}
            }
        }
        let life = crate::card::costs::life_cost(costs);
        if life > 0 {
            reserved.push(PaymentReservation::Life(life));
        }
        reserved
    }

    pub(in crate::game) fn payment_reservations_hold(
        &self,
        player: PlayerId,
        reserved: &[PaymentReservation],
    ) -> bool {
        reserved.iter().all(|reservation| match reservation {
            PaymentReservation::Hand(amount) => self.players[player.index()].hand.len() >= *amount,
            PaymentReservation::Library(amount) => {
                self.players[player.index()].library.len() >= *amount
            }
            PaymentReservation::Quota {
                source,
                cost,
                excluded,
            } => match cost {
                CostDef::TapPermanents {
                    object,
                    controller,
                    count,
                } => {
                    self.activation_tap_candidates(
                        player,
                        *object,
                        *controller,
                        *source,
                        excluded,
                        false,
                    )
                    .len()
                        >= usize::from(*count)
                }
                CostDef::SacrificePermanents {
                    object,
                    controller,
                    count,
                } => {
                    self.activation_sacrifice_candidates(
                        player,
                        *object,
                        *controller,
                        *source,
                        excluded,
                    )
                    .len()
                        >= usize::from(*count)
                }
                CostDef::TapCreaturesWithTotalPower { minimum } => {
                    let mut view = self.clone();
                    for p in &mut view.battlefield {
                        if excluded.contains(&p.card.id) {
                            p.tapped = true;
                        }
                    }
                    view.can_pay_total_power_tap(player, *source, *minimum)
                }
                _ => unreachable!("only object quotas are reserved"),
            },
            PaymentReservation::Life(amount) => self.can_pay_life(player, *amount),
            PaymentReservation::Object(object) => {
                self.battlefield.iter().any(|p| p.card.id == *object)
                    || self.card_in_nonbattlefield_zone(*object).is_some()
            }
            PaymentReservation::Untapped(object) => self
                .battlefield
                .iter()
                .any(|p| p.card.id == *object && !p.tapped),
            PaymentReservation::Tapped(object) => self
                .battlefield
                .iter()
                .any(|p| p.card.id == *object && p.tapped),
            PaymentReservation::Counters(object, kind, amount) => self
                .battlefield
                .iter()
                .any(|p| p.card.id == *object && p.counters(*kind) >= *amount),
        })
    }
}
