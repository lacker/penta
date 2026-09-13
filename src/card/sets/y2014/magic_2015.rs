//! Magic 2015 cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::ComparisonDef;
use crate::card::CostDef;
use crate::card::CreateTokenDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ManaRestrictionDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SubtypeDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "M15",
    slug: "magic-2015",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// M15 14 — Heliod's Pilgrim
pub(in crate::card::sets) static HELIOD_S_PILGRIM: CardRecord = CardRecord::new(
    "Heliod's Pilgrim",
    "7ea54b97-9182-4d46-9d70-3cc7f9b18ada",
    "Izzy",
    // The body is beside the point: this is a three-mana tutor that an Aura
    // deck plays for whichever Aura the board asks for.
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Cleric"], 1, 2).with_ability(
        abilities::enters_trigger(
            "When this creature enters, you may search your library for an Aura card, reveal it, \
             put it into your hand, then shuffle.",
            // Two ways to decline: the outer may, and a minimum of zero for a
            // search that finds nothing worth taking.
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Aura")),
                    minimum: 0,
                    maximum: ValueDef::Constant(1),
                    reveal: true,
                    destination: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: false,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            },
        ),
    ),
);

// M15 15 — Hushwing Gryff
pub(in crate::card::sets) static HUSHWING_GRYFF_15: CardRecord = CardRecord::new(
    "Hushwing Gryff",
    "7b44eb0d-5a3a-4624-aee4-11d6978fb4b0",
    "John Severin Brassell",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Hippogriff"], 2, 1).with_abilities(&[
        abilities::flash(),
        abilities::flying(),
        AbilityDef::static_ability(
            "Creatures entering don't cause abilities to trigger.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::players(PlayerSetDef::All),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::ModifyTriggers(
                    &crate::card::TriggerModificationDef {
                        cause: TriggerEventDef::zone_changed(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            None,
                            Some(ZoneKind::Battlefield),
                        ),
                        permanent: None,
                        kind: crate::card::TriggerModificationKindDef::Suppress,
                    },
                )),
            },
        ),
    ]),
);

// M15 40 — Triplicate Spirits
pub(in crate::card::sets) static TRIPLICATE_SPIRITS: CardRecord = CardRecord::new(
    "Triplicate Spirits",
    "3d6498d3-bf1f-4bf1-a602-7c21fb44c106",
    "Izzy",
    // Six mana printed, but the tokens it already made are what pay for the
    // next copy, so the real cost falls every time a token deck casts it.
    CardRules::new_sorcery(mana_cost!("{4}{W}{W}")).with_abilities(&[
        abilities::convoke(),
        AbilityDef::spell(
            "Create three 1/1 white Spirit creature tokens with flying.",
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(
                    TokenCharacteristics::creature(&["Spirit"], &[ManaColor::White], 1, 1)
                        .with_abilities(&[abilities::flying()]),
                ))
                .with_amount(3),
            ),
        ),
    ]),
);

// M15 119 — Ulcerate
pub(in crate::card::sets) static ULCERATE_119: CardRecord = CardRecord::new(
    "Ulcerate",
    "2e06e6c8-05c0-4d87-9961-605b888bc794",
    "Johann Bodin",
    CardRules::new_instant(mana_cost!("{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets -3/-3 until end of turn. You lose 3 life.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-3),
                    ValueDef::Constant(-3),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ]),
    )]),
);

// M15 122 — Waste Not
// Audit: unsupported — Discarded has no card predicate. Object-set conditions inspect a live zone object rather than the discarded card snapshot, so they cannot classify discard-cost events whose hand object was replaced or already moved.
pub(in crate::card::sets) static WASTE_NOT_122: CardRecord = CardRecord::new(
    "Waste Not",
    "241d8f7d-3981-47c1-b7b8-748277fa452f",
    "Matt Stewart",
    crate::card::CardRules::unsupported(),
);

// M15 138 — Crowd's Favor
pub(in crate::card::sets) static CROWD_S_FAVOR_138: CardRecord = CardRecord::new(
    "Crowd's Favor",
    "536b8104-9d8d-444b-8535-62bcbe279de2",
    "Slawomir Maniak",
    CardRules::new_instant(mana_cost!("{R}")).with_abilities(&[
abilities::convoke(),
AbilityDef::spell_with_targets("Target creature gets +1/+0 and gains first strike until end of turn. (It deals combat damage before creatures without first strike.)", &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Creature))], EffectDef::Apply { recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY), effect: AppliedEffectDef::Composite(&[AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(0)), AppliedEffectDef::add_ability(&abilities::first_strike())]), duration: ResolvedEffectDurationDef::UntilEndOfTurn })
]),
);

// M15 142 — Frenzied Goblin (reprint)
const FRENZIED_GOBLIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2005::ravnica_city_of_guilds::FRENZIED_GOBLIN,
    "7ddfe382-3a80-45f3-a022-54739c4b69a6",
    "Carl Critchlow",
);

// M15 143 — Generator Servant
// Audit: unsupported — Mana-spend grants have no duration field; they can grant haste to a paid creature spell, but cannot expire that grant at end of turn.
pub(in crate::card::sets) static GENERATOR_SERVANT_143: CardRecord = CardRecord::new(
    "Generator Servant",
    "74d0c422-4201-4d6f-9df7-659e8b78b541",
    "Mathias Kollros",
    crate::card::CardRules::unsupported(),
);

// M15 145 — Goblin Rabblemaster
pub(in crate::card::sets) static GOBLIN_RABBLEMASTER: CardRecord = CardRecord::new(
    "Goblin Rabblemaster",
    "ee9c697e-d2c0-413b-9142-ecf5d7cf5322",
    "Svetlin Velinov",
// Three mana that makes a Goblin every turn and then sends the whole
    // pile in whether or not that was the plan.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin", "Warrior"], 2, 2)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Other Goblin creatures you control attack each combat if able.",
                EffectDef::StaticApply {
                    // "Other Goblin creatures you control": the Rabblemaster is a Goblin too
                    // and is not made to attack by its own clause.
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Goblin")),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ))),
                    effect: AppliedEffectDef::add_ability(&abilities::attacks_each_combat_if_able()),
                },
            ),
            AbilityDef::triggered(
                "At the beginning of combat on your turn, create a 1/1 red Goblin creature token with \
                 haste.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::BeginningOfCombat,
                    player: PlayerRelation::You,
                },
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                    TokenCharacteristics::creature(&["Goblin"], &[ManaColor::Red], 1, 1)
                        .with_abilities(&[abilities::haste()])
                        .with_art(CardArt::new(
                            "98993a45-4aff-4f9b-a030-7d72fbb4ec6c",
                            "Karl Kopinski",
                        )),
                ))),
            ),
            AbilityDef::triggered(
                "Whenever this creature attacks, it gets +1/+0 until end of turn for each other attacking \
                 Goblin.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        // Every other Goblin in the attack, whoever controls it. The count is read
                        // as the trigger resolves, so a Goblin that was removed in response is not
                        // among them.
                        ValueDef::CountMatchingObjects(&ObjectQueryDef::new(
                            ObjectPredicateDef::All(&[
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Goblin")),
                                ObjectPredicateDef::Attacking,
                                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ]),
                            &[ZoneKind::Battlefield],
                        )),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// M15 164 — Stoke the Flames
pub(in crate::card::sets) static STOKE_THE_FLAMES_164: CardRecord = CardRecord::new(
    "Stoke the Flames",
    "1d94c000-52e0-4215-83af-6351dc43e636",
    "Ryan Barger",
    CardRules::new_instant(mana_cost!("{2}{R}{R}")).with_abilities(&[
        abilities::convoke(),
        AbilityDef::spell_with_targets(
            "Stoke the Flames deals 4 damage to any target.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(4),
            ),
        ),
    ]),
);

// M15 194 — Reclamation Sage
pub(in crate::card::sets) static RECLAMATION_SAGE: CardRecord = CardRecord::new(
    "Reclamation Sage",
    "47227cfa-4cef-4874-b331-d2f628f29dae",
    "Christopher Moeller",
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Elf", "Shaman"], 2, 1).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this creature enters, you may destroy target artifact or \
             enchantment.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                ]),
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            },
        ),
    ]),
);

// M15 209 — Yisan, the Wanderer Bard
// Audit: unsupported — PutCountersOnSource is supported in resolving payments but not activation-cost enumeration or payment; the counter must be paid before this ability goes on the stack.
pub(in crate::card::sets) static YISAN_THE_WANDERER_BARD_209: CardRecord = CardRecord::new(
    "Yisan, the Wanderer Bard",
    "65cd97cd-6d6e-4512-a050-6851b7527567",
    "Chase Stone",
    crate::card::CardRules::unsupported(),
);

// M15 215 — The Chain Veil
// Audit: unsupported — MayActivateLoyaltyAnyTime changes timing only; the engine has no additional loyalty activation allowance that composes with prior activations and repeated resolutions.
pub(in crate::card::sets) static THE_CHAIN_VEIL_215: CardRecord = CardRecord::new(
    "The Chain Veil",
    "0415cc0e-979e-42cc-a56d-88d13153a7de",
    "Volkan Baǵa",
    crate::card::CardRules::unsupported(),
);

// M15 247 — Sliver Hive
pub(in crate::card::sets) static SLIVER_HIVE: CardRecord = CardRecord::new(
    "Sliver Hive",
    "91cef7ce-aa9f-4659-ac24-394c5ab9f77c",
    "Igor Kieryluk",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_mana(
            "{T}: Add one mana of any color. Spend this mana only to cast a Sliver spell.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color().with_restrictions(&[
                ManaRestrictionDef::CastSpell(ObjectPredicateDef::Subtype(SubtypeDef::Literal(
                    "Sliver",
                ))),
            ])),
        ),
        AbilityDef::activated(
            "{5}, {T}: Create a 1/1 colorless Sliver creature token. Activate only if you \
             control a Sliver.",
            &[CostDef::Mana(mana_cost!("{5}")), CostDef::TapSource],
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::creature(&["Sliver"], &[], 1, 1).with_art(CardArt::new(
                    "dec96e95-5580-4110-86ec-561007ab0f1e",
                    "Igor Kieryluk",
                )),
            ))),
        )
        .with_activation_condition(&TriggerConditionDef::ObjectCount {
            query: ObjectQueryDef::matching(
                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Sliver")),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            comparison: ComparisonDef::GreaterOrEqual,
            amount: 1,
        }),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &HELIOD_S_PILGRIM,
    &HUSHWING_GRYFF_15,
    &TRIPLICATE_SPIRITS,
    &ULCERATE_119,
    &WASTE_NOT_122,
    &CROWD_S_FAVOR_138,
    &GENERATOR_SERVANT_143,
    &GOBLIN_RABBLEMASTER,
    &STOKE_THE_FLAMES_164,
    &RECLAMATION_SAGE,
    &YISAN_THE_WANDERER_BARD_209,
    &THE_CHAIN_VEIL_215,
    &SLIVER_HIVE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] =
    &[FRENZIED_GOBLIN_REPRINT];
