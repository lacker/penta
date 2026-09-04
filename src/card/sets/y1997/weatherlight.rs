//! Weatherlight cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::KeywordAbility;
use crate::card::abilities;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedEffectDef, AppliedRuleDef, BattlefieldEntryModificationDef, CardRules, CardSet,
    CardType, CostModificationDef, CounterKind, EffectDef, EffectPaymentDef, EffectRecipientDef,
    HalvedValueDef, ManaColor, ObjectPredicateDef, ObjectSetDef, PayOrDef, PlayActionMatcherDef,
    PlayRestrictionDef, PlayerRefDef, PlayerRelation, PlayerSetDef, ReplacementEffectDef,
    ResolvedEffectDurationDef, RoundingDef, TriggerEventDef, ValueDef, ZoneKind, ZonePlacement,
};
use crate::{TargetIndex, mana_cost};

// WTH 1 — Abeyance
pub(in crate::card::sets) static ABEYANCE: CardRecord = CardRecord::new(
    CardSet::Weatherlight,
    "Abeyance",
    "125a355d-bfcf-4125-aa6c-35e7dea6f63e",
    "Thomas Gianni",
    // A counterspell that replaces itself and stops the next one too: the
    // deck holding it is buying one turn without interaction.
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Until end of turn, target player can't cast instant or sorcery spells, and that player can't activate abilities that aren't mana abilities.\nDraw a card.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::target_players(TargetIndex::PRIMARY),
                // Both halves of the same lock, applied to the same player for the same
                // turn: no instants or sorceries, and no activations but mana abilities.
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(PlayRestrictionDef::new(
                        PlayActionMatcherDef::CastSpell,
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Instant),
                            ObjectPredicateDef::HasType(CardType::Sorcery),
                        ]),
                    ))),
                    AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(PlayRestrictionDef::new(
                        PlayActionMatcherDef::ActivateNonManaAbility,
                        ObjectPredicateDef::Any,
                    ))),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// WTH 2 — Alabaster Dragon (reprint)
const ALABASTER_DRAGON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::ALABASTER_DRAGON,
    "3a2fcc23-ac09-4ada-b194-424739c9c734",
    "Bob Eggleton",
);

// WTH 3 — Alms
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALMS: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Alms",
    "97382dd8-2754-4ca3-8ba8-d655acaf22ac",
    "Rogério Vilela",
    crate::card::CardRules::unsupported(),
);

// WTH 4 — Angelic Renewal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANGELIC_RENEWAL: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Angelic Renewal",
    "7dddde7d-8565-45a7-a1db-f2dea2a6a3ba",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// WTH 5 — Ardent Militia (reprint)
const ARDENT_MILITIA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::ARDENT_MILITIA,
    "bb212ca5-bbb5-4c83-9a7b-9d5ab451e032",
    "Zina Saunders",
);

// WTH 6 — Argivian Find
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARGIVIAN_FIND: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Argivian Find",
    "89f23295-ad0a-4e2d-ae04-1a9c065e575d",
    "Roger Raupp",
    crate::card::CardRules::unsupported(),
);

// WTH 7 — Aura of Silence
/// The tax names spells an opponent casts, so it never touches your own.
static OPPONENTS_ARTIFACTS_AND_ENCHANTMENTS: ObjectPredicateDef = ObjectPredicateDef::AnyOf(&[
    ObjectPredicateDef::HasType(CardType::Artifact),
    ObjectPredicateDef::HasType(CardType::Enchantment),
]);

pub(in crate::card::sets) static AURA_OF_SILENCE: CardRecord = CardRecord::new(
    CardSet::Weatherlight,
    "Aura of Silence",
    "57e6c366-b8c7-4f66-b8e1-82dc69c0081c",
    "D. Alexander Gregory",
    // It taxes while it sits and answers something on the way out, so the
    // opponent pays either way.
    CardRules::new_enchantment(mana_cost!("{1}{W}{W}")).with_abilities(&[
        AbilityDef::static_ability(
            "Artifact and enchantment spells your opponents cast cost {2} more to cast.",
            EffectDef::ModifyCost(CostModificationDef::increase_spell(
                OPPONENTS_ARTIFACTS_AND_ENCHANTMENTS,
                PlayerRelation::Opponent,
                mana_cost!("{2}"),
            )),
        ),
        AbilityDef::activated_with_targets(
            "Sacrifice this enchantment: Destroy target artifact or enchantment.",
            &[AbilityCostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one_permanent(
                OPPONENTS_ARTIFACTS_AND_ENCHANTMENTS,
            )],
            EffectDef::destroy_target(TargetIndex::PRIMARY, true),
        ),
    ]),
);

// WTH 8 — Benalish Infantry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BENALISH_INFANTRY: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Benalish Infantry",
    "e8472303-b8ee-402b-a9ea-49abe2e01152",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// WTH 9 — Benalish Knight
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BENALISH_KNIGHT: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Benalish Knight",
    "c2c184bb-6c7d-4118-a111-ef27171cfee6",
    "Zina Saunders",
    crate::card::CardRules::unsupported(),
);

// WTH 10 — Benalish Missionary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BENALISH_MISSIONARY: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Benalish Missionary",
    "e9ac1992-6212-4f05-af16-c892dfc40643",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// WTH 11 — Debt of Loyalty
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEBT_OF_LOYALTY: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Debt of Loyalty",
    "d19ed33b-42d4-4a5d-a763-cfb43348769c",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// WTH 12 — Duskrider Falcon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DUSKRIDER_FALCON: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Duskrider Falcon",
    "bee3a23a-6ecf-439c-8637-e096fa8c1a80",
    "Cecil Fernando",
    crate::card::CardRules::unsupported(),
);

// WTH 13 — Empyrial Armor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EMPYRIAL_ARMOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Empyrial Armor",
    "5518a79f-bcae-417a-b01b-b6ff572be0be",
    "D. Alexander Gregory",
    crate::card::CardRules::unsupported(),
);

// WTH 14 — Foriysian Brigade
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORIYSIAN_BRIGADE: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Foriysian Brigade",
    "0d11b6ef-3a24-4709-a62f-c5e062a6cee1",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// WTH 15 — Gerrard's Wisdom
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GERRARD_S_WISDOM: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Gerrard's Wisdom",
    "f81defa5-edb4-4f1f-b13c-7cfb34511138",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// WTH 16 — Guided Strike
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GUIDED_STRIKE: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Guided Strike",
    "c6e8ec37-abe8-45a9-a1a0-6d4e37c74c45",
    "Gary Leach",
    crate::card::CardRules::unsupported(),
);

// WTH 17 — Heavy Ballista
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEAVY_BALLISTA: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Heavy Ballista",
    "bdfe3eed-e415-4b28-8b4d-e50a19235683",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// WTH 18 — Inner Sanctum
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INNER_SANCTUM: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Inner Sanctum",
    "2298faae-370e-4b87-bf32-d20c2282a928",
    "D. Alexander Gregory",
    crate::card::CardRules::unsupported(),
);

// WTH 19 — Kithkin Armor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KITHKIN_ARMOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Kithkin Armor",
    "395e7882-0429-46aa-8e38-be707067c588",
    "Charles Gillespie",
    crate::card::CardRules::unsupported(),
);

// WTH 20 — Master of Arms
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MASTER_OF_ARMS: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Master of Arms",
    "ac97ff43-c0b6-4f67-ad09-5ba8710c681a",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// WTH 21 — Mistmoon Griffin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISTMOON_GRIFFIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Mistmoon Griffin",
    "8ec71a29-19db-4747-8276-7fd4d563d4df",
    "David A. Cherry",
    crate::card::CardRules::unsupported(),
);

// WTH 22 — Peacekeeper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PEACEKEEPER: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Peacekeeper",
    "592a5683-5f2f-4933-9fc3-5f7773f72f93",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// WTH 23 — Revered Unicorn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REVERED_UNICORN: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Revered Unicorn",
    "8c642dd2-1a3e-4b08-917e-6e8aed358b72",
    "David A. Cherry",
    crate::card::CardRules::unsupported(),
);

// WTH 24 — Serenity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SERENITY: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Serenity",
    "dca975ab-b3ee-4584-9f92-860b4c2369f3",
    "Cliff Nielsen",
    crate::card::CardRules::unsupported(),
);

// WTH 25 — Serra's Blessing
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SERRA_S_BLESSING: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Serra's Blessing",
    "2794cca9-3df0-4864-8a98-4de71a2bcf17",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// WTH 26 — Soul Shepherd
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOUL_SHEPHERD: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Soul Shepherd",
    "f45a39ba-5fbf-46c3-8dc7-3058ac6d24e8",
    "John Coulthart",
    crate::card::CardRules::unsupported(),
);

// WTH 27 — Southern Paladin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOUTHERN_PALADIN: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Southern Paladin",
    "2a3c94a1-8455-4521-a0d5-ee2982527b89",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// WTH 28 — Tariff
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TARIFF: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Tariff",
    "24333832-2a87-4810-9443-ec993468d103",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// WTH 29 — Volunteer Reserves
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VOLUNTEER_RESERVES: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Volunteer Reserves",
    "5344911f-25e8-45ce-87b9-607e42db0139",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// WTH 30 — Abduction
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ABDUCTION: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Abduction",
    "ac81264d-0e03-44ac-8ff5-049b9aaebcca",
    "Colin MacNeil",
    crate::card::CardRules::unsupported(),
);

// WTH 31 — Abjure
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ABJURE: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Abjure",
    "fbad9449-d09c-4fd0-b2ad-2aa3a29e03bf",
    "Ted Naifeh",
    crate::card::CardRules::unsupported(),
);

// WTH 32 — Ancestral Knowledge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANCESTRAL_KNOWLEDGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Ancestral Knowledge",
    "05b90d72-00ac-4423-8cdf-e1471c6cd0ae",
    "Colin MacNeil",
    crate::card::CardRules::unsupported(),
);

// WTH 33 — Apathy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static APATHY: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Apathy",
    "adf3a6fe-e234-4c3f-96fc-3eb5eb22c0b8",
    "Phil Foglio",
    crate::card::CardRules::unsupported(),
);

// WTH 34 — Argivian Restoration
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARGIVIAN_RESTORATION: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Argivian Restoration",
    "9f1a9d35-1b2a-44a2-9bbc-8529a7487905",
    "Roger Raupp",
    crate::card::CardRules::unsupported(),
);

// WTH 35 — Avizoa
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVIZOA: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Avizoa",
    "a993986c-e8f1-41b1-86e6-c72021c53b87",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// WTH 36 — Cloud Djinn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLOUD_DJINN: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Cloud Djinn",
    "c857a151-45fe-43af-a9be-a93d26f220f3",
    "Mike Dringenberg",
    crate::card::CardRules::unsupported(),
);

// WTH 37 — Disrupt
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DISRUPT: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Disrupt",
    "c6cc89b0-9acf-452b-ac1a-bc7e90eb32fc",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// WTH 38 — Ertai's Familiar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ERTAI_S_FAMILIAR: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Ertai's Familiar",
    "354c9de7-0cdf-4302-9d1a-ae17eca13053",
    "Kipling West",
    crate::card::CardRules::unsupported(),
);

// WTH 39 — Flux (reprint)
const FLUX_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::FLUX,
    "368b28e4-a367-4a38-866d-c3768bd9b7ad",
    "Richard Kane Ferguson",
);

// WTH 40 — Fog Elemental
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FOG_ELEMENTAL: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Fog Elemental",
    "28b454d0-7dc7-419f-aefa-f20f37444658",
    "Jon J Muth",
    crate::card::CardRules::unsupported(),
);

// WTH 41 — Mana Chains
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MANA_CHAINS: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Mana Chains",
    "77802038-0d86-4911-97ed-e6bd2ed55e23",
    "Bryan Talbot",
    crate::card::CardRules::unsupported(),
);

// WTH 42 — Manta Ray
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MANTA_RAY: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Manta Ray",
    "80f74884-9b82-419d-9e97-c947a6b7d09f",
    "Una Fricker",
    crate::card::CardRules::unsupported(),
);

// WTH 43 — Merfolk Traders
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MERFOLK_TRADERS: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Merfolk Traders",
    "ebacbf23-4b69-481c-aaf7-5de7b4a6db6f",
    "DiTerlizzi",
    crate::card::CardRules::unsupported(),
);

// WTH 44 — Noble Benefactor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NOBLE_BENEFACTOR: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Noble Benefactor",
    "bd221f30-1773-4e05-a40f-022a9306ef89",
    "DiTerlizzi",
    crate::card::CardRules::unsupported(),
);

// WTH 45 — Ophidian
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OPHIDIAN: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Ophidian",
    "0de0a010-76a7-460f-bb4e-a152c10c3bb7",
    "Cliff Nielsen",
    crate::card::CardRules::unsupported(),
);

// WTH 46 — Paradigm Shift
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PARADIGM_SHIFT: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Paradigm Shift",
    "e64a17a8-091d-4029-908e-31d6a050b479",
    "Cliff Nielsen",
    crate::card::CardRules::unsupported(),
);

// WTH 47 — Pendrell Mists
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PENDRELL_MISTS: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Pendrell Mists",
    "b902b972-3a93-4e4e-aa77-02ada81e6b95",
    "Andrew Robinson",
    crate::card::CardRules::unsupported(),
);

// WTH 48 — Phantom Warrior (reprint)
const PHANTOM_WARRIOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::PHANTOM_WARRIOR,
    "b414c9f8-ee46-4368-a8dc-0767c645a9c1",
    "John Matson",
);

// WTH 49 — Phantom Wings
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHANTOM_WINGS: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Phantom Wings",
    "a0db4c6c-aa51-487b-a591-78d93c67c775",
    "Una Fricker",
    crate::card::CardRules::unsupported(),
);

// WTH 50 — Psychic Vortex
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PSYCHIC_VORTEX: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Psychic Vortex",
    "3bc2a419-7122-4eeb-bb64-738a647cfd82",
    "Steve Luke",
    crate::card::CardRules::unsupported(),
);

// WTH 51 — Relearn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RELEARN: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Relearn",
    "902f8480-8ae7-4b5f-abdf-1bd46066049e",
    "Zina Saunders",
    crate::card::CardRules::unsupported(),
);

// WTH 52 — Sage Owl
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAGE_OWL: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Sage Owl",
    "3ee2d6a1-8b1e-47e5-9720-5683ac458250",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// WTH 53 — Teferi's Veil
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEFERI_S_VEIL: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Teferi's Veil",
    "cbf39b80-d972-4f79-902f-cc613c32e446",
    "Brom",
    crate::card::CardRules::unsupported(),
);

// WTH 54 — Timid Drake
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TIMID_DRAKE: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Timid Drake",
    "01bbdbd8-1517-4bfd-926b-465a32724082",
    "Mike Dringenberg",
    crate::card::CardRules::unsupported(),
);

// WTH 55 — Tolarian Drake
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOLARIAN_DRAKE: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Tolarian Drake",
    "e04bba8a-48ee-4981-adc2-4f82c0f2c1bd",
    "Mark Harrison",
    crate::card::CardRules::unsupported(),
);

// WTH 56 — Tolarian Entrancer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOLARIAN_ENTRANCER: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Tolarian Entrancer",
    "c29dd04a-b3aa-48b6-beef-3314344b84a6",
    "Bryan Talbot",
    crate::card::CardRules::unsupported(),
);

// WTH 57 — Tolarian Serpent
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOLARIAN_SERPENT: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Tolarian Serpent",
    "9236a857-c4ca-4de2-a4a2-e0914d16b54b",
    "Stuart Griffin",
    crate::card::CardRules::unsupported(),
);

// WTH 58 — Vodalian Illusionist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VODALIAN_ILLUSIONIST: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Vodalian Illusionist",
    "9ce0e28b-9fd6-4763-8d6b-952b530358ab",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// WTH 59 — Abyssal Gatekeeper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ABYSSAL_GATEKEEPER: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Abyssal Gatekeeper",
    "1734df5a-7d3a-46c7-a0ad-adbbd1be958f",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// WTH 60 — Agonizing Memories
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AGONIZING_MEMORIES: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Agonizing Memories",
    "be277367-a58e-429e-af1b-58163becf861",
    "Mike Dringenberg",
    crate::card::CardRules::unsupported(),
);

// WTH 61 — Barrow Ghoul
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARROW_GHOUL: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Barrow Ghoul",
    "f7055007-83dd-40fe-b2a1-4b3132f636db",
    "Bryan Talbot",
    crate::card::CardRules::unsupported(),
);

// WTH 62 — Bone Dancer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BONE_DANCER: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Bone Dancer",
    "207bb4cd-4525-47e0-b412-0d0e29717d44",
    "Scott Kirschner",
    crate::card::CardRules::unsupported(),
);

// WTH 63 — Buried Alive
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BURIED_ALIVE: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Buried Alive",
    "56b92eb5-72b0-46b4-8b16-8a7a7ac80f56",
    "Brian Horton",
    crate::card::CardRules::unsupported(),
);

// WTH 64 — Circling Vultures
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CIRCLING_VULTURES: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Circling Vultures",
    "8dae8e49-c2b6-4965-9249-49f93449d271",
    "Una Fricker",
    crate::card::CardRules::unsupported(),
);

// WTH 65 — Coils of the Medusa
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COILS_OF_THE_MEDUSA: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Coils of the Medusa",
    "502bfb38-4a37-4053-af20-d5606ffc67c8",
    "Darbury Stenderu",
    crate::card::CardRules::unsupported(),
);

// WTH 66 — Doomsday
pub(in crate::card::sets) static DOOMSDAY: CardRecord = CardRecord::new(
    CardSet::Weatherlight,
    "Doomsday",
    "5b3c6d87-9383-450b-bba5-33435b6b0d08",
    "Adrian Smith",
    // A five-card library you built yourself, and half your life for it. The
    // deck that plays it is not trying to survive the exile -- it is trying
    // to draw the five cards it just stacked and win on the spot.
    CardRules::new_sorcery(mana_cost!("{B}{B}{B}")).with_ability(AbilityDef::spell(
        "Search your library and graveyard for five cards and exile the rest. Put the chosen cards on top of your library in any order. You lose half your life, rounded up.",
        // The search and the life are one clause resolving in order, and the order
        // matters: the five cards are chosen while the library still exists.
        EffectDef::Sequence(&[
            EffectDef::SearchZonesAndExileRest {
                player: EffectRecipientDef::Controller,
                zones: &[ZoneKind::Library, ZoneKind::Graveyard],
                count: 5,
            },
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Controller,
                // Half the life you have, rounded up: at twenty that is ten, and the deck
                // casting this intends to win before losing the other ten.
                amount: ValueDef::Halved(&HalvedValueDef::new(ValueDef::LifeTotal(PlayerRelation::You), RoundingDef::Up)),
            },
        ]),
    )),
);

// WTH 67 — Fatal Blow
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FATAL_BLOW: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Fatal Blow",
    "044dc7c2-6198-4526-b79a-f3d8ee7a157a",
    "George Pratt",
    crate::card::CardRules::unsupported(),
);

// WTH 68 — Festering Evil
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FESTERING_EVIL: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Festering Evil",
    "2d688bda-fee2-496d-9793-794c2568b54e",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// WTH 69 — Fledgling Djinn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLEDGLING_DJINN: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Fledgling Djinn",
    "1b0fdf2a-d6d2-42da-8f41-0f67dd0bf4d2",
    "Thomas Gianni",
    crate::card::CardRules::unsupported(),
);

// WTH 70 — Gallowbraid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GALLOWBRAID: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Gallowbraid",
    "8df86192-6374-42ac-94bc-95e2e8284bd6",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// WTH 71 — Haunting Misery
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HAUNTING_MISERY: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Haunting Misery",
    "939b83ba-8ba8-4b98-8a13-a037ba7805e9",
    "Gary Leach",
    crate::card::CardRules::unsupported(),
);

// WTH 72 — Hidden Horror
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HIDDEN_HORROR: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Hidden Horror",
    "885dc4c5-2ade-4497-b579-0307c67ac783",
    "Clint Langley",
    crate::card::CardRules::unsupported(),
);

// WTH 73 — Infernal Tribute
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INFERNAL_TRIBUTE: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Infernal Tribute",
    "569739b2-f212-4cc9-84db-1be17b3f90fb",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// WTH 74 — Mischievous Poltergeist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISCHIEVOUS_POLTERGEIST: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Mischievous Poltergeist",
    "054254ee-29cf-48d7-afbf-cb6de83e513e",
    "DiTerlizzi",
    crate::card::CardRules::unsupported(),
);

// WTH 75 — Morinfen
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MORINFEN: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Morinfen",
    "b5006ad3-16ca-4be3-8d56-d4fe4e9e0a44",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// WTH 76 — Necratog
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NECRATOG: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Necratog",
    "fb19c519-c09a-44a0-8d4b-ab6c15dabdef",
    "Bryan Talbot",
    crate::card::CardRules::unsupported(),
);

// WTH 77 — Odylic Wraith
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ODYLIC_WRAITH: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Odylic Wraith",
    "3a3b7cd1-051c-43a8-b5f0-72a9d704efbc",
    "Ian Miller",
    crate::card::CardRules::unsupported(),
);

// WTH 78 — Razortooth Rats
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAZORTOOTH_RATS: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Razortooth Rats",
    "ae869780-27e8-4a6d-9ac6-cdab617725e2",
    "Brian Horton",
    crate::card::CardRules::unsupported(),
);

// WTH 79 — Shadow Rider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHADOW_RIDER: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Shadow Rider",
    "5bfdec24-e689-4cca-a546-a8f5d0929f8d",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// WTH 80 — Shattered Crypt
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHATTERED_CRYPT: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Shattered Crypt",
    "117df45d-4500-459b-96b5-ca41952580c1",
    "Gary Leach",
    crate::card::CardRules::unsupported(),
);

// WTH 81 — Spinning Darkness
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPINNING_DARKNESS: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Spinning Darkness",
    "58e64a8e-84b1-416c-9fa7-8b10130dc9e9",
    "John Coulthart",
    crate::card::CardRules::unsupported(),
);

// WTH 82 — Strands of Night
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STRANDS_OF_NIGHT: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Strands of Night",
    "872ef62f-e119-470b-b212-9beb48469095",
    "Patrick Kochakji",
    crate::card::CardRules::unsupported(),
);

// WTH 83 — Tendrils of Despair
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TENDRILS_OF_DESPAIR: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Tendrils of Despair",
    "b5d73ddb-bd3c-4625-9f75-ba2079553915",
    "John Coulthart",
    crate::card::CardRules::unsupported(),
);

// WTH 84 — Urborg Justice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static URBORG_JUSTICE: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Urborg Justice",
    "39f322ff-0b04-41ce-90cd-9896f941e703",
    "Gary Leach",
    crate::card::CardRules::unsupported(),
);

// WTH 85 — Urborg Stalker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static URBORG_STALKER: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Urborg Stalker",
    "2d33e3d5-c608-4ba8-8614-0b9d0385af64",
    "Cliff Nielsen",
    crate::card::CardRules::unsupported(),
);

// WTH 86 — Wave of Terror
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WAVE_OF_TERROR: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Wave of Terror",
    "d40ab3e7-9abb-4acc-9932-de03b533722f",
    "Adrian Smith",
    crate::card::CardRules::unsupported(),
);

// WTH 87 — Zombie Scavengers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ZOMBIE_SCAVENGERS: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Zombie Scavengers",
    "2ec786b1-6097-4e97-99b0-571d6e3e73e7",
    "Patrick Kochakji",
    crate::card::CardRules::unsupported(),
);

// WTH 88 — Aether Flash
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AETHER_FLASH: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Aether Flash",
    "28f6642d-393d-49a5-8c49-c1f62524ea20",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// WTH 89 — Betrothed of Fire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BETROTHED_OF_FIRE: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Betrothed of Fire",
    "5e517aa4-d8ba-4a49-bf9f-172bf029fa52",
    "Clint Langley",
    crate::card::CardRules::unsupported(),
);

// WTH 90 — Bloodrock Cyclops
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLOODROCK_CYCLOPS: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Bloodrock Cyclops",
    "5c642fd9-38f7-4029-ab93-e1dc5636c1ad",
    "Tom Wänerstrand",
    crate::card::CardRules::unsupported(),
);

// WTH 91 — Bogardan Firefiend
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOGARDAN_FIREFIEND: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Bogardan Firefiend",
    "80ff9650-d25f-4c6b-b96e-794b50af3f14",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// WTH 92 — Boiling Blood
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOILING_BLOOD: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Boiling Blood",
    "3fcb85b6-ab5a-40db-aaae-555315f32877",
    "Cliff Nielsen",
    crate::card::CardRules::unsupported(),
);

// WTH 93 — Cinder Giant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CINDER_GIANT: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Cinder Giant",
    "de97c939-2c44-4c43-9d66-1087bcee692b",
    "Rogério Vilela",
    crate::card::CardRules::unsupported(),
);

// WTH 94 — Cinder Wall
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CINDER_WALL: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Cinder Wall",
    "6c1e429c-2e66-4363-b50a-b12b72efa060",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// WTH 95 — Cone of Flame
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CONE_OF_FLAME: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Cone of Flame",
    "5713f17a-9a57-41f8-b492-ced876e1a37f",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// WTH 96 — Desperate Gambit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DESPERATE_GAMBIT: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Desperate Gambit",
    "f4245160-274e-4c39-9bcd-c64e9a44dfdb",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// WTH 97 — Dwarven Berserker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DWARVEN_BERSERKER: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Dwarven Berserker",
    "7bc734e9-fb09-4094-94b6-76c0458649e9",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// WTH 98 — Dwarven Thaumaturgist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DWARVEN_THAUMATURGIST: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Dwarven Thaumaturgist",
    "8e68aa29-9f38-48a2-b00a-39aef9d91f6d",
    "Kipling West",
    crate::card::CardRules::unsupported(),
);

// WTH 99 — Fervor
pub(in crate::card::sets) static FERVOR: CardRecord = CardRecord::new(
    CardSet::Weatherlight,
    "Fervor",
    "b4df70ea-2b6b-4e25-a564-655989ef16fa",
    "Franz Vohwinkel",
    CardRules::new_enchantment(mana_cost!("{2}{R}")).with_ability(AbilityDef::static_ability(
        "Creatures you control have haste.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::add_ability(&abilities::haste()),
        },
    )),
);

// WTH 100 — Fire Whip
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FIRE_WHIP: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Fire Whip",
    "3ee194b4-f18f-4ebd-b42f-c7dfef42f22e",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// WTH 101 — Firestorm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FIRESTORM: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Firestorm",
    "e674aa8a-668a-4345-95ee-73a0b87bbcb1",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// WTH 102 — Fit of Rage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FIT_OF_RAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Fit of Rage",
    "09e7b9ec-90cf-4d23-af5e-48394398ff06",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// WTH 103 — Goblin Bomb
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_BOMB: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Goblin Bomb",
    "97e8a436-9fd0-409f-a020-0f9f41602d50",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// WTH 104 — Goblin Grenadiers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_GRENADIERS: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Goblin Grenadiers",
    "5a73db23-727f-4d63-97d7-2ca542276722",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// WTH 105 — Goblin Vandal
pub(in crate::card::sets) static GOBLIN_VANDAL: CardRecord = CardRecord::new(
    CardSet::Weatherlight,
    "Goblin Vandal",
    "b7ad3b81-f706-4b33-b1ec-7600182a5232",
    "Franz Vohwinkel",
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin", "Rogue"], 1, 1).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks and isn't blocked, you may pay {R}. If you do, destroy target artifact defending player controls and this creature assigns no combat damage this turn.",
            TriggerEventDef::AttacksAndIsNotBlocked {
                attacker: ObjectPredicateDef::Source,
            },
            // The artifact has to belong to the player being attacked, which in a
            // two-player game is the only opponent there is.
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Artifact),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::PayOr(PayOrDef::optional(
                EffectPaymentDef::mana(
                    PlayerSetDef::Related(PlayerRelation::You),
                    mana_cost!("{R}"),
                ),
                // Paying trades the hit for the artifact: the Vandal connects, and then
                // deals nothing because it spent the swing breaking something instead.
                &EffectDef::Sequence(&[
                    EffectDef::Destroy {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        can_regenerate: true,
                        then: None,
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::Rule(AppliedRuleDef::AssignsNoCombatDamage),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            )),
        ),
    ),
);

// WTH 106 — Heart of Bogardan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEART_OF_BOGARDAN: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Heart of Bogardan",
    "4e30d025-1df9-4a08-b686-037e9cbf23a6",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// WTH 107 — Heat Stroke
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEAT_STROKE: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Heat Stroke",
    "1baf2a6c-57ec-4b38-8b08-4b3f800dbe99",
    "Andrew Robinson",
    crate::card::CardRules::unsupported(),
);

// WTH 108 — Hurloon Shaman
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HURLOON_SHAMAN: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Hurloon Shaman",
    "70a359c9-1889-426d-acaf-074cfd9f274d",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// WTH 109 — Lava Hounds
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAVA_HOUNDS: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Lava Hounds",
    "896dcaf0-3e52-4189-990f-cabab40ffbd1",
    "Steve White",
    crate::card::CardRules::unsupported(),
);

// WTH 110 — Lava Storm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAVA_STORM: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Lava Storm",
    "61fcd58e-e5e2-45f4-9edd-300a871ae5f5",
    "Scott Kirschner",
    crate::card::CardRules::unsupported(),
);

// WTH 111 — Maraxus of Keld
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MARAXUS_OF_KELD: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Maraxus of Keld",
    "59329155-a423-4e8d-a7d4-c99555ff5ed1",
    "Adrian Smith",
    crate::card::CardRules::unsupported(),
);

// WTH 112 — Orcish Settlers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORCISH_SETTLERS: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Orcish Settlers",
    "d54764f6-6f65-405c-ba30-1e485ce3fe21",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// WTH 113 — Roc Hatchling
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROC_HATCHLING: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Roc Hatchling",
    "25857884-6bb7-4a8e-a08b-fa610af8a5c3",
    "Una Fricker",
    crate::card::CardRules::unsupported(),
);

// WTH 114 — Sawtooth Ogre
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAWTOOTH_OGRE: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Sawtooth Ogre",
    "4a237580-f7f6-4d6b-a342-0d11fc0b5a59",
    "Brom",
    crate::card::CardRules::unsupported(),
);

// WTH 115 — Thunderbolt
pub(in crate::card::sets) static THUNDERBOLT: CardRecord = CardRecord::new(
    CardSet::Weatherlight,
    "Thunderbolt",
    "a0a4b641-2eb3-482b-91a1-236ebe2a7a41",
    "Dylan Martens",
    CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Thunderbolt deals 3 damage to target player or planeswalker.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
                )],
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(3),
                },
            ),
            AbilityDef::spell_with_targets(
                "Thunderbolt deals 4 damage to target creature with flying.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                    ]),
                )],
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(4),
                },
            ),
        ],
    )),
);

// WTH 116 — Thundermare (reprint)
const THUNDERMARE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::THUNDERMARE,
    "e936e5cb-0a8e-4348-afea-e5f96b19fe23",
    "Bob Eggleton",
);

// WTH 117 — Aboroth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ABOROTH: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Aboroth",
    "8c72ac67-e4fb-49a1-b1e5-cd2e414bec28",
    "Brom",
    crate::card::CardRules::unsupported(),
);

// WTH 118 — Arctic Wolves
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARCTIC_WOLVES: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Arctic Wolves",
    "b5fb56a2-5138-4c31-aa4b-0824a1a24573",
    "Steve White",
    crate::card::CardRules::unsupported(),
);

// WTH 119 — Barishi
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARISHI: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Barishi",
    "f263eb80-f8f2-4b32-8e8b-a297de9f3666",
    "Ted Naifeh",
    crate::card::CardRules::unsupported(),
);

// WTH 120 — Blossoming Wreath
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLOSSOMING_WREATH: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Blossoming Wreath",
    "2f944ad9-c9ce-47b2-80fa-d0f7fcf0fd5d",
    "Brian Durfee",
    crate::card::CardRules::unsupported(),
);

// WTH 121 — Briar Shield
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRIAR_SHIELD: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Briar Shield",
    "68100ac2-9677-4eb5-93dc-54e49b15985d",
    "Scott Kirschner",
    crate::card::CardRules::unsupported(),
);

// WTH 122 — Call of the Wild
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CALL_OF_THE_WILD: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Call of the Wild",
    "a742bc7c-7f0d-4dff-b229-f16d54fe1347",
    "Brom",
    crate::card::CardRules::unsupported(),
);

// WTH 123 — Choking Vines
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHOKING_VINES: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Choking Vines",
    "6cc4a7ee-f6f0-454a-9074-5988fdee1f34",
    "Ted Naifeh",
    crate::card::CardRules::unsupported(),
);

// WTH 124 — Dense Foliage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DENSE_FOLIAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Dense Foliage",
    "c60a2035-59cb-426e-b2ae-45d8d6ce0bb8",
    "Alan Rabinowitz",
    crate::card::CardRules::unsupported(),
);

// WTH 125 — Downdraft
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DOWNDRAFT: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Downdraft",
    "ab4ced80-926a-4e4d-8ebd-d4fe7374a6ad",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// WTH 126 — Fallow Wurm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FALLOW_WURM: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Fallow Wurm",
    "1ba02b6f-6010-47a4-8670-406391a52a68",
    "Stephen L. Walsh",
    crate::card::CardRules::unsupported(),
);

// WTH 127 — Familiar Ground
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FAMILIAR_GROUND: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Familiar Ground",
    "f993f517-999f-4ee6-8ffb-946bffdcf7fe",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// WTH 128 — Fungus Elemental
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FUNGUS_ELEMENTAL: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Fungus Elemental",
    "4336bfd1-27a4-414d-b6fe-f186a0563dc0",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// WTH 129 — Gaea's Blessing
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GAEA_S_BLESSING: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Gaea's Blessing",
    "ee83d511-57e0-40fb-a4db-62f6c2c39888",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// WTH 130 — Harvest Wurm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HARVEST_WURM: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Harvest Wurm",
    "9d21139d-edfc-4140-aa43-d4165331d7f3",
    "Stephen L. Walsh",
    crate::card::CardRules::unsupported(),
);

// WTH 131 — Liege of the Hollows
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIEGE_OF_THE_HOLLOWS: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Liege of the Hollows",
    "dff4512b-8244-4e38-bffb-0062a97d9531",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// WTH 132 — Llanowar Behemoth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LLANOWAR_BEHEMOTH: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Llanowar Behemoth",
    "3d5d9bd0-7ce9-4a1e-a8b2-5c1dbb014917",
    "Hannibal King",
    crate::card::CardRules::unsupported(),
);

// WTH 133 — Llanowar Druid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LLANOWAR_DRUID: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Llanowar Druid",
    "ffad279c-762a-42cf-ac20-f4e48734c194",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// WTH 134 — Llanowar Sentinel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LLANOWAR_SENTINEL: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Llanowar Sentinel",
    "6f37ea4b-66e2-4ad5-ae7f-d02fd59131bd",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// WTH 135 — Mwonvuli Ooze
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MWONVULI_OOZE: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Mwonvuli Ooze",
    "aa9c6f65-93a1-4913-87e7-a17ebfcc7780",
    "Zina Saunders",
    crate::card::CardRules::unsupported(),
);

// WTH 136 — Nature's Kiss
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NATURE_S_KISS: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Nature's Kiss",
    "64b09c44-d463-45a9-9fa2-89407c21200b",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// WTH 137 — Nature's Resurgence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NATURE_S_RESURGENCE: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Nature's Resurgence",
    "2df9fb85-f7fa-4617-87bd-4d457c830f46",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// WTH 138 — Redwood Treefolk (reprint)
const REDWOOD_TREEFOLK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::REDWOOD_TREEFOLK,
    "0274e162-33e4-4604-a6ea-51fc1a5c6a04",
    "Phil Foglio",
);

// WTH 139 — Rogue Elephant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROGUE_ELEPHANT: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Rogue Elephant",
    "1b622b2f-84ad-4203-97fa-35af09e1c370",
    "Steve White",
    crate::card::CardRules::unsupported(),
);

// WTH 140 — Striped Bears
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STRIPED_BEARS: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Striped Bears",
    "0bf54365-56ae-485d-b931-784a4cf9d8f2",
    "Una Fricker",
    crate::card::CardRules::unsupported(),
);

// WTH 141 — Sylvan Hierophant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SYLVAN_HIEROPHANT: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Sylvan Hierophant",
    "432a6908-0ee3-45c5-9089-b7f8cf1184bb",
    "Brian Durfee",
    crate::card::CardRules::unsupported(),
);

// WTH 142 — Tranquil Grove
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRANQUIL_GROVE: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Tranquil Grove",
    "c4a145f2-b59d-4728-922c-9bc228451432",
    "Dylan Martens",
    crate::card::CardRules::unsupported(),
);

// WTH 143 — Uktabi Efreet
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UKTABI_EFREET: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Uktabi Efreet",
    "3678a224-d314-4108-8a39-de0c1b635b5c",
    "Alan Rabinowitz",
    crate::card::CardRules::unsupported(),
);

// WTH 144 — Veteran Explorer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VETERAN_EXPLORER: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Veteran Explorer",
    "bdac36f2-99ce-4d48-90fa-aa7439778ffc",
    "David A. Cherry",
    crate::card::CardRules::unsupported(),
);

// WTH 145 — Vitalize
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VITALIZE: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Vitalize",
    "d6ee4997-4b1a-4e03-88ac-63b451bb7b38",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// WTH 146 — Bubble Matrix
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BUBBLE_MATRIX: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Bubble Matrix",
    "0ca9c239-84ff-4527-aa23-bdb11856744c",
    "Brom",
    crate::card::CardRules::unsupported(),
);

// WTH 147 — Bösium Strip
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOSIUM_STRIP: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Bösium Strip",
    "3884bede-df28-42e8-9ac9-ae03118b1985",
    "Steve Luke",
    crate::card::CardRules::unsupported(),
);

// WTH 148 — Chimeric Sphere
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHIMERIC_SPHERE: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Chimeric Sphere",
    "cc96857c-b38e-4614-9838-cacd3700e3ee",
    "Colin MacNeil",
    crate::card::CardRules::unsupported(),
);

// WTH 149 — Dingus Staff
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DINGUS_STAFF: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Dingus Staff",
    "065b4358-5dee-4f13-bff9-8254bdb92069",
    "Richard Kane Ferguson",
    crate::card::CardRules::unsupported(),
);

// WTH 150 — Jabari's Banner
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JABARI_S_BANNER: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Jabari's Banner",
    "3d51a496-1ca6-4286-bdbe-990d43196a25",
    "Mark Harrison",
    crate::card::CardRules::unsupported(),
);

// WTH 151 — Jangling Automaton
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JANGLING_AUTOMATON: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Jangling Automaton",
    "2e2a427b-9869-4059-aeeb-d9b97b324e4e",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// WTH 152 — Mana Web
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MANA_WEB: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Mana Web",
    "2c72ec90-dacc-496f-a7f5-f18bfce5eb3e",
    "Hannibal King",
    crate::card::CardRules::unsupported(),
);

// WTH 153 — Mind Stone
pub(in crate::card::sets) static MIND_STONE: CardRecord = CardRecord::new(
    CardSet::Weatherlight,
    "Mind Stone",
    "162e81d3-6cd4-4cb8-8ed8-cfbd8d34ca71",
    "Adam Rex",
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {C}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
        ),
        AbilityDef::activated(
            "{1}, {T}, Sacrifice this artifact: Draw a card.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificeSource,
            ],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// WTH 154 — Null Rod
pub(in crate::card::sets) static NULL_ROD: CardRecord = CardRecord::new(
    CardSet::Weatherlight,
    "Null Rod",
    "bc45f2cb-c256-4a0f-879a-c7db5b1a0b94",
    "Anson Maddocks",
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(AbilityDef::static_ability(
        "Activated abilities of artifacts can't be activated.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Artifact),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotActivateAbilities),
        },
    )),
);

// WTH 155 — Phyrexian Furnace
pub(in crate::card::sets) static PHYREXIAN_FURNACE: CardRecord = CardRecord::new(
    CardSet::Weatherlight,
    "Phyrexian Furnace",
    "e98bca31-8c05-430b-b5d7-331bdc55710a",
    "George Pratt",
    // The tap mode eats a graveyard from the bottom, one card a turn; the
    // sacrifice mode answers the one card that actually mattered.
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}: Exile the bottom card of target player's graveyard.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::objects(ObjectSetDef::BottomOfGraveyard(
                    PlayerRefDef::Target(TargetIndex::PRIMARY),
                )),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
            },
        ),
        AbilityDef::activated_with_targets(
            "{1}, Sacrifice this artifact: Exile target card from a graveyard. Draw a card.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::SacrificeSource,
            ],
            // Any card in any graveyard, which is what the sacrifice mode reaches. The
            // tap mode needs no target beyond the player, because a graveyard has only
            // one bottom card.
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Sequence(
                &const {
                    [
                        EffectDef::MoveToZone {
                            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            zone: ZoneKind::Exile,
                            placement: ZonePlacement::Top,
                        },
                        EffectDef::DrawCards {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(1),
                        },
                    ]
                },
            ),
        ),
    ]),
);

// WTH 156 — Serrated Biskelion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SERRATED_BISKELION: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Serrated Biskelion",
    "c449126c-ac01-4a90-b967-8c3ad112091b",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// WTH 157 — Steel Golem
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STEEL_GOLEM: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Steel Golem",
    "9aa927e0-5a65-4ac1-8eca-c000bb8080e7",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// WTH 158 — Straw Golem
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STRAW_GOLEM: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Straw Golem",
    "43d62479-92ac-43e2-a3d3-b41dfe0fbb20",
    "Bryan Talbot",
    crate::card::CardRules::unsupported(),
);

// WTH 159 — Thran Forge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THRAN_FORGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Thran Forge",
    "b9c9691b-bee8-4251-8275-5f6ba14a8ecd",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// WTH 160 — Thran Tome
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THRAN_TOME: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Thran Tome",
    "63db7360-fe6e-430f-bfee-a2f80bcb6fec",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// WTH 161 — Touchstone
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOUCHSTONE: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Touchstone",
    "923afe8a-e82c-4b93-bb42-8f5073acae13",
    "George Pratt",
    crate::card::CardRules::unsupported(),
);

// WTH 162 — Well of Knowledge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WELL_OF_KNOWLEDGE: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Well of Knowledge",
    "5184b967-f474-4c9b-9a20-65ddb0d6e4f8",
    "D. Alexander Gregory",
    crate::card::CardRules::unsupported(),
);

// WTH 163 — Xanthic Statue
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static XANTHIC_STATUE: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Xanthic Statue",
    "8becb285-cd91-4de0-af59-ddaa7d8c5366",
    "Hannibal King",
    crate::card::CardRules::unsupported(),
);

// WTH 164 — Gemstone Mine
pub(in crate::card::sets) static GEMSTONE_MINE: CardRecord = CardRecord::new(
    CardSet::Weatherlight,
    "Gemstone Mine",
    "09507f7f-c58f-4f57-b878-b39811a5b619",
    "Brom",
    // Three activations of perfect mana, and then nothing: the deck that
    // plays four of these is buying the first three turns, not the tenth.
    CardRules::new_land(&[]).with_abilities(&[
        AbilityDef::as_enters(
            "This land enters with three mining counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::named("mining"),
                    amount: 3,
                },
            ),
        ),
        AbilityDef::activated_mana(
            "{T}, Remove a mining counter from this land: Add one mana of any color. If there are no mining counters on this land, sacrifice it.",
            &[
                AbilityCostDef::TapSource,
                AbilityCostDef::RemoveCountersFromSource {
                    kind: CounterKind::named("mining"),
                    amount: 1,
                },
            ],
            EffectDef::AddMana(
                AddManaEffectDef::any_color().sacrificing_source_when_out_of(CounterKind::named("mining")),
            ),
        ),
    ]),
);

// WTH 165 — Lotus Vale
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LOTUS_VALE: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Lotus Vale",
    "2e5cd12a-2a07-44a8-8eac-de00d26fe9e3",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// WTH 166 — Scorched Ruins
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCORCHED_RUINS: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Scorched Ruins",
    "75a4e843-937c-47fb-8768-0f42c5cb4e4f",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// WTH 167 — Winding Canyons
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WINDING_CANYONS: CardRecord = CardRecord::new(
    crate::card::CardSet::Weatherlight,
    "Winding Canyons",
    "f26672a8-f4ff-4c64-bb3e-f5072bbc9e3e",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ABEYANCE,
    &ALMS,
    &ANGELIC_RENEWAL,
    &ARGIVIAN_FIND,
    &AURA_OF_SILENCE,
    &BENALISH_INFANTRY,
    &BENALISH_KNIGHT,
    &BENALISH_MISSIONARY,
    &DEBT_OF_LOYALTY,
    &DUSKRIDER_FALCON,
    &EMPYRIAL_ARMOR,
    &FORIYSIAN_BRIGADE,
    &GERRARD_S_WISDOM,
    &GUIDED_STRIKE,
    &HEAVY_BALLISTA,
    &INNER_SANCTUM,
    &KITHKIN_ARMOR,
    &MASTER_OF_ARMS,
    &MISTMOON_GRIFFIN,
    &PEACEKEEPER,
    &REVERED_UNICORN,
    &SERENITY,
    &SERRA_S_BLESSING,
    &SOUL_SHEPHERD,
    &SOUTHERN_PALADIN,
    &TARIFF,
    &VOLUNTEER_RESERVES,
    &ABDUCTION,
    &ABJURE,
    &ANCESTRAL_KNOWLEDGE,
    &APATHY,
    &ARGIVIAN_RESTORATION,
    &AVIZOA,
    &CLOUD_DJINN,
    &DISRUPT,
    &ERTAI_S_FAMILIAR,
    &FOG_ELEMENTAL,
    &MANA_CHAINS,
    &MANTA_RAY,
    &MERFOLK_TRADERS,
    &NOBLE_BENEFACTOR,
    &OPHIDIAN,
    &PARADIGM_SHIFT,
    &PENDRELL_MISTS,
    &PHANTOM_WINGS,
    &PSYCHIC_VORTEX,
    &RELEARN,
    &SAGE_OWL,
    &TEFERI_S_VEIL,
    &TIMID_DRAKE,
    &TOLARIAN_DRAKE,
    &TOLARIAN_ENTRANCER,
    &TOLARIAN_SERPENT,
    &VODALIAN_ILLUSIONIST,
    &ABYSSAL_GATEKEEPER,
    &AGONIZING_MEMORIES,
    &BARROW_GHOUL,
    &BONE_DANCER,
    &BURIED_ALIVE,
    &CIRCLING_VULTURES,
    &COILS_OF_THE_MEDUSA,
    &DOOMSDAY,
    &FATAL_BLOW,
    &FESTERING_EVIL,
    &FLEDGLING_DJINN,
    &GALLOWBRAID,
    &HAUNTING_MISERY,
    &HIDDEN_HORROR,
    &INFERNAL_TRIBUTE,
    &MISCHIEVOUS_POLTERGEIST,
    &MORINFEN,
    &NECRATOG,
    &ODYLIC_WRAITH,
    &RAZORTOOTH_RATS,
    &SHADOW_RIDER,
    &SHATTERED_CRYPT,
    &SPINNING_DARKNESS,
    &STRANDS_OF_NIGHT,
    &TENDRILS_OF_DESPAIR,
    &URBORG_JUSTICE,
    &URBORG_STALKER,
    &WAVE_OF_TERROR,
    &ZOMBIE_SCAVENGERS,
    &AETHER_FLASH,
    &BETROTHED_OF_FIRE,
    &BLOODROCK_CYCLOPS,
    &BOGARDAN_FIREFIEND,
    &BOILING_BLOOD,
    &CINDER_GIANT,
    &CINDER_WALL,
    &CONE_OF_FLAME,
    &DESPERATE_GAMBIT,
    &DWARVEN_BERSERKER,
    &DWARVEN_THAUMATURGIST,
    &FERVOR,
    &FIRE_WHIP,
    &FIRESTORM,
    &FIT_OF_RAGE,
    &GOBLIN_BOMB,
    &GOBLIN_GRENADIERS,
    &GOBLIN_VANDAL,
    &HEART_OF_BOGARDAN,
    &HEAT_STROKE,
    &HURLOON_SHAMAN,
    &LAVA_HOUNDS,
    &LAVA_STORM,
    &MARAXUS_OF_KELD,
    &ORCISH_SETTLERS,
    &ROC_HATCHLING,
    &SAWTOOTH_OGRE,
    &THUNDERBOLT,
    &ABOROTH,
    &ARCTIC_WOLVES,
    &BARISHI,
    &BLOSSOMING_WREATH,
    &BRIAR_SHIELD,
    &CALL_OF_THE_WILD,
    &CHOKING_VINES,
    &DENSE_FOLIAGE,
    &DOWNDRAFT,
    &FALLOW_WURM,
    &FAMILIAR_GROUND,
    &FUNGUS_ELEMENTAL,
    &GAEA_S_BLESSING,
    &HARVEST_WURM,
    &LIEGE_OF_THE_HOLLOWS,
    &LLANOWAR_BEHEMOTH,
    &LLANOWAR_DRUID,
    &LLANOWAR_SENTINEL,
    &MWONVULI_OOZE,
    &NATURE_S_KISS,
    &NATURE_S_RESURGENCE,
    &ROGUE_ELEPHANT,
    &STRIPED_BEARS,
    &SYLVAN_HIEROPHANT,
    &TRANQUIL_GROVE,
    &UKTABI_EFREET,
    &VETERAN_EXPLORER,
    &VITALIZE,
    &BUBBLE_MATRIX,
    &BOSIUM_STRIP,
    &CHIMERIC_SPHERE,
    &DINGUS_STAFF,
    &JABARI_S_BANNER,
    &JANGLING_AUTOMATON,
    &MANA_WEB,
    &MIND_STONE,
    &NULL_ROD,
    &PHYREXIAN_FURNACE,
    &SERRATED_BISKELION,
    &STEEL_GOLEM,
    &STRAW_GOLEM,
    &THRAN_FORGE,
    &THRAN_TOME,
    &TOUCHSTONE,
    &WELL_OF_KNOWLEDGE,
    &XANTHIC_STATUE,
    &GEMSTONE_MINE,
    &LOTUS_VALE,
    &SCORCHED_RUINS,
    &WINDING_CANYONS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    ALABASTER_DRAGON_REPRINT,
    ARDENT_MILITIA_REPRINT,
    FLUX_REPRINT,
    PHANTOM_WARRIOR_REPRINT,
    THUNDERMARE_REPRINT,
    REDWOOD_TREEFOLK_REPRINT,
];
