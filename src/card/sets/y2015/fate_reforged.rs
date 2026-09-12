//! FRF card records required by supported formats.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AppliedEffectDef;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::ComparisonDef;
use crate::card::ConditionalStaticEffectDef;
use crate::card::CostDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::KeywordAbility;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectSetCountConditionDef;
use crate::card::ObjectSetDef;
use crate::card::ObjectSetFilterDef;
use crate::card::ObjectSetPredicateDef;
use crate::card::PayOrDef;
use crate::card::PlayerRelation;
use crate::card::ResolvedEffectDurationDef;
use crate::card::StaticApplyDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "FRF",
    slug: "fate-reforged",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// FRF 28 — Valorous Stance
pub(in crate::card::sets) static VALOROUS_STANCE: CardRecord = CardRecord::new(
    "Valorous Stance",
    "65998e94-15a0-41f1-8288-730b957f81df",
    "Willian Murai",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Target creature gains indestructible until end of turn. \
                 (Damage and effects that say \"destroy\" don't destroy it.)",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::spell_with_targets(
                "Destroy target creature with toughness 4 or greater.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ToughnessGreaterThan(ValueDef::Constant(3)),
                    ]),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            ),
        ],
    )]),
);

// FRF 72 — Gurmag Angler
pub(in crate::card::sets) static GURMAG_ANGLER: CardRecord = CardRecord::new(
    "Gurmag Angler",
    "c60a8cf1-a8c7-4f45-bbd3-188fab2652f9",
    "YW Tang",
    // Printed at seven and cast for one, which is why a deck that fills its
    // own graveyard treats the mana cost as a formality.
    CardRules::new_creature(mana_cost!("{6}{B}"), &["Zombie", "Fish"], 5, 5)
        .with_ability(abilities::delve()),
);

// FRF 84 — Soulflayer
const fn soulflayer_ability(keyword: KeywordAbility, ability: &'static AbilityDef) -> EffectDef {
    EffectDef::ConditionalStatic(ConditionalStaticEffectDef {
        condition: ObjectSetCountConditionDef {
            objects: &ObjectSetDef::Matching {
                objects: &ObjectSetDef::LinkedExiles,
                object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::HasType(
                    CardType::Creature,
                )),
            },
            predicate: ObjectSetPredicateDef {
                filter: Some(ObjectSetFilterDef::HasKeyword(keyword)),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
        },
        then: StaticApplyDef {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::add_ability(ability),
        },
    })
}

pub(in crate::card::sets) static SOULFLAYER: CardRecord = CardRecord::new(
    "Soulflayer",
    "5084c8ff-1296-4d8e-bd06-93b1a3401661",
    "Seb McKinnon",
CardRules::new_creature(mana_cost!("{4}{B}{B}"), &["Demon"], 4, 4).with_abilities(&[
        abilities::delve(),
        AbilityDef::static_ability(
            "If a creature card with flying was exiled with delve to cast this creature, this creature has flying. The same is true for first strike, double strike, deathtouch, haste, hexproof, indestructible, lifelink, reach, trample, and vigilance.",
            EffectDef::Sequence(&[
                soulflayer_ability(KeywordAbility::Flying, &abilities::flying()),
                soulflayer_ability(KeywordAbility::FirstStrike, &abilities::first_strike()),
                soulflayer_ability(KeywordAbility::DoubleStrike, &abilities::double_strike()),
                soulflayer_ability(KeywordAbility::Deathtouch, &abilities::deathtouch()),
                soulflayer_ability(KeywordAbility::Haste, &abilities::haste()),
                soulflayer_ability(KeywordAbility::Hexproof, &abilities::hexproof()),
                soulflayer_ability(KeywordAbility::Indestructible, &abilities::indestructible()),
                soulflayer_ability(KeywordAbility::Lifelink, &abilities::lifelink()),
                soulflayer_ability(KeywordAbility::Reach, &abilities::reach()),
                soulflayer_ability(KeywordAbility::Trample, &abilities::trample()),
                soulflayer_ability(KeywordAbility::Vigilance, &abilities::vigilance()),
            ]),
        ),
    ]),
);

// FRF 100 — Flamewake Phoenix
pub(in crate::card::sets) static FLAMEWAKE_PHOENIX: CardRecord = CardRecord::new(
    "Flamewake Phoenix",
    "fefd5848-9fe1-4129-a5d7-e51606bf76ef",
    "Min Yum",
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Phoenix"], 2, 2).with_abilities(&[
        abilities::flying(),
        abilities::haste(),
        abilities::attacks_each_combat_if_able(),
        AbilityDef::triggered_if(
            "Ferocious — At the beginning of combat on your turn, if you \
             control a creature with power 4 or greater, you may pay {R}. \
             If you do, return this card from your graveyard to the \
             battlefield.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::PowerAtLeast(4),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::Mana(mana_cost!("{R}"))],
                &EffectDef::move_to_zone(
                    EffectRecipientDef::Source,
                    ZoneKind::Battlefield,
                    ZonePlacement::Top,
                ),
            )),
        )
        .with_source_zones(&[ZoneKind::Graveyard]),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &VALOROUS_STANCE,
    &GURMAG_ANGLER,
    &SOULFLAYER,
    &FLAMEWAKE_PHOENIX,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
