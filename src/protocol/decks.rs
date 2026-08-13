use crate::{Deck, Format, decks};

const OLD_SCHOOL_DECK_NAMES: &[&str] = &[
    "Goblins",
    "Sligh",
    "Artifacts",
    "Robots",
    "The Deck",
    "Mono Black",
    "White Weenie",
    "Erhnamgeddon",
    "Counterburn",
    "Lions DIB",
    "Lion Dib Bolt",
    "BWR Aggro",
    "GR Aggro",
    "Troll Disk",
    "Jeskai Aggro",
];

const ISD_DGM_STANDARD_DECK_NAMES: &[&str] = &[
    "Briksza Naya Midrange",
    "Greer G/R Aggro",
    "Fyrberg B/G Midrange",
    "Smith Naya Midrange",
    "McDuffie U/W/R Flash",
    "Lorren U/W Flash",
    "Arch U/W Flash",
    "Kuenzinger Junk Reanimator",
    "Anderson Omnidoor Thragfire",
    "Braun-Duin Naya Midrange",
];

/// Parses a public protocol format slug.
///
/// # Errors
///
/// Returns a stable message when the slug does not name a supported format.
pub fn parse_format_slug(slug: &str) -> Result<Format, String> {
    match slug.trim().to_ascii_lowercase().as_str() {
        "old-school-93-94" | "old_school_93_94" => Ok(Format::OldSchool9394),
        "isd-dgm-standard" | "isd_dgm_standard" | "isd-rtr-standard" | "isd_rtr_standard" => {
            Ok(Format::IsdDgmStandard)
        }
        _ => Err(format!("unknown format: {slug}")),
    }
}

/// The deck names accepted for `format`, in menu order.
#[must_use]
pub fn deck_names_for_format(format: Format) -> Vec<&'static str> {
    match format {
        Format::OldSchool9394 => OLD_SCHOOL_DECK_NAMES.to_vec(),
        Format::IsdDgmStandard => ISD_DGM_STANDARD_DECK_NAMES.to_vec(),
    }
}

/// Looks up one built-in deck within `format`, case-insensitively.
#[must_use]
pub fn deck_by_name_for_format(format: Format, name: &str) -> Option<Deck> {
    let name = name.trim().to_ascii_lowercase();
    match format {
        Format::OldSchool9394 => match name.as_str() {
            "goblins" => Some(decks::old_school_93_94::goblins()),
            "sligh" => Some(decks::old_school_93_94::sligh()),
            "artifacts" | "mono red atog" | "mono-red atog" => {
                Some(decks::old_school_93_94::artifacts())
            }
            "robots" => Some(decks::old_school_93_94::robots()),
            "the deck" => Some(decks::old_school_93_94::the_deck()),
            "mono black" => Some(decks::old_school_93_94::mono_black()),
            "white weenie" => Some(decks::old_school_93_94::white_weenie()),
            "erhnamgeddon" => Some(decks::old_school_93_94::erhnamgeddon()),
            "counterburn" => Some(decks::old_school_93_94::counterburn()),
            "lions/dib" | "lions dib" => Some(decks::old_school_93_94::lions_dib()),
            "bwr aggro" => Some(decks::old_school_93_94::bwr_aggro()),
            "gr aggro" | "g/r aggro" => Some(decks::old_school_93_94::gr_aggro()),
            "troll disk" => Some(decks::old_school_93_94::troll_disk()),
            "jeskai aggro" => Some(decks::old_school_93_94::jeskai_aggro()),
            "lion dib bolt" | "lions/dib bolt" | "lions dib bolt" => {
                Some(decks::old_school_93_94::lions_dib_bolt())
            }
            _ => None,
        },
        Format::IsdDgmStandard => match name.as_str() {
            "briksza naya midrange"
            | "rudy briksza naya midrange"
            | "naya midrange rudy briksza"
            | "naya_midrange_rudy_briksza" => {
                Some(decks::isd_dgm_standard::naya_midrange_rudy_briksza())
            }
            "greer g/r aggro"
            | "joseph greer g/r aggro"
            | "g/r aggro joseph greer"
            | "gr_aggro_joseph_greer" => Some(decks::isd_dgm_standard::gr_aggro_joseph_greer()),
            "fyrberg b/g midrange"
            | "mike fyrberg b/g midrange"
            | "b/g midrange mike fyrberg"
            | "bg_midrange_mike_fyrberg" => {
                Some(decks::isd_dgm_standard::bg_midrange_mike_fyrberg())
            }
            "smith naya midrange"
            | "jimmie smith naya midrange"
            | "naya midrange jimmie smith"
            | "naya_midrange_jimmie_smith" => {
                Some(decks::isd_dgm_standard::naya_midrange_jimmie_smith())
            }
            "mcduffie u/w/r flash"
            | "korey mcduffie u/w/r flash"
            | "u/w/r flash korey mcduffie"
            | "uwr_flash_korey_mcduffie" => {
                Some(decks::isd_dgm_standard::uwr_flash_korey_mcduffie())
            }
            "lorren u/w flash"
            | "phillip lorren u/w flash"
            | "u/w flash phillip lorren"
            | "uw_flash_phillip_lorren" => Some(decks::isd_dgm_standard::uw_flash_phillip_lorren()),
            "arch u/w flash"
            | "clayton arch u/w flash"
            | "u/w flash clayton arch"
            | "uw_flash_clayton_arch" => Some(decks::isd_dgm_standard::uw_flash_clayton_arch()),
            "kuenzinger junk reanimator"
            | "drew kuenzinger junk reanimator"
            | "junk reanimator drew kuenzinger"
            | "junk_reanimator_drew_kuenzinger" => {
                Some(decks::isd_dgm_standard::junk_reanimator_drew_kuenzinger())
            }
            "anderson omnidoor thragfire"
            | "todd anderson omnidoor thragfire"
            | "omnidoor thragfire todd anderson"
            | "omnidoor_thragfire_todd_anderson" => {
                Some(decks::isd_dgm_standard::omnidoor_thragfire_todd_anderson())
            }
            "braun-duin naya midrange"
            | "brian braun-duin naya midrange"
            | "naya midrange brian braun-duin"
            | "naya_midrange_brian_braun_duin" => {
                Some(decks::isd_dgm_standard::naya_midrange_brian_braun_duin())
            }
            _ => None,
        },
    }
}

/// The original Old School deck registry, retained for compatibility.
#[must_use]
pub fn deck_names() -> Vec<&'static str> {
    deck_names_for_format(Format::OldSchool9394)
}

/// Looks up an Old School deck by display name, case-insensitively.
#[must_use]
pub fn deck_by_name(name: &str) -> Option<Deck> {
    deck_by_name_for_format(Format::OldSchool9394, name)
}
