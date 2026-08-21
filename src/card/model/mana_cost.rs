use std::error::Error;
use std::fmt;
use std::str::FromStr;

use super::{FlexibleManaSymbol, HybridPair, ManaColor};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct ManaCost {
    pub generic: u16,
    pub white: u16,
    pub blue: u16,
    pub black: u16,
    pub red: u16,
    pub green: u16,
    /// How many `{C}` symbols this cost carries. Unlike `generic`, which any
    /// mana pays, these can only be paid with colorless mana.
    pub colorless: u16,
    /// How many hybrid symbols of each colour pair this cost carries, indexed
    /// by [`HybridPair::index`].
    pub hybrid: [u16; HybridPair::COUNT],
    /// Flexible symbols beyond ordinary two-colour hybrid, indexed by
    /// [`FlexibleManaSymbol::additional_index`]. Ordinary pairs remain in
    /// `hybrid` so the longstanding public representation stays compatible.
    pub additional_flexible: [u16; FlexibleManaSymbol::ADDITIONAL_COUNT],
    pub variable_x: bool,
    pub x_multiplier: u16,
}

/// Why a symbolic mana-cost string could not be represented by [`ManaCost`].
///
/// Penta accepts the canonical braced notation used by Oracle, such as
/// `{2}{G}{G}` or `{X}{R}`. Symbols outside the engine's current mana model
/// are rejected instead of being approximated.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ManaCostParseError {
    pub offset: usize,
    pub kind: ManaCostParseErrorKind,
}

impl ManaCostParseError {
    const fn new(offset: usize, kind: ManaCostParseErrorKind) -> Self {
        Self { offset, kind }
    }
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ManaCostParseErrorKind {
    Empty,
    ExpectedOpeningBrace,
    UnterminatedSymbol,
    EmptySymbol,
    InvalidSymbol,
    DuplicateGenericSymbol,
    Overflow,
}

impl fmt::Display for ManaCostParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let problem = match self.kind {
            ManaCostParseErrorKind::Empty => "a mana cost cannot be empty",
            ManaCostParseErrorKind::ExpectedOpeningBrace => {
                "each mana symbol must start with an opening brace"
            }
            ManaCostParseErrorKind::UnterminatedSymbol => {
                "a mana symbol is missing its closing brace"
            }
            ManaCostParseErrorKind::EmptySymbol => "a mana symbol cannot be empty",
            ManaCostParseErrorKind::InvalidSymbol => {
                "the mana symbol is invalid or unsupported by the current engine"
            }
            ManaCostParseErrorKind::DuplicateGenericSymbol => {
                "a mana cost may contain only one numeric generic symbol"
            }
            ManaCostParseErrorKind::Overflow => "the mana cost exceeds the supported numeric range",
        };
        write!(formatter, "{problem} at byte {}", self.offset)
    }
}

impl Error for ManaCostParseError {}

impl ManaCost {
    /// Parses canonical braced mana symbols without allocating.
    ///
    /// This is `const` so [`crate::mana_cost!`] can validate literals during
    /// compilation. Runtime callers will usually prefer `str::parse`, which
    /// uses the same parser through [`FromStr`]. An empty string is invalid:
    /// a card with no mana cost is represented by [`PrintedManaCost::None`],
    /// while `{0}` is a real, payable printed cost.
    ///
    /// # Errors
    ///
    /// Returns a [`ManaCostParseError`] at the first malformed or currently
    /// unsupported symbol, duplicate numeric symbol, or numeric overflow.
    #[allow(clippy::too_many_lines)]
    pub const fn parse_symbols(symbols: &str) -> Result<Self, ManaCostParseError> {
        let bytes = symbols.as_bytes();
        if bytes.is_empty() {
            return Err(ManaCostParseError::new(0, ManaCostParseErrorKind::Empty));
        }

        let mut cost = Self {
            generic: 0,
            white: 0,
            blue: 0,
            black: 0,
            red: 0,
            green: 0,
            colorless: 0,
            hybrid: [0; HybridPair::COUNT],
            additional_flexible: [0; FlexibleManaSymbol::ADDITIONAL_COUNT],
            variable_x: false,
            x_multiplier: 0,
        };
        let mut offset = 0;
        let mut saw_generic = false;

        while offset < bytes.len() {
            if bytes[offset] != b'{' {
                return Err(ManaCostParseError::new(
                    offset,
                    ManaCostParseErrorKind::ExpectedOpeningBrace,
                ));
            }
            let symbol_start = offset + 1;
            let mut symbol_end = symbol_start;
            while symbol_end < bytes.len() && bytes[symbol_end] != b'}' {
                symbol_end += 1;
            }
            if symbol_end == bytes.len() {
                return Err(ManaCostParseError::new(
                    offset,
                    ManaCostParseErrorKind::UnterminatedSymbol,
                ));
            }
            if symbol_end == symbol_start {
                return Err(ManaCostParseError::new(
                    symbol_start,
                    ManaCostParseErrorKind::EmptySymbol,
                ));
            }

            let symbol_len = symbol_end - symbol_start;
            if symbol_len == 1 {
                let parsed = match bytes[symbol_start] {
                    b'W' => Self::checked_increment(cost.white),
                    b'U' => Self::checked_increment(cost.blue),
                    b'B' => Self::checked_increment(cost.black),
                    b'R' => Self::checked_increment(cost.red),
                    b'G' => Self::checked_increment(cost.green),
                    b'C' => Self::checked_increment(cost.colorless),
                    b'X' => Self::checked_increment(cost.x_multiplier),
                    b'0'..=b'9' => {
                        if saw_generic {
                            return Err(ManaCostParseError::new(
                                symbol_start,
                                ManaCostParseErrorKind::DuplicateGenericSymbol,
                            ));
                        }
                        saw_generic = true;
                        Ok((bytes[symbol_start] - b'0') as u16)
                    }
                    _ => Err(ManaCostParseErrorKind::InvalidSymbol),
                };
                let value = match parsed {
                    Ok(value) => value,
                    Err(kind) => return Err(ManaCostParseError::new(symbol_start, kind)),
                };
                match bytes[symbol_start] {
                    b'W' => cost.white = value,
                    b'U' => cost.blue = value,
                    b'B' => cost.black = value,
                    b'R' => cost.red = value,
                    b'G' => cost.green = value,
                    b'C' => cost.colorless = value,
                    b'X' => {
                        cost.variable_x = true;
                        cost.x_multiplier = value;
                    }
                    b'0'..=b'9' => cost.generic = value,
                    _ => {}
                }
            } else if let Some(symbol) =
                Self::parse_flexible_symbol(bytes, symbol_start, symbol_end)
            {
                let value = match Self::checked_increment(cost.flexible_count(symbol)) {
                    Ok(value) => value,
                    Err(kind) => return Err(ManaCostParseError::new(symbol_start, kind)),
                };
                cost = cost.with_flexible_symbol(symbol, value);
            } else {
                let first = bytes[symbol_start];
                if !first.is_ascii_digit() {
                    return Err(ManaCostParseError::new(
                        symbol_start,
                        ManaCostParseErrorKind::InvalidSymbol,
                    ));
                }
                if saw_generic {
                    return Err(ManaCostParseError::new(
                        symbol_start,
                        ManaCostParseErrorKind::DuplicateGenericSymbol,
                    ));
                }
                if first == b'0' {
                    return Err(ManaCostParseError::new(
                        symbol_start,
                        ManaCostParseErrorKind::InvalidSymbol,
                    ));
                }
                let mut value = 0_u16;
                let mut digit = symbol_start;
                while digit < symbol_end {
                    let byte = bytes[digit];
                    if !byte.is_ascii_digit() {
                        return Err(ManaCostParseError::new(
                            digit,
                            ManaCostParseErrorKind::InvalidSymbol,
                        ));
                    }
                    value = match value.checked_mul(10) {
                        Some(value) => value,
                        None => {
                            return Err(ManaCostParseError::new(
                                symbol_start,
                                ManaCostParseErrorKind::Overflow,
                            ));
                        }
                    };
                    value = match value.checked_add((byte - b'0') as u16) {
                        Some(value) => value,
                        None => {
                            return Err(ManaCostParseError::new(
                                symbol_start,
                                ManaCostParseErrorKind::Overflow,
                            ));
                        }
                    };
                    digit += 1;
                }
                cost.generic = value;
                saw_generic = true;
            }

            offset = symbol_end + 1;
        }

        Ok(cost)
    }

    const fn parse_flexible_symbol(
        bytes: &[u8],
        start: usize,
        end: usize,
    ) -> Option<FlexibleManaSymbol> {
        let len = end - start;
        if len == 3 && bytes[start + 1] == b'/' {
            let first = bytes[start];
            let second = bytes[start + 2];
            if let Some(pair) = HybridPair::from_letters(first, second) {
                return Some(FlexibleManaSymbol::from_hybrid_pair(pair));
            }
            if first == b'2'
                && let Some(color) = ManaColor::from_letter(second)
            {
                return FlexibleManaSymbol::two_brid(color);
            }
            if second == b'P'
                && let Some(color) = ManaColor::from_letter(first)
            {
                return FlexibleManaSymbol::phyrexian(color);
            }
            if first == b'C'
                && let Some(color) = ManaColor::from_letter(second)
            {
                return FlexibleManaSymbol::colorless_hybrid(color);
            }
            if second == b'C'
                && let Some(color) = ManaColor::from_letter(first)
            {
                return FlexibleManaSymbol::colorless_hybrid(color);
            }
            return None;
        }
        if len == 5
            && bytes[start + 1] == b'/'
            && bytes[start + 3] == b'/'
            && bytes[start + 4] == b'P'
            && let Some(pair) = HybridPair::from_letters(bytes[start], bytes[start + 2])
        {
            return Some(FlexibleManaSymbol::phyrexian_hybrid(pair));
        }
        None
    }

    const fn checked_increment(value: u16) -> Result<u16, ManaCostParseErrorKind> {
        match value.checked_add(1) {
            Some(value) => Ok(value),
            None => Err(ManaCostParseErrorKind::Overflow),
        }
    }

    /// Mana value with each `{X}` treated as zero.
    #[must_use]
    pub const fn mana_value(self) -> u16 {
        let mut value = self
            .generic
            .saturating_add(self.white)
            .saturating_add(self.blue)
            .saturating_add(self.black)
            .saturating_add(self.red)
            .saturating_add(self.green)
            .saturating_add(self.colorless);
        let mut index = 0;
        while index < FlexibleManaSymbol::COUNT {
            let symbol = FlexibleManaSymbol::ALL[index];
            value = value.saturating_add(
                self.flexible_count(symbol)
                    .saturating_mul(symbol.mana_value()),
            );
            index += 1;
        }
        value
    }

    #[must_use]
    pub const fn new(generic: u16, red: u16) -> Self {
        Self {
            generic,
            white: 0,
            blue: 0,
            black: 0,
            red,
            green: 0,
            colorless: 0,
            hybrid: [0; HybridPair::COUNT],
            additional_flexible: [0; FlexibleManaSymbol::ADDITIONAL_COUNT],
            variable_x: false,
            x_multiplier: 0,
        }
    }

    #[must_use]
    pub const fn colored(
        generic: u16,
        white: u16,
        blue: u16,
        black: u16,
        red: u16,
        green: u16,
    ) -> Self {
        Self {
            generic,
            white,
            blue,
            black,
            red,
            green,
            colorless: 0,
            hybrid: [0; HybridPair::COUNT],
            additional_flexible: [0; FlexibleManaSymbol::ADDITIONAL_COUNT],
            variable_x: false,
            x_multiplier: 0,
        }
    }

    /// `amount` mana of one colour and nothing else, for a payment whose
    /// size is counted at resolution rather than printed.
    #[must_use]
    pub const fn of_color(color: ManaColor, amount: u16) -> Self {
        match color {
            ManaColor::White => Self::colored(0, amount, 0, 0, 0, 0),
            ManaColor::Blue => Self::colored(0, 0, amount, 0, 0, 0),
            ManaColor::Black => Self::colored(0, 0, 0, amount, 0, 0),
            ManaColor::Red => Self::colored(0, 0, 0, 0, amount, 0),
            ManaColor::Green => Self::colored(0, 0, 0, 0, 0, amount),
            ManaColor::Colorless => {
                let mut cost = Self::new(0, 0);
                cost.colorless = amount;
                cost
            }
        }
    }

    #[must_use]
    pub const fn with_x(red: u16) -> Self {
        Self {
            generic: 0,
            white: 0,
            blue: 0,
            black: 0,
            red,
            green: 0,
            colorless: 0,
            hybrid: [0; HybridPair::COUNT],
            additional_flexible: [0; FlexibleManaSymbol::ADDITIONAL_COUNT],
            variable_x: true,
            x_multiplier: 1,
        }
    }

    /// The same cost with every coloured requirement turned generic, for a
    /// payment allowed to spend mana as though it were mana of any colour.
    ///
    /// Hybrid symbols come along, since a symbol any colour pays is already
    /// satisfied by a permission that makes every colour interchangeable.
    /// `{C}` does not: colourless is not a colour, so a permission that
    /// speaks about colours leaves those symbols exactly where they were.
    #[must_use]
    pub const fn as_any_color(self) -> Self {
        let mut generic = self.generic;
        generic = generic
            .saturating_add(self.white)
            .saturating_add(self.blue)
            .saturating_add(self.black)
            .saturating_add(self.red)
            .saturating_add(self.green);
        let mut symbol = 0;
        while symbol < FlexibleManaSymbol::COUNT {
            generic = generic.saturating_add(self.flexible_count(FlexibleManaSymbol::ALL[symbol]));
            symbol += 1;
        }
        Self {
            generic,
            white: 0,
            blue: 0,
            black: 0,
            red: 0,
            green: 0,
            hybrid: [0; HybridPair::COUNT],
            additional_flexible: [0; FlexibleManaSymbol::ADDITIONAL_COUNT],
            ..self
        }
    }

    #[must_use]
    pub const fn colored_x(white: u16, blue: u16, black: u16, red: u16, green: u16) -> Self {
        Self {
            generic: 0,
            white,
            blue,
            black,
            red,
            green,
            colorless: 0,
            hybrid: [0; HybridPair::COUNT],
            additional_flexible: [0; FlexibleManaSymbol::ADDITIONAL_COUNT],
            variable_x: true,
            x_multiplier: 1,
        }
    }

    #[must_use]
    pub const fn variable(
        generic: u16,
        white: u16,
        blue: u16,
        black: u16,
        red: u16,
        green: u16,
        x_multiplier: u16,
    ) -> Self {
        Self {
            generic,
            white,
            blue,
            black,
            red,
            green,
            colorless: 0,
            hybrid: [0; HybridPair::COUNT],
            additional_flexible: [0; FlexibleManaSymbol::ADDITIONAL_COUNT],
            variable_x: true,
            x_multiplier,
        }
    }

    /// How many flexible mana symbols this cost carries in total.
    #[must_use]
    pub const fn hybrid_total(&self) -> u16 {
        let mut total: u16 = 0;
        let mut index = 0;
        while index < FlexibleManaSymbol::COUNT {
            total = total.saturating_add(self.flexible_count(FlexibleManaSymbol::ALL[index]));
            index += 1;
        }
        total
    }

    /// How many copies of one flexible symbol this cost carries.
    #[must_use]
    pub const fn flexible_count(&self, symbol: FlexibleManaSymbol) -> u16 {
        match symbol.hybrid_pair() {
            Some(pair) => self.hybrid[pair.index()],
            None => match symbol.additional_index() {
                Some(index) => self.additional_flexible[index],
                None => 0,
            },
        }
    }

    /// Sets one flexible symbol's multiplicity.
    #[must_use]
    pub const fn with_flexible_symbol(mut self, symbol: FlexibleManaSymbol, count: u16) -> Self {
        match symbol.hybrid_pair() {
            Some(pair) => self.hybrid[pair.index()] = count,
            None => {
                if let Some(index) = symbol.additional_index() {
                    self.additional_flexible[index] = count;
                }
            }
        }
        self
    }

    /// Removes `count` copies of one symbol, rejecting a forged overpayment.
    #[must_use]
    pub const fn without_flexible(self, symbol: FlexibleManaSymbol, count: u16) -> Option<Self> {
        let present = self.flexible_count(symbol);
        if count > present {
            None
        } else {
            Some(self.with_flexible_symbol(symbol, present - count))
        }
    }

    #[must_use]
    pub const fn hybrid_pair(pair: HybridPair, count: u16) -> Self {
        Self {
            generic: 0,
            white: 0,
            blue: 0,
            black: 0,
            red: 0,
            green: 0,
            colorless: 0,
            hybrid: {
                let mut hybrid = [0; HybridPair::COUNT];
                hybrid[pair.index()] = count;
                hybrid
            },
            additional_flexible: [0; FlexibleManaSymbol::ADDITIONAL_COUNT],
            variable_x: false,
            x_multiplier: 0,
        }
    }
}

impl fmt::Display for ManaCost {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut wrote_symbol = false;
        if self.generic > 0 {
            write!(formatter, "{{{}}}", self.generic)?;
            wrote_symbol = true;
        }
        if self.variable_x {
            for _ in 0..self.x_multiplier.max(1) {
                formatter.write_str("{X}")?;
                wrote_symbol = true;
            }
        }
        for (amount, symbol) in [
            (self.white, "W"),
            (self.blue, "U"),
            (self.black, "B"),
            (self.red, "R"),
            (self.green, "G"),
            (self.colorless, "C"),
        ] {
            for _ in 0..amount {
                write!(formatter, "{{{symbol}}}")?;
                wrote_symbol = true;
            }
        }
        for symbol in FlexibleManaSymbol::ALL {
            for _ in 0..self.flexible_count(symbol) {
                write!(formatter, "{{{}}}", symbol.symbol())?;
                wrote_symbol = true;
            }
        }
        if !wrote_symbol {
            formatter.write_str("{0}")?;
        }
        Ok(())
    }
}

impl FromStr for ManaCost {
    type Err = ManaCostParseError;

    fn from_str(symbols: &str) -> Result<Self, Self::Err> {
        Self::parse_symbols(symbols)
    }
}
