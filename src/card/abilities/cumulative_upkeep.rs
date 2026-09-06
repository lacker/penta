/// Cumulative upkeep (CR 702.24): one age counter, then one indivisible
/// payment containing the unit cost once for each age counter on the source.
///
/// Common printed costs receive their Oracle reminder text here. A card with
/// different wording can call [`AbilityDef::override_text`] on the result.
#[must_use]
pub const fn cumulative_upkeep(cost: CostDef) -> AbilityDef {
    let text = match cost {
        CostDef::Mana(cost) if mana_cost_is_generic(cost, 1) => {
            "Cumulative upkeep {1} (At the beginning of your upkeep, put an age counter on this permanent, then sacrifice it unless you pay its upkeep cost for each age counter on it.)"
        }
        CostDef::Mana(cost) if mana_cost_is_generic(cost, 2) => {
            "Cumulative upkeep {2} (At the beginning of your upkeep, put an age counter on this permanent, then sacrifice it unless you pay its upkeep cost for each age counter on it.)"
        }
        CostDef::Mana(cost) if mana_cost_is_green(cost, 1) => {
            "Cumulative upkeep {G} (At the beginning of your upkeep, put an age counter on this permanent, then sacrifice it unless you pay its upkeep cost for each age counter on it.)"
        }
        CostDef::PayLife(1) => {
            "Cumulative upkeep—Pay 1 life. (At the beginning of your upkeep, put an age counter on this permanent, then sacrifice it unless you pay its upkeep cost for each age counter on it.)"
        }
        CostDef::PayLife(2) => {
            "Cumulative upkeep—Pay 2 life. (At the beginning of your upkeep, put an age counter on this permanent, then sacrifice it unless you pay its upkeep cost for each age counter on it.)"
        }
        CostDef::DrawCards(1) => {
            "Cumulative upkeep—Draw a card. (At the beginning of your upkeep, put an age counter on this permanent, then sacrifice it unless you pay its upkeep cost for each age counter on it.)"
        }
        CostDef::DiscardCards(1) => {
            "Cumulative upkeep—Discard a card. (At the beginning of your upkeep, put an age counter on this permanent, then sacrifice it unless you pay its upkeep cost for each age counter on it.)"
        }
        CostDef::PutCountersOnSource {
            kind: CounterKind::MinusOneMinusOne,
            amount: 1,
        } => {
            "Cumulative upkeep—Put a -1/-1 counter on this creature. (At the beginning of your upkeep, put an age counter on this permanent, then sacrifice it unless you pay its upkeep cost for each age counter on it.)"
        }
        _ => "Cumulative upkeep",
    };
    AbilityDef::triggered(
        text,
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::You,
        },
        EffectDef::CumulativeUpkeep(cost),
    )
}

const fn mana_cost_is_generic(cost: ManaCost, amount: u16) -> bool {
    cost.generic == amount
        && cost.white == 0
        && cost.blue == 0
        && cost.black == 0
        && cost.red == 0
        && cost.green == 0
        && cost.colorless == 0
        && all_zero(&cost.hybrid)
        && all_zero(&cost.additional_flexible)
        && !cost.variable_x
        && cost.x_multiplier == 0
}

const fn mana_cost_is_green(cost: ManaCost, amount: u16) -> bool {
    cost.generic == 0
        && cost.white == 0
        && cost.blue == 0
        && cost.black == 0
        && cost.red == 0
        && cost.green == amount
        && cost.colorless == 0
        && all_zero(&cost.hybrid)
        && all_zero(&cost.additional_flexible)
        && !cost.variable_x
        && cost.x_multiplier == 0
}

const fn all_zero(values: &[u16]) -> bool {
    let mut index = 0;
    while index < values.len() {
        if values[index] != 0 {
            return false;
        }
        index += 1;
    }
    true
}
