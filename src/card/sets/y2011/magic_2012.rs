//! Magic 2012 cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, CardArt, CardRules, CardSet, CardType, CardTypeSet, EffectDef, EffectRecipientDef,
    ObjectPredicateDef, ReplacementEffectDef, TriggerEventDef,
};
use crate::ids::AbilityId;
use crate::mana_cost;

/// The copy keeps the Image's own subtype line and its own second ability:
/// the card is printed as an Illusion and prints the sacrifice clause, so
/// "except it's an Illusion in addition to its other types and it has ..."
/// names nothing the card does not already say.
static PHANTASMAL_IMAGE_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::replacement(
        "You may have this creature enter as a copy of any creature on the battlefield, except \
         it's an Illusion in addition to its other types and it has \"When this creature becomes \
         the target of a spell or ability, sacrifice it.\"",
        ReplacementEffectDef::CopyEntering {
            object: ObjectPredicateDef::HasType(CardType::Creature),
            added_types: CardTypeSet::empty(),
            retain_printed_subtypes: true,
            retained_abilities: &[AbilityId(1)],
        },
    ),
    // Printed on the Image rather than granted by the copy, which is what
    // lets the copy hand it back: an Image that enters as itself is a 0/0
    // and dies before this matters.
    AbilityDef::triggered(
        "When this creature becomes the target of a spell or ability, sacrifice it.",
        // The predicate reads the spell or ability doing the pointing, not
        // the permanent being pointed at: anything at all sets this off.
        TriggerEventDef::BecomesTargetOfSpellOrAbility(ObjectPredicateDef::Any),
        EffectDef::Sacrifice {
            object: EffectRecipientDef::Source,
        },
    ),
];

// M12 72 — Phantasmal Image
pub(in crate::card::sets) static PHANTASMAL_IMAGE: CardRecord = CardRecord::new_with_legacy_id(
    2276,
    "Phantasmal Image",
    CardArt::new("98e7bf8f-dba7-4005-8cee-634c9153931d", "Nils Hamm"),
    CardSet::Magic2012,
    // Two mana for the best creature on the board, which the cube is happy to
    // pay because the drawback only matters to a deck holding removal.
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Illusion"], 0, 0)
        .with_abilities(&PHANTASMAL_IMAGE_ABILITIES),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&PHANTASMAL_IMAGE];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
