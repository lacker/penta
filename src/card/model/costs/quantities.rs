/// The quantity a semantic cost asks its payer to provide.
///
/// Fixed, chosen-X, mode-count, and arithmetic quantities apply to scalar
/// costs such as paying life as well as object costs. The threshold variants
/// describe sets of objects and are only meaningful for costs that choose
/// cards or permanents.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CostQuantityDef {
    Fixed(u8),
    /// The X announced for the spell or ability.
    ChosenX,
    /// How many modes were selected for the spell being cast.
    ModeCount,
    /// How many targets the spell names as it is cast. Repeated targets in
    /// separate slots each count because this is the number of targets, not
    /// the number of distinct objects or players targeted.
    TargetCount,
    /// The left quantity minus the right, floored at zero because a cost
    /// cannot ask for a negative quantity.
    Subtract(&'static Self, &'static Self),
    /// Choose a minimal set whose composed value reaches a threshold.
    ObjectSetValueAtLeast(&'static ObjectSetValueAtLeastDef),
}

/// A scalar derived from the objects chosen to pay one cost.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ObjectSetValueDef {
    Aggregate {
        select: ObjectValueDef,
        operation: AggregateOperationDef,
    },
    CardTypeCount,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ObjectSetValueAtLeastDef {
    pub value: ObjectSetValueDef,
    pub minimum: u16,
}

/// A chosen-object cost whose payment is a zone change.
///
/// The destination is explicit rather than inferred from the source zone, so
/// the same shape covers exiling from a graveyard, discarding from a hand,
/// returning a permanent, and future forced moves without another enum case.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MoveToZoneCostDef {
    pub object: ObjectPredicateDef,
    pub from: ZoneKind,
    pub to: ZoneKind,
    pub quantity: CostQuantityDef,
    /// Saves the paid objects' successor identities for another cost or the
    /// resolving effect. A single-object binding requires a fixed count of 1.
    pub binding: Option<Binding>,
}

impl MoveToZoneCostDef {
    #[must_use]
    pub const fn new(object: ObjectPredicateDef, from: ZoneKind, to: ZoneKind, count: u8) -> Self {
        Self::with_quantity(object, from, to, CostQuantityDef::Fixed(count))
    }

    #[must_use]
    pub const fn with_quantity(
        object: ObjectPredicateDef,
        from: ZoneKind,
        to: ZoneKind,
        quantity: CostQuantityDef,
    ) -> Self {
        Self {
            object,
            from,
            to,
            quantity,
            binding: None,
        }
    }

    #[must_use]
    pub const fn chosen_x(object: ObjectPredicateDef, from: ZoneKind, to: ZoneKind) -> Self {
        Self::with_quantity(object, from, to, CostQuantityDef::ChosenX)
    }

    #[must_use]
    pub const fn fixed_count(self) -> Option<u8> {
        self.quantity.fixed_value()
    }

    #[must_use]
    pub const fn binding(mut self, binding: Binding) -> Self {
        self.binding = Some(binding);
        self
    }
}

impl CostQuantityDef {
    /// Resolves an expression made entirely from fixed quantities.
    #[must_use]
    pub const fn fixed_value(self) -> Option<u8> {
        match self {
            Self::Fixed(value) => Some(value),
            Self::Subtract(left, right) => {
                let (Some(left), Some(right)) = (left.fixed_value(), right.fixed_value()) else {
                    return None;
                };
                Some(left.saturating_sub(right))
            }
            Self::ChosenX
            | Self::ModeCount
            | Self::TargetCount
            | Self::ObjectSetValueAtLeast(_) => None,
        }
    }
}
