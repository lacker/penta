//! Assassin's Creed card records required by the cEDH corpus.

use super::{CardRecord, PrintingRecord};

pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "ACR",
    slug: "assassin-s-creed",
});
pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());























// ACR 8 — Senu, Keen-Eyed Protector
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SENU_KEEN_EYED_PROTECTOR_8: CardRecord = CardRecord::new(
    "Senu, Keen-Eyed Protector",
    "5671a03d-0858-41e7-976c-60825c29af04",
    "Michael MacRae",
    crate::card::CardRules::unsupported(),
);

// ACR 63 — Shao Jun
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHAO_JUN_63: CardRecord = CardRecord::new(
    "Shao Jun",
    "085568e5-d622-45ff-9a31-52eaa513ff31",
    "Stephen Stark",
    crate::card::CardRules::unsupported(),
);

// ACR 70 — Apple of Eden, Isu Relic
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static APPLE_OF_EDEN_ISU_RELIC_70: CardRecord = CardRecord::new(
    "Apple of Eden, Isu Relic",
    "17dd0b7f-bd26-4a46-a7a1-bc65138d54ed",
    "L J Koh",
    crate::card::CardRules::unsupported(),
);

// ACR 72 — Excalibur, Sword of Eden
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXCALIBUR_SWORD_OF_EDEN_72: CardRecord = CardRecord::new(
    "Excalibur, Sword of Eden",
    "26dbf574-3193-413e-a982-4b9d27dafaf5",
    "Thanh Tuấn",
    crate::card::CardRules::unsupported(),
);

// ACR 79 — Abstergo Entertainment
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ABSTERGO_ENTERTAINMENT_79: CardRecord = CardRecord::new(
    "Abstergo Entertainment",
    "4d197866-7633-493c-80dd-ec3a09165934",
    "Alexander Gering",
    crate::card::CardRules::unsupported(),
);

// ACR 134 — Alexios, Deimos of Kosmos
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALEXIOS_DEIMOS_OF_KOSMOS_134: CardRecord = CardRecord::new(
    "Alexios, Deimos of Kosmos",
    "ab907c09-56c0-40ed-aebd-63b64c7e1c2e",
    "Jessie Lam",
    crate::card::CardRules::unsupported(),
);

// ACR 141 — Basim Ibn Ishaq
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BASIM_IBN_ISHAQ_141: CardRecord = CardRecord::new(
    "Basim Ibn Ishaq",
    "0e74cc38-108d-46d4-9d4b-43ed5653982a",
    "Astri Lohne",
    crate::card::CardRules::unsupported(),
);

// ACR 158 — Crystal Skull, Isu Spyglass
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRYSTAL_SKULL_ISU_SPYGLASS_158: CardRecord = CardRecord::new(
    "Crystal Skull, Isu Spyglass",
    "1068ed8a-a062-4249-802d-6f4070992de0",
    "Thanh Tuấn",
    crate::card::CardRules::unsupported(),
);

// ACR 182 — Tax Collector
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TAX_COLLECTOR_182: CardRecord = CardRecord::new(
    "Tax Collector",
    "825615d2-2bb9-486e-8a05-4244b7ba66c3",
    "Miklós Ligeti",
    crate::card::CardRules::unsupported(),
);

// ACR 218 — Overpowering Attack
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OVERPOWERING_ATTACK_218: CardRecord = CardRecord::new(
    "Overpowering Attack",
    "b62be00b-b6cb-47df-aa60-7c77c2f15fd3",
    "Kim Sokol",
    crate::card::CardRules::unsupported(),
);

// ACR 231 — Bayek of Siwa
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BAYEK_OF_SIWA_231: CardRecord = CardRecord::new(
    "Bayek of Siwa",
    "381497ac-653e-4c98-be03-2192d42885f5",
    "JB Casacop",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &SENU_KEEN_EYED_PROTECTOR_8,
    &SHAO_JUN_63,
    &APPLE_OF_EDEN_ISU_RELIC_70,
    &EXCALIBUR_SWORD_OF_EDEN_72,
    &ABSTERGO_ENTERTAINMENT_79,
    &ALEXIOS_DEIMOS_OF_KOSMOS_134,
    &BASIM_IBN_ISHAQ_141,
    &CRYSTAL_SKULL_ISU_SPYGLASS_158,
    &TAX_COLLECTOR_182,
    &OVERPOWERING_ATTACK_218,
    &BAYEK_OF_SIWA_231,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
