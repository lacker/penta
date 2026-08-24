//! Supported format definitions and the rules they share.

use std::fmt;

use crate::card::{CardDefinition, CardSet, CardStructure};

pub mod cubes;
mod old_school_9394;
mod premodern;
pub mod standards;

pub use old_school_9394::{
    ALLOWED_SETS as OLD_SCHOOL_ALLOWED_SETS, BANNED_CARDS as OLD_SCHOOL_BANNED_CARDS,
    RESTRICTED_CARDS as OLD_SCHOOL_RESTRICTED_CARDS,
};
pub use premodern::{
    ALLOWED_SETS as PREMODERN_ALLOWED_SETS, BANNED_CARDS as PREMODERN_BANNED_CARDS,
    RESTRICTED_CARDS as PREMODERN_RESTRICTED_CARDS,
};
pub use standards::isd_m14::{
    ALLOWED_SETS as ISD_M14_STANDARD_ALLOWED_SETS, BANNED_CARDS as ISD_M14_STANDARD_BANNED_CARDS,
    RESTRICTED_CARDS as ISD_M14_STANDARD_RESTRICTED_CARDS,
};

/// A supported format.
///
/// Keep this value in a [`crate::Game`] rather than consulting global rules:
/// two games using different formats must be able to run in the same process.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Format {
    #[default]
    OldSchool9394,
    Premodern,
    IsdM14Standard,
    SomM13Standard,
    VintageCube,
    PauperCube,
}

/// The family used to group formats in reports and presentation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FormatCategory {
    OldSchool,
    Premodern,
    Standard,
    Cube,
}

impl FormatCategory {
    pub const ALL: &'static [Self] =
        &[Self::OldSchool, Self::Premodern, Self::Standard, Self::Cube];

    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::OldSchool => "Old School",
            Self::Premodern => "Premodern",
            Self::Standard => "Standard",
            Self::Cube => "Cubes",
        }
    }

    #[must_use]
    pub const fn formats(self) -> &'static [Format] {
        match self {
            Self::OldSchool => &[Format::OldSchool9394],
            Self::Premodern => &[Format::Premodern],
            Self::Standard => &[Format::IsdM14Standard, Format::SomM13Standard],
            Self::Cube => &[Format::VintageCube, Format::PauperCube],
        }
    }
}

/// Construction and in-game rules shared by set formats and cubes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FormatRules {
    pub starting_life: u8,
    pub opening_hand_size: usize,
    pub minimum_main_deck_size: usize,
    pub maximum_sideboard_size: usize,
    pub maximum_copies: usize,
    /// Modern rules empty mana pools after every step and phase. EC 93/94
    /// instead keeps mana through steps and empties pools only between phases.
    pub mana_empties_at_end_of_step: bool,
    pub mana_burn: bool,
}

pub(super) const CONSTRUCTED_RULES: FormatRules = FormatRules {
    starting_life: 20,
    opening_hand_size: 7,
    minimum_main_deck_size: 60,
    maximum_sideboard_size: 15,
    maximum_copies: 4,
    mana_empties_at_end_of_step: true,
    mana_burn: false,
};

pub(super) const CUBE_RULES: FormatRules = FormatRules {
    starting_life: 20,
    opening_hand_size: 7,
    minimum_main_deck_size: 40,
    maximum_sideboard_size: 15,
    maximum_copies: 1,
    mana_empties_at_end_of_step: true,
    mana_burn: false,
};

/// A format whose legality comes from printings in sets plus ban policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SetFormatDefinition {
    pub rules: FormatRules,
    pub allowed_sets: &'static [CardSet],
    pub banned_cards: &'static [&'static str],
    pub restricted_cards: &'static [&'static str],
}

/// A cube whose legality comes from membership in one fixed card list.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CubeFormatDefinition {
    pub rules: FormatRules,
    pub cards: &'static [&'static str],
}

/// The legality model for one format. Cubes cannot accidentally be treated as
/// empty set windows, and set formats cannot accidentally acquire a card pool.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FormatDefinition {
    Sets(&'static SetFormatDefinition),
    Cube(&'static CubeFormatDefinition),
}

impl Format {
    /// Every supported format, grouped in [`FormatCategory`] order.
    pub const ALL: &'static [Self] = &[
        Self::OldSchool9394,
        Self::Premodern,
        Self::IsdM14Standard,
        Self::SomM13Standard,
        Self::VintageCube,
        Self::PauperCube,
    ];

    #[must_use]
    pub const fn category(self) -> FormatCategory {
        match self {
            Self::OldSchool9394 => FormatCategory::OldSchool,
            Self::Premodern => FormatCategory::Premodern,
            Self::IsdM14Standard | Self::SomM13Standard => FormatCategory::Standard,
            Self::VintageCube | Self::PauperCube => FormatCategory::Cube,
        }
    }

    #[must_use]
    pub const fn definition(self) -> FormatDefinition {
        match self {
            Self::OldSchool9394 => FormatDefinition::Sets(&old_school_9394::DEFINITION),
            Self::Premodern => FormatDefinition::Sets(&premodern::DEFINITION),
            Self::IsdM14Standard => FormatDefinition::Sets(&standards::isd_m14::DEFINITION),
            Self::SomM13Standard => FormatDefinition::Sets(&standards::som_m13::DEFINITION),
            Self::VintageCube => FormatDefinition::Cube(&cubes::vintage::DEFINITION),
            Self::PauperCube => FormatDefinition::Cube(&cubes::pauper::DEFINITION),
        }
    }

    #[must_use]
    pub const fn rules(self) -> &'static FormatRules {
        match self.definition() {
            FormatDefinition::Sets(definition) => &definition.rules,
            FormatDefinition::Cube(definition) => &definition.rules,
        }
    }

    #[must_use]
    pub const fn set_definition(self) -> Option<&'static SetFormatDefinition> {
        match self.definition() {
            FormatDefinition::Sets(definition) => Some(definition),
            FormatDefinition::Cube(_) => None,
        }
    }

    #[must_use]
    pub const fn cube_definition(self) -> Option<&'static CubeFormatDefinition> {
        match self.definition() {
            FormatDefinition::Sets(_) => None,
            FormatDefinition::Cube(definition) => Some(definition),
        }
    }

    #[must_use]
    pub const fn slug(self) -> &'static str {
        match self {
            Self::OldSchool9394 => "old-school-93-94",
            Self::Premodern => "premodern",
            Self::IsdM14Standard => "isd-m14-standard",
            Self::SomM13Standard => "som-m13-standard",
            Self::VintageCube => "vintage-cube",
            Self::PauperCube => "pauper-cube",
        }
    }

    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::OldSchool9394 => "Old School 93/94",
            Self::Premodern => "Premodern",
            Self::IsdM14Standard => "Standard: ISD-M14",
            Self::SomM13Standard => "Standard: SOM-M13",
            Self::VintageCube => "Cube: Vintage",
            Self::PauperCube => "Cube: The Pauper Cube",
        }
    }

    /// Whether a printing represented by `set` is legal in this format.
    #[must_use]
    pub fn allows_set(self, set: CardSet) -> bool {
        self.set_definition()
            .is_some_and(|definition| definition.allowed_sets.contains(&set))
    }

    /// Whether this card identity can be used to construct a deck.
    #[must_use]
    pub fn allows_card(self, card: &CardDefinition) -> bool {
        if card.is_basic_land() {
            return true;
        }
        match self.definition() {
            FormatDefinition::Cube(definition) => {
                contains_name(definition.cards, &card.name)
                    || matches!(card.structure, CardStructure::DoubleFaced { .. })
                        && card
                            .primary_part()
                            .is_some_and(|part| contains_name(definition.cards, &part.name))
            }
            FormatDefinition::Sets(definition) => card
                .printings
                .iter()
                .any(|printing| definition.allowed_sets.contains(&printing.id.set)),
        }
    }

    #[must_use]
    pub fn is_banned(self, name: &str) -> bool {
        self.set_definition()
            .is_some_and(|definition| contains_name(definition.banned_cards, name))
    }

    #[must_use]
    pub fn is_restricted(self, name: &str) -> bool {
        self.set_definition()
            .is_some_and(|definition| contains_name(definition.restricted_cards, name))
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
mod tests;
