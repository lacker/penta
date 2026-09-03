//! Marvel Super Heroes Commander cards used for legend-rule coverage.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AppliedEffectDef, AppliedRuleDef, CardArt, CardRules, CardSet, CardSupertype,
    CardType, ComparisonDef, CopyExceptionsDef, EffectDef, EffectRecipientDef, ObjectPredicateDef,
    PlayerRelation, PlayerRuleDef, PlayerSetDef, SpellCastQueryDef, TriggerConditionDef,
    TriggerEventDef, TurnStepDef, ValueComparisonDef, ValueDef,
};
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

// MSC 28 — Council of Reeds
pub(in crate::card::sets) static COUNCIL_OF_REEDS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a0d824ea-75d2-4de5-923b-813bba44e80b"),
    "Council of Reeds",
    CardArt::new(
        "a0d824ea-75d2-4de5-923b-813bba44e80b",
        "Vlad Petruchik",
    ),
    CardSet::MarvelSuperHeroesCommander,
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
