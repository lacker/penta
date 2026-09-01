use super::{CardBehavior, CardRules, UNSUPPORTED_RULES, y1993};

// This is deliberately only an index: special hooks and the legacy
// CardDefinition::new compatibility keys resolve to card-local rules.
pub(in crate::card) const fn rules(behavior: CardBehavior) -> &'static CardRules {
    match behavior {
        CardBehavior::Balance => &y1993::alpha::BALANCE.rules,
        CardBehavior::Fireball => &y1993::alpha::FIREBALL.rules,
        CardBehavior::Mountain => &y1993::alpha::MOUNTAIN.rules,
        CardBehavior::Plains => &y1993::alpha::PLAINS.rules,
        CardBehavior::Unsupported => &UNSUPPORTED_RULES,
    }
}
