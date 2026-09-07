//! Frozen scalar actions used by repeated and combined resolving payments.

use crate::card::{CostDef, ManaSelectionDef, ManaTypeDef, PlayerRelation};
use crate::game::{Game, GameObjectId, ResolvedEffectPayment as Payment};

impl Game {
    pub(in crate::game) fn resolve_repeated_scalar_cost(
        cost: CostDef,
        source: GameObjectId,
        times: u16,
    ) -> Option<Payment> {
        let repeated = |amount: u16| amount.checked_mul(times);
        Some(match cost {
            CostDef::Mana(cost) => Payment::Mana(repeat_mana_cost(cost, times)),
            CostDef::SnowMana(amount) => Payment::SnowMana {
                source,
                amount: repeated(amount)?,
            },
            CostDef::PayLife(amount) => Payment::Life(repeated(amount)?),
            CostDef::Energy(amount) => Payment::Energy(repeated(amount)?),
            CostDef::DrawCards(amount) => Payment::DrawCards(repeated(amount)?),
            CostDef::PutCountersOnSource { kind, amount } => Payment::PutCounters {
                object: source,
                kind,
                amount,
                times,
            },
            CostDef::ExileTopCards(amount) => Payment::ExileTopCards(repeated(amount)?),
            CostDef::AddMana(effect) => {
                let ManaSelectionDef::One(ManaTypeDef::Fixed(color)) = effect.mana else {
                    return None;
                };
                if effect.also.is_some()
                    || effect.variable_amount.is_some()
                    || effect.amount_override.is_some()
                    || effect.damage_to_controller != 0
                    || effect.sacrifice_source_when_out_of.is_some()
                    || !effect.restrictions.is_empty()
                    || !effect.spend_effects.is_empty()
                {
                    return None;
                }
                Payment::AddMana {
                    color,
                    amount: repeated(effect.amount)?,
                }
            }
            CostDef::GainLife {
                player: PlayerRelation::Opponent,
                amount,
            } => Payment::OpponentGainsLife(repeated(amount)?),
            CostDef::CreateTokens {
                player: PlayerRelation::Opponent,
                token,
                amount,
            } => Payment::OpponentCreatesTokens {
                token: *token,
                amount: repeated(amount)?,
            },
            CostDef::FlipCoins(amount) => Payment::FlipCoins(repeated(amount)?),
            _ => return None,
        })
    }
}

pub(in crate::game) fn repeat_mana_cost(mut cost: crate::ManaCost, times: u16) -> crate::ManaCost {
    cost.generic = cost.generic.saturating_mul(times);
    cost.white = cost.white.saturating_mul(times);
    cost.blue = cost.blue.saturating_mul(times);
    cost.black = cost.black.saturating_mul(times);
    cost.red = cost.red.saturating_mul(times);
    cost.green = cost.green.saturating_mul(times);
    cost.colorless = cost.colorless.saturating_mul(times);
    for amount in &mut cost.hybrid {
        *amount = amount.saturating_mul(times);
    }
    for amount in &mut cost.additional_flexible {
        *amount = amount.saturating_mul(times);
    }
    cost.x_multiplier = cost.x_multiplier.saturating_mul(times);
    cost
}
