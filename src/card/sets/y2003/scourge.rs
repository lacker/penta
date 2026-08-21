//! Scourge cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, ActivationTimingDef,
    AppliedEffectDef, BasicLandType, CardArt, CardRules, CardSet, CardType, ComparisonDef,
    CounterKind, EffectDef, EffectPaymentCostDef, EffectPaymentDef, EffectRecipientDef, ManaColor,
    ObjectPredicateDef, PayOrDef, PlayerRefDef, PlayerRelation, PlayerSetDef,
    ResolvedEffectDurationDef, StackTargetKindDef, TriggerConditionDef, TriggerEventDef, ValueDef,
    ZoneKind, ZonePlacement, abilities, cards,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

static GOBLIN_SPELLS: ObjectPredicateDef = ObjectPredicateDef::Subtype("Goblin");

/// The cycling half: X is settled by the payment rather than by a cast, so
/// the branch that makes the tokens reads back what was actually paid.
static DECREE_SOLDIERS: EffectDef =
    EffectDef::create_creature_token(&["Soldier"], &[ManaColor::White], 1, 1)
        .with_count(ValueDef::PaidAmount)
        .with_art(CardArt::new(
            "70205fb6-7722-4974-a8c6-8909dbb1c96d",
            "Bachzim",
        ));

static DECREE_CYCLING_TRIGGER: EffectDef = EffectDef::PayOr(PayOrDef::optional(
    EffectPaymentDef {
        payer: PlayerSetDef::Related(PlayerRelation::You),
        cost: EffectPaymentCostDef::ChosenGenericMana,
    },
    &DECREE_SOLDIERS,
));

// SCG 8 — Decree of Justice
pub(in crate::card::sets) static DECREE_OF_JUSTICE: CardRecord = CardRecord::new(
    cards::DECREE_OF_JUSTICE,
    "Decree of Justice",
    CardArt::new("5e8a7e5c-2a37-4e73-b5c9-b8a4b9d0b6e9", "Adam Rex"),
    CardSet::Scourge,
    // Cast for Angels when the game went long, cycled for Soldiers at the end
    // of an opponent's turn when it did not. Landstill wants the second mode
    // far more often than the first.
    CardRules::new_sorcery(mana_cost!("{X}{X}{2}{W}{W}")).with_abilities(&[
        AbilityDef::spell(
            "Create X 4/4 white Angel creature tokens with flying.",
            EffectDef::create_creature_token(&["Angel"], &[ManaColor::White], 4, 4)
                .with_count(ValueDef::ChosenX)
                .with_abilities(&[abilities::flying()])
                .with_art(CardArt::new(
                    "bb6d0a6a-3007-47fc-a42c-3db311c9c41f",
                    "Magali Villeneuve",
                )),
        ),
        abilities::cycling(
            "Cycling {2}{W} ({2}{W}, Discard this card: Draw a card.)",
            mana_cost!("{2}{W}"),
        ),
        AbilityDef::triggered(
            "When you cycle this card, you may pay {X}. If you do, create X 1/1 white Soldier creature tokens.",
            TriggerEventDef::Cycled,
            DECREE_CYCLING_TRIGGER,
        ),
    ]),
);

// SCG 12 — Eternal Dragon
pub(in crate::card::sets) static ETERNAL_DRAGON: CardRecord = CardRecord::new(
    cards::ETERNAL_DRAGON,
    "Eternal Dragon",
    CardArt::new("0596928c-2b20-4dbb-aa78-3ab6c3ce0d72", "Justin Sweet"),
    CardSet::Scourge,
    // Three cards in one: a land early, a threat late, and a threat again
    // every turn after that. Control decks play it as a one-of because it
    // never runs out.
    CardRules::new_creature(mana_cost!("{5}{W}{W}"), &["Dragon", "Spirit"], 5, 5).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{3}{W}{W}: Return this card from your graveyard to your hand. Activate only during your upkeep.",
            &[AbilityCostDef::Mana(mana_cost!("{3}{W}{W}"))],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Source,
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
                controller: None,
                arrival_effect: None,
                attachment: None,
            },
        )
        .with_source_zones(&[ZoneKind::Graveyard])
        .with_activation_timing(ActivationTimingDef::YourUpkeep),
        abilities::typecycling(
            "Plainscycling {2} ({2}, Discard this card: Search your library for a Plains card, reveal it, put it into your hand, then shuffle.)",
            mana_cost!("{2}"),
            ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Plains]),
        ),
    ]),
);

/// Storm, as one clause: the copies are made when the spell is cast, and each
/// is offered its own target because the printed reminder text says so.
static BRAIN_FREEZE_STORM: EffectDef = EffectDef::CopyResolvingSpell {
    chooser: PlayerRefDef::EffectController,
    count: ValueDef::SpellsCastBeforeThisTurn,
};

static A_PLAYER: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Any),
)];

// SCG 29 — Brain Freeze
pub(in crate::card::sets) static BRAIN_FREEZE: CardRecord = CardRecord::new(
    cards::BRAIN_FREEZE,
    "Brain Freeze",
    CardArt::new("59a43ef5-9f6a-4d3e-8e3f-9b3d8f6c1a2b", "Tim Hildebrandt"),
    CardSet::Scourge,
    // Three cards a copy, and a Stasis deck casting four cheap spells in a
    // turn mills a dozen: the sideboard plan against another control deck.
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target player mills three cards.",
            &A_PLAYER,
            EffectDef::Mill {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
                binding: None,
                then: None,
            },
        ),
        AbilityDef::triggered(
            "Storm (When you cast this spell, copy it for each spell cast before it this turn. You may choose new targets for the copies.)",
            TriggerEventDef::SpellCast(ObjectPredicateDef::Source),
            BRAIN_FREEZE_STORM,
        ),
    ]),
);

/// Counter the spell, mark the enchantment, and go when the third mark
/// lands. The sacrifice is checked in the same resolution rather than as a
/// state trigger, which is what the printed clause says.
static DECREE_OF_SILENCE_ANSWER: EffectDef = EffectDef::Sequence(&[
    EffectDef::Counter {
        object: EffectRecipientDef::TriggeringObject,
        zone: ZoneKind::Graveyard,
    },
    EffectDef::AddCounters {
        object: EffectRecipientDef::Source,
        kind: CounterKind::Depletion,
        amount: ValueDef::Constant(1),
    },
    EffectDef::IfCondition {
        condition: &TriggerConditionDef::SourceCounters {
            kind: CounterKind::Depletion,
            comparison: ComparisonDef::GreaterOrEqual,
            amount: 3,
        },
        then: &EffectDef::Sacrifice {
            object: EffectRecipientDef::Source,
        },
    },
]);

static AN_OPPONENTS_SPELL: ObjectPredicateDef =
    ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent);

// SCG 32 — Decree of Silence
pub(in crate::card::sets) static DECREE_OF_SILENCE: CardRecord = CardRecord::new(
    cards::DECREE_OF_SILENCE,
    "Decree of Silence",
    CardArt::new("064fcd41-176d-460d-8e63-8437cfa9b4b1", "Adam Rex"),
    CardSet::Scourge,
    // Eight mana is not what the deck pays: it cycles this to counter one
    // spell, and Replenish puts it onto the battlefield afterwards.
    CardRules::new_enchantment(mana_cost!("{6}{U}{U}")).with_abilities(&[
        AbilityDef::triggered(
            "Whenever an opponent casts a spell, counter that spell and put a depletion counter on this enchantment. If there are three or more depletion counters on this enchantment, sacrifice it.",
            TriggerEventDef::SpellCast(AN_OPPONENTS_SPELL),
            DECREE_OF_SILENCE_ANSWER,
        ),
        abilities::cycling(
            "Cycling {4}{U}{U} ({4}{U}{U}, Discard this card: Draw a card.)",
            mana_cost!("{4}{U}{U}"),
        ),
        AbilityDef::triggered_with_targets(
            "When you cycle this card, you may counter target spell.",
            TriggerEventDef::Cycled,
            &[AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::Any)],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Counter {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Graveyard,
                },
            },
        ),
    ]),
);

static STIFLE_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::StackObject {
        object: ObjectPredicateDef::Any,
        controller: None,
        kind: StackTargetKindDef::AbilityOnly,
    },
)];

// SCG 52 — Stifle
pub(in crate::card::sets) static STIFLE: CardRecord = CardRecord::new(
    cards::STIFLE,
    "Stifle",
    CardArt::new("b3adbbdc-9747-4745-95f1-fda5617529f2", "Dany Orizio"),
    CardSet::Scourge,
    // One mana that answers a fetchland, a Dreadnought's own drawback, or
    // whatever the opponent built their turn around.
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target activated or triggered ability.",
        &STIFLE_TARGET,
        EffectDef::Counter {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Graveyard,
        },
    )),
);

static DRAGON_BREATH_HASTE: AbilityDef = abilities::haste();

/// Six or more, which the deck reaches by assembling a creature rather than
/// by paying for one: the Ghoul arrives enormous and the Breath comes back
/// attached to give it haste.
static A_BIG_CREATURE_ENTERING: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::Not(&ObjectPredicateDef::ManaValueAtMostValue(
        ValueDef::Constant(5),
    )),
]);

/// Life loss rather than damage: nothing prevents it, nothing watching for
/// damage sees it, and the two life you gain is a flat two however little
/// they had left to lose.
static TENDRILS_DRAINS: EffectDef = EffectDef::Sequence(&[
    EffectDef::LoseLife {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        amount: ValueDef::Constant(2),
    },
    EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(2),
    },
]);

static TENDRILS_STORM: EffectDef = EffectDef::CopyResolvingSpell {
    chooser: PlayerRefDef::EffectController,
    count: ValueDef::SpellsCastBeforeThisTurn,
};

// SCG 75 — Tendrils of Agony
pub(in crate::card::sets) static TENDRILS_OF_AGONY: CardRecord = CardRecord::new(
    cards::TENDRILS_OF_AGONY,
    "Tendrils of Agony",
    CardArt::new("0559352e-95c1-403b-bd8f-d0679717cfa2", "Pete Venters"),
    CardSet::Scourge,
    // Four life is nothing; ten copies of it is the whole game, which is why
    // every ritual in the format is really a Tendrils card.
    CardRules::new_sorcery(mana_cost!("{2}{B}{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target player loses 2 life and you gain 2 life.",
            &A_PLAYER,
            TENDRILS_DRAINS,
        ),
        AbilityDef::triggered(
            "Storm (When you cast this spell, copy it for each spell cast before it this turn. \
             You may choose new targets for the copies.)",
            TriggerEventDef::SpellCast(ObjectPredicateDef::Source),
            TENDRILS_STORM,
        ),
    ]),
);

// SCG 86 — Dragon Breath
pub(in crate::card::sets) static DRAGON_BREATH: CardRecord = CardRecord::new(
    cards::DRAGON_BREATH,
    "Dragon Breath",
    CardArt::new("addf9bde-5caf-4b0d-bdc0-a36c18c12604", "Greg Staples"),
    CardSet::Scourge,
    // Nobody casts it. It is discarded on the way to filling a graveyard and
    // comes back for free the turn something enormous arrives.
    CardRules::new_enchantment(mana_cost!("{1}{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell("Enchant creature", &abilities::ENCHANT_CREATURE_TARGET),
            AbilityDef::static_ability(
                "Enchanted creature has haste.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&DRAGON_BREATH_HASTE),
                },
            ),
            AbilityDef::activated(
                "{R}: Enchanted creature gets +1/+0 until end of turn.",
                &[AbilityCostDef::Mana(mana_cost!("{R}"))],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::triggered(
                "When a creature with mana value 6 or greater enters, you may return this card from your graveyard to the battlefield attached to that creature.",
                TriggerEventDef::zone_changed(
                    A_BIG_CREATURE_ENTERING,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    effect: &EffectDef::ReturnAttached {
                        object: EffectRecipientDef::Source,
                        attach_to: EffectRecipientDef::TriggeringObject,
                    },
                },
            )
            .with_source_zones(&[ZoneKind::Graveyard]),
        ]),
);

// SCG 97 — Goblin Warchief
pub(in crate::card::sets) static GOBLIN_WARCHIEF: CardRecord = CardRecord::new(
    cards::GOBLIN_WARCHIEF,
    "Goblin Warchief",
    CardArt::new(
        "66864a4b-8924-40ef-a337-15b12413a158",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    CardSet::Scourge,
    // The haste is what makes the discount matter: a Goblin cast for one
    // less that also attacks the turn it lands.
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Goblin", "Warrior"], 2, 2).with_abilities(
        &[
            AbilityDef::static_ability(
                "Goblin spells you cast cost {1} less to cast.",
                EffectDef::ReduceMatchingSpellCostBy {
                    spell: GOBLIN_SPELLS,
                    caster: PlayerRelation::You,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::static_ability(
                "Goblins you control have haste.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::Subtype("Goblin"),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::add_ability(&abilities::haste()),
                },
            ),
        ],
    ),
);

// SCG 103 — Siege-Gang Commander
pub(in crate::card::sets) static SIEGE_GANG_COMMANDER: CardRecord = CardRecord::new(
    cards::SIEGE_GANG_COMMANDER,
    "Siege-Gang Commander",
    CardArt::new(
        "92e78cec-aaf9-4fe8-887b-b7e356d63315",
        "Christopher Moeller",
    ),
    CardSet::Scourge,
    // Four bodies for five mana, and the ability turns any of them --
    // including itself -- into two damage anywhere.
    CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Goblin"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "When this creature enters, create three 1/1 red Goblin creature tokens.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::create_creature_token(&["Goblin"], &[ManaColor::Red], 1, 1)
                .with_amount(3)
                .with_art(CardArt::new(
                    "09faad62-42ff-4e37-b8a5-d8e8a0f6d096",
                    "Wizard of Barge",
                )),
        ),
        AbilityDef::activated_with_targets(
            "{1}{R}, Sacrifice a Goblin: This creature deals 2 damage to any target.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{R}")),
                AbilityCostDef::SacrificePermanent {
                    object: ObjectPredicateDef::Subtype("Goblin"),
                    controller: PlayerRelation::You,
                },
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &DECREE_OF_JUSTICE,
    &ETERNAL_DRAGON,
    &BRAIN_FREEZE,
    &DECREE_OF_SILENCE,
    &STIFLE,
    &TENDRILS_OF_AGONY,
    &DRAGON_BREATH,
    &GOBLIN_WARCHIEF,
    &SIEGE_GANG_COMMANDER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
