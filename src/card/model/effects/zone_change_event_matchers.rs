/// How a zone-change trigger observes the object that moved.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ZoneChangeObservationDef {
    /// Match the object as it existed immediately before it left its zone.
    Before,
    /// Match the new object created in the destination zone.
    After,
}

/// A matcher over one committed zone transition.
///
/// `previously_damaged_by` consults the damage-source history frozen as the
/// object leaves the battlefield. It therefore remains valid for simultaneous
/// deaths and never re-reads a fresh object in the destination zone.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ZoneChangeEventMatcherDef {
    pub object: ObjectPredicateDef,
    pub from: Option<ZoneKind>,
    pub to: Option<ZoneKind>,
    pub observation: ZoneChangeObservationDef,
    pub previously_damaged_by: Option<ObjectRefDef>,
}

impl ZoneChangeEventMatcherDef {
    #[must_use]
    pub const fn new(
        object: ObjectPredicateDef,
        from: Option<ZoneKind>,
        to: Option<ZoneKind>,
    ) -> Self {
        Self {
            object,
            from,
            to,
            // Leaves-the-battlefield triggers are the common look-back case.
            // Arrivals (including "from anywhere") are checked in the
            // destination against the new object. Less common look-back
            // clauses from another zone opt in with `observing_before`.
            observation: if matches!(from, Some(ZoneKind::Battlefield)) {
                ZoneChangeObservationDef::Before
            } else {
                ZoneChangeObservationDef::After
            },
            previously_damaged_by: None,
        }
    }

    #[must_use]
    pub const fn observing_before(mut self) -> Self {
        self.observation = ZoneChangeObservationDef::Before;
        self
    }

    #[must_use]
    pub const fn observing_after(mut self) -> Self {
        self.observation = ZoneChangeObservationDef::After;
        self
    }

    #[must_use]
    pub const fn previously_damaged_by(mut self, source: ObjectRefDef) -> Self {
        self.previously_damaged_by = Some(source);
        self
    }
}
