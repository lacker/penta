//! Judgment cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::sets::y1993::arabian_nights as catalog_arn;
use crate::card::sets::y1997::weatherlight as catalog_wth;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AggregateOperationDef, AlternativeCastKindDef, AppliedEffectDef, CardRules, CardSet, CardType,
    CharacteristicOperationDef, ChoiceVisibilityDef, ChooseDef, CostQuantityDef, EffectDef,
    EffectRecipientDef, ManaColor, ObjectChoiceBindingDef, ObjectPredicateDef, ObjectQueryDef,
    ObjectSetDef, ObjectValueAggregateDef, ObjectValueDef, PlayerRefDef, PlayerRelation,
    PlayerSetDef, PowerToughnessOperationDef, ReplacementChoiceDef, ReplacementEffectDef,
    ResolvedEffectDurationDef, SpellAdditionalCostDef, ValueDef, ZoneKind, ZonePlacement,
    abilities,
};
use crate::ids::ParentBinding;
use crate::{TargetIndex, mana_cost};

// JUD 1 — Ancestor's Chosen
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANCESTOR_S_CHOSEN: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Ancestor's Chosen",
    "c0cf71e1-3c57-47f9-a4ef-e0d0ad1ee329",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// JUD 2 — Aven Warcraft
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVEN_WARCRAFT: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Aven Warcraft",
    "1fd5d21a-b151-4fea-ac0d-6659af131bf9",
    "Roger Raupp",
    crate::card::CardRules::unsupported(),
);

// JUD 3 — Battle Screech
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BATTLE_SCREECH: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Battle Screech",
    "c3c38264-0d79-47d4-bca2-a20a991bbac9",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// JUD 4 — Battlewise Aven
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BATTLEWISE_AVEN: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Battlewise Aven",
    "ca363409-cba9-43ed-bf88-3519521e1983",
    "Wayne England",
    crate::card::CardRules::unsupported(),
);

// JUD 5 — Benevolent Bodyguard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BENEVOLENT_BODYGUARD: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Benevolent Bodyguard",
    "22492fb3-5ceb-4d5e-ba82-ae1a6a69c105",
    "Roger Raupp",
    crate::card::CardRules::unsupported(),
);

// JUD 6 — Border Patrol
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BORDER_PATROL: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Border Patrol",
    "a49a85c8-3516-4dda-b16b-bf1bf890becb",
    "Roger Raupp",
    crate::card::CardRules::unsupported(),
);

// JUD 7 — Cagemail
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CAGEMAIL: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Cagemail",
    "72ab91ab-2bcf-4617-bec3-2bf040d4997c",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// JUD 8 — Chastise
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHASTISE: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Chastise",
    "1169dab7-8f4c-474d-9289-42765a275376",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// JUD 9 — Commander Eesha
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COMMANDER_EESHA: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Commander Eesha",
    "3607f6a9-b8d2-4119-9f70-95dcedc0662d",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// JUD 10 — Funeral Pyre
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FUNERAL_PYRE: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Funeral Pyre",
    "4d8542f6-ee34-42c6-acd5-07b0c7cc2f63",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// JUD 11 — Glory
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLORY: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Glory",
    "7a414f0e-b157-4570-8213-1c58a96bf7a5",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// JUD 12 — Golden Wish
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOLDEN_WISH: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Golden Wish",
    "dc409ded-41f3-4f14-8199-72a9fe98bac0",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// JUD 13 — Guided Strike (reprint)
const GUIDED_STRIKE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_wth::GUIDED_STRIKE,
    "d13138c1-e98f-4803-8c68-ffc80139c168",
    "Dave Dorman",
);

// JUD 14 — Lead Astray
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LEAD_ASTRAY: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Lead Astray",
    "20a8fd2f-11fa-4879-be89-ea7833cf60d4",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// JUD 15 — Nomad Mythmaker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NOMAD_MYTHMAKER: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Nomad Mythmaker",
    "9a5694fe-57d2-4359-857a-63213d986747",
    "Eric Peterson",
    crate::card::CardRules::unsupported(),
);

// JUD 16 — Phantom Flock
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHANTOM_FLOCK: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Phantom Flock",
    "617d4246-c827-4742-ab6a-c5170c12cb87",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// JUD 17 — Phantom Nomad
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHANTOM_NOMAD: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Phantom Nomad",
    "6c5309f5-8b32-4a57-99f2-dcf7a8341898",
    "Jim Nelson",
    crate::card::CardRules::unsupported(),
);

// JUD 18 — Prismatic Strands
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PRISMATIC_STRANDS: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Prismatic Strands",
    "3454ef42-2e0b-4ce4-945f-e4ec3e83c39d",
    "Eric Peterson",
    crate::card::CardRules::unsupported(),
);

// JUD 19 — Pulsemage Advocate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PULSEMAGE_ADVOCATE: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Pulsemage Advocate",
    "0dce0e8f-9ad6-42b6-af61-c883613efc97",
    "Jeff Easley",
    crate::card::CardRules::unsupported(),
);

// JUD 20 — Ray of Revelation
pub(in crate::card::sets) static RAY_OF_REVELATION: CardRecord = CardRecord::new(
    CardSet::Judgment,
    "Ray of Revelation",
    "6d762c8c-6172-4dc0-8fcc-d0f6dd8ca013",
    "Doug Chaffee",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Destroy target enchantment.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Enchantment),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
        ),
        abilities::flashback(mana_cost!("{G}")),
    ]),
);

// JUD 21 — Selfless Exorcist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SELFLESS_EXORCIST: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Selfless Exorcist",
    "c9b1c300-aec3-4512-9902-309615e86c73",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// JUD 22 — Shieldmage Advocate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHIELDMAGE_ADVOCATE: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Shieldmage Advocate",
    "2ea66a41-cb2e-49d6-81fe-3f69b0dfd40e",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// JUD 23 — Silver Seraph
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SILVER_SERAPH: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Silver Seraph",
    "1465ca9e-a997-4b8c-9677-6c7961f67eba",
    "Matthew D. Wilson",
    crate::card::CardRules::unsupported(),
);

// JUD 24 — Solitary Confinement
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOLITARY_CONFINEMENT: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Solitary Confinement",
    "e7a8eb7a-eb3f-405e-8f44-d8ea64d76386",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// JUD 25 — Soulcatchers' Aerie
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOULCATCHERS_AERIE: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Soulcatchers' Aerie",
    "b30df994-bb09-4d16-8443-223c6ce342dc",
    "Rob Alexander",
    crate::card::CardRules::unsupported(),
);

// JUD 26 — Spirit Cairn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIRIT_CAIRN: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Spirit Cairn",
    "d8baf60f-c20b-4b2f-9fe1-df008a9273c6",
    "Gary Ruddell",
    crate::card::CardRules::unsupported(),
);

// JUD 27 — Spurnmage Advocate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPURNMAGE_ADVOCATE: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Spurnmage Advocate",
    "008c8d72-097e-472d-88c8-78bf29e42e32",
    "Ron Spears",
    crate::card::CardRules::unsupported(),
);

// JUD 28 — Suntail Hawk
pub(in crate::card::sets) static SUNTAIL_HAWK: CardRecord = CardRecord::new(
    CardSet::Judgment,
    "Suntail Hawk",
    "5fbdae0b-b4aa-40ff-9017-b4349bd6b627",
    "Heather Hudson",
    CardRules::new_creature(mana_cost!("{W}"), &["Bird"], 1, 1)
        .with_abilities(&[abilities::flying()]),
);

// JUD 29 — Test of Endurance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEST_OF_ENDURANCE: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Test of Endurance",
    "cf16bd6b-e99c-4da3-bd03-11f63b7ee85d",
    "Mike Ploog",
    crate::card::CardRules::unsupported(),
);

// JUD 30 — Trained Pronghorn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRAINED_PRONGHORN: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Trained Pronghorn",
    "720ec745-226c-4211-974f-e04a4f9e1902",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// JUD 31 — Unquestioned Authority
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNQUESTIONED_AUTHORITY: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Unquestioned Authority",
    "a015205e-5895-4038-9c2f-ed4766c498ff",
    "Alex Horley-Orlandelli",
    crate::card::CardRules::unsupported(),
);

// JUD 32 — Valor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VALOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Valor",
    "58095f6b-d937-4871-b3af-5c6a1d9c04b3",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// JUD 33 — Vigilant Sentry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VIGILANT_SENTRY: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Vigilant Sentry",
    "c3790282-ea04-4600-8912-dac541ffd081",
    "Eric Peterson",
    crate::card::CardRules::unsupported(),
);

// JUD 34 — Aven Fogbringer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVEN_FOGBRINGER: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Aven Fogbringer",
    "c0ee9e09-c4b1-4133-90a3-350677f0b72a",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// JUD 35 — Cephalid Constable
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CEPHALID_CONSTABLE: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Cephalid Constable",
    "6d98f05b-ddfe-4b93-b247-dbd1a89e0731",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// JUD 36 — Cephalid Inkshrouder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CEPHALID_INKSHROUDER: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Cephalid Inkshrouder",
    "2c3e8c80-690b-4d79-9dee-99d1a3876160",
    "Tony Szczudlo",
    crate::card::CardRules::unsupported(),
);

// JUD 37 — Cunning Wish
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CUNNING_WISH: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Cunning Wish",
    "ca097675-5e82-493d-beab-9fc11efd7492",
    "Jim Nelson",
    crate::card::CardRules::unsupported(),
);

// JUD 38 — Defy Gravity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEFY_GRAVITY: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Defy Gravity",
    "461413fe-0392-41c1-b50f-05e87ea1c338",
    "Ben Thompson",
    crate::card::CardRules::unsupported(),
);

// JUD 39 — Envelop
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENVELOP: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Envelop",
    "e7ed250e-12d0-4ebc-9410-5711e71c6d1f",
    "Don Hazeltine",
    crate::card::CardRules::unsupported(),
);

// JUD 40 — Flash of Insight
pub(in crate::card::sets) static FLASH_OF_INSIGHT: CardRecord = CardRecord::new(
    CardSet::Judgment,
    "Flash of Insight",
    "ffaab905-0b97-42c2-a1a3-1e72275caa82",
    "Ben Thompson",
    // Cast small early, flashed back huge late: the graveyard a control deck
    // fills is the second casting's mana.
    CardRules::new_instant(mana_cost!("{X}{1}{U}")).with_abilities(&[
        AbilityDef::spell(
            "Look at the top X cards of your library. Put one of them into your hand and the rest on the bottom of your library in any order.",
            abilities::look_at_top_cards_choose_to_hand_rest_bottom(
                ValueDef::ChosenX,
                ObjectPredicateDef::Any,
                1,
                1,
            ),
        ),
        AbilityDef::alternative_cast(
            mana_cost!("{1}{U}"),
            AlternativeCastKindDef::Flashback,
            Some("Flashback—{1}{U}, Exile X blue cards from your graveyard."),
            EffectDef::None,
        )
        // X blue cards from your own graveyard, exiled to pay. The count is the same
        // X the spell is cast for, which is what makes the flashback expensive
        // exactly when it is worth casting big.
        .with_alternative_additional_cost(&SpellAdditionalCostDef::exile(
            ObjectPredicateDef::Color(ManaColor::Blue),
            ZoneKind::Graveyard,
            CostQuantityDef::ChosenX,
        )),
    ]),
);

// JUD 41 — Grip of Amnesia
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRIP_OF_AMNESIA: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Grip of Amnesia",
    "43dc7e2a-5b9b-4f0f-8b2e-a7c7f847e1f1",
    "Bradley Williams",
    crate::card::CardRules::unsupported(),
);

// JUD 42 — Hapless Researcher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HAPLESS_RESEARCHER: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Hapless Researcher",
    "22ed0ee7-6749-4f38-8e53-c11b46b17e5d",
    "Ron Spears",
    crate::card::CardRules::unsupported(),
);

// JUD 43 — Keep Watch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KEEP_WATCH: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Keep Watch",
    "6e702ee4-62b5-4d3b-a202-8cac4b84591c",
    "Fred Rahmqvist",
    crate::card::CardRules::unsupported(),
);

// JUD 44 — Laquatus's Disdain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAQUATUS_S_DISDAIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Laquatus's Disdain",
    "e2ea5448-2d72-42eb-814c-197153d8e06a",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// JUD 45 — Lost in Thought
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LOST_IN_THOUGHT: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Lost in Thought",
    "f5fb391a-2687-461d-b5ef-a494287ddb5d",
    "Ben Thompson",
    crate::card::CardRules::unsupported(),
);

// JUD 46 — Mental Note
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MENTAL_NOTE: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Mental Note",
    "1f343724-6ecd-494f-8bfc-93676af4e173",
    "Bradley Williams",
    crate::card::CardRules::unsupported(),
);

// JUD 47 — Mirror Wall
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIRROR_WALL: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Mirror Wall",
    "d0e52b73-ebdf-4339-8780-84327a59ca57",
    "Mark Brill",
    crate::card::CardRules::unsupported(),
);

// JUD 48 — Mist of Stagnation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIST_OF_STAGNATION: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Mist of Stagnation",
    "76d03121-9515-4101-9d60-e01225533f44",
    "Mike Ploog",
    crate::card::CardRules::unsupported(),
);

// JUD 49 — Quiet Speculation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static QUIET_SPECULATION: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Quiet Speculation",
    "71a314fa-293b-486f-95d8-267d340e4d8e",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// JUD 50 — Scalpelexis
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCALPELEXIS: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Scalpelexis",
    "29c3b7fa-78e7-4a0c-bcdc-4b829638e3f6",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// JUD 51 — Spelljack
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPELLJACK: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Spelljack",
    "3eda8c7b-ce35-482a-bece-52a30cc78a9a",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// JUD 52 — Telekinetic Bonds
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TELEKINETIC_BONDS: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Telekinetic Bonds",
    "d68fad1d-a517-433a-b939-d0635e8f5535",
    "Jim Nelson",
    crate::card::CardRules::unsupported(),
);

// JUD 53 — Web of Inertia
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WEB_OF_INERTIA: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Web of Inertia",
    "0e6d2721-1dfc-4f4f-a914-9352ca6981c0",
    "Don Hazeltine",
    crate::card::CardRules::unsupported(),
);

// JUD 54 — Wonder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WONDER: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Wonder",
    "44670666-9028-4b4a-a5af-a3bf35fc6a21",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// JUD 55 — Wormfang Behemoth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WORMFANG_BEHEMOTH: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Wormfang Behemoth",
    "1c7f29aa-c069-4adb-b313-6a56849905d4",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// JUD 56 — Wormfang Crab
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WORMFANG_CRAB: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Wormfang Crab",
    "dcf56dcf-ec1a-4298-8644-1fe248443b7e",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// JUD 57 — Wormfang Drake
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WORMFANG_DRAKE: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Wormfang Drake",
    "b6afd312-6448-4bd1-8539-0910cefead0d",
    "Thomas M. Baxa",
    crate::card::CardRules::unsupported(),
);

// JUD 58 — Wormfang Manta
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WORMFANG_MANTA: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Wormfang Manta",
    "bc9bf91d-6f7c-4fb5-bbc6-c012212e62e9",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// JUD 59 — Wormfang Newt
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WORMFANG_NEWT: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Wormfang Newt",
    "df8012c1-76ec-4c36-8b38-5bc41ce5e156",
    "Doug Chaffee",
    crate::card::CardRules::unsupported(),
);

// JUD 60 — Wormfang Turtle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WORMFANG_TURTLE: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Wormfang Turtle",
    "48404362-7579-4896-a71a-8eb40e5ac416",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// JUD 61 — Balthor the Defiled
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BALTHOR_THE_DEFILED: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Balthor the Defiled",
    "ed4cc273-adc3-4f46-9743-134b552d1d56",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// JUD 62 — Cabal Therapy
pub(in crate::card::sets) static CABAL_THERAPY: CardRecord = CardRecord::new(
    CardSet::Judgment,
    "Cabal Therapy",
    "0a5df970-c6ba-4824-b8ba-67244aec2b82",
    "Ron Spencer",
    // A guess for one mana, and the same guess again later for a creature
    // that has already attacked.
    CardRules::new_sorcery(mana_cost!("{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Choose a nonland card name. Target player reveals their hand and discards all cards with that name.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::ChooseCardName {
                chooser: PlayerRefDef::EffectController,
                nonland_only: true,
                matched_in: PlayerRefDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                binding: ParentBinding,
                // Everything of the named card in the target's hand, revealed first so the
                // choice is answered honestly and then taken all at once.
                then: &EffectDef::Sequence(&[
                    EffectDef::RevealHand {
                        player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                    EffectDef::DiscardCards {
                        object: EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding)),
                    },
                ]),
            },
        ),
        AbilityDef::alternative_cast(
            mana_cost!("{0}"),
            AlternativeCastKindDef::Flashback,
            Some("Flashback—Sacrifice a creature."),
            EffectDef::None,
        )
        .with_alternative_additional_cost(&SpellAdditionalCostDef::sacrifice(
            ObjectPredicateDef::HasType(CardType::Creature),
            CostQuantityDef::Fixed(1),
        )),
    ]),
);

// JUD 63 — Cabal Trainee
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CABAL_TRAINEE: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Cabal Trainee",
    "d345d702-b205-4391-985a-6201e707f0ba",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// JUD 64 — Death Wish
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEATH_WISH: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Death Wish",
    "7bf134c9-a50d-4eff-a5a8-7cfe6a010080",
    "Jeff Easley",
    crate::card::CardRules::unsupported(),
);

// JUD 65 — Earsplitting Rats
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EARSPLITTING_RATS: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Earsplitting Rats",
    "5dad63b5-ced3-4150-ad84-1ca05a892840",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// JUD 66 — Filth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FILTH: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Filth",
    "37de06dc-c0c1-4edb-9732-2d16dbabfb31",
    "Thomas M. Baxa",
    crate::card::CardRules::unsupported(),
);

// JUD 67 — Grave Consequences
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRAVE_CONSEQUENCES: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Grave Consequences",
    "9ad5f9f2-282a-4ee0-a259-cc24404ddf6f",
    "Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// JUD 68 — Guiltfeeder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GUILTFEEDER: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Guiltfeeder",
    "e2e9af4e-bd02-4d91-898f-68d192446904",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// JUD 69 — Masked Gorgon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MASKED_GORGON: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Masked Gorgon",
    "0d62728b-b834-4fa8-aed5-6348033ee69c",
    "Matthew D. Wilson",
    crate::card::CardRules::unsupported(),
);

// JUD 70 — Morality Shift
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MORALITY_SHIFT: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Morality Shift",
    "b5c83e4d-ccc1-4ebf-9e74-2cc1ff9a7b07",
    "Jerry Tiritilli",
    crate::card::CardRules::unsupported(),
);

// JUD 71 — Rats' Feast
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RATS_FEAST: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Rats' Feast",
    "b243ce02-4fff-444c-acc4-e1a199621a53",
    "Bob Petillo",
    crate::card::CardRules::unsupported(),
);

// JUD 72 — Stitch Together
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STITCH_TOGETHER: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Stitch Together",
    "b43c6172-2400-4038-8b8b-c62f2fbfce39",
    "Arnie Swekel",
    crate::card::CardRules::unsupported(),
);

// JUD 73 — Sutured Ghoul
pub(in crate::card::sets) static SUTURED_GHOUL: CardRecord = CardRecord::new(
    CardSet::Judgment,
    "Sutured Ghoul",
    "f769536f-def1-40b8-863f-ba12c7cb0d87",
    "Carl Critchlow",
    // Seven mana for a creature the deck never pays for: it is reanimated
    // onto a graveyard the Druid has already filled, and eats all of it.
    CardRules::new_creature(mana_cost!("{4}{B}{B}{B}"), &["Zombie"], 0, 0).with_abilities(&[
        abilities::trample(),
        AbilityDef::as_enters(
            "As this creature enters, exile any number of creature cards from your graveyard.",
            ReplacementEffectDef::Choose(ReplacementChoiceDef::ExileMatchingFromGraveyard(
                ObjectPredicateDef::HasType(CardType::Creature),
            )),
        ),
        AbilityDef::static_ability(
            "Sutured Ghoul's power is equal to the total power of the exiled cards and its toughness is equal to their total toughness.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                // A body assembled from the graveyard, read live off the pile rather than
                // fixed as it entered: a characteristic-defining ability keeps answering.
                // A body assembled from the graveyard, read live off the pile rather than
                // fixed as it entered: a characteristic-defining ability keeps answering.
                // This sets the base rather than adding to it, which is what a printed
                // */* says.
                effect: AppliedEffectDef::Characteristic(
                    CharacteristicOperationDef::PowerToughness(PowerToughnessOperationDef::SetBase {
                        power: ValueDef::AggregateObjectValues(&ObjectValueAggregateDef {
                            objects: ObjectSetDef::LinkedExiles,
                            select: ObjectValueDef::Power,
                            operation: AggregateOperationDef::Sum,
                        }),
                        toughness: ValueDef::AggregateObjectValues(&ObjectValueAggregateDef {
                            objects: ObjectSetDef::LinkedExiles,
                            select: ObjectValueDef::Toughness,
                            operation: AggregateOperationDef::Sum,
                        }),
                    }),
                ),
            },
        ),
    ]),
);

// JUD 74 — Toxic Stench
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOXIC_STENCH: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Toxic Stench",
    "8c4d1f59-0dba-4b83-8386-ae564fb4b771",
    "Bradley Williams",
    crate::card::CardRules::unsupported(),
);

// JUD 75 — Treacherous Vampire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TREACHEROUS_VAMPIRE: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Treacherous Vampire",
    "a00562ae-b8b4-4f8f-8ea8-15d20568997d",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// JUD 76 — Treacherous Werewolf
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TREACHEROUS_WEREWOLF: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Treacherous Werewolf",
    "c9647726-302b-4fc4-91d7-2aa0bba0b653",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// JUD 77 — Anger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANGER: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Anger",
    "fa2920af-e6a1-4939-ab59-67af4430e5b8",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// JUD 78 — Arcane Teachings
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARCANE_TEACHINGS: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Arcane Teachings",
    "02c56677-c8e2-4500-9ee0-0b102496f454",
    "Mark Brill",
    crate::card::CardRules::unsupported(),
);

// JUD 79 — Barbarian Bully
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARBARIAN_BULLY: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Barbarian Bully",
    "e38f0f9f-ad7b-48da-89f3-b3e5346a3b71",
    "Mike Ploog",
    crate::card::CardRules::unsupported(),
);

// JUD 80 — Book Burning
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOOK_BURNING: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Book Burning",
    "bead678c-7b6a-4668-9919-623312e08a65",
    "Dave Dorman",
    crate::card::CardRules::unsupported(),
);

// JUD 81 — Breaking Point
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BREAKING_POINT: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Breaking Point",
    "765ec2c9-8ffe-488a-bebe-e5dd63825a8c",
    "Matthew D. Wilson",
    crate::card::CardRules::unsupported(),
);

// JUD 82 — Browbeat
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BROWBEAT: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Browbeat",
    "74f20068-f225-4055-be7a-5c4a18e33b0b",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// JUD 83 — Burning Wish
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BURNING_WISH: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Burning Wish",
    "1c9b692a-e832-4612-a6ec-93b52f6a0410",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// JUD 84 — Dwarven Bloodboiler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DWARVEN_BLOODBOILER: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Dwarven Bloodboiler",
    "9ac576b2-cda4-4aea-aa5c-933ec0457dda",
    "Arnie Swekel",
    crate::card::CardRules::unsupported(),
);

// JUD 85 — Dwarven Driller
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DWARVEN_DRILLER: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Dwarven Driller",
    "69d815d3-7e33-4de9-aa36-ff5ffb893d73",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// JUD 86 — Dwarven Scorcher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DWARVEN_SCORCHER: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Dwarven Scorcher",
    "099873b1-7181-4b9d-8ce1-8ec63c814afe",
    "Thomas M. Baxa",
    crate::card::CardRules::unsupported(),
);

// JUD 87 — Ember Shot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EMBER_SHOT: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Ember Shot",
    "6a9eb72b-9ae2-4b64-bbb9-187446b5fd2f",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// JUD 88 — Firecat Blitz
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FIRECAT_BLITZ: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Firecat Blitz",
    "d4e1d485-02d5-4a07-bcc6-d2a8d95763e8",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// JUD 89 — Flaring Pain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLARING_PAIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Flaring Pain",
    "eeb5c96a-1d16-459d-9968-ced9a8f1c520",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// JUD 90 — Fledgling Dragon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLEDGLING_DRAGON: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Fledgling Dragon",
    "315e5b4e-ae58-412a-be27-c4ef4899fbbd",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// JUD 91 — Goretusk Firebeast
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GORETUSK_FIREBEAST: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Goretusk Firebeast",
    "9919d2dd-d6a1-4d45-b6aa-227ed05d7051",
    "Keith Garletts",
    crate::card::CardRules::unsupported(),
);

// JUD 92 — Infectious Rage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INFECTIOUS_RAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Infectious Rage",
    "8569cdf7-e0e9-4733-98b5-56fac216fad3",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// JUD 93 — Jeska, Warrior Adept
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JESKA_WARRIOR_ADEPT: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Jeska, Warrior Adept",
    "1cf96a59-8b7d-4a5b-adfd-17eeedd95db5",
    "rk post",
    crate::card::CardRules::unsupported(),
);

// JUD 94 — Lava Dart
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAVA_DART: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Lava Dart",
    "865bb1d3-5b7d-40e9-87cc-96be9524a105",
    "Darrell Riche",
    crate::card::CardRules::unsupported(),
);

// JUD 95 — Liberated Dwarf
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIBERATED_DWARF: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Liberated Dwarf",
    "e2c07842-9b70-40b1-9b97-9a9279b7ebc4",
    "Greg Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// JUD 96 — Lightning Surge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIGHTNING_SURGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Lightning Surge",
    "0452d78d-eafc-4ccb-a478-d1f46bcefffe",
    "Ron Spears",
    crate::card::CardRules::unsupported(),
);

// JUD 97 — Planar Chaos
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PLANAR_CHAOS: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Planar Chaos",
    "5dae5e16-d2fc-488c-9c53-d35c377d6a00",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// JUD 98 — Shaman's Trance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHAMAN_S_TRANCE: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Shaman's Trance",
    "dfc33a4f-9ec2-4324-82a1-a4b9700572f2",
    "Greg Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// JUD 99 — Soulgorger Orgg
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOULGORGER_ORGG: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Soulgorger Orgg",
    "8aef55b8-5813-4aff-a35d-4b3cbd4a9ffb",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// JUD 100 — Spellgorger Barbarian
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPELLGORGER_BARBARIAN: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Spellgorger Barbarian",
    "043fcf80-dd20-4cc2-a0d5-4bb22b8b0789",
    "Mark Romanoski",
    crate::card::CardRules::unsupported(),
);

// JUD 101 — Swelter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SWELTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Swelter",
    "8f667c26-40f5-4ac5-87a4-cb03f70590a2",
    "Ben Thompson",
    crate::card::CardRules::unsupported(),
);

// JUD 102 — Swirling Sandstorm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SWIRLING_SANDSTORM: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Swirling Sandstorm",
    "4d757ec3-c15f-4d6e-8e18-36ebae985448",
    "Tony Szczudlo",
    crate::card::CardRules::unsupported(),
);

// JUD 103 — Worldgorger Dragon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WORLDGORGER_DRAGON: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Worldgorger Dragon",
    "99783a2b-a95a-457b-82d6-001933aee5ec",
    "Wayne England",
    crate::card::CardRules::unsupported(),
);

// JUD 104 — Anurid Barkripper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANURID_BARKRIPPER: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Anurid Barkripper",
    "33255dfd-f8a9-4a15-aac5-c53dc0257859",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// JUD 105 — Anurid Swarmsnapper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANURID_SWARMSNAPPER: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Anurid Swarmsnapper",
    "3636a9f8-d1d7-4452-8a53-788b514fdb97",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// JUD 106 — Battlefield Scrounger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BATTLEFIELD_SCROUNGER: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Battlefield Scrounger",
    "f5ac74bc-1198-4a9a-bcde-668cca08b274",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// JUD 107 — Brawn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRAWN: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Brawn",
    "e9f4b7fc-8793-43af-ade3-b23846a80457",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// JUD 108 — Canopy Claws
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CANOPY_CLAWS: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Canopy Claws",
    "c4530da2-fd04-40d9-ad69-5c2847921509",
    "Matthew Mitchell",
    crate::card::CardRules::unsupported(),
);

// JUD 109 — Centaur Rootcaster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CENTAUR_ROOTCASTER: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Centaur Rootcaster",
    "3f10dfd9-9889-4d9e-872a-07623dee6b6b",
    "Eric Peterson",
    crate::card::CardRules::unsupported(),
);

// JUD 110 — Crush of Wurms
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRUSH_OF_WURMS: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Crush of Wurms",
    "32a924b3-3bd6-43ad-acbd-1303dd670db4",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// JUD 111 — Elephant Guide
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELEPHANT_GUIDE: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Elephant Guide",
    "7d3a7226-f574-430e-9b8f-4e531a21540f",
    "Jim Nelson",
    crate::card::CardRules::unsupported(),
);

// JUD 112 — Epic Struggle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EPIC_STRUGGLE: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Epic Struggle",
    "0dc71f6f-f831-409e-aafd-3fa82a318e72",
    "Greg Hildebrandt & Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// JUD 113 — Erhnam Djinn (reprint)
const ERHNAM_DJINN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_arn::ERHNAM_DJINN,
    "5bcf61ba-37fd-4029-b299-add7cf9d70bc",
    "Greg Staples",
);

// JUD 114 — Exoskeletal Armor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXOSKELETAL_ARMOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Exoskeletal Armor",
    "e111fcab-17f7-4a02-b4eb-606ba18812b3",
    "Wayne England",
    crate::card::CardRules::unsupported(),
);

// JUD 115 — Folk Medicine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FOLK_MEDICINE: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Folk Medicine",
    "751bd716-5352-41d7-89fb-d5f100f6646b",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// JUD 116 — Forcemage Advocate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORCEMAGE_ADVOCATE: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Forcemage Advocate",
    "1ad217fe-9309-4c67-8a6a-cfb8b1ce91f1",
    "Darrell Riche",
    crate::card::CardRules::unsupported(),
);

// JUD 117 — Genesis
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GENESIS: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Genesis",
    "b43aee5e-b12e-43ea-9fae-16310acdc640",
    "Mark Zug",
    crate::card::CardRules::unsupported(),
);

// JUD 118 — Giant Warthog
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GIANT_WARTHOG: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Giant Warthog",
    "c402ef0e-51e7-4da6-a434-b99c5d435698",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// JUD 119 — Grizzly Fate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRIZZLY_FATE: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Grizzly Fate",
    "92d23432-6181-44c3-8d36-d7632a8a329f",
    "Dave Dorman",
    crate::card::CardRules::unsupported(),
);

// JUD 120 — Harvester Druid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HARVESTER_DRUID: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Harvester Druid",
    "97337e6e-1b3f-43a2-91f2-ca8f6c5dea88",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// JUD 121 — Ironshell Beetle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IRONSHELL_BEETLE: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Ironshell Beetle",
    "be9299cc-6f21-4185-ac63-2fd92e843faa",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// JUD 122 — Krosan Reclamation
pub(in crate::card::sets) static KROSAN_RECLAMATION: CardRecord = CardRecord::new(
    CardSet::Judgment,
    "Krosan Reclamation",
    "5b3c5144-7e15-46c6-b819-d729ecb30bb1",
    "Gary Ruddell",
    // Graveyard hate that answers a single card twice, which is what a
    // combo deck holding one Sutured Ghoul actually needs.
    CardRules::new_instant(mana_cost!("{1}{G}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target player shuffles up to two target cards from their graveyard into their library.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                // The graveyard the cards come out of belongs to the targeted player, which
                // is what makes the choice a resolution choice here rather than a second
                // target: the constraint is "from their graveyard", and choosing on
                // resolution states it exactly.
                candidates: ObjectSetDef::Query(ObjectQueryDef::owned_by(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Graveyard],
                    PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
                )),
                exclude: None,
                minimum: 0,
                maximum: 2,
                visibility: ChoiceVisibilityDef::Public,
                // The chosen shuffled back in. The shuffle follows the move so the
                // library the cards join is the one that gets randomized.
                then: &const {
                    EffectDef::Sequence(&const {
                        [
                            EffectDef::MoveToZone {
                                object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    ParentBinding,
                                )),
                                zone: ZoneKind::Library,
                                placement: ZonePlacement::Top,
                            },
                            EffectDef::ShuffleLibrary {
                                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            },
                        ]
                    })
                },
            }),
        ),
        AbilityDef::alternative_cast(
            mana_cost!("{1}{G}"),
            AlternativeCastKindDef::Flashback,
            Some("Flashback {1}{G}"),
            EffectDef::None,
        ),
    ]),
);

// JUD 123 — Krosan Wayfarer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KROSAN_WAYFARER: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Krosan Wayfarer",
    "5356e684-c2fc-465e-a16c-7300824d2a8d",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// JUD 124 — Living Wish
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIVING_WISH: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Living Wish",
    "2478a8d2-ca44-4c42-8d75-dd9cb1b59f61",
    "Eric Peterson",
    crate::card::CardRules::unsupported(),
);

// JUD 125 — Nantuko Tracer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NANTUKO_TRACER: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Nantuko Tracer",
    "16b93c93-5944-4289-bc5a-30b6e73b0dfd",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// JUD 126 — Nullmage Advocate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NULLMAGE_ADVOCATE: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Nullmage Advocate",
    "1c29991d-82f2-479d-95ca-5c88e9f3f219",
    "Darrell Riche",
    crate::card::CardRules::unsupported(),
);

// JUD 127 — Phantom Centaur
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHANTOM_CENTAUR: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Phantom Centaur",
    "c421c2f1-2137-41bd-9a89-74d8a76fb5c5",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// JUD 128 — Phantom Nantuko
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHANTOM_NANTUKO: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Phantom Nantuko",
    "66f8ca45-b60f-4bb9-9f7e-1b5e13478f22",
    "Wayne England",
    crate::card::CardRules::unsupported(),
);

// JUD 129 — Phantom Tiger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHANTOM_TIGER: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Phantom Tiger",
    "32839296-e583-4f71-aa44-dbe16408665e",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// JUD 130 — Seedtime
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEEDTIME: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Seedtime",
    "4ffd5c52-f260-400c-b088-8792282509a5",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// JUD 131 — Serene Sunset
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SERENE_SUNSET: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Serene Sunset",
    "0a6ded26-b748-406d-8740-9b8590be2bb1",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// JUD 132 — Sudden Strength
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUDDEN_STRENGTH: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Sudden Strength",
    "e3ca1108-ccf2-48ea-8ca7-986aa45d5fe8",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// JUD 133 — Sylvan Safekeeper
pub(in crate::card::sets) static SYLVAN_SAFEKEEPER: CardRecord = CardRecord::new(
    CardSet::Judgment,
    "Sylvan Safekeeper",
    "f1b8413f-c9fc-4cea-b416-a1fcf651b009",
    "Pete Venters",
    CardRules::new_creature(mana_cost!("{G}"), &["Human", "Wizard"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "Sacrifice a land: Target creature you control gains shroud until end of turn.",
            &[AbilityCostDef::SacrificePermanent {
                object: ObjectPredicateDef::HasType(CardType::Land),
                controller: PlayerRelation::You,
            }],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::shroud()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// JUD 134 — Thriss, Nantuko Primus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THRISS_NANTUKO_PRIMUS: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Thriss, Nantuko Primus",
    "ad9e647d-903f-4a77-a56c-cd5c0c2f12cf",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// JUD 135 — Tunneler Wurm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TUNNELER_WURM: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Tunneler Wurm",
    "c8e246c8-3b3f-47c4-8a1b-b5f2d36f0ca4",
    "Jeff Easley",
    crate::card::CardRules::unsupported(),
);

// JUD 136 — Venomous Vines
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VENOMOUS_VINES: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Venomous Vines",
    "db10359c-1ea8-4453-bc01-f638ad20a5ec",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// JUD 137 — Anurid Brushhopper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANURID_BRUSHHOPPER: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Anurid Brushhopper",
    "b09204c7-3e3d-484a-a4f7-da1b818e3884",
    "Arnie Swekel",
    crate::card::CardRules::unsupported(),
);

// JUD 138 — Hunting Grounds
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HUNTING_GROUNDS: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Hunting Grounds",
    "5b14a736-5223-457b-9d4e-f4a2d6ed9a8d",
    "Mark Brill",
    crate::card::CardRules::unsupported(),
);

// JUD 139 — Mirari's Wake
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIRARI_S_WAKE: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Mirari's Wake",
    "b5ddad46-5e2e-43c9-8c91-7aca6ca23562",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// JUD 140 — Phantom Nishoba
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHANTOM_NISHOBA: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Phantom Nishoba",
    "56ebc372-aabd-4174-a943-c7bf59e5028d",
    "Arnie Swekel",
    crate::card::CardRules::unsupported(),
);

// JUD 141 — Krosan Verge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KROSAN_VERGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Krosan Verge",
    "bae25abc-22c2-436d-9e08-f123543a0911",
    "Tony Szczudlo",
    crate::card::CardRules::unsupported(),
);

// JUD 142 — Nantuko Monastery
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NANTUKO_MONASTERY: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Nantuko Monastery",
    "cb870406-9d59-4493-9f81-0f4b84642001",
    "Rob Alexander",
    crate::card::CardRules::unsupported(),
);

// JUD 143 — Riftstone Portal
pub(in crate::card::sets) static RIFTSTONE_PORTAL: CardRecord = CardRecord::new(
    crate::card::CardSet::Judgment,
    "Riftstone Portal",
    "92ece630-e484-4221-911f-e32048894f23",
    "Don Hazeltine",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::static_ability(
            "As long as this card is in your graveyard, lands you control have \"{T}: Add {G} or {W}.\"",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Land),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&AbilityDef::activated_mana(
                    "{T}: Add {G} or {W}.",
                    &[AbilityCostDef::TapSource],
                    EffectDef::AddMana(AddManaEffectDef::choice(&[
                        ManaColor::Green,
                        ManaColor::White,
                    ])),
                )),
            },
        )
        .with_source_zones(&[ZoneKind::Graveyard]),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ANCESTOR_S_CHOSEN,
    &AVEN_WARCRAFT,
    &BATTLE_SCREECH,
    &BATTLEWISE_AVEN,
    &BENEVOLENT_BODYGUARD,
    &BORDER_PATROL,
    &CAGEMAIL,
    &CHASTISE,
    &COMMANDER_EESHA,
    &FUNERAL_PYRE,
    &GLORY,
    &GOLDEN_WISH,
    &LEAD_ASTRAY,
    &NOMAD_MYTHMAKER,
    &PHANTOM_FLOCK,
    &PHANTOM_NOMAD,
    &PRISMATIC_STRANDS,
    &PULSEMAGE_ADVOCATE,
    &RAY_OF_REVELATION,
    &SELFLESS_EXORCIST,
    &SHIELDMAGE_ADVOCATE,
    &SILVER_SERAPH,
    &SOLITARY_CONFINEMENT,
    &SOULCATCHERS_AERIE,
    &SPIRIT_CAIRN,
    &SPURNMAGE_ADVOCATE,
    &SUNTAIL_HAWK,
    &TEST_OF_ENDURANCE,
    &TRAINED_PRONGHORN,
    &UNQUESTIONED_AUTHORITY,
    &VALOR,
    &VIGILANT_SENTRY,
    &AVEN_FOGBRINGER,
    &CEPHALID_CONSTABLE,
    &CEPHALID_INKSHROUDER,
    &CUNNING_WISH,
    &DEFY_GRAVITY,
    &ENVELOP,
    &FLASH_OF_INSIGHT,
    &GRIP_OF_AMNESIA,
    &HAPLESS_RESEARCHER,
    &KEEP_WATCH,
    &LAQUATUS_S_DISDAIN,
    &LOST_IN_THOUGHT,
    &MENTAL_NOTE,
    &MIRROR_WALL,
    &MIST_OF_STAGNATION,
    &QUIET_SPECULATION,
    &SCALPELEXIS,
    &SPELLJACK,
    &TELEKINETIC_BONDS,
    &WEB_OF_INERTIA,
    &WONDER,
    &WORMFANG_BEHEMOTH,
    &WORMFANG_CRAB,
    &WORMFANG_DRAKE,
    &WORMFANG_MANTA,
    &WORMFANG_NEWT,
    &WORMFANG_TURTLE,
    &BALTHOR_THE_DEFILED,
    &CABAL_THERAPY,
    &CABAL_TRAINEE,
    &DEATH_WISH,
    &EARSPLITTING_RATS,
    &FILTH,
    &GRAVE_CONSEQUENCES,
    &GUILTFEEDER,
    &MASKED_GORGON,
    &MORALITY_SHIFT,
    &RATS_FEAST,
    &STITCH_TOGETHER,
    &SUTURED_GHOUL,
    &TOXIC_STENCH,
    &TREACHEROUS_VAMPIRE,
    &TREACHEROUS_WEREWOLF,
    &ANGER,
    &ARCANE_TEACHINGS,
    &BARBARIAN_BULLY,
    &BOOK_BURNING,
    &BREAKING_POINT,
    &BROWBEAT,
    &BURNING_WISH,
    &DWARVEN_BLOODBOILER,
    &DWARVEN_DRILLER,
    &DWARVEN_SCORCHER,
    &EMBER_SHOT,
    &FIRECAT_BLITZ,
    &FLARING_PAIN,
    &FLEDGLING_DRAGON,
    &GORETUSK_FIREBEAST,
    &INFECTIOUS_RAGE,
    &JESKA_WARRIOR_ADEPT,
    &LAVA_DART,
    &LIBERATED_DWARF,
    &LIGHTNING_SURGE,
    &PLANAR_CHAOS,
    &SHAMAN_S_TRANCE,
    &SOULGORGER_ORGG,
    &SPELLGORGER_BARBARIAN,
    &SWELTER,
    &SWIRLING_SANDSTORM,
    &WORLDGORGER_DRAGON,
    &ANURID_BARKRIPPER,
    &ANURID_SWARMSNAPPER,
    &BATTLEFIELD_SCROUNGER,
    &BRAWN,
    &CANOPY_CLAWS,
    &CENTAUR_ROOTCASTER,
    &CRUSH_OF_WURMS,
    &ELEPHANT_GUIDE,
    &EPIC_STRUGGLE,
    &EXOSKELETAL_ARMOR,
    &FOLK_MEDICINE,
    &FORCEMAGE_ADVOCATE,
    &GENESIS,
    &GIANT_WARTHOG,
    &GRIZZLY_FATE,
    &HARVESTER_DRUID,
    &IRONSHELL_BEETLE,
    &KROSAN_RECLAMATION,
    &KROSAN_WAYFARER,
    &LIVING_WISH,
    &NANTUKO_TRACER,
    &NULLMAGE_ADVOCATE,
    &PHANTOM_CENTAUR,
    &PHANTOM_NANTUKO,
    &PHANTOM_TIGER,
    &SEEDTIME,
    &SERENE_SUNSET,
    &SUDDEN_STRENGTH,
    &SYLVAN_SAFEKEEPER,
    &THRISS_NANTUKO_PRIMUS,
    &TUNNELER_WURM,
    &VENOMOUS_VINES,
    &ANURID_BRUSHHOPPER,
    &HUNTING_GROUNDS,
    &MIRARI_S_WAKE,
    &PHANTOM_NISHOBA,
    &KROSAN_VERGE,
    &NANTUKO_MONASTERY,
    &RIFTSTONE_PORTAL,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] =
    &[GUIDED_STRIKE_REPRINT, ERHNAM_DJINN_REPRINT];
