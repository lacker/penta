//! The Big Score cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    CardArt, CardRules, CardSet, CardSupertype, CardType, EffectDef, EffectRecipientDef,
    ObjectPredicateDef, PlayerRelation, TriggerEventDef, ValueDef, ZoneKind, abilities,
};
use crate::{TargetIndex, mana_cost};

static ANY_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::AnyTarget,
)];

// BIG 9 — Harvester of Misery
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HARVESTER_OF_MISERY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a3012af9-621d-4fae-b00d-079a89ae35fe"),
    "Harvester of Misery",
    crate::card::CardArt::new("a3012af9-621d-4fae-b00d-079a89ae35fe", "Jorge Jacinto"),
    crate::card::CardSet::TheBigScore,
    crate::card::CardRules::unsupported(),
);

// BIG 12 — Legion Extruder
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

// BIG 21 — Loot, the Pathfinder
static LOOT_ABILITIES: [AbilityDef; 6] = [
    abilities::double_strike(),
    abilities::vigilance(),
    abilities::haste(),
    AbilityDef::activated_mana(
        "Exhaust — {G}, {T}: Add three mana of any one color. (Activate each exhaust ability \
         only once.)",
        &LOOT_GREEN_COST,
        EffectDef::AddMana(AddManaEffectDef::any_color().with_amount(3)),
    )
    .exhausting(),
    AbilityDef::activated(
        "Exhaust — {U}, {T}: Draw three cards.",
        &LOOT_BLUE_COST,
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(3),
        },
    )
    .exhausting(),
    AbilityDef::activated_with_targets(
        "Exhaust — {R}, {T}: This creature deals 3 damage to any target.",
        &LOOT_RED_COST,
        &ANY_TARGET,
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(3),
        },
    )
    .exhausting(),
];

static LOOT_GREEN_COST: [AbilityCostDef; 2] = [
    AbilityCostDef::Mana(mana_cost!("{G}")),
    AbilityCostDef::TapSource,
];

static LOOT_BLUE_COST: [AbilityCostDef; 2] = [
    AbilityCostDef::Mana(mana_cost!("{U}")),
    AbilityCostDef::TapSource,
];

static LOOT_RED_COST: [AbilityCostDef; 2] = [
    AbilityCostDef::Mana(mana_cost!("{R}")),
    AbilityCostDef::TapSource,
];

pub(in crate::card::sets) static LOOT_THE_PATHFINDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fb169fa2-c92e-45f7-89a2-0ca0e3910a1c"),
    "Loot, the Pathfinder",
    CardArt::new("fb169fa2-c92e-45f7-89a2-0ca0e3910a1c", "Rudy Siswanto"),
    CardSet::TheBigScore,
    // Five mana for a hasty double striker that also unloads three cards,
    // three mana, or three damage -- once each, and never twice, because
    // every one of them taps it.
    CardRules::new_creature(mana_cost!("{2}{G}{U}{R}"), &["Beast", "Noble"], 2, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&LOOT_ABILITIES),
);

// BIG 41 — Generous Plunderer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GENEROUS_PLUNDERER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("351eea06-f5be-4044-b3b3-cc6bf805abb1"),
    "Generous Plunderer",
    crate::card::CardArt::new(
        "351eea06-f5be-4044-b3b3-cc6bf805abb1",
        "Josiah \"Jo\" Cameron",
    ),
    crate::card::CardSet::TheBigScore,
    crate::card::CardRules::unsupported(),
);

// BIG 85 — Vaultborn Tyrant
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VAULTBORN_TYRANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("07ca436a-e992-40a9-978a-501a82e443ed"),
    "Vaultborn Tyrant",
    crate::card::CardArt::new("07ca436a-e992-40a9-978a-501a82e443ed", "Loïc Canavaggia"),
    crate::card::CardSet::TheBigScore,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &HARVESTER_OF_MISERY,
    &LEGION_EXTRUDER,
    &LOOT_THE_PATHFINDER,
    &GENEROUS_PLUNDERER,
    &VAULTBORN_TYRANT,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
