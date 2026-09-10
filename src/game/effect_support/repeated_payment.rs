impl Game {
    pub(in crate::game) fn resolved_repeated_payment(
        cost: crate::card::CostDef,
        source: GameObjectId,
        times: u16,
        label: Option<crate::card::AbilityLabel>,
    ) -> crate::game::ResolvedEffectPayment {
        use crate::card::CostDef as Cost;
        use crate::game::ResolvedEffectPayment as Resolved;

        let repeated = |amount: u16| amount.saturating_mul(times);
        match cost {
            Cost::Mana(cost) => match label {
                Some(label) => Resolved::LabeledMana {
                    source,
                    label,
                    cost: repeat_mana_cost(cost, times),
                },
                None => Resolved::Mana(repeat_mana_cost(cost, times)),
            },
            Cost::SnowMana(amount) => Resolved::SnowMana {
                label,
                source,
                amount: repeated(amount),
            },
            Cost::Energy(amount) => Resolved::Energy(repeated(amount)),
            Cost::MillCards(amount) => Resolved::Mill(repeated(amount)),
            Cost::PayLife(amount) => Resolved::Life(repeated(amount)),
            Cost::DrawCards(amount) => Resolved::DrawCards(repeated(amount)),
            Cost::DiscardCards(amount) => Resolved::DiscardCards(repeated(amount)),
            Cost::PutCountersOnSource { kind, amount } => Resolved::PutCounters {
                object: source,
                kind,
                amount,
                times,
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
                    panic!("unsupported repeated payment mana output")
                };
                assert!(
                    effect.also.is_none()
                        && effect.variable_amount.is_none()
                        && effect.amount_override.is_none()
                        && effect.damage_to_controller == 0
                        && effect.sacrifice_source_when_out_of.is_none()
                        && effect.restrictions.is_empty()
                        && effect.spend_effects.is_empty(),
                    "unsupported repeated payment mana output",
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
            Cost::FlipCoins(amount) => Resolved::FlipCoins(repeated(amount)),
            _ => panic!("unsupported repeated payment cost"),
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
