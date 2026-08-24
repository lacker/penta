//! Core Set 2020 cards cataloged for the Vintage Cube.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AppliedEffectDef, AppliedRuleDef, CardArt,
    CardRules, CardSet, CardType, ComparisonDef, EffectDef, EffectRecipientDef, ObjectPredicateDef,
    ObjectQueryDef, PlayerRelation, PlayerSetDef, ResolvedEffectDurationDef, TriggerConditionDef,
    ValueDef, ZoneKind, ZonePlacement,
};
use crate::{TargetIndex, mana_cost};

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

// M20 169 — Elvish Reclaimer
/// "Three or more land cards in your graveyard": the fetchlands that made
/// him a 3/4 are the same ones his own ability puts there, which is why he
/// grows on the turn he is used.
static LAND_CARDS_IN_YOUR_GRAVEYARD: ObjectQueryDef = ObjectQueryDef::owned_by(
    ObjectPredicateDef::HasType(CardType::Land),
    &[ZoneKind::Graveyard],
    PlayerSetDef::Related(PlayerRelation::You),
);

static THREE_LANDS_IN_YOUR_GRAVEYARD: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: LAND_CARDS_IN_YOUR_GRAVEYARD,
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 3,
};

static RECLAIMER_GROWS: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::Source,
    effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(2), ValueDef::Constant(2)),
};

static RECLAIMER_FETCH_COST: [AbilityCostDef; 3] = [
    AbilityCostDef::Mana(mana_cost!("{2}")),
    AbilityCostDef::TapSource,
    AbilityCostDef::SacrificePermanent {
        object: ObjectPredicateDef::HasType(CardType::Land),
        controller: PlayerRelation::You,
    },
];

pub(in crate::card::sets) static ELVISH_RECLAIMER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("39c431d7-d94b-46c4-bb89-f3db56214ab4"),
    "Elvish Reclaimer",
    CardArt::new(
        "39c431d7-d94b-46c4-bb89-f3db56214ab4",
        "Victor Adame Minguez",
    ),
    CardSet::Magic2020,
    // One mana for a body that turns a spent fetchland into whatever land
    // the deck is built around, and is a 3/4 by the time it has done it
    // twice.
    CardRules::new_creature(mana_cost!("{G}"), &["Elf", "Warrior"], 1, 2).with_abilities(&[
        AbilityDef::static_ability(
            "This creature gets +2/+2 as long as there are three or more land cards in your \
             graveyard.",
            EffectDef::IfCondition {
                condition: &THREE_LANDS_IN_YOUR_GRAVEYARD,
                then: &RECLAIMER_GROWS,
            },
        ),
        AbilityDef::activated(
            "{2}, {T}, Sacrifice a land: Search your library for a land card, put it onto the \
             battlefield tapped, then shuffle.",
            &RECLAIMER_FETCH_COST,
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::HasType(CardType::Land),
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: false,
                destination: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: true,
                attachment: None,
                binding: None,
                then: None,
            },
        ),
    ]),
);

// M20 230 — Manifold Key
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
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// M20 247 — Field of the Dead
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FIELD_OF_THE_DEAD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("470ca3f4-29aa-4c4c-8ff2-8cdd70c69943"),
    "Field of the Dead",
    crate::card::CardArt::new("470ca3f4-29aa-4c4c-8ff2-8cdd70c69943", "Kev Walker"),
    crate::card::CardSet::Magic2020,
    crate::card::CardRules::unsupported(),
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
    &ELVISH_RECLAIMER,
    &MANIFOLD_KEY,
    &FIELD_OF_THE_DEAD,
    &WILDFIRE_ELEMENTAL,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
