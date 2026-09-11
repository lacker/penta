use std::collections::HashMap;

use super::{CardSet, CatalogError};

/// A catalog may repeat a set across cards, but its code and wire slug must
/// describe one unambiguous identity throughout definitions and printings.
#[derive(Default)]
pub(super) struct SetMetadata {
    slugs_by_code: HashMap<&'static str, &'static str>,
    codes_by_slug: HashMap<&'static str, &'static str>,
}

impl SetMetadata {
    pub(super) fn register(&mut self, set: CardSet) -> Result<(), CatalogError> {
        if let Some(&slug) = self.slugs_by_code.get(set.code())
            && slug != set.slug()
        {
            return Err(CatalogError::ConflictingSetSlug {
                code: set.code(),
                first: slug,
                second: set.slug(),
            });
        }
        if let Some(&code) = self.codes_by_slug.get(set.slug())
            && code != set.code()
        {
            return Err(CatalogError::DuplicateSetSlug {
                slug: set.slug(),
                first: code,
                second: set.code(),
            });
        }
        self.slugs_by_code.insert(set.code(), set.slug());
        self.codes_by_slug.insert(set.slug(), set.code());
        Ok(())
    }
}
