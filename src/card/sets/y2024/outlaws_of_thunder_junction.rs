//! Outlaws of Thunder Junction card inventory.

use super::CardRecord;
use super::PrintingRecord;
use crate::ParentBinding;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AggregateOperationDef;
use crate::card::AppliedEffectDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::ChangeStackTargetsDef;
use crate::card::CopyStackObjectDef;
use crate::card::CostDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::DiscardSelectionDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::MoveObjectsDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectSetDef;
use crate::card::ObjectValueAggregateDef;
use crate::card::ObjectValueDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::ResolvedEffectDurationDef;
use crate::card::RevealObjectsDef;
use crate::card::ScaledValueDef;
use crate::card::StackTargetChangeDef;
use crate::card::SubtypeDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TokenStatsDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y2015::magic_origins as catalog_ori;
use crate::card::sets::y2016::kaladesh as catalog_kld;
use crate::card::sets::y2017::ixalan as catalog_xln;
use crate::card::sets::y2020::core_set_2021 as catalog_m21;
use crate::card::sets::y2021::kaldheim as catalog_khm;
use crate::card::sets::y2022::dominaria_united as catalog_dmu;
use crate::card::sets::y2022::streets_of_new_capenna as catalog_snc;
use crate::card::sets::y2023::march_of_the_machine as catalog_mom;

pub const SPREE: crate::card::MechanicId = crate::card::MechanicId::from_name("mtg:spree");

/// Choose one or more modes and pay the additional costs of the chosen modes.
///
/// # Panics
///
/// Panics if the mode list is empty or contains more than 255 modes.
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub const fn spree(modes: &'static [(&'static [CostDef], AbilityDef)]) -> AbilityDef {
    assert!(!modes.is_empty() && modes.len() <= u8::MAX as usize);
    AbilityDef::defined(
        "Spree (Choose one or more additional costs.)",
        crate::card::DeclarativeAbilityDef::Spell(crate::card::SpellAbilityDef::Modal(
            crate::card::ModalSpellDef::with_costed_modes(modes, 1, modes.len() as u8, false),
        )),
        EffectDef::None,
    )
    .labeled(SPREE)
}

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "OTJ",
    slug: "outlaws-of-thunder-junction",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// OTJ 1 — Another Round
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANOTHER_ROUND: CardRecord = CardRecord::new(
    "Another Round",
    "4f8dc511-e307-4412-bb79-375a6077312d",
    "Darrell Riche",
    crate::card::CardRules::unsupported(),
);

// OTJ 2 — Archangel of Tithes (reprint)
const ARCHANGEL_OF_TITHES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ori::ARCHANGEL_OF_TITHES,
    "c853d04c-864b-491c-8c6f-72d2d4874d2f",
    "Denys Tsiperko",
);

// OTJ 3 — Armored Armadillo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARMORED_ARMADILLO: CardRecord = CardRecord::new(
    "Armored Armadillo",
    "263232df-69b8-4205-93ad-c724fe57ec11",
    "Leon Tukker",
    crate::card::CardRules::unsupported(),
);

// OTJ 4 — Aven Interrupter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVEN_INTERRUPTER: CardRecord = CardRecord::new(
    "Aven Interrupter",
    "d3ca43a4-d194-440f-8099-f1fa103a108d",
    "Daniel Romanovsky",
    crate::card::CardRules::unsupported(),
);

// OTJ 5 — Bounding Felidar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOUNDING_FELIDAR: CardRecord = CardRecord::new(
    "Bounding Felidar",
    "8925279f-c16a-43b4-b791-ce450157275b",
    "Lars Grant-West",
    crate::card::CardRules::unsupported(),
);

// OTJ 6 — Bovine Intervention
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOVINE_INTERVENTION: CardRecord = CardRecord::new(
    "Bovine Intervention",
    "26c36742-456f-4618-99bc-793ef20b31b0",
    "Julia Metzger",
    crate::card::CardRules::unsupported(),
);

// OTJ 7 — Bridled Bighorn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRIDLED_BIGHORN: CardRecord = CardRecord::new(
    "Bridled Bighorn",
    "fa7bf089-fa9b-4ffc-bf84-45cd51c76463",
    "Edgar Sánchez Hidalgo",
    crate::card::CardRules::unsupported(),
);

// OTJ 8 — Claim Jumper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLAIM_JUMPER: CardRecord = CardRecord::new(
    "Claim Jumper",
    "654ade6b-0369-4b90-a744-2f57a45b04f4",
    "Gaboleps",
    crate::card::CardRules::unsupported(),
);

// OTJ 9 — Dust Animus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DUST_ANIMUS: CardRecord = CardRecord::new(
    "Dust Animus",
    "70719e7b-6f02-4c1a-9f11-79b2b0d9846a",
    "Uriah Voth",
    crate::card::CardRules::unsupported(),
);

// OTJ 10 — Eriette's Lullaby
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ERIETTE_S_LULLABY: CardRecord = CardRecord::new(
    "Eriette's Lullaby",
    "184c18b6-40e2-4f7f-a3bb-49bc695b68ec",
    "Anton Solovianchyk",
    crate::card::CardRules::unsupported(),
);

// OTJ 11 — Final Showdown
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FINAL_SHOWDOWN: CardRecord = CardRecord::new(
    "Final Showdown",
    "358968f9-45bd-4022-b6bc-f1f7e0adf0e7",
    "Izzy",
    crate::card::CardRules::unsupported(),
);

// OTJ 12 — Fortune, Loyal Steed
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORTUNE_LOYAL_STEED: CardRecord = CardRecord::new(
    "Fortune, Loyal Steed",
    "069294a8-e65a-47af-942f-7e99d18658f2",
    "Artur Nakhodkin",
    crate::card::CardRules::unsupported(),
);

// OTJ 13 — Frontier Seeker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FRONTIER_SEEKER: CardRecord = CardRecord::new(
    "Frontier Seeker",
    "9368cc76-bee5-4b46-a309-981106c3addf",
    "Raluca Marinescu",
    crate::card::CardRules::unsupported(),
);

// OTJ 14 — Getaway Glamer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GETAWAY_GLAMER: CardRecord = CardRecord::new(
    "Getaway Glamer",
    "69689049-a704-4f16-84ee-4d5d915028ec",
    "Forrest Imel",
    crate::card::CardRules::unsupported(),
);

// OTJ 15 — High Noon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HIGH_NOON: CardRecord = CardRecord::new(
    "High Noon",
    "9995e0e6-7c9c-4fef-8fd2-8fb1622e6ec8",
    "Eduardo Francisco",
    crate::card::CardRules::unsupported(),
);

// OTJ 16 — Holy Cow
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HOLY_COW: CardRecord = CardRecord::new(
    "Holy Cow",
    "90de84c9-941b-4056-8501-ce8a948b9643",
    "Justyna Dura",
    crate::card::CardRules::unsupported(),
);

// OTJ 17 — Inventive Wingsmith
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INVENTIVE_WINGSMITH: CardRecord = CardRecord::new(
    "Inventive Wingsmith",
    "b6b36bb3-dacc-44f6-adcd-2c2d65513d8c",
    "David Astruga",
    crate::card::CardRules::unsupported(),
);

// OTJ 18 — Lassoed by the Law
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LASSOED_BY_THE_LAW: CardRecord = CardRecord::new(
    "Lassoed by the Law",
    "ea96eeac-c316-4247-a81f-0ddf52675ebf",
    "Leanna Crossan",
    crate::card::CardRules::unsupported(),
);

// OTJ 19 — Mystical Tether
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MYSTICAL_TETHER: CardRecord = CardRecord::new(
    "Mystical Tether",
    "18344498-952e-489c-8b03-bd1bef4c26ca",
    "Adam Volker",
    crate::card::CardRules::unsupported(),
);

// OTJ 20 — Nurturing Pixie
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NURTURING_PIXIE: CardRecord = CardRecord::new(
    "Nurturing Pixie",
    "0fe155f6-888b-41a0-a9a0-be7bea998718",
    "Iris Compiet",
    crate::card::CardRules::unsupported(),
);

// OTJ 21 — Omenport Vigilante
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OMENPORT_VIGILANTE: CardRecord = CardRecord::new(
    "Omenport Vigilante",
    "7ecd8b6f-b9aa-466a-909c-3209beef8244",
    "Forrest Imel",
    crate::card::CardRules::unsupported(),
);

// OTJ 22 — One Last Job
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ONE_LAST_JOB: CardRecord = CardRecord::new(
    "One Last Job",
    "71bfbfae-e7eb-4f80-81c6-9ab6a1bbd39d",
    "Caroline Gariba",
    crate::card::CardRules::unsupported(),
);

// OTJ 23 — Outlaw Medic
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OUTLAW_MEDIC: CardRecord = CardRecord::new(
    "Outlaw Medic",
    "2feaa51e-47fb-4849-b420-ee7278f3489a",
    "Nino Vecia",
    crate::card::CardRules::unsupported(),
);

// OTJ 24 — Prairie Dog
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PRAIRIE_DOG: CardRecord = CardRecord::new(
    "Prairie Dog",
    "37302b5d-e528-4baa-947a-c859e4ddcff9",
    "Kevin Sidharta",
    crate::card::CardRules::unsupported(),
);

// OTJ 25 — Prosperity Tycoon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PROSPERITY_TYCOON: CardRecord = CardRecord::new(
    "Prosperity Tycoon",
    "f87824f3-aa9f-4d3d-99f7-0fbca43d8a51",
    "Caio Monteiro",
    crate::card::CardRules::unsupported(),
);

// OTJ 26 — Requisition Raid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REQUISITION_RAID: CardRecord = CardRecord::new(
    "Requisition Raid",
    "154e9ba9-0d0e-4b0e-acf2-f66a993cf3a2",
    "Viko Menezes",
    crate::card::CardRules::unsupported(),
);

// OTJ 27 — Rustler Rampage
pub(in crate::card::sets) static RUSTLER_RAMPAGE: CardRecord = CardRecord::new(
    "Rustler Rampage",
    "33ed7ca3-894b-45f4-a15f-51b6bcd3f474",
    "Josu Hernaiz",
    CardRules::new_instant(mana_cost!("{W}")).with_ability(spree(&[
        (
            &[CostDef::Mana(mana_cost!("{1}"))],
            AbilityDef::spell_with_targets(
                "Untap all creatures target player controls.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::Untap {
                    object: EffectRecipientDef::objects(ObjectSetDef::PermanentsControlledBy(
                        PlayerRefDef::Target(TargetIndex::PRIMARY),
                    )),
                },
            ),
        ),
        (
            &[CostDef::Mana(mana_cost!("{1}"))],
            AbilityDef::spell_with_targets(
                "Target creature gains double strike until end of turn.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&abilities::double_strike()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ),
    ])),
);

// OTJ 28 — Shepherd of the Clouds
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHEPHERD_OF_THE_CLOUDS: CardRecord = CardRecord::new(
    "Shepherd of the Clouds",
    "c2245f36-2138-4f01-9b70-151137a5ac59",
    "Valera Lutfullina",
    crate::card::CardRules::unsupported(),
);

// OTJ 29 — Sheriff of Safe Passage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHERIFF_OF_SAFE_PASSAGE: CardRecord = CardRecord::new(
    "Sheriff of Safe Passage",
    "c38a845d-f25d-45ab-9154-3fa5291b0ba0",
    "Brian Valeza",
    crate::card::CardRules::unsupported(),
);

// OTJ 30 — Stagecoach Security
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STAGECOACH_SECURITY: CardRecord = CardRecord::new(
    "Stagecoach Security",
    "21301998-b3e6-4f3d-89f4-5e17aeb79a1e",
    "Nino Vecia",
    crate::card::CardRules::unsupported(),
);

// OTJ 31 — Steer Clear
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STEER_CLEAR: CardRecord = CardRecord::new(
    "Steer Clear",
    "523a4d6e-122b-49b4-bf3d-17d29c0007fb",
    "Edgar Sánchez Hidalgo",
    crate::card::CardRules::unsupported(),
);

// OTJ 32 — Sterling Keykeeper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STERLING_KEYKEEPER: CardRecord = CardRecord::new(
    "Sterling Keykeeper",
    "019d539f-04c2-43f1-8677-6d6fbb0e94f7",
    "David Astruga",
    crate::card::CardRules::unsupported(),
);

// OTJ 33 — Sterling Supplier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STERLING_SUPPLIER: CardRecord = CardRecord::new(
    "Sterling Supplier",
    "b6000be7-67db-440e-87ba-276df20b803e",
    "Camille Alquier",
    crate::card::CardRules::unsupported(),
);

// OTJ 34 — Take Up the Shield (reprint)
const TAKE_UP_THE_SHIELD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dmu::TAKE_UP_THE_SHIELD,
    "76a31968-ba6d-4c01-838f-4cb8c64e73fb",
    "Josiah \"Jo\" Cameron",
);

// OTJ 35 — Thunder Lasso
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THUNDER_LASSO: CardRecord = CardRecord::new(
    "Thunder Lasso",
    "73dff5fc-2adf-447a-b35f-e7883e0fd821",
    "Camille Alquier",
    crate::card::CardRules::unsupported(),
);

// OTJ 36 — Trained Arynx
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRAINED_ARYNX: CardRecord = CardRecord::new(
    "Trained Arynx",
    "ef32a5f8-f69d-47dc-a800-4f0ddf4eada5",
    "Milivoj Ćeran",
    crate::card::CardRules::unsupported(),
);

// OTJ 37 — Vengeful Townsfolk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VENGEFUL_TOWNSFOLK: CardRecord = CardRecord::new(
    "Vengeful Townsfolk",
    "2df404af-571a-4867-83f5-bb4163b433ff",
    "Irina Nordsol",
    crate::card::CardRules::unsupported(),
);

// OTJ 38 — Wanted Griffin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WANTED_GRIFFIN: CardRecord = CardRecord::new(
    "Wanted Griffin",
    "624a176b-fe24-4441-8877-464cf172ccff",
    "Alexandre Honoré",
    crate::card::CardRules::unsupported(),
);

// OTJ 39 — Archmage's Newt
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARCHMAGE_S_NEWT: CardRecord = CardRecord::new(
    "Archmage's Newt",
    "a440bbd6-8e51-4db1-90d9-7fa9fc327ad5",
    "Edgar Sánchez Hidalgo",
    crate::card::CardRules::unsupported(),
);

// OTJ 40 — Canyon Crab
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CANYON_CRAB: CardRecord = CardRecord::new(
    "Canyon Crab",
    "b740a8a8-e1d3-4642-a214-03731c9b5553",
    "Ignatius Budi",
    crate::card::CardRules::unsupported(),
);

// OTJ 41 — Daring Thunder-Thief
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARING_THUNDER_THIEF: CardRecord = CardRecord::new(
    "Daring Thunder-Thief",
    "b41a2bf7-248c-4f8b-92ec-3010d276cb59",
    "Inkognit",
    crate::card::CardRules::unsupported(),
);

// OTJ 42 — Deepmuck Desperado
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEEPMUCK_DESPERADO: CardRecord = CardRecord::new(
    "Deepmuck Desperado",
    "f95ee726-7465-40f8-a954-19b19e636c12",
    "Loïc Canavaggia",
    crate::card::CardRules::unsupported(),
);

// OTJ 43 — Djinn of Fool's Fall
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DJINN_OF_FOOL_S_FALL: CardRecord = CardRecord::new(
    "Djinn of Fool's Fall",
    "48bfc6af-c651-485e-a1cc-f9d00aeaf812",
    "Inkognit",
    crate::card::CardRules::unsupported(),
);

// OTJ 44 — Double Down
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DOUBLE_DOWN: CardRecord = CardRecord::new(
    "Double Down",
    "8ecccdf3-98c6-4aed-8757-913623efb677",
    "Javier Charro",
    crate::card::CardRules::unsupported(),
);

// OTJ 45 — Duelist of the Mind
pub(in crate::card::sets) static DUELIST_OF_THE_MIND: CardRecord = CardRecord::new(
    "Duelist of the Mind",
    "2b58e47b-c165-4a58-aa2a-033a35645adc",
    "Darren Tan",
// A 0/3 flier that grows with every draw and feeds itself once a turn,
    // provided you point something at your opponent.
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Human", "Advisor"], 0, 3)
        .with_abilities(&[
            abilities::flying(),
            abilities::vigilance(),
            AbilityDef::static_ability(
                "Duelist of the Mind's power is equal to the number of cards you've drawn this turn.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    // The count defines her power outright, which is why it also
                    // answers in a hand or a graveyard; the printed toughness is
                    // left alone.
                    effect: AppliedEffectDef::define_power(ValueDef::CardsDrawnThisTurn(
                        PlayerRelation::You,
                    )),
                },
            ),
            AbilityDef::triggered(
                "Whenever you commit a crime, you may draw a card. If you do, discard a card. This ability triggers only once each turn.",
                TriggerEventDef::CommittedCrime(PlayerRelation::You),
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    // "Draw a card. If you do, discard a card." A draw from an empty library
                    // does not happen, so the discard is conditional on the draw rather than
                    // sequenced after it.
                    effect: &EffectDef::Sequence(&[
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
                },
            )
            .triggering_at_most(1),
        ]),
);

// OTJ 46 — Emergent Haunting
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EMERGENT_HAUNTING: CardRecord = CardRecord::new(
    "Emergent Haunting",
    "623053a8-7abe-45ec-9f26-97e1c037120b",
    "Jorge Jacinto",
    crate::card::CardRules::unsupported(),
);

// OTJ 47 — Failed Fording
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FAILED_FORDING: CardRecord = CardRecord::new(
    "Failed Fording",
    "62bbe11b-e959-4080-98ac-09bd57519c00",
    "José Parodi",
    crate::card::CardRules::unsupported(),
);

// OTJ 48 — Fblthp, Lost on the Range
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FBLTHP_LOST_ON_THE_RANGE: CardRecord = CardRecord::new(
    "Fblthp, Lost on the Range",
    "01d3e6ea-4791-4948-af22-c1bd04c34c1e",
    "Brian Valeza",
    crate::card::CardRules::unsupported(),
);

// OTJ 49 — Fleeting Reflection
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLEETING_REFLECTION: CardRecord = CardRecord::new(
    "Fleeting Reflection",
    "8f8c931e-219f-4032-b55b-b5975fbea1e7",
    "Camille Alquier",
    crate::card::CardRules::unsupported(),
);

// OTJ 50 — Geralf, the Fleshwright
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GERALF_THE_FLESHWRIGHT: CardRecord = CardRecord::new(
    "Geralf, the Fleshwright",
    "afe3b678-b340-4c53-bbf6-19252a809d73",
    "Chris Rahn",
    crate::card::CardRules::unsupported(),
);

// OTJ 51 — Geyser Drake
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GEYSER_DRAKE: CardRecord = CardRecord::new(
    "Geyser Drake",
    "b270377b-33eb-4e5e-9d14-0da2876da74f",
    "Daniel Romanovsky",
    crate::card::CardRules::unsupported(),
);

// OTJ 52 — Harrier Strix
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HARRIER_STRIX: CardRecord = CardRecord::new(
    "Harrier Strix",
    "70ca61a9-8938-41bf-bd14-5759f4de6521",
    "Brian Valeza",
    crate::card::CardRules::unsupported(),
);

// OTJ 53 — Jailbreak Scheme
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JAILBREAK_SCHEME: CardRecord = CardRecord::new(
    "Jailbreak Scheme",
    "a4be8e47-9006-4770-8d99-68a684064a43",
    "Inkognit",
    crate::card::CardRules::unsupported(),
);

// OTJ 54 — The Key to the Vault
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_KEY_TO_THE_VAULT: CardRecord = CardRecord::new(
    "The Key to the Vault",
    "166814af-a444-4a62-937e-7491673d9387",
    "Leon Tukker",
    crate::card::CardRules::unsupported(),
);

// OTJ 55 — Loan Shark
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LOAN_SHARK: CardRecord = CardRecord::new(
    "Loan Shark",
    "49f12760-ff07-4c9f-a7a9-1e64bd3a9adf",
    "Wayne Reynolds",
    crate::card::CardRules::unsupported(),
);

// OTJ 56 — Marauding Sphinx
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MARAUDING_SPHINX: CardRecord = CardRecord::new(
    "Marauding Sphinx",
    "34071884-c5b6-42c0-9eb3-9f32910c29d8",
    "Mila Pesic",
    crate::card::CardRules::unsupported(),
);

// OTJ 57 — Metamorphic Blast
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static METAMORPHIC_BLAST: CardRecord = CardRecord::new(
    "Metamorphic Blast",
    "bd38d922-cfc1-43a8-82d8-5de441c71076",
    "Michal Ivan",
    crate::card::CardRules::unsupported(),
);

// OTJ 58 — Nimble Brigand
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NIMBLE_BRIGAND: CardRecord = CardRecord::new(
    "Nimble Brigand",
    "73c74d48-362d-4c3b-9ff7-39bdd19657a6",
    "Kim Sokol",
    crate::card::CardRules::unsupported(),
);

// OTJ 59 — Outlaw Stitcher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OUTLAW_STITCHER: CardRecord = CardRecord::new(
    "Outlaw Stitcher",
    "ba9584f0-55b8-448d-99a7-041934053f42",
    "Alix Branwyn",
    crate::card::CardRules::unsupported(),
);

// OTJ 60 — Peerless Ropemaster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PEERLESS_ROPEMASTER: CardRecord = CardRecord::new(
    "Peerless Ropemaster",
    "ae044509-2f31-4506-a124-0e445b7181a2",
    "Wayne Wu",
    crate::card::CardRules::unsupported(),
);

// OTJ 61 — Phantom Interference
pub(in crate::card::sets) static PHANTOM_INTERFERENCE: CardRecord = CardRecord::new(
    "Phantom Interference",
    "00bf4dd1-5468-4594-9c7b-0737610f19d4",
    "Ruxing Gao",
    // Two mana to counter, four to do both, and never dead: spree is what
    // lets one card be a Spirit on the turn nothing needs answering.
    CardRules::new_instant(mana_cost!("{U}")).with_ability(spree(&[
        (
            &[CostDef::Mana(mana_cost!("{3}"))],
            AbilityDef::spell(
                "Create a 2/2 white Spirit creature token with flying.",
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                    TokenCharacteristics::creature(&["Spirit"], &[ManaColor::White], 2, 2)
                        .with_abilities(&[abilities::flying()]),
                ))),
            ),
        ),
        (
            &[CostDef::Mana(mana_cost!("{1}"))],
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
                abilities::counter_target_unless_paid(&[CostDef::GenericMana(ValueDef::Constant(
                    2,
                ))]),
            ),
        ),
    ])),
);

// OTJ 62 — Plan the Heist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PLAN_THE_HEIST: CardRecord = CardRecord::new(
    "Plan the Heist",
    "1e8c3a0e-d61c-457f-ac85-577d0bb94b96",
    "Fariba Khamseh",
    crate::card::CardRules::unsupported(),
);

// OTJ 63 — Razzle-Dazzler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAZZLE_DAZZLER: CardRecord = CardRecord::new(
    "Razzle-Dazzler",
    "f7d08008-9272-405e-82ef-566e6d42bb17",
    "Wayne Wu",
    crate::card::CardRules::unsupported(),
);

// OTJ 64 — Seize the Secrets
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEIZE_THE_SECRETS: CardRecord = CardRecord::new(
    "Seize the Secrets",
    "1bdbdfa8-aa28-4b3d-95e7-3d0e7e37f982",
    "Miranda Meeks",
    crate::card::CardRules::unsupported(),
);

// OTJ 65 — Shackle Slinger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHACKLE_SLINGER: CardRecord = CardRecord::new(
    "Shackle Slinger",
    "e6cfe383-e483-47e7-99d1-991a06b089bc",
    "Josh Hass",
    crate::card::CardRules::unsupported(),
);

// OTJ 66 — Shifting Grift
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHIFTING_GRIFT: CardRecord = CardRecord::new(
    "Shifting Grift",
    "20b8313b-a680-4dca-959a-1a7fa5cb4b1b",
    "Nereida",
    crate::card::CardRules::unsupported(),
);

// OTJ 67 — Slickshot Lockpicker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SLICKSHOT_LOCKPICKER: CardRecord = CardRecord::new(
    "Slickshot Lockpicker",
    "109a9464-ce30-4747-874e-3bbf75913081",
    "Wei Wei",
    crate::card::CardRules::unsupported(),
);

// OTJ 68 — Slickshot Vault-Buster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SLICKSHOT_VAULT_BUSTER: CardRecord = CardRecord::new(
    "Slickshot Vault-Buster",
    "592ccc36-3d10-4a12-8743-9b300b80cb4d",
    "Julia Metzger",
    crate::card::CardRules::unsupported(),
);

// OTJ 69 — Spring Splasher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPRING_SPLASHER: CardRecord = CardRecord::new(
    "Spring Splasher",
    "b6822d12-1a25-42e7-94cc-71bd29daed93",
    "Loïc Canavaggia",
    crate::card::CardRules::unsupported(),
);

// OTJ 70 — Step Between Worlds
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STEP_BETWEEN_WORLDS: CardRecord = CardRecord::new(
    "Step Between Worlds",
    "70ea2054-3d22-42ce-ab50-501ef09c2128",
    "Chris Ostrowski",
    crate::card::CardRules::unsupported(),
);

// OTJ 71 — Stoic Sphinx
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STOIC_SPHINX: CardRecord = CardRecord::new(
    "Stoic Sphinx",
    "f93f5055-30d8-4fc4-afa5-29212e8c7536",
    "Andreas Zafiratos",
    crate::card::CardRules::unsupported(),
);

// OTJ 72 — Stop Cold
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STOP_COLD: CardRecord = CardRecord::new(
    "Stop Cold",
    "9ff9c158-0080-427c-8cf9-0289011ea63e",
    "David Astruga",
    crate::card::CardRules::unsupported(),
);

// OTJ 73 — Take the Fall
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TAKE_THE_FALL: CardRecord = CardRecord::new(
    "Take the Fall",
    "9fea80c4-923c-40a1-9363-bfa4c267a024",
    "Eduardo Francisco",
    crate::card::CardRules::unsupported(),
);

// OTJ 74 — This Town Ain't Big Enough
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THIS_TOWN_AIN_T_BIG_ENOUGH: CardRecord = CardRecord::new(
    "This Town Ain't Big Enough",
    "bb206e27-da4d-4abe-9d8c-6d18c5f2f52a",
    "Andrew Mar",
    crate::card::CardRules::unsupported(),
);

// OTJ 75 — Three Steps Ahead
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THREE_STEPS_AHEAD: CardRecord = CardRecord::new(
    "Three Steps Ahead",
    "8fffd839-2337-4a14-9312-cee085a17f4b",
    "Francisco Miyara",
    crate::card::CardRules::unsupported(),
);

// OTJ 76 — Visage Bandit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VISAGE_BANDIT: CardRecord = CardRecord::new(
    "Visage Bandit",
    "685ec4c6-3332-498f-8b56-d7ad8fc5230c",
    "Miranda Meeks",
    crate::card::CardRules::unsupported(),
);

// OTJ 77 — Ambush Gigapede
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AMBUSH_GIGAPEDE: CardRecord = CardRecord::new(
    "Ambush Gigapede",
    "93b17d09-974a-4e33-b14d-5fe4230ab241",
    "Kekai Kotaki",
    crate::card::CardRules::unsupported(),
);

// OTJ 78 — Binding Negotiation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BINDING_NEGOTIATION: CardRecord = CardRecord::new(
    "Binding Negotiation",
    "1c4c26b9-981f-47cf-b0f4-769e788d9537",
    "Caroline Gariba",
    crate::card::CardRules::unsupported(),
);

// OTJ 79 — Blacksnag Buzzard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLACKSNAG_BUZZARD: CardRecord = CardRecord::new(
    "Blacksnag Buzzard",
    "ef10b2ad-9b9b-4c5d-a2c7-3ce742224b50",
    "Michele Giorgi",
    crate::card::CardRules::unsupported(),
);

// OTJ 80 — Blood Hustler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLOOD_HUSTLER: CardRecord = CardRecord::new(
    "Blood Hustler",
    "1016a750-2a18-4443-a600-957eb4026d3a",
    "Anna Pavleeva",
    crate::card::CardRules::unsupported(),
);

// OTJ 81 — Boneyard Desecrator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BONEYARD_DESECRATOR: CardRecord = CardRecord::new(
    "Boneyard Desecrator",
    "d43981b1-60c8-4896-8885-b07e73b99b30",
    "Maxime Minard",
    crate::card::CardRules::unsupported(),
);

// OTJ 82 — Caustic Bronco
/// The reveal itself: one card off the top, shown to everybody, into your
/// hand, and then the clause above reads what it cost.
pub(in crate::card::sets) static CAUSTIC_BRONCO: CardRecord = CardRecord::new(
    "Caustic Bronco",
    "e9a268ba-c442-4fe4-90b4-2810c8474f4e",
    "Brent Hollowell",
// Two mana for a 2/2 that draws you an extra card every attack. Whether
    // that card costs you or them is what the saddle buys.
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Snake", "Horse", "Mount"], 2, 2)
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever this creature attacks, reveal the top card of your library and put it \
                 into your hand. You lose life equal to that card's mana value if this creature \
                 isn't saddled. Otherwise, each opponent loses that much life.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                abilities::bind_top_cards_then(
                    PlayerRefDef::EffectController,
                    ValueDef::Constant(1),
                    &EffectDef::Sequence(&[
                        EffectDef::RevealObjects(RevealObjectsDef {
                            input: ObjectSetDef::Binding(ParentBinding),
                            then: &EffectDef::None,
                        }),
                        EffectDef::MoveObjects(MoveObjectsDef {
                            input: ObjectSetDef::Binding(ParentBinding),
                            from: Some(ZoneKind::Library),
                            zone: ZoneKind::Hand,
                            placement: ZonePlacement::Top,
                            moved: Some(ParentBinding),
                            then: &EffectDef::IfElseCondition {
                                condition: &TriggerConditionDef::SourceMatches {
                                    object: ObjectPredicateDef::Saddled,
                                },
                                then: &EffectDef::LoseLife {
                                    recipient: EffectRecipientDef::Opponent,
                                    amount: ValueDef::AggregateObjectValues(
                                        &ObjectValueAggregateDef {
                                            objects: ObjectSetDef::Binding(ParentBinding),
                                            select: ObjectValueDef::ManaValue,
                                            operation: AggregateOperationDef::Maximum,
                                        },
                                    ),
                                },
                                otherwise: &EffectDef::LoseLife {
                                    recipient: EffectRecipientDef::Controller,
                                    amount: ValueDef::AggregateObjectValues(
                                        &ObjectValueAggregateDef {
                                            objects: ObjectSetDef::Binding(ParentBinding),
                                            select: ObjectValueDef::ManaValue,
                                            operation: AggregateOperationDef::Maximum,
                                        },
                                    ),
                                },
                            },
                        }),
                    ]),
                ),
            ),
            abilities::saddle(
                &[CostDef::TapCreaturesWithTotalPower { minimum: 3 }],
                "Saddle 3 (Tap any number of other creatures you control with total power 3 or \
                                         more: This Mount becomes saddled until end of turn. Saddle only as a sorcery.)",
            ),
        ]),
);

// OTJ 83 — Consuming Ashes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CONSUMING_ASHES: CardRecord = CardRecord::new(
    "Consuming Ashes",
    "54f96be9-60fc-4e2f-9172-4cc53c9a095a",
    "Campbell White",
    crate::card::CardRules::unsupported(),
);

// OTJ 84 — Corrupted Conviction (reprint)
const CORRUPTED_CONVICTION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_mom::CORRUPTED_CONVICTION,
    "8046f892-3317-4ef7-9cf7-97b9060540c8",
    "Inkognit",
);

// OTJ 85 — Desert's Due
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DESERT_S_DUE: CardRecord = CardRecord::new(
    "Desert's Due",
    "35e899e4-e2de-44cf-b6f4-cace8d3770cb",
    "David Palumbo",
    crate::card::CardRules::unsupported(),
);

// OTJ 86 — Desperate Bloodseeker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DESPERATE_BLOODSEEKER: CardRecord = CardRecord::new(
    "Desperate Bloodseeker",
    "a59da027-b5dd-4920-b3a1-9da05fcb1977",
    "Camille Alquier",
    crate::card::CardRules::unsupported(),
);

// OTJ 87 — Fake Your Own Death (reprint)
const FAKE_YOUR_OWN_DEATH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_snc::FAKE_YOUR_OWN_DEATH,
    "79a17ab9-13c9-41d4-a143-82d8caacfd8b",
    "Monztre",
);

// OTJ 88 — Forsaken Miner
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORSAKEN_MINER: CardRecord = CardRecord::new(
    "Forsaken Miner",
    "1679f74d-00f8-436c-9f8c-aa3f843a546c",
    "Andrey Kuzinskiy",
    crate::card::CardRules::unsupported(),
);

// OTJ 89 — Gisa, the Hellraiser
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GISA_THE_HELLRAISER: CardRecord = CardRecord::new(
    "Gisa, the Hellraiser",
    "db7c07b2-02b2-4e62-bf1b-4848e06eec28",
    "Chris Rahn",
    crate::card::CardRules::unsupported(),
);

// OTJ 90 — Hollow Marauder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HOLLOW_MARAUDER: CardRecord = CardRecord::new(
    "Hollow Marauder",
    "df2913d5-57c7-4f9b-bc96-8a46beef2563",
    "Wero Gallo",
    crate::card::CardRules::unsupported(),
);

// OTJ 91 — Insatiable Avarice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INSATIABLE_AVARICE: CardRecord = CardRecord::new(
    "Insatiable Avarice",
    "a108b0e4-1b43-4659-9e91-facb0bd57ebb",
    "Scott Murphy",
    crate::card::CardRules::unsupported(),
);

// OTJ 92 — Kaervek, the Punisher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KAERVEK_THE_PUNISHER: CardRecord = CardRecord::new(
    "Kaervek, the Punisher",
    "7f3affc1-be42-48c7-89ff-b59550ff278c",
    "Cristi Balanescu",
    crate::card::CardRules::unsupported(),
);

// OTJ 93 — Lively Dirge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIVELY_DIRGE: CardRecord = CardRecord::new(
    "Lively Dirge",
    "0c35a0d3-12f7-46f3-a6b3-02a490d45ca0",
    "Warren Mahy",
    crate::card::CardRules::unsupported(),
);

// OTJ 94 — Mourner's Surprise
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOURNER_S_SURPRISE: CardRecord = CardRecord::new(
    "Mourner's Surprise",
    "980b0b68-7218-49c6-b6bf-022218f3abf4",
    "Zuzanna Wużyk",
    crate::card::CardRules::unsupported(),
);

// OTJ 95 — Neutralize the Guards
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NEUTRALIZE_THE_GUARDS: CardRecord = CardRecord::new(
    "Neutralize the Guards",
    "60f1a481-598e-4e05-8471-eedb12a39022",
    "Nereida",
    crate::card::CardRules::unsupported(),
);

// OTJ 96 — Nezumi Linkbreaker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NEZUMI_LINKBREAKER: CardRecord = CardRecord::new(
    "Nezumi Linkbreaker",
    "6dd0095b-6136-4368-94cb-4c82621aaf37",
    "Miro Petrov",
    crate::card::CardRules::unsupported(),
);

// OTJ 97 — Overzealous Muscle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OVERZEALOUS_MUSCLE: CardRecord = CardRecord::new(
    "Overzealous Muscle",
    "ce57d977-e9c5-4ed1-915f-cdc90a54f8f8",
    "Sam White",
    crate::card::CardRules::unsupported(),
);

// OTJ 98 — Pitiless Carnage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PITILESS_CARNAGE: CardRecord = CardRecord::new(
    "Pitiless Carnage",
    "fa76fc45-a106-4dc9-9d44-a005eaa2784d",
    "Richard Kane Ferguson",
    crate::card::CardRules::unsupported(),
);

// OTJ 99 — Rakish Crew
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAKISH_CREW: CardRecord = CardRecord::new(
    "Rakish Crew",
    "70f64358-58db-40ab-90e8-6137c3bd0a29",
    "Ilse Gort",
    crate::card::CardRules::unsupported(),
);

// OTJ 100 — Rattleback Apothecary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RATTLEBACK_APOTHECARY: CardRecord = CardRecord::new(
    "Rattleback Apothecary",
    "9a88e233-f09c-49e7-b1e3-386fba851fdf",
    "Loïc Canavaggia",
    crate::card::CardRules::unsupported(),
);

// OTJ 101 — Raven of Fell Omens
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAVEN_OF_FELL_OMENS: CardRecord = CardRecord::new(
    "Raven of Fell Omens",
    "e2df3cb4-1658-450a-912a-df336706acdc",
    "Justin Cornell",
    crate::card::CardRules::unsupported(),
);

// OTJ 102 — Rictus Robber
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RICTUS_ROBBER: CardRecord = CardRecord::new(
    "Rictus Robber",
    "252def94-2d89-48f7-8ff7-9c8682ca3ec6",
    "Caio Monteiro",
    crate::card::CardRules::unsupported(),
);

// OTJ 103 — Rooftop Assassin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROOFTOP_ASSASSIN: CardRecord = CardRecord::new(
    "Rooftop Assassin",
    "e4e4311a-8583-4cf0-8126-508dbfbcddb6",
    "Josu Hernaiz",
    crate::card::CardRules::unsupported(),
);

// OTJ 104 — Rush of Dread
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUSH_OF_DREAD: CardRecord = CardRecord::new(
    "Rush of Dread",
    "721c7122-91b6-45ea-ba28-de0246a2fc1b",
    "Chris Seaman",
    crate::card::CardRules::unsupported(),
);

// OTJ 105 — Servant of the Stinger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SERVANT_OF_THE_STINGER: CardRecord = CardRecord::new(
    "Servant of the Stinger",
    "b5c98650-b195-4071-8e02-4df35fddddc7",
    "Steven Russell Black",
    crate::card::CardRules::unsupported(),
);

// OTJ 106 — Shoot the Sheriff
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHOOT_THE_SHERIFF: CardRecord = CardRecord::new(
    "Shoot the Sheriff",
    "180d6528-c524-4bb8-8a72-b3775cd2c177",
    "Fariba Khamseh",
    crate::card::CardRules::unsupported(),
);

// OTJ 107 — Skulduggery (reprint)
const SKULDUGGERY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_xln::SKULDUGGERY,
    "03709166-164a-4075-ad0d-ea3b516ab771",
    "Miro Petrov",
);

// OTJ 108 — Tinybones Joins Up
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TINYBONES_JOINS_UP: CardRecord = CardRecord::new(
    "Tinybones Joins Up",
    "5724a15f-0ba0-421a-9cd4-a2b701e6141f",
    "Wylie Beckert",
    crate::card::CardRules::unsupported(),
);

// OTJ 109 — Tinybones, the Pickpocket
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TINYBONES_THE_PICKPOCKET: CardRecord = CardRecord::new(
    "Tinybones, the Pickpocket",
    "3d3025a2-4a17-4137-ba0b-bd676c6f5f88",
    "Ekaterina Burmak",
    crate::card::CardRules::unsupported(),
);

// OTJ 110 — Treasure Dredger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TREASURE_DREDGER: CardRecord = CardRecord::new(
    "Treasure Dredger",
    "c88df498-147c-4609-8a94-d8d10a87e37c",
    "Nereida",
    crate::card::CardRules::unsupported(),
);

// OTJ 111 — Unfortunate Accident
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNFORTUNATE_ACCIDENT: CardRecord = CardRecord::new(
    "Unfortunate Accident",
    "0c5a25d9-926e-4004-8830-2b2bf7bc0775",
    "Josiah \"Jo\" Cameron",
    crate::card::CardRules::unsupported(),
);

// OTJ 112 — Unscrupulous Contractor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNSCRUPULOUS_CONTRACTOR: CardRecord = CardRecord::new(
    "Unscrupulous Contractor",
    "9e56a8df-db04-4a88-a5ab-6954d3449976",
    "Mila Pesic",
    crate::card::CardRules::unsupported(),
);

// OTJ 113 — Vadmir, New Blood
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VADMIR_NEW_BLOOD: CardRecord = CardRecord::new(
    "Vadmir, New Blood",
    "828b5855-af5a-46a6-8fd4-0a2e28f3bb01",
    "Andreas Zafiratos",
    crate::card::CardRules::unsupported(),
);

// OTJ 114 — Vault Plunderer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VAULT_PLUNDERER: CardRecord = CardRecord::new(
    "Vault Plunderer",
    "2e6bf35c-8763-47cc-ab2d-5dbabeb28072",
    "Evyn Fong",
    crate::card::CardRules::unsupported(),
);

// OTJ 115 — Brimstone Roundup
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRIMSTONE_ROUNDUP: CardRecord = CardRecord::new(
    "Brimstone Roundup",
    "bd13011e-a4fc-4107-988f-60cfc851ecd3",
    "Milivoj Ćeran",
    crate::card::CardRules::unsupported(),
);

// OTJ 116 — Calamity, Galloping Inferno
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CALAMITY_GALLOPING_INFERNO: CardRecord = CardRecord::new(
    "Calamity, Galloping Inferno",
    "e7a70f5a-2056-4c26-b6ea-9f751b5d0d8c",
    "Artur Nakhodkin",
    crate::card::CardRules::unsupported(),
);

// OTJ 117 — Caught in the Crossfire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CAUGHT_IN_THE_CROSSFIRE: CardRecord = CardRecord::new(
    "Caught in the Crossfire",
    "3a2fd0c4-509e-49c2-ad57-f772efcbc207",
    "Xabi Gaztelua",
    crate::card::CardRules::unsupported(),
);

// OTJ 118 — Cunning Coyote
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CUNNING_COYOTE: CardRecord = CardRecord::new(
    "Cunning Coyote",
    "5b4ac6ea-c67b-4f90-be4f-aa25882f5794",
    "David Auden Nash",
    crate::card::CardRules::unsupported(),
);

// OTJ 119 — Deadeye Duelist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEADEYE_DUELIST: CardRecord = CardRecord::new(
    "Deadeye Duelist",
    "e9a50b7a-8741-4520-8d45-8e6b128c2628",
    "Diana Cearley",
    crate::card::CardRules::unsupported(),
);

// OTJ 120 — Demonic Ruckus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEMONIC_RUCKUS: CardRecord = CardRecord::new(
    "Demonic Ruckus",
    "5b491d11-d00a-4541-8389-2785a455eeee",
    "Andrew Mar",
    crate::card::CardRules::unsupported(),
);

// OTJ 121 — Discerning Peddler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DISCERNING_PEDDLER: CardRecord = CardRecord::new(
    "Discerning Peddler",
    "73b359bf-afd8-439d-a4d2-985db1ad368c",
    "Josu Hernaiz",
    crate::card::CardRules::unsupported(),
);

// OTJ 122 — Explosive Derailment
pub(in crate::card::sets) static EXPLOSIVE_DERAILMENT: CardRecord = CardRecord::new(
    "Explosive Derailment",
    "f0e3df9c-0a86-4e6f-a3c7-84a883328a3d",
    "Leon Tukker",
    CardRules::new_instant(mana_cost!("{R}")).with_ability(spree(&[
        (
            &[CostDef::Mana(mana_cost!("{2}"))],
            AbilityDef::spell_with_targets(
                "Explosive Derailment deals 4 damage to target creature.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(4),
                ),
            ),
        ),
        (
            &[CostDef::Mana(mana_cost!("{2}"))],
            AbilityDef::destroy_target(
                "Destroy target artifact.",
                &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                    CardType::Artifact,
                )),
            ),
        ),
    ])),
);

// OTJ 123 — Ferocification
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FEROCIFICATION: CardRecord = CardRecord::new(
    "Ferocification",
    "78db4260-6fc8-4500-afd7-2c845ac0d53b",
    "Mila Pesic",
    crate::card::CardRules::unsupported(),
);

// OTJ 124 — Gila Courser
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GILA_COURSER: CardRecord = CardRecord::new(
    "Gila Courser",
    "f568803d-65c0-48d7-916f-671267a9e00e",
    "Brent Hollowell",
    crate::card::CardRules::unsupported(),
);

// OTJ 125 — Great Train Heist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GREAT_TRAIN_HEIST: CardRecord = CardRecord::new(
    "Great Train Heist",
    "357dd9b2-5d2d-49f6-86f0-f5c4d63474dd",
    "Campbell White",
    crate::card::CardRules::unsupported(),
);

// OTJ 126 — Hell to Pay
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HELL_TO_PAY: CardRecord = CardRecord::new(
    "Hell to Pay",
    "84ad8ed7-1429-432e-8217-a4db3b97675c",
    "Liiga Smilshkalne",
    crate::card::CardRules::unsupported(),
);

// OTJ 127 — Hellspur Brute
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HELLSPUR_BRUTE: CardRecord = CardRecord::new(
    "Hellspur Brute",
    "3b99db5b-cd13-4e69-98b7-753e72c781f8",
    "Caio Monteiro",
    crate::card::CardRules::unsupported(),
);

// OTJ 128 — Hellspur Posse Boss
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HELLSPUR_POSSE_BOSS: CardRecord = CardRecord::new(
    "Hellspur Posse Boss",
    "5f348c7e-7d72-40f1-a65f-ee7ff4b09412",
    "Artur Nakhodkin",
    crate::card::CardRules::unsupported(),
);

// OTJ 129 — Highway Robbery
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HIGHWAY_ROBBERY: CardRecord = CardRecord::new(
    "Highway Robbery",
    "31a88429-9204-4a23-a7a8-babbd6bab79f",
    "Scott Murphy",
    crate::card::CardRules::unsupported(),
);

// OTJ 130 — Irascible Wolverine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IRASCIBLE_WOLVERINE: CardRecord = CardRecord::new(
    "Irascible Wolverine",
    "324c0af5-7cdf-4c71-84cc-6349f10e0d66",
    "Darrell Riche",
    crate::card::CardRules::unsupported(),
);

// OTJ 131 — Iron-Fist Pulverizer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IRON_FIST_PULVERIZER: CardRecord = CardRecord::new(
    "Iron-Fist Pulverizer",
    "cd7f984a-0b56-45df-958d-6178e4da61ed",
    "Xabi Gaztelua",
    crate::card::CardRules::unsupported(),
);

// OTJ 132 — Longhorn Sharpshooter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LONGHORN_SHARPSHOOTER: CardRecord = CardRecord::new(
    "Longhorn Sharpshooter",
    "398d9a16-d72c-42e2-a0ea-d9da642ee046",
    "Diego Gisbert",
    crate::card::CardRules::unsupported(),
);

// OTJ 133 — Magda, the Hoardmaster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAGDA_THE_HOARDMASTER: CardRecord = CardRecord::new(
    "Magda, the Hoardmaster",
    "4443d112-209b-49ec-bc40-3a11dcdb092e",
    "Diego Gisbert",
    crate::card::CardRules::unsupported(),
);

// OTJ 134 — Magebane Lizard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAGEBANE_LIZARD: CardRecord = CardRecord::new(
    "Magebane Lizard",
    "62e12566-375f-4f31-aa91-1b13a96d9ece",
    "Camille Alquier",
    crate::card::CardRules::unsupported(),
);

// OTJ 135 — Mine Raider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MINE_RAIDER: CardRecord = CardRecord::new(
    "Mine Raider",
    "19cfacff-e884-4954-aa6b-ed56ca942bf2",
    "Warren Mahy",
    crate::card::CardRules::unsupported(),
);

// OTJ 136 — Outlaws' Fury
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OUTLAWS_FURY: CardRecord = CardRecord::new(
    "Outlaws' Fury",
    "f7502b9c-b759-499a-8e94-22f87f5eb142",
    "Diego Gisbert",
    crate::card::CardRules::unsupported(),
);

// OTJ 137 — Prickly Pair
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PRICKLY_PAIR: CardRecord = CardRecord::new(
    "Prickly Pair",
    "e70f2278-8857-46e4-aa2b-fff5589b750f",
    "Brian Valeza",
    crate::card::CardRules::unsupported(),
);

// OTJ 138 — Quick Draw
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static QUICK_DRAW: CardRecord = CardRecord::new(
    "Quick Draw",
    "56399cd0-1214-42b6-be38-f2cbd770915f",
    "Lie Setiawan",
    crate::card::CardRules::unsupported(),
);

// OTJ 139 — Quilled Charger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static QUILLED_CHARGER: CardRecord = CardRecord::new(
    "Quilled Charger",
    "56e326c4-39f1-41ee-9e47-ebe468c51718",
    "Diego Gisbert",
    crate::card::CardRules::unsupported(),
);

// OTJ 140 — Reckless Lackey
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RECKLESS_LACKEY: CardRecord = CardRecord::new(
    "Reckless Lackey",
    "912fcd14-5e81-418c-997b-771f2f38f63d",
    "Edgar Sánchez Hidalgo",
    crate::card::CardRules::unsupported(),
);

// OTJ 141 — Resilient Roadrunner
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RESILIENT_ROADRUNNER: CardRecord = CardRecord::new(
    "Resilient Roadrunner",
    "e07d3ee9-d3c4-4f07-839e-ec81c2587ae0",
    "David Auden Nash",
    crate::card::CardRules::unsupported(),
);

// OTJ 142 — Return the Favor
pub(in crate::card::sets) static RETURN_THE_FAVOR: CardRecord = CardRecord::new(
    "Return the Favor",
    "a9cc02d1-799d-42aa-9bc2-4c05452b63b4",
    "Eli Minaya",
CardRules::new_instant(mana_cost!("{R}{R}")).with_ability(spree(&[(&[CostDef::Mana(mana_cost!("{1}"))], AbilityDef::spell_with_targets(
                "Copy target instant spell, sorcery spell, activated ability, or triggered ability. You may choose new targets for the copy.",
                &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Spell,
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Instant),
                                ObjectPredicateDef::HasType(CardType::Sorcery),
                            ]),
                        ]),
                        ObjectPredicateDef::Ability,
                    ]),
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                })],
                EffectDef::CopyStackObject(&CopyStackObjectDef {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    controller: PlayerRefDef::EffectController,
                    count: ValueDef::Constant(1),
                    retarget: true,
                    colors: None,
                }),
            )),
(&[CostDef::Mana(mana_cost!("{1}"))], AbilityDef::spell_with_targets(
                "Change the target of target spell or ability with a single target.",
                &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::DeclaredTargetCount {
                        minimum: 1,
                        maximum: 1,
                    },
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                })],
                EffectDef::ChangeStackTargets(&ChangeStackTargetsDef {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    chooser: PlayerRefDef::EffectController,
                    change: StackTargetChangeDef::ChooseNew {
                        optional: false,
                        restriction: None,
                    },
                }),
            ))])),
);

// OTJ 143 — Rodeo Pyromancers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RODEO_PYROMANCERS: CardRecord = CardRecord::new(
    "Rodeo Pyromancers",
    "df877a29-06e1-474d-8600-410bbec674ae",
    "Kim Sokol",
    crate::card::CardRules::unsupported(),
);

// OTJ 144 — Scalestorm Summoner
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCALESTORM_SUMMONER: CardRecord = CardRecord::new(
    "Scalestorm Summoner",
    "d603c00f-048a-4a05-9df9-52844819d523",
    "Xabi Gaztelua",
    crate::card::CardRules::unsupported(),
);

// OTJ 145 — Scorching Shot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCORCHING_SHOT: CardRecord = CardRecord::new(
    "Scorching Shot",
    "f93d8357-83bd-4157-a389-68e4aa4985c8",
    "Caio Monteiro",
    crate::card::CardRules::unsupported(),
);

// OTJ 146 — Slickshot Show-Off (alternate printing)
const SLICKSHOT_SHOW_OFF_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SLICKSHOT_SHOW_OFF,
    1,
    "7054012b-4f9d-44a0-aaf9-7fd3bddc7b2d",
    "Augusto Quirino",
);

// OTJ 147 — Stingerback Terror
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STINGERBACK_TERROR: CardRecord = CardRecord::new(
    "Stingerback Terror",
    "d84d6e52-5c35-47bc-b160-876a3b0fcbe1",
    "Slawomir Maniak",
    crate::card::CardRules::unsupported(),
);

// OTJ 148 — Take for a Ride
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TAKE_FOR_A_RIDE: CardRecord = CardRecord::new(
    "Take for a Ride",
    "c8e2ff9c-0e98-46f9-a33c-739388c5f3d0",
    "Artur Treffner",
    crate::card::CardRules::unsupported(),
);

// OTJ 149 — Terror of the Peaks (reprint)
const TERROR_OF_THE_PEAKS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_m21::TERROR_OF_THE_PEAKS,
    "904ff94a-4db4-44a6-8593-89c32905b3fc",
    "Joshua Raphael",
);

// OTJ 150 — Thunder Salvo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THUNDER_SALVO: CardRecord = CardRecord::new(
    "Thunder Salvo",
    "a0bf0ea9-0929-4d33-815a-6df29c399e7e",
    "Johann Bodin",
    crate::card::CardRules::unsupported(),
);

// OTJ 151 — Trick Shot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRICK_SHOT: CardRecord = CardRecord::new(
    "Trick Shot",
    "cd3c2d02-67ca-4858-9b7a-3cfe8a08356c",
    "Brian Valeza",
    crate::card::CardRules::unsupported(),
);

// OTJ 152 — Aloe Alchemist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALOE_ALCHEMIST: CardRecord = CardRecord::new(
    "Aloe Alchemist",
    "69f2f632-b6cc-4092-acd5-a6b152e90488",
    "Borja Pindado",
    crate::card::CardRules::unsupported(),
);

// OTJ 153 — Ankle Biter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANKLE_BITER: CardRecord = CardRecord::new(
    "Ankle Biter",
    "424972d6-3b2c-449b-b786-749a77020fa1",
    "Monztre",
    crate::card::CardRules::unsupported(),
);

// OTJ 154 — Beastbond Outcaster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BEASTBOND_OUTCASTER: CardRecord = CardRecord::new(
    "Beastbond Outcaster",
    "073b9ae8-8ac3-4824-aec4-84a80531aa23",
    "Viko Menezes",
    crate::card::CardRules::unsupported(),
);

// OTJ 155 — Betrayal at the Vault
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BETRAYAL_AT_THE_VAULT: CardRecord = CardRecord::new(
    "Betrayal at the Vault",
    "c494820f-607d-4ed2-8a86-a916ae390272",
    "Andreas Zafiratos",
    crate::card::CardRules::unsupported(),
);

// OTJ 156 — Bristlepack Sentry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRISTLEPACK_SENTRY: CardRecord = CardRecord::new(
    "Bristlepack Sentry",
    "6aea6702-16a8-4073-9c83-fbea20a5fd32",
    "Josiah \"Jo\" Cameron",
    crate::card::CardRules::unsupported(),
);

// OTJ 157 — Bristly Bill, Spine Sower
pub(in crate::card::sets) static BRISTLY_BILL_SPINE_SOWER: CardRecord =
    CardRecord::new(
    "Bristly Bill, Spine Sower",
    "52eef0d6-24b7-40b7-8403-e8e863d0cd55",
    "Daniel Zrom",
// The counters accumulate for free off lands, and then the activation
        // turns a slow board into a lethal one in a single turn.
        CardRules::new_creature(mana_cost!("{1}{G}"), &["Plant", "Druid"], 2, 2)
            .with_supertype(CardSupertype::Legendary)
            .with_abilities(&[
                AbilityDef::triggered_with_targets(
                    "Landfall — Whenever a land you control enters, put a +1/+1 counter on target creature.",
                    TriggerEventDef::zone_changed(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ]),
                        None,
                        Some(ZoneKind::Battlefield),
                    ),
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::HasType(CardType::Creature),
                    )],
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                ),
                // Each creature doubles its own, so a board of one-counter creatures
                // gains one apiece and a single large one gains everything it has.
                AbilityDef::activated(
                    "{3}{G}{G}: Double the number of +1/+1 counters on each creature you control.",
                    &[CostDef::Mana(mana_cost!("{3}{G}{G}"))],
                    EffectDef::DoubleCounters {
                        object: EffectRecipientDef::matching_objects(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                        kind: CounterKind::PlusOnePlusOne,
                    },
                ),
            ]),
);

// OTJ 158 — Cactarantula
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CACTARANTULA: CardRecord = CardRecord::new(
    "Cactarantula",
    "2e0e27f9-dc2c-4366-b810-3e8d0bdff8c3",
    "Filip Burburan",
    crate::card::CardRules::unsupported(),
);

// OTJ 159 — Colossal Rattlewurm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COLOSSAL_RATTLEWURM: CardRecord = CardRecord::new(
    "Colossal Rattlewurm",
    "17a104d3-e4ac-44a0-9c6a-39965b1b9751",
    "Filip Burburan",
    crate::card::CardRules::unsupported(),
);

// OTJ 160 — Dance of the Tumbleweeds
static DANCE_LANDS_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Land),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);
pub(in crate::card::sets) static DANCE_OF_THE_TUMBLEWEEDS: CardRecord = CardRecord::new(
    "Dance of the Tumbleweeds",
    "caf0e715-befb-4904-82e6-d3f8c7fbd454",
    "Dan Murayama Scott",
CardRules::new_sorcery(mana_cost!("{1}{G}")).with_ability(spree(&[(&[CostDef::Mana(mana_cost!("{1}"))], AbilityDef::spell(
                "Search your library for a basic land card or a Desert card, put it onto the battlefield, then shuffle.",
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::Supertype(CardSupertype::Basic),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Desert")),
                        ]),
                    ]),
                    minimum: 0,
                    maximum: ValueDef::Constant(1),
                    reveal: true,
                    destination: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: false,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            )),
(&[CostDef::Mana(mana_cost!("{3}"))], AbilityDef::spell(
                "Create an X/X green Elemental creature token, where X is the number of lands you control.",
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                    TokenCharacteristics::creature_with_stats(
                        &["Elemental"],
                        &[ManaColor::Green],
                        &TokenStatsDef {
                            power: ValueDef::CountMatchingObjects(&DANCE_LANDS_YOU_CONTROL),
                            toughness: ValueDef::CountMatchingObjects(&DANCE_LANDS_YOU_CONTROL),
                        },
                    ),
                ))),
            ),
        ),
    ])),
);

// OTJ 161 — Drover Grizzly
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DROVER_GRIZZLY: CardRecord = CardRecord::new(
    "Drover Grizzly",
    "560062cd-34f8-4d30-9e25-099b03961724",
    "Adrián Rodríguez Pérez",
    crate::card::CardRules::unsupported(),
);

// OTJ 162 — Freestrider Commando
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FREESTRIDER_COMMANDO: CardRecord = CardRecord::new(
    "Freestrider Commando",
    "92762169-095e-46e2-82f6-5b2ff2232240",
    "Adrián Rodríguez Pérez",
    crate::card::CardRules::unsupported(),
);

// OTJ 163 — Freestrider Lookout
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FREESTRIDER_LOOKOUT: CardRecord = CardRecord::new(
    "Freestrider Lookout",
    "32370f05-52a2-405f-b2bb-1b8a9b0b69f8",
    "Matt Zeilinger",
    crate::card::CardRules::unsupported(),
);

// OTJ 164 — Full Steam Ahead
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FULL_STEAM_AHEAD: CardRecord = CardRecord::new(
    "Full Steam Ahead",
    "084748d8-7169-4e86-a69c-631c6d7d3a1e",
    "Inkognit",
    crate::card::CardRules::unsupported(),
);

// OTJ 165 — Giant Beaver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GIANT_BEAVER: CardRecord = CardRecord::new(
    "Giant Beaver",
    "919826a9-c427-42c6-8885-a87f0b6d2192",
    "Lars Grant-West",
    crate::card::CardRules::unsupported(),
);

// OTJ 166 — Gold Rush
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOLD_RUSH: CardRecord = CardRecord::new(
    "Gold Rush",
    "15845be1-919d-4450-9de6-3552c52e8623",
    "Eric Wilkerson",
    crate::card::CardRules::unsupported(),
);

// OTJ 167 — Goldvein Hydra
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOLDVEIN_HYDRA: CardRecord = CardRecord::new(
    "Goldvein Hydra",
    "de0e01e4-e143-47fd-8565-7b48219bb546",
    "David Auden Nash",
    crate::card::CardRules::unsupported(),
);

// OTJ 168 — Hardbristle Bandit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HARDBRISTLE_BANDIT: CardRecord = CardRecord::new(
    "Hardbristle Bandit",
    "cbe544fb-93b7-4640-b886-cb0b3e437357",
    "Francis Tneh",
    crate::card::CardRules::unsupported(),
);

// OTJ 169 — Intrepid Stablemaster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INTREPID_STABLEMASTER: CardRecord = CardRecord::new(
    "Intrepid Stablemaster",
    "4d6cdf2a-026a-41ba-87d9-8fcd8a67f06e",
    "Svetlin Velinov",
    crate::card::CardRules::unsupported(),
);

// OTJ 170 — Map the Frontier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAP_THE_FRONTIER: CardRecord = CardRecord::new(
    "Map the Frontier",
    "165f4428-e1a1-477d-bd90-138189e88163",
    "Darrell Riche",
    crate::card::CardRules::unsupported(),
);

// OTJ 171 — Ornery Tumblewagg
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORNERY_TUMBLEWAGG: CardRecord = CardRecord::new(
    "Ornery Tumblewagg",
    "0020c31b-002a-4121-bc61-2c2c16e9afc8",
    "Izzy",
    crate::card::CardRules::unsupported(),
);

// OTJ 172 — Outcaster Greenblade
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OUTCASTER_GREENBLADE: CardRecord = CardRecord::new(
    "Outcaster Greenblade",
    "c9458f0f-5593-4ac9-934c-e215ef8093a7",
    "Josu Hernaiz",
    crate::card::CardRules::unsupported(),
);

// OTJ 173 — Outcaster Trailblazer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OUTCASTER_TRAILBLAZER: CardRecord = CardRecord::new(
    "Outcaster Trailblazer",
    "33b9cd6c-d75c-4905-aa38-ff03a9c4b398",
    "Denys Tsiperko",
    crate::card::CardRules::unsupported(),
);

// OTJ 174 — Patient Naturalist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PATIENT_NATURALIST: CardRecord = CardRecord::new(
    "Patient Naturalist",
    "1dd17cea-9e8c-4dba-b6ab-a6b9de87a306",
    "Inka Schulz",
    crate::card::CardRules::unsupported(),
);

// OTJ 175 — Railway Brawler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAILWAY_BRAWLER: CardRecord = CardRecord::new(
    "Railway Brawler",
    "9ec1f76f-f21d-4f06-8c02-be6745183348",
    "Kevin Sidharta",
    crate::card::CardRules::unsupported(),
);

// OTJ 176 — Rambling Possum
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAMBLING_POSSUM: CardRecord = CardRecord::new(
    "Rambling Possum",
    "19d1e75f-0fee-4e07-9420-df771b696e85",
    "Adrián Rodríguez Pérez",
    crate::card::CardRules::unsupported(),
);

// OTJ 177 — Raucous Entertainer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAUCOUS_ENTERTAINER: CardRecord = CardRecord::new(
    "Raucous Entertainer",
    "8dc3cbfe-410f-40e3-8021-647a2efb50bf",
    "Forrest Imel",
    crate::card::CardRules::unsupported(),
);

// OTJ 178 — Reach for the Sky
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REACH_FOR_THE_SKY: CardRecord = CardRecord::new(
    "Reach for the Sky",
    "eb871985-a11b-4dfe-b0e3-898888c86277",
    "Villarrte",
    crate::card::CardRules::unsupported(),
);

// OTJ 179 — Rise of the Varmints
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RISE_OF_THE_VARMINTS: CardRecord = CardRecord::new(
    "Rise of the Varmints",
    "f4e879d8-a058-48e8-9733-f58b1e0da4b9",
    "Ralph Horsley",
    crate::card::CardRules::unsupported(),
);

// OTJ 180 — Smuggler's Surprise
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SMUGGLER_S_SURPRISE: CardRecord = CardRecord::new(
    "Smuggler's Surprise",
    "e7fbb489-e2b5-4278-8162-86802cf124d8",
    "Jonas De Ro",
    crate::card::CardRules::unsupported(),
);

// OTJ 181 — Snakeskin Veil (reprint)
const SNAKESKIN_VEIL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_khm::SNAKESKIN_VEIL,
    "133fbdec-0d00-433f-9015-5eb091126e3a",
    "Dan Murayama Scott",
);

// OTJ 182 — Spinewoods Armadillo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPINEWOODS_ARMADILLO: CardRecord = CardRecord::new(
    "Spinewoods Armadillo",
    "f79d63e7-a8c6-4750-91c9-c575a4d0561b",
    "Iris Compiet",
    crate::card::CardRules::unsupported(),
);

// OTJ 183 — Spinewoods Paladin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPINEWOODS_PALADIN: CardRecord = CardRecord::new(
    "Spinewoods Paladin",
    "1b2b432d-9e73-4ab2-a098-546d406df6c0",
    "Kai Carpenter",
    crate::card::CardRules::unsupported(),
);

// OTJ 184 — Stubborn Burrowfiend
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STUBBORN_BURROWFIEND: CardRecord = CardRecord::new(
    "Stubborn Burrowfiend",
    "6d963eb4-d20b-4d3f-bf5d-c75f7bcb9670",
    "Ângelo Bortolini",
    crate::card::CardRules::unsupported(),
);

// OTJ 185 — Throw from the Saddle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THROW_FROM_THE_SADDLE: CardRecord = CardRecord::new(
    "Throw from the Saddle",
    "775874cc-4b78-4904-9c97-431c2e400c64",
    "Eilene Cherie",
    crate::card::CardRules::unsupported(),
);

// OTJ 186 — Trash the Town
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRASH_THE_TOWN: CardRecord = CardRecord::new(
    "Trash the Town",
    "eda59f99-1d6d-4051-ac89-b7cbfa19262e",
    "David Auden Nash",
    crate::card::CardRules::unsupported(),
);

// OTJ 187 — Tumbleweed Rising
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TUMBLEWEED_RISING: CardRecord = CardRecord::new(
    "Tumbleweed Rising",
    "275d2d2a-ef85-48c9-919d-bc62cdad8a10",
    "Jason Smith",
    crate::card::CardRules::unsupported(),
);

// OTJ 188 — Voracious Varmint
pub(in crate::card::sets) static VORACIOUS_VARMINT: CardRecord = CardRecord::new(
    "Voracious Varmint",
    "99b74fa3-c1d7-4780-977d-f2d6663a529a",
    "Adrián Rodríguez Pérez",
    // Maindeckable artifact and enchantment removal that is a body until it
    // is needed, which is what vigilance is doing on a two-drop.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Varmint"], 2, 2).with_abilities(&[
        abilities::vigilance(),
        AbilityDef::activated_with_targets(
            "{1}, Sacrifice this creature: Destroy target artifact or enchantment.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                ]),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
    ]),
);

// OTJ 189 — Akul the Unrepentant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AKUL_THE_UNREPENTANT: CardRecord = CardRecord::new(
    "Akul the Unrepentant",
    "68fd8548-50db-4243-9154-377f32408d58",
    "Kekai Kotaki",
    crate::card::CardRules::unsupported(),
);

// OTJ 190 — Annie Flash, the Veteran
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANNIE_FLASH_THE_VETERAN: CardRecord = CardRecord::new(
    "Annie Flash, the Veteran",
    "8d4af7c3-a70d-4f71-b27d-b268c4a0f81e",
    "Kieran Yanner",
    crate::card::CardRules::unsupported(),
);

// OTJ 191 — Annie Joins Up
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANNIE_JOINS_UP: CardRecord = CardRecord::new(
    "Annie Joins Up",
    "1624a5f4-f5bc-47c9-85de-c5520ee234ce",
    "Wylie Beckert",
    crate::card::CardRules::unsupported(),
);

// OTJ 192 — Assimilation Aegis
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ASSIMILATION_AEGIS: CardRecord = CardRecord::new(
    "Assimilation Aegis",
    "014bf3c6-e46f-48f8-902f-82deeba260b2",
    "Matt Stewart",
    crate::card::CardRules::unsupported(),
);

// OTJ 193 — At Knifepoint
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AT_KNIFEPOINT: CardRecord = CardRecord::new(
    "At Knifepoint",
    "897d594d-b5b0-43dc-b877-b483942416ce",
    "Francisco Miyara",
    crate::card::CardRules::unsupported(),
);

// OTJ 194 — Badlands Revival
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BADLANDS_REVIVAL: CardRecord = CardRecord::new(
    "Badlands Revival",
    "8d3ef971-cdd4-410c-97c3-df98e4f02ab2",
    "Carlos Palma Cruchaga",
    crate::card::CardRules::unsupported(),
);

// OTJ 195 — Baron Bertram Graywater
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARON_BERTRAM_GRAYWATER: CardRecord = CardRecord::new(
    "Baron Bertram Graywater",
    "e9da18b4-1efc-44b7-8001-a2cfd44c69bf",
    "Johan Grenier",
    crate::card::CardRules::unsupported(),
);

// OTJ 196 — Bonny Pall, Clearcutter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BONNY_PALL_CLEARCUTTER: CardRecord = CardRecord::new(
    "Bonny Pall, Clearcutter",
    "4383ae7c-58ea-4354-93e4-677ad185c3bb",
    "Bryan Sola",
    crate::card::CardRules::unsupported(),
);

// OTJ 197 — Breeches, the Blastmaker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BREECHES_THE_BLASTMAKER: CardRecord = CardRecord::new(
    "Breeches, the Blastmaker",
    "cf3bda9e-42af-4f99-a504-c96c25c2794b",
    "Dmitry Burmak",
    crate::card::CardRules::unsupported(),
);

// OTJ 198 — Bruse Tarl, Roving Rancher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRUSE_TARL_ROVING_RANCHER: CardRecord = CardRecord::new(
    "Bruse Tarl, Roving Rancher",
    "286c55c2-dcc1-4e87-a83f-9981d28ab62d",
    "Forrest Imel",
    crate::card::CardRules::unsupported(),
);

// OTJ 199 — Cactusfolk Sureshot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CACTUSFOLK_SURESHOT: CardRecord = CardRecord::new(
    "Cactusfolk Sureshot",
    "318b8c5d-9fb0-488f-9b32-c2e29d1f1dbb",
    "Artur Nakhodkin",
    crate::card::CardRules::unsupported(),
);

// OTJ 200 — Congregation Gryff
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CONGREGATION_GRYFF: CardRecord = CardRecord::new(
    "Congregation Gryff",
    "0ef4907a-2fc1-42c5-bffc-3b3f93601fb9",
    "Joseph Meehan",
    crate::card::CardRules::unsupported(),
);

// OTJ 201 — Doc Aurlock, Grizzled Genius
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DOC_AURLOCK_GRIZZLED_GENIUS: CardRecord = CardRecord::new(
    "Doc Aurlock, Grizzled Genius",
    "6fc27b30-8c8e-434c-a72c-e1d409efc1ae",
    "Jesper Ejsing",
    crate::card::CardRules::unsupported(),
);

// OTJ 202 — Eriette, the Beguiler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ERIETTE_THE_BEGUILER: CardRecord = CardRecord::new(
    "Eriette, the Beguiler",
    "f46c133a-7ae4-431b-88f2-ec606a7baf69",
    "Chris Rallis",
    crate::card::CardRules::unsupported(),
);

// OTJ 203 — Ertha Jo, Frontier Mentor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ERTHA_JO_FRONTIER_MENTOR: CardRecord = CardRecord::new(
    "Ertha Jo, Frontier Mentor",
    "a4e81be6-6447-4f1e-be00-6fcdb2ab35af",
    "Michal Ivan",
    crate::card::CardRules::unsupported(),
);

// OTJ 204 — Form a Posse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORM_A_POSSE: CardRecord = CardRecord::new(
    "Form a Posse",
    "39ee1387-c24e-4a66-8ad6-9afa9c0abcbb",
    "J.Lonnee",
    crate::card::CardRules::unsupported(),
);

// OTJ 205 — Ghired, Mirror of the Wilds
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GHIRED_MIRROR_OF_THE_WILDS: CardRecord = CardRecord::new(
    "Ghired, Mirror of the Wilds",
    "e43e3d71-4fb8-4ab1-8c8f-b65ae3ad4cc4",
    "Diego Gisbert",
    crate::card::CardRules::unsupported(),
);

// OTJ 206 — The Gitrog, Ravenous Ride
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_GITROG_RAVENOUS_RIDE: CardRecord = CardRecord::new(
    "The Gitrog, Ravenous Ride",
    "82512813-8618-483b-a7f0-e6a611d9d487",
    "Johan Grenier",
    crate::card::CardRules::unsupported(),
);

// OTJ 207 — Honest Rutstein
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HONEST_RUTSTEIN: CardRecord = CardRecord::new(
    "Honest Rutstein",
    "259ddf66-76af-4857-83c3-c812327a6e23",
    "Javier Charro",
    crate::card::CardRules::unsupported(),
);

// OTJ 208 — Intimidation Campaign
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INTIMIDATION_CAMPAIGN: CardRecord = CardRecord::new(
    "Intimidation Campaign",
    "596fb7a2-bb79-44b7-ad84-414c8139ec13",
    "Svetlin Velinov",
    crate::card::CardRules::unsupported(),
);

// OTJ 209 — Jem Lightfoote, Sky Explorer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JEM_LIGHTFOOTE_SKY_EXPLORER: CardRecord = CardRecord::new(
    "Jem Lightfoote, Sky Explorer",
    "e24fe6dc-662a-4abc-ad60-a1959b2be006",
    "Darren Tan",
    crate::card::CardRules::unsupported(),
);

// OTJ 210 — Jolene, Plundering Pugilist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JOLENE_PLUNDERING_PUGILIST: CardRecord = CardRecord::new(
    "Jolene, Plundering Pugilist",
    "fe30b5c8-4889-4350-bb1d-3e2a67d9dfb2",
    "Andreas Zafiratos",
    crate::card::CardRules::unsupported(),
);

// OTJ 211 — Kambal, Profiteering Mayor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KAMBAL_PROFITEERING_MAYOR: CardRecord = CardRecord::new(
    "Kambal, Profiteering Mayor",
    "d53a775d-5898-41a8-b404-9b7d4721c6ba",
    "Andreas Zafiratos",
    crate::card::CardRules::unsupported(),
);

// OTJ 212 — Kellan Joins Up
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KELLAN_JOINS_UP: CardRecord = CardRecord::new(
    "Kellan Joins Up",
    "2e7f95d5-b279-4469-9c89-1e02630d61e6",
    "Wylie Beckert",
    crate::card::CardRules::unsupported(),
);

// OTJ 213 — Kellan, the Kid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KELLAN_THE_KID: CardRecord = CardRecord::new(
    "Kellan, the Kid",
    "04dfbc4c-ab21-45db-bbd9-b9d245d60015",
    "Magali Villeneuve",
    crate::card::CardRules::unsupported(),
);

// OTJ 214 — Kraum, Violent Cacophony
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KRAUM_VIOLENT_CACOPHONY: CardRecord = CardRecord::new(
    "Kraum, Violent Cacophony",
    "958a3e6b-7e20-40ea-8b2c-7c728934b5e5",
    "Artur Nakhodkin",
    crate::card::CardRules::unsupported(),
);

// OTJ 215 — Laughing Jasper Flint
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAUGHING_JASPER_FLINT: CardRecord = CardRecord::new(
    "Laughing Jasper Flint",
    "af0b3a41-ba99-41e8-bcfb-5796500c17c7",
    "Francis Tneh",
    crate::card::CardRules::unsupported(),
);

// OTJ 216 — Lazav, Familiar Stranger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAZAV_FAMILIAR_STRANGER: CardRecord = CardRecord::new(
    "Lazav, Familiar Stranger",
    "00293326-3eb2-492c-b565-7abafa037d8c",
    "Tyler Jacobson",
    crate::card::CardRules::unsupported(),
);

// OTJ 217 — Lilah, Undefeated Slickshot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LILAH_UNDEFEATED_SLICKSHOT: CardRecord = CardRecord::new(
    "Lilah, Undefeated Slickshot",
    "e21f90ea-5934-4757-8515-38ef116afac1",
    "Andreas Zafiratos",
    crate::card::CardRules::unsupported(),
);

// OTJ 218 — Make Your Own Luck
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAKE_YOUR_OWN_LUCK: CardRecord = CardRecord::new(
    "Make Your Own Luck",
    "0557b0a3-2b48-408f-a508-9f4da2ab1cd1",
    "Chris Seaman",
    crate::card::CardRules::unsupported(),
);

// OTJ 219 — Malcolm, the Eyes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MALCOLM_THE_EYES: CardRecord = CardRecord::new(
    "Malcolm, the Eyes",
    "521dffaa-813b-41e4-b7c2-a8c407167875",
    "Dmitry Burmak",
    crate::card::CardRules::unsupported(),
);

// OTJ 220 — Marchesa, Dealer of Death
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MARCHESA_DEALER_OF_DEATH: CardRecord = CardRecord::new(
    "Marchesa, Dealer of Death",
    "ee29b59c-d57c-4a03-bae7-e9dfa57d6bb1",
    "Ryan Pancoast",
    crate::card::CardRules::unsupported(),
);

// OTJ 221 — Miriam, Herd Whisperer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIRIAM_HERD_WHISPERER: CardRecord = CardRecord::new(
    "Miriam, Herd Whisperer",
    "bfa7750a-7c32-4413-b762-62e24d992c6b",
    "Viko Menezes",
    crate::card::CardRules::unsupported(),
);

// OTJ 222 — Obeka, Splitter of Seconds
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OBEKA_SPLITTER_OF_SECONDS: CardRecord = CardRecord::new(
    "Obeka, Splitter of Seconds",
    "03415c42-086e-4a2e-9be8-5cdcde83f134",
    "Ryan Pancoast",
    crate::card::CardRules::unsupported(),
);

// OTJ 223 — Oko, the Ringleader
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OKO_THE_RINGLEADER: CardRecord = CardRecord::new(
    "Oko, the Ringleader",
    "396df8d6-e85d-4486-8116-68841b7e1e2e",
    "Magali Villeneuve",
    crate::card::CardRules::unsupported(),
);

// OTJ 224 — Pillage the Bog
pub(in crate::card::sets) static PILLAGE_THE_BOG: CardRecord = CardRecord::new(
    "Pillage the Bog",
    "fa3b415f-7901-4ab4-84fe-60b90d40ac90",
    "Forrest Imel",
    // Two mana to find the one card the deck is built around, and plot is
    // what makes the two mana free: pay three on a turn with nothing to do,
    // and dig for nothing on the turn it matters.
    CardRules::new_sorcery(mana_cost!("{B}{G}")).with_abilities(&[
        AbilityDef::spell(
            "Look at the top X cards of your library, where X is twice the number of lands you \
             control. Put one of them into your hand and the rest on the bottom of your library \
             in a random order.",
            abilities::look_at_top_cards_choose_to_hand_rest_random_bottom(
                // "Twice the number of lands you control", which is what makes the card a
                // land-count payoff rather than a fixed dig: six lands look at twelve.
                ValueDef::Scaled(&ScaledValueDef::new(
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    2,
                )),
                ObjectPredicateDef::Any,
                1,
                1,
            ),
        ),
        abilities::plot(&[CostDef::Mana(mana_cost!("{1}{B}{G}"))]),
    ]),
);

// OTJ 225 — Rakdos Joins Up
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAKDOS_JOINS_UP: CardRecord = CardRecord::new(
    "Rakdos Joins Up",
    "c7154dca-7e10-4c34-aa56-9f200c6277d1",
    "Wylie Beckert",
    crate::card::CardRules::unsupported(),
);

// OTJ 226 — Rakdos, the Muscle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAKDOS_THE_MUSCLE: CardRecord = CardRecord::new(
    "Rakdos, the Muscle",
    "bb34babd-1b85-4a7d-a066-a8337805056e",
    "Victor Maury",
    crate::card::CardRules::unsupported(),
);

// OTJ 227 — Riku of Many Paths
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIKU_OF_MANY_PATHS: CardRecord = CardRecord::new(
    "Riku of Many Paths",
    "21b63544-4c31-4f38-9907-0407719a60b1",
    "Denys Tsiperko",
    crate::card::CardRules::unsupported(),
);

// OTJ 228 — Roxanne, Starfall Savant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROXANNE_STARFALL_SAVANT: CardRecord = CardRecord::new(
    "Roxanne, Starfall Savant",
    "11fbe52f-febd-49fc-8391-28d3efe9c3eb",
    "Ina Wong",
    crate::card::CardRules::unsupported(),
);

// OTJ 229 — Ruthless Lawbringer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUTHLESS_LAWBRINGER: CardRecord = CardRecord::new(
    "Ruthless Lawbringer",
    "927b5498-23f1-47c0-b441-7daaeb54f9b8",
    "Joshua Raphael",
    crate::card::CardRules::unsupported(),
);

// OTJ 230 — Satoru, the Infiltrator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SATORU_THE_INFILTRATOR: CardRecord = CardRecord::new(
    "Satoru, the Infiltrator",
    "acc9a5cc-2b3c-4c2f-8176-4a2d86265cc5",
    "Heonhwa",
    crate::card::CardRules::unsupported(),
);

// OTJ 231 — Selvala, Eager Trailblazer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SELVALA_EAGER_TRAILBLAZER: CardRecord = CardRecord::new(
    "Selvala, Eager Trailblazer",
    "7d2e167f-7cb2-4f15-a1db-7ee56b7ba523",
    "Viko Menezes",
    crate::card::CardRules::unsupported(),
);

// OTJ 232 — Seraphic Steed
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SERAPHIC_STEED: CardRecord = CardRecord::new(
    "Seraphic Steed",
    "7ada7ab4-0dee-4ff3-9817-1e61ca3f2ccf",
    "Jonas De Ro",
    crate::card::CardRules::unsupported(),
);

// OTJ 233 — Slick Sequence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SLICK_SEQUENCE: CardRecord = CardRecord::new(
    "Slick Sequence",
    "beb1c974-0d35-4e9f-a310-44eb2af64494",
    "Fajareka Setiawan",
    crate::card::CardRules::unsupported(),
);

// OTJ 234 — Taii Wakeen, Perfect Shot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TAII_WAKEEN_PERFECT_SHOT: CardRecord = CardRecord::new(
    "Taii Wakeen, Perfect Shot",
    "1643af0b-fcbf-4636-8c50-77ec77eaa34d",
    "David Auden Nash",
    crate::card::CardRules::unsupported(),
);

// OTJ 235 — Vial Smasher, Gleeful Grenadier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VIAL_SMASHER_GLEEFUL_GRENADIER: CardRecord = CardRecord::new(
    "Vial Smasher, Gleeful Grenadier",
    "3afce4e6-ac59-4fba-b63a-8fed96bbdc4a",
    "Borja Pindado",
    crate::card::CardRules::unsupported(),
);

// OTJ 236 — Vraska Joins Up
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VRASKA_JOINS_UP: CardRecord = CardRecord::new(
    "Vraska Joins Up",
    "06e546c2-737e-4b17-bf60-3069b1ccdf31",
    "Wylie Beckert",
    crate::card::CardRules::unsupported(),
);

// OTJ 237 — Vraska, the Silencer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VRASKA_THE_SILENCER: CardRecord = CardRecord::new(
    "Vraska, the Silencer",
    "b042abf2-c40b-4235-a4fa-2e4901c375c3",
    "Kieran Yanner",
    crate::card::CardRules::unsupported(),
);

// OTJ 238 — Wrangler of the Damned
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WRANGLER_OF_THE_DAMNED: CardRecord = CardRecord::new(
    "Wrangler of the Damned",
    "b4d163dd-67dc-4aab-afb9-d043352d109c",
    "Michal Ivan",
    crate::card::CardRules::unsupported(),
);

// OTJ 239 — Wylie Duke, Atiin Hero
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WYLIE_DUKE_ATIIN_HERO: CardRecord = CardRecord::new(
    "Wylie Duke, Atiin Hero",
    "bc97ffcf-4f51-44cd-8daa-a7dae4592ee5",
    "Ekaterina Burmak",
    crate::card::CardRules::unsupported(),
);

// OTJ 240 — Bandit's Haul
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BANDIT_S_HAUL: CardRecord = CardRecord::new(
    "Bandit's Haul",
    "68b2e74b-933b-4285-963b-dda3a986a914",
    "Monztre",
    crate::card::CardRules::unsupported(),
);

// OTJ 241 — Boom Box
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOOM_BOX: CardRecord = CardRecord::new(
    "Boom Box",
    "ea61d964-6d73-422e-9e08-360ac66f237a",
    "Caio Monteiro",
    crate::card::CardRules::unsupported(),
);

// OTJ 242 — Gold Pan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOLD_PAN: CardRecord = CardRecord::new(
    "Gold Pan",
    "dc098aae-3d9b-453b-a37e-e102f81a8311",
    "Gaboleps",
    crate::card::CardRules::unsupported(),
);

// OTJ 243 — Lavaspur Boots
pub(in crate::card::sets) static LAVASPUR_BOOTS: CardRecord = CardRecord::new(
    "Lavaspur Boots",
    "e50709de-e6ef-4dbc-af1e-290fed279f34",
    "Mila Pesic",
CardRules::new_artifact(mana_cost!("{1}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +1/+0 and has haste and ward {1}.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(0),
                        ),
                        AppliedEffectDef::add_ability(&abilities::haste()),
                        // Ward reads as one clause on the Boots, so the granted ability carries the
                        // whole of the printed reminder rather than a paraphrase of it.
                        AppliedEffectDef::add_ability(&abilities::ward(
                            &[CostDef::Mana(crate::ManaCost::new(1, 0))],
                            "Ward {1} (Whenever this creature becomes the target of a spell or ability an opponent \
                            controls, counter it unless that player pays {1}.)",
                        )),
                    ]),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{1}"))], "Equip {1}"),
        ]),
);

// OTJ 244 — Luxurious Locomotive
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LUXURIOUS_LOCOMOTIVE: CardRecord = CardRecord::new(
    "Luxurious Locomotive",
    "cc598338-eeba-4815-a0a6-ff2dc09790d2",
    "Leon Tukker",
    crate::card::CardRules::unsupported(),
);

// OTJ 245 — Mobile Homestead
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOBILE_HOMESTEAD: CardRecord = CardRecord::new(
    "Mobile Homestead",
    "d5fa82e4-0b77-498a-bec0-52764d24957a",
    "Artur Nakhodkin",
    crate::card::CardRules::unsupported(),
);

// OTJ 246 — Oasis Gardener
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OASIS_GARDENER: CardRecord = CardRecord::new(
    "Oasis Gardener",
    "ee0dc663-4bfb-46d4-af79-d0143c799487",
    "Kristina Carroll",
    crate::card::CardRules::unsupported(),
);

// OTJ 247 — Redrock Sentinel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REDROCK_SENTINEL: CardRecord = CardRecord::new(
    "Redrock Sentinel",
    "8eb98381-8418-4dfd-b7d1-4353570e611b",
    "Milivoj Ćeran",
    crate::card::CardRules::unsupported(),
);

// OTJ 248 — Silver Deputy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SILVER_DEPUTY: CardRecord = CardRecord::new(
    "Silver Deputy",
    "39d2c11d-1eb8-4768-bc61-fa8f20a69462",
    "Artur Nakhodkin",
    crate::card::CardRules::unsupported(),
);

// OTJ 249 — Sterling Hound
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STERLING_HOUND: CardRecord = CardRecord::new(
    "Sterling Hound",
    "9e5bfcf5-6e5c-47fe-af6c-6b18938261c6",
    "Leon Tukker",
    crate::card::CardRules::unsupported(),
);

// OTJ 250 — Tomb Trawler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOMB_TRAWLER: CardRecord = CardRecord::new(
    "Tomb Trawler",
    "36e61cb8-219a-4fe6-a2e6-307665ffa38f",
    "Anton Solovianchyk",
    crate::card::CardRules::unsupported(),
);

// OTJ 251 — Abraded Bluffs
pub(in crate::card::sets) static ABRADED_BLUFFS: CardRecord = CardRecord::new(
    "Abraded Bluffs",
    "19e96521-b4ce-4a36-a887-200e05ccc804",
    "Piotr Dura",
    // The red-white Desert; only the two colours below are its own.
    CardRules::new_land(&["Desert"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::desert_entry_ping(),
        AbilityDef::activated_mana(
            "{T}: Add {R} or {W}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Red,
                ManaColor::White,
            ])),
        ),
    ]),
);

// OTJ 252 — Arid Archway
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARID_ARCHWAY: CardRecord = CardRecord::new(
    "Arid Archway",
    "3f8c8fa2-12ab-4f6a-9f7a-2bc69e9ba024",
    "Raymond Bonilla",
    crate::card::CardRules::unsupported(),
);

// OTJ 253 — Bristling Backwoods
pub(in crate::card::sets) static BRISTLING_BACKWOODS: CardRecord = CardRecord::new(
    "Bristling Backwoods",
    "d61dfeb7-7f6b-4601-8396-2cbb98165489",
    "Viko Menezes",
    // A tapped dual that pays a point of damage for the tempo, and a Desert
    // for whatever cares about that.
    CardRules::new_land(&["Desert"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::desert_entry_ping(),
        AbilityDef::activated_mana(
            "{T}: Add {R} or {G}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Red,
                ManaColor::Green,
            ])),
        ),
    ]),
);

// OTJ 254 — Conduit Pylons
pub(in crate::card::sets) static CONDUIT_PYLONS: CardRecord = CardRecord::new(
    "Conduit Pylons",
    "5ffa48cc-b991-4d47-b7ec-cf678915c758",
    "Raymond Bonilla",
    // Untapped and colourless by default, so the fixing costs a mana rather
    // than a turn: the Desert deck plays it as a land that is never dead.
    CardRules::new_land(&["Desert"]).with_abilities(&[
        abilities::enters_trigger(
            "When this land enters, surveil 1. (Look at the top card of your library. You may put \
             it into your graveyard.)",
            abilities::surveil(ValueDef::Constant(1)),
        ),
        AbilityDef::activated_mana(
            "{T}: Add {C}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
        ),
        AbilityDef::activated_mana(
            "{1}, {T}: Add one mana of any color.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// OTJ 255 — Creosote Heath
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CREOSOTE_HEATH: CardRecord = CardRecord::new(
    "Creosote Heath",
    "c5523dac-7aa0-4486-89c8-3b22a1411f26",
    "Leon Tukker",
    crate::card::CardRules::unsupported(),
);

// OTJ 256 — Eroded Canyon
pub(in crate::card::sets) static ERODED_CANYON: CardRecord = CardRecord::new(
    "Eroded Canyon",
    "5c9d080f-28d7-41d6-a4e0-5b3e3a5ed770",
    "Piotr Dura",
    // The blue-red Desert; only the two colours below are its own.
    CardRules::new_land(&["Desert"]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::desert_entry_ping(),
        AbilityDef::activated_mana(
            "{T}: Add {U} or {R}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[ManaColor::Blue, ManaColor::Red])),
        ),
    ]),
);

// OTJ 257 — Festering Gulch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FESTERING_GULCH: CardRecord = CardRecord::new(
    "Festering Gulch",
    "4ad841eb-da0d-43d4-8b60-efe30922990b",
    "Daniel Romanovsky",
    crate::card::CardRules::unsupported(),
);

// OTJ 258 — Forlorn Flats
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORLORN_FLATS: CardRecord = CardRecord::new(
    "Forlorn Flats",
    "963c100e-4e12-438f-b5ae-14391406dff6",
    "Robin Olausson",
    crate::card::CardRules::unsupported(),
);

// OTJ 259 — Jagged Barrens
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JAGGED_BARRENS: CardRecord = CardRecord::new(
    "Jagged Barrens",
    "5d809f5b-d965-4cb1-a9f8-2048f8534373",
    "Leonardo Borazio",
    crate::card::CardRules::unsupported(),
);

// OTJ 260 — Lonely Arroyo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LONELY_ARROYO: CardRecord = CardRecord::new(
    "Lonely Arroyo",
    "4b778b63-e5fc-4d63-a93b-4372f32cade2",
    "Josu Hernaiz",
    crate::card::CardRules::unsupported(),
);

// OTJ 261 — Lush Oasis
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LUSH_OASIS: CardRecord = CardRecord::new(
    "Lush Oasis",
    "988e44c5-4632-4ebb-b6ae-c3886e49d637",
    "Piotr Dura",
    crate::card::CardRules::unsupported(),
);

// OTJ 262 — Mirage Mesa
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIRAGE_MESA: CardRecord = CardRecord::new(
    "Mirage Mesa",
    "c3d2e816-c06d-4c5d-98fe-c350d8cfab27",
    "Andrew Mar",
    crate::card::CardRules::unsupported(),
);

// OTJ 263 — Sandstorm Verge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SANDSTORM_VERGE: CardRecord = CardRecord::new(
    "Sandstorm Verge",
    "3ab4e0a4-2faf-456b-99e3-ee06c008538c",
    "Jorge Jacinto",
    crate::card::CardRules::unsupported(),
);

// OTJ 264 — Soured Springs
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOURED_SPRINGS: CardRecord = CardRecord::new(
    "Soured Springs",
    "67daa31c-d9c4-4c22-b29c-1b8a17d577e5",
    "Leonardo Borazio",
    crate::card::CardRules::unsupported(),
);

// OTJ 265 — Bucolic Ranch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BUCOLIC_RANCH: CardRecord = CardRecord::new(
    "Bucolic Ranch",
    "6c4f6b81-53d0-49fb-b404-c2ad67de7493",
    "Leonardo Borazio",
    crate::card::CardRules::unsupported(),
);

// OTJ 266 — Blooming Marsh (reprint)
const BLOOMING_MARSH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_kld::BLOOMING_MARSH,
    "861caabb-0573-4e94-8b03-342f90465064",
    "Yeong-Hao Han",
);

// OTJ 267 — Botanical Sanctum (reprint)
const BOTANICAL_SANCTUM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_kld::BOTANICAL_SANCTUM,
    "cc18d5f4-a56a-4f7d-9f56-ccc92cbfb7f7",
    "Jorge Jacinto",
);

// OTJ 268 — Concealed Courtyard (reprint)
const CONCEALED_COURTYARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_kld::CONCEALED_COURTYARD,
    "b75df1f0-0513-40e4-a449-454f75de6434",
    "Rockey Chen",
);

// OTJ 269 — Inspiring Vantage (reprint)
const INSPIRING_VANTAGE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_kld::INSPIRING_VANTAGE,
    "85df6b6a-2dcf-4828-a4a8-e07d52e1fddd",
    "Volkan Baǵa",
);

// OTJ 270 — Spirebluff Canal (reprint)
const SPIREBLUFF_CANAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_kld::SPIREBLUFF_CANAL,
    "59a04e16-a767-4112-ab01-6ca1b09c286c",
    "Ron Spears",
);

// OTJ 271 — Jace Reawakened
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JACE_REAWAKENED: CardRecord = CardRecord::new(
    "Jace Reawakened",
    "fd17e8d4-499e-4005-ae3c-bc9c44dc5a67",
    "Cristi Balanescu",
    crate::card::CardRules::unsupported(),
);

// OTJ 272 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAINS,
    "cfe51d97-66f6-4ac3-b926-01ab7e4c5686",
    "Salvatorre Zee Yazzie",
);

// OTJ 273 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND,
    "a624d656-207d-4e73-b615-59e7cf64ad64",
    "Salvatorre Zee Yazzie",
);

// OTJ 274 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWAMP,
    "3b383b16-8128-4d9e-a0d0-9b8ccc9ad6df",
    "Salvatorre Zee Yazzie",
);

// OTJ 275 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MOUNTAIN,
    "0a7dbfd2-cda7-4fc9-9677-e442fb5f5f6f",
    "Salvatorre Zee Yazzie",
);

// OTJ 276 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOREST,
    "baf8a774-65f3-431e-b084-328ff1000895",
    "Salvatorre Zee Yazzie",
);

// OTJ 277 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    1,
    "6f501773-1b39-4a4c-9b45-a950385c9e82",
    "Sergey Glushakov",
);

// OTJ 278 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    2,
    "fe60da77-084c-49e8-9948-9ac4b6a6382f",
    "Adam Paquette",
);

// OTJ 279 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    1,
    "acd6be3f-745c-41ad-95c9-1db66ba56be2",
    "Sergey Glushakov",
);

// OTJ 280 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    2,
    "91be4db0-7cb3-4202-939b-cb7a26e90019",
    "Adam Paquette",
);

// OTJ 281 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    1,
    "eb7dc259-9949-4673-a8f1-874396948392",
    "Sergey Glushakov",
);

// OTJ 282 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    2,
    "5c2c9dc0-7f3a-4f3a-ba08-5c2f87e252bc",
    "Adam Paquette",
);

// OTJ 283 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    1,
    "9137b4aa-2289-4a4d-b1f5-ae75a0928278",
    "Sergey Glushakov",
);

// OTJ 284 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    2,
    "2237ee9b-fff6-472c-903c-11faf9bb116d",
    "Adam Paquette",
);

// OTJ 285 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    1,
    "dc20a07e-5f92-49c2-9c97-d5eda536d3f6",
    "Sergey Glushakov",
);

// OTJ 286 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    2,
    "8e9ac507-7c8f-431f-8d1a-220ceeacf871",
    "Adam Paquette",
);

// OTJ 287 — Geralf, the Fleshwright (alternate printing)
const GERALF_THE_FLESHWRIGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GERALF_THE_FLESHWRIGHT,
    1,
    "6ef3f55e-b8e6-4ef1-85e4-2aef5afc15ab",
    "Pedro Potier",
);

// OTJ 288 — Gisa, the Hellraiser (alternate printing)
const GISA_THE_HELLRAISER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GISA_THE_HELLRAISER,
    1,
    "37c1ad56-cf6e-4717-a56e-feeb7339b8c3",
    "Greg Staples",
);

// OTJ 289 — Kaervek, the Punisher (alternate printing)
const KAERVEK_THE_PUNISHER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KAERVEK_THE_PUNISHER,
    1,
    "95e759e4-bda5-4d01-a9a8-3986df1d4428",
    "Pedro Potier",
);

// OTJ 290 — Tinybones, the Pickpocket (alternate printing)
const TINYBONES_THE_PICKPOCKET_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TINYBONES_THE_PICKPOCKET,
    1,
    "1b2a3962-315d-48df-9af5-7cb4dd2040de",
    "Michael Walsh",
);

// OTJ 291 — Annie Flash, the Veteran (alternate printing)
const ANNIE_FLASH_THE_VETERAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ANNIE_FLASH_THE_VETERAN,
    1,
    "0ea8d21f-a003-4711-ad5c-0bde87e1edc6",
    "Justine Mara Andersen",
);

// OTJ 292 — Breeches, the Blastmaker (alternate printing)
const BREECHES_THE_BLASTMAKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BREECHES_THE_BLASTMAKER,
    1,
    "fcde5cd5-4ce6-4b94-8d97-534a647dbfc1",
    "Michael Walsh",
);

// OTJ 293 — Eriette, the Beguiler (alternate printing)
const ERIETTE_THE_BEGUILER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ERIETTE_THE_BEGUILER,
    1,
    "f69b5791-a405-4578-acba-959cc181e9ad",
    "Dibujante Nocturno",
);

// OTJ 294 — Kellan, the Kid (alternate printing)
const KELLAN_THE_KID_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KELLAN_THE_KID,
    1,
    "789659f6-af39-4357-985f-a006f192893c",
    "Benjamin Ee",
);

// OTJ 295 — Malcolm, the Eyes (alternate printing)
const MALCOLM_THE_EYES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MALCOLM_THE_EYES,
    1,
    "480de469-3fac-4a2b-8dec-7899ce00551e",
    "Michael Walsh",
);

// OTJ 296 — Oko, the Ringleader (alternate printing)
const OKO_THE_RINGLEADER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OKO_THE_RINGLEADER,
    1,
    "9c025072-0d0a-4ca7-a386-f1ed7c97638b",
    "Dibujante Nocturno",
);

// OTJ 297 — Rakdos, the Muscle (alternate printing)
const RAKDOS_THE_MUSCLE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RAKDOS_THE_MUSCLE,
    1,
    "b85635e0-fb5b-42ea-8a30-d67922cbca95",
    "Greg Staples",
);

// OTJ 298 — Satoru, the Infiltrator (alternate printing)
const SATORU_THE_INFILTRATOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SATORU_THE_INFILTRATOR,
    1,
    "4b9a76f0-b697-4c1d-9b60-c66f7fc4316e",
    "Denis Medri",
);

// OTJ 299 — Vraska, the Silencer (alternate printing)
const VRASKA_THE_SILENCER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VRASKA_THE_SILENCER,
    1,
    "2a6cc9ab-a1d8-47de-8ac5-112e5fee00f9",
    "Jarel Threat",
);

// OTJ 300 — Blooming Marsh (alternate printing)
const BLOOMING_MARSH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_kld::BLOOMING_MARSH,
    1,
    "1cfb4a80-3319-41ad-9d97-a9a53c9f84fb",
    "Piotr Dura",
);

// OTJ 301 — Botanical Sanctum (alternate printing)
const BOTANICAL_SANCTUM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_kld::BOTANICAL_SANCTUM,
    1,
    "82670072-9851-48c7-a8f5-7842bff6c252",
    "Piotr Dura",
);

// OTJ 302 — Concealed Courtyard (alternate printing)
const CONCEALED_COURTYARD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_kld::CONCEALED_COURTYARD,
    1,
    "d7e85ace-7e2c-46f6-adb9-07f33f8e1750",
    "Piotr Dura",
);

// OTJ 303 — Inspiring Vantage (alternate printing)
const INSPIRING_VANTAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_kld::INSPIRING_VANTAGE,
    1,
    "551e216d-a82f-49a1-a3fd-8165715a05c4",
    "Piotr Dura",
);

// OTJ 304 — Spirebluff Canal (alternate printing)
const SPIREBLUFF_CANAL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_kld::SPIREBLUFF_CANAL,
    1,
    "9d83bdb3-1f04-4b99-a36b-312da0cbed50",
    "Piotr Dura",
);

// OTJ 305 — Oko, the Ringleader (alternate printing)
const OKO_THE_RINGLEADER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &OKO_THE_RINGLEADER,
    2,
    "0bfbb249-f2cc-4295-b6e3-4fd7e1eac183",
    "Lie Setiawan",
);

// OTJ 306 — Jace Reawakened (alternate printing)
const JACE_REAWAKENED_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &JACE_REAWAKENED,
    1,
    "d73bdf79-6c18-46bc-a8bb-97e07dd23aff",
    "Chris Rallis",
);

// OTJ 307 — Another Round (alternate printing)
const ANOTHER_ROUND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ANOTHER_ROUND,
    1,
    "ddec615b-537a-4fef-a2a7-9bdf74c0192a",
    "Darrell Riche",
);

// OTJ 308 — Archangel of Tithes (alternate printing)
const ARCHANGEL_OF_TITHES_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_ori::ARCHANGEL_OF_TITHES,
    1,
    "d34f043c-d01e-4462-b381-39748d7fa31b",
    "Denys Tsiperko",
);

// OTJ 309 — Aven Interrupter (alternate printing)
const AVEN_INTERRUPTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AVEN_INTERRUPTER,
    1,
    "75143fdb-5651-4289-86df-76c9655a0599",
    "Daniel Romanovsky",
);

// OTJ 310 — Claim Jumper (alternate printing)
const CLAIM_JUMPER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CLAIM_JUMPER,
    1,
    "dfad3c84-4264-4757-8e83-25dbbed67070",
    "Gaboleps",
);

// OTJ 311 — Dust Animus (alternate printing)
const DUST_ANIMUS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DUST_ANIMUS,
    1,
    "d0d9d4f0-48ec-43e6-bb93-a9589790417b",
    "Uriah Voth",
);

// OTJ 312 — Final Showdown (alternate printing)
const FINAL_SHOWDOWN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FINAL_SHOWDOWN,
    1,
    "94397320-b814-487f-aca2-537517ff9eff",
    "Izzy",
);

// OTJ 313 — Fortune, Loyal Steed (alternate printing)
const FORTUNE_LOYAL_STEED_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FORTUNE_LOYAL_STEED,
    1,
    "02ffb299-a327-43c1-868e-1c5225204956",
    "Artur Nakhodkin",
);

// OTJ 314 — High Noon (alternate printing)
const HIGH_NOON_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HIGH_NOON,
    1,
    "0d69fce2-3307-4518-a196-e0b8155dca73",
    "Eduardo Francisco",
);

// OTJ 315 — One Last Job (alternate printing)
const ONE_LAST_JOB_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ONE_LAST_JOB,
    1,
    "ad6db7ba-3150-4ae8-84e3-11661bf1d1c4",
    "Caroline Gariba",
);

// OTJ 316 — Archmage's Newt (alternate printing)
const ARCHMAGE_S_NEWT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ARCHMAGE_S_NEWT,
    1,
    "25563be1-71f1-4f70-8527-02c5855a0b9d",
    "Edgar Sánchez Hidalgo",
);

// OTJ 317 — Double Down (alternate printing)
const DOUBLE_DOWN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DOUBLE_DOWN,
    1,
    "d6f24b8b-f9ed-4c77-8cb0-2a94848ee69b",
    "Javier Charro",
);

// OTJ 318 — Duelist of the Mind (alternate printing)
const DUELIST_OF_THE_MIND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DUELIST_OF_THE_MIND,
    1,
    "4126ae45-00b3-419c-8f07-d1286ac6b121",
    "Darren Tan",
);

// OTJ 319 — Fblthp, Lost on the Range (alternate printing)
const FBLTHP_LOST_ON_THE_RANGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FBLTHP_LOST_ON_THE_RANGE,
    1,
    "51bbf861-448b-455b-8922-38515ba65c40",
    "Brian Valeza",
);

// OTJ 320 — The Key to the Vault (alternate printing)
const THE_KEY_TO_THE_VAULT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_KEY_TO_THE_VAULT,
    1,
    "d70388ee-2f97-4282-ba74-af95462ac0aa",
    "Leon Tukker",
);

// OTJ 321 — Step Between Worlds (alternate printing)
const STEP_BETWEEN_WORLDS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STEP_BETWEEN_WORLDS,
    1,
    "0e532e5b-6d84-4669-8788-f471b1498c7b",
    "Chris Ostrowski",
);

// OTJ 322 — Stoic Sphinx (alternate printing)
const STOIC_SPHINX_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STOIC_SPHINX,
    1,
    "e346fab1-48db-4cd2-836d-dad5e8437308",
    "Andreas Zafiratos",
);

// OTJ 323 — Three Steps Ahead (alternate printing)
const THREE_STEPS_AHEAD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THREE_STEPS_AHEAD,
    1,
    "be980c3b-b4ab-4ed7-9f61-d8db54c226d9",
    "Francisco Miyara",
);

// OTJ 324 — Caustic Bronco (alternate printing)
const CAUSTIC_BRONCO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CAUSTIC_BRONCO,
    1,
    "517505a0-0d05-4e81-8582-999b88040f48",
    "Brent Hollowell",
);

// OTJ 325 — Insatiable Avarice (alternate printing)
const INSATIABLE_AVARICE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &INSATIABLE_AVARICE,
    1,
    "aae03d91-8269-4124-ba50-a6c65f47718b",
    "Scott Murphy",
);

// OTJ 326 — Pitiless Carnage (alternate printing)
const PITILESS_CARNAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PITILESS_CARNAGE,
    1,
    "2bea6391-e70d-4f36-86b7-fd08440b4976",
    "Richard Kane Ferguson",
);

// OTJ 327 — Rush of Dread (alternate printing)
const RUSH_OF_DREAD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RUSH_OF_DREAD,
    1,
    "e4ca7ac5-5552-487f-9ac9-1092fe6cd165",
    "Chris Seaman",
);

// OTJ 328 — Tinybones Joins Up (alternate printing)
const TINYBONES_JOINS_UP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TINYBONES_JOINS_UP,
    1,
    "a3afc0c2-95e7-4b9f-94c5-144cce17c7ed",
    "Wylie Beckert",
);

// OTJ 329 — Vadmir, New Blood (alternate printing)
const VADMIR_NEW_BLOOD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VADMIR_NEW_BLOOD,
    1,
    "d876a888-e119-494d-857a-e780a277c817",
    "Andreas Zafiratos",
);

// OTJ 330 — Calamity, Galloping Inferno (alternate printing)
const CALAMITY_GALLOPING_INFERNO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CALAMITY_GALLOPING_INFERNO,
    1,
    "295e63c2-b533-4dbf-8c8a-c6493de31457",
    "Artur Nakhodkin",
);

// OTJ 331 — Great Train Heist (alternate printing)
const GREAT_TRAIN_HEIST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GREAT_TRAIN_HEIST,
    1,
    "b570d9c5-3348-482f-a211-ed8c9777a9fa",
    "Campbell White",
);

// OTJ 332 — Hell to Pay (alternate printing)
const HELL_TO_PAY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HELL_TO_PAY,
    1,
    "c522c232-0f86-49ad-969f-a2b086432a97",
    "Liiga Smilshkalne",
);

// OTJ 333 — Hellspur Posse Boss (alternate printing)
const HELLSPUR_POSSE_BOSS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HELLSPUR_POSSE_BOSS,
    1,
    "259262b9-f74b-4e71-9c4e-cf18ba2dc3c2",
    "Artur Nakhodkin",
);

// OTJ 334 — Magda, the Hoardmaster (alternate printing)
const MAGDA_THE_HOARDMASTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MAGDA_THE_HOARDMASTER,
    1,
    "adf42a57-02f0-4a3f-8677-d68ffe3090c0",
    "Diego Gisbert",
);

// OTJ 335 — Slickshot Show-Off
pub(in crate::card::sets) static SLICKSHOT_SHOW_OFF: CardRecord = CardRecord::new(
    "Slickshot Show-Off",
    "304523e7-f332-4c1d-9590-ff9a70daff26",
    "Augusto Quirino",
    // Two mana for a hasty flier that grows with every spell after it, and
    // a plot cost that pays the two a turn early so the whole of a later
    // turn's mana can go into the spells it grows on.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Bird", "Wizard"], 1, 2).with_abilities(&[
        abilities::flying(),
        abilities::haste(),
        AbilityDef::triggered(
            "Whenever you cast a noncreature spell, this creature gets +2/+0 until end of turn.",
            // A noncreature spell you cast, which is prowess with a bigger number and
            // no toughness: what the Bird wants is one turn with several spells in it.
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::plot(&[CostDef::Mana(mana_cost!("{1}{R}"))]),
    ]),
);

// OTJ 336 — Stingerback Terror (alternate printing)
const STINGERBACK_TERROR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STINGERBACK_TERROR,
    1,
    "2e9fa00b-bc34-4be8-a21f-11ae695f166d",
    "Slawomir Maniak",
);

// OTJ 337 — Terror of the Peaks (alternate printing)
const TERROR_OF_THE_PEAKS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_m21::TERROR_OF_THE_PEAKS,
    1,
    "cd20aae9-9de0-498b-8325-f8e8290d56e3",
    "Joshua Raphael",
);

// OTJ 338 — Bristly Bill, Spine Sower (alternate printing)
const BRISTLY_BILL_SPINE_SOWER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BRISTLY_BILL_SPINE_SOWER,
    1,
    "f921e711-168d-4836-bca2-f9bb43992708",
    "Daniel Zrom",
);

// OTJ 339 — Colossal Rattlewurm (alternate printing)
const COLOSSAL_RATTLEWURM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &COLOSSAL_RATTLEWURM,
    1,
    "e6fbb0c7-29ac-4394-87e1-6e8227602aac",
    "Filip Burburan",
);

// OTJ 340 — Freestrider Lookout (alternate printing)
const FREESTRIDER_LOOKOUT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FREESTRIDER_LOOKOUT,
    1,
    "81289833-ebdb-4fe7-ad17-6e39eb399e69",
    "Matt Zeilinger",
);

// OTJ 341 — Goldvein Hydra (alternate printing)
const GOLDVEIN_HYDRA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GOLDVEIN_HYDRA,
    1,
    "8700b3a6-327b-4dc0-aa8d-434ca971a7b9",
    "David Auden Nash",
);

// OTJ 342 — Ornery Tumblewagg (alternate printing)
const ORNERY_TUMBLEWAGG_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ORNERY_TUMBLEWAGG,
    1,
    "3a1729d3-8e27-4db0-a7c6-e629a8faf946",
    "Izzy",
);

// OTJ 343 — Outcaster Trailblazer (alternate printing)
const OUTCASTER_TRAILBLAZER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OUTCASTER_TRAILBLAZER,
    1,
    "8cf1fe32-eac2-4d50-b403-25c3666f6d80",
    "Denys Tsiperko",
);

// OTJ 344 — Railway Brawler (alternate printing)
const RAILWAY_BRAWLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RAILWAY_BRAWLER,
    1,
    "78db8c96-e6b8-4d4a-9935-29f12385a151",
    "Kevin Sidharta",
);

// OTJ 345 — Smuggler's Surprise (alternate printing)
const SMUGGLER_S_SURPRISE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SMUGGLER_S_SURPRISE,
    1,
    "41b76d0d-da55-44ec-add8-61696ee40d21",
    "Jonas De Ro",
);

// OTJ 346 — Akul the Unrepentant (alternate printing)
const AKUL_THE_UNREPENTANT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &AKUL_THE_UNREPENTANT,
    1,
    "6534bdb1-9436-4919-8a8b-5401bc070fc2",
    "Kekai Kotaki",
);

// OTJ 347 — Annie Joins Up (alternate printing)
const ANNIE_JOINS_UP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ANNIE_JOINS_UP,
    1,
    "9ce19341-e652-448f-8f14-0439bd8fa385",
    "Wylie Beckert",
);

// OTJ 348 — Assimilation Aegis (alternate printing)
const ASSIMILATION_AEGIS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ASSIMILATION_AEGIS,
    1,
    "0d8e4f7b-bdf6-44d3-9b2b-4420bab84864",
    "Matt Stewart",
);

// OTJ 349 — Bonny Pall, Clearcutter (alternate printing)
const BONNY_PALL_CLEARCUTTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BONNY_PALL_CLEARCUTTER,
    1,
    "ee29f20b-27cd-433a-8874-41f500720109",
    "Bryan Sola",
);

// OTJ 350 — Bruse Tarl, Roving Rancher (alternate printing)
const BRUSE_TARL_ROVING_RANCHER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BRUSE_TARL_ROVING_RANCHER,
    1,
    "026257bf-f4e7-45f8-9c93-7248f201c583",
    "Forrest Imel",
);

// OTJ 351 — Ghired, Mirror of the Wilds (alternate printing)
const GHIRED_MIRROR_OF_THE_WILDS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GHIRED_MIRROR_OF_THE_WILDS,
    1,
    "203c84a3-5f1a-440f-93e7-bb65d1e07886",
    "Diego Gisbert",
);

// OTJ 352 — The Gitrog, Ravenous Ride (alternate printing)
const THE_GITROG_RAVENOUS_RIDE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_GITROG_RAVENOUS_RIDE,
    1,
    "a634b934-710b-4c83-b769-d8a034e297b8",
    "Johan Grenier",
);

// OTJ 353 — Kambal, Profiteering Mayor (alternate printing)
const KAMBAL_PROFITEERING_MAYOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KAMBAL_PROFITEERING_MAYOR,
    1,
    "8cfd5a0f-8859-4272-a08f-13b788422f4c",
    "Andreas Zafiratos",
);

// OTJ 354 — Kellan Joins Up (alternate printing)
const KELLAN_JOINS_UP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KELLAN_JOINS_UP,
    1,
    "1d927dc7-77b7-473d-b50b-e81c52d66b55",
    "Wylie Beckert",
);

// OTJ 355 — Laughing Jasper Flint (alternate printing)
const LAUGHING_JASPER_FLINT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LAUGHING_JASPER_FLINT,
    1,
    "5df7fadd-33ea-4bf4-9122-362821ef96dc",
    "Francis Tneh",
);

// OTJ 356 — Lilah, Undefeated Slickshot (alternate printing)
const LILAH_UNDEFEATED_SLICKSHOT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LILAH_UNDEFEATED_SLICKSHOT,
    1,
    "95efd401-579e-448b-9655-31311c0ae41e",
    "Andreas Zafiratos",
);

// OTJ 357 — Marchesa, Dealer of Death (alternate printing)
const MARCHESA_DEALER_OF_DEATH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MARCHESA_DEALER_OF_DEATH,
    1,
    "724309cd-fb05-4632-a7be-33a9ba024e4a",
    "Ryan Pancoast",
);

// OTJ 358 — Obeka, Splitter of Seconds (alternate printing)
const OBEKA_SPLITTER_OF_SECONDS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OBEKA_SPLITTER_OF_SECONDS,
    1,
    "183b8c06-62ab-41e8-82c1-f23066d832ee",
    "Ryan Pancoast",
);

// OTJ 359 — Pillage the Bog (alternate printing)
const PILLAGE_THE_BOG_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PILLAGE_THE_BOG,
    1,
    "c05939b7-1877-4464-98fe-d3b9ae754fb9",
    "Forrest Imel",
);

// OTJ 360 — Rakdos Joins Up (alternate printing)
const RAKDOS_JOINS_UP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RAKDOS_JOINS_UP,
    1,
    "761a743a-6358-4e08-a6b7-6eaaf7ab9ddc",
    "Wylie Beckert",
);

// OTJ 361 — Riku of Many Paths (alternate printing)
const RIKU_OF_MANY_PATHS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RIKU_OF_MANY_PATHS,
    1,
    "215b4632-adce-4357-bfcf-b6014ed2a24b",
    "Denys Tsiperko",
);

// OTJ 362 — Roxanne, Starfall Savant (alternate printing)
const ROXANNE_STARFALL_SAVANT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ROXANNE_STARFALL_SAVANT,
    1,
    "7b8aea8d-452a-4b23-bc78-8c7db54610d3",
    "Ina Wong",
);

// OTJ 363 — Selvala, Eager Trailblazer (alternate printing)
const SELVALA_EAGER_TRAILBLAZER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SELVALA_EAGER_TRAILBLAZER,
    1,
    "e523de28-b2d5-4be9-be78-883cef2499e9",
    "Viko Menezes",
);

// OTJ 364 — Seraphic Steed (alternate printing)
const SERAPHIC_STEED_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SERAPHIC_STEED,
    1,
    "1c9df2d7-6738-496c-8164-8bdf7d011df3",
    "Jonas De Ro",
);

// OTJ 365 — Taii Wakeen, Perfect Shot (alternate printing)
const TAII_WAKEEN_PERFECT_SHOT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TAII_WAKEEN_PERFECT_SHOT,
    1,
    "3c558349-87bc-4e0f-96c2-b075f7da97d5",
    "David Auden Nash",
);

// OTJ 366 — Vraska Joins Up (alternate printing)
const VRASKA_JOINS_UP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VRASKA_JOINS_UP,
    1,
    "43579701-c89a-4455-8f6a-0589af0c6bd3",
    "Wylie Beckert",
);

// OTJ 367 — Wylie Duke, Atiin Hero (alternate printing)
const WYLIE_DUKE_ATIIN_HERO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WYLIE_DUKE_ATIIN_HERO,
    1,
    "17955d83-89ea-4151-950c-d72acf1a852a",
    "Ekaterina Burmak",
);

// OTJ 368 — Frontier Seeker (alternate printing)
const FRONTIER_SEEKER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FRONTIER_SEEKER,
    1,
    "5fd49911-9ddb-4bef-8cd0-89f558ccd5cc",
    "Raluca Marinescu",
);

// OTJ 369 — Scorching Shot (alternate printing)
const SCORCHING_SHOT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SCORCHING_SHOT,
    1,
    "188694af-b856-4439-8ce0-8306b38caacb",
    "Caio Monteiro",
);

// OTJ 370 — Honest Rutstein (alternate printing)
const HONEST_RUTSTEIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HONEST_RUTSTEIN,
    1,
    "d8df8b87-3ea1-44eb-9218-9e7e06dfd0db",
    "Javier Charro",
);

// OTJ 371 — Make Your Own Luck (alternate printing)
const MAKE_YOUR_OWN_LUCK_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MAKE_YOUR_OWN_LUCK,
    1,
    "d9073963-5866-4132-a059-a46f96cd2a8d",
    "Chris Seaman",
);

// OTJ 372 — Ruthless Lawbringer (alternate printing)
const RUTHLESS_LAWBRINGER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RUTHLESS_LAWBRINGER,
    1,
    "f19e6995-4926-4513-b8c1-f35a22aafbe5",
    "Joshua Raphael",
);

// OTJ 373 — The Key to the Vault (alternate printing)
const THE_KEY_TO_THE_VAULT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THE_KEY_TO_THE_VAULT,
    2,
    "e695db51-f9d5-4eef-84d9-62f7602792b4",
    "Eli Minaya",
);

// OTJ 374 — Magda, the Hoardmaster (alternate printing)
const MAGDA_THE_HOARDMASTER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MAGDA_THE_HOARDMASTER,
    2,
    "2834e84e-e932-4052-9dad-2e1c8e76fbbc",
    "Loïc Canavaggia",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ANOTHER_ROUND,
    &ARMORED_ARMADILLO,
    &AVEN_INTERRUPTER,
    &BOUNDING_FELIDAR,
    &BOVINE_INTERVENTION,
    &BRIDLED_BIGHORN,
    &CLAIM_JUMPER,
    &DUST_ANIMUS,
    &ERIETTE_S_LULLABY,
    &FINAL_SHOWDOWN,
    &FORTUNE_LOYAL_STEED,
    &FRONTIER_SEEKER,
    &GETAWAY_GLAMER,
    &HIGH_NOON,
    &HOLY_COW,
    &INVENTIVE_WINGSMITH,
    &LASSOED_BY_THE_LAW,
    &MYSTICAL_TETHER,
    &NURTURING_PIXIE,
    &OMENPORT_VIGILANTE,
    &ONE_LAST_JOB,
    &OUTLAW_MEDIC,
    &PRAIRIE_DOG,
    &PROSPERITY_TYCOON,
    &REQUISITION_RAID,
    &RUSTLER_RAMPAGE,
    &SHEPHERD_OF_THE_CLOUDS,
    &SHERIFF_OF_SAFE_PASSAGE,
    &STAGECOACH_SECURITY,
    &STEER_CLEAR,
    &STERLING_KEYKEEPER,
    &STERLING_SUPPLIER,
    &THUNDER_LASSO,
    &TRAINED_ARYNX,
    &VENGEFUL_TOWNSFOLK,
    &WANTED_GRIFFIN,
    &ARCHMAGE_S_NEWT,
    &CANYON_CRAB,
    &DARING_THUNDER_THIEF,
    &DEEPMUCK_DESPERADO,
    &DJINN_OF_FOOL_S_FALL,
    &DOUBLE_DOWN,
    &DUELIST_OF_THE_MIND,
    &EMERGENT_HAUNTING,
    &FAILED_FORDING,
    &FBLTHP_LOST_ON_THE_RANGE,
    &FLEETING_REFLECTION,
    &GERALF_THE_FLESHWRIGHT,
    &GEYSER_DRAKE,
    &HARRIER_STRIX,
    &JAILBREAK_SCHEME,
    &THE_KEY_TO_THE_VAULT,
    &LOAN_SHARK,
    &MARAUDING_SPHINX,
    &METAMORPHIC_BLAST,
    &NIMBLE_BRIGAND,
    &OUTLAW_STITCHER,
    &PEERLESS_ROPEMASTER,
    &PHANTOM_INTERFERENCE,
    &PLAN_THE_HEIST,
    &RAZZLE_DAZZLER,
    &SEIZE_THE_SECRETS,
    &SHACKLE_SLINGER,
    &SHIFTING_GRIFT,
    &SLICKSHOT_LOCKPICKER,
    &SLICKSHOT_VAULT_BUSTER,
    &SPRING_SPLASHER,
    &STEP_BETWEEN_WORLDS,
    &STOIC_SPHINX,
    &STOP_COLD,
    &TAKE_THE_FALL,
    &THIS_TOWN_AIN_T_BIG_ENOUGH,
    &THREE_STEPS_AHEAD,
    &VISAGE_BANDIT,
    &AMBUSH_GIGAPEDE,
    &BINDING_NEGOTIATION,
    &BLACKSNAG_BUZZARD,
    &BLOOD_HUSTLER,
    &BONEYARD_DESECRATOR,
    &CAUSTIC_BRONCO,
    &CONSUMING_ASHES,
    &DESERT_S_DUE,
    &DESPERATE_BLOODSEEKER,
    &FORSAKEN_MINER,
    &GISA_THE_HELLRAISER,
    &HOLLOW_MARAUDER,
    &INSATIABLE_AVARICE,
    &KAERVEK_THE_PUNISHER,
    &LIVELY_DIRGE,
    &MOURNER_S_SURPRISE,
    &NEUTRALIZE_THE_GUARDS,
    &NEZUMI_LINKBREAKER,
    &OVERZEALOUS_MUSCLE,
    &PITILESS_CARNAGE,
    &RAKISH_CREW,
    &RATTLEBACK_APOTHECARY,
    &RAVEN_OF_FELL_OMENS,
    &RICTUS_ROBBER,
    &ROOFTOP_ASSASSIN,
    &RUSH_OF_DREAD,
    &SERVANT_OF_THE_STINGER,
    &SHOOT_THE_SHERIFF,
    &TINYBONES_JOINS_UP,
    &TINYBONES_THE_PICKPOCKET,
    &TREASURE_DREDGER,
    &UNFORTUNATE_ACCIDENT,
    &UNSCRUPULOUS_CONTRACTOR,
    &VADMIR_NEW_BLOOD,
    &VAULT_PLUNDERER,
    &BRIMSTONE_ROUNDUP,
    &CALAMITY_GALLOPING_INFERNO,
    &CAUGHT_IN_THE_CROSSFIRE,
    &CUNNING_COYOTE,
    &DEADEYE_DUELIST,
    &DEMONIC_RUCKUS,
    &DISCERNING_PEDDLER,
    &EXPLOSIVE_DERAILMENT,
    &FEROCIFICATION,
    &GILA_COURSER,
    &GREAT_TRAIN_HEIST,
    &HELL_TO_PAY,
    &HELLSPUR_BRUTE,
    &HELLSPUR_POSSE_BOSS,
    &HIGHWAY_ROBBERY,
    &IRASCIBLE_WOLVERINE,
    &IRON_FIST_PULVERIZER,
    &LONGHORN_SHARPSHOOTER,
    &MAGDA_THE_HOARDMASTER,
    &MAGEBANE_LIZARD,
    &MINE_RAIDER,
    &OUTLAWS_FURY,
    &PRICKLY_PAIR,
    &QUICK_DRAW,
    &QUILLED_CHARGER,
    &RECKLESS_LACKEY,
    &RESILIENT_ROADRUNNER,
    &RETURN_THE_FAVOR,
    &RODEO_PYROMANCERS,
    &SCALESTORM_SUMMONER,
    &SCORCHING_SHOT,
    &STINGERBACK_TERROR,
    &TAKE_FOR_A_RIDE,
    &THUNDER_SALVO,
    &TRICK_SHOT,
    &ALOE_ALCHEMIST,
    &ANKLE_BITER,
    &BEASTBOND_OUTCASTER,
    &BETRAYAL_AT_THE_VAULT,
    &BRISTLEPACK_SENTRY,
    &BRISTLY_BILL_SPINE_SOWER,
    &CACTARANTULA,
    &COLOSSAL_RATTLEWURM,
    &DANCE_OF_THE_TUMBLEWEEDS,
    &DROVER_GRIZZLY,
    &FREESTRIDER_COMMANDO,
    &FREESTRIDER_LOOKOUT,
    &FULL_STEAM_AHEAD,
    &GIANT_BEAVER,
    &GOLD_RUSH,
    &GOLDVEIN_HYDRA,
    &HARDBRISTLE_BANDIT,
    &INTREPID_STABLEMASTER,
    &MAP_THE_FRONTIER,
    &ORNERY_TUMBLEWAGG,
    &OUTCASTER_GREENBLADE,
    &OUTCASTER_TRAILBLAZER,
    &PATIENT_NATURALIST,
    &RAILWAY_BRAWLER,
    &RAMBLING_POSSUM,
    &RAUCOUS_ENTERTAINER,
    &REACH_FOR_THE_SKY,
    &RISE_OF_THE_VARMINTS,
    &SMUGGLER_S_SURPRISE,
    &SPINEWOODS_ARMADILLO,
    &SPINEWOODS_PALADIN,
    &STUBBORN_BURROWFIEND,
    &THROW_FROM_THE_SADDLE,
    &TRASH_THE_TOWN,
    &TUMBLEWEED_RISING,
    &VORACIOUS_VARMINT,
    &AKUL_THE_UNREPENTANT,
    &ANNIE_FLASH_THE_VETERAN,
    &ANNIE_JOINS_UP,
    &ASSIMILATION_AEGIS,
    &AT_KNIFEPOINT,
    &BADLANDS_REVIVAL,
    &BARON_BERTRAM_GRAYWATER,
    &BONNY_PALL_CLEARCUTTER,
    &BREECHES_THE_BLASTMAKER,
    &BRUSE_TARL_ROVING_RANCHER,
    &CACTUSFOLK_SURESHOT,
    &CONGREGATION_GRYFF,
    &DOC_AURLOCK_GRIZZLED_GENIUS,
    &ERIETTE_THE_BEGUILER,
    &ERTHA_JO_FRONTIER_MENTOR,
    &FORM_A_POSSE,
    &GHIRED_MIRROR_OF_THE_WILDS,
    &THE_GITROG_RAVENOUS_RIDE,
    &HONEST_RUTSTEIN,
    &INTIMIDATION_CAMPAIGN,
    &JEM_LIGHTFOOTE_SKY_EXPLORER,
    &JOLENE_PLUNDERING_PUGILIST,
    &KAMBAL_PROFITEERING_MAYOR,
    &KELLAN_JOINS_UP,
    &KELLAN_THE_KID,
    &KRAUM_VIOLENT_CACOPHONY,
    &LAUGHING_JASPER_FLINT,
    &LAZAV_FAMILIAR_STRANGER,
    &LILAH_UNDEFEATED_SLICKSHOT,
    &MAKE_YOUR_OWN_LUCK,
    &MALCOLM_THE_EYES,
    &MARCHESA_DEALER_OF_DEATH,
    &MIRIAM_HERD_WHISPERER,
    &OBEKA_SPLITTER_OF_SECONDS,
    &OKO_THE_RINGLEADER,
    &PILLAGE_THE_BOG,
    &RAKDOS_JOINS_UP,
    &RAKDOS_THE_MUSCLE,
    &RIKU_OF_MANY_PATHS,
    &ROXANNE_STARFALL_SAVANT,
    &RUTHLESS_LAWBRINGER,
    &SATORU_THE_INFILTRATOR,
    &SELVALA_EAGER_TRAILBLAZER,
    &SERAPHIC_STEED,
    &SLICK_SEQUENCE,
    &TAII_WAKEEN_PERFECT_SHOT,
    &VIAL_SMASHER_GLEEFUL_GRENADIER,
    &VRASKA_JOINS_UP,
    &VRASKA_THE_SILENCER,
    &WRANGLER_OF_THE_DAMNED,
    &WYLIE_DUKE_ATIIN_HERO,
    &BANDIT_S_HAUL,
    &BOOM_BOX,
    &GOLD_PAN,
    &LAVASPUR_BOOTS,
    &LUXURIOUS_LOCOMOTIVE,
    &MOBILE_HOMESTEAD,
    &OASIS_GARDENER,
    &REDROCK_SENTINEL,
    &SILVER_DEPUTY,
    &STERLING_HOUND,
    &TOMB_TRAWLER,
    &ABRADED_BLUFFS,
    &ARID_ARCHWAY,
    &BRISTLING_BACKWOODS,
    &CONDUIT_PYLONS,
    &CREOSOTE_HEATH,
    &ERODED_CANYON,
    &FESTERING_GULCH,
    &FORLORN_FLATS,
    &JAGGED_BARRENS,
    &LONELY_ARROYO,
    &LUSH_OASIS,
    &MIRAGE_MESA,
    &SANDSTORM_VERGE,
    &SOURED_SPRINGS,
    &BUCOLIC_RANCH,
    &JACE_REAWAKENED,
    &SLICKSHOT_SHOW_OFF,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    ARCHANGEL_OF_TITHES_REPRINT,
    TAKE_UP_THE_SHIELD_REPRINT,
    CORRUPTED_CONVICTION_REPRINT,
    FAKE_YOUR_OWN_DEATH_REPRINT,
    SKULDUGGERY_REPRINT,
    SLICKSHOT_SHOW_OFF_ALTERNATE_1,
    TERROR_OF_THE_PEAKS_REPRINT,
    SNAKESKIN_VEIL_REPRINT,
    BLOOMING_MARSH_REPRINT,
    BOTANICAL_SANCTUM_REPRINT,
    CONCEALED_COURTYARD_REPRINT,
    INSPIRING_VANTAGE_REPRINT,
    SPIREBLUFF_CANAL_REPRINT,
    PLAINS_REPRINT,
    ISLAND_REPRINT,
    SWAMP_REPRINT,
    MOUNTAIN_REPRINT,
    FOREST_REPRINT,
    PLAINS_ALTERNATE_1,
    PLAINS_ALTERNATE_2,
    ISLAND_ALTERNATE_1,
    ISLAND_ALTERNATE_2,
    SWAMP_ALTERNATE_1,
    SWAMP_ALTERNATE_2,
    MOUNTAIN_ALTERNATE_1,
    MOUNTAIN_ALTERNATE_2,
    FOREST_ALTERNATE_1,
    FOREST_ALTERNATE_2,
    GERALF_THE_FLESHWRIGHT_ALTERNATE_1,
    GISA_THE_HELLRAISER_ALTERNATE_1,
    KAERVEK_THE_PUNISHER_ALTERNATE_1,
    TINYBONES_THE_PICKPOCKET_ALTERNATE_1,
    ANNIE_FLASH_THE_VETERAN_ALTERNATE_1,
    BREECHES_THE_BLASTMAKER_ALTERNATE_1,
    ERIETTE_THE_BEGUILER_ALTERNATE_1,
    KELLAN_THE_KID_ALTERNATE_1,
    MALCOLM_THE_EYES_ALTERNATE_1,
    OKO_THE_RINGLEADER_ALTERNATE_1,
    RAKDOS_THE_MUSCLE_ALTERNATE_1,
    SATORU_THE_INFILTRATOR_ALTERNATE_1,
    VRASKA_THE_SILENCER_ALTERNATE_1,
    BLOOMING_MARSH_ALTERNATE_1,
    BOTANICAL_SANCTUM_ALTERNATE_1,
    CONCEALED_COURTYARD_ALTERNATE_1,
    INSPIRING_VANTAGE_ALTERNATE_1,
    SPIREBLUFF_CANAL_ALTERNATE_1,
    OKO_THE_RINGLEADER_ALTERNATE_2,
    JACE_REAWAKENED_ALTERNATE_1,
    ANOTHER_ROUND_ALTERNATE_1,
    ARCHANGEL_OF_TITHES_ALTERNATE_1,
    AVEN_INTERRUPTER_ALTERNATE_1,
    CLAIM_JUMPER_ALTERNATE_1,
    DUST_ANIMUS_ALTERNATE_1,
    FINAL_SHOWDOWN_ALTERNATE_1,
    FORTUNE_LOYAL_STEED_ALTERNATE_1,
    HIGH_NOON_ALTERNATE_1,
    ONE_LAST_JOB_ALTERNATE_1,
    ARCHMAGE_S_NEWT_ALTERNATE_1,
    DOUBLE_DOWN_ALTERNATE_1,
    DUELIST_OF_THE_MIND_ALTERNATE_1,
    FBLTHP_LOST_ON_THE_RANGE_ALTERNATE_1,
    THE_KEY_TO_THE_VAULT_ALTERNATE_1,
    STEP_BETWEEN_WORLDS_ALTERNATE_1,
    STOIC_SPHINX_ALTERNATE_1,
    THREE_STEPS_AHEAD_ALTERNATE_1,
    CAUSTIC_BRONCO_ALTERNATE_1,
    INSATIABLE_AVARICE_ALTERNATE_1,
    PITILESS_CARNAGE_ALTERNATE_1,
    RUSH_OF_DREAD_ALTERNATE_1,
    TINYBONES_JOINS_UP_ALTERNATE_1,
    VADMIR_NEW_BLOOD_ALTERNATE_1,
    CALAMITY_GALLOPING_INFERNO_ALTERNATE_1,
    GREAT_TRAIN_HEIST_ALTERNATE_1,
    HELL_TO_PAY_ALTERNATE_1,
    HELLSPUR_POSSE_BOSS_ALTERNATE_1,
    MAGDA_THE_HOARDMASTER_ALTERNATE_1,
    STINGERBACK_TERROR_ALTERNATE_1,
    TERROR_OF_THE_PEAKS_ALTERNATE_1,
    BRISTLY_BILL_SPINE_SOWER_ALTERNATE_1,
    COLOSSAL_RATTLEWURM_ALTERNATE_1,
    FREESTRIDER_LOOKOUT_ALTERNATE_1,
    GOLDVEIN_HYDRA_ALTERNATE_1,
    ORNERY_TUMBLEWAGG_ALTERNATE_1,
    OUTCASTER_TRAILBLAZER_ALTERNATE_1,
    RAILWAY_BRAWLER_ALTERNATE_1,
    SMUGGLER_S_SURPRISE_ALTERNATE_1,
    AKUL_THE_UNREPENTANT_ALTERNATE_1,
    ANNIE_JOINS_UP_ALTERNATE_1,
    ASSIMILATION_AEGIS_ALTERNATE_1,
    BONNY_PALL_CLEARCUTTER_ALTERNATE_1,
    BRUSE_TARL_ROVING_RANCHER_ALTERNATE_1,
    GHIRED_MIRROR_OF_THE_WILDS_ALTERNATE_1,
    THE_GITROG_RAVENOUS_RIDE_ALTERNATE_1,
    KAMBAL_PROFITEERING_MAYOR_ALTERNATE_1,
    KELLAN_JOINS_UP_ALTERNATE_1,
    LAUGHING_JASPER_FLINT_ALTERNATE_1,
    LILAH_UNDEFEATED_SLICKSHOT_ALTERNATE_1,
    MARCHESA_DEALER_OF_DEATH_ALTERNATE_1,
    OBEKA_SPLITTER_OF_SECONDS_ALTERNATE_1,
    PILLAGE_THE_BOG_ALTERNATE_1,
    RAKDOS_JOINS_UP_ALTERNATE_1,
    RIKU_OF_MANY_PATHS_ALTERNATE_1,
    ROXANNE_STARFALL_SAVANT_ALTERNATE_1,
    SELVALA_EAGER_TRAILBLAZER_ALTERNATE_1,
    SERAPHIC_STEED_ALTERNATE_1,
    TAII_WAKEEN_PERFECT_SHOT_ALTERNATE_1,
    VRASKA_JOINS_UP_ALTERNATE_1,
    WYLIE_DUKE_ATIIN_HERO_ALTERNATE_1,
    FRONTIER_SEEKER_ALTERNATE_1,
    SCORCHING_SHOT_ALTERNATE_1,
    HONEST_RUTSTEIN_ALTERNATE_1,
    MAKE_YOUR_OWN_LUCK_ALTERNATE_1,
    RUTHLESS_LAWBRINGER_ALTERNATE_1,
    THE_KEY_TO_THE_VAULT_ALTERNATE_2,
    MAGDA_THE_HOARDMASTER_ALTERNATE_2,
];
