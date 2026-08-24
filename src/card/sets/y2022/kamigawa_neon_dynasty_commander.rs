//! Kamigawa: Neon Dynasty Commander cards cataloged for the Vintage Cube
//! pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AppliedEffectDef, AppliedRuleDef, CardArt, CardRules, CardSet, CardType,
    CounterKind, EffectDef, EffectRecipientDef, ObjectPredicateDef, PlayerRelation,
    ResolvedEffectDurationDef, TriggerEventDef, ValueDef, ZoneKind, abilities,
};
use crate::mana_cost;

// NEC 14 — Kappa Cannoneer
/// "This creature or another artifact you control": the Cannoneer's own
/// arrival counts, and so does every artifact after it -- including the ones
/// that are not creatures.
static CANNONEER_ARRIVALS: [TriggerEventDef; 2] = [
    TriggerEventDef::zone_changed(
        ObjectPredicateDef::Source,
        None,
        Some(ZoneKind::Battlefield),
    ),
    TriggerEventDef::zone_changed(
        ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Artifact),
            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
        ]),
        None,
        Some(ZoneKind::Battlefield),
    ),
];

static CANNONEER_GROWS: [EffectDef; 2] = [
    EffectDef::AddCounters {
        object: EffectRecipientDef::Source,
        kind: CounterKind::PlusOnePlusOne,
        amount: ValueDef::Constant(1),
    },
    EffectDef::Apply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    },
];

pub(in crate::card::sets) static KAPPA_CANNONEER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("85a89077-b384-4fca-9d26-7297962c1541"),
    "Kappa Cannoneer",
    CardArt::new("85a89077-b384-4fca-9d26-7297962c1541", "Jesper Ejsing"),
    CardSet::KamigawaNeonDynastyCommander,
    // Six mana on paper and rarely six in practice: the artifacts that make
    // it cheap are the same ones that make it bigger and unblockable.
    CardRules::new_artifact_creature(mana_cost!("{5}{U}"), &["Turtle", "Warrior"], 4, 4)
        .with_abilities(&[
            abilities::improvise(),
            abilities::ward(4, "Ward {4}"),
            AbilityDef::triggered(
                "Whenever this creature or another artifact you control enters, put a +1/+1 \
                 counter on this creature. It can't be blocked this turn.",
                TriggerEventDef::AnyOf(&CANNONEER_ARRIVALS),
                EffectDef::Sequence(&CANNONEER_GROWS),
            ),
        ]),
);

// NEC 76 — Shorikai, Genesis Engine
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHORIKAI_GENESIS_ENGINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0347cf84-42f5-4674-99de-619b0ae51d62"),
    "Shorikai, Genesis Engine",
    crate::card::CardArt::new("0347cf84-42f5-4674-99de-619b0ae51d62", "Wisnu Tan"),
    crate::card::CardSet::KamigawaNeonDynastyCommander,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&KAPPA_CANNONEER, &SHORIKAI_GENESIS_ENGINE];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
