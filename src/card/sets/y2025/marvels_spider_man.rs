//! SPM card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCoverageDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AppliedEffectDef,
    CardArt, CardRules, CardSet, CardType, EffectDef, EffectRecipientDef, ObjectPredicateDef,
    ObjectRefDef, PlayerRelation, ResolvedEffectDurationDef, ValueDef, ZoneKind,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

// SPM 141 — Rhino's Rampage
// Audit: partial — The pump and fight are implemented. The excess-damage
// reflexive trigger needs a target chosen after the fight, rather than a spell
// target selected while casting.
static RHINOS_RAMPAGE_TARGETS: [AbilityTargetDef; 2] = [
    AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::You),
        owner: None,
    }),
    AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::Opponent),
        owner: None,
    }),
];

static RHINOS_RAMPAGE_EFFECTS: [EffectDef; 2] = [
    EffectDef::Apply {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        effect: AppliedEffectDef::modify_power_toughness(
            ValueDef::Constant(1),
            ValueDef::Constant(0),
        ),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    },
    EffectDef::Fight {
        first: ObjectRefDef::Target(TargetIndex::PRIMARY),
        second: ObjectRefDef::Target(TargetIndex(1)),
        excess: None,
    },
];

pub(in crate::card::sets) static RHINOS_RAMPAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f668817c-1cab-44c5-b6a8-95113e480d5e"),
    "Rhino's Rampage",
    CardArt::new("f668817c-1cab-44c5-b6a8-95113e480d5e", "Nino Is"),
    CardSet::MarvelsSpiderMan,
    CardRules::new_sorcery(mana_cost!("{R/G}")).with_ability(
        AbilityDef::spell_with_targets(
            "Target creature you control gets +1/+0 until end of turn. It fights target creature an opponent controls. When excess damage is dealt this way, destroy up to one target noncreature artifact with mana value 3 or less.",
            &RHINOS_RAMPAGE_TARGETS,
            EffectDef::Sequence(&RHINOS_RAMPAGE_EFFECTS),
        )
        .with_coverage(AbilityCoverageDef::partial(
            "The pump and simultaneous fight are implemented; the reflexive trigger must choose its optional artifact target after excess damage is known.",
        )),
    ),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&RHINOS_RAMPAGE];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
