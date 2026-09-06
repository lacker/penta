use super::CostDef;

/// A card action available before the first turn begins. Opening-hand actions
/// happen after every player has kept; mulligan actions happen while that card
/// is in a prospective opening hand. Neither uses the stack or priority.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PregameAbilityDef {
    pub timing: PregameTimingDef,
    pub condition: PregameConditionDef,
    pub costs: &'static [CostDef],
    pub reveals_source: bool,
}

impl PregameAbilityDef {
    #[must_use]
    pub const fn new(timing: PregameTimingDef) -> Self {
        Self {
            timing,
            condition: PregameConditionDef::Always,
            costs: &[],
            reveals_source: false,
        }
    }

    #[must_use]
    pub const fn with_condition(mut self, condition: PregameConditionDef) -> Self {
        self.condition = condition;
        self
    }

    #[must_use]
    pub const fn with_costs(mut self, costs: &'static [CostDef]) -> Self {
        self.costs = costs;
        self
    }

    #[must_use]
    pub const fn revealing_source(mut self) -> Self {
        self.reveals_source = true;
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PregameTimingDef {
    Mulligan,
    OpeningHand,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PregameConditionDef {
    Always,
    NotStartingPlayer,
}
