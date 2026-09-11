//! Ice Age cards used by the staged Premodern deck tranche.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::ActivationTimingDef;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::BasicLandType;
use crate::card::BlockRestrictionDef;
use crate::card::BlockRestrictionMatchDef;
use crate::card::BlockRestrictionSubjectDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::ComparisonDef;
use crate::card::ConditionalStaticEffectDef;
use crate::card::ControlDurationDef;
use crate::card::CostDef;
use crate::card::CostQuantityDef;
use crate::card::CounterKind;
use crate::card::DamageEventMatcherDef;
use crate::card::DamagePreventionDef;
use crate::card::DividedTotal;
use crate::card::EffectChoiceDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::InstalledTriggerDef;
use crate::card::KeywordAbility;
use crate::card::ManaColor;
use crate::card::ManaRestrictionDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetCountConditionDef;
use crate::card::ObjectSetDef;
use crate::card::ObjectSetPredicateDef;
use crate::card::PayOrDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::ResolvedEffectDurationDef;
use crate::card::ScaledValueDef;
use crate::card::StaticApplyDef;
use crate::card::SubtypeDef;
use crate::card::TargetChooserDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::abilities;
use crate::card::actions;
use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y1993::beta as catalog_leb;
use crate::mana_cost;

const DRAW_AT_NEXT_UPKEEP: EffectDef =
    EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
        "At the beginning of the next turn's upkeep, draw a card.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::Any,
        },
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    )));

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "ICE",
    slug: "ice-age",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// ICE 1 — Adarkar Unicorn
pub(in crate::card::sets) static ADARKAR_UNICORN: CardRecord = CardRecord::new(
    "Adarkar Unicorn",
    "0ba7526f-dba8-4483-b925-946164fc0ae9",
    "Quinton Hoover",
    CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Unicorn"], 2, 2).with_ability(
        AbilityDef::activated_mana(
            "{T}: Add {U} or {C}{U}. Spend this mana only to pay cumulative upkeep costs.",
            &[CostDef::TapSource],
            EffectDef::AddMana(
                AddManaEffectDef::choice_of_bundles(&[
                    crate::card::ManaSplit::from_amounts([(ManaColor::Blue, 1)]),
                    crate::card::ManaSplit::from_amounts([
                        (ManaColor::Blue, 1),
                        (ManaColor::Colorless, 1),
                    ]),
                ])
                .with_restrictions(&[ManaRestrictionDef::Payment(
                    crate::card::AbilityLabel::CUMULATIVE_UPKEEP,
                )]),
            ),
        ),
    ),
);

// ICE 2 — Arctic Foxes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARCTIC_FOXES: CardRecord = CardRecord::new(
    "Arctic Foxes",
    "98f99c3e-dddc-492f-aab6-1d899346a385",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// ICE 3 — Arenson's Aura
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARENSON_S_AURA: CardRecord = CardRecord::new(
    "Arenson's Aura",
    "f94f3e87-1b39-49a8-ad0d-f18c854e298a",
    "Nicola Leonard",
    crate::card::CardRules::unsupported(),
);

// ICE 4 — Armor of Faith
pub(in crate::card::sets) static ARMOR_OF_FAITH: CardRecord = CardRecord::new(
    "Armor of Faith",
    "fccbbc47-99c6-4ba9-95c2-992d5d2a67b2",
    "Anson Maddocks",
    CardRules::new_enchantment(mana_cost!("{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +1/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                },
            ),
            AbilityDef::activated(
                "{W}: Enchanted creature gets +0/+1 until end of turn.",
                &[CostDef::Mana(mana_cost!("{W}"))],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(0),
                        ValueDef::Constant(1),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// ICE 5 — Battle Cry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BATTLE_CRY: CardRecord = CardRecord::new(
    "Battle Cry",
    "c558a8c4-035c-464e-9ff8-c188c1bb619e",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// ICE 6 — Black Scarab
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLACK_SCARAB: CardRecord = CardRecord::new(
    "Black Scarab",
    "5bfd4ee1-05f9-45ae-a31d-1225b271dbe6",
    "Kaja Foglio",
    crate::card::CardRules::unsupported(),
);

// ICE 7 — Blessed Wine
pub(in crate::card::sets) static BLESSED_WINE: CardRecord = CardRecord::new(
    "Blessed Wine",
    "6b9a92f9-9bbc-4887-9fbc-0f7212fd5e66",
    "Kaja Foglio",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell(
        "You gain 1 life.\nDraw a card at the beginning of the next turn's upkeep.",
        EffectDef::Sequence(&[
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
            DRAW_AT_NEXT_UPKEEP,
        ]),
    )),
);

// ICE 8 — Blinking Spirit
pub(in crate::card::sets) static BLINKING_SPIRIT: CardRecord = CardRecord::new(
    "Blinking Spirit",
    "14fc0683-9cfa-4439-a533-8773e7747ec4",
    "Allen Williams",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Spirit"], 2, 2).with_ability(
        AbilityDef::activated(
            "{0}: Return this creature to its owner's hand.",
            &[],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Source,
                zone: ZoneKind::Hand,
                placement: crate::card::ZonePlacement::Top,
            },
        ),
    ),
);

// ICE 9 — Blue Scarab
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLUE_SCARAB: CardRecord = CardRecord::new(
    "Blue Scarab",
    "b423bb5a-eaac-4c1d-981a-1c635001fc5a",
    "Amy Weber",
    crate::card::CardRules::unsupported(),
);

// ICE 10 — Call to Arms
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CALL_TO_ARMS: CardRecord = CardRecord::new(
    "Call to Arms",
    "a92f0d4a-23d8-47d4-b910-d142e0eefd3d",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// ICE 11 — Caribou Range
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CARIBOU_RANGE: CardRecord = CardRecord::new(
    "Caribou Range",
    "1e5f8041-67fc-4e00-b119-d216e5cc5a3a",
    "Ruth Thompson",
    crate::card::CardRules::unsupported(),
);

// ICE 12 — Circle of Protection: Black (reprint)
const CIRCLE_OF_PROTECTION_BLACK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leb::CIRCLE_OF_PROTECTION_BLACK,
    "d528045d-3b80-48fd-b606-c132da052685",
    "Sandra Everingham",
);

// ICE 13 — Circle of Protection: Blue (reprint)
const CIRCLE_OF_PROTECTION_BLUE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_BLUE,
    "e0d377ec-c43c-43b9-934a-91b4d11650ab",
    "Pete Venters",
);

// ICE 14 — Circle of Protection: Green (reprint)
const CIRCLE_OF_PROTECTION_GREEN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_GREEN,
    "487dfb1f-b3ab-4daa-bbd9-c43dc91a5fba",
    "Sandra Everingham",
);

// ICE 15 — Circle of Protection: Red (reprint)
const CIRCLE_OF_PROTECTION_RED_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_RED,
    "5790ce22-a94f-402e-bcc7-b98f71af9fe5",
    "Pete Venters",
);

// ICE 16 — Circle of Protection: White (reprint)
const CIRCLE_OF_PROTECTION_WHITE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::CIRCLE_OF_PROTECTION_WHITE,
    "48bc4bb0-350c-424e-976e-b800915f7fb4",
    "Sandra Everingham",
);

// ICE 17 — Cold Snap
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COLD_SNAP: CardRecord = CardRecord::new(
    "Cold Snap",
    "81b87a58-b20c-4f38-afa3-59d398195740",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// ICE 18 — Cooperation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COOPERATION: CardRecord = CardRecord::new(
    "Cooperation",
    "21a815ed-c8b4-4414-8b27-ea612e2977e2",
    "Phil Foglio",
    crate::card::CardRules::unsupported(),
);

// ICE 19 — Death Ward (reprint)
const DEATH_WARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DEATH_WARD,
    "c7b21d29-050d-4704-a4c8-93e3b55086ac",
    "Harold McNeill",
);

// ICE 20 — Disenchant (reprint)
const DISENCHANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DISENCHANT,
    "b6085d0c-ab2b-445d-bf9d-0fa0a19183a2",
    "Brian Snõddy",
);

// ICE 21 — Drought
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DROUGHT: CardRecord = CardRecord::new(
    "Drought",
    "97736696-3de3-416d-94cf-4fac792f23f0",
    "NéNé Thomas",
    crate::card::CardRules::unsupported(),
);

// ICE 22 — Elvish Healer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELVISH_HEALER: CardRecord = CardRecord::new(
    "Elvish Healer",
    "00bd8485-d63a-4077-a3d1-4d0f2f4d8035",
    "Rick Emond",
    crate::card::CardRules::unsupported(),
);

// ICE 23 — Enduring Renewal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENDURING_RENEWAL: CardRecord = CardRecord::new(
    "Enduring Renewal",
    "be77edac-9a8b-4b7f-a859-27df76b10aa6",
    "Harold McNeill",
    crate::card::CardRules::unsupported(),
);

// ICE 24 — Energy Storm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENERGY_STORM: CardRecord = CardRecord::new(
    "Energy Storm",
    "3955e358-4285-44e2-9e24-9804346a6e58",
    "Sandra Everingham",
    crate::card::CardRules::unsupported(),
);

// ICE 25 — Formation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORMATION: CardRecord = CardRecord::new(
    "Formation",
    "78446ead-61b0-485f-a5a9-b3e72d8075a7",
    "Ken Meyer, Jr.",
    crate::card::CardRules::unsupported(),
);

// ICE 26 — Fylgja
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FYLGJA: CardRecord = CardRecord::new(
    "Fylgja",
    "3c6358a1-37f0-4b40-93d4-4f1652c38404",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// ICE 27 — General Jarkeld
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GENERAL_JARKELD: CardRecord = CardRecord::new(
    "General Jarkeld",
    "6a4f5a28-0bd2-4cc4-b67f-324e89193caa",
    "Richard Thomas",
    crate::card::CardRules::unsupported(),
);

// ICE 28 — Green Scarab
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GREEN_SCARAB: CardRecord = CardRecord::new(
    "Green Scarab",
    "0fbf9266-c97e-4666-b0fa-1802a69a62cc",
    "Nicola Leonard",
    crate::card::CardRules::unsupported(),
);

// ICE 29 — Hallowed Ground
pub(in crate::card::sets) static HALLOWED_GROUND: CardRecord = CardRecord::new(
    "Hallowed Ground",
    "4b35c0f4-5633-4ea9-9bda-daaf787aebdd",
    "Douglas Shuler",
    CardRules::new_enchantment(mana_cost!("{1}{W}")).with_ability(
        AbilityDef::activated_with_targets(
            "{W}{W}: Return target nonsnow land you control to its owner's hand.",
            &[CostDef::Mana(mana_cost!("{W}{W}"))],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(
                            CardSupertype::Snow,
                        )),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: crate::card::ZonePlacement::Top,
            },
        ),
    ),
);

// ICE 30 — Heal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEAL: CardRecord = CardRecord::new(
    "Heal",
    "9e6b2704-685e-4c74-875a-25846175e5e4",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// ICE 31 — Hipparion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HIPPARION: CardRecord = CardRecord::new(
    "Hipparion",
    "5969875a-f647-4daf-b76c-d1514d45c312",
    "Dameon Willich",
    crate::card::CardRules::unsupported(),
);

// ICE 32 — Justice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JUSTICE: CardRecord = CardRecord::new(
    "Justice",
    "9a6e0c8d-0fc1-4f52-8357-e550b0ac579a",
    "Ruth Thompson",
    crate::card::CardRules::unsupported(),
);

// ICE 33 — Kelsinko Ranger
pub(in crate::card::sets) static KELSINKO_RANGER: CardRecord = CardRecord::new(
    "Kelsinko Ranger",
    "8402543e-5406-404f-95c4-800a1dce35f1",
    "Mark Poole",
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Ranger"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{1}{W}: Target green creature gains first strike until end of turn.",
            &[CostDef::Mana(mana_cost!("{1}{W}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Color(ManaColor::Green),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ICE 34 — Kjeldoran Elite Guard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KJELDORAN_ELITE_GUARD: CardRecord = CardRecord::new(
    "Kjeldoran Elite Guard",
    "a73bc4b6-f7d0-494c-9e60-48279c11b7b6",
    "Melissa A. Benson",
    crate::card::CardRules::unsupported(),
);

// ICE 35 — Kjeldoran Guard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KJELDORAN_GUARD: CardRecord = CardRecord::new(
    "Kjeldoran Guard",
    "bdf41f17-8f82-4a8c-adec-0f3804faff3b",
    "Anthony S. Waters",
    crate::card::CardRules::unsupported(),
);

// ICE 36 — Kjeldoran Knight
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KJELDORAN_KNIGHT: CardRecord = CardRecord::new(
    "Kjeldoran Knight",
    "d5b9db8f-93b5-44e3-9e2b-728c80dfbb37",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// ICE 37 — Kjeldoran Phalanx
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KJELDORAN_PHALANX: CardRecord = CardRecord::new(
    "Kjeldoran Phalanx",
    "b6e91ba0-b229-4ab1-84f3-2a490dfa5051",
    "Richard Kane Ferguson",
    crate::card::CardRules::unsupported(),
);

// ICE 38 — Kjeldoran Royal Guard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KJELDORAN_ROYAL_GUARD: CardRecord = CardRecord::new(
    "Kjeldoran Royal Guard",
    "66343008-c38a-48a9-b767-fd2243103690",
    "Allen Williams",
    crate::card::CardRules::unsupported(),
);

// ICE 39 — Kjeldoran Skycaptain
pub(in crate::card::sets) static KJELDORAN_SKYCAPTAIN: CardRecord = CardRecord::new(
    "Kjeldoran Skycaptain",
    "cf0115e0-6192-48a9-9e58-f3ef77ef77c2",
    "Mark Poole",
    // Flying, first strike, and banding on one body: it wins the air and
    // hands you the damage assignment when it does not.
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Human", "Soldier"], 2, 2).with_abilities(&[
        abilities::flying(),
        abilities::first_strike(),
        abilities::banding(),
    ]),
);

// ICE 40 — Kjeldoran Skyknight
pub(in crate::card::sets) static KJELDORAN_SKYKNIGHT: CardRecord = CardRecord::new(
    "Kjeldoran Skyknight",
    "f794665a-8353-482a-b065-2a0777a8acda",
    "Mark Poole",
    // The cheap version of the same three keywords, on a body too small to
    // use most of them.
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Knight"], 1, 1).with_abilities(&[
        abilities::flying(),
        abilities::first_strike(),
        abilities::banding(),
    ]),
);

// ICE 41 — Kjeldoran Warrior
pub(in crate::card::sets) static KJELDORAN_WARRIOR: CardRecord = CardRecord::new(
    "Kjeldoran Warrior",
    "ce76f38f-566e-49ff-b197-510cfa1cb51c",
    "Mark Poole",
    // A one-mana banding body, which exists to let a real attacker join a
    // band rather than to attack itself.
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Warrior"], 1, 1)
        .with_ability(abilities::banding()),
);

// ICE 42 — Lightning Blow
pub(in crate::card::sets) static LIGHTNING_BLOW: CardRecord = CardRecord::new(
    "Lightning Blow",
    "d1a4ed99-f38c-4e0f-9ff2-2e1e9126e6ef",
    "Harold McNeill",
CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(
        AbilityDef::spell_with_targets(
            "Target creature gains first strike until end of turn.\nDraw a card at the beginning of the next turn's upkeep.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                DRAW_AT_NEXT_UPKEEP,
            ]),
        ),
    ),
);

// ICE 43 — Lost Order of Jarkeld
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LOST_ORDER_OF_JARKELD: CardRecord = CardRecord::new(
    "Lost Order of Jarkeld",
    "0f8fe1e5-69d2-401f-97cb-3cc01064bad3",
    "Andi Rusu",
    crate::card::CardRules::unsupported(),
);

// ICE 44 — Mercenaries
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MERCENARIES: CardRecord = CardRecord::new(
    "Mercenaries",
    "7b28762d-1ab7-460e-b433-27f5fa858959",
    "Cornelius Brudi",
    crate::card::CardRules::unsupported(),
);

// ICE 45 — Order of the Sacred Torch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORDER_OF_THE_SACRED_TORCH: CardRecord = CardRecord::new(
    "Order of the Sacred Torch",
    "ccc5cb36-c43d-4c71-8019-9b683e160a0a",
    "Ruth Thompson",
    crate::card::CardRules::unsupported(),
);

// ICE 46 — Order of the White Shield
pub(in crate::card::sets) static ORDER_OF_THE_WHITE_SHIELD: CardRecord = CardRecord::new(
    "Order of the White Shield",
    "92e55b10-375f-4b4f-b676-3b9b8085fdd2",
    "Ruth Thompson",
    CardRules::new_creature(mana_cost!("{W}{W}"), &["Human", "Knight"], 2, 1).with_abilities(&[
        abilities::protection_from_color(ManaColor::Black),
        abilities::apply_to_self_until_end_of_turn(
            "{W}: This creature gains first strike until end of turn.",
            &[CostDef::Mana(mana_cost!("{W}"))],
            AppliedEffectDef::add_ability(&abilities::first_strike()),
        ),
        AbilityDef::activated(
            "{W}{W}: This creature gets +1/+0 until end of turn.",
            &[CostDef::Mana(mana_cost!("{W}{W}"))],
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

// ICE 47 — Prismatic Ward
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PRISMATIC_WARD: CardRecord = CardRecord::new(
    "Prismatic Ward",
    "6f8b50fd-3d1d-4ea8-a3c7-98ca7a8a455e",
    "Allen Williams",
    crate::card::CardRules::unsupported(),
);

// ICE 48 — Rally
pub(in crate::card::sets) static RALLY: CardRecord = CardRecord::new(
    "Rally",
    "e1e9f80e-5d75-45b7-9c66-c0f30996f4dc",
    "Heather Hudson",
    CardRules::new_instant(mana_cost!("{W}{W}")).with_ability(AbilityDef::spell(
        "Blocking creatures get +1/+1 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::Blocking,
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(1),
                ValueDef::Constant(1),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// ICE 49 — Red Scarab
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RED_SCARAB: CardRecord = CardRecord::new(
    "Red Scarab",
    "9a734154-5944-42f4-a02e-c426a45847f3",
    "Sandra Everingham",
    crate::card::CardRules::unsupported(),
);

// ICE 50 — Sacred Boon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SACRED_BOON: CardRecord = CardRecord::new(
    "Sacred Boon",
    "d721569d-9cf2-4c3c-b11c-4c46c258a0d2",
    "Mike Raabe",
    crate::card::CardRules::unsupported(),
);

// ICE 51 — Seraph
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SERAPH: CardRecord = CardRecord::new(
    "Seraph",
    "ab675291-3189-43f3-b11b-0724eca8b941",
    "Christopher Rush",
    crate::card::CardRules::unsupported(),
);

// ICE 52 — Shield Bearer
pub(in crate::card::sets) static SHIELD_BEARER: CardRecord = CardRecord::new(
    "Shield Bearer",
    "318ff2da-d309-469c-8e2f-fa3c7517a15a",
    "Dan Frazier",
    // Zero power and banding: it blocks, and it hands the damage
    // assignment to you, which is the whole of what it does.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Soldier"], 0, 3)
        .with_ability(abilities::banding()),
);

// ICE 53 — Snow Hound
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SNOW_HOUND: CardRecord = CardRecord::new(
    "Snow Hound",
    "084437ba-26d4-4af6-ab00-dcb145dd2cd0",
    "Pat Lewis",
    crate::card::CardRules::unsupported(),
);

// ICE 54 — Swords to Plowshares (reprint)
const SWORDS_TO_PLOWSHARES_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWORDS_TO_PLOWSHARES,
    "375fd2cb-443b-4be4-ad60-6d1a8e74f510",
    "Kaja Foglio",
);

// ICE 55 — Warning
pub(in crate::card::sets) static WARNING: CardRecord = CardRecord::new(
    "Warning",
    "cca5b4a7-df11-4635-a147-df12cd13a67c",
    "Pat Lewis",
    // One mana that blanks an attacker without killing it, and leaves the
    // blocker free to kill it back.
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell_with_targets(
        "Prevent all combat damage that would be dealt by target attacking creature this turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Attacking,
            ]),
        )],
        // Damage dealt *by* it, so it still takes damage from whatever blocks
        // it -- this saves the blocker, not the attacker.
        EffectDef::PreventDamage {
            prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::combat_from(
                ObjectRefDef::Target(TargetIndex::PRIMARY),
            )),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// ICE 56 — White Scarab
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WHITE_SCARAB: CardRecord = CardRecord::new(
    "White Scarab",
    "c57726b5-dfdd-4e47-bc52-ebf6eedbf3bd",
    "Phil Foglio",
    crate::card::CardRules::unsupported(),
);

// ICE 57 — Arnjlot's Ascent
pub(in crate::card::sets) static ARNJLOT_S_ASCENT: CardRecord = CardRecord::new(
    "Arnjlot's Ascent",
    "2307fb16-8b77-45b5-8a02-51a13214791d",
    "Drew Tucker",
    // Evasion by the mana rather than by the card: one mana a turn keeps the
    // biggest creature on the board flying over everything.
    CardRules::new_enchantment(mana_cost!("{1}{U}{U}")).with_abilities(&[
        abilities::cumulative_upkeep(&[CostDef::mana(mana_cost!("{U}"))]),
        AbilityDef::activated_with_targets(
            "{1}: Target creature gains flying until end of turn.",
            &[CostDef::Mana(mana_cost!("{1}"))],
            &const {
                [AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )]
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&const { abilities::flying() }),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// ICE 58 — Balduvian Conjurer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BALDUVIAN_CONJURER: CardRecord = CardRecord::new(
    "Balduvian Conjurer",
    "5b616963-fac0-451c-8df4-2cacc9466b17",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// ICE 59 — Balduvian Shaman
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BALDUVIAN_SHAMAN: CardRecord = CardRecord::new(
    "Balduvian Shaman",
    "74859723-8ddf-4ee6-a0a7-87192c84e8ad",
    "Quinton Hoover",
    crate::card::CardRules::unsupported(),
);

// ICE 60 — Binding Grasp
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BINDING_GRASP: CardRecord = CardRecord::new(
    "Binding Grasp",
    "6b086186-5fbf-4ba7-af0d-ee3ad61d27bb",
    "Ruth Thompson",
    crate::card::CardRules::unsupported(),
);

// ICE 61 — Brainstorm
pub(in crate::card::sets) static BRAINSTORM: CardRecord = CardRecord::new(
    "Brainstorm",
    "8d42d7aa-7f53-4cfc-842a-086aab2448d1",
    "Christopher Rush",
    // One mana, no card advantage, and the best blue card in the format:
    // what it buys is the top of the library, and a fetchland turns the two
    // cards put back into two cards nobody has to draw.
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell(
        "Draw three cards, then put two cards from your hand on top of your library in any \
             order.",
        abilities::brainstorm(),
    )),
);

// ICE 62 — Breath of Dreams
pub(in crate::card::sets) static BREATH_OF_DREAMS: CardRecord = CardRecord::new(
    "Breath of Dreams",
    "e40c9657-fab4-489d-8eb0-960ba2605add",
    "Phil Foglio",
    CardRules::new_enchantment(mana_cost!("{2}{U}{U}")).with_abilities(&[
        abilities::cumulative_upkeep(&[CostDef::mana(mana_cost!("{U}"))])
            .override_text("Cumulative upkeep {U}"),
        AbilityDef::static_ability(
            "Green creatures have cumulative upkeep {1}.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Color(ManaColor::Green),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::cumulative_upkeep(&[
                    CostDef::mana(mana_cost!("{1}")),
                ])),
            },
        ),
    ]),
);

// ICE 63 — Clairvoyance
pub(in crate::card::sets) static CLAIRVOYANCE: CardRecord = CardRecord::new(
    "Clairvoyance",
    "46740353-e2ba-4d80-a97d-1368bc67bf30",
    "Ken Meyer, Jr.",
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell_with_targets(
        "Look at target player's hand.\nDraw a card at the beginning of the next turn's upkeep.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Sequence(&[
            EffectDef::LookAtHand {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
            DRAW_AT_NEXT_UPKEEP,
        ]),
    )),
);

// ICE 64 — Counterspell (reprint)
const COUNTERSPELL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::COUNTERSPELL,
    "aedbcbaa-40f0-485f-8427-778edc2d2ec0",
    "Allen Williams",
);

// ICE 65 — Deflection
pub(in crate::card::sets) static DEFLECTION: CardRecord = CardRecord::new(
    "Deflection",
    "1005a00a-6a0e-44cb-abea-37e2e53125e2",
    "Mike Raabe",
    CardRules::new_instant(mana_cost!("{3}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Change the target of target spell with a single target.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Spell,
                    ObjectPredicateDef::DeclaredTargetCount {
                        minimum: 1,
                        maximum: 1,
                    },
                ]),
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::ChangeStackTargets(&crate::card::ChangeStackTargetsDef {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            chooser: PlayerRefDef::EffectController,
            change: crate::card::StackTargetChangeDef::ChooseNew {
                optional: false,
                restriction: None,
            },
        }),
    )),
);

// ICE 66 — Dreams of the Dead
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DREAMS_OF_THE_DEAD: CardRecord = CardRecord::new(
    "Dreams of the Dead",
    "93372854-57e7-4db7-a1a6-376c9f49a514",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// ICE 67 — Enervate
pub(in crate::card::sets) static ENERVATE: CardRecord = CardRecord::new(
    "Enervate",
    "c4fdfc5b-c2ab-4c4d-b120-301e17f3d9c6",
    "Allen Williams",
CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Tap target artifact, creature, or land.\nDraw a card at the beginning of the next turn's upkeep.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::HasType(CardType::Land),
            ]),
        )],
        EffectDef::Sequence(&[
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
            DRAW_AT_NEXT_UPKEEP,
        ]),
    )),
);

// ICE 68 — Errant Minion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ERRANT_MINION: CardRecord = CardRecord::new(
    "Errant Minion",
    "61648ddb-6efb-43d0-b2b1-418cc957854c",
    "Harold McNeill",
    crate::card::CardRules::unsupported(),
);

// ICE 69 — Essence Flare
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ESSENCE_FLARE: CardRecord = CardRecord::new(
    "Essence Flare",
    "13ebb5dd-d7f1-4b06-8585-7004045be542",
    "Richard Kane Ferguson",
    crate::card::CardRules::unsupported(),
);

// ICE 70 — Force Void
pub(in crate::card::sets) static FORCE_VOID: CardRecord = CardRecord::new(
    "Force Void",
    "226555ba-22af-45f1-a3f4-d265f8685dd5",
    "Mark Tedin",
CardRules::new_instant(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target spell unless its controller pays {1}.\nDraw a card at the beginning of the next turn's upkeep.",
        &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::Spell,
            zones: &[ZoneKind::Stack],
            controller: None,
            owner: None,
        })],
        EffectDef::Sequence(&[
            abilities::counter_target_unless_paid(&[CostDef::GenericMana(ValueDef::Constant(1))]),
            DRAW_AT_NEXT_UPKEEP,
        ]),
    )),
);

// ICE 71 — Glacial Wall
pub(in crate::card::sets) static GLACIAL_WALL: CardRecord = CardRecord::new(
    "Glacial Wall",
    "07b71bc1-d9a2-4e99-a8fa-cd696925328d",
    "Dameon Willich",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Wall"], 0, 7)
        .with_ability(abilities::defender()),
);

// ICE 72 — Hydroblast
pub(in crate::card::sets) static HYDROBLAST: CardRecord = CardRecord::new(
    "Hydroblast",
    "f62716f0-fde2-49ef-b8a4-c1b03f451194",
    "Kaja Foglio",
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::counter_target(
                "Counter target spell if it's red.",
                &AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Spell,
                        ObjectPredicateDef::Color(ManaColor::Red),
                    ]),
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                }),
            ),
            AbilityDef::destroy_target(
                "Destroy target permanent if it's red.",
                &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Color(ManaColor::Red)),
            ),
        ],
    )),
);

// ICE 73 — Iceberg
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ICEBERG: CardRecord = CardRecord::new(
    "Iceberg",
    "a2f70e49-17fa-4033-bd45-63374f7f5ec5",
    "Jeff A. Menges",
    crate::card::CardRules::unsupported(),
);

// ICE 74 — Icy Prison
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ICY_PRISON: CardRecord = CardRecord::new(
    "Icy Prison",
    "39a7e496-8d2e-49db-b298-475d9017537a",
    "Anson Maddocks",
    crate::card::CardRules::unsupported(),
);

// ICE 75 — Illusionary Forces
pub(in crate::card::sets) static ILLUSIONARY_FORCES: CardRecord = CardRecord::new(
    "Illusionary Forces",
    "ab02268e-01cf-4729-95ca-5773afd40b56",
    "Justin Hampton",
// A 4/4 flier for four, rented one blue mana at a time -- and the rent
    // goes up every turn it stays.
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Illusion"], 4, 4).with_abilities(&[
        abilities::flying(),
        abilities::cumulative_upkeep(
            &[CostDef::Mana(mana_cost!("{U}"))],
        ).override_text(
                "Cumulative upkeep {U} (At the beginning of your upkeep, put an age counter on this permanent, then sacrifice it unless you pay its upkeep cost for each age counter on it.)",
            ),
    ]),
);

// ICE 76 — Illusionary Presence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ILLUSIONARY_PRESENCE: CardRecord = CardRecord::new(
    "Illusionary Presence",
    "aa31efed-4a11-4f59-a623-bac45d20091d",
    "Kaja Foglio",
    crate::card::CardRules::unsupported(),
);

// ICE 77 — Illusionary Terrain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ILLUSIONARY_TERRAIN: CardRecord = CardRecord::new(
    "Illusionary Terrain",
    "691f4a1b-4706-41aa-82da-ae920739f036",
    "Rob Alexander",
    crate::card::CardRules::unsupported(),
);

// ICE 78 — Illusionary Wall
pub(in crate::card::sets) static ILLUSIONARY_WALL: CardRecord = CardRecord::new(
    "Illusionary Wall",
    "6430e8e2-fee3-4744-820e-d6e16cb992bd",
    "Mark Poole",
// Nothing gets past it on the ground or in the air, for as long as the
    // blue mana holds out.
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Illusion", "Wall"], 7, 4).with_abilities(&[
        abilities::defender(),
        abilities::flying(),
        abilities::first_strike(),
        abilities::cumulative_upkeep(
            &[CostDef::Mana(mana_cost!("{U}"))],
        ).override_text(
                "Cumulative upkeep {U} (At the beginning of your upkeep, put an age counter on this permanent, then sacrifice it unless you pay its upkeep cost for each age counter on it.)",
            ),
    ]),
);

// ICE 79 — Illusions of Grandeur
pub(in crate::card::sets) static ILLUSIONS_OF_GRANDEUR: CardRecord = CardRecord::new(
    "Illusions of Grandeur",
    "17eeeef2-2ced-42b8-a5e0-1095c9e13b02",
    "Quinton Hoover",
    CardRules::new_enchantment(mana_cost!("{3}{U}")).with_abilities(&[
        abilities::cumulative_upkeep(&[CostDef::mana(mana_cost!("{2}"))]),
        abilities::enters_trigger(
            "When this enchantment enters, you gain 20 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(20),
            },
        ),
        AbilityDef::triggered(
            "When this enchantment leaves the battlefield, you lose 20 life.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                None,
            ),
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(20),
            },
        ),
    ]),
);

// ICE 80 — Infuse
pub(in crate::card::sets) static INFUSE: CardRecord = CardRecord::new(
    "Infuse",
    "223287b6-224c-4e00-946c-e7ac5539bd45",
    "Randy Gallegos",
CardRules::new_instant(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Untap target artifact, creature, or land.\nDraw a card at the beginning of the next turn's upkeep.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::HasType(CardType::Land),
            ]),
        )],
        EffectDef::Sequence(&[
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
            DRAW_AT_NEXT_UPKEEP,
        ]),
    )),
);

// ICE 81 — Krovikan Sorcerer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KROVIKAN_SORCERER: CardRecord = CardRecord::new(
    "Krovikan Sorcerer",
    "9c5fc053-7b0b-4e76-bf87-ccdb1e8752ed",
    "Pat Lewis",
    crate::card::CardRules::unsupported(),
);

// ICE 82 — Magus of the Unseen
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MAGUS_OF_THE_UNSEEN: CardRecord = CardRecord::new(
    "Magus of the Unseen",
    "86da04e9-b94d-42af-add3-02baf772bd33",
    "Kaja Foglio",
    crate::card::CardRules::unsupported(),
);

// ICE 83 — Mesmeric Trance
pub(in crate::card::sets) static MESMERIC_TRANCE: CardRecord = CardRecord::new(
    "Mesmeric Trance",
    "ae3df593-e9d5-479d-9a9a-1c7262dd9c6c",
    "Dan Frazier",
    // A looter that costs mana instead of a card, so the rent it charges is the
    // only thing stopping it from filtering the whole deck.
    CardRules::new_enchantment(mana_cost!("{1}{U}{U}")).with_abilities(&[
        abilities::cumulative_upkeep(&[CostDef::mana(mana_cost!("{1}"))]),
        AbilityDef::activated(
            "{U}, Discard a card: Draw a card.",
            &[
                CostDef::Mana(mana_cost!("{U}")),
                CostDef::discard(ObjectPredicateDef::Any),
            ],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// ICE 84 — Mistfolk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISTFOLK: CardRecord = CardRecord::new(
    "Mistfolk",
    "4f3f4d4e-ca4a-4fba-b9fd-cd1d9457cfa1",
    "Quinton Hoover",
    crate::card::CardRules::unsupported(),
);

// ICE 85 — Musician
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MUSICIAN: CardRecord = CardRecord::new(
    "Musician",
    "9f8d2247-a10e-413a-b497-2add3918f991",
    "Drew Tucker",
    crate::card::CardRules::unsupported(),
);

// ICE 86 — Mystic Might
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MYSTIC_MIGHT: CardRecord = CardRecord::new(
    "Mystic Might",
    "e35d7f08-0687-41bd-8c53-31a49adabb11",
    "Nicola Leonard",
    crate::card::CardRules::unsupported(),
);

// ICE 87 — Mystic Remora
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MYSTIC_REMORA: CardRecord = CardRecord::new(
    "Mystic Remora",
    "58e93dff-b774-4765-b7bd-d3957e42ff4a",
    "Ken Meyer, Jr.",
    crate::card::CardRules::unsupported(),
);

// ICE 88 — Phantasmal Mount
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHANTASMAL_MOUNT: CardRecord = CardRecord::new(
    "Phantasmal Mount",
    "75afdbe6-a3f9-49cf-b4ef-f370e518e960",
    "Melissa A. Benson",
    crate::card::CardRules::unsupported(),
);

// ICE 89 — Polar Kraken
pub(in crate::card::sets) static POLAR_KRAKEN: CardRecord = CardRecord::new(
    "Polar Kraken",
    "aee01e9c-0445-4228-a73a-3e5744844ed3",
    "Mark Tedin",
    CardRules::new_creature(mana_cost!("{8}{U}{U}{U}"), &["Kraken"], 11, 11).with_abilities(&[
        abilities::trample(),
        abilities::enters_tapped(CardType::Creature),
        abilities::cumulative_upkeep(&[actions::choose_sacrifice(1)
            .matching(ObjectPredicateDef::HasType(CardType::Land))
            .as_cost()])
        .override_text("Cumulative upkeep—Sacrifice a land."),
    ]),
);

// ICE 90 — Portent
pub(in crate::card::sets) static PORTENT: CardRecord = CardRecord::new(
    "Portent",
    "e040be83-3fb5-4da5-ba7a-4923b8854b74",
    "Liz Danforth",
CardRules::new_sorcery(mana_cost!("{U}")).with_ability(AbilityDef::spell_with_targets(
        "Look at the top three cards of target player's library, then put them back in any order. You may have that player shuffle.\nDraw a card at the beginning of the next turn's upkeep.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Sequence(&[
            abilities::look_at_top_cards_and_reorder(
                PlayerRefDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(3),
            ),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::ShuffleLibrary {
                    player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            },
            EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                "At the beginning of the next turn's upkeep, draw a card.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::Any,
                },
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ))),
        ]),
    )),
);

// ICE 91 — Power Sink (reprint)
const POWER_SINK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::POWER_SINK,
    "85cbec45-81b4-40cc-b356-d6713a6a9b2b",
    "Mark Poole",
);

// ICE 92 — Ray of Command
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAY_OF_COMMAND: CardRecord = CardRecord::new(
    "Ray of Command",
    "638abe5f-2a8a-42ca-bcdf-a52a3df66946",
    "Harold McNeill",
    crate::card::CardRules::unsupported(),
);

// ICE 93 — Ray of Erasure
pub(in crate::card::sets) static RAY_OF_ERASURE: CardRecord = CardRecord::new(
    "Ray of Erasure",
    "5a09fc0b-7b9c-4283-8336-f2607f5ffaf5",
    "Mike Raabe",
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell_with_targets(
        "Target player mills a card.\nDraw a card at the beginning of the next turn's upkeep.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Sequence(&[
            EffectDef::Mill {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
            DRAW_AT_NEXT_UPKEEP,
        ]),
    )),
);

// ICE 94 — Reality Twist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REALITY_TWIST: CardRecord = CardRecord::new(
    "Reality Twist",
    "1b7e955c-3de2-430c-93b9-0b39ccea5420",
    "James Ernest",
    crate::card::CardRules::unsupported(),
);

// ICE 95 — Sea Spirit
pub(in crate::card::sets) static SEA_SPIRIT: CardRecord = CardRecord::new(
    "Sea Spirit",
    "f2d93d05-98bc-4504-9045-dedb925895ae",
    "Rob Alexander",
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Elemental", "Spirit"], 2, 3).with_ability(
        AbilityDef::activated(
            "{U}: This creature gets +1/+0 until end of turn.",
            &[CostDef::Mana(mana_cost!("{U}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ICE 96 — Shyft
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHYFT: CardRecord = CardRecord::new(
    "Shyft",
    "99a60c33-b641-42c4-870d-95d07bc975dc",
    "Richard Thomas",
    crate::card::CardRules::unsupported(),
);

// ICE 97 — Sibilant Spirit
pub(in crate::card::sets) static SIBILANT_SPIRIT: CardRecord = CardRecord::new(
    "Sibilant Spirit",
    "47364ad2-5ce9-4b19-a9d2-f6a33188b882",
    "Ron Spencer",
    CardRules::new_creature(mana_cost!("{5}{U}"), &["Spirit"], 5, 6).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever this creature attacks, defending player may draw a card.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::May {
                player: EffectRecipientDef::Opponent,
                effect: &EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Opponent,
                    amount: ValueDef::Constant(1),
                },
            },
        ),
    ]),
);

// ICE 98 — Silver Erne
pub(in crate::card::sets) static SILVER_ERNE: CardRecord = CardRecord::new(
    "Silver Erne",
    "685076cc-098c-4f98-918c-0ad825eda10f",
    "Melissa A. Benson",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Bird"], 2, 2)
        .with_abilities(&[abilities::flying(), abilities::trample()]),
);

// ICE 99 — Sleight of Mind (reprint)
const SLEIGHT_OF_MIND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SLEIGHT_OF_MIND,
    "93dc9f02-11ad-4c4a-8199-9d20c23d31a7",
    "Nicola Leonard",
);

// ICE 100 — Snow Devil
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SNOW_DEVIL: CardRecord = CardRecord::new(
    "Snow Devil",
    "2be3a9a5-2ac5-4ea4-915d-8cff35c0e72f",
    "Ken Meyer, Jr.",
    crate::card::CardRules::unsupported(),
);

// ICE 101 — Snowfall
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SNOWFALL: CardRecord = CardRecord::new(
    "Snowfall",
    "788ed793-3993-4a63-b9f9-9ac3947c3108",
    "Phil Foglio",
    crate::card::CardRules::unsupported(),
);

// ICE 102 — Soldevi Machinist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOLDEVI_MACHINIST: CardRecord = CardRecord::new(
    "Soldevi Machinist",
    "1f0999df-2f94-499e-b9af-fe377d515400",
    "Jeff A. Menges",
    crate::card::CardRules::unsupported(),
);

// ICE 103 — Soul Barrier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOUL_BARRIER: CardRecord = CardRecord::new(
    "Soul Barrier",
    "9ad7fac7-db4d-45b2-aba6-16f4fd1a586f",
    "Harold McNeill",
    crate::card::CardRules::unsupported(),
);

// ICE 104 — Thunder Wall
pub(in crate::card::sets) static THUNDER_WALL: CardRecord = CardRecord::new(
    "Thunder Wall",
    "4fc5d510-c4f7-4a09-bf86-83c3fa3f8928",
    "Richard Thomas",
    CardRules::new_creature(mana_cost!("{1}{U}{U}"), &["Wall"], 0, 2).with_abilities(&[
        abilities::defender(),
        abilities::flying(),
        AbilityDef::activated(
            "{U}: This creature gets +1/+1 until end of turn.",
            &[CostDef::Mana(mana_cost!("{U}"))],
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

// ICE 105 — Updraft
pub(in crate::card::sets) static UPDRAFT: CardRecord = CardRecord::new(
    "Updraft",
    "d1bd4e16-27fe-4c7b-ae25-78ed77d8e8e7",
    "Allen Williams",
CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gains flying until end of turn.\nDraw a card at the beginning of the next turn's upkeep.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::flying()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            DRAW_AT_NEXT_UPKEEP,
        ]),
    )),
);

// ICE 106 — Wind Spirit
pub(in crate::card::sets) static WIND_SPIRIT: CardRecord = CardRecord::new(
    "Wind Spirit",
    "4d882447-9594-4aab-b1a7-8bb275f250cf",
    "Kaja Foglio",
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Elemental", "Spirit"], 3, 2)
        .with_abilities(&[abilities::flying(), abilities::menace()]),
);

// ICE 107 — Winter's Chill
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WINTER_S_CHILL: CardRecord = CardRecord::new(
    "Winter's Chill",
    "a779aca7-ff2c-48d8-9484-6ad04b2c6bcb",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// ICE 108 — Word of Undoing
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WORD_OF_UNDOING: CardRecord = CardRecord::new(
    "Word of Undoing",
    "22b04476-5a5d-4843-a948-82db209c4218",
    "Christopher Rush",
    crate::card::CardRules::unsupported(),
);

// ICE 109 — Wrath of Marit Lage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WRATH_OF_MARIT_LAGE: CardRecord = CardRecord::new(
    "Wrath of Marit Lage",
    "1d512f5c-0327-4d49-8a26-672574a49102",
    "Mike Raabe",
    crate::card::CardRules::unsupported(),
);

// ICE 110 — Zur's Weirding
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ZUR_S_WEIRDING: CardRecord = CardRecord::new(
    "Zur's Weirding",
    "e1f8531f-19ca-48a2-baf2-c5dc6f18d79c",
    "Liz Danforth",
    crate::card::CardRules::unsupported(),
);

// ICE 111 — Zuran Enchanter
pub(in crate::card::sets) static ZURAN_ENCHANTER: CardRecord = CardRecord::new(
    "Zuran Enchanter",
    "721edcef-f40a-4d43-9d80-26161dc425cb",
    "Douglas Shuler",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Human", "Wizard"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{2}{B}, {T}: Target player discards a card. Activate only during your turn.",
            &[CostDef::Mana(mana_cost!("{2}{B}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Discard {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
                selection: crate::card::DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        )
        .with_activation_timing(ActivationTimingDef::YourTurn),
    ),
);

// ICE 112 — Zuran Spellcaster
pub(in crate::card::sets) static ZURAN_SPELLCASTER: CardRecord = CardRecord::new(
    "Zuran Spellcaster",
    "152a72b1-a7b7-4e5c-8558-fab97465f549",
    "Edward P. Beard, Jr.",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Wizard"], 1, 1).with_ability(
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
    ),
);

// ICE 113 — Abyssal Specter
pub(in crate::card::sets) static ABYSSAL_SPECTER: CardRecord = CardRecord::new(
    "Abyssal Specter",
    "fc26f19c-bcf7-4bd8-af42-4757dbe47fb1",
    "Ruth Thompson",
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Specter"], 2, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever this creature deals damage to a player, that player discards a card.",
            TriggerEventDef::damage_to_player(ObjectPredicateDef::Source, PlayerRelation::Any),
            EffectDef::Discard {
                recipient: EffectRecipientDef::EventPlayer,
                amount: ValueDef::Constant(1),
                selection: crate::card::DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ),
    ]),
);

// ICE 114 — Ashen Ghoul
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ASHEN_GHOUL: CardRecord = CardRecord::new(
    "Ashen Ghoul",
    "6bb83301-5662-4628-b536-6a3ee0296f2e",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// ICE 115 — Brine Shaman
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRINE_SHAMAN: CardRecord = CardRecord::new(
    "Brine Shaman",
    "f445962c-44a1-4f3f-88d4-17048f8ca9dc",
    "Cornelius Brudi",
    crate::card::CardRules::unsupported(),
);

// ICE 116 — Burnt Offering
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BURNT_OFFERING: CardRecord = CardRecord::new(
    "Burnt Offering",
    "1dae52a2-3af7-4b97-9d2e-2448b7c413fb",
    "Daniel Gelon",
    crate::card::CardRules::unsupported(),
);

// ICE 117 — Cloak of Confusion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLOAK_OF_CONFUSION: CardRecord = CardRecord::new(
    "Cloak of Confusion",
    "dc45d103-0fca-4431-a5c0-869f0f9be93e",
    "Margaret Organ-Kean",
    crate::card::CardRules::unsupported(),
);

// ICE 118 — Dance of the Dead
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DANCE_OF_THE_DEAD: CardRecord = CardRecord::new(
    "Dance of the Dead",
    "e7c53ba4-9956-4cd6-85ca-2d6b61a5127c",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// ICE 119 — Dark Banishing
pub(in crate::card::sets) static DARK_BANISHING: CardRecord = CardRecord::new(
    "Dark Banishing",
    "f7dc2716-ed62-4797-ad2b-227eca5408d0",
    "Drew Tucker",
    CardRules::new_instant(mana_cost!("{2}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target nonblack creature. It can't be regenerated.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Not(&ObjectPredicateDef::Color(ManaColor::Black)),
            ]),
        )],
        EffectDef::WithRule {
            rule: AppliedRuleDef::CannotRegenerate,
            effect: &EffectDef::destroy_target(TargetIndex::PRIMARY),
        },
    )),
);

// ICE 120 — Dark Ritual (reprint)
const DARK_RITUAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DARK_RITUAL,
    "4ebcd681-1871-4914-bcd7-6bd95829f6e0",
    "Justin Hampton",
);

// ICE 121 — Demonic Consultation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEMONIC_CONSULTATION: CardRecord = CardRecord::new(
    "Demonic Consultation",
    "8d727b9b-6114-414d-9172-16b6e1db41cc",
    "Rob Alexander",
    crate::card::CardRules::unsupported(),
);

// ICE 122 — Dread Wight
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DREAD_WIGHT: CardRecord = CardRecord::new(
    "Dread Wight",
    "65d332e2-4b2d-4131-84f7-862cb138c477",
    "Daniel Gelon",
    crate::card::CardRules::unsupported(),
);

// ICE 123 — Drift of the Dead
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRIFT_OF_THE_DEAD: CardRecord = CardRecord::new(
    "Drift of the Dead",
    "d8b65656-9f8c-4179-81aa-4b15d8280baa",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// ICE 124 — Fear (reprint)
const FEAR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FEAR,
    "5709398f-0744-4780-a1d2-eead96c8f348",
    "Rick Emond",
);

// ICE 125 — Flow of Maggots
pub(in crate::card::sets) static FLOW_OF_MAGGOTS: CardRecord = CardRecord::new(
    "Flow of Maggots",
    "6880a4d3-5cbc-4a01-9190-3565617efcc9",
    "Ron Spencer",
    // Unblockable in practice, since a deck that kept Walls around to stop it
    // has already given up the initiative -- and the rent is only one mana.
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Insect"], 2, 2).with_abilities(&[
        abilities::cumulative_upkeep(&[CostDef::mana(mana_cost!("{1}"))]),
        AbilityDef::static_ability(
            "This creature can't be blocked by non-Wall creatures.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype(SubtypeDef::Literal(
                        "Wall",
                    ))),
                )),
            },
        ),
    ]),
);

// ICE 126 — Foul Familiar
pub(in crate::card::sets) static FOUL_FAMILIAR: CardRecord = CardRecord::new(
    "Foul Familiar",
    "8bad3541-8e40-4a2f-ac9d-f7b61f3d75a1",
    "Anson Maddocks",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Spirit"], 3, 1).with_abilities(&[
        AbilityDef::static_ability(
            "This creature can't block.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
            },
        ),
        AbilityDef::activated(
            "{B}, Pay 1 life: Return this creature to its owner's hand.",
            &[CostDef::Mana(mana_cost!("{B}")), CostDef::PayLife(1)],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Source,
                zone: ZoneKind::Hand,
                placement: crate::card::ZonePlacement::Top,
            },
        ),
    ]),
);

// ICE 127 — Gangrenous Zombies
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GANGRENOUS_ZOMBIES: CardRecord = CardRecord::new(
    "Gangrenous Zombies",
    "08be4d83-99be-4360-90f1-104dee1c3c2f",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// ICE 128 — Gaze of Pain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GAZE_OF_PAIN: CardRecord = CardRecord::new(
    "Gaze of Pain",
    "48401643-ec4b-444a-8f9a-1a5ea471ff4a",
    "Anson Maddocks",
    crate::card::CardRules::unsupported(),
);

// ICE 129 — Gravebind
pub(in crate::card::sets) static GRAVEBIND: CardRecord = CardRecord::new(
    "Gravebind",
    "4782fd4f-2474-4d0d-8301-e0b52af93746",
    "Drew Tucker",
CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature can't be regenerated this turn.\nDraw a card at the beginning of the next turn's upkeep.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotRegenerate),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            DRAW_AT_NEXT_UPKEEP,
        ]),
    )),
);

// ICE 130 — Hecatomb
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HECATOMB: CardRecord = CardRecord::new(
    "Hecatomb",
    "8f59620f-ff9e-44d8-9c4e-be9de1a919e8",
    "NéNé Thomas",
    crate::card::CardRules::unsupported(),
);

// ICE 131 — Hoar Shade
pub(in crate::card::sets) static HOAR_SHADE: CardRecord = CardRecord::new(
    "Hoar Shade",
    "72242dff-15ca-4da0-b3ae-9984d037b31f",
    "Richard Thomas",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Shade"], 1, 2).with_ability(
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

// ICE 132 — Howl from Beyond (reprint)
const HOWL_FROM_BEYOND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HOWL_FROM_BEYOND,
    "ca9d0d6b-056e-4b94-8de5-a325768f67b6",
    "Mark Poole",
);

// ICE 133 — Hyalopterous Lemure
pub(in crate::card::sets) static HYALOPTEROUS_LEMURE: CardRecord = CardRecord::new(
    "Hyalopterous Lemure",
    "d2c9e037-f4d5-46fd-b439-56bee6fb2ad3",
    "Richard Thomas",
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Spirit"], 4, 3).with_ability(
        AbilityDef::activated(
            "{0}: This creature gets -1/-0 and gains flying until end of turn.",
            &[],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-1),
                        ValueDef::Constant(0),
                    ),
                    AppliedEffectDef::add_ability(&abilities::flying()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ICE 134 — Icequake
pub(in crate::card::sets) static ICEQUAKE: CardRecord = CardRecord::new(
    "Icequake",
    "14b4dd4d-c617-4603-8a87-761ec6fc6883",
    "Richard Kane Ferguson",
    // Three mana to kill a land, with a point of damage against the decks
    // this block was printed to punish.
    CardRules::new_sorcery(mana_cost!("{1}{B}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target land. If that land was a snow land, Icequake deals 1 damage to \
             that land's controller.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Land),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
            // "Was a snow land" is read after the destruction, so the slot is
            // asked about a land that has already left.
            EffectDef::IfCondition {
                condition: &const {
                    TriggerConditionDef::TargetMatches {
                        slot: TargetIndex::PRIMARY,
                        object: ObjectPredicateDef::Supertype(CardSupertype::Snow),
                    }
                },
                then: &const {
                    EffectDef::damage(
                        EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                            ObjectRefDef::Target(TargetIndex::PRIMARY),
                        )),
                        ValueDef::Constant(1),
                    )
                },
            },
        ]),
    )),
);

// ICE 135 — Infernal Darkness
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INFERNAL_DARKNESS: CardRecord = CardRecord::new(
    "Infernal Darkness",
    "f3475eb3-909d-450b-9597-b241b259b425",
    "Phil Foglio",
    crate::card::CardRules::unsupported(),
);

// ICE 136 — Infernal Denizen
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INFERNAL_DENIZEN: CardRecord = CardRecord::new(
    "Infernal Denizen",
    "b63ac9a6-aaa5-4659-97d1-c5f6b0d5ccfe",
    "Drew Tucker",
    crate::card::CardRules::unsupported(),
);

// ICE 137 — Kjeldoran Dead
pub(in crate::card::sets) static KJELDORAN_DEAD: CardRecord = CardRecord::new(
    "Kjeldoran Dead",
    "d3f7b614-6075-4b7c-acc7-ab63185b570b",
    "Melissa A. Benson",
    // A 3/1 regenerator for one mana. The sacrifice is what makes it a
    // real cost rather than a free clock.
    CardRules::new_creature(mana_cost!("{B}"), &["Skeleton"], 3, 1).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, sacrifice a creature.",
            // Not "another creature", so with nothing else out it eats
            // itself, which is the drawback the body is priced on.
            EffectDef::sacrifice(EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            )),
        ),
        abilities::regenerate_self(
            "{B}: Regenerate this creature.",
            &[CostDef::Mana(mana_cost!("{B}"))],
        ),
    ]),
);

// ICE 138 — Knight of Stromgald
pub(in crate::card::sets) static KNIGHT_OF_STROMGALD: CardRecord = CardRecord::new(
    "Knight of Stromgald",
    "2b87069b-ebaf-4705-b5da-446932af9b73",
    "Mark Poole",
    CardRules::new_creature(mana_cost!("{B}{B}"), &["Human", "Knight"], 2, 1).with_abilities(&[
        abilities::protection_from_color(ManaColor::White),
        abilities::apply_to_self_until_end_of_turn(
            "{B}: This creature gains first strike until end of turn.",
            &[CostDef::Mana(mana_cost!("{B}"))],
            AppliedEffectDef::add_ability(&abilities::first_strike()),
        ),
        AbilityDef::activated(
            "{B}{B}: This creature gets +1/+0 until end of turn.",
            &[CostDef::Mana(mana_cost!("{B}{B}"))],
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

// ICE 139 — Krovikan Elementalist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KROVIKAN_ELEMENTALIST: CardRecord = CardRecord::new(
    "Krovikan Elementalist",
    "bbedca18-a074-4441-b0a9-7b14fdb07412",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// ICE 140 — Krovikan Fetish
pub(in crate::card::sets) static KROVIKAN_FETISH: CardRecord = CardRecord::new(
    "Krovikan Fetish",
    "844e73e6-b201-4b2e-b46a-b719484fba0e",
    "Heather Hudson",
    CardRules::new_enchantment(mana_cost!("{2}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            abilities::enters_trigger(
                "When this Aura enters, draw a card at the beginning of the next turn's upkeep.",
                DRAW_AT_NEXT_UPKEEP,
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +1/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                },
            ),
        ]),
);

// ICE 141 — Krovikan Vampire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KROVIKAN_VAMPIRE: CardRecord = CardRecord::new(
    "Krovikan Vampire",
    "717c5dda-8e38-4c76-b241-685198402284",
    "Quinton Hoover",
    crate::card::CardRules::unsupported(),
);

// ICE 142 — Legions of Lim-Dûl
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LEGIONS_OF_LIM_DUL: CardRecord = CardRecord::new(
    "Legions of Lim-Dûl",
    "75b67eb2-b60e-46b4-9d48-11c284957bec",
    "Anson Maddocks",
    crate::card::CardRules::unsupported(),
);

// ICE 143 — Leshrac's Rite
pub(in crate::card::sets) static LESHRAC_S_RITE: CardRecord = CardRecord::new(
    "Leshrac's Rite",
    "4e0a6b4e-95b4-40f6-bb19-568dbd908a2b",
    "Richard Thomas",
    CardRules::new_enchantment(mana_cost!("{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature has swampwalk.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::landwalk(
                        BasicLandType::Swamp,
                    )),
                },
            ),
        ]),
);

// ICE 144 — Leshrac's Sigil
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LESHRAC_S_SIGIL: CardRecord = CardRecord::new(
    "Leshrac's Sigil",
    "ad5ba7ee-d6df-4b62-a8a1-c81e6fca392a",
    "Drew Tucker",
    crate::card::CardRules::unsupported(),
);

// ICE 145 — Lim-Dûl's Cohort
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIM_DUL_S_COHORT: CardRecord = CardRecord::new(
    "Lim-Dûl's Cohort",
    "3d0006f6-2f96-453d-9145-eaefa588efbc",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// ICE 146 — Lim-Dûl's Hex
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIM_DUL_S_HEX: CardRecord = CardRecord::new(
    "Lim-Dûl's Hex",
    "af976f42-3d56-4e32-8294-970a276a4bf3",
    "Liz Danforth",
    crate::card::CardRules::unsupported(),
);

// ICE 147 — Mind Ravel
pub(in crate::card::sets) static MIND_RAVEL: CardRecord = CardRecord::new(
    "Mind Ravel",
    "61cf3ac5-985d-4b48-b230-d5ae4ab1ace8",
    "Mark Tedin",
    CardRules::new_sorcery(mana_cost!("{2}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target player discards a card.\nDraw a card at the beginning of the next turn's upkeep.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Sequence(&[
            EffectDef::Discard {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
                selection: crate::card::DiscardSelectionDef::RecipientChooses,
                then: None,
            },
            DRAW_AT_NEXT_UPKEEP,
        ]),
    )),
);

// ICE 148 — Mind Warp
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIND_WARP: CardRecord = CardRecord::new(
    "Mind Warp",
    "de150cd6-0bbc-47f7-a781-cd1aa10eabc6",
    "Liz Danforth",
    crate::card::CardRules::unsupported(),
);

// ICE 149 — Mind Whip
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIND_WHIP: CardRecord = CardRecord::new(
    "Mind Whip",
    "3f3ff5fb-4126-4a18-b540-2beaae382e59",
    "Drew Tucker",
    crate::card::CardRules::unsupported(),
);

// ICE 150 — Minion of Leshrac
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MINION_OF_LESHRAC: CardRecord = CardRecord::new(
    "Minion of Leshrac",
    "61278908-a1b4-4b4c-84f5-498ca41fc6b6",
    "Allen Williams",
    crate::card::CardRules::unsupported(),
);

// ICE 151 — Minion of Tevesh Szat
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MINION_OF_TEVESH_SZAT: CardRecord = CardRecord::new(
    "Minion of Tevesh Szat",
    "ea9f3ab5-6a31-47db-b8bf-4c56a7ff19d1",
    "Julie Baroh",
    crate::card::CardRules::unsupported(),
);

// ICE 152 — Mole Worms
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOLE_WORMS: CardRecord = CardRecord::new(
    "Mole Worms",
    "4914f6fc-e3e7-426b-8688-12157c7df9e7",
    "Daniel Gelon",
    crate::card::CardRules::unsupported(),
);

// ICE 153 — Moor Fiend
pub(in crate::card::sets) static MOOR_FIEND: CardRecord = CardRecord::new(
    "Moor Fiend",
    "57089dd4-e30d-498d-9341-43c104c6f3f9",
    "Anson Maddocks",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Horror"], 3, 3)
        .with_ability(abilities::landwalk(BasicLandType::Swamp)),
);

// ICE 154 — Necropotence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NECROPOTENCE: CardRecord = CardRecord::new(
    "Necropotence",
    "54d7a0c1-efb4-4a8d-ad92-a96d43835052",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// ICE 155 — Norritt
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NORRITT: CardRecord = CardRecord::new(
    "Norritt",
    "35abefe6-c39b-4fe5-b2e3-d213f0c4f447",
    "Mike Raabe",
    crate::card::CardRules::unsupported(),
);

// ICE 156 — Oath of Lim-Dûl
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OATH_OF_LIM_DUL: CardRecord = CardRecord::new(
    "Oath of Lim-Dûl",
    "f16df768-06de-43a0-b548-44fb0887490b",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// ICE 157 — Pestilence Rats
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PESTILENCE_RATS: CardRecord = CardRecord::new(
    "Pestilence Rats",
    "bff7f6a6-0e90-4eb4-b76e-d98454975fb6",
    "Jeff A. Menges",
    crate::card::CardRules::unsupported(),
);

// ICE 158 — Pox
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static POX: CardRecord = CardRecord::new(
    "Pox",
    "a914138c-a593-414c-bbcb-83d3c1bc4f6f",
    "Cornelius Brudi",
    crate::card::CardRules::unsupported(),
);

// ICE 159 — Seizures
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEIZURES: CardRecord = CardRecord::new(
    "Seizures",
    "da369c86-7e17-43d8-b626-b6842e3d2d50",
    "Julie Baroh",
    crate::card::CardRules::unsupported(),
);

// ICE 160 — Songs of the Damned
pub(in crate::card::sets) static SONGS_OF_THE_DAMNED: CardRecord = CardRecord::new(
    "Songs of the Damned",
    "6cff3547-8c72-439a-91fe-ebe729dab748",
    "Pete Venters",
    // A ritual whose size is the graveyard, so it does nothing early and
    // everything after a sweeper.
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell(
        "Add {B} for each creature card in your graveyard.",
        EffectDef::AddMana(
            AddManaEffectDef::one(ManaColor::Black).with_variable_amount(
                ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Graveyard],
                    PlayerRelation::You,
                )),
            ),
        ),
    )),
);

// ICE 161 — Soul Burn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOUL_BURN: CardRecord = CardRecord::new(
    "Soul Burn",
    "eb8e00d2-2381-4d45-bed8-c9bf738a9419",
    "Rob Alexander",
    crate::card::CardRules::unsupported(),
);

// ICE 162 — Soul Kiss
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOUL_KISS: CardRecord = CardRecord::new(
    "Soul Kiss",
    "42fbf6a5-86fe-41a3-891e-f72f11ad0aee",
    "Nicola Leonard",
    crate::card::CardRules::unsupported(),
);

// ICE 163 — Spoils of Evil
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPOILS_OF_EVIL: CardRecord = CardRecord::new(
    "Spoils of Evil",
    "fd368eb6-72f0-42d4-afa5-3daa7de949ff",
    "Quinton Hoover",
    crate::card::CardRules::unsupported(),
);

// ICE 164 — Spoils of War
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPOILS_OF_WAR: CardRecord = CardRecord::new(
    "Spoils of War",
    "b38af8bd-d927-46d0-a1b1-fb437ea9ea66",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// ICE 165 — Stench of Evil
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STENCH_OF_EVIL: CardRecord = CardRecord::new(
    "Stench of Evil",
    "4c7065a2-f819-4cbe-b453-a55e904f0461",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// ICE 166 — Stromgald Cabal
pub(in crate::card::sets) static STROMGALD_CABAL: CardRecord = CardRecord::new(
    "Stromgald Cabal",
    "6ac6fa0c-753e-4fbc-8a70-0f956503cf4e",
    "Anson Maddocks",
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Human", "Knight"], 2, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{T}, Pay 1 life: Counter target white spell.",
            &[CostDef::TapSource, CostDef::PayLife(1)],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Spell,
                        ObjectPredicateDef::Color(ManaColor::White),
                    ]),
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::counter_target(TargetIndex::PRIMARY),
        ),
    ),
);

// ICE 167 — Touch of Death
pub(in crate::card::sets) static TOUCH_OF_DEATH: CardRecord = CardRecord::new(
    "Touch of Death",
    "a49c658f-e657-490b-af1f-e67e48d0046e",
    "Melissa A. Benson",
CardRules::new_sorcery(mana_cost!("{2}{B}")).with_ability(AbilityDef::spell_with_targets(
        "This spell deals 1 damage to target player or planeswalker. You gain 1 life.\nDraw a card at the beginning of the next turn's upkeep.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
        )],
        EffectDef::Sequence(&[
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(1),
            ),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
            DRAW_AT_NEXT_UPKEEP,
        ]),
    )),
);

// ICE 168 — Withering Wisps
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WITHERING_WISPS: CardRecord = CardRecord::new(
    "Withering Wisps",
    "ad1e6ae5-c972-42c0-ae78-f203873aeeb1",
    "NéNé Thomas",
    crate::card::CardRules::unsupported(),
);

// ICE 169 — Aggression
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AGGRESSION: CardRecord = CardRecord::new(
    "Aggression",
    "f3f26060-0c24-496c-b8e2-4dac7ea6166b",
    "Rick Emond",
    crate::card::CardRules::unsupported(),
);

// ICE 170 — Anarchy
pub(in crate::card::sets) static ANARCHY: CardRecord = CardRecord::new(
    "Anarchy",
    "28d941da-b5cb-4b7e-84f2-ece883f89af3",
    "Phil Foglio",
    CardRules::new_sorcery(mana_cost!("{2}{R}{R}")).with_ability(AbilityDef::spell(
        "Destroy all white permanents.",
        EffectDef::Destroy {
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::Color(ManaColor::White),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            then: None,
        },
    )),
);

// ICE 171 — Avalanche
pub(in crate::card::sets) static AVALANCHE: CardRecord = CardRecord::new(
    "Avalanche",
    "d3a925e5-0d0a-42ec-b1c6-9793b8e11625",
    "Brian Snõddy",
    // Four mana plus one a land, aimed only at snow lands: a sideboard card
    // that reads as blank against half the field.
    CardRules::new_sorcery(mana_cost!("{X}{2}{R}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "Destroy X target snow lands.",
            &[AbilityTargetDef::exactly_chosen_x(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Supertype(CardSupertype::Snow),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
    ),
);

// ICE 172 — Balduvian Barbarians
pub(in crate::card::sets) static BALDUVIAN_BARBARIANS: CardRecord = CardRecord::new(
    "Balduvian Barbarians",
    "efeabe8e-8107-4d19-8a43-362aa79cdd92",
    "Mark Poole",
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Human", "Barbarian"], 3, 2),
);

// ICE 173 — Balduvian Hydra
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BALDUVIAN_HYDRA: CardRecord = CardRecord::new(
    "Balduvian Hydra",
    "c3a3b37f-daa6-4502-bb12-c72afe3df035",
    "Melissa A. Benson",
    crate::card::CardRules::unsupported(),
);

// ICE 174 — Barbarian Guides
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARBARIAN_GUIDES: CardRecord = CardRecord::new(
    "Barbarian Guides",
    "fe65a045-dacb-4392-bcb6-843394ef98c9",
    "Richard Thomas",
    crate::card::CardRules::unsupported(),
);

// ICE 175 — Battle Frenzy
pub(in crate::card::sets) static BATTLE_FRENZY: CardRecord = CardRecord::new(
    "Battle Frenzy",
    "a85ae675-56ca-4a00-83d2-ee035f33d6d1",
    "Brian Snõddy",
CardRules::new_instant(mana_cost!("{2}{R}")).with_ability(AbilityDef::spell(
        "Green creatures you control get +1/+1 until end of turn.\nNongreen creatures you control get +1/+0 until end of turn.",
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Color(ManaColor::Green),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Color(ManaColor::Green)),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )),
);

// ICE 176 — Bone Shaman
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BONE_SHAMAN: CardRecord = CardRecord::new(
    "Bone Shaman",
    "0a5e3d54-4dc4-482b-8ecc-bb819ba03d2c",
    "Anson Maddocks",
    crate::card::CardRules::unsupported(),
);

// ICE 177 — Brand of Ill Omen
pub(in crate::card::sets) static BRAND_OF_ILL_OMEN: CardRecord = CardRecord::new(
    "Brand of Ill Omen",
    "ceeb7bbc-2d41-4709-95be-1ceb952ed1fb",
    "Rob Alexander",
    // Audit: unsupported — "enchanted creature's controller can't cast
    // creature spells" needs a static play restriction aimed at the host's
    // controller, and `static_player_relation_supported` admits no relation
    // naming the attached permanent's controller, only the enchanted player
    // of a player-Aura. Everything else on the card is expressible.
    CardRules::unsupported(),
);

// ICE 178 — Chaos Lord
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHAOS_LORD: CardRecord = CardRecord::new(
    "Chaos Lord",
    "ee245922-b380-4b2e-a43f-ab1ba8078943",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// ICE 179 — Chaos Moon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHAOS_MOON: CardRecord = CardRecord::new(
    "Chaos Moon",
    "aae0543f-7f8b-4327-b735-ac21244e9936",
    "Drew Tucker",
    crate::card::CardRules::unsupported(),
);

// ICE 180 — Conquer
pub(in crate::card::sets) static CONQUER: CardRecord = CardRecord::new(
    "Conquer",
    "ae610e66-7bcb-40ec-bed5-86dcfd098654",
    "Randy Gallegos",
    CardRules::new_enchantment(mana_cost!("{3}{R}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_land(),
            AbilityDef::static_ability(
                "You control enchanted land.",
                EffectDef::gain_control(
                    EffectRecipientDef::AttachedPermanent,
                    PlayerRefDef::EffectController,
                    ControlDurationDef::WhileSourceRemains {
                        while_tapped: false,
                    },
                ),
            ),
        ]),
);

// ICE 181 — Curse of Marit Lage
pub(in crate::card::sets) static CURSE_OF_MARIT_LAGE: CardRecord = CardRecord::new(
    "Curse of Marit Lage",
    "69b381c1-aa71-4d40-a320-70f58a440d51",
    "Amy Weber",
    CardRules::new_enchantment(mana_cost!("{3}{R}{R}")).with_abilities(&[
        abilities::enters_trigger(
            "When this enchantment enters, tap all Islands.",
            EffectDef::Tap {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
            },
        ),
        AbilityDef::static_ability(
            "Islands don't untap during their controllers' untap steps.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
            },
        ),
    ]),
);

// ICE 182 — Dwarven Armory
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DWARVEN_ARMORY: CardRecord = CardRecord::new(
    "Dwarven Armory",
    "7d14a430-6e08-40cf-970a-cae84bba6ef7",
    "Richard Thomas",
    crate::card::CardRules::unsupported(),
);

// ICE 183 — Errantry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ERRANTRY: CardRecord = CardRecord::new(
    "Errantry",
    "8346e741-61f8-4283-be51-f5f80e9595a5",
    "Allen Williams",
    crate::card::CardRules::unsupported(),
);

// ICE 184 — Flame Spirit
pub(in crate::card::sets) static FLAME_SPIRIT: CardRecord = CardRecord::new(
    "Flame Spirit",
    "add2b82a-9aa5-4d5c-a1c2-e313541f12c8",
    "Justin Hampton",
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Elemental", "Spirit"], 2, 3).with_ability(
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
    ),
);

// ICE 185 — Flare
pub(in crate::card::sets) static FLARE: CardRecord = CardRecord::new(
    "Flare",
    "d5350236-7bd2-462d-9768-50087626c764",
    "Drew Tucker",
CardRules::new_instant(mana_cost!("{2}{R}")).with_ability(AbilityDef::spell_with_targets(
        "This spell deals 1 damage to any target.\nDraw a card at the beginning of the next turn's upkeep.",
        &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget)],
        EffectDef::Sequence(&[
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(1),
            ),
            DRAW_AT_NEXT_UPKEEP,
        ]),
    )),
);

// ICE 186 — Game of Chaos
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GAME_OF_CHAOS: CardRecord = CardRecord::new(
    "Game of Chaos",
    "08265332-2c0e-4c42-8c51-83ac20462eed",
    "Drew Tucker",
    crate::card::CardRules::unsupported(),
);

// ICE 187 — Glacial Crevasses
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLACIAL_CREVASSES: CardRecord = CardRecord::new(
    "Glacial Crevasses",
    "2726b192-f239-470b-8ad6-69887405e7f9",
    "Mike Raabe",
    crate::card::CardRules::unsupported(),
);

// ICE 188 — Goblin Mutant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_MUTANT: CardRecord = CardRecord::new(
    "Goblin Mutant",
    "6db54f95-6652-45a3-b960-c2fc118beca1",
    "Daniel Gelon",
    crate::card::CardRules::unsupported(),
);

// ICE 189 — Goblin Sappers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_SAPPERS: CardRecord = CardRecord::new(
    "Goblin Sappers",
    "de839540-a7b9-4f91-91df-3fd4f5c0bc4e",
    "Jeff A. Menges",
    crate::card::CardRules::unsupported(),
);

// ICE 190 — Goblin Ski Patrol
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_SKI_PATROL: CardRecord = CardRecord::new(
    "Goblin Ski Patrol",
    "fde1c8b5-1e01-4920-8d02-bf80d5b238c5",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// ICE 191 — Goblin Snowman
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_SNOWMAN: CardRecord = CardRecord::new(
    "Goblin Snowman",
    "5bbb260a-6763-4d1c-a009-4e34cd572519",
    "Daniel Gelon",
    crate::card::CardRules::unsupported(),
);

// ICE 192 — Grizzled Wolverine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRIZZLED_WOLVERINE: CardRecord = CardRecord::new(
    "Grizzled Wolverine",
    "95bb17b9-55c4-4cc1-83f6-75490b9a97d0",
    "Cornelius Brudi",
    crate::card::CardRules::unsupported(),
);

// ICE 193 — Imposing Visage
pub(in crate::card::sets) static IMPOSING_VISAGE: CardRecord = CardRecord::new(
    "Imposing Visage",
    "cca42b74-9b42-482b-b12a-79cafdcd087e",
    "Phil Foglio",
    CardRules::new_enchantment(mana_cost!("{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature has menace.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::menace()),
                },
            ),
        ]),
);

// ICE 194 — Incinerate
pub(in crate::card::sets) static INCINERATE: CardRecord = CardRecord::new(
    "Incinerate",
    "9c3f00af-010d-4485-b8b7-47400d99c496",
    "Mark Poole",
CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "Incinerate deals 3 damage to any target. A creature dealt damage this way can't be regenerated this turn.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage(
                crate::card::DamageDef::new(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(3),
                )
                .with_follow_up(crate::card::DamageFollowUpDef::ApplyToDamaged {
                    effect: &AppliedEffectDef::Rule(AppliedRuleDef::CannotRegenerate),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                }),
            ),
        ),
    ),
);

// ICE 195 — Jokulhaups
pub(in crate::card::sets) static JOKULHAUPS: CardRecord = CardRecord::new(
    "Jokulhaups",
    "3bf0d325-5928-4593-8faa-64ffa414cb48",
    "Richard Thomas",
    // Six mana to end the game as a board state. Everything but the
    // enchantments and the players goes.
    CardRules::new_sorcery(mana_cost!("{4}{R}{R}")).with_ability(AbilityDef::spell(
        "Destroy all artifacts, creatures, and lands. They can't be regenerated.",
        EffectDef::WithRule {
            rule: AppliedRuleDef::CannotRegenerate,
            effect: &const {
                EffectDef::Destroy {
                    object: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Land),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    then: None,
                }
            },
        },
    )),
);

// ICE 196 — Karplusan Giant
pub(in crate::card::sets) static KARPLUSAN_GIANT: CardRecord = CardRecord::new(
    "Karplusan Giant",
    "c524ac2a-294c-4b19-b00b-999e370a3b95",
    "Daniel Gelon",
    // Seven mana for a 3/3 that grows for free, which only reads as a deal
    // in a deck whose lands are all snow anyway.
    CardRules::new_creature(mana_cost!("{6}{R}"), &["Giant"], 3, 3).with_ability(
        AbilityDef::activated(
            "Tap an untapped snow land you control: This creature gets +1/+1 until end of turn.",
            &[CostDef::TapPermanents {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::Supertype(CardSupertype::Snow),
                ]),
                controller: PlayerRelation::You,
                count: 1,
            }],
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

// ICE 197 — Karplusan Yeti
pub(in crate::card::sets) static KARPLUSAN_YETI: CardRecord = CardRecord::new(
    "Karplusan Yeti",
    "7dd9b214-d9fe-4c2e-b45b-7145ad98c408",
    "Quinton Hoover",
CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Yeti"], 3, 3).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: This creature deals damage equal to its power to target creature. That creature deals damage equal to its power to this creature.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Fight {
                first: ObjectRefDef::Source,
                second: ObjectRefDef::Target(TargetIndex::PRIMARY),
                excess: None,
            },
        ),
    ),
);

// ICE 198 — Lava Burst
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAVA_BURST: CardRecord = CardRecord::new(
    "Lava Burst",
    "79dc0e20-5790-4927-8432-cf0e9b7381d4",
    "Tom Wänerstrand",
    crate::card::CardRules::unsupported(),
);

// ICE 199 — Márton Stromgald
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MARTON_STROMGALD: CardRecord = CardRecord::new(
    "Márton Stromgald",
    "7880e815-53e7-43e0-befd-e368f00a75d8",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// ICE 200 — Melee
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MELEE: CardRecord = CardRecord::new(
    "Melee",
    "b13a064d-bff4-4a48-a158-1b61951b0ac3",
    "Dameon Willich",
    crate::card::CardRules::unsupported(),
);

// ICE 201 — Melting
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MELTING: CardRecord = CardRecord::new(
    "Melting",
    "8d90065e-2c7e-44e5-9f59-015d468214bf",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// ICE 202 — Meteor Shower
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static METEOR_SHOWER: CardRecord = CardRecord::new(
    "Meteor Shower",
    "50b4851e-677b-468e-9baa-e47a3b4b8339",
    "Rick Emond",
    crate::card::CardRules::unsupported(),
);

// ICE 203 — Mountain Goat
pub(in crate::card::sets) static MOUNTAIN_GOAT: CardRecord = CardRecord::new(
    "Mountain Goat",
    "ccf70276-a40c-4d25-b584-4c8a07a00602",
    "Cornelius Brudi",
    CardRules::new_creature(mana_cost!("{R}"), &["Goat"], 1, 1)
        .with_ability(abilities::landwalk(BasicLandType::Mountain)),
);

// ICE 204 — Mudslide
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MUDSLIDE: CardRecord = CardRecord::new(
    "Mudslide",
    "65acce56-8674-471e-9d5e-91b7e3f672c1",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// ICE 205 — Orcish Cannoneers
pub(in crate::card::sets) static ORCISH_CANNONEERS: CardRecord = CardRecord::new(
    "Orcish Cannoneers",
    "a4309a2f-27f5-4652-b0b4-6a6119436f75",
    "Dan Frazier",
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Orc", "Warrior"], 1, 3).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: This creature deals 2 damage to any target and 3 damage to you.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::Sequence(&[
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(2),
                ),
                EffectDef::damage(EffectRecipientDef::Controller, ValueDef::Constant(3)),
            ]),
        ),
    ),
);

// ICE 206 — Orcish Conscripts
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORCISH_CONSCRIPTS: CardRecord = CardRecord::new(
    "Orcish Conscripts",
    "e71394f8-3038-4cad-adea-a704f004777f",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// ICE 207 — Orcish Farmer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORCISH_FARMER: CardRecord = CardRecord::new(
    "Orcish Farmer",
    "efa5beef-d609-4809-a813-621b0b4cff7f",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// ICE 208 — Orcish Healer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORCISH_HEALER: CardRecord = CardRecord::new(
    "Orcish Healer",
    "7ff511f3-416e-4919-acd6-fd8183bf5c60",
    "Quinton Hoover",
    crate::card::CardRules::unsupported(),
);

// ICE 209 — Orcish Librarian
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORCISH_LIBRARIAN: CardRecord = CardRecord::new(
    "Orcish Librarian",
    "8ed908d6-6d06-4ccb-9577-37ef2d01c1a5",
    "Phil Foglio",
    crate::card::CardRules::unsupported(),
);

// ICE 210 — Orcish Lumberjack
pub(in crate::card::sets) static ORCISH_LUMBERJACK: CardRecord = CardRecord::new(
    "Orcish Lumberjack",
    "21ef13e3-658c-43a3-a290-4c5dde8e8b55",
    "Dan Frazier",
    // One mana for a 1/1 that turns a land into three mana of either colour:
    // the land is gone and the body is nothing, and the deck playing it only
    // needs the turn it buys.
    CardRules::new_creature(mana_cost!("{R}"), &["Orc"], 1, 1).with_ability(
        AbilityDef::activated_mana(
            "{T}, Sacrifice a Forest: Add three mana in any combination of {R} and/or {G}.",
            // "Sacrifice a Forest" reads the land type rather than the card name, so a
            // dual land with the type counts and a Forest somebody enchanted still
            // does. Which one is spent is chosen as the ability is activated.
            &[
                CostDef::TapSource,
                CostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
                    controller: PlayerRelation::You,
                },
            ],
            EffectDef::AddMana(AddManaEffectDef::combination(
                &[ManaColor::Red, ManaColor::Green],
                3,
            )),
        ),
    ),
);

// ICE 211 — Orcish Squatters
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORCISH_SQUATTERS: CardRecord = CardRecord::new(
    "Orcish Squatters",
    "f3ee7bd5-612b-4916-a914-1294805b8f64",
    "Richard Kane Ferguson",
    crate::card::CardRules::unsupported(),
);

// ICE 212 — Panic
pub(in crate::card::sets) static PANIC: CardRecord = CardRecord::new(
    "Panic",
    "a9ab85ac-311c-4e36-943a-817e43a3c8a8",
    "Mike Kimble",
CardRules::new_instant(mana_cost!("{R}"))
        .cast_only_before_blockers_declared()
        .with_ability(AbilityDef::spell_with_targets(
            "Target creature can't block this turn.\nDraw a card at the beginning of the next turn's upkeep.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                DRAW_AT_NEXT_UPKEEP,
            ]),
        )),
);

// ICE 213 — Pyroblast
pub(in crate::card::sets) static PYROBLAST: CardRecord = CardRecord::new(
    "Pyroblast",
    "c342cac5-08ae-4428-9c2c-f6c5904e54d2",
    "Kaja Foglio",
    CardRules::new_instant(mana_cost!("{R}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::counter_target(
                "Counter target spell if it's blue.",
                &AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Spell,
                        ObjectPredicateDef::Color(ManaColor::Blue),
                    ]),
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                }),
            ),
            AbilityDef::destroy_target(
                "Destroy target permanent if it's blue.",
                &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Color(
                    ManaColor::Blue,
                )),
            ),
        ],
    )),
);

// ICE 214 — Pyroclasm
pub(in crate::card::sets) static PYROCLASM: CardRecord = CardRecord::new(
    "Pyroclasm",
    "88040748-ad76-4b9a-bd4e-87e5980e9816",
    "Pat Lewis",
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_ability(AbilityDef::spell(
        "This spell deals 2 damage to each creature.",
        EffectDef::damage(
            EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            ValueDef::Constant(2),
        ),
    )),
);

// ICE 215 — Sabretooth Tiger
pub(in crate::card::sets) static SABRETOOTH_TIGER: CardRecord = CardRecord::new(
    "Sabretooth Tiger",
    "6914c5a8-2114-41c5-a471-ca97524d622f",
    "Melissa A. Benson",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Cat"], 2, 1)
        .with_ability(abilities::first_strike()),
);

// ICE 216 — Shatter (reprint)
const SHATTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SHATTER,
    "7eb18d53-20de-43d7-86f7-97a6d14d54b8",
    "Bryon Wackwitz",
);

// ICE 217 — Stone Rain (reprint)
const STONE_RAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::STONE_RAIN,
    "5a002e6d-ea59-4694-b3e5-075d6020b0d9",
    "Kaja Foglio",
);

// ICE 218 — Stone Spirit
pub(in crate::card::sets) static STONE_SPIRIT: CardRecord = CardRecord::new(
    "Stone Spirit",
    "789dfae7-fe23-4e2e-9f5f-304535d22a78",
    "Jeff A. Menges",
    // A ground creature that fliers cannot stop, which in a format full of
    // them is nearly unblockable.
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Elemental", "Spirit"], 4, 3).with_ability(
        AbilityDef::static_ability(
            "This creature can't be blocked by creatures with flying.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::BlockRestriction(
                    BlockRestrictionDef::prohibit(
                        BlockRestrictionSubjectDef::Attacker,
                        BlockRestrictionMatchDef::Matching(ObjectPredicateDef::HasKeyword(
                            KeywordAbility::Flying,
                        )),
                    ),
                )),
            },
        ),
    ),
);

// ICE 219 — Stonehands
pub(in crate::card::sets) static STONEHANDS: CardRecord = CardRecord::new(
    "Stonehands",
    "d23fa1af-78e5-4d23-bbf6-cd62bc54b4e9",
    "Dan Frazier",
    CardRules::new_enchantment(mana_cost!("{2}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +0/+2.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(0),
                        ValueDef::Constant(2),
                    ),
                },
            ),
            AbilityDef::activated(
                "{R}: Enchanted creature gets +1/+0 until end of turn.",
                &[CostDef::Mana(mana_cost!("{R}"))],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// ICE 220 — Tor Giant
pub(in crate::card::sets) static TOR_GIANT: CardRecord = CardRecord::new(
    "Tor Giant",
    "7ef8f279-1a10-4685-99d6-bc971a7f922b",
    "Douglas Shuler",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Giant"], 3, 3),
);

// ICE 221 — Total War
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOTAL_WAR: CardRecord = CardRecord::new(
    "Total War",
    "6107388b-ec1e-401e-a407-a821c908ed8d",
    "Drew Tucker",
    crate::card::CardRules::unsupported(),
);

// ICE 222 — Vertigo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VERTIGO: CardRecord = CardRecord::new(
    "Vertigo",
    "3067e7af-7bbd-48c1-9f1d-df2a91a0ec54",
    "Drew Tucker",
    crate::card::CardRules::unsupported(),
);

// ICE 223 — Wall of Lava
pub(in crate::card::sets) static WALL_OF_LAVA: CardRecord = CardRecord::new(
    "Wall of Lava",
    "b99d6d11-b3f7-4d73-967c-3049af82a9d8",
    "Pete Venters",
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Wall"], 1, 3).with_abilities(&[
        abilities::defender(),
        AbilityDef::activated(
            "{R}: This creature gets +1/+1 until end of turn.",
            &[CostDef::Mana(mana_cost!("{R}"))],
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

// ICE 224 — Word of Blasting
pub(in crate::card::sets) static WORD_OF_BLASTING: CardRecord = CardRecord::new(
    "Word of Blasting",
    "46b383c8-d604-4131-a869-9e9d13e30b94",
    "Ken Meyer, Jr.",
    // Two mana that kills a Wall and burns for what it cost, which in a
    // format full of defensive four-drops was a real tempo swing.
    CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target Wall. It can't be regenerated. Word of Blasting deals damage equal to \
         that Wall's mana value to the Wall's controller.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Wall")),
        )],
        EffectDef::Sequence(&[
            // Applied before the destruction so a shield already on the Wall
            // cannot replace it.
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotRegenerate),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
            // The damage is its own sentence, so it happens whether or not
            // the Wall actually died; the mana value is read from last-known
            // information either way.
            EffectDef::damage(
                EffectRecipientDef::player(PlayerRefDef::ControllerOf(ObjectRefDef::Target(
                    TargetIndex::PRIMARY,
                ))),
                ValueDef::TargetManaValue(TargetIndex::PRIMARY),
            ),
        ]),
    )),
);

// ICE 225 — Aurochs
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AUROCHS: CardRecord = CardRecord::new(
    "Aurochs",
    "7e973a84-7f7d-4524-9f2f-ec9a014d52ee",
    "Ken Meyer, Jr.",
    crate::card::CardRules::unsupported(),
);

// ICE 226 — Balduvian Bears
pub(in crate::card::sets) static BALDUVIAN_BEARS: CardRecord = CardRecord::new(
    "Balduvian Bears",
    "ef5297cb-e763-4871-9cd3-0e2dbcc52095",
    "Quinton Hoover",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Bear"], 2, 2),
);

// ICE 227 — Blizzard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLIZZARD: CardRecord = CardRecord::new(
    "Blizzard",
    "c369e4f9-0f2b-446c-9e2d-d3eefab0586d",
    "Anson Maddocks",
    crate::card::CardRules::unsupported(),
);

// ICE 228 — Brown Ouphe
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BROWN_OUPHE: CardRecord = CardRecord::new(
    "Brown Ouphe",
    "e26ce35b-ba65-451d-a5ed-e1db6f1d0c6f",
    "Daniel Gelon",
    crate::card::CardRules::unsupported(),
);

// ICE 229 — Chub Toad
pub(in crate::card::sets) static CHUB_TOAD: CardRecord = CardRecord::new(
    "Chub Toad",
    "b6ebcc1d-0c5c-4bc2-ade7-41944f69162e",
    "Daniel Gelon",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Frog"], 1, 1).with_ability(
        AbilityDef::triggered(
            "Whenever this creature blocks or becomes blocked, it gets +2/+2 until end of turn.",
            TriggerEventDef::BlocksOrBecomesBlockedBy {
                creature: ObjectPredicateDef::Source,
                other: ObjectPredicateDef::Any,
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ICE 230 — Dire Wolves
pub(in crate::card::sets) static DIRE_WOLVES: CardRecord = CardRecord::new(
    "Dire Wolves",
    "a602c93d-e00f-4b4f-a7ff-95316b7e7641",
    "Ron Spencer",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Wolf"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "This creature has banding as long as you control a Plains.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::controls_basic_land_type(
                    PlayerRelation::You,
                    BasicLandType::Plains,
                ),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::banding()),
                },
            },
        ),
    ),
);

// ICE 231 — Earthlore
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EARTHLORE: CardRecord = CardRecord::new(
    "Earthlore",
    "319d252e-7c43-47d6-8873-f69b0e063256",
    "Drew Tucker",
    crate::card::CardRules::unsupported(),
);

// ICE 232 — Elder Druid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELDER_DRUID: CardRecord = CardRecord::new(
    "Elder Druid",
    "210f6fab-62f0-42ab-bd01-00d647bd25e7",
    "Richard Kane Ferguson",
    crate::card::CardRules::unsupported(),
);

// ICE 233 — Essence Filter
pub(in crate::card::sets) static ESSENCE_FILTER: CardRecord = CardRecord::new(
    "Essence Filter",
    "9b610103-dafd-4248-9d79-ce57f84b9e03",
    "Rick Emond",
    CardRules::new_sorcery(mana_cost!("{1}{G}{G}")).with_ability(AbilityDef::spell(
        "Destroy all enchantments or all nonwhite enchantments.",
        EffectDef::ChooseEffect {
            player: EffectRecipientDef::Controller,
            choices: &[
                EffectChoiceDef {
                    label: "Destroy all enchantments",
                    effect: EffectDef::Destroy {
                        object: EffectRecipientDef::matching_objects(
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Any,
                        ),
                        then: None,
                    },
                },
                EffectChoiceDef {
                    label: "Destroy all nonwhite enchantments",
                    effect: EffectDef::Destroy {
                        object: EffectRecipientDef::matching_objects(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Enchantment),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Color(
                                    ManaColor::White,
                                )),
                            ]),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Any,
                        ),
                        then: None,
                    },
                },
            ],
        },
    )),
);

// ICE 234 — Fanatical Fever
pub(in crate::card::sets) static FANATICAL_FEVER: CardRecord = CardRecord::new(
    "Fanatical Fever",
    "2abba7f1-5d07-4137-88a2-5967396a3e42",
    "Julie Baroh",
    CardRules::new_instant(mana_cost!("{2}{G}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +3/+0 and gains trample until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(3),
                    ValueDef::Constant(0),
                ),
                AppliedEffectDef::add_ability(&abilities::trample()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// ICE 235 — Folk of the Pines
pub(in crate::card::sets) static FOLK_OF_THE_PINES: CardRecord = CardRecord::new(
    "Folk of the Pines",
    "0c13311d-db83-483f-ba2b-4f54ceb8b026",
    "NéNé Thomas & Catherine Buck",
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Dryad"], 2, 5).with_ability(
        AbilityDef::activated(
            "{1}{G}: This creature gets +1/+0 until end of turn.",
            &[CostDef::Mana(mana_cost!("{1}{G}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ICE 236 — Forbidden Lore
pub(in crate::card::sets) static FORBIDDEN_LORE: CardRecord = CardRecord::new(
    "Forbidden Lore",
    "5fc225cf-4fe2-4a5b-828e-ffcb99e404e8",
    "Christopher Rush",
    // A combat trick every turn out of a land, which costs the deck a card
    // once rather than every time.
    CardRules::new_enchantment(mana_cost!("{2}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_land(),
            AbilityDef::static_ability(
                "Enchanted land has \"{T}: Target creature gets +2/+1 until end of turn.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(
                        &const {
                            AbilityDef::activated_with_targets(
                                "{T}: Target creature gets +2/+1 until end of turn.",
                                &[CostDef::TapSource],
                                &const {
                                    [AbilityTargetDef::exactly_one_permanent(
                                        ObjectPredicateDef::HasType(CardType::Creature),
                                    )]
                                },
                                EffectDef::Apply {
                                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                    effect: AppliedEffectDef::modify_power_toughness(
                                        ValueDef::Constant(2),
                                        ValueDef::Constant(1),
                                    ),
                                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                                },
                            )
                        },
                    ),
                },
            ),
        ]),
);

// ICE 237 — Forgotten Lore
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORGOTTEN_LORE: CardRecord = CardRecord::new(
    "Forgotten Lore",
    "fb01dd39-a957-4c1a-86cf-f31a699a154a",
    "Harold McNeill",
    crate::card::CardRules::unsupported(),
);

// ICE 238 — Foxfire
pub(in crate::card::sets) static FOXFIRE: CardRecord = CardRecord::new(
    "Foxfire",
    "88db9685-6a2f-4548-b6c4-669918d653b4",
    "Margaret Organ-Kean",
CardRules::new_instant(mana_cost!("{2}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Untap target attacking creature. Prevent all combat damage that would be dealt to and dealt by that creature this turn.\nDraw a card at the beginning of the next turn's upkeep.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Attacking,
            ]),
        )],
        EffectDef::Sequence(&[
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
            EffectDef::PreventDamage {
                prevention: crate::card::DamagePreventionDef::unlimited(
                    crate::card::DamageEventMatcherDef::combat_to(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::PreventDamage {
                prevention: crate::card::DamagePreventionDef::unlimited(
                    crate::card::DamageEventMatcherDef::combat_from(ObjectRefDef::Target(
                        TargetIndex::PRIMARY,
                    )),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            DRAW_AT_NEXT_UPKEEP,
        ]),
    )),
);

// ICE 239 — Freyalise Supplicant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FREYALISE_SUPPLICANT: CardRecord = CardRecord::new(
    "Freyalise Supplicant",
    "5b1e718a-882a-4bdc-9d62-4dda88da0ba0",
    "Liz Danforth & Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// ICE 240 — Freyalise's Charm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FREYALISE_S_CHARM: CardRecord = CardRecord::new(
    "Freyalise's Charm",
    "3e147ac1-d221-49c7-966e-5e665ddeab6b",
    "Margaret Organ-Kean",
    crate::card::CardRules::unsupported(),
);

// ICE 241 — Freyalise's Winds
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FREYALISE_S_WINDS: CardRecord = CardRecord::new(
    "Freyalise's Winds",
    "b11cd2e0-9419-4267-807e-5b73915c748a",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// ICE 242 — Fyndhorn Brownie
pub(in crate::card::sets) static FYNDHORN_BROWNIE: CardRecord = CardRecord::new(
    "Fyndhorn Brownie",
    "06204e82-9dfd-4334-a23a-f8240fc37772",
    "Richard Thomas",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Ouphe"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{2}{G}, {T}: Untap target creature.",
            &[CostDef::Mana(mana_cost!("{2}{G}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ),
);

// ICE 243 — Fyndhorn Elder
pub(in crate::card::sets) static FYNDHORN_ELDER: CardRecord = CardRecord::new(
    "Fyndhorn Elder",
    "fca8aa11-f7cb-4f88-a041-30098579f1d2",
    "Christopher Rush",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Elf", "Druid"], 1, 1).with_ability(
        AbilityDef::activated_mana(
            "{T}: Add {G}{G}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::combination(&[ManaColor::Green], 2)),
        ),
    ),
);

// ICE 244 — Fyndhorn Elves
pub(in crate::card::sets) static FYNDHORN_ELVES: CardRecord = CardRecord::new(
    "Fyndhorn Elves",
    "3ba95ffa-990a-4013-98b7-5d8c0b34e9c4",
    "Justin Hampton",
    CardRules::new_creature(mana_cost!("{G}"), &["Elf", "Druid"], 1, 1).with_ability(
        AbilityDef::activated_mana(
            "{T}: Add {G}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
        ),
    ),
);

// ICE 245 — Fyndhorn Pollen
pub(in crate::card::sets) static FYNDHORN_POLLEN: CardRecord = CardRecord::new(
    "Fyndhorn Pollen",
    "3efbe59d-bebc-40b1-85ac-2e4c1ff3731e",
    "Phil Foglio",
    // A standing tax on every attacker in the game, with a pump-in-reverse to
    // finish the job on the turn it matters.
    CardRules::new_enchantment(mana_cost!("{2}{G}")).with_abilities(&[
        abilities::cumulative_upkeep(&[CostDef::mana(mana_cost!("{1}"))]),
        AbilityDef::static_ability(
            "All creatures get -1/-0.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-1),
                    ValueDef::Constant(0),
                ),
            },
        ),
        AbilityDef::activated(
            "{1}{G}: All creatures get -1/-0 until end of turn.",
            &[CostDef::Mana(mana_cost!("{1}{G}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-1),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// ICE 246 — Giant Growth (reprint)
const GIANT_GROWTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::GIANT_GROWTH,
    "431c9749-fd7b-4960-a910-8d41d3704e6c",
    "Allen Williams",
);

// ICE 247 — Gorilla Pack
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GORILLA_PACK: CardRecord = CardRecord::new(
    "Gorilla Pack",
    "046f6b76-5f17-4728-aa34-72b7eff1d4c9",
    "Anthony S. Waters",
    crate::card::CardRules::unsupported(),
);

// ICE 248 — Hot Springs
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HOT_SPRINGS: CardRecord = CardRecord::new(
    "Hot Springs",
    "1d4fe072-81a7-424e-8d21-aaca010d5b1d",
    "Nicola Leonard",
    crate::card::CardRules::unsupported(),
);

// ICE 249 — Hurricane (reprint)
const HURRICANE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HURRICANE,
    "a8cc6db7-1f40-40e3-a7ea-92f1d05e2e3d",
    "Cornelius Brudi",
);

// ICE 250 — Johtull Wurm
pub(in crate::card::sets) static JOHTULL_WURM: CardRecord = CardRecord::new(
    "Johtull Wurm",
    "64a22e88-f7b1-48c8-a199-e57edcd50654",
    "Daniel Gelon",
    // The same penalty on a bigger body, and the toughness holds up better than
    // the power does, so it survives the swarm it can no longer kill.
    CardRules::new_creature(mana_cost!("{5}{G}"), &["Wurm"], 6, 6).with_ability(
        AbilityDef::triggered(
            "Whenever this creature becomes blocked, it gets -2/-1 until end of turn for each \
             creature blocking it beyond the first.",
            TriggerEventDef::BecomesBlocked(ObjectPredicateDef::Source),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Scaled(
                        &const { ScaledValueDef::new(ValueDef::TriggerEventAmount, -2) },
                    ),
                    ValueDef::Scaled(
                        &const { ScaledValueDef::new(ValueDef::TriggerEventAmount, -1) },
                    ),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ICE 251 — Juniper Order Druid
pub(in crate::card::sets) static JUNIPER_ORDER_DRUID: CardRecord = CardRecord::new(
    "Juniper Order Druid",
    "cb211704-ff8e-498b-b7bb-f8384f198ffd",
    "Jeff A. Menges",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human", "Cleric", "Druid"], 1, 1)
        .with_ability(AbilityDef::activated_with_targets(
            "{T}: Untap target land.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Land),
            )],
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        )),
);

// ICE 252 — Lhurgoyf
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LHURGOYF: CardRecord = CardRecord::new(
    "Lhurgoyf",
    "fee6d385-d44b-4f1a-beb1-13aeebde063e",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// ICE 253 — Lure (reprint)
const LURE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::LURE,
    "87af69ee-c2bb-46ea-8d36-d484d04a3c8a",
    "Phil Foglio",
);

// ICE 254 — Maddening Wind
pub(in crate::card::sets) static MADDENING_WIND: CardRecord = CardRecord::new(
    "Maddening Wind",
    "5277656c-70f5-4660-bd58-7d9261d53fb5",
    "Dameon Willich",
    // An Aura that does nothing to the creature it hangs on and everything to
    // the player holding it, two life at a time.
    CardRules::new_enchantment(mana_cost!("{2}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            abilities::cumulative_upkeep(&[CostDef::mana(mana_cost!("{G}"))]),
            abilities::enchanted_controller_upkeep(
                "At the beginning of the upkeep of enchanted creature's controller, this Aura \
                 deals 2 damage to that player.",
                EffectDef::damage(
                    EffectRecipientDef::player(PlayerRefDef::ControllerOf(
                        ObjectRefDef::AttachedToSource,
                    )),
                    ValueDef::Constant(2),
                ),
            ),
        ]),
);

// ICE 255 — Nature's Lore
pub(in crate::card::sets) static NATURE_S_LORE: CardRecord = CardRecord::new(
    "Nature's Lore",
    "668d2969-b6b7-4507-bdd4-20bbaa68035a",
    "Rick Emond",
    CardRules::new_sorcery(mana_cost!("{1}{G}")).with_ability(AbilityDef::spell(
        "Search your library for a Forest card, put that card onto the battlefield, then shuffle.",
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
            minimum: 0,
            maximum: ValueDef::Constant(1),
            reveal: false,
            destination: ZoneKind::Battlefield,
            placement: crate::card::ZonePlacement::Top,
            shuffle: true,
            enters_tapped: false,
            attachment: None,
            binding: None,
            then: None,
        },
    )),
);

// ICE 256 — Pale Bears
pub(in crate::card::sets) static PALE_BEARS: CardRecord = CardRecord::new(
    "Pale Bears",
    "7f19c2a3-6403-4a78-bf45-6e339578d673",
    "Anthony S. Waters",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Bear"], 2, 2)
        .with_ability(abilities::landwalk(BasicLandType::Island)),
);

// ICE 257 — Pygmy Allosaurus
pub(in crate::card::sets) static PYGMY_ALLOSAURUS: CardRecord = CardRecord::new(
    "Pygmy Allosaurus",
    "88a68767-9822-4f15-895e-32164e2159be",
    "Anson Maddocks",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Dinosaur"], 2, 2)
        .with_ability(abilities::landwalk(BasicLandType::Swamp)),
);

// ICE 258 — Pyknite
pub(in crate::card::sets) static PYKNITE: CardRecord = CardRecord::new(
    "Pyknite",
    "6ffc64e4-ae3c-49f9-8ed6-518dd497bfe6",
    "Edward P. Beard, Jr.",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Ouphe"], 1, 1).with_ability(
        abilities::enters_trigger(
            "When this creature enters, draw a card at the beginning of the next turn's upkeep.",
            DRAW_AT_NEXT_UPKEEP,
        ),
    ),
);

// ICE 259 — Regeneration (reprint)
const REGENERATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::REGENERATION,
    "1dacfaec-6b61-450d-a134-2087c38a298a",
    "Justin Hampton",
);

// ICE 260 — Rime Dryad
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIME_DRYAD: CardRecord = CardRecord::new(
    "Rime Dryad",
    "7a93e6ce-1295-41f8-b454-2dfe321481a6",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// ICE 261 — Ritual of Subdual
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RITUAL_OF_SUBDUAL: CardRecord = CardRecord::new(
    "Ritual of Subdual",
    "5c5c01e7-8116-45fc-afc3-d52a31a635cb",
    "Justin Hampton",
    crate::card::CardRules::unsupported(),
);

// ICE 262 — Scaled Wurm
pub(in crate::card::sets) static SCALED_WURM: CardRecord = CardRecord::new(
    "Scaled Wurm",
    "499cd7fa-c86c-4a5f-b36d-8160e8a6af1f",
    "Daniel Gelon",
    CardRules::new_creature(mana_cost!("{7}{G}"), &["Wurm"], 7, 6),
);

// ICE 263 — Shambling Strider
pub(in crate::card::sets) static SHAMBLING_STRIDER: CardRecord = CardRecord::new(
    "Shambling Strider",
    "8886ba2d-b25a-4b74-9299-911c509ae864",
    "Douglas Shuler",
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Yeti"], 5, 5).with_ability(
        AbilityDef::activated(
            "{R}{G}: This creature gets +1/-1 until end of turn.",
            &[CostDef::Mana(mana_cost!("{R}{G}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(-1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ICE 264 — Snowblind
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SNOWBLIND: CardRecord = CardRecord::new(
    "Snowblind",
    "5f62c376-487a-42bc-bd85-ab8b0480f7dc",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// ICE 265 — Stampede
pub(in crate::card::sets) static STAMPEDE: CardRecord = CardRecord::new(
    "Stampede",
    "bc8265a1-4621-4d25-8f7f-f0179951a694",
    "Jeff A. Menges",
    CardRules::new_instant(mana_cost!("{1}{G}{G}")).with_ability(AbilityDef::spell(
        "Attacking creatures get +1/+0 and gain trample until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::Attacking,
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
                AppliedEffectDef::add_ability(&abilities::trample()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// ICE 266 — Stunted Growth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STUNTED_GROWTH: CardRecord = CardRecord::new(
    "Stunted Growth",
    "4c9b7393-eb35-4c99-bbf5-bcf924aa8ff3",
    "NéNé Thomas",
    crate::card::CardRules::unsupported(),
);

// ICE 267 — Tarpan
pub(in crate::card::sets) static TARPAN: CardRecord = CardRecord::new(
    "Tarpan",
    "b1420ec5-367c-4514-86c5-3993bf339e37",
    "Margaret Organ-Kean",
    CardRules::new_creature(mana_cost!("{G}"), &["Horse"], 1, 1).with_ability(
        abilities::dies_trigger(
            "When this creature dies, you gain 1 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// ICE 268 — Thermokarst
pub(in crate::card::sets) static THERMOKARST: CardRecord = CardRecord::new(
    "Thermokarst",
    "00ae906b-2c4d-48e9-9f2d-217777e22292",
    "Ken Meyer, Jr.",
    // The green printing: the same land destruction, with a life rebate
    // instead of the damage.
    CardRules::new_sorcery(mana_cost!("{1}{G}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target land. If that land was a snow land, you gain 1 life.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Land),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
            // "Was a snow land" is read after the destruction, so the slot is
            // asked about a land that has already left.
            EffectDef::IfCondition {
                condition: &const {
                    TriggerConditionDef::TargetMatches {
                        slot: TargetIndex::PRIMARY,
                        object: ObjectPredicateDef::Supertype(CardSupertype::Snow),
                    }
                },
                then: &const {
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    }
                },
            },
        ]),
    )),
);

// ICE 269 — Thoughtleech
pub(in crate::card::sets) static THOUGHTLEECH: CardRecord = CardRecord::new(
    "Thoughtleech",
    "d8fe7f9d-644f-48d0-93fa-d9a536f1f755",
    "Mark Tedin",
    CardRules::new_enchantment(mana_cost!("{G}{G}")).with_ability(AbilityDef::triggered(
        "Whenever an Island an opponent controls becomes tapped, you may gain 1 life.",
        TriggerEventDef::tapped(ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
            ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
        ])),
        EffectDef::May {
            player: EffectRecipientDef::Controller,
            effect: &EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        },
    )),
);

// ICE 270 — Tinder Wall
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TINDER_WALL: CardRecord = CardRecord::new(
    "Tinder Wall",
    "2a7c6489-21e9-4b86-a54a-b1e2f1fce318",
    "Rick Emond",
    crate::card::CardRules::unsupported(),
);

// ICE 271 — Touch of Vitae
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOUCH_OF_VITAE: CardRecord = CardRecord::new(
    "Touch of Vitae",
    "48d2cd18-a24d-40e0-a654-777d9e623ae2",
    "Allen Williams",
    crate::card::CardRules::unsupported(),
);

// ICE 272 — Trailblazer
pub(in crate::card::sets) static TRAILBLAZER: CardRecord = CardRecord::new(
    "Trailblazer",
    "9194c69d-c849-4c4a-976c-d1382bd5cf32",
    "Julie Baroh",
    CardRules::new_instant(mana_cost!("{2}{G}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature can't be blocked this turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// ICE 273 — Venomous Breath
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VENOMOUS_BREATH: CardRecord = CardRecord::new(
    "Venomous Breath",
    "8eeb9e02-1d26-4959-a878-2ef8db2358bc",
    "Allen Williams",
    crate::card::CardRules::unsupported(),
);

// ICE 274 — Wall of Pine Needles
pub(in crate::card::sets) static WALL_OF_PINE_NEEDLES: CardRecord = CardRecord::new(
    "Wall of Pine Needles",
    "5d879923-55fc-46ab-9306-5e1f10441c89",
    "Brian Snõddy",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Plant", "Wall"], 3, 3).with_abilities(&[
        abilities::defender(),
        AbilityDef::activated(
            "{G}: Regenerate this creature.",
            &[CostDef::Mana(mana_cost!("{G}"))],
            EffectDef::Regenerate {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// ICE 275 — Whiteout
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WHITEOUT: CardRecord = CardRecord::new(
    "Whiteout",
    "a8645e4f-eaa8-4420-a6a3-eb53c311fab1",
    "NéNé Thomas",
    crate::card::CardRules::unsupported(),
);

// ICE 276 — Wiitigo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WIITIGO: CardRecord = CardRecord::new(
    "Wiitigo",
    "9ee86bf2-6c54-4c6e-8394-eb39f98d5a85",
    "Melissa A. Benson",
    crate::card::CardRules::unsupported(),
);

// ICE 277 — Wild Growth (reprint)
const WILD_GROWTH_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::WILD_GROWTH,
    "f8047ab9-a0fc-4933-bcbc-e761aa0f622b",
    "Mike Raabe",
);

// ICE 278 — Woolly Mammoths
pub(in crate::card::sets) static WOOLLY_MAMMOTHS: CardRecord = CardRecord::new(
    "Woolly Mammoths",
    "eaca1216-99c8-4ad5-a51a-3c4ff3b82097",
    "Dan Frazier",
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Elephant"], 3, 2).with_ability(
        AbilityDef::static_ability(
            "This creature has trample as long as you control a snow land.",
            EffectDef::ConditionalStatic(ConditionalStaticEffectDef {
                condition: ObjectSetCountConditionDef {
                    objects: &ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::Supertype(CardSupertype::Snow),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    predicate: ObjectSetPredicateDef {
                        filter: None,
                        comparison: ComparisonDef::GreaterOrEqual,
                        amount: 1,
                    },
                },
                then: StaticApplyDef {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::trample()),
                },
            }),
        ),
    ),
);

// ICE 279 — Woolly Spider
pub(in crate::card::sets) static WOOLLY_SPIDER: CardRecord = CardRecord::new(
    "Woolly Spider",
    "e10520b2-b5a7-4328-84c8-20443b6f588a",
    "Daniel Gelon",
CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Spider"], 2, 3).with_abilities(&[
        abilities::reach(),
        AbilityDef::triggered(
            "Whenever this creature blocks a creature with flying, this creature gets +0/+2 until end of turn.",
            TriggerEventDef::Blocks {
                blocked: ObjectPredicateDef::HasKeyword(crate::card::KeywordAbility::Flying),
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// ICE 280 — Yavimaya Gnats
pub(in crate::card::sets) static YAVIMAYA_GNATS: CardRecord = CardRecord::new(
    "Yavimaya Gnats",
    "9d8b7020-ca8f-4867-bc51-13d824daf154",
    "Dan Frazier",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Insect"], 0, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{G}: Regenerate this creature.",
            &[CostDef::Mana(mana_cost!("{G}"))],
            EffectDef::Regenerate {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// ICE 281 — Altar of Bone
pub(in crate::card::sets) static ALTAR_OF_BONE: CardRecord = CardRecord::new(
    "Altar of Bone",
    "75d5b014-8675-4d91-a539-ac5c31d44b35",
    "Melissa A. Benson",
CardRules::new_sorcery(mana_cost!("{G}{W}"))
        .with_ability(
            AbilityDef::spell(
                "Search your library for a creature card, reveal it, put it into your hand, then shuffle.",
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    minimum: 0,
                    maximum: ValueDef::Constant(1),
                    reveal: true,
                    destination: ZoneKind::Hand,
                    placement: crate::card::ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: false,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            )
            .with_spell_additional_cost(&CostDef::sacrifice(
                ObjectPredicateDef::HasType(CardType::Creature),
                CostQuantityDef::Fixed(1),
            )),
        ),
);

// ICE 282 — Centaur Archer
pub(in crate::card::sets) static CENTAUR_ARCHER: CardRecord = CardRecord::new(
    "Centaur Archer",
    "e275c295-72da-4a86-82c6-cfd75b38b19c",
    "Melissa A. Benson",
    CardRules::new_creature(mana_cost!("{1}{R}{G}"), &["Centaur", "Archer"], 3, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: This creature deals 1 damage to target creature with flying.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasKeyword(crate::card::KeywordAbility::Flying),
                ]),
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(1),
            ),
        ),
    ),
);

// ICE 283 — Chromatic Armor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHROMATIC_ARMOR: CardRecord = CardRecord::new(
    "Chromatic Armor",
    "2657e85b-8f77-41fa-9df2-233443efef43",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// ICE 284 — Diabolic Vision
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIABOLIC_VISION: CardRecord = CardRecord::new(
    "Diabolic Vision",
    "1ea01324-1cfb-498c-8299-f690373864bd",
    "Anthony S. Waters",
    crate::card::CardRules::unsupported(),
);

// ICE 285 — Earthlink
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EARTHLINK: CardRecord = CardRecord::new(
    "Earthlink",
    "a83cb1c4-7c5b-4a5e-b15e-138d644f5cdb",
    "Richard Kane Ferguson",
    crate::card::CardRules::unsupported(),
);

// ICE 286 — Elemental Augury
pub(in crate::card::sets) static ELEMENTAL_AUGURY: CardRecord = CardRecord::new(
    "Elemental Augury",
    "62bbff2a-5109-400a-961b-eacffb9aed67",
    "Anthony S. Waters",
CardRules::new_enchantment(mana_cost!("{U}{B}{R}")).with_ability(
        AbilityDef::activated_with_targets(
            "{3}: Look at the top three cards of target player's library, then put them back in any order.",
            &[CostDef::Mana(mana_cost!("{3}"))],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            abilities::look_at_top_cards_and_reorder(
                PlayerRefDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(3),
            ),
        ),
    ),
);

// ICE 287 — Essence Vortex
// Audit: unsupported — Needs a life payment scaled by the target. CostDef::Life takes a fixed number, so "pays life equal to its toughness" has no cost to put on the unless.
pub(in crate::card::sets) static ESSENCE_VORTEX: CardRecord = CardRecord::new(
    "Essence Vortex",
    "fe07e496-5070-4116-a91a-a3bbe19c12af",
    "Margaret Organ-Kean",
    crate::card::CardRules::unsupported(),
);

// ICE 288 — Fiery Justice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FIERY_JUSTICE: CardRecord = CardRecord::new(
    "Fiery Justice",
    "8965ce61-0522-4f77-a82d-89441d1ba867",
    "Melissa A. Benson",
    crate::card::CardRules::unsupported(),
);

// ICE 289 — Fire Covenant
pub(in crate::card::sets) static FIRE_COVENANT: CardRecord = CardRecord::new(
    "Fire Covenant",
    "6a0139c2-ad86-4c71-ab6d-4840c37d5d20",
    "Dan Frazier",
    // The life is paid as it is cast, so it is spent whether or not the
    // spell resolves -- and it is life, so nothing about the board caps how
    // much damage three mana can deal.
    CardRules::new_instant(mana_cost!("{1}{B}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "As an additional cost to cast this spell, pay X life. This spell deals X damage \
             divided as you choose among any number of target creatures.",
            // "Any number of target creatures" is however many shares X splits into,
            // and X is the life its caster was willing to spend rather than anything in
            // the mana cost -- three mana kills a board if you have the life for it.
            &[AbilityTargetDef {
                predicate: AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                minimum: 0,
                maximum: AbilityTargetDef::UNLIMITED,
                exact_count: None,
                divided_total: Some(DividedTotal::ChosenX),
                another: false,
                excludes_source: false,
                chooser: TargetChooserDef::Controller,
            }],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::DividedAmongTargets,
            ),
        )
        .with_spell_additional_cost(&CostDef::pay_life(CostQuantityDef::ChosenX)),
    ),
);

// ICE 290 — Flooded Woodlands
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLOODED_WOODLANDS: CardRecord = CardRecord::new(
    "Flooded Woodlands",
    "de89e9e1-485b-42e5-9728-5d6f948999e1",
    "Kaja Foglio",
    crate::card::CardRules::unsupported(),
);

// ICE 291 — Fumarole
pub(in crate::card::sets) static FUMAROLE: CardRecord = CardRecord::new(
    "Fumarole",
    "efa53e9a-0d7c-4d17-b2be-56930edfa2c2",
    "Drew Tucker",
    // Two permanents for five mana and three life, which is the sort of rate a
    // gold card in a slow format could still ask for.
    CardRules::new_sorcery(mana_cost!("{3}{B}{R}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, pay 3 life.\nDestroy target creature and \
             target land.",
            &const {
                [
                    AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                        CardType::Creature,
                    )),
                    AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                        CardType::Land,
                    )),
                ]
            },
            CostDef::pay_life(CostQuantityDef::Fixed(3)),
            EffectDef::Sequence(
                &const {
                    [
                        EffectDef::Destroy {
                            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            then: None,
                        },
                        EffectDef::Destroy {
                            object: EffectRecipientDef::Target(TargetIndex(1)),
                            then: None,
                        },
                    ]
                },
            ),
        ),
    ),
);

// ICE 292 — Ghostly Flame
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GHOSTLY_FLAME: CardRecord = CardRecord::new(
    "Ghostly Flame",
    "6314344b-6493-4142-9c76-da9b90b8d3e1",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// ICE 293 — Giant Trap Door Spider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GIANT_TRAP_DOOR_SPIDER: CardRecord = CardRecord::new(
    "Giant Trap Door Spider",
    "8965dfa8-dc90-4cf2-a93b-72bf88b58936",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// ICE 294 — Glaciers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLACIERS: CardRecord = CardRecord::new(
    "Glaciers",
    "b86e159b-ecf1-4b4a-9041-4e97fdf935e5",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// ICE 295 — Hymn of Rebirth
pub(in crate::card::sets) static HYMN_OF_REBIRTH: CardRecord = CardRecord::new(
    "Hymn of Rebirth",
    "61d0f2f2-f6e2-4b8a-8418-10b17c5e0ea9",
    "Richard Kane Ferguson",
    CardRules::new_sorcery(mana_cost!("{3}{G}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Put target creature card from a graveyard onto the battlefield under your control.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::WithBattlefieldArrival {
            effect: &EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Battlefield,
                placement: crate::card::ZonePlacement::Top,
            },
            arrival: crate::card::BattlefieldArrivalDef {
                controller: Some(PlayerRelation::You),
                ..crate::card::BattlefieldArrivalDef::DEFAULT
            },
        },
    )),
);

// ICE 296 — Kjeldoran Frostbeast
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KJELDORAN_FROSTBEAST: CardRecord = CardRecord::new(
    "Kjeldoran Frostbeast",
    "2fccb1d0-b324-4780-bb9e-4533240da06d",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// ICE 297 — Merieke Ri Berit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MERIEKE_RI_BERIT: CardRecord = CardRecord::new(
    "Merieke Ri Berit",
    "3bf47c0a-5c17-47d0-b663-becff62fbdf8",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// ICE 298 — Monsoon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MONSOON: CardRecord = CardRecord::new(
    "Monsoon",
    "254fcc50-79a5-40cd-b028-e78dde3f8480",
    "NéNé Thomas",
    crate::card::CardRules::unsupported(),
);

// ICE 299 — Mountain Titan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOUNTAIN_TITAN: CardRecord = CardRecord::new(
    "Mountain Titan",
    "bcc1d589-02a2-4896-a283-9d0385534667",
    "Melissa A. Benson",
    crate::card::CardRules::unsupported(),
);

// ICE 300 — Reclamation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RECLAMATION: CardRecord = CardRecord::new(
    "Reclamation",
    "ca335f4f-d345-4eb9-9bc6-74595c501078",
    "Dameon Willich",
    crate::card::CardRules::unsupported(),
);

// ICE 301 — Skeleton Ship
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKELETON_SHIP: CardRecord = CardRecord::new(
    "Skeleton Ship",
    "271c8a7c-0f71-4f9d-ab0e-ca7c8c4aca50",
    "Amy Weber & Tom Wänerstrand",
    crate::card::CardRules::unsupported(),
);

// ICE 302 — Spectral Shield
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPECTRAL_SHIELD: CardRecord = CardRecord::new(
    "Spectral Shield",
    "7fe0a783-d086-4dc8-ae4a-59f3c2daaca0",
    "Margaret Organ-Kean",
    crate::card::CardRules::unsupported(),
);

// ICE 303 — Storm Spirit
pub(in crate::card::sets) static STORM_SPIRIT: CardRecord = CardRecord::new(
    "Storm Spirit",
    "7a383a5f-4814-4b92-aa80-2a6440a719bc",
    "Pete Venters",
    CardRules::new_creature(mana_cost!("{3}{G}{W}{U}"), &["Elemental", "Spirit"], 3, 3)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::activated_with_targets(
                "{T}: This creature deals 2 damage to target creature.",
                &[CostDef::TapSource],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(2),
                ),
            ),
        ]),
);

// ICE 304 — Stormbind
pub(in crate::card::sets) static STORMBIND: CardRecord = CardRecord::new(
    "Stormbind",
    "c2d5d91b-aeb4-4d7e-b748-77f9960da55f",
    "NéNé Thomas & Phillip Mosness",
    CardRules::new_enchantment(mana_cost!("{1}{R}{G}")).with_ability(
        AbilityDef::activated_with_targets(
            "{2}, Discard a card at random: This enchantment deals 2 damage to any target.",
            &[
                CostDef::Mana(mana_cost!("{2}")),
                CostDef::DiscardCardsAtRandom(1),
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(2),
            ),
        ),
    ),
);

// ICE 305 — Wings of Aesthir
pub(in crate::card::sets) static WINGS_OF_AESTHIR: CardRecord = CardRecord::new(
    "Wings of Aesthir",
    "eeb0282d-ccec-4556-8b70-b6f665077afe",
    "Edward P. Beard, Jr.",
    CardRules::new_enchantment(mana_cost!("{W}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +1/+0 and has flying and first strike.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(0),
                        ),
                        AppliedEffectDef::add_ability(&abilities::flying()),
                        AppliedEffectDef::add_ability(&abilities::first_strike()),
                    ]),
                },
            ),
        ]),
);

// ICE 306 — Adarkar Sentinel
pub(in crate::card::sets) static ADARKAR_SENTINEL: CardRecord = CardRecord::new(
    "Adarkar Sentinel",
    "ff62754b-f4f0-4731-8dd7-327a820f60a8",
    "Melissa A. Benson",
    CardRules::new_artifact_creature(mana_cost!("{5}"), &["Soldier"], 3, 3).with_ability(
        AbilityDef::activated(
            "{1}: This creature gets +0/+1 until end of turn.",
            &[CostDef::Mana(mana_cost!("{1}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ICE 307 — Aegis of the Meek
pub(in crate::card::sets) static AEGIS_OF_THE_MEEK: CardRecord = CardRecord::new(
    "Aegis of the Meek",
    "5d272051-f442-4f6e-8c64-df28b398d2e8",
    "Allen Williams",
    CardRules::new_artifact(mana_cost!("{3}")).with_ability(AbilityDef::activated_with_targets(
        "{1}, {T}: Target 1/1 creature gets +1/+2 until end of turn.",
        &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::PowerExactly(1),
                ObjectPredicateDef::ToughnessExactly(1),
            ]),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(1),
                ValueDef::Constant(2),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// ICE 308 — Amulet of Quoz
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AMULET_OF_QUOZ: CardRecord = CardRecord::new(
    "Amulet of Quoz",
    "764ec6a8-a878-446c-b7e4-6026c2a3e9a4",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// ICE 309 — Arcum's Sleigh
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARCUM_S_SLEIGH: CardRecord = CardRecord::new(
    "Arcum's Sleigh",
    "e9780ce2-756c-48e5-9936-45f6a224f61d",
    "Tom Wänerstrand",
    crate::card::CardRules::unsupported(),
);

// ICE 310 — Arcum's Weathervane
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARCUM_S_WEATHERVANE: CardRecord = CardRecord::new(
    "Arcum's Weathervane",
    "9e142435-6930-4596-bc3b-60abde1229df",
    "Tom Wänerstrand",
    crate::card::CardRules::unsupported(),
);

// ICE 311 — Arcum's Whistle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARCUM_S_WHISTLE: CardRecord = CardRecord::new(
    "Arcum's Whistle",
    "73c07c87-0e44-4a5a-92b7-728350cd02de",
    "Quinton Hoover",
    crate::card::CardRules::unsupported(),
);

// ICE 312 — Barbed Sextant
// Audit: unsupported — Activated mana abilities cannot yet install the delayed draw trigger
// while remaining inside the shared mana-ability runtime boundary.
pub(in crate::card::sets) static BARBED_SEXTANT: CardRecord = CardRecord::new(
    "Barbed Sextant",
    "edb82654-de12-4dce-8c6b-f28d68f0fbe1",
    "Amy Weber",
    CardRules::unsupported(),
);

// ICE 313 — Baton of Morale
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BATON_OF_MORALE: CardRecord = CardRecord::new(
    "Baton of Morale",
    "8bc29872-b1a2-4851-9eca-f3e67ae6e14c",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// ICE 314 — Celestial Sword
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CELESTIAL_SWORD: CardRecord = CardRecord::new(
    "Celestial Sword",
    "2bc0e8d3-633b-4281-863f-c51c69eed0b6",
    "Amy Weber",
    crate::card::CardRules::unsupported(),
);

// ICE 315 — Crown of the Ages
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CROWN_OF_THE_AGES: CardRecord = CardRecord::new(
    "Crown of the Ages",
    "fce2991f-48e1-4cfe-af0a-18b6d9400493",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// ICE 316 — Despotic Scepter
pub(in crate::card::sets) static DESPOTIC_SCEPTER: CardRecord = CardRecord::new(
    "Despotic Scepter",
    "53e381a4-810e-4b75-aed3-c16cf0eb06fa",
    "Richard Thomas",
    CardRules::new_artifact(mana_cost!("{1}")).with_ability(AbilityDef::activated_with_targets(
        "{T}: Destroy target permanent you own. It can't be regenerated.",
        &[CostDef::TapSource],
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Any,
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: Some(PlayerRelation::You),
            },
        )],
        EffectDef::WithRule {
            rule: AppliedRuleDef::CannotRegenerate,
            effect: &EffectDef::destroy_target(TargetIndex::PRIMARY),
        },
    )),
);

// ICE 317 — Elkin Bottle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELKIN_BOTTLE: CardRecord = CardRecord::new(
    "Elkin Bottle",
    "49301c19-55a0-4146-9474-0b86cd320e31",
    "Quinton Hoover",
    crate::card::CardRules::unsupported(),
);

// ICE 318 — Fyndhorn Bow
pub(in crate::card::sets) static FYNDHORN_BOW: CardRecord = CardRecord::new(
    "Fyndhorn Bow",
    "65dd0a41-cc51-4728-b597-fdb2510accd8",
    "Rob Alexander",
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(AbilityDef::activated_with_targets(
        "{3}, {T}: Target creature gains first strike until end of turn.",
        &[CostDef::Mana(mana_cost!("{3}")), CostDef::TapSource],
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// ICE 319 — Goblin Lyre
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_LYRE: CardRecord = CardRecord::new(
    "Goblin Lyre",
    "951114fb-5ae5-4eb0-8e03-6e39b0b634b5",
    "Mike Kimble",
    crate::card::CardRules::unsupported(),
);

// ICE 320 — Hematite Talisman
pub(in crate::card::sets) static HEMATITE_TALISMAN: CardRecord = CardRecord::new(
    "Hematite Talisman",
    "83585337-56a9-44d2-9ed1-8a959bcfb010",
    "Allen Williams",
    // Three mana to untap anything, offered every time the opponent casts a
    // red spell -- which in a red deck is every turn.
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(AbilityDef::triggered_with_targets(
        "Whenever a player casts a red spell, you may pay {3}. If you do, untap target permanent.",
        TriggerEventDef::spell_cast(ObjectPredicateDef::Color(ManaColor::Red)),
        &const {
            [AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Any,
            )]
        },
        EffectDef::PayOr(PayOrDef::optional(
            &[CostDef::Mana(mana_cost!("{3}"))],
            &const {
                EffectDef::Untap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                }
            },
        )),
    )),
);

// ICE 321 — Ice Cauldron
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ICE_CAULDRON: CardRecord = CardRecord::new(
    "Ice Cauldron",
    "1a3e095a-7056-4df3-bf7d-9c217d591446",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// ICE 322 — Icy Manipulator (reprint)
const ICY_MANIPULATOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ICY_MANIPULATOR,
    "1eda936f-7691-4440-9b83-eb0c6035b109",
    "Amy Weber",
);

// ICE 323 — Infinite Hourglass
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INFINITE_HOURGLASS: CardRecord = CardRecord::new(
    "Infinite Hourglass",
    "f9a42152-32c0-47ff-aaac-8deaf01873ca",
    "Harold McNeill",
    crate::card::CardRules::unsupported(),
);

// ICE 324 — Jester's Cap
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JESTER_S_CAP: CardRecord = CardRecord::new(
    "Jester's Cap",
    "47ac44d0-8090-4e7b-ac47-c567294f185e",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// ICE 325 — Jester's Mask
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JESTER_S_MASK: CardRecord = CardRecord::new(
    "Jester's Mask",
    "daa1ba0c-cb89-4bb2-8a35-6a4a4eecccf7",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// ICE 326 — Jeweled Amulet
// Audit: unsupported — Needs remembered mana type. The artifact notes which type paid for its first ability and produces that type later, and no card-local state records a mana type.
pub(in crate::card::sets) static JEWELED_AMULET: CardRecord = CardRecord::new(
    "Jeweled Amulet",
    "34f7bad2-d28f-42d2-9246-fe3545ef49a7",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// ICE 327 — Lapis Lazuli Talisman
pub(in crate::card::sets) static LAPIS_LAZULI_TALISMAN: CardRecord = CardRecord::new(
    "Lapis Lazuli Talisman",
    "ce00bb19-983e-427d-be54-ae6daf0ccdde",
    "Amy Weber",
    // The blue member of the cycle, and the one whose trigger a control deck
    // hands you over and over.
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(AbilityDef::triggered_with_targets(
        "Whenever a player casts a blue spell, you may pay {3}. If you do, untap target permanent.",
        TriggerEventDef::spell_cast(ObjectPredicateDef::Color(ManaColor::Blue)),
        &const {
            [AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Any,
            )]
        },
        EffectDef::PayOr(PayOrDef::optional(
            &[CostDef::Mana(mana_cost!("{3}"))],
            &const {
                EffectDef::Untap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                }
            },
        )),
    )),
);

// ICE 328 — Malachite Talisman
pub(in crate::card::sets) static MALACHITE_TALISMAN: CardRecord = CardRecord::new(
    "Malachite Talisman",
    "63fb8a24-ce53-4a69-be2a-55c6dbba5ee7",
    "Christopher Rush",
// Green's copy: untapping a land is what it usually buys, which is the
    // mana back and then some.
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(AbilityDef::triggered_with_targets(
        "Whenever a player casts a green spell, you may pay {3}. If you do, untap target permanent.",
        TriggerEventDef::spell_cast(ObjectPredicateDef::Color(ManaColor::Green)),
        &const { [AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Any)] },
        EffectDef::PayOr(PayOrDef::optional(
            &[CostDef::Mana(mana_cost!("{3}"))],
            &const {
                EffectDef::Untap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                }
            },
        )),
    )),
);

// ICE 329 — Nacre Talisman
pub(in crate::card::sets) static NACRE_TALISMAN: CardRecord = CardRecord::new(
    "Nacre Talisman",
    "06912236-8225-4eb0-8086-c6a163c69892",
    "Mark Tedin",
// White's copy, which untaps a blocker on the turn the white deck was
    // counting on it being tapped.
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(AbilityDef::triggered_with_targets(
        "Whenever a player casts a white spell, you may pay {3}. If you do, untap target permanent.",
        TriggerEventDef::spell_cast(ObjectPredicateDef::Color(ManaColor::White)),
        &const { [AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Any)] },
        EffectDef::PayOr(PayOrDef::optional(
            &[CostDef::Mana(mana_cost!("{3}"))],
            &const {
                EffectDef::Untap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                }
            },
        )),
    )),
);

// ICE 330 — Naked Singularity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NAKED_SINGULARITY: CardRecord = CardRecord::new(
    "Naked Singularity",
    "cabadfb2-93cd-4c7a-b901-59c3dd1a7c3c",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// ICE 331 — Onyx Talisman
pub(in crate::card::sets) static ONYX_TALISMAN: CardRecord = CardRecord::new(
    "Onyx Talisman",
    "a89b2368-1180-4821-bcb8-8161c18e5538",
    "Sandra Everingham",
// Black's copy, and the only one whose trigger a black deck can be
    // relied on to give you.
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(AbilityDef::triggered_with_targets(
        "Whenever a player casts a black spell, you may pay {3}. If you do, untap target permanent.",
        TriggerEventDef::spell_cast(ObjectPredicateDef::Color(ManaColor::Black)),
        &const { [AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Any)] },
        EffectDef::PayOr(PayOrDef::optional(
            &[CostDef::Mana(mana_cost!("{3}"))],
            &const {
                EffectDef::Untap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                }
            },
        )),
    )),
);

// ICE 332 — Pentagram of the Ages
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PENTAGRAM_OF_THE_AGES: CardRecord = CardRecord::new(
    "Pentagram of the Ages",
    "b8d889a5-f6c7-410d-97f9-acf08b9091c8",
    "Douglas Shuler",
    crate::card::CardRules::unsupported(),
);

// ICE 333 — Pit Trap
pub(in crate::card::sets) static PIT_TRAP: CardRecord = CardRecord::new(
    "Pit Trap",
    "c588fe7f-945d-4459-904c-67442f88b4e1",
    "Anson Maddocks",
CardRules::new_artifact(mana_cost!("{2}")).with_ability(
        AbilityDef::activated_with_targets(
            "{2}, {T}, Sacrifice this artifact: Destroy target attacking creature without flying. It can't be regenerated.",
            &[
                CostDef::Mana(mana_cost!("{2}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Attacking,
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasKeyword(
                        crate::card::KeywordAbility::Flying,
                    )),
                ]),
            )],
            EffectDef::WithRule {
                rule: AppliedRuleDef::CannotRegenerate,
                effect: &EffectDef::destroy_target(TargetIndex::PRIMARY),
            },
        ),
    ),
);

// ICE 334 — Runed Arch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUNED_ARCH: CardRecord = CardRecord::new(
    "Runed Arch",
    "ca02861b-9639-480d-8e54-e024f0c70158",
    "Phil Foglio",
    crate::card::CardRules::unsupported(),
);

// ICE 335 — Shield of the Ages
pub(in crate::card::sets) static SHIELD_OF_THE_AGES: CardRecord = CardRecord::new(
    "Shield of the Ages",
    "7411ab40-47f6-44d1-8e33-9ff5301dcd9b",
    "Anson Maddocks",
    // No tap in the cost, so with enough mana it prevents as much as you
    // can pay for -- one point at a time, which is the catch.
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(AbilityDef::activated(
        "{2}: Prevent the next 1 damage that would be dealt to you this turn.",
        &[CostDef::Mana(mana_cost!("{2}"))],
        EffectDef::PreventDamage {
            prevention: DamagePreventionDef::amount(
                DamageEventMatcherDef::to(EffectRecipientDef::Controller),
                ValueDef::Constant(1),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// ICE 336 — Skull Catapult
pub(in crate::card::sets) static SKULL_CATAPULT: CardRecord = CardRecord::new(
    "Skull Catapult",
    "eb92a3e6-dc30-4a08-baba-e125290cadc5",
    "Bryon Wackwitz",
    CardRules::new_artifact(mana_cost!("{4}")).with_ability(AbilityDef::activated_with_targets(
        "{1}, {T}, Sacrifice a creature: This artifact deals 2 damage to any target.",
        &[
            CostDef::Mana(mana_cost!("{1}")),
            CostDef::TapSource,
            CostDef::SacrificePermanent {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                controller: PlayerRelation::You,
            },
        ],
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::damage(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ValueDef::Constant(2),
        ),
    )),
);

// ICE 337 — Snow Fortress
// Audit: unsupported — Targeting can identify an attacking creature, but cannot yet distinguish
// one attacking you from one attacking a planeswalker you protect.
pub(in crate::card::sets) static SNOW_FORTRESS: CardRecord = CardRecord::new(
    "Snow Fortress",
    "1c480e07-fb26-4760-865f-47985f7447bb",
    "Jeff A. Menges",
    CardRules::unsupported(),
);

// ICE 338 — Soldevi Golem
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOLDEVI_GOLEM: CardRecord = CardRecord::new(
    "Soldevi Golem",
    "64d35e88-81d3-4a54-aa79-190615abc616",
    "Anson Maddocks",
    crate::card::CardRules::unsupported(),
);

// ICE 339 — Soldevi Simulacrum
pub(in crate::card::sets) static SOLDEVI_SIMULACRUM: CardRecord = CardRecord::new(
    "Soldevi Simulacrum",
    "9fabc7b6-e766-4e3c-816e-04cfeceaff09",
    "Dan Frazier",
// A body that grows as long as you feed it, which is a mana sink for a
    // deck with nothing else to spend on.
    CardRules::new_artifact_creature(mana_cost!("{4}"), &["Construct"], 2, 4).with_abilities(&[
        abilities::cumulative_upkeep(
            &[CostDef::Mana(mana_cost!("{1}"))],
        ).override_text(
                "Cumulative upkeep {1} (At the beginning of your upkeep, put an age counter on this permanent, then sacrifice it unless you pay its upkeep cost for each age counter on it.)",
            ),
        AbilityDef::activated(
        "{1}: This creature gets +1/+0 until end of turn.",
        &[CostDef::Mana(mana_cost!("{1}"))],
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

// ICE 340 — Staff of the Ages
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STAFF_OF_THE_AGES: CardRecord = CardRecord::new(
    "Staff of the Ages",
    "5c709836-55b6-4de9-b190-b5f66dc53c87",
    "Daniel Gelon",
    crate::card::CardRules::unsupported(),
);

// ICE 341 — Sunstone
pub(in crate::card::sets) static SUNSTONE: CardRecord = CardRecord::new(
    "Sunstone",
    "3c1c67fa-ff88-4a61-b8a5-8a872b3dc44f",
    "Phil Foglio",
    CardRules::new_artifact(mana_cost!("{3}")).with_ability(AbilityDef::activated(
        "{2}, Sacrifice a snow land: Prevent all combat damage that would be dealt this turn.",
        &[
            CostDef::Mana(mana_cost!("{2}")),
            CostDef::SacrificePermanent {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::Supertype(CardSupertype::Snow),
                ]),
                controller: PlayerRelation::You,
            },
        ],
        EffectDef::PreventDamage {
            prevention: crate::card::DamagePreventionDef::unlimited(
                crate::card::DamageEventMatcherDef::COMBAT,
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// ICE 342 — Time Bomb
pub(in crate::card::sets) static TIME_BOMB: CardRecord = CardRecord::new(
    "Time Bomb",
    "092ec691-4729-46d3-a4e2-0cfc5df42a31",
    "Amy Weber",
CardRules::new_artifact(mana_cost!("{4}")).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep, put a time counter on this artifact.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::named("time"),
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated(
            "{1}, {T}, Sacrifice this artifact: This artifact deals damage equal to the number of time counters on it to each creature and each player.",
            &[
                CostDef::Mana(mana_cost!("{1}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            EffectDef::Sequence(&[
                EffectDef::damage(
                    EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    ),
                    ValueDef::CountersOnSource(CounterKind::named("time")),
                ),
                EffectDef::damage(
                    EffectRecipientDef::EachPlayer,
                    ValueDef::CountersOnSource(CounterKind::named("time")),
                ),
            ]),
        ),
    ]),
);

// ICE 343 — Urza's Bauble
pub(in crate::card::sets) static URZAS_BAUBLE: CardRecord = CardRecord::new(
    "Urza's Bauble",
    "58c9e9a7-e170-4361-b7d5-22fc0771c489",
    "Christopher Rush",
    // A free artifact that replaces itself a turn later, which is why the
    // decks that count artifacts or graveyard cards play it for no other
    // reason.
    CardRules::new_artifact(mana_cost!("{0}")).with_ability(AbilityDef::activated_with_targets(
        "{T}, Sacrifice this artifact: Look at a card at random in target player's hand. You draw \
         a card at the beginning of the next turn's upkeep.",
        &[CostDef::TapSource, CostDef::SacrificeSource],
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Sequence(&[
            EffectDef::LookAtRandomCardInHand {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
            // "You draw a card at the beginning of the next turn's upkeep": a delayed
            // draw rather than a cantrip, which is what makes the Bauble free to play
            // and slow to pay.
            EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                "At the beginning of the next turn's upkeep, you draw a card.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::Any,
                },
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ))),
        ]),
    )),
);

// ICE 344 — Vexing Arcanix
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VEXING_ARCANIX: CardRecord = CardRecord::new(
    "Vexing Arcanix",
    "0c9ea118-6a19-4e1b-aa5a-9b2729efc096",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// ICE 345 — Vibrating Sphere
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VIBRATING_SPHERE: CardRecord = CardRecord::new(
    "Vibrating Sphere",
    "48f93ded-ecf6-4a70-8ca3-a9c0c3201c21",
    "Richard Thomas",
    crate::card::CardRules::unsupported(),
);

// ICE 346 — Walking Wall
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WALKING_WALL: CardRecord = CardRecord::new(
    "Walking Wall",
    "cba1238c-1969-452d-8112-124cbbd49417",
    "Anthony S. Waters",
    crate::card::CardRules::unsupported(),
);

// ICE 347 — Wall of Shields
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WALL_OF_SHIELDS: CardRecord = CardRecord::new(
    "Wall of Shields",
    "6376c7c4-aaca-4625-83d4-a49f01aec535",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// ICE 348 — War Chariot
pub(in crate::card::sets) static WAR_CHARIOT: CardRecord = CardRecord::new(
    "War Chariot",
    "d0ea0c6c-aa76-4b16-bc99-2ff46dc56d4e",
    "Dameon Willich",
    CardRules::new_artifact(mana_cost!("{3}")).with_ability(AbilityDef::activated_with_targets(
        "{3}, {T}: Target creature gains trample until end of turn.",
        &[CostDef::Mana(mana_cost!("{3}")), CostDef::TapSource],
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::add_ability(&abilities::trample()),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// ICE 349 — Whalebone Glider
pub(in crate::card::sets) static WHALEBONE_GLIDER: CardRecord = CardRecord::new(
    "Whalebone Glider",
    "4b75adf0-9501-4776-a213-456c2b821070",
    "Amy Weber",
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(AbilityDef::activated_with_targets(
        "{2}, {T}: Target creature with power 3 or less gains flying until end of turn.",
        &[CostDef::Mana(mana_cost!("{2}")), CostDef::TapSource],
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::PowerLessThan(ValueDef::Constant(4)),
            ]),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::add_ability(&abilities::flying()),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// ICE 350 — Zuran Orb
pub(in crate::card::sets) static ZURAN_ORB: CardRecord = CardRecord::new(
    "Zuran Orb",
    "3a9d1082-a862-45d4-9e5e-392e879fead6",
    "Sandra Everingham",
    CardRules::new_artifact(mana_cost!("{0}")).with_ability(AbilityDef::activated(
        "Sacrifice a land: You gain 2 life.",
        &[CostDef::SacrificePermanent {
            object: ObjectPredicateDef::HasType(CardType::Land),
            controller: PlayerRelation::You,
        }],
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(2),
        },
    )),
);

// ICE 351 — Adarkar Wastes
pub(in crate::card::sets) static ADARKAR_WASTES: CardRecord = CardRecord::new(
    "Adarkar Wastes",
    "09dd9023-f7ee-4e99-8821-7059deb83730",
    "Mike Raabe",
    CardRules::new_land(&[]).with_abilities(&abilities::pain_land(
        "{T}: Add {W} or {U}. This land deals 1 damage to you.",
        &[ManaColor::White, ManaColor::Blue],
    )),
);

// ICE 352 — Brushland
pub(in crate::card::sets) static BRUSHLAND: CardRecord = CardRecord::new(
    "Brushland",
    "170e5ccd-54bf-4c6d-86b4-0359ca8f36e8",
    "Bryon Wackwitz",
    CardRules::new_land(&[]).with_abilities(&abilities::pain_land(
        "{T}: Add {G} or {W}. This land deals 1 damage to you.",
        &[ManaColor::Green, ManaColor::White],
    )),
);

// ICE 353 — Glacial Chasm
pub(in crate::card::sets) static GLACIAL_CHASM: CardRecord = CardRecord::new(
    "Glacial Chasm",
    "3d23f800-7a6f-40e3-b242-9f5955e47a75",
    "Liz Danforth",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::cumulative_upkeep(&[CostDef::life(2)]),
        abilities::enters_trigger(
            "When this land enters, sacrifice a land.",
            EffectDef::SacrificeOfChoice {
                player: EffectRecipientDef::Controller,
                object: ObjectPredicateDef::HasType(CardType::Land),
                count: ValueDef::Constant(1),
                then: None,
                amount: crate::card::SacrificedAmountDef::Power,
                otherwise: None,
                optional: false,
            },
        ),
        AbilityDef::static_ability(
            "Creatures you control can't attack.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_ATTACK),
            },
        ),
        AbilityDef::static_ability(
            "Prevent all damage that would be dealt to you.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::prevent_damage_from(ObjectPredicateDef::Any),
            },
        ),
    ]),
);

// ICE 354 — Halls of Mist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HALLS_OF_MIST: CardRecord = CardRecord::new(
    "Halls of Mist",
    "b926a189-90b6-47bb-b5d6-b033e57007b4",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// ICE 355 — Ice Floe
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ICE_FLOE: CardRecord = CardRecord::new(
    "Ice Floe",
    "85ce04fb-e687-41e0-ae9a-16a51df5d943",
    "Jeff A. Menges",
    crate::card::CardRules::unsupported(),
);

// ICE 356 — Karplusan Forest
pub(in crate::card::sets) static KARPLUSAN_FOREST: CardRecord = CardRecord::new(
    "Karplusan Forest",
    "ba6f1263-d598-49fb-b5f8-09f11822ebd0",
    "Nicola Leonard",
    CardRules::new_land(&[]).with_abilities(&abilities::pain_land(
        "{T}: Add {R} or {G}. This land deals 1 damage to you.",
        &[ManaColor::Red, ManaColor::Green],
    )),
);

// ICE 357 — Land Cap
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAND_CAP: CardRecord = CardRecord::new(
    "Land Cap",
    "c4806c02-7a4d-42e3-affd-0338084bd3ab",
    "Allen Williams",
    crate::card::CardRules::unsupported(),
);

// ICE 358 — Lava Tubes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAVA_TUBES: CardRecord = CardRecord::new(
    "Lava Tubes",
    "5e7c2cf6-f36f-451b-bba5-19a82c659c4c",
    "Bryon Wackwitz",
    crate::card::CardRules::unsupported(),
);

// ICE 359 — River Delta
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIVER_DELTA: CardRecord = CardRecord::new(
    "River Delta",
    "ea335fc0-0591-4acd-9ae8-7858222770da",
    "Sandra Everingham",
    crate::card::CardRules::unsupported(),
);

// ICE 360 — Sulfurous Springs
pub(in crate::card::sets) static SULFUROUS_SPRINGS: CardRecord = CardRecord::new(
    "Sulfurous Springs",
    "2fdeab50-b45f-412b-85a3-c6cf009ce567",
    "Phil Foglio",
    CardRules::new_land(&[]).with_abilities(&abilities::pain_land(
        "{T}: Add {B} or {R}. This land deals 1 damage to you.",
        &[ManaColor::Black, ManaColor::Red],
    )),
);

// ICE 361 — Timberline Ridge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TIMBERLINE_RIDGE: CardRecord = CardRecord::new(
    "Timberline Ridge",
    "87cc2fc9-0a24-4ac1-afcc-9317b90c7178",
    "Jeff A. Menges",
    crate::card::CardRules::unsupported(),
);

// ICE 362 — Underground River
pub(in crate::card::sets) static UNDERGROUND_RIVER: CardRecord = CardRecord::new(
    "Underground River",
    "92369d7e-5e5a-46f9-bb31-c57d62410283",
    "NéNé Thomas",
    CardRules::new_land(&[]).with_abilities(&abilities::pain_land(
        "{T}: Add {U} or {B}. This land deals 1 damage to you.",
        &[ManaColor::Blue, ManaColor::Black],
    )),
);

// ICE 363 — Veldt
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VELDT: CardRecord = CardRecord::new(
    "Veldt",
    "987534fb-74a9-46a3-805f-fe2fe2df4a90",
    "Bryon Wackwitz",
    crate::card::CardRules::unsupported(),
);

// ICE 364 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAINS,
    "7b68bdb0-41cc-48f6-905e-7da1ff4ba5e0",
    "Christopher Rush",
);

// ICE 365 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    1,
    "df3e94f7-9f97-4652-a1f1-381feb15f688",
    "Christopher Rush",
);

// ICE 366 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    2,
    "27ac1fc7-0698-4a94-8353-cc4c13bd6ffa",
    "Christopher Rush",
);

// ICE 367 — Snow-Covered Plains
pub(in crate::card::sets) static SNOW_COVERED_PLAINS: CardRecord = CardRecord::new(
    "Snow-Covered Plains",
    "cb3ac778-fb45-4fd3-a9af-8a0791f833e8",
    "Christopher Rush",
    CardRules::new_land(&["Plains"])
        .with_supertype(CardSupertype::Basic)
        .with_supertype(CardSupertype::Snow),
);

// ICE 368 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND,
    "ef2d6fc9-ddad-4dd2-b218-afa1a5449b7e",
    "Anson Maddocks",
);

// ICE 369 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    1,
    "61a467ab-4460-4e5e-94c1-8150bfe0c954",
    "Anson Maddocks",
);

// ICE 370 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    2,
    "82f11c42-9d67-4833-9519-e165e6a7e9c4",
    "Anson Maddocks",
);

// ICE 371 — Snow-Covered Island
pub(in crate::card::sets) static SNOW_COVERED_ISLAND: CardRecord = CardRecord::new(
    "Snow-Covered Island",
    "ad8b77cf-b53e-4da3-9c27-3851b7b25a98",
    "Anson Maddocks",
    CardRules::new_land(&["Island"])
        .with_supertype(CardSupertype::Basic)
        .with_supertype(CardSupertype::Snow),
);

// ICE 372 — Snow-Covered Swamp
pub(in crate::card::sets) static SNOW_COVERED_SWAMP: CardRecord = CardRecord::new(
    "Snow-Covered Swamp",
    "65a3c27f-6b15-49b6-ac89-36cfb79b3b54",
    "Douglas Shuler",
    CardRules::new_land(&["Swamp"])
        .with_supertype(CardSupertype::Basic)
        .with_supertype(CardSupertype::Snow),
);

// ICE 373 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWAMP,
    "4695653a-5c4c-4ff3-b80c-f4b6c685f370",
    "Douglas Shuler",
);

// ICE 374 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    1,
    "6a90b49f-53b3-4ce0-92c1-bcd76d6981ea",
    "Douglas Shuler",
);

// ICE 375 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    2,
    "ddca7e2e-bb0a-47ed-ade3-31900da992dc",
    "Douglas Shuler",
);

// ICE 376 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MOUNTAIN,
    "4ecf39c3-3b5f-4263-a7b5-9881bded3494",
    "Tom Wänerstrand",
);

// ICE 377 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    1,
    "2eb15b42-be2a-4663-b064-aad6c7cb2714",
    "Tom Wänerstrand",
);

// ICE 378 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    2,
    "17ac61e4-b543-4c37-9bfa-43f0c928152d",
    "Tom Wänerstrand",
);

// ICE 379 — Snow-Covered Mountain
pub(in crate::card::sets) static SNOW_COVERED_MOUNTAIN: CardRecord = CardRecord::new(
    "Snow-Covered Mountain",
    "ccd3afb3-5574-4f2d-adbe-969a428f1c63",
    "Tom Wänerstrand",
    CardRules::new_land(&["Mountain"])
        .with_supertype(CardSupertype::Basic)
        .with_supertype(CardSupertype::Snow),
);

// ICE 380 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOREST,
    "fbdcbd97-90a9-45ea-94f6-2a1c6faaf965",
    "Pat Lewis",
);

// ICE 381 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    1,
    "b346b784-7bde-49d0-bfa9-56236cbe19d9",
    "Pat Lewis",
);

// ICE 382 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    2,
    "768c4d8f-5700-4f0a-9ff2-58422aeb1dac",
    "Pat Lewis",
);

// ICE 383 — Snow-Covered Forest
pub(in crate::card::sets) static SNOW_COVERED_FOREST: CardRecord = CardRecord::new(
    "Snow-Covered Forest",
    "4c0ad95c-d62c-4138-ada0-fa39a63a449e",
    "Pat Lewis",
    CardRules::new_land(&["Forest"])
        .with_supertype(CardSupertype::Basic)
        .with_supertype(CardSupertype::Snow),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ADARKAR_UNICORN,
    &ARCTIC_FOXES,
    &ARENSON_S_AURA,
    &ARMOR_OF_FAITH,
    &BATTLE_CRY,
    &BLACK_SCARAB,
    &BLESSED_WINE,
    &BLINKING_SPIRIT,
    &BLUE_SCARAB,
    &CALL_TO_ARMS,
    &CARIBOU_RANGE,
    &COLD_SNAP,
    &COOPERATION,
    &DROUGHT,
    &ELVISH_HEALER,
    &ENDURING_RENEWAL,
    &ENERGY_STORM,
    &FORMATION,
    &FYLGJA,
    &GENERAL_JARKELD,
    &GREEN_SCARAB,
    &HALLOWED_GROUND,
    &HEAL,
    &HIPPARION,
    &JUSTICE,
    &KELSINKO_RANGER,
    &KJELDORAN_ELITE_GUARD,
    &KJELDORAN_GUARD,
    &KJELDORAN_KNIGHT,
    &KJELDORAN_PHALANX,
    &KJELDORAN_ROYAL_GUARD,
    &KJELDORAN_SKYCAPTAIN,
    &KJELDORAN_SKYKNIGHT,
    &KJELDORAN_WARRIOR,
    &LIGHTNING_BLOW,
    &LOST_ORDER_OF_JARKELD,
    &MERCENARIES,
    &ORDER_OF_THE_SACRED_TORCH,
    &ORDER_OF_THE_WHITE_SHIELD,
    &PRISMATIC_WARD,
    &RALLY,
    &RED_SCARAB,
    &SACRED_BOON,
    &SERAPH,
    &SHIELD_BEARER,
    &SNOW_HOUND,
    &WARNING,
    &WHITE_SCARAB,
    &ARNJLOT_S_ASCENT,
    &BALDUVIAN_CONJURER,
    &BALDUVIAN_SHAMAN,
    &BINDING_GRASP,
    &BRAINSTORM,
    &BREATH_OF_DREAMS,
    &CLAIRVOYANCE,
    &DEFLECTION,
    &DREAMS_OF_THE_DEAD,
    &ENERVATE,
    &ERRANT_MINION,
    &ESSENCE_FLARE,
    &FORCE_VOID,
    &GLACIAL_WALL,
    &HYDROBLAST,
    &ICEBERG,
    &ICY_PRISON,
    &ILLUSIONARY_FORCES,
    &ILLUSIONARY_PRESENCE,
    &ILLUSIONARY_TERRAIN,
    &ILLUSIONARY_WALL,
    &ILLUSIONS_OF_GRANDEUR,
    &INFUSE,
    &KROVIKAN_SORCERER,
    &MAGUS_OF_THE_UNSEEN,
    &MESMERIC_TRANCE,
    &MISTFOLK,
    &MUSICIAN,
    &MYSTIC_MIGHT,
    &MYSTIC_REMORA,
    &PHANTASMAL_MOUNT,
    &POLAR_KRAKEN,
    &PORTENT,
    &RAY_OF_COMMAND,
    &RAY_OF_ERASURE,
    &REALITY_TWIST,
    &SEA_SPIRIT,
    &SHYFT,
    &SIBILANT_SPIRIT,
    &SILVER_ERNE,
    &SNOW_DEVIL,
    &SNOWFALL,
    &SOLDEVI_MACHINIST,
    &SOUL_BARRIER,
    &THUNDER_WALL,
    &UPDRAFT,
    &WIND_SPIRIT,
    &WINTER_S_CHILL,
    &WORD_OF_UNDOING,
    &WRATH_OF_MARIT_LAGE,
    &ZUR_S_WEIRDING,
    &ZURAN_ENCHANTER,
    &ZURAN_SPELLCASTER,
    &ABYSSAL_SPECTER,
    &ASHEN_GHOUL,
    &BRINE_SHAMAN,
    &BURNT_OFFERING,
    &CLOAK_OF_CONFUSION,
    &DANCE_OF_THE_DEAD,
    &DARK_BANISHING,
    &DEMONIC_CONSULTATION,
    &DREAD_WIGHT,
    &DRIFT_OF_THE_DEAD,
    &FLOW_OF_MAGGOTS,
    &FOUL_FAMILIAR,
    &GANGRENOUS_ZOMBIES,
    &GAZE_OF_PAIN,
    &GRAVEBIND,
    &HECATOMB,
    &HOAR_SHADE,
    &HYALOPTEROUS_LEMURE,
    &ICEQUAKE,
    &INFERNAL_DARKNESS,
    &INFERNAL_DENIZEN,
    &KJELDORAN_DEAD,
    &KNIGHT_OF_STROMGALD,
    &KROVIKAN_ELEMENTALIST,
    &KROVIKAN_FETISH,
    &KROVIKAN_VAMPIRE,
    &LEGIONS_OF_LIM_DUL,
    &LESHRAC_S_RITE,
    &LESHRAC_S_SIGIL,
    &LIM_DUL_S_COHORT,
    &LIM_DUL_S_HEX,
    &MIND_RAVEL,
    &MIND_WARP,
    &MIND_WHIP,
    &MINION_OF_LESHRAC,
    &MINION_OF_TEVESH_SZAT,
    &MOLE_WORMS,
    &MOOR_FIEND,
    &NECROPOTENCE,
    &NORRITT,
    &OATH_OF_LIM_DUL,
    &PESTILENCE_RATS,
    &POX,
    &SEIZURES,
    &SONGS_OF_THE_DAMNED,
    &SOUL_BURN,
    &SOUL_KISS,
    &SPOILS_OF_EVIL,
    &SPOILS_OF_WAR,
    &STENCH_OF_EVIL,
    &STROMGALD_CABAL,
    &TOUCH_OF_DEATH,
    &WITHERING_WISPS,
    &AGGRESSION,
    &ANARCHY,
    &AVALANCHE,
    &BALDUVIAN_BARBARIANS,
    &BALDUVIAN_HYDRA,
    &BARBARIAN_GUIDES,
    &BATTLE_FRENZY,
    &BONE_SHAMAN,
    &BRAND_OF_ILL_OMEN,
    &CHAOS_LORD,
    &CHAOS_MOON,
    &CONQUER,
    &CURSE_OF_MARIT_LAGE,
    &DWARVEN_ARMORY,
    &ERRANTRY,
    &FLAME_SPIRIT,
    &FLARE,
    &GAME_OF_CHAOS,
    &GLACIAL_CREVASSES,
    &GOBLIN_MUTANT,
    &GOBLIN_SAPPERS,
    &GOBLIN_SKI_PATROL,
    &GOBLIN_SNOWMAN,
    &GRIZZLED_WOLVERINE,
    &IMPOSING_VISAGE,
    &INCINERATE,
    &JOKULHAUPS,
    &KARPLUSAN_GIANT,
    &KARPLUSAN_YETI,
    &LAVA_BURST,
    &MARTON_STROMGALD,
    &MELEE,
    &MELTING,
    &METEOR_SHOWER,
    &MOUNTAIN_GOAT,
    &MUDSLIDE,
    &ORCISH_CANNONEERS,
    &ORCISH_CONSCRIPTS,
    &ORCISH_FARMER,
    &ORCISH_HEALER,
    &ORCISH_LIBRARIAN,
    &ORCISH_LUMBERJACK,
    &ORCISH_SQUATTERS,
    &PANIC,
    &PYROBLAST,
    &PYROCLASM,
    &SABRETOOTH_TIGER,
    &STONE_SPIRIT,
    &STONEHANDS,
    &TOR_GIANT,
    &TOTAL_WAR,
    &VERTIGO,
    &WALL_OF_LAVA,
    &WORD_OF_BLASTING,
    &AUROCHS,
    &BALDUVIAN_BEARS,
    &BLIZZARD,
    &BROWN_OUPHE,
    &CHUB_TOAD,
    &DIRE_WOLVES,
    &EARTHLORE,
    &ELDER_DRUID,
    &ESSENCE_FILTER,
    &FANATICAL_FEVER,
    &FOLK_OF_THE_PINES,
    &FORBIDDEN_LORE,
    &FORGOTTEN_LORE,
    &FOXFIRE,
    &FREYALISE_SUPPLICANT,
    &FREYALISE_S_CHARM,
    &FREYALISE_S_WINDS,
    &FYNDHORN_BROWNIE,
    &FYNDHORN_ELDER,
    &FYNDHORN_ELVES,
    &FYNDHORN_POLLEN,
    &GORILLA_PACK,
    &HOT_SPRINGS,
    &JOHTULL_WURM,
    &JUNIPER_ORDER_DRUID,
    &LHURGOYF,
    &MADDENING_WIND,
    &NATURE_S_LORE,
    &PALE_BEARS,
    &PYGMY_ALLOSAURUS,
    &PYKNITE,
    &RIME_DRYAD,
    &RITUAL_OF_SUBDUAL,
    &SCALED_WURM,
    &SHAMBLING_STRIDER,
    &SNOWBLIND,
    &STAMPEDE,
    &STUNTED_GROWTH,
    &TARPAN,
    &THERMOKARST,
    &THOUGHTLEECH,
    &TINDER_WALL,
    &TOUCH_OF_VITAE,
    &TRAILBLAZER,
    &VENOMOUS_BREATH,
    &WALL_OF_PINE_NEEDLES,
    &WHITEOUT,
    &WIITIGO,
    &WOOLLY_MAMMOTHS,
    &WOOLLY_SPIDER,
    &YAVIMAYA_GNATS,
    &ALTAR_OF_BONE,
    &CENTAUR_ARCHER,
    &CHROMATIC_ARMOR,
    &DIABOLIC_VISION,
    &EARTHLINK,
    &ELEMENTAL_AUGURY,
    &ESSENCE_VORTEX,
    &FIERY_JUSTICE,
    &FIRE_COVENANT,
    &FLOODED_WOODLANDS,
    &FUMAROLE,
    &GHOSTLY_FLAME,
    &GIANT_TRAP_DOOR_SPIDER,
    &GLACIERS,
    &HYMN_OF_REBIRTH,
    &KJELDORAN_FROSTBEAST,
    &MERIEKE_RI_BERIT,
    &MONSOON,
    &MOUNTAIN_TITAN,
    &RECLAMATION,
    &SKELETON_SHIP,
    &SPECTRAL_SHIELD,
    &STORM_SPIRIT,
    &STORMBIND,
    &WINGS_OF_AESTHIR,
    &ADARKAR_SENTINEL,
    &AEGIS_OF_THE_MEEK,
    &AMULET_OF_QUOZ,
    &ARCUM_S_SLEIGH,
    &ARCUM_S_WEATHERVANE,
    &ARCUM_S_WHISTLE,
    &BARBED_SEXTANT,
    &BATON_OF_MORALE,
    &CELESTIAL_SWORD,
    &CROWN_OF_THE_AGES,
    &DESPOTIC_SCEPTER,
    &ELKIN_BOTTLE,
    &FYNDHORN_BOW,
    &GOBLIN_LYRE,
    &HEMATITE_TALISMAN,
    &ICE_CAULDRON,
    &INFINITE_HOURGLASS,
    &JESTER_S_CAP,
    &JESTER_S_MASK,
    &JEWELED_AMULET,
    &LAPIS_LAZULI_TALISMAN,
    &MALACHITE_TALISMAN,
    &NACRE_TALISMAN,
    &NAKED_SINGULARITY,
    &ONYX_TALISMAN,
    &PENTAGRAM_OF_THE_AGES,
    &PIT_TRAP,
    &RUNED_ARCH,
    &SHIELD_OF_THE_AGES,
    &SKULL_CATAPULT,
    &SNOW_FORTRESS,
    &SOLDEVI_GOLEM,
    &SOLDEVI_SIMULACRUM,
    &STAFF_OF_THE_AGES,
    &SUNSTONE,
    &TIME_BOMB,
    &URZAS_BAUBLE,
    &VEXING_ARCANIX,
    &VIBRATING_SPHERE,
    &WALKING_WALL,
    &WALL_OF_SHIELDS,
    &WAR_CHARIOT,
    &WHALEBONE_GLIDER,
    &ZURAN_ORB,
    &ADARKAR_WASTES,
    &BRUSHLAND,
    &GLACIAL_CHASM,
    &HALLS_OF_MIST,
    &ICE_FLOE,
    &KARPLUSAN_FOREST,
    &LAND_CAP,
    &LAVA_TUBES,
    &RIVER_DELTA,
    &SULFUROUS_SPRINGS,
    &TIMBERLINE_RIDGE,
    &UNDERGROUND_RIVER,
    &VELDT,
    &SNOW_COVERED_PLAINS,
    &SNOW_COVERED_ISLAND,
    &SNOW_COVERED_SWAMP,
    &SNOW_COVERED_MOUNTAIN,
    &SNOW_COVERED_FOREST,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    CIRCLE_OF_PROTECTION_BLACK_REPRINT,
    CIRCLE_OF_PROTECTION_BLUE_REPRINT,
    CIRCLE_OF_PROTECTION_GREEN_REPRINT,
    CIRCLE_OF_PROTECTION_RED_REPRINT,
    CIRCLE_OF_PROTECTION_WHITE_REPRINT,
    DEATH_WARD_REPRINT,
    DISENCHANT_REPRINT,
    SWORDS_TO_PLOWSHARES_REPRINT,
    COUNTERSPELL_REPRINT,
    POWER_SINK_REPRINT,
    SLEIGHT_OF_MIND_REPRINT,
    DARK_RITUAL_REPRINT,
    FEAR_REPRINT,
    HOWL_FROM_BEYOND_REPRINT,
    SHATTER_REPRINT,
    STONE_RAIN_REPRINT,
    GIANT_GROWTH_REPRINT,
    HURRICANE_REPRINT,
    LURE_REPRINT,
    REGENERATION_REPRINT,
    WILD_GROWTH_REPRINT,
    ICY_MANIPULATOR_REPRINT,
    PLAINS_REPRINT,
    PLAINS_ALTERNATE_1,
    PLAINS_ALTERNATE_2,
    ISLAND_REPRINT,
    ISLAND_ALTERNATE_1,
    ISLAND_ALTERNATE_2,
    SWAMP_REPRINT,
    SWAMP_ALTERNATE_1,
    SWAMP_ALTERNATE_2,
    MOUNTAIN_REPRINT,
    MOUNTAIN_ALTERNATE_1,
    MOUNTAIN_ALTERNATE_2,
    FOREST_REPRINT,
    FOREST_ALTERNATE_1,
    FOREST_ALTERNATE_2,
];
