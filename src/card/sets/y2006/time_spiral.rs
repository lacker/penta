//! TSP card records required by supported formats.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AlternativeCastKindDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CardTypeSet;
use crate::card::ColorSet;
use crate::card::ComparisonDef;
use crate::card::CostDef;
use crate::card::CostQuantityDef;
use crate::card::CounterKind;
use crate::card::CounterKindDef;
use crate::card::CounterOperationDef;
use crate::card::DiscardSelectionDef;
use crate::card::EffectChoiceDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::PlayerRelation;
use crate::card::PregameConditionDef;
use crate::card::PrintedManaCost;
use crate::card::ResolvedEffectDurationDef;
use crate::card::TokenCountersDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

// TSP 4 — Benalish Cavalry
pub(in crate::card::sets) static BENALISH_CAVALRY: CardRecord = CardRecord::new(
    "Benalish Cavalry",
    "1013ca9c-1d29-42f6-8665-92f98d076ff8",
    "Paolo Parente",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Knight"], 2, 2)
        .with_ability(abilities::flanking()),
);

// TSP 6 — Cavalry Master
pub(in crate::card::sets) static CAVALRY_MASTER: CardRecord = CardRecord::new(
    "Cavalry Master",
    "f7b19194-87bf-432c-8d34-91dd9520cbd2",
    "Thomas M. Baxa",
    CardRules::new_creature(mana_cost!("{2}{W}{W}"), &["Human", "Knight"], 3, 3).with_abilities(&[
        abilities::flanking(),
        AbilityDef::static_ability(
            "Other creatures you control with flanking have flanking.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ObjectPredicateDef::HasKeyword(crate::card::KeywordAbility::Flanking),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::flanking()),
            },
        ),
    ]),
);

// TSP 19 — Fortify
pub(in crate::card::sets) static FORTIFY: CardRecord = CardRecord::new(
    "Fortify",
    "fd063dc7-a35c-44c9-9f8d-b7bb2dc95bec",
    "Christopher Moeller",
    CardRules::new_instant(mana_cost!("{2}{W}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell(
                "Creatures you control get +2/+0 until end of turn.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::spell(
                "Creatures you control get +0/+2 until end of turn.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(0),
                        ValueDef::Constant(2),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ],
    )),
);

// TSP 26 — Knight of the Holy Nimbus
pub(in crate::card::sets) static KNIGHT_OF_THE_HOLY_NIMBUS: CardRecord = CardRecord::new(
    "Knight of the Holy Nimbus",
    "8fdeb716-4632-4895-b771-0ebd59c868d5",
    "Wayne England",
CardRules::new_creature(
        mana_cost!("{W}{W}"),
        &["Human", "Rebel", "Knight"],
        2,
        2,
    )
    .with_abilities(&[
        abilities::flanking(),
        abilities::regenerates_if_destroyed(
            "If this creature would be destroyed, regenerate it. (Tap it, remove it from combat, and heal all damage on it.)",
        ),
        AbilityDef::activated(
            "{2}: This creature can't be regenerated this turn. Only your opponents may activate this ability.",
            &[CostDef::Mana(mana_cost!("{2}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotRegenerate),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .only_opponents_may_activate(),
    ]),
);

// TSP 29 — Momentary Blink
pub(in crate::card::sets) static MOMENTARY_BLINK: CardRecord = CardRecord::new(
    "Momentary Blink",
    "032e072a-0630-472b-9106-5df554dff785",
    "Anthony S. Waters",
    // Two enters triggers for four mana across two casts, and the blue
    // flashback is why a white deck splashes for it at all.
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Exile target creature you control, then return it to the battlefield under its \
             owner's control.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            // Exiled linked to this spell and returned by the same
            // resolution, so it comes back as a new object with its enters
            // triggers armed.
            EffectDef::Sequence(&[
                EffectDef::ExileLinkedToSource {
                    until_source_leaves: false,
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    face_down: false,
                    then: None,
                },
                EffectDef::ReturnLinkedExiles {
                    object: ObjectPredicateDef::Any,
                    counters: None,
                    zone: ZoneKind::Battlefield,
                    grant: None,
                    controller: None,
                    transformed: false,
                },
            ]),
        ),
        abilities::flashback(&[CostDef::Mana(mana_cost!("{3}{U}"))]),
    ]),
);

// TSP 40 — Serra Avenger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SERRA_AVENGER: CardRecord = CardRecord::new(
    "Serra Avenger",
    "9e9d7c1c-3bfd-4705-9bc2-5ca3f84cc32a",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// TSP 48 — Ancestral Vision
pub(in crate::card::sets) static ANCESTRAL_VISION: CardRecord = CardRecord::new(
    "Ancestral Vision",
    "bccedc4d-38c7-4bf3-9ca7-4febd6c49d3d",
    "Mark Poole",
    CardRules::base(
        CardTypeSet::single(CardType::Sorcery),
        PrintedManaCost::None,
    )
    .with_abilities(&[
        abilities::suspend(
            "Suspend 4—{U}",
            &crate::card::SuspendAbilityDef::fixed(4, &[CostDef::Mana(mana_cost!("{U}"))]),
        ),
        AbilityDef::spell_with_targets(
            "Target player draws three cards.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
            },
        ),
    ]),
);

// TSP 51 — Cancel
pub(in crate::card::sets) static CANCEL: CardRecord = CardRecord::new(
    "Cancel",
    "b4e175f7-f649-451b-9ee5-ad1140b2e8a7",
    "Mark Poole",
    CardRules::new_instant(mana_cost!("{1}{U}{U}")).with_ability(AbilityDef::counter_target(
        "Counter target spell.",
        &AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::Spell,
            zones: &[ZoneKind::Stack],
            controller: None,
            owner: None,
        }),
    )),
);

// TSP 53 — Clockspinning
pub(in crate::card::sets) static CLOCKSPINNING: CardRecord = CardRecord::new(
    "Clockspinning",
    "1323d548-e2fe-47c5-8df3-f181aed537c5",
    "Zoltan Boros & Gabor Szikszai",
CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[
        abilities::buyback(
            &[CostDef::Mana(mana_cost!("{3}"))],
        ),
        AbilityDef::spell_with_targets(
            "Choose a counter on target permanent or suspended card. Remove that counter from that permanent or card or put another of those counters on it.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyOf(&[
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasAnyCounter,
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    AbilityTargetPredicate::Object {
                        object: abilities::SUSPENDED_CARD,
                        zones: &[ZoneKind::Exile],
                        controller: None,
                        owner: None,
                    },
                ]),
            )],
            EffectDef::ChooseCounterKind {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: &EffectDef::ChooseEffect {
                    player: EffectRecipientDef::Controller,
                    choices: &[
                        EffectChoiceDef {
                            label: "Remove the chosen counter",
                            effect: EffectDef::ModifyCounters {
                                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                kind: CounterKindDef::Chosen,
                                operation: CounterOperationDef::Remove,
                                amount: ValueDef::Constant(1),
                            },
                        },
                        EffectChoiceDef {
                            label: "Put another of the chosen counter",
                            effect: EffectDef::ModifyCounters {
                                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                kind: CounterKindDef::Chosen,
                                operation: CounterOperationDef::Add,
                                amount: ValueDef::Constant(1),
                            },
                        },
                    ],
                },
            },
        ),
    ]),
);

// TSP 56 — Deep-Sea Kraken
pub(in crate::card::sets) static DEEP_SEA_KRAKEN: CardRecord = CardRecord::new(
    "Deep-Sea Kraken",
    "8e050532-e245-4eea-90a5-03e3e410dcbe",
    "Christopher Moeller",
CardRules::new_creature(mana_cost!("{7}{U}{U}{U}"), &["Kraken"], 6, 6).with_abilities(&[
        abilities::cannot_be_blocked(),
        abilities::suspend(
            "Suspend 9—{2}{U}",
            &crate::card::SuspendAbilityDef::fixed(9, &[CostDef::Mana(mana_cost!("{2}{U}"))]),
        ),
        AbilityDef::triggered_if(
            "Whenever an opponent casts a spell, if this card is suspended, remove a time counter from it.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent)),
            &abilities::SUSPEND_SOURCE_IS_SUSPENDED,
            EffectDef::RemoveCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::named("time"),
                amount: ValueDef::Constant(1),
            },
        )
        .with_source_zones(&[ZoneKind::Exile]),
    ]),
);

// TSP 66 — Looter il-Kor
pub(in crate::card::sets) static LOOTER_IL_KOR: CardRecord = CardRecord::new(
    "Looter il-Kor",
    "368ee06f-9021-4b65-9f53-9c326bf3a27f",
    "Mike Dringenberg",
    // Shadow makes the trigger unconditional in practice, which is why a
    // 1/1 that loots every turn is worth two mana.
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Kor", "Rogue"], 1, 1).with_abilities(&[
        abilities::shadow(),
        AbilityDef::triggered(
            "Whenever this creature deals damage to an opponent, draw a card, then discard a \
             card.",
            // Any damage rather than combat damage, so a pump that pings
            // still loots.
            TriggerEventDef::damage_to_player(ObjectPredicateDef::Source, PlayerRelation::Opponent),
            EffectDef::Sequence(&[
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
        ),
    ]),
);

// TSP 86 — Think Twice
pub(in crate::card::sets) static THINK_TWICE: CardRecord = CardRecord::new(
    "Think Twice",
    "352d99db-de6d-4405-90ec-b144abbaa5a4",
    "Jim Nelson",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[
        AbilityDef::spell("Draw a card.", abilities::draw_cards(ValueDef::Constant(1))),
        abilities::flashback(&[CostDef::Mana(mana_cost!("{2}{U}"))]),
    ]),
);

// TSP 104 — Dread Return
pub(in crate::card::sets) static DREAD_RETURN: CardRecord = CardRecord::new(
    "Dread Return",
    "d7e304fc-0ace-459e-8d2f-376f1899639c",
    "Kev Walker",
    // The flashback costs no mana at all, which is why the card is about
    // having three expendable bodies rather than about having four lands.
    CardRules::new_sorcery(mana_cost!("{2}{B}{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Return target creature card from your graveyard to the battlefield.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
            },
        ),
        AbilityDef::alternative_cast(
            &[CostDef::sacrifice(
                ObjectPredicateDef::HasType(CardType::Creature),
                CostQuantityDef::Fixed(3),
            )],
            AlternativeCastKindDef::Flashback,
            Some("Flashback—Sacrifice three creatures."),
            EffectDef::None,
        ),
    ]),
);

// TSP 131 — Smallpox
// Audit: unsupported — Needs each player's discard and sacrifice choices frozen in APNAP order before each shared batch moves.
pub(in crate::card::sets) static SMALLPOX: CardRecord = CardRecord::new(
    "Smallpox",
    "175d5a88-2597-4e85-aed6-7a65c0595fb4",
    "Janine Johnston",
    crate::card::CardRules::unsupported(),
);

// TSP 143 — Ancient Grudge
pub(in crate::card::sets) static ANCIENT_GRUDGE: CardRecord = CardRecord::new(
    "Ancient Grudge",
    "89cbad1f-4f16-4d5f-a485-5bf950565216",
    "Jim Nelson",
    CardRules::new_instant(mana_cost!("{1}{R}")).with_abilities(&[
        AbilityDef::destroy_target(
            "Destroy target artifact.",
            &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                CardType::Artifact,
            )),
        ),
        abilities::flashback(&[CostDef::Mana(mana_cost!("{G}"))]),
    ]),
);

// TSP 146 — Blazing Blade Askari
pub(in crate::card::sets) static BLAZING_BLADE_ASKARI: CardRecord = CardRecord::new(
    "Blazing Blade Askari",
    "cabf35d5-de8a-4d9d-be59-7ad7039873c6",
    "Dan Frazier",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Human", "Knight"], 2, 2).with_abilities(&[
        abilities::flanking(),
        AbilityDef::activated(
            "{2}: This creature becomes colorless until end of turn.",
            &[CostDef::Mana(mana_cost!("{2}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::set_colors(ColorSet::empty()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// TSP 161 — Greater Gargadon
pub(in crate::card::sets) static GREATER_GARGADON: CardRecord = CardRecord::new(
    "Greater Gargadon",
    "653ddfa0-2088-4503-a3ab-b0f1d55d8351",
    "Rob Alexander",
CardRules::new_creature(mana_cost!("{9}{R}"), &["Beast"], 9, 7).with_abilities(&[
        abilities::suspend(
            "Suspend 10—{R}",
            &crate::card::SuspendAbilityDef::fixed(10, &[CostDef::Mana(mana_cost!("{R}"))]),
        ),
        AbilityDef::activated(
            "Sacrifice an artifact, creature, or land: Remove a time counter from this card. Activate only if this card is suspended.",
            &[CostDef::SacrificePermanent {
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Land),
                ]),
                controller: PlayerRelation::You,
            }],
            EffectDef::RemoveCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::named("time"),
                amount: ValueDef::Constant(1),
            },
        )
        .with_source_zones(&[ZoneKind::Exile])
        .with_activation_condition(&abilities::SUSPEND_SOURCE_IS_SUSPENDED),
    ]),
);

// TSP 170 — Mogg War Marshal
pub(in crate::card::sets) static MOGG_WAR_MARSHAL: CardRecord = CardRecord::new(
    "Mogg War Marshal",
    "8b9e0bdb-b615-447a-b80d-d7244c25c56e",
    "Wayne England",
// Letting the echo go unpaid is the normal line: three bodies for two
    // mana, and the last one arrives because the Marshal died.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin", "Warrior"], 1, 1).with_abilities(&[
        abilities::echo(
            "Echo {1}{R} (At the beginning of your upkeep, if this came under your control since the beginning of your last upkeep, sacrifice it unless you pay its echo cost.)",
            &[CostDef::Mana(mana_cost!("{1}{R}"))],
        ),
        AbilityDef::triggered(
            "When this creature enters or dies, create a 1/1 red Goblin creature token.",
            // One printed sentence with two ways in, so it is one ability
            // watching both zone changes rather than two abilities.
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
            ]),
            EffectDef::create_creature_token(&["Goblin"], &[ManaColor::Red], 1, 1),
        ),
    ]),
);

// TSP 176 — Rift Bolt
pub(in crate::card::sets) static RIFT_BOLT: CardRecord = CardRecord::new(
    "Rift Bolt",
    "88dde96e-6824-4d26-9fb5-86b9f3c50959",
    "Michael Sutfin",
    CardRules::new_sorcery(mana_cost!("{2}{R}")).with_abilities(&[
        abilities::suspend(
            "Suspend 1—{R}",
            &crate::card::SuspendAbilityDef::fixed(1, &[CostDef::Mana(mana_cost!("{R}"))]),
        ),
        AbilityDef::spell_with_targets(
            "Rift Bolt deals 3 damage to any target.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(3),
            ),
        ),
    ]),
);

// TSP 180 — Sulfurous Blast
// Audit: unsupported — Needs a "cast during your main phase" condition. The closest is SourceCastAtInstantSpeed, and its negation is not the same question: a spell cast in your own main phase in response to something was cast when a sorcery could not have been, so the negation would report the smaller amount where the printed card gives the larger one.
pub(in crate::card::sets) static SULFUROUS_BLAST: CardRecord = CardRecord::new(
    "Sulfurous Blast",
    "67511e0e-be09-4f4e-9949-b9ecbdc7f536",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// TSP 193 — Durkwood Baloth
pub(in crate::card::sets) static DURKWOOD_BALOTH: CardRecord = CardRecord::new(
    "Durkwood Baloth",
    "670521c3-df02-487d-a299-49419e41889f",
    "Dan Frazier",
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Beast"], 5, 5).with_ability(
        abilities::suspend(
            "Suspend 5—{G}",
            &crate::card::SuspendAbilityDef::fixed(5, &[CostDef::Mana(mana_cost!("{G}"))]),
        ),
    ),
);

// TSP 251 — Chromatic Star
pub(in crate::card::sets) static CHROMATIC_STAR: CardRecord = CardRecord::new(
    "Chromatic Star",
    "1d7a1357-debd-49b0-9fd5-560d5b3f589e",
    "Alex Horley-Orlandelli",
    // A card that fixes one mana and replaces itself, and does the second
    // half however it dies rather than only when it is spent.
    // The draw is a separate trigger rather than part of the mana ability,
    // which is the whole difference from Chromatic Sphere: the mana arrives at
    // once and the card waits on the stack, so anything that answers the Star
    // after it has been sacrificed is already too late.
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::activated_mana(
            "{1}, {T}, Sacrifice this artifact: Add one mana of any color.",
            &[
                CostDef::Mana(mana_cost!("{1}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
        AbilityDef::triggered(
            "When this artifact is put into a graveyard from the battlefield, draw a card.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// TSP 257 — Jhoira's Timebug
pub(in crate::card::sets) static JHOIRAS_TIMEBUG: CardRecord = CardRecord::new(
    "Jhoira's Timebug",
    "9ce2c6d7-505b-490b-9c6f-b5166c9ff71d",
    "Dan Frazier",
CardRules::new_artifact_creature(mana_cost!("{2}"), &["Insect"], 1, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Choose target permanent you control or suspended card you own. If it has a time counter on it, you may remove a time counter from it or put another time counter on it.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyOf(&[
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Any,
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                    AbilityTargetPredicate::Object {
                        object: abilities::SUSPENDED_CARD,
                        zones: &[ZoneKind::Exile],
                        controller: None,
                        owner: Some(PlayerRelation::You),
                    },
                ]))],
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::TargetMatches {
                    slot: TargetIndex::PRIMARY,
                    object: ObjectPredicateDef::HasCounter(CounterKind::named("time")),
                },
                then: &EffectDef::ChooseEffect {
                    player: EffectRecipientDef::Controller,
                    choices: &[
                        EffectChoiceDef {
                            label: "Do nothing",
                            effect: EffectDef::None,
                        },
                        EffectChoiceDef {
                            label: "Remove a time counter",
                            effect: EffectDef::ModifyCounters {
                                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                kind: CounterKindDef::Fixed(CounterKind::named("time")),
                                operation: CounterOperationDef::Remove,
                                amount: ValueDef::Constant(1),
                            },
                        },
                        EffectChoiceDef {
                            label: "Put another time counter",
                            effect: EffectDef::ModifyCounters {
                                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                kind: CounterKindDef::Fixed(CounterKind::named("time")),
                                operation: CounterOperationDef::Add,
                                amount: ValueDef::Constant(1),
                            },
                        },
                    ],
                },
            },
        ),
    ),
);

// TSP 264 — Stuffy Doll
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STUFFY_DOLL: CardRecord = CardRecord::new(
    "Stuffy Doll",
    "14ca7425-a499-4864-b955-369ef2577849",
    "Dave Allsop",
    crate::card::CardRules::unsupported(),
);

// TSP 274 — Gemstone Caverns
pub(in crate::card::sets) static GEMSTONE_CAVERNS: CardRecord = CardRecord::new(
    "Gemstone Caverns",
    "94d74254-4750-4fb3-9e53-473a5f98b315",
    "Martina Pilcerova",
CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::opening_hand_with(
                "If this card is in your opening hand and you're not the starting player, you may begin the game with Gemstone Caverns on the battlefield with a luck counter on it. If you do, exile a card from your hand.",
                PregameConditionDef::NotStartingPlayer,
                &[CostDef::ExileCardFromHand(ObjectPredicateDef::Any)],
                EffectDef::WithBattlefieldArrival {
                    effect: &const {
                        EffectDef::MoveToZone {
                            object: EffectRecipientDef::Source,
                            zone: ZoneKind::Battlefield,
                            placement: ZonePlacement::Top,
                        }
                    },
                    arrival: crate::card::BattlefieldArrivalDef {
                        counters: Some(TokenCountersDef {
                            kind: CounterKind::named("luck"),
                            amount: ValueDef::Constant(1),
                        }),
                        ..crate::card::BattlefieldArrivalDef::DEFAULT
                    },
                },
            ),
            AbilityDef::activated_mana_if(
                "{T}: Add {C}.",
                &[CostDef::TapSource],
                &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("luck"),
                    comparison: ComparisonDef::LessOrEqual,
                    amount: 0,
                },
                EffectDef::AddMana(AddManaEffectDef::one(crate::card::ManaColor::Colorless)),
            ),
            AbilityDef::activated_mana_if(
                "{T}: If this land has a luck counter on it, add one mana of any color instead.",
                &[CostDef::TapSource],
                &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("luck"),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                },
                EffectDef::AddMana(AddManaEffectDef::any_color()),
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &BENALISH_CAVALRY,
    &CAVALRY_MASTER,
    &FORTIFY,
    &KNIGHT_OF_THE_HOLY_NIMBUS,
    &MOMENTARY_BLINK,
    &SERRA_AVENGER,
    &ANCESTRAL_VISION,
    &CANCEL,
    &CLOCKSPINNING,
    &DEEP_SEA_KRAKEN,
    &LOOTER_IL_KOR,
    &THINK_TWICE,
    &DREAD_RETURN,
    &SMALLPOX,
    &ANCIENT_GRUDGE,
    &BLAZING_BLADE_ASKARI,
    &GREATER_GARGADON,
    &MOGG_WAR_MARSHAL,
    &RIFT_BOLT,
    &SULFUROUS_BLAST,
    &DURKWOOD_BALOTH,
    &CHROMATIC_STAR,
    &JHOIRAS_TIMEBUG,
    &STUFFY_DOLL,
    &GEMSTONE_CAVERNS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
