//! Conflux cards cataloged for the Vintage Cube.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AddManaEffectDef, CardArt, CardRules, CardSet,
    CardSupertype, CardType, EffectDef, EffectRecipientDef, ManaColor, ObjectPredicateDef,
    ObjectRefDef, PlayerRefDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

// CON 15 — Path to Exile
/// Any creature, including one of your own: the compensation is what keeps
/// the printed cost at one mana, not a restriction on whom it may hit.
static PATH_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Creature),
)];

static PATH_STEPS: [EffectDef; 2] = [
    EffectDef::MoveToZone {
        counters: None,
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        from: None,
        zone: ZoneKind::Exile,
        controller: None,
        placement: ZonePlacement::Top,
        arrival_effect: None,
        attachment: None,
        tapped: false,
    },
    // The searcher is the creature's controller, read from the announced
    // target: by now the creature is in exile and cannot be asked. A minimum
    // of zero is the printed "may" -- declining to search and searching
    // without finding are the same answer from a hidden zone.
    EffectDef::SearchZone {
        player: EffectRecipientDef::player(PlayerRefDef::ControllerOf(ObjectRefDef::Target(
            TargetIndex::PRIMARY,
        ))),
        source: ZoneKind::Library,
        object: ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Land),
            ObjectPredicateDef::Supertype(CardSupertype::Basic),
        ]),
        minimum: 0,
        maximum: ValueDef::Constant(1),
        reveal: false,
        destination: ZoneKind::Battlefield,
        placement: ZonePlacement::Top,
        shuffle: true,
        enters_tapped: true,
        attachment: None,
        binding: None,
        then: None,
    },
];

pub(in crate::card::sets) static PATH_TO_EXILE: CardRecord = CardRecord::new_with_legacy_id(
    2189,
    "Path to Exile",
    CardArt::new("29b7a8b1-b98e-483a-87a4-73bd831c03d4", "Todd Lockwood"),
    CardSet::Conflux,
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell_with_targets(
        "Exile target creature. Its controller may search their library for a basic land card, put that card onto the battlefield tapped, then shuffle.",
        &PATH_TARGET,
        EffectDef::Sequence(&PATH_STEPS),
    )),
);

// CON 87 — Noble Hierarch
static HIERARCH_MANA_COST: [AbilityCostDef; 1] = [AbilityCostDef::TapSource];

static NOBLE_HIERARCH_ABILITIES: [AbilityDef; 2] = [
    abilities::exalted(),
    AbilityDef::activated_mana(
        "{T}: Add {G}, {W}, or {U}.",
        &HIERARCH_MANA_COST,
        EffectDef::AddMana(AddManaEffectDef::choice(&[
            ManaColor::Green,
            ManaColor::White,
            ManaColor::Blue,
        ])),
    ),
];

pub(in crate::card::sets) static NOBLE_HIERARCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6adfe928-1305-444d-b709-1e714544daaf"),
    "Noble Hierarch",
    CardArt::new("6adfe928-1305-444d-b709-1e714544daaf", "Mark Zug"),
    CardSet::Conflux,
    // A one-mana accelerant for three colours whose body is beside the
    // point, except that exalted makes the 0/1 into a real attacker's
    // dividend on any turn nothing else attacks.
    CardRules::new_creature(mana_cost!("{G}"), &["Human", "Druid"], 0, 1)
        .with_abilities(&NOBLE_HIERARCH_ABILITIES),
);

// CON 113 — Knight of the Reliquary
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KNIGHT_OF_THE_RELIQUARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ad8b8518-c09e-4cb7-95b2-08e4e370d89c"),
    "Knight of the Reliquary",
    crate::card::CardArt::new("ad8b8518-c09e-4cb7-95b2-08e4e370d89c", "Michael Komarck"),
    crate::card::CardSet::Conflux,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&PATH_TO_EXILE, &NOBLE_HIERARCH, &KNIGHT_OF_THE_RELIQUARY];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
