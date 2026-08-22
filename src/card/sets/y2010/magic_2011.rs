//! Magic 2011 cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, CardArt, CardRules, CardSet, CardType, EffectDef, EffectRecipientDef,
    ObjectPredicateDef, TopCardSelectionDef, TriggerEventDef, ValueDef, ZoneKind, ZonePlacement,
    abilities,
};
use crate::mana_cost;

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
    selected_order_follows_choice: true,
    then: Some(&PREORDAIN_DRAW),
    selected_face_down: None,
};

// M11 70 — Preordain
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

// M11 192 — Primeval Titan
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

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&PREORDAIN, &PRIMEVAL_TITAN];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
