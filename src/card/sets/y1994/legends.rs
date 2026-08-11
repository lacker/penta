use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    AppliedEffectDef, CardArt, CardBehavior, CardRules, CardSet, CardSupertype, CardType,
    CounterKind, EffectDef, EffectDurationDef, EffectExecutionDef, EffectRecipientDef, ManaColor,
    ObjectPredicateDef, PlayerRelation, TriggerConditionDef, TriggerEventDef, TurnStepDef,
    ValueDef, ZoneKind, abilities, cards,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

pub(in crate::card::sets) static CHAIN_LIGHTNING: CardRecord = CardRecord::new(
    cards::CHAIN_LIGHTNING,
    "Chain Lightning",
    CardArt::new("b5883762-ca0a-4932-8d2a-41a45796a5f8", "Sandra Everingham"),
    CardSet::Legends,
    CardRules::new_sorcery(mana_cost!("{R}")).with_abilities(&[
        AbilityDef::spell_with_targets("Chain Lightning deals 3 damage to any target.", &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )], EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(3),
            }),
        AbilityDef::custom_full(
            "Then that player or that permanent's controller may pay {R}{R}. If the player does, they may copy this spell and may choose a new target for that copy.",
            CardBehavior::ChainLightning,
            "The optional payment and spell-copy procedure are implemented by the card-local follow-up resolver.",
        ),
    ]),
);

pub(in crate::card::sets) static DIVINE_OFFERING: CardRecord = CardRecord::new(
    cards::DIVINE_OFFERING,
    "Divine Offering",
    CardArt::new("9c78c2f3-2f40-48ad-9dc4-55d1fa399a56", "Jeff A. Menges"),
    CardSet::Legends,
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Destroy target artifact. You gain life equal to its mana value.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Artifact),
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            },
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::TargetManaValue(TargetIndex::PRIMARY),
            },
        ]),
    )]),
);

/// The mana arrives later, so the amount is read from what the countered
/// spell was rather than from anything still on the stack.
static MANA_DRAIN_EFFECT: [EffectDef; 2] = [
    EffectDef::Counter {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        zone: ZoneKind::Graveyard,
    },
    EffectDef::AtNextStep {
        step: TurnStepDef::PrecombatMain,
        player: PlayerRelation::You,
        effect: &EffectDef::AddManaEqualTo {
            color: ManaColor::Colorless,
            amount: ValueDef::TargetManaValue(TargetIndex::PRIMARY),
        },
    },
];

pub(in crate::card::sets) static MANA_DRAIN: CardRecord = CardRecord::new(
    cards::MANA_DRAIN,
    "Mana Drain",
    CardArt::new("e691adef-3027-4e6a-889f-9f4e2df36a7c", "Mark Tedin"),
    CardSet::Legends,
    CardRules::new_instant(mana_cost!("{U}{U}"))
        .with_abilities(&[AbilityDef::spell_with_targets(
            "Counter target spell. At the beginning of your next main phase, add an amount of {C} equal to that spell's mana value.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Spell,
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            })],
            EffectDef::Sequence(&MANA_DRAIN_EFFECT),
        )]),
);

pub(in crate::card::sets) static RECALL: CardRecord = CardRecord::new(
    cards::RECALL,
    "Recall",
    CardArt::new("33296718-0625-4422-a65c-b21cf99c52ec", "Brian Snõddy"),
    CardSet::Legends,
    CardRules::new_sorcery(mana_cost!("{X}{X}{U}"))
    .with_abilities(&[AbilityDef::custom_full(
        "Discard X cards, then return a card from your graveyard to your hand for each card discarded this way. Exile Recall.",
        CardBehavior::Recall,
        "The card-local resolver discards on resolution and then returns that many cards, so a countered Recall costs nothing and the discarded cards are themselves returnable.",
    )]),
);

pub(in crate::card::sets) static SYLVAN_LIBRARY: CardRecord = CardRecord::new(
    cards::SYLVAN_LIBRARY,
    "Sylvan Library",
    CardArt::new("f486df00-7c4a-4ff0-bb0b-c8b5432ac742", "Harold McNeill"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{1}{G}"))
    .with_abilities(&[AbilityDef::triggered(
        "At the beginning of your draw step, you may draw two additional cards. If you do, choose two cards in your hand drawn this turn. For each of those cards, pay 4 life or put the card on top of your library.",
        TriggerEventDef::StepBegins {
            step: TurnStepDef::Draw,
            player: PlayerRelation::You,
        },
        EffectDef::Special("Offer the extra draws, then settle each chosen card"),
    )
    .with_effect_execution(EffectExecutionDef::Custom(CardBehavior::SylvanLibrary))
    .with_coverage(AbilityCoverageDef::explained_complete(
        "The trigger is declarative and uses the shared stack; the card-local resolver offers the draws and then the pay-or-top choice for each card drawn this turn.",
    ))]),
);

pub(in crate::card::sets) static THUNDER_SPIRIT: CardRecord = CardRecord::new(
    cards::THUNDER_SPIRIT,
    "Thunder Spirit",
    CardArt::new(
        "61a59775-b1cd-4ed0-8abf-c2b37f7be0d5",
        "Randy Asplund-Faith",
    ),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Elemental", "Spirit"], 2, 2)
        .with_abilities(&[abilities::flying(), abilities::first_strike()]),
);

static DERVISH_DREW_BLOOD: TriggerConditionDef =
    TriggerConditionDef::SourceDealtDamageToOpponentThisTurn;

pub(in crate::card::sets) static WHIRLING_DERVISH: CardRecord = CardRecord::new(
    cards::WHIRLING_DERVISH,
    "Whirling Dervish",
    CardArt::new("eba294e7-7097-4bc3-b396-72e85dd4f441", "Susan Van Camp"),
    CardSet::Legends,
    CardRules::new_creature(mana_cost!("{G}{G}"), &["Human", "Monk"], 1, 1)
        .with_abilities(&[
            abilities::protection_from(ManaColor::Black),
            AbilityDef::triggered_if(
                "At the beginning of each end step, if this creature dealt damage to an opponent this turn, put a +1/+1 counter on it.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::End,
                    player: PlayerRelation::Any,
                },
                &DERVISH_DREW_BLOOD,
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

pub(in crate::card::sets) static MOAT: CardRecord = CardRecord::new(
    cards::MOAT,
    "Moat",
    CardArt::new("952ba126-0915-47f0-9b6a-a0a6dcd22c6f", "Jeff A. Menges"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{2}{W}{W}")).with_abilities(&[AbilityDef::custom_full(
        "Creatures without flying can't attack.",
        CardBehavior::Moat,
        "The attack restriction is implemented by the legacy combat legality check.",
    )]),
);

/// "Target 1/1 creature" is read as the creature is now, so a creature that
/// has already been pumped is not one, and one that stops being 1/1 before
/// the ability resolves loses the ability with it.
static PENDELHAVEN_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::PowerExactly(1),
            ObjectPredicateDef::ToughnessExactly(1),
        ]),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
)];

pub(in crate::card::sets) static PENDELHAVEN: CardRecord = CardRecord::new(
    cards::PENDELHAVEN,
    "Pendelhaven",
    CardArt::new("79427109-c1f3-476d-a029-0049217237b5", "Bryon Wackwitz"),
    CardSet::Legends,
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::tap_for(ManaColor::Green),
            AbilityDef::activated_with_targets(
                "{T}: Target 1/1 creature gets +1/+2 until end of turn.",
                &[AbilityCostDef::TapSource],
                &PENDELHAVEN_TARGET,
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(1),
                        toughness: ValueDef::Constant(2),
                    },
                    duration: EffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

pub(in crate::card::sets) static RELIC_BARRIER: CardRecord = CardRecord::new(
    cards::RELIC_BARRIER,
    "Relic Barrier",
    CardArt::new("c062cbae-ce5e-43be-9932-c81a0a3622e8", "Harold McNeill"),
    CardSet::Legends,
    CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}: Tap target artifact.",
            &[AbilityCostDef::TapSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Artifact),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ]),
);

pub(in crate::card::sets) static THE_ABYSS: CardRecord = CardRecord::new(
    cards::THE_ABYSS,
    "The Abyss",
    CardArt::new("86a27d68-3e58-4ade-976d-36381beed451", "Pete Venters"),
    CardSet::Legends,
    CardRules::new_enchantment(mana_cost!("{3}{B}"))
        .with_supertype(CardSupertype::World)
        .with_abilities(&[AbilityDef::triggered(
            "At the beginning of each player's upkeep, destroy target nonartifact creature that player controls of their choice. It can't be regenerated.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::Any,
            },
            EffectDef::DestroyOfChoice {
                player: EffectRecipientDef::EventPlayer,
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Artifact)),
                ]),
                can_regenerate: false,
            },
        )]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &CHAIN_LIGHTNING,
    &DIVINE_OFFERING,
    &MANA_DRAIN,
    &RECALL,
    &SYLVAN_LIBRARY,
    &THUNDER_SPIRIT,
    &WHIRLING_DERVISH,
    &MOAT,
    &PENDELHAVEN,
    &RELIC_BARRIER,
    &THE_ABYSS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
