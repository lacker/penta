//! Urza's Legacy cards used by the staged Premodern deck tranche.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::BasicLandType;
use crate::card::BattlefieldEntryChoiceDestinationDef;
use crate::card::BattlefieldEntryScalarChoiceDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CardTypeSet;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseDef;
use crate::card::ColorChoiceOperationDef;
use crate::card::ColorSet;
use crate::card::CostDef;
use crate::card::CostModificationDef;
use crate::card::CostQuantityDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::CreatureTypeSetDef;
use crate::card::DiscardSelectionDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::InstalledTriggerDef;
use crate::card::KeywordAbility;
use crate::card::ManaColor;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::ReplacementChoiceDef;
use crate::card::ReplacementEffectDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::ScaledValueDef;
use crate::card::SubtypeDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::card::sets::y1998::portal_second_age as catalog_p02;
use crate::ids::ParentBinding;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "ULG",
    slug: "urzas-legacy",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// ULG 1 — Angelic Curator
pub(in crate::card::sets) static ANGELIC_CURATOR: CardRecord = CardRecord::new(
    "Angelic Curator",
    "c63ba2da-6dea-44ac-8439-527222da565b",
    "Greg Staples",
    // A flier that no artifact can block or kill, in a block where nearly
    // everything was an artifact.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Angel", "Spirit"], 1, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::keyword(
            "Protection from artifacts",
            KeywordAbility::ProtectionFrom(&ObjectPredicateDef::HasType(CardType::Artifact)),
        ),
    ]),
);

// ULG 2 — Blessed Reversal (reprint)
const BLESSED_REVERSAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::BLESSED_REVERSAL,
    "3fb6d738-f6a8-4626-8103-68e63874eda4",
    "Pete Venters",
);

// ULG 3 — Burst of Energy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BURST_OF_ENERGY: CardRecord = CardRecord::new(
    "Burst of Energy",
    "43d590d2-cfa3-43d1-9e65-bc68b5a2a3ee",
    "Mark Brill",
    crate::card::CardRules::unsupported(),
);

// ULG 4 — Cessation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CESSATION: CardRecord = CardRecord::new(
    "Cessation",
    "3a113f0c-8249-427b-979b-10898ec66a3a",
    "Mark Zug",
    crate::card::CardRules::unsupported(),
);

// ULG 5 — Defender of Law
pub(in crate::card::sets) static DEFENDER_OF_LAW: CardRecord = CardRecord::new(
    "Defender of Law",
    "9c8e8719-8c33-429d-8b95-b7f813888850",
    "Carl Critchlow",
    // A blocker held up on the opponent's turn that the red deck's burn
    // cannot answer, which is most of a sideboard card in one body.
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Knight"], 2, 1).with_abilities(&[
        abilities::flash(),
        abilities::protection_from_color(ManaColor::Red),
    ]),
);

// ULG 6 — Devout Harpist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEVOUT_HARPIST: CardRecord = CardRecord::new(
    "Devout Harpist",
    "985b5c60-8a5e-4473-ba43-583aef50f19e",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// ULG 7 — Erase
pub(in crate::card::sets) static ERASE: CardRecord = CardRecord::new(
    "Erase",
    "05c89d61-3c2e-4d70-86d6-f70eba3d327f",
    "Ron Spears",
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell_with_targets(
        "Exile target enchantment.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Enchantment),
        )],
        EffectDef::move_to_zone(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ZoneKind::Exile,
            ZonePlacement::Top,
        ),
    )),
);

// ULG 8 — Expendable Troops
pub(in crate::card::sets) static EXPENDABLE_TROOPS: CardRecord = CardRecord::new(
    "Expendable Troops",
    "f31d7d1b-a219-4653-be99-a885bc9b2e2f",
    "Carl Critchlow",
// It only fires in combat, so the two mana buys a blocker that kills
    // something the turn it stops blocking.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Soldier"], 2, 1).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}, Sacrifice this creature: It deals 2 damage to target attacking or blocking creature.",
            &[CostDef::TapSource, CostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::AttackingOrBlocking,
            ]))],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(2),
            ),
        ),
    ]),
);

// ULG 9 — Hope and Glory
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HOPE_AND_GLORY: CardRecord = CardRecord::new(
    "Hope and Glory",
    "7cc6478f-4ae5-4f26-baa9-b28e992f962e",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// ULG 10 — Iron Will
pub(in crate::card::sets) static IRON_WILL: CardRecord = CardRecord::new(
    "Iron Will",
    "bee0ee84-6c22-4649-b621-e3fdb08bbe45",
    "Val Mayerik",
    // A combat trick that is never a dead card, which is the whole reason
    // cycling was printed on tricks at all.
    CardRules::new_instant(mana_cost!("{W}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target creature gets +0/+4 until end of turn.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(4),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::cycling!(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{2}"))],
        ),
    ]),
);

// ULG 11 — Karmic Guide
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KARMIC_GUIDE: CardRecord = CardRecord::new(
    "Karmic Guide",
    "77d23045-905b-44cb-9af9-cc6ad717477d",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// ULG 12 — Knighthood
pub(in crate::card::sets) static KNIGHTHOOD: CardRecord = CardRecord::new(
    "Knighthood",
    "1d5e98d3-2521-4340-8d48-98e8c2c7818d",
    "Kev Walker",
    // First strike on everything, which turns a board of small creatures
    // into blockers nothing profitably attacks into.
    CardRules::new_enchantment(mana_cost!("{2}{W}")).with_ability(AbilityDef::static_ability(
        "Creatures you control have first strike.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
        },
    )),
);

// ULG 13 — Martyr's Cause
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MARTYR_S_CAUSE: CardRecord = CardRecord::new(
    "Martyr's Cause",
    "4c1f026b-8c7f-4051-9922-5684a6b2c06b",
    "Jeff Laubenstein",
    crate::card::CardRules::unsupported(),
);

// ULG 14 — Mother of Runes
pub(in crate::card::sets) static MOTHER_OF_RUNES: CardRecord = CardRecord::new(
    "Mother of Runes",
    "0b1a46ab-95cb-4c24-924f-fc2afd4fcac7",
    "Scott M. Fischer",
CardRules::new_creature(mana_cost!("{W}"), &["Human", "Cleric"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Target creature you control gains protection from the color of your choice until end of turn.",
            &[CostDef::TapSource],
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
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OPAL_AVENGER: CardRecord = CardRecord::new(
    "Opal Avenger",
    "f9337bbe-e092-469d-8122-77f92e233306",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// ULG 16 — Opal Champion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OPAL_CHAMPION: CardRecord = CardRecord::new(
    "Opal Champion",
    "2699cf3b-df54-4c77-ba19-3bc7598ae3fa",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// ULG 17 — Peace and Quiet
pub(in crate::card::sets) static PEACE_AND_QUIET: CardRecord = CardRecord::new(
    "Peace and Quiet",
    "5d73accc-8f19-44d4-8216-c1acdbef3856",
    "Don Hazeltine",
    // The white version, two mana cheaper because enchantments were rarer
    // than artifacts in the format.
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy two target enchantments.",
        &[AbilityTargetDef::exactly_value(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Enchantment),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
            ValueDef::Constant(2),
        )],
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            then: None,
        },
    )),
);

// ULG 18 — Planar Collapse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PLANAR_COLLAPSE: CardRecord = CardRecord::new(
    "Planar Collapse",
    "ee22cf3c-51b0-4790-ab13-985cbe900c3b",
    "Mark Zug",
    crate::card::CardRules::unsupported(),
);

// ULG 19 — Purify
pub(in crate::card::sets) static PURIFY: CardRecord = CardRecord::new(
    "Purify",
    "5341da18-df05-4135-b948-7aa3e3d7a492",
    "John Avon",
    // Five mana to answer every artifact and enchantment at once, which was
    // a sideboard card in a block full of both.
    CardRules::new_sorcery(mana_cost!("{3}{W}{W}")).with_ability(AbilityDef::spell(
        "Destroy all artifacts and enchantments.",
        EffectDef::Destroy {
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            then: None,
        },
    )),
);

// ULG 20 — Radiant, Archangel
pub(in crate::card::sets) static RADIANT_ARCHANGEL: CardRecord = CardRecord::new(
    "Radiant, Archangel",
    "99509da7-3e11-4c38-804b-286ce572f36e",
    "Michael Sutfin",
    // It counts the opponent's fliers too, so the decks best placed to
    // block it are the ones that make it biggest.
    CardRules::new_creature(mana_cost!("{3}{W}{W}"), &["Angel"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            abilities::vigilance(),
            AbilityDef::static_ability(
                "Radiant gets +1/+1 for each other creature on the battlefield with flying.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Any,
                        )),
                        ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Any,
                        )),
                    ),
                },
            ),
        ]),
);

// ULG 21 — Radiant's Dragoons
pub(in crate::card::sets) static RADIANT_S_DRAGOONS: CardRecord = CardRecord::new(
    "Radiant's Dragoons",
    "8a0f39de-6ad2-410c-bc6c-75fd3c8d159b",
    "Pete Venters",
// Five life and a wall, and the echo means you pay for it twice or
    // give the wall back.
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Soldier"], 2, 5).with_abilities(&[
        abilities::echo(
            "Echo {3}{W} (At the beginning of your upkeep, if this came under your control since the beginning of your last upkeep, sacrifice it unless you pay its echo cost.)",
            &[CostDef::Mana(mana_cost!("{3}{W}"))],
        ),
        abilities::enters_trigger("When this creature enters, you gain 5 life.", EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(5),
            }),
    ]),
);

// ULG 22 — Radiant's Judgment
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RADIANT_S_JUDGMENT: CardRecord = CardRecord::new(
    "Radiant's Judgment",
    "28d2718e-c6fc-4961-b094-11f25f1177ff",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// ULG 23 — Sustainer of the Realm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUSTAINER_OF_THE_REALM: CardRecord = CardRecord::new(
    "Sustainer of the Realm",
    "ca280e1e-4231-48e5-be1b-965480822c46",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// ULG 24 — Tragic Poet
pub(in crate::card::sets) static TRAGIC_POET: CardRecord = CardRecord::new(
    "Tragic Poet",
    "294aa7fc-12be-4722-b288-de14a28919b2",
    "Quinton Hoover",
// One mana held back to buy an answer out of the graveyard, which matters in
    // a format where the enchantment was the removal.
    CardRules::new_creature(mana_cost!("{W}"), &["Human"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}, Sacrifice this creature: Return target enchantment card from your graveyard to your \
             hand.",
            &[CostDef::TapSource, CostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Enchantment),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            })],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
        ),
    ),
);

// ULG 25 — Anthroplasm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANTHROPLASM: CardRecord = CardRecord::new(
    "Anthroplasm",
    "089e2bc7-0063-47bf-8f66-48bed6eb046b",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// ULG 26 — Archivist
pub(in crate::card::sets) static ARCHIVIST: CardRecord = CardRecord::new(
    "Archivist",
    "9936cb4d-f4e3-4fc7-869e-8f17056e57d5",
    "Pete Venters",
    // A card a turn for as long as it lives, which is a rate no four-mana
    // 1/1 has ever been allowed since.
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Human", "Wizard"], 1, 1).with_ability(
        AbilityDef::activated(
            "{T}: Draw a card.",
            &[CostDef::TapSource],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// ULG 27 — Aura Flux
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AURA_FLUX: CardRecord = CardRecord::new(
    "Aura Flux",
    "a6be1542-70b8-4e97-a951-100966dc46ce",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// ULG 28 — Bouncing Beebles
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOUNCING_BEEBLES: CardRecord = CardRecord::new(
    "Bouncing Beebles",
    "8656bdd4-0c45-43f9-b2dc-d11a355ff747",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// ULG 29 — Cloud of Faeries
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLOUD_OF_FAERIES: CardRecord = CardRecord::new(
    "Cloud of Faeries",
    "4e76d04a-0038-4b5b-a026-3056ee940da9",
    "Melissa A. Benson",
    crate::card::CardRules::unsupported(),
);

// ULG 30 — Delusions of Mediocrity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DELUSIONS_OF_MEDIOCRITY: CardRecord = CardRecord::new(
    "Delusions of Mediocrity",
    "899088b3-4cdf-47c0-8c52-3c9f55c086c4",
    "Jeff Laubenstein",
    crate::card::CardRules::unsupported(),
);

// ULG 31 — Fleeting Image
pub(in crate::card::sets) static FLEETING_IMAGE: CardRecord = CardRecord::new(
    "Fleeting Image",
    "ef9a5501-f149-47d0-9d79-151a524c7c54",
    "Scott M. Fischer",
    // Two mana to dodge anything, every time, which makes a 2/1 flier
    // impossible to answer with cards rather than with damage.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Illusion"], 2, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{1}{U}: Return this creature to its owner's hand.",
            &[CostDef::Mana(mana_cost!("{1}{U}"))],
            EffectDef::move_to_zone(
                EffectRecipientDef::Source,
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
        ),
    ]),
);

// ULG 32 — Frantic Search
pub(in crate::card::sets) static FRANTIC_SEARCH: CardRecord = CardRecord::new(
    "Frantic Search",
    "1904db14-6df7-424f-afa5-e3dfab31300a",
    "Jeff Miracola",
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
                    binding: ObjectChoiceBindingDef::Objects(ParentBinding),
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
                        object: EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding)),
                    },
                }),
            ]),
        ]),
    )),
);

// ULG 33 — Intervene
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INTERVENE: CardRecord = CardRecord::new(
    "Intervene",
    "4b0e3894-5dfe-4d03-9996-eebf96c58168",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// ULG 34 — King Crab
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KING_CRAB: CardRecord = CardRecord::new(
    "King Crab",
    "aedea953-b5f1-4ec7-bd9f-b7827f9d40fe",
    "Daniel Gelon",
    crate::card::CardRules::unsupported(),
);

// ULG 35 — Levitation
pub(in crate::card::sets) static LEVITATION: CardRecord = CardRecord::new(
    "Levitation",
    "ca18a2e7-6b01-4d10-82b5-0c1cb6ba0d2b",
    "Heather Hudson",
    CardRules::new_enchantment(mana_cost!("{2}{U}{U}")).with_ability(AbilityDef::static_ability(
        "Creatures you control have flying.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::add_ability(&abilities::flying()),
        },
    )),
);

// ULG 36 — Miscalculation
pub(in crate::card::sets) static MISCALCULATION: CardRecord = CardRecord::new(
    "Miscalculation",
    "4b4956a2-9a39-4152-9c98-70e4b2acfa26",
    "Jeff Laubenstein",
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
            abilities::counter_target_unless_paid(&[CostDef::GenericMana(ValueDef::Constant(2))]),
        ),
        abilities::cycling!(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{2}"))],
        ),
    ]),
);

// ULG 37 — Opportunity
pub(in crate::card::sets) static OPPORTUNITY: CardRecord = CardRecord::new(
    "Opportunity",
    "63a09767-a15d-42b0-aaa3-794a4e11037a",
    "Ron Spears",
    CardRules::new_instant(mana_cost!("{4}{U}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Target player draws four cards.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(4),
        },
    )),
);

// ULG 38 — Palinchron
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PALINCHRON: CardRecord = CardRecord::new(
    "Palinchron",
    "5621db3f-a9e7-4350-9c6a-0ba04a628947",
    "Matthew D. Wilson",
    crate::card::CardRules::unsupported(),
);

// ULG 39 — Raven Familiar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAVEN_FAMILIAR: CardRecord = CardRecord::new(
    "Raven Familiar",
    "b104638d-29aa-490c-8cfb-e08fc94efb59",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// ULG 40 — Rebuild
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REBUILD: CardRecord = CardRecord::new(
    "Rebuild",
    "4dc1613c-a149-4f04-9950-41637d35d675",
    "Allen Williams",
    crate::card::CardRules::unsupported(),
);

// ULG 41 — Second Chance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SECOND_CHANCE: CardRecord = CardRecord::new(
    "Second Chance",
    "62d1a0da-40b9-4e79-bace-b93f98ae4695",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// ULG 42 — Slow Motion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SLOW_MOTION: CardRecord = CardRecord::new(
    "Slow Motion",
    "2860a20d-e1bf-4e46-8c07-a858f616d5a5",
    "Todd Lockwood",
    crate::card::CardRules::unsupported(),
);

// ULG 43 — Snap
pub(in crate::card::sets) static SNAP: CardRecord = CardRecord::new(
    "Snap",
    "f7e0549e-2d23-4ea8-b8d1-ae21af2c9091",
    "Mike Raabe",
    // The two lands pay the two mana back, so this bounces for free -- which
    // is what makes it a combo piece rather than a tempo card.
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Return target creature to its owner's hand. Untap up to two lands.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                // Any lands, not only your own: the printed clause names no
                // controller.
                candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Land),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                )),
                exclude: None,
                minimum: 0,
                maximum: 2,
                visibility: ChoiceVisibilityDef::Public,
                then: &EffectDef::Untap {
                    object: EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding)),
                },
            }),
        ]),
    )),
);

// ULG 44 — Thornwind Faeries
pub(in crate::card::sets) static THORNWIND_FAERIES: CardRecord = CardRecord::new(
    "Thornwind Faeries",
    "9cb4b20a-448a-4855-9e60-19625f921a4d",
    "Rebecca Guay",
    // A 1/1 flier that kills an X/1 every turn, which in its format was
    // most of the creatures worth killing.
    CardRules::new_creature(mana_cost!("{1}{U}{U}"), &["Faerie"], 1, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated_with_targets(
            "{T}: This creature deals 1 damage to any target.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(1),
            ),
        ),
    ]),
);

// ULG 45 — Tinker
pub(in crate::card::sets) static TINKER: CardRecord = CardRecord::new(
    "Tinker",
    "7da23b15-dfb8-4267-9b33-d7a4c035c434",
    "Mike Raabe",
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
            CostDef::sacrifice(
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
pub(in crate::card::sets) static VIGILANT_DRAKE: CardRecord = CardRecord::new(
    "Vigilant Drake",
    "37940486-2d7f-40d9-9c19-151b9307d374",
    "Greg Staples",
    // Vigilance bought by the turn rather than printed, which is worth it
    // exactly when there is spare mana.
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Drake"], 3, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{2}{U}: Untap this creature.",
            &[CostDef::Mana(mana_cost!("{2}{U}"))],
            EffectDef::Untap {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// ULG 47 — Walking Sponge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WALKING_SPONGE: CardRecord = CardRecord::new(
    "Walking Sponge",
    "b125d1e7-5d9b-4997-88b0-71bdfc19c6f2",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// ULG 48 — Weatherseed Faeries
pub(in crate::card::sets) static WEATHERSEED_FAERIES: CardRecord = CardRecord::new(
    "Weatherseed Faeries",
    "8c7ebec7-7375-4362-9489-437ff9305f19",
    "Don Hazeltine",
    // Blue's version of the same hoser, which matters because blue's other
    // two-drops all die to the same burn spell.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Faerie"], 2, 1).with_abilities(&[
        abilities::flying(),
        abilities::protection_from_color(ManaColor::Red),
    ]),
);

// ULG 49 — Bone Shredder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BONE_SHREDDER: CardRecord = CardRecord::new(
    "Bone Shredder",
    "ece050ad-788e-4451-b773-ca42c37549d2",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// ULG 50 — Brink of Madness
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRINK_OF_MADNESS: CardRecord = CardRecord::new(
    "Brink of Madness",
    "ff5391d8-b546-4159-955e-16bb58052311",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// ULG 51 — Engineered Plague
pub(in crate::card::sets) static ENGINEERED_PLAGUE: CardRecord = CardRecord::new(
    "Engineered Plague",
    "27e158d5-efb2-4f90-8898-60ede98f7d29",
    "Michael Sutfin",
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
pub(in crate::card::sets) static EVISCERATOR: CardRecord = CardRecord::new(
    "Eviscerator",
    "167e7f67-8d44-4134-b7b9-54ccdfb8675c",
    "Michael Sutfin",
    // Five power for five mana and five life, which is a rate only a deck
    // already racing can pay.
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Phyrexian", "Horror"], 5, 5)
        .with_abilities(&[
            abilities::protection_from_color(ManaColor::White),
            abilities::enters_trigger(
                "When this creature enters, you lose 5 life.",
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(5),
                },
            ),
        ]),
);

// ULG 53 — Fog of Gnats
pub(in crate::card::sets) static FOG_OF_GNATS: CardRecord = CardRecord::new(
    "Fog of Gnats",
    "3f1e9c54-134b-41da-8c3d-ec699d96778a",
    "Jeff Miracola",
    // A two-mana flier that black mana keeps alive, so it blocks forever
    // against anything without a real answer.
    CardRules::new_creature(mana_cost!("{B}{B}"), &["Insect"], 1, 1).with_abilities(&[
        abilities::flying(),
        abilities::regenerate_self(
            "{B}: Regenerate this creature.",
            &[CostDef::Mana(mana_cost!("{B}"))],
        ),
    ]),
);

// ULG 54 — Giant Cockroach
pub(in crate::card::sets) static GIANT_COCKROACH: CardRecord = CardRecord::new(
    "Giant Cockroach",
    "0521bf0c-9f43-402e-8065-d2fc02e20194",
    "Heather Hudson",
    // A vanilla 4/2 for four. It attacks well and blocks once, which is the
    // trade black keeps offering.
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Insect"], 4, 2),
);

// ULG 55 — Lurking Skirge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LURKING_SKIRGE: CardRecord = CardRecord::new(
    "Lurking Skirge",
    "9063cc12-a822-4488-856e-93d70ecfe37f",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// ULG 56 — No Mercy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NO_MERCY: CardRecord = CardRecord::new(
    "No Mercy",
    "4e2fc29c-0223-4b03-864f-eb9149abc921",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// ULG 57 — Ostracize
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OSTRACIZE: CardRecord = CardRecord::new(
    "Ostracize",
    "7b00193a-84ae-4465-943d-01e3d5fa9aca",
    "Chippy",
    crate::card::CardRules::unsupported(),
);

// ULG 58 — Phyrexian Broodlings
pub(in crate::card::sets) static PHYREXIAN_BROODLINGS: CardRecord = CardRecord::new(
    "Phyrexian Broodlings",
    "2313481c-baf9-4dc7-80c7-1ebc6502dce7",
    "Daren Bader",
    // A sacrifice outlet that keeps the value, so every creature about to
    // die is a permanent counter instead.
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Phyrexian", "Minion"], 2, 2).with_ability(
        AbilityDef::activated(
            "{1}, Sacrifice a creature: Put a +1/+1 counter on this creature.",
            &[
                CostDef::Mana(mana_cost!("{1}")),
                CostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    controller: PlayerRelation::You,
                },
            ],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// ULG 59 — Phyrexian Debaser
pub(in crate::card::sets) static PHYREXIAN_DEBASER: CardRecord = CardRecord::new(
    "Phyrexian Debaser",
    "672dcca2-096b-4bcc-9b02-7180c4c0d4c7",
    "Mark Tedin",
    // A flier that trades with anything small on the way out, which is more than
    // most four-drops manage on the turn they die.
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Phyrexian", "Carrier"], 2, 2).with_abilities(
        &[
            abilities::flying(),
            AbilityDef::activated_with_targets(
                "{T}, Sacrifice this creature: Target creature gets -2/-2 until end of turn.",
                &[CostDef::TapSource, CostDef::SacrificeSource],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-2),
                        ValueDef::Constant(-2),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ],
    ),
);

// ULG 60 — Phyrexian Defiler
pub(in crate::card::sets) static PHYREXIAN_DEFILER: CardRecord = CardRecord::new(
    "Phyrexian Defiler",
    "0d555b5e-9f8a-4b1b-a4a6-dee8e177d9e8",
    "DiTerlizzi",
    // Removal stapled to a body, though the tap means it has to survive a
    // turn before it can trade itself in.
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Phyrexian", "Carrier"], 3, 3).with_ability(
        AbilityDef::activated_with_targets(
            "{T}, Sacrifice this creature: Target creature gets -3/-3 until end of turn.",
            &[CostDef::TapSource, CostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-3),
                    ValueDef::Constant(-3),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ULG 61 — Phyrexian Denouncer
pub(in crate::card::sets) static PHYREXIAN_DENOUNCER: CardRecord = CardRecord::new(
    "Phyrexian Denouncer",
    "dcf849a0-9b53-4a8a-87a7-dc38d97311ab",
    "Brian Snõddy",
    // The small version, which kills an X/1 for two mana and a summoning
    // sickness delay.
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Phyrexian", "Carrier"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}, Sacrifice this creature: Target creature gets -1/-1 until end of turn.",
            &[CostDef::TapSource, CostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-1),
                    ValueDef::Constant(-1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ULG 62 — Phyrexian Plaguelord
pub(in crate::card::sets) static PHYREXIAN_PLAGUELORD: CardRecord = CardRecord::new(
    "Phyrexian Plaguelord",
    "307bd530-4b11-428e-864e-e24e96051e3e",
    "Kev Walker",
    // A sacrifice outlet that turns every other creature on the board into a
    // -1/-1, which is how a five-drop ends a game of small creatures.
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Phyrexian", "Carrier"], 4, 4)
        .with_abilities(&[
            AbilityDef::activated_with_targets(
                "{T}, Sacrifice this creature: Target creature gets -4/-4 until end of turn.",
                &[CostDef::TapSource, CostDef::SacrificeSource],
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
            ),
            AbilityDef::activated_with_targets(
                "Sacrifice a creature: Target creature gets -1/-1 until end of turn.",
                &[CostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    controller: PlayerRelation::You,
                }],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-1),
                        ValueDef::Constant(-1),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// ULG 63 — Phyrexian Reclamation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_RECLAMATION: CardRecord = CardRecord::new(
    "Phyrexian Reclamation",
    "228a2bb7-d9f0-47b5-a0d9-2adf1b33e995",
    "rk post",
    crate::card::CardRules::unsupported(),
);

// ULG 64 — Plague Beetle
pub(in crate::card::sets) static PLAGUE_BEETLE: CardRecord = CardRecord::new(
    "Plague Beetle",
    "c07f4e55-57f9-49f6-a1a2-1c94dcbe7d71",
    "Tom Fleming",
    // A 1/1 for one that the mirror cannot block. It is a clock only in the
    // matchup that was already going to be long.
    CardRules::new_creature(mana_cost!("{B}"), &["Insect"], 1, 1)
        .with_ability(abilities::landwalk(BasicLandType::Swamp)),
);

// ULG 65 — Rank and File
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RANK_AND_FILE: CardRecord = CardRecord::new(
    "Rank and File",
    "59481cb5-2cb0-4b8c-84ee-519399862d46",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// ULG 66 — Sick and Tired
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SICK_AND_TIRED: CardRecord = CardRecord::new(
    "Sick and Tired",
    "8736f8a2-ee8d-49d2-883f-b22cbe3f3645",
    "Val Mayerik",
    crate::card::CardRules::unsupported(),
);

// ULG 67 — Sleeper's Guile
pub(in crate::card::sets) static SLEEPER_S_GUILE: CardRecord = CardRecord::new(
    "Sleeper's Guile",
    "a001ca83-35b5-48e5-8337-92258d5affc2",
    "Daren Bader",
// The Aura comes back when it dies, so removing the creature under it
    // costs a card and buys nothing.
    CardRules::new_enchantment(mana_cost!("{2}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature has fear.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: abilities::FEAR_RESTRICTION,
                },
            ),
            abilities::dies_trigger(
                "When this Aura is put into a graveyard from the battlefield, return it to its owner's hand.",
                EffectDef::move_to_zone(
                    EffectRecipientDef::TriggeringZoneChangeResult,
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
            ),
        ]),
);

// ULG 68 — Subversion
pub(in crate::card::sets) static SUBVERSION: CardRecord = CardRecord::new(
    "Subversion",
    "50f1bca9-5831-4e8b-8920-f28ebb3ffb27",
    "Rob Alexander",
// A point a turn that no blocker answers, which is a clock a control
    // deck can win with while doing nothing else.
    CardRules::new_enchantment(mana_cost!("{3}{B}{B}")).with_ability(AbilityDef::triggered(
        "At the beginning of your upkeep, each opponent loses 1 life. You gain life equal to the life lost this way.",
        TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
        // Loss of life rather than damage, so nothing prevents it; with one
        // opponent the life gained is the same fixed point.
        EffectDef::Sequence(&[
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Opponent,
                amount: ValueDef::Constant(1),
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// ULG 69 — Swat
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SWAT: CardRecord = CardRecord::new(
    "Swat",
    "947b8923-d9d6-4dd8-928b-91be9105ffb4",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// ULG 70 — Tethered Skirge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TETHERED_SKIRGE: CardRecord = CardRecord::new(
    "Tethered Skirge",
    "0ab1d02c-5d9d-4436-af3b-fb7190c1c028",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// ULG 71 — Treacherous Link
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TREACHEROUS_LINK: CardRecord = CardRecord::new(
    "Treacherous Link",
    "2e5f8581-411b-4403-9c3a-3cf2156f6779",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// ULG 72 — Unearth
pub(in crate::card::sets) static UNEARTH: CardRecord = CardRecord::new(
    "Unearth",
    "b6cb2549-e485-44d6-9d65-7605c568909e",
    "Don Hazeltine",
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
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Battlefield,
                ZonePlacement::Top,
            ),
        ),
        abilities::cycling!(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{2}"))],
        ),
    ]),
);

// ULG 73 — About Face
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ABOUT_FACE: CardRecord = CardRecord::new(
    "About Face",
    "85e71828-095b-4729-ab11-c6c39ba29aab",
    "Melissa A. Benson",
    crate::card::CardRules::unsupported(),
);

// ULG 74 — Avalanche Riders
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVALANCHE_RIDERS: CardRecord = CardRecord::new(
    "Avalanche Riders",
    "3bdc5330-c76b-40ca-a694-58fa4b9b7304",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// ULG 75 — Defender of Chaos
pub(in crate::card::sets) static DEFENDER_OF_CHAOS: CardRecord = CardRecord::new(
    "Defender of Chaos",
    "717ae26a-e5a0-4478-9995-00ea6bd84c03",
    "Carl Critchlow",
    // The mirror of the Defender of Law, aimed at the white weenie deck
    // that would otherwise block it all day.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Knight"], 2, 1).with_abilities(&[
        abilities::flash(),
        abilities::protection_from_color(ManaColor::White),
    ]),
);

// ULG 76 — Ghitu Fire-Eater
pub(in crate::card::sets) static GHITU_FIRE_EATER: CardRecord = CardRecord::new(
    "Ghitu Fire-Eater",
    "131dce1c-e9c8-437a-b7aa-36a47049d2d2",
    "Melissa A. Benson",
    // It trades itself for its own power at any target, which makes it a
    // burn spell that had to be answered before it grew.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Spellshaper"], 2, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{T}, Sacrifice this creature: It deals damage equal to its power to any target.",
            &[CostDef::TapSource, CostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::SourcePower,
            ),
        ),
    ),
);

// ULG 77 — Ghitu Slinger
pub(in crate::card::sets) static GHITU_SLINGER: CardRecord = CardRecord::new(
    "Ghitu Slinger",
    "67e4bc1d-6a4b-408a-8921-433249c960f9",
    "Melissa A. Benson",
// Two damage and a 2/2, which is two cards' worth for one -- and the
    // echo is what pays for it.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Nomad"], 2, 2).with_abilities(&[
        abilities::echo(
            "Echo {2}{R} (At the beginning of your upkeep, if this came under your control since the beginning of your last upkeep, sacrifice it unless you pay its echo cost.)",
            &[CostDef::Mana(mana_cost!("{2}{R}"))],
        ),
        abilities::enters_trigger_with_targets(
            "When this creature enters, it deals 2 damage to any target.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget)],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(2),
            ),
        ),
    ]),
);

// ULG 78 — Ghitu War Cry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GHITU_WAR_CRY: CardRecord = CardRecord::new(
    "Ghitu War Cry",
    "c9786c4f-f09b-46ce-966c-10efb1e5e609",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// ULG 79 — Goblin Medics
pub(in crate::card::sets) static GOBLIN_MEDICS: CardRecord = CardRecord::new(
    "Goblin Medics",
    "72cc08b6-f31a-46b3-b233-f6bb2c6b1106",
    "Jeff Laubenstein",
    // It pings whenever anything taps it, so an opponent's tap effect works
    // for you as readily as your own.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin"], 1, 1).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever this creature becomes tapped, it deals 1 damage to any target.",
            TriggerEventDef::tapped(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(1),
            ),
        ),
    ),
);

// ULG 80 — Goblin Welder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_WELDER: CardRecord = CardRecord::new(
    "Goblin Welder",
    "6171e136-1167-4329-acb2-6853d3a814e5",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// ULG 81 — Granite Grip
pub(in crate::card::sets) static GRANITE_GRIP: CardRecord = CardRecord::new(
    "Granite Grip",
    "ee9e0e7e-ada8-49f5-9dd9-f62464697675",
    "Mike Raabe",
    // All of it in power, which suits a colour that would rather trade
    // than survive.
    CardRules::new_enchantment(mana_cost!("{2}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +1/+0 for each Mountain you control.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Mountain]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        )),
                        ValueDef::Constant(0),
                    ),
                },
            ),
        ]),
);

// ULG 82 — Impending Disaster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IMPENDING_DISASTER: CardRecord = CardRecord::new(
    "Impending Disaster",
    "44497303-9686-4810-bf0f-876dd9696cab",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// ULG 83 — Last-Ditch Effort
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAST_DITCH_EFFORT: CardRecord = CardRecord::new(
    "Last-Ditch Effort",
    "295f7fe0-0681-4b25-807f-30ed70ec78d5",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// ULG 84 — Lava Axe (reprint)
const LAVA_AXE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::LAVA_AXE,
    "e11ec278-46f5-4970-ad0b-f6718c73de6c",
    "Brian Snõddy",
);

// ULG 85 — Molten Hydra
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOLTEN_HYDRA: CardRecord = CardRecord::new(
    "Molten Hydra",
    "95234b29-9ac8-4200-b42d-9653ba51b010",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// ULG 86 — Parch
pub(in crate::card::sets) static PARCH: CardRecord = CardRecord::new(
    "Parch",
    "d3ab8065-cecc-4b19-be93-7cf791a93e62",
    "Ron Spencer",
    // A maindeck burn spell that doubles against blue, which in a format of
    // blue creatures is most of a sideboard card for free.
    CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Parch deals 2 damage to any target.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::AnyTarget,
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(2),
                ),
            ),
            AbilityDef::spell_with_targets(
                "Parch deals 4 damage to target blue creature.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Color(ManaColor::Blue),
                    ]),
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(4),
                ),
            ),
        ],
    )),
);

// ULG 87 — Pygmy Pyrosaur
pub(in crate::card::sets) static PYGMY_PYROSAUR: CardRecord = CardRecord::new(
    "Pygmy Pyrosaur",
    "96136626-4777-4e58-865b-c4d3f6ceb59d",
    "Dan Frazier",
    // A one-drop that only attacks, with a mana sink that makes every spare
    // red mana a point of damage.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Lizard"], 1, 1).with_abilities(&[
        AbilityDef::static_ability(
            "This creature can't block.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
            },
        ),
        AbilityDef::activated(
            "{R}: This creature gets +1/+0 until end of turn.",
            &[CostDef::Mana(mana_cost!("{R}"))],
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

// ULG 88 — Pyromancy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PYROMANCY: CardRecord = CardRecord::new(
    "Pyromancy",
    "35a91a58-cc8b-47a3-8c53-43c32753a00d",
    "Quinton Hoover",
    crate::card::CardRules::unsupported(),
);

// ULG 89 — Rack and Ruin
pub(in crate::card::sets) static RACK_AND_RUIN: CardRecord = CardRecord::new(
    "Rack and Ruin",
    "a46e8f6a-3a1a-4c30-9348-4b31882267eb",
    "Donato Giancola",
    // Two artifacts for three mana at instant speed, which in its block was
    // closer to a two-for-one than a sweeper.
    CardRules::new_instant(mana_cost!("{2}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy two target artifacts.",
        &[AbilityTargetDef::exactly_value(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Artifact),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
            ValueDef::Constant(2),
        )],
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            then: None,
        },
    )),
);

// ULG 90 — Rivalry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIVALRY: CardRecord = CardRecord::new(
    "Rivalry",
    "3b79677b-d5b9-47c7-a5f5-45446d5cddff",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// ULG 91 — Shivan Phoenix
pub(in crate::card::sets) static SHIVAN_PHOENIX: CardRecord = CardRecord::new(
    "Shivan Phoenix",
    "112aa0e2-7e4a-4ae8-bedb-d84b4116df5e",
    "Daren Bader",
    // Six mana buys a threat that only ever costs the opponent's removal,
    // never yours -- so long as you can keep paying six.
    CardRules::new_creature(mana_cost!("{4}{R}{R}"), &["Phoenix"], 3, 4).with_abilities(&[
        abilities::flying(),
        abilities::dies_trigger(
            "When this creature dies, return it to its owner's hand.",
            EffectDef::move_to_zone(
                EffectRecipientDef::SourceZoneChangeSuccessor,
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
        ),
    ]),
);

// ULG 92 — Sluggishness
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SLUGGISHNESS: CardRecord = CardRecord::new(
    "Sluggishness",
    "2ba91431-3fcd-4b44-ae7b-a69eb18efd5f",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// ULG 93 — Viashino Bey
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VIASHINO_BEY: CardRecord = CardRecord::new(
    "Viashino Bey",
    "26cc07c6-60c7-4abe-8197-7544887ec64d",
    "Bradley Williams",
    crate::card::CardRules::unsupported(),
);

// ULG 94 — Viashino Cutthroat
pub(in crate::card::sets) static VIASHINO_CUTTHROAT: CardRecord = CardRecord::new(
    "Viashino Cutthroat",
    "bcbab69d-3259-40f4-a588-ab550858a178",
    "Edward P. Beard, Jr.",
    // Five damage the turn it lands, every turn, for four mana -- as long
    // as the mana is spent again each time.
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Lizard"], 5, 3).with_abilities(&[
        abilities::haste(),
        AbilityDef::triggered(
            "At the beginning of the end step, return this creature to its owner's hand.",
            // Any end step, not only yours: cast on their turn it comes
            // back the same turn, which is what makes it a trick.
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::Any,
            },
            EffectDef::move_to_zone(
                EffectRecipientDef::Source,
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
        ),
    ]),
);

// ULG 95 — Viashino Heretic
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VIASHINO_HERETIC: CardRecord = CardRecord::new(
    "Viashino Heretic",
    "143e435e-e3a1-45b0-81c3-bd47916df8ac",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// ULG 96 — Viashino Sandscout
pub(in crate::card::sets) static VIASHINO_SANDSCOUT: CardRecord = CardRecord::new(
    "Viashino Sandscout",
    "12dd888a-ca98-44dd-a213-858c3539dc97",
    "Scott M. Fischer",
    // The cheap end of the same deal: two damage a turn for two mana, paid
    // again and again.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Lizard", "Scout"], 2, 1).with_abilities(&[
        abilities::haste(),
        AbilityDef::triggered(
            "At the beginning of the end step, return this creature to its owner's hand.",
            // Any end step, not only yours: cast on their turn it comes
            // back the same turn, which is what makes it a trick.
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::Any,
            },
            EffectDef::move_to_zone(
                EffectRecipientDef::Source,
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
        ),
    ]),
);

// ULG 97 — Bloated Toad
pub(in crate::card::sets) static BLOATED_TOAD: CardRecord = CardRecord::new(
    "Bloated Toad",
    "9686f1a8-035e-415e-9a06-933d6ce1cd5c",
    "Una Fricker",
    // A blocker aimed at one colour, which cycling lets you maindeck without
    // ever drawing it against the others.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Frog"], 2, 2).with_abilities(&[
        abilities::protection_from_color(ManaColor::Blue),
        abilities::cycling!(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{2}"))],
        ),
    ]),
);

// ULG 98 — Crop Rotation
pub(in crate::card::sets) static CROP_ROTATION: CardRecord = CardRecord::new(
    "Crop Rotation",
    "6563f790-862c-465a-b963-7a61f2385516",
    "DiTerlizzi",
CardRules::new_instant(mana_cost!("{G}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, sacrifice a land.\nSearch your library for a land card, put that card onto the battlefield, then shuffle.",
            &[],
            // Sacrificing a land is what makes this an instant-speed tutor rather than a
            // ramp spell: the land you give up pays for the one you go and get, so the
            // board count never moves.
            CostDef::sacrifice(
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
pub(in crate::card::sets) static DARKWATCH_ELVES: CardRecord = CardRecord::new(
    "Darkwatch Elves",
    "212a807f-d5d0-4787-b390-3351783a1ae4",
    "Don Hazeltine",
    // The black half of the same deal, and the one a green deck actually
    // wanted in this format.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Elf"], 2, 2).with_abilities(&[
        abilities::protection_from_color(ManaColor::Black),
        abilities::cycling!(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            &[CostDef::Mana(mana_cost!("{2}"))],
        ),
    ]),
);

// ULG 100 — Defense of the Heart
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEFENSE_OF_THE_HEART: CardRecord = CardRecord::new(
    "Defense of the Heart",
    "9e93381f-627f-41b6-b1b6-45d712a44d8e",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// ULG 101 — Deranged Hermit
pub(in crate::card::sets) static DERANGED_HERMIT: CardRecord = CardRecord::new(
    "Deranged Hermit",
    "bf0e94c9-61c4-4cc0-b5ce-db62bc2660ee",
    "Kev Walker",
// Five mana for eight power across five bodies, rented one echo payment
    // at a time -- and the tokens stay when the rent goes unpaid.
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Elf"], 1, 1).with_abilities(&[
        abilities::echo(
            "Echo {3}{G}{G} (At the beginning of your upkeep, if this came under your control since the beginning of your last upkeep, sacrifice it unless you pay its echo cost.)",
            &[CostDef::Mana(mana_cost!("{3}{G}{G}"))],
        ),
        abilities::enters_trigger(
            "When this creature enters, create four 1/1 green Squirrel creature tokens.",
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(TokenCharacteristics::creature(
                    &["Squirrel"],
                    &[ManaColor::Green],
                    1,
                    1,
                )))
                .with_count(ValueDef::Constant(4)),
            ),
        ),
        AbilityDef::static_ability(
            "Squirrel creatures get +1/+1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Squirrel")),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
            },
        ),
    ]),
);

// ULG 102 — Gang of Elk
pub(in crate::card::sets) static GANG_OF_ELK: CardRecord = CardRecord::new(
    "Gang of Elk",
    "5a84177f-43a3-4d14-9a4c-2ca931cfe092",
    "Una Fricker",
    // A 5/4 that a single blocker cannot stop and a double block makes
    // enormous. It is meant to be unblockable in practice.
    CardRules::new_creature(mana_cost!("{5}{G}"), &["Elk", "Beast"], 5, 4).with_ability(
        AbilityDef::triggered(
            "Whenever this creature becomes blocked, it gets +2/+2 until end of turn \
             for each creature blocking it.",
            TriggerEventDef::BecomesBlocked(ObjectPredicateDef::Source),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                // Counted as the trigger resolves, so a blocker that
                // has already left is not counted and one added by a
                // later effect is.
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Scaled(&ScaledValueDef {
                        value: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::BlockingSource,
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Any,
                        )),
                        factor: 2,
                    }),
                    ValueDef::Scaled(&ScaledValueDef {
                        value: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                            ObjectPredicateDef::BlockingSource,
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Any,
                        )),
                        factor: 2,
                    }),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ULG 103 — Harmonic Convergence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HARMONIC_CONVERGENCE: CardRecord = CardRecord::new(
    "Harmonic Convergence",
    "5aafc380-cf4d-4843-b9c3-c389d9c5e942",
    "John Avon",
    crate::card::CardRules::unsupported(),
);

// ULG 104 — Hidden Gibbons
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HIDDEN_GIBBONS: CardRecord = CardRecord::new(
    "Hidden Gibbons",
    "8f94da51-b4d9-4d79-9113-39f8f4a1be34",
    "Una Fricker",
    crate::card::CardRules::unsupported(),
);

// ULG 105 — Lone Wolf (reprint)
const LONE_WOLF_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_p02::LONE_WOLF,
    "e70ae8e9-852e-4e47-b48d-d1847666fc50",
    "Una Fricker",
);

// ULG 106 — Might of Oaks
pub(in crate::card::sets) static MIGHT_OF_OAKS: CardRecord = CardRecord::new(
    "Might of Oaks",
    "5e48b989-bb64-4c71-9921-0a230fed5b11",
    "Ron Spencer",
    // Seven power at instant speed on a creature they have already blocked,
    // which is the whole card: a combat trick that wins any fight.
    CardRules::new_instant(mana_cost!("{3}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +7/+7 until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(7),
                ValueDef::Constant(7),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// ULG 107 — Multani, Maro-Sorcerer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MULTANI_MARO_SORCERER: CardRecord = CardRecord::new(
    "Multani, Maro-Sorcerer",
    "0d6cc98b-b376-40af-8308-198bab00b2b1",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// ULG 108 — Multani's Acolyte
pub(in crate::card::sets) static MULTANI_S_ACOLYTE: CardRecord = CardRecord::new(
    "Multani's Acolyte",
    "4e5fdecb-bca0-48ea-b5bb-d0886c7d3316",
    "Edward P. Beard, Jr.",
// A 2/1 that replaces itself, rented for a turn, which is the whole
    // echo bargain in its cheapest form.
    CardRules::new_creature(mana_cost!("{G}{G}"), &["Elf"], 2, 1).with_abilities(&[
        abilities::echo(
            "Echo {G}{G} (At the beginning of your upkeep, if this came under your control since the beginning of your last upkeep, sacrifice it unless you pay its echo cost.)",
            &[CostDef::Mana(mana_cost!("{G}{G}"))],
        ),
        abilities::enters_trigger("When this creature enters, draw a card.", EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            }),
    ]),
);

// ULG 109 — Multani's Presence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MULTANI_S_PRESENCE: CardRecord = CardRecord::new(
    "Multani's Presence",
    "38bfa984-5fe9-44ad-b13f-3276951f9f10",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// ULG 110 — Rancor
pub(in crate::card::sets) static RANCOR: CardRecord = CardRecord::new(
    "Rancor",
    "59e256c2-38df-4012-9308-ce17dd889e5f",
    "Kev Walker",
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
            abilities::dies_trigger("When this Aura is put into a graveyard from the battlefield, return it to its owner's hand.", EffectDef::move_to_zone(
                    EffectRecipientDef::TriggeringZoneChangeResult,
                    ZoneKind::Hand,
                    ZonePlacement::Top,
)),
        ]),
);

// ULG 111 — Repopulate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REPOPULATE: CardRecord = CardRecord::new(
    "Repopulate",
    "aac77869-97bc-4976-9ee0-3d60e162b78a",
    "Una Fricker",
    crate::card::CardRules::unsupported(),
);

// ULG 112 — Silk Net
pub(in crate::card::sets) static SILK_NET: CardRecord = CardRecord::new(
    "Silk Net",
    "9498a97a-0e32-4eb8-9cb4-0698ff3a7ded",
    "Rob Alexander",
    // One mana to turn any creature into a flier-catcher, which is green's
    // answer to evasion at its cheapest.
    CardRules::new_instant(mana_cost!("{G}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +1/+1 and gains reach until end of turn.",
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
                AppliedEffectDef::add_ability(&abilities::reach()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// ULG 113 — Simian Grunts
pub(in crate::card::sets) static SIMIAN_GRUNTS: CardRecord = CardRecord::new(
    "Simian Grunts",
    "a0aaea3e-a67a-4d9c-9059-e6beb05f97b1",
    "Pete Venters",
// Flash on a 3/4 is a combat trick that stays; the echo is what stops it
    // being simply better than every other three-drop.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Ape"], 3, 4).with_abilities(&[
        abilities::flash(),
        abilities::echo(
            "Echo {2}{G} (At the beginning of your upkeep, if this came under your control since the beginning of your last upkeep, sacrifice it unless you pay its echo cost.)",
            &[CostDef::Mana(mana_cost!("{2}{G}"))],
        ),
    ]),
);

// ULG 114 — Treefolk Mystic
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TREEFOLK_MYSTIC: CardRecord = CardRecord::new(
    "Treefolk Mystic",
    "9697acf5-9bc5-411d-8574-fe6185f18672",
    "DiTerlizzi",
    crate::card::CardRules::unsupported(),
);

// ULG 115 — Weatherseed Elf
pub(in crate::card::sets) static WEATHERSEED_ELF: CardRecord = CardRecord::new(
    "Weatherseed Elf",
    "4e74f8ae-992b-40a6-87e3-7b321dba4ffa",
    "Heather Hudson",
    // Evasion for one creature a turn, which in a green mirror is the
    // whole difference between the boards.
    CardRules::new_creature(mana_cost!("{G}"), &["Elf"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Target creature gains forestwalk until end of turn.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::forestwalk()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ULG 116 — Weatherseed Treefolk
pub(in crate::card::sets) static WEATHERSEED_TREEFOLK: CardRecord = CardRecord::new(
    "Weatherseed Treefolk",
    "f42cce45-3b6a-43e2-8329-68c30135c5c1",
    "Heather Hudson",
    // Five power of trample that answers removal by asking for five mana
    // again, which a green deck was always going to have.
    CardRules::new_creature(mana_cost!("{2}{G}{G}{G}"), &["Treefolk"], 5, 3).with_abilities(&[
        abilities::trample(),
        abilities::dies_trigger(
            "When this creature dies, return it to its owner's hand.",
            EffectDef::move_to_zone(
                EffectRecipientDef::SourceZoneChangeSuccessor,
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
        ),
    ]),
);

// ULG 117 — Wing Snare
pub(in crate::card::sets) static WING_SNARE: CardRecord = CardRecord::new(
    "Wing Snare",
    "19116d5d-8f2d-4e85-849d-1fbaa67e8cfd",
    "Henry Van Der Linde",
    // Three mana at sorcery speed for a flier, which is what green pays for
    // reaching into the one place it cannot block.
    CardRules::new_sorcery(mana_cost!("{2}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target creature with flying.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
            ]),
        )],
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            then: None,
        },
    )),
);

// ULG 118 — Yavimaya Granger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static YAVIMAYA_GRANGER: CardRecord = CardRecord::new(
    "Yavimaya Granger",
    "05414a41-b50c-49b6-9c27-f3170017d9b0",
    "Henry Van Der Linde",
    crate::card::CardRules::unsupported(),
);

// ULG 119 — Yavimaya Scion
pub(in crate::card::sets) static YAVIMAYA_SCION: CardRecord = CardRecord::new(
    "Yavimaya Scion",
    "a2f80036-1058-4513-8549-0557df9b5d61",
    "DiTerlizzi",
    // Protection aimed at a whole card type, which in an artifact block is
    // closer to an answer than a keyword.
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Treefolk"], 4, 4).with_ability(
        AbilityDef::keyword(
            "Protection from artifacts",
            KeywordAbility::ProtectionFrom(&ObjectPredicateDef::HasType(CardType::Artifact)),
        ),
    ),
);

// ULG 120 — Yavimaya Wurm
pub(in crate::card::sets) static YAVIMAYA_WURM: CardRecord = CardRecord::new(
    "Yavimaya Wurm",
    "dde16069-7176-42dc-88d8-fb37b7894007",
    "Melissa A. Benson",
    // Two mana cheaper than Rootbreaker Wurm and two toughness worse,
    // which is the whole difference.
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Wurm"], 6, 4)
        .with_ability(abilities::trample()),
);

// ULG 121 — Angel's Trumpet
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANGEL_S_TRUMPET: CardRecord = CardRecord::new(
    "Angel's Trumpet",
    "0c7b248a-3c74-4592-b357-47989568298c",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// ULG 122 — Beast of Burden
pub(in crate::card::sets) static BEAST_OF_BURDEN: CardRecord = CardRecord::new(
    "Beast of Burden",
    "06578d72-50e9-468d-96d2-c0cbda14961a",
    "Ron Spears",
// Six mana for a body the size of the board, which means it is a 0/0
    // on an empty one and dies immediately.
    CardRules::new_creature(mana_cost!("{6}"), &["Golem"], 0, 0)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Beast of Burden's power and toughness are each equal to the number of creatures on the battlefield.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::define_power_toughness(
                        ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    )),
                        ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    )),
                    ),
                },
            ),
        ]),
);

// ULG 123 — Crawlspace
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRAWLSPACE: CardRecord = CardRecord::new(
    "Crawlspace",
    "4913a54f-f4d0-483a-a181-716007f65658",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// ULG 124 — Damping Engine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DAMPING_ENGINE: CardRecord = CardRecord::new(
    "Damping Engine",
    "87841977-75ff-49c3-b832-3f0cf48b50b2",
    "rk post",
    crate::card::CardRules::unsupported(),
);

// ULG 125 — Defense Grid
pub(in crate::card::sets) static DEFENSE_GRID: CardRecord = CardRecord::new(
    "Defense Grid",
    "5c2592c9-7e64-40c1-a9cb-c0994094a1e0",
    "Mark Tedin",
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
pub(in crate::card::sets) static GRIM_MONOLITH: CardRecord = CardRecord::new(
    "Grim Monolith",
    "9ddc9fe1-17c8-4e1d-aeb8-c4214e881280",
    "Chippy",
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
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless).with_amount(3)),
        ),
        AbilityDef::activated(
            "{4}: Untap this artifact.",
            &[CostDef::Mana(mana_cost!("{4}"))],
            EffectDef::Untap {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// ULG 127 — Iron Maiden
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IRON_MAIDEN: CardRecord = CardRecord::new(
    "Iron Maiden",
    "ad925fb0-1d5c-44a0-8347-202a38c23107",
    "Tom Wänerstrand",
    crate::card::CardRules::unsupported(),
);

// ULG 128 — Jhoira's Toolbox
pub(in crate::card::sets) static JHOIRA_S_TOOLBOX: CardRecord = CardRecord::new(
    "Jhoira's Toolbox",
    "edb38309-c02c-496c-894f-786a2f6e3d1c",
    "Mike Raabe",
    // Regeneration for the whole artifact half of the board, in a block
    // where that was most of it.
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Insect"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{2}: Regenerate target artifact creature.",
            &[CostDef::Mana(mana_cost!("{2}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                ]),
            )],
            EffectDef::Regenerate {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ),
);

// ULG 129 — Memory Jar
pub(in crate::card::sets) static MEMORY_JAR: CardRecord = CardRecord::new(
    "Memory Jar",
    "a15d33d6-7213-4482-a1be-ac0a73644af6",
    "Donato Giancola",
// Seven cards for everyone, and everyone gets their old hand back at the
    // end of the turn -- which is a windfall only for the player who built a
    // deck that can spend seven cards in one turn.
    CardRules::new_artifact(mana_cost!("{5}")).with_ability(AbilityDef::activated(
        "{T}, Sacrifice this artifact: Each player exiles all cards from their hand face down \
         and draws seven cards. At the beginning of the next end step, each player discards \
         their hand and returns to their hand each card they exiled this way.",
        &[CostDef::TapSource, CostDef::SacrificeSource],
        EffectDef::Sequence(&[
            // Face down: the point of the clause is that nobody learns what the
            // other player put away, only how much of it there was.
            EffectDef::ExileLinkedToSource {
                until_source_leaves: false,
                // Everything in both hands, wherever it came from. The exile is linked to
                // the Jar so the end step can name exactly these cards rather than
                // everything that happens to be in exile by then.
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Hand],
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
            EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                    "At the beginning of the next end step, each player discards their hand and returns to \
                     their hand each card they exiled this way.",
                    TriggerEventDef::StepBegins {
                        step: TurnStepDef::End,
                        player: PlayerRelation::Any,
                    },
                    EffectDef::Sequence(&[
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
                        ]),
                ))),
        ]),
    )),
);

// ULG 130 — Quicksilver Amulet
pub(in crate::card::sets) static QUICKSILVER_AMULET: CardRecord = CardRecord::new(
    "Quicksilver Amulet",
    "ecfdebbe-6432-426f-ac2a-5a9af3047813",
    "Douglas Shuler",
    // The colourless version at twice the activation, so any deck can do it
    // and none does it cheaply.
    CardRules::new_artifact(mana_cost!("{4}")).with_ability(AbilityDef::activated(
        "{4}, {T}: You may put a creature card from your hand onto the battlefield.",
        &[CostDef::Mana(mana_cost!("{4}")), CostDef::TapSource],
        // "You may": a minimum of none, so activating it with an empty
        // hand is legal and does nothing.
        EffectDef::Choose(ChooseDef {
            binding: ObjectChoiceBindingDef::Object(ParentBinding),
            unchosen: None,
            chooser: PlayerRefDef::EffectController,
            candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Hand],
                PlayerRelation::You,
            )),
            exclude: None,
            minimum: 0,
            maximum: 1,
            visibility: ChoiceVisibilityDef::Public,
            then: &EffectDef::move_to_zone(
                EffectRecipientDef::object(ObjectRefDef::Binding(ParentBinding)),
                ZoneKind::Battlefield,
                ZonePlacement::Top,
            ),
        }),
    )),
);

// ULG 131 — Ring of Gix
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RING_OF_GIX: CardRecord = CardRecord::new(
    "Ring of Gix",
    "0b09dc9b-ed01-49de-9675-48c41f428385",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// ULG 132 — Scrapheap
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCRAPHEAP: CardRecord = CardRecord::new(
    "Scrapheap",
    "14aa4474-96a0-4c1d-a09d-73b9c1073b00",
    "Bradley Williams",
    crate::card::CardRules::unsupported(),
);

// ULG 133 — Thran Lens
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THRAN_LENS: CardRecord = CardRecord::new(
    "Thran Lens",
    "200c3666-5ba0-4f0a-adbe-a97af0aa28d1",
    "Allen Williams",
    crate::card::CardRules::unsupported(),
);

// ULG 134 — Thran War Machine
pub(in crate::card::sets) static THRAN_WAR_MACHINE: CardRecord = CardRecord::new(
    "Thran War Machine",
    "5908714a-be91-4279-b87e-e2bc09dbaaba",
    "Pete Venters",
// Four power for four that costs four again next turn and cannot be
    // held back, which is a rate only a deck already ahead can use.
    CardRules::new_artifact_creature(mana_cost!("{4}"), &["Construct"], 4, 5).with_abilities(&[
        abilities::echo(
            "Echo {4} (At the beginning of your upkeep, if this came under your control since the beginning of your last upkeep, sacrifice it unless you pay its echo cost.)",
            &[CostDef::Mana(mana_cost!("{4}"))],
        ),
        abilities::attacks_each_combat_if_able(),
    ]),
);

// ULG 135 — Thran Weaponry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THRAN_WEAPONRY: CardRecord = CardRecord::new(
    "Thran Weaponry",
    "60f005ae-fca1-4a48-84a3-4b217ac879ce",
    "Anthony S. Waters",
    crate::card::CardRules::unsupported(),
);

// ULG 136 — Ticking Gnomes
pub(in crate::card::sets) static TICKING_GNOMES: CardRecord = CardRecord::new(
    "Ticking Gnomes",
    "6241755c-ff3d-44db-a99d-960bea54633e",
    "Henry Van Der Linde",
// Three mana for a 3/3 and a ping, rented for three more -- and the ping
    // is there whether or not the rent is paid.
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Construct"], 3, 3).with_abilities(&[
        abilities::echo(
            "Echo {3} (At the beginning of your upkeep, if this came under your control since the beginning of your last upkeep, sacrifice it unless you pay its echo cost.)",
            &[CostDef::Mana(mana_cost!("{3}"))],
        ),
        AbilityDef::activated_with_targets(
        "Sacrifice this creature: It deals 1 damage to any target.",
        &[CostDef::SacrificeSource],
        &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget)],
        EffectDef::damage(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ValueDef::Constant(1),
        ),
    ),
    ]),
);

// ULG 137 — Urza's Blueprints
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static URZA_S_BLUEPRINTS: CardRecord = CardRecord::new(
    "Urza's Blueprints",
    "026f0d4b-c13e-48a6-915f-b0edd2ac0ae8",
    "Tom Wänerstrand",
    crate::card::CardRules::unsupported(),
);

// ULG 138 — Wheel of Torture
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WHEEL_OF_TORTURE: CardRecord = CardRecord::new(
    "Wheel of Torture",
    "87edc873-169a-4cd3-8a94-84b5810b5ed8",
    "Henry Van Der Linde",
    crate::card::CardRules::unsupported(),
);

// ULG 139 — Faerie Conclave
pub(in crate::card::sets) static FAERIE_CONCLAVE: CardRecord = CardRecord::new(
    "Faerie Conclave",
    "ae3ede87-b026-4781-81ab-8652664f8e41",
    "Val Mayerik",
// A land that attacks for two in the air when the game stalls, at the
    // cost of coming down tapped on turn one.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::tap_for(ManaColor::Blue),
        AbilityDef::activated(
            "{1}{U}: This land becomes a 2/1 blue Faerie creature with flying until end of turn. It's still a land.",
            &[CostDef::Mana(mana_cost!("{1}{U}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                // Animating keeps the land: the creature type is added on top
                // of what is printed rather than replacing it.
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::Blue])),
                    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                    AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Faerie"])),
                    AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(2), ValueDef::Constant(1)),
                    AppliedEffectDef::add_ability(&abilities::flying()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// ULG 140 — Forbidding Watchtower
pub(in crate::card::sets) static FORBIDDING_WATCHTOWER: CardRecord = CardRecord::new(
    "Forbidding Watchtower",
    "96503ed7-aa68-439f-95b0-6ac2c48e3935",
    "Mark Brill",
// The defensive member of the cycle: a land that blocks anything on the
    // ground and survives.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::tap_for(ManaColor::White),
        AbilityDef::activated(
            "{1}{W}: This land becomes a 1/5 white Soldier creature until end of turn. It's still a land.",
            &[CostDef::Mana(mana_cost!("{1}{W}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                // Animating keeps the land: the creature type is added on top
                // of what is printed rather than replacing it.
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::White])),
                    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                    AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Soldier"])),
                    AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(1), ValueDef::Constant(5)),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// ULG 141 — Ghitu Encampment
pub(in crate::card::sets) static GHITU_ENCAMPMENT: CardRecord = CardRecord::new(
    "Ghitu Encampment",
    "bf09ecef-1e30-4206-9648-8fe5c8a71c71",
    "Don Hazeltine",
// First strike makes it win the fight a 2/1 should lose, which is what a
    // land has to do to be worth attacking with.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::tap_for(ManaColor::Red),
        AbilityDef::activated(
            "{1}{R}: This land becomes a 2/1 red Warrior creature with first strike until end of turn. It's still a land.",
            &[CostDef::Mana(mana_cost!("{1}{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                // Animating keeps the land: the creature type is added on top
                // of what is printed rather than replacing it.
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::Red])),
                    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                    AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Warrior"])),
                    AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(2), ValueDef::Constant(1)),
                    AppliedEffectDef::add_ability(&abilities::first_strike()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// ULG 142 — Spawning Pool
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPAWNING_POOL: CardRecord = CardRecord::new(
    "Spawning Pool",
    "43ffec42-57f7-4592-99ab-6284d59829a1",
    "Rob Alexander",
    crate::card::CardRules::unsupported(),
);

// ULG 143 — Treetop Village
pub(in crate::card::sets) static TREETOP_VILLAGE: CardRecord = CardRecord::new(
    "Treetop Village",
    "02212bd8-0c0f-4e8e-99f1-a8477476c03a",
    "Anthony S. Waters",
// The best of the cycle: three trampling power out of a land the deck was
    // playing anyway.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::tap_for(ManaColor::Green),
        AbilityDef::activated(
            "{1}{G}: This land becomes a 3/3 green Ape creature with trample until end of turn. It's still a land.",
            &[CostDef::Mana(mana_cost!("{1}{G}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                // Animating keeps the land: the creature type is added on top
                // of what is printed rather than replacing it.
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::Green])),
                    AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                    AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Ape"])),
                    AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(3), ValueDef::Constant(3)),
                    AppliedEffectDef::add_ability(&abilities::trample()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ANGELIC_CURATOR,
    &BURST_OF_ENERGY,
    &CESSATION,
    &DEFENDER_OF_LAW,
    &DEVOUT_HARPIST,
    &ERASE,
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
    &LEVITATION,
    &MISCALCULATION,
    &OPPORTUNITY,
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
    &QUICKSILVER_AMULET,
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
    BLESSED_REVERSAL_REPRINT,
    LAVA_AXE_REPRINT,
    LONE_WOLF_REPRINT,
];
