//! Built-in decks generated from every YAML file under `decks/` at build time.
//!
//! Names, aliases, descriptions, and card lists are authored only in YAML. The
//! generated constructors retain the native API without a second registry.

use crate::card::{self, CardCatalog};
use crate::{Deck, Format};

pub(crate) struct BuiltinDeck {
    pub(crate) format: Format,
    pub(crate) id: &'static str,
    pub(crate) name: &'static str,
    pub(crate) aliases: &'static [&'static str],
    source: &'static str,
    main: &'static [(&'static str, usize)],
    sideboard: &'static [(&'static str, usize)],
}

impl BuiltinDeck {
    pub(crate) fn build(&self) -> Deck {
        let catalog = card::catalog().expect("built-in card catalog must be valid");
        self.resolve(&catalog)
    }

    fn resolve(&self, catalog: &CardCatalog) -> Deck {
        let resolve = |entries: &[(&str, usize)]| {
            entries
                .iter()
                .flat_map(|&(name, count)| {
                    let id = catalog
                        .find_by_name(name)
                        .unwrap_or_else(|| panic!("{}: unknown card {name:?}", self.source));
                    std::iter::repeat_n(id, count)
                })
                .collect()
        };
        Deck {
            main: resolve(self.main),
            sideboard: resolve(self.sideboard),
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/builtin_decks.rs"));

// The original Old School constructors remain available at the module root.
pub use old_school_93_94::*;

#[cfg(test)]
mod codegen;
#[cfg(test)]
mod tests;
