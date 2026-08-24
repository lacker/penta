//! Murders at Karlov Manor cards cataloged for the Vintage Cube.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, CardArt, CardRules, CardSet, CardType, CostModificationDef, EffectDef,
    EffectRecipientDef, ObjectPredicateDef, PlayerRelation, TopCardSelectionDef, TriggerEventDef,
    ValueDef, ZoneKind, ZonePlacement, abilities, tokens,
};
use crate::mana_cost;

/// Surveil 1: look at the top card and choose whether to bin it. Nothing is
/// revealed and nothing has to go, so the minimum is zero and the card that
/// stays goes back where it came from.
static SURVEIL_ONE: TopCardSelectionDef = TopCardSelectionDef {
    count: ValueDef::Constant(1),
    object: None,
    minimum: 0,
    maximum: 1,
    select_all_matching: false,
    reveal_selected: false,
    counted: None,
    selected_zone: ZoneKind::Graveyard,
    selected_placement: ZonePlacement::Top,
    rest_zone: ZoneKind::Library,
    rest_placement: ZonePlacement::Top,
    rest_random_order: false,
    rest_counters: None,
    selected_order_follows_choice: false,
    then: None,
    selected_hidden: false,
    selected_linked_to_source: false,
    selected_face_down: None,
};

static SURVEIL_LAND_ABILITIES: [AbilityDef; 2] = [
    abilities::enters_tapped("This land enters tapped."),
    abilities::enters_trigger(
        "When this land enters, surveil 1. (Look at the top card of your library. You may put it \
         into your graveyard.)",
        EffectDef::LookAtTopAndSelect {
            player: EffectRecipientDef::Controller,
            looker: EffectRecipientDef::Controller,
            selection: &SURVEIL_ONE,
        },
    ),
];

/// The surveil-land cycle: two basic types, tapped on the way in, and one
/// look at the top of your library to pay for it. The mana abilities come
/// from the types rather than from a printed clause.
const fn surveil_land(types: &'static [&'static str]) -> CardRules {
    CardRules::new_land(types).with_abilities(&SURVEIL_LAND_ABILITIES)
}

// MKM 29 — Novice Inspector
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NOVICE_INSPECTOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0ad38866-fc5f-4f62-89c1-afc0f50765aa"),
    "Novice Inspector",
    crate::card::CardArt::new("0ad38866-fc5f-4f62-89c1-afc0f50765aa", "Fajareka Setiawan"),
    crate::card::CardSet::MurdersAtKarlovManor,
    crate::card::CardRules::unsupported(),
);

// MKM 57 — Forensic Gadgeteer
/// An artifact spell you cast, which is the whole of the trigger: what it
/// does is not part of the condition.
static AN_ARTIFACT_SPELL_YOU_CAST: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Artifact),
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
]);

static ARTIFACTS_YOU_CONTROL: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Artifact),
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
]);

pub(in crate::card::sets) static FORENSIC_GADGETEER: CardRecord = CardRecord::new_with_legacy_id(
    2206,
    "Forensic Gadgeteer",
    CardArt::new("97d08a15-e61c-4421-a541-c68a4f87cb74", "Volkan Baǵa"),
    CardSet::MurdersAtKarlovManor,
    // Every artifact you cast is a card later, and every artifact you
    // already have is cheaper to use -- including the Clues it just made.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Vedalken", "Artificer", "Detective"], 2, 3)
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever you cast an artifact spell, investigate. (Create a Clue token. It's an artifact with \"{2}, Sacrifice this token: Draw a card.\")",
                TriggerEventDef::SpellCast(AN_ARTIFACT_SPELL_YOU_CAST),
                EffectDef::create_token(tokens::clue()).with_art(CardArt::new(
                    "ef607895-d6d2-44ab-a6b4-84af55fce593",
                    "Daneen Wilkerson",
                )),
            ),
            AbilityDef::static_ability(
                "Activated abilities of artifacts you control cost {1} less to activate. This effect can't reduce the mana in that cost to less than one mana.",
                EffectDef::ModifyCost(CostModificationDef::AbilityReduction {
                    permanent: ARTIFACTS_YOU_CONTROL,
                    amount: ValueDef::Constant(1),
                    minimum: 1,
                }),
            ),
        ]),
);

// MKM 105 — Snarling Gorehound
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SNARLING_GOREHOUND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("93ab3e11-8584-406f-b9ae-9e1df4396cbc"),
    "Snarling Gorehound",
    crate::card::CardArt::new("93ab3e11-8584-406f-b9ae-9e1df4396cbc", "John Tedrick"),
    crate::card::CardSet::MurdersAtKarlovManor,
    crate::card::CardRules::unsupported(),
);

// MKM 174 — Rubblebelt Maverick
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RUBBLEBELT_MAVERICK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("81c7ff67-b9e1-4d2e-b1ae-da9b946da00b"),
    "Rubblebelt Maverick",
    crate::card::CardArt::new("81c7ff67-b9e1-4d2e-b1ae-da9b946da00b", "Carissa Susilo"),
    crate::card::CardSet::MurdersAtKarlovManor,
    crate::card::CardRules::unsupported(),
);

// MKM 197 — Dog Walker
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DOG_WALKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a6e0adb7-a030-4dcc-9284-cd91c7598a22"),
    "Dog Walker",
    crate::card::CardArt::new("a6e0adb7-a030-4dcc-9284-cd91c7598a22", "Milivoj Ćeran"),
    crate::card::CardSet::MurdersAtKarlovManor,
    crate::card::CardRules::unsupported(),
);

// MKM 221 — No More Lies
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NO_MORE_LIES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1e0c695d-62f9-4805-9e2f-7032e8464136"),
    "No More Lies",
    crate::card::CardArt::new("1e0c695d-62f9-4805-9e2f-7032e8464136", "Liiga Smilshkalne"),
    crate::card::CardSet::MurdersAtKarlovManor,
    crate::card::CardRules::unsupported(),
);

// MKM 259 — Commercial District
pub(in crate::card::sets) static COMMERCIAL_DISTRICT: CardRecord = CardRecord::new_with_legacy_id(
    2275,
    "Commercial District",
    CardArt::new(
        "bf220c06-3cce-4bdd-aa58-83940c223e9c",
        "Julian Kok Joon Wen",
    ),
    CardSet::MurdersAtKarlovManor,
    // The red-green half, which wants the graveyard less than the others and
    // plays it anyway because a tapped dual is what the mana costs.
    surveil_land(&["Mountain", "Forest"]),
);

// MKM 261 — Escape Tunnel
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ESCAPE_TUNNEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("93ddde4f-d35e-4128-8f43-d0eadbd715de"),
    "Escape Tunnel",
    crate::card::CardArt::new(
        "93ddde4f-d35e-4128-8f43-d0eadbd715de",
        "Carlos Palma Cruchaga",
    ),
    crate::card::CardSet::MurdersAtKarlovManor,
    crate::card::CardRules::unsupported(),
);

// MKM 262 — Hedge Maze
pub(in crate::card::sets) static HEDGE_MAZE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5260f8ae-805b-4eae-badf-62de0f768867"),
    "Hedge Maze",
    CardArt::new("5260f8ae-805b-4eae-badf-62de0f768867", "Andrew Mar"),
    CardSet::MurdersAtKarlovManor,
    // The green-blue half of the cycle, and the one whose deck is usually
    // happiest to see the surveil: the graveyard is where half its cards
    // want to be anyway.
    surveil_land(&["Forest", "Island"]),
);

// MKM 263 — Lush Portico
pub(in crate::card::sets) static LUSH_PORTICO: CardRecord = CardRecord::new_with_legacy_id(
    2248,
    "Lush Portico",
    CardArt::new("c17816e8-28b1-4295-a637-efb0e5c18873", "Kamila Szutenberg"),
    CardSet::MurdersAtKarlovManor,
    // The green-white half of the cycle, which the decks that want it are
    // playing for the fixing rather than for the graveyard.
    surveil_land(&["Forest", "Plains"]),
);

// MKM 264 — Meticulous Archive
pub(in crate::card::sets) static METICULOUS_ARCHIVE: CardRecord = CardRecord::new_with_legacy_id(
    2303,
    "Meticulous Archive",
    CardArt::new("652236c2-84ef-45e4-b5fc-ed6170bc3d6c", "Sam Burley"),
    CardSet::MurdersAtKarlovManor,
    // The white-blue half, which wants the graveyard least of the cycle and
    // is played for the dual land the tempo decks cannot otherwise have.
    surveil_land(&["Plains", "Island"]),
);

// MKM 269 — Thundering Falls
pub(in crate::card::sets) static THUNDERING_FALLS: CardRecord = CardRecord::new_with_legacy_id(
    2226,
    "Thundering Falls",
    CardArt::new("17260fff-b239-4af4-9306-3236ae3fa5a5", "Grady Frederick"),
    CardSet::MurdersAtKarlovManor,
    // A dual that costs you the turn it lands and pays a little of it back by
    // filling the graveyard the decks that want it are built around.
    surveil_land(&["Island", "Mountain"]),
);

// MKM 270 — Undercity Sewers
pub(in crate::card::sets) static UNDERCITY_SEWERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2b5801fb-2026-4f25-98bc-ebb2f99684b9"),
    "Undercity Sewers",
    CardArt::new("2b5801fb-2026-4f25-98bc-ebb2f99684b9", "Yeong-Hao Han"),
    CardSet::MurdersAtKarlovManor,
    // The blue-black half, and the one the cycle was designed for: the deck
    // playing it is already trying to fill a graveyard, so the look costs it
    // nothing it was not going to spend.
    surveil_land(&["Island", "Swamp"]),
);

// MKM 329 — Raucous Theater
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RAUCOUS_THEATER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2faf0337-c7a3-45a0-bb14-c431526da2cd"),
    "Raucous Theater",
    crate::card::CardArt::new("2faf0337-c7a3-45a0-bb14-c431526da2cd", "Sergey Glushakov"),
    crate::card::CardSet::MurdersAtKarlovManor,
    crate::card::CardRules::unsupported(),
);

// MKM 330 — Shadowy Backstreet
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHADOWY_BACKSTREET: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("27eae4ce-e0b3-482b-9136-6fc17333877e"),
    "Shadowy Backstreet",
    crate::card::CardArt::new("27eae4ce-e0b3-482b-9136-6fc17333877e", "Sergey Glushakov"),
    crate::card::CardSet::MurdersAtKarlovManor,
    crate::card::CardRules::unsupported(),
);

// MKM 333 — Underground Mortuary
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static UNDERGROUND_MORTUARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0d8938e4-bfa5-47e1-8c71-9c6583346300"),
    "Underground Mortuary",
    crate::card::CardArt::new("0d8938e4-bfa5-47e1-8c71-9c6583346300", "Sergey Glushakov"),
    crate::card::CardSet::MurdersAtKarlovManor,
    crate::card::CardRules::unsupported(),
);

// MKM 396 — Proft's Eidetic Memory
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PROFT_S_EIDETIC_MEMORY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a3472756-0305-4567-b425-f7dbf9b3cc7f"),
    "Proft's Eidetic Memory",
    crate::card::CardArt::new("a3472756-0305-4567-b425-f7dbf9b3cc7f", "Julie Dillon"),
    crate::card::CardSet::MurdersAtKarlovManor,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &NOVICE_INSPECTOR,
    &FORENSIC_GADGETEER,
    &SNARLING_GOREHOUND,
    &RUBBLEBELT_MAVERICK,
    &DOG_WALKER,
    &NO_MORE_LIES,
    &COMMERCIAL_DISTRICT,
    &ESCAPE_TUNNEL,
    &HEDGE_MAZE,
    &LUSH_PORTICO,
    &METICULOUS_ARCHIVE,
    &THUNDERING_FALLS,
    &UNDERCITY_SEWERS,
    &RAUCOUS_THEATER,
    &SHADOWY_BACKSTREET,
    &UNDERGROUND_MORTUARY,
    &PROFT_S_EIDETIC_MEMORY,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
