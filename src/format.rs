//! Tournament-format rules that vary independently of individual cards.

use std::fmt;

use crate::card::{CardDefinition, CardSet};

pub mod vintage_cube;

/// A supported constructed format.
///
/// Keep this value in a [`crate::Game`] rather than consulting global rules:
/// two games using different formats must be able to run in the same process.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Format {
    /// Eternal Central Old School 93/94.
    #[default]
    OldSchool9394,
    /// The final pre-Theros Standard pool, from Innistrad through Magic 2014.
    IsdDgmStandard,
    /// Premodern, the community format spanning Fourth Edition through
    /// Scourge.
    Premodern,
    /// The MTGO Vintage Cube, played from a fixed singleton list rather than
    /// from a set window.
    VintageCube,
}

/// The construction and game-start values shared by every game in a format.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FormatRules {
    pub starting_life: u8,
    pub opening_hand_size: usize,
    pub minimum_main_deck_size: usize,
    pub maximum_sideboard_size: usize,
    pub maximum_copies: usize,
    pub allowed_sets: &'static [CardSet],
    /// Modern rules empty mana pools after every step and phase. EC 93/94
    /// instead keeps mana through steps and empties pools only between phases.
    pub mana_empties_at_end_of_step: bool,
    pub mana_burn: bool,
    pub banned_cards: &'static [&'static str],
    pub restricted_cards: &'static [&'static str],
    /// A fixed card list this format is played from, when it has one. A cube
    /// is not a set window: legality is membership in the list, so a format
    /// carrying one ignores `allowed_sets` entirely.
    pub card_pool: Option<&'static [&'static str]>,
}

pub const OLD_SCHOOL_BANNED_CARDS: &[&str] = &[
    "Bronze Tablet",
    "Contract from Below",
    "Darkpact",
    "Demonic Attorney",
    "Jeweled Bird",
    "Rebirth",
    "Tempest Efreet",
];

pub const OLD_SCHOOL_ALLOWED_SETS: &[CardSet] = &[
    CardSet::Alpha,
    CardSet::Beta,
    CardSet::Unlimited,
    CardSet::CollectorsEdition,
    CardSet::InternationalCollectorsEdition,
    CardSet::ArabianNights,
    CardSet::Antiquities,
    CardSet::Revised,
    CardSet::Legends,
    CardSet::TheDark,
    CardSet::FallenEmpires,
    CardSet::Promo1994,
];

pub const OLD_SCHOOL_RESTRICTED_CARDS: &[&str] = &[
    "Ancestral Recall",
    "Balance",
    "Black Lotus",
    "Braingeyser",
    "Chaos Orb",
    "Channel",
    "Demonic Tutor",
    "Library of Alexandria",
    "Mana Drain",
    "Mind Twist",
    "Mox Emerald",
    "Mox Jet",
    "Mox Pearl",
    "Mox Ruby",
    "Mox Sapphire",
    "Recall",
    "Regrowth",
    "Sol Ring",
    "Time Vault",
    "Time Walk",
    "Timetwister",
    "Wheel of Fortune",
];

pub const ISD_DGM_STANDARD_BANNED_CARDS: &[&str] = &[];
pub const ISD_DGM_STANDARD_RESTRICTED_CARDS: &[&str] = &[];
pub const ISD_DGM_STANDARD_ALLOWED_SETS: &[CardSet] = &[
    CardSet::Innistrad,
    CardSet::DarkAscension,
    CardSet::AvacynRestored,
    CardSet::Magic2013,
    CardSet::ReturnToRavnica,
    CardSet::Gatecrash,
    CardSet::DragonsMaze,
    CardSet::Magic2014,
];

/// The twenty-nine sets Premodern names, in release order. Portal and promo
/// printings are excluded by the format rather than by the card: a card whose
/// only modeled printing is a Portal one is not Premodern legal, which is why
/// the window is stated as sets rather than as dates.
pub const PREMODERN_ALLOWED_SETS: &[CardSet] = &[
    CardSet::FourthEdition,
    CardSet::IceAge,
    CardSet::Chronicles,
    CardSet::Homelands,
    CardSet::Alliances,
    CardSet::Mirage,
    CardSet::Visions,
    CardSet::FifthEdition,
    CardSet::Weatherlight,
    CardSet::Tempest,
    CardSet::Stronghold,
    CardSet::Exodus,
    CardSet::UrzasSaga,
    CardSet::UrzasLegacy,
    CardSet::ClassicSixthEdition,
    CardSet::UrzasDestiny,
    CardSet::MercadianMasques,
    CardSet::Nemesis,
    CardSet::Prophecy,
    CardSet::Invasion,
    CardSet::Planeshift,
    CardSet::SeventhEdition,
    CardSet::Apocalypse,
    CardSet::Odyssey,
    CardSet::Torment,
    CardSet::Judgment,
    CardSet::Onslaught,
    CardSet::Legions,
    CardSet::Scourge,
];

/// Premodern's own maintained ban list, which is not derived from any other
/// format's: several of these are banned to leave room for weaker cards
/// rather than because they would be illegal elsewhere.
pub const PREMODERN_BANNED_CARDS: &[&str] = &[
    "Amulet of Quoz",
    "Balance",
    "Brainstorm",
    "Bronze Tablet",
    "Channel",
    "Demonic Consultation",
    "Earthcraft",
    "Entomb",
    "Flash",
    "Force of Will",
    "Goblin Recruiter",
    "Grim Monolith",
    "Jeweled Bird",
    "Land Tax",
    "Mana Vault",
    "Memory Jar",
    "Mind Twist",
    "Mind's Desire",
    "Mystical Tutor",
    "Necropotence",
    "Parallax Tide",
    "Rebirth",
    "Strip Mine",
    "Tempest Efreet",
    "Tendrils of Agony",
    "Time Spiral",
    "Timmerian Fiends",
    "Tolarian Academy",
    "Vampiric Tutor",
    "Windfall",
    "Worldgorger Dragon",
    "Yawgmoth's Bargain",
    "Yawgmoth's Will",
];

/// Premodern restricts nothing: the ban list is the whole of its card policy.
pub const PREMODERN_RESTRICTED_CARDS: &[&str] = &[];

const PREMODERN_RULES: FormatRules = FormatRules {
    starting_life: 20,
    opening_hand_size: 7,
    minimum_main_deck_size: 60,
    maximum_sideboard_size: 15,
    maximum_copies: 4,
    allowed_sets: PREMODERN_ALLOWED_SETS,
    // "Played with contemporary Magic: the Gathering in-game rules", so the
    // pool is old and the rules are not: no mana burn, and pools empty at
    // every step.
    mana_empties_at_end_of_step: true,
    mana_burn: false,
    banned_cards: PREMODERN_BANNED_CARDS,
    restricted_cards: PREMODERN_RESTRICTED_CARDS,
    card_pool: None,
};

/// Forty cards and one of each: a cube is drafted, and the list it is drafted
/// from is singleton. Nothing is banned or restricted -- a card is either in
/// the pool or it is not.
const VINTAGE_CUBE_RULES: FormatRules = FormatRules {
    starting_life: 20,
    opening_hand_size: 7,
    minimum_main_deck_size: 40,
    maximum_sideboard_size: 15,
    maximum_copies: 1,
    // Unused while a pool is present, and empty so that nothing reads it as
    // a set window by accident.
    allowed_sets: &[],
    mana_empties_at_end_of_step: true,
    mana_burn: false,
    banned_cards: &[],
    restricted_cards: &[],
    card_pool: Some(vintage_cube::VINTAGE_CUBE_POOL),
};

const OLD_SCHOOL_RULES: FormatRules = FormatRules {
    starting_life: 20,
    opening_hand_size: 7,
    minimum_main_deck_size: 60,
    maximum_sideboard_size: 15,
    maximum_copies: 4,
    allowed_sets: OLD_SCHOOL_ALLOWED_SETS,
    mana_empties_at_end_of_step: false,
    mana_burn: true,
    banned_cards: OLD_SCHOOL_BANNED_CARDS,
    restricted_cards: OLD_SCHOOL_RESTRICTED_CARDS,
    card_pool: None,
};

#[deprecated(note = "use ISD_DGM_STANDARD_BANNED_CARDS")]
pub const ISD_RTR_STANDARD_BANNED_CARDS: &[&str] = ISD_DGM_STANDARD_BANNED_CARDS;
#[deprecated(note = "use ISD_DGM_STANDARD_RESTRICTED_CARDS")]
pub const ISD_RTR_STANDARD_RESTRICTED_CARDS: &[&str] = ISD_DGM_STANDARD_RESTRICTED_CARDS;
#[deprecated(note = "use ISD_DGM_STANDARD_ALLOWED_SETS")]
pub const ISD_RTR_STANDARD_ALLOWED_SETS: &[CardSet] = ISD_DGM_STANDARD_ALLOWED_SETS;

const ISD_DGM_STANDARD_RULES: FormatRules = FormatRules {
    starting_life: 20,
    opening_hand_size: 7,
    minimum_main_deck_size: 60,
    maximum_sideboard_size: 15,
    maximum_copies: 4,
    allowed_sets: ISD_DGM_STANDARD_ALLOWED_SETS,
    mana_empties_at_end_of_step: true,
    mana_burn: false,
    banned_cards: ISD_DGM_STANDARD_BANNED_CARDS,
    restricted_cards: ISD_DGM_STANDARD_RESTRICTED_CARDS,
    card_pool: None,
};

impl Format {
    /// Source-compatible alias for the former format name.
    #[allow(non_upper_case_globals)]
    #[deprecated(note = "use Format::IsdDgmStandard")]
    pub const IsdRtrStandard: Self = Self::IsdDgmStandard;

    /// Every supported format, in the order they were added.
    pub const ALL: &'static [Self] = &[
        Self::OldSchool9394,
        Self::IsdDgmStandard,
        Self::Premodern,
        Self::VintageCube,
    ];

    #[must_use]
    pub const fn rules(self) -> &'static FormatRules {
        match self {
            Self::OldSchool9394 => &OLD_SCHOOL_RULES,
            Self::IsdDgmStandard => &ISD_DGM_STANDARD_RULES,
            Self::Premodern => &PREMODERN_RULES,
            Self::VintageCube => &VINTAGE_CUBE_RULES,
        }
    }

    #[must_use]
    pub const fn slug(self) -> &'static str {
        match self {
            Self::OldSchool9394 => "old-school-93-94",
            Self::IsdDgmStandard => "isd-dgm-standard",
            Self::Premodern => "premodern",
            Self::VintageCube => "vintage-cube",
        }
    }

    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::OldSchool9394 => "Old School 93/94",
            Self::IsdDgmStandard => "ISD-DGM Standard",
            Self::Premodern => "Premodern",
            Self::VintageCube => "Vintage Cube",
        }
    }

    /// Whether a printing represented by `set` is legal in this format.
    #[must_use]
    pub fn allows_set(self, set: CardSet) -> bool {
        self.rules().allowed_sets.contains(&set)
    }

    /// Whether this card identity can be used to construct a deck.
    ///
    /// Basic lands are shared by every supported format. In a format built on
    /// a fixed pool, every other card is legal exactly when the pool names it.
    /// Otherwise a card is legal when at least one known printing belongs to an
    /// allowed set; the physical printing selected for a deck does not change
    /// that identity's legality.
    #[must_use]
    pub fn allows_card(self, card: &CardDefinition) -> bool {
        if card.is_basic_land() {
            return true;
        }
        // A pool says which cards exist in the format outright, so where one
        // is given the set a card was printed in has nothing to do with it.
        if let Some(pool) = self.rules().card_pool {
            return contains_name(pool, &card.name);
        }
        card.printings
            .iter()
            .any(|printing| self.allows_set(printing.id.set))
    }

    #[must_use]
    pub fn is_banned(self, name: &str) -> bool {
        contains_name(self.rules().banned_cards, name)
    }

    #[must_use]
    pub fn is_restricted(self, name: &str) -> bool {
        contains_name(self.rules().restricted_cards, name)
    }
}

impl fmt::Display for Format {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.display_name())
    }
}

fn contains_name(names: &[&str], candidate: &str) -> bool {
    let candidate = candidate.trim();
    names
        .iter()
        .any(|name| name.eq_ignore_ascii_case(candidate))
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::{Format, VINTAGE_CUBE_RULES};
    use crate::CardDefinitionId;
    use crate::card::{CardBehavior, CardDefinition, CardPrinting, CardSet};

    /// The sets `format` is played from, or `None` when it is played from a
    /// fixed pool instead and has no set window to check.
    fn set_window(format: Format) -> Option<&'static [CardSet]> {
        let rules = format.rules();
        if rules.card_pool.is_some() {
            return None;
        }
        Some(rules.allowed_sets)
    }

    #[test]
    fn vintage_cube_is_a_singleton_pool_rather_than_a_set_window() {
        assert_eq!(VINTAGE_CUBE_RULES.maximum_copies, 1);
        assert_eq!(VINTAGE_CUBE_RULES.minimum_main_deck_size, 40);
        assert!(
            VINTAGE_CUBE_RULES.allowed_sets.is_empty(),
            "a pool format must not also claim a set window"
        );
        assert!(VINTAGE_CUBE_RULES.banned_cards.is_empty());
        assert!(VINTAGE_CUBE_RULES.restricted_cards.is_empty());
    }

    #[test]
    fn vintage_cube_pool_is_sorted_and_free_of_duplicates() {
        let pool = VINTAGE_CUBE_RULES
            .card_pool
            .expect("the cube is played from a pool");
        assert!(!pool.is_empty());
        for pair in pool.windows(2) {
            assert!(
                pair[0] < pair[1],
                "the pool is kept sorted for diffability: {} then {}",
                pair[0],
                pair[1]
            );
        }
    }

    #[test]
    fn pool_membership_decides_legality_regardless_of_printing() {
        // Two cards from the same set, one in the pool and one not: only
        // membership separates them.
        let inside = CardDefinition::new(
            CardDefinitionId::new(1),
            "Ancestral Recall",
            CardSet::Alpha,
            false,
            CardBehavior::Unsupported,
        );
        let outside = CardDefinition::new(
            CardDefinitionId::new(2),
            "Sorrow's Path",
            CardSet::Alpha,
            false,
            CardBehavior::Unsupported,
        );
        assert!(Format::VintageCube.allows_card(&inside));
        assert!(!Format::VintageCube.allows_card(&outside));
    }

    #[test]
    fn format_set_windows_are_nonempty_unique_and_exclude_tokens() {
        for &format in Format::ALL {
            let Some(sets) = set_window(format) else {
                continue;
            };
            assert!(!sets.is_empty(), "{format} needs an allowed set window");
            assert!(
                !sets.contains(&CardSet::Token),
                "tokens are never printable"
            );

            let unique = sets.iter().copied().collect::<HashSet<_>>();
            assert_eq!(
                unique.len(),
                sets.len(),
                "{format} repeats a set in its allowed window"
            );
        }
    }

    #[test]
    fn formats_allow_only_their_sets_but_share_basic_lands() {
        let old_spell = CardDefinition::new(
            CardDefinitionId::new(1),
            "Old spell",
            CardSet::Alpha,
            false,
            CardBehavior::Unsupported,
        );
        let standard_spell = CardDefinition::new(
            CardDefinitionId::new(2),
            "Standard spell",
            CardSet::Innistrad,
            false,
            CardBehavior::Unsupported,
        );
        let old_printing_of_a_basic = CardDefinition::new(
            CardDefinitionId::new(3),
            "Plains",
            CardSet::Alpha,
            true,
            CardBehavior::Plains,
        );

        assert!(Format::OldSchool9394.allows_card(&old_spell));
        assert!(!Format::OldSchool9394.allows_card(&standard_spell));
        assert!(Format::IsdDgmStandard.allows_card(&standard_spell));
        assert!(!Format::IsdDgmStandard.allows_card(&old_spell));
        assert!(Format::IsdDgmStandard.allows_card(&old_printing_of_a_basic));
        assert!(Format::OldSchool9394.allows_card(&old_printing_of_a_basic));
    }

    #[test]
    fn any_allowed_reprint_makes_the_canonical_card_identity_legal() {
        let id = CardDefinitionId::new(1);
        let mut reprinted_spell = CardDefinition::new(
            id,
            "Reprinted spell",
            CardSet::Alpha,
            false,
            CardBehavior::Unsupported,
        );
        reprinted_spell
            .printings
            .push(CardPrinting::new(id, CardSet::Magic2014));

        assert!(Format::OldSchool9394.allows_card(&reprinted_spell));
        assert!(Format::IsdDgmStandard.allows_card(&reprinted_spell));
    }

    /// Premodern is a window, not a date range: a card whose only printing
    /// is outside it stays illegal however old it is, which is what keeps
    /// Portal out.
    #[test]
    fn premodern_takes_the_window_from_fourth_edition_through_scourge() {
        let sets = Format::Premodern.rules().allowed_sets;
        assert_eq!(sets.len(), 29, "the format names twenty-nine sets");
        assert_eq!(sets.first(), Some(&CardSet::FourthEdition));
        assert_eq!(sets.last(), Some(&CardSet::Scourge));

        for inside in [
            CardSet::FourthEdition,
            CardSet::Alliances,
            CardSet::Tempest,
            CardSet::UrzasSaga,
            CardSet::Invasion,
            CardSet::Odyssey,
            CardSet::Scourge,
        ] {
            assert!(Format::Premodern.allows_set(inside), "{inside:?} is legal");
        }
        for outside in [
            // Older than the window on one side, newer on the other, and
            // the Portal set that sits inside the years but outside the
            // format.
            CardSet::Alpha,
            CardSet::FallenEmpires,
            CardSet::PortalSecondAge,
            CardSet::Darksteel,
            CardSet::Innistrad,
        ] {
            assert!(
                !Format::Premodern.allows_set(outside),
                "{outside:?} is not legal"
            );
        }
    }

    /// The ban list is Premodern's own. It shares names with Old School in
    /// both directions -- cards banned there and legal here, and the reverse
    /// -- so neither list can be derived from the other.
    #[test]
    fn premodern_bans_its_own_list_and_restricts_nothing() {
        for banned in [
            "Brainstorm",
            "Force of Will",
            "Necropotence",
            "Strip Mine",
            "Yawgmoth's Will",
            // Whitespace and case come from user input, not from the table.
            "  mind twist  ",
        ] {
            assert!(Format::Premodern.is_banned(banned), "{banned} is banned");
        }
        for legal in [
            "Swords to Plowshares",
            "Wrath of God",
            "Contract from Below",
        ] {
            assert!(!Format::Premodern.is_banned(legal), "{legal} is not");
        }

        assert!(
            Format::Premodern.rules().restricted_cards.is_empty(),
            "Premodern restricts nothing",
        );
        assert!(!Format::Premodern.is_restricted("Black Lotus"));
    }

    /// An old card pool played by new rules, which is the one thing that
    /// separates it from Old School mechanically.
    #[test]
    fn premodern_uses_contemporary_mana_rules() {
        assert!(!Format::Premodern.rules().mana_burn);
        assert!(Format::Premodern.rules().mana_empties_at_end_of_step);
    }

    #[test]
    fn only_old_school_has_mana_burn_and_restrictions() {
        assert!(Format::OldSchool9394.rules().mana_burn);
        assert!(!Format::OldSchool9394.rules().mana_empties_at_end_of_step);
        assert!(Format::OldSchool9394.is_restricted(" black lotus "));
        assert!(Format::OldSchool9394.is_banned("CONTRACT FROM BELOW"));

        assert!(!Format::IsdDgmStandard.rules().mana_burn);
        assert!(Format::IsdDgmStandard.rules().mana_empties_at_end_of_step);
        assert!(!Format::IsdDgmStandard.is_restricted("Black Lotus"));
        assert!(!Format::IsdDgmStandard.is_banned("Contract from Below"));
    }
}
