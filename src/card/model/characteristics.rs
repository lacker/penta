use std::fmt;

/// One of the five colors of Magic, or colorless mana.
///
/// The same vocabulary is used by card characteristics, mana-producing
/// effects, and the runtime mana pool. `Colorless` is a mana type rather than
/// a color, so it has no index in a card's five-color characteristic set.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ManaColor {
    White,
    Blue,
    Black,
    Red,
    Green,
    Colorless,
}

impl ManaColor {
    /// Human-facing name used by public color choices.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::White => "White",
            Self::Blue => "Blue",
            Self::Black => "Black",
            Self::Red => "Red",
            Self::Green => "Green",
            Self::Colorless => "Colorless",
        }
    }

    #[must_use]
    pub fn from_label(label: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|color| color.label().eq_ignore_ascii_case(label))
    }

    /// The single letter Magic prints for this colour.
    #[must_use]
    pub const fn from_letter(letter: u8) -> Option<Self> {
        match letter {
            b'W' => Some(Self::White),
            b'U' => Some(Self::Blue),
            b'B' => Some(Self::Black),
            b'R' => Some(Self::Red),
            b'G' => Some(Self::Green),
            _ => None,
        }
    }

    pub const COLORS: [Self; 5] = [Self::White, Self::Blue, Self::Black, Self::Red, Self::Green];

    pub const ALL: [Self; 6] = [
        Self::White,
        Self::Blue,
        Self::Black,
        Self::Red,
        Self::Green,
        Self::Colorless,
    ];

    /// This type's position in [`ManaColor::ALL`]. Unlike
    /// [`Self::color_index`], colorless has one: it is a mana type, and a
    /// mana pool holds it like any other.
    #[must_use]
    pub const fn index(self) -> usize {
        match self {
            Self::White => 0,
            Self::Blue => 1,
            Self::Black => 2,
            Self::Red => 3,
            Self::Green => 4,
            Self::Colorless => 5,
        }
    }

    #[must_use]
    pub const fn color_index(self) -> Option<usize> {
        match self {
            Self::White => Some(0),
            Self::Blue => Some(1),
            Self::Black => Some(2),
            Self::Red => Some(3),
            Self::Green => Some(4),
            Self::Colorless => None,
        }
    }
}

/// How many mana of each type one activation produces, for the abilities
/// whose printed amount is split across types rather than fixed to one --
/// "add three mana in any combination of {U} and/or {R}".
///
/// Counts are indexed by [`ManaColor::ALL`], so the split is a plain value:
/// a mana ability resolves without ever holding priority, so the way the
/// amount is divided is enumerated into the activation rather than asked
/// afterwards, exactly as a counter size or a sacrificed permanent already
/// is.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ManaSplit([u16; ManaColor::ALL.len()]);

impl ManaSplit {
    #[must_use]
    pub const fn empty() -> Self {
        Self([0; ManaColor::ALL.len()])
    }

    #[must_use]
    pub const fn get(self, color: ManaColor) -> u16 {
        self.0[color.index()]
    }

    pub const fn add(&mut self, color: ManaColor, amount: u16) {
        self.0[color.index()] += amount;
    }

    pub const fn remove(&mut self, color: ManaColor, amount: u16) {
        self.0[color.index()] -= amount;
    }

    #[must_use]
    pub const fn total(self) -> u16 {
        let mut total = 0;
        let mut index = 0;
        while index < self.0.len() {
            total += self.0[index];
            index += 1;
        }
        total
    }

    /// The types this split actually produces, in [`ManaColor::ALL`] order.
    /// A type it produces none of is left out, so a split of three red mana
    /// reads as one entry rather than six.
    pub fn iter(self) -> impl Iterator<Item = (ManaColor, u16)> {
        ManaColor::ALL
            .into_iter()
            .filter(move |color| self.get(*color) > 0)
            .map(move |color| (color, self.get(color)))
    }
}

/// The colors an object has as a characteristic.
///
/// Colorless is represented by the empty set, not by a sixth flag. The
/// protocol-facing [`CardRules::colors`] method continues to project this as
/// `[white, blue, black, red, green]` for compatibility.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct ColorSet(u8);

impl ColorSet {
    #[must_use]
    pub const fn empty() -> Self {
        Self(0)
    }

    #[must_use]
    /// # Panics
    ///
    /// Panics if `colors` contains [`ManaColor::Colorless`], which is a mana
    /// type rather than a color characteristic.
    pub const fn from_colors(colors: &[ManaColor]) -> Self {
        let mut result = Self::empty();
        let mut index = 0;
        while index < colors.len() {
            result = result.with(colors[index]);
            index += 1;
        }
        result
    }

    #[must_use]
    /// # Panics
    ///
    /// Panics if `color` is [`ManaColor::Colorless`]. A colorless object is
    /// represented by an empty set.
    pub const fn with(mut self, color: ManaColor) -> Self {
        let Some(index) = color.color_index() else {
            panic!("colorless is not a color characteristic");
        };
        self.0 |= 1 << index;
        self
    }

    #[must_use]
    pub const fn contains(self, color: ManaColor) -> bool {
        let Some(index) = color.color_index() else {
            return false;
        };
        self.0 & (1 << index) != 0
    }

    #[must_use]
    pub const fn is_colorless(self) -> bool {
        self.0 == 0
    }

    #[must_use]
    pub const fn to_flags(self) -> [bool; 5] {
        [
            self.contains(ManaColor::White),
            self.contains(ManaColor::Blue),
            self.contains(ManaColor::Black),
            self.contains(ManaColor::Red),
            self.contains(ManaColor::Green),
        ]
    }
}

/// One atomic card type. A card's type line is represented by a
/// [`CardTypeSet`], so combinations such as artifact creatures, enchantment
/// creatures, artifact lands, and land creatures do not require bespoke enum
/// variants.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CardType {
    Artifact,
    Creature,
    Enchantment,
    Instant,
    /// CR 205.2a. A card type that does nothing by itself: what it is for is
    /// letting a noncreature card carry creature subtypes, so a Kindred
    /// Instant -- Goblin is a Goblin card wherever it is, and everything
    /// that counts Goblins counts it.
    Kindred,
    Land,
    Planeswalker,
    Sorcery,
}

impl CardType {
    pub const COUNT: usize = 8;
    pub const ALL: [Self; Self::COUNT] = [
        Self::Artifact,
        Self::Creature,
        Self::Enchantment,
        Self::Instant,
        Self::Kindred,
        Self::Land,
        Self::Planeswalker,
        Self::Sorcery,
    ];

    /// Conventional type-line order for the combinations the catalog can
    /// currently express. This is deliberately independent of bit indexes.
    pub const DISPLAY_ORDER: [Self; Self::COUNT] = [
        // Kindred leads the line it appears on, which is where the type
        // line prints it: "Kindred Instant -- Goblin".
        Self::Kindred,
        Self::Artifact,
        Self::Enchantment,
        Self::Land,
        Self::Creature,
        Self::Planeswalker,
        Self::Instant,
        Self::Sorcery,
    ];

    #[must_use]
    pub const fn index(self) -> usize {
        match self {
            Self::Artifact => 0,
            Self::Creature => 1,
            Self::Enchantment => 2,
            Self::Instant => 3,
            Self::Land => 4,
            Self::Planeswalker => 5,
            Self::Sorcery => 6,
            Self::Kindred => 7,
        }
    }

    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Artifact => "Artifact",
            Self::Creature => "Creature",
            Self::Enchantment => "Enchantment",
            Self::Instant => "Instant",
            Self::Land => "Land",
            Self::Planeswalker => "Planeswalker",
            Self::Sorcery => "Sorcery",
            Self::Kindred => "Kindred",
        }
    }
}

/// A const-friendly set of card types stored on one card part.
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
pub struct CardTypeSet(u32);

impl CardTypeSet {
    pub const EMPTY: Self = Self(0);

    #[must_use]
    pub const fn empty() -> Self {
        Self::EMPTY
    }

    #[must_use]
    pub const fn single(card_type: CardType) -> Self {
        Self(1 << card_type.index())
    }

    #[must_use]
    pub const fn with(mut self, card_type: CardType) -> Self {
        self.0 |= 1 << card_type.index();
        self
    }

    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    #[must_use]
    pub const fn intersects(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }

    #[must_use]
    pub const fn contains(self, card_type: CardType) -> bool {
        self.0 & (1 << card_type.index()) != 0
    }

    /// How many distinct types are in the set. "For each card type among
    /// them" counts this rather than the cards. There are only eight of
    /// them, so the width is never in question.
    #[must_use]
    pub const fn count(self) -> u16 {
        // Eight types fit in a byte, so the count fits anywhere.
        #[allow(clippy::cast_possible_truncation)]
        {
            self.0.count_ones() as u16
        }
    }

    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[must_use]
    pub const fn is_creature(self) -> bool {
        self.contains(CardType::Creature)
    }

    #[must_use]
    pub const fn is_artifact(self) -> bool {
        self.contains(CardType::Artifact)
    }

    #[must_use]
    pub const fn is_permanent(self) -> bool {
        self.contains(CardType::Artifact)
            || self.contains(CardType::Creature)
            || self.contains(CardType::Enchantment)
            || self.contains(CardType::Land)
            || self.contains(CardType::Planeswalker)
    }

    /// Compatibility spelling used by the existing protocol `kind` field.
    ///
    /// Current single-type cards retain names such as `Instant`; an artifact
    /// creature retains `ArtifactCreature`. New combinations are represented
    /// by concatenating their card types in rules-defined type-line order.
    #[must_use]
    pub fn kind_name(self) -> String {
        CardType::DISPLAY_ORDER
            .into_iter()
            .filter(|card_type| self.contains(*card_type))
            .map(CardType::name)
            .collect()
    }

    #[must_use]
    pub fn type_name(self) -> String {
        CardType::DISPLAY_ORDER
            .into_iter()
            .filter(|card_type| self.contains(*card_type))
            .map(CardType::name)
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl fmt::Debug for CardTypeSet {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_set()
            .entries(
                CardType::DISPLAY_ORDER
                    .into_iter()
                    .filter(|card_type| self.contains(*card_type)),
            )
            .finish()
    }
}
