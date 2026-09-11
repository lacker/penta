//! Edge of Eternities card inventory.

use super::CardRecord;
use super::PrintingRecord;
use crate::ParentBinding;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AlternativeCastKindDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CardTypeSet;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseDef;
use crate::card::ComparisonDef;
use crate::card::CostDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::CreatureTypeSetDef;
use crate::card::DamageEventMatcherDef;
use crate::card::DamageRecipientMatcherDef;
use crate::card::DeclarativeAbilityDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::EmblemCharacteristics;
use crate::card::GraveyardPlayPermissionDef;
use crate::card::HalvedValueDef;
use crate::card::ManaColor;
use crate::card::ModalSpellDef;
use crate::card::MoveObjectsDef;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectSetDef;
use crate::card::PlayActionMatcherDef;
use crate::card::PlayRestrictionDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::QuantifierDef;
use crate::card::RandomizeObjectOrderDef;
use crate::card::ReplacementAbilityDef;
use crate::card::ReplacementConditionDef;
use crate::card::ReplacementEffectDef;
use crate::card::ReplacementEventDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::RoundingDef;
use crate::card::SubtypeDef;
use crate::card::SumValueDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TriggeredAbilityDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

use crate::card::sets::y1993::alpha as catalog_lea;
use crate::card::sets::y1998::urzas_saga as catalog_usg;
use crate::card::sets::y2005::ravnica_city_of_guilds as catalog_rav;
use crate::card::sets::y2006::dissension as catalog_dis;
use crate::card::sets::y2006::guildpact as catalog_gpt;
use crate::card::sets::y2014::journey_into_nyx as catalog_jou;
use crate::card::sets::y2018::rivals_of_ixalan as catalog_rix;
use crate::card::sets::y2022::kamigawa_neon_dynasty as catalog_neo;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "EOE",
    slug: "edge-of-eternities",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// EOE 1 — Anticausal Vestige
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANTICAUSAL_VESTIGE: CardRecord = CardRecord::new(
    "Anticausal Vestige",
    "35372b69-6086-44e0-9f7c-681e362e5142",
    "Chase Stone",
    crate::card::CardRules::unsupported(),
);

// EOE 2 — Tezzeret, Cruel Captain
static AN_ARTIFACT_YOU_CONTROL: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Artifact),
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
]);

pub(in crate::card::sets) static TEZZERET_CRUEL_CAPTAIN: CardRecord = CardRecord::new(
    "Tezzeret, Cruel Captain",
    "02e8e540-8aa3-4e6a-9a11-c3949cab5f0f",
    "Chris Rahn",
// Three colourless for a planeswalker that an artifact deck keeps
    // topping up, and whose zero is free every turn.
    CardRules::new_planeswalker(mana_cost!("{3}"), &["Tezzeret"], 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever an artifact you control enters, put a loyalty counter on Tezzeret.",
                TriggerEventDef::zone_changed(AN_ARTIFACT_YOU_CONTROL, None, Some(ZoneKind::Battlefield)),
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::Loyalty,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::activated_with_targets(
                "0: Untap target artifact or creature. If it\'s an artifact creature, put a +1/+1 counter \
                 on it.",
                &[CostDef::Loyalty(0)],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                )],
                EffectDef::Sequence(&[
                    EffectDef::Untap {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                    EffectDef::IfCondition {
                        // The rider is asked of the target as the ability resolves, so an artifact
                        // animated in response is a legal thing to grow.
                        condition: &TriggerConditionDef::TargetMatches {
                                slot: TargetIndex::PRIMARY,
                                object: ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::HasType(CardType::Artifact),
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                ]),
                            },
                        then: &EffectDef::AddCounters {
                            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::Constant(1),
                        },
                    },
                ]),
            ),
            AbilityDef::activated(
                "−3: Search your library for an artifact card with mana value 1 or less, reveal it, put \
                 it into your hand, then shuffle.",
                &[CostDef::Loyalty(-3)],
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    // A one-mana artifact, which is what the deck this is in is made of.
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::ManaValueAtMost(1),
                    ]),
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
            ),
            AbilityDef::activated(
                "−7: You get an emblem with \"At the beginning of combat on your turn, put three +1/+1 \
                 counters on target artifact you control. If it\'s not a creature, it becomes a 0/0 Robot \
                 artifact creature.\"",
                &[CostDef::Loyalty(-7)],
                EffectDef::CreateEmblem {
                    emblem: EmblemCharacteristics::new("Tezzeret, Cruel Captain emblem", &[AbilityDef::triggered_with_targets(
                            "At the beginning of combat on your turn, put three +1/+1 counters on target artifact you \
                             control. If it's not a creature, it becomes a 0/0 Robot artifact creature.",
                            TriggerEventDef::StepBegins {
                                step: TurnStepDef::BeginningOfCombat,
                                player: PlayerRelation::You,
                            },
                            &[AbilityTargetDef::exactly_one_permanent(
                                    AN_ARTIFACT_YOU_CONTROL,
                                )],
                            EffectDef::Sequence(&[
                                EffectDef::AddCounters {
                                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                    kind: CounterKind::PlusOnePlusOne,
                                    amount: ValueDef::Constant(3),
                                },
                                EffectDef::IfCondition {
                                    condition: &TriggerConditionDef::TargetMatches {
                                            slot: TargetIndex::PRIMARY,
                                            object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                                        },
                                    then: &EffectDef::Apply {
                                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                        // "If it's not a creature, it becomes a 0/0 Robot artifact creature." The
                                        // counters go on first, so an artifact that was not a creature ends up a
                                        // 3/3: the base is what changes, and the counters sit on top of it.
                                        effect: AppliedEffectDef::Composite(&[
                                            AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                                            AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&["Robot"])),
                                            AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(0), ValueDef::Constant(0)),
                                        ]),
                                        duration: ResolvedEffectDurationDef::Permanent,
                                    },
                                },
                            ]),
                        )]),
                },
            ),
        ]),
);

// EOE 3 — All-Fates Stalker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALL_FATES_STALKER: CardRecord = CardRecord::new(
    "All-Fates Stalker",
    "82ae4f7b-8122-4af6-8079-888eabf1a11e",
    "Lius Lasahido",
    crate::card::CardRules::unsupported(),
);

// EOE 4 — Astelli Reclaimer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ASTELLI_RECLAIMER: CardRecord = CardRecord::new(
    "Astelli Reclaimer",
    "4fb36405-cd28-432f-b0a4-e74ff8be928d",
    "Carly Milligan",
    crate::card::CardRules::unsupported(),
);

// EOE 5 — Auxiliary Boosters
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AUXILIARY_BOOSTERS: CardRecord = CardRecord::new(
    "Auxiliary Boosters",
    "43706295-afd6-442c-8828-8cf978152701",
    "Dmitry Burmak",
    crate::card::CardRules::unsupported(),
);

// EOE 6 — Banishing Light (reprint)
const BANISHING_LIGHT_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_jou::BANISHING_LIGHT,
    "c45f11cd-a0aa-4d14-aa21-57f0969f3e2b",
    "Rovina Cai",
);

// EOE 7 — Beyond the Quiet
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BEYOND_THE_QUIET: CardRecord = CardRecord::new(
    "Beyond the Quiet",
    "ce503869-8130-4afe-9691-4e90376b4bc4",
    "Yohann Schepacz",
    crate::card::CardRules::unsupported(),
);

// EOE 8 — Brightspear Zealot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRIGHTSPEAR_ZEALOT: CardRecord = CardRecord::new(
    "Brightspear Zealot",
    "e7f7541a-5910-4f33-8c1d-3507ce3a426e",
    "Bryan Sola",
    crate::card::CardRules::unsupported(),
);

// EOE 9 — Cosmogrand Zenith
pub(in crate::card::sets) static COSMOGRAND_ZENITH: CardRecord = CardRecord::new(
    "Cosmogrand Zenith",
    "b3c1e5e3-4e6b-456a-958c-7a75c38f8183",
    "Anna Steinbauer",
// Three mana for a 2/4 that pays a second time every turn the hand has
    // two spells in it, and the choice is between going wider and going
    // taller.
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Soldier"], 2, 4)
        .with_abilities(&[AbilityDef::defined(
            "Whenever you cast your second spell each turn, choose one —\n• Create two 1/1 white Human \
             Soldier creature tokens.\n• Put a +1/+1 counter on each creature you control.",
            DeclarativeAbilityDef::Triggered(
                TriggeredAbilityDef::new(TriggerEventDef::spell_cast(
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ))
                // Exactly the second, not the second or later: the spell that caused the
                // trigger has already been counted by the time this is read.
                .with_condition(&TriggerConditionDef::SpellsCastThisTurn {
                    quantifier: QuantifierDef::Any,
                    player: PlayerRelation::You,
                    comparison: ComparisonDef::Equal,
                    amount: 2,
                })
                .with_modes(ModalSpellDef::choose_one(&[
                    AbilityDef::spell(
                        "Create two 1/1 white Human Soldier creature tokens.",
                        EffectDef::CreateToken(
                            CreateTokenDef::new(TokenDef::Literal(TokenCharacteristics::creature(
                                &["Human", "Soldier"],
                                &[ManaColor::White],
                                1,
                                1,
                            )))
                            .with_count(ValueDef::Constant(2)),
                        ),
                    ),
                    // Each creature you control as the trigger resolves, which includes the
                    // tokens the other mode would have made and the Zenith itself.
                    AbilityDef::spell(
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
                    ),
                ])),
            ),
            EffectDef::None,
        )]),
);

// EOE 10 — Dawnstrike Vanguard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DAWNSTRIKE_VANGUARD: CardRecord = CardRecord::new(
    "Dawnstrike Vanguard",
    "5a041722-9483-469f-9c17-7f0253b0db50",
    "Arif Wijaya",
    crate::card::CardRules::unsupported(),
);

// EOE 11 — Dockworker Drone
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DOCKWORKER_DRONE: CardRecord = CardRecord::new(
    "Dockworker Drone",
    "eeff069f-427b-42ad-afb1-36f0e547fb74",
    "Marco Gorlei",
    crate::card::CardRules::unsupported(),
);

// EOE 12 — Dual-Sun Adepts
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DUAL_SUN_ADEPTS: CardRecord = CardRecord::new(
    "Dual-Sun Adepts",
    "c7e8c830-77ae-437f-8e28-ce61c5fde6b6",
    "Ioannis Fiore",
    crate::card::CardRules::unsupported(),
);

// EOE 13 — Dual-Sun Technique
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DUAL_SUN_TECHNIQUE: CardRecord = CardRecord::new(
    "Dual-Sun Technique",
    "f8931132-391f-4f16-b480-0a245ab2ec21",
    "Ioannis Fiore",
    crate::card::CardRules::unsupported(),
);

// EOE 14 — Emergency Eject
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EMERGENCY_EJECT: CardRecord = CardRecord::new(
    "Emergency Eject",
    "fc98b2d4-86fc-4c47-b2a7-1f3c89463607",
    "Leon Tukker",
    crate::card::CardRules::unsupported(),
);

// EOE 15 — Exalted Sunborn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXALTED_SUNBORN: CardRecord = CardRecord::new(
    "Exalted Sunborn",
    "7e1fe101-f634-41e5-9aa4-e8d7474535dc",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// EOE 16 — Exosuit Savior
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXOSUIT_SAVIOR: CardRecord = CardRecord::new(
    "Exosuit Savior",
    "826c0455-a6ce-43ad-bd5c-0a5df169da90",
    "Benjamin Ee",
    crate::card::CardRules::unsupported(),
);

// EOE 17 — Flight-Deck Coordinator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLIGHT_DECK_COORDINATOR: CardRecord = CardRecord::new(
    "Flight-Deck Coordinator",
    "88cc6328-a035-4f74-b786-390e5c7c324c",
    "Diego Gisbert",
    crate::card::CardRules::unsupported(),
);

// EOE 18 — Focus Fire
pub(in crate::card::sets) static FOCUS_FIRE: CardRecord = CardRecord::new(
    "Focus Fire",
    "a9ddfcbc-0f84-4315-aaa3-ca54ff64d7de",
    "Borja Pindado",
    // The floor is already two damage in combat, and a board counts twice:
    // each body both survives the trade and raises what this kills.
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell_with_targets(
        "Focus Fire deals X damage to target attacking or blocking creature, where X is 2 plus \
         the number of creatures and/or Spacecraft you control.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::AttackingOrBlocking,
            ]),
        )],
        EffectDef::damage(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ValueDef::Sum(&SumValueDef::new(
                ValueDef::Constant(2),
                ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    // A Spacecraft that has stationed up is already a
                    // creature, so the two halves overlap and the query has
                    // to match each permanent once rather than twice.
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Spacecraft")),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
            )),
        ),
    )),
);

// EOE 19 — Haliya, Guided by Light
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HALIYA_GUIDED_BY_LIGHT: CardRecord = CardRecord::new(
    "Haliya, Guided by Light",
    "6f7c63ae-5df3-410f-8643-b8c69133ca9d",
    "Kieran Yanner",
    crate::card::CardRules::unsupported(),
);

// EOE 20 — Hardlight Containment
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HARDLIGHT_CONTAINMENT: CardRecord = CardRecord::new(
    "Hardlight Containment",
    "0b934241-4d6b-4b9c-99f1-c49cb387cf56",
    "Dominik Mayer",
    crate::card::CardRules::unsupported(),
);

// EOE 21 — Honor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HONOR: CardRecord = CardRecord::new(
    "Honor",
    "d0b4e925-1d59-41a1-bc06-9982695d778f",
    "Eli Minaya",
    crate::card::CardRules::unsupported(),
);

// EOE 22 — Honored Knight-Captain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HONORED_KNIGHT_CAPTAIN: CardRecord = CardRecord::new(
    "Honored Knight-Captain",
    "05a6ab03-f0b9-4738-a5f9-5d95bb22de75",
    "Forrest Imel",
    crate::card::CardRules::unsupported(),
);

// EOE 23 — Knight Luminary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KNIGHT_LUMINARY: CardRecord = CardRecord::new(
    "Knight Luminary",
    "34334971-c1b7-4506-a6dd-77f66b3ae4e7",
    "Aaron Miller",
    crate::card::CardRules::unsupported(),
);

// EOE 24 — Lightstall Inquisitor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIGHTSTALL_INQUISITOR: CardRecord = CardRecord::new(
    "Lightstall Inquisitor",
    "635245e9-c27f-4a51-a6f1-bae62e696542",
    "Arif Wijaya",
    crate::card::CardRules::unsupported(),
);

// EOE 25 — Lumen-Class Frigate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LUMEN_CLASS_FRIGATE: CardRecord = CardRecord::new(
    "Lumen-Class Frigate",
    "0cc59b5a-65fa-47cc-8ac7-b7c3f533a782",
    "Zezhou Chen",
    crate::card::CardRules::unsupported(),
);

// EOE 26 — Luxknight Breacher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LUXKNIGHT_BREACHER: CardRecord = CardRecord::new(
    "Luxknight Breacher",
    "c1236731-0d33-4705-8077-3cf58acf9a39",
    "Cristi Balanescu",
    crate::card::CardRules::unsupported(),
);

// EOE 27 — Pinnacle Starcage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PINNACLE_STARCAGE: CardRecord = CardRecord::new(
    "Pinnacle Starcage",
    "b1f40c4c-a955-4d9c-8225-251fa4159124",
    "Leon Tukker",
    crate::card::CardRules::unsupported(),
);

// EOE 28 — Pulsar Squadron Ace
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PULSAR_SQUADRON_ACE: CardRecord = CardRecord::new(
    "Pulsar Squadron Ace",
    "8d989cdc-cbd7-4b71-9589-59618597ac8a",
    "Javier Charro",
    crate::card::CardRules::unsupported(),
);

// EOE 29 — Radiant Strike
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RADIANT_STRIKE: CardRecord = CardRecord::new(
    "Radiant Strike",
    "8c38e4cb-918b-4493-872b-66c90dcfd339",
    "Aleksi Briclot",
    crate::card::CardRules::unsupported(),
);

// EOE 30 — Rayblade Trooper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAYBLADE_TROOPER: CardRecord = CardRecord::new(
    "Rayblade Trooper",
    "c08c7bf9-a2ed-45c6-8b48-15122d9d9e37",
    "Cristi Balanescu",
    crate::card::CardRules::unsupported(),
);

// EOE 31 — Reroute Systems
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REROUTE_SYSTEMS: CardRecord = CardRecord::new(
    "Reroute Systems",
    "3bbdba38-2b99-4226-98e3-6d2580345d6d",
    "Sergey Glushakov",
    crate::card::CardRules::unsupported(),
);

// EOE 32 — Rescue Skiff
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RESCUE_SKIFF: CardRecord = CardRecord::new(
    "Rescue Skiff",
    "6fe86bfb-c67d-4df9-88c9-f083091f4cda",
    "Viko Menezes",
    crate::card::CardRules::unsupported(),
);

// EOE 33 — Scout for Survivors
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCOUT_FOR_SURVIVORS: CardRecord = CardRecord::new(
    "Scout for Survivors",
    "ebf3a6dd-a447-46f9-8b10-091ac8cbaa18",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// EOE 34 — Seam Rip
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEAM_RIP: CardRecord = CardRecord::new(
    "Seam Rip",
    "9d298847-2d02-4593-b4d3-c5b722edac1e",
    "Sam Guay",
    crate::card::CardRules::unsupported(),
);

// EOE 35 — The Seriema
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_SERIEMA: CardRecord = CardRecord::new(
    "The Seriema",
    "dec91ec3-42d3-4922-96f0-dbb50a576084",
    "Sergey Glushakov",
    crate::card::CardRules::unsupported(),
);

// EOE 36 — Squire's Lightblade
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SQUIRE_S_LIGHTBLADE: CardRecord = CardRecord::new(
    "Squire's Lightblade",
    "2a0accba-85d4-4aa4-a70c-80fcce48c261",
    "Edgar Sánchez Hidalgo",
    crate::card::CardRules::unsupported(),
);

// EOE 37 — Starfield Shepherd
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STARFIELD_SHEPHERD: CardRecord = CardRecord::new(
    "Starfield Shepherd",
    "1226e575-aa78-4c68-be1d-6e5c2dc6315b",
    "Marta Nael",
    crate::card::CardRules::unsupported(),
);

// EOE 38 — Starfighter Pilot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STARFIGHTER_PILOT: CardRecord = CardRecord::new(
    "Starfighter Pilot",
    "1e571246-aadc-4d1f-a284-9a529e150fe0",
    "Nathaniel Himawan",
    crate::card::CardRules::unsupported(),
);

// EOE 39 — Starport Security
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STARPORT_SECURITY: CardRecord = CardRecord::new(
    "Starport Security",
    "238cb1db-6c41-4fe1-bc34-340048dfde18",
    "Lie Setiawan",
    crate::card::CardRules::unsupported(),
);

// EOE 40 — Sunstar Chaplain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUNSTAR_CHAPLAIN: CardRecord = CardRecord::new(
    "Sunstar Chaplain",
    "83719626-3ff8-4566-9911-88212e753c69",
    "Valera Lutfullina",
    crate::card::CardRules::unsupported(),
);

// EOE 41 — Sunstar Expansionist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUNSTAR_EXPANSIONIST: CardRecord = CardRecord::new(
    "Sunstar Expansionist",
    "93b0e6e5-9fb6-4342-9322-1a4cc09a7a76",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// EOE 42 — Sunstar Lightsmith
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUNSTAR_LIGHTSMITH: CardRecord = CardRecord::new(
    "Sunstar Lightsmith",
    "5f60b09d-9814-4a36-a57d-59b0e04c1c2f",
    "Jarel Threat",
    crate::card::CardRules::unsupported(),
);

// EOE 43 — Wedgelight Rammer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WEDGELIGHT_RAMMER: CardRecord = CardRecord::new(
    "Wedgelight Rammer",
    "2cb0984f-dc8b-4bb3-a4fd-8d6d4ae20198",
    "Nadia Hurianova",
    crate::card::CardRules::unsupported(),
);

// EOE 44 — Weftblade Enhancer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WEFTBLADE_ENHANCER: CardRecord = CardRecord::new(
    "Weftblade Enhancer",
    "8d72b00c-5043-4630-949a-fc17eeb962bc",
    "Nathaniel Himawan",
    crate::card::CardRules::unsupported(),
);

// EOE 45 — Zealous Display
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ZEALOUS_DISPLAY: CardRecord = CardRecord::new(
    "Zealous Display",
    "f5973fd3-d6bc-48f1-8a44-57d2a6dda228",
    "Chris Rallis",
    crate::card::CardRules::unsupported(),
);

// EOE 46 — Annul (reprint)
const ANNUL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_usg::ANNUL,
    "4feeebea-aa55-4599-ab5a-4e41a54d0dfd",
    "Carlos Palma Cruchaga",
);

// EOE 47 — Atomic Microsizer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ATOMIC_MICROSIZER: CardRecord = CardRecord::new(
    "Atomic Microsizer",
    "3554f0c7-ea73-43e9-a061-bbb8ef6abce1",
    "Gabor Szikszai",
    crate::card::CardRules::unsupported(),
);

// EOE 48 — Cerebral Download
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CEREBRAL_DOWNLOAD: CardRecord = CardRecord::new(
    "Cerebral Download",
    "4d3b5d73-694c-4f9a-8b4f-d8d8c58c8d65",
    "Antonio José Manzanedo",
    crate::card::CardRules::unsupported(),
);

// EOE 49 — Cloudsculpt Technician
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLOUDSCULPT_TECHNICIAN: CardRecord = CardRecord::new(
    "Cloudsculpt Technician",
    "51077a54-15cf-4088-8e84-088d72e8e861",
    "Elizabeth Peiró",
    crate::card::CardRules::unsupported(),
);

// EOE 50 — Codecracker Hound
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CODECRACKER_HOUND: CardRecord = CardRecord::new(
    "Codecracker Hound",
    "6723b891-6013-4ec6-b439-2233d270dc48",
    "Julia Metzger",
    crate::card::CardRules::unsupported(),
);

// EOE 51 — Consult the Star Charts
/// "Where X is the number of lands you control", which is the whole reason
/// the card is playable: it looks at more the longer the game goes.
static LANDS_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Land),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

/// One selection differs from the other only in how many it keeps, so the
/// two are the same workflow twice rather than a count the spell could carry.
macro_rules! consult_choice {
    ($cards:expr, $chosen:expr, $rest:expr) => {
        EffectDef::Choose(ChooseDef {
            binding: ObjectChoiceBindingDef::Objects($chosen),
            unchosen: Some($rest),
            chooser: PlayerRefDef::EffectController,
            candidates: ObjectSetDef::Binding(ParentBinding),
            exclude: None,
            minimum: $cards,
            maximum: $cards,
            visibility: ChoiceVisibilityDef::Private,
            then: &EffectDef::Sequence(&[
                EffectDef::MoveObjects(MoveObjectsDef {
                    input: ObjectSetDef::Binding($chosen),
                    from: Some(ZoneKind::Library),
                    zone: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                    moved: None,
                    then: &EffectDef::None,
                }),
                EffectDef::RandomizeObjectOrder(RandomizeObjectOrderDef {
                    input: ObjectSetDef::Binding($rest),
                    randomized: ParentBinding,
                    then: &EffectDef::MoveObjects(MoveObjectsDef {
                        input: ObjectSetDef::Binding(ParentBinding),
                        from: Some(ZoneKind::Library),
                        zone: ZoneKind::Library,
                        placement: ZonePlacement::Bottom,
                        moved: None,
                        then: &EffectDef::None,
                    }),
                }),
            ]),
        })
    };
}

pub(in crate::card::sets) static CONSULT_THE_STAR_CHARTS: CardRecord = CardRecord::new(
    "Consult the Star Charts",
    "a16a6555-2e3a-4587-aacd-0307d696b26c",
    "Antonio José Manzanedo",
    // Two mana to dig as deep as your mana base, and four to keep twice as
    // much of what it finds.
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{2}{U}{U}"))],
            AlternativeCastKindDef::Kicked,
            Some("Kicker {1}{U} (You may pay an additional {1}{U} as you cast this spell.)"),
            EffectDef::None,
        ),
        AbilityDef::spell(
            "Look at the top X cards of your library, where X is the number of lands you \
             control. Put one of those cards into your hand. If this spell was kicked, put two \
             of those cards into your hand instead. Put the rest on the bottom of your library \
             in a random order.",
            EffectDef::IfElseCondition {
                condition: &TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked),
                then: &abilities::bind_top_cards_then(
                    PlayerRefDef::EffectController,
                    ValueDef::CountMatchingObjects(&LANDS_YOU_CONTROL),
                    &consult_choice!(
                        2,
                        Binding!("consult_kicked_chosen"),
                        Binding!("consult_kicked_rest")
                    ),
                ),
                otherwise: &abilities::bind_top_cards_then(
                    PlayerRefDef::EffectController,
                    ValueDef::CountMatchingObjects(&LANDS_YOU_CONTROL),
                    &consult_choice!(
                        1,
                        Binding!("consult_normal_chosen"),
                        Binding!("consult_normal_rest")
                    ),
                ),
            },
        ),
    ]),
);

// EOE 52 — Cryogen Relic
pub(in crate::card::sets) static CRYOGEN_RELIC: CardRecord = CardRecord::new(
    "Cryogen Relic",
    "7bfb33b6-e2bf-498f-8c58-ae21a840cf75",
    "Eelis Kyttanen",
// Sacrificing it draws the second card, so the tap-down costs nothing
    // in cards -- only the two mana and the artifact itself.
    CardRules::new_artifact(mana_cost!("{1}{U}")).with_abilities(&[
        AbilityDef::triggered(
            "When this artifact enters or leaves the battlefield, draw a card.",
            // One printed sentence with two ways in, so it is one ability
            // watching both zone changes.
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    None,
                ),
            ]),
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_with_targets(
            "{1}{U}, Sacrifice this artifact: Put a stun counter on up to one target tapped creature.",
            &[
                CostDef::Mana(mana_cost!("{1}{U}")),
                CostDef::SacrificeSource,
            ],
            // "Up to one", so it can be sacrificed purely for the leave
            // trigger's card when nothing is tapped.
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Tapped,
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                1,
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::Stun,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// EOE 53 — Cryoshatter
pub(in crate::card::sets) static CRYOSHATTER: CardRecord = CardRecord::new(
    "Cryoshatter",
    "7b62b1e2-9e43-4a66-a647-7e5de2871f2a",
    "Jeremy Wilson",
    // One mana blanks the creature immediately and kills it the moment it
    // is used for anything, which is what makes the -5/-0 half enough.
    CardRules::new_enchantment(mana_cost!("{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets -5/-0.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-5),
                        ValueDef::Constant(0),
                    ),
                },
            ),
            AbilityDef::triggered(
                "When enchanted creature becomes tapped or is dealt damage, destroy it.",
                // Two ways into one printed sentence, both read against the
                // creature this Aura is on rather than the Aura itself.
                TriggerEventDef::AnyOf(&[
                    TriggerEventDef::tapped(ObjectPredicateDef::AttachedToSource),
                    TriggerEventDef::DamageDealt(DamageEventMatcherDef {
                        recipient: DamageRecipientMatcherDef::Recipients(
                            EffectRecipientDef::AttachedPermanent,
                        ),
                        ..DamageEventMatcherDef::ANY
                    }),
                ]),
                EffectDef::Destroy {
                    object: EffectRecipientDef::AttachedPermanent,
                    then: None,
                },
            ),
        ]),
);

// EOE 54 — Desculpting Blast
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DESCULPTING_BLAST: CardRecord = CardRecord::new(
    "Desculpting Blast",
    "77fdbf32-b5f4-4346-846d-d8e0e53e6e53",
    "Jeremy Wilson",
    crate::card::CardRules::unsupported(),
);

// EOE 55 — Divert Disaster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIVERT_DISASTER: CardRecord = CardRecord::new(
    "Divert Disaster",
    "c5f7d2fe-628b-4493-8281-0e5f91ce5d61",
    "David Álvarez",
    crate::card::CardRules::unsupported(),
);

// EOE 56 — Emissary Escort
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EMISSARY_ESCORT: CardRecord = CardRecord::new(
    "Emissary Escort",
    "b52ba87f-3ac7-4f32-901c-d089df979f94",
    "Igor Grechanyi",
    crate::card::CardRules::unsupported(),
);

// EOE 57 — Gigastorm Titan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GIGASTORM_TITAN: CardRecord = CardRecord::new(
    "Gigastorm Titan",
    "abc83e0a-0ae5-4087-a751-058a1ba6a920",
    "Bryan Sola",
    crate::card::CardRules::unsupported(),
);

// EOE 58 — Illvoi Galeblade
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ILLVOI_GALEBLADE: CardRecord = CardRecord::new(
    "Illvoi Galeblade",
    "769f7d13-a312-4d79-8639-6ae248452448",
    "Nathaniel Himawan",
    crate::card::CardRules::unsupported(),
);

// EOE 59 — Illvoi Infiltrator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ILLVOI_INFILTRATOR: CardRecord = CardRecord::new(
    "Illvoi Infiltrator",
    "2db4ed41-0426-4d7f-bb43-e43392bed83b",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// EOE 60 — Illvoi Light Jammer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ILLVOI_LIGHT_JAMMER: CardRecord = CardRecord::new(
    "Illvoi Light Jammer",
    "efb3d961-543c-4404-b4ed-1cb28ee411b3",
    "David Álvarez",
    crate::card::CardRules::unsupported(),
);

// EOE 61 — Illvoi Operative
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ILLVOI_OPERATIVE: CardRecord = CardRecord::new(
    "Illvoi Operative",
    "d0ae9fc7-1802-4806-9996-1f1f458ff6a7",
    "Quintin Gleim",
    crate::card::CardRules::unsupported(),
);

// EOE 62 — Lost in Space
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LOST_IN_SPACE: CardRecord = CardRecord::new(
    "Lost in Space",
    "6d9d7979-97af-4c85-86f5-1b3704f74e8b",
    "Allen Panakal",
    crate::card::CardRules::unsupported(),
);

// EOE 63 — Mechan Assembler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MECHAN_ASSEMBLER: CardRecord = CardRecord::new(
    "Mechan Assembler",
    "3fd46726-095e-4eb2-a804-eeeb988eee1d",
    "Mirko Failoni",
    crate::card::CardRules::unsupported(),
);

// EOE 64 — Mechan Navigator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MECHAN_NAVIGATOR: CardRecord = CardRecord::new(
    "Mechan Navigator",
    "a1fe1d39-42c8-41d0-8bf0-46973e4b07d4",
    "Konstantin Porubov",
    crate::card::CardRules::unsupported(),
);

// EOE 65 — Mechan Shieldmate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MECHAN_SHIELDMATE: CardRecord = CardRecord::new(
    "Mechan Shieldmate",
    "745b2119-4d9f-431f-89b9-10ad48b6dc47",
    "Daniel Ljunggren",
    crate::card::CardRules::unsupported(),
);

// EOE 66 — Mechanozoa
pub(in crate::card::sets) static MECHANOZOA: CardRecord = CardRecord::new(
    "Mechanozoa",
    "0cb8d8ce-329a-4a97-b3d8-796703ebcb37",
    "Daarken",
    // Warped on three it is a Frost Lynx that comes back; cast on six it is
    // a 5/7 that does the same thing again. Either way the stun counter is
    // what buys the tempo.
    CardRules::new_artifact_creature(mana_cost!("{4}{U}{U}"), &["Robot", "Jellyfish"], 5, 7)
        .with_abilities(&[
            abilities::enters_trigger_with_targets(
                "When this creature enters, tap target artifact or creature an opponent controls \
                 and put a stun counter on it. (If a permanent with a stun counter would become \
                 untapped, remove one from it instead.)",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Creature),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::Opponent),
                        owner: None,
                    },
                )],
                // Tapping first and stunning second is the printed order, and
                // it matters: a permanent that was already tapped still takes
                // the counter, so the untap it misses is the next one.
                EffectDef::Sequence(&[
                    EffectDef::Tap {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        kind: CounterKind::Stun,
                        amount: ValueDef::Constant(1),
                    },
                ]),
            ),
            abilities::warp(
                &[CostDef::Mana(mana_cost!("{2}{U}"))],
                "Warp {2}{U} (You may cast this card from your hand for its warp cost. Exile it \
                at the beginning of the next end step, then you may cast it from exile on a \
                later turn.)",
            ),
            abilities::warped_exile(),
        ]),
);

// EOE 67 — Mental Modulation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MENTAL_MODULATION: CardRecord = CardRecord::new(
    "Mental Modulation",
    "0f2d12fc-38a0-42e6-9caa-7c18bfcf0011",
    "Andreia Ugrai",
    crate::card::CardRules::unsupported(),
);

// EOE 68 — Mm'menon, the Right Hand
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MM_MENON_THE_RIGHT_HAND: CardRecord = CardRecord::new(
    "Mm'menon, the Right Hand",
    "82add0a0-e402-4b31-b101-81c0bf332015",
    "Joshua Raphael",
    crate::card::CardRules::unsupported(),
);

// EOE 69 — Moonlit Meditation
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOONLIT_MEDITATION: CardRecord = CardRecord::new(
    "Moonlit Meditation",
    "f2a56007-5bca-4edf-9cc4-5f77a273636c",
    "Liiga Smilshkalne",
    crate::card::CardRules::unsupported(),
);

// EOE 70 — Mouth of the Storm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOUTH_OF_THE_STORM: CardRecord = CardRecord::new(
    "Mouth of the Storm",
    "380f16d6-ad43-4e0d-9645-6abde6248182",
    "Domenico Cava",
    crate::card::CardRules::unsupported(),
);

// EOE 71 — Nanoform Sentinel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NANOFORM_SENTINEL: CardRecord = CardRecord::new(
    "Nanoform Sentinel",
    "3eeae8c3-7939-4c79-92f0-fbdb9c1b71d3",
    "Tianxing Xu",
    crate::card::CardRules::unsupported(),
);

// EOE 72 — Quantum Riddler
pub(in crate::card::sets) static QUANTUM_RIDDLER: CardRecord = CardRecord::new(
    "Quantum Riddler",
    "120be808-ff3b-4fca-96a1-4db6b9825856",
    "Izzy",
    // Five mana for a 4/6 flier that draws a card, or two mana for the same
    // body until the end of turn and the card it comes back with later.
    CardRules::new_creature(mana_cost!("{3}{U}{U}"), &["Sphinx"], 4, 6).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, draw a card.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::defined_replacement(
            "As long as you have one or fewer cards in hand, if you would draw one or more \
             cards, you draw that many cards plus one instead.",
            // "As long as you have one or fewer cards in hand, if you would draw one
            // or more cards, you draw that many cards plus one instead." One
            // replacement of the whole instruction: a draw of three becomes a draw of
            // four rather than a draw of six.
            ReplacementAbilityDef::new()
                .with_event(ReplacementEventDef::WouldDraw {
                    player: PlayerRelation::You,
                    during_own_draw_step: false,
                    except_first_in_draw_step: false,
                })
                .with_condition(ReplacementConditionDef::ControllerHandAtMost(1)),
            ReplacementEffectDef::AddToEventAmount(1),
        ),
        abilities::warp(
            &[CostDef::Mana(mana_cost!("{1}{U}"))],
            "Warp {1}{U} (You may cast this card from your hand for its warp cost. Exile it at \
            the beginning of the next end step, then you may cast it from exile on a later \
            turn.)",
        ),
        abilities::warped_exile(),
    ]),
);

// EOE 73 — Scour for Scrap
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCOUR_FOR_SCRAP: CardRecord = CardRecord::new(
    "Scour for Scrap",
    "517d1b00-7ec4-489a-ac52-657da24a6379",
    "Filip Burburan",
    crate::card::CardRules::unsupported(),
);

// EOE 74 — Selfcraft Mechan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SELFCRAFT_MECHAN: CardRecord = CardRecord::new(
    "Selfcraft Mechan",
    "5b2de056-5c27-44d6-871d-909411bd52dd",
    "Milivoj Ćeran",
    crate::card::CardRules::unsupported(),
);

// EOE 75 — Sinister Cryologist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SINISTER_CRYOLOGIST: CardRecord = CardRecord::new(
    "Sinister Cryologist",
    "e8fbe740-05ec-4ced-bb9d-3084c8c2b631",
    "Domenico Cava",
    crate::card::CardRules::unsupported(),
);

// EOE 76 — Specimen Freighter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPECIMEN_FREIGHTER: CardRecord = CardRecord::new(
    "Specimen Freighter",
    "b862a9f8-2361-4220-99bd-ae2530905195",
    "Sergey Glushakov",
    crate::card::CardRules::unsupported(),
);

// EOE 77 — Starbreach Whale
pub(in crate::card::sets) static STARBREACH_WHALE: CardRecord = CardRecord::new(
    "Starbreach Whale",
    "8a1a0476-7145-4493-97e5-4fc05c85e476",
    "Sam Burley",
    // Warp buys the surveil on turn two and the 3/5 flier later, off one
    // card. The entry trigger is what makes the early half worth a turn:
    // it fires both times.
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Whale"], 3, 5).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, surveil 2. (Look at the top two cards of your library, \
             then put any number of them into your graveyard and the rest on top of your library \
             in any order.)",
            abilities::surveil(ValueDef::Constant(2)),
        ),
        abilities::warp(
            &[CostDef::Mana(mana_cost!("{1}{U}"))],
            "Warp {1}{U} (You may cast this card from your hand for its warp cost. Exile it at \
            the beginning of the next end step, then you may cast it from exile on a later \
            turn.)",
        ),
        abilities::warped_exile(),
    ]),
);

// EOE 78 — Starfield Vocalist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STARFIELD_VOCALIST: CardRecord = CardRecord::new(
    "Starfield Vocalist",
    "deca0b2a-e7f3-444a-883d-7c41dd62c9cc",
    "Nathaniel Himawan",
    crate::card::CardRules::unsupported(),
);

// EOE 79 — Starwinder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STARWINDER: CardRecord = CardRecord::new(
    "Starwinder",
    "27d1a010-5790-4b35-9fdc-0e366eed021d",
    "Devin Elle Kurtz",
    crate::card::CardRules::unsupported(),
);

// EOE 80 — Steelswarm Operator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STEELSWARM_OPERATOR: CardRecord = CardRecord::new(
    "Steelswarm Operator",
    "ca468b86-f31a-4cd7-a574-eb984bc4bc3e",
    "Cristi Balanescu",
    crate::card::CardRules::unsupported(),
);

// EOE 81 — Synthesizer Labship
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SYNTHESIZER_LABSHIP: CardRecord = CardRecord::new(
    "Synthesizer Labship",
    "fba6332c-acba-43f9-877c-ca6c5328aae9",
    "Adrián Rodríguez Pérez",
    crate::card::CardRules::unsupported(),
);

// EOE 82 — Tractor Beam
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRACTOR_BEAM: CardRecord = CardRecord::new(
    "Tractor Beam",
    "efc96d54-d6e1-49b6-b4c2-70b997776548",
    "Sergey Glushakov",
    crate::card::CardRules::unsupported(),
);

// EOE 83 — Unravel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNRAVEL: CardRecord = CardRecord::new(
    "Unravel",
    "e8978214-c853-453d-872d-af56bdaaa3d7",
    "Josh Hass",
    crate::card::CardRules::unsupported(),
);

// EOE 84 — Uthros Psionicist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UTHROS_PSIONICIST: CardRecord = CardRecord::new(
    "Uthros Psionicist",
    "e23cc5fd-afe4-480c-8858-ed80a082584e",
    "Inkognit",
    crate::card::CardRules::unsupported(),
);

// EOE 85 — Uthros Scanship
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UTHROS_SCANSHIP: CardRecord = CardRecord::new(
    "Uthros Scanship",
    "1f93887f-35c5-472f-83d0-54227b3bd1d2",
    "Sergey Glushakov",
    crate::card::CardRules::unsupported(),
);

// EOE 86 — Weftwalking
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WEFTWALKING: CardRecord = CardRecord::new(
    "Weftwalking",
    "39d48ddd-4529-4284-9da3-5272ad362b9b",
    "Rovina Cai",
    crate::card::CardRules::unsupported(),
);

// EOE 87 — Alpharael, Stonechosen
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALPHARAEL_STONECHOSEN: CardRecord = CardRecord::new(
    "Alpharael, Stonechosen",
    "33063d26-37f7-4e35-8da2-5770dfabdc41",
    "Kieran Yanner",
    crate::card::CardRules::unsupported(),
);

// EOE 88 — Archenemy's Charm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARCHENEMY_S_CHARM: CardRecord = CardRecord::new(
    "Archenemy's Charm",
    "dcde5f27-e2f0-4d2a-afa3-f300896ec4b1",
    "Brigitte Roka & Clifton Stommel",
    crate::card::CardRules::unsupported(),
);

// EOE 89 — Beamsaw Prospector
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BEAMSAW_PROSPECTOR: CardRecord = CardRecord::new(
    "Beamsaw Prospector",
    "6f717a3f-c6db-4e8d-8b62-6361ab33d000",
    "Aurore Folny",
    crate::card::CardRules::unsupported(),
);

// EOE 90 — Blade of the Swarm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLADE_OF_THE_SWARM: CardRecord = CardRecord::new(
    "Blade of the Swarm",
    "b157330a-2652-4ed9-b8fa-8e72b4eda15c",
    "Nino Is",
    crate::card::CardRules::unsupported(),
);

// EOE 91 — Chorale of the Void
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHORALE_OF_THE_VOID: CardRecord = CardRecord::new(
    "Chorale of the Void",
    "7389fe88-f6ff-4497-a037-9ca283fb89e3",
    "Alix Branwyn",
    crate::card::CardRules::unsupported(),
);

// EOE 92 — Comet Crawler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COMET_CRAWLER: CardRecord = CardRecord::new(
    "Comet Crawler",
    "becca990-ab2e-4aa4-be7d-293ec727cb08",
    "Cristi Balanescu",
    crate::card::CardRules::unsupported(),
);

// EOE 93 — Dark Endurance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARK_ENDURANCE: CardRecord = CardRecord::new(
    "Dark Endurance",
    "fac87b49-a0cd-42d5-b30a-efc6d5526fc3",
    "Leon Tukker",
    crate::card::CardRules::unsupported(),
);

// EOE 94 — Decode Transmissions
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DECODE_TRANSMISSIONS: CardRecord = CardRecord::new(
    "Decode Transmissions",
    "cecb4936-14ca-49f9-b209-6519cab54b30",
    "Josh Hass",
    crate::card::CardRules::unsupported(),
);

// EOE 95 — Depressurize
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEPRESSURIZE: CardRecord = CardRecord::new(
    "Depressurize",
    "25520d5a-1a83-42cc-8ace-8b1156019d64",
    "Danny Schwartz",
    crate::card::CardRules::unsupported(),
);

// EOE 96 — Dubious Delicacy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DUBIOUS_DELICACY: CardRecord = CardRecord::new(
    "Dubious Delicacy",
    "153265b4-0ca4-4245-9226-dd1a083ec91c",
    "Tianxing Xu",
    crate::card::CardRules::unsupported(),
);

// EOE 97 — Elegy Acolyte
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ELEGY_ACOLYTE: CardRecord = CardRecord::new(
    "Elegy Acolyte",
    "c69ed7c7-1f49-4299-b3e6-75150258ac59",
    "Diana Franco",
    crate::card::CardRules::unsupported(),
);

// EOE 98 — Embrace Oblivion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EMBRACE_OBLIVION: CardRecord = CardRecord::new(
    "Embrace Oblivion",
    "3754fc20-7aaa-437d-97df-d6cf1c29586c",
    "Andreas Zafiratos",
    crate::card::CardRules::unsupported(),
);

// EOE 99 — Entropic Battlecruiser
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENTROPIC_BATTLECRUISER: CardRecord = CardRecord::new(
    "Entropic Battlecruiser",
    "cc59796b-9025-44b6-a188-cf6684ebffb9",
    "Josiah \"Jo\" Cameron",
    crate::card::CardRules::unsupported(),
);

// EOE 100 — Faller's Faithful
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FALLER_S_FAITHFUL: CardRecord = CardRecord::new(
    "Faller's Faithful",
    "cbb1b46f-72e0-4c2f-8012-74529bd29a0d",
    "Lie Setiawan",
    crate::card::CardRules::unsupported(),
);

// EOE 101 — Fell Gravship
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FELL_GRAVSHIP: CardRecord = CardRecord::new(
    "Fell Gravship",
    "e94b130d-3547-43c5-a319-5ebc571c2e2d",
    "David Álvarez",
    crate::card::CardRules::unsupported(),
);

// EOE 102 — Gravblade Heavy
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRAVBLADE_HEAVY: CardRecord = CardRecord::new(
    "Gravblade Heavy",
    "b3872341-d711-407b-85e4-46ccb99988e1",
    "Andrew Mar",
    crate::card::CardRules::unsupported(),
);

// EOE 103 — Gravkill
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRAVKILL: CardRecord = CardRecord::new(
    "Gravkill",
    "cfa6c57f-a193-48cc-9764-d8348548a111",
    "Dominik Mayer",
    crate::card::CardRules::unsupported(),
);

// EOE 104 — Gravpack Monoist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRAVPACK_MONOIST: CardRecord = CardRecord::new(
    "Gravpack Monoist",
    "2a968f01-36ef-4cf6-b1db-630c9cde2064",
    "Edgar Sánchez Hidalgo",
    crate::card::CardRules::unsupported(),
);

// EOE 105 — Hullcarver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HULLCARVER: CardRecord = CardRecord::new(
    "Hullcarver",
    "817b5b18-beb5-48c8-aa45-0515ff9ca5da",
    "Michal Ivan",
    crate::card::CardRules::unsupported(),
);

// EOE 106 — Hylderblade
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HYLDERBLADE: CardRecord = CardRecord::new(
    "Hylderblade",
    "ac80cf18-0707-4358-bdd4-0c2b90d0a1d9",
    "Viko Menezes",
    crate::card::CardRules::unsupported(),
);

// EOE 107 — Hymn of the Faller
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HYMN_OF_THE_FALLER: CardRecord = CardRecord::new(
    "Hymn of the Faller",
    "e468d528-6cf0-4563-9da2-e388ba56cb9d",
    "Danny Schwartz",
    crate::card::CardRules::unsupported(),
);

// EOE 108 — Insatiable Skittermaw
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INSATIABLE_SKITTERMAW: CardRecord = CardRecord::new(
    "Insatiable Skittermaw",
    "e9d30cca-ea33-418f-bba3-5103f1dbd751",
    "Diego Gisbert",
    crate::card::CardRules::unsupported(),
);

// EOE 109 — Lightless Evangel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIGHTLESS_EVANGEL: CardRecord = CardRecord::new(
    "Lightless Evangel",
    "6860556a-6c34-4c41-89ea-f0bc495a159c",
    "Viko Menezes",
    crate::card::CardRules::unsupported(),
);

// EOE 110 — Monoist Circuit-Feeder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MONOIST_CIRCUIT_FEEDER: CardRecord = CardRecord::new(
    "Monoist Circuit-Feeder",
    "957ae7aa-98d6-402a-9b20-e3e5b7e8dfe3",
    "Quintin Gleim",
    crate::card::CardRules::unsupported(),
);

// EOE 111 — Monoist Sentry
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MONOIST_SENTRY: CardRecord = CardRecord::new(
    "Monoist Sentry",
    "acc503e2-5c3a-4200-beb0-7d193d6c869e",
    "Nino Is",
    crate::card::CardRules::unsupported(),
);

// EOE 112 — Perigee Beckoner
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PERIGEE_BECKONER: CardRecord = CardRecord::new(
    "Perigee Beckoner",
    "f3666a08-d449-496f-969a-bf21d4afbd77",
    "Dmitry Burmak",
    crate::card::CardRules::unsupported(),
);

// EOE 113 — Requiem Monolith
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REQUIEM_MONOLITH: CardRecord = CardRecord::new(
    "Requiem Monolith",
    "837d710a-652f-4c60-a52d-d786231160a4",
    "Warren Mahy",
    crate::card::CardRules::unsupported(),
);

// EOE 114 — Scrounge for Eternity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SCROUNGE_FOR_ETERNITY: CardRecord = CardRecord::new(
    "Scrounge for Eternity",
    "baeff907-017e-4dee-aae1-19dfaab309de",
    "Konstantin Porubov",
    crate::card::CardRules::unsupported(),
);

// EOE 115 — Sothera, the Supervoid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SOTHERA_THE_SUPERVOID: CardRecord = CardRecord::new(
    "Sothera, the Supervoid",
    "e99d6fc0-dcf2-4b25-81c2-02c230a36246",
    "Dominik Mayer",
    crate::card::CardRules::unsupported(),
);

// EOE 116 — Sunset Saboteur
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUNSET_SABOTEUR: CardRecord = CardRecord::new(
    "Sunset Saboteur",
    "396bca07-82ba-49b7-b79e-7784b3a06d48",
    "Mirko Failoni",
    crate::card::CardRules::unsupported(),
);

// EOE 117 — Susurian Dirgecraft
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUSURIAN_DIRGECRAFT: CardRecord = CardRecord::new(
    "Susurian Dirgecraft",
    "b67cdb6e-9a3b-4887-924d-318faa3c443d",
    "Mark Poole",
    crate::card::CardRules::unsupported(),
);

// EOE 118 — Susurian Voidborn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUSURIAN_VOIDBORN: CardRecord = CardRecord::new(
    "Susurian Voidborn",
    "beb97e7b-0ae7-4b08-9ceb-6a7f825bcd49",
    "Jehan Choo",
    crate::card::CardRules::unsupported(),
);

// EOE 119 — Swarm Culler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SWARM_CULLER: CardRecord = CardRecord::new(
    "Swarm Culler",
    "2a8f583c-88b6-4797-b93e-3086845fc326",
    "April Prime",
    crate::card::CardRules::unsupported(),
);

// EOE 120 — Temporal Intervention
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TEMPORAL_INTERVENTION: CardRecord = CardRecord::new(
    "Temporal Intervention",
    "79f9525a-4cb7-411e-b7b5-2113e93bcbc3",
    "Chris Rallis",
    crate::card::CardRules::unsupported(),
);

// EOE 121 — Timeline Culler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TIMELINE_CULLER: CardRecord = CardRecord::new(
    "Timeline Culler",
    "33410410-72f2-49c4-9e63-a72202cd075a",
    "Alfonso Santano",
    crate::card::CardRules::unsupported(),
);

// EOE 122 — Tragic Trajectory
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRAGIC_TRAJECTORY: CardRecord = CardRecord::new(
    "Tragic Trajectory",
    "78c8bc60-1378-4028-8ab1-3286e459bffb",
    "Ovidio Cartagena",
    crate::card::CardRules::unsupported(),
);

// EOE 123 — Umbral Collar Zealot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UMBRAL_COLLAR_ZEALOT: CardRecord = CardRecord::new(
    "Umbral Collar Zealot",
    "bbcc1d84-9772-475d-924a-75bb54c9bc20",
    "Dmitry Burmak",
    crate::card::CardRules::unsupported(),
);

// EOE 124 — Virus Beetle (reprint)
const VIRUS_BEETLE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_neo::VIRUS_BEETLE,
    "e96c986c-684c-4546-a9c9-b6b903bda101",
    "Leesha Hannigan",
);

// EOE 125 — Voidforged Titan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VOIDFORGED_TITAN: CardRecord = CardRecord::new(
    "Voidforged Titan",
    "6119c016-01b2-44f3-9550-6988324c1d1f",
    "Diego Gisbert",
    crate::card::CardRules::unsupported(),
);

// EOE 126 — Vote Out
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VOTE_OUT: CardRecord = CardRecord::new(
    "Vote Out",
    "4f50bb47-cc4d-4b81-b5f1-817ca8744d12",
    "David Álvarez",
    crate::card::CardRules::unsupported(),
);

// EOE 127 — Xu-Ifit, Osteoharmonist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static XU_IFIT_OSTEOHARMONIST: CardRecord = CardRecord::new(
    "Xu-Ifit, Osteoharmonist",
    "c0838f25-2193-4305-b73a-bf0c0bb4981a",
    "Michal Ivan",
    crate::card::CardRules::unsupported(),
);

// EOE 128 — Zero Point Ballad
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ZERO_POINT_BALLAD: CardRecord = CardRecord::new(
    "Zero Point Ballad",
    "59cf9f4d-54cd-4cda-9726-65e16100ab46",
    "David Astruga",
    crate::card::CardRules::unsupported(),
);

// EOE 129 — Bombard (reprint)
const BOMBARD_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rix::BOMBARD,
    "29492df8-3077-4ded-a6e2-51d3bd4669b4",
    "Diego Gisbert",
);

// EOE 130 — Cut Propulsion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CUT_PROPULSION: CardRecord = CardRecord::new(
    "Cut Propulsion",
    "d96f1c41-3d11-48a8-b962-db46a2d054de",
    "Andrea Piparo",
    crate::card::CardRules::unsupported(),
);

// EOE 131 — Debris Field Crusher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEBRIS_FIELD_CRUSHER: CardRecord = CardRecord::new(
    "Debris Field Crusher",
    "ea713f35-6442-4388-8839-2714374fb4b6",
    "David Álvarez",
    crate::card::CardRules::unsupported(),
);

// EOE 132 — Devastating Onslaught
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEVASTATING_ONSLAUGHT: CardRecord = CardRecord::new(
    "Devastating Onslaught",
    "fc779971-24e2-46a8-86be-16f0b244a3d2",
    "Chris Seaman",
    crate::card::CardRules::unsupported(),
);

// EOE 133 — Drill Too Deep
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRILL_TOO_DEEP: CardRecord = CardRecord::new(
    "Drill Too Deep",
    "b9d3b6e8-47c8-49e3-b204-8b659e127bde",
    "Bartek Fedyczak",
    crate::card::CardRules::unsupported(),
);

// EOE 134 — Frontline War-Rager
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FRONTLINE_WAR_RAGER: CardRecord = CardRecord::new(
    "Frontline War-Rager",
    "fa232943-818b-4944-be60-2d80c806bf62",
    "Jason Rainville",
    crate::card::CardRules::unsupported(),
);

// EOE 135 — Full Bore
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FULL_BORE: CardRecord = CardRecord::new(
    "Full Bore",
    "cfee64ef-d22d-4024-bc65-59cbd1731d1c",
    "Olivier Bernard",
    crate::card::CardRules::unsupported(),
);

// EOE 136 — Galvanizing Sawship
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GALVANIZING_SAWSHIP: CardRecord = CardRecord::new(
    "Galvanizing Sawship",
    "5bbce9fb-401f-4e78-acd5-9d3b506687fd",
    "Constantin Marin",
    crate::card::CardRules::unsupported(),
);

// EOE 137 — Invasive Maneuvers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INVASIVE_MANEUVERS: CardRecord = CardRecord::new(
    "Invasive Maneuvers",
    "b68010e2-7810-43cb-a52e-e73b1834e54e",
    "Leon Tukker",
    crate::card::CardRules::unsupported(),
);

// EOE 138 — Kav Landseeker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KAV_LANDSEEKER: CardRecord = CardRecord::new(
    "Kav Landseeker",
    "7a5a7e89-50e3-43cb-af93-d7d80a630c11",
    "Karl Kopinski",
    crate::card::CardRules::unsupported(),
);

// EOE 139 — Kavaron Harrier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KAVARON_HARRIER: CardRecord = CardRecord::new(
    "Kavaron Harrier",
    "1da2a740-e4ff-4661-ba57-39b15c58e26e",
    "Hardy Fowler",
    crate::card::CardRules::unsupported(),
);

// EOE 140 — Kavaron Skywarden
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KAVARON_SKYWARDEN: CardRecord = CardRecord::new(
    "Kavaron Skywarden",
    "617038a8-0544-4d0e-8ff1-c786e60ecd59",
    "Diana Franco",
    crate::card::CardRules::unsupported(),
);

// EOE 141 — Kavaron Turbodrone
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KAVARON_TURBODRONE: CardRecord = CardRecord::new(
    "Kavaron Turbodrone",
    "f5e92cdd-75df-499e-94f7-22287f1000b3",
    "Leesha Hannigan",
    crate::card::CardRules::unsupported(),
);

// EOE 142 — Lithobraking
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LITHOBRAKING: CardRecord = CardRecord::new(
    "Lithobraking",
    "5a22024c-2c0f-4487-98d1-ee89cf3dba89",
    "Andrew Mar",
    crate::card::CardRules::unsupported(),
);

// EOE 143 — Melded Moxite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MELDED_MOXITE: CardRecord = CardRecord::new(
    "Melded Moxite",
    "474c067f-eb24-4ae8-b4c0-f6f8e24cdb2a",
    "Alexandr Leskinen",
    crate::card::CardRules::unsupported(),
);

// EOE 144 — Memorial Team Leader
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MEMORIAL_TEAM_LEADER: CardRecord = CardRecord::new(
    "Memorial Team Leader",
    "3ddc240a-62df-4773-98d7-48a9adaf1846",
    "Andrew Mar",
    crate::card::CardRules::unsupported(),
);

// EOE 145 — Memorial Vault
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MEMORIAL_VAULT: CardRecord = CardRecord::new(
    "Memorial Vault",
    "a14ecd13-325a-4555-b1dd-d0d0d0826031",
    "Javier Charro",
    crate::card::CardRules::unsupported(),
);

// EOE 146 — Molecular Modifier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOLECULAR_MODIFIER: CardRecord = CardRecord::new(
    "Molecular Modifier",
    "7b80b9c9-a871-4c04-b8be-feb81a900591",
    "Konstantin Porubov",
    crate::card::CardRules::unsupported(),
);

// EOE 147 — Nebula Dragon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NEBULA_DRAGON: CardRecord = CardRecord::new(
    "Nebula Dragon",
    "0ee509dd-9fba-4b6d-a9d4-cc8bf5822ddd",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// EOE 148 — Nova Hellkite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NOVA_HELLKITE: CardRecord = CardRecord::new(
    "Nova Hellkite",
    "424af0d0-398c-4d78-9ad5-2171bf1bcbd1",
    "Raymond Swanland",
    crate::card::CardRules::unsupported(),
);

// EOE 149 — Orbital Plunge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORBITAL_PLUNGE: CardRecord = CardRecord::new(
    "Orbital Plunge",
    "2dc7cc17-5319-4694-99c6-8c56a0b40a44",
    "Inkognit",
    crate::card::CardRules::unsupported(),
);

// EOE 150 — Oreplate Pangolin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OREPLATE_PANGOLIN: CardRecord = CardRecord::new(
    "Oreplate Pangolin",
    "90209957-95db-4b59-979a-316d14ef876c",
    "Dmitry Burmak",
    crate::card::CardRules::unsupported(),
);

// EOE 151 — Pain for All
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PAIN_FOR_ALL: CardRecord = CardRecord::new(
    "Pain for All",
    "d2948913-817b-4715-92d5-ed3cde347be7",
    "Dmitry Burmak",
    crate::card::CardRules::unsupported(),
);

// EOE 152 — Plasma Bolt
// Audit: unsupported — Needs void. Neither half of its condition is available: ControllerHadPermanentLeaveThisTurn is per-player and does not exclude lands, and nothing records that a spell was warped this turn.
pub(in crate::card::sets) static PLASMA_BOLT: CardRecord = CardRecord::new(
    "Plasma Bolt",
    "a1a1834b-76c2-4496-b8c5-18b69ab34c4c",
    "Viko Menezes",
    crate::card::CardRules::unsupported(),
);

// EOE 153 — Possibility Technician
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static POSSIBILITY_TECHNICIAN: CardRecord = CardRecord::new(
    "Possibility Technician",
    "4b146c78-403f-48c8-941d-41114498bb89",
    "Antonio José Manzanedo",
    crate::card::CardRules::unsupported(),
);

// EOE 154 — Red Tiger Mechan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RED_TIGER_MECHAN: CardRecord = CardRecord::new(
    "Red Tiger Mechan",
    "b7b2fa48-cd2d-42ea-afd8-8cbd7a1bcdab",
    "Simon Dominic",
    crate::card::CardRules::unsupported(),
);

// EOE 155 — Remnant Elemental
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REMNANT_ELEMENTAL: CardRecord = CardRecord::new(
    "Remnant Elemental",
    "830d5532-3b24-470e-912f-f0f5df1cb530",
    "Nereida",
    crate::card::CardRules::unsupported(),
);

// EOE 156 — Rig for War
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RIG_FOR_WAR: CardRecord = CardRecord::new(
    "Rig for War",
    "3edd0515-dcc4-4cb5-8b54-9c00173d8a6d",
    "Diana Franco",
    crate::card::CardRules::unsupported(),
);

// EOE 157 — Roving Actuator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROVING_ACTUATOR: CardRecord = CardRecord::new(
    "Roving Actuator",
    "1111173b-ea49-4de4-b5b0-07d768c626b9",
    "Sergey Glushakov",
    crate::card::CardRules::unsupported(),
);

// EOE 158 — Ruinous Rampage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUINOUS_RAMPAGE: CardRecord = CardRecord::new(
    "Ruinous Rampage",
    "91d7a4c2-1a4b-4e9f-b543-225b6906752f",
    "David Astruga",
    crate::card::CardRules::unsupported(),
);

// EOE 159 — Rust Harvester
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUST_HARVESTER: CardRecord = CardRecord::new(
    "Rust Harvester",
    "7ce58765-d2f4-4fbf-8635-580c9400ff2e",
    "Jake Murray",
    crate::card::CardRules::unsupported(),
);

// EOE 160 — Slagdrill Scrapper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SLAGDRILL_SCRAPPER: CardRecord = CardRecord::new(
    "Slagdrill Scrapper",
    "155dcf54-8fdb-4715-97dc-4eb5d3d80d78",
    "Edgar Sánchez Hidalgo",
    crate::card::CardRules::unsupported(),
);

// EOE 161 — Systems Override
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SYSTEMS_OVERRIDE: CardRecord = CardRecord::new(
    "Systems Override",
    "2a34c71b-8d3c-435b-9cf8-4902f997d10d",
    "Hardy Fowler",
    crate::card::CardRules::unsupported(),
);

// EOE 162 — Tannuk, Steadfast Second
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TANNUK_STEADFAST_SECOND: CardRecord = CardRecord::new(
    "Tannuk, Steadfast Second",
    "44607ed3-9523-40ac-9f61-0edd011cf762",
    "Raymond Swanland",
    crate::card::CardRules::unsupported(),
);

// EOE 163 — Terminal Velocity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TERMINAL_VELOCITY: CardRecord = CardRecord::new(
    "Terminal Velocity",
    "1d18dc06-16f0-4a3b-8d52-dbf4aa2c393d",
    "Xabi Gaztelua",
    crate::card::CardRules::unsupported(),
);

// EOE 164 — Terrapact Intimidator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TERRAPACT_INTIMIDATOR: CardRecord = CardRecord::new(
    "Terrapact Intimidator",
    "13fe3358-fd68-4245-a2ad-aa9200cf4655",
    "Slawomir Maniak",
    crate::card::CardRules::unsupported(),
);

// EOE 165 — Territorial Bruntar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TERRITORIAL_BRUNTAR: CardRecord = CardRecord::new(
    "Territorial Bruntar",
    "dbb25585-6048-4a85-828e-675bf0da6508",
    "Julie Dillon",
    crate::card::CardRules::unsupported(),
);

// EOE 166 — Vaultguard Trooper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VAULTGUARD_TROOPER: CardRecord = CardRecord::new(
    "Vaultguard Trooper",
    "36afe3b1-43a2-47b4-bc0a-24efb1e2e5a0",
    "David Palumbo",
    crate::card::CardRules::unsupported(),
);

// EOE 167 — Warmaker Gunship
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WARMAKER_GUNSHIP: CardRecord = CardRecord::new(
    "Warmaker Gunship",
    "9e5957f4-1cae-4989-8b40-27fc6e2fcf5e",
    "Julian Kok Joon Wen",
    crate::card::CardRules::unsupported(),
);

// EOE 168 — Weapons Manufacturing
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WEAPONS_MANUFACTURING: CardRecord = CardRecord::new(
    "Weapons Manufacturing",
    "a058f1a6-318c-4bba-981e-ace079ada806",
    "Marco Gorlei",
    crate::card::CardRules::unsupported(),
);

// EOE 169 — Weftstalker Ardent
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WEFTSTALKER_ARDENT: CardRecord = CardRecord::new(
    "Weftstalker Ardent",
    "cddb48cc-8eb1-47ce-90f0-7aad1e93e2c4",
    "Valera Lutfullina",
    crate::card::CardRules::unsupported(),
);

// EOE 170 — Zookeeper Mechan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ZOOKEEPER_MECHAN: CardRecord = CardRecord::new(
    "Zookeeper Mechan",
    "8d5cd0be-4337-4aba-a4f6-5adab7735a73",
    "Justyna Dura",
    crate::card::CardRules::unsupported(),
);

// EOE 171 — Atmospheric Greenhouse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ATMOSPHERIC_GREENHOUSE: CardRecord = CardRecord::new(
    "Atmospheric Greenhouse",
    "bf05e378-7a0c-49e3-8c6e-c0fd56796434",
    "Sergey Glushakov",
    crate::card::CardRules::unsupported(),
);

// EOE 172 — Bioengineered Future
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BIOENGINEERED_FUTURE: CardRecord = CardRecord::new(
    "Bioengineered Future",
    "800ee479-c1dc-4dd0-9b98-436c78997958",
    "Constantin Marin",
    crate::card::CardRules::unsupported(),
);

// EOE 173 — Biosynthic Burst
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BIOSYNTHIC_BURST: CardRecord = CardRecord::new(
    "Biosynthic Burst",
    "6d73a4a8-5d52-4c12-a96a-45cd202bcc62",
    "Loïc Canavaggia",
    crate::card::CardRules::unsupported(),
);

// EOE 174 — Blooming Stinger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLOOMING_STINGER: CardRecord = CardRecord::new(
    "Blooming Stinger",
    "2ee859bd-d99c-4b1d-9372-7ff4fc1e8c6a",
    "Alexandre Honoré",
    crate::card::CardRules::unsupported(),
);

// EOE 175 — Broodguard Elite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BROODGUARD_ELITE: CardRecord = CardRecord::new(
    "Broodguard Elite",
    "08b1d019-65ab-4dea-9076-041fd6338a35",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// EOE 176 — Close Encounter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLOSE_ENCOUNTER: CardRecord = CardRecord::new(
    "Close Encounter",
    "a8a77351-9115-470f-8141-222c1916b337",
    "Inkognit",
    crate::card::CardRules::unsupported(),
);

// EOE 177 — Diplomatic Relations
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIPLOMATIC_RELATIONS: CardRecord = CardRecord::new(
    "Diplomatic Relations",
    "e0a104c5-61fb-4733-97ab-a31a15a49443",
    "Néstor Ossandón Leal",
    crate::card::CardRules::unsupported(),
);

// EOE 177† — Diplomatic Relations (alternate printing)
const DIPLOMATIC_RELATIONS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DIPLOMATIC_RELATIONS,
    1,
    "143e5853-9a31-4c8d-b21d-5ef120eb6952",
    "Néstor Ossandón Leal",
);

// EOE 178 — Drix Fatemaker
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRIX_FATEMAKER: CardRecord = CardRecord::new(
    "Drix Fatemaker",
    "1beb7566-305e-4091-bdc4-cf4c789ac05a",
    "Anna Pavleeva",
    crate::card::CardRules::unsupported(),
);

// EOE 179 — Edge Rover
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EDGE_ROVER: CardRecord = CardRecord::new(
    "Edge Rover",
    "90741ec9-6893-42d4-b510-8664666094e3",
    "Francisco Badilla",
    crate::card::CardRules::unsupported(),
);

// EOE 180 — Eumidian Terrabotanist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EUMIDIAN_TERRABOTANIST: CardRecord = CardRecord::new(
    "Eumidian Terrabotanist",
    "64fb2981-86ed-478a-89cd-c6bb078a5bc7",
    "Loïc Canavaggia",
    crate::card::CardRules::unsupported(),
);

// EOE 181 — Eusocial Engineering
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EUSOCIAL_ENGINEERING: CardRecord = CardRecord::new(
    "Eusocial Engineering",
    "011bd7d8-6d60-482a-91b7-d3f0aad13b71",
    "Francisco Badilla",
    crate::card::CardRules::unsupported(),
);

// EOE 182 — Famished Worldsire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FAMISHED_WORLDSIRE: CardRecord = CardRecord::new(
    "Famished Worldsire",
    "2934c9c8-d23a-462b-83d5-94e88c8663ac",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// EOE 183 — Frenzied Baloth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FRENZIED_BALOTH: CardRecord = CardRecord::new(
    "Frenzied Baloth",
    "c72d85e9-a0bc-4f73-8d73-c58843577f4e",
    "Diana Franco",
    crate::card::CardRules::unsupported(),
);

// EOE 184 — Fungal Colossus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FUNGAL_COLOSSUS: CardRecord = CardRecord::new(
    "Fungal Colossus",
    "9dc9559f-cece-4c85-807c-158291666007",
    "Sergey Glushakov",
    crate::card::CardRules::unsupported(),
);

// EOE 185 — Galactic Wayfarer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GALACTIC_WAYFARER: CardRecord = CardRecord::new(
    "Galactic Wayfarer",
    "85b898d2-050f-49a2-87af-07d54d105336",
    "Quintin Gleim",
    crate::card::CardRules::unsupported(),
);

// EOE 186 — Gene Pollinator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GENE_POLLINATOR: CardRecord = CardRecord::new(
    "Gene Pollinator",
    "ce7a8eec-a029-4ee1-b2d6-405d903d4640",
    "Milivoj Ćeran",
    crate::card::CardRules::unsupported(),
);

// EOE 187 — Germinating Wurm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GERMINATING_WURM: CardRecord = CardRecord::new(
    "Germinating Wurm",
    "fcde173a-6314-4904-bddd-68b2ab1e4867",
    "Monztre",
    crate::card::CardRules::unsupported(),
);

// EOE 188 — Glacier Godmaw
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GLACIER_GODMAW: CardRecord = CardRecord::new(
    "Glacier Godmaw",
    "d3291c51-e963-4970-813d-9a06a47aa71e",
    "Bruce Brenneise",
    crate::card::CardRules::unsupported(),
);

// EOE 189 — Harmonious Grovestrider
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HARMONIOUS_GROVESTRIDER: CardRecord = CardRecord::new(
    "Harmonious Grovestrider",
    "e320abed-f145-42d3-b402-4f82e3a56389",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// EOE 190 — Hemosymbic Mite
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HEMOSYMBIC_MITE: CardRecord = CardRecord::new(
    "Hemosymbic Mite",
    "c14137a4-2d44-444c-ad50-e2edf9380571",
    "Amanda Lee",
    crate::card::CardRules::unsupported(),
);

// EOE 191 — Icecave Crasher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ICECAVE_CRASHER: CardRecord = CardRecord::new(
    "Icecave Crasher",
    "e6c1ed0c-0c0d-47a7-8ebc-67854cb226e0",
    "Julia Metzger",
    crate::card::CardRules::unsupported(),
);

// EOE 192 — Icetill Explorer (alternate printing)
const ICETILL_EXPLORER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ICETILL_EXPLORER,
    1,
    "d9482aab-6ddf-48e1-84fa-b13d5ff81e69",
    "Warren Mahy",
);

// EOE 193 — Intrepid Tenderfoot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INTREPID_TENDERFOOT: CardRecord = CardRecord::new(
    "Intrepid Tenderfoot",
    "809df0ea-deff-47b9-83df-cc1f360d377e",
    "Xavier Ribeiro",
    crate::card::CardRules::unsupported(),
);

// EOE 194 — Larval Scoutlander
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LARVAL_SCOUTLANDER: CardRecord = CardRecord::new(
    "Larval Scoutlander",
    "d6083f43-58dc-46fc-aeff-347b1080417b",
    "Javier Charro",
    crate::card::CardRules::unsupported(),
);

// EOE 195 — Lashwhip Predator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LASHWHIP_PREDATOR: CardRecord = CardRecord::new(
    "Lashwhip Predator",
    "24553e98-29a9-47e3-91c7-9add708d9ad1",
    "Brian Valeza",
    crate::card::CardRules::unsupported(),
);

// EOE 196 — Loading Zone
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LOADING_ZONE: CardRecord = CardRecord::new(
    "Loading Zone",
    "0d2c95bd-79af-4a23-b265-62cc0b164e3e",
    "Matt Stewart",
    crate::card::CardRules::unsupported(),
);

// EOE 197 — Meltstrider Eulogist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MELTSTRIDER_EULOGIST: CardRecord = CardRecord::new(
    "Meltstrider Eulogist",
    "df61aa0c-effc-4d57-be19-876a82c41d33",
    "Jason A. Engle",
    crate::card::CardRules::unsupported(),
);

// EOE 198 — Meltstrider's Gear
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MELTSTRIDER_S_GEAR: CardRecord = CardRecord::new(
    "Meltstrider's Gear",
    "d75629cb-91e2-46fa-9c80-6feb29e1ceb8",
    "Camille Alquier",
    crate::card::CardRules::unsupported(),
);

// EOE 199 — Meltstrider's Resolve
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MELTSTRIDER_S_RESOLVE: CardRecord = CardRecord::new(
    "Meltstrider's Resolve",
    "53c08af4-b975-4d9d-baba-73e6727f2778",
    "Carlos Palma Cruchaga",
    crate::card::CardRules::unsupported(),
);

// EOE 200 — Mightform Harmonizer (alternate printing)
const MIGHTFORM_HARMONIZER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MIGHTFORM_HARMONIZER,
    1,
    "f32302f1-b54f-4489-9d0b-9b771e59da06",
    "Bartek Fedyczak",
);

// EOE 201 — Ouroboroid
pub(in crate::card::sets) static OUROBOROID: CardRecord = CardRecord::new(
    "Ouroboroid",
    "209c591a-4ab2-4e89-9523-a7b766cf4e51",
    "Samuel Perin",
    // A 1/3 that doubles itself every combat and takes the rest of the board
    // with it: one counter each the first turn, two the next, four after
    // that.
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Plant", "Wurm"], 1, 3).with_ability(
        AbilityDef::triggered(
            "At the beginning of combat on your turn, put X +1/+1 counters on each creature you \
             control, where X is this creature's power.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::You,
            },
            // X is read once, as the ability resolves, and every creature
            // gets that many -- including the Wurm, whose own growth does
            // not raise the number partway through.
            EffectDef::AddCounters {
                // "Each creature you control" includes the Wurm itself, so the counters it
                // hands out make the next round of them bigger.
                object: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ))),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::SourcePower,
            },
        ),
    ),
);

// EOE 202 — Pull Through the Weft
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PULL_THROUGH_THE_WEFT: CardRecord = CardRecord::new(
    "Pull Through the Weft",
    "a70d0877-1a1e-436b-bf8c-0ff6df9efc6a",
    "Andrew Mar",
    crate::card::CardRules::unsupported(),
);

// EOE 203 — Sami's Curiosity
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAMI_S_CURIOSITY: CardRecord = CardRecord::new(
    "Sami's Curiosity",
    "703ad0f3-bd05-42b0-85fb-0cd37807dc91",
    "Tuan Duong Chu",
    crate::card::CardRules::unsupported(),
);

// EOE 204 — Seedship Agrarian
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEEDSHIP_AGRARIAN: CardRecord = CardRecord::new(
    "Seedship Agrarian",
    "2fc7946d-37b0-4dc8-9daa-8d2204d8e4d2",
    "Helge C. Balzer",
    crate::card::CardRules::unsupported(),
);

// EOE 205 — Seedship Impact
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEEDSHIP_IMPACT: CardRecord = CardRecord::new(
    "Seedship Impact",
    "d060d95f-f33a-4fc0-b002-c394d4cd82ce",
    "Constantin Marin",
    crate::card::CardRules::unsupported(),
);

// EOE 206 — Shattered Wings
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHATTERED_WINGS: CardRecord = CardRecord::new(
    "Shattered Wings",
    "bbece737-bd4f-4dc8-bf5c-f77930246ab1",
    "Sergey Glushakov",
    crate::card::CardRules::unsupported(),
);

// EOE 207 — Skystinger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKYSTINGER: CardRecord = CardRecord::new(
    "Skystinger",
    "dfea9941-3675-4c0f-bc4d-981c28deed36",
    "Carlos Palma Cruchaga",
    crate::card::CardRules::unsupported(),
);

// EOE 208 — Sledge-Class Seedship
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SLEDGE_CLASS_SEEDSHIP: CardRecord = CardRecord::new(
    "Sledge-Class Seedship",
    "dc0cb4b6-cc20-49ea-84b2-2f3af3b0a19e",
    "Leon Tukker",
    crate::card::CardRules::unsupported(),
);

// EOE 209 — Tapestry Warden
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TAPESTRY_WARDEN: CardRecord = CardRecord::new(
    "Tapestry Warden",
    "7cbbab6c-43ae-4e50-97ce-532a3316591a",
    "Andreas Zafiratos",
    crate::card::CardRules::unsupported(),
);

// EOE 210 — Terrasymbiosis
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TERRASYMBIOSIS: CardRecord = CardRecord::new(
    "Terrasymbiosis",
    "26008c7d-5dbe-4da2-b475-4dd307e7bc68",
    "Viko Menezes",
    crate::card::CardRules::unsupported(),
);

// EOE 211 — Thawbringer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THAWBRINGER: CardRecord = CardRecord::new(
    "Thawbringer",
    "0b7f934f-8eb4-408b-a55f-245ec5cc4a8a",
    "Olivier Bernard",
    crate::card::CardRules::unsupported(),
);

// EOE 212 — Alpharael, Dreaming Acolyte
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALPHARAEL_DREAMING_ACOLYTE: CardRecord = CardRecord::new(
    "Alpharael, Dreaming Acolyte",
    "349a2211-2b23-418d-a1ef-1c72ad2e171d",
    "Cristi Balanescu",
    crate::card::CardRules::unsupported(),
);

// EOE 213 — Biomechan Engineer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BIOMECHAN_ENGINEER: CardRecord = CardRecord::new(
    "Biomechan Engineer",
    "7edcef2c-029c-46bd-bfeb-ff56a57dd63a",
    "Monztre",
    crate::card::CardRules::unsupported(),
);

// EOE 214 — Biotech Specialist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BIOTECH_SPECIALIST: CardRecord = CardRecord::new(
    "Biotech Specialist",
    "127c221f-94e7-4a0e-a7a6-79ef399862d3",
    "Alexandre Honoré",
    crate::card::CardRules::unsupported(),
);

// EOE 215 — Cosmogoyf
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COSMOGOYF: CardRecord = CardRecord::new(
    "Cosmogoyf",
    "5e07d3c6-60a5-44d1-a926-6414be85bd50",
    "Chris Rahn",
    crate::card::CardRules::unsupported(),
);

// EOE 216 — Dyadrine, Synthesis Amalgam
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DYADRINE_SYNTHESIS_AMALGAM: CardRecord = CardRecord::new(
    "Dyadrine, Synthesis Amalgam",
    "994ca692-7138-4dcb-bf46-5da530f86036",
    "Igor Grechanyi",
    crate::card::CardRules::unsupported(),
);

// EOE 217 — Genemorph Imago
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GENEMORPH_IMAGO: CardRecord = CardRecord::new(
    "Genemorph Imago",
    "40f0ecf7-f49e-46ff-aa5b-9ff5361b72c5",
    "Brian Valeza",
    crate::card::CardRules::unsupported(),
);

// EOE 218 — Haliya, Ascendant Cadet
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HALIYA_ASCENDANT_CADET: CardRecord = CardRecord::new(
    "Haliya, Ascendant Cadet",
    "683d6eba-7f98-4105-b318-7f2290012f32",
    "Justyna Dura",
    crate::card::CardRules::unsupported(),
);

// EOE 219 — Infinite Guideline Station
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INFINITE_GUIDELINE_STATION: CardRecord = CardRecord::new(
    "Infinite Guideline Station",
    "5688894a-bbec-476b-ae2e-94000be258d0",
    "Piotr Dura",
    crate::card::CardRules::unsupported(),
);

// EOE 220 — Interceptor Mechan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INTERCEPTOR_MECHAN: CardRecord = CardRecord::new(
    "Interceptor Mechan",
    "198211af-f413-4e9b-9baf-4b4fcb81eadc",
    "Leonardo Santanna",
    crate::card::CardRules::unsupported(),
);

// EOE 221 — Mm'menon, Uthros Exile
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MM_MENON_UTHROS_EXILE: CardRecord = CardRecord::new(
    "Mm'menon, Uthros Exile",
    "5546c044-5826-48c3-9d28-866f3c3c5f2c",
    "Fajareka Setiawan",
    crate::card::CardRules::unsupported(),
);

// EOE 222 — Mutinous Massacre
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MUTINOUS_MASSACRE: CardRecord = CardRecord::new(
    "Mutinous Massacre",
    "42d5034f-18f0-4d57-9840-6be52c286247",
    "Quintin Gleim",
    crate::card::CardRules::unsupported(),
);

// EOE 223 — Pinnacle Emissary
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PINNACLE_EMISSARY: CardRecord = CardRecord::new(
    "Pinnacle Emissary",
    "3c922347-f05f-40a4-bbee-6bc02a1e0de5",
    "Alejandro Pacheco",
    crate::card::CardRules::unsupported(),
);

// EOE 224 — Ragost, Deft Gastronaut
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RAGOST_DEFT_GASTRONAUT: CardRecord = CardRecord::new(
    "Ragost, Deft Gastronaut",
    "011374c3-f69d-4573-8c32-5bd0fe083d6a",
    "Zack Stella",
    crate::card::CardRules::unsupported(),
);

// EOE 225 — Sami, Ship's Engineer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAMI_SHIP_S_ENGINEER: CardRecord = CardRecord::new(
    "Sami, Ship's Engineer",
    "75c38cc9-07de-46d4-8195-f04b2b7e0fee",
    "Zara Alfonso",
    crate::card::CardRules::unsupported(),
);

// EOE 226 — Sami, Wildcat Captain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAMI_WILDCAT_CAPTAIN: CardRecord = CardRecord::new(
    "Sami, Wildcat Captain",
    "bed64207-9193-4770-8f8f-e3203289d5a6",
    "Kieran Yanner",
    crate::card::CardRules::unsupported(),
);

// EOE 227 — Seedship Broodtender
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEEDSHIP_BROODTENDER: CardRecord = CardRecord::new(
    "Seedship Broodtender",
    "1abc176f-2ccf-4371-b4b5-030dd99ff7fc",
    "Eric Wilkerson",
    crate::card::CardRules::unsupported(),
);

// EOE 228 — Singularity Rupture
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SINGULARITY_RUPTURE: CardRecord = CardRecord::new(
    "Singularity Rupture",
    "a34012e3-ec7a-4713-a2c2-f8efff49e364",
    "Liiga Smilshkalne",
    crate::card::CardRules::unsupported(),
);

// EOE 229 — Space-Time Anomaly
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPACE_TIME_ANOMALY: CardRecord = CardRecord::new(
    "Space-Time Anomaly",
    "edb8dc2a-ddce-48fa-b57e-0e57c87c6671",
    "Loïc Canavaggia",
    crate::card::CardRules::unsupported(),
);

// EOE 230 — Station Monitor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STATION_MONITOR: CardRecord = CardRecord::new(
    "Station Monitor",
    "ba9f6d16-ee3e-4fbb-b78a-6292188eb61f",
    "Camille Alquier",
    crate::card::CardRules::unsupported(),
);

// EOE 231 — Syr Vondam, Sunstar Exemplar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SYR_VONDAM_SUNSTAR_EXEMPLAR: CardRecord = CardRecord::new(
    "Syr Vondam, Sunstar Exemplar",
    "49554198-549b-4066-86ce-77a03fda0a2f",
    "Ryan Pancoast",
    crate::card::CardRules::unsupported(),
);

// EOE 232 — Syr Vondam, the Lucent
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SYR_VONDAM_THE_LUCENT: CardRecord = CardRecord::new(
    "Syr Vondam, the Lucent",
    "ea954205-5ff5-493b-bf30-6212042c2bc9",
    "Cristi Balanescu",
    crate::card::CardRules::unsupported(),
);

// EOE 233 — Tannuk, Memorial Ensign
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TANNUK_MEMORIAL_ENSIGN: CardRecord = CardRecord::new(
    "Tannuk, Memorial Ensign",
    "52498b7b-0389-4e7b-b29f-7ac86aab9229",
    "David Auden Nash",
    crate::card::CardRules::unsupported(),
);

// EOE 234 — All-Fates Scroll
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ALL_FATES_SCROLL: CardRecord = CardRecord::new(
    "All-Fates Scroll",
    "3a5ed010-cb17-45df-b169-ebc807dae534",
    "Sam Guay",
    crate::card::CardRules::unsupported(),
);

// EOE 235 — Bygone Colossus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BYGONE_COLOSSUS: CardRecord = CardRecord::new(
    "Bygone Colossus",
    "4bb8f2ef-4398-4a07-9130-5005356a3b4a",
    "Maxime Minard",
    crate::card::CardRules::unsupported(),
);

// EOE 236 — Chrome Companion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHROME_COMPANION: CardRecord = CardRecord::new(
    "Chrome Companion",
    "8ef269a0-1cb9-4901-81e6-43db3ae3756c",
    "Gray Highsmith",
    crate::card::CardRules::unsupported(),
);

// EOE 237 — Dauntless Scrapbot
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DAUNTLESS_SCRAPBOT: CardRecord = CardRecord::new(
    "Dauntless Scrapbot",
    "0efe5342-42b7-4f49-b4b4-d77055508c4d",
    "Alix Branwyn",
    crate::card::CardRules::unsupported(),
);

// EOE 238 — Dawnsire, Sunstar Dreadnought
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DAWNSIRE_SUNSTAR_DREADNOUGHT: CardRecord = CardRecord::new(
    "Dawnsire, Sunstar Dreadnought",
    "6133355c-3dcf-466a-b771-fe6c44d4fa4d",
    "Jaime Jones",
    crate::card::CardRules::unsupported(),
);

// EOE 239 — The Dominion Bracelet
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_DOMINION_BRACELET: CardRecord = CardRecord::new(
    "The Dominion Bracelet",
    "f5360880-2849-45d6-b1aa-08c7e01083af",
    "Nathaniel Himawan",
    crate::card::CardRules::unsupported(),
);

// EOE 240 — The Endstone (alternate printing)
const THE_ENDSTONE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_ENDSTONE,
    1,
    "a451a459-18e0-4c53-a171-3e9da534ebf1",
    "Ryan Pancoast",
);

// EOE 241 — The Eternity Elevator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THE_ETERNITY_ELEVATOR: CardRecord = CardRecord::new(
    "The Eternity Elevator",
    "1bb90ab9-43b9-4991-806a-0afc4d8caf5f",
    "Josu Solano",
    crate::card::CardRules::unsupported(),
);

// EOE 242 — Extinguisher Battleship
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EXTINGUISHER_BATTLESHIP: CardRecord = CardRecord::new(
    "Extinguisher Battleship",
    "5541cdd2-84a6-4667-83eb-fffbe5b3cd3d",
    "Danny Schwartz",
    crate::card::CardRules::unsupported(),
);

// EOE 243 — Nutrient Block
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NUTRIENT_BLOCK: CardRecord = CardRecord::new(
    "Nutrient Block",
    "a26064bb-c568-4ed6-86db-3aab69b050db",
    "Francisco Miyara",
    crate::card::CardRules::unsupported(),
);

// EOE 244 — Pinnacle Kill-Ship
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PINNACLE_KILL_SHIP: CardRecord = CardRecord::new(
    "Pinnacle Kill-Ship",
    "bf784de8-5ae2-4c07-92bb-a5b7f593b773",
    "Alexandre Honoré",
    crate::card::CardRules::unsupported(),
);

// EOE 245 — Survey Mechan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SURVEY_MECHAN: CardRecord = CardRecord::new(
    "Survey Mechan",
    "9b4278ea-6cd8-45ad-b024-daf3dedd29e0",
    "Johann Bodin",
    crate::card::CardRules::unsupported(),
);

// EOE 246 — Thaumaton Torpedo
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THAUMATON_TORPEDO: CardRecord = CardRecord::new(
    "Thaumaton Torpedo",
    "1817f1b5-960a-435c-bdec-8cc8cbcb3358",
    "Madeline Boni",
    crate::card::CardRules::unsupported(),
);

// EOE 247 — Thrumming Hivepool
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static THRUMMING_HIVEPOOL: CardRecord = CardRecord::new(
    "Thrumming Hivepool",
    "85caf659-7b43-462e-a342-34703d46eb57",
    "Rob Rey",
    crate::card::CardRules::unsupported(),
);

// EOE 248 — Virulent Silencer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VIRULENT_SILENCER: CardRecord = CardRecord::new(
    "Virulent Silencer",
    "4120fcfc-3547-4774-a15d-b9cccac04e76",
    "Kenn Yap",
    crate::card::CardRules::unsupported(),
);

// EOE 249 — Wurmwall Sweeper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WURMWALL_SWEEPER: CardRecord = CardRecord::new(
    "Wurmwall Sweeper",
    "9ace282a-5901-4d36-ad21-17eb88bc5138",
    "Hardy Fowler",
    crate::card::CardRules::unsupported(),
);

// EOE 250 — Adagia, Windswept Bastion
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ADAGIA_WINDSWEPT_BASTION: CardRecord = CardRecord::new(
    "Adagia, Windswept Bastion",
    "c634273a-94b0-4104-9d10-ae522ece1fc7",
    "Adam Paquette",
    crate::card::CardRules::unsupported(),
);

// EOE 251 — Breeding Pool (reprint)
const BREEDING_POOL_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_dis::BREEDING_POOL,
    "3c750d5a-f743-41ff-b5ba-02025ca0bec2",
    "Constantin Marin",
);

// EOE 252 — Command Bridge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COMMAND_BRIDGE: CardRecord = CardRecord::new(
    "Command Bridge",
    "247670d2-a7cd-4ed7-9c77-704c7962b815",
    "Constantin Marin",
    crate::card::CardRules::unsupported(),
);

// EOE 253 — Evendo, Waking Haven
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EVENDO_WAKING_HAVEN: CardRecord = CardRecord::new(
    "Evendo, Waking Haven",
    "2fa09104-acbe-4410-b101-2fe6ac28efde",
    "Adam Paquette",
    crate::card::CardRules::unsupported(),
);

// EOE 254 — Godless Shrine (reprint)
const GODLESS_SHRINE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_gpt::GODLESS_SHRINE,
    "8c542ea4-98c3-4c2d-9066-205ab7aa697a",
    "Rob Rey",
);

// EOE 255 — Kavaron, Memorial World
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KAVARON_MEMORIAL_WORLD: CardRecord = CardRecord::new(
    "Kavaron, Memorial World",
    "60f3ca25-9dcc-4781-bf7b-ab6736d8db29",
    "Adam Paquette",
    crate::card::CardRules::unsupported(),
);

// EOE 256 — Sacred Foundry (reprint)
const SACRED_FOUNDRY_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rav::SACRED_FOUNDRY,
    "8b4e2642-3c87-4708-b9b4-2e7f7359ac7d",
    "Titus Lunter",
);

// EOE 257 — Secluded Starforge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SECLUDED_STARFORGE: CardRecord = CardRecord::new(
    "Secluded Starforge",
    "a997ff9f-045a-44a2-983d-f36414cef1ab",
    "Chris Rahn",
    crate::card::CardRules::unsupported(),
);

// EOE 258 — Stomping Ground (reprint)
const STOMPING_GROUND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_gpt::STOMPING_GROUND,
    "69be21b4-c613-47c6-ba57-f4785861af3e",
    "Bruce Brenneise",
);

// EOE 259 — Susur Secundi, Void Altar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUSUR_SECUNDI_VOID_ALTAR: CardRecord = CardRecord::new(
    "Susur Secundi, Void Altar",
    "aefb8c0d-2bc6-4bec-851e-0137b4abfb22",
    "Adam Paquette",
    crate::card::CardRules::unsupported(),
);

// EOE 260 — Uthros, Titanic Godcore
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UTHROS_TITANIC_GODCORE: CardRecord = CardRecord::new(
    "Uthros, Titanic Godcore",
    "11da39d6-cfa6-498d-91b1-11454cc7e5a3",
    "Adam Paquette",
    crate::card::CardRules::unsupported(),
);

// EOE 261 — Watery Grave (reprint)
const WATERY_GRAVE_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_rav::WATERY_GRAVE,
    "5b8170dc-6a90-46fc-9989-7575f3d402b5",
    "Sergey Glushakov",
);

// EOE 262 — Plains (reprint)
const PLAINS_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::PLAINS,
    "089ae01b-e042-4255-b0ee-17d8f416a8d9",
    "Adam Paquette",
);

// EOE 263 — Island (reprint)
const ISLAND_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::ISLAND,
    "76313c69-8ec7-49e3-a34e-ade26097284c",
    "Adam Paquette",
);

// EOE 264 — Swamp (reprint)
const SWAMP_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::SWAMP,
    "3e724e54-a622-4c77-8183-5a397a7c14a9",
    "Adam Paquette",
);

// EOE 265 — Mountain (reprint)
const MOUNTAIN_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::MOUNTAIN,
    "3204f401-1bcb-4d97-b15d-113a5d3c3e9f",
    "Adam Paquette",
);

// EOE 266 — Forest (reprint)
const FOREST_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &catalog_lea::FOREST,
    "8335b9f1-e726-423b-8ba7-6151448ab3fd",
    "Adam Paquette",
);

// EOE 267 — Plains (alternate printing)
const PLAINS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    1,
    "df31f72c-076e-4d8e-9975-6d6281ab71f6",
    "Alayna Danner",
);

// EOE 268 — Plains (alternate printing)
const PLAINS_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    2,
    "a26fd28e-d053-48b3-ab4c-2dc6fd33fa64",
    "Sergey Glushakov",
);

// EOE 269 — Island (alternate printing)
const ISLAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    1,
    "aaa212be-1db5-4493-86c8-c01d29348e31",
    "Sergey Glushakov",
);

// EOE 270 — Island (alternate printing)
const ISLAND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    2,
    "2fb9ecfd-8025-48c0-a306-b4deebde2949",
    "Liiga Smilshkalne",
);

// EOE 271 — Swamp (alternate printing)
const SWAMP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    1,
    "58496a56-437f-4204-9691-f63795e5cdab",
    "Sergey Glushakov",
);

// EOE 272 — Swamp (alternate printing)
const SWAMP_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    2,
    "07ab266b-ad6a-422e-9277-27d9f48d2c29",
    "Liiga Smilshkalne",
);

// EOE 273 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    1,
    "5cd39b01-9a06-4575-9c63-9fb3ba9ef101",
    "Sergey Glushakov",
);

// EOE 274 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    2,
    "de0ac2b0-175b-4e71-9172-ce2a72234818",
    "Julian Kok Joon Wen",
);

// EOE 275 — Forest (alternate printing)
const FOREST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    1,
    "fb62605c-a58e-4e53-8336-b2bee316b5a6",
    "Sergey Glushakov",
);

// EOE 276 — Forest (alternate printing)
const FOREST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    2,
    "673141ec-826f-4132-8282-d499990495a7",
    "Julian Kok Joon Wen",
);

// EOE 277 — Adagia, Windswept Bastion (alternate printing)
const ADAGIA_WINDSWEPT_BASTION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ADAGIA_WINDSWEPT_BASTION,
    1,
    "4d6901e7-d519-4dfc-a74d-63fff5e4ffae",
    "Piotr Dura",
);

// EOE 278 — Breeding Pool (alternate printing)
const BREEDING_POOL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_dis::BREEDING_POOL,
    1,
    "1c575871-9b46-4cd4-8596-54dc04f76456",
    "Chris Ostrowski",
);

// EOE 279 — Evendo, Waking Haven (alternate printing)
const EVENDO_WAKING_HAVEN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EVENDO_WAKING_HAVEN,
    1,
    "91d18bf4-0e95-41f7-a65d-238fc010fd82",
    "Piotr Dura",
);

// EOE 280 — Godless Shrine (alternate printing)
const GODLESS_SHRINE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_gpt::GODLESS_SHRINE,
    1,
    "7bb18340-5507-44da-b008-6b010daad617",
    "Chris Ostrowski",
);

// EOE 281 — Kavaron, Memorial World (alternate printing)
const KAVARON_MEMORIAL_WORLD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KAVARON_MEMORIAL_WORLD,
    1,
    "0f03d025-3ec8-4ae4-93e3-0fb96e105b7f",
    "Piotr Dura",
);

// EOE 282 — Sacred Foundry (alternate printing)
const SACRED_FOUNDRY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_rav::SACRED_FOUNDRY,
    1,
    "9f74d8ad-9db9-4162-9d20-9779233a634e",
    "Chris Ostrowski",
);

// EOE 283 — Stomping Ground (alternate printing)
const STOMPING_GROUND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_gpt::STOMPING_GROUND,
    1,
    "f802db1f-6533-4118-b677-562bfb135904",
    "Chris Ostrowski",
);

// EOE 284 — Susur Secundi, Void Altar (alternate printing)
const SUSUR_SECUNDI_VOID_ALTAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUSUR_SECUNDI_VOID_ALTAR,
    1,
    "f6bb05de-4cf5-479d-a0f7-1a2792a6b6d0",
    "Piotr Dura",
);

// EOE 285 — Uthros, Titanic Godcore (alternate printing)
const UTHROS_TITANIC_GODCORE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &UTHROS_TITANIC_GODCORE,
    1,
    "a76ecc4f-5e77-4d96-9df7-fe703bbc39df",
    "Piotr Dura",
);

// EOE 286 — Watery Grave (alternate printing)
const WATERY_GRAVE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_rav::WATERY_GRAVE,
    1,
    "b248ff3a-e10d-4797-8c04-7f4094608d32",
    "Chris Ostrowski",
);

// EOE 287 — Tezzeret, Cruel Captain (alternate printing)
const TEZZERET_CRUEL_CAPTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TEZZERET_CRUEL_CAPTAIN,
    1,
    "5cf53baa-b0c3-4190-a5c6-c141d54cff32",
    "Magali Villeneuve",
);

// EOE 288 — Astelli Reclaimer (alternate printing)
const ASTELLI_RECLAIMER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ASTELLI_RECLAIMER,
    1,
    "f01aaada-2525-4f92-83e8-bbc61fa7b9cd",
    "Benjamin Ee",
);

// EOE 289 — Haliya, Guided by Light (alternate printing)
const HALIYA_GUIDED_BY_LIGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HALIYA_GUIDED_BY_LIGHT,
    1,
    "e97f7f80-51f0-4366-a094-ee75ce03adb4",
    "Eleonor Piteira",
);

// EOE 290 — Mm'menon, the Right Hand (alternate printing)
const MM_MENON_THE_RIGHT_HAND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MM_MENON_THE_RIGHT_HAND,
    1,
    "53bd9149-e157-4f0b-becf-0b64f47dbad3",
    "Dominik Mayer",
);

// EOE 291 — Starwinder (alternate printing)
const STARWINDER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STARWINDER,
    1,
    "637a4457-5600-4d33-81c7-f4009df3d8a5",
    "Justin Hernandez & Alexis Hernandez",
);

// EOE 292 — Alpharael, Stonechosen (alternate printing)
const ALPHARAEL_STONECHOSEN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ALPHARAEL_STONECHOSEN,
    1,
    "cabf4bdb-160a-4d0e-aa07-87bb74aef34a",
    "Jeremy Wilson",
);

// EOE 293 — Elegy Acolyte (alternate printing)
const ELEGY_ACOLYTE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ELEGY_ACOLYTE,
    1,
    "ffa43efe-15ea-4c38-ab4a-764baa084e44",
    "Justin Hernandez & Alexis Hernandez",
);

// EOE 294 — Xu-Ifit, Osteoharmonist (alternate printing)
const XU_IFIT_OSTEOHARMONIST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &XU_IFIT_OSTEOHARMONIST,
    1,
    "98e7fb9e-44c2-4fa6-8d81-895f909ea9b7",
    "Ashley Mackenzie",
);

// EOE 295 — Possibility Technician (alternate printing)
const POSSIBILITY_TECHNICIAN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &POSSIBILITY_TECHNICIAN,
    1,
    "d68ed960-c7ea-454c-8395-6cbb621a7940",
    "Matthew G. Lewis",
);

// EOE 296 — Tannuk, Steadfast Second (alternate printing)
const TANNUK_STEADFAST_SECOND_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TANNUK_STEADFAST_SECOND,
    1,
    "a94a4760-cf43-4f27-96e1-be9d13611d99",
    "Pascal Blanché",
);

// EOE 297 — Mightform Harmonizer
pub(in crate::card::sets) static MIGHTFORM_HARMONIZER: CardRecord = CardRecord::new(
    "Mightform Harmonizer",
    "29bc9be4-4fc3-440a-a851-0c7f8989c9b5",
    "Jessica Fong",
// Four mana for a 4/4 that makes every land drop a pump spell, or three
    // for one turn of it now and the whole card again later.
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Insect", "Druid"], 4, 4)
        .with_abilities(&[
            AbilityDef::triggered_with_targets(
                "Landfall — Whenever a land you control enters, double the power of target creature you \
                 control until end of turn.",
                TriggerEventDef::zone_changed(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]), None, Some(ZoneKind::Battlefield)),
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                // Doubling is +X/+0 where X is the target's power as the trigger resolves,
                // so two landfalls in a turn compound: the second reads the size the first
                // left behind, and a creature answered in between doubles nothing.
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::TargetPower(TargetIndex::PRIMARY),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            abilities::warp(
                &[CostDef::Mana(mana_cost!("{2}{G}"))],
                "Warp {2}{G} (You may cast this card from your hand for its warp cost. Exile this \
                creature at the beginning of the next end step, then you may cast it from exile on a \
                later turn.)",
            ),
            abilities::warped_exile(),
        ]),
);

// EOE 298 — Dyadrine, Synthesis Amalgam (alternate printing)
const DYADRINE_SYNTHESIS_AMALGAM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DYADRINE_SYNTHESIS_AMALGAM,
    1,
    "3cba1ffa-9b43-41f9-a5ea-57cd48013c1c",
    "Matthew G. Lewis",
);

// EOE 299 — Genemorph Imago (alternate printing)
const GENEMORPH_IMAGO_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &GENEMORPH_IMAGO,
    1,
    "37c2ab54-e856-41d5-86ba-c6a7d2774546",
    "Benjamin Ee",
);

// EOE 300 — Ragost, Deft Gastronaut (alternate printing)
const RAGOST_DEFT_GASTRONAUT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RAGOST_DEFT_GASTRONAUT,
    1,
    "4a62293d-63a7-4038-b3e9-197bb2b9ec90",
    "Dominik Mayer",
);

// EOE 301 — Sami, Wildcat Captain (alternate printing)
const SAMI_WILDCAT_CAPTAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SAMI_WILDCAT_CAPTAIN,
    1,
    "968fb6d4-f4b9-4c8d-b296-fea002a8c003",
    "Ashley Mackenzie",
);

// EOE 302 — Syr Vondam, Sunstar Exemplar (alternate printing)
const SYR_VONDAM_SUNSTAR_EXEMPLAR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SYR_VONDAM_SUNSTAR_EXEMPLAR,
    1,
    "9c39b081-ad53-4c43-93f4-3a4ebb24299b",
    "Jeremy Wilson",
);

// EOE 303 — Beyond the Quiet (alternate printing)
const BEYOND_THE_QUIET_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BEYOND_THE_QUIET,
    1,
    "215f488b-6d33-4b08-88ff-b258c00515d6",
    "Serena Malyon",
);

// EOE 304 — Cosmogrand Zenith (alternate printing)
const COSMOGRAND_ZENITH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &COSMOGRAND_ZENITH,
    1,
    "d68d891d-333f-46c9-b5a3-3b1d4d3e4563",
    "Marlene Yui",
);

// EOE 305 — Quantum Riddler (alternate printing)
const QUANTUM_RIDDLER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &QUANTUM_RIDDLER,
    1,
    "2b0e16d5-a3e5-4e58-9f97-3d967618f015",
    "Cacho Rubione",
);

// EOE 306 — Starwinder (alternate printing)
const STARWINDER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &STARWINDER,
    2,
    "5a9c1d90-b2a3-40a9-86d7-ce6930acee0e",
    "Cacho Rubione",
);

// EOE 307 — Archenemy's Charm (alternate printing)
const ARCHENEMY_S_CHARM_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ARCHENEMY_S_CHARM,
    1,
    "c9e9b71b-a54c-4891-9c16-28aebfafb644",
    "Peter Diamond",
);

// EOE 308 — Devastating Onslaught (alternate printing)
const DEVASTATING_ONSLAUGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DEVASTATING_ONSLAUGHT,
    1,
    "376d4b86-b530-4063-b530-87cc5c32e2fa",
    "Deb JJ Lee",
);

// EOE 309 — Nova Hellkite (alternate printing)
const NOVA_HELLKITE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &NOVA_HELLKITE,
    1,
    "c627d558-98e8-459e-860b-7ac4dfac44de",
    "Micha Huigen",
);

// EOE 310 — Rust Harvester (alternate printing)
const RUST_HARVESTER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &RUST_HARVESTER,
    1,
    "7aca78c5-3ee4-49b8-a2bd-99f5ace9d9f7",
    "Scott Balmer",
);

// EOE 311 — Weapons Manufacturing (alternate printing)
const WEAPONS_MANUFACTURING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WEAPONS_MANUFACTURING,
    1,
    "b2c86408-5e19-46ca-a3ce-949eb148acde",
    "Micha Huigen",
);

// EOE 312 — Terrasymbiosis (alternate printing)
const TERRASYMBIOSIS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TERRASYMBIOSIS,
    1,
    "cc708b6c-6224-42b7-8249-a78fcb31fef4",
    "Jack Hughes",
);

// EOE 313 — Cosmogoyf (alternate printing)
const COSMOGOYF_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &COSMOGOYF,
    1,
    "15b98944-23c9-4609-b323-7925adaa9aac",
    "Princess Hidir",
);

// EOE 314 — Mutinous Massacre (alternate printing)
const MUTINOUS_MASSACRE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MUTINOUS_MASSACRE,
    1,
    "b4bdb1ac-abe2-4ce7-84cb-eb53196b4b01",
    "Jack Hughes",
);

// EOE 315 — Space-Time Anomaly (alternate printing)
const SPACE_TIME_ANOMALY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SPACE_TIME_ANOMALY,
    1,
    "b2e78e22-7a19-4e3a-93c5-6432bc5922ae",
    "Princess Hidir",
);

// EOE 316 — Secluded Starforge (alternate printing)
const SECLUDED_STARFORGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SECLUDED_STARFORGE,
    1,
    "9024ae78-8a60-4856-9d26-97e3d9310542",
    "Jaime A. Zuverza",
);

// EOE 317 — Anticausal Vestige (alternate printing)
const ANTICAUSAL_VESTIGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ANTICAUSAL_VESTIGE,
    1,
    "fb9ecadf-6145-49e7-972f-bfe28ca92266",
    "Chase Stone",
);

// EOE 318 — Exalted Sunborn (alternate printing)
const EXALTED_SUNBORN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EXALTED_SUNBORN,
    1,
    "c295983e-e8eb-4adc-8c72-9c3ecf3a29df",
    "Scott M. Fischer",
);

// EOE 319 — Hardlight Containment (alternate printing)
const HARDLIGHT_CONTAINMENT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &HARDLIGHT_CONTAINMENT,
    1,
    "a84523b1-bfa4-4506-96d3-f655727f1cfc",
    "Dominik Mayer",
);

// EOE 320 — Lightstall Inquisitor (alternate printing)
const LIGHTSTALL_INQUISITOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LIGHTSTALL_INQUISITOR,
    1,
    "ad782559-9b45-4a87-8ae6-82be6fd32f76",
    "Arif Wijaya",
);

// EOE 321 — Lumen-Class Frigate (alternate printing)
const LUMEN_CLASS_FRIGATE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LUMEN_CLASS_FRIGATE,
    1,
    "7fffe390-d16c-4891-b94a-99e895f219d0",
    "Zezhou Chen",
);

// EOE 322 — Pinnacle Starcage (alternate printing)
const PINNACLE_STARCAGE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PINNACLE_STARCAGE,
    1,
    "884f3feb-994f-4a65-b0e1-958e2fb38af1",
    "Leon Tukker",
);

// EOE 323 — The Seriema (alternate printing)
const THE_SERIEMA_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_SERIEMA,
    1,
    "9361996c-414b-4d0d-9d94-b6d5fe04e987",
    "Sergey Glushakov",
);

// EOE 324 — Sunstar Chaplain (alternate printing)
const SUNSTAR_CHAPLAIN_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUNSTAR_CHAPLAIN,
    1,
    "d1588748-3398-4b9d-97a6-8266f11d0862",
    "Valera Lutfullina",
);

// EOE 325 — Consult the Star Charts (alternate printing)
const CONSULT_THE_STAR_CHARTS_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CONSULT_THE_STAR_CHARTS,
    1,
    "94d11159-c389-4461-abf9-2a3be6b39d8d",
    "Antonio José Manzanedo",
);

// EOE 326 — Emissary Escort (alternate printing)
const EMISSARY_ESCORT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EMISSARY_ESCORT,
    1,
    "c2dca493-9f1e-479a-a1d2-893d4799be3e",
    "Igor Grechanyi",
);

// EOE 327 — Moonlit Meditation (alternate printing)
const MOONLIT_MEDITATION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MOONLIT_MEDITATION,
    1,
    "9763f489-6b2b-4a19-84a4-1fc602124890",
    "Liiga Smilshkalne",
);

// EOE 328 — Starfield Vocalist (alternate printing)
const STARFIELD_VOCALIST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STARFIELD_VOCALIST,
    1,
    "9831b27e-90f5-4799-8b87-644a91b853e6",
    "Nathaniel Himawan",
);

// EOE 329 — Synthesizer Labship (alternate printing)
const SYNTHESIZER_LABSHIP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SYNTHESIZER_LABSHIP,
    1,
    "85bbbbed-1dbb-4c85-b585-cde6713890ef",
    "Adrián Rodríguez Pérez",
);

// EOE 330 — Weftwalking (alternate printing)
const WEFTWALKING_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WEFTWALKING,
    1,
    "59a862ac-ef3b-41ff-abb7-d09879e9a7b1",
    "Rovina Cai",
);

// EOE 331 — Chorale of the Void (alternate printing)
const CHORALE_OF_THE_VOID_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &CHORALE_OF_THE_VOID,
    1,
    "6fda7329-0a1e-4921-bc6f-f2f4f1544878",
    "Alix Branwyn",
);

// EOE 332 — Entropic Battlecruiser (alternate printing)
const ENTROPIC_BATTLECRUISER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ENTROPIC_BATTLECRUISER,
    1,
    "c3cf23c5-cef9-4d78-9f09-a737e59b8d70",
    "Josiah \"Jo\" Cameron",
);

// EOE 333 — Requiem Monolith (alternate printing)
const REQUIEM_MONOLITH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &REQUIEM_MONOLITH,
    1,
    "2c44310c-80e2-4398-9b7a-097eca86ee48",
    "Warren Mahy",
);

// EOE 334 — Sunset Saboteur (alternate printing)
const SUNSET_SABOTEUR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SUNSET_SABOTEUR,
    1,
    "2e75cf29-a084-4ff7-ae58-eb267befa293",
    "Mirko Failoni",
);

// EOE 335 — Zero Point Ballad (alternate printing)
const ZERO_POINT_BALLAD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &ZERO_POINT_BALLAD,
    1,
    "c56ea5ca-f9b3-4ffa-9d2b-0190ab43cabc",
    "David Astruga",
);

// EOE 336 — Memorial Vault (alternate printing)
const MEMORIAL_VAULT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &MEMORIAL_VAULT,
    1,
    "6b4cf1fb-9a56-4d75-a42e-5a92bcfe5b08",
    "Javier Charro",
);

// EOE 337 — Pain for All (alternate printing)
const PAIN_FOR_ALL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PAIN_FOR_ALL,
    1,
    "2a9e9951-7ded-41de-808f-8a0fea964307",
    "Dmitry Burmak",
);

// EOE 338 — Terminal Velocity (alternate printing)
const TERMINAL_VELOCITY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &TERMINAL_VELOCITY,
    1,
    "d36147a8-dfcd-4d56-a98d-72fee52578d6",
    "Xabi Gaztelua",
);

// EOE 339 — Warmaker Gunship (alternate printing)
const WARMAKER_GUNSHIP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &WARMAKER_GUNSHIP,
    1,
    "515c0b63-c710-4419-a84d-4777d874bf19",
    "Julian Kok Joon Wen",
);

// EOE 340 — Bioengineered Future (alternate printing)
const BIOENGINEERED_FUTURE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BIOENGINEERED_FUTURE,
    1,
    "8064eb67-5397-4a25-a02e-f2a0a3e839aa",
    "Constantin Marin",
);

// EOE 341 — Famished Worldsire (alternate printing)
const FAMISHED_WORLDSIRE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FAMISHED_WORLDSIRE,
    1,
    "4253d6f1-f568-4419-8b5c-a2e8855057c7",
    "Kev Walker",
);

// EOE 342 — Frenzied Baloth (alternate printing)
const FRENZIED_BALOTH_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &FRENZIED_BALOTH,
    1,
    "dbd6cfbd-1cb7-4b15-9d03-f07b3a874e2d",
    "Diana Franco",
);

// EOE 343 — Icetill Explorer (alternate printing)
const ICETILL_EXPLORER_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ICETILL_EXPLORER,
    2,
    "f63de2e4-f094-4b10-b754-1285ad64effc",
    "Warren Mahy",
);

// EOE 344 — Loading Zone (alternate printing)
const LOADING_ZONE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &LOADING_ZONE,
    1,
    "d2111f0f-924e-4293-ba1d-15ae31088bb7",
    "Matt Stewart",
);

// EOE 345 — Ouroboroid (alternate printing)
const OUROBOROID_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &OUROBOROID,
    1,
    "9a6eb356-7418-443f-a51b-37ebc3526fce",
    "Samuel Perin",
);

// EOE 346 — Sledge-Class Seedship (alternate printing)
const SLEDGE_CLASS_SEEDSHIP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SLEDGE_CLASS_SEEDSHIP,
    1,
    "4d0c21a9-033c-42af-acbe-84284e0849f7",
    "Leon Tukker",
);

// EOE 347 — Biotech Specialist (alternate printing)
const BIOTECH_SPECIALIST_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &BIOTECH_SPECIALIST,
    1,
    "97dc831c-bd78-4c21-a1f3-24794db8eb5f",
    "Alexandre Honoré",
);

// EOE 348 — Infinite Guideline Station (alternate printing)
const INFINITE_GUIDELINE_STATION_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &INFINITE_GUIDELINE_STATION,
    1,
    "334654be-c433-43b6-bf33-8ba040c3c3a8",
    "Piotr Dura",
);

// EOE 349 — Pinnacle Emissary (alternate printing)
const PINNACLE_EMISSARY_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PINNACLE_EMISSARY,
    1,
    "2b755ebd-a194-4b78-b1e8-606a7d231598",
    "Alejandro Pacheco",
);

// EOE 350 — Singularity Rupture (alternate printing)
const SINGULARITY_RUPTURE_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SINGULARITY_RUPTURE,
    1,
    "e88877c1-0a77-4730-8bc3-5ca0dbdd4566",
    "Liiga Smilshkalne",
);

// EOE 351 — Dawnsire, Sunstar Dreadnought (alternate printing)
const DAWNSIRE_SUNSTAR_DREADNOUGHT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &DAWNSIRE_SUNSTAR_DREADNOUGHT,
    1,
    "8a242c45-07a7-48bf-8cd0-159fd9f5ecbc",
    "Jaime Jones",
);

// EOE 352 — The Dominion Bracelet (alternate printing)
const THE_DOMINION_BRACELET_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_DOMINION_BRACELET,
    1,
    "bb1492cf-b56f-40c7-a52b-7e354bcadc1d",
    "Nathaniel Himawan",
);

// EOE 353 — The Endstone (alternate printing)
const THE_ENDSTONE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THE_ENDSTONE,
    2,
    "62a08de0-27d6-466b-8ba8-a1bb7719049d",
    "Ryan Pancoast",
);

// EOE 354 — The Eternity Elevator (alternate printing)
const THE_ETERNITY_ELEVATOR_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THE_ETERNITY_ELEVATOR,
    1,
    "23d2c721-42a6-4108-91c2-93f59b645313",
    "Josu Solano",
);

// EOE 355 — Extinguisher Battleship (alternate printing)
const EXTINGUISHER_BATTLESHIP_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &EXTINGUISHER_BATTLESHIP,
    1,
    "5062b3a3-0ec6-4c1b-9394-e2f2e2301c43",
    "Danny Schwartz",
);

// EOE 356 — Thrumming Hivepool (alternate printing)
const THRUMMING_HIVEPOOL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THRUMMING_HIVEPOOL,
    1,
    "ef76c1af-394d-4ab6-b89f-d0c6009c8299",
    "Rob Rey",
);

// EOE 357 — Anticausal Vestige (alternate printing)
const ANTICAUSAL_VESTIGE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ANTICAUSAL_VESTIGE,
    2,
    "a8c94ccb-f6af-4a26-b773-624d6ce54330",
    "Nottsuo",
);

// EOE 358 — Exalted Sunborn (alternate printing)
const EXALTED_SUNBORN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &EXALTED_SUNBORN,
    2,
    "59183da2-8315-49f4-ab87-01b957b26366",
    "Makoron",
);

// EOE 359 — Starfield Vocalist (alternate printing)
const STARFIELD_VOCALIST_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &STARFIELD_VOCALIST,
    2,
    "eb6f8c8c-1905-4c3c-aa03-d56bce983559",
    "nina",
);

// EOE 360 — Sothera, the Supervoid (alternate printing)
const SOTHERA_THE_SUPERVOID_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &SOTHERA_THE_SUPERVOID,
    1,
    "80d0b4ed-116a-445a-a571-7ebfc1983654",
    "Mateusz Urbanowicz",
);

// EOE 361 — Devastating Onslaught (alternate printing)
const DEVASTATING_ONSLAUGHT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &DEVASTATING_ONSLAUGHT,
    2,
    "3d043992-251c-40cb-b1bc-481ba2830cfe",
    "Naochika Morishita",
);

// EOE 362 — Icetill Explorer
pub(in crate::card::sets) static ICETILL_EXPLORER: CardRecord = CardRecord::new(
    "Icetill Explorer",
    "895e5e9b-84dd-4741-8a2c-442165ea9b15",
    "Raimaru",
    // Four mana for a 2/4 whose three clauses feed each other: the extra
    // land drop wants lands, the mill finds them, and the graveyard is
    // where the mill puts them.
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Insect", "Scout"], 2, 4).with_abilities(&[
        AbilityDef::static_ability(
            "You may play an additional land on each of your turns.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MayPlayAdditionalLands(1)),
            },
        ),
        AbilityDef::static_ability(
            "You may play lands from your graveyard.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MayPlayFromGraveyard(
                    // Lands only, played the ordinary way: what the permission adds is the
                    // zone, not a way of casting anything out of it.
                    GraveyardPlayPermissionDef::unlimited(PlayRestrictionDef::new(
                        PlayActionMatcherDef::PlayLand,
                        ObjectPredicateDef::HasType(CardType::Land),
                    )),
                )),
            },
        ),
        AbilityDef::triggered(
            "Landfall — Whenever a land you control enters, mill a card.",
            TriggerEventDef::zone_changed(
                // A land you control arriving, which is what landfall is: a land somebody
                // else plays is not one, and the mill is what turns the extra land drop
                // into more lands to play.
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::Mill {
                player: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// EOE 363 — Mutinous Massacre (alternate printing)
const MUTINOUS_MASSACRE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &MUTINOUS_MASSACRE,
    2,
    "b446c5a9-0963-474c-a4d6-13da808dbbc1",
    "Aogachou",
);

// EOE 364 — The Dominion Bracelet (alternate printing)
const THE_DOMINION_BRACELET_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &THE_DOMINION_BRACELET,
    2,
    "2aea7e3a-7dc4-4770-a22e-8ab29f494844",
    "Mai Minamiura",
);

// EOE 365 — The Endstone (alternate printing)
const THE_ENDSTONE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &THE_ENDSTONE,
    3,
    "c9f2599c-9fec-4392-8228-03acfdee50e5",
    "Hidetaka Tenjin",
);

// EOE 366 — Secluded Starforge (alternate printing)
const SECLUDED_STARFORGE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SECLUDED_STARFORGE,
    2,
    "ec78b16c-be63-4aef-9308-f7a093f032a7",
    "Makoto Yukimura",
);

// EOE 367 — Plains (alternate printing)
const PLAINS_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::PLAINS,
    3,
    "926aad15-87a8-4510-b327-d1648f89c497",
    "Adam Paquette",
);

// EOE 368 — Island (alternate printing)
const ISLAND_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::ISLAND,
    3,
    "81368899-5fef-4f10-80ea-b282eca0f42f",
    "Adam Paquette",
);

// EOE 369 — Swamp (alternate printing)
const SWAMP_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::SWAMP,
    3,
    "b938ac10-bd0f-4ce3-a743-958d5beadf58",
    "Adam Paquette",
);

// EOE 370 — Mountain (alternate printing)
const MOUNTAIN_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::MOUNTAIN,
    3,
    "6b092822-f34f-4384-9d0a-23d863d27231",
    "Adam Paquette",
);

// EOE 371 — Forest (alternate printing)
const FOREST_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &catalog_lea::FOREST,
    3,
    "0d6250d3-728b-4412-8efc-911bb6f5e910",
    "Adam Paquette",
);

// EOE 372 — Adagia, Windswept Bastion (alternate printing)
const ADAGIA_WINDSWEPT_BASTION_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &ADAGIA_WINDSWEPT_BASTION,
    2,
    "bf314021-ba15-46c2-99e0-94141f118bc9",
    "Piotr Dura",
);

// EOE 373 — Breeding Pool (alternate printing)
const BREEDING_POOL_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_dis::BREEDING_POOL,
    2,
    "fda15a78-a370-4cc6-a4df-92200d6ca826",
    "Chris Ostrowski",
);

// EOE 374 — Evendo, Waking Haven (alternate printing)
const EVENDO_WAKING_HAVEN_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &EVENDO_WAKING_HAVEN,
    2,
    "064255cc-445f-4794-aa64-3aee34e11181",
    "Piotr Dura",
);

// EOE 375 — Godless Shrine (alternate printing)
const GODLESS_SHRINE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_gpt::GODLESS_SHRINE,
    2,
    "da9708aa-c4db-4ed5-bcf2-29ea15af7d8b",
    "Chris Ostrowski",
);

// EOE 376 — Kavaron, Memorial World (alternate printing)
const KAVARON_MEMORIAL_WORLD_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &KAVARON_MEMORIAL_WORLD,
    2,
    "4d5a29a4-98f6-4de9-b685-120e26c8b785",
    "Piotr Dura",
);

// EOE 377 — Sacred Foundry (alternate printing)
const SACRED_FOUNDRY_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_rav::SACRED_FOUNDRY,
    2,
    "15e9f2fe-7703-42bf-a6f7-c4db6e62621f",
    "Chris Ostrowski",
);

// EOE 378 — Stomping Ground (alternate printing)
const STOMPING_GROUND_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_gpt::STOMPING_GROUND,
    2,
    "3ba7e782-3c6a-4f3b-b00d-cf982ad31376",
    "Chris Ostrowski",
);

// EOE 379 — Susur Secundi, Void Altar (alternate printing)
const SUSUR_SECUNDI_VOID_ALTAR_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SUSUR_SECUNDI_VOID_ALTAR,
    2,
    "dd59063a-8ad7-4efa-8f40-7b3c9ccc68f1",
    "Piotr Dura",
);

// EOE 380 — Uthros, Titanic Godcore (alternate printing)
const UTHROS_TITANIC_GODCORE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &UTHROS_TITANIC_GODCORE,
    2,
    "634bccd9-ab66-4620-a7b1-b9c984641558",
    "Piotr Dura",
);

// EOE 381 — Watery Grave (alternate printing)
const WATERY_GRAVE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &catalog_rav::WATERY_GRAVE,
    2,
    "d854480d-a163-422b-a3dc-54dffd7b3eab",
    "Chris Ostrowski",
);

// EOE 382 — Sothera, the Supervoid (alternate printing)
const SOTHERA_THE_SUPERVOID_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SOTHERA_THE_SUPERVOID,
    2,
    "07b10f1b-b03b-4f19-bc7e-69d5eaa5ff07",
    "Micha Huigen",
);

// EOE 383 — Anticausal Vestige (alternate printing)
const ANTICAUSAL_VESTIGE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &ANTICAUSAL_VESTIGE,
    3,
    "c0a00a77-1f7b-4528-aaf1-b2d78e73a604",
    "Nottsuo",
);

// EOE 384 — Exalted Sunborn (alternate printing)
const EXALTED_SUNBORN_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &EXALTED_SUNBORN,
    3,
    "a827ede4-eec6-4a9d-b491-cddfab6aa39b",
    "Makoron",
);

// EOE 385 — Starfield Vocalist (alternate printing)
const STARFIELD_VOCALIST_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &STARFIELD_VOCALIST,
    3,
    "d8a6759d-e858-41d0-a5e3-a2ac1d074263",
    "nina",
);

// EOE 386 — Sothera, the Supervoid (alternate printing)
const SOTHERA_THE_SUPERVOID_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &SOTHERA_THE_SUPERVOID,
    3,
    "668b1796-2722-4d69-9a41-8067ad189ad6",
    "Mateusz Urbanowicz",
);

// EOE 387 — Devastating Onslaught (alternate printing)
const DEVASTATING_ONSLAUGHT_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &DEVASTATING_ONSLAUGHT,
    3,
    "2dedcdcd-4dbf-4bc1-9de3-823f0c7e3961",
    "Naochika Morishita",
);

// EOE 388 — Icetill Explorer (alternate printing)
const ICETILL_EXPLORER_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &ICETILL_EXPLORER,
    3,
    "b2ace91b-0329-4511-bf45-cb72e0ebeae0",
    "Raimaru",
);

// EOE 389 — Mutinous Massacre (alternate printing)
const MUTINOUS_MASSACRE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &MUTINOUS_MASSACRE,
    3,
    "8f319e32-56e2-402e-8e99-3cf2953ae19d",
    "Aogachou",
);

// EOE 390 — The Dominion Bracelet (alternate printing)
const THE_DOMINION_BRACELET_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &THE_DOMINION_BRACELET,
    3,
    "dac293bd-0724-40e9-856e-a6a0483565ca",
    "Mai Minamiura",
);

// EOE 391 — The Endstone
pub(in crate::card::sets) static THE_ENDSTONE: CardRecord = CardRecord::new(
    "The Endstone",
    "1227eb7f-c2a5-4112-98d0-70275a63c26a",
    "Hidetaka Tenjin",
// Seven mana that draws a card for everything you do and hands the ten
    // life back every end step, which is what makes the seven payable.
    CardRules::new_artifact(mana_cost!("{7}"))
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever you play a land or cast a spell, draw a card.",
                // One ability with two events rather than two abilities: the card prints
                // one, and a turn with a land and a spell in it draws twice either way.
                TriggerEventDef::AnyOf(&[
                    TriggerEventDef::LandPlayed {
                        land: ObjectPredicateDef::Any,
                        player: PlayerRelation::You,
                    },
                    TriggerEventDef::spell_cast(ObjectPredicateDef::ControlledBy(PlayerRelation::You)),
                ]),
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::triggered(
                "At the beginning of your end step, your life total becomes half your starting life \
                 total, rounded up.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::End,
                    player: PlayerRelation::You,
                },
                EffectDef::SetLifeTotal {
                    recipient: EffectRecipientDef::Controller,
                    // Half of what the game began on rather than half of what is left: it sets
                    // the total to the same number every end step, which is a gain from below
                    // it and a loss from above.
                    total: ValueDef::Halved(&HalvedValueDef::new(
                        ValueDef::StartingLifeTotal,
                        RoundingDef::Up,
                    )),
                },
            ),
        ]),
);

// EOE 392 — Secluded Starforge (alternate printing)
const SECLUDED_STARFORGE_ALTERNATE_3: PrintingRecord = PrintingRecord::alternate(
    &SECLUDED_STARFORGE,
    3,
    "492a1bc6-6f4b-400c-959f-1bfef03a68c8",
    "Makoto Yukimura",
);

// EOE 393 — Starfield Shepherd (alternate printing)
const STARFIELD_SHEPHERD_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &STARFIELD_SHEPHERD,
    1,
    "67dfd9e5-bd2e-4140-91c0-da33dc7f46e4",
    "Marta Nael",
);

// EOE 394 — Annul (alternate printing)
const ANNUL_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &catalog_usg::ANNUL,
    1,
    "26dd02c4-569b-437f-b9dd-bd2f1d86a968",
    "Carlos Palma Cruchaga",
);

// EOE 395 — Umbral Collar Zealot (alternate printing)
const UMBRAL_COLLAR_ZEALOT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &UMBRAL_COLLAR_ZEALOT,
    1,
    "9b14b659-9b4d-4099-90a0-d2a673b86648",
    "Dmitry Burmak",
);

// EOE 396 — Kavaron Harrier (alternate printing)
const KAVARON_HARRIER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &KAVARON_HARRIER,
    1,
    "57264a8e-764a-46e6-afea-747f1b3dbf23",
    "Hardy Fowler",
);

// EOE 397 — Pull Through the Weft (alternate printing)
const PULL_THROUGH_THE_WEFT_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &PULL_THROUGH_THE_WEFT,
    1,
    "07404acc-4f25-45e6-8174-683f296daa8a",
    "Andrew Mar",
);

// EOE 398 — Singularity Rupture (alternate printing)
const SINGULARITY_RUPTURE_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &SINGULARITY_RUPTURE,
    2,
    "422d7bc1-e2a2-4cfc-a29d-9d67354cdafd",
    "Néstor Ossandón Leal",
);

// EOE 399 — Emissary Escort (alternate printing)
const EMISSARY_ESCORT_ALTERNATE_2: PrintingRecord = PrintingRecord::alternate(
    &EMISSARY_ESCORT,
    2,
    "f32df6a8-d77f-40ce-bcac-9495db458210",
    "Lius Lasahido",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ANTICAUSAL_VESTIGE,
    &TEZZERET_CRUEL_CAPTAIN,
    &ALL_FATES_STALKER,
    &ASTELLI_RECLAIMER,
    &AUXILIARY_BOOSTERS,
    &BEYOND_THE_QUIET,
    &BRIGHTSPEAR_ZEALOT,
    &COSMOGRAND_ZENITH,
    &DAWNSTRIKE_VANGUARD,
    &DOCKWORKER_DRONE,
    &DUAL_SUN_ADEPTS,
    &DUAL_SUN_TECHNIQUE,
    &EMERGENCY_EJECT,
    &EXALTED_SUNBORN,
    &EXOSUIT_SAVIOR,
    &FLIGHT_DECK_COORDINATOR,
    &FOCUS_FIRE,
    &HALIYA_GUIDED_BY_LIGHT,
    &HARDLIGHT_CONTAINMENT,
    &HONOR,
    &HONORED_KNIGHT_CAPTAIN,
    &KNIGHT_LUMINARY,
    &LIGHTSTALL_INQUISITOR,
    &LUMEN_CLASS_FRIGATE,
    &LUXKNIGHT_BREACHER,
    &PINNACLE_STARCAGE,
    &PULSAR_SQUADRON_ACE,
    &RADIANT_STRIKE,
    &RAYBLADE_TROOPER,
    &REROUTE_SYSTEMS,
    &RESCUE_SKIFF,
    &SCOUT_FOR_SURVIVORS,
    &SEAM_RIP,
    &THE_SERIEMA,
    &SQUIRE_S_LIGHTBLADE,
    &STARFIELD_SHEPHERD,
    &STARFIGHTER_PILOT,
    &STARPORT_SECURITY,
    &SUNSTAR_CHAPLAIN,
    &SUNSTAR_EXPANSIONIST,
    &SUNSTAR_LIGHTSMITH,
    &WEDGELIGHT_RAMMER,
    &WEFTBLADE_ENHANCER,
    &ZEALOUS_DISPLAY,
    &ATOMIC_MICROSIZER,
    &CEREBRAL_DOWNLOAD,
    &CLOUDSCULPT_TECHNICIAN,
    &CODECRACKER_HOUND,
    &CONSULT_THE_STAR_CHARTS,
    &CRYOGEN_RELIC,
    &CRYOSHATTER,
    &DESCULPTING_BLAST,
    &DIVERT_DISASTER,
    &EMISSARY_ESCORT,
    &GIGASTORM_TITAN,
    &ILLVOI_GALEBLADE,
    &ILLVOI_INFILTRATOR,
    &ILLVOI_LIGHT_JAMMER,
    &ILLVOI_OPERATIVE,
    &LOST_IN_SPACE,
    &MECHAN_ASSEMBLER,
    &MECHAN_NAVIGATOR,
    &MECHAN_SHIELDMATE,
    &MECHANOZOA,
    &MENTAL_MODULATION,
    &MM_MENON_THE_RIGHT_HAND,
    &MOONLIT_MEDITATION,
    &MOUTH_OF_THE_STORM,
    &NANOFORM_SENTINEL,
    &QUANTUM_RIDDLER,
    &SCOUR_FOR_SCRAP,
    &SELFCRAFT_MECHAN,
    &SINISTER_CRYOLOGIST,
    &SPECIMEN_FREIGHTER,
    &STARBREACH_WHALE,
    &STARFIELD_VOCALIST,
    &STARWINDER,
    &STEELSWARM_OPERATOR,
    &SYNTHESIZER_LABSHIP,
    &TRACTOR_BEAM,
    &UNRAVEL,
    &UTHROS_PSIONICIST,
    &UTHROS_SCANSHIP,
    &WEFTWALKING,
    &ALPHARAEL_STONECHOSEN,
    &ARCHENEMY_S_CHARM,
    &BEAMSAW_PROSPECTOR,
    &BLADE_OF_THE_SWARM,
    &CHORALE_OF_THE_VOID,
    &COMET_CRAWLER,
    &DARK_ENDURANCE,
    &DECODE_TRANSMISSIONS,
    &DEPRESSURIZE,
    &DUBIOUS_DELICACY,
    &ELEGY_ACOLYTE,
    &EMBRACE_OBLIVION,
    &ENTROPIC_BATTLECRUISER,
    &FALLER_S_FAITHFUL,
    &FELL_GRAVSHIP,
    &GRAVBLADE_HEAVY,
    &GRAVKILL,
    &GRAVPACK_MONOIST,
    &HULLCARVER,
    &HYLDERBLADE,
    &HYMN_OF_THE_FALLER,
    &INSATIABLE_SKITTERMAW,
    &LIGHTLESS_EVANGEL,
    &MONOIST_CIRCUIT_FEEDER,
    &MONOIST_SENTRY,
    &PERIGEE_BECKONER,
    &REQUIEM_MONOLITH,
    &SCROUNGE_FOR_ETERNITY,
    &SOTHERA_THE_SUPERVOID,
    &SUNSET_SABOTEUR,
    &SUSURIAN_DIRGECRAFT,
    &SUSURIAN_VOIDBORN,
    &SWARM_CULLER,
    &TEMPORAL_INTERVENTION,
    &TIMELINE_CULLER,
    &TRAGIC_TRAJECTORY,
    &UMBRAL_COLLAR_ZEALOT,
    &VOIDFORGED_TITAN,
    &VOTE_OUT,
    &XU_IFIT_OSTEOHARMONIST,
    &ZERO_POINT_BALLAD,
    &CUT_PROPULSION,
    &DEBRIS_FIELD_CRUSHER,
    &DEVASTATING_ONSLAUGHT,
    &DRILL_TOO_DEEP,
    &FRONTLINE_WAR_RAGER,
    &FULL_BORE,
    &GALVANIZING_SAWSHIP,
    &INVASIVE_MANEUVERS,
    &KAV_LANDSEEKER,
    &KAVARON_HARRIER,
    &KAVARON_SKYWARDEN,
    &KAVARON_TURBODRONE,
    &LITHOBRAKING,
    &MELDED_MOXITE,
    &MEMORIAL_TEAM_LEADER,
    &MEMORIAL_VAULT,
    &MOLECULAR_MODIFIER,
    &NEBULA_DRAGON,
    &NOVA_HELLKITE,
    &ORBITAL_PLUNGE,
    &OREPLATE_PANGOLIN,
    &PAIN_FOR_ALL,
    &PLASMA_BOLT,
    &POSSIBILITY_TECHNICIAN,
    &RED_TIGER_MECHAN,
    &REMNANT_ELEMENTAL,
    &RIG_FOR_WAR,
    &ROVING_ACTUATOR,
    &RUINOUS_RAMPAGE,
    &RUST_HARVESTER,
    &SLAGDRILL_SCRAPPER,
    &SYSTEMS_OVERRIDE,
    &TANNUK_STEADFAST_SECOND,
    &TERMINAL_VELOCITY,
    &TERRAPACT_INTIMIDATOR,
    &TERRITORIAL_BRUNTAR,
    &VAULTGUARD_TROOPER,
    &WARMAKER_GUNSHIP,
    &WEAPONS_MANUFACTURING,
    &WEFTSTALKER_ARDENT,
    &ZOOKEEPER_MECHAN,
    &ATMOSPHERIC_GREENHOUSE,
    &BIOENGINEERED_FUTURE,
    &BIOSYNTHIC_BURST,
    &BLOOMING_STINGER,
    &BROODGUARD_ELITE,
    &CLOSE_ENCOUNTER,
    &DIPLOMATIC_RELATIONS,
    &DRIX_FATEMAKER,
    &EDGE_ROVER,
    &EUMIDIAN_TERRABOTANIST,
    &EUSOCIAL_ENGINEERING,
    &FAMISHED_WORLDSIRE,
    &FRENZIED_BALOTH,
    &FUNGAL_COLOSSUS,
    &GALACTIC_WAYFARER,
    &GENE_POLLINATOR,
    &GERMINATING_WURM,
    &GLACIER_GODMAW,
    &HARMONIOUS_GROVESTRIDER,
    &HEMOSYMBIC_MITE,
    &ICECAVE_CRASHER,
    &INTREPID_TENDERFOOT,
    &LARVAL_SCOUTLANDER,
    &LASHWHIP_PREDATOR,
    &LOADING_ZONE,
    &MELTSTRIDER_EULOGIST,
    &MELTSTRIDER_S_GEAR,
    &MELTSTRIDER_S_RESOLVE,
    &OUROBOROID,
    &PULL_THROUGH_THE_WEFT,
    &SAMI_S_CURIOSITY,
    &SEEDSHIP_AGRARIAN,
    &SEEDSHIP_IMPACT,
    &SHATTERED_WINGS,
    &SKYSTINGER,
    &SLEDGE_CLASS_SEEDSHIP,
    &TAPESTRY_WARDEN,
    &TERRASYMBIOSIS,
    &THAWBRINGER,
    &ALPHARAEL_DREAMING_ACOLYTE,
    &BIOMECHAN_ENGINEER,
    &BIOTECH_SPECIALIST,
    &COSMOGOYF,
    &DYADRINE_SYNTHESIS_AMALGAM,
    &GENEMORPH_IMAGO,
    &HALIYA_ASCENDANT_CADET,
    &INFINITE_GUIDELINE_STATION,
    &INTERCEPTOR_MECHAN,
    &MM_MENON_UTHROS_EXILE,
    &MUTINOUS_MASSACRE,
    &PINNACLE_EMISSARY,
    &RAGOST_DEFT_GASTRONAUT,
    &SAMI_SHIP_S_ENGINEER,
    &SAMI_WILDCAT_CAPTAIN,
    &SEEDSHIP_BROODTENDER,
    &SINGULARITY_RUPTURE,
    &SPACE_TIME_ANOMALY,
    &STATION_MONITOR,
    &SYR_VONDAM_SUNSTAR_EXEMPLAR,
    &SYR_VONDAM_THE_LUCENT,
    &TANNUK_MEMORIAL_ENSIGN,
    &ALL_FATES_SCROLL,
    &BYGONE_COLOSSUS,
    &CHROME_COMPANION,
    &DAUNTLESS_SCRAPBOT,
    &DAWNSIRE_SUNSTAR_DREADNOUGHT,
    &THE_DOMINION_BRACELET,
    &THE_ETERNITY_ELEVATOR,
    &EXTINGUISHER_BATTLESHIP,
    &NUTRIENT_BLOCK,
    &PINNACLE_KILL_SHIP,
    &SURVEY_MECHAN,
    &THAUMATON_TORPEDO,
    &THRUMMING_HIVEPOOL,
    &VIRULENT_SILENCER,
    &WURMWALL_SWEEPER,
    &ADAGIA_WINDSWEPT_BASTION,
    &COMMAND_BRIDGE,
    &EVENDO_WAKING_HAVEN,
    &KAVARON_MEMORIAL_WORLD,
    &SECLUDED_STARFORGE,
    &SUSUR_SECUNDI_VOID_ALTAR,
    &UTHROS_TITANIC_GODCORE,
    &MIGHTFORM_HARMONIZER,
    &ICETILL_EXPLORER,
    &THE_ENDSTONE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    BANISHING_LIGHT_REPRINT,
    ANNUL_REPRINT,
    VIRUS_BEETLE_REPRINT,
    BOMBARD_REPRINT,
    DIPLOMATIC_RELATIONS_ALTERNATE_1,
    ICETILL_EXPLORER_ALTERNATE_1,
    MIGHTFORM_HARMONIZER_ALTERNATE_1,
    THE_ENDSTONE_ALTERNATE_1,
    BREEDING_POOL_REPRINT,
    GODLESS_SHRINE_REPRINT,
    SACRED_FOUNDRY_REPRINT,
    STOMPING_GROUND_REPRINT,
    WATERY_GRAVE_REPRINT,
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
    ADAGIA_WINDSWEPT_BASTION_ALTERNATE_1,
    BREEDING_POOL_ALTERNATE_1,
    EVENDO_WAKING_HAVEN_ALTERNATE_1,
    GODLESS_SHRINE_ALTERNATE_1,
    KAVARON_MEMORIAL_WORLD_ALTERNATE_1,
    SACRED_FOUNDRY_ALTERNATE_1,
    STOMPING_GROUND_ALTERNATE_1,
    SUSUR_SECUNDI_VOID_ALTAR_ALTERNATE_1,
    UTHROS_TITANIC_GODCORE_ALTERNATE_1,
    WATERY_GRAVE_ALTERNATE_1,
    TEZZERET_CRUEL_CAPTAIN_ALTERNATE_1,
    ASTELLI_RECLAIMER_ALTERNATE_1,
    HALIYA_GUIDED_BY_LIGHT_ALTERNATE_1,
    MM_MENON_THE_RIGHT_HAND_ALTERNATE_1,
    STARWINDER_ALTERNATE_1,
    ALPHARAEL_STONECHOSEN_ALTERNATE_1,
    ELEGY_ACOLYTE_ALTERNATE_1,
    XU_IFIT_OSTEOHARMONIST_ALTERNATE_1,
    POSSIBILITY_TECHNICIAN_ALTERNATE_1,
    TANNUK_STEADFAST_SECOND_ALTERNATE_1,
    DYADRINE_SYNTHESIS_AMALGAM_ALTERNATE_1,
    GENEMORPH_IMAGO_ALTERNATE_1,
    RAGOST_DEFT_GASTRONAUT_ALTERNATE_1,
    SAMI_WILDCAT_CAPTAIN_ALTERNATE_1,
    SYR_VONDAM_SUNSTAR_EXEMPLAR_ALTERNATE_1,
    BEYOND_THE_QUIET_ALTERNATE_1,
    COSMOGRAND_ZENITH_ALTERNATE_1,
    QUANTUM_RIDDLER_ALTERNATE_1,
    STARWINDER_ALTERNATE_2,
    ARCHENEMY_S_CHARM_ALTERNATE_1,
    DEVASTATING_ONSLAUGHT_ALTERNATE_1,
    NOVA_HELLKITE_ALTERNATE_1,
    RUST_HARVESTER_ALTERNATE_1,
    WEAPONS_MANUFACTURING_ALTERNATE_1,
    TERRASYMBIOSIS_ALTERNATE_1,
    COSMOGOYF_ALTERNATE_1,
    MUTINOUS_MASSACRE_ALTERNATE_1,
    SPACE_TIME_ANOMALY_ALTERNATE_1,
    SECLUDED_STARFORGE_ALTERNATE_1,
    ANTICAUSAL_VESTIGE_ALTERNATE_1,
    EXALTED_SUNBORN_ALTERNATE_1,
    HARDLIGHT_CONTAINMENT_ALTERNATE_1,
    LIGHTSTALL_INQUISITOR_ALTERNATE_1,
    LUMEN_CLASS_FRIGATE_ALTERNATE_1,
    PINNACLE_STARCAGE_ALTERNATE_1,
    THE_SERIEMA_ALTERNATE_1,
    SUNSTAR_CHAPLAIN_ALTERNATE_1,
    CONSULT_THE_STAR_CHARTS_ALTERNATE_1,
    EMISSARY_ESCORT_ALTERNATE_1,
    MOONLIT_MEDITATION_ALTERNATE_1,
    STARFIELD_VOCALIST_ALTERNATE_1,
    SYNTHESIZER_LABSHIP_ALTERNATE_1,
    WEFTWALKING_ALTERNATE_1,
    CHORALE_OF_THE_VOID_ALTERNATE_1,
    ENTROPIC_BATTLECRUISER_ALTERNATE_1,
    REQUIEM_MONOLITH_ALTERNATE_1,
    SUNSET_SABOTEUR_ALTERNATE_1,
    ZERO_POINT_BALLAD_ALTERNATE_1,
    MEMORIAL_VAULT_ALTERNATE_1,
    PAIN_FOR_ALL_ALTERNATE_1,
    TERMINAL_VELOCITY_ALTERNATE_1,
    WARMAKER_GUNSHIP_ALTERNATE_1,
    BIOENGINEERED_FUTURE_ALTERNATE_1,
    FAMISHED_WORLDSIRE_ALTERNATE_1,
    FRENZIED_BALOTH_ALTERNATE_1,
    ICETILL_EXPLORER_ALTERNATE_2,
    LOADING_ZONE_ALTERNATE_1,
    OUROBOROID_ALTERNATE_1,
    SLEDGE_CLASS_SEEDSHIP_ALTERNATE_1,
    BIOTECH_SPECIALIST_ALTERNATE_1,
    INFINITE_GUIDELINE_STATION_ALTERNATE_1,
    PINNACLE_EMISSARY_ALTERNATE_1,
    SINGULARITY_RUPTURE_ALTERNATE_1,
    DAWNSIRE_SUNSTAR_DREADNOUGHT_ALTERNATE_1,
    THE_DOMINION_BRACELET_ALTERNATE_1,
    THE_ENDSTONE_ALTERNATE_2,
    THE_ETERNITY_ELEVATOR_ALTERNATE_1,
    EXTINGUISHER_BATTLESHIP_ALTERNATE_1,
    THRUMMING_HIVEPOOL_ALTERNATE_1,
    ANTICAUSAL_VESTIGE_ALTERNATE_2,
    EXALTED_SUNBORN_ALTERNATE_2,
    STARFIELD_VOCALIST_ALTERNATE_2,
    SOTHERA_THE_SUPERVOID_ALTERNATE_1,
    DEVASTATING_ONSLAUGHT_ALTERNATE_2,
    MUTINOUS_MASSACRE_ALTERNATE_2,
    THE_DOMINION_BRACELET_ALTERNATE_2,
    THE_ENDSTONE_ALTERNATE_3,
    SECLUDED_STARFORGE_ALTERNATE_2,
    PLAINS_ALTERNATE_3,
    ISLAND_ALTERNATE_3,
    SWAMP_ALTERNATE_3,
    MOUNTAIN_ALTERNATE_3,
    FOREST_ALTERNATE_3,
    ADAGIA_WINDSWEPT_BASTION_ALTERNATE_2,
    BREEDING_POOL_ALTERNATE_2,
    EVENDO_WAKING_HAVEN_ALTERNATE_2,
    GODLESS_SHRINE_ALTERNATE_2,
    KAVARON_MEMORIAL_WORLD_ALTERNATE_2,
    SACRED_FOUNDRY_ALTERNATE_2,
    STOMPING_GROUND_ALTERNATE_2,
    SUSUR_SECUNDI_VOID_ALTAR_ALTERNATE_2,
    UTHROS_TITANIC_GODCORE_ALTERNATE_2,
    WATERY_GRAVE_ALTERNATE_2,
    SOTHERA_THE_SUPERVOID_ALTERNATE_2,
    ANTICAUSAL_VESTIGE_ALTERNATE_3,
    EXALTED_SUNBORN_ALTERNATE_3,
    STARFIELD_VOCALIST_ALTERNATE_3,
    SOTHERA_THE_SUPERVOID_ALTERNATE_3,
    DEVASTATING_ONSLAUGHT_ALTERNATE_3,
    ICETILL_EXPLORER_ALTERNATE_3,
    MUTINOUS_MASSACRE_ALTERNATE_3,
    THE_DOMINION_BRACELET_ALTERNATE_3,
    SECLUDED_STARFORGE_ALTERNATE_3,
    STARFIELD_SHEPHERD_ALTERNATE_1,
    ANNUL_ALTERNATE_1,
    UMBRAL_COLLAR_ZEALOT_ALTERNATE_1,
    KAVARON_HARRIER_ALTERNATE_1,
    PULL_THROUGH_THE_WEFT_ALTERNATE_1,
    SINGULARITY_RUPTURE_ALTERNATE_2,
    EMISSARY_ESCORT_ALTERNATE_2,
];
