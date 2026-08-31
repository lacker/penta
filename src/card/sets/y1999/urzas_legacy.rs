//! Urza's Legacy cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::CostQuantityDef;
use crate::card::sets::y1998::portal_second_age as catalog_p02;
use crate::card::sets::y2011::magic_2012 as catalog_m12;
use crate::card::sets::y2012::magic_2013 as catalog_m13;
use crate::card::sets::y2013::magic_2014 as catalog_m14;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedEffectDef, AppliedRuleDef, BattlefieldEntryChoiceDestinationDef,
    BattlefieldEntryScalarChoiceDef, CardArt, CardRules, CardSet, CardType, ChoiceVisibilityDef,
    ChooseDef, ColorChoiceOperationDef, CostModificationDef, DiscardSelectionDef, EffectDef,
    EffectRecipientDef, InstalledTriggerDef, ManaColor, ObjectChoiceBindingDef, ObjectPredicateDef,
    ObjectQueryDef, ObjectSetDef, PlayerRefDef, PlayerRelation, ReplacementChoiceDef,
    ReplacementEffectDef, ResolvedEffectDurationDef, SpellAdditionalCostDef, TriggerEventDef,
    TurnStepDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::ids::ObjectSetBindingIndex;
use crate::{TargetIndex, mana_cost};

// ULG 1 — Angelic Curator
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ANGELIC_CURATOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c63ba2da-6dea-44ac-8439-527222da565b"),
    "Angelic Curator",
    crate::card::CardArt::new("c63ba2da-6dea-44ac-8439-527222da565b", "Greg Staples"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 2 — Blessed Reversal
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BLESSED_REVERSAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("899ecc19-8106-4e5a-bb25-aaea9684ba0e"),
    "Blessed Reversal",
    crate::card::CardArt::new("3fb6d738-f6a8-4626-8103-68e63874eda4", "Pete Venters"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 3 — Burst of Energy
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BURST_OF_ENERGY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("43d590d2-cfa3-43d1-9e65-bc68b5a2a3ee"),
    "Burst of Energy",
    crate::card::CardArt::new("43d590d2-cfa3-43d1-9e65-bc68b5a2a3ee", "Mark Brill"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 4 — Cessation
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CESSATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3a113f0c-8249-427b-979b-10898ec66a3a"),
    "Cessation",
    crate::card::CardArt::new("3a113f0c-8249-427b-979b-10898ec66a3a", "Mark Zug"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 5 — Defender of Law
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DEFENDER_OF_LAW: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9c8e8719-8c33-429d-8b95-b7f813888850"),
    "Defender of Law",
    crate::card::CardArt::new("9c8e8719-8c33-429d-8b95-b7f813888850", "Carl Critchlow"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 6 — Devout Harpist
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DEVOUT_HARPIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("985b5c60-8a5e-4473-ba43-583aef50f19e"),
    "Devout Harpist",
    crate::card::CardArt::new("985b5c60-8a5e-4473-ba43-583aef50f19e", "Rebecca Guay"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 7 — Erase (reprint)

// ULG 8 — Expendable Troops
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static EXPENDABLE_TROOPS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f31d7d1b-a219-4653-be99-a885bc9b2e2f"),
    "Expendable Troops",
    crate::card::CardArt::new("f31d7d1b-a219-4653-be99-a885bc9b2e2f", "Carl Critchlow"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 9 — Hope and Glory
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HOPE_AND_GLORY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7cc6478f-4ae5-4f26-baa9-b28e992f962e"),
    "Hope and Glory",
    crate::card::CardArt::new("7cc6478f-4ae5-4f26-baa9-b28e992f962e", "Heather Hudson"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 10 — Iron Will
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static IRON_WILL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bee0ee84-6c22-4649-b621-e3fdb08bbe45"),
    "Iron Will",
    crate::card::CardArt::new("bee0ee84-6c22-4649-b621-e3fdb08bbe45", "Val Mayerik"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 11 — Karmic Guide
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KARMIC_GUIDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("77d23045-905b-44cb-9af9-cc6ad717477d"),
    "Karmic Guide",
    crate::card::CardArt::new("77d23045-905b-44cb-9af9-cc6ad717477d", "Heather Hudson"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 12 — Knighthood
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KNIGHTHOOD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1d5e98d3-2521-4340-8d48-98e8c2c7818d"),
    "Knighthood",
    crate::card::CardArt::new("1d5e98d3-2521-4340-8d48-98e8c2c7818d", "Kev Walker"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 13 — Martyr's Cause
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MARTYR_S_CAUSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4c1f026b-8c7f-4051-9922-5684a6b2c06b"),
    "Martyr's Cause",
    crate::card::CardArt::new("4c1f026b-8c7f-4051-9922-5684a6b2c06b", "Jeff Laubenstein"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 14 — Mother of Runes
pub(in crate::card::sets) static MOTHER_OF_RUNES: CardRecord = CardRecord::new_with_legacy_id(
    2119,
    "Mother of Runes",
    CardArt::new("0b1a46ab-95cb-4c24-924f-fc2afd4fcac7", "Scott M. Fischer"),
    CardSet::UrzasLegacy,
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Cleric"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Target creature you control gains protection from the color of your choice until end of turn.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::ChooseColor {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                operation: ColorChoiceOperationDef::ProtectionFromChosenColor,
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ULG 15 — Opal Avenger
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static OPAL_AVENGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f9337bbe-e092-469d-8122-77f92e233306"),
    "Opal Avenger",
    crate::card::CardArt::new(
        "f9337bbe-e092-469d-8122-77f92e233306",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 16 — Opal Champion
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static OPAL_CHAMPION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2699cf3b-df54-4c77-ba19-3bc7598ae3fa"),
    "Opal Champion",
    crate::card::CardArt::new(
        "2699cf3b-df54-4c77-ba19-3bc7598ae3fa",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 17 — Peace and Quiet
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PEACE_AND_QUIET: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5d73accc-8f19-44d4-8216-c1acdbef3856"),
    "Peace and Quiet",
    crate::card::CardArt::new("5d73accc-8f19-44d4-8216-c1acdbef3856", "Don Hazeltine"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 18 — Planar Collapse
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PLANAR_COLLAPSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ee22cf3c-51b0-4790-ab13-985cbe900c3b"),
    "Planar Collapse",
    crate::card::CardArt::new("ee22cf3c-51b0-4790-ab13-985cbe900c3b", "Mark Zug"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 19 — Purify
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PURIFY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5341da18-df05-4135-b948-7aa3e3d7a492"),
    "Purify",
    crate::card::CardArt::new("5341da18-df05-4135-b948-7aa3e3d7a492", "John Avon"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 20 — Radiant, Archangel
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RADIANT_ARCHANGEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("99509da7-3e11-4c38-804b-286ce572f36e"),
    "Radiant, Archangel",
    crate::card::CardArt::new("99509da7-3e11-4c38-804b-286ce572f36e", "Michael Sutfin"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 21 — Radiant's Dragoons
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RADIANT_S_DRAGOONS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8a0f39de-6ad2-410c-bc6c-75fd3c8d159b"),
    "Radiant's Dragoons",
    crate::card::CardArt::new("8a0f39de-6ad2-410c-bc6c-75fd3c8d159b", "Pete Venters"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 22 — Radiant's Judgment
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RADIANT_S_JUDGMENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("28d2718e-c6fc-4961-b094-11f25f1177ff"),
    "Radiant's Judgment",
    crate::card::CardArt::new("28d2718e-c6fc-4961-b094-11f25f1177ff", "Greg Staples"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 23 — Sustainer of the Realm
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SUSTAINER_OF_THE_REALM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ca280e1e-4231-48e5-be1b-965480822c46"),
    "Sustainer of the Realm",
    crate::card::CardArt::new("ca280e1e-4231-48e5-be1b-965480822c46", "Greg Staples"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 24 — Tragic Poet
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TRAGIC_POET: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("294aa7fc-12be-4722-b288-de14a28919b2"),
    "Tragic Poet",
    crate::card::CardArt::new("294aa7fc-12be-4722-b288-de14a28919b2", "Quinton Hoover"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 25 — Anthroplasm
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ANTHROPLASM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("089e2bc7-0063-47bf-8f66-48bed6eb046b"),
    "Anthroplasm",
    crate::card::CardArt::new("089e2bc7-0063-47bf-8f66-48bed6eb046b", "Ron Spencer"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 26 — Archivist
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ARCHIVIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9936cb4d-f4e3-4fc7-869e-8f17056e57d5"),
    "Archivist",
    crate::card::CardArt::new("9936cb4d-f4e3-4fc7-869e-8f17056e57d5", "Pete Venters"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 27 — Aura Flux
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static AURA_FLUX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a6be1542-70b8-4e97-a951-100966dc46ce"),
    "Aura Flux",
    crate::card::CardArt::new("a6be1542-70b8-4e97-a951-100966dc46ce", "John Avon"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 28 — Bouncing Beebles
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BOUNCING_BEEBLES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8656bdd4-0c45-43f9-b2dc-d11a355ff747"),
    "Bouncing Beebles",
    crate::card::CardArt::new("8656bdd4-0c45-43f9-b2dc-d11a355ff747", "Jeff Miracola"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 29 — Cloud of Faeries
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CLOUD_OF_FAERIES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4e76d04a-0038-4b5b-a026-3056ee940da9"),
    "Cloud of Faeries",
    crate::card::CardArt::new("4e76d04a-0038-4b5b-a026-3056ee940da9", "Melissa A. Benson"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 30 — Delusions of Mediocrity
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DELUSIONS_OF_MEDIOCRITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("899088b3-4cdf-47c0-8c52-3c9f55c086c4"),
    "Delusions of Mediocrity",
    crate::card::CardArt::new("899088b3-4cdf-47c0-8c52-3c9f55c086c4", "Jeff Laubenstein"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 31 — Fleeting Image
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FLEETING_IMAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ef9a5501-f149-47d0-9d79-151a524c7c54"),
    "Fleeting Image",
    crate::card::CardArt::new("ef9a5501-f149-47d0-9d79-151a524c7c54", "Scott M. Fischer"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 32 — Frantic Search
pub(in crate::card::sets) static FRANTIC_SEARCH: CardRecord = CardRecord::new_with_legacy_id(
    2078,
    "Frantic Search",
    CardArt::new("6cec132b-939d-4730-9bbd-2760c63c3cb4", "Jeff Miracola"),
    CardSet::UrzasLegacy,
    // Free if three of the lands paying for it untap again, which is why a
    // deck that wants to fill its graveyard plays it over a plain cantrip.
    CardRules::new_instant(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell(
        "Draw two cards, then discard two cards. Untap up to three lands.",
        EffectDef::Sequence(&[
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
            EffectDef::Sequence(&[
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(ObjectSetBindingIndex::PRIMARY),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    // Any lands, not only your own: the printed clause names no controller,
                    // which is what lets it untap a land an opponent's effect left tapped.
                    candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    )),
                    exclude: None,
                    minimum: 0,
                    maximum: 3,
                    visibility: ChoiceVisibilityDef::Public,
                    // The untap follows the discard rather than preceding it, which is the
                    // printed order and the reason the card is free: the lands it untaps can
                    // pay for the spell it just found.
                    then: &EffectDef::Untap {
                        object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                            ObjectSetBindingIndex::PRIMARY,
                        )),
                    },
                }),
            ]),
        ]),
    )),
);

// ULG 33 — Intervene
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static INTERVENE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4b0e3894-5dfe-4d03-9996-eebf96c58168"),
    "Intervene",
    crate::card::CardArt::new("4b0e3894-5dfe-4d03-9996-eebf96c58168", "Heather Hudson"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 34 — King Crab
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KING_CRAB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aedea953-b5f1-4ec7-bd9f-b7827f9d40fe"),
    "King Crab",
    crate::card::CardArt::new("aedea953-b5f1-4ec7-bd9f-b7827f9d40fe", "Daniel Gelon"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 35 — Levitation (reprint)

// ULG 36 — Miscalculation
pub(in crate::card::sets) static MISCALCULATION: CardRecord = CardRecord::new_with_legacy_id(
    2116,
    "Miscalculation",
    CardArt::new("4b4956a2-9a39-4152-9c98-70e4b2acfa26", "Jeff Laubenstein"),
    CardSet::UrzasLegacy,
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Counter target spell unless its controller pays {2}.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Spell,
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                },
            )],
            abilities::counter_target_unless_paid(ValueDef::Constant(2)),
        ),
        abilities::cycling(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            mana_cost!("{2}"),
        ),
    ]),
);

// ULG 37 — Opportunity (reprint)

// ULG 38 — Palinchron
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PALINCHRON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5621db3f-a9e7-4350-9c6a-0ba04a628947"),
    "Palinchron",
    crate::card::CardArt::new("5621db3f-a9e7-4350-9c6a-0ba04a628947", "Matthew D. Wilson"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 39 — Raven Familiar
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RAVEN_FAMILIAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b104638d-29aa-490c-8cfb-e08fc94efb59"),
    "Raven Familiar",
    crate::card::CardArt::new(
        "b104638d-29aa-490c-8cfb-e08fc94efb59",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 40 — Rebuild
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static REBUILD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4dc1613c-a149-4f04-9950-41637d35d675"),
    "Rebuild",
    crate::card::CardArt::new("4dc1613c-a149-4f04-9950-41637d35d675", "Allen Williams"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 41 — Second Chance
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SECOND_CHANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("62d1a0da-40b9-4e79-bace-b93f98ae4695"),
    "Second Chance",
    crate::card::CardArt::new("62d1a0da-40b9-4e79-bace-b93f98ae4695", "Mark Tedin"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 42 — Slow Motion
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SLOW_MOTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2860a20d-e1bf-4e46-8c07-a858f616d5a5"),
    "Slow Motion",
    crate::card::CardArt::new("2860a20d-e1bf-4e46-8c07-a858f616d5a5", "Todd Lockwood"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 43 — Snap
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SNAP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f7e0549e-2d23-4ea8-b8d1-ae21af2c9091"),
    "Snap",
    crate::card::CardArt::new("f7e0549e-2d23-4ea8-b8d1-ae21af2c9091", "Mike Raabe"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 44 — Thornwind Faeries
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static THORNWIND_FAERIES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9cb4b20a-448a-4855-9e60-19625f921a4d"),
    "Thornwind Faeries",
    crate::card::CardArt::new("9cb4b20a-448a-4855-9e60-19625f921a4d", "Rebecca Guay"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 45 — Tinker
pub(in crate::card::sets) static TINKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7da23b15-dfb8-4267-9b33-d7a4c035c434"),
    "Tinker",
    CardArt::new("7da23b15-dfb8-4267-9b33-d7a4c035c434", "Mike Raabe"),
    CardSet::UrzasLegacy,
    // Three mana that turns a Lotus Petal into whatever the deck's best
    // artifact is, which is why it is restricted where it is legal at all.
    CardRules::new_sorcery(mana_cost!("{2}{U}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, sacrifice an artifact.\nSearch your \
             library for an artifact card, put that card onto the battlefield, then shuffle.",
            &[],
            // Any artifact at all, and the one you give up is usually the cheapest
            // thing you own: what the cost measures is a card on the battlefield rather
            // than what it was worth.
            SpellAdditionalCostDef::sacrifice(
                ObjectPredicateDef::HasType(CardType::Artifact),
                CostQuantityDef::Fixed(1),
            ),
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::HasType(CardType::Artifact),
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
        ),
    ),
);

// ULG 46 — Vigilant Drake
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VIGILANT_DRAKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("37940486-2d7f-40d9-9c19-151b9307d374"),
    "Vigilant Drake",
    crate::card::CardArt::new("37940486-2d7f-40d9-9c19-151b9307d374", "Greg Staples"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 47 — Walking Sponge
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WALKING_SPONGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b125d1e7-5d9b-4997-88b0-71bdfc19c6f2"),
    "Walking Sponge",
    crate::card::CardArt::new("b125d1e7-5d9b-4997-88b0-71bdfc19c6f2", "Ron Spencer"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 48 — Weatherseed Faeries
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WEATHERSEED_FAERIES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8c7ebec7-7375-4362-9489-437ff9305f19"),
    "Weatherseed Faeries",
    crate::card::CardArt::new("8c7ebec7-7375-4362-9489-437ff9305f19", "Don Hazeltine"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 49 — Bone Shredder
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BONE_SHREDDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ece050ad-788e-4451-b773-ca42c37549d2"),
    "Bone Shredder",
    crate::card::CardArt::new("ece050ad-788e-4451-b773-ca42c37549d2", "Ron Spencer"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 50 — Brink of Madness
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BRINK_OF_MADNESS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ff5391d8-b546-4159-955e-16bb58052311"),
    "Brink of Madness",
    crate::card::CardArt::new("ff5391d8-b546-4159-955e-16bb58052311", "Donato Giancola"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 51 — Engineered Plague
pub(in crate::card::sets) static ENGINEERED_PLAGUE: CardRecord = CardRecord::new_with_legacy_id(
    2048,
    "Engineered Plague",
    CardArt::new("27e158d5-efb2-4f90-8898-60ede98f7d29", "Michael Sutfin"),
    CardSet::UrzasLegacy,
    CardRules::new_enchantment(mana_cost!("{2}{B}")).with_abilities(&[
        AbilityDef::replacement(
            "As this enchantment enters, choose a creature type.",
            ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(
                BattlefieldEntryScalarChoiceDef::CREATURE_TYPE,
            )),
        ),
        // Both players' creatures, which is what makes it a sideboard card
        // rather than a lord: it shrinks the mirror as readily as the matchup
        // it was brought in for.
        AbilityDef::static_ability(
            "All creatures of the chosen type get -1/-1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    // Creatures of whatever type the Plague named. The chosen type lives on the
                    // enchantment, so the predicate reads it from the ability's source rather
                    // than naming a tribe the way a printed lord does.
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasSourcesChosenScalar(
                            BattlefieldEntryChoiceDestinationDef::CreatureType,
                        ),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-1),
                    ValueDef::Constant(-1),
                ),
            },
        ),
    ]),
);

// ULG 52 — Eviscerator
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static EVISCERATOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("167e7f67-8d44-4134-b7b9-54ccdfb8675c"),
    "Eviscerator",
    crate::card::CardArt::new("167e7f67-8d44-4134-b7b9-54ccdfb8675c", "Michael Sutfin"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 53 — Fog of Gnats
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FOG_OF_GNATS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3f1e9c54-134b-41da-8c3d-ec699d96778a"),
    "Fog of Gnats",
    crate::card::CardArt::new("3f1e9c54-134b-41da-8c3d-ec699d96778a", "Jeff Miracola"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 54 — Giant Cockroach
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GIANT_COCKROACH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0521bf0c-9f43-402e-8065-d2fc02e20194"),
    "Giant Cockroach",
    crate::card::CardArt::new("0521bf0c-9f43-402e-8065-d2fc02e20194", "Heather Hudson"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 55 — Lurking Skirge
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LURKING_SKIRGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9063cc12-a822-4488-856e-93d70ecfe37f"),
    "Lurking Skirge",
    crate::card::CardArt::new("9063cc12-a822-4488-856e-93d70ecfe37f", "Daren Bader"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 56 — No Mercy
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NO_MERCY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4e2fc29c-0223-4b03-864f-eb9149abc921"),
    "No Mercy",
    crate::card::CardArt::new("4e2fc29c-0223-4b03-864f-eb9149abc921", "Mark Tedin"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 57 — Ostracize
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static OSTRACIZE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7b00193a-84ae-4465-943d-01e3d5fa9aca"),
    "Ostracize",
    crate::card::CardArt::new("7b00193a-84ae-4465-943d-01e3d5fa9aca", "Chippy"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 58 — Phyrexian Broodlings
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_BROODLINGS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2313481c-baf9-4dc7-80c7-1ebc6502dce7"),
    "Phyrexian Broodlings",
    crate::card::CardArt::new("2313481c-baf9-4dc7-80c7-1ebc6502dce7", "Daren Bader"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 59 — Phyrexian Debaser
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_DEBASER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("672dcca2-096b-4bcc-9b02-7180c4c0d4c7"),
    "Phyrexian Debaser",
    crate::card::CardArt::new("672dcca2-096b-4bcc-9b02-7180c4c0d4c7", "Mark Tedin"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 60 — Phyrexian Defiler
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_DEFILER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0d555b5e-9f8a-4b1b-a4a6-dee8e177d9e8"),
    "Phyrexian Defiler",
    crate::card::CardArt::new("0d555b5e-9f8a-4b1b-a4a6-dee8e177d9e8", "DiTerlizzi"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 61 — Phyrexian Denouncer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_DENOUNCER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dcf849a0-9b53-4a8a-87a7-dc38d97311ab"),
    "Phyrexian Denouncer",
    crate::card::CardArt::new("dcf849a0-9b53-4a8a-87a7-dc38d97311ab", "Brian Snõddy"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 62 — Phyrexian Plaguelord
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_PLAGUELORD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("307bd530-4b11-428e-864e-e24e96051e3e"),
    "Phyrexian Plaguelord",
    crate::card::CardArt::new("307bd530-4b11-428e-864e-e24e96051e3e", "Kev Walker"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 63 — Phyrexian Reclamation
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_RECLAMATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("228a2bb7-d9f0-47b5-a0d9-2adf1b33e995"),
    "Phyrexian Reclamation",
    crate::card::CardArt::new("228a2bb7-d9f0-47b5-a0d9-2adf1b33e995", "rk post"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 64 — Plague Beetle
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PLAGUE_BEETLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c07f4e55-57f9-49f6-a1a2-1c94dcbe7d71"),
    "Plague Beetle",
    crate::card::CardArt::new("c07f4e55-57f9-49f6-a1a2-1c94dcbe7d71", "Tom Fleming"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 65 — Rank and File
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RANK_AND_FILE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("59481cb5-2cb0-4b8c-84ee-519399862d46"),
    "Rank and File",
    crate::card::CardArt::new("59481cb5-2cb0-4b8c-84ee-519399862d46", "Donato Giancola"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 66 — Sick and Tired
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SICK_AND_TIRED: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8736f8a2-ee8d-49d2-883f-b22cbe3f3645"),
    "Sick and Tired",
    crate::card::CardArt::new("8736f8a2-ee8d-49d2-883f-b22cbe3f3645", "Val Mayerik"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 67 — Sleeper's Guile
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SLEEPER_S_GUILE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a001ca83-35b5-48e5-8337-92258d5affc2"),
    "Sleeper's Guile",
    crate::card::CardArt::new("a001ca83-35b5-48e5-8337-92258d5affc2", "Daren Bader"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 68 — Subversion
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SUBVERSION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("50f1bca9-5831-4e8b-8920-f28ebb3ffb27"),
    "Subversion",
    crate::card::CardArt::new("50f1bca9-5831-4e8b-8920-f28ebb3ffb27", "Rob Alexander"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 69 — Swat
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SWAT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("947b8923-d9d6-4dd8-928b-91be9105ffb4"),
    "Swat",
    crate::card::CardArt::new("947b8923-d9d6-4dd8-928b-91be9105ffb4", "Daren Bader"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 70 — Tethered Skirge
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TETHERED_SKIRGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0ab1d02c-5d9d-4436-af3b-fb7190c1c028"),
    "Tethered Skirge",
    crate::card::CardArt::new("0ab1d02c-5d9d-4436-af3b-fb7190c1c028", "Brian Snõddy"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 71 — Treacherous Link
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TREACHEROUS_LINK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2e5f8581-411b-4403-9c3a-3cf2156f6779"),
    "Treacherous Link",
    crate::card::CardArt::new("2e5f8581-411b-4403-9c3a-3cf2156f6779", "Carl Critchlow"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 72 — Unearth
pub(in crate::card::sets) static UNEARTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b6cb2549-e485-44d6-9d65-7605c568909e"),
    "Unearth",
    CardArt::new("b6cb2549-e485-44d6-9d65-7605c568909e", "Don Hazeltine"),
    CardSet::UrzasLegacy,
    // One black mana for a creature you already paid for, and a cycling
    // cost for the games where there is nothing worth raising.
    CardRules::new_sorcery(mana_cost!("{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Return target creature card with mana value 3 or less from your graveyard to the \
             battlefield.",
            // "Creature card with mana value 3 or less" in your own graveyard. The
            // bound is what keeps a one-mana reanimation honest: it buys back the
            // creature you were going to cast anyway, not the one you cheated in.
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ManaValueAtMost(3),
                    ]),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
            },
        ),
        abilities::cycling(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            mana_cost!("{2}"),
        ),
    ]),
);

// ULG 73 — About Face
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ABOUT_FACE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("85e71828-095b-4729-ab11-c6c39ba29aab"),
    "About Face",
    crate::card::CardArt::new("85e71828-095b-4729-ab11-c6c39ba29aab", "Melissa A. Benson"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 74 — Avalanche Riders
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static AVALANCHE_RIDERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3bdc5330-c76b-40ca-a694-58fa4b9b7304"),
    "Avalanche Riders",
    crate::card::CardArt::new(
        "3bdc5330-c76b-40ca-a694-58fa4b9b7304",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 75 — Defender of Chaos
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DEFENDER_OF_CHAOS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("717ae26a-e5a0-4478-9995-00ea6bd84c03"),
    "Defender of Chaos",
    crate::card::CardArt::new("717ae26a-e5a0-4478-9995-00ea6bd84c03", "Carl Critchlow"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 76 — Ghitu Fire-Eater
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GHITU_FIRE_EATER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("131dce1c-e9c8-437a-b7aa-36a47049d2d2"),
    "Ghitu Fire-Eater",
    crate::card::CardArt::new("131dce1c-e9c8-437a-b7aa-36a47049d2d2", "Melissa A. Benson"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 77 — Ghitu Slinger
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GHITU_SLINGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("67e4bc1d-6a4b-408a-8921-433249c960f9"),
    "Ghitu Slinger",
    crate::card::CardArt::new("67e4bc1d-6a4b-408a-8921-433249c960f9", "Melissa A. Benson"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 78 — Ghitu War Cry
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GHITU_WAR_CRY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c9786c4f-f09b-46ce-966c-10efb1e5e609"),
    "Ghitu War Cry",
    crate::card::CardArt::new("c9786c4f-f09b-46ce-966c-10efb1e5e609", "Douglas Shuler"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 79 — Goblin Medics
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_MEDICS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("72cc08b6-f31a-46b3-b233-f6bb2c6b1106"),
    "Goblin Medics",
    crate::card::CardArt::new("72cc08b6-f31a-46b3-b233-f6bb2c6b1106", "Jeff Laubenstein"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 80 — Goblin Welder
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_WELDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6171e136-1167-4329-acb2-6853d3a814e5"),
    "Goblin Welder",
    crate::card::CardArt::new("6171e136-1167-4329-acb2-6853d3a814e5", "Scott M. Fischer"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 81 — Granite Grip
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GRANITE_GRIP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ee9e0e7e-ada8-49f5-9dd9-f62464697675"),
    "Granite Grip",
    crate::card::CardArt::new("ee9e0e7e-ada8-49f5-9dd9-f62464697675", "Mike Raabe"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 82 — Impending Disaster
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static IMPENDING_DISASTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("44497303-9686-4810-bf0f-876dd9696cab"),
    "Impending Disaster",
    crate::card::CardArt::new("44497303-9686-4810-bf0f-876dd9696cab", "Pete Venters"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 83 — Last-Ditch Effort
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LAST_DITCH_EFFORT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("295f7fe0-0681-4b25-807f-30ed70ec78d5"),
    "Last-Ditch Effort",
    crate::card::CardArt::new("295f7fe0-0681-4b25-807f-30ed70ec78d5", "Dan Frazier"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 84 — Lava Axe (reprint)

// ULG 85 — Molten Hydra
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MOLTEN_HYDRA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("95234b29-9ac8-4200-b42d-9653ba51b010"),
    "Molten Hydra",
    crate::card::CardArt::new("95234b29-9ac8-4200-b42d-9653ba51b010", "Greg Staples"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 86 — Parch
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PARCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d3ab8065-cecc-4b19-be93-7cf791a93e62"),
    "Parch",
    crate::card::CardArt::new("d3ab8065-cecc-4b19-be93-7cf791a93e62", "Ron Spencer"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 87 — Pygmy Pyrosaur
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PYGMY_PYROSAUR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("96136626-4777-4e58-865b-c4d3f6ceb59d"),
    "Pygmy Pyrosaur",
    crate::card::CardArt::new("96136626-4777-4e58-865b-c4d3f6ceb59d", "Dan Frazier"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 88 — Pyromancy
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PYROMANCY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("35a91a58-cc8b-47a3-8c53-43c32753a00d"),
    "Pyromancy",
    crate::card::CardArt::new("35a91a58-cc8b-47a3-8c53-43c32753a00d", "Quinton Hoover"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 89 — Rack and Ruin
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RACK_AND_RUIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a46e8f6a-3a1a-4c30-9348-4b31882267eb"),
    "Rack and Ruin",
    crate::card::CardArt::new("a46e8f6a-3a1a-4c30-9348-4b31882267eb", "Donato Giancola"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 90 — Rivalry
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RIVALRY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3b79677b-d5b9-47c7-a5f5-45446d5cddff"),
    "Rivalry",
    crate::card::CardArt::new("3b79677b-d5b9-47c7-a5f5-45446d5cddff", "Brian Snõddy"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 91 — Shivan Phoenix
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHIVAN_PHOENIX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("112aa0e2-7e4a-4ae8-bedb-d84b4116df5e"),
    "Shivan Phoenix",
    crate::card::CardArt::new("112aa0e2-7e4a-4ae8-bedb-d84b4116df5e", "Daren Bader"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 92 — Sluggishness
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SLUGGISHNESS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2ba91431-3fcd-4b44-ae7b-a69eb18efd5f"),
    "Sluggishness",
    crate::card::CardArt::new("2ba91431-3fcd-4b44-ae7b-a69eb18efd5f", "Pete Venters"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 93 — Viashino Bey
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VIASHINO_BEY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("26cc07c6-60c7-4abe-8197-7544887ec64d"),
    "Viashino Bey",
    crate::card::CardArt::new("26cc07c6-60c7-4abe-8197-7544887ec64d", "Bradley Williams"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 94 — Viashino Cutthroat
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VIASHINO_CUTTHROAT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bcbab69d-3259-40f4-a588-ab550858a178"),
    "Viashino Cutthroat",
    crate::card::CardArt::new(
        "bcbab69d-3259-40f4-a588-ab550858a178",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 95 — Viashino Heretic
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VIASHINO_HERETIC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("143e435e-e3a1-45b0-81c3-bd47916df8ac"),
    "Viashino Heretic",
    crate::card::CardArt::new("143e435e-e3a1-45b0-81c3-bd47916df8ac", "Douglas Shuler"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 96 — Viashino Sandscout
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VIASHINO_SANDSCOUT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("12dd888a-ca98-44dd-a213-858c3539dc97"),
    "Viashino Sandscout",
    crate::card::CardArt::new("12dd888a-ca98-44dd-a213-858c3539dc97", "Scott M. Fischer"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 97 — Bloated Toad
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BLOATED_TOAD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9686f1a8-035e-415e-9a06-933d6ce1cd5c"),
    "Bloated Toad",
    crate::card::CardArt::new("9686f1a8-035e-415e-9a06-933d6ce1cd5c", "Una Fricker"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 98 — Crop Rotation
pub(in crate::card::sets) static CROP_ROTATION: CardRecord = CardRecord::new_with_legacy_id(
    2143,
    "Crop Rotation",
    CardArt::new("6563f790-862c-465a-b963-7a61f2385516", "DiTerlizzi"),
    CardSet::UrzasLegacy,
    CardRules::new_instant(mana_cost!("{G}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, sacrifice a land.\nSearch your library for a land card, put that card onto the battlefield, then shuffle.",
            &[],
            // Sacrificing a land is what makes this an instant-speed tutor rather than a
            // ramp spell: the land you give up pays for the one you go and get, so the
            // board count never moves.
            SpellAdditionalCostDef::sacrifice(
                ObjectPredicateDef::HasType(CardType::Land),
                CostQuantityDef::Fixed(1),
            ),
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::HasType(CardType::Land),
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
        ),
    ),
);

// ULG 99 — Darkwatch Elves
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DARKWATCH_ELVES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("212a807f-d5d0-4787-b390-3351783a1ae4"),
    "Darkwatch Elves",
    crate::card::CardArt::new("212a807f-d5d0-4787-b390-3351783a1ae4", "Don Hazeltine"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 100 — Defense of the Heart
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DEFENSE_OF_THE_HEART: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9e93381f-627f-41b6-b1b6-45d712a44d8e"),
    "Defense of the Heart",
    crate::card::CardArt::new("9e93381f-627f-41b6-b1b6-45d712a44d8e", "Rebecca Guay"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 101 — Deranged Hermit
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DERANGED_HERMIT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bf0e94c9-61c4-4cc0-b5ce-db62bc2660ee"),
    "Deranged Hermit",
    crate::card::CardArt::new("bf0e94c9-61c4-4cc0-b5ce-db62bc2660ee", "Kev Walker"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 102 — Gang of Elk
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GANG_OF_ELK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5a84177f-43a3-4d14-9a4c-2ca931cfe092"),
    "Gang of Elk",
    crate::card::CardArt::new("5a84177f-43a3-4d14-9a4c-2ca931cfe092", "Una Fricker"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 103 — Harmonic Convergence
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HARMONIC_CONVERGENCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5aafc380-cf4d-4843-b9c3-c389d9c5e942"),
    "Harmonic Convergence",
    crate::card::CardArt::new("5aafc380-cf4d-4843-b9c3-c389d9c5e942", "John Avon"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 104 — Hidden Gibbons
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HIDDEN_GIBBONS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8f94da51-b4d9-4d79-9113-39f8f4a1be34"),
    "Hidden Gibbons",
    crate::card::CardArt::new("8f94da51-b4d9-4d79-9113-39f8f4a1be34", "Una Fricker"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 105 — Lone Wolf (reprint)

// ULG 106 — Might of Oaks
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MIGHT_OF_OAKS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5e48b989-bb64-4c71-9921-0a230fed5b11"),
    "Might of Oaks",
    crate::card::CardArt::new("5e48b989-bb64-4c71-9921-0a230fed5b11", "Ron Spencer"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 107 — Multani, Maro-Sorcerer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MULTANI_MARO_SORCERER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0d6cc98b-b376-40af-8308-198bab00b2b1"),
    "Multani, Maro-Sorcerer",
    crate::card::CardArt::new("0d6cc98b-b376-40af-8308-198bab00b2b1", "Daren Bader"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 108 — Multani's Acolyte
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MULTANI_S_ACOLYTE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4e5fdecb-bca0-48ea-b5bb-d0886c7d3316"),
    "Multani's Acolyte",
    crate::card::CardArt::new(
        "4e5fdecb-bca0-48ea-b5bb-d0886c7d3316",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 109 — Multani's Presence
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MULTANI_S_PRESENCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("38bfa984-5fe9-44ad-b13f-3276951f9f10"),
    "Multani's Presence",
    crate::card::CardArt::new("38bfa984-5fe9-44ad-b13f-3276951f9f10", "Scott M. Fischer"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 110 — Rancor
pub(in crate::card::sets) static RANCOR: CardRecord = CardRecord::new_with_legacy_id(
    2124,
    "Rancor",
    CardArt::new("59e256c2-38df-4012-9308-ce17dd889e5f", "Kev Walker"),
    CardSet::UrzasLegacy,
    CardRules::new_enchantment(mana_cost!("{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+0 and has trample.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(ValueDef::Constant(2), ValueDef::Constant(0)),
                        AppliedEffectDef::add_ability(&abilities::trample()),
                    ]),
                },
            ),
            // An Aura put into the graveyard with its host still triggers, so
            // this fires whether the creature was answered or the Aura was.
            // It is the same trigger either way, and the card that comes back
            // is the one already in the graveyard.
            abilities::dies_trigger("When this Aura is put into a graveyard from the battlefield, return it to its owner's hand.", EffectDef::MoveToZone {
                    object: EffectRecipientDef::TriggeringZoneChangeResult,
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
}),
        ]),
);

// ULG 111 — Repopulate
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static REPOPULATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aac77869-97bc-4976-9ee0-3d60e162b78a"),
    "Repopulate",
    crate::card::CardArt::new("aac77869-97bc-4976-9ee0-3d60e162b78a", "Una Fricker"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 112 — Silk Net
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SILK_NET: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9498a97a-0e32-4eb8-9cb4-0698ff3a7ded"),
    "Silk Net",
    crate::card::CardArt::new("9498a97a-0e32-4eb8-9cb4-0698ff3a7ded", "Rob Alexander"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 113 — Simian Grunts
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SIMIAN_GRUNTS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a0aaea3e-a67a-4d9c-9059-e6beb05f97b1"),
    "Simian Grunts",
    crate::card::CardArt::new("a0aaea3e-a67a-4d9c-9059-e6beb05f97b1", "Pete Venters"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 114 — Treefolk Mystic
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TREEFOLK_MYSTIC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9697acf5-9bc5-411d-8574-fe6185f18672"),
    "Treefolk Mystic",
    crate::card::CardArt::new("9697acf5-9bc5-411d-8574-fe6185f18672", "DiTerlizzi"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 115 — Weatherseed Elf
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WEATHERSEED_ELF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4e74f8ae-992b-40a6-87e3-7b321dba4ffa"),
    "Weatherseed Elf",
    crate::card::CardArt::new("4e74f8ae-992b-40a6-87e3-7b321dba4ffa", "Heather Hudson"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 116 — Weatherseed Treefolk
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WEATHERSEED_TREEFOLK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f42cce45-3b6a-43e2-8329-68c30135c5c1"),
    "Weatherseed Treefolk",
    crate::card::CardArt::new("f42cce45-3b6a-43e2-8329-68c30135c5c1", "Heather Hudson"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 117 — Wing Snare
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WING_SNARE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("19116d5d-8f2d-4e85-849d-1fbaa67e8cfd"),
    "Wing Snare",
    crate::card::CardArt::new(
        "19116d5d-8f2d-4e85-849d-1fbaa67e8cfd",
        "Henry Van Der Linde",
    ),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 118 — Yavimaya Granger
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static YAVIMAYA_GRANGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("05414a41-b50c-49b6-9c27-f3170017d9b0"),
    "Yavimaya Granger",
    crate::card::CardArt::new(
        "05414a41-b50c-49b6-9c27-f3170017d9b0",
        "Henry Van Der Linde",
    ),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 119 — Yavimaya Scion
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static YAVIMAYA_SCION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a2f80036-1058-4513-8549-0557df9b5d61"),
    "Yavimaya Scion",
    crate::card::CardArt::new("a2f80036-1058-4513-8549-0557df9b5d61", "DiTerlizzi"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 120 — Yavimaya Wurm
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static YAVIMAYA_WURM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dde16069-7176-42dc-88d8-fb37b7894007"),
    "Yavimaya Wurm",
    crate::card::CardArt::new("dde16069-7176-42dc-88d8-fb37b7894007", "Melissa A. Benson"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 121 — Angel's Trumpet
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ANGEL_S_TRUMPET: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0c7b248a-3c74-4592-b357-47989568298c"),
    "Angel's Trumpet",
    crate::card::CardArt::new("0c7b248a-3c74-4592-b357-47989568298c", "Kev Walker"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 122 — Beast of Burden
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BEAST_OF_BURDEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("42db4cd3-6351-498e-944a-4b93e32e1494"),
    "Beast of Burden",
    crate::card::CardArt::new("06578d72-50e9-468d-96d2-c0cbda14961a", "Ron Spears"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 123 — Crawlspace
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CRAWLSPACE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4913a54f-f4d0-483a-a181-716007f65658"),
    "Crawlspace",
    crate::card::CardArt::new("4913a54f-f4d0-483a-a181-716007f65658", "Douglas Shuler"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 124 — Damping Engine
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DAMPING_ENGINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("87841977-75ff-49c3-b832-3f0cf48b50b2"),
    "Damping Engine",
    crate::card::CardArt::new("87841977-75ff-49c3-b832-3f0cf48b50b2", "rk post"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 125 — Defense Grid
pub(in crate::card::sets) static DEFENSE_GRID: CardRecord = CardRecord::new_with_legacy_id(
    2065,
    "Defense Grid",
    CardArt::new("5c2592c9-3f8c-4b7e-9e0a-4a6f2c1d8b3e", "Mark Tedin"),
    CardSet::UrzasLegacy,
    // "Except during its controller's turn" is the nonactive player: the tax
    // lands on the instant held up and not on the sorcery cast on time.
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(AbilityDef::static_ability(
        "Each spell costs {3} more to cast except during its controller's turn.",
        EffectDef::ModifyCost(CostModificationDef::increase_spell(
            ObjectPredicateDef::Any,
            PlayerRelation::NonactivePlayer,
            mana_cost!("{3}"),
        )),
    )),
);

// ULG 126 — Grim Monolith
pub(in crate::card::sets) static GRIM_MONOLITH: CardRecord = CardRecord::new_with_legacy_id(
    2118,
    "Grim Monolith",
    CardArt::new("9ddc9fe1-17c8-4e1d-aeb8-c4214e881280", "Chippy"),
    CardSet::UrzasLegacy,
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::static_ability(
            "This artifact doesn't untap during your untap step.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
            },
        ),
        AbilityDef::activated_mana(
            "{T}: Add {C}{C}{C}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless).with_amount(3)),
        ),
        AbilityDef::activated(
            "{4}: Untap this artifact.",
            &[AbilityCostDef::Mana(mana_cost!("{4}"))],
            EffectDef::Untap {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// ULG 127 — Iron Maiden
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static IRON_MAIDEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ad925fb0-1d5c-44a0-8347-202a38c23107"),
    "Iron Maiden",
    crate::card::CardArt::new("ad925fb0-1d5c-44a0-8347-202a38c23107", "Tom Wänerstrand"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 128 — Jhoira's Toolbox
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static JHOIRA_S_TOOLBOX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("edb38309-c02c-496c-894f-786a2f6e3d1c"),
    "Jhoira's Toolbox",
    crate::card::CardArt::new("edb38309-c02c-496c-894f-786a2f6e3d1c", "Mike Raabe"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 129 — Memory Jar
pub(in crate::card::sets) static MEMORY_JAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a15d33d6-7213-4482-a1be-ac0a73644af6"),
    "Memory Jar",
    CardArt::new("a15d33d6-7213-4482-a1be-ac0a73644af6", "Donato Giancola"),
    CardSet::UrzasLegacy,
    // Seven cards for everyone, and everyone gets their old hand back at the
    // end of the turn -- which is a windfall only for the player who built a
    // deck that can spend seven cards in one turn.
    CardRules::new_artifact(mana_cost!("{5}")).with_ability(AbilityDef::activated(
        "{T}, Sacrifice this artifact: Each player exiles all cards from their hand face down \
         and draws seven cards. At the beginning of the next end step, each player discards \
         their hand and returns to their hand each card they exiled this way.",
        &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
        EffectDef::Sequence(&const {
            [
                // Face down: the point of the clause is that nobody learns what the
                // other player put away, only how much of it there was.
                EffectDef::ExileLinkedToSource {
                    until_source_leaves: false,
                    // Everything in both hands, wherever it came from. The exile is linked to
                    // the Jar so the end step can name exactly these cards rather than
                    // everything that happens to be in exile by then.
                    object: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::Any,
                        &const { [ZoneKind::Hand] },
                        PlayerRelation::Any,
                    ),
                    face_down: true,
                    then: None,
                },
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::EachPlayer,
                    amount: ValueDef::Constant(7),
                },
                // The discard comes first and the return second, which is what makes
                // the seven new cards a loan rather than a hand: whatever is left of
                // them at the end step is thrown away.
                EffectDef::InstallTrigger(InstalledTriggerDef::once(&const {
                    AbilityDef::triggered(
                        "At the beginning of the next end step, each player discards their hand and returns to \
                         their hand each card they exiled this way.",
                        TriggerEventDef::StepBegins {
                            step: TurnStepDef::End,
                            player: PlayerRelation::Any,
                        },
                        EffectDef::Sequence(&const {
                            [
                                EffectDef::Discard {
                                    recipient: EffectRecipientDef::EachPlayer,
                                    // `Discard` saturates at the recipient's hand size, so the largest amount
                                    // is how "their hand" is said.
                                    amount: ValueDef::Constant(i32::MAX),
                                    selection: DiscardSelectionDef::RecipientChooses,
                                    then: None,
                                },
                                EffectDef::ReturnLinkedExiles {
                                    object: ObjectPredicateDef::Any,
                                    counters: None,
                                    zone: ZoneKind::Hand,
                                    grant: None,
                                    controller: None,
                                    transformed: false,
                                },
                            ]
                        }),
                    )
                })),
            ]
        }),
    )),
);

// ULG 130 — Quicksilver Amulet (reprint)

// ULG 131 — Ring of Gix
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RING_OF_GIX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0b09dc9b-ed01-49de-9675-48c41f428385"),
    "Ring of Gix",
    crate::card::CardArt::new("0b09dc9b-ed01-49de-9675-48c41f428385", "Mark Tedin"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 132 — Scrapheap
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SCRAPHEAP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("14aa4474-96a0-4c1d-a09d-73b9c1073b00"),
    "Scrapheap",
    crate::card::CardArt::new("14aa4474-96a0-4c1d-a09d-73b9c1073b00", "Bradley Williams"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 133 — Thran Lens
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static THRAN_LENS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("200c3666-5ba0-4f0a-adbe-a97af0aa28d1"),
    "Thran Lens",
    crate::card::CardArt::new("200c3666-5ba0-4f0a-adbe-a97af0aa28d1", "Allen Williams"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 134 — Thran War Machine
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static THRAN_WAR_MACHINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5908714a-be91-4279-b87e-e2bc09dbaaba"),
    "Thran War Machine",
    crate::card::CardArt::new("5908714a-be91-4279-b87e-e2bc09dbaaba", "Pete Venters"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 135 — Thran Weaponry
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static THRAN_WEAPONRY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("60f005ae-fca1-4a48-84a3-4b217ac879ce"),
    "Thran Weaponry",
    crate::card::CardArt::new("60f005ae-fca1-4a48-84a3-4b217ac879ce", "Anthony S. Waters"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 136 — Ticking Gnomes
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TICKING_GNOMES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6241755c-ff3d-44db-a99d-960bea54633e"),
    "Ticking Gnomes",
    crate::card::CardArt::new(
        "6241755c-ff3d-44db-a99d-960bea54633e",
        "Henry Van Der Linde",
    ),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 137 — Urza's Blueprints
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static URZA_S_BLUEPRINTS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("026f0d4b-c13e-48a6-915f-b0edd2ac0ae8"),
    "Urza's Blueprints",
    crate::card::CardArt::new("026f0d4b-c13e-48a6-915f-b0edd2ac0ae8", "Tom Wänerstrand"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 138 — Wheel of Torture
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WHEEL_OF_TORTURE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("87edc873-169a-4cd3-8a94-84b5810b5ed8"),
    "Wheel of Torture",
    crate::card::CardArt::new(
        "87edc873-169a-4cd3-8a94-84b5810b5ed8",
        "Henry Van Der Linde",
    ),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 139 — Faerie Conclave
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FAERIE_CONCLAVE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ae3ede87-b026-4781-81ab-8652664f8e41"),
    "Faerie Conclave",
    crate::card::CardArt::new("ae3ede87-b026-4781-81ab-8652664f8e41", "Val Mayerik"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 140 — Forbidding Watchtower
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FORBIDDING_WATCHTOWER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("96503ed7-aa68-439f-95b0-6ac2c48e3935"),
    "Forbidding Watchtower",
    crate::card::CardArt::new("96503ed7-aa68-439f-95b0-6ac2c48e3935", "Mark Brill"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 141 — Ghitu Encampment
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GHITU_ENCAMPMENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bf09ecef-1e30-4206-9648-8fe5c8a71c71"),
    "Ghitu Encampment",
    crate::card::CardArt::new("bf09ecef-1e30-4206-9648-8fe5c8a71c71", "Don Hazeltine"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 142 — Spawning Pool
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SPAWNING_POOL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("43ffec42-57f7-4592-99ab-6284d59829a1"),
    "Spawning Pool",
    crate::card::CardArt::new("43ffec42-57f7-4592-99ab-6284d59829a1", "Rob Alexander"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

// ULG 143 — Treetop Village
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TREETOP_VILLAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("02212bd8-0c0f-4e8e-99f1-a8477476c03a"),
    "Treetop Village",
    crate::card::CardArt::new("02212bd8-0c0f-4e8e-99f1-a8477476c03a", "Anthony S. Waters"),
    crate::card::CardSet::UrzasLegacy,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ANGELIC_CURATOR,
    &BLESSED_REVERSAL,
    &BURST_OF_ENERGY,
    &CESSATION,
    &DEFENDER_OF_LAW,
    &DEVOUT_HARPIST,
    &EXPENDABLE_TROOPS,
    &HOPE_AND_GLORY,
    &IRON_WILL,
    &KARMIC_GUIDE,
    &KNIGHTHOOD,
    &MARTYR_S_CAUSE,
    &MOTHER_OF_RUNES,
    &OPAL_AVENGER,
    &OPAL_CHAMPION,
    &PEACE_AND_QUIET,
    &PLANAR_COLLAPSE,
    &PURIFY,
    &RADIANT_ARCHANGEL,
    &RADIANT_S_DRAGOONS,
    &RADIANT_S_JUDGMENT,
    &SUSTAINER_OF_THE_REALM,
    &TRAGIC_POET,
    &ANTHROPLASM,
    &ARCHIVIST,
    &AURA_FLUX,
    &BOUNCING_BEEBLES,
    &CLOUD_OF_FAERIES,
    &DELUSIONS_OF_MEDIOCRITY,
    &FLEETING_IMAGE,
    &FRANTIC_SEARCH,
    &INTERVENE,
    &KING_CRAB,
    &MISCALCULATION,
    &PALINCHRON,
    &RAVEN_FAMILIAR,
    &REBUILD,
    &SECOND_CHANCE,
    &SLOW_MOTION,
    &SNAP,
    &THORNWIND_FAERIES,
    &TINKER,
    &VIGILANT_DRAKE,
    &WALKING_SPONGE,
    &WEATHERSEED_FAERIES,
    &BONE_SHREDDER,
    &BRINK_OF_MADNESS,
    &ENGINEERED_PLAGUE,
    &EVISCERATOR,
    &FOG_OF_GNATS,
    &GIANT_COCKROACH,
    &LURKING_SKIRGE,
    &NO_MERCY,
    &OSTRACIZE,
    &PHYREXIAN_BROODLINGS,
    &PHYREXIAN_DEBASER,
    &PHYREXIAN_DEFILER,
    &PHYREXIAN_DENOUNCER,
    &PHYREXIAN_PLAGUELORD,
    &PHYREXIAN_RECLAMATION,
    &PLAGUE_BEETLE,
    &RANK_AND_FILE,
    &SICK_AND_TIRED,
    &SLEEPER_S_GUILE,
    &SUBVERSION,
    &SWAT,
    &TETHERED_SKIRGE,
    &TREACHEROUS_LINK,
    &UNEARTH,
    &ABOUT_FACE,
    &AVALANCHE_RIDERS,
    &DEFENDER_OF_CHAOS,
    &GHITU_FIRE_EATER,
    &GHITU_SLINGER,
    &GHITU_WAR_CRY,
    &GOBLIN_MEDICS,
    &GOBLIN_WELDER,
    &GRANITE_GRIP,
    &IMPENDING_DISASTER,
    &LAST_DITCH_EFFORT,
    &MOLTEN_HYDRA,
    &PARCH,
    &PYGMY_PYROSAUR,
    &PYROMANCY,
    &RACK_AND_RUIN,
    &RIVALRY,
    &SHIVAN_PHOENIX,
    &SLUGGISHNESS,
    &VIASHINO_BEY,
    &VIASHINO_CUTTHROAT,
    &VIASHINO_HERETIC,
    &VIASHINO_SANDSCOUT,
    &BLOATED_TOAD,
    &CROP_ROTATION,
    &DARKWATCH_ELVES,
    &DEFENSE_OF_THE_HEART,
    &DERANGED_HERMIT,
    &GANG_OF_ELK,
    &HARMONIC_CONVERGENCE,
    &HIDDEN_GIBBONS,
    &MIGHT_OF_OAKS,
    &MULTANI_MARO_SORCERER,
    &MULTANI_S_ACOLYTE,
    &MULTANI_S_PRESENCE,
    &RANCOR,
    &REPOPULATE,
    &SILK_NET,
    &SIMIAN_GRUNTS,
    &TREEFOLK_MYSTIC,
    &WEATHERSEED_ELF,
    &WEATHERSEED_TREEFOLK,
    &WING_SNARE,
    &YAVIMAYA_GRANGER,
    &YAVIMAYA_SCION,
    &YAVIMAYA_WURM,
    &ANGEL_S_TRUMPET,
    &BEAST_OF_BURDEN,
    &CRAWLSPACE,
    &DAMPING_ENGINE,
    &DEFENSE_GRID,
    &GRIM_MONOLITH,
    &IRON_MAIDEN,
    &JHOIRA_S_TOOLBOX,
    &MEMORY_JAR,
    &RING_OF_GIX,
    &SCRAPHEAP,
    &THRAN_LENS,
    &THRAN_WAR_MACHINE,
    &THRAN_WEAPONRY,
    &TICKING_GNOMES,
    &URZA_S_BLUEPRINTS,
    &WHEEL_OF_TORTURE,
    &FAERIE_CONCLAVE,
    &FORBIDDING_WATCHTOWER,
    &GHITU_ENCAMPMENT,
    &SPAWNING_POOL,
    &TREETOP_VILLAGE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&catalog_m13::ERASE), // ULG 7
    PrintingRecord::reprint(&catalog_m12::LEVITATION), // ULG 35
    PrintingRecord::reprint(&catalog_m14::OPPORTUNITY), // ULG 37
    PrintingRecord::reprint(&catalog_m14::LAVA_AXE), // ULG 84
    PrintingRecord::reprint(&catalog_p02::LONE_WOLF), // ULG 105
    PrintingRecord::reprint(&catalog_m12::QUICKSILVER_AMULET), // ULG 130
];
