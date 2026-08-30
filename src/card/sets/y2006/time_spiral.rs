//! TSP card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    CardArt, CardRules, CardSet, CardSupertype, CardType, CardTypeSet, CounterKind, CounterKindDef,
    CounterOperationDef, EffectChoiceDef, EffectDef, EffectRecipientDef, ObjectPredicateDef,
    PlayerRelation, PregameConditionDef, PrintedManaCost, TokenCountersDef, TriggerConditionDef,
    TriggerEventDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::{TargetIndex, mana_cost};

// TSP 29 — Momentary Blink
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MOMENTARY_BLINK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("032e072a-0630-472b-9106-5df554dff785"),
    "Momentary Blink",
    crate::card::CardArt::new("032e072a-0630-472b-9106-5df554dff785", "Anthony S. Waters"),
    crate::card::CardSet::TimeSpiral,
    crate::card::CardRules::unsupported(),
);

// TSP 40 — Serra Avenger
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SERRA_AVENGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9e9d7c1c-3bfd-4705-9bc2-5ca3f84cc32a"),
    "Serra Avenger",
    crate::card::CardArt::new("9e9d7c1c-3bfd-4705-9bc2-5ca3f84cc32a", "Scott M. Fischer"),
    crate::card::CardSet::TimeSpiral,
    crate::card::CardRules::unsupported(),
);

// TSP 48 — Ancestral Vision
pub(in crate::card::sets) static ANCESTRAL_VISION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bccedc4d-38c7-4bf3-9ca7-4febd6c49d3d"),
    "Ancestral Vision",
    CardArt::new("bccedc4d-38c7-4bf3-9ca7-4febd6c49d3d", "Mark Poole"),
    CardSet::TimeSpiral,
    CardRules::base(
        CardTypeSet::single(CardType::Sorcery),
        PrintedManaCost::None,
    )
    .with_abilities(&[
        abilities::suspend("Suspend 4—{U}", 4, &mana_cost!("{U}")),
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

// TSP 53 — Clockspinning
static CLOCKSPINNING_TARGET_ALTERNATIVES: [AbilityTargetPredicate; 2] = [
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
];
static CLOCKSPINNING_TARGET: AbilityTargetDef = AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::AnyOf(&CLOCKSPINNING_TARGET_ALTERNATIVES),
);
static CLOCKSPINNING_COUNTER_CHOICES: [EffectChoiceDef; 2] = [
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
];
static CLOCKSPINNING_ADJUST_COUNTER: EffectDef = EffectDef::ChooseEffect {
    player: EffectRecipientDef::Controller,
    choices: &CLOCKSPINNING_COUNTER_CHOICES,
};
pub(in crate::card::sets) static CLOCKSPINNING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1323d548-e2fe-47c5-8df3-f181aed537c5"),
    "Clockspinning",
    CardArt::new("1323d548-e2fe-47c5-8df3-f181aed537c5", "Zoltan Boros & Gabor Szikszai"),
    CardSet::TimeSpiral,
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[
        abilities::buyback(mana_cost!("{3}")),
        AbilityDef::spell_with_targets(
            "Choose a counter on target permanent or suspended card. Remove that counter from that permanent or card or put another of those counters on it.",
            &[CLOCKSPINNING_TARGET],
            EffectDef::ChooseCounterKind {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: &CLOCKSPINNING_ADJUST_COUNTER,
            },
        ),
    ]),
);

// TSP 56 — Deep-Sea Kraken
pub(in crate::card::sets) static DEEP_SEA_KRAKEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8e050532-e245-4eea-90a5-03e3e410dcbe"),
    "Deep-Sea Kraken",
    CardArt::new("8e050532-e245-4eea-90a5-03e3e410dcbe", "Christopher Moeller"),
    CardSet::TimeSpiral,
    CardRules::new_creature(mana_cost!("{7}{U}{U}{U}"), &["Kraken"], 6, 6).with_abilities(&[
        abilities::cannot_be_blocked("This creature can't be blocked."),
        abilities::suspend("Suspend 9—{2}{U}", 9, &mana_cost!("{2}{U}")),
        AbilityDef::triggered_if(
            "Whenever an opponent casts a spell, if this card is suspended, remove a time counter from it.",
            TriggerEventDef::SpellCast(ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent)),
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
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LOOTER_IL_KOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("368ee06f-9021-4b65-9f53-9c326bf3a27f"),
    "Looter il-Kor",
    crate::card::CardArt::new("368ee06f-9021-4b65-9f53-9c326bf3a27f", "Mike Dringenberg"),
    crate::card::CardSet::TimeSpiral,
    crate::card::CardRules::unsupported(),
);

// TSP 104 — Dread Return
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DREAD_RETURN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d7e304fc-0ace-459e-8d2f-376f1899639c"),
    "Dread Return",
    crate::card::CardArt::new("d7e304fc-0ace-459e-8d2f-376f1899639c", "Kev Walker"),
    crate::card::CardSet::TimeSpiral,
    crate::card::CardRules::unsupported(),
);

// TSP 161 — Greater Gargadon
static GARGADON_SACRIFICE: ObjectPredicateDef = ObjectPredicateDef::AnyOf(&[
    ObjectPredicateDef::HasType(CardType::Artifact),
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::HasType(CardType::Land),
]);
pub(in crate::card::sets) static GREATER_GARGADON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("653ddfa0-2088-4503-a3ab-b0f1d55d8351"),
    "Greater Gargadon",
    CardArt::new("653ddfa0-2088-4503-a3ab-b0f1d55d8351", "Rob Alexander"),
    CardSet::TimeSpiral,
    CardRules::new_creature(mana_cost!("{9}{R}"), &["Beast"], 9, 7).with_abilities(&[
        abilities::suspend("Suspend 10—{R}", 10, &mana_cost!("{R}")),
        AbilityDef::activated(
            "Sacrifice an artifact, creature, or land: Remove a time counter from this card. Activate only if this card is suspended.",
            &[AbilityCostDef::SacrificePermanent {
                object: GARGADON_SACRIFICE,
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

// TSP 176 — Rift Bolt
pub(in crate::card::sets) static RIFT_BOLT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("88dde96e-6824-4d26-9fb5-86b9f3c50959"),
    "Rift Bolt",
    CardArt::new("88dde96e-6824-4d26-9fb5-86b9f3c50959", "Michael Sutfin"),
    CardSet::TimeSpiral,
    CardRules::new_sorcery(mana_cost!("{2}{R}")).with_abilities(&[
        abilities::suspend("Suspend 1—{R}", 1, &mana_cost!("{R}")),
        AbilityDef::spell_with_targets(
            "Rift Bolt deals 3 damage to any target.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
            },
        ),
    ]),
);

// TSP 180 — Sulfurous Blast
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SULFUROUS_BLAST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("67511e0e-be09-4f4e-9949-b9ecbdc7f536"),
    "Sulfurous Blast",
    crate::card::CardArt::new("67511e0e-be09-4f4e-9949-b9ecbdc7f536", "Jeff Miracola"),
    crate::card::CardSet::TimeSpiral,
    crate::card::CardRules::unsupported(),
);

// TSP 193 — Durkwood Baloth
pub(in crate::card::sets) static DURKWOOD_BALOTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("670521c3-df02-487d-a299-49419e41889f"),
    "Durkwood Baloth",
    CardArt::new("670521c3-df02-487d-a299-49419e41889f", "Dan Frazier"),
    CardSet::TimeSpiral,
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Beast"], 5, 5)
        .with_ability(abilities::suspend("Suspend 5—{G}", 5, &mana_cost!("{G}"))),
);

// TSP 251 — Chromatic Star
static STAR_COST: [AbilityCostDef; 3] = [
    AbilityCostDef::Mana(mana_cost!("{1}")),
    AbilityCostDef::TapSource,
    AbilityCostDef::SacrificeSource,
];

/// The draw is a separate trigger rather than part of the mana ability,
/// which is the whole difference from Chromatic Sphere: the mana arrives at
/// once and the card waits on the stack, so anything that answers the Star
/// after it has been sacrificed is already too late.
static STAR_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::activated_mana(
        "{1}, {T}, Sacrifice this artifact: Add one mana of any color.",
        &STAR_COST,
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
];

pub(in crate::card::sets) static CHROMATIC_STAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1d7a1357-debd-49b0-9fd5-560d5b3f589e"),
    "Chromatic Star",
    CardArt::new(
        "1d7a1357-debd-49b0-9fd5-560d5b3f589e",
        "Alex Horley-Orlandelli",
    ),
    CardSet::TimeSpiral,
    // A card that fixes one mana and replaces itself, and does the second
    // half however it dies rather than only when it is spent.
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&STAR_ABILITIES),
);

// TSP 257 — Jhoira's Timebug
static TIMEBUG_TARGET_ALTERNATIVES: [AbilityTargetPredicate; 2] = [
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
];
static TIMEBUG_TARGET: AbilityTargetDef =
    AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyOf(&TIMEBUG_TARGET_ALTERNATIVES));
static TIMEBUG_TIME_COUNTER: TriggerConditionDef = TriggerConditionDef::TargetMatches {
    slot: TargetIndex::PRIMARY,
    object: ObjectPredicateDef::HasCounter(CounterKind::named("time")),
};
static TIMEBUG_COUNTER_CHOICES: [EffectChoiceDef; 3] = [
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
];
static TIMEBUG_CHOOSE_ADJUSTMENT: EffectDef = EffectDef::ChooseEffect {
    player: EffectRecipientDef::Controller,
    choices: &TIMEBUG_COUNTER_CHOICES,
};
static TIMEBUG_ADJUST_TIME: EffectDef = EffectDef::IfCondition {
    condition: &TIMEBUG_TIME_COUNTER,
    then: &TIMEBUG_CHOOSE_ADJUSTMENT,
};
pub(in crate::card::sets) static JHOIRAS_TIMEBUG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9ce2c6d7-505b-490b-9c6f-b5166c9ff71d"),
    "Jhoira's Timebug",
    CardArt::new("9ce2c6d7-505b-490b-9c6f-b5166c9ff71d", "Dan Frazier"),
    CardSet::TimeSpiral,
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Insect"], 1, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Choose target permanent you control or suspended card you own. If it has a time counter on it, you may remove a time counter from it or put another time counter on it.",
            &[AbilityCostDef::TapSource],
            &[TIMEBUG_TARGET],
            TIMEBUG_ADJUST_TIME,
        ),
    ),
);

// TSP 264 — Stuffy Doll
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static STUFFY_DOLL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("14ca7425-a499-4864-b955-369ef2577849"),
    "Stuffy Doll",
    crate::card::CardArt::new("14ca7425-a499-4864-b955-369ef2577849", "Dave Allsop"),
    crate::card::CardSet::TimeSpiral,
    crate::card::CardRules::unsupported(),
);

static GEMSTONE_CAVERNS_OPENING_COST: [AbilityCostDef; 1] =
    [AbilityCostDef::ExileCardFromHand(ObjectPredicateDef::Any)];

// TSP 274 — Gemstone Caverns
// Audit: partial — The opening-hand action is declarative; its conditional mana replacement needs a mana ability that branches on a luck counter.
pub(in crate::card::sets) static GEMSTONE_CAVERNS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("94d74254-4750-4fb3-9e53-473a5f98b315"),
    "Gemstone Caverns",
    CardArt::new("94d74254-4750-4fb3-9e53-473a5f98b315", "Martina Pilcerova"),
    CardSet::TimeSpiral,
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::opening_hand_with(
                "If this card is in your opening hand and you're not the starting player, you may begin the game with Gemstone Caverns on the battlefield with a luck counter on it. If you do, exile a card from your hand.",
                PregameConditionDef::NotStartingPlayer,
                &GEMSTONE_CAVERNS_OPENING_COST,
                EffectDef::WithBattlefieldArrival {
                    effect: &EffectDef::MoveToZone {
                        object: EffectRecipientDef::Source,
                        zone: ZoneKind::Battlefield,
                        placement: ZonePlacement::Top,
                    },
                    arrival: crate::card::BattlefieldArrivalDef {
                        counters: Some(TokenCountersDef {
                            kind: CounterKind::Luck,
                            amount: ValueDef::Constant(1),
                        }),
                        ..crate::card::BattlefieldArrivalDef::DEFAULT
                    },
                },
            ),
            AbilityDef::not_implemented(
                "{T}: Add {C}. If Gemstone Caverns has a luck counter on it, instead add one mana of any color.",
                "Needs a conditional activated-mana result keyed to a counter on its source.",
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &MOMENTARY_BLINK,
    &SERRA_AVENGER,
    &ANCESTRAL_VISION,
    &CLOCKSPINNING,
    &DEEP_SEA_KRAKEN,
    &LOOTER_IL_KOR,
    &DREAD_RETURN,
    &GREATER_GARGADON,
    &RIFT_BOLT,
    &SULFUROUS_BLAST,
    &DURKWOOD_BALOTH,
    &CHROMATIC_STAR,
    &JHOIRAS_TIMEBUG,
    &STUFFY_DOLL,
    &GEMSTONE_CAVERNS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
