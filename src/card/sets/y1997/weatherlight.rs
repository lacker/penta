//! Weatherlight cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::sets::y2012::avacyn_restored as catalog_avr;
use crate::card::sets::y2012::magic_2013 as catalog_m13;
use crate::card::sets::y2013::magic_2014 as catalog_m14;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedEffectDef, AppliedRuleDef, BattlefieldEntryModificationDef, CardArt, CardRules, CardSet,
    CardType, CostModificationDef, CounterKind, EffectDef, EffectPaymentDef, EffectRecipientDef,
    HalvedValueDef, ManaColor, ObjectPredicateDef, ObjectSetDef, PayOrDef, PlayActionMatcherDef,
    PlayRestrictionDef, PlayerRefDef, PlayerRelation, PlayerSetDef, ReplacementEffectDef,
    ResolvedEffectDurationDef, RoundingDef, TriggerEventDef, ValueDef, ZoneKind, ZonePlacement,
};
use crate::{TargetIndex, mana_cost};

// WTH 1 — Abeyance
pub(in crate::card::sets) static ABEYANCE: CardRecord = CardRecord::new_with_legacy_id(
    2086,
    "Abeyance",
    CardArt::new("efb452f0-c019-4409-bfb1-600a97d58fdd", "Thomas Gianni"),
    CardSet::Weatherlight,
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

// WTH 2 — Alabaster Dragon
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ALABASTER_DRAGON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1edc6ec1-3b34-45e0-8573-39eba1d10efa"),
    "Alabaster Dragon",
    crate::card::CardArt::new("3a2fcc23-ac09-4ada-b194-424739c9c734", "Bob Eggleton"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 3 — Alms
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ALMS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("97382dd8-2754-4ca3-8ba8-d655acaf22ac"),
    "Alms",
    crate::card::CardArt::new("97382dd8-2754-4ca3-8ba8-d655acaf22ac", "Rogério Vilela"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 4 — Angelic Renewal
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ANGELIC_RENEWAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7dddde7d-8565-45a7-a1db-f2dea2a6a3ba"),
    "Angelic Renewal",
    crate::card::CardArt::new("7dddde7d-8565-45a7-a1db-f2dea2a6a3ba", "Rebecca Guay"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 5 — Ardent Militia
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ARDENT_MILITIA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("543f8c6a-bcf1-4400-82e5-83d36cb60464"),
    "Ardent Militia",
    crate::card::CardArt::new("bb212ca5-bbb5-4c83-9a7b-9d5ab451e032", "Zina Saunders"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 6 — Argivian Find
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ARGIVIAN_FIND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("89f23295-ad0a-4e2d-ae04-1a9c065e575d"),
    "Argivian Find",
    crate::card::CardArt::new("89f23295-ad0a-4e2d-ae04-1a9c065e575d", "Roger Raupp"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 7 — Aura of Silence
/// The tax names spells an opponent casts, so it never touches your own.
static OPPONENTS_ARTIFACTS_AND_ENCHANTMENTS: ObjectPredicateDef = ObjectPredicateDef::AnyOf(&[
    ObjectPredicateDef::HasType(CardType::Artifact),
    ObjectPredicateDef::HasType(CardType::Enchantment),
]);

pub(in crate::card::sets) static AURA_OF_SILENCE: CardRecord = CardRecord::new_with_legacy_id(
    2042,
    "Aura of Silence",
    CardArt::new(
        "57e6c366-b8c7-4f66-b8e1-82dc69c0081c",
        "D. Alexander Gregory",
    ),
    CardSet::Weatherlight,
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
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BENALISH_INFANTRY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e8472303-b8ee-402b-a9ea-49abe2e01152"),
    "Benalish Infantry",
    crate::card::CardArt::new("e8472303-b8ee-402b-a9ea-49abe2e01152", "Dan Frazier"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 9 — Benalish Knight
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BENALISH_KNIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c2c184bb-6c7d-4118-a111-ef27171cfee6"),
    "Benalish Knight",
    crate::card::CardArt::new("c2c184bb-6c7d-4118-a111-ef27171cfee6", "Zina Saunders"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 10 — Benalish Missionary
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BENALISH_MISSIONARY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e9ac1992-6212-4f05-af16-c892dfc40643"),
    "Benalish Missionary",
    crate::card::CardArt::new("e9ac1992-6212-4f05-af16-c892dfc40643", "Pete Venters"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 11 — Debt of Loyalty
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DEBT_OF_LOYALTY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d19ed33b-42d4-4a5d-a763-cfb43348769c"),
    "Debt of Loyalty",
    crate::card::CardArt::new("d19ed33b-42d4-4a5d-a763-cfb43348769c", "Pete Venters"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 12 — Duskrider Falcon
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DUSKRIDER_FALCON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bee3a23a-6ecf-439c-8637-e096fa8c1a80"),
    "Duskrider Falcon",
    crate::card::CardArt::new("bee3a23a-6ecf-439c-8637-e096fa8c1a80", "Cecil Fernando"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 13 — Empyrial Armor
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static EMPYRIAL_ARMOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5518a79f-bcae-417a-b01b-b6ff572be0be"),
    "Empyrial Armor",
    crate::card::CardArt::new(
        "5518a79f-bcae-417a-b01b-b6ff572be0be",
        "D. Alexander Gregory",
    ),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 14 — Foriysian Brigade
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FORIYSIAN_BRIGADE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0d11b6ef-3a24-4709-a62f-c5e062a6cee1"),
    "Foriysian Brigade",
    crate::card::CardArt::new("0d11b6ef-3a24-4709-a62f-c5e062a6cee1", "Kev Walker"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 15 — Gerrard's Wisdom
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GERRARD_S_WISDOM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f81defa5-edb4-4f1f-b13c-7cfb34511138"),
    "Gerrard's Wisdom",
    crate::card::CardArt::new("f81defa5-edb4-4f1f-b13c-7cfb34511138", "Heather Hudson"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 16 — Guided Strike
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GUIDED_STRIKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c6e8ec37-abe8-45a9-a1a0-6d4e37c74c45"),
    "Guided Strike",
    crate::card::CardArt::new("c6e8ec37-abe8-45a9-a1a0-6d4e37c74c45", "Gary Leach"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 17 — Heavy Ballista
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HEAVY_BALLISTA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bdfe3eed-e415-4b28-8b4d-e50a19235683"),
    "Heavy Ballista",
    crate::card::CardArt::new("bdfe3eed-e415-4b28-8b4d-e50a19235683", "Ron Spencer"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 18 — Inner Sanctum
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static INNER_SANCTUM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2298faae-370e-4b87-bf32-d20c2282a928"),
    "Inner Sanctum",
    crate::card::CardArt::new(
        "2298faae-370e-4b87-bf32-d20c2282a928",
        "D. Alexander Gregory",
    ),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 19 — Kithkin Armor
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static KITHKIN_ARMOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("395e7882-0429-46aa-8e38-be707067c588"),
    "Kithkin Armor",
    crate::card::CardArt::new("395e7882-0429-46aa-8e38-be707067c588", "Charles Gillespie"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 20 — Master of Arms
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MASTER_OF_ARMS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ac97ff43-c0b6-4f67-ad09-5ba8710c681a"),
    "Master of Arms",
    crate::card::CardArt::new("ac97ff43-c0b6-4f67-ad09-5ba8710c681a", "Dan Frazier"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 21 — Mistmoon Griffin
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MISTMOON_GRIFFIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8ec71a29-19db-4747-8276-7fd4d563d4df"),
    "Mistmoon Griffin",
    crate::card::CardArt::new("8ec71a29-19db-4747-8276-7fd4d563d4df", "David A. Cherry"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 22 — Peacekeeper
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PEACEKEEPER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("592a5683-5f2f-4933-9fc3-5f7773f72f93"),
    "Peacekeeper",
    crate::card::CardArt::new("592a5683-5f2f-4933-9fc3-5f7773f72f93", "Donato Giancola"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 23 — Revered Unicorn
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static REVERED_UNICORN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8c642dd2-1a3e-4b08-917e-6e8aed358b72"),
    "Revered Unicorn",
    crate::card::CardArt::new("8c642dd2-1a3e-4b08-917e-6e8aed358b72", "David A. Cherry"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 24 — Serenity
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SERENITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dca975ab-b3ee-4584-9f92-860b4c2369f3"),
    "Serenity",
    crate::card::CardArt::new("dca975ab-b3ee-4584-9f92-860b4c2369f3", "Cliff Nielsen"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 25 — Serra's Blessing
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SERRA_S_BLESSING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2794cca9-3df0-4864-8a98-4de71a2bcf17"),
    "Serra's Blessing",
    crate::card::CardArt::new("2794cca9-3df0-4864-8a98-4de71a2bcf17", "Rebecca Guay"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 26 — Soul Shepherd
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SOUL_SHEPHERD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f45a39ba-5fbf-46c3-8dc7-3058ac6d24e8"),
    "Soul Shepherd",
    crate::card::CardArt::new("f45a39ba-5fbf-46c3-8dc7-3058ac6d24e8", "John Coulthart"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 27 — Southern Paladin
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SOUTHERN_PALADIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2a3c94a1-8455-4521-a0d5-ee2982527b89"),
    "Southern Paladin",
    crate::card::CardArt::new("2a3c94a1-8455-4521-a0d5-ee2982527b89", "Douglas Shuler"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 28 — Tariff
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TARIFF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("24333832-2a87-4810-9443-ec993468d103"),
    "Tariff",
    crate::card::CardArt::new("24333832-2a87-4810-9443-ec993468d103", "Kev Walker"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 29 — Volunteer Reserves
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VOLUNTEER_RESERVES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5344911f-25e8-45ce-87b9-607e42db0139"),
    "Volunteer Reserves",
    crate::card::CardArt::new("5344911f-25e8-45ce-87b9-607e42db0139", "Kev Walker"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 30 — Abduction
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ABDUCTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ac81264d-0e03-44ac-8ff5-049b9aaebcca"),
    "Abduction",
    crate::card::CardArt::new("ac81264d-0e03-44ac-8ff5-049b9aaebcca", "Colin MacNeil"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 31 — Abjure
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ABJURE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fbad9449-d09c-4fd0-b2ad-2aa3a29e03bf"),
    "Abjure",
    crate::card::CardArt::new("fbad9449-d09c-4fd0-b2ad-2aa3a29e03bf", "Ted Naifeh"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 32 — Ancestral Knowledge
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ANCESTRAL_KNOWLEDGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("05b90d72-00ac-4423-8cdf-e1471c6cd0ae"),
    "Ancestral Knowledge",
    crate::card::CardArt::new("05b90d72-00ac-4423-8cdf-e1471c6cd0ae", "Colin MacNeil"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 33 — Apathy
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static APATHY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("adf3a6fe-e234-4c3f-96fc-3eb5eb22c0b8"),
    "Apathy",
    crate::card::CardArt::new("adf3a6fe-e234-4c3f-96fc-3eb5eb22c0b8", "Phil Foglio"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 34 — Argivian Restoration
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ARGIVIAN_RESTORATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9f1a9d35-1b2a-44a2-9bbc-8529a7487905"),
    "Argivian Restoration",
    crate::card::CardArt::new("9f1a9d35-1b2a-44a2-9bbc-8529a7487905", "Roger Raupp"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 35 — Avizoa
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static AVIZOA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a993986c-e8f1-41b1-86e6-c72021c53b87"),
    "Avizoa",
    crate::card::CardArt::new("a993986c-e8f1-41b1-86e6-c72021c53b87", "Paolo Parente"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 36 — Cloud Djinn
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CLOUD_DJINN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c857a151-45fe-43af-a9be-a93d26f220f3"),
    "Cloud Djinn",
    crate::card::CardArt::new("c857a151-45fe-43af-a9be-a93d26f220f3", "Mike Dringenberg"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 37 — Disrupt
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DISRUPT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c6cc89b0-9acf-452b-ac1a-bc7e90eb32fc"),
    "Disrupt",
    crate::card::CardArt::new("c6cc89b0-9acf-452b-ac1a-bc7e90eb32fc", "Adam Rex"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 38 — Ertai's Familiar
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ERTAI_S_FAMILIAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("354c9de7-0cdf-4302-9d1a-ae17eca13053"),
    "Ertai's Familiar",
    crate::card::CardArt::new("354c9de7-0cdf-4302-9d1a-ae17eca13053", "Kipling West"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 39 — Flux
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FLUX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3c26bf66-8fa8-4f69-9556-c9fcc56a7f33"),
    "Flux",
    crate::card::CardArt::new(
        "368b28e4-a367-4a38-866d-c3768bd9b7ad",
        "Richard Kane Ferguson",
    ),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 40 — Fog Elemental
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FOG_ELEMENTAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("28b454d0-7dc7-419f-aefa-f20f37444658"),
    "Fog Elemental",
    crate::card::CardArt::new("28b454d0-7dc7-419f-aefa-f20f37444658", "Jon J Muth"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 41 — Mana Chains
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MANA_CHAINS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("77802038-0d86-4911-97ed-e6bd2ed55e23"),
    "Mana Chains",
    crate::card::CardArt::new("77802038-0d86-4911-97ed-e6bd2ed55e23", "Bryan Talbot"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 42 — Manta Ray
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MANTA_RAY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("80f74884-9b82-419d-9e97-c947a6b7d09f"),
    "Manta Ray",
    crate::card::CardArt::new("80f74884-9b82-419d-9e97-c947a6b7d09f", "Una Fricker"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 43 — Merfolk Traders
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MERFOLK_TRADERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ebacbf23-4b69-481c-aaf7-5de7b4a6db6f"),
    "Merfolk Traders",
    crate::card::CardArt::new("ebacbf23-4b69-481c-aaf7-5de7b4a6db6f", "DiTerlizzi"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 44 — Noble Benefactor
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NOBLE_BENEFACTOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bd221f30-1773-4e05-a40f-022a9306ef89"),
    "Noble Benefactor",
    crate::card::CardArt::new("bd221f30-1773-4e05-a40f-022a9306ef89", "DiTerlizzi"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 45 — Ophidian
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static OPHIDIAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0de0a010-76a7-460f-bb4e-a152c10c3bb7"),
    "Ophidian",
    crate::card::CardArt::new("0de0a010-76a7-460f-bb4e-a152c10c3bb7", "Cliff Nielsen"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 46 — Paradigm Shift
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PARADIGM_SHIFT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e64a17a8-091d-4029-908e-31d6a050b479"),
    "Paradigm Shift",
    crate::card::CardArt::new("e64a17a8-091d-4029-908e-31d6a050b479", "Cliff Nielsen"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 47 — Pendrell Mists
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PENDRELL_MISTS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b902b972-3a93-4e4e-aa77-02ada81e6b95"),
    "Pendrell Mists",
    crate::card::CardArt::new("b902b972-3a93-4e4e-aa77-02ada81e6b95", "Andrew Robinson"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 48 — Phantom Warrior (reprint)

// WTH 49 — Phantom Wings
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PHANTOM_WINGS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a0db4c6c-aa51-487b-a591-78d93c67c775"),
    "Phantom Wings",
    crate::card::CardArt::new("a0db4c6c-aa51-487b-a591-78d93c67c775", "Una Fricker"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 50 — Psychic Vortex
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PSYCHIC_VORTEX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3bc2a419-7122-4eeb-bb64-738a647cfd82"),
    "Psychic Vortex",
    crate::card::CardArt::new("3bc2a419-7122-4eeb-bb64-738a647cfd82", "Steve Luke"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 51 — Relearn
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RELEARN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("902f8480-8ae7-4b5f-abdf-1bd46066049e"),
    "Relearn",
    crate::card::CardArt::new("902f8480-8ae7-4b5f-abdf-1bd46066049e", "Zina Saunders"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 52 — Sage Owl
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SAGE_OWL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3ee2d6a1-8b1e-47e5-9720-5683ac458250"),
    "Sage Owl",
    crate::card::CardArt::new("3ee2d6a1-8b1e-47e5-9720-5683ac458250", "Mark Poole"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 53 — Teferi's Veil
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TEFERI_S_VEIL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cbf39b80-d972-4f79-902f-cc613c32e446"),
    "Teferi's Veil",
    crate::card::CardArt::new("cbf39b80-d972-4f79-902f-cc613c32e446", "Brom"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 54 — Timid Drake
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TIMID_DRAKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("01bbdbd8-1517-4bfd-926b-465a32724082"),
    "Timid Drake",
    crate::card::CardArt::new("01bbdbd8-1517-4bfd-926b-465a32724082", "Mike Dringenberg"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 55 — Tolarian Drake
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TOLARIAN_DRAKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e04bba8a-48ee-4981-adc2-4f82c0f2c1bd"),
    "Tolarian Drake",
    crate::card::CardArt::new("e04bba8a-48ee-4981-adc2-4f82c0f2c1bd", "Mark Harrison"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 56 — Tolarian Entrancer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TOLARIAN_ENTRANCER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c29dd04a-b3aa-48b6-beef-3314344b84a6"),
    "Tolarian Entrancer",
    crate::card::CardArt::new("c29dd04a-b3aa-48b6-beef-3314344b84a6", "Bryan Talbot"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 57 — Tolarian Serpent
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TOLARIAN_SERPENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9236a857-c4ca-4de2-a4a2-e0914d16b54b"),
    "Tolarian Serpent",
    crate::card::CardArt::new("9236a857-c4ca-4de2-a4a2-e0914d16b54b", "Stuart Griffin"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 58 — Vodalian Illusionist
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VODALIAN_ILLUSIONIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9ce0e28b-9fd6-4763-8d6b-952b530358ab"),
    "Vodalian Illusionist",
    crate::card::CardArt::new("9ce0e28b-9fd6-4763-8d6b-952b530358ab", "John Matson"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 59 — Abyssal Gatekeeper
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ABYSSAL_GATEKEEPER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1734df5a-7d3a-46c7-a0ad-adbbd1be958f"),
    "Abyssal Gatekeeper",
    crate::card::CardArt::new("1734df5a-7d3a-46c7-a0ad-adbbd1be958f", "Mark Tedin"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 60 — Agonizing Memories
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static AGONIZING_MEMORIES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("be277367-a58e-429e-af1b-58163becf861"),
    "Agonizing Memories",
    crate::card::CardArt::new("be277367-a58e-429e-af1b-58163becf861", "Mike Dringenberg"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 61 — Barrow Ghoul
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BARROW_GHOUL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f7055007-83dd-40fe-b2a1-4b3132f636db"),
    "Barrow Ghoul",
    crate::card::CardArt::new("f7055007-83dd-40fe-b2a1-4b3132f636db", "Bryan Talbot"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 62 — Bone Dancer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BONE_DANCER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("207bb4cd-4525-47e0-b412-0d0e29717d44"),
    "Bone Dancer",
    crate::card::CardArt::new("207bb4cd-4525-47e0-b412-0d0e29717d44", "Scott Kirschner"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 63 — Buried Alive
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BURIED_ALIVE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("56b92eb5-72b0-46b4-8b16-8a7a7ac80f56"),
    "Buried Alive",
    crate::card::CardArt::new("56b92eb5-72b0-46b4-8b16-8a7a7ac80f56", "Brian Horton"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 64 — Circling Vultures
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CIRCLING_VULTURES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8dae8e49-c2b6-4965-9249-49f93449d271"),
    "Circling Vultures",
    crate::card::CardArt::new("8dae8e49-c2b6-4965-9249-49f93449d271", "Una Fricker"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 65 — Coils of the Medusa
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static COILS_OF_THE_MEDUSA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("502bfb38-4a37-4053-af20-d5606ffc67c8"),
    "Coils of the Medusa",
    crate::card::CardArt::new("502bfb38-4a37-4053-af20-d5606ffc67c8", "Darbury Stenderu"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 66 — Doomsday
pub(in crate::card::sets) static DOOMSDAY: CardRecord = CardRecord::new_with_legacy_id(
    2185,
    "Doomsday",
    CardArt::new("5b3c6d87-9383-450b-bba5-33435b6b0d08", "Adrian Smith"),
    CardSet::Weatherlight,
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
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FATAL_BLOW: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("044dc7c2-6198-4526-b79a-f3d8ee7a157a"),
    "Fatal Blow",
    crate::card::CardArt::new("044dc7c2-6198-4526-b79a-f3d8ee7a157a", "George Pratt"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 68 — Festering Evil
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FESTERING_EVIL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2d688bda-fee2-496d-9793-794c2568b54e"),
    "Festering Evil",
    crate::card::CardArt::new("2d688bda-fee2-496d-9793-794c2568b54e", "John Matson"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 69 — Fledgling Djinn
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FLEDGLING_DJINN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1b0fdf2a-d6d2-42da-8f41-0f67dd0bf4d2"),
    "Fledgling Djinn",
    crate::card::CardArt::new("1b0fdf2a-d6d2-42da-8f41-0f67dd0bf4d2", "Thomas Gianni"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 70 — Gallowbraid
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GALLOWBRAID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8df86192-6374-42ac-94bc-95e2e8284bd6"),
    "Gallowbraid",
    crate::card::CardArt::new("8df86192-6374-42ac-94bc-95e2e8284bd6", "Carl Critchlow"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 71 — Haunting Misery
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HAUNTING_MISERY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("939b83ba-8ba8-4b98-8a13-a037ba7805e9"),
    "Haunting Misery",
    crate::card::CardArt::new("939b83ba-8ba8-4b98-8a13-a037ba7805e9", "Gary Leach"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 72 — Hidden Horror
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HIDDEN_HORROR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("885dc4c5-2ade-4497-b579-0307c67ac783"),
    "Hidden Horror",
    crate::card::CardArt::new("885dc4c5-2ade-4497-b579-0307c67ac783", "Clint Langley"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 73 — Infernal Tribute
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static INFERNAL_TRIBUTE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("569739b2-f212-4cc9-84db-1be17b3f90fb"),
    "Infernal Tribute",
    crate::card::CardArt::new("569739b2-f212-4cc9-84db-1be17b3f90fb", "Terese Nielsen"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 74 — Mischievous Poltergeist
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MISCHIEVOUS_POLTERGEIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("054254ee-29cf-48d7-afbf-cb6de83e513e"),
    "Mischievous Poltergeist",
    crate::card::CardArt::new("054254ee-29cf-48d7-afbf-cb6de83e513e", "DiTerlizzi"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 75 — Morinfen
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MORINFEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b5006ad3-16ca-4be3-8d56-d4fe4e9e0a44"),
    "Morinfen",
    crate::card::CardArt::new("b5006ad3-16ca-4be3-8d56-d4fe4e9e0a44", "Carl Critchlow"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 76 — Necratog
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NECRATOG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fb19c519-c09a-44a0-8d4b-ab6c15dabdef"),
    "Necratog",
    crate::card::CardArt::new("fb19c519-c09a-44a0-8d4b-ab6c15dabdef", "Bryan Talbot"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 77 — Odylic Wraith
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ODYLIC_WRAITH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3a3b7cd1-051c-43a8-b5f0-72a9d704efbc"),
    "Odylic Wraith",
    crate::card::CardArt::new("3a3b7cd1-051c-43a8-b5f0-72a9d704efbc", "Ian Miller"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 78 — Razortooth Rats
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static RAZORTOOTH_RATS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ae869780-27e8-4a6d-9ac6-cdab617725e2"),
    "Razortooth Rats",
    crate::card::CardArt::new("ae869780-27e8-4a6d-9ac6-cdab617725e2", "Brian Horton"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 79 — Shadow Rider
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHADOW_RIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5bfdec24-e689-4cca-a546-a8f5d0929f8d"),
    "Shadow Rider",
    crate::card::CardArt::new("5bfdec24-e689-4cca-a546-a8f5d0929f8d", "Pete Venters"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 80 — Shattered Crypt
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHATTERED_CRYPT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("117df45d-4500-459b-96b5-ca41952580c1"),
    "Shattered Crypt",
    crate::card::CardArt::new("117df45d-4500-459b-96b5-ca41952580c1", "Gary Leach"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 81 — Spinning Darkness
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SPINNING_DARKNESS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("58e64a8e-84b1-416c-9fa7-8b10130dc9e9"),
    "Spinning Darkness",
    crate::card::CardArt::new("58e64a8e-84b1-416c-9fa7-8b10130dc9e9", "John Coulthart"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 82 — Strands of Night
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STRANDS_OF_NIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("872ef62f-e119-470b-b212-9beb48469095"),
    "Strands of Night",
    crate::card::CardArt::new("872ef62f-e119-470b-b212-9beb48469095", "Patrick Kochakji"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 83 — Tendrils of Despair
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TENDRILS_OF_DESPAIR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b5d73ddb-bd3c-4625-9f75-ba2079553915"),
    "Tendrils of Despair",
    crate::card::CardArt::new("b5d73ddb-bd3c-4625-9f75-ba2079553915", "John Coulthart"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 84 — Urborg Justice
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static URBORG_JUSTICE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("39f322ff-0b04-41ce-90cd-9896f941e703"),
    "Urborg Justice",
    crate::card::CardArt::new("39f322ff-0b04-41ce-90cd-9896f941e703", "Gary Leach"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 85 — Urborg Stalker
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static URBORG_STALKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2d33e3d5-c608-4ba8-8614-0b9d0385af64"),
    "Urborg Stalker",
    crate::card::CardArt::new("2d33e3d5-c608-4ba8-8614-0b9d0385af64", "Cliff Nielsen"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 86 — Wave of Terror
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WAVE_OF_TERROR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d40ab3e7-9abb-4acc-9932-de03b533722f"),
    "Wave of Terror",
    crate::card::CardArt::new("d40ab3e7-9abb-4acc-9932-de03b533722f", "Adrian Smith"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 87 — Zombie Scavengers
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ZOMBIE_SCAVENGERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2ec786b1-6097-4e97-99b0-571d6e3e73e7"),
    "Zombie Scavengers",
    crate::card::CardArt::new("2ec786b1-6097-4e97-99b0-571d6e3e73e7", "Patrick Kochakji"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 88 — Aether Flash
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static AETHER_FLASH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("28f6642d-393d-49a5-8c49-c1f62524ea20"),
    "Aether Flash",
    crate::card::CardArt::new("28f6642d-393d-49a5-8c49-c1f62524ea20", "Ron Spencer"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 89 — Betrothed of Fire
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BETROTHED_OF_FIRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5e517aa4-d8ba-4a49-bf9f-172bf029fa52"),
    "Betrothed of Fire",
    crate::card::CardArt::new("5e517aa4-d8ba-4a49-bf9f-172bf029fa52", "Clint Langley"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 90 — Bloodrock Cyclops
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BLOODROCK_CYCLOPS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5c642fd9-38f7-4029-ab93-e1dc5636c1ad"),
    "Bloodrock Cyclops",
    crate::card::CardArt::new("5c642fd9-38f7-4029-ab93-e1dc5636c1ad", "Tom Wänerstrand"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 91 — Bogardan Firefiend
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BOGARDAN_FIREFIEND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("80ff9650-d25f-4c6b-b96e-794b50af3f14"),
    "Bogardan Firefiend",
    crate::card::CardArt::new("80ff9650-d25f-4c6b-b96e-794b50af3f14", "Terese Nielsen"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 92 — Boiling Blood
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BOILING_BLOOD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3fcb85b6-ab5a-40db-aaae-555315f32877"),
    "Boiling Blood",
    crate::card::CardArt::new("3fcb85b6-ab5a-40db-aaae-555315f32877", "Cliff Nielsen"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 93 — Cinder Giant
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CINDER_GIANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("de97c939-2c44-4c43-9d66-1087bcee692b"),
    "Cinder Giant",
    crate::card::CardArt::new("de97c939-2c44-4c43-9d66-1087bcee692b", "Rogério Vilela"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 94 — Cinder Wall
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CINDER_WALL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6c1e429c-2e66-4363-b50a-b12b72efa060"),
    "Cinder Wall",
    crate::card::CardArt::new("6c1e429c-2e66-4363-b50a-b12b72efa060", "Randy Gallegos"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 95 — Cone of Flame
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CONE_OF_FLAME: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5713f17a-9a57-41f8-b492-ced876e1a37f"),
    "Cone of Flame",
    crate::card::CardArt::new("5713f17a-9a57-41f8-b492-ced876e1a37f", "Ron Spencer"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 96 — Desperate Gambit
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DESPERATE_GAMBIT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f4245160-274e-4c39-9bcd-c64e9a44dfdb"),
    "Desperate Gambit",
    crate::card::CardArt::new("f4245160-274e-4c39-9bcd-c64e9a44dfdb", "Pete Venters"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 97 — Dwarven Berserker
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DWARVEN_BERSERKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7bc734e9-fb09-4094-94b6-76c0458649e9"),
    "Dwarven Berserker",
    crate::card::CardArt::new("7bc734e9-fb09-4094-94b6-76c0458649e9", "Douglas Shuler"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 98 — Dwarven Thaumaturgist
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DWARVEN_THAUMATURGIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8e68aa29-9f38-48a2-b00a-39aef9d91f6d"),
    "Dwarven Thaumaturgist",
    crate::card::CardArt::new("8e68aa29-9f38-48a2-b00a-39aef9d91f6d", "Kipling West"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 99 — Fervor (reprint)

// WTH 100 — Fire Whip
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FIRE_WHIP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3ee194b4-f18f-4ebd-b42f-c7dfef42f22e"),
    "Fire Whip",
    crate::card::CardArt::new("3ee194b4-f18f-4ebd-b42f-c7dfef42f22e", "Jeff Miracola"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 101 — Firestorm
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FIRESTORM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e674aa8a-668a-4345-95ee-73a0b87bbcb1"),
    "Firestorm",
    crate::card::CardArt::new("e674aa8a-668a-4345-95ee-73a0b87bbcb1", "Jeff Miracola"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 102 — Fit of Rage
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FIT_OF_RAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("09e7b9ec-90cf-4d23-af5e-48394398ff06"),
    "Fit of Rage",
    crate::card::CardArt::new("09e7b9ec-90cf-4d23-af5e-48394398ff06", "Douglas Shuler"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 103 — Goblin Bomb
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_BOMB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("97e8a436-9fd0-409f-a020-0f9f41602d50"),
    "Goblin Bomb",
    crate::card::CardArt::new("97e8a436-9fd0-409f-a020-0f9f41602d50", "Ron Spencer"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 104 — Goblin Grenadiers
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_GRENADIERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5a73db23-727f-4d63-97d7-2ca542276722"),
    "Goblin Grenadiers",
    crate::card::CardArt::new("5a73db23-727f-4d63-97d7-2ca542276722", "Dan Frazier"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 105 — Goblin Vandal
pub(in crate::card::sets) static GOBLIN_VANDAL: CardRecord = CardRecord::new_with_legacy_id(
    2032,
    "Goblin Vandal",
    CardArt::new("b7ad3b81-f706-4b33-b1ec-7600182a5232", "Franz Vohwinkel"),
    CardSet::Weatherlight,
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
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HEART_OF_BOGARDAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4e30d025-1df9-4a08-b686-037e9cbf23a6"),
    "Heart of Bogardan",
    crate::card::CardArt::new("4e30d025-1df9-4a08-b686-037e9cbf23a6", "Terese Nielsen"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 107 — Heat Stroke
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HEAT_STROKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1baf2a6c-57ec-4b38-8b08-4b3f800dbe99"),
    "Heat Stroke",
    crate::card::CardArt::new("1baf2a6c-57ec-4b38-8b08-4b3f800dbe99", "Andrew Robinson"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 108 — Hurloon Shaman
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HURLOON_SHAMAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("70a359c9-1889-426d-acaf-074cfd9f274d"),
    "Hurloon Shaman",
    crate::card::CardArt::new("70a359c9-1889-426d-acaf-074cfd9f274d", "Scott M. Fischer"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 109 — Lava Hounds
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LAVA_HOUNDS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("896dcaf0-3e52-4189-990f-cabab40ffbd1"),
    "Lava Hounds",
    crate::card::CardArt::new("896dcaf0-3e52-4189-990f-cabab40ffbd1", "Steve White"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 110 — Lava Storm
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LAVA_STORM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("61fcd58e-e5e2-45f4-9edd-300a871ae5f5"),
    "Lava Storm",
    crate::card::CardArt::new("61fcd58e-e5e2-45f4-9edd-300a871ae5f5", "Scott Kirschner"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 111 — Maraxus of Keld
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MARAXUS_OF_KELD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("59329155-a423-4e8d-a7d4-c99555ff5ed1"),
    "Maraxus of Keld",
    crate::card::CardArt::new("59329155-a423-4e8d-a7d4-c99555ff5ed1", "Adrian Smith"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 112 — Orcish Settlers
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ORCISH_SETTLERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d54764f6-6f65-405c-ba30-1e485ce3fe21"),
    "Orcish Settlers",
    crate::card::CardArt::new("d54764f6-6f65-405c-ba30-1e485ce3fe21", "Pete Venters"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 113 — Roc Hatchling
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ROC_HATCHLING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("25857884-6bb7-4a8e-a08b-fa610af8a5c3"),
    "Roc Hatchling",
    crate::card::CardArt::new("25857884-6bb7-4a8e-a08b-fa610af8a5c3", "Una Fricker"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 114 — Sawtooth Ogre
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SAWTOOTH_OGRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4a237580-f7f6-4d6b-a342-0d11fc0b5a59"),
    "Sawtooth Ogre",
    crate::card::CardArt::new("4a237580-f7f6-4d6b-a342-0d11fc0b5a59", "Brom"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 115 — Thunderbolt (reprint)

// WTH 116 — Thundermare
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static THUNDERMARE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("59a9f3f5-c80f-47a4-bf84-b7262437017f"),
    "Thundermare",
    crate::card::CardArt::new("e936e5cb-0a8e-4348-afea-e5f96b19fe23", "Bob Eggleton"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 117 — Aboroth
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ABOROTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8c72ac67-e4fb-49a1-b1e5-cd2e414bec28"),
    "Aboroth",
    crate::card::CardArt::new("8c72ac67-e4fb-49a1-b1e5-cd2e414bec28", "Brom"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 118 — Arctic Wolves
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ARCTIC_WOLVES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b5fb56a2-5138-4c31-aa4b-0824a1a24573"),
    "Arctic Wolves",
    crate::card::CardArt::new("b5fb56a2-5138-4c31-aa4b-0824a1a24573", "Steve White"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 119 — Barishi
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BARISHI: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f263eb80-f8f2-4b32-8e8b-a297de9f3666"),
    "Barishi",
    crate::card::CardArt::new("f263eb80-f8f2-4b32-8e8b-a297de9f3666", "Ted Naifeh"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 120 — Blossoming Wreath
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BLOSSOMING_WREATH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2f944ad9-c9ce-47b2-80fa-d0f7fcf0fd5d"),
    "Blossoming Wreath",
    crate::card::CardArt::new("2f944ad9-c9ce-47b2-80fa-d0f7fcf0fd5d", "Brian Durfee"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 121 — Briar Shield
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BRIAR_SHIELD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("68100ac2-9677-4eb5-93dc-54e49b15985d"),
    "Briar Shield",
    crate::card::CardArt::new("68100ac2-9677-4eb5-93dc-54e49b15985d", "Scott Kirschner"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 122 — Call of the Wild
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CALL_OF_THE_WILD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a742bc7c-7f0d-4dff-b229-f16d54fe1347"),
    "Call of the Wild",
    crate::card::CardArt::new("a742bc7c-7f0d-4dff-b229-f16d54fe1347", "Brom"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 123 — Choking Vines
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CHOKING_VINES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6cc4a7ee-f6f0-454a-9074-5988fdee1f34"),
    "Choking Vines",
    crate::card::CardArt::new("6cc4a7ee-f6f0-454a-9074-5988fdee1f34", "Ted Naifeh"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 124 — Dense Foliage
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DENSE_FOLIAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c60a2035-59cb-426e-b2ae-45d8d6ce0bb8"),
    "Dense Foliage",
    crate::card::CardArt::new("c60a2035-59cb-426e-b2ae-45d8d6ce0bb8", "Alan Rabinowitz"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 125 — Downdraft
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DOWNDRAFT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ab4ced80-926a-4e4d-8ebd-d4fe7374a6ad"),
    "Downdraft",
    crate::card::CardArt::new("ab4ced80-926a-4e4d-8ebd-d4fe7374a6ad", "John Matson"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 126 — Fallow Wurm
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FALLOW_WURM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1ba02b6f-6010-47a4-8670-406391a52a68"),
    "Fallow Wurm",
    crate::card::CardArt::new("1ba02b6f-6010-47a4-8670-406391a52a68", "Stephen L. Walsh"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 127 — Familiar Ground
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FAMILIAR_GROUND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f993f517-999f-4ee6-8ffb-946bffdcf7fe"),
    "Familiar Ground",
    crate::card::CardArt::new("f993f517-999f-4ee6-8ffb-946bffdcf7fe", "Jeff Miracola"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 128 — Fungus Elemental
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FUNGUS_ELEMENTAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4336bfd1-27a4-414d-b6fe-f186a0563dc0"),
    "Fungus Elemental",
    crate::card::CardArt::new("4336bfd1-27a4-414d-b6fe-f186a0563dc0", "Scott M. Fischer"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 129 — Gaea's Blessing
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GAEA_S_BLESSING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ee83d511-57e0-40fb-a4db-62f6c2c39888"),
    "Gaea's Blessing",
    crate::card::CardArt::new("ee83d511-57e0-40fb-a4db-62f6c2c39888", "Rebecca Guay"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 130 — Harvest Wurm
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HARVEST_WURM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9d21139d-edfc-4140-aa43-d4165331d7f3"),
    "Harvest Wurm",
    crate::card::CardArt::new("9d21139d-edfc-4140-aa43-d4165331d7f3", "Stephen L. Walsh"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 131 — Liege of the Hollows
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LIEGE_OF_THE_HOLLOWS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dff4512b-8244-4e38-bffb-0062a97d9531"),
    "Liege of the Hollows",
    crate::card::CardArt::new("dff4512b-8244-4e38-bffb-0062a97d9531", "Ron Spencer"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 132 — Llanowar Behemoth
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LLANOWAR_BEHEMOTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3d5d9bd0-7ce9-4a1e-a8b2-5c1dbb014917"),
    "Llanowar Behemoth",
    crate::card::CardArt::new("3d5d9bd0-7ce9-4a1e-a8b2-5c1dbb014917", "Hannibal King"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 133 — Llanowar Druid
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LLANOWAR_DRUID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ffad279c-762a-42cf-ac20-f4e48734c194"),
    "Llanowar Druid",
    crate::card::CardArt::new("ffad279c-762a-42cf-ac20-f4e48734c194", "Pete Venters"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 134 — Llanowar Sentinel
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LLANOWAR_SENTINEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6f37ea4b-66e2-4ad5-ae7f-d02fd59131bd"),
    "Llanowar Sentinel",
    crate::card::CardArt::new("6f37ea4b-66e2-4ad5-ae7f-d02fd59131bd", "Douglas Shuler"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 135 — Mwonvuli Ooze
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MWONVULI_OOZE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aa9c6f65-93a1-4913-87e7-a17ebfcc7780"),
    "Mwonvuli Ooze",
    crate::card::CardArt::new("aa9c6f65-93a1-4913-87e7-a17ebfcc7780", "Zina Saunders"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 136 — Nature's Kiss
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NATURE_S_KISS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("64b09c44-d463-45a9-9fa2-89407c21200b"),
    "Nature's Kiss",
    crate::card::CardArt::new("64b09c44-d463-45a9-9fa2-89407c21200b", "Scott M. Fischer"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 137 — Nature's Resurgence
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NATURE_S_RESURGENCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2df9fb85-f7fa-4617-87bd-4d457c830f46"),
    "Nature's Resurgence",
    crate::card::CardArt::new("2df9fb85-f7fa-4617-87bd-4d457c830f46", "Scott M. Fischer"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 138 — Redwood Treefolk
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static REDWOOD_TREEFOLK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e9399667-ae2a-4b64-84dd-8f97f3e5fe79"),
    "Redwood Treefolk",
    crate::card::CardArt::new("0274e162-33e4-4604-a6ea-51fc1a5c6a04", "Phil Foglio"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 139 — Rogue Elephant
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ROGUE_ELEPHANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1b622b2f-84ad-4203-97fa-35af09e1c370"),
    "Rogue Elephant",
    crate::card::CardArt::new("1b622b2f-84ad-4203-97fa-35af09e1c370", "Steve White"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 140 — Striped Bears
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STRIPED_BEARS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0bf54365-56ae-485d-b931-784a4cf9d8f2"),
    "Striped Bears",
    crate::card::CardArt::new("0bf54365-56ae-485d-b931-784a4cf9d8f2", "Una Fricker"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 141 — Sylvan Hierophant
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SYLVAN_HIEROPHANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("432a6908-0ee3-45c5-9089-b7f8cf1184bb"),
    "Sylvan Hierophant",
    crate::card::CardArt::new("432a6908-0ee3-45c5-9089-b7f8cf1184bb", "Brian Durfee"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 142 — Tranquil Grove
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TRANQUIL_GROVE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c4a145f2-b59d-4728-922c-9bc228451432"),
    "Tranquil Grove",
    crate::card::CardArt::new("c4a145f2-b59d-4728-922c-9bc228451432", "Dylan Martens"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 143 — Uktabi Efreet
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static UKTABI_EFREET: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3678a224-d314-4108-8a39-de0c1b635b5c"),
    "Uktabi Efreet",
    crate::card::CardArt::new("3678a224-d314-4108-8a39-de0c1b635b5c", "Alan Rabinowitz"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 144 — Veteran Explorer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VETERAN_EXPLORER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bdac36f2-99ce-4d48-90fa-aa7439778ffc"),
    "Veteran Explorer",
    crate::card::CardArt::new("bdac36f2-99ce-4d48-90fa-aa7439778ffc", "David A. Cherry"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 145 — Vitalize
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VITALIZE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d6ee4997-4b1a-4e03-88ac-63b451bb7b38"),
    "Vitalize",
    crate::card::CardArt::new("d6ee4997-4b1a-4e03-88ac-63b451bb7b38", "Pete Venters"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 146 — Bubble Matrix
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BUBBLE_MATRIX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0ca9c239-84ff-4527-aa23-bdb11856744c"),
    "Bubble Matrix",
    crate::card::CardArt::new("0ca9c239-84ff-4527-aa23-bdb11856744c", "Brom"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 147 — Bösium Strip
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BOSIUM_STRIP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3884bede-df28-42e8-9ac9-ae03118b1985"),
    "Bösium Strip",
    crate::card::CardArt::new("3884bede-df28-42e8-9ac9-ae03118b1985", "Steve Luke"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 148 — Chimeric Sphere
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CHIMERIC_SPHERE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cc96857c-b38e-4614-9838-cacd3700e3ee"),
    "Chimeric Sphere",
    crate::card::CardArt::new("cc96857c-b38e-4614-9838-cacd3700e3ee", "Colin MacNeil"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 149 — Dingus Staff
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DINGUS_STAFF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("065b4358-5dee-4f13-bff9-8254bdb92069"),
    "Dingus Staff",
    crate::card::CardArt::new(
        "065b4358-5dee-4f13-bff9-8254bdb92069",
        "Richard Kane Ferguson",
    ),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 150 — Jabari's Banner
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static JABARI_S_BANNER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3d51a496-1ca6-4286-bdbe-990d43196a25"),
    "Jabari's Banner",
    crate::card::CardArt::new("3d51a496-1ca6-4286-bdbe-990d43196a25", "Mark Harrison"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 151 — Jangling Automaton
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static JANGLING_AUTOMATON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2e2a427b-9869-4059-aeeb-d9b97b324e4e"),
    "Jangling Automaton",
    crate::card::CardArt::new("2e2a427b-9869-4059-aeeb-d9b97b324e4e", "Adam Rex"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 152 — Mana Web
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MANA_WEB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2c72ec90-dacc-496f-a7f5-f18bfce5eb3e"),
    "Mana Web",
    crate::card::CardArt::new("2c72ec90-dacc-496f-a7f5-f18bfce5eb3e", "Hannibal King"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 153 — Mind Stone
pub(in crate::card::sets) static MIND_STONE: CardRecord = CardRecord::new_with_legacy_id(
    2117,
    "Mind Stone",
    CardArt::new("162e81d3-6cd4-4cb8-8ed8-cfbd8d34ca71", "Adam Rex"),
    CardSet::Weatherlight,
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
pub(in crate::card::sets) static NULL_ROD: CardRecord = CardRecord::new_with_legacy_id(
    2283,
    "Null Rod",
    CardArt::new("bc45f2cb-c256-4a0f-879a-c7db5b1a0b94", "Anson Maddocks"),
    CardSet::Weatherlight,
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
pub(in crate::card::sets) static PHYREXIAN_FURNACE: CardRecord = CardRecord::new_with_legacy_id(
    2054,
    "Phyrexian Furnace",
    CardArt::new("e98bca31-a1f4-4d9e-bbb8-fd9b6f4d2b91", "George Pratt"),
    CardSet::Weatherlight,
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
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SERRATED_BISKELION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c449126c-ac01-4a90-b967-8c3ad112091b"),
    "Serrated Biskelion",
    crate::card::CardArt::new("c449126c-ac01-4a90-b967-8c3ad112091b", "Ron Spencer"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 157 — Steel Golem
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STEEL_GOLEM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9aa927e0-5a65-4ac1-8eca-c000bb8080e7"),
    "Steel Golem",
    crate::card::CardArt::new("9aa927e0-5a65-4ac1-8eca-c000bb8080e7", "Donato Giancola"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 158 — Straw Golem
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STRAW_GOLEM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("43d62479-92ac-43e2-a3d3-b41dfe0fbb20"),
    "Straw Golem",
    crate::card::CardArt::new("43d62479-92ac-43e2-a3d3-b41dfe0fbb20", "Bryan Talbot"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 159 — Thran Forge
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static THRAN_FORGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b9c9691b-bee8-4251-8275-5f6ba14a8ecd"),
    "Thran Forge",
    crate::card::CardArt::new("b9c9691b-bee8-4251-8275-5f6ba14a8ecd", "Mark Poole"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 160 — Thran Tome
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static THRAN_TOME: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("63db7360-fe6e-430f-bfee-a2f80bcb6fec"),
    "Thran Tome",
    crate::card::CardArt::new("63db7360-fe6e-430f-bfee-a2f80bcb6fec", "Donato Giancola"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 161 — Touchstone
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TOUCHSTONE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("923afe8a-e82c-4b93-bb42-8f5073acae13"),
    "Touchstone",
    crate::card::CardArt::new("923afe8a-e82c-4b93-bb42-8f5073acae13", "George Pratt"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 162 — Well of Knowledge
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WELL_OF_KNOWLEDGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5184b967-f474-4c9b-9a20-65ddb0d6e4f8"),
    "Well of Knowledge",
    crate::card::CardArt::new(
        "5184b967-f474-4c9b-9a20-65ddb0d6e4f8",
        "D. Alexander Gregory",
    ),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 163 — Xanthic Statue
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static XANTHIC_STATUE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8becb285-cd91-4de0-af59-ddaa7d8c5366"),
    "Xanthic Statue",
    crate::card::CardArt::new("8becb285-cd91-4de0-af59-ddaa7d8c5366", "Hannibal King"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 164 — Gemstone Mine
pub(in crate::card::sets) static GEMSTONE_MINE: CardRecord = CardRecord::new_with_legacy_id(
    2049,
    "Gemstone Mine",
    CardArt::new("09507f7f-c58f-4f57-b878-b39811a5b619", "Brom"),
    CardSet::Weatherlight,
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
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LOTUS_VALE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2e5cd12a-2a07-44a8-8eac-de00d26fe9e3"),
    "Lotus Vale",
    crate::card::CardArt::new("2e5cd12a-2a07-44a8-8eac-de00d26fe9e3", "John Avon"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 166 — Scorched Ruins
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SCORCHED_RUINS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("75a4e843-937c-47fb-8768-0f42c5cb4e4f"),
    "Scorched Ruins",
    crate::card::CardArt::new("75a4e843-937c-47fb-8768-0f42c5cb4e4f", "John Avon"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

// WTH 167 — Winding Canyons
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static WINDING_CANYONS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f26672a8-f4ff-4c64-bb3e-f5072bbc9e3e"),
    "Winding Canyons",
    crate::card::CardArt::new("f26672a8-f4ff-4c64-bb3e-f5072bbc9e3e", "John Avon"),
    crate::card::CardSet::Weatherlight,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ABEYANCE,
    &ALABASTER_DRAGON,
    &ALMS,
    &ANGELIC_RENEWAL,
    &ARDENT_MILITIA,
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
    &FLUX,
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
    &THUNDERMARE,
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
    &REDWOOD_TREEFOLK,
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
    PrintingRecord::reprint(&catalog_m14::PHANTOM_WARRIOR), // WTH 48
    PrintingRecord::reprint(&catalog_m13::FERVOR),          // WTH 99
    PrintingRecord::reprint(&catalog_avr::THUNDERBOLT),     // WTH 115
];
