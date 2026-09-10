/// Evoke (CR 702.74), expanded into its alternative cost and its separate
/// enters-the-battlefield sacrifice trigger. Both clauses share the cost
/// binding here, so card declarations cannot omit the rider accidentally.
/// Use [`crate::ability_list!`] to include the entire expansion in a card.
///
/// Mana costs and the colored-card exile costs use derived/default text.
/// Other nonmana costs can use [`evoke_with_text`].
///
/// # Panics
///
/// Panics for nonmana costs without default wording; use `evoke_with_text`.
#[must_use]
pub const fn evoke(costs: &'static [CostDef]) -> [AbilityDef; 2] {
    let text = match costs {
        [CostDef::Mana(_)] => None,
        [
            CostDef::Exile {
                object: ObjectPredicateDef::Color(color),
                from: ZoneKind::Hand,
                quantity: super::model::CostQuantityDef::Fixed(1),
            },
        ] => Some(match color {
            ManaColor::White => "Evoke—Exile a white card from your hand.",
            ManaColor::Blue => "Evoke—Exile a blue card from your hand.",
            ManaColor::Black => "Evoke—Exile a black card from your hand.",
            ManaColor::Red => "Evoke—Exile a red card from your hand.",
            ManaColor::Green => "Evoke—Exile a green card from your hand.",
            ManaColor::Colorless => panic!("use evoke_with_text for this nonmana cost"),
        }),
        _ => panic!("use evoke_with_text for this nonmana cost"),
    };
    evoke_expansion(costs, text)
}

/// Evoke with card-local wording; the cost and sacrifice still expand together.
#[must_use]
pub const fn evoke_with_text(costs: &'static [CostDef], text: &'static str) -> [AbilityDef; 2] {
    evoke_expansion(costs, Some(text))
}

const fn evoke_expansion(costs: &'static [CostDef], text: Option<&'static str>) -> [AbilityDef; 2] {
    let mut alternative = AbilityDef::alternative_cast(
        costs,
        AlternativeCastKindDef::AlternativeCost,
        text,
        EffectDef::None,
    )
    .with_alternative_cost_binding(crate::Binding!("evoke"));
    if let super::model::DeclarativeAbilityDef::AlternativeCast(mut definition) =
        alternative.definition
    {
        definition.cost_header = Some("Evoke");
        alternative.definition = super::model::DeclarativeAbilityDef::AlternativeCast(definition);
    }
    [
        alternative,
        AbilityDef::triggered_if(
            "When this creature enters, if it was evoked, sacrifice it.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &EVOKE_PAID,
            actions::sacrifice(EffectRecipientDef::Source).as_effect(),
        ),
    ]
}

static EVOKE_PAID: TriggerConditionDef =
    TriggerConditionDef::SourcePaidAlternativeCost(crate::Binding!("evoke"));

#[cfg(test)]
mod evoke_tests {
    use super::*;
    use crate::card::{CostQuantityDef, DeclarativeAbilityDef};

    #[test]
    fn evoke_costs_and_text_remain_together_in_the_expansion() {
        const EXILE: CostDef = CostDef::exile(
            ObjectPredicateDef::Color(ManaColor::White),
            ZoneKind::Hand,
            CostQuantityDef::Fixed(1),
        );
        for (cost, expansion, expected_text) in [
            (
                CostDef::Mana(crate::mana_cost!("{X}{W/U}")),
                evoke(&[CostDef::Mana(crate::mana_cost!("{X}{W/U}"))]),
                "Evoke {X}{W/U}",
            ),
            (
                EXILE,
                evoke(&[EXILE]),
                "Evoke—Exile a white card from your hand.",
            ),
            (
                CostDef::PayLife(2),
                evoke_with_text(&[CostDef::PayLife(2)], "Evoke—Pay 2 life."),
                "Evoke—Pay 2 life.",
            ),
        ] {
            assert_eq!(expansion[0].rules_text(), expected_text);
            let DeclarativeAbilityDef::AlternativeCast(alternative) = expansion[0].definition
            else {
                panic!("the expansion starts with the alternative cost");
            };
            assert_eq!(alternative.costs, &[cost]);
            let DeclarativeAbilityDef::Triggered(trigger) = expansion[1].definition else {
                panic!("the expansion includes a separate trigger");
            };
            assert_eq!(trigger.condition, Some(&EVOKE_PAID));
            assert_eq!(alternative.binding, Some(crate::Binding!("evoke")));
        }
    }
}
