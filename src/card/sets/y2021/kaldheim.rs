//! Kaldheim cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AppliedEffectDef, CardArt, CardRules, CardSet, CardSupertype,
    CardType, EffectDef, EffectRecipientDef, ObjectPredicateDef, PlayerRelation, TriggerEventDef,
    ValueDef, ZoneKind, ZonePlacement, tokens,
};
use crate::mana_cost;

/// "Other Dwarves you control": Magda pumps the rest of the Dwarves and not
/// herself, which is the whole reason she is a 2/1 rather than a 3/1.
static OTHER_DWARVES_YOU_CONTROL: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::Subtype("Dwarf"),
    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
]);

/// Any Dwarf you control becoming tapped, not just an attack: tapping one
/// for mana or to pay a cost makes a Treasure just the same.
static A_DWARF_YOU_CONTROL: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::Subtype("Dwarf"),
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
]);

static AN_ARTIFACT_OR_DRAGON_CARD: ObjectPredicateDef = ObjectPredicateDef::AnyOf(&[
    ObjectPredicateDef::HasType(CardType::Artifact),
    ObjectPredicateDef::Subtype("Dragon"),
]);

static FIVE_TREASURES: [AbilityCostDef; 1] = [AbilityCostDef::SacrificePermanents {
    object: ObjectPredicateDef::Subtype("Treasure"),
    controller: PlayerRelation::You,
    count: 5,
}];

static MAGDA_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::static_ability(
        "Other Dwarves you control get +1/+0.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                OTHER_DWARVES_YOU_CONTROL,
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(1),
                ValueDef::Constant(0),
            ),
        },
    ),
    AbilityDef::triggered(
        "Whenever a Dwarf you control becomes tapped, create a Treasure token.",
        TriggerEventDef::tapped(A_DWARF_YOU_CONTROL),
        EffectDef::create_token(tokens::treasure()).with_art(CardArt::new(
            "4ae9f454-4f8c-4123-9886-674bc439dfe7",
            "Olena Richards",
        )),
    ),
    AbilityDef::activated(
        "Sacrifice five Treasures: Search your library for an artifact or Dragon card, put that \
         card onto the battlefield, then shuffle.",
        &FIVE_TREASURES,
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: AN_ARTIFACT_OR_DRAGON_CARD,
            minimum: 0,
            maximum: ValueDef::Constant(1),
            reveal: false,
            destination: ZoneKind::Battlefield,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: false,
            binding: None,
            then: None,
        },
    ),
];

// KHM 142 — Magda, Brazen Outlaw
pub(in crate::card::sets) static MAGDA_BRAZEN_OUTLAW: CardRecord = CardRecord::new_with_legacy_id(
    2298,
    "Magda, Brazen Outlaw",
    CardArt::new("079e6263-e54c-4899-a336-5315909b9322", "Slawomir Maniak"),
    CardSet::Kaldheim,
    // Two mana that turns every tap into a Treasure, and five Treasures into
    // whatever artifact the deck is built around.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Dwarf", "Berserker"], 2, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&MAGDA_ABILITIES),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&MAGDA_BRAZEN_OUTLAW];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
