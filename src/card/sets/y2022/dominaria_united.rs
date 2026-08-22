//! Dominaria United cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, CardArt, CardRules, CardSet, CardSupertype, CardType,
    DrawEventMatcherDef, EffectDef, EffectRecipientDef, ObjectPredicateDef, PlayerRelation,
    TriggerEventDef, ValueDef, abilities,
};
use crate::{TargetIndex, mana_cost};

/// Two clauses rather than one symmetrical one, because they are not
/// symmetrical: yours gains and theirs loses, and a card that made both
/// players lose would read very differently.
static SHEOLDRED_ABILITIES: [AbilityDef; 3] = [
    abilities::deathtouch(),
    AbilityDef::triggered(
        "Whenever you draw a card, you gain 2 life.",
        TriggerEventDef::DrewCard(DrawEventMatcherDef::any(PlayerRelation::You)),
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(2),
        },
    ),
    AbilityDef::triggered(
        "Whenever an opponent draws a card, they lose 2 life.",
        TriggerEventDef::DrewCard(DrawEventMatcherDef::any(PlayerRelation::Opponent)),
        EffectDef::LoseLife {
            recipient: EffectRecipientDef::EventPlayer,
            amount: ValueDef::Constant(2),
        },
    ),
];

/// "Total power and toughness 5 or less" is read live, so a creature that
/// was in range stops being a legal target the moment anything pumps it.
static CUT_DOWN_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::TotalPowerAndToughnessAtMost(5),
    ]),
)];

// DMU 89 — Cut Down
pub(in crate::card::sets) static CUT_DOWN: CardRecord = CardRecord::new_with_legacy_id(
    2204,
    "Cut Down",
    CardArt::new("753db072-5d6a-4f37-8f7d-255572ecd3bd", "Dominik Mayer"),
    CardSet::DominariaUnited,
    // One black mana answers most of what an aggressive deck plays and
    // nothing of what a big one does, which is the whole design.
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target creature with total power and toughness 5 or less.",
        &CUT_DOWN_TARGET,
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            can_regenerate: true,
        },
    )),
);

// DMU 107 — Sheoldred, the Apocalypse
pub(in crate::card::sets) static SHEOLDRED_THE_APOCALYPSE: CardRecord =
    CardRecord::new_with_legacy_id(
        2180,
        "Sheoldred, the Apocalypse",
        CardArt::new("d67be074-cdd4-41d9-ac89-0a0456c4e4b2", "Chris Rahn"),
        CardSet::DominariaUnited,
        // A four-mana 4/5 deathtouch would be playable on its own. The draw
        // clauses are what make it unanswerable: the opponent's own draw step
        // pays for it, every turn it survives.
        CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Phyrexian", "Praetor"], 4, 5)
            .with_supertype(CardSupertype::Legendary)
            .with_abilities(&SHEOLDRED_ABILITIES),
    );

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&CUT_DOWN, &SHEOLDRED_THE_APOCALYPSE];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
