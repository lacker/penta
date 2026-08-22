//! Scars of Mirrodin cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AddManaEffectDef, CardArt, CardRules, CardSet, EffectDef,
    ManaColor, abilities,
};

/// The fastland cycle: untapped while the board is still small, an expensive
/// tapped land after that. Every one of the ten prints this same clause, and
/// only the colour pair below it differs.
static FAST_LAND_ENTERS: AbilityDef = abilities::fast_land_enters(
    "This land enters tapped unless you control two or fewer other lands.",
);

static BLACKCLEAVE_CLIFFS_ABILITIES: [AbilityDef; 2] = [
    FAST_LAND_ENTERS,
    AbilityDef::activated_mana(
        "{T}: Add {B} or {R}.",
        &[AbilityCostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::choice(&[
            ManaColor::Black,
            ManaColor::Red,
        ])),
    ),
];

// SOM 224 — Blackcleave Cliffs
pub(in crate::card::sets) static BLACKCLEAVE_CLIFFS: CardRecord = CardRecord::new_with_legacy_id(
    2131,
    "Blackcleave Cliffs",
    CardArt::new("3d71be5f-0fd7-4a88-8041-f4d6bc4cc9ac", "Dave Kendall"),
    CardSet::ScarsOfMirrodin,
    CardRules::new_land(&[]).with_abilities(&BLACKCLEAVE_CLIFFS_ABILITIES),
);

static COPPERLINE_GORGE_ABILITIES: [AbilityDef; 2] = [
    FAST_LAND_ENTERS,
    AbilityDef::activated_mana(
        "{T}: Add {R} or {G}.",
        &[AbilityCostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::choice(&[
            ManaColor::Red,
            ManaColor::Green,
        ])),
    ),
];

// SOM 225 — Copperline Gorge
pub(in crate::card::sets) static COPPERLINE_GORGE: CardRecord = CardRecord::new_with_legacy_id(
    2132,
    "Copperline Gorge",
    CardArt::new(
        "28f1d784-f286-418d-a712-bc07ad10d4a2",
        "Zoltan Boros & Gabor Szikszai",
    ),
    CardSet::ScarsOfMirrodin,
    CardRules::new_land(&[]).with_abilities(&COPPERLINE_GORGE_ABILITIES),
);

static DARKSLICK_SHORES_ABILITIES: [AbilityDef; 2] = [
    FAST_LAND_ENTERS,
    AbilityDef::activated_mana(
        "{T}: Add {U} or {B}.",
        &[AbilityCostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::choice(&[
            ManaColor::Blue,
            ManaColor::Black,
        ])),
    ),
];

// SOM 226 — Darkslick Shores
pub(in crate::card::sets) static DARKSLICK_SHORES: CardRecord = CardRecord::new_with_legacy_id(
    2133,
    "Darkslick Shores",
    CardArt::new("e530388b-eb19-4211-abd8-8a4c3c38c3af", "Charles Urbach"),
    CardSet::ScarsOfMirrodin,
    CardRules::new_land(&[]).with_abilities(&DARKSLICK_SHORES_ABILITIES),
);

static RAZORVERGE_THICKET_ABILITIES: [AbilityDef; 2] = [
    FAST_LAND_ENTERS,
    AbilityDef::activated_mana(
        "{T}: Add {G} or {W}.",
        &[AbilityCostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::choice(&[
            ManaColor::Green,
            ManaColor::White,
        ])),
    ),
];

// SOM 228 — Razorverge Thicket
pub(in crate::card::sets) static RAZORVERGE_THICKET: CardRecord = CardRecord::new_with_legacy_id(
    2134,
    "Razorverge Thicket",
    CardArt::new("345e053a-3178-485c-8602-1624bbf2f064", "James Paick"),
    CardSet::ScarsOfMirrodin,
    CardRules::new_land(&[]).with_abilities(&RAZORVERGE_THICKET_ABILITIES),
);

static SEACHROME_COAST_ABILITIES: [AbilityDef; 2] = [
    FAST_LAND_ENTERS,
    AbilityDef::activated_mana(
        "{T}: Add {W} or {U}.",
        &[AbilityCostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::choice(&[
            ManaColor::White,
            ManaColor::Blue,
        ])),
    ),
];

// SOM 229 — Seachrome Coast
pub(in crate::card::sets) static SEACHROME_COAST: CardRecord = CardRecord::new_with_legacy_id(
    2135,
    "Seachrome Coast",
    CardArt::new("99939b90-e88c-4c2f-ba78-56d455611703", "Lars Grant-West"),
    CardSet::ScarsOfMirrodin,
    CardRules::new_land(&[]).with_abilities(&SEACHROME_COAST_ABILITIES),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &BLACKCLEAVE_CLIFFS,
    &COPPERLINE_GORGE,
    &DARKSLICK_SHORES,
    &RAZORVERGE_THICKET,
    &SEACHROME_COAST,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
