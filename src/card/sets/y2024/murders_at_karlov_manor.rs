//! Murders at Karlov Manor cards cataloged for the Vintage Cube.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, CardArt, CardRules, CardSet, CardType, EffectDef, EffectRecipientDef,
    ObjectPredicateDef, PlayerRelation, TopCardSelectionDef, TriggerEventDef, ValueDef, ZoneKind,
    ZonePlacement, abilities, tokens,
};
use crate::mana_cost;

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

// MKM 57 — Forensic Gadgeteer
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
                EffectDef::ReduceMatchingAbilityCostBy {
                    permanent: ARTIFACTS_YOU_CONTROL,
                    amount: ValueDef::Constant(1),
                    minimum: 1,
                },
            ),
        ]),
);

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
    selected_zone: ZoneKind::Graveyard,
    selected_placement: ZonePlacement::Top,
    rest_zone: ZoneKind::Library,
    rest_placement: ZonePlacement::Top,
    selected_order_follows_choice: false,
    then: None,
    selected_face_down: None,
};

static SURVEIL_LAND_ABILITIES: [AbilityDef; 2] = [
    abilities::enters_tapped("This land enters tapped."),
    AbilityDef::triggered(
        "When this land enters, surveil 1. (Look at the top card of your library. You may put it \
         into your graveyard.)",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::Source,
            None,
            Some(ZoneKind::Battlefield),
        ),
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

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &FORENSIC_GADGETEER,
    &COMMERCIAL_DISTRICT,
    &LUSH_PORTICO,
    &METICULOUS_ARCHIVE,
    &THUNDERING_FALLS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
