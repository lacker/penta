//! Innistrad card records used by the built-in ISD–M14 Standard deck tranche.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::sets::y2007::lorwyn as catalog_lrw;
use crate::card::sets::{y1993::alpha, y2002::onslaught, y2009::zendikar};
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityPolicyHint, AbilityPredicateDef,
    AbilityTargetDef, AbilityTargetPredicate, ActivationTimingDef, AddManaEffectDef,
    AppliedEffectDef, AppliedRuleDef, ArrivalAttachmentDef, BasicLandType,
    BattlefieldEntryModificationDef, CardAbilityBinding, CardArt, CardBehavior,
    CardChoiceSourceDef, CardRules, CardSet, CardSupertype, CardType, ComparisonDef,
    ConditionalValueDef, ControlDurationDef, CopyExceptionsDef, CostModificationDef, CounterKind,
    DamageEventMatcherDef, DestroyFollowUpDef, DiscardSelectionDef, EffectDef, EffectExecutionDef,
    EffectPaymentDef, EffectRecipientDef, GraveyardPlayPermissionDef, HalvedValueDef,
    KeywordAbility, ManaColor, MillUntilDef, ObjectPredicateDef, ObjectQueryDef, ObjectRefDef,
    ObjectSetDef, PayOrDef, PlayActionMatcherDef, PlayRestrictionDef, PlayerRefDef, PlayerRelation,
    PlayerSetDef, QuantifierDef, ReplacementConditionDef, ReplacementEffectDef,
    ResolvedEffectDurationDef, RoundingDef, SacrificedAmountDef, SimultaneousChooseDef,
    SpellAdditionalCostCountDef, SpellAdditionalCostDef, SpendModeDef, TargetChooserDef,
    TargetConditionDef, TopCardSelectionDef, TriggerConditionDef, TriggerEventDef, TurnStepDef,
    ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::game::{
    CardAbilityResolver, CardRuntime, PileChoice, PileChosen, PileSplit, PilesSeparated,
    ResolvedAbility,
};
use crate::ids::{
    AbilityId, AdditionalCostObjectIndex, CardPartId, ObjectBindingIndex, ObjectSetBindingIndex,
    TargetIndex, TargetSlotId,
};
use crate::mana_cost;

static CREATURES_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

/// "As an additional cost to cast this spell, exile a creature card from your
/// graveyard."
static EXILE_A_CREATURE_CARD: SpellAdditionalCostDef = SpellAdditionalCostDef {
    or_life: None,
    object: ObjectPredicateDef::HasType(CardType::Creature),
    zone: ZoneKind::Graveyard,
    count: 1,
    counted: SpellAdditionalCostCountDef::Printed,
    spend: SpendModeDef::ByZone,
    or: None,
};

static SACRIFICE_A_CREATURE: SpellAdditionalCostDef = SpellAdditionalCostDef {
    or_life: None,
    object: ObjectPredicateDef::HasType(CardType::Creature),
    zone: ZoneKind::Battlefield,
    count: 1,
    counted: SpellAdditionalCostCountDef::Printed,
    spend: SpendModeDef::ByZone,
    or: None,
};

static ISD_MORBID_A_CREATURE_DIED: TriggerConditionDef = TriggerConditionDef::CreatureDiedThisTurn;

static CREATURE_CARDS_IN_YOUR_GRAVEYARD: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Graveyard],
    PlayerRelation::You,
);

/// The common continuation after this set randomly selects a card from its
/// controller's graveyard. The selection binds the old graveyard object; the
/// move creates and follows its hand-zone successor.
static RETURN_RANDOM_GRAVEYARD_CARD_TO_HAND: EffectDef = EffectDef::MoveToZone {
    object: EffectRecipientDef::objects(ObjectSetDef::Binding(ObjectSetBindingIndex::PRIMARY)),
    zone: ZoneKind::Hand,
    controller: None,
    placement: ZonePlacement::Top,
    arrival_effect: None,
    attachment: None,
    counters: None,
    tapped: false,
};

static NO_SPELLS_LAST_TURN: TriggerConditionDef = TriggerConditionDef::SpellsCastLastTurn {
    quantifier: QuantifierDef::Every,
    player: PlayerRelation::Any,
    comparison: ComparisonDef::LessOrEqual,
    amount: 0,
};

static TWO_SPELLS_LAST_TURN: TriggerConditionDef = TriggerConditionDef::SpellsCastLastTurn {
    quantifier: QuantifierDef::Any,
    player: PlayerRelation::Any,
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 2,
};

static WEREWOLF_FRONT_TRANSFORM: AbilityDef = AbilityDef::triggered_if(
    "At the beginning of each upkeep, if no spells were cast last turn, transform this creature.",
    TriggerEventDef::StepBegins {
        step: crate::card::TurnStepDef::Upkeep,
        player: PlayerRelation::Any,
    },
    &NO_SPELLS_LAST_TURN,
    EffectDef::Transform {
        object: EffectRecipientDef::Source,
    },
);

static WEREWOLF_BACK_TRANSFORM: AbilityDef = AbilityDef::triggered_if(
    "At the beginning of each upkeep, if a player cast two or more spells last turn, transform this creature.",
    TriggerEventDef::StepBegins {
        step: crate::card::TurnStepDef::Upkeep,
        player: PlayerRelation::Any,
    },
    &TWO_SPELLS_LAST_TURN,
    EffectDef::Transform {
        object: EffectRecipientDef::Source,
    },
);

/// Morbid's entry bonus, originating in Innistrad and reused by Dark
/// Ascension. The condition is checked as the creature enters, so a creature
/// dying in response to the spell still counts.
pub(in crate::card::sets) const fn morbid_entry_counters(
    text: &'static str,
    amount: u16,
) -> AbilityDef {
    AbilityDef::as_enters_if(
        text,
        ReplacementConditionDef::CreatureDiedThisTurn,
        ReplacementEffectDef::ModifyBattlefieldEntry(
            BattlefieldEntryModificationDef::AddCounters {
                kind: CounterKind::PlusOnePlusOne,
                amount,
            },
        ),
    )
}

static MORBID_TWO_COUNTERS: AbilityDef = morbid_entry_counters(
    "Morbid — This creature enters with two +1/+1 counters on it if a creature died this turn.",
    2,
);

static ATTACHED_PERMANENT_IS_HUMAN: TriggerConditionDef =
    TriggerConditionDef::AttachedPermanentMatches {
        object: ObjectPredicateDef::Subtype("Human"),
    };

// ISD 1 — Abbey Griffin
pub(in crate::card::sets) static ABBEY_GRIFFIN: CardRecord = CardRecord::new_with_legacy_id(
    843,
    "Abbey Griffin",
    CardArt::new("bf87803b-e7c6-4122-add4-72e596167b7e", "Jaime Jones"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Griffin"], 2, 2)
        .with_abilities(&[abilities::flying(), abilities::vigilance()]),
);

// ISD 2 — Angel of Flight Alabaster
pub(in crate::card::sets) static ANGEL_OF_FLIGHT_ALABASTER: CardRecord = CardRecord::new_with_legacy_id(
    844,
    "Angel of Flight Alabaster",
    CardArt::new("8dfe629f-485c-4619-9713-32d2ae406e63", "Howard Lyon"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Angel"], 4, 4).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "At the beginning of your upkeep, return target Spirit card from your graveyard to your hand.",
            TriggerEventDef::StepBegins {
                step: crate::card::TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Subtype("Spirit"),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            })],
            EffectDef::MoveToZone {
                counters: None,
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                controller: None,
                placement: ZonePlacement::Top,
                arrival_effect: None,
                attachment: None,
                            tapped: false,
},
        ),
    ]),
);

// ISD 3 — Angelic Overseer
static ANGELIC_OVERSEER_CONTROLS_A_HUMAN: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: ObjectQueryDef::matching(
        ObjectPredicateDef::Subtype("Human"),
        &[ZoneKind::Battlefield],
        PlayerRelation::You,
    ),
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 1,
};

static ANGELIC_OVERSEER_HEXPROOF: AbilityDef = abilities::hexproof();
static ANGELIC_OVERSEER_INDESTRUCTIBLE: AbilityDef = abilities::indestructible();

static ANGELIC_OVERSEER_GRANTS: [AppliedEffectDef; 2] = [
    AppliedEffectDef::add_ability(&ANGELIC_OVERSEER_HEXPROOF),
    AppliedEffectDef::add_ability(&ANGELIC_OVERSEER_INDESTRUCTIBLE),
];

static ANGELIC_OVERSEER_BONUS: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::Source,
    effect: AppliedEffectDef::Composite(&ANGELIC_OVERSEER_GRANTS),
};

pub(in crate::card::sets) static ANGELIC_OVERSEER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("221d999c-dde1-4a0f-87cf-9e9f44969f94"),
    "Angelic Overseer",
    CardArt::new("221d999c-dde1-4a0f-87cf-9e9f44969f94", "Jason Chan"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{3}{W}{W}"), &["Angel"], 5, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "As long as you control a Human, this creature has hexproof and indestructible.",
            EffectDef::IfCondition {
                condition: &ANGELIC_OVERSEER_CONTROLS_A_HUMAN,
                then: &ANGELIC_OVERSEER_BONUS,
            },
        ),
    ]),
);

// ISD 4 — Avacynian Priest
pub(in crate::card::sets) static AVACYNIAN_PRIEST: CardRecord = CardRecord::new_with_legacy_id(
    845,
    "Avacynian Priest",
    CardArt::new("08a47828-a79a-4189-9eef-2a5fc5125b61", "Greg Staples"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Cleric"], 1, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{1}, {T}: Tap target non-Human creature.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Human")),
                ]),
            )],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ),
);

// ISD 5 — Bonds of Faith
static BONDS_OF_FAITH_PACIFIED: [AppliedEffectDef; 2] = [
    AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_ATTACK),
    AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
];

static BONDS_OF_FAITH_HUMAN_BOOST: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::AttachedPermanent,
    effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(2), ValueDef::Constant(2)),
};

static ATTACHED_PERMANENT_IS_NOT_HUMAN: TriggerConditionDef =
    TriggerConditionDef::AttachedPermanentMatches {
        object: ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Human")),
    };

static BONDS_OF_FAITH_NON_HUMAN_RESTRICTIONS: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::AttachedPermanent,
    effect: AppliedEffectDef::Composite(&BONDS_OF_FAITH_PACIFIED),
};

static BONDS_OF_FAITH_STATIC_EFFECTS: [EffectDef; 2] = [
    EffectDef::IfCondition {
        condition: &ATTACHED_PERMANENT_IS_HUMAN,
        then: &BONDS_OF_FAITH_HUMAN_BOOST,
    },
    EffectDef::IfCondition {
        condition: &ATTACHED_PERMANENT_IS_NOT_HUMAN,
        then: &BONDS_OF_FAITH_NON_HUMAN_RESTRICTIONS,
    },
];

pub(in crate::card::sets) static BONDS_OF_FAITH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cc8d1ce0-78c5-4e97-9cca-33e7b6ff3440"),
    "Bonds of Faith",
    CardArt::new("cc8d1ce0-78c5-4e97-9cca-33e7b6ff3440", "Steve Argyle"),
    CardSet::Innistrad,
    CardRules::new_enchantment(mana_cost!("{1}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+2 as long as it's a Human. Otherwise, it can't attack or block.",
                EffectDef::Sequence(&BONDS_OF_FAITH_STATIC_EFFECTS),
            ),
        ]),
);

// ISD 6 — Champion of the Parish
pub(in crate::card::sets) static CHAMPION_OF_THE_PARISH: CardRecord =
    CardRecord::new_with_legacy_id(
        846,
        "Champion of the Parish",
        CardArt::new("f7314414-c2d2-48ed-af2c-764cf0207c62", "Svetlin Velinov"),
        CardSet::Innistrad,
        CardRules::new_creature(mana_cost!("{W}"), &["Human", "Soldier"], 1, 1).with_ability(
            AbilityDef::triggered(
                "Whenever another Human you control enters, put a +1/+1 counter on this creature.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Subtype("Human"),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
        ),
    );

// ISD 7 — Chapel Geist
pub(in crate::card::sets) static CHAPEL_GEIST: CardRecord = CardRecord::new_with_legacy_id(
    847,
    "Chapel Geist",
    CardArt::new("790cdf67-80d6-4ade-aecf-f77120b509b0", "Peter Mohrbacher"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Spirit"], 2, 3)
        .with_ability(abilities::flying()),
);

// ISD 8 — Cloistered Youth // Unholy Fiend
const fn cloistered_youth_front_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human"], 1, 1).with_ability(
        AbilityDef::triggered(
            "At the beginning of your upkeep, you may transform this creature.",
            TriggerEventDef::StepBegins {
                step: crate::card::TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Transform {
                    object: EffectRecipientDef::Source,
                },
            },
        ),
    )
}

const fn unholy_fiend_rules() -> CardRules {
    CardRules::new_creature_without_mana_cost(&["Horror"], 3, 3)
        .printed_colors(&[ManaColor::Black])
        .with_ability(AbilityDef::triggered(
            "At the beginning of your end step, you lose 1 life.",
            TriggerEventDef::StepBegins {
                step: crate::card::TurnStepDef::End,
                player: PlayerRelation::You,
            },
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ))
}

pub(in crate::card::sets) static CLOISTERED_YOUTH: CardRecord = CardRecord::new_dfc_with_legacy_id(
    848,
    "Cloistered Youth // Unholy Fiend",
    CardArt::new("f8b8f0b4-71e1-4822-99a1-b1b3c2f10cb2", "Igor Kieryluk"),
    CardSet::Innistrad,
    &[
        ("Cloistered Youth", cloistered_youth_front_rules()),
        ("Unholy Fiend", unholy_fiend_rules()),
    ],
);

static DEARLY_DEPARTED_HUMAN_ENTERS: AbilityDef = AbilityDef::as_enters(
    "This creature enters with an additional +1/+1 counter on it.",
    ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::AddCounters {
        kind: CounterKind::PlusOnePlusOne,
        amount: 1,
    }),
);

// ISD 9 — Dearly Departed
pub(in crate::card::sets) static DEARLY_DEPARTED: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d008061f-cda4-4bcf-b6b3-d1b4a251cc66"),
    "Dearly Departed",
    crate::card::CardArt::new("d008061f-cda4-4bcf-b6b3-d1b4a251cc66", "Daniel Ljunggren"),
    crate::card::CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{4}{W}{W}"), &["Spirit"], 5, 5).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "As long as this card is in your graveyard, each Human creature you control enters with an additional +1/+1 counter on it.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Human"),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&DEARLY_DEPARTED_HUMAN_ENTERS),
            },
        )
        .with_source_zones(&[ZoneKind::Graveyard]),
    ]),
);

// ISD 10 — Divine Reckoning
static DIVINE_RECKONING_DESTROY_REST: EffectDef = EffectDef::Destroy {
    object: EffectRecipientDef::objects(ObjectSetDef::Binding(ObjectSetBindingIndex::new(1))),
    can_regenerate: true,
    then: None,
};

static DIVINE_RECKONING_CHOICE: SimultaneousChooseDef = SimultaneousChooseDef {
    player: EffectRecipientDef::EachPlayer,
    candidates: ObjectPredicateDef::HasType(CardType::Creature),
    one_of_each: &[ObjectPredicateDef::Any],
    chosen: ObjectSetBindingIndex::PRIMARY,
    unchosen: ObjectSetBindingIndex::new(1),
    then: &DIVINE_RECKONING_DESTROY_REST,
};

pub(in crate::card::sets) static DIVINE_RECKONING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("446ea3a4-206a-4097-87c1-c04bb7812972"),
    "Divine Reckoning",
    crate::card::CardArt::new("446ea3a4-206a-4097-87c1-c04bb7812972", "Greg Staples"),
    crate::card::CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{2}{W}{W}")).with_abilities(&[
        AbilityDef::spell(
            "Each player chooses a creature they control. Destroy the rest.",
            EffectDef::SimultaneousChoose(DIVINE_RECKONING_CHOICE),
        ),
        abilities::flashback(mana_cost!("{5}{W}{W}")),
    ]),
);

// ISD 11 — Doomed Traveler
pub(in crate::card::sets) static DOOMED_TRAVELER: CardRecord = CardRecord::new_with_legacy_id(
    849,
    "Doomed Traveler",
    CardArt::new("652c3bbb-cac8-47ad-81de-41e954e17a29", "Lars Grant-West"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Soldier"], 1, 1).with_ability(
        abilities::dies_trigger(
            "When this creature dies, create a 1/1 white Spirit creature token with flying.",
            EffectDef::create_creature_token(&["Spirit"], &[ManaColor::White], 1, 1)
                .with_abilities(&[abilities::flying()])
                .with_art(CardArt::new(
                    "59e79ba0-33c8-46c8-8694-8bf854345fe7",
                    "Ryan Yee",
                )),
        ),
    ),
);

// ISD 12 — Elder Cathar
static ELDER_CATHAR_COUNTERS: ValueDef = ValueDef::IfTargetMatches(&TargetConditionDef {
    slot: TargetIndex::PRIMARY,
    object: ObjectPredicateDef::Subtype("Human"),
    then: ValueDef::Constant(2),
    otherwise: ValueDef::Constant(1),
});

pub(in crate::card::sets) static ELDER_CATHAR: CardRecord = CardRecord::new_with_legacy_id(
    850,
    "Elder Cathar",
    CardArt::new("c21b9e51-fecd-4f9a-9354-a6dc1613feb3", "Chris Rahn"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Soldier"], 2, 2).with_ability(
        abilities::dies_trigger_with_targets("When this creature dies, put a +1/+1 counter on target creature you control. If that creature is a Human, put two +1/+1 counters on it instead.", &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            })], EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ELDER_CATHAR_COUNTERS,
            }),
    ),
);

// ISD 13 — Elite Inquisitor
pub(in crate::card::sets) static ELITE_INQUISITOR: CardRecord = CardRecord::new_with_legacy_id(
    1907,
    "Elite Inquisitor",
    CardArt::new(
        "c9411c44-92a8-4f5d-b3de-d80046649c8c",
        "Jana Schirmer & Johannes Voss",
    ),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{W}{W}"), &["Human", "Soldier"], 2, 2).with_abilities(&[
        abilities::first_strike(),
        abilities::vigilance(),
        AbilityDef::keyword(
            "Protection from Vampires, from Werewolves, and from Zombies",
            KeywordAbility::ProtectionFrom(&ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::Subtype("Vampire"),
                ObjectPredicateDef::Subtype("Werewolf"),
                ObjectPredicateDef::Subtype("Zombie"),
            ])),
        ),
    ]),
);

// ISD 14 — Feeling of Dread
pub(in crate::card::sets) static FEELING_OF_DREAD: CardRecord = CardRecord::new_with_legacy_id(
    851,
    "Feeling of Dread",
    CardArt::new("846a2f9e-ad4f-4666-b152-fdeab7559d86", "John Stanko"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Tap up to two target creatures.",
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                2,
            )],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
        abilities::flashback(mana_cost!("{1}{U}")),
    ]),
);

// ISD 15 — Fiend Hunter
pub(in crate::card::sets) static FIEND_HUNTER: CardRecord = CardRecord::new_with_legacy_id(
    852,
    "Fiend Hunter",
    CardArt::new("f1e4c7d8-11a5-40fe-962b-7e938bf08616", "Wayne Reynolds"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Human", "Cleric"], 1, 3)
        .with_abilities(&[
            abilities::enters_trigger_with_targets("When this creature enters, you may exile another target creature.", &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    1,
                )], EffectDef::ExileLinkedToSource {
                    until_source_leaves: false,
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
face_down: false,
then: None,
}),
            AbilityDef::triggered(
                "When this creature leaves the battlefield, return the exiled card to the battlefield under its owner's control.",
                TriggerEventDef::zone_changed(ObjectPredicateDef::Source, Some(ZoneKind::Battlefield), None),
                EffectDef::ReturnLinkedExiles {
                    object: ObjectPredicateDef::Any,
                    counters: None,
                    arrival_effect: None,
                    zone: ZoneKind::Battlefield,
                    grant: None,
                    controller: None,
                    transformed: false,
                },
            ),
        ]),
);

// ISD 16 — Gallows Warden
pub(in crate::card::sets) static GALLOWS_WARDEN: CardRecord = CardRecord::new_with_legacy_id(
    853,
    "Gallows Warden",
    CardArt::new("15947b20-8c8e-42ed-9599-8b180a382d21", "Dan Murayama Scott"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Spirit"], 3, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "Other Spirit creatures you control get +0/+1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Spirit"),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(1),
                ),
            },
        ),
    ]),
);

// ISD 17 — Geist-Honored Monk
pub(in crate::card::sets) static GEIST_HONORED_MONK: CardRecord = CardRecord::new_with_legacy_id(
    854,
    "Geist-Honored Monk",
    CardArt::new("5d51355e-55fa-43bb-a5de-fc55ac7b6446", "Clint Cearley"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{3}{W}{W}"), &["Human", "Monk"], 0, 0)
        .with_abilities(&[
            abilities::vigilance(),
            AbilityDef::static_ability(
                "Geist-Honored Monk's power and toughness are each equal to the number of creatures you control.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(ValueDef::CountMatchingObjects(&CREATURES_YOU_CONTROL), ValueDef::CountMatchingObjects(&CREATURES_YOU_CONTROL)),
                },
            ),
            abilities::enters_trigger("When this creature enters, create two 1/1 white Spirit creature tokens with flying.", EffectDef::create_creature_token(&["Spirit"], &[ManaColor::White], 1, 1).with_abilities(&[abilities::flying()]).with_art(CardArt::new("59e79ba0-33c8-46c8-8694-8bf854345fe7", "Ryan Yee")).with_amount(2)),
        ]),
);

// ISD 18 — Ghostly Possession
/// The same two-sided shield Gaseous Form wears, with flying alongside it.
/// Prevention names a source or a recipient and never both, so "to and dealt
/// by" is two rules sharing one Apply.
static GHOSTLY_POSSESSION_EFFECT: [AppliedEffectDef; 3] = [
    AppliedEffectDef::add_ability(&abilities::flying()),
    AppliedEffectDef::Rule(AppliedRuleDef::PreventDamage(
        DamageEventMatcherDef::COMBAT_FROM_AFFECTED,
    )),
    AppliedEffectDef::Rule(AppliedRuleDef::PreventDamage(
        DamageEventMatcherDef::COMBAT_TO_AFFECTED,
    )),
];

pub(in crate::card::sets) static GHOSTLY_POSSESSION: CardRecord = CardRecord::new_with_legacy_id(
    1756,
    "Ghostly Possession",
    CardArt::new("a66a41c5-c707-4d6e-a1c6-ee7122413c4c", "Howard Lyon"),
    CardSet::Innistrad,
    CardRules::new_enchantment(mana_cost!("{2}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature has flying. Prevent all combat damage that would be dealt to \
                 and dealt by enchanted creature.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&GHOSTLY_POSSESSION_EFFECT),
                },
            ),
        ]),
);

// ISD 19 — Intangible Virtue
static INTANGIBLE_VIRTUE_VIGILANCE: AbilityDef = abilities::vigilance();

static INTANGIBLE_VIRTUE_GRANT: [AppliedEffectDef; 2] = [
    AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)),
    AppliedEffectDef::add_ability(&INTANGIBLE_VIRTUE_VIGILANCE),
];

pub(in crate::card::sets) static INTANGIBLE_VIRTUE: CardRecord = CardRecord::new_with_legacy_id(
    1886,
    "Intangible Virtue",
    CardArt::new("0dd21f5e-d284-4072-87b9-7f0e6140fe60", "Clint Cearley"),
    CardSet::Innistrad,
    // Creature *tokens*, so a nontoken creature beside them gets nothing --
    // which is what makes this an enchantment for a token deck rather than
    // an anthem.
    CardRules::new_enchantment(mana_cost!("{1}{W}")).with_ability(AbilityDef::static_ability(
        "Creature tokens you control get +1/+1 and have vigilance.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Token,
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::Composite(&INTANGIBLE_VIRTUE_GRANT),
        },
    )),
);

// ISD 20 — Mausoleum Guard
pub(in crate::card::sets) static MAUSOLEUM_GUARD: CardRecord = CardRecord::new_with_legacy_id(
    855,
    "Mausoleum Guard",
    CardArt::new("2c7b19de-96a6-4590-bfc3-31b0c7b2e25e", "David Palumbo"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Scout"], 2, 2).with_ability(
        abilities::dies_trigger(
            "When this creature dies, create two 1/1 white Spirit creature tokens with flying.",
            EffectDef::create_creature_token(&["Spirit"], &[ManaColor::White], 1, 1)
                .with_abilities(&[abilities::flying()])
                .with_art(CardArt::new(
                    "59e79ba0-33c8-46c8-8694-8bf854345fe7",
                    "Ryan Yee",
                ))
                .with_amount(2),
        ),
    ),
);

// ISD 21 — Mentor of the Meek
/// "Power 2 or less" said with the strict comparison the predicates offer:
/// power is an integer, so at most two and below three are the same set.
static MENTOR_OF_THE_MEEK_ARRIVAL: EffectDef = EffectDef::PayOr(PayOrDef::optional(
    EffectPaymentDef::mana(
        PlayerSetDef::Related(PlayerRelation::You),
        mana_cost!("{1}"),
    ),
    &EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    },
));

pub(in crate::card::sets) static MENTOR_OF_THE_MEEK: CardRecord = CardRecord::new_with_legacy_id(
    2015,
    "Mentor of the Meek",
    CardArt::new(
        "bd8f179a-f6ab-4d4c-8195-ed077a7770d3",
        "Jana Schirmer & Johannes Voss",
    ),
    CardSet::Innistrad,
    // A mana a card, as long as what you play stays small -- and it never
    // pays for itself, since "another" excludes the Mentor.
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Soldier"], 2, 2).with_ability(
        AbilityDef::triggered(
            "Whenever another creature you control with power 2 or less enters, you may pay {1}. If you do, draw a card.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ObjectPredicateDef::PowerLessThan(ValueDef::Constant(3)),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            MENTOR_OF_THE_MEEK_ARRIVAL,
        ),
    ),
);

// ISD 22 — Midnight Haunting
pub(in crate::card::sets) static MIDNIGHT_HAUNTING: CardRecord = CardRecord::new_with_legacy_id(
    856,
    "Midnight Haunting",
    CardArt::new("fe1eb098-7128-4ec8-8218-51fdde3e8326", "Matt Stewart"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{2}{W}")).with_ability(AbilityDef::spell(
        "Create two 1/1 white Spirit creature tokens with flying.",
        EffectDef::create_creature_token(&["Spirit"], &[ManaColor::White], 1, 1)
            .with_abilities(&[abilities::flying()])
            .with_art(CardArt::new(
                "59e79ba0-33c8-46c8-8694-8bf854345fe7",
                "Ryan Yee",
            ))
            .with_amount(2),
    )),
);

// ISD 23 — Mikaeus, the Lunarch
pub(in crate::card::sets) static MIKAEUS_THE_LUNARCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("77976ca8-7c7a-469c-a99a-808de1b8bf71"),
    "Mikaeus, the Lunarch",
    crate::card::CardArt::new("c22dc283-ea54-4344-b1ca-fd6cc05080d9", "Steven Belledin"),
    crate::card::CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{X}{W}"), &["Human", "Cleric"], 0, 0)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::as_enters(
                "Mikaeus enters with X +1/+1 counters on it.",
                ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::AddCastXCounters {
                        kind: CounterKind::PlusOnePlusOne,
                    },
                ),
            ),
            AbilityDef::activated(
                "{T}: Put a +1/+1 counter on Mikaeus.",
                &[AbilityCostDef::TapSource],
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::activated(
                "{T}, Remove a +1/+1 counter from Mikaeus: Put a +1/+1 counter on each other creature you control.",
                &[
                    AbilityCostDef::TapSource,
                    AbilityCostDef::RemoveCountersFromSource {
                        kind: CounterKind::PlusOnePlusOne,
                        amount: 1,
                    },
                ],
                EffectDef::AddCounters {
                    object: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// ISD 24 — Moment of Heroism
pub(in crate::card::sets) static MOMENT_OF_HEROISM: CardRecord = CardRecord::new_with_legacy_id(
    857,
    "Moment of Heroism",
    CardArt::new(
        "ba8d15bc-889d-4fd0-9688-00e22db30036",
        "Christopher Moeller",
    ),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +2/+2 and gains lifelink until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::lifelink()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )),
);

// ISD 25 — Nevermore
pub(in crate::card::sets) static NEVERMORE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("67b610fe-36ee-4d58-8ed4-04e7a12587b2"),
    "Nevermore",
    crate::card::CardArt::new("67b610fe-36ee-4d58-8ed4-04e7a12587b2", "Jason A. Engle"),
    crate::card::CardSet::Innistrad,
    CardRules::new_enchantment(mana_cost!("{1}{W}{W}")).with_abilities(&[
        abilities::choose_card_name_as_enters(
            "As this enchantment enters, choose a nonland card name.",
            crate::card::BattlefieldEntryScalarChoiceDef::NONLAND_CARD_NAME,
        ),
        abilities::cannot_cast_spells_with_chosen_name(
            "Spells with the chosen name can't be cast.",
        ),
    ]),
);

// ISD 26 — Paraselene
static PARASELENE_GAIN_LIFE: EffectDef = EffectDef::GainLife {
    recipient: EffectRecipientDef::Controller,
    amount: ValueDef::BoundObjectCount(ObjectSetBindingIndex::PRIMARY),
};

pub(in crate::card::sets) static PARASELENE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("406380ab-2695-4084-99a5-f5560304f8cb"),
    "Paraselene",
    CardArt::new("406380ab-2695-4084-99a5-f5560304f8cb", "Ryan Yee"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{2}{W}")).with_ability(AbilityDef::spell(
        "Destroy all enchantments. You gain 1 life for each enchantment destroyed this way.",
        EffectDef::Destroy {
            object: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Enchantment),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            can_regenerate: true,
            then: Some(DestroyFollowUpDef {
                binding: ObjectSetBindingIndex::PRIMARY,
                effect: &PARASELENE_GAIN_LIFE,
            }),
        },
    )),
);

// ISD 27 — Purify the Grave
pub(in crate::card::sets) static PURIFY_THE_GRAVE: CardRecord = CardRecord::new_with_legacy_id(
    858,
    "Purify the Grave",
    CardArt::new("7cf39365-e468-46ac-bb5b-7f43faa19458", "Drew Baker"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{W}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Exile target card from a graveyard.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::MoveToZone {
                counters: None,
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Exile,
                controller: None,
                placement: ZonePlacement::Top,
                arrival_effect: None,
                attachment: None,
                tapped: false,
            },
        ),
        abilities::flashback(mana_cost!("{W}")),
    ]),
);

// ISD 28 — Rally the Peasants
pub(in crate::card::sets) static RALLY_THE_PEASANTS: CardRecord = CardRecord::new_with_legacy_id(
    859,
    "Rally the Peasants",
    CardArt::new("514fe7de-16b2-42c0-adb1-f0af1c89cfd6", "Jaime Jones"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{2}{W}")).with_abilities(&[
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
        abilities::flashback(mana_cost!("{2}{R}")),
    ]),
);

// ISD 29 — Rebuke
pub(in crate::card::sets) static REBUKE: CardRecord = CardRecord::new_with_legacy_id(
    860,
    "Rebuke",
    CardArt::new("267185ac-a176-423e-a7f8-ee966d1d9a1e", "Igor Kieryluk"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{2}{W}")).with_ability(AbilityDef::destroy_target(
        "Destroy target attacking creature.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Attacking,
        ])),
        true,
    )),
);

// ISD 30 — Selfless Cathar
pub(in crate::card::sets) static SELFLESS_CATHAR: CardRecord = CardRecord::new_with_legacy_id(
    861,
    "Selfless Cathar",
    CardArt::new("5a1dc067-1972-4d46-ad5d-56e6a563f638", "Slawomir Maniak"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Cleric"], 1, 1).with_ability(
        AbilityDef::activated(
            "{1}{W}, Sacrifice this creature: Creatures you control get +1/+1 until end of turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{W}")),
                AbilityCostDef::SacrificeSource,
            ],
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ISD 31 — Silverchase Fox
pub(in crate::card::sets) static SILVERCHASE_FOX: CardRecord = CardRecord::new_with_legacy_id(
    862,
    "Silverchase Fox",
    CardArt::new("0a81bfab-3397-4562-8b82-5f24cef167e3", "Howard Lyon"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Fox"], 2, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{1}{W}, Sacrifice this creature: Exile target enchantment.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{W}")),
                AbilityCostDef::SacrificeSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Enchantment),
            )],
            EffectDef::MoveToZone {
                counters: None,
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Exile,
                controller: None,
                placement: ZonePlacement::Top,
                arrival_effect: None,
                attachment: None,
                tapped: false,
            },
        ),
    ),
);

// ISD 32 — Slayer of the Wicked
pub(in crate::card::sets) static SLAYER_OF_THE_WICKED: CardRecord = CardRecord::new_with_legacy_id(
    863,
    "Slayer of the Wicked",
    CardArt::new("1c2cd68e-ff4c-49c7-ba0d-f2299d9c21f4", "Anthony Palumbo"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Soldier"], 3, 2).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, you may destroy target Vampire, Werewolf, or Zombie.",
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::Subtype("Vampire"),
                        ObjectPredicateDef::Subtype("Werewolf"),
                        ObjectPredicateDef::Subtype("Zombie"),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                1,
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
        ),
    ),
);

// ISD 33 — Smite the Monstrous
pub(in crate::card::sets) static SMITE_THE_MONSTROUS: CardRecord = CardRecord::new_with_legacy_id(
    864,
    "Smite the Monstrous",
    CardArt::new("0103f3b1-88c2-4cbf-a67c-49420f92970f", "Jason Felix"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{3}{W}")).with_ability(AbilityDef::destroy_target(
        "Destroy target creature with power 4 or greater.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::PowerAtLeast(4),
        ])),
        true,
    )),
);

// ISD 34 — Spare from Evil
static SPARE_FROM_EVIL_PROTECTION: AbilityDef = AbilityDef::keyword(
    "Protection from non-Human creatures",
    KeywordAbility::ProtectionFrom(&ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Human")),
    ])),
);

pub(in crate::card::sets) static SPARE_FROM_EVIL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d01b5d97-b5ae-42a7-944a-feb12febd63c"),
    "Spare from Evil",
    CardArt::new("d01b5d97-b5ae-42a7-944a-feb12febd63c", "Jason Felix"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell(
        "Creatures you control gain protection from non-Human creatures until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::add_ability(&SPARE_FROM_EVIL_PROTECTION),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// ISD 35 — Spectral Rider
pub(in crate::card::sets) static SPECTRAL_RIDER: CardRecord = CardRecord::new_with_legacy_id(
    865,
    "Spectral Rider",
    CardArt::new("b47e4e56-8bde-480d-b59c-17a017665b19", "Igor Kieryluk"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{W}{W}"), &["Spirit", "Knight"], 2, 2)
        .with_ability(abilities::intimidate()),
);

// ISD 36 — Stony Silence
pub(in crate::card::sets) static STONY_SILENCE: CardRecord = CardRecord::new_with_legacy_id(
    1967,
    "Stony Silence",
    CardArt::new("f56a5a73-5f10-4f97-989f-7cea0a8d95e3", "Wayne England"),
    CardSet::Innistrad,
    // Every artifact, whoever controls it, and mana abilities included --
    // which is what makes this an answer to a mana engine rather than to one
    // permanent.
    CardRules::new_enchantment(mana_cost!("{1}{W}")).with_ability(AbilityDef::static_ability(
        "Activated abilities of artifacts can't be activated.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Artifact),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotActivateAbilities),
        },
    )),
);

// ISD 37 — Thraben Purebloods
pub(in crate::card::sets) static THRABEN_PUREBLOODS: CardRecord = CardRecord::new_with_legacy_id(
    866,
    "Thraben Purebloods",
    CardArt::new("16db28f4-3d96-42f5-a264-592fdc2d4196", "Martina Pilcerova"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Dog"], 3, 5),
);

// ISD 38 — Thraben Sentry // Thraben Militia
static THRABEN_SENTRY_FRONT_ABILITIES: [AbilityDef; 2] = [
    abilities::vigilance(),
    AbilityDef::triggered(
        "Whenever another creature you control dies, you may transform this creature.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
            ]),
            Some(ZoneKind::Battlefield),
            Some(ZoneKind::Graveyard),
        ),
        EffectDef::May {
            player: EffectRecipientDef::Controller,
            effect: &EffectDef::Transform {
                object: EffectRecipientDef::Source,
            },
        },
    ),
];

const fn thraben_sentry_front_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Soldier"], 2, 2)
        .with_abilities(&THRABEN_SENTRY_FRONT_ABILITIES)
}

const fn thraben_militia_rules() -> CardRules {
    CardRules::new_creature_without_mana_cost(&["Human", "Soldier"], 5, 4)
        .printed_colors(&[ManaColor::White])
        .with_ability(abilities::trample())
}

pub(in crate::card::sets) static THRABEN_SENTRY: CardRecord = CardRecord::new_dfc_with_legacy_id(
    867,
    "Thraben Sentry // Thraben Militia",
    CardArt::new("58ae9cbc-d88d-42df-ab76-63ab5d05c023", "David Rapoza"),
    CardSet::Innistrad,
    &[
        ("Thraben Sentry", thraben_sentry_front_rules()),
        ("Thraben Militia", thraben_militia_rules()),
    ],
);

// ISD 39 — Unruly Mob
pub(in crate::card::sets) static UNRULY_MOB: CardRecord = CardRecord::new_with_legacy_id(
    868,
    "Unruly Mob",
    CardArt::new("491c6e40-151a-4efd-980c-e6b6a1057c58", "Ryan Pancoast"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human"], 1, 1).with_ability(
        AbilityDef::triggered(
            "Whenever another creature you control dies, put a +1/+1 counter on this creature.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// ISD 40 — Urgent Exorcism
pub(in crate::card::sets) static URGENT_EXORCISM: CardRecord = CardRecord::new_with_legacy_id(
    235,
    "Urgent Exorcism",
    CardArt::new("516a437c-a2ee-43c6-876c-1a63a455c97c", "Svetlin Velinov"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Destroy target Spirit or enchantment.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Subtype("Spirit"),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            can_regenerate: true,
            then: None,
        },
    )]),
);

// ISD 41 — Village Bell-Ringer
pub(in crate::card::sets) static VILLAGE_BELL_RINGER: CardRecord = CardRecord::new_with_legacy_id(
    869,
    "Village Bell-Ringer",
    CardArt::new("cb6912b3-bab9-4937-afdd-3711e6d792a0", "David Palumbo"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Scout"], 1, 4).with_abilities(&[
        abilities::flash(),
        abilities::enters_trigger(
            "When this creature enters, untap all creatures you control.",
            EffectDef::Untap {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
            },
        ),
    ]),
);

// ISD 42 — Voiceless Spirit
pub(in crate::card::sets) static VOICELESS_SPIRIT: CardRecord = CardRecord::new_with_legacy_id(
    870,
    "Voiceless Spirit",
    CardArt::new("d24d9bd7-5721-4436-a86f-35e376727f46", "Daarken"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Spirit"], 2, 1)
        .with_abilities(&[abilities::flying(), abilities::first_strike()]),
);

// ISD 43 — Armored Skaab
pub(in crate::card::sets) static ARMORED_SKAAB: CardRecord = CardRecord::new_with_legacy_id(
    871,
    "Armored Skaab",
    CardArt::new("ce4d00f2-30e6-41d5-b997-c66350fe783c", "Volkan Baǵa"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Zombie", "Warrior"], 1, 4).with_ability(
        abilities::enters_trigger(
            "When this creature enters, mill four cards.",
            EffectDef::Mill {
                player: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(4),
                binding: None,
                then: None,
            },
        ),
    ),
);

// ISD 44 — Back from the Brink
pub(in crate::card::sets) static BACK_FROM_THE_BRINK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b4bba140-5c06-4542-9ae0-b2517104ab7c"),
    "Back from the Brink",
    crate::card::CardArt::new("b4bba140-5c06-4542-9ae0-b2517104ab7c", "Anthony Palumbo"),
    crate::card::CardSet::Innistrad,
    CardRules::new_enchantment(mana_cost!("{4}{U}{U}")).with_ability(
        AbilityDef::activated(
            "Exile a creature card from your graveyard and pay its mana cost: Create a token that's a copy of that card. Activate only as a sorcery.",
            &[
                AbilityCostDef::ManaCostOf(ObjectRefDef::Binding(ObjectBindingIndex::PRIMARY)),
                AbilityCostDef::MoveToZone(
                    crate::card::MoveToZoneCostDef::new(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ZoneKind::Graveyard,
                        ZoneKind::Exile,
                        1,
                    )
                    .binding(ObjectBindingIndex::PRIMARY),
                ),
            ],
            EffectDef::create_token_from_copy(&crate::card::TokenCopyDef {
                object: &EffectRecipientDef::binding_zone_change_successor(
                    ObjectBindingIndex::PRIMARY,
                ),
                exceptions: CopyExceptionsDef::NONE,
            }),
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ),
);

// ISD 45 — Battleground Geist
pub(in crate::card::sets) static BATTLEGROUND_GEIST: CardRecord = CardRecord::new_with_legacy_id(
    872,
    "Battleground Geist",
    CardArt::new("129905ef-5b3b-4860-923c-109a7d7cad80", "Clint Cearley"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Spirit"], 3, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "Other Spirit creatures you control get +1/+0.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Spirit"),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(0),
                ),
            },
        ),
    ]),
);

// ISD 46 — Cackling Counterpart
pub(in crate::card::sets) static CACKLING_COUNTERPART: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8a2a2b93-94dc-4285-a6fd-455a796426bc"),
    "Cackling Counterpart",
    crate::card::CardArt::new("8a2a2b93-94dc-4285-a6fd-455a796426bc", "David Rapoza"),
    crate::card::CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{1}{U}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Create a token that's a copy of target creature you control.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::create_token_from_copy(&crate::card::TokenCopyDef {
                object: &EffectRecipientDef::Target(TargetIndex::PRIMARY),
                exceptions: CopyExceptionsDef::NONE,
            }),
        ),
        abilities::flashback(mana_cost!("{5}{U}{U}")),
    ]),
);

// ISD 47 — Civilized Scholar // Homicidal Brute
// Audit: metadata-only — Needs a discard choice linked to a creature-card test, conditional untap, and transform continuation.
pub(in crate::card::sets) static CIVILIZED_SCHOLAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7bf864db-4754-433d-9d77-6695f78f6c09"),
    "Civilized Scholar",
    crate::card::CardArt::new("7bf864db-4754-433d-9d77-6695f78f6c09", "Michael C. Hayes"),
    crate::card::CardSet::Innistrad,
    crate::card::CardRules::unsupported(),
);

// ISD 48 — Claustrophobia
pub(in crate::card::sets) static CLAUSTROPHOBIA: CardRecord = CardRecord::new_with_legacy_id(
    873,
    "Claustrophobia",
    CardArt::new("b7e5f600-4d19-42a4-b57e-650c76041798", "Ryan Pancoast"),
    CardSet::Innistrad,
    CardRules::new_enchantment(mana_cost!("{1}{U}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            abilities::enters_trigger(
                "When this Aura enters, tap enchanted creature.",
                EffectDef::Tap {
                    object: EffectRecipientDef::AttachedPermanent,
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature doesn't untap during its controller's untap step.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
                },
            ),
        ]),
);

// ISD 49 — Curiosity
static CURIOSITY_GRANTED_ABILITY: AbilityDef = AbilityDef::triggered(
    "Whenever this creature deals damage to an opponent, you may draw a card.",
    TriggerEventDef::damage_to_player(ObjectPredicateDef::Source, PlayerRelation::Opponent),
    EffectDef::May {
        player: EffectRecipientDef::Controller,
        effect: &EffectDef::DrawCards {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
        },
    },
);

pub(in crate::card::sets) static CURIOSITY: CardRecord = CardRecord::new_with_legacy_id(
    874,
    "Curiosity",
    CardArt::new("b212c36a-6d1f-4217-b384-1c2b0e07b68a", "Igor Kieryluk"),
    CardSet::Innistrad,
    CardRules::new_enchantment(mana_cost!("{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Whenever enchanted creature deals damage to an opponent, you may draw a card.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&CURIOSITY_GRANTED_ABILITY),
                },
            ),
        ]),
);

// ISD 50 — Curse of the Bloody Tome
pub(in crate::card::sets) static CURSE_OF_THE_BLOODY_TOME: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c7865e11-263b-4d61-af54-907c1acbb54f"),
    "Curse of the Bloody Tome",
    crate::card::CardArt::new("c7865e11-263b-4d61-af54-907c1acbb54f", "Jaime Jones"),
    crate::card::CardSet::Innistrad,
    CardRules::new_enchantment(mana_cost!("{2}{U}"))
        .with_subtypes(&["Aura", "Curse"])
        .with_abilities(&[
            abilities::enchant_player(),
            abilities::enchanted_player_upkeep(
                "At the beginning of enchanted player's upkeep, that player mills two cards.",
                EffectDef::Mill {
                    player: EffectRecipientDef::EnchantedPlayer,
                    amount: ValueDef::Constant(2),
                    binding: None,
                    then: None,
                },
            ),
        ]),
);

// ISD 51 — Delver of Secrets // Insectile Aberration
static DELVER_REVEALED_INSTANT_OR_SORCERY: TriggerConditionDef =
    TriggerConditionDef::ValueComparison(&crate::card::ValueComparisonDef {
        left: ValueDef::MatchedCount,
        comparison: ComparisonDef::GreaterOrEqual,
        right: ValueDef::Constant(1),
    });

static DELVER_TRANSFORM: EffectDef = EffectDef::IfCondition {
    condition: &DELVER_REVEALED_INSTANT_OR_SORCERY,
    then: &EffectDef::Transform {
        object: EffectRecipientDef::Source,
    },
};

static DELVER_LOOK: TopCardSelectionDef = TopCardSelectionDef {
    count: ValueDef::Constant(1),
    object: None,
    minimum: 0,
    maximum: 1,
    select_all_matching: false,
    select_one_of_each_type: false,
    reveal_inspected: false,
    reveal_selected: true,
    counted: Some(ObjectPredicateDef::AnyOf(&[
        ObjectPredicateDef::HasType(CardType::Instant),
        ObjectPredicateDef::HasType(CardType::Sorcery),
    ])),
    selected_zone: ZoneKind::Library,
    selected_placement: ZonePlacement::Top,
    selected_hidden: false,
    selected_linked_to_source: false,
    selected_face_down: None,
    rest_zone: ZoneKind::Library,
    rest_placement: ZonePlacement::Top,
    rest_random_order: false,
    rest_counters: None,
    selected_order_follows_choice: false,
    then: Some(&DELVER_TRANSFORM),
};

const fn delver_of_secrets_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{U}"), &["Human", "Wizard"], 1, 1).with_ability(
        AbilityDef::triggered(
            "At the beginning of your upkeep, look at the top card of your library. You may reveal that card. If an instant or sorcery card is revealed this way, transform this creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::LookAtTopAndSelect {
                player: EffectRecipientDef::Controller,
                looker: EffectRecipientDef::Controller,
                selection: &DELVER_LOOK,
            },
        ),
    )
}

const fn insectile_aberration_rules() -> CardRules {
    CardRules::new_creature_without_mana_cost(&["Human", "Insect"], 3, 2)
        .printed_colors(&[ManaColor::Blue])
        .with_ability(abilities::flying())
}

pub(in crate::card::sets) static DELVER_OF_SECRETS: CardRecord = CardRecord::new_dfc(
    PrintingAnchor::scryfall("11bf83bb-c95b-4b4f-9a56-ce7a1816307a"),
    "Delver of Secrets // Insectile Aberration",
    CardArt::new("11bf83bb-c95b-4b4f-9a56-ce7a1816307a", "Nils Hamm"),
    CardSet::Innistrad,
    &[
        ("Delver of Secrets", delver_of_secrets_rules()),
        ("Insectile Aberration", insectile_aberration_rules()),
    ],
);

// ISD 52 — Deranged Assistant
pub(in crate::card::sets) static DERANGED_ASSISTANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a4c03171-5ff0-4f79-bb03-16decf7d34ce"),
    "Deranged Assistant",
    CardArt::new("a4c03171-5ff0-4f79-bb03-16decf7d34ce", "Nils Hamm"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Human", "Wizard"], 1, 1).with_ability(
        AbilityDef::activated(
            "{T}, Mill a card: Add {C}.",
            &[AbilityCostDef::TapSource, AbilityCostDef::MillCards(1)],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
        ),
    ),
);

// ISD 53 — Dissipate
pub(in crate::card::sets) static DISSIPATE: CardRecord = CardRecord::new_with_legacy_id(
    156,
    "Dissipate",
    CardArt::new("5d778082-bcdb-423a-b16f-57ac0d4dace7", "Tomasz Jedruszek"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{1}{U}{U}")).with_ability(
        AbilityDef::spell_with_targets(
            "Counter target spell. If that spell is countered this way, exile it instead of putting it into its owner's graveyard.",
            &[AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::Any)],
            EffectDef::Counter {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
            },
        ),
    ),
);

// ISD 54 — Dream Twist
pub(in crate::card::sets) static DREAM_TWIST: CardRecord = CardRecord::new_with_legacy_id(
    875,
    "Dream Twist",
    CardArt::new("d5dd8790-bfdf-427d-8e8d-a5c3a64a3063", "Dan Murayama Scott"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target player mills three cards.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Mill {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
                binding: None,
                then: None,
            },
        ),
        abilities::flashback(mana_cost!("{1}{U}")),
    ]),
);

// ISD 55 — Forbidden Alchemy
static FORBIDDEN_ALCHEMY_SELECTION: TopCardSelectionDef = TopCardSelectionDef {
    count: ValueDef::Constant(4),
    object: None,
    minimum: 1,
    maximum: 1,
    select_all_matching: false,
    select_one_of_each_type: false,
    reveal_inspected: false,
    reveal_selected: false,
    counted: None,
    selected_zone: ZoneKind::Hand,
    selected_placement: ZonePlacement::Top,
    rest_zone: ZoneKind::Graveyard,
    rest_placement: ZonePlacement::Top,
    rest_random_order: false,
    rest_counters: None,
    selected_order_follows_choice: false,
    then: None,
    selected_hidden: false,
    selected_linked_to_source: false,
    selected_face_down: None,
};

pub(in crate::card::sets) static FORBIDDEN_ALCHEMY: CardRecord = CardRecord::new_with_legacy_id(
    876,
    "Forbidden Alchemy",
    CardArt::new("eb22ae62-6207-4693-87cf-7adf0fc1fe29", "David Rapoza"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{2}{U}")).with_abilities(&[
        AbilityDef::spell(
            "Look at the top four cards of your library. Put one of them into your hand and the rest into your graveyard.",
            EffectDef::LookAtTopAndSelect {
                player: EffectRecipientDef::Controller,
                looker: EffectRecipientDef::Controller,
                selection: &FORBIDDEN_ALCHEMY_SELECTION,
            },
        ),
        abilities::flashback(mana_cost!("{6}{B}")),
    ]),
);

// ISD 56 — Fortress Crab
pub(in crate::card::sets) static FORTRESS_CRAB: CardRecord = CardRecord::new_with_legacy_id(
    877,
    "Fortress Crab",
    CardArt::new("87ca16d4-089f-42a7-a648-55301a77faea", "Vincent Proce"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Crab"], 1, 6),
);

// ISD 57 — Frightful Delusion
pub(in crate::card::sets) static FRIGHTFUL_DELUSION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("38c9ba98-90b4-4c28-9eef-a4fe0913b921"),
    "Frightful Delusion",
    CardArt::new("38c9ba98-90b4-4c28-9eef-a4fe0913b921", "Anthony Palumbo"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target spell unless its controller pays {1}. That player discards a card.",
        &[AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::Any)],
        EffectDef::Sequence(&[
            abilities::counter_target_unless_paid(ValueDef::Constant(1)),
            EffectDef::Discard {
                recipient: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ]),
    )),
);

// ISD 58 — Grasp of Phantoms
pub(in crate::card::sets) static GRASP_OF_PHANTOMS: CardRecord = CardRecord::new_with_legacy_id(
    878,
    "Grasp of Phantoms",
    CardArt::new("02655d3d-82d0-4be6-bb64-25e1478edfc3", "Izzy"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{3}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Put target creature on top of its owner's library.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::MoveToZone {
                counters: None,
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Library,
                controller: None,
                placement: ZonePlacement::Top,
                arrival_effect: None,
                attachment: None,
                tapped: false,
            },
        ),
        abilities::flashback(mana_cost!("{7}{U}")),
    ]),
);

// ISD 59 — Hysterical Blindness
pub(in crate::card::sets) static HYSTERICAL_BLINDNESS: CardRecord = CardRecord::new_with_legacy_id(
    879,
    "Hysterical Blindness",
    CardArt::new("5aeaa757-e3b0-4606-a689-e8a20a686c3a", "Wayne England"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell(
        "Creatures your opponents control get -4/-0 until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::Opponent,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(-4),
                ValueDef::Constant(0),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// ISD 60 — Invisible Stalker
pub(in crate::card::sets) static INVISIBLE_STALKER: CardRecord = CardRecord::new_with_legacy_id(
    880,
    "Invisible Stalker",
    CardArt::new("0013620d-8e17-4246-86bf-71eafd51b806", "Bud Cook"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Human", "Rogue"], 1, 1).with_abilities(&[
        abilities::hexproof(),
        AbilityDef::static_ability(
            "This creature can't be blocked.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                    ObjectPredicateDef::Any,
                )),
            },
        ),
    ]),
);

// ISD 61 — Laboratory Maniac
pub(in crate::card::sets) static LABORATORY_MANIAC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("809205f3-acf5-4244-b360-09ce4ba76795"),
    "Laboratory Maniac",
    crate::card::CardArt::new("809205f3-acf5-4244-b360-09ce4ba76795", "Jason Felix"),
    crate::card::CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Wizard"], 2, 2)
        .with_ability(abilities::empty_library_draw_wins()),
);

// ISD 62 — Lantern Spirit
pub(in crate::card::sets) static LANTERN_SPIRIT: CardRecord = CardRecord::new_with_legacy_id(
    881,
    "Lantern Spirit",
    CardArt::new("b50a5772-f411-458a-97f9-9f3967bb79c5", "Johann Bodin"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Spirit"], 2, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{U}: Return this creature to its owner's hand.",
            &[AbilityCostDef::Mana(mana_cost!("{U}"))],
            EffectDef::MoveToZone {
                counters: None,
                object: EffectRecipientDef::Source,
                zone: ZoneKind::Hand,
                controller: None,
                placement: ZonePlacement::Top,
                arrival_effect: None,
                attachment: None,
                tapped: false,
            },
        ),
    ]),
);

// ISD 63 — Lost in the Mist
static LOST_IN_THE_MIST_TARGETS: [AbilityTargetDef; 2] = [
    AbilityTargetDef::exactly_one_spell(ObjectPredicateDef::Any),
    AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Any),
];

pub(in crate::card::sets) static LOST_IN_THE_MIST: CardRecord = CardRecord::new_with_legacy_id(
    882,
    "Lost in the Mist",
    CardArt::new("1e5fc39d-590a-436b-ab90-a1741d2ae3da", "David Palumbo"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{3}{U}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Counter target spell. Return target permanent to its owner's hand.",
        &LOST_IN_THE_MIST_TARGETS,
        EffectDef::Sequence(&[
            EffectDef::Counter {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Graveyard,
                placement: ZonePlacement::Top,
            },
            EffectDef::MoveToZone {
                counters: None,
                object: EffectRecipientDef::Target(TargetIndex(1)),
                zone: ZoneKind::Hand,
                controller: None,
                placement: ZonePlacement::Top,
                arrival_effect: None,
                attachment: None,
                tapped: false,
            },
        ]),
    )),
);

// ISD 64 — Ludevic's Test Subject // Ludevic's Abomination
const HATCHLING_COUNTER: CounterKind = CounterKind::named("hatchling");

static LUDEVIC_HAS_FIVE_HATCHLING_COUNTERS: TriggerConditionDef =
    TriggerConditionDef::SourceCounters {
        kind: HATCHLING_COUNTER,
        comparison: ComparisonDef::GreaterOrEqual,
        amount: 5,
    };

static LUDEVIC_HATCHES: EffectDef = EffectDef::Sequence(&[
    EffectDef::RemoveAllCounters {
        object: EffectRecipientDef::Source,
        kind: Some(HATCHLING_COUNTER),
    },
    EffectDef::Transform {
        object: EffectRecipientDef::Source,
    },
]);

static LUDEVIC_HATCHLING_ABILITY: AbilityDef = AbilityDef::activated(
    "{1}{U}: Put a hatchling counter on this creature. Then if there are five or more hatchling counters on it, remove all of them and transform it.",
    &[AbilityCostDef::Mana(mana_cost!("{1}{U}"))],
    EffectDef::Sequence(&[
        EffectDef::AddCounters {
            object: EffectRecipientDef::Source,
            kind: HATCHLING_COUNTER,
            amount: ValueDef::Constant(1),
        },
        EffectDef::IfCondition {
            condition: &LUDEVIC_HAS_FIVE_HATCHLING_COUNTERS,
            then: &LUDEVIC_HATCHES,
        },
    ]),
);

static LUDEVIC_FRONT_ABILITIES: [AbilityDef; 2] =
    [abilities::defender(), LUDEVIC_HATCHLING_ABILITY];

const fn ludevics_test_subject_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Lizard", "Egg"], 0, 3)
        .with_abilities(&LUDEVIC_FRONT_ABILITIES)
}

const fn ludevics_abomination_rules() -> CardRules {
    CardRules::new_creature_without_mana_cost(&["Lizard", "Horror"], 13, 13)
        .printed_colors(&[ManaColor::Blue])
        .with_ability(abilities::trample())
}

pub(in crate::card::sets) static LUDEVIC_S_TEST_SUBJECT: CardRecord = CardRecord::new_dfc(
    PrintingAnchor::scryfall("340b660c-9aa1-4bd2-8c6f-d9af7fee5f4a"),
    "Ludevic's Test Subject // Ludevic's Abomination",
    CardArt::new("ebf5e16f-a8bd-419f-b5ca-8c7fce09c4f1", "Nils Hamm"),
    CardSet::Innistrad,
    &[
        ("Ludevic's Test Subject", ludevics_test_subject_rules()),
        ("Ludevic's Abomination", ludevics_abomination_rules()),
    ],
);

// ISD 65 — Makeshift Mauler
pub(in crate::card::sets) static MAKESHIFT_MAULER: CardRecord = CardRecord::new_with_legacy_id(
    1603,
    "Makeshift Mauler",
    CardArt::new("d869de57-9454-47ff-af14-eaefd387047a", "James Ryman"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Zombie", "Horror"], 4, 5).with_abilities(&[
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, exile a creature card from your \
             graveyard.",
            &[],
            EXILE_A_CREATURE_CARD,
            EffectDef::None,
        ),
    ]),
);

// ISD 66 — Memory's Journey
pub(in crate::card::sets) static MEMORY_S_JOURNEY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("265aaa73-1a1e-4282-a860-f7c422f21db3"),
    "Memory's Journey",
    crate::card::CardArt::new("265aaa73-1a1e-4282-a860-f7c422f21db3", "Slawomir Maniak"),
    crate::card::CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target player shuffles up to three target cards from their graveyard into their library.",
            &[
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
                    PlayerRelation::Any,
                )),
                AbilityTargetDef::up_to(
                    AbilityTargetPredicate::OwnedByTargetPlayer {
                        object: ObjectPredicateDef::Any,
                        zones: &[ZoneKind::Graveyard],
                        slot: TargetIndex::PRIMARY,
                    },
                    3,
                ),
            ],
            EffectDef::Sequence(&[
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex(1)),
                    zone: ZoneKind::Library,
                    placement: ZonePlacement::Top,
                    controller: None,
                    arrival_effect: None,
                    attachment: None,
                    counters: None,
                    tapped: false,
                },
                EffectDef::ShuffleLibrary {
                    player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ]),
        ),
        abilities::flashback(mana_cost!("{G}")),
    ]),
);

// ISD 67 — Mindshrieker
static MINDSHRIEKER_PUMP: EffectDef = EffectDef::Apply {
    recipient: EffectRecipientDef::Source,
    effect: AppliedEffectDef::modify_power_toughness(
        ValueDef::MatchedManaValue,
        ValueDef::MatchedManaValue,
    ),
    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
};

pub(in crate::card::sets) static MINDSHRIEKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ab1e52af-6746-4ec6-afbf-008594c874f8"),
    "Mindshrieker",
    CardArt::new("ab1e52af-6746-4ec6-afbf-008594c874f8", "Dave Kendall"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Spirit", "Bird"], 1, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated_with_targets(
            "{2}: Target player mills a card. This creature gets +X/+X until end of turn, where X is the milled card's mana value.",
            &[AbilityCostDef::Mana(mana_cost!("{2}"))],
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
                PlayerRelation::Any,
            ))],
            EffectDef::Mill {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
                binding: None,
                then: Some(&MINDSHRIEKER_PUMP),
            },
        ),
    ]),
);

// ISD 68 — Mirror-Mad Phantasm
static MIRROR_MAD_REVEAL: MillUntilDef = MillUntilDef {
    player: EffectRecipientDef::player(PlayerRefDef::OwnerOf(ObjectRefDef::Source)),
    object: ObjectPredicateDef::Named("Mirror-Mad Phantasm"),
    matched_zone: ZoneKind::Battlefield,
    binding: None,
    then: None,
};

static MIRROR_MAD_STEPS: [EffectDef; 3] = [
    EffectDef::MoveToZone {
        object: EffectRecipientDef::matching_objects(
            ObjectPredicateDef::Source,
            &[ZoneKind::Battlefield],
            PlayerRelation::Any,
        ),
        zone: ZoneKind::Library,
        placement: ZonePlacement::Top,
        controller: None,
        arrival_effect: None,
        attachment: None,
        counters: None,
        tapped: false,
    },
    EffectDef::ShuffleLibrary {
        player: EffectRecipientDef::player(PlayerRefDef::OwnerOf(ObjectRefDef::Source)),
    },
    EffectDef::MillUntil(&MIRROR_MAD_REVEAL),
];

/// CR 118.12 makes shuffling the source into its owner's library a mandatory
/// cost paid during resolution. If that object has already left the
/// battlefield, the cost cannot begin and the conditional remainder is skipped.
static MIRROR_MAD_IF_PRESENT: EffectDef = EffectDef::IfCondition {
    condition: &TriggerConditionDef::SourceOnBattlefield,
    then: &EffectDef::Sequence(&MIRROR_MAD_STEPS),
};

pub(in crate::card::sets) static MIRROR_MAD_PHANTASM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b20eea41-9daf-4ac1-8bad-bb4aa211bb53"),
    "Mirror-Mad Phantasm",
    crate::card::CardArt::new("b20eea41-9daf-4ac1-8bad-bb4aa211bb53", "Howard Lyon"),
    crate::card::CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{3}{U}{U}"), &["Spirit"], 5, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "{1}{U}: This creature's owner shuffles it into their library. If that player does, they reveal cards from the top of that library until a card named Mirror-Mad Phantasm is revealed. The player puts that card onto the battlefield and all other cards revealed this way into their graveyard.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{U}"))],
            MIRROR_MAD_IF_PRESENT,
        ),
    ]),
);

// ISD 69 — Moon Heron
pub(in crate::card::sets) static MOON_HERON: CardRecord = CardRecord::new_with_legacy_id(
    883,
    "Moon Heron",
    CardArt::new("a24de601-1d7b-41c4-aba1-fdb6fd8d5251", "Charles Urbach"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Spirit", "Bird"], 3, 2)
        .with_ability(abilities::flying()),
);

// ISD 70 — Murder of Crows
pub(in crate::card::sets) static MURDER_OF_CROWS: CardRecord = CardRecord::new_with_legacy_id(
    884,
    "Murder of Crows",
    CardArt::new("f914f7e4-06fc-4943-8597-b7f834938c00", "Drew Baker"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{3}{U}{U}"), &["Bird"], 4, 4).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever another creature dies, you may draw a card. If you do, discard a card.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Sequence(&[
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
            },
        ),
    ]),
);

// ISD 71 — Rooftop Storm
pub(in crate::card::sets) static ROOFTOP_STORM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ab01d871-ba50-400a-95e7-09af9e34405f"),
    "Rooftop Storm",
    CardArt::new("ab01d871-ba50-400a-95e7-09af9e34405f", "John Stanko"),
    CardSet::Innistrad,
    CardRules::new_enchantment(mana_cost!("{5}{U}")).with_ability(AbilityDef::static_ability(
        "You may pay {0} rather than pay the mana cost for Zombie creature spells you cast.",
        EffectDef::ModifyCost(CostModificationDef::SpellAlternative {
            spell: ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::Subtype("Zombie"),
            ]),
            caster: PlayerRelation::You,
            zones: &[
                ZoneKind::Library,
                ZoneKind::Hand,
                ZoneKind::Graveyard,
                ZoneKind::Exile,
            ],
            cost: mana_cost!("{0}"),
        }),
    )),
);

// ISD 72 — Runic Repetition
pub(in crate::card::sets) static RUNIC_REPETITION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("53e47ba6-3a55-41b4-b8fe-580041669408"),
    "Runic Repetition",
    crate::card::CardArt::new("53e47ba6-3a55-41b4-b8fe-580041669408", "Svetlin Velinov"),
    crate::card::CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Return target exiled card with flashback you own to your hand.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasAbility(AbilityPredicateDef::Flashback),
                zones: &[ZoneKind::Exile],
                controller: None,
                owner: Some(PlayerRelation::You),
            },
        )],
        EffectDef::MoveToZone {
            counters: None,
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Hand,
            controller: None,
            placement: ZonePlacement::Top,
            arrival_effect: None,
            attachment: None,
            tapped: false,
        },
    )),
);

// ISD 73 — Selhoff Occultist
pub(in crate::card::sets) static SELHOFF_OCCULTIST: CardRecord = CardRecord::new_with_legacy_id(
    885,
    "Selhoff Occultist",
    CardArt::new("aeac4885-bd04-42bd-8e10-06c3efbce108", "Igor Kieryluk"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Rogue"], 2, 3).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever this creature or another creature dies, target player mills a card.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::HasType(CardType::Creature),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Mill {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
                binding: None,
                then: None,
            },
        ),
    ),
);

// ISD 74 — Sensory Deprivation
pub(in crate::card::sets) static SENSORY_DEPRIVATION: CardRecord = CardRecord::new_with_legacy_id(
    886,
    "Sensory Deprivation",
    CardArt::new("454739db-a3d6-45e8-849a-287438c36627", "Steven Belledin"),
    CardSet::Innistrad,
    CardRules::new_enchantment(mana_cost!("{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets -3/-0.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-3),
                        ValueDef::Constant(0),
                    ),
                },
            ),
        ]),
);

// ISD 75 — Silent Departure
pub(in crate::card::sets) static SILENT_DEPARTURE: CardRecord = CardRecord::new_with_legacy_id(
    887,
    "Silent Departure",
    CardArt::new("a18dea16-d535-4310-94ff-836645253d73", "John Avon"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Return target creature to its owner's hand.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::MoveToZone {
                counters: None,
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Hand,
                controller: None,
                placement: ZonePlacement::Top,
                arrival_effect: None,
                attachment: None,
                tapped: false,
            },
        ),
        abilities::flashback(mana_cost!("{4}{U}")),
    ]),
);

// ISD 76 — Skaab Goliath
/// `SpendModeDef::ByZone` would also exile a graveyard card, but the
/// Skaabs print "exile" and keep that operation explicit in their definitions.
const fn exile_creature_cards_from_graveyard(count: u8) -> SpellAdditionalCostDef {
    SpellAdditionalCostDef::new(
        ObjectPredicateDef::HasType(CardType::Creature),
        ZoneKind::Graveyard,
        count,
    )
    .spent(SpendModeDef::Exile)
}

/// Two so a graveyard holding one creature cannot pay at all and a
/// graveyard holding several offers every pair.
static EXILE_TWO_CREATURE_CARDS: SpellAdditionalCostDef = exile_creature_cards_from_graveyard(2);

pub(in crate::card::sets) static SKAAB_GOLIATH: CardRecord = CardRecord::new_with_legacy_id(
    1964,
    "Skaab Goliath",
    CardArt::new("7c1134a5-5434-4733-812b-3587b1817813", "Volkan Baǵa"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{5}{U}"), &["Zombie", "Giant"], 6, 9).with_abilities(&[
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, exile two creature cards from your \
             graveyard.",
            &[],
            EXILE_TWO_CREATURE_CARDS,
            EffectDef::None,
        ),
        abilities::trample(),
    ]),
);

// ISD 77 — Skaab Ruinator
static EXILE_THREE_CREATURE_CARDS: SpellAdditionalCostDef = exile_creature_cards_from_graveyard(3);

static CAST_SKAAB_RUINATOR_FROM_GRAVEYARD: PlayRestrictionDef =
    PlayRestrictionDef::new(PlayActionMatcherDef::CastSpell, ObjectPredicateDef::Source);

pub(in crate::card::sets) static SKAAB_RUINATOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("44c40cdc-a11a-47df-b902-d8fbe9014d03"),
    "Skaab Ruinator",
    CardArt::new("44c40cdc-a11a-47df-b902-d8fbe9014d03", "Chris Rahn"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{1}{U}{U}"), &["Zombie", "Horror"], 5, 6).with_abilities(
        &[
            AbilityDef::spell_with_additional_cost(
                "As an additional cost to cast this spell, exile three creature cards from your \
                 graveyard.",
                &[],
                EXILE_THREE_CREATURE_CARDS,
                EffectDef::None,
            ),
            abilities::flying(),
            AbilityDef::static_ability(
                "You may cast this card from your graveyard.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Controller,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::MayPlayFromGraveyard(
                        GraveyardPlayPermissionDef::unlimited(CAST_SKAAB_RUINATOR_FROM_GRAVEYARD),
                    )),
                },
            )
            .with_source_zones(&[ZoneKind::Graveyard]),
        ],
    ),
);

// ISD 78 — Snapcaster Mage
pub(in crate::card::sets) static SNAPCASTER_MAGE: CardRecord = CardRecord::new_with_legacy_id(
    215,
    "Snapcaster Mage",
    CardArt::new("9e5b279e-4670-4a1e-87d0-3cab7e4f9e58", "Volkan Baǵa"),
    CardSet::Innistrad,
    CardRules::new_creature(
        mana_cost!("{1}{U}"),
        &["Human", "Wizard"],
        2,
        1,
    )
    .with_abilities(&[
        abilities::flash(),
        abilities::enters_trigger_with_targets("When this creature enters, target instant or sorcery card in your graveyard gains flashback until end of turn. The flashback cost is equal to its mana cost. (You may cast that card from your graveyard for its flashback cost. Then exile it.)", &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::HasType(CardType::Sorcery),
                ]),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            },
        )], EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(
                    &abilities::flashback_for_card_mana_cost(),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            }),
    ]),
);

// ISD 79 — Spectral Flight
pub(in crate::card::sets) static SPECTRAL_FLIGHT: CardRecord = CardRecord::new_with_legacy_id(
    888,
    "Spectral Flight",
    CardArt::new("f7149f2a-6917-4ad7-8035-c7a1babd4d4b", "Johann Bodin"),
    CardSet::Innistrad,
    CardRules::new_enchantment(mana_cost!("{1}{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+2 and has flying.",
                EffectDef::Sequence(&[
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(2),
                        ),
                    },
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::add_ability(&abilities::flying()),
                    },
                ]),
            ),
        ]),
);

// ISD 80 — Stitched Drake
pub(in crate::card::sets) static STITCHED_DRAKE: CardRecord = CardRecord::new_with_legacy_id(
    1604,
    "Stitched Drake",
    CardArt::new("ad81266a-488f-449a-9daf-637727564865", "Chris Rahn"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{1}{U}{U}"), &["Zombie", "Drake"], 3, 4).with_abilities(&[
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, exile a creature card from your \
             graveyard.",
            &[],
            EXILE_A_CREATURE_CARD,
            EffectDef::None,
        ),
        abilities::flying(),
    ]),
);

// ISD 81 — Stitcher's Apprentice
pub(in crate::card::sets) static STITCHERS_APPRENTICE: CardRecord = CardRecord::new_with_legacy_id(
    889,
    "Stitcher's Apprentice",
    CardArt::new("7e0fcc53-cd0b-4b4c-b6de-5d301232106a", "Johann Bodin"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Homunculus"], 1, 2).with_ability(
        AbilityDef::activated(
            "{1}{U}, {T}: Create a 2/2 blue Homunculus creature token, then sacrifice a creature.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{U}")),
                AbilityCostDef::TapSource,
            ],
            EffectDef::Sequence(&[
                EffectDef::create_creature_token(&["Homunculus"], &[ManaColor::Blue], 2, 2)
                    .with_art(CardArt::new(
                        "e2020f53-d012-4d26-be13-87ed0f196c53",
                        "Johann Bodin",
                    )),
                EffectDef::SacrificeOfChoice {
                    count: ValueDef::Constant(1),
                    player: EffectRecipientDef::Controller,
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    then: None,
                    amount: SacrificedAmountDef::Power,
                    otherwise: None,
                    optional: false,
                },
            ]),
        ),
    ),
);

// ISD 82 — Sturmgeist
/// A threshold of zero is the exact count, which is what a characteristic
/// defined by the hand needs.
static CARDS_IN_YOUR_HAND: ValueDef = ValueDef::CardsInHandAbove {
    player: PlayerRelation::You,
    threshold: 0,
};

pub(in crate::card::sets) static STURMGEIST: CardRecord = CardRecord::new_with_legacy_id(
    1968,
    "Sturmgeist",
    CardArt::new("c409d1d0-fc45-40bf-adac-83b680209a38", "Terese Nielsen"),
    CardSet::Innistrad,
    // Its own trigger feeds it: connecting draws a card, which is one more
    // power the next time it swings.
    CardRules::new_creature(mana_cost!("{3}{U}{U}"), &["Spirit"], 0, 0).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "Sturmgeist's power and toughness are each equal to the number of cards in your hand.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::set_base_power_toughness(
                    CARDS_IN_YOUR_HAND,
                    CARDS_IN_YOUR_HAND,
                ),
            },
        ),
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, draw a card.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// ISD 83 — Think Twice
pub(in crate::card::sets) static THINK_TWICE: CardRecord = CardRecord::new_with_legacy_id(
    226,
    "Think Twice",
    CardArt::new("53e44060-a9a2-4095-9f5b-f60297525315", "Anthony Francisco"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[
        AbilityDef::spell(
            "Draw a card.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        abilities::flashback(mana_cost!("{2}{U}")),
    ]),
);

// ISD 84 — Undead Alchemist
// Audit: metadata-only — Needs a combat-damage replacement that mills instead, plus a linked library-to-graveyard creature-card trigger.
pub(in crate::card::sets) static UNDEAD_ALCHEMIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("717f4592-6c81-43ac-8975-f6d5d6710310"),
    "Undead Alchemist",
    crate::card::CardArt::new("717f4592-6c81-43ac-8975-f6d5d6710310", "Michael C. Hayes"),
    crate::card::CardSet::Innistrad,
    crate::card::CardRules::unsupported(),
);

// ISD 85 — Abattoir Ghoul
pub(in crate::card::sets) static ABATTOIR_GHOUL: CardRecord = CardRecord::new_with_legacy_id(
    2003,
    "Abattoir Ghoul",
    CardArt::new("59cf0906-04fa-4b30-a7a6-3d117931154f", "Volkan Baǵa"),
    CardSet::Innistrad,
    // First strike is what feeds it: the creature it kills in combat dies
    // before it can deal damage back.
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Zombie"], 3, 2).with_abilities(&[
        abilities::first_strike(),
        abilities::creature_damaged_by_source_dies_trigger(
            "Whenever a creature dealt damage by this creature this turn dies, you gain life equal to that creature's toughness.",
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                // Last known: the creature is already in a graveyard by the
                // time this runs, which is the only time it is interesting.
                amount: ValueDef::TriggeringObjectToughness,
            },
        ),
    ]),
);

// ISD 86 — Altar's Reap
pub(in crate::card::sets) static ALTARS_REAP: CardRecord = CardRecord::new_with_legacy_id(
    1607,
    "Altar's Reap",
    CardArt::new("4dc2eec4-7e68-45d5-8736-6b32a47c671b", "Donato Giancola"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{1}{B}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, sacrifice a creature.\nDraw two cards.",
            &[],
            SACRIFICE_A_CREATURE,
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
    ),
);

// ISD 87 — Army of the Damned
pub(in crate::card::sets) static ARMY_OF_THE_DAMNED: CardRecord = CardRecord::new_with_legacy_id(
    1887,
    "Army of the Damned",
    CardArt::new("260a4544-a1eb-4d07-943f-0401ae288e13", "Ryan Pancoast"),
    CardSet::Innistrad,
    // Tapped, so the army cannot attack the turn it arrives; flashback is
    // what makes the second one worth the wait.
    CardRules::new_sorcery(mana_cost!("{5}{B}{B}{B}")).with_abilities(&[
        AbilityDef::spell(
            "Create thirteen tapped 2/2 black Zombie creature tokens.",
            EffectDef::create_creature_token(&["Zombie"], &[ManaColor::Black], 2, 2)
                .with_art(CardArt::new(
                    "b877c19d-6022-4377-92e7-4511e24eb98e",
                    "Lucas Graciano",
                ))
                .with_amount(13)
                .entering_tapped(),
        ),
        abilities::flashback(mana_cost!("{7}{B}{B}{B}")),
    ]),
);

// ISD 88 — Bitterheart Witch
pub(in crate::card::sets) static BITTERHEART_WITCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cf2ff2b4-8f40-42c0-af3c-b55bfa8839be"),
    "Bitterheart Witch",
    crate::card::CardArt::new("cf2ff2b4-8f40-42c0-af3c-b55bfa8839be", "Karl Kopinski"),
    crate::card::CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Human", "Shaman"], 1, 2).with_abilities(&[
        abilities::deathtouch(),
        abilities::dies_trigger_with_targets(
            "When this creature dies, you may search your library for a Curse card, put it onto the battlefield attached to target player, then shuffle.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
                PlayerRelation::Any,
            ))],
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Subtype("Aura"),
                    ObjectPredicateDef::Subtype("Curse"),
                ]),
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: true,
                destination: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: false,
                attachment: Some(ArrivalAttachmentDef::ArrivalToPlayer(
                    PlayerRefDef::Target(TargetIndex::PRIMARY),
                )),
                binding: None,
                then: None,
            },
        ),
    ]),
);

// ISD 89 — Bloodgift Demon
pub(in crate::card::sets) static BLOODGIFT_DEMON: CardRecord = CardRecord::new_with_legacy_id(
    890,
    "Bloodgift Demon",
    CardArt::new("f271addb-e267-4397-b181-f1eaeabbfe71", "Peter Mohrbacher"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Demon"], 5, 4).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "At the beginning of your upkeep, target player draws a card and loses 1 life.",
            TriggerEventDef::StepBegins {
                step: crate::card::TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Sequence(&[
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                },
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
    ]),
);

// ISD 90 — Bloodline Keeper // Lord of Lineage
static BLOODLINE_KEEPER_FIVE_VAMPIRES: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: ObjectQueryDef::matching(
        ObjectPredicateDef::Subtype("Vampire"),
        &[ZoneKind::Battlefield],
        PlayerRelation::You,
    ),
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 5,
};

static BLOODLINE_KEEPER_CREATE_VAMPIRE: AbilityDef = AbilityDef::activated(
    "{T}: Create a 2/2 black Vampire creature token with flying.",
    &[AbilityCostDef::TapSource],
    EffectDef::create_creature_token(&["Vampire"], &[ManaColor::Black], 2, 2)
        .with_abilities(&[abilities::flying()]),
);

pub(in crate::card::sets) static BLOODLINE_KEEPER: CardRecord = CardRecord::new_dfc(
    PrintingAnchor::scryfall("13896468-e3d0-4bcb-b09e-b5c187aecb03"),
    "Bloodline Keeper // Lord of Lineage",
    CardArt::new("13896468-e3d0-4bcb-b09e-b5c187aecb03", "Jason Chan"),
    CardSet::Innistrad,
    &[
        (
            "Bloodline Keeper",
            CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Vampire"], 3, 3)
                .with_abilities(&[
                    abilities::flying(),
                    BLOODLINE_KEEPER_CREATE_VAMPIRE,
                    AbilityDef::activated(
                        "{B}: Transform this creature. Activate only if you control five or more Vampires.",
                        &[AbilityCostDef::Mana(mana_cost!("{B}"))],
                        EffectDef::Transform {
                            object: EffectRecipientDef::Source,
                        },
                    )
                    .with_activation_condition(&BLOODLINE_KEEPER_FIVE_VAMPIRES),
                ]),
        ),
        (
            "Lord of Lineage",
            CardRules::new_creature_without_mana_cost(&["Vampire"], 5, 5)
                .printed_colors(&[ManaColor::Black])
                .with_abilities(&[
                    abilities::flying(),
                    AbilityDef::static_ability(
                        "Other Vampire creatures you control get +2/+2.",
                        EffectDef::StaticApply {
                            recipient: EffectRecipientDef::matching_objects(
                                ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::Subtype("Vampire"),
                                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                                ]),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            ),
                            effect: AppliedEffectDef::modify_power_toughness(
                                ValueDef::Constant(2),
                                ValueDef::Constant(2),
                            ),
                        },
                    ),
                    BLOODLINE_KEEPER_CREATE_VAMPIRE,
                ]),
        ),
    ],
);

// ISD 91 — Brain Weevil
pub(in crate::card::sets) static BRAIN_WEEVIL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("40e1bd88-939a-4adc-8693-210a7ba9a5a1"),
    "Brain Weevil",
    crate::card::CardArt::new("40e1bd88-939a-4adc-8693-210a7ba9a5a1", "Anthony Jones"),
    crate::card::CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Insect"], 1, 1).with_abilities(&[
        abilities::intimidate(),
        AbilityDef::activated_with_targets(
            "Sacrifice this creature: Target player discards two cards. Activate only as a sorcery.",
            &[AbilityCostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Discard {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed),
    ]),
);

// ISD 92 — Bump in the Night
pub(in crate::card::sets) static BUMP_IN_THE_NIGHT: CardRecord = CardRecord::new_with_legacy_id(
    891,
    "Bump in the Night",
    CardArt::new("5c3ec389-a267-484f-994d-4a29ef494eb1", "Kev Walker"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Target opponent loses 3 life.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
            )],
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
            },
        ),
        abilities::flashback(mana_cost!("{5}{R}")),
    ]),
);

// ISD 93 — Corpse Lunge
pub(in crate::card::sets) static CORPSE_LUNGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2a74b987-527a-4560-a018-19d6bdf7e8b7"),
    "Corpse Lunge",
    crate::card::CardArt::new(
        "2a74b987-527a-4560-a018-19d6bdf7e8b7",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{2}{B}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, exile a creature card from your graveyard.\nThis spell deals damage equal to the exiled card's power to target creature.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EXILE_A_CREATURE_CARD,
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::ObjectPower(ObjectRefDef::AdditionalCostObject(
                    AdditionalCostObjectIndex::PRIMARY,
                )),
            },
        ),
    ),
);

// ISD 94 — Curse of Death's Hold
static CURSE_OF_DEATHS_HOLD_CREATURES: ObjectQueryDef = ObjectQueryDef::new(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::ControlledBy(PlayerRelation::EnchantedPlayer),
    ]),
    &[ZoneKind::Battlefield],
);
pub(in crate::card::sets) static CURSE_OF_DEATH_S_HOLD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1774d0a8-1cd3-4582-ace0-1caff92af0e7"),
    "Curse of Death's Hold",
    crate::card::CardArt::new("1774d0a8-1cd3-4582-ace0-1caff92af0e7", "Clint Cearley"),
    crate::card::CardSet::Innistrad,
    CardRules::new_enchantment(mana_cost!("{3}{B}{B}"))
        .with_subtypes(&["Aura", "Curse"])
        .with_abilities(&[
            abilities::enchant_player(),
            AbilityDef::static_ability(
                "Creatures enchanted player controls get -1/-1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        CURSE_OF_DEATHS_HOLD_CREATURES,
                    )),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-1),
                        ValueDef::Constant(-1),
                    ),
                },
            ),
        ]),
);

// ISD 95 — Curse of Oblivion
static CURSE_OF_OBLIVION_GRAVEYARD: [CardChoiceSourceDef; 1] =
    [CardChoiceSourceDef::Zone(ZoneKind::Graveyard)];
pub(in crate::card::sets) static CURSE_OF_OBLIVION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c15cbd2f-8bbf-423a-81fe-521fd99bc8bf"),
    "Curse of Oblivion",
    crate::card::CardArt::new(
        "c15cbd2f-8bbf-423a-81fe-521fd99bc8bf",
        "Jana Schirmer & Johannes Voss",
    ),
    crate::card::CardSet::Innistrad,
    CardRules::new_enchantment(mana_cost!("{3}{B}"))
        .with_subtypes(&["Aura", "Curse"])
        .with_abilities(&[
            abilities::enchant_player(),
            abilities::enchanted_player_upkeep(
                "At the beginning of enchanted player's upkeep, that player exiles two cards from their graveyard.",
                EffectDef::ChooseCards {
                    player: EffectRecipientDef::EnchantedPlayer,
                    sources: &CURSE_OF_OBLIVION_GRAVEYARD,
                    object: ObjectPredicateDef::Any,
                    minimum: 2,
                    maximum: 2,
                    reveal: false,
                    destination: ZoneKind::Exile,
                    placement: ZonePlacement::Top,
                    arrival_effect: None,
                },
            ),
        ]),
);

// ISD 96 — Dead Weight
pub(in crate::card::sets) static DEAD_WEIGHT: CardRecord = CardRecord::new_with_legacy_id(
    892,
    "Dead Weight",
    CardArt::new("7933987e-7b8c-4d5a-804a-708d6bb6d231", "Randy Gallegos"),
    CardSet::Innistrad,
    CardRules::new_enchantment(mana_cost!("{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets -2/-2.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-2),
                        ValueDef::Constant(-2),
                    ),
                },
            ),
        ]),
);

// ISD 97 — Diregraf Ghoul
pub(in crate::card::sets) static DIREGRAF_GHOUL: CardRecord = CardRecord::new_with_legacy_id(
    893,
    "Diregraf Ghoul",
    CardArt::new("4ed5790a-3354-49c2-89b6-3fc0de8dcc7c", "Dave Kendall"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{B}"), &["Zombie"], 2, 2)
        .with_ability(abilities::enters_tapped("This creature enters tapped.")),
);

// ISD 98 — Disciple of Griselbrand
/// The same shape Diamond Valley uses: the sacrifice is the ability's own
/// decision rather than a cost paid before it, because what is sacrificed
/// has to be readable by what follows.
static DISCIPLE_OF_GRISELBRAND_PAYOFF: EffectDef = EffectDef::GainLife {
    recipient: EffectRecipientDef::Controller,
    amount: ValueDef::TriggerEventAmount,
};

pub(in crate::card::sets) static DISCIPLE_OF_GRISELBRAND: CardRecord = CardRecord::new_with_legacy_id(
    1976,
    "Disciple of Griselbrand",
    CardArt::new("0c4acaa1-7d99-41ce-81ce-f6aef3e4dc1d", "Clint Cearley"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Cleric"], 1, 1).with_ability(
        AbilityDef::activated(
            "{1}, Sacrifice a creature: You gain life equal to the sacrificed creature's toughness.",
            &[AbilityCostDef::Mana(mana_cost!("{1}"))],
            EffectDef::SacrificeOfChoice {
                count: ValueDef::Constant(1),
                player: EffectRecipientDef::Controller,
                object: ObjectPredicateDef::HasType(CardType::Creature),
                then: Some(&DISCIPLE_OF_GRISELBRAND_PAYOFF),
                amount: SacrificedAmountDef::Toughness,
                otherwise: None,
                optional: false,
            },
        ),
    ),
);

// ISD 99 — Endless Ranks of the Dead
static ENDLESS_RANKS_ZOMBIES: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::Subtype("Zombie"),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

/// Rounded down, so a lone Zombie makes none and the engine only starts once
/// there are two.
static ENDLESS_RANKS_COUNT: HalvedValueDef = HalvedValueDef::new(
    ValueDef::CountMatchingObjects(&ENDLESS_RANKS_ZOMBIES),
    RoundingDef::Down,
);

pub(in crate::card::sets) static ENDLESS_RANKS_OF_THE_DEAD: CardRecord = CardRecord::new_with_legacy_id(
    1888,
    "Endless Ranks of the Dead",
    CardArt::new("5db15c5f-80b7-4f7f-985a-9bbec3199ad9", "Ryan Yee"),
    CardSet::Innistrad,
    CardRules::new_enchantment(mana_cost!("{2}{B}{B}")).with_ability(AbilityDef::triggered(
        "At the beginning of your upkeep, create X 2/2 black Zombie creature tokens, where X is \
         half the number of Zombies you control, rounded down.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerRelation::You,
        },
        EffectDef::create_creature_token(&["Zombie"], &[ManaColor::Black], 2, 2)
            .with_art(CardArt::new(
                "b877c19d-6022-4377-92e7-4511e24eb98e",
                "Lucas Graciano",
            ))
            .with_count(ValueDef::Halved(&ENDLESS_RANKS_COUNT)),
    )),
);

// ISD 100 — Falkenrath Noble
pub(in crate::card::sets) static FALKENRATH_NOBLE: CardRecord = CardRecord::new_with_legacy_id(
    894,
    "Falkenrath Noble",
    CardArt::new("e2286f94-4cf9-4462-b5d7-cee7f6910018", "Slawomir Maniak"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Vampire", "Noble"], 2, 2)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::triggered_with_targets(
                "Whenever this creature or another creature dies, target player loses 1 life and you gain 1 life.",
                TriggerEventDef::zone_changed(ObjectPredicateDef::HasType(CardType::Creature), Some(ZoneKind::Battlefield), Some(ZoneKind::Graveyard)),
                &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
                    PlayerRelation::Any,
                ))],
                EffectDef::Sequence(&[
                    EffectDef::LoseLife {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                ]),
            ),
        ]),
);

// ISD 101 — Ghoulcaller's Chant
static GHOULCALLERS_CHANT_ONE_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Graveyard],
        controller: None,
        owner: Some(PlayerRelation::You),
    },
)];

static GHOULCALLERS_CHANT_TWO_TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef {
    predicate: AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Subtype("Zombie"),
        ]),
        zones: &[ZoneKind::Graveyard],
        controller: None,
        owner: Some(PlayerRelation::You),
    },
    minimum: 2,
    maximum: 2,
    divided_total: None,
    another: false,
    excludes_source: false,
    chooser: TargetChooserDef::Controller,
}];

static GHOULCALLERS_CHANT_MODES: [AbilityDef; 2] = [
    AbilityDef::spell_with_targets(
        "Return target creature card from your graveyard to your hand.",
        &GHOULCALLERS_CHANT_ONE_TARGET,
        EffectDef::MoveToZone {
            counters: None,
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Hand,
            controller: None,
            placement: ZonePlacement::Top,
            arrival_effect: None,
            attachment: None,
            tapped: false,
        },
    ),
    AbilityDef::spell_with_targets(
        "Return two target Zombie cards from your graveyard to your hand.",
        &GHOULCALLERS_CHANT_TWO_TARGETS,
        EffectDef::MoveToZone {
            counters: None,
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Hand,
            controller: None,
            placement: ZonePlacement::Top,
            arrival_effect: None,
            attachment: None,
            tapped: false,
        },
    ),
];

pub(in crate::card::sets) static GHOULCALLERS_CHANT: CardRecord = CardRecord::new_with_legacy_id(
    895,
    "Ghoulcaller's Chant",
    CardArt::new("2b8c1b10-2155-404a-8f20-eb8f643849d6", "Randy Gallegos"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{B}")).with_ability(AbilityDef::choose_one_spell(
        "Choose one — Return target creature card from your graveyard to your hand; or return two target Zombie cards from your graveyard to your hand.",
        &GHOULCALLERS_CHANT_MODES,
    )),
);

// ISD 102 — Ghoulraiser
static GHOULRAISER_ZOMBIE_CARD: ObjectPredicateDef = ObjectPredicateDef::Subtype("Zombie");

pub(in crate::card::sets) static GHOULRAISER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("52c537d1-2d57-4a87-9dac-594d40d95633"),
    "Ghoulraiser",
    CardArt::new("52c537d1-2d57-4a87-9dac-594d40d95633", "Steve Prescott"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Zombie"], 2, 2).with_ability(
        abilities::enters_trigger(
            "When this creature enters, return a Zombie card at random from your graveyard to your hand.",
            EffectDef::SelectAtRandomFromZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Graveyard,
                object: GHOULRAISER_ZOMBIE_CARD,
                binding: ObjectSetBindingIndex::PRIMARY,
                then: &RETURN_RANDOM_GRAVEYARD_CARD_TO_HAND,
            },
        ),
    ),
);

// ISD 103 — Gruesome Deformity
pub(in crate::card::sets) static GRUESOME_DEFORMITY: CardRecord = CardRecord::new_with_legacy_id(
    896,
    "Gruesome Deformity",
    CardArt::new("5696db03-206f-4e7e-9b65-ccef31bfd7d2", "Matt Stewart"),
    CardSet::Innistrad,
    CardRules::new_enchantment(mana_cost!("{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature has intimidate.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&abilities::intimidate()),
                },
            ),
        ]),
);

// ISD 104 — Heartless Summoning
pub(in crate::card::sets) static HEARTLESS_SUMMONING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("14f8f638-b7fc-4e38-8623-ce7d2ebc82e6"),
    "Heartless Summoning",
    crate::card::CardArt::new("14f8f638-b7fc-4e38-8623-ce7d2ebc82e6", "Anthony Palumbo"),
    crate::card::CardSet::Innistrad,
    CardRules::new_enchantment(mana_cost!("{1}{B}")).with_abilities(&[
        abilities::spell_cost_reduction(
            "Creature spells you cast cost {2} less to cast.",
            ObjectPredicateDef::HasType(CardType::Creature),
            PlayerRelation::You,
            ValueDef::Constant(2),
        ),
        AbilityDef::static_ability(
            "Creatures you control get -1/-1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-1),
                    ValueDef::Constant(-1),
                ),
            },
        ),
    ]),
);

// ISD 105 — Liliana of the Veil
// Audit: custom — Needs declarative pile separation, opponent pile choice, and simultaneous sacrifice of the chosen pile.
static LILIANA_ULTIMATE_RESOLVER: CardAbilityResolver = CardAbilityResolver::new(
    "innistrad/liliana-of-the-veil/ultimate",
    resolve_liliana_ultimate,
);

const LILIANA_ULTIMATE_ABILITY: AbilityDef = AbilityDef::activated_with_targets(
    "−6: Separate all permanents target player controls into two piles. That player sacrifices all permanents in the pile of their choice.",
    &[AbilityCostDef::Loyalty(-6)],
    &[AbilityTargetDef::exactly_one(
        AbilityTargetPredicate::Player(PlayerRelation::Any),
    )],
    EffectDef::None,
)
.with_effect_execution(EffectExecutionDef::CardOwned)
.with_coverage(AbilityCoverageDef::explained_complete(
    "Pile separation, pile choice, and the chosen-pile sacrifice are composed by Liliana's card-owned resolver from shared runtime primitives.",
));

static LILIANA_ABILITY_BINDINGS: [CardAbilityBinding; 1] = [CardAbilityBinding::new(
    CardPartId::PRIMARY,
    AbilityId(2),
    LILIANA_ULTIMATE_ABILITY,
    &LILIANA_ULTIMATE_RESOLVER,
)
.with_policy_hint(
    AbilityPolicyHint::TargetPlayerSacrificesOneOfTwoPermanentPiles {
        target: TargetSlotId(0),
    },
)];

fn resolve_liliana_ultimate(runtime: &mut CardRuntime<'_>, ability: &ResolvedAbility) {
    let Some(victim) = ability.target_player(TargetIndex::PRIMARY) else {
        return;
    };
    let permanents = runtime.controlled_permanents(victim);
    runtime.queue_permanent_partition(
        ability.controller(),
        ability.controller(),
        victim,
        &permanents,
        LILIANA_PILES_SEPARATED,
    );
}

pub(in crate::card) static LILIANA_PILES_SEPARATED: PilesSeparated =
    PilesSeparated::new("lilianaOfTheVeil.pilesSeparated", liliana_piles_separated);

pub(in crate::card) static LILIANA_PILE_CHOSEN: PileChosen =
    PileChosen::new("lilianaOfTheVeil.pileChosen", liliana_pile_chosen);

fn liliana_piles_separated(runtime: &mut CardRuntime<'_>, piles: PileSplit) {
    let victim = piles.subject();
    runtime.queue_pile_choice(
        victim,
        piles,
        "Choose a pile to sacrifice",
        "Sacrifice pile",
        LILIANA_PILE_CHOSEN,
    );
}

fn liliana_pile_chosen(runtime: &mut CardRuntime<'_>, choice: PileChoice) {
    let victim = choice.subject();
    let resolving_controller = choice.resolving_controller();
    let (chosen, _unchosen) = choice.into_parts();
    runtime.sacrifice_permanents_simultaneously(&chosen, victim, resolving_controller);
}

pub(in crate::card::sets) static LILIANA_OF_THE_VEIL: CardRecord = CardRecord::new_with_legacy_id(
    184,
    "Liliana of the Veil",
    CardArt::new("ac506c17-adc8-49c6-9d8d-43db7cb1ec9d", "Steve Argyle"),
    CardSet::Innistrad,
    CardRules::new_planeswalker(mana_cost!("{1}{B}{B}"), &["Liliana"], 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated(
                "+1: Each player discards a card.",
                &[AbilityCostDef::Loyalty(1)],
                EffectDef::Discard {
                    recipient: EffectRecipientDef::EachPlayer,
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
            ),
            AbilityDef::activated_with_targets(
                "−2: Target player sacrifices a creature.",
                &[AbilityCostDef::Loyalty(-2)],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::SacrificeOfChoice {
                    count: ValueDef::Constant(1),
                    player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    then: None,
                    amount: SacrificedAmountDef::Power,
                    otherwise: None,
                    optional: false,
                },
            ),
            LILIANA_ULTIMATE_ABILITY,
        ]),
)
.with_ability_bindings(&LILIANA_ABILITY_BINDINGS);

// ISD 106 — Manor Skeleton
pub(in crate::card::sets) static MANOR_SKELETON: CardRecord = CardRecord::new_with_legacy_id(
    1432,
    "Manor Skeleton",
    CardArt::new("e7b45197-d5c2-48c8-b72e-00236552e338", "Eric Deschamps"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Skeleton"], 1, 1).with_abilities(&[
        abilities::haste(),
        abilities::regenerate_self(
            "{1}{B}: Regenerate this creature.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{B}"))],
        ),
    ]),
);

// ISD 107 — Markov Patrician
pub(in crate::card::sets) static MARKOV_PATRICIAN: CardRecord = CardRecord::new_with_legacy_id(
    897,
    "Markov Patrician",
    CardArt::new(
        "29c3d3f7-5e28-4fec-8422-87856fcd1e8e",
        "Jana Schirmer & Johannes Voss",
    ),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Vampire"], 3, 1)
        .with_ability(abilities::lifelink()),
);

// ISD 108 — Maw of the Mire
pub(in crate::card::sets) static MAW_OF_THE_MIRE: CardRecord = CardRecord::new_with_legacy_id(
    898,
    "Maw of the Mire",
    CardArt::new("90b34a03-3270-412c-90ca-03c1b3e61222", "Vincent Proce"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{4}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target land. You gain 4 life.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Land),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(4),
            },
        ]),
    )),
);

// ISD 109 — Moan of the Unhallowed
pub(in crate::card::sets) static MOAN_OF_THE_UNHALLOWED: CardRecord =
    CardRecord::new_with_legacy_id(
        899,
        "Moan of the Unhallowed",
        CardArt::new("3e2c5a8f-c03a-40ab-8390-ff6b5b654717", "Nils Hamm"),
        CardSet::Innistrad,
        CardRules::new_sorcery(mana_cost!("{2}{B}{B}")).with_abilities(&[
            AbilityDef::spell(
                "Create two 2/2 black Zombie creature tokens.",
                EffectDef::create_creature_token(&["Zombie"], &[ManaColor::Black], 2, 2)
                    .with_art(CardArt::new(
                        "b877c19d-6022-4377-92e7-4511e24eb98e",
                        "Lucas Graciano",
                    ))
                    .with_amount(2),
            ),
            abilities::flashback(mana_cost!("{5}{B}{B}")),
        ]),
    );

// ISD 110 — Morkrut Banshee
pub(in crate::card::sets) static MORKRUT_BANSHEE: CardRecord = CardRecord::new_with_legacy_id(
    1853,
    "Morkrut Banshee",
    CardArt::new("fff9989f-77a3-4f73-ade6-c04306c98501", "Svetlin Velinov"),
    CardSet::Innistrad,
    // The condition suppresses the trigger rather than its effect, so on a
    // turn with nothing dead there is no target to choose either.
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Spirit"], 4, 4).with_ability(
        AbilityDef::triggered_if_with_targets(
            "Morbid — When this creature enters, if a creature died this turn, target creature \
             gets -4/-4 until end of turn.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &ISD_MORBID_A_CREATURE_DIED,
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-4),
                    ValueDef::Constant(-4),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ISD 111 — Night Terrors
pub(in crate::card::sets) static NIGHT_TERRORS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5091658c-0314-42ee-87d8-95d3f457c4ab"),
    "Night Terrors",
    crate::card::CardArt::new(
        "5091658c-0314-42ee-87d8-95d3f457c4ab",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{2}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target player reveals their hand. You choose a nonland card from it. Exile that card.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Sequence(&abilities::reveal_hand_and_exile_chosen_card(
            PlayerRefDef::Target(TargetIndex::PRIMARY),
            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
        )),
    )),
);

// ISD 112 — Reaper from the Abyss
pub(in crate::card::sets) static REAPER_FROM_THE_ABYSS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f0d74c3e-8370-419b-808d-96b8d9306024"),
    "Reaper from the Abyss",
    crate::card::CardArt::new("f0d74c3e-8370-419b-808d-96b8d9306024", "Matt Stewart"),
    crate::card::CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{3}{B}{B}{B}"), &["Demon"], 6, 6).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_if_with_targets(
            "Morbid — At the beginning of each end step, if a creature died this turn, destroy target non-Demon creature.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::End,
                player: PlayerRelation::Any,
            },
            &ISD_MORBID_A_CREATURE_DIED,
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Demon")),
                ]),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
        ),
    ]),
);

// ISD 113 — Rotting Fensnake
pub(in crate::card::sets) static ROTTING_FENSNAKE: CardRecord = CardRecord::new_with_legacy_id(
    900,
    "Rotting Fensnake",
    CardArt::new("c21cbb10-9157-4887-a752-29b9e94fc77a", "Tomasz Jedruszek"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Zombie", "Snake"], 5, 1),
);

// ISD 114 — Screeching Bat // Stalking Vampire
static SCREECHING_BAT_TRANSFORM: EffectDef = EffectDef::Transform {
    object: EffectRecipientDef::Source,
};

static SCREECHING_BAT_UPKEEP_ABILITY: AbilityDef = AbilityDef::triggered(
    "At the beginning of your upkeep, you may pay {2}{B}{B}. If you do, transform this creature.",
    TriggerEventDef::StepBegins {
        step: crate::card::TurnStepDef::Upkeep,
        player: PlayerRelation::You,
    },
    EffectDef::PayOr(PayOrDef::optional(
        EffectPaymentDef::mana(
            PlayerSetDef::Related(PlayerRelation::You),
            mana_cost!("{2}{B}{B}"),
        ),
        &SCREECHING_BAT_TRANSFORM,
    )),
);

static SCREECHING_BAT_FRONT_ABILITIES: [AbilityDef; 2] =
    [abilities::flying(), SCREECHING_BAT_UPKEEP_ABILITY];

const fn screeching_bat_front_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Bat"], 2, 2)
        .with_abilities(&SCREECHING_BAT_FRONT_ABILITIES)
}

const fn stalking_vampire_rules() -> CardRules {
    CardRules::new_creature_without_mana_cost(&["Vampire"], 5, 5)
        .printed_colors(&[ManaColor::Black])
        .with_ability(SCREECHING_BAT_UPKEEP_ABILITY)
}

pub(in crate::card::sets) static SCREECHING_BAT: CardRecord = CardRecord::new_dfc_with_legacy_id(
    901,
    "Screeching Bat // Stalking Vampire",
    CardArt::new("88db324f-11f1-43d3-a897-f4e3caf8d642", "Slawomir Maniak"),
    CardSet::Innistrad,
    &[
        ("Screeching Bat", screeching_bat_front_rules()),
        ("Stalking Vampire", stalking_vampire_rules()),
    ],
);

// ISD 115 — Sever the Bloodline
pub(in crate::card::sets) static SEVER_THE_BLOODLINE: CardRecord = CardRecord::new_with_legacy_id(
    902,
    "Sever the Bloodline",
    CardArt::new("5c6da820-dfb9-4b61-aff8-56dfc9f4894e", "Clint Cearley"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{3}{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Exile target creature and all other creatures with the same name as that creature.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::MoveToZone {
                counters: None,
                object: EffectRecipientDef::ObjectsSharingNameWithTarget(TargetIndex::PRIMARY),
                zone: ZoneKind::Exile,
                controller: None,
                placement: ZonePlacement::Top,
                arrival_effect: None,
                attachment: None,
                tapped: false,
            },
        ),
        abilities::flashback(mana_cost!("{5}{B}{B}")),
    ]),
);

// ISD 116 — Skeletal Grimace
pub(in crate::card::sets) static SKELETAL_GRIMACE: CardRecord = CardRecord::new_with_legacy_id(
    1486,
    "Skeletal Grimace",
    CardArt::new("b9b28f37-d6b8-4d35-95e9-9533aea0a071", "Eric Deschamps"),
    CardSet::Innistrad,
    CardRules::new_enchantment(mana_cost!("{1}{B}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +1/+1 and has \"{B}: Regenerate this creature.\"",
                EffectDef::Sequence(&[
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                    },
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::add_ability(&abilities::regenerate_self(
                            "{B}: Regenerate this creature.",
                            &[AbilityCostDef::Mana(mana_cost!("{B}"))],
                        )),
                    },
                ]),
            ),
        ]),
);

// ISD 117 — Skirsdag High Priest
// Audit: metadata-only — Needs tapping two separately chosen other creatures as an activation cost and a morbid activation restriction.
pub(in crate::card::sets) static SKIRSDAG_HIGH_PRIEST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("09aa6b66-f69b-4f89-b802-e30c247f90e3"),
    "Skirsdag High Priest",
    crate::card::CardArt::new("09aa6b66-f69b-4f89-b802-e30c247f90e3", "Jason A. Engle"),
    crate::card::CardSet::Innistrad,
    crate::card::CardRules::unsupported(),
);

// ISD 118 — Stromkirk Patrol
pub(in crate::card::sets) static STROMKIRK_PATROL: CardRecord = CardRecord::new_with_legacy_id(
    903,
    "Stromkirk Patrol",
    CardArt::new("d86634a1-7016-4500-8857-924d51857bad", "Karl Kopinski"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{4}{B}"), &["Vampire", "Soldier"], 4, 3).with_ability(
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, put a +1/+1 counter on it.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// ISD 119 — Tribute to Hunger
/// The life comes to the caster rather than to the player who paid, which is
/// the whole difference between this and Devour Flesh.
static TRIBUTE_TO_HUNGER_PAYOFF: EffectDef = EffectDef::GainLife {
    recipient: EffectRecipientDef::Controller,
    amount: ValueDef::TriggerEventAmount,
};

pub(in crate::card::sets) static TRIBUTE_TO_HUNGER: CardRecord = CardRecord::new_with_legacy_id(
    1974,
    "Tribute to Hunger",
    CardArt::new("f77e0f88-2285-4b59-9165-9948c75d77a3", "Dave Kendall"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{2}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Target opponent sacrifices a creature of their choice. You gain life equal to that creature's toughness.",
        &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
            PlayerRelation::Opponent,
        ))],
        EffectDef::SacrificeOfChoice {
            count: ValueDef::Constant(1),
            player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            object: ObjectPredicateDef::HasType(CardType::Creature),
            then: Some(&TRIBUTE_TO_HUNGER_PAYOFF),
            amount: SacrificedAmountDef::Toughness,
            otherwise: None,
            optional: false,
        },
    )),
);

// ISD 120 — Typhoid Rats
pub(in crate::card::sets) static TYPHOID_RATS: CardRecord = CardRecord::new_with_legacy_id(
    904,
    "Typhoid Rats",
    CardArt::new("4490ce65-c73a-4809-abd1-ccc3175bd2a4", "Kev Walker"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{B}"), &["Rat"], 1, 1)
        .with_ability(abilities::deathtouch()),
);

// ISD 121 — Unbreathing Horde
// Audit: metadata-only — Needs a dynamic enters-with-counters count and a damage replacement that removes a counter instead.
pub(in crate::card::sets) static UNBREATHING_HORDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1a91ea47-0c06-4333-a309-ac360c5cc9bd"),
    "Unbreathing Horde",
    crate::card::CardArt::new("1a91ea47-0c06-4333-a309-ac360c5cc9bd", "Dave Kendall"),
    crate::card::CardSet::Innistrad,
    crate::card::CardRules::unsupported(),
);

// ISD 122 — Unburial Rites
pub(in crate::card::sets) static UNBURIAL_RITES: CardRecord = CardRecord::new_with_legacy_id(
    232,
    "Unburial Rites",
    CardArt::new("2794c82b-e5ce-4369-894e-bf56c6402ae1", "Ryan Pancoast"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{4}{B}")).with_abilities(&[
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
                counters: None,
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Battlefield,
                controller: None,
                placement: ZonePlacement::Top,
                arrival_effect: None,
                attachment: None,
                tapped: false,
            },
        ),
        abilities::flashback(mana_cost!("{3}{W}")),
    ]),
);

// ISD 123 — Vampire Interloper
pub(in crate::card::sets) static VAMPIRE_INTERLOPER: CardRecord = CardRecord::new_with_legacy_id(
    1517,
    "Vampire Interloper",
    CardArt::new("48105c2e-ee36-4117-b56b-3440298da995", "James Ryman"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Vampire", "Scout"], 2, 1).with_abilities(&[
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

// ISD 124 — Victim of Night
pub(in crate::card::sets) static VICTIM_OF_NIGHT: CardRecord = CardRecord::new_with_legacy_id(
    905,
    "Victim of Night",
    CardArt::new("ee4c6135-eee9-43ec-bbe8-76912352dcac", "Winona Nelson"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{B}{B}")).with_ability(AbilityDef::destroy_target(
        "Destroy target non-Vampire, non-Werewolf, non-Zombie creature.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Vampire")),
            ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Werewolf")),
            ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Zombie")),
        ])),
        true,
    )),
);

// ISD 125 — Village Cannibals
pub(in crate::card::sets) static VILLAGE_CANNIBALS: CardRecord = CardRecord::new_with_legacy_id(
    906,
    "Village Cannibals",
    CardArt::new("a5400460-da9d-437b-bb81-cf382beb371e", "Bud Cook"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Human"], 2, 2).with_ability(
        AbilityDef::triggered(
            "Whenever another Human creature dies, put a +1/+1 counter on this creature.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Subtype("Human"),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// ISD 126 — Walking Corpse
pub(in crate::card::sets) static WALKING_CORPSE: CardRecord = CardRecord::new_with_legacy_id(
    907,
    "Walking Corpse",
    CardArt::new("8e033384-3334-4082-9541-f2443d3bc424", "Igor Kieryluk"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Zombie"], 2, 2),
);

// ISD 127 — Ancient Grudge
pub(in crate::card::sets) static ANCIENT_GRUDGE: CardRecord = CardRecord::new_with_legacy_id(
    908,
    "Ancient Grudge",
    CardArt::new("e5e7b966-7c5b-44e6-a6df-4bd7af4edaa9", "Ryan Yee"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{1}{R}")).with_abilities(&[
        AbilityDef::destroy_target(
            "Destroy target artifact.",
            &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                CardType::Artifact,
            )),
            true,
        ),
        abilities::flashback(mana_cost!("{G}")),
    ]),
);

// ISD 128 — Ashmouth Hound
pub(in crate::card::sets) static ASHMOUTH_HOUND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("900ff07e-e5d2-4fe6-ad1a-d0d7e1a272ea"),
    "Ashmouth Hound",
    crate::card::CardArt::new("900ff07e-e5d2-4fe6-ad1a-d0d7e1a272ea", "Daarken"),
    crate::card::CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Elemental", "Dog"], 2, 1).with_ability(
        AbilityDef::triggered(
            "Whenever this creature blocks or becomes blocked by a creature, this creature deals 1 damage to that creature.",
            TriggerEventDef::BlocksOrBecomesBlockedBy {
                creature: ObjectPredicateDef::Source,
                other: ObjectPredicateDef::HasType(CardType::Creature),
            },
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::TriggeringObject,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// ISD 129 — Balefire Dragon
pub(in crate::card::sets) static BALEFIRE_DRAGON: CardRecord = CardRecord::new_with_legacy_id(
    909,
    "Balefire Dragon",
    CardArt::new("b0dce4ac-f472-4f3b-b01a-eff0902a578f", "Eric Deschamps"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{5}{R}{R}"), &["Dragon"], 6, 6).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, it deals that much damage to each creature that player controls.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::HasType(CardType::Creature), &[ZoneKind::Battlefield], PlayerRelation::Opponent),
                amount: ValueDef::TriggerEventAmount,
            },
        ),
    ]),
);

// ISD 130 — Blasphemous Act
/// Every creature anyone controls, which is what the reduction counts.
static EVERY_CREATURE: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::Any,
);

pub(in crate::card::sets) static BLASPHEMOUS_ACT: CardRecord = CardRecord::new_with_legacy_id(
    140,
    "Blasphemous Act",
    CardArt::new("509ce648-fb76-486d-8b39-183e368b7cb7", "Daarken"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{8}{R}")).with_abilities(&[
        AbilityDef::static_ability(
            "This spell costs {1} less to cast for each creature on the battlefield.",
            EffectDef::ReduceGenericCostBy(ValueDef::CountMatchingObjects(&EVERY_CREATURE)),
        )
        .with_source_zones(&[ZoneKind::Hand]),
        AbilityDef::spell(
            "Blasphemous Act deals 13 damage to each creature.",
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                amount: ValueDef::Constant(13),
            },
        ),
    ]),
);

// ISD 131 — Bloodcrazed Neonate
pub(in crate::card::sets) static BLOODCRAZED_NEONATE: CardRecord = CardRecord::new_with_legacy_id(
    910,
    "Bloodcrazed Neonate",
    CardArt::new("68d2452e-309d-44ae-9360-9d6e22a15e2b", "Cynthia Sheppard"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Vampire"], 2, 1).with_abilities(&[
        abilities::attacks_each_combat_if_able("This creature attacks each combat if able."),
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, put a +1/+1 counter on it.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// ISD 132 — Brimstone Volley
static BRIMSTONE_VOLLEY_AMOUNT: ValueDef = ValueDef::IfCreatureDiedThisTurn(&ConditionalValueDef {
    then: ValueDef::Constant(5),
    otherwise: ValueDef::Constant(3),
});

pub(in crate::card::sets) static BRIMSTONE_VOLLEY: CardRecord = CardRecord::new_with_legacy_id(
    911,
    "Brimstone Volley",
    CardArt::new("6960f2da-6b84-4680-8ab2-f0567a5d1b0a", "Eytan Zana"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{2}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "Brimstone Volley deals 3 damage to any target. Morbid — It deals 5 damage instead if a creature died this turn.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: BRIMSTONE_VOLLEY_AMOUNT,
            },
        ),
    ),
);

// ISD 133 — Burning Vengeance
// Audit: metadata-only — Needs spell-cast events to retain the zone from which the spell was cast.
pub(in crate::card::sets) static BURNING_VENGEANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fd403810-840b-46ac-ae6e-5df23ce16fec"),
    "Burning Vengeance",
    crate::card::CardArt::new("fd403810-840b-46ac-ae6e-5df23ce16fec", "Raymond Swanland"),
    crate::card::CardSet::Innistrad,
    crate::card::CardRules::unsupported(),
);

// ISD 134 — Charmbreaker Devils
static CHARMBREAKER_INSTANT_OR_SORCERY: ObjectPredicateDef = ObjectPredicateDef::AnyOf(&[
    ObjectPredicateDef::HasType(CardType::Instant),
    ObjectPredicateDef::HasType(CardType::Sorcery),
]);
pub(in crate::card::sets) static CHARMBREAKER_DEVILS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a9197a67-6609-496c-9aae-825ede4f755b"),
    "Charmbreaker Devils",
    CardArt::new("a9197a67-6609-496c-9aae-825ede4f755b", "Dan Murayama Scott"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{5}{R}"), &["Devil"], 4, 4).with_abilities(&[
        AbilityDef::triggered(
            "At the beginning of your upkeep, return an instant or sorcery card at random from your graveyard to your hand.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::SelectAtRandomFromZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Graveyard,
                object: CHARMBREAKER_INSTANT_OR_SORCERY,
                binding: ObjectSetBindingIndex::PRIMARY,
                then: &RETURN_RANDOM_GRAVEYARD_CARD_TO_HAND,
            },
        ),
        AbilityDef::triggered(
            "Whenever you cast an instant or sorcery spell, this creature gets +4/+0 until end of turn.",
            TriggerEventDef::SpellCast(ObjectPredicateDef::All(&[
                CHARMBREAKER_INSTANT_OR_SORCERY,
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(4),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// ISD 135 — Crossway Vampire
pub(in crate::card::sets) static CROSSWAY_VAMPIRE: CardRecord = CardRecord::new_with_legacy_id(
    1518,
    "Crossway Vampire",
    CardArt::new("3e7a137f-e19e-43a6-aab8-02b175c9d626", "Mark Evans"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Vampire"], 3, 2).with_ability(
        abilities::enters_trigger_with_targets(
            "When this creature enters, target creature can't block this turn.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ISD 136 — Curse of Stalked Prey
pub(in crate::card::sets) static CURSE_OF_STALKED_PREY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("11a18883-8990-40a0-bcb2-e01d0e82bfad"),
    "Curse of Stalked Prey",
    crate::card::CardArt::new(
        "11a18883-8990-40a0-bcb2-e01d0e82bfad",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Innistrad,
    CardRules::new_enchantment(mana_cost!("{1}{R}"))
        .with_subtypes(&["Aura", "Curse"])
        .with_abilities(&[
            abilities::enchant_player(),
            AbilityDef::triggered(
                "Whenever a creature deals combat damage to enchanted player, put a +1/+1 counter on that creature.",
                TriggerEventDef::combat_damage_to_related_player(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    PlayerRelation::EnchantedPlayer,
                ),
                EffectDef::AddCounters {
                    object: EffectRecipientDef::TriggeringObject,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// ISD 137 — Curse of the Nightly Hunt
static CURSE_OF_THE_NIGHTLY_HUNT_CREATURES: ObjectQueryDef = ObjectQueryDef::new(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::ControlledBy(PlayerRelation::EnchantedPlayer),
    ]),
    &[ZoneKind::Battlefield],
);
static CURSE_OF_THE_NIGHTLY_HUNT_ATTACKS: AbilityDef =
    abilities::attacks_each_combat_if_able("This creature attacks each combat if able.");
pub(in crate::card::sets) static CURSE_OF_THE_NIGHTLY_HUNT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("49cbfcf2-462e-4bbf-a529-a70816eb1436"),
    "Curse of the Nightly Hunt",
    crate::card::CardArt::new("49cbfcf2-462e-4bbf-a529-a70816eb1436", "Daarken"),
    crate::card::CardSet::Innistrad,
    CardRules::new_enchantment(mana_cost!("{2}{R}"))
        .with_subtypes(&["Aura", "Curse"])
        .with_abilities(&[
            abilities::enchant_player(),
            AbilityDef::static_ability(
                "Creatures enchanted player controls attack each combat if able.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        CURSE_OF_THE_NIGHTLY_HUNT_CREATURES,
                    )),
                    effect: AppliedEffectDef::add_ability(&CURSE_OF_THE_NIGHTLY_HUNT_ATTACKS),
                },
            ),
        ]),
);

// ISD 138 — Curse of the Pierced Heart
pub(in crate::card::sets) static CURSE_OF_THE_PIERCED_HEART: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("71010182-c004-4d18-adab-80319cd1e625"),
    "Curse of the Pierced Heart",
    crate::card::CardArt::new("71010182-c004-4d18-adab-80319cd1e625", "E. M. Gist"),
    crate::card::CardSet::Innistrad,
    CardRules::new_enchantment(mana_cost!("{1}{R}"))
        .with_subtypes(&["Aura", "Curse"])
        .with_abilities(&[
            abilities::enchant_player(),
            AbilityDef::triggered_with_targets(
                "At the beginning of enchanted player's upkeep, this Aura deals 1 damage to that player or a planeswalker that player controls.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::EnchantedPlayer,
                },
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::PlayerOrPlaneswalker(
                        PlayerRelation::EnchantedPlayer,
                    ),
                )],
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// ISD 139 — Desperate Ravings
pub(in crate::card::sets) static DESPERATE_RAVINGS: CardRecord = CardRecord::new_with_legacy_id(
    912,
    "Desperate Ravings",
    CardArt::new("2ba3ab3e-d16c-492f-a860-6d8efcadf679", "John Stanko"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{1}{R}")).with_abilities(&[
        AbilityDef::spell(
            "Draw two cards, then discard a card at random.",
            EffectDef::Sequence(&[
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::Random,
                    then: None,
                },
            ]),
        ),
        abilities::flashback(mana_cost!("{2}{U}")),
    ]),
);

// ISD 140 — Devil's Play
pub(in crate::card::sets) static DEVILS_PLAY: CardRecord = CardRecord::new_with_legacy_id(
    913,
    "Devil's Play",
    CardArt::new("c80596a4-b464-4b9e-8186-94a1c44838eb", "Austin Hsu"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{X}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Devil's Play deals X damage to any target.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::ChosenX,
            },
        ),
        abilities::flashback(mana_cost!("{X}{R}{R}{R}")),
    ]),
);

// ISD 141 — Falkenrath Marauders
pub(in crate::card::sets) static FALKENRATH_MARAUDERS: CardRecord = CardRecord::new_with_legacy_id(
    914,
    "Falkenrath Marauders",
    CardArt::new("b9c09887-6d2b-48b4-a483-16b8a45babd0", "James Ryman"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Vampire", "Warrior"], 2, 2)
        .with_abilities(&[
        abilities::flying(),
        abilities::haste(),
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, put two +1/+1 counters on it.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(2),
            },
        ),
    ]),
);

// ISD 142 — Feral Ridgewolf
pub(in crate::card::sets) static FERAL_RIDGEWOLF: CardRecord = CardRecord::new_with_legacy_id(
    915,
    "Feral Ridgewolf",
    CardArt::new("78c66cc0-cb0f-4daf-8141-0923ad46a834", "Martina Pilcerova"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Wolf"], 1, 2).with_abilities(&[
        abilities::trample(),
        AbilityDef::activated(
            "{1}{R}: This creature gets +2/+0 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{R}"))],
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

// ISD 143 — Furor of the Bitten
pub(in crate::card::sets) static FUROR_OF_THE_BITTEN: CardRecord = CardRecord::new_with_legacy_id(
    916,
    "Furor of the Bitten",
    CardArt::new("ff4a4c19-6427-4a03-a543-992c910e668f", "Randy Gallegos"),
    CardSet::Innistrad,
    CardRules::new_enchantment(mana_cost!("{R}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +2/+2 and attacks each combat if able.",
                EffectDef::Sequence(&[
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(2),
                            ValueDef::Constant(2),
                        ),
                    },
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::AttachedPermanent,
                        effect: AppliedEffectDef::add_ability(
                            &abilities::attacks_each_combat_if_able(
                                "This creature attacks each combat if able.",
                            ),
                        ),
                    },
                ]),
            ),
        ]),
);

// ISD 144 — Geistflame
pub(in crate::card::sets) static GEISTFLAME: CardRecord = CardRecord::new_with_legacy_id(
    917,
    "Geistflame",
    CardArt::new("1b856f31-ac80-4338-95a5-3f8acda74cfe", "Scott Chou"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Geistflame deals 1 damage to any target.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ),
        abilities::flashback(mana_cost!("{3}{R}")),
    ]),
);

// ISD 145 — Hanweir Watchkeep // Bane of Hanweir
static HANWEIR_WATCHKEEP_FRONT_ABILITIES: [AbilityDef; 2] =
    [abilities::defender(), WEREWOLF_FRONT_TRANSFORM];

static BANE_OF_HANWEIR_ABILITIES: [AbilityDef; 2] = [
    abilities::attacks_each_combat_if_able("This creature attacks each combat if able."),
    WEREWOLF_BACK_TRANSFORM,
];

const fn hanweir_watchkeep_front_rules() -> CardRules {
    CardRules::new_creature(
        mana_cost!("{2}{R}"),
        &["Human", "Warrior", "Werewolf"],
        1,
        5,
    )
    .with_abilities(&HANWEIR_WATCHKEEP_FRONT_ABILITIES)
}

const fn bane_of_hanweir_rules() -> CardRules {
    CardRules::new_creature_without_mana_cost(&["Werewolf"], 5, 5)
        .printed_colors(&[ManaColor::Red])
        .with_abilities(&BANE_OF_HANWEIR_ABILITIES)
}

pub(in crate::card::sets) static HANWEIR_WATCHKEEP: CardRecord = CardRecord::new_dfc_with_legacy_id(
    918,
    "Hanweir Watchkeep // Bane of Hanweir",
    CardArt::new("2b14ed17-1a35-4c49-ac46-3cad42d46c14", "Wayne Reynolds"),
    CardSet::Innistrad,
    &[
        ("Hanweir Watchkeep", hanweir_watchkeep_front_rules()),
        ("Bane of Hanweir", bane_of_hanweir_rules()),
    ],
);

// ISD 146 — Harvest Pyre
// Audit: metadata-only — Needs choosing and exiling X graveyard cards as a casting cost linked to the damage amount.
pub(in crate::card::sets) static HARVEST_PYRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4d6220b4-a5b8-45c8-9422-fab9eb32322c"),
    "Harvest Pyre",
    crate::card::CardArt::new("4d6220b4-a5b8-45c8-9422-fab9eb32322c", "Ryan Yee"),
    crate::card::CardSet::Innistrad,
    crate::card::CardRules::unsupported(),
);

// ISD 147 — Heretic's Punishment
// Audit: metadata-only — Needs the greatest mana value among three specifically milled cards.
pub(in crate::card::sets) static HERETIC_S_PUNISHMENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d2c9e963-bb0c-4490-8238-9476b924abf7"),
    "Heretic's Punishment",
    crate::card::CardArt::new("d2c9e963-bb0c-4490-8238-9476b924abf7", "Vincent Proce"),
    crate::card::CardSet::Innistrad,
    crate::card::CardRules::unsupported(),
);

// ISD 148 — Infernal Plunge
pub(in crate::card::sets) static INFERNAL_PLUNGE: CardRecord = CardRecord::new_with_legacy_id(
    1963,
    "Infernal Plunge",
    CardArt::new("b3f50e17-c29c-4d2c-b3e7-45d1216b81ea", "Daarken"),
    CardSet::Innistrad,
    // A sorcery rather than a mana ability, so the three red arrive on the
    // stack's terms and cannot be used to pay for the Plunge itself.
    CardRules::new_sorcery(mana_cost!("{R}")).with_ability(AbilityDef::spell_with_additional_cost(
        "As an additional cost to cast this spell, sacrifice a creature.\nAdd {R}{R}{R}.",
        &[],
        SACRIFICE_A_CREATURE,
        EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red).with_amount(3)),
    )),
);

// ISD 149 — Instigator Gang // Wildblood Pack
static INSTIGATOR_GANG_FRONT_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::static_ability(
        "Attacking creatures you control get +1/+0.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Attacking,
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(1),
                ValueDef::Constant(0),
            ),
        },
    ),
    WEREWOLF_FRONT_TRANSFORM,
];

static WILDBLOOD_PACK_ABILITIES: [AbilityDef; 3] = [
    abilities::trample(),
    AbilityDef::static_ability(
        "Attacking creatures you control get +3/+0.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Attacking,
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(3),
                ValueDef::Constant(0),
            ),
        },
    ),
    WEREWOLF_BACK_TRANSFORM,
];

const fn instigator_gang_front_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Human", "Werewolf"], 2, 3)
        .with_abilities(&INSTIGATOR_GANG_FRONT_ABILITIES)
}

const fn wildblood_pack_rules() -> CardRules {
    CardRules::new_creature_without_mana_cost(&["Werewolf"], 5, 5)
        .printed_colors(&[ManaColor::Red])
        .with_abilities(&WILDBLOOD_PACK_ABILITIES)
}

pub(in crate::card::sets) static INSTIGATOR_GANG: CardRecord = CardRecord::new_dfc_with_legacy_id(
    919,
    "Instigator Gang // Wildblood Pack",
    CardArt::new("bb90a6f1-c7f2-4c2e-ab1e-59c5c7937841", "Greg Staples"),
    CardSet::Innistrad,
    &[
        ("Instigator Gang", instigator_gang_front_rules()),
        ("Wildblood Pack", wildblood_pack_rules()),
    ],
);

// ISD 150 — Into the Maw of Hell
pub(in crate::card::sets) static INTO_THE_MAW_OF_HELL: CardRecord = CardRecord::new_with_legacy_id(
    920,
    "Into the Maw of Hell",
    CardArt::new("5d188d9b-7a12-4eaf-855b-af4f0204dc5a", "Raymond Swanland"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{4}{R}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target land. Into the Maw of Hell deals 13 damage to target creature.",
        &[
            AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Land)),
            AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                CardType::Creature,
            )),
        ],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex(1)),
                amount: ValueDef::Constant(13),
            },
        ]),
    )),
);

// ISD 151 — Kessig Wolf
pub(in crate::card::sets) static KESSIG_WOLF: CardRecord = CardRecord::new_with_legacy_id(
    921,
    "Kessig Wolf",
    CardArt::new("3255480b-c1cf-43d9-a40e-43e38112bb18", "Wayne England"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Wolf"], 3, 1).with_ability(
        AbilityDef::activated(
            "{1}{R}: This creature gains first strike until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{1}{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ISD 152 — Kruin Outlaw // Terror of Kruin Pass
static KRUIN_OUTLAW_ABILITIES: [AbilityDef; 2] =
    [abilities::first_strike(), WEREWOLF_FRONT_TRANSFORM];

static TERROR_OF_KRUIN_PASS_MENACE: AbilityDef = abilities::menace();

static TERROR_OF_KRUIN_PASS_ABILITIES: [AbilityDef; 3] = [
    abilities::double_strike(),
    AbilityDef::static_ability(
        "Werewolves you control have menace. (A creature with menace can't be blocked except by two or more creatures.)",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::Subtype("Werewolf"),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::add_ability(&TERROR_OF_KRUIN_PASS_MENACE),
        },
    ),
    WEREWOLF_BACK_TRANSFORM,
];

pub(in crate::card::sets) static KRUIN_OUTLAW: CardRecord = CardRecord::new_dfc(
    PrintingAnchor::scryfall("ec00d2d2-6597-474a-9353-345bbedfe57e"),
    "Kruin Outlaw // Terror of Kruin Pass",
    CardArt::new("ec00d2d2-6597-474a-9353-345bbedfe57e", "David Rapoza"),
    CardSet::Innistrad,
    &[
        (
            "Kruin Outlaw",
            CardRules::new_creature(
                mana_cost!("{1}{R}{R}"),
                &["Human", "Rogue", "Werewolf"],
                2,
                2,
            )
            .with_abilities(&KRUIN_OUTLAW_ABILITIES),
        ),
        (
            "Terror of Kruin Pass",
            CardRules::new_creature_without_mana_cost(&["Werewolf"], 3, 3)
                .printed_colors(&[ManaColor::Red])
                .with_abilities(&TERROR_OF_KRUIN_PASS_ABILITIES),
        ),
    ],
);

// ISD 153 — Night Revelers
static NIGHT_REVELERS_HASTE: AbilityDef = abilities::haste();

/// An opponent's Human, so a Human of your own does not wake it up.
static AN_OPPONENT_CONTROLS_A_HUMAN: TriggerConditionDef = TriggerConditionDef::ObjectCount {
    query: ObjectQueryDef::matching(
        ObjectPredicateDef::Subtype("Human"),
        &[ZoneKind::Battlefield],
        PlayerRelation::Opponent,
    ),
    comparison: ComparisonDef::GreaterOrEqual,
    amount: 1,
};

static NIGHT_REVELERS_GRANT: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::Source,
    effect: AppliedEffectDef::add_ability(&NIGHT_REVELERS_HASTE),
};

pub(in crate::card::sets) static NIGHT_REVELERS: CardRecord = CardRecord::new_with_legacy_id(
    1913,
    "Night Revelers",
    CardArt::new("e3f82c5c-77fa-45f3-a91e-4c2489444855", "Steve Argyle"),
    CardSet::Innistrad,
    // "As long as", so the haste comes and goes with the opponent's board
    // rather than being checked once.
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Vampire"], 4, 4).with_ability(
        AbilityDef::static_ability(
            "This creature has haste as long as an opponent controls a Human.",
            EffectDef::IfCondition {
                condition: &AN_OPPONENT_CONTROLS_A_HUMAN,
                then: &NIGHT_REVELERS_GRANT,
            },
        ),
    ),
);

// ISD 154 — Nightbird's Clutches
static CLUTCHES_TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::up_to(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
    2,
)];

pub(in crate::card::sets) static NIGHTBIRDS_CLUTCHES: CardRecord = CardRecord::new_with_legacy_id(
    1519,
    "Nightbird's Clutches",
    CardArt::new("b5c7410d-b69b-41a3-b469-e12c6ffc7578", "Jason A. Engle"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Up to two target creatures can't block this turn.",
            &CLUTCHES_TARGETS,
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::flashback(mana_cost!("{3}{R}")),
    ]),
);

// ISD 155 — Past in Flames
pub(in crate::card::sets) static PAST_IN_FLAMES: CardRecord = CardRecord::new_with_legacy_id(
    922,
    "Past in Flames",
    CardArt::new("23af6033-4930-48e4-821d-14cbbe1754b4", "Anthony Jones"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{3}{R}")).with_abilities(&[
        AbilityDef::spell(
            "Each instant and sorcery card in your graveyard gains flashback until end of turn. The flashback cost is equal to its mana cost.",
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                    ]), &[ZoneKind::Graveyard], PlayerRelation::You),
                effect: AppliedEffectDef::add_ability(&abilities::flashback_for_card_mana_cost()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
        abilities::flashback(mana_cost!("{4}{R}")),
    ]),
);

// ISD 156 — Pitchburn Devils
pub(in crate::card::sets) static PITCHBURN_DEVILS: CardRecord = CardRecord::new_with_legacy_id(
    923,
    "Pitchburn Devils",
    CardArt::new("d31d3de5-4028-457f-8eba-82e829061a40", "Johann Bodin"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Devil"], 3, 3).with_ability(
        abilities::dies_trigger_with_targets(
            "When this creature dies, it deals 3 damage to any target.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
            },
        ),
    ),
);

// ISD 157 — Rage Thrower
pub(in crate::card::sets) static RAGE_THROWER: CardRecord = CardRecord::new_with_legacy_id(
    924,
    "Rage Thrower",
    CardArt::new("f16db004-3e0c-491b-b8b6-0ae046d11761", "Peter Mohrbacher"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{5}{R}"), &["Human", "Shaman"], 4, 2).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever another creature dies, this creature deals 2 damage to target player or planeswalker.",
            TriggerEventDef::zone_changed(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]), Some(ZoneKind::Battlefield), Some(ZoneKind::Graveyard)),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
        ),
    ),
);

// ISD 158 — Rakish Heir
pub(in crate::card::sets) static RAKISH_HEIR: CardRecord = CardRecord::new_with_legacy_id(
    925,
    "Rakish Heir",
    CardArt::new("4afab3a6-95e3-4786-94f2-d9aa7365a4de", "Winona Nelson"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Vampire"], 2, 2).with_ability(
        AbilityDef::triggered(
            "Whenever a Vampire you control deals combat damage to a player, put a +1/+1 counter on it.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Subtype("Vampire"),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
            EffectDef::AddCounters {
                object: EffectRecipientDef::TriggeringObject,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// ISD 159 — Reckless Waif // Merciless Predator
const fn reckless_waif_front_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{R}"), &["Human", "Rogue", "Werewolf"], 1, 1)
        .with_ability(WEREWOLF_FRONT_TRANSFORM)
}

const fn merciless_predator_rules() -> CardRules {
    CardRules::new_creature_without_mana_cost(&["Werewolf"], 3, 2)
        .printed_colors(&[ManaColor::Red])
        .with_ability(WEREWOLF_BACK_TRANSFORM)
}

pub(in crate::card::sets) static RECKLESS_WAIF: CardRecord = CardRecord::new_dfc_with_legacy_id(
    926,
    "Reckless Waif // Merciless Predator",
    CardArt::new("028aeebc-4073-4595-94da-02f9f96ea148", "Michael C. Hayes"),
    CardSet::Innistrad,
    &[
        ("Reckless Waif", reckless_waif_front_rules()),
        ("Merciless Predator", merciless_predator_rules()),
    ],
);

// ISD 160 — Riot Devils
pub(in crate::card::sets) static RIOT_DEVILS: CardRecord = CardRecord::new_with_legacy_id(
    927,
    "Riot Devils",
    CardArt::new("cd35107b-6aaf-4fd8-bf1c-12b724d1482e", "Svetlin Velinov"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Devil"], 2, 3),
);

// ISD 161 — Rolling Temblor
pub(in crate::card::sets) static ROLLING_TEMBLOR: CardRecord = CardRecord::new_with_legacy_id(
    928,
    "Rolling Temblor",
    CardArt::new("060ce982-94dd-4b9e-b240-15da297e29f9", "Cliff Childs"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{2}{R}")).with_abilities(&[
        AbilityDef::spell(
            "Rolling Temblor deals 2 damage to each creature without flying.",
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasKeyword(
                            crate::card::KeywordAbility::Flying,
                        )),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                amount: ValueDef::Constant(2),
            },
        ),
        abilities::flashback(mana_cost!("{4}{R}{R}")),
    ]),
);

// ISD 162 — Scourge of Geier Reach
static OPPONENT_CREATURES: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Battlefield],
    PlayerRelation::Opponent,
);

pub(in crate::card::sets) static SCOURGE_OF_GEIER_REACH: CardRecord =
    CardRecord::new_with_legacy_id(
        929,
        "Scourge of Geier Reach",
        CardArt::new("e0c25932-96e7-4ae5-b544-8780f92d0be7", "Jung Park"),
        CardSet::Innistrad,
        CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Elemental"], 3, 3).with_ability(
            AbilityDef::static_ability(
                "This creature gets +1/+1 for each creature your opponents control.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::CountMatchingObjects(&OPPONENT_CREATURES),
                        ValueDef::CountMatchingObjects(&OPPONENT_CREATURES),
                    ),
                },
            ),
        ),
    );

// ISD 163 — Skirsdag Cultist
pub(in crate::card::sets) static SKIRSDAG_CULTIST: CardRecord = CardRecord::new_with_legacy_id(
    930,
    "Skirsdag Cultist",
    CardArt::new("e63fa0de-2ec3-41ff-8e5d-0b54f400f27f", "Slawomir Maniak"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Human", "Shaman"], 2, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{R}, {T}, Sacrifice a creature: This creature deals 2 damage to any target.",
            &[
                AbilityCostDef::Mana(mana_cost!("{R}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
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
    ),
);

// ISD 164 — Stromkirk Noble
pub(in crate::card::sets) static STROMKIRK_NOBLE: CardRecord = CardRecord::new_with_legacy_id(
    931,
    "Stromkirk Noble",
    CardArt::new("9c16cf74-f9e0-4d80-9a29-b91dec0b6b38", "James Ryman"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{R}"), &["Vampire", "Noble"], 1, 1).with_abilities(&[
        AbilityDef::static_ability(
            "This creature can't be blocked by Humans.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                    ObjectPredicateDef::Subtype("Human"),
                )),
            },
        ),
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, put a +1/+1 counter on it.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// ISD 165 — Tormented Pariah // Rampaging Werewolf
const fn tormented_pariah_front_rules() -> CardRules {
    CardRules::new_creature(
        mana_cost!("{3}{R}"),
        &["Human", "Warrior", "Werewolf"],
        3,
        2,
    )
    .with_ability(WEREWOLF_FRONT_TRANSFORM)
}

const fn rampaging_werewolf_rules() -> CardRules {
    CardRules::new_creature_without_mana_cost(&["Werewolf"], 6, 4)
        .printed_colors(&[ManaColor::Red])
        .with_ability(WEREWOLF_BACK_TRANSFORM)
}

pub(in crate::card::sets) static TORMENTED_PARIAH: CardRecord = CardRecord::new_dfc_with_legacy_id(
    932,
    "Tormented Pariah // Rampaging Werewolf",
    CardArt::new("6151cae7-92a4-4891-a952-21def412d3e4", "Bud Cook"),
    CardSet::Innistrad,
    &[
        ("Tormented Pariah", tormented_pariah_front_rules()),
        ("Rampaging Werewolf", rampaging_werewolf_rules()),
    ],
);

// ISD 166 — Traitorous Blood
pub(in crate::card::sets) static TRAITOROUS_BLOOD: CardRecord = CardRecord::new_with_legacy_id(
    933,
    "Traitorous Blood",
    CardArt::new("8220f18a-f23f-4fe6-bb58-58b6c5f36c79", "Raymond Swanland"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{1}{R}{R}")).with_ability(
        AbilityDef::spell_with_targets(
            "Gain control of target creature until end of turn. Untap it. It gains trample and haste until end of turn.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Sequence(&[
                EffectDef::GainControl {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    duration: ControlDurationDef::UntilEndOfTurn,
                    controller: PlayerRefDef::EffectController,
                },
                EffectDef::Untap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&abilities::trample()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&abilities::haste()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
    ),
);

// ISD 167 — Vampiric Fury
pub(in crate::card::sets) static VAMPIRIC_FURY: CardRecord = CardRecord::new_with_legacy_id(
    934,
    "Vampiric Fury",
    CardArt::new("de4fd254-0ae9-498d-b9da-4fb3d6a1a55c", "Matt Stewart"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(AbilityDef::spell(
        "Vampire creatures you control get +2/+0 and gain first strike until end of turn.",
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Vampire"),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(0),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Vampire"),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )),
);

// ISD 168 — Village Ironsmith // Ironfang
static VILLAGE_IRONSMITH_FRONT_ABILITIES: [AbilityDef; 2] =
    [abilities::first_strike(), WEREWOLF_FRONT_TRANSFORM];

static IRONFANG_ABILITIES: [AbilityDef; 2] = [abilities::first_strike(), WEREWOLF_BACK_TRANSFORM];

const fn village_ironsmith_front_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Werewolf"], 1, 1)
        .with_abilities(&VILLAGE_IRONSMITH_FRONT_ABILITIES)
}

const fn ironfang_rules() -> CardRules {
    CardRules::new_creature_without_mana_cost(&["Werewolf"], 3, 1)
        .printed_colors(&[ManaColor::Red])
        .with_abilities(&IRONFANG_ABILITIES)
}

pub(in crate::card::sets) static VILLAGE_IRONSMITH: CardRecord = CardRecord::new_dfc_with_legacy_id(
    935,
    "Village Ironsmith // Ironfang",
    CardArt::new(
        "cd5435d0-789f-4c42-8efc-165c072404a2",
        "Christopher Moeller",
    ),
    CardSet::Innistrad,
    &[
        ("Village Ironsmith", village_ironsmith_front_rules()),
        ("Ironfang", ironfang_rules()),
    ],
);

// ISD 169 — Ambush Viper
pub(in crate::card::sets) static AMBUSH_VIPER: CardRecord = CardRecord::new_with_legacy_id(
    936,
    "Ambush Viper",
    CardArt::new("0c082aa8-bf7f-47f2-baf8-43ad253fd7d7", "Alan Pollack"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Snake"], 2, 1)
        .with_abilities(&[abilities::flash(), abilities::deathtouch()]),
);

// ISD 170 — Avacyn's Pilgrim
pub(in crate::card::sets) static AVACYNS_PILGRIM: CardRecord = CardRecord::new_with_legacy_id(
    138,
    "Avacyn's Pilgrim",
    CardArt::new(
        "7eb39e97-53c2-4df0-9fb3-a3d6a24ec41f",
        "Jana Schirmer & Johannes Voss",
    ),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{G}"), &["Human", "Monk"], 1, 1)
        .with_abilities(&[abilities::tap_for(ManaColor::White)]),
);

// ISD 171 — Boneyard Wurm
pub(in crate::card::sets) static BONEYARD_WURM: CardRecord = CardRecord::new_with_legacy_id(
    937,
    "Boneyard Wurm",
    CardArt::new("75f3d9eb-462c-41b5-ad1a-baab7dc5eac3", "Jaime Jones"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Wurm"], 0, 0).with_ability(
        AbilityDef::static_ability(
            "Boneyard Wurm's power and toughness are each equal to the number of creature cards in your graveyard.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(ValueDef::CountMatchingObjects(&CREATURE_CARDS_IN_YOUR_GRAVEYARD), ValueDef::CountMatchingObjects(&CREATURE_CARDS_IN_YOUR_GRAVEYARD)),
            },
        ),
    ),
);

// ISD 172 — Bramblecrush
pub(in crate::card::sets) static BRAMBLECRUSH: CardRecord = CardRecord::new_with_legacy_id(
    938,
    "Bramblecrush",
    CardArt::new("60fa219e-5dba-4d49-9cae-40d254f140e4", "Drew Baker"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{2}{G}{G}")).with_ability(AbilityDef::destroy_target(
        "Destroy target noncreature permanent.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Not(
            &ObjectPredicateDef::HasType(CardType::Creature),
        )),
        true,
    )),
);

// ISD 173 — Caravan Vigil
// Audit: metadata-only — Needs the searched card's destination to branch on morbid while preserving the hidden-zone search choice.
pub(in crate::card::sets) static CARAVAN_VIGIL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9a8dfb98-a975-41bf-8aac-c0001c9ddaa7"),
    "Caravan Vigil",
    crate::card::CardArt::new("9a8dfb98-a975-41bf-8aac-c0001c9ddaa7", "Drew Baker"),
    crate::card::CardSet::Innistrad,
    crate::card::CardRules::unsupported(),
);

// ISD 174 — Creeping Renaissance
// Audit: metadata-only — Needs a permanent-card-type choice and a graveyard sweep keyed to the chosen type.
pub(in crate::card::sets) static CREEPING_RENAISSANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b965069d-8513-41ab-98f6-3fbd46c19e2d"),
    "Creeping Renaissance",
    crate::card::CardArt::new("b965069d-8513-41ab-98f6-3fbd46c19e2d", "Tomasz Jedruszek"),
    crate::card::CardSet::Innistrad,
    crate::card::CardRules::unsupported(),
);

// ISD 175 — Darkthicket Wolf
pub(in crate::card::sets) static DARKTHICKET_WOLF: CardRecord = CardRecord::new_with_legacy_id(
    1463,
    "Darkthicket Wolf",
    CardArt::new("fec37c5a-8223-441c-a8a6-8da1a2dfc3fb", "Wayne England"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Wolf"], 2, 2).with_abilities(&[
        AbilityDef::activated(
            "{2}{G}: This creature gets +2/+2 until end of turn. Activate only once each turn.",
            &[AbilityCostDef::Mana(mana_cost!("{2}{G}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        )
        .once_each_turn(),
    ]),
);

// ISD 176 — Daybreak Ranger // Nightfall Predator
static DAYBREAK_RANGER_TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::HasKeyword(crate::card::KeywordAbility::Flying),
    ]),
)];

static DAYBREAK_RANGER_FRONT_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::activated_with_targets(
        "{T}: This creature deals 2 damage to target creature with flying.",
        &[AbilityCostDef::TapSource],
        &DAYBREAK_RANGER_TARGETS,
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(2),
        },
    ),
    WEREWOLF_FRONT_TRANSFORM,
];

static NIGHTFALL_PREDATOR_TARGETS: [AbilityTargetDef; 1] =
    [AbilityTargetDef::exactly_one_permanent(
        ObjectPredicateDef::HasType(CardType::Creature),
    )];

static NIGHTFALL_PREDATOR_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::activated_with_targets(
        "{R}, {T}: This creature fights target creature.",
        &[
            AbilityCostDef::Mana(mana_cost!("{R}")),
            AbilityCostDef::TapSource,
        ],
        &NIGHTFALL_PREDATOR_TARGETS,
        EffectDef::Sequence(&[
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::SourcePower,
            },
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Source,
                amount: ValueDef::TargetPower(TargetIndex::PRIMARY),
            },
        ]),
    ),
    WEREWOLF_BACK_TRANSFORM,
];

const fn daybreak_ranger_front_rules() -> CardRules {
    CardRules::new_creature(
        mana_cost!("{2}{G}"),
        &["Human", "Archer", "Ranger", "Werewolf"],
        2,
        2,
    )
    .with_abilities(&DAYBREAK_RANGER_FRONT_ABILITIES)
}

const fn nightfall_predator_rules() -> CardRules {
    CardRules::new_creature_without_mana_cost(&["Werewolf"], 4, 4)
        .printed_colors(&[ManaColor::Green])
        .with_abilities(&NIGHTFALL_PREDATOR_ABILITIES)
}

pub(in crate::card::sets) static DAYBREAK_RANGER: CardRecord = CardRecord::new_dfc_with_legacy_id(
    939,
    "Daybreak Ranger // Nightfall Predator",
    CardArt::new("25b54a1d-e201-453b-9173-b04e06ee6fb7", "Steve Prescott"),
    CardSet::Innistrad,
    &[
        ("Daybreak Ranger", daybreak_ranger_front_rules()),
        ("Nightfall Predator", nightfall_predator_rules()),
    ],
);

// ISD 177 — Elder of Laurels
pub(in crate::card::sets) static ELDER_OF_LAURELS: CardRecord = CardRecord::new_with_legacy_id(
    940,
    "Elder of Laurels",
    CardArt::new("32b82ef0-c974-4357-b21a-4c2a28ec7279", "Terese Nielsen"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human", "Advisor"], 2, 3).with_ability(
        AbilityDef::activated_with_targets(
            "{3}{G}: Target creature gets +X/+X until end of turn, where X is the number of creatures you control.",
            &[AbilityCostDef::Mana(mana_cost!("{3}{G}"))],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(ValueDef::CountMatchingObjects(&CREATURES_YOU_CONTROL), ValueDef::CountMatchingObjects(&CREATURES_YOU_CONTROL)),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ISD 178 — Essence of the Wild
// Audit: metadata-only — Needs a battlefield entry replacement that copies the source's copiable values onto other creatures.
pub(in crate::card::sets) static ESSENCE_OF_THE_WILD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dec48cba-1b5d-44e7-9e25-16922dedb67d"),
    "Essence of the Wild",
    crate::card::CardArt::new("dec48cba-1b5d-44e7-9e25-16922dedb67d", "Terese Nielsen"),
    crate::card::CardSet::Innistrad,
    crate::card::CardRules::unsupported(),
);

// ISD 179 — Festerhide Boar
pub(in crate::card::sets) static FESTERHIDE_BOAR: CardRecord = CardRecord::new_with_legacy_id(
    1609,
    "Festerhide Boar",
    CardArt::new("31740fe9-27d2-416e-93de-509ac1a7b7cd", "Nils Hamm"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Boar"], 3, 3)
        .with_abilities(&[abilities::trample(), MORBID_TWO_COUNTERS]),
);

// ISD 180 — Full Moon's Rise
pub(in crate::card::sets) static FULL_MOONS_RISE: CardRecord = CardRecord::new_with_legacy_id(
    1487,
    "Full Moon's Rise",
    CardArt::new("02a35eac-b962-466e-a4da-a4010c68ef16", "Terese Nielsen"),
    CardSet::Innistrad,
    CardRules::new_enchantment(mana_cost!("{1}{G}")).with_abilities(&[
        AbilityDef::static_ability(
            "Werewolf creatures you control get +1/+0 and have trample.",
            EffectDef::Sequence(&[
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype("Werewolf"),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
                },
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype("Werewolf"),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::add_ability(&abilities::trample()),
                },
            ]),
        ),
        AbilityDef::activated(
            "Sacrifice this enchantment: Regenerate all Werewolf creatures you control.",
            &[AbilityCostDef::SacrificeSource],
            EffectDef::Regenerate {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Werewolf"),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
            },
        ),
    ]),
);

// ISD 181 — Garruk Relentless // Garruk, the Veil-Cursed
static GARRUK_FRONT_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::triggered_if(
        "When Garruk has two or fewer loyalty counters on him, transform him.",
        TriggerEventDef::StateCondition,
        &GARRUK_LOW_LOYALTY,
        EffectDef::Transform {
            object: EffectRecipientDef::Source,
        },
    ),
    AbilityDef::activated_with_targets(
        "0: Garruk deals 3 damage to target creature. That creature deals damage equal to its power to him.",
        &[AbilityCostDef::Loyalty(0)],
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        // The creature hits back with the power it had when the ability
        // resolved, which is why the loyalty it costs Garruk is read off
        // the target rather than printed.
        EffectDef::Sequence(&[
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
            },
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Source,
                amount: ValueDef::TargetPower(TargetIndex::PRIMARY),
            },
        ]),
    ),
    AbilityDef::activated(
        "0: Create a 2/2 green Wolf creature token.",
        &[AbilityCostDef::Loyalty(0)],
        EffectDef::create_creature_token(&["Wolf"], &[ManaColor::Green], 2, 2).with_art(
            CardArt::new("a53f8031-aaa8-424c-929a-5478538a8cc6", "David Palumbo"),
        ),
    ),
];

const fn garruk_front_rules() -> CardRules {
    CardRules::new_planeswalker(mana_cost!("{3}{G}"), &["Garruk"], 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&GARRUK_FRONT_ABILITIES)
}

/// Two or fewer is at most two, checked as a state trigger so it turns the
/// moment the damage lands rather than waiting for anything.
static GARRUK_LOW_LOYALTY: TriggerConditionDef = TriggerConditionDef::SourceLoyalty {
    comparison: ComparisonDef::LessOrEqual,
    amount: 2,
};

static GARRUK_TUTOR: EffectDef = EffectDef::SearchZone {
    player: EffectRecipientDef::Controller,
    source: ZoneKind::Library,
    object: ObjectPredicateDef::HasType(CardType::Creature),
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
};

static GARRUK_TRAMPLE: AbilityDef = abilities::trample();

static GARRUK_GRAVEYARD_CREATURES: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Creature),
    &[ZoneKind::Graveyard],
    PlayerRelation::You,
);

static GARRUK_BACK_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::activated(
        "+1: Create a 1/1 black Wolf creature token with deathtouch.",
        &[AbilityCostDef::Loyalty(1)],
        EffectDef::create_creature_token(&["Wolf"], &[ManaColor::Black], 1, 1)
            .with_abilities(&[abilities::deathtouch()])
            .with_art(CardArt::new(
                "7a49607c-427a-474c-ad77-60cd05844b3c",
                "Daniel Ljunggren",
            )),
    ),
    AbilityDef::activated(
        "−1: Sacrifice a creature. If you do, search your library for a creature card, reveal it, put it into your hand, then shuffle.",
        &[AbilityCostDef::Loyalty(-1)],
        EffectDef::SacrificeOfChoice {
            count: ValueDef::Constant(1),
            player: EffectRecipientDef::Controller,
            object: ObjectPredicateDef::HasType(CardType::Creature),
            then: Some(&GARRUK_TUTOR),
            amount: SacrificedAmountDef::Power,
            otherwise: None,
            optional: false,
        },
    ),
    AbilityDef::activated(
        "−3: Creatures you control gain trample and get +X/+X until end of turn, where X is the number of creature cards in your graveyard.",
        &[AbilityCostDef::Loyalty(-3)],
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: GARRUK_PUMP,
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    ),
];

static GARRUK_PUMP_PARTS: [AppliedEffectDef; 2] = [
    AppliedEffectDef::add_ability(&GARRUK_TRAMPLE),
    AppliedEffectDef::modify_power_toughness(
        ValueDef::CountMatchingObjects(&GARRUK_GRAVEYARD_CREATURES),
        ValueDef::CountMatchingObjects(&GARRUK_GRAVEYARD_CREATURES),
    ),
];

static GARRUK_PUMP: AppliedEffectDef = AppliedEffectDef::Composite(&GARRUK_PUMP_PARTS);

const fn garruk_back_rules() -> CardRules {
    CardRules::new_planeswalker_without_mana_cost(&["Garruk"])
        .with_supertype(CardSupertype::Legendary)
        .printed_colors(&[ManaColor::Black, ManaColor::Green])
        .with_abilities(&GARRUK_BACK_ABILITIES)
}

pub(in crate::card::sets) static GARRUK_RELENTLESS: CardRecord = CardRecord::new_dfc_with_legacy_id(
    165,
    "Garruk Relentless // Garruk, the Veil-Cursed",
    CardArt::new("b4160322-ff40-41a4-887a-73cd6b85ae45", "Eric Deschamps"),
    CardSet::Innistrad,
    &[
        ("Garruk Relentless", garruk_front_rules()),
        ("Garruk, the Veil-Cursed", garruk_back_rules()),
    ],
);

// ISD 182 — Gatstaf Shepherd // Gatstaf Howler
const fn gatstaf_shepherd_front_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Human", "Werewolf"], 2, 2)
        .with_ability(WEREWOLF_FRONT_TRANSFORM)
}

static GATSTAF_HOWLER_ABILITIES: [AbilityDef; 2] =
    [abilities::intimidate(), WEREWOLF_BACK_TRANSFORM];

const fn gatstaf_howler_rules() -> CardRules {
    CardRules::new_creature_without_mana_cost(&["Werewolf"], 3, 3)
        .printed_colors(&[ManaColor::Green])
        .with_abilities(&GATSTAF_HOWLER_ABILITIES)
}

pub(in crate::card::sets) static GATSTAF_SHEPHERD: CardRecord = CardRecord::new_dfc_with_legacy_id(
    941,
    "Gatstaf Shepherd // Gatstaf Howler",
    CardArt::new("57f0907f-74f4-4d86-93df-f2e50c9d0b2f", "Mark Evans"),
    CardSet::Innistrad,
    &[
        ("Gatstaf Shepherd", gatstaf_shepherd_front_rules()),
        ("Gatstaf Howler", gatstaf_howler_rules()),
    ],
);

// ISD 183 — Gnaw to the Bone
// Audit: metadata-only — Needs multiplying the number of creature cards in your graveyard by two for a life-gain amount.
pub(in crate::card::sets) static GNAW_TO_THE_BONE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("416148c8-13d3-46d3-ac93-6eb7cbab2881"),
    "Gnaw to the Bone",
    crate::card::CardArt::new("416148c8-13d3-46d3-ac93-6eb7cbab2881", "Scott Chou"),
    crate::card::CardSet::Innistrad,
    crate::card::CardRules::unsupported(),
);

// ISD 184 — Grave Bramble
pub(in crate::card::sets) static GRAVE_BRAMBLE: CardRecord = CardRecord::new_with_legacy_id(
    1908,
    "Grave Bramble",
    CardArt::new("8be1d4d2-5215-44b2-9b67-627d088efdb5", "Anthony Jones"),
    CardSet::Innistrad,
    // A wall that Zombies cannot get past at all: they can neither block it
    // nor be blocked into it, and their damage does not land.
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Plant"], 3, 4).with_abilities(&[
        abilities::defender(),
        AbilityDef::keyword(
            "Protection from Zombies",
            KeywordAbility::ProtectionFrom(&ObjectPredicateDef::Subtype("Zombie")),
        ),
    ]),
);

// ISD 185 — Grizzled Outcasts // Krallenhorde Wantons
const fn grizzled_outcasts_front_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Human", "Werewolf"], 4, 4)
        .with_ability(WEREWOLF_FRONT_TRANSFORM)
}

const fn krallenhorde_wantons_rules() -> CardRules {
    CardRules::new_creature_without_mana_cost(&["Werewolf"], 7, 7)
        .printed_colors(&[ManaColor::Green])
        .with_ability(WEREWOLF_BACK_TRANSFORM)
}

pub(in crate::card::sets) static GRIZZLED_OUTCASTS: CardRecord = CardRecord::new_dfc_with_legacy_id(
    942,
    "Grizzled Outcasts // Krallenhorde Wantons",
    CardArt::new("4b43b0cb-a5a3-47b4-9b6b-9d2638222bb6", "Randy Gallegos"),
    CardSet::Innistrad,
    &[
        ("Grizzled Outcasts", grizzled_outcasts_front_rules()),
        ("Krallenhorde Wantons", krallenhorde_wantons_rules()),
    ],
);

// ISD 186 — Gutter Grime
// Audit: metadata-only — Needs slime counters and a source-linked Ooze token whose P/T tracks the source's counter count.
pub(in crate::card::sets) static GUTTER_GRIME: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a9d007a2-163d-4e09-a70b-280a6fa3203b"),
    "Gutter Grime",
    crate::card::CardArt::new("a9d007a2-163d-4e09-a70b-280a6fa3203b", "Erica Yang"),
    crate::card::CardSet::Innistrad,
    crate::card::CardRules::unsupported(),
);

// ISD 187 — Hamlet Captain
/// One printed clause, two trigger events: a creature cannot both attack and
/// block, so exactly one of these fires and the bonus lands once.
static HAMLET_CAPTAIN_RALLY: EffectDef = EffectDef::Apply {
    recipient: EffectRecipientDef::matching_objects(
        ObjectPredicateDef::All(&[
            ObjectPredicateDef::Subtype("Human"),
            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
        ]),
        &[ZoneKind::Battlefield],
        PlayerRelation::You,
    ),
    effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)),
    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
};

pub(in crate::card::sets) static HAMLET_CAPTAIN: CardRecord = CardRecord::new_with_legacy_id(
    1755,
    "Hamlet Captain",
    CardArt::new("c8476d90-1212-4783-8d9e-05af32ad5ff1", "Wayne Reynolds"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Human", "Warrior"], 2, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever this creature attacks, other Humans you control get +1/+1 until end of \
             turn.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            HAMLET_CAPTAIN_RALLY,
        ),
        AbilityDef::triggered(
            "Whenever this creature blocks, other Humans you control get +1/+1 until end of turn.",
            TriggerEventDef::Blocks {
                blocked: ObjectPredicateDef::HasType(CardType::Creature),
            },
            HAMLET_CAPTAIN_RALLY,
        ),
    ]),
);

// ISD 188 — Hollowhenge Scavenger
pub(in crate::card::sets) static HOLLOWHENGE_SCAVENGER: CardRecord = CardRecord::new_with_legacy_id(
    1854,
    "Hollowhenge Scavenger",
    CardArt::new("6c9ff632-0e27-4521-9e9d-5725e618f5dd", "Slawomir Maniak"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Elemental"], 4, 5).with_ability(
        AbilityDef::triggered_if(
            "Morbid — When this creature enters, if a creature died this turn, you gain 5 life.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &ISD_MORBID_A_CREATURE_DIED,
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(5),
            },
        ),
    ),
);

// ISD 189 — Kessig Cagebreakers
// Audit: metadata-only — Needs a dynamic number of Wolf tokens entering tapped and attacking.
pub(in crate::card::sets) static KESSIG_CAGEBREAKERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fae22886-da03-49f2-873c-98a7ea2ee17d"),
    "Kessig Cagebreakers",
    crate::card::CardArt::new("fae22886-da03-49f2-873c-98a7ea2ee17d", "Wayne England"),
    crate::card::CardSet::Innistrad,
    crate::card::CardRules::unsupported(),
);

// ISD 190 — Kindercatch
pub(in crate::card::sets) static KINDERCATCH: CardRecord = CardRecord::new_with_legacy_id(
    943,
    "Kindercatch",
    CardArt::new("4954e8a3-e72b-4f28-8762-2b1c658c31b6", "Terese Nielsen"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{3}{G}{G}{G}"), &["Spirit"], 6, 6),
);

// ISD 191 — Lumberknot
pub(in crate::card::sets) static LUMBERKNOT: CardRecord = CardRecord::new_with_legacy_id(
    944,
    "Lumberknot",
    CardArt::new("6c86c84e-9bab-4a2c-b594-7f7b4b6bba88", "Jason A. Engle"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Treefolk"], 1, 1).with_abilities(&[
        abilities::hexproof(),
        AbilityDef::triggered(
            "Whenever a creature dies, put a +1/+1 counter on this creature.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::HasType(CardType::Creature),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// ISD 192 — Make a Wish
// Audit: metadata-only — Needs deterministic random selection of two cards from your graveyard.
pub(in crate::card::sets) static MAKE_A_WISH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d0a8508d-25a7-4fb2-8aa3-349275f80c42"),
    "Make a Wish",
    crate::card::CardArt::new("d0a8508d-25a7-4fb2-8aa3-349275f80c42", "Howard Lyon"),
    crate::card::CardSet::Innistrad,
    crate::card::CardRules::unsupported(),
);

// ISD 193 — Mayor of Avabruck // Howlpack Alpha
static MAYOR_OF_AVABRUCK_FRONT_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::static_ability(
        "Other Human creatures you control get +1/+1.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Subtype("Human"),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(1),
                ValueDef::Constant(1),
            ),
        },
    ),
    WEREWOLF_FRONT_TRANSFORM,
];

const fn mayor_of_avabruck_front_rules() -> CardRules {
    CardRules::new_creature(
        mana_cost!("{1}{G}"),
        &["Human", "Advisor", "Werewolf"],
        1,
        1,
    )
    .with_abilities(&MAYOR_OF_AVABRUCK_FRONT_ABILITIES)
}

static HOWLPACK_ALPHA_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::static_ability(
        "Each other creature you control that's a Werewolf or a Wolf gets +1/+1.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::Subtype("Werewolf"),
                        ObjectPredicateDef::Subtype("Wolf"),
                    ]),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(1),
                ValueDef::Constant(1),
            ),
        },
    ),
    AbilityDef::triggered(
        "At the beginning of your end step, create a 2/2 green Wolf creature token.",
        TriggerEventDef::StepBegins {
            step: crate::card::TurnStepDef::End,
            player: PlayerRelation::You,
        },
        EffectDef::create_creature_token(&["Wolf"], &[ManaColor::Green], 2, 2).with_art(
            CardArt::new("a53f8031-aaa8-424c-929a-5478538a8cc6", "David Palumbo"),
        ),
    ),
    WEREWOLF_BACK_TRANSFORM,
];

const fn howlpack_alpha_rules() -> CardRules {
    CardRules::new_creature_without_mana_cost(&["Werewolf"], 3, 3)
        .printed_colors(&[ManaColor::Green])
        .with_abilities(&HOWLPACK_ALPHA_ABILITIES)
}

pub(in crate::card::sets) static MAYOR_OF_AVABRUCK: CardRecord = CardRecord::new_dfc_with_legacy_id(
    945,
    "Mayor of Avabruck // Howlpack Alpha",
    CardArt::new("dd8ca448-f734-4cb9-b1d5-790eed9a4b2d", "Svetlin Velinov"),
    CardSet::Innistrad,
    &[
        ("Mayor of Avabruck", mayor_of_avabruck_front_rules()),
        ("Howlpack Alpha", howlpack_alpha_rules()),
    ],
);

// ISD 194 — Moldgraf Monstrosity
// Audit: metadata-only — Needs deterministic random selection of two creature cards from your graveyard after exiling the source.
pub(in crate::card::sets) static MOLDGRAF_MONSTROSITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f387c86a-702f-4f86-bcb9-d2bfa46fd211"),
    "Moldgraf Monstrosity",
    crate::card::CardArt::new("f387c86a-702f-4f86-bcb9-d2bfa46fd211", "Tomasz Jedruszek"),
    crate::card::CardSet::Innistrad,
    crate::card::CardRules::unsupported(),
);

// ISD 195 — Moonmist
// Audit: metadata-only — Needs transforming all Human double-faced permanents and selectively preventing combat damage from non-Werewolves and non-Wolves.
pub(in crate::card::sets) static MOONMIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("57153c3f-9e55-418c-b67b-36901f29f9c1"),
    "Moonmist",
    crate::card::CardArt::new("57153c3f-9e55-418c-b67b-36901f29f9c1", "Ryan Yee"),
    crate::card::CardSet::Innistrad,
    crate::card::CardRules::unsupported(),
);

// ISD 196 — Mulch
// Audit: custom — Needs declarative top-N reveal that moves all matching lands to hand and the remainder to the graveyard.
pub(in crate::card::sets) static MULCH: CardRecord = CardRecord::new_with_legacy_id(
    188,
    "Mulch",
    CardArt::new("52a1dabd-82df-4814-9d64-bf7bf9c1018d", "Christopher Moeller"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{1}{G}")).with_ability(
        AbilityDef::custom_full(
            "Reveal the top four cards of your library. Put all land cards revealed this way into your hand and the rest into your graveyard.",
            CardBehavior::Mulch,
            "Implemented by the named card-local special behavior.",
        ),
    ),
);

// ISD 197 — Naturalize (reprint)

// ISD 198 — Orchard Spirit
pub(in crate::card::sets) static ORCHARD_SPIRIT: CardRecord = CardRecord::new_with_legacy_id(
    946,
    "Orchard Spirit",
    CardArt::new("aac43ced-35b0-4e70-a049-1a65db9b2b1e", "Howard Lyon"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Spirit"], 2, 2).with_ability(
        AbilityDef::static_ability(
            "This creature can't be blocked except by creatures with flying or reach.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasKeyword(crate::card::KeywordAbility::Flying),
                        ObjectPredicateDef::HasKeyword(crate::card::KeywordAbility::Reach),
                    ])),
                )),
            },
        ),
    ),
);

// ISD 199 — Parallel Lives
// Audit: metadata-only — Needs a token-creation replacement event that doubles the number of tokens an effect would create.
pub(in crate::card::sets) static PARALLEL_LIVES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("01033dae-fec1-41f2-b7f2-cc6a43331790"),
    "Parallel Lives",
    crate::card::CardArt::new("01033dae-fec1-41f2-b7f2-cc6a43331790", "Steve Prescott"),
    crate::card::CardSet::Innistrad,
    crate::card::CardRules::unsupported(),
);

// ISD 200 — Prey Upon
pub(in crate::card::sets) static PREY_UPON: CardRecord = CardRecord::new_with_legacy_id(
    947,
    "Prey Upon",
    CardArt::new("b7b3eaf0-4207-4bac-923d-29f348c95a35", "Dave Kendall"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{G}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature you control fights target creature you don't control.",
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
                controller: Some(PlayerRelation::NotYou),
                owner: None,
            }),
        ],
        EffectDef::Sequence(&[
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex(1)),
                amount: ValueDef::TargetPower(TargetIndex::PRIMARY),
            },
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::TargetPower(TargetIndex(1)),
            },
        ]),
    )),
);

// ISD 201 — Ranger's Guile
pub(in crate::card::sets) static RANGERS_GUILE: CardRecord = CardRecord::new_with_legacy_id(
    948,
    "Ranger's Guile",
    CardArt::new("c90742ae-c48b-4d32-a6b7-aa51a94018bd", "Steve Prescott"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{G}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature you control gets +1/+1 and gains hexproof until end of turn.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::hexproof()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )),
);

// ISD 202 — Somberwald Spider
pub(in crate::card::sets) static SOMBERWALD_SPIDER: CardRecord = CardRecord::new_with_legacy_id(
    1610,
    "Somberwald Spider",
    CardArt::new("43003ad7-2f42-4c85-8b00-77cbf3f50a7b", "Volkan Baǵa"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{4}{G}"), &["Spider"], 2, 4)
        .with_abilities(&[abilities::reach(), MORBID_TWO_COUNTERS]),
);

// ISD 203 — Spider Spawning
pub(in crate::card::sets) static SPIDER_SPAWNING: CardRecord = CardRecord::new_with_legacy_id(
    949,
    "Spider Spawning",
    CardArt::new("f97007af-6642-4105-8d8c-4223681e1cf9", "Daniel Ljunggren"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{4}{G}")).with_abilities(&[
        AbilityDef::spell(
            "Create a 1/2 green Spider creature token with reach for each creature card in your graveyard.",
            EffectDef::create_creature_token(&["Spider"], &[ManaColor::Green], 1, 2).with_abilities(&[abilities::reach()]).with_art(CardArt::new("71031ff1-17dc-46b7-a72b-3297137a83bb", "Daniel Ljunggren")).with_count(ValueDef::CountMatchingObjects(&CREATURE_CARDS_IN_YOUR_GRAVEYARD)),
        ),
        abilities::flashback(mana_cost!("{6}{B}")),
    ]),
);

// ISD 204 — Spidery Grasp
pub(in crate::card::sets) static SPIDERY_GRASP: CardRecord = CardRecord::new_with_legacy_id(
    950,
    "Spidery Grasp",
    CardArt::new("ccbdfd82-d025-4070-a1f5-4ee759978bcb", "James Ryman"),
    CardSet::Innistrad,
    CardRules::new_instant(mana_cost!("{2}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Untap target creature. It gets +2/+4 and gains reach until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(4),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::reach()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )),
);

// ISD 205 — Splinterfright
pub(in crate::card::sets) static SPLINTERFRIGHT: CardRecord = CardRecord::new_with_legacy_id(
    951,
    "Splinterfright",
    CardArt::new("37068a41-bc5c-44b9-a307-5d3919794233", "Eric Deschamps"),
    CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Elemental"], 0, 0).with_abilities(&[
        abilities::trample(),
        AbilityDef::static_ability(
            "Splinterfright's power and toughness are each equal to the number of creature cards in your graveyard.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(ValueDef::CountMatchingObjects(&CREATURE_CARDS_IN_YOUR_GRAVEYARD), ValueDef::CountMatchingObjects(&CREATURE_CARDS_IN_YOUR_GRAVEYARD)),
            },
        ),
        AbilityDef::triggered(
            "At the beginning of your upkeep, mill two cards.",
            TriggerEventDef::StepBegins {
                step: crate::card::TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::Mill {
                player: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
                binding: None,
                then: None,
            },
        ),
    ]),
);

// ISD 206 — Travel Preparations
pub(in crate::card::sets) static TRAVEL_PREPARATIONS: CardRecord = CardRecord::new_with_legacy_id(
    952,
    "Travel Preparations",
    CardArt::new("e9654ae7-af2c-4956-be3a-68befa33f523", "Vincent Proce"),
    CardSet::Innistrad,
    CardRules::new_sorcery(mana_cost!("{1}{G}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Put a +1/+1 counter on each of up to two target creatures.",
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                2,
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
        abilities::flashback(mana_cost!("{1}{W}")),
    ]),
);

// ISD 207 — Tree of Redemption
// Audit: metadata-only — Needs exchanging the controller's life total with the source's current toughness.
pub(in crate::card::sets) static TREE_OF_REDEMPTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c6110bba-5c2d-4183-9dd0-d85a4cc42753"),
    "Tree of Redemption",
    crate::card::CardArt::new("c6110bba-5c2d-4183-9dd0-d85a4cc42753", "Vincent Proce"),
    crate::card::CardSet::Innistrad,
    crate::card::CardRules::unsupported(),
);

// ISD 208 — Ulvenwald Mystics // Ulvenwald Primordials
const fn ulvenwald_mystics_front_rules() -> CardRules {
    CardRules::new_creature(
        mana_cost!("{2}{G}{G}"),
        &["Human", "Shaman", "Werewolf"],
        3,
        3,
    )
    .with_ability(WEREWOLF_FRONT_TRANSFORM)
}

static ULVENWALD_PRIMORDIALS_ABILITIES: [AbilityDef; 2] = [
    abilities::regenerate_self(
        "{G}: Regenerate this creature.",
        &[AbilityCostDef::Mana(mana_cost!("{G}"))],
    ),
    WEREWOLF_BACK_TRANSFORM,
];

const fn ulvenwald_primordials_rules() -> CardRules {
    CardRules::new_creature_without_mana_cost(&["Werewolf"], 5, 5)
        .printed_colors(&[ManaColor::Green])
        .with_abilities(&ULVENWALD_PRIMORDIALS_ABILITIES)
}

pub(in crate::card::sets) static ULVENWALD_MYSTICS: CardRecord = CardRecord::new_dfc_with_legacy_id(
    1488,
    "Ulvenwald Mystics // Ulvenwald Primordials",
    CardArt::new("8325c570-4d74-4e65-891c-3e153abf4bf9", "Dan Murayama Scott"),
    CardSet::Innistrad,
    &[
        ("Ulvenwald Mystics", ulvenwald_mystics_front_rules()),
        ("Ulvenwald Primordials", ulvenwald_primordials_rules()),
    ],
);

// ISD 209 — Villagers of Estwald // Howlpack of Estwald
const fn villagers_of_estwald_front_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human", "Werewolf"], 2, 3)
        .with_ability(WEREWOLF_FRONT_TRANSFORM)
}

const fn howlpack_of_estwald_rules() -> CardRules {
    CardRules::new_creature_without_mana_cost(&["Werewolf"], 4, 6)
        .printed_colors(&[ManaColor::Green])
        .with_ability(WEREWOLF_BACK_TRANSFORM)
}

pub(in crate::card::sets) static VILLAGERS_OF_ESTWALD: CardRecord =
    CardRecord::new_dfc_with_legacy_id(
        953,
        "Villagers of Estwald // Howlpack of Estwald",
        CardArt::new("e42a0a3d-a987-4b24-b9d4-27380a12e093", "Kev Walker"),
        CardSet::Innistrad,
        &[
            ("Villagers of Estwald", villagers_of_estwald_front_rules()),
            ("Howlpack of Estwald", howlpack_of_estwald_rules()),
        ],
    );

// ISD 210 — Woodland Sleuth
static WOODLAND_SLEUTH_CREATURE_CARD: ObjectPredicateDef =
    ObjectPredicateDef::HasType(CardType::Creature);
pub(in crate::card::sets) static WOODLAND_SLEUTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3088a924-58c6-4ab7-baf0-842d6688fcec"),
    "Woodland Sleuth",
    crate::card::CardArt::new("3088a924-58c6-4ab7-baf0-842d6688fcec", "Tomasz Jedruszek"),
    crate::card::CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Human", "Scout"], 2, 3).with_ability(
        AbilityDef::triggered_if(
            "Morbid — When this creature enters, if a creature died this turn, return a creature card at random from your graveyard to your hand.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &ISD_MORBID_A_CREATURE_DIED,
            EffectDef::SelectAtRandomFromZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Graveyard,
                object: WOODLAND_SLEUTH_CREATURE_CARD,
                binding: ObjectSetBindingIndex::PRIMARY,
                then: &RETURN_RANDOM_GRAVEYARD_CARD_TO_HAND,
            },
        ),
    ),
);

// ISD 211 — Wreath of Geists
pub(in crate::card::sets) static WREATH_OF_GEISTS: CardRecord = CardRecord::new_with_legacy_id(
    954,
    "Wreath of Geists",
    CardArt::new("7604e22e-1f29-4a8f-b887-b18f43e3745e", "Jason A. Engle"),
    CardSet::Innistrad,
    CardRules::new_enchantment(mana_cost!("{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            AbilityDef::spell_with_targets(
                "Enchant creature",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature gets +X/+X, where X is the number of creature cards in your graveyard.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(ValueDef::CountMatchingObjects(&CREATURE_CARDS_IN_YOUR_GRAVEYARD), ValueDef::CountMatchingObjects(&CREATURE_CARDS_IN_YOUR_GRAVEYARD)),
                },
            ),
        ]),
);

// ISD 212 — Evil Twin
static EVIL_TWIN_DESTROY: AbilityDef = AbilityDef::activated_with_targets(
    "{U}{B}, {T}: Destroy target creature with the same name as this creature.",
    &[
        AbilityCostDef::Mana(mana_cost!("{U}{B}")),
        AbilityCostDef::TapSource,
    ],
    &[AbilityTargetDef::exactly_one_permanent(
        ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::HasName(ObjectRefDef::Source),
        ]),
    )],
    EffectDef::Destroy {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        can_regenerate: true,
        then: None,
    },
);

pub(in crate::card::sets) static EVIL_TWIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3a53487a-c00b-42da-904c-f022a0c5b1ed"),
    "Evil Twin",
    crate::card::CardArt::new("3a53487a-c00b-42da-904c-f022a0c5b1ed", "Greg Staples"),
    crate::card::CardSet::Innistrad,
    CardRules::new_creature(mana_cost!("{2}{U}{B}"), &["Shapeshifter"], 0, 0).with_ability(
        AbilityDef::replacement(
            "You may have this creature enter as a copy of any creature on the battlefield, except it has \"{U}{B}, {T}: Destroy target creature with the same name as this creature.\"",
            ReplacementEffectDef::CopyEntering {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                exceptions: CopyExceptionsDef::NONE.with_abilities(&[
                    crate::card::CopyAbilityDef::Ability(&EVIL_TWIN_DESTROY),
                ]),
            },
        ),
    ),
);

// ISD 213 — Geist of Saint Traft
// Audit: metadata-only — Needs a token entering tapped and attacking, linked to exile at end of combat.
pub(in crate::card::sets) static GEIST_OF_SAINT_TRAFT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("35b57113-b39a-460b-b4aa-02606b40bbd0"),
    "Geist of Saint Traft",
    crate::card::CardArt::new("35b57113-b39a-460b-b4aa-02606b40bbd0", "Igor Kieryluk"),
    crate::card::CardSet::Innistrad,
    crate::card::CardRules::unsupported(),
);

// ISD 214 — Grimgrin, Corpse-Born
// Audit: metadata-only — Needs an attack target restricted to the defending player's creatures and a linked destroy-then-counter continuation.
pub(in crate::card::sets) static GRIMGRIN_CORPSE_BORN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a8648734-ed6c-471f-91a1-6b710bbaf370"),
    "Grimgrin, Corpse-Born",
    crate::card::CardArt::new("a8648734-ed6c-471f-91a1-6b710bbaf370", "Peter Mohrbacher"),
    crate::card::CardSet::Innistrad,
    crate::card::CardRules::unsupported(),
);

// ISD 215 — Olivia Voldaren
// Audit: metadata-only — Needs a permanent subtype-adding effect and control lasting only while the source remains controlled.
pub(in crate::card::sets) static OLIVIA_VOLDAREN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ed750692-ba6a-4a89-ad6d-92fda7edc2cb"),
    "Olivia Voldaren",
    crate::card::CardArt::new("ed750692-ba6a-4a89-ad6d-92fda7edc2cb", "Eric Deschamps"),
    crate::card::CardSet::Innistrad,
    crate::card::CardRules::unsupported(),
);

// ISD 216 — Blazing Torch (reprint)

// ISD 217 — Butcher's Cleaver
static BUTCHERS_CLEAVER_LIFELINK: AbilityDef = abilities::lifelink();

static BUTCHERS_CLEAVER_HUMAN: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::AttachedPermanent,
    effect: AppliedEffectDef::add_ability(&BUTCHERS_CLEAVER_LIFELINK),
};

pub(in crate::card::sets) static BUTCHERS_CLEAVER: CardRecord = CardRecord::new_with_legacy_id(
    1593,
    "Butcher's Cleaver",
    CardArt::new("e141fe62-515e-4fe4-b032-81f169ec58d6", "Jason Felix"),
    CardSet::Innistrad,
    CardRules::new_artifact(mana_cost!("{3}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +3/+0.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(3),
                        ValueDef::Constant(0),
                    ),
                },
            ),
            AbilityDef::static_ability(
                "As long as equipped creature is a Human, it has lifelink.",
                EffectDef::IfCondition {
                    condition: &ATTACHED_PERMANENT_IS_HUMAN,
                    then: &BUTCHERS_CLEAVER_HUMAN,
                },
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{3}"))], "Equip {3}"),
        ]),
);

// ISD 218 — Cellar Door
// Audit: metadata-only — Needs moving the bottom library card and branching on that moved card's creature type.
pub(in crate::card::sets) static CELLAR_DOOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("97bdfb00-7773-4af6-895c-c90088a96b07"),
    "Cellar Door",
    crate::card::CardArt::new("97bdfb00-7773-4af6-895c-c90088a96b07", "Rob Alexander"),
    crate::card::CardSet::Innistrad,
    crate::card::CardRules::unsupported(),
);

// ISD 219 — Cobbled Wings
static COBBLED_WINGS_FLYING: AbilityDef = abilities::flying();

pub(in crate::card::sets) static COBBLED_WINGS: CardRecord = CardRecord::new_with_legacy_id(
    1591,
    "Cobbled Wings",
    CardArt::new("24abd762-e533-491a-97b6-aed40c214e9d", "Matt Stewart"),
    CardSet::Innistrad,
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature has flying.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&COBBLED_WINGS_FLYING),
                },
            ),
            abilities::equip(
                &[AbilityCostDef::Mana(mana_cost!("{1}"))],
                "Equip {1} ({1}: Attach to target creature you control. Equip only as a \
                 sorcery.)",
            ),
        ]),
);

// ISD 220 — Creepy Doll
// Audit: metadata-only — Needs a recorded coin flip after combat damage to a creature and a conditional destroy branch.
pub(in crate::card::sets) static CREEPY_DOLL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("762a598b-8753-47ec-9dd6-2c3d8882fda6"),
    "Creepy Doll",
    crate::card::CardArt::new("762a598b-8753-47ec-9dd6-2c3d8882fda6", "Matt Stewart"),
    crate::card::CardSet::Innistrad,
    crate::card::CardRules::unsupported(),
);

// ISD 221 — Demonmail Hauberk
static DEMONMAIL_HAUBERK_EQUIP_COST: [AbilityCostDef; 1] = [AbilityCostDef::SacrificePermanent {
    object: ObjectPredicateDef::HasType(CardType::Creature),
    controller: PlayerRelation::You,
}];

pub(in crate::card::sets) static DEMONMAIL_HAUBERK: CardRecord = CardRecord::new_with_legacy_id(
    2306,
    "Demonmail Hauberk",
    CardArt::new("aa33caa8-2a07-4f6c-a6c2-d21cf2d61193", "Jason Felix"),
    CardSet::Innistrad,
    CardRules::new_artifact(mana_cost!("{4}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +4/+2.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(4),
                        ValueDef::Constant(2),
                    ),
                },
            ),
            abilities::equip(&DEMONMAIL_HAUBERK_EQUIP_COST, "Equip—Sacrifice a creature."),
        ]),
);

// ISD 222 — Galvanic Juggernaut
pub(in crate::card::sets) static GALVANIC_JUGGERNAUT: CardRecord = CardRecord::new_with_legacy_id(
    955,
    "Galvanic Juggernaut",
    CardArt::new("d14bc109-d5d5-4777-90e4-bef26d106571", "Lucas Graciano"),
    CardSet::Innistrad,
    CardRules::new_artifact_creature(mana_cost!("{4}"), &["Juggernaut"], 5, 5).with_abilities(&[
        abilities::attacks_each_combat_if_able("This creature attacks each combat if able."),
        AbilityDef::static_ability(
            "This creature doesn't untap during your untap step.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::DoesNotUntapDuringUntapStep),
            },
        ),
        AbilityDef::triggered(
            "Whenever another creature dies, untap this creature.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            EffectDef::Untap {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// ISD 223 — Geistcatcher's Rig
pub(in crate::card::sets) static GEISTCATCHERS_RIG: CardRecord = CardRecord::new_with_legacy_id(
    956,
    "Geistcatcher's Rig",
    CardArt::new("cfb8ecf0-8c12-4a14-9a75-4cc5bf9e47f1", "Vincent Proce"),
    CardSet::Innistrad,
    CardRules::new_artifact_creature(mana_cost!("{6}"), &["Construct"], 4, 5)
        .with_ability(abilities::enters_trigger_with_targets("When this creature enters, you may have it deal 4 damage to target creature with flying.", &[AbilityTargetDef::up_to(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasKeyword(crate::card::KeywordAbility::Flying),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
            1,
        )], EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(4),
        })),
);

// ISD 224 — Ghoulcaller's Bell
pub(in crate::card::sets) static GHOULCALLERS_BELL: CardRecord = CardRecord::new_with_legacy_id(
    957,
    "Ghoulcaller's Bell",
    CardArt::new("863e7c2a-698c-4dce-a10b-ca58e4affa57", "Lars Grant-West"),
    CardSet::Innistrad,
    CardRules::new_artifact(mana_cost!("{1}")).with_ability(AbilityDef::activated(
        "{T}: Each player mills a card.",
        &[AbilityCostDef::TapSource],
        EffectDef::Mill {
            player: EffectRecipientDef::EachPlayer,
            amount: ValueDef::Constant(1),
            binding: None,
            then: None,
        },
    )),
);

// ISD 225 — Graveyard Shovel
// Audit: metadata-only — Needs the targeted player to choose one card from their graveyard and a creature-card test after exile.
pub(in crate::card::sets) static GRAVEYARD_SHOVEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1a4b8888-a10c-48b1-ba19-c041e5667b29"),
    "Graveyard Shovel",
    crate::card::CardArt::new("1a4b8888-a10c-48b1-ba19-c041e5667b29", "Martina Pilcerova"),
    crate::card::CardSet::Innistrad,
    crate::card::CardRules::unsupported(),
);

// ISD 226 — Grimoire of the Dead
// Audit: metadata-only — Needs study counters and a graveyard sweep that changes returned creatures' colors and types.
pub(in crate::card::sets) static GRIMOIRE_OF_THE_DEAD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d268d078-b854-47c1-bc7f-7698723405a2"),
    "Grimoire of the Dead",
    crate::card::CardArt::new("d268d078-b854-47c1-bc7f-7698723405a2", "Steven Belledin"),
    crate::card::CardSet::Innistrad,
    crate::card::CardRules::unsupported(),
);

// ISD 227 — Inquisitor's Flail
// Audit: metadata-only — Needs combat-damage replacement effects tied to an equipped creature.
pub(in crate::card::sets) static INQUISITOR_S_FLAIL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3014f59d-9012-473a-8bb1-8085c6e91632"),
    "Inquisitor's Flail",
    crate::card::CardArt::new("3014f59d-9012-473a-8bb1-8085c6e91632", "Rob Alexander"),
    crate::card::CardSet::Innistrad,
    crate::card::CardRules::unsupported(),
);

// ISD 228 — Manor Gargoyle
// Audit: metadata-only — Needs indestructible to depend continuously on retaining defender while an activation temporarily removes defender.
pub(in crate::card::sets) static MANOR_GARGOYLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6bb40965-9096-4a19-b71d-4da2a5b36baa"),
    "Manor Gargoyle",
    crate::card::CardArt::new("6bb40965-9096-4a19-b71d-4da2a5b36baa", "Matt Stewart"),
    crate::card::CardSet::Innistrad,
    crate::card::CardRules::unsupported(),
);

// ISD 229 — Mask of Avacyn
static MASK_OF_AVACYN_HEXPROOF: AbilityDef = abilities::hexproof();

static MASK_OF_AVACYN_BONUS: [AppliedEffectDef; 2] = [
    AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(2)),
    AppliedEffectDef::add_ability(&MASK_OF_AVACYN_HEXPROOF),
];

pub(in crate::card::sets) static MASK_OF_AVACYN: CardRecord = CardRecord::new_with_legacy_id(
    1626,
    "Mask of Avacyn",
    CardArt::new("4ff1acce-bed4-452c-8416-06726004f2e8", "James Paick"),
    CardSet::Innistrad,
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +1/+2 and has hexproof.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&MASK_OF_AVACYN_BONUS),
                },
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{3}"))], "Equip {3}"),
        ]),
);

// ISD 230 — One-Eyed Scarecrow
pub(in crate::card::sets) static ONE_EYED_SCARECROW: CardRecord = CardRecord::new_with_legacy_id(
    958,
    "One-Eyed Scarecrow",
    CardArt::new("5d495d85-6458-44d5-b3b4-5e09569057e3", "Dave Kendall"),
    CardSet::Innistrad,
    CardRules::new_artifact_creature(mana_cost!("{3}"), &["Scarecrow"], 2, 3).with_abilities(&[
        abilities::defender(),
        AbilityDef::static_ability(
            "Creatures with flying your opponents control get -1/-0.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasKeyword(crate::card::KeywordAbility::Flying),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Opponent,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-1),
                    ValueDef::Constant(0),
                ),
            },
        ),
    ]),
);

// ISD 231 — Runechanter's Pike
static RUNECHANTERS_PIKE_FIRST_STRIKE: AbilityDef = abilities::first_strike();

/// Your graveyard, whoever the Pike is on, and recounted continuously -- so
/// casting one more instant grows the creature mid-combat.
static RUNECHANTERS_PIKE_SPELLS: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::AnyOf(&[
        ObjectPredicateDef::HasType(CardType::Instant),
        ObjectPredicateDef::HasType(CardType::Sorcery),
    ]),
    &[ZoneKind::Graveyard],
    PlayerRelation::You,
);

static RUNECHANTERS_PIKE_BONUS: [AppliedEffectDef; 2] = [
    AppliedEffectDef::modify_power_toughness(
        ValueDef::CountMatchingObjects(&RUNECHANTERS_PIKE_SPELLS),
        ValueDef::Constant(0),
    ),
    AppliedEffectDef::add_ability(&RUNECHANTERS_PIKE_FIRST_STRIKE),
];

pub(in crate::card::sets) static RUNECHANTERS_PIKE: CardRecord = CardRecord::new_with_legacy_id(
    1927,
    "Runechanter's Pike",
    CardArt::new("0f54e38b-b4a0-4406-a635-7a5ab3722f25", "John Avon"),
    CardSet::Innistrad,
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature has first strike and gets +X/+0, where X is the number of \
                 instant and sorcery cards in your graveyard.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&RUNECHANTERS_PIKE_BONUS),
                },
            ),
            abilities::equip(
                &[AbilityCostDef::Mana(mana_cost!("{2}"))],
                "Equip {2} ({2}: Attach to target creature you control. Equip only as a \
                 sorcery.)",
            ),
        ]),
);

// ISD 232 — Sharpened Pitchfork
static SHARPENED_PITCHFORK_FIRST_STRIKE: AbilityDef = abilities::first_strike();

static SHARPENED_PITCHFORK_HUMAN: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::AttachedPermanent,
    effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)),
};

pub(in crate::card::sets) static SHARPENED_PITCHFORK: CardRecord = CardRecord::new_with_legacy_id(
    1594,
    "Sharpened Pitchfork",
    CardArt::new("4ce20f19-a159-40e6-bb67-6108872ac1e0", "Winona Nelson"),
    CardSet::Innistrad,
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature has first strike.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&SHARPENED_PITCHFORK_FIRST_STRIKE),
                },
            ),
            AbilityDef::static_ability(
                "As long as equipped creature is a Human, it gets +1/+1.",
                EffectDef::IfCondition {
                    condition: &ATTACHED_PERMANENT_IS_HUMAN,
                    then: &SHARPENED_PITCHFORK_HUMAN,
                },
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{1}"))], "Equip {1}"),
        ]),
);

// ISD 233 — Silver-Inlaid Dagger
static SILVER_INLAID_DAGGER_HUMAN: EffectDef = EffectDef::StaticApply {
    recipient: EffectRecipientDef::AttachedPermanent,
    effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(0)),
};

pub(in crate::card::sets) static SILVER_INLAID_DAGGER: CardRecord = CardRecord::new_with_legacy_id(
    1595,
    "Silver-Inlaid Dagger",
    CardArt::new("f8b8162a-68f0-45df-bb25-8fd4487257a4", "Austin Hsu"),
    CardSet::Innistrad,
    CardRules::new_artifact(mana_cost!("{1}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +2/+0.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(0),
                    ),
                },
            ),
            AbilityDef::static_ability(
                "As long as equipped creature is a Human, it gets an additional +1/+0.",
                EffectDef::IfCondition {
                    condition: &ATTACHED_PERMANENT_IS_HUMAN,
                    then: &SILVER_INLAID_DAGGER_HUMAN,
                },
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{2}"))], "Equip {2}"),
        ]),
);

// ISD 234 — Traveler's Amulet
pub(in crate::card::sets) static TRAVELERS_AMULET: CardRecord = CardRecord::new_with_legacy_id(
    959,
    "Traveler's Amulet",
    CardArt::new("a5b0afa7-e9f9-4751-af36-d85343fabc26", "Alan Pollack"),
    CardSet::Innistrad,
    CardRules::new_artifact(mana_cost!("{1}")).with_ability(AbilityDef::activated(
        "{1}, Sacrifice this artifact: Search your library for a basic land card, reveal it, put it into your hand, then shuffle.",
        &[
            AbilityCostDef::Mana(mana_cost!("{1}")),
            AbilityCostDef::SacrificeSource,
        ],
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Land),
                ObjectPredicateDef::Supertype(CardSupertype::Basic),
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
    )),
);

// ISD 235 — Trepanation Blade
static MILL_UNTIL_1: MillUntilDef = MillUntilDef {
    player: EffectRecipientDef::EventPlayer,
    object: ObjectPredicateDef::HasType(CardType::Land),
    matched_zone: ZoneKind::Graveyard,
    binding: Some(ObjectSetBindingIndex::PRIMARY),
    then: Some(&TREPANATION_BLADE_PUMP),
};

static TREPANATION_BLADE_PUMP: EffectDef = EffectDef::Apply {
    recipient: EffectRecipientDef::TriggeringObject,
    effect: AppliedEffectDef::modify_power_toughness(
        ValueDef::BoundObjectCount(ObjectSetBindingIndex::PRIMARY),
        ValueDef::Constant(0),
    ),
    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
};

pub(in crate::card::sets) static TREPANATION_BLADE: CardRecord = CardRecord::new_with_legacy_id(
    2315,
    "Trepanation Blade",
    CardArt::new(
        "2182be77-9186-4d16-a070-9577d4392999",
        "Daniel Ljunggren",
    ),
    CardSet::Innistrad,
    CardRules::new_artifact(mana_cost!("{3}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever equipped creature attacks, defending player reveals cards from the top of their library until they reveal a land card. The creature gets +1/+0 until end of turn for each card revealed this way. That player puts the revealed cards into their graveyard.",
                TriggerEventDef::attacks(ObjectPredicateDef::AttachedToSource),
                EffectDef::MillUntil(&MILL_UNTIL_1),
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{2}"))], "Equip {2}"),
        ]),
);

// ISD 236 — Witchbane Orb
// Audit: metadata-only — Needs player hexproof and identifying and destroying all Curses attached to that player.
pub(in crate::card::sets) static WITCHBANE_ORB: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("53e0bf16-62f5-4b62-96e8-bc7e049bcf89"),
    "Witchbane Orb",
    crate::card::CardArt::new("53e0bf16-62f5-4b62-96e8-bc7e049bcf89", "John Avon"),
    crate::card::CardSet::Innistrad,
    crate::card::CardRules::unsupported(),
);

// ISD 237 — Wooden Stake
pub(in crate::card::sets) static WOODEN_STAKE: CardRecord = CardRecord::new_with_legacy_id(
    2310,
    "Wooden Stake",
    CardArt::new("7e2825f5-8112-4108-910a-4303b2d57356", "David Palumbo"),
    CardSet::Innistrad,
    CardRules::new_artifact(mana_cost!("{2}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::static_ability(
                "Equipped creature gets +1/+0.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
                },
            ),
            AbilityDef::triggered(
                "Whenever equipped creature blocks or becomes blocked by a Vampire, destroy that creature. It can't be regenerated.",
                TriggerEventDef::BlocksOrBecomesBlockedBy {
                    creature: ObjectPredicateDef::AttachedToSource,
                    other: ObjectPredicateDef::Subtype("Vampire"),
                },
                EffectDef::Destroy {
                    object: EffectRecipientDef::TriggeringObject,
                    can_regenerate: false,
                    then: None,
                },
            ),
            abilities::equip(
                &[AbilityCostDef::Mana(mana_cost!("{1}"))],
                "Equip {1} ({1}: Attach to target creature you control. Equip only as a sorcery.)",
            ),
        ]),
);

// ISD 238 — Clifftop Retreat
pub(in crate::card::sets) static CLIFFTOP_RETREAT: CardRecord = CardRecord::new_with_legacy_id(
    149,
    "Clifftop Retreat",
    CardArt::new("fd7e1bf9-bd6a-48e3-9331-178e5142c06a", "John Avon"),
    CardSet::Innistrad,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::check_land_enters(
            "This land enters tapped unless you control a Mountain or a Plains.",
            &[BasicLandType::Mountain, BasicLandType::Plains],
        ),
        AbilityDef::activated_mana(
            "{T}: Add {R} or {W}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Red,
                ManaColor::White,
            ])),
        ),
    ]),
);

// ISD 239 — Gavony Township
pub(in crate::card::sets) static GAVONY_TOWNSHIP: CardRecord = CardRecord::new_with_legacy_id(
    166,
    "Gavony Township",
    CardArt::new("b5f73443-2fe8-424f-8e71-fc7ce1f3a3eb", "Peter Mohrbacher"),
    CardSet::Innistrad,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{2}{G}{W}, {T}: Put a +1/+1 counter on each creature you control.",
            &[
                AbilityCostDef::Mana(mana_cost!("{2}{G}{W}")),
                AbilityCostDef::TapSource,
            ],
            EffectDef::AddCounters {
                kind: CounterKind::PlusOnePlusOne,
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// ISD 240 — Ghost Quarter
pub(in crate::card::sets) static GHOST_QUARTER: CardRecord = CardRecord::new_with_legacy_id(
    169,
    "Ghost Quarter",
    CardArt::new("1c6456ed-0ffb-4d22-b252-5775076030ce", "Peter Mohrbacher"),
    CardSet::Innistrad,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets("{T}, Sacrifice this land: Destroy target land. Its controller may search their library for a basic land card, put it onto the battlefield, then shuffle.", &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource], &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Land),
        )], EffectDef::Sequence(&[
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    can_regenerate: true,
                    then: None,
                },
                // Declining the printed "may" skips the entire search, including
                // its shuffle. If accepted, the qualified hidden-zone search
                // may still legally fail to find. The controller is read after
                // destruction from last-known information.
                EffectDef::May {
                    player: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                    effect: &EffectDef::SearchZone {
                    player: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                    source: ZoneKind::Library,
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Supertype(CardSupertype::Basic),
                    ]),
                    minimum: 0,
                    maximum: ValueDef::Constant(1),
                    reveal: false,
                    destination: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                        enters_tapped: false,
                        attachment: None,
                        binding: None,
                        then: None,
                    },
                },
            ])),
    ]),
);

// ISD 241 — Hinterland Harbor
pub(in crate::card::sets) static HINTERLAND_HARBOR: CardRecord = CardRecord::new_with_legacy_id(
    960,
    "Hinterland Harbor",
    CardArt::new("72f15306-56fe-4643-bb4c-4c7c12378d01", "Karl Kopinski"),
    CardSet::Innistrad,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::check_land_enters(
            "This land enters tapped unless you control a Forest or an Island.",
            &[BasicLandType::Forest, BasicLandType::Island],
        ),
        AbilityDef::activated_mana(
            "{T}: Add {G} or {U}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Green,
                ManaColor::Blue,
            ])),
        ),
    ]),
);

// ISD 242 — Isolated Chapel
pub(in crate::card::sets) static ISOLATED_CHAPEL: CardRecord = CardRecord::new_with_legacy_id(
    177,
    "Isolated Chapel",
    CardArt::new("b3c1a371-5ded-4a3a-bf96-503c4f1a665d", "Cliff Childs"),
    CardSet::Innistrad,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::check_land_enters(
            "This land enters tapped unless you control a Plains or a Swamp.",
            &[BasicLandType::Plains, BasicLandType::Swamp],
        ),
        AbilityDef::activated_mana(
            "{T}: Add {W} or {B}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::White,
                ManaColor::Black,
            ])),
        ),
    ]),
);

// ISD 243 — Kessig Wolf Run
pub(in crate::card::sets) static KESSIG_WOLF_RUN: CardRecord = CardRecord::new_with_legacy_id(
    182,
    "Kessig Wolf Run",
    CardArt::new("4a8447fe-7368-470a-911a-1083ec6cc831", "Eytan Zana"),
    CardSet::Innistrad,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets(
            "{X}{R}{G}, {T}: Target creature gets +X/+0 and gains trample until end of turn.",
            &[
                AbilityCostDef::Mana(mana_cost!("{X}{R}{G}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::ChosenX,
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&abilities::trample()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ]),
        ),
    ]),
);

// ISD 244 — Moorland Haunt
pub(in crate::card::sets) static MOORLAND_HAUNT: CardRecord = CardRecord::new_with_legacy_id(
    187,
    "Moorland Haunt",
    CardArt::new("1d5569e3-278c-4cf3-860e-712010333fe6", "James Paick"),
    CardSet::Innistrad,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{W}{U}, {T}, Exile a creature card from your graveyard: Create a 1/1 white Spirit creature token with flying.",
            &[
                AbilityCostDef::Mana(mana_cost!("{W}{U}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::MoveToZone(crate::card::MoveToZoneCostDef::new(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ZoneKind::Graveyard,
                    ZoneKind::Exile,
                    1,
                )),
            ],
            EffectDef::create_creature_token(&["Spirit"], &[ManaColor::White], 1, 1).with_abilities(&[abilities::flying()]).with_art(CardArt::new("59e79ba0-33c8-46c8-8694-8bf854345fe7", "Ryan Yee")),
        ),
    ]),
);

// ISD 245 — Nephalia Drownyard
pub(in crate::card::sets) static NEPHALIA_DROWNYARD: CardRecord = CardRecord::new_with_legacy_id(
    961,
    "Nephalia Drownyard",
    CardArt::new("ef058312-6926-49f8-ae72-a8d60fedbf6c", "Cliff Childs"),
    CardSet::Innistrad,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets(
            "{1}{U}{B}, {T}: Target player mills three cards.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}{U}{B}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::Mill {
                player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
                binding: None,
                then: None,
            },
        ),
    ]),
);

// ISD 246 — Shimmering Grotto (reprint)

// ISD 247 — Stensia Bloodhall
pub(in crate::card::sets) static STENSIA_BLOODHALL: CardRecord = CardRecord::new_with_legacy_id(
    962,
    "Stensia Bloodhall",
    CardArt::new("cc2741d8-2c02-4acd-8ca2-55b4bf6aef1c", "John Avon"),
    CardSet::Innistrad,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets(
            "{3}{B}{R}, {T}: This land deals 2 damage to target player or planeswalker.",
            &[
                AbilityCostDef::Mana(mana_cost!("{3}{B}{R}")),
                AbilityCostDef::TapSource,
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(2),
            },
        ),
    ]),
);

// ISD 248 — Sulfur Falls
pub(in crate::card::sets) static SULFUR_FALLS: CardRecord = CardRecord::new_with_legacy_id(
    220,
    "Sulfur Falls",
    CardArt::new("4968b65d-50e5-4d7e-b78b-cdada1cbf7a7", "Cliff Childs"),
    CardSet::Innistrad,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::check_land_enters(
            "This land enters tapped unless you control an Island or a Mountain.",
            &[BasicLandType::Island, BasicLandType::Mountain],
        ),
        AbilityDef::activated_mana(
            "{T}: Add {U} or {R}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[ManaColor::Blue, ManaColor::Red])),
        ),
    ]),
);

// ISD 249 — Woodland Cemetery
pub(in crate::card::sets) static WOODLAND_CEMETERY: CardRecord = CardRecord::new_with_legacy_id(
    243,
    "Woodland Cemetery",
    CardArt::new("67139101-ec5e-434b-be3a-21338cc33840", "Lars Grant-West"),
    CardSet::Innistrad,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::check_land_enters(
            "This land enters tapped unless you control a Swamp or a Forest.",
            &[BasicLandType::Swamp, BasicLandType::Forest],
        ),
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
// ISD 250 — Plains (reprint)

// ISD 251 — Plains (alternate printing)

// ISD 252 — Plains (alternate printing)

// ISD 253 — Island (reprint)

// ISD 254 — Island (alternate printing)

// ISD 255 — Island (alternate printing)

// ISD 256 — Swamp (reprint)

// ISD 257 — Swamp (alternate printing)

// ISD 258 — Swamp (alternate printing)

// ISD 259 — Mountain (reprint)

// ISD 260 — Mountain (alternate printing)

// ISD 261 — Mountain (alternate printing)

// ISD 262 — Forest (reprint)

// ISD 263 — Forest (alternate printing)

// ISD 264 — Forest (alternate printing)

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ABBEY_GRIFFIN,
    &ANGEL_OF_FLIGHT_ALABASTER,
    &ANGELIC_OVERSEER,
    &AVACYNIAN_PRIEST,
    &BONDS_OF_FAITH,
    &CHAMPION_OF_THE_PARISH,
    &CHAPEL_GEIST,
    &CLOISTERED_YOUTH,
    &DEARLY_DEPARTED,
    &DIVINE_RECKONING,
    &DOOMED_TRAVELER,
    &ELDER_CATHAR,
    &ELITE_INQUISITOR,
    &FEELING_OF_DREAD,
    &FIEND_HUNTER,
    &GALLOWS_WARDEN,
    &GEIST_HONORED_MONK,
    &GHOSTLY_POSSESSION,
    &INTANGIBLE_VIRTUE,
    &MAUSOLEUM_GUARD,
    &MENTOR_OF_THE_MEEK,
    &MIDNIGHT_HAUNTING,
    &MIKAEUS_THE_LUNARCH,
    &MOMENT_OF_HEROISM,
    &NEVERMORE,
    &PARASELENE,
    &PURIFY_THE_GRAVE,
    &RALLY_THE_PEASANTS,
    &REBUKE,
    &SELFLESS_CATHAR,
    &SILVERCHASE_FOX,
    &SLAYER_OF_THE_WICKED,
    &SMITE_THE_MONSTROUS,
    &SPARE_FROM_EVIL,
    &SPECTRAL_RIDER,
    &STONY_SILENCE,
    &THRABEN_PUREBLOODS,
    &THRABEN_SENTRY,
    &UNRULY_MOB,
    &URGENT_EXORCISM,
    &VILLAGE_BELL_RINGER,
    &VOICELESS_SPIRIT,
    &ARMORED_SKAAB,
    &BACK_FROM_THE_BRINK,
    &BATTLEGROUND_GEIST,
    &CACKLING_COUNTERPART,
    &CIVILIZED_SCHOLAR,
    &CLAUSTROPHOBIA,
    &CURIOSITY,
    &CURSE_OF_THE_BLOODY_TOME,
    &DELVER_OF_SECRETS,
    &DERANGED_ASSISTANT,
    &DISSIPATE,
    &DREAM_TWIST,
    &FORBIDDEN_ALCHEMY,
    &FORTRESS_CRAB,
    &FRIGHTFUL_DELUSION,
    &GRASP_OF_PHANTOMS,
    &HYSTERICAL_BLINDNESS,
    &INVISIBLE_STALKER,
    &LABORATORY_MANIAC,
    &LANTERN_SPIRIT,
    &LOST_IN_THE_MIST,
    &LUDEVIC_S_TEST_SUBJECT,
    &MAKESHIFT_MAULER,
    &MEMORY_S_JOURNEY,
    &MINDSHRIEKER,
    &MIRROR_MAD_PHANTASM,
    &MOON_HERON,
    &MURDER_OF_CROWS,
    &ROOFTOP_STORM,
    &RUNIC_REPETITION,
    &SELHOFF_OCCULTIST,
    &SENSORY_DEPRIVATION,
    &SILENT_DEPARTURE,
    &SKAAB_GOLIATH,
    &SKAAB_RUINATOR,
    &SNAPCASTER_MAGE,
    &SPECTRAL_FLIGHT,
    &STITCHED_DRAKE,
    &STITCHERS_APPRENTICE,
    &STURMGEIST,
    &THINK_TWICE,
    &UNDEAD_ALCHEMIST,
    &ABATTOIR_GHOUL,
    &ALTARS_REAP,
    &ARMY_OF_THE_DAMNED,
    &BITTERHEART_WITCH,
    &BLOODGIFT_DEMON,
    &BLOODLINE_KEEPER,
    &BRAIN_WEEVIL,
    &BUMP_IN_THE_NIGHT,
    &CORPSE_LUNGE,
    &CURSE_OF_DEATH_S_HOLD,
    &CURSE_OF_OBLIVION,
    &DEAD_WEIGHT,
    &DIREGRAF_GHOUL,
    &DISCIPLE_OF_GRISELBRAND,
    &ENDLESS_RANKS_OF_THE_DEAD,
    &FALKENRATH_NOBLE,
    &GHOULCALLERS_CHANT,
    &GHOULRAISER,
    &GRUESOME_DEFORMITY,
    &HEARTLESS_SUMMONING,
    &LILIANA_OF_THE_VEIL,
    &MANOR_SKELETON,
    &MARKOV_PATRICIAN,
    &MAW_OF_THE_MIRE,
    &MOAN_OF_THE_UNHALLOWED,
    &MORKRUT_BANSHEE,
    &NIGHT_TERRORS,
    &REAPER_FROM_THE_ABYSS,
    &ROTTING_FENSNAKE,
    &SCREECHING_BAT,
    &SEVER_THE_BLOODLINE,
    &SKELETAL_GRIMACE,
    &SKIRSDAG_HIGH_PRIEST,
    &STROMKIRK_PATROL,
    &TRIBUTE_TO_HUNGER,
    &TYPHOID_RATS,
    &UNBREATHING_HORDE,
    &UNBURIAL_RITES,
    &VAMPIRE_INTERLOPER,
    &VICTIM_OF_NIGHT,
    &VILLAGE_CANNIBALS,
    &WALKING_CORPSE,
    &ANCIENT_GRUDGE,
    &ASHMOUTH_HOUND,
    &BALEFIRE_DRAGON,
    &BLASPHEMOUS_ACT,
    &BLOODCRAZED_NEONATE,
    &BRIMSTONE_VOLLEY,
    &BURNING_VENGEANCE,
    &CHARMBREAKER_DEVILS,
    &CROSSWAY_VAMPIRE,
    &CURSE_OF_STALKED_PREY,
    &CURSE_OF_THE_NIGHTLY_HUNT,
    &CURSE_OF_THE_PIERCED_HEART,
    &DESPERATE_RAVINGS,
    &DEVILS_PLAY,
    &FALKENRATH_MARAUDERS,
    &FERAL_RIDGEWOLF,
    &FUROR_OF_THE_BITTEN,
    &GEISTFLAME,
    &HANWEIR_WATCHKEEP,
    &HARVEST_PYRE,
    &HERETIC_S_PUNISHMENT,
    &INFERNAL_PLUNGE,
    &INSTIGATOR_GANG,
    &INTO_THE_MAW_OF_HELL,
    &KESSIG_WOLF,
    &KRUIN_OUTLAW,
    &NIGHT_REVELERS,
    &NIGHTBIRDS_CLUTCHES,
    &PAST_IN_FLAMES,
    &PITCHBURN_DEVILS,
    &RAGE_THROWER,
    &RAKISH_HEIR,
    &RECKLESS_WAIF,
    &RIOT_DEVILS,
    &ROLLING_TEMBLOR,
    &SCOURGE_OF_GEIER_REACH,
    &SKIRSDAG_CULTIST,
    &STROMKIRK_NOBLE,
    &TORMENTED_PARIAH,
    &TRAITOROUS_BLOOD,
    &VAMPIRIC_FURY,
    &VILLAGE_IRONSMITH,
    &AMBUSH_VIPER,
    &AVACYNS_PILGRIM,
    &BONEYARD_WURM,
    &BRAMBLECRUSH,
    &CARAVAN_VIGIL,
    &CREEPING_RENAISSANCE,
    &DARKTHICKET_WOLF,
    &DAYBREAK_RANGER,
    &ELDER_OF_LAURELS,
    &ESSENCE_OF_THE_WILD,
    &FESTERHIDE_BOAR,
    &FULL_MOONS_RISE,
    &GARRUK_RELENTLESS,
    &GATSTAF_SHEPHERD,
    &GNAW_TO_THE_BONE,
    &GRAVE_BRAMBLE,
    &GRIZZLED_OUTCASTS,
    &GUTTER_GRIME,
    &HAMLET_CAPTAIN,
    &HOLLOWHENGE_SCAVENGER,
    &KESSIG_CAGEBREAKERS,
    &KINDERCATCH,
    &LUMBERKNOT,
    &MAKE_A_WISH,
    &MAYOR_OF_AVABRUCK,
    &MOLDGRAF_MONSTROSITY,
    &MOONMIST,
    &MULCH,
    &ORCHARD_SPIRIT,
    &PARALLEL_LIVES,
    &PREY_UPON,
    &RANGERS_GUILE,
    &SOMBERWALD_SPIDER,
    &SPIDER_SPAWNING,
    &SPIDERY_GRASP,
    &SPLINTERFRIGHT,
    &TRAVEL_PREPARATIONS,
    &TREE_OF_REDEMPTION,
    &ULVENWALD_MYSTICS,
    &VILLAGERS_OF_ESTWALD,
    &WOODLAND_SLEUTH,
    &WREATH_OF_GEISTS,
    &EVIL_TWIN,
    &GEIST_OF_SAINT_TRAFT,
    &GRIMGRIN_CORPSE_BORN,
    &OLIVIA_VOLDAREN,
    &BUTCHERS_CLEAVER,
    &CELLAR_DOOR,
    &COBBLED_WINGS,
    &CREEPY_DOLL,
    &DEMONMAIL_HAUBERK,
    &GALVANIC_JUGGERNAUT,
    &GEISTCATCHERS_RIG,
    &GHOULCALLERS_BELL,
    &GRAVEYARD_SHOVEL,
    &GRIMOIRE_OF_THE_DEAD,
    &INQUISITOR_S_FLAIL,
    &MANOR_GARGOYLE,
    &MASK_OF_AVACYN,
    &ONE_EYED_SCARECROW,
    &RUNECHANTERS_PIKE,
    &SHARPENED_PITCHFORK,
    &SILVER_INLAID_DAGGER,
    &TRAVELERS_AMULET,
    &TREPANATION_BLADE,
    &WITCHBANE_ORB,
    &WOODEN_STAKE,
    &CLIFFTOP_RETREAT,
    &GAVONY_TOWNSHIP,
    &GHOST_QUARTER,
    &HINTERLAND_HARBOR,
    &ISOLATED_CHAPEL,
    &KESSIG_WOLF_RUN,
    &MOORLAND_HAUNT,
    &NEPHALIA_DROWNYARD,
    &STENSIA_BLOODHALL,
    &SULFUR_FALLS,
    &WOODLAND_CEMETERY,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::reprint(&onslaught::NATURALIZE), // ISD 197
    PrintingRecord::reprint(&zendikar::BLAZING_TORCH), // ISD 216
    PrintingRecord::reprint(&catalog_lrw::SHIMMERING_GROTTO), // ISD 246
    PrintingRecord::reprint(&alpha::PLAINS),         // ISD 250
    PrintingRecord::alternate(&alpha::PLAINS, 1),    // ISD 251
    PrintingRecord::alternate(&alpha::PLAINS, 2),    // ISD 252
    PrintingRecord::reprint(&alpha::ISLAND),         // ISD 253
    PrintingRecord::alternate(&alpha::ISLAND, 1),    // ISD 254
    PrintingRecord::alternate(&alpha::ISLAND, 2),    // ISD 255
    PrintingRecord::reprint(&alpha::SWAMP),          // ISD 256
    PrintingRecord::alternate(&alpha::SWAMP, 1),     // ISD 257
    PrintingRecord::alternate(&alpha::SWAMP, 2),     // ISD 258
    PrintingRecord::reprint(&alpha::MOUNTAIN),       // ISD 259
    PrintingRecord::alternate(&alpha::MOUNTAIN, 1),  // ISD 260
    PrintingRecord::alternate(&alpha::MOUNTAIN, 2),  // ISD 261
    PrintingRecord::reprint(&alpha::FOREST),         // ISD 262
    PrintingRecord::alternate(&alpha::FOREST, 1),    // ISD 263
    PrintingRecord::alternate(&alpha::FOREST, 2),    // ISD 264
];
