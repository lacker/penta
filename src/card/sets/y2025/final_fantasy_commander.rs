//! Final Fantasy Commander cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, CardArt, CardRules, CardSet, CardSupertype, CounterKind, EffectDef,
    EffectRecipientDef, ObjectPredicateDef, PlayerRelation, TriggerConditionDef, TriggerEventDef,
    TurnStepDef, ValueDef,
};
use crate::mana_cost;

// FIC 55 — Gau, Feral Youth
/// An intervening-if, so it is checked twice: once when the end step begins
/// and again as the ability resolves. A graveyard that gave a card up and
/// then got it back is still a graveyard a card left.
static A_CARD_LEFT_YOUR_GRAVEYARD: TriggerConditionDef =
    TriggerConditionDef::ControllerHadCardLeaveGraveyardThisTurn;

static GAU_ABILITIES: [AbilityDef; 2] = [
    // "Rage" is an ability word: flavour on the front of an ordinary attack
    // trigger, and nothing the rules read.
    AbilityDef::triggered(
        "Rage — Whenever Gau attacks, put a +1/+1 counter on it.",
        TriggerEventDef::attacks(ObjectPredicateDef::Source),
        EffectDef::AddCounters {
            object: EffectRecipientDef::Source,
            kind: CounterKind::PlusOnePlusOne,
            amount: ValueDef::Constant(1),
        },
    ),
    // Each end step, not just yours: a graveyard emptied on their turn pays
    // out on their turn too.
    AbilityDef::triggered_if(
        "At the beginning of each end step, if a card left your graveyard this turn, Gau deals \
         damage equal to its power to each opponent.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::End,
            player: PlayerRelation::Any,
        },
        &A_CARD_LEFT_YOUR_GRAVEYARD,
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Opponent,
            amount: ValueDef::SourcePower,
        },
    ),
];

pub(in crate::card::sets) static GAU_FERAL_YOUTH: CardRecord = CardRecord::new_with_legacy_id(
    2304,
    "Gau, Feral Youth",
    CardArt::new("89175ce1-0746-4ba1-970e-617d134b0527", "Eglė Mosakaitė"),
    CardSet::FinalFantasyCommander,
    // Two mana that grows every attack and, in a deck that is already using
    // its graveyard, throws that growth at the opponent every end step.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Berserker"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&GAU_ABILITIES),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&GAU_FERAL_YOUTH];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
