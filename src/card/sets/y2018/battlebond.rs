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
pub const SET: crate::card::CardSet = crate::card::CardSet::new("BBD", "battlebond");

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

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

// BBD 71 — Grothama, All-Devouring
// Audit: unsupported — Needs per-recipient damage history grouped by each source's controller.
pub(in crate::card::sets) static GROTHAMA_ALL_DEVOURING: CardRecord = CardRecord::new(
    "Grothama, All-Devouring",
    "ab8935b1-ec87-4330-9952-9ef8cd344531",
    "Mark Behm",
    CardRules::unsupported(),
);

// BBD 209 — Pulse of Murasa (reprint)
const PULSE_OF_MURASA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2016::oath_of_the_gatewatch::PULSE_OF_MURASA,
    "c591c615-69e8-4661-a089-8c4e152adac7",
    "Matt Stewart",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&SPELLSEEKER, &GROTHAMA_ALL_DEVOURING];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] =
    &[PULSE_OF_MURASA_REPRINT];
