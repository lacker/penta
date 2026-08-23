//! Scars of Mirrodin cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedEffectDef, BasicLandType, CardArt, CardRules, CardSet, CardType, DiscardSelectionDef,
    EffectDef, EffectRecipientDef, KeywordAbility, ManaColor, ObjectPredicateDef, PlayerRelation,
    ResolvedEffectDurationDef, TriggerEventDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::{TargetIndex, mana_cost};

/// The fastland cycle: untapped while the board is still small, an expensive
/// tapped land after that. Every one of the ten prints this same clause, and
/// only the colour pair below it differs.
static FAST_LAND_ENTERS: AbilityDef = abilities::fast_land_enters(
    "This land enters tapped unless you control two or fewer other lands.",
);

static BLACKCLEAVE_CLIFFS_ABILITIES: [AbilityDef; 2] = [
    FAST_LAND_ENTERS,
    AbilityDef::activated_mana(
        "{T}: Add {B} or {R}.",
        &[AbilityCostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::choice(&[
            ManaColor::Black,
            ManaColor::Red,
        ])),
    ),
];

// SOM 1 — Abuna Acolyte
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ABUNA_ACOLYTE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9e17bbf7-00c0-46f2-9718-2762fd7388d3"),
    "Abuna Acolyte",
    crate::card::CardArt::new("9e17bbf7-00c0-46f2-9718-2762fd7388d3", "Igor Kieryluk"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 2 — Arrest (reprint)

// SOM 3 — Auriok Edgewright
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static AURIOK_EDGEWRIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0f76b18a-396b-41f5-b34b-ac232b7f316b"),
    "Auriok Edgewright",
    crate::card::CardArt::new("0f76b18a-396b-41f5-b34b-ac232b7f316b", "Mike Bierek"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 4 — Auriok Sunchaser
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static AURIOK_SUNCHASER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e274a8b3-2d92-43d9-a436-d3f6f619ca95"),
    "Auriok Sunchaser",
    crate::card::CardArt::new("e274a8b3-2d92-43d9-a436-d3f6f619ca95", "James Ryman"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 5 — Dispense Justice
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DISPENSE_JUSTICE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7b3330a1-98b6-4b09-9bca-6c7c89447ba2"),
    "Dispense Justice",
    crate::card::CardArt::new("7b3330a1-98b6-4b09-9bca-6c7c89447ba2", "Austin Hsu"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 6 — Elspeth Tirel
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ELSPETH_TIREL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ebe9116e-7b04-4f2a-aa67-89a42c6e1801"),
    "Elspeth Tirel",
    crate::card::CardArt::new("ebe9116e-7b04-4f2a-aa67-89a42c6e1801", "Michael Komarck"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 7 — Fulgent Distraction
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FULGENT_DISTRACTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c33a8cf1-e413-4633-b348-2ef594a945a5"),
    "Fulgent Distraction",
    crate::card::CardArt::new("c33a8cf1-e413-4633-b348-2ef594a945a5", "Nic Klein"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 8 — Ghalma's Warden
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GHALMA_S_WARDEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("efbf5ff1-6539-4116-ad4f-ce412ae20640"),
    "Ghalma's Warden",
    crate::card::CardArt::new("efbf5ff1-6539-4116-ad4f-ce412ae20640", "Mike Bierek"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 9 — Glimmerpoint Stag
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GLIMMERPOINT_STAG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5fb553f3-b1f6-47e7-94c1-8c09410c7163"),
    "Glimmerpoint Stag",
    crate::card::CardArt::new("5fb553f3-b1f6-47e7-94c1-8c09410c7163", "Ryan Pancoast"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 10 — Glint Hawk
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GLINT_HAWK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("284c4710-4183-4743-9c8b-515cc98cbbb8"),
    "Glint Hawk",
    crate::card::CardArt::new("284c4710-4183-4743-9c8b-515cc98cbbb8", "Dave Allsop"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 11 — Indomitable Archangel
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static INDOMITABLE_ARCHANGEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a50e72a2-1e94-43cf-a605-bf3bb456d12f"),
    "Indomitable Archangel",
    crate::card::CardArt::new("a50e72a2-1e94-43cf-a605-bf3bb456d12f", "Allen Williams"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 12 — Kemba, Kha Regent
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KEMBA_KHA_REGENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1964ca48-3260-4e2d-9014-984c1efc9a43"),
    "Kemba, Kha Regent",
    crate::card::CardArt::new("1964ca48-3260-4e2d-9014-984c1efc9a43", "Todd Lockwood"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 13 — Kemba's Skyguard
pub(in crate::card::sets) static KEMBA_S_SKYGUARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("66ce7c11-09bf-4884-893c-fc8bdbe776d4"),
    "Kemba's Skyguard",
    crate::card::CardArt::new("b9f20a74-7614-4bd9-ac08-0e098f98df0c", "Whit Brachna"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Cat", "Knight"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "When this creature enters, you gain 2 life.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
    ]),
);

// SOM 14 — Leonin Arbiter
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LEONIN_ARBITER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4b0453cd-62ab-41ba-8d9c-9d6d25dc9a56"),
    "Leonin Arbiter",
    crate::card::CardArt::new("4b0453cd-62ab-41ba-8d9c-9d6d25dc9a56", "Shelly Wan"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 15 — Loxodon Wayfarer
pub(in crate::card::sets) static LOXODON_WAYFARER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("356c5e6a-c0bd-43f7-bc84-a6ae8718a7a2"),
    "Loxodon Wayfarer",
    crate::card::CardArt::new("356c5e6a-c0bd-43f7-bc84-a6ae8718a7a2", "Steven Belledin"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Elephant", "Monk"], 1, 5),
);

// SOM 16 — Myrsmith
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MYRSMITH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("13429b63-085c-4c78-9ce3-247db5841b9d"),
    "Myrsmith",
    crate::card::CardArt::new("13429b63-085c-4c78-9ce3-247db5841b9d", "Eric Deschamps"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 17 — Razor Hippogriff
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RAZOR_HIPPOGRIFF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fc7ac3bf-eed2-417d-8b60-e8c84bfb98ab"),
    "Razor Hippogriff",
    crate::card::CardArt::new("fc7ac3bf-eed2-417d-8b60-e8c84bfb98ab", "David Rapoza"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 18 — Revoke Existence
pub(in crate::card::sets) static REVOKE_EXISTENCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("18ae62f9-361c-4849-b0af-2b08fc0421c8"),
    "Revoke Existence",
    crate::card::CardArt::new("18ae62f9-361c-4849-b0af-2b08fc0421c8", "Allen Williams"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_sorcery(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Exile target artifact or enchantment.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Enchantment),
            ]),
        )],
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Exile,
            placement: ZonePlacement::Top,
            counters: None,
            controller: None,
            arrival_effect: None,
            attachment: None,
        },
    )),
);

// SOM 19 — Salvage Scout
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SALVAGE_SCOUT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5909e77e-a930-4713-bca4-c6b265238c17"),
    "Salvage Scout",
    crate::card::CardArt::new("5909e77e-a930-4713-bca4-c6b265238c17", "Randis Albion"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 20 — Seize the Initiative
pub(in crate::card::sets) static SEIZE_THE_INITIATIVE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6d745f35-944a-4157-a351-baa06f67b725"),
    "Seize the Initiative",
    crate::card::CardArt::new("6d745f35-944a-4157-a351-baa06f67b725", "Steve Argyle"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +1/+1 and gains first strike until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                AppliedEffectDef::add_ability(&abilities::first_strike()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// SOM 21 — Soul Parry
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SOUL_PARRY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e241ea47-cbbe-4241-94f9-315cc7cfd79b"),
    "Soul Parry",
    crate::card::CardArt::new("e241ea47-cbbe-4241-94f9-315cc7cfd79b", "Igor Kieryluk"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 22 — Sunblast Angel
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SUNBLAST_ANGEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("32217d3b-8a44-40e3-a4fd-c849fdffc1e4"),
    "Sunblast Angel",
    crate::card::CardArt::new("32217d3b-8a44-40e3-a4fd-c849fdffc1e4", "Jason Chan"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 23 — Sunspear Shikari
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SUNSPEAR_SHIKARI: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("20ac29ef-02e1-4500-bb83-5987beeaa849"),
    "Sunspear Shikari",
    crate::card::CardArt::new("20ac29ef-02e1-4500-bb83-5987beeaa849", "Allen Williams"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 24 — Tempered Steel
pub(in crate::card::sets) static TEMPERED_STEEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6661b39d-505a-48f4-bc06-59084c6a3b0c"),
    "Tempered Steel",
    crate::card::CardArt::new("6661b39d-505a-48f4-bc06-59084c6a3b0c", "Wayne Reynolds"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_enchantment(mana_cost!("{1}{W}{W}")).with_ability(AbilityDef::static_ability(
        "Artifact creatures you control get +2/+2.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(2),
                ValueDef::Constant(2),
            ),
        },
    )),
);

// SOM 25 — True Conviction
pub(in crate::card::sets) static TRUE_CONVICTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("23a1d384-1b36-42d0-957f-48103f9cdbdd"),
    "True Conviction",
    crate::card::CardArt::new("23a1d384-1b36-42d0-957f-48103f9cdbdd", "Svetlin Velinov"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_enchantment(mana_cost!("{3}{W}{W}{W}")).with_ability(
        AbilityDef::static_ability(
            "Creatures you control have double strike and lifelink.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::add_ability(&abilities::double_strike()),
                    AppliedEffectDef::add_ability(&abilities::lifelink()),
                ]),
            },
        ),
    ),
);

// SOM 26 — Vigil for the Lost
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VIGIL_FOR_THE_LOST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4a87b48b-2ae9-4753-8719-62411f94ca87"),
    "Vigil for the Lost",
    crate::card::CardArt::new("4a87b48b-2ae9-4753-8719-62411f94ca87", "Igor Kieryluk"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 27 — Whitesun's Passage
pub(in crate::card::sets) static WHITESUN_S_PASSAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a74d1bf3-4630-4be0-af5f-590789d27a0c"),
    "Whitesun's Passage",
    crate::card::CardArt::new("a74d1bf3-4630-4be0-af5f-590789d27a0c", "John Avon"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell(
        "You gain 5 life.",
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(5),
        },
    )),
);

// SOM 28 — Argent Sphinx
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ARGENT_SPHINX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("280e75af-7e43-4c15-a8a8-bec7389c6c4e"),
    "Argent Sphinx",
    crate::card::CardArt::new("280e75af-7e43-4c15-a8a8-bec7389c6c4e", "Chris Rahn"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 29 — Bonds of Quicksilver
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BONDS_OF_QUICKSILVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c071dca0-fccb-48b8-b65a-74741b12e3f0"),
    "Bonds of Quicksilver",
    crate::card::CardArt::new("c071dca0-fccb-48b8-b65a-74741b12e3f0", "Steven Belledin"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 30 — Darkslick Drake
pub(in crate::card::sets) static DARKSLICK_DRAKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("234f4131-1e7f-4220-b46c-bb4a6713876e"),
    "Darkslick Drake",
    crate::card::CardArt::new("234f4131-1e7f-4220-b46c-bb4a6713876e", "Chippy"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Phyrexian", "Drake"], 2, 4).with_abilities(
        &[
            abilities::flying(),
            AbilityDef::triggered(
                "When this creature dies, draw a card.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
        ],
    ),
);

// SOM 31 — Disperse (reprint)

// SOM 32 — Dissipation Field
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DISSIPATION_FIELD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("247694c5-5813-4256-9fd8-478d4be52081"),
    "Dissipation Field",
    crate::card::CardArt::new("247694c5-5813-4256-9fd8-478d4be52081", "Matt Cavotta"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 33 — Grand Architect
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GRAND_ARCHITECT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c59599de-c781-4c26-a159-cbf0cd72d361"),
    "Grand Architect",
    crate::card::CardArt::new("c59599de-c781-4c26-a159-cbf0cd72d361", "Steven Belledin"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 34 — Halt Order
pub(in crate::card::sets) static HALT_ORDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7fed18af-7301-4d03-ba7c-e94f07f078b3"),
    "Halt Order",
    crate::card::CardArt::new("7fed18af-7301-4d03-ba7c-e94f07f078b3", "Izzy"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_instant(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target artifact spell. Draw a card.",
        &[AbilityTargetDef::exactly_one_spell(
            ObjectPredicateDef::HasType(CardType::Artifact),
        )],
        EffectDef::Sequence(&[
            EffectDef::Counter {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Graveyard,
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// SOM 35 — Inexorable Tide
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static INEXORABLE_TIDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8f41e281-fcbb-450b-8a67-7b072c55c6f0"),
    "Inexorable Tide",
    crate::card::CardArt::new("8f41e281-fcbb-450b-8a67-7b072c55c6f0", "Dave Kendall"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 36 — Lumengrid Drake
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LUMENGRID_DRAKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f44e9820-2209-40a2-bc4f-46b440c05e9d"),
    "Lumengrid Drake",
    crate::card::CardArt::new("f44e9820-2209-40a2-bc4f-46b440c05e9d", "Johann Bodin"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 37 — Neurok Invisimancer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NEUROK_INVISIMANCER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e88f78f4-77d8-4c3e-a5bf-a9dd902aaae1"),
    "Neurok Invisimancer",
    crate::card::CardArt::new("e88f78f4-77d8-4c3e-a5bf-a9dd902aaae1", "Izzy"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 38 — Plated Seastrider
pub(in crate::card::sets) static PLATED_SEASTRIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("97171611-c677-48a6-b081-98a27ecef979"),
    "Plated Seastrider",
    crate::card::CardArt::new("97171611-c677-48a6-b081-98a27ecef979", "Izzy"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_creature(mana_cost!("{U}{U}"), &["Beast"], 1, 4),
);

// SOM 39 — Quicksilver Gargantuan
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static QUICKSILVER_GARGANTUAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b83f5aea-80f2-4f3d-8508-9619413e0087"),
    "Quicksilver Gargantuan",
    crate::card::CardArt::new("b83f5aea-80f2-4f3d-8508-9619413e0087", "Steven Belledin"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 40 — Riddlesmith
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RIDDLESMITH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("08e25713-05ea-4eed-aa7f-5ca4e57a8152"),
    "Riddlesmith",
    crate::card::CardArt::new("08e25713-05ea-4eed-aa7f-5ca4e57a8152", "Eric Deschamps"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 41 — Scrapdiver Serpent
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SCRAPDIVER_SERPENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8c6b5db0-7d2c-4337-b1c4-9e1219f603c7"),
    "Scrapdiver Serpent",
    crate::card::CardArt::new("8c6b5db0-7d2c-4337-b1c4-9e1219f603c7", "Adrian Smith"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 42 — Screeching Silcaw
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SCREECHING_SILCAW: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1767355d-82a2-495e-ae95-d91984a9c62a"),
    "Screeching Silcaw",
    crate::card::CardArt::new("1767355d-82a2-495e-ae95-d91984a9c62a", "Mike Bierek"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 43 — Shape Anew
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHAPE_ANEW: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b3d5462e-f60c-4550-b29e-4d9f9cd72385"),
    "Shape Anew",
    crate::card::CardArt::new(
        "b3d5462e-f60c-4550-b29e-4d9f9cd72385",
        "Zoltan Boros & Gabor Szikszai",
    ),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 44 — Sky-Eel School
pub(in crate::card::sets) static SKY_EEL_SCHOOL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5cfc4db7-13b5-4c88-91f2-581c9792f1ff"),
    "Sky-Eel School",
    crate::card::CardArt::new("5cfc4db7-13b5-4c88-91f2-581c9792f1ff", "Daniel Ljunggren"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_creature(mana_cost!("{3}{U}{U}"), &["Fish"], 3, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "When this creature enters, draw a card, then discard a card.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
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
    ]),
);

// SOM 45 — Steady Progress
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STEADY_PROGRESS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6fe212ed-31cb-4f10-8ba7-e97af1d30d24"),
    "Steady Progress",
    crate::card::CardArt::new("6fe212ed-31cb-4f10-8ba7-e97af1d30d24", "Efrem Palacios"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 46 — Stoic Rebuttal
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STOIC_REBUTTAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f2805239-f30a-4eca-a10b-41673daaa287"),
    "Stoic Rebuttal",
    crate::card::CardArt::new("f2805239-f30a-4eca-a10b-41673daaa287", "Chris Rahn"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 47 — Thrummingbird
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static THRUMMINGBIRD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dc2dd336-e457-49a1-88ae-c35f0c846e99"),
    "Thrummingbird",
    crate::card::CardArt::new("dc2dd336-e457-49a1-88ae-c35f0c846e99", "Efrem Palacios"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 48 — Trinket Mage
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TRINKET_MAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4c5a41ab-1840-4abb-a8bb-f0b1e7d1b450"),
    "Trinket Mage",
    crate::card::CardArt::new("cb52e7ba-5340-44e1-9b63-775e1f387925", "Scott Chou"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 49 — Turn Aside
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TURN_ASIDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("56226f57-6ff0-430e-aba6-6b3dd51f8d3c"),
    "Turn Aside",
    crate::card::CardArt::new("56226f57-6ff0-430e-aba6-6b3dd51f8d3c", "Shelly Wan"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 50 — Twisted Image
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TWISTED_IMAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aa18c2c2-f1a1-469d-acd8-9d6e0605bcf9"),
    "Twisted Image",
    crate::card::CardArt::new("aa18c2c2-f1a1-469d-acd8-9d6e0605bcf9", "Izzy"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 51 — Vault Skyward
pub(in crate::card::sets) static VAULT_SKYWARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4e934192-2ea3-48fe-a2a9-42c2ee9b22f7"),
    "Vault Skyward",
    crate::card::CardArt::new("4e934192-2ea3-48fe-a2a9-42c2ee9b22f7", "Dan Murayama Scott"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gains flying until end of turn. Untap it.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::flying()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ]),
    )),
);

// SOM 52 — Vedalken Certarch
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VEDALKEN_CERTARCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ffbc2a26-32f1-4d9c-8ee7-74698f64dce0"),
    "Vedalken Certarch",
    crate::card::CardArt::new("ffbc2a26-32f1-4d9c-8ee7-74698f64dce0", "Karl Kopinski"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 53 — Volition Reins
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VOLITION_REINS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aa8fa025-56e6-4d24-a615-a51b6be937e9"),
    "Volition Reins",
    crate::card::CardArt::new("aa8fa025-56e6-4d24-a615-a51b6be937e9", "Svetlin Velinov"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 54 — Blackcleave Goblin
pub(in crate::card::sets) static BLACKCLEAVE_GOBLIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("95986875-59f5-414f-867f-94f30cefa5d6"),
    "Blackcleave Goblin",
    crate::card::CardArt::new("95986875-59f5-414f-867f-94f30cefa5d6", "Nils Hamm"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_creature(
        mana_cost!("{3}{B}"),
        &["Phyrexian", "Goblin", "Zombie"],
        2,
        1,
    )
    .with_abilities(&[abilities::haste(), abilities::infect()]),
);

// SOM 55 — Bleak Coven Vampires
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BLEAK_COVEN_VAMPIRES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9d3386e4-bbd6-4756-b29d-f55619e98d0d"),
    "Bleak Coven Vampires",
    crate::card::CardArt::new("9d3386e4-bbd6-4756-b29d-f55619e98d0d", "Randis Albion"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 56 — Blistergrub
pub(in crate::card::sets) static BLISTERGRUB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5431debc-0037-49ff-a38f-3fa2f9f5ee33"),
    "Blistergrub",
    crate::card::CardArt::new("5431debc-0037-49ff-a38f-3fa2f9f5ee33", "Daarken"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Phyrexian", "Horror"], 2, 2).with_abilities(
        &[
            abilities::landwalk(BasicLandType::Swamp),
            AbilityDef::triggered(
                "When this creature dies, each opponent loses 2 life.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Opponent,
                    amount: ValueDef::Constant(2),
                },
            ),
        ],
    ),
);

// SOM 57 — Carnifex Demon
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CARNIFEX_DEMON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c191dba2-659d-40e7-a558-c99ece872197"),
    "Carnifex Demon",
    crate::card::CardArt::new("c191dba2-659d-40e7-a558-c99ece872197", "Aleksi Briclot"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 58 — Contagious Nim
pub(in crate::card::sets) static CONTAGIOUS_NIM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e83a9dea-2aa1-48cd-afe2-f98057b95f6e"),
    "Contagious Nim",
    crate::card::CardArt::new("e83a9dea-2aa1-48cd-afe2-f98057b95f6e", "Efrem Palacios"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Phyrexian", "Zombie"], 2, 2)
        .with_abilities(&[abilities::infect()]),
);

// SOM 59 — Corrupted Harvester
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CORRUPTED_HARVESTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b54625ac-484f-4522-8048-38e01c545ac3"),
    "Corrupted Harvester",
    crate::card::CardArt::new("b54625ac-484f-4522-8048-38e01c545ac3", "Nils Hamm"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 60 — Dross Hopper
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DROSS_HOPPER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1a0656f6-a016-479a-a003-72e106e986b0"),
    "Dross Hopper",
    crate::card::CardArt::new("1a0656f6-a016-479a-a003-72e106e986b0", "Dave Allsop"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 61 — Exsanguinate
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static EXSANGUINATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0878b541-a730-49db-b062-5a01656e269d"),
    "Exsanguinate",
    crate::card::CardArt::new("0878b541-a730-49db-b062-5a01656e269d", "Carl Critchlow"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 62 — Flesh Allergy
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FLESH_ALLERGY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9c729525-b954-42dd-9877-f4360d99b961"),
    "Flesh Allergy",
    crate::card::CardArt::new("9c729525-b954-42dd-9877-f4360d99b961", "Vance Kovacs"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 63 — Fume Spitter
pub(in crate::card::sets) static FUME_SPITTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("58cd149b-ecf4-43ed-b6e5-98870953b4b8"),
    "Fume Spitter",
    crate::card::CardArt::new("58cd149b-ecf4-43ed-b6e5-98870953b4b8", "Nils Hamm"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_creature(mana_cost!("{B}"), &["Phyrexian", "Horror"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "Sacrifice this creature: Put a -1/-1 counter on target creature.",
            &[AbilityCostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: crate::card::CounterKind::MinusOneMinusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// SOM 64 — Geth, Lord of the Vault
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GETH_LORD_OF_THE_VAULT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fed31f2f-370d-4bbe-aa57-82249ed1b4d4"),
    "Geth, Lord of the Vault",
    crate::card::CardArt::new("fed31f2f-370d-4bbe-aa57-82249ed1b4d4", "Whit Brachna"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 65 — Grasp of Darkness
pub(in crate::card::sets) static GRASP_OF_DARKNESS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cda628ba-19f4-4e24-9500-cca295a992bb"),
    "Grasp of Darkness",
    crate::card::CardArt::new("cda628ba-19f4-4e24-9500-cca295a992bb", "Johann Bodin"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_instant(mana_cost!("{B}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets -4/-4 until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(-4),
                ValueDef::Constant(-4),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// SOM 66 — Hand of the Praetors
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HAND_OF_THE_PRAETORS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("94ca493e-f09b-4b11-bb47-0562dfc203ca"),
    "Hand of the Praetors",
    crate::card::CardArt::new("94ca493e-f09b-4b11-bb47-0562dfc203ca", "Izzy"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 67 — Ichor Rats
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ICHOR_RATS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2013aed6-7415-4bf0-a3bb-46d6beecbaff"),
    "Ichor Rats",
    crate::card::CardArt::new("2013aed6-7415-4bf0-a3bb-46d6beecbaff", "Matt Stewart"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 68 — Instill Infection
pub(in crate::card::sets) static INSTILL_INFECTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("82ef2567-f798-4447-9735-c7c0d88aba85"),
    "Instill Infection",
    crate::card::CardArt::new("82ef2567-f798-4447-9735-c7c0d88aba85", "Chris Rahn"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_instant(mana_cost!("{3}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Put a -1/-1 counter on target creature. Draw a card.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: crate::card::CounterKind::MinusOneMinusOne,
                amount: ValueDef::Constant(1),
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// SOM 69 — Memoricide
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MEMORICIDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c9d74bae-0b96-4a78-b805-a0b764d0716c"),
    "Memoricide",
    crate::card::CardArt::new("acc5b944-a9fe-4a64-bf11-51817a26f22b", "James Ryman"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 70 — Moriok Reaver
pub(in crate::card::sets) static MORIOK_REAVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e2a0410f-95c5-49bf-856d-dea796c96e3b"),
    "Moriok Reaver",
    crate::card::CardArt::new("e2a0410f-95c5-49bf-856d-dea796c96e3b", "Marc Simonetti"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Human", "Warrior"], 3, 2),
);

// SOM 71 — Necrogen Scudder
pub(in crate::card::sets) static NECROGEN_SCUDDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7d69c045-d705-478b-9e8f-272a24737225"),
    "Necrogen Scudder",
    crate::card::CardArt::new("7d69c045-d705-478b-9e8f-272a24737225", "Raymond Swanland"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Phyrexian", "Horror"], 3, 3).with_abilities(
        &[
            abilities::flying(),
            AbilityDef::triggered(
                "When this creature enters, you lose 3 life.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(3),
                },
            ),
        ],
    ),
);

// SOM 72 — Necrotic Ooze
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NECROTIC_OOZE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8af2c79f-a151-4628-90fe-c0ff7ccd9c2c"),
    "Necrotic Ooze",
    crate::card::CardArt::new("8af2c79f-a151-4628-90fe-c0ff7ccd9c2c", "James Ryman"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 73 — Painful Quandary
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PAINFUL_QUANDARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fecf3dae-1a0c-4cf3-b9bd-ec2ad6acaa1b"),
    "Painful Quandary",
    crate::card::CardArt::new("fecf3dae-1a0c-4cf3-b9bd-ec2ad6acaa1b", "Whit Brachna"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 74 — Painsmith
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PAINSMITH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b8e531ab-29ed-4e54-ae9c-681a220666ad"),
    "Painsmith",
    crate::card::CardArt::new("b8e531ab-29ed-4e54-ae9c-681a220666ad", "Eric Deschamps"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 75 — Plague Stinger
pub(in crate::card::sets) static PLAGUE_STINGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aae856bc-f65f-42ba-9344-1a30b356c041"),
    "Plague Stinger",
    crate::card::CardArt::new("aae856bc-f65f-42ba-9344-1a30b356c041", "Ryan Pancoast"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_creature(
        mana_cost!("{1}{B}"),
        &["Phyrexian", "Insect", "Horror"],
        1,
        1,
    )
    .with_abilities(&[abilities::flying(), abilities::infect()]),
);

// SOM 76 — Psychic Miasma
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PSYCHIC_MIASMA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fd9c3267-7988-416c-85a4-0e314e42ddb9"),
    "Psychic Miasma",
    crate::card::CardArt::new("fd9c3267-7988-416c-85a4-0e314e42ddb9", "Svetlin Velinov"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 77 — Relic Putrescence
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RELIC_PUTRESCENCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ca940b4e-6f5e-4492-b6e0-dbf619eddadd"),
    "Relic Putrescence",
    crate::card::CardArt::new("ca940b4e-6f5e-4492-b6e0-dbf619eddadd", "Allen Williams"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 78 — Skinrender
pub(in crate::card::sets) static SKINRENDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("be358357-2abe-4ead-bb18-76cad8274489"),
    "Skinrender",
    crate::card::CardArt::new("be358357-2abe-4ead-bb18-76cad8274489", "David Rapoza"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Phyrexian", "Zombie"], 3, 3).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature enters, put three -1/-1 counters on target creature.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: crate::card::CounterKind::MinusOneMinusOne,
                amount: ValueDef::Constant(3),
            },
        ),
    ),
);

// SOM 79 — Skithiryx, the Blight Dragon
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SKITHIRYX_THE_BLIGHT_DRAGON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c930c9cc-1b64-4f36-afe2-6bf120a74ce2"),
    "Skithiryx, the Blight Dragon",
    crate::card::CardArt::new("c930c9cc-1b64-4f36-afe2-6bf120a74ce2", "Chippy"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 80 — Tainted Strike
pub(in crate::card::sets) static TAINTED_STRIKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d0f82007-99f6-4c6c-8182-ee631c33531f"),
    "Tainted Strike",
    crate::card::CardArt::new("d0f82007-99f6-4c6c-8182-ee631c33531f", "James Ryman"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +1/+0 and gains infect until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                AppliedEffectDef::add_ability(&abilities::infect()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// SOM 81 — Arc Trail
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ARC_TRAIL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("445e3a0a-29a7-4dc0-80fe-569b9e751db3"),
    "Arc Trail",
    crate::card::CardArt::new("445e3a0a-29a7-4dc0-80fe-569b9e751db3", "Marc Simonetti"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 82 — Assault Strobe
pub(in crate::card::sets) static ASSAULT_STROBE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9b505c78-5dbd-483d-92bb-5144060e962f"),
    "Assault Strobe",
    crate::card::CardArt::new("9b505c78-5dbd-483d-92bb-5144060e962f", "Kev Walker"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_sorcery(mana_cost!("{R}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gains double strike until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::add_ability(&abilities::double_strike()),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// SOM 83 — Barrage Ogre
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BARRAGE_OGRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e02c6f71-2448-47e1-9133-7af6a4d4577a"),
    "Barrage Ogre",
    crate::card::CardArt::new("e02c6f71-2448-47e1-9133-7af6a4d4577a", "David Rapoza"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 84 — Blade-Tribe Berserkers
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BLADE_TRIBE_BERSERKERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("acd124bb-1ed1-469c-8527-d7261ea720b9"),
    "Blade-Tribe Berserkers",
    crate::card::CardArt::new("acd124bb-1ed1-469c-8527-d7261ea720b9", "Kev Walker"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 85 — Bloodshot Trainee
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BLOODSHOT_TRAINEE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b930b146-d132-454f-b35d-4a247c14c054"),
    "Bloodshot Trainee",
    crate::card::CardArt::new("c2d5ce81-6cca-4990-a515-34ac44cae039", "Matt Stewart"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 86 — Cerebral Eruption
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CEREBRAL_ERUPTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("77161159-ee2c-485d-8674-d8590ccc62e1"),
    "Cerebral Eruption",
    crate::card::CardArt::new("77161159-ee2c-485d-8674-d8590ccc62e1", "Kev Walker"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 87 — Embersmith
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static EMBERSMITH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ee86cfc8-9faa-474c-90a9-5405f3f6037c"),
    "Embersmith",
    crate::card::CardArt::new("ee86cfc8-9faa-474c-90a9-5405f3f6037c", "Eric Deschamps"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 88 — Ferrovore
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FERROVORE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8dcc7170-38d9-4b9e-a5f9-73ac1208c439"),
    "Ferrovore",
    crate::card::CardArt::new("8dcc7170-38d9-4b9e-a5f9-73ac1208c439", "Austin Hsu"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 89 — Flameborn Hellion
pub(in crate::card::sets) static FLAMEBORN_HELLION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("84e0e5f5-b51a-4386-827b-c0eb8c877efb"),
    "Flameborn Hellion",
    crate::card::CardArt::new("84e0e5f5-b51a-4386-827b-c0eb8c877efb", "Aleksi Briclot"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_creature(mana_cost!("{5}{R}"), &["Hellion"], 5, 4).with_abilities(&[
        abilities::haste(),
        abilities::attacks_each_combat_if_able("This creature attacks each combat if able."),
    ]),
);

// SOM 90 — Furnace Celebration
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FURNACE_CELEBRATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a21fa7cb-a8ac-4312-80d4-82ee87650a55"),
    "Furnace Celebration",
    crate::card::CardArt::new("a21fa7cb-a8ac-4312-80d4-82ee87650a55", "Svetlin Velinov"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 91 — Galvanic Blast
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GALVANIC_BLAST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f5881bbc-8600-464d-9dcd-5a7780918d1d"),
    "Galvanic Blast",
    crate::card::CardArt::new("f5881bbc-8600-464d-9dcd-5a7780918d1d", "Marc Simonetti"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 92 — Goblin Gaveleer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_GAVELEER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6f65af25-8007-415d-a3fa-7736f6118284"),
    "Goblin Gaveleer",
    crate::card::CardArt::new("6f65af25-8007-415d-a3fa-7736f6118284", "Svetlin Velinov"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 93 — Hoard-Smelter Dragon
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HOARD_SMELTER_DRAGON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fcdd1d89-719d-4552-aeae-499c09b2ec6e"),
    "Hoard-Smelter Dragon",
    crate::card::CardArt::new("fcdd1d89-719d-4552-aeae-499c09b2ec6e", "Eric Deschamps"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 94 — Koth of the Hammer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KOTH_OF_THE_HAMMER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("af8b9c79-a161-4d7d-944d-82a44a5f2ab9"),
    "Koth of the Hammer",
    crate::card::CardArt::new("af8b9c79-a161-4d7d-944d-82a44a5f2ab9", "Jason Chan"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 95 — Kuldotha Phoenix
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KULDOTHA_PHOENIX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6bb79b56-81f1-417f-b5ad-030ad29f904b"),
    "Kuldotha Phoenix",
    crate::card::CardArt::new("6bb79b56-81f1-417f-b5ad-030ad29f904b", "Mike Bierek"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 96 — Kuldotha Rebirth
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KULDOTHA_REBIRTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7ee07266-a95d-4cd8-9863-1664922e9490"),
    "Kuldotha Rebirth",
    crate::card::CardArt::new("7ee07266-a95d-4cd8-9863-1664922e9490", "Goran Josic"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 97 — Melt Terrain
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MELT_TERRAIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1d94a1d1-6d24-46e1-9568-42e1a810ad31"),
    "Melt Terrain",
    crate::card::CardArt::new("1d94a1d1-6d24-46e1-9568-42e1a810ad31", "John Avon"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 98 — Molten Psyche
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MOLTEN_PSYCHE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("57e2382d-1f27-40d1-b809-c188c19ebc72"),
    "Molten Psyche",
    crate::card::CardArt::new("57e2382d-1f27-40d1-b809-c188c19ebc72", "Ryan Yee"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 99 — Ogre Geargrabber
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static OGRE_GEARGRABBER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f0f6e2c3-0e0d-47ff-9d92-afc86a8c8aac"),
    "Ogre Geargrabber",
    crate::card::CardArt::new("f0f6e2c3-0e0d-47ff-9d92-afc86a8c8aac", "David Rapoza"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 100 — Oxidda Daredevil
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static OXIDDA_DAREDEVIL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4b0bde7b-dc2d-45d2-b124-69b4b51ef3d9"),
    "Oxidda Daredevil",
    crate::card::CardArt::new("4b0bde7b-dc2d-45d2-b124-69b4b51ef3d9", "Pete Venters"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 101 — Oxidda Scrapmelter
pub(in crate::card::sets) static OXIDDA_SCRAPMELTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c64fe85b-e471-489a-8c38-2357da1c7969"),
    "Oxidda Scrapmelter",
    crate::card::CardArt::new("c64fe85b-e471-489a-8c38-2357da1c7969", "Igor Kieryluk"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Beast"], 3, 3).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature enters, destroy target artifact.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Artifact),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            },
        ),
    ),
);

// SOM 102 — Scoria Elemental
pub(in crate::card::sets) static SCORIA_ELEMENTAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ca4d9198-52a7-4dfe-8f7f-4fa6e19a2479"),
    "Scoria Elemental",
    crate::card::CardArt::new("ca4d9198-52a7-4dfe-8f7f-4fa6e19a2479", "Karl Kopinski"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Elemental"], 6, 1),
);

// SOM 103 — Shatter (reprint)

// SOM 104 — Spikeshot Elder
pub(in crate::card::sets) static SPIKESHOT_ELDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fad5621d-eb77-4b4a-80e7-1bfa75a6fcfb"),
    "Spikeshot Elder",
    crate::card::CardArt::new("fad5621d-eb77-4b4a-80e7-1bfa75a6fcfb", "Izzy"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin", "Shaman"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{1}{R}{R}: This creature deals damage equal to its power to any target.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{R}{R}"))],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::SourcePower,
            },
        ),
    ),
);

// SOM 105 — Tunnel Ignus
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TUNNEL_IGNUS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c3016e6b-32b2-4fa7-91c0-ec8fbe345760"),
    "Tunnel Ignus",
    crate::card::CardArt::new("c3016e6b-32b2-4fa7-91c0-ec8fbe345760", "Scott Chou"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 106 — Turn to Slag (reprint)

// SOM 107 — Vulshok Heartstoker
pub(in crate::card::sets) static VULSHOK_HEARTSTOKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9d3152bc-5c59-4e98-95de-a51de05a3c98"),
    "Vulshok Heartstoker",
    crate::card::CardArt::new("9d3152bc-5c59-4e98-95de-a51de05a3c98", "Shelly Wan"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Shaman"], 2, 2).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature enters, target creature gets +2/+0 until end of turn.",
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
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// SOM 108 — Acid Web Spider
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ACID_WEB_SPIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("968a25a5-9ec1-47fa-bf1f-e65eb75fdb00"),
    "Acid Web Spider",
    crate::card::CardArt::new("968a25a5-9ec1-47fa-bf1f-e65eb75fdb00", "Austin Hsu"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 109 — Alpha Tyrranax
pub(in crate::card::sets) static ALPHA_TYRRANAX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4a2e5279-f28c-4a78-9f8a-16c9f72f8d38"),
    "Alpha Tyrranax",
    crate::card::CardArt::new("4a2e5279-f28c-4a78-9f8a-16c9f72f8d38", "Dave Kendall"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Dinosaur", "Beast"], 6, 5),
);

// SOM 110 — Asceticism
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ASCETICISM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ec2b56b0-126c-411b-8c43-b690fc8c194b"),
    "Asceticism",
    crate::card::CardArt::new("ec2b56b0-126c-411b-8c43-b690fc8c194b", "Daarken"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 111 — Bellowing Tanglewurm
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BELLOWING_TANGLEWURM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("44eb3e3a-60ee-4293-a321-daa452d4c70d"),
    "Bellowing Tanglewurm",
    crate::card::CardArt::new("44eb3e3a-60ee-4293-a321-daa452d4c70d", "jD"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 112 — Blight Mamba
pub(in crate::card::sets) static BLIGHT_MAMBA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cf9b3335-565c-406d-bd94-f36974602552"),
    "Blight Mamba",
    crate::card::CardArt::new("cf9b3335-565c-406d-bd94-f36974602552", "Drew Baker"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Phyrexian", "Snake"], 1, 1).with_abilities(&[
        abilities::infect(),
        abilities::regenerate_self(
            "{1}{G}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{G}"))],
        ),
    ]),
);

// SOM 113 — Blunt the Assault
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BLUNT_THE_ASSAULT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6ecff12a-37d5-4a7b-b615-4c5e3bd950bb"),
    "Blunt the Assault",
    crate::card::CardArt::new("6ecff12a-37d5-4a7b-b615-4c5e3bd950bb", "Matt Stewart"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 114 — Carapace Forger
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CARAPACE_FORGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e9948e4c-d583-4fde-a305-df926cf00199"),
    "Carapace Forger",
    crate::card::CardArt::new("e9948e4c-d583-4fde-a305-df926cf00199", "Matt Cavotta"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 115 — Carrion Call
pub(in crate::card::sets) static CARRION_CALL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bc3c1a8e-3bdb-42cf-9442-5de7e4670d66"),
    "Carrion Call",
    crate::card::CardArt::new("bc3c1a8e-3bdb-42cf-9442-5de7e4670d66", "Adrian Smith"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_instant(mana_cost!("{3}{G}")).with_ability(AbilityDef::spell(
        "Create two 1/1 green Phyrexian Insect creature tokens with infect.",
        EffectDef::create_creature_token(&["Phyrexian", "Insect"], &[ManaColor::Green], 1, 1)
            .with_abilities(&[abilities::infect()])
            .with_amount(2),
    )),
);

// SOM 116 — Copperhorn Scout
pub(in crate::card::sets) static COPPERHORN_SCOUT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4ee7f99e-7324-4d16-b163-8f1b2edb7b89"),
    "Copperhorn Scout",
    crate::card::CardArt::new("4ee7f99e-7324-4d16-b163-8f1b2edb7b89", "Shelly Wan"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_creature(mana_cost!("{G}"), &["Elf", "Scout"], 1, 1).with_ability(
        AbilityDef::triggered(
            "Whenever this creature attacks, untap each other creature you control.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::Untap {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
            },
        ),
    ),
);

// SOM 117 — Cystbearer
pub(in crate::card::sets) static CYSTBEARER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b6c10302-f0b3-4076-ae5c-a8c8c09a7d41"),
    "Cystbearer",
    crate::card::CardArt::new("b6c10302-f0b3-4076-ae5c-a8c8c09a7d41", "Kev Walker"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Phyrexian", "Beast"], 2, 3)
        .with_abilities(&[abilities::infect()]),
);

// SOM 118 — Engulfing Slagwurm
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ENGULFING_SLAGWURM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8aeabc4a-7b4f-4e3d-bcc7-423bb703563a"),
    "Engulfing Slagwurm",
    crate::card::CardArt::new("8aeabc4a-7b4f-4e3d-bcc7-423bb703563a", "Jaime Jones"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 119 — Ezuri, Renegade Leader
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static EZURI_RENEGADE_LEADER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e9544132-bbb5-4ec4-af82-dad56e5091af"),
    "Ezuri, Renegade Leader",
    crate::card::CardArt::new("e9544132-bbb5-4ec4-af82-dad56e5091af", "Karl Kopinski"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 120 — Ezuri's Archers
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static EZURI_S_ARCHERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("32cc93af-d9a0-4ed8-8c22-686d005ea77e"),
    "Ezuri's Archers",
    crate::card::CardArt::new("32cc93af-d9a0-4ed8-8c22-686d005ea77e", "Shelly Wan"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 121 — Ezuri's Brigade
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static EZURI_S_BRIGADE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("079a6b44-3492-4484-aed1-5cd2449e702d"),
    "Ezuri's Brigade",
    crate::card::CardArt::new("079a6b44-3492-4484-aed1-5cd2449e702d", "Nic Klein"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 122 — Genesis Wave
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GENESIS_WAVE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c920236f-c3d7-421c-b021-103996da790e"),
    "Genesis Wave",
    crate::card::CardArt::new("c920236f-c3d7-421c-b021-103996da790e", "James Paick"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 123 — Liege of the Tangle
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LIEGE_OF_THE_TANGLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f7fc5b67-f521-4ba4-a10f-103e8b6af688"),
    "Liege of the Tangle",
    crate::card::CardArt::new("f7fc5b67-f521-4ba4-a10f-103e8b6af688", "Jason Chan"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 124 — Lifesmith
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LIFESMITH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("28e5dcac-0d59-4bcc-8a0e-036cc23065b5"),
    "Lifesmith",
    crate::card::CardArt::new("28e5dcac-0d59-4bcc-8a0e-036cc23065b5", "Eric Deschamps"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 125 — Molder Beast
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MOLDER_BEAST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d1340a63-f549-440b-aad3-14247113896a"),
    "Molder Beast",
    crate::card::CardArt::new("d1340a63-f549-440b-aad3-14247113896a", "Randis Albion"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 126 — Putrefax
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PUTREFAX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b2b2c3f9-a831-4fd2-80e8-b67b0df3e98b"),
    "Putrefax",
    crate::card::CardArt::new("b2b2c3f9-a831-4fd2-80e8-b67b0df3e98b", "Steven Belledin"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 127 — Slice in Twain
pub(in crate::card::sets) static SLICE_IN_TWAIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("de9c572a-6dc0-432f-92e9-c52fb0efddb5"),
    "Slice in Twain",
    crate::card::CardArt::new("de9c572a-6dc0-432f-92e9-c52fb0efddb5", "Efrem Palacios"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_instant(mana_cost!("{2}{G}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target artifact or enchantment. Draw a card.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Enchantment),
            ]),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// SOM 128 — Tangle Angler
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TANGLE_ANGLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b678bd68-e866-4081-95f9-2bd93a84d400"),
    "Tangle Angler",
    crate::card::CardArt::new("b678bd68-e866-4081-95f9-2bd93a84d400", "Igor Kieryluk"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 129 — Tel-Jilad Defiance
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TEL_JILAD_DEFIANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ef01d3f6-c172-43fb-bc65-ff12567111da"),
    "Tel-Jilad Defiance",
    crate::card::CardArt::new("ef01d3f6-c172-43fb-bc65-ff12567111da", "Goran Josic"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 130 — Tel-Jilad Fallen
pub(in crate::card::sets) static TEL_JILAD_FALLEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("643891b6-23d0-4734-81e0-b315d2d58f50"),
    "Tel-Jilad Fallen",
    crate::card::CardArt::new("643891b6-23d0-4734-81e0-b315d2d58f50", "James Ryman"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_creature(
        mana_cost!("{2}{G}{G}"),
        &["Phyrexian", "Elf", "Warrior"],
        3,
        1,
    )
    .with_abilities(&[
        AbilityDef::keyword(
            "Protection from artifacts",
            KeywordAbility::ProtectionFrom(&ObjectPredicateDef::HasType(CardType::Artifact)),
        ),
        abilities::infect(),
    ]),
);

// SOM 131 — Untamed Might
pub(in crate::card::sets) static UNTAMED_MIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("17979f0e-bd39-449f-b4ed-9156c229223b"),
    "Untamed Might",
    crate::card::CardArt::new("17979f0e-bd39-449f-b4ed-9156c229223b", "Erica Yang"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_instant(mana_cost!("{X}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +X/+X until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::modify_power_toughness(ValueDef::ChosenX, ValueDef::ChosenX),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// SOM 132 — Viridian Revel
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VIRIDIAN_REVEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2d7f565e-0fb8-40c8-9540-213d35af846a"),
    "Viridian Revel",
    crate::card::CardArt::new("2d7f565e-0fb8-40c8-9540-213d35af846a", "rk post"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 133 — Wing Puncture
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WING_PUNCTURE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("05a5188b-9ae3-4ca0-8289-b8a266a9073b"),
    "Wing Puncture",
    crate::card::CardArt::new("05a5188b-9ae3-4ca0-8289-b8a266a9073b", "jD"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 134 — Withstand Death
pub(in crate::card::sets) static WITHSTAND_DEATH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b059cca0-2373-428b-a3a6-c8be5523c96f"),
    "Withstand Death",
    crate::card::CardArt::new("b059cca0-2373-428b-a3a6-c8be5523c96f", "Tomasz Jedruszek"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_instant(mana_cost!("{G}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gains indestructible until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// SOM 135 — Venser, the Sojourner
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VENSER_THE_SOJOURNER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3d48d62e-5c1f-464c-aa81-8a5d2690f48e"),
    "Venser, the Sojourner",
    crate::card::CardArt::new("3d48d62e-5c1f-464c-aa81-8a5d2690f48e", "Eric Deschamps"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 136 — Accorder's Shield (reprint)

// SOM 137 — Argentum Armor
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ARGENTUM_ARMOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1283c05a-905b-421a-9096-e86b9c807aaf"),
    "Argentum Armor",
    crate::card::CardArt::new("1283c05a-905b-421a-9096-e86b9c807aaf", "Matt Cavotta"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 138 — Auriok Replica
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static AURIOK_REPLICA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("02745a0a-9872-4c30-a25d-61695c5fa9cc"),
    "Auriok Replica",
    crate::card::CardArt::new(
        "02745a0a-9872-4c30-a25d-61695c5fa9cc",
        "Zoltan Boros & Gabor Szikszai",
    ),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 139 — Barbed Battlegear
pub(in crate::card::sets) static BARBED_BATTLEGEAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("03b80b2f-8d07-4ad3-9b20-4ba0fe9f37a2"),
    "Barbed Battlegear",
    crate::card::CardArt::new("03b80b2f-8d07-4ad3-9b20-4ba0fe9f37a2", "Steve Argyle"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_artifact(mana_cost!("{3}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +4/-1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(4),
                        ValueDef::Constant(-1),
                    ),
                },
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{2}"))], "Equip {2}"),
        ]),
);

// SOM 140 — Bladed Pinions
pub(in crate::card::sets) static BLADED_PINIONS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bf479c90-c791-4152-a8e6-fd3123f698df"),
    "Bladed Pinions",
    crate::card::CardArt::new("bf479c90-c791-4152-a8e6-fd3123f698df", "Steve Argyle"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature has flying and first strike.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_ability(&abilities::flying()),
                        AppliedEffectDef::add_ability(&abilities::first_strike()),
                    ]),
                },
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{2}"))], "Equip {2}"),
        ]),
);

// SOM 141 — Chimeric Mass
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CHIMERIC_MASS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bcdb3af4-eaba-47b0-b242-dafa25ff0969"),
    "Chimeric Mass",
    crate::card::CardArt::new("bcdb3af4-eaba-47b0-b242-dafa25ff0969", "David Palumbo"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 142 — Chrome Steed
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CHROME_STEED: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ce881675-690f-4d4c-a951-ab8302e904ab"),
    "Chrome Steed",
    crate::card::CardArt::new(
        "ce881675-690f-4d4c-a951-ab8302e904ab",
        "Jana Schirmer & Johannes Voss",
    ),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 143 — Clone Shell
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CLONE_SHELL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cc386c6c-c27e-4673-96eb-1d004fd71993"),
    "Clone Shell",
    crate::card::CardArt::new("cc386c6c-c27e-4673-96eb-1d004fd71993", "Volkan Baǵa"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 144 — Contagion Clasp
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CONTAGION_CLASP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d134d21d-4a18-4fbe-a6c2-428e99a86466"),
    "Contagion Clasp",
    crate::card::CardArt::new("7fafcefa-d33c-4d73-b3b7-2930f28b845e", "Anthony Palumbo"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 145 — Contagion Engine
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CONTAGION_ENGINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dce72636-08e4-484e-ad81-4d1597a31ffb"),
    "Contagion Engine",
    crate::card::CardArt::new("dce72636-08e4-484e-ad81-4d1597a31ffb", "Daarken"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 146 — Copper Myr
pub(in crate::card::sets) static COPPER_MYR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a52b2dc4-4fb3-4ddf-bdb6-c63e8c8efc09"),
    "Copper Myr",
    crate::card::CardArt::new("323efe27-da58-4207-9c0c-dba5031bfa04", "Alan Pollack"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Myr"], 1, 1).with_ability(
        AbilityDef::activated_mana(
            "{T}: Add {G}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
        ),
    ),
);

// SOM 147 — Corpse Cur
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CORPSE_CUR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9c6e19a1-b9ea-4724-96d6-63c4b4967257"),
    "Corpse Cur",
    crate::card::CardArt::new("9c6e19a1-b9ea-4724-96d6-63c4b4967257", "Pete Venters"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 148 — Culling Dais
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CULLING_DAIS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ba7665c7-c211-45d7-bde1-f7952548025f"),
    "Culling Dais",
    crate::card::CardArt::new("ba7665c7-c211-45d7-bde1-f7952548025f", "Anthony Palumbo"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 149 — Darksteel Axe
pub(in crate::card::sets) static DARKSTEEL_AXE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b997c3e6-4b0e-4f4a-9f66-3fc1d8395494"),
    "Darksteel Axe",
    crate::card::CardArt::new("b997c3e6-4b0e-4f4a-9f66-3fc1d8395494", "Daniel Ljunggren"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_artifact(mana_cost!("{1}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::indestructible(),
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
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{2}"))], "Equip {2}"),
        ]),
);

// SOM 150 — Darksteel Juggernaut
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DARKSTEEL_JUGGERNAUT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ed1f540f-0d51-4e32-a4f9-c8977834572a"),
    "Darksteel Juggernaut",
    crate::card::CardArt::new("ed1f540f-0d51-4e32-a4f9-c8977834572a", "Randis Albion"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 151 — Darksteel Myr
pub(in crate::card::sets) static DARKSTEEL_MYR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0f5712cf-c6a9-4a2e-90db-8ca17c621724"),
    "Darksteel Myr",
    crate::card::CardArt::new("0f5712cf-c6a9-4a2e-90db-8ca17c621724", "Randis Albion"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Myr"], 0, 1)
        .with_abilities(&[abilities::indestructible()]),
);

// SOM 152 — Darksteel Sentinel
pub(in crate::card::sets) static DARKSTEEL_SENTINEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("768e9dde-59e5-4b50-9b38-b46e2a593107"),
    "Darksteel Sentinel",
    crate::card::CardArt::new("768e9dde-59e5-4b50-9b38-b46e2a593107", "Erica Yang"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_artifact_creature(mana_cost!("{6}"), &["Golem"], 3, 3).with_abilities(&[
        abilities::flash(),
        abilities::vigilance(),
        abilities::indestructible(),
    ]),
);

// SOM 153 — Echo Circlet
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ECHO_CIRCLET: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("49e661c6-bc3e-45b4-ae1c-5002e381faf3"),
    "Echo Circlet",
    crate::card::CardArt::new("49e661c6-bc3e-45b4-ae1c-5002e381faf3", "Daarken"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 154 — Etched Champion
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ETCHED_CHAMPION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ab2242c2-7379-4fff-a745-d180685da6db"),
    "Etched Champion",
    crate::card::CardArt::new("ab2242c2-7379-4fff-a745-d180685da6db", "Matt Cavotta"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 155 — Flight Spellbomb
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FLIGHT_SPELLBOMB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0fa09e06-08fd-4ecd-83fe-f0e0856547a5"),
    "Flight Spellbomb",
    crate::card::CardArt::new("0fa09e06-08fd-4ecd-83fe-f0e0856547a5", "Franz Vohwinkel"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 156 — Glint Hawk Idol
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GLINT_HAWK_IDOL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0a742da4-638d-4888-94f1-db2f4ada9f94"),
    "Glint Hawk Idol",
    crate::card::CardArt::new("0a742da4-638d-4888-94f1-db2f4ada9f94", "Dave Allsop"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 157 — Gold Myr
pub(in crate::card::sets) static GOLD_MYR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fa9b4040-ab49-476b-b101-5ef2b1824e10"),
    "Gold Myr",
    crate::card::CardArt::new("ac92126c-fb22-4b97-bbc5-b0533a0baad8", "Alan Pollack"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Myr"], 1, 1).with_ability(
        AbilityDef::activated_mana(
            "{T}: Add {W}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::White)),
        ),
    ),
);

// SOM 158 — Golden Urn
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GOLDEN_URN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ec7abeca-da01-4962-b107-dd7a77469753"),
    "Golden Urn",
    crate::card::CardArt::new("ec7abeca-da01-4962-b107-dd7a77469753", "Charles Urbach"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 159 — Golem Artisan
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GOLEM_ARTISAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7ccfc314-2f18-43c2-9ccd-59bb5dbe35e9"),
    "Golem Artisan",
    crate::card::CardArt::new("7ccfc314-2f18-43c2-9ccd-59bb5dbe35e9", "Nic Klein"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 160 — Golem Foundry
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GOLEM_FOUNDRY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3cef2e6a-e46b-4425-b507-3213cfd1400c"),
    "Golem Foundry",
    crate::card::CardArt::new("3cef2e6a-e46b-4425-b507-3213cfd1400c", "Nic Klein"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 161 — Golem's Heart
pub(in crate::card::sets) static GOLEM_S_HEART: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("647ecb81-2d23-40f3-8570-0b86e2ed1c5e"),
    "Golem's Heart",
    crate::card::CardArt::new("647ecb81-2d23-40f3-8570-0b86e2ed1c5e", "Matt Cavotta"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(AbilityDef::triggered(
        "Whenever a player casts an artifact spell, you may gain 1 life.",
        TriggerEventDef::SpellCast(ObjectPredicateDef::HasType(CardType::Artifact)),
        EffectDef::May {
            player: EffectRecipientDef::Controller,
            effect: &EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        },
    )),
);

// SOM 162 — Grafted Exoskeleton
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GRAFTED_EXOSKELETON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9aa64374-0693-47c9-8b69-56def3817b14"),
    "Grafted Exoskeleton",
    crate::card::CardArt::new("9aa64374-0693-47c9-8b69-56def3817b14", "Allen Williams"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 163 — Grindclock
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GRINDCLOCK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a6df2e7f-e46e-4808-8125-42a3aa66377c"),
    "Grindclock",
    crate::card::CardArt::new("a6df2e7f-e46e-4808-8125-42a3aa66377c", "Nils Hamm"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 164 — Heavy Arbalest
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HEAVY_ARBALEST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5737246f-1292-4af6-aecf-8f161f5300cb"),
    "Heavy Arbalest",
    crate::card::CardArt::new("5737246f-1292-4af6-aecf-8f161f5300cb", "David Rapoza"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 165 — Horizon Spellbomb
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HORIZON_SPELLBOMB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9d93378e-1de2-4954-9458-dd3306f2996e"),
    "Horizon Spellbomb",
    crate::card::CardArt::new("9d93378e-1de2-4954-9458-dd3306f2996e", "Franz Vohwinkel"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 166 — Ichorclaw Myr
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ICHORCLAW_MYR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("faef8b8b-2c45-4fed-b6ba-a8ac49c66330"),
    "Ichorclaw Myr",
    crate::card::CardArt::new("faef8b8b-2c45-4fed-b6ba-a8ac49c66330", "Eric Deschamps"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 167 — Infiltration Lens
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static INFILTRATION_LENS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1baa10da-2733-4657-a1ea-74eb5a5a82b1"),
    "Infiltration Lens",
    crate::card::CardArt::new("1baa10da-2733-4657-a1ea-74eb5a5a82b1", "Izzy"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 168 — Iron Myr
pub(in crate::card::sets) static IRON_MYR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("08e17883-0767-40b5-ac44-a52a1ea54993"),
    "Iron Myr",
    crate::card::CardArt::new("5bd0a588-b695-4060-b5d5-c6a74710ff0f", "Alan Pollack"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Myr"], 1, 1).with_ability(
        AbilityDef::activated_mana(
            "{T}: Add {R}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)),
        ),
    ),
);

// SOM 169 — Kuldotha Forgemaster
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KULDOTHA_FORGEMASTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ad590bea-b872-4af7-a612-c8e8759d59df"),
    "Kuldotha Forgemaster",
    crate::card::CardArt::new("ad590bea-b872-4af7-a612-c8e8759d59df", "jD"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 170 — Leaden Myr
pub(in crate::card::sets) static LEADEN_MYR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("555efe5f-848f-44da-92b5-69c8e852f179"),
    "Leaden Myr",
    crate::card::CardArt::new("3a709559-fec3-44f4-a2bf-3396989b9189", "Alan Pollack"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Myr"], 1, 1).with_ability(
        AbilityDef::activated_mana(
            "{T}: Add {B}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Black)),
        ),
    ),
);

// SOM 171 — Liquimetal Coating
pub(in crate::card::sets) static LIQUIMETAL_COATING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("43ec9201-06e7-4a70-8dcf-7462a019965d"),
    "Liquimetal Coating",
    crate::card::CardArt::new("43ec9201-06e7-4a70-8dcf-7462a019965d", "Johann Bodin"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(AbilityDef::activated_with_targets(
        "{T}: Target permanent becomes an artifact in addition to its other types until end of turn.",
        &[AbilityCostDef::TapSource],
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::Any,
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::add_card_types(crate::card::CardTypeSet::single(
                CardType::Artifact,
            )),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// SOM 172 — Livewire Lash
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LIVEWIRE_LASH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bbef3e31-eb5a-43f7-a0b2-12348df6968d"),
    "Livewire Lash",
    crate::card::CardArt::new("bbef3e31-eb5a-43f7-a0b2-12348df6968d", "Daniel Ljunggren"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 173 — Lux Cannon
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LUX_CANNON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("95e274ea-e8f6-48ea-a877-c84b77c96d0c"),
    "Lux Cannon",
    crate::card::CardArt::new("95e274ea-e8f6-48ea-a877-c84b77c96d0c", "Martina Pilcerova"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 174 — Memnite
pub(in crate::card::sets) static MEMNITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("469cc4e0-49c0-4009-97ea-28e44addec69"),
    "Memnite",
    crate::card::CardArt::new("469cc4e0-49c0-4009-97ea-28e44addec69", "Svetlin Velinov"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_artifact_creature(mana_cost!("{0}"), &["Construct"], 1, 1),
);

// SOM 175 — Mimic Vat
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MIMIC_VAT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("736fff86-2417-4a77-b8eb-be2d1d142a9f"),
    "Mimic Vat",
    crate::card::CardArt::new("736fff86-2417-4a77-b8eb-be2d1d142a9f", "Matt Cavotta"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 176 — Mindslaver
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MINDSLAVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("98fb1eaa-2871-491a-a4f5-3e358778ba40"),
    "Mindslaver",
    crate::card::CardArt::new("00d03b17-75ae-40d2-8570-b219ef0dfd4a", "Volkan Baǵa"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 177 — Molten-Tail Masticore
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MOLTEN_TAIL_MASTICORE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("48311a45-c0e1-4170-8dab-2b3495096c48"),
    "Molten-Tail Masticore",
    crate::card::CardArt::new("48311a45-c0e1-4170-8dab-2b3495096c48", "Whit Brachna"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 178 — Moriok Replica
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MORIOK_REPLICA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("480311ae-b9af-4fb7-881b-35566598cf07"),
    "Moriok Replica",
    crate::card::CardArt::new(
        "480311ae-b9af-4fb7-881b-35566598cf07",
        "Zoltan Boros & Gabor Szikszai",
    ),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 179 — Mox Opal
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MOX_OPAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6be9b1d5-9ab8-4adb-ba54-2c0117e842fa"),
    "Mox Opal",
    crate::card::CardArt::new("6be9b1d5-9ab8-4adb-ba54-2c0117e842fa", "Volkan Baǵa"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 180 — Myr Battlesphere
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MYR_BATTLESPHERE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b0ae94ed-7314-470b-baba-f2f58bbc894a"),
    "Myr Battlesphere",
    crate::card::CardArt::new("b0ae94ed-7314-470b-baba-f2f58bbc894a", "Franz Vohwinkel"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 181 — Myr Galvanizer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MYR_GALVANIZER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e55ca835-b7f3-497c-b0bc-50a182cabecf"),
    "Myr Galvanizer",
    crate::card::CardArt::new("e55ca835-b7f3-497c-b0bc-50a182cabecf", "Greg Staples"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 182 — Myr Propagator
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MYR_PROPAGATOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("837e4b25-d70b-48d8-aaad-9622ad93e154"),
    "Myr Propagator",
    crate::card::CardArt::new("837e4b25-d70b-48d8-aaad-9622ad93e154", "Ryan Pancoast"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 183 — Myr Reservoir
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MYR_RESERVOIR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("60678391-44b2-4525-94dc-ffc5a433b79b"),
    "Myr Reservoir",
    crate::card::CardArt::new("60678391-44b2-4525-94dc-ffc5a433b79b", "Jung Park"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 184 — Necrogen Censer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NECROGEN_CENSER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4f707119-ede9-4697-b723-d6cea96e6f2b"),
    "Necrogen Censer",
    crate::card::CardArt::new("4f707119-ede9-4697-b723-d6cea96e6f2b", "Pete Venters"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 185 — Necropede
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NECROPEDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8d2e522b-e6f8-4fae-8c08-ce2bb8bed04f"),
    "Necropede",
    crate::card::CardArt::new("8d2e522b-e6f8-4fae-8c08-ce2bb8bed04f", "Nic Klein"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 186 — Neurok Replica
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NEUROK_REPLICA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4e32d5a8-0916-4728-9cb2-3903262bf873"),
    "Neurok Replica",
    crate::card::CardArt::new(
        "4e32d5a8-0916-4728-9cb2-3903262bf873",
        "Zoltan Boros & Gabor Szikszai",
    ),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 187 — Nihil Spellbomb
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NIHIL_SPELLBOMB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("603d217b-6375-46fc-992a-8dbd779da1e5"),
    "Nihil Spellbomb",
    crate::card::CardArt::new("603d217b-6375-46fc-992a-8dbd779da1e5", "Franz Vohwinkel"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 188 — Nim Deathmantle
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NIM_DEATHMANTLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f638bd96-8424-461f-87bf-4b7a7153fd35"),
    "Nim Deathmantle",
    crate::card::CardArt::new("f638bd96-8424-461f-87bf-4b7a7153fd35", "Karl Kopinski"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 189 — Origin Spellbomb
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ORIGIN_SPELLBOMB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("91e7faa4-160e-47d9-a9a1-5928d9d2b5e4"),
    "Origin Spellbomb",
    crate::card::CardArt::new("91e7faa4-160e-47d9-a9a1-5928d9d2b5e4", "Franz Vohwinkel"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 190 — Palladium Myr
pub(in crate::card::sets) static PALLADIUM_MYR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("18c016ad-bb82-4944-8c06-ab180b808041"),
    "Palladium Myr",
    crate::card::CardArt::new("18c016ad-bb82-4944-8c06-ab180b808041", "Alan Pollack"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Myr"], 2, 2).with_ability(
        AbilityDef::activated_mana(
            "{T}: Add {C}{C}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::combination(&[ManaColor::Colorless], 2)),
        ),
    ),
);

// SOM 191 — Panic Spellbomb
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PANIC_SPELLBOMB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e9a29832-8630-498a-9ac3-bc709a6dc95d"),
    "Panic Spellbomb",
    crate::card::CardArt::new("e9a29832-8630-498a-9ac3-bc709a6dc95d", "Franz Vohwinkel"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 192 — Perilous Myr
pub(in crate::card::sets) static PERILOUS_MYR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4b942605-eb4a-452d-9b07-a4f912f96958"),
    "Perilous Myr",
    crate::card::CardArt::new("4b942605-eb4a-452d-9b07-a4f912f96958", "Jason Felix"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Phyrexian", "Myr"], 1, 1).with_ability(
        AbilityDef::triggered_with_targets(
            "When this creature dies, it deals 2 damage to any target.",
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
                amount: ValueDef::Constant(2),
            },
        ),
    ),
);

// SOM 193 — Platinum Emperion
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PLATINUM_EMPERION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b7919474-db2b-441a-b368-9e430ddf70ab"),
    "Platinum Emperion",
    crate::card::CardArt::new("b7919474-db2b-441a-b368-9e430ddf70ab", "Chris Rahn"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 194 — Precursor Golem
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PRECURSOR_GOLEM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1c4625ad-1c83-4095-a5a2-0fc9fa4dd5f2"),
    "Precursor Golem",
    crate::card::CardArt::new("1c4625ad-1c83-4095-a5a2-0fc9fa4dd5f2", "Chippy"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 195 — Prototype Portal
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PROTOTYPE_PORTAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("10b264aa-303b-4982-a653-9573d39c28de"),
    "Prototype Portal",
    crate::card::CardArt::new("10b264aa-303b-4982-a653-9573d39c28de", "Drew Baker"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 196 — Ratchet Bomb (reprint)

// SOM 197 — Razorfield Thresher
pub(in crate::card::sets) static RAZORFIELD_THRESHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b0a74203-d342-489d-a584-bca78ef3331d"),
    "Razorfield Thresher",
    crate::card::CardArt::new("b0a74203-d342-489d-a584-bca78ef3331d", "Karl Kopinski"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_artifact_creature(mana_cost!("{7}"), &["Construct"], 6, 4),
);

// SOM 198 — Rust Tick
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RUST_TICK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1d638741-1cfe-4496-8d7e-7849a82dcb24"),
    "Rust Tick",
    crate::card::CardArt::new("1d638741-1cfe-4496-8d7e-7849a82dcb24", "Carl Critchlow"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 199 — Rusted Relic
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RUSTED_RELIC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d2419dd5-9c31-42b2-b6ef-bbdf11c558ac"),
    "Rusted Relic",
    crate::card::CardArt::new("d2419dd5-9c31-42b2-b6ef-bbdf11c558ac", "Igor Kieryluk"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 200 — Saberclaw Golem
pub(in crate::card::sets) static SABERCLAW_GOLEM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6656b6d1-1c92-4da4-8afb-36f11610b0b4"),
    "Saberclaw Golem",
    crate::card::CardArt::new("6656b6d1-1c92-4da4-8afb-36f11610b0b4", "Mike Bierek"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_artifact_creature(mana_cost!("{5}"), &["Golem"], 4, 2).with_ability(
        AbilityDef::activated(
            "{R}: This creature gains first strike until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// SOM 201 — Semblance Anvil
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SEMBLANCE_ANVIL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0380b46d-1660-404d-9d11-705d8809ea46"),
    "Semblance Anvil",
    crate::card::CardArt::new("0380b46d-1660-404d-9d11-705d8809ea46", "Dan Murayama Scott"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 202 — Silver Myr
pub(in crate::card::sets) static SILVER_MYR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b83a73a2-fedb-40bd-8e29-82a7abd6f211"),
    "Silver Myr",
    crate::card::CardArt::new("fdd60081-3942-4e0e-aacd-a0c121bb08c7", "Alan Pollack"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Myr"], 1, 1).with_ability(
        AbilityDef::activated_mana(
            "{T}: Add {U}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Blue)),
        ),
    ),
);

// SOM 203 — Snapsail Glider
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SNAPSAIL_GLIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fc98e0af-b18e-4172-bc56-19952ebd0303"),
    "Snapsail Glider",
    crate::card::CardArt::new("fc98e0af-b18e-4172-bc56-19952ebd0303", "Efrem Palacios"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 204 — Soliton
pub(in crate::card::sets) static SOLITON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7b608c28-18cc-47d6-861e-2fd783aa3ade"),
    "Soliton",
    crate::card::CardArt::new("7b608c28-18cc-47d6-861e-2fd783aa3ade", "Jason Felix"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_artifact_creature(mana_cost!("{5}"), &["Construct"], 3, 4).with_ability(
        AbilityDef::activated(
            "{U}: Untap this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{U}"))],
            EffectDef::Untap {
                object: EffectRecipientDef::Source,
            },
        ),
    ),
);

// SOM 205 — Steel Hellkite
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STEEL_HELLKITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b268cd3b-af91-4c22-ac73-347babb69200"),
    "Steel Hellkite",
    crate::card::CardArt::new("b126ee24-9597-4ee8-9c4d-5caed585424a", "James Paick"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 206 — Strata Scythe
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STRATA_SCYTHE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8f2cb906-3748-4675-89b3-bde2f9a8444a"),
    "Strata Scythe",
    crate::card::CardArt::new("8f2cb906-3748-4675-89b3-bde2f9a8444a", "Scott Chou"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 207 — Strider Harness
pub(in crate::card::sets) static STRIDER_HARNESS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9d7b9e54-b3ef-44fb-9240-0d67c1c4b7f6"),
    "Strider Harness",
    crate::card::CardArt::new("9d7b9e54-b3ef-44fb-9240-0d67c1c4b7f6", "Matt Stewart"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_artifact(mana_cost!("{3}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +1/+1 and has haste.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_ability(&abilities::haste()),
                    ]),
                },
            ),
            abilities::equip(
                &[AbilityCostDef::Mana(mana_cost!("{1}"))],
                "Equip {1} ({1}: Attach to target creature you control. Equip only as a sorcery.)",
            ),
        ]),
);

// SOM 208 — Sword of Body and Mind
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SWORD_OF_BODY_AND_MIND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5bc29686-a89c-46e6-b32c-a70733aba389"),
    "Sword of Body and Mind",
    crate::card::CardArt::new("03cc5caf-b2d7-4211-a1a4-f0ad6e70e3f4", "Chris Rahn"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 209 — Sylvok Lifestaff
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SYLVOK_LIFESTAFF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("abbc5ae5-8e8b-4106-844f-2d49d2a51ed9"),
    "Sylvok Lifestaff",
    crate::card::CardArt::new("abbc5ae5-8e8b-4106-844f-2d49d2a51ed9", "Martina Pilcerova"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 210 — Sylvok Replica
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SYLVOK_REPLICA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7caa3ce3-15a9-40ca-ad45-baff0f276483"),
    "Sylvok Replica",
    crate::card::CardArt::new(
        "7caa3ce3-15a9-40ca-ad45-baff0f276483",
        "Zoltan Boros & Gabor Szikszai",
    ),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 211 — Throne of Geth
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static THRONE_OF_GETH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("583d7386-3eb5-4f1d-8da9-f00e020a307b"),
    "Throne of Geth",
    crate::card::CardArt::new(
        "583d7386-3eb5-4f1d-8da9-f00e020a307b",
        "Jana Schirmer & Johannes Voss",
    ),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 212 — Tower of Calamities
pub(in crate::card::sets) static TOWER_OF_CALAMITIES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8a77391b-5727-4408-bb50-970f7a13a83c"),
    "Tower of Calamities",
    crate::card::CardArt::new("8a77391b-5727-4408-bb50-970f7a13a83c", "Aleksi Briclot"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_artifact(mana_cost!("{4}")).with_ability(AbilityDef::activated_with_targets(
        "{8}, {T}: This artifact deals 12 damage to target creature.",
        &[
            AbilityCostDef::Mana(mana_cost!("{8}")),
            AbilityCostDef::TapSource,
        ],
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(12),
        },
    )),
);

// SOM 213 — Trigon of Corruption
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TRIGON_OF_CORRUPTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("26e215e0-836c-4b37-8f9a-9093a535bff1"),
    "Trigon of Corruption",
    crate::card::CardArt::new("26e215e0-836c-4b37-8f9a-9093a535bff1", "Nils Hamm"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 214 — Trigon of Infestation
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TRIGON_OF_INFESTATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("be409a80-846c-4883-8aee-c2e3f973fc0f"),
    "Trigon of Infestation",
    crate::card::CardArt::new("be409a80-846c-4883-8aee-c2e3f973fc0f", "Dave Allsop"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 215 — Trigon of Mending
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TRIGON_OF_MENDING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("241142e0-3a79-4bce-8535-18ae7e392f5e"),
    "Trigon of Mending",
    crate::card::CardArt::new("241142e0-3a79-4bce-8535-18ae7e392f5e", "Igor Kieryluk"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 216 — Trigon of Rage
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TRIGON_OF_RAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1135f3b7-8c6b-47ff-b895-b7127836b0bf"),
    "Trigon of Rage",
    crate::card::CardArt::new("1135f3b7-8c6b-47ff-b895-b7127836b0bf", "Marc Simonetti"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 217 — Trigon of Thought
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TRIGON_OF_THOUGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f8da37ba-52e3-417e-8d7b-6c3e060552a4"),
    "Trigon of Thought",
    crate::card::CardArt::new("f8da37ba-52e3-417e-8d7b-6c3e060552a4", "Mike Bierek"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 218 — Tumble Magnet
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TUMBLE_MAGNET: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e6478389-15be-405f-b755-108c942d72ec"),
    "Tumble Magnet",
    crate::card::CardArt::new("e6478389-15be-405f-b755-108c942d72ec", "Drew Baker"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 219 — Vector Asp
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VECTOR_ASP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7ffe86e1-ad47-4ccb-aa55-119dc681d370"),
    "Vector Asp",
    crate::card::CardArt::new("7ffe86e1-ad47-4ccb-aa55-119dc681d370", "Erica Yang"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 220 — Venser's Journal
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VENSER_S_JOURNAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2763643d-5b53-49d0-bc3d-5626bf00f3f4"),
    "Venser's Journal",
    crate::card::CardArt::new(
        "2763643d-5b53-49d0-bc3d-5626bf00f3f4",
        "Christopher Moeller",
    ),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 221 — Vulshok Replica
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VULSHOK_REPLICA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("32885a6c-b293-405f-9f2e-9e0dd7d1cb8c"),
    "Vulshok Replica",
    crate::card::CardArt::new(
        "32885a6c-b293-405f-9f2e-9e0dd7d1cb8c",
        "Zoltan Boros & Gabor Szikszai",
    ),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 222 — Wall of Tanglecord
pub(in crate::card::sets) static WALL_OF_TANGLECORD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("792e2aed-ce6e-4fa1-a31c-a4574e5cf1f5"),
    "Wall of Tanglecord",
    crate::card::CardArt::new("792e2aed-ce6e-4fa1-a31c-a4574e5cf1f5", "Vance Kovacs"),
    crate::card::CardSet::ScarsOfMirrodin,
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Wall"], 0, 6).with_abilities(&[
        abilities::defender(),
        AbilityDef::activated(
            "{G}: This creature gains reach until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{G}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::reach()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// SOM 223 — Wurmcoil Engine
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WURMCOIL_ENGINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b9bdfa34-f608-40d4-b634-b260ad620f18"),
    "Wurmcoil Engine",
    crate::card::CardArt::new("33672990-4860-4aa6-ac1b-f9da66f5da59", "Raymond Swanland"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 224 — Blackcleave Cliffs
pub(in crate::card::sets) static BLACKCLEAVE_CLIFFS: CardRecord = CardRecord::new_with_legacy_id(
    2131,
    "Blackcleave Cliffs",
    CardArt::new("3d71be5f-0fd7-4a88-8041-f4d6bc4cc9ac", "Dave Kendall"),
    CardSet::ScarsOfMirrodin,
    CardRules::new_land(&[]).with_abilities(&BLACKCLEAVE_CLIFFS_ABILITIES),
);

static COPPERLINE_GORGE_ABILITIES: [AbilityDef; 2] = [
    FAST_LAND_ENTERS,
    AbilityDef::activated_mana(
        "{T}: Add {R} or {G}.",
        &[AbilityCostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::choice(&[
            ManaColor::Red,
            ManaColor::Green,
        ])),
    ),
];

// SOM 225 — Copperline Gorge
pub(in crate::card::sets) static COPPERLINE_GORGE: CardRecord = CardRecord::new_with_legacy_id(
    2132,
    "Copperline Gorge",
    CardArt::new(
        "28f1d784-f286-418d-a712-bc07ad10d4a2",
        "Zoltan Boros & Gabor Szikszai",
    ),
    CardSet::ScarsOfMirrodin,
    CardRules::new_land(&[]).with_abilities(&COPPERLINE_GORGE_ABILITIES),
);

static DARKSLICK_SHORES_ABILITIES: [AbilityDef; 2] = [
    FAST_LAND_ENTERS,
    AbilityDef::activated_mana(
        "{T}: Add {U} or {B}.",
        &[AbilityCostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::choice(&[
            ManaColor::Blue,
            ManaColor::Black,
        ])),
    ),
];

// SOM 226 — Darkslick Shores
pub(in crate::card::sets) static DARKSLICK_SHORES: CardRecord = CardRecord::new_with_legacy_id(
    2133,
    "Darkslick Shores",
    CardArt::new("e530388b-eb19-4211-abd8-8a4c3c38c3af", "Charles Urbach"),
    CardSet::ScarsOfMirrodin,
    CardRules::new_land(&[]).with_abilities(&DARKSLICK_SHORES_ABILITIES),
);

static RAZORVERGE_THICKET_ABILITIES: [AbilityDef; 2] = [
    FAST_LAND_ENTERS,
    AbilityDef::activated_mana(
        "{T}: Add {G} or {W}.",
        &[AbilityCostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::choice(&[
            ManaColor::Green,
            ManaColor::White,
        ])),
    ),
];

// SOM 227 — Glimmerpost
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GLIMMERPOST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8b63efb6-249c-4f57-9af1-baffe938520c"),
    "Glimmerpost",
    crate::card::CardArt::new("8b63efb6-249c-4f57-9af1-baffe938520c", "Matt Cavotta"),
    crate::card::CardSet::ScarsOfMirrodin,
    crate::card::CardRules::unsupported(),
);

// SOM 228 — Razorverge Thicket
pub(in crate::card::sets) static RAZORVERGE_THICKET: CardRecord = CardRecord::new_with_legacy_id(
    2134,
    "Razorverge Thicket",
    CardArt::new("345e053a-3178-485c-8602-1624bbf2f064", "James Paick"),
    CardSet::ScarsOfMirrodin,
    CardRules::new_land(&[]).with_abilities(&RAZORVERGE_THICKET_ABILITIES),
);

static SEACHROME_COAST_ABILITIES: [AbilityDef; 2] = [
    FAST_LAND_ENTERS,
    AbilityDef::activated_mana(
        "{T}: Add {W} or {U}.",
        &[AbilityCostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::choice(&[
            ManaColor::White,
            ManaColor::Blue,
        ])),
    ),
];

// SOM 229 — Seachrome Coast
pub(in crate::card::sets) static SEACHROME_COAST: CardRecord = CardRecord::new_with_legacy_id(
    2135,
    "Seachrome Coast",
    CardArt::new("99939b90-e88c-4c2f-ba78-56d455611703", "Lars Grant-West"),
    CardSet::ScarsOfMirrodin,
    CardRules::new_land(&[]).with_abilities(&SEACHROME_COAST_ABILITIES),
);

// SOM 230 — Plains (reprint)

// SOM 231 — Plains (alternate printing)

// SOM 232 — Plains (alternate printing)

// SOM 233 — Plains (alternate printing)

// SOM 234 — Island (reprint)

// SOM 235 — Island (alternate printing)

// SOM 236 — Island (alternate printing)

// SOM 237 — Island (alternate printing)

// SOM 238 — Swamp (reprint)

// SOM 239 — Swamp (alternate printing)

// SOM 240 — Swamp (alternate printing)

// SOM 241 — Swamp (alternate printing)

// SOM 242 — Mountain (reprint)

// SOM 243 — Mountain (alternate printing)

// SOM 244 — Mountain (alternate printing)

// SOM 245 — Mountain (alternate printing)

// SOM 246 — Forest (reprint)

// SOM 247 — Forest (alternate printing)

// SOM 248 — Forest (alternate printing)

// SOM 249 — Forest (alternate printing)

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ABUNA_ACOLYTE,
    &AURIOK_EDGEWRIGHT,
    &AURIOK_SUNCHASER,
    &DISPENSE_JUSTICE,
    &ELSPETH_TIREL,
    &FULGENT_DISTRACTION,
    &GHALMA_S_WARDEN,
    &GLIMMERPOINT_STAG,
    &GLINT_HAWK,
    &INDOMITABLE_ARCHANGEL,
    &KEMBA_KHA_REGENT,
    &KEMBA_S_SKYGUARD,
    &LEONIN_ARBITER,
    &LOXODON_WAYFARER,
    &MYRSMITH,
    &RAZOR_HIPPOGRIFF,
    &REVOKE_EXISTENCE,
    &SALVAGE_SCOUT,
    &SEIZE_THE_INITIATIVE,
    &SOUL_PARRY,
    &SUNBLAST_ANGEL,
    &SUNSPEAR_SHIKARI,
    &TEMPERED_STEEL,
    &TRUE_CONVICTION,
    &VIGIL_FOR_THE_LOST,
    &WHITESUN_S_PASSAGE,
    &ARGENT_SPHINX,
    &BONDS_OF_QUICKSILVER,
    &DARKSLICK_DRAKE,
    &DISSIPATION_FIELD,
    &GRAND_ARCHITECT,
    &HALT_ORDER,
    &INEXORABLE_TIDE,
    &LUMENGRID_DRAKE,
    &NEUROK_INVISIMANCER,
    &PLATED_SEASTRIDER,
    &QUICKSILVER_GARGANTUAN,
    &RIDDLESMITH,
    &SCRAPDIVER_SERPENT,
    &SCREECHING_SILCAW,
    &SHAPE_ANEW,
    &SKY_EEL_SCHOOL,
    &STEADY_PROGRESS,
    &STOIC_REBUTTAL,
    &THRUMMINGBIRD,
    &TRINKET_MAGE,
    &TURN_ASIDE,
    &TWISTED_IMAGE,
    &VAULT_SKYWARD,
    &VEDALKEN_CERTARCH,
    &VOLITION_REINS,
    &BLACKCLEAVE_GOBLIN,
    &BLEAK_COVEN_VAMPIRES,
    &BLISTERGRUB,
    &CARNIFEX_DEMON,
    &CONTAGIOUS_NIM,
    &CORRUPTED_HARVESTER,
    &DROSS_HOPPER,
    &EXSANGUINATE,
    &FLESH_ALLERGY,
    &FUME_SPITTER,
    &GETH_LORD_OF_THE_VAULT,
    &GRASP_OF_DARKNESS,
    &HAND_OF_THE_PRAETORS,
    &ICHOR_RATS,
    &INSTILL_INFECTION,
    &MEMORICIDE,
    &MORIOK_REAVER,
    &NECROGEN_SCUDDER,
    &NECROTIC_OOZE,
    &PAINFUL_QUANDARY,
    &PAINSMITH,
    &PLAGUE_STINGER,
    &PSYCHIC_MIASMA,
    &RELIC_PUTRESCENCE,
    &SKINRENDER,
    &SKITHIRYX_THE_BLIGHT_DRAGON,
    &TAINTED_STRIKE,
    &ARC_TRAIL,
    &ASSAULT_STROBE,
    &BARRAGE_OGRE,
    &BLADE_TRIBE_BERSERKERS,
    &BLOODSHOT_TRAINEE,
    &CEREBRAL_ERUPTION,
    &EMBERSMITH,
    &FERROVORE,
    &FLAMEBORN_HELLION,
    &FURNACE_CELEBRATION,
    &GALVANIC_BLAST,
    &GOBLIN_GAVELEER,
    &HOARD_SMELTER_DRAGON,
    &KOTH_OF_THE_HAMMER,
    &KULDOTHA_PHOENIX,
    &KULDOTHA_REBIRTH,
    &MELT_TERRAIN,
    &MOLTEN_PSYCHE,
    &OGRE_GEARGRABBER,
    &OXIDDA_DAREDEVIL,
    &OXIDDA_SCRAPMELTER,
    &SCORIA_ELEMENTAL,
    &SPIKESHOT_ELDER,
    &TUNNEL_IGNUS,
    &VULSHOK_HEARTSTOKER,
    &ACID_WEB_SPIDER,
    &ALPHA_TYRRANAX,
    &ASCETICISM,
    &BELLOWING_TANGLEWURM,
    &BLIGHT_MAMBA,
    &BLUNT_THE_ASSAULT,
    &CARAPACE_FORGER,
    &CARRION_CALL,
    &COPPERHORN_SCOUT,
    &CYSTBEARER,
    &ENGULFING_SLAGWURM,
    &EZURI_RENEGADE_LEADER,
    &EZURI_S_ARCHERS,
    &EZURI_S_BRIGADE,
    &GENESIS_WAVE,
    &LIEGE_OF_THE_TANGLE,
    &LIFESMITH,
    &MOLDER_BEAST,
    &PUTREFAX,
    &SLICE_IN_TWAIN,
    &TANGLE_ANGLER,
    &TEL_JILAD_DEFIANCE,
    &TEL_JILAD_FALLEN,
    &UNTAMED_MIGHT,
    &VIRIDIAN_REVEL,
    &WING_PUNCTURE,
    &WITHSTAND_DEATH,
    &VENSER_THE_SOJOURNER,
    &ARGENTUM_ARMOR,
    &AURIOK_REPLICA,
    &BARBED_BATTLEGEAR,
    &BLADED_PINIONS,
    &CHIMERIC_MASS,
    &CHROME_STEED,
    &CLONE_SHELL,
    &CONTAGION_CLASP,
    &CONTAGION_ENGINE,
    &COPPER_MYR,
    &CORPSE_CUR,
    &CULLING_DAIS,
    &DARKSTEEL_AXE,
    &DARKSTEEL_JUGGERNAUT,
    &DARKSTEEL_MYR,
    &DARKSTEEL_SENTINEL,
    &ECHO_CIRCLET,
    &ETCHED_CHAMPION,
    &FLIGHT_SPELLBOMB,
    &GLINT_HAWK_IDOL,
    &GOLD_MYR,
    &GOLDEN_URN,
    &GOLEM_ARTISAN,
    &GOLEM_FOUNDRY,
    &GOLEM_S_HEART,
    &GRAFTED_EXOSKELETON,
    &GRINDCLOCK,
    &HEAVY_ARBALEST,
    &HORIZON_SPELLBOMB,
    &ICHORCLAW_MYR,
    &INFILTRATION_LENS,
    &IRON_MYR,
    &KULDOTHA_FORGEMASTER,
    &LEADEN_MYR,
    &LIQUIMETAL_COATING,
    &LIVEWIRE_LASH,
    &LUX_CANNON,
    &MEMNITE,
    &MIMIC_VAT,
    &MINDSLAVER,
    &MOLTEN_TAIL_MASTICORE,
    &MORIOK_REPLICA,
    &MOX_OPAL,
    &MYR_BATTLESPHERE,
    &MYR_GALVANIZER,
    &MYR_PROPAGATOR,
    &MYR_RESERVOIR,
    &NECROGEN_CENSER,
    &NECROPEDE,
    &NEUROK_REPLICA,
    &NIHIL_SPELLBOMB,
    &NIM_DEATHMANTLE,
    &ORIGIN_SPELLBOMB,
    &PALLADIUM_MYR,
    &PANIC_SPELLBOMB,
    &PERILOUS_MYR,
    &PLATINUM_EMPERION,
    &PRECURSOR_GOLEM,
    &PROTOTYPE_PORTAL,
    &RAZORFIELD_THRESHER,
    &RUST_TICK,
    &RUSTED_RELIC,
    &SABERCLAW_GOLEM,
    &SEMBLANCE_ANVIL,
    &SILVER_MYR,
    &SNAPSAIL_GLIDER,
    &SOLITON,
    &STEEL_HELLKITE,
    &STRATA_SCYTHE,
    &STRIDER_HARNESS,
    &SWORD_OF_BODY_AND_MIND,
    &SYLVOK_LIFESTAFF,
    &SYLVOK_REPLICA,
    &THRONE_OF_GETH,
    &TOWER_OF_CALAMITIES,
    &TRIGON_OF_CORRUPTION,
    &TRIGON_OF_INFESTATION,
    &TRIGON_OF_MENDING,
    &TRIGON_OF_RAGE,
    &TRIGON_OF_THOUGHT,
    &TUMBLE_MAGNET,
    &VECTOR_ASP,
    &VENSER_S_JOURNAL,
    &VULSHOK_REPLICA,
    &WALL_OF_TANGLECORD,
    &WURMCOIL_ENGINE,
    &BLACKCLEAVE_CLIFFS,
    &COPPERLINE_GORGE,
    &DARKSLICK_SHORES,
    &GLIMMERPOST,
    &RAZORVERGE_THICKET,
    &SEACHROME_COAST,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&crate::card::sets::y1999::mercadian_masques::ARREST), // SOM 2
    PrintingRecord::reprint(&crate::card::sets::y2013::magic_2014::DISPERSE),      // SOM 31
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::SHATTER),            // SOM 103
    PrintingRecord::reprint(&crate::card::sets::y2012::magic_2013::TURN_TO_SLAG),  // SOM 106
    PrintingRecord::reprint(&crate::card::sets::y2013::magic_2014::ACCORDERS_SHIELD), // SOM 136
    PrintingRecord::reprint(&crate::card::sets::y2013::magic_2014::RATCHET_BOMB),  // SOM 196
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::PLAINS),             // SOM 230
    PrintingRecord::alternate(&catalog_lea::PLAINS, 1),                            // SOM 231
    PrintingRecord::alternate(&catalog_lea::PLAINS, 2),                            // SOM 232
    PrintingRecord::alternate(&catalog_lea::PLAINS, 3),                            // SOM 233
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::ISLAND),             // SOM 234
    PrintingRecord::alternate(&catalog_lea::ISLAND, 1),                            // SOM 235
    PrintingRecord::alternate(&catalog_lea::ISLAND, 2),                            // SOM 236
    PrintingRecord::alternate(&catalog_lea::ISLAND, 3),                            // SOM 237
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::SWAMP),              // SOM 238
    PrintingRecord::alternate(&catalog_lea::SWAMP, 1),                             // SOM 239
    PrintingRecord::alternate(&catalog_lea::SWAMP, 2),                             // SOM 240
    PrintingRecord::alternate(&catalog_lea::SWAMP, 3),                             // SOM 241
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::MOUNTAIN),           // SOM 242
    PrintingRecord::alternate(&catalog_lea::MOUNTAIN, 1),                          // SOM 243
    PrintingRecord::alternate(&catalog_lea::MOUNTAIN, 2),                          // SOM 244
    PrintingRecord::alternate(&catalog_lea::MOUNTAIN, 3),                          // SOM 245
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::FOREST),             // SOM 246
    PrintingRecord::alternate(&catalog_lea::FOREST, 1),                            // SOM 247
    PrintingRecord::alternate(&catalog_lea::FOREST, 2),                            // SOM 248
    PrintingRecord::alternate(&catalog_lea::FOREST, 3),                            // SOM 249
];
