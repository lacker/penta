//! Torment cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y2012::magic_2013 as catalog_m13;
use crate::card::sets::y2016::eternal_masters as catalog_ema;
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef, AppliedEffectDef,
    BasicLandType, CardArt, CardRules, CardSet, CardType, ChoiceVisibilityDef, ChooseDef,
    ComparisonDef, ConditionDef, CostDef, EffectDef, EffectRecipientDef, ManaColor,
    ObjectChoiceBindingDef, ObjectCountConditionDef, ObjectPredicateDef, ObjectQueryDef,
    ObjectRefDef, ObjectSetDef, PlayerRefDef, PlayerRelation, ResolvedEffectDurationDef,
    TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, ZonePlacement,
    abilities,
};
use crate::ids::ParentBinding;
use crate::{TargetIndex, mana_cost};

// TOR 1 — Angel of Retribution
pub(in crate::card::sets) static ANGEL_OF_RETRIBUTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7f3215ba-e492-4cfd-aa16-0da4818eed1b"),
    "Angel of Retribution",
    CardArt::new("7f3215ba-e492-4cfd-aa16-0da4818eed1b", "rk post"),
    CardSet::Torment,
    // Seven mana for a 5/5 flier that wins every fight in the air, which is
    // what an Angel cost before they started costing less.
    CardRules::new_creature(mana_cost!("{6}{W}"), &["Angel"], 5, 5)
        .with_abilities(&[abilities::flying(), abilities::first_strike()]),
);

// TOR 2 — Aven Trooper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVEN_TROOPER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("79c8f774-6d4f-4fd0-85c0-26ef713e6b89"),
    "Aven Trooper",
    crate::card::CardArt::new("79c8f774-6d4f-4fd0-85c0-26ef713e6b89", "Greg Staples"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 3 — Cleansing Meditation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLEANSING_MEDITATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fd6609ef-71af-4775-affc-34153700c556"),
    "Cleansing Meditation",
    crate::card::CardArt::new("fd6609ef-71af-4775-affc-34153700c556", "Ron Spears"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 4 — Equal Treatment
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EQUAL_TREATMENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("310cb525-9299-471e-b310-392353b25472"),
    "Equal Treatment",
    crate::card::CardArt::new(
        "310cb525-9299-471e-b310-392353b25472",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 5 — Floating Shield
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLOATING_SHIELD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4dbda39b-a998-4ad9-95df-72ed9556c390"),
    "Floating Shield",
    crate::card::CardArt::new("4dbda39b-a998-4ad9-95df-72ed9556c390", "Keith Garletts"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 6 — Frantic Purification
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FRANTIC_PURIFICATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("04f156d1-2655-42aa-a785-d8eb15cb5422"),
    "Frantic Purification",
    crate::card::CardArt::new("04f156d1-2655-42aa-a785-d8eb15cb5422", "Mark Brill"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 7 — Hypochondria
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HYPOCHONDRIA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3467c940-dae4-4667-bec8-524dbce13283"),
    "Hypochondria",
    crate::card::CardArt::new(
        "3467c940-dae4-4667-bec8-524dbce13283",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 8 — Major Teroh
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAJOR_TEROH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3229ca5a-5340-48b8-bd46-b0b924c8faf7"),
    "Major Teroh",
    crate::card::CardArt::new("3229ca5a-5340-48b8-bd46-b0b924c8faf7", "Daren Bader"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 9 — Militant Monk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MILITANT_MONK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("886a3cfd-7f83-480a-bb22-eec67ad35e4f"),
    "Militant Monk",
    crate::card::CardArt::new("886a3cfd-7f83-480a-bb22-eec67ad35e4f", "Mark Brill"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 10 — Morningtide
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MORNINGTIDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a62e17a7-3602-45af-bdb6-fa5f2a9f1155"),
    "Morningtide",
    crate::card::CardArt::new("a62e17a7-3602-45af-bdb6-fa5f2a9f1155", "Tony Szczudlo"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 11 — Mystic Familiar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MYSTIC_FAMILIAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b235567a-424e-48da-a94b-c6674a73b3fa"),
    "Mystic Familiar",
    crate::card::CardArt::new(
        "b235567a-424e-48da-a94b-c6674a73b3fa",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 12 — Pay No Heed
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PAY_NO_HEED: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9520a1be-db77-4f31-a67d-6309a3ddc566"),
    "Pay No Heed",
    crate::card::CardArt::new("9520a1be-db77-4f31-a67d-6309a3ddc566", "Adam Rex"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 13 — Possessed Nomad
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static POSSESSED_NOMAD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5d9dfae5-63f6-401a-9dec-afe838190824"),
    "Possessed Nomad",
    crate::card::CardArt::new("5d9dfae5-63f6-401a-9dec-afe838190824", "Eric Peterson"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 14 — Reborn Hero
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REBORN_HERO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b4acbc7f-c86e-4285-a66d-d7b03fe44377"),
    "Reborn Hero",
    crate::card::CardArt::new("b4acbc7f-c86e-4285-a66d-d7b03fe44377", "Gary Ruddell"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 15 — Spirit Flare
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIRIT_FLARE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("efdbe7c6-99ad-4c73-bffe-7bbdc6965854"),
    "Spirit Flare",
    crate::card::CardArt::new("efdbe7c6-99ad-4c73-bffe-7bbdc6965854", "rk post"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 16 — Stern Judge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STERN_JUDGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("436044d7-981c-4273-a0b4-6dd455608c47"),
    "Stern Judge",
    crate::card::CardArt::new("436044d7-981c-4273-a0b4-6dd455608c47", "Matt Cavotta"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 17 — Strength of Isolation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STRENGTH_OF_ISOLATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7676e6fb-030d-40aa-a914-27ab6d7d5fc5"),
    "Strength of Isolation",
    crate::card::CardArt::new("7676e6fb-030d-40aa-a914-27ab6d7d5fc5", "Jerry Tiritilli"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 18 — Teroh's Faithful
pub(in crate::card::sets) static TEROH_S_FAITHFUL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bff8cf45-c84f-49d7-ad3d-b5e046286cb3"),
    "Teroh's Faithful",
    CardArt::new("bff8cf45-c84f-49d7-ad3d-b5e046286cb3", "Tim Hildebrandt"),
    CardSet::Torment,
    // A 1/4 wall that gains four life on the way in, printed for a set that
    // wanted white to survive rather than win.
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Cleric"], 1, 4).with_ability(
        abilities::enters_trigger(
            "When this creature enters, you gain 4 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(4),
            },
        ),
    ),
);

// TOR 19 — Teroh's Vanguard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEROH_S_VANGUARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1a39f909-7114-4482-8302-b2a56ca9f556"),
    "Teroh's Vanguard",
    crate::card::CardArt::new(
        "1a39f909-7114-4482-8302-b2a56ca9f556",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 20 — Transcendence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRANSCENDENCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("84e8bee2-9e3d-493c-a937-645bf3dcf0db"),
    "Transcendence",
    crate::card::CardArt::new("84e8bee2-9e3d-493c-a937-645bf3dcf0db", "Rebecca Guay"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 21 — Vengeful Dreams
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VENGEFUL_DREAMS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("48bfe8b2-cc98-4430-b576-085163bdf0b6"),
    "Vengeful Dreams",
    crate::card::CardArt::new("48bfe8b2-cc98-4430-b576-085163bdf0b6", "Mark Tedin"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 22 — Alter Reality
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALTER_REALITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("64cd68be-6e6a-4577-8465-a892463b6d6c"),
    "Alter Reality",
    crate::card::CardArt::new("64cd68be-6e6a-4577-8465-a892463b6d6c", "Justin Sweet"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 23 — Ambassador Laquatus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AMBASSADOR_LAQUATUS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("887cac61-2001-4bd1-aeeb-20149ebc5856"),
    "Ambassador Laquatus",
    crate::card::CardArt::new("887cac61-2001-4bd1-aeeb-20149ebc5856", "Eric Peterson"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 24 — Aquamoeba
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AQUAMOEBA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1243552a-ca57-42ce-817e-d6268fc673e0"),
    "Aquamoeba",
    crate::card::CardArt::new("1243552a-ca57-42ce-817e-d6268fc673e0", "Arnie Swekel"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 25 — Balshan Collaborator
pub(in crate::card::sets) static BALSHAN_COLLABORATOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e23ebd3b-59bf-4f3d-b320-9283871c4540"),
    "Balshan Collaborator",
    CardArt::new("e23ebd3b-59bf-4f3d-b320-9283871c4540", "DiTerlizzi"),
    CardSet::Torment,
    // A blue creature that pumps with black mana, which is the whole of
    // what Torment meant by collaboration.
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Bird", "Soldier"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{B}: This creature gets +1/+1 until end of turn.",
            &[CostDef::Mana(mana_cost!("{B}"))],
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

// TOR 26 — Breakthrough
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BREAKTHROUGH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a59a2cea-3b65-43da-bc6e-0e3c82f25b3c"),
    "Breakthrough",
    crate::card::CardArt::new("a59a2cea-3b65-43da-bc6e-0e3c82f25b3c", "Gary Ruddell"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 27 — Cephalid Aristocrat
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CEPHALID_ARISTOCRAT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bd9ca63d-7e77-48f3-abdc-2d2f9cb3a0d9"),
    "Cephalid Aristocrat",
    crate::card::CardArt::new("bd9ca63d-7e77-48f3-abdc-2d2f9cb3a0d9", "Rob Alexander"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 28 — Cephalid Illusionist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CEPHALID_ILLUSIONIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dceb8cf5-b31a-400e-aea5-ad0c3552d697"),
    "Cephalid Illusionist",
    crate::card::CardArt::new("dceb8cf5-b31a-400e-aea5-ad0c3552d697", "Pete Venters"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 29 — Cephalid Sage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CEPHALID_SAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("434e2ece-7b00-4ad1-9c35-cca16fbb002b"),
    "Cephalid Sage",
    crate::card::CardArt::new("434e2ece-7b00-4ad1-9c35-cca16fbb002b", "Keith Garletts"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 30 — Cephalid Snitch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CEPHALID_SNITCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("33d989b2-0198-4e5b-8aad-ee939191dd28"),
    "Cephalid Snitch",
    crate::card::CardArt::new("33d989b2-0198-4e5b-8aad-ee939191dd28", "Jerry Tiritilli"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 31 — Cephalid Vandal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CEPHALID_VANDAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2500976f-a329-4a51-bcc1-cafe39bc3ccf"),
    "Cephalid Vandal",
    crate::card::CardArt::new(
        "2500976f-a329-4a51-bcc1-cafe39bc3ccf",
        "Alex Horley-Orlandelli",
    ),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 32 — Churning Eddy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHURNING_EDDY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bd28bc8e-842f-4788-92a0-3019e3c2385f"),
    "Churning Eddy",
    crate::card::CardArt::new("bd28bc8e-842f-4788-92a0-3019e3c2385f", "Thomas M. Baxa"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 33 — Circular Logic
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CIRCULAR_LOGIC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cd9198d6-201d-4175-8f70-eef92d7d5bb5"),
    "Circular Logic",
    crate::card::CardArt::new("cd9198d6-201d-4175-8f70-eef92d7d5bb5", "Anthony S. Waters"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 34 — Compulsion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COMPULSION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2d1f56e8-55cb-4b81-9946-9f0f813e3d4a"),
    "Compulsion",
    crate::card::CardArt::new(
        "2d1f56e8-55cb-4b81-9946-9f0f813e3d4a",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 35 — Coral Net
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CORAL_NET: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("abe9cf1e-d398-41e0-8b11-afe1015e4fd9"),
    "Coral Net",
    crate::card::CardArt::new("abe9cf1e-d398-41e0-8b11-afe1015e4fd9", "Roger Raupp"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 36 — Deep Analysis (reprint)

// TOR 37 — False Memories
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FALSE_MEMORIES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3a45ea90-03af-446e-9df3-ef64e9613f2c"),
    "False Memories",
    crate::card::CardArt::new("3a45ea90-03af-446e-9df3-ef64e9613f2c", "Ron Spencer"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 38 — Ghostly Wings
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GHOSTLY_WINGS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("58fb2705-82e7-49d9-b8cc-f98f652dd6c1"),
    "Ghostly Wings",
    crate::card::CardArt::new("58fb2705-82e7-49d9-b8cc-f98f652dd6c1", "David Martin"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 39 — Hydromorph Guardian
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HYDROMORPH_GUARDIAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("40e4ace0-421d-4e8e-ab85-8f74757e99c7"),
    "Hydromorph Guardian",
    crate::card::CardArt::new("40e4ace0-421d-4e8e-ab85-8f74757e99c7", "Glen Angus"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 40 — Hydromorph Gull
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HYDROMORPH_GULL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("06b1c8fc-e09b-479b-ac15-f659acb2f50f"),
    "Hydromorph Gull",
    crate::card::CardArt::new("06b1c8fc-e09b-479b-ac15-f659acb2f50f", "Arnie Swekel"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 41 — Liquify
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIQUIFY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("12fadf25-0995-440d-a3e6-7964ed86cff6"),
    "Liquify",
    crate::card::CardArt::new("12fadf25-0995-440d-a3e6-7964ed86cff6", "Ron Spencer"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 42 — Llawan, Cephalid Empress
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LLAWAN_CEPHALID_EMPRESS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a9821970-a5da-4045-93d8-f58c9e5797c1"),
    "Llawan, Cephalid Empress",
    crate::card::CardArt::new("a9821970-a5da-4045-93d8-f58c9e5797c1", "Mark Zug"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 43 — Obsessive Search
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OBSESSIVE_SEARCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("79977d84-7a20-47ef-930f-0ce7a5eff88e"),
    "Obsessive Search",
    crate::card::CardArt::new("79977d84-7a20-47ef-930f-0ce7a5eff88e", "Jim Nelson"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 44 — Plagiarize
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PLAGIARIZE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("333bbcba-19a7-4307-abe7-7cffa3569e4d"),
    "Plagiarize",
    crate::card::CardArt::new("333bbcba-19a7-4307-abe7-7cffa3569e4d", "Ben Thompson"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 45 — Possessed Aven
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static POSSESSED_AVEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("997c56c7-c625-4209-b4b6-d274dee2db48"),
    "Possessed Aven",
    crate::card::CardArt::new("997c56c7-c625-4209-b4b6-d274dee2db48", "Scott M. Fischer"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 46 — Retraced Image
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RETRACED_IMAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c62fa778-ce44-4a3e-958a-91dfe5398944"),
    "Retraced Image",
    crate::card::CardArt::new("c62fa778-ce44-4a3e-958a-91dfe5398944", "Greg Staples"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 47 — Skywing Aven
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKYWING_AVEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("91a20b6b-43e6-4feb-9792-909332e1a846"),
    "Skywing Aven",
    crate::card::CardArt::new("91a20b6b-43e6-4feb-9792-909332e1a846", "Matt Cavotta"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 48 — Stupefying Touch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STUPEFYING_TOUCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7f817379-9c0b-4e43-a345-492516ccb6e1"),
    "Stupefying Touch",
    crate::card::CardArt::new("7f817379-9c0b-4e43-a345-492516ccb6e1", "Bradley Williams"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 49 — Turbulent Dreams
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TURBULENT_DREAMS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d879226b-8ee3-4a48-b03a-d183594dc586"),
    "Turbulent Dreams",
    crate::card::CardArt::new("d879226b-8ee3-4a48-b03a-d183594dc586", "Wayne England"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 50 — Boneshard Slasher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BONESHARD_SLASHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("27201370-32cc-4b90-890d-8c3f5362ad70"),
    "Boneshard Slasher",
    crate::card::CardArt::new("27201370-32cc-4b90-890d-8c3f5362ad70", "Ron Spencer"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 51 — Cabal Ritual
pub(in crate::card::sets) static CABAL_RITUAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5403b49d-03a7-4cc3-af3c-df098c1c9c2e"),
    "Cabal Ritual",
    CardArt::new("5403b49d-03a7-4cc3-af3c-df098c1c9c2e", "Greg Hildebrandt"),
    CardSet::Torment,
    // Dark Ritual with a late-game mode: two mana for three early, and for
    // five once the graveyard is deep enough to say so.
    CardRules::new_instant(mana_cost!("{1}{B}")).with_ability(AbilityDef::spell(
        "Add {B}{B}{B}.\nThreshold — Add {B}{B}{B}{B}{B} instead if there are seven or more \
         cards in your graveyard.",
        EffectDef::AddMana(
            AddManaEffectDef::one(ManaColor::Black)
                .with_amount(3)
                // Threshold counts the graveyard as the spell resolves, so the cards the
                // turn has already spent are in it -- which is why a storm turn tends to
                // reach seven before it needs the five mana.
                .with_amount_override(
                    ConditionDef::ObjectCount(&ObjectCountConditionDef {
                        query: ObjectQueryDef::matching(
                            ObjectPredicateDef::Any,
                            &[ZoneKind::Graveyard],
                            PlayerRelation::You,
                        ),
                        comparison: ComparisonDef::GreaterOrEqual,
                        amount: 7,
                    }),
                    5,
                ),
        ),
    )),
);

// TOR 52 — Cabal Surgeon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CABAL_SURGEON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("29ce55a2-d838-48ab-862e-d419b0cda3c9"),
    "Cabal Surgeon",
    crate::card::CardArt::new("29ce55a2-d838-48ab-862e-d419b0cda3c9", "Donato Giancola"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 53 — Cabal Torturer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CABAL_TORTURER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ebcf8dd7-f45c-4ac2-9507-fa175fe89887"),
    "Cabal Torturer",
    crate::card::CardArt::new("ebcf8dd7-f45c-4ac2-9507-fa175fe89887", "Pete Venters"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 54 — Carrion Rats
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CARRION_RATS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2efa2579-c048-4506-babc-ec1c29bb99a8"),
    "Carrion Rats",
    crate::card::CardArt::new(
        "2efa2579-c048-4506-babc-ec1c29bb99a8",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 55 — Carrion Wurm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CARRION_WURM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("37c2b228-94c0-4e84-ad6d-80b170bb6c0c"),
    "Carrion Wurm",
    crate::card::CardArt::new("37c2b228-94c0-4e84-ad6d-80b170bb6c0c", "Glen Angus"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 56 — Chainer, Dementia Master
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHAINER_DEMENTIA_MASTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("40342b11-1005-4ccc-bef4-9ea4c640b048"),
    "Chainer, Dementia Master",
    crate::card::CardArt::new("40342b11-1005-4ccc-bef4-9ea4c640b048", "Mark Zug"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 57 — Chainer's Edict
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHAINER_S_EDICT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("014af391-3b4a-4aab-a3e9-f60e61016985"),
    "Chainer's Edict",
    crate::card::CardArt::new("014af391-3b4a-4aab-a3e9-f60e61016985", "Ben Thompson"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 58 — Crippling Fatigue
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRIPPLING_FATIGUE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0bb53a87-ba48-4c77-b284-3be321c8836e"),
    "Crippling Fatigue",
    crate::card::CardArt::new("0bb53a87-ba48-4c77-b284-3be321c8836e", "Heather Hudson"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 59 — Dawn of the Dead
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DAWN_OF_THE_DEAD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("baabadd3-3fc1-4c9a-ad3b-c79d60fb3390"),
    "Dawn of the Dead",
    crate::card::CardArt::new("baabadd3-3fc1-4c9a-ad3b-c79d60fb3390", "Pete Venters"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 60 — Faceless Butcher
pub(in crate::card::sets) static FACELESS_BUTCHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4073be21-c54a-4eee-9109-f3adfe757c4e"),
    "Faceless Butcher",
    CardArt::new("4073be21-c54a-4eee-9109-f3adfe757c4e", "Daren Bader"),
    CardSet::Torment,
    // The exile is not a "may" and not optional: with only its own body on
    // the board the Butcher has to eat something, and killing it hands the
    // creature back.
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Nightmare", "Horror"], 2, 3)
        .with_abilities(&[
            abilities::enters_trigger_with_targets(
                "When this creature enters, exile another target creature.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        zones: &[ZoneKind::Battlefield],
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
            AbilityDef::triggered(
                "When this creature leaves the battlefield, return the exiled card to the battlefield under its owner's control.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    None,
                ),
                EffectDef::ReturnLinkedExiles {
                    object: ObjectPredicateDef::Any,
                    counters: None,
                    zone: ZoneKind::Battlefield,
                    grant: None,
                    controller: None,
                    transformed: false,
                },
            ),
        ]),
);

// TOR 61 — Gloomdrifter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLOOMDRIFTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("baaf8c24-3f5e-4ea5-941b-40ad1299af39"),
    "Gloomdrifter",
    crate::card::CardArt::new("baaf8c24-3f5e-4ea5-941b-40ad1299af39", "Adam Rex"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 62 — Gravegouger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRAVEGOUGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("acfcd559-374e-4e6f-9333-2e5c855abff5"),
    "Gravegouger",
    crate::card::CardArt::new("acfcd559-374e-4e6f-9333-2e5c855abff5", "Daren Bader"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 63 — Grotesque Hybrid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GROTESQUE_HYBRID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6b063c3a-267f-4f22-be51-0a14880afc24"),
    "Grotesque Hybrid",
    crate::card::CardArt::new("6b063c3a-267f-4f22-be51-0a14880afc24", "Terese Nielsen"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 64 — Hypnox
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HYPNOX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0e0e53a1-11d5-4975-b698-8b5d72d241f0"),
    "Hypnox",
    crate::card::CardArt::new("0e0e53a1-11d5-4975-b698-8b5d72d241f0", "Greg Staples"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 65 — Ichorid
pub(in crate::card::sets) static ICHORID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("97431dca-54ca-47ef-ab00-943140e8e758"),
    "Ichorid",
    crate::card::CardArt::new("97431dca-54ca-47ef-ab00-943140e8e758", "rk post"),
    crate::card::CardSet::Torment,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Horror"], 3, 1).with_abilities(&[
        abilities::haste(),
        AbilityDef::triggered(
            "At the beginning of the end step, sacrifice this creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::Any,
            },
            EffectDef::Sacrifice {
                object: EffectRecipientDef::Source,
            },
        ),
        AbilityDef::triggered_if(
            "At the beginning of your upkeep, if this card is in your graveyard, you may exile a black creature card other than this card from your graveyard. If you do, return this card to the battlefield.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            &const { TriggerConditionDef::SourceInZone(ZoneKind::Graveyard) },
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Object(ParentBinding),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Color(ManaColor::Black),
                    ]),
                    &[ZoneKind::Graveyard],
                    PlayerRelation::You,
                )),
                exclude: Some(ObjectRefDef::Source),
                minimum: 0,
                maximum: 1,
                visibility: ChoiceVisibilityDef::Public,
                then: &const {
                    EffectDef::IfCondition {
                        condition: &const {
                            TriggerConditionDef::BoundObjectMatches {
                                binding: ParentBinding,
                                object: ObjectPredicateDef::Any,
                            }
                        },
                        then: &const {
                            EffectDef::Sequence(&const {
                                [
                                    EffectDef::MoveToZone {
                                        object: EffectRecipientDef::object(ObjectRefDef::Binding(
                                            ParentBinding,
                                        )),
                                        zone: ZoneKind::Exile,
                                        placement: ZonePlacement::Top,
                                    },
                                    EffectDef::MoveToZone {
                                        object: EffectRecipientDef::Source,
                                        zone: ZoneKind::Battlefield,
                                        placement: ZonePlacement::Top,
                                    },
                                ]
                            })
                        },
                    }
                },
            }),
        )
        .with_source_zones(&[ZoneKind::Graveyard]),
    ]),
);

// TOR 66 — Insidious Dreams
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INSIDIOUS_DREAMS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e8a29622-23a6-42c7-8e56-690613572c94"),
    "Insidious Dreams",
    crate::card::CardArt::new("e8a29622-23a6-42c7-8e56-690613572c94", "John Avon"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 67 — Laquatus's Champion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAQUATUS_S_CHAMPION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7f995525-7e82-45fc-8e55-76607487151a"),
    "Laquatus's Champion",
    crate::card::CardArt::new("74a397bb-6910-4724-833d-7f2d92723e3b", "Greg Staples"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 68 — Last Laugh
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAST_LAUGH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d4c05f2a-b844-42b3-af91-65694b4211dd"),
    "Last Laugh",
    crate::card::CardArt::new("d4c05f2a-b844-42b3-af91-65694b4211dd", "John Matson"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 69 — Mesmeric Fiend
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MESMERIC_FIEND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b6edd4ea-c587-4d93-a675-4cdec3e0b1ca"),
    "Mesmeric Fiend",
    crate::card::CardArt::new("b6edd4ea-c587-4d93-a675-4cdec3e0b1ca", "Dana Knutson"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 70 — Mind Sludge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIND_SLUDGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("34bf3c19-f148-4542-8505-604191fd5a02"),
    "Mind Sludge",
    crate::card::CardArt::new("34bf3c19-f148-4542-8505-604191fd5a02", "Eric Peterson"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 71 — Mortal Combat
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MORTAL_COMBAT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("21146226-ff05-40d5-bc4e-42f2263104b1"),
    "Mortal Combat",
    crate::card::CardArt::new("21146226-ff05-40d5-bc4e-42f2263104b1", "Mike Ploog"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 72 — Mortiphobia
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MORTIPHOBIA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c7b93caa-0f43-43ac-84e9-99f6bcbf9e81"),
    "Mortiphobia",
    crate::card::CardArt::new(
        "c7b93caa-0f43-43ac-84e9-99f6bcbf9e81",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 73 — Mutilate (reprint)

// TOR 74 — Nantuko Shade
pub(in crate::card::sets) static NANTUKO_SHADE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2ed9dc9c-b92b-4305-8c54-1a63f750f8d1"),
    "Nantuko Shade",
    CardArt::new("2ed9dc9c-b92b-4305-8c54-1a63f750f8d1", "Brian Snõddy"),
    CardSet::Torment,
    // Two mana for a 2/1 that grows without limit. In a mono-black deck
    // every untapped Swamp is another point of damage.
    CardRules::new_creature(mana_cost!("{B}{B}"), &["Insect", "Shade"], 2, 1).with_ability(
        AbilityDef::activated(
            "{B}: This creature gets +1/+1 until end of turn.",
            &[CostDef::Mana(mana_cost!("{B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// TOR 75 — Organ Grinder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORGAN_GRINDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6233d4c3-9407-41b0-92a9-5f90dfd6a584"),
    "Organ Grinder",
    crate::card::CardArt::new("6233d4c3-9407-41b0-92a9-5f90dfd6a584", "Adam Rex"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 76 — Psychotic Haze
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PSYCHOTIC_HAZE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8d3f6cd2-0138-40e7-a975-3f7c68db0d93"),
    "Psychotic Haze",
    crate::card::CardArt::new(
        "8d3f6cd2-0138-40e7-a975-3f7c68db0d93",
        "Alex Horley-Orlandelli",
    ),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 77 — Putrid Imp
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PUTRID_IMP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1b9e6c5c-4bc4-4f41-8c2c-f5b8c97c53c5"),
    "Putrid Imp",
    crate::card::CardArt::new("1b9e6c5c-4bc4-4f41-8c2c-f5b8c97c53c5", "Wayne England"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 78 — Rancid Earth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RANCID_EARTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("23d07a96-85ba-4714-94a5-4a8125954f58"),
    "Rancid Earth",
    crate::card::CardArt::new("23d07a96-85ba-4714-94a5-4a8125954f58", "Ciruelo"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 79 — Restless Dreams
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RESTLESS_DREAMS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("63795680-5310-4c99-92c1-70e453391de9"),
    "Restless Dreams",
    crate::card::CardArt::new("63795680-5310-4c99-92c1-70e453391de9", "John Matson"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 80 — Sengir Vampire (reprint)

// TOR 81 — Shade's Form
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHADE_S_FORM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("449baad3-9a4a-4f5f-9e9f-02cc97ab71e6"),
    "Shade's Form",
    crate::card::CardArt::new("449baad3-9a4a-4f5f-9e9f-02cc97ab71e6", "Clyde Caldwell"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 82 — Shambling Swarm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHAMBLING_SWARM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b5b93715-985f-4719-a3c3-044c2a150e96"),
    "Shambling Swarm",
    crate::card::CardArt::new("b5b93715-985f-4719-a3c3-044c2a150e96", "Arnie Swekel"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 83 — Sickening Dreams
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SICKENING_DREAMS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9396ac77-9f53-46bd-b126-02441a0f5594"),
    "Sickening Dreams",
    crate::card::CardArt::new("9396ac77-9f53-46bd-b126-02441a0f5594", "Scott M. Fischer"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 84 — Slithery Stalker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SLITHERY_STALKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0c875bb6-55bb-41cc-8825-8011581ff001"),
    "Slithery Stalker",
    crate::card::CardArt::new("0c875bb6-55bb-41cc-8825-8011581ff001", "John Avon"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 85 — Soul Scourge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOUL_SCOURGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4aa7d91b-cfbb-4da0-97fa-3c0a15f1dbb9"),
    "Soul Scourge",
    crate::card::CardArt::new("4aa7d91b-cfbb-4da0-97fa-3c0a15f1dbb9", "Carl Critchlow"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 86 — Strength of Lunacy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STRENGTH_OF_LUNACY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("afda26e7-4fdd-45e1-bcd1-f0abf91979aa"),
    "Strength of Lunacy",
    crate::card::CardArt::new("afda26e7-4fdd-45e1-bcd1-f0abf91979aa", "Greg Hildebrandt"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 87 — Unhinge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNHINGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b89deafd-cb7c-4da7-ab9b-8f795554a705"),
    "Unhinge",
    crate::card::CardArt::new("b89deafd-cb7c-4da7-ab9b-8f795554a705", "Keith Garletts"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 88 — Waste Away
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WASTE_AWAY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("78e859c2-3457-4c4c-9b1c-0db647dcf259"),
    "Waste Away",
    crate::card::CardArt::new("78e859c2-3457-4c4c-9b1c-0db647dcf259", "Alan Pollack"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 89 — Zombie Trailblazer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ZOMBIE_TRAILBLAZER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("82e4675f-ebec-483f-a111-b505629760fd"),
    "Zombie Trailblazer",
    crate::card::CardArt::new("82e4675f-ebec-483f-a111-b505629760fd", "Brian Snõddy"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 90 — Accelerate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ACCELERATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6a28dd88-db90-4f02-8aa9-39051d2c4763"),
    "Accelerate",
    crate::card::CardArt::new("6a28dd88-db90-4f02-8aa9-39051d2c4763", "Gary Ruddell"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 91 — Balthor the Stout
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BALTHOR_THE_STOUT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e81ecdc5-d2c7-4292-9b59-fd6bf3ba29d5"),
    "Balthor the Stout",
    crate::card::CardArt::new("e81ecdc5-d2c7-4292-9b59-fd6bf3ba29d5", "Ron Spears"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 92 — Barbarian Outcast
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARBARIAN_OUTCAST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b9d67b5c-ab20-456e-8ff5-7521be8273b2"),
    "Barbarian Outcast",
    crate::card::CardArt::new("b9d67b5c-ab20-456e-8ff5-7521be8273b2", "Mark Tedin"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 93 — Crackling Club
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRACKLING_CLUB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0810f7e3-03ff-4c46-a88f-2f8144540780"),
    "Crackling Club",
    crate::card::CardArt::new("0810f7e3-03ff-4c46-a88f-2f8144540780", "Mike Ploog"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 94 — Crazed Firecat
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRAZED_FIRECAT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5ef809de-ff64-4832-95dd-2ebda5942df8"),
    "Crazed Firecat",
    crate::card::CardArt::new("5ef809de-ff64-4832-95dd-2ebda5942df8", "Ron Spears"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 95 — Devastating Dreams
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEVASTATING_DREAMS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9fffeed0-a5ea-47ac-a7a4-0cc3bb1d408a"),
    "Devastating Dreams",
    crate::card::CardArt::new("9fffeed0-a5ea-47ac-a7a4-0cc3bb1d408a", "Tony Szczudlo"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 96 — Enslaved Dwarf
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENSLAVED_DWARF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("da5003a6-211e-43d3-9a3c-756496357163"),
    "Enslaved Dwarf",
    crate::card::CardArt::new("da5003a6-211e-43d3-9a3c-756496357163", "Terese Nielsen"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 97 — Fiery Temper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FIERY_TEMPER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("918e46b7-cbca-4acf-8e83-94b5fcadcc49"),
    "Fiery Temper",
    crate::card::CardArt::new("918e46b7-cbca-4acf-8e83-94b5fcadcc49", "Greg Hildebrandt"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 98 — Flaming Gambit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLAMING_GAMBIT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fb7fd9b7-c394-4ab3-b945-b4aab694eb6a"),
    "Flaming Gambit",
    crate::card::CardArt::new("fb7fd9b7-c394-4ab3-b945-b4aab694eb6a", "Donato Giancola"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 99 — Flash of Defiance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLASH_OF_DEFIANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3e550776-b806-4f66-8aab-e6a5dac6d5cc"),
    "Flash of Defiance",
    crate::card::CardArt::new("3e550776-b806-4f66-8aab-e6a5dac6d5cc", "Carl Critchlow"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 100 — Grim Lavamancer
pub(in crate::card::sets) static GRIM_LAVAMANCER: CardRecord = CardRecord::new_with_legacy_id(
    2036,
    "Grim Lavamancer",
    CardArt::new("5dd72697-24be-42c7-a6d9-a837bdbd4662", "Jim Nelson"),
    CardSet::Torment,
    // The graveyard is the limit: two cards a turn is the rate, and a deck
    // that empties its hand quickly is the one that can pay it.
    CardRules::new_creature(mana_cost!("{R}"), &["Human", "Wizard"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{R}, {T}, Exile two cards from your graveyard: This creature deals 2 damage to any target.",
            &[
                CostDef::Mana(mana_cost!("{R}")),
                CostDef::TapSource,
                CostDef::MoveToZone(crate::card::MoveToZoneCostDef::new(
                    ObjectPredicateDef::Any,
                    ZoneKind::Graveyard,
                    ZoneKind::Exile,
                    2,
                )),
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

// TOR 101 — Hell-Bent Raider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HELL_BENT_RAIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7f41ac1c-2603-4234-8963-cd47bb894024"),
    "Hell-Bent Raider",
    crate::card::CardArt::new("7f41ac1c-2603-4234-8963-cd47bb894024", "Mike Ploog"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 102 — Kamahl's Sledge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KAMAHL_S_SLEDGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("38c55518-7bdf-4a42-ae30-cd6525557a59"),
    "Kamahl's Sledge",
    crate::card::CardArt::new("38c55518-7bdf-4a42-ae30-cd6525557a59", "Don Hazeltine"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 103 — Longhorn Firebeast
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LONGHORN_FIREBEAST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bf0dcf33-8d3f-429c-8ad8-a65d07d7c790"),
    "Longhorn Firebeast",
    crate::card::CardArt::new("bf0dcf33-8d3f-429c-8ad8-a65d07d7c790", "Glen Angus"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 104 — Overmaster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OVERMASTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("afd2e0f0-8a8e-4021-bd24-aff1a3212345"),
    "Overmaster",
    crate::card::CardArt::new("afd2e0f0-8a8e-4021-bd24-aff1a3212345", "Anthony S. Waters"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 105 — Pardic Arsonist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PARDIC_ARSONIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1c2538fa-2eb7-47ca-a1cf-e5546e26e584"),
    "Pardic Arsonist",
    crate::card::CardArt::new("1c2538fa-2eb7-47ca-a1cf-e5546e26e584", "rk post"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 106 — Pardic Collaborator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PARDIC_COLLABORATOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a9a60f33-1d1a-4c7c-9eb2-d9fc0d56b127"),
    "Pardic Collaborator",
    crate::card::CardArt::new("a9a60f33-1d1a-4c7c-9eb2-d9fc0d56b127", "Pete Venters"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 107 — Pardic Lancer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PARDIC_LANCER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f487a6a7-066a-49bb-ab76-d79fc4300c29"),
    "Pardic Lancer",
    crate::card::CardArt::new("f487a6a7-066a-49bb-ab76-d79fc4300c29", "Justin Sweet"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 108 — Petradon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PETRADON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("75ac6311-8516-4db2-8c1f-626f0db0d36f"),
    "Petradon",
    crate::card::CardArt::new("75ac6311-8516-4db2-8c1f-626f0db0d36f", "Jim Nelson"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 109 — Petravark
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PETRAVARK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ffc98d09-439e-426b-8403-4a3e12167336"),
    "Petravark",
    crate::card::CardArt::new("ffc98d09-439e-426b-8403-4a3e12167336", "Wayne England"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 110 — Pitchstone Wall
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PITCHSTONE_WALL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9bfea263-6ff0-41be-9755-03bcdebfb255"),
    "Pitchstone Wall",
    crate::card::CardArt::new("9bfea263-6ff0-41be-9755-03bcdebfb255", "David Martin"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 111 — Possessed Barbarian
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static POSSESSED_BARBARIAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("57e79610-8cfe-4711-bc71-8c25d68036da"),
    "Possessed Barbarian",
    crate::card::CardArt::new("57e79610-8cfe-4711-bc71-8c25d68036da", "Scott M. Fischer"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 112 — Pyromania
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PYROMANIA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6a5af4e9-a2b6-43c7-8ff4-e761fbf693a7"),
    "Pyromania",
    crate::card::CardArt::new(
        "6a5af4e9-a2b6-43c7-8ff4-e761fbf693a7",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 113 — Radiate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RADIATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2562c25a-999e-4fb5-a595-f376c8abf1ff"),
    "Radiate",
    crate::card::CardArt::new("2562c25a-999e-4fb5-a595-f376c8abf1ff", "Carl Critchlow"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 114 — Skullscorch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKULLSCORCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("88f1343c-77bf-4f44-8226-fdfb2c2c7015"),
    "Skullscorch",
    crate::card::CardArt::new("88f1343c-77bf-4f44-8226-fdfb2c2c7015", "Bradley Williams"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 115 — Sonic Seizure
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SONIC_SEIZURE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("98eb9371-aa20-4790-baf8-a1ad95de39de"),
    "Sonic Seizure",
    crate::card::CardArt::new("98eb9371-aa20-4790-baf8-a1ad95de39de", "Terese Nielsen"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 116 — Temporary Insanity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEMPORARY_INSANITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("415d0678-539f-4c19-8a6e-b43e747a97de"),
    "Temporary Insanity",
    crate::card::CardArt::new("415d0678-539f-4c19-8a6e-b43e747a97de", "Mark Romanoski"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 117 — Violent Eruption
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VIOLENT_ERUPTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ff19c870-331d-4703-a92d-c5b1d4289338"),
    "Violent Eruption",
    crate::card::CardArt::new("ff19c870-331d-4703-a92d-c5b1d4289338", "Bob Petillo"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 118 — Acorn Harvest
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ACORN_HARVEST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("32987720-cc0c-416b-b79b-217d3b37542d"),
    "Acorn Harvest",
    crate::card::CardArt::new(
        "32987720-cc0c-416b-b79b-217d3b37542d",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 119 — Anurid Scavenger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANURID_SCAVENGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("21a21190-3c05-40fe-9310-493ed0f9e42e"),
    "Anurid Scavenger",
    crate::card::CardArt::new("21a21190-3c05-40fe-9310-493ed0f9e42e", "Bob Petillo"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 120 — Arrogant Wurm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARROGANT_WURM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("11b849c7-c91d-4c67-a357-f7d17f9b187a"),
    "Arrogant Wurm",
    crate::card::CardArt::new("11b849c7-c91d-4c67-a357-f7d17f9b187a", "John Avon"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 121 — Basking Rootwalla
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BASKING_ROOTWALLA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1a67768a-6cd9-4163-b941-752f29c87a8d"),
    "Basking Rootwalla",
    crate::card::CardArt::new("1a67768a-6cd9-4163-b941-752f29c87a8d", "Heather Hudson"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 122 — Centaur Chieftain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CENTAUR_CHIEFTAIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a8fa0a57-e7eb-454e-9527-3db702b54935"),
    "Centaur Chieftain",
    crate::card::CardArt::new("a8fa0a57-e7eb-454e-9527-3db702b54935", "Justin Sweet"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 123 — Centaur Veteran
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CENTAUR_VETERAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a83903c1-40fe-47a5-8bcc-2f0877c58225"),
    "Centaur Veteran",
    crate::card::CardArt::new("a83903c1-40fe-47a5-8bcc-2f0877c58225", "Mark Zug"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 124 — Dwell on the Past
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DWELL_ON_THE_PAST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("77ac905a-839b-4fb5-8c17-7fd6c2c0c492"),
    "Dwell on the Past",
    crate::card::CardArt::new("77ac905a-839b-4fb5-8c17-7fd6c2c0c492", "Rebecca Guay"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 125 — Far Wanderings
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FAR_WANDERINGS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bd3a735a-5f51-41af-99d1-92296cec7b22"),
    "Far Wanderings",
    crate::card::CardArt::new("bd3a735a-5f51-41af-99d1-92296cec7b22", "Darrell Riche"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 126 — Gurzigost
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GURZIGOST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f4e672c6-6ddc-4dd2-b4c7-5083d7566e87"),
    "Gurzigost",
    crate::card::CardArt::new("f4e672c6-6ddc-4dd2-b4c7-5083d7566e87", "Scott M. Fischer"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 127 — Insist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INSIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("48e1f72d-ef5e-4cea-a0ee-645570a36104"),
    "Insist",
    crate::card::CardArt::new("48e1f72d-ef5e-4cea-a0ee-645570a36104", "Franz Vohwinkel"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 128 — Invigorating Falls
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INVIGORATING_FALLS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ec74e797-390e-4ebd-a53b-098ef3edd7d1"),
    "Invigorating Falls",
    crate::card::CardArt::new("ec74e797-390e-4ebd-a53b-098ef3edd7d1", "Rebecca Guay"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 129 — Krosan Constrictor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KROSAN_CONSTRICTOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f24b3479-18c3-4c43-b8d1-8b4a25d271a7"),
    "Krosan Constrictor",
    crate::card::CardArt::new("f24b3479-18c3-4c43-b8d1-8b4a25d271a7", "Jim Nelson"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 130 — Krosan Restorer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KROSAN_RESTORER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("06f9f54d-b8c7-407d-bc25-dad4db833208"),
    "Krosan Restorer",
    crate::card::CardArt::new("06f9f54d-b8c7-407d-bc25-dad4db833208", "Clyde Caldwell"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 131 — Nantuko Blightcutter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NANTUKO_BLIGHTCUTTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("42a6f1b6-e1ed-497a-b50e-1bfa257c18c0"),
    "Nantuko Blightcutter",
    crate::card::CardArt::new("42a6f1b6-e1ed-497a-b50e-1bfa257c18c0", "Matt Cavotta"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 132 — Nantuko Calmer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NANTUKO_CALMER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c76b1198-feff-437c-9d4c-94785144c98f"),
    "Nantuko Calmer",
    crate::card::CardArt::new("c76b1198-feff-437c-9d4c-94785144c98f", "Mark Romanoski"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 133 — Nantuko Cultivator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NANTUKO_CULTIVATOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9d258fe7-7906-43ca-8ebd-344aa81cb85b"),
    "Nantuko Cultivator",
    crate::card::CardArt::new("9d258fe7-7906-43ca-8ebd-344aa81cb85b", "Darrell Riche"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 134 — Narcissism
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NARCISSISM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("eea658dd-7567-4b93-88a4-08b4ffb3dad7"),
    "Narcissism",
    crate::card::CardArt::new(
        "eea658dd-7567-4b93-88a4-08b4ffb3dad7",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 135 — Nostalgic Dreams
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NOSTALGIC_DREAMS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("70f1e794-a32e-4f91-acbb-7e60dad4cf53"),
    "Nostalgic Dreams",
    crate::card::CardArt::new("70f1e794-a32e-4f91-acbb-7e60dad4cf53", "Darrell Riche"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 136 — Parallel Evolution
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PARALLEL_EVOLUTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("73cce010-a8e7-477b-9179-bbad38aa6438"),
    "Parallel Evolution",
    crate::card::CardArt::new("73cce010-a8e7-477b-9179-bbad38aa6438", "Matt Cavotta"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 137 — Possessed Centaur
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static POSSESSED_CENTAUR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("608e11b2-9636-48a0-8705-f5ce3bc98117"),
    "Possessed Centaur",
    crate::card::CardArt::new(
        "608e11b2-9636-48a0-8705-f5ce3bc98117",
        "Alex Horley-Orlandelli",
    ),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 138 — Seton's Scout
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SETON_S_SCOUT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("732bf7bb-5326-4742-8311-96ddbfe14b38"),
    "Seton's Scout",
    crate::card::CardArt::new("732bf7bb-5326-4742-8311-96ddbfe14b38", "Mark Romanoski"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 139 — Cabal Coffers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CABAL_COFFERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2b934c78-258e-4b1e-9783-ec9f734e8776"),
    "Cabal Coffers",
    crate::card::CardArt::new("2b934c78-258e-4b1e-9783-ec9f734e8776", "Don Hazeltine"),
    crate::card::CardSet::Torment,
    crate::card::CardRules::unsupported(),
);

// TOR 140 — Tainted Field
pub(in crate::card::sets) static TAINTED_FIELD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("75b5f0aa-1570-4064-ad20-aac7be8b2c9c"),
    "Tainted Field",
    CardArt::new("75b5f0aa-1570-4064-ad20-aac7be8b2c9c", "Don Hazeltine"),
    CardSet::Torment,
    // Colourless for free, or two real colours once a Swamp is out. The
    // black deck's fixing, priced on already being the black deck.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_mana_if(
            "{T}: Add {W} or {B}. Activate only if you control a Swamp.",
            &[CostDef::TapSource],
            // A subtype check rather than a card name, so any land that
            // is a Swamp turns it on -- including this cycle's own
            // partners in a deck that runs two of them.
            &TriggerConditionDef::controls_basic_land_type(
                PlayerRelation::You,
                BasicLandType::Swamp,
            ),
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Black,
            ])),
        ),
    ]),
);

// TOR 141 — Tainted Isle
pub(in crate::card::sets) static TAINTED_ISLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b462e121-015c-49c4-838a-ab788f213322"),
    "Tainted Isle",
    CardArt::new("b462e121-015c-49c4-838a-ab788f213322", "Alan Pollack"),
    CardSet::Torment,
    // The blue member of the same cycle.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_mana_if(
            "{T}: Add {U} or {B}. Activate only if you control a Swamp.",
            &[CostDef::TapSource],
            // A subtype check rather than a card name, so any land that
            // is a Swamp turns it on -- including this cycle's own
            // partners in a deck that runs two of them.
            &TriggerConditionDef::controls_basic_land_type(
                PlayerRelation::You,
                BasicLandType::Swamp,
            ),
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Blue,
                ManaColor::Black,
            ])),
        ),
    ]),
);

// TOR 142 — Tainted Peak
pub(in crate::card::sets) static TAINTED_PEAK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4dcaaabe-e1d7-4047-9960-79178af3d903"),
    "Tainted Peak",
    CardArt::new("4dcaaabe-e1d7-4047-9960-79178af3d903", "Tony Szczudlo"),
    CardSet::Torment,
    // The red member of the same cycle.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_mana_if(
            "{T}: Add {B} or {R}. Activate only if you control a Swamp.",
            &[CostDef::TapSource],
            // A subtype check rather than a card name, so any land that
            // is a Swamp turns it on -- including this cycle's own
            // partners in a deck that runs two of them.
            &TriggerConditionDef::controls_basic_land_type(
                PlayerRelation::You,
                BasicLandType::Swamp,
            ),
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Black,
                ManaColor::Red,
            ])),
        ),
    ]),
);

// TOR 143 — Tainted Wood
pub(in crate::card::sets) static TAINTED_WOOD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a20a35cc-69e5-42b8-b28c-ae5147451150"),
    "Tainted Wood",
    CardArt::new("a20a35cc-69e5-42b8-b28c-ae5147451150", "Rob Alexander"),
    CardSet::Torment,
    // The green member of the same cycle.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_mana_if(
            "{T}: Add {B} or {G}. Activate only if you control a Swamp.",
            &[CostDef::TapSource],
            // A subtype check rather than a card name, so any land that
            // is a Swamp turns it on -- including this cycle's own
            // partners in a deck that runs two of them.
            &TriggerConditionDef::controls_basic_land_type(
                PlayerRelation::You,
                BasicLandType::Swamp,
            ),
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Black,
                ManaColor::Green,
            ])),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ANGEL_OF_RETRIBUTION,
    &AVEN_TROOPER,
    &CLEANSING_MEDITATION,
    &EQUAL_TREATMENT,
    &FLOATING_SHIELD,
    &FRANTIC_PURIFICATION,
    &HYPOCHONDRIA,
    &MAJOR_TEROH,
    &MILITANT_MONK,
    &MORNINGTIDE,
    &MYSTIC_FAMILIAR,
    &PAY_NO_HEED,
    &POSSESSED_NOMAD,
    &REBORN_HERO,
    &SPIRIT_FLARE,
    &STERN_JUDGE,
    &STRENGTH_OF_ISOLATION,
    &TEROH_S_FAITHFUL,
    &TEROH_S_VANGUARD,
    &TRANSCENDENCE,
    &VENGEFUL_DREAMS,
    &ALTER_REALITY,
    &AMBASSADOR_LAQUATUS,
    &AQUAMOEBA,
    &BALSHAN_COLLABORATOR,
    &BREAKTHROUGH,
    &CEPHALID_ARISTOCRAT,
    &CEPHALID_ILLUSIONIST,
    &CEPHALID_SAGE,
    &CEPHALID_SNITCH,
    &CEPHALID_VANDAL,
    &CHURNING_EDDY,
    &CIRCULAR_LOGIC,
    &COMPULSION,
    &CORAL_NET,
    &FALSE_MEMORIES,
    &GHOSTLY_WINGS,
    &HYDROMORPH_GUARDIAN,
    &HYDROMORPH_GULL,
    &LIQUIFY,
    &LLAWAN_CEPHALID_EMPRESS,
    &OBSESSIVE_SEARCH,
    &PLAGIARIZE,
    &POSSESSED_AVEN,
    &RETRACED_IMAGE,
    &SKYWING_AVEN,
    &STUPEFYING_TOUCH,
    &TURBULENT_DREAMS,
    &BONESHARD_SLASHER,
    &CABAL_RITUAL,
    &CABAL_SURGEON,
    &CABAL_TORTURER,
    &CARRION_RATS,
    &CARRION_WURM,
    &CHAINER_DEMENTIA_MASTER,
    &CHAINER_S_EDICT,
    &CRIPPLING_FATIGUE,
    &DAWN_OF_THE_DEAD,
    &FACELESS_BUTCHER,
    &GLOOMDRIFTER,
    &GRAVEGOUGER,
    &GROTESQUE_HYBRID,
    &HYPNOX,
    &ICHORID,
    &INSIDIOUS_DREAMS,
    &LAQUATUS_S_CHAMPION,
    &LAST_LAUGH,
    &MESMERIC_FIEND,
    &MIND_SLUDGE,
    &MORTAL_COMBAT,
    &MORTIPHOBIA,
    &NANTUKO_SHADE,
    &ORGAN_GRINDER,
    &PSYCHOTIC_HAZE,
    &PUTRID_IMP,
    &RANCID_EARTH,
    &RESTLESS_DREAMS,
    &SHADE_S_FORM,
    &SHAMBLING_SWARM,
    &SICKENING_DREAMS,
    &SLITHERY_STALKER,
    &SOUL_SCOURGE,
    &STRENGTH_OF_LUNACY,
    &UNHINGE,
    &WASTE_AWAY,
    &ZOMBIE_TRAILBLAZER,
    &ACCELERATE,
    &BALTHOR_THE_STOUT,
    &BARBARIAN_OUTCAST,
    &CRACKLING_CLUB,
    &CRAZED_FIRECAT,
    &DEVASTATING_DREAMS,
    &ENSLAVED_DWARF,
    &FIERY_TEMPER,
    &FLAMING_GAMBIT,
    &FLASH_OF_DEFIANCE,
    &GRIM_LAVAMANCER,
    &HELL_BENT_RAIDER,
    &KAMAHL_S_SLEDGE,
    &LONGHORN_FIREBEAST,
    &OVERMASTER,
    &PARDIC_ARSONIST,
    &PARDIC_COLLABORATOR,
    &PARDIC_LANCER,
    &PETRADON,
    &PETRAVARK,
    &PITCHSTONE_WALL,
    &POSSESSED_BARBARIAN,
    &PYROMANIA,
    &RADIATE,
    &SKULLSCORCH,
    &SONIC_SEIZURE,
    &TEMPORARY_INSANITY,
    &VIOLENT_ERUPTION,
    &ACORN_HARVEST,
    &ANURID_SCAVENGER,
    &ARROGANT_WURM,
    &BASKING_ROOTWALLA,
    &CENTAUR_CHIEFTAIN,
    &CENTAUR_VETERAN,
    &DWELL_ON_THE_PAST,
    &FAR_WANDERINGS,
    &GURZIGOST,
    &INSIST,
    &INVIGORATING_FALLS,
    &KROSAN_CONSTRICTOR,
    &KROSAN_RESTORER,
    &NANTUKO_BLIGHTCUTTER,
    &NANTUKO_CALMER,
    &NANTUKO_CULTIVATOR,
    &NARCISSISM,
    &NOSTALGIC_DREAMS,
    &PARALLEL_EVOLUTION,
    &POSSESSED_CENTAUR,
    &SETON_S_SCOUT,
    &CABAL_COFFERS,
    &TAINTED_FIELD,
    &TAINTED_ISLE,
    &TAINTED_PEAK,
    &TAINTED_WOOD,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&catalog_ema::DEEP_ANALYSIS), // TOR 36
    PrintingRecord::reprint(&catalog_m13::MUTILATE),      // TOR 73
    PrintingRecord::reprint(&catalog_lea::SENGIR_VAMPIRE), // TOR 80
];
