//! Aetherdrift Commander card records required by the cEDH corpus.

use super::{CardRecord, PrintingRecord};
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::CardRules;
use crate::card::CostDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::SubtypeDef;
use crate::card::abilities;
use crate::mana_cost;

pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "DRC",
    slug: "aetherdrift-commander",
});
pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());







// DRC 1 — Hashaton, Scarab's Fist
// Audit: unsupported — Discard triggers have no creature-card predicate using the discard event's last-known characteristics. Testing the live graveyard card misses replacement destinations and discarded activation-cost objects.
pub(in crate::card::sets) static HASHATON_SCARAB_S_FIST_1: CardRecord = CardRecord::new(
    "Hashaton, Scarab's Fist",
    "02645651-cd55-4bd0-8a4d-fa257270a0e0",
    "Wisnu Tan",
    crate::card::CardRules::unsupported(),
);

// DRC 19 — Stridehangar Automaton
// Audit: unsupported — Token-creation replacements can multiply a batch, but cannot append one different Thopter token to an artifact-token batch as part of the same replacement.
pub(in crate::card::sets) static STRIDEHANGAR_AUTOMATON_19: CardRecord = CardRecord::new(
    "Stridehangar Automaton",
    "4c398d11-a2fc-43e2-96e9-7f3383e1a43c",
    "Aaron J. Riley",
    crate::card::CardRules::unsupported(),
);

// DRC 36 — Accursed Duneyard
pub(in crate::card::sets) static ACCURSED_DUNEYARD_36: CardRecord = CardRecord::new(
    "Accursed Duneyard",
    "7f288449-e9a8-444e-9639-fbcb742d2409",
    "Maxime Minard",
    CardRules::new_land(&[]).with_abilities(&[
abilities::tap_for(ManaColor::Colorless),
AbilityDef::activated_with_targets("{2}, {T}: Regenerate target Shade, Skeleton, Specter, Spirit, Vampire, Wraith, or Zombie. (The next time it would be destroyed this turn, instead tap it, remove it from combat, and heal all damage on it.)", &[CostDef::Mana(mana_cost!("{2}")), CostDef::TapSource], &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::Subtype(SubtypeDef::Literal("Shade")), ObjectPredicateDef::Subtype(SubtypeDef::Literal("Skeleton")), ObjectPredicateDef::Subtype(SubtypeDef::Literal("Specter")), ObjectPredicateDef::Subtype(SubtypeDef::Literal("Spirit")), ObjectPredicateDef::Subtype(SubtypeDef::Literal("Vampire")), ObjectPredicateDef::Subtype(SubtypeDef::Literal("Wraith")), ObjectPredicateDef::Subtype(SubtypeDef::Literal("Zombie"))]))], EffectDef::Regenerate { object: EffectRecipientDef::Target(TargetIndex::PRIMARY) })
]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &HASHATON_SCARAB_S_FIST_1,
    &STRIDEHANGAR_AUTOMATON_19,
    &ACCURSED_DUNEYARD_36,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
