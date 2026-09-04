//! Urza's Destiny cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::ResolvedEffectDurationDef;
use crate::card::abilities;
use crate::card::{
    AbilityCostDef, AbilityDef, AppliedEffectDef, CardRules, CardSet, CardType, CardTypeSet,
    CharacteristicOperationDef, CounterKind, EffectDef, EffectRecipientDef, ObjectPredicateDef,
    ObjectQueryDef, ObjectSetDef, PlayerRelation, PowerToughnessOperationDef, SetOperationDef,
    TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, ZonePlacement,
};
use crate::mana_cost;

// UDS 1 — Academy Rector
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ACADEMY_RECTOR: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Academy Rector",
    "4367bc78-0912-4abd-8edd-bc792558d01a",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// UDS 2 — Archery Training
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARCHERY_TRAINING: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Archery Training",
    "151232e6-68cc-4cac-a532-9ade8e925961",
    "Mark Brill",
    crate::card::CardRules::unsupported(),
);

// UDS 3 — Capashen Knight
pub(in crate::card::sets) static CAPASHEN_KNIGHT: CardRecord = CardRecord::new(
    CardSet::UrzasDestiny,
    "Capashen Knight",
    "34d7f78e-d310-4305-942e-7839583bd893",
    "Kev Walker",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Knight"], 1, 1).with_abilities(&[
        abilities::first_strike(),
        AbilityDef::activated(
            "{1}{W}: This creature gets +1/+0 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{W}"))],
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

// UDS 4 — Capashen Standard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CAPASHEN_STANDARD: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Capashen Standard",
    "16665386-405e-48c9-8c69-c21b03931c2f",
    "Todd Lockwood",
    crate::card::CardRules::unsupported(),
);

// UDS 5 — Capashen Templar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CAPASHEN_TEMPLAR: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Capashen Templar",
    "0976a193-463a-4bcb-a951-ca73347a5572",
    "Todd Lockwood",
    crate::card::CardRules::unsupported(),
);

// UDS 6 — False Prophet
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FALSE_PROPHET: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "False Prophet",
    "5fcb46d3-1ddf-4e3b-9ac7-a3fee49f04c6",
    "Eric Peterson",
    crate::card::CardRules::unsupported(),
);

// UDS 7 — Fend Off
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FEND_OFF: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Fend Off",
    "a64d7a33-986d-45ad-8662-7bca80d3628d",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// UDS 8 — Field Surgeon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FIELD_SURGEON: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Field Surgeon",
    "bb830403-0832-47f7-b4b4-4f241f1b9112",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// UDS 9 — Flicker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLICKER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Flicker",
    "f55e7ec5-6488-483f-8020-b48e1a951f09",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// UDS 10 — Jasmine Seer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JASMINE_SEER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Jasmine Seer",
    "a6641dd2-5b9c-4089-8a71-a3a1a9c29f8b",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// UDS 11 — Mask of Law and Grace
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MASK_OF_LAW_AND_GRACE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Mask of Law and Grace",
    "6bd97150-b405-4bb5-b5a8-fceda4a45ebb",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// UDS 12 — Master Healer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MASTER_HEALER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Master Healer",
    "e21a342c-ad80-43e4-8b2d-1c48241c52b1",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// UDS 13 — Opalescence
pub(in crate::card::sets) static OPALESCENCE: CardRecord = CardRecord::new(
    CardSet::UrzasDestiny,
    "Opalescence",
    "3c0071fb-afa5-47b5-b266-2b10a4f5a98a",
    "John Avon",
    // The deck's whole win condition: the enchantments it already wanted to
    // resolve stand up and attack.
    CardRules::new_enchantment(mana_cost!("{2}{W}{W}")).with_ability(AbilityDef::static_ability(
        "Each other non-Aura enchantment is a creature in addition to its other types and has base power and base toughness each equal to its mana value.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                // Every other non-Aura enchantment. An Aura is left alone because a
                // creature Aura would fall off whatever it was attached to, and the
                // enchantment doing the animating is not one of the things it animates.
                ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Enchantment),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Aura")),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
            )),
            // A creature in addition to its other types, with a body its own cost
            // decides: the number is read off each affected enchantment rather than off
            // the Opalescence.
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::Characteristic(CharacteristicOperationDef::CardTypes(SetOperationDef::Add(
                    CardTypeSet::single(CardType::Creature),
                ))),
                AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
                    PowerToughnessOperationDef::SetBase {
                        power: ValueDef::AffectedManaValue,
                        toughness: ValueDef::AffectedManaValue,
                    },
                )),
            ]),
        },
    )),
);

// UDS 14 — Reliquary Monk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RELIQUARY_MONK: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Reliquary Monk",
    "243e9386-2a7f-406a-9ed3-77d4bf1b50fd",
    "Thomas M. Baxa",
    crate::card::CardRules::unsupported(),
);

// UDS 15 — Replenish
pub(in crate::card::sets) static REPLENISH: CardRecord = CardRecord::new(
    CardSet::UrzasDestiny,
    "Replenish",
    "7fd2fe13-bbc0-42b7-bc42-3b51910ce118",
    "Jim Nelson",
    // The deck is built to fill its own graveyard first, so this is not
    // recursion so much as the whole board arriving on one turn.
    CardRules::new_sorcery(mana_cost!("{3}{W}")).with_ability(AbilityDef::spell(
        "Return all enchantment cards from your graveyard to the battlefield.",
        EffectDef::MoveToZone {
            // Every enchantment card the graveyard holds, all at once. The printed
            // reminder about Auras is the ordinary rule for an Aura arriving with
            // nothing to enchant, not a clause of its own.
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Enchantment),
                &[ZoneKind::Graveyard],
                PlayerRelation::You,
            ),
            zone: ZoneKind::Battlefield,
            placement: ZonePlacement::Top,
        },
    )),
);

// UDS 16 — Sanctimony
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SANCTIMONY: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Sanctimony",
    "cfc6b744-92c7-4839-9b27-833bedb92bba",
    "Mark Brill",
    crate::card::CardRules::unsupported(),
);

// UDS 17 — Scent of Jasmine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCENT_OF_JASMINE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Scent of Jasmine",
    "c0dae0cf-9696-498c-8d28-dc8c239faec7",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// UDS 18 — Scour
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCOUR: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Scour",
    "cac5162e-39ea-4f01-92eb-182fe23c1608",
    "Eric Peterson",
    crate::card::CardRules::unsupported(),
);

// UDS 19 — Serra Advocate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SERRA_ADVOCATE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Serra Advocate",
    "cac540c7-8b3a-4e28-96a2-d414ff613640",
    "Scott Hampton",
    crate::card::CardRules::unsupported(),
);

// UDS 20 — Solidarity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOLIDARITY: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Solidarity",
    "c5e1589a-ec4f-47a4-b758-ada7f49ffb8f",
    "John Zeleznik",
    crate::card::CardRules::unsupported(),
);

// UDS 21 — Tethered Griffin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TETHERED_GRIFFIN: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Tethered Griffin",
    "2bed19fa-e497-48de-8459-030e60fdc9a8",
    "Matthew D. Wilson",
    crate::card::CardRules::unsupported(),
);

// UDS 22 — Tormented Angel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TORMENTED_ANGEL: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Tormented Angel",
    "00d4d751-50df-4d8f-a6d9-4e76797c429a",
    "Greg Hildebrandt & Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// UDS 23 — Voice of Duty
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VOICE_OF_DUTY: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Voice of Duty",
    "1c648e59-c872-4e04-b45f-2729b42410af",
    "Mark Zug",
    crate::card::CardRules::unsupported(),
);

// UDS 24 — Voice of Reason
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VOICE_OF_REASON: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Voice of Reason",
    "ed3d5a10-6d4b-4383-b400-7323f2b4670e",
    "Ray Lago",
    crate::card::CardRules::unsupported(),
);

// UDS 25 — Wall of Glare
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WALL_OF_GLARE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Wall of Glare",
    "5f159193-fcf8-437c-b14a-06718a446a5c",
    "Patrick Ho",
    crate::card::CardRules::unsupported(),
);

// UDS 26 — Aura Thief
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AURA_THIEF: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Aura Thief",
    "8ae591d2-b9d3-4bc5-bcec-5d3d79a13b41",
    "Ron Spears",
    crate::card::CardRules::unsupported(),
);

// UDS 27 — Blizzard Elemental
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLIZZARD_ELEMENTAL: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Blizzard Elemental",
    "5949c5a7-9656-466a-add8-1800973fefee",
    "Thomas M. Baxa",
    crate::card::CardRules::unsupported(),
);

// UDS 28 — Brine Seer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRINE_SEER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Brine Seer",
    "2f6e5575-b004-417f-9366-6ba7840a79e7",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// UDS 29 — Bubbling Beebles
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BUBBLING_BEEBLES: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Bubbling Beebles",
    "002cf7d8-3fc2-48eb-a727-a1ce5a049665",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// UDS 30 — Disappear
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DISAPPEAR: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Disappear",
    "bdf280f4-74a1-4e6f-aec6-1852f04204e4",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// UDS 31 — Donate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DONATE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Donate",
    "7f6d8ce9-f8c8-45ad-b74c-97fba0e2982e",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// UDS 32 — Fatigue
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FATIGUE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Fatigue",
    "660fb109-dd65-4410-99b9-a2a14f8ea202",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// UDS 33 — Fledgling Osprey
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLEDGLING_OSPREY: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Fledgling Osprey",
    "8cd46bfa-ca09-422f-9891-db9399fa2d3a",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// UDS 34 — Illuminated Wings
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ILLUMINATED_WINGS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Illuminated Wings",
    "7f98e703-a0d3-497f-840a-aa026b02d47f",
    "Jim Nelson",
    crate::card::CardRules::unsupported(),
);

// UDS 35 — Iridescent Drake
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IRIDESCENT_DRAKE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Iridescent Drake",
    "70cbc36d-3391-4086-9b81-fb1ef0b83046",
    "Jim Nelson",
    crate::card::CardRules::unsupported(),
);

// UDS 36 — Kingfisher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KINGFISHER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Kingfisher",
    "442bc3ba-00b3-4616-a5b2-55524ff8a736",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// UDS 37 — Mental Discipline
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MENTAL_DISCIPLINE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Mental Discipline",
    "5e9ffd83-b5c9-46b4-bc5a-172ca34ddc79",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// UDS 38 — Metathran Elite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static METATHRAN_ELITE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Metathran Elite",
    "aa941f17-1b81-4017-90ae-4466eba8da2f",
    "Jim Nelson",
    crate::card::CardRules::unsupported(),
);

// UDS 39 — Metathran Soldier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static METATHRAN_SOLDIER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Metathran Soldier",
    "650d40d0-78ec-4b6e-8ea0-28d43ce175d5",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// UDS 40 — Opposition
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OPPOSITION: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Opposition",
    "95be2701-af7c-483e-8165-e8bd4b2774ed",
    "Todd Lockwood",
    crate::card::CardRules::unsupported(),
);

// UDS 41 — Private Research
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PRIVATE_RESEARCH: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Private Research",
    "6f7f9849-a4bd-4501-9473-79345c751701",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// UDS 42 — Quash
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static QUASH: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Quash",
    "62019ac4-a5a1-4a8c-bfb4-96e818949bbe",
    "Don Hazeltine",
    crate::card::CardRules::unsupported(),
);

// UDS 43 — Rayne, Academy Chancellor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAYNE_ACADEMY_CHANCELLOR: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Rayne, Academy Chancellor",
    "8ee6480c-7697-47e2-893b-ca88c0ab3376",
    "Matthew D. Wilson",
    crate::card::CardRules::unsupported(),
);

// UDS 44 — Rescue
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RESCUE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Rescue",
    "63fc979e-7758-4310-9259-659e9ced2c7f",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// UDS 45 — Scent of Brine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCENT_OF_BRINE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Scent of Brine",
    "d117bf8d-23ec-4f9d-99d0-3a990c5f7075",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// UDS 46 — Sigil of Sleep
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SIGIL_OF_SLEEP: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Sigil of Sleep",
    "a31f3c70-e2c3-479e-8d22-2fd1429e9857",
    "Greg Hildebrandt & Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// UDS 47 — Telepathic Spies
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TELEPATHIC_SPIES: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Telepathic Spies",
    "769e7f64-e32c-4242-aae9-45d50b89ff1f",
    "Thomas M. Baxa",
    crate::card::CardRules::unsupported(),
);

// UDS 48 — Temporal Adept
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEMPORAL_ADEPT: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Temporal Adept",
    "07bb9695-a8b0-47b4-9a03-11d559412f33",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// UDS 49 — Thieving Magpie
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THIEVING_MAGPIE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Thieving Magpie",
    "2b6b23b9-4569-40ff-988f-ad1d5d3fe573",
    "Una Fricker",
    crate::card::CardRules::unsupported(),
);

// UDS 50 — Treachery
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TREACHERY: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Treachery",
    "613694aa-b169-400d-8063-2b83d8303611",
    "Matthew D. Wilson",
    crate::card::CardRules::unsupported(),
);

// UDS 51 — Apprentice Necromancer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static APPRENTICE_NECROMANCER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Apprentice Necromancer",
    "6d7cc1f6-9897-4de4-8e94-40cbe2d962a2",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// UDS 52 — Attrition
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ATTRITION: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Attrition",
    "e3eb615b-249d-433f-a521-8310e8784b5d",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// UDS 53 — Body Snatcher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BODY_SNATCHER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Body Snatcher",
    "c7d4c858-5a11-485d-a514-12a6d80459f0",
    "Mark Zug",
    crate::card::CardRules::unsupported(),
);

// UDS 54 — Bubbling Muck
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BUBBLING_MUCK: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Bubbling Muck",
    "6ca76614-78a1-4535-9162-70469d1e8a13",
    "Greg Hildebrandt & Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// UDS 55 — Carnival of Souls
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CARNIVAL_OF_SOULS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Carnival of Souls",
    "847340fb-9251-4439-b33b-f86bff507dcd",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// UDS 56 — Chime of Night
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHIME_OF_NIGHT: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Chime of Night",
    "1ec7c917-b254-4643-afc2-b6387f267469",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// UDS 57 — Disease Carriers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DISEASE_CARRIERS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Disease Carriers",
    "49125cfc-dbae-4543-9d2d-4cc78f45ce9a",
    "Chippy & Matthew D. Wilson",
    crate::card::CardRules::unsupported(),
);

// UDS 58 — Dying Wail
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DYING_WAIL: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Dying Wail",
    "2a25a472-495e-4062-b66f-c37f148b494f",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// UDS 59 — Encroach
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENCROACH: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Encroach",
    "fbd48dac-0a1a-49c4-8daf-11972b990454",
    "rk post & Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// UDS 60 — Eradicate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ERADICATE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Eradicate",
    "0fad4607-c11d-4407-b5fa-bd34f74e41b3",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// UDS 61 — Festering Wound
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FESTERING_WOUND: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Festering Wound",
    "927eed13-510b-4b06-811d-91a6a069cb8c",
    "Chippy & Matthew D. Wilson",
    crate::card::CardRules::unsupported(),
);

// UDS 62 — Lurking Jackals
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LURKING_JACKALS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Lurking Jackals",
    "97d082d8-f401-47ad-845c-77776ee647ba",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// UDS 63 — Nightshade Seer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NIGHTSHADE_SEER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Nightshade Seer",
    "e2262467-1354-4aec-84a2-21916c44b9ef",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// UDS 64 — Phyrexian Monitor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_MONITOR: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Phyrexian Monitor",
    "571058b0-7e10-4259-8db9-5c8b78c1e13d",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// UDS 65 — Phyrexian Negator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_NEGATOR: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Phyrexian Negator",
    "45a02d67-5931-49ae-a28e-57aa6f9c7f83",
    "John Zeleznik",
    crate::card::CardRules::unsupported(),
);

// UDS 66 — Plague Dogs
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PLAGUE_DOGS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Plague Dogs",
    "6b9cebd8-aa3f-4e22-8d15-d4b7bad355e4",
    "Chippy & Matthew D. Wilson",
    crate::card::CardRules::unsupported(),
);

// UDS 67 — Rapid Decay
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAPID_DECAY: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Rapid Decay",
    "1678d911-1456-4631-a2f4-d7de4906644b",
    "Chippy",
    crate::card::CardRules::unsupported(),
);

// UDS 68 — Ravenous Rats (reprint)
const RAVENOUS_RATS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1998::portal_second_age::RAVENOUS_RATS,
    "a9185188-9a32-4ccd-8f33-b0c299901d81",
    "Carl Critchlow",
);

// UDS 69 — Scent of Nightshade
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCENT_OF_NIGHTSHADE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Scent of Nightshade",
    "582468a6-0ea9-411e-a694-13977d47c877",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// UDS 70 — Skittering Horror
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKITTERING_HORROR: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Skittering Horror",
    "80cd2771-9681-4b4a-8c2c-a2ffd7361c35",
    "Mark Zug",
    crate::card::CardRules::unsupported(),
);

// UDS 71 — Slinking Skirge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SLINKING_SKIRGE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Slinking Skirge",
    "00522c4b-4e64-4403-96b1-df41afbe255f",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// UDS 72 — Soul Feast
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOUL_FEAST: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Soul Feast",
    "9417a9db-5101-4fe1-84b7-283ca1fd42e5",
    "Ray Lago",
    crate::card::CardRules::unsupported(),
);

// UDS 73 — Squirming Mass
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SQUIRMING_MASS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Squirming Mass",
    "e47793a3-9a98-4733-8d6a-2fb1a67b15c9",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// UDS 74 — Twisted Experiment
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TWISTED_EXPERIMENT: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Twisted Experiment",
    "64e37889-7dc0-476b-8b99-8f06881d352c",
    "rk post",
    crate::card::CardRules::unsupported(),
);

// UDS 75 — Yawgmoth's Bargain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static YAWGMOTH_S_BARGAIN: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Yawgmoth's Bargain",
    "86901bf2-7722-43f8-b879-7a30630371fa",
    "Michael Sutfin",
    crate::card::CardRules::unsupported(),
);

// UDS 76 — Aether Sting
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AETHER_STING: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Aether Sting",
    "66a09917-50ce-4b51-a5cf-e28e88a45762",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// UDS 77 — Bloodshot Cyclops
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLOODSHOT_CYCLOPS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Bloodshot Cyclops",
    "9320f3d8-0e51-43d0-aedb-bfed771101e9",
    "Ray Lago",
    crate::card::CardRules::unsupported(),
);

// UDS 78 — Cinder Seer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CINDER_SEER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Cinder Seer",
    "d96e7522-e0bc-4e23-8e4b-40a0c28ea986",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// UDS 79 — Colos Yearling
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COLOS_YEARLING: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Colos Yearling",
    "1d68eb62-9f86-4c85-8696-46a248c744ff",
    "Patrick Ho",
    crate::card::CardRules::unsupported(),
);

// UDS 80 — Covetous Dragon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COVETOUS_DRAGON: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Covetous Dragon",
    "c5f37e36-c004-4b89-a668-5cd984c59019",
    "rk post",
    crate::card::CardRules::unsupported(),
);

// UDS 81 — Flame Jet
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLAME_JET: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Flame Jet",
    "a511f9df-b53b-4fea-87cd-9f18f6833f92",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// UDS 82 — Goblin Berserker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_BERSERKER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Goblin Berserker",
    "a3c7635d-98b2-4505-9153-d7e9e53ea16d",
    "Christopher Rush",
    crate::card::CardRules::unsupported(),
);

// UDS 83 — Goblin Festival
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_FESTIVAL: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Goblin Festival",
    "ac067eb8-427f-4bfa-b392-0bb41ac8370e",
    "Jeff Laubenstein",
    crate::card::CardRules::unsupported(),
);

// UDS 84 — Goblin Gardener
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_GARDENER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Goblin Gardener",
    "7eab0544-9c0b-4365-86bb-bc0c3e9d87ce",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// UDS 85 — Goblin Marshal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_MARSHAL: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Goblin Marshal",
    "6a85b2f9-c12c-46dd-ae04-470ebf5ec6d9",
    "DiTerlizzi",
    crate::card::CardRules::unsupported(),
);

// UDS 86 — Goblin Masons
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_MASONS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Goblin Masons",
    "124070d9-c362-4053-a405-9438b1cfac02",
    "DiTerlizzi",
    crate::card::CardRules::unsupported(),
);

// UDS 87 — Hulking Ogre
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HULKING_OGRE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Hulking Ogre",
    "0676d39e-229f-480b-874e-ff0cb8e335d8",
    "Greg Hildebrandt & Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// UDS 88 — Impatience
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IMPATIENCE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Impatience",
    "d39c8166-9e63-4b02-af7b-4caf14ca73ac",
    "Mark Brill",
    crate::card::CardRules::unsupported(),
);

// UDS 89 — Incendiary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INCENDIARY: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Incendiary",
    "854f4775-29b1-4ed1-94d9-db5930a35157",
    "Jeff Laubenstein",
    crate::card::CardRules::unsupported(),
);

// UDS 90 — Keldon Champion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KELDON_CHAMPION: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Keldon Champion",
    "b1eee4d2-fe28-418e-a81f-73a66e831b05",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// UDS 91 — Keldon Vandals
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KELDON_VANDALS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Keldon Vandals",
    "f18cdf4d-42ce-4f2d-8b8f-8cf52a1b8db4",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// UDS 92 — Landslide
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LANDSLIDE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Landslide",
    "c0ddc0dc-8783-4659-bbbd-db6698843b47",
    "Jeff Laubenstein",
    crate::card::CardRules::unsupported(),
);

// UDS 93 — Mark of Fury
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MARK_OF_FURY: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Mark of Fury",
    "5b21b2f4-a05d-477b-8a39-632f7ff7f5f5",
    "Thomas M. Baxa",
    crate::card::CardRules::unsupported(),
);

// UDS 94 — Reckless Abandon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RECKLESS_ABANDON: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Reckless Abandon",
    "8f335d43-cacb-40ad-93c1-9a861e9f66c7",
    "Ron Spears",
    crate::card::CardRules::unsupported(),
);

// UDS 95 — Repercussion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REPERCUSSION: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Repercussion",
    "d0f3c78e-16c0-4fbc-8ef4-fbf610f9d464",
    "Michael Sutfin",
    crate::card::CardRules::unsupported(),
);

// UDS 96 — Scent of Cinder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCENT_OF_CINDER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Scent of Cinder",
    "c030eca0-bc5f-403b-8600-1f295fc85fee",
    "Marc Fishman",
    crate::card::CardRules::unsupported(),
);

// UDS 97 — Sowing Salt
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOWING_SALT: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Sowing Salt",
    "de2f7251-f71a-47d2-a779-c898d94e807c",
    "Todd Lockwood",
    crate::card::CardRules::unsupported(),
);

// UDS 98 — Trumpet Blast
pub(in crate::card::sets) static TRUMPET_BLAST: CardRecord = CardRecord::new(
    CardSet::UrzasDestiny,
    "Trumpet Blast",
    "9fcce5fd-9efb-42fa-9cd6-ed45d8ee46d7",
    "Carl Critchlow",
    CardRules::new_instant(mana_cost!("{2}{R}")).with_ability(AbilityDef::spell(
        "Attacking creatures get +2/+0 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::Attacking,
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(2),
                ValueDef::Constant(0),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// UDS 99 — Wake of Destruction
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WAKE_OF_DESTRUCTION: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Wake of Destruction",
    "0c070f12-0342-48d5-ab0e-4fc4701c3669",
    "Todd Lockwood",
    crate::card::CardRules::unsupported(),
);

// UDS 100 — Wild Colos
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WILD_COLOS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Wild Colos",
    "2d39f746-7b82-476a-9774-3375debb47bd",
    "Marc Fishman",
    crate::card::CardRules::unsupported(),
);

// UDS 101 — Ancient Silverback
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANCIENT_SILVERBACK: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Ancient Silverback",
    "49651dd4-a489-42d3-b4eb-51f5353b334e",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// UDS 102 — Compost
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COMPOST: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Compost",
    "2523c403-0025-48c7-8ff1-e66ca27ee585",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// UDS 103 — Elvish Lookout
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELVISH_LOOKOUT: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Elvish Lookout",
    "d9a8a0e2-311a-4627-8a48-43df045c3112",
    "Greg Hildebrandt & Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// UDS 104 — Elvish Piper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELVISH_PIPER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Elvish Piper",
    "55e76333-0959-4572-a1ca-d77f76da1279",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// UDS 105 — Emperor Crocodile
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EMPEROR_CROCODILE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Emperor Crocodile",
    "9ccba208-1e24-45bb-a556-a3eb936efb10",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// UDS 106 — Gamekeeper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GAMEKEEPER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Gamekeeper",
    "b6006c21-28b5-4550-8b4e-ac631f39cdf7",
    "Scott Hampton",
    crate::card::CardRules::unsupported(),
);

// UDS 107 — Goliath Beetle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOLIATH_BEETLE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Goliath Beetle",
    "f83d8765-f654-4837-9b06-739610188415",
    "Don Hazeltine",
    crate::card::CardRules::unsupported(),
);

// UDS 108 — Heart Warden
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEART_WARDEN: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Heart Warden",
    "96e42dbe-3eeb-4367-bb6d-0f5c71f5da80",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// UDS 109 — Hunting Moa
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HUNTING_MOA: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Hunting Moa",
    "926cefa1-3c5c-4bd6-859b-de620a3ee777",
    "DiTerlizzi",
    crate::card::CardRules::unsupported(),
);

// UDS 110 — Ivy Seer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IVY_SEER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Ivy Seer",
    "018ad11c-1351-4eff-94ac-3926037d7247",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// UDS 111 — Magnify
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAGNIFY: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Magnify",
    "7b9bb2c6-f1a6-42c3-a7cb-3a1a46854c9b",
    "Michael Sutfin",
    crate::card::CardRules::unsupported(),
);

// UDS 112 — Marker Beetles
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MARKER_BEETLES: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Marker Beetles",
    "5cbd3c78-197a-40b9-94d1-bbb1ec1e64b1",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// UDS 113 — Momentum
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOMENTUM: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Momentum",
    "10bd11a2-7cab-4d3f-b52b-f5bb66fbbec6",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// UDS 114 — Multani's Decree
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MULTANI_S_DECREE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Multani's Decree",
    "58b4d5c8-23fc-4fb8-99d6-bb64e66cc4db",
    "Eric Peterson",
    crate::card::CardRules::unsupported(),
);

// UDS 115 — Pattern of Rebirth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PATTERN_OF_REBIRTH: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Pattern of Rebirth",
    "9f23c4f4-a191-4225-a3b7-dab5b1462922",
    "Mark Brill",
    crate::card::CardRules::unsupported(),
);

// UDS 116 — Plated Spider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PLATED_SPIDER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Plated Spider",
    "3529f49b-7e5e-4fa8-a03d-a94877761525",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// UDS 117 — Plow Under
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PLOW_UNDER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Plow Under",
    "a30735c4-7f12-4db9-972b-9b7568a8ada8",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// UDS 118 — Rofellos, Llanowar Emissary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROFELLOS_LLANOWAR_EMISSARY: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Rofellos, Llanowar Emissary",
    "6aa5cc65-f8f1-4f6f-8b4e-2fedccbda684",
    "Michael Sutfin",
    crate::card::CardRules::unsupported(),
);

// UDS 119 — Rofellos's Gift
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROFELLOS_S_GIFT: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Rofellos's Gift",
    "a41347ba-b2e3-4d7e-8018-e6fd30243559",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// UDS 120 — Scent of Ivy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCENT_OF_IVY: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Scent of Ivy",
    "a56b4894-b959-4d00-b631-95d26eb85a4e",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// UDS 121 — Splinter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPLINTER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Splinter",
    "eb32175c-f2e6-460b-b4bf-dd85cac3eb4f",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// UDS 122 — Taunting Elf
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TAUNTING_ELF: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Taunting Elf",
    "85bfa6b9-c898-4bb6-a444-6cf336bfb260",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// UDS 123 — Thorn Elemental
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THORN_ELEMENTAL: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Thorn Elemental",
    "971d4b0d-fe3e-46f5-86df-3fbac6b900b0",
    "rk post",
    crate::card::CardRules::unsupported(),
);

// UDS 124 — Yavimaya Elder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static YAVIMAYA_ELDER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Yavimaya Elder",
    "325d9372-01c9-4e99-a966-13c8f8566e2e",
    "Ray Lago",
    crate::card::CardRules::unsupported(),
);

// UDS 125 — Yavimaya Enchantress
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static YAVIMAYA_ENCHANTRESS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Yavimaya Enchantress",
    "c9e3934e-6169-416e-92bb-359e41900c3b",
    "Matthew D. Wilson",
    crate::card::CardRules::unsupported(),
);

// UDS 126 — Braidwood Cup
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRAIDWOOD_CUP: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Braidwood Cup",
    "c2e783b7-9bd1-4f82-bf20-5d201413f5e8",
    "Greg Hildebrandt & Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// UDS 127 — Braidwood Sextant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRAIDWOOD_SEXTANT: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Braidwood Sextant",
    "16dc7634-8ef5-4a03-8276-7e1dae4244c2",
    "Don Hazeltine",
    crate::card::CardRules::unsupported(),
);

// UDS 128 — Brass Secretary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRASS_SECRETARY: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Brass Secretary",
    "c5685cff-b607-4a81-aa47-6676ab1a5782",
    "DiTerlizzi",
    crate::card::CardRules::unsupported(),
);

// UDS 129 — Caltrops
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CALTROPS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Caltrops",
    "a9cf74e4-31d2-4cd2-8f3a-b2141301f686",
    "Jeff Laubenstein",
    crate::card::CardRules::unsupported(),
);

// UDS 130 — Extruder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXTRUDER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Extruder",
    "2fc2f0d0-273d-428f-9a8a-c582f4d16394",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// UDS 131 — Fodder Cannon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FODDER_CANNON: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Fodder Cannon",
    "229ba320-69c9-4400-a0d7-f0f79e8d9856",
    "DiTerlizzi",
    crate::card::CardRules::unsupported(),
);

// UDS 132 — Junk Diver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JUNK_DIVER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Junk Diver",
    "c4f5a9d8-80b9-4765-adb2-10d53baaacb0",
    "Eric Peterson",
    crate::card::CardRules::unsupported(),
);

// UDS 133 — Mantis Engine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MANTIS_ENGINE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Mantis Engine",
    "97f7ebfb-f955-4849-a0f5-6806ff6ae891",
    "John Zeleznik",
    crate::card::CardRules::unsupported(),
);

// UDS 134 — Masticore
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MASTICORE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Masticore",
    "908a2215-7231-43a4-8fec-5d1e4233c028",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// UDS 135 — Metalworker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static METALWORKER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Metalworker",
    "2050d414-71c7-4c42-a1ff-4c04068ba7f2",
    "Don Hazeltine",
    crate::card::CardRules::unsupported(),
);

// UDS 136 — Powder Keg
pub(in crate::card::sets) static POWDER_KEG: CardRecord = CardRecord::new(
    CardSet::UrzasDestiny,
    "Powder Keg",
    "4d9715c2-9036-4ae2-a5b4-1b190d50c963",
    "Dan Frazier",
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep, you may put a fuse counter on this artifact.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            // The counter is optional, so the Keg can be held at whatever size the board
            // calls for rather than ticking past it.
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::named("fuse"),
                    amount: ValueDef::Constant(1),
                },
            },
        ),
        AbilityDef::activated(
            "{T}, Sacrifice this artifact: Destroy each artifact and creature with mana value equal to the number of fuse counters on this artifact.",
            &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
            EffectDef::Destroy {
                object: EffectRecipientDef::matching_objects(
                    // Everything the fuse counters name. A Keg with no counters on it destroys
                    // every nothing-cost permanent, which is the mode that answers a board of
                    // tokens.
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Creature),
                        ]),
                        ObjectPredicateDef::ManaValueEqualTo(ValueDef::CountersOnSource(CounterKind::named("fuse"))),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                can_regenerate: true,
                then: None,
            },
        ),
    ]),
);

// UDS 137 — Scrying Glass
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCRYING_GLASS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Scrying Glass",
    "7286819f-6c57-4503-898c-528786ad86e9",
    "Patrick Ho",
    crate::card::CardRules::unsupported(),
);

// UDS 138 — Storage Matrix
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STORAGE_MATRIX: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Storage Matrix",
    "77378279-024c-4c36-b5bf-6294fe5c32f5",
    "Patrick Ho",
    crate::card::CardRules::unsupported(),
);

// UDS 139 — Thran Dynamo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THRAN_DYNAMO: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Thran Dynamo",
    "3f0c9dee-fab5-4522-9821-343f84b0c8ab",
    "Ron Spears",
    crate::card::CardRules::unsupported(),
);

// UDS 140 — Thran Foundry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THRAN_FOUNDRY: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Thran Foundry",
    "9cdc0d42-d96f-490f-87cb-3577dfdce807",
    "John Zeleznik",
    crate::card::CardRules::unsupported(),
);

// UDS 141 — Thran Golem
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THRAN_GOLEM: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Thran Golem",
    "5778c52b-248b-4131-b5c0-12ea1986786e",
    "Ron Spears",
    crate::card::CardRules::unsupported(),
);

// UDS 142 — Urza's Incubator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static URZA_S_INCUBATOR: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Urza's Incubator",
    "bdf96c2c-b3d6-4d84-9572-fb115a795bed",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// UDS 143 — Yavimaya Hollow
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static YAVIMAYA_HOLLOW: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasDestiny,
    "Yavimaya Hollow",
    "47dd5c4b-5972-43e1-ae2a-ebf275006458",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ACADEMY_RECTOR,
    &ARCHERY_TRAINING,
    &CAPASHEN_KNIGHT,
    &CAPASHEN_STANDARD,
    &CAPASHEN_TEMPLAR,
    &FALSE_PROPHET,
    &FEND_OFF,
    &FIELD_SURGEON,
    &FLICKER,
    &JASMINE_SEER,
    &MASK_OF_LAW_AND_GRACE,
    &MASTER_HEALER,
    &OPALESCENCE,
    &RELIQUARY_MONK,
    &REPLENISH,
    &SANCTIMONY,
    &SCENT_OF_JASMINE,
    &SCOUR,
    &SERRA_ADVOCATE,
    &SOLIDARITY,
    &TETHERED_GRIFFIN,
    &TORMENTED_ANGEL,
    &VOICE_OF_DUTY,
    &VOICE_OF_REASON,
    &WALL_OF_GLARE,
    &AURA_THIEF,
    &BLIZZARD_ELEMENTAL,
    &BRINE_SEER,
    &BUBBLING_BEEBLES,
    &DISAPPEAR,
    &DONATE,
    &FATIGUE,
    &FLEDGLING_OSPREY,
    &ILLUMINATED_WINGS,
    &IRIDESCENT_DRAKE,
    &KINGFISHER,
    &MENTAL_DISCIPLINE,
    &METATHRAN_ELITE,
    &METATHRAN_SOLDIER,
    &OPPOSITION,
    &PRIVATE_RESEARCH,
    &QUASH,
    &RAYNE_ACADEMY_CHANCELLOR,
    &RESCUE,
    &SCENT_OF_BRINE,
    &SIGIL_OF_SLEEP,
    &TELEPATHIC_SPIES,
    &TEMPORAL_ADEPT,
    &THIEVING_MAGPIE,
    &TREACHERY,
    &APPRENTICE_NECROMANCER,
    &ATTRITION,
    &BODY_SNATCHER,
    &BUBBLING_MUCK,
    &CARNIVAL_OF_SOULS,
    &CHIME_OF_NIGHT,
    &DISEASE_CARRIERS,
    &DYING_WAIL,
    &ENCROACH,
    &ERADICATE,
    &FESTERING_WOUND,
    &LURKING_JACKALS,
    &NIGHTSHADE_SEER,
    &PHYREXIAN_MONITOR,
    &PHYREXIAN_NEGATOR,
    &PLAGUE_DOGS,
    &RAPID_DECAY,
    &SCENT_OF_NIGHTSHADE,
    &SKITTERING_HORROR,
    &SLINKING_SKIRGE,
    &SOUL_FEAST,
    &SQUIRMING_MASS,
    &TWISTED_EXPERIMENT,
    &YAWGMOTH_S_BARGAIN,
    &AETHER_STING,
    &BLOODSHOT_CYCLOPS,
    &CINDER_SEER,
    &COLOS_YEARLING,
    &COVETOUS_DRAGON,
    &FLAME_JET,
    &GOBLIN_BERSERKER,
    &GOBLIN_FESTIVAL,
    &GOBLIN_GARDENER,
    &GOBLIN_MARSHAL,
    &GOBLIN_MASONS,
    &HULKING_OGRE,
    &IMPATIENCE,
    &INCENDIARY,
    &KELDON_CHAMPION,
    &KELDON_VANDALS,
    &LANDSLIDE,
    &MARK_OF_FURY,
    &RECKLESS_ABANDON,
    &REPERCUSSION,
    &SCENT_OF_CINDER,
    &SOWING_SALT,
    &TRUMPET_BLAST,
    &WAKE_OF_DESTRUCTION,
    &WILD_COLOS,
    &ANCIENT_SILVERBACK,
    &COMPOST,
    &ELVISH_LOOKOUT,
    &ELVISH_PIPER,
    &EMPEROR_CROCODILE,
    &GAMEKEEPER,
    &GOLIATH_BEETLE,
    &HEART_WARDEN,
    &HUNTING_MOA,
    &IVY_SEER,
    &MAGNIFY,
    &MARKER_BEETLES,
    &MOMENTUM,
    &MULTANI_S_DECREE,
    &PATTERN_OF_REBIRTH,
    &PLATED_SPIDER,
    &PLOW_UNDER,
    &ROFELLOS_LLANOWAR_EMISSARY,
    &ROFELLOS_S_GIFT,
    &SCENT_OF_IVY,
    &SPLINTER,
    &TAUNTING_ELF,
    &THORN_ELEMENTAL,
    &YAVIMAYA_ELDER,
    &YAVIMAYA_ENCHANTRESS,
    &BRAIDWOOD_CUP,
    &BRAIDWOOD_SEXTANT,
    &BRASS_SECRETARY,
    &CALTROPS,
    &EXTRUDER,
    &FODDER_CANNON,
    &JUNK_DIVER,
    &MANTIS_ENGINE,
    &MASTICORE,
    &METALWORKER,
    &POWDER_KEG,
    &SCRYING_GLASS,
    &STORAGE_MATRIX,
    &THRAN_DYNAMO,
    &THRAN_FOUNDRY,
    &THRAN_GOLEM,
    &URZA_S_INCUBATOR,
    &YAVIMAYA_HOLLOW,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[RAVENOUS_RATS_REPRINT];
