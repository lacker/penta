//! The Big Score cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, CardArt, CardRules,
    CardSet, CardType, EffectDef, EffectRecipientDef, ObjectPredicateDef, PlayerRelation,
    TriggerEventDef, ValueDef, ZoneKind,
};
use crate::{TargetIndex, mana_cost};

static ANY_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::AnyTarget,
)];

/// Another one: the Extruder is an artifact itself and may not eat itself,
/// which is what stops a two-mana artifact from being a Golem on its own.
static ANOTHER_ARTIFACT: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Artifact),
    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
]);

static EXTRUDER_GOLEM_COST: [AbilityCostDef; 3] = [
    AbilityCostDef::Mana(mana_cost!("{2}")),
    AbilityCostDef::TapSource,
    AbilityCostDef::SacrificePermanent {
        object: ANOTHER_ARTIFACT,
        controller: PlayerRelation::You,
    },
];

static LEGION_EXTRUDER_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::triggered_with_targets(
        "When this artifact enters, it deals 2 damage to any target.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::Source,
            None,
            Some(ZoneKind::Battlefield),
        ),
        &ANY_TARGET,
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(2),
        },
    ),
    AbilityDef::activated(
        "{2}, {T}, Sacrifice another artifact: Create a 3/3 colorless Golem artifact creature \
         token.",
        &EXTRUDER_GOLEM_COST,
        EffectDef::create_artifact_creature_token(&["Golem"], &[], 3, 3).with_art(CardArt::new(
            "406e2960-f560-48bb-b4a6-4bd35889a8f8",
            "Brian Valeza",
        )),
    ),
];

// BIG 12 — Legion Extruder
pub(in crate::card::sets) static LEGION_EXTRUDER: CardRecord = CardRecord::new_with_legacy_id(
    2288,
    "Legion Extruder",
    CardArt::new("5a077de0-1893-40d0-a499-ee2e6e2258f1", "Anton Solovianchyk"),
    CardSet::TheBigScore,
    // Two mana that answers a creature on the way in and then turns every
    // spent artifact -- a cracked Lotus Petal, an emptied Bauble -- into a
    // 3/3, which is what the cube's artifact decks have lying around.
    CardRules::new_artifact(mana_cost!("{1}{R}")).with_abilities(&LEGION_EXTRUDER_ABILITIES),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&LEGION_EXTRUDER];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
