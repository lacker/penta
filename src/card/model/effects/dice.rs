/// A fair numbered die and its result table. Each upper bound is inclusive;
/// the first range starts at one and later ranges start after the previous bound.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RollDieDef {
    sides: u16,
    outcomes: &'static [(u16, EffectDef)],
}

impl RollDieDef {
    /// Construct a complete, ordered partition of the die's faces.
    ///
    /// # Panics
    /// Panics if there are no faces, or the ranges are unordered, out of bounds,
    /// or do not cover every face.
    #[must_use]
    pub const fn new(sides: u16, outcomes: &'static [(u16, EffectDef)]) -> Self {
        assert!(sides > 0, "a die must have at least one face");
        let mut previous = 0;
        let mut index = 0;
        while index < outcomes.len() {
            let upper = outcomes[index].0;
            assert!(
                upper > previous && upper <= sides,
                "die ranges must be ordered and in bounds"
            );
            previous = upper;
            index += 1;
        }
        assert!(previous == sides, "die ranges must cover every face");
        Self { sides, outcomes }
    }

    #[must_use]
    pub const fn sides(self) -> u16 {
        self.sides
    }

    #[must_use]
    pub const fn outcomes(self) -> &'static [(u16, EffectDef)] {
        self.outcomes
    }

    /// Return the effect for a one-based die result.
    ///
    /// # Panics
    /// Panics if the result is outside `1..=self.sides()`.
    #[must_use]
    pub fn outcome(self, result: u16) -> EffectDef {
        assert!(
            result > 0 && result <= self.sides,
            "die result out of bounds"
        );
        self.outcomes
            .iter()
            .find(|(upper, _)| result <= *upper)
            .unwrap()
            .1
    }
}
