//! Battlebond cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ObjectPredicateDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "BBD",
    slug: "battlebond",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// BBD 15 — Chakram Retriever
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHAKRAM_RETRIEVER_15: CardRecord = CardRecord::new(
    "Chakram Retriever",
    "b57d518c-21bb-4451-8248-9ee838460c09",
    "Dmitry Burmak",
    crate::card::CardRules::unsupported(),
);

// BBD 41 — Spellseeker
pub(in crate::card::sets) static SPELLSEEKER: CardRecord = CardRecord::new(
    "Spellseeker",
    "74b4c336-5d4c-4bc5-b82a-35084a6ad808",
    "Igor Kieryluk",
CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Wizard"], 1, 1).with_ability(
        abilities::enters_trigger("When this creature enters, you may search your library for an instant or sorcery card with mana value 2 or less, reveal it, put it into your hand, then shuffle.", EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    // A cheap instant or sorcery: the body is beside the point, and what it
                    // fetches is whichever answer the board is asking for.
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Instant),
                            ObjectPredicateDef::HasType(CardType::Sorcery),
                        ]),
                        ObjectPredicateDef::ManaValueAtMost(2),
                    ]),
                    minimum: 0,
                    maximum: ValueDef::Constant(1),
                    reveal: true,
                    destination: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: false,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            }),
    ),
);

// BBD 56 — Bonus Round
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BONUS_ROUND_56: CardRecord = CardRecord::new(
    "Bonus Round",
    "e21aa266-4264-4e02-9403-02930e641573",
    "Lake Hurwitz",
    crate::card::CardRules::unsupported(),
);

// BBD 62 — Najeela, the Blade-Blossom
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NAJEELA_THE_BLADE_BLOSSOM_62: CardRecord = CardRecord::new(
    "Najeela, the Blade-Blossom",
    "2cb1d1da-6077-46b5-8c63-39882b8016f2",
    "Matt Stewart",
    crate::card::CardRules::unsupported(),
);

// BBD 71 — Grothama, All-Devouring
// Audit: unsupported — Needs per-recipient damage history grouped by each source's controller.
pub(in crate::card::sets) static GROTHAMA_ALL_DEVOURING: CardRecord = CardRecord::new(
    "Grothama, All-Devouring",
    "ab8935b1-ec87-4330-9952-9ef8cd344531",
    "Mark Behm",
    CardRules::unsupported(),
);

// BBD 79 — Sentinel Tower
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SENTINEL_TOWER_79: CardRecord = CardRecord::new(
    "Sentinel Tower",
    "25d9500e-a536-4f8e-bfdd-6cb08f709890",
    "Jung Park",
    crate::card::CardRules::unsupported(),
);

// BBD 81 — Bountiful Promenade
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOUNTIFUL_PROMENADE_81: CardRecord = CardRecord::new(
    "Bountiful Promenade",
    "21865ed6-5edd-41f4-9ae0-f501872d91dc",
    "Jung Park",
    crate::card::CardRules::unsupported(),
);

// BBD 82 — Luxury Suite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LUXURY_SUITE_82: CardRecord = CardRecord::new(
    "Luxury Suite",
    "81298b0b-9d47-4777-998e-0c17821ef536",
    "Jonas De Ro",
    crate::card::CardRules::unsupported(),
);

// BBD 83 — Morphic Pool
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MORPHIC_POOL_83: CardRecord = CardRecord::new(
    "Morphic Pool",
    "63b9efd6-0709-4d4b-a907-d8a77ec1a327",
    "Grzegorz Rutkowski",
    crate::card::CardRules::unsupported(),
);

// BBD 84 — Sea of Clouds
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEA_OF_CLOUDS_84: CardRecord = CardRecord::new(
    "Sea of Clouds",
    "080aebe8-535e-4632-b733-8aba98abff22",
    "Florian de Gesincourt",
    crate::card::CardRules::unsupported(),
);

// BBD 85 — Spire Garden
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIRE_GARDEN_85: CardRecord = CardRecord::new(
    "Spire Garden",
    "64943615-7543-4acd-a884-22ece8f0ed3e",
    "Darek Zabrocki",
    crate::card::CardRules::unsupported(),
);

// BBD 209 — Pulse of Murasa (reprint)
const PULSE_OF_MURASA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2016::oath_of_the_gatewatch::PULSE_OF_MURASA,
    "c591c615-69e8-4661-a089-8c4e152adac7",
    "Matt Stewart",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &CHAKRAM_RETRIEVER_15,
    &SPELLSEEKER,
    &BONUS_ROUND_56,
    &NAJEELA_THE_BLADE_BLOSSOM_62,
    &GROTHAMA_ALL_DEVOURING,
    &SENTINEL_TOWER_79,
    &BOUNTIFUL_PROMENADE_81,
    &LUXURY_SUITE_82,
    &MORPHIC_POOL_83,
    &SEA_OF_CLOUDS_84,
    &SPIRE_GARDEN_85,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] =
    &[PULSE_OF_MURASA_REPRINT];
