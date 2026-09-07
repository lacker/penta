/// The shared rules identity, independent of this program's source location.
pub const CUMULATIVE_UPKEEP: crate::MechanicId =
    crate::MechanicId::from_name("mtg:cumulative-upkeep");

/// Construct the static program at its declaration site. A macro permits
/// Rust to promote the parameterized sequence without adding a special
/// effect variant or allocating runtime definitions.
macro_rules! cumulative_upkeep {
    ($cost:expr $(,)?) => {{
        const UNIT: $crate::card::CostDef = $cost;
        const REPEATED: $crate::card::CostDef = $crate::card::CostDef::Repeat {
            cost: &UNIT,
            times: $crate::card::ValueDef::CountersOnSource($crate::CounterKind::named("age")),
        };
        const STEPS: [$crate::card::EffectDef; 2] = [
            $crate::card::EffectDef::AddCounters {
                object: $crate::card::EffectRecipientDef::Source,
                kind: $crate::CounterKind::named("age"),
                amount: $crate::card::ValueDef::Constant(1),
            },
            $crate::card::EffectDef::PayOr($crate::card::PayOrDef::unless(
                $crate::card::EffectPaymentDef {
                    payer: $crate::card::PlayerSetDef::One($crate::card::PlayerRefDef::EffectController),
                    cost: $crate::card::CostDef::Named {
                        mechanic: $crate::card::abilities::CUMULATIVE_UPKEEP,
                        cost: &REPEATED,
                    },
                },
                &$crate::card::EffectDef::Sacrifice {
                    object: $crate::card::EffectRecipientDef::Source,
                },
            )),
        ];
        $crate::card::abilities::cumulative_upkeep_ability(
            UNIT,
            &$crate::card::EffectDef::Sequence(&STEPS),
        )
    }};
}
pub(crate) use cumulative_upkeep;

/// The clauses remain ordinary effects; wording stays in shared vocabulary.
#[must_use]
pub const fn cumulative_upkeep_ability(cost: CostDef, program: &'static EffectDef) -> AbilityDef {
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
        CostDef::Discard { object: crate::card::ObjectPredicateDef::Any, quantity: crate::card::CostQuantityDef::Fixed(1) } => {
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
        EffectDef::IfCondition {
            condition: &TriggerConditionDef::SourceOnBattlefield,
            then: program,
        },
    ).with_mechanics(&[CUMULATIVE_UPKEEP])
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
