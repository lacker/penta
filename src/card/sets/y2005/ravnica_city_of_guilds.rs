//! Ravnica: City of Guilds cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::BasicLandType;
use crate::CardType;
use crate::ParentBinding;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityKindDef;
use crate::card::AbilityPredicateDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::ActivationTimingDef;
use crate::card::AddManaEffectDef;
use crate::card::AggregateOperationDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::BlockRestrictionDef;
use crate::card::CardRules;
use crate::card::CostDef;
use crate::card::DiscardSelectionDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::MoveObjectsDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectSetDef;
use crate::card::ObjectValueAggregateDef;
use crate::card::ObjectValueDef;
use crate::card::PayOrDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::ResolvedEffectDurationDef;
use crate::card::RevealObjectsDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "RAV",
    slug: "ravnica-city-of-guilds",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// RAV 16 — Faith's Fetters
pub(in crate::card::sets) static FAITH_S_FETTERS: CardRecord = CardRecord::new(
    "Faith's Fetters",
    "5b8ffba3-44a9-41ce-a5a1-37413346db2f",
    "Chippy",
    // Four mana and four life for an answer that reaches anything, which is
    // what a slow deck pays for not having to guess what it will face.
    CardRules::new_enchantment(mana_cost!("{3}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell(
                "Enchant permanent",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::Any,
                )],
            ),
            abilities::enters_trigger(
                "When this Aura enters, you gain 4 life.",
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(4),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted permanent can't attack or block, and its activated abilities can't be \
                 activated unless they're mana abilities.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    // The mana exception is why this is NonManaActivated
                    // rather than Any: a land or rock it lands on still taps
                    // for mana.
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_ATTACK),
                        AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                        AppliedEffectDef::cannot_activate_abilities(AbilityPredicateDef::Is(
                            AbilityKindDef::NonManaActivated,
                        )),
                    ]),
                },
            ),
        ]),
);

// RAV 38 — Belltower Sphinx
pub(in crate::card::sets) static BELLTOWER_SPHINX: CardRecord = CardRecord::new(
    "Belltower Sphinx",
    "452a23a0-62de-4561-b361-9c0de9151129",
    "Jim Nelson",
CardRules::new_creature(mana_cost!("{4}{U}"), &["Sphinx"], 2, 5).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever a source deals damage to this creature, that source's controller mills that many cards.",
            TriggerEventDef::damage_to_source(),
            EffectDef::Mill {
                player: EffectRecipientDef::ControllerOfTriggeringObject,
                amount: ValueDef::TriggerEventAmount,
            },
        ),
    ]),
);

// RAV 42 — Copy Enchantment
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COPY_ENCHANTMENT_42: CardRecord = CardRecord::new(
    "Copy Enchantment",
    "ac22117d-bd58-439f-b199-da72bc7160b2",
    "Joel Thomas",
    crate::card::CardRules::unsupported(),
);

// RAV 43 — Dizzy Spell
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIZZY_SPELL_43: CardRecord = CardRecord::new(
    "Dizzy Spell",
    "6e0db10d-fb6d-44df-9ff2-6f1e0e8f8209",
    "Christopher Moeller",
    crate::card::CardRules::unsupported(),
);

// RAV 46 — Drift of Phantasms
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DRIFT_OF_PHANTASMS_46: CardRecord = CardRecord::new(
    "Drift of Phantasms",
    "c1096ce5-f776-4028-b231-e6eaee35014b",
    "Michael Phillippi",
    crate::card::CardRules::unsupported(),
);

// RAV 60 — Muddle the Mixture
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MUDDLE_THE_MIXTURE_60: CardRecord = CardRecord::new(
    "Muddle the Mixture",
    "4cc785b0-0a77-4b02-b0b4-2bda2fc621cc",
    "Luca Zontini",
    crate::card::CardRules::unsupported(),
);

// RAV 61 — Peel from Reality
pub(in crate::card::sets) static PEEL_FROM_REALITY: CardRecord = CardRecord::new(
    "Peel from Reality",
    "e4e6ca71-ba17-4a16-a331-b787363874e2",
    "Puddnhead",
CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(
        AbilityDef::spell_with_targets(
            "Return target creature you control and target creature you don't control to their owners' hands.",
            &[
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                }),
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                }),
            ],
            EffectDef::Sequence(&[
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
),
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex(1)),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
),
            ]),
        ),
    ),
);

// RAV 63 — Remand
pub(in crate::card::sets) static REMAND: CardRecord = CardRecord::new(
    "Remand",
    "581f3780-c480-48c6-b15c-1618f2feccb9",
    "Mark A. Nelson",
    // Two mana to buy a turn and replace itself. What it answers comes back,
    // so this is tempo rather than an answer.
    CardRules::new_instant(mana_cost!("{1}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target spell. If that spell is countered this way, put it into its owner's hand \
         instead of into that player's graveyard.\nDraw a card.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Spell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        // The countered card goes to its owner's hand rather than their graveyard,
        // which the counter effect's own destination says. The draw is a second
        // clause and happens whether or not the counter found anything to do.
        EffectDef::Sequence(&[
            EffectDef::Counter {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// RAV 74 — Vedalken Entrancer
pub(in crate::card::sets) static VEDALKEN_ENTRANCER: CardRecord = CardRecord::new(
    "Vedalken Entrancer",
    "faf5e4b8-3bb9-4a4c-b8fa-2cae5372ba24",
    "Dan Murayama Scott",
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Vedalken", "Wizard"], 1, 4).with_ability(
        AbilityDef::activated_with_targets(
            "{U}, {T}: Target player mills two cards.",
            &[CostDef::Mana(mana_cost!("{U}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Mill {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
        ),
    ),
);

// RAV 81 — Dark Confidant
/// One card off the top, shown to everybody, into your hand. Nothing is
/// chosen and nothing may be declined: the minimum and the maximum are both
/// the one card the trigger names.
pub(in crate::card::sets) static DARK_CONFIDANT: CardRecord = CardRecord::new(
    "Dark Confidant",
    "94f7a441-bf2d-46fb-a7b6-9bd6137f86d9",
    "Ron Spears",
    // Two mana for an extra card every turn, at whatever the top of your
    // deck happens to cost -- which is why the decks that play him keep
    // their curve low enough to survive him.
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Wizard"], 2, 1).with_ability(
        AbilityDef::triggered(
            "At the beginning of your upkeep, reveal the top card of your library and put that \
             card into your hand. You lose life equal to its mana value.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
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
                        // "You lose life equal to its mana value." The card is in your hand by the
                        // time this is asked, so what the reveal hands on is the number rather than
                        // the card.
                        then: &EffectDef::LoseLife {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::AggregateObjectValues(&ObjectValueAggregateDef {
                                objects: ObjectSetDef::Binding(ParentBinding),
                                select: ObjectValueDef::ManaValue,
                                operation: AggregateOperationDef::Maximum,
                            }),
                        },
                    }),
                ]),
            ),
        ),
    ),
);

// RAV 83 — Dimir House Guard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIMIR_HOUSE_GUARD_83: CardRecord = CardRecord::new(
    "Dimir House Guard",
    "9a021caf-d9e7-470b-85be-3af42a3adfd3",
    "John Zeleznik",
    crate::card::CardRules::unsupported(),
);

// RAV 84 — Dimir Machinations
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIMIR_MACHINATIONS_84: CardRecord = CardRecord::new(
    "Dimir Machinations",
    "14bfd72a-78c1-4167-89bf-ea1fccccd5b1",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// RAV 93 — Last Gasp
pub(in crate::card::sets) static LAST_GASP: CardRecord = CardRecord::new(
    "Last Gasp",
    "34e035b3-bd83-43a4-8f31-d2393d29cd94",
    "Thomas M. Baxa",
    CardRules::new_instant(mana_cost!("{1}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets -3/-3 until end of turn.",
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
    )]),
);

// RAV 116 — Breath of Fury
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BREATH_OF_FURY_116: CardRecord = CardRecord::new(
    "Breath of Fury",
    "dbef6f4a-f9a0-4a4c-b8a2-6c3a8fb7e14a",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// RAV 125 — Frenzied Goblin
pub(in crate::card::sets) static FRENZIED_GOBLIN: CardRecord = CardRecord::new(
    "Frenzied Goblin",
    "d307d8c7-b9b5-4f8f-933d-f1c64cbbf92f",
    "Carl Critchlow",
// One mana an attack to push whichever blocker matters, which is what
    // keeps a one-drop relevant into the late game.
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin"], 1, 1).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks, you may pay {R}. If you do, target creature can't block this turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            // The target is chosen as the trigger goes on the stack and the
            // payment is offered as it resolves, so the creature is named
            // before its controller knows whether the mana is there.
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::PayOr(PayOrDef::optional(
                &[CostDef::ColoredMana {
                    color: ManaColor::Red,
                    amount: ValueDef::Constant(1),
                }],
                &EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::BlockRestriction(
                        BlockRestrictionDef::CANNOT_BLOCK,
                    )),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            )),
        ),
    ),
);

// RAV 139 — Reroute
pub(in crate::card::sets) static REROUTE: CardRecord = CardRecord::new(
    "Reroute",
    "42794e10-ddcd-4d2d-ab0c-a6b99b6d4662",
    "Christopher Rush",
CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "Change the target of target activated ability with a single target. (Mana abilities can't be targeted.)\nDraw a card.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::ActivatedAbility,
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
            EffectDef::Sequence(&[
                EffectDef::ChangeStackTargets(&crate::card::ChangeStackTargetsDef {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    chooser: PlayerRefDef::EffectController,
                    change: crate::card::StackTargetChangeDef::ChooseNew {
                        optional: false,
                        restriction: None,
                    },
                }),
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
    ),
);

// RAV 156 — Chord of Calling
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHORD_OF_CALLING_156: CardRecord = CardRecord::new(
    "Chord of Calling",
    "e064174b-8f07-4fea-9eef-c3b5d0220b1a",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// RAV 158 — Doubling Season
// Audit: unsupported — Needs prospective token-creation and counter-placement replacement events, including whether counters are placed by an effect rather than a cost or turn-based action.
pub(in crate::card::sets) static DOUBLING_SEASON: CardRecord = CardRecord::new(
    "Doubling Season",
    "c7e71299-98f6-494e-b187-8d22ce5f50af",
    "Wayne Reynolds",
    CardRules::unsupported(),
);

// RAV 163 — Farseek
pub(in crate::card::sets) static FARSEEK: CardRecord = CardRecord::new(
    "Farseek",
    "8180abec-9459-4b81-987e-b1794e45d543",
    "Martina Pilcerova",
CardRules::new_sorcery(mana_cost!("{1}{G}")).with_ability(AbilityDef::spell(
        "Search your library for a Plains, Island, Swamp, or Mountain card, put it onto the battlefield tapped, then shuffle.",
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::HasAnyBasicLandType(&[
                BasicLandType::Plains,
                BasicLandType::Island,
                BasicLandType::Swamp,
                BasicLandType::Mountain,
            ]),
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
    )),
);

// RAV 184 — Stone-Seeder Hierophant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STONE_SEEDER_HIEROPHANT_184: CardRecord = CardRecord::new(
    "Stone-Seeder Hierophant",
    "f4e1b9f9-e58c-4474-9a31-8e5d9f96492e",
    "William Simpson",
    crate::card::CardRules::unsupported(),
);

// RAV 198 — Congregation at Dawn
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CONGREGATION_AT_DAWN_198: CardRecord = CardRecord::new(
    "Congregation at Dawn",
    "2f1b950a-b2fe-4afc-bb79-c9f4c272ea36",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// RAV 203 — Dimir Infiltrator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIMIR_INFILTRATOR_203: CardRecord = CardRecord::new(
    "Dimir Infiltrator",
    "3db9204c-dde8-4241-aac2-1f090566f604",
    "Jim Nelson",
    crate::card::CardRules::unsupported(),
);

// RAV 213 — Lightning Helix
pub(in crate::card::sets) static LIGHTNING_HELIX: CardRecord = CardRecord::new(
    "Lightning Helix",
    "1b2ecf55-c1cc-4b28-b7ce-e1b25305155e",
    "Kev Walker",
    CardRules::new_instant(mana_cost!("{R}{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Lightning Helix deals 3 damage to any target and you gain 3 life.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::Sequence(&[
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(3),
            ),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ]),
    )]),
);

// RAV 221 — Putrefy
pub(in crate::card::sets) static PUTREFY: CardRecord = CardRecord::new(
    "Putrefy",
    "0a16086c-5a74-45d0-8b38-e832cfbc80f7",
    "Jim Nelson",
    CardRules::new_instant(mana_cost!("{1}{B}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target artifact or creature. It can't be regenerated.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Creature),
            ]),
        )],
        EffectDef::WithRule {
            rule: AppliedRuleDef::CannotRegenerate,
            effect: &EffectDef::destroy_target(TargetIndex::PRIMARY),
        },
    )),
);

// RAV 232 — Skyknight Legionnaire
pub(in crate::card::sets) static SKYKNIGHT_LEGIONNAIRE: CardRecord = CardRecord::new(
    "Skyknight Legionnaire",
    "d697ef7f-0e51-4bf1-b0f5-742325706d2a",
    "Jim Murray",
    CardRules::new_creature(mana_cost!("{1}{R}{W}"), &["Human", "Knight"], 2, 2)
        .with_abilities(&[abilities::flying(), abilities::haste()]),
);

// RAV 245 — Dimir Guildmage
pub(in crate::card::sets) static DIMIR_GUILDMAGE: CardRecord = CardRecord::new(
    "Dimir Guildmage",
    "69b822aa-4144-400a-b993-f146cbeed54f",
    "Adam Rex",
    // Castable off either colour but only useful with both: the hybrid cost
    // is what gets it into the deck, and the two halves are why it stays.
    CardRules::new_creature(mana_cost!("{U/B}{U/B}"), &["Human", "Wizard"], 2, 2).with_abilities(
        &[
            AbilityDef::activated_with_targets(
                "{3}{U}: Target player draws a card. Activate only as a sorcery.",
                &[CostDef::Mana(mana_cost!("{3}{U}"))],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                },
            )
            .with_activation_timing(ActivationTimingDef::SorcerySpeed),
            AbilityDef::activated_with_targets(
                "{3}{B}: Target player discards a card. Activate only as a sorcery.",
                &[CostDef::Mana(mana_cost!("{3}{B}"))],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
            )
            .with_activation_timing(ActivationTimingDef::SorcerySpeed),
        ],
    ),
);

// RAV 255 — Boros Signet
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOROS_SIGNET_255: CardRecord = CardRecord::new(
    "Boros Signet",
    "1bae1f86-4639-4424-b47b-fdc826bf6e97",
    "Greg Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// RAV 257 — Cloudstone Curio
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLOUDSTONE_CURIO_257: CardRecord = CardRecord::new(
    "Cloudstone Curio",
    "47cbda17-d368-4dc3-b41c-95b146468b44",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// RAV 260 — Dimir Signet
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIMIR_SIGNET_260: CardRecord = CardRecord::new(
    "Dimir Signet",
    "9a9a1df5-a4e8-49a4-aebe-ca93894ccfcf",
    "Greg Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// RAV 275 — Boros Garrison
pub(in crate::card::sets) static BOROS_GARRISON: CardRecord = CardRecord::new(
    "Boros Garrison",
    "7dfe3f03-078f-44fb-89cd-efa3ebfaf637",
    "John Avon",
    // A karoo: it costs a land drop and a turn, and pays that back one mana
    // at a time. Returning itself is legal and is what an empty board does.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::karoo_bounce(),
        AbilityDef::activated_mana(
            "{T}: Add {R}{W}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one_of_each(
                ManaColor::Red,
                ManaColor::White,
            )),
        ),
    ]),
);

// RAV 276 — Dimir Aqueduct
pub(in crate::card::sets) static DIMIR_AQUEDUCT: CardRecord = CardRecord::new(
    "Dimir Aqueduct",
    "df3c3d56-8291-407e-87a1-94b7d12811fd",
    "John Avon",
    // The blue-black karoo; only the two colours below are its own.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::karoo_bounce(),
        AbilityDef::activated_mana(
            "{T}: Add {U}{B}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one_of_each(
                ManaColor::Blue,
                ManaColor::Black,
            )),
        ),
    ]),
);

// RAV 278 — Golgari Rot Farm
pub(in crate::card::sets) static GOLGARI_ROT_FARM: CardRecord = CardRecord::new(
    "Golgari Rot Farm",
    "104364d5-ede8-4ac5-900f-19947f51bbc1",
    "John Avon",
    // The black-green karoo; only the two colours below are its own.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::karoo_bounce(),
        AbilityDef::activated_mana(
            "{T}: Add {B}{G}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one_of_each(
                ManaColor::Black,
                ManaColor::Green,
            )),
        ),
    ]),
);

// RAV 279 — Overgrown Tomb
pub(in crate::card::sets) static OVERGROWN_TOMB: CardRecord = CardRecord::new(
    "Overgrown Tomb",
    "fce07335-cc78-4683-b2f0-9c98a06ea1d8",
    "Rob Alexander",
    CardRules::new_land(&["Swamp", "Forest"]).with_ability(abilities::shock_land_enters()),
);

// RAV 280 — Sacred Foundry
pub(in crate::card::sets) static SACRED_FOUNDRY: CardRecord = CardRecord::new(
    "Sacred Foundry",
    "168ef687-5797-4b45-b75b-393d8117cebd",
    "Rob Alexander",
    CardRules::new_land(&["Mountain", "Plains"]).with_ability(abilities::shock_land_enters()),
);

// RAV 281 — Selesnya Sanctuary
pub(in crate::card::sets) static SELESNYA_SANCTUARY: CardRecord = CardRecord::new(
    "Selesnya Sanctuary",
    "c5e51787-f9c9-4926-9df1-a384a3092676",
    "John Avon",
    // The green-white karoo; only the two colours below are its own.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::karoo_bounce(),
        AbilityDef::activated_mana(
            "{T}: Add {G}{W}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one_of_each(
                ManaColor::Green,
                ManaColor::White,
            )),
        ),
    ]),
);

// RAV 284 — Temple Garden
pub(in crate::card::sets) static TEMPLE_GARDEN: CardRecord = CardRecord::new(
    "Temple Garden",
    "794a2b79-8c55-4423-8843-7e6e96f84071",
    "Rob Alexander",
    CardRules::new_land(&["Forest", "Plains"]).with_ability(abilities::shock_land_enters()),
);

// RAV 286 — Watery Grave
pub(in crate::card::sets) static WATERY_GRAVE: CardRecord = CardRecord::new(
    "Watery Grave",
    "139b90cd-8272-457a-be32-1298145345be",
    "Rob Alexander",
    CardRules::new_land(&["Island", "Swamp"]).with_ability(abilities::shock_land_enters()),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &FAITH_S_FETTERS,
    &BELLTOWER_SPHINX,
    &COPY_ENCHANTMENT_42,
    &DIZZY_SPELL_43,
    &DRIFT_OF_PHANTASMS_46,
    &MUDDLE_THE_MIXTURE_60,
    &PEEL_FROM_REALITY,
    &REMAND,
    &VEDALKEN_ENTRANCER,
    &DARK_CONFIDANT,
    &DIMIR_HOUSE_GUARD_83,
    &DIMIR_MACHINATIONS_84,
    &LAST_GASP,
    &BREATH_OF_FURY_116,
    &FRENZIED_GOBLIN,
    &REROUTE,
    &CHORD_OF_CALLING_156,
    &DOUBLING_SEASON,
    &FARSEEK,
    &STONE_SEEDER_HIEROPHANT_184,
    &CONGREGATION_AT_DAWN_198,
    &DIMIR_INFILTRATOR_203,
    &LIGHTNING_HELIX,
    &PUTREFY,
    &SKYKNIGHT_LEGIONNAIRE,
    &DIMIR_GUILDMAGE,
    &BOROS_SIGNET_255,
    &CLOUDSTONE_CURIO_257,
    &DIMIR_SIGNET_260,
    &BOROS_GARRISON,
    &DIMIR_AQUEDUCT,
    &GOLGARI_ROT_FARM,
    &OVERGROWN_TOMB,
    &SACRED_FOUNDRY,
    &SELESNYA_SANCTUARY,
    &TEMPLE_GARDEN,
    &WATERY_GRAVE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
