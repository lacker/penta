//! Complete payments for special actions, before their zone or face change.
use super::{
    DecisionContinuation, DecisionOption, DecisionPreference, DecisionVisibility, Game,
    GameObjectId, ManaCost, PlayerId, ResolvedEffectPayment,
};
use crate::card::{CostDef, SuspendAbilityDef, SuspendTimeDef};

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub(in crate::game) enum PaidSpecialAction {
    Plot,
    TurnFaceUp,
    Suspend { ability: usize, x: u16 },
}

impl Game {
    pub(in crate::game) fn special_action_payment(
        &self,
        player: PlayerId,
        source: GameObjectId,
        action: PaidSpecialAction,
    ) -> Option<ResolvedEffectPayment> {
        let (costs, x, printed_mana) = match action {
            PaidSpecialAction::TurnFaceUp => {
                let permanent = self.battlefield.iter().find(|p| {
                    p.card.id == source && p.controller == player && p.face_down.is_some()
                })?;
                (
                    self.face_up_cost(permanent)?,
                    0,
                    self.catalog
                        .get(permanent.card.definition.card_definition()?)?
                        .part(permanent.presented)?
                        .rules
                        .mana_cost(),
                )
            }
            PaidSpecialAction::Plot | PaidSpecialAction::Suspend { .. } => {
                let card = self.players[player.index()]
                    .hand
                    .iter()
                    .find(|card| card.id == source)?;
                let printed = self.catalog.get(card.definition)?.rules.mana_cost();
                match action {
                    PaidSpecialAction::Plot => {
                        (self.card_plot_cost(card.definition)?.to_vec(), 0, printed)
                    }
                    PaidSpecialAction::Suspend { ability, x } => {
                        let (_, SuspendAbilityDef::Hand { time, costs }) =
                            *self.suspend_abilities_in_hand(card).get(ability)?
                        else {
                            return None;
                        };
                        match time {
                            SuspendTimeDef::Fixed(_) if x != 0 => return None,
                            SuspendTimeDef::ChosenX { minimum } if x < minimum => return None,
                            _ => {}
                        }
                        (costs.to_vec(), x, printed)
                    }
                    PaidSpecialAction::TurnFaceUp => unreachable!(),
                }
            }
        };
        resolve_list(&costs, x, printed_mana)
    }

    pub(in crate::game) fn special_action_payment_options(
        &self,
        player: PlayerId,
        source: GameObjectId,
        payment: ResolvedEffectPayment,
    ) -> Vec<DecisionOption> {
        let can_pay = self.can_pay_effect_payment(player, payment.clone());
        self.payment_options(player, payment, can_pay, "Decline")
            .into_iter()
            .filter(|option| {
                option.id != 0
                    && option.card.is_none_or(|(id, _)| id != source)
                    && option.members.iter().all(|(id, _)| *id != source)
            })
            .collect()
    }

    pub(in crate::game) fn can_pay_special_action(
        &self,
        player: PlayerId,
        source: GameObjectId,
        action: PaidSpecialAction,
    ) -> bool {
        self.special_action_payment(player, source, action)
            .is_some_and(|payment| {
                !self
                    .special_action_payment_options(player, source, payment)
                    .is_empty()
            })
    }

    pub(in crate::game) fn begin_special_action_payment(
        &mut self,
        player: PlayerId,
        source: GameObjectId,
        action: PaidSpecialAction,
    ) {
        let Some(payment) = self.special_action_payment(player, source, action) else {
            return;
        };
        let options = self.special_action_payment_options(player, source, payment.clone());
        if options.len() == 1 {
            if self
                .settle_payment_decision(player, payment, &[options[0].id], &options)
                .is_some()
            {
                self.finish_paid_special_action(player, source, action);
            }
        } else if !options.is_empty() {
            self.queue_decision(
                player,
                "Pay the special action cost",
                DecisionVisibility::Private,
                DecisionPreference::Neutral,
                1..=1,
                false,
                options,
                DecisionContinuation::PaySpecialAction {
                    player,
                    source,
                    action,
                    payment,
                },
            );
        }
    }

    pub(in crate::game) fn finish_paid_special_action(
        &mut self,
        player: PlayerId,
        source: GameObjectId,
        action: PaidSpecialAction,
    ) {
        match action {
            PaidSpecialAction::Plot => self.finish_plot(player, source),
            PaidSpecialAction::TurnFaceUp => self.finish_turn_face_up(source),
            PaidSpecialAction::Suspend { ability, x } => {
                self.finish_suspend(player, source, ability, x);
            }
        }
    }
}

fn resolve(cost: CostDef, x: u16, printed: Option<ManaCost>) -> Option<ResolvedEffectPayment> {
    Some(match cost {
        CostDef::Mana(mut mana) => {
            if mana.variable_x {
                mana.generic = mana
                    .generic
                    .saturating_add(x.saturating_mul(mana.x_multiplier.max(1)));
                mana.variable_x = false;
                mana.x_multiplier = 0;
            }
            ResolvedEffectPayment::Mana(mana)
        }
        CostDef::ManaCostOf(crate::ObjectRefDef::Source) => {
            resolve(CostDef::Mana(printed?), x, printed)?
        }
        CostDef::PayLife(amount) => ResolvedEffectPayment::Life(amount),
        CostDef::Energy(amount) => ResolvedEffectPayment::Energy(amount),
        CostDef::DiscardCards(amount) => ResolvedEffectPayment::DiscardCards(amount),
        CostDef::DiscardMatching(object) | CostDef::DiscardCardMatching(object) => {
            ResolvedEffectPayment::DiscardMatching(object)
        }
        CostDef::SacrificePermanent {
            object,
            controller: crate::PlayerRelation::You,
        } => ResolvedEffectPayment::SacrificePermanentMatching(object),
        CostDef::MillCards(amount) => ResolvedEffectPayment::Mill(amount),
        CostDef::All(costs) => resolve_list(costs, x, printed)?,
        _ => return None,
    })
}
fn resolve_list(
    costs: &[crate::CostDef],
    x: u16,
    printed: Option<ManaCost>,
) -> Option<ResolvedEffectPayment> {
    let payments = costs
        .iter()
        .map(|cost| resolve(*cost, x, printed))
        .collect::<Option<Vec<_>>>()?;
    Some(ResolvedEffectPayment::all(payments))
}
