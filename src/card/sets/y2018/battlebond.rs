//! Battlebond cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, CardArt, CardRules, CardSet, CardType, EffectDef, EffectRecipientDef,
    ObjectPredicateDef, TriggerEventDef, ValueDef, ZoneKind, ZonePlacement,
};
use crate::mana_cost;

/// A cheap instant or sorcery: the body is beside the point, and what it
/// fetches is whichever answer the board is asking for.
static A_CHEAP_SPELL: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::AnyOf(&[
        ObjectPredicateDef::HasType(CardType::Instant),
        ObjectPredicateDef::HasType(CardType::Sorcery),
    ]),
    ObjectPredicateDef::ManaValueAtMost(2),
]);

static SPELLSEEKER_SEARCH: EffectDef = EffectDef::SearchZone {
    player: EffectRecipientDef::Controller,
    source: ZoneKind::Library,
    object: A_CHEAP_SPELL,
    minimum: 0,
    maximum: ValueDef::Constant(1),
    reveal: true,
    destination: ZoneKind::Hand,
    placement: ZonePlacement::Top,
    shuffle: true,
    enters_tapped: false,
    binding: None,
    then: None,
};

// BBD 41 — Spellseeker
pub(in crate::card::sets) static SPELLSEEKER: CardRecord = CardRecord::new_with_legacy_id(
    2150,
    "Spellseeker",
    CardArt::new("74b4c336-5d4c-4bc5-b82a-35084a6ad808", "Igor Kieryluk"),
    CardSet::Battlebond,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Wizard"], 1, 1).with_ability(
        AbilityDef::triggered(
            "When this creature enters, you may search your library for an instant or sorcery card with mana value 2 or less, reveal it, put it into your hand, then shuffle.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &SPELLSEEKER_SEARCH,
            },
        ),
    ),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&SPELLSEEKER];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
