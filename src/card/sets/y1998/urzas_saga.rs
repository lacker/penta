//! Urza's Saga cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::DamageEventMatcherDef;
use crate::DamageKindDef;
use crate::DamageRecipientMatcherDef;
use crate::DamageSourceMatcherDef;
use crate::card::ScaledValueDef;
use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y1994::legends as catalog_leg;
use crate::card::sets::y1995::ice_age as catalog_ice;
use crate::card::sets::y1998::portal_second_age as catalog_p02;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AppliedEffectDef, AppliedRuleDef, BasicLandType, CardChoiceSourceDef, CardRules, CardSet,
    CardSupertype, CardType, ChoiceVisibilityDef, ChooseDef, CounterKind, DiscardSelectionDef,
    EffectDef, EffectRecipientDef, GraveyardPlayPermissionDef, InstalledTriggerDef, ManaColor,
    ObjectChoiceBindingDef, ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef,
    OngoingEffectDef, PlayActionMatcherDef, PlayRestrictionDef, PlayerRefDef, PlayerRelation,
    PlayerSetDef, ReplacementEffectDef, ReplacementEventDef, ResolvedEffectDurationDef,
    SpellResolutionDestinationDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, ZonePlacement,
    abilities,
};
use crate::ids::ParentBinding;
use crate::{TargetIndex, mana_cost};

// USG 1 — Absolute Grace
pub(in crate::card::sets) static ABSOLUTE_GRACE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Absolute Grace",
    "fe7d1839-7180-4b4c-8ddb-7df24573f740",
    "Jeff Miracola",
    CardRules::new_enchantment(mana_cost!("{1}{W}")).with_ability(AbilityDef::static_ability(
        "All creatures have protection from black.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::add_ability(&abilities::protection_from_color(
                ManaColor::Black,
            )),
        },
    )),
);

// USG 2 — Absolute Law
pub(in crate::card::sets) static ABSOLUTE_LAW: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Absolute Law",
    "59d1f05b-f165-47c7-8a78-3b60ee3298ca",
    "Mark Zug",
    CardRules::new_enchantment(mana_cost!("{1}{W}")).with_ability(AbilityDef::static_ability(
        "All creatures have protection from red.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::add_ability(&abilities::protection_from_color(
                ManaColor::Red,
            )),
        },
    )),
);

// USG 3 — Angelic Chorus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANGELIC_CHORUS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Angelic Chorus",
    "907bf221-a1bf-41ab-9b7e-e5a64c385642",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// USG 4 — Angelic Page
pub(in crate::card::sets) static ANGELIC_PAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Angelic Page",
    "f50a4378-8b14-484c-b285-09cc4c4e1b3c",
    "Rebecca Guay",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Angel", "Spirit"], 1, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated_with_targets(
            "{T}: Target attacking or blocking creature gets +1/+1 until end of turn.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::AttackingOrBlocking,
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// USG 5 — Brilliant Halo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRILLIANT_HALO: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Brilliant Halo",
    "3d454961-1ab3-442b-935d-68c25b56aea0",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// USG 6 — Catastrophe
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CATASTROPHE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Catastrophe",
    "294d21dc-5c76-4449-936f-9b7541d37c86",
    "Andrew Robinson",
    crate::card::CardRules::unsupported(),
);

// USG 7 — Clear
pub(in crate::card::sets) static CLEAR: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Clear",
    "c7cdb977-7d5b-4050-bb01-181f6b363de7",
    "Andrew Robinson",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[
        AbilityDef::destroy_target(
            "Destroy target enchantment.",
            &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                CardType::Enchantment,
            )),
            true,
        ),
        abilities::cycling(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            mana_cost!("{2}"),
        ),
    ]),
);

// USG 8 — Congregate
pub(in crate::card::sets) static CONGREGATE: CardRecord = CardRecord::new(
    CardSet::UrzasSaga,
    "Congregate",
    "80b7923b-eb1c-49ce-8250-a1ea6efbb56e",
    "Mark Zug",
    CardRules::new_instant(mana_cost!("{3}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Target player gains 2 life for each creature on the battlefield.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Scaled(
                &// Every creature on the battlefield, both sides included: the card says "on
                // the battlefield" rather than "you control".
                ScaledValueDef::new(ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                )), 2),
            ),
        },
    )),
);

// USG 9 — Defensive Formation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEFENSIVE_FORMATION: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Defensive Formation",
    "fdd76db0-1a67-4965-81a5-1d8d86b63971",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// USG 10 — Disciple of Grace
pub(in crate::card::sets) static DISCIPLE_OF_GRACE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Disciple of Grace",
    "83fa36d2-0a60-40a5-a182-a63e1e65b2bd",
    "Robh Ruppel",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Cleric"], 1, 2).with_abilities(&[
        abilities::protection_from_color(ManaColor::Black),
        abilities::cycling(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            mana_cost!("{2}"),
        ),
    ]),
);

// USG 11 — Disciple of Law
pub(in crate::card::sets) static DISCIPLE_OF_LAW: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Disciple of Law",
    "7a5c8701-a294-4474-9747-129f972cfb18",
    "Matthew D. Wilson",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Cleric"], 1, 2).with_abilities(&[
        abilities::protection_from_color(ManaColor::Red),
        abilities::cycling(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            mana_cost!("{2}"),
        ),
    ]),
);

// USG 12 — Disenchant (reprint)
const DISENCHANT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DISENCHANT,
    "1da67d77-1cbd-4f0e-a109-87fb4c84bcca",
    "Donato Giancola",
);

// USG 13 — Elite Archers
pub(in crate::card::sets) static ELITE_ARCHERS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Elite Archers",
    "6d7ee95c-3ce8-4b8c-a1a7-1caa5b8a3cc9",
    "Greg Staples",
    CardRules::new_creature(mana_cost!("{5}{W}"), &["Human", "Soldier", "Archer"], 3, 3)
        .with_ability(AbilityDef::activated_with_targets(
            "{T}: This creature deals 3 damage to target attacking or blocking creature.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::AttackingOrBlocking,
                ]),
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
            },
        )),
);

// USG 14 — Faith Healer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FAITH_HEALER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Faith Healer",
    "5d51ff6a-ba1b-4015-8d0b-df1a1bb5a0c1",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// USG 15 — Glorious Anthem
pub(in crate::card::sets) static GLORIOUS_ANTHEM: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Glorious Anthem",
    "61f867c5-0727-4408-b479-b81518daa0ec",
    "Kev Walker",
    CardRules::new_enchantment(mana_cost!("{1}{W}{W}")).with_ability(AbilityDef::static_ability(
        "Creatures you control get +1/+1.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(1),
                ValueDef::Constant(1),
            ),
        },
    )),
);

// USG 16 — Healing Salve (reprint)
const HEALING_SALVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::HEALING_SALVE,
    "89146f6a-583f-4ed1-8b43-75e9a55892c6",
    "Heather Hudson",
);

// USG 17 — Herald of Serra
pub(in crate::card::sets) static HERALD_OF_SERRA: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Herald of Serra",
    "22a2b882-d616-495e-99f6-196031235f93",
    "Matthew D. Wilson",
    CardRules::new_creature(mana_cost!("{2}{W}{W}"), &["Angel"], 3, 4).with_abilities(&[
        abilities::flying(),
        abilities::vigilance(),
        abilities::echo(
            "Echo {2}{W}{W} (At the beginning of your upkeep, if this came under your control since the beginning of your last upkeep, sacrifice it unless you pay its echo cost.)",
            mana_cost!("{2}{W}{W}"),
        ),
    ]),
);

// USG 18 — Humble
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HUMBLE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Humble",
    "01383c7f-685f-4e77-a143-8418fe1fe436",
    "Val Mayerik",
    crate::card::CardRules::unsupported(),
);

// USG 19 — Intrepid Hero
pub(in crate::card::sets) static INTREPID_HERO: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Intrepid Hero",
    "0f2a5c67-f76a-4021-959e-3e084a06b80f",
    "Brian Snõddy",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Soldier"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Destroy target creature with power 4 or greater.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::PowerAtLeast(4),
                ]),
            )],
            EffectDef::destroy_target(TargetIndex::PRIMARY, true),
        ),
    ),
);

// USG 20 — Monk Idealist
pub(in crate::card::sets) static MONK_IDEALIST: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Monk Idealist",
    "285a867b-f82e-49cb-a59c-31a25129baf9",
    "Daren Bader",
    CardRules::new_creature(
        mana_cost!("{2}{W}"),
        &["Human", "Monk", "Cleric"],
        2,
        2,
    )
    .with_ability(abilities::enters_trigger_with_targets(
        "When this creature enters, return target enchantment card from your graveyard to your hand.",
        &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::HasType(CardType::Enchantment),
            zones: &[ZoneKind::Graveyard],
            controller: None,
            owner: Some(PlayerRelation::You),
        })],
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Hand,
            placement: ZonePlacement::Top,
        },
    )),
);

// USG 21 — Monk Realist
pub(in crate::card::sets) static MONK_REALIST: CardRecord = CardRecord::new(
    CardSet::UrzasSaga,
    "Monk Realist",
    "7a7fe9f1-f3c0-43e4-aa30-d0bdab4ae94d",
    "Daren Bader",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Monk", "Cleric"], 1, 1).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, destroy target enchantment.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Enchantment),
            )],
            EffectDef::destroy_target(TargetIndex::PRIMARY, true),
        ),
    ),
);

// USG 22 — Opal Acrolith
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OPAL_ACROLITH: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Opal Acrolith",
    "839b4c10-f68f-4321-82ee-5ec257f63866",
    "Robh Ruppel",
    crate::card::CardRules::unsupported(),
);

// USG 23 — Opal Archangel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OPAL_ARCHANGEL: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Opal Archangel",
    "a75fca33-fa06-4385-866c-5d463ae6aaf6",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// USG 24 — Opal Caryatid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OPAL_CARYATID: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Opal Caryatid",
    "3a8a2e24-c959-40a2-883d-c0114589cfe7",
    "Berry",
    crate::card::CardRules::unsupported(),
);

// USG 25 — Opal Gargoyle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OPAL_GARGOYLE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Opal Gargoyle",
    "a8a67943-8f07-445b-a84c-893879dae7ca",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// USG 26 — Opal Titan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OPAL_TITAN: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Opal Titan",
    "379f1f01-88c6-4cc2-9049-078aa6980582",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// USG 27 — Pacifism (reprint)
const PACIFISM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1996::mirage::PACIFISM,
    "dc81638f-a74c-47fc-825a-ae778c524f66",
    "Randy Gallegos",
);

// USG 28 — Pariah
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PARIAH: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Pariah",
    "0447f9e1-792b-4200-9ef3-7cd95c326b88",
    "Jon J Muth",
    crate::card::CardRules::unsupported(),
);

// USG 29 — Path of Peace (reprint)
const PATH_OF_PEACE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::PATH_OF_PEACE,
    "af7a2719-7910-4601-be88-7b3c249199d3",
    "Val Mayerik",
);

// USG 30 — Pegasus Charger
pub(in crate::card::sets) static PEGASUS_CHARGER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Pegasus Charger",
    "d62a5287-25ec-4e13-9e39-1c87a4052c4d",
    "Val Mayerik",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Pegasus"], 2, 1)
        .with_abilities(&[abilities::flying(), abilities::first_strike()]),
);

// USG 31 — Planar Birth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PLANAR_BIRTH: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Planar Birth",
    "c7cacdff-aa83-4644-b2f0-ce8c89dddfbf",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// USG 32 — Presence of the Master (reprint)
const PRESENCE_OF_THE_MASTER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1994::legends::PRESENCE_OF_THE_MASTER,
    "849adb29-61ad-4307-98b9-61e33aec6500",
    "Ciruelo",
);

// USG 33 — Redeem
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REDEEM: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Redeem",
    "05a7756d-df25-4969-96ad-b006df09788b",
    "D. Alexander Gregory",
    crate::card::CardRules::unsupported(),
);

// USG 34 — Remembrance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REMEMBRANCE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Remembrance",
    "556d334e-3ac9-45b0-98d2-aff49ada0f75",
    "Val Mayerik",
    crate::card::CardRules::unsupported(),
);

// USG 35 — Rune of Protection: Artifacts
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUNE_OF_PROTECTION_ARTIFACTS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Rune of Protection: Artifacts",
    "e18cce33-92be-4189-9f99-cb47bd617fd2",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// USG 36 — Rune of Protection: Black
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUNE_OF_PROTECTION_BLACK: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Rune of Protection: Black",
    "3f69050b-c54d-43f2-8348-2801c365dc4c",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// USG 37 — Rune of Protection: Blue
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUNE_OF_PROTECTION_BLUE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Rune of Protection: Blue",
    "85f4ed3a-1851-49b1-baac-fdc8c00b6b71",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// USG 38 — Rune of Protection: Green
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUNE_OF_PROTECTION_GREEN: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Rune of Protection: Green",
    "905712b2-3177-4935-bca1-6990439b8d78",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// USG 39 — Rune of Protection: Lands
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUNE_OF_PROTECTION_LANDS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Rune of Protection: Lands",
    "4e874700-8002-41e7-8861-b4ce29ba6d9e",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// USG 40 — Rune of Protection: Red
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUNE_OF_PROTECTION_RED: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Rune of Protection: Red",
    "2916023b-cf67-443f-9ac7-f03313f9d3b7",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// USG 41 — Rune of Protection: White
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUNE_OF_PROTECTION_WHITE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Rune of Protection: White",
    "6408417e-aca3-43f3-9eea-fed5a402d8ab",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// USG 42 — Sanctum Custodian
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SANCTUM_CUSTODIAN: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Sanctum Custodian",
    "27a64b98-9002-48fe-a3e5-4449050c87e1",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// USG 43 — Sanctum Guardian
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SANCTUM_GUARDIAN: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Sanctum Guardian",
    "0d09137f-0f4c-4389-a915-0ce02c833d94",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// USG 44 — Seasoned Marshal (reprint)
const SEASONED_MARSHAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::SEASONED_MARSHAL,
    "9de20845-06b7-4542-8d61-4b97309669f9",
    "Matthew D. Wilson",
);

// USG 45 — Serra Avatar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SERRA_AVATAR: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Serra Avatar",
    "288b0976-78e8-4fbe-8607-2e55d8761d3e",
    "Dermot Power",
    crate::card::CardRules::unsupported(),
);

// USG 46 — Serra Zealot
pub(in crate::card::sets) static SERRA_ZEALOT: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Serra Zealot",
    "0b311542-599f-4d2f-a871-18d5b0b7bbe5",
    "DiTerlizzi",
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Soldier"], 1, 1)
        .with_ability(abilities::first_strike()),
);

// USG 47 — Serra's Embrace
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SERRA_S_EMBRACE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Serra's Embrace",
    "145c3ebd-7a67-4606-8427-f3b91ab26b84",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// USG 48 — Serra's Hymn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SERRA_S_HYMN: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Serra's Hymn",
    "74b8205a-608e-4274-a32c-802a7ce52d9c",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// USG 49 — Serra's Liturgy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SERRA_S_LITURGY: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Serra's Liturgy",
    "b84be3ef-a820-41e3-b66a-09303dad32dd",
    "rk post",
    crate::card::CardRules::unsupported(),
);

// USG 50 — Shimmering Barrier
pub(in crate::card::sets) static SHIMMERING_BARRIER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Shimmering Barrier",
    "2aa7158a-5c00-4969-a116-c40cefdf4591",
    "D. Alexander Gregory",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Wall"], 1, 3).with_abilities(&[
        abilities::defender(),
        abilities::first_strike(),
        abilities::cycling(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            mana_cost!("{2}"),
        ),
    ]),
);

// USG 51 — Silent Attendant
pub(in crate::card::sets) static SILENT_ATTENDANT: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Silent Attendant",
    "56e90087-3738-40df-929b-d2f880264b55",
    "Rebecca Guay",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Cleric"], 0, 2).with_ability(
        AbilityDef::activated(
            "{T}: You gain 1 life.",
            &[AbilityCostDef::TapSource],
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// USG 52 — Songstitcher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SONGSTITCHER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Songstitcher",
    "fa8d306f-3f4e-4c21-9461-caa3daf4fc50",
    "Berry",
    crate::card::CardRules::unsupported(),
);

// USG 53 — Soul Sculptor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOUL_SCULPTOR: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Soul Sculptor",
    "fe6ef073-1c83-47a4-b19d-86fb8bed5db9",
    "Ciruelo",
    crate::card::CardRules::unsupported(),
);

// USG 54 — Voice of Grace
pub(in crate::card::sets) static VOICE_OF_GRACE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Voice of Grace",
    "72e8eb3b-3ebf-426c-8dc8-138ec9b7c671",
    "Jeff Miracola",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Angel"], 2, 2).with_abilities(&[
        abilities::flying(),
        abilities::protection_from_color(ManaColor::Black),
    ]),
);

// USG 55 — Voice of Law
pub(in crate::card::sets) static VOICE_OF_LAW: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Voice of Law",
    "daec52a4-02da-4bff-aff4-5247baed1326",
    "Mark Zug",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Angel"], 2, 2).with_abilities(&[
        abilities::flying(),
        abilities::protection_from_color(ManaColor::Red),
    ]),
);

// USG 56 — Waylay
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WAYLAY: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Waylay",
    "867a33ee-0340-413a-8243-9d6bc2d944e2",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// USG 57 — Worship
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WORSHIP: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Worship",
    "908781a0-1ba4-4027-bd9d-13f9faf08686",
    "Mark Zug",
    crate::card::CardRules::unsupported(),
);

// USG 58 — Academy Researchers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ACADEMY_RESEARCHERS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Academy Researchers",
    "6ca58c8e-9f40-4ed8-a3ed-a01fe67c600d",
    "Stephen Daniele",
    crate::card::CardRules::unsupported(),
);

// USG 59 — Annul
pub(in crate::card::sets) static ANNUL: CardRecord = CardRecord::new(
    CardSet::UrzasSaga,
    "Annul",
    "3f8c73ff-be92-41ca-93a7-76f9823adb38",
    "Greg Simanson",
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::counter_target(
        "Counter target artifact or enchantment spell.",
        &AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::Spell,
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                ]),
            ]),
            zones: &[ZoneKind::Stack],
            controller: None,
            owner: None,
        }),
    )),
);

// USG 60 — Arcane Laboratory
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARCANE_LABORATORY: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Arcane Laboratory",
    "2f4a88e8-aab0-488a-a0b8-fa3feedbf278",
    "Stephen Daniele",
    crate::card::CardRules::unsupported(),
);

// USG 61 — Attunement
pub(in crate::card::sets) static ATTUNEMENT: CardRecord = CardRecord::new(
    CardSet::UrzasSaga,
    "Attunement",
    "f6723528-8b2c-4beb-a465-800300faf158",
    "Randy Gallegos",
    // A net card down every time, and that is the point: the deck wants the
    // graveyard, and the enchantment comes back to do it again.
    CardRules::new_enchantment(mana_cost!("{2}{U}")).with_ability(AbilityDef::activated(
        "Return this enchantment to its owner's hand: Draw three cards, then discard four cards.",
        &[AbilityCostDef::ReturnSourceToHand],
        EffectDef::Sequence(&[
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
            EffectDef::Discard {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(4),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ]),
    )),
);

// USG 62 — Back to Basics
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BACK_TO_BASICS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Back to Basics",
    "fab4cd7e-b56f-4408-a0e9-c07e040cc38f",
    "Andrew Robinson",
    crate::card::CardRules::unsupported(),
);

// USG 63 — Barrin, Master Wizard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARRIN_MASTER_WIZARD: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Barrin, Master Wizard",
    "ec79e35f-9e78-462d-8b71-4f044e2eff90",
    "Michael Sutfin",
    crate::card::CardRules::unsupported(),
);

// USG 64 — Catalog
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CATALOG: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Catalog",
    "31bdef28-0e27-4c8d-a04c-5413519dcb4e",
    "Berry",
    crate::card::CardRules::unsupported(),
);

// USG 65 — Cloak of Mists
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLOAK_OF_MISTS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Cloak of Mists",
    "dd54251c-5a2e-48e4-9790-a64dcc44eb8e",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// USG 66 — Confiscate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CONFISCATE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Confiscate",
    "7cba6d4a-58d0-42d6-b49b-65c72b86007f",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// USG 67 — Coral Merfolk
pub(in crate::card::sets) static CORAL_MERFOLK: CardRecord = CardRecord::new(
    CardSet::UrzasSaga,
    "Coral Merfolk",
    "837d82a9-aef8-4079-a485-05ada1b66322",
    "rk post",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Merfolk"], 2, 1),
);

// USG 68 — Curfew
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CURFEW: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Curfew",
    "49ee9af3-d61c-4964-88a6-6e8ad6a6a29a",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// USG 69 — Disruptive Student
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DISRUPTIVE_STUDENT: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Disruptive Student",
    "ee43681d-e0f7-422b-a363-0d630f68d363",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// USG 70 — Douse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DOUSE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Douse",
    "93a8d857-184d-4339-88f4-261378e5bd3c",
    "Val Mayerik",
    crate::card::CardRules::unsupported(),
);

// USG 71 — Drifting Djinn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRIFTING_DJINN: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Drifting Djinn",
    "971d0eda-91b8-48f2-a988-016bcc7ab35e",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// USG 72 — Enchantment Alteration (reprint)
const ENCHANTMENT_ALTERATION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_leg::ENCHANTMENT_ALTERATION,
    "254aa8d0-f0f5-4fb2-a6ba-07453d71e229",
    "D. Alexander Gregory",
);

// USG 73 — Energy Field
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENERGY_FIELD: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Energy Field",
    "81ff5770-b207-41e1-97b7-b9347c72b407",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// USG 74 — Exhaustion (reprint)
const EXHAUSTION_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::EXHAUSTION,
    "666efc89-b566-4b2b-a0e2-52f1dedc9e10",
    "Paolo Parente",
);

// USG 75 — Fog Bank
pub(in crate::card::sets) static FOG_BANK: CardRecord = CardRecord::new(
    CardSet::UrzasSaga,
    "Fog Bank",
    "6ade0d30-5a57-439e-95e8-5f865880031f",
    "Scott Kirschner",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Wall"], 0, 2).with_abilities(&[
        abilities::defender(),
        abilities::flying(),
        AbilityDef::static_ability(
            "Prevent all combat damage that would be dealt to and dealt by this creature.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                // Both directions of the same clause: nothing it deals lands and nothing
                // dealt to it lands, so it blocks anything and survives, and kills nothing.
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::Rule(AppliedRuleDef::PreventDamage(DamageEventMatcherDef {
                        kind: DamageKindDef::Combat,
                        source: DamageSourceMatcherDef::Any,
                        recipient: DamageRecipientMatcherDef::AffectedObject,
                    })),
                    AppliedEffectDef::Rule(AppliedRuleDef::PreventDamage(DamageEventMatcherDef {
                        kind: DamageKindDef::Combat,
                        source: DamageSourceMatcherDef::AffectedObject,
                        recipient: DamageRecipientMatcherDef::Any,
                    })),
                ]),
            },
        ),
    ]),
);

// USG 76 — Gilded Drake
pub(in crate::card::sets) static GILDED_DRAKE: CardRecord = CardRecord::new(
    CardSet::UrzasSaga,
    "Gilded Drake",
    "8de3fdae-cc2c-4a14-b15b-4fe1a983dfbf",
    "Bob Eggleton",
    // Two mana to take the best creature on the board and hand back a 3/3
    // flier. Against a board with nothing worth taking it simply dies.
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Drake"], 3, 3).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger_with_targets("When this creature enters, exchange control of this creature and up to one target creature an opponent controls. If you don't or can't make an exchange, sacrifice this creature. This ability still resolves if its target becomes illegal.", &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
                1,
            // One effect rather than two control changes: both controllers are read
            // before either permanent moves, and failure runs the printed sacrifice.
            )], EffectDef::ExchangeControl {
                first: EffectRecipientDef::Source,
                second: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                otherwise: Some(&EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                }),
            })
            .resolves_with_illegal_targets(),
    ]),
);

// USG 77 — Great Whale
pub(in crate::card::sets) static GREAT_WHALE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Great Whale",
    "58a2acf1-dad8-4f93-a34e-891e5178a48f",
    "Bob Eggleton",
    CardRules::new_creature(mana_cost!("{5}{U}{U}"), &["Whale"], 5, 5).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, untap up to seven lands.",
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Land),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                7,
            )],
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ),
);

// USG 78 — Hermetic Study
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HERMETIC_STUDY: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Hermetic Study",
    "8321888a-a450-4c15-9461-255cfaa05367",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// USG 79 — Hibernation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HIBERNATION: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Hibernation",
    "68b7444c-fabb-4437-8db9-a1008ea09415",
    "Scott Kirschner",
    crate::card::CardRules::unsupported(),
);

// USG 80 — Horseshoe Crab
pub(in crate::card::sets) static HORSESHOE_CRAB: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Horseshoe Crab",
    "9b33646b-a0e3-4344-873e-6711743bc85c",
    "Scott Kirschner",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Crab"], 1, 3).with_ability(
        AbilityDef::activated(
            "{U}: Untap this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{U}"))],
            EffectDef::Untap {
                object: EffectRecipientDef::Source,
            },
        ),
    ),
);

// USG 81 — Imaginary Pet
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IMAGINARY_PET: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Imaginary Pet",
    "1aa19ecb-146e-4109-b4ef-74675b35d8c4",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// USG 82 — Launch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAUNCH: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Launch",
    "58f78667-b3ab-44af-89df-9e9332dc5485",
    "Val Mayerik",
    crate::card::CardRules::unsupported(),
);

// USG 83 — Lilting Refrain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LILTING_REFRAIN: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Lilting Refrain",
    "ef319154-c5fc-4432-a860-a05508c132d4",
    "Berry",
    crate::card::CardRules::unsupported(),
);

// USG 84 — Lingering Mirage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LINGERING_MIRAGE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Lingering Mirage",
    "050b4a82-a1d5-4dcc-9264-96005fdf53f5",
    "Jerry Tiritilli",
    crate::card::CardRules::unsupported(),
);

// USG 85 — Morphling
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MORPHLING: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Morphling",
    "812f4d5c-aacf-4bd8-849d-80a357a7804d",
    "rk post",
    crate::card::CardRules::unsupported(),
);

// USG 86 — Pendrell Drake
pub(in crate::card::sets) static PENDRELL_DRAKE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Pendrell Drake",
    "986b6708-5ed4-4085-b9b7-d359b2d5b26f",
    "Jeff Miracola",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Drake"], 2, 3).with_abilities(&[
        abilities::flying(),
        abilities::cycling(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            mana_cost!("{2}"),
        ),
    ]),
);

// USG 87 — Pendrell Flux
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PENDRELL_FLUX: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Pendrell Flux",
    "34847e03-c529-415e-9d49-fd4647ca8892",
    "Andrew Robinson",
    crate::card::CardRules::unsupported(),
);

// USG 88 — Peregrine Drake
pub(in crate::card::sets) static PEREGRINE_DRAKE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Peregrine Drake",
    "4951863f-1c16-4d09-ba9a-f57dc3d81a20",
    "Bob Eggleton",
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Drake"], 2, 3).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, untap up to five lands.",
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Land),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                5,
            )],
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

// USG 89 — Power Sink (reprint)
const POWER_SINK_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::POWER_SINK,
    "662cf693-18c4-4169-bcce-09862778f60c",
    "Andrew Robinson",
);

// USG 90 — Power Taint
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static POWER_TAINT: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Power Taint",
    "13d0296c-c0f5-491e-9e61-be8f5af2e631",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// USG 91 — Recantation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RECANTATION: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Recantation",
    "95479839-779e-4dbe-8989-ccf26cc488fe",
    "Greg Simanson",
    crate::card::CardRules::unsupported(),
);

// USG 92 — Rescind
pub(in crate::card::sets) static RESCIND: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Rescind",
    "58dde1dc-8eee-4a66-87d9-fdfb42270744",
    "Adam Rex",
    CardRules::new_instant(mana_cost!("{1}{U}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Return target permanent to its owner's hand.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Any,
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        ),
        abilities::cycling(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            mana_cost!("{2}"),
        ),
    ]),
);

// USG 93 — Rewind
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REWIND: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Rewind",
    "9e51c4fb-fb29-4b1c-b78e-1fadf94fc9a5",
    "Dermot Power",
    crate::card::CardRules::unsupported(),
);

// USG 94 — Sandbar Merfolk
pub(in crate::card::sets) static SANDBAR_MERFOLK: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Sandbar Merfolk",
    "65ce3960-abf1-4f28-8434-ab3b27d3b7cb",
    "rk post",
    CardRules::new_creature(mana_cost!("{U}"), &["Merfolk"], 1, 1).with_ability(
        abilities::cycling(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            mana_cost!("{2}"),
        ),
    ),
);

// USG 95 — Sandbar Serpent
pub(in crate::card::sets) static SANDBAR_SERPENT: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Sandbar Serpent",
    "b3b430ec-28e1-4b2c-bea8-3bfd3a0e8cf8",
    "Jim Nelson",
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Serpent"], 3, 4).with_ability(
        abilities::cycling(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            mana_cost!("{2}"),
        ),
    ),
);

// USG 96 — Show and Tell
pub(in crate::card::sets) static SHOW_AND_TELL: CardRecord = CardRecord::new(
    CardSet::UrzasSaga,
    "Show and Tell",
    "4b851c17-55ed-4671-b471-dc7b34944432",
    "Jeff Laubenstein",
    // Three mana to skip the mana cost of the biggest thing in your deck,
    // and to let them do it too -- which the deck playing it is built to
    // win anyway.
    CardRules::new_sorcery(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell(
        "Each player may put an artifact, creature, enchantment, or land card from their hand \
         onto the battlefield.",
        EffectDef::ChooseCards {
            player: EffectRecipientDef::players(PlayerSetDef::All),
            // Each player looks only at their own hand, and what they put down arrives
            // under their own control. The active player chooses first and the other
            // knows what they chose (CR 101.4a); what this cannot do is land both
            // cards at the same instant, so the first is already a permanent as the
            // second is chosen.
            sources: &[CardChoiceSourceDef::Zone(ZoneKind::Hand)],
            // Everything a permanent card can be except a planeswalker or a battle,
            // which is what the card listed before either existed.
            object: ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::HasType(CardType::Enchantment),
                ObjectPredicateDef::HasType(CardType::Land),
            ]),
            // "May": nobody has to, and a player with nothing it names is
            // never asked.
            minimum: 0,
            maximum: 1,
            reveal: false,
            destination: ZoneKind::Battlefield,
            placement: ZonePlacement::Top,
        },
    )),
);

// USG 97 — Somnophore
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOMNOPHORE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Somnophore",
    "c35aa7a8-5579-46b7-83ef-f5ecc2b31847",
    "Andrew Robinson",
    crate::card::CardRules::unsupported(),
);

// USG 98 — Spire Owl
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIRE_OWL: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Spire Owl",
    "c66b2aa6-e891-4ae5-b6c9-1537b797c3ab",
    "Steve Firchow",
    crate::card::CardRules::unsupported(),
);

// USG 99 — Stern Proctor
pub(in crate::card::sets) static STERN_PROCTOR: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Stern Proctor",
    "7042fdc8-e2dd-4f9a-97b9-00d95c9eae74",
    "Randy Gallegos",
    CardRules::new_creature(mana_cost!("{U}{U}"), &["Human", "Wizard"], 1, 2).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, return target artifact or enchantment to its owner's hand.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                ]),
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        ),
    ),
);

// USG 100 — Stroke of Genius
pub(in crate::card::sets) static STROKE_OF_GENIUS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Stroke of Genius",
    "5e977755-8ea4-4a8b-90c4-dd175321e05d",
    "Stephen Daniele",
    CardRules::new_instant(mana_cost!("{X}{2}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Target player draws X cards.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::DrawCards {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::ChosenX,
        },
    )),
);

// USG 101 — Sunder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUNDER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Sunder",
    "cd9dd7c6-36b6-4fe2-b3d3-f62a6e10a428",
    "Stephen Daniele",
    crate::card::CardRules::unsupported(),
);

// USG 102 — Telepathy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TELEPATHY: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Telepathy",
    "51729f36-0a0c-47fb-a3bf-22afc78df7a4",
    "Matthew D. Wilson",
    crate::card::CardRules::unsupported(),
);

// USG 103 — Time Spiral
pub(in crate::card::sets) static TIME_SPIRAL: CardRecord = CardRecord::new(
    CardSet::UrzasSaga,
    "Time Spiral",
    "f3d62dbd-63db-4ac9-950f-9852627f23f2",
    "Michael Sutfin",
    // Six mana that gives back six, so the wheel is free and the seven new
    // cards arrive with the mana to cast them still up.
    CardRules::new_sorcery(mana_cost!("{4}{U}{U}")).with_ability(
        AbilityDef::spell(
            "Exile Time Spiral. Each player shuffles their hand and graveyard into their \
             library, then draws seven cards. You untap up to six lands.",
            EffectDef::Sequence(&[
                abilities::shuffle_back_and_draw_seven(),
                // "Up to six", and not your own: the lands are chosen as the spell resolves
                // rather than targeted, and nothing in the clause says who controls them.
                // A minimum of none is what "up to" means.
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::Any,
                    )),
                    exclude: None,
                    minimum: 0,
                    maximum: 6,
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::Untap {
                        object: EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding)),
                    },
                }),
            ]),
        )
        // "Exile Time Spiral" is the first thing printed and the last thing
        // that happens: the card is on the stack while the rest resolves, so
        // what the clause settles is where it goes afterwards.
        .with_resolution_destination(SpellResolutionDestinationDef::Exile),
    ),
);

// USG 104 — Tolarian Winds
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOLARIAN_WINDS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Tolarian Winds",
    "5c29399d-0a59-4dcb-9bd4-f31eea3f39f9",
    "Lawrence Snelly",
    crate::card::CardRules::unsupported(),
);

// USG 105 — Turnabout
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TURNABOUT: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Turnabout",
    "52000066-a8cd-418f-98d2-30354b959b32",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// USG 106 — Veil of Birds
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VEIL_OF_BIRDS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Veil of Birds",
    "55bd3fac-44b6-4078-a096-8d47b01ea979",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// USG 107 — Veiled Apparition
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VEILED_APPARITION: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Veiled Apparition",
    "8f9ea09f-5017-423c-84ad-c332329992b3",
    "Andrew Robinson",
    crate::card::CardRules::unsupported(),
);

// USG 108 — Veiled Crocodile
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VEILED_CROCODILE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Veiled Crocodile",
    "9be15ae1-5262-40ba-937c-217a33d131da",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// USG 109 — Veiled Sentry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VEILED_SENTRY: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Veiled Sentry",
    "de458a9d-6c09-42bb-b470-c2691e95345a",
    "Ron Spears",
    crate::card::CardRules::unsupported(),
);

// USG 110 — Veiled Serpent
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VEILED_SERPENT: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Veiled Serpent",
    "25193fd6-3156-48ff-90fa-71328ee7adf5",
    "Bob Eggleton",
    crate::card::CardRules::unsupported(),
);

// USG 111 — Windfall
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WINDFALL: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Windfall",
    "2aef4608-5ba8-4636-b5e7-cac57c5c0608",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// USG 112 — Wizard Mentor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WIZARD_MENTOR: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Wizard Mentor",
    "49805401-9bd9-48a3-9b99-0120a8bb1fb5",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// USG 113 — Zephid
pub(in crate::card::sets) static ZEPHID: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Zephid",
    "e0317fff-dbad-4c47-a191-0369d81cdda2",
    "Daren Bader",
    CardRules::new_creature(mana_cost!("{4}{U}{U}"), &["Illusion"], 3, 4)
        .with_abilities(&[abilities::flying(), abilities::shroud()]),
);

// USG 114 — Zephid's Embrace
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ZEPHID_S_EMBRACE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Zephid's Embrace",
    "fb3312de-c153-4e75-8f8c-b7762b30d492",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// USG 115 — Abyssal Horror
pub(in crate::card::sets) static ABYSSAL_HORROR: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Abyssal Horror",
    "94396d26-cede-4a61-b30a-50aecc730407",
    "rk post",
    CardRules::new_creature(mana_cost!("{4}{B}{B}"), &["Horror"], 2, 2).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, target player discards two cards.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Discard {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ),
    ]),
);

// USG 116 — Befoul
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BEFOUL: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Befoul",
    "f92cb48d-315b-4877-b615-ffdf275c4d61",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// USG 117 — Bereavement
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BEREAVEMENT: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Bereavement",
    "06a1e5f5-a164-4359-bae1-e5dfd4801688",
    "Andrew Goldhawk",
    crate::card::CardRules::unsupported(),
);

// USG 118 — Blood Vassal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLOOD_VASSAL: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Blood Vassal",
    "7e692dea-750b-40b2-9440-8b570e67c23e",
    "Chippy",
    crate::card::CardRules::unsupported(),
);

// USG 119 — Bog Raiders (reprint)
const BOG_RAIDERS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::BOG_RAIDERS,
    "3739188b-f2b3-4ab0-8e5c-b3a1d2a1ad09",
    "Carl Critchlow",
);

// USG 119s — Bog Raiders (alternate printing)
const BOG_RAIDERS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &crate::card::sets::y1997::portal::BOG_RAIDERS,
    1,
    "3c8a8e6f-4c3b-4d92-bd05-9bbb0150d653",
    "Carl Critchlow",
);

// USG 120 — Breach
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BREACH: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Breach",
    "eada28cb-92bf-47e0-b09d-4709be32dbe6",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// USG 121 — Cackling Fiend
pub(in crate::card::sets) static CACKLING_FIEND: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Cackling Fiend",
    "ae410ae8-1e72-4727-96df-c7c195063fb5",
    "Brian Despain",
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Zombie"], 2, 1).with_ability(
        abilities::enters_trigger(
            "When this creature enters, each opponent discards a card.",
            EffectDef::Discard {
                recipient: EffectRecipientDef::players(PlayerSetDef::Related(
                    PlayerRelation::Opponent,
                )),
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ),
    ),
);

// USG 122 — Carrion Beetles
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CARRION_BEETLES: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Carrion Beetles",
    "46d4f4d7-a35b-45ea-ba51-ee65b3ff98d4",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// USG 123 — Contamination
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CONTAMINATION: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Contamination",
    "86067dfe-65c3-4c96-bccd-b3915d6663f9",
    "Stephen Daniele",
    crate::card::CardRules::unsupported(),
);

// USG 124 — Corrupt
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CORRUPT: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Corrupt",
    "32e1c65c-ced6-484d-b3b4-db913c6bf84b",
    "Vincent Evans",
    crate::card::CardRules::unsupported(),
);

// USG 125 — Crazed Skirge
pub(in crate::card::sets) static CRAZED_SKIRGE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Crazed Skirge",
    "816272de-f134-45fa-ac1f-70d35d30c7e1",
    "Ron Spencer",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Phyrexian", "Imp"], 2, 2)
        .with_abilities(&[abilities::flying(), abilities::haste()]),
);

// USG 126 — Dark Hatchling
pub(in crate::card::sets) static DARK_HATCHLING: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Dark Hatchling",
    "87a538c1-8539-4955-ae3e-27312ce9e800",
    "Mark A. Nelson",
    CardRules::new_creature(mana_cost!("{4}{B}{B}"), &["Horror"], 3, 3).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, destroy target nonblack creature. It can't be regenerated.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Color(ManaColor::Black)),
                ]),
            )],
            EffectDef::destroy_target(TargetIndex::PRIMARY, false),
        ),
    ]),
);

// USG 127 — Dark Ritual (reprint)
const DARK_RITUAL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::DARK_RITUAL,
    "3f0e9d0d-e1a3-4e0a-bf39-e9aaf4d36d67",
    "Tom Fleming",
);

// USG 128 — Darkest Hour
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARKEST_HOUR: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Darkest Hour",
    "1ad98ca8-d358-450a-8439-5abf57197b83",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// USG 129 — Despondency
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DESPONDENCY: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Despondency",
    "ef400a6a-628a-40d6-80dc-feabe40d6ce1",
    "D. Alexander Gregory",
    crate::card::CardRules::unsupported(),
);

// USG 130 — Diabolic Servitude
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIABOLIC_SERVITUDE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Diabolic Servitude",
    "0cb1e0c9-9041-44cd-9e96-87cf14c67068",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// USG 130s — Diabolic Servitude (alternate printing)
const DIABOLIC_SERVITUDE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DIABOLIC_SERVITUDE,
    1,
    "d353d6bc-693b-4c2b-9c3b-b0db241ebb42",
    "Scott M. Fischer",
);

// USG 131 — Discordant Dirge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DISCORDANT_DIRGE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Discordant Dirge",
    "d48e753c-ba11-4aa3-9a73-584f8a7538f5",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// USG 132 — Duress
pub(in crate::card::sets) static DURESS: CardRecord = CardRecord::new(
    CardSet::UrzasSaga,
    "Duress",
    "ca367f49-0f4a-4b7f-8104-851893fbcd8a",
    "Lawrence Snelly",
    CardRules::new_sorcery(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target opponent reveals their hand. You choose a noncreature, nonland card from it. That player discards that card.",
        &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
            PlayerRelation::Opponent,
        ))],
        EffectDef::Sequence(&abilities::reveal_hand_and_discard_chosen_card(
            PlayerRefDef::Target(TargetIndex::PRIMARY),
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
            ]),
        )),
    )),
);

// USG 133 — Eastern Paladin
pub(in crate::card::sets) static EASTERN_PALADIN: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Eastern Paladin",
    "f042a8c7-f07b-42bd-8251-c588d890683c",
    "Carl Critchlow",
    CardRules::new_creature(
        mana_cost!("{2}{B}{B}"),
        &["Phyrexian", "Zombie", "Knight"],
        3,
        3,
    )
    .with_ability(AbilityDef::activated_with_targets(
        "{B}{B}, {T}: Destroy target green creature.",
        &[
            AbilityCostDef::Mana(mana_cost!("{B}{B}")),
            AbilityCostDef::TapSource,
        ],
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Color(ManaColor::Green),
            ]),
        )],
        EffectDef::destroy_target(TargetIndex::PRIMARY, true),
    )),
);

// USG 134 — Exhume
pub(in crate::card::sets) static EXHUME: CardRecord = CardRecord::new(
    CardSet::UrzasSaga,
    "Exhume",
    "a88b23ce-ce19-47da-b9f2-055a4d6bdc79",
    "Carl Critchlow",
    // Two mana for the biggest thing anybody has discarded, and the reason
    // the deck playing it discarded something bigger than the other one has.
    CardRules::new_sorcery(mana_cost!("{1}{B}")).with_ability(AbilityDef::spell(
        "Each player puts a creature card from their graveyard onto the battlefield.",
        EffectDef::ChooseCards {
            player: EffectRecipientDef::players(PlayerSetDef::All),
            // Each player looks only at their own graveyard, and what they find arrives
            // under their own control: the choice is asked of each of them in turn
            // rather than made once by the caster.
            sources: &[CardChoiceSourceDef::Zone(ZoneKind::Graveyard)],
            object: ObjectPredicateDef::HasType(CardType::Creature),
            // Not a "may": a player with a creature card down there has to
            // put one back, and one with none is never asked.
            minimum: 1,
            maximum: 1,
            reveal: false,
            destination: ZoneKind::Battlefield,
            placement: ZonePlacement::Top,
        },
    )),
);

// USG 135 — Expunge
pub(in crate::card::sets) static EXPUNGE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Expunge",
    "0576ffe8-a7b9-479b-8ea0-418b430b1aa1",
    "Christopher Moeller",
    CardRules::new_instant(mana_cost!("{2}{B}")).with_abilities(&[
        AbilityDef::destroy_target(
            "Destroy target nonartifact, nonblack creature. It can't be regenerated.",
            &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Artifact)),
                ObjectPredicateDef::Not(&ObjectPredicateDef::Color(ManaColor::Black)),
            ])),
            false,
        ),
        abilities::cycling(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            mana_cost!("{2}"),
        ),
    ]),
);

// USG 136 — Flesh Reaver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLESH_REAVER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Flesh Reaver",
    "e3dc6a91-ca13-45da-ba65-8fbb16c159c0",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// USG 137 — Hollow Dogs
pub(in crate::card::sets) static HOLLOW_DOGS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Hollow Dogs",
    "6e84fa4f-617d-4449-8141-783a9ce017c1",
    "Jeff Miracola",
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Phyrexian", "Zombie", "Dog"], 3, 3)
        .with_ability(AbilityDef::triggered(
            "Whenever this creature attacks, it gets +2/+0 until end of turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )),
);

// USG 138 — Ill-Gotten Gains
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ILL_GOTTEN_GAINS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Ill-Gotten Gains",
    "826230ad-6b2b-42a0-9d6f-ed07d3554efd",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// USG 139 — Looming Shade (alternate printing)
const LOOMING_SHADE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LOOMING_SHADE,
    1,
    "525f63ae-e002-4d47-bea5-b24dc89d4d06",
    "Vincent Evans",
);

// USG 139s — Looming Shade
pub(in crate::card::sets) static LOOMING_SHADE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Looming Shade",
    "3e94ab55-4390-4b3d-8e7f-a95996e2c5b7",
    "Vincent Evans",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Shade"], 1, 1).with_ability(
        AbilityDef::activated(
            "{B}: This creature gets +1/+1 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{B}"))],
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

// USG 140 — Lurking Evil
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LURKING_EVIL: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Lurking Evil",
    "f5ef7201-4443-4a77-b9e1-6f1f39e8f993",
    "Scott Kirschner",
    crate::card::CardRules::unsupported(),
);

// USG 141 — Mana Leech
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MANA_LEECH: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Mana Leech",
    "8a288616-5350-43eb-b718-5bfeb5be4ed4",
    "Mark A. Nelson",
    crate::card::CardRules::unsupported(),
);

// USG 142 — No Rest for the Wicked (alternate printing)
const NO_REST_FOR_THE_WICKED_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &NO_REST_FOR_THE_WICKED,
    1,
    "d9f9b2b0-65aa-42a1-b9f8-563b194ca5b1",
    "Carl Critchlow",
);

// USG 142s — No Rest for the Wicked
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NO_REST_FOR_THE_WICKED: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "No Rest for the Wicked",
    "c20a4bce-15db-43a7-9514-5e657b618aac",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// USG 143 — Oppression
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OPPRESSION: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Oppression",
    "8838e751-b206-4052-9263-a67b8fea05cc",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// USG 144 — Order of Yawgmoth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORDER_OF_YAWGMOTH: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Order of Yawgmoth",
    "555e858d-d8d2-4626-9343-070cf86949ab",
    "Chippy",
    crate::card::CardRules::unsupported(),
);

// USG 145 — Parasitic Bond
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PARASITIC_BOND: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Parasitic Bond",
    "47bd3995-3013-468e-b586-0c5720a0bde6",
    "Scott Kirschner",
    crate::card::CardRules::unsupported(),
);

// USG 146 — Persecute
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PERSECUTE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Persecute",
    "4f8e2be6-124d-412e-be34-aa4495a83e02",
    "D. Alexander Gregory",
    crate::card::CardRules::unsupported(),
);

// USG 147 — Pestilence (reprint)
const PESTILENCE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PESTILENCE,
    "2f1f5630-647f-4789-8506-e7fafcbdd671",
    "Pete Venters",
);

// USG 148 — Phyrexian Ghoul
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_GHOUL: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Phyrexian Ghoul",
    "4843ba92-fde2-4b46-8fdb-e0f8aca96959",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// USG 149 — Planar Void
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PLANAR_VOID: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Planar Void",
    "b035c718-ac11-4c97-a9bb-0cd88dc71904",
    "Andrew Goldhawk",
    crate::card::CardRules::unsupported(),
);

// USG 150 — Priest of Gix
pub(in crate::card::sets) static PRIEST_OF_GIX: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Priest of Gix",
    "64166899-fcc6-4000-9994-643f5a4cd214",
    "Brian Despain",
    CardRules::new_creature(
        mana_cost!("{2}{B}"),
        &["Phyrexian", "Human", "Cleric", "Minion"],
        2,
        1,
    )
    .with_ability(abilities::enters_trigger(
        "When this creature enters, add {B}{B}{B}.",
        EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Black).with_amount(3)),
    )),
);

// USG 151 — Rain of Filth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAIN_OF_FILTH: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Rain of Filth",
    "1cf07d5a-6618-48e2-a9f0-669b75bb6e85",
    "Stephen Daniele",
    crate::card::CardRules::unsupported(),
);

// USG 152 — Ravenous Skirge
pub(in crate::card::sets) static RAVENOUS_SKIRGE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Ravenous Skirge",
    "d0b6e655-e05e-44b5-9c7f-9dbbc66e6e28",
    "Ron Spencer",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Phyrexian", "Imp"], 1, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever this creature attacks, it gets +2/+0 until end of turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// USG 153 — Reclusive Wight
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RECLUSIVE_WIGHT: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Reclusive Wight",
    "384b14c6-78a0-4e82-924e-781719c8defb",
    "Vincent Evans",
    crate::card::CardRules::unsupported(),
);

// USG 154 — Reprocess
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REPROCESS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Reprocess",
    "569408b3-43c8-4cb1-a70f-15ab2aeaf8ed",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// USG 155 — Sanguine Guard
pub(in crate::card::sets) static SANGUINE_GUARD: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Sanguine Guard",
    "5c33fbb0-f49d-4b4d-804d-84b03e0daf4d",
    "Kev Walker",
    CardRules::new_creature(
        mana_cost!("{1}{B}{B}"),
        &["Phyrexian", "Zombie", "Knight"],
        2,
        2,
    )
    .with_abilities(&[
        abilities::first_strike(),
        abilities::regenerate_self(
            "{1}{B}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{B}"))],
        ),
    ]),
);

// USG 156 — Sicken
pub(in crate::card::sets) static SICKEN: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Sicken",
    "aa1beb5d-0ef2-4013-932b-5e4a5d0af559",
    "Heather Hudson",
    CardRules::new_enchantment(mana_cost!("{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets -1/-1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-1),
                        ValueDef::Constant(-1),
                    ),
                },
            ),
            abilities::cycling(
                "Cycling {2} ({2}, Discard this card: Draw a card.)",
                mana_cost!("{2}"),
            ),
        ]),
);

// USG 157 — Skirge Familiar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKIRGE_FAMILIAR: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Skirge Familiar",
    "935ee538-3ea1-4a80-bb8f-01d562f63b5d",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// USG 158 — Skittering Skirge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKITTERING_SKIRGE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Skittering Skirge",
    "93aba9d5-5f96-4aba-8248-74398b8bfe9d",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// USG 159 — Sleeper Agent
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SLEEPER_AGENT: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Sleeper Agent",
    "b1b2c4bd-2397-4ebd-b4fb-3b3e9c6c7dec",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// USG 160 — Spined Fluke
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPINED_FLUKE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Spined Fluke",
    "8a14524b-e690-4f77-b43e-416d5ec3cbb9",
    "Mark A. Nelson",
    crate::card::CardRules::unsupported(),
);

// USG 161 — Tainted Aether
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TAINTED_AETHER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Tainted Aether",
    "d1dd33a1-1603-4368-9c32-e4c9fd6ecd08",
    "Thomas M. Baxa",
    crate::card::CardRules::unsupported(),
);

// USG 162 — Unnerve
pub(in crate::card::sets) static UNNERVE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Unnerve",
    "f72779ce-bcd3-41dc-8b3f-bfb9a1b137d8",
    "Terese Nielsen",
    CardRules::new_sorcery(mana_cost!("{3}{B}")).with_ability(AbilityDef::spell(
        "Each opponent discards two cards.",
        EffectDef::Discard {
            recipient: EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::Opponent)),
            amount: ValueDef::Constant(2),
            selection: DiscardSelectionDef::RecipientChooses,
            then: None,
        },
    )),
);

// USG 163 — Unworthy Dead
pub(in crate::card::sets) static UNWORTHY_DEAD: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Unworthy Dead",
    "0f42c561-1762-43c4-a539-0cf9a5ce7f4f",
    "Carl Critchlow",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Phyrexian", "Skeleton"], 1, 1).with_ability(
        abilities::regenerate_self(
            "{B}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{B}"))],
        ),
    ),
);

// USG 163s — Unworthy Dead (alternate printing)
const UNWORTHY_DEAD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &UNWORTHY_DEAD,
    1,
    "fcf88542-e5af-430e-9d48-03a216d7526f",
    "Carl Critchlow",
);

// USG 164 — Vampiric Embrace
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VAMPIRIC_EMBRACE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Vampiric Embrace",
    "889be765-5716-4549-b544-0a49d3962e16",
    "Thomas M. Baxa",
    crate::card::CardRules::unsupported(),
);

// USG 164s — Vampiric Embrace (alternate printing)
const VAMPIRIC_EMBRACE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &VAMPIRIC_EMBRACE,
    1,
    "91d48d7c-3db8-437a-85e2-2bf9699a305c",
    "Thomas M. Baxa",
);

// USG 165 — Vebulid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VEBULID: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Vebulid",
    "d5623194-0cd9-4f3f-bab1-133d6b1e94fe",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// USG 166 — Victimize
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VICTIMIZE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Victimize",
    "caafe7da-0167-4c53-bbad-172f900d137b",
    "Val Mayerik",
    crate::card::CardRules::unsupported(),
);

// USG 167 — Vile Requiem
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VILE_REQUIEM: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Vile Requiem",
    "2be44b37-2383-4bf4-ba30-d8d4e2cb8939",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// USG 168 — Western Paladin
pub(in crate::card::sets) static WESTERN_PALADIN: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Western Paladin",
    "c4bcfe0f-2397-488b-8f02-dae1f6cd5824",
    "Carl Critchlow",
    CardRules::new_creature(
        mana_cost!("{2}{B}{B}"),
        &["Phyrexian", "Zombie", "Knight"],
        3,
        3,
    )
    .with_ability(AbilityDef::activated_with_targets(
        "{B}{B}, {T}: Destroy target white creature.",
        &[
            AbilityCostDef::Mana(mana_cost!("{B}{B}")),
            AbilityCostDef::TapSource,
        ],
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Color(ManaColor::White),
            ]),
        )],
        EffectDef::destroy_target(TargetIndex::PRIMARY, true),
    )),
);

// USG 169 — Witch Engine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WITCH_ENGINE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Witch Engine",
    "ef749290-58e2-4b40-a141-5fe294f9b995",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// USG 170 — Yawgmoth's Edict
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static YAWGMOTH_S_EDICT: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Yawgmoth's Edict",
    "3f16be4b-4540-476a-b3ac-7442507ed314",
    "Scott Kirschner",
    crate::card::CardRules::unsupported(),
);

// USG 171 — Yawgmoth's Will
pub(in crate::card::sets) static YAWGMOTH_S_WILL: CardRecord = CardRecord::new(
    CardSet::UrzasSaga,
    "Yawgmoth's Will",
    "6d3e3c3a-d351-4d91-8884-312d4b6f540d",
    "Ron Spencer",
    // Three mana to play the turn over again out of the graveyard, and the
    // exile clause is what stops it being played a third time.
    CardRules::new_sorcery(mana_cost!("{2}{B}")).with_ability(AbilityDef::spell(
        "Until end of turn, you may play lands and cast spells from your graveyard.\nIf a card \
         would be put into your graveyard from anywhere this turn, exile that card instead.",
        // The permission belongs to the player. The replacement belongs to nothing
        // at all: the card making it is in the graveyard -- or in exile, by its own
        // clause -- before it applies, so it is created as an effect object that
        // lasts the turn rather than granted to a source that will not be there.
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MayPlayFromGraveyard(
                    // Everything, played every way: the permission names no card type and no
                    // one play action, which is the whole of "play lands and cast spells".
                    GraveyardPlayPermissionDef::unlimited(PlayRestrictionDef::new(PlayActionMatcherDef::Any, ObjectPredicateDef::Any)),
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::CreateOngoingEffect(OngoingEffectDef::unbound(
                // "A card", not "a card or token": a token put into a graveyard goes there
                // and ceases to exist as it always would.
                &AbilityDef::replacement_for(
                    "If a card would be put into your graveyard from anywhere this turn, exile that card instead.",
                    ReplacementEventDef::AnyObjectWouldMove {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::OwnedBy(PlayerRelation::You),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                        ]),
                        to: ZoneKind::Graveyard,
                    },
                    ReplacementEffectDef::MoveToZone(ZoneKind::Exile),
                ),
                ResolvedEffectDurationDef::UntilEndOfTurn,
            )),
        ]),
    )),
);

// USG 172 — Acidic Soil
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ACIDIC_SOIL: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Acidic Soil",
    "790157c9-b1ed-4da5-9d50-e99e0dd807b7",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// USG 173 — Antagonism
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANTAGONISM: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Antagonism",
    "6b9db511-089e-413f-8005-ccb05ec3b06e",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// USG 174 — Arc Lightning
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARC_LIGHTNING: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Arc Lightning",
    "0c81ade7-0074-4447-ba2c-b16fa0f09ccb",
    "Andrew Goldhawk",
    crate::card::CardRules::unsupported(),
);

// USG 175 — Bedlam
pub(in crate::card::sets) static BEDLAM: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Bedlam",
    "a3e97dc9-8df3-4912-8564-7bdb2ac6564b",
    "Mike Raabe",
    CardRules::new_enchantment(mana_cost!("{2}{R}{R}")).with_ability(AbilityDef::static_ability(
        "Creatures can't block.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
        },
    )),
);

// USG 176 — Brand
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRAND: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Brand",
    "bafbd034-c8a0-4798-a680-555c13bdd251",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// USG 177 — Bravado
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRAVADO: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Bravado",
    "019ba422-4837-400c-a913-b56d87b49e26",
    "Jerry Tiritilli",
    crate::card::CardRules::unsupported(),
);

// USG 178 — Bulwark
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BULWARK: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Bulwark",
    "a2041b95-ec5b-4512-b012-f875ca686669",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// USG 179 — Crater Hellion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRATER_HELLION: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Crater Hellion",
    "2382e525-1750-484a-bf95-dbb42bbb30ae",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// USG 180 — Destructive Urge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DESTRUCTIVE_URGE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Destructive Urge",
    "479645b3-54af-4fbf-928e-2224540fe892",
    "Andrew Robinson",
    crate::card::CardRules::unsupported(),
);

// USG 181 — Disorder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DISORDER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Disorder",
    "3fa5ec10-dfea-4e6d-8996-553a4a0eb8a4",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// USG 182 — Dromosaur
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DROMOSAUR: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Dromosaur",
    "65de0b43-64df-44dc-850c-25f48d6ab53b",
    "Ciruelo",
    crate::card::CardRules::unsupported(),
);

// USG 183 — Electryte
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELECTRYTE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Electryte",
    "85c3d04f-4010-4db3-9e4e-afa8116b263d",
    "Thomas M. Baxa",
    crate::card::CardRules::unsupported(),
);

// USG 184 — Falter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FALTER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Falter",
    "e279126c-0512-4ee6-ad83-6fdfc7ae46c5",
    "Mike Raabe",
    crate::card::CardRules::unsupported(),
);

// USG 185 — Fault Line
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FAULT_LINE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Fault Line",
    "cab4fd0e-9f84-4628-92a7-858ad8064531",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// USG 186 — Fiery Mantle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FIERY_MANTLE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Fiery Mantle",
    "c41deea2-c12f-40cd-8fc4-47227c762f42",
    "Bob Eggleton",
    crate::card::CardRules::unsupported(),
);

// USG 187 — Fire Ants
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FIRE_ANTS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Fire Ants",
    "ae2e38af-a2e5-4d52-870e-1b6e0cb33cef",
    "Tom Fleming",
    crate::card::CardRules::unsupported(),
);

// USG 187s — Fire Ants (alternate printing)
const FIRE_ANTS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FIRE_ANTS,
    1,
    "c5e32a3d-c477-419a-97c2-851974c6b89e",
    "Tom Fleming",
);

// USG 188 — Gamble
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GAMBLE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Gamble",
    "0ee0f160-7339-4d98-8a8c-f08889ee52f5",
    "Andrew Goldhawk",
    crate::card::CardRules::unsupported(),
);

// USG 189 — Goblin Cadets
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_CADETS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Goblin Cadets",
    "60081115-16bc-4924-b76d-7cfc0ad2287c",
    "Jerry Tiritilli",
    crate::card::CardRules::unsupported(),
);

// USG 190 — Goblin Lackey
pub(in crate::card::sets) static GOBLIN_LACKEY: CardRecord = CardRecord::new(
    CardSet::UrzasSaga,
    "Goblin Lackey",
    "9b848caa-aad8-4060-8f86-304a8556de2d",
    "Jerry Tiritilli",
    // One connection puts a Siege-Gang Commander down for free, which is the
    // whole reason a 1/1 for one is a format staple.
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin"], 1, 1).with_ability(
        AbilityDef::triggered(
            "Whenever this creature deals damage to a player, you may put a Goblin permanent card from your hand onto the battlefield.",
            TriggerEventDef::damage_to_player(ObjectPredicateDef::Source, PlayerRelation::Any),
            // A minimum of zero is the "you may": the offer may be answered with
            // nothing, and with no Goblin in hand it is never made at all.
            EffectDef::ChooseCards {
                player: EffectRecipientDef::Controller,
                // "A Goblin permanent card": Gempalm Incinerator is a Goblin card that is
                // also a creature, and nothing in the pool is a Goblin instant, but the
                // clause names permanents rather than creatures and so does this.
                sources: &const { [CardChoiceSourceDef::Zone(ZoneKind::Hand)] },
                object: ObjectPredicateDef::All(&const {
                    [
                        ObjectPredicateDef::Subtype("Goblin"),
                        ObjectPredicateDef::Not(&const {
                            ObjectPredicateDef::AnyOf(&const {
                                [
                                    ObjectPredicateDef::HasType(CardType::Instant),
                                    ObjectPredicateDef::HasType(CardType::Sorcery),
                                ]
                            })
                        }),
                    ]
                }),
                minimum: 0,
                maximum: 1,
                reveal: false,
                destination: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
            },
        ),
    ),
);

// USG 191 — Goblin Matron (reprint)
const GOBLIN_MATRON_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1998::portal_second_age::GOBLIN_MATRON,
    "9e9e2e5d-ad06-4378-9afb-ffb174e6a5b4",
    "DiTerlizzi",
);

// USG 192 — Goblin Offensive
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_OFFENSIVE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Goblin Offensive",
    "e9813857-5527-4499-af86-758a5971e21a",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// USG 193 — Goblin Patrol
pub(in crate::card::sets) static GOBLIN_PATROL: CardRecord = CardRecord::new(
    CardSet::UrzasSaga,
    "Goblin Patrol",
    "d0fcd8d3-f159-49a1-8dd9-582ae4a0adc3",
    "Greg Staples",
    // A 2/1 for one, rented rather than bought: the echo comes due on your
    // next upkeep and once only.
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin"], 2, 1).with_ability(abilities::echo(
        "Echo {R} (At the beginning of your upkeep, if this came under your control since the beginning of your last upkeep, sacrifice it unless you pay its echo cost.)",
        mana_cost!("{R}"),
    )),
);

// USG 194 — Goblin Raider (reprint)
const GOBLIN_RAIDER_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_p02::GOBLIN_RAIDER,
    "e150a460-8a81-4097-8100-ccb8a9bb1bd7",
    "Greg Staples",
);

// USG 195 — Goblin Spelunkers
pub(in crate::card::sets) static GOBLIN_SPELUNKERS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Goblin Spelunkers",
    "7d02a81f-2dac-41f7-a818-811baa238021",
    "DiTerlizzi",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin", "Warrior"], 2, 2)
        .with_ability(abilities::mountainwalk()),
);

// USG 196 — Goblin War Buggy
pub(in crate::card::sets) static GOBLIN_WAR_BUGGY: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Goblin War Buggy",
    "d2d0fc9e-fb6b-4a00-b422-32565f7ce454",
    "DiTerlizzi",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin"], 2, 2).with_abilities(&[
        abilities::haste(),
        abilities::echo(
            "Echo {1}{R} (At the beginning of your upkeep, if this came under your control since the beginning of your last upkeep, sacrifice it unless you pay its echo cost.)",
            mana_cost!("{1}{R}"),
        ),
    ]),
);

// USG 197 — Guma
pub(in crate::card::sets) static GUMA: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Guma",
    "d6246f17-6034-4423-82c5-1aea8d71f94e",
    "Daren Bader",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Cat"], 2, 2)
        .with_ability(abilities::protection_from_color(ManaColor::Blue)),
);

// USG 198 — Headlong Rush
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEADLONG_RUSH: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Headlong Rush",
    "9f0db04c-5101-4a43-9109-3964ac66bdab",
    "Dermot Power",
    crate::card::CardRules::unsupported(),
);

// USG 199 — Heat Ray
pub(in crate::card::sets) static HEAT_RAY: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Heat Ray",
    "6a27f90e-d156-439c-b5e5-6d53bd510fe7",
    "Brian Snõddy",
    CardRules::new_instant(mana_cost!("{X}{R}")).with_ability(AbilityDef::spell_with_targets(
        "This spell deals X damage to target creature.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::ChosenX,
        },
    )),
);

// USG 200 — Jagged Lightning (reprint)
const JAGGED_LIGHTNING_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_p02::JAGGED_LIGHTNING,
    "49304f24-a961-4d1e-b85c-af6e3d8d5edc",
    "Mike Raabe",
);

// USG 201 — Lay Waste
pub(in crate::card::sets) static LAY_WASTE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Lay Waste",
    "46fa1186-51fa-419a-9cd0-42403d1dd4a7",
    "Terese Nielsen",
    CardRules::new_sorcery(mana_cost!("{3}{R}")).with_abilities(&[
        AbilityDef::destroy_target(
            "Destroy target land.",
            &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Land)),
            true,
        ),
        abilities::cycling(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            mana_cost!("{2}"),
        ),
    ]),
);

// USG 202 — Lightning Dragon
pub(in crate::card::sets) static LIGHTNING_DRAGON: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Lightning Dragon",
    "342fc7bc-657f-43a3-9558-f516fa545a09",
    "Ron Spencer",
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Dragon"], 4, 4).with_abilities(&[
        abilities::flying(),
        abilities::echo(
            "Echo {2}{R}{R} (At the beginning of your upkeep, if this came under your control since the beginning of your last upkeep, sacrifice it unless you pay its echo cost.)",
            mana_cost!("{2}{R}{R}"),
        ),
        AbilityDef::activated(
            "{R}: This creature gets +1/+0 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{R}"))],
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

// USG 203 — Meltdown
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MELTDOWN: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Meltdown",
    "9e7a967a-35a0-4e5c-a32b-123a9cfdb79e",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// USG 204 — Okk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OKK: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Okk",
    "5fecbdbc-dcfb-42ee-956c-a508db6eaafa",
    "Mike Raabe",
    crate::card::CardRules::unsupported(),
);

// USG 205 — Outmaneuver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OUTMANEUVER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Outmaneuver",
    "4a5a69fa-71ff-4e4f-9406-7cfebccb3384",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// USG 206 — Rain of Salt (reprint)
const RAIN_OF_SALT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::RAIN_OF_SALT,
    "4792293a-e11d-4c5e-bbd9-6f09e69ee617",
    "Adam Rex",
);

// USG 207 — Raze
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAZE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Raze",
    "56d51b3c-24e9-41b6-b7cd-c70329e498ca",
    "Mike Raabe",
    crate::card::CardRules::unsupported(),
);

// USG 208 — Reflexes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REFLEXES: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Reflexes",
    "614782c2-a38a-4b3e-9716-7f5c09c4ad43",
    "Steve White",
    crate::card::CardRules::unsupported(),
);

// USG 209 — Retromancer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RETROMANCER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Retromancer",
    "e9fbf63d-7106-47d5-97c3-4596d8239925",
    "Robh Ruppel",
    crate::card::CardRules::unsupported(),
);

// USG 210 — Rumbling Crescendo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUMBLING_CRESCENDO: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Rumbling Crescendo",
    "ddf267c2-c3d5-48ba-93af-dd0af133add3",
    "Lawrence Snelly",
    crate::card::CardRules::unsupported(),
);

// USG 211 — Scald
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCALD: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Scald",
    "5ff97d4b-301f-4ade-a6df-2667dff566c2",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// USG 212 — Scoria Wurm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCORIA_WURM: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Scoria Wurm",
    "4175d6e4-98fc-46f6-b549-c13e9f647eef",
    "Steve Firchow",
    crate::card::CardRules::unsupported(),
);

// USG 213 — Scrap
pub(in crate::card::sets) static SCRAP: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Scrap",
    "3f070430-59d6-462f-9f04-306ffc2ae01b",
    "Donato Giancola",
    CardRules::new_instant(mana_cost!("{2}{R}")).with_abilities(&[
        AbilityDef::destroy_target(
            "Destroy target artifact.",
            &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                CardType::Artifact,
            )),
            true,
        ),
        abilities::cycling(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            mana_cost!("{2}"),
        ),
    ]),
);

// USG 214 — Shivan Hellkite
pub(in crate::card::sets) static SHIVAN_HELLKITE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Shivan Hellkite",
    "8d74f30e-277f-4d8d-ad75-567545a78d97",
    "Bob Eggleton",
    CardRules::new_creature(mana_cost!("{5}{R}{R}"), &["Dragon"], 5, 5).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated_with_targets(
            "{1}{R}: This creature deals 1 damage to any target.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{R}"))],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// USG 215 — Shivan Raptor
pub(in crate::card::sets) static SHIVAN_RAPTOR: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Shivan Raptor",
    "0fc45153-3cb1-43bc-b694-06f6a74b3eb7",
    "Bob Eggleton",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Dinosaur"], 3, 1).with_abilities(&[
        abilities::first_strike(),
        abilities::haste(),
        abilities::echo(
            "Echo {2}{R} (At the beginning of your upkeep, if this came under your control since the beginning of your last upkeep, sacrifice it unless you pay its echo cost.)",
            mana_cost!("{2}{R}"),
        ),
    ]),
);

// USG 216 — Shiv's Embrace
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHIV_S_EMBRACE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Shiv's Embrace",
    "7c9ba1a3-1de4-4771-b76a-89354bc799d3",
    "Bob Eggleton",
    crate::card::CardRules::unsupported(),
);

// USG 217 — Shower of Sparks
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHOWER_OF_SPARKS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Shower of Sparks",
    "54428999-a83d-40a5-9753-dfefdf705a9e",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// USG 218 — Sneak Attack
pub(in crate::card::sets) static SNEAK_ATTACK: CardRecord = CardRecord::new(
    CardSet::UrzasSaga,
    "Sneak Attack",
    "d07dc95d-82a8-4a58-8ea2-d4513bd7316d",
    "Jerry Tiritilli",
    // One red mana per creature, as often as you like: what the deck is
    // paying four mana for is permission to stop casting things.
    CardRules::new_enchantment(mana_cost!("{3}{R}")).with_ability(AbilityDef::activated(
        "{R}: You may put a creature card from your hand onto the battlefield. That creature \
         gains haste. Sacrifice the creature at the beginning of the next end step.",
        &[AbilityCostDef::Mana(mana_cost!("{R}"))],
        // "You may": a minimum of none, so activating it with nothing worth
        // cheating in is legal and does nothing.
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
            // Haste and the delayed sacrifice are separate effects on the permanent
            // created by the move.
            then: &const {
                EffectDef::PutOntoBattlefieldThen {
                    object: EffectRecipientDef::object(ObjectRefDef::Binding(
                        ParentBinding,
                    )),
                    binding: ParentBinding,
                    counters: None,
                    then: &const {
                        EffectDef::Sequence(&const {
                            [
                                EffectDef::Apply {
                                    recipient: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                        ParentBinding,
                                    )),
                                    effect: AppliedEffectDef::add_ability(&const {
                                        abilities::haste()
                                    }),
                                    duration: ResolvedEffectDurationDef::Permanent,
                                },
                                // Installed as the creature arrives, so it names that permanent rather than
                                // whatever is on the battlefield when the end step comes.
                                EffectDef::InstallTrigger(InstalledTriggerDef::once(&const {
                                    // "At the beginning of the next end step", whoever's turn it is: a creature
                                    // cheated in on their turn is sacrificed at the end of that turn rather
                                    // than surviving to yours.
                                    AbilityDef::triggered(
                                        "Sacrifice the creature at the beginning of the next end step.",
                                        TriggerEventDef::StepBegins {
                                            step: TurnStepDef::End,
                                            player: PlayerRelation::Any,
                                        },
                                        EffectDef::SacrificeYours {
                                            object: EffectRecipientDef::objects(
                                                ObjectSetDef::Binding(ParentBinding),
                                            ),
                                        },
                                    )
                                })),
                            ]
                        })
                    },
                }
            },
        }),
    )),
);

// USG 219 — Steam Blast
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STEAM_BLAST: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Steam Blast",
    "144a1b4e-d960-4c3a-810b-11a0c78635ad",
    "Mike Raabe",
    crate::card::CardRules::unsupported(),
);

// USG 220 — Sulfuric Vapors
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SULFURIC_VAPORS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Sulfuric Vapors",
    "e133997f-1620-4fe9-8275-057edc998fba",
    "Lawrence Snelly",
    crate::card::CardRules::unsupported(),
);

// USG 221 — Thundering Giant
pub(in crate::card::sets) static THUNDERING_GIANT: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Thundering Giant",
    "98afd54f-2f86-4694-a688-ce3dcefccdbc",
    "Mark Zug",
    CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Giant"], 4, 3)
        .with_ability(abilities::haste()),
);

// USG 222 — Torch Song
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TORCH_SONG: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Torch Song",
    "6ba8cdf0-492a-4e64-9143-4ccb29ba1d56",
    "Thomas M. Baxa",
    crate::card::CardRules::unsupported(),
);

// USG 223 — Viashino Outrider
pub(in crate::card::sets) static VIASHINO_OUTRIDER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Viashino Outrider",
    "26ba659c-7e0f-4d8b-b91c-3c0725102ba2",
    "Ciruelo",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Lizard"], 4, 3).with_ability(
        abilities::echo(
            "Echo {2}{R} (At the beginning of your upkeep, if this came under your control since the beginning of your last upkeep, sacrifice it unless you pay its echo cost.)",
            mana_cost!("{2}{R}"),
        ),
    ),
);

// USG 224 — Viashino Runner
pub(in crate::card::sets) static VIASHINO_RUNNER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Viashino Runner",
    "15bf72e0-0b0c-4b29-8709-9dcd460508cb",
    "Steve White",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Lizard"], 3, 2)
        .with_ability(abilities::menace()),
);

// USG 225 — Viashino Sandswimmer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VIASHINO_SANDSWIMMER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Viashino Sandswimmer",
    "0790607f-51b7-40ff-80f9-4f7f5cd2d63c",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// USG 226 — Viashino Weaponsmith
pub(in crate::card::sets) static VIASHINO_WEAPONSMITH: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Viashino Weaponsmith",
    "def316ed-b080-4d1d-b946-d7a86ebb8ad9",
    "Dermot Power",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Lizard"], 2, 2).with_ability(
        AbilityDef::triggered(
            "Whenever this creature becomes blocked by a creature, this creature gets +2/+2 until end of turn.",
            TriggerEventDef::BecomesBlockedBy {
                blocker: ObjectPredicateDef::HasType(CardType::Creature),
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

// USG 227 — Vug Lizard
pub(in crate::card::sets) static VUG_LIZARD: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Vug Lizard",
    "39bbb1f6-f3c4-4e11-bb71-91ea31797d1e",
    "Heather Hudson",
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Lizard"], 3, 4).with_abilities(&[
        abilities::mountainwalk(),
        abilities::echo(
            "Echo {1}{R}{R} (At the beginning of your upkeep, if this came under your control since the beginning of your last upkeep, sacrifice it unless you pay its echo cost.)",
            mana_cost!("{1}{R}{R}"),
        ),
    ]),
);

// USG 228 — Wildfire (reprint)
const WILDFIRE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_p02::WILDFIRE,
    "72d50972-4549-40cd-9c33-4b341333803f",
    "Carl Critchlow",
);

// USG 229 — Abundance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ABUNDANCE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Abundance",
    "f6d051fa-bd3a-4c94-ae41-34d89a7bb77d",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// USG 230 — Acridian
pub(in crate::card::sets) static ACRIDIAN: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Acridian",
    "05d5a38f-5a60-46da-af1c-440e4bf7fe9e",
    "rk post",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Insect"], 2, 4).with_ability(abilities::echo(
        "Echo {1}{G} (At the beginning of your upkeep, if this came under your control since the beginning of your last upkeep, sacrifice it unless you pay its echo cost.)",
        mana_cost!("{1}{G}"),
    )),
);

// USG 231 — Albino Troll
pub(in crate::card::sets) static ALBINO_TROLL: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Albino Troll",
    "58a3c112-f0c1-4d30-8df6-63fc01356a4f",
    "Paolo Parente",
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Troll"], 3, 3).with_abilities(&[
        abilities::echo(
            "Echo {1}{G} (At the beginning of your upkeep, if this came under your control since the beginning of your last upkeep, sacrifice it unless you pay its echo cost.)",
            mana_cost!("{1}{G}"),
        ),
        abilities::regenerate_self(
            "{1}{G}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{G}"))],
        ),
    ]),
);

// USG 232 — Anaconda (reprint)
const ANACONDA_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::ANACONDA,
    "1be798fd-18c9-45b0-8207-7e5e01c83f49",
    "Stephen Daniele",
);

// USG 233 — Argothian Elder
pub(in crate::card::sets) static ARGOTHIAN_ELDER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Argothian Elder",
    "453e7cb4-bc37-4932-85b7-3a4e160b73dc",
    "DiTerlizzi",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Elf", "Druid"], 2, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Untap two target lands.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_value(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Land),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                ValueDef::Constant(2),
            )],
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ),
);

// USG 234 — Argothian Enchantress
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARGOTHIAN_ENCHANTRESS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Argothian Enchantress",
    "9ababc1a-515e-4e20-8819-19d84d9b0af5",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// USG 235 — Argothian Swine
pub(in crate::card::sets) static ARGOTHIAN_SWINE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Argothian Swine",
    "afe5e4ec-9c0e-4b1a-b3c6-e9631cf214eb",
    "Randy Elliott",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Boar"], 3, 3)
        .with_ability(abilities::trample()),
);

// USG 236 — Argothian Wurm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARGOTHIAN_WURM: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Argothian Wurm",
    "93294349-75ae-4a6b-896d-b403a5d69e98",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// USG 237 — Blanchwood Armor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLANCHWOOD_ARMOR: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Blanchwood Armor",
    "9b5f3776-74f4-4626-833b-e1b0921d3cbc",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// USG 238 — Blanchwood Treefolk
pub(in crate::card::sets) static BLANCHWOOD_TREEFOLK: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Blanchwood Treefolk",
    "f824502c-d712-41af-ba44-33e8294c3735",
    "DiTerlizzi",
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Treefolk"], 4, 5),
);

// USG 239 — Bull Hippo (reprint)
const BULL_HIPPO_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::BULL_HIPPO,
    "1d1f8259-1825-4a46-8026-75adc4480322",
    "Daren Bader",
);

// USG 240 — Carpet of Flowers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CARPET_OF_FLOWERS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Carpet of Flowers",
    "93abb48a-85f2-432d-8602-0a1d17fbb409",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// USG 241 — Cave Tiger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CAVE_TIGER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Cave Tiger",
    "c782eb28-0cdd-4c3d-9d89-8b23c29cddd4",
    "Hannibal King",
    crate::card::CardRules::unsupported(),
);

// USG 242 — Child of Gaea
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHILD_OF_GAEA: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Child of Gaea",
    "a836d9bd-a4cb-4676-935c-5fb7b793a42f",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// USG 243 — Citanul Centaurs
pub(in crate::card::sets) static CITANUL_CENTAURS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Citanul Centaurs",
    "5a3ac987-7906-4159-a007-ed409baea9d7",
    "Val Mayerik",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Centaur"], 6, 3).with_abilities(&[
        abilities::shroud(),
        abilities::echo(
            "Echo {3}{G} (At the beginning of your upkeep, if this came under your control since the beginning of your last upkeep, sacrifice it unless you pay its echo cost.)",
            mana_cost!("{3}{G}"),
        ),
    ]),
);

// USG 244 — Citanul Hierophants
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CITANUL_HIEROPHANTS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Citanul Hierophants",
    "905506cc-f77e-4214-8eb3-6f141997336c",
    "Vincent Evans",
    crate::card::CardRules::unsupported(),
);

// USG 245 — Cradle Guard
pub(in crate::card::sets) static CRADLE_GUARD: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Cradle Guard",
    "8b659c1c-cc0b-40f0-87b4-aeddb44dfac5",
    "Mark Zug",
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Treefolk"], 4, 4).with_abilities(&[
        abilities::trample(),
        abilities::echo(
            "Echo {1}{G}{G} (At the beginning of your upkeep, if this came under your control since the beginning of your last upkeep, sacrifice it unless you pay its echo cost.)",
            mana_cost!("{1}{G}{G}"),
        ),
    ]),
);

// USG 246 — Crosswinds
pub(in crate::card::sets) static CROSSWINDS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Crosswinds",
    "f182a85b-a119-46e2-8b8b-48b6758d9c39",
    "Randy Elliott",
    CardRules::new_enchantment(mana_cost!("{1}{G}")).with_ability(AbilityDef::static_ability(
        "Creatures with flying get -2/-0.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasKeyword(crate::card::KeywordAbility::Flying),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(-2),
                ValueDef::Constant(0),
            ),
        },
    )),
);

// USG 247 — Elvish Herder
pub(in crate::card::sets) static ELVISH_HERDER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Elvish Herder",
    "3cb2b07e-3d50-4b85-be2a-c99e9c8ebf25",
    "Tom Fleming",
    CardRules::new_creature(mana_cost!("{G}"), &["Elf"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{G}: Target creature gains trample until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{G}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::trample()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// USG 248 — Elvish Lyrist
pub(in crate::card::sets) static ELVISH_LYRIST: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Elvish Lyrist",
    "1c63ea60-4ce0-4dc7-bda6-7f623b0f9e2a",
    "Rebecca Guay",
    CardRules::new_creature(mana_cost!("{G}"), &["Elf"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{G}, {T}, Sacrifice this creature: Destroy target enchantment.",
            &[
                AbilityCostDef::Mana(mana_cost!("{G}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Enchantment),
            )],
            EffectDef::destroy_target(TargetIndex::PRIMARY, true),
        ),
    ),
);

// USG 249 — Endless Wurm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENDLESS_WURM: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Endless Wurm",
    "53772435-e20e-4b9e-a2f1-c1c6a4dcac79",
    "DiTerlizzi",
    crate::card::CardRules::unsupported(),
);

// USG 250 — Exploration
pub(in crate::card::sets) static EXPLORATION: CardRecord = CardRecord::new(
    CardSet::UrzasSaga,
    "Exploration",
    "2f09e451-0246-45a2-8bfd-07d3c65ddfe6",
    "Brian Snõddy",
    // One mana for a second land drop every turn, which is nothing at all
    // in a deck with no lands left to play and the best card in the deck in
    // one that keeps finding them.
    CardRules::new_enchantment(mana_cost!("{G}")).with_ability(AbilityDef::static_ability(
        "You may play an additional land on each of your turns.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::Controller,
            effect: AppliedEffectDef::Rule(AppliedRuleDef::MayPlayAdditionalLands(1)),
        },
    )),
);

// USG 251 — Fecundity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FECUNDITY: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Fecundity",
    "54c93c11-448c-402d-ac55-8d5ab4c83d7b",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// USG 252 — Fertile Ground
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FERTILE_GROUND: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Fertile Ground",
    "091dda35-59e5-456d-8804-61513a610aed",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// USG 253 — Fortitude
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FORTITUDE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Fortitude",
    "d54d5240-8afc-4c61-aaf6-a78d2b92e5c9",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// USG 254 — Gaea's Bounty
pub(in crate::card::sets) static GAEA_S_BOUNTY: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Gaea's Bounty",
    "66b55e37-fcad-4d50-89d4-5d88269fee66",
    "Stephen Daniele",
    CardRules::new_sorcery(mana_cost!("{2}{G}")).with_ability(AbilityDef::spell(
        "Search your library for up to two Forest cards, reveal those cards, put them into your hand, then shuffle.",
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
            minimum: 0,
            maximum: ValueDef::Constant(2),
            reveal: true,
            destination: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: false,
            attachment: None,
            binding: None,
            then: None,
        },
    )),
);

// USG 255 — Gaea's Embrace
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GAEA_S_EMBRACE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Gaea's Embrace",
    "cce99456-d7cc-480f-b283-0ba063c89e0a",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// USG 256 — Gorilla Warrior (reprint)
const GORILLA_WARRIOR_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y1997::portal::GORILLA_WARRIOR,
    "76c7e2b0-2df0-4cde-8565-762c93e6c14f",
    "Steve White",
);

// USG 257 — Greater Good
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GREATER_GOOD: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Greater Good",
    "12befd35-2dc6-4852-a153-75b553042643",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// USG 258 — Greener Pastures
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GREENER_PASTURES: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Greener Pastures",
    "55c3222e-ac18-4f57-9c9e-50deb0a69db3",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// USG 259 — Hawkeater Moth
pub(in crate::card::sets) static HAWKEATER_MOTH: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Hawkeater Moth",
    "842e8c4c-32f3-4ec3-b636-1d7dc9f0023e",
    "Heather Hudson",
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Insect"], 1, 2)
        .with_abilities(&[abilities::flying(), abilities::shroud()]),
);

// USG 260 — Hidden Ancients
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HIDDEN_ANCIENTS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Hidden Ancients",
    "45f4f257-c04e-489f-8ffd-814bf075e7d1",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// USG 261 — Hidden Guerrillas
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HIDDEN_GUERRILLAS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Hidden Guerrillas",
    "1ab16fbb-1542-4ba6-9a06-30806176572c",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// USG 262 — Hidden Herd
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HIDDEN_HERD: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Hidden Herd",
    "20f07992-1f81-440a-a80a-164c44d0a471",
    "Andrew Robinson",
    crate::card::CardRules::unsupported(),
);

// USG 263 — Hidden Predators
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HIDDEN_PREDATORS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Hidden Predators",
    "27f8ee86-a576-4f40-b165-be5b4558507d",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// USG 264 — Hidden Spider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HIDDEN_SPIDER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Hidden Spider",
    "ab2640cb-7d0b-40e5-9624-02c64b2f9830",
    "Thomas M. Baxa",
    crate::card::CardRules::unsupported(),
);

// USG 265 — Hidden Stag
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HIDDEN_STAG: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Hidden Stag",
    "0501e115-ef61-4db5-bdb3-36dc4334431c",
    "Berry",
    crate::card::CardRules::unsupported(),
);

// USG 266 — Hush
pub(in crate::card::sets) static HUSH: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Hush",
    "35f01c95-ce9e-4b45-9d15-a5d37100a5d8",
    "Rebecca Guay",
    CardRules::new_sorcery(mana_cost!("{3}{G}")).with_abilities(&[
        AbilityDef::spell(
            "Destroy all enchantments.",
            EffectDef::Destroy {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                can_regenerate: true,
                then: None,
            },
        ),
        abilities::cycling(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            mana_cost!("{2}"),
        ),
    ]),
);

// USG 267 — Lull
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LULL: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Lull",
    "eec5d627-b3d7-4e11-81c7-bf4cef5409a6",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// USG 268 — Midsummer Revel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIDSUMMER_REVEL: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Midsummer Revel",
    "b40a7f65-10fe-4ce5-ad1c-be53a635d7a9",
    "Steve Firchow",
    crate::card::CardRules::unsupported(),
);

// USG 269 — Pouncing Jaguar
pub(in crate::card::sets) static POUNCING_JAGUAR: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Pouncing Jaguar",
    "d35ac6e5-3e46-4290-9683-51d6f54e4edf",
    "Daren Bader",
    CardRules::new_creature(mana_cost!("{G}"), &["Cat"], 2, 2).with_ability(abilities::echo(
        "Echo {G} (At the beginning of your upkeep, if this came under your control since the beginning of your last upkeep, sacrifice it unless you pay its echo cost.)",
        mana_cost!("{G}"),
    )),
);

// USG 270 — Priest of Titania
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PRIEST_OF_TITANIA: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Priest of Titania",
    "965c33c3-0c68-4516-b8b0-5a0552ed44b6",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// USG 271 — Rejuvenate
pub(in crate::card::sets) static REJUVENATE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Rejuvenate",
    "fe3709a3-7a1b-4644-b1d5-e1ffef549f94",
    "Greg Simanson",
    CardRules::new_sorcery(mana_cost!("{3}{G}")).with_abilities(&[
        AbilityDef::spell(
            "You gain 6 life.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(6),
            },
        ),
        abilities::cycling(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            mana_cost!("{2}"),
        ),
    ]),
);

// USG 272 — Retaliation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RETALIATION: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Retaliation",
    "7a6e3814-5b9f-40da-8205-263fc49294da",
    "Tom Fleming",
    crate::card::CardRules::unsupported(),
);

// USG 273 — Sporogenesis
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPOROGENESIS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Sporogenesis",
    "839d969b-b1f7-4978-b00c-db0766161f63",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// USG 274 — Spreading Algae
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPREADING_ALGAE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Spreading Algae",
    "20cfc00e-180c-4277-8911-43afa691b9d7",
    "Stephen Daniele",
    crate::card::CardRules::unsupported(),
);

// USG 275 — Symbiosis
pub(in crate::card::sets) static SYMBIOSIS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Symbiosis",
    "5d35e2c5-1871-4749-aefa-4a7a69645c03",
    "Jeff Miracola",
    CardRules::new_instant(mana_cost!("{1}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Two target creatures each get +2/+2 until end of turn.",
        &[AbilityTargetDef::exactly_value(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
            ValueDef::Constant(2),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(2),
                ValueDef::Constant(2),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// USG 276 — Titania's Boon
pub(in crate::card::sets) static TITANIA_S_BOON: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Titania's Boon",
    "b8d64591-8552-4e94-b932-7a23922513a1",
    "Val Mayerik",
    CardRules::new_sorcery(mana_cost!("{3}{G}")).with_ability(AbilityDef::spell(
        "Put a +1/+1 counter on each creature you control.",
        EffectDef::AddCounters {
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            kind: CounterKind::PlusOnePlusOne,
            amount: ValueDef::Constant(1),
        },
    )),
);

// USG 277 — Titania's Chosen
pub(in crate::card::sets) static TITANIA_S_CHOSEN: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Titania's Chosen",
    "d9450340-2c29-4573-8da2-0d3cae9759c1",
    "Mark Zug",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Elf", "Archer"], 1, 1).with_ability(
        AbilityDef::triggered(
            "Whenever a player casts a green spell, put a +1/+1 counter on this creature.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::Color(ManaColor::Green)),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// USG 278 — Treefolk Seedlings
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TREEFOLK_SEEDLINGS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Treefolk Seedlings",
    "1ec5ee47-c2ef-442d-b1e7-323a8dba6627",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// USG 279 — Treetop Rangers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TREETOP_RANGERS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Treetop Rangers",
    "33f1a343-250b-4a30-a3b5-296282a70446",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// USG 280 — Venomous Fangs
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VENOMOUS_FANGS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Venomous Fangs",
    "f5ae1348-c063-4ec8-9d8c-3d19a6421800",
    "Lawrence Snelly",
    crate::card::CardRules::unsupported(),
);

// USG 281 — Vernal Bloom
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VERNAL_BLOOM: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Vernal Bloom",
    "80a49c80-02ee-4d5a-83cb-53ac3e7870e7",
    "Bob Eggleton",
    crate::card::CardRules::unsupported(),
);

// USG 282 — War Dance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WAR_DANCE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "War Dance",
    "392b4acf-4548-47dd-be58-bdf4ca4ece7e",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// USG 283 — Whirlwind
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WHIRLWIND: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Whirlwind",
    "8101bab4-ef93-451a-a24f-e1456c82837c",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// USG 284 — Wild Dogs
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WILD_DOGS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Wild Dogs",
    "7c7811e2-8bff-44a9-96bf-8e302c3cade9",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// USG 285 — Winding Wurm
pub(in crate::card::sets) static WINDING_WURM: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Winding Wurm",
    "ed75dc43-172c-4302-8807-23bfdd65baf4",
    "DiTerlizzi",
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Wurm"], 6, 6).with_ability(abilities::echo(
        "Echo {4}{G} (At the beginning of your upkeep, if this came under your control since the beginning of your last upkeep, sacrifice it unless you pay its echo cost.)",
        mana_cost!("{4}{G}"),
    )),
);

// USG 286 — Barrin's Codex
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BARRIN_S_CODEX: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Barrin's Codex",
    "9951b225-f11e-41bd-ab6e-e1fda957dff3",
    "DiTerlizzi",
    crate::card::CardRules::unsupported(),
);

// USG 287 — Cathodion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CATHODION: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Cathodion",
    "28def495-1806-40ec-b170-ca727c914f30",
    "Henry G. Higginbotham",
    crate::card::CardRules::unsupported(),
);

// USG 288 — Chimeric Staff
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHIMERIC_STAFF: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Chimeric Staff",
    "a27c228b-8339-49d1-af0c-05e7d7ba4c9d",
    "Michael Sutfin",
    crate::card::CardRules::unsupported(),
);

// USG 289 — Citanul Flute
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CITANUL_FLUTE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Citanul Flute",
    "bf71fab2-d936-40f0-bda2-b0630948e1fa",
    "Berry",
    crate::card::CardRules::unsupported(),
);

// USG 290 — Claws of Gix
pub(in crate::card::sets) static CLAWS_OF_GIX: CardRecord = CardRecord::new(
    CardSet::UrzasSaga,
    "Claws of Gix",
    "78372366-8c4c-46ac-bd7c-a735c2b24b5d",
    "Henry G. Higginbotham",
    CardRules::new_artifact(mana_cost!("{0}")).with_ability(AbilityDef::activated(
        "{1}, Sacrifice a permanent: You gain 1 life.",
        &[
            AbilityCostDef::Mana(mana_cost!("{1}")),
            AbilityCostDef::SacrificePermanent {
                object: ObjectPredicateDef::Any,
                controller: PlayerRelation::You,
            },
        ],
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    )),
);

// USG 291 — Copper Gnomes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COPPER_GNOMES: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Copper Gnomes",
    "d5e326b7-6f6a-4249-a315-c5f017931c73",
    "Jeff Laubenstein",
    crate::card::CardRules::unsupported(),
);

// USG 292 — Crystal Chimes
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CRYSTAL_CHIMES: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Crystal Chimes",
    "d5d03d54-7aa4-4586-bb4b-8a5d269fe289",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// USG 293 — Dragon Blood
pub(in crate::card::sets) static DRAGON_BLOOD: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Dragon Blood",
    "ff72806a-ae41-44f5-ad74-56c3338ebfcb",
    "Greg Simanson",
    CardRules::new_artifact(mana_cost!("{3}")).with_ability(AbilityDef::activated_with_targets(
        "{3}, {T}: Put a +1/+1 counter on target creature.",
        &[
            AbilityCostDef::Mana(mana_cost!("{3}")),
            AbilityCostDef::TapSource,
        ],
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::AddCounters {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            kind: CounterKind::PlusOnePlusOne,
            amount: ValueDef::Constant(1),
        },
    )),
);

// USG 294 — Endoskeleton
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENDOSKELETON: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Endoskeleton",
    "ce4630c3-bf8e-46ac-be68-f454c4ca1047",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// USG 295 — Fluctuator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLUCTUATOR: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Fluctuator",
    "92078408-e0e4-443e-b0fd-aac0ac651f46",
    "John Matson",
    crate::card::CardRules::unsupported(),
);

// USG 296 — Grafted Skullcap
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRAFTED_SKULLCAP: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Grafted Skullcap",
    "3b8424e6-4a92-4be3-b69e-49ac9a736f94",
    "Brian Despain",
    crate::card::CardRules::unsupported(),
);

// USG 297 — Hopping Automaton
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HOPPING_AUTOMATON: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Hopping Automaton",
    "4717b6c3-9fba-454f-9d02-e8c2869c4450",
    "Val Mayerik",
    crate::card::CardRules::unsupported(),
);

// USG 298 — Karn, Silver Golem
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KARN_SILVER_GOLEM: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Karn, Silver Golem",
    "811a0988-2900-426c-9413-8f1778d99678",
    "Mark Zug",
    crate::card::CardRules::unsupported(),
);

// USG 299 — Lifeline
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIFELINE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Lifeline",
    "40cee82f-36b2-48a9-930a-8e23cb2742fc",
    "D. Alexander Gregory",
    crate::card::CardRules::unsupported(),
);

// USG 300 — Lotus Blossom
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LOTUS_BLOSSOM: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Lotus Blossom",
    "46d17e0a-5407-4fde-8eb0-8f580eab5565",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// USG 301 — Metrognome
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static METROGNOME: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Metrognome",
    "d71f153d-2d07-4a8a-b725-747bb96afe00",
    "Jeff Laubenstein",
    crate::card::CardRules::unsupported(),
);

// USG 302 — Mishra's Helix
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISHRA_S_HELIX: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Mishra's Helix",
    "5c62507f-465d-45e9-96bf-06671d19e79e",
    "Berry",
    crate::card::CardRules::unsupported(),
);

// USG 303 — Mobile Fort
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOBILE_FORT: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Mobile Fort",
    "5f642246-5b2a-46a6-9236-524a297d7608",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// USG 304 — Noetic Scales
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NOETIC_SCALES: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Noetic Scales",
    "18506777-cd69-42f6-99c9-cde9b0958868",
    "Andrew Robinson",
    crate::card::CardRules::unsupported(),
);

// USG 305 — Phyrexian Colossus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_COLOSSUS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Phyrexian Colossus",
    "1b0ba37a-ce52-4c01-89aa-25e90cda04ba",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// USG 306 — Phyrexian Processor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_PROCESSOR: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Phyrexian Processor",
    "6875ce99-badd-44da-8e5d-509600efa1d0",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// USG 307 — Pit Trap (reprint)
const PIT_TRAP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_ice::PIT_TRAP,
    "4e2003c1-356b-4714-9a2a-4e12f782321e",
    "Brian Snõddy",
);

// USG 308 — Purging Scythe
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PURGING_SCYTHE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Purging Scythe",
    "0ef8128a-e0f5-43d6-b7bd-3f2003d83eab",
    "Matthew D. Wilson",
    crate::card::CardRules::unsupported(),
);

// USG 309 — Smokestack
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SMOKESTACK: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Smokestack",
    "6a37ab2d-094f-431e-878f-93aef0360413",
    "Scott Kirschner",
    crate::card::CardRules::unsupported(),
);

// USG 310 — Temporal Aperture
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEMPORAL_APERTURE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Temporal Aperture",
    "5b5c2555-c707-4814-933a-c275b9ebc0a3",
    "Michael Sutfin",
    crate::card::CardRules::unsupported(),
);

// USG 311 — Thran Turbine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THRAN_TURBINE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Thran Turbine",
    "791c310f-b50d-4993-9add-9f585f6172af",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// USG 312 — Umbilicus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UMBILICUS: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Umbilicus",
    "29a6db88-a11d-49b4-8692-28b24d23f3c7",
    "Dermot Power",
    crate::card::CardRules::unsupported(),
);

// USG 313 — Urza's Armor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static URZA_S_ARMOR: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Urza's Armor",
    "df901d78-e8fb-4adf-bafe-99a567a375c1",
    "rk post",
    crate::card::CardRules::unsupported(),
);

// USG 314 — Voltaic Key
pub(in crate::card::sets) static VOLTAIC_KEY: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Voltaic Key",
    "1aa4baf7-4693-4c55-af04-2fa5d901d701",
    "Henry G. Higginbotham",
    CardRules::new_artifact(mana_cost!("{1}")).with_ability(AbilityDef::activated_with_targets(
        "{1}, {T}: Untap target artifact.",
        &[
            AbilityCostDef::Mana(mana_cost!("{1}")),
            AbilityCostDef::TapSource,
        ],
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Artifact),
        )],
        EffectDef::Untap {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        },
    )),
);

// USG 315 — Wall of Junk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WALL_OF_JUNK: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Wall of Junk",
    "cd3f88b3-9796-4d4d-af8b-543f0d280e77",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// USG 316 — Whetstone
pub(in crate::card::sets) static WHETSTONE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Whetstone",
    "627805a0-535f-4cba-8176-a4de290b9c15",
    "Greg Simanson",
    CardRules::new_artifact(mana_cost!("{3}")).with_ability(AbilityDef::activated(
        "{3}: Each player mills two cards.",
        &[AbilityCostDef::Mana(mana_cost!("{3}"))],
        EffectDef::Mill {
            player: EffectRecipientDef::EachPlayer,
            amount: ValueDef::Constant(2),
        },
    )),
);

// USG 317 — Wirecat
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WIRECAT: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Wirecat",
    "6333d686-58ec-4360-8929-9f7302f9a09c",
    "Michael Sutfin",
    crate::card::CardRules::unsupported(),
);

// USG 318 — Worn Powerstone
pub(in crate::card::sets) static WORN_POWERSTONE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Worn Powerstone",
    "2224d7ef-2e2f-47dd-a4a0-e36b3170b124",
    "Henry G. Higginbotham",
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        abilities::enters_tapped("This artifact enters tapped."),
        AbilityDef::activated_mana(
            "{T}: Add {C}{C}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless).with_amount(2)),
        ),
    ]),
);

// USG 319 — Blasted Landscape
pub(in crate::card::sets) static BLASTED_LANDSCAPE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Blasted Landscape",
    "ac71d910-6c50-4894-8092-d39cbb7a83b4",
    "Ciruelo",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        abilities::cycling(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            mana_cost!("{2}"),
        ),
    ]),
);

// USG 320 — Drifting Meadow
pub(in crate::card::sets) static DRIFTING_MEADOW: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Drifting Meadow",
    "a8f669f7-0b36-4c82-8e32-15314ec0c0c4",
    "Bob Eggleton",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        abilities::tap_for(ManaColor::White),
        abilities::cycling(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            mana_cost!("{2}"),
        ),
    ]),
);

// USG 321 — Gaea's Cradle
pub(in crate::card::sets) static GAEAS_CRADLE: CardRecord = CardRecord::new(
    CardSet::UrzasSaga,
    "Gaea's Cradle",
    "25b0b816-0583-44aa-9dc5-f3ff48993a51",
    "Mark Zug",
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::activated_mana(
            "{T}: Add {G} for each creature you control.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddManaEqualTo {
                color: ManaColor::Green,
                amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
            },
        )),
);

// USG 322 — Phyrexian Tower
pub(in crate::card::sets) static PHYREXIAN_TOWER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Phyrexian Tower",
    "4f915cf0-6273-4896-bf24-fb0ec17b6096",
    "Chippy",
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::tap_for(ManaColor::Colorless),
            AbilityDef::activated_mana(
                "{T}, Sacrifice a creature: Add {B}{B}.",
                &[
                    AbilityCostDef::TapSource,
                    AbilityCostDef::SacrificePermanent {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        controller: PlayerRelation::You,
                    },
                ],
                EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Black).with_amount(2)),
            ),
        ]),
);

// USG 323 — Polluted Mire
pub(in crate::card::sets) static POLLUTED_MIRE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Polluted Mire",
    "cfe2c562-ac25-4395-a9f0-8b246c7954b6",
    "Stephen Daniele",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        abilities::tap_for(ManaColor::Black),
        abilities::cycling(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            mana_cost!("{2}"),
        ),
    ]),
);

// USG 324 — Remote Isle
pub(in crate::card::sets) static REMOTE_ISLE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Remote Isle",
    "e48d55ff-10d0-4d9b-9202-02ebb2137953",
    "Ciruelo",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        abilities::tap_for(ManaColor::Blue),
        abilities::cycling(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            mana_cost!("{2}"),
        ),
    ]),
);

// USG 325 — Serra's Sanctum
pub(in crate::card::sets) static SERRA_S_SANCTUM: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Serra's Sanctum",
    "f7a18130-dbaa-4657-a885-3a96a985935a",
    "Ciruelo",
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::activated_mana(
            "{T}: Add {W} for each enchantment you control.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddManaEqualTo {
                color: ManaColor::White,
                amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
            },
        )),
);

// USG 326 — Shivan Gorge
pub(in crate::card::sets) static SHIVAN_GORGE: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Shivan Gorge",
    "1531eb8c-af8a-45c6-9058-3337c44e609f",
    "John Matson",
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::tap_for(ManaColor::Colorless),
            AbilityDef::activated(
                "{2}{R}, {T}: This land deals 1 damage to each opponent.",
                &[
                    AbilityCostDef::Mana(mana_cost!("{2}{R}")),
                    AbilityCostDef::TapSource,
                ],
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::players(PlayerSetDef::Related(
                        PlayerRelation::Opponent,
                    )),
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// USG 327 — Slippery Karst
pub(in crate::card::sets) static SLIPPERY_KARST: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Slippery Karst",
    "6d01a6a7-c006-4fce-a546-8138baef421b",
    "Stephen Daniele",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        abilities::tap_for(ManaColor::Green),
        abilities::cycling(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            mana_cost!("{2}"),
        ),
    ]),
);

// USG 328 — Smoldering Crater
pub(in crate::card::sets) static SMOLDERING_CRATER: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Smoldering Crater",
    "e9940ce4-09d7-4e89-b456-e3126a83cfe1",
    "Mark Tedin",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        abilities::tap_for(ManaColor::Red),
        abilities::cycling(
            "Cycling {2} ({2}, Discard this card: Draw a card.)",
            mana_cost!("{2}"),
        ),
    ]),
);

// USG 329 — Thran Quarry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THRAN_QUARRY: CardRecord = CardRecord::new(
    crate::card::CardSet::UrzasSaga,
    "Thran Quarry",
    "4b2d6c41-7d82-4062-a783-37d88536279c",
    "Michael Sutfin",
    crate::card::CardRules::unsupported(),
);

// USG 330 — Tolarian Academy
pub(in crate::card::sets) static TOLARIAN_ACADEMY: CardRecord = CardRecord::new(
    CardSet::UrzasSaga,
    "Tolarian Academy",
    "ad7ac9a5-340f-4509-826c-7b9416d47887",
    "Stephen Daniele",
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::activated_mana(
            "{T}: Add {U} for each artifact you control.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddManaEqualTo {
                color: ManaColor::Blue,
                amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
            },
        )),
);

// USG 331 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAINS,
    "b9e35567-05df-4a3f-8c29-d8327abc2e8d",
    "Rob Alexander",
);

// USG 332 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    1,
    "a34ef351-2864-42f6-8943-75bb6fffc9f7",
    "Rob Alexander",
);

// USG 333 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    2,
    "54db7822-ebed-4c7c-8ede-469d72fd694a",
    "Rob Alexander",
);

// USG 334 — Plains (alternate printing)
const PLAINS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    3,
    "11f87b5c-78b4-443c-ad92-f37bb62d0bbe",
    "Rob Alexander",
);

// USG 335 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND,
    "8e4ce9ff-c295-475b-b9fa-88ed65a84f35",
    "Donato Giancola",
);

// USG 336 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    1,
    "bbf69bc5-8ee3-4b17-a3b1-51e35dd2d0dc",
    "Donato Giancola",
);

// USG 337 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    2,
    "92724530-9c7a-44ab-9381-ee56bfccb641",
    "Donato Giancola",
);

// USG 338 — Island (alternate printing)
const ISLAND_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    3,
    "43297ca7-846c-4bc4-a347-997279bc73d6",
    "Donato Giancola",
);

// USG 339 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWAMP,
    "c04dac6d-a7c2-44ee-b735-bd4eade06e4e",
    "John Avon",
);

// USG 340 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    1,
    "8a97cc92-6894-4c6f-8c8d-bfc9fdd4f974",
    "John Avon",
);

// USG 341 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    2,
    "f72d1102-0ad5-40df-9ca2-26448321d3d1",
    "John Avon",
);

// USG 342 — Swamp (alternate printing)
const SWAMP_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    3,
    "f7f24df7-65c6-4488-967a-e847bdec7db0",
    "John Avon",
);

// USG 343 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MOUNTAIN,
    "e9d34a2f-09ed-4fb4-891a-890f450699bd",
    "John Avon",
);

// USG 344 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    1,
    "be7e4b30-7f45-4109-b5ed-9223a9422d95",
    "John Avon",
);

// USG 345 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    2,
    "3d811021-40b1-43b1-88f1-04d711c2ab57",
    "John Avon",
);

// USG 346 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    3,
    "29b39c91-e367-487d-9820-893870df23b1",
    "John Avon",
);

// USG 347 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOREST,
    "b38c68a5-86eb-4fb2-8a43-4a7d63195462",
    "Anthony S. Waters",
);

// USG 348 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    1,
    "c151d945-7da0-4ab8-b0db-e41d10c0eb91",
    "Anthony S. Waters",
);

// USG 349 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    2,
    "a8990632-f055-4583-ae85-5ac742549b61",
    "Anthony S. Waters",
);

// USG 350 — Forest (alternate printing)
const FOREST_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    3,
    "0b43815e-8b8a-4745-bdf3-f72c8d60c48c",
    "Anthony S. Waters",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ABSOLUTE_GRACE,
    &ABSOLUTE_LAW,
    &ANGELIC_CHORUS,
    &ANGELIC_PAGE,
    &BRILLIANT_HALO,
    &CATASTROPHE,
    &CLEAR,
    &CONGREGATE,
    &DEFENSIVE_FORMATION,
    &DISCIPLE_OF_GRACE,
    &DISCIPLE_OF_LAW,
    &ELITE_ARCHERS,
    &FAITH_HEALER,
    &GLORIOUS_ANTHEM,
    &HERALD_OF_SERRA,
    &HUMBLE,
    &INTREPID_HERO,
    &MONK_IDEALIST,
    &MONK_REALIST,
    &OPAL_ACROLITH,
    &OPAL_ARCHANGEL,
    &OPAL_CARYATID,
    &OPAL_GARGOYLE,
    &OPAL_TITAN,
    &PARIAH,
    &PEGASUS_CHARGER,
    &PLANAR_BIRTH,
    &REDEEM,
    &REMEMBRANCE,
    &RUNE_OF_PROTECTION_ARTIFACTS,
    &RUNE_OF_PROTECTION_BLACK,
    &RUNE_OF_PROTECTION_BLUE,
    &RUNE_OF_PROTECTION_GREEN,
    &RUNE_OF_PROTECTION_LANDS,
    &RUNE_OF_PROTECTION_RED,
    &RUNE_OF_PROTECTION_WHITE,
    &SANCTUM_CUSTODIAN,
    &SANCTUM_GUARDIAN,
    &SERRA_AVATAR,
    &SERRA_ZEALOT,
    &SERRA_S_EMBRACE,
    &SERRA_S_HYMN,
    &SERRA_S_LITURGY,
    &SHIMMERING_BARRIER,
    &SILENT_ATTENDANT,
    &SONGSTITCHER,
    &SOUL_SCULPTOR,
    &VOICE_OF_GRACE,
    &VOICE_OF_LAW,
    &WAYLAY,
    &WORSHIP,
    &ACADEMY_RESEARCHERS,
    &ANNUL,
    &ARCANE_LABORATORY,
    &ATTUNEMENT,
    &BACK_TO_BASICS,
    &BARRIN_MASTER_WIZARD,
    &CATALOG,
    &CLOAK_OF_MISTS,
    &CONFISCATE,
    &CORAL_MERFOLK,
    &CURFEW,
    &DISRUPTIVE_STUDENT,
    &DOUSE,
    &DRIFTING_DJINN,
    &ENERGY_FIELD,
    &FOG_BANK,
    &GILDED_DRAKE,
    &GREAT_WHALE,
    &HERMETIC_STUDY,
    &HIBERNATION,
    &HORSESHOE_CRAB,
    &IMAGINARY_PET,
    &LAUNCH,
    &LILTING_REFRAIN,
    &LINGERING_MIRAGE,
    &MORPHLING,
    &PENDRELL_DRAKE,
    &PENDRELL_FLUX,
    &PEREGRINE_DRAKE,
    &POWER_TAINT,
    &RECANTATION,
    &RESCIND,
    &REWIND,
    &SANDBAR_MERFOLK,
    &SANDBAR_SERPENT,
    &SHOW_AND_TELL,
    &SOMNOPHORE,
    &SPIRE_OWL,
    &STERN_PROCTOR,
    &STROKE_OF_GENIUS,
    &SUNDER,
    &TELEPATHY,
    &TIME_SPIRAL,
    &TOLARIAN_WINDS,
    &TURNABOUT,
    &VEIL_OF_BIRDS,
    &VEILED_APPARITION,
    &VEILED_CROCODILE,
    &VEILED_SENTRY,
    &VEILED_SERPENT,
    &WINDFALL,
    &WIZARD_MENTOR,
    &ZEPHID,
    &ZEPHID_S_EMBRACE,
    &ABYSSAL_HORROR,
    &BEFOUL,
    &BEREAVEMENT,
    &BLOOD_VASSAL,
    &BREACH,
    &CACKLING_FIEND,
    &CARRION_BEETLES,
    &CONTAMINATION,
    &CORRUPT,
    &CRAZED_SKIRGE,
    &DARK_HATCHLING,
    &DARKEST_HOUR,
    &DESPONDENCY,
    &DIABOLIC_SERVITUDE,
    &DISCORDANT_DIRGE,
    &DURESS,
    &EASTERN_PALADIN,
    &EXHUME,
    &EXPUNGE,
    &FLESH_REAVER,
    &HOLLOW_DOGS,
    &ILL_GOTTEN_GAINS,
    &LOOMING_SHADE,
    &LURKING_EVIL,
    &MANA_LEECH,
    &NO_REST_FOR_THE_WICKED,
    &OPPRESSION,
    &ORDER_OF_YAWGMOTH,
    &PARASITIC_BOND,
    &PERSECUTE,
    &PHYREXIAN_GHOUL,
    &PLANAR_VOID,
    &PRIEST_OF_GIX,
    &RAIN_OF_FILTH,
    &RAVENOUS_SKIRGE,
    &RECLUSIVE_WIGHT,
    &REPROCESS,
    &SANGUINE_GUARD,
    &SICKEN,
    &SKIRGE_FAMILIAR,
    &SKITTERING_SKIRGE,
    &SLEEPER_AGENT,
    &SPINED_FLUKE,
    &TAINTED_AETHER,
    &UNNERVE,
    &UNWORTHY_DEAD,
    &VAMPIRIC_EMBRACE,
    &VEBULID,
    &VICTIMIZE,
    &VILE_REQUIEM,
    &WESTERN_PALADIN,
    &WITCH_ENGINE,
    &YAWGMOTH_S_EDICT,
    &YAWGMOTH_S_WILL,
    &ACIDIC_SOIL,
    &ANTAGONISM,
    &ARC_LIGHTNING,
    &BEDLAM,
    &BRAND,
    &BRAVADO,
    &BULWARK,
    &CRATER_HELLION,
    &DESTRUCTIVE_URGE,
    &DISORDER,
    &DROMOSAUR,
    &ELECTRYTE,
    &FALTER,
    &FAULT_LINE,
    &FIERY_MANTLE,
    &FIRE_ANTS,
    &GAMBLE,
    &GOBLIN_CADETS,
    &GOBLIN_LACKEY,
    &GOBLIN_OFFENSIVE,
    &GOBLIN_PATROL,
    &GOBLIN_SPELUNKERS,
    &GOBLIN_WAR_BUGGY,
    &GUMA,
    &HEADLONG_RUSH,
    &HEAT_RAY,
    &LAY_WASTE,
    &LIGHTNING_DRAGON,
    &MELTDOWN,
    &OKK,
    &OUTMANEUVER,
    &RAZE,
    &REFLEXES,
    &RETROMANCER,
    &RUMBLING_CRESCENDO,
    &SCALD,
    &SCORIA_WURM,
    &SCRAP,
    &SHIVAN_HELLKITE,
    &SHIVAN_RAPTOR,
    &SHIV_S_EMBRACE,
    &SHOWER_OF_SPARKS,
    &SNEAK_ATTACK,
    &STEAM_BLAST,
    &SULFURIC_VAPORS,
    &THUNDERING_GIANT,
    &TORCH_SONG,
    &VIASHINO_OUTRIDER,
    &VIASHINO_RUNNER,
    &VIASHINO_SANDSWIMMER,
    &VIASHINO_WEAPONSMITH,
    &VUG_LIZARD,
    &ABUNDANCE,
    &ACRIDIAN,
    &ALBINO_TROLL,
    &ARGOTHIAN_ELDER,
    &ARGOTHIAN_ENCHANTRESS,
    &ARGOTHIAN_SWINE,
    &ARGOTHIAN_WURM,
    &BLANCHWOOD_ARMOR,
    &BLANCHWOOD_TREEFOLK,
    &CARPET_OF_FLOWERS,
    &CAVE_TIGER,
    &CHILD_OF_GAEA,
    &CITANUL_CENTAURS,
    &CITANUL_HIEROPHANTS,
    &CRADLE_GUARD,
    &CROSSWINDS,
    &ELVISH_HERDER,
    &ELVISH_LYRIST,
    &ENDLESS_WURM,
    &EXPLORATION,
    &FECUNDITY,
    &FERTILE_GROUND,
    &FORTITUDE,
    &GAEA_S_BOUNTY,
    &GAEA_S_EMBRACE,
    &GREATER_GOOD,
    &GREENER_PASTURES,
    &HAWKEATER_MOTH,
    &HIDDEN_ANCIENTS,
    &HIDDEN_GUERRILLAS,
    &HIDDEN_HERD,
    &HIDDEN_PREDATORS,
    &HIDDEN_SPIDER,
    &HIDDEN_STAG,
    &HUSH,
    &LULL,
    &MIDSUMMER_REVEL,
    &POUNCING_JAGUAR,
    &PRIEST_OF_TITANIA,
    &REJUVENATE,
    &RETALIATION,
    &SPOROGENESIS,
    &SPREADING_ALGAE,
    &SYMBIOSIS,
    &TITANIA_S_BOON,
    &TITANIA_S_CHOSEN,
    &TREEFOLK_SEEDLINGS,
    &TREETOP_RANGERS,
    &VENOMOUS_FANGS,
    &VERNAL_BLOOM,
    &WAR_DANCE,
    &WHIRLWIND,
    &WILD_DOGS,
    &WINDING_WURM,
    &BARRIN_S_CODEX,
    &CATHODION,
    &CHIMERIC_STAFF,
    &CITANUL_FLUTE,
    &CLAWS_OF_GIX,
    &COPPER_GNOMES,
    &CRYSTAL_CHIMES,
    &DRAGON_BLOOD,
    &ENDOSKELETON,
    &FLUCTUATOR,
    &GRAFTED_SKULLCAP,
    &HOPPING_AUTOMATON,
    &KARN_SILVER_GOLEM,
    &LIFELINE,
    &LOTUS_BLOSSOM,
    &METROGNOME,
    &MISHRA_S_HELIX,
    &MOBILE_FORT,
    &NOETIC_SCALES,
    &PHYREXIAN_COLOSSUS,
    &PHYREXIAN_PROCESSOR,
    &PURGING_SCYTHE,
    &SMOKESTACK,
    &TEMPORAL_APERTURE,
    &THRAN_TURBINE,
    &UMBILICUS,
    &URZA_S_ARMOR,
    &VOLTAIC_KEY,
    &WALL_OF_JUNK,
    &WHETSTONE,
    &WIRECAT,
    &WORN_POWERSTONE,
    &BLASTED_LANDSCAPE,
    &DRIFTING_MEADOW,
    &GAEAS_CRADLE,
    &PHYREXIAN_TOWER,
    &POLLUTED_MIRE,
    &REMOTE_ISLE,
    &SERRA_S_SANCTUM,
    &SHIVAN_GORGE,
    &SLIPPERY_KARST,
    &SMOLDERING_CRATER,
    &THRAN_QUARRY,
    &TOLARIAN_ACADEMY,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    DISENCHANT_REPRINT,
    HEALING_SALVE_REPRINT,
    PACIFISM_REPRINT,
    PATH_OF_PEACE_REPRINT,
    PRESENCE_OF_THE_MASTER_REPRINT,
    SEASONED_MARSHAL_REPRINT,
    ENCHANTMENT_ALTERATION_REPRINT,
    EXHAUSTION_REPRINT,
    POWER_SINK_REPRINT,
    BOG_RAIDERS_REPRINT,
    BOG_RAIDERS_ALTERNATE_1,
    DARK_RITUAL_REPRINT,
    DIABOLIC_SERVITUDE_ALTERNATE_1,
    LOOMING_SHADE_ALTERNATE_1,
    NO_REST_FOR_THE_WICKED_ALTERNATE_1,
    PESTILENCE_REPRINT,
    UNWORTHY_DEAD_ALTERNATE_1,
    VAMPIRIC_EMBRACE_ALTERNATE_1,
    FIRE_ANTS_ALTERNATE_1,
    GOBLIN_MATRON_REPRINT,
    GOBLIN_RAIDER_REPRINT,
    JAGGED_LIGHTNING_REPRINT,
    RAIN_OF_SALT_REPRINT,
    WILDFIRE_REPRINT,
    ANACONDA_REPRINT,
    BULL_HIPPO_REPRINT,
    GORILLA_WARRIOR_REPRINT,
    PIT_TRAP_REPRINT,
    PLAINS_REPRINT,
    PLAINS_ALTERNATE_1,
    PLAINS_ALTERNATE_2,
    PLAINS_ALTERNATE_3,
    ISLAND_REPRINT,
    ISLAND_ALTERNATE_1,
    ISLAND_ALTERNATE_2,
    ISLAND_ALTERNATE_3,
    SWAMP_REPRINT,
    SWAMP_ALTERNATE_1,
    SWAMP_ALTERNATE_2,
    SWAMP_ALTERNATE_3,
    MOUNTAIN_REPRINT,
    MOUNTAIN_ALTERNATE_1,
    MOUNTAIN_ALTERNATE_2,
    MOUNTAIN_ALTERNATE_3,
    FOREST_REPRINT,
    FOREST_ALTERNATE_1,
    FOREST_ALTERNATE_2,
    FOREST_ALTERNATE_3,
];
