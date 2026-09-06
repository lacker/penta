//! Alliances cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::CostQuantityDef;
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef, AlternativeCastKindDef,
    AppliedEffectDef, AppliedRuleDef, BattlefieldEntryScalarChoiceDef, BlockRestrictionDef,
    CardArt, CardRules, CardSet, CardSupertype, CardType, ControlDurationDef, CostDef, CounterKind,
    CreatedTokensDef, DamageAssignmentDef, DamageEventMatcherDef, DamagePreventionDef,
    DividedTotal, EffectDef, EffectPaymentDef, EffectRecipientDef, InstalledTriggerDef, ManaColor,
    ManaTypeDef, ObjectPredicateDef, ObjectRefDef, ObjectSetDef, PayOrDef, PlayerRefDef,
    PlayerRelation, PlayerSetDef, ReplacementChoiceDef, ReplacementEffectDef,
    ResolvedEffectDurationDef, SacrificedAmountDef, ScaledValueDef, SumValueDef, TargetChooserDef,
    TokenCharacteristics, TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef,
    ZoneChangeEventMatcherDef, ZoneKind, ZonePlacement, abilities,
};
use crate::{AdditionalCostIndex, TargetIndex, mana_cost};

// ALL 1a — Carrier Pigeons
pub(in crate::card::sets) static CARRIER_PIGEONS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5543b08d-d470-435e-83d9-a3a84c1cc2e6"),
    "Carrier Pigeons",
    crate::card::CardArt::new("5543b08d-d470-435e-83d9-a3a84c1cc2e6", "Pat Lewis"),
    crate::card::CardSet::Alliances,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Bird"], 1, 1).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, draw a card at the beginning of the next turn's upkeep.",
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
        ),
    ]),
);

// ALL 1b — Carrier Pigeons (alternate printing)

// ALL 2a — Errand of Duty
pub(in crate::card::sets) static ERRAND_OF_DUTY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6d3c539b-4039-45c2-8d43-80648d946e91"),
    "Errand of Duty",
    crate::card::CardArt::new("6d3c539b-4039-45c2-8d43-80648d946e91", "Julie Baroh"),
    crate::card::CardSet::Alliances,
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell(
        "Create a 1/1 white Knight creature token with banding.",
        EffectDef::create_creature_token(&["Knight"], &[ManaColor::White], 1, 1)
            .with_abilities(&[abilities::banding()]),
    )),
);

// ALL 2b — Errand of Duty (alternate printing)

// ALL 3 — Exile
pub(in crate::card::sets) static EXILE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("108b85ff-ed03-4b3e-872f-1cad1a27b930"),
    "Exile",
    crate::card::CardArt::new("108b85ff-ed03-4b3e-872f-1cad1a27b930", "Rob Alexander"),
    crate::card::CardSet::Alliances,
    CardRules::new_instant(mana_cost!("{2}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Exile target nonwhite attacking creature. You gain life equal to its toughness.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Attacking,
                ObjectPredicateDef::Not(&ObjectPredicateDef::Color(ManaColor::White)),
            ]),
        )],
        EffectDef::Sequence(&[
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::TargetToughness(TargetIndex::PRIMARY),
            },
        ]),
    )),
);

// ALL 4 — Inheritance
pub(in crate::card::sets) static INHERITANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9fe88de7-b226-4a43-9662-8b408e4281d3"),
    "Inheritance",
    crate::card::CardArt::new("9fe88de7-b226-4a43-9662-8b408e4281d3", "Kaja Foglio"),
    crate::card::CardSet::Alliances,
    CardRules::new_enchantment(mana_cost!("{W}")).with_ability(abilities::dies_trigger_matching(
        "Whenever a creature dies, you may pay {3}. If you do, draw a card.",
        ObjectPredicateDef::HasType(CardType::Creature),
        EffectDef::PayOr(PayOrDef::optional(
            EffectPaymentDef::mana(
                PlayerSetDef::Related(PlayerRelation::You),
                mana_cost!("{3}"),
            ),
            &EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        )),
    )),
);

// ALL 5 — Ivory Gargoyle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static IVORY_GARGOYLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("365820e4-7b43-423b-98ce-f383eb4d2a96"),
    "Ivory Gargoyle",
    crate::card::CardArt::new("365820e4-7b43-423b-98ce-f383eb4d2a96", "Quinton Hoover"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 6 — Juniper Order Advocate
pub(in crate::card::sets) static JUNIPER_ORDER_ADVOCATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9185d10f-7368-4b40-b4a6-baf46c616c34"),
    "Juniper Order Advocate",
    crate::card::CardArt::new("9185d10f-7368-4b40-b4a6-baf46c616c34", "Douglas Shuler"),
    crate::card::CardSet::Alliances,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Knight"], 1, 2).with_ability(
        AbilityDef::static_ability(
            "As long as this creature is untapped, green creatures you control get +1/+1.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::SourceUntapped,
                then: &EffectDef::StaticApply {
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
                },
            },
        ),
    ),
);

// ALL 7a — Kjeldoran Escort
pub(in crate::card::sets) static KJELDORAN_ESCORT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0fd7536a-5417-4de0-9a48-82a5f82d9af4"),
    "Kjeldoran Escort",
    crate::card::CardArt::new("0fd7536a-5417-4de0-9a48-82a5f82d9af4", "Bryon Wackwitz"),
    crate::card::CardSet::Alliances,
    CardRules::new_creature(mana_cost!("{2}{W}{W}"), &["Human", "Soldier"], 2, 3)
        .with_abilities(&[abilities::banding()]),
);

// ALL 7b — Kjeldoran Escort (alternate printing)

// ALL 8 — Kjeldoran Home Guard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KJELDORAN_HOME_GUARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("794d16f9-848f-44ca-8e85-d01a58558077"),
    "Kjeldoran Home Guard",
    crate::card::CardArt::new("794d16f9-848f-44ca-8e85-d01a58558077", "Andi Rusu"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 9a — Kjeldoran Pride (alternate printing)

// ALL 9b — Kjeldoran Pride
pub(in crate::card::sets) static KJELDORAN_PRIDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a88d1c1a-b53e-459b-8a83-4d559177188a"),
    "Kjeldoran Pride",
    crate::card::CardArt::new("a88d1c1a-b53e-459b-8a83-4d559177188a", "Kaja Foglio"),
    crate::card::CardSet::Alliances,
    CardRules::new_enchantment(mana_cost!("{1}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +1/+2.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(2),
                    ),
                },
            ),
            AbilityDef::activated_with_targets(
                "{2}{U}: Attach this Aura to target creature other than enchanted creature.",
                &[CostDef::Mana(mana_cost!("{2}{U}"))],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::AttachedToSource),
                    ]),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
        ]),
);

// ALL 10a — Martyrdom (alternate printing)

// ALL 10b — Martyrdom
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MARTYRDOM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("07f91817-8e79-4885-a57b-d26241c4791f"),
    "Martyrdom",
    crate::card::CardArt::new("07f91817-8e79-4885-a57b-d26241c4791f", "Mark Poole"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 11a — Noble Steeds
pub(in crate::card::sets) static NOBLE_STEEDS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("45a35751-a232-40ba-a73b-d3ca7a44867d"),
    "Noble Steeds",
    crate::card::CardArt::new("45a35751-a232-40ba-a73b-d3ca7a44867d", "Rebecca Guay"),
    crate::card::CardSet::Alliances,
    CardRules::new_enchantment(mana_cost!("{2}{W}")).with_ability(
        AbilityDef::activated_with_targets(
            "{1}{W}: Target creature gains first strike until end of turn.",
            &[CostDef::Mana(mana_cost!("{1}{W}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ALL 11b — Noble Steeds (alternate printing)

// ALL 12a — Reinforcements (alternate printing)

// ALL 12b — Reinforcements
pub(in crate::card::sets) static REINFORCEMENTS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c0b26881-3ad7-4d70-8051-a7e222d910bf"),
    "Reinforcements",
    CardArt::new("c0b26881-3ad7-4d70-8051-a7e222d910bf", "Diana Vick"),
    CardSet::Alliances,
    // One mana to put three creatures back on top, which is three turns of
    // draws already decided.
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell_with_targets(
        "Put up to three target creature cards from your graveyard on top of your library.",
        // "Up to three", so it still casts with an empty graveyard, and the
        // cards are yours by ownership rather than control.
        &[AbilityTargetDef::up_to(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            },
            3,
        )],
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Library,
            placement: ZonePlacement::Top,
        },
    )),
);

// ALL 13a — Reprisal
pub(in crate::card::sets) static REPRISAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("179f50be-6658-42f4-b9b9-c97c7d3f239a"),
    "Reprisal",
    crate::card::CardArt::new(
        "179f50be-6658-42f4-b9b9-c97c7d3f239a",
        "Randy Asplund-Faith",
    ),
    crate::card::CardSet::Alliances,
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target creature with power 4 or greater. It can't be regenerated.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::PowerAtLeast(4),
            ]),
        )],
        EffectDef::WithRule {
            rule: AppliedRuleDef::CannotRegenerate,
            effect: &EffectDef::destroy_target(TargetIndex::PRIMARY),
        },
    )),
);

// ALL 13b — Reprisal (alternate printing)

// ALL 14 — Royal Decree
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROYAL_DECREE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d22231f5-30af-4f46-b2c9-0b71124c6939"),
    "Royal Decree",
    crate::card::CardArt::new("d22231f5-30af-4f46-b2c9-0b71124c6939", "Pete Venters"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 15a — Royal Herbalist
// Audit: unsupported — Needs an exile-from-library activation cost. CostDef can mill cards but not exile them, and "exile the top card of your library" as a cost has no variant.
pub(in crate::card::sets) static ROYAL_HERBALIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("027e03e1-1a39-47ba-b206-44d022b4c346"),
    "Royal Herbalist",
    crate::card::CardArt::new("027e03e1-1a39-47ba-b206-44d022b4c346", "Douglas Shuler"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 15b — Royal Herbalist (alternate printing)

// ALL 16 — Scars of the Veteran
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCARS_OF_THE_VETERAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("632870c3-7c0b-48ad-865d-95f8c4e887d0"),
    "Scars of the Veteran",
    crate::card::CardArt::new("632870c3-7c0b-48ad-865d-95f8c4e887d0", "Dan Frazier"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 17 — Seasoned Tactician
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEASONED_TACTICIAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f8be4b6b-23a2-42d2-911d-fa14f7f5a95b"),
    "Seasoned Tactician",
    crate::card::CardArt::new("f8be4b6b-23a2-42d2-911d-fa14f7f5a95b", "Dan Frazier"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 18 — Sustaining Spirit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUSTAINING_SPIRIT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c9ecf91a-9ce1-44a1-8859-7163d32cfba6"),
    "Sustaining Spirit",
    crate::card::CardArt::new("c9ecf91a-9ce1-44a1-8859-7163d32cfba6", "Rebecca Guay"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 19 — Sworn Defender
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SWORN_DEFENDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("328e6ceb-30f7-415e-93b4-7075af0fed89"),
    "Sworn Defender",
    crate::card::CardArt::new(
        "328e6ceb-30f7-415e-93b4-7075af0fed89",
        "D. Alexander Gregory",
    ),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 20 — Unlikely Alliance
pub(in crate::card::sets) static UNLIKELY_ALLIANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c14d2c73-1934-4504-bbfb-62ba82e0a0e3"),
    "Unlikely Alliance",
    crate::card::CardArt::new("c14d2c73-1934-4504-bbfb-62ba82e0a0e3", "Phil Foglio"),
    crate::card::CardSet::Alliances,
    CardRules::new_enchantment(mana_cost!("{1}{W}")).with_ability(
        AbilityDef::activated_with_targets(
            "{1}{W}: Target nonattacking, nonblocking creature gets +0/+2 until end of turn.",
            &[CostDef::Mana(mana_cost!("{1}{W}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Attacking),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Blocking),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ALL 21a — Wild Aesthir
pub(in crate::card::sets) static WILD_AESTHIR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dd0decda-d77a-4b7b-8ca4-08528d476f51"),
    "Wild Aesthir",
    crate::card::CardArt::new("dd0decda-d77a-4b7b-8ca4-08528d476f51", "Greg Simanson"),
    crate::card::CardSet::Alliances,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Bird"], 1, 1).with_abilities(&[
        abilities::flying(),
        abilities::first_strike(),
        AbilityDef::activated(
            "{W}{W}: This creature gets +2/+0 until end of turn. Activate only once each turn.",
            &[CostDef::Mana(mana_cost!("{W}{W}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .once_each_turn(),
    ]),
);

// ALL 21b — Wild Aesthir (alternate printing)

// ALL 22a — Arcane Denial
/// "Up to two" is two questions rather than one number: take the first card,
/// then decide about the second. The reachable answers -- none, one, or both
/// -- are the ones the printed card offers.
static DENIED_CONTROLLER: EffectRecipientDef = EffectRecipientDef::player(
    PlayerRefDef::ControllerOf(ObjectRefDef::Target(TargetIndex::PRIMARY)),
);

pub(in crate::card::sets) static ARCANE_DENIAL: CardRecord = CardRecord::new_with_legacy_id(
    2061,
    "Arcane Denial",
    CardArt::new("b0c5728e-4a52-4d2f-9b04-3c1c7d3f5e6a", "Richard Kane Ferguson"),
    CardSet::Alliances,
    // Two mana to answer anything, and the cards it gives back arrive a turn
    // too late to matter in a deck that is about to lock the game up.
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target spell. Its controller may draw up to two cards at the beginning of the next turn's upkeep.\nYou draw a card at the beginning of the next turn's upkeep.",
        &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::Spell,
            zones: &[ZoneKind::Stack],
            controller: None,
            owner: None,
        })],
        EffectDef::Sequence(&[
            EffectDef::Counter {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Graveyard,
                placement: ZonePlacement::Top,
            },
            // Both draws are delayed to the next upkeep, which is what makes the card a
            // real counterspell rather than a gift: the two cards arrive a turn later,
            // and by then the spell it answered is long gone.
            EffectDef::Sequence(&[
                EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                    "At the beginning of the next turn's upkeep, that spell's controller may draw up to two cards.",
                    TriggerEventDef::StepBegins {
                        step: TurnStepDef::Upkeep,
                        player: PlayerRelation::Any,
                    },
                    EffectDef::May {
                        player: DENIED_CONTROLLER,
                        effect: &EffectDef::Sequence(&[
                            EffectDef::DrawCards {
                                recipient: DENIED_CONTROLLER,
                                amount: ValueDef::Constant(1),
                            },
                            EffectDef::May {
                                player: DENIED_CONTROLLER,
                                effect: &EffectDef::DrawCards {
                                    recipient: DENIED_CONTROLLER,
                                    amount: ValueDef::Constant(1),
                                },
                            },
                        ]),
                    },
                ))),
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
        ]),
    )),
);

// ALL 22b — Arcane Denial (alternate printing)

// ALL 23a — Awesome Presence (alternate printing)

// ALL 23b — Awesome Presence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AWESOME_PRESENCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0aa8a120-5c13-4852-bdc8-80ae50a6e3d3"),
    "Awesome Presence",
    crate::card::CardArt::new("0aa8a120-5c13-4852-bdc8-80ae50a6e3d3", "Lawrence Snelly"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 24a — Benthic Explorers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BENTHIC_EXPLORERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("146eb650-92c8-48a9-a40d-e7bba6545f36"),
    "Benthic Explorers",
    crate::card::CardArt::new("146eb650-92c8-48a9-a40d-e7bba6545f36", "Greg Simanson"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 24b — Benthic Explorers (alternate printing)

// ALL 25 — Browse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BROWSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("578549f0-5643-4891-b467-2d1cb49fe4ea"),
    "Browse",
    crate::card::CardArt::new("578549f0-5643-4891-b467-2d1cb49fe4ea", "Phil Foglio"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 26 — Diminishing Returns
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIMINISHING_RETURNS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a375ec24-4841-4792-ad58-f29cdf0d1bbb"),
    "Diminishing Returns",
    crate::card::CardArt::new("a375ec24-4841-4792-ad58-f29cdf0d1bbb", "Allen Williams"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 27a — False Demise
pub(in crate::card::sets) static FALSE_DEMISE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("69773e2b-bfee-449d-b8e8-5646442f5487"),
    "False Demise",
    crate::card::CardArt::new("69773e2b-bfee-449d-b8e8-5646442f5487", "Randy Gallegos"),
    crate::card::CardSet::Alliances,
    CardRules::new_enchantment(mana_cost!("{2}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::triggered(
                "When enchanted creature dies, return that card to the battlefield under your control.",
                TriggerEventDef::ZoneChanged(ZoneChangeEventMatcherDef::new(
                    ObjectPredicateDef::AttachedToSource,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                )),
                EffectDef::WithBattlefieldArrival {
                    effect: &EffectDef::MoveToZone {
                        object: EffectRecipientDef::TriggeringZoneChangeResult,
                        zone: ZoneKind::Battlefield,
                        placement: ZonePlacement::Top,
                    },
                    arrival: crate::card::BattlefieldArrivalDef {
                        controller: Some(PlayerRelation::You),
                        ..crate::card::BattlefieldArrivalDef::DEFAULT
                    },
                },
            ),
        ]),
);

// ALL 27b — False Demise (alternate printing)

// ALL 28 — Force of Will
pub(in crate::card::sets) static FORCE_OF_WILL: CardRecord = CardRecord::new_with_legacy_id(
    2174,
    "Force of Will",
    CardArt::new("9a879b60-4381-447d-8a5a-8e0b6a1d49ca", "Terese Nielsen"),
    CardSet::Alliances,
    // Answering a spell for no mana is what makes an entire format possible:
    // a deck can tap out and still not be dead to the one card that would
    // have beaten it.
    CardRules::new_instant(mana_cost!("{3}{U}{U}")).with_abilities(&[
        AbilityDef::alternative_cast(
            mana_cost!("{0}"),
            AlternativeCastKindDef::AlternativeCost,
            Some(
                "You may pay 1 life and exile a blue card from your hand rather than pay this \
                 spell's mana cost.",
            ),
            EffectDef::None,
        )
        .with_alternative_additional_cost(&CostDef::exile(
            ObjectPredicateDef::Color(ManaColor::Blue),
            ZoneKind::Hand,
            CostQuantityDef::Fixed(1),
        ))
        .with_alternative_life(1),
        AbilityDef::spell_with_targets(
            "Counter target spell.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Spell,
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::counter_target(TargetIndex::PRIMARY),
        ),
    ]),
);

// ALL 29a — Foresight
pub(in crate::card::sets) static FORESIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a12e624c-8879-4e60-a1be-286abc5e0106"),
    "Foresight",
    crate::card::CardArt::new("a12e624c-8879-4e60-a1be-286abc5e0106", "Terese Nielsen"),
    crate::card::CardSet::Alliances,
    CardRules::new_sorcery(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell(
        "Search your library for three cards, exile them, then shuffle.\nDraw a card at the beginning of the next turn's upkeep.",
        EffectDef::Sequence(&[
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::Any,
                minimum: 3,
                maximum: ValueDef::Constant(3),
                reveal: false,
                destination: ZoneKind::Exile,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: false,
                attachment: None,
                binding: None,
                then: None,
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

// ALL 29b — Foresight (alternate printing)

// ALL 30a — Lat-Nam's Legacy (alternate printing)

// ALL 30b — Lat-Nam's Legacy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAT_NAM_S_LEGACY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cd3b0741-dd5e-4d98-a50b-19a0f20dd72c"),
    "Lat-Nam's Legacy",
    crate::card::CardArt::new("cd3b0741-dd5e-4d98-a50b-19a0f20dd72c", "Tom Wänerstrand"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 31 — Library of Lat-Nam
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIBRARY_OF_LAT_NAM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5f5fa739-e8d4-4e1d-8b6b-c334d1e91bef"),
    "Library of Lat-Nam",
    crate::card::CardArt::new("5f5fa739-e8d4-4e1d-8b6b-c334d1e91bef", "Alan Rabinowitz"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 32 — Phantasmal Sphere
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHANTASMAL_SPHERE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a84617c7-c70a-497c-b834-3d98346180cf"),
    "Phantasmal Sphere",
    crate::card::CardArt::new("a84617c7-c70a-497c-b834-3d98346180cf", "Mark Tedin"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 33a — Soldevi Heretic (alternate printing)

// ALL 33b — Soldevi Heretic
pub(in crate::card::sets) static SOLDEVI_HERETIC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9613ca47-c9d1-4485-b0bd-71b0b587567e"),
    "Soldevi Heretic",
    crate::card::CardArt::new("9613ca47-c9d1-4485-b0bd-71b0b587567e", "Mike Kimble"),
    crate::card::CardSet::Alliances,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Cleric"], 2, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{W}, {T}: Prevent the next 2 damage that would be dealt to target creature this turn. Target opponent may draw a card.",
            &[
                CostDef::Mana(mana_cost!("{W}")),
                CostDef::TapSource,
            ],
            &[
                AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                    CardType::Creature,
                )),
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
                    PlayerRelation::Opponent,
                )),
            ],
            EffectDef::Sequence(&[
                EffectDef::PreventDamage {
                    prevention: DamagePreventionDef::amount(
                        DamageEventMatcherDef::to(EffectRecipientDef::Target(
                            TargetIndex::PRIMARY,
                        )),
                        ValueDef::Constant(2),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::May {
                    player: EffectRecipientDef::Target(TargetIndex(1)),
                    effect: &EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Target(TargetIndex(1)),
                        amount: ValueDef::Constant(1),
                    },
                },
            ]),
        ),
    ),
);

// ALL 34a — Soldevi Sage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOLDEVI_SAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("07392841-2df5-47f1-9868-edae3376e35a"),
    "Soldevi Sage",
    crate::card::CardArt::new("07392841-2df5-47f1-9868-edae3376e35a", "Carol Heyer"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 34b — Soldevi Sage (alternate printing)

// ALL 35 — Spiny Starfish
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPINY_STARFISH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c4242dda-6078-481d-a068-e7b10c873b89"),
    "Spiny Starfish",
    crate::card::CardArt::new("c4242dda-6078-481d-a068-e7b10c873b89", "Alan Rabinowitz"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 36a — Storm Crow (alternate printing)

// ALL 36b — Storm Crow
pub(in crate::card::sets) static STORM_CROW: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2dbf72f7-2360-4105-beae-946556884e40"),
    "Storm Crow",
    crate::card::CardArt::new("2dbf72f7-2360-4105-beae-946556884e40", "Sandra Everingham"),
    crate::card::CardSet::Alliances,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Bird"], 1, 2)
        .with_abilities(&[abilities::flying()]),
);

// ALL 37 — Storm Elemental
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STORM_ELEMENTAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("24de2b5e-78b1-490d-ac47-67f7076bc6b6"),
    "Storm Elemental",
    crate::card::CardArt::new("24de2b5e-78b1-490d-ac47-67f7076bc6b6", "John Matson"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 38 — Suffocation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUFFOCATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d22104df-8147-45fd-897a-f99a815be062"),
    "Suffocation",
    crate::card::CardArt::new("d22104df-8147-45fd-897a-f99a815be062", "Allen Williams"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 39 — Thought Lash
pub(in crate::card::sets) static THOUGHT_LASH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d59bbac1-ca51-4c72-9f1f-5fc6c82a4a27"),
    "Thought Lash",
    crate::card::CardArt::new("d59bbac1-ca51-4c72-9f1f-5fc6c82a4a27", "Mark Tedin"),
    crate::card::CardSet::Alliances,
    CardRules::new_enchantment(mana_cost!("{2}{U}{U}")).with_abilities(&[
        abilities::cumulative_upkeep(CostDef::exile_top_cards(1))
            .override_text("Cumulative upkeep—Exile the top card of your library."),
        AbilityDef::triggered(
            "When this enchantment's cumulative upkeep isn't paid, exile all cards from your library.",
            TriggerEventDef::CumulativeUpkeepNotPaid,
            EffectDef::MoveToZone {
                object: EffectRecipientDef::objects(ObjectSetDef::Query(
                    crate::card::ObjectQueryDef::owned_by(
                        ObjectPredicateDef::Any,
                        &[ZoneKind::Library],
                        PlayerSetDef::Related(PlayerRelation::You),
                    ),
                )),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
            },
        ),
        AbilityDef::activated(
            "Exile the top card of your library: Prevent the next 1 damage that would be dealt to you this turn.",
            &[CostDef::ExileTopCards(1)],
            EffectDef::PreventDamage {
                prevention: DamagePreventionDef::amount(
                    DamageEventMatcherDef::to(EffectRecipientDef::Controller),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// ALL 40 — Tidal Control
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TIDAL_CONTROL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cb9a7b7d-3d37-4bb6-ab48-1fec2bfb4fdc"),
    "Tidal Control",
    crate::card::CardArt::new("cb9a7b7d-3d37-4bb6-ab48-1fec2bfb4fdc", "Randy Gallegos"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 41a — Viscerid Armor (alternate printing)

// ALL 41b — Viscerid Armor
pub(in crate::card::sets) static VISCERID_ARMOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b719f89d-2a2c-460c-95e4-ada21353b340"),
    "Viscerid Armor",
    crate::card::CardArt::new("b719f89d-2a2c-460c-95e4-ada21353b340", "Heather Hudson"),
    crate::card::CardSet::Alliances,
    CardRules::new_enchantment(mana_cost!("{1}{U}"))
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
                "{1}{U}: Return this Aura to its owner's hand.",
                &[CostDef::Mana(mana_cost!("{1}{U}"))],
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Source,
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                },
            ),
        ]),
);

// ALL 42 — Viscerid Drone
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VISCERID_DRONE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2ccd245f-e374-4bb8-8ac9-743b27ecf817"),
    "Viscerid Drone",
    crate::card::CardArt::new("2ccd245f-e374-4bb8-8ac9-743b27ecf817", "Heather Hudson"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 43 — Balduvian Dead
pub(in crate::card::sets) static BALDUVIAN_DEAD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fac1875a-feab-4213-aa15-69892b7df58b"),
    "Balduvian Dead",
    crate::card::CardArt::new("fac1875a-feab-4213-aa15-69892b7df58b", "Mike Kimble"),
    crate::card::CardSet::Alliances,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Zombie"], 2, 3).with_ability(
        AbilityDef::activated(
            "{2}{R}, Exile a creature card from your graveyard: Create a 3/1 black and red Graveborn creature token with haste. Sacrifice it at the beginning of the next end step.",
            &[
                CostDef::Mana(mana_cost!("{2}{R}")),
                CostDef::MoveToZone(crate::card::MoveToZoneCostDef::new(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ZoneKind::Graveyard,
                    ZoneKind::Exile,
                    1,
                )),
            ],
            EffectDef::create_creature_token(
                &["Graveborn"],
                &[ManaColor::Black, ManaColor::Red],
                3,
                1,
            )
            .with_abilities(&[abilities::haste()])
            .with_created_tokens(CreatedTokensDef {
                binding: crate::ParentBinding,
                then: &EffectDef::InstallTrigger(InstalledTriggerDef::once(
                    &AbilityDef::triggered(
                        "At the beginning of the next end step, sacrifice that token.",
                        TriggerEventDef::StepBegins {
                            step: TurnStepDef::End,
                            player: PlayerRelation::Any,
                        },
                        EffectDef::Sacrifice {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                crate::ParentBinding,
                            )),
                        },
                    ),
                )),
            }),
        ),
    ),
);

// ALL 44a — Casting of Bones (alternate printing)

// ALL 44b — Casting of Bones
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CASTING_OF_BONES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("88442ddf-c12b-4a25-804d-29fef5a90a0c"),
    "Casting of Bones",
    crate::card::CardArt::new("88442ddf-c12b-4a25-804d-29fef5a90a0c", "Anson Maddocks"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 45 — Contagion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CONTAGION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("00c8f94a-7690-47f5-b664-61411a32ab74"),
    "Contagion",
    crate::card::CardArt::new("00c8f94a-7690-47f5-b664-61411a32ab74", "Mike Raabe"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 46 — Diseased Vermin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DISEASED_VERMIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("39703080-524d-4aa1-8c58-d512c41ae5d4"),
    "Diseased Vermin",
    crate::card::CardArt::new("39703080-524d-4aa1-8c58-d512c41ae5d4", "Scott Kirschner"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 47 — Dystopia
pub(in crate::card::sets) static DYSTOPIA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5f8bb451-706d-44ff-bbad-9ddc6f9f786a"),
    "Dystopia",
    crate::card::CardArt::new("5f8bb451-706d-44ff-bbad-9ddc6f9f786a", "Ruth Thompson"),
    crate::card::CardSet::Alliances,
    CardRules::new_enchantment(mana_cost!("{1}{B}{B}")).with_abilities(&[
        abilities::cumulative_upkeep(CostDef::life(1)),
        AbilityDef::triggered(
            "At the beginning of each player's upkeep, that player sacrifices a green or white permanent of their choice.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::Any,
            },
            EffectDef::SacrificeOfChoice {
                player: EffectRecipientDef::EventPlayer,
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Color(ManaColor::Green),
                    ObjectPredicateDef::Color(ManaColor::White),
                ]),
                count: ValueDef::Constant(1),
                then: None,
                amount: SacrificedAmountDef::Power,
                otherwise: None,
                optional: false,
            },
        ),
    ]),
);

// ALL 48 — Fatal Lore
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FATAL_LORE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("24ba0b83-9671-4ee7-996d-57a3616b9c66"),
    "Fatal Lore",
    crate::card::CardArt::new("24ba0b83-9671-4ee7-996d-57a3616b9c66", "Lawrence Snelly"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 49a — Feast or Famine
pub(in crate::card::sets) static FEAST_OR_FAMINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7c185b4d-8da5-4b8a-85f0-5f0622c7bade"),
    "Feast or Famine",
    crate::card::CardArt::new("7c185b4d-8da5-4b8a-85f0-5f0622c7bade", "Pete Venters"),
    crate::card::CardSet::Alliances,
    CardRules::new_instant(mana_cost!("{3}{B}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell(
                "Create a 2/2 black Zombie creature token.",
                EffectDef::create_creature_token(&["Zombie"], &[ManaColor::Black], 2, 2),
            ),
            AbilityDef::spell_with_targets(
                "Destroy target nonartifact, nonblack creature. It can't be regenerated.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Artifact)),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Color(ManaColor::Black)),
                    ]),
                )],
                EffectDef::WithRule {
                    rule: AppliedRuleDef::CannotRegenerate,
                    effect: &EffectDef::destroy_target(TargetIndex::PRIMARY),
                },
            ),
        ],
    )),
);

// ALL 49b — Feast or Famine (alternate printing)

// ALL 50a — Fevered Strength
pub(in crate::card::sets) static FEVERED_STRENGTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("13e53d6c-67f5-4d74-8205-6325c75d1d07"),
    "Fevered Strength",
    crate::card::CardArt::new("13e53d6c-67f5-4d74-8205-6325c75d1d07", "Brian Snõddy"),
    crate::card::CardSet::Alliances,
    CardRules::new_instant(mana_cost!("{2}{B}")).with_ability(
        AbilityDef::spell_with_targets(
            "Target creature gets +2/+0 until end of turn.\nDraw a card at the beginning of the next turn's upkeep.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
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
        ),
    ),
);

// ALL 50b — Fevered Strength (alternate printing)

// ALL 51a — Insidious Bookworms
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INSIDIOUS_BOOKWORMS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4bfb7c7e-5a0a-4d4d-be98-ffed0386592b"),
    "Insidious Bookworms",
    crate::card::CardArt::new("4bfb7c7e-5a0a-4d4d-be98-ffed0386592b", "Greg Simanson"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 51b — Insidious Bookworms (alternate printing)

// ALL 52 — Keeper of Tresserhorn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KEEPER_OF_TRESSERHORN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aaf8b0ec-f81a-488c-850c-098a8a3119e5"),
    "Keeper of Tresserhorn",
    crate::card::CardArt::new(
        "aaf8b0ec-f81a-488c-850c-098a8a3119e5",
        "Zak Plucinski & D. Alexander Gregory",
    ),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 53 — Krovikan Horror
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KROVIKAN_HORROR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e1f3cb1c-6bde-4b55-b5bc-5b64b56930f2"),
    "Krovikan Horror",
    crate::card::CardArt::new("e1f3cb1c-6bde-4b55-b5bc-5b64b56930f2", "Christopher Rush"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 54 — Krovikan Plague
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KROVIKAN_PLAGUE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b258e192-20af-4a45-981f-05181f4cd997"),
    "Krovikan Plague",
    crate::card::CardArt::new("b258e192-20af-4a45-981f-05181f4cd997", "Liz Danforth"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 55a — Lim-Dûl's High Guard (alternate printing)

// ALL 55b — Lim-Dûl's High Guard
pub(in crate::card::sets) static LIM_DUL_S_HIGH_GUARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5470fce6-30cf-43bd-a258-a9fde4be0be8"),
    "Lim-Dûl's High Guard",
    crate::card::CardArt::new("5470fce6-30cf-43bd-a258-a9fde4be0be8", "Anson Maddocks"),
    crate::card::CardSet::Alliances,
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Skeleton"], 2, 1).with_abilities(&[
        abilities::first_strike(),
        abilities::regenerate_self(
            "{1}{B}: Regenerate this creature.",
            &[CostDef::Mana(mana_cost!("{1}{B}"))],
        ),
    ]),
);

// ALL 56 — Misinformation
pub(in crate::card::sets) static MISINFORMATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2f8638df-7915-4867-882a-95439486bd7b"),
    "Misinformation",
    CardArt::new(
        "2f8638df-7915-4867-882a-95439486bd7b",
        "Richard Kane Ferguson",
    ),
    CardSet::Alliances,
    // The same effect pointed the other way: it buries their graveyard
    // under their own draws.
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
        "Put up to three target cards from an opponent's graveyard on top of their library in \
         any order.",
        // Any cards, not only creatures, and from their graveyard: it is a
        // way to deny recursion rather than to rebuy anything.
        &[AbilityTargetDef::up_to(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Any,
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::Opponent),
            },
            3,
        )],
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Library,
            placement: ZonePlacement::Top,
        },
    )),
);

// ALL 57a — Phantasmal Fiend (alternate printing)

// ALL 57b — Phantasmal Fiend
pub(in crate::card::sets) static PHANTASMAL_FIEND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2c2842a1-25b8-4c4b-b5f8-496929288ff3"),
    "Phantasmal Fiend",
    crate::card::CardArt::new("2c2842a1-25b8-4c4b-b5f8-496929288ff3", "Scott Kirschner"),
    crate::card::CardSet::Alliances,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Illusion"], 1, 5).with_abilities(&[
        AbilityDef::activated(
            "{B}: This creature gets +1/-1 until end of turn.",
            &[CostDef::Mana(mana_cost!("{B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(-1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated(
            "{1}{U}: Switch this creature's power and toughness until end of turn.",
            &[CostDef::Mana(mana_cost!("{1}{U}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::switch_power_toughness(),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// ALL 58a — Phyrexian Boon (alternate printing)

// ALL 58b — Phyrexian Boon
pub(in crate::card::sets) static PHYREXIAN_BOON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6f82668b-50b3-4746-b7fd-82f8560ebd95"),
    "Phyrexian Boon",
    crate::card::CardArt::new("6f82668b-50b3-4746-b7fd-82f8560ebd95", "Mark Tedin"),
    crate::card::CardSet::Alliances,
    CardRules::new_enchantment(mana_cost!("{2}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+1 as long as it's black. Otherwise, it gets -1/-2.",
                EffectDef::IfElseCondition {
                    condition: &TriggerConditionDef::AttachedPermanentMatches {
                        object: ObjectPredicateDef::Color(ManaColor::Black),
                    },
                    then: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(1),
                        ),
                    },
                    otherwise: &EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(-1),
                            ValueDef::Constant(-2),
                        ),
                    },
                },
            ),
        ]),
);

// ALL 59 — Ritual of the Machine
pub(in crate::card::sets) static RITUAL_OF_THE_MACHINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("537b4109-ae7c-451a-8576-97f817a70d75"),
    "Ritual of the Machine",
    crate::card::CardArt::new("537b4109-ae7c-451a-8576-97f817a70d75", "Anson Maddocks"),
    crate::card::CardSet::Alliances,
    CardRules::new_sorcery(mana_cost!("{2}{B}{B}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, sacrifice a creature.\nGain control of target nonartifact, nonblack creature.",
            &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Artifact)),
                ObjectPredicateDef::Not(&ObjectPredicateDef::Color(ManaColor::Black)),
            ]))],
            CostDef::sacrifice(
                ObjectPredicateDef::HasType(CardType::Creature),
                CostQuantityDef::Fixed(1),
            ),
            EffectDef::GainControl {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                duration: ControlDurationDef::Indefinitely,
                controller: PlayerRefDef::EffectController,
            },
        ),
    ),
);

// ALL 60a — Soldevi Adnate (alternate printing)

// ALL 60b — Soldevi Adnate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOLDEVI_ADNATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("80812871-d9a7-40de-94a5-b854e55409db"),
    "Soldevi Adnate",
    crate::card::CardArt::new("80812871-d9a7-40de-94a5-b854e55409db", "Christopher Rush"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 61a — Stench of Decay (alternate printing)

// ALL 61b — Stench of Decay
pub(in crate::card::sets) static STENCH_OF_DECAY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b4b93845-f17a-4892-a1ce-a4630dced218"),
    "Stench of Decay",
    crate::card::CardArt::new("b4b93845-f17a-4892-a1ce-a4630dced218", "Heather Hudson"),
    crate::card::CardSet::Alliances,
    CardRules::new_instant(mana_cost!("{1}{B}{B}")).with_ability(AbilityDef::spell(
        "Nonartifact creatures get -1/-1 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Artifact)),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(-1),
                ValueDef::Constant(-1),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// ALL 62 — Stromgald Spy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STROMGALD_SPY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0cf8cecc-449f-4cc6-ac4d-440722df0ab9"),
    "Stromgald Spy",
    crate::card::CardArt::new("0cf8cecc-449f-4cc6-ac4d-440722df0ab9", "Zak Plucinski"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 63a — Swamp Mosquito
pub(in crate::card::sets) static SWAMP_MOSQUITO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("21961b79-637a-4aa5-89b5-4e6e9f60d4d1"),
    "Swamp Mosquito",
    CardArt::new("21961b79-637a-4aa5-89b5-4e6e9f60d4d1", "Nicola Leonard"),
    CardSet::Alliances,
    // No power at all, which does not matter: the counter comes from being
    // unblocked rather than from damage.
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Insect"], 0, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever this creature attacks and isn't blocked, defending player gets a poison counter.",
            TriggerEventDef::AttacksAndIsNotBlocked {
                attacker: ObjectPredicateDef::Source,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::DefenderOfSource,
                kind: CounterKind::Poison,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// ALL 63b — Swamp Mosquito (alternate printing)

// ALL 64a — Agent of Stromgald
// Audit: unsupported — The shared mana planner cannot activate a mana ability whose cost itself requires mana.
pub(in crate::card::sets) static AGENT_OF_STROMGALD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4a7506f8-cf09-46ca-ad80-3c398c487ae2"),
    "Agent of Stromgald",
    crate::card::CardArt::new("4a7506f8-cf09-46ca-ad80-3c398c487ae2", "Alan Rabinowitz"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 64b — Agent of Stromgald (alternate printing)

// ALL 65 — Balduvian Horde
// Audit: unsupported — Needs a random discard as a resolving payment. The shared runtime's payment costs include DiscardCards and DiscardMatching, both of which let the payer choose, and "discard a card at random" is a different cost.
pub(in crate::card::sets) static BALDUVIAN_HORDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8e167a6c-05f8-4d90-9f6b-eb0f1046d54a"),
    "Balduvian Horde",
    crate::card::CardArt::new("8e167a6c-05f8-4d90-9f6b-eb0f1046d54a", "Brian Snõddy"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 66a — Balduvian War-Makers
pub(in crate::card::sets) static BALDUVIAN_WAR_MAKERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("12fd561e-6a26-4140-a033-1204f5dda5f3"),
    "Balduvian War-Makers",
    crate::card::CardArt::new("12fd561e-6a26-4140-a033-1204f5dda5f3", "Mike Kimble"),
    crate::card::CardSet::Alliances,
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Human", "Barbarian"], 3, 3)
        .with_abilities(&[abilities::haste(), abilities::rampage(1)]),
);

// ALL 66b — Balduvian War-Makers (alternate printing)

// ALL 67a — Bestial Fury
pub(in crate::card::sets) static BESTIAL_FURY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("626225b9-2cfd-4cf5-b11c-89e5a231b09e"),
    "Bestial Fury",
    crate::card::CardArt::new("626225b9-2cfd-4cf5-b11c-89e5a231b09e", "Mike Raabe"),
    crate::card::CardSet::Alliances,
    CardRules::new_enchantment(mana_cost!("{2}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            abilities::enters_trigger(
                "When this Aura enters, draw a card at the beginning of the next turn's upkeep.",
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
            ),
            AbilityDef::triggered(
                "Whenever enchanted creature becomes blocked, it gets +4/+0 and gains trample until end of turn.",
                TriggerEventDef::BecomesBlocked(ObjectPredicateDef::AttachedToSource),
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(4),
                            ValueDef::Constant(0),
                        ),
                        AppliedEffectDef::add_ability(&abilities::trample()),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// ALL 67b — Bestial Fury (alternate printing)

// ALL 68 — Burnout
pub(in crate::card::sets) static BURNOUT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5a8f5a18-e490-4010-ac1c-c74a5f2dcbda"),
    "Burnout",
    crate::card::CardArt::new("5a8f5a18-e490-4010-ac1c-c74a5f2dcbda", "Mike Raabe"),
    crate::card::CardSet::Alliances,
    CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "Counter target instant spell if it's blue.\nDraw a card at the beginning of the next turn's upkeep.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Spell,
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::Color(ManaColor::Blue),
                ]),
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            })],
            EffectDef::Sequence(&[
                EffectDef::counter_target(TargetIndex::PRIMARY),
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
        ),
    ),
);

// ALL 69 — Chaos Harlequin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHAOS_HARLEQUIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ec7d7c80-4e3c-454e-b2ed-6f0436df19c9"),
    "Chaos Harlequin",
    crate::card::CardArt::new("ec7d7c80-4e3c-454e-b2ed-6f0436df19c9", "Alan Rabinowitz"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 70 — Death Spark
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEATH_SPARK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ba841b44-475c-402c-ac11-763de0cf27d9"),
    "Death Spark",
    crate::card::CardArt::new("ba841b44-475c-402c-ac11-763de0cf27d9", "Mark Tedin"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 71a — Enslaved Scout (alternate printing)

// ALL 71b — Enslaved Scout
pub(in crate::card::sets) static ENSLAVED_SCOUT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aac0e04a-d223-426b-b856-2829dbdffda0"),
    "Enslaved Scout",
    crate::card::CardArt::new("aac0e04a-d223-426b-b856-2829dbdffda0", "Rebecca Guay"),
    crate::card::CardSet::Alliances,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin", "Scout"], 2, 2).with_ability(
        abilities::gain_ability_until_end_of_turn_for_mana(
            "{2}: This creature gains mountainwalk until end of turn.",
            mana_cost!("{2}"),
            &abilities::mountainwalk(),
        ),
    ),
);

// ALL 72a — Gorilla Shaman (alternate printing)

// ALL 72b — Gorilla Shaman
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GORILLA_SHAMAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5a16231c-1f73-4dec-9d88-e3d62e93a70f"),
    "Gorilla Shaman",
    crate::card::CardArt::new("5a16231c-1f73-4dec-9d88-e3d62e93a70f", "Anthony S. Waters"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 73a — Gorilla War Cry
pub(in crate::card::sets) static GORILLA_WAR_CRY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("613762ea-6111-4f74-bea2-c13b76e0751c"),
    "Gorilla War Cry",
    crate::card::CardArt::new("613762ea-6111-4f74-bea2-c13b76e0751c", "Bryon Wackwitz"),
    crate::card::CardSet::Alliances,
    CardRules::new_instant(mana_cost!("{1}{R}"))
        .cast_only_before_blockers_declared()
        .with_abilities(&[
            AbilityDef::enforced_when_cast(
                "Cast this spell only during combat before blockers are declared.",
                "The play option refuses the cast outside combat and once blockers are in.",
            ),
            AbilityDef::spell(
                "All creatures gain menace until end of turn.\nDraw a card at the beginning of the next turn's upkeep.",
                EffectDef::Sequence(&[
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::matching_objects(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Any,
                        ),
                        effect: AppliedEffectDef::add_ability(&abilities::menace()),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                    EffectDef::InstallTrigger(InstalledTriggerDef::once(
                        &AbilityDef::triggered(
                            "At the beginning of the next turn's upkeep, draw a card.",
                            TriggerEventDef::StepBegins {
                                step: TurnStepDef::Upkeep,
                                player: PlayerRelation::Any,
                            },
                            EffectDef::DrawCards {
                                recipient: EffectRecipientDef::Controller,
                                amount: ValueDef::Constant(1),
                            },
                        ),
                    )),
                ]),
            ),
        ]),
);

// ALL 73b — Gorilla War Cry (alternate printing)

// ALL 74a — Guerrilla Tactics (alternate printing)

// ALL 74b — Guerrilla Tactics
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GUERRILLA_TACTICS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3c005ca3-0508-4ac2-afec-3d4a27334c31"),
    "Guerrilla Tactics",
    crate::card::CardArt::new(
        "3c005ca3-0508-4ac2-afec-3d4a27334c31",
        "Randy Asplund-Faith",
    ),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 75 — Omen of Fire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OMEN_OF_FIRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9c724b46-6e17-4bee-9bc6-e9fc5a379dd7"),
    "Omen of Fire",
    crate::card::CardArt::new("9c724b46-6e17-4bee-9bc6-e9fc5a379dd7", "Pete Venters"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 76 — Pillage
pub(in crate::card::sets) static PILLAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("389ecb50-b007-4086-89fb-ec2daa5afdcf"),
    "Pillage",
    crate::card::CardArt::new(
        "389ecb50-b007-4086-89fb-ec2daa5afdcf",
        "Richard Kane Ferguson",
    ),
    crate::card::CardSet::Alliances,
    CardRules::new_sorcery(mana_cost!("{1}{R}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target artifact or land. It can't be regenerated.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Land),
            ]),
        )],
        EffectDef::WithRule {
            rule: AppliedRuleDef::CannotRegenerate,
            effect: &EffectDef::destroy_target(TargetIndex::PRIMARY),
        },
    )),
);

// ALL 77 — Primitive Justice
pub(in crate::card::sets) static PRIMITIVE_JUSTICE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d6b7829b-2a10-47e7-9cf9-8ae49d2b398a"),
    "Primitive Justice",
    crate::card::CardArt::new("d6b7829b-2a10-47e7-9cf9-8ae49d2b398a", "Anthony S. Waters"),
    crate::card::CardSet::Alliances,
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_abilities(&[
        abilities::repeatable_additional_mana_cost(
            "As an additional cost to cast this spell, you may pay {1}{R} any number of times.",
            "{1}{R} additional cost",
            mana_cost!("{1}{R}"),
        ),
        abilities::repeatable_additional_mana_cost(
            "As an additional cost to cast this spell, you may pay {1}{G} any number of times.",
            "{1}{G} additional cost",
            mana_cost!("{1}{G}"),
        ),
        AbilityDef::spell_with_targets(
            "Destroy target artifact. For each additional {1}{R} you paid, destroy another target artifact. For each additional {1}{G} you paid, destroy another target artifact, and you gain 1 life.",
            &[AbilityTargetDef::exactly_value(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Artifact),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                ValueDef::Sum(&SumValueDef::new(
                    ValueDef::Constant(1),
                    ValueDef::Sum(&SumValueDef::new(
                        ValueDef::AdditionalCostPayments(AdditionalCostIndex::PRIMARY),
                        ValueDef::AdditionalCostPayments(AdditionalCostIndex::SECONDARY),
                    )),
                )),
            )],
            EffectDef::Sequence(&[
                EffectDef::Destroy {
                    object: EffectRecipientDef::objects(ObjectSetDef::LegalTargets(
                        TargetIndex::PRIMARY,
                    )),
                    then: None,
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::AdditionalCostPayments(AdditionalCostIndex::SECONDARY),
                },
            ]),
        ),
    ]),
);

// ALL 78 — Pyrokinesis
pub(in crate::card::sets) static PYROKINESIS: CardRecord = CardRecord::new_with_legacy_id(
    2031,
    "Pyrokinesis",
    CardArt::new("db2a5e85-6cbc-43c1-9362-4056ad017ef0", "Ron Spencer"),
    CardSet::Alliances,
    // The free cast is what the card is played for -- a blowout from an empty
    // board -- so the printed cost alone understates it considerably.
    CardRules::new_instant(mana_cost!("{4}{R}{R}")).with_abilities(&[
        AbilityDef::alternative_cast(
            mana_cost!("{0}"),
            AlternativeCastKindDef::AlternativeCost,
            Some("You may exile a red card from your hand rather than pay this spell's mana cost."),
            EffectDef::None,
        )
        // Exiled from hand rather than discarded: the card is spent without ever
        // becoming a graveyard card, which is what "exile a red card" means.
        .with_alternative_additional_cost(&CostDef::exile(
            ObjectPredicateDef::Color(ManaColor::Red),
            ZoneKind::Hand,
            CostQuantityDef::Fixed(1),
        )),
        AbilityDef::spell_with_targets(
            "Pyrokinesis deals 4 damage divided as you choose among any number of target creatures.",
            // Four damage split however the caster likes. There is no printed ceiling on
            // the number of creatures, but the division supplies one anyway: every target
            // must be assigned at least one damage, so four is the most it can ever
            // reach.
            &[AbilityTargetDef {
                predicate: AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[crate::card::ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                minimum: 1,
                maximum: AbilityTargetDef::UNLIMITED,
                exact_count: None,
                divided_total: Some(DividedTotal::Fixed(4)),
                another: false,
                excludes_source: false,
                chooser: TargetChooserDef::Controller,
            }],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::DividedAmongTargets,
            },
        ),
    ]),
);

// ALL 79 — Rogue Skycaptain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROGUE_SKYCAPTAIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("97aebf3b-e77d-4d18-b58b-117ae91792e2"),
    "Rogue Skycaptain",
    crate::card::CardArt::new(
        "97aebf3b-e77d-4d18-b58b-117ae91792e2",
        "Randy Asplund-Faith",
    ),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 80 — Soldier of Fortune
pub(in crate::card::sets) static SOLDIER_OF_FORTUNE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("37c05f46-2081-4ebb-a758-894ac040ea2a"),
    "Soldier of Fortune",
    crate::card::CardArt::new("37c05f46-2081-4ebb-a758-894ac040ea2a", "Douglas Shuler"),
    crate::card::CardSet::Alliances,
    CardRules::new_creature(mana_cost!("{R}"), &["Human", "Mercenary"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{R}, {T}: Target player shuffles their library.",
            &[CostDef::Mana(mana_cost!("{R}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::ShuffleLibrary {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ),
);

// ALL 81a — Storm Shaman (alternate printing)

// ALL 81b — Storm Shaman
pub(in crate::card::sets) static STORM_SHAMAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3a8f1150-6306-42a6-84e1-7dd5bfef6d14"),
    "Storm Shaman",
    crate::card::CardArt::new("3a8f1150-6306-42a6-84e1-7dd5bfef6d14", "Carol Heyer"),
    crate::card::CardSet::Alliances,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Cleric", "Shaman"], 0, 4)
        .with_ability(AbilityDef::activated(
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
        )),
);

// ALL 82a — Varchild's Crusader
pub(in crate::card::sets) static VARCHILD_S_CRUSADER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b5ade7ad-ce32-4296-8cec-20bd79c7b16a"),
    "Varchild's Crusader",
    crate::card::CardArt::new("b5ade7ad-ce32-4296-8cec-20bd79c7b16a", "Mark Poole"),
    crate::card::CardSet::Alliances,
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Human", "Knight"], 3, 2).with_ability(
        AbilityDef::activated(
            "{0}: This creature can't be blocked this turn except by Walls. Sacrifice this creature at the beginning of the next end step.",
            &[],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Wall")),
                    )),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                    "At the beginning of the next end step, sacrifice this creature.",
                    TriggerEventDef::StepBegins {
                        step: TurnStepDef::End,
                        player: PlayerRelation::Any,
                    },
                    EffectDef::Sacrifice {
                        object: EffectRecipientDef::Source,
                    },
                ))),
            ]),
        ),
    ),
);

// ALL 82b — Varchild's Crusader (alternate printing)

// ALL 83 — Varchild's War-Riders
pub(in crate::card::sets) static VARCHILD_S_WAR_RIDERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ee1d41da-aa72-434b-811f-95d4bae4ba5c"),
    "Varchild's War-Riders",
    crate::card::CardArt::new("ee1d41da-aa72-434b-811f-95d4bae4ba5c", "Susan Van Camp"),
    crate::card::CardSet::Alliances,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Warrior"], 3, 4).with_abilities(&[
        abilities::cumulative_upkeep(CostDef::create_tokens(
            PlayerRelation::Opponent,
            &TokenCharacteristics::creature(&["Survivor"], &[ManaColor::Red], 1, 1),
            1,
        ))
        .override_text(
            "Cumulative upkeep—Have an opponent create a 1/1 red Survivor creature token.",
        ),
        abilities::trample(),
        abilities::rampage(1),
    ]),
);

// ALL 84a — Veteran's Voice (alternate printing)

// ALL 84b — Veteran's Voice
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VETERAN_S_VOICE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6e1ecb9a-7443-49cb-8197-ef180124aabb"),
    "Veteran's Voice",
    crate::card::CardArt::new("6e1ecb9a-7443-49cb-8197-ef180124aabb", "Andi Rusu"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 85 — Bounty of the Hunt
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOUNTY_OF_THE_HUNT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("21ed522a-cf5a-41e1-9677-1226f689ec9c"),
    "Bounty of the Hunt",
    crate::card::CardArt::new("21ed522a-cf5a-41e1-9677-1226f689ec9c", "Jeff A. Menges"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 86a — Deadly Insect (alternate printing)

// ALL 86b — Deadly Insect
pub(in crate::card::sets) static DEADLY_INSECT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("030963d9-b59f-4ccb-abed-d817a4bc4e05"),
    "Deadly Insect",
    crate::card::CardArt::new("030963d9-b59f-4ccb-abed-d817a4bc4e05", "Scott Kirschner"),
    crate::card::CardSet::Alliances,
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Insect"], 6, 1)
        .with_abilities(&[abilities::shroud()]),
);

// ALL 87 — Elvish Bard
pub(in crate::card::sets) static ELVISH_BARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("62261004-ed32-4865-824a-4320548f4234"),
    "Elvish Bard",
    crate::card::CardArt::new("62261004-ed32-4865-824a-4320548f4234", "Susan Van Camp"),
    crate::card::CardSet::Alliances,
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Elf", "Shaman", "Bard"], 2, 4)
        .with_ability(AbilityDef::static_ability(
            "All creatures able to block this creature do so.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MustBeBlockedBy(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )),
            },
        )),
);

// ALL 88a — Elvish Ranger (alternate printing)

// ALL 88b — Elvish Ranger
pub(in crate::card::sets) static ELVISH_RANGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7b08a164-d6f4-423e-8666-e4a4c2d21045"),
    "Elvish Ranger",
    crate::card::CardArt::new("7b08a164-d6f4-423e-8666-e4a4c2d21045", "Terese Nielsen"),
    crate::card::CardSet::Alliances,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Elf", "Ranger"], 4, 1),
);

// ALL 89 — Elvish Spirit Guide
pub(in crate::card::sets) static ELVISH_SPIRIT_GUIDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5b94f37f-ebdf-4b79-a615-58331d27cf4e"),
    "Elvish Spirit Guide",
    CardArt::new("5b94f37f-ebdf-4b79-a615-58331d27cf4e", "Julie Baroh"),
    CardSet::Alliances,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Elf", "Spirit"], 2, 2).with_ability(
        AbilityDef::activated_mana(
            "Exile this card from your hand: Add {G}.",
            &[CostDef::ExileSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
        )
        .with_source_zones(&[ZoneKind::Hand]),
    ),
);

// ALL 90a — Fyndhorn Druid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FYNDHORN_DRUID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("778b028f-fa4e-4638-82b4-fb287223ea20"),
    "Fyndhorn Druid",
    crate::card::CardArt::new("778b028f-fa4e-4638-82b4-fb287223ea20", "Rob Alexander"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 90b — Fyndhorn Druid (alternate printing)

// ALL 91 — Gargantuan Gorilla
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GARGANTUAN_GORILLA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("49f367c2-f47e-43e1-9936-4324be664475"),
    "Gargantuan Gorilla",
    crate::card::CardArt::new("49f367c2-f47e-43e1-9936-4324be664475", "Greg Simanson"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 92a — Gift of the Woods (alternate printing)

// ALL 92b — Gift of the Woods
pub(in crate::card::sets) static GIFT_OF_THE_WOODS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6a0df4e9-b201-4fc7-8e37-59d99b583f76"),
    "Gift of the Woods",
    crate::card::CardArt::new("6a0df4e9-b201-4fc7-8e37-59d99b583f76", "Susan Van Camp"),
    crate::card::CardSet::Alliances,
    CardRules::new_enchantment(mana_cost!("{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::triggered(
                "Whenever enchanted creature blocks or becomes blocked, it gets +0/+3 until end of turn and you gain 1 life.",
                TriggerEventDef::BlocksOrBecomesBlockedBy {
                    creature: ObjectPredicateDef::AttachedToSource,
                    other: ObjectPredicateDef::HasType(CardType::Creature),
                },
                EffectDef::Sequence(&[
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(0),
                            ValueDef::Constant(3),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                ]),
            ),
        ]),
);

// ALL 93a — Gorilla Berserkers
pub(in crate::card::sets) static GORILLA_BERSERKERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("344b4613-17f8-4c8b-b5bc-f773a8f8007a"),
    "Gorilla Berserkers",
    crate::card::CardArt::new("344b4613-17f8-4c8b-b5bc-f773a8f8007a", "John Matson"),
    crate::card::CardSet::Alliances,
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Ape", "Berserker"], 2, 3).with_abilities(
        &[
            abilities::trample(),
            abilities::rampage(2),
            AbilityDef::static_ability(
                "This creature can't be blocked except by three or more creatures.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::BlockRestriction(
                        BlockRestrictionDef::MinimumBlockers(3),
                    )),
                },
            ),
        ],
    ),
);

// ALL 93b — Gorilla Berserkers (alternate printing)

// ALL 94a — Gorilla Chieftain
pub(in crate::card::sets) static GORILLA_CHIEFTAIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("47f1eedd-7021-4cce-a808-2e9384a5ef15"),
    "Gorilla Chieftain",
    crate::card::CardArt::new("47f1eedd-7021-4cce-a808-2e9384a5ef15", "Quinton Hoover"),
    crate::card::CardSet::Alliances,
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Ape"], 3, 3).with_ability(
        abilities::regenerate_self(
            "{1}{G}: Regenerate this creature.",
            &[CostDef::Mana(mana_cost!("{1}{G}"))],
        ),
    ),
);

// ALL 94b — Gorilla Chieftain (alternate printing)

// ALL 95 — Hail Storm
pub(in crate::card::sets) static HAIL_STORM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a7e9d786-4e9b-447b-a5dc-ca117c4961c5"),
    "Hail Storm",
    crate::card::CardArt::new("a7e9d786-4e9b-447b-a5dc-ca117c4961c5", "Jeff A. Menges"),
    crate::card::CardSet::Alliances,
    CardRules::new_instant(mana_cost!("{1}{G}{G}")).with_ability(AbilityDef::spell(
        "Hail Storm deals 2 damage to each attacking creature and 1 damage to you and each creature you control.",
        EffectDef::DealDamageSimultaneously(&[
            DamageAssignmentDef::from_effect(
                EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Attacking,
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                ValueDef::Constant(2),
            ),
            DamageAssignmentDef::from_effect(
                EffectRecipientDef::Controller,
                ValueDef::Constant(1),
            ),
            DamageAssignmentDef::from_effect(
                EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                ValueDef::Constant(1),
            ),
        ]),
    )),
);

// ALL 96 — Kaysa
pub(in crate::card::sets) static KAYSA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cd4b6daf-cf37-43c6-9446-3aa0de222ac4"),
    "Kaysa",
    crate::card::CardArt::new("cd4b6daf-cf37-43c6-9446-3aa0de222ac4", "Rebecca Guay"),
    crate::card::CardSet::Alliances,
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Elf", "Druid"], 2, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::static_ability(
            "Green creatures you control get +1/+1.",
            EffectDef::StaticApply {
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
            },
        )),
);

// ALL 97 — Nature's Chosen
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NATURE_S_CHOSEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7bd0b831-9d7e-40ce-8514-e852daee1a9e"),
    "Nature's Chosen",
    crate::card::CardArt::new("7bd0b831-9d7e-40ce-8514-e852daee1a9e", "Rebecca Guay"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 98 — Nature's Wrath
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NATURE_S_WRATH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("450759f0-5d60-4f05-9011-b0b66dbb06a7"),
    "Nature's Wrath",
    crate::card::CardArt::new("450759f0-5d60-4f05-9011-b0b66dbb06a7", "Liz Danforth"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 99 — Splintering Wind
pub(in crate::card::sets) static SPLINTERING_WIND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0afa94e5-fef6-4f3a-9196-d7aa6dd841c2"),
    "Splintering Wind",
    crate::card::CardArt::new("0afa94e5-fef6-4f3a-9196-d7aa6dd841c2", "Ron Spencer"),
    crate::card::CardSet::Alliances,
    CardRules::new_enchantment(mana_cost!("{2}{G}{G}")).with_ability(
        AbilityDef::activated_with_targets(
            "{2}{G}: This enchantment deals 1 damage to target creature. Create a 1/1 green Splinter creature token with flying and 'Cumulative upkeep {G}.' When it leaves the battlefield, it deals 1 damage to you and each creature you control.",
            &[CostDef::Mana(mana_cost!("{2}{G}"))],
            &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                CardType::Creature,
            ))],
            EffectDef::Sequence(&[
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                },
                EffectDef::create_token(
                    TokenCharacteristics::creature(&["Splinter"], &[ManaColor::Green], 1, 1)
                        .with_abilities(&[
                            abilities::flying(),
                            abilities::cumulative_upkeep(CostDef::mana(
                                mana_cost!("{G}"),
                            )),
                            AbilityDef::triggered(
                                "When this creature leaves the battlefield, it deals 1 damage to you and each creature you control.",
                                TriggerEventDef::zone_changed(
                                    ObjectPredicateDef::Source,
                                    Some(ZoneKind::Battlefield),
                                    None,
                                ),
                                EffectDef::DealDamageSimultaneously(&[
                                    DamageAssignmentDef::from_effect(
                                        EffectRecipientDef::Controller,
                                        ValueDef::Constant(1),
                                    ),
                                    DamageAssignmentDef::from_effect(
                                        EffectRecipientDef::matching_objects(
                                            ObjectPredicateDef::HasType(CardType::Creature),
                                            &[ZoneKind::Battlefield],
                                            PlayerRelation::You,
                                        ),
                                        ValueDef::Constant(1),
                                    ),
                                ]),
                            ),
                        ]),
                ),
            ]),
        ),
    ),
);

// ALL 100a — Taste of Paradise (alternate printing)

// ALL 100b — Taste of Paradise
pub(in crate::card::sets) static TASTE_OF_PARADISE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a774c426-ec0e-48de-b00f-5a05cc6dc34b"),
    "Taste of Paradise",
    crate::card::CardArt::new("a774c426-ec0e-48de-b00f-5a05cc6dc34b", "Lawrence Snelly"),
    crate::card::CardSet::Alliances,
    CardRules::new_sorcery(mana_cost!("{3}{G}")).with_abilities(&[
        abilities::repeatable_additional_mana_cost(
            "As an additional cost to cast this spell, you may pay {1}{G} any number of times.",
            "{1}{G} additional cost",
            mana_cost!("{1}{G}"),
        ),
        AbilityDef::spell(
            "You gain 3 life plus an additional 3 life for each additional {1}{G} you paid.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Sum(&SumValueDef::new(
                    ValueDef::Constant(3),
                    ValueDef::Scaled(&ScaledValueDef::new(
                        ValueDef::AdditionalCostPayments(AdditionalCostIndex::PRIMARY),
                        3,
                    )),
                )),
            },
        ),
    ]),
);

// ALL 101 — Tornado
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TORNADO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a2fd58e4-eb9a-4a12-8914-0a9a8300626c"),
    "Tornado",
    crate::card::CardArt::new("a2fd58e4-eb9a-4a12-8914-0a9a8300626c", "Susan Van Camp"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 102a — Undergrowth (alternate printing)

// ALL 102b — Undergrowth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNDERGROWTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7b07df91-49be-4a50-9d3b-ddde0e6c1be9"),
    "Undergrowth",
    crate::card::CardArt::new("7b07df91-49be-4a50-9d3b-ddde0e6c1be9", "Pat Lewis"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 103a — Whip Vine (alternate printing)

// ALL 103b — Whip Vine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WHIP_VINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("31ee1c89-d7df-4ee7-b403-24dfabae38a0"),
    "Whip Vine",
    crate::card::CardArt::new("31ee1c89-d7df-4ee7-b403-24dfabae38a0", "Allen Williams"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 104a — Yavimaya Ancients (alternate printing)

// ALL 104b — Yavimaya Ancients
pub(in crate::card::sets) static YAVIMAYA_ANCIENTS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("91708e45-f9a1-4c2e-973d-bfc294926c93"),
    "Yavimaya Ancients",
    crate::card::CardArt::new("91708e45-f9a1-4c2e-973d-bfc294926c93", "Quinton Hoover"),
    crate::card::CardSet::Alliances,
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Treefolk"], 2, 7).with_ability(
        AbilityDef::activated(
            "{G}: This creature gets +1/-2 until end of turn.",
            &[CostDef::Mana(mana_cost!("{G}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(-2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ALL 105 — Yavimaya Ants
pub(in crate::card::sets) static YAVIMAYA_ANTS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5ded1c83-a289-4951-b72a-477a041610d3"),
    "Yavimaya Ants",
    CardArt::new("5ded1c83-a289-4951-b72a-477a041610d3", "Pat Lewis"),
    CardSet::Alliances,
    // Five hasty power the turn it lands, which is the whole card -- the
    // upkeep is what stops it being one afterwards.
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Insect"], 5, 1).with_abilities(&[
        abilities::trample(),
        abilities::haste(),
        abilities::cumulative_upkeep(CostDef::Mana(mana_cost!("{G}{G}"))).override_text(
                "Cumulative upkeep {G}{G} (At the beginning of your upkeep, put an age counter on this permanent, then sacrifice it unless you pay its upkeep cost for each age counter on it.)",
            ),
    ]),
);

// ALL 106 — Energy Arc
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENERGY_ARC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f81cd99e-902a-44dd-8928-803a96fe25c4"),
    "Energy Arc",
    crate::card::CardArt::new("f81cd99e-902a-44dd-8928-803a96fe25c4", "Terese Nielsen"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 107 — Lim-Dûl's Vault
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIM_DUL_S_VAULT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f9b0164c-2d4e-48ab-addd-322d9b504739"),
    "Lim-Dûl's Vault",
    crate::card::CardArt::new("f9b0164c-2d4e-48ab-addd-322d9b504739", "Rob Alexander"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 108 — Lim-Dûl's Paladin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIM_DUL_S_PALADIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("44be2d66-359e-4cc1-9670-119cb9c7d5f5"),
    "Lim-Dûl's Paladin",
    crate::card::CardArt::new("44be2d66-359e-4cc1-9670-119cb9c7d5f5", "Christopher Rush"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 109 — Surge of Strength
pub(in crate::card::sets) static SURGE_OF_STRENGTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("96fff700-af02-4861-b7ed-be9950e69bf1"),
    "Surge of Strength",
    crate::card::CardArt::new("96fff700-af02-4861-b7ed-be9950e69bf1", "Ruth Thompson"),
    crate::card::CardSet::Alliances,
    CardRules::new_instant(mana_cost!("{R}{G}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, discard a red or green card.\nTarget creature gains trample and gets +X/+0 until end of turn, where X is that creature's mana value.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            CostDef::discard(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Color(ManaColor::Red),
                    ObjectPredicateDef::Color(ManaColor::Green),
                ]),
                CostQuantityDef::Fixed(1),
            ),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::TargetManaValue(TargetIndex::PRIMARY),
                        ValueDef::Constant(0),
                    ),
                    AppliedEffectDef::add_ability(&abilities::trample()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ALL 110 — Nature's Blessing
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NATURE_S_BLESSING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5ba0e677-361d-4e03-9c2c-018d1c383456"),
    "Nature's Blessing",
    crate::card::CardArt::new("5ba0e677-361d-4e03-9c2c-018d1c383456", "Sandra Everingham"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 111 — Wandering Mage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WANDERING_MAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8d9b1b6c-1f02-4918-bb5c-2dbcdb0997ec"),
    "Wandering Mage",
    crate::card::CardArt::new("8d9b1b6c-1f02-4918-bb5c-2dbcdb0997ec", "Pete Venters"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 112 — Lord of Tresserhorn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LORD_OF_TRESSERHORN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5fc9497a-42bf-4d78-afaf-67645514ade4"),
    "Lord of Tresserhorn",
    crate::card::CardArt::new("5fc9497a-42bf-4d78-afaf-67645514ade4", "Anson Maddocks"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 113 — Misfortune
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISFORTUNE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b14cc32a-eb4f-4690-aceb-160780743ebe"),
    "Misfortune",
    crate::card::CardArt::new("b14cc32a-eb4f-4690-aceb-160780743ebe", "Ron Spencer"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 114 — Winter's Night
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WINTER_S_NIGHT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7f020ebc-4950-4407-8cb8-7630cad226f6"),
    "Winter's Night",
    crate::card::CardArt::new("7f020ebc-4950-4407-8cb8-7630cad226f6", "Rob Alexander"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 115 — Phelddagrif
pub(in crate::card::sets) static PHELDDAGRIF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d9631cb2-d53b-4401-b53b-29d27bdefc44"),
    "Phelddagrif",
    crate::card::CardArt::new("d9631cb2-d53b-4401-b53b-29d27bdefc44", "Amy Weber"),
    crate::card::CardSet::Alliances,
    CardRules::new_creature(mana_cost!("{1}{G}{W}{U}"), &["Phelddagrif"], 4, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated_with_targets(
                "{G}: Phelddagrif gains trample until end of turn. Target opponent creates a 1/1 green Hippo creature token.",
                &[CostDef::Mana(mana_cost!("{G}"))],
                &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
                    PlayerRelation::Opponent,
                ))],
                EffectDef::Sequence(&[
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::add_ability(&abilities::trample()),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                    EffectDef::create_creature_token(&["Hippo"], &[ManaColor::Green], 1, 1)
                        .with_controller(PlayerRefDef::Target(TargetIndex::PRIMARY)),
                ]),
            ),
            AbilityDef::activated_with_targets(
                "{W}: Phelddagrif gains flying until end of turn. Target opponent gains 2 life.",
                &[CostDef::Mana(mana_cost!("{W}"))],
                &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
                    PlayerRelation::Opponent,
                ))],
                EffectDef::Sequence(&[
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::add_ability(&abilities::flying()),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(2),
                    },
                ]),
            ),
            AbilityDef::activated_with_targets(
                "{U}: Return Phelddagrif to its owner's hand. Target opponent may draw a card.",
                &[CostDef::Mana(mana_cost!("{U}"))],
                &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
                    PlayerRelation::Opponent,
                ))],
                EffectDef::Sequence(&[
                    EffectDef::MoveToZone {
                        object: EffectRecipientDef::Source,
                        zone: ZoneKind::Hand,
                        placement: ZonePlacement::Top,
                    },
                    EffectDef::May {
                        player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: &EffectDef::DrawCards {
                            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            amount: ValueDef::Constant(1),
                        },
                    },
                ]),
            ),
        ]),
);

// ALL 116a — Aesthir Glider
pub(in crate::card::sets) static AESTHIR_GLIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("35a8080f-ca3c-46fe-81cf-003ac7ba7f24"),
    "Aesthir Glider",
    crate::card::CardArt::new("35a8080f-ca3c-46fe-81cf-003ac7ba7f24", "Ruth Thompson"),
    crate::card::CardSet::Alliances,
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Bird", "Construct"], 2, 1)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::static_ability(
                "This creature can't block.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                },
            ),
        ]),
);

// ALL 116b — Aesthir Glider (alternate printing)

// ALL 117 — Ashnod's Cylix
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ASHNOD_S_CYLIX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d84e6fcf-4745-4dfb-9103-17beec4e45b6"),
    "Ashnod's Cylix",
    crate::card::CardArt::new("d84e6fcf-4745-4dfb-9103-17beec4e45b6", "Nicola Leonard"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 118a — Astrolabe (alternate printing)

// ALL 118b — Astrolabe
// Audit: unsupported — The shared mana planner cannot activate a mana ability whose cost itself requires mana.
pub(in crate::card::sets) static ASTROLABE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8e3a4e30-f919-4c96-89f2-467355135f8f"),
    "Astrolabe",
    crate::card::CardArt::new("8e3a4e30-f919-4c96-89f2-467355135f8f", "Amy Weber"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 119 — Floodwater Dam
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLOODWATER_DAM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d272c3cb-0b68-4693-abef-8a5375b2463e"),
    "Floodwater Dam",
    crate::card::CardArt::new("d272c3cb-0b68-4693-abef-8a5375b2463e", "Randy Gallegos"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 120 — Gustha's Scepter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GUSTHA_S_SCEPTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("797c84fa-3704-4fec-bd72-468d6415ae70"),
    "Gustha's Scepter",
    crate::card::CardArt::new("797c84fa-3704-4fec-bd72-468d6415ae70", "Sandra Everingham"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 121 — Helm of Obedience
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HELM_OF_OBEDIENCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b17e9216-b1ed-4101-a04e-2bb139ccfa55"),
    "Helm of Obedience",
    crate::card::CardArt::new("b17e9216-b1ed-4101-a04e-2bb139ccfa55", "Brian Snõddy"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 122 — Lodestone Bauble
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LODESTONE_BAUBLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("84d88a33-3990-4044-a5fe-4123d5781f18"),
    "Lodestone Bauble",
    crate::card::CardArt::new("84d88a33-3990-4044-a5fe-4123d5781f18", "Douglas Shuler"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 123 — Mishra's Groundbreaker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MISHRA_S_GROUNDBREAKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("74e2dc26-30aa-4e20-84b0-ea4be8894475"),
    "Mishra's Groundbreaker",
    crate::card::CardArt::new("74e2dc26-30aa-4e20-84b0-ea4be8894475", "Randy Gallegos"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 124 — Mystic Compass
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MYSTIC_COMPASS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("de53ba3a-f2f7-4ea6-a2f6-dd5b87029e58"),
    "Mystic Compass",
    crate::card::CardArt::new("de53ba3a-f2f7-4ea6-a2f6-dd5b87029e58", "Amy Weber"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 125 — Phyrexian Devourer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_DEVOURER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("319430fa-11e4-426e-8297-67df8474c3cc"),
    "Phyrexian Devourer",
    crate::card::CardArt::new("319430fa-11e4-426e-8297-67df8474c3cc", "Mark Tedin"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 126 — Phyrexian Portal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_PORTAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("74f77387-1239-4ad2-b59f-d13e317477ba"),
    "Phyrexian Portal",
    crate::card::CardArt::new("74f77387-1239-4ad2-b59f-d13e317477ba", "Pete Venters"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 127a — Phyrexian War Beast
pub(in crate::card::sets) static PHYREXIAN_WAR_BEAST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e7a83384-8762-4028-8cab-b690593790a6"),
    "Phyrexian War Beast",
    crate::card::CardArt::new("e7a83384-8762-4028-8cab-b690593790a6", "Bill Sienkiewicz"),
    crate::card::CardSet::Alliances,
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Phyrexian", "Beast"], 3, 4)
        .with_ability(AbilityDef::triggered(
            "When this creature leaves the battlefield, sacrifice a land and this creature deals 1 damage to you.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                None,
            ),
            EffectDef::Sequence(&[
                EffectDef::SacrificeOfChoice {
                    player: EffectRecipientDef::Controller,
                    object: ObjectPredicateDef::HasType(CardType::Land),
                    count: ValueDef::Constant(1),
                    then: None,
                    amount: SacrificedAmountDef::Power,
                    otherwise: None,
                    optional: false,
                },
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ]),
        )),
);

// ALL 127b — Phyrexian War Beast (alternate printing)

// ALL 128 — Scarab of the Unseen
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCARAB_OF_THE_UNSEEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d5da1c71-6059-4e4e-933d-dbca1cc4bd15"),
    "Scarab of the Unseen",
    crate::card::CardArt::new("d5da1c71-6059-4e4e-933d-dbca1cc4bd15", "Sandra Everingham"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 129 — Shield Sphere
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHIELD_SPHERE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1730d219-a28f-4930-8088-4cfcb627f157"),
    "Shield Sphere",
    crate::card::CardArt::new("1730d219-a28f-4930-8088-4cfcb627f157", "Alan Rabinowitz"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 130 — Sol Grail
pub(in crate::card::sets) static SOL_GRAIL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("62652722-e345-4670-9547-d9579efa227d"),
    "Sol Grail",
    crate::card::CardArt::new("62652722-e345-4670-9547-d9579efa227d", "Christopher Rush"),
    crate::card::CardSet::Alliances,
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::as_enters(
            "As this artifact enters, choose a color.",
            ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(
                BattlefieldEntryScalarChoiceDef::COLOR,
            )),
        ),
        AbilityDef::activated_mana(
            "{T}: Add one mana of the chosen color.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one_of_type(ManaTypeDef::ChosenColor)),
        ),
    ]),
);

// ALL 131 — Soldevi Digger
pub(in crate::card::sets) static SOLDEVI_DIGGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5a3a0ab4-e8ef-45fd-9a73-86d1ee30cb48"),
    "Soldevi Digger",
    crate::card::CardArt::new("5a3a0ab4-e8ef-45fd-9a73-86d1ee30cb48", "Amy Weber"),
    crate::card::CardSet::Alliances,
    CardRules::new_artifact(mana_cost!("{2}")).with_ability(AbilityDef::activated(
        "{2}: Put the top card of your graveyard on the bottom of your library.",
        &[CostDef::Mana(mana_cost!("{2}"))],
        EffectDef::MoveToZone {
            object: EffectRecipientDef::objects(ObjectSetDef::TopOfGraveyardMatching {
                player: PlayerRefDef::EffectController,
                object: ObjectPredicateDef::Any,
            }),
            zone: ZoneKind::Library,
            placement: ZonePlacement::Bottom,
        },
    )),
);

// ALL 132a — Soldevi Sentry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOLDEVI_SENTRY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("85976b5c-4eed-4cf9-b2b0-a8421a97ab2a"),
    "Soldevi Sentry",
    crate::card::CardArt::new("85976b5c-4eed-4cf9-b2b0-a8421a97ab2a", "Alan Rabinowitz"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 132b — Soldevi Sentry (alternate printing)

// ALL 133a — Soldevi Steam Beast (alternate printing)

// ALL 133b — Soldevi Steam Beast
pub(in crate::card::sets) static SOLDEVI_STEAM_BEAST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9de5e730-1d5c-4326-b3fc-2f0f97edc07e"),
    "Soldevi Steam Beast",
    crate::card::CardArt::new("9de5e730-1d5c-4326-b3fc-2f0f97edc07e", "Bill Sienkiewicz"),
    crate::card::CardSet::Alliances,
    CardRules::new_artifact_creature(mana_cost!("{5}"), &["Beast"], 4, 2).with_abilities(&[
        AbilityDef::triggered_with_targets(
            "Whenever this creature becomes tapped, target opponent gains 2 life.",
            TriggerEventDef::tapped(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
            )],
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
        ),
        abilities::regenerate_self(
            "{2}: Regenerate this creature.",
            &[CostDef::Mana(mana_cost!("{2}"))],
        ),
    ]),
);

// ALL 134 — Storm Cauldron
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STORM_CAULDRON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1f68b531-a3f2-4830-b170-fb8a1195c149"),
    "Storm Cauldron",
    crate::card::CardArt::new("1f68b531-a3f2-4830-b170-fb8a1195c149", "Dan Frazier"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 135 — Urza's Engine
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static URZA_S_ENGINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("273b54c3-325b-4f2e-857b-fc1d59b6b3c5"),
    "Urza's Engine",
    crate::card::CardArt::new("273b54c3-325b-4f2e-857b-fc1d59b6b3c5", "Greg Simanson"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 136 — Whirling Catapult
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WHIRLING_CATAPULT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6206d65a-6907-4d11-acb0-8820277f2cf2"),
    "Whirling Catapult",
    crate::card::CardArt::new("6206d65a-6907-4d11-acb0-8820277f2cf2", "Dan Frazier"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 137 — Balduvian Trading Post
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BALDUVIAN_TRADING_POST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a329ff98-36fd-44c3-b037-dcc6e78ee61e"),
    "Balduvian Trading Post",
    crate::card::CardArt::new("a329ff98-36fd-44c3-b037-dcc6e78ee61e", "Tom Wänerstrand"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 138 — Heart of Yavimaya
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEART_OF_YAVIMAYA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("40c59cb9-559b-4716-9bd7-c818b3f46f1d"),
    "Heart of Yavimaya",
    crate::card::CardArt::new("40c59cb9-559b-4716-9bd7-c818b3f46f1d", "Pete Venters"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 139 — Kjeldoran Outpost
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KJELDORAN_OUTPOST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e0769fc7-50b5-4b49-8aff-af04536288fb"),
    "Kjeldoran Outpost",
    crate::card::CardArt::new("e0769fc7-50b5-4b49-8aff-af04536288fb", "Jeff A. Menges"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 140 — Lake of the Dead
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAKE_OF_THE_DEAD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aee806ce-effa-4244-9659-43246e944d80"),
    "Lake of the Dead",
    crate::card::CardArt::new("aee806ce-effa-4244-9659-43246e944d80", "Pete Venters"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 141 — School of the Unseen
// Audit: unsupported — The shared mana planner cannot activate the any-color mana ability because its cost itself requires mana.
pub(in crate::card::sets) static SCHOOL_OF_THE_UNSEEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1438606d-556d-4b96-9662-fcac051af045"),
    "School of the Unseen",
    crate::card::CardArt::new("1438606d-556d-4b96-9662-fcac051af045", "Pat Lewis"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 142 — Sheltered Valley
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHELTERED_VALLEY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("049d7a08-1605-4ce2-b8c5-634ce2a261e0"),
    "Sheltered Valley",
    crate::card::CardArt::new("049d7a08-1605-4ce2-b8c5-634ce2a261e0", "Rob Alexander"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 143 — Soldevi Excavations
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOLDEVI_EXCAVATIONS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8dbda146-ed0a-4bf6-b99d-dc6d59bd9447"),
    "Soldevi Excavations",
    crate::card::CardArt::new("8dbda146-ed0a-4bf6-b99d-dc6d59bd9447", "Liz Danforth"),
    crate::card::CardSet::Alliances,
    crate::card::CardRules::unsupported(),
);

// ALL 144 — Thawing Glaciers
pub(in crate::card::sets) static THAWING_GLACIERS: CardRecord = CardRecord::new_with_legacy_id(
    2057,
    "Thawing Glaciers",
    CardArt::new("6411a8c6-010f-4863-a0fa-bbebe09d5c34", "Jeff A. Menges"),
    CardSet::Alliances,
    // One basic a turn, forever: slow enough that only a deck with nothing
    // better to do at end of turn wants it, which is exactly Landstill.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        AbilityDef::activated(
            "{1}, {T}: Search your library for a basic land card, put that card onto the battlefield tapped, then shuffle. Return this land to its owner's hand at the beginning of the next cleanup step.",
            &[
                CostDef::Mana(mana_cost!("{1}")),
                CostDef::TapSource,
            ],
            EffectDef::Sequence(&const {
                [
                    EffectDef::SearchZone {
                        player: EffectRecipientDef::Controller,
                        source: ZoneKind::Library,
                        object: ObjectPredicateDef::All(&const {
                            [
                                ObjectPredicateDef::HasType(CardType::Land),
                                ObjectPredicateDef::Supertype(CardSupertype::Basic),
                            ]
                        }),
                        minimum: 0,
                        maximum: ValueDef::Constant(1),
                        reveal: false,
                        destination: ZoneKind::Battlefield,
                        placement: ZonePlacement::Top,
                        shuffle: true,
                        enters_tapped: true,
                        attachment: None,
                        binding: None,
                        then: None,
                    },
                    // The land fetches, and then leaves: the return is a delayed trigger so
                    // that the land is available to tap again next turn rather than staying to
                    // be tapped twice in one.
                    EffectDef::InstallTrigger(InstalledTriggerDef::once(&const {
                        AbilityDef::triggered(
                        "At the beginning of the next cleanup step, return this land to its owner's hand.",
                        TriggerEventDef::StepBegins {
                            step: TurnStepDef::Cleanup,
                            player: PlayerRelation::Any,
                        },
                        EffectDef::MoveToZone {
                            object: EffectRecipientDef::Source,
                            zone: ZoneKind::Hand,
                            placement: ZonePlacement::Top,
                        },
                    )
                    })),
                ]
            }),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &CARRIER_PIGEONS,
    &ERRAND_OF_DUTY,
    &EXILE,
    &INHERITANCE,
    &IVORY_GARGOYLE,
    &JUNIPER_ORDER_ADVOCATE,
    &KJELDORAN_ESCORT,
    &KJELDORAN_HOME_GUARD,
    &KJELDORAN_PRIDE,
    &MARTYRDOM,
    &NOBLE_STEEDS,
    &REINFORCEMENTS,
    &REPRISAL,
    &ROYAL_DECREE,
    &ROYAL_HERBALIST,
    &SCARS_OF_THE_VETERAN,
    &SEASONED_TACTICIAN,
    &SUSTAINING_SPIRIT,
    &SWORN_DEFENDER,
    &UNLIKELY_ALLIANCE,
    &WILD_AESTHIR,
    &ARCANE_DENIAL,
    &AWESOME_PRESENCE,
    &BENTHIC_EXPLORERS,
    &BROWSE,
    &DIMINISHING_RETURNS,
    &FALSE_DEMISE,
    &FORCE_OF_WILL,
    &FORESIGHT,
    &LAT_NAM_S_LEGACY,
    &LIBRARY_OF_LAT_NAM,
    &PHANTASMAL_SPHERE,
    &SOLDEVI_HERETIC,
    &SOLDEVI_SAGE,
    &SPINY_STARFISH,
    &STORM_CROW,
    &STORM_ELEMENTAL,
    &SUFFOCATION,
    &THOUGHT_LASH,
    &TIDAL_CONTROL,
    &VISCERID_ARMOR,
    &VISCERID_DRONE,
    &BALDUVIAN_DEAD,
    &CASTING_OF_BONES,
    &CONTAGION,
    &DISEASED_VERMIN,
    &DYSTOPIA,
    &FATAL_LORE,
    &FEAST_OR_FAMINE,
    &FEVERED_STRENGTH,
    &INSIDIOUS_BOOKWORMS,
    &KEEPER_OF_TRESSERHORN,
    &KROVIKAN_HORROR,
    &KROVIKAN_PLAGUE,
    &LIM_DUL_S_HIGH_GUARD,
    &MISINFORMATION,
    &PHANTASMAL_FIEND,
    &PHYREXIAN_BOON,
    &RITUAL_OF_THE_MACHINE,
    &SOLDEVI_ADNATE,
    &STENCH_OF_DECAY,
    &STROMGALD_SPY,
    &SWAMP_MOSQUITO,
    &AGENT_OF_STROMGALD,
    &BALDUVIAN_HORDE,
    &BALDUVIAN_WAR_MAKERS,
    &BESTIAL_FURY,
    &BURNOUT,
    &CHAOS_HARLEQUIN,
    &DEATH_SPARK,
    &ENSLAVED_SCOUT,
    &GORILLA_SHAMAN,
    &GORILLA_WAR_CRY,
    &GUERRILLA_TACTICS,
    &OMEN_OF_FIRE,
    &PILLAGE,
    &PRIMITIVE_JUSTICE,
    &PYROKINESIS,
    &ROGUE_SKYCAPTAIN,
    &SOLDIER_OF_FORTUNE,
    &STORM_SHAMAN,
    &VARCHILD_S_CRUSADER,
    &VARCHILD_S_WAR_RIDERS,
    &VETERAN_S_VOICE,
    &BOUNTY_OF_THE_HUNT,
    &DEADLY_INSECT,
    &ELVISH_BARD,
    &ELVISH_RANGER,
    &ELVISH_SPIRIT_GUIDE,
    &FYNDHORN_DRUID,
    &GARGANTUAN_GORILLA,
    &GIFT_OF_THE_WOODS,
    &GORILLA_BERSERKERS,
    &GORILLA_CHIEFTAIN,
    &HAIL_STORM,
    &KAYSA,
    &NATURE_S_CHOSEN,
    &NATURE_S_WRATH,
    &SPLINTERING_WIND,
    &TASTE_OF_PARADISE,
    &TORNADO,
    &UNDERGROWTH,
    &WHIP_VINE,
    &YAVIMAYA_ANCIENTS,
    &YAVIMAYA_ANTS,
    &ENERGY_ARC,
    &LIM_DUL_S_VAULT,
    &LIM_DUL_S_PALADIN,
    &SURGE_OF_STRENGTH,
    &NATURE_S_BLESSING,
    &WANDERING_MAGE,
    &LORD_OF_TRESSERHORN,
    &MISFORTUNE,
    &WINTER_S_NIGHT,
    &PHELDDAGRIF,
    &AESTHIR_GLIDER,
    &ASHNOD_S_CYLIX,
    &ASTROLABE,
    &FLOODWATER_DAM,
    &GUSTHA_S_SCEPTER,
    &HELM_OF_OBEDIENCE,
    &LODESTONE_BAUBLE,
    &MISHRA_S_GROUNDBREAKER,
    &MYSTIC_COMPASS,
    &PHYREXIAN_DEVOURER,
    &PHYREXIAN_PORTAL,
    &PHYREXIAN_WAR_BEAST,
    &SCARAB_OF_THE_UNSEEN,
    &SHIELD_SPHERE,
    &SOL_GRAIL,
    &SOLDEVI_DIGGER,
    &SOLDEVI_SENTRY,
    &SOLDEVI_STEAM_BEAST,
    &STORM_CAULDRON,
    &URZA_S_ENGINE,
    &WHIRLING_CATAPULT,
    &BALDUVIAN_TRADING_POST,
    &HEART_OF_YAVIMAYA,
    &KJELDORAN_OUTPOST,
    &LAKE_OF_THE_DEAD,
    &SCHOOL_OF_THE_UNSEEN,
    &SHELTERED_VALLEY,
    &SOLDEVI_EXCAVATIONS,
    &THAWING_GLACIERS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::alternate(&CARRIER_PIGEONS, 1), // ALL 1b
    PrintingRecord::alternate(&ERRAND_OF_DUTY, 1),  // ALL 2b
    PrintingRecord::alternate(&KJELDORAN_ESCORT, 1), // ALL 7b
    PrintingRecord::alternate(&KJELDORAN_PRIDE, 1), // ALL 9a
    PrintingRecord::alternate(&MARTYRDOM, 1),       // ALL 10a
    PrintingRecord::alternate(&NOBLE_STEEDS, 1),    // ALL 11b
    PrintingRecord::alternate(&REINFORCEMENTS, 1),  // ALL 12a
    PrintingRecord::alternate(&REPRISAL, 1),        // ALL 13b
    PrintingRecord::alternate(&ROYAL_HERBALIST, 1), // ALL 15b
    PrintingRecord::alternate(&WILD_AESTHIR, 1),    // ALL 21b
    PrintingRecord::alternate(&ARCANE_DENIAL, 1),   // ALL 22b
    PrintingRecord::alternate(&AWESOME_PRESENCE, 1), // ALL 23a
    PrintingRecord::alternate(&BENTHIC_EXPLORERS, 1), // ALL 24b
    PrintingRecord::alternate(&FALSE_DEMISE, 1),    // ALL 27b
    PrintingRecord::alternate(&FORESIGHT, 1),       // ALL 29b
    PrintingRecord::alternate(&LAT_NAM_S_LEGACY, 1), // ALL 30a
    PrintingRecord::alternate(&SOLDEVI_HERETIC, 1), // ALL 33a
    PrintingRecord::alternate(&SOLDEVI_SAGE, 1),    // ALL 34b
    PrintingRecord::alternate(&STORM_CROW, 1),      // ALL 36a
    PrintingRecord::alternate(&VISCERID_ARMOR, 1),  // ALL 41a
    PrintingRecord::alternate(&CASTING_OF_BONES, 1), // ALL 44a
    PrintingRecord::alternate(&FEAST_OR_FAMINE, 1), // ALL 49b
    PrintingRecord::alternate(&FEVERED_STRENGTH, 1), // ALL 50b
    PrintingRecord::alternate(&INSIDIOUS_BOOKWORMS, 1), // ALL 51b
    PrintingRecord::alternate(&LIM_DUL_S_HIGH_GUARD, 1), // ALL 55a
    PrintingRecord::alternate(&PHANTASMAL_FIEND, 1), // ALL 57a
    PrintingRecord::alternate(&PHYREXIAN_BOON, 1),  // ALL 58a
    PrintingRecord::alternate(&SOLDEVI_ADNATE, 1),  // ALL 60a
    PrintingRecord::alternate(&STENCH_OF_DECAY, 1), // ALL 61a
    PrintingRecord::alternate(&SWAMP_MOSQUITO, 1),  // ALL 63b
    PrintingRecord::alternate(&AGENT_OF_STROMGALD, 1), // ALL 64b
    PrintingRecord::alternate(&BALDUVIAN_WAR_MAKERS, 1), // ALL 66b
    PrintingRecord::alternate(&BESTIAL_FURY, 1),    // ALL 67b
    PrintingRecord::alternate(&ENSLAVED_SCOUT, 1),  // ALL 71a
    PrintingRecord::alternate(&GORILLA_SHAMAN, 1),  // ALL 72a
    PrintingRecord::alternate(&GORILLA_WAR_CRY, 1), // ALL 73b
    PrintingRecord::alternate(&GUERRILLA_TACTICS, 1), // ALL 74a
    PrintingRecord::alternate(&STORM_SHAMAN, 1),    // ALL 81a
    PrintingRecord::alternate(&VARCHILD_S_CRUSADER, 1), // ALL 82b
    PrintingRecord::alternate(&VETERAN_S_VOICE, 1), // ALL 84a
    PrintingRecord::alternate(&DEADLY_INSECT, 1),   // ALL 86a
    PrintingRecord::alternate(&ELVISH_RANGER, 1),   // ALL 88a
    PrintingRecord::alternate(&FYNDHORN_DRUID, 1),  // ALL 90b
    PrintingRecord::alternate(&GIFT_OF_THE_WOODS, 1), // ALL 92a
    PrintingRecord::alternate(&GORILLA_BERSERKERS, 1), // ALL 93b
    PrintingRecord::alternate(&GORILLA_CHIEFTAIN, 1), // ALL 94b
    PrintingRecord::alternate(&TASTE_OF_PARADISE, 1), // ALL 100a
    PrintingRecord::alternate(&UNDERGROWTH, 1),     // ALL 102a
    PrintingRecord::alternate(&WHIP_VINE, 1),       // ALL 103a
    PrintingRecord::alternate(&YAVIMAYA_ANCIENTS, 1), // ALL 104a
    PrintingRecord::alternate(&AESTHIR_GLIDER, 1),  // ALL 116b
    PrintingRecord::alternate(&ASTROLABE, 1),       // ALL 118a
    PrintingRecord::alternate(&PHYREXIAN_WAR_BEAST, 1), // ALL 127b
    PrintingRecord::alternate(&SOLDEVI_SENTRY, 1),  // ALL 132b
    PrintingRecord::alternate(&SOLDEVI_STEAM_BEAST, 1), // ALL 133a
];
