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
pub const fn evoke(cost: CostDef) -> [AbilityDef; 2] {
    let text = match cost {
        CostDef::Mana(_) => None,
        CostDef::Exile {
            object: ObjectPredicateDef::Color(color),
            from: ZoneKind::Hand,
            quantity: super::model::CostQuantityDef::Fixed(1),
        } => Some(match color {
            ManaColor::White => "Evoke—Exile a white card from your hand.",
            ManaColor::Blue => "Evoke—Exile a blue card from your hand.",
            ManaColor::Black => "Evoke—Exile a black card from your hand.",
            ManaColor::Red => "Evoke—Exile a red card from your hand.",
            ManaColor::Green => "Evoke—Exile a green card from your hand.",
            ManaColor::Colorless => panic!("use evoke_with_text for this nonmana cost"),
        }),
        _ => panic!("use evoke_with_text for this nonmana cost"),
    };
    evoke_expansion(cost, text)
}

/// Evoke with card-local wording; the cost and sacrifice still expand together.
#[must_use]
pub const fn evoke_with_text(cost: CostDef, text: &'static str) -> [AbilityDef; 2] {
    evoke_expansion(cost, Some(text))
}

const fn evoke_expansion(cost: CostDef, text: Option<&'static str>) -> [AbilityDef; 2] {
    let mut alternative = match cost {
        CostDef::Mana(mana) => AbilityDef::alternative_cast(
            mana,
            AlternativeCastKindDef::AlternativeCost,
            text,
            EffectDef::None,
        ),
        nonmana => AbilityDef::alternative_cast_with_additional_cost(
            super::model::AlternativeCastManaCostDef::Fixed(ManaCost::new(0, 0)),
            AlternativeCastKindDef::AlternativeCost,
            text,
            nonmana,
            EffectDef::None,
        ),
    }
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
            EffectDef::Sacrifice {
                object: EffectRecipientDef::Source,
            },
        ),
    ]
}

static EVOKE_PAID: TriggerConditionDef =
    TriggerConditionDef::SourcePaidAlternativeCost(crate::Binding!("evoke"));

#[cfg(test)]
mod evoke_tests {
    use super::*;
    use crate::card::{AlternativeCastManaCostDef, CostQuantityDef, DeclarativeAbilityDef};

    #[test]
    fn evoke_costs_and_text_remain_together_in_the_expansion() {
        let exile = CostDef::exile(
            ObjectPredicateDef::Color(ManaColor::White),
            ZoneKind::Hand,
            CostQuantityDef::Fixed(1),
        );
        for (cost, expansion, expected_text) in [
            (
                CostDef::Mana(crate::mana_cost!("{X}{W/U}")),
                evoke(CostDef::Mana(crate::mana_cost!("{X}{W/U}"))),
                "Evoke {X}{W/U}",
            ),
            (
                exile,
                evoke(exile),
                "Evoke—Exile a white card from your hand.",
            ),
            (
                CostDef::PayLife(2),
                evoke_with_text(CostDef::PayLife(2), "Evoke—Pay 2 life."),
                "Evoke—Pay 2 life.",
            ),
        ] {
            assert_eq!(expansion[0].rules_text(), expected_text);
            let DeclarativeAbilityDef::AlternativeCast(alternative) = expansion[0].definition
            else {
                panic!("the expansion starts with the alternative cost");
            };
            match cost {
                CostDef::Mana(mana) => {
                    assert_eq!(
                        alternative.mana_cost,
                        AlternativeCastManaCostDef::Fixed(mana)
                    );
                    assert_eq!(alternative.additional_cost, None);
                }
                nonmana => {
                    assert_eq!(
                        alternative.mana_cost,
                        AlternativeCastManaCostDef::Fixed(ManaCost::new(0, 0))
                    );
                    assert_eq!(alternative.additional_cost, Some(nonmana));
                }
            }
            let DeclarativeAbilityDef::Triggered(trigger) = expansion[1].definition else {
                panic!("the expansion includes a separate trigger");
            };
            assert_eq!(trigger.condition, Some(&EVOKE_PAID));
            assert_eq!(alternative.binding, Some(crate::Binding!("evoke")));
        }
    }
}
