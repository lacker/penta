//! Core Set 2020 cards cataloged for the Vintage Cube.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AppliedEffectDef, AppliedRuleDef, CardArt,
    CardRules, CardSet, CardType, EffectDef, EffectRecipientDef, ObjectPredicateDef,
    ResolvedEffectDurationDef,
};
use crate::{TargetIndex, mana_cost};

/// "Another" excludes the Key itself, which is what stops it untapping
/// itself for free every turn.
static ANOTHER_ARTIFACT: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Artifact),
        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
    ]),
)];

static A_CREATURE: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Creature),
)];

// M20 3 — Ancestral Blade
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ANCESTRAL_BLADE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2ba18114-af6c-48cd-82c9-eb6541d566bf"),
    "Ancestral Blade",
    crate::card::CardArt::new("2ba18114-af6c-48cd-82c9-eb6541d566bf", "Scott Murphy"),
    crate::card::CardSet::Magic2020,
    crate::card::CardRules::unsupported(),
);

// M20 34 — Raise the Alarm
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RAISE_THE_ALARM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4be510c8-fc01-4374-ac04-7968d24480fe"),
    "Raise the Alarm",
    crate::card::CardArt::new("764a7a53-314e-4b1f-aa33-0f312d06df71", "Zoltan Boros"),
    crate::card::CardSet::Magic2020,
    crate::card::CardRules::unsupported(),
);

// M20 54 — Cloudkin Seer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CLOUDKIN_SEER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e2111753-a930-403f-9d94-a86dfcb069da"),
    "Cloudkin Seer",
    crate::card::CardArt::new(
        "e2111753-a930-403f-9d94-a86dfcb069da",
        "Anastasia Ovchinnikova",
    ),
    crate::card::CardSet::Magic2020,
    crate::card::CardRules::unsupported(),
);

// M20 230 — Manifold Key
pub(in crate::card::sets) static MANIFOLD_KEY: CardRecord = CardRecord::new_with_legacy_id(
    2207,
    "Manifold Key",
    CardArt::new("715e637a-dfd8-45a0-b1ea-53e4abd29307", "Lake Hurwitz"),
    CardSet::Magic2020,
    // One mana that untaps a Mox for profit and, when there is nothing to
    // untap, pushes a creature through instead.
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{1}, {T}: Untap another target artifact.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::TapSource,
            ],
            &ANOTHER_ARTIFACT,
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
        AbilityDef::activated_with_targets(
            "{3}, {T}: Target creature can't be blocked this turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{3}")),
                AbilityCostDef::TapSource,
            ],
            &A_CREATURE,
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBeBlocked),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// M20 297 — Wildfire Elemental
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WILDFIRE_ELEMENTAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("272e317c-55c4-43b2-91aa-3e0009cfd7d5"),
    "Wildfire Elemental",
    crate::card::CardArt::new("272e317c-55c4-43b2-91aa-3e0009cfd7d5", "Svetlin Velinov"),
    crate::card::CardSet::Magic2020,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ANCESTRAL_BLADE,
    &RAISE_THE_ALARM,
    &CLOUDKIN_SEER,
    &MANIFOLD_KEY,
    &WILDFIRE_ELEMENTAL,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
