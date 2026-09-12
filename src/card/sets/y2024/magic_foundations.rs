//! Foundations card inventory.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::ActivationTimingDef;
use crate::card::AlternativeCastKindDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CardTypeSet;
use crate::card::CastTimingPermissionDef;
use crate::card::CharacteristicOperationDef;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseCardsFromCollectionDef;
use crate::card::ChooseDef;
use crate::card::ChooseForEachPlayerDef;
use crate::card::CollectionInspectionDef;
use crate::card::ComparisonDef;
use crate::card::CopyAbilityDef;
use crate::card::CopyExceptionsDef;
use crate::card::CostDef;
use crate::card::CostQuantityDef;
use crate::card::CountConditionDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::CreatureTypeSetDef;
use crate::card::DamageEventMatcherDef;
use crate::card::DamagePreventionDef;
use crate::card::DiscardSelectionDef;
use crate::card::DrawEventMatcherDef;
use crate::card::EffectChoiceDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::EmblemCharacteristics;
use crate::card::ExilePlayDurationDef;
use crate::card::InstalledTriggerDef;
use crate::card::ManaColor;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectCollectionSourceDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::PayOrDef;
use crate::card::PerPlayerSelectionDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::PowerToughnessOperationDef;
use crate::card::RandomizeObjectOrderDef;
use crate::card::ReplacementEffectDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::RevealObjectsDef;
use crate::card::SetOperationDef;
use crate::card::SubtypeDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TokenCopyDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::card::tokens;
use crate::mana_cost;

use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y1994::antiquities as catalog_atq;
use crate::card::sets::y1994::the_dark as catalog_drk;
use crate::card::sets::y1996::mirage as catalog_mir;
use crate::card::sets::y1997::tempest as catalog_tmp;
use crate::card::sets::y1998::portal_second_age as catalog_p02;
use crate::card::sets::y1998::urzas_saga as catalog_usg;
use crate::card::sets::y2000::invasion as catalog_inv;
use crate::card::sets::y2001::apocalypse as catalog_apc;
use crate::card::sets::y2001::odyssey as catalog_ody;
use crate::card::sets::y2002::onslaught as catalog_ons;
use crate::card::sets::y2003::mirrodin as catalog_mrd;
use crate::card::sets::y2003::scourge as catalog_scg;
use crate::card::sets::y2004::champions_of_kamigawa as catalog_chk;
use crate::card::sets::y2004::darksteel as catalog_dst;
use crate::card::sets::y2005::ravnica_city_of_guilds as catalog_rav;
use crate::card::sets::y2005::saviors_of_kamigawa as catalog_sok;
use crate::card::sets::y2006::coldsnap as catalog_csp;
use crate::card::sets::y2006::dissension as catalog_dis;
use crate::card::sets::y2006::guildpact as catalog_gpt;
use crate::card::sets::y2006::time_spiral as catalog_tsp;
use crate::card::sets::y2007::future_sight as catalog_fut;
use crate::card::sets::y2007::lorwyn as catalog_lrw;
use crate::card::sets::y2008::morningtide as catalog_mor;
use crate::card::sets::y2008::shadowmoor as catalog_shm;
use crate::card::sets::y2008::shards_of_alara as catalog_ala;
use crate::card::sets::y2009::alara_reborn as catalog_arb;
use crate::card::sets::y2009::conflux as catalog_con;
use crate::card::sets::y2009::magic_2010 as catalog_m10;
use crate::card::sets::y2009::zendikar as catalog_zen;
use crate::card::sets::y2010::archenemy as catalog_arc;
use crate::card::sets::y2010::magic_2011 as catalog_m11;
use crate::card::sets::y2010::rise_of_the_eldrazi as catalog_roe;
use crate::card::sets::y2010::scars_of_mirrodin as catalog_som;
use crate::card::sets::y2010::worldwake as catalog_wwk;
use crate::card::sets::y2011::commander_2011 as catalog_cmd;
use crate::card::sets::y2011::innistrad as catalog_isd;
use crate::card::sets::y2011::magic_2012 as catalog_m12;
use crate::card::sets::y2011::mirrodin_besieged as catalog_mbs;
use crate::card::sets::y2012::avacyn_restored as catalog_avr;
use crate::card::sets::y2012::dark_ascension as catalog_dka;
use crate::card::sets::y2012::magic_2013 as catalog_m13;
use crate::card::sets::y2012::return_to_ravnica as catalog_rtr;
use crate::card::sets::y2013::commander_2013 as catalog_c13;
use crate::card::sets::y2013::dragons_maze as catalog_dgm;
use crate::card::sets::y2013::gatecrash as catalog_gtc;
use crate::card::sets::y2013::magic_2014 as catalog_m14;
use crate::card::sets::y2013::theros as catalog_ths;
use crate::card::sets::y2014::born_of_the_gods as catalog_bng;
use crate::card::sets::y2014::commander_2014 as catalog_c14;
use crate::card::sets::y2014::journey_into_nyx as catalog_jou;
use crate::card::sets::y2014::khans_of_tarkir as catalog_ktk;
use crate::card::sets::y2014::magic_2015 as catalog_m15;
use crate::card::sets::y2015::battle_for_zendikar as catalog_bfz;
use crate::card::sets::y2015::commander_2015 as catalog_c15;
use crate::card::sets::y2015::dragons_of_tarkir as catalog_dtk;
use crate::card::sets::y2015::fate_reforged as catalog_frf;
use crate::card::sets::y2015::magic_origins as catalog_ori;
use crate::card::sets::y2016::eldritch_moon as catalog_emn;
use crate::card::sets::y2016::kaladesh as catalog_kld;
use crate::card::sets::y2016::oath_of_the_gatewatch as catalog_ogw;
use crate::card::sets::y2016::shadows_over_innistrad as catalog_soi;
use crate::card::sets::y2017::aether_revolt as catalog_aer;
use crate::card::sets::y2017::amonkhet as catalog_akh;
use crate::card::sets::y2017::commander_2017 as catalog_c17;
use crate::card::sets::y2017::hour_of_devastation as catalog_hou;
use crate::card::sets::y2017::ixalan as catalog_xln;
use crate::card::sets::y2018::core_set_2019 as catalog_m19;
use crate::card::sets::y2018::dominaria as catalog_dom;
use crate::card::sets::y2018::global_series_jiang_yanggu_and_mu_yanling as catalog_gs1;
use crate::card::sets::y2018::guilds_of_ravnica as catalog_grn;
use crate::card::sets::y2018::rivals_of_ixalan as catalog_rix;
use crate::card::sets::y2019::magic_2020 as catalog_m20;
use crate::card::sets::y2019::modern_horizons as catalog_mh1;
use crate::card::sets::y2019::ravnica_allegiance as catalog_rna;
use crate::card::sets::y2019::throne_of_eldraine as catalog_eld;
use crate::card::sets::y2019::war_of_the_spark as catalog_war;
use crate::card::sets::y2020::core_set_2021 as catalog_m21;
use crate::card::sets::y2020::ikoria as catalog_iko;
use crate::card::sets::y2020::jumpstart as catalog_jmp;
use crate::card::sets::y2020::theros_beyond_death as catalog_thb;
use crate::card::sets::y2020::zendikar_rising as catalog_znr;
use crate::card::sets::y2021::adventures_in_the_forgotten_realms as catalog_afr;
use crate::card::sets::y2021::innistrad_crimson_vow as catalog_vow;
use crate::card::sets::y2021::innistrad_crimson_vow_commander as catalog_voc;
use crate::card::sets::y2021::innistrad_midnight_hunt as catalog_mid;
use crate::card::sets::y2021::kaldheim as catalog_khm;
use crate::card::sets::y2021::kaldheim_commander as catalog_khc;
use crate::card::sets::y2021::modern_horizons_2 as catalog_mh2;
use crate::card::sets::y2021::strixhaven_school_of_mages as catalog_stx;
use crate::card::sets::y2022::commander_legends_baldurs_gate as catalog_clb;
use crate::card::sets::y2022::dominaria_united as catalog_dmu;
use crate::card::sets::y2022::jumpstart_2022 as catalog_j22;
use crate::card::sets::y2022::kamigawa_neon_dynasty as catalog_neo;
use crate::card::sets::y2022::streets_of_new_capenna as catalog_snc;
use crate::card::sets::y2022::streets_of_new_capenna_commander as catalog_ncc;
use crate::card::sets::y2022::the_brothers_war as catalog_bro;
use crate::card::sets::y2023::lost_caverns_of_ixalan as catalog_lci;
use crate::card::sets::y2023::march_of_the_machine as catalog_mom;
use crate::card::sets::y2023::phyrexia_all_will_be_one as catalog_one;
use crate::card::sets::y2023::wilds_of_eldraine as catalog_woe;
use crate::card::sets::y2024::bloomburrow as catalog_blb;
use crate::card::sets::y2024::foundations_jumpstart as catalog_j25;
use crate::card::sets::y2024::murders_at_karlov_manor as catalog_mkm;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "FDN",
    slug: "magic-foundations",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// FDN 1 — Sire of Seven Deaths
pub(in crate::card::sets) static SIRE_OF_SEVEN_DEATHS: CardRecord = CardRecord::new(
    "Sire of Seven Deaths",
    "8d8432a7-1c8a-4cfb-947c-ecf9791063eb",
    "Lius Lasahido",
    CardRules::new_creature(mana_cost!("{7}"), &["Eldrazi"], 7, 7).with_abilities(&[
        abilities::reach(),
        abilities::first_strike(),
        abilities::vigilance(),
        abilities::menace(),
        abilities::trample(),
        abilities::lifelink(),
        abilities::ward(&[CostDef::PayLife(7)], "Ward—Pay 7 life."),
    ]),
);

// FDN 2 — Arahbo, the First Fang
pub(in crate::card::sets) static ARAHBO_THE_FIRST_FANG: CardRecord = CardRecord::new(
    "Arahbo, the First Fang",
    "524a5d93-26ed-436d-a437-dc9460acce98",
    "Simon Dominic",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Cat", "Avatar"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Other Cats you control get +1/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Cat")),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                },
            ),
            AbilityDef::triggered(
                "Whenever Arahbo or another nontoken Cat you control enters, \
                 create a 1/1 white Cat creature token.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Cat")),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                        ]),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::create_creature_token(&["Cat"], &[ManaColor::White], 1, 1),
            ),
        ]),
);

// FDN 3 — Armasaur Guide
pub(in crate::card::sets) static ARMASAUR_GUIDE: CardRecord = CardRecord::new(
    "Armasaur Guide",
    "c80fc380-0499-4499-8a60-c43844c02c9b",
    "Loïc Canavaggia",
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Dinosaur"], 4, 4).with_abilities(&[
        abilities::vigilance(),
        AbilityDef::triggered_with_targets(
            "Whenever you attack with three or more creatures, put a +1/+1 \
             counter on target creature you control.",
            TriggerEventDef::attack_declared(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                3,
                None,
            ),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// FDN 4 — Cat Collector
// Audit: unsupported — Needs a life-gain event ordinal for the first gain in a turn; a per-source trigger limit incorrectly triggers after an earlier gain that occurred before this creature entered.
pub(in crate::card::sets) static CAT_COLLECTOR: CardRecord = CardRecord::new(
    "Cat Collector",
    "526fe356-bff1-4211-9e88-bf913ac76b1d",
    "Chris Seaman",
    CardRules::unsupported(),
);

// FDN 5 — Celestial Armor
pub(in crate::card::sets) static CELESTIAL_ARMOR: CardRecord = CardRecord::new(
    "Celestial Armor",
    "809ee8ad-1573-49e5-9e84-b7cdd29efcae",
    "Olena Richards",
    CardRules::new_artifact(mana_cost!("{2}{W}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::flash(),
            abilities::enters_trigger_with_targets(
                "When this Equipment enters, attach it to target creature you \
                 control. That creature gains hexproof and indestructible \
                 until end of turn.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::Sequence(&[
                    EffectDef::Attach {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::add_ability(&abilities::hexproof()),
                            AppliedEffectDef::add_ability(&abilities::indestructible()),
                        ]),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            ),
            AbilityDef::static_ability(
                "Equipped creature gets +2/+0 and has flying.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(0),
                        ),
                        AppliedEffectDef::add_ability(&abilities::flying()),
                    ]),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{3}{W}"))], "Equip {3}{W}"),
        ]),
);

// FDN 6 — Claws Out
pub(in crate::card::sets) static CLAWS_OUT: CardRecord = CardRecord::new(
    "Claws Out",
    "4396049c-b976-4b7f-8ecd-564e24ebd631",
    "Warren Mahy",
    CardRules::new_instant(mana_cost!("{3}{W}{W}")).with_abilities(&[
        AbilityDef::static_ability(
            "Affinity for Cats (This spell costs {1} less to cast for each \
             Cat you control.)",
            EffectDef::ReduceGenericCostBy(ValueDef::CountMatchingObjects(
                &ObjectQueryDef::matching(
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Cat")),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
            )),
        )
        .with_source_zones(&[ZoneKind::Hand]),
        AbilityDef::spell(
            "Creatures you control get +2/+2 until end of turn.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// FDN 7 — Crystal Barricade
// Audit: unsupported — Needs a noncombat-only damage matcher for its prevention replacement; current damage-kind matching exposes Any and Combat, but not Noncombat.
pub(in crate::card::sets) static CRYSTAL_BARRICADE: CardRecord = CardRecord::new(
    "Crystal Barricade",
    "905d3e02-ea06-45e7-9adb-c8e7583323a2",
    "Rockey Chen",
    CardRules::unsupported(),
);

// FDN 8 — Dauntless Veteran
pub(in crate::card::sets) static DAUNTLESS_VETERAN: CardRecord = CardRecord::new(
    "Dauntless Veteran",
    "7a136f26-ac66-407f-b389-357222d2c4a2",
    "Chris Rallis",
    CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Human", "Soldier"], 2, 2).with_abilities(
        &[AbilityDef::triggered(
            "Whenever this creature attacks, creatures you control get \
             +1/+1 until end of turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
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
        )],
    ),
);

// FDN 9 — Dazzling Angel
pub(in crate::card::sets) static DAZZLING_ANGEL: CardRecord = CardRecord::new(
    "Dazzling Angel",
    "027dc444-e544-4693-8653-3dcdda530162",
    "Daneen Wilkerson",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Angel"], 2, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever another creature you control enters, you gain 1 life.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// FDN 10 — Divine Resilience
// Audit: unsupported — Needs target-count bounds that change with a kicker payment: exactly one when unkicked and zero or more when kicked; current slots have fixed minimum/maximum bounds or one exact computed count.
pub(in crate::card::sets) static DIVINE_RESILIENCE: CardRecord = CardRecord::new(
    "Divine Resilience",
    "f3a08245-a535-4d24-b8c0-78759bb9c4b0",
    "Svetlin Velinov",
    CardRules::unsupported(),
);

// FDN 11 — Exemplar of Light
// Audit: unsupported — Needs counter-placement events to identify the player instructed to place the counters; current CountersPlaced matches the object and counter kind but not the placing player.
pub(in crate::card::sets) static EXEMPLAR_OF_LIGHT: CardRecord = CardRecord::new(
    "Exemplar of Light",
    "920c8fc5-fdd2-446a-a676-5c363f96928f",
    "Ekaterina Burmak",
    CardRules::unsupported(),
);

// FDN 12 — Felidar Savior
pub(in crate::card::sets) static FELIDAR_SAVIOR: CardRecord = CardRecord::new(
    "Felidar Savior",
    "cd092b14-d72f-4de0-8f19-1338661b9e3b",
    "Ilse Gort",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Cat", "Beast"], 2, 3).with_abilities(&[
        abilities::lifelink(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, put a +1/+1 counter on each of up \
             to two other target creatures you control.",
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
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
    ]),
);

// FDN 13 — Fleeting Flight
pub(in crate::card::sets) static FLEETING_FLIGHT: CardRecord = CardRecord::new(
    "Fleeting Flight",
    "55139100-9342-41fd-b10a-8e9932e605d4",
    "Leonardo Santanna",
    CardRules::new_instant(mana_cost!("{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Put a +1/+1 counter on target creature. It gains flying until \
         end of turn. Prevent all combat damage that would be dealt to \
         it this turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::flying()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::PreventDamage {
                prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::combat_to(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )]),
);

// FDN 14 — Guarded Heir
pub(in crate::card::sets) static GUARDED_HEIR: CardRecord = CardRecord::new(
    "Guarded Heir",
    "525ba5c7-3ce5-4e52-b8b5-96c9040a6738",
    "Craig J Spearing",
    CardRules::new_creature(mana_cost!("{5}{W}"), &["Human", "Noble"], 1, 1).with_abilities(&[
        abilities::lifelink(),
        abilities::enters_trigger(
            "When this creature enters, create two 3/3 white Knight \
             creature tokens.",
            EffectDef::create_creature_token(&["Knight"], &[ManaColor::White], 3, 3)
                .with_count(ValueDef::Constant(2)),
        ),
    ]),
);

// FDN 15 — Hare Apparent
// Audit: unsupported — Needs declarative deck-construction metadata allowing an unlimited number of this named card; card rules currently have no per-card copy-limit exception.
pub(in crate::card::sets) static HARE_APPARENT: CardRecord = CardRecord::new(
    "Hare Apparent",
    "9fc6f0e9-eb5f-4bc0-b3d7-756644b66d12",
    "Milivoj Ćeran",
    CardRules::unsupported(),
);

// FDN 16 — Helpful Hunter
pub(in crate::card::sets) static HELPFUL_HUNTER: CardRecord = CardRecord::new(
    "Helpful Hunter",
    "1b9a0e91-80b5-428f-8f08-931d0631be14",
    "Xabi Gaztelua",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Cat"], 1, 1).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, draw a card.",
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
    ]),
);

// FDN 17 — Herald of Eternal Dawn
// Audit: unsupported — Needs player-scoped prohibitions on losing the game and opponents winning, applied to every game-ending condition and alternate win/loss effect.
pub(in crate::card::sets) static HERALD_OF_ETERNAL_DAWN: CardRecord = CardRecord::new(
    "Herald of Eternal Dawn",
    "c9fdfebf-98e0-4718-bac3-6eee1cd0623d",
    "Martina Fačková",
    CardRules::unsupported(),
);

// FDN 18 — Inspiring Paladin
pub(in crate::card::sets) static INSPIRING_PALADIN: CardRecord = CardRecord::new(
    "Inspiring Paladin",
    "0763be06-25b2-4d6b-ab33-a1af85aeb443",
    "Valera Lutfullina",
    // First strike only while attacking, which is the trade for handing it
    // out to the whole team: it never helps the blocks.
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Knight"], 3, 3).with_abilities(&[
        AbilityDef::static_ability(
            "During your turn, this creature has first strike. (It deals combat damage before \
             creatures without first strike.)",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::You),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                },
            },
        ),
        // A second printed ability rather than a rider: it reaches every
        // creature with a counter, and this one only if something has put a
        // counter on it.
        AbilityDef::static_ability(
            "During your turn, creatures you control with +1/+1 counters on them have first \
             strike.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::You),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasCounter(CounterKind::PlusOnePlusOne),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                },
            },
        ),
    ]),
);

// FDN 19 — Joust Through
pub(in crate::card::sets) static JOUST_THROUGH: CardRecord = CardRecord::new(
    "Joust Through",
    "846adb38-f9bb-4fed-b8ed-36ec7885f989",
    "Miro Petrov",
    CardRules::new_instant(mana_cost!("{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Joust Through deals 3 damage to target attacking or blocking \
         creature. You gain 1 life.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Attacking,
                    ObjectPredicateDef::Blocking,
                ]),
            ]),
        )],
        EffectDef::Sequence(&[
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(3),
            ),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )]),
);

// FDN 20 — Luminous Rebuke
// Audit: unsupported — Needs a self spell-cost reduction based on whether the chosen target is tapped; self-cost evaluation does not read selected spell targets.
pub(in crate::card::sets) static LUMINOUS_REBUKE: CardRecord = CardRecord::new(
    "Luminous Rebuke",
    "621839e1-2756-4cdc-a25c-5f76ea98dd87",
    "Mike Sass",
    CardRules::unsupported(),
);

// FDN 21 — Prideful Parent
pub(in crate::card::sets) static PRIDEFUL_PARENT: CardRecord = CardRecord::new(
    "Prideful Parent",
    "b742117a-8a72-43b9-b05d-274829d138a2",
    "Leonardo Santanna",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Cat"], 2, 2).with_abilities(&[
        abilities::vigilance(),
        abilities::enters_trigger(
            "When this creature enters, create a 1/1 white Cat creature token.",
            EffectDef::create_creature_token(&["Cat"], &[ManaColor::White], 1, 1),
        ),
    ]),
);

// FDN 22 — Raise the Past
pub(in crate::card::sets) static RAISE_THE_PAST: CardRecord = CardRecord::new(
    "Raise the Past",
    "6c6be129-56da-4fe7-a6bd-6a1d402c09e1",
    "Nathaniel Himawan",
    CardRules::new_sorcery(mana_cost!("{2}{W}{W}")).with_abilities(&[AbilityDef::spell(
        "Return all creature cards with mana value 2 or less from your \
         graveyard to the battlefield.",
        EffectDef::move_to_zone(
            EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ManaValueAtMost(2),
                ]),
                &[ZoneKind::Graveyard],
                PlayerRelation::You,
            ))),
            ZoneKind::Battlefield,
            ZonePlacement::Top,
        ),
    )]),
);

// FDN 23 — Skyknight Squire
pub(in crate::card::sets) static SKYKNIGHT_SQUIRE: CardRecord = CardRecord::new(
    "Skyknight Squire",
    "fcfe4e62-c153-47b8-8e09-cedaf91f53d8",
    "Alexander Mokhov",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Cat", "Scout"], 1, 1).with_abilities(&[
        AbilityDef::triggered(
            "Whenever another creature you control enters, put a +1/+1 \
             counter on this creature.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
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
        AbilityDef::static_ability(
            "As long as this creature has three or more +1/+1 counters on \
             it, it has flying and is a Knight in addition to its other \
             types.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 3,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_ability(&abilities::flying()),
                        AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&[
                            "Knight",
                        ])),
                    ]),
                },
            },
        ),
    ]),
);

// FDN 24 — Squad Rallier
pub(in crate::card::sets) static SQUAD_RALLIER: CardRecord = CardRecord::new(
    "Squad Rallier",
    "65e1ee86-6f08-4aa0-bf63-ae12028ef080",
    "Edgar Sánchez Hidalgo",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Scout"], 3, 4).with_abilities(&[
        AbilityDef::activated(
            "{2}{W}: Look at the top four cards of your library. You may \
             reveal a creature card with power 2 or less from among them \
             and put it into your hand. Put the rest on the bottom of your \
             library in a random order.",
            &[CostDef::Mana(mana_cost!("{2}{W}"))],
            EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                source: ObjectCollectionSourceDef::TopCards {
                    player: PlayerRefDef::EffectController,
                    count: ValueDef::Constant(4),
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

// FDN 25 — Sun-Blessed Healer
pub(in crate::card::sets) static SUN_BLESSED_HEALER: CardRecord = CardRecord::new(
    "Sun-Blessed Healer",
    "323d029e-9a88-4188-b3a4-38ef32cffc9f",
    "Mark Zug",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Cleric"], 3, 1).with_abilities(&[
        abilities::kicker(&[CostDef::Mana(mana_cost!("{1}{W}"))]),
        abilities::lifelink(),
        AbilityDef::triggered_if_with_targets(
            "When this creature enters, if it was kicked, return target \
             nonland permanent card with mana value 2 or less from your \
             graveyard to the battlefield.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &TriggerConditionDef::SourcePaidAdditionalCost(crate::AdditionalCostIndex::PRIMARY),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::HasType(CardType::Planeswalker),
                        ]),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                        ObjectPredicateDef::ManaValueAtMost(2),
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
    ]),
);

// FDN 26 — Twinblade Blessing
pub(in crate::card::sets) static TWINBLADE_BLESSING: CardRecord = CardRecord::new(
    "Twinblade Blessing",
    "ecf01cbe-9fcb-4f35-bc6b-2280620b06ff",
    "Miro Petrov",
    CardRules::new_enchantment(mana_cost!("{1}{W}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::flash(),
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature has double strike. (It deals both \
                 first-strike and regular combat damage.)",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::double_strike()),
                },
            ),
        ]),
);

// FDN 27 — Valkyrie's Call
// Audit: unsupported — Needs additional Angel type and flying established simultaneously with the return to the battlefield (CR 611.2e), before entry replacements and trigger matching.
pub(in crate::card::sets) static VALKYRIE_S_CALL: CardRecord = CardRecord::new(
    "Valkyrie's Call",
    "0e1f1ff2-fa8f-4d38-b631-2d6e08e614c8",
    "Scott Murphy",
    CardRules::unsupported(),
);

// FDN 28 — Vanguard Seraph
// Audit: unsupported — Needs the first life-gain event of the turn to be identifiable independently of when this creature entered; a once-per-turn trigger limit only counts its own triggers.
pub(in crate::card::sets) static VANGUARD_SERAPH: CardRecord = CardRecord::new(
    "Vanguard Seraph",
    "4329c861-fc16-4a96-9c03-25af6ac2adc8",
    "Zezhou Chen",
    CardRules::unsupported(),
);

// FDN 29 — Arcane Epiphany
pub(in crate::card::sets) static ARCANE_EPIPHANY: CardRecord = CardRecord::new(
    "Arcane Epiphany",
    "06431793-5dfe-4cbf-990b-4bcc960d1f31",
    "Andrew Mar",
    CardRules::new_instant(mana_cost!("{3}{U}{U}")).with_abilities(&[
        AbilityDef::static_ability(
            "This spell costs {1} less to cast if you control a Wizard.",
            EffectDef::ReduceGenericCostBy(ValueDef::IfMatchingObjectCount(&CountConditionDef {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Wizard")),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
                then: ValueDef::Constant(1),
                otherwise: ValueDef::Constant(0),
            })),
        )
        .with_source_zones(&[ZoneKind::Hand]),
        AbilityDef::spell(
            "Draw three cards.",
            abilities::draw_cards(ValueDef::Constant(3)),
        ),
    ]),
);

// FDN 30 — Archmage of Runes
pub(in crate::card::sets) static ARCHMAGE_OF_RUNES: CardRecord = CardRecord::new(
    "Archmage of Runes",
    "334b5018-2da9-49f1-9d09-83d312ecfb02",
    "Kai Carpenter",
    CardRules::new_creature(mana_cost!("{3}{U}{U}"), &["Giant", "Wizard"], 3, 6).with_abilities(&[
        abilities::spell_cost_reduction(
            "Instant and sorcery spells you cast cost {1} less to cast.",
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Instant),
                ObjectPredicateDef::HasType(CardType::Sorcery),
            ]),
            PlayerRelation::You,
            ValueDef::Constant(1),
        ),
        AbilityDef::triggered(
            "Whenever you cast an instant or sorcery spell, draw a card.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::HasType(CardType::Sorcery),
                ]),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
    ]),
);

// FDN 31 — Bigfin Bouncer
pub(in crate::card::sets) static BIGFIN_BOUNCER: CardRecord = CardRecord::new(
    "Bigfin Bouncer",
    "9b1d5b76-b07e-45c6-800d-4cfce085164f",
    "Brent Hollowell",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Shark", "Pirate"], 3, 2).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this creature enters, return target creature an opponent \
             controls to its owner's hand.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
        ),
    ]),
);

// FDN 32 — Cephalid Inkmage
pub(in crate::card::sets) static CEPHALID_INKMAGE: CardRecord = CardRecord::new(
    "Cephalid Inkmage",
    "b7e47680-18c7-4ffb-aac4-c5db6e7095ba",
    "Christopher Burdett",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Octopus", "Wizard"], 2, 2).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, surveil 3. (Look at the top three \
             cards of your library, then put any number of them into your \
             graveyard and the rest on top of your library in any order.)",
            abilities::surveil(ValueDef::Constant(3)),
        ),
        AbilityDef::static_ability(
            "Threshold — This creature can't be blocked as long as there \
             are seven or more cards in your graveyard.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::Any,
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 7,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                },
            },
        ),
    ]),
);

// FDN 33 — Clinquant Skymage
pub(in crate::card::sets) static CLINQUANT_SKYMAGE: CardRecord = CardRecord::new(
    "Clinquant Skymage",
    "36012810-0e83-4640-8ba7-7262229f1b84",
    "Kevin Sidharta",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Bird", "Wizard"], 1, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever you draw a card, put a +1/+1 counter on this creature.",
            TriggerEventDef::DrewCard(DrawEventMatcherDef::any(PlayerRelation::You)),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// FDN 34 — Curator of Destinies
// Audit: unsupported — Needs a library partition choice with one face-down pile and one face-up pile, followed by an opponent choosing a pile without seeing the hidden pile.
pub(in crate::card::sets) static CURATOR_OF_DESTINIES: CardRecord = CardRecord::new(
    "Curator of Destinies",
    "9ff79da7-c3f7-4541-87a0-503544c699b5",
    "Ralph Horsley",
    CardRules::unsupported(),
);

// FDN 35 — Drake Hatcher
pub(in crate::card::sets) static DRAKE_HATCHER: CardRecord = CardRecord::new(
    "Drake Hatcher",
    "bcaf4196-6bf3-47fa-b5c7-0e77f45cf820",
    "Chris Rallis",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Human", "Wizard"], 1, 3).with_abilities(&[
        abilities::vigilance(),
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
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, put \
             that many incubation counters on it.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::named("incubation"),
                amount: ValueDef::TriggerEventAmount,
            },
        ),
        AbilityDef::activated(
            "Remove three incubation counters from this creature: Create a \
             2/2 blue Drake creature token with flying.",
            &[CostDef::RemoveCountersFromSource {
                kind: CounterKind::named("incubation"),
                amount: 3,
            }],
            EffectDef::create_creature_token(&["Drake"], &[ManaColor::Blue], 2, 2)
                .with_abilities(&[abilities::flying()]),
        ),
    ]),
);

// FDN 36 — Elementalist Adept
pub(in crate::card::sets) static ELEMENTALIST_ADEPT: CardRecord = CardRecord::new(
    "Elementalist Adept",
    "d9768cc6-8f53-4922-ae32-376a2f32d719",
    "L.A. Draws",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Human", "Wizard"], 2, 1).with_abilities(&[
        abilities::flash(),
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
    ]),
);

// FDN 37 — Erudite Wizard
pub(in crate::card::sets) static ERUDITE_WIZARD: CardRecord = CardRecord::new(
    "Erudite Wizard",
    "9273c417-0fcd-4273-b24e-afff76336d0c",
    "Ioannis Fiore",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Wizard"], 2, 3).with_abilities(&[
        AbilityDef::triggered(
            "Whenever you draw your second card each turn, put a +1/+1 \
             counter on this creature.",
            TriggerEventDef::DrewCard(DrawEventMatcherDef::nth_each_turn(PlayerRelation::You, 2)),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// FDN 38 — Faebloom Trick
// Audit: unsupported — Needs a reflexive trigger after token creation with targets chosen after the tokens enter; an ordinary targeted spell clause chooses those targets too early.
pub(in crate::card::sets) static FAEBLOOM_TRICK: CardRecord = CardRecord::new(
    "Faebloom Trick",
    "0c3bee8f-f5be-4404-a696-c902637799c3",
    "Annie Stegg",
    CardRules::unsupported(),
);

// FDN 39 — Grappling Kraken
pub(in crate::card::sets) static GRAPPLING_KRAKEN: CardRecord = CardRecord::new(
    "Grappling Kraken",
    "d1f5cab3-3fc0-448d-8252-cd55abf5b596",
    "Ben Wootten",
    CardRules::new_creature(mana_cost!("{4}{U}{U}"), &["Kraken"], 5, 6).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "Landfall — Whenever a land you control enters, tap target \
             creature an opponent controls and put a stun counter on it. \
             (If a permanent with a stun counter would become untapped, \
             remove one from it instead.)",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
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
    ]),
);

// FDN 40 — High Fae Trickster
pub(in crate::card::sets) static HIGH_FAE_TRICKSTER: CardRecord = CardRecord::new(
    "High Fae Trickster",
    "7f1b93ea-1ec1-4010-9343-765742f5088b",
    "Justyna Dura",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Faerie", "Wizard"], 4, 2).with_abilities(&[
        abilities::flash(),
        abilities::flying(),
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

// FDN 41 — Homunculus Horde
pub(in crate::card::sets) static HOMUNCULUS_HORDE: CardRecord = CardRecord::new(
    "Homunculus Horde",
    "470c4d03-340a-4e0e-a59f-f19d05497785",
    "Adrián Rodríguez Pérez",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Homunculus"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever you draw your second card each turn, create a token \
             that's a copy of this creature.",
            TriggerEventDef::DrewCard(DrawEventMatcherDef::nth_each_turn(PlayerRelation::You, 2)),
            EffectDef::create_token_from_copy(&TokenCopyDef {
                object: &EffectRecipientDef::Source,
                exceptions: CopyExceptionsDef::NONE,
            }),
        ),
    ]),
);

// FDN 42 — Icewind Elemental
pub(in crate::card::sets) static ICEWIND_ELEMENTAL: CardRecord = CardRecord::new(
    "Icewind Elemental",
    "fd0eba76-3829-408b-828f-0b223c884728",
    "Andrew Mar",
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Elemental"], 3, 4).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, draw a card, then discard a card.",
            EffectDef::Sequence(&[
                abilities::draw_cards(ValueDef::Constant(1)),
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

// FDN 43 — Inspiration from Beyond
pub(in crate::card::sets) static INSPIRATION_FROM_BEYOND: CardRecord = CardRecord::new(
    "Inspiration from Beyond",
    "b636fe95-664f-4fb1-aab9-28856edeccd6",
    "Xavier Ribeiro",
    CardRules::new_sorcery(mana_cost!("{2}{U}")).with_abilities(&[
        AbilityDef::spell(
            "Mill three cards, then return an instant or sorcery card from \
             your graveyard to your hand.",
            EffectDef::Sequence(&[
                EffectDef::Mill {
                    player: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(3),
                },
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Instant),
                            ObjectPredicateDef::HasType(CardType::Sorcery),
                        ]),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    )),
                    exclude: None,
                    minimum: 1,
                    maximum: 1,
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
        abilities::flashback(&[CostDef::Mana(mana_cost!("{5}{U}{U}"))]),
    ]),
);

// FDN 44 — Kaito, Cunning Infiltrator
pub(in crate::card::sets) static KAITO_CUNNING_INFILTRATOR: CardRecord = CardRecord::new(
    "Kaito, Cunning Infiltrator",
    "5dabdea9-2015-49b9-853d-4f7e1262eab3",
    "Evyn Fong",
    CardRules::new_planeswalker(mana_cost!("{1}{U}{U}"), &["Kaito"], 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever a creature you control deals combat damage to a \
                 player, put a loyalty counter on Kaito.",
                TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::Loyalty,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::activated_with_targets(
                "+1: Up to one target creature you control can't be blocked \
                 this turn. Draw a card, then discard a card.",
                &[CostDef::Loyalty(1)],
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                    1,
                )],
                EffectDef::Sequence(&[
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                    abilities::draw_cards(ValueDef::Constant(1)),
                    EffectDef::Discard {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                        selection: DiscardSelectionDef::RecipientChooses,
                        then: None,
                    },
                ]),
            ),
            AbilityDef::activated(
                "−2: Create a 2/1 blue Ninja creature token.",
                &[CostDef::Loyalty(-2)],
                EffectDef::create_creature_token(&["Ninja"], &[ManaColor::Blue], 2, 1),
            ),
            AbilityDef::activated(
                "−9: You get an emblem with \"Whenever a player casts a spell, \
                 you create a 2/1 blue Ninja creature token.\"",
                &[CostDef::Loyalty(-9)],
                EffectDef::CreateEmblem {
                    emblem: EmblemCharacteristics::new(
                        "Kaito Emblem",
                        &[AbilityDef::triggered(
                            "Whenever a player casts a spell, you create a 2/1 blue Ninja \
                             creature token.",
                            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                                ObjectPredicateDef::Any,
                                ObjectPredicateDef::ControlledBy(PlayerRelation::Any),
                            ])),
                            EffectDef::create_creature_token(&["Ninja"], &[ManaColor::Blue], 2, 1),
                        )],
                    ),
                },
            ),
        ]),
);

// FDN 45 — Kiora, the Rising Tide
pub(in crate::card::sets) static KIORA_THE_RISING_TIDE: CardRecord = CardRecord::new(
    "Kiora, the Rising Tide",
    "83f20a32-9f5d-4a68-8995-549e57554da2",
    "Julian Kok Joon Wen",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Merfolk", "Noble"], 3, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::enters_trigger(
                "When Kiora enters, draw two cards, then discard two cards.",
                EffectDef::Sequence(&[
                    abilities::draw_cards(ValueDef::Constant(2)),
                    EffectDef::Discard {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(2),
                        selection: DiscardSelectionDef::RecipientChooses,
                        then: None,
                    },
                ]),
            ),
            AbilityDef::triggered_if(
                "Threshold — Whenever Kiora attacks, if there are seven or \
                 more cards in your graveyard, you may create Scion of the \
                 Deep, a legendary 8/8 blue Octopus creature token.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::Any,
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 7,
                },
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    effect: &EffectDef::create_token(
                        TokenCharacteristics::creature(&["Octopus"], &[ManaColor::Blue], 8, 8)
                            .with_name("Scion of the Deep")
                            .with_supertype(CardSupertype::Legendary),
                    ),
                },
            ),
        ]),
);

// FDN 46 — Lunar Insight
// Audit: unsupported — Needs an aggregate counting distinct mana values among a changing set of permanents; current object aggregates count objects or combine scalar values without distinctness.
pub(in crate::card::sets) static LUNAR_INSIGHT: CardRecord = CardRecord::new(
    "Lunar Insight",
    "a9a159f6-fecf-4bdd-b2f8-a9665a5cc32d",
    "Dan Murayama Scott",
    CardRules::unsupported(),
);

// FDN 47 — Mischievous Mystic
pub(in crate::card::sets) static MISCHIEVOUS_MYSTIC: CardRecord = CardRecord::new(
    "Mischievous Mystic",
    "20d89cec-528b-4b2a-87db-e11ce0000622",
    "Steve Prescott",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Human", "Wizard"], 2, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever you draw your second card each turn, create a 1/1 \
             blue Faerie creature token with flying.",
            TriggerEventDef::DrewCard(DrawEventMatcherDef::nth_each_turn(PlayerRelation::You, 2)),
            EffectDef::create_creature_token(&["Faerie"], &[ManaColor::Blue], 1, 1)
                .with_abilities(&[abilities::flying()]),
        ),
    ]),
);

// FDN 48 — Refute
pub(in crate::card::sets) static REFUTE: CardRecord = CardRecord::new(
    "Refute",
    "38806934-dd9c-4ad4-a59c-a16dce03a14a",
    "Ignatius Budi",
    CardRules::new_instant(mana_cost!("{1}{U}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Counter target spell. Draw a card, then discard a card.",
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
                abilities::draw_cards(ValueDef::Constant(1)),
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

// FDN 49 — Rune-Sealed Wall
pub(in crate::card::sets) static RUNE_SEALED_WALL: CardRecord = CardRecord::new(
    "Rune-Sealed Wall",
    "da0f147b-95ed-4f32-9b46-6a633ae31976",
    "Rockey Chen",
    CardRules::new_artifact_creature(mana_cost!("{2}{U}"), &["Wall"], 0, 6).with_abilities(&[
        abilities::defender(),
        AbilityDef::activated(
            "{T}: Surveil 1. (Look at the top card of your library. You \
             may put it into your graveyard.)",
            &[CostDef::TapSource],
            abilities::surveil(ValueDef::Constant(1)),
        ),
    ]),
);

// FDN 50 — Skyship Buccaneer
// Audit: unsupported — Needs per-player attack history for the current turn, retained after attacking creatures leave or change controllers; current object predicates only identify individual permanents that attacked.
pub(in crate::card::sets) static SKYSHIP_BUCCANEER: CardRecord = CardRecord::new(
    "Skyship Buccaneer",
    "62958fc3-55dc-4b97-a070-490d6ed27820",
    "Javier Charro",
    CardRules::unsupported(),
);

// FDN 51 — Sphinx of Forgotten Lore
pub(in crate::card::sets) static SPHINX_OF_FORGOTTEN_LORE: CardRecord = CardRecord::new(
    "Sphinx of Forgotten Lore",
    "af6e46b8-62ed-4bca-ba38-a821f225b59f",
    "Dmitry Burmak",
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Sphinx"], 3, 3).with_abilities(&[
        abilities::flash(),
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks, target instant or sorcery \
             card in your graveyard gains flashback until end of turn. The \
             flashback cost is equal to that card's mana cost. (You may \
             cast that card from your graveyard for its flashback cost. \
             Then exile it.)",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                    ]),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::flashback_for_card_mana_cost()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// FDN 52 — Strix Lookout
pub(in crate::card::sets) static STRIX_LOOKOUT: CardRecord = CardRecord::new(
    "Strix Lookout",
    "fbd2422e-8e84-4c39-af29-3b4d38baee63",
    "Josiah \"Jo\" Cameron",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Bird"], 1, 2).with_abilities(&[
        abilities::flying(),
        abilities::vigilance(),
        AbilityDef::activated(
            "{1}{U}, {T}: Draw a card, then discard a card.",
            &[CostDef::Mana(mana_cost!("{1}{U}")), CostDef::TapSource],
            EffectDef::Sequence(&[
                abilities::draw_cards(ValueDef::Constant(1)),
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

// FDN 53 — Uncharted Voyage
pub(in crate::card::sets) static UNCHARTED_VOYAGE: CardRecord = CardRecord::new(
    "Uncharted Voyage",
    "e0846820-e595-4743-8a28-29c57d728677",
    "Julian Kok Joon Wen",
    CardRules::new_instant(mana_cost!("{3}{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature's owner puts it on their choice of the top or \
         bottom of their library.\nSurveil 1. (Look at the top card of \
         your library. You may put it into your graveyard.)",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
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

// FDN 54 — Abyssal Harvester
// Audit: unsupported — Needs a graveyard-entry timestamp predicate for cards put there this turn; EnteredThisTurn only describes battlefield permanents.
pub(in crate::card::sets) static ABYSSAL_HARVESTER: CardRecord = CardRecord::new(
    "Abyssal Harvester",
    "f2e0f538-5825-47e9-883c-3ec6fd5b25ea",
    "Diana Franco",
    CardRules::unsupported(),
);

// FDN 55 — Arbiter of Woe
pub(in crate::card::sets) static ARBITER_OF_WOE: CardRecord = CardRecord::new(
    "Arbiter of Woe",
    "b2496c4a-df03-4583-bd76-f98ed5cb61ee",
    "Jim Pavelec",
    CardRules::new_creature(mana_cost!("{4}{B}{B}"), &["Demon"], 5, 4).with_abilities(&[
        AbilityDef::spell(
            "As an additional cost to cast this spell, sacrifice a creature.",
            EffectDef::None,
        )
        .with_spell_additional_cost(&CostDef::Sacrifice {
            object: ObjectPredicateDef::HasType(CardType::Creature),
            quantity: CostQuantityDef::Fixed(1),
        }),
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, each opponent discards a card and \
             loses 2 life. You draw a card and gain 2 life.",
            EffectDef::Sequence(&[
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Opponent,
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Opponent,
                    amount: ValueDef::Constant(2),
                },
                abilities::draw_cards(ValueDef::Constant(1)),
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
            ]),
        ),
    ]),
);

// FDN 56 — Billowing Shriekmass
pub(in crate::card::sets) static BILLOWING_SHRIEKMASS: CardRecord = CardRecord::new(
    "Billowing Shriekmass",
    "7b3587a9-0667-4d53-807b-c437bcb1d7b3",
    "Brent Hollowell",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Spirit"], 2, 3).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, mill three cards. (Put the top \
             three cards of your library into your graveyard.)",
            EffectDef::Mill {
                player: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ),
        AbilityDef::static_ability(
            "Threshold — This creature gets +2/+1 as long as there are \
             seven or more cards in your graveyard.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::Any,
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 7,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(1),
                    ),
                },
            },
        ),
    ]),
);

// FDN 57 — Blasphemous Edict
pub(in crate::card::sets) static BLASPHEMOUS_EDICT: CardRecord = CardRecord::new(
    "Blasphemous Edict",
    "11040ecd-3153-4029-b42b-1441bc51ec34",
    "Andrew Mar",
    CardRules::new_sorcery(mana_cost!("{3}{B}{B}")).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{B}"))],
            AlternativeCastKindDef::AlternativeCost,
            Some(
                "You may pay {B} rather than pay this spell's mana cost if \
                 there are thirteen or more creatures on the battlefield.",
            ),
            EffectDef::None,
        )
        .with_alternative_condition(&TriggerConditionDef::ObjectCount {
            query: ObjectQueryDef::matching(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            comparison: ComparisonDef::GreaterOrEqual,
            amount: 13,
        }),
        AbilityDef::spell(
            "Each player sacrifices thirteen creatures of their choice.",
            EffectDef::ChooseForEachPlayer(ChooseForEachPlayerDef {
                player: EffectRecipientDef::EachPlayer,
                zone: ZoneKind::Battlefield,
                candidates: ObjectPredicateDef::HasType(CardType::Creature),
                selection: PerPlayerSelectionDef::Count(ValueDef::Constant(13)),
                chosen: crate::Binding!("sacrifices"),
                unchosen: crate::Binding!("unchosen_sacrifices"),
                visibility: ChoiceVisibilityDef::Public,
                then: &EffectDef::sacrifice(EffectRecipientDef::objects(ObjectSetDef::Binding(
                    crate::Binding!("sacrifices"),
                ))),
            }),
        ),
    ]),
);

// FDN 58 — Bloodthirsty Conqueror
// Audit: unsupported — Needs a life-loss event carrying the amount actually lost, including payments and life-total changes; damage events cannot represent every life loss.
pub(in crate::card::sets) static BLOODTHIRSTY_CONQUEROR: CardRecord = CardRecord::new(
    "Bloodthirsty Conqueror",
    "ce860ed4-a5bd-4347-9eab-dd716ea84db1",
    "Dmitry Burmak",
    CardRules::unsupported(),
);

// FDN 59 — Crypt Feaster
pub(in crate::card::sets) static CRYPT_FEASTER: CardRecord = CardRecord::new(
    "Crypt Feaster",
    "3b072811-998a-4a71-b59c-6afecc0dc4b6",
    "John Di Giovanni",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Zombie"], 3, 4).with_abilities(&[
        abilities::menace(),
        AbilityDef::triggered_if(
            "Threshold — Whenever this creature attacks, if there are \
             seven or more cards in your graveyard, this creature gets \
             +2/+0 until end of turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Graveyard],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 7,
            },
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

// FDN 60 — Gutless Plunderer
// Audit: unsupported — Needs per-player attack history for the current turn, retained after attacking creatures leave or change controllers; current object predicates only identify individual permanents that attacked.
pub(in crate::card::sets) static GUTLESS_PLUNDERER: CardRecord = CardRecord::new(
    "Gutless Plunderer",
    "909d7778-c7f8-4fa4-89f2-8b32e86e96e4",
    "Loïc Canavaggia",
    CardRules::unsupported(),
);

// FDN 61 — High-Society Hunter
pub(in crate::card::sets) static HIGH_SOCIETY_HUNTER: CardRecord = CardRecord::new(
    "High-Society Hunter",
    "51da4a4b-ea12-4169-a7cf-eb4427f13e84",
    "Daneen Wilkerson",
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Vampire", "Noble"], 5, 3).with_abilities(
        &[
            abilities::flying(),
            AbilityDef::triggered(
                "Whenever this creature attacks, you may sacrifice another \
                 creature. If you do, put a +1/+1 counter on this creature.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                EffectDef::PayOr(PayOrDef::optional(
                    &[CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]))],
                    &EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                )),
            ),
            AbilityDef::triggered(
                "Whenever another nontoken creature dies, draw a card.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                    ]),
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                abilities::draw_cards(ValueDef::Constant(1)),
            ),
        ],
    ),
);

// FDN 62 — Hungry Ghoul
pub(in crate::card::sets) static HUNGRY_GHOUL: CardRecord = CardRecord::new(
    "Hungry Ghoul",
    "790f9433-7565-4f7f-88e8-8af762ea0296",
    "Paolo Parente",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Zombie"], 2, 2).with_abilities(&[
        AbilityDef::activated(
            "{1}, Sacrifice another creature: Put a +1/+1 counter on this \
             creature.",
            &[
                CostDef::Mana(mana_cost!("{1}")),
                CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ])),
            ],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// FDN 63 — Infernal Vessel
// Audit: unsupported — Needs the returning creature to have its additional Demon type as it enters; applying a type effect afterward gives entry replacements and triggers incorrect characteristics.
pub(in crate::card::sets) static INFERNAL_VESSEL: CardRecord = CardRecord::new(
    "Infernal Vessel",
    "877b6330-2d0b-4f2f-a848-f10b06fb4ef5",
    "Franz Vohwinkel",
    CardRules::unsupported(),
);

// FDN 64 — Infestation Sage
pub(in crate::card::sets) static INFESTATION_SAGE: CardRecord = CardRecord::new(
    "Infestation Sage",
    "d40c73de-7a5f-46f2-a70b-449bc8ecfe24",
    "Daneen Wilkerson",
    CardRules::new_creature(mana_cost!("{B}"), &["Elf", "Warlock"], 1, 1).with_abilities(&[
        abilities::dies_trigger(
            "When this creature dies, create a 1/1 black and green Insect \
             creature token with flying.",
            EffectDef::create_creature_token(
                &["Insect"],
                &[ManaColor::Black, ManaColor::Green],
                1,
                1,
            )
            .with_abilities(&[abilities::flying()]),
        ),
    ]),
);

// FDN 65 — Midnight Snack
// Audit: unsupported — Needs per-player attack history for the current turn, retained after attacking creatures leave or change controllers; current object predicates only identify individual permanents that attacked.
pub(in crate::card::sets) static MIDNIGHT_SNACK: CardRecord = CardRecord::new(
    "Midnight Snack",
    "c9b7543f-2a45-4db6-b560-d15507a58c91",
    "Kai Carpenter",
    CardRules::unsupported(),
);

// FDN 66 — Nine-Lives Familiar
// Audit: unsupported — Needs a delayed battlefield arrival carrying a frozen last-known revival-counter count minus one; entry counters cannot read a scalar captured by an earlier death trigger.
pub(in crate::card::sets) static NINE_LIVES_FAMILIAR: CardRecord = CardRecord::new(
    "Nine-Lives Familiar",
    "988c23f6-59fe-49f9-a9ce-9881dccb7033",
    "Bram Sels",
    CardRules::unsupported(),
);

// FDN 67 — Revenge of the Rats
pub(in crate::card::sets) static REVENGE_OF_THE_RATS: CardRecord = CardRecord::new(
    "Revenge of the Rats",
    "1f463c55-39a0-4f2f-aae3-0c5540bde5b7",
    "Filipe Pagliuso",
    CardRules::new_sorcery(mana_cost!("{2}{B}{B}")).with_abilities(&[
        AbilityDef::spell(
            "Create a tapped 1/1 black Rat creature token for each \
             creature card in your graveyard.",
            EffectDef::create_creature_token(&["Rat"], &[ManaColor::Black], 1, 1)
                .with_count(ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Graveyard],
                    PlayerRelation::You,
                )))
                .entering_tapped(),
        ),
        abilities::flashback(&[CostDef::Mana(mana_cost!("{2}{B}{B}"))]),
    ]),
);

// FDN 68 — Sanguine Syphoner
pub(in crate::card::sets) static SANGUINE_SYPHONER: CardRecord = CardRecord::new(
    "Sanguine Syphoner",
    "b1daf5bb-c8e9-4e79-a532-ca92a9a885cd",
    "Irina Nordsol",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Vampire", "Warlock"], 1, 3).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature attacks, each opponent loses 1 life \
             and you gain 1 life.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
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
    ]),
);

// FDN 69 — Seeker's Folly
pub(in crate::card::sets) static SEEKER_S_FOLLY: CardRecord = CardRecord::new(
    "Seeker's Folly",
    "bc359da6-8b7f-45ec-b530-ce159fc35953",
    "Valera Lutfullina",
    CardRules::new_sorcery(mana_cost!("{2}{B}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Target opponent discards two cards.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                )],
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(2),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
            ),
            AbilityDef::spell(
                "Creatures your opponents control get -1/-1 until end of turn.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Opponent,
                        ),
                    )),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-1),
                        ValueDef::Constant(-1),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ],
    )]),
);

// FDN 70 — Soul-Shackled Zombie
// Audit: unsupported — Needs a target-group constraint requiring all selected cards to have the same graveyard owner, while permitting either player's graveyard.
pub(in crate::card::sets) static SOUL_SHACKLED_ZOMBIE: CardRecord = CardRecord::new(
    "Soul-Shackled Zombie",
    "deea5690-6eb2-4353-b917-cbbf840e4e71",
    "Diana Franco",
    CardRules::unsupported(),
);

// FDN 71 — Stab
pub(in crate::card::sets) static STAB: CardRecord = CardRecord::new(
    "Stab",
    "6859a5ba-1c1c-4631-bba8-f9900b827178",
    "Milivoj Ćeran",
    CardRules::new_instant(mana_cost!("{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets -2/-2 until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(-2),
                ValueDef::Constant(-2),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// FDN 72 — Tinybones, Bauble Burglar
// Audit: unsupported — Needs counters on exiled cards and a global exile-play permission selecting opponent-owned cards with a particular counter, restricted to your turn.
pub(in crate::card::sets) static TINYBONES_BAUBLE_BURGLAR: CardRecord = CardRecord::new(
    "Tinybones, Bauble Burglar",
    "ff3d85bc-ef2d-4251-baf4-a14bd0cee61e",
    "Leonardo Santanna",
    CardRules::unsupported(),
);

// FDN 73 — Tragic Banshee
pub(in crate::card::sets) static TRAGIC_BANSHEE: CardRecord = CardRecord::new(
    "Tragic Banshee",
    "30df3e33-2f17-4067-99f1-5db6b0f41fd4",
    "Camille Alquier",
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Spirit"], 5, 3).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "Morbid — When this creature enters, target creature an \
             opponent controls gets -1/-1 until end of turn. If a creature \
             died this turn, that creature gets -13/-13 until end of turn \
             instead.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::IfElseCondition {
                condition: &TriggerConditionDef::CreatureDiedThisTurn,
                then: &EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-13),
                        ValueDef::Constant(-13),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                otherwise: &EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-1),
                        ValueDef::Constant(-1),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            },
        ),
    ]),
);

// FDN 74 — Vampire Gourmand
pub(in crate::card::sets) static VAMPIRE_GOURMAND: CardRecord = CardRecord::new(
    "Vampire Gourmand",
    "917514c0-9cd5-4b97-85b9-c4f753560ad4",
    "Chris Rallis",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Vampire"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature attacks, you may sacrifice another \
             creature. If you do, draw a card and this creature can't be \
             blocked this turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::sacrifice_permanent(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]))],
                &EffectDef::Sequence(&[
                    abilities::draw_cards(ValueDef::Constant(1)),
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            )),
        ),
    ]),
);

// FDN 75 — Vampire Soulcaller
pub(in crate::card::sets) static VAMPIRE_SOULCALLER: CardRecord = CardRecord::new(
    "Vampire Soulcaller",
    "2d076293-3b45-4878-8f67-978927cc1f68",
    "Aaron J. Riley",
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Vampire", "Warlock"], 3, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "This creature can't block.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
            },
        ),
        abilities::enters_trigger_with_targets(
            "When this creature enters, return target creature card from \
             your graveyard to your hand.",
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
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
        ),
    ]),
);

// FDN 76 — Vengeful Bloodwitch
pub(in crate::card::sets) static VENGEFUL_BLOODWITCH: CardRecord = CardRecord::new(
    "Vengeful Bloodwitch",
    "bd0c12dd-f138-45c0-9614-d83a1d8e8399",
    "Jarel Threat",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Vampire", "Warlock"], 1, 1).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "Whenever this creature or another creature you control dies, \
             target opponent loses 1 life and you gain 1 life.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
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

// FDN 77 — Zul Ashur, Lich Lord
pub(in crate::card::sets) static ZUL_ASHUR_LICH_LORD: CardRecord = CardRecord::new(
    "Zul Ashur, Lich Lord",
    "34ad4fdb-9805-45b3-ba20-e47a15d6ff38",
    "Raluca Marinescu",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Zombie", "Warlock"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::ward(&[CostDef::PayLife(2)], "Ward—Pay 2 life."),
            AbilityDef::activated_with_targets(
                "{T}: You may cast target Zombie creature card from your \
                 graveyard this turn.",
                &[CostDef::TapSource],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Zombie")),
                        ]),
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: Some(PlayerRelation::You),
                    },
                )],
                EffectDef::PermitCastFromGraveyardThisTurn {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
        ]),
);

// FDN 78 — Battlesong Berserker
pub(in crate::card::sets) static BATTLESONG_BERSERKER: CardRecord = CardRecord::new(
    "Battlesong Berserker",
    "a1f8b199-5d62-485f-b1c3-b30aa550595b",
    "Mirko Failoni",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Human", "Berserker"], 3, 4).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "Whenever you attack, target creature you control gets +1/+0 \
             and gains menace until end of turn. (It can't be blocked \
             except by two or more creatures.)",
            TriggerEventDef::attack_declared(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                1,
                None,
            ),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
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
                    AppliedEffectDef::add_ability(&abilities::menace()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// FDN 79 — Boltwave
pub(in crate::card::sets) static BOLTWAVE: CardRecord = CardRecord::new(
    "Boltwave",
    "8d1ec351-5e70-4eb2-b590-6bff94ef8178",
    "Caio Monteiro",
    CardRules::new_sorcery(mana_cost!("{R}")).with_abilities(&[AbilityDef::spell(
        "Boltwave deals 3 damage to each opponent.",
        EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::Constant(3)),
    )]),
);

// FDN 80 — Bulk Up
pub(in crate::card::sets) static BULK_UP: CardRecord = CardRecord::new(
    "Bulk Up",
    "977dcc50-da10-4281-b522-9240c1204f5d",
    "Warren Mahy",
    CardRules::new_instant(mana_cost!("{1}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Double target creature's power until end of turn.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::TargetPower(TargetIndex::PRIMARY),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::flashback(&[CostDef::Mana(mana_cost!("{4}{R}{R}"))]),
    ]),
);

// FDN 81 — Chandra, Flameshaper
// Audit: unsupported — Needs a play permission granted to a card selected from an already-exiled group, without moving that card again; existing exile-and-play instructions couple the permission to the exile move.
pub(in crate::card::sets) static CHANDRA_FLAMESHAPER: CardRecord = CardRecord::new(
    "Chandra, Flameshaper",
    "a22d21ec-0fb3-4574-a803-6442ec13167e",
    "Mark Winters",
    CardRules::unsupported(),
);

// FDN 82 — Courageous Goblin
pub(in crate::card::sets) static COURAGEOUS_GOBLIN: CardRecord = CardRecord::new(
    "Courageous Goblin",
    "8db6819c-666a-409d-85a5-b9ac34d8dd2f",
    "Ben Wootten",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature attacks while you control a creature \
             with power 4 or greater, this creature gets +1/+0 and gains \
             menace until end of turn. (It can't be blocked except by two \
             or more creatures.)",
            TriggerEventDef::While {
                event: &TriggerEventDef::attacks(ObjectPredicateDef::Source),
                condition: &TriggerConditionDef::ObjectCount {
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
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
                    AppliedEffectDef::add_ability(&abilities::menace()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// FDN 83 — Crackling Cyclops
pub(in crate::card::sets) static CRACKLING_CYCLOPS: CardRecord = CardRecord::new(
    "Crackling Cyclops",
    "6e5b899a-52f7-471b-ad50-4fa6566758fd",
    "Kev Walker",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Cyclops", "Wizard"], 0, 4).with_abilities(&[
        AbilityDef::triggered(
            "Whenever you cast a noncreature spell, this creature gets \
             +3/+0 until end of turn.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(3),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// FDN 84 — Dragon Trainer
pub(in crate::card::sets) static DRAGON_TRAINER: CardRecord = CardRecord::new(
    "Dragon Trainer",
    "91bd75a1-cb54-4e38-9ce1-e8f32a73c6eb",
    "Steve Prescott",
    CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Human"], 1, 1).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, create a 4/4 red Dragon creature \
             token with flying.",
            EffectDef::create_creature_token(&["Dragon"], &[ManaColor::Red], 4, 4)
                .with_abilities(&[abilities::flying()]),
        ),
    ]),
);

// FDN 85 — Electroduplicate
pub(in crate::card::sets) static ELECTRODUPLICATE: CardRecord = CardRecord::new(
    "Electroduplicate",
    "abb06b1c-5d4e-49b9-9c4a-e60ab656a257",
    "Warren Mahy",
    CardRules::new_sorcery(mana_cost!("{2}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Create a token that's a copy of target creature you control, \
             except it has haste and \"At the beginning of the end step, \
             sacrifice this token.\"",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::create_token_from_copy(&TokenCopyDef {
                object: &EffectRecipientDef::Target(TargetIndex::PRIMARY),
                exceptions: CopyExceptionsDef {
                    added_abilities: &[
                        CopyAbilityDef::Ability(&abilities::haste()),
                        CopyAbilityDef::Ability(&AbilityDef::triggered(
                            "At the beginning of the end step, sacrifice this token.",
                            TriggerEventDef::StepBegins {
                                step: TurnStepDef::End,
                                player: PlayerRelation::Any,
                            },
                            EffectDef::sacrifice(EffectRecipientDef::Source),
                        )),
                    ],
                    ..CopyExceptionsDef::NONE
                },
            }),
        ),
        abilities::flashback(&[CostDef::Mana(mana_cost!("{2}{R}{R}"))]),
    ]),
);

// FDN 86 — Fiery Annihilation
// Audit: unsupported — Needs a target constraint tying the optional Equipment target to the earlier creature target's attachment relation, including rechecking both targets at resolution.
pub(in crate::card::sets) static FIERY_ANNIHILATION: CardRecord = CardRecord::new(
    "Fiery Annihilation",
    "54fe00aa-d284-48f9-b5a2-1bd4c5fa8e58",
    "Warren Mahy",
    CardRules::unsupported(),
);

// FDN 87 — Goblin Boarders
// Audit: unsupported — Needs per-player attack history for the current turn, retained after attacking creatures leave or change controllers; current object predicates only identify individual permanents that attacked.
pub(in crate::card::sets) static GOBLIN_BOARDERS: CardRecord = CardRecord::new(
    "Goblin Boarders",
    "4409a063-bf2a-4a49-803e-3ce6bd474353",
    "Filipe Pagliuso",
    CardRules::unsupported(),
);

// FDN 88 — Goblin Negotiation
// Audit: unsupported — Needs excess-damage output from an ordinary damage instruction, including prevention and lethal deathtouch semantics; the current excess continuation is available only on fight.
pub(in crate::card::sets) static GOBLIN_NEGOTIATION: CardRecord = CardRecord::new(
    "Goblin Negotiation",
    "f2016585-e26c-4d13-b09f-af6383c192f7",
    "Svetlin Velinov",
    CardRules::unsupported(),
);

// FDN 89 — Gorehorn Raider
// Audit: unsupported — Needs per-player attack history for the current turn, retained after attacking creatures leave or change controllers; current object predicates only identify individual permanents that attacked.
pub(in crate::card::sets) static GOREHORN_RAIDER: CardRecord = CardRecord::new(
    "Gorehorn Raider",
    "78ce6c40-3452-4aa0-a45b-dbfd70f8d220",
    "Warren Mahy",
    CardRules::unsupported(),
);

// FDN 90 — Incinerating Blast
pub(in crate::card::sets) static INCINERATING_BLAST: CardRecord = CardRecord::new(
    "Incinerating Blast",
    "d58e20ab-c5ca-4295-884d-78efdaa83243",
    "Zoltan Boros",
    CardRules::new_sorcery(mana_cost!("{4}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Incinerating Blast deals 6 damage to target creature.\nYou \
         may discard a card. If you do, draw a card.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(6),
            ),
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::DiscardCards(1)],
                &abilities::draw_cards(ValueDef::Constant(1)),
            )),
        ]),
    )]),
);

// FDN 91 — Kellan, Planar Trailblazer (alternate printing)
const KELLAN_PLANAR_TRAILBLAZER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KELLAN_PLANAR_TRAILBLAZER,
    1,
    "f46a9329-7b91-441d-8653-50c1152c9120",
    "Zoltan Boros",
);

// FDN 92 — Rite of the Dragoncaller
pub(in crate::card::sets) static RITE_OF_THE_DRAGONCALLER: CardRecord = CardRecord::new(
    "Rite of the Dragoncaller",
    "673e4561-8dfd-46db-b492-878009666ac7",
    "PINDURSKI",
    CardRules::new_enchantment(mana_cost!("{4}{R}{R}")).with_abilities(&[AbilityDef::triggered(
        "Whenever you cast an instant or sorcery spell, create a 5/5 \
         red Dragon creature token with flying.",
        TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Instant),
                ObjectPredicateDef::HasType(CardType::Sorcery),
            ]),
            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
        ])),
        EffectDef::create_creature_token(&["Dragon"], &[ManaColor::Red], 5, 5)
            .with_abilities(&[abilities::flying()]),
    )]),
);

// FDN 93 — Searslicer Goblin
// Audit: unsupported — Needs per-player attack history for the current turn, retained after attacking creatures leave or change controllers; current object predicates only identify individual permanents that attacked.
pub(in crate::card::sets) static SEARSLICER_GOBLIN: CardRecord = CardRecord::new(
    "Searslicer Goblin",
    "94ad0b97-a318-4e76-ac79-b3e83417c333",
    "Wayne Reynolds",
    CardRules::unsupported(),
);

// FDN 94 — Slumbering Cerberus
pub(in crate::card::sets) static SLUMBERING_CERBERUS: CardRecord = CardRecord::new(
    "Slumbering Cerberus",
    "9d06faa8-201d-45db-b398-ad56f7b01848",
    "Kari Christensen",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Dog"], 4, 2).with_abilities(&[
        AbilityDef::static_ability(
            "This creature doesn't untap during your untap step.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
            },
        ),
        AbilityDef::triggered_if(
            "Morbid — At the beginning of each end step, if a creature \
             died this turn, untap this creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::Any,
            },
            &TriggerConditionDef::CreatureDiedThisTurn,
            EffectDef::Untap {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// FDN 95 — Sower of Chaos
pub(in crate::card::sets) static SOWER_OF_CHAOS: CardRecord = CardRecord::new(
    "Sower of Chaos",
    "7ff50606-491c-4946-8d03-719b01cfad77",
    "Brian Valeza",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Devil"], 4, 3).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{2}{R}: Target creature can't block this turn.",
            &[CostDef::Mana(mana_cost!("{2}{R}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// FDN 96 — Strongbox Raider
// Audit: unsupported — Needs per-player attack history for the current turn, retained after attacking creatures leave or change controllers; current object predicates only identify individual permanents that attacked.
pub(in crate::card::sets) static STRONGBOX_RAIDER: CardRecord = CardRecord::new(
    "Strongbox Raider",
    "b2223eb8-59f9-489b-a3f3-b6496218cb79",
    "Craig J Spearing",
    CardRules::unsupported(),
);

// FDN 97 — Twinflame Tyrant
// Audit: unsupported — Needs a prospective damage amount multiplier filtered by source controller and damage recipient; current damage rules support prevention and limits, but no multiplication replacement.
pub(in crate::card::sets) static TWINFLAME_TYRANT: CardRecord = CardRecord::new(
    "Twinflame Tyrant",
    "1eb34f51-0bd2-43c3-af95-2ce8dabcc7bb",
    "Xabi Gaztelua",
    CardRules::unsupported(),
);

// FDN 98 — Ambush Wolf
pub(in crate::card::sets) static AMBUSH_WOLF: CardRecord = CardRecord::new(
    "Ambush Wolf",
    "2903832c-318e-42ab-bf58-c682ec2f7afd",
    "Paolo Parente",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Wolf"], 4, 2).with_abilities(&[
        abilities::flash(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, exile up to one target card from a \
             graveyard.",
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: None,
                },
                1,
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Exile,
                ZonePlacement::Top,
            ),
        ),
    ]),
);

// FDN 99 — Apothecary Stomper
pub(in crate::card::sets) static APOTHECARY_STOMPER: CardRecord = CardRecord::new(
    "Apothecary Stomper",
    "680b7b0c-0e1b-46ce-9917-9fc6e05aa148",
    "Edgar Sánchez Hidalgo",
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Elephant"], 4, 4).with_abilities(&[
        abilities::vigilance(),
        AbilityDef::modal_triggered(
            "When this creature enters, choose one —",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[
                AbilityDef::spell_with_targets(
                    "Put two +1/+1 counters on target creature you control.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::HasType(CardType::Creature),
                            zones: &[ZoneKind::Battlefield],
                            controller: Some(PlayerRelation::You),
                            owner: None,
                        },
                    )],
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(2),
                    },
                ),
                AbilityDef::spell(
                    "You gain 4 life.",
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(4),
                    },
                ),
            ],
        ),
    ]),
);

// FDN 100 — Beast-Kin Ranger
pub(in crate::card::sets) static BEAST_KIN_RANGER: CardRecord = CardRecord::new(
    "Beast-Kin Ranger",
    "0102e0be-5783-4825-9489-713b1b1df0b2",
    "Alexander Mokhov",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Elf", "Ranger"], 3, 3).with_abilities(&[
        abilities::trample(),
        AbilityDef::triggered(
            "Whenever another creature you control enters, this creature \
             gets +1/+0 until end of turn.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// FDN 101 — Cackling Prowler
pub(in crate::card::sets) static CACKLING_PROWLER: CardRecord = CardRecord::new(
    "Cackling Prowler",
    "1bd8e971-c075-4203-8d83-c28f22d4f9b9",
    "Christopher Burdett",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Hyena", "Rogue"], 4, 3).with_abilities(&[
        abilities::ward(&[CostDef::Mana(mana_cost!("{2}"))], "Ward {2}"),
        AbilityDef::triggered_if(
            "Morbid — At the beginning of your end step, if a creature \
             died this turn, put a +1/+1 counter on this creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::CreatureDiedThisTurn,
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// FDN 102 — Eager Trufflesnout
pub(in crate::card::sets) static EAGER_TRUFFLESNOUT: CardRecord = CardRecord::new(
    "Eager Trufflesnout",
    "a6e8433d-eb2a-43d1-b59b-7d70ff97c8e7",
    "Filipe Pagliuso",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Boar"], 4, 2).with_abilities(&[
        abilities::trample(),
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, \
             create a Food token. (It's an artifact with \"{2}, {T}, \
             Sacrifice this token: You gain 3 life.\")",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::create_token(tokens::food()).with_count(ValueDef::Constant(1)),
        ),
    ]),
);

// FDN 103 — Elfsworn Giant
pub(in crate::card::sets) static ELFSWORN_GIANT: CardRecord = CardRecord::new(
    "Elfsworn Giant",
    "5128a5be-ffa6-4998-8488-872d80b24cb2",
    "Dave Dorman",
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Giant"], 5, 3).with_abilities(&[
        abilities::reach(),
        AbilityDef::triggered(
            "Landfall — Whenever a land you control enters, create a 1/1 \
             green Elf Warrior creature token.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::create_creature_token(&["Elf", "Warrior"], &[ManaColor::Green], 1, 1),
        ),
    ]),
);

// FDN 104 — Elvish Regrower
pub(in crate::card::sets) static ELVISH_REGROWER: CardRecord = CardRecord::new(
    "Elvish Regrower",
    "2694e3cd-26ed-4a10-ae55-fb84d7800253",
    "Zoltan Boros",
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Elf", "Druid"], 4, 3).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this creature enters, return target permanent card from \
             your graveyard to your hand.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::HasType(CardType::Planeswalker),
                    ]),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
        ),
    ]),
);

// FDN 105 — Felling Blow
pub(in crate::card::sets) static FELLING_BLOW: CardRecord = CardRecord::new(
    "Felling Blow",
    "96948ae3-b15d-4d6d-aa73-9f52084cd903",
    "Miro Petrov",
    CardRules::new_sorcery(mana_cost!("{2}{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Put a +1/+1 counter on target creature you control. Then that \
         creature deals damage equal to its power to target creature \
         an opponent controls.",
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
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
            EffectDef::damage_from(
                ObjectRefDef::Target(TargetIndex::PRIMARY),
                EffectRecipientDef::Target(TargetIndex(1)),
                ValueDef::TargetPower(TargetIndex::PRIMARY),
            ),
        ]),
    )]),
);

// FDN 106 — Loot, Exuberant Explorer
pub(in crate::card::sets) static LOOT_EXUBERANT_EXPLORER: CardRecord = CardRecord::new(
    "Loot, Exuberant Explorer",
    "09980ce6-425b-4e03-94d0-0f02043cb361",
    "Arif Wijaya",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Beast", "Noble"], 1, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "You may play an additional land on each of your turns.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Controller,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::MayPlayAdditionalLands(1)),
                },
            ),
            AbilityDef::activated(
                "{4}{G}{G}, {T}: Look at the top six cards of your library. \
                 You may reveal a creature card with mana value less than or \
                 equal to the number of lands you control from among them and \
                 put it onto the battlefield. Put the rest on the bottom in a \
                 random order.",
                &[CostDef::Mana(mana_cost!("{4}{G}{G}")), CostDef::TapSource],
                EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                    source: ObjectCollectionSourceDef::TopCards {
                        player: PlayerRefDef::EffectController,
                        count: ValueDef::Constant(6),
                    },
                    actor: PlayerRefDef::EffectController,
                    inspection: CollectionInspectionDef::Look,
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ManaValueAtMostValue(ValueDef::CountMatchingObjects(
                            &ObjectQueryDef::matching(
                                ObjectPredicateDef::HasType(CardType::Land),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            ),
                        )),
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
                                EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("random_bottom"),
                                )),
                                ZoneKind::Library,
                                ZonePlacement::Bottom,
                            ),
                        }),
                    ]),
                }),
            ),
        ]),
);

// FDN 107 — Mossborn Hydra
pub(in crate::card::sets) static MOSSBORN_HYDRA: CardRecord = CardRecord::new(
    "Mossborn Hydra",
    "7054a0d7-396f-40b4-ab24-db591c3b08f0",
    "Monztre",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Elemental", "Hydra"], 0, 0).with_abilities(&[
        abilities::trample(),
        AbilityDef::as_enters(
            "This creature enters with a +1/+1 counter on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 1,
                },
            ),
        ),
        AbilityDef::triggered(
            "Landfall — Whenever a land you control enters, double the \
             number of +1/+1 counters on this creature.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::CountersOnSource(CounterKind::PlusOnePlusOne),
            },
        ),
    ]),
);

// FDN 108 — Needletooth Pack
pub(in crate::card::sets) static NEEDLETOOTH_PACK: CardRecord = CardRecord::new(
    "Needletooth Pack",
    "993c1679-e02b-44f2-b34e-12fd6b5142e9",
    "Monztre",
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Dinosaur"], 4, 5).with_abilities(&[
        AbilityDef::triggered_if_with_targets(
            "Morbid — At the beginning of your end step, if a creature \
             died this turn, put two +1/+1 counters on target creature you \
             control.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::CreatureDiedThisTurn,
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(2),
            },
        ),
    ]),
);

// FDN 109 — Preposterous Proportions
pub(in crate::card::sets) static PREPOSTEROUS_PROPORTIONS: CardRecord = CardRecord::new(
    "Preposterous Proportions",
    "acb65189-60e4-42e0-9fb1-da6b716b91d7",
    "Ben Wootten",
    CardRules::new_sorcery(mana_cost!("{5}{G}{G}")).with_abilities(&[AbilityDef::spell(
        "Creatures you control get +10/+10 and gain vigilance until \
         end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ))),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(10),
                    ValueDef::Constant(10),
                ),
                AppliedEffectDef::add_ability(&abilities::vigilance()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// FDN 110 — Quakestrider Ceratops
pub(in crate::card::sets) static QUAKESTRIDER_CERATOPS: CardRecord = CardRecord::new(
    "Quakestrider Ceratops",
    "067f72c2-ead6-4879-bc9d-696c9f87c0b2",
    "Josiah \"Jo\" Cameron",
    CardRules::new_creature(mana_cost!("{3}{G}{G}{G}"), &["Dinosaur"], 12, 8),
);

// FDN 111 — Quilled Greatwurm
// Audit: unsupported — Needs a graveyard casting cost that removes six counters of player-chosen kinds distributed among controlled creatures; current removal costs fix one counter kind or one source.
pub(in crate::card::sets) static QUILLED_GREATWURM: CardRecord = CardRecord::new(
    "Quilled Greatwurm",
    "31b60531-3d33-4e66-923a-29008716b15c",
    "Michal Ivan",
    CardRules::unsupported(),
);

// FDN 112 — Spinner of Souls
// Audit: unsupported — Needs a reveal-until operation that keeps nonmatching cards in the library and puts them on the bottom in random order; MillUntil mills the intervening cards and ExileFromTopUntil exiles them.
pub(in crate::card::sets) static SPINNER_OF_SOULS: CardRecord = CardRecord::new(
    "Spinner of Souls",
    "f50a8dec-b079-4192-9098-6cdc1026c693",
    "Xavier Ribeiro",
    CardRules::unsupported(),
);

// FDN 113 — Sylvan Scavenging
pub(in crate::card::sets) static SYLVAN_SCAVENGING: CardRecord = CardRecord::new(
    "Sylvan Scavenging",
    "c35b683c-d3b2-46a1-876a-81b34e8ba2fc",
    "Josiah \"Jo\" Cameron",
    CardRules::new_enchantment(mana_cost!("{1}{G}{G}")).with_abilities(&[
        AbilityDef::modal_triggered(
            "At the beginning of your end step, choose one —",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::You,
            },
            &[
                AbilityDef::spell_with_targets(
                    "Put a +1/+1 counter on target creature you control.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::HasType(CardType::Creature),
                            zones: &[ZoneKind::Battlefield],
                            controller: Some(PlayerRelation::You),
                            owner: None,
                        },
                    )],
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                ),
                AbilityDef::spell(
                    "Create a 3/3 green Raccoon creature token if you control a \
                     creature with power 4 or greater.",
                    EffectDef::IfCondition {
                        condition: &TriggerConditionDef::ObjectCount {
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
                        then: &EffectDef::create_creature_token(
                            &["Raccoon"],
                            &[ManaColor::Green],
                            3,
                            3,
                        ),
                    },
                ),
            ],
        ),
    ]),
);

// FDN 114 — Treetop Snarespinner
pub(in crate::card::sets) static TREETOP_SNARESPINNER: CardRecord = CardRecord::new(
    "Treetop Snarespinner",
    "88e68fa3-159d-49a6-8ac6-afc9bd6f1718",
    "Steve Ellis",
    // Reach and deathtouch already answer anything that attacks into it, so
    // the counters are what a stalled board turns spare mana into.
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Spider"], 1, 4).with_abilities(&[
        abilities::reach(),
        abilities::deathtouch(),
        AbilityDef::activated_with_targets(
            "{2}{G}: Put a +1/+1 counter on target creature you control. Activate only as a \
             sorcery.",
            &[CostDef::Mana(mana_cost!("{2}{G}"))],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// FDN 115 — Alesha, Who Laughs at Fate
// Audit: unsupported — Needs per-player attack history for the current turn, retained after attacking creatures leave or change controllers; current object predicates only identify individual permanents that attacked.
pub(in crate::card::sets) static ALESHA_WHO_LAUGHS_AT_FATE: CardRecord = CardRecord::new(
    "Alesha, Who Laughs at Fate",
    "a93e3406-4e29-4bc0-ae52-cbd2ac1f99a4",
    "Ekaterina Burmak",
    CardRules::unsupported(),
);

// FDN 116 — Anthem of Champions
pub(in crate::card::sets) static ANTHEM_OF_CHAMPIONS: CardRecord = CardRecord::new(
    "Anthem of Champions",
    "42fe3a40-9cbe-4235-86f9-32576aaebba8",
    "Chris Rallis",
    CardRules::new_enchantment(mana_cost!("{G}{W}")).with_abilities(&[AbilityDef::static_ability(
        "Creatures you control get +1/+1.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ))),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(1),
                ValueDef::Constant(1),
            ),
        },
    )]),
);

// FDN 117 — Ashroot Animist
pub(in crate::card::sets) static ASHROOT_ANIMIST: CardRecord = CardRecord::new(
    "Ashroot Animist",
    "dece147f-d71a-4c95-9fe5-f5c862ef14ac",
    "Caio Monteiro",
    CardRules::new_creature(mana_cost!("{2}{R}{G}"), &["Lizard", "Druid"], 4, 4).with_abilities(&[
        abilities::trample(),
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks, another target creature you \
             control gains trample and gets +X/+X until end of turn, where \
             X is this creature's power.",
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
                    AppliedEffectDef::add_ability(&abilities::trample()),
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::SourcePower,
                        ValueDef::SourcePower,
                    ),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// FDN 118 — Dreadwing Scavenger
pub(in crate::card::sets) static DREADWING_SCAVENGER: CardRecord = CardRecord::new(
    "Dreadwing Scavenger",
    "e24d838b-ab48-410a-9a50-dbfea5da089b",
    "Xavier Ribeiro",
    CardRules::new_creature(mana_cost!("{1}{U}{B}"), &["Nightmare", "Bird"], 2, 2).with_abilities(
        &[
            abilities::flying(),
            AbilityDef::triggered(
                "Whenever this creature enters or attacks, draw a card, then \
                 discard a card.",
                TriggerEventDef::AnyOf(&[
                    TriggerEventDef::zone_changed(
                        ObjectPredicateDef::Source,
                        None,
                        Some(ZoneKind::Battlefield),
                    ),
                    TriggerEventDef::attacks(ObjectPredicateDef::Source),
                ]),
                EffectDef::Sequence(&[
                    abilities::draw_cards(ValueDef::Constant(1)),
                    EffectDef::Discard {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                        selection: DiscardSelectionDef::RecipientChooses,
                        then: None,
                    },
                ]),
            ),
            AbilityDef::static_ability(
                "Threshold — This creature gets +1/+1 and has deathtouch as \
                 long as there are seven or more cards in your graveyard.",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ObjectCount {
                        query: ObjectQueryDef::matching(
                            ObjectPredicateDef::Any,
                            &[ZoneKind::Graveyard],
                            PlayerRelation::You,
                        ),
                        comparison: ComparisonDef::GreaterOrEqual,
                        amount: 7,
                    },
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::Source,
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
        ],
    ),
);

// FDN 119 — Elenda, Saint of Dusk
// Audit: unsupported — Needs hexproof filtered by an opposing spell or ability source being an instant; existing hexproof has no source-characteristic filter.
pub(in crate::card::sets) static ELENDA_SAINT_OF_DUSK: CardRecord = CardRecord::new(
    "Elenda, Saint of Dusk",
    "24955f5f-093c-4d33-b0c1-911cd36032ce",
    "Chris Rahn",
    CardRules::unsupported(),
);

// FDN 120 — Fiendish Panda
pub(in crate::card::sets) static FIENDISH_PANDA: CardRecord = CardRecord::new(
    "Fiendish Panda",
    "4e434d74-cad0-45f5-bc8d-f34aa5e1d879",
    "Brian Valeza",
    CardRules::new_creature(mana_cost!("{2}{W}{B}"), &["Bear", "Demon"], 3, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever you gain life, put a +1/+1 counter on this creature.",
            TriggerEventDef::LifeGained(PlayerRelation::You),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::triggered_with_targets(
            "When this creature dies, return another target non-Bear \
             creature card with mana value less than or equal to this \
             creature's power from your graveyard to the battlefield.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype(SubtypeDef::Literal(
                            "Bear",
                        ))),
                        ObjectPredicateDef::ManaValueAtMostValue(ValueDef::SourcePower),
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
    ]),
);

// FDN 121 — Koma, World-Eater
pub(in crate::card::sets) static KOMA_WORLD_EATER: CardRecord = CardRecord::new(
    "Koma, World-Eater",
    "c3b92caa-3401-4b11-9515-152f3e057c05",
    "Mark Zug",
    CardRules::new_creature(mana_cost!("{3}{G}{G}{U}{U}"), &["Serpent"], 8, 12)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "This spell can't be countered.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBeCountered),
                },
            )
            .with_source_zones(&[ZoneKind::Stack]),
            abilities::trample(),
            abilities::ward(&[CostDef::Mana(mana_cost!("{4}"))], "Ward {4}"),
            AbilityDef::triggered(
                "Whenever Koma deals combat damage to a player, create four \
                 3/3 blue Serpent creature tokens named Koma's Coil.",
                TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
                EffectDef::create_creature_token(&["Serpent"], &[ManaColor::Blue], 3, 3)
                    .with_count(ValueDef::Constant(4))
                    .with_name("Koma's Coil"),
            ),
        ]),
);

// FDN 122 — Kykar, Zephyr Awakener
pub(in crate::card::sets) static KYKAR_ZEPHYR_AWAKENER: CardRecord = CardRecord::new(
    "Kykar, Zephyr Awakener",
    "6c980998-7124-4ec6-a0b6-e8d9a2364925",
    "Dmitry Burmak",
    CardRules::new_creature(mana_cost!("{2}{W}{U}"), &["Bird", "Wizard"], 3, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::modal_triggered(
                "Whenever you cast a noncreature spell, choose one —",
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                &[
                    AbilityDef::spell_with_targets(
                        "Exile another target creature you control. Return that card \
                         to the battlefield under its owner's control at the beginning \
                         of the next end step.",
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
                        EffectDef::WithZoneMoveResult {
                            effect: &EffectDef::move_to_zone(
                                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                ZoneKind::Exile,
                                ZonePlacement::Top,
                            ),
                            binding: crate::Binding!("blinked"),
                            then: &EffectDef::InstallTrigger(InstalledTriggerDef::once(
                                &AbilityDef::triggered(
                                    "At the beginning of the next end step, return that card to \
                                     the battlefield.",
                                    TriggerEventDef::StepBegins {
                                        step: TurnStepDef::End,
                                        player: PlayerRelation::Any,
                                    },
                                    EffectDef::move_to_zone(
                                        EffectRecipientDef::objects(
                                            ObjectSetDef::ZoneChangeSuccessorsOfBinding(
                                                crate::Binding!("blinked"),
                                            ),
                                        ),
                                        ZoneKind::Battlefield,
                                        ZonePlacement::Top,
                                    ),
                                ),
                            )),
                        },
                    ),
                    AbilityDef::spell(
                        "Create a 1/1 white Spirit creature token with flying.",
                        EffectDef::create_creature_token(&["Spirit"], &[ManaColor::White], 1, 1)
                            .with_abilities(&[abilities::flying()]),
                    ),
                ],
            ),
        ]),
);

// FDN 123 — Niv-Mizzet, Visionary
// Audit: unsupported — Needs a noncombat-only damage trigger matcher; the shared damage-kind vocabulary currently exposes Any and Combat without Noncombat.
pub(in crate::card::sets) static NIV_MIZZET_VISIONARY: CardRecord = CardRecord::new(
    "Niv-Mizzet, Visionary",
    "7a69a618-d588-4745-8ede-0ff0a9f356f1",
    "Dan Murayama Scott",
    CardRules::unsupported(),
);

// FDN 124 — Perforating Artist
// Audit: unsupported — Needs per-player attack history for the current turn, retained after attacking creatures leave or change controllers; current object predicates only identify individual permanents that attacked.
pub(in crate::card::sets) static PERFORATING_ARTIST: CardRecord = CardRecord::new(
    "Perforating Artist",
    "72980409-53f0-43c1-965e-06f22e7bb608",
    "Arif Wijaya",
    CardRules::unsupported(),
);

// FDN 125 — Wardens of the Cycle
pub(in crate::card::sets) static WARDENS_OF_THE_CYCLE: CardRecord = CardRecord::new(
    "Wardens of the Cycle",
    "83ea9b2c-5723-4eff-88ac-6669975939e3",
    "Caroline Gariba",
    CardRules::new_creature(mana_cost!("{1}{B}{G}{G}"), &["Elf", "Warlock"], 3, 4).with_abilities(
        &[AbilityDef::modal_triggered(
            "Morbid — At the beginning of your end step, if a creature \
             died this turn, choose one —",
            TriggerEventDef::While {
                event: &TriggerEventDef::StepBegins {
                    step: TurnStepDef::End,
                    player: PlayerRelation::You,
                },
                condition: &TriggerConditionDef::CreatureDiedThisTurn,
            },
            &[
                AbilityDef::spell(
                    "You gain 2 life.",
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(2),
                    },
                ),
                AbilityDef::spell(
                    "You draw a card and you lose 1 life.",
                    EffectDef::Sequence(&[
                        abilities::draw_cards(ValueDef::Constant(1)),
                        EffectDef::LoseLife {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(1),
                        },
                    ]),
                ),
            ],
        )],
    ),
);

// FDN 126 — Zimone, Paradox Sculptor
// Audit: unsupported — Needs an effect that doubles a dynamically enumerated collection of counter kinds on each selected object; existing counter effects name one fixed counter kind.
pub(in crate::card::sets) static ZIMONE_PARADOX_SCULPTOR: CardRecord = CardRecord::new(
    "Zimone, Paradox Sculptor",
    "20ccbfdd-ddae-440c-9bc0-38b15a56fdd1",
    "Nathaniel Himawan",
    CardRules::unsupported(),
);

// FDN 127 — Banner of Kinship
// Audit: unsupported — Needs entry-value queries to read the entering object's newly chosen creature type before that object exists on the battlefield; current query matching cannot see that prospective scalar choice.
pub(in crate::card::sets) static BANNER_OF_KINSHIP: CardRecord = CardRecord::new(
    "Banner of Kinship",
    "a14c16c0-4053-46b0-8fa6-be8b4a7a1c8a",
    "Olena Richards",
    CardRules::unsupported(),
);

// FDN 128 — Fishing Pole
// Audit: unsupported — Needs a granted mana-independent activation cost that taps the granting Equipment, plus an untap event carrying its attached creature and an actual counter-removal result for the token condition.
pub(in crate::card::sets) static FISHING_POLE: CardRecord = CardRecord::new(
    "Fishing Pole",
    "c95ab836-3277-4223-9aaa-ef2c77256b65",
    "Franz Vohwinkel",
    CardRules::unsupported(),
);

// FDN 129 — Leyline Axe
pub(in crate::card::sets) static LEYLINE_AXE: CardRecord = CardRecord::new(
    "Leyline Axe",
    "b9c03336-a321-4c06-94d1-809f328fabd8",
    "Edgar Sánchez Hidalgo",
    CardRules::new_artifact(mana_cost!("{4}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::begin_game_on_battlefield(),
            AbilityDef::static_ability(
                "Equipped creature gets +1/+1 and has double strike and trample.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_ability(&abilities::double_strike()),
                        AppliedEffectDef::add_ability(&abilities::trample()),
                    ]),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{3}"))], "Equip {3}"),
        ]),
);

// FDN 130 — Quick-Draw Katana
pub(in crate::card::sets) static QUICK_DRAW_KATANA: CardRecord = CardRecord::new(
    "Quick-Draw Katana",
    "69beec98-c89c-4673-953c-8b3ef3d81560",
    "Paolo Parente",
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "During your turn, equipped creature gets +2/+0 and has first \
                 strike. (It deals combat damage before creatures without \
                 first strike.)",
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ActivePlayer(PlayerRelation::You),
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::modify_power_toughness(
                                ValueDef::Constant(2),
                                ValueDef::Constant(0),
                            ),
                            AppliedEffectDef::add_ability(&abilities::first_strike()),
                        ]),
                    },
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{2}"))], "Equip {2}"),
        ]),
);

// FDN 131 — Ravenous Amulet
pub(in crate::card::sets) static RAVENOUS_AMULET: CardRecord = CardRecord::new(
    "Ravenous Amulet",
    "80cadee5-6f26-4440-ad31-a8e573a90436",
    "Igor Krstic",
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::activated(
            "{1}, {T}, Sacrifice a creature: Draw a card and put a soul \
             counter on this artifact. Activate only as a sorcery.",
            &[
                CostDef::Mana(mana_cost!("{1}")),
                CostDef::TapSource,
                CostDef::sacrifice_permanent(ObjectPredicateDef::HasType(CardType::Creature)),
            ],
            EffectDef::Sequence(&[
                abilities::draw_cards(ValueDef::Constant(1)),
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::named("soul"),
                    amount: ValueDef::Constant(1),
                },
            ]),
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
        AbilityDef::activated(
            "{4}, {T}, Sacrifice this artifact: Each opponent loses life \
             equal to the number of soul counters on this artifact.",
            &[
                CostDef::Mana(mana_cost!("{4}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Opponent,
                amount: ValueDef::CountersOnSource(CounterKind::named("soul")),
            },
        ),
    ]),
);

// FDN 132 — Scrawling Crawler
pub(in crate::card::sets) static SCRAWLING_CRAWLER: CardRecord = CardRecord::new(
    "Scrawling Crawler",
    "a1176dcf-40ee-4342-aa74-791b8352e99a",
    "Miro Petrov",
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Phyrexian", "Construct"], 3, 2)
        .with_abilities(&[
            AbilityDef::triggered(
                "At the beginning of your upkeep, each player draws a card.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::You,
                },
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::EachPlayer,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::triggered(
                "Whenever an opponent draws a card, that player loses 1 life.",
                TriggerEventDef::DrewCard(DrawEventMatcherDef::any(PlayerRelation::Opponent)),
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::EventPlayer,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// FDN 133 — Soulstone Sanctuary
pub(in crate::card::sets) static SOULSTONE_SANCTUARY: CardRecord = CardRecord::new(
    "Soulstone Sanctuary",
    "642553a7-6d0f-483d-a873-3a703786db42",
    "Daniel Ljunggren",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{4}: This land becomes a 3/3 creature with vigilance and all \
             creature types. It's still a land.",
            &[CostDef::Mana(mana_cost!("{4}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                    AppliedEffectDef::set_base_power_toughness(
                        ValueDef::Constant(3),
                        ValueDef::Constant(3),
                    ),
                    AppliedEffectDef::add_creature_types(CreatureTypeSetDef::ALL),
                    AppliedEffectDef::add_ability(&abilities::vigilance()),
                ]),
                duration: ResolvedEffectDurationDef::Permanent,
            },
        ),
    ]),
);

// FDN 134 — Ajani, Caller of the Pride (reprint)
const AJANI_CALLER_OF_THE_PRIDE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m13::AJANI_CALLER_OF_THE_PRIDE,
    "59793f1c-8c7e-433e-9c09-40aa3ce931a1",
    "D. Alexander Gregory",
);

// FDN 135 — Ajani's Pridemate (reprint)
const AJANI_S_PRIDEMATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m11::AJANI_S_PRIDEMATE,
    "222c1a68-e34c-4103-b1be-17d4ceaef6ce",
    "Kevin Sidharta",
);

// FDN 136 — Angel of Finality (reprint)
const ANGEL_OF_FINALITY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_c13::ANGEL_OF_FINALITY,
    "baaabd52-3aa9-4e2f-9369-d4db8b405ba8",
    "Howard Lyon",
);

// FDN 137 — Authority of the Consuls (reprint)
const AUTHORITY_OF_THE_CONSULS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_kld::AUTHORITY_OF_THE_CONSULS,
    "42ce2d7f-5924-47c0-b5ed-dacf9f9617a0",
    "Lake Hurwitz",
);

// FDN 138 — Banishing Light (reprint)
const BANISHING_LIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_jou::BANISHING_LIGHT,
    "e38dc3b3-1629-491b-8afd-0e7a9a857713",
    "Willian Murai",
);

// FDN 139 — Cathar Commando (reprint)
const CATHAR_COMMANDO_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mid::CATHAR_COMMANDO,
    "19cf024d-edb6-4a79-8676-73f8db0cdf1f",
    "Evyn Fong",
);

// FDN 140 — Day of Judgment (reprint)
const DAY_OF_JUDGMENT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_zen::DAY_OF_JUDGMENT,
    "96e84bdc-8a9a-4c58-ba8b-9f052fd60069",
    "Vincent Proce",
);

// FDN 141 — Giada, Font of Hope (reprint)
const GIADA_FONT_OF_HOPE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_snc::GIADA_FONT_OF_HOPE,
    "8ae6fc26-cfad-4da8-98d9-49c27c24d293",
    "Kai Carpenter",
);

// FDN 142 — Healer's Hawk (reprint)
const HEALER_S_HAWK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_grn::HEALER_S_HAWK,
    "cc8e4563-04bb-46b5-835e-64ba11c0e972",
    "Milivoj Ćeran",
);

// FDN 143 — Make Your Move (reprint)
const MAKE_YOUR_MOVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mkm::MAKE_YOUR_MOVE,
    "7368f861-3288-4645-90a7-ca35d6da3721",
    "Xabi Gaztelua",
);

// FDN 144 — Mischievous Pup (reprint)
const MISCHIEVOUS_PUP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lci::MISCHIEVOUS_PUP,
    "7214d984-6400-44d7-bde6-57d96b606e78",
    "Devin Platts",
);

// FDN 145 — Resolute Reinforcements (reprint)
const RESOLUTE_REINFORCEMENTS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dmu::RESOLUTE_REINFORCEMENTS,
    "940f3989-77cc-49a9-92e0-095a75d80f0f",
    "Billy Christian",
);

// FDN 146 — Savannah Lions (reprint)
const SAVANNAH_LIONS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SAVANNAH_LIONS,
    "9c9ac1bc-cdf3-4fa6-8319-a7ea164e9e47",
    "Winona Nelson",
);

// FDN 147 — Serra Angel (reprint)
const SERRA_ANGEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SERRA_ANGEL,
    "3cee9303-9d65-45a2-93d4-ef4aba59141b",
    "Greg Staples",
);

// FDN 148 — Stroke of Midnight (reprint)
const STROKE_OF_MIDNIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_woe::STROKE_OF_MIDNIGHT,
    "ab135925-d924-456d-851a-6ccdaaf27271",
    "Julia Metzger",
);

// FDN 149 — Youthful Valkyrie (reprint)
const YOUTHFUL_VALKYRIE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_khm::YOUTHFUL_VALKYRIE,
    "9d795f79-c3a5-4ea1-a5cf-1ce73d6837b6",
    "Anna Steinbauer",
);

// FDN 150 — Aegis Turtle (reprint)
const AEGIS_TURTLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_iko::AEGIS_TURTLE,
    "c7f2014a-fbc9-447c-a440-e06d01066bb9",
    "Milivoj Ćeran",
);

// FDN 151 — Aetherize (reprint)
const AETHERIZE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_gtc::AETHERIZE,
    "1e5530fc-0291-4a17-b048-c5d24e6f51d8",
    "Alexandre Honoré",
);

// FDN 152 — Brineborn Cutthroat (reprint)
const BRINEBORN_CUTTHROAT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m20::BRINEBORN_CUTTHROAT,
    "acf7aafb-931f-49e5-8691-eab8cb34b05e",
    "Caio Monteiro",
);

// FDN 153 — Essence Scatter (reprint)
const ESSENCE_SCATTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m10::ESSENCE_SCATTER,
    "dd05c850-f91e-4ffb-b4cc-8418d49dad90",
    "Josh Hass",
);

// FDN 154 — Extravagant Replication (reprint)
const EXTRAVAGANT_REPLICATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ncc::EXTRAVAGANT_REPLICATION,
    "6a41dfae-bc7e-4105-8f7e-fd0109197ad8",
    "Pauline Voss",
);

// FDN 155 — Fleeting Distraction (reprint)
const FLEETING_DISTRACTION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_roe::FLEETING_DISTRACTION,
    "c0b86a7b-4912-43a7-ab89-c3432385baa1",
    "Ryan Yee",
);

// FDN 156 — Imprisoned in the Moon (reprint)
const IMPRISONED_IN_THE_MOON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_emn::IMPRISONED_IN_THE_MOON,
    "ee28e147-6622-4399-a314-c14a5c912dd0",
    "Ryan Alexander Lee",
);

// FDN 157 — Lightshell Duo (reprint)
const LIGHTSHELL_DUO_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_blb::LIGHTSHELL_DUO,
    "bb75315c-ea8f-4eb0-899e-c73ef75fc396",
    "Mariah Tekulve",
);

// FDN 158 — Micromancer (reprint)
const MICROMANCER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dmu::MICROMANCER,
    "e6af54ea-b57a-4e50-8e46-1747cca14430",
    "Ernanda Souza",
);

// FDN 159 — Mocking Sprite (reprint)
const MOCKING_SPRITE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_woe::MOCKING_SPRITE,
    "f6792f63-b651-497d-8aa5-cddf4cedeca8",
    "Ben Hill",
);

// FDN 160 — An Offer You Can't Refuse (reprint)
const AN_OFFER_YOU_CAN_T_REFUSE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_snc::AN_OFFER_YOU_CAN_T_REFUSE,
    "a829747f-cf9b-4d81-ba66-9f0630ed4565",
    "Dallas Williams",
);

// FDN 161 — Omniscience (reprint)
const OMNISCIENCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m13::OMNISCIENCE,
    "d33d91d0-1506-45e4-9def-975bf901815e",
    "Jason Chan",
);

// FDN 162 — Run Away Together (reprint)
const RUN_AWAY_TOGETHER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_eld::RUN_AWAY_TOGETHER,
    "e598eb7b-10dc-49e6-ac60-2fefa987173e",
    "Filip Burburan",
);

// FDN 163 — Self-Reflection (reprint)
const SELF_REFLECTION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lci::SELF_REFLECTION,
    "e1e6abc9-25b2-4d51-b519-2525079eab51",
    "Henry Peters",
);

// FDN 164 — Spectral Sailor (reprint)
const SPECTRAL_SAILOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m20::SPECTRAL_SAILOR,
    "03a49535-c5f3-4a6f-b333-7ac7bffdc9ae",
    "Cristi Balanescu",
);

// FDN 165 — Think Twice (reprint)
const THINK_TWICE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tsp::THINK_TWICE,
    "d88faaa1-eb41-40f7-991c-5c06e1138f3d",
    "Anthony Francisco",
);

// FDN 166 — Time Stop (reprint)
const TIME_STOP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_chk::TIME_STOP,
    "d938603b-0f1e-44c4-b86e-38e15f4a0d27",
    "Scott M. Fischer",
);

// FDN 167 — Tolarian Terror (reprint)
const TOLARIAN_TERROR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dmu::TOLARIAN_TERROR,
    "2569d4f3-55ed-4f99-9592-34c7df0aab72",
    "Vincent Christiaens",
);

// FDN 168 — Witness Protection (reprint)
const WITNESS_PROTECTION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_snc::WITNESS_PROTECTION,
    "f231e981-0069-43ce-ac1c-c85ced613e93",
    "Dominik Mayer",
);

// FDN 169 — Bake into a Pie (reprint)
const BAKE_INTO_A_PIE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_eld::BAKE_INTO_A_PIE,
    "2ab0e660-86a3-4b92-82fa-77dcb5db947d",
    "Zoltan Boros",
);

// FDN 170 — Burglar Rat (reprint)
const BURGLAR_RAT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_grn::BURGLAR_RAT,
    "de1c8758-ce3d-49cf-8173-c0eb46f5e7bc",
    "Tyler Walpole",
);

// FDN 171 — Diregraf Ghoul (reprint)
const DIREGRAF_GHOUL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_isd::DIREGRAF_GHOUL,
    "4682012c-d7e0-4257-b538-3de497507464",
    "Dave Kendall",
);

// FDN 172 — Eaten Alive (reprint)
const EATEN_ALIVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mid::EATEN_ALIVE,
    "1c4f7b20-b2a8-498c-8c36-dc296863b0b9",
    "Nicholas Gregory",
);

// FDN 173 — Exsanguinate (reprint)
const EXSANGUINATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_som::EXSANGUINATE,
    "f11d7311-4066-4a5d-ba28-9857fa707a0b",
    "Marie Magny",
);

// FDN 174 — Fake Your Own Death (reprint)
const FAKE_YOUR_OWN_DEATH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_snc::FAKE_YOUR_OWN_DEATH,
    "693635a6-df50-44c5-9598-0c79b45d4df4",
    "Monztre",
);

// FDN 175 — Hero's Downfall (reprint)
const HERO_S_DOWNFALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ths::HERO_S_DOWNFALL,
    "ad2c01d9-8f54-46c0-9dc9-d4d4764ce1c9",
    "Chris Rallis",
);

// FDN 176 — Liliana, Dreadhorde General (reprint)
const LILIANA_DREADHORDE_GENERAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_war::LILIANA_DREADHORDE_GENERAL,
    "cb28d217-795e-4320-a032-cd713f7ecc8a",
    "Chris Rallis",
);

// FDN 177 — Macabre Waltz (reprint)
const MACABRE_WALTZ_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dis::MACABRE_WALTZ,
    "4d1f3c84-89ba-4426-a80b-d524f172c912",
    "Willian Murai",
);

// FDN 178 — Marauding Blight-Priest (reprint)
const MARAUDING_BLIGHT_PRIEST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_znr::MARAUDING_BLIGHT_PRIEST,
    "5f70dafc-c638-4ec0-ab5b-62998f752720",
    "Caio Monteiro",
);

// FDN 179 — Painful Quandary (reprint)
const PAINFUL_QUANDARY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_som::PAINFUL_QUANDARY,
    "05757669-e8a6-4a2f-8479-d4e2ade822ca",
    "David Palumbo",
);

// FDN 180 — Phyrexian Arena (reprint)
const PHYREXIAN_ARENA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_apc::PHYREXIAN_ARENA,
    "0784b6f0-9ebf-43d2-ba0f-a6bc93ba0c48",
    "Svetlin Velinov",
);

// FDN 181 — Pilfer (reprint)
const PILFER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dmu::PILFER,
    "8c7c88b5-6d09-453b-b9c1-7dcbba8f1080",
    "Pauline Voss",
);

// FDN 182 — Reassembling Skeleton (reprint)
const REASSEMBLING_SKELETON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arc::REASSEMBLING_SKELETON,
    "28e84b1b-1c05-4e1b-93b8-9cc2ca73509d",
    "Austin Hsu",
);

// FDN 183 — Rise of the Dark Realms (reprint)
const RISE_OF_THE_DARK_REALMS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m14::RISE_OF_THE_DARK_REALMS,
    "8645bf0c-631f-4003-bd24-3e069ae23513",
    "Michael Komarck",
);

// FDN 184 — Rune-Scarred Demon (reprint)
const RUNE_SCARRED_DEMON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m12::RUNE_SCARRED_DEMON,
    "1eae3165-554d-4759-8f18-794e2a7d8464",
    "Michael Komarck",
);

// FDN 185 — Stromkirk Bloodthief (reprint)
const STROMKIRK_BLOODTHIEF_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mid::STROMKIRK_BLOODTHIEF,
    "485d6a5a-2054-47d5-91b8-71ce308ed4dc",
    "Caroline Gariba",
);

// FDN 186 — Vampire Nighthawk (reprint)
const VAMPIRE_NIGHTHAWK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_zen::VAMPIRE_NIGHTHAWK,
    "0a1934ab-3171-4fc6-8033-ad998899ba73",
    "Jason Chan",
);

// FDN 187 — Zombify (reprint)
const ZOMBIFY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ody::ZOMBIFY,
    "dc798e6f-13c4-457c-b052-b7b65bc83cfe",
    "Jason A. Engle",
);

// FDN 188 — Abrade (reprint)
const ABRADE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hou::ABRADE,
    "548947dc-a5ca-43b5-9531-bcef20fa4ae5",
    "Jonas De Ro",
);

// FDN 189 — Axgard Cavalry (reprint)
const AXGARD_CAVALRY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_khm::AXGARD_CAVALRY,
    "fe3cc41a-adae-4c9b-b4d3-03f3ca862fed",
    "Evyn Fong",
);

// FDN 190 — Brass's Bounty (reprint)
const BRASS_S_BOUNTY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rix::BRASS_S_BOUNTY,
    "65fe7127-b0ec-400f-97f1-6e17ab8e319d",
    "Grzegorz Rutkowski",
);

// FDN 191 — Brazen Scourge (reprint)
const BRAZEN_SCOURGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_kld::BRAZEN_SCOURGE,
    "eb84b86c-3276-4fc1-a09d-47de388cb729",
    "Kev Walker",
);

// FDN 192 — Burst Lightning (reprint)
const BURST_LIGHTNING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_zen::BURST_LIGHTNING,
    "aec5d380-d354-4750-931a-6c91853e2edc",
    "Vance Kovacs",
);

// FDN 193 — Drakuseth, Maw of Flames (reprint)
const DRAKUSETH_MAW_OF_FLAMES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m20::DRAKUSETH_MAW_OF_FLAMES,
    "029b1edb-e1de-4f1c-81df-8d17f4920318",
    "Grzegorz Rutkowski",
);

// FDN 194 — Etali, Primal Storm (reprint)
const ETALI_PRIMAL_STORM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rix::ETALI_PRIMAL_STORM,
    "b6af9894-95b5-4c8e-902f-a9ba70f02e4a",
    "Raymond Swanland",
);

// FDN 195 — Fanatical Firebrand (reprint)
const FANATICAL_FIREBRAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2018::rivals_of_ixalan::FANATICAL_FIREBRAND,
    "d1296316-7781-4e98-95e6-7020648be6a5",
    "Wayne Reynolds",
);

// FDN 196 — Firebrand Archer (reprint)
const FIREBRAND_ARCHER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hou::FIREBRAND_ARCHER,
    "fe0312f1-4c98-4b7f-8a34-0059ea80edef",
    "John Stanko",
);

// FDN 197 — Firespitter Whelp (reprint)
const FIRESPITTER_WHELP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_j25::FIRESPITTER_WHELP,
    "4b3a4c7d-3126-4bde-9dca-cb6a1e2f37c9",
    "David Álvarez",
);

// FDN 198 — Flamewake Phoenix (reprint)
const FLAMEWAKE_PHOENIX_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_frf::FLAMEWAKE_PHOENIX,
    "a94f008e-48a0-406b-83fa-99cd396831f8",
    "Min Yum",
);

// FDN 199 — Frenzied Goblin (reprint)
const FRENZIED_GOBLIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rav::FRENZIED_GOBLIN,
    "d5592573-2889-40b1-b1d5-c2802482549a",
    "Randy Vargas",
);

// FDN 200 — Goblin Surprise
pub(in crate::card::sets) static GOBLIN_SURPRISE: CardRecord = CardRecord::new(
    "Goblin Surprise",
    "527dd5d4-5f72-40bb-8a9d-1f5ac3f81e2e",
    "Kevin Sidharta",
    // Held up as a combat trick either way: the tokens are the mode you
    // take when the attack did not happen.
    CardRules::new_instant(mana_cost!("{2}{R}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
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
            AbilityDef::spell(
                "Create two 1/1 red Goblin creature tokens.",
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(TokenCharacteristics::creature(
                        &["Goblin"],
                        &[ManaColor::Red],
                        1,
                        1,
                    )))
                    .with_amount(2),
                ),
            ),
        ],
    )),
);

// FDN 201 — Heartfire Immolator (reprint)
const HEARTFIRE_IMMOLATOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m21::HEARTFIRE_IMMOLATOR,
    "3ca38f4d-01f5-4a02-9000-01261a440dbf",
    "Donato Giancola",
);

// FDN 202 — Hidetsugu's Second Rite (reprint)
const HIDETSUGU_S_SECOND_RITE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_sok::HIDETSUGU_S_SECOND_RITE,
    "609421da-8d89-4365-b18b-778832d91482",
    "Ben Hill",
);

// FDN 203 — Involuntary Employment (reprint)
const INVOLUNTARY_EMPLOYMENT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_snc::INVOLUNTARY_EMPLOYMENT,
    "f3ad3d62-2f24-4562-b3fa-809213dbc4a4",
    "Milivoj Ćeran",
);

// FDN 204 — Krenko, Mob Boss (reprint)
const KRENKO_MOB_BOSS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m13::KRENKO_MOB_BOSS,
    "824b2d73-2151-4e5e-9f05-8f63e2bdcaa9",
    "Lie Setiawan",
);

// FDN 205 — Seismic Rupture (reprint)
const SEISMIC_RUPTURE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dtk::SEISMIC_RUPTURE,
    "2519a51a-26a0-4884-9ba8-9db135c9ee49",
    "Jason A. Engle",
);

// FDN 206 — Shivan Dragon (reprint)
const SHIVAN_DRAGON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SHIVAN_DRAGON,
    "1fcff1e0-2745-448d-a27b-e31719e222e9",
    "Donato Giancola",
);

// FDN 207 — Slagstorm (reprint)
const SLAGSTORM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mbs::SLAGSTORM,
    "9db2e3a9-b90d-44cd-a2bd-eeb7dbe255b0",
    "Dan Murayama Scott",
);

// FDN 208 — Spitfire Lagac (reprint)
const SPITFIRE_LAGAC_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_znr::SPITFIRE_LAGAC,
    "30f600cd-b696-4f49-9cbc-5a33aa43d04c",
    "Antonio José Manzanedo",
);

// FDN 209 — Sure Strike (reprint)
const SURE_STRIKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_bfz::SURE_STRIKE,
    "5de6a1e4-5c66-43e6-9f2a-2635bdab03f6",
    "Izzy",
);

// FDN 210 — Thrill of Possibility (reprint)
const THRILL_OF_POSSIBILITY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_eld::THRILL_OF_POSSIBILITY,
    "882b348c-076b-41d8-b505-063480636669",
    "Steve Argyle",
);

// FDN 211 — Affectionate Indrik (reprint)
const AFFECTIONATE_INDRIK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_grn::AFFECTIONATE_INDRIK,
    "2da8347d-06a4-46e0-a55e-cc2da4660263",
    "Steve Prescott",
);

// FDN 212 — Bite Down (reprint)
const BITE_DOWN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dmu::BITE_DOWN,
    "f8d70b3b-f6f9-4b3c-ad70-0ce369e812b5",
    "Kitt Lapeña",
);

// FDN 213 — Blanchwood Armor (reprint)
const BLANCHWOOD_ARMOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::BLANCHWOOD_ARMOR,
    "1fd7ec1a-dafa-42ca-bc25-f6848fb03f60",
    "Manuel Castañón",
);

// FDN 214 — Broken Wings (reprint)
const BROKEN_WINGS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_znr::BROKEN_WINGS,
    "61f9cbeb-cc9c-4562-be65-8a77053faefe",
    "Svetlin Velinov",
);

// FDN 215 — Bushwhack (reprint)
const BUSHWHACK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_bro::BUSHWHACK,
    "03ebdb36-55e0-49dd-a514-785fbeb4ae19",
    "Artur Nakhodkin",
);

// FDN 216 — Doubling Season (reprint)
const DOUBLING_SEASON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rav::DOUBLING_SEASON,
    "f2c4f80e-84a0-463b-82c3-5c6503809351",
    "Chuck Lukacs",
);

// FDN 217 — Dwynen, Gilt-Leaf Daen (reprint)
const DWYNEN_GILT_LEAF_DAEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ori::DWYNEN_GILT_LEAF_DAEN,
    "01c00d7b-7fac-4f8c-a1ea-de2cf4d06627",
    "Johannes Voss",
);

// FDN 218 — Dwynen's Elite (reprint)
const DWYNEN_S_ELITE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ori::DWYNEN_S_ELITE,
    "89d94c28-ea2e-4a3d-935f-6b2d9f2efc7a",
    "Volkan Baǵa",
);

// FDN 219 — Elvish Archdruid (reprint)
const ELVISH_ARCHDRUID_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m10::ELVISH_ARCHDRUID,
    "341da856-7414-403b-b2e3-4bebd58a5aa4",
    "Karl Kopinski",
);

// FDN 220 — Garruk's Uprising (reprint)
const GARRUK_S_UPRISING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m21::GARRUK_S_UPRISING,
    "4805c303-e73b-443b-a09f-49d2c2c88bb5",
    "Wisnu Tan",
);

// FDN 221 — Genesis Wave (reprint)
const GENESIS_WAVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_som::GENESIS_WAVE,
    "d46f7ddb-f986-4f1f-b096-ae1a02d0bdc8",
    "Arif Wijaya",
);

// FDN 222 — Ghalta, Primal Hunger (reprint)
const GHALTA_PRIMAL_HUNGER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rix::GHALTA_PRIMAL_HUNGER,
    "6a9c39e4-a8cf-42dd-8d0e-45634b335546",
    "Chase Stone",
);

// FDN 223 — Giant Growth (reprint)
const GIANT_GROWTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GIANT_GROWTH,
    "bd0bf74e-14c1-4428-88d8-2181a080b5d0",
    "Matt Cavotta",
);

// FDN 224 — Gnarlid Colony (reprint)
const GNARLID_COLONY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_znr::GNARLID_COLONY,
    "47565d10-96bf-4fb0-820f-f20a44a76b6f",
    "Izzy",
);

// FDN 225 — Grow from the Ashes (reprint)
const GROW_FROM_THE_ASHES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dom::GROW_FROM_THE_ASHES,
    "42525f8a-aee7-4811-8f05-471b559c2c4a",
    "Richard Wright",
);

// FDN 226 — Inspiring Call (reprint)
const INSPIRING_CALL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dtk::INSPIRING_CALL,
    "3e241642-5172-4437-b694-f6aa159d5cd9",
    "Dan Murayama Scott",
);

// FDN 227 — Llanowar Elves (reprint)
const LLANOWAR_ELVES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LLANOWAR_ELVES,
    "6a0b230b-d391-4998-a3f7-7b158a0ec2cd",
    "Kev Walker",
);

// FDN 228 — Mild-Mannered Librarian (reprint)
const MILD_MANNERED_LIBRARIAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_j22::MILD_MANNERED_LIBRARIAN,
    "5389663a-fe25-41b9-8c92-1f4d7721ffc2",
    "Justyna Dura",
);

// FDN 229 — Nessian Hornbeetle (reprint)
const NESSIAN_HORNBEETLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_thb::NESSIAN_HORNBEETLE,
    "3d4d93de-85c6-4653-8ddd-d8bf21516d44",
    "Jason Felix",
);

// FDN 230 — Overrun (reprint)
const OVERRUN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tmp::OVERRUN,
    "1d8e9cbb-8bf4-4a48-a58e-79deb3abdf7f",
    "Carl Critchlow",
);

// FDN 231 — Reclamation Sage (reprint)
const RECLAMATION_SAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m15::RECLAMATION_SAGE,
    "1918ea65-ab7f-4d40-97fd-a656c892a2a1",
    "Christopher Moeller",
);

// FDN 232 — Scavenging Ooze (reprint)
const SCAVENGING_OOZE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_cmd::SCAVENGING_OOZE,
    "8c504c23-1e9a-411b-9cfe-4180d0c744f6",
    "Austin Hsu",
);

// FDN 233 — Snakeskin Veil (reprint)
const SNAKESKIN_VEIL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_khm::SNAKESKIN_VEIL,
    "6cc4c21d-9bdc-4490-9203-17f51db0ddd1",
    "Dan Murayama Scott",
);

// FDN 234 — Vivien Reid (reprint)
const VIVIEN_REID_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m19::VIVIEN_REID,
    "38769247-42a1-4571-9071-6d59fe28650a",
    "Anna Steinbauer",
);

// FDN 235 — Wary Thespian (reprint)
const WARY_THESPIAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mom::WARY_THESPIAN,
    "a3d62d04-0974-4cb5-9a35-5e996c6456e2",
    "Billy Christian",
);

// FDN 236 — Wildwood Scourge (reprint)
const WILDWOOD_SCOURGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m21::WILDWOOD_SCOURGE,
    "36359fb6-fb8c-4382-8555-e348422f116c",
    "Bryan Sola",
);

// FDN 237 — Balmor, Battlemage Captain (reprint)
const BALMOR_BATTLEMAGE_CAPTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dmu::BALMOR_BATTLEMAGE_CAPTAIN,
    "0b45ab13-9bb6-48af-8b37-d97b25801ac8",
    "Bram Sels",
);

// FDN 238 — Consuming Aberration (reprint)
const CONSUMING_ABERRATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_gtc::CONSUMING_ABERRATION,
    "bc2b28fd-66b0-457c-80ea-7caed2cc7926",
    "Karl Kopinski",
);

// FDN 239 — Empyrean Eagle (reprint)
const EMPYREAN_EAGLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m20::EMPYREAN_EAGLE,
    "577e99a7-4a55-4314-8f08-2ae0c33b85c7",
    "Jason A. Engle",
);

// FDN 240 — Good-Fortune Unicorn (reprint)
const GOOD_FORTUNE_UNICORN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mh1::GOOD_FORTUNE_UNICORN,
    "eabbe163-2b15-42e3-89ce-7363e6250d3a",
    "Kee Lo",
);

// FDN 241 — Heroic Reinforcements (reprint)
const HEROIC_REINFORCEMENTS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m19::HEROIC_REINFORCEMENTS,
    "6a05e8d5-c2ad-489a-888d-22622886b620",
    "Scott Murphy",
);

// FDN 242 — Lathril, Blade of the Elves (reprint)
const LATHRIL_BLADE_OF_THE_ELVES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_khc::LATHRIL_BLADE_OF_THE_ELVES,
    "8d4e5480-a287-4a25-b855-a26dae555b1c",
    "Caroline Gariba",
);

// FDN 243 — Muldrotha, the Gravetide (reprint)
const MULDROTHA_THE_GRAVETIDE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dom::MULDROTHA_THE_GRAVETIDE,
    "51710b19-68e3-4853-901f-e618bde61161",
    "Jason Rainville",
);

// FDN 244 — Progenitus (reprint)
const PROGENITUS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_con::PROGENITUS,
    "e77fbc87-d78e-4602-baa0-da9b0d464dfb",
    "Jaime Jones",
);

// FDN 245 — Ruby, Daring Tracker (reprint)
const RUBY_DARING_TRACKER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_woe::RUBY_DARING_TRACKER,
    "fe3e7dd2-b66d-4218-9fde-f84bec26b7bf",
    "Ekaterina Burmak",
);

// FDN 246 — Swiftblade Vindicator (reprint)
const SWIFTBLADE_VINDICATOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_grn::SWIFTBLADE_VINDICATOR,
    "f94618ec-000c-4371-b925-05ff82bfe221",
    "Viktor Titov",
);

// FDN 247 — Tatyova, Benthic Druid (reprint)
const TATYOVA_BENTHIC_DRUID_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dom::TATYOVA_BENTHIC_DRUID,
    "eabc978a-0666-472d-bdc6-d4b29d29eca4",
    "Mathias Kollros",
);

// FDN 248 — Thousand-Year Storm (reprint)
const THOUSAND_YEAR_STORM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_grn::THOUSAND_YEAR_STORM,
    "76c48a67-1410-40f1-9b93-0172d85e4688",
    "Dimitar Marinski",
);

// FDN 249 — Adventuring Gear (reprint)
const ADVENTURING_GEAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_zen::ADVENTURING_GEAR,
    "361f9b99-5b5d-40da-b4b9-5ad90f6280ee",
    "Howard Lyon",
);

// FDN 250 — Burnished Hart (reprint)
const BURNISHED_HART_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ths::BURNISHED_HART,
    "65ebbff0-fbe6-4310-a33f-e00bb2534979",
    "Yeong-Hao Han",
);

// FDN 251 — Campus Guide (reprint)
const CAMPUS_GUIDE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_stx::CAMPUS_GUIDE,
    "43c59814-3167-4b05-bb85-6c736f3956a4",
    "Slawomir Maniak",
);

// FDN 252 — Gleaming Barrier (reprint)
const GLEAMING_BARRIER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rix::GLEAMING_BARRIER,
    "1b49b009-e6f2-494a-9235-f5c25c2d70a9",
    "Jason Felix",
);

// FDN 253 — Goldvein Pick (reprint)
const GOLDVEIN_PICK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_khm::GOLDVEIN_PICK,
    "a241317d-2277-467e-a8f9-aa71c944e244",
    "Dan Murayama Scott",
);

// FDN 254 — Heraldic Banner (reprint)
const HERALDIC_BANNER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_eld::HERALDIC_BANNER,
    "743ea709-dbb3-4db8-a2ce-544f47eb6339",
    "Ravenna Tran",
);

// FDN 255 — Juggernaut (reprint)
const JUGGERNAUT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::JUGGERNAUT,
    "f4468fff-cd6f-428c-b7a0-ff89f5bbea2e",
    "Kev Walker",
);

// FDN 256 — Meteor Golem (reprint)
const METEOR_GOLEM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m19::METEOR_GOLEM,
    "d291ea1e-36bc-46b3-b3ae-084fa0ba69eb",
    "Lake Hurwitz",
);

// FDN 257 — Solemn Simulacrum (reprint)
const SOLEMN_SIMULACRUM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mrd::SOLEMN_SIMULACRUM,
    "5383f45e-3da2-40fb-beee-801448bbb60f",
    "Dan Murayama Scott",
);

// FDN 258 — Swiftfoot Boots (reprint)
const SWIFTFOOT_BOOTS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m12::SWIFTFOOT_BOOTS,
    "41040541-b129-4cf4-9411-09b1d9d32c19",
    "Svetlin Velinov",
);

// FDN 259 — Bloodfell Caves (reprint)
const BLOODFELL_CAVES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::BLOODFELL_CAVES,
    "8b90dc92-cb66-41d9-89f9-2b6e3cfc8082",
    "Adam Paquette",
);

// FDN 260 — Blossoming Sands (reprint)
const BLOSSOMING_SANDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::BLOSSOMING_SANDS,
    "37676ed8-588c-4bca-8065-874b74d84807",
    "Sam Burley",
);

// FDN 261 — Dismal Backwater (reprint)
const DISMAL_BACKWATER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::DISMAL_BACKWATER,
    "dbb0df36-8467-4a41-8e1c-6c3584d4fd10",
    "Sam Burley",
);

// FDN 262 — Evolving Wilds (reprint)
const EVOLVING_WILDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_roe::EVOLVING_WILDS,
    "3a0b9356-5b91-4542-8802-f0f7275238e1",
    "Sam Burley",
);

// FDN 263 — Jungle Hollow (reprint)
const JUNGLE_HOLLOW_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::JUNGLE_HOLLOW,
    "dc758e14-d370-45e4-bbc5-938fb4d21127",
    "Eytan Zana",
);

// FDN 264 — Rogue's Passage (reprint)
const ROGUES_PASSAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rtr::ROGUES_PASSAGE,
    "a2a424ea-ef32-4ac5-8f8c-3ea1839f01d4",
    "Christine Choi",
);

// FDN 265 — Rugged Highlands (reprint)
const RUGGED_HIGHLANDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::RUGGED_HIGHLANDS,
    "fd6eaf8e-8881-4d7b-bafc-75e4ca5cbef6",
    "Eytan Zana",
);

// FDN 266 — Scoured Barrens (reprint)
const SCOURED_BARRENS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::SCOURED_BARRENS,
    "2632a4b2-9ca6-4b67-9a99-14f52ad3dc41",
    "Eytan Zana",
);

// FDN 267 — Secluded Courtyard (reprint)
const SECLUDED_COURTYARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_neo::SECLUDED_COURTYARD,
    "d13373d2-139b-48c7-a8c9-828cefc4f150",
    "Sam Burley",
);

// FDN 268 — Swiftwater Cliffs (reprint)
const SWIFTWATER_CLIFFS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::SWIFTWATER_CLIFFS,
    "fb88667d-7088-4889-960f-317486ebe856",
    "Adam Paquette",
);

// FDN 269 — Thornwood Falls (reprint)
const THORNWOOD_FALLS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::THORNWOOD_FALLS,
    "42799f51-0f8c-444b-974e-dae281a5c697",
    "Eytan Zana",
);

// FDN 270 — Tranquil Cove (reprint)
const TRANQUIL_COVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::TRANQUIL_COVE,
    "7c9cabca-5bcc-4b97-b2ac-a345ad3ee43c",
    "Jonas De Ro",
);

// FDN 271 — Wind-Scarred Crag (reprint)
const WIND_SCARRED_CRAG_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::WIND_SCARRED_CRAG,
    "759e99df-11a8-4aee-b6bc-344e84e10d94",
    "Jonas De Ro",
);

// FDN 272 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAINS,
    "4ef17ed4-a9b5-4b8e-b4cb-2ecb7e5898c3",
    "Rebecca Guay",
);

// FDN 273 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    1,
    "37edc4b5-75f5-4b43-a57e-a8192565a2a0",
    "Tingting Yeh",
);

// FDN 274 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND,
    "17e2b637-72b1-4457-aaba-66d51107be4c",
    "John Avon",
);

// FDN 275 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    1,
    "23635e40-d040-40b7-8b98-90ed362aa028",
    "Rebecca Guay",
);

// FDN 276 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWAMP,
    "319bc1f0-ee42-44e5-b08b-735613ded2ba",
    "Mike Bierek",
);

// FDN 277 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    1,
    "13505c15-14e0-4200-82bd-fb9bce949e68",
    "Rebecca Guay",
);

// FDN 278 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MOUNTAIN,
    "279df7e2-2a3b-464a-a7df-e91da28e3a8c",
    "Piotr Dura",
);

// FDN 279 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    1,
    "1edc5050-69bd-416d-b04c-7f82de2a1901",
    "Rebecca Guay",
);

// FDN 280 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOREST,
    "d232fcc2-12f6-401a-b1aa-ddff11cb9378",
    "Rebecca Guay",
);

// FDN 281 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    1,
    "ab8affbb-d2a2-436b-bbc2-9e8b6cf0d2c4",
    "Jim Nelson",
);

// FDN 282 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    2,
    "6e6f19b3-4c76-4078-8ed2-b2832a33d066",
    "Sam Burley",
);

// FDN 283 — Plains (alternate printing)
const PLAINS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    3,
    "7a0f9892-89cd-46ff-bc87-114e175cb575",
    "Julian Kok Joon Wen",
);

// FDN 284 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    2,
    "886eae6e-ced2-4d94-96fa-9bb5385b06f4",
    "Daniel Ljunggren",
);

// FDN 285 — Island (alternate printing)
const ISLAND_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    3,
    "6f534ddc-f610-45cd-84dc-bb6df6c5a84f",
    "Adam Paquette",
);

// FDN 286 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    2,
    "6f23a73a-522b-40cf-a14b-ffdf47a24c01",
    "Steven Belledin",
);

// FDN 287 — Swamp (alternate printing)
const SWAMP_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    3,
    "fda1dbfa-a57b-4aa8-9993-c8f97aec28bb",
    "Piotr Dura",
);

// FDN 288 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    2,
    "3a3afd00-da06-4a9f-8cd1-7133728e0fdd",
    "Dan Murayama Scott",
);

// FDN 289 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    3,
    "042b04b4-f7f4-4c1a-86ad-b50d788aa99e",
    "Salvatorre Zee Yazzie",
);

// FDN 290 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    2,
    "bbbeb57d-5fa0-4ff7-b5e8-caafc139669b",
    "Sam Burley",
);

// FDN 291 — Forest (alternate printing)
const FOREST_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    3,
    "117ab60a-b888-4585-b0c6-769d387069f7",
    "Piotr Dura",
);

// FDN 292 — Sire of Seven Deaths (alternate printing)
const SIRE_OF_SEVEN_DEATHS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SIRE_OF_SEVEN_DEATHS,
    1,
    "1b486b75-4680-4a94-af8c-c1c335c9782b",
    "Alexander Mokhov",
);

// FDN 293 — Ajani's Pridemate (alternate printing)
const AJANI_S_PRIDEMATE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_m11::AJANI_S_PRIDEMATE,
    1,
    "d4cfb9bc-4273-4e5f-a7ac-2006a8345a4e",
    "Chris Rallis",
);

// FDN 294 — Arahbo, the First Fang (alternate printing)
const ARAHBO_THE_FIRST_FANG_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ARAHBO_THE_FIRST_FANG,
    1,
    "813a39af-bafe-4a38-a270-39a0ce0f4aa5",
    "Chris Rahn",
);

// FDN 295 — Celestial Armor (alternate printing)
const CELESTIAL_ARMOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CELESTIAL_ARMOR,
    1,
    "0302a684-563c-49b2-9029-7f079ac58fd2",
    "José Parodi",
);

// FDN 296 — Crystal Barricade (alternate printing)
const CRYSTAL_BARRICADE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CRYSTAL_BARRICADE,
    1,
    "e3c62996-3dc9-4f1a-8979-790b9b6b134e",
    "Alayna Danner",
);

// FDN 297 — Exemplar of Light (alternate printing)
const EXEMPLAR_OF_LIGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EXEMPLAR_OF_LIGHT,
    1,
    "089b28fe-f10f-4e3e-8c19-ed012349faf4",
    "Chris Rahn",
);

// FDN 298 — Giada, Font of Hope (alternate printing)
const GIADA_FONT_OF_HOPE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_snc::GIADA_FONT_OF_HOPE,
    1,
    "0b235e9f-a8a6-45d7-b301-bc6db752dda8",
    "Scott M. Fischer",
);

// FDN 299 — Herald of Eternal Dawn (alternate printing)
const HERALD_OF_ETERNAL_DAWN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HERALD_OF_ETERNAL_DAWN,
    1,
    "4e5e421e-c187-4bf5-be9d-20a1e32b570b",
    "PINDURSKI",
);

// FDN 300 — Raise the Past (alternate printing)
const RAISE_THE_PAST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RAISE_THE_PAST,
    1,
    "eff36db9-560b-4cda-8122-d9ff748acf4d",
    "Jorge Jacinto",
);

// FDN 301 — Skyknight Squire (alternate printing)
const SKYKNIGHT_SQUIRE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SKYKNIGHT_SQUIRE,
    1,
    "e52d0d34-4d66-4e0d-9d64-bb7786930b7a",
    "Arif Wijaya",
);

// FDN 302 — Valkyrie's Call (alternate printing)
const VALKYRIE_S_CALL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VALKYRIE_S_CALL,
    1,
    "2c0f6b48-63e6-4eff-92db-9f2a4652c5c1",
    "Julie Dillon",
);

// FDN 303 — Youthful Valkyrie (alternate printing)
const YOUTHFUL_VALKYRIE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_khm::YOUTHFUL_VALKYRIE,
    1,
    "a8d2690b-8d73-468c-be25-576bc615069a",
    "Néstor Ossandón Leal",
);

// FDN 304 — Archmage of Runes (alternate printing)
const ARCHMAGE_OF_RUNES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ARCHMAGE_OF_RUNES,
    1,
    "15f15cc5-c3dc-4830-aa55-13080c738739",
    "Zoltan Boros",
);

// FDN 305 — Curator of Destinies (alternate printing)
const CURATOR_OF_DESTINIES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CURATOR_OF_DESTINIES,
    1,
    "abab9f07-ea0f-4f6f-81f4-a9177909c9e7",
    "Alayna Danner",
);

// FDN 306 — Drake Hatcher (alternate printing)
const DRAKE_HATCHER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DRAKE_HATCHER,
    1,
    "b60cf418-b927-49b6-9613-41eae6296902",
    "Zoltan Boros",
);

// FDN 307 — High Fae Trickster (alternate printing)
const HIGH_FAE_TRICKSTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HIGH_FAE_TRICKSTER,
    1,
    "a21180a4-208f-4c13-a704-58403ddaf12f",
    "Arif Wijaya",
);

// FDN 308 — Homunculus Horde (alternate printing)
const HOMUNCULUS_HORDE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HOMUNCULUS_HORDE,
    1,
    "18dbb6ff-1262-44cc-8d36-153b2d3eace0",
    "Filipe Pagliuso",
);

// FDN 309 — Kiora, the Rising Tide (alternate printing)
const KIORA_THE_RISING_TIDE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KIORA_THE_RISING_TIDE,
    1,
    "5a123794-096f-4d01-bfd4-1d23f22608f7",
    "Magali Villeneuve",
);

// FDN 310 — Lunar Insight (alternate printing)
const LUNAR_INSIGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LUNAR_INSIGHT,
    1,
    "0a80f2eb-e34a-4f39-a831-d6fb42f6b4cc",
    "Julie Dillon",
);

// FDN 311 — An Offer You Can't Refuse (alternate printing)
const AN_OFFER_YOU_CAN_T_REFUSE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_snc::AN_OFFER_YOU_CAN_T_REFUSE,
    1,
    "6f6aaee9-8c44-4e23-8167-ead64e599711",
    "Aaron J. Riley",
);

// FDN 312 — Omniscience (alternate printing)
const OMNISCIENCE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_m13::OMNISCIENCE,
    1,
    "7c72dced-adae-4c66-af2a-0a1216953ab6",
    "Dominik Mayer",
);

// FDN 313 — Refute (alternate printing)
const REFUTE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &REFUTE,
    1,
    "26a65d5b-d07e-4ff0-900a-30d509ce0f35",
    "Monztre",
);

// FDN 314 — Sphinx of Forgotten Lore (alternate printing)
const SPHINX_OF_FORGOTTEN_LORE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPHINX_OF_FORGOTTEN_LORE,
    1,
    "9571a123-37b4-42fa-96ba-42580afb6d9d",
    "Chuck Lukacs",
);

// FDN 315 — Think Twice (alternate printing)
const THINK_TWICE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_tsp::THINK_TWICE,
    1,
    "40a7e9bb-0772-4539-bbf9-7a95d5e47947",
    "Alix Branwyn",
);

// FDN 316 — Abyssal Harvester (alternate printing)
const ABYSSAL_HARVESTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ABYSSAL_HARVESTER,
    1,
    "7e44b856-1803-4e63-ad81-43a1c4ef5020",
    "Aaron J. Riley",
);

// FDN 317 — Blasphemous Edict (alternate printing)
const BLASPHEMOUS_EDICT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BLASPHEMOUS_EDICT,
    1,
    "56a669cb-24fc-4055-9e53-70b794805f3b",
    "Dmitry Burmak",
);

// FDN 318 — Bloodthirsty Conqueror (alternate printing)
const BLOODTHIRSTY_CONQUEROR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BLOODTHIRSTY_CONQUEROR,
    1,
    "3c340ba8-9287-4072-937a-438251b5ff1d",
    "Alexander Mokhov",
);

// FDN 319 — Hero's Downfall (alternate printing)
const HERO_S_DOWNFALL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ths::HERO_S_DOWNFALL,
    1,
    "10cedc6d-075a-4f9b-a858-e2c29809ee33",
    "Zoltan Boros",
);

// FDN 320 — High-Society Hunter (alternate printing)
const HIGH_SOCIETY_HUNTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HIGH_SOCIETY_HUNTER,
    1,
    "f30c5bb1-58c6-40a4-9e87-3f71b026c523",
    "Aaron J. Riley",
);

// FDN 321 — Nine-Lives Familiar (alternate printing)
const NINE_LIVES_FAMILIAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &NINE_LIVES_FAMILIAR,
    1,
    "6cc1623f-370d-42b5-88a2-039f31e9be0b",
    "Xabi Gaztelua",
);

// FDN 322 — Phyrexian Arena (alternate printing)
const PHYREXIAN_ARENA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_apc::PHYREXIAN_ARENA,
    1,
    "f0ea5129-7e83-4fb5-8096-2f5ebdc3efe1",
    "Andrey Kuzinskiy",
);

// FDN 323 — Rise of the Dark Realms (alternate printing)
const RISE_OF_THE_DARK_REALMS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_m14::RISE_OF_THE_DARK_REALMS,
    1,
    "7d466d44-3203-4e9d-befc-877414a7308d",
    "Jorge Jacinto",
);

// FDN 324 — Tinybones, Bauble Burglar (alternate printing)
const TINYBONES_BAUBLE_BURGLAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TINYBONES_BAUBLE_BURGLAR,
    1,
    "daf8d5d4-b52d-42c7-aa56-c104a539133c",
    "Rudy Siswanto",
);

// FDN 325 — Vengeful Bloodwitch (alternate printing)
const VENGEFUL_BLOODWITCH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VENGEFUL_BLOODWITCH,
    1,
    "c944c5dc-9cb6-4dfd-aa90-5e180ffaa0fe",
    "Zezhou Chen",
);

// FDN 326 — Zul Ashur, Lich Lord (alternate printing)
const ZUL_ASHUR_LICH_LORD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ZUL_ASHUR_LICH_LORD,
    1,
    "1b1aad48-88f0-464d-9776-ecd023d87b8f",
    "PINDURSKI",
);

// FDN 327 — Abrade (alternate printing)
const ABRADE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_hou::ABRADE,
    1,
    "aa74ab7c-b9de-47ab-83ea-2b98738838c7",
    "Tyler Walpole",
);

// FDN 328 — Electroduplicate (alternate printing)
const ELECTRODUPLICATE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ELECTRODUPLICATE,
    1,
    "f3f3ab83-3b8e-4747-98ae-7b981bcc77b1",
    "Andrew Mar",
);

// FDN 329 — Etali, Primal Storm (alternate printing)
const ETALI_PRIMAL_STORM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_rix::ETALI_PRIMAL_STORM,
    1,
    "9f30943b-f739-49eb-bafb-fec6614a0c5e",
    "Raymond Swanland",
);

// FDN 330 — Kellan, Planar Trailblazer
pub(in crate::card::sets) static KELLAN_PLANAR_TRAILBLAZER: CardRecord = CardRecord::new(
    "Kellan, Planar Trailblazer",
    "0e413f37-b59a-4302-86d3-2abce81edc78",
    "Aaron J. Riley",
// One mana for a 2/1 that grows into what the rest of the turn's mana
    // has nothing better to do with.
    CardRules::new_creature(mana_cost!("{R}"), &["Human", "Faerie", "Scout"], 2, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated(
                "{1}{R}: If Kellan is a Scout, it becomes a Human Faerie Detective and gains \"Whenever \
                 Kellan deals combat damage to a player, exile the top card of your library. You may play \
                 that card this turn.\"",
                &[CostDef::Mana(mana_cost!("{1}{R}"))],
                EffectDef::IfCondition {
                    // Each activation asks what Kellan is now, so the two have to be paid in
                    // order and neither does anything twice.
                    condition: &TriggerConditionDef::SourceMatches {
                        object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Scout")),
                    },
                    then: &EffectDef::Apply {
                        recipient: EffectRecipientDef::Source,
                        // "It becomes a Human Faerie Detective": a set rather than an addition, so
                        // the Scout it was is gone and the second activation has something to ask
                        // about.
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::Characteristic(CharacteristicOperationDef::CreatureTypes(
                                SetOperationDef::Set(CreatureTypeSetDef::named(&["Human", "Faerie", "Detective"])),
                            )),
                            // The Detective's own clause, granted rather than printed: a card exiled
                            // off the top and playable for the turn, which is what the second
                            // activation is paying to turn on.
                            AppliedEffectDef::add_ability(&AbilityDef::triggered(
                                "Whenever Kellan deals combat damage to a player, exile the top card of your library. You may \
                                 play that card this turn.",
                                TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
                                EffectDef::ExileTopOfLibraryToPlay {
                                    player: EffectRecipientDef::Controller,
                                    amount: ValueDef::Constant(1),
                                    free: false,
                                    face_down: false,
                                    duration: ExilePlayDurationDef::ThisTurn,
                                    spend_any_color: false,
                                    play_condition: None,
                                    cast_only: false,
                                },
                            )),
                        ]),
                        duration: ResolvedEffectDurationDef::Permanent,
                    },
                },
            ),
            AbilityDef::activated(
                "{2}{R}: If Kellan is a Detective, it becomes a 3/2 Human Faerie Rogue and gains double \
                 strike.",
                &[CostDef::Mana(mana_cost!("{2}{R}"))],
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::SourceMatches {
                        object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Detective")),
                    },
                    then: &EffectDef::Apply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
                                PowerToughnessOperationDef::SetBase {
                                    power: ValueDef::Constant(3),
                                    toughness: ValueDef::Constant(2),
                                },
                            )),
                            AppliedEffectDef::Characteristic(CharacteristicOperationDef::CreatureTypes(
                                SetOperationDef::Set(CreatureTypeSetDef::named(&["Human", "Faerie", "Rogue"])),
                            )),
                            AppliedEffectDef::add_ability(&abilities::double_strike()),
                        ]),
                        duration: ResolvedEffectDurationDef::Permanent,
                    },
                },
            ),
        ]),
);

// FDN 331 — Rite of the Dragoncaller (alternate printing)
const RITE_OF_THE_DRAGONCALLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RITE_OF_THE_DRAGONCALLER,
    1,
    "1fa981d3-4a41-4bf1-ba64-2cd7224a6059",
    "Olivier Bernard",
);

// FDN 332 — Searslicer Goblin (alternate printing)
const SEARSLICER_GOBLIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SEARSLICER_GOBLIN,
    1,
    "58deba9d-5c95-4633-a86c-2637a8bb7ce2",
    "Arif Wijaya",
);

// FDN 333 — Twinflame Tyrant (alternate printing)
const TWINFLAME_TYRANT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TWINFLAME_TYRANT,
    1,
    "aab62a48-93b5-4eb3-aa6b-92dd4cc617f6",
    "Justin Gerard",
);

// FDN 334 — Genesis Wave (alternate printing)
const GENESIS_WAVE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_som::GENESIS_WAVE,
    1,
    "4b51445b-1590-4072-9712-5ca344b25e3e",
    "Liiga Smilshkalne",
);

// FDN 335 — Ghalta, Primal Hunger (alternate printing)
const GHALTA_PRIMAL_HUNGER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_rix::GHALTA_PRIMAL_HUNGER,
    1,
    "4561678b-db52-4b89-9f34-50fe600a55d7",
    "Justin Gerard",
);

// FDN 336 — Loot, Exuberant Explorer (alternate printing)
const LOOT_EXUBERANT_EXPLORER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LOOT_EXUBERANT_EXPLORER,
    1,
    "73ce9555-a687-4a37-864c-eb59b1e80a8f",
    "Rudy Siswanto",
);

// FDN 337 — Mossborn Hydra (alternate printing)
const MOSSBORN_HYDRA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MOSSBORN_HYDRA,
    1,
    "2be84175-062c-439f-abad-c57cd9eb490f",
    "Monztre",
);

// FDN 338 — Preposterous Proportions (alternate printing)
const PREPOSTEROUS_PROPORTIONS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PREPOSTEROUS_PROPORTIONS,
    1,
    "d611f8cd-fe7d-41d3-9572-9dda77d83d25",
    "Filipe Pagliuso",
);

// FDN 339 — Quilled Greatwurm (alternate printing)
const QUILLED_GREATWURM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &QUILLED_GREATWURM,
    1,
    "f00e8e36-bd52-4eca-ae48-f3e03a655704",
    "Maxime Minard",
);

// FDN 340 — Reclamation Sage (alternate printing)
const RECLAMATION_SAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_m15::RECLAMATION_SAGE,
    1,
    "af3f0764-9d80-4e15-a402-1f93e652bcbb",
    "Andrew Mar",
);

// FDN 341 — Spinner of Souls (alternate printing)
const SPINNER_OF_SOULS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPINNER_OF_SOULS,
    1,
    "984be4a8-8d34-4911-8210-0741f173bfab",
    "Xavier Ribeiro",
);

// FDN 342 — Sylvan Scavenging (alternate printing)
const SYLVAN_SCAVENGING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SYLVAN_SCAVENGING,
    1,
    "2bd7c1c1-e5e2-4481-b860-a4f7f7a1034b",
    "Quintin Gleim",
);

// FDN 343 — Alesha, Who Laughs at Fate (alternate printing)
const ALESHA_WHO_LAUGHS_AT_FATE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ALESHA_WHO_LAUGHS_AT_FATE,
    1,
    "c48858f3-40a4-4117-b4c5-e1b973be4869",
    "Dmitry Burmak",
);

// FDN 344 — Anthem of Champions (alternate printing)
const ANTHEM_OF_CHAMPIONS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ANTHEM_OF_CHAMPIONS,
    1,
    "7bf3fcd4-bac8-4cf1-b042-743cfc62e517",
    "Ryan Pancoast",
);

// FDN 345 — Ashroot Animist (alternate printing)
const ASHROOT_ANIMIST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ASHROOT_ANIMIST,
    1,
    "05b39d89-9902-4a45-a774-bea1c17ca5e4",
    "Lie Setiawan",
);

// FDN 346 — Elenda, Saint of Dusk (alternate printing)
const ELENDA_SAINT_OF_DUSK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ELENDA_SAINT_OF_DUSK,
    1,
    "14994b68-5930-49a4-af1c-e8123c8c8025",
    "Lie Setiawan",
);

// FDN 347 — Koma, World-Eater (alternate printing)
const KOMA_WORLD_EATER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KOMA_WORLD_EATER,
    1,
    "8889e1ca-eec1-408b-b11e-98cc0a357a97",
    "Néstor Ossandón Leal",
);

// FDN 348 — Kykar, Zephyr Awakener (alternate printing)
const KYKAR_ZEPHYR_AWAKENER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KYKAR_ZEPHYR_AWAKENER,
    1,
    "a1763648-802f-4178-a46a-a0c19142fd67",
    "Zezhou Chen",
);

// FDN 349 — Lathril, Blade of the Elves (alternate printing)
const LATHRIL_BLADE_OF_THE_ELVES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_khc::LATHRIL_BLADE_OF_THE_ELVES,
    1,
    "61f766b8-a92e-4353-b457-ac62fc470713",
    "Ekaterina Burmak",
);

// FDN 350 — Niv-Mizzet, Visionary (alternate printing)
const NIV_MIZZET_VISIONARY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &NIV_MIZZET_VISIONARY,
    1,
    "666749f4-3481-424b-9f18-37763ebf769a",
    "Raymond Swanland",
);

// FDN 351 — Zimone, Paradox Sculptor (alternate printing)
const ZIMONE_PARADOX_SCULPTOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ZIMONE_PARADOX_SCULPTOR,
    1,
    "c0490270-f94d-4f17-8a6c-527c9eab5d73",
    "Raluca Marinescu",
);

// FDN 352 — Banner of Kinship (alternate printing)
const BANNER_OF_KINSHIP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BANNER_OF_KINSHIP,
    1,
    "19a9b8b0-c1ba-48c3-8f90-6af6948274ee",
    "Chris Seaman",
);

// FDN 353 — Leyline Axe (alternate printing)
const LEYLINE_AXE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LEYLINE_AXE,
    1,
    "ae9df1f5-1b60-47b8-ac87-5c58719d7de4",
    "Julian Kok Joon Wen",
);

// FDN 354 — Scrawling Crawler (alternate printing)
const SCRAWLING_CRAWLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SCRAWLING_CRAWLER,
    1,
    "14d8c9b5-6863-45d1-89f0-6a4f8189758b",
    "Maxime Minard",
);

// FDN 355 — Swiftfoot Boots (alternate printing)
const SWIFTFOOT_BOOTS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_m12::SWIFTFOOT_BOOTS,
    1,
    "3e9b53da-4744-429d-97c6-f7ee4d568731",
    "Gabor Szikszai",
);

// FDN 356 — Soulstone Sanctuary (alternate printing)
const SOULSTONE_SANCTUARY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SOULSTONE_SANCTUARY,
    1,
    "98f4cc78-c25f-494c-b57e-c185d37605e8",
    "Jorge Jacinto",
);

// FDN 357 — Ajani, Caller of the Pride (alternate printing)
const AJANI_CALLER_OF_THE_PRIDE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_m13::AJANI_CALLER_OF_THE_PRIDE,
    1,
    "9f329a0b-13cc-48c5-9fb7-38d8a537aa30",
    "Victor Adame Minguez",
);

// FDN 358 — Kaito, Cunning Infiltrator (alternate printing)
const KAITO_CUNNING_INFILTRATOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KAITO_CUNNING_INFILTRATOR,
    1,
    "be447408-0846-49be-b47e-c6f256032deb",
    "Michal Ivan",
);

// FDN 359 — Liliana, Dreadhorde General (alternate printing)
const LILIANA_DREADHORDE_GENERAL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_war::LILIANA_DREADHORDE_GENERAL,
    1,
    "ba461127-2220-4274-81cb-423a1700c9eb",
    "Dmitry Burmak",
);

// FDN 360 — Chandra, Flameshaper (alternate printing)
const CHANDRA_FLAMESHAPER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CHANDRA_FLAMESHAPER,
    1,
    "02db5a2d-28d3-4f57-9045-f0ef16d8aad1",
    "Lie Setiawan",
);

// FDN 361 — Vivien Reid (alternate printing)
const VIVIEN_REID_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_m19::VIVIEN_REID,
    1,
    "37cc3e80-2ffa-4d48-bd67-a6315375b236",
    "Zara Alfonso",
);

// FDN 362 — Sire of Seven Deaths (alternate printing)
const SIRE_OF_SEVEN_DEATHS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SIRE_OF_SEVEN_DEATHS,
    2,
    "d5493f0f-6eec-4776-a5aa-661eef1389fa",
    "Alexander Mokhov",
);

// FDN 363 — Arahbo, the First Fang (alternate printing)
const ARAHBO_THE_FIRST_FANG_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ARAHBO_THE_FIRST_FANG,
    2,
    "2507fddd-2dc0-45f0-ac47-1ddd8690be46",
    "Chris Rahn",
);

// FDN 364 — Celestial Armor (alternate printing)
const CELESTIAL_ARMOR_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &CELESTIAL_ARMOR,
    2,
    "ea0787c7-338d-4db5-bfe8-70d578a422b9",
    "José Parodi",
);

// FDN 365 — Crystal Barricade (alternate printing)
const CRYSTAL_BARRICADE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &CRYSTAL_BARRICADE,
    2,
    "77f6acd2-3b91-4bd7-996f-42c75b88ee87",
    "Alayna Danner",
);

// FDN 366 — Exemplar of Light (alternate printing)
const EXEMPLAR_OF_LIGHT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &EXEMPLAR_OF_LIGHT,
    2,
    "f67b3e48-98e8-404b-93b0-c3cbbd764d1c",
    "Chris Rahn",
);

// FDN 367 — Giada, Font of Hope (alternate printing)
const GIADA_FONT_OF_HOPE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_snc::GIADA_FONT_OF_HOPE,
    2,
    "637ed447-aade-4fd8-8787-2330436c750f",
    "Scott M. Fischer",
);

// FDN 368 — Herald of Eternal Dawn (alternate printing)
const HERALD_OF_ETERNAL_DAWN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &HERALD_OF_ETERNAL_DAWN,
    2,
    "18cc6624-d288-4f99-9606-3ce914890530",
    "PINDURSKI",
);

// FDN 369 — Raise the Past (alternate printing)
const RAISE_THE_PAST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &RAISE_THE_PAST,
    2,
    "4542ac9d-74fe-4a38-ac54-e141f4923540",
    "Jorge Jacinto",
);

// FDN 370 — Skyknight Squire (alternate printing)
const SKYKNIGHT_SQUIRE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SKYKNIGHT_SQUIRE,
    2,
    "3ccc2ced-861f-40c3-90e9-49250e1874d9",
    "Arif Wijaya",
);

// FDN 371 — Valkyrie's Call (alternate printing)
const VALKYRIE_S_CALL_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &VALKYRIE_S_CALL,
    2,
    "7fbb6810-6f51-4e28-a4e7-40edcffc2059",
    "Julie Dillon",
);

// FDN 372 — Archmage of Runes (alternate printing)
const ARCHMAGE_OF_RUNES_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ARCHMAGE_OF_RUNES,
    2,
    "b6117837-83d4-4f84-967a-f5551515f0b9",
    "Zoltan Boros",
);

// FDN 373 — Curator of Destinies (alternate printing)
const CURATOR_OF_DESTINIES_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &CURATOR_OF_DESTINIES,
    2,
    "4aa7b1e9-1a11-41bf-8257-a8d3218b34fc",
    "Alayna Danner",
);

// FDN 374 — Drake Hatcher (alternate printing)
const DRAKE_HATCHER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DRAKE_HATCHER,
    2,
    "79ef01c5-c79e-4fc1-af17-0956dc156a28",
    "Zoltan Boros",
);

// FDN 375 — High Fae Trickster (alternate printing)
const HIGH_FAE_TRICKSTER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &HIGH_FAE_TRICKSTER,
    2,
    "257630d3-8f82-4cfc-b0fd-21399a2cde6c",
    "Arif Wijaya",
);

// FDN 376 — Homunculus Horde (alternate printing)
const HOMUNCULUS_HORDE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &HOMUNCULUS_HORDE,
    2,
    "6ae8f2d9-5759-49bf-b5d6-f2b9515ef919",
    "Filipe Pagliuso",
);

// FDN 377 — Kiora, the Rising Tide (alternate printing)
const KIORA_THE_RISING_TIDE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &KIORA_THE_RISING_TIDE,
    2,
    "18c7e2a5-9b56-48de-9cfd-be1ac6c692d9",
    "Magali Villeneuve",
);

// FDN 378 — Lunar Insight (alternate printing)
const LUNAR_INSIGHT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &LUNAR_INSIGHT,
    2,
    "605c9474-f9d1-4123-aafa-dd41950ea112",
    "Julie Dillon",
);

// FDN 379 — Omniscience (alternate printing)
const OMNISCIENCE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_m13::OMNISCIENCE,
    2,
    "8f173425-8bae-47b1-8a2c-e27a071acd24",
    "Dominik Mayer",
);

// FDN 380 — Sphinx of Forgotten Lore (alternate printing)
const SPHINX_OF_FORGOTTEN_LORE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SPHINX_OF_FORGOTTEN_LORE,
    2,
    "0552a2c4-36d4-405f-a56f-dc55610ae191",
    "Chuck Lukacs",
);

// FDN 381 — Abyssal Harvester (alternate printing)
const ABYSSAL_HARVESTER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ABYSSAL_HARVESTER,
    2,
    "a8aeaa6f-984a-4e8d-a934-547c95f0b482",
    "Aaron J. Riley",
);

// FDN 382 — Blasphemous Edict (alternate printing)
const BLASPHEMOUS_EDICT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BLASPHEMOUS_EDICT,
    2,
    "01e35e15-9456-46eb-b6be-5faab05185b2",
    "Dmitry Burmak",
);

// FDN 383 — Bloodthirsty Conqueror (alternate printing)
const BLOODTHIRSTY_CONQUEROR_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BLOODTHIRSTY_CONQUEROR,
    2,
    "9b10ba4e-4e6a-4127-a8a3-d9728e8d5f8b",
    "Alexander Mokhov",
);

// FDN 384 — High-Society Hunter (alternate printing)
const HIGH_SOCIETY_HUNTER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &HIGH_SOCIETY_HUNTER,
    2,
    "cafa089f-e33b-4933-a43d-e110dba29069",
    "Aaron J. Riley",
);

// FDN 385 — Nine-Lives Familiar (alternate printing)
const NINE_LIVES_FAMILIAR_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &NINE_LIVES_FAMILIAR,
    2,
    "54357fb3-f76b-411e-a466-c97643a8c0c7",
    "Xabi Gaztelua",
);

// FDN 386 — Phyrexian Arena (alternate printing)
const PHYREXIAN_ARENA_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_apc::PHYREXIAN_ARENA,
    2,
    "844f9f00-b33a-434d-8293-3a4c9e614358",
    "Andrey Kuzinskiy",
);

// FDN 387 — Rise of the Dark Realms (alternate printing)
const RISE_OF_THE_DARK_REALMS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_m14::RISE_OF_THE_DARK_REALMS,
    2,
    "afefa7d5-e83a-4bbe-a9eb-e20c1bc25cdd",
    "Jorge Jacinto",
);

// FDN 388 — Tinybones, Bauble Burglar (alternate printing)
const TINYBONES_BAUBLE_BURGLAR_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &TINYBONES_BAUBLE_BURGLAR,
    2,
    "25bdc11d-bef3-4823-b8bd-209cdac4ea31",
    "Rudy Siswanto",
);

// FDN 389 — Zul Ashur, Lich Lord (alternate printing)
const ZUL_ASHUR_LICH_LORD_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ZUL_ASHUR_LICH_LORD,
    2,
    "b8d94cc2-938d-4e98-a190-3a481b0a2966",
    "PINDURSKI",
);

// FDN 390 — Electroduplicate (alternate printing)
const ELECTRODUPLICATE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ELECTRODUPLICATE,
    2,
    "d47172bc-71c3-472a-b1bd-eff9b7c140e0",
    "Andrew Mar",
);

// FDN 391 — Etali, Primal Storm (alternate printing)
const ETALI_PRIMAL_STORM_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_rix::ETALI_PRIMAL_STORM,
    2,
    "28d02e4b-4da8-4304-bce3-b4c273462106",
    "Raymond Swanland",
);

// FDN 392 — Kellan, Planar Trailblazer (alternate printing)
const KELLAN_PLANAR_TRAILBLAZER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &KELLAN_PLANAR_TRAILBLAZER,
    2,
    "c7913c8f-a374-4d12-8ec9-8aee7604e135",
    "Aaron J. Riley",
);

// FDN 393 — Rite of the Dragoncaller (alternate printing)
const RITE_OF_THE_DRAGONCALLER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &RITE_OF_THE_DRAGONCALLER,
    2,
    "5a73750c-ddd4-448e-94f3-f64bada20508",
    "Olivier Bernard",
);

// FDN 394 — Searslicer Goblin (alternate printing)
const SEARSLICER_GOBLIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SEARSLICER_GOBLIN,
    2,
    "0523f8d8-4324-4e1d-9a88-4fd871e93bb8",
    "Arif Wijaya",
);

// FDN 395 — Twinflame Tyrant (alternate printing)
const TWINFLAME_TYRANT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &TWINFLAME_TYRANT,
    2,
    "fd09badf-b28d-48b9-8cb4-2336e5b1e4b9",
    "Justin Gerard",
);

// FDN 396 — Genesis Wave (alternate printing)
const GENESIS_WAVE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_som::GENESIS_WAVE,
    2,
    "5a4742df-edef-4b72-b523-429b1c284102",
    "Liiga Smilshkalne",
);

// FDN 397 — Ghalta, Primal Hunger (alternate printing)
const GHALTA_PRIMAL_HUNGER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_rix::GHALTA_PRIMAL_HUNGER,
    2,
    "3f89bc6f-fc43-49ad-881f-8897750e4b5f",
    "Justin Gerard",
);

// FDN 398 — Loot, Exuberant Explorer (alternate printing)
const LOOT_EXUBERANT_EXPLORER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &LOOT_EXUBERANT_EXPLORER,
    2,
    "9a25b231-f2cf-4915-b6e1-557142b4c7bb",
    "Rudy Siswanto",
);

// FDN 399 — Mossborn Hydra (alternate printing)
const MOSSBORN_HYDRA_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MOSSBORN_HYDRA,
    2,
    "b4fca15a-16bb-439a-9dfe-b5168918a4ed",
    "Monztre",
);

// FDN 400 — Preposterous Proportions (alternate printing)
const PREPOSTEROUS_PROPORTIONS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &PREPOSTEROUS_PROPORTIONS,
    2,
    "52bb6f1e-2545-4393-9985-383d11379977",
    "Filipe Pagliuso",
);

// FDN 401 — Quilled Greatwurm (alternate printing)
const QUILLED_GREATWURM_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &QUILLED_GREATWURM,
    2,
    "6224fdea-d899-4bb2-af29-ef6faef5a0ed",
    "Maxime Minard",
);

// FDN 402 — Spinner of Souls (alternate printing)
const SPINNER_OF_SOULS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SPINNER_OF_SOULS,
    2,
    "fce5bc0f-ddb2-4135-a2a8-e8b21648a711",
    "Xavier Ribeiro",
);

// FDN 403 — Sylvan Scavenging (alternate printing)
const SYLVAN_SCAVENGING_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SYLVAN_SCAVENGING,
    2,
    "2a60b8a2-3302-43ea-a5c9-89c86b9104ac",
    "Quintin Gleim",
);

// FDN 404 — Alesha, Who Laughs at Fate (alternate printing)
const ALESHA_WHO_LAUGHS_AT_FATE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ALESHA_WHO_LAUGHS_AT_FATE,
    2,
    "7ea65396-4873-4bb0-9781-ea74f2ba923b",
    "Dmitry Burmak",
);

// FDN 405 — Anthem of Champions (alternate printing)
const ANTHEM_OF_CHAMPIONS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ANTHEM_OF_CHAMPIONS,
    2,
    "4065802f-6194-49e9-94f5-880392519562",
    "Ryan Pancoast",
);

// FDN 406 — Ashroot Animist (alternate printing)
const ASHROOT_ANIMIST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ASHROOT_ANIMIST,
    2,
    "d1e3dcae-c1d5-46ba-8d0d-021968992d76",
    "Lie Setiawan",
);

// FDN 407 — Elenda, Saint of Dusk (alternate printing)
const ELENDA_SAINT_OF_DUSK_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ELENDA_SAINT_OF_DUSK,
    2,
    "4fcc24bc-1287-449e-8b8a-02b44f569610",
    "Lie Setiawan",
);

// FDN 408 — Koma, World-Eater (alternate printing)
const KOMA_WORLD_EATER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &KOMA_WORLD_EATER,
    2,
    "a338cc7f-1134-42b4-b384-a93a4f92edc9",
    "Néstor Ossandón Leal",
);

// FDN 409 — Kykar, Zephyr Awakener (alternate printing)
const KYKAR_ZEPHYR_AWAKENER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &KYKAR_ZEPHYR_AWAKENER,
    2,
    "1f8ec6f1-4889-46e2-b681-59bf7b581195",
    "Zezhou Chen",
);

// FDN 410 — Lathril, Blade of the Elves (alternate printing)
const LATHRIL_BLADE_OF_THE_ELVES_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_khc::LATHRIL_BLADE_OF_THE_ELVES,
    2,
    "c8463968-c2e9-48d7-ab9d-6a9b71470c3e",
    "Ekaterina Burmak",
);

// FDN 411 — Niv-Mizzet, Visionary (alternate printing)
const NIV_MIZZET_VISIONARY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &NIV_MIZZET_VISIONARY,
    2,
    "53c07579-1567-4e3f-a28a-ca7f106fd1dc",
    "Raymond Swanland",
);

// FDN 412 — Zimone, Paradox Sculptor (alternate printing)
const ZIMONE_PARADOX_SCULPTOR_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ZIMONE_PARADOX_SCULPTOR,
    2,
    "62ac2011-4021-46bc-b6cf-08de8db134aa",
    "Raluca Marinescu",
);

// FDN 413 — Banner of Kinship (alternate printing)
const BANNER_OF_KINSHIP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &BANNER_OF_KINSHIP,
    2,
    "1fa434d9-c0c2-47f2-85ba-d2258ad91f58",
    "Chris Seaman",
);

// FDN 414 — Leyline Axe (alternate printing)
const LEYLINE_AXE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &LEYLINE_AXE,
    2,
    "26d3f263-cabd-4a81-a68a-cb93cb1c2107",
    "Julian Kok Joon Wen",
);

// FDN 415 — Scrawling Crawler (alternate printing)
const SCRAWLING_CRAWLER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SCRAWLING_CRAWLER,
    2,
    "6c935438-2f3d-469d-a645-bfb3c95c3e0a",
    "Maxime Minard",
);

// FDN 416 — Soulstone Sanctuary (alternate printing)
const SOULSTONE_SANCTUARY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SOULSTONE_SANCTUARY,
    2,
    "ab27d7a8-de69-4939-a35b-760b1b42e070",
    "Jorge Jacinto",
);

// FDN 417 — Ajani, Caller of the Pride (alternate printing)
const AJANI_CALLER_OF_THE_PRIDE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_m13::AJANI_CALLER_OF_THE_PRIDE,
    2,
    "345f7610-5b95-4d24-9ab0-e18d27ccabed",
    "Victor Adame Minguez",
);

// FDN 418 — Kaito, Cunning Infiltrator (alternate printing)
const KAITO_CUNNING_INFILTRATOR_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &KAITO_CUNNING_INFILTRATOR,
    2,
    "6feaf235-ab3a-41f2-ae81-4f2d0b109ebc",
    "Michal Ivan",
);

// FDN 419 — Liliana, Dreadhorde General (alternate printing)
const LILIANA_DREADHORDE_GENERAL_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_war::LILIANA_DREADHORDE_GENERAL,
    2,
    "7a097413-b77c-45e8-a4ac-5b2dc1d34e16",
    "Dmitry Burmak",
);

// FDN 420 — Chandra, Flameshaper (alternate printing)
const CHANDRA_FLAMESHAPER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &CHANDRA_FLAMESHAPER,
    2,
    "cd73be04-3ec2-4ea3-8d93-47fa39aaeed8",
    "Lie Setiawan",
);

// FDN 421 — Vivien Reid (alternate printing)
const VIVIEN_REID_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_m19::VIVIEN_REID,
    2,
    "162a3a3b-257e-46c7-bb29-65627b00363d",
    "Zara Alfonso",
);

// FDN 422 — Day of Judgment (alternate printing)
const DAY_OF_JUDGMENT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_zen::DAY_OF_JUDGMENT,
    1,
    "66be885f-a319-4c17-b909-0467fe66cf3b",
    "Takayama Toshiaki",
);

// FDN 423 — Herald of Eternal Dawn (alternate printing)
const HERALD_OF_ETERNAL_DAWN_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &HERALD_OF_ETERNAL_DAWN,
    3,
    "fb6b336e-7471-48d8-ac21-66b2a01578d5",
    "Nijihayashi",
);

// FDN 424 — Kaito, Cunning Infiltrator (alternate printing)
const KAITO_CUNNING_INFILTRATOR_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &KAITO_CUNNING_INFILTRATOR,
    3,
    "a6a1cbdf-36aa-4619-932a-609c1699cc6a",
    "Ittoku",
);

// FDN 425 — Think Twice (alternate printing)
const THINK_TWICE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_tsp::THINK_TWICE,
    2,
    "6615cc95-0d7a-474d-bf35-3ea4dfc4cced",
    "HNCL",
);

// FDN 426 — Bloodthirsty Conqueror (alternate printing)
const BLOODTHIRSTY_CONQUEROR_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &BLOODTHIRSTY_CONQUEROR,
    3,
    "feb45b34-a1e7-4861-b0fe-b701aa5e894a",
    "Raita Kazama",
);

// FDN 427 — Twinflame Tyrant (alternate printing)
const TWINFLAME_TYRANT_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &TWINFLAME_TYRANT,
    3,
    "765e6a52-773d-420c-829c-fefe4993bfd9",
    "Mikio Masuda",
);

// FDN 428 — Doubling Season (alternate printing)
const DOUBLING_SEASON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_rav::DOUBLING_SEASON,
    1,
    "4ba8fabe-3af7-4f19-9508-5b5a6a608a3c",
    "Kawasumi",
);

// FDN 429 — Llanowar Elves (alternate printing)
const LLANOWAR_ELVES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::LLANOWAR_ELVES,
    1,
    "cd43a84a-cde2-48f8-b6ab-1ec023c9e0c3",
    "Hisashi Momose",
);

// FDN 430 — Muldrotha, the Gravetide (alternate printing)
const MULDROTHA_THE_GRAVETIDE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_dom::MULDROTHA_THE_GRAVETIDE,
    1,
    "ff672733-486b-4042-8209-fca70df86b7d",
    "KERA",
);

// FDN 431 — Progenitus (alternate printing)
const PROGENITUS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_con::PROGENITUS,
    1,
    "7fee7f2b-de71-4ab2-8104-5fb467b4100b",
    "Sansyu",
);

// FDN 432 — Day of Judgment (alternate printing)
const DAY_OF_JUDGMENT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_zen::DAY_OF_JUDGMENT,
    2,
    "e7b2d913-0b0e-4840-bf88-29dd6f9da631",
    "Takayama Toshiaki",
);

// FDN 433 — Herald of Eternal Dawn (alternate printing)
const HERALD_OF_ETERNAL_DAWN_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &HERALD_OF_ETERNAL_DAWN,
    4,
    "edb46675-5286-4eaf-9c25-eef773a4daeb",
    "Nijihayashi",
);

// FDN 434 — Kaito, Cunning Infiltrator (alternate printing)
const KAITO_CUNNING_INFILTRATOR_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &KAITO_CUNNING_INFILTRATOR,
    4,
    "84779aa7-6147-4c83-aab2-ae4346524977",
    "Ittoku",
);

// FDN 435 — Think Twice (alternate printing)
const THINK_TWICE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_tsp::THINK_TWICE,
    3,
    "7228018d-3e25-41f0-ae16-beb96dd5e1cc",
    "HNCL",
);

// FDN 436 — Bloodthirsty Conqueror (alternate printing)
const BLOODTHIRSTY_CONQUEROR_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &BLOODTHIRSTY_CONQUEROR,
    4,
    "926074a7-a01d-43bb-a257-0d95394773a0",
    "Raita Kazama",
);

// FDN 437 — Twinflame Tyrant (alternate printing)
const TWINFLAME_TYRANT_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &TWINFLAME_TYRANT,
    4,
    "89d02e4b-3854-4e10-acb7-0b6128401b2e",
    "Mikio Masuda",
);

// FDN 438 — Doubling Season (alternate printing)
const DOUBLING_SEASON_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_rav::DOUBLING_SEASON,
    2,
    "0d5bf249-3773-404f-9e46-d7745d3826e8",
    "Kawasumi",
);

// FDN 439 — Llanowar Elves (alternate printing)
const LLANOWAR_ELVES_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::LLANOWAR_ELVES,
    2,
    "11bd6cef-f887-4b07-a957-4c53cb3c9c87",
    "Hisashi Momose",
);

// FDN 440 — Muldrotha, the Gravetide (alternate printing)
const MULDROTHA_THE_GRAVETIDE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_dom::MULDROTHA_THE_GRAVETIDE,
    2,
    "adc15cd2-e6d1-406e-af58-2f5fbaa74a30",
    "KERA",
);

// FDN 441 — Progenitus (alternate printing)
const PROGENITUS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_con::PROGENITUS,
    2,
    "4d284ddb-c1de-49a3-8a2a-8146f7c3fbeb",
    "Sansyu",
);

// FDN 442 — Arahbo, the First Fang (alternate printing)
const ARAHBO_THE_FIRST_FANG_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &ARAHBO_THE_FIRST_FANG,
    3,
    "52605015-ba08-4427-8c06-b47ecd603b27",
    "Simon Dominic",
);

// FDN 443 — Celestial Armor (alternate printing)
const CELESTIAL_ARMOR_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &CELESTIAL_ARMOR,
    3,
    "78e9e1d6-5cb3-42c1-9211-9769b2c04db6",
    "Olena Richards",
);

// FDN 444 — Crystal Barricade (alternate printing)
const CRYSTAL_BARRICADE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &CRYSTAL_BARRICADE,
    3,
    "9cb02c30-8898-4692-a22b-7ea7855e8f81",
    "Rockey Chen",
);

// FDN 445 — Exemplar of Light (alternate printing)
const EXEMPLAR_OF_LIGHT_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &EXEMPLAR_OF_LIGHT,
    3,
    "98d61887-a6f7-4ce0-a44d-9661e8bb2107",
    "Ekaterina Burmak",
);

// FDN 446 — Herald of Eternal Dawn (alternate printing)
const HERALD_OF_ETERNAL_DAWN_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &HERALD_OF_ETERNAL_DAWN,
    5,
    "b38dc08b-9a80-47ad-8c0e-ed19487f043f",
    "Martina Fačková",
);

// FDN 447 — Raise the Past (alternate printing)
const RAISE_THE_PAST_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &RAISE_THE_PAST,
    3,
    "399f9421-da89-4221-8a59-a8d112a5598d",
    "Nathaniel Himawan",
);

// FDN 448 — Skyknight Squire (alternate printing)
const SKYKNIGHT_SQUIRE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &SKYKNIGHT_SQUIRE,
    3,
    "28007f11-6610-42e8-9fb6-1a6091ec9435",
    "Alexander Mokhov",
);

// FDN 449 — Valkyrie's Call (alternate printing)
const VALKYRIE_S_CALL_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &VALKYRIE_S_CALL,
    3,
    "650498aa-3fc1-47ac-b8d1-d13433640d57",
    "Scott Murphy",
);

// FDN 450 — Archmage of Runes (alternate printing)
const ARCHMAGE_OF_RUNES_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &ARCHMAGE_OF_RUNES,
    3,
    "28658ae0-9b5b-497d-a3c9-0a0626b7b344",
    "Kai Carpenter",
);

// FDN 451 — Curator of Destinies (alternate printing)
const CURATOR_OF_DESTINIES_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &CURATOR_OF_DESTINIES,
    3,
    "70d11928-1d4b-413d-b43c-280525238d58",
    "Ralph Horsley",
);

// FDN 452 — Drake Hatcher (alternate printing)
const DRAKE_HATCHER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &DRAKE_HATCHER,
    3,
    "c4c31f0d-b77f-4cf9-8350-2dde15d427a7",
    "Chris Rallis",
);

// FDN 453 — High Fae Trickster (alternate printing)
const HIGH_FAE_TRICKSTER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &HIGH_FAE_TRICKSTER,
    3,
    "347e6783-3809-4b8f-ac68-e786c944e7f5",
    "Justyna Dura",
);

// FDN 454 — Homunculus Horde (alternate printing)
const HOMUNCULUS_HORDE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &HOMUNCULUS_HORDE,
    3,
    "f3ec1d32-9467-4c79-a9d6-bf3aad2d7fed",
    "Adrián Rodríguez Pérez",
);

// FDN 455 — Kiora, the Rising Tide (alternate printing)
const KIORA_THE_RISING_TIDE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &KIORA_THE_RISING_TIDE,
    3,
    "6b137e8d-3537-454d-8d6b-24d2a6d81993",
    "Julian Kok Joon Wen",
);

// FDN 456 — Lunar Insight (alternate printing)
const LUNAR_INSIGHT_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &LUNAR_INSIGHT,
    3,
    "cdec49a8-508d-4c4b-a60f-4cbeea56f4cd",
    "Dan Murayama Scott",
);

// FDN 457 — Sphinx of Forgotten Lore (alternate printing)
const SPHINX_OF_FORGOTTEN_LORE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &SPHINX_OF_FORGOTTEN_LORE,
    3,
    "ce455a01-87c1-42d1-a51e-1f9a7bd553d6",
    "Dmitry Burmak",
);

// FDN 458 — Abyssal Harvester (alternate printing)
const ABYSSAL_HARVESTER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &ABYSSAL_HARVESTER,
    3,
    "10f257e3-1d03-4395-8a2d-c71f702db9ad",
    "Diana Franco",
);

// FDN 459 — Blasphemous Edict (alternate printing)
const BLASPHEMOUS_EDICT_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &BLASPHEMOUS_EDICT,
    3,
    "252a6c01-70c4-4e84-b5ca-9a9d4e7c0d86",
    "Andrew Mar",
);

// FDN 460 — Bloodthirsty Conqueror (alternate printing)
const BLOODTHIRSTY_CONQUEROR_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &BLOODTHIRSTY_CONQUEROR,
    5,
    "cbd05eaa-7c0d-45f0-b085-87dc1d610c34",
    "Dmitry Burmak",
);

// FDN 461 — High-Society Hunter (alternate printing)
const HIGH_SOCIETY_HUNTER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &HIGH_SOCIETY_HUNTER,
    3,
    "fdf85410-7abe-4bd8-97be-c6432142c947",
    "Daneen Wilkerson",
);

// FDN 462 — Nine-Lives Familiar (alternate printing)
const NINE_LIVES_FAMILIAR_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &NINE_LIVES_FAMILIAR,
    3,
    "bf1c1b96-7ec0-4063-833f-fb32b720650e",
    "Bram Sels",
);

// FDN 463 — Tinybones, Bauble Burglar (alternate printing)
const TINYBONES_BAUBLE_BURGLAR_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &TINYBONES_BAUBLE_BURGLAR,
    3,
    "f01d5f4b-a780-4a89-82b1-84d4f3b62a7b",
    "Leonardo Santanna",
);

// FDN 464 — Zul Ashur, Lich Lord (alternate printing)
const ZUL_ASHUR_LICH_LORD_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &ZUL_ASHUR_LICH_LORD,
    3,
    "5ddcb2c4-210c-4361-9232-dc1327f0a972",
    "Raluca Marinescu",
);

// FDN 465 — Electroduplicate (alternate printing)
const ELECTRODUPLICATE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &ELECTRODUPLICATE,
    3,
    "cec6ab9e-99f7-4296-b8b2-6cdfa91c5fba",
    "Warren Mahy",
);

// FDN 466 — Kellan, Planar Trailblazer (alternate printing)
const KELLAN_PLANAR_TRAILBLAZER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &KELLAN_PLANAR_TRAILBLAZER,
    3,
    "64e35fbe-55b7-40c8-b24b-2d9d933bcdaa",
    "Zoltan Boros",
);

// FDN 467 — Rite of the Dragoncaller (alternate printing)
const RITE_OF_THE_DRAGONCALLER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &RITE_OF_THE_DRAGONCALLER,
    3,
    "3a99d9bc-da4e-488b-bc4a-90594c072b66",
    "PINDURSKI",
);

// FDN 468 — Searslicer Goblin (alternate printing)
const SEARSLICER_GOBLIN_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &SEARSLICER_GOBLIN,
    3,
    "9e3059e4-b198-4eba-98e8-f0f9f83d1928",
    "Wayne Reynolds",
);

// FDN 469 — Twinflame Tyrant (alternate printing)
const TWINFLAME_TYRANT_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &TWINFLAME_TYRANT,
    5,
    "04a474e3-e7de-4b4d-ad89-a72571968687",
    "Xabi Gaztelua",
);

// FDN 470 — Loot, Exuberant Explorer (alternate printing)
const LOOT_EXUBERANT_EXPLORER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &LOOT_EXUBERANT_EXPLORER,
    3,
    "b6671b19-28d3-4d67-a8fa-83e4f6596c68",
    "Arif Wijaya",
);

// FDN 471 — Mossborn Hydra (alternate printing)
const MOSSBORN_HYDRA_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &MOSSBORN_HYDRA,
    3,
    "a60e591e-5673-4504-9b1e-8448fae664ea",
    "Monztre",
);

// FDN 472 — Preposterous Proportions (alternate printing)
const PREPOSTEROUS_PROPORTIONS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &PREPOSTEROUS_PROPORTIONS,
    3,
    "838e2693-2dd4-4aec-bde9-369a069a3fef",
    "Ben Wootten",
);

// FDN 473 — Quilled Greatwurm (alternate printing)
const QUILLED_GREATWURM_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &QUILLED_GREATWURM,
    3,
    "354df84b-28a1-4b8f-a8ab-94060a35e05f",
    "Michal Ivan",
);

// FDN 474 — Spinner of Souls (alternate printing)
const SPINNER_OF_SOULS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &SPINNER_OF_SOULS,
    3,
    "3b3f233a-d4df-4836-84cc-9d3e86165926",
    "Xavier Ribeiro",
);

// FDN 475 — Sylvan Scavenging (alternate printing)
const SYLVAN_SCAVENGING_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &SYLVAN_SCAVENGING,
    3,
    "05b4b8c6-ded3-4f97-b5a4-346792a25991",
    "Josiah \"Jo\" Cameron",
);

// FDN 476 — Alesha, Who Laughs at Fate (alternate printing)
const ALESHA_WHO_LAUGHS_AT_FATE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &ALESHA_WHO_LAUGHS_AT_FATE,
    3,
    "003d05be-4cf2-43c6-a823-c2b9c6482d38",
    "Ekaterina Burmak",
);

// FDN 477 — Anthem of Champions (alternate printing)
const ANTHEM_OF_CHAMPIONS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &ANTHEM_OF_CHAMPIONS,
    3,
    "6749d0b4-4c37-4b43-9ad3-3b36d831f9e7",
    "Chris Rallis",
);

// FDN 478 — Ashroot Animist (alternate printing)
const ASHROOT_ANIMIST_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &ASHROOT_ANIMIST,
    3,
    "9c436e43-e575-4a01-ba03-9d72810a2883",
    "Caio Monteiro",
);

// FDN 479 — Elenda, Saint of Dusk (alternate printing)
const ELENDA_SAINT_OF_DUSK_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &ELENDA_SAINT_OF_DUSK,
    3,
    "9c76cc76-317c-4680-b411-43842000aa9f",
    "Chris Rahn",
);

// FDN 480 — Koma, World-Eater (alternate printing)
const KOMA_WORLD_EATER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &KOMA_WORLD_EATER,
    3,
    "e7ac4ece-8551-4662-baca-4e7d5beecb5e",
    "Mark Zug",
);

// FDN 481 — Kykar, Zephyr Awakener (alternate printing)
const KYKAR_ZEPHYR_AWAKENER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &KYKAR_ZEPHYR_AWAKENER,
    3,
    "fa23ced5-0baa-4a8d-9263-ec2d94233a84",
    "Dmitry Burmak",
);

// FDN 482 — Niv-Mizzet, Visionary (alternate printing)
const NIV_MIZZET_VISIONARY_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &NIV_MIZZET_VISIONARY,
    3,
    "76503c75-ff97-4c27-a55b-74c3e68578c9",
    "Dan Murayama Scott",
);

// FDN 483 — Zimone, Paradox Sculptor (alternate printing)
const ZIMONE_PARADOX_SCULPTOR_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &ZIMONE_PARADOX_SCULPTOR,
    3,
    "09b82d3f-eaba-499a-9783-e90f5a2b623b",
    "Nathaniel Himawan",
);

// FDN 484 — Banner of Kinship (alternate printing)
const BANNER_OF_KINSHIP_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &BANNER_OF_KINSHIP,
    3,
    "e4709aed-a74b-4319-87c1-40f14cca6dad",
    "Olena Richards",
);

// FDN 485 — Leyline Axe (alternate printing)
const LEYLINE_AXE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &LEYLINE_AXE,
    3,
    "f1dd7a17-24f5-43d5-bd5a-8b51b48e8a39",
    "Edgar Sánchez Hidalgo",
);

// FDN 486 — Scrawling Crawler (alternate printing)
const SCRAWLING_CRAWLER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &SCRAWLING_CRAWLER,
    3,
    "c1835cd8-16c1-467b-a613-a4df6f752894",
    "Miro Petrov",
);

// FDN 487 — Soulstone Sanctuary (alternate printing)
const SOULSTONE_SANCTUARY_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &SOULSTONE_SANCTUARY,
    3,
    "4620f95e-fc95-4643-ba11-7f9116176253",
    "Daniel Ljunggren",
);

// FDN 488 — Adamant Will (reprint)
const ADAMANT_WILL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dom::ADAMANT_WILL,
    "1fd2c9ab-b3dd-4f68-b91f-0f075a045757",
    "Irina Nordsol",
);

// FDN 489 — Ancestor Dragon (reprint)
const ANCESTOR_DRAGON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_gs1::ANCESTOR_DRAGON,
    "df43d9d4-3fef-4a56-8b38-d52824117201",
    "Shinchuen Chen",
);

// FDN 490 — Angelic Edict (reprint)
const ANGELIC_EDICT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_gtc::ANGELIC_EDICT,
    "c0bf349e-2974-4aea-a5c0-fdaea77325cc",
    "Trevor Claxton",
);

// FDN 491 — Bishop's Soldier (reprint)
const BISHOP_S_SOLDIER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_xln::BISHOP_S_SOLDIER,
    "16dfd7b3-6d01-4e98-aec3-b27e8e2444e8",
    "Scott Murphy",
);

// FDN 492 — Deadly Riposte (reprint)
const DEADLY_RIPOSTE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_bro::DEADLY_RIPOSTE,
    "65f5804a-075b-41d8-9639-785cad5c0974",
    "Olena Richards",
);

// FDN 493 — Elspeth's Smite (reprint)
const ELSPETH_S_SMITE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mom::ELSPETH_S_SMITE,
    "b405a442-cf6f-4c0b-9950-f208b30c052e",
    "Livia Prima",
);

// FDN 494 — Herald of Faith (reprint)
const HERALD_OF_FAITH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m19::HERALD_OF_FAITH,
    "5579f35b-b3d2-4342-a52b-ae36b579d044",
    "Tommy Arnold",
);

// FDN 495 — Ingenious Leonin (reprint)
const INGENIOUS_LEONIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_j22::INGENIOUS_LEONIN,
    "ea566679-4202-4076-9314-142241485f6e",
    "Eric Deschamps",
);

// FDN 496 — Inspiring Overseer (reprint)
const INSPIRING_OVERSEER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_snc::INSPIRING_OVERSEER,
    "c3e9aa54-e5d5-4cbb-9d2a-263b9eba398f",
    "Irina Nordsol",
);

// FDN 497 — Jazal Goldmane (reprint)
const JAZAL_GOLDMANE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_c14::JAZAL_GOLDMANE,
    "5aa8f635-c638-4207-8631-f6b5185f8696",
    "Aaron Miller",
);

// FDN 498 — Leonin Skyhunter (reprint)
const LEONIN_SKYHUNTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mrd::LEONIN_SKYHUNTER,
    "218e0009-5f11-4348-97b8-4bc7b41f80b8",
    "Kev Walker",
);

// FDN 499 — Leonin Vanguard (reprint)
const LEONIN_VANGUARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m19::LEONIN_VANGUARD,
    "17b25850-f1bd-4410-beec-603b2a77f264",
    "Aaron Miller",
);

// FDN 500 — Moment of Triumph (reprint)
const MOMENT_OF_TRIUMPH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rix::MOMENT_OF_TRIUMPH,
    "911deea0-38d2-41d9-a2ae-fb8eb45b2c23",
    "Steven Belledin",
);

// FDN 501 — Pacifism (reprint)
const PACIFISM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mir::PACIFISM,
    "839160d2-44a3-4566-be9d-558d043beac8",
    "Kev Walker",
);

// FDN 502 — Prayer of Binding (reprint)
const PRAYER_OF_BINDING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dmu::PRAYER_OF_BINDING,
    "a30b42c4-90d8-462d-aca5-891906992ab8",
    "Wylie Beckert",
);

// FDN 503 — Twinblade Paladin (reprint)
const TWINBLADE_PALADIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m20::TWINBLADE_PALADIN,
    "5cd9e73d-de8d-486f-bbbd-a2f5d3f8f686",
    "Jana Schirmer",
);

// FDN 504 — Burrog Befuddler (reprint)
const BURROG_BEFUDDLER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_stx::BURROG_BEFUDDLER,
    "c5f11ea2-cd4c-417a-804c-3df80d9ddd5f",
    "Zoltan Boros",
);

// FDN 505 — Cancel (reprint)
const CANCEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tsp::CANCEL,
    "475bff39-220a-4490-9c2e-d311e306a6db",
    "Mathias Kollros",
);

// FDN 506 — Corsair Captain (reprint)
const CORSAIR_CAPTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_jmp::CORSAIR_CAPTAIN,
    "d5017dbc-07fd-45ea-9629-7c584144e8be",
    "Victor Adame Minguez",
);

// FDN 507 — Eaten by Piranhas (reprint)
const EATEN_BY_PIRANHAS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lci::EATEN_BY_PIRANHAS,
    "475e28bb-1333-45e1-b6fd-83121c2f1ab9",
    "Abz J Harding",
);

// FDN 508 — Exclusion Mage (reprint)
const EXCLUSION_MAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m19::EXCLUSION_MAGE,
    "62c8024e-490a-484a-bcee-da7728a64a1f",
    "Chris Seaman",
);

// FDN 509 — Into the Roil (reprint)
const INTO_THE_ROIL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_zen::INTO_THE_ROIL,
    "25edefa9-9fff-4b71-83ae-696c196a795c",
    "Campbell White",
);

// FDN 510 — Kitesail Corsair (reprint)
const KITESAIL_CORSAIR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rix::KITESAIL_CORSAIR,
    "c82d7c6d-523c-46c9-a189-44c0bd8386d1",
    "Greg Opalinski",
);

// FDN 511 — Mystic Archaeologist (reprint)
const MYSTIC_ARCHAEOLOGIST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m19::MYSTIC_ARCHAEOLOGIST,
    "c069ff32-6759-48a2-8674-4f729c91dcd0",
    "Eric Deschamps",
);

// FDN 512 — Opt (reprint)
const OPT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_inv::OPT,
    "58d26b54-0093-4e90-a2b1-b57c64340f9c",
    "Tyler Jacobson",
);

// FDN 513 — Quick Study (reprint)
const QUICK_STUDY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_woe::QUICK_STUDY,
    "098c86fd-5a38-41fd-be44-ab963448f2a2",
    "Iris Compiet",
);

// FDN 514 — Starlight Snare (reprint)
const STARLIGHT_SNARE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_j25::STARLIGHT_SNARE,
    "74fb19b2-4f6c-4cbd-8756-a7eb5c7c9ef6",
    "Borja Pindado",
);

// FDN 515 — Storm Fleet Spy (reprint)
const STORM_FLEET_SPY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_xln::STORM_FLEET_SPY,
    "f6c5206e-63db-44c0-86ab-f645cd358b3b",
    "Scott Murphy",
);

// FDN 516 — Bloodtithe Collector (reprint)
const BLOODTITHE_COLLECTOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mid::BLOODTITHE_COLLECTOR,
    "ff16d951-dd85-4d22-9505-ce6e2eaf47cf",
    "Maria Zolotukhina",
);

// FDN 517 — Cemetery Recruitment (reprint)
const CEMETERY_RECRUITMENT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_emn::CEMETERY_RECRUITMENT,
    "34a80873-86d7-44a9-894b-65e8f77dd2ea",
    "Kieran Yanner",
);

// FDN 518 — Crossway Troublemakers (reprint)
const CROSSWAY_TROUBLEMAKERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_voc::CROSSWAY_TROUBLEMAKERS,
    "03a5e19c-4cb8-4bd7-923c-4db747dc6ca3",
    "Aaron J. Riley",
);

// FDN 519 — Crow of Dark Tidings (reprint)
const CROW_OF_DARK_TIDINGS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_soi::CROW_OF_DARK_TIDINGS,
    "2cd74e93-064a-42d7-8e6e-c413912a08cd",
    "Simon Dominic",
);

// FDN 520 — Deadly Plot (reprint)
const DEADLY_PLOT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_j22::DEADLY_PLOT,
    "b32fddc3-a38f-4eea-ae01-4158e3cbca6c",
    "Peter Polach",
);

// FDN 521 — Death Baron (reprint)
const DEATH_BARON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ala::DEATH_BARON,
    "34a27142-cf45-4164-9e10-373e5c8bf7c6",
    "Nils Hamm",
);

// FDN 522 — Highborn Vampire (reprint)
const HIGHBORN_VAMPIRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_znr::HIGHBORN_VAMPIRE,
    "53c56a23-eb4f-4be1-ada9-1ad5b80195d3",
    "Denman Rooke",
);

// FDN 523 — Maalfeld Twins (reprint)
const MAALFELD_TWINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_avr::MAALFELD_TWINS,
    "c166df8f-9508-427a-8ec7-bc8541b6ed88",
    "Mike Sass",
);

// FDN 524 — Moment of Craving (reprint)
const MOMENT_OF_CRAVING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rix::MOMENT_OF_CRAVING,
    "3627f9fb-b828-49cd-887d-e2ab3ef43dfb",
    "Steven Belledin",
);

// FDN 525 — Offer Immortality (reprint)
const OFFER_IMMORTALITY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_one::OFFER_IMMORTALITY,
    "6d5802ed-507d-4a52-90e3-d989cd61961b",
    "A. M. Sartor",
);

// FDN 526 — Skeleton Archer (reprint)
const SKELETON_ARCHER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m19::SKELETON_ARCHER,
    "7a959048-0d8a-41bb-8a33-52264e525085",
    "Randy Vargas",
);

// FDN 527 — Suspicious Shambler (reprint)
const SUSPICIOUS_SHAMBLER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_j22::SUSPICIOUS_SHAMBLER,
    "3d2c5345-eb55-4bca-9183-b4e1404405f8",
    "Javier Charro",
);

// FDN 528 — Undying Malice (reprint)
const UNDYING_MALICE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2021::innistrad_crimson_vow::UNDYING_MALICE,
    "97b3cf11-e352-4ee1-8c03-13898f576ef9",
    "Igor Kieryluk",
);

// FDN 529 — Untamed Hunger (reprint)
const UNTAMED_HUNGER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ogw::UNTAMED_HUNGER,
    "05937911-897b-4638-9536-2e463884f453",
    "Willian Murai",
);

// FDN 530 — Vampire Interloper (reprint)
const VAMPIRE_INTERLOPER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_isd::VAMPIRE_INTERLOPER,
    "65455d94-487c-4104-a1a0-149515a90eaa",
    "James Ryman",
);

// FDN 531 — Vampire Neonate (reprint)
const VAMPIRE_NEONATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m19::VAMPIRE_NEONATE,
    "86d64a1d-cda4-4c76-8d58-ccab5a6859a0",
    "Daarken",
);

// FDN 532 — Vampire Spawn (reprint)
const VAMPIRE_SPAWN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_afr::VAMPIRE_SPAWN,
    "b6fc8d2e-ad63-498d-83b6-5f72b5ae0e67",
    "Alex Brock",
);

// FDN 533 — Battle-Rattle Shaman (reprint)
const BATTLE_RATTLE_SHAMAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_roe::BATTLE_RATTLE_SHAMAN,
    "fee05f7e-d4da-4a72-8140-d505eecbdd32",
    "Warren Mahy",
);

// FDN 534 — Carnelian Orb of Dragonkind (reprint)
const CARNELIAN_ORB_OF_DRAGONKIND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_clb::CARNELIAN_ORB_OF_DRAGONKIND,
    "f0c07546-c5a1-4dfc-b5e1-d697578a8c11",
    "Lars Grant-West",
);

// FDN 535 — Dragon Fodder (reprint)
const DRAGON_FODDER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ala::DRAGON_FODDER,
    "f9abe517-f601-4784-8d62-2350bd755146",
    "Volkan Baǵa",
);

// FDN 536 — Dragonlord's Servant (reprint)
const DRAGONLORD_S_SERVANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dtk::DRAGONLORD_S_SERVANT,
    "37e35f88-a580-4b55-9c11-0b58e2f752d1",
    "Steve Prescott",
);

// FDN 537 — Dropkick Bomber (reprint)
const DROPKICK_BOMBER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_j25::DROPKICK_BOMBER,
    "b44f758e-716a-408e-96d4-b58403591c2a",
    "Quintin Gleim",
);

// FDN 538 — Fire Elemental (reprint)
const FIRE_ELEMENTAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FIRE_ELEMENTAL,
    "dc506f58-048d-49cc-ad8c-2eb851b08bb6",
    "Joe Slucher",
);

// FDN 539 — Goblin Oriflamme (reprint)
const GOBLIN_ORIFLAMME_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mh1::GOBLIN_ORIFLAMME,
    "0259e4d8-849b-468e-a902-aa1d7f2d5b6a",
    "David Palumbo",
);

// FDN 540 — Goblin Smuggler (reprint)
const GOBLIN_SMUGGLER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m20::GOBLIN_SMUGGLER,
    "5837ccf1-9bac-4afb-bcc9-82c5f3c0e8e2",
    "Milivoj Ćeran",
);

// FDN 541 — Kargan Dragonrider (reprint)
const KARGAN_DRAGONRIDER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m19::KARGAN_DRAGONRIDER,
    "701aef07-4291-4dfd-b9b2-bfc26745341c",
    "Greg Opalinski",
);

// FDN 542 — Kindled Fury (reprint)
const KINDLED_FURY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mor::KINDLED_FURY,
    "af158462-91e3-4ad7-b435-95d4eb32fe0a",
    "Wayne Reynolds",
);

// FDN 543 — Raging Redcap (reprint)
const RAGING_REDCAP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_eld::RAGING_REDCAP,
    "9627b0a7-bda9-44df-81c9-aa70cc976331",
    "Dan Murayama Scott",
);

// FDN 544 — Rapacious Dragon (reprint)
const RAPACIOUS_DRAGON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m20::RAPACIOUS_DRAGON,
    "5eaddac1-d8ca-4949-8140-c9193e3df921",
    "Johan Grenier",
);

// FDN 545 — Scorching Dragonfire (reprint)
const SCORCHING_DRAGONFIRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_eld::SCORCHING_DRAGONFIRE,
    "44ea09c1-d420-4e99-a968-ade614579287",
    "Eric Velhagen",
);

// FDN 546 — Seize the Spoils (reprint)
const SEIZE_THE_SPOILS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_khm::SEIZE_THE_SPOILS,
    "efcf7e75-5de2-45cc-9275-caccd73214fa",
    "Brian Valeza",
);

// FDN 547 — Skyraker Giant (reprint)
const SKYRAKER_GIANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ori::SKYRAKER_GIANT,
    "5e3fbafb-e915-43eb-8a68-245840ba73ff",
    "Anastasia Ovchinnikova",
);

// FDN 548 — Swab Goblin (reprint)
const SWAB_GOBLIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rix::SWAB_GOBLIN,
    "8db11970-74c2-463d-88bb-9f88aa079eaa",
    "Josu Hernaiz",
);

// FDN 549 — Terror of Mount Velus (reprint)
const TERROR_OF_MOUNT_VELUS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_thb::TERROR_OF_MOUNT_VELUS,
    "3959a6c7-d615-4524-b94d-65f4164a2656",
    "Billy Christian",
);

// FDN 550 — Volley Veteran (reprint)
const VOLLEY_VETERAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m19::VOLLEY_VETERAN,
    "7fc914fe-e699-4a21-aa09-f3573d020b87",
    "Olivier Bernard",
);

// FDN 551 — Aggressive Mammoth (reprint)
const AGGRESSIVE_MAMMOTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m19::AGGRESSIVE_MAMMOTH,
    "7724b978-999b-4654-96f3-58e28aa7cb34",
    "Filip Burburan",
);

// FDN 552 — Bear Cub (reprint)
const BEAR_CUB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_p02::BEAR_CUB,
    "d8662ebb-068b-41d2-b504-4b5854e4d4aa",
    "Ron Spencer",
);

// FDN 553 — Biogenic Upgrade (reprint)
const BIOGENIC_UPGRADE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rna::BIOGENIC_UPGRADE,
    "ef20af5e-1ffe-426a-805a-ca4a6a122260",
    "Kev Fang",
);

// FDN 554 — Druid of the Cowl (reprint)
const DRUID_OF_THE_COWL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_aer::DRUID_OF_THE_COWL,
    "db2d0ee9-865c-4fc9-8cb6-540c597e1bf4",
    "Magali Villeneuve",
);

// FDN 555 — Joraga Invocation (reprint)
const JORAGA_INVOCATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ori::JORAGA_INVOCATION,
    "01ceca57-65f4-4f99-bd14-9fe5f86c1f62",
    "Kieran Yanner",
);

// FDN 556 — Magnigoth Sentry (reprint)
const MAGNIGOTH_SENTRY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dmu::MAGNIGOTH_SENTRY,
    "5624e610-c4c0-4103-a32b-0f1264030a7a",
    "Dave Kendall",
);

// FDN 557 — New Horizons (reprint)
const NEW_HORIZONS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_xln::NEW_HORIZONS,
    "86b3923c-c35c-4eb2-9dd3-b15c13778ecf",
    "Eytan Zana",
);

// FDN 558 — Tajuru Pathwarden (reprint)
const TAJURU_PATHWARDEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ogw::TAJURU_PATHWARDEN,
    "da20a0d3-2022-4dea-84c8-85adc5a974f8",
    "Victor Adame Minguez",
);

// FDN 559 — Thornweald Archer (reprint)
const THORNWEALD_ARCHER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_fut::THORNWEALD_ARCHER,
    "189f6199-f2fe-49a5-89ca-3c4cb39fbf2b",
    "Dave Kendall",
);

// FDN 560 — Thrashing Brontodon (reprint)
const THRASHING_BRONTODON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rix::THRASHING_BRONTODON,
    "592364bf-68b5-4ea0-97d6-c7cadce940fd",
    "Jakub Kasper",
);

// FDN 561 — Wildheart Invoker (reprint)
const WILDHEART_INVOKER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_roe::WILDHEART_INVOKER,
    "60f42969-bb5c-4183-8bd3-ba87f008391d",
    "Erica Yang",
);

// FDN 562 — Goblin Firebomb (reprint)
const GOBLIN_FIREBOMB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_bro::GOBLIN_FIREBOMB,
    "55ab3f99-9541-4100-818c-5d9e916791e4",
    "Noah Thatcher",
);

// FDN 563 — Pirate's Cutlass (reprint)
const PIRATE_S_CUTLASS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_xln::PIRATE_S_CUTLASS,
    "d9d20aef-d35b-4353-a241-e6cfa9730975",
    "John Stanko",
);

// FDN 564 — Uncharted Haven (reprint)
const UNCHARTED_HAVEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_neo::UNCHARTED_HAVEN,
    "172cd5b7-98fc-4add-b858-a0b3dfb75c19",
    "Adam Paquette",
);

// FDN 565 — Angelic Destiny (reprint)
const ANGELIC_DESTINY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m12::ANGELIC_DESTINY,
    "066d73fa-369b-44a3-b1a9-d22176ac3566",
    "Jana Schirmer & Johannes Voss",
);

// FDN 566 — Archway Angel (reprint)
const ARCHWAY_ANGEL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rna::ARCHWAY_ANGEL,
    "2c3ff489-bdd7-4aeb-8130-3c234d0dd3dd",
    "Milivoj Ćeran",
);

// FDN 567 — Ballyrush Banneret (reprint)
const BALLYRUSH_BANNERET_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mor::BALLYRUSH_BANNERET,
    "b18a7daa-a112-4728-9123-594366694915",
    "Ralph Horsley",
);

// FDN 568 — Charming Prince (reprint)
const CHARMING_PRINCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_eld::CHARMING_PRINCE,
    "aa7b47e1-7e32-4f2f-aecf-bac7ca197081",
    "Randy Vargas",
);

// FDN 569 — Crusader of Odric (reprint)
const CRUSADER_OF_ODRIC_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m13::CRUSADER_OF_ODRIC,
    "009029c2-26b9-498f-8765-e91c2e1f3aee",
    "Michael Komarck",
);

// FDN 570 — Dawnwing Marshal (reprint)
const DAWNWING_MARSHAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_j25::DAWNWING_MARSHAL,
    "807db84f-addb-4ef0-b9ba-32b3e1bced3d",
    "Aldo Domínguez",
);

// FDN 571 — Devout Decree (reprint)
const DEVOUT_DECREE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m20::DEVOUT_DECREE,
    "d174cb01-b1cf-444c-a893-cc61ddf88b8c",
    "Zoltan Boros",
);

// FDN 572 — Disenchant (reprint)
const DISENCHANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DISENCHANT,
    "7ac43e16-8b14-46f2-877a-600ea918766b",
    "Richard Kane Ferguson",
);

// FDN 573 — Felidar Cub (reprint)
const FELIDAR_CUB_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_bfz::FELIDAR_CUB,
    "0d61d50b-7ab6-45a9-b207-31d87aa2e555",
    "Steve Prescott",
);

// FDN 574 — Felidar Retreat (reprint)
const FELIDAR_RETREAT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_znr::FELIDAR_RETREAT,
    "89e3cc09-5057-4c05-88fc-d6cda809fc74",
    "Ralph Horsley",
);

// FDN 575 — Fumigate (reprint)
const FUMIGATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_kld::FUMIGATE,
    "e2c1e655-1883-428f-abe8-69c011cfbd4b",
    "Svetlin Velinov",
);

// FDN 576 — Knight of Grace (reprint)
const KNIGHT_OF_GRACE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dom::KNIGHT_OF_GRACE,
    "4efb8633-0a70-4ddc-80af-42508ec75cff",
    "Sidharth Chaturvedi",
);

// FDN 577 — Linden, the Steadfast Queen (reprint)
const LINDEN_THE_STEADFAST_QUEEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_eld::LINDEN_THE_STEADFAST_QUEEN,
    "0dcd0898-db3d-44ec-9d56-a1b558da90f4",
    "Ryan Pancoast",
);

// FDN 578 — Mentor of the Meek (reprint)
const MENTOR_OF_THE_MEEK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_isd::MENTOR_OF_THE_MEEK,
    "60790977-efd4-470a-823a-6929f84016e7",
    "Jana Schirmer & Johannes Voss",
);

// FDN 579 — Regal Caracal (reprint)
const REGAL_CARACAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_akh::REGAL_CARACAL,
    "b6310a82-73db-40cb-ae64-6f07f869024c",
    "Filip Burburan",
);

// FDN 580 — Release the Dogs (reprint)
const RELEASE_THE_DOGS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_jmp::RELEASE_THE_DOGS,
    "07f10906-4fd5-44e8-99c7-3f9dcc988c48",
    "Jason Kang",
);

// FDN 581 — Stasis Snare (reprint)
const STASIS_SNARE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_bfz::STASIS_SNARE,
    "ce41e348-44c2-47f8-8e7d-da2e4d16d648",
    "Jason Felix",
);

// FDN 582 — Syr Alin, the Lion's Claw (reprint)
const SYR_ALIN_THE_LION_S_CLAW_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_eld::SYR_ALIN_THE_LION_S_CLAW,
    "5d42be5f-a6a7-4699-abf7-9632de6daede",
    "Paul Scott Canavan",
);

// FDN 583 — Valorous Stance (reprint)
const VALOROUS_STANCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_frf::VALOROUS_STANCE,
    "bc4c47e3-04b6-4760-9a80-b6acef1e4524",
    "Willian Murai",
);

// FDN 584 — Zetalpa, Primal Dawn (reprint)
const ZETALPA_PRIMAL_DAWN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rix::ZETALPA_PRIMAL_DAWN,
    "694df80e-77d1-4455-b6f1-58d212267d76",
    "Chris Rallis",
);

// FDN 585 — Arcanis the Omnipotent (reprint)
const ARCANIS_THE_OMNIPOTENT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ons::ARCANIS_THE_OMNIPOTENT,
    "a6fee90a-7cb3-4fad-91a7-1b1edbde0133",
    "Justin Sweet",
);

// FDN 586 — Chart a Course (reprint)
const CHART_A_COURSE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_xln::CHART_A_COURSE,
    "7599d459-fe71-48f4-8c65-0f519bb63a68",
    "James Ryman",
);

// FDN 587 — Dictate of Kruphix (reprint)
const DICTATE_OF_KRUPHIX_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_jou::DICTATE_OF_KRUPHIX,
    "b5e4087b-e692-4eb2-bb59-ff2a001d6dd4",
    "Daarken",
);

// FDN 588 — Dive Down (reprint)
const DIVE_DOWN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_xln::DIVE_DOWN,
    "65d5cff9-a3ec-432d-9ce5-68949e524279",
    "Magali Villeneuve",
);

// FDN 589 — Finale of Revelation (reprint)
const FINALE_OF_REVELATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_war::FINALE_OF_REVELATION,
    "0a77e063-e8f6-4cb2-954e-74cf41ce73da",
    "Johann Bodin",
);

// FDN 590 — Flashfreeze (reprint)
const FLASHFREEZE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_csp::FLASHFREEZE,
    "91e37a7e-6093-4ef5-b6e4-4aa800ddfc1b",
    "Brian Despain",
);

// FDN 591 — Fog Bank (reprint)
const FOG_BANK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::FOG_BANK,
    "18748b1d-4161-482c-a726-8762b4c1819c",
    "Howard Lyon",
);

// FDN 592 — Gateway Sneak (reprint)
const GATEWAY_SNEAK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rna::GATEWAY_SNEAK,
    "3834176f-29c5-4511-87d6-75d0c348a770",
    "Matt Stewart",
);

// FDN 593 — Harbinger of the Tides (reprint)
const HARBINGER_OF_THE_TIDES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ori::HARBINGER_OF_THE_TIDES,
    "9ca4f70d-ec17-49a4-8968-598ecdbe8243",
    "Svetlin Velinov",
);

// FDN 594 — Mystical Teachings (reprint)
const MYSTICAL_TEACHINGS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_tsp::MYSTICAL_TEACHINGS,
    "3f88eacf-5c7b-4a35-86f0-af3b0516d4c2",
    "Ron Spears",
);

// FDN 595 — River's Rebuke (reprint)
const RIVER_S_REBUKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_xln::RIVER_S_REBUKE,
    "32ae984e-5d1e-4497-9a71-7406f78c09f3",
    "Raymond Swanland",
);

// FDN 596 — Shipwreck Dowser (reprint)
const SHIPWRECK_DOWSER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2020::core_set_2021::SHIPWRECK_DOWSER,
    "1f20fe3d-792a-4030-a25c-e81b48b2bcb4",
    "Caroline Gariba",
);

// FDN 597 — Sphinx of the Final Word (reprint)
const SPHINX_OF_THE_FINAL_WORD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ogw::SPHINX_OF_THE_FINAL_WORD,
    "a2cf2366-40d0-4f94-84a5-8aa42308e223",
    "Lius Lasahido",
);

// FDN 598 — Tempest Djinn (reprint)
const TEMPEST_DJINN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dom::TEMPEST_DJINN,
    "7766506c-bc18-4ac3-9034-e8f7e54c4039",
    "Zezhou Chen",
);

// FDN 599 — Unsummon (reprint)
const UNSUMMON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::UNSUMMON,
    "b378d319-ab98-422a-8f72-121a9d26f5c6",
    "Ron Spencer",
);

// FDN 600 — Voracious Greatshark (reprint)
const VORACIOUS_GREATSHARK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_iko::VORACIOUS_GREATSHARK,
    "c27b40dd-9b2a-4a99-a984-ee9cfdb091a1",
    "Mathias Kollros",
);

// FDN 601 — Deathmark (reprint)
const DEATHMARK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_csp::DEATHMARK,
    "42592d75-593e-4719-b13b-bca374017e79",
    "Jeremy Jarvis",
);

// FDN 602 — Demonic Pact (reprint)
const DEMONIC_PACT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ori::DEMONIC_PACT,
    "5b0b7242-df91-48cf-bf4e-b68306c9965f",
    "Manuel Castañón",
);

// FDN 603 — Desecration Demon (reprint)
const DESECRATION_DEMON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rtr::DESECRATION_DEMON,
    "52bf6460-0df6-4dd3-8af8-df728683bcaa",
    "Jason Chan",
);

// FDN 604 — Dread Summons (reprint)
const DREAD_SUMMONS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_c15::DREAD_SUMMONS,
    "dfb5dd33-6e5b-4d78-92fa-678c9f01c606",
    "Izzy",
);

// FDN 605 — Driver of the Dead (reprint)
const DRIVER_OF_THE_DEAD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_avr::DRIVER_OF_THE_DEAD,
    "4443696a-0b9a-4081-91aa-800b5c4065d2",
    "James Ryman",
);

// FDN 606 — Duress (reprint)
const DURESS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::DURESS,
    "34c3a894-ee75-4db9-a69f-711bb3cc150a",
    "PINDURSKI",
);

// FDN 607 — Kalastria Highborn (reprint)
const KALASTRIA_HIGHBORN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wwk::KALASTRIA_HIGHBORN,
    "1a52f6ef-7759-4be3-8b80-a57e9d6ae386",
    "D. Alexander Gregory",
);

// FDN 608 — Knight of Malice (reprint)
const KNIGHT_OF_MALICE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dom::KNIGHT_OF_MALICE,
    "bb678e10-e5a4-4143-8e49-7b523118674e",
    "Sidharth Chaturvedi",
);

// FDN 609 — Midnight Reaper (reprint)
const MIDNIGHT_REAPER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_grn::MIDNIGHT_REAPER,
    "f229b0e6-1ffd-410e-b04b-a0afc179c58c",
    "Sidharth Chaturvedi",
);

// FDN 610 — Myojin of Night's Reach (reprint)
const MYOJIN_OF_NIGHT_S_REACH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_chk::MYOJIN_OF_NIGHT_S_REACH,
    "73bc5505-7fa0-4ebf-a691-b4ca5f52b019",
    "Kev Walker",
);

// FDN 611 — Nullpriest of Oblivion (reprint)
const NULLPRIEST_OF_OBLIVION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_znr::NULLPRIEST_OF_OBLIVION,
    "b2c7613c-38fc-49c3-93ee-1df93136455a",
    "Yongjae Choi",
);

// FDN 612 — Pulse Tracker (reprint)
const PULSE_TRACKER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wwk::PULSE_TRACKER,
    "fdcbbe4d-5c93-4b14-8123-cd835452870f",
    "Andrew Robinson",
);

// FDN 613 — Sanguine Indulgence (reprint)
const SANGUINE_INDULGENCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m21::SANGUINE_INDULGENCE,
    "03a26ca7-b26f-4b86-a060-712f15f4bf7f",
    "Andrey Kuzinskiy",
);

// FDN 614 — Tribute to Hunger (reprint)
const TRIBUTE_TO_HUNGER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_isd::TRIBUTE_TO_HUNGER,
    "11f1b897-bb7d-4217-97a7-c89f2453c8fa",
    "Dave Kendall",
);

// FDN 615 — Vampiric Rites (reprint)
const VAMPIRIC_RITES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_bfz::VAMPIRIC_RITES,
    "9ae0b1e0-a481-4a98-b67b-0cf8bac53fdd",
    "Anastasia Ovchinnikova",
);

// FDN 616 — Vile Entomber (reprint)
const VILE_ENTOMBER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mh2::VILE_ENTOMBER,
    "833b5a32-5d77-4e46-a524-25bada29ef59",
    "Chris Cold",
);

// FDN 617 — Wishclaw Talisman (reprint)
const WISHCLAW_TALISMAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_eld::WISHCLAW_TALISMAN,
    "69d0f5bd-ccea-49b2-bd79-ad5e4d850cf5",
    "Daarken",
);

// FDN 618 — Ball Lightning (reprint)
const BALL_LIGHTNING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_drk::BALL_LIGHTNING,
    "5f27dbf0-6818-40ea-832d-10686b4c2900",
    "Trevor Claxton",
);

// FDN 619 — Bolt Bend (reprint)
const BOLT_BEND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_war::BOLT_BEND,
    "cf01af2c-68e5-4bb5-81de-f3a1b860fb2e",
    "Svetlin Velinov",
);

// FDN 620 — Crash Through (reprint)
const CRASH_THROUGH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_hou::CRASH_THROUGH,
    "9eb16918-6363-4849-8d74-e26822a0ddf7",
    "Izzy",
);

// FDN 621 — Dragon Mage (reprint)
const DRAGON_MAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_scg::DRAGON_MAGE,
    "5525cfde-a92b-4fb5-8f15-a05efebecd3d",
    "Matthew D. Wilson",
);

// FDN 622 — Dragonmaster Outcast (reprint)
const DRAGONMASTER_OUTCAST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wwk::DRAGONMASTER_OUTCAST,
    "b720f3a8-f38f-4460-86a3-dad541e7add7",
    "Raymond Swanland",
);

// FDN 623 — Ghitu Lavarunner (reprint)
const GHITU_LAVARUNNER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dom::GHITU_LAVARUNNER,
    "8940d76d-c09f-4d72-a037-439c70ee8d9d",
    "Jesper Ejsing",
);

// FDN 624 — Giant Cindermaw (reprint)
const GIANT_CINDERMAW_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_bro::GIANT_CINDERMAW,
    "d65f5298-0e57-49cf-aaf5-2bb16f3e636c",
    "Edgar Sánchez Hidalgo",
);

// FDN 625 — Harmless Offering (reprint)
const HARMLESS_OFFERING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_emn::HARMLESS_OFFERING,
    "47082081-bf52-4914-bac2-ace398beff56",
    "Howard Lyon",
);

// FDN 626 — Hoarding Dragon (reprint)
const HOARDING_DRAGON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m11::HOARDING_DRAGON,
    "c3a35b3b-0c92-4b40-8210-86a5b5f275eb",
    "Matt Cavotta",
);

// FDN 627 — Lathliss, Dragon Queen (reprint)
const LATHLISS_DRAGON_QUEEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m19::LATHLISS_DRAGON_QUEEN,
    "409acb8f-cf03-4a56-a8c0-e4c97a01ee10",
    "Alex Konstad",
);

// FDN 628 — Mindsparker (reprint)
const MINDSPARKER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m14::MINDSPARKER,
    "4c8b3e58-be07-451e-a5c7-61c70cc3a5a2",
    "Wayne Reynolds",
);

// FDN 629 — Obliterating Bolt (reprint)
const OBLITERATING_BOLT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_bro::OBLITERATING_BOLT,
    "d59e836b-1c7b-43ac-889b-5f94e1638c42",
    "Campbell White",
);

// FDN 630 — Ravenous Giant (reprint)
const RAVENOUS_GIANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mh1::RAVENOUS_GIANT,
    "050b70c4-9086-4db4-8e65-f9bb5f67cf1b",
    "Milivoj Ćeran",
);

// FDN 631 — Redcap Gutter-Dweller (reprint)
const REDCAP_GUTTER_DWELLER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_woe::REDCAP_GUTTER_DWELLER,
    "c6c7f55a-bcc7-4ca3-853d-fc6e260d9429",
    "Alexey Kruglov",
);

// FDN 632 — Stromkirk Noble (reprint)
const STROMKIRK_NOBLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_isd::STROMKIRK_NOBLE,
    "0f306c7e-cacc-4c26-a3f1-fad4f3ff7cf3",
    "James Ryman",
);

// FDN 633 — Taurean Mauler (reprint)
const TAUREAN_MAULER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mor::TAUREAN_MAULER,
    "fca7caa5-d920-4f06-90bd-214aa4e85cb3",
    "Dominick Domingo",
);

// FDN 634 — Viashino Pyromancer (reprint)
const VIASHINO_PYROMANCER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m19::VIASHINO_PYROMANCER,
    "9f52ada2-cabc-46a3-99df-271833a86909",
    "Jesper Ejsing",
);

// FDN 635 — Circuitous Route (reprint)
const CIRCUITOUS_ROUTE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_grn::CIRCUITOUS_ROUTE,
    "a806dec2-d4f9-4f8d-a1dd-170777941131",
    "Milivoj Ćeran",
);

// FDN 636 — Fierce Empath (reprint)
const FIERCE_EMPATH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_scg::FIERCE_EMPATH,
    "28a1dd7f-2e42-4062-a941-b489b98a49fc",
    "Johann Bodin",
);

// FDN 637 — Fynn, the Fangbearer (reprint)
const FYNN_THE_FANGBEARER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_khm::FYNN_THE_FANGBEARER,
    "699efb9e-2649-432b-8b2d-10775114c314",
    "Lie Setiawan",
);

// FDN 638 — Gnarlback Rhino (reprint)
const GNARLBACK_RHINO_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m20::GNARLBACK_RHINO,
    "6b638f3d-7eb6-4675-9538-4a87a7f75c43",
    "YW Tang",
);

// FDN 639 — Heroes' Bane (reprint)
const HEROES_BANE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_jou::HEROES_BANE,
    "7f83195c-5150-4763-a2e1-5b109d185d55",
    "Raymond Swanland",
);

// FDN 640 — Mold Adder (reprint)
const MOLD_ADDER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m10::MOLD_ADDER,
    "48d665cd-a2c6-4b81-8032-379e476b5ea5",
    "Matt Cavotta",
);

// FDN 641 — Ordeal of Nylea (reprint)
const ORDEAL_OF_NYLEA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ths::ORDEAL_OF_NYLEA,
    "1a70424d-86a7-44a3-acda-4463d7ac503b",
    "David Palumbo",
);

// FDN 642 — Predator Ooze (reprint)
const PREDATOR_OOZE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dka::PREDATOR_OOZE,
    "333b3cca-ebbf-4ceb-a6a5-3d49cb2e143a",
    "Ryan Yee",
);

// FDN 643 — Primal Might (reprint)
const PRIMAL_MIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m21::PRIMAL_MIGHT,
    "9ae57d6b-15e8-4f2a-97c2-76d801a1a42a",
    "Randy Vargas",
);

// FDN 644 — Primeval Bounty (reprint)
const PRIMEVAL_BOUNTY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m14::PRIMEVAL_BOUNTY,
    "332c9742-dc3b-48e5-8736-7724fae1b4c4",
    "Christine Choi",
);

// FDN 645 — Rampaging Baloths (reprint)
const RAMPAGING_BALOTHS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_zen::RAMPAGING_BALOTHS,
    "c25ee47b-d099-4788-9c2b-73d151bf56fb",
    "Steve Prescott",
);

// FDN 646 — Springbloom Druid (reprint)
const SPRINGBLOOM_DRUID_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mh1::SPRINGBLOOM_DRUID,
    "fa87cb5f-4dc9-49a4-9ae6-ebc7fedac018",
    "Randy Gallegos",
);

// FDN 647 — Surrak, the Hunt Caller (reprint)
const SURRAK_THE_HUNT_CALLER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dtk::SURRAK_THE_HUNT_CALLER,
    "d6a8759d-0dc8-4218-93ac-b1a35088f776",
    "Wesley Burt",
);

// FDN 648 — Venom Connoisseur (reprint)
const VENOM_CONNOISSEUR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_snc::VENOM_CONNOISSEUR,
    "90f8d876-3697-4359-9f2b-6053682b556c",
    "Marta Nael",
);

// FDN 649 — Vizier of the Menagerie (reprint)
const VIZIER_OF_THE_MENAGERIE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_akh::VIZIER_OF_THE_MENAGERIE,
    "3010ec33-c3f6-42ce-ad5a-e149e5c9a805",
    "Victor Adame Minguez",
);

// FDN 650 — Wildborn Preserver (reprint)
const WILDBORN_PRESERVER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_eld::WILDBORN_PRESERVER,
    "8a249d1c-a5fe-48e5-bd9b-e50d8eea391b",
    "Lius Lasahido",
);

// FDN 651 — Aurelia, the Warleader (reprint)
const AURELIA_THE_WARLEADER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_gtc::AURELIA_THE_WARLEADER,
    "bc6ffc1c-575b-4116-83c9-d13b29886c35",
    "Slawomir Maniak",
);

// FDN 652 — Ayli, Eternal Pilgrim (reprint)
const AYLI_ETERNAL_PILGRIM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ogw::AYLI_ETERNAL_PILGRIM,
    "06969031-2c13-4b5a-bb4e-86eed025e614",
    "Cynthia Sheppard",
);

// FDN 653 — Cloudblazer (reprint)
const CLOUDBLAZER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_kld::CLOUDBLAZER,
    "a9c966ab-0d92-4fa7-8a91-1f77089d8fc7",
    "Dan Murayama Scott",
);

// FDN 654 — Deadly Brew (reprint)
const DEADLY_BREW_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_stx::DEADLY_BREW,
    "4cd4d5a4-4342-49fb-b1e4-7d08e6cf5376",
    "Randy Vargas",
);

// FDN 655 — Drogskol Reaver (reprint)
const DROGSKOL_REAVER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dka::DROGSKOL_REAVER,
    "c0a4450c-8dfd-4d05-adb9-e84ec6066c7d",
    "Vincent Proce",
);

// FDN 656 — Dryad Militant (reprint)
const DRYAD_MILITANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rtr::DRYAD_MILITANT,
    "8fb36712-26e9-4ec7-8946-626bb7094c15",
    "Aaron J. Riley",
);

// FDN 657 — Enigma Drake (reprint)
const ENIGMA_DRAKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_akh::ENIGMA_DRAKE,
    "13ce1338-5a46-4d96-a991-d5fa8d4330ae",
    "Steve Argyle",
);

// FDN 658 — Garna, Bloodfist of Keld (reprint)
const GARNA_BLOODFIST_OF_KELD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dmu::GARNA_BLOODFIST_OF_KELD,
    "5d7fcd03-fffe-467a-88a5-e583e32afd7a",
    "Andrey Kuzinskiy",
);

// FDN 659 — Halana and Alena, Partners (reprint)
const HALANA_AND_ALENA_PARTNERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_vow::HALANA_AND_ALENA_PARTNERS,
    "b649f459-c9dc-495d-8703-b685996bb80c",
    "Jason Rainville",
);

// FDN 660 — Immersturm Predator (reprint)
const IMMERSTURM_PREDATOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_khm::IMMERSTURM_PREDATOR,
    "52dfc234-e87a-4594-8dcf-0a2f95eedc97",
    "Nicholas Gregory",
);

// FDN 661 — Maelstrom Pulse (reprint)
const MAELSTROM_PULSE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arb::MAELSTROM_PULSE,
    "019e8d47-a086-4e9f-8130-8c3b2dc50179",
    "John Avon",
);

// FDN 662 — Mortify (reprint)
const MORTIFY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_gpt::MORTIFY,
    "88e85619-3a7d-48c2-b9b0-20718c913696",
    "Nils Hamm",
);

// FDN 663 — Ovika, Enigma Goliath (reprint)
const OVIKA_ENIGMA_GOLIATH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_one::OVIKA_ENIGMA_GOLIATH,
    "54819d7a-eec6-49e6-a0bb-d9d07d42b4a6",
    "Antonio José Manzanedo",
);

// FDN 664 — Prime Speaker Zegana (reprint)
const PRIME_SPEAKER_ZEGANA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_gtc::PRIME_SPEAKER_ZEGANA,
    "d2f007b0-b578-44f8-be65-cd9e2ac56e09",
    "Willian Murai",
);

// FDN 665 — Savage Ventmaw (reprint)
const SAVAGE_VENTMAW_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dtk::SAVAGE_VENTMAW,
    "50027d69-8bb0-444c-8e9b-4c9afd91cbda",
    "Slawomir Maniak",
);

// FDN 666 — Teach by Example (reprint)
const TEACH_BY_EXAMPLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_stx::TEACH_BY_EXAMPLE,
    "516baae5-f7bf-4a90-a80d-192ff19dbae5",
    "Johan Grenier",
);

// FDN 667 — Trygon Predator (reprint)
const TRYGON_PREDATOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dis::TRYGON_PREDATOR,
    "cdb88a22-8086-4bf7-89e8-cce929440dd8",
    "Carl Critchlow",
);

// FDN 668 — Wilt-Leaf Liege (reprint)
const WILT_LEAF_LIEGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_shm::WILT_LEAF_LIEGE,
    "4d383bbf-6bb1-4c2c-899b-65d8df9d0889",
    "Jason Chan",
);

// FDN 669 — Basilisk Collar (reprint)
const BASILISK_COLLAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wwk::BASILISK_COLLAR,
    "7b36fba7-71f7-4b7f-bde5-b3a9752ad21c",
    "Craig J Spearing",
);

// FDN 670 — Cultivator's Caravan (reprint)
const CULTIVATOR_S_CARAVAN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_kld::CULTIVATOR_S_CARAVAN,
    "249aa94c-85ab-4606-aa2c-c902bd83ac21",
    "Mark Zug",
);

// FDN 671 — Darksteel Colossus (reprint)
const DARKSTEEL_COLOSSUS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dst::DARKSTEEL_COLOSSUS,
    "70197723-c61b-4600-b61d-41380c8f3067",
    "Carl Critchlow",
);

// FDN 672 — Diamond Mare (reprint)
const DIAMOND_MARE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m19::DIAMOND_MARE,
    "721463ad-9531-487f-bd82-5e638c4fc35f",
    "Alayna Danner",
);

// FDN 673 — Feldon's Cane (reprint)
const FELDONS_CANE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_atq::FELDONS_CANE,
    "026f55c3-8e65-4520-b26d-6208344f73b2",
    "Warren Mahy",
);

// FDN 674 — Fireshrieker (reprint)
const FIRESHRIEKER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mrd::FIRESHRIEKER,
    "484fbce6-71bd-40eb-a71b-86958a094708",
    "Christopher Moeller",
);

// FDN 675 — Gate Colossus (reprint)
const GATE_COLOSSUS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rna::GATE_COLOSSUS,
    "bc49d9cb-8c7a-4a8f-96ea-bbe6ade8ae00",
    "Izzy",
);

// FDN 676 — Mazemind Tome (reprint)
const MAZEMIND_TOME_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m21::MAZEMIND_TOME,
    "f45072cd-e3f2-4090-b984-50ec8d360bf2",
    "Randy Gallegos",
);

// FDN 677 — Pyromancer's Goggles (reprint)
const PYROMANCER_S_GOGGLES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ori::PYROMANCER_S_GOGGLES,
    "f340d8a4-196a-4433-807c-b7124e96e44f",
    "Kevin Sidharta",
);

// FDN 678 — Ramos, Dragon Engine (reprint)
const RAMOS_DRAGON_ENGINE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_c17::RAMOS_DRAGON_ENGINE,
    "8f95f753-bbaa-4282-9cd9-f1737136fc9e",
    "Joseph Meehan",
);

// FDN 679 — Sorcerous Spyglass (reprint)
const SORCEROUS_SPYGLASS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_xln::SORCEROUS_SPYGLASS,
    "0f99356b-eed0-4d92-818b-80754bcb75f3",
    "Kieran Yanner",
);

// FDN 680 — Soul-Guide Lantern (reprint)
const SOUL_GUIDE_LANTERN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_thb::SOUL_GUIDE_LANTERN,
    "6d3ff537-86f0-405e-b96b-1250720af031",
    "Iris Compiet",
);

// FDN 681 — Steel Hellkite (reprint)
const STEEL_HELLKITE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_som::STEEL_HELLKITE,
    "931ad21b-a6bb-4a89-8d6f-80adfcf126c7",
    "Jaime Jones",
);

// FDN 682 — Three Tree Mascot (reprint)
const THREE_TREE_MASCOT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_blb::THREE_TREE_MASCOT,
    "40b8bf3a-1cb5-4ce2-ac25-9410f17130de",
    "Gina Matarazzo",
);

// FDN 683 — Azorius Guildgate (reprint)
const AZORIUS_GUILDGATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rtr::AZORIUS_GUILDGATE,
    "f98a7264-0a83-42c8-a94d-05ad4c234242",
    "Drew Baker",
);

// FDN 684 — Boros Guildgate (reprint)
const BOROS_GUILDGATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_gtc::BOROS_GUILDGATE,
    "3e3c74ea-40e9-4ad9-a491-c208403b68ad",
    "Titus Lunter",
);

// FDN 685 — Crawling Barrens (reprint)
const CRAWLING_BARRENS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_znr::CRAWLING_BARRENS,
    "ac2aff0e-1319-4d3b-903c-fa1ce3db7602",
    "Jonas De Ro",
);

// FDN 686 — Cryptic Caves (reprint)
const CRYPTIC_CAVES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m20::CRYPTIC_CAVES,
    "a7976098-b560-458a-ad3c-18f7873add21",
    "Sung Choi",
);

// FDN 687 — Demolition Field (reprint)
const DEMOLITION_FIELD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_bro::DEMOLITION_FIELD,
    "0c7e51b6-4898-4632-b39c-3ce438caa882",
    "Kamila Szutenberg",
);

// FDN 688 — Dimir Guildgate (reprint)
const DIMIR_GUILDGATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_gtc::DIMIR_GUILDGATE,
    "f9b8a159-5e58-4432-8ecd-62f39afa96da",
    "Cliff Childs",
);

// FDN 689 — Golgari Guildgate (reprint)
const GOLGARI_GUILDGATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rtr::GOLGARI_GUILDGATE,
    "92d4646c-a375-4835-aa58-8bb77d1a5abf",
    "Eytan Zana",
);

// FDN 690 — Gruul Guildgate (reprint)
const GRUUL_GUILDGATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_gtc::GRUUL_GUILDGATE,
    "3ab6c240-c97d-4a5c-bc39-860c2d9901c2",
    "Randy Gallegos",
);

// FDN 691 — Izzet Guildgate (reprint)
const IZZET_GUILDGATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rtr::IZZET_GUILDGATE,
    "db9e6fc9-813d-4a71-8a68-8e0f83fa945d",
    "Kirsten Zirngibl",
);

// FDN 692 — Orzhov Guildgate (reprint)
const ORZHOV_GUILDGATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_gtc::ORZHOV_GUILDGATE,
    "a917be03-0c17-4454-b044-c4375e5c8085",
    "John Avon",
);

// FDN 693 — Rakdos Guildgate (reprint)
const RAKDOS_GUILDGATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rtr::RAKDOS_GUILDGATE,
    "e1f01964-c610-4d0f-a2b4-f52e46dc50d2",
    "Jonas De Ro",
);

// FDN 694 — Selesnya Guildgate (reprint)
const SELESNYA_GUILDGATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rtr::SELESNYA_GUILDGATE,
    "6718d4e7-768e-473f-8064-a68422e977f6",
    "Dimitar Marinski",
);

// FDN 695 — Simic Guildgate (reprint)
const SIMIC_GUILDGATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_gtc::SIMIC_GUILDGATE,
    "96590855-1ee5-4d69-9070-776e23f71976",
    "Svetlin Velinov",
);

// FDN 696 — Temple of Abandon (reprint)
const TEMPLE_OF_ABANDON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ths::TEMPLE_OF_ABANDON,
    "8a990a92-8d0f-489f-ae97-68c90fc5ccf1",
    "Adam Paquette",
);

// FDN 697 — Temple of Deceit (reprint)
const TEMPLE_OF_DECEIT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ths::TEMPLE_OF_DECEIT,
    "e18748ce-e52e-4cd1-89d4-cd2578a0d574",
    "Jonas De Ro",
);

// FDN 698 — Temple of Enlightenment (reprint)
const TEMPLE_OF_ENLIGHTENMENT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_bng::TEMPLE_OF_ENLIGHTENMENT,
    "01f23c1a-a0fe-4520-88ed-045aa4044567",
    "Piotr Dura",
);

// FDN 699 — Temple of Epiphany (reprint)
const TEMPLE_OF_EPIPHANY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_jou::TEMPLE_OF_EPIPHANY,
    "224e8255-7bc6-44a2-86af-14f8446f4f77",
    "Adam Paquette",
);

// FDN 700 — Temple of Malady (reprint)
const TEMPLE_OF_MALADY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_jou::TEMPLE_OF_MALADY,
    "10e7d0d5-2329-42a6-b6ff-a5197ba5f1d0",
    "Titus Lunter",
);

// FDN 701 — Temple of Malice (reprint)
const TEMPLE_OF_MALICE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_bng::TEMPLE_OF_MALICE,
    "d78eccc4-1500-43f1-94f5-b39c160b6381",
    "Jonas De Ro",
);

// FDN 702 — Temple of Mystery (reprint)
const TEMPLE_OF_MYSTERY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ths::TEMPLE_OF_MYSTERY,
    "bd581f1d-d8ba-47f9-b10b-b7b6d46239ce",
    "Piotr Dura",
);

// FDN 703 — Temple of Plenty (reprint)
const TEMPLE_OF_PLENTY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_bng::TEMPLE_OF_PLENTY,
    "69762774-c8e1-42de-90b3-5a4d50f5d84d",
    "Chris Ostrowski",
);

// FDN 704 — Temple of Silence (reprint)
const TEMPLE_OF_SILENCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ths::TEMPLE_OF_SILENCE,
    "408ad759-052a-4949-9adf-7541cb2ef77e",
    "Adam Paquette",
);

// FDN 705 — Temple of Triumph (reprint)
const TEMPLE_OF_TRIUMPH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ths::TEMPLE_OF_TRIUMPH,
    "d2f58450-838d-4404-a68f-159e82b0e58d",
    "Piotr Dura",
);

// FDN 706 — Angel of Vitality (reprint)
const ANGEL_OF_VITALITY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m20::ANGEL_OF_VITALITY,
    "c279947e-168f-4af5-902b-aba724f52b8a",
    "Johannes Voss",
);

// FDN 707 — Lyra Dawnbringer (reprint)
const LYRA_DAWNBRINGER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dom::LYRA_DAWNBRINGER,
    "b2abce4d-ef21-4028-8a86-b7d1387bc937",
    "Chris Rahn",
);

// FDN 708 — Make a Stand (reprint)
const MAKE_A_STAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ogw::MAKE_A_STAND,
    "9c26415d-9e98-480b-9e30-b3aed00d5f3d",
    "Magali Villeneuve",
);

// FDN 709 — Confiscate (reprint)
const CONFISCATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::CONFISCATE,
    "a98a1553-e5f3-4c9c-833d-579e82e1708f",
    "Caroline Gariba",
);

// FDN 710 — Negate (reprint)
const NEGATE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mor::NEGATE,
    "ff3b0dba-0207-4249-bdab-e807c76ce39e",
    "Magali Villeneuve",
);

// FDN 711 — Rite of Replication (reprint)
const RITE_OF_REPLICATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_zen::RITE_OF_REPLICATION,
    "fe98958d-30ee-4a47-aacc-064e02d109b3",
    "Matt Cavotta",
);

// FDN 712 — Feed the Swarm (reprint)
const FEED_THE_SWARM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_znr::FEED_THE_SWARM,
    "e66d4541-160c-4137-98e7-0eaa692c7d7a",
    "Andrey Kuzinskiy",
);

// FDN 713 — Gatekeeper of Malakir (reprint)
const GATEKEEPER_OF_MALAKIR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_zen::GATEKEEPER_OF_MALAKIR,
    "2cd61c12-f5e2-4698-9080-097679754f16",
    "Karl Kopinski",
);

// FDN 714 — Massacre Wurm (reprint)
const MASSACRE_WURM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mbs::MASSACRE_WURM,
    "39be2675-986c-4812-9448-99737e797671",
    "Jason Chan",
);

// FDN 715 — Gratuitous Violence (reprint)
const GRATUITOUS_VIOLENCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ons::GRATUITOUS_VIOLENCE,
    "e89024c5-eef5-468c-92b4-e53dd212909c",
    "Jesper Ejsing",
);

// FDN 716 — Guttersnipe (reprint)
const GUTTERSNIPE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rtr::GUTTERSNIPE,
    "dde674d6-f4e7-410b-acf0-34021290576d",
    "Andrey Kuzinskiy",
);

// FDN 717 — Impact Tremors (reprint)
const IMPACT_TREMORS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dtk::IMPACT_TREMORS,
    "d0b7cecf-b51b-4d30-b7e9-cd7976271e07",
    "Lake Hurwitz",
);

// FDN 718 — Gigantosaurus (reprint)
const GIGANTOSAURUS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m19::GIGANTOSAURUS,
    "bc1c518a-dcb4-407c-85be-ed5935a24198",
    "Loïc Canavaggia",
);

// FDN 719 — Imperious Perfect (reprint)
const IMPERIOUS_PERFECT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lrw::IMPERIOUS_PERFECT,
    "83b4a09f-ea53-42b4-a41a-4299192a6fd3",
    "Scott M. Fischer",
);

// FDN 720 — Pelakka Wurm (reprint)
const PELAKKA_WURM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_roe::PELAKKA_WURM,
    "fa150070-80b6-453d-b00f-4462175cac79",
    "Daniel Ljunggren",
);

// FDN 721 — Boros Charm (reprint)
const BOROS_CHARM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_gtc::BOROS_CHARM,
    "e0d8c9f6-cbbd-4694-b100-01cfb81036cc",
    "Zoltan Boros",
);

// FDN 722 — Unflinching Courage (reprint)
const UNFLINCHING_COURAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dgm::UNFLINCHING_COURAGE,
    "a1201875-b3d8-4f95-801e-bf18ee77919b",
    "Mike Bierek",
);

// FDN 723 — Adaptive Automaton (reprint)
const ADAPTIVE_AUTOMATON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m12::ADAPTIVE_AUTOMATON,
    "e35e614c-51a9-4d5c-b7fa-1a0a87108785",
    "Igor Kieryluk",
);

// FDN 724 — Expedition Map (reprint)
const EXPEDITION_MAP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_zen::EXPEDITION_MAP,
    "08e66835-c228-48fa-bcaa-eb96edbd4f5a",
    "Franz Vohwinkel",
);

// FDN 725 — Gilded Lotus (reprint)
const GILDED_LOTUS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mrd::GILDED_LOTUS,
    "aa5e88f1-0ddf-45d7-bbd0-baa88d121867",
    "Volkan Baǵa",
);

// FDN 726 — Hedron Archive (reprint)
const HEDRON_ARCHIVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_bfz::HEDRON_ARCHIVE,
    "535b9ac6-dba2-4162-86ed-df3e9fad0306",
    "Craig J Spearing",
);

// FDN 727 — Maze's End (reprint)
const MAZE_S_END_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dgm::MAZE_S_END,
    "ea9a4d1a-79dd-4b15-8e3b-f111f16d6bfc",
    "Cliff Childs",
);

// FDN 728 — Phyrexian Arena (alternate printing)
const PHYREXIAN_ARENA_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_apc::PHYREXIAN_ARENA,
    3,
    "534d28b7-1e56-428d-84a7-0eb6a2b5efda",
    "Aaron J. Riley",
);

// FDN 729 — Solemn Simulacrum (alternate printing)
const SOLEMN_SIMULACRUM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_mrd::SOLEMN_SIMULACRUM,
    1,
    "b89aae48-a783-468d-aa14-552307ec2021",
    "Forrest Imel",
);

// FDN 730 — Hinterland Sanctifier (reprint)
const HINTERLAND_SANCTIFIER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_j25::HINTERLAND_SANCTIFIER,
    "632df69e-6377-43d0-bba5-65518a320aa5",
    "Justine Cruz",
);

// FDN 731 — Crusader of Odric (alternate printing)
const CRUSADER_OF_ODRIC_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_m13::CRUSADER_OF_ODRIC,
    1,
    "2fd0400a-ef3e-4b03-9852-63e28304da53",
    "Michael Komarck",
);

// FDN 732 — Dazzling Angel (alternate printing)
const DAZZLING_ANGEL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DAZZLING_ANGEL,
    1,
    "a0bdf4d1-576f-41b6-a077-8725be608331",
    "Daneen Wilkerson",
);

// FDN 733 — Exemplar of Light (alternate printing)
const EXEMPLAR_OF_LIGHT_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &EXEMPLAR_OF_LIGHT,
    4,
    "b7393efb-da40-4b55-a6ff-ce774f0815f9",
    "Ekaterina Burmak",
);

// FDN 734 — Healer's Hawk (alternate printing)
const HEALER_S_HAWK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_grn::HEALER_S_HAWK,
    1,
    "069cfaa5-bba4-4503-b54e-b98fa9f0a0fc",
    "Milivoj Ćeran",
);

// FDN 735 — Herald of Faith (alternate printing)
const HERALD_OF_FAITH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_m19::HERALD_OF_FAITH,
    1,
    "2e1705da-dc35-4bcb-82d4-b77712e79af3",
    "Tommy Arnold",
);

// FDN 736 — Inspiring Overseer (alternate printing)
const INSPIRING_OVERSEER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_snc::INSPIRING_OVERSEER,
    1,
    "79016cf3-6eea-4b21-9ff3-f187d606e19a",
    "Irina Nordsol",
);

// FDN 737 — Linden, the Steadfast Queen (alternate printing)
const LINDEN_THE_STEADFAST_QUEEN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_eld::LINDEN_THE_STEADFAST_QUEEN,
    1,
    "e0bb07cd-9b74-4ba7-8089-27a565d74197",
    "Ryan Pancoast",
);

// FDN 738 — Lyra Dawnbringer (alternate printing)
const LYRA_DAWNBRINGER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_dom::LYRA_DAWNBRINGER,
    1,
    "f9fa30b6-3a33-46fd-8b32-ac1cfa41500d",
    "Chris Rahn",
);

// FDN 739 — Prayer of Binding (alternate printing)
const PRAYER_OF_BINDING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_dmu::PRAYER_OF_BINDING,
    1,
    "8d05289c-d8de-4085-ac01-5dd8fd954d35",
    "Wylie Beckert",
);

// FDN 740 — Serra Angel (alternate printing)
const SERRA_ANGEL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SERRA_ANGEL,
    1,
    "b8c5e74c-96e7-4a1f-93b7-14d776fe4b2d",
    "Greg Staples",
);

// FDN 741 — Arcanis the Omnipotent (alternate printing)
const ARCANIS_THE_OMNIPOTENT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ons::ARCANIS_THE_OMNIPOTENT,
    1,
    "9d31e3e3-f398-4c0e-a2c9-4c614a4d41de",
    "Justin Sweet",
);

// FDN 742 — Icewind Elemental (alternate printing)
const ICEWIND_ELEMENTAL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ICEWIND_ELEMENTAL,
    1,
    "c2bfdfb6-aa0b-4bd7-89ba-6435da730e5e",
    "Andrew Mar",
);

// FDN 743 — Kitesail Corsair (alternate printing)
const KITESAIL_CORSAIR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_rix::KITESAIL_CORSAIR,
    1,
    "aa127602-10b2-4144-985b-38e075f1e1f7",
    "Greg Opalinski",
);

// FDN 744 — Mocking Sprite (alternate printing)
const MOCKING_SPRITE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_woe::MOCKING_SPRITE,
    1,
    "d52d16be-c74b-4e40-94f9-2ffa497b6337",
    "Ben Hill",
);

// FDN 745 — Rune-Sealed Wall (alternate printing)
const RUNE_SEALED_WALL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RUNE_SEALED_WALL,
    1,
    "46674bd4-5160-44b2-bd2a-2da85c1e2697",
    "Rockey Chen",
);

// FDN 746 — Spectral Sailor (alternate printing)
const SPECTRAL_SAILOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_m20::SPECTRAL_SAILOR,
    1,
    "5ef30b6c-0806-4bf6-9a84-c2aea8d84cf1",
    "Cristi Balanescu",
);

// FDN 747 — Sphinx of the Final Word (alternate printing)
const SPHINX_OF_THE_FINAL_WORD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ogw::SPHINX_OF_THE_FINAL_WORD,
    1,
    "6071239e-0b85-4c54-bef8-d9456eb8d8fc",
    "Lius Lasahido",
);

// FDN 748 — Strix Lookout (alternate printing)
const STRIX_LOOKOUT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STRIX_LOOKOUT,
    1,
    "3b0c77ab-15cd-49d8-ac5b-7b8b968d14cd",
    "Josiah \"Jo\" Cameron",
);

// FDN 749 — Tempest Djinn (alternate printing)
const TEMPEST_DJINN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_dom::TEMPEST_DJINN,
    1,
    "3de55b97-059c-4d7b-afd0-44fd8380df4c",
    "Zezhou Chen",
);

// FDN 750 — Unsummon (alternate printing)
const UNSUMMON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::UNSUMMON,
    1,
    "2815d8e6-3668-4b9d-abe6-eca6a500707d",
    "Ron Spencer",
);

// FDN 751 — Bloodtithe Collector (alternate printing)
const BLOODTITHE_COLLECTOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_mid::BLOODTITHE_COLLECTOR,
    1,
    "37931135-100d-4a23-a6e3-baf90fb259ee",
    "Maria Zolotukhina",
);

// FDN 752 — Gatekeeper of Malakir (alternate printing)
const GATEKEEPER_OF_MALAKIR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_zen::GATEKEEPER_OF_MALAKIR,
    1,
    "58c45158-7858-409e-ab37-8a9a66ad8714",
    "Karl Kopinski",
);

// FDN 753 — Kalastria Highborn (alternate printing)
const KALASTRIA_HIGHBORN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_wwk::KALASTRIA_HIGHBORN,
    1,
    "a3a055ed-60b0-4d35-ad58-04f324b56d65",
    "D. Alexander Gregory",
);

// FDN 754 — Massacre Wurm (alternate printing)
const MASSACRE_WURM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_mbs::MASSACRE_WURM,
    1,
    "670a36cc-34e1-4d11-808e-1b6bc88eb5d8",
    "Jason Chan",
);

// FDN 755 — Pulse Tracker (alternate printing)
const PULSE_TRACKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_wwk::PULSE_TRACKER,
    1,
    "7bc1441e-7d75-4a74-8733-19077dfa69b2",
    "Andrew Robinson",
);

// FDN 756 — Vampire Interloper (alternate printing)
const VAMPIRE_INTERLOPER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_isd::VAMPIRE_INTERLOPER,
    1,
    "ac01d4eb-10ec-4bda-aa9b-04c74dabfcaf",
    "James Ryman",
);

// FDN 757 — Vampire Nighthawk (alternate printing)
const VAMPIRE_NIGHTHAWK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_zen::VAMPIRE_NIGHTHAWK,
    1,
    "7aff07f9-9528-4149-9af0-f4e3c66c9dc5",
    "Jason Chan",
);

// FDN 758 — Vampire Soulcaller (alternate printing)
const VAMPIRE_SOULCALLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VAMPIRE_SOULCALLER,
    1,
    "4734e3fe-7cd7-4045-983f-f89eae9396fa",
    "Aaron J. Riley",
);

// FDN 759 — Carnelian Orb of Dragonkind (alternate printing)
const CARNELIAN_ORB_OF_DRAGONKIND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_clb::CARNELIAN_ORB_OF_DRAGONKIND,
    1,
    "6db2741d-2722-4eb7-b09d-0d81649c7ca2",
    "Lars Grant-West",
);

// FDN 760 — Drakuseth, Maw of Flames (alternate printing)
const DRAKUSETH_MAW_OF_FLAMES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_m20::DRAKUSETH_MAW_OF_FLAMES,
    1,
    "9e9b40f5-3dd0-4607-a78d-b004d57fa4ea",
    "Grzegorz Rutkowski",
);

// FDN 761 — Firespitter Whelp (alternate printing)
const FIRESPITTER_WHELP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_j25::FIRESPITTER_WHELP,
    1,
    "ddcc3c1b-b564-4444-9a1a-0f62f8e6b8bb",
    "David Álvarez",
);

// FDN 762 — Kargan Dragonrider (alternate printing)
const KARGAN_DRAGONRIDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_m19::KARGAN_DRAGONRIDER,
    1,
    "7023129c-09f5-49cf-ae32-b9bd30e0ff63",
    "Greg Opalinski",
);

// FDN 763 — Shivan Dragon (alternate printing)
const SHIVAN_DRAGON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SHIVAN_DRAGON,
    1,
    "702c4781-670b-49ae-b511-90ed119841b0",
    "Donato Giancola",
);

// FDN 764 — Terror of Mount Velus (alternate printing)
const TERROR_OF_MOUNT_VELUS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_thb::TERROR_OF_MOUNT_VELUS,
    1,
    "4047b1c3-06eb-400d-993d-69b5210977ee",
    "Billy Christian",
);

// FDN 765 — Pelakka Wurm (alternate printing)
const PELAKKA_WURM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_roe::PELAKKA_WURM,
    1,
    "304c635c-de4d-46ee-8ab0-e5c4d55b61b3",
    "Daniel Ljunggren",
);

// FDN 766 — Primal Might (alternate printing)
const PRIMAL_MIGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_m21::PRIMAL_MIGHT,
    1,
    "69b3f7b9-9499-4883-b5c5-c5474e470b21",
    "Randy Vargas",
);

// FDN 767 — Springbloom Druid (alternate printing)
const SPRINGBLOOM_DRUID_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_mh1::SPRINGBLOOM_DRUID,
    1,
    "17d67b88-3f76-401f-b04a-f2bcaab4aa97",
    "Randy Gallegos",
);

// FDN 768 — Surrak, the Hunt Caller (alternate printing)
const SURRAK_THE_HUNT_CALLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_dtk::SURRAK_THE_HUNT_CALLER,
    1,
    "deb328c7-fdda-4d63-9fe4-0482c1de6098",
    "Wesley Burt",
);

// FDN 769 — Vizier of the Menagerie (alternate printing)
const VIZIER_OF_THE_MENAGERIE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_akh::VIZIER_OF_THE_MENAGERIE,
    1,
    "14ab742a-89dc-4c01-99e3-21fc609128f2",
    "Victor Adame Minguez",
);

// FDN 770 — Diamond Mare (alternate printing)
const DIAMOND_MARE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_m19::DIAMOND_MARE,
    1,
    "3c39b0ce-db4c-472c-b68e-07e6ba3a4588",
    "Alayna Danner",
);

// FDN 771 — Cryptic Caves (alternate printing)
const CRYPTIC_CAVES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_m20::CRYPTIC_CAVES,
    1,
    "9d126c8f-a3df-497c-aad5-a448c55f51cc",
    "Sung Choi",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &SIRE_OF_SEVEN_DEATHS,
    &ARAHBO_THE_FIRST_FANG,
    &ARMASAUR_GUIDE,
    &CAT_COLLECTOR,
    &CELESTIAL_ARMOR,
    &CLAWS_OUT,
    &CRYSTAL_BARRICADE,
    &DAUNTLESS_VETERAN,
    &DAZZLING_ANGEL,
    &DIVINE_RESILIENCE,
    &EXEMPLAR_OF_LIGHT,
    &FELIDAR_SAVIOR,
    &FLEETING_FLIGHT,
    &GUARDED_HEIR,
    &HARE_APPARENT,
    &HELPFUL_HUNTER,
    &HERALD_OF_ETERNAL_DAWN,
    &INSPIRING_PALADIN,
    &JOUST_THROUGH,
    &LUMINOUS_REBUKE,
    &PRIDEFUL_PARENT,
    &RAISE_THE_PAST,
    &SKYKNIGHT_SQUIRE,
    &SQUAD_RALLIER,
    &SUN_BLESSED_HEALER,
    &TWINBLADE_BLESSING,
    &VALKYRIE_S_CALL,
    &VANGUARD_SERAPH,
    &ARCANE_EPIPHANY,
    &ARCHMAGE_OF_RUNES,
    &BIGFIN_BOUNCER,
    &CEPHALID_INKMAGE,
    &CLINQUANT_SKYMAGE,
    &CURATOR_OF_DESTINIES,
    &DRAKE_HATCHER,
    &ELEMENTALIST_ADEPT,
    &ERUDITE_WIZARD,
    &FAEBLOOM_TRICK,
    &GRAPPLING_KRAKEN,
    &HIGH_FAE_TRICKSTER,
    &HOMUNCULUS_HORDE,
    &ICEWIND_ELEMENTAL,
    &INSPIRATION_FROM_BEYOND,
    &KAITO_CUNNING_INFILTRATOR,
    &KIORA_THE_RISING_TIDE,
    &LUNAR_INSIGHT,
    &MISCHIEVOUS_MYSTIC,
    &REFUTE,
    &RUNE_SEALED_WALL,
    &SKYSHIP_BUCCANEER,
    &SPHINX_OF_FORGOTTEN_LORE,
    &STRIX_LOOKOUT,
    &UNCHARTED_VOYAGE,
    &ABYSSAL_HARVESTER,
    &ARBITER_OF_WOE,
    &BILLOWING_SHRIEKMASS,
    &BLASPHEMOUS_EDICT,
    &BLOODTHIRSTY_CONQUEROR,
    &CRYPT_FEASTER,
    &GUTLESS_PLUNDERER,
    &HIGH_SOCIETY_HUNTER,
    &HUNGRY_GHOUL,
    &INFERNAL_VESSEL,
    &INFESTATION_SAGE,
    &MIDNIGHT_SNACK,
    &NINE_LIVES_FAMILIAR,
    &REVENGE_OF_THE_RATS,
    &SANGUINE_SYPHONER,
    &SEEKER_S_FOLLY,
    &SOUL_SHACKLED_ZOMBIE,
    &STAB,
    &TINYBONES_BAUBLE_BURGLAR,
    &TRAGIC_BANSHEE,
    &VAMPIRE_GOURMAND,
    &VAMPIRE_SOULCALLER,
    &VENGEFUL_BLOODWITCH,
    &ZUL_ASHUR_LICH_LORD,
    &BATTLESONG_BERSERKER,
    &BOLTWAVE,
    &BULK_UP,
    &CHANDRA_FLAMESHAPER,
    &COURAGEOUS_GOBLIN,
    &CRACKLING_CYCLOPS,
    &DRAGON_TRAINER,
    &ELECTRODUPLICATE,
    &FIERY_ANNIHILATION,
    &GOBLIN_BOARDERS,
    &GOBLIN_NEGOTIATION,
    &GOREHORN_RAIDER,
    &INCINERATING_BLAST,
    &RITE_OF_THE_DRAGONCALLER,
    &SEARSLICER_GOBLIN,
    &SLUMBERING_CERBERUS,
    &SOWER_OF_CHAOS,
    &STRONGBOX_RAIDER,
    &TWINFLAME_TYRANT,
    &AMBUSH_WOLF,
    &APOTHECARY_STOMPER,
    &BEAST_KIN_RANGER,
    &CACKLING_PROWLER,
    &EAGER_TRUFFLESNOUT,
    &ELFSWORN_GIANT,
    &ELVISH_REGROWER,
    &FELLING_BLOW,
    &LOOT_EXUBERANT_EXPLORER,
    &MOSSBORN_HYDRA,
    &NEEDLETOOTH_PACK,
    &PREPOSTEROUS_PROPORTIONS,
    &QUAKESTRIDER_CERATOPS,
    &QUILLED_GREATWURM,
    &SPINNER_OF_SOULS,
    &SYLVAN_SCAVENGING,
    &TREETOP_SNARESPINNER,
    &ALESHA_WHO_LAUGHS_AT_FATE,
    &ANTHEM_OF_CHAMPIONS,
    &ASHROOT_ANIMIST,
    &DREADWING_SCAVENGER,
    &ELENDA_SAINT_OF_DUSK,
    &FIENDISH_PANDA,
    &KOMA_WORLD_EATER,
    &KYKAR_ZEPHYR_AWAKENER,
    &NIV_MIZZET_VISIONARY,
    &PERFORATING_ARTIST,
    &WARDENS_OF_THE_CYCLE,
    &ZIMONE_PARADOX_SCULPTOR,
    &BANNER_OF_KINSHIP,
    &FISHING_POLE,
    &LEYLINE_AXE,
    &QUICK_DRAW_KATANA,
    &RAVENOUS_AMULET,
    &SCRAWLING_CRAWLER,
    &SOULSTONE_SANCTUARY,
    &GOBLIN_SURPRISE,
    &KELLAN_PLANAR_TRAILBLAZER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    KELLAN_PLANAR_TRAILBLAZER_ALTERNATE_1,
    AJANI_CALLER_OF_THE_PRIDE_REPRINT,
    AJANI_S_PRIDEMATE_REPRINT,
    ANGEL_OF_FINALITY_REPRINT,
    AUTHORITY_OF_THE_CONSULS_REPRINT,
    BANISHING_LIGHT_REPRINT,
    CATHAR_COMMANDO_REPRINT,
    DAY_OF_JUDGMENT_REPRINT,
    GIADA_FONT_OF_HOPE_REPRINT,
    HEALER_S_HAWK_REPRINT,
    MAKE_YOUR_MOVE_REPRINT,
    MISCHIEVOUS_PUP_REPRINT,
    RESOLUTE_REINFORCEMENTS_REPRINT,
    SAVANNAH_LIONS_REPRINT,
    SERRA_ANGEL_REPRINT,
    STROKE_OF_MIDNIGHT_REPRINT,
    YOUTHFUL_VALKYRIE_REPRINT,
    AEGIS_TURTLE_REPRINT,
    AETHERIZE_REPRINT,
    BRINEBORN_CUTTHROAT_REPRINT,
    ESSENCE_SCATTER_REPRINT,
    EXTRAVAGANT_REPLICATION_REPRINT,
    FLEETING_DISTRACTION_REPRINT,
    IMPRISONED_IN_THE_MOON_REPRINT,
    LIGHTSHELL_DUO_REPRINT,
    MICROMANCER_REPRINT,
    MOCKING_SPRITE_REPRINT,
    AN_OFFER_YOU_CAN_T_REFUSE_REPRINT,
    OMNISCIENCE_REPRINT,
    RUN_AWAY_TOGETHER_REPRINT,
    SELF_REFLECTION_REPRINT,
    SPECTRAL_SAILOR_REPRINT,
    THINK_TWICE_REPRINT,
    TIME_STOP_REPRINT,
    TOLARIAN_TERROR_REPRINT,
    WITNESS_PROTECTION_REPRINT,
    BAKE_INTO_A_PIE_REPRINT,
    BURGLAR_RAT_REPRINT,
    DIREGRAF_GHOUL_REPRINT,
    EATEN_ALIVE_REPRINT,
    EXSANGUINATE_REPRINT,
    FAKE_YOUR_OWN_DEATH_REPRINT,
    HERO_S_DOWNFALL_REPRINT,
    LILIANA_DREADHORDE_GENERAL_REPRINT,
    MACABRE_WALTZ_REPRINT,
    MARAUDING_BLIGHT_PRIEST_REPRINT,
    PAINFUL_QUANDARY_REPRINT,
    PHYREXIAN_ARENA_REPRINT,
    PILFER_REPRINT,
    REASSEMBLING_SKELETON_REPRINT,
    RISE_OF_THE_DARK_REALMS_REPRINT,
    RUNE_SCARRED_DEMON_REPRINT,
    STROMKIRK_BLOODTHIEF_REPRINT,
    VAMPIRE_NIGHTHAWK_REPRINT,
    ZOMBIFY_REPRINT,
    ABRADE_REPRINT,
    AXGARD_CAVALRY_REPRINT,
    BRASS_S_BOUNTY_REPRINT,
    BRAZEN_SCOURGE_REPRINT,
    BURST_LIGHTNING_REPRINT,
    DRAKUSETH_MAW_OF_FLAMES_REPRINT,
    ETALI_PRIMAL_STORM_REPRINT,
    FANATICAL_FIREBRAND_REPRINT,
    FIREBRAND_ARCHER_REPRINT,
    FIRESPITTER_WHELP_REPRINT,
    FLAMEWAKE_PHOENIX_REPRINT,
    FRENZIED_GOBLIN_REPRINT,
    HEARTFIRE_IMMOLATOR_REPRINT,
    HIDETSUGU_S_SECOND_RITE_REPRINT,
    INVOLUNTARY_EMPLOYMENT_REPRINT,
    KRENKO_MOB_BOSS_REPRINT,
    SEISMIC_RUPTURE_REPRINT,
    SHIVAN_DRAGON_REPRINT,
    SLAGSTORM_REPRINT,
    SPITFIRE_LAGAC_REPRINT,
    SURE_STRIKE_REPRINT,
    THRILL_OF_POSSIBILITY_REPRINT,
    AFFECTIONATE_INDRIK_REPRINT,
    BITE_DOWN_REPRINT,
    BLANCHWOOD_ARMOR_REPRINT,
    BROKEN_WINGS_REPRINT,
    BUSHWHACK_REPRINT,
    DOUBLING_SEASON_REPRINT,
    DWYNEN_GILT_LEAF_DAEN_REPRINT,
    DWYNEN_S_ELITE_REPRINT,
    ELVISH_ARCHDRUID_REPRINT,
    GARRUK_S_UPRISING_REPRINT,
    GENESIS_WAVE_REPRINT,
    GHALTA_PRIMAL_HUNGER_REPRINT,
    GIANT_GROWTH_REPRINT,
    GNARLID_COLONY_REPRINT,
    GROW_FROM_THE_ASHES_REPRINT,
    INSPIRING_CALL_REPRINT,
    LLANOWAR_ELVES_REPRINT,
    MILD_MANNERED_LIBRARIAN_REPRINT,
    NESSIAN_HORNBEETLE_REPRINT,
    OVERRUN_REPRINT,
    RECLAMATION_SAGE_REPRINT,
    SCAVENGING_OOZE_REPRINT,
    SNAKESKIN_VEIL_REPRINT,
    VIVIEN_REID_REPRINT,
    WARY_THESPIAN_REPRINT,
    WILDWOOD_SCOURGE_REPRINT,
    BALMOR_BATTLEMAGE_CAPTAIN_REPRINT,
    CONSUMING_ABERRATION_REPRINT,
    EMPYREAN_EAGLE_REPRINT,
    GOOD_FORTUNE_UNICORN_REPRINT,
    HEROIC_REINFORCEMENTS_REPRINT,
    LATHRIL_BLADE_OF_THE_ELVES_REPRINT,
    MULDROTHA_THE_GRAVETIDE_REPRINT,
    PROGENITUS_REPRINT,
    RUBY_DARING_TRACKER_REPRINT,
    SWIFTBLADE_VINDICATOR_REPRINT,
    TATYOVA_BENTHIC_DRUID_REPRINT,
    THOUSAND_YEAR_STORM_REPRINT,
    ADVENTURING_GEAR_REPRINT,
    BURNISHED_HART_REPRINT,
    CAMPUS_GUIDE_REPRINT,
    GLEAMING_BARRIER_REPRINT,
    GOLDVEIN_PICK_REPRINT,
    HERALDIC_BANNER_REPRINT,
    JUGGERNAUT_REPRINT,
    METEOR_GOLEM_REPRINT,
    SOLEMN_SIMULACRUM_REPRINT,
    SWIFTFOOT_BOOTS_REPRINT,
    BLOODFELL_CAVES_REPRINT,
    BLOSSOMING_SANDS_REPRINT,
    DISMAL_BACKWATER_REPRINT,
    EVOLVING_WILDS_REPRINT,
    JUNGLE_HOLLOW_REPRINT,
    ROGUES_PASSAGE_REPRINT,
    RUGGED_HIGHLANDS_REPRINT,
    SCOURED_BARRENS_REPRINT,
    SECLUDED_COURTYARD_REPRINT,
    SWIFTWATER_CLIFFS_REPRINT,
    THORNWOOD_FALLS_REPRINT,
    TRANQUIL_COVE_REPRINT,
    WIND_SCARRED_CRAG_REPRINT,
    PLAINS_REPRINT,
    PLAINS_ALTERNATE_1,
    ISLAND_REPRINT,
    ISLAND_ALTERNATE_1,
    SWAMP_REPRINT,
    SWAMP_ALTERNATE_1,
    MOUNTAIN_REPRINT,
    MOUNTAIN_ALTERNATE_1,
    FOREST_REPRINT,
    FOREST_ALTERNATE_1,
    PLAINS_ALTERNATE_2,
    PLAINS_ALTERNATE_3,
    ISLAND_ALTERNATE_2,
    ISLAND_ALTERNATE_3,
    SWAMP_ALTERNATE_2,
    SWAMP_ALTERNATE_3,
    MOUNTAIN_ALTERNATE_2,
    MOUNTAIN_ALTERNATE_3,
    FOREST_ALTERNATE_2,
    FOREST_ALTERNATE_3,
    SIRE_OF_SEVEN_DEATHS_ALTERNATE_1,
    AJANI_S_PRIDEMATE_ALTERNATE_1,
    ARAHBO_THE_FIRST_FANG_ALTERNATE_1,
    CELESTIAL_ARMOR_ALTERNATE_1,
    CRYSTAL_BARRICADE_ALTERNATE_1,
    EXEMPLAR_OF_LIGHT_ALTERNATE_1,
    GIADA_FONT_OF_HOPE_ALTERNATE_1,
    HERALD_OF_ETERNAL_DAWN_ALTERNATE_1,
    RAISE_THE_PAST_ALTERNATE_1,
    SKYKNIGHT_SQUIRE_ALTERNATE_1,
    VALKYRIE_S_CALL_ALTERNATE_1,
    YOUTHFUL_VALKYRIE_ALTERNATE_1,
    ARCHMAGE_OF_RUNES_ALTERNATE_1,
    CURATOR_OF_DESTINIES_ALTERNATE_1,
    DRAKE_HATCHER_ALTERNATE_1,
    HIGH_FAE_TRICKSTER_ALTERNATE_1,
    HOMUNCULUS_HORDE_ALTERNATE_1,
    KIORA_THE_RISING_TIDE_ALTERNATE_1,
    LUNAR_INSIGHT_ALTERNATE_1,
    AN_OFFER_YOU_CAN_T_REFUSE_ALTERNATE_1,
    OMNISCIENCE_ALTERNATE_1,
    REFUTE_ALTERNATE_1,
    SPHINX_OF_FORGOTTEN_LORE_ALTERNATE_1,
    THINK_TWICE_ALTERNATE_1,
    ABYSSAL_HARVESTER_ALTERNATE_1,
    BLASPHEMOUS_EDICT_ALTERNATE_1,
    BLOODTHIRSTY_CONQUEROR_ALTERNATE_1,
    HERO_S_DOWNFALL_ALTERNATE_1,
    HIGH_SOCIETY_HUNTER_ALTERNATE_1,
    NINE_LIVES_FAMILIAR_ALTERNATE_1,
    PHYREXIAN_ARENA_ALTERNATE_1,
    RISE_OF_THE_DARK_REALMS_ALTERNATE_1,
    TINYBONES_BAUBLE_BURGLAR_ALTERNATE_1,
    VENGEFUL_BLOODWITCH_ALTERNATE_1,
    ZUL_ASHUR_LICH_LORD_ALTERNATE_1,
    ABRADE_ALTERNATE_1,
    ELECTRODUPLICATE_ALTERNATE_1,
    ETALI_PRIMAL_STORM_ALTERNATE_1,
    RITE_OF_THE_DRAGONCALLER_ALTERNATE_1,
    SEARSLICER_GOBLIN_ALTERNATE_1,
    TWINFLAME_TYRANT_ALTERNATE_1,
    GENESIS_WAVE_ALTERNATE_1,
    GHALTA_PRIMAL_HUNGER_ALTERNATE_1,
    LOOT_EXUBERANT_EXPLORER_ALTERNATE_1,
    MOSSBORN_HYDRA_ALTERNATE_1,
    PREPOSTEROUS_PROPORTIONS_ALTERNATE_1,
    QUILLED_GREATWURM_ALTERNATE_1,
    RECLAMATION_SAGE_ALTERNATE_1,
    SPINNER_OF_SOULS_ALTERNATE_1,
    SYLVAN_SCAVENGING_ALTERNATE_1,
    ALESHA_WHO_LAUGHS_AT_FATE_ALTERNATE_1,
    ANTHEM_OF_CHAMPIONS_ALTERNATE_1,
    ASHROOT_ANIMIST_ALTERNATE_1,
    ELENDA_SAINT_OF_DUSK_ALTERNATE_1,
    KOMA_WORLD_EATER_ALTERNATE_1,
    KYKAR_ZEPHYR_AWAKENER_ALTERNATE_1,
    LATHRIL_BLADE_OF_THE_ELVES_ALTERNATE_1,
    NIV_MIZZET_VISIONARY_ALTERNATE_1,
    ZIMONE_PARADOX_SCULPTOR_ALTERNATE_1,
    BANNER_OF_KINSHIP_ALTERNATE_1,
    LEYLINE_AXE_ALTERNATE_1,
    SCRAWLING_CRAWLER_ALTERNATE_1,
    SWIFTFOOT_BOOTS_ALTERNATE_1,
    SOULSTONE_SANCTUARY_ALTERNATE_1,
    AJANI_CALLER_OF_THE_PRIDE_ALTERNATE_1,
    KAITO_CUNNING_INFILTRATOR_ALTERNATE_1,
    LILIANA_DREADHORDE_GENERAL_ALTERNATE_1,
    CHANDRA_FLAMESHAPER_ALTERNATE_1,
    VIVIEN_REID_ALTERNATE_1,
    SIRE_OF_SEVEN_DEATHS_ALTERNATE_2,
    ARAHBO_THE_FIRST_FANG_ALTERNATE_2,
    CELESTIAL_ARMOR_ALTERNATE_2,
    CRYSTAL_BARRICADE_ALTERNATE_2,
    EXEMPLAR_OF_LIGHT_ALTERNATE_2,
    GIADA_FONT_OF_HOPE_ALTERNATE_2,
    HERALD_OF_ETERNAL_DAWN_ALTERNATE_2,
    RAISE_THE_PAST_ALTERNATE_2,
    SKYKNIGHT_SQUIRE_ALTERNATE_2,
    VALKYRIE_S_CALL_ALTERNATE_2,
    ARCHMAGE_OF_RUNES_ALTERNATE_2,
    CURATOR_OF_DESTINIES_ALTERNATE_2,
    DRAKE_HATCHER_ALTERNATE_2,
    HIGH_FAE_TRICKSTER_ALTERNATE_2,
    HOMUNCULUS_HORDE_ALTERNATE_2,
    KIORA_THE_RISING_TIDE_ALTERNATE_2,
    LUNAR_INSIGHT_ALTERNATE_2,
    OMNISCIENCE_ALTERNATE_2,
    SPHINX_OF_FORGOTTEN_LORE_ALTERNATE_2,
    ABYSSAL_HARVESTER_ALTERNATE_2,
    BLASPHEMOUS_EDICT_ALTERNATE_2,
    BLOODTHIRSTY_CONQUEROR_ALTERNATE_2,
    HIGH_SOCIETY_HUNTER_ALTERNATE_2,
    NINE_LIVES_FAMILIAR_ALTERNATE_2,
    PHYREXIAN_ARENA_ALTERNATE_2,
    RISE_OF_THE_DARK_REALMS_ALTERNATE_2,
    TINYBONES_BAUBLE_BURGLAR_ALTERNATE_2,
    ZUL_ASHUR_LICH_LORD_ALTERNATE_2,
    ELECTRODUPLICATE_ALTERNATE_2,
    ETALI_PRIMAL_STORM_ALTERNATE_2,
    KELLAN_PLANAR_TRAILBLAZER_ALTERNATE_2,
    RITE_OF_THE_DRAGONCALLER_ALTERNATE_2,
    SEARSLICER_GOBLIN_ALTERNATE_2,
    TWINFLAME_TYRANT_ALTERNATE_2,
    GENESIS_WAVE_ALTERNATE_2,
    GHALTA_PRIMAL_HUNGER_ALTERNATE_2,
    LOOT_EXUBERANT_EXPLORER_ALTERNATE_2,
    MOSSBORN_HYDRA_ALTERNATE_2,
    PREPOSTEROUS_PROPORTIONS_ALTERNATE_2,
    QUILLED_GREATWURM_ALTERNATE_2,
    SPINNER_OF_SOULS_ALTERNATE_2,
    SYLVAN_SCAVENGING_ALTERNATE_2,
    ALESHA_WHO_LAUGHS_AT_FATE_ALTERNATE_2,
    ANTHEM_OF_CHAMPIONS_ALTERNATE_2,
    ASHROOT_ANIMIST_ALTERNATE_2,
    ELENDA_SAINT_OF_DUSK_ALTERNATE_2,
    KOMA_WORLD_EATER_ALTERNATE_2,
    KYKAR_ZEPHYR_AWAKENER_ALTERNATE_2,
    LATHRIL_BLADE_OF_THE_ELVES_ALTERNATE_2,
    NIV_MIZZET_VISIONARY_ALTERNATE_2,
    ZIMONE_PARADOX_SCULPTOR_ALTERNATE_2,
    BANNER_OF_KINSHIP_ALTERNATE_2,
    LEYLINE_AXE_ALTERNATE_2,
    SCRAWLING_CRAWLER_ALTERNATE_2,
    SOULSTONE_SANCTUARY_ALTERNATE_2,
    AJANI_CALLER_OF_THE_PRIDE_ALTERNATE_2,
    KAITO_CUNNING_INFILTRATOR_ALTERNATE_2,
    LILIANA_DREADHORDE_GENERAL_ALTERNATE_2,
    CHANDRA_FLAMESHAPER_ALTERNATE_2,
    VIVIEN_REID_ALTERNATE_2,
    DAY_OF_JUDGMENT_ALTERNATE_1,
    HERALD_OF_ETERNAL_DAWN_ALTERNATE_3,
    KAITO_CUNNING_INFILTRATOR_ALTERNATE_3,
    THINK_TWICE_ALTERNATE_2,
    BLOODTHIRSTY_CONQUEROR_ALTERNATE_3,
    TWINFLAME_TYRANT_ALTERNATE_3,
    DOUBLING_SEASON_ALTERNATE_1,
    LLANOWAR_ELVES_ALTERNATE_1,
    MULDROTHA_THE_GRAVETIDE_ALTERNATE_1,
    PROGENITUS_ALTERNATE_1,
    DAY_OF_JUDGMENT_ALTERNATE_2,
    HERALD_OF_ETERNAL_DAWN_ALTERNATE_4,
    KAITO_CUNNING_INFILTRATOR_ALTERNATE_4,
    THINK_TWICE_ALTERNATE_3,
    BLOODTHIRSTY_CONQUEROR_ALTERNATE_4,
    TWINFLAME_TYRANT_ALTERNATE_4,
    DOUBLING_SEASON_ALTERNATE_2,
    LLANOWAR_ELVES_ALTERNATE_2,
    MULDROTHA_THE_GRAVETIDE_ALTERNATE_2,
    PROGENITUS_ALTERNATE_2,
    ARAHBO_THE_FIRST_FANG_ALTERNATE_3,
    CELESTIAL_ARMOR_ALTERNATE_3,
    CRYSTAL_BARRICADE_ALTERNATE_3,
    EXEMPLAR_OF_LIGHT_ALTERNATE_3,
    HERALD_OF_ETERNAL_DAWN_ALTERNATE_5,
    RAISE_THE_PAST_ALTERNATE_3,
    SKYKNIGHT_SQUIRE_ALTERNATE_3,
    VALKYRIE_S_CALL_ALTERNATE_3,
    ARCHMAGE_OF_RUNES_ALTERNATE_3,
    CURATOR_OF_DESTINIES_ALTERNATE_3,
    DRAKE_HATCHER_ALTERNATE_3,
    HIGH_FAE_TRICKSTER_ALTERNATE_3,
    HOMUNCULUS_HORDE_ALTERNATE_3,
    KIORA_THE_RISING_TIDE_ALTERNATE_3,
    LUNAR_INSIGHT_ALTERNATE_3,
    SPHINX_OF_FORGOTTEN_LORE_ALTERNATE_3,
    ABYSSAL_HARVESTER_ALTERNATE_3,
    BLASPHEMOUS_EDICT_ALTERNATE_3,
    BLOODTHIRSTY_CONQUEROR_ALTERNATE_5,
    HIGH_SOCIETY_HUNTER_ALTERNATE_3,
    NINE_LIVES_FAMILIAR_ALTERNATE_3,
    TINYBONES_BAUBLE_BURGLAR_ALTERNATE_3,
    ZUL_ASHUR_LICH_LORD_ALTERNATE_3,
    ELECTRODUPLICATE_ALTERNATE_3,
    KELLAN_PLANAR_TRAILBLAZER_ALTERNATE_3,
    RITE_OF_THE_DRAGONCALLER_ALTERNATE_3,
    SEARSLICER_GOBLIN_ALTERNATE_3,
    TWINFLAME_TYRANT_ALTERNATE_5,
    LOOT_EXUBERANT_EXPLORER_ALTERNATE_3,
    MOSSBORN_HYDRA_ALTERNATE_3,
    PREPOSTEROUS_PROPORTIONS_ALTERNATE_3,
    QUILLED_GREATWURM_ALTERNATE_3,
    SPINNER_OF_SOULS_ALTERNATE_3,
    SYLVAN_SCAVENGING_ALTERNATE_3,
    ALESHA_WHO_LAUGHS_AT_FATE_ALTERNATE_3,
    ANTHEM_OF_CHAMPIONS_ALTERNATE_3,
    ASHROOT_ANIMIST_ALTERNATE_3,
    ELENDA_SAINT_OF_DUSK_ALTERNATE_3,
    KOMA_WORLD_EATER_ALTERNATE_3,
    KYKAR_ZEPHYR_AWAKENER_ALTERNATE_3,
    NIV_MIZZET_VISIONARY_ALTERNATE_3,
    ZIMONE_PARADOX_SCULPTOR_ALTERNATE_3,
    BANNER_OF_KINSHIP_ALTERNATE_3,
    LEYLINE_AXE_ALTERNATE_3,
    SCRAWLING_CRAWLER_ALTERNATE_3,
    SOULSTONE_SANCTUARY_ALTERNATE_3,
    ADAMANT_WILL_REPRINT,
    ANCESTOR_DRAGON_REPRINT,
    ANGELIC_EDICT_REPRINT,
    BISHOP_S_SOLDIER_REPRINT,
    DEADLY_RIPOSTE_REPRINT,
    ELSPETH_S_SMITE_REPRINT,
    HERALD_OF_FAITH_REPRINT,
    INGENIOUS_LEONIN_REPRINT,
    INSPIRING_OVERSEER_REPRINT,
    JAZAL_GOLDMANE_REPRINT,
    LEONIN_SKYHUNTER_REPRINT,
    LEONIN_VANGUARD_REPRINT,
    MOMENT_OF_TRIUMPH_REPRINT,
    PACIFISM_REPRINT,
    PRAYER_OF_BINDING_REPRINT,
    TWINBLADE_PALADIN_REPRINT,
    BURROG_BEFUDDLER_REPRINT,
    CANCEL_REPRINT,
    CORSAIR_CAPTAIN_REPRINT,
    EATEN_BY_PIRANHAS_REPRINT,
    EXCLUSION_MAGE_REPRINT,
    INTO_THE_ROIL_REPRINT,
    KITESAIL_CORSAIR_REPRINT,
    MYSTIC_ARCHAEOLOGIST_REPRINT,
    OPT_REPRINT,
    QUICK_STUDY_REPRINT,
    STARLIGHT_SNARE_REPRINT,
    STORM_FLEET_SPY_REPRINT,
    BLOODTITHE_COLLECTOR_REPRINT,
    CEMETERY_RECRUITMENT_REPRINT,
    CROSSWAY_TROUBLEMAKERS_REPRINT,
    CROW_OF_DARK_TIDINGS_REPRINT,
    DEADLY_PLOT_REPRINT,
    DEATH_BARON_REPRINT,
    HIGHBORN_VAMPIRE_REPRINT,
    MAALFELD_TWINS_REPRINT,
    MOMENT_OF_CRAVING_REPRINT,
    OFFER_IMMORTALITY_REPRINT,
    SKELETON_ARCHER_REPRINT,
    SUSPICIOUS_SHAMBLER_REPRINT,
    UNDYING_MALICE_REPRINT,
    UNTAMED_HUNGER_REPRINT,
    VAMPIRE_INTERLOPER_REPRINT,
    VAMPIRE_NEONATE_REPRINT,
    VAMPIRE_SPAWN_REPRINT,
    BATTLE_RATTLE_SHAMAN_REPRINT,
    CARNELIAN_ORB_OF_DRAGONKIND_REPRINT,
    DRAGON_FODDER_REPRINT,
    DRAGONLORD_S_SERVANT_REPRINT,
    DROPKICK_BOMBER_REPRINT,
    FIRE_ELEMENTAL_REPRINT,
    GOBLIN_ORIFLAMME_REPRINT,
    GOBLIN_SMUGGLER_REPRINT,
    KARGAN_DRAGONRIDER_REPRINT,
    KINDLED_FURY_REPRINT,
    RAGING_REDCAP_REPRINT,
    RAPACIOUS_DRAGON_REPRINT,
    SCORCHING_DRAGONFIRE_REPRINT,
    SEIZE_THE_SPOILS_REPRINT,
    SKYRAKER_GIANT_REPRINT,
    SWAB_GOBLIN_REPRINT,
    TERROR_OF_MOUNT_VELUS_REPRINT,
    VOLLEY_VETERAN_REPRINT,
    AGGRESSIVE_MAMMOTH_REPRINT,
    BEAR_CUB_REPRINT,
    BIOGENIC_UPGRADE_REPRINT,
    DRUID_OF_THE_COWL_REPRINT,
    JORAGA_INVOCATION_REPRINT,
    MAGNIGOTH_SENTRY_REPRINT,
    NEW_HORIZONS_REPRINT,
    TAJURU_PATHWARDEN_REPRINT,
    THORNWEALD_ARCHER_REPRINT,
    THRASHING_BRONTODON_REPRINT,
    WILDHEART_INVOKER_REPRINT,
    GOBLIN_FIREBOMB_REPRINT,
    PIRATE_S_CUTLASS_REPRINT,
    UNCHARTED_HAVEN_REPRINT,
    ANGELIC_DESTINY_REPRINT,
    ARCHWAY_ANGEL_REPRINT,
    BALLYRUSH_BANNERET_REPRINT,
    CHARMING_PRINCE_REPRINT,
    CRUSADER_OF_ODRIC_REPRINT,
    DAWNWING_MARSHAL_REPRINT,
    DEVOUT_DECREE_REPRINT,
    DISENCHANT_REPRINT,
    FELIDAR_CUB_REPRINT,
    FELIDAR_RETREAT_REPRINT,
    FUMIGATE_REPRINT,
    KNIGHT_OF_GRACE_REPRINT,
    LINDEN_THE_STEADFAST_QUEEN_REPRINT,
    MENTOR_OF_THE_MEEK_REPRINT,
    REGAL_CARACAL_REPRINT,
    RELEASE_THE_DOGS_REPRINT,
    STASIS_SNARE_REPRINT,
    SYR_ALIN_THE_LION_S_CLAW_REPRINT,
    VALOROUS_STANCE_REPRINT,
    ZETALPA_PRIMAL_DAWN_REPRINT,
    ARCANIS_THE_OMNIPOTENT_REPRINT,
    CHART_A_COURSE_REPRINT,
    DICTATE_OF_KRUPHIX_REPRINT,
    DIVE_DOWN_REPRINT,
    FINALE_OF_REVELATION_REPRINT,
    FLASHFREEZE_REPRINT,
    FOG_BANK_REPRINT,
    GATEWAY_SNEAK_REPRINT,
    HARBINGER_OF_THE_TIDES_REPRINT,
    MYSTICAL_TEACHINGS_REPRINT,
    RIVER_S_REBUKE_REPRINT,
    SHIPWRECK_DOWSER_REPRINT,
    SPHINX_OF_THE_FINAL_WORD_REPRINT,
    TEMPEST_DJINN_REPRINT,
    UNSUMMON_REPRINT,
    VORACIOUS_GREATSHARK_REPRINT,
    DEATHMARK_REPRINT,
    DEMONIC_PACT_REPRINT,
    DESECRATION_DEMON_REPRINT,
    DREAD_SUMMONS_REPRINT,
    DRIVER_OF_THE_DEAD_REPRINT,
    DURESS_REPRINT,
    KALASTRIA_HIGHBORN_REPRINT,
    KNIGHT_OF_MALICE_REPRINT,
    MIDNIGHT_REAPER_REPRINT,
    MYOJIN_OF_NIGHT_S_REACH_REPRINT,
    NULLPRIEST_OF_OBLIVION_REPRINT,
    PULSE_TRACKER_REPRINT,
    SANGUINE_INDULGENCE_REPRINT,
    TRIBUTE_TO_HUNGER_REPRINT,
    VAMPIRIC_RITES_REPRINT,
    VILE_ENTOMBER_REPRINT,
    WISHCLAW_TALISMAN_REPRINT,
    BALL_LIGHTNING_REPRINT,
    BOLT_BEND_REPRINT,
    CRASH_THROUGH_REPRINT,
    DRAGON_MAGE_REPRINT,
    DRAGONMASTER_OUTCAST_REPRINT,
    GHITU_LAVARUNNER_REPRINT,
    GIANT_CINDERMAW_REPRINT,
    HARMLESS_OFFERING_REPRINT,
    HOARDING_DRAGON_REPRINT,
    LATHLISS_DRAGON_QUEEN_REPRINT,
    MINDSPARKER_REPRINT,
    OBLITERATING_BOLT_REPRINT,
    RAVENOUS_GIANT_REPRINT,
    REDCAP_GUTTER_DWELLER_REPRINT,
    STROMKIRK_NOBLE_REPRINT,
    TAUREAN_MAULER_REPRINT,
    VIASHINO_PYROMANCER_REPRINT,
    CIRCUITOUS_ROUTE_REPRINT,
    FIERCE_EMPATH_REPRINT,
    FYNN_THE_FANGBEARER_REPRINT,
    GNARLBACK_RHINO_REPRINT,
    HEROES_BANE_REPRINT,
    MOLD_ADDER_REPRINT,
    ORDEAL_OF_NYLEA_REPRINT,
    PREDATOR_OOZE_REPRINT,
    PRIMAL_MIGHT_REPRINT,
    PRIMEVAL_BOUNTY_REPRINT,
    RAMPAGING_BALOTHS_REPRINT,
    SPRINGBLOOM_DRUID_REPRINT,
    SURRAK_THE_HUNT_CALLER_REPRINT,
    VENOM_CONNOISSEUR_REPRINT,
    VIZIER_OF_THE_MENAGERIE_REPRINT,
    WILDBORN_PRESERVER_REPRINT,
    AURELIA_THE_WARLEADER_REPRINT,
    AYLI_ETERNAL_PILGRIM_REPRINT,
    CLOUDBLAZER_REPRINT,
    DEADLY_BREW_REPRINT,
    DROGSKOL_REAVER_REPRINT,
    DRYAD_MILITANT_REPRINT,
    ENIGMA_DRAKE_REPRINT,
    GARNA_BLOODFIST_OF_KELD_REPRINT,
    HALANA_AND_ALENA_PARTNERS_REPRINT,
    IMMERSTURM_PREDATOR_REPRINT,
    MAELSTROM_PULSE_REPRINT,
    MORTIFY_REPRINT,
    OVIKA_ENIGMA_GOLIATH_REPRINT,
    PRIME_SPEAKER_ZEGANA_REPRINT,
    SAVAGE_VENTMAW_REPRINT,
    TEACH_BY_EXAMPLE_REPRINT,
    TRYGON_PREDATOR_REPRINT,
    WILT_LEAF_LIEGE_REPRINT,
    BASILISK_COLLAR_REPRINT,
    CULTIVATOR_S_CARAVAN_REPRINT,
    DARKSTEEL_COLOSSUS_REPRINT,
    DIAMOND_MARE_REPRINT,
    FELDONS_CANE_REPRINT,
    FIRESHRIEKER_REPRINT,
    GATE_COLOSSUS_REPRINT,
    MAZEMIND_TOME_REPRINT,
    PYROMANCER_S_GOGGLES_REPRINT,
    RAMOS_DRAGON_ENGINE_REPRINT,
    SORCEROUS_SPYGLASS_REPRINT,
    SOUL_GUIDE_LANTERN_REPRINT,
    STEEL_HELLKITE_REPRINT,
    THREE_TREE_MASCOT_REPRINT,
    AZORIUS_GUILDGATE_REPRINT,
    BOROS_GUILDGATE_REPRINT,
    CRAWLING_BARRENS_REPRINT,
    CRYPTIC_CAVES_REPRINT,
    DEMOLITION_FIELD_REPRINT,
    DIMIR_GUILDGATE_REPRINT,
    GOLGARI_GUILDGATE_REPRINT,
    GRUUL_GUILDGATE_REPRINT,
    IZZET_GUILDGATE_REPRINT,
    ORZHOV_GUILDGATE_REPRINT,
    RAKDOS_GUILDGATE_REPRINT,
    SELESNYA_GUILDGATE_REPRINT,
    SIMIC_GUILDGATE_REPRINT,
    TEMPLE_OF_ABANDON_REPRINT,
    TEMPLE_OF_DECEIT_REPRINT,
    TEMPLE_OF_ENLIGHTENMENT_REPRINT,
    TEMPLE_OF_EPIPHANY_REPRINT,
    TEMPLE_OF_MALADY_REPRINT,
    TEMPLE_OF_MALICE_REPRINT,
    TEMPLE_OF_MYSTERY_REPRINT,
    TEMPLE_OF_PLENTY_REPRINT,
    TEMPLE_OF_SILENCE_REPRINT,
    TEMPLE_OF_TRIUMPH_REPRINT,
    ANGEL_OF_VITALITY_REPRINT,
    LYRA_DAWNBRINGER_REPRINT,
    MAKE_A_STAND_REPRINT,
    CONFISCATE_REPRINT,
    NEGATE_REPRINT,
    RITE_OF_REPLICATION_REPRINT,
    FEED_THE_SWARM_REPRINT,
    GATEKEEPER_OF_MALAKIR_REPRINT,
    MASSACRE_WURM_REPRINT,
    GRATUITOUS_VIOLENCE_REPRINT,
    GUTTERSNIPE_REPRINT,
    IMPACT_TREMORS_REPRINT,
    GIGANTOSAURUS_REPRINT,
    IMPERIOUS_PERFECT_REPRINT,
    PELAKKA_WURM_REPRINT,
    BOROS_CHARM_REPRINT,
    UNFLINCHING_COURAGE_REPRINT,
    ADAPTIVE_AUTOMATON_REPRINT,
    EXPEDITION_MAP_REPRINT,
    GILDED_LOTUS_REPRINT,
    HEDRON_ARCHIVE_REPRINT,
    MAZE_S_END_REPRINT,
    PHYREXIAN_ARENA_ALTERNATE_3,
    SOLEMN_SIMULACRUM_ALTERNATE_1,
    HINTERLAND_SANCTIFIER_REPRINT,
    CRUSADER_OF_ODRIC_ALTERNATE_1,
    DAZZLING_ANGEL_ALTERNATE_1,
    EXEMPLAR_OF_LIGHT_ALTERNATE_4,
    HEALER_S_HAWK_ALTERNATE_1,
    HERALD_OF_FAITH_ALTERNATE_1,
    INSPIRING_OVERSEER_ALTERNATE_1,
    LINDEN_THE_STEADFAST_QUEEN_ALTERNATE_1,
    LYRA_DAWNBRINGER_ALTERNATE_1,
    PRAYER_OF_BINDING_ALTERNATE_1,
    SERRA_ANGEL_ALTERNATE_1,
    ARCANIS_THE_OMNIPOTENT_ALTERNATE_1,
    ICEWIND_ELEMENTAL_ALTERNATE_1,
    KITESAIL_CORSAIR_ALTERNATE_1,
    MOCKING_SPRITE_ALTERNATE_1,
    RUNE_SEALED_WALL_ALTERNATE_1,
    SPECTRAL_SAILOR_ALTERNATE_1,
    SPHINX_OF_THE_FINAL_WORD_ALTERNATE_1,
    STRIX_LOOKOUT_ALTERNATE_1,
    TEMPEST_DJINN_ALTERNATE_1,
    UNSUMMON_ALTERNATE_1,
    BLOODTITHE_COLLECTOR_ALTERNATE_1,
    GATEKEEPER_OF_MALAKIR_ALTERNATE_1,
    KALASTRIA_HIGHBORN_ALTERNATE_1,
    MASSACRE_WURM_ALTERNATE_1,
    PULSE_TRACKER_ALTERNATE_1,
    VAMPIRE_INTERLOPER_ALTERNATE_1,
    VAMPIRE_NIGHTHAWK_ALTERNATE_1,
    VAMPIRE_SOULCALLER_ALTERNATE_1,
    CARNELIAN_ORB_OF_DRAGONKIND_ALTERNATE_1,
    DRAKUSETH_MAW_OF_FLAMES_ALTERNATE_1,
    FIRESPITTER_WHELP_ALTERNATE_1,
    KARGAN_DRAGONRIDER_ALTERNATE_1,
    SHIVAN_DRAGON_ALTERNATE_1,
    TERROR_OF_MOUNT_VELUS_ALTERNATE_1,
    PELAKKA_WURM_ALTERNATE_1,
    PRIMAL_MIGHT_ALTERNATE_1,
    SPRINGBLOOM_DRUID_ALTERNATE_1,
    SURRAK_THE_HUNT_CALLER_ALTERNATE_1,
    VIZIER_OF_THE_MENAGERIE_ALTERNATE_1,
    DIAMOND_MARE_ALTERNATE_1,
    CRYPTIC_CAVES_ALTERNATE_1,
];
