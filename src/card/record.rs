use super::{
    CardArt, CardComposition, CardDefinition, CardPrinting, CardRules, CardSet, DoubleFacedKind,
};
use crate::CardDefinitionId;
use sha2::{Digest, Sha256};

type CompositionBuilder = fn() -> CardComposition;

#[derive(Clone, Copy)]
enum CompositionSource {
    Builder(CompositionBuilder),
    DoubleFaced {
        faces: &'static [(&'static str, CardRules); 2],
        kind: DoubleFacedKind,
    },
    Split {
        halves: &'static [(&'static str, CardRules); 2],
        fuse_cost: Option<super::ManaCost>,
    },
}

/// Internal source record from which the runtime catalog is built.
pub(super) struct CardRecord {
    pub(super) name: &'static str,
    pub(super) art: CardArt,
    pub(super) debut_set: CardSet,
    pub(super) rules: CardRules,
    composition: Option<CompositionSource>,
}

impl CardRecord {
    /// Defines a card whose ID is derived from its exact debut artwork.
    #[allow(clippy::large_types_passed_by_value)]
    pub(super) const fn new(
        debut_set: CardSet,
        name: &'static str,
        scryfall_id: &'static str,
        artist: &'static str,
        rules: CardRules,
    ) -> Self {
        Self {
            name,
            art: CardArt::new(scryfall_id, artist),
            debut_set,
            rules,
            composition: None,
        }
    }

    /// Defines a double-faced card whose ID is derived from its immutable
    /// debut artwork.
    const fn new_double_faced(
        debut_set: CardSet,
        name: &'static str,
        scryfall_id: &'static str,
        artist: &'static str,
        faces: &'static [(&'static str, CardRules); 2],
        kind: DoubleFacedKind,
    ) -> Self {
        Self {
            name,
            art: CardArt::new(scryfall_id, artist),
            debut_set,
            rules: faces[0].1,
            composition: Some(CompositionSource::DoubleFaced { faces, kind }),
        }
    }

    /// Defines a transforming double-faced card whose ID is derived from its
    /// exact debut artwork.
    pub(super) const fn new_dfc(
        debut_set: CardSet,
        name: &'static str,
        scryfall_id: &'static str,
        artist: &'static str,
        faces: &'static [(&'static str, CardRules); 2],
    ) -> Self {
        Self::new_double_faced(
            debut_set,
            name,
            scryfall_id,
            artist,
            faces,
            DoubleFacedKind::Transforming,
        )
    }

    /// Defines a modal double-faced card whose ID is derived from its
    /// exact debut artwork.
    pub(super) const fn new_mdfc(
        debut_set: CardSet,
        name: &'static str,
        scryfall_id: &'static str,
        artist: &'static str,
        faces: &'static [(&'static str, CardRules); 2],
    ) -> Self {
        Self::new_double_faced(
            debut_set,
            name,
            scryfall_id,
            artist,
            faces,
            DoubleFacedKind::Modal,
        )
    }

    /// Defines a split card directly from its two printed halves.
    pub(super) const fn new_split(
        debut_set: CardSet,
        name: &'static str,
        scryfall_id: &'static str,
        artist: &'static str,
        halves: &'static [(&'static str, CardRules); 2],
    ) -> Self {
        Self {
            name,
            art: CardArt::new(scryfall_id, artist),
            debut_set,
            rules: halves[0].1,
            composition: Some(CompositionSource::Split {
                halves,
                fuse_cost: None,
            }),
        }
    }

    /// Defines a fuse card directly from its two printed halves and combined cost.
    pub(super) const fn new_fuse(
        debut_set: CardSet,
        name: &'static str,
        scryfall_id: &'static str,
        artist: &'static str,
        halves: &'static [(&'static str, CardRules); 2],
        fuse_cost: super::ManaCost,
    ) -> Self {
        let mut record = Self::new_split(debut_set, name, scryfall_id, artist, halves);
        record.composition = Some(CompositionSource::Split {
            halves,
            fuse_cost: Some(fuse_cost),
        });
        record
    }

    #[must_use]
    pub(super) fn id(&self) -> CardDefinitionId {
        super::compatibility::historical_definition_id(self.art.scryfall_id).unwrap_or_else(|| {
            let mut hash = Sha256::new();
            hash.update(b"penta/card-printing-id/v1\0");
            hash.update(self.art.scryfall_id.as_bytes());
            hash.update(0_u32.to_be_bytes());
            let digest = hash.finalize();
            let prefix = u64::from_be_bytes(
                digest[..8]
                    .try_into()
                    .expect("SHA-256 digest always has an eight-byte prefix"),
            );
            CardDefinitionId::new(prefix >> 12)
        })
    }

    /// Supplies logical parts and play options for a structured or modal card.
    #[must_use]
    pub(super) const fn with_composition(mut self, builder: CompositionBuilder) -> Self {
        self.composition = Some(CompositionSource::Builder(builder));
        self
    }

    pub(super) fn definition(&self) -> CardDefinition {
        let id = self.id();
        let composition = match self.composition {
            None => CardComposition::single(self.name, self.rules),
            Some(CompositionSource::Builder(builder)) => builder(),
            Some(CompositionSource::DoubleFaced { faces, kind }) => {
                CardComposition::double_faced(faces, kind)
            }
            Some(CompositionSource::Split { halves, fuse_cost }) => {
                CardComposition::split(halves, fuse_cost)
            }
        };
        CardDefinition {
            id,
            name: self.name.into(),
            art: Some(self.art),
            debut_set: self.debut_set,
            printings: vec![CardPrinting::with_art(id, self.debut_set, self.art)],
            rules: self.rules,
            parts: composition.parts,
            structure: composition.structure,
            play_options: composition.play_options,
        }
    }
}

/// A reprint or alternate-art printing whose rules come from `card`.
pub(super) struct PrintingRecord {
    pub(super) card: &'static CardRecord,
    pub(super) variant: u16,
    pub(super) art: CardArt,
}

impl PrintingRecord {
    /// Adds the default variant of `card` to another set.
    pub(super) const fn reprint(
        card: &'static CardRecord,
        scryfall_id: &'static str,
        artist: &'static str,
    ) -> Self {
        Self {
            card,
            variant: 0,
            art: CardArt::new(scryfall_id, artist),
        }
    }

    /// Adds another distinguishable printing of `card` within the same set.
    pub(super) const fn alternate(
        card: &'static CardRecord,
        variant: u16,
        scryfall_id: &'static str,
        artist: &'static str,
    ) -> Self {
        assert!(variant > 0, "alternate printing variants start at one");
        Self {
            card,
            variant,
            art: CardArt::new(scryfall_id, artist),
        }
    }

    pub(super) fn printing(&self, set: CardSet) -> CardPrinting {
        CardPrinting::with_variant_and_art(self.card.id(), set, self.variant, self.art)
    }
}

#[cfg(test)]
mod tests {
    use super::CardRecord;
    use crate::CardDefinitionId;
    use crate::card::{CardRules, CardSet};

    const ANCHOR: &str = "00000000-0000-0000-0000-000000000001";

    fn derived() -> CardRecord {
        CardRecord::new(
            CardSet::Alpha,
            "Derived identity test",
            ANCHOR,
            "Test Artist",
            CardRules::unsupported(),
        )
    }

    #[test]
    fn printing_ids_follow_the_frozen_sha256_vector() {
        assert_eq!(derived().id(), CardDefinitionId::new(4_013_269_539_742_549),);
    }
}
