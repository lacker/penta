//! Ravnica: City of Guilds cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::abilities;
use crate::card::{
    AbilityDef, CardArt, CardRules, CardSet, EffectDef, EffectRecipientDef, MoveObjectsDef,
    ObjectSetDef, PlayerRefDef, PlayerRelation, RevealObjectsDef, TriggerEventDef, TurnStepDef,
    ValueDef, ZoneKind, ZonePlacement,
};
use crate::{ObjectSetBindingIndex, mana_cost};

// RAV 81 — Dark Confidant
/// "You lose life equal to its mana value." The card is in your hand by the
/// time this is asked, so what the reveal hands on is the number rather than
/// the card.
static CONFIDANT_PAYMENT: EffectDef = EffectDef::LoseLife {
    recipient: EffectRecipientDef::Controller,
    amount: ValueDef::MatchedManaValue,
};

/// One card off the top, shown to everybody, into your hand. Nothing is
/// chosen and nothing may be declined: the minimum and the maximum are both
/// the one card the trigger names.
const CONFIDANT_CARD: ObjectSetBindingIndex = ObjectSetBindingIndex::new(0);
static CONFIDANT_PUT_IN_HAND: EffectDef = EffectDef::MoveObjects(MoveObjectsDef {
    input: ObjectSetDef::Binding(CONFIDANT_CARD),
    from: Some(ZoneKind::Library),
    zone: ZoneKind::Hand,
    placement: ZonePlacement::Top,
    moved: None,
    then: &CONFIDANT_PAYMENT,
});
static CONFIDANT_REVEAL: EffectDef = EffectDef::RevealObjects(RevealObjectsDef {
    input: ObjectSetDef::Binding(CONFIDANT_CARD),
    then: &CONFIDANT_PUT_IN_HAND,
});
static CONFIDANT_EFFECT: EffectDef = abilities::bind_top_cards_then(
    PlayerRefDef::EffectController,
    ValueDef::Constant(1),
    CONFIDANT_CARD,
    &CONFIDANT_REVEAL,
);

pub(in crate::card::sets) static DARK_CONFIDANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("94f7a441-bf2d-46fb-a7b6-9bd6137f86d9"),
    "Dark Confidant",
    CardArt::new("94f7a441-bf2d-46fb-a7b6-9bd6137f86d9", "Ron Spears"),
    CardSet::RavnicaCityOfGuilds,
    // Two mana for an extra card every turn, at whatever the top of your
    // deck happens to cost -- which is why the decks that play him keep
    // their curve low enough to survive him.
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Wizard"], 2, 1).with_ability(
        AbilityDef::triggered(
            "At the beginning of your upkeep, reveal the top card of your library and put that \
             card into your hand. You lose life equal to its mana value.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            CONFIDANT_EFFECT,
        ),
    ),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&DARK_CONFIDANT];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
