//! Marvel Super Heroes Commander cards used for legend-rule coverage.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::ComparisonDef;
use crate::card::CopyExceptionsDef;
use crate::card::CreateTokenDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ObjectPredicateDef;
use crate::card::PlayerRelation;
use crate::card::PlayerRuleDef;
use crate::card::PlayerSetDef;
use crate::card::SpellCastQueryDef;
use crate::card::TokenDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueComparisonDef;
use crate::card::ValueDef;
use crate::mana_cost;

static NONCREATURE_SPELLS_YOU_CAST: SpellCastQueryDef = SpellCastQueryDef {
    player: PlayerRelation::You,
    spell: ObjectPredicateDef::NoncreatureSpell,
};

static CAST_A_NONCREATURE_SPELL_THIS_TURN: ValueComparisonDef = ValueComparisonDef {
    left: ValueDef::CountSpellsCastThisTurn(&NONCREATURE_SPELLS_YOU_CAST),
    comparison: ComparisonDef::GreaterOrEqual,
    right: ValueDef::Constant(1),
};

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "MSC",
    slug: "marvel-super-heroes-commander",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// MSC 28 — Council of Reeds
pub(in crate::card::sets) static COUNCIL_OF_REEDS: CardRecord = CardRecord::new(
    "Council of Reeds",
    "a0d824ea-75d2-4de5-923b-813bba44e80b",
    "Vlad Petruchik",
CardRules::new_creature(
        mana_cost!("{2}{U}"),
        &["Human", "Scientist", "Hero"],
        2,
        2,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        AbilityDef::static_ability(
            "The \"legend rule\" doesn't apply to creatures you control.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::players(PlayerSetDef::Related(
                    PlayerRelation::You,
                )),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::PlayerRule(
                    PlayerRuleDef::LegendRuleDoesNotApplyTo(&ObjectPredicateDef::HasType(
                        CardType::Creature,
                    )),
                )),
            },
        ),
        AbilityDef::triggered_if(
            "At the beginning of combat on your turn, if you've cast a noncreature spell this turn, create a token that's a copy of Council of Reeds.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::ValueComparison(&CAST_A_NONCREATURE_SPELL_THIS_TURN),
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Copy(
                &crate::card::TokenCopyDef {
                    object: &EffectRecipientDef::Source,
                    exceptions: CopyExceptionsDef::NONE,
                },
            ))),
        ),
    ]),
);

// MSC 56 — Loki's Scepter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LOKI_S_SCEPTER_56: CardRecord = CardRecord::new(
    "Loki's Scepter",
    "57e90938-9225-4c15-b4fb-aad2cced2e6a",
    "L J Koh",
    crate::card::CardRules::unsupported(),
);

// MSC 106 — H.E.R.B.I.E., Lovable Robot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static H_E_R_B_I_E_LOVABLE_ROBOT_106: CardRecord = CardRecord::new(
    "H.E.R.B.I.E., Lovable Robot",
    "5e2f9f4c-dbdd-4da6-b08f-1f4d3fcb1328",
    "Nanna Marie Steffensen",
    crate::card::CardRules::unsupported(),
);

// MSC 395 — Black Widow, Agile Avenger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLACK_WIDOW_AGILE_AVENGER_395: CardRecord = CardRecord::new(
    "Black Widow, Agile Avenger",
    "0b165b56-ad86-4d8c-91e2-1464b2ac5c6d",
    "Junggeun Yoon",
    crate::card::CardRules::unsupported(),
);

// MSC 654 — Doctor Doom, Unrivaled
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DOCTOR_DOOM_UNRIVALED_654: CardRecord = CardRecord::new(
    "Doctor Doom, Unrivaled",
    "2973e855-fe93-41a1-a62e-4699ef2c3d1d",
    "Vilhelmas Banys",
    crate::card::CardRules::unsupported(),
);

// MSC 678 — Asgardian Inspiration
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ASGARDIAN_INSPIRATION_678: CardRecord = CardRecord::new(
    "Asgardian Inspiration",
    "d1fe7eef-2636-484f-b476-4dc9c9dbfddd",
    "Nathaniel Himawan",
    crate::card::CardRules::unsupported(),
);

// MSC 707 — The Vision and Scarlet Witch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_VISION_AND_SCARLET_WITCH_707: CardRecord = CardRecord::new(
    "The Vision and Scarlet Witch",
    "930afb5f-54b7-4cca-8c28-3e48938f3a43",
    "Tyler Walpole",
    crate::card::CardRules::unsupported(),
);

// MSC 719 — Devil Dinosaur
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEVIL_DINOSAUR_719: CardRecord = CardRecord::new(
    "Devil Dinosaur",
    "fcb4c268-441f-41cf-8c25-538eefc6a710",
    "Wero Gallo",
    crate::card::CardRules::unsupported(),
);

// MSC 754 — Fogwell's Gym
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FOGWELL_S_GYM_754: CardRecord = CardRecord::new(
    "Fogwell's Gym",
    "e63523d5-0f2c-436a-b411-662eb11fd150",
    "Pace Wilder",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &COUNCIL_OF_REEDS,
    &LOKI_S_SCEPTER_56,
    &H_E_R_B_I_E_LOVABLE_ROBOT_106,
    &BLACK_WIDOW_AGILE_AVENGER_395,
    &DOCTOR_DOOM_UNRIVALED_654,
    &ASGARDIAN_INSPIRATION_678,
    &THE_VISION_AND_SCARLET_WITCH_707,
    &DEVIL_DINOSAUR_719,
    &FOGWELL_S_GYM_754,
];
pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
