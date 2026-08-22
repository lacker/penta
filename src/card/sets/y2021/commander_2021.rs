//! Commander 2021 cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, CardArt, CardRules, CardSet, CardSupertype, CounterKind, EffectDef,
    EffectRecipientDef, ObjectPredicateDef, PlayerRelation, TriggerEventDef, ValueDef, ZoneKind,
    abilities,
};
use crate::mana_cost;

/// "From your library and/or your graveyard": one clause naming two zones,
/// so either answers it and a move that takes cards from both is still one
/// trigger.
static YOUR_LIBRARY_OR_GRAVEYARD: [ZoneKind; 2] = [ZoneKind::Library, ZoneKind::Graveyard];

static LAELIA_ABILITIES: [AbilityDef; 3] = [
    abilities::haste(),
    AbilityDef::triggered(
        "Whenever Laelia attacks, exile the top card of your library. You may play that card this \
         turn.",
        TriggerEventDef::attacks(ObjectPredicateDef::Source),
        EffectDef::ExileTopOfLibraryToPlay {
            player: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    ),
    // One counter for the move rather than one per card, which is what "one
    // or more" means: her own attack trigger gives one, and a Breach exiling
    // three still gives one.
    AbilityDef::triggered(
        "Whenever one or more cards are put into exile from your library and/or your graveyard, \
         put a +1/+1 counter on Laelia.",
        TriggerEventDef::CardsExiled {
            zones: &YOUR_LIBRARY_OR_GRAVEYARD,
            owner: PlayerRelation::You,
        },
        EffectDef::AddCounters {
            object: EffectRecipientDef::Source,
            kind: CounterKind::PlusOnePlusOne,
            amount: ValueDef::Constant(1),
        },
    ),
];

// C21 53 — Laelia, the Blade Reforged
pub(in crate::card::sets) static LAELIA_THE_BLADE_REFORGED: CardRecord =
    CardRecord::new_with_legacy_id(
        2302,
        "Laelia, the Blade Reforged",
        CardArt::new("a3bb2881-e8fb-4fba-a9f9-d93e6ca24378", "Wisnu Tan"),
        CardSet::Commander2021,
        // Three mana with haste that attacks as a 3/3 on the turn it lands, and
        // grows every attack after because her own trigger feeds the other one.
        CardRules::new_creature(mana_cost!("{2}{R}"), &["Spirit", "Warrior"], 2, 2)
            .with_supertype(CardSupertype::Legendary)
            .with_abilities(&LAELIA_ABILITIES),
    );

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&LAELIA_THE_BLADE_REFORGED];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
