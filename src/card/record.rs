use super::{
    AbilityDef, CardArt, CardComposition, CardDefinition, CardPrinting, CardRules, CardSet,
};
use crate::game::CardAbilityResolver;
use crate::{AbilityId, CardDefinitionId, CardPartId, TargetSlotId};
use sha2::{Digest, Sha256};

type CompositionBuilder = fn() -> CardComposition;

/// Immutable exact first-printing anchor from which a new definition ID is derived.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(super) struct PrintingAnchor {
    scryfall_id: &'static str,
    nonce: u32,
}

impl PrintingAnchor {
    #[must_use]
    pub(super) const fn scryfall(scryfall_id: &'static str) -> Self {
        Self {
            scryfall_id,
            nonce: 0,
        }
    }

    /// Commits the deterministic collision escape for one printing.
    #[must_use]
    #[allow(dead_code)] // Used only when a newly authored printing collides.
    pub(super) const fn scryfall_with_nonce(scryfall_id: &'static str, nonce: u32) -> Self {
        Self { scryfall_id, nonce }
    }
}

/// Strategic meaning used to evaluate a card-owned ability without making its
/// runtime procedure part of the public rules model.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) enum AbilityPolicyHint {
    TargetPlayerSacrificesOneOfTwoPermanentPiles { target: TargetSlotId },
}

/// Internal runtime metadata attached to one printed ability.
///
/// `expected` guards the positional identity: if a card's abilities are
/// reordered without updating its binding, lookup fails instead of dispatching
/// the wrong procedure.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct CardAbilityBinding {
    pub(crate) part: CardPartId,
    pub(crate) ability: AbilityId,
    pub(crate) expected: AbilityDef,
    resolver: &'static CardAbilityResolver,
    policy_hint: Option<AbilityPolicyHint>,
}

impl CardAbilityBinding {
    #[must_use]
    #[allow(clippy::large_types_passed_by_value)]
    pub(crate) const fn new(
        part: CardPartId,
        ability: AbilityId,
        expected: AbilityDef,
        resolver: &'static CardAbilityResolver,
    ) -> Self {
        Self {
            part,
            ability,
            expected,
            resolver,
            policy_hint: None,
        }
    }

    #[must_use]
    pub(crate) const fn with_policy_hint(mut self, hint: AbilityPolicyHint) -> Self {
        self.policy_hint = Some(hint);
        self
    }

    #[must_use]
    pub(crate) const fn resolver(self) -> &'static CardAbilityResolver {
        self.resolver
    }

    #[must_use]
    pub(crate) const fn policy_hint(self) -> Option<AbilityPolicyHint> {
        self.policy_hint
    }
}

/// Internal source record from which the runtime catalog is built.
pub(super) struct CardRecord {
    legacy_id: Option<CardDefinitionId>,
    identity_anchor: PrintingAnchor,
    pub(super) name: &'static str,
    pub(super) art: CardArt,
    pub(super) debut_set: CardSet,
    pub(super) rules: CardRules,
    composition: Option<CompositionBuilder>,
    pub(crate) ability_bindings: &'static [CardAbilityBinding],
}

impl CardRecord {
    /// Defines a card whose ID is derived from its immutable first-printing anchor.
    #[allow(clippy::large_types_passed_by_value)]
    #[allow(dead_code)] // Existing records use the migration-only legacy constructor.
    pub(super) const fn new(
        identity_anchor: PrintingAnchor,
        name: &'static str,
        art: CardArt,
        debut_set: CardSet,
        rules: CardRules,
    ) -> Self {
        Self {
            legacy_id: None,
            identity_anchor,
            name,
            art,
            debut_set,
            rules,
            composition: None,
            ability_bindings: &[],
        }
    }

    /// Preserves an existing numeric ID while recording an anchor printing.
    ///
    /// Existing records default their anchor to the chosen art printing. A
    /// record whose presentation intentionally comes from another printing
    /// can override that with [`Self::with_identity_anchor`].
    #[allow(clippy::large_types_passed_by_value)]
    pub(super) const fn new_with_legacy_id(
        legacy_id: u64,
        name: &'static str,
        art: CardArt,
        debut_set: CardSet,
        rules: CardRules,
    ) -> Self {
        Self {
            legacy_id: Some(CardDefinitionId::new(legacy_id)),
            identity_anchor: PrintingAnchor::scryfall(art.scryfall_id),
            name,
            art,
            debut_set,
            rules,
            composition: None,
            ability_bindings: &[],
        }
    }

    /// Uses an identity printing distinct from the chosen presentation art.
    #[must_use]
    pub(super) const fn with_identity_anchor(mut self, anchor: PrintingAnchor) -> Self {
        self.identity_anchor = anchor;
        self
    }

    #[must_use]
    pub(super) fn id(&self) -> CardDefinitionId {
        self.legacy_id.unwrap_or_else(|| {
            let mut hash = Sha256::new();
            hash.update(b"penta/card-printing-id/v1\0");
            hash.update(self.identity_anchor.scryfall_id.as_bytes());
            hash.update(self.identity_anchor.nonce.to_be_bytes());
            let digest = hash.finalize();
            let prefix = u64::from_be_bytes(
                digest[..8]
                    .try_into()
                    .expect("SHA-256 digest always has an eight-byte prefix"),
            );
            CardDefinitionId::new(prefix >> 12)
        })
    }

    #[must_use]
    #[cfg(test)]
    pub(super) const fn identity_anchor(&self) -> &'static str {
        self.identity_anchor.scryfall_id
    }

    /// Supplies logical parts and play options for a structured or modal card.
    #[must_use]
    pub(super) const fn with_composition(mut self, builder: CompositionBuilder) -> Self {
        self.composition = Some(builder);
        self
    }

    /// Attaches card-owned runtime procedures without changing the public
    /// rules value produced by this record.
    #[must_use]
    pub(crate) const fn with_ability_bindings(
        mut self,
        bindings: &'static [CardAbilityBinding],
    ) -> Self {
        self.ability_bindings = bindings;
        self
    }

    pub(super) fn definition(&self) -> CardDefinition {
        let id = self.id();
        let composition = self.composition.map_or_else(
            || CardComposition::single(self.name, self.rules),
            |builder| builder(),
        );
        CardDefinition {
            id,
            name: self.name.into(),
            art: Some(self.art),
            debut_set: self.debut_set,
            printings: vec![CardPrinting::new(id, self.debut_set)],
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
}

impl PrintingRecord {
    /// Adds the default variant of `card` to another set.
    pub(super) const fn reprint(card: &'static CardRecord) -> Self {
        Self { card, variant: 0 }
    }

    /// Adds another distinguishable printing of `card` within the same set.
    pub(super) const fn alternate(card: &'static CardRecord, variant: u16) -> Self {
        assert!(variant > 0, "alternate printing variants start at one");
        Self { card, variant }
    }

    pub(super) fn printing(&self, set: CardSet) -> CardPrinting {
        CardPrinting::with_variant(self.card.id(), set, self.variant)
    }
}

#[cfg(test)]
mod tests {
    use super::{CardRecord, PrintingAnchor};
    use crate::CardDefinitionId;
    use crate::card::{CardArt, CardRules, CardSet};

    const ANCHOR: &str = "00000000-0000-0000-0000-000000000001";

    fn derived(anchor: PrintingAnchor) -> CardRecord {
        CardRecord::new(
            anchor,
            "Derived identity test",
            CardArt::new(ANCHOR, "Test Artist"),
            CardSet::Alpha,
            CardRules::unsupported(),
        )
    }

    #[test]
    fn printing_ids_follow_the_frozen_sha256_vector() {
        assert_eq!(
            derived(PrintingAnchor::scryfall(ANCHOR)).id(),
            CardDefinitionId::new(4_013_269_539_742_549),
        );
        assert_eq!(
            derived(PrintingAnchor::scryfall_with_nonce(ANCHOR, 1)).id(),
            CardDefinitionId::new(2_624_005_265_348_835),
        );
    }

    #[test]
    fn legacy_ids_do_not_depend_on_the_anchor_printing() {
        let record = CardRecord::new_with_legacy_id(
            587,
            "Legacy identity test",
            CardArt::new(ANCHOR, "Test Artist"),
            CardSet::Alpha,
            CardRules::unsupported(),
        );

        assert_eq!(record.id(), CardDefinitionId::new(587));
        assert_eq!(record.identity_anchor(), ANCHOR);
    }
}
