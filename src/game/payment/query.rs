//! Announcement queries inspect an isolated game projection. Search limits do
//! not decide the legality of a player-supplied payment.

#[derive(Clone, Debug, Default)]
pub(in crate::game) struct PaymentQuery {
    unfunded: bool,
    x: Option<u16>,
    minimum_only: bool,
}

impl PaymentQuery {
    pub(in crate::game) fn announcement() -> Self {
        Self {
            unfunded: true,
            x: None,
            minimum_only: false,
        }
    }
    pub(in crate::game) fn with_x(x: u16) -> Self {
        Self {
            unfunded: true,
            x: Some(x),
            minimum_only: false,
        }
    }
    pub(in crate::game) fn minimum_announcement() -> Self {
        Self {
            minimum_only: true,
            ..Self::announcement()
        }
    }
    pub(in crate::game) fn unfunded(&self) -> bool {
        self.unfunded
    }
    pub(in crate::game) fn x_values(
        &self,
        minimum: u16,
        maximum: u16,
    ) -> std::ops::RangeInclusive<u16> {
        if self.minimum_only {
            return minimum..=minimum;
        }
        match self.x {
            Some(x) if maximum > 0 && (minimum..=maximum).contains(&x) => x..=x,
            Some(_) if minimum > 0 || maximum > 0 => std::ops::RangeInclusive::new(1, 0),
            _ => minimum..=maximum,
        }
    }
}
