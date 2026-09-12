//! Duskmourn: House of Horror card inventory.

use crate::card::GraveyardPlayPermissionDef;
use crate::card::PlayActionMatcherDef;
use crate::card::PlayRestrictionDef;
use crate::card::TokenCountersDef;
use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityPredicateDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::ActivationTimingDef;
use crate::card::AddManaEffectDef;
use crate::card::AggregateOperationDef;
use crate::card::AlternativeCastKindDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::BasicLandType;
use crate::card::BattlefieldArrivalDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::BattlefieldEntryScalarChoiceDef;
use crate::card::BindObjectsDef;
use crate::card::CardArt;
use crate::card::CardComposition;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CardTypeSet;
use crate::card::ChangeStackTargetsDef;
use crate::card::CharacteristicOperationDef;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseCardsFromCollectionDef;
use crate::card::ChooseDef;
use crate::card::ChooseForEachPlayerDef;
use crate::card::CollectionInspectionDef;
use crate::card::ColorSet;
use crate::card::ComparisonDef;
use crate::card::ControlDurationDef;
use crate::card::CopyStackObjectDef;
use crate::card::CostDef;
use crate::card::CostModificationDef;
use crate::card::CostQuantityDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::CreatureStats;
use crate::card::CreatureTypeSetDef;
use crate::card::DamageAssignmentDef;
use crate::card::DamageDef;
use crate::card::DamageEventMatcherDef;
use crate::card::DamageKindDef;
use crate::card::DamageRecipientMatcherDef;
use crate::card::DamageSourceMatcherDef;
use crate::card::DestroyFollowUpDef;
use crate::card::DiscardFollowUpDef;
use crate::card::DiscardSelectionDef;
use crate::card::DividedTotal;
use crate::card::DrawEventMatcherDef;
use crate::card::EffectChoiceDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::EmblemCharacteristics;
use crate::card::ExilePlayDurationDef;
use crate::card::GameActionDef;
use crate::card::HalvedValueDef;
use crate::card::IfNoObjectsDef;
use crate::card::InstalledTriggerDef;
use crate::card::KeywordAbility;
use crate::card::ManaColor;
use crate::card::ManaTypeDef;
use crate::card::MoveObjectsDef;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectCollectionSourceDef;
use crate::card::ObjectCounterValueDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetCountConditionDef;
use crate::card::ObjectSetDef;
use crate::card::ObjectSetFilterDef;
use crate::card::ObjectSetPredicateDef;
use crate::card::ObjectValueAggregateDef;
use crate::card::ObjectValueDef;
use crate::card::PayOrDef;
use crate::card::PerPlayerSelectionDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::PlayerRuleDef;
use crate::card::PlayerSetDef;
use crate::card::PutObjectsOntoBattlefieldFaceDownDef;
use crate::card::RandomizeObjectOrderDef;
use crate::card::ReplacementChoiceDef;
use crate::card::ReplacementConditionDef;
use crate::card::ReplacementEffectDef;
use crate::card::ReplacementEventDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::RevealObjectsDef;
use crate::card::RoundingDef;
use crate::card::SetOperationDef;
use crate::card::StackTargetChangeDef;
use crate::card::SubtypeDef;
use crate::card::SumValueDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnPhaseDef;
use crate::card::TurnStepDef;
use crate::card::ValueComparisonDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::ids::ParentBinding;
use crate::mana_cost;

use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y1995::ice_age as catalog_ice;
use crate::card::sets::y2006::guildpact as catalog_gpt;
use crate::card::sets::y2006::time_spiral as catalog_tsp;
use crate::card::sets::y2012::magic_2013 as catalog_m13;
use crate::card::sets::y2012::return_to_ravnica as catalog_rtr;
use crate::card::sets::y2019::throne_of_eldraine as catalog_eld;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "DSK",
    slug: "duskmourn-house-of-horror",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

const fn impending_cast(costs: &'static [CostDef], text: &'static str) -> AbilityDef {
    AbilityDef::alternative_cast(
        costs,
        AlternativeCastKindDef::Impending,
        Some(text),
        EffectDef::None,
    )
}
const fn impending_entry(amount: u16) -> AbilityDef {
    AbilityDef::as_enters_if(
        "If you chose to pay this permanent's impending cost, it \
         enters with time counters.",
        ReplacementConditionDef::SourceCastWith(AlternativeCastKindDef::Impending),
        ReplacementEffectDef::ModifyBattlefieldEntry(
            BattlefieldEntryModificationDef::AddCounters {
                kind: CounterKind::named("time"),
                amount,
            },
        ),
    )
}
const fn impending_countdown() -> AbilityDef {
    const CONDITION: TriggerConditionDef = TriggerConditionDef::All(&[
        TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Impending),
        TriggerConditionDef::SourceCounters {
            kind: CounterKind::named("time"),
            comparison: ComparisonDef::GreaterOrEqual,
            amount: 1,
        },
    ]);
    AbilityDef::triggered_if(
        "At the beginning of your end step, if this permanent's \
         impending cost was paid and it has a time counter on it, \
         remove a time counter from it.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::End,
            player: PlayerRelation::You,
        },
        &CONDITION,
        EffectDef::RemoveCounters {
            object: EffectRecipientDef::Source,
            kind: CounterKind::named("time"),
            amount: ValueDef::Constant(1),
        },
    )
}

const TREASURE_TOKEN: TokenCharacteristics = crate::card::tokens::treasure().with_art(
    CardArt::new("5c0b31c3-5775-41a6-9981-44fc4a6d4aa8", "Michele Giorgi"),
);

const DEMON_TOKEN: TokenCharacteristics =
    TokenCharacteristics::creature(&["Demon"], &[ManaColor::Black], 6, 6)
        .with_abilities(&[abilities::flying()])
        .with_art(CardArt::new(
            "bba307eb-814c-4c87-acdf-b54c87d04f82",
            "Josu Solano",
        ));
const GREMLIN_TOKEN: TokenCharacteristics =
    TokenCharacteristics::creature(&["Gremlin"], &[ManaColor::Red], 1, 1).with_art(CardArt::new(
        "d948b503-890a-49d5-a3cf-cb6e604851b8",
        "Joseph Weston",
    ));
const INSECT_TOKEN: TokenCharacteristics =
    TokenCharacteristics::creature(&["Insect"], &[ManaColor::Black, ManaColor::Green], 1, 1)
        .with_abilities(&[abilities::flying()])
        .with_art(CardArt::new(
            "377f1a20-b270-4b07-9892-7170cd0bee38",
            "Helge C. Balzer",
        ));

// DSK 1 — Acrobatic Cheerleader
// Audit: unsupported — Needs the ordinal number of the current main phase in the turn (CR 505.1b); PostcombatMain also matches third and later main phases, so it cannot restrict survival to the second main phase.
pub(in crate::card::sets) static ACROBATIC_CHEERLEADER: CardRecord = CardRecord::new(
    "Acrobatic Cheerleader",
    "6f1a7590-3eee-4803-b192-d4fb771e6a86",
    "Julia Metzger",
    CardRules::unsupported(),
);

// DSK 2 — Cult Healer
// Audit: unsupported — Needs a committed event for a Room becoming fully unlocked, observed by other permanents and graveyard abilities. DoorUnlocked only dispatches to the door being opened, so it cannot implement the second eerie trigger.
pub(in crate::card::sets) static CULT_HEALER: CardRecord = CardRecord::new(
    "Cult Healer",
    "9c9b8fbe-8a5e-4b62-b53f-9ead8147bbbb",
    "Diana Franco",
    CardRules::unsupported(),
);

// DSK 3 — Dazzling Theater // Prop Room
// Audit: unsupported — Needs a continuous grant of convoke to creature spells on the stack; executable spell ability grants are outside the supported static recipient boundary.
pub(in crate::card::sets) static DAZZLING_THEATER: CardRecord = CardRecord::new(
    "Dazzling Theater // Prop Room",
    "8e2fae80-60af-44cf-95b4-177837435d1a",
    "Henry Peters",
    CardRules::unsupported(),
);

// DSK 4 — Dollmaker's Shop // Porcelain Gallery
// Audit: unsupported — Needs a batched attack-declaration matcher restricted to attacks against a player; current batched declarations count creatures attacking planeswalkers too, while the player-only matcher fires separately for each creature.
pub(in crate::card::sets) static DOLLMAKER_S_SHOP: CardRecord = CardRecord::new(
    "Dollmaker's Shop // Porcelain Gallery",
    "c5ee6651-9946-4bae-b21e-6cf28fa77b13",
    "Chris Cold",
    CardRules::unsupported(),
);

// DSK 5 — Emerge from the Cocoon
pub(in crate::card::sets) static EMERGE_FROM_THE_COCOON: CardRecord = CardRecord::new(
    "Emerge from the Cocoon",
    "3ba16fbf-2d44-4337-87cb-6ff6b84a258a",
    "Marta Nael",
    CardRules::new_sorcery(mana_cost!("{4}{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Return target creature card from your graveyard to the \
         battlefield. You gain 3 life.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Battlefield,
                ZonePlacement::Top,
            ),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ]),
    )]),
);

// DSK 6 — Enduring Innocence
// Audit: unsupported — Needs a return-to-battlefield instruction that establishes the permanent as a noncreature enchantment before entry replacement effects and enters triggers inspect it; applying a type change after the return produces an incorrect creature entry.
pub(in crate::card::sets) static ENDURING_INNOCENCE: CardRecord = CardRecord::new(
    "Enduring Innocence",
    "08f79439-b8f8-418f-9772-26d81844749e",
    "Liiga Smilshkalne",
    CardRules::unsupported(),
);

// DSK 7 — Ethereal Armor (reprint)
const ETHEREAL_ARMOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rtr::ETHEREAL_ARMOR,
    "fd0e8a82-0201-483e-a976-5f29764b35a4",
    "Tyler Walpole",
);

// DSK 8 — Exorcise
pub(in crate::card::sets) static EXORCISE: CardRecord = CardRecord::new(
    "Exorcise",
    "f49006f2-a097-417d-8eb0-b8016ff2e0d5",
    "Dominik Mayer",
    CardRules::new_sorcery(mana_cost!("{1}{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Exile target artifact, enchantment, or creature with power 4 \
         or greater.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Enchantment),
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::PowerAtLeast(4),
                ]),
            ]),
        )],
        EffectDef::move_to_zone(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ZoneKind::Exile,
            ZonePlacement::Top,
        ),
    )]),
);

// DSK 9 — Fear of Abduction
// Audit: unsupported — Needs the exiled battlefield card paid as an additional casting cost to become part of the permanent's linked exile group; casting cost facts do not establish those linked identities for the leaves trigger.
pub(in crate::card::sets) static FEAR_OF_ABDUCTION: CardRecord = CardRecord::new(
    "Fear of Abduction",
    "fc9374be-5e4b-4c23-8b6e-94c03d4f5ef1",
    "Fernando Falcone",
    CardRules::unsupported(),
);

// DSK 10 — Fear of Immobility
pub(in crate::card::sets) static FEAR_OF_IMMOBILITY: CardRecord = CardRecord::new(
    "Fear of Immobility",
    "9220c8fa-6ef7-4bc1-acb9-fc54cc43e498",
    "Martin de Diego Sádaba",
    CardRules::new_enchantment_creature(mana_cost!("{4}{W}"), &["Nightmare"], 4, 4).with_abilities(
        &[abilities::enters_trigger_with_targets(
            "When this creature enters, tap up to one target creature. If \
             an opponent controls that creature, put a stun counter on it. \
             (If a permanent with a stun counter would become untapped, \
             remove one from it instead.)",
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                1,
            )],
            EffectDef::Sequence(&[
                EffectDef::Tap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::TargetMatches {
                        slot: TargetIndex::PRIMARY,
                        object: ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
                    },
                    then: &EffectDef::AddCounters {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        kind: CounterKind::Stun,
                        amount: ValueDef::Constant(1),
                    },
                },
            ]),
        )],
    ),
);

// DSK 11 — Fear of Surveillance
pub(in crate::card::sets) static FEAR_OF_SURVEILLANCE: CardRecord = CardRecord::new(
    "Fear of Surveillance",
    "6e98559d-d9fd-4bdf-8df9-389eee756a3a",
    "Jana Heidersdorf",
    CardRules::new_enchantment_creature(mana_cost!("{1}{W}"), &["Nightmare"], 2, 2).with_abilities(
        &[
            abilities::vigilance(),
            AbilityDef::triggered(
                "Whenever this creature attacks, surveil 1. (Look at the top \
                 card of your library. You may put it into your graveyard.)",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                abilities::surveil(ValueDef::Constant(1)),
            ),
        ],
    ),
);

// DSK 12 — Friendly Ghost
pub(in crate::card::sets) static FRIENDLY_GHOST: CardRecord = CardRecord::new(
    "Friendly Ghost",
    "20e0dba2-d15c-4d55-9b68-00648175e760",
    "Sean Murray",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Spirit"], 2, 4).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, target creature gets +2/+4 until \
             end of turn.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(4),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// DSK 13 — Ghostly Dancers
// Audit: unsupported — Needs a resolving effect that selects and unlocks a locked Room door, plus a fully-unlocked event observed by other abilities; only the ordinary unlock special action and local door triggers are available.
pub(in crate::card::sets) static GHOSTLY_DANCERS: CardRecord = CardRecord::new(
    "Ghostly Dancers",
    "ab38adb5-8f16-4a4a-8dbc-f6ec14ca6c9f",
    "Josh Newton",
    CardRules::unsupported(),
);

// DSK 14 — Glimmer Seeker
// Audit: unsupported — Needs the ordinal number of the current main phase in the turn (CR 505.1b); PostcombatMain also matches third and later main phases, so it cannot restrict survival to the second main phase.
pub(in crate::card::sets) static GLIMMER_SEEKER: CardRecord = CardRecord::new(
    "Glimmer Seeker",
    "7f06bbb1-0c7d-4803-9b35-8a2206803eed",
    "Kev Fang",
    CardRules::unsupported(),
);

// DSK 15 — Grand Entryway // Elegant Rotunda
pub(in crate::card::sets) static GRAND_ENTRYWAY: CardRecord = CardRecord::new(
    "Grand Entryway // Elegant Rotunda",
    "ef8a8ba7-f955-425d-9a16-816fc48a2a83",
    "Carlos Palma Cruchaga",
    CardRules::new_enchantment(mana_cost!("{1}{W}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::triggered(
            "When you unlock this door, create a 1/1 white Glimmer \
             enchantment creature token.",
            TriggerEventDef::DoorUnlocked,
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::new(
                    CardTypeSet::single(CardType::Enchantment).with(CardType::Creature),
                    &["Glimmer"],
                    &[ManaColor::White],
                    Some(CreatureStats {
                        power: 1,
                        toughness: 1,
                    }),
                ),
            ))),
        )]),
)
.with_composition(|| {
    const FRONT: CardRules = CardRules::new_enchantment(mana_cost!("{1}{W}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::triggered(
            "When you unlock this door, create a 1/1 white Glimmer \
             enchantment creature token.",
            TriggerEventDef::DoorUnlocked,
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::new(
                    CardTypeSet::single(CardType::Enchantment).with(CardType::Creature),
                    &["Glimmer"],
                    &[ManaColor::White],
                    Some(CreatureStats {
                        power: 1,
                        toughness: 1,
                    }),
                ),
            ))),
        )]);
    const BACK: CardRules = CardRules::new_enchantment(mana_cost!("{2}{W}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::triggered_with_targets(
            "When you unlock this door, put a +1/+1 counter on each of up \
             to two target creatures.",
            TriggerEventDef::DoorUnlocked,
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                2,
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        )]);
    const BOTH: CardRules = CardRules::new_enchantment(mana_cost!("{3}{W}{W}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[
            AbilityDef::triggered(
                "When you unlock this door, create a 1/1 white Glimmer \
                 enchantment creature token.",
                TriggerEventDef::DoorUnlocked,
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                    TokenCharacteristics::new(
                        CardTypeSet::single(CardType::Enchantment).with(CardType::Creature),
                        &["Glimmer"],
                        &[ManaColor::White],
                        Some(CreatureStats {
                            power: 1,
                            toughness: 1,
                        }),
                    ),
                ))),
            ),
            AbilityDef::triggered_with_targets(
                "When you unlock this door, put a +1/+1 counter on each of up \
                 to two target creatures.",
                TriggerEventDef::DoorUnlocked,
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    2,
                )],
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]);
    CardComposition::room(
        "Grand Entryway // Elegant Rotunda",
        "Grand Entryway",
        FRONT,
        "Elegant Rotunda",
        BACK,
        BOTH,
    )
});

// DSK 16 — Hardened Escort
pub(in crate::card::sets) static HARDENED_ESCORT: CardRecord = CardRecord::new(
    "Hardened Escort",
    "12054e10-46d6-4581-a7e5-ff277cb0e229",
    "John Stanko",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Soldier"], 2, 4).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks, another target creature you \
             control gets +1/+0 and gains indestructible until end of \
             turn. (Damage and effects that say \"destroy\" don't destroy \
             it.)",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
                    AppliedEffectDef::add_ability(&abilities::indestructible()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// DSK 17 — Jump Scare
pub(in crate::card::sets) static JUMP_SCARE: CardRecord = CardRecord::new(
    "Jump Scare",
    "a2dd8903-31ca-470d-b2ff-280f6d40c794",
    "John Tedrick",
    CardRules::new_instant(mana_cost!("{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Until end of turn, target creature gets +2/+2, gains flying, \
         and becomes a Horror enchantment creature in addition to its \
         other types.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                AppliedEffectDef::add_ability(&abilities::flying()),
                AppliedEffectDef::add_card_types(
                    CardTypeSet::single(CardType::Enchantment).with(CardType::Creature),
                ),
                AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Horror"])),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// DSK 18 — Leyline of Hope
pub(in crate::card::sets) static LEYLINE_OF_HOPE: CardRecord = CardRecord::new(
    "Leyline of Hope",
    "40960e47-3065-485e-aede-29a62411034e",
    "Sergey Glushakov",
CardRules::new_enchantment(mana_cost!("{2}{W}{W}")).with_abilities(&[
        abilities::begin_game_on_battlefield(),
        AbilityDef::replacement_for(
            "If you would gain life, you gain that much life plus 1 instead.",
            ReplacementEventDef::WouldGainLife(PlayerRelation::You),
            ReplacementEffectDef::AddToEventAmount(1),
        ),
        AbilityDef::static_ability(
            "As long as you have at least 7 life more than your starting life total, creatures you control get +2/+2.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::LifeTotal(PlayerRelation::You),
                    comparison: ComparisonDef::GreaterOrEqual,
                    right: ValueDef::Sum(&SumValueDef::new(
                        ValueDef::StartingLifeTotal,
                        ValueDef::Constant(7),
                    )),
                }),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                },
            },
        ),
    ]),
);

// DSK 19 — Lionheart Glimmer
pub(in crate::card::sets) static LIONHEART_GLIMMER: CardRecord = CardRecord::new(
    "Lionheart Glimmer",
    "483e1c6f-331c-45f1-bf5d-9b9742aa8903",
    "Josu Hernaiz",
    CardRules::new_enchantment_creature(mana_cost!("{3}{W}{W}"), &["Cat", "Glimmer"], 2, 5)
        .with_abilities(&[
            abilities::ward(&[CostDef::Mana(mana_cost!("{2}"))], "Ward {2}"),
            AbilityDef::triggered(
                "Whenever you attack, creatures you control get +1/+1 until \
                 end of turn.",
                TriggerEventDef::attack_declared(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    1,
                    None,
                ),
                EffectDef::Apply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// DSK 20 — Living Phone
pub(in crate::card::sets) static LIVING_PHONE: CardRecord = CardRecord::new(
    "Living Phone",
    "8266f93b-f91c-4427-a222-075ceb0be3af",
    "Domenico Cava",
    CardRules::new_artifact_creature(mana_cost!("{2}{W}"), &["Toy"], 2, 1).with_abilities(&[
        abilities::dies_trigger(
            "When this creature dies, look at the top five cards of your \
             library. You may reveal a creature card with power 2 or less \
             from among them and put it into your hand. Put the rest on \
             the bottom of your library in a random order.",
            EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                source: ObjectCollectionSourceDef::TopCards {
                    player: PlayerRefDef::EffectController,
                    count: ValueDef::Constant(5),
                },
                actor: PlayerRefDef::EffectController,
                inspection: CollectionInspectionDef::Look,
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::PowerLessThan(ValueDef::Constant(3)),
                ]),
                minimum: 0,
                maximum: 1,
                chosen: crate::Binding!("chosen"),
                remainder: crate::Binding!("rest"),
                then: &EffectDef::Sequence(&[
                    EffectDef::RevealObjects(RevealObjectsDef {
                        input: ObjectSetDef::Binding(crate::Binding!("chosen")),
                        then: &EffectDef::None,
                    }),
                    EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                            "chosen"
                        ))),
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                    EffectDef::RandomizeObjectOrder(RandomizeObjectOrderDef {
                        input: ObjectSetDef::Binding(crate::Binding!("rest")),
                        randomized: crate::Binding!("random_bottom"),
                        then: &EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                                "random_bottom"
                            ))),
                            ZoneKind::Library,
                            ZonePlacement::Bottom,
                        ),
                    }),
                ]),
            }),
        ),
    ]),
);

// DSK 21 — Optimistic Scavenger
// Audit: unsupported — Needs a committed event for a Room becoming fully unlocked, observed by other permanents and graveyard abilities. DoorUnlocked only dispatches to the door being opened, so it cannot implement the second eerie trigger.
pub(in crate::card::sets) static OPTIMISTIC_SCAVENGER: CardRecord = CardRecord::new(
    "Optimistic Scavenger",
    "9c72fc6f-6a96-420d-812d-b5cf0f57cc7f",
    "Brian Valeza",
    CardRules::unsupported(),
);

// DSK 22 — Orphans of the Wheat
// Audit: unsupported — Needs an optional multi-permanent tap effect with a result binding containing only permanents actually tapped, so prevented or replaced taps do not increase the bonus.
pub(in crate::card::sets) static ORPHANS_OF_THE_WHEAT: CardRecord = CardRecord::new(
    "Orphans of the Wheat",
    "8ef4aab1-8bc0-4652-91d7-3e8b20b411ad",
    "Julie Dillon",
    CardRules::unsupported(),
);

// DSK 23 — Overlord of the Mistmoors (alternate printing)
const OVERLORD_OF_THE_MISTMOORS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OVERLORD_OF_THE_MISTMOORS,
    1,
    "6bcafc2e-cef6-412d-8c5d-1658c3337292",
    "Steven Belledin",
);

// DSK 24 — Patched Plaything
// Audit: unsupported — Needs a positive cast-from-hand condition for a prospective entry replacement; the exposed entry condition is SourceNotCastFrom and has no composable negation.
pub(in crate::card::sets) static PATCHED_PLAYTHING: CardRecord = CardRecord::new(
    "Patched Plaything",
    "513da431-4f3a-4f4a-8be4-7e162dd93307",
    "Domenico Cava",
    CardRules::unsupported(),
);

// DSK 25 — Possessed Goat
pub(in crate::card::sets) static POSSESSED_GOAT: CardRecord = CardRecord::new(
    "Possessed Goat",
    "cd02b8ff-ff65-4a38-b8fb-f8dd130edbf7",
    "Edgar Sánchez Hidalgo",
    CardRules::new_creature(mana_cost!("{W}"), &["Goat"], 1, 1).with_abilities(&[
        AbilityDef::activated(
            "{3}, Discard a card: Put three +1/+1 counters on this \
             creature and it becomes a black Demon in addition to its \
             other colors and types. Activate only once.",
            &[
                CostDef::Mana(mana_cost!("{3}")),
                CostDef::discard(ObjectPredicateDef::Any),
            ],
            EffectDef::Sequence(&[
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(3),
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_colors(ColorSet::from_colors(&[ManaColor::Black])),
                        AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Demon"])),
                    ]),
                    duration: ResolvedEffectDurationDef::Permanent,
                },
            ]),
        )
        .once_per_object(),
    ]),
);

// DSK 26 — Reluctant Role Model
// Audit: unsupported — Needs the ordinal number of the current main phase in the turn (CR 505.1b); PostcombatMain also matches third and later main phases, so it cannot restrict survival to the second main phase.
pub(in crate::card::sets) static RELUCTANT_ROLE_MODEL: CardRecord = CardRecord::new(
    "Reluctant Role Model",
    "4dde86d6-34a0-4b3b-a46a-d9941501d08c",
    "Chris Rallis",
    CardRules::unsupported(),
);

// DSK 27 — Savior of the Small
// Audit: unsupported — Needs the ordinal number of the current main phase in the turn (CR 505.1b); PostcombatMain also matches third and later main phases, so it cannot restrict survival to the second main phase.
pub(in crate::card::sets) static SAVIOR_OF_THE_SMALL: CardRecord = CardRecord::new(
    "Savior of the Small",
    "e2ed31ea-c278-4e8c-afa7-6d09af399345",
    "Elizabeth Peiró",
    CardRules::unsupported(),
);

// DSK 28 — Seized from Slumber
// Audit: unsupported — Needs a self spell-cost reduction based on the selected target's tapped state; the self-cost evaluator cannot read casting target choices.
pub(in crate::card::sets) static SEIZED_FROM_SLUMBER: CardRecord = CardRecord::new(
    "Seized from Slumber",
    "24456083-0c76-46c5-9b18-8d468702df69",
    "Miranda Meeks",
    CardRules::unsupported(),
);

// DSK 29 — Shardmage's Rescue
pub(in crate::card::sets) static SHARDMAGE_S_RESCUE: CardRecord = CardRecord::new(
    "Shardmage's Rescue",
    "aed0cafa-d701-4e1f-9773-abcf817c244c",
    "Jarel Threat",
    CardRules::new_enchantment(mana_cost!("{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::flash(),
            abilities::aura_spell(
                "Enchant creature you control",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
            ),
            AbilityDef::static_ability(
                "As long as this Aura entered this turn, enchanted creature \
                 has hexproof.",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::SourceMatches {
                        object: ObjectPredicateDef::EnteredThisTurn,
                    },
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::add_ability(&abilities::hexproof()),
                    },
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +1/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                },
            ),
        ]),
);

// DSK 30 — Sheltered by Ghosts
// Audit: unsupported — Needs an exile-until-source-leaves duration with immediate return when that duration ends (CR 610.3); an ordinary leaves trigger returns the card later through the stack.
pub(in crate::card::sets) static SHELTERED_BY_GHOSTS: CardRecord = CardRecord::new(
    "Sheltered by Ghosts",
    "389f3f7b-be40-4a2d-b5cc-28471a577981",
    "Mirko Failoni",
    CardRules::unsupported(),
);

// DSK 31 — Shepherding Spirits
pub(in crate::card::sets) static SHEPHERDING_SPIRITS: CardRecord = CardRecord::new(
    "Shepherding Spirits",
    "86c02ab1-0fb2-4cb0-a871-bfad406726fc",
    "Billy Christian",
    CardRules::new_creature(mana_cost!("{4}{W}{W}"), &["Spirit"], 4, 5).with_abilities(&[
        abilities::flying(),
        abilities::typecycling!(
            "Plainscycling {2}",
            &[CostDef::Mana(mana_cost!("{2}"))],
            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Plains"))
        ),
    ]),
);

// DSK 32 — Split Up
pub(in crate::card::sets) static SPLIT_UP: CardRecord = CardRecord::new(
    "Split Up",
    "1bb9a5b1-48d2-453e-8991-c788bc63e482",
    "Dominik Mayer",
    CardRules::new_sorcery(mana_cost!("{1}{W}{W}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell(
                "Destroy all tapped creatures.",
                EffectDef::Destroy {
                    object: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Tapped,
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Any,
                        ),
                    )),
                    then: None,
                },
            ),
            AbilityDef::spell(
                "Destroy all untapped creatures.",
                EffectDef::Destroy {
                    object: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Tapped),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Any,
                        ),
                    )),
                    then: None,
                },
            ),
        ],
    )]),
);

// DSK 33 — Splitskin Doll
pub(in crate::card::sets) static SPLITSKIN_DOLL: CardRecord = CardRecord::new(
    "Splitskin Doll",
    "0453be94-e59b-48f1-a488-7a9fd96a627e",
    "Diana Franco",
    CardRules::new_artifact_creature(mana_cost!("{1}{W}"), &["Toy"], 2, 1).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, draw a card. Then discard a card \
             unless you control another creature with power 2 or less.",
            EffectDef::Sequence(&[
                abilities::draw_cards(ValueDef::Constant(1)),
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::Not(&TriggerConditionDef::ObjectCount {
                        query: ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                                ObjectPredicateDef::PowerLessThan(ValueDef::Constant(3)),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                        comparison: ComparisonDef::GreaterOrEqual,
                        amount: 1,
                    }),
                    then: &EffectDef::Discard {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                        selection: DiscardSelectionDef::RecipientChooses,
                        then: None,
                    },
                },
            ]),
        ),
    ]),
);

// DSK 34 — Surgical Suite // Hospital Room
pub(in crate::card::sets) static SURGICAL_SUITE: CardRecord = CardRecord::new(
    "Surgical Suite // Hospital Room",
    "96b2dc14-4477-4444-a9eb-4fa4c02dfbde",
    "Titus Lunter",
    CardRules::new_enchantment(mana_cost!("{1}{W}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::triggered_with_targets(
            "When you unlock this door, return target creature card with \
             mana value 3 or less from your graveyard to the battlefield.",
            TriggerEventDef::DoorUnlocked,
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ManaValueAtMost(3),
                    ]),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Battlefield,
                ZonePlacement::Top,
            ),
        )]),
)
.with_composition(|| {
    const FRONT: CardRules = CardRules::new_enchantment(mana_cost!("{1}{W}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::triggered_with_targets(
            "When you unlock this door, return target creature card with \
             mana value 3 or less from your graveyard to the battlefield.",
            TriggerEventDef::DoorUnlocked,
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ManaValueAtMost(3),
                    ]),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Battlefield,
                ZonePlacement::Top,
            ),
        )]);
    const BACK: CardRules = CardRules::new_enchantment(mana_cost!("{3}{W}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::triggered_with_targets(
            "Whenever you attack, put a +1/+1 counter on target attacking \
             creature.",
            TriggerEventDef::attack_declared(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                1,
                None,
            ),
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Attacking,
                ]),
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        )]);
    const BOTH: CardRules = CardRules::new_enchantment(mana_cost!("{4}{W}{W}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[
            AbilityDef::triggered_with_targets(
                "When you unlock this door, return target creature card with \
                 mana value 3 or less from your graveyard to the battlefield.",
                TriggerEventDef::DoorUnlocked,
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::ManaValueAtMost(3),
                        ]),
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: Some(PlayerRelation::You),
                    },
                )],
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Battlefield,
                    ZonePlacement::Top,
                ),
            ),
            AbilityDef::triggered_with_targets(
                "Whenever you attack, put a +1/+1 counter on target attacking \
                 creature.",
                TriggerEventDef::attack_declared(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    1,
                    None,
                ),
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Attacking,
                    ]),
                )],
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]);
    CardComposition::room(
        "Surgical Suite // Hospital Room",
        "Surgical Suite",
        FRONT,
        "Hospital Room",
        BACK,
        BOTH,
    )
});

// DSK 35 — Toby, Beastie Befriender
pub(in crate::card::sets) static TOBY_BEASTIE_BEFRIENDER: CardRecord = CardRecord::new(
    "Toby, Beastie Befriender",
    "f33d3948-fe7b-4c3b-ab67-1022623fbb2b",
    "Jehan Choo",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Wizard"], 1, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger(
                "When Toby enters, create a 4/4 white Beast creature token \
                 with \"This token can't attack or block alone.\"",
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                    TokenCharacteristics::creature(&["Beast"], &[ManaColor::White], 4, 4)
                        .with_abilities(&[AbilityDef::static_ability(
                            "This token can't attack or block alone.",
                            EffectDef::StaticApply {
                                recipient: EffectRecipientDef::Source,
                                effect: AppliedEffectDef::Composite(&[
                                    AppliedEffectDef::Rule(AppliedRuleDef::CannotAttackAlone),
                                    AppliedEffectDef::Rule(AppliedRuleDef::CannotBlockAlone),
                                ]),
                            },
                        )]),
                ))),
            ),
            AbilityDef::static_ability(
                "As long as you control four or more creature tokens, creature \
                 tokens you control have flying.",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ObjectCount {
                        query: ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Token,
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                        comparison: ComparisonDef::GreaterOrEqual,
                        amount: 4,
                    },
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                            ObjectQueryDef::matching(
                                ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::Token,
                                ]),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            ),
                        )),
                        effect: AppliedEffectDef::add_ability(&abilities::flying()),
                    },
                },
            ),
        ]),
);

// DSK 36 — Trapped in the Screen
// Audit: unsupported — Needs an exile-until-source-leaves duration with immediate return when that duration ends (CR 610.3); an ordinary leaves trigger returns the card later through the stack.
pub(in crate::card::sets) static TRAPPED_IN_THE_SCREEN: CardRecord = CardRecord::new(
    "Trapped in the Screen",
    "1fe95bfb-8ca7-434f-a2e7-a6b2e699584e",
    "Michael Phillippi",
    CardRules::unsupported(),
);

// DSK 37 — Unidentified Hovership
pub(in crate::card::sets) static UNIDENTIFIED_HOVERSHIP: CardRecord = CardRecord::new(
    "Unidentified Hovership",
    "bd23f168-9ad0-4b4e-bfff-ae004c163727",
    "Jana Heidersdorf",
    CardRules::new_vehicle(mana_cost!("{1}{W}{W}"), 2, 2).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger_with_targets(
            "When this Vehicle enters, exile up to one target creature \
             with toughness 5 or less.",
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::ToughnessGreaterThan(
                            ValueDef::Constant(5),
                        )),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                1,
            )],
            EffectDef::ExileLinkedToSource {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                face_down: false,
                until_source_leaves: false,
                then: None,
            },
        ),
        AbilityDef::triggered(
            "When this Vehicle leaves the battlefield, the exiled card's \
             owner manifests dread.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                None,
            ),
            EffectDef::Sequence(&[
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ObjectSetCount(&ObjectSetCountConditionDef {
                        objects: &ObjectSetDef::LinkedExiles,
                        predicate: ObjectSetPredicateDef::contains(&ObjectPredicateDef::OwnedBy(
                            PlayerRelation::You,
                        )),
                    }),
                    then: &abilities::manifest_dread(),
                },
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ObjectSetCount(&ObjectSetCountConditionDef {
                        objects: &ObjectSetDef::LinkedExiles,
                        predicate: ObjectSetPredicateDef::contains(&ObjectPredicateDef::OwnedBy(
                            PlayerRelation::Opponent,
                        )),
                    }),
                    then: &abilities::bind_top_cards_then(
                        PlayerRefDef::Opponent,
                        ValueDef::Constant(2),
                        &EffectDef::Choose(ChooseDef {
                            binding: ObjectChoiceBindingDef::Objects(crate::Binding!(
                                "dread_permanent"
                            )),
                            unchosen: Some(crate::Binding!("dread_graveyard")),
                            chooser: PlayerRefDef::Opponent,
                            candidates: ObjectSetDef::Binding(crate::ParentBinding),
                            exclude: None,
                            minimum: 1,
                            maximum: 1,
                            visibility: ChoiceVisibilityDef::Private,
                            then: &EffectDef::Sequence(&[
                                EffectDef::PutObjectsOntoBattlefieldFaceDown(
                                    PutObjectsOntoBattlefieldFaceDownDef {
                                        input: ObjectSetDef::Binding(crate::Binding!(
                                            "dread_permanent"
                                        )),
                                        controller: PlayerRefDef::Opponent,
                                        characteristics: crate::card::face_down::manifest(),
                                        turn_up_for_mana_cost: true,
                                        moved: None,
                                        then: &EffectDef::None,
                                    },
                                ),
                                EffectDef::MoveObjects(MoveObjectsDef {
                                    input: ObjectSetDef::Binding(crate::Binding!(
                                        "dread_graveyard"
                                    )),
                                    from: Some(ZoneKind::Library),
                                    zone: ZoneKind::Graveyard,
                                    placement: ZonePlacement::Top,
                                    moved: None,
                                    then: &EffectDef::None,
                                }),
                            ]),
                        }),
                    ),
                },
            ]),
        ),
        abilities::crew("Crew 1", 1),
    ]),
);

// DSK 38 — Unsettling Twins
pub(in crate::card::sets) static UNSETTLING_TWINS: CardRecord = CardRecord::new(
    "Unsettling Twins",
    "85da2e65-4e22-48b6-98ad-6969684d69e1",
    "Lauren K. Cannon",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human"], 2, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, manifest dread. (Look at the top \
             two cards of your library. Put one onto the battlefield face \
             down as a 2/2 creature and the other into your graveyard. \
             Turn it face up any time for its mana cost if it's a creature \
             card.)",
            abilities::manifest_dread(),
        ),
    ]),
);

// DSK 39 — Unwanted Remake
pub(in crate::card::sets) static UNWANTED_REMAKE: CardRecord = CardRecord::new(
    "Unwanted Remake",
    "7b54447f-1daf-4352-b5c3-3c0ec8b8f4d0",
    "Eli Minaya",
    CardRules::new_instant(mana_cost!("{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Destroy target creature. Its controller manifests dread. \
         (That player looks at the top two cards of their library, \
         then puts one onto the battlefield face down as a 2/2 \
         creature and the other into their graveyard. If it's a \
         creature card, it can be turned face up any time for its mana \
         cost.)",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
            abilities::bind_top_cards_then(
                PlayerRefDef::ControllerOf(ObjectRefDef::Target(TargetIndex::PRIMARY)),
                ValueDef::Constant(2),
                &EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(crate::Binding!("dread_permanent")),
                    unchosen: Some(crate::Binding!("dread_graveyard")),
                    chooser: PlayerRefDef::ControllerOf(ObjectRefDef::Target(TargetIndex::PRIMARY)),
                    candidates: ObjectSetDef::Binding(crate::ParentBinding),
                    exclude: None,
                    minimum: 1,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Private,
                    then: &EffectDef::Sequence(&[
                        EffectDef::PutObjectsOntoBattlefieldFaceDown(
                            PutObjectsOntoBattlefieldFaceDownDef {
                                input: ObjectSetDef::Binding(crate::Binding!("dread_permanent")),
                                controller: PlayerRefDef::ControllerOf(ObjectRefDef::Target(
                                    TargetIndex::PRIMARY,
                                )),
                                characteristics: crate::card::face_down::manifest(),
                                turn_up_for_mana_cost: true,
                                moved: None,
                                then: &EffectDef::None,
                            },
                        ),
                        EffectDef::MoveObjects(MoveObjectsDef {
                            input: ObjectSetDef::Binding(crate::Binding!("dread_graveyard")),
                            from: Some(ZoneKind::Library),
                            zone: ZoneKind::Graveyard,
                            placement: ZonePlacement::Top,
                            moved: None,
                            then: &EffectDef::None,
                        }),
                    ]),
                }),
            ),
        ]),
    )]),
);

// DSK 40 — Veteran Survivor
// Audit: unsupported — Needs the ordinal number of the current main phase in the turn (CR 505.1b); PostcombatMain also matches third and later main phases, so it cannot restrict survival to the second main phase.
pub(in crate::card::sets) static VETERAN_SURVIVOR: CardRecord = CardRecord::new(
    "Veteran Survivor",
    "39368ea2-f665-40b1-b042-c0182f7c6df0",
    "Kai Carpenter",
    CardRules::unsupported(),
);

// DSK 41 — The Wandering Rescuer
pub(in crate::card::sets) static THE_WANDERING_RESCUER: CardRecord = CardRecord::new(
    "The Wandering Rescuer",
    "e1ccca86-df8b-4fd9-8fdf-0a5a7b14cdee",
    "Anna Pavleeva",
    CardRules::new_creature(
        mana_cost!("{3}{W}{W}"),
        &["Human", "Samurai", "Noble"],
        3,
        4,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        abilities::flash(),
        abilities::convoke(),
        abilities::double_strike(),
        AbilityDef::static_ability(
            "Other tapped creatures you control have hexproof.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ObjectPredicateDef::Tapped,
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::add_ability(&abilities::hexproof()),
            },
        ),
    ]),
);

// DSK 42 — Abhorrent Oculus
pub(in crate::card::sets) static ABHORRENT_OCULUS: CardRecord = CardRecord::new(
    "Abhorrent Oculus",
    "d2705b43-a94a-44c0-8740-82e0b296820c",
    "Bryan Sola",
// A three-mana 5/5 flier for a deck that filled its own graveyard on
    // purpose, and a body every turn afterwards for nothing.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Eye"], 5, 5).with_abilities(&[
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, exile six cards from your graveyard.",
            &[],
            // Six cards out of your own graveyard, exiled to pay. Nothing is chosen
            // after the fact: the additional cost travels with the cast.
            CostDef::exile(
                ObjectPredicateDef::Any,
                ZoneKind::Graveyard,
                CostQuantityDef::Fixed(6),
            ),
            EffectDef::None,
        ),
        abilities::flying(),
        AbilityDef::triggered(
            "At the beginning of each opponent's upkeep, manifest dread. (Look at the top two cards \
             of your library. Put one onto the battlefield face down as a 2/2 creature and the other \
             into your graveyard. Turn it face up any time for its mana cost if it's a creature \
             card.)",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::Opponent,
            },
            abilities::manifest_dread(),
        ),
    ]),
);

// DSK 43 — Bottomless Pool // Locker Room
pub(in crate::card::sets) static BOTTOMLESS_POOL: CardRecord = CardRecord::new(
    "Bottomless Pool // Locker Room",
    "606fe87c-d17b-4fa7-8e82-e7002d8229ef",
    "Diana Franco",
    CardRules::new_enchantment(mana_cost!("{U}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::triggered_with_targets(
            "When you unlock this door, return up to one target creature \
             to its owner's hand.",
            TriggerEventDef::DoorUnlocked,
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                1,
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
        )]),
)
.with_composition(|| {
    const FRONT: CardRules = CardRules::new_enchantment(mana_cost!("{U}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::triggered_with_targets(
            "When you unlock this door, return up to one target creature \
             to its owner's hand.",
            TriggerEventDef::DoorUnlocked,
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                1,
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
        )]);
    const BACK: CardRules = CardRules::new_enchantment(mana_cost!("{4}{U}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::triggered(
            "Whenever one or more creatures you control deal combat damage \
             to a player, draw a card.",
            TriggerEventDef::CombatDamageDealtToPlayers {
                sources: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                players: PlayerRelation::Any,
            },
            abilities::draw_cards(ValueDef::Constant(1)),
        )]);
    const BOTH: CardRules = CardRules::new_enchantment(mana_cost!("{4}{U}{U}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[
            AbilityDef::triggered_with_targets(
                "When you unlock this door, return up to one target creature \
                 to its owner's hand.",
                TriggerEventDef::DoorUnlocked,
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    1,
                )],
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            ),
            AbilityDef::triggered(
                "Whenever one or more creatures you control deal combat damage \
                 to a player, draw a card.",
                TriggerEventDef::CombatDamageDealtToPlayers {
                    sources: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    players: PlayerRelation::Any,
                },
                abilities::draw_cards(ValueDef::Constant(1)),
            ),
        ]);
    CardComposition::room(
        "Bottomless Pool // Locker Room",
        "Bottomless Pool",
        FRONT,
        "Locker Room",
        BACK,
        BOTH,
    )
});

// DSK 44 — Central Elevator // Promising Stairs
// Audit: unsupported — Needs a value that counts distinct names of unlocked Room doors; ordinary object names do not represent individual unlocked doors.
pub(in crate::card::sets) static CENTRAL_ELEVATOR: CardRecord = CardRecord::new(
    "Central Elevator // Promising Stairs",
    "e548befc-4cd4-46be-951f-045452261cda",
    "Cristi Balanescu",
    CardRules::unsupported(),
);

// DSK 45 — Clammy Prowler
pub(in crate::card::sets) static CLAMMY_PROWLER: CardRecord = CardRecord::new(
    "Clammy Prowler",
    "237f0b93-f12e-4c5f-a3d7-83e8f20f8493",
    "John Tedrick",
    CardRules::new_enchantment_creature(mana_cost!("{3}{U}"), &["Horror"], 2, 5).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks, another target attacking \
             creature can't be blocked this turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ObjectPredicateDef::Attacking,
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// DSK 46 — Creeping Peeper
// Audit: unsupported — Needs mana-use restrictions that include door-unlock and turn-face-up special actions; current restrictions cannot distinguish those special-action payment purposes.
pub(in crate::card::sets) static CREEPING_PEEPER: CardRecord = CardRecord::new(
    "Creeping Peeper",
    "7ad59368-1335-4d8e-a254-ccd889933e57",
    "Maxime Minard",
    CardRules::unsupported(),
);

// DSK 47 — Cursed Windbreaker
pub(in crate::card::sets) static CURSED_WINDBREAKER: CardRecord = CardRecord::new(
    "Cursed Windbreaker",
    "f651e216-f9da-4696-8a1d-6d674e9044c0",
    "Nino Vecia",
    CardRules::new_artifact(mana_cost!("{2}{U}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::enters_trigger(
                "When this Equipment enters, manifest dread, then attach this \
                 Equipment to that creature. (Look at the top two cards of \
                 your library. Put one onto the battlefield face down as a 2/2 \
                 creature and the other into your graveyard. Turn it face up \
                 any time for its mana cost if it's a creature card.)",
                abilities::bind_top_cards_then(
                    PlayerRefDef::EffectController,
                    ValueDef::Constant(2),
                    &EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(crate::Binding!(
                            "dread_permanent"
                        )),
                        unchosen: Some(crate::Binding!("dread_graveyard")),
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Binding(crate::ParentBinding),
                        exclude: None,
                        minimum: 1,
                        maximum: 1,
                        visibility: ChoiceVisibilityDef::Private,
                        then: &EffectDef::PutObjectsOntoBattlefieldFaceDown(
                            PutObjectsOntoBattlefieldFaceDownDef {
                                input: ObjectSetDef::Binding(crate::Binding!("dread_permanent")),
                                controller: PlayerRefDef::EffectController,
                                characteristics: crate::card::face_down::manifest(),
                                turn_up_for_mana_cost: true,
                                moved: Some(crate::Binding!("manifested")),
                                then: &EffectDef::Sequence(&[
                                    EffectDef::MoveObjects(MoveObjectsDef {
                                        input: ObjectSetDef::Binding(crate::Binding!(
                                            "dread_graveyard"
                                        )),
                                        from: Some(ZoneKind::Library),
                                        zone: ZoneKind::Graveyard,
                                        placement: ZonePlacement::Top,
                                        moved: None,
                                        then: &EffectDef::None,
                                    }),
                                    EffectDef::Attach {
                                        object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                            crate::Binding!("manifested"),
                                        )),
                                    },
                                ]),
                            },
                        ),
                    }),
                ),
            ),
            AbilityDef::static_ability(
                "Equipped creature has flying.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::flying()),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{3}"))], "Equip {3}"),
        ]),
);

// DSK 48 — Daggermaw Megalodon
pub(in crate::card::sets) static DAGGERMAW_MEGALODON: CardRecord = CardRecord::new(
    "Daggermaw Megalodon",
    "e72a38b9-29aa-4804-ab27-4c40321f0bc3",
    "Helge C. Balzer",
    CardRules::new_creature(mana_cost!("{4}{U}{U}"), &["Shark"], 5, 7).with_abilities(&[
        abilities::vigilance(),
        abilities::typecycling!(
            "Islandcycling {2}",
            &[CostDef::Mana(mana_cost!("{2}"))],
            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Island"))
        ),
    ]),
);

// DSK 49 — Don't Make a Sound
pub(in crate::card::sets) static DON_T_MAKE_A_SOUND: CardRecord = CardRecord::new(
    "Don't Make a Sound",
    "b5f42e59-7315-41cb-9c41-346e44f0c5fb",
    "Zezhou Chen",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Counter target spell unless its controller pays {2}. If they \
         do, surveil 2. (Look at the top two cards of your library, \
         then put any number of them into your graveyard and the rest \
         on top of your library in any order.)",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Spell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::PayOr(
            PayOrDef::optional_or(
                &[CostDef::Mana(mana_cost!("{2}"))],
                &abilities::surveil(ValueDef::Constant(2)),
                &EffectDef::counter_target(TargetIndex::PRIMARY),
            )
            .with_payer(PlayerSetDef::One(PlayerRefDef::ControllerOf(
                ObjectRefDef::Target(TargetIndex::PRIMARY),
            ))),
        ),
    )]),
);

// DSK 50 — Duskmourn's Domination
pub(in crate::card::sets) static DUSKMOURN_S_DOMINATION: CardRecord = CardRecord::new(
    "Duskmourn's Domination",
    "1d96dbc9-c2fd-42c1-9d55-5c14fa1c1c6f",
    "Eli Minaya",
    CardRules::new_enchantment(mana_cost!("{4}{U}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "You control enchanted creature.",
                EffectDef::gain_control(
                    EffectRecipientDef::AttachedPermanent,
                    PlayerRefDef::EffectController,
                    ControlDurationDef::WhileSourceRemains {
                        while_tapped: false,
                    },
                ),
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets -3/-0 and loses all abilities.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(-3),
                            ValueDef::Constant(0),
                        ),
                        AppliedEffectDef::remove_abilities(AbilityPredicateDef::Any),
                    ]),
                },
            ),
        ]),
);

// DSK 51 — Enduring Curiosity
// Audit: unsupported — Needs a return-to-battlefield instruction that establishes the permanent as a noncreature enchantment before entry replacement effects and enters triggers inspect it; applying a type change after the return produces an incorrect creature entry.
pub(in crate::card::sets) static ENDURING_CURIOSITY: CardRecord = CardRecord::new(
    "Enduring Curiosity",
    "8616629e-08f9-41ad-bfec-f86c8096f1cb",
    "Julie Dillon",
    CardRules::unsupported(),
);

// DSK 52 — Enter the Enigma
pub(in crate::card::sets) static ENTER_THE_ENIGMA: CardRecord = CardRecord::new(
    "Enter the Enigma",
    "cf5479c7-9e46-4a57-abe7-8cc670de89e4",
    "Chris Rallis",
    CardRules::new_sorcery(mana_cost!("{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature can't be blocked this turn.\nDraw a card.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            abilities::draw_cards(ValueDef::Constant(1)),
        ]),
    )]),
);

// DSK 53 — Entity Tracker
// Audit: unsupported — Needs a committed event for a Room becoming fully unlocked, observed by other permanents and graveyard abilities. DoorUnlocked only dispatches to the door being opened, so it cannot implement the second eerie trigger.
pub(in crate::card::sets) static ENTITY_TRACKER: CardRecord = CardRecord::new(
    "Entity Tracker",
    "ae54d697-6d06-4af1-a617-8a47a6ab9c01",
    "Cristi Balanescu",
    CardRules::unsupported(),
);

// DSK 54 — Erratic Apparition
// Audit: unsupported — Needs a committed event for a Room becoming fully unlocked, observed by other permanents and graveyard abilities. DoorUnlocked only dispatches to the door being opened, so it cannot implement the second eerie trigger.
pub(in crate::card::sets) static ERRATIC_APPARITION: CardRecord = CardRecord::new(
    "Erratic Apparition",
    "a74fd612-2890-4333-a379-5bf7650fbb87",
    "Miranda Meeks",
    CardRules::unsupported(),
);

// DSK 55 — Fear of Failed Tests
pub(in crate::card::sets) static FEAR_OF_FAILED_TESTS: CardRecord = CardRecord::new(
    "Fear of Failed Tests",
    "a87c39df-7636-454b-8485-a1cc9362fd84",
    "Jana Heidersdorf",
    CardRules::new_enchantment_creature(mana_cost!("{4}{U}"), &["Nightmare"], 2, 7).with_abilities(
        &[AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, draw \
             that many cards.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            abilities::draw_cards(ValueDef::TriggerEventAmount),
        )],
    ),
);

// DSK 56 — Fear of Falling
pub(in crate::card::sets) static FEAR_OF_FALLING: CardRecord = CardRecord::new(
    "Fear of Falling",
    "0e814e48-cd9d-428f-90e2-74d97cb9c8f1",
    "Maxime Minard",
    CardRules::new_enchantment_creature(mana_cost!("{3}{U}{U}"), &["Nightmare"], 4, 4)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::triggered_with_targets(
                "Whenever this creature attacks, target creature defending \
                 player controls gets -2/-0 and loses flying until your next \
                 turn.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::DefendingPlayer),
                        owner: None,
                    },
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(-2),
                            ValueDef::Constant(0),
                        ),
                        AppliedEffectDef::remove_abilities(AbilityPredicateDef::Keyword(
                            KeywordAbility::Flying,
                        )),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilYourNextTurn,
                },
            ),
        ]),
);

// DSK 57 — Fear of Impostors
pub(in crate::card::sets) static FEAR_OF_IMPOSTORS: CardRecord = CardRecord::new(
    "Fear of Impostors",
    "bdee441e-14ab-42d1-b447-5a6488fd713a",
    "David Szabo",
    CardRules::new_enchantment_creature(mana_cost!("{1}{U}{U}"), &["Nightmare"], 3, 2)
        .with_abilities(&[
            abilities::flash(),
            abilities::enters_trigger_with_targets(
                "When this creature enters, counter target spell. Its \
                 controller manifests dread. (That player looks at the top two \
                 cards of their library, then puts one onto the battlefield \
                 face down as a 2/2 creature and the other into their \
                 graveyard. If it's a creature card, it can be turned face up \
                 any time for its mana cost.)",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Spell,
                        zones: &[ZoneKind::Stack],
                        controller: None,
                        owner: None,
                    },
                )],
                EffectDef::Sequence(&[
                    EffectDef::counter_target(TargetIndex::PRIMARY),
                    abilities::bind_top_cards_then(
                        PlayerRefDef::ControllerOf(ObjectRefDef::Target(TargetIndex::PRIMARY)),
                        ValueDef::Constant(2),
                        &EffectDef::Choose(ChooseDef {
                            binding: ObjectChoiceBindingDef::Objects(crate::Binding!(
                                "dread_permanent"
                            )),
                            unchosen: Some(crate::Binding!("dread_graveyard")),
                            chooser: PlayerRefDef::ControllerOf(ObjectRefDef::Target(
                                TargetIndex::PRIMARY,
                            )),
                            candidates: ObjectSetDef::Binding(crate::ParentBinding),
                            exclude: None,
                            minimum: 1,
                            maximum: 1,
                            visibility: ChoiceVisibilityDef::Private,
                            then: &EffectDef::Sequence(&[
                                EffectDef::PutObjectsOntoBattlefieldFaceDown(
                                    PutObjectsOntoBattlefieldFaceDownDef {
                                        input: ObjectSetDef::Binding(crate::Binding!(
                                            "dread_permanent"
                                        )),
                                        controller: PlayerRefDef::ControllerOf(
                                            ObjectRefDef::Target(TargetIndex::PRIMARY),
                                        ),
                                        characteristics: crate::card::face_down::manifest(),
                                        turn_up_for_mana_cost: true,
                                        moved: None,
                                        then: &EffectDef::None,
                                    },
                                ),
                                EffectDef::MoveObjects(MoveObjectsDef {
                                    input: ObjectSetDef::Binding(crate::Binding!(
                                        "dread_graveyard"
                                    )),
                                    from: Some(ZoneKind::Library),
                                    zone: ZoneKind::Graveyard,
                                    placement: ZonePlacement::Top,
                                    moved: None,
                                    then: &EffectDef::None,
                                }),
                            ]),
                        }),
                    ),
                ]),
            ),
        ]),
);

// DSK 58 — Fear of Isolation
pub(in crate::card::sets) static FEAR_OF_ISOLATION: CardRecord = CardRecord::new(
    "Fear of Isolation",
    "69aa6054-8c59-4bbc-a283-adb453639786",
    "Irina Nordsol",
    CardRules::new_enchantment_creature(mana_cost!("{1}{U}"), &["Nightmare"], 2, 3).with_abilities(
        &[
            AbilityDef::spell(
                "As an additional cost to cast this spell, return a permanent \
                 you control to its owner's hand.",
                EffectDef::None,
            )
            .with_spell_additional_cost(&CostDef::ReturnToHand {
                object: ObjectPredicateDef::Any,
                quantity: CostQuantityDef::Fixed(1),
            }),
            abilities::flying(),
        ],
    ),
);

// DSK 59 — Floodpits Drowner
pub(in crate::card::sets) static FLOODPITS_DROWNER: CardRecord = CardRecord::new(
    "Floodpits Drowner",
    "a6a62aa3-8edb-4000-8ebd-15ec4b00eed7",
    "John Tedrick",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Merfolk"], 2, 1).with_abilities(&[
        abilities::flash(),
        abilities::vigilance(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, tap target creature an opponent \
             controls and put a stun counter on it.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::Tap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::Stun,
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
        AbilityDef::activated_with_targets(
            "{1}{U}, {T}: Shuffle this creature and target creature with a \
             stun counter on it into their owners' libraries.",
            &[CostDef::Mana(mana_cost!("{1}{U}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasCounter(CounterKind::Stun),
                ]),
            )],
            EffectDef::BindObjects(BindObjectsDef {
                source: ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::Union(&[
                    ObjectSetDef::One(ObjectRefDef::Source),
                    ObjectSetDef::LegalTargets(TargetIndex::PRIMARY),
                ])),
                binding: crate::Binding!("shuffle_pair"),
                then: &EffectDef::Sequence(&[
                    EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                            "shuffle_pair"
                        ))),
                        ZoneKind::Library,
                        ZonePlacement::Top,
                    ),
                    EffectDef::IfCondition {
                        condition: &TriggerConditionDef::ObjectSetCount(
                            &ObjectSetCountConditionDef {
                                objects: &ObjectSetDef::Binding(crate::Binding!("shuffle_pair")),
                                predicate: ObjectSetPredicateDef::contains(
                                    &ObjectPredicateDef::OwnedBy(PlayerRelation::You),
                                ),
                            },
                        ),
                        then: &EffectDef::ShuffleLibrary {
                            player: EffectRecipientDef::Controller,
                        },
                    },
                    EffectDef::IfCondition {
                        condition: &TriggerConditionDef::ObjectSetCount(
                            &ObjectSetCountConditionDef {
                                objects: &ObjectSetDef::Binding(crate::Binding!("shuffle_pair")),
                                predicate: ObjectSetPredicateDef::contains(
                                    &ObjectPredicateDef::OwnedBy(PlayerRelation::Opponent),
                                ),
                            },
                        ),
                        then: &EffectDef::ShuffleLibrary {
                            player: EffectRecipientDef::Opponent,
                        },
                    },
                ]),
            }),
        ),
    ]),
);

// DSK 60 — Get Out
pub(in crate::card::sets) static GET_OUT: CardRecord = CardRecord::new(
    "Get Out",
    "5d9471c8-2db9-4ce3-a1e5-42b85134cb8e",
    "Mirko Failoni",
    CardRules::new_instant(mana_cost!("{U}{U}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Counter target creature or enchantment spell.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                        ]),
                        zones: &[ZoneKind::Stack],
                        controller: None,
                        owner: None,
                    },
                )],
                EffectDef::counter_target(TargetIndex::PRIMARY),
            ),
            AbilityDef::spell_with_targets(
                "Return one or two target creatures and/or enchantments you \
                 own to your hand.",
                &[AbilityTargetDef {
                    minimum: 1,
                    maximum: 2,
                    ..AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: Some(PlayerRelation::You),
                    })
                }],
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            ),
        ],
    )]),
);

// DSK 61 — Ghostly Keybearer
// Audit: unsupported — Needs a resolving effect that unlocks a chosen locked door of a targeted Room; the existing unlock path is a player special action, not a shared effect.
pub(in crate::card::sets) static GHOSTLY_KEYBEARER: CardRecord = CardRecord::new(
    "Ghostly Keybearer",
    "11d04a98-6997-4653-9719-e6b215567599",
    "Marco Gorlei",
    CardRules::unsupported(),
);

// DSK 62 — Glimmerburst
pub(in crate::card::sets) static GLIMMERBURST: CardRecord = CardRecord::new(
    "Glimmerburst",
    "fe1ab8db-3994-4b6d-9eaf-75243a78b715",
    "Dan Watson",
    CardRules::new_instant(mana_cost!("{3}{U}")).with_abilities(&[AbilityDef::spell(
        "Draw two cards. Create a 1/1 white Glimmer enchantment \
         creature token.",
        EffectDef::Sequence(&[
            abilities::draw_cards(ValueDef::Constant(2)),
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::new(
                    CardTypeSet::single(CardType::Enchantment).with(CardType::Creature),
                    &["Glimmer"],
                    &[ManaColor::White],
                    Some(CreatureStats {
                        power: 1,
                        toughness: 1,
                    }),
                ),
            ))),
        ]),
    )]),
);

// DSK 63 — Leyline of Transformation (alternate printing)
const LEYLINE_OF_TRANSFORMATION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LEYLINE_OF_TRANSFORMATION,
    1,
    "4bd941ca-f3d2-44c1-8df3-851362f6b848",
    "Sergey Glushakov",
);

// DSK 64 — Marina Vendrell's Grimoire
// Audit: unsupported — Needs a life-loss event carrying the actual amount lost, including payments and life-total changes; damage and life-gain events cannot represent every such loss.
pub(in crate::card::sets) static MARINA_VENDRELL_S_GRIMOIRE: CardRecord = CardRecord::new(
    "Marina Vendrell's Grimoire",
    "1ab1aef7-4171-4869-8eb5-fe42e3ca9e45",
    "Denys Tsiperko",
    CardRules::unsupported(),
);

// DSK 65 — Meat Locker // Drowned Diner
pub(in crate::card::sets) static MEAT_LOCKER: CardRecord = CardRecord::new(
    "Meat Locker // Drowned Diner",
    "b3c773f0-9e65-48de-a362-a9a943198693",
    "Sergey Glushakov",
    CardRules::new_enchantment(mana_cost!("{2}{U}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::triggered_with_targets(
            "When you unlock this door, tap up to one target creature and \
             put two stun counters on it. (If a permanent with a stun \
             counter would become untapped, remove one from it instead.)",
            TriggerEventDef::DoorUnlocked,
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                1,
            )],
            EffectDef::Sequence(&[
                EffectDef::Tap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::Stun,
                    amount: ValueDef::Constant(2),
                },
            ]),
        )]),
)
.with_composition(|| {
    const FRONT: CardRules = CardRules::new_enchantment(mana_cost!("{2}{U}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::triggered_with_targets(
            "When you unlock this door, tap up to one target creature and \
             put two stun counters on it. (If a permanent with a stun \
             counter would become untapped, remove one from it instead.)",
            TriggerEventDef::DoorUnlocked,
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                1,
            )],
            EffectDef::Sequence(&[
                EffectDef::Tap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::Stun,
                    amount: ValueDef::Constant(2),
                },
            ]),
        )]);
    const BACK: CardRules = CardRules::new_enchantment(mana_cost!("{3}{U}{U}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::triggered(
            "When you unlock this door, draw three cards, then discard a card.",
            TriggerEventDef::DoorUnlocked,
            EffectDef::Sequence(&[
                abilities::draw_cards(ValueDef::Constant(3)),
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
            ]),
        )]);
    const BOTH: CardRules = CardRules::new_enchantment(mana_cost!("{5}{U}{U}{U}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[
            AbilityDef::triggered_with_targets(
                "When you unlock this door, tap up to one target creature and \
                 put two stun counters on it. (If a permanent with a stun \
                 counter would become untapped, remove one from it instead.)",
                TriggerEventDef::DoorUnlocked,
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    1,
                )],
                EffectDef::Sequence(&[
                    EffectDef::Tap {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        kind: CounterKind::Stun,
                        amount: ValueDef::Constant(2),
                    },
                ]),
            ),
            AbilityDef::triggered(
                "When you unlock this door, draw three cards, then discard a card.",
                TriggerEventDef::DoorUnlocked,
                EffectDef::Sequence(&[
                    abilities::draw_cards(ValueDef::Constant(3)),
                    EffectDef::Discard {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                        selection: DiscardSelectionDef::RecipientChooses,
                        then: None,
                    },
                ]),
            ),
        ]);
    CardComposition::room(
        "Meat Locker // Drowned Diner",
        "Meat Locker",
        FRONT,
        "Drowned Diner",
        BACK,
        BOTH,
    )
});

// DSK 66 — The Mindskinner
// Audit: unsupported — Needs a damage-prevention replacement that subsequently mills each opponent by the prevented amount; supported prevention modifiers do not carry a general post-prevention effect continuation.
pub(in crate::card::sets) static THE_MINDSKINNER: CardRecord = CardRecord::new(
    "The Mindskinner",
    "7f1bb4c5-99be-46cc-ad54-7affb5f0144c",
    "Abz J Harding",
    CardRules::unsupported(),
);

// DSK 67 — Mirror Room // Fractured Realm
// Audit: unsupported — Needs a trigger multiplier for all triggered abilities of controlled permanents; TriggersAnAdditionalTime currently multiplies only triggers caused by matching battlefield entries.
pub(in crate::card::sets) static MIRROR_ROOM: CardRecord = CardRecord::new(
    "Mirror Room // Fractured Realm",
    "c2e085dd-a448-4f5a-9cfa-5c2034234e7c",
    "Helge C. Balzer",
    CardRules::unsupported(),
);

// DSK 68 — Overlord of the Floodpits
pub(in crate::card::sets) static OVERLORD_OF_THE_FLOODPITS: CardRecord = CardRecord::new(
    "Overlord of the Floodpits",
    "5fca5de8-9cda-4370-9f72-4462f8cfd696",
    "Abz J Harding",
    CardRules::new_enchantment_creature(mana_cost!("{3}{U}{U}"), &["Avatar", "Horror"], 5, 3)
        .with_abilities(&[
            impending_cast(
                &[CostDef::Mana(mana_cost!("{1}{U}{U}"))],
                "Impending 4—{1}{U}{U} (If you cast this spell for its \
                 impending cost, it enters with four time counters and isn't a \
                 creature until the last is removed. At the beginning of your \
                 end step, remove a time counter from it.)",
            ),
            impending_entry(4),
            impending_countdown(),
            abilities::flying(),
            AbilityDef::triggered(
                "Whenever this permanent enters or attacks, draw two cards, \
                 then discard a card.",
                TriggerEventDef::AnyOf(&[
                    TriggerEventDef::zone_changed(
                        ObjectPredicateDef::Source,
                        None,
                        Some(ZoneKind::Battlefield),
                    ),
                    TriggerEventDef::attacks(ObjectPredicateDef::Source),
                ]),
                EffectDef::Sequence(&[
                    abilities::draw_cards(ValueDef::Constant(2)),
                    EffectDef::Discard {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                        selection: DiscardSelectionDef::RecipientChooses,
                        then: None,
                    },
                ]),
            ),
        ]),
);

// DSK 69 — Paranormal Analyst
// Audit: unsupported — Needs a completed manifest-dread action event carrying the card actually put into the graveyard; the helper resolves its steps without publishing that action/result event.
pub(in crate::card::sets) static PARANORMAL_ANALYST: CardRecord = CardRecord::new(
    "Paranormal Analyst",
    "60cf954a-5503-460c-8720-8960842eea47",
    "James Ryman",
    CardRules::unsupported(),
);

// DSK 70 — Piranha Fly
pub(in crate::card::sets) static PIRANHA_FLY: CardRecord = CardRecord::new(
    "Piranha Fly",
    "cd891675-12d4-40df-996a-97d694155227",
    "Kekai Kotaki",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Fish", "Insect"], 2, 1).with_abilities(&[
        abilities::flying(),
        abilities::enters_tapped(CardType::Creature),
    ]),
);

// DSK 71 — Scrabbling Skullcrab
// Audit: unsupported — Needs a committed event for a Room becoming fully unlocked, observed by other permanents and graveyard abilities. DoorUnlocked only dispatches to the door being opened, so it cannot implement the second eerie trigger.
pub(in crate::card::sets) static SCRABBLING_SKULLCRAB: CardRecord = CardRecord::new(
    "Scrabbling Skullcrab",
    "c1017b8e-e7fa-41de-8eb2-2e4a59db5117",
    "John Tedrick",
    CardRules::unsupported(),
);

// DSK 72 — Silent Hallcreeper
// Audit: unsupported — Needs per-incarnation history of previously chosen trigger modes; current modal triggers do not exclude modes already chosen on earlier resolutions.
pub(in crate::card::sets) static SILENT_HALLCREEPER: CardRecord = CardRecord::new(
    "Silent Hallcreeper",
    "aac4f0cc-63be-4f08-956e-39839c9735ba",
    "Joshua Raphael",
    CardRules::unsupported(),
);

// DSK 73 — Stalked Researcher
// Audit: unsupported — Needs a committed event for a Room becoming fully unlocked, observed by other permanents and graveyard abilities. DoorUnlocked only dispatches to the door being opened, so it cannot implement the second eerie trigger.
pub(in crate::card::sets) static STALKED_RESEARCHER: CardRecord = CardRecord::new(
    "Stalked Researcher",
    "8a26ccad-adfa-43cc-be4c-dc8dd584bca3",
    "Marta Nael",
    CardRules::unsupported(),
);

// DSK 74 — Stay Hidden, Stay Silent
pub(in crate::card::sets) static STAY_HIDDEN_STAY_SILENT: CardRecord = CardRecord::new(
    "Stay Hidden, Stay Silent",
    "b661ffe7-4f58-43fe-a1f1-dd7a6d4d28a7",
    "Josu Hernaiz",
    CardRules::new_enchantment(mana_cost!("{1}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            abilities::enters_trigger(
                "When this Aura enters, tap enchanted creature.",
                EffectDef::Tap {
                    object: EffectRecipientDef::AttachedPermanent,
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature doesn't untap during its controller's \
                 untap step.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
                },
            ),
            AbilityDef::activated(
                "{4}{U}{U}: Shuffle enchanted creature into its owner's \
                 library, then manifest dread. Activate only as a sorcery.",
                &[CostDef::Mana(mana_cost!("{4}{U}{U}"))],
                EffectDef::Sequence(&[
                    EffectDef::Sequence(&[
                        EffectDef::move_to_zone(
                            EffectRecipientDef::AttachedPermanent,
                            ZoneKind::Library,
                            ZonePlacement::Top,
                        ),
                        EffectDef::ShuffleLibrary {
                            player: EffectRecipientDef::player(PlayerRefDef::OwnerOf(
                                ObjectRefDef::AttachedToSource,
                            )),
                        },
                    ]),
                    abilities::manifest_dread(),
                ]),
            )
            .with_activation_timing(ActivationTimingDef::SorcerySpeed),
        ]),
);

// DSK 75 — The Tale of Tamiyo
// Audit: unsupported — Needs an unbounded conditional mill-and-repeat procedure, plus a batch of castable copies of selected non-stack cards with their normal costs.
pub(in crate::card::sets) static THE_TALE_OF_TAMIYO: CardRecord = CardRecord::new(
    "The Tale of Tamiyo",
    "aaeba193-05d3-4c2d-a304-bbe7114c2eef",
    "Anna Pavleeva",
    CardRules::unsupported(),
);

// DSK 76 — Tunnel Surveyor
pub(in crate::card::sets) static TUNNEL_SURVEYOR: CardRecord = CardRecord::new(
    "Tunnel Surveyor",
    "12ee2690-f464-4a38-81e1-6dd2053a3327",
    "John Stanko",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Detective"], 2, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, create a 1/1 white Glimmer \
             enchantment creature token.",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::new(
                    CardTypeSet::single(CardType::Enchantment).with(CardType::Creature),
                    &["Glimmer"],
                    &[ManaColor::White],
                    Some(CreatureStats {
                        power: 1,
                        toughness: 1,
                    }),
                ),
            ))),
        ),
    ]),
);

// DSK 77 — Twist Reality
pub(in crate::card::sets) static TWIST_REALITY: CardRecord = CardRecord::new(
    "Twist Reality",
    "644714dd-7a0b-4d4b-9a61-8f6e505b1d22",
    "Allen Williams",
    CardRules::new_instant(mana_cost!("{1}{U}{U}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Counter target spell.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Spell,
                        zones: &[ZoneKind::Stack],
                        controller: None,
                        owner: None,
                    },
                )],
                EffectDef::counter_target(TargetIndex::PRIMARY),
            ),
            AbilityDef::spell(
                "Manifest dread. (Look at the top two cards of your library. \
                 Put one onto the battlefield face down as a 2/2 creature and \
                 the other into your graveyard. Turn it face up any time for \
                 its mana cost if it's a creature card.)",
                abilities::manifest_dread(),
            ),
        ],
    )]),
);

// DSK 78 — Unable to Scream
// Audit: unsupported — Needs a prohibition on turning a face-down permanent face up and static addition of Artifact to an attached creature; neither is accepted by the shared static boundary.
pub(in crate::card::sets) static UNABLE_TO_SCREAM: CardRecord = CardRecord::new(
    "Unable to Scream",
    "7c59e0cd-10a8-4a32-9c0a-a2c6ef1ed9a6",
    "Fariba Khamseh",
    CardRules::unsupported(),
);

// DSK 79 — Underwater Tunnel // Slimy Aquarium
pub(in crate::card::sets) static UNDERWATER_TUNNEL: CardRecord = CardRecord::new(
    "Underwater Tunnel // Slimy Aquarium",
    "2dd69f2d-8c9c-41fc-93ea-48fc7a6d5272",
    "Titus Lunter",
    CardRules::new_enchantment(mana_cost!("{U}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::triggered(
            "When you unlock this door, surveil 2. (Look at the top two \
             cards of your library, then put any number of them into your \
             graveyard and the rest on top of your library in any order.)",
            TriggerEventDef::DoorUnlocked,
            abilities::surveil(ValueDef::Constant(2)),
        )]),
)
.with_composition(|| {
    const FRONT: CardRules = CardRules::new_enchantment(mana_cost!("{U}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::triggered(
            "When you unlock this door, surveil 2. (Look at the top two \
             cards of your library, then put any number of them into your \
             graveyard and the rest on top of your library in any order.)",
            TriggerEventDef::DoorUnlocked,
            abilities::surveil(ValueDef::Constant(2)),
        )]);
    const BACK: CardRules = CardRules::new_enchantment(mana_cost!("{3}{U}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::triggered(
            "When you unlock this door, manifest dread, then put a +1/+1 \
             counter on that creature.",
            TriggerEventDef::DoorUnlocked,
            abilities::bind_top_cards_then(
                PlayerRefDef::EffectController,
                ValueDef::Constant(2),
                &EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(crate::Binding!("dread_permanent")),
                    unchosen: Some(crate::Binding!("dread_graveyard")),
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Binding(crate::ParentBinding),
                    exclude: None,
                    minimum: 1,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Private,
                    then: &EffectDef::PutObjectsOntoBattlefieldFaceDown(
                        PutObjectsOntoBattlefieldFaceDownDef {
                            input: ObjectSetDef::Binding(crate::Binding!("dread_permanent")),
                            controller: PlayerRefDef::EffectController,
                            characteristics: crate::card::face_down::manifest(),
                            turn_up_for_mana_cost: true,
                            moved: Some(crate::Binding!("manifested")),
                            then: &EffectDef::Sequence(&[
                                EffectDef::MoveObjects(MoveObjectsDef {
                                    input: ObjectSetDef::Binding(crate::Binding!(
                                        "dread_graveyard"
                                    )),
                                    from: Some(ZoneKind::Library),
                                    zone: ZoneKind::Graveyard,
                                    placement: ZonePlacement::Top,
                                    moved: None,
                                    then: &EffectDef::None,
                                }),
                                EffectDef::AddCounters {
                                    object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                        crate::Binding!("manifested"),
                                    )),
                                    kind: CounterKind::PlusOnePlusOne,
                                    amount: ValueDef::Constant(1),
                                },
                            ]),
                        },
                    ),
                }),
            ),
        )]);
    const BOTH: CardRules = CardRules::new_enchantment(mana_cost!("{3}{U}{U}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[
            AbilityDef::triggered(
                "When you unlock this door, surveil 2. (Look at the top two \
                 cards of your library, then put any number of them into your \
                 graveyard and the rest on top of your library in any order.)",
                TriggerEventDef::DoorUnlocked,
                abilities::surveil(ValueDef::Constant(2)),
            ),
            AbilityDef::triggered(
                "When you unlock this door, manifest dread, then put a +1/+1 \
                 counter on that creature.",
                TriggerEventDef::DoorUnlocked,
                abilities::bind_top_cards_then(
                    PlayerRefDef::EffectController,
                    ValueDef::Constant(2),
                    &EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(crate::Binding!(
                            "dread_permanent"
                        )),
                        unchosen: Some(crate::Binding!("dread_graveyard")),
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Binding(crate::ParentBinding),
                        exclude: None,
                        minimum: 1,
                        maximum: 1,
                        visibility: ChoiceVisibilityDef::Private,
                        then: &EffectDef::PutObjectsOntoBattlefieldFaceDown(
                            PutObjectsOntoBattlefieldFaceDownDef {
                                input: ObjectSetDef::Binding(crate::Binding!("dread_permanent")),
                                controller: PlayerRefDef::EffectController,
                                characteristics: crate::card::face_down::manifest(),
                                turn_up_for_mana_cost: true,
                                moved: Some(crate::Binding!("manifested")),
                                then: &EffectDef::Sequence(&[
                                    EffectDef::MoveObjects(MoveObjectsDef {
                                        input: ObjectSetDef::Binding(crate::Binding!(
                                            "dread_graveyard"
                                        )),
                                        from: Some(ZoneKind::Library),
                                        zone: ZoneKind::Graveyard,
                                        placement: ZonePlacement::Top,
                                        moved: None,
                                        then: &EffectDef::None,
                                    }),
                                    EffectDef::AddCounters {
                                        object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                            crate::Binding!("manifested"),
                                        )),
                                        kind: CounterKind::PlusOnePlusOne,
                                        amount: ValueDef::Constant(1),
                                    },
                                ]),
                            },
                        ),
                    }),
                ),
            ),
        ]);
    CardComposition::room(
        "Underwater Tunnel // Slimy Aquarium",
        "Underwater Tunnel",
        FRONT,
        "Slimy Aquarium",
        BACK,
        BOTH,
    )
});

// DSK 80 — Unnerving Grasp
pub(in crate::card::sets) static UNNERVING_GRASP: CardRecord = CardRecord::new(
    "Unnerving Grasp",
    "5602756d-76d2-4502-965f-36fc44596123",
    "Jeremy Wilson",
    CardRules::new_sorcery(mana_cost!("{2}{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Return up to one target nonland permanent to its owner's \
         hand. Manifest dread. (Look at the top two cards of your \
         library. Put one onto the battlefield face down as a 2/2 \
         creature and the other into your graveyard. Turn it face up \
         any time for its mana cost if it's a creature card.)",
        &[AbilityTargetDef::up_to(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
            1,
        )],
        EffectDef::Sequence(&[
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
            abilities::manifest_dread(),
        ]),
    )]),
);

// DSK 81 — Unwilling Vessel
// Audit: unsupported — Needs a committed event for a Room becoming fully unlocked, observed by other permanents and graveyard abilities. DoorUnlocked only dispatches to the door being opened, so it cannot implement the second eerie trigger.
pub(in crate::card::sets) static UNWILLING_VESSEL: CardRecord = CardRecord::new(
    "Unwilling Vessel",
    "5d1758ed-fe33-4ead-8c83-e54fabcb4cfe",
    "Josu Hernaiz",
    CardRules::unsupported(),
);

// DSK 82 — Vanish from Sight
pub(in crate::card::sets) static VANISH_FROM_SIGHT: CardRecord = CardRecord::new(
    "Vanish from Sight",
    "5254988b-3113-42f7-b751-517ffb3b40f0",
    "Billy Christian",
    CardRules::new_instant(mana_cost!("{3}{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target nonland permanent's owner puts it on their choice of \
         the top or bottom of their library. Surveil 1. (Look at the \
         top card of your library. You may put it into your \
         graveyard.)",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
        )],
        EffectDef::Sequence(&[
            EffectDef::ChooseEffect {
                player: EffectRecipientDef::player(PlayerRefDef::OwnerOf(ObjectRefDef::Target(
                    TargetIndex::PRIMARY,
                ))),
                choices: &[
                    EffectChoiceDef {
                        label: "Top",
                        effect: EffectDef::move_to_zone(
                            EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            ZoneKind::Library,
                            ZonePlacement::Top,
                        ),
                    },
                    EffectChoiceDef {
                        label: "Bottom",
                        effect: EffectDef::move_to_zone(
                            EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            ZoneKind::Library,
                            ZonePlacement::Bottom,
                        ),
                    },
                ],
            },
            abilities::surveil(ValueDef::Constant(1)),
        ]),
    )]),
);

// DSK 83 — Appendage Amalgam
pub(in crate::card::sets) static APPENDAGE_AMALGAM: CardRecord = CardRecord::new(
    "Appendage Amalgam",
    "d16b3cf3-7b85-4153-b25a-e6b5fce725cf",
    "Bartek Fedyczak",
    CardRules::new_enchantment_creature(mana_cost!("{2}{B}"), &["Horror"], 3, 2).with_abilities(&[
        abilities::flash(),
        AbilityDef::triggered(
            "Whenever this creature attacks, surveil 1. (Look at the top \
             card of your library. You may put it into your graveyard.)",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            abilities::surveil(ValueDef::Constant(1)),
        ),
    ]),
);

// DSK 84 — Balemurk Leech
// Audit: unsupported — Needs a committed event for a Room becoming fully unlocked, observed by other permanents and graveyard abilities. DoorUnlocked only dispatches to the door being opened, so it cannot implement the second eerie trigger.
pub(in crate::card::sets) static BALEMURK_LEECH: CardRecord = CardRecord::new(
    "Balemurk Leech",
    "f0621b32-95c5-4f70-96cf-d46d20efc85d",
    "John Tedrick",
    CardRules::unsupported(),
);

// DSK 85 — Cackling Slasher
pub(in crate::card::sets) static CACKLING_SLASHER: CardRecord = CardRecord::new(
    "Cackling Slasher",
    "b1269ccf-febf-42b0-8c20-21ccb731a3ec",
    "Johann Bodin",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Human", "Assassin"], 3, 3).with_abilities(&[
        abilities::deathtouch(),
        AbilityDef::as_enters_if(
            "This creature enters with a +1/+1 counter on it if a creature \
             died this turn.",
            ReplacementConditionDef::CreatureDiedThisTurn,
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 1,
                },
            ),
        ),
    ]),
);

// DSK 86 — Come Back Wrong
pub(in crate::card::sets) static COME_BACK_WRONG: CardRecord = CardRecord::new(
    "Come Back Wrong",
    "72ee3b45-aa4e-4c5b-a9e6-608bfbd93f8b",
    "David Auden Nash",
    CardRules::new_sorcery(mana_cost!("{2}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Destroy target creature. If a creature card is put into a \
         graveyard this way, return it to the battlefield under your \
         control. Sacrifice it at the beginning of your next end step.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            then: Some(DestroyFollowUpDef {
                binding: crate::Binding!("destroyed"),
                effect: &EffectDef::WithZoneMoveResult {
                    effect: &EffectDef::WithBattlefieldArrival {
                        effect: &EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Matching {
                                objects: &ObjectSetDef::ZoneChangeSuccessorsOfBinding(
                                    crate::Binding!("destroyed"),
                                ),
                                object: ObjectSetFilterDef::Predicate(
                                    &ObjectPredicateDef::HasType(CardType::Creature),
                                ),
                            }),
                            ZoneKind::Battlefield,
                            ZonePlacement::Top,
                        ),
                        arrival: BattlefieldArrivalDef {
                            controller: Some(PlayerRelation::You),
                            ..BattlefieldArrivalDef::DEFAULT
                        },
                    },
                    binding: crate::Binding!("returned_creature"),
                    then: &EffectDef::InstallTrigger(InstalledTriggerDef::once(
                        &AbilityDef::triggered(
                            "At the beginning of the next end step, sacrifice this permanent.",
                            TriggerEventDef::StepBegins {
                                step: TurnStepDef::End,
                                player: PlayerRelation::You,
                            },
                            EffectDef::sacrifice(EffectRecipientDef::objects(
                                ObjectSetDef::ZoneChangeSuccessorsOfBinding(crate::Binding!(
                                    "returned_creature"
                                )),
                            )),
                        ),
                    )),
                },
            }),
        },
    )]),
);

// DSK 87 — Commune with Evil
pub(in crate::card::sets) static COMMUNE_WITH_EVIL: CardRecord = CardRecord::new(
    "Commune with Evil",
    "9ab94ebe-b51b-4c6f-af43-0ed80e2808a3",
    "Ovidio Cartagena",
    CardRules::new_sorcery(mana_cost!("{2}{B}")).with_abilities(&[AbilityDef::spell(
        "Look at the top four cards of your library. Put one of them \
         into your hand and the rest into your graveyard. You gain 3 \
         life.",
        EffectDef::Sequence(&[
            EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                source: ObjectCollectionSourceDef::TopCards {
                    player: PlayerRefDef::EffectController,
                    count: ValueDef::Constant(4),
                },
                actor: PlayerRefDef::EffectController,
                inspection: CollectionInspectionDef::Look,
                object: ObjectPredicateDef::Any,
                minimum: 1,
                maximum: 1,
                chosen: crate::Binding!("chosen"),
                remainder: crate::Binding!("rest"),
                then: &EffectDef::Sequence(&[
                    EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                            "chosen"
                        ))),
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                    EffectDef::MoveObjects(MoveObjectsDef {
                        input: ObjectSetDef::Binding(crate::Binding!("rest")),
                        from: Some(ZoneKind::Library),
                        zone: ZoneKind::Graveyard,
                        placement: ZonePlacement::Top,
                        moved: None,
                        then: &EffectDef::None,
                    }),
                ]),
            }),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ]),
    )]),
);

// DSK 88 — Cracked Skull
pub(in crate::card::sets) static CRACKED_SKULL: CardRecord = CardRecord::new(
    "Cracked Skull",
    "7616ad5e-ed30-4876-8743-6f0f9f143ea1",
    "Mirko Failoni",
    CardRules::new_enchantment(mana_cost!("{2}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            abilities::enters_trigger_with_targets(
                "When this Aura enters, look at target player's hand. You may \
                 choose a nonland card from it. That player discards that \
                 card.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::Sequence(&[
                    EffectDef::LookAtHand {
                        player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                    EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(crate::Binding!("discard_chosen")),
                        unchosen: None,
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Query(ObjectQueryDef::owned_by(
                            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                            &[ZoneKind::Hand],
                            PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
                        )),
                        exclude: None,
                        minimum: 0,
                        maximum: 1,
                        visibility: ChoiceVisibilityDef::Private,
                        then: &EffectDef::discard_cards(EffectRecipientDef::objects(
                            ObjectSetDef::Binding(crate::Binding!("discard_chosen")),
                        )),
                    }),
                ]),
            ),
            AbilityDef::triggered(
                "When enchanted creature is dealt damage, destroy it.",
                TriggerEventDef::DamageDealt(DamageEventMatcherDef {
                    kind: DamageKindDef::Any,
                    source: DamageSourceMatcherDef::Any,
                    recipient: DamageRecipientMatcherDef::Recipients(
                        EffectRecipientDef::AttachedPermanent,
                    ),
                }),
                EffectDef::Destroy {
                    object: EffectRecipientDef::AttachedPermanent,
                    then: None,
                },
            ),
        ]),
);

// DSK 89 — Cynical Loner
// Audit: unsupported — Needs the ordinal number of the current main phase in the turn (CR 505.1b); PostcombatMain also matches third and later main phases, so it cannot restrict survival to the second main phase.
pub(in crate::card::sets) static CYNICAL_LONER: CardRecord = CardRecord::new(
    "Cynical Loner",
    "cc93bcb8-778d-491e-877b-e6ad432764cb",
    "Miranda Meeks",
    CardRules::unsupported(),
);

// DSK 90 — Dashing Bloodsucker
// Audit: unsupported — Needs a committed event for a Room becoming fully unlocked, observed by other permanents and graveyard abilities. DoorUnlocked only dispatches to the door being opened, so it cannot implement the second eerie trigger.
pub(in crate::card::sets) static DASHING_BLOODSUCKER: CardRecord = CardRecord::new(
    "Dashing Bloodsucker",
    "790a90cc-d36f-43b5-8423-89e30bdf7b9f",
    "Randy Gallegos",
    CardRules::unsupported(),
);

// DSK 91 — Defiled Crypt // Cadaver Lab
// Audit: unsupported — Needs graveyard-departure events with the departing card's owner and a once-per-turn batch trigger; current committed nonbattlefield moves do not supply that complete event.
pub(in crate::card::sets) static DEFILED_CRYPT: CardRecord = CardRecord::new(
    "Defiled Crypt // Cadaver Lab",
    "d94fb4da-0d3f-4d84-966b-914b84b23289",
    "Martin de Diego Sádaba",
    CardRules::unsupported(),
);

// DSK 92 — Demonic Counsel
pub(in crate::card::sets) static DEMONIC_COUNSEL: CardRecord = CardRecord::new(
    "Demonic Counsel",
    "ff79c845-4115-4fbf-b20f-37470f2bf7fb",
    "Babs Webb",
    CardRules::new_sorcery(mana_cost!("{1}{B}")).with_abilities(&[AbilityDef::spell(
        "Search your library for a Demon card, reveal it, put it into \
         your hand, then shuffle.\nDelirium — If there are four or \
         more card types among cards in your graveyard, instead search \
         your library for any card, put it into your hand, then \
         shuffle.",
        EffectDef::IfElseCondition {
            condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                left: ValueDef::CardTypesAmongGraveyards(PlayerRelation::You),
                comparison: ComparisonDef::GreaterOrEqual,
                right: ValueDef::Constant(4),
            }),
            then: &EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::Any,
                minimum: 1,
                maximum: ValueDef::Constant(1),
                reveal: false,
                destination: ZoneKind::Hand,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: false,
                attachment: None,
                binding: None,
                then: None,
            },
            otherwise: &EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Demon")),
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: true,
                destination: ZoneKind::Hand,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: false,
                attachment: None,
                binding: None,
                then: None,
            },
        },
    )]),
);

// DSK 93 — Derelict Attic // Widow's Walk
pub(in crate::card::sets) static DERELICT_ATTIC: CardRecord = CardRecord::new(
    "Derelict Attic // Widow's Walk",
    "2cae24c1-53f1-4f3f-8795-b634c46a17c4",
    "Marc Simonetti",
    CardRules::new_enchantment(mana_cost!("{2}{B}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::triggered(
            "When you unlock this door, you draw two cards and you lose 2 \
             life.",
            TriggerEventDef::DoorUnlocked,
            EffectDef::Sequence(&[
                abilities::draw_cards(ValueDef::Constant(2)),
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
            ]),
        )]),
)
.with_composition(|| {
    const FRONT: CardRules = CardRules::new_enchantment(mana_cost!("{2}{B}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::triggered(
            "When you unlock this door, you draw two cards and you lose 2 \
             life.",
            TriggerEventDef::DoorUnlocked,
            EffectDef::Sequence(&[
                abilities::draw_cards(ValueDef::Constant(2)),
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
            ]),
        )]);
    const BACK: CardRules = CardRules::new_enchantment(mana_cost!("{3}{B}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::triggered(
            "Whenever a creature you control attacks alone, it gets +1/+0 \
             and gains deathtouch until end of turn.",
            TriggerEventDef::attacks_in_declaration(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                1,
                Some(1),
            ),
            EffectDef::Apply {
                recipient: EffectRecipientDef::TriggeringObject,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
                    AppliedEffectDef::add_ability(&abilities::deathtouch()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )]);
    const BOTH: CardRules = CardRules::new_enchantment(mana_cost!("{5}{B}{B}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[
            AbilityDef::triggered(
                "When you unlock this door, you draw two cards and you lose 2 \
                 life.",
                TriggerEventDef::DoorUnlocked,
                EffectDef::Sequence(&[
                    abilities::draw_cards(ValueDef::Constant(2)),
                    EffectDef::LoseLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(2),
                    },
                ]),
            ),
            AbilityDef::triggered(
                "Whenever a creature you control attacks alone, it gets +1/+0 \
                 and gains deathtouch until end of turn.",
                TriggerEventDef::attacks_in_declaration(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    1,
                    Some(1),
                ),
                EffectDef::Apply {
                    recipient: EffectRecipientDef::TriggeringObject,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(0),
                        ),
                        AppliedEffectDef::add_ability(&abilities::deathtouch()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]);
    CardComposition::room(
        "Derelict Attic // Widow's Walk",
        "Derelict Attic",
        FRONT,
        "Widow's Walk",
        BACK,
        BOTH,
    )
});

// DSK 94 — Doomsday Excruciator
// Audit: unsupported — Needs a general move-to-exile instruction that keeps the chosen library cards face down without linking them to the source; the existing face-down exile paths also create source links or cast permissions.
pub(in crate::card::sets) static DOOMSDAY_EXCRUCIATOR: CardRecord = CardRecord::new(
    "Doomsday Excruciator",
    "542c89b6-48c6-4fcb-8a4c-b5ed2fa1d384",
    "Denys Tsiperko",
    CardRules::unsupported(),
);

// DSK 95 — Enduring Tenacity
// Audit: unsupported — Needs a return-to-battlefield instruction that establishes the permanent as a noncreature enchantment before entry replacement effects and enters triggers inspect it; applying a type change after the return produces an incorrect creature entry.
pub(in crate::card::sets) static ENDURING_TENACITY: CardRecord = CardRecord::new(
    "Enduring Tenacity",
    "d5756d4b-3068-412c-8643-880d3459151e",
    "Isis",
    CardRules::unsupported(),
);

// DSK 96 — Fanatic of the Harrowing
pub(in crate::card::sets) static FANATIC_OF_THE_HARROWING: CardRecord = CardRecord::new(
    "Fanatic of the Harrowing",
    "2a0acb05-91e0-4f7c-b48b-99e1068fad16",
    "Fajareka Setiawan",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Human", "Cleric"], 2, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, each player discards a card. If \
             you discarded a card this way, draw a card.",
            EffectDef::Discard {
                recipient: EffectRecipientDef::EachPlayer,
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::RecipientChooses,
                then: Some(DiscardFollowUpDef {
                    counted: ObjectPredicateDef::Any,
                    bound: Some(crate::Binding!("discarded")),
                    effect: &EffectDef::IfCondition {
                        condition: &TriggerConditionDef::ObjectSetCount(
                            &ObjectSetCountConditionDef {
                                objects: &ObjectSetDef::Binding(crate::Binding!("discarded")),
                                predicate: ObjectSetPredicateDef::contains(
                                    &ObjectPredicateDef::OwnedBy(PlayerRelation::You),
                                ),
                            },
                        ),
                        then: &abilities::draw_cards(ValueDef::Constant(1)),
                    },
                }),
            },
        ),
    ]),
);

// DSK 97 — Fear of Lost Teeth
pub(in crate::card::sets) static FEAR_OF_LOST_TEETH: CardRecord = CardRecord::new(
    "Fear of Lost Teeth",
    "259045fe-f349-4be1-bf29-465d084ed35e",
    "Oriana Menendez",
    CardRules::new_enchantment_creature(mana_cost!("{B}"), &["Nightmare"], 1, 1).with_abilities(&[
        abilities::dies_trigger_with_targets(
            "When this creature dies, it deals 1 damage to any target and \
             you gain 1 life.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::Sequence(&[
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(1),
                ),
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
    ]),
);

// DSK 98 — Fear of the Dark
pub(in crate::card::sets) static FEAR_OF_THE_DARK: CardRecord = CardRecord::new(
    "Fear of the Dark",
    "8700eb8d-1cc0-45ff-b769-c875ee6500ea",
    "Sam Wolfe Connelly",
    CardRules::new_enchantment_creature(mana_cost!("{4}{B}"), &["Nightmare"], 5, 5).with_abilities(
        &[AbilityDef::triggered_if(
            "Whenever this creature attacks, if defending player controls \
             no Glimmer creatures, it gains menace and deathtouch until \
             end of turn. (A creature with menace can't be blocked except \
             by two or more creatures.)",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &TriggerConditionDef::Not(&TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Glimmer")),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Opponent,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            }),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_ability(&abilities::menace()),
                    AppliedEffectDef::add_ability(&abilities::deathtouch()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )],
    ),
);

// DSK 99 — Final Vengeance
pub(in crate::card::sets) static FINAL_VENGEANCE: CardRecord = CardRecord::new(
    "Final Vengeance",
    "648da934-71f6-4945-a08f-09698414417b",
    "David Szabo",
    CardRules::new_sorcery(mana_cost!("{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Exile target creature.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::move_to_zone(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ZoneKind::Exile,
            ZonePlacement::Top,
        ),
    )
    .with_spell_additional_cost(&CostDef::Sacrifice {
        object: ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::HasType(CardType::Enchantment),
        ]),
        quantity: CostQuantityDef::Fixed(1),
    })]),
);

// DSK 100 — Funeral Room // Awakening Hall
pub(in crate::card::sets) static FUNERAL_ROOM: CardRecord = CardRecord::new(
    "Funeral Room // Awakening Hall",
    "48237c98-5067-47a8-af74-7b9bce57c6a4",
    "Miklós Ligeti",
    CardRules::new_enchantment(mana_cost!("{2}{B}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::triggered(
            "Whenever a creature you control dies, each opponent loses 1 \
             life and you gain 1 life.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::Sequence(&[
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Opponent,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ]),
        )]),
)
.with_composition(|| {
    const FRONT: CardRules = CardRules::new_enchantment(mana_cost!("{2}{B}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::triggered(
            "Whenever a creature you control dies, each opponent loses 1 \
             life and you gain 1 life.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::Sequence(&[
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Opponent,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ]),
        )]);
    const BACK: CardRules = CardRules::new_enchantment(mana_cost!("{6}{B}{B}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::triggered(
            "When you unlock this door, return all creature cards from \
             your graveyard to the battlefield.",
            TriggerEventDef::DoorUnlocked,
            EffectDef::move_to_zone(
                EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Graveyard],
                    PlayerRelation::You,
                ))),
                ZoneKind::Battlefield,
                ZonePlacement::Top,
            ),
        )]);
    const BOTH: CardRules = CardRules::new_enchantment(mana_cost!("{8}{B}{B}{B}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever a creature you control dies, each opponent loses 1 \
                 life and you gain 1 life.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                EffectDef::Sequence(&[
                    EffectDef::LoseLife {
                        recipient: EffectRecipientDef::Opponent,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                ]),
            ),
            AbilityDef::triggered(
                "When you unlock this door, return all creature cards from \
                 your graveyard to the battlefield.",
                TriggerEventDef::DoorUnlocked,
                EffectDef::move_to_zone(
                    EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    ))),
                    ZoneKind::Battlefield,
                    ZonePlacement::Top,
                ),
            ),
        ]);
    CardComposition::room(
        "Funeral Room // Awakening Hall",
        "Funeral Room",
        FRONT,
        "Awakening Hall",
        BACK,
        BOTH,
    )
});

// DSK 101 — Give In to Violence
pub(in crate::card::sets) static GIVE_IN_TO_VIOLENCE: CardRecord = CardRecord::new(
    "Give In to Violence",
    "00ac1759-d4cc-41d5-a9b7-a89b80d2190c",
    "Septian Fajrianto",
    CardRules::new_instant(mana_cost!("{1}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets +2/+2 and gains lifelink until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                AppliedEffectDef::add_ability(&abilities::lifelink()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// DSK 102 — Grievous Wound
pub(in crate::card::sets) static GRIEVOUS_WOUND: CardRecord = CardRecord::new(
    "Grievous Wound",
    "6ed3dd5e-cd27-4b18-ad60-f5c4d9a811b9",
    "Martina Fačková",
    CardRules::new_enchantment(mana_cost!("{3}{B}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_player(),
            AbilityDef::static_ability(
                "Enchanted player can't gain life.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::EnchantedPlayer,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotGainLife),
                },
            ),
            AbilityDef::triggered(
                "Whenever enchanted player is dealt damage, they lose half \
                 their life, rounded up.",
                TriggerEventDef::DamageDealt(DamageEventMatcherDef {
                    kind: DamageKindDef::Any,
                    source: DamageSourceMatcherDef::Any,
                    recipient: DamageRecipientMatcherDef::Recipients(
                        EffectRecipientDef::EnchantedPlayer,
                    ),
                }),
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::EnchantedPlayer,
                    amount: ValueDef::Halved(&HalvedValueDef::new(
                        ValueDef::LifeTotal(PlayerRelation::EnchantedPlayer),
                        RoundingDef::Up,
                    )),
                },
            ),
        ]),
);

// DSK 103 — Innocuous Rat
pub(in crate::card::sets) static INNOCUOUS_RAT: CardRecord = CardRecord::new(
    "Innocuous Rat",
    "94edbbc5-5673-4753-bfad-4432c4b7dca4",
    "Maxime Minard",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Rat"], 1, 1).with_abilities(&[
        abilities::dies_trigger(
            "When this creature dies, manifest dread. (Look at the top two \
             cards of your library. Put one onto the battlefield face down \
             as a 2/2 creature and the other into your graveyard. Turn it \
             face up any time for its mana cost if it's a creature card.)",
            abilities::manifest_dread(),
        ),
    ]),
);

// DSK 104 — Killer's Mask
pub(in crate::card::sets) static KILLER_S_MASK: CardRecord = CardRecord::new(
    "Killer's Mask",
    "f1fc02ae-77b8-4e5e-94b4-22ecf7ae40ae",
    "Wero Gallo",
    CardRules::new_artifact(mana_cost!("{2}{B}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::enters_trigger(
                "When this Equipment enters, manifest dread, then attach this \
                 Equipment to that creature. (Look at the top two cards of \
                 your library. Put one onto the battlefield face down as a 2/2 \
                 creature and the other into your graveyard. Turn it face up \
                 any time for its mana cost if it's a creature card.)",
                abilities::bind_top_cards_then(
                    PlayerRefDef::EffectController,
                    ValueDef::Constant(2),
                    &EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(crate::Binding!(
                            "dread_permanent"
                        )),
                        unchosen: Some(crate::Binding!("dread_graveyard")),
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Binding(crate::ParentBinding),
                        exclude: None,
                        minimum: 1,
                        maximum: 1,
                        visibility: ChoiceVisibilityDef::Private,
                        then: &EffectDef::PutObjectsOntoBattlefieldFaceDown(
                            PutObjectsOntoBattlefieldFaceDownDef {
                                input: ObjectSetDef::Binding(crate::Binding!("dread_permanent")),
                                controller: PlayerRefDef::EffectController,
                                characteristics: crate::card::face_down::manifest(),
                                turn_up_for_mana_cost: true,
                                moved: Some(crate::Binding!("manifested")),
                                then: &EffectDef::Sequence(&[
                                    EffectDef::MoveObjects(MoveObjectsDef {
                                        input: ObjectSetDef::Binding(crate::Binding!(
                                            "dread_graveyard"
                                        )),
                                        from: Some(ZoneKind::Library),
                                        zone: ZoneKind::Graveyard,
                                        placement: ZonePlacement::Top,
                                        moved: None,
                                        then: &EffectDef::None,
                                    }),
                                    EffectDef::Attach {
                                        object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                            crate::Binding!("manifested"),
                                        )),
                                    },
                                ]),
                            },
                        ),
                    }),
                ),
            ),
            AbilityDef::static_ability(
                "Equipped creature has menace.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::menace()),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{2}"))], "Equip {2}"),
        ]),
);

// DSK 105 — Let's Play a Game
// Audit: unsupported — Needs a conditional modal maximum that can count distinct card types in the graveyard; existing modal conditions can count matching objects but cannot express delirium.
pub(in crate::card::sets) static LET_S_PLAY_A_GAME: CardRecord = CardRecord::new(
    "Let's Play a Game",
    "645911b4-7728-4380-9097-e4139b986423",
    "Riccardo Federici",
    CardRules::unsupported(),
);

// DSK 106 — Leyline of the Void (reprint)
const LEYLINE_OF_THE_VOID_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_gpt::LEYLINE_OF_THE_VOID,
    "aeaa3aff-608d-4723-bb7c-8daedebe9f36",
    "Sergey Glushakov",
);

// DSK 107 — Live or Die
pub(in crate::card::sets) static LIVE_OR_DIE: CardRecord = CardRecord::new(
    "Live or Die",
    "cd81e438-e38f-42a1-b677-202dbcb3837c",
    "Lorenzo Mastroianni",
    CardRules::new_instant(mana_cost!("{3}{B}{B}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Return target creature card from your graveyard to the \
                 battlefield.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: Some(PlayerRelation::You),
                    },
                )],
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Battlefield,
                    ZonePlacement::Top,
                ),
            ),
            AbilityDef::spell_with_targets(
                "Destroy target creature.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            ),
        ],
    )]),
);

// DSK 108 — Meathook Massacre II
// Audit: unsupported — Needs retained cast X when this enchantment's enters trigger resolves after it has left the battlefield; SourceCastX currently reads the live permanent and loses that value when the source departs.
pub(in crate::card::sets) static MEATHOOK_MASSACRE_II: CardRecord = CardRecord::new(
    "Meathook Massacre II",
    "3db59d06-a226-42b2-8f01-6b63a6eea83f",
    "Tiffany Turrill",
    CardRules::unsupported(),
);

// DSK 109 — Miasma Demon
// Audit: unsupported — Needs a reflexive trigger after discarding a chosen number of cards, with a target-count maximum derived from the completed discard.
pub(in crate::card::sets) static MIASMA_DEMON: CardRecord = CardRecord::new(
    "Miasma Demon",
    "6d167c00-75ff-4301-855a-8319b89e3689",
    "Mathias Kollros",
    CardRules::unsupported(),
);

// DSK 110 — Murder (reprint)
const MURDER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m13::MURDER,
    "2c249609-9cf7-46f1-b94c-9329add966bb",
    "Domenico Cava",
);

// DSK 111 — Nowhere to Run
// Audit: unsupported — Needs a controller-filtered targeting permission that ignores opponents' creature hexproof and suppresses those creatures' ward triggers without removing unrelated abilities.
pub(in crate::card::sets) static NOWHERE_TO_RUN: CardRecord = CardRecord::new(
    "Nowhere to Run",
    "fee60e9d-9ee7-444a-88f3-c1929e1888fb",
    "Jodie Muir",
    CardRules::unsupported(),
);

// DSK 112 — Osseous Sticktwister
// Audit: unsupported — Needs a resolving payment choice between public permanent sacrifice and private discard, retaining which opponents completed either choice for the damage follow-up.
pub(in crate::card::sets) static OSSEOUS_STICKTWISTER: CardRecord = CardRecord::new(
    "Osseous Sticktwister",
    "4f43473e-1302-4543-b331-1a86cfbc3ced",
    "Mathias Kollros",
    CardRules::unsupported(),
);

// DSK 113 — Overlord of the Balemurk
pub(in crate::card::sets) static OVERLORD_OF_THE_BALEMURK: CardRecord = CardRecord::new(
    "Overlord of the Balemurk",
    "9b911653-7b96-4cf3-a907-13c5c53a14f7",
    "Babs Webb",
    CardRules::new_enchantment_creature(mana_cost!("{3}{B}{B}"), &["Avatar", "Horror"], 5, 5)
        .with_abilities(&[
            AbilityDef::alternative_cast(
                &[CostDef::Mana(mana_cost!("{1}{B}"))],
                AlternativeCastKindDef::Impending,
                Some(
                    "Impending 5—{1}{B} (If you cast this spell for its impending \
                     cost, it enters with five time counters and isn't a creature \
                     until the last is removed. At the beginning of your end step, \
                     remove a time counter from it.)",
                ),
                EffectDef::None,
            ),
            AbilityDef::as_enters_if(
                "If you cast this spell for its impending cost, it enters with \
                 five time counters.",
                ReplacementConditionDef::SourceCastWith(AlternativeCastKindDef::Impending),
                ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::AddCounters {
                        kind: CounterKind::named("time"),
                        amount: 5,
                    },
                ),
            ),
            impending_countdown(),
            AbilityDef::triggered(
                "Whenever this permanent enters or attacks, mill four cards, \
                 then you may return a non-Avatar creature card or a \
                 planeswalker card from your graveyard to your hand.",
                TriggerEventDef::AnyOf(&[
                    TriggerEventDef::zone_changed(
                        ObjectPredicateDef::Source,
                        None,
                        Some(ZoneKind::Battlefield),
                    ),
                    TriggerEventDef::attacks(ObjectPredicateDef::Source),
                ]),
                EffectDef::Sequence(&[
                    EffectDef::Mill {
                        player: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(4),
                    },
                    // The whole graveyard, not only what the mill just put there: the clause
                    // says "from your graveyard" and means it.
                    EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                        unchosen: None,
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Query(ObjectQueryDef::owned_by(
                            // "A non-Avatar creature card or a planeswalker card." The Overlord itself
                            // is an Avatar, which is what the exclusion is there for: it cannot buy
                            // itself back.
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype(
                                        SubtypeDef::Literal("Avatar"),
                                    )),
                                ]),
                                ObjectPredicateDef::HasType(CardType::Planeswalker),
                            ]),
                            &[ZoneKind::Graveyard],
                            PlayerSetDef::Related(PlayerRelation::You),
                        )),
                        exclude: None,
                        minimum: 0,
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

// DSK 114 — Popular Egotist
pub(in crate::card::sets) static POPULAR_EGOTIST: CardRecord = CardRecord::new(
    "Popular Egotist",
    "35e64605-8edb-4def-9522-765e90d1f0f3",
    "Julia Metzger",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Human", "Rogue"], 3, 2).with_abilities(&[
        AbilityDef::activated(
            "{1}{B}, Sacrifice another creature or enchantment: This \
             creature gains indestructible until end of turn. Tap it. \
             (Damage and effects that say \"destroy\" don't destroy it.)",
            &[
                CostDef::Mana(mana_cost!("{1}{B}")),
                CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                    ]),
                ])),
            ],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Tap {
                    object: EffectRecipientDef::Source,
                },
            ]),
        ),
        AbilityDef::triggered_with_targets(
            "Whenever you sacrifice a permanent, target opponent loses 1 \
             life and you gain 1 life.",
            TriggerEventDef::Sacrificed {
                object: ObjectPredicateDef::Any,
                player: PlayerRelation::You,
            },
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
            )],
            EffectDef::Sequence(&[
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
    ]),
);

// DSK 115 — Resurrected Cultist
pub(in crate::card::sets) static RESURRECTED_CULTIST: CardRecord = CardRecord::new(
    "Resurrected Cultist",
    "e41bd259-e81f-432a-bebf-4c6534f23db7",
    "Tyler Walpole",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Human", "Cleric"], 4, 1).with_abilities(&[
        AbilityDef::activated(
            "Delirium — {2}{B}{B}: Return this card from your graveyard to \
             the battlefield with a finality counter on it. Activate only \
             if there are four or more card types among cards in your \
             graveyard and only as a sorcery. (If a creature with a \
             finality counter on it would die, exile it instead.)",
            &[CostDef::Mana(mana_cost!("{2}{B}{B}"))],
            EffectDef::WithBattlefieldArrival {
                effect: &EffectDef::move_to_zone(
                    EffectRecipientDef::Source,
                    ZoneKind::Battlefield,
                    ZonePlacement::Top,
                ),
                arrival: BattlefieldArrivalDef {
                    modifications: &[BattlefieldEntryModificationDef::AddCountersValue {
                        kind: CounterKind::Finality,
                        amount: ValueDef::Constant(1),
                    }],
                    ..BattlefieldArrivalDef::DEFAULT
                },
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed)
        .with_source_zones(&[ZoneKind::Graveyard])
        .with_activation_condition(&TriggerConditionDef::ValueComparison(&ValueComparisonDef {
            left: ValueDef::CardTypesAmongGraveyards(PlayerRelation::You),
            comparison: ComparisonDef::GreaterOrEqual,
            right: ValueDef::Constant(4),
        })),
    ]),
);

// DSK 116 — Spectral Snatcher
pub(in crate::card::sets) static SPECTRAL_SNATCHER: CardRecord = CardRecord::new(
    "Spectral Snatcher",
    "68f90f57-94a0-4ee1-9383-093e8fca52ea",
    "Domenico Cava",
    CardRules::new_creature(mana_cost!("{4}{B}{B}"), &["Spirit"], 6, 5).with_abilities(&[
        abilities::ward(&[CostDef::DiscardCards(1)], "Ward—Discard a card."),
        abilities::typecycling!(
            "Swampcycling {2}",
            &[CostDef::Mana(mana_cost!("{2}"))],
            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Swamp"))
        ),
    ]),
);

// DSK 117 — Sporogenic Infection
// Audit: unsupported — Needs a sacrifice-selection predicate excluding the Aura's attached permanent; current cost selection cannot compare candidate identity with AttachedPermanent.
pub(in crate::card::sets) static SPOROGENIC_INFECTION: CardRecord = CardRecord::new(
    "Sporogenic Infection",
    "eaae086e-0781-4f4d-bc9c-a98228bc380c",
    "Warren Mahy",
    CardRules::unsupported(),
);

// DSK 118 — Unholy Annex // Ritual Chamber
pub(in crate::card::sets) static UNHOLY_ANNEX: CardRecord = CardRecord::new(
    "Unholy Annex // Ritual Chamber",
    "0fb4c734-c698-46f2-bc78-5f036f472e5b",
    "Matteo Bassini",
    CardRules::new_enchantment(mana_cost!("{2}{B}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::triggered(
            "At the beginning of your end step, draw a card. If you \
             control a Demon, each opponent loses 2 life and you gain 2 \
             life. Otherwise, you lose 2 life.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::You,
            },
            EffectDef::Sequence(&[
                abilities::draw_cards(ValueDef::Constant(1)),
                EffectDef::IfElseCondition {
                    condition: &TriggerConditionDef::ObjectCount {
                        query: ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Demon")),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                        comparison: ComparisonDef::GreaterOrEqual,
                        amount: 1,
                    },
                    then: &EffectDef::Sequence(&[
                        EffectDef::LoseLife {
                            recipient: EffectRecipientDef::Opponent,
                            amount: ValueDef::Constant(2),
                        },
                        EffectDef::GainLife {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(2),
                        },
                    ]),
                    otherwise: &EffectDef::LoseLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(2),
                    },
                },
            ]),
        )]),
)
.with_composition(|| {
    const FRONT: CardRules = CardRules::new_enchantment(mana_cost!("{2}{B}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::triggered(
            "At the beginning of your end step, draw a card. If you \
             control a Demon, each opponent loses 2 life and you gain 2 \
             life. Otherwise, you lose 2 life.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::You,
            },
            EffectDef::Sequence(&[
                abilities::draw_cards(ValueDef::Constant(1)),
                EffectDef::IfElseCondition {
                    condition: &TriggerConditionDef::ObjectCount {
                        query: ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Demon")),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                        comparison: ComparisonDef::GreaterOrEqual,
                        amount: 1,
                    },
                    then: &EffectDef::Sequence(&[
                        EffectDef::LoseLife {
                            recipient: EffectRecipientDef::Opponent,
                            amount: ValueDef::Constant(2),
                        },
                        EffectDef::GainLife {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(2),
                        },
                    ]),
                    otherwise: &EffectDef::LoseLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(2),
                    },
                },
            ]),
        )]);
    const BACK: CardRules = CardRules::new_enchantment(mana_cost!("{3}{B}{B}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::triggered(
            "When you unlock this door, create a 6/6 black Demon creature \
             token with flying.",
            TriggerEventDef::DoorUnlocked,
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(DEMON_TOKEN))),
        )]);
    const BOTH: CardRules = CardRules::new_enchantment(mana_cost!("{5}{B}{B}{B}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[
            AbilityDef::triggered(
                "At the beginning of your end step, draw a card. If you \
                 control a Demon, each opponent loses 2 life and you gain 2 \
                 life. Otherwise, you lose 2 life.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::End,
                    player: PlayerRelation::You,
                },
                EffectDef::Sequence(&[
                    abilities::draw_cards(ValueDef::Constant(1)),
                    EffectDef::IfElseCondition {
                        condition: &TriggerConditionDef::ObjectCount {
                            query: ObjectQueryDef::matching(
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Demon")),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            ),
                            comparison: ComparisonDef::GreaterOrEqual,
                            amount: 1,
                        },
                        then: &EffectDef::Sequence(&[
                            EffectDef::LoseLife {
                                recipient: EffectRecipientDef::Opponent,
                                amount: ValueDef::Constant(2),
                            },
                            EffectDef::GainLife {
                                recipient: EffectRecipientDef::Controller,
                                amount: ValueDef::Constant(2),
                            },
                        ]),
                        otherwise: &EffectDef::LoseLife {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(2),
                        },
                    },
                ]),
            ),
            AbilityDef::triggered(
                "When you unlock this door, create a 6/6 black Demon creature \
                 token with flying.",
                TriggerEventDef::DoorUnlocked,
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(DEMON_TOKEN))),
            ),
        ]);
    CardComposition::room(
        "Unholy Annex // Ritual Chamber",
        "Unholy Annex",
        FRONT,
        "Ritual Chamber",
        BACK,
        BOTH,
    )
});

// DSK 119 — Unstoppable Slasher
// Audit: unsupported — Needs a last-known total-counter count across all counter kinds on the dying creature; the current source counter predicates require one fixed kind and cannot express had no counters of any kind.
pub(in crate::card::sets) static UNSTOPPABLE_SLASHER: CardRecord = CardRecord::new(
    "Unstoppable Slasher",
    "c78da035-6b5b-4136-9ab6-f622b64fdc54",
    "Maxime Minard",
    CardRules::unsupported(),
);

// DSK 120 — Valgavoth, Terror Eater
// Audit: unsupported — Needs linked exile-play permission whose alternative cost is life equal to each selected spell's mana value, with the permission restricted to the controller's turn.
pub(in crate::card::sets) static VALGAVOTH_TERROR_EATER: CardRecord = CardRecord::new(
    "Valgavoth, Terror Eater",
    "7740ff55-67bb-409e-90f7-2c2c8b8c770a",
    "Antonio José Manzanedo",
    CardRules::unsupported(),
);

// DSK 121 — Valgavoth's Faithful
pub(in crate::card::sets) static VALGAVOTH_S_FAITHFUL: CardRecord = CardRecord::new(
    "Valgavoth's Faithful",
    "995194a0-ceb7-48c2-be5d-a208b3971e77",
    "Jodie Muir",
    CardRules::new_creature(mana_cost!("{B}"), &["Human", "Cleric"], 1, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{3}{B}, Sacrifice this creature: Return target creature card \
             from your graveyard to the battlefield. Activate only as a \
             sorcery.",
            &[
                CostDef::Mana(mana_cost!("{3}{B}")),
                CostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Battlefield,
                ZonePlacement::Top,
            ),
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// DSK 122 — Vile Mutilator
pub(in crate::card::sets) static VILE_MUTILATOR: CardRecord = CardRecord::new(
    "Vile Mutilator",
    "383fe040-98bb-40f3-b809-48c429b83f47",
    "Néstor Ossandón Leal",
    CardRules::new_creature(mana_cost!("{5}{B}{B}"), &["Demon"], 6, 5).with_abilities(&[
        AbilityDef::spell(
            "As an additional cost to cast this spell, sacrifice a \
             creature or enchantment.",
            EffectDef::None,
        )
        .with_spell_additional_cost(&CostDef::Sacrifice {
            object: ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::HasType(CardType::Enchantment),
            ]),
            quantity: CostQuantityDef::Fixed(1),
        }),
        abilities::flying(),
        abilities::trample(),
        abilities::enters_trigger(
            "When this creature enters, each opponent sacrifices a \
             nontoken enchantment of their choice, then sacrifices a \
             nontoken creature of their choice.",
            EffectDef::Sequence(&[
                EffectDef::ChooseForEachPlayer(ChooseForEachPlayerDef {
                    player: EffectRecipientDef::Opponent,
                    zone: ZoneKind::Battlefield,
                    candidates: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                    ]),
                    selection: PerPlayerSelectionDef::Count(ValueDef::Constant(1)),
                    chosen: crate::Binding!("sacrifice"),
                    unchosen: crate::Binding!("rest"),
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::sacrifice(EffectRecipientDef::objects(
                        ObjectSetDef::Binding(crate::Binding!("sacrifice")),
                    )),
                }),
                EffectDef::ChooseForEachPlayer(ChooseForEachPlayerDef {
                    player: EffectRecipientDef::Opponent,
                    zone: ZoneKind::Battlefield,
                    candidates: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                    ]),
                    selection: PerPlayerSelectionDef::Count(ValueDef::Constant(1)),
                    chosen: crate::Binding!("sacrifice_creature"),
                    unchosen: crate::Binding!("remaining_creatures"),
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::sacrifice(EffectRecipientDef::objects(
                        ObjectSetDef::Binding(crate::Binding!("sacrifice_creature")),
                    )),
                }),
            ]),
        ),
    ]),
);

// DSK 123 — Winter's Intervention
pub(in crate::card::sets) static WINTER_S_INTERVENTION: CardRecord = CardRecord::new(
    "Winter's Intervention",
    "b33d38fa-bbcf-4bcc-93a8-adbfd9c93faa",
    "Cristi Balanescu",
    CardRules::new_instant(mana_cost!("{1}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Winter's Intervention deals 2 damage to target creature. You \
         gain 2 life.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(2),
            ),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ]),
    )]),
);

// DSK 124 — Withering Torment
pub(in crate::card::sets) static WITHERING_TORMENT: CardRecord = CardRecord::new(
    "Withering Torment",
    "38048d14-3463-4157-951f-1d68ec78a64e",
    "Inkognit",
    CardRules::new_instant(mana_cost!("{2}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Destroy target creature or enchantment. You lose 2 life.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::HasType(CardType::Enchantment),
            ]),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ]),
    )]),
);

// DSK 125 — Bedhead Beastie
pub(in crate::card::sets) static BEDHEAD_BEASTIE: CardRecord = CardRecord::new(
    "Bedhead Beastie",
    "77c72b5b-dd81-4eb4-80d9-a0124e27e1dc",
    "David Auden Nash",
    CardRules::new_creature(mana_cost!("{4}{R}{R}"), &["Beast"], 5, 6).with_abilities(&[
        abilities::menace(),
        abilities::typecycling!(
            "Mountaincycling {2}",
            &[CostDef::Mana(mana_cost!("{2}"))],
            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Mountain"))
        ),
    ]),
);

// DSK 126 — Betrayer's Bargain
pub(in crate::card::sets) static BETRAYER_S_BARGAIN: CardRecord = CardRecord::new(
    "Betrayer's Bargain",
    "7956ae00-8f0c-48f0-8110-19ff53863876",
    "Billy Christian",
    CardRules::new_instant(mana_cost!("{1}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Betrayer's Bargain deals 5 damage to target creature. If that \
         creature would die this turn, exile it instead.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(5),
            ),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::ExileInsteadOfDying),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )
    .with_spell_additional_cost(&CostDef::Choice(&[
        CostDef::Sacrifice {
            object: ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::HasType(CardType::Enchantment),
            ]),
            quantity: CostQuantityDef::Fixed(1),
        },
        CostDef::Mana(mana_cost!("{2}")),
    ]))]),
);

// DSK 127 — Boilerbilges Ripper
// Audit: unsupported — Needs a reflexive trigger installed by the resolving sacrifice clause, retaining its source even after it leaves. SacrificePerformed currently only comes from the legacy sacrifice-of-choice path and requires a live source.
pub(in crate::card::sets) static BOILERBILGES_RIPPER: CardRecord = CardRecord::new(
    "Boilerbilges Ripper",
    "1a68009c-83cd-455f-81e9-bdd720d23a43",
    "Kai Carpenter",
    CardRules::unsupported(),
);

// DSK 128 — Chainsaw (alternate printing)
const CHAINSAW_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CHAINSAW,
    1,
    "54e0c2cd-fa5f-427d-8e15-6066f002a8e3",
    "J.P. Targete",
);

// DSK 129 — Charred Foyer // Warped Space
// Audit: unsupported — Needs a once-per-turn alternative spell cost applicable specifically to casts from exile; current cast permissions cannot represent that independent limited alternative cost.
pub(in crate::card::sets) static CHARRED_FOYER: CardRecord = CardRecord::new(
    "Charred Foyer // Warped Space",
    "a128e6d1-b90f-45a1-b587-f8c29bd0ec8c",
    "Andrew Mar",
    CardRules::unsupported(),
);

// DSK 130 — Clockwork Percussionist (alternate printing)
const CLOCKWORK_PERCUSSIONIST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CLOCKWORK_PERCUSSIONIST,
    1,
    "10986e5a-9fc6-41e2-8352-289328245171",
    "Eric Wilkerson",
);

// DSK 131 — Cursed Recording
// Audit: unsupported — Needs a delayed trigger that expires after either one matching spell or the current turn; installed triggers offer separate once and turn lifetimes, not their intersection.
pub(in crate::card::sets) static CURSED_RECORDING: CardRecord = CardRecord::new(
    "Cursed Recording",
    "d13a247c-c941-488a-b13b-bffb1f1f368a",
    "Kim Sokol",
    CardRules::unsupported(),
);

// DSK 132 — Diversion Specialist
pub(in crate::card::sets) static DIVERSION_SPECIALIST: CardRecord = CardRecord::new(
    "Diversion Specialist",
    "acc9b17c-4210-49f4-a920-b1dc9dfa950f",
    "Tuan Duong Chu",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Human", "Warrior"], 4, 3).with_abilities(&[
        abilities::menace(),
        AbilityDef::activated(
            "{1}, Sacrifice another creature or enchantment: Exile the top \
             card of your library. You may play it this turn.",
            &[
                CostDef::Mana(mana_cost!("{1}")),
                CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                    ]),
                ])),
            ],
            EffectDef::ExileTopOfLibraryToPlay {
                player: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
                duration: ExilePlayDurationDef::ThisTurn,
                free: false,
                face_down: false,
                spend_any_color: false,
                play_condition: None,
                cast_only: false,
            },
        ),
    ]),
);

// DSK 133 — Enduring Courage
// Audit: unsupported — Needs a return-to-battlefield instruction that establishes the permanent as a noncreature enchantment before entry replacement effects and enters triggers inspect it; applying a type change after the return produces an incorrect creature entry.
pub(in crate::card::sets) static ENDURING_COURAGE: CardRecord = CardRecord::new(
    "Enduring Courage",
    "f46ac55f-d68e-4d5d-af0a-3879f97f705e",
    "Yigit Koroglu",
    CardRules::unsupported(),
);

// DSK 134 — Fear of Being Hunted
// Audit: unsupported — Needs a combat requirement that this attacker be blocked by at least one creature if able; MustBeBlockedBy instead requires every matching creature to block, and minimum-blocker restrictions only constrain blocks that are declared.
pub(in crate::card::sets) static FEAR_OF_BEING_HUNTED: CardRecord = CardRecord::new(
    "Fear of Being Hunted",
    "a0a5e716-68c1-4fa0-aa08-5b08114e08d8",
    "Maxime Minard",
    CardRules::unsupported(),
);

// DSK 135 — Fear of Burning Alive
// Audit: unsupported — Needs a noncombat-only damage matcher; the shared damage-kind vocabulary currently distinguishes combat from any damage, not noncombat.
pub(in crate::card::sets) static FEAR_OF_BURNING_ALIVE: CardRecord = CardRecord::new(
    "Fear of Burning Alive",
    "b282f8e3-8b79-47e9-8c18-62284211442b",
    "J.P. Targete",
    CardRules::unsupported(),
);

// DSK 136 — Fear of Missing Out
pub(in crate::card::sets) static FEAR_OF_MISSING_OUT: CardRecord = CardRecord::new(
    "Fear of Missing Out",
    "9d48aaff-46ab-411b-9456-171d4709f951",
    "John Stanko",
    // Two mana for a body that fills its own graveyard on the way in and
    // then, once the graveyard is deep enough, hands the whole team a second
    // attack.
    CardRules::new_enchantment_creature(mana_cost!("{1}{R}"), &["Nightmare"], 2, 3).with_abilities(
        &[
            abilities::enters_trigger(
                "When this creature enters, discard a card, then draw a card.",
                EffectDef::Sequence(&[
                    EffectDef::Discard {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                        selection: DiscardSelectionDef::RecipientChooses,
                        then: None,
                    },
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                ]),
            ),
            AbilityDef::triggered_if_with_targets(
                "Delirium — Whenever this creature attacks for the first time each turn, if there \
                 are four or more card types among cards in your graveyard, untap target \
                 creature. After this phase, there is an additional combat phase.",
                TriggerEventDef::attacks_first_time_this_turn(ObjectPredicateDef::Source),
                &// Delirium: four or more card types among the cards in your graveyard,
                    // counted as the trigger is placed and again as it resolves. The discard
                    // his own arrival asks for is often what turns it on.
                    TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                        left: ValueDef::CardTypesAmongGraveyards(PlayerRelation::You),
                        comparison: ComparisonDef::GreaterOrEqual,
                        right: ValueDef::Constant(4),
                    }),
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                // Untapping is what makes the extra combat worth having: the creature that
                // just attacked can attack again.
                EffectDef::Sequence(&[
                    EffectDef::Untap {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                    EffectDef::ScheduleTurnPhases(&[TurnPhaseDef::Combat]),
                ]),
            ),
        ],
    ),
);

// DSK 137 — Glassworks // Shattered Yard
pub(in crate::card::sets) static GLASSWORKS: CardRecord = CardRecord::new(
    "Glassworks // Shattered Yard",
    "fe32f667-8d9f-4414-913e-256cbc2fbc45",
    "Sergey Glushakov",
    CardRules::new_enchantment(mana_cost!("{2}{R}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::triggered_with_targets(
            "When you unlock this door, this Room deals 4 damage to target \
             creature an opponent controls.",
            TriggerEventDef::DoorUnlocked,
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(4),
            ),
        )]),
)
.with_composition(|| {
    const FRONT: CardRules = CardRules::new_enchantment(mana_cost!("{2}{R}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::triggered_with_targets(
            "When you unlock this door, this Room deals 4 damage to target \
             creature an opponent controls.",
            TriggerEventDef::DoorUnlocked,
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(4),
            ),
        )]);
    const BACK: CardRules = CardRules::new_enchantment(mana_cost!("{4}{R}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::triggered(
            "At the beginning of your end step, this Room deals 1 damage \
             to each opponent.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::You,
            },
            EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::Constant(1)),
        )]);
    const BOTH: CardRules = CardRules::new_enchantment(mana_cost!("{6}{R}{R}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[
            AbilityDef::triggered_with_targets(
                "When you unlock this door, this Room deals 4 damage to target \
                 creature an opponent controls.",
                TriggerEventDef::DoorUnlocked,
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::Opponent),
                        owner: None,
                    },
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(4),
                ),
            ),
            AbilityDef::triggered(
                "At the beginning of your end step, this Room deals 1 damage \
                 to each opponent.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::End,
                    player: PlayerRelation::You,
                },
                EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::Constant(1)),
            ),
        ]);
    CardComposition::room(
        "Glassworks // Shattered Yard",
        "Glassworks",
        FRONT,
        "Shattered Yard",
        BACK,
        BOTH,
    )
});

// DSK 138 — Grab the Prize
pub(in crate::card::sets) static GRAB_THE_PRIZE: CardRecord = CardRecord::new(
"Grab the Prize",
"50895202-f1a1-4840-a11a-55b78b8b5929",
"Halil Ural",
CardRules::new_sorcery(mana_cost!("{1}{R}")).with_abilities(&[
AbilityDef::spell_with_additional_cost("As an additional cost to cast this spell, discard a card.\nDraw two cards. If the discarded card wasn't a land card, Grab the Prize deals 2 damage to each opponent.", &[], CostDef::discard(ObjectPredicateDef::Any), EffectDef::Sequence(&[abilities::draw_cards(ValueDef::Constant(2)), EffectDef::BindObjects(BindObjectsDef { source: ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::One(ObjectRefDef::AdditionalCostObject(crate::AdditionalCostObjectIndex::PRIMARY))), binding: ParentBinding, then: &EffectDef::IfCondition { condition: &TriggerConditionDef::ObjectSetCount(&crate::card::ObjectSetCountConditionDef { objects: &ObjectSetDef::Binding(ParentBinding), predicate: crate::card::ObjectSetPredicateDef { filter: Some(crate::card::ObjectSetFilterDef::Predicate(&ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)))), comparison: ComparisonDef::GreaterOrEqual, amount: 1 } }), then: &EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::Constant(2)) } })]))
]),
);

// DSK 139 — Hand That Feeds
pub(in crate::card::sets) static HAND_THAT_FEEDS: CardRecord = CardRecord::new(
    "Hand That Feeds",
    "297c2860-5a68-4a18-a23a-ca5cfdfbcac8",
    "Loïc Canavaggia",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Mutant"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "Delirium — Whenever this creature attacks while there are \
             four or more card types among cards in your graveyard, it \
             gets +2/+0 and gains menace until end of turn. (It can't be \
             blocked except by two or more creatures.)",
            TriggerEventDef::While {
                event: &TriggerEventDef::attacks(ObjectPredicateDef::Source),
                condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::CardTypesAmongGraveyards(PlayerRelation::You),
                    comparison: ComparisonDef::GreaterOrEqual,
                    right: ValueDef::Constant(4),
                }),
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(0),
                    ),
                    AppliedEffectDef::add_ability(&abilities::menace()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// DSK 140 — Impossible Inferno
// Audit: unsupported — Needs exile-play permission that expires at cleanup of the controller's next turn; the current turn-count permission can remain usable during the following opponent turn.
pub(in crate::card::sets) static IMPOSSIBLE_INFERNO: CardRecord = CardRecord::new(
    "Impossible Inferno",
    "a35248f9-9a4e-4758-a4c1-0e0c83e3fd75",
    "Edgar Sánchez Hidalgo",
    CardRules::unsupported(),
);

// DSK 141 — Infernal Phantom
// Audit: unsupported — Needs a committed event for a Room becoming fully unlocked, observed by other permanents and graveyard abilities. DoorUnlocked only dispatches to the door being opened, so it cannot implement the second eerie trigger.
pub(in crate::card::sets) static INFERNAL_PHANTOM: CardRecord = CardRecord::new(
    "Infernal Phantom",
    "f0c5999f-a185-4fc7-86fa-c4e5b091e768",
    "Mike Sass",
    CardRules::unsupported(),
);

// DSK 142 — Irreverent Gremlin
// Audit: unsupported — Needs a once-per-turn limit on accepting and completing the discard choice, rather than on the number of triggers; declining must leave later triggers usable.
pub(in crate::card::sets) static IRREVERENT_GREMLIN: CardRecord = CardRecord::new(
    "Irreverent Gremlin",
    "8da254f5-53f2-41d2-a4f0-a90b3dd6209c",
    "Fajareka Setiawan",
    CardRules::unsupported(),
);

// DSK 143 — Leyline of Resonance
pub(in crate::card::sets) static LEYLINE_OF_RESONANCE: CardRecord = CardRecord::new(
    "Leyline of Resonance",
    "92c5f0e3-345a-40a8-9cda-565a62156692",
    "Sergey Glushakov",
CardRules::new_enchantment(mana_cost!("{2}{R}{R}")).with_abilities(&[
        abilities::begin_game_on_battlefield(),
        AbilityDef::triggered(
            "Whenever you cast an instant or sorcery spell that targets only a single creature you control, copy that spell. You may choose new targets for the copy.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::HasType(CardType::Sorcery),
                ]),
                ObjectPredicateDef::DeclaredTargetCount {
                    minimum: 1,
                    maximum: 1,
                },
                ObjectPredicateDef::TargetsObjectMatching(
                    &ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                ),
            ])),
            EffectDef::CopyStackObject(&CopyStackObjectDef {
                object: EffectRecipientDef::TriggeringObject,
                controller: PlayerRefDef::EffectController,
                count: ValueDef::Constant(1),
                retarget: true,
                colors: None,
            }),
        ),
    ]),
);

// DSK 144 — Most Valuable Slayer
pub(in crate::card::sets) static MOST_VALUABLE_SLAYER: CardRecord = CardRecord::new(
    "Most Valuable Slayer",
    "c2b7b635-4c72-4129-bf95-1eef05cce3d3",
    "Patrik Hell",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Human", "Warrior"], 2, 4).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "Whenever you attack, target attacking creature gets +1/+0 and \
             gains first strike until end of turn.",
            TriggerEventDef::attack_declared(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                1,
                None,
            ),
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Attacking,
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
                    AppliedEffectDef::add_ability(&abilities::first_strike()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// DSK 145 — Norin, Swift Survivalist
pub(in crate::card::sets) static NORIN_SWIFT_SURVIVALIST: CardRecord = CardRecord::new(
    "Norin, Swift Survivalist",
    "49f0fdf4-3881-4327-924f-2c1b67ccda93",
    "Yigit Koroglu",
    CardRules::new_creature(mana_cost!("{R}"), &["Human", "Coward"], 2, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Norin can't block.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                },
            ),
            AbilityDef::triggered(
                "Whenever a creature you control becomes blocked, you may \
                 exile it. You may play that card from exile this turn.",
                TriggerEventDef::BecomesBlocked(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    effect: &EffectDef::ExileGrantingControllerPlayThisTurn {
                        object: EffectRecipientDef::TriggeringObject,
                    },
                },
            ),
        ]),
);

// DSK 146 — Overlord of the Boilerbilges
pub(in crate::card::sets) static OVERLORD_OF_THE_BOILERBILGES: CardRecord = CardRecord::new(
    "Overlord of the Boilerbilges",
    "d58d3545-043a-457a-8324-facc3c2363ec",
    "Helge C. Balzer",
    CardRules::new_enchantment_creature(mana_cost!("{4}{R}{R}"), &["Avatar", "Horror"], 5, 5)
        .with_abilities(&[
            impending_cast(
                &[CostDef::Mana(mana_cost!("{2}{R}{R}"))],
                "Impending 4—{2}{R}{R} (If you cast this spell for its \
                 impending cost, it enters with four time counters and isn't a \
                 creature until the last is removed. At the beginning of your \
                 end step, remove a time counter from it.)",
            ),
            impending_entry(4),
            impending_countdown(),
            AbilityDef::triggered_with_targets(
                "Whenever this permanent enters or attacks, it deals 4 damage \
                 to any target.",
                TriggerEventDef::AnyOf(&[
                    TriggerEventDef::zone_changed(
                        ObjectPredicateDef::Source,
                        None,
                        Some(ZoneKind::Battlefield),
                    ),
                    TriggerEventDef::attacks(ObjectPredicateDef::Source),
                ]),
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::AnyTarget,
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(4),
                ),
            ),
        ]),
);

// DSK 147 — Painter's Studio // Defaced Gallery
// Audit: unsupported — Needs exile-play permission that expires at cleanup of the controller's next turn; the current turn-count permission can remain usable during the following opponent turn.
pub(in crate::card::sets) static PAINTER_S_STUDIO: CardRecord = CardRecord::new(
    "Painter's Studio // Defaced Gallery",
    "e96901eb-5b57-43f4-a7b1-ae3b809bc36e",
    "Marc Simonetti",
    CardRules::unsupported(),
);

// DSK 148 — Piggy Bank
pub(in crate::card::sets) static PIGGY_BANK: CardRecord = CardRecord::new(
    "Piggy Bank",
    "55d35442-5ca0-4fd7-8ff1-b7347b3e6690",
    "Steve Ellis",
    CardRules::new_artifact_creature(mana_cost!("{1}{R}"), &["Boar", "Toy"], 3, 2).with_abilities(
        &[abilities::dies_trigger(
            "When this creature dies, create a Treasure token. (It's an \
             artifact with \"{T}, Sacrifice this token: Add one mana of \
             any color.\")",
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                    .with_count(ValueDef::Constant(1)),
            ),
        )],
    ),
);

// DSK 149 — Pyroclasm (reprint)
const PYROCLASM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::PYROCLASM,
    "4391b0af-2f26-4a45-9e2a-5bd8e9838107",
    "Néstor Ossandón Leal",
);

// DSK 150 — Ragged Playmate
pub(in crate::card::sets) static RAGGED_PLAYMATE: CardRecord = CardRecord::new(
    "Ragged Playmate",
    "000499a4-3f39-4d25-a2a2-b2f014e30754",
    "Kaitlyn McCulley",
    CardRules::new_artifact_creature(mana_cost!("{1}{R}"), &["Toy"], 2, 2).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{1}, {T}: Target creature with power 2 or less can't be \
             blocked this turn.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::PowerLessThan(ValueDef::Constant(3)),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// DSK 151 — Rampaging Soulrager
// Audit: unsupported — Needs a continuous value or condition counting unlocked doors across controlled Rooms; counting permanents does not count their individual doors.
pub(in crate::card::sets) static RAMPAGING_SOULRAGER: CardRecord = CardRecord::new(
    "Rampaging Soulrager",
    "569c914b-95af-428f-a142-6f20e418bc59",
    "Slawomir Maniak",
    CardRules::unsupported(),
);

// DSK 152 — Razorkin Hordecaller
pub(in crate::card::sets) static RAZORKIN_HORDECALLER: CardRecord = CardRecord::new(
    "Razorkin Hordecaller",
    "f7fb0f11-d1d0-4941-a1a7-a2db88f30394",
    "David Álvarez",
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Human", "Clown", "Berserker"], 4, 4)
        .with_abilities(&[
            abilities::haste(),
            AbilityDef::triggered(
                "Whenever you attack, create a 1/1 red Gremlin creature token.",
                TriggerEventDef::attack_declared(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    1,
                    None,
                ),
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(GREMLIN_TOKEN))),
            ),
        ]),
);

// DSK 153 — Razorkin Needlehead
pub(in crate::card::sets) static RAZORKIN_NEEDLEHEAD: CardRecord = CardRecord::new(
    "Razorkin Needlehead",
    "bc73b963-23c0-46d2-853a-34a8b463994e",
    "Riccardo Federici",
    CardRules::new_creature(mana_cost!("{R}{R}"), &["Human", "Assassin"], 2, 2).with_abilities(&[
        AbilityDef::static_ability(
            "This creature has first strike during your turn.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::You),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                },
            },
        ),
        AbilityDef::triggered(
            "Whenever an opponent draws a card, this creature deals 1 \
             damage to them.",
            TriggerEventDef::DrewCard(DrawEventMatcherDef::any(PlayerRelation::Opponent)),
            EffectDef::damage(EffectRecipientDef::EventPlayer, ValueDef::Constant(1)),
        ),
    ]),
);

// DSK 154 — Ripchain Razorkin
pub(in crate::card::sets) static RIPCHAIN_RAZORKIN: CardRecord = CardRecord::new(
    "Ripchain Razorkin",
    "effad662-3309-434c-addc-4394db1f359c",
    "David Szabo",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Human", "Berserker"], 5, 3).with_abilities(&[
        abilities::reach(),
        AbilityDef::activated(
            "{2}{R}, Sacrifice a land: Draw a card.",
            &[
                CostDef::Mana(mana_cost!("{2}{R}")),
                CostDef::sacrifice_permanent(ObjectPredicateDef::HasType(CardType::Land)),
            ],
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
    ]),
);

// DSK 155 — The Rollercrusher Ride
// Audit: unsupported — Needs a noncombat-only damage replacement matcher and a variable target-count limit tied to cast X for the enters trigger.
pub(in crate::card::sets) static THE_ROLLERCRUSHER_RIDE: CardRecord = CardRecord::new(
    "The Rollercrusher Ride",
    "70019956-fca1-4090-b3b6-6a963528e05b",
    "Deruchenko Alexander",
    CardRules::unsupported(),
);

// DSK 156 — Scorching Dragonfire (reprint)
const SCORCHING_DRAGONFIRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_eld::SCORCHING_DRAGONFIRE,
    "a55f4f37-796a-4b18-a5ee-1d4f711f19b8",
    "Marta Nael",
);

// DSK 157 — Screaming Nemesis (alternate printing)
const SCREAMING_NEMESIS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SCREAMING_NEMESIS,
    1,
    "ce35e6fb-ff54-44c4-a216-7ddd37f46882",
    "Liiga Smilshkalne",
);

// DSK 158 — Ticket Booth // Tunnel of Hate
pub(in crate::card::sets) static TICKET_BOOTH: CardRecord = CardRecord::new(
    "Ticket Booth // Tunnel of Hate",
    "058ffe36-ed2d-4b67-83cd-162db8383a32",
    "Marco Gorlei",
    CardRules::new_enchantment(mana_cost!("{2}{R}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::triggered(
            "When you unlock this door, manifest dread.",
            TriggerEventDef::DoorUnlocked,
            abilities::manifest_dread(),
        )]),
)
.with_composition(|| {
    const FRONT: CardRules = CardRules::new_enchantment(mana_cost!("{2}{R}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::triggered(
            "When you unlock this door, manifest dread.",
            TriggerEventDef::DoorUnlocked,
            abilities::manifest_dread(),
        )]);
    const BACK: CardRules = CardRules::new_enchantment(mana_cost!("{4}{R}{R}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::triggered_with_targets(
            "Whenever you attack, target attacking creature gains double \
             strike until end of turn.",
            TriggerEventDef::attack_declared(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                1,
                None,
            ),
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Attacking,
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::double_strike()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )]);
    const BOTH: CardRules = CardRules::new_enchantment(mana_cost!("{6}{R}{R}{R}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[
            AbilityDef::triggered(
                "When you unlock this door, manifest dread.",
                TriggerEventDef::DoorUnlocked,
                abilities::manifest_dread(),
            ),
            AbilityDef::triggered_with_targets(
                "Whenever you attack, target attacking creature gains double \
                 strike until end of turn.",
                TriggerEventDef::attack_declared(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    1,
                    None,
                ),
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Attacking,
                    ]),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&abilities::double_strike()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]);
    CardComposition::room(
        "Ticket Booth // Tunnel of Hate",
        "Ticket Booth",
        FRONT,
        "Tunnel of Hate",
        BACK,
        BOTH,
    )
});

// DSK 159 — Trial of Agony
pub(in crate::card::sets) static TRIAL_OF_AGONY: CardRecord = CardRecord::new(
    "Trial of Agony",
    "fa62f67a-d20f-4d99-b0a2-327634299c9f",
    "Mike Sass",
    CardRules::new_sorcery(mana_cost!("{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Choose two target creatures controlled by the same opponent. \
         That player chooses one of those creatures. Trial of Agony \
         deals 5 damage to that creature, and the other can't block \
         this turn.",
        &[AbilityTargetDef {
            minimum: 2,
            maximum: 2,
            ..AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::Opponent),
                owner: None,
            })
        }],
        EffectDef::Choose(ChooseDef {
            binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
            unchosen: Some(crate::Binding!("other")),
            chooser: PlayerRefDef::Opponent,
            candidates: ObjectSetDef::LegalTargets(TargetIndex::PRIMARY),
            exclude: None,
            minimum: 1,
            maximum: 1,
            visibility: ChoiceVisibilityDef::Public,
            then: &EffectDef::Sequence(&[
                EffectDef::damage(
                    EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!("chosen"))),
                    ValueDef::Constant(5),
                ),
                EffectDef::Apply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                        "other"
                    ))),
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        }),
    )]),
);

// DSK 160 — Turn Inside Out
// Audit: unsupported — Needs a delayed dies trigger tied to the targeted creature for this turn, retaining the spell controller; granting a dies ability to the creature changes who manifests if that creature changes controller.
pub(in crate::card::sets) static TURN_INSIDE_OUT: CardRecord = CardRecord::new(
    "Turn Inside Out",
    "57e2a92c-06d3-4cb5-883d-cba428a7e98e",
    "Loïc Canavaggia",
    CardRules::unsupported(),
);

// DSK 161 — Untimely Malfunction
pub(in crate::card::sets) static UNTIMELY_MALFUNCTION: CardRecord = CardRecord::new(
    "Untimely Malfunction",
    "857bfb0e-17dc-4dda-bc37-3df927a9eae6",
    "Jarel Threat",
    CardRules::new_instant(mana_cost!("{1}{R}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Destroy target artifact.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Artifact),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            ),
            AbilityDef::spell_with_targets(
                "Change the target of target spell or ability with a single \
                 target.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::DeclaredTargetCount {
                            minimum: 1,
                            maximum: 1,
                        },
                        zones: &[ZoneKind::Stack],
                        controller: None,
                        owner: None,
                    },
                )],
                EffectDef::ChangeStackTargets(&ChangeStackTargetsDef {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    chooser: PlayerRefDef::EffectController,
                    change: StackTargetChangeDef::ChooseNew {
                        optional: false,
                        restriction: None,
                    },
                }),
            ),
            AbilityDef::spell_with_targets(
                "One or two target creatures can't block this turn.",
                &[AbilityTargetDef {
                    minimum: 1,
                    maximum: 2,
                    ..AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                        CardType::Creature,
                    ))
                }],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ],
    )]),
);

// DSK 162 — Vengeful Possession
pub(in crate::card::sets) static VENGEFUL_POSSESSION: CardRecord = CardRecord::new(
    "Vengeful Possession",
    "d6918d50-a4c3-40d8-9480-343f14773a69",
    "Dominik Mayer",
    CardRules::new_sorcery(mana_cost!("{2}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Gain control of target creature until end of turn. Untap it. \
         It gains haste until end of turn. You may discard a card. If \
         you do, draw a card.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Perform(GameActionDef::GainControl {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                controller: PlayerRefDef::EffectController,
                duration: ControlDurationDef::UntilEndOfTurn,
            }),
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::haste()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::DiscardCards(1)],
                &abilities::draw_cards(ValueDef::Constant(1)),
            )),
        ]),
    )]),
);

// DSK 163 — Vicious Clown
pub(in crate::card::sets) static VICIOUS_CLOWN: CardRecord = CardRecord::new(
    "Vicious Clown",
    "12649f73-9d29-4eed-bc89-cc0d1a8969e3",
    "Johann Bodin",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Clown"], 2, 3).with_abilities(&[
        AbilityDef::triggered(
            "Whenever another creature you control with power 2 or less \
             enters, this creature gets +2/+0 until end of turn.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ObjectPredicateDef::PowerLessThan(ValueDef::Constant(3)),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// DSK 164 — Violent Urge
pub(in crate::card::sets) static VIOLENT_URGE: CardRecord = CardRecord::new(
    "Violent Urge",
    "a47c968b-1edd-45ac-a67c-311647e7e2fc",
    "Mirko Failoni",
    CardRules::new_instant(mana_cost!("{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets +1/+0 and gains first strike until end \
         of turn.\nDelirium — If there are four or more card types \
         among cards in your graveyard, that creature gains double \
         strike until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
                    AppliedEffectDef::add_ability(&abilities::first_strike()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::CardTypesAmongGraveyards(PlayerRelation::You),
                    comparison: ComparisonDef::GreaterOrEqual,
                    right: ValueDef::Constant(4),
                }),
                then: &EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&abilities::double_strike()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            },
        ]),
    )]),
);

// DSK 165 — Waltz of Rage
// Audit: unsupported — Needs exile-play permission that expires at cleanup of the controller's next turn; the current turn-count permission can remain usable during the following opponent turn.
pub(in crate::card::sets) static WALTZ_OF_RAGE: CardRecord = CardRecord::new(
    "Waltz of Rage",
    "abf17d8b-12bc-4122-865d-50cf91f04f67",
    "Mathias Kollros",
    CardRules::unsupported(),
);

// DSK 166 — Altanak, the Thrice-Called
pub(in crate::card::sets) static ALTANAK_THE_THRICE_CALLED: CardRecord = CardRecord::new(
    "Altanak, the Thrice-Called",
    "807b0674-8eba-4204-b6b6-fa2b785a79e9",
    "Sam Wolfe Connelly",
    CardRules::new_creature(mana_cost!("{5}{G}{G}"), &["Insect", "Beast"], 9, 9)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::trample(),
            AbilityDef::triggered(
                "Whenever Altanak becomes the target of a spell or ability an \
                 opponent controls, draw a card.",
                TriggerEventDef::becomes_targeted(ObjectPredicateDef::ControlledBy(
                    PlayerRelation::Opponent,
                )),
                abilities::draw_cards(ValueDef::Constant(1)),
            ),
            AbilityDef::activated_with_targets(
                "{1}{G}, Discard this card: Return target land card from your \
                 graveyard to the battlefield tapped.",
                &[CostDef::Mana(mana_cost!("{1}{G}")), CostDef::DiscardSource],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Land),
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: Some(PlayerRelation::You),
                    },
                )],
                EffectDef::WithBattlefieldArrival {
                    effect: &EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Battlefield,
                        ZonePlacement::Top,
                    ),
                    arrival: BattlefieldArrivalDef {
                        modifications: &[BattlefieldEntryModificationDef::Tapped],
                        ..BattlefieldArrivalDef::DEFAULT
                    },
                },
            )
            .with_source_zones(&[ZoneKind::Hand]),
        ]),
);

// DSK 167 — Anthropede
// Audit: unsupported — Needs a resolving payment alternative combining private discard selection with mana payment, then a reflexive trigger that chooses a Room target after payment.
pub(in crate::card::sets) static ANTHROPEDE: CardRecord = CardRecord::new(
    "Anthropede",
    "51216ab0-9806-4faa-afbd-143e95dc255b",
    "Loïc Canavaggia",
    CardRules::unsupported(),
);

// DSK 168 — Balustrade Wurm
pub(in crate::card::sets) static BALUSTRADE_WURM: CardRecord = CardRecord::new(
    "Balustrade Wurm",
    "76610e6e-b60f-494f-bb11-68371eb494f2",
    "Maxime Minard",
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Wurm"], 5, 5).with_abilities(&[
        AbilityDef::static_ability(
            "This spell can't be countered.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBeCountered),
            },
        )
        .with_source_zones(&[ZoneKind::Stack]),
        abilities::trample(),
        abilities::haste(),
        AbilityDef::activated(
            "Delirium — {2}{G}{G}: Return this card from your graveyard to \
             the battlefield with a finality counter on it. Activate only \
             if there are four or more card types among cards in your \
             graveyard and only as a sorcery.",
            &[CostDef::Mana(mana_cost!("{2}{G}{G}"))],
            EffectDef::WithBattlefieldArrival {
                effect: &EffectDef::move_to_zone(
                    EffectRecipientDef::Source,
                    ZoneKind::Battlefield,
                    ZonePlacement::Top,
                ),
                arrival: BattlefieldArrivalDef {
                    modifications: &[BattlefieldEntryModificationDef::AddCountersValue {
                        kind: CounterKind::Finality,
                        amount: ValueDef::Constant(1),
                    }],
                    ..BattlefieldArrivalDef::DEFAULT
                },
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed)
        .with_source_zones(&[ZoneKind::Graveyard])
        .with_activation_condition(&TriggerConditionDef::ValueComparison(&ValueComparisonDef {
            left: ValueDef::CardTypesAmongGraveyards(PlayerRelation::You),
            comparison: ComparisonDef::GreaterOrEqual,
            right: ValueDef::Constant(4),
        })),
    ]),
);

// DSK 169 — Bashful Beastie
pub(in crate::card::sets) static BASHFUL_BEASTIE: CardRecord = CardRecord::new(
    "Bashful Beastie",
    "c20fa7ee-a4c2-4eb0-9467-195f3b894fa0",
    "Aaron Miller",
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Beast"], 5, 4).with_abilities(&[
        abilities::dies_trigger(
            "When this creature dies, manifest dread. (Look at the top two \
             cards of your library. Put one onto the battlefield face down \
             as a 2/2 creature and the other into your graveyard. Turn it \
             face up any time for its mana cost if it's a creature card.)",
            abilities::manifest_dread(),
        ),
    ]),
);

// DSK 170 — Break Down the Door
pub(in crate::card::sets) static BREAK_DOWN_THE_DOOR: CardRecord = CardRecord::new(
    "Break Down the Door",
    "209e9bbc-a15d-47fc-b149-6b0c57dd09ea",
    "Ralph Horsley",
    CardRules::new_instant(mana_cost!("{2}{G}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Exile target artifact.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Artifact),
                )],
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
            ),
            AbilityDef::spell_with_targets(
                "Exile target enchantment.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                )],
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
            ),
            AbilityDef::spell(
                "Manifest dread. (Look at the top two cards of your library. \
                 Put one onto the battlefield face down as a 2/2 creature and \
                 the other into your graveyard. Turn it face up any time for \
                 its mana cost if it's a creature card.)",
                abilities::manifest_dread(),
            ),
        ],
    )]),
);

// DSK 171 — Cathartic Parting
pub(in crate::card::sets) static CATHARTIC_PARTING: CardRecord = CardRecord::new(
    "Cathartic Parting",
    "6afc250a-854d-4555-ba78-db9283fa7c22",
    "Miranda Meeks",
    CardRules::new_sorcery(mana_cost!("{1}{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "The owner of target artifact or enchantment an opponent \
         controls shuffles it into their library. You may shuffle up \
         to four target cards from your graveyard into your library.",
        &[
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::Opponent),
                owner: None,
            }),
            AbilityTargetDef {
                minimum: 0,
                maximum: 4,
                ..AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                })
            },
        ],
        EffectDef::Sequence(&[
            EffectDef::Sequence(&[
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Library,
                    ZonePlacement::Top,
                ),
                EffectDef::ShuffleLibrary {
                    player: EffectRecipientDef::player(PlayerRefDef::OwnerOf(
                        ObjectRefDef::Target(TargetIndex::PRIMARY),
                    )),
                },
            ]),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Sequence(&[
                    EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex(1)),
                        ZoneKind::Library,
                        ZonePlacement::Top,
                    ),
                    EffectDef::ShuffleLibrary {
                        player: EffectRecipientDef::Controller,
                    },
                ]),
            },
        ]),
    )]),
);

// DSK 172 — Cautious Survivor
// Audit: unsupported — Needs the ordinal number of the current main phase in the turn (CR 505.1b); PostcombatMain also matches third and later main phases, so it cannot restrict survival to the second main phase.
pub(in crate::card::sets) static CAUTIOUS_SURVIVOR: CardRecord = CardRecord::new(
    "Cautious Survivor",
    "ee2b4c1a-e058-4e06-bc46-e250fd9c9b54",
    "Jodie Muir",
    CardRules::unsupported(),
);

// DSK 173 — Coordinated Clobbering
pub(in crate::card::sets) static COORDINATED_CLOBBERING: CardRecord = CardRecord::new(
    "Coordinated Clobbering",
    "d498cd5d-5807-4297-bc8a-c0941f2f5ce2",
    "Fajareka Setiawan",
    CardRules::new_sorcery(mana_cost!("{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Tap one or two target untapped creatures you control. They \
         each deal damage equal to their power to target creature an \
         opponent controls.",
        &[
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Tapped),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            }),
            AbilityTargetDef {
                minimum: 0,
                maximum: 1,
                another: true,
                ..AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Tapped),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                })
            },
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::Opponent),
                owner: None,
            }),
        ],
        EffectDef::BindObjects(BindObjectsDef {
            source: ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::LegalTargets(TargetIndex(
                0,
            ))),
            binding: crate::Binding!("first"),
            then: &EffectDef::BindObjects(BindObjectsDef {
                source: ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::LegalTargets(
                    TargetIndex(1),
                )),
                binding: crate::Binding!("second"),
                then: &EffectDef::BindObjects(BindObjectsDef {
                    source: ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::LegalTargets(
                        TargetIndex(2),
                    )),
                    binding: crate::Binding!("enemy"),
                    then: &EffectDef::Sequence(&[
                        EffectDef::Tap {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                crate::Binding!("first"),
                            )),
                        },
                        EffectDef::Tap {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                crate::Binding!("second"),
                            )),
                        },
                        EffectDef::IfNoObjects(IfNoObjectsDef {
                            input: ObjectSetDef::Binding(crate::Binding!("first")),
                            if_empty: &EffectDef::ForEachInBinding {
                                objects: crate::Binding!("second"),
                                binding: crate::Binding!("only"),
                                effect: &EffectDef::DealDamage(DamageDef::simultaneous(&[
                                    DamageAssignmentDef::from(
                                        ObjectRefDef::Binding(crate::Binding!("only")),
                                        EffectRecipientDef::objects(ObjectSetDef::Binding(
                                            crate::Binding!("enemy"),
                                        )),
                                        ValueDef::AggregateObjectValues(&ObjectValueAggregateDef {
                                            objects: ObjectSetDef::One(ObjectRefDef::Binding(
                                                crate::Binding!("only"),
                                            )),
                                            select: ObjectValueDef::Power,
                                            operation: AggregateOperationDef::Maximum,
                                        }),
                                    ),
                                ])),
                            },
                            otherwise: &EffectDef::ForEachInBinding {
                                objects: crate::Binding!("first"),
                                binding: crate::Binding!("a"),
                                effect: &EffectDef::IfNoObjects(IfNoObjectsDef {
                                    input: ObjectSetDef::Binding(crate::Binding!("second")),
                                    if_empty: &EffectDef::DealDamage(DamageDef::simultaneous(&[
                                        DamageAssignmentDef::from(
                                            ObjectRefDef::Binding(crate::Binding!("a")),
                                            EffectRecipientDef::objects(ObjectSetDef::Binding(
                                                crate::Binding!("enemy"),
                                            )),
                                            ValueDef::AggregateObjectValues(
                                                &ObjectValueAggregateDef {
                                                    objects: ObjectSetDef::One(
                                                        ObjectRefDef::Binding(crate::Binding!("a")),
                                                    ),
                                                    select: ObjectValueDef::Power,
                                                    operation: AggregateOperationDef::Maximum,
                                                },
                                            ),
                                        ),
                                    ])),
                                    otherwise: &EffectDef::ForEachInBinding {
                                        objects: crate::Binding!("second"),
                                        binding: crate::Binding!("b"),
                                        effect: &EffectDef::DealDamage(DamageDef::simultaneous(&[
                                            DamageAssignmentDef::from(
                                                ObjectRefDef::Binding(crate::Binding!("a")),
                                                EffectRecipientDef::objects(ObjectSetDef::Binding(
                                                    crate::Binding!("enemy"),
                                                )),
                                                ValueDef::AggregateObjectValues(
                                                    &ObjectValueAggregateDef {
                                                        objects: ObjectSetDef::One(
                                                            ObjectRefDef::Binding(crate::Binding!(
                                                                "a"
                                                            )),
                                                        ),
                                                        select: ObjectValueDef::Power,
                                                        operation: AggregateOperationDef::Maximum,
                                                    },
                                                ),
                                            ),
                                            DamageAssignmentDef::from(
                                                ObjectRefDef::Binding(crate::Binding!("b")),
                                                EffectRecipientDef::objects(ObjectSetDef::Binding(
                                                    crate::Binding!("enemy"),
                                                )),
                                                ValueDef::AggregateObjectValues(
                                                    &ObjectValueAggregateDef {
                                                        objects: ObjectSetDef::One(
                                                            ObjectRefDef::Binding(crate::Binding!(
                                                                "b"
                                                            )),
                                                        ),
                                                        select: ObjectValueDef::Power,
                                                        operation: AggregateOperationDef::Maximum,
                                                    },
                                                ),
                                            ),
                                        ])),
                                    },
                                }),
                            },
                        }),
                    ]),
                }),
            }),
        }),
    )]),
);

// DSK 174 — Cryptid Inspector
// Audit: unsupported — Needs a committed turned-face-up event in addition to matching face-down entries; existing zone-change events cannot represent turning a permanent face up.
pub(in crate::card::sets) static CRYPTID_INSPECTOR: CardRecord = CardRecord::new(
    "Cryptid Inspector",
    "820c2932-2e23-4f67-92d0-a630aec6f1b4",
    "Kim Sokol",
    CardRules::unsupported(),
);

// DSK 175 — Defiant Survivor
// Audit: unsupported — Needs the ordinal number of the current main phase in the turn (CR 505.1b); PostcombatMain also matches third and later main phases, so it cannot restrict survival to the second main phase.
pub(in crate::card::sets) static DEFIANT_SURVIVOR: CardRecord = CardRecord::new(
    "Defiant Survivor",
    "327772f3-5a87-47af-9308-c1119ad2711d",
    "Jessica Fong",
    CardRules::unsupported(),
);

// DSK 176 — Enduring Vitality
// Audit: unsupported — Needs a return-to-battlefield instruction that establishes the permanent as a noncreature enchantment before entry replacement effects and enters triggers inspect it; applying a type change after the return produces an incorrect creature entry.
pub(in crate::card::sets) static ENDURING_VITALITY: CardRecord = CardRecord::new(
    "Enduring Vitality",
    "9d76a30c-0431-4334-892a-9822dda9671a",
    "Valera Lutfullina",
    CardRules::unsupported(),
);

// DSK 177 — Fear of Exposure
pub(in crate::card::sets) static FEAR_OF_EXPOSURE: CardRecord = CardRecord::new(
    "Fear of Exposure",
    "93bb4abc-5af1-4cf5-919b-244b6a36f8ec",
    "Josu Hernaiz",
    CardRules::new_enchantment_creature(mana_cost!("{2}{G}"), &["Nightmare"], 5, 4).with_abilities(
        &[
            AbilityDef::spell(
                "As an additional cost to cast this spell, tap two untapped \
                 creatures and/or lands you control.",
                EffectDef::None,
            )
            .with_spell_additional_cost(&CostDef::Tap {
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Land),
                ]),
                quantity: CostQuantityDef::Fixed(2),
            }),
            abilities::trample(),
        ],
    ),
);

// DSK 178 — Flesh Burrower
pub(in crate::card::sets) static FLESH_BURROWER: CardRecord = CardRecord::new(
    "Flesh Burrower",
    "60499c90-a512-4abb-98eb-0735a7138421",
    "Maxime Minard",
// It already has deathtouch, which is why the trigger says "another":
    // the point is to make a second attacker just as unblockable.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Insect"], 2, 2).with_abilities(&[
        abilities::deathtouch(),
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks, another target creature you control gains deathtouch until end of turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::deathtouch()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// DSK 179 — Frantic Strength
pub(in crate::card::sets) static FRANTIC_STRENGTH: CardRecord = CardRecord::new(
    "Frantic Strength",
    "bbd9d35d-7fd0-4193-9f7e-b8a59fae4ac5",
    "Flavio Greco Paglia",
    CardRules::new_enchantment(mana_cost!("{2}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::flash(),
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+2 and has trample.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(2),
                        ),
                        AppliedEffectDef::add_ability(&abilities::trample()),
                    ]),
                },
            ),
        ]),
);

// DSK 180 — Grasping Longneck
pub(in crate::card::sets) static GRASPING_LONGNECK: CardRecord = CardRecord::new(
    "Grasping Longneck",
    "7fddcd48-3efe-4b56-9d69-9659b3dc6021",
    "Mathias Kollros",
    CardRules::new_enchantment_creature(mana_cost!("{2}{G}"), &["Horror"], 4, 2).with_abilities(&[
        abilities::reach(),
        abilities::dies_trigger(
            "When this creature dies, you gain 2 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
    ]),
);

// DSK 181 — Greenhouse // Rickety Gazebo
pub(in crate::card::sets) static GREENHOUSE: CardRecord = CardRecord::new(
    "Greenhouse // Rickety Gazebo",
    "a22e8038-0706-47ec-bfda-f421a4912774",
    "John Di Giovanni",
    CardRules::new_enchantment(mana_cost!("{2}{G}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::static_ability(
            "Lands you control have \"{T}: Add one mana of any color.\"",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::add_ability(&AbilityDef::activated_mana(
                    "{T}: Add one mana of any color.",
                    &[CostDef::TapSource],
                    EffectDef::AddMana(AddManaEffectDef::any_color()),
                )),
            },
        )]),
)
.with_composition(|| {
    const FRONT: CardRules = CardRules::new_enchantment(mana_cost!("{2}{G}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::static_ability(
            "Lands you control have \"{T}: Add one mana of any color.\"",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::add_ability(&AbilityDef::activated_mana(
                    "{T}: Add one mana of any color.",
                    &[CostDef::TapSource],
                    EffectDef::AddMana(AddManaEffectDef::any_color()),
                )),
            },
        )]);
    const BACK: CardRules = CardRules::new_enchantment(mana_cost!("{3}{G}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::triggered(
            "When you unlock this door, mill four cards, then return up to \
             two permanent cards from among them to your hand.",
            TriggerEventDef::DoorUnlocked,
            EffectDef::Sequence(&[
                EffectDef::BindOutput {
                    binding: crate::Binding!("milled"),
                    effect: &EffectDef::Mill {
                        player: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(4),
                    },
                },
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Matching {
                        objects: &ObjectSetDef::Binding(crate::Binding!("milled")),
                        object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::HasType(CardType::Planeswalker),
                        ])),
                    },
                    exclude: None,
                    minimum: 0,
                    maximum: 2,
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                            "chosen"
                        ))),
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                }),
            ]),
        )]);
    const BOTH: CardRules = CardRules::new_enchantment(mana_cost!("{5}{G}{G}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Lands you control have \"{T}: Add one mana of any color.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Land),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::add_ability(&AbilityDef::activated_mana(
                        "{T}: Add one mana of any color.",
                        &[CostDef::TapSource],
                        EffectDef::AddMana(AddManaEffectDef::any_color()),
                    )),
                },
            ),
            AbilityDef::triggered(
                "When you unlock this door, mill four cards, then return up to \
                 two permanent cards from among them to your hand.",
                TriggerEventDef::DoorUnlocked,
                EffectDef::Sequence(&[
                    EffectDef::BindOutput {
                        binding: crate::Binding!("milled"),
                        effect: &EffectDef::Mill {
                            player: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(4),
                        },
                    },
                    EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                        unchosen: None,
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Matching {
                            objects: &ObjectSetDef::Binding(crate::Binding!("milled")),
                            object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                ObjectPredicateDef::HasType(CardType::Enchantment),
                                ObjectPredicateDef::HasType(CardType::Land),
                                ObjectPredicateDef::HasType(CardType::Planeswalker),
                            ])),
                        },
                        exclude: None,
                        minimum: 0,
                        maximum: 2,
                        visibility: ChoiceVisibilityDef::Public,
                        then: &EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                                "chosen"
                            ))),
                            ZoneKind::Hand,
                            ZonePlacement::Top,
                        ),
                    }),
                ]),
            ),
        ]);
    CardComposition::room(
        "Greenhouse // Rickety Gazebo",
        "Greenhouse",
        FRONT,
        "Rickety Gazebo",
        BACK,
        BOTH,
    )
});

// DSK 182 — Hauntwoods Shrieker
// Audit: unsupported — Needs an effect that reveals a face-down permanent's underlying card and turns it face up without paying its turn-up cost; only the paid turn-up special action is available.
pub(in crate::card::sets) static HAUNTWOODS_SHRIEKER: CardRecord = CardRecord::new(
    "Hauntwoods Shrieker",
    "744ef0bc-9973-450e-a0c4-056d8244f357",
    "John Tedrick",
    CardRules::unsupported(),
);

// DSK 183 — Hedge Shredder
// Audit: unsupported — Needs one trigger carrying the batch of land cards actually moved from the library to the controller's graveyard, retaining each card's resulting identity.
pub(in crate::card::sets) static HEDGE_SHREDDER: CardRecord = CardRecord::new(
    "Hedge Shredder",
    "39e83502-2ffd-4169-94e3-116701323ed5",
    "Cristi Balanescu",
    CardRules::unsupported(),
);

// DSK 184 — Horrid Vigor
pub(in crate::card::sets) static HORRID_VIGOR: CardRecord = CardRecord::new(
    "Horrid Vigor",
    "c175c998-7b80-4187-bdbc-5b6e0d7eac91",
    "Chris Cold",
    CardRules::new_instant(mana_cost!("{1}{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gains deathtouch and indestructible until end \
         of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::add_ability(&abilities::deathtouch()),
                AppliedEffectDef::add_ability(&abilities::indestructible()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// DSK 185 — House Cartographer
// Audit: unsupported — Needs the ordinal number of the current main phase in the turn (CR 505.1b); PostcombatMain also matches third and later main phases, so it cannot restrict survival to the second main phase.
pub(in crate::card::sets) static HOUSE_CARTOGRAPHER: CardRecord = CardRecord::new(
    "House Cartographer",
    "2a534918-a009-4f7d-87c9-5ef600b6e7c2",
    "Kai Carpenter",
    CardRules::unsupported(),
);

// DSK 186 — Insidious Fungus
pub(in crate::card::sets) static INSIDIOUS_FUNGUS: CardRecord = CardRecord::new(
    "Insidious Fungus",
    "d60d2e62-06da-410a-81ed-6cebb2632fb6",
    "Slawomir Maniak",
    CardRules::new_creature(mana_cost!("{G}"), &["Fungus"], 1, 2).with_abilities(&[
        AbilityDef::modal_activated(
            "{2}, Sacrifice this creature: Choose one —",
            &[CostDef::Mana(mana_cost!("{2}")), CostDef::SacrificeSource],
            &[
                AbilityDef::spell_with_targets(
                    "Destroy target artifact.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::HasType(CardType::Artifact),
                    )],
                    EffectDef::Destroy {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        then: None,
                    },
                ),
                AbilityDef::spell_with_targets(
                    "Destroy target enchantment.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                    )],
                    EffectDef::Destroy {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        then: None,
                    },
                ),
                AbilityDef::spell(
                    "Draw a card. Then you may put a land card from your hand onto \
                     the battlefield tapped.",
                    EffectDef::Sequence(&[
                        abilities::draw_cards(ValueDef::Constant(1)),
                        EffectDef::Choose(ChooseDef {
                            binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                            unchosen: None,
                            chooser: PlayerRefDef::EffectController,
                            candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                                ObjectPredicateDef::HasType(CardType::Land),
                                &[ZoneKind::Hand],
                                PlayerRelation::You,
                            )),
                            exclude: None,
                            minimum: 0,
                            maximum: 1,
                            visibility: ChoiceVisibilityDef::Public,
                            then: &EffectDef::WithBattlefieldArrival {
                                effect: &EffectDef::move_to_zone(
                                    EffectRecipientDef::objects(ObjectSetDef::Binding(
                                        crate::Binding!("chosen"),
                                    )),
                                    ZoneKind::Battlefield,
                                    ZonePlacement::Top,
                                ),
                                arrival: BattlefieldArrivalDef {
                                    modifications: &[BattlefieldEntryModificationDef::Tapped],
                                    ..BattlefieldArrivalDef::DEFAULT
                                },
                            },
                        }),
                    ]),
                ),
            ],
            1,
            1,
            false,
        ),
    ]),
);

// DSK 187 — Kona, Rescue Beastie
// Audit: unsupported — Needs the ordinal number of the current main phase in the turn (CR 505.1b); PostcombatMain also matches third and later main phases, so it cannot restrict survival to the second main phase.
pub(in crate::card::sets) static KONA_RESCUE_BEASTIE: CardRecord = CardRecord::new(
    "Kona, Rescue Beastie",
    "6f035294-2787-4719-8520-227bf03e84e7",
    "Brian Valeza",
    CardRules::unsupported(),
);

// DSK 188 — Leyline of Mutation
pub(in crate::card::sets) static LEYLINE_OF_MUTATION: CardRecord = CardRecord::new(
    "Leyline of Mutation",
    "2359b670-41f0-4ec7-8db9-3f87f7577bc3",
    "Sergey Glushakov",
    CardRules::new_enchantment(mana_cost!("{2}{G}{G}")).with_abilities(&[
        abilities::begin_game_on_battlefield(),
        AbilityDef::static_ability(
            "You may pay {W}{U}{B}{R}{G} rather than pay the mana cost for spells you cast.",
            EffectDef::ModifyCost(CostModificationDef::SpellAlternative {
                spell: ObjectPredicateDef::Any,
                caster: PlayerRelation::You,
                zones: &[
                    ZoneKind::Library,
                    ZoneKind::Hand,
                    ZoneKind::Graveyard,
                    ZoneKind::Exile,
                ],
                costs: &[CostDef::Mana(mana_cost!("{W}{U}{B}{R}{G}"))],
            }),
        ),
    ]),
);

// DSK 189 — Manifest Dread
pub(in crate::card::sets) static MANIFEST_DREAD: CardRecord = CardRecord::new(
    "Manifest Dread",
    "a649265b-6c32-49e7-b6cb-6086c40d26e8",
    "Andrey Kuzinskiy",
    CardRules::new_sorcery(mana_cost!("{1}{G}")).with_abilities(&[AbilityDef::spell(
        "Manifest dread. (Look at the top two cards of your library. \
         Put one onto the battlefield face down as a 2/2 creature and \
         the other into your graveyard. Turn it face up any time for \
         its mana cost if it's a creature card.)",
        abilities::manifest_dread(),
    )]),
);

// DSK 190 — Moldering Gym // Weight Room
pub(in crate::card::sets) static MOLDERING_GYM: CardRecord = CardRecord::new(
    "Moldering Gym // Weight Room",
    "245d5a61-c40c-4039-aea8-3ad61415b8f0",
    "Helge C. Balzer",
    CardRules::new_enchantment(mana_cost!("{2}{G}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::triggered(
            "When you unlock this door, search your library for a basic \
             land card, put it onto the battlefield tapped, then shuffle.",
            TriggerEventDef::DoorUnlocked,
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::Supertype(CardSupertype::Basic),
                ]),
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: true,
                destination: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: true,
                attachment: None,
                binding: None,
                then: None,
            },
        )]),
)
.with_composition(|| {
    const FRONT: CardRules = CardRules::new_enchantment(mana_cost!("{2}{G}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::triggered(
            "When you unlock this door, search your library for a basic \
             land card, put it onto the battlefield tapped, then shuffle.",
            TriggerEventDef::DoorUnlocked,
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::Supertype(CardSupertype::Basic),
                ]),
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: true,
                destination: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: true,
                attachment: None,
                binding: None,
                then: None,
            },
        )]);
    const BACK: CardRules = CardRules::new_enchantment(mana_cost!("{5}{G}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::triggered(
            "When you unlock this door, manifest dread, then put three \
             +1/+1 counters on that creature.",
            TriggerEventDef::DoorUnlocked,
            abilities::bind_top_cards_then(
                PlayerRefDef::EffectController,
                ValueDef::Constant(2),
                &EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(crate::Binding!("dread_permanent")),
                    unchosen: Some(crate::Binding!("dread_graveyard")),
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Binding(crate::ParentBinding),
                    exclude: None,
                    minimum: 1,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Private,
                    then: &EffectDef::PutObjectsOntoBattlefieldFaceDown(
                        PutObjectsOntoBattlefieldFaceDownDef {
                            input: ObjectSetDef::Binding(crate::Binding!("dread_permanent")),
                            controller: PlayerRefDef::EffectController,
                            characteristics: crate::card::face_down::manifest(),
                            turn_up_for_mana_cost: true,
                            moved: Some(crate::Binding!("manifested")),
                            then: &EffectDef::Sequence(&[
                                EffectDef::MoveObjects(MoveObjectsDef {
                                    input: ObjectSetDef::Binding(crate::Binding!(
                                        "dread_graveyard"
                                    )),
                                    from: Some(ZoneKind::Library),
                                    zone: ZoneKind::Graveyard,
                                    placement: ZonePlacement::Top,
                                    moved: None,
                                    then: &EffectDef::None,
                                }),
                                EffectDef::AddCounters {
                                    object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                        crate::Binding!("manifested"),
                                    )),
                                    kind: CounterKind::PlusOnePlusOne,
                                    amount: ValueDef::Constant(3),
                                },
                            ]),
                        },
                    ),
                }),
            ),
        )]);
    const BOTH: CardRules = CardRules::new_enchantment(mana_cost!("{7}{G}{G}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[
            AbilityDef::triggered(
                "When you unlock this door, search your library for a basic \
                 land card, put it onto the battlefield tapped, then shuffle.",
                TriggerEventDef::DoorUnlocked,
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Supertype(CardSupertype::Basic),
                    ]),
                    minimum: 0,
                    maximum: ValueDef::Constant(1),
                    reveal: true,
                    destination: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: true,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            ),
            AbilityDef::triggered(
                "When you unlock this door, manifest dread, then put three \
                 +1/+1 counters on that creature.",
                TriggerEventDef::DoorUnlocked,
                abilities::bind_top_cards_then(
                    PlayerRefDef::EffectController,
                    ValueDef::Constant(2),
                    &EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(crate::Binding!(
                            "dread_permanent"
                        )),
                        unchosen: Some(crate::Binding!("dread_graveyard")),
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Binding(crate::ParentBinding),
                        exclude: None,
                        minimum: 1,
                        maximum: 1,
                        visibility: ChoiceVisibilityDef::Private,
                        then: &EffectDef::PutObjectsOntoBattlefieldFaceDown(
                            PutObjectsOntoBattlefieldFaceDownDef {
                                input: ObjectSetDef::Binding(crate::Binding!("dread_permanent")),
                                controller: PlayerRefDef::EffectController,
                                characteristics: crate::card::face_down::manifest(),
                                turn_up_for_mana_cost: true,
                                moved: Some(crate::Binding!("manifested")),
                                then: &EffectDef::Sequence(&[
                                    EffectDef::MoveObjects(MoveObjectsDef {
                                        input: ObjectSetDef::Binding(crate::Binding!(
                                            "dread_graveyard"
                                        )),
                                        from: Some(ZoneKind::Library),
                                        zone: ZoneKind::Graveyard,
                                        placement: ZonePlacement::Top,
                                        moved: None,
                                        then: &EffectDef::None,
                                    }),
                                    EffectDef::AddCounters {
                                        object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                            crate::Binding!("manifested"),
                                        )),
                                        kind: CounterKind::PlusOnePlusOne,
                                        amount: ValueDef::Constant(3),
                                    },
                                ]),
                            },
                        ),
                    }),
                ),
            ),
        ]);
    CardComposition::room(
        "Moldering Gym // Weight Room",
        "Moldering Gym",
        FRONT,
        "Weight Room",
        BACK,
        BOTH,
    )
});

// DSK 191 — Monstrous Emergence
// Audit: unsupported — Needs a casting-time additional-cost choice between a battlefield creature reference and revealing a creature card, retaining the selected power for resolution.
pub(in crate::card::sets) static MONSTROUS_EMERGENCE: CardRecord = CardRecord::new(
    "Monstrous Emergence",
    "b999eb47-b842-47f1-be91-c79fc46e1896",
    "Loïc Canavaggia",
    CardRules::unsupported(),
);

// DSK 192 — Omnivorous Flytrap
pub(in crate::card::sets) static OMNIVOROUS_FLYTRAP: CardRecord = CardRecord::new(
    "Omnivorous Flytrap",
    "4370a197-68cc-46dd-bf0a-e9dab4ff6638",
    "Antonio José Manzanedo",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Plant"], 2, 4).with_abilities(&[
        AbilityDef::triggered_if_with_targets(
            "Delirium — Whenever this creature enters or attacks, if there \
             are four or more card types among cards in your graveyard, \
             distribute two +1/+1 counters among one or two target \
             creatures. Then if there are six or more card types among \
             cards in your graveyard, double the number of +1/+1 counters \
             on those creatures.",
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
            ]),
            &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                left: ValueDef::CardTypesAmongGraveyards(PlayerRelation::You),
                comparison: ComparisonDef::GreaterOrEqual,
                right: ValueDef::Constant(4),
            }),
            &[AbilityTargetDef {
                minimum: 1,
                maximum: 2,
                divided_total: Some(DividedTotal::Fixed(2)),
                ..AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                    CardType::Creature,
                ))
            }],
            EffectDef::Sequence(&[
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::DividedAmongTargets,
                },
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                        left: ValueDef::CardTypesAmongGraveyards(PlayerRelation::You),
                        comparison: ComparisonDef::GreaterOrEqual,
                        right: ValueDef::Constant(6),
                    }),
                    then: &EffectDef::BindObjects(BindObjectsDef {
                        source: ObjectCollectionSourceDef::ObjectSet(ObjectSetDef::LegalTargets(
                            TargetIndex::PRIMARY,
                        )),
                        binding: crate::Binding!("counter_targets"),
                        then: &EffectDef::ForEachInBinding {
                            objects: crate::Binding!("counter_targets"),
                            binding: crate::Binding!("counter_target"),
                            effect: &EffectDef::AddCounters {
                                object: EffectRecipientDef::object(ObjectRefDef::Binding(
                                    crate::Binding!("counter_target"),
                                )),
                                kind: CounterKind::PlusOnePlusOne,
                                amount: ValueDef::CountersOnObject(&ObjectCounterValueDef::new(
                                    ObjectRefDef::Binding(crate::Binding!("counter_target")),
                                    CounterKind::PlusOnePlusOne,
                                )),
                            },
                        },
                    }),
                },
            ]),
        ),
    ]),
);

// DSK 193 — Overgrown Zealot
// Audit: unsupported — Needs produced mana restricted specifically to turn-face-up special-action costs; the current restriction vocabulary has no such spending context.
pub(in crate::card::sets) static OVERGROWN_ZEALOT: CardRecord = CardRecord::new(
    "Overgrown Zealot",
    "96d68d29-499a-4864-be18-bd98fda0d173",
    "Tyler Walpole",
    CardRules::unsupported(),
);

// DSK 194 — Overlord of the Hauntwoods
pub(in crate::card::sets) static OVERLORD_OF_THE_HAUNTWOODS: CardRecord = CardRecord::new(
    "Overlord of the Hauntwoods",
    "05d08ff1-edcc-4c76-96e0-683b3da36ebb",
    "Tiffany Turrill",
    CardRules::new_enchantment_creature(mana_cost!("{3}{G}{G}"), &["Avatar", "Horror"], 6, 5)
        .with_abilities(&[
            impending_cast(
                &[CostDef::Mana(mana_cost!("{1}{G}{G}"))],
                "Impending 4—{1}{G}{G} (If you cast this spell for its \
                 impending cost, it enters with four time counters and isn't a \
                 creature until the last is removed. At the beginning of your \
                 end step, remove a time counter from it.)",
            ),
            impending_entry(4),
            impending_countdown(),
            AbilityDef::triggered(
                "Whenever this permanent enters or attacks, create a tapped \
                 colorless land token named Everywhere that is every basic \
                 land type.",
                TriggerEventDef::AnyOf(&[
                    TriggerEventDef::zone_changed(
                        ObjectPredicateDef::Source,
                        None,
                        Some(ZoneKind::Battlefield),
                    ),
                    TriggerEventDef::attacks(ObjectPredicateDef::Source),
                ]),
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(
                        TokenCharacteristics::new(
                            CardTypeSet::single(CardType::Land),
                            &["Plains", "Island", "Swamp", "Mountain", "Forest"],
                            &[],
                            None,
                        )
                        .with_name("Everywhere"),
                    ))
                    .entering_tapped(),
                ),
            ),
        ]),
);

// DSK 195 — Patchwork Beastie
pub(in crate::card::sets) static PATCHWORK_BEASTIE: CardRecord = CardRecord::new(
    "Patchwork Beastie",
    "895de583-36d2-43fd-927f-8876d4302c73",
    "John Tedrick",
    CardRules::new_artifact_creature(mana_cost!("{G}"), &["Beast"], 3, 3).with_abilities(&[
        AbilityDef::static_ability(
            "Delirium — This creature can't attack or block unless there \
             are four or more card types among cards in your graveyard.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::Not(&TriggerConditionDef::ValueComparison(
                    &ValueComparisonDef {
                        left: ValueDef::CardTypesAmongGraveyards(PlayerRelation::You),
                        comparison: ComparisonDef::GreaterOrEqual,
                        right: ValueDef::Constant(4),
                    },
                )),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_ATTACK),
                        AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                    ]),
                },
            },
        ),
        AbilityDef::triggered(
            "At the beginning of your upkeep, you may mill a card. (You \
             may put the top card of your library into your graveyard.)",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Mill {
                    player: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            },
        ),
    ]),
);

// DSK 196 — Rootwise Survivor
// Audit: unsupported — Needs the ordinal number of the current main phase in the turn (CR 505.1b); PostcombatMain also matches third and later main phases, so it cannot restrict survival to the second main phase.
pub(in crate::card::sets) static ROOTWISE_SURVIVOR: CardRecord = CardRecord::new(
    "Rootwise Survivor",
    "d843c242-088f-4131-9e52-7ee2d0db5e20",
    "Joseph Weston",
    CardRules::unsupported(),
);

// DSK 197 — Say Its Name
// Audit: unsupported — Needs optional selection of which of several hidden/public zones to search, with a shuffle only if the library was searched; existing multi-zone card choices do not retain that search-zone decision independently of the card found.
pub(in crate::card::sets) static SAY_ITS_NAME: CardRecord = CardRecord::new(
    "Say Its Name",
    "94c58683-b5f2-4863-9562-6f6be1ec21fe",
    "Sam Wolfe Connelly",
    CardRules::unsupported(),
);

// DSK 198 — Slavering Branchsnapper
pub(in crate::card::sets) static SLAVERING_BRANCHSNAPPER: CardRecord = CardRecord::new(
    "Slavering Branchsnapper",
    "5ed7ca4d-5895-4074-8315-656363d14862",
    "John Tedrick",
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Lizard"], 7, 6).with_abilities(&[
        abilities::trample(),
        abilities::typecycling!(
            "Forestcycling {2}",
            &[CostDef::Mana(mana_cost!("{2}"))],
            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Forest"))
        ),
    ]),
);

// DSK 199 — Spineseeker Centipede
pub(in crate::card::sets) static SPINESEEKER_CENTIPEDE: CardRecord = CardRecord::new(
    "Spineseeker Centipede",
    "b50d697c-8358-429b-8f79-7ad9d01a5edd",
    "Dave Kendall",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Insect"], 2, 1).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, search your library for a basic \
             land card, reveal it, put it into your hand, then shuffle.",
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::Supertype(CardSupertype::Basic),
                ]),
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: true,
                destination: ZoneKind::Hand,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: false,
                attachment: None,
                binding: None,
                then: None,
            },
        ),
        AbilityDef::static_ability(
            "Delirium — This creature gets +1/+2 and has vigilance as long \
             as there are four or more card types among cards in your \
             graveyard.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::CardTypesAmongGraveyards(PlayerRelation::You),
                    comparison: ComparisonDef::GreaterOrEqual,
                    right: ValueDef::Constant(4),
                }),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(2),
                        ),
                        AppliedEffectDef::add_ability(&abilities::vigilance()),
                    ]),
                },
            },
        ),
    ]),
);

// DSK 200 — Threats Around Every Corner
// Audit: unsupported — Needs an object predicate that identifies face-down permanents for the enters trigger; manifest can create those permanents, but the exposed event predicate vocabulary cannot test that characteristic.
pub(in crate::card::sets) static THREATS_AROUND_EVERY_CORNER: CardRecord = CardRecord::new(
    "Threats Around Every Corner",
    "7201ee12-9104-48d8-aeec-08a318c8ee10",
    "Andrea Piparo",
    CardRules::unsupported(),
);

// DSK 201 — Twitching Doll
// Audit: unsupported — Needs a mana ability that both produces mana and places a counter as part of its immediate resolution; the current mana-ability boundary admits mana production without this additional effect.
pub(in crate::card::sets) static TWITCHING_DOLL: CardRecord = CardRecord::new(
    "Twitching Doll",
    "416c025b-e40e-4d95-a774-ba3961f43808",
    "Warren Mahy",
    CardRules::unsupported(),
);

// DSK 202 — Tyvar, the Pummeler
pub(in crate::card::sets) static TYVAR_THE_PUMMELER: CardRecord = CardRecord::new(
    "Tyvar, the Pummeler",
    "5f7687b1-630a-4e95-89c7-eaf456d8cb68",
    "Olivier Bernard",
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Elf", "Warrior"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated(
                "Tap another untapped creature you control: Tyvar gains \
                 indestructible until end of turn. Tap it.",
                &[CostDef::TapPermanents {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    controller: PlayerRelation::You,
                    count: 1,
                }],
                EffectDef::Sequence(&[
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                    EffectDef::Tap {
                        object: EffectRecipientDef::Source,
                    },
                ]),
            ),
            AbilityDef::activated(
                "{3}{G}{G}: Creatures you control get +X/+X until end of turn, \
                 where X is the greatest power among creatures you control.",
                &[CostDef::Mana(mana_cost!("{3}{G}{G}"))],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::AggregateObjectValues(&ObjectValueAggregateDef {
                            objects: ObjectSetDef::Query(ObjectQueryDef::matching(
                                ObjectPredicateDef::HasType(CardType::Creature),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            )),
                            select: ObjectValueDef::Power,
                            operation: AggregateOperationDef::Maximum,
                        }),
                        ValueDef::AggregateObjectValues(&ObjectValueAggregateDef {
                            objects: ObjectSetDef::Query(ObjectQueryDef::matching(
                                ObjectPredicateDef::HasType(CardType::Creature),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            )),
                            select: ObjectValueDef::Power,
                            operation: AggregateOperationDef::Maximum,
                        }),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// DSK 203 — Under the Skin
pub(in crate::card::sets) static UNDER_THE_SKIN: CardRecord = CardRecord::new(
    "Under the Skin",
    "0aaf1ad3-00ad-48ec-a71f-812649d55e14",
    "Fernando Falcone",
    CardRules::new_sorcery(mana_cost!("{2}{G}")).with_abilities(&[AbilityDef::spell(
        "Manifest dread. (Look at the top two cards of your library. \
         Put one onto the battlefield face down as a 2/2 creature and \
         the other into your graveyard. Turn it face up any time for \
         its mana cost if it's a creature card.)\nYou may return a \
         permanent card from your graveyard to your hand.",
        EffectDef::Sequence(&[
            abilities::manifest_dread(),
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::HasType(CardType::Planeswalker),
                    ]),
                    &[ZoneKind::Graveyard],
                    PlayerRelation::You,
                )),
                exclude: None,
                minimum: 0,
                maximum: 1,
                visibility: ChoiceVisibilityDef::Public,
                then: &EffectDef::move_to_zone(
                    EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!("chosen"))),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            }),
        ]),
    )]),
);

// DSK 204 — Valgavoth's Onslaught
// Audit: unsupported — Needs a repeated manifest operation that accumulates all resulting permanent identities for a later group counter placement; the existing repeat continuation does not accumulate per-iteration result bindings.
pub(in crate::card::sets) static VALGAVOTH_S_ONSLAUGHT: CardRecord = CardRecord::new(
    "Valgavoth's Onslaught",
    "7d4ba274-3c6f-4e12-ba2c-a81c3da3f7e2",
    "Lie Setiawan",
    CardRules::unsupported(),
);

// DSK 205 — Walk-In Closet // Forgotten Cellar
// Audit: unsupported — Needs a temporary player-scoped graveyard-move replacement that persists after this Room leaves the battlefield; granting the replacement to the Room makes it end too early.
pub(in crate::card::sets) static WALK_IN_CLOSET_FORGOTTEN_CELLAR: CardRecord = CardRecord::new(
    "Walk-In Closet // Forgotten Cellar",
    "0adcd4e5-d542-4293-8774-ace2305ef820",
    "Miklós Ligeti",
    CardRules::unsupported(),
);

// DSK 206 — Wary Watchdog
pub(in crate::card::sets) static WARY_WATCHDOG: CardRecord = CardRecord::new(
    "Wary Watchdog",
    "25ed97f2-b47e-49b7-9b1a-694c4bbeca3b",
    "Olivier Bernard",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Dog"], 3, 1).with_abilities(&[
        AbilityDef::triggered(
            "When this creature enters or dies, surveil 1. (Look at the \
             top card of your library. You may put it into your \
             graveyard.)",
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
            ]),
            abilities::surveil(ValueDef::Constant(1)),
        ),
    ]),
);

// DSK 207 — Wickerfolk Thresher
pub(in crate::card::sets) static WICKERFOLK_THRESHER: CardRecord = CardRecord::new(
    "Wickerfolk Thresher",
    "b3a74892-20cd-47f7-b514-a4c7f14cca8b",
    "WolfSkullJack",
    CardRules::new_artifact_creature(mana_cost!("{3}{G}"), &["Scarecrow"], 5, 4).with_abilities(&[
        AbilityDef::triggered_if(
            "Delirium — Whenever this creature attacks, if there are four \
             or more card types among cards in your graveyard, look at the \
             top card of your library. If it's a land card, you may put it \
             onto the battlefield. If you don't put the card onto the \
             battlefield, put it into your hand.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                left: ValueDef::CardTypesAmongGraveyards(PlayerRelation::You),
                comparison: ComparisonDef::GreaterOrEqual,
                right: ValueDef::Constant(4),
            }),
            EffectDef::Sequence(&[
                abilities::look_at_top_cards(PlayerRefDef::EffectController, ValueDef::Constant(1)),
                EffectDef::BindObjects(BindObjectsDef {
                    source: ObjectCollectionSourceDef::TopCards {
                        player: PlayerRefDef::EffectController,
                        count: ValueDef::Constant(1),
                    },
                    binding: crate::Binding!("top"),
                    then: &EffectDef::Sequence(&[
                        EffectDef::Choose(ChooseDef {
                            binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                            unchosen: None,
                            chooser: PlayerRefDef::EffectController,
                            candidates: ObjectSetDef::Matching {
                                objects: &ObjectSetDef::Binding(crate::Binding!("top")),
                                object: ObjectSetFilterDef::Predicate(
                                    &ObjectPredicateDef::HasType(CardType::Land),
                                ),
                            },
                            exclude: None,
                            minimum: 0,
                            maximum: 1,
                            visibility: ChoiceVisibilityDef::Public,
                            then: &EffectDef::move_to_zone(
                                EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("chosen"),
                                )),
                                ZoneKind::Battlefield,
                                ZonePlacement::Top,
                            ),
                        }),
                        EffectDef::MoveObjects(MoveObjectsDef {
                            input: ObjectSetDef::Binding(crate::Binding!("top")),
                            from: Some(ZoneKind::Library),
                            zone: ZoneKind::Hand,
                            placement: ZonePlacement::Top,
                            moved: None,
                            then: &EffectDef::None,
                        }),
                    ]),
                }),
            ]),
        ),
    ]),
);

// DSK 208 — Arabella, Abandoned Doll
pub(in crate::card::sets) static ARABELLA_ABANDONED_DOLL: CardRecord = CardRecord::new(
    "Arabella, Abandoned Doll",
    "f683d5a1-b8bf-446f-9fe3-88a4398bf3cf",
    "J.P. Targete",
    CardRules::new_artifact_creature(mana_cost!("{R}{W}"), &["Toy"], 1, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[AbilityDef::triggered(
            "Whenever Arabella attacks, it deals X damage to each opponent \
             and you gain X life, where X is the number of creatures you \
             control with power 2 or less.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::Sequence(&[
                EffectDef::damage(
                    EffectRecipientDef::Opponent,
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::PowerLessThan(ValueDef::Constant(3)),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                ),
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::PowerLessThan(ValueDef::Constant(3)),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                },
            ]),
        )]),
);

// DSK 209 — Baseball Bat
pub(in crate::card::sets) static BASEBALL_BAT: CardRecord = CardRecord::new(
    "Baseball Bat",
    "438accb1-6d2c-4710-adeb-df4301a7b8f1",
    "John Stanko",
    CardRules::new_artifact(mana_cost!("{G}{W}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::enters_trigger_with_targets(
                "When this Equipment enters, attach it to target creature you \
                 control.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Equipped creature gets +1/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                },
            ),
            AbilityDef::triggered_with_targets(
                "Whenever equipped creature attacks, tap up to one target \
                 creature.",
                TriggerEventDef::attacks(ObjectPredicateDef::AttachedToSource),
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    1,
                )],
                EffectDef::Tap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{3}"))], "Equip {3}"),
        ]),
);

// DSK 210 — Beastie Beatdown
pub(in crate::card::sets) static BEASTIE_BEATDOWN: CardRecord = CardRecord::new(
    "Beastie Beatdown",
    "5f889c95-46af-4fd9-aff2-573d5384fd58",
    "Inkognit",
    CardRules::new_sorcery(mana_cost!("{R}{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Choose target creature you control and target creature an \
         opponent controls.\nDelirium — If there are four or more card \
         types among cards in your graveyard, put two +1/+1 counters \
         on the creature you control.\nThe creature you control deals \
         damage equal to its power to the creature an opponent \
         controls.",
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
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::CardTypesAmongGraveyards(PlayerRelation::You),
                    comparison: ComparisonDef::GreaterOrEqual,
                    right: ValueDef::Constant(4),
                }),
                then: &EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(2),
                },
            },
            EffectDef::damage_from(
                ObjectRefDef::Target(TargetIndex::PRIMARY),
                EffectRecipientDef::Target(TargetIndex(1)),
                ValueDef::TargetPower(TargetIndex::PRIMARY),
            ),
        ]),
    )]),
);

// DSK 211 — Broodspinner
pub(in crate::card::sets) static BROODSPINNER: CardRecord = CardRecord::new(
    "Broodspinner",
    "dcdd2622-ab7a-4990-b026-3667cac42894",
    "Igor Krstic",
    CardRules::new_creature(mana_cost!("{B}{G}"), &["Spider"], 2, 3).with_abilities(&[
        abilities::reach(),
        abilities::enters_trigger(
            "When this creature enters, surveil 2. (Look at the top two \
             cards of your library, then put any number of them into your \
             graveyard and the rest on top of your library in any order.)",
            abilities::surveil(ValueDef::Constant(2)),
        ),
        AbilityDef::activated(
            "{4}{B}{G}, {T}, Sacrifice this creature: Create a number of \
             1/1 black and green Insect creature tokens with flying equal \
             to the number of card types among cards in your graveyard.",
            &[
                CostDef::Mana(mana_cost!("{4}{B}{G}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(INSECT_TOKEN))
                    .with_count(ValueDef::CardTypesAmongGraveyards(PlayerRelation::You)),
            ),
        ),
    ]),
);

// DSK 212 — Disturbing Mirth
pub(in crate::card::sets) static DISTURBING_MIRTH: CardRecord = CardRecord::new(
    "Disturbing Mirth",
    "f79b0c5d-6823-439c-a011-8b9bf424bdad",
    "Nino Vecia",
    CardRules::new_enchantment(mana_cost!("{B}{R}")).with_abilities(&[
        abilities::enters_trigger(
            "When this enchantment enters, you may sacrifice another \
             enchantment or creature. If you do, draw two cards.",
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                    ]),
                ]))],
                &abilities::draw_cards(ValueDef::Constant(2)),
            )),
        ),
        AbilityDef::triggered(
            "When you sacrifice this enchantment, manifest dread. (Look at \
             the top two cards of your library. Put one onto the \
             battlefield face down as a 2/2 creature and the other into \
             your graveyard. Turn it face up any time for its mana cost if \
             it's a creature card.)",
            TriggerEventDef::Sacrificed {
                object: ObjectPredicateDef::Source,
                player: PlayerRelation::You,
            },
            abilities::manifest_dread(),
        ),
    ]),
);

// DSK 213 — Drag to the Roots
// Audit: unsupported — Needs a self spell-cost reduction conditional on the number of card types in the controller's graveyard; the self-cost evaluator rejects IfCardTypesAmongGraveyards even though normal resolving effects can read it.
pub(in crate::card::sets) static DRAG_TO_THE_ROOTS: CardRecord = CardRecord::new(
    "Drag to the Roots",
    "46f46095-6479-46b0-9e59-194d83f86a46",
    "Deruchenko Alexander",
    CardRules::unsupported(),
);

// DSK 214 — Fear of Infinity
// Audit: unsupported — Needs a committed event for a Room becoming fully unlocked, observed by other permanents and graveyard abilities. DoorUnlocked only dispatches to the door being opened, so it cannot implement the second eerie trigger.
pub(in crate::card::sets) static FEAR_OF_INFINITY: CardRecord = CardRecord::new(
    "Fear of Infinity",
    "81756844-c642-406f-842d-35c1e404fec0",
    "Fernando Falcone",
    CardRules::unsupported(),
);

// DSK 215 — Gremlin Tamer
// Audit: unsupported — Needs a committed event for a Room becoming fully unlocked, observed by other permanents and graveyard abilities. DoorUnlocked only dispatches to the door being opened, so it cannot implement the second eerie trigger.
pub(in crate::card::sets) static GREMLIN_TAMER: CardRecord = CardRecord::new(
    "Gremlin Tamer",
    "3593a222-21c5-4f91-bde1-763ea08071da",
    "Billy Christian",
    CardRules::unsupported(),
);

// DSK 216 — Growing Dread
// Audit: unsupported — Needs a committed turned-face-up event that carries the affected permanent so its counter can be placed.
pub(in crate::card::sets) static GROWING_DREAD: CardRecord = CardRecord::new(
    "Growing Dread",
    "5479ac50-8335-4c6a-be99-750a44e24f25",
    "Maxime Minard",
    CardRules::unsupported(),
);

// DSK 217 — Inquisitive Glimmer
// Audit: unsupported — Needs a cost modifier for Room door-unlock special actions; current modifiers cover spell and activated-ability costs.
pub(in crate::card::sets) static INQUISITIVE_GLIMMER: CardRecord = CardRecord::new(
    "Inquisitive Glimmer",
    "f1f66e3e-9f1f-4601-aa30-30b66805a5a8",
    "Julie Dillon",
    CardRules::unsupported(),
);

// DSK 218 — Intruding Soulrager
pub(in crate::card::sets) static INTRUDING_SOULRAGER: CardRecord = CardRecord::new(
    "Intruding Soulrager",
    "adffc298-0219-4807-ab05-d94460e767bc",
    "Jeremy Wilson",
    CardRules::new_creature(mana_cost!("{U}{R}"), &["Spirit"], 2, 2).with_abilities(&[
        abilities::vigilance(),
        AbilityDef::activated(
            "{T}, Sacrifice a Room: This creature deals 2 damage to each \
             opponent. Draw a card.",
            &[
                CostDef::TapSource,
                CostDef::sacrifice_permanent(ObjectPredicateDef::Subtype(SubtypeDef::Literal(
                    "Room",
                ))),
            ],
            EffectDef::Sequence(&[
                EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::Constant(2)),
                abilities::draw_cards(ValueDef::Constant(1)),
            ]),
        ),
    ]),
);

// DSK 219 — The Jolly Balloon Man
// Audit: unsupported — Needs an additive copiable color exception: the Balloon token is red in addition to its original colors. CopyExceptionsDef can replace colors, but applying a later color effect would not make that addition copiable.
pub(in crate::card::sets) static THE_JOLLY_BALLOON_MAN: CardRecord = CardRecord::new(
    "The Jolly Balloon Man",
    "a3c4e2e0-1c0e-475d-a0f4-1be4216c2bad",
    "Campbell White",
    CardRules::unsupported(),
);

// DSK 220 — Kaito, Bane of Nightmares
pub(in crate::card::sets) static KAITO_BANE_OF_NIGHTMARES: CardRecord = CardRecord::new(
    "Kaito, Bane of Nightmares",
    "55a14f30-4ff9-4472-90a6-c3139f1c18e5",
    "Joshua Raphael",
// Four mana, or a ninjutsu out of a connected attacker: he arrives
    // attacking, is a hexproof 3/4 for as long as it is your turn, and is a
    // planeswalker again the moment it is not.
    CardRules::new_planeswalker(mana_cost!("{2}{U}{B}"), &["Kaito"], 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::ninjutsu!(
                "Ninjutsu {1}{U}{B} ({1}{U}{B}, Return an unblocked attacker you control to hand: Put \
                 this card onto the battlefield from your hand tapped and attacking.)",
                &[CostDef::Mana(mana_cost!("{1}{U}{B}"))],
            ),
            AbilityDef::static_ability(
                "During your turn, as long as Kaito has one or more loyalty counters on him, he's a 3/4 \
                 Ninja creature and has hexproof.",
                EffectDef::IfCondition {
                    // He is a creature only while it is your turn and only while he still has
                    // loyalty: the pair of conditions is what keeps him from being a creature
                    // anyone can answer on their own turn.
                    condition: &TriggerConditionDef::All(&[
                        TriggerConditionDef::ActivePlayer(PlayerRelation::You),
                        TriggerConditionDef::SourceCounters {
                            kind: CounterKind::Loyalty,
                            comparison: ComparisonDef::GreaterOrEqual,
                            amount: 1,
                        },
                    ]),
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::Characteristic(CharacteristicOperationDef::CardTypes(SetOperationDef::Add(
                                CardTypeSet::single(CardType::Creature),
                            ))),
                            AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&["Ninja"])),
                            AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(3), ValueDef::Constant(4)),
                            AppliedEffectDef::add_ability(&abilities::hexproof()),
                        ]),
                    },
                },
            ),
            AbilityDef::activated(
                "+1: You get an emblem with \"Ninjas you control get +1/+1.\"",
                &[CostDef::Loyalty(1)],
                EffectDef::CreateEmblem {
                    emblem: EmblemCharacteristics::new("Kaito, Bane of Nightmares emblem", &[AbilityDef::static_ability(
                            "Ninjas you control get +1/+1.",
                            EffectDef::StaticApply {
                                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Ninja")),
                                    &[ZoneKind::Battlefield],
                                    PlayerRelation::You,
                                ))),
                                effect: AppliedEffectDef::modify_power_toughness(
                                    ValueDef::Constant(1),
                                    ValueDef::Constant(1),
                                ),
                            },
                        )]),
                },
            ),
            AbilityDef::activated(
                "0: Surveil 2. Then draw a card for each opponent who lost life this turn.",
                &[CostDef::Loyalty(0)],
                EffectDef::Sequence(&[
                    abilities::surveil(ValueDef::Constant(2)),
                    // "A card for each opponent who lost life this turn" is a count of players
                    // rather than of life, which in a two-player game is one card or none.
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::OpponentsWhoLostLifeThisTurn,
                    },
                ]),
            ),
            AbilityDef::activated_with_targets(
                "\u{2212}2: Tap target creature. Put two stun counters on it.",
                &[CostDef::Loyalty(-2)],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Sequence(&[
                    EffectDef::Tap {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        kind: CounterKind::Stun,
                        amount: ValueDef::Constant(2),
                    },
                ]),
            ),
        ]),
);

// DSK 221 — Marina Vendrell
// Audit: unsupported — Needs resolving lock/unlock effects that choose a door of a targeted Room, including removing that door's abilities when it is locked.
pub(in crate::card::sets) static MARINA_VENDRELL: CardRecord = CardRecord::new(
    "Marina Vendrell",
    "6428cddc-2fb6-41af-a643-e83c81dc04f5",
    "Magali Villeneuve",
    CardRules::unsupported(),
);

// DSK 222 — Midnight Mayhem
pub(in crate::card::sets) static MIDNIGHT_MAYHEM: CardRecord = CardRecord::new(
    "Midnight Mayhem",
    "4fc0b94f-08b3-4fb6-9b09-2f3a215203ca",
    "Olivier Bernard",
    CardRules::new_sorcery(mana_cost!("{2}{R}{W}")).with_abilities(&[AbilityDef::spell(
        "Create three 1/1 red Gremlin creature tokens. Gremlins you \
         control gain menace, lifelink, and haste until end of turn. \
         (A creature with menace can't be blocked except by two or \
         more creatures.)",
        EffectDef::Sequence(&[
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(GREMLIN_TOKEN))
                    .with_count(ValueDef::Constant(3)),
            ),
            EffectDef::Apply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Gremlin")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_ability(&abilities::menace()),
                    AppliedEffectDef::add_ability(&abilities::lifelink()),
                    AppliedEffectDef::add_ability(&abilities::haste()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )]),
);

// DSK 223 — Nashi, Searcher in the Dark
pub(in crate::card::sets) static NASHI_SEARCHER_IN_THE_DARK: CardRecord = CardRecord::new(
    "Nashi, Searcher in the Dark",
    "0fbf9e1e-43f2-499e-844d-22fc10dbad06",
    "Johan Grenier",
    CardRules::new_creature(mana_cost!("{U}{B}"), &["Rat", "Ninja", "Wizard"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::menace(),
            AbilityDef::triggered(
                "Whenever Nashi deals combat damage to a player, you mill that \
                 many cards. You may put any number of legendary and/or \
                 enchantment cards from among them into your hand. If you put \
                 no cards into your hand this way, put a +1/+1 counter on \
                 Nashi.",
                TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
                EffectDef::Sequence(&[
                    EffectDef::BindOutput {
                        binding: crate::Binding!("milled"),
                        effect: &EffectDef::Mill {
                            player: EffectRecipientDef::Controller,
                            amount: ValueDef::TriggerEventAmount,
                        },
                    },
                    EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                        unchosen: None,
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Matching {
                            objects: &ObjectSetDef::Binding(crate::Binding!("milled")),
                            object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                                ObjectPredicateDef::HasType(CardType::Enchantment),
                            ])),
                        },
                        exclude: None,
                        minimum: 0,
                        maximum: usize::MAX,
                        visibility: ChoiceVisibilityDef::Public,
                        then: &EffectDef::WithZoneMoveResult {
                            effect: &EffectDef::move_to_zone(
                                EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("chosen"),
                                )),
                                ZoneKind::Hand,
                                ZonePlacement::Top,
                            ),
                            binding: crate::Binding!("returned"),
                            then: &EffectDef::IfCondition {
                                condition: &TriggerConditionDef::ObjectSetCount(
                                    &ObjectSetCountConditionDef {
                                        objects: &ObjectSetDef::ZoneChangeSuccessorsOfBinding(
                                            crate::Binding!("returned"),
                                        ),
                                        predicate: ObjectSetPredicateDef {
                                            filter: None,
                                            comparison: ComparisonDef::Equal,
                                            amount: 0,
                                        },
                                    },
                                ),
                                then: &EffectDef::AddCounters {
                                    object: EffectRecipientDef::Source,
                                    kind: CounterKind::PlusOnePlusOne,
                                    amount: ValueDef::Constant(1),
                                },
                            },
                        },
                    }),
                ]),
            ),
        ]),
);

// DSK 224 — Niko, Light of Hope
// Audit: unsupported — Needs a copy effect with an expiry at the beginning of the next end step, distinct from end-of-turn cleanup, while preserving the source card's exiled identity.
pub(in crate::card::sets) static NIKO_LIGHT_OF_HOPE: CardRecord = CardRecord::new(
    "Niko, Light of Hope",
    "91ad013f-de8d-4980-b4b6-c7f91ff495b1",
    "Aurore Folny",
    CardRules::unsupported(),
);

// DSK 225 — Oblivious Bookworm
// Audit: unsupported — Needs turn history for face-down entries and face-up turns, including objects that have since left the battlefield.
pub(in crate::card::sets) static OBLIVIOUS_BOOKWORM: CardRecord = CardRecord::new(
    "Oblivious Bookworm",
    "c7b4c50b-fe76-430d-8f96-208f15ca4cd7",
    "Josh Newton",
    CardRules::unsupported(),
);

// DSK 226 — Peer Past the Veil
pub(in crate::card::sets) static PEER_PAST_THE_VEIL: CardRecord = CardRecord::new(
    "Peer Past the Veil",
    "874eadb6-602e-47dc-8094-82e37ac89c94",
    "Tuan Duong Chu",
    CardRules::new_instant(mana_cost!("{2}{R}{G}")).with_abilities(&[AbilityDef::spell(
        "Discard your hand. Then draw X cards, where X is the number \
         of card types among cards in your graveyard.",
        EffectDef::Sequence(&[
            EffectDef::Discard {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Hand],
                    PlayerRelation::You,
                )),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
            abilities::draw_cards(ValueDef::CardTypesAmongGraveyards(PlayerRelation::You)),
        ]),
    )]),
);

// DSK 227 — Restricted Office // Lecture Hall
pub(in crate::card::sets) static RESTRICTED_OFFICE: CardRecord = CardRecord::new(
    "Restricted Office // Lecture Hall",
    "ac794b2f-b5ea-449d-bb0b-4f0e6cc145ef",
    "Antonio José Manzanedo",
    CardRules::new_enchantment(mana_cost!("{2}{W}{W}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::triggered(
            "When you unlock this door, destroy all creatures with power 3 \
             or greater.",
            TriggerEventDef::DoorUnlocked,
            EffectDef::Destroy {
                object: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::PowerAtLeast(3),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ))),
                then: None,
            },
        )]),
)
.with_composition(|| {
    const FRONT: CardRules = CardRules::new_enchantment(mana_cost!("{2}{W}{W}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::triggered(
            "When you unlock this door, destroy all creatures with power 3 \
             or greater.",
            TriggerEventDef::DoorUnlocked,
            EffectDef::Destroy {
                object: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::PowerAtLeast(3),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ))),
                then: None,
            },
        )]);
    const BACK: CardRules = CardRules::new_enchantment(mana_cost!("{5}{U}{U}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::static_ability(
            "Other permanents you control have hexproof.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::add_ability(&abilities::hexproof()),
            },
        )]);
    const BOTH: CardRules = CardRules::new_enchantment(mana_cost!("{7}{W}{W}{U}{U}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[
            AbilityDef::triggered(
                "When you unlock this door, destroy all creatures with power 3 \
                 or greater.",
                TriggerEventDef::DoorUnlocked,
                EffectDef::Destroy {
                    object: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::PowerAtLeast(3),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Any,
                        ),
                    )),
                    then: None,
                },
            ),
            AbilityDef::static_ability(
                "Other permanents you control have hexproof.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::add_ability(&abilities::hexproof()),
                },
            ),
        ]);
    CardComposition::room(
        "Restricted Office // Lecture Hall",
        "Restricted Office",
        FRONT,
        "Lecture Hall",
        BACK,
        BOTH,
    )
});

// DSK 228 — Rip, Spawn Hunter
// Audit: unsupported — Needs the ordinal number of the current main phase in the turn (CR 505.1b); PostcombatMain also matches third and later main phases, so it cannot restrict survival to the second main phase.
pub(in crate::card::sets) static RIP_SPAWN_HUNTER: CardRecord = CardRecord::new(
    "Rip, Spawn Hunter",
    "5f12e3b3-a260-4913-b13a-7fbb753dd702",
    "Justine Cruz",
    CardRules::unsupported(),
);

// DSK 229 — Rite of the Moth
pub(in crate::card::sets) static RITE_OF_THE_MOTH: CardRecord = CardRecord::new(
    "Rite of the Moth",
    "0e79123f-accd-4193-8c72-b750e0d6f3fa",
    "A. M. Sartor",
    CardRules::new_sorcery(mana_cost!("{1}{W}{B}{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Return target creature card from your graveyard to the \
             battlefield with a finality counter on it. (If a creature \
             with a finality counter on it would die, exile it instead.)",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )],
            EffectDef::WithBattlefieldArrival {
                effect: &EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Battlefield,
                    ZonePlacement::Top,
                ),
                arrival: BattlefieldArrivalDef {
                    modifications: &[BattlefieldEntryModificationDef::AddCountersValue {
                        kind: CounterKind::Finality,
                        amount: ValueDef::Constant(1),
                    }],
                    ..BattlefieldArrivalDef::DEFAULT
                },
            },
        ),
        abilities::flashback(&[CostDef::Mana(mana_cost!("{3}{W}{W}{B}"))]),
    ]),
);

// DSK 230 — Roaring Furnace // Steaming Sauna
pub(in crate::card::sets) static ROARING_FURNACE: CardRecord = CardRecord::new(
    "Roaring Furnace // Steaming Sauna",
    "94352ffb-d716-484d-b018-4e1c033ef2f3",
    "Miklós Ligeti",
    CardRules::new_enchantment(mana_cost!("{1}{R}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::triggered_with_targets(
            "When you unlock this door, this Room deals damage equal to \
             the number of cards in your hand to target creature an \
             opponent controls.",
            TriggerEventDef::DoorUnlocked,
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Hand],
                    PlayerRelation::You,
                )),
            ),
        )]),
)
.with_composition(|| {
    const FRONT: CardRules = CardRules::new_enchantment(mana_cost!("{1}{R}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[AbilityDef::triggered_with_targets(
            "When you unlock this door, this Room deals damage equal to \
             the number of cards in your hand to target creature an \
             opponent controls.",
            TriggerEventDef::DoorUnlocked,
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Hand],
                    PlayerRelation::You,
                )),
            ),
        )]);
    const BACK: CardRules = CardRules::new_enchantment(mana_cost!("{3}{U}{U}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "You have no maximum hand size.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Controller,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::PlayerRule(
                        PlayerRuleDef::NoMaximumHandSize,
                    )),
                },
            ),
            AbilityDef::triggered(
                "At the beginning of your end step, draw a card.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::End,
                    player: PlayerRelation::You,
                },
                abilities::draw_cards(ValueDef::Constant(1)),
            ),
        ]);
    const BOTH: CardRules = CardRules::new_enchantment(mana_cost!("{4}{U}{U}{R}"))
        .with_subtypes(&["Room"])
        .with_abilities(&[
            AbilityDef::triggered_with_targets(
                "When you unlock this door, this Room deals damage equal to \
                 the number of cards in your hand to target creature an \
                 opponent controls.",
                TriggerEventDef::DoorUnlocked,
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::Opponent),
                        owner: None,
                    },
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::Any,
                        &[ZoneKind::Hand],
                        PlayerRelation::You,
                    )),
                ),
            ),
            AbilityDef::static_ability(
                "You have no maximum hand size.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Controller,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::PlayerRule(
                        PlayerRuleDef::NoMaximumHandSize,
                    )),
                },
            ),
            AbilityDef::triggered(
                "At the beginning of your end step, draw a card.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::End,
                    player: PlayerRelation::You,
                },
                abilities::draw_cards(ValueDef::Constant(1)),
            ),
        ]);
    CardComposition::room(
        "Roaring Furnace // Steaming Sauna",
        "Roaring Furnace",
        FRONT,
        "Steaming Sauna",
        BACK,
        BOTH,
    )
});

// DSK 231 — Sawblade Skinripper
// Audit: unsupported — Needs a per-controller turn history count of sacrificed permanents, including those no longer in the graveyard; the global creature-death count is not sacrifice history.
pub(in crate::card::sets) static SAWBLADE_SKINRIPPER: CardRecord = CardRecord::new(
    "Sawblade Skinripper",
    "cbe51964-50d2-49b0-9c3c-81751dcbeade",
    "Zezhou Chen",
    CardRules::unsupported(),
);

// DSK 232 — Shrewd Storyteller
// Audit: unsupported — Needs the ordinal number of the current main phase in the turn (CR 505.1b); PostcombatMain also matches third and later main phases, so it cannot restrict survival to the second main phase.
pub(in crate::card::sets) static SHREWD_STORYTELLER: CardRecord = CardRecord::new(
    "Shrewd Storyteller",
    "f9636877-8fcc-4ad5-8eb2-a5d5ba49583d",
    "David Palumbo",
    CardRules::unsupported(),
);

// DSK 233 — Shroudstomper
pub(in crate::card::sets) static SHROUDSTOMPER: CardRecord = CardRecord::new(
    "Shroudstomper",
    "53f746bf-0642-48db-835e-27a7fe369fef",
    "Campbell White",
    CardRules::new_creature(mana_cost!("{3}{W}{W}{B}{B}"), &["Elemental"], 5, 5).with_abilities(&[
        abilities::deathtouch(),
        AbilityDef::triggered(
            "Whenever this creature enters or attacks, each opponent loses \
             2 life. You gain 2 life and draw a card.",
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
            ]),
            EffectDef::Sequence(&[
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Opponent,
                    amount: ValueDef::Constant(2),
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
                abilities::draw_cards(ValueDef::Constant(1)),
            ]),
        ),
    ]),
);

// DSK 234 — Skullsnap Nuisance
// Audit: unsupported — Needs a committed event for a Room becoming fully unlocked, observed by other permanents and graveyard abilities. DoorUnlocked only dispatches to the door being opened, so it cannot implement the second eerie trigger.
pub(in crate::card::sets) static SKULLSNAP_NUISANCE: CardRecord = CardRecord::new(
    "Skullsnap Nuisance",
    "0fdcdfd0-8c66-4767-a894-58cf0c6d7e07",
    "Allen Douglas",
    CardRules::unsupported(),
);

// DSK 235 — Smoky Lounge // Misty Salon
// Audit: unsupported — Needs mana restricted to Room casting or door-unlock payments and a value counting unlocked Room doors for the token's size.
pub(in crate::card::sets) static SMOKY_LOUNGE: CardRecord = CardRecord::new(
    "Smoky Lounge // Misty Salon",
    "4700987d-fc55-44eb-bc9f-0e0316ca65e2",
    "Marco Gorlei",
    CardRules::unsupported(),
);

// DSK 236 — The Swarmweaver
pub(in crate::card::sets) static THE_SWARMWEAVER: CardRecord = CardRecord::new(
    "The Swarmweaver",
    "dcf3b17b-f2f6-4702-864d-8c96100b0563",
    "Helge C. Balzer",
    CardRules::new_artifact_creature(mana_cost!("{2}{B}{G}"), &["Scarecrow"], 2, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger(
                "When The Swarmweaver enters, create two 1/1 black and green \
                 Insect creature tokens with flying.",
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(INSECT_TOKEN))
                        .with_count(ValueDef::Constant(2)),
                ),
            ),
            AbilityDef::static_ability(
                "Delirium — As long as there are four or more card types among \
                 cards in your graveyard, Insects and Spiders you control get \
                 +1/+1 and have deathtouch.",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                        left: ValueDef::CardTypesAmongGraveyards(PlayerRelation::You),
                        comparison: ComparisonDef::GreaterOrEqual,
                        right: ValueDef::Constant(4),
                    }),
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                            ObjectQueryDef::matching(
                                ObjectPredicateDef::AnyOf(&[
                                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Insect")),
                                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Spider")),
                                ]),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            ),
                        )),
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::modify_power_toughness(
                                ValueDef::Constant(1),
                                ValueDef::Constant(1),
                            ),
                            AppliedEffectDef::add_ability(&abilities::deathtouch()),
                        ]),
                    },
                },
            ),
        ]),
);

// DSK 237 — Undead Sprinter
// Audit: unsupported — Needs turn death history filtered by the dead creature's last-known creature types; the current global death count cannot exclude Zombies.
pub(in crate::card::sets) static UNDEAD_SPRINTER: CardRecord = CardRecord::new(
    "Undead Sprinter",
    "c6b96951-4884-4df7-bdf0-94be2bc044f0",
    "Nino Vecia",
    CardRules::unsupported(),
);

// DSK 238 — Victor, Valgavoth's Seneschal
// Audit: unsupported — Needs a committed event for a Room becoming fully unlocked, observed by other permanents and graveyard abilities. DoorUnlocked only dispatches to the door being opened, so it cannot implement the second eerie trigger.
pub(in crate::card::sets) static VICTOR_VALGAVOTH_S_SENESCHAL: CardRecord = CardRecord::new(
    "Victor, Valgavoth's Seneschal",
    "51392ece-c9f5-46b0-9dce-0a1a0343a536",
    "Jeremy Wilson",
    CardRules::unsupported(),
);

// DSK 239 — Wildfire Wickerfolk
pub(in crate::card::sets) static WILDFIRE_WICKERFOLK: CardRecord = CardRecord::new(
    "Wildfire Wickerfolk",
    "7c7fbc6e-09c6-4f0e-92e2-766aae950b3d",
    "J.Lonnee",
    CardRules::new_artifact_creature(mana_cost!("{R}{G}"), &["Scarecrow"], 3, 2).with_abilities(&[
        abilities::haste(),
        AbilityDef::static_ability(
            "Delirium — This creature gets +1/+1 and has trample as long \
             as there are four or more card types among cards in your \
             graveyard.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::CardTypesAmongGraveyards(PlayerRelation::You),
                    comparison: ComparisonDef::GreaterOrEqual,
                    right: ValueDef::Constant(4),
                }),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_ability(&abilities::trample()),
                    ]),
                },
            },
        ),
    ]),
);

// DSK 240 — Winter, Misanthropic Guide
// Audit: unsupported — Needs a dynamically computed maximum hand size; current player rules provide no maximum or a fixed modifier, not a value derived from changing graveyard types.
pub(in crate::card::sets) static WINTER_MISANTHROPIC_GUIDE: CardRecord = CardRecord::new(
    "Winter, Misanthropic Guide",
    "e9b81421-44cb-440f-a6ac-3ddf620f1989",
    "Jodie Muir",
    CardRules::unsupported(),
);

// DSK 241 — Zimone, All-Questioning
// Audit: unsupported — Needs turn history for a land entering even if that land later leaves, plus an unbounded primality predicate on the controlled land count.
pub(in crate::card::sets) static ZIMONE_ALL_QUESTIONING: CardRecord = CardRecord::new(
    "Zimone, All-Questioning",
    "7722f4f7-fe38-4107-a715-7b27b6a4e341",
    "Ekaterina Burmak",
    CardRules::unsupported(),
);

// DSK 242 — Attack-in-the-Box
pub(in crate::card::sets) static ATTACK_IN_THE_BOX: CardRecord = CardRecord::new(
    "Attack-in-the-Box",
    "a477dc3e-0fa1-4ce4-b3de-8cae0d1a0763",
    "Domenico Cava",
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Toy"], 2, 4).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature attacks, you may have it get +4/+0 \
             until end of turn. If you do, sacrifice it at the beginning \
             of the next end step.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Sequence(&[
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(4),
                            ValueDef::Constant(0),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                    EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                        "At the beginning of the next end step, sacrifice this permanent.",
                        TriggerEventDef::StepBegins {
                            step: TurnStepDef::End,
                            player: PlayerRelation::Any,
                        },
                        EffectDef::sacrifice(EffectRecipientDef::Source),
                    ))),
                ]),
            },
        ),
    ]),
);

// DSK 243 — Bear Trap
pub(in crate::card::sets) static BEAR_TRAP: CardRecord = CardRecord::new(
    "Bear Trap",
    "e27c27d6-dc3a-4420-bb35-d97dc216e002",
    "Michele Giorgi",
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        abilities::flash(),
        AbilityDef::activated_with_targets(
            "{3}, {T}, Sacrifice this artifact: It deals 3 damage to \
             target creature.",
            &[
                CostDef::Mana(mana_cost!("{3}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(3),
            ),
        ),
    ]),
);

// DSK 244 — Conductive Machete
pub(in crate::card::sets) static CONDUCTIVE_MACHETE: CardRecord = CardRecord::new(
    "Conductive Machete",
    "1cf37c1a-096b-4306-97cf-bc4d4c47d4a1",
    "Steven Russell Black",
    CardRules::new_artifact(mana_cost!("{4}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::enters_trigger(
                "When this Equipment enters, manifest dread, then attach this \
                 Equipment to that creature. (Look at the top two cards of \
                 your library. Put one onto the battlefield face down as a 2/2 \
                 creature and the other into your graveyard. Turn it face up \
                 any time for its mana cost if it's a creature card.)",
                abilities::bind_top_cards_then(
                    PlayerRefDef::EffectController,
                    ValueDef::Constant(2),
                    &EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(crate::Binding!(
                            "dread_permanent"
                        )),
                        unchosen: Some(crate::Binding!("dread_graveyard")),
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Binding(crate::ParentBinding),
                        exclude: None,
                        minimum: 1,
                        maximum: 1,
                        visibility: ChoiceVisibilityDef::Private,
                        then: &EffectDef::PutObjectsOntoBattlefieldFaceDown(
                            PutObjectsOntoBattlefieldFaceDownDef {
                                input: ObjectSetDef::Binding(crate::Binding!("dread_permanent")),
                                controller: PlayerRefDef::EffectController,
                                characteristics: crate::card::face_down::manifest(),
                                turn_up_for_mana_cost: true,
                                moved: Some(crate::Binding!("manifested")),
                                then: &EffectDef::Sequence(&[
                                    EffectDef::MoveObjects(MoveObjectsDef {
                                        input: ObjectSetDef::Binding(crate::Binding!(
                                            "dread_graveyard"
                                        )),
                                        from: Some(ZoneKind::Library),
                                        zone: ZoneKind::Graveyard,
                                        placement: ZonePlacement::Top,
                                        moved: None,
                                        then: &EffectDef::None,
                                    }),
                                    EffectDef::Attach {
                                        object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                            crate::Binding!("manifested"),
                                        )),
                                    },
                                ]),
                            },
                        ),
                    }),
                ),
            ),
            AbilityDef::static_ability(
                "Equipped creature gets +2/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(1),
                    ),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{4}"))], "Equip {4}"),
        ]),
);

// DSK 245 — Dissection Tools
pub(in crate::card::sets) static DISSECTION_TOOLS: CardRecord = CardRecord::new(
    "Dissection Tools",
    "048bb2c6-91bd-4d6a-a070-c73d8277c264",
    "Diana Franco",
    CardRules::new_artifact(mana_cost!("{5}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::enters_trigger(
                "When this Equipment enters, manifest dread, then attach this \
                 Equipment to that creature.",
                abilities::bind_top_cards_then(
                    PlayerRefDef::EffectController,
                    ValueDef::Constant(2),
                    &EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(crate::Binding!(
                            "dread_permanent"
                        )),
                        unchosen: Some(crate::Binding!("dread_graveyard")),
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Binding(crate::ParentBinding),
                        exclude: None,
                        minimum: 1,
                        maximum: 1,
                        visibility: ChoiceVisibilityDef::Private,
                        then: &EffectDef::PutObjectsOntoBattlefieldFaceDown(
                            PutObjectsOntoBattlefieldFaceDownDef {
                                input: ObjectSetDef::Binding(crate::Binding!("dread_permanent")),
                                controller: PlayerRefDef::EffectController,
                                characteristics: crate::card::face_down::manifest(),
                                turn_up_for_mana_cost: true,
                                moved: Some(crate::Binding!("manifested")),
                                then: &EffectDef::Sequence(&[
                                    EffectDef::MoveObjects(MoveObjectsDef {
                                        input: ObjectSetDef::Binding(crate::Binding!(
                                            "dread_graveyard"
                                        )),
                                        from: Some(ZoneKind::Library),
                                        zone: ZoneKind::Graveyard,
                                        placement: ZonePlacement::Top,
                                        moved: None,
                                        then: &EffectDef::None,
                                    }),
                                    EffectDef::Attach {
                                        object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                            crate::Binding!("manifested"),
                                        )),
                                    },
                                ]),
                            },
                        ),
                    }),
                ),
            ),
            AbilityDef::static_ability(
                "Equipped creature gets +2/+2 and has deathtouch and lifelink.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(2),
                        ),
                        AppliedEffectDef::add_ability(&abilities::deathtouch()),
                        AppliedEffectDef::add_ability(&abilities::lifelink()),
                    ]),
                },
            ),
            abilities::equip(
                &[CostDef::sacrifice_permanent(ObjectPredicateDef::HasType(
                    CardType::Creature,
                ))],
                "Equip—Sacrifice a creature.",
            ),
        ]),
);

// DSK 246 — Found Footage
// Audit: unsupported — Needs a continuous visibility permission for opponents' face-down battlefield creatures, distinct from looking at libraries or one-shot revealing cards.
pub(in crate::card::sets) static FOUND_FOOTAGE: CardRecord = CardRecord::new(
    "Found Footage",
    "b12eb087-762e-4e7d-a6e0-f48df603b7c7",
    "Jarel Threat",
    CardRules::unsupported(),
);

// DSK 247 — Friendly Teddy
pub(in crate::card::sets) static FRIENDLY_TEDDY: CardRecord = CardRecord::new(
    "Friendly Teddy",
    "82b142be-4586-487a-8dd8-a55eff776458",
    "Johann Bodin",
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Bear", "Toy"], 2, 2).with_abilities(&[
        abilities::dies_trigger(
            "When this creature dies, each player draws a card.",
            EffectDef::Sequence(&[
                abilities::draw_cards(ValueDef::Constant(1)),
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Opponent,
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
    ]),
);

// DSK 248 — Ghost Vacuum
// Audit: unsupported — Needs the returned creatures' 1/1 base size and additional Spirit type established simultaneously with battlefield entry (CR 611.2e); applying those continuous effects after returning them lets entry replacements and triggers observe the wrong characteristics.
pub(in crate::card::sets) static GHOST_VACUUM: CardRecord = CardRecord::new(
    "Ghost Vacuum",
    "8ac39c01-127f-4471-bc74-11a90c48e306",
    "David Szabo",
    CardRules::unsupported(),
);

// DSK 249 — Glimmerlight
pub(in crate::card::sets) static GLIMMERLIGHT: CardRecord = CardRecord::new(
    "Glimmerlight",
    "1071691c-5c65-42d4-ac96-d302185ca678",
    "Wero Gallo",
// The Equipment brings its own creature to hold it, so two mana buys a
    // 2/2 across two bodies rather than a dead artifact.
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::enters_trigger(
                "When this Equipment enters, create a 1/1 white Glimmer enchantment creature token.",
                // An enchantment creature, so it needs the general token
                // constructor rather than the creature-only shorthand.
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                    TokenCharacteristics::new(
                        CardTypeSet::single(CardType::Enchantment).with(CardType::Creature),
                        &["Glimmer"],
                        &[ManaColor::White],
                        Some(CreatureStats {
                            power: 1,
                            toughness: 1,
                        }),
                    ),
                ))),
            ),
            AbilityDef::static_ability(
                "Equipped creature gets +1/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{1}"))], "Equip {1}"),
        ]),
);

// DSK 250 — Haunted Screen
pub(in crate::card::sets) static HAUNTED_SCREEN: CardRecord = CardRecord::new(
    "Haunted Screen",
    "5e7d552f-a7ac-4a49-a582-9d378137005f",
    "Sean Murray",
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {W} or {B}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Black,
            ])),
        ),
        AbilityDef::activated_mana(
            "{T}, Pay 1 life: Add {G}, {U}, or {R}.",
            &[CostDef::TapSource, CostDef::PayLife(1)],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Green,
                ManaColor::Blue,
                ManaColor::Red,
            ])),
        ),
        AbilityDef::activated(
            "{7}: Put seven +1/+1 counters on this artifact. It becomes a \
             0/0 Spirit creature in addition to its other types. Activate \
             only once.",
            &[CostDef::Mana(mana_cost!("{7}"))],
            EffectDef::Sequence(&[
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(7),
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                        AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&[
                            "Spirit",
                        ])),
                        AppliedEffectDef::set_base_power_toughness(
                            ValueDef::Constant(0),
                            ValueDef::Constant(0),
                        ),
                    ]),
                    duration: ResolvedEffectDurationDef::Permanent,
                },
            ]),
        )
        .once_per_object(),
    ]),
);

// DSK 251 — Keys to the House
// Audit: unsupported — Needs resolving lock/unlock effects for a chosen door of a targeted Room; the existing special action only pays to unlock a locked door.
pub(in crate::card::sets) static KEYS_TO_THE_HOUSE: CardRecord = CardRecord::new(
    "Keys to the House",
    "8c11a413-7f33-4b63-bdd9-e143e529f56d",
    "Artur Treffner",
    CardRules::unsupported(),
);

// DSK 252 — Malevolent Chandelier
pub(in crate::card::sets) static MALEVOLENT_CHANDELIER: CardRecord = CardRecord::new(
    "Malevolent Chandelier",
    "ef09d296-4493-47f7-ad76-ad76c747df78",
    "Mirko Failoni",
    CardRules::new_artifact_creature(mana_cost!("{6}"), &["Construct"], 4, 4).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated_with_targets(
            "{2}: Put target card from a graveyard on the bottom of its \
             owner's library. Activate only as a sorcery.",
            &[CostDef::Mana(mana_cost!("{2}"))],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::Any),
                },
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Library,
                ZonePlacement::Bottom,
            ),
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// DSK 253 — Marvin, Murderous Mimic
// Audit: unsupported — Needs continuous copying of activated abilities from dynamically matching other creatures, preserving their mana and nonmana ability semantics and updating as the source set changes.
pub(in crate::card::sets) static MARVIN_MURDEROUS_MIMIC: CardRecord = CardRecord::new(
    "Marvin, Murderous Mimic",
    "66898970-b99b-48f2-9240-68c301c95500",
    "Mirko Failoni",
    CardRules::unsupported(),
);

// DSK 254 — Saw
// Audit: unsupported — Needs a resolving sacrifice predicate excluding both the Equipment and its attached creature; the general cost object predicate can exclude Source but cannot compare a candidate with the attached host identity.
pub(in crate::card::sets) static SAW: CardRecord = CardRecord::new(
    "Saw",
    "603c3ef4-4ef1-4db8-9ed2-e2b0926269d5",
    "Jarel Threat",
    CardRules::unsupported(),
);

// DSK 255 — Abandoned Campground
// Audit: unsupported — Needs a prospective entry replacement condition comparing each player's life total with 13; ordinary resolution-time life values are not accepted by the entry-condition evaluator.
pub(in crate::card::sets) static ABANDONED_CAMPGROUND: CardRecord = CardRecord::new(
    "Abandoned Campground",
    "ee0565f5-ebdb-43f9-bbb4-0485b1968937",
    "Cristi Balanescu",
    CardRules::unsupported(),
);

// DSK 256 — Blazemire Verge
pub(in crate::card::sets) static BLAZEMIRE_VERGE: CardRecord = CardRecord::new(
    "Blazemire Verge",
    "d151c8e2-d715-470d-868a-f45191db9fa0",
    "Andrew Mar",
    // Untapped and free either way: the black is unconditional, and the red
    // is what the rest of the mana base is for.
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {B}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Black)),
        ),
        AbilityDef::activated_mana_if(
            "{T}: Add {R}. Activate only if you control a Swamp or a Mountain.",
            &[CostDef::TapSource],
            &TriggerConditionDef::ObjectCount {
                // The same condition in this cycle's Rakdos colours. Either type answers
                // it, so a Badlands is both halves at once.
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::HasAnyBasicLandType(&[
                        BasicLandType::Swamp,
                        BasicLandType::Mountain,
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)),
        ),
    ]),
);

// DSK 257 — Bleeding Woods
// Audit: unsupported — Needs a prospective entry replacement condition comparing each player's life total with 13; ordinary resolution-time life values are not accepted by the entry-condition evaluator.
pub(in crate::card::sets) static BLEEDING_WOODS: CardRecord = CardRecord::new(
    "Bleeding Woods",
    "cb224874-aff5-461f-82ee-89b06663231a",
    "Henry Peters",
    CardRules::unsupported(),
);

// DSK 258 — Etched Cornfield
// Audit: unsupported — Needs a prospective entry replacement condition comparing each player's life total with 13; ordinary resolution-time life values are not accepted by the entry-condition evaluator.
pub(in crate::card::sets) static ETCHED_CORNFIELD: CardRecord = CardRecord::new(
    "Etched Cornfield",
    "f8900b89-0e10-4602-bba2-da8d60ea5885",
    "Randy Gallegos",
    CardRules::unsupported(),
);

// DSK 259 — Floodfarm Verge
pub(in crate::card::sets) static FLOODFARM_VERGE: CardRecord = CardRecord::new(
    "Floodfarm Verge",
    "d53ed0db-1199-44b3-8eda-8189dfcf53d1",
    "Randy Gallegos",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::White),
        AbilityDef::activated_mana_if(
            "{T}: Add {U}. Activate only if you control a Plains or an Island.",
            &[CostDef::TapSource],
            &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::HasAnyBasicLandType(&[
                        BasicLandType::Plains,
                        BasicLandType::Island,
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Blue)),
        ),
    ]),
);

// DSK 260 — Gloomlake Verge
pub(in crate::card::sets) static GLOOMLAKE_VERGE: CardRecord = CardRecord::new(
    "Gloomlake Verge",
    "83f510b7-4cbd-4883-9c26-c8824bc668ac",
    "Marco Gorlei",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Blue),
        AbilityDef::activated_mana_if(
            "{T}: Add {B}. Activate only if you control an Island or a Swamp.",
            &[CostDef::TapSource],
            &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::HasAnyBasicLandType(&[
                        BasicLandType::Island,
                        BasicLandType::Swamp,
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Black)),
        ),
    ]),
);

// DSK 261 — Hushwood Verge
pub(in crate::card::sets) static HUSHWOOD_VERGE: CardRecord = CardRecord::new(
    "Hushwood Verge",
    "ec288d76-c1f5-471b-8a53-504f88469c1b",
    "Kasia 'Kafis' Zielińska",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Green),
        AbilityDef::activated_mana_if(
            "{T}: Add {W}. Activate only if you control a Forest or a Plains.",
            &[CostDef::TapSource],
            &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::HasAnyBasicLandType(&[
                        BasicLandType::Forest,
                        BasicLandType::Plains,
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::White)),
        ),
    ]),
);

// DSK 262 — Lakeside Shack
// Audit: unsupported — Needs a prospective entry replacement condition comparing each player's life total with 13; ordinary resolution-time life values are not accepted by the entry-condition evaluator.
pub(in crate::card::sets) static LAKESIDE_SHACK: CardRecord = CardRecord::new(
    "Lakeside Shack",
    "a9367acd-393a-4966-ba60-af2ecd4e7596",
    "Bartek Fedyczak",
    CardRules::unsupported(),
);

// DSK 263 — Murky Sewer
// Audit: unsupported — Needs a prospective entry replacement condition comparing each player's life total with 13; ordinary resolution-time life values are not accepted by the entry-condition evaluator.
pub(in crate::card::sets) static MURKY_SEWER: CardRecord = CardRecord::new(
    "Murky Sewer",
    "6098d8be-4e3f-455d-8799-91435bf45a1c",
    "Martin de Diego Sádaba",
    CardRules::unsupported(),
);

// DSK 264 — Neglected Manor
// Audit: unsupported — Needs a prospective entry replacement condition comparing each player's life total with 13; ordinary resolution-time life values are not accepted by the entry-condition evaluator.
pub(in crate::card::sets) static NEGLECTED_MANOR: CardRecord = CardRecord::new(
    "Neglected Manor",
    "11cf1531-8a3c-4e28-a114-d3a342b33bb6",
    "Carlos Palma Cruchaga",
    CardRules::unsupported(),
);

// DSK 265 — Peculiar Lighthouse
// Audit: unsupported — Needs a prospective entry replacement condition comparing each player's life total with 13; ordinary resolution-time life values are not accepted by the entry-condition evaluator.
pub(in crate::card::sets) static PECULIAR_LIGHTHOUSE: CardRecord = CardRecord::new(
    "Peculiar Lighthouse",
    "3a6e40c0-e70e-4353-a920-9851cfac71dd",
    "Raymond Bonilla",
    CardRules::unsupported(),
);

// DSK 266 — Raucous Carnival
// Audit: unsupported — Needs a prospective entry replacement condition comparing each player's life total with 13; ordinary resolution-time life values are not accepted by the entry-condition evaluator.
pub(in crate::card::sets) static RAUCOUS_CARNIVAL: CardRecord = CardRecord::new(
    "Raucous Carnival",
    "3604a211-9bf7-474e-bd78-32a862f4259c",
    "Josu Solano",
    CardRules::unsupported(),
);

// DSK 267 — Razortrap Gorge
// Audit: unsupported — Needs a prospective entry replacement condition comparing each player's life total with 13; ordinary resolution-time life values are not accepted by the entry-condition evaluator.
pub(in crate::card::sets) static RAZORTRAP_GORGE: CardRecord = CardRecord::new(
    "Razortrap Gorge",
    "98d0d067-b52d-47ec-ba7b-8cfcd716c0e5",
    "Filip Burburan",
    CardRules::unsupported(),
);

// DSK 268 — Strangled Cemetery
// Audit: unsupported — Needs a prospective entry replacement condition comparing each player's life total with 13; ordinary resolution-time life values are not accepted by the entry-condition evaluator.
pub(in crate::card::sets) static STRANGLED_CEMETERY: CardRecord = CardRecord::new(
    "Strangled Cemetery",
    "c1ce9250-bdbe-4c77-9243-6db9ffffe69b",
    "Marco Gorlei",
    CardRules::unsupported(),
);

// DSK 269 — Terramorphic Expanse (reprint)
const TERRAMORPHIC_EXPANSE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tsp::TERRAMORPHIC_EXPANSE,
    "b379c8f1-817c-4f18-8f58-45c40504433e",
    "Sam Burley",
);

// DSK 270 — Thornspire Verge
pub(in crate::card::sets) static THORNSPIRE_VERGE: CardRecord = CardRecord::new(
    "Thornspire Verge",
    "7e1cdc03-6faa-4138-9a52-caafbe34fb59",
    "Kasia 'Kafis' Zielińska",
    // Untapped and free either way: the red is unconditional, and the green
    // is what the rest of the mana base is for.
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {R}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)),
        ),
        AbilityDef::activated_mana_if(
            "{T}: Add {G}. Activate only if you control a Mountain or a Forest.",
            &[CostDef::TapSource],
            &TriggerConditionDef::ObjectCount {
                // The verge condition in this cycle's Gruul colours. Either type answers
                // it, so a Taiga is both halves at once.
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::HasAnyBasicLandType(&[
                        BasicLandType::Mountain,
                        BasicLandType::Forest,
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
        ),
    ]),
);

// DSK 271 — Valgavoth's Lair
pub(in crate::card::sets) static VALGAVOTH_S_LAIR: CardRecord = CardRecord::new(
    "Valgavoth's Lair",
    "65ff914e-3f3e-4b7a-b69d-73575b68fb8e",
    "Martin de Diego Sádaba",
    CardRules::new_land(&[])
        .with_type(CardType::Enchantment)
        .with_abilities(&[
            abilities::hexproof(),
            abilities::enters_tapped(CardType::Land),
            AbilityDef::as_enters(
                "As this land enters, choose a color.",
                ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(
                    BattlefieldEntryScalarChoiceDef::COLOR,
                )),
            ),
            AbilityDef::activated_mana(
                "{T}: Add one mana of the chosen color.",
                &[CostDef::TapSource],
                EffectDef::AddMana(AddManaEffectDef::one_of_type(ManaTypeDef::ChosenColor)),
            ),
        ]),
);

// DSK 272 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAINS,
    "e67ce864-bf29-42f1-81ca-a98022892eec",
    "Dan Mumford",
);

// DSK 273 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND,
    "40e3bf00-84cc-498c-b214-1052b4904d92",
    "Dan Mumford",
);

// DSK 274 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWAMP,
    "7442c1c7-1c10-4387-92e6-4bdea263064f",
    "Dan Mumford",
);

// DSK 275 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MOUNTAIN,
    "8c8841b2-9b4e-4f65-8e30-1e9423cb8fbc",
    "Dan Mumford",
);

// DSK 276 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOREST,
    "ba2a4cbb-f325-4178-80db-7090f5672414",
    "Dan Mumford",
);

// DSK 277 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    1,
    "1b499b37-efaf-4484-95e8-a70a9778c804",
    "Marco Gorlei",
);

// DSK 278 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    2,
    "756abff9-9810-4e2d-b1d7-ec4e2d6d5187",
    "Josu Hernaiz",
);

// DSK 279 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    1,
    "947702ca-d065-4368-9f26-f859d4642cb6",
    "Raymond Bonilla",
);

// DSK 280 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    2,
    "e1d10d9c-8771-4870-aebf-e767d0fada32",
    "Leonardo Borazio",
);

// DSK 281 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    1,
    "3c51de66-a3ed-4ca9-befb-9813e96c4ade",
    "Martin de Diego Sádaba",
);

// DSK 282 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    2,
    "cbd95de0-702a-4b88-a1cc-981cf1d9673e",
    "Néstor Ossandón Leal",
);

// DSK 283 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    1,
    "accb56f0-3903-4894-86f0-3965162064d4",
    "Ralph Horsley",
);

// DSK 284 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    2,
    "1a44d8a4-21de-497d-9eed-702d7b592728",
    "Néstor Ossandón Leal",
);

// DSK 285 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    1,
    "e84c0880-728d-4ac2-b685-4f18f66c24db",
    "Martin de Diego Sádaba",
);

// DSK 286 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    2,
    "0da5fbc2-24ad-4520-a60a-436d3a485fec",
    "Josu Hernaiz",
);

// DSK 287 — Grand Entryway // Elegant Rotunda (alternate printing)
const GRAND_ENTRYWAY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GRAND_ENTRYWAY,
    1,
    "6ef39d0e-4caa-4808-949f-1d75e160dbc3",
    "Carlos Palma Cruchaga",
);

// DSK 288 — Optimistic Scavenger (alternate printing)
const OPTIMISTIC_SCAVENGER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OPTIMISTIC_SCAVENGER,
    1,
    "f67d6bd4-b03a-4d04-bc38-85b3ee39aa8a",
    "Brian Valeza",
);

// DSK 289 — Reluctant Role Model (alternate printing)
const RELUCTANT_ROLE_MODEL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RELUCTANT_ROLE_MODEL,
    1,
    "693a6f3e-d517-4978-8958-db189fe3f1f9",
    "Chris Rallis",
);

// DSK 290 — Entity Tracker (alternate printing)
const ENTITY_TRACKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ENTITY_TRACKER,
    1,
    "2c0a079e-26f4-44ed-859a-f7df4b40a3cd",
    "Cristi Balanescu",
);

// DSK 291 — Stay Hidden, Stay Silent (alternate printing)
const STAY_HIDDEN_STAY_SILENT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STAY_HIDDEN_STAY_SILENT,
    1,
    "9fdd7bdd-2ba8-4658-9825-741aecad5f42",
    "Josu Hernaiz",
);

// DSK 292 — Come Back Wrong (alternate printing)
const COME_BACK_WRONG_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &COME_BACK_WRONG,
    1,
    "4abe8236-a9e3-435a-a459-833ec3af93c7",
    "David Auden Nash",
);

// DSK 293 — Meathook Massacre II (alternate printing)
const MEATHOOK_MASSACRE_II_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MEATHOOK_MASSACRE_II,
    1,
    "cacb88b3-96a9-45e4-8053-2f8fcbe4028e",
    "Tiffany Turrill",
);

// DSK 294 — Unstoppable Slasher (alternate printing)
const UNSTOPPABLE_SLASHER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &UNSTOPPABLE_SLASHER,
    1,
    "57d865cf-8c5f-4c27-a731-33564b01bc64",
    "Maxime Minard",
);

// DSK 295 — Clockwork Percussionist
// Audit: unsupported — Needs exile-play permission that expires at cleanup of the controller's next turn; the current turn-count permission can remain usable during the following opponent turn.
pub(in crate::card::sets) static CLOCKWORK_PERCUSSIONIST: CardRecord = CardRecord::new(
    "Clockwork Percussionist",
    "e44340c7-d3bb-4cf9-a105-ebbf6ce3ace1",
    "Eric Wilkerson",
    CardRules::unsupported(),
);

// DSK 296 — Cursed Recording (alternate printing)
const CURSED_RECORDING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CURSED_RECORDING,
    1,
    "f1d2c67b-b523-4da3-826e-5f56f19914ba",
    "Kim Sokol",
);

// DSK 297 — Norin, Swift Survivalist (alternate printing)
const NORIN_SWIFT_SURVIVALIST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &NORIN_SWIFT_SURVIVALIST,
    1,
    "f86f4026-b75a-4d74-871e-715ece0225d3",
    "Yigit Koroglu",
);

// DSK 298 — The Rollercrusher Ride (alternate printing)
const THE_ROLLERCRUSHER_RIDE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_ROLLERCRUSHER_RIDE,
    1,
    "bb57a981-e70b-4298-b620-577914d67cb7",
    "Deruchenko Alexander",
);

// DSK 299 — Kona, Rescue Beastie (alternate printing)
const KONA_RESCUE_BEASTIE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KONA_RESCUE_BEASTIE,
    1,
    "3c3bfb93-c7f2-4a96-8c57-c356d8ce21c0",
    "Brian Valeza",
);

// DSK 300 — Oblivious Bookworm (alternate printing)
const OBLIVIOUS_BOOKWORM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OBLIVIOUS_BOOKWORM,
    1,
    "9f328937-6585-4325-a1f2-95facab2c58a",
    "Josh Newton",
);

// DSK 301 — The Swarmweaver (alternate printing)
const THE_SWARMWEAVER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_SWARMWEAVER,
    1,
    "9b8fbdac-e3bb-4b6b-b210-febb23846c23",
    "Helge C. Balzer",
);

// DSK 302 — Ghostly Dancers (alternate printing)
const GHOSTLY_DANCERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GHOSTLY_DANCERS,
    1,
    "851b9caa-bb70-4d84-8139-b70867193922",
    "Scott Buoncristiano",
);

// DSK 303 — Reluctant Role Model (alternate printing)
const RELUCTANT_ROLE_MODEL_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &RELUCTANT_ROLE_MODEL,
    2,
    "1fcea7eb-3174-4549-a1de-8dda577cd0b9",
    "Ivan Shavrin",
);

// DSK 304 — Split Up (alternate printing)
const SPLIT_UP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPLIT_UP,
    1,
    "7428a157-67e8-48fb-9882-54bf3ae001e3",
    "Toni Infante",
);

// DSK 305 — Unidentified Hovership (alternate printing)
const UNIDENTIFIED_HOVERSHIP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &UNIDENTIFIED_HOVERSHIP,
    1,
    "426b1aa9-e94f-4654-88c7-9cea52f1bc0a",
    "Calder Moore",
);

// DSK 306 — Unwanted Remake (alternate printing)
const UNWANTED_REMAKE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &UNWANTED_REMAKE,
    1,
    "be897dda-8603-4264-a022-87870f635b40",
    "Ivan Shavrin",
);

// DSK 307 — Entity Tracker (alternate printing)
const ENTITY_TRACKER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ENTITY_TRACKER,
    2,
    "885098d1-f9b4-429b-97b5-c5c448111e22",
    "Ivan Shavrin",
);

// DSK 308 — Marina Vendrell's Grimoire (alternate printing)
const MARINA_VENDRELL_S_GRIMOIRE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MARINA_VENDRELL_S_GRIMOIRE,
    1,
    "b976d642-3cf0-42c5-b3ba-12818e596f9c",
    "Lenka Šimečková & Scott Okumura",
);

// DSK 309 — Come Back Wrong (alternate printing)
const COME_BACK_WRONG_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &COME_BACK_WRONG,
    2,
    "0e8761b8-3583-422d-a23c-b3f0915ce15b",
    "Alexis Ziritt",
);

// DSK 310 — Demonic Counsel (alternate printing)
const DEMONIC_COUNSEL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DEMONIC_COUNSEL,
    1,
    "fa9defeb-d28e-451e-9333-914394231fbb",
    "Bastien Grivet",
);

// DSK 311 — Meathook Massacre II (alternate printing)
const MEATHOOK_MASSACRE_II_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MEATHOOK_MASSACRE_II,
    2,
    "b3686075-cef4-48b7-ac50-3e27282a3f70",
    "SchmandrewART",
);

// DSK 312 — Unstoppable Slasher (alternate printing)
const UNSTOPPABLE_SLASHER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &UNSTOPPABLE_SLASHER,
    2,
    "0a30a301-8e87-46e7-89aa-cd55b1a14420",
    "SchmandrewART",
);

// DSK 313 — Withering Torment (alternate printing)
const WITHERING_TORMENT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WITHERING_TORMENT,
    1,
    "87e03d07-c491-4ea1-a2f7-49c022005059",
    "Ivan Shavrin",
);

// DSK 314 — Chainsaw
pub(in crate::card::sets) static CHAINSAW: CardRecord = CardRecord::new(
    "Chainsaw",
    "1c8d0f4e-6b1e-4444-8851-adf857273964",
    "Alexis Ziritt",
// Two mana that shoots something on the way in and then grows for the
    // rest of the game, on a board where creatures keep dying anyway.
    CardRules::new_artifact(mana_cost!("{1}{R}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::enters_trigger_with_targets(
                "When this Equipment enters, it deals 3 damage to up to one target creature.",
                // "Up to one target creature": the Equipment arrives whether or not there
                // is anything worth shooting.
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    1,
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(3),
                ),
            ),
            // One counter for the batch rather than one per creature, which is what
            // "one or more" means: a board wipe revs it once.
            AbilityDef::triggered(
                "Whenever one or more creatures die, put a rev counter on this Equipment.",
                TriggerEventDef::ObjectsDied {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::named("rev"),
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::static_ability(
                "Equipped creature gets +X/+0, where X is the number of rev counters on this Equipment.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::CountersOnSource(CounterKind::named("rev")),
                        ValueDef::Constant(0),
                    ),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{3}"))], "Equip {3}"),
        ]),
);

// DSK 315 — Cursed Recording (alternate printing)
const CURSED_RECORDING_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &CURSED_RECORDING,
    2,
    "5aacedd1-de6b-4485-891e-0283ef33f3d4",
    "Alexis Ziritt",
);

// DSK 316 — Fear of Missing Out (alternate printing)
const FEAR_OF_MISSING_OUT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FEAR_OF_MISSING_OUT,
    1,
    "45b924a5-6533-4ca6-bd2e-32debdfb6c08",
    "Cacho Rubione",
);

// DSK 317 — The Rollercrusher Ride (alternate printing)
const THE_ROLLERCRUSHER_RIDE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THE_ROLLERCRUSHER_RIDE,
    2,
    "4d14e9ee-f82f-4062-b972-9486377dacdb",
    "Cacho Rubione",
);

// DSK 318 — Waltz of Rage (alternate printing)
const WALTZ_OF_RAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WALTZ_OF_RAGE,
    1,
    "dcecce11-59e3-4c95-bff9-b02d004c917b",
    "SchmandrewART",
);

// DSK 319 — Balustrade Wurm (alternate printing)
const BALUSTRADE_WURM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BALUSTRADE_WURM,
    1,
    "d7944c86-c578-48fa-b222-7fce3f5d301d",
    "Alexis Ziritt",
);

// DSK 320 — Hedge Shredder (alternate printing)
const HEDGE_SHREDDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HEDGE_SHREDDER,
    1,
    "01595caa-7abd-4b47-8015-3754d8eb39cf",
    "Neo.G",
);

// DSK 321 — Insidious Fungus (alternate printing)
const INSIDIOUS_FUNGUS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &INSIDIOUS_FUNGUS,
    1,
    "68fadf08-c799-47da-8330-5d0df7b41ca1",
    "Bastien Grivet",
);

// DSK 322 — Omnivorous Flytrap (alternate printing)
const OMNIVOROUS_FLYTRAP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OMNIVOROUS_FLYTRAP,
    1,
    "f779cc02-2470-4fc0-ae08-503adeb8e0a9",
    "Chun Lo",
);

// DSK 323 — Under the Skin (alternate printing)
const UNDER_THE_SKIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &UNDER_THE_SKIN,
    1,
    "9eb34e17-bc12-4a4e-a418-0dbd3e6665a8",
    "Cacho Rubione",
);

// DSK 324 — Valgavoth's Onslaught (alternate printing)
const VALGAVOTH_S_ONSLAUGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VALGAVOTH_S_ONSLAUGHT,
    1,
    "3518fbe6-5640-4aca-a2fd-e83c82d2e2fd",
    "Cacho Rubione",
);

// DSK 325 — Peer Past the Veil (alternate printing)
const PEER_PAST_THE_VEIL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PEER_PAST_THE_VEIL,
    1,
    "b70e7b3a-3fe2-4f3c-8292-b44e4a0fc5a4",
    "Scott Buoncristiano",
);

// DSK 326 — Ghost Vacuum (alternate printing)
const GHOST_VACUUM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GHOST_VACUUM,
    1,
    "abf254c0-7c3c-4b68-8b3e-4ae444baa886",
    "Lenka Šimečková & Scott Okumura",
);

// DSK 327 — Valgavoth's Lair (alternate printing)
const VALGAVOTH_S_LAIR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VALGAVOTH_S_LAIR,
    1,
    "7b5041f1-7cb8-4b60-94f6-436430bbece4",
    "Ivan Shavrin",
);

// DSK 328 — Kaito, Bane of Nightmares (alternate printing)
const KAITO_BANE_OF_NIGHTMARES_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &KAITO_BANE_OF_NIGHTMARES,
    2,
    "79d24cf8-107e-4b5b-a4f5-a7498647abef",
    "Marta Nael",
);

// DSK 329 — Blazemire Verge (alternate printing)
const BLAZEMIRE_VERGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BLAZEMIRE_VERGE,
    1,
    "73a926c5-ba2b-4ac5-9717-6c9181f9a827",
    "Allen Douglas",
);

// DSK 330 — Floodfarm Verge (alternate printing)
const FLOODFARM_VERGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FLOODFARM_VERGE,
    1,
    "025152a1-4fdf-4e4e-a792-c540dd43fccd",
    "Sam Burley",
);

// DSK 331 — Gloomlake Verge (alternate printing)
const GLOOMLAKE_VERGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GLOOMLAKE_VERGE,
    1,
    "b414e4bd-a443-4ab3-bee4-1ad1d039aa1a",
    "Julian Kok Joon Wen",
);

// DSK 332 — Hushwood Verge (alternate printing)
const HUSHWOOD_VERGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HUSHWOOD_VERGE,
    1,
    "2bc1b38f-11bf-4eb0-b333-4d605290e75b",
    "Henry Peters",
);

// DSK 333 — Thornspire Verge (alternate printing)
const THORNSPIRE_VERGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THORNSPIRE_VERGE,
    1,
    "a817f77d-19ea-4342-b90f-d92f62543303",
    "Pavel Kolomeyets",
);

// DSK 334 — Dazzling Theater // Prop Room (alternate printing)
const DAZZLING_THEATER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DAZZLING_THEATER,
    1,
    "988219dd-1b39-4c46-a3e3-844ca38e6e26",
    "Ivan Shavrin",
);

// DSK 335 — Dollmaker's Shop // Porcelain Gallery (alternate printing)
const DOLLMAKER_S_SHOP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DOLLMAKER_S_SHOP,
    1,
    "4b9225ce-f29d-4235-8e9d-98a4fd159f73",
    "Cacho Rubione",
);

// DSK 336 — Central Elevator // Promising Stairs (alternate printing)
const CENTRAL_ELEVATOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CENTRAL_ELEVATOR,
    1,
    "8634ba10-0312-4efe-9ddd-786c87d22131",
    "Ivan Shavrin",
);

// DSK 337 — Mirror Room // Fractured Realm (alternate printing)
const MIRROR_ROOM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MIRROR_ROOM,
    1,
    "f0c32b23-0586-4ac2-9304-d90b4636e7ae",
    "Cacho Rubione",
);

// DSK 338 — Funeral Room // Awakening Hall (alternate printing)
const FUNERAL_ROOM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FUNERAL_ROOM,
    1,
    "da30e940-acba-4f10-bd0e-3a0b165233f9",
    "Alexandre Chaudret",
);

// DSK 339 — Unholy Annex // Ritual Chamber (alternate printing)
const UNHOLY_ANNEX_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &UNHOLY_ANNEX,
    1,
    "681caa94-e95e-47f0-8305-eca0ed55cb5e",
    "Alexis Ziritt",
);

// DSK 340 — Charred Foyer // Warped Space (alternate printing)
const CHARRED_FOYER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CHARRED_FOYER,
    1,
    "c802966b-642c-4560-9c8e-e04adc9f41a4",
    "Oliver Barrett",
);

// DSK 341 — Walk-In Closet // Forgotten Cellar (alternate printing)
const WALK_IN_CLOSET_FORGOTTEN_CELLAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WALK_IN_CLOSET_FORGOTTEN_CELLAR,
    1,
    "c2e9868b-da2a-474d-8560-8bc4eb672cd3",
    "Bastien Grivet",
);

// DSK 342 — Restricted Office // Lecture Hall (alternate printing)
const RESTRICTED_OFFICE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RESTRICTED_OFFICE,
    1,
    "e35b3ab8-9230-469e-8c01-5f735fea5c9e",
    "Scott Buoncristiano",
);

// DSK 343 — Roaring Furnace // Steaming Sauna (alternate printing)
const ROARING_FURNACE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ROARING_FURNACE,
    1,
    "fe8e3d06-ae97-4f92-a379-b54e183e2aa0",
    "Toni Infante",
);

// DSK 344 — Abhorrent Oculus (alternate printing)
const ABHORRENT_OCULUS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ABHORRENT_OCULUS,
    1,
    "5f89402a-ef5c-47c1-8cef-f40920f98fac",
    "Igor Krstic",
);

// DSK 345 — Silent Hallcreeper (alternate printing)
const SILENT_HALLCREEPER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SILENT_HALLCREEPER,
    1,
    "a0ee370f-99d1-44bc-9952-29bcafbf7887",
    "Inkognit",
);

// DSK 346 — Doomsday Excruciator (alternate printing)
const DOOMSDAY_EXCRUCIATOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DOOMSDAY_EXCRUCIATOR,
    1,
    "50f9cecb-0ff7-4837-a8c5-4af394834e2c",
    "Jarel Threat",
);

// DSK 347 — Razorkin Needlehead (alternate printing)
const RAZORKIN_NEEDLEHEAD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RAZORKIN_NEEDLEHEAD,
    1,
    "25c34cb6-c8cb-4814-a647-9c63b10f02c4",
    "Loïc Canavaggia",
);

// DSK 348 — Screaming Nemesis
pub(in crate::card::sets) static SCREAMING_NEMESIS: CardRecord = CardRecord::new(
    "Screaming Nemesis",
    "ad3f4c72-ff6e-4d7f-8eb8-45a0a9605fc0",
    "Inkognit",
    // Three mana that attacks into anything: blocking it, burning it, or
    // fighting it all send the damage somewhere else, and a player who takes
    // it is out of lifegain for good.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Spirit"], 3, 3).with_abilities(&[
        abilities::haste(),
        AbilityDef::triggered_with_targets(
            "Whenever this creature is dealt damage, it deals that much damage to any other \
             target. If a player is dealt damage this way, they can't gain life for the rest of \
             the game.",
            TriggerEventDef::DamageDealt(DamageEventMatcherDef {
                kind: DamageKindDef::Any,
                source: DamageSourceMatcherDef::Any,
                recipient: DamageRecipientMatcherDef::MatchingObject(ObjectPredicateDef::Source),
            }),
            // "Any other target": everything an ordinary any-target slot offers, minus
            // the Spirit itself. Without the exclusion it could answer its own trigger
            // and hit itself, which would trigger it again.
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget).excluding_source()],
            // The damage and the rider are one effect rather than a sequence, because
            // the rider is about what actually took the damage: prevented damage stops
            // nothing from being gained.
            EffectDef::DealDamage(
                crate::card::DamageDef::new(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::TriggerEventAmount,
                )
                .with_follow_up(crate::card::DamageFollowUpDef::ApplyToDamaged {
                    effect: &AppliedEffectDef::Rule(AppliedRuleDef::CannotGainLife),
                    duration: ResolvedEffectDurationDef::Permanent,
                }),
            ),
        ),
    ]),
);

// DSK 349 — Hauntwoods Shrieker (alternate printing)
const HAUNTWOODS_SHRIEKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HAUNTWOODS_SHRIEKER,
    1,
    "0c6148f7-01cb-4789-87a6-53d741f44ec2",
    "Sidharth Chaturvedi",
);

// DSK 350 — Undead Sprinter (alternate printing)
const UNDEAD_SPRINTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &UNDEAD_SPRINTER,
    1,
    "ef349a98-d4df-4eab-81e3-96a74aff5902",
    "John Thacker",
);

// DSK 351 — The Wandering Rescuer (alternate printing)
const THE_WANDERING_RESCUER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_WANDERING_RESCUER,
    1,
    "42e3837d-0481-42cd-a7da-56b7e5cb1b7e",
    "Vance Kelly",
);

// DSK 352 — Valgavoth, Terror Eater (alternate printing)
const VALGAVOTH_TERROR_EATER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VALGAVOTH_TERROR_EATER,
    1,
    "9ee79357-2488-41a1-8f8c-dfcb77206f8a",
    "Pig Hands",
);

// DSK 353 — Tyvar, the Pummeler (alternate printing)
const TYVAR_THE_PUMMELER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TYVAR_THE_PUMMELER,
    1,
    "1c6f8dd7-6ec8-44b5-8827-192f255c4211",
    "Richard Luong",
);

// DSK 354 — Kaito, Bane of Nightmares (alternate printing)
const KAITO_BANE_OF_NIGHTMARES_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &KAITO_BANE_OF_NIGHTMARES,
    3,
    "a7927a69-62a9-4f69-923d-f651bb03a7b4",
    "Richard Luong",
);

// DSK 355 — Niko, Light of Hope (alternate printing)
const NIKO_LIGHT_OF_HOPE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &NIKO_LIGHT_OF_HOPE,
    1,
    "bb6f4e4e-8c3c-4860-bc9e-cbce44bcdc23",
    "Vance Kelly",
);

// DSK 356 — Toby, Beastie Befriender (alternate printing)
const TOBY_BEASTIE_BEFRIENDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TOBY_BEASTIE_BEFRIENDER,
    1,
    "cafe5755-88b0-4fee-8596-4232844f60fc",
    "Ashley Mackenzie",
);

// DSK 357 — The Mindskinner (alternate printing)
const THE_MINDSKINNER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_MINDSKINNER,
    1,
    "3130a342-3c98-4c69-975b-a958ccddfe37",
    "Goñi Montes",
);

// DSK 358 — Kona, Rescue Beastie (alternate printing)
const KONA_RESCUE_BEASTIE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &KONA_RESCUE_BEASTIE,
    2,
    "bf256f95-d184-4ea9-9039-fc64f461d9b3",
    "Sam Wolfe Connelly",
);

// DSK 359 — The Jolly Balloon Man (alternate printing)
const THE_JOLLY_BALLOON_MAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_JOLLY_BALLOON_MAN,
    1,
    "594df270-02d0-438b-a758-c32d71e8d4ff",
    "Richard Luong",
);

// DSK 360 — Marina Vendrell (alternate printing)
const MARINA_VENDRELL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MARINA_VENDRELL,
    1,
    "f6424845-7e9b-4514-93aa-1cf6c45f634c",
    "Jack Hughes",
);

// DSK 361 — Nashi, Searcher in the Dark (alternate printing)
const NASHI_SEARCHER_IN_THE_DARK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &NASHI_SEARCHER_IN_THE_DARK,
    1,
    "44aa4f0c-7b2b-44cc-a59f-95f276b25898",
    "Ashley Mackenzie",
);

// DSK 362 — Rip, Spawn Hunter (alternate printing)
const RIP_SPAWN_HUNTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RIP_SPAWN_HUNTER,
    1,
    "1ce8da0e-7822-4e8b-b073-5d4edcb38c59",
    "Ashley Mackenzie",
);

// DSK 363 — The Swarmweaver (alternate printing)
const THE_SWARMWEAVER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THE_SWARMWEAVER,
    2,
    "f4542e4f-2fb7-4036-aefc-291f5c010ca8",
    "Sam Wolfe Connelly",
);

// DSK 364 — Victor, Valgavoth's Seneschal (alternate printing)
const VICTOR_VALGAVOTH_S_SENESCHAL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VICTOR_VALGAVOTH_S_SENESCHAL,
    1,
    "1db4440e-0107-434d-9d8e-77a11bc0245a",
    "Ashley Mackenzie",
);

// DSK 365 — Winter, Misanthropic Guide (alternate printing)
const WINTER_MISANTHROPIC_GUIDE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WINTER_MISANTHROPIC_GUIDE,
    1,
    "7cf9f742-a662-46b3-84e1-1609177d0c8a",
    "Sam Wolfe Connelly",
);

// DSK 366 — Zimone, All-Questioning (alternate printing)
const ZIMONE_ALL_QUESTIONING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ZIMONE_ALL_QUESTIONING,
    1,
    "c8af5a2b-b87e-4ee2-88bd-f80670559875",
    "Sam Wolfe Connelly",
);

// DSK 367 — Marvin, Murderous Mimic (alternate printing)
const MARVIN_MURDEROUS_MIMIC_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MARVIN_MURDEROUS_MIMIC,
    1,
    "ed4f107a-ce90-464c-8be0-eb6c1db0f80f",
    "Sam Wolfe Connelly",
);

// DSK 368 — Enduring Innocence (alternate printing)
const ENDURING_INNOCENCE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ENDURING_INNOCENCE,
    1,
    "6d908299-aac0-46a6-8fa5-780d5b3e0386",
    "Liiga Smilshkalne",
);

// DSK 369 — Leyline of Hope (alternate printing)
const LEYLINE_OF_HOPE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LEYLINE_OF_HOPE,
    1,
    "e6209ca8-3a2a-4c0a-9f86-54283af9df75",
    "Sergey Glushakov",
);

// DSK 370 — Overlord of the Mistmoors (alternate printing)
const OVERLORD_OF_THE_MISTMOORS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &OVERLORD_OF_THE_MISTMOORS,
    2,
    "509735a5-2b41-4e52-9ab1-0e4a6fa41d1b",
    "Steven Belledin",
);

// DSK 371 — Enduring Curiosity (alternate printing)
const ENDURING_CURIOSITY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ENDURING_CURIOSITY,
    1,
    "ce740fb7-d2d1-451b-bfa0-27364b804161",
    "Julie Dillon",
);

// DSK 372 — Leyline of Transformation
pub(in crate::card::sets) static LEYLINE_OF_TRANSFORMATION: CardRecord = CardRecord::new(
    "Leyline of Transformation",
    "fd545d86-9a3e-4e4f-b0fe-9363a85b9290",
    "Sergey Glushakov",
CardRules::new_enchantment(mana_cost!("{2}{U}{U}")).with_abilities(&[
        abilities::begin_game_on_battlefield(),
        AbilityDef::replacement(
            "As this enchantment enters, choose a creature type.",
            ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(
                BattlefieldEntryScalarChoiceDef::CREATURE_TYPE,
            )),
        ),
        AbilityDef::static_ability(
            "Creatures you control are the chosen type in addition to their other types. The same is true for creature spells you control and creature cards you own that aren't on the battlefield.",
            EffectDef::StaticApply {
                // `matching` is deliberately zone-relative: controller on the
                // battlefield and stack, owner for cards everywhere else.
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[
                        ZoneKind::Battlefield,
                        ZoneKind::Stack,
                        ZoneKind::Library,
                        ZoneKind::Hand,
                        ZoneKind::Graveyard,
                        ZoneKind::Exile,
                        ZoneKind::Command,
                    ],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_chosen_creature_type(),
            },
        ),
    ]),
);

// DSK 373 — Overlord of the Floodpits (alternate printing)
const OVERLORD_OF_THE_FLOODPITS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OVERLORD_OF_THE_FLOODPITS,
    1,
    "5b0d446b-75ac-49d8-911a-73d17419ff05",
    "Abz J Harding",
);

// DSK 374 — Enduring Tenacity (alternate printing)
const ENDURING_TENACITY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ENDURING_TENACITY,
    1,
    "5d2d0033-1a73-4927-93a9-76d00ae7c6a8",
    "Isis",
);

// DSK 375 — Grievous Wound (alternate printing)
const GRIEVOUS_WOUND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GRIEVOUS_WOUND,
    1,
    "b3222c21-8d81-487d-a0c4-2413b30a26ab",
    "Martina Fačková",
);

// DSK 376 — Leyline of the Void (alternate printing)
const LEYLINE_OF_THE_VOID_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_gpt::LEYLINE_OF_THE_VOID,
    1,
    "b30a559e-79f9-479e-8ef1-65fb91b9507d",
    "Sergey Glushakov",
);

// DSK 377 — Overlord of the Balemurk (alternate printing)
const OVERLORD_OF_THE_BALEMURK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OVERLORD_OF_THE_BALEMURK,
    1,
    "6da8e15d-033e-41ab-9f95-656a5737bdac",
    "Babs Webb",
);

// DSK 378 — Enduring Courage (alternate printing)
const ENDURING_COURAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ENDURING_COURAGE,
    1,
    "98c10aa8-111a-4b02-a0a4-fee4c17f9c24",
    "Yigit Koroglu",
);

// DSK 379 — Leyline of Resonance (alternate printing)
const LEYLINE_OF_RESONANCE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LEYLINE_OF_RESONANCE,
    1,
    "c006baf9-451e-461c-9355-7bf5a4f0d601",
    "Sergey Glushakov",
);

// DSK 380 — Overlord of the Boilerbilges (alternate printing)
const OVERLORD_OF_THE_BOILERBILGES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OVERLORD_OF_THE_BOILERBILGES,
    1,
    "27266b27-3f16-4690-aaa0-07ea10b37bb6",
    "Helge C. Balzer",
);

// DSK 381 — Enduring Vitality (alternate printing)
const ENDURING_VITALITY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ENDURING_VITALITY,
    1,
    "400eaca3-d7ee-4334-bade-1b05f3591417",
    "Valera Lutfullina",
);

// DSK 382 — Leyline of Mutation (alternate printing)
const LEYLINE_OF_MUTATION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LEYLINE_OF_MUTATION,
    1,
    "cb5ad232-9863-454f-9455-9725e69fa33f",
    "Sergey Glushakov",
);

// DSK 383 — Overlord of the Hauntwoods (alternate printing)
const OVERLORD_OF_THE_HAUNTWOODS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OVERLORD_OF_THE_HAUNTWOODS,
    1,
    "3832d950-eb12-470f-9593-0f10d2a3a0f2",
    "Tiffany Turrill",
);

// DSK 384 — Twitching Doll (alternate printing)
const TWITCHING_DOLL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TWITCHING_DOLL,
    1,
    "0b47d003-bbdb-4c58-a648-3f0d7474f326",
    "Warren Mahy",
);

// DSK 385 — Dissection Tools (alternate printing)
const DISSECTION_TOOLS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DISSECTION_TOOLS,
    1,
    "0a376a56-7385-4d64-bbaa-952218531a76",
    "Diana Franco",
);

// DSK 386 — Enduring Innocence (alternate printing)
const ENDURING_INNOCENCE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ENDURING_INNOCENCE,
    2,
    "7aca1ee8-c1ae-4052-b9d0-1a2903f61c88",
    "Kawasumi",
);

// DSK 387 — Overlord of the Mistmoors
pub(in crate::card::sets) static OVERLORD_OF_THE_MISTMOORS: CardRecord = CardRecord::new(
    "Overlord of the Mistmoors",
    "1951ed76-16a1-4639-b824-08dfc3d6d098",
    "Takeuchi Moto",
    CardRules::new_enchantment_creature(mana_cost!("{5}{W}{W}"), &["Avatar", "Horror"], 6, 6)
        .with_abilities(&[
            AbilityDef::alternative_cast(
                &[CostDef::Mana(mana_cost!("{2}{W}{W}"))],
                AlternativeCastKindDef::Impending,
                Some(
                    "Impending 4—{2}{W}{W} (If you cast this spell for its \
                     impending cost, it enters with four time counters and isn't a \
                     creature until the last is removed. At the beginning of your \
                     end step, remove a time counter from it.)",
                ),
                EffectDef::None,
            ),
            AbilityDef::as_enters_if(
                "If you cast this spell for its impending cost, it enters with \
                 four time counters.",
                ReplacementConditionDef::SourceCastWith(AlternativeCastKindDef::Impending),
                ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::AddCounters {
                        kind: CounterKind::named("time"),
                        amount: 4,
                    },
                ),
            ),
            impending_countdown(),
            AbilityDef::triggered(
                "Whenever this permanent enters or attacks, create two 2/1 \
                 white Insect creature tokens with flying.",
                TriggerEventDef::AnyOf(&[
                    TriggerEventDef::zone_changed(
                        ObjectPredicateDef::Source,
                        None,
                        Some(ZoneKind::Battlefield),
                    ),
                    TriggerEventDef::attacks(ObjectPredicateDef::Source),
                ]),
                // Two at a time, which is one instruction rather than two: what watches
                // for tokens being created sees one batch of two.
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(
                        TokenCharacteristics::creature(&["Insect"], &[ManaColor::White], 2, 1)
                            .with_abilities(&[abilities::flying()]),
                    ))
                    .with_amount(2),
                ),
            ),
        ]),
);

// DSK 388 — Enduring Curiosity (alternate printing)
const ENDURING_CURIOSITY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ENDURING_CURIOSITY,
    2,
    "86b3b6b0-ad74-43d1-bf2d-fc830214bdb4",
    "D-suzuki",
);

// DSK 389 — Overlord of the Floodpits (alternate printing)
const OVERLORD_OF_THE_FLOODPITS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &OVERLORD_OF_THE_FLOODPITS,
    2,
    "3194a6f4-f994-494c-aa29-1de8af48a3f1",
    "Sansyu",
);

// DSK 390 — Enduring Tenacity (alternate printing)
const ENDURING_TENACITY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ENDURING_TENACITY,
    2,
    "e74a0fbc-7150-4c44-983f-ac31e74644fd",
    "TAPIOCA",
);

// DSK 391 — Overlord of the Balemurk (alternate printing)
const OVERLORD_OF_THE_BALEMURK_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &OVERLORD_OF_THE_BALEMURK,
    2,
    "49993a18-f733-4d1f-b629-12fc45cbb327",
    "Mikio Masuda",
);

// DSK 392 — Enduring Courage (alternate printing)
const ENDURING_COURAGE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ENDURING_COURAGE,
    2,
    "82c7fa72-3c5f-49db-95f0-9a1c5d404651",
    "Nuisuke",
);

// DSK 393 — Overlord of the Boilerbilges (alternate printing)
const OVERLORD_OF_THE_BOILERBILGES_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &OVERLORD_OF_THE_BOILERBILGES,
    2,
    "8607011b-0222-47c2-a537-b817dff93541",
    "akio",
);

// DSK 394 — Enduring Vitality (alternate printing)
const ENDURING_VITALITY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ENDURING_VITALITY,
    2,
    "2999c030-66c1-41f3-b59a-8ba1ef5a756c",
    "Issei Murakami",
);

// DSK 395 — Overlord of the Hauntwoods (alternate printing)
const OVERLORD_OF_THE_HAUNTWOODS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &OVERLORD_OF_THE_HAUNTWOODS,
    2,
    "5033440c-7d52-4f2c-bc62-ccd6d587d528",
    "Aogachou",
);

// DSK 396 — Enduring Innocence (alternate printing)
const ENDURING_INNOCENCE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &ENDURING_INNOCENCE,
    3,
    "dc36adbc-544c-46a1-9e99-92cc7b2b2af1",
    "Kawasumi",
);

// DSK 397 — Overlord of the Mistmoors (alternate printing)
const OVERLORD_OF_THE_MISTMOORS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &OVERLORD_OF_THE_MISTMOORS,
    3,
    "5b2c4873-dfa7-4dd8-aded-a5d1b6048ad7",
    "Takeuchi Moto",
);

// DSK 398 — Enduring Curiosity (alternate printing)
const ENDURING_CURIOSITY_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &ENDURING_CURIOSITY,
    3,
    "d1c10e8d-b393-41fd-ba31-5e39b2dc7cce",
    "D-suzuki",
);

// DSK 399 — Overlord of the Floodpits (alternate printing)
const OVERLORD_OF_THE_FLOODPITS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &OVERLORD_OF_THE_FLOODPITS,
    3,
    "bf42de20-43af-4c37-a5af-aa7c48699596",
    "Sansyu",
);

// DSK 400 — Enduring Tenacity (alternate printing)
const ENDURING_TENACITY_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &ENDURING_TENACITY,
    3,
    "59b698d6-4107-45d3-bc27-fb0746b7f91a",
    "TAPIOCA",
);

// DSK 401 — Overlord of the Balemurk (alternate printing)
const OVERLORD_OF_THE_BALEMURK_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &OVERLORD_OF_THE_BALEMURK,
    3,
    "49df3de4-ed62-4828-8256-a05220d9eada",
    "Mikio Masuda",
);

// DSK 402 — Enduring Courage (alternate printing)
const ENDURING_COURAGE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &ENDURING_COURAGE,
    3,
    "b07d4d21-2a88-4cfc-b0ce-9def76c20b7d",
    "Nuisuke",
);

// DSK 403 — Overlord of the Boilerbilges (alternate printing)
const OVERLORD_OF_THE_BOILERBILGES_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &OVERLORD_OF_THE_BOILERBILGES,
    3,
    "0e8c085e-db0e-45a8-96d0-80b480361771",
    "akio",
);

// DSK 404 — Enduring Vitality (alternate printing)
const ENDURING_VITALITY_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &ENDURING_VITALITY,
    3,
    "8a8c711f-f29f-4f6a-88c4-3dbf9ce471c3",
    "Issei Murakami",
);

// DSK 405 — Overlord of the Hauntwoods (alternate printing)
const OVERLORD_OF_THE_HAUNTWOODS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &OVERLORD_OF_THE_HAUNTWOODS,
    3,
    "f9f3ecff-2d67-4676-8706-9191d67215fc",
    "Aogachou",
);

// DSK 406 — The Wandering Rescuer (alternate printing)
const THE_WANDERING_RESCUER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THE_WANDERING_RESCUER,
    2,
    "a9a4f2ba-2d16-4dda-85e8-dafce0d5ee1b",
    "Vance Kelly",
);

// DSK 407 — Valgavoth, Terror Eater (alternate printing)
const VALGAVOTH_TERROR_EATER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &VALGAVOTH_TERROR_EATER,
    2,
    "497ec48f-5e6a-4f57-86ca-3640b7235002",
    "Pig Hands",
);

// DSK 408 — Tyvar, the Pummeler (alternate printing)
const TYVAR_THE_PUMMELER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &TYVAR_THE_PUMMELER,
    2,
    "bd10920a-25b2-4191-b7f2-c43dbd1c5cc5",
    "Richard Luong",
);

// DSK 409 — Kaito, Bane of Nightmares (alternate printing)
const KAITO_BANE_OF_NIGHTMARES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KAITO_BANE_OF_NIGHTMARES,
    1,
    "14901700-881a-4c79-b162-aeeb1579757e",
    "Richard Luong",
);

// DSK 410 — Niko, Light of Hope (alternate printing)
const NIKO_LIGHT_OF_HOPE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &NIKO_LIGHT_OF_HOPE,
    2,
    "a54c9d18-76d7-4c24-ab19-7ad57d9a33a4",
    "Vance Kelly",
);

// DSK 411 — Shardmage's Rescue (alternate printing)
const SHARDMAGE_S_RESCUE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SHARDMAGE_S_RESCUE,
    1,
    "05652e01-1144-4214-b962-c34f677e0a07",
    "Jarel Threat",
);

// DSK 412 — Valgavoth's Faithful (alternate printing)
const VALGAVOTH_S_FAITHFUL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VALGAVOTH_S_FAITHFUL,
    1,
    "25ef0c5b-4bcf-435f-b9bb-c8175c7b4257",
    "Jodie Muir",
);

// DSK 413 — Pyroclasm (alternate printing)
const PYROCLASM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ice::PYROCLASM,
    1,
    "f4005974-fac5-465f-b742-bcb41eb8ba74",
    "Néstor Ossandón Leal",
);

// DSK 414 — Drag to the Roots (alternate printing)
const DRAG_TO_THE_ROOTS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DRAG_TO_THE_ROOTS,
    1,
    "f764d201-b62e-4968-a1fc-a69247859854",
    "Deruchenko Alexander",
);

// DSK 415 — Inquisitive Glimmer (alternate printing)
const INQUISITIVE_GLIMMER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &INQUISITIVE_GLIMMER,
    1,
    "e8cc1465-ee00-40f4-aaa9-02f0465beb88",
    "Julie Dillon",
);

// DSK 416 — Grievous Wound (alternate printing)
const GRIEVOUS_WOUND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &GRIEVOUS_WOUND,
    2,
    "bea4b0aa-7ed8-4e1c-867b-50754447f41b",
    "Fajareka Setiawan",
);

// DSK 417 — Twitching Doll (alternate printing)
const TWITCHING_DOLL_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &TWITCHING_DOLL,
    2,
    "23684053-024e-4e15-8aec-de3d4bc7f126",
    "John Tedrick",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ACROBATIC_CHEERLEADER,
    &CULT_HEALER,
    &DAZZLING_THEATER,
    &DOLLMAKER_S_SHOP,
    &EMERGE_FROM_THE_COCOON,
    &ENDURING_INNOCENCE,
    &EXORCISE,
    &FEAR_OF_ABDUCTION,
    &FEAR_OF_IMMOBILITY,
    &FEAR_OF_SURVEILLANCE,
    &FRIENDLY_GHOST,
    &GHOSTLY_DANCERS,
    &GLIMMER_SEEKER,
    &GRAND_ENTRYWAY,
    &HARDENED_ESCORT,
    &JUMP_SCARE,
    &LEYLINE_OF_HOPE,
    &LIONHEART_GLIMMER,
    &LIVING_PHONE,
    &OPTIMISTIC_SCAVENGER,
    &ORPHANS_OF_THE_WHEAT,
    &PATCHED_PLAYTHING,
    &POSSESSED_GOAT,
    &RELUCTANT_ROLE_MODEL,
    &SAVIOR_OF_THE_SMALL,
    &SEIZED_FROM_SLUMBER,
    &SHARDMAGE_S_RESCUE,
    &SHELTERED_BY_GHOSTS,
    &SHEPHERDING_SPIRITS,
    &SPLIT_UP,
    &SPLITSKIN_DOLL,
    &SURGICAL_SUITE,
    &TOBY_BEASTIE_BEFRIENDER,
    &TRAPPED_IN_THE_SCREEN,
    &UNIDENTIFIED_HOVERSHIP,
    &UNSETTLING_TWINS,
    &UNWANTED_REMAKE,
    &VETERAN_SURVIVOR,
    &THE_WANDERING_RESCUER,
    &ABHORRENT_OCULUS,
    &BOTTOMLESS_POOL,
    &CENTRAL_ELEVATOR,
    &CLAMMY_PROWLER,
    &CREEPING_PEEPER,
    &CURSED_WINDBREAKER,
    &DAGGERMAW_MEGALODON,
    &DON_T_MAKE_A_SOUND,
    &DUSKMOURN_S_DOMINATION,
    &ENDURING_CURIOSITY,
    &ENTER_THE_ENIGMA,
    &ENTITY_TRACKER,
    &ERRATIC_APPARITION,
    &FEAR_OF_FAILED_TESTS,
    &FEAR_OF_FALLING,
    &FEAR_OF_IMPOSTORS,
    &FEAR_OF_ISOLATION,
    &FLOODPITS_DROWNER,
    &GET_OUT,
    &GHOSTLY_KEYBEARER,
    &GLIMMERBURST,
    &MARINA_VENDRELL_S_GRIMOIRE,
    &MEAT_LOCKER,
    &THE_MINDSKINNER,
    &MIRROR_ROOM,
    &OVERLORD_OF_THE_FLOODPITS,
    &PARANORMAL_ANALYST,
    &PIRANHA_FLY,
    &SCRABBLING_SKULLCRAB,
    &SILENT_HALLCREEPER,
    &STALKED_RESEARCHER,
    &STAY_HIDDEN_STAY_SILENT,
    &THE_TALE_OF_TAMIYO,
    &TUNNEL_SURVEYOR,
    &TWIST_REALITY,
    &UNABLE_TO_SCREAM,
    &UNDERWATER_TUNNEL,
    &UNNERVING_GRASP,
    &UNWILLING_VESSEL,
    &VANISH_FROM_SIGHT,
    &APPENDAGE_AMALGAM,
    &BALEMURK_LEECH,
    &CACKLING_SLASHER,
    &COME_BACK_WRONG,
    &COMMUNE_WITH_EVIL,
    &CRACKED_SKULL,
    &CYNICAL_LONER,
    &DASHING_BLOODSUCKER,
    &DEFILED_CRYPT,
    &DEMONIC_COUNSEL,
    &DERELICT_ATTIC,
    &DOOMSDAY_EXCRUCIATOR,
    &ENDURING_TENACITY,
    &FANATIC_OF_THE_HARROWING,
    &FEAR_OF_LOST_TEETH,
    &FEAR_OF_THE_DARK,
    &FINAL_VENGEANCE,
    &FUNERAL_ROOM,
    &GIVE_IN_TO_VIOLENCE,
    &GRIEVOUS_WOUND,
    &INNOCUOUS_RAT,
    &KILLER_S_MASK,
    &LET_S_PLAY_A_GAME,
    &LIVE_OR_DIE,
    &MEATHOOK_MASSACRE_II,
    &MIASMA_DEMON,
    &NOWHERE_TO_RUN,
    &OSSEOUS_STICKTWISTER,
    &OVERLORD_OF_THE_BALEMURK,
    &POPULAR_EGOTIST,
    &RESURRECTED_CULTIST,
    &SPECTRAL_SNATCHER,
    &SPOROGENIC_INFECTION,
    &UNHOLY_ANNEX,
    &UNSTOPPABLE_SLASHER,
    &VALGAVOTH_TERROR_EATER,
    &VALGAVOTH_S_FAITHFUL,
    &VILE_MUTILATOR,
    &WINTER_S_INTERVENTION,
    &WITHERING_TORMENT,
    &BEDHEAD_BEASTIE,
    &BETRAYER_S_BARGAIN,
    &BOILERBILGES_RIPPER,
    &CHARRED_FOYER,
    &CURSED_RECORDING,
    &DIVERSION_SPECIALIST,
    &ENDURING_COURAGE,
    &FEAR_OF_BEING_HUNTED,
    &FEAR_OF_BURNING_ALIVE,
    &FEAR_OF_MISSING_OUT,
    &GLASSWORKS,
    &GRAB_THE_PRIZE,
    &HAND_THAT_FEEDS,
    &IMPOSSIBLE_INFERNO,
    &INFERNAL_PHANTOM,
    &IRREVERENT_GREMLIN,
    &LEYLINE_OF_RESONANCE,
    &MOST_VALUABLE_SLAYER,
    &NORIN_SWIFT_SURVIVALIST,
    &OVERLORD_OF_THE_BOILERBILGES,
    &PAINTER_S_STUDIO,
    &PIGGY_BANK,
    &RAGGED_PLAYMATE,
    &RAMPAGING_SOULRAGER,
    &RAZORKIN_HORDECALLER,
    &RAZORKIN_NEEDLEHEAD,
    &RIPCHAIN_RAZORKIN,
    &THE_ROLLERCRUSHER_RIDE,
    &TICKET_BOOTH,
    &TRIAL_OF_AGONY,
    &TURN_INSIDE_OUT,
    &UNTIMELY_MALFUNCTION,
    &VENGEFUL_POSSESSION,
    &VICIOUS_CLOWN,
    &VIOLENT_URGE,
    &WALTZ_OF_RAGE,
    &ALTANAK_THE_THRICE_CALLED,
    &ANTHROPEDE,
    &BALUSTRADE_WURM,
    &BASHFUL_BEASTIE,
    &BREAK_DOWN_THE_DOOR,
    &CATHARTIC_PARTING,
    &CAUTIOUS_SURVIVOR,
    &COORDINATED_CLOBBERING,
    &CRYPTID_INSPECTOR,
    &DEFIANT_SURVIVOR,
    &ENDURING_VITALITY,
    &FEAR_OF_EXPOSURE,
    &FLESH_BURROWER,
    &FRANTIC_STRENGTH,
    &GRASPING_LONGNECK,
    &GREENHOUSE,
    &HAUNTWOODS_SHRIEKER,
    &HEDGE_SHREDDER,
    &HORRID_VIGOR,
    &HOUSE_CARTOGRAPHER,
    &INSIDIOUS_FUNGUS,
    &KONA_RESCUE_BEASTIE,
    &LEYLINE_OF_MUTATION,
    &MANIFEST_DREAD,
    &MOLDERING_GYM,
    &MONSTROUS_EMERGENCE,
    &OMNIVOROUS_FLYTRAP,
    &OVERGROWN_ZEALOT,
    &OVERLORD_OF_THE_HAUNTWOODS,
    &PATCHWORK_BEASTIE,
    &ROOTWISE_SURVIVOR,
    &SAY_ITS_NAME,
    &SLAVERING_BRANCHSNAPPER,
    &SPINESEEKER_CENTIPEDE,
    &THREATS_AROUND_EVERY_CORNER,
    &TWITCHING_DOLL,
    &TYVAR_THE_PUMMELER,
    &UNDER_THE_SKIN,
    &VALGAVOTH_S_ONSLAUGHT,
    &WALK_IN_CLOSET_FORGOTTEN_CELLAR,
    &WARY_WATCHDOG,
    &WICKERFOLK_THRESHER,
    &ARABELLA_ABANDONED_DOLL,
    &BASEBALL_BAT,
    &BEASTIE_BEATDOWN,
    &BROODSPINNER,
    &DISTURBING_MIRTH,
    &DRAG_TO_THE_ROOTS,
    &FEAR_OF_INFINITY,
    &GREMLIN_TAMER,
    &GROWING_DREAD,
    &INQUISITIVE_GLIMMER,
    &INTRUDING_SOULRAGER,
    &THE_JOLLY_BALLOON_MAN,
    &KAITO_BANE_OF_NIGHTMARES,
    &MARINA_VENDRELL,
    &MIDNIGHT_MAYHEM,
    &NASHI_SEARCHER_IN_THE_DARK,
    &NIKO_LIGHT_OF_HOPE,
    &OBLIVIOUS_BOOKWORM,
    &PEER_PAST_THE_VEIL,
    &RESTRICTED_OFFICE,
    &RIP_SPAWN_HUNTER,
    &RITE_OF_THE_MOTH,
    &ROARING_FURNACE,
    &SAWBLADE_SKINRIPPER,
    &SHREWD_STORYTELLER,
    &SHROUDSTOMPER,
    &SKULLSNAP_NUISANCE,
    &SMOKY_LOUNGE,
    &THE_SWARMWEAVER,
    &UNDEAD_SPRINTER,
    &VICTOR_VALGAVOTH_S_SENESCHAL,
    &WILDFIRE_WICKERFOLK,
    &WINTER_MISANTHROPIC_GUIDE,
    &ZIMONE_ALL_QUESTIONING,
    &ATTACK_IN_THE_BOX,
    &BEAR_TRAP,
    &CONDUCTIVE_MACHETE,
    &DISSECTION_TOOLS,
    &FOUND_FOOTAGE,
    &FRIENDLY_TEDDY,
    &GHOST_VACUUM,
    &GLIMMERLIGHT,
    &HAUNTED_SCREEN,
    &KEYS_TO_THE_HOUSE,
    &MALEVOLENT_CHANDELIER,
    &MARVIN_MURDEROUS_MIMIC,
    &SAW,
    &ABANDONED_CAMPGROUND,
    &BLAZEMIRE_VERGE,
    &BLEEDING_WOODS,
    &ETCHED_CORNFIELD,
    &FLOODFARM_VERGE,
    &GLOOMLAKE_VERGE,
    &HUSHWOOD_VERGE,
    &LAKESIDE_SHACK,
    &MURKY_SEWER,
    &NEGLECTED_MANOR,
    &PECULIAR_LIGHTHOUSE,
    &RAUCOUS_CARNIVAL,
    &RAZORTRAP_GORGE,
    &STRANGLED_CEMETERY,
    &THORNSPIRE_VERGE,
    &VALGAVOTH_S_LAIR,
    &CLOCKWORK_PERCUSSIONIST,
    &CHAINSAW,
    &SCREAMING_NEMESIS,
    &LEYLINE_OF_TRANSFORMATION,
    &OVERLORD_OF_THE_MISTMOORS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    ETHEREAL_ARMOR_REPRINT,
    OVERLORD_OF_THE_MISTMOORS_ALTERNATE_1,
    LEYLINE_OF_TRANSFORMATION_ALTERNATE_1,
    LEYLINE_OF_THE_VOID_REPRINT,
    MURDER_REPRINT,
    CHAINSAW_ALTERNATE_1,
    CLOCKWORK_PERCUSSIONIST_ALTERNATE_1,
    PYROCLASM_REPRINT,
    SCORCHING_DRAGONFIRE_REPRINT,
    SCREAMING_NEMESIS_ALTERNATE_1,
    TERRAMORPHIC_EXPANSE_REPRINT,
    PLAINS_REPRINT,
    ISLAND_REPRINT,
    SWAMP_REPRINT,
    MOUNTAIN_REPRINT,
    FOREST_REPRINT,
    PLAINS_ALTERNATE_1,
    PLAINS_ALTERNATE_2,
    ISLAND_ALTERNATE_1,
    ISLAND_ALTERNATE_2,
    SWAMP_ALTERNATE_1,
    SWAMP_ALTERNATE_2,
    MOUNTAIN_ALTERNATE_1,
    MOUNTAIN_ALTERNATE_2,
    FOREST_ALTERNATE_1,
    FOREST_ALTERNATE_2,
    GRAND_ENTRYWAY_ALTERNATE_1,
    OPTIMISTIC_SCAVENGER_ALTERNATE_1,
    RELUCTANT_ROLE_MODEL_ALTERNATE_1,
    ENTITY_TRACKER_ALTERNATE_1,
    STAY_HIDDEN_STAY_SILENT_ALTERNATE_1,
    COME_BACK_WRONG_ALTERNATE_1,
    MEATHOOK_MASSACRE_II_ALTERNATE_1,
    UNSTOPPABLE_SLASHER_ALTERNATE_1,
    CURSED_RECORDING_ALTERNATE_1,
    NORIN_SWIFT_SURVIVALIST_ALTERNATE_1,
    THE_ROLLERCRUSHER_RIDE_ALTERNATE_1,
    KONA_RESCUE_BEASTIE_ALTERNATE_1,
    OBLIVIOUS_BOOKWORM_ALTERNATE_1,
    THE_SWARMWEAVER_ALTERNATE_1,
    GHOSTLY_DANCERS_ALTERNATE_1,
    RELUCTANT_ROLE_MODEL_ALTERNATE_2,
    SPLIT_UP_ALTERNATE_1,
    UNIDENTIFIED_HOVERSHIP_ALTERNATE_1,
    UNWANTED_REMAKE_ALTERNATE_1,
    ENTITY_TRACKER_ALTERNATE_2,
    MARINA_VENDRELL_S_GRIMOIRE_ALTERNATE_1,
    COME_BACK_WRONG_ALTERNATE_2,
    DEMONIC_COUNSEL_ALTERNATE_1,
    MEATHOOK_MASSACRE_II_ALTERNATE_2,
    UNSTOPPABLE_SLASHER_ALTERNATE_2,
    WITHERING_TORMENT_ALTERNATE_1,
    CURSED_RECORDING_ALTERNATE_2,
    FEAR_OF_MISSING_OUT_ALTERNATE_1,
    THE_ROLLERCRUSHER_RIDE_ALTERNATE_2,
    WALTZ_OF_RAGE_ALTERNATE_1,
    BALUSTRADE_WURM_ALTERNATE_1,
    HEDGE_SHREDDER_ALTERNATE_1,
    INSIDIOUS_FUNGUS_ALTERNATE_1,
    OMNIVOROUS_FLYTRAP_ALTERNATE_1,
    UNDER_THE_SKIN_ALTERNATE_1,
    VALGAVOTH_S_ONSLAUGHT_ALTERNATE_1,
    PEER_PAST_THE_VEIL_ALTERNATE_1,
    GHOST_VACUUM_ALTERNATE_1,
    VALGAVOTH_S_LAIR_ALTERNATE_1,
    KAITO_BANE_OF_NIGHTMARES_ALTERNATE_2,
    BLAZEMIRE_VERGE_ALTERNATE_1,
    FLOODFARM_VERGE_ALTERNATE_1,
    GLOOMLAKE_VERGE_ALTERNATE_1,
    HUSHWOOD_VERGE_ALTERNATE_1,
    THORNSPIRE_VERGE_ALTERNATE_1,
    DAZZLING_THEATER_ALTERNATE_1,
    DOLLMAKER_S_SHOP_ALTERNATE_1,
    CENTRAL_ELEVATOR_ALTERNATE_1,
    MIRROR_ROOM_ALTERNATE_1,
    FUNERAL_ROOM_ALTERNATE_1,
    UNHOLY_ANNEX_ALTERNATE_1,
    CHARRED_FOYER_ALTERNATE_1,
    WALK_IN_CLOSET_FORGOTTEN_CELLAR_ALTERNATE_1,
    RESTRICTED_OFFICE_ALTERNATE_1,
    ROARING_FURNACE_ALTERNATE_1,
    ABHORRENT_OCULUS_ALTERNATE_1,
    SILENT_HALLCREEPER_ALTERNATE_1,
    DOOMSDAY_EXCRUCIATOR_ALTERNATE_1,
    RAZORKIN_NEEDLEHEAD_ALTERNATE_1,
    HAUNTWOODS_SHRIEKER_ALTERNATE_1,
    UNDEAD_SPRINTER_ALTERNATE_1,
    THE_WANDERING_RESCUER_ALTERNATE_1,
    VALGAVOTH_TERROR_EATER_ALTERNATE_1,
    TYVAR_THE_PUMMELER_ALTERNATE_1,
    KAITO_BANE_OF_NIGHTMARES_ALTERNATE_3,
    NIKO_LIGHT_OF_HOPE_ALTERNATE_1,
    TOBY_BEASTIE_BEFRIENDER_ALTERNATE_1,
    THE_MINDSKINNER_ALTERNATE_1,
    KONA_RESCUE_BEASTIE_ALTERNATE_2,
    THE_JOLLY_BALLOON_MAN_ALTERNATE_1,
    MARINA_VENDRELL_ALTERNATE_1,
    NASHI_SEARCHER_IN_THE_DARK_ALTERNATE_1,
    RIP_SPAWN_HUNTER_ALTERNATE_1,
    THE_SWARMWEAVER_ALTERNATE_2,
    VICTOR_VALGAVOTH_S_SENESCHAL_ALTERNATE_1,
    WINTER_MISANTHROPIC_GUIDE_ALTERNATE_1,
    ZIMONE_ALL_QUESTIONING_ALTERNATE_1,
    MARVIN_MURDEROUS_MIMIC_ALTERNATE_1,
    ENDURING_INNOCENCE_ALTERNATE_1,
    LEYLINE_OF_HOPE_ALTERNATE_1,
    OVERLORD_OF_THE_MISTMOORS_ALTERNATE_2,
    ENDURING_CURIOSITY_ALTERNATE_1,
    OVERLORD_OF_THE_FLOODPITS_ALTERNATE_1,
    ENDURING_TENACITY_ALTERNATE_1,
    GRIEVOUS_WOUND_ALTERNATE_1,
    LEYLINE_OF_THE_VOID_ALTERNATE_1,
    OVERLORD_OF_THE_BALEMURK_ALTERNATE_1,
    ENDURING_COURAGE_ALTERNATE_1,
    LEYLINE_OF_RESONANCE_ALTERNATE_1,
    OVERLORD_OF_THE_BOILERBILGES_ALTERNATE_1,
    ENDURING_VITALITY_ALTERNATE_1,
    LEYLINE_OF_MUTATION_ALTERNATE_1,
    OVERLORD_OF_THE_HAUNTWOODS_ALTERNATE_1,
    TWITCHING_DOLL_ALTERNATE_1,
    DISSECTION_TOOLS_ALTERNATE_1,
    ENDURING_INNOCENCE_ALTERNATE_2,
    ENDURING_CURIOSITY_ALTERNATE_2,
    OVERLORD_OF_THE_FLOODPITS_ALTERNATE_2,
    ENDURING_TENACITY_ALTERNATE_2,
    OVERLORD_OF_THE_BALEMURK_ALTERNATE_2,
    ENDURING_COURAGE_ALTERNATE_2,
    OVERLORD_OF_THE_BOILERBILGES_ALTERNATE_2,
    ENDURING_VITALITY_ALTERNATE_2,
    OVERLORD_OF_THE_HAUNTWOODS_ALTERNATE_2,
    ENDURING_INNOCENCE_ALTERNATE_3,
    OVERLORD_OF_THE_MISTMOORS_ALTERNATE_3,
    ENDURING_CURIOSITY_ALTERNATE_3,
    OVERLORD_OF_THE_FLOODPITS_ALTERNATE_3,
    ENDURING_TENACITY_ALTERNATE_3,
    OVERLORD_OF_THE_BALEMURK_ALTERNATE_3,
    ENDURING_COURAGE_ALTERNATE_3,
    OVERLORD_OF_THE_BOILERBILGES_ALTERNATE_3,
    ENDURING_VITALITY_ALTERNATE_3,
    OVERLORD_OF_THE_HAUNTWOODS_ALTERNATE_3,
    THE_WANDERING_RESCUER_ALTERNATE_2,
    VALGAVOTH_TERROR_EATER_ALTERNATE_2,
    TYVAR_THE_PUMMELER_ALTERNATE_2,
    KAITO_BANE_OF_NIGHTMARES_ALTERNATE_1,
    NIKO_LIGHT_OF_HOPE_ALTERNATE_2,
    SHARDMAGE_S_RESCUE_ALTERNATE_1,
    VALGAVOTH_S_FAITHFUL_ALTERNATE_1,
    PYROCLASM_ALTERNATE_1,
    DRAG_TO_THE_ROOTS_ALTERNATE_1,
    INQUISITIVE_GLIMMER_ALTERNATE_1,
    GRIEVOUS_WOUND_ALTERNATE_2,
    TWITCHING_DOLL_ALTERNATE_2,
];
