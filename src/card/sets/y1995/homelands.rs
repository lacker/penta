//! HML card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityPredicateDef, AbilityTargetDef, AbilityTargetPredicate, ActivationTimingDef,
    AddManaEffectDef, AppliedEffectDef, AppliedRuleDef, BasicLandType,
    BattlefieldEntryChoiceDestinationDef, BattlefieldEntryModificationDef,
    BattlefieldEntryScalarChoiceDef, BlockRestrictionDef, BlockRestrictionMatchDef,
    BlockRestrictionSubjectDef, CardArt, CardChoiceSourceDef, CardRules, CardSet, CardSupertype,
    CardType, ComparisonDef, CostDef, CostQuantityDef, CounterKind, DiscardFollowUpDef,
    DiscardSelectionDef, EffectDef, EffectRecipientDef, InstalledTriggerDef, KeywordAbility,
    ManaColor, MoveToZoneCostDef, ObjectCollectionSourceDef, ObjectPredicateDef, ObjectQueryDef,
    ObjectRefDef, ObjectSetCountConditionDef, ObjectSetDef, ObjectSetPredicateDef, PayOrDef,
    PlayActionMatcherDef, PlayRestrictionDef, PlayerRefDef, PlayerRelation, PlayerSetDef,
    ReplacementChoiceDef, ReplacementEffectDef, ResolvedEffectDurationDef,
    RevealAndClassifyCardsDef, SacrificedAmountDef, SumValueDef, TriggerConditionDef,
    TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::{ParentBinding, TargetIndex, mana_cost};

// HML 1 — Abbey Gargoyles
pub(in crate::card::sets) static ABBEY_GARGOYLES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("71a5d8de-25f1-4070-a7a6-dc3f2339ce30"),
    "Abbey Gargoyles",
    crate::card::CardArt::new("71a5d8de-25f1-4070-a7a6-dc3f2339ce30", "Christopher Rush"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{2}{W}{W}{W}"), &["Gargoyle"], 3, 4).with_abilities(&[
        abilities::flying(),
        abilities::protection_from_color(ManaColor::Red),
    ]),
);

// HML 2a — Abbey Matron
pub(in crate::card::sets) static ABBEY_MATRON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("158caa84-da2e-4c4c-b24d-0c035c900e20"),
    "Abbey Matron",
    crate::card::CardArt::new("158caa84-da2e-4c4c-b24d-0c035c900e20", "Mike Kimble"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Cleric"], 1, 3).with_ability(
        AbilityDef::activated(
            "{W}, {T}: This creature gets +0/+3 until end of turn.",
            &[CostDef::Mana(mana_cost!("{W}")), CostDef::TapSource],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(3),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// HML 2b — Abbey Matron (alternate printing)

// HML 3a — Aysen Bureaucrats (alternate printing)

// HML 3b — Aysen Bureaucrats
pub(in crate::card::sets) static AYSEN_BUREAUCRATS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7ca3fa70-50b3-4157-afda-fe58bf72ee16"),
    "Aysen Bureaucrats",
    crate::card::CardArt::new("7ca3fa70-50b3-4157-afda-fe58bf72ee16", "Alan Rabinowitz"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Advisor"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Tap target creature with power 2 or less.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::PowerAtLeast(3)),
                ]),
            )],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ),
);

// HML 4 — Aysen Crusader
pub(in crate::card::sets) static AYSEN_CRUSADER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7908cfdc-5ed6-48a8-a5b9-351864f8b4fd"),
    "Aysen Crusader",
    crate::card::CardArt::new("7908cfdc-5ed6-48a8-a5b9-351864f8b4fd", "NéNé Thomas"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{2}{W}{W}"), &["Human", "Knight"], 2, 2)
        .with_ability(AbilityDef::static_ability(
            "Aysen Crusader's power and toughness are each equal to 2 plus the number of Soldiers and Warriors you control.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::Subtype("Soldier"),
                                ObjectPredicateDef::Subtype("Warrior"),
                            ]),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::Subtype("Soldier"),
                                ObjectPredicateDef::Subtype("Warrior"),
                            ]),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                ),
            },
        )),
);

// HML 5 — Aysen Highway
pub(in crate::card::sets) static AYSEN_HIGHWAY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("adfa87eb-9a11-459c-bff8-87bb09b61b87"),
    "Aysen Highway",
    crate::card::CardArt::new("adfa87eb-9a11-459c-bff8-87bb09b61b87", "NéNé Thomas"),
    crate::card::CardSet::Homelands,
    CardRules::new_enchantment(mana_cost!("{3}{W}{W}{W}")).with_ability(
        AbilityDef::static_ability(
            "White creatures have plainswalk.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Color(ManaColor::White),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::landwalk(BasicLandType::Plains)),
            },
        ),
    ),
);

// HML 6 — Beast Walkers
pub(in crate::card::sets) static BEAST_WALKERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("99b42f6c-5c7e-4ba8-b0fb-ac8564aaf825"),
    "Beast Walkers",
    crate::card::CardArt::new("99b42f6c-5c7e-4ba8-b0fb-ac8564aaf825", "Heather Hudson"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(
        mana_cost!("{1}{W}{W}"),
        &["Human", "Beast", "Soldier"],
        2,
        2,
    )
    .with_ability(abilities::gain_ability_until_end_of_turn_for_mana(
        "{G}: This creature gains banding until end of turn.",
        mana_cost!("{G}"),
        &abilities::banding(),
    )),
);

// HML 7 — Death Speakers
pub(in crate::card::sets) static DEATH_SPEAKERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e17c19a4-0186-45a0-89b9-d7b0fb0ddd8a"),
    "Death Speakers",
    crate::card::CardArt::new("e17c19a4-0186-45a0-89b9-d7b0fb0ddd8a", "Douglas Shuler"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Cleric"], 1, 1)
        .with_ability(abilities::protection_from_color(ManaColor::Black)),
);

// HML 8 — Hazduhr the Abbot
// Audit: unsupported — Needs an amount-limited damage-redirection shield from a targeted creature to the source; the existing redirection rule redirects every matching damage event for its duration.
pub(in crate::card::sets) static HAZDUHR_THE_ABBOT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("adfd416a-dddf-40e4-acf0-84057edb7a58"),
    "Hazduhr the Abbot",
    crate::card::CardArt::new("adfd416a-dddf-40e4-acf0-84057edb7a58", "Dan Frazier"),
    crate::card::CardSet::Homelands,
    crate::card::CardRules::unsupported(),
);

// HML 9 — Leeches
// Audit: unsupported — Needs removing counters from a player and carrying the number actually removed into a damage value.
pub(in crate::card::sets) static LEECHES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("90db206e-b254-476c-b2f3-1cd56bb5297d"),
    "Leeches",
    crate::card::CardArt::new("90db206e-b254-476c-b2f3-1cd56bb5297d", "Alan Rabinowitz"),
    crate::card::CardSet::Homelands,
    crate::card::CardRules::unsupported(),
);

// HML 10a — Mesa Falcon (alternate printing)

// HML 10b — Mesa Falcon
pub(in crate::card::sets) static MESA_FALCON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("04d2f5e2-fb95-48b0-b7bf-689d45fa8970"),
    "Mesa Falcon",
    crate::card::CardArt::new("04d2f5e2-fb95-48b0-b7bf-689d45fa8970", "Mark Poole"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Bird"], 1, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{1}{W}: This creature gets +0/+1 until end of turn.",
            &[CostDef::Mana(mana_cost!("{1}{W}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// HML 11 — Prophecy
pub(in crate::card::sets) static PROPHECY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f514eb63-3a4e-4410-ba3d-487cf81f7063"),
    "Prophecy",
    crate::card::CardArt::new("f514eb63-3a4e-4410-ba3d-487cf81f7063", "Christopher Rush"),
    crate::card::CardSet::Homelands,
    CardRules::new_sorcery(mana_cost!("{W}")).with_ability(AbilityDef::spell_with_targets(
        "Reveal the top card of target opponent's library. If it's a land, you gain 1 life. Then that player shuffles.\nDraw a card at the beginning of the next turn's upkeep.",
        &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
            PlayerRelation::Opponent,
        ))],
        EffectDef::RevealAndClassifyCards(RevealAndClassifyCardsDef {
            source: ObjectCollectionSourceDef::TopCards {
                player: PlayerRefDef::Target(TargetIndex::PRIMARY),
                count: ValueDef::Constant(1),
            },
            object: ObjectPredicateDef::HasType(CardType::Land),
            matching: Binding!("prophecy_land"),
            remainder: Binding!("prophecy_nonland"),
            then: &EffectDef::Sequence(&[
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ObjectSetCount(
                        &ObjectSetCountConditionDef {
                            objects: &ObjectSetDef::Binding(Binding!("prophecy_land")),
                            predicate: ObjectSetPredicateDef {
                                filter: None,
                                comparison: ComparisonDef::GreaterOrEqual,
                                amount: 1,
                            },
                        },
                    ),
                    then: &EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                },
                EffectDef::ShuffleLibrary {
                    player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
                EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                    "At the beginning of the next turn's upkeep, draw a card.",
                    TriggerEventDef::StepBegins {
                        step: TurnStepDef::Upkeep,
                        player: PlayerRelation::Any,
                    },
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                ))),
            ]),
        }),
    )),
);

// HML 12 — Rashka the Slayer
// Audit: unsupported — Needs a batched blocking-declaration event so blocking one or several black creatures creates exactly one trigger; the current block event fires once per blocking relationship.
pub(in crate::card::sets) static RASHKA_THE_SLAYER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ddf30363-9db2-44c5-8c13-dbf1aaa8c86b"),
    "Rashka the Slayer",
    crate::card::CardArt::new("ddf30363-9db2-44c5-8c13-dbf1aaa8c86b", "Christopher Rush"),
    crate::card::CardSet::Homelands,
    crate::card::CardRules::unsupported(),
);

// HML 13a — Samite Alchemist (alternate printing)

// HML 13b — Samite Alchemist
pub(in crate::card::sets) static SAMITE_ALCHEMIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0545fc43-9c67-4ad4-b1d9-6b57b53321af"),
    "Samite Alchemist",
    crate::card::CardArt::new("0545fc43-9c67-4ad4-b1d9-6b57b53321af", "Tom Wänerstrand"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Cleric"], 0, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{W}{W}, {T}: Prevent the next 4 damage that would be dealt this turn to target creature you control. Tap that creature. It doesn't untap during your next untap step.",
            &[
                CostDef::Mana(mana_cost!("{W}{W}")),
                CostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            })],
            EffectDef::Sequence(&[
                EffectDef::PreventDamage {
                    prevention: crate::card::DamagePreventionDef::amount(
                        crate::card::DamageEventMatcherDef::to(EffectRecipientDef::Target(
                            TargetIndex::PRIMARY,
                        )),
                        ValueDef::Constant(4),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Tap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
                EffectDef::SkipNextUntapSteps {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    count: 1,
                },
            ]),
        ),
    ),
);

// HML 14 — Serra Aviary
pub(in crate::card::sets) static SERRA_AVIARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("688a8de2-0167-4b35-a38d-3574034a892c"),
    "Serra Aviary",
    crate::card::CardArt::new("688a8de2-0167-4b35-a38d-3574034a892c", "Nicola Leonard"),
    crate::card::CardSet::Homelands,
    CardRules::new_enchantment(mana_cost!("{3}{W}"))
        .with_supertype(CardSupertype::World)
        .with_ability(AbilityDef::static_ability(
            "Creatures with flying get +1/+1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
            },
        )),
);

// HML 15 — Serra Bestiary
// Audit: unsupported — Needs an activation prohibition that selects only abilities with {T} in their costs; the current rule can prohibit all activated abilities but cannot inspect their costs.
pub(in crate::card::sets) static SERRA_BESTIARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aab1c8b8-9b3e-444a-a12c-bd09ec899641"),
    "Serra Bestiary",
    crate::card::CardArt::new("aab1c8b8-9b3e-444a-a12c-bd09ec899641", "Anson Maddocks"),
    crate::card::CardSet::Homelands,
    crate::card::CardRules::unsupported(),
);

// HML 16 — Serra Inquisitors
// Audit: unsupported — Needs a batched blocking-declaration event so one or several black creatures create exactly one trigger; the current relationship event fires once per creature.
pub(in crate::card::sets) static SERRA_INQUISITORS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f1fbe3c8-92fb-41b9-b778-726d22c63054"),
    "Serra Inquisitors",
    crate::card::CardArt::new("f1fbe3c8-92fb-41b9-b778-726d22c63054", "Dennis Detwiller"),
    crate::card::CardSet::Homelands,
    crate::card::CardRules::unsupported(),
);

// HML 17 — Serra Paladin
pub(in crate::card::sets) static SERRA_PALADIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7bd25adf-4d97-4229-abdb-1c060036cfbd"),
    "Serra Paladin",
    crate::card::CardArt::new("7bd25adf-4d97-4229-abdb-1c060036cfbd", "Pete Venters"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{2}{W}{W}"), &["Human", "Knight"], 2, 2).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}: Prevent the next 1 damage that would be dealt to any target this turn.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::PreventDamage {
                prevention: crate::card::DamagePreventionDef::amount(
                    crate::card::DamageEventMatcherDef::to(EffectRecipientDef::Target(
                        TargetIndex::PRIMARY,
                    )),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated_with_targets(
            "{1}{W}{W}, {T}: Target creature gains vigilance until end of turn.",
            &[CostDef::Mana(mana_cost!("{1}{W}{W}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::vigilance()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// HML 18 — Soraya the Falconer
pub(in crate::card::sets) static SORAYA_THE_FALCONER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("19fb3ce2-a660-4829-9af4-330cfd612f06"),
    "Soraya the Falconer",
    crate::card::CardArt::new("19fb3ce2-a660-4829-9af4-330cfd612f06", "Dennis Detwiller"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Human"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Bird creatures get +1/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype("Bird"),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                },
            ),
            AbilityDef::activated_with_targets(
                "{1}{W}: Target Bird creature gains banding until end of turn.",
                &[CostDef::Mana(mana_cost!("{1}{W}"))],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Bird"),
                    ]),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&abilities::banding()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// HML 19a — Trade Caravan (alternate printing)

// HML 19b — Trade Caravan
// Audit: unsupported — Needs an opponent-upkeep-only activation window; activation timing currently distinguishes your upkeep from any upkeep, but not only an opponent's.
pub(in crate::card::sets) static TRADE_CARAVAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e60ddb1e-e607-4080-849c-3e1a79052729"),
    "Trade Caravan",
    crate::card::CardArt::new("e60ddb1e-e607-4080-849c-3e1a79052729", "Kaja Foglio"),
    crate::card::CardSet::Homelands,
    crate::card::CardRules::unsupported(),
);

// HML 20 — Truce
// Audit: unsupported — Needs each player to independently choose zero, one, or two draws and gain life from that player's shortfall; resolution choices currently select for only one player.
pub(in crate::card::sets) static TRUCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("35c5fd74-bd46-4833-ae25-1a11a8c15ed2"),
    "Truce",
    crate::card::CardArt::new("35c5fd74-bd46-4833-ae25-1a11a8c15ed2", "Melissa A. Benson"),
    crate::card::CardSet::Homelands,
    crate::card::CardRules::unsupported(),
);

// HML 21 — Aether Storm
pub(in crate::card::sets) static AETHER_STORM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ce479e91-7b21-4312-a3c0-950d9f6dc029"),
    "Aether Storm",
    crate::card::CardArt::new("ce479e91-7b21-4312-a3c0-950d9f6dc029", "Mark Tedin"),
    crate::card::CardSet::Homelands,
    CardRules::new_enchantment(mana_cost!("{3}{U}")).with_abilities(&[
        AbilityDef::static_ability(
            "Creature spells can't be cast.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::EachPlayer,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(
                    PlayRestrictionDef::new(
                        PlayActionMatcherDef::CastSpell,
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ),
                )),
            },
        ),
        AbilityDef::activated(
            "Pay 4 life: Destroy this enchantment. It can't be regenerated. Any player may activate this ability.",
            &[CostDef::PayLife(4)],
            EffectDef::WithRule {
                rule: AppliedRuleDef::CannotRegenerate,
                effect: &EffectDef::Destroy {
                    object: EffectRecipientDef::Source,
                    then: None,
                },
            },
        )
        .open_to_any_player(),
    ]),
);

// HML 22 — Baki's Curse
// Audit: unsupported — Needs a per-recipient value counting Auras attached to each creature; attachment counts currently address players, not an arbitrary affected permanent.
pub(in crate::card::sets) static BAKI_S_CURSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e3261b4c-7963-4ca0-875d-77b7c8571b3f"),
    "Baki's Curse",
    crate::card::CardArt::new("e3261b4c-7963-4ca0-875d-77b7c8571b3f", "Nicola Leonard"),
    crate::card::CardSet::Homelands,
    crate::card::CardRules::unsupported(),
);

// HML 23 — Chain Stasis
// Audit: unsupported — Needs a target's controller to pay during resolution to copy the resolving spell and choose a new target, including repeating that process for each copy.
pub(in crate::card::sets) static CHAIN_STASIS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f14f0c52-67c2-4302-82bd-fbb4e3c6d4f4"),
    "Chain Stasis",
    crate::card::CardArt::new("f14f0c52-67c2-4302-82bd-fbb4e3c6d4f4", "Pat Lewis"),
    crate::card::CardSet::Homelands,
    crate::card::CardRules::unsupported(),
);

// HML 24 — Coral Reef
pub(in crate::card::sets) static CORAL_REEF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("42fe2280-a996-4072-b5bf-f4fd56607a51"),
    "Coral Reef",
    crate::card::CardArt::new("42fe2280-a996-4072-b5bf-f4fd56607a51", "Amy Weber"),
    crate::card::CardSet::Homelands,
    CardRules::new_enchantment(mana_cost!("{U}{U}")).with_abilities(&[
        AbilityDef::as_enters(
            "This enchantment enters with four polyp counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::named("polyp"),
                    amount: 4,
                },
            ),
        ),
        AbilityDef::activated(
            "Sacrifice an Island: Put two polyp counters on this enchantment.",
            &[CostDef::SacrificePermanent {
                object: ObjectPredicateDef::HasAnyBasicLandType(&[
                    crate::card::BasicLandType::Island,
                ]),
                controller: PlayerRelation::You,
            }],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::named("polyp"),
                amount: ValueDef::Constant(2),
            },
        ),
        AbilityDef::activated_with_targets(
            "{U}, Tap an untapped blue creature you control, Remove a polyp counter from this enchantment: Put a +0/+1 counter on target creature.",
            &[
                CostDef::Mana(mana_cost!("{U}")),
                CostDef::TapPermanents {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Color(ManaColor::Blue),
                    ]),
                    controller: PlayerRelation::You,
                    count: 1,
                },
                CostDef::RemoveCountersFromSource {
                    kind: CounterKind::named("polyp"),
                    amount: 1,
                },
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::power_toughness(0, 1),
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// HML 25a — Dark Maze
pub(in crate::card::sets) static DARK_MAZE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c20a8ed9-db1e-4ce8-bfb3-92604a577df7"),
    "Dark Maze",
    crate::card::CardArt::new("c20a8ed9-db1e-4ce8-bfb3-92604a577df7", "Rob Alexander"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Wall"], 4, 5).with_abilities(&[
        abilities::defender(),
        AbilityDef::activated(
            "{0}: This creature can attack this turn as though it didn't have defender. Exile it at the beginning of the next end step.",
            &[CostDef::Mana(mana_cost!("{0}"))],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::MayAttackDespiteDefender),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                    "At the beginning of the next end step, exile this creature.",
                    TriggerEventDef::StepBegins {
                        step: TurnStepDef::End,
                        player: PlayerRelation::Any,
                    },
                    EffectDef::MoveToZone {
                        object: EffectRecipientDef::Source,
                        zone: ZoneKind::Exile,
                        placement: ZonePlacement::Top,
                    },
                ))),
            ]),
        ),
    ]),
);

// HML 25b — Dark Maze (alternate printing)

// HML 26 — Forget
pub(in crate::card::sets) static FORGET: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("df3115a9-ad65-4213-9320-6f39c11676f3"),
    "Forget",
    crate::card::CardArt::new("df3115a9-ad65-4213-9320-6f39c11676f3", "Mike Kimble"),
    crate::card::CardSet::Homelands,
    CardRules::new_sorcery(mana_cost!("{U}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Target player discards two cards, then draws as many cards as they discarded this way.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Discard {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(2),
            selection: DiscardSelectionDef::RecipientChooses,
            then: Some(DiscardFollowUpDef {
                counted: ObjectPredicateDef::Any,
                bound: Some(ParentBinding),
                effect: &EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::BoundObjectCount(ParentBinding),
                },
            }),
        },
    )),
);

// HML 27a — Giant Albatross
// Audit: unsupported — Needs the exact set of creatures that dealt damage to the source this turn, followed by a separate unless-life-paid choice for each creature's controller.
pub(in crate::card::sets) static GIANT_ALBATROSS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bce05870-74d3-43f1-92d0-dc1744c0138d"),
    "Giant Albatross",
    crate::card::CardArt::new("bce05870-74d3-43f1-92d0-dc1744c0138d", "David A. Cherry"),
    crate::card::CardSet::Homelands,
    crate::card::CardRules::unsupported(),
);

// HML 27b — Giant Albatross (alternate printing)

// HML 28 — Giant Oyster
// Audit: unsupported — Needs a target-bound recurring draw-step effect whose untap restriction and counters persist only while the source remains tapped, plus cleanup when that duration ends.
pub(in crate::card::sets) static GIANT_OYSTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f8045d23-e6e6-474c-a3e7-ddfc6121657a"),
    "Giant Oyster",
    crate::card::CardArt::new("f8045d23-e6e6-474c-a3e7-ddfc6121657a", "Nicola Leonard"),
    crate::card::CardSet::Homelands,
    crate::card::CardRules::unsupported(),
);

// HML 29 — Jinx
// Audit: unsupported — Needs a resolution-time choice of one basic land type that feeds a temporary type-setting effect; current land-type choices cover as-enters choices or two-type substitutions.
pub(in crate::card::sets) static JINX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f81fca41-2315-4d12-b05c-d921a4c3c19e"),
    "Jinx",
    crate::card::CardArt::new("f81fca41-2315-4d12-b05c-d921a4c3c19e", "Mike Kimble"),
    crate::card::CardSet::Homelands,
    crate::card::CardRules::unsupported(),
);

// HML 30a — Labyrinth Minotaur (alternate printing)

// HML 30b — Labyrinth Minotaur
pub(in crate::card::sets) static LABYRINTH_MINOTAUR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0663c756-9db9-4298-8a6e-a1af935286a0"),
    "Labyrinth Minotaur",
    crate::card::CardArt::new("0663c756-9db9-4298-8a6e-a1af935286a0", "Anson Maddocks"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Minotaur"], 1, 4).with_ability(
        AbilityDef::triggered(
            "Whenever this creature blocks a creature, that creature doesn't untap during its controller's next untap step.",
            TriggerEventDef::Blocks {
                blocked: ObjectPredicateDef::HasType(CardType::Creature),
            },
            EffectDef::SkipNextUntapSteps {
                object: EffectRecipientDef::TriggeringObject,
                count: 1,
            },
        ),
    ),
);

// HML 31 — Marjhan
pub(in crate::card::sets) static MARJHAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b6aa3299-3b7a-4ea5-bc1f-beead26d8116"),
    "Marjhan",
    crate::card::CardArt::new("b6aa3299-3b7a-4ea5-bc1f-beead26d8116", "Daniel Gelon"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{5}{U}{U}"), &["Serpent"], 8, 8).with_abilities(&[
        AbilityDef::static_ability(
            "This creature doesn't untap during your untap step.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
            },
        ),
        AbilityDef::activated(
            "{U}{U}, Sacrifice a creature: Untap this creature. Activate only during your upkeep.",
            &[
                CostDef::Mana(mana_cost!("{U}{U}")),
                CostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    controller: PlayerRelation::You,
                },
            ],
            EffectDef::Untap {
                object: EffectRecipientDef::Source,
            },
        )
        .with_activation_timing(ActivationTimingDef::YourUpkeep),
        AbilityDef::static_ability(
            "This creature can't attack unless defending player controls an Island.",
            EffectDef::CannotAttackUnless(&ObjectQueryDef::matching(
                ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Opponent,
            )),
        ),
        AbilityDef::activated_with_targets(
            "{U}{U}: This creature gets -1/-0 until end of turn and deals 1 damage to target attacking creature without flying.",
            &[CostDef::Mana(mana_cost!("{U}{U}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Attacking,
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasKeyword(
                        KeywordAbility::Flying,
                    )),
                ]),
            )],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-1),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
        AbilityDef::triggered_if(
            "When you control no Islands, sacrifice this creature.",
            TriggerEventDef::StateCondition,
            &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::Equal,
                amount: 0,
            },
            EffectDef::Sacrifice {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// HML 32a — Memory Lapse
pub(in crate::card::sets) static MEMORY_LAPSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3d2cc591-3a81-468a-91a4-3c3aac83a21a"),
    "Memory Lapse",
    CardArt::new("3d2cc591-3a81-468a-91a4-3c3aac83a21a", "Mark Tedin"),
    CardSet::Homelands,
    // Two mana that buys a turn rather than a card, which in a deck built to
    // use the turn is the better half of the trade.
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target spell. If that spell is countered this way, put it on top of its owner's \
         library instead of into that player's graveyard.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Spell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        // "Put it on top of its owner's library instead of into that player's
        // graveyard": the counter still happens, and what changes is only where
        // the card lands afterwards.
        EffectDef::Counter {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Library,
            placement: ZonePlacement::Top,
        },
    )),
);

// HML 32b — Memory Lapse (alternate printing)

// HML 33 — Merchant Scroll
pub(in crate::card::sets) static MERCHANT_SCROLL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d4133ceb-6176-411a-9eb8-51721c1bb435"),
    "Merchant Scroll",
    crate::card::CardArt::new("d4133ceb-6176-411a-9eb8-51721c1bb435", "Liz Danforth"),
    crate::card::CardSet::Homelands,
    CardRules::new_sorcery(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell(
        "Search your library for a blue instant card, reveal that card, put it into your hand, then shuffle.",
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Instant),
                ObjectPredicateDef::Color(ManaColor::Blue),
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
    )),
);

// HML 34 — Mystic Decree
pub(in crate::card::sets) static MYSTIC_DECREE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8b069e6a-2c0e-4fc9-8e19-08bf1245a6c0"),
    "Mystic Decree",
    crate::card::CardArt::new("8b069e6a-2c0e-4fc9-8e19-08bf1245a6c0", "Liz Danforth"),
    crate::card::CardSet::Homelands,
    CardRules::new_enchantment(mana_cost!("{2}{U}{U}"))
        .with_supertype(CardSupertype::World)
        .with_ability(AbilityDef::static_ability(
            "All creatures lose flying and islandwalk.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::remove_abilities(AbilityPredicateDef::Keyword(
                        KeywordAbility::Flying,
                    )),
                    AppliedEffectDef::remove_abilities(AbilityPredicateDef::Keyword(
                        KeywordAbility::Landwalk(BasicLandType::Island),
                    )),
                ]),
            },
        )),
);

// HML 35 — Narwhal
pub(in crate::card::sets) static NARWHAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("202d3ed5-f493-43b6-bf36-81ad289e6fb0"),
    "Narwhal",
    crate::card::CardArt::new("202d3ed5-f493-43b6-bf36-81ad289e6fb0", "David A. Cherry"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Whale"], 2, 2).with_abilities(&[
        abilities::first_strike(),
        abilities::protection_from_color(ManaColor::Red),
    ]),
);

// HML 36a — Reef Pirates
pub(in crate::card::sets) static REEF_PIRATES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5b742d75-860d-4b90-89cb-4292f18aed39"),
    "Reef Pirates",
    crate::card::CardArt::new("5b742d75-860d-4b90-89cb-4292f18aed39", "Tom Wänerstrand"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{1}{U}{U}"), &["Zombie", "Pirate"], 2, 2).with_ability(
        AbilityDef::triggered(
            "Whenever this creature deals damage to an opponent, that player mills a card.",
            TriggerEventDef::damage_to_player(ObjectPredicateDef::Source, PlayerRelation::Opponent),
            EffectDef::Mill {
                player: EffectRecipientDef::EventPlayer,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// HML 36b — Reef Pirates (alternate printing)

// HML 37 — Reveka, Wizard Savant
pub(in crate::card::sets) static REVEKA_WIZARD_SAVANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a952236e-3085-4e6e-8639-355976b7c8f5"),
    "Reveka, Wizard Savant",
    crate::card::CardArt::new("a952236e-3085-4e6e-8639-355976b7c8f5", "Susan Van Camp"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Dwarf", "Wizard"], 0, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::activated_with_targets(
            "{T}: Reveka deals 2 damage to any target and doesn't untap during your next untap step.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget)],
            EffectDef::Sequence(&[
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(2),
                },
                EffectDef::SkipNextUntapSteps {
                    object: EffectRecipientDef::Source,
                    count: 1,
                },
            ]),
        )),
);

// HML 38 — Sea Sprite
pub(in crate::card::sets) static SEA_SPRITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("88fb001b-3afb-44f5-ab78-af2bf9a4e63a"),
    "Sea Sprite",
    crate::card::CardArt::new("88fb001b-3afb-44f5-ab78-af2bf9a4e63a", "Susan Van Camp"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Faerie"], 1, 1).with_abilities(&[
        abilities::flying(),
        abilities::protection_from_color(ManaColor::Red),
    ]),
);

// HML 39 — Sea Troll
// Audit: unsupported — Needs an activation condition that remembers whether the source blocked or was blocked by a blue creature this turn.
pub(in crate::card::sets) static SEA_TROLL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b7da23d4-f9fb-40a5-8395-51b47a064600"),
    "Sea Troll",
    crate::card::CardArt::new("b7da23d4-f9fb-40a5-8395-51b47a064600", "Daniel Gelon"),
    crate::card::CardSet::Homelands,
    crate::card::CardRules::unsupported(),
);

// HML 40 — Wall of Kelp
pub(in crate::card::sets) static WALL_OF_KELP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("52ff5051-e24b-4453-aaae-ed4f2bf213ab"),
    "Wall of Kelp",
    crate::card::CardArt::new("52ff5051-e24b-4453-aaae-ed4f2bf213ab", "Alan Rabinowitz"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{U}{U}"), &["Plant", "Wall"], 0, 3).with_abilities(&[
        abilities::defender(),
        AbilityDef::activated(
            "{U}{U}, {T}: Create a 0/1 blue Plant Wall creature token with defender named Kelp.",
            &[CostDef::Mana(mana_cost!("{U}{U}")), CostDef::TapSource],
            EffectDef::create_creature_token(&["Plant", "Wall"], &[ManaColor::Blue], 0, 1)
                .with_name("Kelp")
                .with_abilities(&[abilities::defender()]),
        ),
    ]),
);

// HML 41 — Baron Sengir
pub(in crate::card::sets) static BARON_SENGIR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("51bdddac-02fc-493a-a0ea-689273252d7e"),
    "Baron Sengir",
    crate::card::CardArt::new("51bdddac-02fc-493a-a0ea-689273252d7e", "Pete Venters"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{5}{B}{B}{B}"), &["Vampire", "Noble"], 5, 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            abilities::creature_damaged_by_source_dies_trigger(
                "Whenever a creature dealt damage by Baron Sengir this turn dies, put a +2/+2 counter on Baron Sengir.",
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::power_toughness(2, 2),
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::activated_with_targets(
                "{T}: Regenerate another target Vampire.",
                &[CostDef::TapSource],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Subtype("Vampire"),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                )],
                EffectDef::Regenerate {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
        ]),
);

// HML 42 — Black Carriage
pub(in crate::card::sets) static BLACK_CARRIAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("87068116-6000-44ee-b47f-f5cb8c233bb2"),
    "Black Carriage",
    crate::card::CardArt::new("87068116-6000-44ee-b47f-f5cb8c233bb2", "David A. Cherry"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Horse"], 4, 4).with_abilities(&[
        abilities::trample(),
        AbilityDef::static_ability(
            "This creature doesn't untap during your untap step.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
            },
        ),
        AbilityDef::activated(
            "Sacrifice a creature: Untap this creature. Activate only during your upkeep.",
            &[CostDef::SacrificePermanent {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                controller: PlayerRelation::You,
            }],
            EffectDef::Untap {
                object: EffectRecipientDef::Source,
            },
        )
        .with_activation_timing(ActivationTimingDef::YourUpkeep),
    ]),
);

// HML 43 — Broken Visage
// Audit: unsupported — Needs a created token whose base power and toughness are dynamic values read from a destroyed target's last-known information; token base stats are currently fixed integers.
pub(in crate::card::sets) static BROKEN_VISAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9be199e7-feaa-4f23-b93c-3eab54a02e74"),
    "Broken Visage",
    crate::card::CardArt::new("9be199e7-feaa-4f23-b93c-3eab54a02e74", "Mike Kimble"),
    crate::card::CardSet::Homelands,
    crate::card::CardRules::unsupported(),
);

// HML 44a — Cemetery Gate
pub(in crate::card::sets) static CEMETERY_GATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0c6f0614-06dc-4bd2-b8b9-d951ae27db21"),
    "Cemetery Gate",
    crate::card::CardArt::new("0c6f0614-06dc-4bd2-b8b9-d951ae27db21", "Melissa A. Benson"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Wall"], 0, 5).with_abilities(&[
        abilities::defender(),
        abilities::protection_from_color(ManaColor::Black),
    ]),
);

// HML 44b — Cemetery Gate (alternate printing)

// HML 45 — Drudge Spell
pub(in crate::card::sets) static DRUDGE_SPELL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("52b352de-e989-4ad5-963c-818092fc9f1a"),
    "Drudge Spell",
    crate::card::CardArt::new("52b352de-e989-4ad5-963c-818092fc9f1a", "NéNé Thomas"),
    crate::card::CardSet::Homelands,
    CardRules::new_enchantment(mana_cost!("{B}{B}")).with_abilities(&[
        AbilityDef::activated(
            "{B}, Exile two creature cards from your graveyard: Create a 1/1 black Skeleton creature token. It has \"{B}: Regenerate this token.\"",
            &[
                CostDef::Mana(mana_cost!("{B}")),
                CostDef::MoveToZone(MoveToZoneCostDef::new(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ZoneKind::Graveyard,
                    ZoneKind::Exile,
                    2,
                )),
            ],
            EffectDef::create_creature_token(&["Skeleton"], &[ManaColor::Black], 1, 1)
                .with_abilities(&[AbilityDef::activated(
                    "{B}: Regenerate this token.",
                    &[CostDef::Mana(mana_cost!("{B}"))],
                    EffectDef::Regenerate {
                        object: EffectRecipientDef::Source,
                    },
                )]),
        ),
        AbilityDef::triggered(
            "When this enchantment leaves the battlefield, destroy all Skeleton tokens. They can't be regenerated.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                None,
            ),
            EffectDef::WithRule {
                rule: AppliedRuleDef::CannotRegenerate,
                effect: &EffectDef::Destroy {
                    object: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Subtype("Skeleton"),
                            ObjectPredicateDef::Token,
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    then: None,
                },
            },
        ),
    ]),
);

// HML 46a — Dry Spell
pub(in crate::card::sets) static DRY_SPELL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("547c10ea-8ace-4496-8b99-61863c0cec1b"),
    "Dry Spell",
    crate::card::CardArt::new("547c10ea-8ace-4496-8b99-61863c0cec1b", "Brian Snõddy"),
    crate::card::CardSet::Homelands,
    CardRules::new_sorcery(mana_cost!("{1}{B}")).with_ability(AbilityDef::spell(
        "Dry Spell deals 1 damage to each creature and each player.",
        EffectDef::Sequence(&[
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                amount: ValueDef::Constant(1),
            },
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::players(PlayerSetDef::All),
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// HML 46b — Dry Spell (alternate printing)

// HML 47a — Feast of the Unicorn
pub(in crate::card::sets) static FEAST_OF_THE_UNICORN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("096e41d6-79c3-463f-ae63-872c3d8729a7"),
    "Feast of the Unicorn",
    crate::card::CardArt::new("096e41d6-79c3-463f-ae63-872c3d8729a7", "Dennis Detwiller"),
    crate::card::CardSet::Homelands,
    CardRules::new_enchantment(mana_cost!("{3}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +4/+0.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(4),
                        ValueDef::Constant(0),
                    ),
                },
            ),
        ]),
);

// HML 47b — Feast of the Unicorn (alternate printing)

// HML 48 — Funeral March
pub(in crate::card::sets) static FUNERAL_MARCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("054c5678-63d1-45d9-bd51-43100fd10afd"),
    "Funeral March",
    crate::card::CardArt::new("054c5678-63d1-45d9-bd51-43100fd10afd", "Melissa A. Benson"),
    crate::card::CardSet::Homelands,
    CardRules::new_enchantment(mana_cost!("{1}{B}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::triggered(
                "When enchanted creature leaves the battlefield, its controller sacrifices a creature of their choice.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::AttachedToSource,
                    Some(ZoneKind::Battlefield),
                    None,
                ),
                EffectDef::SacrificeOfChoice {
                    player: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                        ObjectRefDef::TriggeringObject,
                    )),
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    count: ValueDef::Constant(1),
                    then: None,
                    amount: SacrificedAmountDef::Power,
                    otherwise: None,
                    optional: false,
                },
            ),
        ]),
);

// HML 49 — Ghost Hounds
pub(in crate::card::sets) static GHOST_HOUNDS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d1298b43-0b10-4b7c-9d33-786d4d7bd80e"),
    "Ghost Hounds",
    crate::card::CardArt::new("d1298b43-0b10-4b7c-9d33-786d4d7bd80e", "Jeff A. Menges"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Dog", "Spirit"], 1, 1).with_abilities(&[
        abilities::vigilance(),
        AbilityDef::triggered(
            "Whenever this creature blocks or becomes blocked by a white creature, this creature gains first strike until end of turn.",
            TriggerEventDef::BlocksOrBecomesBlockedBy {
                creature: ObjectPredicateDef::Source,
                other: ObjectPredicateDef::Color(ManaColor::White),
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// HML 50 — Grandmother Sengir
pub(in crate::card::sets) static GRANDMOTHER_SENGIR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("efb0ac91-5e8e-47b1-aa34-902eef60349f"),
    "Grandmother Sengir",
    crate::card::CardArt::new("efb0ac91-5e8e-47b1-aa34-902eef60349f", "Pete Venters"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Human", "Wizard"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::activated_with_targets(
            "{1}{B}, {T}: Target creature gets -1/-1 until end of turn.",
            &[CostDef::Mana(mana_cost!("{1}{B}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-1),
                    ValueDef::Constant(-1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )),
);

// HML 51 — Greater Werewolf
pub(in crate::card::sets) static GREATER_WEREWOLF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8c29c45f-db1a-43e3-ae42-1a72dabe7880"),
    "Greater Werewolf",
    crate::card::CardArt::new("8c29c45f-db1a-43e3-ae42-1a72dabe7880", "Dennis Detwiller"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Werewolf"], 2, 4).with_ability(
        AbilityDef::triggered(
            "At end of combat, put a -0/-2 counter on each creature blocking or blocked by this creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::EndOfCombat,
                player: PlayerRelation::Any,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::BlockedBySource,
                        ObjectPredicateDef::BlockingSource,
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                kind: CounterKind::power_toughness(0, -2),
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// HML 52 — Headstone
pub(in crate::card::sets) static HEADSTONE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2fdae7fa-1076-4ff3-b771-fc3f5d9ba89f"),
    "Headstone",
    crate::card::CardArt::new("2fdae7fa-1076-4ff3-b771-fc3f5d9ba89f", "David A. Cherry"),
    crate::card::CardSet::Homelands,
    CardRules::new_instant(mana_cost!("{1}{B}")).with_ability(
        AbilityDef::spell_with_targets(
            "Exile target card from a graveyard.\nDraw a card at the beginning of the next turn's upkeep.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Any,
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: None,
            })],
            EffectDef::Sequence(&[
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Exile,
                    placement: ZonePlacement::Top,
                },
                EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                    "At the beginning of the next turn's upkeep, draw a card.",
                    TriggerEventDef::StepBegins {
                        step: TurnStepDef::Upkeep,
                        player: PlayerRelation::Any,
                    },
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                ))),
            ]),
        ),
    ),
);

// HML 53 — Ihsan's Shade
pub(in crate::card::sets) static IHSAN_S_SHADE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("82351724-2814-4d9e-b065-bb72c761b2e7"),
    "Ihsan's Shade",
    crate::card::CardArt::new("82351724-2814-4d9e-b065-bb72c761b2e7", "Christopher Rush"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{3}{B}{B}{B}"), &["Shade", "Knight"], 5, 5)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(abilities::protection_from_color(ManaColor::White)),
);

// HML 54 — Irini Sengir
pub(in crate::card::sets) static IRINI_SENGIR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("518e3b77-d482-4b90-94c0-0b8cdd949b9f"),
    "Irini Sengir",
    crate::card::CardArt::new("518e3b77-d482-4b90-94c0-0b8cdd949b9f", "Pete Venters"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Vampire", "Dwarf"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(abilities::spell_cost_increase(
            "Green enchantment spells and white enchantment spells cost {2} more to cast.",
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Enchantment),
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Color(ManaColor::Green),
                    ObjectPredicateDef::Color(ManaColor::White),
                ]),
            ]),
            PlayerRelation::Any,
            mana_cost!("{2}"),
        )),
);

// HML 55 — Koskun Falls
// Audit: unsupported — The attack tax is supported, but the upkeep clause needs tapping a chosen untapped creature as an effect payment, which resolution payments cannot express.
pub(in crate::card::sets) static KOSKUN_FALLS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("04292a4e-8910-4911-a76d-4f2c3e15da33"),
    "Koskun Falls",
    crate::card::CardArt::new("04292a4e-8910-4911-a76d-4f2c3e15da33", "Rob Alexander"),
    crate::card::CardSet::Homelands,
    crate::card::CardRules::unsupported(),
);

// HML 56 — Sengir Autocrat
pub(in crate::card::sets) static SENGIR_AUTOCRAT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0d16e024-7865-43d0-8cd8-8933ef741d05"),
    "Sengir Autocrat",
    crate::card::CardArt::new("0d16e024-7865-43d0-8cd8-8933ef741d05", "David A. Cherry"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Human"], 2, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, create three 0/1 black Serf creature tokens.",
            EffectDef::create_creature_token(&["Serf"], &[ManaColor::Black], 0, 1).with_amount(3),
        ),
        AbilityDef::triggered(
            "When this creature leaves the battlefield, exile all Serf tokens.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                None,
            ),
            EffectDef::MoveToZone {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Token,
                        ObjectPredicateDef::Subtype("Serf"),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
            },
        ),
    ]),
);

// HML 57a — Sengir Bats (alternate printing)

// HML 57b — Sengir Bats
pub(in crate::card::sets) static SENGIR_BATS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2ddb981a-d9e3-4efe-a383-5c98ee3b0b84"),
    "Sengir Bats",
    crate::card::CardArt::new("2ddb981a-d9e3-4efe-a383-5c98ee3b0b84", "Dan Frazier"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Bat"], 1, 2).with_abilities(&[
        abilities::flying(),
        abilities::creature_damaged_by_source_dies_trigger(
            "Whenever a creature dealt damage by this creature this turn dies, put a +1/+1 counter on this creature.",
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// HML 58 — Timmerian Fiends
// Audit: unsupported — Ante cards require ante-zone ownership exchange and legality rules, which the engine does not model.
pub(in crate::card::sets) static TIMMERIAN_FIENDS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("90643766-c92f-4a25-bd02-227f3c91f391"),
    "Timmerian Fiends",
    crate::card::CardArt::new("90643766-c92f-4a25-bd02-227f3c91f391", "Mike Kimble"),
    crate::card::CardSet::Homelands,
    crate::card::CardRules::unsupported(),
);

// HML 59a — Torture
pub(in crate::card::sets) static TORTURE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2cb8cc6c-7e24-4629-a9ce-5f717f236c37"),
    "Torture",
    crate::card::CardArt::new("2cb8cc6c-7e24-4629-a9ce-5f717f236c37", "Mark Tedin"),
    crate::card::CardSet::Homelands,
    CardRules::new_enchantment(mana_cost!("{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::activated(
                "{1}{B}: Put a -1/-1 counter on enchanted creature.",
                &[CostDef::Mana(mana_cost!("{1}{B}"))],
                EffectDef::AddCounters {
                    object: EffectRecipientDef::AttachedPermanent,
                    kind: CounterKind::MinusOneMinusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// HML 59b — Torture (alternate printing)

// HML 60 — Veldrane of Sengir
pub(in crate::card::sets) static VELDRANE_OF_SENGIR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fe0ce7d7-d370-4ef8-b1fa-aa70b2fd5ab1"),
    "Veldrane of Sengir",
    crate::card::CardArt::new("fe0ce7d7-d370-4ef8-b1fa-aa70b2fd5ab1", "Susan Van Camp"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{5}{B}{B}"), &["Human", "Rogue"], 5, 5)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::activated(
            "{1}{B}{B}: Veldrane gets -3/-0 and gains forestwalk until end of turn.",
            &[CostDef::Mana(mana_cost!("{1}{B}{B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-3),
                        ValueDef::Constant(0),
                    ),
                    AppliedEffectDef::add_ability(&abilities::forestwalk()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )),
);

// HML 61a — Aliban's Tower
pub(in crate::card::sets) static ALIBAN_S_TOWER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7f711ea8-a73a-42da-8bf7-101ba588f203"),
    "Aliban's Tower",
    crate::card::CardArt::new("7f711ea8-a73a-42da-8bf7-101ba588f203", "Jeff A. Menges"),
    crate::card::CardSet::Homelands,
    CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Target blocking creature gets +3/+1 until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::Blocking,
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(3),
                ValueDef::Constant(1),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// HML 61b — Aliban's Tower (alternate printing)

// HML 62 — Ambush
pub(in crate::card::sets) static AMBUSH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b7dd8623-b64a-4b47-a69d-ed62d44596fb"),
    "Ambush",
    crate::card::CardArt::new("b7dd8623-b64a-4b47-a69d-ed62d44596fb", "Alan Rabinowitz"),
    crate::card::CardSet::Homelands,
    CardRules::new_instant(mana_cost!("{3}{R}")).with_ability(AbilityDef::spell(
        "Blocking creatures gain first strike until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::Blocking,
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// HML 63a — Ambush Party
pub(in crate::card::sets) static AMBUSH_PARTY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("87e24788-cc7c-4f5d-84d8-dcb35e10626f"),
    "Ambush Party",
    crate::card::CardArt::new("87e24788-cc7c-4f5d-84d8-dcb35e10626f", "Mark Poole"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Human", "Rogue"], 3, 1)
        .with_abilities(&[abilities::first_strike(), abilities::haste()]),
);

// HML 63b — Ambush Party (alternate printing)

// HML 64 — An-Zerrin Ruins
pub(in crate::card::sets) static AN_ZERRIN_RUINS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4f905d57-2f52-4179-8041-2667b1fb1baa"),
    "An-Zerrin Ruins",
    crate::card::CardArt::new("4f905d57-2f52-4179-8041-2667b1fb1baa", "Dennis Detwiller"),
    crate::card::CardSet::Homelands,
    CardRules::new_enchantment(mana_cost!("{2}{R}{R}")).with_abilities(&[
        AbilityDef::replacement(
            "As this enchantment enters, choose a creature type.",
            ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(
                BattlefieldEntryScalarChoiceDef::CREATURE_TYPE,
            )),
        ),
        AbilityDef::static_ability(
            "Creatures of the chosen type don't untap during their controllers' untap steps.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasSourcesChosenScalar(
                            BattlefieldEntryChoiceDestinationDef::CreatureType,
                        ),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
            },
        ),
    ]),
);

// HML 65 — Anaba Ancestor
pub(in crate::card::sets) static ANABA_ANCESTOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c4d33cc0-525d-4e25-927b-b6b18087c27b"),
    "Anaba Ancestor",
    crate::card::CardArt::new("c4d33cc0-525d-4e25-927b-b6b18087c27b", "Anson Maddocks"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Minotaur", "Spirit"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Another target Minotaur creature gets +1/+1 until end of turn.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Subtype("Minotaur"),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// HML 66a — Anaba Bodyguard (alternate printing)

// HML 66b — Anaba Bodyguard
pub(in crate::card::sets) static ANABA_BODYGUARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("56a54048-4640-499b-a1c3-192917c25169"),
    "Anaba Bodyguard",
    crate::card::CardArt::new("56a54048-4640-499b-a1c3-192917c25169", "Anson Maddocks"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Minotaur"], 2, 3)
        .with_ability(abilities::first_strike()),
);

// HML 67a — Anaba Shaman (alternate printing)

// HML 67b — Anaba Shaman
pub(in crate::card::sets) static ANABA_SHAMAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4b456355-9f73-45c2-9554-6e6b20d949a1"),
    "Anaba Shaman",
    crate::card::CardArt::new("4b456355-9f73-45c2-9554-6e6b20d949a1", "Anson Maddocks"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Minotaur", "Shaman"], 2, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{R}, {T}: This creature deals 1 damage to any target.",
            &[CostDef::Mana(mana_cost!("{R}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// HML 68 — Anaba Spirit Crafter
pub(in crate::card::sets) static ANABA_SPIRIT_CRAFTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e9aaabc2-1dab-4f9c-8ed3-60bc1aa995ba"),
    "Anaba Spirit Crafter",
    crate::card::CardArt::new("e9aaabc2-1dab-4f9c-8ed3-60bc1aa995ba", "Anson Maddocks"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Minotaur", "Shaman"], 1, 3).with_ability(
        AbilityDef::static_ability(
            "Minotaur creatures get +1/+0.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Minotaur"),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
            },
        ),
    ),
);

// HML 69 — Chandler
pub(in crate::card::sets) static CHANDLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4dd3a8e3-9a90-44f4-996c-57242d3c47a5"),
    "Chandler",
    crate::card::CardArt::new("4dd3a8e3-9a90-44f4-996c-57242d3c47a5", "Douglas Shuler"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Human", "Rogue"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::activated_with_targets(
            "{R}{R}{R}, {T}: Destroy target artifact creature.",
            &[CostDef::Mana(mana_cost!("{R}{R}{R}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                ]),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        )),
);

// HML 70 — Dwarven Pony
pub(in crate::card::sets) static DWARVEN_PONY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("53a3019f-0b27-4ba3-be4c-73ed50eb9514"),
    "Dwarven Pony",
    crate::card::CardArt::new(
        "53a3019f-0b27-4ba3-be4c-73ed50eb9514",
        "Margaret Organ-Kean",
    ),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{R}"), &["Horse"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{1}{R}, {T}: Target Dwarf creature gains mountainwalk until end of turn.",
            &[CostDef::Mana(mana_cost!("{1}{R}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Subtype("Dwarf"),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::mountainwalk()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// HML 71 — Dwarven Sea Clan
// Audit: unsupported — Needs a target predicate relating an attacking or blocking creature's controller to control of an Island, then binding that target into delayed end-of-combat damage.
pub(in crate::card::sets) static DWARVEN_SEA_CLAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4cb722d9-1998-4912-a6f2-4ffa8d21311a"),
    "Dwarven Sea Clan",
    crate::card::CardArt::new("4cb722d9-1998-4912-a6f2-4ffa8d21311a", "Amy Weber"),
    crate::card::CardSet::Homelands,
    crate::card::CardRules::unsupported(),
);

// HML 72a — Dwarven Trader
pub(in crate::card::sets) static DWARVEN_TRADER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4db9aa47-f42b-41e9-948c-8b012c3809fb"),
    "Dwarven Trader",
    crate::card::CardArt::new(
        "4db9aa47-f42b-41e9-948c-8b012c3809fb",
        "Margaret Organ-Kean",
    ),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{R}"), &["Dwarf"], 1, 1),
);

// HML 72b — Dwarven Trader (alternate printing)

// HML 73 — Eron the Relentless
pub(in crate::card::sets) static ERON_THE_RELENTLESS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b6329bd9-1e03-43e6-b50b-8abe1356ffcc"),
    "Eron the Relentless",
    crate::card::CardArt::new("b6329bd9-1e03-43e6-b50b-8abe1356ffcc", "Christopher Rush"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Human", "Rogue"], 5, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::haste(),
            abilities::regenerate_self(
                "{R}{R}{R}: Regenerate Eron.",
                &[CostDef::Mana(mana_cost!("{R}{R}{R}"))],
            ),
        ]),
);

// HML 74 — Evaporate
pub(in crate::card::sets) static EVAPORATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a3c99939-4854-4e28-a142-4cb7f89fe898"),
    "Evaporate",
    crate::card::CardArt::new("a3c99939-4854-4e28-a142-4cb7f89fe898", "Alan Rabinowitz"),
    crate::card::CardSet::Homelands,
    CardRules::new_sorcery(mana_cost!("{2}{R}")).with_ability(AbilityDef::spell(
        "Evaporate deals 1 damage to each white and/or blue creature.",
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::Color(ManaColor::White),
                        ObjectPredicateDef::Color(ManaColor::Blue),
                    ]),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            amount: ValueDef::Constant(1),
        },
    )),
);

// HML 75 — Heart Wolf
// Audit: unsupported — Needs a turn-scoped delayed zone-change listener restricted to the targeted Dwarf so its departure sacrifices the source.
pub(in crate::card::sets) static HEART_WOLF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e0427dcd-26da-462b-b936-a382d3d8afce"),
    "Heart Wolf",
    crate::card::CardArt::new(
        "e0427dcd-26da-462b-b936-a382d3d8afce",
        "Margaret Organ-Kean",
    ),
    crate::card::CardSet::Homelands,
    crate::card::CardRules::unsupported(),
);

// HML 76 — Ironclaw Curse
// Audit: unsupported — Needs a block restriction comparing each prospective attacker's power with the enchanted creature's current toughness.
pub(in crate::card::sets) static IRONCLAW_CURSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e796f0ff-4e7b-4849-b463-0aac860c72ea"),
    "Ironclaw Curse",
    crate::card::CardArt::new("e796f0ff-4e7b-4849-b463-0aac860c72ea", "Dennis Detwiller"),
    crate::card::CardSet::Homelands,
    crate::card::CardRules::unsupported(),
);

// HML 77 — Joven
pub(in crate::card::sets) static JOVEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0dabe3af-cd5b-461e-95a4-aad046646419"),
    "Joven",
    crate::card::CardArt::new("0dabe3af-cd5b-461e-95a4-aad046646419", "Douglas Shuler"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Human", "Rogue"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::activated_with_targets(
            "{R}{R}{R}, {T}: Destroy target noncreature artifact.",
            &[CostDef::Mana(mana_cost!("{R}{R}{R}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                ]),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        )),
);

// HML 78 — Orcish Mine
pub(in crate::card::sets) static ORCISH_MINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3a630875-b43d-4591-992c-117e1212fa34"),
    "Orcish Mine",
    crate::card::CardArt::new("3a630875-b43d-4591-992c-117e1212fa34", "Kaja Foglio"),
    crate::card::CardSet::Homelands,
    CardRules::new_enchantment(mana_cost!("{1}{R}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_land(),
            AbilityDef::as_enters(
                "This Aura enters with three ore counters on it.",
                ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::AddCounters {
                        kind: CounterKind::named("ore"),
                        amount: 3,
                    },
                ),
            ),
            AbilityDef::triggered(
                "At the beginning of your upkeep, remove an ore counter from this Aura.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                EffectDef::RemoveCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::named("ore"),
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::triggered(
                "Whenever enchanted land becomes tapped, remove an ore counter from this Aura.",
                TriggerEventDef::tapped(ObjectPredicateDef::AttachedToSource),
                EffectDef::RemoveCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::named("ore"),
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::triggered(
                "When the last ore counter is removed from this Aura, destroy enchanted land and this Aura deals 2 damage to that land's controller.",
                TriggerEventDef::LastCounterRemoved {
                    object: ObjectPredicateDef::Source,
                    kind: CounterKind::named("ore"),
                },
                EffectDef::Sequence(&[
                    EffectDef::Destroy {
                        object: EffectRecipientDef::AttachedPermanent,
                        then: None,
                    },
                    EffectDef::DealDamage {
                        recipient: EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                            ObjectRefDef::AttachedToSource,
                        )),
                        amount: ValueDef::Constant(2),
                    },
                ]),
            ),
        ]),
);

// HML 79 — Retribution
// Audit: unsupported — Needs two targets constrained to the same opponent's control and a resolution choice by that opponent between those targeted objects.
pub(in crate::card::sets) static RETRIBUTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b3adf9a6-7137-4995-9a83-2d410cb3cd20"),
    "Retribution",
    crate::card::CardArt::new("b3adf9a6-7137-4995-9a83-2d410cb3cd20", "Mark Tedin"),
    crate::card::CardSet::Homelands,
    crate::card::CardRules::unsupported(),
);

// HML 80 — Winter Sky
pub(in crate::card::sets) static WINTER_SKY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("af1035f3-3027-4a41-834c-55222b13c2bc"),
    "Winter Sky",
    crate::card::CardArt::new("af1035f3-3027-4a41-834c-55222b13c2bc", "Mike Kimble"),
    crate::card::CardSet::Homelands,
    CardRules::new_sorcery(mana_cost!("{R}")).with_ability(AbilityDef::spell(
        "Flip a coin. If you win the flip, Winter Sky deals 1 damage to each creature and each player. If you lose the flip, each player draws a card.",
        EffectDef::FlipCoin {
            on_win: &EffectDef::Sequence(&[
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    amount: ValueDef::Constant(1),
                },
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::EachPlayer,
                    amount: ValueDef::Constant(1),
                },
            ]),
            on_loss: &EffectDef::DrawCards {
                recipient: EffectRecipientDef::EachPlayer,
                amount: ValueDef::Constant(1),
            },
        },
    )),
);

// HML 81 — An-Havva Constable
pub(in crate::card::sets) static AN_HAVVA_CONSTABLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("42c5a793-a777-44f9-a977-d16d26d3f852"),
    "An-Havva Constable",
    crate::card::CardArt::new("42c5a793-a777-44f9-a977-d16d26d3f852", "Dan Frazier"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Human"], 2, 1).with_ability(
        AbilityDef::static_ability(
            "An-Havva Constable's toughness is equal to 1 plus the number of green creatures on the battlefield.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Color(ManaColor::Green),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    )),
                ),
            },
        ),
    ),
);

// HML 82 — An-Havva Inn
pub(in crate::card::sets) static AN_HAVVA_INN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("eff4531f-d19d-44af-861a-33087197d21c"),
    "An-Havva Inn",
    crate::card::CardArt::new("eff4531f-d19d-44af-861a-33087197d21c", "Brian Snõddy"),
    crate::card::CardSet::Homelands,
    CardRules::new_sorcery(mana_cost!("{1}{G}{G}")).with_ability(AbilityDef::spell(
        "You gain X plus 1 life, where X is the number of green creatures on the battlefield.",
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Sum(&SumValueDef::new(
                ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Color(ManaColor::Green),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                )),
                ValueDef::Constant(1),
            )),
        },
    )),
);

// HML 83 — Autumn Willow
// Audit: unsupported — Needs a player-scoped permission to target one shrouded permanent until end of turn without removing shroud for other players.
pub(in crate::card::sets) static AUTUMN_WILLOW: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cea60340-bbdb-48e2-94a6-5ac1197e978a"),
    "Autumn Willow",
    crate::card::CardArt::new(
        "cea60340-bbdb-48e2-94a6-5ac1197e978a",
        "Margaret Organ-Kean",
    ),
    crate::card::CardSet::Homelands,
    crate::card::CardRules::unsupported(),
);

// HML 84a — Carapace
pub(in crate::card::sets) static CARAPACE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("07159586-270e-4a3e-9b21-0d74cf3e49d7"),
    "Carapace",
    crate::card::CardArt::new("07159586-270e-4a3e-9b21-0d74cf3e49d7", "Anson Maddocks"),
    crate::card::CardSet::Homelands,
    CardRules::new_enchantment(mana_cost!("{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +0/+2.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(0),
                        ValueDef::Constant(2),
                    ),
                },
            ),
            AbilityDef::activated(
                "Sacrifice this Aura: Regenerate enchanted creature.",
                &[CostDef::SacrificeSource],
                EffectDef::Regenerate {
                    object: EffectRecipientDef::AttachedPermanent,
                },
            ),
        ]),
);

// HML 84b — Carapace (alternate printing)

// HML 85 — Daughter of Autumn
// Audit: unsupported — Needs a one-point damage-redirection shield from a targeted white creature to the source; the existing redirection rule redirects the entire matching event.
pub(in crate::card::sets) static DAUGHTER_OF_AUTUMN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("972e9c59-f340-414c-b55b-39d46dd97e8e"),
    "Daughter of Autumn",
    crate::card::CardArt::new(
        "972e9c59-f340-414c-b55b-39d46dd97e8e",
        "Margaret Organ-Kean",
    ),
    crate::card::CardSet::Homelands,
    crate::card::CardRules::unsupported(),
);

// HML 86 — Faerie Noble
pub(in crate::card::sets) static FAERIE_NOBLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("00f8931e-6402-483c-a9e8-63ee344c36a7"),
    "Faerie Noble",
    crate::card::CardArt::new("00f8931e-6402-483c-a9e8-63ee344c36a7", "Susan Van Camp"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Faerie", "Noble"], 1, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "Other Faerie creatures you control get +0/+1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Faerie"),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(1),
                ),
            },
        ),
        AbilityDef::activated(
            "{T}: Other Faerie creatures you control get +1/+0 until end of turn.",
            &[CostDef::TapSource],
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Faerie"),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// HML 87a — Folk of An-Havva (alternate printing)

// HML 87b — Folk of An-Havva
// Audit: unsupported — Needs a blocker-declared event that fires once when the source blocks, independent of how many creatures it blocks; the current event is per blocking relationship.
pub(in crate::card::sets) static FOLK_OF_AN_HAVVA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4118c563-08a7-4654-973e-ab9c454f00f9"),
    "Folk of An-Havva",
    crate::card::CardArt::new("4118c563-08a7-4654-973e-ab9c454f00f9", "Julie Baroh"),
    crate::card::CardSet::Homelands,
    crate::card::CardRules::unsupported(),
);

// HML 88a — Hungry Mist
pub(in crate::card::sets) static HUNGRY_MIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("085973eb-56cd-4bb5-aefd-bdf36f2d2a3e"),
    "Hungry Mist",
    crate::card::CardArt::new("085973eb-56cd-4bb5-aefd-bdf36f2d2a3e", "Heather Hudson"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Elemental"], 6, 2).with_ability(
        AbilityDef::triggered(
            "At the beginning of your upkeep, sacrifice this creature unless you pay {G}{G}.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::PayOr(PayOrDef::unless_mana(
                mana_cost!("{G}{G}"),
                &EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            )),
        ),
    ),
);

// HML 88b — Hungry Mist (alternate printing)

// HML 89 — Joven's Ferrets
// Audit: unsupported — Needs the historical set of creatures that blocked the source this turn; the current combat-relation predicate forgets a blocker removed from combat before the end step.
pub(in crate::card::sets) static JOVEN_S_FERRETS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1f95eda2-f791-46e9-bb82-31422b8c5ce4"),
    "Joven's Ferrets",
    crate::card::CardArt::new("1f95eda2-f791-46e9-bb82-31422b8c5ce4", "Amy Weber"),
    crate::card::CardSet::Homelands,
    crate::card::CardRules::unsupported(),
);

// HML 90 — Leaping Lizard
pub(in crate::card::sets) static LEAPING_LIZARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4b0e4744-4d73-4e6e-950b-bb4c83229499"),
    "Leaping Lizard",
    crate::card::CardArt::new("4b0e4744-4d73-4e6e-950b-bb4c83229499", "Amy Weber"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Lizard"], 2, 3).with_ability(
        AbilityDef::activated(
            "{1}{G}: This creature gets -0/-1 and gains flying until end of turn.",
            &[CostDef::Mana(mana_cost!("{1}{G}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(0),
                        ValueDef::Constant(-1),
                    ),
                    AppliedEffectDef::add_ability(&abilities::flying()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// HML 91 — Mammoth Harness
pub(in crate::card::sets) static MAMMOTH_HARNESS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5be67121-068c-4770-bc42-c081577a442c"),
    "Mammoth Harness",
    crate::card::CardArt::new("5be67121-068c-4770-bc42-c081577a442c", "Melissa A. Benson"),
    crate::card::CardSet::Homelands,
    CardRules::new_enchantment(mana_cost!("{3}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature loses flying.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::remove_abilities(AbilityPredicateDef::Keyword(
                        KeywordAbility::Flying,
                    )),
                },
            ),
            AbilityDef::triggered(
                "Whenever enchanted creature blocks or becomes blocked by a creature, the other creature gains first strike until end of turn.",
                TriggerEventDef::BlocksOrBecomesBlockedBy {
                    creature: ObjectPredicateDef::AttachedToSource,
                    other: ObjectPredicateDef::HasType(CardType::Creature),
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::TriggeringObject,
                    effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// HML 92 — Primal Order
pub(in crate::card::sets) static PRIMAL_ORDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("21a3b8f7-c794-40ef-9ebd-dec5357260d4"),
    "Primal Order",
    crate::card::CardArt::new("21a3b8f7-c794-40ef-9ebd-dec5357260d4", "Rob Alexander"),
    crate::card::CardSet::Homelands,
    CardRules::new_enchantment(mana_cost!("{2}{G}{G}")).with_ability(AbilityDef::triggered(
        "At the beginning of each player's upkeep, this enchantment deals damage to that player equal to the number of nonbasic lands they control.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::Any,
        },
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::EventPlayer,
            amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(CardSupertype::Basic)),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::EventPlayer,
            )),
        },
    )),
);

// HML 93 — Renewal
pub(in crate::card::sets) static RENEWAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ab998cd1-2f49-42e7-b889-c6717b0ce884"),
    "Renewal",
    crate::card::CardArt::new("ab998cd1-2f49-42e7-b889-c6717b0ce884", "Kaja Foglio"),
    crate::card::CardSet::Homelands,
    CardRules::new_sorcery(mana_cost!("{2}{G}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, sacrifice a land.\nSearch your library for a basic land card, put that card onto the battlefield, then shuffle.\nDraw a card at the beginning of the next turn's upkeep.",
            &[],
            CostDef::sacrifice(
                ObjectPredicateDef::HasType(CardType::Land),
                CostQuantityDef::Fixed(1),
            ),
            EffectDef::Sequence(&[
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Supertype(CardSupertype::Basic),
                    ]),
                    minimum: 0,
                    maximum: ValueDef::Constant(1),
                    reveal: false,
                    destination: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: false,
                    attachment: None,
                    binding: None,
                    then: None,
                },
                EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                    "At the beginning of the next turn's upkeep, draw a card.",
                    TriggerEventDef::StepBegins {
                        step: TurnStepDef::Upkeep,
                        player: PlayerRelation::Any,
                    },
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                ))),
            ]),
        ),
    ),
);

// HML 94 — Root Spider
// Audit: unsupported — Needs a blocker-declared event that fires once when the source blocks, independent of how many creatures it blocks; the current event is per blocking relationship.
pub(in crate::card::sets) static ROOT_SPIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("407d67b0-d496-401b-8844-8e3ea2fd2046"),
    "Root Spider",
    crate::card::CardArt::new("407d67b0-d496-401b-8844-8e3ea2fd2046", "Mike Kimble"),
    crate::card::CardSet::Homelands,
    crate::card::CardRules::unsupported(),
);

// HML 95 — Roots
pub(in crate::card::sets) static ROOTS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("efb4c256-e790-41ec-a9ab-e6358e810798"),
    "Roots",
    crate::card::CardArt::new("efb4c256-e790-41ec-a9ab-e6358e810798", "Nicola Leonard"),
    crate::card::CardSet::Homelands,
    CardRules::new_enchantment(mana_cost!("{3}{G}"))
        .with_subtypes(&["Aura"])
        .enchanting(ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Not(&ObjectPredicateDef::HasKeyword(KeywordAbility::Flying)),
        ]))
        .with_abilities(&[
            abilities::aura_spell(
                "Enchant creature without flying",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasKeyword(
                            KeywordAbility::Flying,
                        )),
                    ]),
                )],
            ),
            abilities::enters_trigger(
                "When this Aura enters, tap enchanted creature.",
                EffectDef::Tap {
                    object: EffectRecipientDef::AttachedPermanent,
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature doesn't untap during its controller's untap step.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
                },
            ),
        ]),
);

// HML 96 — Rysorian Badger
pub(in crate::card::sets) static RYSORIAN_BADGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ab87a387-678b-4913-a0c7-85f0238cee26"),
    "Rysorian Badger",
    crate::card::CardArt::new("ab87a387-678b-4913-a0c7-85f0238cee26", "Heather Hudson"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Badger"], 2, 2).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks and isn't blocked, you may exile up to two target creature cards from defending player's graveyard. If you do, you gain 1 life for each card exiled this way and this creature assigns no combat damage this turn.",
            TriggerEventDef::AttacksAndIsNotBlocked {
                attacker: ObjectPredicateDef::Source,
            },
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::DefendingPlayer),
                },
                2,
            )],
            EffectDef::WithZoneMoveResult {
                effect: &EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Exile,
                    placement: ZonePlacement::Top,
                },
                binding: Binding!("rysorian_badger_exiled"),
                then: &EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ObjectSetCount(
                        &ObjectSetCountConditionDef {
                            objects: &ObjectSetDef::Binding(Binding!(
                                "rysorian_badger_exiled"
                            )),
                            predicate: ObjectSetPredicateDef {
                                filter: None,
                                comparison: ComparisonDef::GreaterOrEqual,
                                amount: 1,
                            },
                        },
                    ),
                    then: &EffectDef::Sequence(&[
                        EffectDef::GainLife {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::CountObjects(&ObjectSetDef::Binding(Binding!(
                                "rysorian_badger_exiled"
                            ))),
                        },
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::Source,
                            effect: AppliedEffectDef::Rule(
                                AppliedRuleDef::AssignsNoCombatDamage,
                            ),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                    ]),
                },
            },
        ),
    ),
);

// HML 97a — Shrink
pub(in crate::card::sets) static SHRINK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("30785867-32f7-46c9-94c2-775078e792ae"),
    "Shrink",
    crate::card::CardArt::new("30785867-32f7-46c9-94c2-775078e792ae", "Liz Danforth"),
    crate::card::CardSet::Homelands,
    CardRules::new_instant(mana_cost!("{G}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets -5/-0 until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(-5),
                ValueDef::Constant(0),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// HML 97b — Shrink (alternate printing)

// HML 98 — Spectral Bears
pub(in crate::card::sets) static SPECTRAL_BEARS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7e13875f-f745-4afd-a830-33df9576dce8"),
    "Spectral Bears",
    crate::card::CardArt::new("7e13875f-f745-4afd-a830-33df9576dce8", "Pat Lewis"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Bear", "Spirit"], 3, 3).with_ability(
        AbilityDef::triggered_if(
            "Whenever this creature attacks, if defending player controls no black nontoken permanents, it doesn't untap during your next untap step.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Color(ManaColor::Black),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::DefendingPlayer,
                ),
                comparison: ComparisonDef::Equal,
                amount: 0,
            },
            EffectDef::SkipNextUntapSteps {
                object: EffectRecipientDef::Source,
                count: 1,
            },
        ),
    ),
);

// HML 99a — Willow Faerie (alternate printing)

// HML 99b — Willow Faerie
pub(in crate::card::sets) static WILLOW_FAERIE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0e777dfe-44ed-4e73-bf77-ef4c667092d4"),
    "Willow Faerie",
    crate::card::CardArt::new("0e777dfe-44ed-4e73-bf77-ef4c667092d4", "Susan Van Camp"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Faerie"], 1, 2)
        .with_ability(abilities::flying()),
);

// HML 100 — Willow Priestess
pub(in crate::card::sets) static WILLOW_PRIESTESS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c636a608-26d7-4154-8052-a093b11362b1"),
    "Willow Priestess",
    crate::card::CardArt::new("c636a608-26d7-4154-8052-a093b11362b1", "Susan Van Camp"),
    crate::card::CardSet::Homelands,
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Faerie", "Druid"], 2, 2).with_abilities(&[
        AbilityDef::activated(
            "{T}: You may put a Faerie permanent card from your hand onto the battlefield.",
            &[CostDef::TapSource],
            EffectDef::ChooseCards {
                player: EffectRecipientDef::Controller,
                sources: &[CardChoiceSourceDef::Zone(ZoneKind::Hand)],
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Subtype("Faerie"),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                    ])),
                ]),
                minimum: 0,
                maximum: 1,
                reveal: false,
                destination: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
            },
        ),
        AbilityDef::activated_with_targets(
            "{2}{G}: Target green creature gains protection from black until end of turn.",
            &[CostDef::Mana(mana_cost!("{2}{G}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Color(ManaColor::Green),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::protection_from_color(
                    ManaColor::Black,
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// HML 101 — Apocalypse Chime
pub(in crate::card::sets) static APOCALYPSE_CHIME: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cef20d8f-6e80-4fca-b6a7-541981f6a112"),
    "Apocalypse Chime",
    crate::card::CardArt::new("cef20d8f-6e80-4fca-b6a7-541981f6a112", "Mark Poole"),
    crate::card::CardSet::Homelands,
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(AbilityDef::activated(
        "{2}, {T}, Sacrifice this artifact: Destroy all nontoken permanents with a name originally printed in the Homelands expansion. They can't be regenerated.",
        &[
            CostDef::Mana(mana_cost!("{2}")),
            CostDef::TapSource,
            CostDef::SacrificeSource,
        ],
        EffectDef::WithRule {
            rule: AppliedRuleDef::CannotRegenerate,
            effect: &EffectDef::Destroy {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::DebutSet(CardSet::Homelands),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                then: None,
            },
        },
    )),
);

// HML 102 — Clockwork Gnomes
pub(in crate::card::sets) static CLOCKWORK_GNOMES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3e0ca2ea-e059-4742-8ce4-22876762048c"),
    "Clockwork Gnomes",
    crate::card::CardArt::new("3e0ca2ea-e059-4742-8ce4-22876762048c", "Douglas Shuler"),
    crate::card::CardSet::Homelands,
    CardRules::new_artifact_creature(mana_cost!("{4}"), &["Gnome"], 2, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{3}, {T}: Regenerate target artifact creature.",
            &[CostDef::Mana(mana_cost!("{3}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                ]),
            )],
            EffectDef::Regenerate {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ),
);

// HML 103 — Clockwork Steed
// Audit: unsupported — Needs a resolution choice for up to X counters capped by the source's current counter count so the result never exceeds four.
pub(in crate::card::sets) static CLOCKWORK_STEED: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9b080587-d062-42ff-abc5-8e04a20faece"),
    "Clockwork Steed",
    crate::card::CardArt::new("9b080587-d062-42ff-abc5-8e04a20faece", "Amy Weber"),
    crate::card::CardSet::Homelands,
    crate::card::CardRules::unsupported(),
);

// HML 104 — Clockwork Swarm
// Audit: unsupported — Needs a resolution choice for up to X counters capped by the source's current counter count so the result never exceeds four.
pub(in crate::card::sets) static CLOCKWORK_SWARM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dfd89e5c-79dc-4a57-b5ea-16491443fea1"),
    "Clockwork Swarm",
    crate::card::CardArt::new("dfd89e5c-79dc-4a57-b5ea-16491443fea1", "Amy Weber"),
    crate::card::CardSet::Homelands,
    crate::card::CardRules::unsupported(),
);

// HML 105 — Didgeridoo
pub(in crate::card::sets) static DIDGERIDOO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("828f8f68-abe2-4e39-b3e4-991dceacd5d9"),
    "Didgeridoo",
    crate::card::CardArt::new("828f8f68-abe2-4e39-b3e4-991dceacd5d9", "Melissa A. Benson"),
    crate::card::CardSet::Homelands,
    CardRules::new_artifact(mana_cost!("{1}")).with_ability(AbilityDef::activated(
        "{3}: You may put a Minotaur permanent card from your hand onto the battlefield.",
        &[CostDef::Mana(mana_cost!("{3}"))],
        EffectDef::ChooseCards {
            player: EffectRecipientDef::Controller,
            sources: &[CardChoiceSourceDef::Zone(ZoneKind::Hand)],
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::Subtype("Minotaur"),
                ObjectPredicateDef::Not(&ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::HasType(CardType::Sorcery),
                ])),
            ]),
            minimum: 0,
            maximum: 1,
            reveal: false,
            destination: ZoneKind::Battlefield,
            placement: ZonePlacement::Top,
        },
    )),
);

// HML 106 — Ebony Rhino
pub(in crate::card::sets) static EBONY_RHINO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("81db749e-a1df-4615-9449-94731fa23a9f"),
    "Ebony Rhino",
    crate::card::CardArt::new("81db749e-a1df-4615-9449-94731fa23a9f", "Amy Weber"),
    crate::card::CardSet::Homelands,
    CardRules::new_artifact_creature(mana_cost!("{7}"), &["Rhino"], 4, 5)
        .with_ability(abilities::trample()),
);

// HML 107 — Feroz's Ban
pub(in crate::card::sets) static FEROZ_S_BAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("01ff4430-c8f7-408a-aad2-a098d747ea62"),
    "Feroz's Ban",
    crate::card::CardArt::new("01ff4430-c8f7-408a-aad2-a098d747ea62", "Heather Hudson"),
    crate::card::CardSet::Homelands,
    CardRules::new_artifact(mana_cost!("{6}")).with_ability(abilities::spell_cost_increase(
        "Creature spells cost {2} more to cast.",
        ObjectPredicateDef::HasType(CardType::Creature),
        PlayerRelation::Any,
        mana_cost!("{2}"),
    )),
);

// HML 108 — Joven's Tools
pub(in crate::card::sets) static JOVEN_S_TOOLS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d2520b38-76c1-45a2-9cda-a305f70762bd"),
    "Joven's Tools",
    crate::card::CardArt::new("d2520b38-76c1-45a2-9cda-a305f70762bd", "Nicola Leonard"),
    crate::card::CardSet::Homelands,
    CardRules::new_artifact(mana_cost!("{6}")).with_ability(AbilityDef::activated_with_targets(
        "{4}, {T}: Target creature can't be blocked this turn except by Walls.",
        &[CostDef::Mana(mana_cost!("{4}")), CostDef::TapSource],
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Rule(AppliedRuleDef::BlockRestriction(
                BlockRestrictionDef::prohibit(
                    BlockRestrictionSubjectDef::Attacker,
                    BlockRestrictionMatchDef::Except(ObjectPredicateDef::Subtype("Wall")),
                ),
            )),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// HML 109 — Roterothopter
pub(in crate::card::sets) static ROTEROTHOPTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("22148a1a-2172-4718-8ee4-08770eafed9f"),
    "Roterothopter",
    crate::card::CardArt::new("22148a1a-2172-4718-8ee4-08770eafed9f", "Amy Weber"),
    crate::card::CardSet::Homelands,
    CardRules::new_artifact_creature(mana_cost!("{1}"), &["Thopter"], 0, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{2}: This creature gets +1/+0 until end of turn. Activate no more than twice each turn.",
            &[CostDef::Mana(mana_cost!("{2}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .activations_each_turn(2),
    ]),
);

// HML 110 — Serrated Arrows
pub(in crate::card::sets) static SERRATED_ARROWS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("849a7d2b-3fdb-4e7f-b0b6-f6559dcb32e2"),
    "Serrated Arrows",
    crate::card::CardArt::new("849a7d2b-3fdb-4e7f-b0b6-f6559dcb32e2", "David A. Cherry"),
    crate::card::CardSet::Homelands,
    CardRules::new_artifact(mana_cost!("{4}")).with_abilities(&[
        AbilityDef::as_enters(
            "This artifact enters with three arrowhead counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::named("arrowhead"),
                    amount: 3,
                },
            ),
        ),
        AbilityDef::triggered_if(
            "At the beginning of your upkeep, if there are no arrowhead counters on this artifact, sacrifice it.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::SourceCounters {
                kind: CounterKind::named("arrowhead"),
                comparison: ComparisonDef::Equal,
                amount: 0,
            },
            EffectDef::Sacrifice {
                object: EffectRecipientDef::Source,
            },
        ),
        AbilityDef::activated_with_targets(
            "{T}, Remove an arrowhead counter from this artifact: Put a -1/-1 counter on target creature.",
            &[
                CostDef::TapSource,
                CostDef::RemoveCountersFromSource {
                    kind: CounterKind::named("arrowhead"),
                    amount: 1,
                },
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::MinusOneMinusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// HML 111 — An-Havva Township
pub(in crate::card::sets) static AN_HAVVA_TOWNSHIP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9afac347-4316-43e2-848b-e474ed563af6"),
    "An-Havva Township",
    crate::card::CardArt::new("9afac347-4316-43e2-848b-e474ed563af6", "Liz Danforth"),
    crate::card::CardSet::Homelands,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_mana(
            "{1}, {T}: Add {G}.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
        ),
        AbilityDef::activated_mana(
            "{2}, {T}: Add {R} or {W}.",
            &[CostDef::Mana(mana_cost!("{2}")), CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Red,
                ManaColor::White,
            ])),
        ),
    ]),
);

// HML 112 — Aysen Abbey
pub(in crate::card::sets) static AYSEN_ABBEY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2a2e669b-61b2-4729-b636-094796fb1d93"),
    "Aysen Abbey",
    crate::card::CardArt::new("2a2e669b-61b2-4729-b636-094796fb1d93", "Liz Danforth"),
    crate::card::CardSet::Homelands,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_mana(
            "{1}, {T}: Add {W}.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::White)),
        ),
        AbilityDef::activated_mana(
            "{2}, {T}: Add {G} or {U}.",
            &[CostDef::Mana(mana_cost!("{2}")), CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Green,
                ManaColor::Blue,
            ])),
        ),
    ]),
);

// HML 113 — Castle Sengir
pub(in crate::card::sets) static CASTLE_SENGIR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("16bfba30-4075-4bd6-9e4b-3a37641d43ce"),
    "Castle Sengir",
    crate::card::CardArt::new("16bfba30-4075-4bd6-9e4b-3a37641d43ce", "Pete Venters"),
    crate::card::CardSet::Homelands,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_mana(
            "{1}, {T}: Add {B}.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Black)),
        ),
        AbilityDef::activated_mana(
            "{2}, {T}: Add {U} or {R}.",
            &[CostDef::Mana(mana_cost!("{2}")), CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[ManaColor::Blue, ManaColor::Red])),
        ),
    ]),
);

// HML 114 — Koskun Keep
pub(in crate::card::sets) static KOSKUN_KEEP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("395fe900-ed19-438e-a658-ed7cf85818e5"),
    "Koskun Keep",
    crate::card::CardArt::new("395fe900-ed19-438e-a658-ed7cf85818e5", "Pat Lewis"),
    crate::card::CardSet::Homelands,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_mana(
            "{1}, {T}: Add {R}.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)),
        ),
        AbilityDef::activated_mana(
            "{2}, {T}: Add {B} or {G}.",
            &[CostDef::Mana(mana_cost!("{2}")), CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Black,
                ManaColor::Green,
            ])),
        ),
    ]),
);

// HML 115 — Wizards' School
pub(in crate::card::sets) static WIZARDS_SCHOOL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cd736532-8e98-4f4a-b48f-a66c57efcbfd"),
    "Wizards' School",
    crate::card::CardArt::new("cd736532-8e98-4f4a-b48f-a66c57efcbfd", "Pat Lewis"),
    crate::card::CardSet::Homelands,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_mana(
            "{1}, {T}: Add {U}.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Blue)),
        ),
        AbilityDef::activated_mana(
            "{2}, {T}: Add {W} or {B}.",
            &[CostDef::Mana(mana_cost!("{2}")), CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Black,
            ])),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ABBEY_GARGOYLES,
    &ABBEY_MATRON,
    &AYSEN_BUREAUCRATS,
    &AYSEN_CRUSADER,
    &AYSEN_HIGHWAY,
    &BEAST_WALKERS,
    &DEATH_SPEAKERS,
    &HAZDUHR_THE_ABBOT,
    &LEECHES,
    &MESA_FALCON,
    &PROPHECY,
    &RASHKA_THE_SLAYER,
    &SAMITE_ALCHEMIST,
    &SERRA_AVIARY,
    &SERRA_BESTIARY,
    &SERRA_INQUISITORS,
    &SERRA_PALADIN,
    &SORAYA_THE_FALCONER,
    &TRADE_CARAVAN,
    &TRUCE,
    &AETHER_STORM,
    &BAKI_S_CURSE,
    &CHAIN_STASIS,
    &CORAL_REEF,
    &DARK_MAZE,
    &FORGET,
    &GIANT_ALBATROSS,
    &GIANT_OYSTER,
    &JINX,
    &LABYRINTH_MINOTAUR,
    &MARJHAN,
    &MEMORY_LAPSE,
    &MERCHANT_SCROLL,
    &MYSTIC_DECREE,
    &NARWHAL,
    &REEF_PIRATES,
    &REVEKA_WIZARD_SAVANT,
    &SEA_SPRITE,
    &SEA_TROLL,
    &WALL_OF_KELP,
    &BARON_SENGIR,
    &BLACK_CARRIAGE,
    &BROKEN_VISAGE,
    &CEMETERY_GATE,
    &DRUDGE_SPELL,
    &DRY_SPELL,
    &FEAST_OF_THE_UNICORN,
    &FUNERAL_MARCH,
    &GHOST_HOUNDS,
    &GRANDMOTHER_SENGIR,
    &GREATER_WEREWOLF,
    &HEADSTONE,
    &IHSAN_S_SHADE,
    &IRINI_SENGIR,
    &KOSKUN_FALLS,
    &SENGIR_AUTOCRAT,
    &SENGIR_BATS,
    &TIMMERIAN_FIENDS,
    &TORTURE,
    &VELDRANE_OF_SENGIR,
    &ALIBAN_S_TOWER,
    &AMBUSH,
    &AMBUSH_PARTY,
    &AN_ZERRIN_RUINS,
    &ANABA_ANCESTOR,
    &ANABA_BODYGUARD,
    &ANABA_SHAMAN,
    &ANABA_SPIRIT_CRAFTER,
    &CHANDLER,
    &DWARVEN_PONY,
    &DWARVEN_SEA_CLAN,
    &DWARVEN_TRADER,
    &ERON_THE_RELENTLESS,
    &EVAPORATE,
    &HEART_WOLF,
    &IRONCLAW_CURSE,
    &JOVEN,
    &ORCISH_MINE,
    &RETRIBUTION,
    &WINTER_SKY,
    &AN_HAVVA_CONSTABLE,
    &AN_HAVVA_INN,
    &AUTUMN_WILLOW,
    &CARAPACE,
    &DAUGHTER_OF_AUTUMN,
    &FAERIE_NOBLE,
    &FOLK_OF_AN_HAVVA,
    &HUNGRY_MIST,
    &JOVEN_S_FERRETS,
    &LEAPING_LIZARD,
    &MAMMOTH_HARNESS,
    &PRIMAL_ORDER,
    &RENEWAL,
    &ROOT_SPIDER,
    &ROOTS,
    &RYSORIAN_BADGER,
    &SHRINK,
    &SPECTRAL_BEARS,
    &WILLOW_FAERIE,
    &WILLOW_PRIESTESS,
    &APOCALYPSE_CHIME,
    &CLOCKWORK_GNOMES,
    &CLOCKWORK_STEED,
    &CLOCKWORK_SWARM,
    &DIDGERIDOO,
    &EBONY_RHINO,
    &FEROZ_S_BAN,
    &JOVEN_S_TOOLS,
    &ROTEROTHOPTER,
    &SERRATED_ARROWS,
    &AN_HAVVA_TOWNSHIP,
    &AYSEN_ABBEY,
    &CASTLE_SENGIR,
    &KOSKUN_KEEP,
    &WIZARDS_SCHOOL,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::alternate(&ABBEY_MATRON, 1), // HML 2b
    PrintingRecord::alternate(&AYSEN_BUREAUCRATS, 1), // HML 3a
    PrintingRecord::alternate(&MESA_FALCON, 1),  // HML 10a
    PrintingRecord::alternate(&SAMITE_ALCHEMIST, 1), // HML 13a
    PrintingRecord::alternate(&TRADE_CARAVAN, 1), // HML 19a
    PrintingRecord::alternate(&DARK_MAZE, 1),    // HML 25b
    PrintingRecord::alternate(&GIANT_ALBATROSS, 1), // HML 27b
    PrintingRecord::alternate(&LABYRINTH_MINOTAUR, 1), // HML 30a
    PrintingRecord::alternate(&MEMORY_LAPSE, 1), // HML 32b
    PrintingRecord::alternate(&REEF_PIRATES, 1), // HML 36b
    PrintingRecord::alternate(&CEMETERY_GATE, 1), // HML 44b
    PrintingRecord::alternate(&DRY_SPELL, 1),    // HML 46b
    PrintingRecord::alternate(&FEAST_OF_THE_UNICORN, 1), // HML 47b
    PrintingRecord::alternate(&SENGIR_BATS, 1),  // HML 57a
    PrintingRecord::alternate(&TORTURE, 1),      // HML 59b
    PrintingRecord::alternate(&ALIBAN_S_TOWER, 1), // HML 61b
    PrintingRecord::alternate(&AMBUSH_PARTY, 1), // HML 63b
    PrintingRecord::alternate(&ANABA_BODYGUARD, 1), // HML 66a
    PrintingRecord::alternate(&ANABA_SHAMAN, 1), // HML 67a
    PrintingRecord::alternate(&DWARVEN_TRADER, 1), // HML 72b
    PrintingRecord::alternate(&CARAPACE, 1),     // HML 84b
    PrintingRecord::alternate(&FOLK_OF_AN_HAVVA, 1), // HML 87a
    PrintingRecord::alternate(&HUNGRY_MIST, 1),  // HML 88b
    PrintingRecord::alternate(&SHRINK, 1),       // HML 97b
    PrintingRecord::alternate(&WILLOW_FAERIE, 1), // HML 99a
];
