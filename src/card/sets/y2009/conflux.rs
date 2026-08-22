//! Conflux cards cataloged for the Vintage Cube.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, CardArt, CardRules, CardSet, CardSupertype, CardType, EffectDef,
    EffectRecipientDef, ObjectPredicateDef, ObjectRefDef, PlayerRefDef, ValueDef, ZoneKind,
    ZonePlacement,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

/// Any creature, including one of your own: the compensation is what keeps
/// the printed cost at one mana, not a restriction on whom it may hit.
static PATH_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Creature),
)];

static PATH_STEPS: [EffectDef; 2] = [
    EffectDef::MoveToZone {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        zone: ZoneKind::Exile,
        controller: None,
        placement: ZonePlacement::Top,
        arrival_effect: None,
        attachment: None,
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
        binding: None,
        then: None,
    },
];

// CON 15 — Path to Exile
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

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&PATH_TO_EXILE];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
