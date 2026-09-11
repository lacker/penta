use super::{
    CardArt, CardComposition, CardDefinition, CardPrinting, CardRules, CardSet, DoubleFacedKind,
};
use crate::CardDefinitionId;

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
///
/// Debut-set ownership comes from the set module that registers the record;
/// declarations only carry the metadata that varies from card to card.
pub(super) struct CardRecord {
    pub(super) name: &'static str,
    pub(super) art: CardArt,
    pub(super) rules: CardRules,
    composition: Option<CompositionSource>,
}

impl CardRecord {
    /// Defines a card keyed by its exact debut printing's natural UUID.
    #[allow(clippy::large_types_passed_by_value)]
    pub(super) const fn new(
        name: &'static str,
        scryfall_id: &'static str,
        artist: &'static str,
        rules: CardRules,
    ) -> Self {
        Self {
            name,
            art: CardArt::new(scryfall_id, artist),
            rules,
            composition: None,
        }
    }

    const fn new_double_faced(
        name: &'static str,
        scryfall_id: &'static str,
        artist: &'static str,
        faces: &'static [(&'static str, CardRules); 2],
        kind: DoubleFacedKind,
    ) -> Self {
        Self {
            name,
            art: CardArt::new(scryfall_id, artist),
            rules: faces[0].1,
            composition: Some(CompositionSource::DoubleFaced { faces, kind }),
        }
    }

    pub(super) const fn new_dfc(
        name: &'static str,
        scryfall_id: &'static str,
        artist: &'static str,
        faces: &'static [(&'static str, CardRules); 2],
    ) -> Self {
        Self::new_double_faced(
            name,
            scryfall_id,
            artist,
            faces,
            DoubleFacedKind::Transforming,
        )
    }

    pub(super) const fn new_mdfc(
        name: &'static str,
        scryfall_id: &'static str,
        artist: &'static str,
        faces: &'static [(&'static str, CardRules); 2],
    ) -> Self {
        Self::new_double_faced(name, scryfall_id, artist, faces, DoubleFacedKind::Modal)
    }

    pub(super) const fn new_split(
        name: &'static str,
        scryfall_id: &'static str,
        artist: &'static str,
        halves: &'static [(&'static str, CardRules); 2],
    ) -> Self {
        Self {
            name,
            art: CardArt::new(scryfall_id, artist),
            rules: halves[0].1,
            composition: Some(CompositionSource::Split {
                halves,
                fuse_cost: None,
            }),
        }
    }

    pub(super) const fn new_fuse(
        name: &'static str,
        scryfall_id: &'static str,
        artist: &'static str,
        halves: &'static [(&'static str, CardRules); 2],
        fuse_cost: super::ManaCost,
    ) -> Self {
        let mut record = Self::new_split(name, scryfall_id, artist, halves);
        record.composition = Some(CompositionSource::Split {
            halves,
            fuse_cost: Some(fuse_cost),
        });
        record
    }

    #[must_use]
    pub(super) fn id(&self) -> CardDefinitionId {
        CardDefinitionId::from_uuid(self.art.scryfall_id)
    }

    /// Supplies logical parts and play options for a structured or modal card.
    #[must_use]
    pub(super) const fn with_composition(mut self, builder: CompositionBuilder) -> Self {
        self.composition = Some(CompositionSource::Builder(builder));
        self
    }

    pub(super) fn definition(&self, debut_set: CardSet) -> CardDefinition {
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
            debut_set,
            printings: vec![CardPrinting::with_art(id, debut_set, self.art)],
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
    use crate::card::CardRules;

    const ART_ID: &str = "00000000-0000-0000-0000-000000000001";

    fn derived() -> CardRecord {
        CardRecord::new(
            "Derived identity test",
            ART_ID,
            "Test Artist",
            CardRules::unsupported(),
        )
    }

    #[test]
    fn printing_keys_use_the_authored_uuid() {
        assert_eq!(derived().id(), CardDefinitionId::from_uuid(ART_ID));
    }
}
