//! Mirrodin Besieged cards cataloged as cross-format rules-engine test cases.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AppliedEffectDef,
    CardArt, CardRules, CardSet, EffectDef, EffectRecipientDef, ManaColor, ReplacementEffectDef,
    ReplacementEventDef, SpellResolutionDestinationDef, TokenCharacteristics, ValueDef, ZoneKind,
    ZoneMoveCauseDef, abilities,
};
use crate::{TargetIndex, mana_cost};

// MBS 1 — Accorder Paladin
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ACCORDER_PALADIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("df0a4370-729d-40e7-b68b-21902648492d"),
    "Accorder Paladin",
    crate::card::CardArt::new("df0a4370-729d-40e7-b68b-21902648492d", "Kekai Kotaki"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 2 — Ardent Recruit
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ARDENT_RECRUIT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("69c42dac-f0b3-41aa-b3b2-f203e265131d"),
    "Ardent Recruit",
    crate::card::CardArt::new("69c42dac-f0b3-41aa-b3b2-f203e265131d", "Mike Bierek"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 3 — Banishment Decree
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BANISHMENT_DECREE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2c5f605c-9d16-493e-bc44-0e15bdf8c0bf"),
    "Banishment Decree",
    crate::card::CardArt::new("2c5f605c-9d16-493e-bc44-0e15bdf8c0bf", "James Ryman"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 4 — Choking Fumes
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CHOKING_FUMES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1b9af543-5273-4754-ab7d-75d0b632240f"),
    "Choking Fumes",
    crate::card::CardArt::new("1b9af543-5273-4754-ab7d-75d0b632240f", "Scott Chou"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 6 — Frantic Salvage
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FRANTIC_SALVAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aff909bc-0bda-4e8a-b7a3-ebc963552246"),
    "Frantic Salvage",
    crate::card::CardArt::new("aff909bc-0bda-4e8a-b7a3-ebc963552246", "Scott Chou"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 7 — Gore Vassal
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GORE_VASSAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c2889bba-58a8-46e1-959c-0fd38c1732f9"),
    "Gore Vassal",
    crate::card::CardArt::new("c2889bba-58a8-46e1-959c-0fd38c1732f9", "Matt Cavotta"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 8 — Hero of Bladehold
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HERO_OF_BLADEHOLD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8829efa0-498a-43ca-91aa-f9caeeafe298"),
    "Hero of Bladehold",
    crate::card::CardArt::new("8a3853ec-e307-46e0-96d7-0706b5c45c5e", "Austin Hsu"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 9 — Kemba's Legion
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KEMBA_S_LEGION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("30731756-81a8-480b-938f-48c1d0cb95d7"),
    "Kemba's Legion",
    crate::card::CardArt::new("30731756-81a8-480b-938f-48c1d0cb95d7", "Anthony Francisco"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 10 — Leonin Relic-Warder
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LEONIN_RELIC_WARDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dd0900e1-df78-466d-b747-33f22c273d67"),
    "Leonin Relic-Warder",
    crate::card::CardArt::new("dd0900e1-df78-466d-b747-33f22c273d67", "Greg Staples"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 11 — Leonin Skyhunter
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LEONIN_SKYHUNTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("275a47e1-816c-44f9-bd05-b8b56410436f"),
    "Leonin Skyhunter",
    crate::card::CardArt::new(
        "f7eb723d-aa4c-4a38-98de-1faefffab56b",
        "Jana Schirmer & Johannes Voss",
    ),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 12 — Loxodon Partisan
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LOXODON_PARTISAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a4a76016-96a1-40f5-9002-4b3bed65cd5c"),
    "Loxodon Partisan",
    crate::card::CardArt::new("a4a76016-96a1-40f5-9002-4b3bed65cd5c", "Matt Stewart"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 13 — Master's Call
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MASTER_S_CALL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("04f54188-2cfc-4964-add7-b452f32d57ef"),
    "Master's Call",
    crate::card::CardArt::new("04f54188-2cfc-4964-add7-b452f32d57ef", "David Rapoza"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 14 — Mirran Crusader
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MIRRAN_CRUSADER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d9a8a187-c479-429c-b0ef-c98153ffa5e5"),
    "Mirran Crusader",
    crate::card::CardArt::new("aaf7a821-3587-4aad-8411-fca5c96ab5c4", "Eric Deschamps"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 15 — Phyrexian Rebirth
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_REBIRTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("36b7536d-6b0b-4906-ba88-7fcfe9b854ee"),
    "Phyrexian Rebirth",
    crate::card::CardArt::new("36b7536d-6b0b-4906-ba88-7fcfe9b854ee", "Scott Chou"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 16 — Priests of Norn
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PRIESTS_OF_NORN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a978c49d-483a-42fe-971c-858288d07e40"),
    "Priests of Norn",
    crate::card::CardArt::new("a978c49d-483a-42fe-971c-858288d07e40", "Igor Kieryluk"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 17 — Tine Shrike
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TINE_SHRIKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("845b665e-8236-4ab9-bee4-414f075461d2"),
    "Tine Shrike",
    crate::card::CardArt::new("845b665e-8236-4ab9-bee4-414f075461d2", "Adrian Smith"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 18 — Victory's Herald
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VICTORY_S_HERALD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ac76f857-faf6-482d-a82e-7ff681f8007b"),
    "Victory's Herald",
    crate::card::CardArt::new("ac76f857-faf6-482d-a82e-7ff681f8007b", "rk post"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 19 — White Sun's Zenith
pub(in crate::card::sets) static WHITE_SUNS_ZENITH: CardRecord = CardRecord::new_with_legacy_id(
    1707,
    "White Sun's Zenith",
    CardArt::new("a879940e-6632-47c5-a30e-d29a82d16e9d", "Mike Bierek"),
    CardSet::MirrodinBesieged,
    CardRules::new_instant(mana_cost!("{X}{W}{W}")).with_ability(
        AbilityDef::spell(
            "Create X 2/2 white Cat creature tokens. Shuffle White Sun's Zenith into its owner's library.",
            EffectDef::create_creature_token(&["Cat"], &[ManaColor::White], 2, 2).with_art(CardArt::new("5252ab51-43e8-4b24-9830-de0ad9b9d3dc", "Scott Chou")).with_count(ValueDef::ChosenX),
        )
        .with_resolution_destination(SpellResolutionDestinationDef::LibraryShuffled),
    ),
);

/// Revealed and shuffled back rather than exiled, so the deck keeps it and
/// nothing gets to answer it permanently. The reveal is what makes the
/// shuffle honest: everyone knows the card went back in.
static COLOSSUS_RETURNS: [ReplacementEffectDef; 2] = [
    ReplacementEffectDef::MoveToZone(ZoneKind::Library),
    ReplacementEffectDef::Perform(&EffectDef::ShuffleLibrary {
        player: EffectRecipientDef::Controller,
    }),
];

/// Watched from everywhere the card can be, because "from anywhere" is the
/// point: countered on the stack, discarded from hand, and milled from the
/// library all come back the same way.
static COLOSSUS_ZONES: [ZoneKind; 5] = [
    ZoneKind::Battlefield,
    ZoneKind::Stack,
    ZoneKind::Hand,
    ZoneKind::Library,
    ZoneKind::Graveyard,
];

static COLOSSUS_ABILITIES: [AbilityDef; 4] = [
    abilities::trample(),
    abilities::infect(),
    abilities::indestructible(),
    AbilityDef::replacement_for(
        "If this creature would be put into a graveyard from anywhere, reveal it and shuffle it into its owner's library instead.",
        ReplacementEventDef::WouldMove {
            from: None,
            to: ZoneKind::Graveyard,
            cause: ZoneMoveCauseDef::Any,
        },
        ReplacementEffectDef::Sequence(&COLOSSUS_RETURNS),
    )
    .with_source_zones(&COLOSSUS_ZONES),
];

// MBS 20 — Blue Sun's Zenith
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BLUE_SUN_S_ZENITH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a8150f78-e187-4949-9746-fec64d1675d1"),
    "Blue Sun's Zenith",
    crate::card::CardArt::new("a8150f78-e187-4949-9746-fec64d1675d1", "Izzy"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 21 — Consecrated Sphinx
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CONSECRATED_SPHINX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b7f6b20c-9871-433c-8557-44493447e914"),
    "Consecrated Sphinx",
    crate::card::CardArt::new("b7f6b20c-9871-433c-8557-44493447e914", "Mark Zug"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 22 — Corrupted Conscience
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CORRUPTED_CONSCIENCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7276c584-76ed-4c20-a5ea-627c8f7751e6"),
    "Corrupted Conscience",
    crate::card::CardArt::new("7276c584-76ed-4c20-a5ea-627c8f7751e6", "Jason Chan"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 23 — Cryptoplasm
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CRYPTOPLASM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("15a31710-c1d6-45e4-9dbe-a75453a74da0"),
    "Cryptoplasm",
    crate::card::CardArt::new("15a31710-c1d6-45e4-9dbe-a75453a74da0", "Eric Deschamps"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 24 — Distant Memories
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DISTANT_MEMORIES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("158da0aa-8317-498b-89ed-2f84317fe256"),
    "Distant Memories",
    crate::card::CardArt::new("158da0aa-8317-498b-89ed-2f84317fe256", "Karl Kopinski"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 25 — Fuel for the Cause
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FUEL_FOR_THE_CAUSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4126e0e5-9b23-496f-8a09-7a35499f9a09"),
    "Fuel for the Cause",
    crate::card::CardArt::new("4126e0e5-9b23-496f-8a09-7a35499f9a09", "Steven Belledin"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 26 — Mirran Spy
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MIRRAN_SPY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("50e576ad-3e03-424c-8857-88ec8898a92e"),
    "Mirran Spy",
    crate::card::CardArt::new("50e576ad-3e03-424c-8857-88ec8898a92e", "Dave Kendall"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 27 — Mitotic Manipulation
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MITOTIC_MANIPULATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2abd4521-62c8-4b2f-a406-a8ddfa8f475a"),
    "Mitotic Manipulation",
    crate::card::CardArt::new("2abd4521-62c8-4b2f-a406-a8ddfa8f475a", "Dan Murayama Scott"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 28 — Neurok Commando
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NEUROK_COMMANDO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aa7084f3-9335-401f-9a62-6f131351338d"),
    "Neurok Commando",
    crate::card::CardArt::new("aa7084f3-9335-401f-9a62-6f131351338d", "Matt Stewart"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 29 — Oculus
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static OCULUS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("673bebb4-9c82-40ca-8552-b9030e961005"),
    "Oculus",
    crate::card::CardArt::new("673bebb4-9c82-40ca-8552-b9030e961005", "Dan Murayama Scott"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 30 — Quicksilver Geyser
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static QUICKSILVER_GEYSER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fb209cf5-ac7b-4521-b488-ef72451e3a25"),
    "Quicksilver Geyser",
    crate::card::CardArt::new("fb209cf5-ac7b-4521-b488-ef72451e3a25", "Erica Yang"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 31 — Serum Raker
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SERUM_RAKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f157d08a-7cd7-4120-a8bb-1b50fa0ba99b"),
    "Serum Raker",
    crate::card::CardArt::new("f157d08a-7cd7-4120-a8bb-1b50fa0ba99b", "Austin Hsu"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 32 — Spire Serpent
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SPIRE_SERPENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9a14e2a4-484b-46e4-a5a1-c66cb13be178"),
    "Spire Serpent",
    crate::card::CardArt::new("9a14e2a4-484b-46e4-a5a1-c66cb13be178", "Johann Bodin"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 33 — Steel Sabotage
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STEEL_SABOTAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bb40de7c-1905-4615-844b-4abc231fb01e"),
    "Steel Sabotage",
    crate::card::CardArt::new("bb40de7c-1905-4615-844b-4abc231fb01e", "Daarken"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 34 — Treasure Mage
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TREASURE_MAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6fe4fea1-bb23-46e4-b7b0-e83f8b99ce5d"),
    "Treasure Mage",
    crate::card::CardArt::new("6fe4fea1-bb23-46e4-b7b0-e83f8b99ce5d", "Ryan Pancoast"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 35 — Turn the Tide
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TURN_THE_TIDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bdc91fc7-7927-4c5d-888a-f40cbf658866"),
    "Turn the Tide",
    crate::card::CardArt::new("bdc91fc7-7927-4c5d-888a-f40cbf658866", "Jason Felix"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 36 — Vedalken Anatomist
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VEDALKEN_ANATOMIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0c13bb9b-c4e9-4b82-852a-dbd5602b1aa9"),
    "Vedalken Anatomist",
    crate::card::CardArt::new("0c13bb9b-c4e9-4b82-852a-dbd5602b1aa9", "Greg Staples"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 37 — Vedalken Infuser
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VEDALKEN_INFUSER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1e4f4db7-913c-4dd2-931e-630d90eb98ab"),
    "Vedalken Infuser",
    crate::card::CardArt::new("1e4f4db7-913c-4dd2-931e-630d90eb98ab", "Ryan Pancoast"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 38 — Vivisection
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VIVISECTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("684a3631-f2d6-4a64-a04a-893f452e3a60"),
    "Vivisection",
    crate::card::CardArt::new("684a3631-f2d6-4a64-a04a-893f452e3a60", "Anthony Francisco"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 39 — Black Sun's Zenith
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BLACK_SUN_S_ZENITH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("03bdcf52-50b8-42c0-9665-931d83f5f314"),
    "Black Sun's Zenith",
    crate::card::CardArt::new("03bdcf52-50b8-42c0-9665-931d83f5f314", "Daniel Ljunggren"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 40 — Caustic Hound
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CAUSTIC_HOUND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2a54115f-150a-4ae2-a5c7-20e2ba884dd1"),
    "Caustic Hound",
    crate::card::CardArt::new("2a54115f-150a-4ae2-a5c7-20e2ba884dd1", "Dave Allsop"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 41 — Flensermite
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FLENSERMITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0017de60-ee1c-4675-b04e-cdfa2c2a596e"),
    "Flensermite",
    crate::card::CardArt::new("0017de60-ee1c-4675-b04e-cdfa2c2a596e", "Dave Allsop"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 42 — Flesh-Eater Imp
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FLESH_EATER_IMP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6d5ee9f3-80fe-43b0-be5b-0c93103e1077"),
    "Flesh-Eater Imp",
    crate::card::CardArt::new("6d5ee9f3-80fe-43b0-be5b-0c93103e1077", "Johann Bodin"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 43 — Go for the Throat
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GO_FOR_THE_THROAT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a3109aaa-b1e9-4c68-85f0-7515c8eeadc3"),
    "Go for the Throat",
    crate::card::CardArt::new("1c665cfc-7e9a-444b-96b5-e8e4ef57a98a", "David Rapoza"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 44 — Gruesome Encore
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GRUESOME_ENCORE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3f7f31fb-af96-4c8c-80fa-219ebd7c3d4d"),
    "Gruesome Encore",
    crate::card::CardArt::new("3f7f31fb-af96-4c8c-80fa-219ebd7c3d4d", "Adrian Smith"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 45 — Horrifying Revelation
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HORRIFYING_REVELATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("27ff5fbd-7ce8-4188-8228-0eab0d69a7a1"),
    "Horrifying Revelation",
    crate::card::CardArt::new("27ff5fbd-7ce8-4188-8228-0eab0d69a7a1", "Shelly Wan"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 46 — Massacre Wurm
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MASSACRE_WURM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cdd32ec2-02a8-41fc-bf45-c9585bb2b3ee"),
    "Massacre Wurm",
    crate::card::CardArt::new("cdd32ec2-02a8-41fc-bf45-c9585bb2b3ee", "Jason Chan"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 47 — Morbid Plunder
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MORBID_PLUNDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bd6a8817-05fc-400e-9464-b7d925c5c312"),
    "Morbid Plunder",
    crate::card::CardArt::new("bd6a8817-05fc-400e-9464-b7d925c5c312", "Mike Bierek"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 48 — Nested Ghoul
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NESTED_GHOUL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c035ff58-9df3-4db4-b9d0-97d58080ecfe"),
    "Nested Ghoul",
    crate::card::CardArt::new("c035ff58-9df3-4db4-b9d0-97d58080ecfe", "Dave Kendall"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 49 — Phyresis
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PHYRESIS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0059d21b-0725-4806-8691-2451db36787f"),
    "Phyresis",
    crate::card::CardArt::new("0059d21b-0725-4806-8691-2451db36787f", "Izzy"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 50 — Phyrexian Crusader
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_CRUSADER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("32aaa8b9-987b-4809-8a54-aa29bdc18805"),
    "Phyrexian Crusader",
    crate::card::CardArt::new("32aaa8b9-987b-4809-8a54-aa29bdc18805", "Eric Deschamps"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 51 — Phyrexian Rager
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_RAGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c0a29ba4-fe8b-442f-a8b5-8cce7c765011"),
    "Phyrexian Rager",
    crate::card::CardArt::new("f11889da-d5dd-4bb3-b3d0-0d90698f4f34", "Stephan Martiniere"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 52 — Phyrexian Vatmother
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_VATMOTHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f0024556-0317-4c28-83c3-0a6020d72a2b"),
    "Phyrexian Vatmother",
    crate::card::CardArt::new("f0024556-0317-4c28-83c3-0a6020d72a2b", "Stephan Martiniere"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 53 — Sangromancer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SANGROMANCER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9c374193-4ebb-4f33-a24d-e567aea57b01"),
    "Sangromancer",
    crate::card::CardArt::new("9c374193-4ebb-4f33-a24d-e567aea57b01", "Igor Kieryluk"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 54 — Scourge Servant
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SCOURGE_SERVANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("45ded51b-2ced-46d6-a1d4-0f49d4b1ed2d"),
    "Scourge Servant",
    crate::card::CardArt::new("45ded51b-2ced-46d6-a1d4-0f49d4b1ed2d", "Daarken"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 55 — Septic Rats
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SEPTIC_RATS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8e7915d9-b941-4675-9a1b-18e579977144"),
    "Septic Rats",
    crate::card::CardArt::new("8e7915d9-b941-4675-9a1b-18e579977144", "Cos Koniotis"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 56 — Spread the Sickness
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SPREAD_THE_SICKNESS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("de42a771-4f5c-4295-b070-8cb857a0279e"),
    "Spread the Sickness",
    crate::card::CardArt::new("de42a771-4f5c-4295-b070-8cb857a0279e", "Jaime Jones"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 57 — Virulent Wound
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VIRULENT_WOUND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7ae09e1a-c257-45e9-88c8-7b1a7a6d714b"),
    "Virulent Wound",
    crate::card::CardArt::new("7ae09e1a-c257-45e9-88c8-7b1a7a6d714b", "Whit Brachna"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 58 — Blisterstick Shaman
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BLISTERSTICK_SHAMAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f8187e90-6a60-4ed0-9b3a-3a679743b7d0"),
    "Blisterstick Shaman",
    crate::card::CardArt::new("f8187e90-6a60-4ed0-9b3a-3a679743b7d0", "Svetlin Velinov"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 59 — Burn the Impure
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BURN_THE_IMPURE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b5641730-428d-4484-866e-ec1ac669537f"),
    "Burn the Impure",
    crate::card::CardArt::new("b5641730-428d-4484-866e-ec1ac669537f", "Nic Klein"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 60 — Concussive Bolt
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CONCUSSIVE_BOLT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("41b68e85-a381-441d-aa18-491f9e202a10"),
    "Concussive Bolt",
    crate::card::CardArt::new("41b68e85-a381-441d-aa18-491f9e202a10", "Johann Bodin"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 61 — Crush
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CRUSH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a29d5e01-6f83-4749-8340-774054bd2956"),
    "Crush",
    crate::card::CardArt::new("a29d5e01-6f83-4749-8340-774054bd2956", "Matt Stewart"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 62 — Galvanoth
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GALVANOTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fc1a696b-642a-419f-bd43-09af39a9401b"),
    "Galvanoth",
    crate::card::CardArt::new("fc1a696b-642a-419f-bd43-09af39a9401b", "Kev Walker"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 63 — Gnathosaur
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GNATHOSAUR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("27dcb0c8-e6d5-4f6b-a74f-e495b5e42606"),
    "Gnathosaur",
    crate::card::CardArt::new("27dcb0c8-e6d5-4f6b-a74f-e495b5e42606", "Jason Chan"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 64 — Goblin Wardriver
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_WARDRIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2e220c87-1223-4998-b0e5-23e2d930fa6b"),
    "Goblin Wardriver",
    crate::card::CardArt::new("2e220c87-1223-4998-b0e5-23e2d930fa6b", "Chippy"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 65 — Hellkite Igniter
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HELLKITE_IGNITER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("83715873-9330-4d29-b106-cf1e6a66d1e9"),
    "Hellkite Igniter",
    crate::card::CardArt::new("83715873-9330-4d29-b106-cf1e6a66d1e9", "Jason Chan"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 66 — Hero of Oxid Ridge
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HERO_OF_OXID_RIDGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1a516bce-3d2d-4e0f-afc7-27be3d88848c"),
    "Hero of Oxid Ridge",
    crate::card::CardArt::new("1a516bce-3d2d-4e0f-afc7-27be3d88848c", "Eric Deschamps"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 67 — Into the Core
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static INTO_THE_CORE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9cb91ecb-1962-4cd1-80c1-c9e2485822ae"),
    "Into the Core",
    crate::card::CardArt::new("9cb91ecb-1962-4cd1-80c1-c9e2485822ae", "Whit Brachna"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 68 — Koth's Courier
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KOTH_S_COURIER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("978cc53c-c038-4442-bd46-e0b9e8cdd924"),
    "Koth's Courier",
    crate::card::CardArt::new("978cc53c-c038-4442-bd46-e0b9e8cdd924", "Wayne Reynolds"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 69 — Kuldotha Flamefiend
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KULDOTHA_FLAMEFIEND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("189fea03-24db-4574-bbc2-4d3bc9e629a5"),
    "Kuldotha Flamefiend",
    crate::card::CardArt::new("189fea03-24db-4574-bbc2-4d3bc9e629a5", "Raymond Swanland"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 70 — Kuldotha Ringleader
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KULDOTHA_RINGLEADER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3cda5434-c0a5-4551-8e30-b1923f0001b8"),
    "Kuldotha Ringleader",
    crate::card::CardArt::new("3cda5434-c0a5-4551-8e30-b1923f0001b8", "Greg Staples"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 71 — Metallic Mastery
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static METALLIC_MASTERY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b939ded0-7033-4fee-864c-9e235d8720bb"),
    "Metallic Mastery",
    crate::card::CardArt::new("b939ded0-7033-4fee-864c-9e235d8720bb", "Erica Yang"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 72 — Ogre Resister
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static OGRE_RESISTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("60b7407d-f677-403b-893c-361df456009a"),
    "Ogre Resister",
    crate::card::CardArt::new("60b7407d-f677-403b-893c-361df456009a", "Efrem Palacios"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 73 — Rally the Forces
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RALLY_THE_FORCES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8f251c71-0e83-424d-9e0e-85790289087c"),
    "Rally the Forces",
    crate::card::CardArt::new("8f251c71-0e83-424d-9e0e-85790289087c", "Steven Belledin"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 74 — Red Sun's Zenith
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RED_SUN_S_ZENITH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("373eb109-0e30-41c1-b2df-6bc78d968890"),
    "Red Sun's Zenith",
    crate::card::CardArt::new("373eb109-0e30-41c1-b2df-6bc78d968890", "Svetlin Velinov"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 75 — Slagstorm
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SLAGSTORM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9e318b03-2aad-462b-a2a9-8b6bdf0e93d6"),
    "Slagstorm",
    crate::card::CardArt::new("9e318b03-2aad-462b-a2a9-8b6bdf0e93d6", "Dan Murayama Scott"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 76 — Spiraling Duelist
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SPIRALING_DUELIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("81ac77ab-e51a-4c5c-ab6b-3fb610a5fa27"),
    "Spiraling Duelist",
    crate::card::CardArt::new("81ac77ab-e51a-4c5c-ab6b-3fb610a5fa27", "Karl Kopinski"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 77 — Blightwidow
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BLIGHTWIDOW: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("26da4278-ba44-4555-8837-e24627b46533"),
    "Blightwidow",
    crate::card::CardArt::new("26da4278-ba44-4555-8837-e24627b46533", "Daniel Ljunggren"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 78 — Creeping Corrosion
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CREEPING_CORROSION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("05d5a7b3-18b6-4b1d-85cc-2253e605390c"),
    "Creeping Corrosion",
    crate::card::CardArt::new("05d5a7b3-18b6-4b1d-85cc-2253e605390c", "Ryan Pancoast"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 79 — Fangren Marauder
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FANGREN_MARAUDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f5cf62a2-d03a-495d-924a-bf79524175fa"),
    "Fangren Marauder",
    crate::card::CardArt::new("f5cf62a2-d03a-495d-924a-bf79524175fa", "James Ryman"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 80 — Glissa's Courier
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GLISSA_S_COURIER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("45da44df-a83a-4974-bc22-5243dcda7cbd"),
    "Glissa's Courier",
    crate::card::CardArt::new("45da44df-a83a-4974-bc22-5243dcda7cbd", "Dave Kendall"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 81 — Green Sun's Zenith
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GREEN_SUN_S_ZENITH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("02335747-54e3-4827-ae19-4e362863da9b"),
    "Green Sun's Zenith",
    crate::card::CardArt::new("02335747-54e3-4827-ae19-4e362863da9b", "David Rapoza"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 82 — Lead the Stampede
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LEAD_THE_STAMPEDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("66ed14c8-38c6-4da5-a6ee-f814478161d2"),
    "Lead the Stampede",
    crate::card::CardArt::new("66ed14c8-38c6-4da5-a6ee-f814478161d2", "Efrem Palacios"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 83 — Melira's Keepers
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MELIRA_S_KEEPERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d9a935b4-347c-46d9-a7c5-8c5079948959"),
    "Melira's Keepers",
    crate::card::CardArt::new("d9a935b4-347c-46d9-a7c5-8c5079948959", "Eric Deschamps"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 84 — Mirran Mettle
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MIRRAN_METTLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("edbb5778-fa9a-4a1f-be4f-627630c3e3ca"),
    "Mirran Mettle",
    crate::card::CardArt::new("edbb5778-fa9a-4a1f-be4f-627630c3e3ca", "Karl Kopinski"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 85 — Phyrexian Hydra
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_HYDRA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cb135aa1-9f46-4d60-a1a4-97aa0e852ced"),
    "Phyrexian Hydra",
    crate::card::CardArt::new("cb135aa1-9f46-4d60-a1a4-97aa0e852ced", "Mike Bierek"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 86 — Pistus Strike
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PISTUS_STRIKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1a2918d6-50f7-4bc1-aef2-930a5c84be8d"),
    "Pistus Strike",
    crate::card::CardArt::new("1a2918d6-50f7-4bc1-aef2-930a5c84be8d", "Jaime Jones"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 87 — Plaguemaw Beast
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PLAGUEMAW_BEAST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("52341830-8cea-421f-b901-9229004f2d45"),
    "Plaguemaw Beast",
    crate::card::CardArt::new("52341830-8cea-421f-b901-9229004f2d45", "Whit Brachna"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 88 — Praetor's Counsel
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PRAETOR_S_COUNSEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b67c8bea-d4c9-4759-8a37-10546b234472"),
    "Praetor's Counsel",
    crate::card::CardArt::new("b67c8bea-d4c9-4759-8a37-10546b234472", "Daarken"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 89 — Quilled Slagwurm
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static QUILLED_SLAGWURM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("12c597b9-5024-42bd-b500-5ef6a3accda6"),
    "Quilled Slagwurm",
    crate::card::CardArt::new("12c597b9-5024-42bd-b500-5ef6a3accda6", "Matt Stewart"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 90 — Rot Wolf
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ROT_WOLF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7a81dfcf-a7c4-41a4-b1e9-b9e9c3f75742"),
    "Rot Wolf",
    crate::card::CardArt::new("7a81dfcf-a7c4-41a4-b1e9-b9e9c3f75742", "Nils Hamm"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 91 — Tangle Mantis
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TANGLE_MANTIS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c0e4d333-78f7-4710-9b26-36e285c0d9f8"),
    "Tangle Mantis",
    crate::card::CardArt::new("c0e4d333-78f7-4710-9b26-36e285c0d9f8", "Chris Rahn"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 92 — Thrun, the Last Troll
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static THRUN_THE_LAST_TROLL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5d393da0-4cb6-4ae8-b747-8e6d0fa7f55a"),
    "Thrun, the Last Troll",
    crate::card::CardArt::new("5d393da0-4cb6-4ae8-b747-8e6d0fa7f55a", "Jason Chan"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 93 — Unnatural Predation
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static UNNATURAL_PREDATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2798fdff-0b39-41b6-b0ec-c4f449ca3314"),
    "Unnatural Predation",
    crate::card::CardArt::new("2798fdff-0b39-41b6-b0ec-c4f449ca3314", "Shelly Wan"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 94 — Viridian Corrupter
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VIRIDIAN_CORRUPTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0cc13aee-5a74-4dab-a6af-7dc31255981d"),
    "Viridian Corrupter",
    crate::card::CardArt::new("0cc13aee-5a74-4dab-a6af-7dc31255981d", "Matt Cavotta"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 95 — Viridian Emissary
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VIRIDIAN_EMISSARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("129fa334-f561-4fbd-9f51-2fa044b674e1"),
    "Viridian Emissary",
    crate::card::CardArt::new("129fa334-f561-4fbd-9f51-2fa044b674e1", "Matt Stewart"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 96 — Glissa, the Traitor
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GLISSA_THE_TRAITOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9e293b2d-5eae-43c3-bb1f-4c0ed2b0bd21"),
    "Glissa, the Traitor",
    crate::card::CardArt::new("755e0fbf-4f00-4b05-a535-27e78e96d6b6", "Chris Rahn"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 97 — Tezzeret, Agent of Bolas
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TEZZERET_AGENT_OF_BOLAS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0f3c8470-1cc8-4383-8782-c022867d46e8"),
    "Tezzeret, Agent of Bolas",
    crate::card::CardArt::new("0f3c8470-1cc8-4383-8782-c022867d46e8", "Aleksi Briclot"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 98 — Bladed Sentinel
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BLADED_SENTINEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("69959c54-1350-4c64-8e5a-fc8447bb979c"),
    "Bladed Sentinel",
    crate::card::CardArt::new("69959c54-1350-4c64-8e5a-fc8447bb979c", "Tomasz Jedruszek"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 99 — Blightsteel Colossus
pub(in crate::card::sets) static BLIGHTSTEEL_COLOSSUS: CardRecord = CardRecord::new_with_legacy_id(
    2183,
    "Blightsteel Colossus",
    CardArt::new("7928bb14-7631-4830-a756-26d1ea832ba2", "Chris Rahn"),
    CardSet::MirrodinBesieged,
    // Eleven infect damage is one hit from a win, and the deck that plays it
    // is not paying twelve mana honestly -- it is cheating it into play and
    // attacking once.
    CardRules::new_artifact_creature(mana_cost!("{12}"), &["Phyrexian", "Golem"], 11, 11)
        .with_abilities(&COLOSSUS_ABILITIES),
);

// MBS 100 — Bonehoard
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BONEHOARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e14613bc-0083-48d6-ac10-b2839657e84b"),
    "Bonehoard",
    crate::card::CardArt::new("e14613bc-0083-48d6-ac10-b2839657e84b", "Chippy"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 101 — Brass Squire
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BRASS_SQUIRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("37928b90-ab31-4c73-99b2-fe31feb2afea"),
    "Brass Squire",
    crate::card::CardArt::new("37928b90-ab31-4c73-99b2-fe31feb2afea", "Ryan Pancoast"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 102 — Copper Carapace
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static COPPER_CARAPACE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("60ccb013-4641-400f-a035-86030ac55582"),
    "Copper Carapace",
    crate::card::CardArt::new("60ccb013-4641-400f-a035-86030ac55582", "Franz Vohwinkel"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 103 — Core Prowler
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CORE_PROWLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("05414ba7-0f59-4c73-931c-e599d149d3ba"),
    "Core Prowler",
    crate::card::CardArt::new("05414ba7-0f59-4c73-931c-e599d149d3ba", "Dave Allsop"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 104 — Darksteel Plate
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DARKSTEEL_PLATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ba60731f-ed96-4eba-b2de-94965745f35a"),
    "Darksteel Plate",
    crate::card::CardArt::new("ba60731f-ed96-4eba-b2de-94965745f35a", "Daniel Ljunggren"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 105 — Decimator Web
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DECIMATOR_WEB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fa1fe29f-cf84-45d3-b8fe-e4de1d2bebbf"),
    "Decimator Web",
    crate::card::CardArt::new("fa1fe29f-cf84-45d3-b8fe-e4de1d2bebbf", "Daniel Ljunggren"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 106 — Dross Ripper
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DROSS_RIPPER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("55d54f08-53f0-41b2-8b86-8244515224eb"),
    "Dross Ripper",
    crate::card::CardArt::new("55d54f08-53f0-41b2-8b86-8244515224eb", "David Rapoza"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 107 — Flayer Husk
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FLAYER_HUSK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cbd47a02-5a6e-4daa-9877-f65c8639c569"),
    "Flayer Husk",
    crate::card::CardArt::new("cbd47a02-5a6e-4daa-9877-f65c8639c569", "Igor Kieryluk"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 108 — Gust-Skimmer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GUST_SKIMMER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5970d053-e2e8-471b-b342-2e9b9177724c"),
    "Gust-Skimmer",
    crate::card::CardArt::new("5970d053-e2e8-471b-b342-2e9b9177724c", "Dan Murayama Scott"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 109 — Hexplate Golem
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HEXPLATE_GOLEM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("49b913f3-6581-45ae-9cdb-274c2ccd8899"),
    "Hexplate Golem",
    crate::card::CardArt::new("49b913f3-6581-45ae-9cdb-274c2ccd8899", "Matt Cavotta"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 110 — Ichor Wellspring
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ICHOR_WELLSPRING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2d1ea522-a0f6-45a8-8985-6fcca95d60cc"),
    "Ichor Wellspring",
    crate::card::CardArt::new("2d1ea522-a0f6-45a8-8985-6fcca95d60cc", "Steven Belledin"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 111 — Knowledge Pool
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KNOWLEDGE_POOL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("393454c2-b256-4a6e-9bc2-56a47cab5073"),
    "Knowledge Pool",
    crate::card::CardArt::new("393454c2-b256-4a6e-9bc2-56a47cab5073", "Mike Bierek"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 112 — Lumengrid Gargoyle
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LUMENGRID_GARGOYLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("45350c4b-def2-4b93-ab66-9f32bd426cff"),
    "Lumengrid Gargoyle",
    crate::card::CardArt::new("45350c4b-def2-4b93-ab66-9f32bd426cff", "Randis Albion"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 113 — Magnetic Mine
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MAGNETIC_MINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5ed2f7dc-3ada-4490-8c1f-1d03bd4840f5"),
    "Magnetic Mine",
    crate::card::CardArt::new("5ed2f7dc-3ada-4490-8c1f-1d03bd4840f5", "David Rapoza"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 114 — Mirrorworks
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MIRRORWORKS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cade6dde-1edf-44b8-a37e-a22f9207db51"),
    "Mirrorworks",
    crate::card::CardArt::new("cade6dde-1edf-44b8-a37e-a22f9207db51", "John Avon"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 115 — Mortarpod
pub(in crate::card::sets) static MORTARPOD: CardRecord = CardRecord::new_with_legacy_id(
    1704,
    "Mortarpod",
    CardArt::new("fbd23da5-421f-41d0-bb60-59560da7dece", "Eric Deschamps"),
    CardSet::MirrodinBesieged,
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::living_weapon(
                TokenCharacteristics::creature(
                    &["Phyrexian", "Germ"],
                    &[ManaColor::Black],
                    0,
                    0,
                )
                .with_art(CardArt::new(
                    "65c65445-1016-4fd3-963e-1c9eb252d4a6",
                    "Igor Kieryluk",
                )),
            ),
            AbilityDef::static_ability(
                "Equipped creature gets +0/+1 and has \"Sacrifice this creature: This creature deals 1 damage to any target.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(0),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::add_ability(&AbilityDef::activated_with_targets(
                            "Sacrifice this creature: This creature deals 1 damage to any target.",
                            &[AbilityCostDef::SacrificeSource],
                            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget)],
                            EffectDef::DealDamage {
                                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                amount: ValueDef::Constant(1),
                            },
                        )),
                    ]),
                },
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{2}"))], "Equip {2}"),
        ]),
);

// MBS 116 — Myr Sire
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MYR_SIRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("507979fd-5459-4933-8707-adc303750ce9"),
    "Myr Sire",
    crate::card::CardArt::new("507979fd-5459-4933-8707-adc303750ce9", "Jaime Jones"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 117 — Myr Turbine
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MYR_TURBINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("71a76840-47f7-4e1e-b68b-00cb7da98cdf"),
    "Myr Turbine",
    crate::card::CardArt::new("71a76840-47f7-4e1e-b68b-00cb7da98cdf", "Randis Albion"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 118 — Myr Welder
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MYR_WELDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("eff75f16-413c-4618-b766-67bd8ff4d161"),
    "Myr Welder",
    crate::card::CardArt::new("eff75f16-413c-4618-b766-67bd8ff4d161", "Austin Hsu"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 119 — Peace Strider
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PEACE_STRIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("55710eb0-ae16-420a-9f99-6a245e0f4c14"),
    "Peace Strider",
    crate::card::CardArt::new("55710eb0-ae16-420a-9f99-6a245e0f4c14", "Igor Kieryluk"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 120 — Phyrexian Digester
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_DIGESTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f9112062-1a1b-462b-85d3-821ea91778b8"),
    "Phyrexian Digester",
    crate::card::CardArt::new("f9112062-1a1b-462b-85d3-821ea91778b8", "Dave Allsop"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 121 — Phyrexian Juggernaut
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_JUGGERNAUT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a9f6ed6c-8095-4a81-b428-36b2916eec88"),
    "Phyrexian Juggernaut",
    crate::card::CardArt::new("a9f6ed6c-8095-4a81-b428-36b2916eec88", "Kev Walker"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 122 — Phyrexian Revoker
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_REVOKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7c7bec21-61b0-4e72-848b-82f38e1910e0"),
    "Phyrexian Revoker",
    crate::card::CardArt::new("7c7bec21-61b0-4e72-848b-82f38e1910e0", "Kev Walker"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 123 — Pierce Strider
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PIERCE_STRIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("88b449a5-634f-47b1-a757-86a6849f6777"),
    "Pierce Strider",
    crate::card::CardArt::new("88b449a5-634f-47b1-a757-86a6849f6777", "Igor Kieryluk"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 124 — Piston Sledge
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PISTON_SLEDGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cf6a88da-0b51-42d1-aa63-8c4b5e5c03c3"),
    "Piston Sledge",
    crate::card::CardArt::new("cf6a88da-0b51-42d1-aa63-8c4b5e5c03c3", "Pete Venters"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 125 — Plague Myr
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PLAGUE_MYR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8f932690-9a38-4a3f-8805-a192208152a3"),
    "Plague Myr",
    crate::card::CardArt::new("8f932690-9a38-4a3f-8805-a192208152a3", "Efrem Palacios"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 126 — Psychosis Crawler
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PSYCHOSIS_CRAWLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4dd84701-857e-4948-8cb8-39b8a321a177"),
    "Psychosis Crawler",
    crate::card::CardArt::new("4dd84701-857e-4948-8cb8-39b8a321a177", "Stephan Martiniere"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 127 — Razorfield Rhino
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RAZORFIELD_RHINO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2019a1b4-7f88-4c8a-9ef4-bdfbd2f9e9cc"),
    "Razorfield Rhino",
    crate::card::CardArt::new("2019a1b4-7f88-4c8a-9ef4-bdfbd2f9e9cc", "Kekai Kotaki"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 128 — Rusted Slasher
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RUSTED_SLASHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f2228ee5-507a-46ba-82ae-72ba3088a568"),
    "Rusted Slasher",
    crate::card::CardArt::new("f2228ee5-507a-46ba-82ae-72ba3088a568", "Adrian Smith"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 129 — Shimmer Myr
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHIMMER_MYR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("478c1a5b-237b-4ecb-be73-3c6dafd5ae53"),
    "Shimmer Myr",
    crate::card::CardArt::new(
        "478c1a5b-237b-4ecb-be73-3c6dafd5ae53",
        "Jana Schirmer & Johannes Voss",
    ),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 130 — Shriekhorn
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHRIEKHORN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bb41269f-007d-43ba-a682-d3929cc69696"),
    "Shriekhorn",
    crate::card::CardArt::new("bb41269f-007d-43ba-a682-d3929cc69696", "Erica Yang"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 131 — Signal Pest
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SIGNAL_PEST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("be065962-f2ed-4ab9-be6b-bfc66d63ff4e"),
    "Signal Pest",
    crate::card::CardArt::new("be065962-f2ed-4ab9-be6b-bfc66d63ff4e", "Mark Zug"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 132 — Silverskin Armor
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SILVERSKIN_ARMOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d1dba839-bd1e-4ce8-ab90-005eb1f0102e"),
    "Silverskin Armor",
    crate::card::CardArt::new("d1dba839-bd1e-4ce8-ab90-005eb1f0102e", "Terese Nielsen"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 133 — Skinwing
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SKINWING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d11bc5d8-378a-441f-b92a-a60005745f25"),
    "Skinwing",
    crate::card::CardArt::new("d11bc5d8-378a-441f-b92a-a60005745f25", "Igor Kieryluk"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 134 — Sphere of the Suns
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SPHERE_OF_THE_SUNS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7a2c0735-1816-489b-9793-89d1060d78f7"),
    "Sphere of the Suns",
    crate::card::CardArt::new(
        "7a2c0735-1816-489b-9793-89d1060d78f7",
        "Jana Schirmer & Johannes Voss",
    ),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 135 — Spin Engine
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SPIN_ENGINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fc820134-ae9a-4c99-a869-cee7f1f6d79b"),
    "Spin Engine",
    crate::card::CardArt::new("fc820134-ae9a-4c99-a869-cee7f1f6d79b", "Pete Venters"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 136 — Spine of Ish Sah
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SPINE_OF_ISH_SAH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("59313b00-ac75-484a-ad74-db9d8960c0f8"),
    "Spine of Ish Sah",
    crate::card::CardArt::new("59313b00-ac75-484a-ad74-db9d8960c0f8", "Daniel Ljunggren"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 137 — Strandwalker
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STRANDWALKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d0d7ff8f-7733-4323-8575-c50b3e730dbc"),
    "Strandwalker",
    crate::card::CardArt::new("d0d7ff8f-7733-4323-8575-c50b3e730dbc", "Igor Kieryluk"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 138 — Sword of Feast and Famine
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SWORD_OF_FEAST_AND_FAMINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("580b4818-2a01-46ad-b4d9-7d895a625bb3"),
    "Sword of Feast and Famine",
    crate::card::CardArt::new("580b4818-2a01-46ad-b4d9-7d895a625bb3", "Chris Rahn"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 139 — Tangle Hulk
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TANGLE_HULK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8ed3c301-8d8e-45fe-902a-af03a79525be"),
    "Tangle Hulk",
    crate::card::CardArt::new("8ed3c301-8d8e-45fe-902a-af03a79525be", "Mark Zug"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 140 — Thopter Assembly
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static THOPTER_ASSEMBLY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f1abeca7-0a9c-49ae-9042-9899829e74a3"),
    "Thopter Assembly",
    crate::card::CardArt::new("644ab412-0603-447d-b8ef-dfd79f78e2a5", "Volkan Baǵa"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 141 — Titan Forge
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TITAN_FORGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7a3bb4ab-6fe7-4926-a929-3e37691f287a"),
    "Titan Forge",
    crate::card::CardArt::new("7a3bb4ab-6fe7-4926-a929-3e37691f287a", "Svetlin Velinov"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 142 — Training Drone
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TRAINING_DRONE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8b7e986f-5b28-46d2-8ec2-ee719b07dbfd"),
    "Training Drone",
    crate::card::CardArt::new("8b7e986f-5b28-46d2-8ec2-ee719b07dbfd", "Matt Cavotta"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 143 — Viridian Claw
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VIRIDIAN_CLAW: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2154e3c6-ab69-4661-b1ef-a50cc0f6f763"),
    "Viridian Claw",
    crate::card::CardArt::new("2154e3c6-ab69-4661-b1ef-a50cc0f6f763", "Marc Simonetti"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 144 — Contested War Zone
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CONTESTED_WAR_ZONE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bee9b696-5203-4235-9bdd-f2a389d69813"),
    "Contested War Zone",
    crate::card::CardArt::new("bee9b696-5203-4235-9bdd-f2a389d69813", "Scott Chou"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

// MBS 145 — Inkmoth Nexus
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static INKMOTH_NEXUS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ec50c1c3-885e-47d3-ada7-cc0edbf09df1"),
    "Inkmoth Nexus",
    crate::card::CardArt::new("ec50c1c3-885e-47d3-ada7-cc0edbf09df1", "Jung Park"),
    crate::card::CardSet::MirrodinBesieged,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ACCORDER_PALADIN,
    &ARDENT_RECRUIT,
    &BANISHMENT_DECREE,
    &CHOKING_FUMES,
    &FRANTIC_SALVAGE,
    &GORE_VASSAL,
    &HERO_OF_BLADEHOLD,
    &KEMBA_S_LEGION,
    &LEONIN_RELIC_WARDER,
    &LEONIN_SKYHUNTER,
    &LOXODON_PARTISAN,
    &MASTER_S_CALL,
    &MIRRAN_CRUSADER,
    &PHYREXIAN_REBIRTH,
    &PRIESTS_OF_NORN,
    &TINE_SHRIKE,
    &VICTORY_S_HERALD,
    &WHITE_SUNS_ZENITH,
    &BLUE_SUN_S_ZENITH,
    &CONSECRATED_SPHINX,
    &CORRUPTED_CONSCIENCE,
    &CRYPTOPLASM,
    &DISTANT_MEMORIES,
    &FUEL_FOR_THE_CAUSE,
    &MIRRAN_SPY,
    &MITOTIC_MANIPULATION,
    &NEUROK_COMMANDO,
    &OCULUS,
    &QUICKSILVER_GEYSER,
    &SERUM_RAKER,
    &SPIRE_SERPENT,
    &STEEL_SABOTAGE,
    &TREASURE_MAGE,
    &TURN_THE_TIDE,
    &VEDALKEN_ANATOMIST,
    &VEDALKEN_INFUSER,
    &VIVISECTION,
    &BLACK_SUN_S_ZENITH,
    &CAUSTIC_HOUND,
    &FLENSERMITE,
    &FLESH_EATER_IMP,
    &GO_FOR_THE_THROAT,
    &GRUESOME_ENCORE,
    &HORRIFYING_REVELATION,
    &MASSACRE_WURM,
    &MORBID_PLUNDER,
    &NESTED_GHOUL,
    &PHYRESIS,
    &PHYREXIAN_CRUSADER,
    &PHYREXIAN_RAGER,
    &PHYREXIAN_VATMOTHER,
    &SANGROMANCER,
    &SCOURGE_SERVANT,
    &SEPTIC_RATS,
    &SPREAD_THE_SICKNESS,
    &VIRULENT_WOUND,
    &BLISTERSTICK_SHAMAN,
    &BURN_THE_IMPURE,
    &CONCUSSIVE_BOLT,
    &CRUSH,
    &GALVANOTH,
    &GNATHOSAUR,
    &GOBLIN_WARDRIVER,
    &HELLKITE_IGNITER,
    &HERO_OF_OXID_RIDGE,
    &INTO_THE_CORE,
    &KOTH_S_COURIER,
    &KULDOTHA_FLAMEFIEND,
    &KULDOTHA_RINGLEADER,
    &METALLIC_MASTERY,
    &OGRE_RESISTER,
    &RALLY_THE_FORCES,
    &RED_SUN_S_ZENITH,
    &SLAGSTORM,
    &SPIRALING_DUELIST,
    &BLIGHTWIDOW,
    &CREEPING_CORROSION,
    &FANGREN_MARAUDER,
    &GLISSA_S_COURIER,
    &GREEN_SUN_S_ZENITH,
    &LEAD_THE_STAMPEDE,
    &MELIRA_S_KEEPERS,
    &MIRRAN_METTLE,
    &PHYREXIAN_HYDRA,
    &PISTUS_STRIKE,
    &PLAGUEMAW_BEAST,
    &PRAETOR_S_COUNSEL,
    &QUILLED_SLAGWURM,
    &ROT_WOLF,
    &TANGLE_MANTIS,
    &THRUN_THE_LAST_TROLL,
    &UNNATURAL_PREDATION,
    &VIRIDIAN_CORRUPTER,
    &VIRIDIAN_EMISSARY,
    &GLISSA_THE_TRAITOR,
    &TEZZERET_AGENT_OF_BOLAS,
    &BLADED_SENTINEL,
    &BLIGHTSTEEL_COLOSSUS,
    &BONEHOARD,
    &BRASS_SQUIRE,
    &COPPER_CARAPACE,
    &CORE_PROWLER,
    &DARKSTEEL_PLATE,
    &DECIMATOR_WEB,
    &DROSS_RIPPER,
    &FLAYER_HUSK,
    &GUST_SKIMMER,
    &HEXPLATE_GOLEM,
    &ICHOR_WELLSPRING,
    &KNOWLEDGE_POOL,
    &LUMENGRID_GARGOYLE,
    &MAGNETIC_MINE,
    &MIRRORWORKS,
    &MORTARPOD,
    &MYR_SIRE,
    &MYR_TURBINE,
    &MYR_WELDER,
    &PEACE_STRIDER,
    &PHYREXIAN_DIGESTER,
    &PHYREXIAN_JUGGERNAUT,
    &PHYREXIAN_REVOKER,
    &PIERCE_STRIDER,
    &PISTON_SLEDGE,
    &PLAGUE_MYR,
    &PSYCHOSIS_CRAWLER,
    &RAZORFIELD_RHINO,
    &RUSTED_SLASHER,
    &SHIMMER_MYR,
    &SHRIEKHORN,
    &SIGNAL_PEST,
    &SILVERSKIN_ARMOR,
    &SKINWING,
    &SPHERE_OF_THE_SUNS,
    &SPIN_ENGINE,
    &SPINE_OF_ISH_SAH,
    &STRANDWALKER,
    &SWORD_OF_FEAST_AND_FAMINE,
    &TANGLE_HULK,
    &THOPTER_ASSEMBLY,
    &TITAN_FORGE,
    &TRAINING_DRONE,
    &VIRIDIAN_CLAW,
    &CONTESTED_WAR_ZONE,
    &INKMOTH_NEXUS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&crate::card::sets::y1994::legends::DIVINE_OFFERING), // MBS 5
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::PLAINS),            // MBS 146
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::ISLAND),            // MBS 148
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::SWAMP),             // MBS 150
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::MOUNTAIN),          // MBS 152
    PrintingRecord::reprint(&crate::card::sets::y1993::alpha::FOREST),            // MBS 154
];
