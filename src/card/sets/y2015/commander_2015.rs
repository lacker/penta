//! Commander 2015 cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, CardArt, CardRules, CardSet, CardType, EffectDef,
    EffectRecipientDef, ObjectPredicateDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

static A_SPELL: [AbilityTargetDef; 1] =
    [AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::Any)];

static A_CREATURE: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Creature),
)];

/// Each mode declares its own target slot, and a mode chosen twice gets two
/// of them -- which is what "you may choose the same mode more than once"
/// means for a clause that targets.
static MYSTIC_CONFLUENCE_MODES: [AbilityDef; 3] = [
    AbilityDef::spell_with_targets(
        "Counter target spell unless its controller pays {3}.",
        &A_SPELL,
        abilities::counter_target_unless_paid(ValueDef::Constant(3)),
    ),
    AbilityDef::spell_with_targets(
        "Return target creature to its owner's hand.",
        &A_CREATURE,
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            controller: None,
            arrival_effect: None,
            attachment: None,
        },
    ),
    AbilityDef::spell(
        "Draw a card.",
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    ),
];

// C15 14 — Mystic Confluence
pub(in crate::card::sets) static MYSTIC_CONFLUENCE: CardRecord = CardRecord::new_with_legacy_id(
    2229,
    "Mystic Confluence",
    CardArt::new("81bbffc2-6f58-4baa-8f95-168eab106b15", "Kieran Yanner"),
    CardSet::Commander2015,
    // Five mana that is never dead: three cards when nothing is happening, a
    // hard counter plus a card when something is.
    CardRules::new_instant(mana_cost!("{3}{U}{U}")).with_ability(AbilityDef::modal_spell(
        "Choose three. You may choose the same mode more than once.\n• Counter target spell \
         unless its controller pays {3}.\n• Return target creature to its owner's hand.\n• Draw \
         a card.",
        &MYSTIC_CONFLUENCE_MODES,
        3,
        3,
        true,
    )),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&MYSTIC_CONFLUENCE];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
