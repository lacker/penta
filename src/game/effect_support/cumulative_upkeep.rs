impl Game {
    pub(in crate::game) fn resolved_cumulative_upkeep_payment(
        cost: crate::card::CostDef,
        source: GameObjectId,
        age: u16,
    ) -> crate::game::ResolvedEffectPayment {
        use crate::card::CostDef as Cost;
        use crate::game::ResolvedEffectPayment as Resolved;

        let repeated = |amount: u16| amount.saturating_mul(age);
        match cost {
            Cost::Mana(cost) => Resolved::CumulativeMana {
                source,
                cost: repeat_mana_cost(cost, age),
            },
            Cost::SnowMana(amount) => Resolved::SnowMana {
                source,
                amount: repeated(amount),
            },
            Cost::PayLife(amount) => Resolved::Life(repeated(amount)),
            Cost::DrawCards(amount) => Resolved::DrawCards(repeated(amount)),
            Cost::DiscardCards(amount) => Resolved::DiscardCards(repeated(amount)),
            Cost::PutCountersOnSource { kind, amount } => Resolved::PutCounters {
                object: source,
                kind,
                amount,
                times: age,
            },
            Cost::SacrificePermanents {
                object,
                controller: crate::card::PlayerRelation::You,
                count,
            } => Resolved::SacrificePermanents {
                object,
                amount: repeated(u16::from(count)),
            },
            Cost::ExileTopCards(amount) => Resolved::ExileTopCards(repeated(amount)),
            Cost::AddMana(effect) => {
                let crate::card::ManaSelectionDef::One(crate::card::ManaTypeDef::Fixed(color)) =
                    effect.mana
                else {
                    panic!("unsupported cumulative-upkeep mana output")
                };
                assert!(
                    effect.also.is_none()
                        && effect.variable_amount.is_none()
                        && effect.amount_override.is_none()
                        && effect.damage_to_controller == 0
                        && effect.sacrifice_source_when_out_of.is_none()
                        && effect.restrictions.is_empty()
                        && effect.spend_effects.is_empty(),
                    "unsupported cumulative-upkeep mana output",
                );
                Resolved::AddMana {
                    color,
                    amount: repeated(effect.amount),
                }
            }
            Cost::GainLife {
                player: crate::card::PlayerRelation::Opponent,
                amount,
            } => Resolved::OpponentGainsLife(repeated(amount)),
            Cost::CreateTokens {
                player: crate::card::PlayerRelation::Opponent,
                token,
                amount,
            } => Resolved::OpponentCreatesTokens {
                token: *token,
                amount: repeated(amount),
            },
            Cost::GainControlPermanents { object, amount } => {
                Resolved::GainControlPermanents {
                    source,
                    object,
                    amount: repeated(amount),
                }
            }
            Cost::FlipCoins(amount) => Resolved::FlipCoins(repeated(amount)),
            _ => panic!("unsupported cumulative-upkeep cost"),
        }
    }
}

fn repeat_mana_cost(mut cost: crate::ManaCost, count: u16) -> crate::ManaCost {
    cost.generic = cost.generic.saturating_mul(count);
    cost.white = cost.white.saturating_mul(count);
    cost.blue = cost.blue.saturating_mul(count);
    cost.black = cost.black.saturating_mul(count);
    cost.red = cost.red.saturating_mul(count);
    cost.green = cost.green.saturating_mul(count);
    cost.colorless = cost.colorless.saturating_mul(count);
    for amount in &mut cost.hybrid {
        *amount = amount.saturating_mul(count);
    }
    for amount in &mut cost.additional_flexible {
        *amount = amount.saturating_mul(count);
    }
    cost.x_multiplier = cost.x_multiplier.saturating_mul(count);
    cost
}
