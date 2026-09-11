//! Bloomburrow card inventory.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::ActivationTimingDef;
use crate::card::AddManaEffectDef;
use crate::card::AlternativeCastKindDef;
use crate::card::AppliedEffectDef;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::ComparisonDef;
use crate::card::CopyExceptionsDef;
use crate::card::CopyStackObjectDef;
use crate::card::CostDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::DiscardSelectionDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueComparisonDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::ids::TargetIndex;
use crate::mana_cost;

use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y2008::morningtide as catalog_mor;
use crate::card::sets::y2010::magic_2011 as catalog_m11;
use crate::card::sets::y2014::journey_into_nyx as catalog_jou;
use crate::card::sets::y2014::khans_of_tarkir as catalog_ktk;
use crate::card::sets::y2016::kaladesh as catalog_kld;
use crate::card::sets::y2016::shadows_over_innistrad as catalog_soi;
use crate::card::sets::y2019::throne_of_eldraine as catalog_eld;
use crate::card::sets::y2020::ikoria as catalog_iko;
use crate::card::sets::y2022::dominaria_united as catalog_dmu;
use crate::card::sets::y2022::kamigawa_neon_dynasty as catalog_neo;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "BLB",
    slug: "bloomburrow",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

pub(in crate::card::sets) const FORAGE: crate::card::MechanicId =
    crate::card::MechanicId::from_name("mtg:forage");

const fn forage() -> crate::card::GameActionDef {
    const ACTION: crate::card::GameActionDef = crate::card::actions::choice(&[
        crate::card::actions::choose_exile_from_graveyard(3),
        crate::card::actions::choose_sacrifice(1).matching(ObjectPredicateDef::Subtype(
            crate::card::SubtypeDef::Literal("Food"),
        )),
    ]);
    ACTION.named(FORAGE)
}

// BLB 1 — Banishing Light (reprint)
const BANISHING_LIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_jou::BANISHING_LIGHT,
    "25a06f82-ebdb-4dd6-bfe8-958018ce557c",
    "Zoltan Boros",
);

// BLB 2 — Beza, the Bounding Spring
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BEZA_THE_BOUNDING_SPRING: CardRecord = CardRecord::new(
    "Beza, the Bounding Spring",
    "fc310a26-b6a0-4e42-98ab-bdfd7b06cb63",
    "Martin Wittfooth",
    crate::card::CardRules::unsupported(),
);

// BLB 3 — Brave-Kin Duo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRAVE_KIN_DUO: CardRecord = CardRecord::new(
    "Brave-Kin Duo",
    "b8dd4693-424d-4d6e-86cf-24401a23d6b1",
    "Devin Platts",
    crate::card::CardRules::unsupported(),
);

// BLB 4 — Brightblade Stoat
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRIGHTBLADE_STOAT: CardRecord = CardRecord::new(
    "Brightblade Stoat",
    "df7fea2e-7414-4bc8-adb0-9342e174c009",
    "Lius Lasahido",
    crate::card::CardRules::unsupported(),
);

// BLB 5 — Builder's Talent
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BUILDER_S_TALENT: CardRecord = CardRecord::new(
    "Builder's Talent",
    "15fa581a-724e-4196-a9a3-ff84c54bdb7d",
    "Ovidio Cartagena",
    crate::card::CardRules::unsupported(),
);

// BLB 6 — Caretaker's Talent
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CARETAKER_S_TALENT: CardRecord = CardRecord::new(
    "Caretaker's Talent",
    "ad5ea98a-e36e-4ab9-b4da-cc572f3777db",
    "Lindsey Look",
    crate::card::CardRules::unsupported(),
);

// BLB 7 — Carrot Cake
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CARROT_CAKE: CardRecord = CardRecord::new(
    "Carrot Cake",
    "eb03bb4f-8b4b-417e-bfc6-294cd2186b2e",
    "Forrest Imel",
    crate::card::CardRules::unsupported(),
);

// BLB 8 — Crumb and Get It
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRUMB_AND_GET_IT: CardRecord = CardRecord::new(
    "Crumb and Get It",
    "3c7b3b25-d4b3-4451-9f5c-6eb369541175",
    "Justyna Dura",
    crate::card::CardRules::unsupported(),
);

// BLB 9 — Dawn's Truce
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DAWN_S_TRUCE: CardRecord = CardRecord::new(
    "Dawn's Truce",
    "8f72bfa0-efef-48ce-aff8-d5818ed71ba6",
    "Justin Gerard",
    crate::card::CardRules::unsupported(),
);

// BLB 10 — Dewdrop Cure
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEWDROP_CURE: CardRecord = CardRecord::new(
    "Dewdrop Cure",
    "666aefc2-44e0-4c27-88d5-7906f245a71f",
    "Chris Rallis",
    crate::card::CardRules::unsupported(),
);

// BLB 11 — Driftgloom Coyote
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRIFTGLOOM_COYOTE: CardRecord = CardRecord::new(
    "Driftgloom Coyote",
    "d7ab2de3-3aea-461a-a74f-fb742cf8a198",
    "Betty Jiang",
    crate::card::CardRules::unsupported(),
);

// BLB 12 — Essence Channeler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ESSENCE_CHANNELER: CardRecord = CardRecord::new(
    "Essence Channeler",
    "5aaf7e4c-4d5d-4acc-a834-e6c4a7629408",
    "Wylie Beckert",
    crate::card::CardRules::unsupported(),
);

// BLB 13 — Feather of Flight
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FEATHER_OF_FLIGHT: CardRecord = CardRecord::new(
    "Feather of Flight",
    "9fb41503-8632-4bf1-9bfe-6d9b9993c337",
    "Borja Pindado",
    crate::card::CardRules::unsupported(),
);

// BLB 14 — Flowerfoot Swordmaster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLOWERFOOT_SWORDMASTER: CardRecord = CardRecord::new(
    "Flowerfoot Swordmaster",
    "97ff118f-9c3c-43a2-8085-980c7fe7d227",
    "Aaron Miller",
    crate::card::CardRules::unsupported(),
);

// BLB 15 — Harvestrite Host
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HARVESTRITE_HOST: CardRecord = CardRecord::new(
    "Harvestrite Host",
    "41762689-0c13-4d45-9d81-ba2afad980f8",
    "Julie Dillon",
    crate::card::CardRules::unsupported(),
);

// BLB 16 — Hop to It
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HOP_TO_IT: CardRecord = CardRecord::new(
    "Hop to It",
    "ee7207f8-5daa-42af-aeea-7a489047110b",
    "Eelis Kyttanen",
    crate::card::CardRules::unsupported(),
);

// BLB 17 — Intrepid Rabbit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INTREPID_RABBIT: CardRecord = CardRecord::new(
    "Intrepid Rabbit",
    "4d70b99d-c8bf-4a56-8957-cf587fe60b81",
    "Artur Treffner",
    crate::card::CardRules::unsupported(),
);

// BLB 18 — Jackdaw Savior
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JACKDAW_SAVIOR: CardRecord = CardRecord::new(
    "Jackdaw Savior",
    "121af600-6143-450a-9f87-12ce4833f1ec",
    "Alessandra Pisano",
    crate::card::CardRules::unsupported(),
);

// BLB 19 — Jolly Gerbils
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JOLLY_GERBILS: CardRecord = CardRecord::new(
    "Jolly Gerbils",
    "0eab51d6-ba17-4a8c-8834-25db363f2b6b",
    "Manuel Castañón",
    crate::card::CardRules::unsupported(),
);

// BLB 20 — Lifecreed Duo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIFECREED_DUO: CardRecord = CardRecord::new(
    "Lifecreed Duo",
    "ca543405-5e12-48a0-9a77-082ac9bcb2f2",
    "Lorenzo Mastroianni",
    crate::card::CardRules::unsupported(),
);

// BLB 21 — Mabel's Mettle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MABEL_S_METTLE: CardRecord = CardRecord::new(
    "Mabel's Mettle",
    "5cfcf83f-089c-4e35-855e-b61b98bb1cd8",
    "Johan Grenier",
    crate::card::CardRules::unsupported(),
);

// BLB 22 — Mouse Trapper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOUSE_TRAPPER: CardRecord = CardRecord::new(
    "Mouse Trapper",
    "8ba1bc5a-03e7-44ec-893e-44042cbc02ef",
    "Jakub Kasper",
    crate::card::CardRules::unsupported(),
);

// BLB 23 — Nettle Guard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NETTLE_GUARD: CardRecord = CardRecord::new(
    "Nettle Guard",
    "8c9c3cc3-2aa2-453e-a17c-2baeeaabe0a9",
    "Rob Rey",
    crate::card::CardRules::unsupported(),
);

// BLB 24 — Parting Gust
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PARTING_GUST: CardRecord = CardRecord::new(
    "Parting Gust",
    "1086e826-94b8-4398-8a38-d8eacca56a43",
    "Nils Hamm",
    crate::card::CardRules::unsupported(),
);

// BLB 25 — Pileated Provisioner
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PILEATED_PROVISIONER: CardRecord = CardRecord::new(
    "Pileated Provisioner",
    "ae442cd6-c4df-4aad-9b1d-ccd936c5ec96",
    "Eelis Kyttanen",
    crate::card::CardRules::unsupported(),
);

// BLB 26 — Rabbit Response
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RABBIT_RESPONSE: CardRecord = CardRecord::new(
    "Rabbit Response",
    "c4ded450-346d-4917-917a-b62bc0267509",
    "Rovina Cai",
    crate::card::CardRules::unsupported(),
);

// BLB 27 — Repel Calamity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REPEL_CALAMITY: CardRecord = CardRecord::new(
    "Repel Calamity",
    "d068192a-6270-4981-819d-4945fa4a2b83",
    "Ryan Pancoast",
    crate::card::CardRules::unsupported(),
);

// BLB 28 — Salvation Swan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SALVATION_SWAN: CardRecord = CardRecord::new(
    "Salvation Swan",
    "b2656160-d319-4530-a6e5-c418596c3f12",
    "Christina Kraus",
    crate::card::CardRules::unsupported(),
);

// BLB 29 — Season of the Burrow
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEASON_OF_THE_BURROW: CardRecord = CardRecord::new(
    "Season of the Burrow",
    "33bf9c60-4e58-48a4-8e53-abef7ab3b671",
    "Serena Malyon",
    crate::card::CardRules::unsupported(),
);

// BLB 30 — Seasoned Warrenguard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEASONED_WARRENGUARD: CardRecord = CardRecord::new(
    "Seasoned Warrenguard",
    "90873995-876f-4e89-8bc7-41a74f4d931f",
    "Dave Kendall",
    crate::card::CardRules::unsupported(),
);

// BLB 31 — Shrike Force
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHRIKE_FORCE: CardRecord = CardRecord::new(
    "Shrike Force",
    "306fec2c-d8b7-4f4b-8f58-10e3b9f3158f",
    "Néstor Ossandón Leal",
    crate::card::CardRules::unsupported(),
);

// BLB 32 — Sonar Strike
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SONAR_STRIKE: CardRecord = CardRecord::new(
    "Sonar Strike",
    "a50da179-751f-47a8-a547-8c4a291ed381",
    "Julie Dillon",
    crate::card::CardRules::unsupported(),
);

// BLB 33 — Star Charter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STAR_CHARTER: CardRecord = CardRecord::new(
    "Star Charter",
    "0e209237-00f7-4bf0-8287-ccde02ce8e8d",
    "Valera Lutfullina",
    crate::card::CardRules::unsupported(),
);

// BLB 34 — Starfall Invocation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STARFALL_INVOCATION: CardRecord = CardRecord::new(
    "Starfall Invocation",
    "2aea38e6-ec58-4091-b27c-2761bdd12b13",
    "Rob Rey",
    crate::card::CardRules::unsupported(),
);

// BLB 35 — Thistledown Players
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THISTLEDOWN_PLAYERS: CardRecord = CardRecord::new(
    "Thistledown Players",
    "afa8d83f-8586-4127-8b55-9715e9547488",
    "John Thacker",
    crate::card::CardRules::unsupported(),
);

// BLB 36 — Valley Questcaller
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VALLEY_QUESTCALLER: CardRecord = CardRecord::new(
    "Valley Questcaller",
    "ba629ca8-a368-4282-8a61-9bf6a5c217f0",
    "Steve Prescott",
    crate::card::CardRules::unsupported(),
);

// BLB 37 — Warren Elder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WARREN_ELDER: CardRecord = CardRecord::new(
    "Warren Elder",
    "4bf20069-5a20-4f95-976b-6af2b69f3ad0",
    "Kaitlyn McCulley",
    crate::card::CardRules::unsupported(),
);

// BLB 38 — Warren Warleader
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WARREN_WARLEADER: CardRecord = CardRecord::new(
    "Warren Warleader",
    "eb5237a0-5ac3-4ded-9f92-5f782a7bbbd7",
    "Zack Stella",
    crate::card::CardRules::unsupported(),
);

// BLB 39 — Wax-Wane Witness
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WAX_WANE_WITNESS: CardRecord = CardRecord::new(
    "Wax-Wane Witness",
    "d90ea719-5320-46c6-a347-161853a14776",
    "Mila Pesic",
    crate::card::CardRules::unsupported(),
);

// BLB 40 — Whiskervale Forerunner
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WHISKERVALE_FORERUNNER: CardRecord = CardRecord::new(
    "Whiskervale Forerunner",
    "60a78d59-af31-4af9-95aa-2573fe553925",
    "Ryan Pancoast",
    crate::card::CardRules::unsupported(),
);

// BLB 41 — Azure Beastbinder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AZURE_BEASTBINDER: CardRecord = CardRecord::new(
    "Azure Beastbinder",
    "211af1bf-910b-41a5-b928-f378188d1871",
    "Adam Paquette",
    crate::card::CardRules::unsupported(),
);

// BLB 42 — Bellowing Crier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BELLOWING_CRIER: CardRecord = CardRecord::new(
    "Bellowing Crier",
    "ca2215dd-6300-49cf-b9b2-3a840b786c31",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// BLB 43 — Calamitous Tide
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CALAMITOUS_TIDE: CardRecord = CardRecord::new(
    "Calamitous Tide",
    "178bc8b2-ffa0-4549-aead-aacb3db3cf19",
    "Samuele Bandini",
    crate::card::CardRules::unsupported(),
);

// BLB 44 — Daring Waverider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARING_WAVERIDER: CardRecord = CardRecord::new(
    "Daring Waverider",
    "19422406-0c1a-497e-bed1-708bc556491a",
    "Wisnu Tan",
    crate::card::CardRules::unsupported(),
);

// BLB 45 — Dazzling Denial
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DAZZLING_DENIAL: CardRecord = CardRecord::new(
    "Dazzling Denial",
    "8739f1ac-2e57-4b52-a7ff-cc8df5936aad",
    "Kisung Koh",
    crate::card::CardRules::unsupported(),
);

// BLB 46 — Dire Downdraft
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIRE_DOWNDRAFT: CardRecord = CardRecord::new(
    "Dire Downdraft",
    "f1931f22-974c-43ad-911e-684bf3f9995d",
    "Martin Wittfooth",
    crate::card::CardRules::unsupported(),
);

// BLB 47 — Dour Port-Mage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DOUR_PORT_MAGE: CardRecord = CardRecord::new(
    "Dour Port-Mage",
    "6402133e-eed1-4a46-9667-8b7a310362c1",
    "Ryan Pancoast",
    crate::card::CardRules::unsupported(),
);

// BLB 48 — Eddymurk Crab
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EDDYMURK_CRAB: CardRecord = CardRecord::new(
    "Eddymurk Crab",
    "e6d45abe-4962-47d9-a54e-7e623ea8647c",
    "PINDURSKI",
    crate::card::CardRules::unsupported(),
);

// BLB 49 — Eluge, the Shoreless Sea
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELUGE_THE_SHORELESS_SEA: CardRecord = CardRecord::new(
    "Eluge, the Shoreless Sea",
    "1f2bf6ba-cd1a-4382-9572-6dfbcf6ed0c6",
    "Chase Stone",
    crate::card::CardRules::unsupported(),
);

// BLB 50 — Finch Formation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FINCH_FORMATION: CardRecord = CardRecord::new(
    "Finch Formation",
    "1c671eab-d1ef-4d79-94eb-8b85f0d18699",
    "Rhonda Libbey",
    crate::card::CardRules::unsupported(),
);

// BLB 51 — Gossip's Talent
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOSSIP_S_TALENT: CardRecord = CardRecord::new(
    "Gossip's Talent",
    "b299889a-03d6-4659-b0e1-f0830842e40f",
    "Andrea Sipl",
    crate::card::CardRules::unsupported(),
);

// BLB 52 — Into the Flood Maw
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INTO_THE_FLOOD_MAW: CardRecord = CardRecord::new(
    "Into the Flood Maw",
    "50b9575a-53d9-4df7-b86c-cda021107d3f",
    "Danny Schwartz",
    crate::card::CardRules::unsupported(),
);

// BLB 53 — Kitnap
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KITNAP: CardRecord = CardRecord::new(
    "Kitnap",
    "085be5d1-fd85-46d1-ad39-a8aa75a06a96",
    "Irina Nordsol",
    crate::card::CardRules::unsupported(),
);

// BLB 54 — Kitsa, Otterball Elite
pub(in crate::card::sets) static KITSA_OTTERBALL_ELITE: CardRecord = CardRecord::new(
    "Kitsa, Otterball Elite",
    "c8ff751a-ec64-41d5-b22c-2a483ad9a9b2",
    "Zoltan Boros",
// Two mana for a body that loots every turn it has nothing better to
    // do, and copies the spell that made it big enough on the turns it
    // does. Vigilance is why the tap is not a real cost.
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Otter", "Wizard"], 1, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::vigilance(),
            abilities::prowess(),
            AbilityDef::activated(
                "{T}: Draw a card, then discard a card.",
                &[CostDef::TapSource],
                EffectDef::Sequence(&[
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::Discard {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                        selection: DiscardSelectionDef::RecipientChooses,
                        then: None,
                    },
                ]),
            ),
            AbilityDef::activated_with_targets(
                "{2}, {T}: Copy target instant or sorcery spell you control. You may choose new targets \
                 for the copy. Activate only if Kitsa's power is 3 or greater.",
                &[
                    CostDef::Mana(mana_cost!("{2}")),
                    CostDef::TapSource,
                ],
                // Yours rather than anybody's: Kitsa copies what you are casting, not what
                // is being cast at you.
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
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::CopyStackObject(&CopyStackObjectDef {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    controller: PlayerRefDef::EffectController,
                    count: ValueDef::Constant(1),
                    retarget: true,
                    colors: None,
                }),
            )
            // Read live where the activation is offered, so the prowess trigger from
            // the spell being copied is what turns the ability on: a 1/3 that has cast
            // two noncreature spells this turn is a 3/5.
            .with_activation_condition(&TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::SourcePower,
                    comparison: ComparisonDef::GreaterOrEqual,
                    right: ValueDef::Constant(3),
                })),
        ]),
);

// BLB 55 — Knightfisher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KNIGHTFISHER: CardRecord = CardRecord::new(
    "Knightfisher",
    "c7fb7f4f-2153-4527-8f11-adbf508d3533",
    "Jakob Eirich",
    crate::card::CardRules::unsupported(),
);

// BLB 56 — Lightshell Duo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIGHTSHELL_DUO: CardRecord = CardRecord::new(
    "Lightshell Duo",
    "2f00c834-b4a9-45b8-bb3f-22c2a42314a0",
    "Mariah Tekulve",
    crate::card::CardRules::unsupported(),
);

// BLB 57 — Long River Lurker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LONG_RIVER_LURKER: CardRecord = CardRecord::new(
    "Long River Lurker",
    "7c267719-cd03-4003-b281-e732d5e42a1e",
    "Valera Lutfullina",
    crate::card::CardRules::unsupported(),
);

// BLB 58 — Long River's Pull
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LONG_RIVER_S_PULL: CardRecord = CardRecord::new(
    "Long River's Pull",
    "1c81d0fa-81a1-4f9b-a5fd-5a648fd01dea",
    "Raph Lomotan",
    crate::card::CardRules::unsupported(),
);

// BLB 59 — Mind Spiral
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIND_SPIRAL: CardRecord = CardRecord::new(
    "Mind Spiral",
    "7e24fe6a-607b-49b8-9fca-cecb1e40de7f",
    "Filip Burburan",
    crate::card::CardRules::unsupported(),
);

// BLB 60 — Mindwhisker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MINDWHISKER: CardRecord = CardRecord::new(
    "Mindwhisker",
    "aaa10f34-5bfd-4d87-8f07-58de3b0f5663",
    "Alexander Mokhov",
    crate::card::CardRules::unsupported(),
);

// BLB 61 — Mockingbird
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOCKINGBIRD: CardRecord = CardRecord::new(
    "Mockingbird",
    "ade32396-8841-4ba4-8852-d11146607f21",
    "Aurore Folny",
    crate::card::CardRules::unsupported(),
);

// BLB 62 — Nightwhorl Hermit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NIGHTWHORL_HERMIT: CardRecord = CardRecord::new(
    "Nightwhorl Hermit",
    "0928e04f-2568-41e8-b603-7a25cf5f94d0",
    "Valera Lutfullina",
    crate::card::CardRules::unsupported(),
);

// BLB 63 — Otterball Antics
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OTTERBALL_ANTICS: CardRecord = CardRecord::new(
    "Otterball Antics",
    "3ff83ff7-e428-4ccc-8341-f223dab76bd1",
    "Rhonda Libbey",
    crate::card::CardRules::unsupported(),
);

// BLB 64 — Pearl of Wisdom
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PEARL_OF_WISDOM: CardRecord = CardRecord::new(
    "Pearl of Wisdom",
    "13cb9575-1138-4f99-8e90-0eaf00bdf4a1",
    "Julie Dillon",
    crate::card::CardRules::unsupported(),
);

// BLB 65 — Plumecreed Escort
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PLUMECREED_ESCORT: CardRecord = CardRecord::new(
    "Plumecreed Escort",
    "f71320ed-2f30-49ce-bcb0-19aebba3f0e8",
    "Manuel Castañón",
    crate::card::CardRules::unsupported(),
);

// BLB 66 — Portent of Calamity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PORTENT_OF_CALAMITY: CardRecord = CardRecord::new(
    "Portent of Calamity",
    "8599e2dd-9164-4da3-814f-adccef3b9497",
    "Sam Guay",
    crate::card::CardRules::unsupported(),
);

// BLB 67 — Run Away Together (reprint)
const RUN_AWAY_TOGETHER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_eld::RUN_AWAY_TOGETHER,
    "7cb7ec70-a5a4-4188-ba1a-e88b81bdbad0",
    "Omar Rayyan",
);

// BLB 68 — Season of Weaving
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEASON_OF_WEAVING: CardRecord = CardRecord::new(
    "Season of Weaving",
    "f5713bb4-bdd9-4253-b6b9-e590532ed773",
    "Wylie Beckert",
    crate::card::CardRules::unsupported(),
);

// BLB 69 — Shore Up (reprint)
const SHORE_UP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dmu::SHORE_UP,
    "4dc3b49e-3674-494c-bdea-4374cefd10f4",
    "Raph Lomotan",
);

// BLB 70 — Shoreline Looter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHORELINE_LOOTER: CardRecord = CardRecord::new(
    "Shoreline Looter",
    "d5bf8cf0-419a-4dc9-9342-aad55c1af05a",
    "PINDURSKI",
    crate::card::CardRules::unsupported(),
);

// BLB 71 — Skyskipper Duo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKYSKIPPER_DUO: CardRecord = CardRecord::new(
    "Skyskipper Duo",
    "d6844bad-ffbe-4c6e-b438-08562eccea52",
    "Mariah Tekulve",
    crate::card::CardRules::unsupported(),
);

// BLB 72 — Spellgyre
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPELLGYRE: CardRecord = CardRecord::new(
    "Spellgyre",
    "f6f6620a-1d40-429d-9a0c-aaeb62adaa71",
    "Alix Branwyn",
    crate::card::CardRules::unsupported(),
);

// BLB 73 — Splash Lasher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPLASH_LASHER: CardRecord = CardRecord::new(
    "Splash Lasher",
    "362ee125-35a0-46cd-a201-e6797d12d33a",
    "Brian Valeza",
    crate::card::CardRules::unsupported(),
);

// BLB 74 — Splash Portal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPLASH_PORTAL: CardRecord = CardRecord::new(
    "Splash Portal",
    "adbaa356-28ba-487f-930a-a957d9960ab0",
    "Caio Monteiro",
    crate::card::CardRules::unsupported(),
);

// BLB 75 — Stormchaser's Talent
static MAKE_AN_OTTER: EffectDef = EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
    TokenCharacteristics::creature(&["Otter"], &[ManaColor::Blue, ManaColor::Red], 1, 1)
        .with_abilities(&[abilities::prowess()])
        .with_art(CardArt::new(
            "e6b2c465-c446-4dee-9101-763105dcf813",
            "Julia Griffin",
        )),
)));

pub(in crate::card::sets) static STORMCHASERS_TALENT: CardRecord = CardRecord::new(
    "Stormchaser's Talent",
    "a36e682d-b43d-4e08-bf5b-70d7e924dbe5",
    "Christina Kraus",
// One mana for a body, and a mana sink that buys back a spell and then
    // turns every cantrip afterwards into another creature.
    CardRules::new_enchantment(mana_cost!("{U}"))
        .with_subtypes(&["Class"])
        .with_abilities(&[
            abilities::enters_trigger(
                "When this Class enters, create a 1/1 blue and red Otter creature token with prowess.",
                MAKE_AN_OTTER,
            ),
            AbilityDef::activated(
                "{3}{U}: Level 2",
                &[CostDef::Mana(mana_cost!("{3}{U}"))],
                EffectDef::GainClassLevel { level: 2 },
            )
            .with_activation_timing(ActivationTimingDef::SorcerySpeed)
            // A Class is level 1 with no counters on it, so climbing to two takes one
            // counter and to three takes two. Each level is bought separately, only at
            // sorcery speed (CR 717.2b), and only from the level directly below it:
            // "you can't activate the first level ability of a Class unless that Class
            // is level 1."
            .with_activation_condition(&TriggerConditionDef::SourceCounters {
                kind: CounterKind::named("level"),
                comparison: ComparisonDef::Equal,
                amount: 0,
            }),
            AbilityDef::triggered_with_targets(
                "When this Class becomes level 2, return target instant or sorcery card from your \
                 graveyard to your hand.",
                TriggerEventDef::BecomesLevel(2),
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
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            ),
            AbilityDef::activated(
                "{5}{U}: Level 3",
                &[CostDef::Mana(mana_cost!("{5}{U}"))],
                EffectDef::GainClassLevel { level: 3 },
            )
            .with_activation_timing(ActivationTimingDef::SorcerySpeed)
            // Level 2 and no further: a Class at level 1 cannot buy its way straight
            // to three, and one already at three has nothing left to buy.
            .with_activation_condition(&TriggerConditionDef::SourceCounters {
                kind: CounterKind::named("level"),
                comparison: ComparisonDef::Equal,
                amount: 1,
            }),
            AbilityDef::triggered_if(
                "Whenever you cast an instant or sorcery spell, create a 1/1 blue and red Otter creature \
                 token with prowess.",
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                // The level-3 clause only functions once the Class is there. Written as an
                // intervening-if, which is checked when it would trigger and again as it
                // resolves -- so a Class knocked back down between the two does not make
                // the Otter.
                &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("level"),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 2,
                },
                MAKE_AN_OTTER,
            ),
        ]),
);

// BLB 76 — Sugar Coat
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUGAR_COAT: CardRecord = CardRecord::new(
    "Sugar Coat",
    "fcacbe71-efb0-49e1-b2d0-3ee65ec6cf8b",
    "Gaboleps",
    crate::card::CardRules::unsupported(),
);

// BLB 77 — Thought Shucker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THOUGHT_SHUCKER: CardRecord = CardRecord::new(
    "Thought Shucker",
    "44b0d83b-cc41-4f82-892c-ef6d3293228a",
    "Dave Kendall",
    crate::card::CardRules::unsupported(),
);

// BLB 78 — Thundertrap Trainer
static TRAINER_ARRIVES: TriggerEventDef = TriggerEventDef::zone_changed(
    ObjectPredicateDef::Source,
    None,
    Some(ZoneKind::Battlefield),
);

pub(in crate::card::sets) static THUNDERTRAP_TRAINER: CardRecord = CardRecord::new(
    "Thundertrap Trainer",
    "9cf3af94-b7c8-415c-a5a1-d89967fd0bba",
    "Matt Stewart",
    // Two mana to dig four cards deep for the spell you want, or six for two
    // bodies and two looks.
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Otter", "Wizard"], 1, 2).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{5}{U}"))],
            AlternativeCastKindDef::Offspring,
            Some(
                "Offspring {4} (You may pay an additional {4} as you cast this spell. If you do, \
                 when this creature enters, create a 1/1 token copy of it.)",
            ),
            EffectDef::None,
        ),
        AbilityDef::triggered_if(
            "When this creature enters, create a 1/1 token copy of it.",
            TRAINER_ARRIVES,
            // "If you do, when this creature enters": the arrival asks what the cast
            // paid, which the permanent recorded as it arrived.
            &TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Offspring),
            // A 1/1 copy of himself, which arrives with his own look at four attached
            // to it -- the whole reason the extra four mana is worth paying.
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Copy(
                &crate::card::TokenCopyDef {
                    object: &EffectRecipientDef::Source,
                    exceptions: CopyExceptionsDef::power_toughness(1, 1),
                },
            ))),
        ),
        AbilityDef::triggered(
            "When this creature enters, look at the top four cards of your library. You may \
             reveal a noncreature, nonland card from among them and put it into your hand. Put \
             the rest on the bottom of your library in a random order.",
            TRAINER_ARRIVES,
            abilities::look_at_top_cards_reveal_choice_to_hand_rest_random_bottom(
                ValueDef::Constant(4),
                // A noncreature, nonland card among the four, which is what the deck
                // playing him is digging for.
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                ]),
                0,
                1,
            ),
        ),
    ]),
);

// BLB 79 — Valley Floodcaller
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VALLEY_FLOODCALLER: CardRecord = CardRecord::new(
    "Valley Floodcaller",
    "90b12da0-f666-471d-95f5-15d8c9b31c92",
    "Victor Adame Minguez",
    crate::card::CardRules::unsupported(),
);

// BLB 80 — Waterspout Warden
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WATERSPOUT_WARDEN: CardRecord = CardRecord::new(
    "Waterspout Warden",
    "35898b39-98e2-405b-8f18-0e054bd2c29e",
    "J.P. Targete",
    crate::card::CardRules::unsupported(),
);

// BLB 81 — Wishing Well
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WISHING_WELL: CardRecord = CardRecord::new(
    "Wishing Well",
    "edeb20aa-b253-49b8-9947-c397a3a4002a",
    "Steven Belledin",
    crate::card::CardRules::unsupported(),
);

// BLB 82 — Agate-Blade Assassin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AGATE_BLADE_ASSASSIN: CardRecord = CardRecord::new(
    "Agate-Blade Assassin",
    "39ebb84a-1c52-4b07-9bd0-b360523b3a5b",
    "Hristo D. Chukov",
    crate::card::CardRules::unsupported(),
);

// BLB 83 — Bandit's Talent
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BANDIT_S_TALENT: CardRecord = CardRecord::new(
    "Bandit's Talent",
    "485dc8d8-9e44-4a0f-9ff6-fa448e232290",
    "Volkan Baǵa",
    crate::card::CardRules::unsupported(),
);

// BLB 84 — Bonebind Orator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BONEBIND_ORATOR: CardRecord = CardRecord::new(
    "Bonebind Orator",
    "faf226fa-ca09-4468-8804-87b2a7de2c66",
    "Tuan Duong Chu",
    crate::card::CardRules::unsupported(),
);

// BLB 85 — Bonecache Overseer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BONECACHE_OVERSEER: CardRecord = CardRecord::new(
    "Bonecache Overseer",
    "82defb87-237f-4b77-9673-5bf00607148f",
    "Mila Pesic",
    crate::card::CardRules::unsupported(),
);

// BLB 86 — Coiling Rebirth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COILING_REBIRTH: CardRecord = CardRecord::new(
    "Coiling Rebirth",
    "96d5de3e-0440-4dd1-899c-ab40c0752343",
    "Rovina Cai",
    crate::card::CardRules::unsupported(),
);

// BLB 87 — Consumed by Greed
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CONSUMED_BY_GREED: CardRecord = CardRecord::new(
    "Consumed by Greed",
    "e50acc41-3517-42db-b1d3-1bdfd7294d84",
    "Mathias Kollros",
    crate::card::CardRules::unsupported(),
);

// BLB 88 — Cruelclaw's Heist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRUELCLAW_S_HEIST: CardRecord = CardRecord::new(
    "Cruelclaw's Heist",
    "cab4539a-0157-4cbe-b50f-6e2575df74e9",
    "Brian Valeza",
    crate::card::CardRules::unsupported(),
);

// BLB 89 — Daggerfang Duo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DAGGERFANG_DUO: CardRecord = CardRecord::new(
    "Daggerfang Duo",
    "cea2bb34-e328-44fb-918a-72208c9457e4",
    "Nereida",
    crate::card::CardRules::unsupported(),
);

// BLB 90 — Darkstar Augur
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARKSTAR_AUGUR: CardRecord = CardRecord::new(
    "Darkstar Augur",
    "1c603751-1e2b-4c8e-a8d2-5c0876e7254f",
    "Aurore Folny",
    crate::card::CardRules::unsupported(),
);

// BLB 91 — Diresight
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIRESIGHT: CardRecord = CardRecord::new(
    "Diresight",
    "fada29c0-5293-40a4-b36d-d073ee99e650",
    "Alix Branwyn",
    crate::card::CardRules::unsupported(),
);

// BLB 92 — Downwind Ambusher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DOWNWIND_AMBUSHER: CardRecord = CardRecord::new(
    "Downwind Ambusher",
    "55cfd628-933a-4d3d-b2e5-70bc86960d1c",
    "Aaron Miller",
    crate::card::CardRules::unsupported(),
);

// BLB 93 — Early Winter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EARLY_WINTER: CardRecord = CardRecord::new(
    "Early Winter",
    "5030e6ac-211d-4145-8c87-998a8351a467",
    "Andrew Mar",
    crate::card::CardRules::unsupported(),
);

// BLB 94 — Feed the Cycle
pub(in crate::card::sets) static FEED_THE_CYCLE: CardRecord = CardRecord::new(
    "Feed the Cycle",
    "7e017ff8-2936-4a1b-bece-00004cfbad06",
    "Donato Giancola",
    CardRules::new_instant(mana_cost!("{1}{B}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, forage or pay {B}. (To forage, exile \
             three cards from your graveyard or sacrifice a Food.)\nDestroy target creature or \
             planeswalker.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                ]),
            )],
            CostDef::choice(&[forage().as_cost(), CostDef::pay_mana(mana_cost!("{B}"))]),
            EffectDef::destroy_target(TargetIndex::PRIMARY),
        ),
    ),
);

// BLB 95 — Fell
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FELL: CardRecord = CardRecord::new(
    "Fell",
    "c96ac326-de44-470b-a592-a4c2a052c091",
    "A. M. Sartor",
    crate::card::CardRules::unsupported(),
);

// BLB 96 — Glidedive Duo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLIDEDIVE_DUO: CardRecord = CardRecord::new(
    "Glidedive Duo",
    "4831e7ae-54e3-4bd9-b5af-52dc29f81715",
    "Manuel Castañón",
    crate::card::CardRules::unsupported(),
);

// BLB 97 — Hazel's Nocturne
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HAZEL_S_NOCTURNE: CardRecord = CardRecord::new(
    "Hazel's Nocturne",
    "239363df-4de8-4b64-80fc-a1f4b5c36027",
    "Dominik Mayer",
    crate::card::CardRules::unsupported(),
);

// BLB 98 — Huskburster Swarm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HUSKBURSTER_SWARM: CardRecord = CardRecord::new(
    "Huskburster Swarm",
    "ed2f61d7-4eb0-41c5-8a34-a0793c2abc51",
    "John Tedrick",
    crate::card::CardRules::unsupported(),
);

// BLB 99 — Iridescent Vinelasher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IRIDESCENT_VINELASHER: CardRecord = CardRecord::new(
    "Iridescent Vinelasher",
    "b2bc854c-4e72-48e0-a098-e3451d6e511d",
    "Aaron Miller",
    crate::card::CardRules::unsupported(),
);

// BLB 100 — Maha, Its Feathers Night
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAHA_ITS_FEATHERS_NIGHT: CardRecord = CardRecord::new(
    "Maha, Its Feathers Night",
    "cf3320ec-c4e8-405a-982d-e009c58c9e21",
    "Alessandra Pisano",
    crate::card::CardRules::unsupported(),
);

// BLB 101 — Moonstone Harbinger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOONSTONE_HARBINGER: CardRecord = CardRecord::new(
    "Moonstone Harbinger",
    "59e4aa8d-1d06-48db-b205-aa2f1392bbcb",
    "Kevin Sidharta",
    crate::card::CardRules::unsupported(),
);

// BLB 102 — Nocturnal Hunger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NOCTURNAL_HUNGER: CardRecord = CardRecord::new(
    "Nocturnal Hunger",
    "742c0409-9abd-4559-b52e-932cc90c531a",
    "Sam Guay",
    crate::card::CardRules::unsupported(),
);

// BLB 103 — Osteomancer Adept
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OSTEOMANCER_ADEPT: CardRecord = CardRecord::new(
    "Osteomancer Adept",
    "7d8238dd-858f-466c-96de-986bd66861d7",
    "Daniel Zrom",
    crate::card::CardRules::unsupported(),
);

// BLB 104 — Persistent Marshstalker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PERSISTENT_MARSHSTALKER: CardRecord = CardRecord::new(
    "Persistent Marshstalker",
    "8b900c71-713b-4b7e-b4be-ad9f4aa0c139",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// BLB 105 — Psychic Whorl
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PSYCHIC_WHORL: CardRecord = CardRecord::new(
    "Psychic Whorl",
    "df900308-8432-4a0a-be21-17482026012b",
    "Eli Minaya",
    crate::card::CardRules::unsupported(),
);

// BLB 106 — Ravine Raider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAVINE_RAIDER: CardRecord = CardRecord::new(
    "Ravine Raider",
    "874510be-7ecd-4eff-abad-b9594eb4821a",
    "Simon Dominic",
    crate::card::CardRules::unsupported(),
);

// BLB 107 — Rottenmouth Viper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROTTENMOUTH_VIPER: CardRecord = CardRecord::new(
    "Rottenmouth Viper",
    "735e79b1-a3a9-4ddf-8bbc-f756c8a0452b",
    "Andrea Piparo",
    crate::card::CardRules::unsupported(),
);

// BLB 108 — Ruthless Negotiation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUTHLESS_NEGOTIATION: CardRecord = CardRecord::new(
    "Ruthless Negotiation",
    "c7f4360c-8d68-4058-b9ec-da9948cb060d",
    "Rhonda Libbey",
    crate::card::CardRules::unsupported(),
);

// BLB 109 — Savor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAVOR: CardRecord = CardRecord::new(
    "Savor",
    "1397f689-dca1-4d35-864b-92c5606afb9a",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// BLB 110 — Scales of Shale
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCALES_OF_SHALE: CardRecord = CardRecord::new(
    "Scales of Shale",
    "9ae14276-dbbd-4257-80e9-accd6c19f5b2",
    "Yohann Schepacz",
    crate::card::CardRules::unsupported(),
);

// BLB 111 — Scavenger's Talent
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCAVENGER_S_TALENT: CardRecord = CardRecord::new(
    "Scavenger's Talent",
    "9a52b7fe-87ae-425b-85fd-b24e6e0395f1",
    "Chris Seaman",
    crate::card::CardRules::unsupported(),
);

// BLB 112 — Season of Loss
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEASON_OF_LOSS: CardRecord = CardRecord::new(
    "Season of Loss",
    "cc540652-916b-45c5-ae5a-0a0bc557cee1",
    "Dominik Mayer",
    crate::card::CardRules::unsupported(),
);

// BLB 113 — Sinister Monolith
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SINISTER_MONOLITH: CardRecord = CardRecord::new(
    "Sinister Monolith",
    "2a15e06c-2608-4e7a-a16c-d35417669d86",
    "Adam Paquette",
    crate::card::CardRules::unsupported(),
);

// BLB 114 — Stargaze
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STARGAZE: CardRecord = CardRecord::new(
    "Stargaze",
    "777fc599-8de7-44d2-8fdd-9bddf5948a0c",
    "Serena Malyon",
    crate::card::CardRules::unsupported(),
);

// BLB 115 — Starlit Soothsayer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STARLIT_SOOTHSAYER: CardRecord = CardRecord::new(
    "Starlit Soothsayer",
    "184c1eca-2991-438f-b5d2-cd2529b9c9b4",
    "Kaitlyn McCulley",
    crate::card::CardRules::unsupported(),
);

// BLB 116 — Starscape Cleric
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STARSCAPE_CLERIC: CardRecord = CardRecord::new(
    "Starscape Cleric",
    "53a938a7-0154-4350-87cb-00da24ec3824",
    "Omar Rayyan",
    crate::card::CardRules::unsupported(),
);

// BLB 117 — Thornplate Intimidator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THORNPLATE_INTIMIDATOR: CardRecord = CardRecord::new(
    "Thornplate Intimidator",
    "42f66c4a-feaa-4ba6-aa56-955b43329a9e",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// BLB 118 — Thought-Stalker Warlock
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THOUGHT_STALKER_WARLOCK: CardRecord = CardRecord::new(
    "Thought-Stalker Warlock",
    "42e80284-d489-493b-ae92-95b742d07cb3",
    "Daniel Ljunggren",
    crate::card::CardRules::unsupported(),
);

// BLB 119 — Valley Rotcaller
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VALLEY_ROTCALLER: CardRecord = CardRecord::new(
    "Valley Rotcaller",
    "4da80a9a-b1d5-4fc5-92f7-36946195d0c7",
    "Valera Lutfullina",
    crate::card::CardRules::unsupported(),
);

// BLB 120 — Wick, the Whorled Mind
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WICK_THE_WHORLED_MIND: CardRecord = CardRecord::new(
    "Wick, the Whorled Mind",
    "29089810-d7fb-4abe-b729-bfabed6aed2b",
    "Andrea Piparo",
    crate::card::CardRules::unsupported(),
);

// BLB 121 — Wick's Patrol
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WICK_S_PATROL: CardRecord = CardRecord::new(
    "Wick's Patrol",
    "5fa0c53d-fe7b-4b8b-ad81-7967ca318ff7",
    "Dmitry Burmak",
    crate::card::CardRules::unsupported(),
);

// BLB 122 — Agate Assault
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AGATE_ASSAULT: CardRecord = CardRecord::new(
    "Agate Assault",
    "7dd9946b-515e-4e0d-9da2-711e126e9fa6",
    "Slawomir Maniak",
    crate::card::CardRules::unsupported(),
);

// BLB 123 — Alania's Pathmaker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALANIA_S_PATHMAKER: CardRecord = CardRecord::new(
    "Alania's Pathmaker",
    "d3871fe6-e26e-4ab4-bd81-7e3c7b8135c1",
    "Jason Kang",
    crate::card::CardRules::unsupported(),
);

// BLB 124 — Artist's Talent
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARTIST_S_TALENT: CardRecord = CardRecord::new(
    "Artist's Talent",
    "8b9e51d9-189b-4dd6-87cb-628ea6373e81",
    "Lars Grant-West",
    crate::card::CardRules::unsupported(),
);

// BLB 125 — Blacksmith's Talent
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLACKSMITH_S_TALENT: CardRecord = CardRecord::new(
    "Blacksmith's Talent",
    "4bb318fa-481d-40a7-978e-f01b49101ae0",
    "Vincent Christiaens",
    crate::card::CardRules::unsupported(),
);

// BLB 126 — Blooming Blast
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLOOMING_BLAST: CardRecord = CardRecord::new(
    "Blooming Blast",
    "0cd92a83-cec3-4085-a929-3f204e3e0140",
    "Jakob Eirich",
    crate::card::CardRules::unsupported(),
);

// BLB 127 — Brambleguard Captain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRAMBLEGUARD_CAPTAIN: CardRecord = CardRecord::new(
    "Brambleguard Captain",
    "e200b8bf-f2f3-4157-8e04-02baf07a963e",
    "Quintin Gleim",
    crate::card::CardRules::unsupported(),
);

// BLB 128 — Brazen Collector
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRAZEN_COLLECTOR: CardRecord = CardRecord::new(
    "Brazen Collector",
    "78b55a58-c669-4dc6-aa63-5d9dff52e613",
    "Aldo Domínguez",
    crate::card::CardRules::unsupported(),
);

// BLB 129 — Byway Barterer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BYWAY_BARTERER: CardRecord = CardRecord::new(
    "Byway Barterer",
    "f41fc718-641b-4f32-a8c1-3e5591a05bf8",
    "Ryan Pancoast",
    crate::card::CardRules::unsupported(),
);

// BLB 130 — Conduct Electricity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CONDUCT_ELECTRICITY: CardRecord = CardRecord::new(
    "Conduct Electricity",
    "2f373dd6-2412-453c-85ba-10230dfe473a",
    "Chris Seaman",
    crate::card::CardRules::unsupported(),
);

// BLB 131 — Coruscation Mage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CORUSCATION_MAGE: CardRecord = CardRecord::new(
    "Coruscation Mage",
    "dc2c1de0-6233-469a-be72-a050b97d2c8f",
    "Gaboleps",
    crate::card::CardRules::unsupported(),
);

// BLB 132 — Dragonhawk, Fate's Tempest
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRAGONHAWK_FATE_S_TEMPEST: CardRecord = CardRecord::new(
    "Dragonhawk, Fate's Tempest",
    "8659789c-6a2c-439f-a348-b9b1b06c55b8",
    "Victor Adame Minguez",
    crate::card::CardRules::unsupported(),
);

// BLB 133 — Emberheart Challenger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EMBERHEART_CHALLENGER: CardRecord = CardRecord::new(
    "Emberheart Challenger",
    "0035082e-bb86-4f95-be48-ffc87fe5286d",
    "Chris Rahn",
    crate::card::CardRules::unsupported(),
);

// BLB 134 — Festival of Embers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FESTIVAL_OF_EMBERS: CardRecord = CardRecord::new(
    "Festival of Embers",
    "4433ee12-2013-4fdc-979f-ae065f63a527",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// BLB 135 — Flamecache Gecko
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLAMECACHE_GECKO: CardRecord = CardRecord::new(
    "Flamecache Gecko",
    "fb8e7c97-8393-41b8-bb0b-3983dcc5e7f4",
    "Brian Valeza",
    crate::card::CardRules::unsupported(),
);

// BLB 136 — Frilled Sparkshooter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FRILLED_SPARKSHOOTER: CardRecord = CardRecord::new(
    "Frilled Sparkshooter",
    "674bbd6d-e329-42cf-963d-88d1ce8fe51e",
    "Danny Schwartz",
    crate::card::CardRules::unsupported(),
);

// BLB 137 — Harnesser of Storms
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HARNESSER_OF_STORMS: CardRecord = CardRecord::new(
    "Harnesser of Storms",
    "b56beeb6-88ca-475e-8654-1d4e8b4aa3c0",
    "Bram Sels",
    crate::card::CardRules::unsupported(),
);

// BLB 138 — Heartfire Hero
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEARTFIRE_HERO: CardRecord = CardRecord::new(
    "Heartfire Hero",
    "48ace959-66b2-40c8-9bff-fd7ed9c99a82",
    "Jakub Kasper",
    crate::card::CardRules::unsupported(),
);

// BLB 139 — Hearthborn Battler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEARTHBORN_BATTLER: CardRecord = CardRecord::new(
    "Hearthborn Battler",
    "bef1cf5c-9738-4062-8cb1-87a372d36687",
    "Zoltan Boros",
    crate::card::CardRules::unsupported(),
);

// BLB 140 — Hired Claw
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HIRED_CLAW: CardRecord = CardRecord::new(
    "Hired Claw",
    "1ae41080-0d67-4719-adb2-49bf2a268b6c",
    "Quintin Gleim",
    crate::card::CardRules::unsupported(),
);

// BLB 141 — Hoarder's Overflow
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HOARDER_S_OVERFLOW: CardRecord = CardRecord::new(
    "Hoarder's Overflow",
    "c2ed5079-07b4-4575-a2c8-5f0cbff888c3",
    "Andrea Radeck",
    crate::card::CardRules::unsupported(),
);

// BLB 142 — Kindlespark Duo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KINDLESPARK_DUO: CardRecord = CardRecord::new(
    "Kindlespark Duo",
    "a839fba3-1b66-4dd1-bf43-9b015b44fc81",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// BLB 143 — Manifold Mouse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MANIFOLD_MOUSE: CardRecord = CardRecord::new(
    "Manifold Mouse",
    "db3832b5-e83f-4569-bd49-fb7b86fa2d47",
    "Randy Vargas",
    crate::card::CardRules::unsupported(),
);

// BLB 144 — Might of the Meek
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIGHT_OF_THE_MEEK: CardRecord = CardRecord::new(
    "Might of the Meek",
    "509bf254-8a2b-4dfa-9ae5-386321b35e8b",
    "Danny Schwartz",
    crate::card::CardRules::unsupported(),
);

// BLB 145 — Playful Shove
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PLAYFUL_SHOVE: CardRecord = CardRecord::new(
    "Playful Shove",
    "07956edf-34c1-4218-9784-ddbca13e380c",
    "Zoltan Boros",
    crate::card::CardRules::unsupported(),
);

// BLB 146 — Quaketusk Boar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static QUAKETUSK_BOAR: CardRecord = CardRecord::new(
    "Quaketusk Boar",
    "2f2b7fd3-a139-49ea-8a89-b64261e868ef",
    "Andrew Mar",
    crate::card::CardRules::unsupported(),
);

// BLB 147 — Rabid Gnaw
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RABID_GNAW: CardRecord = CardRecord::new(
    "Rabid Gnaw",
    "2f815bae-820a-49f6-8eed-46f658e7b6ff",
    "Mark Behm",
    crate::card::CardRules::unsupported(),
);

// BLB 148 — Raccoon Rallier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RACCOON_RALLIER: CardRecord = CardRecord::new(
    "Raccoon Rallier",
    "b5b5180f-5a1c-4df8-9019-195e65a50ce3",
    "Borja Pindado",
    crate::card::CardRules::unsupported(),
);

// BLB 149 — Reptilian Recruiter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REPTILIAN_RECRUITER: CardRecord = CardRecord::new(
    "Reptilian Recruiter",
    "81dec453-c9d7-42cb-980a-c82f82bede76",
    "Joshua Cairos",
    crate::card::CardRules::unsupported(),
);

// BLB 150 — Roughshod Duo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROUGHSHOD_DUO: CardRecord = CardRecord::new(
    "Roughshod Duo",
    "78cdcfb9-a247-4c2d-a098-5b57570f8cd5",
    "Michal Ivan",
    crate::card::CardRules::unsupported(),
);

// BLB 151 — Sazacap's Brew
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAZACAP_S_BREW: CardRecord = CardRecord::new(
    "Sazacap's Brew",
    "6d963080-b3ec-467d-82f7-39db6ecd6bbc",
    "Sam Guay",
    crate::card::CardRules::unsupported(),
);

// BLB 152 — Season of the Bold
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEASON_OF_THE_BOLD: CardRecord = CardRecord::new(
    "Season of the Bold",
    "84352565-558b-4f9b-a411-532147806a78",
    "Eli Minaya",
    crate::card::CardRules::unsupported(),
);

// BLB 153 — Steampath Charger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STEAMPATH_CHARGER: CardRecord = CardRecord::new(
    "Steampath Charger",
    "03bf1296-e347-4070-8c6f-5c362c2f9364",
    "Ilse Gort",
    crate::card::CardRules::unsupported(),
);

// BLB 154 — Stormsplitter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STORMSPLITTER: CardRecord = CardRecord::new(
    "Stormsplitter",
    "56f214d3-6b93-40db-a693-55e491c8a283",
    "Lius Lasahido",
    crate::card::CardRules::unsupported(),
);

// BLB 155 — Sunspine Lynx
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUNSPINE_LYNX: CardRecord = CardRecord::new(
    "Sunspine Lynx",
    "8995ceaf-b7e0-423c-8f3e-25212d522502",
    "Martin Wittfooth",
    crate::card::CardRules::unsupported(),
);

// BLB 156 — Take Out the Trash
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TAKE_OUT_THE_TRASH: CardRecord = CardRecord::new(
    "Take Out the Trash",
    "7a1c6f00-af4c-4d35-b682-6c0e759df9a5",
    "Fiona Hsieh",
    crate::card::CardRules::unsupported(),
);

// BLB 157 — Teapot Slinger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEAPOT_SLINGER: CardRecord = CardRecord::new(
    "Teapot Slinger",
    "30506844-349f-4b68-8cc1-d028c1611cc7",
    "Wisnu Tan",
    crate::card::CardRules::unsupported(),
);

// BLB 158 — Valley Flamecaller
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VALLEY_FLAMECALLER: CardRecord = CardRecord::new(
    "Valley Flamecaller",
    "a0812db4-b7d1-4cf7-aaa5-9c0e784079a1",
    "Justin Gerard",
    crate::card::CardRules::unsupported(),
);

// BLB 159 — Valley Rally
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VALLEY_RALLY: CardRecord = CardRecord::new(
    "Valley Rally",
    "b6178258-1ad6-4122-a56f-6eb7d0611e84",
    "Sidharth Chaturvedi",
    crate::card::CardRules::unsupported(),
);

// BLB 160 — War Squeak
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WAR_SQUEAK: CardRecord = CardRecord::new(
    "War Squeak",
    "105964a7-88b7-4340-aa66-e908189a3638",
    "Zoltan Boros",
    crate::card::CardRules::unsupported(),
);

// BLB 161 — Whiskerquill Scribe
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WHISKERQUILL_SCRIBE: CardRecord = CardRecord::new(
    "Whiskerquill Scribe",
    "da653996-9bd4-40bd-afb4-48c7e070a269",
    "Matt Stewart",
    crate::card::CardRules::unsupported(),
);

// BLB 162 — Wildfire Howl
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WILDFIRE_HOWL: CardRecord = CardRecord::new(
    "Wildfire Howl",
    "7392d397-9836-4df2-944d-c930c9566811",
    "Manuel Castañón",
    crate::card::CardRules::unsupported(),
);

// BLB 163 — Bakersbane Duo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BAKERSBANE_DUO: CardRecord = CardRecord::new(
    "Bakersbane Duo",
    "5309354f-1ff4-4fa9-9141-01ea2f7588ab",
    "Raluca Marinescu",
    crate::card::CardRules::unsupported(),
);

// BLB 164 — Bark-Knuckle Boxer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARK_KNUCKLE_BOXER: CardRecord = CardRecord::new(
    "Bark-Knuckle Boxer",
    "582637a9-6aa0-4824-bed7-d5fc91bda35e",
    "Filip Burburan",
    crate::card::CardRules::unsupported(),
);

// BLB 165 — Brambleguard Veteran
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRAMBLEGUARD_VETERAN: CardRecord = CardRecord::new(
    "Brambleguard Veteran",
    "bac9f6f8-6797-4580-9fc4-9a825872e017",
    "Jakob Eirich",
    crate::card::CardRules::unsupported(),
);

// BLB 166 — Bushy Bodyguard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BUSHY_BODYGUARD: CardRecord = CardRecord::new(
    "Bushy Bodyguard",
    "0de60cf7-fa82-4b6f-9f88-6590fba5c863",
    "Andrea Piparo",
    crate::card::CardRules::unsupported(),
);

// BLB 167 — Cache Grab
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CACHE_GRAB: CardRecord = CardRecord::new(
    "Cache Grab",
    "dfd977dc-a7c3-4d0a-aca7-b25bd154e963",
    "Loïc Canavaggia",
    crate::card::CardRules::unsupported(),
);

// BLB 168 — Clifftop Lookout
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLIFFTOP_LOOKOUT: CardRecord = CardRecord::new(
    "Clifftop Lookout",
    "662d3bcc-65f3-4c69-8ea1-446870a1193d",
    "John Thacker",
    crate::card::CardRules::unsupported(),
);

// BLB 169 — Curious Forager
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CURIOUS_FORAGER: CardRecord = CardRecord::new(
    "Curious Forager",
    "64653b4a-e139-45f9-a915-ab49afb6b795",
    "Mariah Tekulve",
    crate::card::CardRules::unsupported(),
);

// BLB 170 — Druid of the Spade
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRUID_OF_THE_SPADE: CardRecord = CardRecord::new(
    "Druid of the Spade",
    "6b485cf7-bad0-4824-9ba7-cb112ce4769f",
    "Andrey Kuzinskiy",
    crate::card::CardRules::unsupported(),
);

// BLB 171 — Fecund Greenshell
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FECUND_GREENSHELL: CardRecord = CardRecord::new(
    "Fecund Greenshell",
    "80b3e815-0e2e-4325-b1d5-5531b7b92da6",
    "Kisung Koh",
    crate::card::CardRules::unsupported(),
);

// BLB 172 — For the Common Good
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FOR_THE_COMMON_GOOD: CardRecord = CardRecord::new(
    "For the Common Good",
    "3ec72a27-b622-47d7-bdf3-970ccaef0d2a",
    "Serena Malyon",
    crate::card::CardRules::unsupported(),
);

// BLB 173 — Galewind Moose
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GALEWIND_MOOSE: CardRecord = CardRecord::new(
    "Galewind Moose",
    "58706bd8-558a-43b9-9f1e-c1ff0044203b",
    "Valera Lutfullina",
    crate::card::CardRules::unsupported(),
);

// BLB 174 — Hazardroot Herbalist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HAZARDROOT_HERBALIST: CardRecord = CardRecord::new(
    "Hazardroot Herbalist",
    "e2882982-b3a3-4762-a550-6b82db1038e8",
    "Josiah \"Jo\" Cameron",
    crate::card::CardRules::unsupported(),
);

// BLB 175 — Heaped Harvest
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEAPED_HARVEST: CardRecord = CardRecord::new(
    "Heaped Harvest",
    "3b5349db-0e0a-4b15-886e-0db403ef49cb",
    "Daniel Ljunggren",
    crate::card::CardRules::unsupported(),
);

// BLB 176 — High Stride
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HIGH_STRIDE: CardRecord = CardRecord::new(
    "High Stride",
    "09c8cf4b-8e65-4a1c-b458-28b5ab56b390",
    "Dan Murayama Scott",
    crate::card::CardRules::unsupported(),
);

// BLB 177 — Hivespine Wolverine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HIVESPINE_WOLVERINE: CardRecord = CardRecord::new(
    "Hivespine Wolverine",
    "821970a3-a291-4fe9-bb13-dfc54f9c3caf",
    "Lars Grant-West",
    crate::card::CardRules::unsupported(),
);

// BLB 178 — Honored Dreyleader
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HONORED_DREYLEADER: CardRecord = CardRecord::new(
    "Honored Dreyleader",
    "bc5ee537-52e1-474a-9326-dfacc2a758ab",
    "Aurore Folny",
    crate::card::CardRules::unsupported(),
);

// BLB 179 — Hunter's Talent
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HUNTER_S_TALENT: CardRecord = CardRecord::new(
    "Hunter's Talent",
    "e9a31863-9649-4a4f-99e4-c93729938bd7",
    "Kisung Koh",
    crate::card::CardRules::unsupported(),
);

// BLB 180 — Innkeeper's Talent
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INNKEEPER_S_TALENT: CardRecord = CardRecord::new(
    "Innkeeper's Talent",
    "941b0afc-0e8f-45f2-ae7f-07595e164611",
    "Alix Branwyn",
    crate::card::CardRules::unsupported(),
);

// BLB 181 — Keen-Eyed Curator (alternate printing)
const KEEN_EYED_CURATOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KEEN_EYED_CURATOR,
    1,
    "8cf33d80-0704-4dc4-8e8d-1dcbcbc35add",
    "PINDURSKI",
);

// BLB 182 — Longstalk Brawl
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LONGSTALK_BRAWL: CardRecord = CardRecord::new(
    "Longstalk Brawl",
    "c7ef748c-b5e5-4e7d-bf2e-d3e6c08edb42",
    "Serena Malyon",
    crate::card::CardRules::unsupported(),
);

// BLB 183 — Lumra, Bellow of the Woods
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LUMRA_BELLOW_OF_THE_WOODS: CardRecord = CardRecord::new(
    "Lumra, Bellow of the Woods",
    "ae4f3aaf-3960-48cd-b34b-32e4ae5ae088",
    "Matt Stewart",
    crate::card::CardRules::unsupported(),
);

// BLB 184 — Mistbreath Elder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISTBREATH_ELDER: CardRecord = CardRecord::new(
    "Mistbreath Elder",
    "e5246540-5a84-41d8-9e30-8e7a6c0e84e1",
    "Jason Kang",
    crate::card::CardRules::unsupported(),
);

// BLB 185 — Overprotect
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OVERPROTECT: CardRecord = CardRecord::new(
    "Overprotect",
    "079e979f-b618-4625-989c-e0ea5b61ed8a",
    "Pavel Kolomeyets",
    crate::card::CardRules::unsupported(),
);

// BLB 186 — Pawpatch Formation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PAWPATCH_FORMATION: CardRecord = CardRecord::new(
    "Pawpatch Formation",
    "b82c20ad-0f69-4822-ae76-770832cccdf7",
    "Julia Griffin",
    crate::card::CardRules::unsupported(),
);

// BLB 187 — Pawpatch Recruit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PAWPATCH_RECRUIT: CardRecord = CardRecord::new(
    "Pawpatch Recruit",
    "7d4d88ba-0ee4-4f66-995b-2e50614f50ee",
    "Johan Grenier",
    crate::card::CardRules::unsupported(),
);

// BLB 188 — Peerless Recycling
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PEERLESS_RECYCLING: CardRecord = CardRecord::new(
    "Peerless Recycling",
    "5f72466c-505b-4371-9366-0fde525a37e6",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// BLB 189 — Polliwallop
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static POLLIWALLOP: CardRecord = CardRecord::new(
    "Polliwallop",
    "6bc4963c-d90b-4588-bdb7-85956e42a623",
    "Martin Wittfooth",
    crate::card::CardRules::unsupported(),
);

// BLB 190 — Rust-Shield Rampager
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUST_SHIELD_RAMPAGER: CardRecord = CardRecord::new(
    "Rust-Shield Rampager",
    "c96b01f5-83de-4237-a68d-f946c53e31a6",
    "Ralph Horsley",
    crate::card::CardRules::unsupported(),
);

// BLB 191 — Scrapshooter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCRAPSHOOTER: CardRecord = CardRecord::new(
    "Scrapshooter",
    "c42ab407-e72d-4c48-9a9e-2055b5e71c69",
    "Chris Rahn",
    crate::card::CardRules::unsupported(),
);

// BLB 192 — Season of Gathering
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEASON_OF_GATHERING: CardRecord = CardRecord::new(
    "Season of Gathering",
    "71dd3c27-e0d5-434e-a0f3-4a95245e21c2",
    "A. M. Sartor",
    crate::card::CardRules::unsupported(),
);

// BLB 193 — Stickytongue Sentinel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STICKYTONGUE_SENTINEL: CardRecord = CardRecord::new(
    "Stickytongue Sentinel",
    "b5fa9651-b217-4f93-9c46-9bdb11feedcb",
    "Lorenzo Mastroianni",
    crate::card::CardRules::unsupported(),
);

// BLB 194 — Stocking the Pantry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STOCKING_THE_PANTRY: CardRecord = CardRecord::new(
    "Stocking the Pantry",
    "50e95c7b-f0b2-4276-8c5e-4191b7ba35d1",
    "Gina Matarazzo",
    crate::card::CardRules::unsupported(),
);

// BLB 195 — Sunshower Druid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUNSHOWER_DRUID: CardRecord = CardRecord::new(
    "Sunshower Druid",
    "7740abc5-54e1-478d-966e-0fa64e727995",
    "Tuan Duong Chu",
    crate::card::CardRules::unsupported(),
);

// BLB 196 — Tender Wildguide
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TENDER_WILDGUIDE: CardRecord = CardRecord::new(
    "Tender Wildguide",
    "6b8bfa91-adb0-4596-8c16-d8bb64fdb26d",
    "Jakob Eirich",
    crate::card::CardRules::unsupported(),
);

// BLB 197 — Thornvault Forager
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THORNVAULT_FORAGER: CardRecord = CardRecord::new(
    "Thornvault Forager",
    "8c2d6b02-a453-40f9-992a-5c5542987cfb",
    "Mark Behm",
    crate::card::CardRules::unsupported(),
);

// BLB 198 — Three Tree Rootweaver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THREE_TREE_ROOTWEAVER: CardRecord = CardRecord::new(
    "Three Tree Rootweaver",
    "d1ab6e14-26e0-4174-b5c6-bc0f5c26b177",
    "Chris Seaman",
    crate::card::CardRules::unsupported(),
);

// BLB 199 — Three Tree Scribe
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THREE_TREE_SCRIBE: CardRecord = CardRecord::new(
    "Three Tree Scribe",
    "ea2ca1b3-4c1a-4be5-b321-f57db5ff0528",
    "Caio Monteiro",
    crate::card::CardRules::unsupported(),
);

// BLB 200 — Treeguard Duo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TREEGUARD_DUO: CardRecord = CardRecord::new(
    "Treeguard Duo",
    "89c8456e-c971-42b7-abf3-ff5ae1320abe",
    "Mila Pesic",
    crate::card::CardRules::unsupported(),
);

// BLB 201 — Treetop Sentries
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TREETOP_SENTRIES: CardRecord = CardRecord::new(
    "Treetop Sentries",
    "e16d4d6e-1fe5-4ff6-9877-8c849a24f5e0",
    "Iris Compiet",
    crate::card::CardRules::unsupported(),
);

// BLB 202 — Valley Mightcaller
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VALLEY_MIGHTCALLER: CardRecord = CardRecord::new(
    "Valley Mightcaller",
    "7256451f-0122-452a-88e8-0fb0f6bea3f3",
    "Matt Stewart",
    crate::card::CardRules::unsupported(),
);

// BLB 203 — Wear Down
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WEAR_DOWN: CardRecord = CardRecord::new(
    "Wear Down",
    "fded2b83-3b7d-4c8c-83c4-0624a1069628",
    "Iris Compiet",
    crate::card::CardRules::unsupported(),
);

// BLB 204 — Alania, Divergent Storm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALANIA_DIVERGENT_STORM: CardRecord = CardRecord::new(
    "Alania, Divergent Storm",
    "436d6a84-4cea-4ca7-94aa-9d08280652af",
    "Joshua Raphael",
    crate::card::CardRules::unsupported(),
);

// BLB 205 — Baylen, the Haymaker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BAYLEN_THE_HAYMAKER: CardRecord = CardRecord::new(
    "Baylen, the Haymaker",
    "00e93be2-e06b-4774-8ba5-ccf82a6da1d8",
    "Ryan Pancoast",
    crate::card::CardRules::unsupported(),
);

// BLB 206 — Burrowguard Mentor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BURROWGUARD_MENTOR: CardRecord = CardRecord::new(
    "Burrowguard Mentor",
    "87138ace-3594-499e-bad5-ec76148613ea",
    "Dmitry Burmak",
    crate::card::CardRules::unsupported(),
);

// BLB 207 — Camellia, the Seedmiser
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CAMELLIA_THE_SEEDMISER: CardRecord = CardRecord::new(
    "Camellia, the Seedmiser",
    "2c16eaec-924c-42f6-9fea-07edd7ed93b9",
    "Steve Prescott",
    crate::card::CardRules::unsupported(),
);

// BLB 208 — Cindering Cutthroat
// Audit: unsupported — Needs an "an opponent lost life this turn" replacement condition. ReplacementConditionDef offers only OpponentWasDealtDamageThisTurn, which is bloodthirst's damage reading: life lost to a payment, a drain, or an effect that says "loses life" would not count, so the counter would be missing in cases the printed card grants it.
pub(in crate::card::sets) static CINDERING_CUTTHROAT: CardRecord = CardRecord::new(
    "Cindering Cutthroat",
    "b2ea10dd-21ea-4622-be27-79d03a802b85",
    "Wayne Reynolds",
    crate::card::CardRules::unsupported(),
);

// BLB 209 — Clement, the Worrywort
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLEMENT_THE_WORRYWORT: CardRecord = CardRecord::new(
    "Clement, the Worrywort",
    "7028130c-c91d-4bf7-b0b0-450f71107d7a",
    "Ekaterina Burmak",
    crate::card::CardRules::unsupported(),
);

// BLB 210 — Corpseberry Cultivator
pub(in crate::card::sets) static CORPSEBERRY_CULTIVATOR: CardRecord = CardRecord::new(
    "Corpseberry Cultivator",
    "c911a759-ed7b-452b-88a3-663478357610",
    "Izzy",
    CardRules::new_creature(mana_cost!("{1}{B/G}{B/G}"), &["Squirrel", "Warlock"], 2, 3)
        .with_abilities(&[
            AbilityDef::triggered(
                "At the beginning of combat on your turn, you may forage. (Exile three cards \
                 from your graveyard or sacrifice a Food.)",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::BeginningOfCombat,
                    player: PlayerRelation::You,
                },
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    effect: &forage().as_effect(),
                },
            ),
            AbilityDef::triggered(
                "Whenever you forage, put a +1/+1 counter on this creature.",
                TriggerEventDef::MechanicPerformed {
                    mechanic: FORAGE,
                    player: PlayerRelation::You,
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// BLB 211 — Dreamdew Entrancer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DREAMDEW_ENTRANCER: CardRecord = CardRecord::new(
    "Dreamdew Entrancer",
    "26bd6b0d-8606-4a37-8be3-a852f1a8e99c",
    "Zoltan Boros",
    crate::card::CardRules::unsupported(),
);

// BLB 212 — Finneas, Ace Archer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FINNEAS_ACE_ARCHER: CardRecord = CardRecord::new(
    "Finneas, Ace Archer",
    "0dee197d-c313-4364-b52c-f83d5f579bc3",
    "Victor Adame Minguez",
    crate::card::CardRules::unsupported(),
);

// BLB 213 — Fireglass Mentor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FIREGLASS_MENTOR: CardRecord = CardRecord::new(
    "Fireglass Mentor",
    "b78fbaa3-c580-4290-9c28-b74169aab2fc",
    "Henry Peters",
    crate::card::CardRules::unsupported(),
);

// BLB 214 — Gev, Scaled Scorch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GEV_SCALED_SCORCH: CardRecord = CardRecord::new(
    "Gev, Scaled Scorch",
    "131ea976-289e-4f32-896d-27bbfd423ba9",
    "Mark Zug",
    crate::card::CardRules::unsupported(),
);

// BLB 215 — Glarb, Calamity's Augur
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLARB_CALAMITY_S_AUGUR: CardRecord = CardRecord::new(
    "Glarb, Calamity's Augur",
    "ffc70b2d-5a3a-49ea-97db-175a62248302",
    "Bram Sels",
    crate::card::CardRules::unsupported(),
);

// BLB 216 — Head of the Homestead
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEAD_OF_THE_HOMESTEAD: CardRecord = CardRecord::new(
    "Head of the Homestead",
    "2fc20157-edd3-484d-8864-925c071c0551",
    "Omar Rayyan",
    crate::card::CardRules::unsupported(),
);

// BLB 217 — Helga, Skittish Seer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HELGA_SKITTISH_SEER: CardRecord = CardRecord::new(
    "Helga, Skittish Seer",
    "40339715-22d0-4f99-822b-a00d9824f27a",
    "Andrea Piparo",
    crate::card::CardRules::unsupported(),
);

// BLB 218 — Hugs, Grisly Guardian
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HUGS_GRISLY_GUARDIAN: CardRecord = CardRecord::new(
    "Hugs, Grisly Guardian",
    "f09d7f4a-c947-4389-befa-1d547d0d1237",
    "Steve Prescott",
    crate::card::CardRules::unsupported(),
);

// BLB 219 — The Infamous Cruelclaw
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_INFAMOUS_CRUELCLAW: CardRecord = CardRecord::new(
    "The Infamous Cruelclaw",
    "dc6c9196-6d28-4cc2-9748-60e9632a502b",
    "Christina Kraus",
    crate::card::CardRules::unsupported(),
);

// BLB 220 — Junkblade Bruiser
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JUNKBLADE_BRUISER: CardRecord = CardRecord::new(
    "Junkblade Bruiser",
    "918fd89b-5ab7-4ae2-920c-faca5e9da7b9",
    "Omar Rayyan",
    crate::card::CardRules::unsupported(),
);

// BLB 221 — Kastral, the Windcrested
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KASTRAL_THE_WINDCRESTED: CardRecord = CardRecord::new(
    "Kastral, the Windcrested",
    "ebf68793-22a2-4a59-9d37-6791584edca1",
    "Jesper Ejsing",
    crate::card::CardRules::unsupported(),
);

// BLB 222 — Lilysplash Mentor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LILYSPLASH_MENTOR: CardRecord = CardRecord::new(
    "Lilysplash Mentor",
    "64de7b1f-a03e-4407-91f1-e108a2f26735",
    "Aldo Domínguez",
    crate::card::CardRules::unsupported(),
);

// BLB 223 — Lunar Convocation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LUNAR_CONVOCATION: CardRecord = CardRecord::new(
    "Lunar Convocation",
    "a9ee50d4-c878-457b-964d-29c039ce9852",
    "Pavel Kolomeyets",
    crate::card::CardRules::unsupported(),
);

// BLB 224 — Mabel, Heir to Cragflame
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MABEL_HEIR_TO_CRAGFLAME: CardRecord = CardRecord::new(
    "Mabel, Heir to Cragflame",
    "be6627fd-729d-44f2-b6bf-5299f49d1e3d",
    "Aurore Folny",
    crate::card::CardRules::unsupported(),
);

// BLB 225 — Mind Drill Assailant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIND_DRILL_ASSAILANT: CardRecord = CardRecord::new(
    "Mind Drill Assailant",
    "507ba708-ca9b-453e-b4c2-23b6650eb5a8",
    "Tuan Duong Chu",
    crate::card::CardRules::unsupported(),
);

// BLB 226 — Moonrise Cleric
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOONRISE_CLERIC: CardRecord = CardRecord::new(
    "Moonrise Cleric",
    "35f2a71f-31e8-4b51-9dd4-51a5336b3b86",
    "Simon Dominic",
    crate::card::CardRules::unsupported(),
);

// BLB 227 — Muerra, Trash Tactician
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MUERRA_TRASH_TACTICIAN: CardRecord = CardRecord::new(
    "Muerra, Trash Tactician",
    "b40e4658-fd68-46d0-9a89-25570a023d19",
    "Volkan Baǵa",
    crate::card::CardRules::unsupported(),
);

// BLB 228 — Plumecreed Mentor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PLUMECREED_MENTOR: CardRecord = CardRecord::new(
    "Plumecreed Mentor",
    "b1aa988f-547e-449a-9f1a-296c01d68d96",
    "Henry Peters",
    crate::card::CardRules::unsupported(),
);

// BLB 229 — Pond Prophet
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static POND_PROPHET: CardRecord = CardRecord::new(
    "Pond Prophet",
    "fb959e74-61ea-453d-bb9f-ad0183c0e1b1",
    "Simon Dominic",
    crate::card::CardRules::unsupported(),
);

// BLB 230 — Ral, Crackling Wit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAL_CRACKLING_WIT: CardRecord = CardRecord::new(
    "Ral, Crackling Wit",
    "acfde780-899a-4c5b-a39b-f4a3ff129103",
    "Rudy Siswanto",
    crate::card::CardRules::unsupported(),
);

// BLB 231 — Seedglaive Mentor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEEDGLAIVE_MENTOR: CardRecord = CardRecord::new(
    "Seedglaive Mentor",
    "d21c3e41-0636-49a3-8c9c-384c5e5c9c3e",
    "Vincent Christiaens",
    crate::card::CardRules::unsupported(),
);

// BLB 232 — Seedpod Squire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEEDPOD_SQUIRE: CardRecord = CardRecord::new(
    "Seedpod Squire",
    "f3684577-51ce-490e-9b59-b19c733be466",
    "Christina Kraus",
    crate::card::CardRules::unsupported(),
);

// BLB 233 — Starseer Mentor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STARSEER_MENTOR: CardRecord = CardRecord::new(
    "Starseer Mentor",
    "6b2f6dc5-9fe8-49c1-b24c-1d99ce1da619",
    "Taras Susak",
    crate::card::CardRules::unsupported(),
);

// BLB 234 — Stormcatch Mentor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STORMCATCH_MENTOR: CardRecord = CardRecord::new(
    "Stormcatch Mentor",
    "99754055-6d67-4fde-aff3-41f6af6ea764",
    "Manuel Castañón",
    crate::card::CardRules::unsupported(),
);

// BLB 235 — Tempest Angler
pub(in crate::card::sets) static TEMPEST_ANGLER: CardRecord = CardRecord::new(
    "Tempest Angler",
    "850daae4-f0b7-4604-95e7-ad044ec165c3",
    "Raluca Marinescu",
    // Counters rather than prowess: what it grows it keeps, so a slow turn
    // of cheap spells leaves a threat rather than a one-turn swing.
    CardRules::new_creature(mana_cost!("{1}{U/R}{U/R}"), &["Otter", "Wizard"], 2, 2).with_ability(
        AbilityDef::triggered(
            "Whenever you cast a noncreature spell, put a +1/+1 counter on this creature.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::NoncreatureSpell,
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// BLB 236 — Tidecaller Mentor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TIDECALLER_MENTOR: CardRecord = CardRecord::new(
    "Tidecaller Mentor",
    "fa10ffac-7cc2-41ef-b8a0-9431923c0542",
    "Irina Nordsol",
    crate::card::CardRules::unsupported(),
);

// BLB 237 — Veteran Guardmouse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VETERAN_GUARDMOUSE: CardRecord = CardRecord::new(
    "Veteran Guardmouse",
    "3db43c46-b616-4ef8-80ed-0fab345ab3d0",
    "Campbell White",
    crate::card::CardRules::unsupported(),
);

// BLB 238 — Vinereap Mentor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VINEREAP_MENTOR: CardRecord = CardRecord::new(
    "Vinereap Mentor",
    "29b615ba-45c4-42a1-8525-1535f0b55300",
    "Valera Lutfullina",
    crate::card::CardRules::unsupported(),
);

// BLB 239 — Vren, the Relentless
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VREN_THE_RELENTLESS: CardRecord = CardRecord::new(
    "Vren, the Relentless",
    "6506277d-f031-4db5-9d16-bf2389094785",
    "Kevin Sidharta",
    crate::card::CardRules::unsupported(),
);

// BLB 240 — Wandertale Mentor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WANDERTALE_MENTOR: CardRecord = CardRecord::new(
    "Wandertale Mentor",
    "8c399a55-d02e-41ed-b827-8784b738c118",
    "Jakub Kasper",
    crate::card::CardRules::unsupported(),
);

// BLB 241 — Ygra, Eater of All
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static YGRA_EATER_OF_ALL: CardRecord = CardRecord::new(
    "Ygra, Eater of All",
    "b9ac7673-eae8-4c4b-889e-5025213a6151",
    "Chris Rahn",
    crate::card::CardRules::unsupported(),
);

// BLB 242 — Zoraline, Cosmos Caller
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ZORALINE_COSMOS_CALLER: CardRecord = CardRecord::new(
    "Zoraline, Cosmos Caller",
    "b7f99fd5-5298-4b27-923d-9d31203c931a",
    "Justin Gerard",
    crate::card::CardRules::unsupported(),
);

// BLB 243 — Barkform Harvester
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARKFORM_HARVESTER: CardRecord = CardRecord::new(
    "Barkform Harvester",
    "f77049a6-0f22-415b-bc89-20bcb32accf6",
    "Zezhou Chen",
    crate::card::CardRules::unsupported(),
);

// BLB 244 — Bumbleflower's Sharepot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BUMBLEFLOWER_S_SHAREPOT: CardRecord = CardRecord::new(
    "Bumbleflower's Sharepot",
    "5f0affd5-5dcd-4dd1-a694-37a9aedf4084",
    "J.P. Targete",
    crate::card::CardRules::unsupported(),
);

// BLB 245 — Fountainport Bell
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FOUNTAINPORT_BELL: CardRecord = CardRecord::new(
    "Fountainport Bell",
    "a5c94bc0-a49d-451b-8e8d-64d46b8b8603",
    "Néstor Ossandón Leal",
    crate::card::CardRules::unsupported(),
);

// BLB 246 — Heirloom Epic
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEIRLOOM_EPIC: CardRecord = CardRecord::new(
    "Heirloom Epic",
    "7839ce48-0175-494a-ab89-9bdfb7a50cb1",
    "Fiona Hsieh",
    crate::card::CardRules::unsupported(),
);

// BLB 247 — Patchwork Banner
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PATCHWORK_BANNER: CardRecord = CardRecord::new(
    "Patchwork Banner",
    "a8a982c8-bc08-44ba-b3ed-9e4b124615d6",
    "Sarah Finnigan",
    crate::card::CardRules::unsupported(),
);

// BLB 248 — Short Bow
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHORT_BOW: CardRecord = CardRecord::new(
    "Short Bow",
    "51d8b72b-fa8f-48d3-bddc-d3ce9b8ba2ea",
    "Zara Alfonso",
    crate::card::CardRules::unsupported(),
);

// BLB 249 — Starforged Sword
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STARFORGED_SWORD: CardRecord = CardRecord::new(
    "Starforged Sword",
    "c23d8e96-b972-4c6c-b0c4-b6627621f048",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// BLB 250 — Tangle Tumbler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TANGLE_TUMBLER: CardRecord = CardRecord::new(
    "Tangle Tumbler",
    "258ef349-5042-4992-bae9-9f8f54b55db0",
    "Victor Adame Minguez",
    crate::card::CardRules::unsupported(),
);

// BLB 251 — Three Tree Mascot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THREE_TREE_MASCOT: CardRecord = CardRecord::new(
    "Three Tree Mascot",
    "aaced75b-6e07-457c-8ea2-f74d99710d15",
    "Gina Matarazzo",
    crate::card::CardRules::unsupported(),
);

// BLB 252 — Fabled Passage (reprint)
const FABLED_PASSAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_eld::FABLED_PASSAGE,
    "8809830f-d8e1-4603-9652-0ad8b00234e9",
    "Adam Paquette",
);

// BLB 253 — Fountainport
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FOUNTAINPORT: CardRecord = CardRecord::new(
    "Fountainport",
    "658cfcb7-81b7-48c6-9dd2-1663d06108cf",
    "Leon Tukker",
    crate::card::CardRules::unsupported(),
);

// BLB 254 — Hidden Grotto
pub(in crate::card::sets) static HIDDEN_GROTTO: CardRecord = CardRecord::new(
    "Hidden Grotto",
    "4ba8f2e7-8357-4862-97dc-1942d066023a",
    "Fiona Hsieh",
    // Untapped and colourless by default, so the fixing costs a mana rather
    // than a turn -- and the surveil pays for playing it over a basic.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_trigger(
            "When this land enters, surveil 1. (Look at the top card of your library. You may \
             put it into your graveyard.)",
            abilities::surveil(ValueDef::Constant(1)),
        ),
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_mana(
            "{1}, {T}: Add one mana of any color.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// BLB 255 — Lilypad Village
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LILYPAD_VILLAGE: CardRecord = CardRecord::new(
    "Lilypad Village",
    "7e95a7cc-ed77-4ca4-80db-61c0fc68bf50",
    "Alexander Forssberg",
    crate::card::CardRules::unsupported(),
);

// BLB 256 — Lupinflower Village
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LUPINFLOWER_VILLAGE: CardRecord = CardRecord::new(
    "Lupinflower Village",
    "8ab9d56f-9178-4ec9-a5f6-b934f50d8d9d",
    "Alayna Danner",
    crate::card::CardRules::unsupported(),
);

// BLB 257 — Mudflat Village
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MUDFLAT_VILLAGE: CardRecord = CardRecord::new(
    "Mudflat Village",
    "53ec4ad3-9cf0-4f1b-a9db-d63feee594ab",
    "Samuele Bandini",
    crate::card::CardRules::unsupported(),
);

// BLB 258 — Oakhollow Village
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OAKHOLLOW_VILLAGE: CardRecord = CardRecord::new(
    "Oakhollow Village",
    "0d49b016-b02b-459f-85e9-c04f6bdcb94e",
    "Julian Kok Joon Wen",
    crate::card::CardRules::unsupported(),
);

// BLB 259 — Rockface Village
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROCKFACE_VILLAGE: CardRecord = CardRecord::new(
    "Rockface Village",
    "62799d24-39a6-4e66-8ac3-7cafa99e6e6d",
    "Thomas Stoop",
    crate::card::CardRules::unsupported(),
);

// BLB 260 — Three Tree City
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THREE_TREE_CITY: CardRecord = CardRecord::new(
    "Three Tree City",
    "56f88a48-cced-4a9d-8c19-e4f105f0d8a2",
    "Grady Frederick",
    crate::card::CardRules::unsupported(),
);

// BLB 261 — Uncharted Haven (reprint)
const UNCHARTED_HAVEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_neo::UNCHARTED_HAVEN,
    "68b90f54-d629-4126-82cc-13b51d6c1c3e",
    "Adam Paquette",
);

// BLB 262 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAINS,
    "3663ff0c-d02f-49ac-bb88-6f0dbd684337",
    "Carlos Palma Cruchaga",
);

// BLB 263 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    1,
    "ab53b300-38b2-436c-834c-b0162dd3b757",
    "Carlos Palma Cruchaga",
);

// BLB 264 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    2,
    "4f04188d-9a76-495d-b3a7-ee44810cf671",
    "Carlos Palma Cruchaga",
);

// BLB 265 — Plains (alternate printing)
const PLAINS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    3,
    "11a94c04-b6d3-4118-b252-5146eedae4e8",
    "Carlos Palma Cruchaga",
);

// BLB 266 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND,
    "106a5332-0104-4ec9-9e1f-806381ae4cad",
    "Rob Rey",
);

// BLB 267 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    1,
    "f7b92adb-a384-4e03-8ba7-1e41b9fb2516",
    "Rob Rey",
);

// BLB 268 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    2,
    "408eb23f-13fa-4f98-b316-18e3057f9136",
    "Rob Rey",
);

// BLB 269 — Island (alternate printing)
const ISLAND_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    3,
    "2a9db98e-9f1d-4139-95c5-49f9d17a2107",
    "Rob Rey",
);

// BLB 270 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWAMP,
    "faa54bef-c90e-4e9b-b94b-942ea829e0d2",
    "John Thacker",
);

// BLB 271 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    1,
    "dcdbc78a-7854-4a66-911f-a40b33e25f39",
    "John Thacker",
);

// BLB 272 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    2,
    "0223cf7f-4c6e-487f-babb-a740a15e1f0c",
    "John Thacker",
);

// BLB 273 — Swamp (alternate printing)
const SWAMP_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    3,
    "4650ebaf-8e9f-431c-998d-e0f426b24d41",
    "John Thacker",
);

// BLB 274 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MOUNTAIN,
    "c1bba8fb-d763-4efa-8db1-e5e81994b5f9",
    "Andrew Theophilopoulos",
);

// BLB 275 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    1,
    "02244055-cbeb-4074-9482-42ec20721312",
    "Andrew Theophilopoulos",
);

// BLB 276 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    2,
    "0b384d24-8771-4860-8fc1-1b74217f1c4c",
    "Andrew Theophilopoulos",
);

// BLB 277 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    3,
    "8a38bea4-2d1b-442b-aad5-ab1a0ae67aac",
    "Andrew Theophilopoulos",
);

// BLB 278 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOREST,
    "f791876a-f3fb-45b8-90a9-af846d8b8f74",
    "David Robert Hovey",
);

// BLB 279 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    1,
    "3a7a2a54-47e1-4694-b4f6-b7d659bb2b1f",
    "David Robert Hovey",
);

// BLB 280 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    2,
    "0000419b-0bba-4488-8f7a-6194544ce91e",
    "David Robert Hovey",
);

// BLB 281 — Forest (alternate printing)
const FOREST_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    3,
    "208014f4-eed8-4856-a9d9-dbac10c4aa6a",
    "David Robert Hovey",
);

// BLB 282 — Season of the Burrow (alternate printing)
const SEASON_OF_THE_BURROW_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SEASON_OF_THE_BURROW,
    1,
    "ba10fbc2-193a-4516-a8dd-8b5f8b33a9de",
    "Edgar Sánchez Hidalgo",
);

// BLB 283 — Season of Weaving (alternate printing)
const SEASON_OF_WEAVING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SEASON_OF_WEAVING,
    1,
    "61da9522-3844-4753-8122-d11ae2780a4c",
    "Andrea Sipl",
);

// BLB 284 — Season of Loss (alternate printing)
const SEASON_OF_LOSS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SEASON_OF_LOSS,
    1,
    "2a428d58-b376-489e-8224-17a41ef6e81b",
    "Anna Pavleeva",
);

// BLB 285 — Season of the Bold (alternate printing)
const SEASON_OF_THE_BOLD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SEASON_OF_THE_BOLD,
    1,
    "e9d88763-35bd-472a-810e-02c795d0c873",
    "Yeong-Hao Han",
);

// BLB 286 — Season of Gathering (alternate printing)
const SEASON_OF_GATHERING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SEASON_OF_GATHERING,
    1,
    "6e99fa55-2594-434b-9396-95b8f661bd0d",
    "Irina Nordsol",
);

// BLB 287 — Beza, the Bounding Spring (alternate printing)
const BEZA_THE_BOUNDING_SPRING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BEZA_THE_BOUNDING_SPRING,
    1,
    "0fc98b72-d268-4ce5-93b4-57c812a24eff",
    "Yeong-Hao Han",
);

// BLB 288 — Eluge, the Shoreless Sea (alternate printing)
const ELUGE_THE_SHORELESS_SEA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ELUGE_THE_SHORELESS_SEA,
    1,
    "df9da428-3af7-4027-b3f0-9138d953c37b",
    "Antonio José Manzanedo",
);

// BLB 289 — Maha, Its Feathers Night (alternate printing)
const MAHA_ITS_FEATHERS_NIGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MAHA_ITS_FEATHERS_NIGHT,
    1,
    "bfd53428-56c3-4c99-828d-665e3c2d15a8",
    "Jeff Carpenter",
);

// BLB 290 — Rottenmouth Viper (alternate printing)
const ROTTENMOUTH_VIPER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ROTTENMOUTH_VIPER,
    1,
    "7bad7555-3e5d-4096-a671-184630d38113",
    "Filip Burburan",
);

// BLB 291 — Dragonhawk, Fate's Tempest (alternate printing)
const DRAGONHAWK_FATE_S_TEMPEST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DRAGONHAWK_FATE_S_TEMPEST,
    1,
    "de4ef78d-ba5e-415e-9684-816464c5611a",
    "Antonio José Manzanedo",
);

// BLB 292 — Sunspine Lynx (alternate printing)
const SUNSPINE_LYNX_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUNSPINE_LYNX,
    1,
    "f1c414b4-3374-44e4-98b4-03193e701eb1",
    "Crystal Sully",
);

// BLB 293 — Lumra, Bellow of the Woods (alternate printing)
const LUMRA_BELLOW_OF_THE_WOODS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LUMRA_BELLOW_OF_THE_WOODS,
    1,
    "3d99587a-7fb1-41fc-b7ea-1178f9625081",
    "Allen Douglas",
);

// BLB 294 — Ygra, Eater of All (alternate printing)
const YGRA_EATER_OF_ALL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &YGRA_EATER_OF_ALL,
    1,
    "97e0d6ca-eed9-4b57-a34a-0105c41b20b9",
    "Ilse Gort",
);

// BLB 295 — Dawn's Truce (alternate printing)
const DAWN_S_TRUCE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DAWN_S_TRUCE,
    1,
    "0cce7aec-f9b0-461b-8245-5286b741409d",
    "Mariah Tekulve",
);

// BLB 296 — Jackdaw Savior (alternate printing)
const JACKDAW_SAVIOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &JACKDAW_SAVIOR,
    1,
    "4411204e-8387-4f1c-bfb3-1e3092d07937",
    "Serena Malyon",
);

// BLB 297 — Salvation Swan (alternate printing)
const SALVATION_SWAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SALVATION_SWAN,
    1,
    "f25e2bd6-c95b-422f-b6c2-d687c0745ecb",
    "Serena Malyon",
);

// BLB 298 — Starfall Invocation (alternate printing)
const STARFALL_INVOCATION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STARFALL_INVOCATION,
    1,
    "29fe9b09-383a-4cf6-9e2c-bd00499e5eb0",
    "Iain McCaig",
);

// BLB 299 — Valley Questcaller (alternate printing)
const VALLEY_QUESTCALLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VALLEY_QUESTCALLER,
    1,
    "d9f25130-678d-4338-8eb4-b20d2da5bc74",
    "Cory Godbey",
);

// BLB 300 — Warren Warleader (alternate printing)
const WARREN_WARLEADER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WARREN_WARLEADER,
    1,
    "48062f31-167a-4f16-9453-7c41f026ca96",
    "Iain McCaig",
);

// BLB 301 — Whiskervale Forerunner (alternate printing)
const WHISKERVALE_FORERUNNER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WHISKERVALE_FORERUNNER,
    1,
    "7ea69966-8961-4447-bd39-70d3d8d48ede",
    "Chuck Grieb",
);

// BLB 302 — Azure Beastbinder (alternate printing)
const AZURE_BEASTBINDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AZURE_BEASTBINDER,
    1,
    "f493ef3c-673b-4e70-b957-0ee7b6d38216",
    "David Petersen",
);

// BLB 303 — Dour Port-Mage (alternate printing)
const DOUR_PORT_MAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DOUR_PORT_MAGE,
    1,
    "9e407118-95b7-4e62-a4f0-dbd8b6b80830",
    "Iain McCaig",
);

// BLB 304 — Kitsa, Otterball Elite (alternate printing)
const KITSA_OTTERBALL_ELITE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KITSA_OTTERBALL_ELITE,
    1,
    "d0bbfdc9-8d68-4b4e-ac68-ac165cdc168d",
    "David Petersen",
);

// BLB 305 — Mockingbird (alternate printing)
const MOCKINGBIRD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MOCKINGBIRD,
    1,
    "5ed827d1-a374-48b4-8b65-abec9044de85",
    "Mariah Tekulve",
);

// BLB 306 — Portent of Calamity (alternate printing)
const PORTENT_OF_CALAMITY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PORTENT_OF_CALAMITY,
    1,
    "5fb6618b-276b-4ce1-873f-eace471fafde",
    "Justine Mara Andersen",
);

// BLB 307 — Thundertrap Trainer (alternate printing)
const THUNDERTRAP_TRAINER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THUNDERTRAP_TRAINER,
    1,
    "54ec3510-c168-4acc-aee8-65529d0f5ad7",
    "Iain McCaig",
);

// BLB 308 — Valley Floodcaller (alternate printing)
const VALLEY_FLOODCALLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VALLEY_FLOODCALLER,
    1,
    "1c744764-3b54-49a0-aa68-1fb22d8162a9",
    "Iris Compiet",
);

// BLB 309 — Coiling Rebirth (alternate printing)
const COILING_REBIRTH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &COILING_REBIRTH,
    1,
    "640725a5-8251-4f7a-b1e3-481519c85b5d",
    "ELK64",
);

// BLB 310 — Cruelclaw's Heist (alternate printing)
const CRUELCLAW_S_HEIST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CRUELCLAW_S_HEIST,
    1,
    "fa16ec21-3eb1-4f4c-a74a-84408fd1dce6",
    "Iris Compiet",
);

// BLB 311 — Darkstar Augur (alternate printing)
const DARKSTAR_AUGUR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DARKSTAR_AUGUR,
    1,
    "8e9abfd1-59ba-46f4-ad1d-cf0e125dd4b9",
    "Ellie Livingston",
);

// BLB 312 — Osteomancer Adept (alternate printing)
const OSTEOMANCER_ADEPT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OSTEOMANCER_ADEPT,
    1,
    "6ef4532d-63ea-4a70-86b3-58327b7e9706",
    "Justine Mara Andersen",
);

// BLB 313 — Valley Rotcaller (alternate printing)
const VALLEY_ROTCALLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VALLEY_ROTCALLER,
    1,
    "4278a127-5f3c-4948-88bd-2af4f6834a03",
    "A. M. Sartor",
);

// BLB 314 — Wick, the Whorled Mind (alternate printing)
const WICK_THE_WHORLED_MIND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WICK_THE_WHORLED_MIND,
    1,
    "f43a9d17-2b19-4e17-9b7e-667bf2285950",
    "Audrey Benjaminsen",
);

// BLB 315 — Emberheart Challenger (alternate printing)
const EMBERHEART_CHALLENGER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EMBERHEART_CHALLENGER,
    1,
    "8fde5980-ad2c-4ddf-8244-f9688a6225f2",
    "Cory Godbey",
);

// BLB 316 — Festival of Embers (alternate printing)
const FESTIVAL_OF_EMBERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FESTIVAL_OF_EMBERS,
    1,
    "02134775-9c86-4f1a-af2e-5c1f531038fe",
    "Cory Godbey",
);

// BLB 317 — Hired Claw (alternate printing)
const HIRED_CLAW_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HIRED_CLAW,
    1,
    "fda3d3ef-c858-4707-9c3f-be1b775eb8c1",
    "Ellie Livingston",
);

// BLB 318 — Manifold Mouse (alternate printing)
const MANIFOLD_MOUSE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MANIFOLD_MOUSE,
    1,
    "907a990c-6ee9-44e1-85f9-83b261507308",
    "Chuck Grieb",
);

// BLB 319 — Stormsplitter (alternate printing)
const STORMSPLITTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STORMSPLITTER,
    1,
    "9cb5235d-46a2-4b04-97fc-b8eb0f1ba14a",
    "Serena Malyon",
);

// BLB 320 — Valley Flamecaller (alternate printing)
const VALLEY_FLAMECALLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VALLEY_FLAMECALLER,
    1,
    "f266f6da-93b0-4036-b7df-be5eac4e1135",
    "Ellie Livingston",
);

// BLB 321 — For the Common Good (alternate printing)
const FOR_THE_COMMON_GOOD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FOR_THE_COMMON_GOOD,
    1,
    "60d1e35e-f5c8-422d-83d2-839ab7766f25",
    "Ellie Livingston",
);

// BLB 322 — Keen-Eyed Curator
pub(in crate::card::sets) static KEEN_EYED_CURATOR: CardRecord = CardRecord::new(
    "Keen-Eyed Curator",
    "004a67ce-60ef-4cc2-9f4d-f30e3029d80a",
    "Mariah Tekulve",
// Two mana for a 3/3 that answers a graveyard a card at a time, and
    // turns into a 7/7 trampler for having done it four kinds of times.
    CardRules::new_creature(mana_cost!("{G}{G}"), &["Raccoon", "Scout"], 3, 3)
        .with_abilities(&[
            // "As long as", so the 7/7 comes and goes with the pile rather than
            // being settled once.
            AbilityDef::static_ability(
                "As long as there are four or more card types among cards exiled with this creature, it \
                 gets +4/+4 and has trample.",
                EffectDef::IfCondition {
                    // Four card types among the cards he took, counted over the pile rather
                    // than over any zone: he keeps them, so a card that leaves exile stops
                    // counting and the rest still do.
                    condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                            left: ValueDef::CardTypesAmongObjects(&ObjectSetDef::LinkedExiles),
                            comparison: ComparisonDef::GreaterOrEqual,
                            right: ValueDef::Constant(4),
                        }),
                    then: &EffectDef::Sequence(&[
                        EffectDef::StaticApply {
                            recipient: EffectRecipientDef::Source,
                            effect: AppliedEffectDef::modify_power_toughness(
                                ValueDef::Constant(4),
                                ValueDef::Constant(4),
                            ),
                        },
                        EffectDef::StaticApply {
                            recipient: EffectRecipientDef::Source,
                            effect: AppliedEffectDef::add_ability(&abilities::trample()),
                        },
                    ]),
                },
            ),
            // Either graveyard: what he is played for is emptying theirs, and the
            // card types he needs come from wherever they are.
            AbilityDef::activated_with_targets(
                "{1}: Exile target card from a graveyard.",
                &[CostDef::Mana(mana_cost!("{1}"))],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Any,
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: None,
                    },
                )],
                EffectDef::ExileLinkedToSource {
                    until_source_leaves: false,
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    face_down: false,
                    then: None,
                },
            ),
        ]),
);

// BLB 323 — Mistbreath Elder (alternate printing)
const MISTBREATH_ELDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MISTBREATH_ELDER,
    1,
    "f2ed58e4-b612-483e-9c0e-f1fbfaa5950c",
    "Ellie Livingston",
);

// BLB 324 — Scrapshooter (alternate printing)
const SCRAPSHOOTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SCRAPSHOOTER,
    1,
    "9afd22ab-64e7-4b85-804d-faccd4452487",
    "Mariah Tekulve",
);

// BLB 325 — Tender Wildguide (alternate printing)
const TENDER_WILDGUIDE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TENDER_WILDGUIDE,
    1,
    "2dc164c8-62ca-4d59-ae1c-ef273fde9d10",
    "Ellie Livingston",
);

// BLB 326 — Valley Mightcaller (alternate printing)
const VALLEY_MIGHTCALLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VALLEY_MIGHTCALLER,
    1,
    "31330554-1824-436b-aee5-931a7b652ddf",
    "Justine Mara Andersen",
);

// BLB 327 — Alania, Divergent Storm (alternate printing)
const ALANIA_DIVERGENT_STORM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ALANIA_DIVERGENT_STORM,
    1,
    "f9dc7b8e-d381-49a0-a7f8-c8cd9acadc54",
    "Cory Godbey",
);

// BLB 328 — Camellia, the Seedmiser (alternate printing)
const CAMELLIA_THE_SEEDMISER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CAMELLIA_THE_SEEDMISER,
    1,
    "49ea9ca9-3dff-4d5d-92ec-3475234b4713",
    "David Petersen",
);

// BLB 329 — Clement, the Worrywort (alternate printing)
const CLEMENT_THE_WORRYWORT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CLEMENT_THE_WORRYWORT,
    1,
    "d1a68d51-cd4e-4ee3-abc7-01435085aa26",
    "Cory Godbey",
);

// BLB 330 — Finneas, Ace Archer (alternate printing)
const FINNEAS_ACE_ARCHER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FINNEAS_ACE_ARCHER,
    1,
    "e77374ac-3456-48a3-a412-4474547d437c",
    "Cory Godbey",
);

// BLB 331 — Glarb, Calamity's Augur (alternate printing)
const GLARB_CALAMITY_S_AUGUR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GLARB_CALAMITY_S_AUGUR,
    1,
    "b6fab12e-0f81-4728-9a35-4dc896ba3744",
    "Chuck Grieb",
);

// BLB 332 — Helga, Skittish Seer (alternate printing)
const HELGA_SKITTISH_SEER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HELGA_SKITTISH_SEER,
    1,
    "768ef947-85fc-4fb2-bda9-e0e446cd8886",
    "David Petersen",
);

// BLB 333 — Hugs, Grisly Guardian (alternate printing)
const HUGS_GRISLY_GUARDIAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HUGS_GRISLY_GUARDIAN,
    1,
    "94b2be42-467d-4210-b9bf-4d09ee504a22",
    "Chuck Lukacs",
);

// BLB 334 — The Infamous Cruelclaw (alternate printing)
const THE_INFAMOUS_CRUELCLAW_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_INFAMOUS_CRUELCLAW,
    1,
    "d7fc26a7-f47e-44d7-a42b-b5415fc86f8f",
    "Iain McCaig",
);

// BLB 335 — Kastral, the Windcrested (alternate printing)
const KASTRAL_THE_WINDCRESTED_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KASTRAL_THE_WINDCRESTED,
    1,
    "75a84790-665b-4a23-a00f-84d1009e3d28",
    "Chuck Grieb",
);

// BLB 336 — Mabel, Heir to Cragflame (alternate printing)
const MABEL_HEIR_TO_CRAGFLAME_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MABEL_HEIR_TO_CRAGFLAME,
    1,
    "1696453f-a678-47ff-b583-53c2130b7d49",
    "David Petersen",
);

// BLB 337 — Three Tree City (alternate printing)
const THREE_TREE_CITY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THREE_TREE_CITY,
    1,
    "d96b0f9e-fa3b-4a40-acea-b6ecc584a79f",
    "Andrew Mar",
);

// BLB 338 — Three Tree City (alternate printing)
const THREE_TREE_CITY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THREE_TREE_CITY,
    2,
    "06008bcb-ce6b-482c-9253-f5b9b5e0718e",
    "Andrew Mar",
);

// BLB 339 — Three Tree City (alternate printing)
const THREE_TREE_CITY_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &THREE_TREE_CITY,
    3,
    "8e1d8486-370a-49b6-9145-060814f38677",
    "Andrew Mar",
);

// BLB 340 — Three Tree City (alternate printing)
const THREE_TREE_CITY_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &THREE_TREE_CITY,
    4,
    "1cb16046-10bf-433a-acaf-7127c0954a1c",
    "Andrew Mar",
);

// BLB 341 — Ral, Crackling Wit (alternate printing)
const RAL_CRACKLING_WIT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RAL_CRACKLING_WIT,
    1,
    "8a74dbdd-5baa-4896-80d4-1a9743410d4c",
    "Scott M. Fischer",
);

// BLB 342 — Lumra, Bellow of the Woods (alternate printing)
const LUMRA_BELLOW_OF_THE_WOODS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &LUMRA_BELLOW_OF_THE_WOODS,
    2,
    "b43b3c33-aa44-4001-87ff-695bf04f51be",
    "Mitsuhiro Arita",
);

// BLB 343 — Lumra, Bellow of the Woods (alternate printing)
const LUMRA_BELLOW_OF_THE_WOODS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &LUMRA_BELLOW_OF_THE_WOODS,
    3,
    "8487cfce-2b73-4082-a1f2-dea263811516",
    "Mitsuhiro Arita",
);

// BLB 344 — Alania, Divergent Storm (alternate printing)
const ALANIA_DIVERGENT_STORM_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ALANIA_DIVERGENT_STORM,
    2,
    "34ace81d-823f-43d1-902e-345418cd8fd2",
    "Issei Murakami",
);

// BLB 345 — Baylen, the Haymaker (alternate printing)
const BAYLEN_THE_HAYMAKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BAYLEN_THE_HAYMAKER,
    1,
    "0b511524-e628-41d4-b0dc-d98961d4e9e1",
    "Kemonomichi",
);

// BLB 346 — Camellia, the Seedmiser (alternate printing)
const CAMELLIA_THE_SEEDMISER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &CAMELLIA_THE_SEEDMISER,
    2,
    "44f59ece-1ba5-422b-820d-e80c83f46290",
    "M I N A T O",
);

// BLB 347 — Clement, the Worrywort (alternate printing)
const CLEMENT_THE_WORRYWORT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &CLEMENT_THE_WORRYWORT,
    2,
    "16dfe689-83d4-4dcb-810c-d618a39c5ab9",
    "Issei Murakami",
);

// BLB 348 — Finneas, Ace Archer (alternate printing)
const FINNEAS_ACE_ARCHER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &FINNEAS_ACE_ARCHER,
    2,
    "f4d99aec-c114-4287-ab61-a34e5adab5ab",
    "O-G Osahune",
);

// BLB 349 — Gev, Scaled Scorch (alternate printing)
const GEV_SCALED_SCORCH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GEV_SCALED_SCORCH,
    1,
    "4ad70a0b-3845-4b92-9c15-985a3fe0e1f2",
    "Taro Yamazaki",
);

// BLB 350 — Kastral, the Windcrested (alternate printing)
const KASTRAL_THE_WINDCRESTED_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &KASTRAL_THE_WINDCRESTED,
    2,
    "2f6d51dc-ca49-417c-8b5a-fcb5cd669c6f",
    "Noki",
);

// BLB 351 — Mabel, Heir to Cragflame (alternate printing)
const MABEL_HEIR_TO_CRAGFLAME_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MABEL_HEIR_TO_CRAGFLAME,
    2,
    "cfbdaae3-acba-44e1-bdd6-9c7066143d33",
    "Airi Yoshihisa",
);

// BLB 352 — Muerra, Trash Tactician (alternate printing)
const MUERRA_TRASH_TACTICIAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MUERRA_TRASH_TACTICIAN,
    1,
    "26b4e695-5ec4-4821-8637-8df501ba0653",
    "Taro Yamazaki",
);

// BLB 353 — Ral, Crackling Wit (alternate printing)
const RAL_CRACKLING_WIT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &RAL_CRACKLING_WIT,
    2,
    "af6c496c-c70e-46f0-b161-661620767de5",
    "Atsushi Furusawa",
);

// BLB 354 — Vren, the Relentless (alternate printing)
const VREN_THE_RELENTLESS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VREN_THE_RELENTLESS,
    1,
    "88ca7c86-92d8-4a24-81b1-9eb11392160d",
    "MAMEZAWA",
);

// BLB 355 — Zoraline, Cosmos Caller (alternate printing)
const ZORALINE_COSMOS_CALLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ZORALINE_COSMOS_CALLER,
    1,
    "34690573-d8d6-471e-9878-cf5d931d9f55",
    "Taro Yamazaki",
);

// BLB 356 — Essence Channeler (alternate printing)
const ESSENCE_CHANNELER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ESSENCE_CHANNELER,
    1,
    "bbd5c86a-0991-4322-a0a2-48424c4be2af",
    "Wylie Beckert",
);

// BLB 357 — Kitnap (alternate printing)
const KITNAP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KITNAP,
    1,
    "12f3b75d-87d6-42dc-8096-41f5480b12fa",
    "Irina Nordsol",
);

// BLB 358 — Wishing Well (alternate printing)
const WISHING_WELL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WISHING_WELL,
    1,
    "e054e77f-94b9-4c43-94a4-75a7a5d1bd30",
    "Steven Belledin",
);

// BLB 359 — Iridescent Vinelasher (alternate printing)
const IRIDESCENT_VINELASHER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &IRIDESCENT_VINELASHER,
    1,
    "859e34b4-6762-465f-a5c7-24e42e112fbe",
    "Aaron Miller",
);

// BLB 360 — Byway Barterer (alternate printing)
const BYWAY_BARTERER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BYWAY_BARTERER,
    1,
    "3aebd0f9-fa08-407f-b3d3-fb4b13b1ea97",
    "Ryan Pancoast",
);

// BLB 361 — Hearthborn Battler (alternate printing)
const HEARTHBORN_BATTLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HEARTHBORN_BATTLER,
    1,
    "73a10a11-a21f-4e93-8771-8d971c158911",
    "Zoltan Boros",
);

// BLB 362 — Fecund Greenshell (alternate printing)
const FECUND_GREENSHELL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FECUND_GREENSHELL,
    1,
    "56301392-3496-48d0-8d91-6b82e1164c98",
    "Kisung Koh",
);

// BLB 363 — Pawpatch Recruit (alternate printing)
const PAWPATCH_RECRUIT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PAWPATCH_RECRUIT,
    1,
    "7fc5b558-4470-4bd7-9458-fbd275ded168",
    "Johan Grenier",
);

// BLB 364 — Thornvault Forager (alternate printing)
const THORNVAULT_FORAGER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THORNVAULT_FORAGER,
    1,
    "22ffe1ab-df93-4017-bb29-bb214c86d73a",
    "Mark Behm",
);

// BLB 365 — Dreamdew Entrancer (alternate printing)
const DREAMDEW_ENTRANCER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DREAMDEW_ENTRANCER,
    1,
    "1f2efb27-c181-43f7-93fe-a8c4d286f8ad",
    "Zoltan Boros",
);

// BLB 366 — Lunar Convocation (alternate printing)
const LUNAR_CONVOCATION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LUNAR_CONVOCATION,
    1,
    "4396e4c7-660d-4055-bc94-4ccea95223b7",
    "Pavel Kolomeyets",
);

// BLB 367 — Fabled Passage (alternate printing)
const FABLED_PASSAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_eld::FABLED_PASSAGE,
    1,
    "0bda8a2e-6bf1-4d8c-b71b-bbc8d8130fff",
    "Adam Paquette",
);

// BLB 368 — Fountainport (alternate printing)
const FOUNTAINPORT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FOUNTAINPORT,
    1,
    "dd04367b-59d8-4ee6-8b5a-abfb1373c226",
    "Leon Tukker",
);

// BLB 369 — Plains (alternate printing)
const PLAINS_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    4,
    "449d4122-ee9d-477c-976a-809ba8cb1443",
    "Piotr Dura",
);

// BLB 370 — Plains (alternate printing)
const PLAINS_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    5,
    "031b61a1-db66-416d-8187-edc32123131f",
    "Julian Kok Joon Wen",
);

// BLB 371 — Island (alternate printing)
const ISLAND_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    4,
    "46848fc2-ba58-4738-b4ce-0399d9053f48",
    "Alexander Forssberg",
);

// BLB 372 — Island (alternate printing)
const ISLAND_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    5,
    "10125dab-fe82-4796-b5db-3bd6ad389e1d",
    "Lorenzo Lanfranconi",
);

// BLB 373 — Swamp (alternate printing)
const SWAMP_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    4,
    "6125ffe3-4e48-4e0f-8390-7462446fc8bf",
    "Piotr Dura",
);

// BLB 374 — Swamp (alternate printing)
const SWAMP_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    5,
    "4dbfdbdd-6322-4ea6-9680-d4e480d78437",
    "Thomas Stoop",
);

// BLB 375 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    4,
    "4b827c54-8bfe-4ef5-830e-e17b8690bbe7",
    "Samuele Bandini",
);

// BLB 376 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    5,
    "62aaa129-69ff-4e22-861e-b2efd1cd5a10",
    "Adam Paquette",
);

// BLB 377 — Forest (alternate printing)
const FOREST_ALTERNATE_4: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    4,
    "43b3be4a-973d-4aeb-a94e-37e2710ac178",
    "Alayna Danner",
);

// BLB 378 — Forest (alternate printing)
const FOREST_ALTERNATE_5: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    5,
    "bddc66f7-4e94-4857-ba7d-6b0083d0bfa0",
    "Donato Giancola",
);

// BLB 379 — Bria, Riptide Rogue
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRIA_RIPTIDE_ROGUE: CardRecord = CardRecord::new(
    "Bria, Riptide Rogue",
    "390c96b3-68da-4a42-89ab-d9ccc79ce0dd",
    "Borja Pindado",
    crate::card::CardRules::unsupported(),
);

// BLB 380 — Byrke, Long Ear of the Law
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BYRKE_LONG_EAR_OF_THE_LAW: CardRecord = CardRecord::new(
    "Byrke, Long Ear of the Law",
    "6441abd3-320b-424a-9753-61e3581fe1a9",
    "Manuel Castañón",
    crate::card::CardRules::unsupported(),
);

// BLB 381 — Hop to It (alternate printing)
const HOP_TO_IT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HOP_TO_IT,
    1,
    "684614e3-5454-4e68-87da-a2027b6af6d7",
    "Eelis Kyttanen",
);

// BLB 382 — Shoreline Looter (alternate printing)
const SHORELINE_LOOTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SHORELINE_LOOTER,
    1,
    "00cb0b3c-a403-4212-93d3-3340c3f1e9db",
    "PINDURSKI",
);

// BLB 383 — Fell (alternate printing)
const FELL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FELL,
    1,
    "7a4bb137-2639-4158-a24b-93c8e4f4cc21",
    "A. M. Sartor",
);

// BLB 384 — Wear Down (alternate printing)
const WEAR_DOWN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WEAR_DOWN,
    1,
    "3c394081-0292-49f8-8a6f-213956754020",
    "Iris Compiet",
);

// BLB 385 — Stormcatch Mentor (alternate printing)
const STORMCATCH_MENTOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STORMCATCH_MENTOR,
    1,
    "e1e5a7a1-4ac5-4d9f-8d27-0877efca47f1",
    "Manuel Castañón",
);

// BLB 386 — Thundertrap Trainer (alternate printing)
const THUNDERTRAP_TRAINER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THUNDERTRAP_TRAINER,
    2,
    "e52caa08-34ae-4c74-83ad-008d17005576",
    "Jesper Ejsing",
);

// BLB 387 — Serra Redeemer (reprint)
const SERRA_REDEEMER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dmu::SERRA_REDEEMER,
    "230b9aef-bd9c-4332-ace4-b5b065bac6d8",
    "Joshua Raphael",
);

// BLB 388 — Charmed Sleep (reprint)
const CHARMED_SLEEP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_eld::CHARMED_SLEEP,
    "7d171802-2604-45a5-a0f2-ab5afa1db5d5",
    "Titus Lunter",
);

// BLB 389 — Mind Spring (reprint)
const MIND_SPRING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mor::MIND_SPRING,
    "e7e7d174-eb7c-41ad-a241-cfbfdc71e3a7",
    "Mark Zug",
);

// BLB 390 — Thieving Otter (reprint)
const THIEVING_OTTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_iko::THIEVING_OTTER,
    "52a258be-39e3-4689-b2d0-7c353ce7d574",
    "Jakub Kasper",
);

// BLB 391 — Flame Lash (reprint)
const FLAME_LASH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_kld::FLAME_LASH,
    "c6440439-7178-4a97-9e18-7fdef4b02678",
    "Viktor Titov",
);

// BLB 392 — Colossification (reprint)
const COLOSSIFICATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_iko::COLOSSIFICATION,
    "c2cad902-ffd2-4bab-b114-b4b6df2ac6b3",
    "Johan Grenier",
);

// BLB 393 — Giant Growth (reprint)
const GIANT_GROWTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GIANT_GROWTH,
    "e70722d6-b4d5-45c2-9488-9a5eb0bdb9bd",
    "Dmitry Burmak",
);

// BLB 394 — Rabid Bite (reprint)
const RABID_BITE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_soi::RABID_BITE,
    "53a73200-b798-4bfd-a431-8b94e17b70be",
    "John Thacker",
);

// BLB 395 — Sword of Vengeance (reprint)
const SWORD_OF_VENGEANCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m11::SWORD_OF_VENGEANCE,
    "f21b5fc1-7611-44ac-ad8d-1f0c6d4fc9a3",
    "Dan Murayama Scott",
);

// BLB 396 — Blossoming Sands (reprint)
const BLOSSOMING_SANDS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::BLOSSOMING_SANDS,
    "ecfa2cc9-e427-4104-bad7-b0294e392b1f",
    "Sam Burley",
);

// BLB 397 — Swiftwater Cliffs (reprint)
const SWIFTWATER_CLIFFS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ktk::SWIFTWATER_CLIFFS,
    "89e43510-c444-4b2e-b2a0-528dcc09c899",
    "Eytan Zana",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &BEZA_THE_BOUNDING_SPRING,
    &BRAVE_KIN_DUO,
    &BRIGHTBLADE_STOAT,
    &BUILDER_S_TALENT,
    &CARETAKER_S_TALENT,
    &CARROT_CAKE,
    &CRUMB_AND_GET_IT,
    &DAWN_S_TRUCE,
    &DEWDROP_CURE,
    &DRIFTGLOOM_COYOTE,
    &ESSENCE_CHANNELER,
    &FEATHER_OF_FLIGHT,
    &FLOWERFOOT_SWORDMASTER,
    &HARVESTRITE_HOST,
    &HOP_TO_IT,
    &INTREPID_RABBIT,
    &JACKDAW_SAVIOR,
    &JOLLY_GERBILS,
    &LIFECREED_DUO,
    &MABEL_S_METTLE,
    &MOUSE_TRAPPER,
    &NETTLE_GUARD,
    &PARTING_GUST,
    &PILEATED_PROVISIONER,
    &RABBIT_RESPONSE,
    &REPEL_CALAMITY,
    &SALVATION_SWAN,
    &SEASON_OF_THE_BURROW,
    &SEASONED_WARRENGUARD,
    &SHRIKE_FORCE,
    &SONAR_STRIKE,
    &STAR_CHARTER,
    &STARFALL_INVOCATION,
    &THISTLEDOWN_PLAYERS,
    &VALLEY_QUESTCALLER,
    &WARREN_ELDER,
    &WARREN_WARLEADER,
    &WAX_WANE_WITNESS,
    &WHISKERVALE_FORERUNNER,
    &AZURE_BEASTBINDER,
    &BELLOWING_CRIER,
    &CALAMITOUS_TIDE,
    &DARING_WAVERIDER,
    &DAZZLING_DENIAL,
    &DIRE_DOWNDRAFT,
    &DOUR_PORT_MAGE,
    &EDDYMURK_CRAB,
    &ELUGE_THE_SHORELESS_SEA,
    &FINCH_FORMATION,
    &GOSSIP_S_TALENT,
    &INTO_THE_FLOOD_MAW,
    &KITNAP,
    &KITSA_OTTERBALL_ELITE,
    &KNIGHTFISHER,
    &LIGHTSHELL_DUO,
    &LONG_RIVER_LURKER,
    &LONG_RIVER_S_PULL,
    &MIND_SPIRAL,
    &MINDWHISKER,
    &MOCKINGBIRD,
    &NIGHTWHORL_HERMIT,
    &OTTERBALL_ANTICS,
    &PEARL_OF_WISDOM,
    &PLUMECREED_ESCORT,
    &PORTENT_OF_CALAMITY,
    &SEASON_OF_WEAVING,
    &SHORELINE_LOOTER,
    &SKYSKIPPER_DUO,
    &SPELLGYRE,
    &SPLASH_LASHER,
    &SPLASH_PORTAL,
    &STORMCHASERS_TALENT,
    &SUGAR_COAT,
    &THOUGHT_SHUCKER,
    &THUNDERTRAP_TRAINER,
    &VALLEY_FLOODCALLER,
    &WATERSPOUT_WARDEN,
    &WISHING_WELL,
    &AGATE_BLADE_ASSASSIN,
    &BANDIT_S_TALENT,
    &BONEBIND_ORATOR,
    &BONECACHE_OVERSEER,
    &COILING_REBIRTH,
    &CONSUMED_BY_GREED,
    &CRUELCLAW_S_HEIST,
    &DAGGERFANG_DUO,
    &DARKSTAR_AUGUR,
    &DIRESIGHT,
    &DOWNWIND_AMBUSHER,
    &EARLY_WINTER,
    &FEED_THE_CYCLE,
    &FELL,
    &GLIDEDIVE_DUO,
    &HAZEL_S_NOCTURNE,
    &HUSKBURSTER_SWARM,
    &IRIDESCENT_VINELASHER,
    &MAHA_ITS_FEATHERS_NIGHT,
    &MOONSTONE_HARBINGER,
    &NOCTURNAL_HUNGER,
    &OSTEOMANCER_ADEPT,
    &PERSISTENT_MARSHSTALKER,
    &PSYCHIC_WHORL,
    &RAVINE_RAIDER,
    &ROTTENMOUTH_VIPER,
    &RUTHLESS_NEGOTIATION,
    &SAVOR,
    &SCALES_OF_SHALE,
    &SCAVENGER_S_TALENT,
    &SEASON_OF_LOSS,
    &SINISTER_MONOLITH,
    &STARGAZE,
    &STARLIT_SOOTHSAYER,
    &STARSCAPE_CLERIC,
    &THORNPLATE_INTIMIDATOR,
    &THOUGHT_STALKER_WARLOCK,
    &VALLEY_ROTCALLER,
    &WICK_THE_WHORLED_MIND,
    &WICK_S_PATROL,
    &AGATE_ASSAULT,
    &ALANIA_S_PATHMAKER,
    &ARTIST_S_TALENT,
    &BLACKSMITH_S_TALENT,
    &BLOOMING_BLAST,
    &BRAMBLEGUARD_CAPTAIN,
    &BRAZEN_COLLECTOR,
    &BYWAY_BARTERER,
    &CONDUCT_ELECTRICITY,
    &CORUSCATION_MAGE,
    &DRAGONHAWK_FATE_S_TEMPEST,
    &EMBERHEART_CHALLENGER,
    &FESTIVAL_OF_EMBERS,
    &FLAMECACHE_GECKO,
    &FRILLED_SPARKSHOOTER,
    &HARNESSER_OF_STORMS,
    &HEARTFIRE_HERO,
    &HEARTHBORN_BATTLER,
    &HIRED_CLAW,
    &HOARDER_S_OVERFLOW,
    &KINDLESPARK_DUO,
    &MANIFOLD_MOUSE,
    &MIGHT_OF_THE_MEEK,
    &PLAYFUL_SHOVE,
    &QUAKETUSK_BOAR,
    &RABID_GNAW,
    &RACCOON_RALLIER,
    &REPTILIAN_RECRUITER,
    &ROUGHSHOD_DUO,
    &SAZACAP_S_BREW,
    &SEASON_OF_THE_BOLD,
    &STEAMPATH_CHARGER,
    &STORMSPLITTER,
    &SUNSPINE_LYNX,
    &TAKE_OUT_THE_TRASH,
    &TEAPOT_SLINGER,
    &VALLEY_FLAMECALLER,
    &VALLEY_RALLY,
    &WAR_SQUEAK,
    &WHISKERQUILL_SCRIBE,
    &WILDFIRE_HOWL,
    &BAKERSBANE_DUO,
    &BARK_KNUCKLE_BOXER,
    &BRAMBLEGUARD_VETERAN,
    &BUSHY_BODYGUARD,
    &CACHE_GRAB,
    &CLIFFTOP_LOOKOUT,
    &CURIOUS_FORAGER,
    &DRUID_OF_THE_SPADE,
    &FECUND_GREENSHELL,
    &FOR_THE_COMMON_GOOD,
    &GALEWIND_MOOSE,
    &HAZARDROOT_HERBALIST,
    &HEAPED_HARVEST,
    &HIGH_STRIDE,
    &HIVESPINE_WOLVERINE,
    &HONORED_DREYLEADER,
    &HUNTER_S_TALENT,
    &INNKEEPER_S_TALENT,
    &LONGSTALK_BRAWL,
    &LUMRA_BELLOW_OF_THE_WOODS,
    &MISTBREATH_ELDER,
    &OVERPROTECT,
    &PAWPATCH_FORMATION,
    &PAWPATCH_RECRUIT,
    &PEERLESS_RECYCLING,
    &POLLIWALLOP,
    &RUST_SHIELD_RAMPAGER,
    &SCRAPSHOOTER,
    &SEASON_OF_GATHERING,
    &STICKYTONGUE_SENTINEL,
    &STOCKING_THE_PANTRY,
    &SUNSHOWER_DRUID,
    &TENDER_WILDGUIDE,
    &THORNVAULT_FORAGER,
    &THREE_TREE_ROOTWEAVER,
    &THREE_TREE_SCRIBE,
    &TREEGUARD_DUO,
    &TREETOP_SENTRIES,
    &VALLEY_MIGHTCALLER,
    &WEAR_DOWN,
    &ALANIA_DIVERGENT_STORM,
    &BAYLEN_THE_HAYMAKER,
    &BURROWGUARD_MENTOR,
    &CAMELLIA_THE_SEEDMISER,
    &CINDERING_CUTTHROAT,
    &CLEMENT_THE_WORRYWORT,
    &CORPSEBERRY_CULTIVATOR,
    &DREAMDEW_ENTRANCER,
    &FINNEAS_ACE_ARCHER,
    &FIREGLASS_MENTOR,
    &GEV_SCALED_SCORCH,
    &GLARB_CALAMITY_S_AUGUR,
    &HEAD_OF_THE_HOMESTEAD,
    &HELGA_SKITTISH_SEER,
    &HUGS_GRISLY_GUARDIAN,
    &THE_INFAMOUS_CRUELCLAW,
    &JUNKBLADE_BRUISER,
    &KASTRAL_THE_WINDCRESTED,
    &LILYSPLASH_MENTOR,
    &LUNAR_CONVOCATION,
    &MABEL_HEIR_TO_CRAGFLAME,
    &MIND_DRILL_ASSAILANT,
    &MOONRISE_CLERIC,
    &MUERRA_TRASH_TACTICIAN,
    &PLUMECREED_MENTOR,
    &POND_PROPHET,
    &RAL_CRACKLING_WIT,
    &SEEDGLAIVE_MENTOR,
    &SEEDPOD_SQUIRE,
    &STARSEER_MENTOR,
    &STORMCATCH_MENTOR,
    &TEMPEST_ANGLER,
    &TIDECALLER_MENTOR,
    &VETERAN_GUARDMOUSE,
    &VINEREAP_MENTOR,
    &VREN_THE_RELENTLESS,
    &WANDERTALE_MENTOR,
    &YGRA_EATER_OF_ALL,
    &ZORALINE_COSMOS_CALLER,
    &BARKFORM_HARVESTER,
    &BUMBLEFLOWER_S_SHAREPOT,
    &FOUNTAINPORT_BELL,
    &HEIRLOOM_EPIC,
    &PATCHWORK_BANNER,
    &SHORT_BOW,
    &STARFORGED_SWORD,
    &TANGLE_TUMBLER,
    &THREE_TREE_MASCOT,
    &FOUNTAINPORT,
    &HIDDEN_GROTTO,
    &LILYPAD_VILLAGE,
    &LUPINFLOWER_VILLAGE,
    &MUDFLAT_VILLAGE,
    &OAKHOLLOW_VILLAGE,
    &ROCKFACE_VILLAGE,
    &THREE_TREE_CITY,
    &KEEN_EYED_CURATOR,
    &BRIA_RIPTIDE_ROGUE,
    &BYRKE_LONG_EAR_OF_THE_LAW,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    BANISHING_LIGHT_REPRINT,
    RUN_AWAY_TOGETHER_REPRINT,
    SHORE_UP_REPRINT,
    KEEN_EYED_CURATOR_ALTERNATE_1,
    FABLED_PASSAGE_REPRINT,
    UNCHARTED_HAVEN_REPRINT,
    PLAINS_REPRINT,
    PLAINS_ALTERNATE_1,
    PLAINS_ALTERNATE_2,
    PLAINS_ALTERNATE_3,
    ISLAND_REPRINT,
    ISLAND_ALTERNATE_1,
    ISLAND_ALTERNATE_2,
    ISLAND_ALTERNATE_3,
    SWAMP_REPRINT,
    SWAMP_ALTERNATE_1,
    SWAMP_ALTERNATE_2,
    SWAMP_ALTERNATE_3,
    MOUNTAIN_REPRINT,
    MOUNTAIN_ALTERNATE_1,
    MOUNTAIN_ALTERNATE_2,
    MOUNTAIN_ALTERNATE_3,
    FOREST_REPRINT,
    FOREST_ALTERNATE_1,
    FOREST_ALTERNATE_2,
    FOREST_ALTERNATE_3,
    SEASON_OF_THE_BURROW_ALTERNATE_1,
    SEASON_OF_WEAVING_ALTERNATE_1,
    SEASON_OF_LOSS_ALTERNATE_1,
    SEASON_OF_THE_BOLD_ALTERNATE_1,
    SEASON_OF_GATHERING_ALTERNATE_1,
    BEZA_THE_BOUNDING_SPRING_ALTERNATE_1,
    ELUGE_THE_SHORELESS_SEA_ALTERNATE_1,
    MAHA_ITS_FEATHERS_NIGHT_ALTERNATE_1,
    ROTTENMOUTH_VIPER_ALTERNATE_1,
    DRAGONHAWK_FATE_S_TEMPEST_ALTERNATE_1,
    SUNSPINE_LYNX_ALTERNATE_1,
    LUMRA_BELLOW_OF_THE_WOODS_ALTERNATE_1,
    YGRA_EATER_OF_ALL_ALTERNATE_1,
    DAWN_S_TRUCE_ALTERNATE_1,
    JACKDAW_SAVIOR_ALTERNATE_1,
    SALVATION_SWAN_ALTERNATE_1,
    STARFALL_INVOCATION_ALTERNATE_1,
    VALLEY_QUESTCALLER_ALTERNATE_1,
    WARREN_WARLEADER_ALTERNATE_1,
    WHISKERVALE_FORERUNNER_ALTERNATE_1,
    AZURE_BEASTBINDER_ALTERNATE_1,
    DOUR_PORT_MAGE_ALTERNATE_1,
    KITSA_OTTERBALL_ELITE_ALTERNATE_1,
    MOCKINGBIRD_ALTERNATE_1,
    PORTENT_OF_CALAMITY_ALTERNATE_1,
    THUNDERTRAP_TRAINER_ALTERNATE_1,
    VALLEY_FLOODCALLER_ALTERNATE_1,
    COILING_REBIRTH_ALTERNATE_1,
    CRUELCLAW_S_HEIST_ALTERNATE_1,
    DARKSTAR_AUGUR_ALTERNATE_1,
    OSTEOMANCER_ADEPT_ALTERNATE_1,
    VALLEY_ROTCALLER_ALTERNATE_1,
    WICK_THE_WHORLED_MIND_ALTERNATE_1,
    EMBERHEART_CHALLENGER_ALTERNATE_1,
    FESTIVAL_OF_EMBERS_ALTERNATE_1,
    HIRED_CLAW_ALTERNATE_1,
    MANIFOLD_MOUSE_ALTERNATE_1,
    STORMSPLITTER_ALTERNATE_1,
    VALLEY_FLAMECALLER_ALTERNATE_1,
    FOR_THE_COMMON_GOOD_ALTERNATE_1,
    MISTBREATH_ELDER_ALTERNATE_1,
    SCRAPSHOOTER_ALTERNATE_1,
    TENDER_WILDGUIDE_ALTERNATE_1,
    VALLEY_MIGHTCALLER_ALTERNATE_1,
    ALANIA_DIVERGENT_STORM_ALTERNATE_1,
    CAMELLIA_THE_SEEDMISER_ALTERNATE_1,
    CLEMENT_THE_WORRYWORT_ALTERNATE_1,
    FINNEAS_ACE_ARCHER_ALTERNATE_1,
    GLARB_CALAMITY_S_AUGUR_ALTERNATE_1,
    HELGA_SKITTISH_SEER_ALTERNATE_1,
    HUGS_GRISLY_GUARDIAN_ALTERNATE_1,
    THE_INFAMOUS_CRUELCLAW_ALTERNATE_1,
    KASTRAL_THE_WINDCRESTED_ALTERNATE_1,
    MABEL_HEIR_TO_CRAGFLAME_ALTERNATE_1,
    THREE_TREE_CITY_ALTERNATE_1,
    THREE_TREE_CITY_ALTERNATE_2,
    THREE_TREE_CITY_ALTERNATE_3,
    THREE_TREE_CITY_ALTERNATE_4,
    RAL_CRACKLING_WIT_ALTERNATE_1,
    LUMRA_BELLOW_OF_THE_WOODS_ALTERNATE_2,
    LUMRA_BELLOW_OF_THE_WOODS_ALTERNATE_3,
    ALANIA_DIVERGENT_STORM_ALTERNATE_2,
    BAYLEN_THE_HAYMAKER_ALTERNATE_1,
    CAMELLIA_THE_SEEDMISER_ALTERNATE_2,
    CLEMENT_THE_WORRYWORT_ALTERNATE_2,
    FINNEAS_ACE_ARCHER_ALTERNATE_2,
    GEV_SCALED_SCORCH_ALTERNATE_1,
    KASTRAL_THE_WINDCRESTED_ALTERNATE_2,
    MABEL_HEIR_TO_CRAGFLAME_ALTERNATE_2,
    MUERRA_TRASH_TACTICIAN_ALTERNATE_1,
    RAL_CRACKLING_WIT_ALTERNATE_2,
    VREN_THE_RELENTLESS_ALTERNATE_1,
    ZORALINE_COSMOS_CALLER_ALTERNATE_1,
    ESSENCE_CHANNELER_ALTERNATE_1,
    KITNAP_ALTERNATE_1,
    WISHING_WELL_ALTERNATE_1,
    IRIDESCENT_VINELASHER_ALTERNATE_1,
    BYWAY_BARTERER_ALTERNATE_1,
    HEARTHBORN_BATTLER_ALTERNATE_1,
    FECUND_GREENSHELL_ALTERNATE_1,
    PAWPATCH_RECRUIT_ALTERNATE_1,
    THORNVAULT_FORAGER_ALTERNATE_1,
    DREAMDEW_ENTRANCER_ALTERNATE_1,
    LUNAR_CONVOCATION_ALTERNATE_1,
    FABLED_PASSAGE_ALTERNATE_1,
    FOUNTAINPORT_ALTERNATE_1,
    PLAINS_ALTERNATE_4,
    PLAINS_ALTERNATE_5,
    ISLAND_ALTERNATE_4,
    ISLAND_ALTERNATE_5,
    SWAMP_ALTERNATE_4,
    SWAMP_ALTERNATE_5,
    MOUNTAIN_ALTERNATE_4,
    MOUNTAIN_ALTERNATE_5,
    FOREST_ALTERNATE_4,
    FOREST_ALTERNATE_5,
    HOP_TO_IT_ALTERNATE_1,
    SHORELINE_LOOTER_ALTERNATE_1,
    FELL_ALTERNATE_1,
    WEAR_DOWN_ALTERNATE_1,
    STORMCATCH_MENTOR_ALTERNATE_1,
    THUNDERTRAP_TRAINER_ALTERNATE_2,
    SERRA_REDEEMER_REPRINT,
    CHARMED_SLEEP_REPRINT,
    MIND_SPRING_REPRINT,
    THIEVING_OTTER_REPRINT,
    FLAME_LASH_REPRINT,
    COLOSSIFICATION_REPRINT,
    GIANT_GROWTH_REPRINT,
    RABID_BITE_REPRINT,
    SWORD_OF_VENGEANCE_REPRINT,
    BLOSSOMING_SANDS_REPRINT,
    SWIFTWATER_CLIFFS_REPRINT,
];
