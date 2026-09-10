/// A compact cost clause when the expressions have context-free wording.
/// Predicates and references with card-specific wording use authored text.
#[must_use]
pub(crate) fn rules_text(costs: &[CostDef]) -> Option<String> {
    if costs.is_empty() {
        return Some("Pay no cost".into());
    }
    costs
        .iter()
        .map(cost_rules_text)
        .collect::<Option<Vec<_>>>()
        .map(|parts| parts.join(", "))
}

fn cost_rules_text(cost: &CostDef) -> Option<String> {
    Some(match cost {
        CostDef::Mana(mana) => mana.to_string(),
        CostDef::ManaCostOf(ObjectRefDef::Source) => "this card's mana cost".into(),
        CostDef::PayLife(amount) => format!("Pay {amount} life"),
        CostDef::DiscardCards(1) => "Discard a card".into(),
        CostDef::DiscardCards(amount) => format!("Discard {amount} cards"),
        CostDef::DiscardHand => "Discard your hand".into(),
        CostDef::DiscardSource => "Discard this card".into(),
        CostDef::SacrificeSource => "Sacrifice this permanent".into(),
        CostDef::TapSource => "{T}".into(),
        CostDef::UntapSource => "{Q}".into(),
        CostDef::ExileSource => "Exile this card".into(),
        CostDef::ReturnSourceToHand => "Return this permanent to its owner's hand".into(),
        CostDef::DrawCards(1) => "Draw a card".into(),
        CostDef::DrawCards(amount) => format!("Draw {amount} cards"),
        CostDef::MillCards(amount) => format!("Mill {amount} cards"),
        CostDef::Energy(amount) => format!("Pay {}", "{E}".repeat(usize::from(*amount))),
        CostDef::All(costs) => rules_text(costs)?,
        _ => return None,
    })
}
