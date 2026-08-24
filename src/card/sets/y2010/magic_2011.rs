//! Magic 2011 cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, CardArt, CardRules, CardSet, CardType, DiscardSelectionDef, EffectDef,
    EffectRecipientDef, ObjectPredicateDef, TopCardSelectionDef, TriggerEventDef, ValueDef,
    ZoneKind, ZonePlacement, abilities,
};
use crate::mana_cost;

// M11 30 — Silence
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SILENCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1559d660-8a9d-422b-95d3-710a046583dd"),
    "Silence",
    crate::card::CardArt::new("37b70d17-e4ec-4731-8892-b444f82be7a2", "Wayne Reynolds"),
    crate::card::CardSet::Magic2011,
    crate::card::CardRules::unsupported(),
);

// M11 66 — Merfolk Spy
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MERFOLK_SPY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b5ae05cc-116b-4268-ba78-709aeff36ab1"),
    "Merfolk Spy",
    crate::card::CardArt::new(
        "b5ae05cc-116b-4268-ba78-709aeff36ab1",
        "Matt Cavotta & Richard Whitters",
    ),
    crate::card::CardSet::Magic2011,
    crate::card::CardRules::unsupported(),
);

// M11 70 — Preordain
static PREORDAIN_DRAW: EffectDef = EffectDef::DrawCards {
    recipient: EffectRecipientDef::Controller,
    amount: ValueDef::Constant(1),
};

/// The reminder text picks out the cards going to the bottom; this picks out
/// the ones staying on top, which is the same partition read from the other
/// end. Selecting the kept half is what makes "the rest on top in any order"
/// a decision: with two cards kept, the order they were chosen in is the
/// order they go back. What falls to the bottom has no observable order.
static PREORDAIN_SCRY: TopCardSelectionDef = TopCardSelectionDef {
    count: ValueDef::Constant(2),
    object: None,
    minimum: 0,
    maximum: 2,
    select_all_matching: false,
    reveal_selected: false,
    selected_zone: ZoneKind::Library,
    selected_placement: ZonePlacement::Top,
    rest_zone: ZoneKind::Library,
    rest_placement: ZonePlacement::Bottom,
    rest_random_order: false,
    rest_counters: None,
    selected_order_follows_choice: true,
    then: Some(&PREORDAIN_DRAW),
    selected_hidden: false,
    selected_linked_to_source: false,
    selected_face_down: None,
};

pub(in crate::card::sets) static PREORDAIN: CardRecord = CardRecord::new_with_legacy_id(
    2130,
    "Preordain",
    CardArt::new("e3868c3d-4fcd-444b-866f-0f8e50ce7b67", "Svetlin Velinov"),
    CardSet::Magic2011,
    CardRules::new_sorcery(mana_cost!("{U}")).with_ability(AbilityDef::spell(
        "Scry 2, then draw a card.",
        EffectDef::LookAtTopAndSelect {
            player: EffectRecipientDef::Controller,
            looker: EffectRecipientDef::Controller,
            selection: &PREORDAIN_SCRY,
        },
    )),
);

// M11 74 — Stormtide Leviathan
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STORMTIDE_LEVIATHAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0e7f3fb6-93ce-4bc9-8efd-11af5a46218f"),
    "Stormtide Leviathan",
    crate::card::CardArt::new("0e7f3fb6-93ce-4bc9-8efd-11af5a46218f", "Karl Kopinski"),
    crate::card::CardSet::Magic2011,
    crate::card::CardRules::unsupported(),
);

// M11 104 — Liliana's Specter
pub(in crate::card::sets) static LILIANA_S_SPECTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("33122581-39fd-44a0-b928-f73e39a0c0f1"),
    "Liliana's Specter",
    crate::card::CardArt::new("33122581-39fd-44a0-b928-f73e39a0c0f1", "Vance Kovacs"),
    crate::card::CardSet::Magic2011,
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Specter"], 2, 1).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, each opponent discards a card.",
            EffectDef::Discard {
                recipient: EffectRecipientDef::Opponent,
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ),
    ]),
);

// M11 110 — Phylactery Lich
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PHYLACTERY_LICH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9d088983-92c1-4f4d-8abf-dd20347495b5"),
    "Phylactery Lich",
    crate::card::CardArt::new("9d088983-92c1-4f4d-8abf-dd20347495b5", "Michael Komarck"),
    crate::card::CardSet::Magic2011,
    crate::card::CardRules::unsupported(),
);

// M11 177 — Garruk's Packleader
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GARRUK_S_PACKLEADER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dfaef299-7879-4f52-8ee4-701ed150b930"),
    "Garruk's Packleader",
    crate::card::CardArt::new("dfaef299-7879-4f52-8ee4-701ed150b930", "Nils Hamm"),
    crate::card::CardSet::Magic2011,
    crate::card::CardRules::unsupported(),
);

// M11 192 — Primeval Titan
/// One printed ability with two ways in, not two abilities: the card says
/// "enters or attacks", and a Titan that does both in a turn triggers twice
/// for the same reason it would have anyway.
static ENTERS_OR_ATTACKS: [TriggerEventDef; 2] = [
    TriggerEventDef::zone_changed(
        ObjectPredicateDef::Source,
        None,
        Some(ZoneKind::Battlefield),
    ),
    TriggerEventDef::attacks(ObjectPredicateDef::Source),
];

/// Any land card, not just a basic: the two it finds are usually the two the
/// deck was built around.
static FETCH_TWO_LANDS: EffectDef = EffectDef::SearchZone {
    player: EffectRecipientDef::Controller,
    source: ZoneKind::Library,
    object: ObjectPredicateDef::HasType(CardType::Land),
    minimum: 0,
    maximum: ValueDef::Constant(2),
    reveal: false,
    destination: ZoneKind::Battlefield,
    placement: ZonePlacement::Top,
    shuffle: true,
    enters_tapped: true,
    binding: None,
    then: None,
};

pub(in crate::card::sets) static PRIMEVAL_TITAN: CardRecord = CardRecord::new_with_legacy_id(
    2128,
    "Primeval Titan",
    CardArt::new("feee9327-b937-46ba-a2aa-6c015ab6cdd5", "Aleksi Briclot"),
    CardSet::Magic2011,
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Giant"], 6, 6).with_abilities(&[
        abilities::trample(),
        AbilityDef::triggered(
            "Whenever this creature enters or attacks, you may search your library for up to two land put them onto the battlefield tapped, then shuffle.",
            TriggerEventDef::AnyOf(&ENTERS_OR_ATTACKS),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &FETCH_TWO_LANDS,
            },
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &SILENCE,
    &MERFOLK_SPY,
    &PREORDAIN,
    &STORMTIDE_LEVIATHAN,
    &LILIANA_S_SPECTER,
    &PHYLACTERY_LICH,
    &GARRUK_S_PACKLEADER,
    &PRIMEVAL_TITAN,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
