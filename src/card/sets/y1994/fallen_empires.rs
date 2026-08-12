use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AppliedEffectDef,
    BattlefieldEntryModificationDef, CardArt, CardBehavior, CardRules, CardSet, CounterKind,
    EffectDef, EffectDurationDef, EffectRecipientDef, ManaColor, PlayerRelation,
    ReplacementEffectDef, ValueDef, abilities, cards,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

// FEM 8a — Icatian Javelineers
pub(in crate::card::sets) static ICATIAN_JAVELINEERS: CardRecord = CardRecord::new(
    cards::ICATIAN_JAVELINEERS,
    "Icatian Javelineers",
    CardArt::new("f04b8356-2384-4743-80dd-f15ca7ec65f7", "Melissa A. Benson"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Soldier"], 1, 1).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters with a javelin counter on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::Javelin,
                    amount: 1,
                },
            ),
        ),
        AbilityDef::activated_with_targets(
            "{T}, Remove a javelin counter from this creature: It deals 1 damage to any target.",
            &[
                AbilityCostDef::TapSource,
                AbilityCostDef::RemoveCountersFromSource {
                    kind: CounterKind::Javelin,
                    amount: 1,
                },
            ],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// FEM 16a — Order of Leitbur
pub(in crate::card::sets) static ORDER_OF_LEITBUR: CardRecord = CardRecord::new(
    cards::ORDER_OF_LEITBUR,
    "Order of Leitbur",
    CardArt::new("ebd6e51e-f042-4673-a898-291607105829", "Bryon Wackwitz"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{W}{W}"), &["Human", "Cleric", "Knight"], 2, 1)
        .with_abilities(&[
            abilities::protection_from(ManaColor::Black),
            AbilityDef::activated(
                "{W}: This creature gains first strike until end of turn.",
                &[AbilityCostDef::Mana(mana_cost!("{W}"))],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::GrantAbility(&abilities::first_strike()),
                    duration: EffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::activated(
                "{W}{W}: This creature gets +1/+0 until end of turn.",
                &[AbilityCostDef::Mana(mana_cost!("{W}{W}"))],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::ModifyPowerToughness {
                        power: ValueDef::Constant(1),
                        toughness: ValueDef::Constant(0),
                    },
                    duration: EffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// FEM 38a — Hymn to Tourach
pub(in crate::card::sets) static HYMN_TO_TOURACH: CardRecord = CardRecord::new(
    cards::HYMN_TO_TOURACH,
    "Hymn to Tourach",
    CardArt::new("eb9273ea-9a41-42e3-8c9c-0d50b127a818", "Susan Van Camp"),
    CardSet::FallenEmpires,
    CardRules::new_sorcery(mana_cost!("{B}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target player discards two cards at random.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::DiscardAtRandom {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(2),
        },
    )]),
);

// FEM 42a — Order of the Ebon Hand
pub(in crate::card::sets) static ORDER_OF_THE_EBON_HAND: CardRecord = CardRecord::new(
    cards::ORDER_OF_THE_EBON_HAND,
    "Order of the Ebon Hand",
    CardArt::new("9e51f5d8-a7cc-4720-8af5-e002bcfd78a0", "Melissa A. Benson"),
    CardSet::FallenEmpires,
    CardRules::new_creature(mana_cost!("{B}{B}"), &["Cleric", "Knight"], 2, 1).with_abilities(&[
        abilities::protection_from(ManaColor::White),
        AbilityDef::activated(
            "{B}: This creature gains first strike until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::GrantAbility(&abilities::first_strike()),
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
        AbilityDef::activated(
            "{B}{B}: This creature gets +1/+0 until end of turn.",
            &[AbilityCostDef::Mana(mana_cost!("{B}{B}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::ModifyPowerToughness {
                    power: ValueDef::Constant(1),
                    toughness: ValueDef::Constant(0),
                },
                duration: EffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// FEM 56a — Goblin Grenade
pub(in crate::card::sets) static GOBLIN_GRENADE: CardRecord = CardRecord::new(
    cards::GOBLIN_GRENADE,
    "Goblin Grenade",
    CardArt::new("8837eaba-9602-4f63-9897-85583fcdcf51", "Ron Spencer"),
    CardSet::FallenEmpires,
    CardRules::new_sorcery(mana_cost!("{R}")).with_abilities(&[
        AbilityDef::custom_full(
            "As an additional cost to cast this spell, sacrifice a Goblin.\nGoblin Grenade deals 5 damage to any target.",
            CardBehavior::GoblinGrenade,
            "The additional cost, target selection, and damage are implemented by the legacy spell resolver.",
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ICATIAN_JAVELINEERS,
    &ORDER_OF_LEITBUR,
    &HYMN_TO_TOURACH,
    &ORDER_OF_THE_EBON_HAND,
    &GOBLIN_GRENADE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
