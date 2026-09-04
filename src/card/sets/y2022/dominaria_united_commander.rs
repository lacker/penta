//! Dominaria United Commander card records required by supported formats.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    CardArt, CardRules, CardSet, CardSupertype, CardType, EffectDef, ManaColor, ObjectPredicateDef,
    ValueDef, abilities,
};
use crate::mana_cost;

// DMC 47 — Torsten, Founder of Benalia
pub(in crate::card::sets) static TORSTEN_FOUNDER_OF_BENALIA: CardRecord = CardRecord::new(
    CardSet::DominariaUnitedCommander,
    "Torsten, Founder of Benalia",
    "0783b426-a527-42c1-9271-be28b229e1c6",
    "Volkan Baǵa",
    // Seven mana, and the two halves answer the two ways it goes wrong: it
    // refills your hand the turn it lands, and leaves seven bodies behind if
    // somebody kills it.
    CardRules::new_creature(mana_cost!("{5}{G}{W}"), &["Human", "Soldier"], 7, 7)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger(
                "When Torsten enters, reveal the top seven cards of your library. Put any number of \
                 creature and/or land cards from among them into your hand and the rest on the bottom of \
                 your library in a random order.",
                // "Any number", so the choice is real: a land you would rather not draw
                // later can be left to the bottom, which is the only reason the clause is
                // bounded rather than mandatory. All seven are revealed, and what remains
                // is randomized rather than ordered as a plan for later.
                abilities::reveal_top_cards_choose_to_hand_rest_random_bottom(
                    ValueDef::Constant(7),
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Land),
                    ]),
                    0,
                    7,
                ),
            ),
            abilities::dies_trigger(
                "When Torsten dies, create seven 1/1 white Soldier creature tokens.",
                EffectDef::create_creature_token(&["Soldier"], &[ManaColor::White], 1, 1)
                    .with_count(ValueDef::Constant(7))
                    .with_art(CardArt::new(
                        "8c4b0257-2ca5-4015-9d63-d7cf6e87ab9d",
                        "Justine Cruz",
                    )),
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&TORSTEN_FOUNDER_OF_BENALIA];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
