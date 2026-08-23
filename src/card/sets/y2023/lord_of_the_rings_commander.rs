//! The Lord of the Rings: Tales of Middle-earth Commander cards cataloged for
//! the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AppliedEffectDef, CardArt, CardRules, CardSet, CardType,
    EffectDef, EffectRecipientDef, ObjectPredicateDef, ResolvedEffectDurationDef, TriggerEventDef,
    ValueDef, abilities,
};
use crate::{TargetIndex, mana_cost};

static A_CREATURE: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Creature),
)];

/// "Up to one target creature", which is the granted ability's own target
/// rather than the spell's: it is chosen as the trigger goes on the stack,
/// so an arrow with nothing to shoot at is still an arrow.
static UP_TO_ONE_CREATURE: [AbilityTargetDef; 1] = [AbilityTargetDef::up_to(
    crate::card::AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[crate::card::ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
    1,
)];

static REFLEXES_SHOT: AbilityDef = AbilityDef::triggered_with_targets(
    "Whenever this creature becomes tapped, it deals damage equal to its power to up to one \
     target creature.",
    TriggerEventDef::tapped(ObjectPredicateDef::Source),
    &UP_TO_ONE_CREATURE,
    EffectDef::DealDamage {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        amount: ValueDef::SourcePower,
    },
);

static REFLEXES_REACH: AbilityDef = abilities::reach();
static REFLEXES_HEXPROOF: AbilityDef = abilities::hexproof();

static REFLEXES_GRANT: [AppliedEffectDef; 3] = [
    AppliedEffectDef::add_ability(&REFLEXES_REACH),
    AppliedEffectDef::add_ability(&REFLEXES_HEXPROOF),
    AppliedEffectDef::add_ability(&REFLEXES_SHOT),
];

static REFLEXES_EFFECT: [EffectDef; 2] = [
    EffectDef::Untap {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
    EffectDef::Apply {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        effect: AppliedEffectDef::Composite(&REFLEXES_GRANT),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    },
];

// LTC 56 — Forth Eorlingas!
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FORTH_EORLINGAS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("06c053d3-028e-4961-93a5-5b7bb5a8601c"),
    "Forth Eorlingas!",
    crate::card::CardArt::new("06c053d3-028e-4961-93a5-5b7bb5a8601c", "Filipe Pagliuso"),
    crate::card::CardSet::LordOfTheRingsCommander,
    crate::card::CardRules::unsupported(),
);

// LTC 159 — Relic of Sauron
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RELIC_OF_SAURON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("15c5d6cd-8af6-4852-8043-e6b1ef771ce6"),
    "Relic of Sauron",
    crate::card::CardArt::new("15c5d6cd-8af6-4852-8043-e6b1ef771ce6", "Anton Solovianchyk"),
    crate::card::CardSet::LordOfTheRingsCommander,
    crate::card::CardRules::unsupported(),
);

// LTC 493 — Legolas's Quick Reflexes
pub(in crate::card::sets) static LEGOLASS_QUICK_REFLEXES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("851c0167-04ba-4d15-b0fa-c211bd8826f1"),
    "Legolas's Quick Reflexes",
    CardArt::new("851c0167-04ba-4d15-b0fa-c211bd8826f1", "Jason Rainville"),
    CardSet::LordOfTheRingsCommander,
    // One green mana nobody can answer: it untaps a blocker, makes it
    // untargetable, and turns every tap it takes afterwards into an arrow.
    CardRules::new_instant(mana_cost!("{G}")).with_abilities(&[
        abilities::split_second(),
        AbilityDef::spell_with_targets(
            "Untap target creature. Until end of turn, it gains reach, hexproof, and \"Whenever \
             this creature becomes tapped, it deals damage equal to its power to up to one target \
             creature.\"",
            &A_CREATURE,
            EffectDef::Sequence(&REFLEXES_EFFECT),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&FORTH_EORLINGAS, &RELIC_OF_SAURON, &LEGOLASS_QUICK_REFLEXES];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
