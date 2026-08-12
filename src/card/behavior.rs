#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CardBehavior {
    AugurOfBolas,
    Balance,
    BloodBaronOfVizkopa,
    ChainLightning,
    Channel,
    DustToDust,
    Duress,
    EssenceScatter,
    Fireball,
    Fork,
    GoblinGrenade,
    GrislySalvage,
    IronclawOrcs,
    KirdApe,
    LifebaneZombie,
    LibraryOfAlexandria,
    Moat,
    Mulch,
    Negate,
    PillarOfFlame,
    Recall,
    SedgeTroll,
    SinCollector,
    SylvanLibrary,
    FellwarStone,
    Smoke,
    SphinxsRevelation,
    /// Tetravus's first upkeep trigger: trade +1/+1 counters for Tetravites.
    TetravusDetach,
    /// Tetravus's second upkeep trigger: exile its own Tetravites to take the
    /// counters back.
    TetravusAssemble,
    WinterOrb,
    // Compatibility rules keys retained while CardDefinition::new still
    // accepts CardBehavior instead of CardRules directly.
    Mountain,
    Plains,
    Unsupported,
}

use super::{
    CardRules, CardSupertype, CardTypeSet, ColorSet, CreatureStats, KeywordAbility, ManaColor,
    ManaCost,
};
use crate::card::sets;

impl CardBehavior {
    /// Returns all declarative rules metadata for this card behavior.
    #[must_use]
    pub const fn rules(self) -> &'static CardRules {
        sets::rules(self)
    }

    #[must_use]
    pub const fn is_legendary(self) -> bool {
        self.rules().has_supertype(CardSupertype::Legendary)
    }

    #[must_use]
    pub fn rules_text(self) -> std::borrow::Cow<'static, str> {
        self.rules().rules_text()
    }

    #[must_use]
    pub const fn types(self) -> CardTypeSet {
        self.rules().types()
    }

    #[must_use]
    pub const fn mana_cost(self) -> Option<ManaCost> {
        self.rules().mana_cost()
    }

    #[must_use]
    pub const fn creature_stats(self) -> Option<CreatureStats> {
        self.rules().creature_stats()
    }

    #[must_use]
    pub fn is_goblin(self) -> bool {
        self.rules().has_subtype("Goblin")
    }

    #[must_use]
    pub fn has_flying(self) -> bool {
        self.rules().has_executable_keyword(KeywordAbility::Flying)
    }

    #[must_use]
    pub fn has_mountainwalk(self) -> bool {
        self.rules()
            .has_executable_keyword(KeywordAbility::Mountainwalk)
    }

    /// Returns the object's printed color-characteristic set.
    #[must_use]
    pub const fn colors(self) -> ColorSet {
        self.rules().color_set()
    }

    #[must_use]
    pub const fn is_red(self) -> bool {
        self.rules().has_color(ManaColor::Red)
    }

    #[must_use]
    pub const fn is_blue(self) -> bool {
        self.rules().has_color(ManaColor::Blue)
    }

    #[must_use]
    pub const fn is_white(self) -> bool {
        self.rules().has_color(ManaColor::White)
    }

    #[must_use]
    pub const fn is_black(self) -> bool {
        self.rules().has_color(ManaColor::Black)
    }

    #[must_use]
    pub const fn is_green(self) -> bool {
        self.rules().has_color(ManaColor::Green)
    }

    #[must_use]
    pub fn has_vigilance(self) -> bool {
        self.rules()
            .has_executable_keyword(KeywordAbility::Vigilance)
    }
}
