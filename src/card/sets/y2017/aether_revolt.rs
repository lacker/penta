//! Aether Revolt cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    BattlefieldEntryModificationDef, CardArt, CardRules, CardSet, CardType, CounterKind, EffectDef,
    EffectRecipientDef, ObjectPredicateDef, ReplacementEffectDef, TriggerConditionDef, ValueDef,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

static A_CREATURE: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Creature),
)];

/// The mana value is read as the spell resolves rather than as it is cast,
/// so anything targetable is a legal target and a creature grown too
/// expensive in between simply survives.
static A_SMALL_CREATURE: TriggerConditionDef = TriggerConditionDef::TargetMatches {
    slot: TargetIndex::PRIMARY,
    object: ObjectPredicateDef::ManaValueAtMost(2),
};

static A_BIGGER_CREATURE: TriggerConditionDef = TriggerConditionDef::TargetMatches {
    slot: TargetIndex::PRIMARY,
    object: ObjectPredicateDef::ManaValueAtMost(4),
};

static REVOLT: TriggerConditionDef = TriggerConditionDef::ControllerHadPermanentLeaveThisTurn;

static WITHOUT_REVOLT: [TriggerConditionDef; 2] =
    [TriggerConditionDef::Not(&REVOLT), A_SMALL_CREATURE];

static WITH_REVOLT: [TriggerConditionDef; 2] = [REVOLT, A_BIGGER_CREATURE];

static PUSH_IT: EffectDef = EffectDef::Destroy {
    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    can_regenerate: true,
};

/// The revolt clause replaces the threshold rather than adding to it, so the
/// two branches are written as the exclusive pair the card prints and only
/// one of them can ever destroy anything.
static FATAL_PUSH_EFFECT: [EffectDef; 2] = [
    EffectDef::IfCondition {
        condition: &TriggerConditionDef::All(&WITHOUT_REVOLT),
        then: &PUSH_IT,
    },
    EffectDef::IfCondition {
        condition: &TriggerConditionDef::All(&WITH_REVOLT),
        then: &PUSH_IT,
    },
];

// AER 57 — Fatal Push
pub(in crate::card::sets) static FATAL_PUSH: CardRecord = CardRecord::new_with_legacy_id(
    2233,
    "Fatal Push",
    CardArt::new("b5e81649-9954-424c-89d1-f87d73b66047", "Eric Deschamps"),
    CardSet::AetherRevolt,
    // One black mana answers most of what a fast deck plays, and a fetchland
    // cracked on the way in stretches it over almost everything else.
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target creature if it has mana value 2 or less.\nRevolt — Destroy that creature \
         if it has mana value 4 or less instead if a permanent left the battlefield under your \
         control this turn.",
        &A_CREATURE,
        EffectDef::Sequence(&FATAL_PUSH_EFFECT),
    )),
);

static ANY_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::AnyTarget,
)];

static BALLISTA_SHOOTS_COST: [AbilityCostDef; 1] = [AbilityCostDef::RemoveCountersFromSource {
    kind: CounterKind::PlusOnePlusOne,
    amount: 1,
}];

static WALKING_BALLISTA_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::as_enters(
        "This creature enters with X +1/+1 counters on it.",
        ReplacementEffectDef::ModifyBattlefieldEntry(
            BattlefieldEntryModificationDef::AddCastXCounters {
                kind: CounterKind::PlusOnePlusOne,
            },
        ),
    ),
    AbilityDef::activated(
        "{4}: Put a +1/+1 counter on this creature.",
        &[AbilityCostDef::Mana(mana_cost!("{4}"))],
        EffectDef::AddCounters {
            object: EffectRecipientDef::Source,
            kind: CounterKind::PlusOnePlusOne,
            amount: ValueDef::Constant(1),
        },
    ),
    // The counter comes off as the cost, so the last one shoots and then the
    // Ballista is a 0/0 that state-based actions clear away.
    AbilityDef::activated_with_targets(
        "Remove a +1/+1 counter from this creature: It deals 1 damage to any target.",
        &BALLISTA_SHOOTS_COST,
        &ANY_TARGET,
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(1),
        },
    ),
];

// AER 181 — Walking Ballista
pub(in crate::card::sets) static WALKING_BALLISTA: CardRecord = CardRecord::new_with_legacy_id(
    2237,
    "Walking Ballista",
    CardArt::new("329a8738-3e17-403a-857a-0ba529ce8cd1", "Daniel Ljunggren"),
    CardSet::AetherRevolt,
    // Two mana per point, which is a bad rate and never a dead card: it is
    // removal, a mana sink, and a creature, and it needs no colours at all.
    CardRules::new_artifact_creature(mana_cost!("{X}{X}"), &["Construct"], 0, 0)
        .with_abilities(&WALKING_BALLISTA_ABILITIES),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&FATAL_PUSH, &WALKING_BALLISTA];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
