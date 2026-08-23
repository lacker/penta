use crate::{Deck, Format, decks};

/// Premodern lists are registered one at a time, as every card in each
/// becomes playable.
const PREMODERN_DECK_NAMES: &[&str] = &[
    "RG Goblins",
    "Sligh",
    "GAT",
    "Landstill",
    "Stasis",
    "BW Control",
    "Replenish",
    "Angry Hermit",
];

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
        "premodern" => Ok(Format::Premodern),
        "isd-m14-standard" | "isd_m14_standard" => Ok(Format::IsdM14Standard),
        "som-m13-standard" | "som_m13_standard" => Ok(Format::SomM13Standard),
        "vintage-cube" | "vintage_cube" => Ok(Format::VintageCube),
        "pauper-cube" | "pauper_cube" => Ok(Format::PauperCube),
        _ => Err(format!("unknown format: {slug}")),
    }
}

/// The deck names accepted for `format`, in menu order.
#[must_use]
pub fn deck_names_for_format(format: Format) -> Vec<&'static str> {
    match format {
        Format::OldSchool9394 => OLD_SCHOOL_DECK_NAMES.to_vec(),
        Format::IsdDgmStandard | Format::IsdM14Standard => ISD_DGM_STANDARD_DECK_NAMES.to_vec(),
        Format::Premodern => PREMODERN_DECK_NAMES.to_vec(),
        // The pool is still being cataloged, so there is nothing to offer
        // yet. A cube is drafted rather than picked from a menu in any case.
        Format::SomM13Standard | Format::VintageCube | Format::PauperCube => Vec::new(),
    }
}

/// Looks up one built-in deck within `format`, case-insensitively.
#[must_use]
pub fn deck_by_name_for_format(format: Format, name: &str) -> Option<Deck> {
    let name = name.trim().to_ascii_lowercase();
    match format {
        Format::SomM13Standard | Format::VintageCube | Format::PauperCube => None,
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
        Format::IsdDgmStandard | Format::IsdM14Standard => match name.as_str() {
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
        // The staged lists are promoted one at a time, as their cards become
        // playable; the rest wait in `decks/premodern/`.
        Format::Premodern => match name.as_str() {
            "rg goblins" | "rg-goblins" => Some(decks::premodern::rg_goblins()),
            "sligh" => Some(decks::premodern::sligh()),
            "gat" => Some(decks::premodern::gat()),
            "landstill" => Some(decks::premodern::landstill()),
            "stasis" => Some(decks::premodern::stasis()),
            "bw control" | "bw-control" => Some(decks::premodern::bw_control()),
            "replenish" => Some(decks::premodern::replenish()),
            "angry hermit" | "angry-hermit" => Some(decks::premodern::angry_hermit()),
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
