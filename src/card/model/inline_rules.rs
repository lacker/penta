use std::hash::{Hash, Hasher};

use super::{
    AbilityDef, BasicLandType, CardRules, CardSupertype, CardType, CardTypeSet, ColorSet,
    CreatureStats, ManaColor, ObjectPredicateDef,
};

/// Compact rules shared by inline virtual-object and face-down values.
///
/// Abilities stay behind a slice because an ability can itself create another
/// inline value. Storing an [`AbilityDef`] here would make the declarative
/// effect schema recursively sized.
#[derive(Clone, Copy, Debug)]
pub(super) struct InlineRules {
    card_types: CardTypeSet,
    supertypes: u8,
    subtypes: &'static [&'static str],
    /// Five color flags followed by two base-5 word-substitution maps. Each
    /// five-word map fits in 12 bits, keeping virtual values inline.
    colors_and_words: u32,
    creature_stats: Option<CreatureStats>,
    abilities: &'static [AbilityDef],
    /// "Enchant creature" on an Aura that was never cast. A Role token is
    /// created already attached, so its restriction has to be printed on it
    /// rather than read off a spell's target. Held by reference to keep
    /// these values inside their inline size budget.
    enchant: Option<&'static ObjectPredicateDef>,
}

impl PartialEq for InlineRules {
    fn eq(&self, other: &Self) -> bool {
        self.card_types == other.card_types
            && self.supertypes == other.supertypes
            && self.subtypes == other.subtypes
            && self.colors_and_words == other.colors_and_words
            && self.creature_stats == other.creature_stats
            && self.enchant == other.enchant
            && std::ptr::eq(self.abilities, other.abilities)
    }
}

impl Eq for InlineRules {}

impl Hash for InlineRules {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.card_types.hash(state);
        self.supertypes.hash(state);
        self.subtypes.hash(state);
        self.colors_and_words.hash(state);
        self.creature_stats.hash(state);
        self.enchant.hash(state);
        self.abilities.as_ptr().hash(state);
        self.abilities.len().hash(state);
    }
}

impl InlineRules {
    const COLOR_MASK: u32 = 0b1_1111;
    const LAND_WORD_SHIFT: u32 = 5;
    const COLOR_WORD_SHIFT: u32 = 17;
    const WORD_MAP_MASK: u32 = 0xfff;

    pub(super) const fn new(
        card_types: CardTypeSet,
        subtypes: &'static [&'static str],
        colors: &'static [ManaColor],
        creature_stats: Option<CreatureStats>,
    ) -> Self {
        Self {
            card_types,
            supertypes: 0,
            subtypes,
            colors_and_words: Self::color_bits(ColorSet::from_colors(colors))
                | (Self::pack_basic_land_type_words(BasicLandType::ALL) << Self::LAND_WORD_SHIFT)
                | (Self::pack_color_words(ManaColor::COLORS) << Self::COLOR_WORD_SHIFT),
            creature_stats,
            abilities: &[],
            enchant: None,
        }
    }

    pub(super) const fn with_type(mut self, card_type: CardType) -> Self {
        self.card_types = self.card_types.with(card_type);
        self
    }

    pub(super) const fn with_supertype(mut self, supertype: CardSupertype) -> Self {
        self.supertypes |= 1 << supertype.index();
        self
    }

    const fn supertypes(self) -> [bool; CardSupertype::COUNT] {
        let mut supertypes = [false; CardSupertype::COUNT];
        let mut index = 0;
        while index < supertypes.len() {
            supertypes[index] = self.supertypes & (1 << index) != 0;
            index += 1;
        }
        supertypes
    }

    pub(super) const fn with_abilities(mut self, abilities: &'static [AbilityDef]) -> Self {
        self.abilities = abilities;
        self
    }

    /// Replaces the printed power and toughness. "An X/X blue Illusion" has
    /// no printed size at all: the effect that creates it works one out, and
    /// what arrives is a token of that size.
    pub(super) const fn with_creature_stats(mut self, stats: CreatureStats) -> Self {
        self.creature_stats = Some(stats);
        self
    }

    pub(super) const fn with_color_set(mut self, colors: ColorSet) -> Self {
        self.colors_and_words =
            (self.colors_and_words & !Self::COLOR_MASK) | Self::color_bits(colors);
        self
    }

    pub(super) const fn basic_land_type_word(self, word: BasicLandType) -> BasicLandType {
        let packed = (self.colors_and_words >> Self::LAND_WORD_SHIFT) & Self::WORD_MAP_MASK;
        BasicLandType::ALL[Self::word_digit(packed, word.index())]
    }

    pub(super) const fn color_word(self, word: ManaColor) -> ManaColor {
        let Some(index) = word.color_index() else {
            return word;
        };
        let packed = (self.colors_and_words >> Self::COLOR_WORD_SHIFT) & Self::WORD_MAP_MASK;
        ManaColor::COLORS[Self::word_digit(packed, index)]
    }

    pub(super) const fn basic_land_type_word_map(
        self,
    ) -> [BasicLandType; BasicLandType::ALL.len()] {
        let mut words = BasicLandType::ALL;
        let mut index = 0;
        while index < words.len() {
            words[index] = self.basic_land_type_word(BasicLandType::ALL[index]);
            index += 1;
        }
        words
    }

    pub(super) const fn color_word_map(self) -> [ManaColor; ManaColor::COLORS.len()] {
        let mut words = ManaColor::COLORS;
        let mut index = 0;
        while index < words.len() {
            words[index] = self.color_word(ManaColor::COLORS[index]);
            index += 1;
        }
        words
    }

    pub(super) const fn with_word_maps(
        mut self,
        basic_land_type_words: [BasicLandType; BasicLandType::ALL.len()],
        color_words: [ManaColor; ManaColor::COLORS.len()],
    ) -> Self {
        let colors = self.colors_and_words & Self::COLOR_MASK;
        self.colors_and_words = colors
            | (Self::pack_basic_land_type_words(basic_land_type_words) << Self::LAND_WORD_SHIFT)
            | (Self::pack_color_words(color_words) << Self::COLOR_WORD_SHIFT);
        self
    }

    const fn color_bits(colors: ColorSet) -> u32 {
        let mut bits = 0;
        let mut index = 0;
        while index < ManaColor::COLORS.len() {
            if colors.contains(ManaColor::COLORS[index]) {
                bits |= 1 << index;
            }
            index += 1;
        }
        bits
    }

    const fn color_set(self) -> ColorSet {
        let mut colors = ColorSet::empty();
        let mut index = 0;
        while index < ManaColor::COLORS.len() {
            if self.colors_and_words & (1 << index) != 0 {
                colors = colors.with(ManaColor::COLORS[index]);
            }
            index += 1;
        }
        colors
    }

    const fn word_digit(mut packed: u32, index: usize) -> usize {
        let mut position = 0;
        while position < index {
            packed /= 5;
            position += 1;
        }
        match packed % 5 {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => 4,
            _ => unreachable!(),
        }
    }

    const fn pack_basic_land_type_words(words: [BasicLandType; BasicLandType::ALL.len()]) -> u32 {
        let mut packed = 0;
        let mut multiplier = 1;
        let mut index = 0;
        while index < words.len() {
            packed += Self::word_index(words[index].index()) * multiplier;
            multiplier *= 5;
            index += 1;
        }
        packed
    }

    const fn pack_color_words(words: [ManaColor; ManaColor::COLORS.len()]) -> u32 {
        let mut packed = 0;
        let mut multiplier = 1;
        let mut index = 0;
        while index < words.len() {
            let Some(word) = words[index].color_index() else {
                panic!("colorless is not a color word");
            };
            packed += Self::word_index(word) * multiplier;
            multiplier *= 5;
            index += 1;
        }
        packed
    }

    const fn word_index(index: usize) -> u32 {
        match index {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => 4,
            _ => panic!("a text word must be one of five values"),
        }
    }

    pub(super) const fn with_enchant(mut self, object: &'static ObjectPredicateDef) -> Self {
        self.enchant = Some(object);
        self
    }

    pub(super) const fn subtypes(self) -> &'static [&'static str] {
        self.subtypes
    }

    pub(super) const fn materialize(self) -> CardRules {
        let rules = CardRules::from_inline_characteristics(
            self.card_types,
            self.supertypes(),
            self.subtypes,
            self.color_set(),
            self.creature_stats,
            self.abilities,
        );
        match self.enchant {
            Some(object) => rules.enchanting(*object),
            None => rules,
        }
    }
}
