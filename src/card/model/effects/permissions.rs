/// A permission to play selected cards from their current zones, with ordinary play restrictions.
///
/// Crucible's line is unbounded: as many lands as your land drops allow, on
/// anybody's turn. Lurrus prints the other shape -- one such spell, and only
/// during your own turns -- and the difference belongs to the permission
/// rather than to what it names.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PlayPermissionDef {
    pub cards: ObjectQueryDef,
    pub restriction: PlayRestrictionDef,
    pub cost: PlayCostDef,
    pub benefit: Option<&'static PlayBenefitDef>,
    /// How many plays each qualifying turn allows. `None` is as many as the
    /// rest of the rules permit.
    pub per_turn: Option<u8>,
    /// Whether it opens only on its controller's own turns.
    pub your_turns_only: bool,
    /// "If you do, it gains ...": what the permanent played this way carries
    /// afterwards. It belongs to the permission because the permission is
    /// the only thing that knows a play was made under it -- and it outlives
    /// the permission's own source, which is why it rides on the permanent
    /// rather than being read back off the card that allowed it.
    pub grants: Option<&'static AppliedEffectDef>,
}

impl PlayPermissionDef {
    #[must_use]
    pub const fn new(cards: ObjectQueryDef, restriction: PlayRestrictionDef) -> Self {
        Self {
            cards,
            restriction,
            cost: PlayCostDef::Printed,
            benefit: None,
            per_turn: None,
            your_turns_only: false,
            grants: None,
        }
    }

    /// "Once during each of your turns, you may cast ..."
    #[must_use]
    pub const fn once_each_of_your_turns(
        cards: ObjectQueryDef,
        restriction: PlayRestrictionDef,
    ) -> Self {
        Self {
            cards,
            restriction,
            cost: PlayCostDef::Printed,
            benefit: None,
            per_turn: Some(1),
            your_turns_only: true,
            grants: None,
        }
    }

    #[must_use]
    pub const fn with_cost(mut self, cost: PlayCostDef) -> Self {
        self.cost = cost;
        self
    }

    #[must_use]
    pub const fn with_benefit(mut self, benefit: Option<&'static PlayBenefitDef>) -> Self {
        self.benefit = benefit;
        self
    }

    #[must_use]
    pub const fn with_limit(mut self, plays: u8, your_turns_only: bool) -> Self {
        self.per_turn = Some(plays);
        self.your_turns_only = your_turns_only;
        self
    }

    /// The same permission, with what it played gaining `effect`.
    #[must_use]
    pub const fn granting(mut self, effect: &'static AppliedEffectDef) -> Self {
        self.grants = Some(effect);
        self
    }
}

/// A resolving permission over exact card objects. Movement is composed separately,
/// so bindings preserve the particular successor rather than granting access to a zone.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ZonePlayGrantDef {
    pub objects: ObjectSetDef,
    pub player: PlayerRefDef,
    /// An alternative mana cost; `None` retains the printed cost.
    pub mana_cost: Option<ManaCost>,
    pub duration: ExilePlayDurationDef,
    pub cast_only: bool,
}

/// Which mana types a player may treat their mana as while paying a spell.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ManaSpendAsDef {
    AnyColor,
    AnyType,
}
