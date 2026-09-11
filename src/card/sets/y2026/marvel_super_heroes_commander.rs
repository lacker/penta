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
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ObjectPredicateDef;
use crate::card::PlayerRelation;
use crate::card::PlayerRuleDef;
use crate::card::PlayerSetDef;
use crate::card::SpellCastQueryDef;
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
            EffectDef::create_token_from_copy(&crate::card::TokenCopyDef {
                object: &EffectRecipientDef::Source,
                exceptions: CopyExceptionsDef::NONE,
            }),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&COUNCIL_OF_REEDS];
pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
