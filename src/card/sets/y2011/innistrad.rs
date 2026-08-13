//! Innistrad card records used by the built-in ISD–DGM Standard deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::sets::{y1993::alpha, y2002::onslaught};
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityPolicyHint, AbilityTargetDef,
    AbilityTargetPredicate, AddManaEffectDef, AppliedEffectDef, AppliedRuleDef, BasicLandType,
    BattlefieldEntryModificationDef, CardAbilityBinding, CardArt, CardBehavior, CardComposition,
    CardEffectStatus, CardPart, CardRules, CardSet, CardStructure, CardSupertype, CardType,
    ComparisonDef, ConditionalValueDef, ControlDurationDef, CounterKind, DiscardSelectionDef,
    DoubleFacedKind, EffectDef, EffectExecutionDef, EffectPaymentDef, EffectRecipientDef,
    ManaColor, ObjectPredicateDef, ObjectQueryDef, PayOrDef, PlayOptionDef, PlayerRelation,
    PlayerSetDef, QuantifierDef, ReplacementConditionDef, ReplacementEffectDef,
    ResolvedEffectDurationDef, SpellAdditionalCostDef, SpellForm, TargetConditionDef,
    TopCardSelectionDef, TriggerConditionDef, TriggerEventDef, ValueDef, ZoneKind, ZonePlacement,
    abilities, cards,
};
use crate::game::{
    CardAbilityResolver, CardRuntime, PileChoice, PileChosen, PileSplit, PilesSeparated,
    ResolvedAbility,
};
use crate::ids::{AbilityId, CardPartId, PlayOptionId, TargetIndex, TargetSlotId};
use crate::mana_cost;

// ISD 1 — Abbey Griffin
pub(in crate::card::sets) static ABBEY_GRIFFIN: CardRecord = CardRecord::new(
    cards::ABBEY_GRIFFIN,
    "Abbey Griffin",
    CardArt::new("bf87803b-e7c6-4122-add4-72e596167b7e", "Jaime Jones"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Griffin"], 2, 2)
        .with_abilities(&[abilities::flying(), abilities::vigilance()]),
);

// ISD 2 — Angel of Flight Alabaster
pub(in crate::card::sets) static ANGEL_OF_FLIGHT_ALABASTER: CardRecord = CardRecord::new(
    cards::ANGEL_OF_FLIGHT_ALABASTER,
    "Angel of Flight Alabaster",
    CardArt::new("8dfe629f-485c-4619-9713-32d2ae406e63", "Howard Lyon"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Angel"], 4, 4).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "At the beginning of your upkeep, return target Spirit card from your graveyard to your hand.",
            TriggerEventDef::StepBegins {
                step: crate::card::TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Subtype("Spirit"),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            })],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                controller: None,
                placement: ZonePlacement::Top,
            },
        ),
    ]),
);

// ISD 3 — Angelic Overseer
// Audit: blocked — Needs a continuous condition that grants hexproof and indestructible only while you control a Human.

// ISD 4 — Avacynian Priest
pub(in crate::card::sets) static AVACYNIAN_PRIEST: CardRecord = CardRecord::new(
    cards::AVACYNIAN_PRIEST,
    "Avacynian Priest",
    CardArt::new("08a47828-a79a-4189-9eef-2a5fc5125b61", "Greg Staples"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Cleric"], 1, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{1}, {T}: Tap target non-Human creature.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Human")),
                ]),
            )],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ),
);

// ISD 5 — Bonds of Faith
// Audit: blocked — Needs a subtype-conditional Aura effect that switches between +2/+2 and an attack-and-block prohibition.

// ISD 6 — Champion of the Parish
pub(in crate::card::sets) static CHAMPION_OF_THE_PARISH: CardRecord = CardRecord::new(
    cards::CHAMPION_OF_THE_PARISH,
    "Champion of the Parish",
    CardArt::new("f7314414-c2d2-48ed-af2c-764cf0207c62", "Svetlin Velinov"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Soldier"], 1, 1).with_ability(
        AbilityDef::triggered(
            "Whenever another Human you control enters, put a +1/+1 counter on this creature.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Subtype("Human"),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// ISD 7 — Chapel Geist
pub(in crate::card::sets) static CHAPEL_GEIST: CardRecord = CardRecord::new(
    cards::CHAPEL_GEIST,
    "Chapel Geist",
    CardArt::new("790cdf67-80d6-4ade-aecf-f77120b509b0", "Peter Mohrbacher"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Spirit"], 2, 3)
        .with_ability(abilities::flying()),
);

const fn cloistered_youth_front_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human"], 1, 1).with_ability(
        AbilityDef::triggered(
            "At the beginning of your upkeep, you may transform this creature.",
            TriggerEventDef::StepBegins {
                step: crate::card::TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Transform {
                    object: EffectRecipientDef::Source,
                },
            },
        ),
    )
}

const fn unholy_fiend_rules() -> CardRules {
    CardRules::new_creature_without_mana_cost(&["Horror"], 3, 3)
        .printed_colors(&[ManaColor::Black])
        .with_ability(AbilityDef::triggered(
            "At the beginning of your end step, you lose 1 life.",
            TriggerEventDef::StepBegins {
                step: crate::card::TurnStepDef::End,
                player: PlayerRelation::You,
            },
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ))
}

fn cloistered_youth_composition() -> CardComposition {
    let front = cloistered_youth_front_rules();
    let back = unholy_fiend_rules();
    CardComposition {
        parts: vec![
            CardPart::new(CardPartId::PRIMARY, "Cloistered Youth", front),
            CardPart::new(CardPartId(1), "Unholy Fiend", back),
        ],
        structure: CardStructure::DoubleFaced {
            front: CardPartId::PRIMARY,
            back: CardPartId(1),
            kind: DoubleFacedKind::Transforming,
        },
        play_options: vec![PlayOptionDef::cast(
            PlayOptionId::DEFAULT,
            "Cloistered Youth",
            SpellForm::Part(CardPartId::PRIMARY),
            mana_cost!("{1}{W}"),
            CardEffectStatus::Implemented,
        )],
    }
}

// ISD 8 — Cloistered Youth
pub(in crate::card::sets) static CLOISTERED_YOUTH: CardRecord = CardRecord::new(
    cards::CLOISTERED_YOUTH,
    "Cloistered Youth",
    CardArt::new("f8b8f0b4-71e1-4822-99a1-b1b3c2f10cb2", "Igor Kieryluk"),
    CardSet::Innistrad,
    cloistered_youth_front_rules(),
)
.with_composition(cloistered_youth_composition);

// ISD 9 — Dearly Departed
// Audit: blocked — Needs a graveyard static ability that modifies how other Human creatures enter with counters.

// ISD 10 — Divine Reckoning
// Audit: blocked — Needs simultaneous per-player creature choices followed by destroying every unchosen creature.

// ISD 11 — Doomed Traveler
pub(in crate::card::sets) static DOOMED_TRAVELER: CardRecord = CardRecord::new(
    cards::DOOMED_TRAVELER,
    "Doomed Traveler",
    CardArt::new("652c3bbb-cac8-47ad-81de-41e954e17a29", "Lars Grant-West"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Soldier"], 1, 1).with_ability(
        AbilityDef::triggered(
            "When this creature dies, create a 1/1 white Spirit creature token with flying.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::CreateToken {
                token: cards::SPIRIT_TOKEN_1_1_WHITE,
                count: ValueDef::Constant(1),
                tapped: false,
            },
        ),
    ),
);

static ELDER_CATHAR_COUNTERS: ValueDef = ValueDef::IfTargetMatches(&TargetConditionDef {
    slot: TargetIndex::PRIMARY,
    object: ObjectPredicateDef::Subtype("Human"),
    then: ValueDef::Constant(2),
    otherwise: ValueDef::Constant(1),
});

// ISD 12 — Elder Cathar
pub(in crate::card::sets) static ELDER_CATHAR: CardRecord = CardRecord::new(
    cards::ELDER_CATHAR,
    "Elder Cathar",
    CardArt::new("c21b9e51-fecd-4f9a-9354-a6dc1613feb3", "Chris Rahn"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Soldier"], 2, 2).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature dies, put a +1/+1 counter on target creature you control. If that creature is a Human, put two +1/+1 counters on it instead.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::Source, Some(ZoneKind::Battlefield), Some(ZoneKind::Graveyard)),
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            })],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ELDER_CATHAR_COUNTERS,
            },
        ),
    ),
);

// ISD 13 — Elite Inquisitor
// Audit: blocked — Needs protection parameterized by creature subtypes, not only by color.

// ISD 14 — Feeling of Dread
pub(in crate::card::sets) static FEELING_OF_DREAD: CardRecord = CardRecord::new(
    cards::FEELING_OF_DREAD,
    "Feeling of Dread",
    CardArt::new("846a2f9e-ad4f-4666-b152-fdeab7559d86", "John Stanko"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Tap up to two target creatures.",
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                2,
            )],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
        abilities::flashback(mana_cost!("{1}{U}")),
    ]),
);

// ISD 15 — Fiend Hunter
pub(in crate::card::sets) static FIEND_HUNTER: CardRecord = CardRecord::new(
    cards::FIEND_HUNTER,
    "Fiend Hunter",
    CardArt::new("f1e4c7d8-11a5-40fe-962b-7e938bf08616", "Wayne Reynolds"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Human", "Cleric"], 1, 3)
        .with_abilities(&[
            AbilityDef::triggered_with_targets(
                "When this creature enters, you may exile another target creature.",
                TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)),
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    1,
                )],
                EffectDef::ExileLinkedToSource {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::triggered(
                "When this creature leaves the battlefield, return the exiled card to the battlefield under its owner's control.",
                TriggerEventDef::zone_changed(ObjectPredicateDef::Source, Some(ZoneKind::Battlefield), None),
                EffectDef::ReturnLinkedExiles {
                    zone: ZoneKind::Battlefield,
                    grant: None,
                },
            ),
        ]),
);

// ISD 16 — Gallows Warden
pub(in crate::card::sets) static GALLOWS_WARDEN: CardRecord = CardRecord::new(
    cards::GALLOWS_WARDEN,
    "Gallows Warden",
    CardArt::new("15947b20-8c8e-42ed-9599-8b180a382d21", "Dan Murayama Scott"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Spirit"], 3, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "Other Spirit creatures you control get +0/+1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Spirit"),
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
    ]),
);

static CREATURES_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

// ISD 17 — Geist-Honored Monk
pub(in crate::card::sets) static GEIST_HONORED_MONK: CardRecord = CardRecord::new(
    cards::GEIST_HONORED_MONK,
    "Geist-Honored Monk",
    CardArt::new("5d51355e-55fa-43bb-a5de-fc55ac7b6446", "Clint Cearley"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{3}{W}{W}"), &["Human", "Monk"], 0, 0)
        .with_abilities(&[
            abilities::vigilance(),
            AbilityDef::static_ability(
                "Geist-Honored Monk's power and toughness are each equal to the number of creatures you control.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(ValueDef::CountMatchingObjects(&CREATURES_YOU_CONTROL), ValueDef::CountMatchingObjects(&CREATURES_YOU_CONTROL)),
                },
            ),
            AbilityDef::triggered(
                "When this creature enters, create two 1/1 white Spirit creature tokens with flying.",
                TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)),
                EffectDef::CreateToken {
                    token: cards::SPIRIT_TOKEN_1_1_WHITE,
                    count: ValueDef::Constant(2),
                    tapped: false,
                },
            ),
        ]),
);

// ISD 18 — Ghostly Possession
// Audit: blocked — Needs a persistent Aura effect preventing all combat damage dealt to and dealt by the enchanted creature.

// ISD 19 — Intangible Virtue
// Audit: blocked — Needs a reusable predicate selecting only creature tokens for the static +1/+1 and vigilance grant.

// ISD 20 — Mausoleum Guard
pub(in crate::card::sets) static MAUSOLEUM_GUARD: CardRecord = CardRecord::new(
    cards::MAUSOLEUM_GUARD,
    "Mausoleum Guard",
    CardArt::new("2c7b19de-96a6-4590-bfc3-31b0c7b2e25e", "David Palumbo"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Scout"], 2, 2).with_ability(
        AbilityDef::triggered(
            "When this creature dies, create two 1/1 white Spirit creature tokens with flying.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::CreateToken {
                token: cards::SPIRIT_TOKEN_1_1_WHITE,
                count: ValueDef::Constant(2),
                tapped: false,
            },
        ),
    ),
);

// ISD 21 — Mentor of the Meek
// Audit: blocked — Needs an enters trigger filtered by power 2 or less and its optional mana-payment continuation.

// ISD 22 — Midnight Haunting
pub(in crate::card::sets) static MIDNIGHT_HAUNTING: CardRecord = CardRecord::new(
    cards::MIDNIGHT_HAUNTING,
    "Midnight Haunting",
    CardArt::new("fe1eb098-7128-4ec8-8218-51fdde3e8326", "Matt Stewart"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{2}{W}")).with_ability(AbilityDef::spell(
        "Create two 1/1 white Spirit creature tokens with flying.",
        EffectDef::CreateToken {
            token: cards::SPIRIT_TOKEN_1_1_WHITE,
            count: ValueDef::Constant(2),
            tapped: false,
        },
    )),
);

// ISD 23 — Mikaeus, the Lunarch
// Audit: blocked — Needs an X-valued enters-with-counters replacement and a counter-removal activation that buffs every other creature.

// ISD 24 — Moment of Heroism
pub(in crate::card::sets) static MOMENT_OF_HEROISM: CardRecord = CardRecord::new(
    cards::MOMENT_OF_HEROISM,
    "Moment of Heroism",
    CardArt::new(
        "ba8d15bc-889d-4fd0-9688-00e22db30036",
        "Christopher Moeller",
    ),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +2/+2 and gains lifelink until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::lifelink()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )),
);

// ISD 25 — Nevermore
// Audit: blocked — Needs a nonland card-name choice and a cast prohibition keyed to that stored name.

// ISD 26 — Paraselene
// Audit: blocked — Needs a linked count of enchantments actually destroyed before gaining that much life.

// ISD 27 — Purify the Grave
pub(in crate::card::sets) static PURIFY_THE_GRAVE: CardRecord = CardRecord::new(
    cards::PURIFY_THE_GRAVE,
    "Purify the Grave",
    CardArt::new("7cf39365-e468-46ac-bb5b-7f43faa19458", "Drew Baker"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{W}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Exile target card from a graveyard.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Exile,
                controller: None,
                placement: ZonePlacement::Top,
            },
        ),
        abilities::flashback(mana_cost!("{W}")),
    ]),
);

// ISD 28 — Rally the Peasants
pub(in crate::card::sets) static RALLY_THE_PEASANTS: CardRecord = CardRecord::new(
    cards::RALLY_THE_PEASANTS,
    "Rally the Peasants",
    CardArt::new("514fe7de-16b2-42c0-adb1-f0af1c89cfd6", "Jaime Jones"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{2}{W}")).with_abilities(&[
        AbilityDef::spell(
            "Creatures you control get +2/+0 until end of turn.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::flashback(mana_cost!("{2}{R}")),
    ]),
);

// ISD 29 — Rebuke
pub(in crate::card::sets) static REBUKE: CardRecord = CardRecord::new(
    cards::REBUKE,
    "Rebuke",
    CardArt::new("267185ac-a176-423e-a7f8-ee966d1d9a1e", "Igor Kieryluk"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{2}{W}")).with_ability(AbilityDef::destroy_target(
        "Destroy target attacking creature.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Attacking,
        ])),
        true,
    )),
);

// ISD 30 — Selfless Cathar
pub(in crate::card::sets) static SELFLESS_CATHAR: CardRecord = CardRecord::new(
    cards::SELFLESS_CATHAR,
    "Selfless Cathar",
    CardArt::new("5a1dc067-1972-4d46-ad5d-56e6a563f638", "Slawomir Maniak"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Cleric"], 1, 1).with_ability(
        AbilityDef::activated(
            "{1}{W}, Sacrifice this creature: Creatures you control get +1/+1 until end of turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{W}")),
                AbilityCostDef::SacrificeSource,
            ],
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ISD 31 — Silverchase Fox
pub(in crate::card::sets) static SILVERCHASE_FOX: CardRecord = CardRecord::new(
    cards::SILVERCHASE_FOX,
    "Silverchase Fox",
    CardArt::new("0a81bfab-3397-4562-8b82-5f24cef167e3", "Howard Lyon"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Fox"], 2, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{1}{W}, Sacrifice this creature: Exile target enchantment.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{W}")),
                AbilityCostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Enchantment),
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Exile,
                controller: None,
                placement: ZonePlacement::Top,
            },
        ),
    ),
);

// ISD 32 — Slayer of the Wicked
pub(in crate::card::sets) static SLAYER_OF_THE_WICKED: CardRecord = CardRecord::new(
    cards::SLAYER_OF_THE_WICKED,
    "Slayer of the Wicked",
    CardArt::new("1c2cd68e-ff4c-49c7-ba0d-f2299d9c21f4", "Anthony Palumbo"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Soldier"], 3, 2).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature enters, you may destroy target Vampire, Werewolf, or Zombie.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::Subtype("Vampire"),
                        ObjectPredicateDef::Subtype("Werewolf"),
                        ObjectPredicateDef::Subtype("Zombie"),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                1,
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            },
        ),
    ),
);

// ISD 33 — Smite the Monstrous
// Audit: partial — PowerAtLeast does not include power changes from static continuous effects when checking target legality.
pub(in crate::card::sets) static SMITE_THE_MONSTROUS: CardRecord = CardRecord::new(
    cards::SMITE_THE_MONSTROUS,
    "Smite the Monstrous",
    CardArt::new("0103f3b1-88c2-4cbf-a67c-49420f92970f", "Jason Felix"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{3}{W}")).with_ability(
        AbilityDef::destroy_target(
            "Destroy target creature with power 4 or greater.",
            &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::PowerAtLeast(4),
            ])),
            true,
        )
        .with_coverage(AbilityCoverageDef::partial(
            "PowerAtLeast reads resolved power changes but not power changes supplied by static continuous effects.",
        )),
    ),
);

// ISD 34 — Spare from Evil
// Audit: blocked — Needs temporary protection from the class of all non-Human creatures.

// ISD 35 — Spectral Rider
pub(in crate::card::sets) static SPECTRAL_RIDER: CardRecord = CardRecord::new(
    cards::SPECTRAL_RIDER,
    "Spectral Rider",
    CardArt::new("b47e4e56-8bde-480d-b59c-17a017665b19", "Igor Kieryluk"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{W}{W}"), &["Spirit", "Knight"], 2, 2)
        .with_ability(abilities::intimidate()),
);

// ISD 36 — Stony Silence
// Audit: blocked — Needs a battlefield-wide prohibition on activating nonmana abilities of artifacts.

// ISD 37 — Thraben Purebloods
pub(in crate::card::sets) static THRABEN_PUREBLOODS: CardRecord = CardRecord::new(
    cards::THRABEN_PUREBLOODS,
    "Thraben Purebloods",
    CardArt::new("16db28f4-3d96-42f5-a264-592fdc2d4196", "Martina Pilcerova"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Dog"], 3, 5),
);

static THRABEN_SENTRY_FRONT_ABILITIES: [AbilityDef; 2] = [
    abilities::vigilance(),
    AbilityDef::triggered(
        "Whenever another creature you control dies, you may transform this creature.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
            ]),
            Some(ZoneKind::Battlefield),
            Some(ZoneKind::Graveyard),
        ),
        EffectDef::May {
            player: EffectRecipientDef::Controller,
            effect: &EffectDef::Transform {
                object: EffectRecipientDef::Source,
            },
        },
    ),
];

const fn thraben_sentry_front_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Soldier"], 2, 2)
        .with_abilities(&THRABEN_SENTRY_FRONT_ABILITIES)
}

const fn thraben_militia_rules() -> CardRules {
    CardRules::new_creature_without_mana_cost(&["Human", "Soldier"], 5, 4)
        .printed_colors(&[ManaColor::White])
        .with_ability(abilities::trample())
}

fn thraben_sentry_composition() -> CardComposition {
    let front = thraben_sentry_front_rules();
    let back = thraben_militia_rules();
    CardComposition {
        parts: vec![
            CardPart::new(CardPartId::PRIMARY, "Thraben Sentry", front),
            CardPart::new(CardPartId(1), "Thraben Militia", back),
        ],
        structure: CardStructure::DoubleFaced {
            front: CardPartId::PRIMARY,
            back: CardPartId(1),
            kind: DoubleFacedKind::Transforming,
        },
        play_options: vec![PlayOptionDef::cast(
            PlayOptionId::DEFAULT,
            "Thraben Sentry",
            SpellForm::Part(CardPartId::PRIMARY),
            mana_cost!("{3}{W}"),
            CardEffectStatus::Implemented,
        )],
    }
}

// ISD 38 — Thraben Sentry
pub(in crate::card::sets) static THRABEN_SENTRY: CardRecord = CardRecord::new(
    cards::THRABEN_SENTRY,
    "Thraben Sentry",
    CardArt::new("58ae9cbc-d88d-42df-ab76-63ab5d05c023", "David Rapoza"),
    CardSet::Innistrad,
    thraben_sentry_front_rules(),
)
.with_composition(thraben_sentry_composition);

// ISD 39 — Unruly Mob
pub(in crate::card::sets) static UNRULY_MOB: CardRecord = CardRecord::new(
    cards::UNRULY_MOB,
    "Unruly Mob",
    CardArt::new("491c6e40-151a-4efd-980c-e6b6a1057c58", "Ryan Pancoast"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human"], 1, 1).with_ability(
        AbilityDef::triggered(
            "Whenever another creature you control dies, put a +1/+1 counter on this creature.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// ISD 40 — Urgent Exorcism
pub(in crate::card::sets) static URGENT_EXORCISM: CardRecord = CardRecord::new(
    cards::URGENT_EXORCISM,
    "Urgent Exorcism",
    CardArt::new("516a437c-a2ee-43c6-876c-1a63a455c97c", "Svetlin Velinov"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Destroy target Spirit or enchantment.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Subtype("Spirit"),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            can_regenerate: true,
        },
    )]),
);

// ISD 41 — Village Bell-Ringer
pub(in crate::card::sets) static VILLAGE_BELL_RINGER: CardRecord = CardRecord::new(
    cards::VILLAGE_BELL_RINGER,
    "Village Bell-Ringer",
    CardArt::new("cb6912b3-bab9-4937-afdd-3711e6d792a0", "David Palumbo"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Scout"], 1, 4).with_abilities(&[
        abilities::flash(),
        AbilityDef::triggered(
            "When this creature enters, untap all creatures you control.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::Untap {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
            },
        ),
    ]),
);

// ISD 42 — Voiceless Spirit
pub(in crate::card::sets) static VOICELESS_SPIRIT: CardRecord = CardRecord::new(
    cards::VOICELESS_SPIRIT,
    "Voiceless Spirit",
    CardArt::new("d24d9bd7-5721-4436-a86f-35e376727f46", "Daarken"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Spirit"], 2, 1)
        .with_abilities(&[abilities::flying(), abilities::first_strike()]),
);

// ISD 43 — Armored Skaab
pub(in crate::card::sets) static ARMORED_SKAAB: CardRecord = CardRecord::new(
    cards::ARMORED_SKAAB,
    "Armored Skaab",
    CardArt::new("ce4d00f2-30e6-41d5-b997-c66350fe783c", "Volkan Baǵa"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Zombie", "Warrior"], 1, 4).with_ability(
        AbilityDef::triggered(
            "When this creature enters, mill four cards.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::Mill {
                player: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(4),
            },
        ),
    ),
);

// ISD 44 — Back from the Brink
// Audit: blocked — Needs a graveyard creature-card cost and a token-copy effect carrying the exiled card's copiable values.

// ISD 45 — Battleground Geist
pub(in crate::card::sets) static BATTLEGROUND_GEIST: CardRecord = CardRecord::new(
    cards::BATTLEGROUND_GEIST,
    "Battleground Geist",
    CardArt::new("129905ef-5b3b-4860-923c-109a7d7cad80", "Clint Cearley"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Spirit"], 3, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "Other Spirit creatures you control get +1/+0.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Spirit"),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
            },
        ),
    ]),
);

// ISD 46 — Cackling Counterpart
// Audit: blocked — Needs creation of a token with the target creature's copiable values.

// ISD 47 — Civilized Scholar
// Audit: blocked — Needs a discard choice linked to a creature-card test, conditional untap, and transform continuation.

// ISD 48 — Claustrophobia
pub(in crate::card::sets) static CLAUSTROPHOBIA: CardRecord = CardRecord::new(
    cards::CLAUSTROPHOBIA,
    "Claustrophobia",
    CardArt::new("b7e5f600-4d19-42a4-b57e-650c76041798", "Ryan Pancoast"),
    CardSet::Innistrad,
    CardRules::new_enchantment(mana_cost!("{1}{U}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::triggered(
                "When this Aura enters, tap enchanted creature.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
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

static CURIOSITY_GRANTED_ABILITY: AbilityDef = AbilityDef::triggered(
    "Whenever this creature deals damage to an opponent, you may draw a card.",
    TriggerEventDef::damage_to_player(ObjectPredicateDef::Source, PlayerRelation::Opponent),
    EffectDef::May {
        player: EffectRecipientDef::Controller,
        effect: &EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    },
);

// ISD 49 — Curiosity
pub(in crate::card::sets) static CURIOSITY: CardRecord = CardRecord::new(
    cards::CURIOSITY,
    "Curiosity",
    CardArt::new("b212c36a-6d1f-4217-b384-1c2b0e07b68a", "Igor Kieryluk"),
    CardSet::Innistrad,
    CardRules::new_enchantment(mana_cost!("{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Whenever enchanted creature deals damage to an opponent, you may draw a card.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&CURIOSITY_GRANTED_ABILITY),
                },
            ),
        ]),
);

// ISD 50 — Curse of the Bloody Tome
// Audit: blocked — Needs an Aura that targets and remains attached to a player, then derives upkeep from that player.

// ISD 51 — Delver of Secrets
// Audit: blocked — Needs the optional top-card reveal procedure and a conditional transform based on the revealed card's type.

// ISD 52 — Deranged Assistant
// Audit: blocked — Needs milling a card as an activation cost of a mana ability.

// ISD 53 — Dissipate
pub(in crate::card::sets) static DISSIPATE: CardRecord = CardRecord::new(
    cards::DISSIPATE,
    "Dissipate",
    CardArt::new("5d778082-bcdb-423a-b16f-57ac0d4dace7", "Tomasz Jedruszek"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{1}{U}{U}")).with_ability(
        AbilityDef::spell_with_targets(
            "Counter target spell. If that spell is countered this way, exile it instead of putting it into its owner's graveyard.",
            &[AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::Any)],
            EffectDef::Counter {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Exile,
            },
        ),
    ),
);

// ISD 54 — Dream Twist
pub(in crate::card::sets) static DREAM_TWIST: CardRecord = CardRecord::new(
    cards::DREAM_TWIST,
    "Dream Twist",
    CardArt::new("d5dd8790-bfdf-427d-8e8d-a5c3a64a3063", "Dan Murayama Scott"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target player mills three cards.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Mill {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
            },
        ),
        abilities::flashback(mana_cost!("{1}{U}")),
    ]),
);

static FORBIDDEN_ALCHEMY_SELECTION: TopCardSelectionDef = TopCardSelectionDef {
    count: ValueDef::Constant(4),
    object: None,
    minimum: 1,
    maximum: 1,
    reveal_selected: false,
    selected_zone: ZoneKind::Hand,
    selected_placement: ZonePlacement::Top,
    rest_zone: ZoneKind::Graveyard,
    rest_placement: ZonePlacement::Top,
    then: None,
};

// ISD 55 — Forbidden Alchemy
pub(in crate::card::sets) static FORBIDDEN_ALCHEMY: CardRecord = CardRecord::new(
    cards::FORBIDDEN_ALCHEMY,
    "Forbidden Alchemy",
    CardArt::new("eb22ae62-6207-4693-87cf-7adf0fc1fe29", "David Rapoza"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{2}{U}")).with_abilities(&[
        AbilityDef::spell(
            "Look at the top four cards of your library. Put one of them into your hand and the rest into your graveyard.",
            EffectDef::LookAtTopAndSelect {
                player: EffectRecipientDef::Controller,
                selection: &FORBIDDEN_ALCHEMY_SELECTION,
            },
        ),
        abilities::flashback(mana_cost!("{6}{B}")),
    ]),
);

// ISD 56 — Fortress Crab
pub(in crate::card::sets) static FORTRESS_CRAB: CardRecord = CardRecord::new(
    cards::FORTRESS_CRAB,
    "Fortress Crab",
    CardArt::new("87ca16d4-089f-42a7-a648-55301a77faea", "Vincent Proce"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Crab"], 1, 6),
);

// ISD 57 — Frightful Delusion
// Audit: blocked — Needs a post-payment continuation so the discard occurs only after the counter-unless-payment decision finishes.

// ISD 58 — Grasp of Phantoms
pub(in crate::card::sets) static GRASP_OF_PHANTOMS: CardRecord = CardRecord::new(
    cards::GRASP_OF_PHANTOMS,
    "Grasp of Phantoms",
    CardArt::new("02655d3d-82d0-4be6-bb64-25e1478edfc3", "Izzy"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{3}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Put target creature on top of its owner's library.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Library,
                controller: None,
                placement: ZonePlacement::Top,
            },
        ),
        abilities::flashback(mana_cost!("{7}{U}")),
    ]),
);

// ISD 59 — Hysterical Blindness
pub(in crate::card::sets) static HYSTERICAL_BLINDNESS: CardRecord = CardRecord::new(
    cards::HYSTERICAL_BLINDNESS,
    "Hysterical Blindness",
    CardArt::new("5aeaa757-e3b0-4606-a689-e8a20a686c3a", "Wayne England"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell(
        "Creatures your opponents control get -4/-0 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::Opponent,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(-4),
                ValueDef::Constant(0),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// ISD 60 — Invisible Stalker
pub(in crate::card::sets) static INVISIBLE_STALKER: CardRecord = CardRecord::new(
    cards::INVISIBLE_STALKER,
    "Invisible Stalker",
    CardArt::new("0013620d-8e17-4246-86bf-71eafd51b806", "Bud Cook"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Human", "Rogue"], 1, 1).with_abilities(&[
        abilities::hexproof(),
        AbilityDef::static_ability(
            "This creature can't be blocked.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBeBlockedBy(
                    ObjectPredicateDef::Any,
                )),
            },
        ),
    ]),
);

// ISD 61 — Laboratory Maniac
// Audit: blocked — Needs a replacement for an empty-library draw that wins the game instead.

// ISD 62 — Lantern Spirit
pub(in crate::card::sets) static LANTERN_SPIRIT: CardRecord = CardRecord::new(
    cards::LANTERN_SPIRIT,
    "Lantern Spirit",
    CardArt::new("b50a5772-f411-458a-97f9-9f3967bb79c5", "Johann Bodin"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Spirit"], 2, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{U}: Return this creature to its owner's hand.",
            &[AbilityCostDef::Mana(mana_cost!("{U}"))],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Source,
                zone: ZoneKind::Hand,
                controller: None,
                placement: ZonePlacement::Top,
            },
        ),
    ]),
);

static LOST_IN_THE_MIST_TARGETS: [AbilityTargetDef; 2] = [
    AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::Any),
    AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Any),
];

// ISD 63 — Lost in the Mist
pub(in crate::card::sets) static LOST_IN_THE_MIST: CardRecord = CardRecord::new(
    cards::LOST_IN_THE_MIST,
    "Lost in the Mist",
    CardArt::new("1e5fc39d-590a-436b-ab90-a1741d2ae3da", "David Palumbo"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{3}{U}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target spell. Return target permanent to its owner's hand.",
        &LOST_IN_THE_MIST_TARGETS,
        EffectDef::Sequence(&[
            EffectDef::Counter {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Graveyard,
            },
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex(1)),
                zone: ZoneKind::Hand,
                controller: None,
                placement: ZonePlacement::Top,
            },
        ]),
    )),
);

// ISD 64 — Ludevic's Test Subject
// Audit: blocked — Needs hatchling counters and an activation continuation that removes all five before transforming.

/// "As an additional cost to cast this spell, exile a creature card from your
/// graveyard."
static EXILE_A_CREATURE_CARD: SpellAdditionalCostDef = SpellAdditionalCostDef {
    object: ObjectPredicateDef::HasType(CardType::Creature),
    zone: ZoneKind::Graveyard,
    count: 1,
};

// ISD 65 — Makeshift Mauler
pub(in crate::card::sets) static MAKESHIFT_MAULER: CardRecord = CardRecord::new(
    cards::MAKESHIFT_MAULER,
    "Makeshift Mauler",
    CardArt::new("d869de57-9454-47ff-af14-eaefd387047a", "James Ryman"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Zombie", "Horror"], 4, 5).with_abilities(&[
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, exile a creature card from your \
             graveyard.",
            &[],
            EXILE_A_CREATURE_CARD,
            EffectDef::None,
        ),
    ]),
);

// ISD 66 — Memory's Journey
// Audit: blocked — Needs a linked target-player relation for up to three cards in that player's graveyard, then shuffling that library.

// ISD 67 — Mindshrieker
// Audit: blocked — Needs the mana value of the specific milled card as a linked temporary P/T value.

// ISD 68 — Mirror-Mad Phantasm
// Audit: blocked — Needs self-shuffle followed by reveal-until-name and separate placement of the named and other revealed cards.

// ISD 69 — Moon Heron
pub(in crate::card::sets) static MOON_HERON: CardRecord = CardRecord::new(
    cards::MOON_HERON,
    "Moon Heron",
    CardArt::new("a24de601-1d7b-41c4-aba1-fdb6fd8d5251", "Charles Urbach"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Spirit", "Bird"], 3, 2)
        .with_ability(abilities::flying()),
);

// ISD 70 — Murder of Crows
pub(in crate::card::sets) static MURDER_OF_CROWS: CardRecord = CardRecord::new(
    cards::MURDER_OF_CROWS,
    "Murder of Crows",
    CardArt::new("f914f7e4-06fc-4943-8597-b7f834938c00", "Drew Baker"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{3}{U}{U}"), &["Bird"], 4, 4).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever another creature dies, you may draw a card. If you do, discard a card.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Sequence(&[
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::Discard {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                        selection: DiscardSelectionDef::RecipientChooses,
                    },
                ]),
            },
        ),
    ]),
);

// ISD 71 — Rooftop Storm
// Audit: blocked — Needs a battlefield-wide alternative cost of {0} for Zombie creature spells you cast.

// ISD 72 — Runic Repetition
// Audit: blocked — Needs a predicate identifying exiled cards that have a flashback ability.

// ISD 73 — Selhoff Occultist
pub(in crate::card::sets) static SELHOFF_OCCULTIST: CardRecord = CardRecord::new(
    cards::SELHOFF_OCCULTIST,
    "Selhoff Occultist",
    CardArt::new("aeac4885-bd04-42bd-8e10-06c3efbce108", "Igor Kieryluk"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Rogue"], 2, 3).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever this creature or another creature dies, target player mills a card.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::HasType(CardType::Creature),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Mill {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// ISD 74 — Sensory Deprivation
pub(in crate::card::sets) static SENSORY_DEPRIVATION: CardRecord = CardRecord::new(
    cards::SENSORY_DEPRIVATION,
    "Sensory Deprivation",
    CardArt::new("454739db-a3d6-45e8-849a-287438c36627", "Steven Belledin"),
    CardSet::Innistrad,
    CardRules::new_enchantment(mana_cost!("{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets -3/-0.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-3),
                        ValueDef::Constant(0),
                    ),
                },
            ),
        ]),
);

// ISD 75 — Silent Departure
pub(in crate::card::sets) static SILENT_DEPARTURE: CardRecord = CardRecord::new(
    cards::SILENT_DEPARTURE,
    "Silent Departure",
    CardArt::new("a18dea16-d535-4310-94ff-836645253d73", "John Avon"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Return target creature to its owner's hand.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                controller: None,
                placement: ZonePlacement::Top,
            },
        ),
        abilities::flashback(mana_cost!("{4}{U}")),
    ]),
);

// ISD 76 — Skaab Goliath
// Audit: blocked — Needs selecting and exiling two creature cards from your graveyard as an additional casting cost.

// ISD 77 — Skaab Ruinator
// Audit: blocked — Needs a three-card graveyard exile casting cost and permission to cast this card from your graveyard.

// ISD 78 — Snapcaster Mage
pub(in crate::card::sets) static SNAPCASTER_MAGE: CardRecord = CardRecord::new(
    cards::SNAPCASTER_MAGE,
    "Snapcaster Mage",
    CardArt::new("9e5b279e-4670-4a1e-87d0-3cab7e4f9e58", "Volkan Baǵa"),
    CardSet::Innistrad,
    CardRules::new_creature(
        mana_cost!("{1}{U}"),
        &["Human", "Wizard"],
        2,
        1,
    )
    .with_abilities(&[
        abilities::flash(),
        AbilityDef::triggered_with_targets("When this creature enters, target instant or sorcery card in your graveyard gains flashback until end of turn. The flashback cost is equal to its mana cost. (You may cast that card from your graveyard for its flashback cost. Then exile it.)", TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)), &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::HasType(CardType::Sorcery),
                ]),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            },
        )], EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(
                    &abilities::flashback_for_card_mana_cost(),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            }),
    ]),
);

// ISD 79 — Spectral Flight
pub(in crate::card::sets) static SPECTRAL_FLIGHT: CardRecord = CardRecord::new(
    cards::SPECTRAL_FLIGHT,
    "Spectral Flight",
    CardArt::new("f7149f2a-6917-4ad7-8035-c7a1babd4d4b", "Johann Bodin"),
    CardSet::Innistrad,
    CardRules::new_enchantment(mana_cost!("{1}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+2 and has flying.",
                EffectDef::Sequence(&[
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(2),
                        ),
                    },
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::add_ability(&abilities::flying()),
                    },
                ]),
            ),
        ]),
);

// ISD 80 — Stitched Drake
pub(in crate::card::sets) static STITCHED_DRAKE: CardRecord = CardRecord::new(
    cards::STITCHED_DRAKE,
    "Stitched Drake",
    CardArt::new("ad81266a-488f-449a-9daf-637727564865", "Chris Rahn"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{1}{U}{U}"), &["Zombie", "Drake"], 3, 4).with_abilities(&[
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, exile a creature card from your \
             graveyard.",
            &[],
            EXILE_A_CREATURE_CARD,
            EffectDef::None,
        ),
        abilities::flying(),
    ]),
);

// ISD 81 — Stitcher's Apprentice
pub(in crate::card::sets) static STITCHERS_APPRENTICE: CardRecord = CardRecord::new(
    cards::STITCHERS_APPRENTICE,
    "Stitcher's Apprentice",
    CardArt::new("7e0fcc53-cd0b-4b4c-b6de-5d301232106a", "Johann Bodin"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Homunculus"], 1, 2).with_ability(
        AbilityDef::activated(
            "{1}{U}, {T}: Create a 2/2 blue Homunculus creature token, then sacrifice a creature.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{U}")),
                AbilityCostDef::TapSource,
            ],
            EffectDef::Sequence(&[
                EffectDef::CreateToken {
                    token: cards::HOMUNCULUS_TOKEN_2_2_BLUE,
                    count: ValueDef::Constant(1),
                    tapped: false,
                },
                EffectDef::SacrificeOfChoice {
                    player: EffectRecipientDef::Controller,
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    then: None,
                    optional: false,
                },
            ]),
        ),
    ),
);

// ISD 82 — Sturmgeist
// Audit: blocked — Needs the exact number of cards in your hand as a continuously evaluated P/T value.

// ISD 83 — Think Twice
pub(in crate::card::sets) static THINK_TWICE: CardRecord = CardRecord::new(
    cards::THINK_TWICE,
    "Think Twice",
    CardArt::new("53e44060-a9a2-4095-9f5b-f60297525315", "Anthony Francisco"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[
        AbilityDef::spell(
            "Draw a card.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        abilities::flashback(mana_cost!("{2}{U}")),
    ]),
);

// ISD 84 — Undead Alchemist
// Audit: blocked — Needs a combat-damage replacement that mills instead, plus a linked library-to-graveyard creature-card trigger.

// ISD 85 — Abattoir Ghoul
// Audit: blocked — Needs the last known toughness of the creature that died after being damaged by this creature.

static SACRIFICE_A_CREATURE: SpellAdditionalCostDef = SpellAdditionalCostDef {
    object: ObjectPredicateDef::HasType(CardType::Creature),
    zone: ZoneKind::Battlefield,
    count: 1,
};

// ISD 86 — Altar's Reap
pub(in crate::card::sets) static ALTARS_REAP: CardRecord = CardRecord::new(
    cards::ALTARS_REAP,
    "Altar's Reap",
    CardArt::new("4dc2eec4-7e68-45d5-8736-6b32a47c671b", "Donato Giancola"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{1}{B}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, sacrifice a creature.\nDraw two cards.",
            &[],
            SACRIFICE_A_CREATURE,
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
    ),
);

// ISD 87 — Army of the Damned
// Audit: blocked — Needs token creation that puts thirteen Zombie tokens onto the battlefield tapped.

// ISD 88 — Bitterheart Witch
// Audit: blocked — Needs searching for a Curse and putting it onto the battlefield attached to a targeted player.

// ISD 89 — Bloodgift Demon
pub(in crate::card::sets) static BLOODGIFT_DEMON: CardRecord = CardRecord::new(
    cards::BLOODGIFT_DEMON,
    "Bloodgift Demon",
    CardArt::new("f271addb-e267-4397-b181-f1eaeabbfe71", "Peter Mohrbacher"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Demon"], 5, 4).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "At the beginning of your upkeep, target player draws a card and loses 1 life.",
            TriggerEventDef::StepBegins {
                step: crate::card::TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Sequence(&[
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                },
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
    ]),
);

// ISD 90 — Bloodline Keeper
// Audit: blocked — Needs an activation restriction based on controlling five Vampires before transforming.

// ISD 91 — Brain Weevil
// Audit: blocked — Needs the “activate only as a sorcery” timing restriction on its sacrifice-and-discard ability.

// ISD 92 — Bump in the Night
pub(in crate::card::sets) static BUMP_IN_THE_NIGHT: CardRecord = CardRecord::new(
    cards::BUMP_IN_THE_NIGHT,
    "Bump in the Night",
    CardArt::new("5c3ec389-a267-484f-994d-4a29ef494eb1", "Kev Walker"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target opponent loses 3 life.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
            )],
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
            },
        ),
        abilities::flashback(mana_cost!("{5}{R}")),
    ]),
);

// ISD 93 — Corpse Lunge
// Audit: blocked — Needs an exiled graveyard card as a casting cost and its linked power as the spell's damage amount.

// ISD 94 — Curse of Death's Hold
// Audit: blocked — Needs an Aura attached to a player and a static effect over creatures that enchanted player controls.

// ISD 95 — Curse of Oblivion
// Audit: blocked — Needs a player Aura whose upkeep trigger makes that player choose two cards in their graveyard to exile.

// ISD 96 — Dead Weight
pub(in crate::card::sets) static DEAD_WEIGHT: CardRecord = CardRecord::new(
    cards::DEAD_WEIGHT,
    "Dead Weight",
    CardArt::new("7933987e-7b8c-4d5a-804a-708d6bb6d231", "Randy Gallegos"),
    CardSet::Innistrad,
    CardRules::new_enchantment(mana_cost!("{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets -2/-2.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-2),
                        ValueDef::Constant(-2),
                    ),
                },
            ),
        ]),
);

// ISD 97 — Diregraf Ghoul
pub(in crate::card::sets) static DIREGRAF_GHOUL: CardRecord = CardRecord::new(
    cards::DIREGRAF_GHOUL,
    "Diregraf Ghoul",
    CardArt::new("4ed5790a-3354-49c2-89b6-3fc0de8dcc7c", "Dave Kendall"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{B}"), &["Zombie"], 2, 2)
        .with_ability(abilities::enters_tapped("This creature enters tapped.")),
);

// ISD 98 — Disciple of Griselbrand
// Audit: blocked — Needs the last known toughness of a creature selected and sacrificed as an activation cost.

// ISD 99 — Endless Ranks of the Dead
// Audit: blocked — Needs integer division of the current Zombie count when determining how many tokens to create.

// ISD 100 — Falkenrath Noble
pub(in crate::card::sets) static FALKENRATH_NOBLE: CardRecord = CardRecord::new(
    cards::FALKENRATH_NOBLE,
    "Falkenrath Noble",
    CardArt::new("e2286f94-4cf9-4462-b5d7-cee7f6910018", "Slawomir Maniak"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Vampire", "Noble"], 2, 2)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::triggered_with_targets(
                "Whenever this creature or another creature dies, target player loses 1 life and you gain 1 life.",
                TriggerEventDef::zone_changed(ObjectPredicateDef::HasType(CardType::Creature), Some(ZoneKind::Battlefield), Some(ZoneKind::Graveyard)),
                &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
                    PlayerRelation::Any,
                ))],
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

static GHOULCALLERS_CHANT_ONE_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Graveyard],
        controller: None,
        owner: Some(PlayerRelation::You),
    },
)];

static GHOULCALLERS_CHANT_TWO_TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef {
    predicate: AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Subtype("Zombie"),
        ]),
        zones: &[ZoneKind::Graveyard],
        controller: None,
        owner: Some(PlayerRelation::You),
    },
    minimum: 2,
    maximum: 2,
    divided_total: None,
}];

static GHOULCALLERS_CHANT_MODES: [AbilityDef; 2] = [
    AbilityDef::spell_with_targets(
        "Return target creature card from your graveyard to your hand.",
        &GHOULCALLERS_CHANT_ONE_TARGET,
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Hand,
            controller: None,
            placement: ZonePlacement::Top,
        },
    ),
    AbilityDef::spell_with_targets(
        "Return two target Zombie cards from your graveyard to your hand.",
        &GHOULCALLERS_CHANT_TWO_TARGETS,
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Hand,
            controller: None,
            placement: ZonePlacement::Top,
        },
    ),
];

// ISD 101 — Ghoulcaller's Chant
pub(in crate::card::sets) static GHOULCALLERS_CHANT: CardRecord = CardRecord::new(
    cards::GHOULCALLERS_CHANT,
    "Ghoulcaller's Chant",
    CardArt::new("2b8c1b10-2155-404a-8f20-eb8f643849d6", "Randy Gallegos"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{B}")).with_ability(AbilityDef::choose_one_spell(
        "Choose one — Return target creature card from your graveyard to your hand; or return two target Zombie cards from your graveyard to your hand.",
        &GHOULCALLERS_CHANT_MODES,
    )),
);

// ISD 102 — Ghoulraiser
// Audit: blocked — Needs deterministic random selection of a Zombie card from your graveyard.

// ISD 103 — Gruesome Deformity
pub(in crate::card::sets) static GRUESOME_DEFORMITY: CardRecord = CardRecord::new(
    cards::GRUESOME_DEFORMITY,
    "Gruesome Deformity",
    CardArt::new("5696db03-206f-4e7e-9b65-ccef31bfd7d2", "Matt Stewart"),
    CardSet::Innistrad,
    CardRules::new_enchantment(mana_cost!("{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature has intimidate.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::intimidate()),
                },
            ),
        ]),
);

// ISD 104 — Heartless Summoning
// Audit: blocked — Needs a battlefield-wide generic cost reduction for creature spells you cast.

static LILIANA_ULTIMATE_RESOLVER: CardAbilityResolver = CardAbilityResolver::new(
    "innistrad/liliana-of-the-veil/ultimate",
    resolve_liliana_ultimate,
);

const LILIANA_ULTIMATE_ABILITY: AbilityDef = AbilityDef::activated_with_targets(
    "−6: Separate all permanents target player controls into two piles. That player sacrifices all permanents in the pile of their choice.",
    &[AbilityCostDef::Loyalty(-6)],
    &[AbilityTargetDef::exactly_one(
        AbilityTargetPredicate::Player(PlayerRelation::Any),
    )],
    EffectDef::None,
)
.with_effect_execution(EffectExecutionDef::CardOwned)
.with_coverage(AbilityCoverageDef::explained_complete(
    "Pile separation, pile choice, and the chosen-pile sacrifice are composed by Liliana's card-owned resolver from shared runtime primitives.",
));

static LILIANA_ABILITY_BINDINGS: [CardAbilityBinding; 1] = [CardAbilityBinding::new(
    CardPartId::PRIMARY,
    AbilityId(2),
    LILIANA_ULTIMATE_ABILITY,
    &LILIANA_ULTIMATE_RESOLVER,
)
.with_policy_hint(
    AbilityPolicyHint::TargetPlayerSacrificesOneOfTwoPermanentPiles {
        target: TargetSlotId(0),
    },
)];

fn resolve_liliana_ultimate(runtime: &mut CardRuntime<'_>, ability: &ResolvedAbility) {
    let Some(victim) = ability.target_player(TargetIndex::PRIMARY) else {
        return;
    };
    let permanents = runtime.controlled_permanents(victim);
    runtime.queue_permanent_partition(
        ability.controller(),
        ability.controller(),
        victim,
        &permanents,
        LILIANA_PILES_SEPARATED,
    );
}

pub(in crate::card) static LILIANA_PILES_SEPARATED: PilesSeparated =
    PilesSeparated::new("lilianaOfTheVeil.pilesSeparated", liliana_piles_separated);

pub(in crate::card) static LILIANA_PILE_CHOSEN: PileChosen =
    PileChosen::new("lilianaOfTheVeil.pileChosen", liliana_pile_chosen);

fn liliana_piles_separated(runtime: &mut CardRuntime<'_>, piles: PileSplit) {
    let victim = piles.subject();
    runtime.queue_pile_choice(
        victim,
        piles,
        "Choose a pile to sacrifice",
        "Sacrifice pile",
        LILIANA_PILE_CHOSEN,
    );
}

fn liliana_pile_chosen(runtime: &mut CardRuntime<'_>, choice: PileChoice) {
    let victim = choice.subject();
    let resolving_controller = choice.resolving_controller();
    let (chosen, _unchosen) = choice.into_parts();
    runtime.sacrifice_permanents_simultaneously(&chosen, victim, resolving_controller);
}

// ISD 105 — Liliana of the Veil
pub(in crate::card::sets) static LILIANA_OF_THE_VEIL: CardRecord = CardRecord::new(
    cards::LILIANA_OF_THE_VEIL,
    "Liliana of the Veil",
    CardArt::new("ac506c17-adc8-49c6-9d8d-43db7cb1ec9d", "Steve Argyle"),
    CardSet::Innistrad,
    CardRules::new_planeswalker(mana_cost!("{1}{B}{B}"), &["Liliana"], 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated(
                "+1: Each player discards a card.",
                &[AbilityCostDef::Loyalty(1)],
                EffectDef::Discard {
                    recipient: EffectRecipientDef::EachPlayer,
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                },
            ),
            AbilityDef::activated_with_targets(
                "−2: Target player sacrifices a creature.",
                &[AbilityCostDef::Loyalty(-2)],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::SacrificeOfChoice {
                    player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    then: None,
                    optional: false,
                },
            ),
            LILIANA_ULTIMATE_ABILITY,
        ]),
)
.with_ability_bindings(&LILIANA_ABILITY_BINDINGS);

// ISD 106 — Manor Skeleton
pub(in crate::card::sets) static MANOR_SKELETON: CardRecord = CardRecord::new(
    cards::MANOR_SKELETON,
    "Manor Skeleton",
    CardArt::new("e7b45197-d5c2-48c8-b72e-00236552e338", "Eric Deschamps"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Skeleton"], 1, 1).with_abilities(&[
        abilities::haste(),
        abilities::regenerate_self(
            "{1}{B}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{B}"))],
        ),
    ]),
);

// ISD 107 — Markov Patrician
pub(in crate::card::sets) static MARKOV_PATRICIAN: CardRecord = CardRecord::new(
    cards::MARKOV_PATRICIAN,
    "Markov Patrician",
    CardArt::new(
        "29c3d3f7-5e28-4fec-8422-87856fcd1e8e",
        "Jana Schirmer & Johannes Voss",
    ),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Vampire"], 3, 1)
        .with_ability(abilities::lifelink()),
);

// ISD 108 — Maw of the Mire
pub(in crate::card::sets) static MAW_OF_THE_MIRE: CardRecord = CardRecord::new(
    cards::MAW_OF_THE_MIRE,
    "Maw of the Mire",
    CardArt::new("90b34a03-3270-412c-90ca-03c1b3e61222", "Vincent Proce"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{4}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target land. You gain 4 life.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Land),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(4),
            },
        ]),
    )),
);

// ISD 109 — Moan of the Unhallowed
pub(in crate::card::sets) static MOAN_OF_THE_UNHALLOWED: CardRecord = CardRecord::new(
    cards::MOAN_OF_THE_UNHALLOWED,
    "Moan of the Unhallowed",
    CardArt::new("3e2c5a8f-c03a-40ab-8390-ff6b5b654717", "Nils Hamm"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{2}{B}{B}")).with_abilities(&[
        AbilityDef::spell(
            "Create two 2/2 black Zombie creature tokens.",
            EffectDef::CreateToken {
                token: cards::ZOMBIE_TOKEN_2_2_BLACK,
                count: ValueDef::Constant(2),
                tapped: false,
            },
        ),
        abilities::flashback(mana_cost!("{5}{B}{B}")),
    ]),
);

// ISD 110 — Morkrut Banshee
// Audit: blocked — Needs an intervening morbid condition that suppresses both the ETB trigger and its target when no creature died.

// ISD 111 — Night Terrors
// Audit: blocked — Needs revealing another player's hand, choosing a nonland card from it, and exiling that choice.

// ISD 112 — Reaper from the Abyss
// Audit: blocked — Needs a morbid intervening-if check on each end step before choosing a non-Demon target.

// ISD 113 — Rotting Fensnake
pub(in crate::card::sets) static ROTTING_FENSNAKE: CardRecord = CardRecord::new(
    cards::ROTTING_FENSNAKE,
    "Rotting Fensnake",
    CardArt::new("c21cbb10-9157-4887-a752-29b9e94fc77a", "Tomasz Jedruszek"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Zombie", "Snake"], 5, 1),
);

#[allow(clippy::large_types_passed_by_value)]
fn two_face_creature_composition(
    front_name: &'static str,
    back_name: &'static str,
    front: CardRules,
    back: CardRules,
    mana_cost: crate::card::ManaCost,
) -> CardComposition {
    CardComposition {
        parts: vec![
            CardPart::new(CardPartId::PRIMARY, front_name, front),
            CardPart::new(CardPartId(1), back_name, back),
        ],
        structure: CardStructure::DoubleFaced {
            front: CardPartId::PRIMARY,
            back: CardPartId(1),
            kind: DoubleFacedKind::Transforming,
        },
        play_options: vec![PlayOptionDef::cast(
            PlayOptionId::DEFAULT,
            front_name,
            SpellForm::Part(CardPartId::PRIMARY),
            mana_cost,
            CardEffectStatus::Implemented,
        )],
    }
}

static SCREECHING_BAT_TRANSFORM: EffectDef = EffectDef::Transform {
    object: EffectRecipientDef::Source,
};
static SCREECHING_BAT_UPKEEP_ABILITY: AbilityDef = AbilityDef::triggered(
    "At the beginning of your upkeep, you may pay {2}{B}{B}. If you do, transform this creature.",
    TriggerEventDef::StepBegins {
        step: crate::card::TurnStepDef::Upkeep,
        player: PlayerRelation::You,
    },
    EffectDef::PayOr(PayOrDef::optional(
        EffectPaymentDef::mana(
            PlayerSetDef::Related(PlayerRelation::You),
            mana_cost!("{2}{B}{B}"),
        ),
        &SCREECHING_BAT_TRANSFORM,
    )),
);
static SCREECHING_BAT_FRONT_ABILITIES: [AbilityDef; 2] =
    [abilities::flying(), SCREECHING_BAT_UPKEEP_ABILITY];

const fn screeching_bat_front_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Bat"], 2, 2)
        .with_abilities(&SCREECHING_BAT_FRONT_ABILITIES)
}

const fn stalking_vampire_rules() -> CardRules {
    CardRules::new_creature_without_mana_cost(&["Vampire"], 5, 5)
        .printed_colors(&[ManaColor::Black])
        .with_ability(SCREECHING_BAT_UPKEEP_ABILITY)
}

fn screeching_bat_composition() -> CardComposition {
    two_face_creature_composition(
        "Screeching Bat",
        "Stalking Vampire",
        screeching_bat_front_rules(),
        stalking_vampire_rules(),
        mana_cost!("{2}{B}"),
    )
}

// ISD 114 — Screeching Bat
pub(in crate::card::sets) static SCREECHING_BAT: CardRecord = CardRecord::new(
    cards::SCREECHING_BAT,
    "Screeching Bat",
    CardArt::new("88db324f-11f1-43d3-a897-f4e3caf8d642", "Slawomir Maniak"),
    CardSet::Innistrad,
    screeching_bat_front_rules(),
)
.with_composition(screeching_bat_composition);

// ISD 115 — Sever the Bloodline
pub(in crate::card::sets) static SEVER_THE_BLOODLINE: CardRecord = CardRecord::new(
    cards::SEVER_THE_BLOODLINE,
    "Sever the Bloodline",
    CardArt::new("5c6da820-dfb9-4b61-aff8-56dfc9f4894e", "Clint Cearley"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{3}{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Exile target creature and all other creatures with the same name as that creature.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::ObjectsSharingNameWithTarget(TargetIndex::PRIMARY),
                zone: ZoneKind::Exile,
                controller: None,
                placement: ZonePlacement::Top,
            },
        ),
        abilities::flashback(mana_cost!("{5}{B}{B}")),
    ]),
);

// ISD 116 — Skeletal Grimace
pub(in crate::card::sets) static SKELETAL_GRIMACE: CardRecord = CardRecord::new(
    cards::SKELETAL_GRIMACE,
    "Skeletal Grimace",
    CardArt::new("b9b28f37-d6b8-4d35-95e9-9533aea0a071", "Eric Deschamps"),
    CardSet::Innistrad,
    CardRules::new_enchantment(mana_cost!("{1}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature gets +1/+1 and has \"{B}: Regenerate this creature.\"",
                EffectDef::Sequence(&[
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                    },
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::add_ability(&abilities::regenerate_self(
                            "{B}: Regenerate this creature.",
                            &[AbilityCostDef::Mana(mana_cost!("{B}"))],
                        )),
                    },
                ]),
            ),
        ]),
);

// ISD 117 — Skirsdag High Priest
// Audit: blocked — Needs tapping two separately chosen other creatures as an activation cost and a morbid activation restriction.

// ISD 118 — Stromkirk Patrol
pub(in crate::card::sets) static STROMKIRK_PATROL: CardRecord = CardRecord::new(
    cards::STROMKIRK_PATROL,
    "Stromkirk Patrol",
    CardArt::new("d86634a1-7016-4500-8857-924d51857bad", "Karl Kopinski"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Vampire", "Soldier"], 4, 3).with_ability(
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, put a +1/+1 counter on it.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// ISD 119 — Tribute to Hunger
// Audit: blocked — Needs the last known toughness of the creature the targeted opponent chooses to sacrifice.

// ISD 120 — Typhoid Rats
pub(in crate::card::sets) static TYPHOID_RATS: CardRecord = CardRecord::new(
    cards::TYPHOID_RATS,
    "Typhoid Rats",
    CardArt::new("4490ce65-c73a-4809-abd1-ccc3175bd2a4", "Kev Walker"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{B}"), &["Rat"], 1, 1)
        .with_ability(abilities::deathtouch()),
);

// ISD 121 — Unbreathing Horde
// Audit: blocked — Needs a dynamic enters-with-counters count and a damage replacement that removes a counter instead.

// ISD 122 — Unburial Rites
pub(in crate::card::sets) static UNBURIAL_RITES: CardRecord = CardRecord::new(
    cards::UNBURIAL_RITES,
    "Unburial Rites",
    CardArt::new("2794c82b-e5ce-4369-894e-bf56c6402ae1", "Ryan Pancoast"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{4}{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Return target creature card from your graveyard to the battlefield.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Battlefield,
                controller: None,
                placement: ZonePlacement::Top,
            },
        ),
        abilities::flashback(mana_cost!("{3}{W}")),
    ]),
);

/// Every creature anyone controls, which is what the reduction counts.
static EVERY_CREATURE: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::Any,
);

// ISD 123 — Vampire Interloper
pub(in crate::card::sets) static VAMPIRE_INTERLOPER: CardRecord = CardRecord::new(
    cards::VAMPIRE_INTERLOPER,
    "Vampire Interloper",
    CardArt::new("48105c2e-ee36-4117-b56b-3440298da995", "James Ryman"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Vampire", "Scout"], 2, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "This creature can't block.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBlock),
            },
        ),
    ]),
);

// ISD 124 — Victim of Night
pub(in crate::card::sets) static VICTIM_OF_NIGHT: CardRecord = CardRecord::new(
    cards::VICTIM_OF_NIGHT,
    "Victim of Night",
    CardArt::new("ee4c6135-eee9-43ec-bbe8-76912352dcac", "Winona Nelson"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{B}{B}")).with_ability(AbilityDef::destroy_target(
        "Destroy target non-Vampire, non-Werewolf, non-Zombie creature.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Vampire")),
            ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Werewolf")),
            ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Zombie")),
        ])),
        true,
    )),
);

// ISD 125 — Village Cannibals
pub(in crate::card::sets) static VILLAGE_CANNIBALS: CardRecord = CardRecord::new(
    cards::VILLAGE_CANNIBALS,
    "Village Cannibals",
    CardArt::new("a5400460-da9d-437b-bb81-cf382beb371e", "Bud Cook"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Human"], 2, 2).with_ability(
        AbilityDef::triggered(
            "Whenever another Human creature dies, put a +1/+1 counter on this creature.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Subtype("Human"),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// ISD 126 — Walking Corpse
pub(in crate::card::sets) static WALKING_CORPSE: CardRecord = CardRecord::new(
    cards::WALKING_CORPSE,
    "Walking Corpse",
    CardArt::new("8e033384-3334-4082-9541-f2443d3bc424", "Igor Kieryluk"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Zombie"], 2, 2),
);

// ISD 127 — Ancient Grudge
pub(in crate::card::sets) static ANCIENT_GRUDGE: CardRecord = CardRecord::new(
    cards::ANCIENT_GRUDGE,
    "Ancient Grudge",
    CardArt::new("e5e7b966-7c5b-44e6-a6df-4bd7af4edaa9", "Ryan Yee"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{1}{R}")).with_abilities(&[
        AbilityDef::destroy_target(
            "Destroy target artifact.",
            &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                CardType::Artifact,
            )),
            true,
        ),
        abilities::flashback(mana_cost!("{G}")),
    ]),
);

// ISD 128 — Ashmouth Hound
// Audit: blocked — Needs a combat block/becomes-blocked trigger that identifies each opposing creature.

// ISD 129 — Balefire Dragon
pub(in crate::card::sets) static BALEFIRE_DRAGON: CardRecord = CardRecord::new(
    cards::BALEFIRE_DRAGON,
    "Balefire Dragon",
    CardArt::new("b0dce4ac-f472-4f3b-b01a-eff0902a578f", "Eric Deschamps"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{5}{R}{R}"), &["Dragon"], 6, 6).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, it deals that much damage to each creature that player controls.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::HasType(CardType::Creature), &[ZoneKind::Battlefield], PlayerRelation::Opponent),
                amount: ValueDef::TriggerEventAmount,
            },
        ),
    ]),
);

// ISD 130 — Blasphemous Act
pub(in crate::card::sets) static BLASPHEMOUS_ACT: CardRecord = CardRecord::new(
    cards::BLASPHEMOUS_ACT,
    "Blasphemous Act",
    CardArt::new("509ce648-fb76-486d-8b39-183e368b7cb7", "Daarken"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{8}{R}")).with_abilities(&[
        AbilityDef::static_ability(
            "This spell costs {1} less to cast for each creature on the battlefield.",
            EffectDef::ReduceGenericCostBy(ValueDef::CountMatchingObjects(&EVERY_CREATURE)),
        )
        .with_source_zones(&[ZoneKind::Hand]),
        AbilityDef::spell(
            "Blasphemous Act deals 13 damage to each creature.",
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                amount: ValueDef::Constant(13),
            },
        ),
    ]),
);

// ISD 131 — Bloodcrazed Neonate
pub(in crate::card::sets) static BLOODCRAZED_NEONATE: CardRecord = CardRecord::new(
    cards::BLOODCRAZED_NEONATE,
    "Bloodcrazed Neonate",
    CardArt::new("68d2452e-309d-44ae-9360-9d6e22a15e2b", "Cynthia Sheppard"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Vampire"], 2, 1).with_abilities(&[
        abilities::attacks_each_combat_if_able("This creature attacks each combat if able."),
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, put a +1/+1 counter on it.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

static BRIMSTONE_VOLLEY_AMOUNT: ValueDef = ValueDef::IfCreatureDiedThisTurn(&ConditionalValueDef {
    then: ValueDef::Constant(5),
    otherwise: ValueDef::Constant(3),
});

// ISD 132 — Brimstone Volley
pub(in crate::card::sets) static BRIMSTONE_VOLLEY: CardRecord = CardRecord::new(
    cards::BRIMSTONE_VOLLEY,
    "Brimstone Volley",
    CardArt::new("6960f2da-6b84-4680-8ab2-f0567a5d1b0a", "Eytan Zana"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{2}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "Brimstone Volley deals 3 damage to any target. Morbid — It deals 5 damage instead if a creature died this turn.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: BRIMSTONE_VOLLEY_AMOUNT,
            },
        ),
    ),
);

// ISD 133 — Burning Vengeance
// Audit: blocked — Needs spell-cast events to retain the zone from which the spell was cast.

// ISD 134 — Charmbreaker Devils
// Audit: blocked — Needs deterministic random selection of an instant or sorcery card from your graveyard.

// ISD 135 — Crossway Vampire
pub(in crate::card::sets) static CROSSWAY_VAMPIRE: CardRecord = CardRecord::new(
    cards::CROSSWAY_VAMPIRE,
    "Crossway Vampire",
    CardArt::new("3e7a137f-e19e-43a6-aab8-02b175c9d626", "Mark Evans"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Vampire"], 3, 2).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature enters, target creature can't block this turn.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBlock),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ISD 136 — Curse of Stalked Prey
// Audit: blocked — Needs a player Aura and a combat-damage trigger derived from the enchanted player.

// ISD 137 — Curse of the Nightly Hunt
// Audit: blocked — Needs a player Aura and an attack requirement over creatures that enchanted player controls.

// ISD 138 — Curse of the Pierced Heart
// Audit: blocked — Needs a player Aura whose upkeep trigger derives the enchanted player and their planeswalkers.

// ISD 139 — Desperate Ravings
pub(in crate::card::sets) static DESPERATE_RAVINGS: CardRecord = CardRecord::new(
    cards::DESPERATE_RAVINGS,
    "Desperate Ravings",
    CardArt::new("2ba3ab3e-d16c-492f-a860-6d8efcadf679", "John Stanko"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{1}{R}")).with_abilities(&[
        AbilityDef::spell(
            "Draw two cards, then discard a card at random.",
            EffectDef::Sequence(&[
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::Random,
                },
            ]),
        ),
        abilities::flashback(mana_cost!("{2}{U}")),
    ]),
);

// ISD 140 — Devil's Play
pub(in crate::card::sets) static DEVILS_PLAY: CardRecord = CardRecord::new(
    cards::DEVILS_PLAY,
    "Devil's Play",
    CardArt::new("c80596a4-b464-4b9e-8186-94a1c44838eb", "Austin Hsu"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{X}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Devil's Play deals X damage to any target.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::ChosenX,
            },
        ),
        abilities::flashback(mana_cost!("{X}{R}{R}{R}")),
    ]),
);

// ISD 141 — Falkenrath Marauders
pub(in crate::card::sets) static FALKENRATH_MARAUDERS: CardRecord = CardRecord::new(
    cards::FALKENRATH_MARAUDERS,
    "Falkenrath Marauders",
    CardArt::new("b9c09887-6d2b-48b4-a483-16b8a45babd0", "James Ryman"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Vampire", "Warrior"], 2, 2)
        .with_abilities(&[
        abilities::flying(),
        abilities::haste(),
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, put two +1/+1 counters on it.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(2),
            },
        ),
    ]),
);

// ISD 142 — Feral Ridgewolf
pub(in crate::card::sets) static FERAL_RIDGEWOLF: CardRecord = CardRecord::new(
    cards::FERAL_RIDGEWOLF,
    "Feral Ridgewolf",
    CardArt::new("78c66cc0-cb0f-4daf-8141-0923ad46a834", "Martina Pilcerova"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Wolf"], 1, 2).with_abilities(&[
        abilities::trample(),
        AbilityDef::activated(
            "{1}{R}: This creature gets +2/+0 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{R}"))],
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

// ISD 143 — Furor of the Bitten
pub(in crate::card::sets) static FUROR_OF_THE_BITTEN: CardRecord = CardRecord::new(
    cards::FUROR_OF_THE_BITTEN,
    "Furor of the Bitten",
    CardArt::new("ff4a4c19-6427-4a03-a543-992c910e668f", "Randy Gallegos"),
    CardSet::Innistrad,
    CardRules::new_enchantment(mana_cost!("{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+2 and attacks each combat if able.",
                EffectDef::Sequence(&[
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(2),
                        ),
                    },
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::add_ability(
                            &abilities::attacks_each_combat_if_able(
                                "This creature attacks each combat if able.",
                            ),
                        ),
                    },
                ]),
            ),
        ]),
);

// ISD 144 — Geistflame
pub(in crate::card::sets) static GEISTFLAME: CardRecord = CardRecord::new(
    cards::GEISTFLAME,
    "Geistflame",
    CardArt::new("1b856f31-ac80-4338-95a5-3f8acda74cfe", "Scott Chou"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Geistflame deals 1 damage to any target.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ),
        abilities::flashback(mana_cost!("{3}{R}")),
    ]),
);

static HANWEIR_WATCHKEEP_FRONT_ABILITIES: [AbilityDef; 2] =
    [abilities::defender(), WEREWOLF_FRONT_TRANSFORM];
static BANE_OF_HANWEIR_ABILITIES: [AbilityDef; 2] = [
    abilities::attacks_each_combat_if_able("This creature attacks each combat if able."),
    WEREWOLF_BACK_TRANSFORM,
];

const fn hanweir_watchkeep_front_rules() -> CardRules {
    CardRules::new_creature(
        mana_cost!("{2}{R}"),
        &["Human", "Warrior", "Werewolf"],
        1,
        5,
    )
    .with_abilities(&HANWEIR_WATCHKEEP_FRONT_ABILITIES)
}

const fn bane_of_hanweir_rules() -> CardRules {
    CardRules::new_creature_without_mana_cost(&["Werewolf"], 5, 5)
        .printed_colors(&[ManaColor::Red])
        .with_abilities(&BANE_OF_HANWEIR_ABILITIES)
}

fn hanweir_watchkeep_composition() -> CardComposition {
    two_face_creature_composition(
        "Hanweir Watchkeep",
        "Bane of Hanweir",
        hanweir_watchkeep_front_rules(),
        bane_of_hanweir_rules(),
        mana_cost!("{2}{R}"),
    )
}

// ISD 145 — Hanweir Watchkeep
pub(in crate::card::sets) static HANWEIR_WATCHKEEP: CardRecord = CardRecord::new(
    cards::HANWEIR_WATCHKEEP,
    "Hanweir Watchkeep",
    CardArt::new("2b14ed17-1a35-4c49-ac46-3cad42d46c14", "Wayne Reynolds"),
    CardSet::Innistrad,
    hanweir_watchkeep_front_rules(),
)
.with_composition(hanweir_watchkeep_composition);

// ISD 146 — Harvest Pyre
// Audit: blocked — Needs choosing and exiling X graveyard cards as a casting cost linked to the damage amount.

// ISD 147 — Heretic's Punishment
// Audit: blocked — Needs the greatest mana value among three specifically milled cards.

// ISD 148 — Infernal Plunge
// Audit: blocked — Needs choosing and sacrificing a creature as an additional casting cost.

static INSTIGATOR_GANG_FRONT_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::static_ability(
        "Attacking creatures you control get +1/+0.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Attacking,
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(1),
                ValueDef::Constant(0),
            ),
        },
    ),
    WEREWOLF_FRONT_TRANSFORM,
];
static WILDBLOOD_PACK_ABILITIES: [AbilityDef; 3] = [
    abilities::trample(),
    AbilityDef::static_ability(
        "Attacking creatures you control get +3/+0.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Attacking,
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(3),
                ValueDef::Constant(0),
            ),
        },
    ),
    WEREWOLF_BACK_TRANSFORM,
];

const fn instigator_gang_front_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Human", "Werewolf"], 2, 3)
        .with_abilities(&INSTIGATOR_GANG_FRONT_ABILITIES)
}

const fn wildblood_pack_rules() -> CardRules {
    CardRules::new_creature_without_mana_cost(&["Werewolf"], 5, 5)
        .printed_colors(&[ManaColor::Red])
        .with_abilities(&WILDBLOOD_PACK_ABILITIES)
}

fn instigator_gang_composition() -> CardComposition {
    two_face_creature_composition(
        "Instigator Gang",
        "Wildblood Pack",
        instigator_gang_front_rules(),
        wildblood_pack_rules(),
        mana_cost!("{3}{R}"),
    )
}

// ISD 149 — Instigator Gang
pub(in crate::card::sets) static INSTIGATOR_GANG: CardRecord = CardRecord::new(
    cards::INSTIGATOR_GANG,
    "Instigator Gang",
    CardArt::new("bb90a6f1-c7f2-4c2e-ab1e-59c5c7937841", "Greg Staples"),
    CardSet::Innistrad,
    instigator_gang_front_rules(),
)
.with_composition(instigator_gang_composition);

// ISD 150 — Into the Maw of Hell
pub(in crate::card::sets) static INTO_THE_MAW_OF_HELL: CardRecord = CardRecord::new(
    cards::INTO_THE_MAW_OF_HELL,
    "Into the Maw of Hell",
    CardArt::new("5d188d9b-7a12-4eaf-855b-af4f0204dc5a", "Raymond Swanland"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{4}{R}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target land. Into the Maw of Hell deals 13 damage to target creature.",
        &[
            AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Land)),
            AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                CardType::Creature,
            )),
        ],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            },
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex(1)),
                amount: ValueDef::Constant(13),
            },
        ]),
    )),
);

// ISD 151 — Kessig Wolf
pub(in crate::card::sets) static KESSIG_WOLF: CardRecord = CardRecord::new(
    cards::KESSIG_WOLF,
    "Kessig Wolf",
    CardArt::new("3255480b-c1cf-43d9-a40e-43e38112bb18", "Wayne England"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Wolf"], 3, 1).with_ability(
        AbilityDef::activated(
            "{1}{R}: This creature gains first strike until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ISD 152 — Kruin Outlaw
// Audit: blocked — Needs menace as an executable minimum-blocker constraint granted to Werewolves on the back face.

// ISD 153 — Night Revelers
// Audit: blocked — Needs a continuous condition that grants haste only while an opponent controls a Human.

// ISD 154 — Nightbird's Clutches
pub(in crate::card::sets) static NIGHTBIRDS_CLUTCHES: CardRecord = CardRecord::new(
    cards::NIGHTBIRDS_CLUTCHES,
    "Nightbird's Clutches",
    CardArt::new("b5c7410d-b69b-41a3-b469-e12c6ffc7578", "Jason A. Engle"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Up to two target creatures can't block this turn.",
            &CLUTCHES_TARGETS,
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBlock),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::flashback(mana_cost!("{3}{R}")),
    ]),
);

static CLUTCHES_TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::up_to(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
    2,
)];

// ISD 155 — Past in Flames
pub(in crate::card::sets) static PAST_IN_FLAMES: CardRecord = CardRecord::new(
    cards::PAST_IN_FLAMES,
    "Past in Flames",
    CardArt::new("23af6033-4930-48e4-821d-14cbbe1754b4", "Anthony Jones"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{3}{R}")).with_abilities(&[
        AbilityDef::spell(
            "Each instant and sorcery card in your graveyard gains flashback until end of turn. The flashback cost is equal to its mana cost.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                    ]), &[ZoneKind::Graveyard], PlayerRelation::You),
                effect: AppliedEffectDef::add_ability(&abilities::flashback_for_card_mana_cost()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::flashback(mana_cost!("{4}{R}")),
    ]),
);

// ISD 156 — Pitchburn Devils
pub(in crate::card::sets) static PITCHBURN_DEVILS: CardRecord = CardRecord::new(
    cards::PITCHBURN_DEVILS,
    "Pitchburn Devils",
    CardArt::new("d31d3de5-4028-457f-8eba-82e829061a40", "Johann Bodin"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Devil"], 3, 3).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature dies, it deals 3 damage to any target.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
            },
        ),
    ),
);

// ISD 157 — Rage Thrower
pub(in crate::card::sets) static RAGE_THROWER: CardRecord = CardRecord::new(
    cards::RAGE_THROWER,
    "Rage Thrower",
    CardArt::new("f16db004-3e0c-491b-b8b6-0ae046d11761", "Peter Mohrbacher"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{5}{R}"), &["Human", "Shaman"], 4, 2).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever another creature dies, this creature deals 2 damage to target player or planeswalker.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]), Some(ZoneKind::Battlefield), Some(ZoneKind::Graveyard)),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
        ),
    ),
);

// ISD 158 — Rakish Heir
pub(in crate::card::sets) static RAKISH_HEIR: CardRecord = CardRecord::new(
    cards::RAKISH_HEIR,
    "Rakish Heir",
    CardArt::new("4afab3a6-95e3-4786-94f2-d9aa7365a4de", "Winona Nelson"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Vampire"], 2, 2).with_ability(
        AbilityDef::triggered(
            "Whenever a Vampire you control deals combat damage to a player, put a +1/+1 counter on it.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Subtype("Vampire"),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
            EffectDef::AddCounters {
                object: EffectRecipientDef::TriggeringObject,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

const fn reckless_waif_front_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{R}"), &["Human", "Rogue", "Werewolf"], 1, 1)
        .with_ability(WEREWOLF_FRONT_TRANSFORM)
}

const fn merciless_predator_rules() -> CardRules {
    CardRules::new_creature_without_mana_cost(&["Werewolf"], 3, 2)
        .printed_colors(&[ManaColor::Red])
        .with_ability(WEREWOLF_BACK_TRANSFORM)
}

fn reckless_waif_composition() -> CardComposition {
    two_face_creature_composition(
        "Reckless Waif",
        "Merciless Predator",
        reckless_waif_front_rules(),
        merciless_predator_rules(),
        mana_cost!("{R}"),
    )
}

// ISD 159 — Reckless Waif
pub(in crate::card::sets) static RECKLESS_WAIF: CardRecord = CardRecord::new(
    cards::RECKLESS_WAIF,
    "Reckless Waif",
    CardArt::new("028aeebc-4073-4595-94da-02f9f96ea148", "Michael C. Hayes"),
    CardSet::Innistrad,
    reckless_waif_front_rules(),
)
.with_composition(reckless_waif_composition);

// ISD 160 — Riot Devils
pub(in crate::card::sets) static RIOT_DEVILS: CardRecord = CardRecord::new(
    cards::RIOT_DEVILS,
    "Riot Devils",
    CardArt::new("cd35107b-6aaf-4fd8-bf1c-12b724d1482e", "Svetlin Velinov"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Devil"], 2, 3),
);

// ISD 161 — Rolling Temblor
pub(in crate::card::sets) static ROLLING_TEMBLOR: CardRecord = CardRecord::new(
    cards::ROLLING_TEMBLOR,
    "Rolling Temblor",
    CardArt::new("060ce982-94dd-4b9e-b240-15da297e29f9", "Cliff Childs"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{2}{R}")).with_abilities(&[
        AbilityDef::spell(
            "Rolling Temblor deals 2 damage to each creature without flying.",
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasKeyword(
                            crate::card::KeywordAbility::Flying,
                        )),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                amount: ValueDef::Constant(2),
            },
        ),
        abilities::flashback(mana_cost!("{4}{R}{R}")),
    ]),
);

static OPPONENT_CREATURES: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::Opponent,
);

// ISD 162 — Scourge of Geier Reach
pub(in crate::card::sets) static SCOURGE_OF_GEIER_REACH: CardRecord = CardRecord::new(
    cards::SCOURGE_OF_GEIER_REACH,
    "Scourge of Geier Reach",
    CardArt::new("e0c25932-96e7-4ae5-b544-8780f92d0be7", "Jung Park"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Elemental"], 3, 3).with_ability(
        AbilityDef::static_ability(
            "This creature gets +1/+1 for each creature your opponents control.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::CountMatchingObjects(&OPPONENT_CREATURES),
                    ValueDef::CountMatchingObjects(&OPPONENT_CREATURES),
                ),
            },
        ),
    ),
);

// ISD 163 — Skirsdag Cultist
pub(in crate::card::sets) static SKIRSDAG_CULTIST: CardRecord = CardRecord::new(
    cards::SKIRSDAG_CULTIST,
    "Skirsdag Cultist",
    CardArt::new("e63fa0de-2ec3-41ff-8e5d-0b54f400f27f", "Slawomir Maniak"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Human", "Shaman"], 2, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{R}, {T}, Sacrifice a creature: This creature deals 2 damage to any target.",
            &[
                AbilityCostDef::Mana(mana_cost!("{R}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    controller: PlayerRelation::You,
                },
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
        ),
    ),
);

// ISD 164 — Stromkirk Noble
pub(in crate::card::sets) static STROMKIRK_NOBLE: CardRecord = CardRecord::new(
    cards::STROMKIRK_NOBLE,
    "Stromkirk Noble",
    CardArt::new("9c16cf74-f9e0-4d80-9a29-b91dec0b6b38", "James Ryman"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{R}"), &["Vampire", "Noble"], 1, 1).with_abilities(&[
        AbilityDef::static_ability(
            "This creature can't be blocked by Humans.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBeBlockedBy(
                    ObjectPredicateDef::Subtype("Human"),
                )),
            },
        ),
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, put a +1/+1 counter on it.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

const fn tormented_pariah_front_rules() -> CardRules {
    CardRules::new_creature(
        mana_cost!("{3}{R}"),
        &["Human", "Warrior", "Werewolf"],
        3,
        2,
    )
    .with_ability(WEREWOLF_FRONT_TRANSFORM)
}

const fn rampaging_werewolf_rules() -> CardRules {
    CardRules::new_creature_without_mana_cost(&["Werewolf"], 6, 4)
        .printed_colors(&[ManaColor::Red])
        .with_ability(WEREWOLF_BACK_TRANSFORM)
}

fn tormented_pariah_composition() -> CardComposition {
    two_face_creature_composition(
        "Tormented Pariah",
        "Rampaging Werewolf",
        tormented_pariah_front_rules(),
        rampaging_werewolf_rules(),
        mana_cost!("{3}{R}"),
    )
}

// ISD 165 — Tormented Pariah
pub(in crate::card::sets) static TORMENTED_PARIAH: CardRecord = CardRecord::new(
    cards::TORMENTED_PARIAH,
    "Tormented Pariah",
    CardArt::new("6151cae7-92a4-4891-a952-21def412d3e4", "Bud Cook"),
    CardSet::Innistrad,
    tormented_pariah_front_rules(),
)
.with_composition(tormented_pariah_composition);

// ISD 166 — Traitorous Blood
pub(in crate::card::sets) static TRAITOROUS_BLOOD: CardRecord = CardRecord::new(
    cards::TRAITOROUS_BLOOD,
    "Traitorous Blood",
    CardArt::new("8220f18a-f23f-4fe6-bb58-58b6c5f36c79", "Raymond Swanland"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{1}{R}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "Gain control of target creature until end of turn. Untap it. It gains trample and haste until end of turn.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Sequence(&[
                EffectDef::GainControl {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    duration: ControlDurationDef::UntilEndOfTurn,
                },
                EffectDef::Untap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&abilities::trample()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&abilities::haste()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
    ),
);

// ISD 167 — Vampiric Fury
pub(in crate::card::sets) static VAMPIRIC_FURY: CardRecord = CardRecord::new(
    cards::VAMPIRIC_FURY,
    "Vampiric Fury",
    CardArt::new("de4fd254-0ae9-498d-b9da-4fb3d6a1a55c", "Matt Stewart"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(AbilityDef::spell(
        "Vampire creatures you control get +2/+0 and gain first strike until end of turn.",
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Vampire"),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Vampire"),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )),
);

static VILLAGE_IRONSMITH_FRONT_ABILITIES: [AbilityDef; 2] =
    [abilities::first_strike(), WEREWOLF_FRONT_TRANSFORM];
static IRONFANG_ABILITIES: [AbilityDef; 2] = [abilities::first_strike(), WEREWOLF_BACK_TRANSFORM];

const fn village_ironsmith_front_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Werewolf"], 1, 1)
        .with_abilities(&VILLAGE_IRONSMITH_FRONT_ABILITIES)
}

const fn ironfang_rules() -> CardRules {
    CardRules::new_creature_without_mana_cost(&["Werewolf"], 3, 1)
        .printed_colors(&[ManaColor::Red])
        .with_abilities(&IRONFANG_ABILITIES)
}

fn village_ironsmith_composition() -> CardComposition {
    two_face_creature_composition(
        "Village Ironsmith",
        "Ironfang",
        village_ironsmith_front_rules(),
        ironfang_rules(),
        mana_cost!("{1}{R}"),
    )
}

// ISD 168 — Village Ironsmith
pub(in crate::card::sets) static VILLAGE_IRONSMITH: CardRecord = CardRecord::new(
    cards::VILLAGE_IRONSMITH,
    "Village Ironsmith",
    CardArt::new(
        "cd5435d0-789f-4c42-8efc-165c072404a2",
        "Christopher Moeller",
    ),
    CardSet::Innistrad,
    village_ironsmith_front_rules(),
)
.with_composition(village_ironsmith_composition);

// ISD 169 — Ambush Viper
pub(in crate::card::sets) static AMBUSH_VIPER: CardRecord = CardRecord::new(
    cards::AMBUSH_VIPER,
    "Ambush Viper",
    CardArt::new("0c082aa8-bf7f-47f2-baf8-43ad253fd7d7", "Alan Pollack"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Snake"], 2, 1)
        .with_abilities(&[abilities::flash(), abilities::deathtouch()]),
);

// ISD 170 — Avacyn's Pilgrim
pub(in crate::card::sets) static AVACYNS_PILGRIM: CardRecord = CardRecord::new(
    cards::AVACYNS_PILGRIM,
    "Avacyn's Pilgrim",
    CardArt::new(
        "7eb39e97-53c2-4df0-9fb3-a3d6a24ec41f",
        "Jana Schirmer & Johannes Voss",
    ),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{G}"), &["Human", "Monk"], 1, 1)
        .with_abilities(&[abilities::tap_for(ManaColor::White)]),
);

static CREATURE_CARDS_IN_YOUR_GRAVEYARD: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Graveyard],
    PlayerRelation::You,
);

// ISD 171 — Boneyard Wurm
pub(in crate::card::sets) static BONEYARD_WURM: CardRecord = CardRecord::new(
    cards::BONEYARD_WURM,
    "Boneyard Wurm",
    CardArt::new("75f3d9eb-462c-41b5-ad1a-baab7dc5eac3", "Jaime Jones"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Wurm"], 0, 0).with_ability(
        AbilityDef::static_ability(
            "Boneyard Wurm's power and toughness are each equal to the number of creature cards in your graveyard.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(ValueDef::CountMatchingObjects(&CREATURE_CARDS_IN_YOUR_GRAVEYARD), ValueDef::CountMatchingObjects(&CREATURE_CARDS_IN_YOUR_GRAVEYARD)),
            },
        ),
    ),
);

// ISD 172 — Bramblecrush
pub(in crate::card::sets) static BRAMBLECRUSH: CardRecord = CardRecord::new(
    cards::BRAMBLECRUSH,
    "Bramblecrush",
    CardArt::new("60fa219e-5dba-4d49-9cae-40d254f140e4", "Drew Baker"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{2}{G}{G}")).with_ability(AbilityDef::destroy_target(
        "Destroy target noncreature permanent.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Not(
            &ObjectPredicateDef::HasType(CardType::Creature),
        )),
        true,
    )),
);

// ISD 173 — Caravan Vigil
// Audit: blocked — Needs the searched card's destination to branch on morbid while preserving the hidden-zone search choice.

// ISD 174 — Creeping Renaissance
// Audit: blocked — Needs a permanent-card-type choice and a graveyard sweep keyed to the chosen type.

// ISD 175 — Darkthicket Wolf
pub(in crate::card::sets) static DARKTHICKET_WOLF: CardRecord = CardRecord::new(
    cards::DARKTHICKET_WOLF,
    "Darkthicket Wolf",
    CardArt::new("fec37c5a-8223-441c-a8a6-8da1a2dfc3fb", "Wayne England"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Wolf"], 2, 2).with_abilities(&[
        AbilityDef::activated(
            "{2}{G}: This creature gets +2/+2 until end of turn. Activate only once each turn.",
            &[AbilityCostDef::Mana(mana_cost!("{2}{G}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .once_each_turn(),
    ]),
);

static NO_SPELLS_LAST_TURN: TriggerConditionDef = TriggerConditionDef::SpellsCastLastTurn {
    quantifier: QuantifierDef::Every,
    player: PlayerRelation::Any,
    comparison: ComparisonDef::LessOrEqual,
    amount: 0,
};

static TWO_SPELLS_LAST_TURN: TriggerConditionDef = TriggerConditionDef::SpellsCastLastTurn {
    quantifier: QuantifierDef::Any,
    player: PlayerRelation::Any,
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 2,
};

static WEREWOLF_FRONT_TRANSFORM: AbilityDef = AbilityDef::triggered_if(
    "At the beginning of each upkeep, if no spells were cast last turn, transform this creature.",
    TriggerEventDef::StepBegins {
        step: crate::card::TurnStepDef::Upkeep,
        player: PlayerRelation::Any,
    },
    &NO_SPELLS_LAST_TURN,
    EffectDef::Transform {
        object: EffectRecipientDef::Source,
    },
);

static WEREWOLF_BACK_TRANSFORM: AbilityDef = AbilityDef::triggered_if(
    "At the beginning of each upkeep, if a player cast two or more spells last turn, transform this creature.",
    TriggerEventDef::StepBegins {
        step: crate::card::TurnStepDef::Upkeep,
        player: PlayerRelation::Any,
    },
    &TWO_SPELLS_LAST_TURN,
    EffectDef::Transform {
        object: EffectRecipientDef::Source,
    },
);

static DAYBREAK_RANGER_TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::HasKeyword(crate::card::KeywordAbility::Flying),
    ]),
)];

static DAYBREAK_RANGER_FRONT_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::activated_with_targets(
        "{T}: This creature deals 2 damage to target creature with flying.",
        &[AbilityCostDef::TapSource],
        &DAYBREAK_RANGER_TARGETS,
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(2),
        },
    ),
    WEREWOLF_FRONT_TRANSFORM,
];

static NIGHTFALL_PREDATOR_TARGETS: [AbilityTargetDef; 1] =
    [AbilityTargetDef::exactly_one_permanent(
        ObjectPredicateDef::HasType(CardType::Creature),
    )];

static NIGHTFALL_PREDATOR_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::activated_with_targets(
        "{R}, {T}: This creature fights target creature.",
        &[
            AbilityCostDef::Mana(mana_cost!("{R}")),
            AbilityCostDef::TapSource,
        ],
        &NIGHTFALL_PREDATOR_TARGETS,
        EffectDef::Sequence(&[
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::SourcePower,
            },
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Source,
                amount: ValueDef::TargetPower(TargetIndex::PRIMARY),
            },
        ]),
    ),
    WEREWOLF_BACK_TRANSFORM,
];

const fn daybreak_ranger_front_rules() -> CardRules {
    CardRules::new_creature(
        mana_cost!("{2}{G}"),
        &["Human", "Archer", "Ranger", "Werewolf"],
        2,
        2,
    )
    .with_abilities(&DAYBREAK_RANGER_FRONT_ABILITIES)
}

const fn nightfall_predator_rules() -> CardRules {
    CardRules::new_creature_without_mana_cost(&["Werewolf"], 4, 4)
        .printed_colors(&[ManaColor::Green])
        .with_abilities(&NIGHTFALL_PREDATOR_ABILITIES)
}

fn daybreak_ranger_composition() -> CardComposition {
    let front = daybreak_ranger_front_rules();
    let back = nightfall_predator_rules();
    CardComposition {
        parts: vec![
            CardPart::new(CardPartId::PRIMARY, "Daybreak Ranger", front),
            CardPart::new(CardPartId(1), "Nightfall Predator", back),
        ],
        structure: CardStructure::DoubleFaced {
            front: CardPartId::PRIMARY,
            back: CardPartId(1),
            kind: DoubleFacedKind::Transforming,
        },
        play_options: vec![PlayOptionDef::cast(
            PlayOptionId::DEFAULT,
            "Daybreak Ranger",
            SpellForm::Part(CardPartId::PRIMARY),
            mana_cost!("{2}{G}"),
            CardEffectStatus::Implemented,
        )],
    }
}

// ISD 176 — Daybreak Ranger
pub(in crate::card::sets) static DAYBREAK_RANGER: CardRecord = CardRecord::new(
    cards::DAYBREAK_RANGER,
    "Daybreak Ranger",
    CardArt::new("25b54a1d-e201-453b-9173-b04e06ee6fb7", "Steve Prescott"),
    CardSet::Innistrad,
    daybreak_ranger_front_rules(),
)
.with_composition(daybreak_ranger_composition);

// ISD 177 — Elder of Laurels
pub(in crate::card::sets) static ELDER_OF_LAURELS: CardRecord = CardRecord::new(
    cards::ELDER_OF_LAURELS,
    "Elder of Laurels",
    CardArt::new("32b82ef0-c974-4357-b21a-4c2a28ec7279", "Terese Nielsen"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human", "Advisor"], 2, 3).with_ability(
        AbilityDef::activated_with_targets(
            "{3}{G}: Target creature gets +X/+X until end of turn, where X is the number of creatures you control.",
            &[AbilityCostDef::Mana(mana_cost!("{3}{G}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(ValueDef::CountMatchingObjects(&CREATURES_YOU_CONTROL), ValueDef::CountMatchingObjects(&CREATURES_YOU_CONTROL)),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ISD 178 — Essence of the Wild
// Audit: blocked — Needs a battlefield entry replacement that copies the source's copiable values onto other creatures.

/// Morbid's entry bonus. The condition is checked as the creature enters, so
/// a creature dying in response to the spell still counts.
static MORBID_TWO_COUNTERS: AbilityDef = AbilityDef::as_enters_if(
    "Morbid — This creature enters with two +1/+1 counters on it if a creature died this turn.",
    ReplacementConditionDef::CreatureDiedThisTurn,
    ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::AddCounters {
        kind: CounterKind::PlusOnePlusOne,
        amount: 2,
    }),
);

// ISD 179 — Festerhide Boar
pub(in crate::card::sets) static FESTERHIDE_BOAR: CardRecord = CardRecord::new(
    cards::FESTERHIDE_BOAR,
    "Festerhide Boar",
    CardArt::new("31740fe9-27d2-416e-93de-509ac1a7b7cd", "Nils Hamm"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Boar"], 3, 3)
        .with_abilities(&[abilities::trample(), MORBID_TWO_COUNTERS]),
);

// ISD 180 — Full Moon's Rise
pub(in crate::card::sets) static FULL_MOONS_RISE: CardRecord = CardRecord::new(
    cards::FULL_MOONS_RISE,
    "Full Moon's Rise",
    CardArt::new("02a35eac-b962-466e-a4da-a4010c68ef16", "Terese Nielsen"),
    CardSet::Innistrad,
    CardRules::new_enchantment(mana_cost!("{1}{G}")).with_abilities(&[
        AbilityDef::static_ability(
            "Werewolf creatures you control get +1/+0 and have trample.",
            EffectDef::Sequence(&[
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype("Werewolf"),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
                },
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype("Werewolf"),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::add_ability(&abilities::trample()),
                },
            ]),
        ),
        AbilityDef::activated(
            "Sacrifice this enchantment: Regenerate all Werewolf creatures you control.",
            &[AbilityCostDef::SacrificeSource],
            EffectDef::Regenerate {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Werewolf"),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
            },
        ),
    ]),
);

static GARRUK_FRONT_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::triggered_if(
        "When Garruk has two or fewer loyalty counters on him, transform him.",
        TriggerEventDef::StateCondition,
        &GARRUK_LOW_LOYALTY,
        EffectDef::Transform {
            object: EffectRecipientDef::Source,
        },
    ),
    AbilityDef::activated_with_targets(
        "0: Garruk deals 3 damage to target creature. That creature deals damage equal to its power to him.",
        &[AbilityCostDef::Loyalty(0)],
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        // The creature hits back with the power it had when the ability
        // resolved, which is why the loyalty it costs Garruk is read off
        // the target rather than printed.
        EffectDef::Sequence(&[
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
            },
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Source,
                amount: ValueDef::TargetPower(TargetIndex::PRIMARY),
            },
        ]),
    ),
    AbilityDef::activated(
        "0: Create a 2/2 green Wolf creature token.",
        &[AbilityCostDef::Loyalty(0)],
        EffectDef::CreateToken {
            token: cards::WOLF_TOKEN_2_2_GREEN,
            count: ValueDef::Constant(1),
            tapped: false,
        },
    ),
];

const fn garruk_front_rules() -> CardRules {
    CardRules::new_planeswalker(mana_cost!("{3}{G}"), &["Garruk"], 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&GARRUK_FRONT_ABILITIES)
}
/// Two or fewer is at most two, checked as a state trigger so it turns the
/// moment the damage lands rather than waiting for anything.
static GARRUK_LOW_LOYALTY: TriggerConditionDef = TriggerConditionDef::SourceLoyalty {
    comparison: ComparisonDef::LessOrEqual,
    amount: 2,
};

static GARRUK_TUTOR: EffectDef = EffectDef::SearchZone {
    player: EffectRecipientDef::Controller,
    source: ZoneKind::Library,
    object: ObjectPredicateDef::HasType(CardType::Creature),
    minimum: 0,
    maximum: 1,
    reveal: true,
    destination: ZoneKind::Hand,
    placement: ZonePlacement::Top,
    shuffle: true,
    enters_tapped: false,
};

static GARRUK_TRAMPLE: AbilityDef = abilities::trample();

static GARRUK_GRAVEYARD_CREATURES: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Graveyard],
    PlayerRelation::You,
);

static GARRUK_BACK_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::activated(
        "+1: Create a 1/1 black Wolf creature token with deathtouch.",
        &[AbilityCostDef::Loyalty(1)],
        EffectDef::CreateToken {
            token: cards::WOLF_TOKEN_1_1_BLACK,
            count: ValueDef::Constant(1),
            tapped: false,
        },
    ),
    AbilityDef::activated(
        "−1: Sacrifice a creature. If you do, search your library for a creature card, reveal it, put it into your hand, then shuffle.",
        &[AbilityCostDef::Loyalty(-1)],
        EffectDef::SacrificeOfChoice {
            player: EffectRecipientDef::Controller,
            object: ObjectPredicateDef::HasType(CardType::Creature),
            then: Some(&GARRUK_TUTOR),
            optional: false,
        },
    ),
    AbilityDef::activated(
        "−3: Creatures you control gain trample and get +X/+X until end of turn, where X is the number of creature cards in your graveyard.",
        &[AbilityCostDef::Loyalty(-3)],
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: GARRUK_PUMP,
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    ),
];

static GARRUK_PUMP_PARTS: [AppliedEffectDef; 2] = [
    AppliedEffectDef::add_ability(&GARRUK_TRAMPLE),
    AppliedEffectDef::modify_power_toughness(
        ValueDef::CountMatchingObjects(&GARRUK_GRAVEYARD_CREATURES),
        ValueDef::CountMatchingObjects(&GARRUK_GRAVEYARD_CREATURES),
    ),
];

static GARRUK_PUMP: AppliedEffectDef = AppliedEffectDef::Composite(&GARRUK_PUMP_PARTS);

fn garruk_composition() -> CardComposition {
    let front = garruk_front_rules();
    let back = CardRules::new_planeswalker_without_mana_cost(&["Garruk"])
        .with_supertype(CardSupertype::Legendary)
        .printed_colors(&[ManaColor::Black, ManaColor::Green])
        .with_abilities(&GARRUK_BACK_ABILITIES);
    CardComposition {
        parts: vec![
            CardPart::new(CardPartId::PRIMARY, "Garruk Relentless", front),
            CardPart::new(CardPartId(1), "Garruk, the Veil-Cursed", back),
        ],
        structure: CardStructure::DoubleFaced {
            front: CardPartId::PRIMARY,
            back: CardPartId(1),
            kind: DoubleFacedKind::Transforming,
        },
        play_options: vec![PlayOptionDef::cast(
            PlayOptionId::DEFAULT,
            "Garruk Relentless",
            SpellForm::Part(CardPartId::PRIMARY),
            front
                .mana_cost()
                .expect("Garruk Relentless has a printed mana cost"),
            CardEffectStatus::Implemented,
        )],
    }
}

// ISD 181 — Garruk Relentless
pub(in crate::card::sets) static GARRUK_RELENTLESS: CardRecord = CardRecord::new(
    cards::GARRUK_RELENTLESS,
    "Garruk Relentless",
    CardArt::new("b4160322-ff40-41a4-887a-73cd6b85ae45", "Eric Deschamps"),
    CardSet::Innistrad,
    garruk_front_rules(),
)
.with_composition(garruk_composition);

const fn gatstaf_shepherd_front_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Human", "Werewolf"], 2, 2)
        .with_ability(WEREWOLF_FRONT_TRANSFORM)
}

static GATSTAF_HOWLER_ABILITIES: [AbilityDef; 2] =
    [abilities::intimidate(), WEREWOLF_BACK_TRANSFORM];

const fn gatstaf_howler_rules() -> CardRules {
    CardRules::new_creature_without_mana_cost(&["Werewolf"], 3, 3)
        .printed_colors(&[ManaColor::Green])
        .with_abilities(&GATSTAF_HOWLER_ABILITIES)
}

fn gatstaf_shepherd_composition() -> CardComposition {
    let front = gatstaf_shepherd_front_rules();
    let back = gatstaf_howler_rules();
    CardComposition {
        parts: vec![
            CardPart::new(CardPartId::PRIMARY, "Gatstaf Shepherd", front),
            CardPart::new(CardPartId(1), "Gatstaf Howler", back),
        ],
        structure: CardStructure::DoubleFaced {
            front: CardPartId::PRIMARY,
            back: CardPartId(1),
            kind: DoubleFacedKind::Transforming,
        },
        play_options: vec![PlayOptionDef::cast(
            PlayOptionId::DEFAULT,
            "Gatstaf Shepherd",
            SpellForm::Part(CardPartId::PRIMARY),
            mana_cost!("{1}{G}"),
            CardEffectStatus::Implemented,
        )],
    }
}

// ISD 182 — Gatstaf Shepherd
pub(in crate::card::sets) static GATSTAF_SHEPHERD: CardRecord = CardRecord::new(
    cards::GATSTAF_SHEPHERD,
    "Gatstaf Shepherd",
    CardArt::new("57f0907f-74f4-4d86-93df-f2e50c9d0b2f", "Mark Evans"),
    CardSet::Innistrad,
    gatstaf_shepherd_front_rules(),
)
.with_composition(gatstaf_shepherd_composition);

// ISD 183 — Gnaw to the Bone
// Audit: blocked — Needs multiplying the number of creature cards in your graveyard by two for a life-gain amount.

// ISD 184 — Grave Bramble
// Audit: blocked — Needs protection parameterized by the Zombie creature subtype.

const fn grizzled_outcasts_front_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Human", "Werewolf"], 4, 4)
        .with_ability(WEREWOLF_FRONT_TRANSFORM)
}

const fn krallenhorde_wantons_rules() -> CardRules {
    CardRules::new_creature_without_mana_cost(&["Werewolf"], 7, 7)
        .printed_colors(&[ManaColor::Green])
        .with_ability(WEREWOLF_BACK_TRANSFORM)
}

fn grizzled_outcasts_composition() -> CardComposition {
    let front = grizzled_outcasts_front_rules();
    let back = krallenhorde_wantons_rules();
    CardComposition {
        parts: vec![
            CardPart::new(CardPartId::PRIMARY, "Grizzled Outcasts", front),
            CardPart::new(CardPartId(1), "Krallenhorde Wantons", back),
        ],
        structure: CardStructure::DoubleFaced {
            front: CardPartId::PRIMARY,
            back: CardPartId(1),
            kind: DoubleFacedKind::Transforming,
        },
        play_options: vec![PlayOptionDef::cast(
            PlayOptionId::DEFAULT,
            "Grizzled Outcasts",
            SpellForm::Part(CardPartId::PRIMARY),
            mana_cost!("{4}{G}"),
            CardEffectStatus::Implemented,
        )],
    }
}

// ISD 185 — Grizzled Outcasts
pub(in crate::card::sets) static GRIZZLED_OUTCASTS: CardRecord = CardRecord::new(
    cards::GRIZZLED_OUTCASTS,
    "Grizzled Outcasts",
    CardArt::new("4b43b0cb-a5a3-47b4-9b6b-9d2638222bb6", "Randy Gallegos"),
    CardSet::Innistrad,
    grizzled_outcasts_front_rules(),
)
.with_composition(grizzled_outcasts_composition);

// ISD 186 — Gutter Grime
// Audit: blocked — Needs slime counters and a source-linked Ooze token whose P/T tracks the source's counter count.

// ISD 187 — Hamlet Captain
// Audit: blocked — Needs a blocks event in addition to the supported attacks event for its temporary Human bonus.

// ISD 188 — Hollowhenge Scavenger
// Audit: blocked — Needs a morbid intervening-if check that suppresses the ETB trigger when no creature died.

// ISD 189 — Kessig Cagebreakers
// Audit: blocked — Needs a dynamic number of Wolf tokens entering tapped and attacking.

// ISD 190 — Kindercatch
pub(in crate::card::sets) static KINDERCATCH: CardRecord = CardRecord::new(
    cards::KINDERCATCH,
    "Kindercatch",
    CardArt::new("4954e8a3-e72b-4f28-8762-2b1c658c31b6", "Terese Nielsen"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{3}{G}{G}{G}"), &["Spirit"], 6, 6),
);

// ISD 191 — Lumberknot
pub(in crate::card::sets) static LUMBERKNOT: CardRecord = CardRecord::new(
    cards::LUMBERKNOT,
    "Lumberknot",
    CardArt::new("6c86c84e-9bab-4a2c-b594-7f7b4b6bba88", "Jason A. Engle"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Treefolk"], 1, 1).with_abilities(&[
        abilities::hexproof(),
        AbilityDef::triggered(
            "Whenever a creature dies, put a +1/+1 counter on this creature.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::HasType(CardType::Creature),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// ISD 192 — Make a Wish
// Audit: blocked — Needs deterministic random selection of two cards from your graveyard.

static MAYOR_OF_AVABRUCK_FRONT_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::static_ability(
        "Other Human creatures you control get +1/+1.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Subtype("Human"),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(1),
                ValueDef::Constant(1),
            ),
        },
    ),
    WEREWOLF_FRONT_TRANSFORM,
];

const fn mayor_of_avabruck_front_rules() -> CardRules {
    CardRules::new_creature(
        mana_cost!("{1}{G}"),
        &["Human", "Advisor", "Werewolf"],
        1,
        1,
    )
    .with_abilities(&MAYOR_OF_AVABRUCK_FRONT_ABILITIES)
}

static HOWLPACK_ALPHA_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::static_ability(
        "Each other creature you control that's a Werewolf or a Wolf gets +1/+1.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::Subtype("Werewolf"),
                        ObjectPredicateDef::Subtype("Wolf"),
                    ]),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(1),
                ValueDef::Constant(1),
            ),
        },
    ),
    AbilityDef::triggered(
        "At the beginning of your end step, create a 2/2 green Wolf creature token.",
        TriggerEventDef::StepBegins {
            step: crate::card::TurnStepDef::End,
            player: PlayerRelation::You,
        },
        EffectDef::CreateToken {
            token: cards::WOLF_TOKEN_2_2_GREEN,
            count: ValueDef::Constant(1),
            tapped: false,
        },
    ),
    WEREWOLF_BACK_TRANSFORM,
];

const fn howlpack_alpha_rules() -> CardRules {
    CardRules::new_creature_without_mana_cost(&["Werewolf"], 3, 3)
        .printed_colors(&[ManaColor::Green])
        .with_abilities(&HOWLPACK_ALPHA_ABILITIES)
}

fn mayor_of_avabruck_composition() -> CardComposition {
    let front = mayor_of_avabruck_front_rules();
    let back = howlpack_alpha_rules();
    CardComposition {
        parts: vec![
            CardPart::new(CardPartId::PRIMARY, "Mayor of Avabruck", front),
            CardPart::new(CardPartId(1), "Howlpack Alpha", back),
        ],
        structure: CardStructure::DoubleFaced {
            front: CardPartId::PRIMARY,
            back: CardPartId(1),
            kind: DoubleFacedKind::Transforming,
        },
        play_options: vec![PlayOptionDef::cast(
            PlayOptionId::DEFAULT,
            "Mayor of Avabruck",
            SpellForm::Part(CardPartId::PRIMARY),
            mana_cost!("{1}{G}"),
            CardEffectStatus::Implemented,
        )],
    }
}

// ISD 193 — Mayor of Avabruck
pub(in crate::card::sets) static MAYOR_OF_AVABRUCK: CardRecord = CardRecord::new(
    cards::MAYOR_OF_AVABRUCK,
    "Mayor of Avabruck",
    CardArt::new("dd8ca448-f734-4cb9-b1d5-790eed9a4b2d", "Svetlin Velinov"),
    CardSet::Innistrad,
    mayor_of_avabruck_front_rules(),
)
.with_composition(mayor_of_avabruck_composition);

// ISD 194 — Moldgraf Monstrosity
// Audit: blocked — Needs deterministic random selection of two creature cards from your graveyard after exiling the source.

// ISD 195 — Moonmist
// Audit: blocked — Needs transforming all Human double-faced permanents and selectively preventing combat damage from non-Werewolves and non-Wolves.

// ISD 196 — Mulch
pub(in crate::card::sets) static MULCH: CardRecord = CardRecord::new(
    cards::MULCH,
    "Mulch",
    CardArt::new("52a1dabd-82df-4814-9d64-bf7bf9c1018d", "Christopher Moeller"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{1}{G}")).with_ability(
        AbilityDef::custom_full(
            "Reveal the top four cards of your library. Put all land cards revealed this way into your hand and the rest into your graveyard.",
            CardBehavior::Mulch,
            "Implemented by the named card-local special behavior.",
        ),
    ),
);

// ISD 198 — Orchard Spirit
pub(in crate::card::sets) static ORCHARD_SPIRIT: CardRecord = CardRecord::new(
    cards::ORCHARD_SPIRIT,
    "Orchard Spirit",
    CardArt::new("aac43ced-35b0-4e70-a049-1a65db9b2b1e", "Howard Lyon"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Spirit"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "This creature can't be blocked except by creatures with flying or reach.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBeBlockedBy(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasKeyword(crate::card::KeywordAbility::Flying),
                        ObjectPredicateDef::HasKeyword(crate::card::KeywordAbility::Reach),
                    ])),
                )),
            },
        ),
    ),
);

// ISD 199 — Parallel Lives
// Audit: blocked — Needs a token-creation replacement event that doubles the number of tokens an effect would create.

// ISD 200 — Prey Upon
pub(in crate::card::sets) static PREY_UPON: CardRecord = CardRecord::new(
    cards::PREY_UPON,
    "Prey Upon",
    CardArt::new("b7b3eaf0-4207-4bac-923d-29f348c95a35", "Dave Kendall"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{G}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature you control fights target creature you don't control.",
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
                controller: Some(PlayerRelation::NotYou),
                owner: None,
            }),
        ],
        EffectDef::Sequence(&[
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex(1)),
                amount: ValueDef::TargetPower(TargetIndex::PRIMARY),
            },
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::TargetPower(TargetIndex(1)),
            },
        ]),
    )),
);

// ISD 201 — Ranger's Guile
pub(in crate::card::sets) static RANGERS_GUILE: CardRecord = CardRecord::new(
    cards::RANGERS_GUILE,
    "Ranger's Guile",
    CardArt::new("c90742ae-c48b-4d32-a6b7-aa51a94018bd", "Steve Prescott"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{G}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature you control gets +1/+1 and gains hexproof until end of turn.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::hexproof()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )),
);

// ISD 202 — Somberwald Spider
pub(in crate::card::sets) static SOMBERWALD_SPIDER: CardRecord = CardRecord::new(
    cards::SOMBERWALD_SPIDER,
    "Somberwald Spider",
    CardArt::new("43003ad7-2f42-4c85-8b00-77cbf3f50a7b", "Volkan Baǵa"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Spider"], 2, 4)
        .with_abilities(&[abilities::reach(), MORBID_TWO_COUNTERS]),
);

// ISD 203 — Spider Spawning
pub(in crate::card::sets) static SPIDER_SPAWNING: CardRecord = CardRecord::new(
    cards::SPIDER_SPAWNING,
    "Spider Spawning",
    CardArt::new("f97007af-6642-4105-8d8c-4223681e1cf9", "Daniel Ljunggren"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{4}{G}")).with_abilities(&[
        AbilityDef::spell(
            "Create a 1/2 green Spider creature token with reach for each creature card in your graveyard.",
            EffectDef::CreateToken {
                token: cards::SPIDER_TOKEN_1_2_GREEN,
                count: ValueDef::CountMatchingObjects(&CREATURE_CARDS_IN_YOUR_GRAVEYARD),
                tapped: false,
            },
        ),
        abilities::flashback(mana_cost!("{6}{B}")),
    ]),
);

// ISD 204 — Spidery Grasp
pub(in crate::card::sets) static SPIDERY_GRASP: CardRecord = CardRecord::new(
    cards::SPIDERY_GRASP,
    "Spidery Grasp",
    CardArt::new("ccbdfd82-d025-4070-a1f5-4ee759978bcb", "James Ryman"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{2}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Untap target creature. It gets +2/+4 and gains reach until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(4),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::reach()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )),
);

// ISD 205 — Splinterfright
pub(in crate::card::sets) static SPLINTERFRIGHT: CardRecord = CardRecord::new(
    cards::SPLINTERFRIGHT,
    "Splinterfright",
    CardArt::new("37068a41-bc5c-44b9-a307-5d3919794233", "Eric Deschamps"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Elemental"], 0, 0).with_abilities(&[
        abilities::trample(),
        AbilityDef::static_ability(
            "Splinterfright's power and toughness are each equal to the number of creature cards in your graveyard.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(ValueDef::CountMatchingObjects(&CREATURE_CARDS_IN_YOUR_GRAVEYARD), ValueDef::CountMatchingObjects(&CREATURE_CARDS_IN_YOUR_GRAVEYARD)),
            },
        ),
        AbilityDef::triggered(
            "At the beginning of your upkeep, mill two cards.",
            TriggerEventDef::StepBegins {
                step: crate::card::TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::Mill {
                player: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
    ]),
);

// ISD 206 — Travel Preparations
pub(in crate::card::sets) static TRAVEL_PREPARATIONS: CardRecord = CardRecord::new(
    cards::TRAVEL_PREPARATIONS,
    "Travel Preparations",
    CardArt::new("e9654ae7-af2c-4956-be3a-68befa33f523", "Vincent Proce"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{1}{G}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Put a +1/+1 counter on each of up to two target creatures.",
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
        abilities::flashback(mana_cost!("{1}{W}")),
    ]),
);

// ISD 207 — Tree of Redemption
// Audit: blocked — Needs exchanging the controller's life total with the source's current toughness.

const fn ulvenwald_mystics_front_rules() -> CardRules {
    CardRules::new_creature(
        mana_cost!("{2}{G}{G}"),
        &["Human", "Shaman", "Werewolf"],
        3,
        3,
    )
    .with_ability(WEREWOLF_FRONT_TRANSFORM)
}

static ULVENWALD_PRIMORDIALS_ABILITIES: [AbilityDef; 2] = [
    abilities::regenerate_self(
        "{G}: Regenerate this creature.",
        &[AbilityCostDef::Mana(mana_cost!("{G}"))],
    ),
    WEREWOLF_BACK_TRANSFORM,
];

const fn ulvenwald_primordials_rules() -> CardRules {
    CardRules::new_creature_without_mana_cost(&["Werewolf"], 5, 5)
        .printed_colors(&[ManaColor::Green])
        .with_abilities(&ULVENWALD_PRIMORDIALS_ABILITIES)
}

fn ulvenwald_mystics_composition() -> CardComposition {
    two_face_creature_composition(
        "Ulvenwald Mystics",
        "Ulvenwald Primordials",
        ulvenwald_mystics_front_rules(),
        ulvenwald_primordials_rules(),
        mana_cost!("{2}{G}{G}"),
    )
}

// ISD 208 — Ulvenwald Mystics
pub(in crate::card::sets) static ULVENWALD_MYSTICS: CardRecord = CardRecord::new(
    cards::ULVENWALD_MYSTICS,
    "Ulvenwald Mystics",
    CardArt::new("8325c570-4d74-4e65-891c-3e153abf4bf9", "Dan Murayama Scott"),
    CardSet::Innistrad,
    ulvenwald_mystics_front_rules(),
)
.with_composition(ulvenwald_mystics_composition);

const fn villagers_of_estwald_front_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human", "Werewolf"], 2, 3)
        .with_ability(WEREWOLF_FRONT_TRANSFORM)
}

const fn howlpack_of_estwald_rules() -> CardRules {
    CardRules::new_creature_without_mana_cost(&["Werewolf"], 4, 6)
        .printed_colors(&[ManaColor::Green])
        .with_ability(WEREWOLF_BACK_TRANSFORM)
}

fn villagers_of_estwald_composition() -> CardComposition {
    let front = villagers_of_estwald_front_rules();
    let back = howlpack_of_estwald_rules();
    CardComposition {
        parts: vec![
            CardPart::new(CardPartId::PRIMARY, "Villagers of Estwald", front),
            CardPart::new(CardPartId(1), "Howlpack of Estwald", back),
        ],
        structure: CardStructure::DoubleFaced {
            front: CardPartId::PRIMARY,
            back: CardPartId(1),
            kind: DoubleFacedKind::Transforming,
        },
        play_options: vec![PlayOptionDef::cast(
            PlayOptionId::DEFAULT,
            "Villagers of Estwald",
            SpellForm::Part(CardPartId::PRIMARY),
            mana_cost!("{2}{G}"),
            CardEffectStatus::Implemented,
        )],
    }
}

// ISD 209 — Villagers of Estwald
pub(in crate::card::sets) static VILLAGERS_OF_ESTWALD: CardRecord = CardRecord::new(
    cards::VILLAGERS_OF_ESTWALD,
    "Villagers of Estwald",
    CardArt::new("e42a0a3d-a987-4b24-b9d4-27380a12e093", "Kev Walker"),
    CardSet::Innistrad,
    villagers_of_estwald_front_rules(),
)
.with_composition(villagers_of_estwald_composition);

// ISD 210 — Woodland Sleuth
// Audit: blocked — Needs deterministic random selection of a creature card from your graveyard under a morbid condition.

// ISD 211 — Wreath of Geists
pub(in crate::card::sets) static WREATH_OF_GEISTS: CardRecord = CardRecord::new(
    cards::WREATH_OF_GEISTS,
    "Wreath of Geists",
    CardArt::new("7604e22e-1f29-4a8f-b887-b18f43e3745e", "Jason A. Engle"),
    CardSet::Innistrad,
    CardRules::new_enchantment(mana_cost!("{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +X/+X, where X is the number of creature cards in your graveyard.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(ValueDef::CountMatchingObjects(&CREATURE_CARDS_IN_YOUR_GRAVEYARD), ValueDef::CountMatchingObjects(&CREATURE_CARDS_IN_YOUR_GRAVEYARD)),
                },
            ),
        ]),
);

// ISD 212 — Evil Twin
// Audit: blocked — Needs a copy-as-enters choice plus a retained same-name destruction ability.

// ISD 213 — Geist of Saint Traft
// Audit: blocked — Needs a token entering tapped and attacking, linked to exile at end of combat.

// ISD 214 — Grimgrin, Corpse-Born
// Audit: blocked — Needs an attack target restricted to the defending player's creatures and a linked destroy-then-counter continuation.

// ISD 215 — Olivia Voldaren
// Audit: blocked — Needs a permanent subtype-adding effect and control lasting only while the source remains controlled.

// ISD 216 — Blazing Torch
// Audit: blocked — Needs the equip procedure, attachment-granted activated abilities, and a sacrifice of the Equipment from another permanent's ability.

static EQUIPPED_CREATURE_IS_HUMAN: TriggerConditionDef =
    TriggerConditionDef::AttachedPermanentMatches {
        object: ObjectPredicateDef::Subtype("Human"),
    };

static BUTCHERS_CLEAVER_LIFELINK: AbilityDef = abilities::lifelink();

static BUTCHERS_CLEAVER_HUMAN: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::AttachedPermanent,
    effect: AppliedEffectDef::add_ability(&BUTCHERS_CLEAVER_LIFELINK),
};

static SHARPENED_PITCHFORK_FIRST_STRIKE: AbilityDef = abilities::first_strike();

static SHARPENED_PITCHFORK_HUMAN: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::AttachedPermanent,
    effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)),
};

static SILVER_INLAID_DAGGER_HUMAN: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::AttachedPermanent,
    effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(0)),
};

// ISD 217 — Butcher's Cleaver
pub(in crate::card::sets) static BUTCHERS_CLEAVER: CardRecord = CardRecord::new(
    cards::BUTCHERS_CLEAVER,
    "Butcher's Cleaver",
    CardArt::new("e141fe62-515e-4fe4-b032-81f169ec58d6", "Jason Felix"),
    CardSet::Innistrad,
    CardRules::new_artifact(mana_cost!("{3}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +3/+0.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(3),
                        ValueDef::Constant(0),
                    ),
                },
            ),
            AbilityDef::static_ability(
                "As long as equipped creature is a Human, it has lifelink.",
                EffectDef::IfCondition {
                    condition: &EQUIPPED_CREATURE_IS_HUMAN,
                    then: &BUTCHERS_CLEAVER_HUMAN,
                },
            ),
            abilities::equip(mana_cost!("{3}"), "Equip {3}"),
        ]),
);

// ISD 218 — Cellar Door
// Audit: blocked — Needs moving the bottom library card and branching on that moved card's creature type.

static COBBLED_WINGS_FLYING: AbilityDef = abilities::flying();

// ISD 219 — Cobbled Wings
pub(in crate::card::sets) static COBBLED_WINGS: CardRecord = CardRecord::new(
    cards::COBBLED_WINGS,
    "Cobbled Wings",
    CardArt::new("24abd762-e533-491a-97b6-aed40c214e9d", "Matt Stewart"),
    CardSet::Innistrad,
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature has flying.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&COBBLED_WINGS_FLYING),
                },
            ),
            abilities::equip(
                mana_cost!("{1}"),
                "Equip {1} ({1}: Attach to target creature you control. Equip only as a \
                 sorcery.)",
            ),
        ]),
);

// ISD 220 — Creepy Doll
// Audit: blocked — Needs a recorded coin flip after combat damage to a creature and a conditional destroy branch.

// ISD 221 — Demonmail Hauberk
// Audit: blocked — Needs the equip procedure with sacrificing a chosen creature as the equip cost.

// ISD 222 — Galvanic Juggernaut
pub(in crate::card::sets) static GALVANIC_JUGGERNAUT: CardRecord = CardRecord::new(
    cards::GALVANIC_JUGGERNAUT,
    "Galvanic Juggernaut",
    CardArt::new("d14bc109-d5d5-4777-90e4-bef26d106571", "Lucas Graciano"),
    CardSet::Innistrad,
    CardRules::new_artifact_creature(mana_cost!("{4}"), &["Juggernaut"], 5, 5).with_abilities(&[
        abilities::attacks_each_combat_if_able("This creature attacks each combat if able."),
        AbilityDef::static_ability(
            "This creature doesn't untap during your untap step.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
            },
        ),
        AbilityDef::triggered(
            "Whenever another creature dies, untap this creature.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::Untap {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// ISD 223 — Geistcatcher's Rig
pub(in crate::card::sets) static GEISTCATCHERS_RIG: CardRecord = CardRecord::new(
    cards::GEISTCATCHERS_RIG,
    "Geistcatcher's Rig",
    CardArt::new("cfb8ecf0-8c12-4a14-9a75-4cc5bf9e47f1", "Vincent Proce"),
    CardSet::Innistrad,
    CardRules::new_artifact_creature(mana_cost!("{6}"), &["Construct"], 4, 5)
        .with_ability(AbilityDef::triggered_with_targets(
        "When this creature enters, you may have it deal 4 damage to target creature with flying.",
        TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)),
        &[AbilityTargetDef::up_to(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasKeyword(crate::card::KeywordAbility::Flying),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
            1,
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(4),
        },
    )),
);

// ISD 224 — Ghoulcaller's Bell
pub(in crate::card::sets) static GHOULCALLERS_BELL: CardRecord = CardRecord::new(
    cards::GHOULCALLERS_BELL,
    "Ghoulcaller's Bell",
    CardArt::new("863e7c2a-698c-4dce-a10b-ca58e4affa57", "Lars Grant-West"),
    CardSet::Innistrad,
    CardRules::new_artifact(mana_cost!("{1}")).with_ability(AbilityDef::activated(
        "{T}: Each player mills a card.",
        &[AbilityCostDef::TapSource],
        EffectDef::Mill {
            player: EffectRecipientDef::EachPlayer,
            amount: ValueDef::Constant(1),
        },
    )),
);

// ISD 225 — Graveyard Shovel
// Audit: blocked — Needs the targeted player to choose one card from their graveyard and a creature-card test after exile.

// ISD 226 — Grimoire of the Dead
// Audit: blocked — Needs study counters and a graveyard sweep that changes returned creatures' colors and types.

// ISD 227 — Inquisitor's Flail
// Audit: blocked — Needs combat-damage replacement effects tied to an equipped creature.

// ISD 228 — Manor Gargoyle
// Audit: blocked — Needs indestructible to depend continuously on retaining defender while an activation temporarily removes defender.

static MASK_OF_AVACYN_HEXPROOF: AbilityDef = abilities::hexproof();

static MASK_OF_AVACYN_BONUS: [AppliedEffectDef; 2] = [
    AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(2)),
    AppliedEffectDef::add_ability(&MASK_OF_AVACYN_HEXPROOF),
];

// ISD 229 — Mask of Avacyn
pub(in crate::card::sets) static MASK_OF_AVACYN: CardRecord = CardRecord::new(
    cards::MASK_OF_AVACYN,
    "Mask of Avacyn",
    CardArt::new("4ff1acce-bed4-452c-8416-06726004f2e8", "James Paick"),
    CardSet::Innistrad,
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +1/+2 and has hexproof.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&MASK_OF_AVACYN_BONUS),
                },
            ),
            abilities::equip(mana_cost!("{3}"), "Equip {3}"),
        ]),
);

// ISD 230 — One-Eyed Scarecrow
pub(in crate::card::sets) static ONE_EYED_SCARECROW: CardRecord = CardRecord::new(
    cards::ONE_EYED_SCARECROW,
    "One-Eyed Scarecrow",
    CardArt::new("5d495d85-6458-44d5-b3b4-5e09569057e3", "Dave Kendall"),
    CardSet::Innistrad,
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Scarecrow"], 2, 3).with_abilities(&[
        abilities::defender(),
        AbilityDef::static_ability(
            "Creatures with flying your opponents control get -1/-0.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasKeyword(crate::card::KeywordAbility::Flying),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Opponent,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-1),
                    ValueDef::Constant(0),
                ),
            },
        ),
    ]),
);

// ISD 231 — Runechanter's Pike
// Audit: blocked — Needs the equip procedure and a dynamic count of instant and sorcery cards in your graveyard.

// ISD 232 — Sharpened Pitchfork
pub(in crate::card::sets) static SHARPENED_PITCHFORK: CardRecord = CardRecord::new(
    cards::SHARPENED_PITCHFORK,
    "Sharpened Pitchfork",
    CardArt::new("4ce20f19-a159-40e6-bb67-6108872ac1e0", "Winona Nelson"),
    CardSet::Innistrad,
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature has first strike.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&SHARPENED_PITCHFORK_FIRST_STRIKE),
                },
            ),
            AbilityDef::static_ability(
                "As long as equipped creature is a Human, it gets +1/+1.",
                EffectDef::IfCondition {
                    condition: &EQUIPPED_CREATURE_IS_HUMAN,
                    then: &SHARPENED_PITCHFORK_HUMAN,
                },
            ),
            abilities::equip(mana_cost!("{1}"), "Equip {1}"),
        ]),
);

// ISD 233 — Silver-Inlaid Dagger
pub(in crate::card::sets) static SILVER_INLAID_DAGGER: CardRecord = CardRecord::new(
    cards::SILVER_INLAID_DAGGER,
    "Silver-Inlaid Dagger",
    CardArt::new("f8b8162a-68f0-45df-bb25-8fd4487257a4", "Austin Hsu"),
    CardSet::Innistrad,
    CardRules::new_artifact(mana_cost!("{1}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +2/+0.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(0),
                    ),
                },
            ),
            AbilityDef::static_ability(
                "As long as equipped creature is a Human, it gets an additional +1/+0.",
                EffectDef::IfCondition {
                    condition: &EQUIPPED_CREATURE_IS_HUMAN,
                    then: &SILVER_INLAID_DAGGER_HUMAN,
                },
            ),
            abilities::equip(mana_cost!("{2}"), "Equip {2}"),
        ]),
);

// ISD 234 — Traveler's Amulet
pub(in crate::card::sets) static TRAVELERS_AMULET: CardRecord = CardRecord::new(
    cards::TRAVELERS_AMULET,
    "Traveler's Amulet",
    CardArt::new("a5b0afa7-e9f9-4751-af36-d85343fabc26", "Alan Pollack"),
    CardSet::Innistrad,
    CardRules::new_artifact(mana_cost!("{1}")).with_ability(AbilityDef::activated(
        "{1}, Sacrifice this artifact: Search your library for a basic land card, reveal it, put it into your hand, then shuffle.",
        &[
            AbilityCostDef::Mana(mana_cost!("{1}")),
            AbilityCostDef::SacrificeSource,
        ],
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Land),
                ObjectPredicateDef::Supertype(CardSupertype::Basic),
            ]),
            minimum: 0,
            maximum: 1,
            reveal: true,
            destination: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: false,
        },
    )),
);

// ISD 235 — Trepanation Blade
// Audit: blocked — Needs reveal-until-land, a revealed-card count, and the equip procedure.

// ISD 236 — Witchbane Orb
// Audit: blocked — Needs player hexproof and identifying and destroying all Curses attached to that player.

// ISD 237 — Wooden Stake
// Audit: blocked — Needs the equip procedure and a block/becomes-blocked trigger linked to the opposing Vampire.

// ISD 238 — Clifftop Retreat
pub(in crate::card::sets) static CLIFFTOP_RETREAT: CardRecord = CardRecord::new(
    cards::CLIFFTOP_RETREAT,
    "Clifftop Retreat",
    CardArt::new("fd7e1bf9-bd6a-48e3-9331-178e5142c06a", "John Avon"),
    CardSet::Innistrad,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::check_land_enters(
            "This land enters tapped unless you control a Mountain or a Plains.",
            &[BasicLandType::Mountain, BasicLandType::Plains],
        ),
        AbilityDef::activated_mana(
            "{T}: Add {R} or {W}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Red,
                ManaColor::White,
            ])),
        ),
    ]),
);

// ISD 239 — Gavony Township
pub(in crate::card::sets) static GAVONY_TOWNSHIP: CardRecord = CardRecord::new(
    cards::GAVONY_TOWNSHIP,
    "Gavony Township",
    CardArt::new("b5f73443-2fe8-424f-8e71-fc7ce1f3a3eb", "Peter Mohrbacher"),
    CardSet::Innistrad,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{2}{G}{W}, {T}: Put a +1/+1 counter on each creature you control.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}{G}{W}")),
                AbilityCostDef::TapSource,
            ],
            EffectDef::AddCounters {
                kind: CounterKind::PlusOnePlusOne,
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// ISD 240 — Ghost Quarter
pub(in crate::card::sets) static GHOST_QUARTER: CardRecord = CardRecord::new(
    cards::GHOST_QUARTER,
    "Ghost Quarter",
    CardArt::new("1c6456ed-0ffb-4d22-b252-5775076030ce", "Peter Mohrbacher"),
    CardSet::Innistrad,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets("{T}, Sacrifice this land: Destroy target land. Its controller may search their library for a basic land card, put it onto the battlefield, then shuffle.", &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource], &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Land),
        )], EffectDef::Sequence(&[
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    can_regenerate: true,
                },
                // Declining the printed "may" skips the entire search, including
                // its shuffle. If accepted, the qualified hidden-zone search
                // may still legally fail to find. The controller is read after
                // destruction from last-known information.
                EffectDef::May {
                    player: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                    effect: &EffectDef::SearchZone {
                    player: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Supertype(CardSupertype::Basic),
                    ]),
                    minimum: 0,
                    maximum: 1,
                    reveal: false,
                    destination: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                        enters_tapped: false,
                    },
                },
            ])),
    ]),
);

// ISD 241 — Hinterland Harbor
pub(in crate::card::sets) static HINTERLAND_HARBOR: CardRecord = CardRecord::new(
    cards::HINTERLAND_HARBOR,
    "Hinterland Harbor",
    CardArt::new("72f15306-56fe-4643-bb4c-4c7c12378d01", "Karl Kopinski"),
    CardSet::Innistrad,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::check_land_enters(
            "This land enters tapped unless you control a Forest or an Island.",
            &[BasicLandType::Forest, BasicLandType::Island],
        ),
        AbilityDef::activated_mana(
            "{T}: Add {G} or {U}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Green,
                ManaColor::Blue,
            ])),
        ),
    ]),
);

// ISD 242 — Isolated Chapel
pub(in crate::card::sets) static ISOLATED_CHAPEL: CardRecord = CardRecord::new(
    cards::ISOLATED_CHAPEL,
    "Isolated Chapel",
    CardArt::new("b3c1a371-5ded-4a3a-bf96-503c4f1a665d", "Cliff Childs"),
    CardSet::Innistrad,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::check_land_enters(
            "This land enters tapped unless you control a Plains or a Swamp.",
            &[BasicLandType::Plains, BasicLandType::Swamp],
        ),
        AbilityDef::activated_mana(
            "{T}: Add {W} or {B}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Black,
            ])),
        ),
    ]),
);

// ISD 243 — Kessig Wolf Run
pub(in crate::card::sets) static KESSIG_WOLF_RUN: CardRecord = CardRecord::new(
    cards::KESSIG_WOLF_RUN,
    "Kessig Wolf Run",
    CardArt::new("4a8447fe-7368-470a-911a-1083ec6cc831", "Eytan Zana"),
    CardSet::Innistrad,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets(
            "{X}{R}{G}, {T}: Target creature gets +X/+0 and gains trample until end of turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{X}{R}{G}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::ChosenX,
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&abilities::trample()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
    ]),
);

// ISD 244 — Moorland Haunt
pub(in crate::card::sets) static MOORLAND_HAUNT: CardRecord = CardRecord::new(
    cards::MOORLAND_HAUNT,
    "Moorland Haunt",
    CardArt::new("1d5569e3-278c-4cf3-860e-712010333fe6", "James Paick"),
    CardSet::Innistrad,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{W}{U}, {T}, Exile a creature card from your graveyard: Create a 1/1 white Spirit creature token with flying.",
            &[
                AbilityCostDef::Mana(mana_cost!("{W}{U}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::ExileCardFromGraveyard(ObjectPredicateDef::HasType(
                    CardType::Creature,
                )),
            ],
            EffectDef::CreateToken {
                token: cards::SPIRIT_TOKEN_1_1_WHITE,
                count: ValueDef::Constant(1),
                tapped: false,
            },
        ),
    ]),
);

// ISD 245 — Nephalia Drownyard
pub(in crate::card::sets) static NEPHALIA_DROWNYARD: CardRecord = CardRecord::new(
    cards::NEPHALIA_DROWNYARD,
    "Nephalia Drownyard",
    CardArt::new("ef058312-6926-49f8-ae72-a8d60fedbf6c", "Cliff Childs"),
    CardSet::Innistrad,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets(
            "{1}{U}{B}, {T}: Target player mills three cards.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{U}{B}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Mill {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
            },
        ),
    ]),
);

// ISD 246 — Shimmering Grotto
// Audit: blocked — Its {1}, {T} mana ability is not executable because the mana runtime rejects mana abilities with a mana activation cost.

// ISD 247 — Stensia Bloodhall
pub(in crate::card::sets) static STENSIA_BLOODHALL: CardRecord = CardRecord::new(
    cards::STENSIA_BLOODHALL,
    "Stensia Bloodhall",
    CardArt::new("cc2741d8-2c02-4acd-8ca2-55b4bf6aef1c", "John Avon"),
    CardSet::Innistrad,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets(
            "{3}{B}{R}, {T}: This land deals 2 damage to target player or planeswalker.",
            &[
                AbilityCostDef::Mana(mana_cost!("{3}{B}{R}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
        ),
    ]),
);

// ISD 248 — Sulfur Falls
pub(in crate::card::sets) static SULFUR_FALLS: CardRecord = CardRecord::new(
    cards::SULFUR_FALLS,
    "Sulfur Falls",
    CardArt::new("4968b65d-50e5-4d7e-b78b-cdada1cbf7a7", "Cliff Childs"),
    CardSet::Innistrad,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::check_land_enters(
            "This land enters tapped unless you control an Island or a Mountain.",
            &[BasicLandType::Island, BasicLandType::Mountain],
        ),
        AbilityDef::activated_mana(
            "{T}: Add {U} or {R}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[ManaColor::Blue, ManaColor::Red])),
        ),
    ]),
);

// ISD 249 — Woodland Cemetery
pub(in crate::card::sets) static WOODLAND_CEMETERY: CardRecord = CardRecord::new(
    cards::WOODLAND_CEMETERY,
    "Woodland Cemetery",
    CardArt::new("67139101-ec5e-434b-be3a-21338cc33840", "Lars Grant-West"),
    CardSet::Innistrad,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::check_land_enters(
            "This land enters tapped unless you control a Swamp or a Forest.",
            &[BasicLandType::Swamp, BasicLandType::Forest],
        ),
        AbilityDef::activated_mana(
            "{T}: Add {B} or {G}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Black,
                ManaColor::Green,
            ])),
        ),
    ]),
);
pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ABBEY_GRIFFIN,
    &ANGEL_OF_FLIGHT_ALABASTER,
    &AVACYNIAN_PRIEST,
    &CHAMPION_OF_THE_PARISH,
    &CHAPEL_GEIST,
    &CLOISTERED_YOUTH,
    &DOOMED_TRAVELER,
    &ELDER_CATHAR,
    &FEELING_OF_DREAD,
    &FIEND_HUNTER,
    &GALLOWS_WARDEN,
    &GEIST_HONORED_MONK,
    &MAUSOLEUM_GUARD,
    &MIDNIGHT_HAUNTING,
    &MOMENT_OF_HEROISM,
    &PURIFY_THE_GRAVE,
    &RALLY_THE_PEASANTS,
    &REBUKE,
    &SELFLESS_CATHAR,
    &SILVERCHASE_FOX,
    &SLAYER_OF_THE_WICKED,
    &SMITE_THE_MONSTROUS,
    &SPECTRAL_RIDER,
    &THRABEN_PUREBLOODS,
    &THRABEN_SENTRY,
    &UNRULY_MOB,
    &URGENT_EXORCISM,
    &VILLAGE_BELL_RINGER,
    &VOICELESS_SPIRIT,
    &ARMORED_SKAAB,
    &BATTLEGROUND_GEIST,
    &CLAUSTROPHOBIA,
    &CURIOSITY,
    &DISSIPATE,
    &DREAM_TWIST,
    &FORBIDDEN_ALCHEMY,
    &FORTRESS_CRAB,
    &GRASP_OF_PHANTOMS,
    &HYSTERICAL_BLINDNESS,
    &INVISIBLE_STALKER,
    &LANTERN_SPIRIT,
    &LOST_IN_THE_MIST,
    &MAKESHIFT_MAULER,
    &MOON_HERON,
    &MURDER_OF_CROWS,
    &SELHOFF_OCCULTIST,
    &SENSORY_DEPRIVATION,
    &SILENT_DEPARTURE,
    &SNAPCASTER_MAGE,
    &SPECTRAL_FLIGHT,
    &STITCHED_DRAKE,
    &STITCHERS_APPRENTICE,
    &THINK_TWICE,
    &ALTARS_REAP,
    &BLOODGIFT_DEMON,
    &BUMP_IN_THE_NIGHT,
    &DEAD_WEIGHT,
    &DIREGRAF_GHOUL,
    &FALKENRATH_NOBLE,
    &GHOULCALLERS_CHANT,
    &GRUESOME_DEFORMITY,
    &LILIANA_OF_THE_VEIL,
    &MANOR_SKELETON,
    &MARKOV_PATRICIAN,
    &MAW_OF_THE_MIRE,
    &MOAN_OF_THE_UNHALLOWED,
    &ROTTING_FENSNAKE,
    &SCREECHING_BAT,
    &SEVER_THE_BLOODLINE,
    &SKELETAL_GRIMACE,
    &STROMKIRK_PATROL,
    &TYPHOID_RATS,
    &UNBURIAL_RITES,
    &VAMPIRE_INTERLOPER,
    &VICTIM_OF_NIGHT,
    &VILLAGE_CANNIBALS,
    &WALKING_CORPSE,
    &ANCIENT_GRUDGE,
    &BALEFIRE_DRAGON,
    &BLASPHEMOUS_ACT,
    &BLOODCRAZED_NEONATE,
    &BRIMSTONE_VOLLEY,
    &CROSSWAY_VAMPIRE,
    &DESPERATE_RAVINGS,
    &DEVILS_PLAY,
    &FALKENRATH_MARAUDERS,
    &FERAL_RIDGEWOLF,
    &FUROR_OF_THE_BITTEN,
    &GEISTFLAME,
    &HANWEIR_WATCHKEEP,
    &INSTIGATOR_GANG,
    &INTO_THE_MAW_OF_HELL,
    &KESSIG_WOLF,
    &NIGHTBIRDS_CLUTCHES,
    &PAST_IN_FLAMES,
    &PITCHBURN_DEVILS,
    &RAGE_THROWER,
    &RAKISH_HEIR,
    &RECKLESS_WAIF,
    &RIOT_DEVILS,
    &ROLLING_TEMBLOR,
    &SCOURGE_OF_GEIER_REACH,
    &SKIRSDAG_CULTIST,
    &STROMKIRK_NOBLE,
    &TORMENTED_PARIAH,
    &TRAITOROUS_BLOOD,
    &VAMPIRIC_FURY,
    &VILLAGE_IRONSMITH,
    &AMBUSH_VIPER,
    &AVACYNS_PILGRIM,
    &BONEYARD_WURM,
    &BRAMBLECRUSH,
    &DARKTHICKET_WOLF,
    &DAYBREAK_RANGER,
    &ELDER_OF_LAURELS,
    &FESTERHIDE_BOAR,
    &FULL_MOONS_RISE,
    &GARRUK_RELENTLESS,
    &GATSTAF_SHEPHERD,
    &GRIZZLED_OUTCASTS,
    &KINDERCATCH,
    &LUMBERKNOT,
    &MAYOR_OF_AVABRUCK,
    &MULCH,
    &ORCHARD_SPIRIT,
    &PREY_UPON,
    &RANGERS_GUILE,
    &SOMBERWALD_SPIDER,
    &SPIDER_SPAWNING,
    &SPIDERY_GRASP,
    &SPLINTERFRIGHT,
    &TRAVEL_PREPARATIONS,
    &ULVENWALD_MYSTICS,
    &VILLAGERS_OF_ESTWALD,
    &WREATH_OF_GEISTS,
    &BUTCHERS_CLEAVER,
    &COBBLED_WINGS,
    &GALVANIC_JUGGERNAUT,
    &GEISTCATCHERS_RIG,
    &GHOULCALLERS_BELL,
    &MASK_OF_AVACYN,
    &ONE_EYED_SCARECROW,
    &SHARPENED_PITCHFORK,
    &SILVER_INLAID_DAGGER,
    &TRAVELERS_AMULET,
    &CLIFFTOP_RETREAT,
    &GAVONY_TOWNSHIP,
    &GHOST_QUARTER,
    &HINTERLAND_HARBOR,
    &ISOLATED_CHAPEL,
    &KESSIG_WOLF_RUN,
    &MOORLAND_HAUNT,
    &NEPHALIA_DROWNYARD,
    &STENSIA_BLOODHALL,
    &SULFUR_FALLS,
    &WOODLAND_CEMETERY,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&onslaught::NATURALIZE), // ISD 197
    PrintingRecord::reprint(&alpha::PLAINS),         // ISD 250
    PrintingRecord::alternate(&alpha::PLAINS, 1),    // ISD 251
    PrintingRecord::alternate(&alpha::PLAINS, 2),    // ISD 252
    PrintingRecord::reprint(&alpha::ISLAND),         // ISD 253
    PrintingRecord::alternate(&alpha::ISLAND, 1),    // ISD 254
    PrintingRecord::alternate(&alpha::ISLAND, 2),    // ISD 255
    PrintingRecord::reprint(&alpha::SWAMP),          // ISD 256
    PrintingRecord::alternate(&alpha::SWAMP, 1),     // ISD 257
    PrintingRecord::alternate(&alpha::SWAMP, 2),     // ISD 258
    PrintingRecord::reprint(&alpha::MOUNTAIN),       // ISD 259
    PrintingRecord::alternate(&alpha::MOUNTAIN, 1),  // ISD 260
    PrintingRecord::alternate(&alpha::MOUNTAIN, 2),  // ISD 261
    PrintingRecord::reprint(&alpha::FOREST),         // ISD 262
    PrintingRecord::alternate(&alpha::FOREST, 1),    // ISD 263
    PrintingRecord::alternate(&alpha::FOREST, 2),    // ISD 264
];
