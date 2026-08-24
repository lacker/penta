//! Built-in decklists compiled from the YAML files in the repository's `decks/` directory.

use std::error::Error;
use std::fmt;

use crate::Deck;
use crate::card::{self, CardCatalog};

#[derive(Clone, Copy)]
enum Zone {
    Main,
    Sideboard,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum BuiltinDeckError {
    DuplicateSection { line: usize, section: &'static str },
    EntryOutsideSection { line: usize },
    InvalidEntry { line: usize },
    InvalidCount { line: usize, value: String },
    UnknownCard { line: usize, name: String },
    MissingSection(&'static str),
}

impl fmt::Display for BuiltinDeckError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateSection { line, section } => {
                write!(formatter, "duplicate {section} section on line {line}")
            }
            Self::EntryOutsideSection { line } => {
                write!(formatter, "deck entry outside a section on line {line}")
            }
            Self::InvalidEntry { line } => write!(formatter, "invalid deck entry on line {line}"),
            Self::InvalidCount { line, value } => {
                write!(formatter, "invalid card count {value:?} on line {line}")
            }
            Self::UnknownCard { line, name } => {
                write!(formatter, "unknown card {name:?} on line {line}")
            }
            Self::MissingSection(section) => write!(formatter, "missing {section} section"),
        }
    }
}

impl Error for BuiltinDeckError {}

fn parse(yaml: &str, catalog: &CardCatalog) -> Result<Deck, BuiltinDeckError> {
    let mut deck = Deck {
        main: Vec::new(),
        sideboard: Vec::new(),
    };
    let mut zone = None;
    let mut saw_main = false;
    let mut saw_sideboard = false;

    for (index, raw_line) in yaml.lines().enumerate() {
        let line_number = index + 1;
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        match line {
            "main:" => {
                if saw_main {
                    return Err(BuiltinDeckError::DuplicateSection {
                        line: line_number,
                        section: "main",
                    });
                }
                saw_main = true;
                zone = Some(Zone::Main);
                continue;
            }
            "sideboard:" => {
                if saw_sideboard {
                    return Err(BuiltinDeckError::DuplicateSection {
                        line: line_number,
                        section: "sideboard",
                    });
                }
                saw_sideboard = true;
                zone = Some(Zone::Sideboard);
                continue;
            }
            _ => {}
        }

        let Some(current_zone) = zone else {
            return Err(BuiltinDeckError::EntryOutsideSection { line: line_number });
        };
        let Some((name, raw_count)) = line.rsplit_once(':') else {
            return Err(BuiltinDeckError::InvalidEntry { line: line_number });
        };
        let name = name.trim();
        if name.is_empty() {
            return Err(BuiltinDeckError::InvalidEntry { line: line_number });
        }
        let count =
            raw_count
                .trim()
                .parse::<usize>()
                .map_err(|_| BuiltinDeckError::InvalidCount {
                    line: line_number,
                    value: raw_count.trim().into(),
                })?;
        let id = catalog
            .find_by_name(name)
            .ok_or_else(|| BuiltinDeckError::UnknownCard {
                line: line_number,
                name: name.into(),
            })?;
        let cards = match current_zone {
            Zone::Main => &mut deck.main,
            Zone::Sideboard => &mut deck.sideboard,
        };
        cards.extend(std::iter::repeat_n(id, count));
    }

    if !saw_main {
        return Err(BuiltinDeckError::MissingSection("main"));
    }
    if !saw_sideboard {
        return Err(BuiltinDeckError::MissingSection("sideboard"));
    }
    Ok(deck)
}

fn builtin(yaml: &str) -> Deck {
    let catalog = card::catalog().expect("built-in card catalog must be valid");
    parse(yaml, &catalog).expect("built-in deck YAML must be valid")
}

macro_rules! deck {
    ($name:ident, $format:literal, $file:literal, $description:literal) => {
        #[doc = $description]
        #[must_use]
        pub fn $name() -> crate::Deck {
            super::builtin(include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/decks/",
                $format,
                "/",
                $file
            )))
        }
    };
}

/// Built-in Premodern decklists, from the Sacred Torch Showdown Top 8.
///
/// A list is registered only once every card in it resolves; the rest of the
/// staged tranche stays in `decks/premodern/` until it does.
pub mod premodern {
    deck!(
        rg_goblins,
        "premodern",
        "rg_goblins_andy_dominguez.yaml",
        "Returns Andy Dominguez's RG Goblins list from the Sacred Torch Showdown."
    );
    deck!(
        sligh,
        "premodern",
        "sligh_neal_sacks.yaml",
        "Returns Neal Sacks's Sligh list from the Sacred Torch Showdown."
    );
    deck!(
        gat,
        "premodern",
        "gat_daniel_sondike.yaml",
        "Returns Daniel Sondike's GAT list from the Sacred Torch Showdown."
    );
    deck!(
        landstill,
        "premodern",
        "landstill_tentaclefan.yaml",
        "Returns the sixth-place Landstill list from the Sacred Torch Showdown."
    );
    deck!(
        stasis,
        "premodern",
        "stasis_drew_glauberg.yaml",
        "Returns Drew Glauberg's Stasis list from the Sacred Torch Showdown."
    );
    deck!(
        bw_control,
        "premodern",
        "bw_control_chris_danis.yaml",
        "Returns Chris Danis's BW Control list from the Sacred Torch Showdown."
    );
    deck!(
        replenish,
        "premodern",
        "replenish_bryan_gulotta.yaml",
        "Returns Bryan Gulotta's Replenish list from the Sacred Torch Showdown."
    );
    deck!(
        angry_hermit,
        "premodern",
        "angry_hermit_ryan_marvin.yaml",
        "Returns Ryan Marvin's Angry Hermit list from the Sacred Torch Showdown."
    );
}

/// Built-in Eternal Central Old School 93/94 decklists.
pub mod old_school_93_94 {
    deck!(
        goblins,
        "old_school_93_94",
        "goblins.yaml",
        "Returns a representative powered EC Goblins deck."
    );
    deck!(
        sligh,
        "old_school_93_94",
        "sligh.yaml",
        "Returns a representative powered EC Sligh deck."
    );
    deck!(
        artifacts,
        "old_school_93_94",
        "artifacts.yaml",
        "Returns a representative powered EC Atog artifact deck."
    );
    deck!(
        robots,
        "old_school_93_94",
        "robots.yaml",
        "Returns a representative powered EC mono-red Robots deck."
    );
    deck!(
        the_deck,
        "old_school_93_94",
        "the_deck.yaml",
        "Returns the classic powered EC control deck known as The Deck."
    );
    deck!(
        mono_black,
        "old_school_93_94",
        "mono_black.yaml",
        "Returns a representative powered EC Mono Black deck."
    );
    deck!(
        white_weenie,
        "old_school_93_94",
        "white_weenie.yaml",
        "Returns a representative powered EC White Weenie deck."
    );
    deck!(
        erhnamgeddon,
        "old_school_93_94",
        "erhnamgeddon.yaml",
        "Returns a representative powered EC Erhnamgeddon deck."
    );
    deck!(
        counterburn,
        "old_school_93_94",
        "counterburn.yaml",
        "Returns a representative powered EC Counterburn deck."
    );
    deck!(
        lions_dib,
        "old_school_93_94",
        "lions_dib.yaml",
        "Returns a representative powered EC Lions/Dib deck."
    );
    deck!(
        bwr_aggro,
        "old_school_93_94",
        "bwr_aggro.yaml",
        "Returns a representative powered BWR aggro deck."
    );
    deck!(
        gr_aggro,
        "old_school_93_94",
        "gr_aggro.yaml",
        "Returns a representative powered green-red aggro deck."
    );
    deck!(
        troll_disk,
        "old_school_93_94",
        "troll_disk.yaml",
        "Returns a representative powered Sedge Troll / Disk deck."
    );
    deck!(
        jeskai_aggro,
        "old_school_93_94",
        "jeskai_aggro.yaml",
        "Returns a representative powered Jeskai tempo deck."
    );
    deck!(
        lions_dib_bolt,
        "old_school_93_94",
        "lions_dib_bolt.yaml",
        "Returns the Lion/Dib shell with its burn package."
    );

    /// Backwards-compatible name for the built-in artifact deck.
    #[must_use]
    pub fn mono_red_atog() -> crate::Deck {
        artifacts()
    }
}

/// Built-in decks legal in the final pre-Theros ISD–M14 Standard card pool.
pub mod isd_m14_standard {
    deck!(
        naya_midrange_rudy_briksza,
        "isd_m14_standard",
        "naya_midrange_rudy_briksza.yaml",
        "Returns Rudy Briksza's first-place Naya Midrange deck from SCG Open Atlanta."
    );
    deck!(
        gr_aggro_joseph_greer,
        "isd_m14_standard",
        "gr_aggro_joseph_greer.yaml",
        "Returns Joseph Greer's second-place G/R Aggro deck from SCG Open Atlanta."
    );
    deck!(
        bg_midrange_mike_fyrberg,
        "isd_m14_standard",
        "bg_midrange_mike_fyrberg.yaml",
        "Returns Mike Fyrberg's third-place B/G Midrange deck from SCG Open Atlanta."
    );
    deck!(
        naya_midrange_jimmie_smith,
        "isd_m14_standard",
        "naya_midrange_jimmie_smith.yaml",
        "Returns Jimmie Smith's fourth-place Naya Midrange deck from SCG Open Atlanta."
    );
    deck!(
        uwr_flash_korey_mcduffie,
        "isd_m14_standard",
        "uwr_flash_korey_mcduffie.yaml",
        "Returns the fifth-place U/W/R Flash deck piloted by Korey `McDuffie` at SCG Open Atlanta."
    );
    deck!(
        uw_flash_phillip_lorren,
        "isd_m14_standard",
        "uw_flash_phillip_lorren.yaml",
        "Returns Phillip Lorren's sixth-place U/W Flash deck from SCG Open Atlanta."
    );
    deck!(
        uw_flash_clayton_arch,
        "isd_m14_standard",
        "uw_flash_clayton_arch.yaml",
        "Returns a legality-corrected version of Clayton Arch's seventh-place U/W Flash deck from SCG Open Atlanta."
    );
    deck!(
        junk_reanimator_drew_kuenzinger,
        "isd_m14_standard",
        "junk_reanimator_drew_kuenzinger.yaml",
        "Returns Drew Kuenzinger's eighth-place Junk Reanimator deck from SCG Open Atlanta."
    );
    deck!(
        omnidoor_thragfire_todd_anderson,
        "isd_m14_standard",
        "omnidoor_thragfire_todd_anderson.yaml",
        "Returns Todd Anderson's Omnidoor Thragfire test deck from January 2013."
    );
    deck!(
        naya_midrange_brian_braun_duin,
        "isd_m14_standard",
        "naya_midrange_brian_braun_duin.yaml",
        "Returns Brian Braun-Duin's Naya Midrange test deck from January 2013."
    );
}

macro_rules! old_school_compatibility_wrapper {
    ($name:ident, $description:literal) => {
        #[doc = $description]
        #[must_use]
        pub fn $name() -> Deck {
            old_school_93_94::$name()
        }
    };
}

old_school_compatibility_wrapper!(goblins, "Returns a representative powered EC Goblins deck.");
old_school_compatibility_wrapper!(sligh, "Returns a representative powered EC Sligh deck.");
old_school_compatibility_wrapper!(
    artifacts,
    "Returns a representative powered EC Atog artifact deck."
);
old_school_compatibility_wrapper!(
    robots,
    "Returns a representative powered EC mono-red Robots deck."
);
old_school_compatibility_wrapper!(
    the_deck,
    "Returns the classic powered EC control deck known as The Deck."
);
old_school_compatibility_wrapper!(
    mono_black,
    "Returns a representative powered EC Mono Black deck."
);
old_school_compatibility_wrapper!(
    white_weenie,
    "Returns a representative powered EC White Weenie deck."
);
old_school_compatibility_wrapper!(
    erhnamgeddon,
    "Returns a representative powered EC Erhnamgeddon deck."
);
old_school_compatibility_wrapper!(
    counterburn,
    "Returns a representative powered EC Counterburn deck."
);
old_school_compatibility_wrapper!(
    lions_dib,
    "Returns a representative powered EC Lions/Dib deck."
);
old_school_compatibility_wrapper!(
    bwr_aggro,
    "Returns a representative powered BWR aggro deck."
);
old_school_compatibility_wrapper!(
    gr_aggro,
    "Returns a representative powered green-red aggro deck."
);
old_school_compatibility_wrapper!(
    troll_disk,
    "Returns a representative powered Sedge Troll / Disk deck."
);
old_school_compatibility_wrapper!(
    jeskai_aggro,
    "Returns a representative powered Jeskai tempo deck."
);
old_school_compatibility_wrapper!(
    lions_dib_bolt,
    "Returns the Lion/Dib shell with its burn package."
);

/// Backwards-compatible name for the built-in artifact deck.
#[must_use]
pub fn mono_red_atog() -> Deck {
    old_school_93_94::mono_red_atog()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::{
        BuiltinDeckError, artifacts, bwr_aggro, counterburn, erhnamgeddon, goblins, gr_aggro,
        isd_m14_standard, jeskai_aggro, lions_dib, lions_dib_bolt, mono_black, mono_red_atog,
        old_school_93_94, parse, robots, sligh, the_deck, troll_disk, white_weenie,
    };
    use crate::card;
    use crate::{Deck, Format};

    type DeckBuilder = fn() -> Deck;

    const PREMODERN_TOP_8: &[(&str, &str, usize)] = &[
        (
            "Sligh — Neal Sacks",
            include_str!("../decks/premodern/sligh_neal_sacks.yaml"),
            60,
        ),
        (
            "GAT — Daniel Sondike",
            include_str!("../decks/premodern/gat_daniel_sondike.yaml"),
            60,
        ),
        (
            "Replenish — Bryan Gulotta",
            include_str!("../decks/premodern/replenish_bryan_gulotta.yaml"),
            60,
        ),
        (
            "Stasis — Drew Glauberg",
            include_str!("../decks/premodern/stasis_drew_glauberg.yaml"),
            61,
        ),
        (
            "BW Control — Chris Danis",
            include_str!("../decks/premodern/bw_control_chris_danis.yaml"),
            60,
        ),
        (
            "Landstill — TentacleFan",
            include_str!("../decks/premodern/landstill_tentaclefan.yaml"),
            60,
        ),
        (
            "RG Goblins — Andy Dominguez",
            include_str!("../decks/premodern/rg_goblins_andy_dominguez.yaml"),
            60,
        ),
        (
            "Angry Hermit — Ryan Marvin",
            include_str!("../decks/premodern/angry_hermit_ryan_marvin.yaml"),
            60,
        ),
    ];

    fn staged_deck_counts_and_names(yaml: &str) -> (usize, usize, BTreeSet<&str>) {
        let mut zone = None;
        let mut main = 0;
        let mut sideboard = 0;
        let mut names = BTreeSet::new();

        for raw_line in yaml.lines() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            match line {
                "main:" => {
                    zone = Some("main");
                    continue;
                }
                "sideboard:" => {
                    zone = Some("sideboard");
                    continue;
                }
                _ => {}
            }

            let (name, raw_count) = line
                .rsplit_once(':')
                .unwrap_or_else(|| panic!("invalid staged deck entry: {line}"));
            let count = raw_count
                .trim()
                .parse::<usize>()
                .unwrap_or_else(|_| panic!("invalid staged deck count: {line}"));
            names.insert(name.trim());
            match zone {
                Some("main") => main += count,
                Some("sideboard") => sideboard += count,
                _ => panic!("staged deck entry outside a section: {line}"),
            }
        }

        (main, sideboard, names)
    }

    #[test]
    fn parser_reports_unknown_cards_with_their_line() {
        let catalog = card::catalog().unwrap();
        let error = parse("main:\n  Not a Card: 60\nsideboard:\n", &catalog).unwrap_err();

        assert_eq!(
            error,
            BuiltinDeckError::UnknownCard {
                line: 2,
                name: "Not a Card".into(),
            }
        );
    }

    #[test]
    fn old_school_top_level_builders_remain_compatible() {
        let builders: &[(DeckBuilder, DeckBuilder)] = &[
            (goblins, old_school_93_94::goblins),
            (sligh, old_school_93_94::sligh),
            (artifacts, old_school_93_94::artifacts),
            (robots, old_school_93_94::robots),
            (the_deck, old_school_93_94::the_deck),
            (mono_black, old_school_93_94::mono_black),
            (white_weenie, old_school_93_94::white_weenie),
            (erhnamgeddon, old_school_93_94::erhnamgeddon),
            (counterburn, old_school_93_94::counterburn),
            (lions_dib, old_school_93_94::lions_dib),
            (bwr_aggro, old_school_93_94::bwr_aggro),
            (gr_aggro, old_school_93_94::gr_aggro),
            (troll_disk, old_school_93_94::troll_disk),
            (jeskai_aggro, old_school_93_94::jeskai_aggro),
            (lions_dib_bolt, old_school_93_94::lions_dib_bolt),
            (mono_red_atog, old_school_93_94::mono_red_atog),
        ];

        for (top_level, namespaced) in builders {
            assert_eq!(top_level(), namespaced());
        }
    }

    #[test]
    fn standard_decks_parse_from_the_union_catalog_and_are_legal_in_their_profiles() {
        let catalog = card::catalog().unwrap();
        let m14_builders: &[fn() -> Deck] = &[
            isd_m14_standard::naya_midrange_rudy_briksza,
            isd_m14_standard::gr_aggro_joseph_greer,
            isd_m14_standard::bg_midrange_mike_fyrberg,
            isd_m14_standard::naya_midrange_jimmie_smith,
            isd_m14_standard::uwr_flash_korey_mcduffie,
            isd_m14_standard::uw_flash_phillip_lorren,
            isd_m14_standard::uw_flash_clayton_arch,
            isd_m14_standard::junk_reanimator_drew_kuenzinger,
            isd_m14_standard::omnidoor_thragfire_todd_anderson,
            isd_m14_standard::naya_midrange_brian_braun_duin,
        ];

        for build in m14_builders {
            let deck = build();
            assert_eq!(deck.main.len(), 60);
            assert_eq!(deck.sideboard.len(), 15);
            deck.validate_for_format(&catalog, Format::IsdM14Standard)
                .unwrap();
        }
    }

    /// A registered Premodern list has to be legal in the format it is
    /// registered under, and no card in it may be metadata-only -- publishing
    /// a deck whose cards the engine cannot carry out would offer legal
    /// actions it then fails to perform.
    #[test]
    fn the_registered_premodern_deck_is_legal_and_fully_playable() {
        let catalog = crate::card::catalog().expect("catalog builds");
        let mut partial = BTreeSet::new();
        for build in [
            super::premodern::rg_goblins as fn() -> crate::Deck,
            super::premodern::sligh,
            super::premodern::gat,
            super::premodern::landstill,
            super::premodern::stasis,
            super::premodern::bw_control,
            super::premodern::replenish,
            super::premodern::angry_hermit,
        ] {
            let deck = build();
            // Sixty is the floor rather than the size: Drew Glauberg
            // registered sixty-one, and a list is transcribed as it was
            // submitted rather than trimmed to a round number.
            assert!(deck.main.len() >= 60, "a legal main deck is at least 60");
            assert_eq!(deck.sideboard.len(), 15);
            deck.clone()
                .validate_for_format(&catalog, Format::Premodern)
                .expect("the list is Premodern legal");

            for definition in deck.main.iter().copied() {
                let card = catalog.get(definition).expect("every card is cataloged");
                assert_ne!(
                    card.rules.implementation_status(),
                    crate::ImplementationStatus::MetadataOnly,
                    "{} is in a registered main deck and does nothing at all",
                    card.name,
                );
                if card.rules.implementation_status() == crate::ImplementationStatus::Partial {
                    partial.insert(card.name.clone());
                }
            }
        }

        // A partial card is allowed in a registered deck only when its main
        // function resolves and the gap is a rider. Naming them here is what
        // stops the list growing quietly: a new partial fails this until
        // somebody decides it is acceptable and says so. Nothing registered is
        // partial today, and an empty set is the strongest form of that.
        assert_eq!(
            partial,
            BTreeSet::new(),
            "the partial cards in registered decks are not the expected ones",
        );
    }

    #[test]
    fn premodern_top_8_lists_retain_their_sources_and_submitted_sizes() {
        let mut unique_cards = BTreeSet::new();

        for (deck_name, yaml, expected_main) in PREMODERN_TOP_8 {
            assert!(
                yaml.contains("# Source: https://melee.gg/Decklist/View/"),
                "{deck_name} must retain its submitted-list source"
            );
            let (main, sideboard, names) = staged_deck_counts_and_names(yaml);
            assert_eq!(main, *expected_main, "{deck_name} main deck");
            assert_eq!(sideboard, 15, "{deck_name} sideboard");
            unique_cards.extend(names);
        }

        assert_eq!(unique_cards.len(), 145);
    }
}
