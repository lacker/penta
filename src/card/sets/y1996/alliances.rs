//! Alliances cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::CostQuantityDef;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AlternativeCastKindDef, CardRules, CardSet, CardSupertype, CardType, DividedTotal, EffectDef,
    EffectRecipientDef, InstalledTriggerDef, ManaColor, ObjectPredicateDef, ObjectRefDef,
    ObjectSetDef, PlayerRefDef, PlayerRelation, SpellAdditionalCostDef, SumValueDef,
    TargetChooserDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::{AdditionalCostIndex, TargetIndex, mana_cost};

// ALL 1a — Carrier Pigeons
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CARRIER_PIGEONS: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Carrier Pigeons",
    "5543b08d-d470-435e-83d9-a3a84c1cc2e6",
    "Pat Lewis",
    crate::card::CardRules::unsupported(),
);

// ALL 1b — Carrier Pigeons (alternate printing)
const CARRIER_PIGEONS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CARRIER_PIGEONS,
    1,
    "9d68befe-78bc-4d9c-968b-f7e6b3042f27",
    "Pat Lewis",
);

// ALL 2a — Errand of Duty
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ERRAND_OF_DUTY: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Errand of Duty",
    "6d3c539b-4039-45c2-8d43-80648d946e91",
    "Julie Baroh",
    crate::card::CardRules::unsupported(),
);

// ALL 2b — Errand of Duty (alternate printing)
const ERRAND_OF_DUTY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ERRAND_OF_DUTY,
    1,
    "8a7362e2-8dc0-4d76-a4eb-55ed56b1cd66",
    "Julie Baroh",
);

// ALL 3 — Exile
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXILE: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Exile",
    "108b85ff-ed03-4b3e-872f-1cad1a27b930",
    "Rob Alexander",
    crate::card::CardRules::unsupported(),
);

// ALL 4 — Inheritance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INHERITANCE: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Inheritance",
    "9fe88de7-b226-4a43-9662-8b408e4281d3",
    "Kaja Foglio",
    crate::card::CardRules::unsupported(),
);

// ALL 5 — Ivory Gargoyle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IVORY_GARGOYLE: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Ivory Gargoyle",
    "365820e4-7b43-423b-98ce-f383eb4d2a96",
    "Quinton Hoover",
    crate::card::CardRules::unsupported(),
);

// ALL 6 — Juniper Order Advocate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JUNIPER_ORDER_ADVOCATE: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Juniper Order Advocate",
    "9185d10f-7368-4b40-b4a6-baf46c616c34",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// ALL 7a — Kjeldoran Escort
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KJELDORAN_ESCORT: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Kjeldoran Escort",
    "0fd7536a-5417-4de0-9a48-82a5f82d9af4",
    "Bryon Wackwitz",
    crate::card::CardRules::unsupported(),
);

// ALL 7b — Kjeldoran Escort (alternate printing)
const KJELDORAN_ESCORT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KJELDORAN_ESCORT,
    1,
    "70f54ca5-1f1b-40c7-ad0b-569c54d1b5aa",
    "Bryon Wackwitz",
);

// ALL 8 — Kjeldoran Home Guard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KJELDORAN_HOME_GUARD: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Kjeldoran Home Guard",
    "794d16f9-848f-44ca-8e85-d01a58558077",
    "Andi Rusu",
    crate::card::CardRules::unsupported(),
);

// ALL 9a — Kjeldoran Pride (alternate printing)
const KJELDORAN_PRIDE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KJELDORAN_PRIDE,
    1,
    "d0acdf4d-6fdd-4430-9204-9c80dc8fb387",
    "Kaja Foglio",
);

// ALL 9b — Kjeldoran Pride
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KJELDORAN_PRIDE: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Kjeldoran Pride",
    "a88d1c1a-b53e-459b-8a83-4d559177188a",
    "Kaja Foglio",
    crate::card::CardRules::unsupported(),
);

// ALL 10a — Martyrdom (alternate printing)
const MARTYRDOM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MARTYRDOM,
    1,
    "6ab5775a-7138-4526-83fe-350127695224",
    "Mark Poole",
);

// ALL 10b — Martyrdom
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MARTYRDOM: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Martyrdom",
    "07f91817-8e79-4885-a57b-d26241c4791f",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// ALL 11a — Noble Steeds
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NOBLE_STEEDS: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Noble Steeds",
    "45a35751-a232-40ba-a73b-d3ca7a44867d",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// ALL 11b — Noble Steeds (alternate printing)
const NOBLE_STEEDS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &NOBLE_STEEDS,
    1,
    "684bc5b2-c5fb-4340-9c11-73891adb4b93",
    "Rebecca Guay",
);

// ALL 12a — Reinforcements (alternate printing)
const REINFORCEMENTS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &REINFORCEMENTS,
    1,
    "d734086b-a0fb-4504-96e5-89842aa587b3",
    "Diana Vick",
);

// ALL 12b — Reinforcements
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REINFORCEMENTS: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Reinforcements",
    "c0b26881-3ad7-4d70-8051-a7e222d910bf",
    "Diana Vick",
    crate::card::CardRules::unsupported(),
);

// ALL 13a — Reprisal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REPRISAL: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Reprisal",
    "179f50be-6658-42f4-b9b9-c97c7d3f239a",
    "Randy Asplund-Faith",
    crate::card::CardRules::unsupported(),
);

// ALL 13b — Reprisal (alternate printing)
const REPRISAL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &REPRISAL,
    1,
    "839df85a-1aca-4d4b-b327-2778caa6d289",
    "Randy Asplund-Faith",
);

// ALL 14 — Royal Decree
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROYAL_DECREE: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Royal Decree",
    "d22231f5-30af-4f46-b2c9-0b71124c6939",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// ALL 15a — Royal Herbalist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROYAL_HERBALIST: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Royal Herbalist",
    "027e03e1-1a39-47ba-b206-44d022b4c346",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// ALL 15b — Royal Herbalist (alternate printing)
const ROYAL_HERBALIST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ROYAL_HERBALIST,
    1,
    "6456ee12-7c09-434e-9028-613506ef7ff6",
    "Douglas Shuler",
);

// ALL 16 — Scars of the Veteran
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCARS_OF_THE_VETERAN: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Scars of the Veteran",
    "632870c3-7c0b-48ad-865d-95f8c4e887d0",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// ALL 17 — Seasoned Tactician
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEASONED_TACTICIAN: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Seasoned Tactician",
    "f8be4b6b-23a2-42d2-911d-fa14f7f5a95b",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// ALL 18 — Sustaining Spirit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUSTAINING_SPIRIT: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Sustaining Spirit",
    "c9ecf91a-9ce1-44a1-8859-7163d32cfba6",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// ALL 19 — Sworn Defender
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SWORN_DEFENDER: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Sworn Defender",
    "328e6ceb-30f7-415e-93b4-7075af0fed89",
    "D. Alexander Gregory",
    crate::card::CardRules::unsupported(),
);

// ALL 20 — Unlikely Alliance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNLIKELY_ALLIANCE: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Unlikely Alliance",
    "c14d2c73-1934-4504-bbfb-62ba82e0a0e3",
    "Phil Foglio",
    crate::card::CardRules::unsupported(),
);

// ALL 21a — Wild Aesthir
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WILD_AESTHIR: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Wild Aesthir",
    "dd0decda-d77a-4b7b-8ca4-08528d476f51",
    "Greg Simanson",
    crate::card::CardRules::unsupported(),
);

// ALL 21b — Wild Aesthir (alternate printing)
const WILD_AESTHIR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WILD_AESTHIR,
    1,
    "ffecf097-a5b1-4bd9-ac3f-784bd44e447c",
    "Greg Simanson",
);

// ALL 22a — Arcane Denial
/// "Up to two" is two questions rather than one number: take the first card,
/// then decide about the second. The reachable answers -- none, one, or both
/// -- are the ones the printed card offers.
static DENIED_CONTROLLER: EffectRecipientDef = EffectRecipientDef::player(
    PlayerRefDef::ControllerOf(ObjectRefDef::Target(TargetIndex::PRIMARY)),
);

pub(in crate::card::sets) static ARCANE_DENIAL: CardRecord = CardRecord::new(
    CardSet::Alliances,
    "Arcane Denial",
    "b0c5728e-43e7-417a-ba18-5038345cec67",
    "Richard Kane Ferguson",
    // Two mana to answer anything, and the cards it gives back arrive a turn
    // too late to matter in a deck that is about to lock the game up.
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target spell. Its controller may draw up to two cards at the beginning of the next turn's upkeep.\nYou draw a card at the beginning of the next turn's upkeep.",
        &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::Spell,
            zones: &[ZoneKind::Stack],
            controller: None,
            owner: None,
        })],
        EffectDef::Sequence(&[
            EffectDef::Counter {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Graveyard,
                placement: ZonePlacement::Top,
            },
            // Both draws are delayed to the next upkeep, which is what makes the card a
            // real counterspell rather than a gift: the two cards arrive a turn later,
            // and by then the spell it answered is long gone.
            EffectDef::Sequence(&[
                EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                    "At the beginning of the next turn's upkeep, that spell's controller may draw up to two cards.",
                    TriggerEventDef::StepBegins {
                        step: TurnStepDef::Upkeep,
                        player: PlayerRelation::Any,
                    },
                    EffectDef::May {
                        player: DENIED_CONTROLLER,
                        effect: &EffectDef::Sequence(&[
                            EffectDef::DrawCards {
                                recipient: DENIED_CONTROLLER,
                                amount: ValueDef::Constant(1),
                            },
                            EffectDef::May {
                                player: DENIED_CONTROLLER,
                                effect: &EffectDef::DrawCards {
                                    recipient: DENIED_CONTROLLER,
                                    amount: ValueDef::Constant(1),
                                },
                            },
                        ]),
                    },
                ))),
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
        ]),
    )),
);

// ALL 22b — Arcane Denial (alternate printing)
const ARCANE_DENIAL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ARCANE_DENIAL,
    1,
    "415a3104-90e6-4235-b67f-69337c7fe714",
    "Richard Kane Ferguson",
);

// ALL 23a — Awesome Presence (alternate printing)
const AWESOME_PRESENCE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AWESOME_PRESENCE,
    1,
    "80017e84-87bc-4eb8-a663-5c263d8df812",
    "Lawrence Snelly",
);

// ALL 23b — Awesome Presence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AWESOME_PRESENCE: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Awesome Presence",
    "0aa8a120-5c13-4852-bdc8-80ae50a6e3d3",
    "Lawrence Snelly",
    crate::card::CardRules::unsupported(),
);

// ALL 24a — Benthic Explorers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BENTHIC_EXPLORERS: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Benthic Explorers",
    "146eb650-92c8-48a9-a40d-e7bba6545f36",
    "Greg Simanson",
    crate::card::CardRules::unsupported(),
);

// ALL 24b — Benthic Explorers (alternate printing)
const BENTHIC_EXPLORERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BENTHIC_EXPLORERS,
    1,
    "397d7602-d374-484c-aab2-3e57f12ceaa4",
    "Greg Simanson",
);

// ALL 25 — Browse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BROWSE: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Browse",
    "578549f0-5643-4891-b467-2d1cb49fe4ea",
    "Phil Foglio",
    crate::card::CardRules::unsupported(),
);

// ALL 26 — Diminishing Returns
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIMINISHING_RETURNS: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Diminishing Returns",
    "a375ec24-4841-4792-ad58-f29cdf0d1bbb",
    "Allen Williams",
    crate::card::CardRules::unsupported(),
);

// ALL 27a — False Demise
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FALSE_DEMISE: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "False Demise",
    "69773e2b-bfee-449d-b8e8-5646442f5487",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// ALL 27b — False Demise (alternate printing)
const FALSE_DEMISE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FALSE_DEMISE,
    1,
    "9e844464-cf1b-4a34-ac37-3a4da858c6bf",
    "Randy Gallegos",
);

// ALL 28 — Force of Will
pub(in crate::card::sets) static FORCE_OF_WILL: CardRecord = CardRecord::new(
    CardSet::Alliances,
    "Force of Will",
    "9a879b60-4381-447d-8a5a-8e0b6a1d49ca",
    "Terese Nielsen",
    // Answering a spell for no mana is what makes an entire format possible:
    // a deck can tap out and still not be dead to the one card that would
    // have beaten it.
    CardRules::new_instant(mana_cost!("{3}{U}{U}")).with_abilities(&[
        AbilityDef::alternative_cast(
            mana_cost!("{0}"),
            AlternativeCastKindDef::AlternativeCost,
            Some(
                "You may pay 1 life and exile a blue card from your hand rather than pay this \
                 spell's mana cost.",
            ),
            EffectDef::None,
        )
        .with_alternative_additional_cost(&SpellAdditionalCostDef::exile(
            ObjectPredicateDef::Color(ManaColor::Blue),
            ZoneKind::Hand,
            CostQuantityDef::Fixed(1),
        ))
        .with_alternative_life(1),
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
    ]),
);

// ALL 29a — Foresight
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORESIGHT: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Foresight",
    "a12e624c-8879-4e60-a1be-286abc5e0106",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// ALL 29b — Foresight (alternate printing)
const FORESIGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FORESIGHT,
    1,
    "a4fe74e0-bcde-4176-8da5-38e78daad5e5",
    "Terese Nielsen",
);

// ALL 30a — Lat-Nam's Legacy (alternate printing)
const LAT_NAM_S_LEGACY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LAT_NAM_S_LEGACY,
    1,
    "e3b3420b-424e-4c30-b329-bc4447f121d3",
    "Tom Wänerstrand",
);

// ALL 30b — Lat-Nam's Legacy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAT_NAM_S_LEGACY: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Lat-Nam's Legacy",
    "cd3b0741-dd5e-4d98-a50b-19a0f20dd72c",
    "Tom Wänerstrand",
    crate::card::CardRules::unsupported(),
);

// ALL 31 — Library of Lat-Nam
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIBRARY_OF_LAT_NAM: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Library of Lat-Nam",
    "5f5fa739-e8d4-4e1d-8b6b-c334d1e91bef",
    "Alan Rabinowitz",
    crate::card::CardRules::unsupported(),
);

// ALL 32 — Phantasmal Sphere
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHANTASMAL_SPHERE: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Phantasmal Sphere",
    "a84617c7-c70a-497c-b834-3d98346180cf",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// ALL 33a — Soldevi Heretic (alternate printing)
const SOLDEVI_HERETIC_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SOLDEVI_HERETIC,
    1,
    "d46accc8-b926-4443-bc12-dfd5870b2d2e",
    "Mike Kimble",
);

// ALL 33b — Soldevi Heretic
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOLDEVI_HERETIC: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Soldevi Heretic",
    "9613ca47-c9d1-4485-b0bd-71b0b587567e",
    "Mike Kimble",
    crate::card::CardRules::unsupported(),
);

// ALL 34a — Soldevi Sage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOLDEVI_SAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Soldevi Sage",
    "07392841-2df5-47f1-9868-edae3376e35a",
    "Carol Heyer",
    crate::card::CardRules::unsupported(),
);

// ALL 34b — Soldevi Sage (alternate printing)
const SOLDEVI_SAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SOLDEVI_SAGE,
    1,
    "53dc5902-817c-4a85-a0b9-20555574fad1",
    "Carol Heyer",
);

// ALL 35 — Spiny Starfish
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPINY_STARFISH: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Spiny Starfish",
    "c4242dda-6078-481d-a068-e7b10c873b89",
    "Alan Rabinowitz",
    crate::card::CardRules::unsupported(),
);

// ALL 36a — Storm Crow (alternate printing)
const STORM_CROW_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STORM_CROW,
    1,
    "a2d4ea78-16f1-46ac-8a60-db20c37aad5e",
    "Sandra Everingham",
);

// ALL 36b — Storm Crow
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STORM_CROW: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Storm Crow",
    "2dbf72f7-2360-4105-beae-946556884e40",
    "Sandra Everingham",
    crate::card::CardRules::unsupported(),
);

// ALL 37 — Storm Elemental
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STORM_ELEMENTAL: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Storm Elemental",
    "24de2b5e-78b1-490d-ac47-67f7076bc6b6",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// ALL 38 — Suffocation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUFFOCATION: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Suffocation",
    "d22104df-8147-45fd-897a-f99a815be062",
    "Allen Williams",
    crate::card::CardRules::unsupported(),
);

// ALL 39 — Thought Lash
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THOUGHT_LASH: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Thought Lash",
    "d59bbac1-ca51-4c72-9f1f-5fc6c82a4a27",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// ALL 40 — Tidal Control
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TIDAL_CONTROL: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Tidal Control",
    "cb9a7b7d-3d37-4bb6-ab48-1fec2bfb4fdc",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// ALL 41a — Viscerid Armor (alternate printing)
const VISCERID_ARMOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VISCERID_ARMOR,
    1,
    "d9c3f55a-5aa7-42e1-9aab-168a7e61c112",
    "Heather Hudson",
);

// ALL 41b — Viscerid Armor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VISCERID_ARMOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Viscerid Armor",
    "b719f89d-2a2c-460c-95e4-ada21353b340",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// ALL 42 — Viscerid Drone
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VISCERID_DRONE: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Viscerid Drone",
    "2ccd245f-e374-4bb8-8ac9-743b27ecf817",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// ALL 43 — Balduvian Dead
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BALDUVIAN_DEAD: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Balduvian Dead",
    "fac1875a-feab-4213-aa15-69892b7df58b",
    "Mike Kimble",
    crate::card::CardRules::unsupported(),
);

// ALL 44a — Casting of Bones (alternate printing)
const CASTING_OF_BONES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CASTING_OF_BONES,
    1,
    "e823c295-b66e-41c3-bd77-1a13f95e69c3",
    "Anson Maddocks",
);

// ALL 44b — Casting of Bones
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CASTING_OF_BONES: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Casting of Bones",
    "88442ddf-c12b-4a25-804d-29fef5a90a0c",
    "Anson Maddocks",
    crate::card::CardRules::unsupported(),
);

// ALL 45 — Contagion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CONTAGION: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Contagion",
    "00c8f94a-7690-47f5-b664-61411a32ab74",
    "Mike Raabe",
    crate::card::CardRules::unsupported(),
);

// ALL 46 — Diseased Vermin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DISEASED_VERMIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Diseased Vermin",
    "39703080-524d-4aa1-8c58-d512c41ae5d4",
    "Scott Kirschner",
    crate::card::CardRules::unsupported(),
);

// ALL 47 — Dystopia
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DYSTOPIA: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Dystopia",
    "5f8bb451-706d-44ff-bbad-9ddc6f9f786a",
    "Ruth Thompson",
    crate::card::CardRules::unsupported(),
);

// ALL 48 — Fatal Lore
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FATAL_LORE: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Fatal Lore",
    "24ba0b83-9671-4ee7-996d-57a3616b9c66",
    "Lawrence Snelly",
    crate::card::CardRules::unsupported(),
);

// ALL 49a — Feast or Famine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FEAST_OR_FAMINE: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Feast or Famine",
    "7c185b4d-8da5-4b8a-85f0-5f0622c7bade",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// ALL 49b — Feast or Famine (alternate printing)
const FEAST_OR_FAMINE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FEAST_OR_FAMINE,
    1,
    "f4ac1586-c3d5-4add-bade-b527dcf4a391",
    "Pete Venters",
);

// ALL 50a — Fevered Strength
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FEVERED_STRENGTH: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Fevered Strength",
    "13e53d6c-67f5-4d74-8205-6325c75d1d07",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// ALL 50b — Fevered Strength (alternate printing)
const FEVERED_STRENGTH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FEVERED_STRENGTH,
    1,
    "ca621684-1e0d-44d3-8bc4-f77e354b9ab4",
    "Brian Snõddy",
);

// ALL 51a — Insidious Bookworms
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INSIDIOUS_BOOKWORMS: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Insidious Bookworms",
    "4bfb7c7e-5a0a-4d4d-be98-ffed0386592b",
    "Greg Simanson",
    crate::card::CardRules::unsupported(),
);

// ALL 51b — Insidious Bookworms (alternate printing)
const INSIDIOUS_BOOKWORMS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &INSIDIOUS_BOOKWORMS,
    1,
    "7a043cc2-1bd4-4b44-ba23-d2585ffc3841",
    "Greg Simanson",
);

// ALL 52 — Keeper of Tresserhorn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KEEPER_OF_TRESSERHORN: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Keeper of Tresserhorn",
    "aaf8b0ec-f81a-488c-850c-098a8a3119e5",
    "Zak Plucinski & D. Alexander Gregory",
    crate::card::CardRules::unsupported(),
);

// ALL 53 — Krovikan Horror
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KROVIKAN_HORROR: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Krovikan Horror",
    "e1f3cb1c-6bde-4b55-b5bc-5b64b56930f2",
    "Christopher Rush",
    crate::card::CardRules::unsupported(),
);

// ALL 54 — Krovikan Plague
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KROVIKAN_PLAGUE: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Krovikan Plague",
    "b258e192-20af-4a45-981f-05181f4cd997",
    "Liz Danforth",
    crate::card::CardRules::unsupported(),
);

// ALL 55a — Lim-Dûl's High Guard (alternate printing)
const LIM_DUL_S_HIGH_GUARD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LIM_DUL_S_HIGH_GUARD,
    1,
    "b5afe9b5-3be8-472a-95c3-2c34231bc042",
    "Anson Maddocks",
);

// ALL 55b — Lim-Dûl's High Guard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIM_DUL_S_HIGH_GUARD: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Lim-Dûl's High Guard",
    "5470fce6-30cf-43bd-a258-a9fde4be0be8",
    "Anson Maddocks",
    crate::card::CardRules::unsupported(),
);

// ALL 56 — Misinformation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISINFORMATION: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Misinformation",
    "2f8638df-7915-4867-882a-95439486bd7b",
    "Richard Kane Ferguson",
    crate::card::CardRules::unsupported(),
);

// ALL 57a — Phantasmal Fiend (alternate printing)
const PHANTASMAL_FIEND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PHANTASMAL_FIEND,
    1,
    "9a05d428-7c70-4813-8e6c-20278cc0b0bd",
    "Scott Kirschner",
);

// ALL 57b — Phantasmal Fiend
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHANTASMAL_FIEND: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Phantasmal Fiend",
    "2c2842a1-25b8-4c4b-b5f8-496929288ff3",
    "Scott Kirschner",
    crate::card::CardRules::unsupported(),
);

// ALL 58a — Phyrexian Boon (alternate printing)
const PHYREXIAN_BOON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PHYREXIAN_BOON,
    1,
    "b9734708-d8e3-4d72-86b9-dd91fcfab5b4",
    "Mark Tedin",
);

// ALL 58b — Phyrexian Boon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_BOON: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Phyrexian Boon",
    "6f82668b-50b3-4746-b7fd-82f8560ebd95",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// ALL 59 — Ritual of the Machine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RITUAL_OF_THE_MACHINE: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Ritual of the Machine",
    "537b4109-ae7c-451a-8576-97f817a70d75",
    "Anson Maddocks",
    crate::card::CardRules::unsupported(),
);

// ALL 60a — Soldevi Adnate (alternate printing)
const SOLDEVI_ADNATE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SOLDEVI_ADNATE,
    1,
    "8b2651b0-1ab2-4d7e-834f-7505797da474",
    "Christopher Rush",
);

// ALL 60b — Soldevi Adnate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOLDEVI_ADNATE: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Soldevi Adnate",
    "80812871-d9a7-40de-94a5-b854e55409db",
    "Christopher Rush",
    crate::card::CardRules::unsupported(),
);

// ALL 61a — Stench of Decay (alternate printing)
const STENCH_OF_DECAY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STENCH_OF_DECAY,
    1,
    "f9a45644-549a-4eaa-8367-b170027bd5a2",
    "Heather Hudson",
);

// ALL 61b — Stench of Decay
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STENCH_OF_DECAY: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Stench of Decay",
    "b4b93845-f17a-4892-a1ce-a4630dced218",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// ALL 62 — Stromgald Spy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STROMGALD_SPY: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Stromgald Spy",
    "0cf8cecc-449f-4cc6-ac4d-440722df0ab9",
    "Zak Plucinski",
    crate::card::CardRules::unsupported(),
);

// ALL 63a — Swamp Mosquito
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SWAMP_MOSQUITO: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Swamp Mosquito",
    "21961b79-637a-4aa5-89b5-4e6e9f60d4d1",
    "Nicola Leonard",
    crate::card::CardRules::unsupported(),
);

// ALL 63b — Swamp Mosquito (alternate printing)
const SWAMP_MOSQUITO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SWAMP_MOSQUITO,
    1,
    "ac2fbe77-8757-4333-a083-975d5c3c6433",
    "Nicola Leonard",
);

// ALL 64a — Agent of Stromgald
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AGENT_OF_STROMGALD: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Agent of Stromgald",
    "4a7506f8-cf09-46ca-ad80-3c398c487ae2",
    "Alan Rabinowitz",
    crate::card::CardRules::unsupported(),
);

// ALL 64b — Agent of Stromgald (alternate printing)
const AGENT_OF_STROMGALD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AGENT_OF_STROMGALD,
    1,
    "d9236d75-1724-4121-9fa9-57fa96b19361",
    "Alan Rabinowitz",
);

// ALL 65 — Balduvian Horde
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BALDUVIAN_HORDE: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Balduvian Horde",
    "8e167a6c-05f8-4d90-9f6b-eb0f1046d54a",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// ALL 66a — Balduvian War-Makers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BALDUVIAN_WAR_MAKERS: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Balduvian War-Makers",
    "12fd561e-6a26-4140-a033-1204f5dda5f3",
    "Mike Kimble",
    crate::card::CardRules::unsupported(),
);

// ALL 66b — Balduvian War-Makers (alternate printing)
const BALDUVIAN_WAR_MAKERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BALDUVIAN_WAR_MAKERS,
    1,
    "dada7aeb-0062-4fb2-8bcb-abdd75029fb2",
    "Mike Kimble",
);

// ALL 67a — Bestial Fury
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BESTIAL_FURY: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Bestial Fury",
    "626225b9-2cfd-4cf5-b11c-89e5a231b09e",
    "Mike Raabe",
    crate::card::CardRules::unsupported(),
);

// ALL 67b — Bestial Fury (alternate printing)
const BESTIAL_FURY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BESTIAL_FURY,
    1,
    "7271e0d7-0c55-4020-97de-b8a27ba51d4b",
    "Mike Raabe",
);

// ALL 68 — Burnout
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BURNOUT: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Burnout",
    "5a8f5a18-e490-4010-ac1c-c74a5f2dcbda",
    "Mike Raabe",
    crate::card::CardRules::unsupported(),
);

// ALL 69 — Chaos Harlequin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHAOS_HARLEQUIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Chaos Harlequin",
    "ec7d7c80-4e3c-454e-b2ed-6f0436df19c9",
    "Alan Rabinowitz",
    crate::card::CardRules::unsupported(),
);

// ALL 70 — Death Spark
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEATH_SPARK: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Death Spark",
    "ba841b44-475c-402c-ac11-763de0cf27d9",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// ALL 71a — Enslaved Scout (alternate printing)
const ENSLAVED_SCOUT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ENSLAVED_SCOUT,
    1,
    "ea21414e-65f7-4d65-b0dc-7fec7b9b416d",
    "Rebecca Guay",
);

// ALL 71b — Enslaved Scout
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENSLAVED_SCOUT: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Enslaved Scout",
    "aac0e04a-d223-426b-b856-2829dbdffda0",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// ALL 72a — Gorilla Shaman (alternate printing)
const GORILLA_SHAMAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GORILLA_SHAMAN,
    1,
    "bf8b213e-31ca-4eb5-bf0b-515a0ad4fd31",
    "Anthony S. Waters",
);

// ALL 72b — Gorilla Shaman
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GORILLA_SHAMAN: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Gorilla Shaman",
    "5a16231c-1f73-4dec-9d88-e3d62e93a70f",
    "Anthony S. Waters",
    crate::card::CardRules::unsupported(),
);

// ALL 73a — Gorilla War Cry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GORILLA_WAR_CRY: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Gorilla War Cry",
    "613762ea-6111-4f74-bea2-c13b76e0751c",
    "Bryon Wackwitz",
    crate::card::CardRules::unsupported(),
);

// ALL 73b — Gorilla War Cry (alternate printing)
const GORILLA_WAR_CRY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GORILLA_WAR_CRY,
    1,
    "e14752ef-bebf-4c31-b130-32167473482f",
    "Bryon Wackwitz",
);

// ALL 74a — Guerrilla Tactics (alternate printing)
const GUERRILLA_TACTICS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GUERRILLA_TACTICS,
    1,
    "51811f2a-7002-4ba7-98d8-5b09d887975c",
    "Randy Asplund-Faith",
);

// ALL 74b — Guerrilla Tactics
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GUERRILLA_TACTICS: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Guerrilla Tactics",
    "3c005ca3-0508-4ac2-afec-3d4a27334c31",
    "Randy Asplund-Faith",
    crate::card::CardRules::unsupported(),
);

// ALL 75 — Omen of Fire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OMEN_OF_FIRE: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Omen of Fire",
    "9c724b46-6e17-4bee-9bc6-e9fc5a379dd7",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// ALL 76 — Pillage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PILLAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Pillage",
    "389ecb50-b007-4086-89fb-ec2daa5afdcf",
    "Richard Kane Ferguson",
    crate::card::CardRules::unsupported(),
);

// ALL 77 — Primitive Justice
pub(in crate::card::sets) static PRIMITIVE_JUSTICE: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Primitive Justice",
    "d6b7829b-2a10-47e7-9cf9-8ae49d2b398a",
    "Anthony S. Waters",
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_abilities(&[
        abilities::repeatable_additional_mana_cost(
            "As an additional cost to cast this spell, you may pay {1}{R} any number of times.",
            "{1}{R} additional cost",
            mana_cost!("{1}{R}"),
        ),
        abilities::repeatable_additional_mana_cost(
            "As an additional cost to cast this spell, you may pay {1}{G} any number of times.",
            "{1}{G} additional cost",
            mana_cost!("{1}{G}"),
        ),
        AbilityDef::spell_with_targets(
            "Destroy target artifact. For each additional {1}{R} you paid, destroy another target artifact. For each additional {1}{G} you paid, destroy another target artifact, and you gain 1 life.",
            &[AbilityTargetDef::exactly_value(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Artifact),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                ValueDef::Sum(&SumValueDef::new(
                    ValueDef::Constant(1),
                    ValueDef::Sum(&SumValueDef::new(
                        ValueDef::AdditionalCostPayments(AdditionalCostIndex::PRIMARY),
                        ValueDef::AdditionalCostPayments(AdditionalCostIndex::SECONDARY),
                    )),
                )),
            )],
            EffectDef::Sequence(&[
                EffectDef::Destroy {
                    object: EffectRecipientDef::objects(ObjectSetDef::LegalTargets(
                        TargetIndex::PRIMARY,
                    )),
                    can_regenerate: true,
                    then: None,
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::AdditionalCostPayments(AdditionalCostIndex::SECONDARY),
                },
            ]),
        ),
    ]),
);

// ALL 78 — Pyrokinesis
pub(in crate::card::sets) static PYROKINESIS: CardRecord = CardRecord::new(
    CardSet::Alliances,
    "Pyrokinesis",
    "db2a5e85-6cbc-43c1-9362-4056ad017ef0",
    "Ron Spencer",
    // The free cast is what the card is played for -- a blowout from an empty
    // board -- so the printed cost alone understates it considerably.
    CardRules::new_instant(mana_cost!("{4}{R}{R}")).with_abilities(&[
        AbilityDef::alternative_cast(
            mana_cost!("{0}"),
            AlternativeCastKindDef::AlternativeCost,
            Some("You may exile a red card from your hand rather than pay this spell's mana cost."),
            EffectDef::None,
        )
        // Exiled from hand rather than discarded: the card is spent without ever
        // becoming a graveyard card, which is what "exile a red card" means.
        .with_alternative_additional_cost(&SpellAdditionalCostDef::exile(
            ObjectPredicateDef::Color(ManaColor::Red),
            ZoneKind::Hand,
            CostQuantityDef::Fixed(1),
        )),
        AbilityDef::spell_with_targets(
            "Pyrokinesis deals 4 damage divided as you choose among any number of target creatures.",
            // Four damage split however the caster likes. There is no printed ceiling on
            // the number of creatures, but the division supplies one anyway: every target
            // must be assigned at least one damage, so four is the most it can ever
            // reach.
            &[AbilityTargetDef {
                predicate: AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[crate::card::ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                minimum: 1,
                maximum: AbilityTargetDef::UNLIMITED,
                exact_count: None,
                divided_total: Some(DividedTotal::Fixed(4)),
                another: false,
                excludes_source: false,
                chooser: TargetChooserDef::Controller,
            }],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::DividedAmongTargets,
            },
        ),
    ]),
);

// ALL 79 — Rogue Skycaptain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROGUE_SKYCAPTAIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Rogue Skycaptain",
    "97aebf3b-e77d-4d18-b58b-117ae91792e2",
    "Randy Asplund-Faith",
    crate::card::CardRules::unsupported(),
);

// ALL 80 — Soldier of Fortune
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOLDIER_OF_FORTUNE: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Soldier of Fortune",
    "37c05f46-2081-4ebb-a758-894ac040ea2a",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// ALL 81a — Storm Shaman (alternate printing)
const STORM_SHAMAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STORM_SHAMAN,
    1,
    "92c30a0a-3083-4d9f-9fe0-de5d6294f80e",
    "Carol Heyer",
);

// ALL 81b — Storm Shaman
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STORM_SHAMAN: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Storm Shaman",
    "3a8f1150-6306-42a6-84e1-7dd5bfef6d14",
    "Carol Heyer",
    crate::card::CardRules::unsupported(),
);

// ALL 82a — Varchild's Crusader
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VARCHILD_S_CRUSADER: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Varchild's Crusader",
    "b5ade7ad-ce32-4296-8cec-20bd79c7b16a",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// ALL 82b — Varchild's Crusader (alternate printing)
const VARCHILD_S_CRUSADER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VARCHILD_S_CRUSADER,
    1,
    "d730611f-ad45-40b7-80c8-5decf2627e79",
    "Mark Poole",
);

// ALL 83 — Varchild's War-Riders
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VARCHILD_S_WAR_RIDERS: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Varchild's War-Riders",
    "ee1d41da-aa72-434b-811f-95d4bae4ba5c",
    "Susan Van Camp",
    crate::card::CardRules::unsupported(),
);

// ALL 84a — Veteran's Voice (alternate printing)
const VETERAN_S_VOICE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VETERAN_S_VOICE,
    1,
    "93babf85-eb13-4e06-b45b-8927791bcde5",
    "Andi Rusu",
);

// ALL 84b — Veteran's Voice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VETERAN_S_VOICE: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Veteran's Voice",
    "6e1ecb9a-7443-49cb-8197-ef180124aabb",
    "Andi Rusu",
    crate::card::CardRules::unsupported(),
);

// ALL 85 — Bounty of the Hunt
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOUNTY_OF_THE_HUNT: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Bounty of the Hunt",
    "21ed522a-cf5a-41e1-9677-1226f689ec9c",
    "Jeff A. Menges",
    crate::card::CardRules::unsupported(),
);

// ALL 86a — Deadly Insect (alternate printing)
const DEADLY_INSECT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DEADLY_INSECT,
    1,
    "add1b999-5c3f-4187-adac-ed1037406b3f",
    "Scott Kirschner",
);

// ALL 86b — Deadly Insect
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEADLY_INSECT: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Deadly Insect",
    "030963d9-b59f-4ccb-abed-d817a4bc4e05",
    "Scott Kirschner",
    crate::card::CardRules::unsupported(),
);

// ALL 87 — Elvish Bard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELVISH_BARD: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Elvish Bard",
    "62261004-ed32-4865-824a-4320548f4234",
    "Susan Van Camp",
    crate::card::CardRules::unsupported(),
);

// ALL 88a — Elvish Ranger (alternate printing)
const ELVISH_RANGER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ELVISH_RANGER,
    1,
    "ad9f1b09-73c2-43c5-a28b-4a40fff7b727",
    "Terese Nielsen",
);

// ALL 88b — Elvish Ranger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELVISH_RANGER: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Elvish Ranger",
    "7b08a164-d6f4-423e-8666-e4a4c2d21045",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// ALL 89 — Elvish Spirit Guide
pub(in crate::card::sets) static ELVISH_SPIRIT_GUIDE: CardRecord = CardRecord::new(
    CardSet::Alliances,
    "Elvish Spirit Guide",
    "5b94f37f-ebdf-4b79-a615-58331d27cf4e",
    "Julie Baroh",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Elf", "Spirit"], 2, 2).with_ability(
        AbilityDef::activated_mana(
            "Exile this card from your hand: Add {G}.",
            &[AbilityCostDef::ExileSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
        )
        .with_source_zones(&[ZoneKind::Hand]),
    ),
);

// ALL 90a — Fyndhorn Druid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FYNDHORN_DRUID: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Fyndhorn Druid",
    "778b028f-fa4e-4638-82b4-fb287223ea20",
    "Rob Alexander",
    crate::card::CardRules::unsupported(),
);

// ALL 90b — Fyndhorn Druid (alternate printing)
const FYNDHORN_DRUID_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FYNDHORN_DRUID,
    1,
    "f79d880c-0052-4e2c-92aa-bf3cbd107cbd",
    "Rob Alexander",
);

// ALL 91 — Gargantuan Gorilla
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GARGANTUAN_GORILLA: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Gargantuan Gorilla",
    "49f367c2-f47e-43e1-9936-4324be664475",
    "Greg Simanson",
    crate::card::CardRules::unsupported(),
);

// ALL 92a — Gift of the Woods (alternate printing)
const GIFT_OF_THE_WOODS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GIFT_OF_THE_WOODS,
    1,
    "da48976b-667d-4a1e-92de-9c3cb25dfd21",
    "Susan Van Camp",
);

// ALL 92b — Gift of the Woods
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GIFT_OF_THE_WOODS: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Gift of the Woods",
    "6a0df4e9-b201-4fc7-8e37-59d99b583f76",
    "Susan Van Camp",
    crate::card::CardRules::unsupported(),
);

// ALL 93a — Gorilla Berserkers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GORILLA_BERSERKERS: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Gorilla Berserkers",
    "344b4613-17f8-4c8b-b5bc-f773a8f8007a",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// ALL 93b — Gorilla Berserkers (alternate printing)
const GORILLA_BERSERKERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GORILLA_BERSERKERS,
    1,
    "e3c32b65-58e7-455b-9a30-7a2edcc27b9d",
    "John Matson",
);

// ALL 94a — Gorilla Chieftain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GORILLA_CHIEFTAIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Gorilla Chieftain",
    "47f1eedd-7021-4cce-a808-2e9384a5ef15",
    "Quinton Hoover",
    crate::card::CardRules::unsupported(),
);

// ALL 94b — Gorilla Chieftain (alternate printing)
const GORILLA_CHIEFTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GORILLA_CHIEFTAIN,
    1,
    "6bdde5d2-3dd2-4eaa-9c52-4ad400b56ed1",
    "Quinton Hoover",
);

// ALL 95 — Hail Storm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HAIL_STORM: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Hail Storm",
    "a7e9d786-4e9b-447b-a5dc-ca117c4961c5",
    "Jeff A. Menges",
    crate::card::CardRules::unsupported(),
);

// ALL 96 — Kaysa
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KAYSA: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Kaysa",
    "cd4b6daf-cf37-43c6-9446-3aa0de222ac4",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// ALL 97 — Nature's Chosen
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NATURE_S_CHOSEN: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Nature's Chosen",
    "7bd0b831-9d7e-40ce-8514-e852daee1a9e",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// ALL 98 — Nature's Wrath
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NATURE_S_WRATH: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Nature's Wrath",
    "450759f0-5d60-4f05-9011-b0b66dbb06a7",
    "Liz Danforth",
    crate::card::CardRules::unsupported(),
);

// ALL 99 — Splintering Wind
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPLINTERING_WIND: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Splintering Wind",
    "0afa94e5-fef6-4f3a-9196-d7aa6dd841c2",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// ALL 100a — Taste of Paradise (alternate printing)
const TASTE_OF_PARADISE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TASTE_OF_PARADISE,
    1,
    "b9248694-88fa-4fe1-9902-f03a41100cd6",
    "Lawrence Snelly",
);

// ALL 100b — Taste of Paradise
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TASTE_OF_PARADISE: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Taste of Paradise",
    "a774c426-ec0e-48de-b00f-5a05cc6dc34b",
    "Lawrence Snelly",
    crate::card::CardRules::unsupported(),
);

// ALL 101 — Tornado
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TORNADO: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Tornado",
    "a2fd58e4-eb9a-4a12-8914-0a9a8300626c",
    "Susan Van Camp",
    crate::card::CardRules::unsupported(),
);

// ALL 102a — Undergrowth (alternate printing)
const UNDERGROWTH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &UNDERGROWTH,
    1,
    "9ade0829-8b90-4ed2-99f3-dd748e7706b8",
    "Pat Lewis",
);

// ALL 102b — Undergrowth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNDERGROWTH: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Undergrowth",
    "7b07df91-49be-4a50-9d3b-ddde0e6c1be9",
    "Pat Lewis",
    crate::card::CardRules::unsupported(),
);

// ALL 103a — Whip Vine (alternate printing)
const WHIP_VINE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WHIP_VINE,
    1,
    "4b66b9fe-47f1-4786-96d5-981d62012663",
    "Allen Williams",
);

// ALL 103b — Whip Vine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WHIP_VINE: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Whip Vine",
    "31ee1c89-d7df-4ee7-b403-24dfabae38a0",
    "Allen Williams",
    crate::card::CardRules::unsupported(),
);

// ALL 104a — Yavimaya Ancients (alternate printing)
const YAVIMAYA_ANCIENTS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &YAVIMAYA_ANCIENTS,
    1,
    "94fc4db5-08e5-4cf8-bf47-f7c6a58162b2",
    "Quinton Hoover",
);

// ALL 104b — Yavimaya Ancients
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static YAVIMAYA_ANCIENTS: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Yavimaya Ancients",
    "91708e45-f9a1-4c2e-973d-bfc294926c93",
    "Quinton Hoover",
    crate::card::CardRules::unsupported(),
);

// ALL 105 — Yavimaya Ants
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static YAVIMAYA_ANTS: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Yavimaya Ants",
    "5ded1c83-a289-4951-b72a-477a041610d3",
    "Pat Lewis",
    crate::card::CardRules::unsupported(),
);

// ALL 106 — Energy Arc
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENERGY_ARC: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Energy Arc",
    "f81cd99e-902a-44dd-8928-803a96fe25c4",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// ALL 107 — Lim-Dûl's Vault
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIM_DUL_S_VAULT: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Lim-Dûl's Vault",
    "f9b0164c-2d4e-48ab-addd-322d9b504739",
    "Rob Alexander",
    crate::card::CardRules::unsupported(),
);

// ALL 108 — Lim-Dûl's Paladin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIM_DUL_S_PALADIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Lim-Dûl's Paladin",
    "44be2d66-359e-4cc1-9670-119cb9c7d5f5",
    "Christopher Rush",
    crate::card::CardRules::unsupported(),
);

// ALL 109 — Surge of Strength
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SURGE_OF_STRENGTH: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Surge of Strength",
    "96fff700-af02-4861-b7ed-be9950e69bf1",
    "Ruth Thompson",
    crate::card::CardRules::unsupported(),
);

// ALL 110 — Nature's Blessing
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NATURE_S_BLESSING: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Nature's Blessing",
    "5ba0e677-361d-4e03-9c2c-018d1c383456",
    "Sandra Everingham",
    crate::card::CardRules::unsupported(),
);

// ALL 111 — Wandering Mage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WANDERING_MAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Wandering Mage",
    "8d9b1b6c-1f02-4918-bb5c-2dbcdb0997ec",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// ALL 112 — Lord of Tresserhorn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LORD_OF_TRESSERHORN: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Lord of Tresserhorn",
    "5fc9497a-42bf-4d78-afaf-67645514ade4",
    "Anson Maddocks",
    crate::card::CardRules::unsupported(),
);

// ALL 113 — Misfortune
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISFORTUNE: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Misfortune",
    "b14cc32a-eb4f-4690-aceb-160780743ebe",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// ALL 114 — Winter's Night
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WINTER_S_NIGHT: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Winter's Night",
    "7f020ebc-4950-4407-8cb8-7630cad226f6",
    "Rob Alexander",
    crate::card::CardRules::unsupported(),
);

// ALL 115 — Phelddagrif
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHELDDAGRIF: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Phelddagrif",
    "d9631cb2-d53b-4401-b53b-29d27bdefc44",
    "Amy Weber",
    crate::card::CardRules::unsupported(),
);

// ALL 116a — Aesthir Glider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AESTHIR_GLIDER: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Aesthir Glider",
    "35a8080f-ca3c-46fe-81cf-003ac7ba7f24",
    "Ruth Thompson",
    crate::card::CardRules::unsupported(),
);

// ALL 116b — Aesthir Glider (alternate printing)
const AESTHIR_GLIDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AESTHIR_GLIDER,
    1,
    "435b78cb-5acc-4d14-966f-979322d99114",
    "Ruth Thompson",
);

// ALL 117 — Ashnod's Cylix
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ASHNOD_S_CYLIX: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Ashnod's Cylix",
    "d84e6fcf-4745-4dfb-9103-17beec4e45b6",
    "Nicola Leonard",
    crate::card::CardRules::unsupported(),
);

// ALL 118a — Astrolabe (alternate printing)
const ASTROLABE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ASTROLABE,
    1,
    "b97ad2d4-0660-4503-9f16-246dae87601c",
    "Amy Weber",
);

// ALL 118b — Astrolabe
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ASTROLABE: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Astrolabe",
    "8e3a4e30-f919-4c96-89f2-467355135f8f",
    "Amy Weber",
    crate::card::CardRules::unsupported(),
);

// ALL 119 — Floodwater Dam
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLOODWATER_DAM: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Floodwater Dam",
    "d272c3cb-0b68-4693-abef-8a5375b2463e",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// ALL 120 — Gustha's Scepter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GUSTHA_S_SCEPTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Gustha's Scepter",
    "797c84fa-3704-4fec-bd72-468d6415ae70",
    "Sandra Everingham",
    crate::card::CardRules::unsupported(),
);

// ALL 121 — Helm of Obedience
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HELM_OF_OBEDIENCE: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Helm of Obedience",
    "b17e9216-b1ed-4101-a04e-2bb139ccfa55",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// ALL 122 — Lodestone Bauble
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LODESTONE_BAUBLE: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Lodestone Bauble",
    "84d88a33-3990-4044-a5fe-4123d5781f18",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// ALL 123 — Mishra's Groundbreaker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISHRA_S_GROUNDBREAKER: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Mishra's Groundbreaker",
    "74e2dc26-30aa-4e20-84b0-ea4be8894475",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// ALL 124 — Mystic Compass
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MYSTIC_COMPASS: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Mystic Compass",
    "de53ba3a-f2f7-4ea6-a2f6-dd5b87029e58",
    "Amy Weber",
    crate::card::CardRules::unsupported(),
);

// ALL 125 — Phyrexian Devourer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_DEVOURER: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Phyrexian Devourer",
    "319430fa-11e4-426e-8297-67df8474c3cc",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// ALL 126 — Phyrexian Portal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_PORTAL: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Phyrexian Portal",
    "74f77387-1239-4ad2-b59f-d13e317477ba",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// ALL 127a — Phyrexian War Beast
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_WAR_BEAST: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Phyrexian War Beast",
    "e7a83384-8762-4028-8cab-b690593790a6",
    "Bill Sienkiewicz",
    crate::card::CardRules::unsupported(),
);

// ALL 127b — Phyrexian War Beast (alternate printing)
const PHYREXIAN_WAR_BEAST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PHYREXIAN_WAR_BEAST,
    1,
    "e7d651f6-50be-4df9-80f8-4c62bb860e71",
    "Bill Sienkiewicz",
);

// ALL 128 — Scarab of the Unseen
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCARAB_OF_THE_UNSEEN: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Scarab of the Unseen",
    "d5da1c71-6059-4e4e-933d-dbca1cc4bd15",
    "Sandra Everingham",
    crate::card::CardRules::unsupported(),
);

// ALL 129 — Shield Sphere
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHIELD_SPHERE: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Shield Sphere",
    "1730d219-a28f-4930-8088-4cfcb627f157",
    "Alan Rabinowitz",
    crate::card::CardRules::unsupported(),
);

// ALL 130 — Sol Grail
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOL_GRAIL: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Sol Grail",
    "62652722-e345-4670-9547-d9579efa227d",
    "Christopher Rush",
    crate::card::CardRules::unsupported(),
);

// ALL 131 — Soldevi Digger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOLDEVI_DIGGER: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Soldevi Digger",
    "5a3a0ab4-e8ef-45fd-9a73-86d1ee30cb48",
    "Amy Weber",
    crate::card::CardRules::unsupported(),
);

// ALL 132a — Soldevi Sentry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOLDEVI_SENTRY: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Soldevi Sentry",
    "85976b5c-4eed-4cf9-b2b0-a8421a97ab2a",
    "Alan Rabinowitz",
    crate::card::CardRules::unsupported(),
);

// ALL 132b — Soldevi Sentry (alternate printing)
const SOLDEVI_SENTRY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SOLDEVI_SENTRY,
    1,
    "be2a84d7-3f49-4652-bb31-4be7e3474e26",
    "Alan Rabinowitz",
);

// ALL 133a — Soldevi Steam Beast (alternate printing)
const SOLDEVI_STEAM_BEAST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SOLDEVI_STEAM_BEAST,
    1,
    "ead79d2c-170e-4106-962d-d69c4b5fead0",
    "Bill Sienkiewicz",
);

// ALL 133b — Soldevi Steam Beast
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOLDEVI_STEAM_BEAST: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Soldevi Steam Beast",
    "9de5e730-1d5c-4326-b3fc-2f0f97edc07e",
    "Bill Sienkiewicz",
    crate::card::CardRules::unsupported(),
);

// ALL 134 — Storm Cauldron
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STORM_CAULDRON: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Storm Cauldron",
    "1f68b531-a3f2-4830-b170-fb8a1195c149",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// ALL 135 — Urza's Engine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static URZA_S_ENGINE: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Urza's Engine",
    "273b54c3-325b-4f2e-857b-fc1d59b6b3c5",
    "Greg Simanson",
    crate::card::CardRules::unsupported(),
);

// ALL 136 — Whirling Catapult
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WHIRLING_CATAPULT: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Whirling Catapult",
    "6206d65a-6907-4d11-acb0-8820277f2cf2",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// ALL 137 — Balduvian Trading Post
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BALDUVIAN_TRADING_POST: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Balduvian Trading Post",
    "a329ff98-36fd-44c3-b037-dcc6e78ee61e",
    "Tom Wänerstrand",
    crate::card::CardRules::unsupported(),
);

// ALL 138 — Heart of Yavimaya
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEART_OF_YAVIMAYA: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Heart of Yavimaya",
    "40c59cb9-559b-4716-9bd7-c818b3f46f1d",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// ALL 139 — Kjeldoran Outpost
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KJELDORAN_OUTPOST: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Kjeldoran Outpost",
    "e0769fc7-50b5-4b49-8aff-af04536288fb",
    "Jeff A. Menges",
    crate::card::CardRules::unsupported(),
);

// ALL 140 — Lake of the Dead
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAKE_OF_THE_DEAD: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Lake of the Dead",
    "aee806ce-effa-4244-9659-43246e944d80",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// ALL 141 — School of the Unseen
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCHOOL_OF_THE_UNSEEN: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "School of the Unseen",
    "1438606d-556d-4b96-9662-fcac051af045",
    "Pat Lewis",
    crate::card::CardRules::unsupported(),
);

// ALL 142 — Sheltered Valley
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHELTERED_VALLEY: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Sheltered Valley",
    "049d7a08-1605-4ce2-b8c5-634ce2a261e0",
    "Rob Alexander",
    crate::card::CardRules::unsupported(),
);

// ALL 143 — Soldevi Excavations
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOLDEVI_EXCAVATIONS: CardRecord = CardRecord::new(
    crate::card::CardSet::Alliances,
    "Soldevi Excavations",
    "8dbda146-ed0a-4bf6-b99d-dc6d59bd9447",
    "Liz Danforth",
    crate::card::CardRules::unsupported(),
);

// ALL 144 — Thawing Glaciers
pub(in crate::card::sets) static THAWING_GLACIERS: CardRecord = CardRecord::new(
    CardSet::Alliances,
    "Thawing Glaciers",
    "6411a8c6-010f-4863-a0fa-bbebe09d5c34",
    "Jeff A. Menges",
    // One basic a turn, forever: slow enough that only a deck with nothing
    // better to do at end of turn wants it, which is exactly Landstill.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        AbilityDef::activated(
            "{1}, {T}: Search your library for a basic land card, put that card onto the battlefield tapped, then shuffle. Return this land to its owner's hand at the beginning of the next cleanup step.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::TapSource,
            ],
            EffectDef::Sequence(&const {
                [
                    EffectDef::SearchZone {
                        player: EffectRecipientDef::Controller,
                        source: ZoneKind::Library,
                        object: ObjectPredicateDef::All(&const {
                            [
                                ObjectPredicateDef::HasType(CardType::Land),
                                ObjectPredicateDef::Supertype(CardSupertype::Basic),
                            ]
                        }),
                        minimum: 0,
                        maximum: ValueDef::Constant(1),
                        reveal: false,
                        destination: ZoneKind::Battlefield,
                        placement: ZonePlacement::Top,
                        shuffle: true,
                        enters_tapped: true,
                        attachment: None,
                        binding: None,
                        then: None,
                    },
                    // The land fetches, and then leaves: the return is a delayed trigger so
                    // that the land is available to tap again next turn rather than staying to
                    // be tapped twice in one.
                    EffectDef::InstallTrigger(InstalledTriggerDef::once(&const {
                        AbilityDef::triggered(
                        "At the beginning of the next cleanup step, return this land to its owner's hand.",
                        TriggerEventDef::StepBegins {
                            step: TurnStepDef::Cleanup,
                            player: PlayerRelation::Any,
                        },
                        EffectDef::MoveToZone {
                            object: EffectRecipientDef::Source,
                            zone: ZoneKind::Hand,
                            placement: ZonePlacement::Top,
                        },
                    )
                    })),
                ]
            }),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &CARRIER_PIGEONS,
    &ERRAND_OF_DUTY,
    &EXILE,
    &INHERITANCE,
    &IVORY_GARGOYLE,
    &JUNIPER_ORDER_ADVOCATE,
    &KJELDORAN_ESCORT,
    &KJELDORAN_HOME_GUARD,
    &KJELDORAN_PRIDE,
    &MARTYRDOM,
    &NOBLE_STEEDS,
    &REINFORCEMENTS,
    &REPRISAL,
    &ROYAL_DECREE,
    &ROYAL_HERBALIST,
    &SCARS_OF_THE_VETERAN,
    &SEASONED_TACTICIAN,
    &SUSTAINING_SPIRIT,
    &SWORN_DEFENDER,
    &UNLIKELY_ALLIANCE,
    &WILD_AESTHIR,
    &ARCANE_DENIAL,
    &AWESOME_PRESENCE,
    &BENTHIC_EXPLORERS,
    &BROWSE,
    &DIMINISHING_RETURNS,
    &FALSE_DEMISE,
    &FORCE_OF_WILL,
    &FORESIGHT,
    &LAT_NAM_S_LEGACY,
    &LIBRARY_OF_LAT_NAM,
    &PHANTASMAL_SPHERE,
    &SOLDEVI_HERETIC,
    &SOLDEVI_SAGE,
    &SPINY_STARFISH,
    &STORM_CROW,
    &STORM_ELEMENTAL,
    &SUFFOCATION,
    &THOUGHT_LASH,
    &TIDAL_CONTROL,
    &VISCERID_ARMOR,
    &VISCERID_DRONE,
    &BALDUVIAN_DEAD,
    &CASTING_OF_BONES,
    &CONTAGION,
    &DISEASED_VERMIN,
    &DYSTOPIA,
    &FATAL_LORE,
    &FEAST_OR_FAMINE,
    &FEVERED_STRENGTH,
    &INSIDIOUS_BOOKWORMS,
    &KEEPER_OF_TRESSERHORN,
    &KROVIKAN_HORROR,
    &KROVIKAN_PLAGUE,
    &LIM_DUL_S_HIGH_GUARD,
    &MISINFORMATION,
    &PHANTASMAL_FIEND,
    &PHYREXIAN_BOON,
    &RITUAL_OF_THE_MACHINE,
    &SOLDEVI_ADNATE,
    &STENCH_OF_DECAY,
    &STROMGALD_SPY,
    &SWAMP_MOSQUITO,
    &AGENT_OF_STROMGALD,
    &BALDUVIAN_HORDE,
    &BALDUVIAN_WAR_MAKERS,
    &BESTIAL_FURY,
    &BURNOUT,
    &CHAOS_HARLEQUIN,
    &DEATH_SPARK,
    &ENSLAVED_SCOUT,
    &GORILLA_SHAMAN,
    &GORILLA_WAR_CRY,
    &GUERRILLA_TACTICS,
    &OMEN_OF_FIRE,
    &PILLAGE,
    &PRIMITIVE_JUSTICE,
    &PYROKINESIS,
    &ROGUE_SKYCAPTAIN,
    &SOLDIER_OF_FORTUNE,
    &STORM_SHAMAN,
    &VARCHILD_S_CRUSADER,
    &VARCHILD_S_WAR_RIDERS,
    &VETERAN_S_VOICE,
    &BOUNTY_OF_THE_HUNT,
    &DEADLY_INSECT,
    &ELVISH_BARD,
    &ELVISH_RANGER,
    &ELVISH_SPIRIT_GUIDE,
    &FYNDHORN_DRUID,
    &GARGANTUAN_GORILLA,
    &GIFT_OF_THE_WOODS,
    &GORILLA_BERSERKERS,
    &GORILLA_CHIEFTAIN,
    &HAIL_STORM,
    &KAYSA,
    &NATURE_S_CHOSEN,
    &NATURE_S_WRATH,
    &SPLINTERING_WIND,
    &TASTE_OF_PARADISE,
    &TORNADO,
    &UNDERGROWTH,
    &WHIP_VINE,
    &YAVIMAYA_ANCIENTS,
    &YAVIMAYA_ANTS,
    &ENERGY_ARC,
    &LIM_DUL_S_VAULT,
    &LIM_DUL_S_PALADIN,
    &SURGE_OF_STRENGTH,
    &NATURE_S_BLESSING,
    &WANDERING_MAGE,
    &LORD_OF_TRESSERHORN,
    &MISFORTUNE,
    &WINTER_S_NIGHT,
    &PHELDDAGRIF,
    &AESTHIR_GLIDER,
    &ASHNOD_S_CYLIX,
    &ASTROLABE,
    &FLOODWATER_DAM,
    &GUSTHA_S_SCEPTER,
    &HELM_OF_OBEDIENCE,
    &LODESTONE_BAUBLE,
    &MISHRA_S_GROUNDBREAKER,
    &MYSTIC_COMPASS,
    &PHYREXIAN_DEVOURER,
    &PHYREXIAN_PORTAL,
    &PHYREXIAN_WAR_BEAST,
    &SCARAB_OF_THE_UNSEEN,
    &SHIELD_SPHERE,
    &SOL_GRAIL,
    &SOLDEVI_DIGGER,
    &SOLDEVI_SENTRY,
    &SOLDEVI_STEAM_BEAST,
    &STORM_CAULDRON,
    &URZA_S_ENGINE,
    &WHIRLING_CATAPULT,
    &BALDUVIAN_TRADING_POST,
    &HEART_OF_YAVIMAYA,
    &KJELDORAN_OUTPOST,
    &LAKE_OF_THE_DEAD,
    &SCHOOL_OF_THE_UNSEEN,
    &SHELTERED_VALLEY,
    &SOLDEVI_EXCAVATIONS,
    &THAWING_GLACIERS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    CARRIER_PIGEONS_ALTERNATE_1,
    ERRAND_OF_DUTY_ALTERNATE_1,
    KJELDORAN_ESCORT_ALTERNATE_1,
    KJELDORAN_PRIDE_ALTERNATE_1,
    MARTYRDOM_ALTERNATE_1,
    NOBLE_STEEDS_ALTERNATE_1,
    REINFORCEMENTS_ALTERNATE_1,
    REPRISAL_ALTERNATE_1,
    ROYAL_HERBALIST_ALTERNATE_1,
    WILD_AESTHIR_ALTERNATE_1,
    ARCANE_DENIAL_ALTERNATE_1,
    AWESOME_PRESENCE_ALTERNATE_1,
    BENTHIC_EXPLORERS_ALTERNATE_1,
    FALSE_DEMISE_ALTERNATE_1,
    FORESIGHT_ALTERNATE_1,
    LAT_NAM_S_LEGACY_ALTERNATE_1,
    SOLDEVI_HERETIC_ALTERNATE_1,
    SOLDEVI_SAGE_ALTERNATE_1,
    STORM_CROW_ALTERNATE_1,
    VISCERID_ARMOR_ALTERNATE_1,
    CASTING_OF_BONES_ALTERNATE_1,
    FEAST_OR_FAMINE_ALTERNATE_1,
    FEVERED_STRENGTH_ALTERNATE_1,
    INSIDIOUS_BOOKWORMS_ALTERNATE_1,
    LIM_DUL_S_HIGH_GUARD_ALTERNATE_1,
    PHANTASMAL_FIEND_ALTERNATE_1,
    PHYREXIAN_BOON_ALTERNATE_1,
    SOLDEVI_ADNATE_ALTERNATE_1,
    STENCH_OF_DECAY_ALTERNATE_1,
    SWAMP_MOSQUITO_ALTERNATE_1,
    AGENT_OF_STROMGALD_ALTERNATE_1,
    BALDUVIAN_WAR_MAKERS_ALTERNATE_1,
    BESTIAL_FURY_ALTERNATE_1,
    ENSLAVED_SCOUT_ALTERNATE_1,
    GORILLA_SHAMAN_ALTERNATE_1,
    GORILLA_WAR_CRY_ALTERNATE_1,
    GUERRILLA_TACTICS_ALTERNATE_1,
    STORM_SHAMAN_ALTERNATE_1,
    VARCHILD_S_CRUSADER_ALTERNATE_1,
    VETERAN_S_VOICE_ALTERNATE_1,
    DEADLY_INSECT_ALTERNATE_1,
    ELVISH_RANGER_ALTERNATE_1,
    FYNDHORN_DRUID_ALTERNATE_1,
    GIFT_OF_THE_WOODS_ALTERNATE_1,
    GORILLA_BERSERKERS_ALTERNATE_1,
    GORILLA_CHIEFTAIN_ALTERNATE_1,
    TASTE_OF_PARADISE_ALTERNATE_1,
    UNDERGROWTH_ALTERNATE_1,
    WHIP_VINE_ALTERNATE_1,
    YAVIMAYA_ANCIENTS_ALTERNATE_1,
    AESTHIR_GLIDER_ALTERNATE_1,
    ASTROLABE_ALTERNATE_1,
    PHYREXIAN_WAR_BEAST_ALTERNATE_1,
    SOLDEVI_SENTRY_ALTERNATE_1,
    SOLDEVI_STEAM_BEAST_ALTERNATE_1,
];
