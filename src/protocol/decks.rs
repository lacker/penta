use crate::{Deck, Format, decks};

/// Parses a public protocol format slug.
///
/// # Errors
///
/// Returns a stable message when the slug does not name a supported format.
pub fn parse_format_slug(slug: &str) -> Result<Format, String> {
    match slug.trim().to_ascii_lowercase().as_str() {
        "old-school-93-94" | "old_school_93_94" => Ok(Format::OldSchool9394),
        "premodern" => Ok(Format::Premodern),
        "isd-m14-standard" | "isd_m14_standard" => Ok(Format::IsdM14Standard),
        "som-m13-standard" | "som_m13_standard" => Ok(Format::SomM13Standard),
        "vintage-cube" | "vintage_cube" => Ok(Format::VintageCube),
        "pauper-cube" | "pauper_cube" => Ok(Format::PauperCube),
        _ => Err(format!("unknown format: {slug}")),
    }
}

/// The deck names accepted for `format`, in alphanumeric display-name order.
#[must_use]
pub fn deck_names_for_format(format: Format) -> Vec<&'static str> {
    decks::BUILTIN_DECKS
        .iter()
        .filter(|deck| deck.format == Some(format))
        .map(|deck| deck.name)
        .collect()
}

/// Looks up one built-in deck within `format`, case-insensitively.
#[must_use]
pub fn deck_by_name_for_format(format: Format, name: &str) -> Option<Deck> {
    let name = name.trim();
    decks::BUILTIN_DECKS
        .iter()
        .find(|deck| {
            deck.format == Some(format)
                && [&deck.name, &deck.id]
                    .into_iter()
                    .chain(deck.aliases.iter())
                    .any(|candidate| candidate.eq_ignore_ascii_case(name))
        })
        .map(decks::BuiltinDeck::build)
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
