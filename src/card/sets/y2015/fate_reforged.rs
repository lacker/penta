//! FRF card records required by supported formats.

use super::CardRecord;
use super::PrintingRecord;
use crate::ParentBinding;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AppliedEffectDef;
use crate::card::BindObjectsDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseDef;
use crate::card::ComparisonDef;
use crate::card::ConditionalStaticEffectDef;
use crate::card::CostDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::KeywordAbility;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectCollectionSourceDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetCountConditionDef;
use crate::card::ObjectSetDef;
use crate::card::ObjectSetFilterDef;
use crate::card::ObjectSetPredicateDef;
use crate::card::PayOrDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::PutObjectsOntoBattlefieldFaceDownDef;
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

// FRF 1 — Ugin, the Spirit Dragon
// Audit: unsupported — Loyalty activation costs are fixed signed integers; the activated-cost planner cannot choose and pay a variable −X loyalty cost.
pub(in crate::card::sets) static UGIN_THE_SPIRIT_DRAGON_1: CardRecord = CardRecord::new(
    "Ugin, the Spirit Dragon",
    "58c1e824-c8a9-4312-8e4c-a29a26d189a4",
    "Raymond Swanland",
    crate::card::CardRules::unsupported(),
);

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

// FRF 46 — Reality Shift
pub(in crate::card::sets) static REALITY_SHIFT_46: CardRecord = CardRecord::new(
    "Reality Shift",
    "e01367cb-79f4-4ed9-b12c-66f3c30264a0",
    "Howard Lyon",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[
AbilityDef::spell_with_targets("Exile target creature. Its controller manifests the top card of their library. (That player puts the top card of their library onto the battlefield face down as a 2/2 creature. If it's a creature card, it can be turned face up any time for its mana cost.)", &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Creature))], EffectDef::Sequence(&[EffectDef::move_to_zone(EffectRecipientDef::Target(TargetIndex::PRIMARY), ZoneKind::Exile, ZonePlacement::Top), EffectDef::BindObjects(BindObjectsDef { source: ObjectCollectionSourceDef::TopCards { player: PlayerRefDef::ControllerOf(ObjectRefDef::Target(TargetIndex::PRIMARY)), count: ValueDef::Constant(1) }, binding: Binding!("shift_manifest"), then: &EffectDef::PutObjectsOntoBattlefieldFaceDown(PutObjectsOntoBattlefieldFaceDownDef { input: ObjectSetDef::Binding(Binding!("shift_manifest")), controller: PlayerRefDef::ControllerOf(ObjectRefDef::Target(TargetIndex::PRIMARY)), characteristics: crate::card::face_down::manifest(), turn_up_for_mana_cost: true, moved: None, then: &EffectDef::None }) })]))
]),
);

// FRF 47 — Refocus
pub(in crate::card::sets) static REFOCUS_47: CardRecord = CardRecord::new(
    "Refocus",
    "35c78973-f2ae-4c76-802f-793d1022fcbd",
    "Kev Walker",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Untap target creature. Draw a card.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
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

// FRF 87 — Tasigur, the Golden Fang
pub(in crate::card::sets) static TASIGUR_THE_GOLDEN_FANG_87: CardRecord = CardRecord::new(
    "Tasigur, the Golden Fang",
    "81f93ac5-d149-4ccf-8b99-13ecf3190c29",
    "Chris Rahn",
    CardRules::new_creature(mana_cost!("{5}{B}"), &["Human", "Shaman"], 4, 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::delve(),
            AbilityDef::activated(
                "{2}{G/U}{G/U}: Mill two cards, then return a nonland card of an opponent's \
                 choice from your graveyard to your hand.",
                &[CostDef::Mana(mana_cost!("{2}{G/U}{G/U}"))],
                EffectDef::Sequence(&[
                    EffectDef::Mill {
                        player: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(2),
                    },
                    EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                        unchosen: None,
                        chooser: PlayerRefDef::Opponent,
                        candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                            &[ZoneKind::Graveyard],
                            PlayerRelation::You,
                        )),
                        exclude: None,
                        minimum: 1,
                        maximum: 1,
                        visibility: ChoiceVisibilityDef::Public,
                        then: &EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding)),
                            ZoneKind::Hand,
                            ZonePlacement::Top,
                        ),
                    }),
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

// FRF 141 — Temur Sabertooth
pub(in crate::card::sets) static TEMUR_SABERTOOTH_141: CardRecord = CardRecord::new(
    "Temur Sabertooth",
    "5d54da7c-8828-4d34-bfd0-a654692d3f5a",
    "Mike Sass",
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Cat"], 4, 3).with_abilities(&[
AbilityDef::activated("{1}{G}: You may return another creature you control to its owner's hand. If you do, this creature gains indestructible until end of turn.", &[CostDef::Mana(mana_cost!("{1}{G}"))], EffectDef::PayOr(PayOrDef::optional(&[CostDef::MovePermanentMatching { object: ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::Not(&ObjectPredicateDef::Source)]), zone: ZoneKind::Hand }], &EffectDef::Apply { recipient: EffectRecipientDef::Source, effect: AppliedEffectDef::add_ability(&abilities::indestructible()), duration: ResolvedEffectDurationDef::UntilEndOfTurn })))
]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &UGIN_THE_SPIRIT_DRAGON_1,
    &VALOROUS_STANCE,
    &REALITY_SHIFT_46,
    &REFOCUS_47,
    &GURMAG_ANGLER,
    &SOULFLAYER,
    &TASIGUR_THE_GOLDEN_FANG_87,
    &FLAMEWAKE_PHOENIX,
    &TEMUR_SABERTOOTH_141,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
