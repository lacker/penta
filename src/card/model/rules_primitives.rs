use super::ManaColor;

/// A kind of counter a permanent can carry. Only `PlusOnePlusOne` has rules
/// meaning of its own; the rest are named markers that the cards putting them
/// there give meaning to.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CounterKind {
    PlusOnePlusOne,
    Javelin,
    Muster,
    Charge,
    Loyalty,
}

impl CounterKind {
    pub const COUNT: usize = 5;

    pub const ALL: [Self; Self::COUNT] = [
        Self::PlusOnePlusOne,
        Self::Javelin,
        Self::Muster,
        Self::Charge,
        Self::Loyalty,
    ];

    #[must_use]
    pub const fn index(self) -> usize {
        match self {
            Self::PlusOnePlusOne => 0,
            Self::Javelin => 1,
            Self::Muster => 2,
            Self::Charge => 3,
            Self::Loyalty => 4,
        }
    }

    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::PlusOnePlusOne => "+1/+1",
            Self::Javelin => "javelin",
            Self::Muster => "muster",
            Self::Charge => "charge",
            Self::Loyalty => "loyalty",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CardSupertype {
    Basic,
    Legendary,
    Snow,
    World,
}

impl CardSupertype {
    pub const COUNT: usize = 4;

    pub const ALL: [Self; Self::COUNT] = [Self::Basic, Self::Legendary, Self::Snow, Self::World];

    #[must_use]
    pub const fn index(self) -> usize {
        match self {
            Self::Basic => 0,
            Self::Legendary => 1,
            Self::Snow => 2,
            Self::World => 3,
        }
    }

    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Basic => "Basic",
            Self::Legendary => "Legendary",
            Self::Snow => "Snow",
            Self::World => "World",
        }
    }
}

/// How completely the engine implements a card or independently modeled part.
///
/// Ordinary construction defaults to [`Self::Complete`]. Explanations live on
/// the non-declarative clause implementations that caused a non-complete
/// aggregate status, rather than being duplicated at card level.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum ImplementationStatus {
    #[default]
    Complete,
    Partial,
    MetadataOnly,
}

impl ImplementationStatus {
    #[must_use]
    pub const fn combine(self, other: Self) -> Self {
        match (self, other) {
            (Self::Partial, _)
            | (_, Self::Partial)
            | (Self::Complete, Self::MetadataOnly)
            | (Self::MetadataOnly, Self::Complete) => Self::Partial,
            (Self::MetadataOnly, Self::MetadataOnly) => Self::MetadataOnly,
            (Self::Complete, Self::Complete) => Self::Complete,
        }
    }

    #[must_use]
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Complete)
    }
}

/// Whether the current game engine may execute an independently modeled play
/// option or mode. Ordinary single-card options derive this gate from their
/// clause-local [`ImplementationStatus`] instead of storing another status on
/// [`CardRules`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CardEffectStatus {
    Implemented,
    MetadataOnly,
}

/// Which end of a library a card is put on.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum ZonePlacement {
    #[default]
    Top,
    Bottom,
}

/// One two-colour hybrid symbol, such as `{R/W}`. Either colour pays it.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum HybridPair {
    WhiteBlue,
    WhiteBlack,
    WhiteRed,
    WhiteGreen,
    BlueBlack,
    BlueRed,
    BlueGreen,
    BlackRed,
    BlackGreen,
    RedGreen,
}

impl HybridPair {
    pub const COUNT: usize = 10;

    pub const ALL: [Self; Self::COUNT] = [
        Self::WhiteBlue,
        Self::WhiteBlack,
        Self::WhiteRed,
        Self::WhiteGreen,
        Self::BlueBlack,
        Self::BlueRed,
        Self::BlueGreen,
        Self::BlackRed,
        Self::BlackGreen,
        Self::RedGreen,
    ];

    #[must_use]
    pub const fn index(self) -> usize {
        self as usize
    }

    /// The two colours, in the order Magic prints them.
    #[must_use]
    pub const fn colors(self) -> (ManaColor, ManaColor) {
        match self {
            Self::WhiteBlue => (ManaColor::White, ManaColor::Blue),
            Self::WhiteBlack => (ManaColor::White, ManaColor::Black),
            Self::WhiteRed => (ManaColor::White, ManaColor::Red),
            Self::WhiteGreen => (ManaColor::White, ManaColor::Green),
            Self::BlueBlack => (ManaColor::Blue, ManaColor::Black),
            Self::BlueRed => (ManaColor::Blue, ManaColor::Red),
            Self::BlueGreen => (ManaColor::Blue, ManaColor::Green),
            Self::BlackRed => (ManaColor::Black, ManaColor::Red),
            Self::BlackGreen => (ManaColor::Black, ManaColor::Green),
            Self::RedGreen => (ManaColor::Red, ManaColor::Green),
        }
    }

    #[must_use]
    pub const fn contains(self, color: ManaColor) -> bool {
        let (first, second) = self.colors();
        matches!(color, c if c as u8 == first as u8)
            || matches!(color, c if c as u8 == second as u8)
    }

    /// The printed symbol between the braces, such as `R/W`. Magic prints
    /// each pair in a fixed order that is not always alphabetical.
    #[must_use]
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::WhiteBlue => "W/U",
            Self::WhiteBlack => "W/B",
            Self::WhiteRed => "R/W",
            Self::WhiteGreen => "G/W",
            Self::BlueBlack => "U/B",
            Self::BlueRed => "U/R",
            Self::BlueGreen => "G/U",
            Self::BlackRed => "B/R",
            Self::BlackGreen => "B/G",
            Self::RedGreen => "R/G",
        }
    }

    /// Parses the two colour letters of a hybrid symbol. The printed order
    /// varies by pair, so both orders are accepted.
    #[must_use]
    pub const fn from_letters(first: u8, second: u8) -> Option<Self> {
        let Some(first) = ManaColor::from_letter(first) else {
            return None;
        };
        let Some(second) = ManaColor::from_letter(second) else {
            return None;
        };
        let mut index = 0;
        while index < Self::COUNT {
            let pair = Self::ALL[index];
            let (a, b) = pair.colors();
            if (a as u8 == first as u8 && b as u8 == second as u8)
                || (a as u8 == second as u8 && b as u8 == first as u8)
            {
                return Some(pair);
            }
            index += 1;
        }
        None
    }
}
