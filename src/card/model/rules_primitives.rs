use super::ManaColor;

mod counters;
pub use counters::*;

include!("rules_primitives/supertypes.rs");

/// Whether the engine implements a complete card definition.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum ImplementationStatus {
    #[default]
    Complete,
    Unsupported,
}

impl ImplementationStatus {
    #[must_use]
    pub const fn combine(self, other: Self) -> Self {
        if matches!(self, Self::Complete) && matches!(other, Self::Complete) {
            Self::Complete
        } else {
            Self::Unsupported
        }
    }

    #[must_use]
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Complete)
    }
}

/// Whether the current game engine may execute a card's play option.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CardEffectStatus {
    Implemented,
    Unsupported,
}

/// Where in a library a card is put.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum ZonePlacement {
    #[default]
    Top,
    Bottom,
    /// A fixed depth counting the card itself: Teferi's "third from the top"
    /// is `FromTop(3)`, which leaves exactly two cards above it. A library
    /// with fewer cards than that has no such position, so the card goes to
    /// the bottom -- which is where counting down from the top runs out.
    ///
    /// Only a library has an inside to be put into. Every other zone is a
    /// set the rules read as a whole, so nothing outside the library moves
    /// reads this.
    FromTop(u8),
}

impl ZonePlacement {
    /// Where a card lands in a library of `size` cards, indexed from the
    /// bottom the way the library itself is stored.
    #[must_use]
    pub const fn library_index(self, size: usize) -> usize {
        match self {
            Self::Top => size,
            Self::Bottom => 0,
            // One card deep is the top, so the depth counts the card itself.
            Self::FromTop(depth) => size.saturating_sub(depth as usize - 1),
        }
    }
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

/// A printed mana symbol with more than one way to pay it.
///
/// The first ten variants are the ordinary two-colour hybrids represented by
/// [`HybridPair`]. The rest cover monocoloured hybrid ("two-brid"),
/// Phyrexian, two-colour Phyrexian, and colourless/colour hybrid symbols. A
/// cost stores ordinary pairs in its longstanding `hybrid` array and the
/// remaining variants in a second dense array; this vocabulary is the shared
/// semantic view over both so parsing, colour derivation, payment, and
/// presentation cannot disagree about what a symbol means.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FlexibleManaSymbol {
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
    TwoWhite,
    TwoBlue,
    TwoBlack,
    TwoRed,
    TwoGreen,
    WhitePhyrexian,
    BluePhyrexian,
    BlackPhyrexian,
    RedPhyrexian,
    GreenPhyrexian,
    WhiteBluePhyrexian,
    WhiteBlackPhyrexian,
    WhiteRedPhyrexian,
    WhiteGreenPhyrexian,
    BlueBlackPhyrexian,
    BlueRedPhyrexian,
    BlueGreenPhyrexian,
    BlackRedPhyrexian,
    BlackGreenPhyrexian,
    RedGreenPhyrexian,
    ColorlessWhite,
    ColorlessBlue,
    ColorlessBlack,
    ColorlessRed,
    ColorlessGreen,
}

impl FlexibleManaSymbol {
    pub const COUNT: usize = 35;
    pub const ADDITIONAL_COUNT: usize = Self::COUNT - HybridPair::COUNT;

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
        Self::TwoWhite,
        Self::TwoBlue,
        Self::TwoBlack,
        Self::TwoRed,
        Self::TwoGreen,
        Self::WhitePhyrexian,
        Self::BluePhyrexian,
        Self::BlackPhyrexian,
        Self::RedPhyrexian,
        Self::GreenPhyrexian,
        Self::WhiteBluePhyrexian,
        Self::WhiteBlackPhyrexian,
        Self::WhiteRedPhyrexian,
        Self::WhiteGreenPhyrexian,
        Self::BlueBlackPhyrexian,
        Self::BlueRedPhyrexian,
        Self::BlueGreenPhyrexian,
        Self::BlackRedPhyrexian,
        Self::BlackGreenPhyrexian,
        Self::RedGreenPhyrexian,
        Self::ColorlessWhite,
        Self::ColorlessBlue,
        Self::ColorlessBlack,
        Self::ColorlessRed,
        Self::ColorlessGreen,
    ];

    #[must_use]
    pub const fn index(self) -> usize {
        self as usize
    }

    /// The legacy ordinary-pair view, when this is a two-colour hybrid.
    #[must_use]
    pub const fn hybrid_pair(self) -> Option<HybridPair> {
        match self {
            Self::WhiteBlue => Some(HybridPair::WhiteBlue),
            Self::WhiteBlack => Some(HybridPair::WhiteBlack),
            Self::WhiteRed => Some(HybridPair::WhiteRed),
            Self::WhiteGreen => Some(HybridPair::WhiteGreen),
            Self::BlueBlack => Some(HybridPair::BlueBlack),
            Self::BlueRed => Some(HybridPair::BlueRed),
            Self::BlueGreen => Some(HybridPair::BlueGreen),
            Self::BlackRed => Some(HybridPair::BlackRed),
            Self::BlackGreen => Some(HybridPair::BlackGreen),
            Self::RedGreen => Some(HybridPair::RedGreen),
            Self::TwoWhite
            | Self::TwoBlue
            | Self::TwoBlack
            | Self::TwoRed
            | Self::TwoGreen
            | Self::WhitePhyrexian
            | Self::BluePhyrexian
            | Self::BlackPhyrexian
            | Self::RedPhyrexian
            | Self::GreenPhyrexian
            | Self::WhiteBluePhyrexian
            | Self::WhiteBlackPhyrexian
            | Self::WhiteRedPhyrexian
            | Self::WhiteGreenPhyrexian
            | Self::BlueBlackPhyrexian
            | Self::BlueRedPhyrexian
            | Self::BlueGreenPhyrexian
            | Self::BlackRedPhyrexian
            | Self::BlackGreenPhyrexian
            | Self::RedGreenPhyrexian
            | Self::ColorlessWhite
            | Self::ColorlessBlue
            | Self::ColorlessBlack
            | Self::ColorlessRed
            | Self::ColorlessGreen => None,
        }
    }

    /// The position in `ManaCost`'s additional-symbol storage.
    #[must_use]
    pub const fn additional_index(self) -> Option<usize> {
        let index = self.index();
        if index < HybridPair::COUNT {
            None
        } else {
            Some(index - HybridPair::COUNT)
        }
    }

    #[must_use]
    pub const fn from_hybrid_pair(pair: HybridPair) -> Self {
        Self::ALL[pair.index()]
    }

    #[must_use]
    pub const fn two_brid(color: ManaColor) -> Option<Self> {
        match color {
            ManaColor::White => Some(Self::TwoWhite),
            ManaColor::Blue => Some(Self::TwoBlue),
            ManaColor::Black => Some(Self::TwoBlack),
            ManaColor::Red => Some(Self::TwoRed),
            ManaColor::Green => Some(Self::TwoGreen),
            ManaColor::Colorless => None,
        }
    }

    #[must_use]
    pub const fn phyrexian(color: ManaColor) -> Option<Self> {
        match color {
            ManaColor::White => Some(Self::WhitePhyrexian),
            ManaColor::Blue => Some(Self::BluePhyrexian),
            ManaColor::Black => Some(Self::BlackPhyrexian),
            ManaColor::Red => Some(Self::RedPhyrexian),
            ManaColor::Green => Some(Self::GreenPhyrexian),
            ManaColor::Colorless => None,
        }
    }

    #[must_use]
    pub const fn phyrexian_hybrid(pair: HybridPair) -> Self {
        Self::ALL[Self::WhiteBluePhyrexian.index() + pair.index()]
    }

    #[must_use]
    pub const fn colorless_hybrid(color: ManaColor) -> Option<Self> {
        match color {
            ManaColor::White => Some(Self::ColorlessWhite),
            ManaColor::Blue => Some(Self::ColorlessBlue),
            ManaColor::Black => Some(Self::ColorlessBlack),
            ManaColor::Red => Some(Self::ColorlessRed),
            ManaColor::Green => Some(Self::ColorlessGreen),
            ManaColor::Colorless => None,
        }
    }

    /// The mana types that pay the one-mana branch of this symbol.
    ///
    /// Two-brid and monocolour Phyrexian symbols have only `first`; ordinary,
    /// Phyrexian-hybrid, and colourless-hybrid symbols have both. Life and
    /// generic alternatives are reported separately below.
    #[must_use]
    const fn mana_option_pair(self) -> (ManaColor, Option<ManaColor>) {
        if let Some(pair) = self.hybrid_pair() {
            let (first, second) = pair.colors();
            return (first, Some(second));
        }
        match self {
            Self::TwoWhite | Self::WhitePhyrexian => (ManaColor::White, None),
            Self::TwoBlue | Self::BluePhyrexian => (ManaColor::Blue, None),
            Self::TwoBlack | Self::BlackPhyrexian => (ManaColor::Black, None),
            Self::TwoRed | Self::RedPhyrexian => (ManaColor::Red, None),
            Self::TwoGreen | Self::GreenPhyrexian => (ManaColor::Green, None),
            Self::WhiteBluePhyrexian => (ManaColor::White, Some(ManaColor::Blue)),
            Self::WhiteBlackPhyrexian => (ManaColor::White, Some(ManaColor::Black)),
            Self::WhiteRedPhyrexian => (ManaColor::White, Some(ManaColor::Red)),
            Self::WhiteGreenPhyrexian => (ManaColor::White, Some(ManaColor::Green)),
            Self::BlueBlackPhyrexian => (ManaColor::Blue, Some(ManaColor::Black)),
            Self::BlueRedPhyrexian => (ManaColor::Blue, Some(ManaColor::Red)),
            Self::BlueGreenPhyrexian => (ManaColor::Blue, Some(ManaColor::Green)),
            Self::BlackRedPhyrexian => (ManaColor::Black, Some(ManaColor::Red)),
            Self::BlackGreenPhyrexian => (ManaColor::Black, Some(ManaColor::Green)),
            Self::RedGreenPhyrexian => (ManaColor::Red, Some(ManaColor::Green)),
            Self::ColorlessWhite => (ManaColor::Colorless, Some(ManaColor::White)),
            Self::ColorlessBlue => (ManaColor::Colorless, Some(ManaColor::Blue)),
            Self::ColorlessBlack => (ManaColor::Colorless, Some(ManaColor::Black)),
            Self::ColorlessRed => (ManaColor::Colorless, Some(ManaColor::Red)),
            Self::ColorlessGreen => (ManaColor::Colorless, Some(ManaColor::Green)),
            Self::WhiteBlue
            | Self::WhiteBlack
            | Self::WhiteRed
            | Self::WhiteGreen
            | Self::BlueBlack
            | Self::BlueRed
            | Self::BlueGreen
            | Self::BlackRed
            | Self::BlackGreen
            | Self::RedGreen => unreachable!(),
        }
    }

    /// The one or two mana types that can pay this symbol's mana branch.
    #[must_use]
    pub const fn mana_options(self) -> &'static [ManaColor] {
        match self.mana_option_pair() {
            (ManaColor::White, None) => &[ManaColor::White],
            (ManaColor::Blue, None) => &[ManaColor::Blue],
            (ManaColor::Black, None) => &[ManaColor::Black],
            (ManaColor::Red, None) => &[ManaColor::Red],
            (ManaColor::Green, None) => &[ManaColor::Green],
            (ManaColor::Colorless, None) => &[ManaColor::Colorless],
            (first, Some(second)) => match (first, second) {
                (ManaColor::White, ManaColor::Blue) => &[ManaColor::White, ManaColor::Blue],
                (ManaColor::White, ManaColor::Black) => &[ManaColor::White, ManaColor::Black],
                (ManaColor::White, ManaColor::Red) => &[ManaColor::White, ManaColor::Red],
                (ManaColor::White, ManaColor::Green) => &[ManaColor::White, ManaColor::Green],
                (ManaColor::Blue, ManaColor::Black) => &[ManaColor::Blue, ManaColor::Black],
                (ManaColor::Blue, ManaColor::Red) => &[ManaColor::Blue, ManaColor::Red],
                (ManaColor::Blue, ManaColor::Green) => &[ManaColor::Blue, ManaColor::Green],
                (ManaColor::Black, ManaColor::Red) => &[ManaColor::Black, ManaColor::Red],
                (ManaColor::Black, ManaColor::Green) => &[ManaColor::Black, ManaColor::Green],
                (ManaColor::Red, ManaColor::Green) => &[ManaColor::Red, ManaColor::Green],
                (ManaColor::Colorless, ManaColor::White) => {
                    &[ManaColor::Colorless, ManaColor::White]
                }
                (ManaColor::Colorless, ManaColor::Blue) => &[ManaColor::Colorless, ManaColor::Blue],
                (ManaColor::Colorless, ManaColor::Black) => {
                    &[ManaColor::Colorless, ManaColor::Black]
                }
                (ManaColor::Colorless, ManaColor::Red) => &[ManaColor::Colorless, ManaColor::Red],
                (ManaColor::Colorless, ManaColor::Green) => {
                    &[ManaColor::Colorless, ManaColor::Green]
                }
                _ => unreachable!(),
            },
        }
    }

    #[must_use]
    pub const fn first_mana(self) -> ManaColor {
        self.mana_option_pair().0
    }

    /// The generic branch of a two-brid symbol.
    #[must_use]
    pub const fn generic_alternative(self) -> Option<u16> {
        match self {
            Self::TwoWhite | Self::TwoBlue | Self::TwoBlack | Self::TwoRed | Self::TwoGreen => {
                Some(2)
            }
            _ => None,
        }
    }

    /// The life branch of a Phyrexian symbol. The mana solver deliberately
    /// does not spend it; a casting or activation action must choose it.
    #[must_use]
    pub const fn life_cost(self) -> Option<u16> {
        match self {
            Self::WhitePhyrexian
            | Self::BluePhyrexian
            | Self::BlackPhyrexian
            | Self::RedPhyrexian
            | Self::GreenPhyrexian
            | Self::WhiteBluePhyrexian
            | Self::WhiteBlackPhyrexian
            | Self::WhiteRedPhyrexian
            | Self::WhiteGreenPhyrexian
            | Self::BlueBlackPhyrexian
            | Self::BlueRedPhyrexian
            | Self::BlueGreenPhyrexian
            | Self::BlackRedPhyrexian
            | Self::BlackGreenPhyrexian
            | Self::RedGreenPhyrexian => Some(2),
            _ => None,
        }
    }

    #[must_use]
    pub const fn is_phyrexian(self) -> bool {
        self.life_cost().is_some()
    }

    /// Whether this symbol contains one coloured mana symbol. This is about
    /// card colour and devotion, not every mana type that can pay it.
    #[must_use]
    pub const fn contains_color(self, color: ManaColor) -> bool {
        if matches!(color, ManaColor::Colorless) {
            return false;
        }
        let (first, second) = self.mana_option_pair();
        first as u8 == color as u8 || matches!(second, Some(other) if other as u8 == color as u8)
    }

    /// Printed mana value contributed by one copy of this symbol.
    #[must_use]
    pub const fn mana_value(self) -> u16 {
        if self.generic_alternative().is_some() {
            2
        } else {
            1
        }
    }

    /// Canonical Oracle notation between braces.
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
            Self::TwoWhite => "2/W",
            Self::TwoBlue => "2/U",
            Self::TwoBlack => "2/B",
            Self::TwoRed => "2/R",
            Self::TwoGreen => "2/G",
            Self::WhitePhyrexian => "W/P",
            Self::BluePhyrexian => "U/P",
            Self::BlackPhyrexian => "B/P",
            Self::RedPhyrexian => "R/P",
            Self::GreenPhyrexian => "G/P",
            Self::WhiteBluePhyrexian => "W/U/P",
            Self::WhiteBlackPhyrexian => "W/B/P",
            Self::WhiteRedPhyrexian => "R/W/P",
            Self::WhiteGreenPhyrexian => "G/W/P",
            Self::BlueBlackPhyrexian => "U/B/P",
            Self::BlueRedPhyrexian => "U/R/P",
            Self::BlueGreenPhyrexian => "G/U/P",
            Self::BlackRedPhyrexian => "B/R/P",
            Self::BlackGreenPhyrexian => "B/G/P",
            Self::RedGreenPhyrexian => "R/G/P",
            Self::ColorlessWhite => "C/W",
            Self::ColorlessBlue => "C/U",
            Self::ColorlessBlack => "C/B",
            Self::ColorlessRed => "C/R",
            Self::ColorlessGreen => "C/G",
        }
    }
}
