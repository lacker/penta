//! BFZ card records required by supported formats.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::CostDef;
use crate::card::CreateTokenDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectRefDef;
use crate::card::PlayerRelation;
use crate::card::ResolvedEffectDurationDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::abilities;
use crate::ids::TargetIndex;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "BFZ",
    slug: "battle-for-zendikar",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

const ELDRAZI_SCION_TOKEN: TokenCharacteristics =
    TokenCharacteristics::creature(&["Eldrazi", "Scion"], &[], 1, 1)
        .with_abilities(&[AbilityDef::activated_mana(
            "Sacrifice this creature: Add {C}.",
            &[CostDef::SacrificeSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
        )])
        .with_art(CardArt::new("b999a0fe-d2d0-4367-9abb-6ce5f3764f19", "Izzy"));

// BFZ 25 — Felidar Cub
pub(in crate::card::sets) static FELIDAR_CUB: CardRecord = CardRecord::new(
    "Felidar Cub",
    "ea76a183-e15c-4968-b29d-91c074aa8681",
    "Steve Prescott",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Cat", "Beast"], 2, 2).with_abilities(&[
        AbilityDef::activated_with_targets(
            "Sacrifice this creature: Destroy target enchantment.",
            &[CostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Enchantment),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
    ]),
);

// BFZ 50 — Stasis Snare
// Audit: unsupported — Needs an exile-until-source-leaves duration with immediate return when that duration ends (CR 610.3); an ordinary leaves trigger returns the card later through the stack.
pub(in crate::card::sets) static STASIS_SNARE: CardRecord = CardRecord::new(
    "Stasis Snare",
    "ff820544-f4a3-40c4-a48e-84b5e2d06caa",
    "Jason Felix",
    CardRules::unsupported(),
);

// BFZ 58 — Eldrazi Skyspawner
pub(in crate::card::sets) static ELDRAZI_SKYSPAWNER: CardRecord = CardRecord::new(
    "Eldrazi Skyspawner",
    "9c9c1a10-446e-492a-95cc-a459dc6c08a0",
    "Chase Stone",
// Three mana for two bodies and a ritual: the Scion is what turns the
    // flier into a fourth-turn six-drop.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Eldrazi", "Drone"], 2, 1).with_abilities(&[
        abilities::devoid(),
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, create a 1/1 colorless Eldrazi Scion creature token. It has \"Sacrifice this token: Add {C}.\"",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(ELDRAZI_SCION_TOKEN))),
        ),
    ]),
);

// BFZ 106 — Carrier Thrall
pub(in crate::card::sets) static CARRIER_THRALL: CardRecord = CardRecord::new(
    "Carrier Thrall",
    "bd2ab895-9225-4eba-90c3-4023db4f8b70",
    "Lius Lasahido",
// Two mana that trades and still leaves a mana behind, which is why the
    // body is aggressive and the death trigger is not.
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Vampire"], 2, 1).with_abilities(&[
        abilities::devoid(),
        abilities::dies_trigger(
            "When this creature dies, create a 1/1 colorless Eldrazi Scion creature token. It has \"Sacrifice this token: Add {C}.\"",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(ELDRAZI_SCION_TOKEN))),
        ),
    ]),
);

// BFZ 124 — Vampiric Rites
pub(in crate::card::sets) static VAMPIRIC_RITES: CardRecord = CardRecord::new(
    "Vampiric Rites",
    "3416309a-5824-48f5-876e-00e0f180acf9",
    "Anastasia Ovchinnikova",
    CardRules::new_enchantment(mana_cost!("{B}")).with_abilities(&[AbilityDef::activated(
        "{1}{B}, Sacrifice a creature: You gain 1 life and draw a card.",
        &[
            CostDef::Mana(mana_cost!("{1}{B}")),
            CostDef::sacrifice_permanent(ObjectPredicateDef::HasType(CardType::Creature)),
        ],
        EffectDef::Sequence(&[
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
            abilities::draw_cards(ValueDef::Constant(1)),
        ]),
    )]),
);

// BFZ 157 — Sure Strike
pub(in crate::card::sets) static SURE_STRIKE: CardRecord = CardRecord::new(
    "Sure Strike",
    "074dd176-4608-42ca-8fc3-c7040cd7b32e",
    "Jakub Kasper",
    CardRules::new_instant(mana_cost!("{1}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets +3/+0 and gains first strike until end \
         of turn. (It deals combat damage before creatures without \
         first strike.)",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(3),
                    ValueDef::Constant(0),
                ),
                AppliedEffectDef::add_ability(&abilities::first_strike()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// BFZ 168 — Unnatural Aggression
pub(in crate::card::sets) static UNNATURAL_AGGRESSION: CardRecord = CardRecord::new(
    "Unnatural Aggression",
    "8293c66d-9a9b-4817-9bc3-ffd57fda290c",
    "James Ryman",
CardRules::new_instant(mana_cost!("{2}{G}"))
        .printed_colors(&[])
        .with_abilities(&[
            abilities::devoid(),
            AbilityDef::spell_with_targets(
                "Target creature you control fights target creature an opponent controls. If the creature an opponent controls would die this turn, exile it instead.",
                &[
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
                ],
                EffectDef::Sequence(&[
                    EffectDef::Fight {
                        first: ObjectRefDef::Target(TargetIndex::PRIMARY),
                        second: ObjectRefDef::Target(TargetIndex(1)),
                        excess: None,
                    },
                    // This sentence is independent of whether the fight dealt damage. If the
                    // opponent's creature remains a legal target, any way it would die later
                    // this turn is replaced with exile.
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex(1)),
                        effect: AppliedEffectDef::Rule(AppliedRuleDef::ExileInsteadOfDying),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            ),
        ]),
);

// BFZ 223 — Hedron Archive
pub(in crate::card::sets) static HEDRON_ARCHIVE: CardRecord = CardRecord::new(
    "Hedron Archive",
    "2519149c-f8a3-413f-b7b2-cd596970be4c",
    "Craig J Spearing",
    CardRules::new_artifact(mana_cost!("{4}")).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {C}{C}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless).with_amount(2)),
        ),
        AbilityDef::activated(
            "{2}, {T}, Sacrifice this artifact: Draw two cards.",
            &[
                CostDef::Mana(mana_cost!("{2}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            abilities::draw_cards(ValueDef::Constant(2)),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &FELIDAR_CUB,
    &STASIS_SNARE,
    &ELDRAZI_SKYSPAWNER,
    &CARRIER_THRALL,
    &VAMPIRIC_RITES,
    &SURE_STRIKE,
    &UNNATURAL_AGGRESSION,
    &HEDRON_ARCHIVE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
