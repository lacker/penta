//! Innistrad: Crimson Vow Commander cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, CardArt, CardRules, CardSet, DiscardFollowUpDef, DiscardSelectionDef, EffectDef,
    EffectRecipientDef, ManaColor, ObjectPredicateDef, ValueDef, abilities, cards,
};
use crate::mana_cost;

/// A Spirit for every card type the discard turned up. Every discarded card
/// is counted, so the predicate is anything at all; what the value counts is
/// the types between them rather than the cards.
static SPIRITS_FOR_THE_TYPES: EffectDef =
    EffectDef::create_creature_token(&["Spirit"], &[ManaColor::White], 1, 1)
        .with_abilities(&[abilities::flying()])
        .with_art(CardArt::new(
            "6bee4081-5d74-4cc2-ba2f-887bc8799513",
            "Kim Sokol",
        ))
        .with_count(ValueDef::MatchedCardTypes);

static EPIPHANY_DISCARD: DiscardFollowUpDef = DiscardFollowUpDef {
    counted: ObjectPredicateDef::Any,
    effect: &SPIRITS_FOR_THE_TYPES,
};

static OCCULT_EPIPHANY_EFFECT: [EffectDef; 2] = [
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::ChosenX,
    },
    EffectDef::Discard {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::ChosenX,
        selection: DiscardSelectionDef::RecipientChooses,
        then: Some(EPIPHANY_DISCARD),
    },
];

// VOC 14 — Occult Epiphany
pub(in crate::card::sets) static OCCULT_EPIPHANY: CardRecord = CardRecord::new(
    cards::OCCULT_EPIPHANY,
    "Occult Epiphany",
    CardArt::new("6920c895-bc98-4871-a53f-219fa27a74e5", "Jason Rainville"),
    CardSet::InnistradCrimsonVowCommander,
    // The draw is a wash and the Spirits are the card: a hand with five
    // types in it turns X of nothing into five fliers.
    CardRules::new_instant(mana_cost!("{X}{U}")).with_ability(AbilityDef::spell(
        "Draw X cards, then discard X cards. Create a 1/1 white Spirit creature token with \
             flying for each card type among cards discarded this way.",
        EffectDef::Sequence(&OCCULT_EPIPHANY_EFFECT),
    )),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&OCCULT_EPIPHANY];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
