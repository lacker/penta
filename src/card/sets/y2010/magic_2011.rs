//! Magic 2011 cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::AbilityCostDef;
use crate::AbilityTargetDef;
use crate::AbilityTargetPredicate;
use crate::BasicLandType;
use crate::ComparisonDef;
use crate::ManaColor;
use crate::ObjectQueryDef;
use crate::ObjectSetDef;
use crate::PlayerRefDef;
use crate::PlayerRelation;
use crate::ResolvedEffectDurationDef;
use crate::TargetChooserDef;
use crate::TargetIndex;
use crate::card::ConditionalStaticEffectDef;
use crate::card::ObjectSetCountConditionDef;
use crate::card::ScaledValueDef;
use crate::card::StaticApplyDef;
use crate::card::{
    AbilityDef, AppliedEffectDef, AppliedRuleDef, CardRules, CardSet, CardType,
    CastTimingPermissionDef, DiscardSelectionDef, EffectDef, EffectRecipientDef,
    ObjectPredicateDef, TriggerEventDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::mana_cost;

// M11 6 — Assault Griffin
pub(in crate::card::sets) static ASSAULT_GRIFFIN: CardRecord = CardRecord::new(
    CardSet::Magic2011,
    "Assault Griffin",
    "f72ced22-1f2c-4fa6-a938-8ebe2c15cc8d",
    "Jesper Ejsing",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Griffin"], 3, 2)
        .with_ability(abilities::flying()),
);

// M11 21 — Leyline of Sanctity
pub(in crate::card::sets) static LEYLINE_OF_SANCTITY: CardRecord = CardRecord::new(
    CardSet::Magic2011,
    "Leyline of Sanctity",
    "262de9ae-d641-4f0e-af6a-03ce0e1c91d3",
    "Ryan Pancoast",
    // Four mana for nothing at all, or nothing at all for a wall the
    // discard and the burn cannot see past.
    CardRules::new_enchantment(mana_cost!("{2}{W}{W}")).with_abilities(&[
        abilities::begin_game_on_battlefield("If this card is in your opening hand, you may begin the game with it on the battlefield."),
        AbilityDef::static_ability(
            "You have hexproof. (You can't be the target of spells or abilities your opponents control.)",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Controller,
                // The player, not the permanents: what this stops is a spell
                // that names its controller, and nothing that names a
                // creature they control.
                effect: AppliedEffectDef::Rule(AppliedRuleDef::PlayerRule(
                    crate::card::PlayerRuleDef::Hexproof,
                )),
            },
        ),
    ]),
);

// M11 22 — Mighty Leap
pub(in crate::card::sets) static MIGHTY_LEAP: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2011,
    "Mighty Leap",
    "bf8e0f93-a450-4188-a735-d601a59ab108",
    "rk post",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +2/+2 and gains flying until end of turn.",
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
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// M11 25 — Roc Egg
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROC_EGG: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2011,
    "Roc Egg",
    "1dca2c1f-3835-478b-860c-51b2036221b2",
    "Paul Bonner",
    crate::card::CardRules::unsupported(),
);

// M11 30 — Silence (reprint)
const SILENCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2009::magic_2010::SILENCE,
    "37b70d17-e4ec-4731-8892-b444f82be7a2",
    "Wayne Reynolds",
);

// M11 35 — Sun Titan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUN_TITAN: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2011,
    "Sun Titan",
    "d8db2b8e-dce9-49b7-833f-381ee55288cb",
    "Todd Lockwood",
    crate::card::CardRules::unsupported(),
);

// M11 38 — War Priest of Thune
pub(in crate::card::sets) static WAR_PRIEST_OF_THUNE: CardRecord = CardRecord::new(
    CardSet::Magic2011,
    "War Priest of Thune",
    "da7d96db-109d-498e-ae10-1430718c33da",
    "Izzy",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Cleric"], 2, 2).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this creature enters, you may destroy target enchantment.",
            &[AbilityTargetDef {
                predicate: AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Enchantment),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                // "You may" is an optional target: declining to choose one is how the
                // trigger does nothing, so the minimum is zero rather than one.
                minimum: 0,
                maximum: 1,
                exact_count: None,
                divided_total: None,
                another: false,
                excludes_source: false,
                chooser: TargetChooserDef::Controller,
            }],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
        ),
    ]),
);

// M11 41 — Aether Adept
pub(in crate::card::sets) static AETHER_ADEPT: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2011,
    "Aether Adept",
    "0b551dab-1a81-406d-b708-b3b7300eb02e",
    "Eric Deschamps",
    CardRules::new_creature(mana_cost!("{1}{U}{U}"), &["Human", "Wizard"], 2, 2).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, return target creature to its owner's hand.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        ),
    ),
);

// M11 42 — Air Servant
pub(in crate::card::sets) static AIR_SERVANT: CardRecord = CardRecord::new(
    CardSet::Magic2011,
    "Air Servant",
    "0f46eb67-a50d-4910-9919-1bb2ca1c0dad",
    "Lars Grant-West",
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Elemental"], 4, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated_with_targets(
            "{2}{U}: Tap target creature with flying.",
            &[AbilityCostDef::Mana(mana_cost!("{2}{U}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasKeyword(crate::card::KeywordAbility::Flying),
                ]),
            )],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// M11 44 — Armored Cancrix
pub(in crate::card::sets) static ARMORED_CANCRIX: CardRecord = CardRecord::new(
    CardSet::Magic2011,
    "Armored Cancrix",
    "53ef0757-8eb0-4384-bf8e-9a7340144dfa",
    "Tomasz Jedruszek",
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Crab"], 2, 5),
);

// M11 55 — Frost Titan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FROST_TITAN: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2011,
    "Frost Titan",
    "065addc8-c235-43cc-a54f-b582826e5df1",
    "Mike Bierek",
    crate::card::CardRules::unsupported(),
);

// M11 56 — Harbor Serpent
pub(in crate::card::sets) static HARBOR_SERPENT: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2011,
    "Harbor Serpent",
    "aa10b43f-eb63-4999-92a0-56826031b686",
    "Daarken",
    CardRules::new_creature(mana_cost!("{4}{U}{U}"), &["Serpent"], 5, 5).with_abilities(&[
        abilities::landwalk(BasicLandType::Island),
        AbilityDef::static_ability(
            "This creature can't attack unless there are five or more Islands on the battlefield.",
            EffectDef::ConditionalStatic(ConditionalStaticEffectDef {
                condition: ObjectSetCountConditionDef {
                    objects: &ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    )),
                    filter: None,
                    comparison: ComparisonDef::Less,
                    amount: 5,
                },
                then: StaticApplyDef {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_ATTACK),
                },
            }),
        ),
    ]),
);

// M11 59 — Jace's Erasure
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JACE_S_ERASURE: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2011,
    "Jace's Erasure",
    "3662d1cc-1279-409f-9f0a-9c15c3407103",
    "Jason Chan",
    crate::card::CardRules::unsupported(),
);

// M11 61 — Leyline of Anticipation
pub(in crate::card::sets) static LEYLINE_OF_ANTICIPATION: CardRecord = CardRecord::new(
    CardSet::Magic2011,
    "Leyline of Anticipation",
    "d7dbb092-3bb0-445e-ab26-d939cac92a73",
    "Charles Urbach",
    CardRules::new_enchantment(mana_cost!("{2}{U}{U}")).with_abilities(&[
        abilities::begin_game_on_battlefield("If this card is in your opening hand, you may begin the game with it on the battlefield."),
        AbilityDef::static_ability(
            "You may cast spells as though they had flash.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MayCastAsThoughItHadFlash(
                    CastTimingPermissionDef::new(ObjectPredicateDef::Any),
                )),
            },
        ),
    ]),
);

// M11 66 — Merfolk Spy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MERFOLK_SPY: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2011,
    "Merfolk Spy",
    "b5ae05cc-116b-4268-ba78-709aeff36ab1",
    "Matt Cavotta & Richard Whitters",
    crate::card::CardRules::unsupported(),
);

// M11 70 — Preordain
pub(in crate::card::sets) static PREORDAIN: CardRecord = CardRecord::new(
    CardSet::Magic2011,
    "Preordain",
    "e3868c3d-4fcd-444b-866f-0f8e50ce7b67",
    "Svetlin Velinov",
    CardRules::new_sorcery(mana_cost!("{U}")).with_ability(AbilityDef::spell(
        "Scry 2, then draw a card.",
        EffectDef::Sequence(&[
            abilities::scry(ValueDef::Constant(2)),
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// M11 71 — Redirect
pub(in crate::card::sets) static REDIRECT: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2011,
    "Redirect",
    "60bae44b-c6f2-40bf-a427-aee5cfbdfea9",
    "Izzy",
    CardRules::new_instant(mana_cost!("{U}{U}")).with_ability(AbilityDef::spell_with_targets(
        "You may choose new targets for target spell.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Spell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::ChangeStackTargets(&crate::card::ChangeStackTargetsDef {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            chooser: PlayerRefDef::EffectController,
            change: crate::card::StackTargetChangeDef::ChooseNew {
                optional: true,
                restriction: None,
            },
        }),
    )),
);

// M11 72 — Scroll Thief
pub(in crate::card::sets) static SCROLL_THIEF: CardRecord = CardRecord::new(
    CardSet::Magic2011,
    "Scroll Thief",
    "6f3b2808-58d9-4e27-a6c2-6db66191151e",
    "Alex Horley-Orlandelli",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Merfolk", "Rogue"], 1, 3).with_ability(
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, draw a card.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
    ),
);

// M11 74 — Stormtide Leviathan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STORMTIDE_LEVIATHAN: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2011,
    "Stormtide Leviathan",
    "0e7f3fb6-93ce-4bc9-8efd-11af5a46218f",
    "Karl Kopinski",
    crate::card::CardRules::unsupported(),
);

// M11 75 — Time Reversal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TIME_REVERSAL: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2011,
    "Time Reversal",
    "1468c851-b20e-4c78-9fcb-45e60b7149db",
    "Howard Lyon",
    crate::card::CardRules::unsupported(),
);

// M11 80 — Water Servant
pub(in crate::card::sets) static WATER_SERVANT: CardRecord = CardRecord::new(
    CardSet::Magic2011,
    "Water Servant",
    "02a3062e-8b83-4ee4-8139-8eee84df37fe",
    "Igor Kieryluk",
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Elemental"], 3, 4).with_abilities(&[
        AbilityDef::activated(
            "{U}: This creature gets +1/-1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{U}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(-1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated(
            "{U}: This creature gets -1/+1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{U}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// M11 97 — Grave Titan
pub(in crate::card::sets) static GRAVE_TITAN: CardRecord = CardRecord::new(
    CardSet::Magic2011,
    "Grave Titan",
    "5fa6d385-6b8e-45ad-83dc-b477799c05a5",
    "Nils Hamm",
    // Ten power over three bodies for six mana, and killing the Titan still
    // leaves four of it behind.
    CardRules::new_creature(mana_cost!("{4}{B}{B}"), &["Giant"], 6, 6)
        .with_abilities(&[
            abilities::deathtouch(),
            AbilityDef::triggered(
                "Whenever this creature enters or attacks, create two 2/2 black Zombie creature tokens.",
                // One printed ability with two ways in, the way every Titan prints it: a
                // Titan that lands and then attacks makes four Zombies, and it makes them
                // as two separate triggers.
                TriggerEventDef::AnyOf(&[
                    TriggerEventDef::zone_changed(
                        ObjectPredicateDef::Source,
                        None,
                        Some(ZoneKind::Battlefield),
                    ),
                    TriggerEventDef::attacks(ObjectPredicateDef::Source),
                ]),
                EffectDef::create_creature_token(&["Zombie"], &[ManaColor::Black], 2, 2).with_amount(2),
            ),
        ]),
);

// M11 104 — Liliana's Specter
pub(in crate::card::sets) static LILIANA_S_SPECTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2011,
    "Liliana's Specter",
    "33122581-39fd-44a0-b928-f73e39a0c0f1",
    "Vance Kovacs",
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Specter"], 2, 1).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, each opponent discards a card.",
            EffectDef::Discard {
                recipient: EffectRecipientDef::Opponent,
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ),
    ]),
);

// M11 109 — Nightwing Shade
pub(in crate::card::sets) static NIGHTWING_SHADE: CardRecord = CardRecord::new(
    CardSet::Magic2011,
    "Nightwing Shade",
    "ba6232c3-f840-450a-8583-540aec0f17ed",
    "Lucas Graciano",
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Shade"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{1}{B}: This creature gets +1/+1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// M11 110 — Phylactery Lich
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHYLACTERY_LICH: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2011,
    "Phylactery Lich",
    "9d088983-92c1-4f4d-8abf-dd20347495b5",
    "Michael Komarck",
    crate::card::CardRules::unsupported(),
);

// M11 111 — Quag Sickness
static QUAG_SICKNESS_PENALTY: ValueDef = ValueDef::Scaled(&ScaledValueDef::new(
    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
        ObjectPredicateDef::Subtype("Swamp"),
        &[ZoneKind::Battlefield],
        PlayerRelation::You,
    )),
    -1,
));

pub(in crate::card::sets) static QUAG_SICKNESS: CardRecord = CardRecord::new(
    CardSet::Magic2011,
    "Quag Sickness",
    "21d56d13-b9de-44db-b235-4f7eea60f424",
    "Martina Pilcerova",
    CardRules::new_enchantment(mana_cost!("{2}{B}"))
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
                "Enchanted creature gets -1/-1 for each Swamp you control.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        QUAG_SICKNESS_PENALTY,
                        QUAG_SICKNESS_PENALTY,
                    ),
                },
            ),
        ]),
);

// M11 130 — Combust
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COMBUST: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2011,
    "Combust",
    "cf23a422-25a7-4c8a-9cff-24563ec20ea7",
    "Jaime Jones",
    crate::card::CardRules::unsupported(),
);

// M11 146 — Inferno Titan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INFERNO_TITAN: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2011,
    "Inferno Titan",
    "f1e4a028-6462-4373-9864-a8adfc78d52b",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// M11 148 — Leyline of Punishment
pub(in crate::card::sets) static LEYLINE_OF_PUNISHMENT: CardRecord = CardRecord::new(
    CardSet::Magic2011,
    "Leyline of Punishment",
    "51a2eec5-f892-4466-b6c6-960626ba5640",
    "Charles Urbach",
    CardRules::new_enchantment(mana_cost!("{2}{R}{R}")).with_abilities(&[
        abilities::begin_game_on_battlefield("If this card is in your opening hand, you may begin the game with it on the battlefield."),
        AbilityDef::static_ability(
            "Players can't gain life. Damage can't be prevented.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::EachPlayer,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::Rule(AppliedRuleDef::CannotGainLife),
                    AppliedEffectDef::Rule(AppliedRuleDef::PlayerRule(
                        crate::card::PlayerRuleDef::DamageCannotBePrevented,
                    )),
                ]),
            },
        ),
    ]),
);

// M11 151 — Manic Vandal
pub(in crate::card::sets) static MANIC_VANDAL: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2011,
    "Manic Vandal",
    "a503697a-4940-4b8f-98b1-5ea9151866fa",
    "Christopher Moeller",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Warrior"], 2, 2).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, destroy target artifact.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Artifact),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
        ),
    ),
);

// M11 155 — Reverberate
pub(in crate::card::sets) static REVERBERATE: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2011,
    "Reverberate",
    "dd435013-0ab9-42f4-985c-66ea2b3760e9",
    "jD",
    CardRules::new_instant(mana_cost!("{R}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Copy target instant or sorcery spell. You may choose new targets for the copy.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Spell,
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                    ]),
                ]),
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::CopyStackObject(&crate::card::CopyStackObjectDef {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            controller: PlayerRefDef::EffectController,
            count: ValueDef::Constant(1),
            retarget: true,
            colors: None,
        }),
    )),
);

// M11 157 — Thunder Strike
pub(in crate::card::sets) static THUNDER_STRIKE: CardRecord = CardRecord::new(
    CardSet::Magic2011,
    "Thunder Strike",
    "1f94f88b-d928-4364-9126-231eabf14086",
    "Wayne Reynolds",
    CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +2/+0 and gains first strike until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(0),
                ),
                AppliedEffectDef::add_ability(&abilities::first_strike()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// M11 158 — Volcanic Strength
pub(in crate::card::sets) static VOLCANIC_STRENGTH: CardRecord = CardRecord::new(
    CardSet::Magic2011,
    "Volcanic Strength",
    "bda0bffa-c58c-4630-8899-a1b332a7b8dc",
    "Izzy",
    CardRules::new_enchantment(mana_cost!("{1}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
        AbilityDef::spell_with_targets("Enchant creature", &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )], EffectDef::Attach {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            }),
        AbilityDef::static_ability(
            "Enchanted creature gets +2/+2 and has mountainwalk. (It can't be blocked as long as defending player controls a Mountain.)",
            EffectDef::Sequence(&[
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(2), ValueDef::Constant(2)),
                },
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::mountainwalk()),
                },
            ]),
        ),
    ]),
);

// M11 162 — Autumn's Veil
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AUTUMN_S_VEIL: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2011,
    "Autumn's Veil",
    "7e354ce5-b4c1-4a9c-99d1-7624301b594b",
    "Kekai Kotaki",
    crate::card::CardRules::unsupported(),
);

// M11 166 — Brindle Boar
pub(in crate::card::sets) static BRINDLE_BOAR: CardRecord = CardRecord::new(
    CardSet::Magic2011,
    "Brindle Boar",
    "f2bc665c-d507-4de3-a8e8-731cc8487840",
    "Dave Allsop",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Boar"], 2, 2).with_ability(
        AbilityDef::activated(
            "Sacrifice this creature: You gain 4 life.",
            &[AbilityCostDef::SacrificeSource],
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(4),
            },
        ),
    ),
);

// M11 176 — Garruk's Companion
pub(in crate::card::sets) static GARRUK_S_COMPANION: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2011,
    "Garruk's Companion",
    "863c9a10-d83f-415b-adf2-2d0f870410b2",
    "Efrem Palacios",
    CardRules::new_creature(mana_cost!("{G}{G}"), &["Beast"], 3, 2)
        .with_abilities(&[abilities::trample()]),
);

// M11 177 — Garruk's Packleader
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GARRUK_S_PACKLEADER: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2011,
    "Garruk's Packleader",
    "dfaef299-7879-4f52-8ee4-701ed150b930",
    "Nils Hamm",
    crate::card::CardRules::unsupported(),
);

// M11 180 — Greater Basilisk
pub(in crate::card::sets) static GREATER_BASILISK: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2011,
    "Greater Basilisk",
    "482f169d-8acd-4ee3-a54c-6df6cbeb7eca",
    "James Ryman",
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Basilisk"], 3, 5)
        .with_abilities(&[abilities::deathtouch()]),
);

// M11 183 — Leyline of Vitality
pub(in crate::card::sets) static LEYLINE_OF_VITALITY: CardRecord = CardRecord::new(
    CardSet::Magic2011,
    "Leyline of Vitality",
    "f5318113-9dfb-492c-9151-de90951d881e",
    "Jim Nelson",
    CardRules::new_enchantment(mana_cost!("{2}{G}{G}")).with_abilities(&[
        abilities::begin_game_on_battlefield("If this card is in your opening hand, you may begin the game with it on the battlefield."),
        AbilityDef::static_ability(
            "Creatures you control get +0/+1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    crate::card::PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(1),
                ),
            },
        ),
        AbilityDef::triggered(
            "Whenever a creature you control enters, you may gain 1 life.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(crate::card::PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            },
        ),
    ]),
);

// M11 192 — Primeval Titan
pub(in crate::card::sets) static PRIMEVAL_TITAN: CardRecord = CardRecord::new(
    CardSet::Magic2011,
    "Primeval Titan",
    "feee9327-b937-46ba-a2aa-6c015ab6cdd5",
    "Aleksi Briclot",
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Giant"], 6, 6).with_abilities(&[
        abilities::trample(),
        AbilityDef::triggered(
            "Whenever this creature enters or attacks, you may search your library for up to two land cards, put them onto the battlefield tapped, then shuffle.",
            // One printed ability with two ways in, not two abilities: the card says
            // "enters or attacks", and a Titan that does both in a turn triggers twice
            // for the same reason it would have anyway.
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
            ]),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                // Any land card, not just a basic: the two it finds are usually the two the
                // deck was built around.
                effect: &EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::HasType(CardType::Land),
                    minimum: 0,
                    maximum: ValueDef::Constant(2),
                    reveal: false,
                    destination: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: true,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            },
        ),
    ]),
);

// M11 196 — Sacred Wolf
pub(in crate::card::sets) static SACRED_WOLF: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2011,
    "Sacred Wolf",
    "a2bffe20-c469-4ac8-a8a9-361a244f4cfe",
    "Matt Stewart",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Wolf"], 3, 1)
        .with_abilities(&[abilities::hexproof()]),
);

// M11 206 — Elixir of Immortality
pub(in crate::card::sets) static ELIXIR_OF_IMMORTALITY: CardRecord = CardRecord::new(
    crate::card::CardSet::Magic2011,
    "Elixir of Immortality",
    "99bd4740-9b1f-40a6-a14d-2c0d642b848b",
    "Zoltan Boros & Gabor Szikszai",
    CardRules::new_artifact(mana_cost!("{1}")).with_ability(AbilityDef::activated(
        "{2}, {T}: You gain 5 life. Shuffle this artifact and your graveyard into their owner's library.",
        &[
            AbilityCostDef::Mana(mana_cost!("{2}")),
            AbilityCostDef::TapSource,
        ],
        EffectDef::Sequence(&[
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(5),
            },
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Source,
                zone: ZoneKind::Library,
                placement: ZonePlacement::Top,
            },
            EffectDef::MoveToZone {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Graveyard],
                    PlayerRelation::You,
                ),
                zone: ZoneKind::Library,
                placement: ZonePlacement::Top,
            },
            EffectDef::ShuffleLibrary {
                player: EffectRecipientDef::Controller,
            },
        ]),
    )),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ASSAULT_GRIFFIN,
    &LEYLINE_OF_SANCTITY,
    &MIGHTY_LEAP,
    &ROC_EGG,
    &SUN_TITAN,
    &WAR_PRIEST_OF_THUNE,
    &AETHER_ADEPT,
    &AIR_SERVANT,
    &ARMORED_CANCRIX,
    &FROST_TITAN,
    &HARBOR_SERPENT,
    &JACE_S_ERASURE,
    &LEYLINE_OF_ANTICIPATION,
    &MERFOLK_SPY,
    &PREORDAIN,
    &REDIRECT,
    &SCROLL_THIEF,
    &STORMTIDE_LEVIATHAN,
    &TIME_REVERSAL,
    &WATER_SERVANT,
    &GRAVE_TITAN,
    &LILIANA_S_SPECTER,
    &NIGHTWING_SHADE,
    &PHYLACTERY_LICH,
    &QUAG_SICKNESS,
    &COMBUST,
    &INFERNO_TITAN,
    &LEYLINE_OF_PUNISHMENT,
    &MANIC_VANDAL,
    &REVERBERATE,
    &THUNDER_STRIKE,
    &VOLCANIC_STRENGTH,
    &AUTUMN_S_VEIL,
    &BRINDLE_BOAR,
    &GARRUK_S_COMPANION,
    &GARRUK_S_PACKLEADER,
    &GREATER_BASILISK,
    &LEYLINE_OF_VITALITY,
    &PRIMEVAL_TITAN,
    &SACRED_WOLF,
    &ELIXIR_OF_IMMORTALITY,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[SILENCE_REPRINT];
