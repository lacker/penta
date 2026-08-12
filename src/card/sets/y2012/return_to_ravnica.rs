//! Return to Ravnica card records used by the built-in ISD–RTR Standard deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    AddManaEffectDef, AppliedEffectDef, CardArt, CardBehavior, CardRules, CardSet, CardSupertype,
    CardType, CounterKind, EffectDef, EffectDurationDef, EffectRecipientDef, ManaColor,
    ObjectPredicateDef, PlayerRelation, ReplacementEventDef, TriggerEventDef, TurnStepDef,
    ValueDef, ZoneKind, ZoneMoveCauseDef, ZonePlacement, abilities, cards,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

// RTR 1 — Angel of Serenity
pub(in crate::card::sets) static ANGEL_OF_SERENITY: CardRecord = CardRecord::new(
    cards::ANGEL_OF_SERENITY,
    "Angel of Serenity",
    CardArt::new("f10d82f7-7759-457e-a9bb-f9a5bd968f82", "Aleksi Briclot"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(
        mana_cost!("{4}{W}{W}{W}"),
        &["Angel"],
        5,
        6,
    )
    .with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_with_targets("When this creature enters, you may exile up to three other target creatures from the battlefield and/or creature cards from graveyards.", TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            }, &[AbilityTargetDef::up_to(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                zones: &[ZoneKind::Battlefield, ZoneKind::Graveyard],
                controller: None,
                owner: None,
            },
            3,
        )], EffectDef::May(&EffectDef::ExileLinkedToSource {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            })),
        AbilityDef::triggered(
            "When this creature leaves the battlefield, return the exiled cards to their owners' hands.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: Some(ZoneKind::Battlefield),
                to: None,
            },
            EffectDef::ReturnLinkedExiles {
                zone: ZoneKind::Hand,
                grant: None,
            },
        ),
    ]),
);

// RTR 18 — Rest in Peace
pub(in crate::card::sets) static REST_IN_PEACE: CardRecord = CardRecord::new(
    cards::REST_IN_PEACE,
    "Rest in Peace",
    CardArt::new("37c2b1d1-faa0-40fd-82f4-216604ce7635", "Terese Nielsen"),
    CardSet::ReturnToRavnica,
    CardRules::new_enchantment(mana_cost!("{1}{W}")).with_abilities(&[
        AbilityDef::triggered(
            "When this enchantment enters, exile all graveyards.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            },
            EffectDef::MoveToZone {
                object: EffectRecipientDef::MatchingObjects {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Graveyard],
                    controller: PlayerRelation::Any,
                },
                zone: ZoneKind::Exile,
                controller: None,
                placement: ZonePlacement::Top,
            },
        ),
        AbilityDef::replacement_for(
            "If a card or token would be put into a graveyard from anywhere, exile it instead.",
            ReplacementEventDef::AnyObjectWouldMove {
                to: ZoneKind::Graveyard,
            },
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Source,
                zone: ZoneKind::Exile,
                controller: None,
                placement: ZonePlacement::Top,
            },
        ),
    ]),
);

// RTR 36 — Dispel
pub(in crate::card::sets) static DISPEL: CardRecord = CardRecord::new(
    cards::DISPEL,
    "Dispel",
    CardArt::new("08d4a8d7-c136-472f-8146-a1100701ca4f", "Chase Stone"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::counter_target(
        "Counter target instant spell.",
        &AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::HasType(CardType::Instant)),
    )),
);

/// The ability Jace's first one leaves behind. It belongs to no permanent,
/// so "an opponent" is read against the player who installed it.
static JACE_ATTACK_TAX: AbilityDef = AbilityDef::triggered(
    "Whenever a creature an opponent controls attacks, it gets -1/-0 until end of turn.",
    TriggerEventDef::Attacks(ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
    ])),
    EffectDef::Apply {
        recipient: EffectRecipientDef::TriggeringObject,
        effect: AppliedEffectDef::ModifyPowerToughness {
            power: ValueDef::Constant(-1),
            toughness: ValueDef::Constant(0),
        },
        duration: EffectDurationDef::UntilEndOfTurn,
    },
);

static JACE_ARCHITECT_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::activated(
        "+1: Until your next turn, whenever a creature an opponent controls attacks, it gets -1/-0 until end of turn.",
        &[AbilityCostDef::Loyalty(1)],
        EffectDef::TriggerUntilYourNextTurn {
            ability: &JACE_ATTACK_TAX,
        },
    ),
    AbilityDef::activated(
        "−2: Reveal the top three cards of your library. An opponent separates those cards into two piles. Put one pile into your hand and the other on the bottom of your library in any order.",
        &[AbilityCostDef::Loyalty(-2)],
        EffectDef::RevealAndSplitIntoPiles {
            count: ValueDef::Constant(3),
            rest: ZoneKind::Library,
            placement: ZonePlacement::Bottom,
        },
    ),
    AbilityDef::not_implemented(
        "−8: For each player, search that player's library for a nonland card and exile it, then that player shuffles. You may cast those cards without paying their mana costs.",
        "Casting an exiled card without paying its mana cost is not an available alternative cost.",
    ),
];

// RTR 44 — Jace, Architect of Thought
pub(in crate::card::sets) static JACE_ARCHITECT_OF_THOUGHT: CardRecord = CardRecord::new(
    cards::JACE_ARCHITECT_OF_THOUGHT,
    "Jace, Architect of Thought",
    CardArt::new("d4df3a38-678e-42dc-a3fd-d1d399368f07", "Jaime Jones"),
    CardSet::ReturnToRavnica,
    CardRules::new_planeswalker(mana_cost!("{2}{U}{U}"), &["Jace"], 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&JACE_ARCHITECT_ABILITIES),
);

// RTR 54 — Syncopate
pub(in crate::card::sets) static SYNCOPATE: CardRecord = CardRecord::new(
    cards::SYNCOPATE,
    "Syncopate",
    CardArt::new("ba6f218f-83b0-4b68-a00f-0327cd79f32a", "Clint Cearley"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{X}{U}")).with_ability(
        AbilityDef::spell_with_targets("Counter target spell unless its controller pays {X}. If that spell is countered this way, exile it instead of putting it into its owner's graveyard.", &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Spell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )], EffectDef::CounterUnlessPaid {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::ChosenX,
                zone: ZoneKind::Exile,
            }),
    ),
);

/// What the Demon takes when an opponent feeds it: it stays home for the turn
/// and grows permanently.
static DESECRATION_DEMON_TRIBUTE: EffectDef = EffectDef::Sequence(&[
    EffectDef::Tap {
        object: EffectRecipientDef::Source,
    },
    EffectDef::AddCounters {
        object: EffectRecipientDef::Source,
        kind: CounterKind::PlusOnePlusOne,
        amount: ValueDef::Constant(1),
    },
]);

// RTR 63 — Desecration Demon
pub(in crate::card::sets) static DESECRATION_DEMON: CardRecord = CardRecord::new(
    cards::DESECRATION_DEMON,
    "Desecration Demon",
    CardArt::new("8242fade-754c-4404-b3fb-f3cccf84b3b6", "Jason Chan"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(
        mana_cost!("{2}{B}{B}"),
        &["Demon"],
        6,
        6,
    )
    .with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "At the beginning of each combat, any opponent may sacrifice a creature of their choice. If a player does, tap this creature and put a +1/+1 counter on it.",
            // Each combat, so on either player's turn.
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::Any,
            },
            EffectDef::SacrificeOfChoice {
                player: EffectRecipientDef::Opponent,
                object: ObjectPredicateDef::HasType(CardType::Creature),
                then: Some(&DESECRATION_DEMON_TRIBUTE),
                optional: true,
            },
        ),
    ]),
);

// RTR 82 — Ultimate Price
pub(in crate::card::sets) static ULTIMATE_PRICE: CardRecord = CardRecord::new(
    cards::ULTIMATE_PRICE,
    "Ultimate Price",
    CardArt::new("d2b4912a-83a2-4870-8fac-81fa79da2830", "Karl Kopinski"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{1}{B}")).with_ability(AbilityDef::destroy_target(
        "Destroy target monocolored creature.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::ColorCount(1),
        ])),
        true,
    )),
);

static UNDERWORLD_CONNECTIONS_DRAW: AbilityDef = AbilityDef::activated(
    "{T}, Pay 1 life: Draw a card.",
    &[AbilityCostDef::TapSource, AbilityCostDef::PayLife(1)],
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    },
);

// RTR 83 — Underworld Connections
pub(in crate::card::sets) static UNDERWORLD_CONNECTIONS: CardRecord = CardRecord::new(
    cards::UNDERWORLD_CONNECTIONS,
    "Underworld Connections",
    CardArt::new("19c52e3b-b3b8-4243-96fe-fa4c8eea7c59", "Yeong-Hao Han"),
    CardSet::ReturnToRavnica,
    CardRules::new_enchantment(mana_cost!("{1}{B}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant land",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Land),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted land has \"{T}, Pay 1 life: Draw a card.\"",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::GrantAbility(&UNDERWORLD_CONNECTIONS_DRAW),
                    duration: EffectDurationDef::WhileSourceRemainsInZone,
                },
            ),
        ]),
);

// RTR 101 — Mizzium Mortars
pub(in crate::card::sets) static MIZZIUM_MORTARS: CardRecord = CardRecord::new(
    cards::MIZZIUM_MORTARS,
    "Mizzium Mortars",
    CardArt::new("d4ded88d-2688-4f5e-a8b2-16216cf9c792", "Noah Bradley"),
    CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Mizzium Mortars deals 4 damage to target creature you don't control.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::NotYou),
                    owner: None,
                },
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(4),
            },
        ),
        abilities::overload(
            mana_cost!("{3}{R}{R}{R}"),
            "Mizzium Mortars deals 4 damage to each creature you don't control.",
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::MatchingObjects {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: PlayerRelation::NotYou,
                },
                amount: ValueDef::Constant(4),
            },
        ),
    ]),
);

// RTR 141 — Abrupt Decay
pub(in crate::card::sets) static ABRUPT_DECAY: CardRecord = CardRecord::new(
    cards::ABRUPT_DECAY,
    "Abrupt Decay",
    CardArt::new("3b1e92b4-6e53-4dba-a572-c67e01965ac5", "Svetlin Velinov"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{B}{G}")).with_abilities(&[
        abilities::cannot_be_countered(),
        AbilityDef::spell_with_targets(
            "Destroy target nonland permanent with mana value 3 or less.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                        ObjectPredicateDef::ManaValueAtMost(3),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            },
        ),
    ]),
);

// RTR 145 — Azorius Charm
pub(in crate::card::sets) static AZORIUS_CHARM: CardRecord = CardRecord::new(
    cards::AZORIUS_CHARM,
    "Azorius Charm",
    CardArt::new("26adc211-d089-4102-91e5-225bbeb5f382", "Zoltan Boros"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{W}{U}")).with_ability(AbilityDef::choose_one_spell(
        "Choose one —\n• Creatures you control gain lifelink until end of turn.\n• Draw a card.\n• Put target attacking or blocking creature on top of its owner's library.",
        &[
            AbilityDef::spell(
                "Creatures you control gain lifelink until end of turn",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::MatchingObjects {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: PlayerRelation::You,
                    },
                    effect: AppliedEffectDef::GrantAbility(&abilities::lifelink()),
                    duration: EffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::spell(
                "Draw a card",
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::spell_with_targets("Put an attacking or blocking creature on top of its owner's library", &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::AttackingOrBlocking,
                ]),
            )], EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Library,
                    controller: None,
                    placement: ZonePlacement::Top,
                }),
        ],
    )),
);

// RTR 153 — Counterflux
pub(in crate::card::sets) static COUNTERFLUX: CardRecord = CardRecord::new(
    cards::COUNTERFLUX,
    "Counterflux",
    CardArt::new("94e4b773-40a4-4272-85dd-f728ada22748", "Scott M. Fischer"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{U}{U}{R}")).with_abilities(&[
        abilities::cannot_be_countered(),
        AbilityDef::spell_with_targets(
            "Counter target spell you don't control.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Spell,
                    zones: &[ZoneKind::Stack],
                    controller: Some(PlayerRelation::NotYou),
                    owner: None,
                },
            )],
            EffectDef::Counter {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Graveyard,
            },
        ),
        abilities::overload(
            mana_cost!("{1}{U}{U}{R}"),
            "Counter each spell you don't control.",
            EffectDef::Counter {
                object: EffectRecipientDef::MatchingObjects {
                    object: ObjectPredicateDef::Spell,
                    zones: &[ZoneKind::Stack],
                    controller: PlayerRelation::NotYou,
                },
                zone: ZoneKind::Graveyard,
            },
        ),
    ]),
);

// RTR 155 — Detention Sphere
pub(in crate::card::sets) static DETENTION_SPHERE: CardRecord = CardRecord::new(
    cards::DETENTION_SPHERE,
    "Detention Sphere",
    CardArt::new("afee5464-83b7-4d7a-b407-9ee7de21535b", "Kev Walker"),
    CardSet::ReturnToRavnica,
    CardRules::new_enchantment(mana_cost!("{1}{W}{U}")).with_abilities(&[
        AbilityDef::triggered_with_targets("When this enchantment enters, you may exile target nonland permanent not named Detention Sphere and all other permanents with the same name as that permanent.", TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: None,
                to: Some(ZoneKind::Battlefield),
            }, &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    // By name rather than by identity, so a second Sphere is
                    // no more a legal target than this one.
                    ObjectPredicateDef::Not(&ObjectPredicateDef::SharesNameWithSource),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )], EffectDef::May(&EffectDef::ExileLinkedToSource {
                object: EffectRecipientDef::ObjectsSharingNameWithTarget(TargetIndex::PRIMARY),
            })),
        AbilityDef::triggered(
            "When this enchantment leaves the battlefield, return the exiled cards to the battlefield under their owner's control.",
            TriggerEventDef::ZoneChanged {
                object: ObjectPredicateDef::Source,
                from: Some(ZoneKind::Battlefield),
                to: None,
            },
            EffectDef::ReturnLinkedExiles {
                zone: ZoneKind::Battlefield,
                grant: None,
            },
        ),
    ]),
);

// RTR 165 — Grisly Salvage
pub(in crate::card::sets) static GRISLY_SALVAGE: CardRecord = CardRecord::new(
    cards::GRISLY_SALVAGE,
    "Grisly Salvage",
    CardArt::new("dcb5eb2a-ae7a-4416-970c-6e9306689c88", "Dave Kendall"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{B}{G}")).with_ability(
        AbilityDef::custom_full(
            "Reveal the top five cards of your library. You may put a creature or land card from among them into your hand. Put the rest into your graveyard.",
            CardBehavior::GrislySalvage,
            "Implemented by the named card-local special behavior.",
        ),
    ),
);

// RTR 172 — Izzet Charm
pub(in crate::card::sets) static IZZET_CHARM: CardRecord = CardRecord::new(
    cards::IZZET_CHARM,
    "Izzet Charm",
    CardArt::new("1e3a5af6-5423-442b-a207-364e97a871d8", "Zoltan Boros"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{U}{R}")).with_ability(
        AbilityDef::choose_one_spell(
            "Choose one —\n• Counter target noncreature spell unless its controller pays {2}.\n• Izzet Charm deals 2 damage to target creature.\n• Draw two cards, then discard two cards.",
            &[
                AbilityDef::spell_with_targets("Counter a noncreature spell unless its controller pays {2}", &[AbilityTargetDef::exactly_one_spell(
                    ObjectPredicateDef::NoncreatureSpell,
                )], EffectDef::CounterUnlessPaid {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(2),
                        zone: ZoneKind::Graveyard,
                    }),
                AbilityDef::spell_with_targets("Deal 2 damage to a creature", &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )], EffectDef::DealDamage {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(2),
                    }),
                AbilityDef::spell(
                    "Draw two cards, then discard two cards",
                    EffectDef::Sequence(&[
                        EffectDef::DrawCards {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(2),
                        },
                        EffectDef::DiscardCards {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(2),
                        },
                    ]),
                ),
            ],
        ),
    ),
);

// RTR 173 — Izzet Staticaster
pub(in crate::card::sets) static IZZET_STATICASTER: CardRecord = CardRecord::new(
    cards::IZZET_STATICASTER,
    "Izzet Staticaster",
    CardArt::new("190ac2fe-532d-4d7e-9d74-07ae6850aac8", "Scott M. Fischer"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(
        mana_cost!("{1}{U}{R}"),
        &["Human", "Wizard"],
        0,
        3,
    )
    .with_abilities(&[
        abilities::flash(),
        abilities::haste(),
        AbilityDef::activated_with_targets("{T}: This creature deals 1 damage to target creature and each other creature with the same name as that creature.", &[AbilityCostDef::TapSource], &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )], // The target and every other creature sharing its name are one
            // set, so the two printed halves are a single sweep.
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::ObjectsSharingNameWithTarget(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            }),
    ]),
);

// RTR 178 — Loxodon Smiter
pub(in crate::card::sets) static LOXODON_SMITER: CardRecord = CardRecord::new(
    cards::LOXODON_SMITER,
    "Loxodon Smiter",
    CardArt::new("69247168-2bfb-4cce-a2a6-61459a0fbce4", "Ryan Barger"),
    CardSet::ReturnToRavnica,
    CardRules::new_creature(
        mana_cost!("{1}{G}{W}"),
        &["Elephant", "Soldier"],
        4,
        4,
    )
    .with_abilities(&[
        abilities::cannot_be_countered(),
        AbilityDef::replacement_for(
            "If a spell or ability an opponent controls causes you to discard this card, put it onto the battlefield instead of putting it into your graveyard.",
            ReplacementEventDef::WouldMove {
                from: ZoneKind::Hand,
                to: ZoneKind::Graveyard,
                cause: ZoneMoveCauseDef::EffectControlledBy(PlayerRelation::Opponent),
            },
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Source,
                zone: ZoneKind::Battlefield,
                controller: None,
                placement: ZonePlacement::Top,
            },
        )
        .with_source_zones(&[ZoneKind::Hand]),
    ]),
);

// RTR 194 — Selesnya Charm
pub(in crate::card::sets) static SELESNYA_CHARM: CardRecord = CardRecord::new(
    cards::SELESNYA_CHARM,
    "Selesnya Charm",
    CardArt::new("a9848eab-1d3a-4ab0-adf6-c20858aa3afb", "Zoltan Boros"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{G}{W}")).with_ability(AbilityDef::choose_one_spell(
        "Choose one —\n• Target creature gets +2/+2 and gains trample until end of turn.\n• Exile target creature with power 5 or greater.\n• Create a 2/2 white Knight creature token with vigilance.",
        &[
            AbilityDef::spell_with_targets("Target creature gets +2/+2 and gains trample until end of turn", &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )], EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::ModifyPowerToughness {
                            power: ValueDef::Constant(2),
                            toughness: ValueDef::Constant(2),
                        },
                        AppliedEffectDef::GrantAbility(&abilities::trample()),
                    ]),
                    duration: EffectDurationDef::UntilEndOfTurn,
                }),
            AbilityDef::spell_with_targets("Exile a creature with power 5 or greater", &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::PowerAtLeast(5),
                ]),
            )], EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Exile,
                    controller: None,
                    placement: ZonePlacement::Top,
                }),
            AbilityDef::spell(
                "Create a 2/2 white Knight creature token with vigilance",
                EffectDef::CreateToken {
                    token: cards::KNIGHT_TOKEN_2_2_WHITE,
                    count: ValueDef::Constant(1),
                },
            ),
        ],
    )),
);

// RTR 200 — Sphinx's Revelation
pub(in crate::card::sets) static SPHINXS_REVELATION: CardRecord = CardRecord::new(
    cards::SPHINXS_REVELATION,
    "Sphinx's Revelation",
    CardArt::new("404d9413-ef57-4b6e-8584-48a1dc7fe6f1", "Slawomir Maniak"),
    CardSet::ReturnToRavnica,
    CardRules::new_instant(mana_cost!("{X}{W}{U}{U}")).with_ability(AbilityDef::custom_full(
        "You gain X life and draw X cards.",
        CardBehavior::SphinxsRevelation,
        "Implemented by the named card-local special behavior.",
    )),
);

// RTR 201 — Supreme Verdict
pub(in crate::card::sets) static SUPREME_VERDICT: CardRecord = CardRecord::new(
    cards::SUPREME_VERDICT,
    "Supreme Verdict",
    CardArt::new("4e9648f9-7a67-4717-bca1-861d1f7fed43", "Sam Burley"),
    CardSet::ReturnToRavnica,
    CardRules::new_sorcery(mana_cost!("{1}{W}{W}{U}")).with_abilities(&[
        abilities::cannot_be_countered(),
        AbilityDef::spell(
            "Destroy all creatures.",
            EffectDef::Destroy {
                object: EffectRecipientDef::MatchingObjects {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: PlayerRelation::Any,
                },
                can_regenerate: true,
            },
        ),
    ]),
);

/// The delayed trigger Vraska's +1 hangs on herself. It reads damage arriving
/// at the planeswalker, which only became reachable once a creature could
/// attack one.
static VRASKA_RETALIATION: AbilityDef = AbilityDef::triggered(
    "Whenever a creature deals combat damage to Vraska, destroy that creature.",
    TriggerEventDef::CombatDamageDealtToSource {
        source: ObjectPredicateDef::HasType(CardType::Creature),
    },
    EffectDef::Destroy {
        object: EffectRecipientDef::TriggeringObject,
        can_regenerate: true,
    },
);

static VRASKA_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::activated(
        "+1: Until your next turn, whenever a creature deals combat damage to Vraska, destroy that creature.",
        &[AbilityCostDef::Loyalty(1)],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::GrantAbility(&VRASKA_RETALIATION),
            duration: EffectDurationDef::UntilYourNextTurn,
        },
    ),
    AbilityDef::activated_with_targets(
        "−3: Destroy target nonland permanent.",
        &[AbilityCostDef::Loyalty(-3)],
        &VRASKA_DESTROY_TARGET,
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            can_regenerate: true,
        },
    ),
    AbilityDef::activated(
        "−7: Create three 1/1 black Assassin creature tokens with \"Whenever this token deals combat damage to a player, that player loses the game.\"",
        &[AbilityCostDef::Loyalty(-7)],
        EffectDef::CreateToken {
            token: cards::ASSASSIN_TOKEN_1_1_BLACK,
            count: ValueDef::Constant(3),
        },
    ),
];

static VRASKA_DESTROY_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
)];
// RTR 208 — Vraska the Unseen
pub(in crate::card::sets) static VRASKA_THE_UNSEEN: CardRecord = CardRecord::new(
    cards::VRASKA_THE_UNSEEN,
    "Vraska the Unseen",
    CardArt::new("8971938c-cd26-4b83-96d7-1408cd0b0de6", "Aleksi Briclot"),
    CardSet::ReturnToRavnica,
    CardRules::new_planeswalker(mana_cost!("{3}{B}{G}"), &["Vraska"], 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&VRASKA_ABILITIES),
);

// RTR 231 — Pithing Needle
pub(in crate::card::sets) static PITHING_NEEDLE: CardRecord = CardRecord::new(
    cards::PITHING_NEEDLE,
    "Pithing Needle",
    CardArt::new("786c1e91-9d75-46a3-9e0d-56d29fcb01a7", "Anthony Palumbo"),
    CardSet::ReturnToRavnica,
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::replacement(
            "As this artifact enters, choose a card name.",
            EffectDef::ChooseCardName {
                object: EffectRecipientDef::Source,
            },
        ),
        // The named card's abilities are locked by the action generator, the
        // same place every other activation restriction is enforced.
        AbilityDef::static_ability(
            "Activated abilities of sources with the chosen name can't be activated unless they're mana abilities.",
            EffectDef::None,
        )
        .with_coverage(AbilityCoverageDef::explained_complete(
            "The activation lock is enforced where activated abilities are offered.",
        )),
    ]),
);

// RTR 239 — Golgari Guildgate
pub(in crate::card::sets) static GOLGARI_GUILDGATE: CardRecord = CardRecord::new(
    cards::GOLGARI_GUILDGATE,
    "Golgari Guildgate",
    CardArt::new("8fe2fd1a-f7d3-48b4-bad8-be5ee45d6121", "Eytan Zana"),
    CardSet::ReturnToRavnica,
    CardRules::new_land(&["Gate"]).with_abilities(&[
        abilities::enters_tapped("This land enters tapped."),
        AbilityDef::activated_mana(
            "{T}: Add {B} or {G}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Black,
                ManaColor::Green,
            ])),
        ),
    ]),
);

// RTR 241 — Hallowed Fountain
pub(in crate::card::sets) static HALLOWED_FOUNTAIN: CardRecord = CardRecord::new(
    cards::HALLOWED_FOUNTAIN,
    "Hallowed Fountain",
    CardArt::new("af7091c9-5f98-4078-a42b-c9e057346d9b", "Jung Park"),
    CardSet::ReturnToRavnica,
    CardRules::new_land(&["Plains", "Island"]).with_ability(abilities::shock_land_enters()),
);

// RTR 243 — Overgrown Tomb
pub(in crate::card::sets) static OVERGROWN_TOMB: CardRecord = CardRecord::new(
    cards::OVERGROWN_TOMB,
    "Overgrown Tomb",
    CardArt::new("1c7d50d6-b63a-4d8c-88fa-1d78ae693a45", "Steven Belledin"),
    CardSet::ReturnToRavnica,
    CardRules::new_land(&["Swamp", "Forest"]).with_ability(abilities::shock_land_enters()),
);

// RTR 247 — Steam Vents
pub(in crate::card::sets) static STEAM_VENTS: CardRecord = CardRecord::new(
    cards::STEAM_VENTS,
    "Steam Vents",
    CardArt::new("de911c88-f5c8-4955-9fa5-1f28a9b17236", "Yeong-Hao Han"),
    CardSet::ReturnToRavnica,
    CardRules::new_land(&["Island", "Mountain"]).with_ability(abilities::shock_land_enters()),
);

// RTR 248 — Temple Garden
pub(in crate::card::sets) static TEMPLE_GARDEN: CardRecord = CardRecord::new(
    cards::TEMPLE_GARDEN,
    "Temple Garden",
    CardArt::new("b821e604-f9fd-47a4-b5ff-bfb5022834c2", "Volkan Baǵa"),
    CardSet::ReturnToRavnica,
    CardRules::new_land(&["Forest", "Plains"]).with_ability(abilities::shock_land_enters()),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ANGEL_OF_SERENITY,
    &REST_IN_PEACE,
    &DISPEL,
    &JACE_ARCHITECT_OF_THOUGHT,
    &SYNCOPATE,
    &DESECRATION_DEMON,
    &ULTIMATE_PRICE,
    &UNDERWORLD_CONNECTIONS,
    &MIZZIUM_MORTARS,
    &ABRUPT_DECAY,
    &AZORIUS_CHARM,
    &COUNTERFLUX,
    &DETENTION_SPHERE,
    &GRISLY_SALVAGE,
    &IZZET_CHARM,
    &IZZET_STATICASTER,
    &LOXODON_SMITER,
    &SELESNYA_CHARM,
    &SPHINXS_REVELATION,
    &SUPREME_VERDICT,
    &VRASKA_THE_UNSEEN,
    &PITHING_NEEDLE,
    &GOLGARI_GUILDGATE,
    &HALLOWED_FOUNTAIN,
    &OVERGROWN_TOMB,
    &STEAM_VENTS,
    &TEMPLE_GARDEN,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
