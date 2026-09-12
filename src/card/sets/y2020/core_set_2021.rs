//! Core Set 2021 card records.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::ComparisonDef;
use crate::card::CostDef;
use crate::card::CostQuantityDef;
use crate::card::CounterKind;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRelation;
use crate::card::ReplacementEffectDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SubtypeDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "M21",
    slug: "core-set-2021",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// M21 71 — Shipwreck Dowser
pub(in crate::card::sets) static SHIPWRECK_DOWSER: CardRecord = CardRecord::new(
    "Shipwreck Dowser",
    "59d38ef7-5017-4ea3-b97f-a8fe12d03e98",
    "Caroline Gariba",
    // Five mana is a lot for a 3/3, so the card it buys back has to be the
    // reason to play it -- and prowess makes the body grow off that card.
    CardRules::new_creature(mana_cost!("{3}{U}{U}"), &["Merfolk", "Wizard"], 3, 3).with_abilities(
        &[
            abilities::prowess(),
            abilities::enters_trigger_with_targets(
                "When this creature enters, return target instant or sorcery card from your \
                 graveyard to your hand.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Instant),
                            ObjectPredicateDef::HasType(CardType::Sorcery),
                        ]),
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        // "Your graveyard" is about ownership, not who happens to
                        // control the card there.
                        owner: Some(PlayerRelation::You),
                    },
                )],
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            ),
        ],
    ),
);

// M21 121 — Sanguine Indulgence
// Audit: unsupported — Needs a self spell-cost reduction based on life gained this turn; the self-cost evaluator does not read LifeGainedThisTurn.
pub(in crate::card::sets) static SANGUINE_INDULGENCE: CardRecord = CardRecord::new(
    "Sanguine Indulgence",
    "abfcd08a-cfb5-4d34-b950-f57a88c5cb8e",
    "Andrey Kuzinskiy",
    CardRules::unsupported(),
);

// M21 126 — Village Rites
pub(in crate::card::sets) static VILLAGE_RITES: CardRecord = CardRecord::new(
    "Village Rites",
    "9c0f60a6-b5c8-4704-8b61-94e8fc463e5d",
    "Bud Cook",
    // The sacrifice is a cost rather than an effect, so it happens on the
    // way to the stack: a creature already dying to removal can be cashed
    // in before it goes.
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_additional_cost(
        "As an additional cost to cast this spell, sacrifice a creature.\nDraw two cards.",
        &[],
        CostDef::sacrifice(
            ObjectPredicateDef::HasType(CardType::Creature),
            CostQuantityDef::Fixed(1),
        ),
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(2),
        },
    )),
);

// M21 150 — Heartfire Immolator
pub(in crate::card::sets) static HEARTFIRE_IMMOLATOR: CardRecord = CardRecord::new(
    "Heartfire Immolator",
    "869fb9f1-0d59-4874-aa52-ac665c3cc0e8",
    "Donato Giancola",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Wizard"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "Prowess",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated_with_targets(
            "{R}, Sacrifice this creature: It deals damage equal to its \
             power to target creature or planeswalker.",
            &[CostDef::Mana(mana_cost!("{R}")), CostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                ]),
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::SourcePower,
            ),
        ),
    ]),
);

// M21 164 — Terror of the Peaks
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TERROR_OF_THE_PEAKS: CardRecord = CardRecord::new(
    "Terror of the Peaks",
    "432ecd5f-966f-4403-a973-51e175a524a0",
    "Andrey Kuzinskiy",
    crate::card::CardRules::unsupported(),
);

// M21 186 — Garruk's Uprising
pub(in crate::card::sets) static GARRUK_S_UPRISING: CardRecord = CardRecord::new(
    "Garruk's Uprising",
    "71a4860a-8bb6-45c0-b00a-b4a42da33ab9",
    "Wisnu Tan",
    CardRules::new_enchantment(mana_cost!("{2}{G}")).with_abilities(&[
        AbilityDef::triggered_if(
            "When this enchantment enters, if you control a creature with \
             power 4 or greater, draw a card.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
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
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
        AbilityDef::static_ability(
            "Creatures you control have trample. (Each of those creatures \
             can deal excess combat damage to the player or planeswalker \
             it's attacking.)",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::add_ability(&abilities::trample()),
            },
        ),
        AbilityDef::triggered(
            "Whenever a creature you control with power 4 or greater \
             enters, draw a card.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::PowerAtLeast(4),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
    ]),
);

// M21 193 — Llanowar Visionary
pub(in crate::card::sets) static LLANOWAR_VISIONARY: CardRecord = CardRecord::new(
    "Llanowar Visionary",
    "d6e23afa-7e08-4049-baf0-d4d0134ba2c8",
    "Cristi Balanescu",
    // A mana dork that costs three is only playable because it replaces
    // itself first, so the ramp is pure profit if it survives.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Elf", "Druid"], 2, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, draw a card.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add {G}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
        ),
    ]),
);

// M21 197 — Primal Might
pub(in crate::card::sets) static PRIMAL_MIGHT: CardRecord = CardRecord::new(
    "Primal Might",
    "1cd8cee8-7ea0-4037-8a3b-39334dc064fb",
    "Randy Vargas",
    CardRules::new_sorcery(mana_cost!("{X}{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature you control gets +X/+X until end of turn. \
         Then it fights up to one target creature you don't control. \
         (Each deals damage equal to its power to the other.)",
        &[
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            }),
            AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::NotYou),
                    owner: None,
                },
                1,
            ),
        ],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::ChosenX,
                    ValueDef::ChosenX,
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::Fight {
                first: ObjectRefDef::Target(TargetIndex::PRIMARY),
                second: ObjectRefDef::Target(TargetIndex(1)),
                excess: None,
            },
        ]),
    )]),
);

// M21 214 — Wildwood Scourge
pub(in crate::card::sets) static WILDWOOD_SCOURGE: CardRecord = CardRecord::new(
    "Wildwood Scourge",
    "46ff0b33-d153-4b0e-ac48-7e5ed70dea09",
    "Bryan Sola",
    CardRules::new_creature(mana_cost!("{X}{G}"), &["Hydra"], 0, 0).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters with X +1/+1 counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCastXCounters {
                    kind: CounterKind::PlusOnePlusOne,
                },
            ),
        ),
        AbilityDef::triggered(
            "Whenever one or more +1/+1 counters are put on another \
             non-Hydra creature you control, put a +1/+1 counter on this \
             creature.",
            TriggerEventDef::CountersPlaced {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype(SubtypeDef::Literal(
                            "Hydra",
                        ))),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                kind: CounterKind::PlusOnePlusOne,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// M21 232 — Mazemind Tome
// Audit: unsupported — Needs a state trigger for crossing a counter threshold, with suppression while that trigger is on the stack and an actual exile-result condition before gaining life.
pub(in crate::card::sets) static MAZEMIND_TOME: CardRecord = CardRecord::new(
    "Mazemind Tome",
    "9fd761f3-6b43-4150-8595-dc3abd85b06c",
    "Randy Gallegos",
    CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &SHIPWRECK_DOWSER,
    &SANGUINE_INDULGENCE,
    &VILLAGE_RITES,
    &HEARTFIRE_IMMOLATOR,
    &TERROR_OF_THE_PEAKS,
    &GARRUK_S_UPRISING,
    &LLANOWAR_VISIONARY,
    &PRIMAL_MIGHT,
    &WILDWOOD_SCOURGE,
    &MAZEMIND_TOME,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
