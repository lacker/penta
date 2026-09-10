// Named costs use ordinary cost grammar. This first selection adapter supports
// independent fixed object alternatives; it does not admit arbitrary programs.
impl CostDef {
    pub(crate) fn named_choices(self) -> Option<&'static [Self]> {
        let Self::Named { cost, .. } = self else {
            return None;
        };
        let choices = match cost {
            Self::Choice(choices) => *choices,
            cost => std::slice::from_ref(cost),
        };
        (!choices.is_empty()
            && choices
                .iter()
                .all(|cost| cost.named_object_selection().is_some()))
        .then_some(choices)
    }

    pub(crate) const fn named_object_selection(
        self,
    ) -> Option<(ObjectPredicateDef, ZoneKind, u16)> {
        match self {
            Self::Sacrifice {
                object,
                quantity: CostQuantityDef::Fixed(count),
            } if count > 0 => Some((object, ZoneKind::Battlefield, count as u16)),
            Self::Exile {
                object,
                from: ZoneKind::Graveyard,
                quantity: CostQuantityDef::Fixed(count),
            } if count > 0 => Some((object, ZoneKind::Graveyard, count as u16)),
            _ => None,
        }
    }
}
