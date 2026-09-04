//! Aether Revolt cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    BattlefieldEntryModificationDef, CardArt, CardRules, CardSet, CardSupertype, CardType,
    CounterKind, CreatedTokensDef, EffectDef, EffectRecipientDef, InstalledTriggerDef, ManaColor,
    ObjectPredicateDef, ObjectSetDef, PlayerRelation, ReplacementEffectDef, TokenCharacteristics,
    TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, ZonePlacement,
    abilities,
};
use crate::ids::{ParentBinding, TargetIndex};
use crate::mana_cost;

// AER 51 — Aether Poisoner
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AETHER_POISONER: CardRecord = CardRecord::new(
    crate::card::CardSet::AetherRevolt,
    "Aether Poisoner",
    "c9b217f1-1621-40d1-8a98-24c1f7cba800",
    "Yongjae Choi",
    crate::card::CardRules::unsupported(),
);

// AER 57 — Fatal Push
static PUSH_IT: EffectDef = EffectDef::Destroy {
    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    can_regenerate: true,
    then: None,
};

pub(in crate::card::sets) static FATAL_PUSH: CardRecord = CardRecord::new(
    CardSet::AetherRevolt,
    "Fatal Push",
    "b5e81649-9954-424c-89d1-f87d73b66047",
    "Eric Deschamps",
    // One black mana answers most of what a fast deck plays, and a fetchland
    // cracked on the way in stretches it over almost everything else.
    CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
        "Destroy target creature if it has mana value 2 or less.\nRevolt — Destroy that creature \
         if it has mana value 4 or less instead if a permanent left the battlefield under your \
         control this turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        // The mana value is read as the spell resolves rather than as it is cast,
        // so anything targetable is a legal target and a creature grown too
        // expensive in between simply survives.
        EffectDef::IfElseCondition {
            condition: &TriggerConditionDef::ControllerHadPermanentLeaveThisTurn,
            then: &EffectDef::IfCondition {
                condition: &TriggerConditionDef::TargetMatches {
                    slot: TargetIndex::PRIMARY,
                    object: ObjectPredicateDef::ManaValueAtMost(4),
                },
                then: &PUSH_IT,
            },
            otherwise: &EffectDef::IfCondition {
                condition: &TriggerConditionDef::TargetMatches {
                    slot: TargetIndex::PRIMARY,
                    object: ObjectPredicateDef::ManaValueAtMost(2),
                },
                then: &PUSH_IT,
            },
        },
    )),
);

// AER 76 — Aether Chaser
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AETHER_CHASER: CardRecord = CardRecord::new(
    crate::card::CardSet::AetherRevolt,
    "Aether Chaser",
    "290cde84-d97a-4737-aff2-c443a4e43f7d",
    "Jason Rainville",
    crate::card::CardRules::unsupported(),
);

// AER 87 — Kari Zev, Skyship Raider
pub(in crate::card::sets) static KARI_ZEV_SKYSHIP_RAIDER: CardRecord = CardRecord::new(
    CardSet::AetherRevolt,
    "Kari Zev, Skyship Raider",
    "72495879-39ce-449d-ad2f-ef32ea46f3aa",
    "Brad Rigney",
    // Two mana that attacks as three power across two bodies, one of which
    // is hard to block and the other of which is gone by the second main
    // phase.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Pirate"], 1, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::first_strike(),
            abilities::menace(),
            AbilityDef::triggered(
                "Whenever Kari Zev attacks, create Ragavan, a legendary 2/1 red Monkey creature token. \
                 Ragavan enters tapped and attacking. Exile that token at end of combat.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                // A named, legendary token: Ragavan is one of the few tokens that is a
                // particular creature rather than a kind of one, which matters because two
                // Kari Zevs cannot keep two of him.
                EffectDef::create_token(TokenCharacteristics::creature(&["Monkey"], &[ManaColor::Red], 2, 1)
                        .with_name("Ragavan")
                        .with_supertype(CardSupertype::Legendary)
                        .with_art(CardArt::new(
                            "1ebc91a9-23e0-4ca1-bc6d-e710ad2efb31",
                            "Daniel Ljunggren",
                        )))
                    .entering_tapped()
                    .entering_attacking()
                    .with_created_tokens(CreatedTokensDef {
                        binding: ParentBinding,
                        // Ragavan is bound as he is made rather than found afterwards: a second
                        // attack the same turn would make another one, and the clause exiles the
                        // Monkey this attack brought.
                        then: &EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                                "Exile that token at end of combat.",
                                TriggerEventDef::StepBegins {
                                    step: TurnStepDef::EndOfCombat,
                                    player: PlayerRelation::Any,
                                },
                                EffectDef::MoveToZone {
                                    object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                        ParentBinding,
                                    )),
                                    zone: ZoneKind::Exile,
                                    placement: ZonePlacement::Top,
                                },
                            ))),
                    }),
            ),
        ]),
);

// AER 101 — Wrangle
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WRANGLE: CardRecord = CardRecord::new(
    crate::card::CardSet::AetherRevolt,
    "Wrangle",
    "5ea93a49-5a7c-4d15-8548-a57c9460e0f0",
    "Jason Rainville",
    crate::card::CardRules::unsupported(),
);

// AER 151 — Foundry Assembler
pub(in crate::card::sets) static FOUNDRY_ASSEMBLER: CardRecord = CardRecord::new(
    CardSet::AetherRevolt,
    "Foundry Assembler",
    "e83a2862-a2d7-4d87-a4b8-def9f441f5fa",
    "Karl Kopinski",
    CardRules::new_artifact_creature(mana_cost!("{5}"), &["Assembly-Worker"], 3, 3)
        .with_ability(crate::card::abilities::improvise()),
);

// AER 181 — Walking Ballista
pub(in crate::card::sets) static WALKING_BALLISTA: CardRecord = CardRecord::new(
    CardSet::AetherRevolt,
    "Walking Ballista",
    "329a8738-3e17-403a-857a-0ba529ce8cd1",
    "Daniel Ljunggren",
    // Two mana per point, which is a bad rate and never a dead card: it is
    // removal, a mana sink, and a creature, and it needs no colours at all.
    CardRules::new_artifact_creature(mana_cost!("{X}{X}"), &["Construct"], 0, 0).with_abilities(&[
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
            &[AbilityCostDef::RemoveCountersFromSource {
                kind: CounterKind::PlusOnePlusOne,
                amount: 1,
            }],
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

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &AETHER_POISONER,
    &FATAL_PUSH,
    &AETHER_CHASER,
    &KARI_ZEV_SKYSHIP_RAIDER,
    &WRANGLE,
    &FOUNDRY_ASSEMBLER,
    &WALKING_BALLISTA,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
