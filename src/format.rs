//! Tournament-format rules that vary independently of individual cards.

use std::fmt;

use crate::card::{CardDefinition, CardSet};

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
    IsdRtrStandard,
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

pub const ISD_RTR_STANDARD_BANNED_CARDS: &[&str] = &[];
pub const ISD_RTR_STANDARD_RESTRICTED_CARDS: &[&str] = &[];
pub const ISD_RTR_STANDARD_ALLOWED_SETS: &[CardSet] = &[
    CardSet::Innistrad,
    CardSet::DarkAscension,
    CardSet::AvacynRestored,
    CardSet::Magic2013,
    CardSet::ReturnToRavnica,
    CardSet::Gatecrash,
    CardSet::DragonsMaze,
    CardSet::Magic2014,
];

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
};

const ISD_RTR_STANDARD_RULES: FormatRules = FormatRules {
    starting_life: 20,
    opening_hand_size: 7,
    minimum_main_deck_size: 60,
    maximum_sideboard_size: 15,
    maximum_copies: 4,
    allowed_sets: ISD_RTR_STANDARD_ALLOWED_SETS,
    mana_empties_at_end_of_step: true,
    mana_burn: false,
    banned_cards: ISD_RTR_STANDARD_BANNED_CARDS,
    restricted_cards: ISD_RTR_STANDARD_RESTRICTED_CARDS,
};

impl Format {
    #[must_use]
    pub const fn rules(self) -> &'static FormatRules {
        match self {
            Self::OldSchool9394 => &OLD_SCHOOL_RULES,
            Self::IsdRtrStandard => &ISD_RTR_STANDARD_RULES,
        }
    }

    #[must_use]
    pub const fn slug(self) -> &'static str {
        match self {
            Self::OldSchool9394 => "old-school-93-94",
            Self::IsdRtrStandard => "isd-rtr-standard",
        }
    }

    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::OldSchool9394 => "Old School 93/94",
            Self::IsdRtrStandard => "ISD-RTR Standard",
        }
    }

    /// Whether a printing represented by `set` is legal in this format.
    #[must_use]
    pub fn allows_set(self, set: CardSet) -> bool {
        self.rules().allowed_sets.contains(&set)
    }

    /// Whether this card identity can be used to construct a deck.
    ///
    /// Basic lands are shared by every supported format. Other cards are legal
    /// when at least one known printing belongs to an allowed set; the physical
    /// printing selected for a deck does not change that identity's legality.
    #[must_use]
    pub fn allows_card(self, card: &CardDefinition) -> bool {
        card.is_basic_land()
            || card
                .printings
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

    use super::Format;
    use crate::CardDefinitionId;
    use crate::card::{CardBehavior, CardDefinition, CardPrinting, CardSet};

    #[test]
    fn format_set_windows_are_nonempty_unique_and_exclude_tokens() {
        for format in [Format::OldSchool9394, Format::IsdRtrStandard] {
            let sets = format.rules().allowed_sets;
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
            CardDefinitionId(1),
            "Old spell",
            CardSet::Alpha,
            false,
            CardBehavior::Unsupported,
        );
        let standard_spell = CardDefinition::new(
            CardDefinitionId(2),
            "Standard spell",
            CardSet::Innistrad,
            false,
            CardBehavior::Unsupported,
        );
        let old_printing_of_a_basic = CardDefinition::new(
            CardDefinitionId(3),
            "Plains",
            CardSet::Alpha,
            true,
            CardBehavior::Plains,
        );

        assert!(Format::OldSchool9394.allows_card(&old_spell));
        assert!(!Format::OldSchool9394.allows_card(&standard_spell));
        assert!(Format::IsdRtrStandard.allows_card(&standard_spell));
        assert!(!Format::IsdRtrStandard.allows_card(&old_spell));
        assert!(Format::IsdRtrStandard.allows_card(&old_printing_of_a_basic));
        assert!(Format::OldSchool9394.allows_card(&old_printing_of_a_basic));
    }

    #[test]
    fn any_allowed_reprint_makes_the_canonical_card_identity_legal() {
        let id = CardDefinitionId(1);
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
        assert!(Format::IsdRtrStandard.allows_card(&reprinted_spell));
    }

    #[test]
    fn only_old_school_has_mana_burn_and_restrictions() {
        assert!(Format::OldSchool9394.rules().mana_burn);
        assert!(!Format::OldSchool9394.rules().mana_empties_at_end_of_step);
        assert!(Format::OldSchool9394.is_restricted(" black lotus "));
        assert!(Format::OldSchool9394.is_banned("CONTRACT FROM BELOW"));

        assert!(!Format::IsdRtrStandard.rules().mana_burn);
        assert!(Format::IsdRtrStandard.rules().mana_empties_at_end_of_step);
        assert!(!Format::IsdRtrStandard.is_restricted("Black Lotus"));
        assert!(!Format::IsdRtrStandard.is_banned("Contract from Below"));
    }
}
