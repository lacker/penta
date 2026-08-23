//! Nemesis cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AlternativeCastKindDef,
    BasicLandType, BattlefieldEntryModificationDef, CardArt, CardRules, CardSet, CardSupertype,
    CardType, ComparisonDef, CounterKind, DamageEventMatcherDef, DamagePreventionDef, EffectDef,
    EffectRecipientDef, ManaColor, ObjectPredicateDef, ObjectQueryDef, ObjectRefDef,
    PlayerRelation, ReplacementEffectDef, ResolvedEffectDurationDef, SpellAdditionalCostDef,
    SpendModeDef, TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, abilities,
};
use crate::{TargetIndex, mana_cost};

/// Fading counts down rather than up: the upkeep that cannot pay a counter
/// is the one that ends the permanent. Five counters is five of its
/// controller's turns, and spending them faster is the whole point of the
/// card -- each one exiles a creature instead.
static WAVE_FADES: EffectDef = EffectDef::IfCondition {
    condition: &TriggerConditionDef::SourceCounters {
        kind: CounterKind::Fade,
        comparison: ComparisonDef::GreaterOrEqual,
        amount: 1,
    },
    then: &EffectDef::RemoveCounters {
        object: EffectRecipientDef::Source,
        kind: CounterKind::Fade,
        amount: ValueDef::Constant(1),
    },
};

/// "If you can't, sacrifice it." Checked as its own clause because the
/// removal above is what fails, and a permanent with no counters left has to
/// go rather than simply skip a turn.
static WAVE_EXPIRES: EffectDef = EffectDef::IfCondition {
    condition: &TriggerConditionDef::SourceCounters {
        kind: CounterKind::Fade,
        comparison: ComparisonDef::LessOrEqual,
        amount: 0,
    },
    then: &EffectDef::Sacrifice {
        object: EffectRecipientDef::Source,
    },
};

static WAVE_UPKEEP: EffectDef = EffectDef::Sequence(&[WAVE_EXPIRES, WAVE_FADES]);

static WAVE_EXILE_COST: [AbilityCostDef; 1] = [AbilityCostDef::RemoveCountersFromSource {
    kind: CounterKind::Fade,
    amount: 1,
}];

// NEM 17 — Parallax Wave
pub(in crate::card::sets) static PARALLAX_WAVE: CardRecord = CardRecord::new_with_legacy_id(
    2081,
    "Parallax Wave",
    CardArt::new("fb552595-ca42-4b93-9a07-395e0b674a6f", "Greg Staples"),
    CardSet::Nemesis,
    // Five creatures answered at instant speed, and then all five come back:
    // the deck playing it wants the board clear for one turn, not forever.
    CardRules::new_enchantment(mana_cost!("{2}{W}{W}")).with_abilities(&[
        AbilityDef::as_enters(
            "Fading 5 (This enchantment enters with five fade counters on it. At the beginning of your upkeep, remove a fade counter from it. If you can't, sacrifice it.)",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::Fade,
                    amount: 5,
                },
            ),
        ),
        AbilityDef::triggered(
            "At the beginning of your upkeep, remove a fade counter from this enchantment. If you can't, sacrifice it.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            WAVE_UPKEEP,
        ),
        AbilityDef::activated_with_targets(
            "Remove a fade counter from this enchantment: Exile target creature.",
            &WAVE_EXILE_COST,
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::ExileLinkedToSource {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
        AbilityDef::triggered(
            "When this enchantment leaves the battlefield, each player returns to the battlefield all cards they own exiled with it.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                None,
            ),
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

// NEM 18 — Seal of Cleansing
pub(in crate::card::sets) static SEAL_OF_CLEANSING: CardRecord = CardRecord::new_with_legacy_id(
    276,
    "Seal of Cleansing",
    CardArt::new(
        "af6c921e-1b82-412c-9979-adfdf83440f7",
        "Christopher Moeller",
    ),
    CardSet::Nemesis,
    CardRules::new_enchantment(mana_cost!("{1}{W}")).with_ability(
        AbilityDef::activated_with_targets(
            "Sacrifice this enchantment: Destroy target artifact or enchantment.",
            &[AbilityCostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                ]),
            )],
            EffectDef::destroy_target(TargetIndex::PRIMARY, true),
        ),
    ),
);

/// One Island back to hand, which is what makes the card free on turn one and
/// a real cost on turn six.
static DAZE_COST: SpellAdditionalCostDef = SpellAdditionalCostDef::new(
    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
    ZoneKind::Battlefield,
    1,
)
.spent(SpendModeDef::ReturnToHand);

static DAZE_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::Spell,
        zones: &[ZoneKind::Stack],
        controller: None,
        owner: None,
    },
)];

// NEM 30 — Daze
pub(in crate::card::sets) static DAZE: CardRecord = CardRecord::new_with_legacy_id(
    2044,
    "Daze",
    CardArt::new("d03bff25-0d5e-4dcf-8d75-6df846afea3b", "Matthew D. Wilson"),
    CardSet::Nemesis,
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Counter target spell unless its controller pays {1}.",
            &DAZE_TARGET,
            abilities::counter_target_unless_paid(ValueDef::Constant(1)),
        ),
        AbilityDef::alternative_cast(
            mana_cost!("{0}"),
            AlternativeCastKindDef::AlternativeCost,
            Some("You may return an Island you control to its owner's hand rather than pay this spell's mana cost."),
            EffectDef::None,
        )
        .with_alternative_additional_cost(&DAZE_COST),
    ]),
);

/// "If an opponent controls an Island and you control a Mountain" -- one
/// condition made of two, checked where the free cast is offered rather than
/// where it resolves.
static AN_OPPONENTS_ISLAND: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
    &[ZoneKind::Battlefield],
    PlayerRelation::Opponent,
);

static YOUR_MOUNTAIN: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Mountain]),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

static SALVAGE_WINDOW: TriggerConditionDef = TriggerConditionDef::All(&[
    TriggerConditionDef::ObjectCount {
        query: AN_OPPONENTS_ISLAND,
        comparison: ComparisonDef::GreaterOrEqual,
        amount: 1,
    },
    TriggerConditionDef::ObjectCount {
        query: YOUR_MOUNTAIN,
        comparison: ComparisonDef::GreaterOrEqual,
        amount: 1,
    },
]);

// NEM 94 — Mogg Salvage
pub(in crate::card::sets) static MOGG_SALVAGE: CardRecord = CardRecord::new_with_legacy_id(
    2047,
    "Mogg Salvage",
    CardArt::new("403aa48c-b684-4c54-8863-460958055a1f", "Paolo Parente"),
    CardSet::Nemesis,
    // Free only against the deck it was printed to beat, which is why it is a
    // sideboard card rather than a maindeck one.
    CardRules::new_instant(mana_cost!("{2}{R}")).with_abilities(&[
        AbilityDef::destroy_target(
            "Destroy target artifact.",
            &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                CardType::Artifact,
            )),
            true,
        ),
        AbilityDef::alternative_cast(
            mana_cost!("{0}"),
            AlternativeCastKindDef::AlternativeCost,
            Some("If an opponent controls an Island and you control a Mountain, you may cast this spell without paying its mana cost."),
            EffectDef::None,
        )
        .with_alternative_condition(&SALVAGE_WINDOW),
    ]),
);

// NEM 98 — Seal of Fire
pub(in crate::card::sets) static SEAL_OF_FIRE: CardRecord = CardRecord::new_with_legacy_id(
    269,
    "Seal of Fire",
    CardArt::new(
        "37eaf1f6-4bdc-4669-9a15-50b65e016ccf",
        "Christopher Moeller",
    ),
    CardSet::Nemesis,
    CardRules::new_enchantment(mana_cost!("{R}")).with_ability(AbilityDef::activated_with_targets(
        "Sacrifice this enchantment: It deals 2 damage to any target.",
        &[AbilityCostDef::SacrificeSource],
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(2),
        },
    )),
);

// NEM 102 — Blastoderm
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BLASTODERM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9db5d6c2-b11f-442a-b172-c0c99c9bec07"),
    "Blastoderm",
    crate::card::CardArt::new("9db5d6c2-b11f-442a-b172-c0c99c9bec07", "Eric Peterson"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 141 — Kor Haven
pub(in crate::card::sets) static KOR_HAVEN: CardRecord = CardRecord::new_with_legacy_id(
    308,
    "Kor Haven",
    CardArt::new("3d5529ca-5c20-4dfd-8595-96d6dfa6debe", "Darrell Riche"),
    CardSet::Nemesis,
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::tap_for(ManaColor::Colorless),
            AbilityDef::activated_with_targets(
                "{1}{W}, {T}: Prevent all combat damage that would be dealt by target attacking creature this turn.",
                &[
                    AbilityCostDef::Mana(mana_cost!("{1}{W}")),
                    AbilityCostDef::TapSource,
                ],
                &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Attacking,
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                })],
                EffectDef::PreventDamage {
                    prevention: DamagePreventionDef::unlimited(
                        DamageEventMatcherDef::combat_from(ObjectRefDef::Target(
                            TargetIndex::PRIMARY,
                        )),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &PARALLAX_WAVE,
    &SEAL_OF_CLEANSING,
    &DAZE,
    &MOGG_SALVAGE,
    &SEAL_OF_FIRE,
    &BLASTODERM,
    &KOR_HAVEN,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
