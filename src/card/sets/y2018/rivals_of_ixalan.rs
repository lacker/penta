//! Rivals of Ixalan card records.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AppliedEffectDef;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CostDef;
use crate::card::CreateTokenDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::PlayerRelation;
use crate::card::ResolvedEffectDurationDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerConditionDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "RIX",
    slug: "rivals-of-ixalan",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

const TREASURE_TOKEN: TokenCharacteristics =
    crate::card::tokens::treasure().with_art(CardArt::new(
        "720f3e68-84c0-462e-a0d1-90236ccc494a",
        "Florian de Gesincourt",
    ));

// RIX 15 — Moment of Triumph
pub(in crate::card::sets) static MOMENT_OF_TRIUMPH: CardRecord = CardRecord::new(
    "Moment of Triumph",
    "08be8c18-eca3-4960-b174-e4a78579ed63",
    "Steven Belledin",
    CardRules::new_instant(mana_cost!("{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets +2/+2 until end of turn. You gain 2 life.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                )]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ]),
    )]),
);

// RIX 30 — Zetalpa, Primal Dawn
pub(in crate::card::sets) static ZETALPA_PRIMAL_DAWN: CardRecord = CardRecord::new(
    "Zetalpa, Primal Dawn",
    "3d10560f-199d-4d04-b573-90024f8aecc4",
    "Chris Rallis",
    CardRules::new_creature(mana_cost!("{6}{W}{W}"), &["Elder", "Dinosaur"], 4, 8)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            abilities::double_strike(),
            abilities::vigilance(),
            abilities::trample(),
            abilities::indestructible(),
        ]),
);

// RIX 41 — Kitesail Corsair
pub(in crate::card::sets) static KITESAIL_CORSAIR: CardRecord = CardRecord::new(
    "Kitesail Corsair",
    "4b8d0e8d-c2d4-4682-8095-827ffd79539b",
    "Greg Opalinski",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Human", "Pirate"], 2, 1).with_abilities(&[
        AbilityDef::static_ability(
            "This creature has flying as long as it's attacking.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceMatches {
                    object: ObjectPredicateDef::Attacking,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::flying()),
                },
            },
        ),
    ]),
);

// RIX 45 — Nezahal, Primal Tide
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NEZAHAL_PRIMAL_TIDE_45: CardRecord = CardRecord::new(
    "Nezahal, Primal Tide",
    "48eba418-94ab-46a3-958c-4d5058fc2bcd",
    "Sam Burley",
    crate::card::CardRules::unsupported(),
);

// RIX 59 — Timestream Navigator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TIMESTREAM_NAVIGATOR_59: CardRecord = CardRecord::new(
    "Timestream Navigator",
    "14770537-209a-4260-88a4-30f4e2b5ede0",
    "Zezhou Chen",
    crate::card::CardRules::unsupported(),
);

// RIX 79 — Moment of Craving
pub(in crate::card::sets) static MOMENT_OF_CRAVING: CardRecord = CardRecord::new(
    "Moment of Craving",
    "c0e3ea55-162d-4466-ba4e-b938a0845fb5",
    "Steven Belledin",
    CardRules::new_instant(mana_cost!("{1}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets -2/-2 until end of turn. You gain 2 life.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-2),
                    ValueDef::Constant(-2),
                )]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ]),
    )]),
);

// RIX 93 — Bombard
pub(in crate::card::sets) static BOMBARD: CardRecord = CardRecord::new(
    "Bombard",
    "0a605abc-78e8-47ba-9022-0fad9006fd05",
    "Alex Konstad",
    CardRules::new_instant(mana_cost!("{2}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Bombard deals 4 damage to target creature.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::damage(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ValueDef::Constant(4),
        ),
    )]),
);

// RIX 94 — Brass's Bounty
pub(in crate::card::sets) static BRASS_S_BOUNTY: CardRecord = CardRecord::new(
    "Brass's Bounty",
    "13505866-6ce9-4d11-b3f3-fc2f09839b71",
    "Grzegorz Rutkowski",
    CardRules::new_sorcery(mana_cost!("{6}{R}")).with_abilities(&[AbilityDef::spell(
        "For each land you control, create a Treasure token. (It's an \
         artifact with \"{T}, Sacrifice this token: Add one mana of \
         any color.\")",
        EffectDef::CreateToken(
            CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN)).with_count(
                ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Land),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
            ),
        ),
    )]),
);

// RIX 100 — Etali, Primal Storm
// Audit: unsupported — Needs a single resolving permission to cast any number of cards from a shared exiled group in a player-chosen order, with normal timing ignored only during that resolution.
pub(in crate::card::sets) static ETALI_PRIMAL_STORM: CardRecord = CardRecord::new(
    "Etali, Primal Storm",
    "1d3d8bb4-0430-45bb-930d-5d6db6521945",
    "Raymond Swanland",
    CardRules::unsupported(),
);

// RIX 101 — Fanatical Firebrand
pub(in crate::card::sets) static FANATICAL_FIREBRAND: CardRecord = CardRecord::new(
    "Fanatical Firebrand",
    "5e5565de-028c-4799-a9f6-4dcd685639eb",
    "Wayne Reynolds",
    // Haste is what makes the sacrifice a one-mana Shock the turn it lands;
    // left alive it is a one-power attacker that can cash itself in later.
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin", "Pirate"], 1, 1).with_abilities(&[
        abilities::haste(),
        AbilityDef::activated_with_targets(
            "{T}, Sacrifice this creature: It deals 1 damage to any target.",
            &[CostDef::TapSource, CostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(1),
            ),
        ),
    ]),
);

// RIX 130 — Ghalta, Primal Hunger
// Audit: unsupported — Needs self spell-cost reduction from the summed power of controlled creatures; the self-cost evaluator accepts object counts but not power aggregates.
pub(in crate::card::sets) static GHALTA_PRIMAL_HUNGER: CardRecord = CardRecord::new(
    "Ghalta, Primal Hunger",
    "0104b5b3-9376-4ad7-9a77-3e564e9c42e6",
    "Chase Stone",
    CardRules::unsupported(),
);

// RIX 148 — Thrashing Brontodon
pub(in crate::card::sets) static THRASHING_BRONTODON: CardRecord = CardRecord::new(
    "Thrashing Brontodon",
    "0d9264ff-9f7c-46f3-862a-fee7ad213250",
    "Jakub Kasper",
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Dinosaur"], 3, 4).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{1}, Sacrifice this creature: Destroy target artifact or \
             enchantment.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one_permanent(
                crate::card::ObjectPredicateDef::AnyOf(&[
                    crate::card::ObjectPredicateDef::HasType(crate::card::CardType::Artifact),
                    crate::card::ObjectPredicateDef::HasType(crate::card::CardType::Enchantment),
                ]),
            )],
            EffectDef::destroy_target(TargetIndex::PRIMARY),
        ),
    ]),
);

// RIX 149 — Thunderherd Migration
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THUNDERHERD_MIGRATION_149: CardRecord = CardRecord::new(
    "Thunderherd Migration",
    "c56de4a3-f5ab-469e-ab66-b8187c8c04a0",
    "Lars Grant-West",
    crate::card::CardRules::unsupported(),
);

// RIX 174 — Zacama, Primal Calamity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ZACAMA_PRIMAL_CALAMITY_174: CardRecord = CardRecord::new(
    "Zacama, Primal Calamity",
    "5aa75f2b-53c5-47c5-96d2-ab796358a96f",
    "Jaime Jones",
    crate::card::CardRules::unsupported(),
);

// RIX 178 — Gleaming Barrier
pub(in crate::card::sets) static GLEAMING_BARRIER: CardRecord = CardRecord::new(
    "Gleaming Barrier",
    "62447a76-4aa7-4823-941e-84bc18eb672a",
    "Jason Felix",
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Wall"], 0, 4).with_abilities(&[
        abilities::defender(),
        abilities::dies_trigger(
            "When this creature dies, create a Treasure token. (It's an \
             artifact with \"{T}, Sacrifice this token: Add one mana of \
             any color.\")",
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
        ),
    ]),
);

// RIX 203 — Swab Goblin
pub(in crate::card::sets) static SWAB_GOBLIN: CardRecord = CardRecord::new(
    "Swab Goblin",
    "0403b8e5-29c5-4a4a-b9e7-1e79a7452f14",
    "Josu Hernaiz",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin", "Pirate"], 2, 2),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &MOMENT_OF_TRIUMPH,
    &ZETALPA_PRIMAL_DAWN,
    &KITESAIL_CORSAIR,
    &NEZAHAL_PRIMAL_TIDE_45,
    &TIMESTREAM_NAVIGATOR_59,
    &MOMENT_OF_CRAVING,
    &BOMBARD,
    &BRASS_S_BOUNTY,
    &ETALI_PRIMAL_STORM,
    &FANATICAL_FIREBRAND,
    &GHALTA_PRIMAL_HUNGER,
    &THRASHING_BRONTODON,
    &THUNDERHERD_MIGRATION_149,
    &ZACAMA_PRIMAL_CALAMITY_174,
    &GLEAMING_BARRIER,
    &SWAB_GOBLIN,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
