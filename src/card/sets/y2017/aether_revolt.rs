//! Aether Revolt cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AppliedEffectDef,
    BattlefieldEntryModificationDef, CardArt, CardRules, CardSet, CardSupertype, CardType,
    ControlDurationDef, CounterKind, CreatedTokensDef, EffectDef, EffectPaymentCostDef,
    EffectPaymentDef, EffectRecipientDef, InstalledTriggerDef, ManaColor, ObjectPredicateDef,
    ObjectSetDef, PayOrDef, PlayerRefDef, PlayerRelation, PlayerSetDef, ReplacementEffectDef,
    ResolvedEffectDurationDef, TokenCharacteristics, TriggerConditionDef, TriggerEventDef,
    TurnStepDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::ids::{ParentBinding, TargetIndex};
use crate::mana_cost;

/// "When this creature enters, you get {E}{E} (two energy counters)." --
/// the entry both Aether artificers share, and the two energy that pay for
/// exactly one Servo before the board has to supply more.
const fn two_energy_on_enters() -> AbilityDef {
    abilities::enters_trigger(
        "When this creature enters, you get {E}{E} (two energy counters).",
        EffectDef::AddPlayerCounters {
            recipient: EffectRecipientDef::Controller,
            kind: CounterKind::named("energy"),
            amount: ValueDef::Constant(2),
        },
    )
}

/// The Servo the same two artificers buy when they attack. The payment is
/// optional because energy has other buyers: spending it here is a choice
/// made attack by attack.
const fn servo_for_two_energy_on_attack() -> AbilityDef {
    AbilityDef::triggered(
        "Whenever this creature attacks, you may pay {E}{E}. If you do, create a 1/1 colorless Servo artifact creature token.",
        TriggerEventDef::attacks(ObjectPredicateDef::Source),
        EffectDef::PayOr(PayOrDef::optional(
            EffectPaymentDef {
                payer: PlayerSetDef::One(PlayerRefDef::EffectController),
                cost: EffectPaymentCostDef::Energy(2),
            },
            &const { EffectDef::create_artifact_creature_token(&["Servo"], &[], 1, 1) },
        )),
    )
}

// AER 51 — Aether Poisoner
pub(in crate::card::sets) static AETHER_POISONER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c9b217f1-1621-40d1-8a98-24c1f7cba800"),
    "Aether Poisoner",
    CardArt::new("c9b217f1-1621-40d1-8a98-24c1f7cba800", "Yongjae Choi"),
    CardSet::AetherRevolt,
    // Deathtouch is what makes the attack safe: a 1/1 the defender cannot
    // profitably block keeps making Servos.
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Artificer"], 1, 1).with_abilities(&[
        abilities::deathtouch(),
        two_energy_on_enters(),
        servo_for_two_energy_on_attack(),
    ]),
);

// AER 57 — Fatal Push
static PUSH_IT: EffectDef = EffectDef::Destroy {
    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    can_regenerate: true,
    then: None,
};

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
pub(in crate::card::sets) static AETHER_CHASER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("290cde84-d97a-4737-aff2-c443a4e43f7d"),
    "Aether Chaser",
    CardArt::new("290cde84-d97a-4737-aff2-c443a4e43f7d", "Jason Rainville"),
    CardSet::AetherRevolt,
    // The red half of the same pair. First strike does for a 2/1 what
    // deathtouch does for the 1/1: it makes attacking into a bigger board
    // survivable often enough to keep the energy flowing.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Artificer"], 2, 1).with_abilities(&[
        abilities::first_strike(),
        two_energy_on_enters(),
        servo_for_two_energy_on_attack(),
    ]),
);

// AER 87 — Kari Zev, Skyship Raider
pub(in crate::card::sets) static KARI_ZEV_SKYSHIP_RAIDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("72495879-39ce-449d-ad2f-ef32ea46f3aa"),
    "Kari Zev, Skyship Raider",
    CardArt::new("72495879-39ce-449d-ad2f-ef32ea46f3aa", "Brad Rigney"),
    CardSet::AetherRevolt,
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
pub(in crate::card::sets) static WRANGLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5ea93a49-5a7c-4d15-8548-a57c9460e0f0"),
    "Wrangle",
    CardArt::new("5ea93a49-5a7c-4d15-8548-a57c9460e0f0", "Jason Rainville"),
    CardSet::AetherRevolt,
    // A Threaten capped at power four, which is what keeps it from simply
    // stealing the thing the opponent spent their turn on.
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Gain control of target creature with power 4 or less until end of turn. Untap that creature. It gains haste until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                // "Power 4 or less" is the complement of "power 5 or
                // greater", which is the only direction the predicate reads.
                ObjectPredicateDef::Not(&ObjectPredicateDef::PowerAtLeast(5)),
            ]),
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
                effect: AppliedEffectDef::add_ability(&abilities::haste()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )),
);

// AER 151 — Foundry Assembler
pub(in crate::card::sets) static FOUNDRY_ASSEMBLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e83a2862-a2d7-4d87-a4b8-def9f441f5fa"),
    "Foundry Assembler",
    CardArt::new("e83a2862-a2d7-4d87-a4b8-def9f441f5fa", "Karl Kopinski"),
    CardSet::AetherRevolt,
    CardRules::new_artifact_creature(mana_cost!("{5}"), &["Assembly-Worker"], 3, 3)
        .with_ability(crate::card::abilities::improvise()),
);

// AER 181 — Walking Ballista
pub(in crate::card::sets) static WALKING_BALLISTA: CardRecord = CardRecord::new_with_legacy_id(
    2237,
    "Walking Ballista",
    CardArt::new("329a8738-3e17-403a-857a-0ba529ce8cd1", "Daniel Ljunggren"),
    CardSet::AetherRevolt,
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
