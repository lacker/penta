//! Nemesis cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::CostQuantityDef;
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AlternativeCastKindDef, AppliedEffectDef,
    AppliedRuleDef, BasicLandType, BattlefieldEntryModificationDef, CardArt, CardRules, CardSet,
    CardSupertype, CardType, ComparisonDef, CostDef, CounterKind, DamageEventMatcherDef,
    DamagePreventionDef, EffectDef, EffectRecipientDef, KeywordAbility, ManaColor,
    ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, PlayerRelation, PlayerSetDef,
    ReplacementEffectDef, ResolvedEffectDurationDef, TriggerConditionDef, TriggerEventDef,
    TurnStepDef, ValueDef, ZoneKind, abilities,
};
use crate::{TargetIndex, mana_cost};

// NEM 1 — Angelic Favor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANGELIC_FAVOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("871ad2f3-1dd2-45ea-881d-529aad3b76ec"),
    "Angelic Favor",
    crate::card::CardArt::new("871ad2f3-1dd2-45ea-881d-529aad3b76ec", "Paolo Parente"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 2 — Avenger en-Dal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVENGER_EN_DAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fcf6f711-c0bc-4e12-b9d0-41581924e13c"),
    "Avenger en-Dal",
    crate::card::CardArt::new("fcf6f711-c0bc-4e12-b9d0-41581924e13c", "Ron Spencer"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 3 — Blinding Angel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLINDING_ANGEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("48c25553-6554-4e31-9012-c50da1f0a171"),
    "Blinding Angel",
    crate::card::CardArt::new("48c25553-6554-4e31-9012-c50da1f0a171", "Todd Lockwood"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 4 — Chieftain en-Dal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHIEFTAIN_EN_DAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0c1f49bc-d144-466f-8795-c0dae7afdc10"),
    "Chieftain en-Dal",
    crate::card::CardArt::new("0c1f49bc-d144-466f-8795-c0dae7afdc10", "Dany Orizio"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 5 — Defender en-Vec
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEFENDER_EN_VEC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e7b224b7-d5f7-4515-beca-523f305ee3b8"),
    "Defender en-Vec",
    crate::card::CardArt::new("e7b224b7-d5f7-4515-beca-523f305ee3b8", "Bradley Williams"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 6 — Defiant Falcon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEFIANT_FALCON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4c80032b-daeb-4661-9a66-61abe9d12ddd"),
    "Defiant Falcon",
    crate::card::CardArt::new("4c80032b-daeb-4661-9a66-61abe9d12ddd", "Heather Hudson"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 7 — Defiant Vanguard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEFIANT_VANGUARD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4c0bd267-59ec-41df-b0b7-37f6e6d6b073"),
    "Defiant Vanguard",
    crate::card::CardArt::new("4c0bd267-59ec-41df-b0b7-37f6e6d6b073", "Pete Venters"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 8 — Fanatical Devotion
pub(in crate::card::sets) static FANATICAL_DEVOTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("be0ed1fb-d380-4e3e-a43f-c39660a996e9"),
    "Fanatical Devotion",
    CardArt::new(
        "be0ed1fb-d380-4e3e-a43f-c39660a996e9",
        "Massimiliano Frezzato",
    ),
    CardSet::Nemesis,
    // A free sacrifice outlet dressed as protection: the creature saved
    // is rarely the point.
    CardRules::new_enchantment(mana_cost!("{2}{W}")).with_ability(
        AbilityDef::activated_with_targets(
            "Sacrifice a creature: Regenerate target creature.",
            &[CostDef::SacrificePermanent {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                controller: PlayerRelation::You,
            }],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Regenerate {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ),
);

// NEM 9 — Lashknife
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LASHKNIFE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fd7451cc-4126-4518-a103-2558fa81323f"),
    "Lashknife",
    crate::card::CardArt::new("fd7451cc-4126-4518-a103-2558fa81323f", "Hannibal King"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 10 — Lawbringer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAWBRINGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2d76b7e3-6890-4120-8575-732909c8bdff"),
    "Lawbringer",
    crate::card::CardArt::new("2d76b7e3-6890-4120-8575-732909c8bdff", "Matt Cavotta"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 11 — Lightbringer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIGHTBRINGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("19451993-7a53-4a50-bfca-ddc9cdfbe168"),
    "Lightbringer",
    crate::card::CardArt::new("19451993-7a53-4a50-bfca-ddc9cdfbe168", "Paolo Parente"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 12 — Lin Sivvi, Defiant Hero
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIN_SIVVI_DEFIANT_HERO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e574e522-2632-4cd4-8545-c582ac3b641f"),
    "Lin Sivvi, Defiant Hero",
    crate::card::CardArt::new("e574e522-2632-4cd4-8545-c582ac3b641f", "rk post"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 13 — Netter en-Dal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NETTER_EN_DAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2190649d-f898-4693-8ccf-80e6709d8496"),
    "Netter en-Dal",
    crate::card::CardArt::new("2190649d-f898-4693-8ccf-80e6709d8496", "Matt Cavotta"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 14 — Noble Stand
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NOBLE_STAND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5f53ab12-7c16-43b1-b9f9-a5e523cf431b"),
    "Noble Stand",
    crate::card::CardArt::new(
        "5f53ab12-7c16-43b1-b9f9-a5e523cf431b",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 15 — Off Balance
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OFF_BALANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("adafe5c4-8de0-4d38-919f-de96bc70c21b"),
    "Off Balance",
    crate::card::CardArt::new("adafe5c4-8de0-4d38-919f-de96bc70c21b", "Jeff Miracola"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 16 — Oracle's Attendants
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORACLE_S_ATTENDANTS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e2e0ea3e-9826-408d-835b-18dfecaac8af"),
    "Oracle's Attendants",
    crate::card::CardArt::new("e2e0ea3e-9826-408d-835b-18dfecaac8af", "Dany Orizio"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

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
                    kind: CounterKind::named("fade"),
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
            // Fading counts down rather than up: the upkeep that cannot pay a counter
            // is the one that ends the permanent. Five counters is five of its
            // controller's turns, and spending them faster is the whole point of the
            // card -- each one exiles a creature instead.
            // "If you can't, sacrifice it." Checked as its own clause because the
            // removal above is what fails, and a permanent with no counters left has to
            // go rather than simply skip a turn.
            EffectDef::IfElseCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("fade"),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                },
                then: &EffectDef::RemoveCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::named("fade"),
                    amount: ValueDef::Constant(1),
                },
                otherwise: &EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            },
        ),
        AbilityDef::activated_with_targets(
            "Remove a fade counter from this enchantment: Exile target creature.",
            &[CostDef::RemoveCountersFromSource {
                kind: CounterKind::named("fade"),
                amount: 1,
            }],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::ExileLinkedToSource {
                until_source_leaves: false,
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
face_down: false,
then: None,
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
            &[CostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                ]),
            )],
            EffectDef::destroy_target(TargetIndex::PRIMARY),
        ),
    ),
);

// NEM 19 — Silkenfist Fighter
pub(in crate::card::sets) static SILKENFIST_FIGHTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3480efc4-1078-4c63-a94c-d00a7507f6b1"),
    "Silkenfist Fighter",
    CardArt::new("3480efc4-1078-4c63-a94c-d00a7507f6b1", "Mark Brill"),
    CardSet::Nemesis,
    // Untapping mid-combat does not remove it from combat: it still fights,
    // and it is untapped afterwards to block.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Kor", "Soldier"], 1, 3).with_ability(
        AbilityDef::triggered(
            "Whenever this creature becomes blocked, untap it.",
            TriggerEventDef::BecomesBlocked(ObjectPredicateDef::Source),
            EffectDef::Untap {
                object: EffectRecipientDef::Source,
            },
        ),
    ),
);

// NEM 20 — Silkenfist Order
pub(in crate::card::sets) static SILKENFIST_ORDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("93741517-90ed-46fe-a505-fe6299f188bf"),
    "Silkenfist Order",
    CardArt::new(
        "93741517-90ed-46fe-a505-fe6299f188bf",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    CardSet::Nemesis,
    // The larger version of the same trick.
    CardRules::new_creature(mana_cost!("{3}{W}{W}"), &["Kor", "Soldier"], 3, 5).with_ability(
        AbilityDef::triggered(
            "Whenever this creature becomes blocked, untap it.",
            TriggerEventDef::BecomesBlocked(ObjectPredicateDef::Source),
            EffectDef::Untap {
                object: EffectRecipientDef::Source,
            },
        ),
    ),
);

// NEM 21 — Sivvi's Ruse
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SIVVI_S_RUSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("132112a0-0fb0-4a80-927d-39d34cf10159"),
    "Sivvi's Ruse",
    crate::card::CardArt::new("132112a0-0fb0-4a80-927d-39d34cf10159", "Kev Walker"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 22 — Sivvi's Valor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SIVVI_S_VALOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9d15f7b5-5070-4742-a05c-623822d874fb"),
    "Sivvi's Valor",
    crate::card::CardArt::new("9d15f7b5-5070-4742-a05c-623822d874fb", "Jeff Miracola"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 23 — Spiritual Asylum
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIRITUAL_ASYLUM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e5eea354-2d92-4b57-aec7-25260ab7a70f"),
    "Spiritual Asylum",
    crate::card::CardArt::new("e5eea354-2d92-4b57-aec7-25260ab7a70f", "Matt Cavotta"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 24 — Topple
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOPPLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a7c25c67-4214-4318-a718-7d351e713f80"),
    "Topple",
    crate::card::CardArt::new("a7c25c67-4214-4318-a718-7d351e713f80", "Daren Bader"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 25 — Voice of Truth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VOICE_OF_TRUTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("40377e3d-77d9-4d86-ac8c-4e27803e48d8"),
    "Voice of Truth",
    crate::card::CardArt::new("40377e3d-77d9-4d86-ac8c-4e27803e48d8", "rk post"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 26 — Accumulated Knowledge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ACCUMULATED_KNOWLEDGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ab061406-38f4-40e7-a9ea-e3cbcaabc127"),
    "Accumulated Knowledge",
    crate::card::CardArt::new("ab061406-38f4-40e7-a9ea-e3cbcaabc127", "Randy Gallegos"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 27 — Aether Barrier
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AETHER_BARRIER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("36298f9f-12cc-43bd-adda-ccabd67a9568"),
    "Aether Barrier",
    crate::card::CardArt::new("36298f9f-12cc-43bd-adda-ccabd67a9568", "David Martin"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 28 — Air Bladder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AIR_BLADDER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a7363c6f-53a3-4f41-b451-8120bc24f1ee"),
    "Air Bladder",
    crate::card::CardArt::new("a7363c6f-53a3-4f41-b451-8120bc24f1ee", "Donato Giancola"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 29 — Cloudskate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLOUDSKATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e7f97e50-3aeb-4c79-81b1-505a2f32d8ac"),
    "Cloudskate",
    crate::card::CardArt::new("e7f97e50-3aeb-4c79-81b1-505a2f32d8ac", "Carl Critchlow"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 30 — Daze
pub(in crate::card::sets) static DAZE: CardRecord = CardRecord::new_with_legacy_id(
    2044,
    "Daze",
    CardArt::new("d03bff25-0d5e-4dcf-8d75-6df846afea3b", "Matthew D. Wilson"),
    CardSet::Nemesis,
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Counter target spell unless its controller pays {1}.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Spell,
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                },
            )],
            abilities::counter_target_unless_paid(ValueDef::Constant(1)),
        ),
        AbilityDef::alternative_cast(
            mana_cost!("{0}"),
            AlternativeCastKindDef::AlternativeCost,
            Some("You may return an Island you control to its owner's hand rather than pay this spell's mana cost."),
            EffectDef::None,
        )
        // One Island back to hand, which is what makes the card free on turn one and
        // a real cost on turn six.
        .with_alternative_additional_cost(&CostDef::return_to_hand(
            ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
            CostQuantityDef::Fixed(1),
        )),
    ]),
);

// NEM 31 — Dominate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DOMINATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("63b2dcb1-8c3e-434c-865a-196d4d799706"),
    "Dominate",
    crate::card::CardArt::new("63b2dcb1-8c3e-434c-865a-196d4d799706", "Scott Hampton"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 32 — Ensnare
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENSNARE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("055b344a-4eb1-4579-ac50-973b18e12fad"),
    "Ensnare",
    crate::card::CardArt::new("055b344a-4eb1-4579-ac50-973b18e12fad", "Gao Yan"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 33 — Infiltrate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static INFILTRATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c549b817-8ad6-44d0-9761-5e4ff9e62c71"),
    "Infiltrate",
    crate::card::CardArt::new("c549b817-8ad6-44d0-9761-5e4ff9e62c71", "Nelson DeCastro"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 34 — Jolting Merfolk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JOLTING_MERFOLK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8b4d1c74-8b73-445c-b226-349c57a972f6"),
    "Jolting Merfolk",
    crate::card::CardArt::new("8b4d1c74-8b73-445c-b226-349c57a972f6", "Glen Angus"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 35 — Oraxid
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORAXID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6c05609a-f32d-4454-af24-a24452997dcb"),
    "Oraxid",
    crate::card::CardArt::new("6c05609a-f32d-4454-af24-a24452997dcb", "Dave Dorman"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 36 — Pale Moon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PALE_MOON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("aeb282bb-d0b8-4822-8197-ff0523549309"),
    "Pale Moon",
    crate::card::CardArt::new("aeb282bb-d0b8-4822-8197-ff0523549309", "Pete Venters"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 37 — Parallax Tide
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PARALLAX_TIDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7fe593eb-df3c-43e5-97a6-418f91e87cb3"),
    "Parallax Tide",
    crate::card::CardArt::new("7fe593eb-df3c-43e5-97a6-418f91e87cb3", "Carl Critchlow"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 38 — Rising Waters
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RISING_WATERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ec9c84db-cf45-43e1-b38f-8bbf53cf088b"),
    "Rising Waters",
    crate::card::CardArt::new("ec9c84db-cf45-43e1-b38f-8bbf53cf088b", "Scott M. Fischer"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 39 — Rootwater Commando
pub(in crate::card::sets) static ROOTWATER_COMMANDO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8e86f36d-584d-49b2-8c66-19c262408950"),
    "Rootwater Commando",
    CardArt::new("8e86f36d-584d-49b2-8c66-19c262408950", "Mark Tedin"),
    CardSet::Nemesis,
    // The blue mirror-breaker: three mana for two damage a turn that the
    // other blue deck cannot block.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Merfolk"], 2, 2)
        .with_ability(abilities::landwalk(BasicLandType::Island)),
);

// NEM 40 — Rootwater Thief
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROOTWATER_THIEF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("38addef3-1dd7-41a1-9706-3be5c86a58c9"),
    "Rootwater Thief",
    crate::card::CardArt::new("38addef3-1dd7-41a1-9706-3be5c86a58c9", "Ron Spears"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 41 — Seahunter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEAHUNTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c375f65a-6d88-4d3c-a7a7-8c7a5cc5807f"),
    "Seahunter",
    crate::card::CardArt::new("c375f65a-6d88-4d3c-a7a7-8c7a5cc5807f", "Heather Hudson"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 42 — Seal of Removal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEAL_OF_REMOVAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("487becfe-a9b1-4029-a487-2a32561570cb"),
    "Seal of Removal",
    crate::card::CardArt::new(
        "487becfe-a9b1-4029-a487-2a32561570cb",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 43 — Sliptide Serpent
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SLIPTIDE_SERPENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f8647649-5669-46c4-8840-9ff967fabd99"),
    "Sliptide Serpent",
    crate::card::CardArt::new("f8647649-5669-46c4-8840-9ff967fabd99", "Daren Bader"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 44 — Sneaky Homunculus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SNEAKY_HOMUNCULUS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e1b2dadb-4ce3-4f7e-9ca7-79757543f04d"),
    "Sneaky Homunculus",
    crate::card::CardArt::new("e1b2dadb-4ce3-4f7e-9ca7-79757543f04d", "Scott M. Fischer"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 45 — Stronghold Biologist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STRONGHOLD_BIOLOGIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6215a5d9-d6d2-4f9f-8a0c-a65d1afd956a"),
    "Stronghold Biologist",
    crate::card::CardArt::new("6215a5d9-d6d2-4f9f-8a0c-a65d1afd956a", "Terese Nielsen"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 46 — Stronghold Machinist
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STRONGHOLD_MACHINIST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d3567b4e-6e31-40b0-83ea-4a2a58bd637c"),
    "Stronghold Machinist",
    crate::card::CardArt::new("d3567b4e-6e31-40b0-83ea-4a2a58bd637c", "Terese Nielsen"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 47 — Stronghold Zeppelin
pub(in crate::card::sets) static STRONGHOLD_ZEPPELIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d672110d-b7c4-4233-9c46-73323be7204d"),
    "Stronghold Zeppelin",
    CardArt::new("d672110d-b7c4-4233-9c46-73323be7204d", "Arnie Swekel"),
    CardSet::Nemesis,
    // A 3/3 flier for four with a real drawback, from a set that was
    // pricing evasion carefully.
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Human"], 3, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "This creature can block only creatures with flying.",
            // A restriction on the blocker rather than the attacker, so
            // it stops this creature from blocking on the ground without
            // saying anything about who may block it.
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::can_block_only(
                    ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                )),
            },
        ),
    ]),
);

// NEM 48 — Submerge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SUBMERGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d2741fe4-37fe-427f-ae85-5107991d4eee"),
    "Submerge",
    crate::card::CardArt::new("d2741fe4-37fe-427f-ae85-5107991d4eee", "Mark Romanoski"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 49 — Trickster Mage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRICKSTER_MAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b31b2e24-1a70-48bd-8946-ff29e12c6f3d"),
    "Trickster Mage",
    crate::card::CardArt::new("b31b2e24-1a70-48bd-8946-ff29e12c6f3d", "Alan Rabinowitz"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 50 — Wandering Eye
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WANDERING_EYE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2869efd2-060f-4af3-b0dc-b7dc5e1143b8"),
    "Wandering Eye",
    crate::card::CardArt::new("2869efd2-060f-4af3-b0dc-b7dc5e1143b8", "Sam Wood"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 51 — Ascendant Evincar
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ASCENDANT_EVINCAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e5c87c93-8cf4-4d1a-9bb8-349600da55bc"),
    "Ascendant Evincar",
    crate::card::CardArt::new("e5c87c93-8cf4-4d1a-9bb8-349600da55bc", "Mark Zug"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 52 — Battlefield Percher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BATTLEFIELD_PERCHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f1ebf021-02a1-4a47-b581-c85a7a76cdec"),
    "Battlefield Percher",
    crate::card::CardArt::new(
        "f1ebf021-02a1-4a47-b581-c85a7a76cdec",
        "Edward P. Beard, Jr.",
    ),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 53 — Belbe's Percher
pub(in crate::card::sets) static BELBE_S_PERCHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d95dcb2e-8945-47dd-ad40-b5bdcc3ea742"),
    "Belbe's Percher",
    CardArt::new(
        "d95dcb2e-8945-47dd-ad40-b5bdcc3ea742",
        "Edward P. Beard, Jr.",
    ),
    CardSet::Nemesis,
    // Black's printing of the same deal: a 2/2 flier that cannot come down
    // to block the ground.
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Bird"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "This creature can block only creatures with flying.",
            // A restriction on the blocker rather than the attacker, so
            // it stops this creature from blocking on the ground without
            // saying anything about who may block it.
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::can_block_only(
                    ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                )),
            },
        ),
    ]),
);

// NEM 54 — Carrion Wall
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CARRION_WALL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("61d6fa78-3422-4ace-88ab-e985c558cba7"),
    "Carrion Wall",
    crate::card::CardArt::new("61d6fa78-3422-4ace-88ab-e985c558cba7", "Tony Szczudlo"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 55 — Dark Triumph
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARK_TRIUMPH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("794657cd-b292-41d7-a4a6-3f3dd20dc07a"),
    "Dark Triumph",
    crate::card::CardArt::new("794657cd-b292-41d7-a4a6-3f3dd20dc07a", "Adam Rex"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 56 — Death Pit Offering
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEATH_PIT_OFFERING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6223e583-8ef6-4d93-8ed0-3ccf4488f166"),
    "Death Pit Offering",
    crate::card::CardArt::new("6223e583-8ef6-4d93-8ed0-3ccf4488f166", "Pete Venters"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 57 — Divining Witch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIVINING_WITCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("be981eef-9dd2-4233-82c9-03f9f2e82c59"),
    "Divining Witch",
    crate::card::CardArt::new("be981eef-9dd2-4233-82c9-03f9f2e82c59", "Donato Giancola"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 58 — Massacre
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MASSACRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f05f5d93-50d1-4aa6-af05-383a6808345b"),
    "Massacre",
    crate::card::CardArt::new("f05f5d93-50d1-4aa6-af05-383a6808345b", "Pete Venters"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 59 — Mind Slash
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIND_SLASH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8bece38b-e09e-4666-95b6-5e5b05867cd5"),
    "Mind Slash",
    crate::card::CardArt::new("8bece38b-e09e-4666-95b6-5e5b05867cd5", "Adam Rex"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 60 — Mind Swords
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIND_SWORDS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3d6d91df-008b-48f2-a84f-550702fbcdb3"),
    "Mind Swords",
    crate::card::CardArt::new("3d6d91df-008b-48f2-a84f-550702fbcdb3", "Daren Bader"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 61 — Murderous Betrayal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MURDEROUS_BETRAYAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f13a3ed0-aa57-4082-b6b0-b1078c93c0b2"),
    "Murderous Betrayal",
    crate::card::CardArt::new("f13a3ed0-aa57-4082-b6b0-b1078c93c0b2", "Randy Gallegos"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 62 — Parallax Dementia
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PARALLAX_DEMENTIA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("154789ac-bbea-467b-9655-76f378a53f40"),
    "Parallax Dementia",
    crate::card::CardArt::new("154789ac-bbea-467b-9655-76f378a53f40", "Eric Peterson"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 63 — Parallax Nexus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PARALLAX_NEXUS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("862c50c7-0840-46e0-a653-5b660fdfd4bd"),
    "Parallax Nexus",
    crate::card::CardArt::new("862c50c7-0840-46e0-a653-5b660fdfd4bd", "Greg Staples"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 64 — Phyrexian Driver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_DRIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("efeef7f3-5b87-440d-a851-95ae2bdb840d"),
    "Phyrexian Driver",
    crate::card::CardArt::new("efeef7f3-5b87-440d-a851-95ae2bdb840d", "Chippy"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 65 — Phyrexian Prowler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_PROWLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e26f79b5-780a-4cbb-b49d-01673a411d1f"),
    "Phyrexian Prowler",
    crate::card::CardArt::new("e26f79b5-780a-4cbb-b49d-01673a411d1f", "Mark Zug"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 66 — Plague Witch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PLAGUE_WITCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ca3614c2-39b4-4f67-adab-373dcb9e4553"),
    "Plague Witch",
    crate::card::CardArt::new("ca3614c2-39b4-4f67-adab-373dcb9e4553", "Nelson DeCastro"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 67 — Rathi Assassin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RATHI_ASSASSIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5a81c130-d7ec-4914-89e3-4769743837aa"),
    "Rathi Assassin",
    crate::card::CardArt::new("3e3597c3-3053-49f8-ab7e-a774e2fb082f", "Dana Knutson"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 68 — Rathi Fiend
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RATHI_FIEND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("07ca1184-ade0-4d6d-87f9-ad17f37679b3"),
    "Rathi Fiend",
    crate::card::CardArt::new("07ca1184-ade0-4d6d-87f9-ad17f37679b3", "Mark Tedin"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 69 — Rathi Intimidator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RATHI_INTIMIDATOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("6fc59fa5-144f-49e6-b6bd-2ba6d3f2eff2"),
    "Rathi Intimidator",
    crate::card::CardArt::new("6fc59fa5-144f-49e6-b6bd-2ba6d3f2eff2", "Mike Ploog"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 70 — Seal of Doom
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEAL_OF_DOOM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("396d9f58-a4ca-4197-94be-0f115427224e"),
    "Seal of Doom",
    crate::card::CardArt::new(
        "396d9f58-a4ca-4197-94be-0f115427224e",
        "Christopher Moeller",
    ),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 71 — Spineless Thug
pub(in crate::card::sets) static SPINELESS_THUG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e4a9bb47-3855-425d-924c-09dbde74735b"),
    "Spineless Thug",
    CardArt::new("e4a9bb47-3855-425d-924c-09dbde74735b", "Matthew D. Wilson"),
    CardSet::Nemesis,
    // A 2/2 for two with the smallest possible drawback, printed for the
    // deck that only attacks.
    CardRules::new_creature(
        mana_cost!("{1}{B}"),
        &["Phyrexian", "Zombie", "Mercenary"],
        2,
        2,
    )
    .with_ability(AbilityDef::static_ability(
        "This creature can't block.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
        },
    )),
);

// NEM 72 — Spiteful Bully
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPITEFUL_BULLY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5535d14a-7126-4a94-96a0-e17ad5c72070"),
    "Spiteful Bully",
    crate::card::CardArt::new("5535d14a-7126-4a94-96a0-e17ad5c72070", "Chippy"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 73 — Stronghold Discipline
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STRONGHOLD_DISCIPLINE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("46fa5472-5341-47cd-884e-fe2fcca12c0d"),
    "Stronghold Discipline",
    crate::card::CardArt::new("46fa5472-5341-47cd-884e-fe2fcca12c0d", "Li Tie"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 74 — Vicious Hunger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VICIOUS_HUNGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ccaff6a0-7831-45db-a50f-881c6cb7ce49"),
    "Vicious Hunger",
    crate::card::CardArt::new(
        "ccaff6a0-7831-45db-a50f-881c6cb7ce49",
        "Massimiliano Frezzato",
    ),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 75 — Volrath the Fallen
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VOLRATH_THE_FALLEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("08bdd66e-9ca1-456e-a61c-7c96cf6f7c56"),
    "Volrath the Fallen",
    crate::card::CardArt::new("08bdd66e-9ca1-456e-a61c-7c96cf6f7c56", "Kev Walker"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 76 — Ancient Hydra
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANCIENT_HYDRA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5de57c84-38b9-4606-b934-0ab270496582"),
    "Ancient Hydra",
    crate::card::CardArt::new("5de57c84-38b9-4606-b934-0ab270496582", "Scott M. Fischer"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 77 — Arc Mage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARC_MAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("62982dab-4c27-45b3-9740-38fec3df7226"),
    "Arc Mage",
    crate::card::CardArt::new("62982dab-4c27-45b3-9740-38fec3df7226", "Terese Nielsen"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 78 — Bola Warrior
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BOLA_WARRIOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("dc6e1de6-e7e0-4037-896a-f80c54b8ef5c"),
    "Bola Warrior",
    crate::card::CardArt::new("dc6e1de6-e7e0-4037-896a-f80c54b8ef5c", "Adam Rex"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 79 — Downhill Charge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DOWNHILL_CHARGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2ebfc91c-d764-4e63-a428-82704f8bf1fd"),
    "Downhill Charge",
    crate::card::CardArt::new(
        "2ebfc91c-d764-4e63-a428-82704f8bf1fd",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 80 — Flame Rift
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLAME_RIFT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7717eeb9-c457-4a65-93a0-e91c7f6a1970"),
    "Flame Rift",
    crate::card::CardArt::new("7717eeb9-c457-4a65-93a0-e91c7f6a1970", "Ben Thompson"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 81 — Flowstone Crusher
pub(in crate::card::sets) static FLOWSTONE_CRUSHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c93f0066-1ff0-4e52-9959-9eb0def60957"),
    "Flowstone Crusher",
    CardArt::new("c93f0066-1ff0-4e52-9959-9eb0def60957", "Ben Thompson"),
    CardSet::Nemesis,
    // A gentler version of the same trade on a bigger body, so it survives
    // more of the pumps it makes.
    CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Beast"], 4, 4).with_ability(
        AbilityDef::activated(
            "{R}: This creature gets +1/-1 until end of turn.",
            &[CostDef::Mana(mana_cost!("{R}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(-1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// NEM 82 — Flowstone Overseer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLOWSTONE_OVERSEER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3e644ab8-3cc3-413d-a918-44fc636087ae"),
    "Flowstone Overseer",
    crate::card::CardArt::new("3e644ab8-3cc3-413d-a918-44fc636087ae", "Andrew Goldhawk"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 83 — Flowstone Slide
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLOWSTONE_SLIDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ec7b02e1-0a20-4247-ae2a-056c5356f168"),
    "Flowstone Slide",
    crate::card::CardArt::new("ec7b02e1-0a20-4247-ae2a-056c5356f168", "Chippy"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 84 — Flowstone Strike
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLOWSTONE_STRIKE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a1203053-0829-4f46-a361-62cb9cd17280"),
    "Flowstone Strike",
    crate::card::CardArt::new("a1203053-0829-4f46-a361-62cb9cd17280", "Mike Ploog"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 85 — Flowstone Surge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLOWSTONE_SURGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bc450922-0bbf-46c4-9955-79f4d41ee488"),
    "Flowstone Surge",
    crate::card::CardArt::new("bc450922-0bbf-46c4-9955-79f4d41ee488", "Scott Hampton"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 86 — Flowstone Wall
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLOWSTONE_WALL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("89844c2f-f0a4-41c6-ad8c-d559fcaec85c"),
    "Flowstone Wall",
    crate::card::CardArt::new("89844c2f-f0a4-41c6-ad8c-d559fcaec85c", "Jeff Miracola"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 87 — Laccolith Grunt
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LACCOLITH_GRUNT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f27fd65a-5631-491f-b158-45012832ccf1"),
    "Laccolith Grunt",
    crate::card::CardArt::new("f27fd65a-5631-491f-b158-45012832ccf1", "Arnie Swekel"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 88 — Laccolith Rig
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LACCOLITH_RIG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4fb92039-03fd-4aee-be74-96997be629d6"),
    "Laccolith Rig",
    crate::card::CardArt::new(
        "4fb92039-03fd-4aee-be74-96997be629d6",
        "Massimiliano Frezzato",
    ),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 89 — Laccolith Titan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LACCOLITH_TITAN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e36bc466-0f74-46fd-add2-c1cf3b3fe46b"),
    "Laccolith Titan",
    crate::card::CardArt::new("e36bc466-0f74-46fd-add2-c1cf3b3fe46b", "Tony Szczudlo"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 90 — Laccolith Warrior
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LACCOLITH_WARRIOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a13b103f-482b-47d5-84a2-3621ba23bd20"),
    "Laccolith Warrior",
    crate::card::CardArt::new("a13b103f-482b-47d5-84a2-3621ba23bd20", "Mark Zug"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 91 — Laccolith Whelp
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LACCOLITH_WHELP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("86eb5b9e-320f-40de-8668-ee0c08f63ec1"),
    "Laccolith Whelp",
    crate::card::CardArt::new("86eb5b9e-320f-40de-8668-ee0c08f63ec1", "Dave Dorman"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 92 — Mana Cache
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MANA_CACHE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("583a33b3-7833-48e5-88c3-849a5771ef6e"),
    "Mana Cache",
    crate::card::CardArt::new("583a33b3-7833-48e5-88c3-849a5771ef6e", "rk post"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 93 — Mogg Alarm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOGG_ALARM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f246e128-0a43-478a-a232-51020fab76d5"),
    "Mogg Alarm",
    crate::card::CardArt::new("f246e128-0a43-478a-a232-51020fab76d5", "Dave Dorman"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

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
            ))
),
        AbilityDef::alternative_cast(
            mana_cost!("{0}"),
            AlternativeCastKindDef::AlternativeCost,
            Some("If an opponent controls an Island and you control a Mountain, you may cast this spell without paying its mana cost."),
            EffectDef::None,
        )
        .with_alternative_condition(&TriggerConditionDef::All(&[
            TriggerConditionDef::ObjectCount {
                // "If an opponent controls an Island and you control a Mountain" -- one
                // condition made of two, checked where the free cast is offered rather than
                // where it resolves.
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Opponent,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
            TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Mountain]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
            },
        ])),
    ]),
);

// NEM 95 — Mogg Toady
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOGG_TOADY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ee8edaf6-d46e-4efb-8bc0-ec11e06eb499"),
    "Mogg Toady",
    crate::card::CardArt::new("ee8edaf6-d46e-4efb-8bc0-ec11e06eb499", "Mike Ploog"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 96 — Moggcatcher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOGGCATCHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9ba582d7-1dce-4664-8bd9-6b419596788c"),
    "Moggcatcher",
    crate::card::CardArt::new("9ba582d7-1dce-4664-8bd9-6b419596788c", "Pete Venters"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 97 — Rupture
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUPTURE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("db53c1fb-3641-44a3-b0b4-b7b2ba993646"),
    "Rupture",
    crate::card::CardArt::new("db53c1fb-3641-44a3-b0b4-b7b2ba993646", "Gao Yan"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
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
        &[CostDef::SacrificeSource],
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(2),
        },
    )),
);

// NEM 99 — Shrieking Mogg
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SHRIEKING_MOGG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0ce46919-d312-490c-8942-39fbb2d375bf"),
    "Shrieking Mogg",
    crate::card::CardArt::new("0ce46919-d312-490c-8942-39fbb2d375bf", "Dan Frazier"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 100 — Stronghold Gambit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STRONGHOLD_GAMBIT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0d18050e-9aad-471e-a6ae-66e5fa2bbb6f"),
    "Stronghold Gambit",
    crate::card::CardArt::new(
        "0d18050e-9aad-471e-a6ae-66e5fa2bbb6f",
        "Greg Hildebrandt & Tim Hildebrandt",
    ),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 101 — Animate Land
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANIMATE_LAND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("20ff4e7d-fa50-48d2-8ab6-6b86e3a05e86"),
    "Animate Land",
    crate::card::CardArt::new("20ff4e7d-fa50-48d2-8ab6-6b86e3a05e86", "Rebecca Guay"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 102 — Blastoderm
pub(in crate::card::sets) static BLASTODERM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9db5d6c2-b11f-442a-b172-c0c99c9bec07"),
    "Blastoderm",
    CardArt::new("9db5d6c2-b11f-442a-b172-c0c99c9bec07", "Eric Peterson"),
    CardSet::Nemesis,
    // Shroud is what makes the fading a fair price: nothing the opponent
    // holds answers it, so three swings for four mana is the whole deal.
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Beast"], 5, 5).with_abilities(&[
        abilities::shroud(),
        AbilityDef::as_enters(
            "Fading 3 (This creature enters with three fade counters on it. At the beginning of your upkeep, remove a fade counter from it. If you can't, sacrifice it.)",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::named("fade"),
                    amount: 3,
                },
            ),
        ),
        AbilityDef::triggered(
            "At the beginning of your upkeep, remove a fade counter from this creature. If you can't, sacrifice it.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            // Three counters is three upkeeps, and the third one after them
            // is the one that cannot pay and kills it -- which is why the
            // creature gets exactly three attacks.
            EffectDef::IfElseCondition {
                condition: &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("fade"),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                },
                then: &EffectDef::RemoveCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::named("fade"),
                    amount: ValueDef::Constant(1),
                },
                otherwise: &EffectDef::Sacrifice {
                    object: EffectRecipientDef::Source,
                },
            },
        ),
    ]),
);

// NEM 103 — Coiling Woodworm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COILING_WOODWORM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("341a70be-2dbf-4365-9c7a-e52cb62a74fa"),
    "Coiling Woodworm",
    crate::card::CardArt::new("341a70be-2dbf-4365-9c7a-e52cb62a74fa", "David Martin"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 104 — Fog Patch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FOG_PATCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("133f9e4f-2b1b-4a24-ad19-285a2c5845b5"),
    "Fog Patch",
    crate::card::CardArt::new("133f9e4f-2b1b-4a24-ad19-285a2c5845b5", "Rebecca Guay"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 105 — Harvest Mage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HARVEST_MAGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("95b29329-b9a3-4d59-b0f8-2abc67337760"),
    "Harvest Mage",
    crate::card::CardArt::new("95b29329-b9a3-4d59-b0f8-2abc67337760", "Dan Frazier"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 106 — Mossdog
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOSSDOG: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2bbe0201-3df3-4d0c-8aa4-8f35f12c322c"),
    "Mossdog",
    crate::card::CardArt::new("2bbe0201-3df3-4d0c-8aa4-8f35f12c322c", "Matt Cavotta"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 107 — Nesting Wurm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NESTING_WURM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5da697da-7026-4dea-b494-8314d789160f"),
    "Nesting Wurm",
    crate::card::CardArt::new("5da697da-7026-4dea-b494-8314d789160f", "rk post"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 108 — Overlaid Terrain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OVERLAID_TERRAIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("230c7926-9a4b-4ead-b4c8-889f84210545"),
    "Overlaid Terrain",
    crate::card::CardArt::new("230c7926-9a4b-4ead-b4c8-889f84210545", "DiTerlizzi"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 109 — Pack Hunt
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PACK_HUNT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1c46caa8-efc0-4b72-b122-61e5d86a5b86"),
    "Pack Hunt",
    crate::card::CardArt::new("1c46caa8-efc0-4b72-b122-61e5d86a5b86", "Sam Wood"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 110 — Refreshing Rain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REFRESHING_RAIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c5e24850-bd9b-40ab-878f-b8a554da1956"),
    "Refreshing Rain",
    crate::card::CardArt::new("c5e24850-bd9b-40ab-878f-b8a554da1956", "Don Hazeltine"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 111 — Reverent Silence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REVERENT_SILENCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b82d3432-2167-4a65-8221-cb7b338e60d0"),
    "Reverent Silence",
    crate::card::CardArt::new("b82d3432-2167-4a65-8221-cb7b338e60d0", "Don Hazeltine"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 112 — Rhox
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RHOX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("58388a29-b2a6-4d16-b872-f198563721d9"),
    "Rhox",
    crate::card::CardArt::new("58388a29-b2a6-4d16-b872-f198563721d9", "Carl Critchlow"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 113 — Saproling Burst
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAPROLING_BURST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("de3a293d-08d8-49e4-b0fa-91afa0a5591d"),
    "Saproling Burst",
    crate::card::CardArt::new("de3a293d-08d8-49e4-b0fa-91afa0a5591d", "Carl Critchlow"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 114 — Saproling Cluster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAPROLING_CLUSTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5f50072b-aedd-4074-b1f7-f9ce477c26c2"),
    "Saproling Cluster",
    crate::card::CardArt::new("5f50072b-aedd-4074-b1f7-f9ce477c26c2", "Matt Cavotta"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 115 — Seal of Strength
pub(in crate::card::sets) static SEAL_OF_STRENGTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("57650f78-3bf0-485a-bba8-7e7e14e47508"),
    "Seal of Strength",
    CardArt::new(
        "57650f78-3bf0-485a-bba8-7e7e14e47508",
        "Christopher Moeller",
    ),
    CardSet::Nemesis,
    // The trick paid for a turn early: the mana is spent when it is spare,
    // and the pump costs nothing on the turn it matters.
    CardRules::new_enchantment(mana_cost!("{G}")).with_ability(AbilityDef::activated_with_targets(
        "Sacrifice this enchantment: Target creature gets +3/+3 until end of turn.",
        &[CostDef::SacrificeSource],
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(3),
                ValueDef::Constant(3),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// NEM 116 — Skyshroud Behemoth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKYSHROUD_BEHEMOTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1c01d17e-45a2-4b6f-aaa5-2af9c8f26181"),
    "Skyshroud Behemoth",
    crate::card::CardArt::new("1c01d17e-45a2-4b6f-aaa5-2af9c8f26181", "Eric Peterson"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 117 — Skyshroud Claim
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKYSHROUD_CLAIM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("cf3e09ff-c917-4c0c-8ddb-e152b4b0b82c"),
    "Skyshroud Claim",
    crate::card::CardArt::new("cf3e09ff-c917-4c0c-8ddb-e152b4b0b82c", "Mark Romanoski"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 118 — Skyshroud Cutter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKYSHROUD_CUTTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a558c4f5-a716-4e46-9234-5f84f1bd57aa"),
    "Skyshroud Cutter",
    crate::card::CardArt::new("a558c4f5-a716-4e46-9234-5f84f1bd57aa", "Tony Szczudlo"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 119 — Skyshroud Poacher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKYSHROUD_POACHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0fb4e44e-656e-4294-a53b-1f7aa96fab31"),
    "Skyshroud Poacher",
    crate::card::CardArt::new("0fb4e44e-656e-4294-a53b-1f7aa96fab31", "Ron Spencer"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 120 — Skyshroud Ridgeback
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKYSHROUD_RIDGEBACK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("410896ab-d3dc-478c-bfd1-c0cad5b1180a"),
    "Skyshroud Ridgeback",
    crate::card::CardArt::new("410896ab-d3dc-478c-bfd1-c0cad5b1180a", "Carl Critchlow"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 121 — Skyshroud Sentinel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKYSHROUD_SENTINEL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a35ab55f-f677-45c8-bd32-56788a776b33"),
    "Skyshroud Sentinel",
    crate::card::CardArt::new("a35ab55f-f677-45c8-bd32-56788a776b33", "Randy Gallegos"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 122 — Stampede Driver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STAMPEDE_DRIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a295758a-ee46-4ed0-8539-67501a37010d"),
    "Stampede Driver",
    crate::card::CardArt::new("a295758a-ee46-4ed0-8539-67501a37010d", "Ron Spears"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 123 — Treetop Bracers
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TREETOP_BRACERS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d6d85032-26e9-44af-ab52-56d9c24e337d"),
    "Treetop Bracers",
    crate::card::CardArt::new("d6d85032-26e9-44af-ab52-56d9c24e337d", "Heather Hudson"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 124 — Wild Mammoth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WILD_MAMMOTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4d927fdb-11c0-42b6-95d4-7af051e45213"),
    "Wild Mammoth",
    crate::card::CardArt::new("4d927fdb-11c0-42b6-95d4-7af051e45213", "Bradley Williams"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 125 — Woodripper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WOODRIPPER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5126b782-d74c-40ca-a9b2-a6c78f94d138"),
    "Woodripper",
    crate::card::CardArt::new("5126b782-d74c-40ca-a9b2-a6c78f94d138", "Alan Pollack"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 126 — Belbe's Armor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BELBE_S_ARMOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0052158b-58d1-4416-a7ce-7c6a7595263c"),
    "Belbe's Armor",
    crate::card::CardArt::new(
        "0052158b-58d1-4416-a7ce-7c6a7595263c",
        "D. Alexander Gregory",
    ),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 127 — Belbe's Portal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BELBE_S_PORTAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fb4eeea1-693e-475c-a209-8a0464df8081"),
    "Belbe's Portal",
    crate::card::CardArt::new("fb4eeea1-693e-475c-a209-8a0464df8081", "Mark Tedin"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 128 — Complex Automaton
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COMPLEX_AUTOMATON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5fb3c7af-74e1-4072-953b-b3e9ccd8aa03"),
    "Complex Automaton",
    crate::card::CardArt::new("5fb3c7af-74e1-4072-953b-b3e9ccd8aa03", "Dana Knutson"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 129 — Eye of Yawgmoth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EYE_OF_YAWGMOTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9c258aa1-cd9f-45e9-b478-d689b78850cd"),
    "Eye of Yawgmoth",
    crate::card::CardArt::new("9c258aa1-cd9f-45e9-b478-d689b78850cd", "DiTerlizzi"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 130 — Flint Golem
pub(in crate::card::sets) static FLINT_GOLEM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0e62aa7e-d9f9-42d4-9eed-5f51f88047c6"),
    "Flint Golem",
    CardArt::new("0e62aa7e-d9f9-42d4-9eed-5f51f88047c6", "Lou Harrison"),
    CardSet::Nemesis,
    // A colorless body that mills whoever stops it, which only matters in
    // a deck that wanted the graveyard filled anyway.
    CardRules::new_artifact_creature(mana_cost!("{4}"), &["Golem"], 2, 3).with_ability(
        AbilityDef::triggered(
            "Whenever this creature becomes blocked, defending player mills three cards.",
            TriggerEventDef::BecomesBlocked(ObjectPredicateDef::Source),
            EffectDef::Mill {
                player: EffectRecipientDef::players(PlayerSetDef::Related(
                    PlayerRelation::DefendingPlayer,
                )),
                amount: ValueDef::Constant(3),
            },
        ),
    ),
);

// NEM 131 — Flowstone Armor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLOWSTONE_ARMOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1160e476-8a2b-4b90-b4db-f386a80ab067"),
    "Flowstone Armor",
    crate::card::CardArt::new("1160e476-8a2b-4b90-b4db-f386a80ab067", "Paolo Parente"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 132 — Flowstone Thopter
pub(in crate::card::sets) static FLOWSTONE_THOPTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5bf016ec-2654-4c3e-8e2e-6c70c4604d28"),
    "Flowstone Thopter",
    CardArt::new("5bf016ec-2654-4c3e-8e2e-6c70c4604d28", "Mike Ploog"),
    CardSet::Nemesis,
    // Seven mana for a 4/4 that can trade toughness for evasion and power
    // one point at a time.
    CardRules::new_artifact_creature(mana_cost!("{7}"), &["Thopter"], 4, 4).with_ability(
        AbilityDef::activated(
            "{1}: This creature gets +1/-1 and gains flying until end of turn.",
            &[CostDef::Mana(mana_cost!("{1}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(-1),
                    ),
                    AppliedEffectDef::add_ability(&const { abilities::flying() }),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// NEM 133 — Kill Switch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static KILL_SWITCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a94cfd63-7f2b-4c0a-8dcb-f22cf83e1e27"),
    "Kill Switch",
    crate::card::CardArt::new("a94cfd63-7f2b-4c0a-8dcb-f22cf83e1e27", "Brian Snõddy"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 134 — Parallax Inhibitor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PARALLAX_INHIBITOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f758617c-f0e4-43d5-8fb4-e33eb2c5b99f"),
    "Parallax Inhibitor",
    crate::card::CardArt::new("f758617c-f0e4-43d5-8fb4-e33eb2c5b99f", "Greg Staples"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 135 — Predator, Flagship
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PREDATOR_FLAGSHIP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("28927927-3974-48c3-81c2-518089a10003"),
    "Predator, Flagship",
    crate::card::CardArt::new("28927927-3974-48c3-81c2-518089a10003", "Mark Tedin"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 136 — Rackling
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RACKLING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("10f82f2e-a6db-491b-b253-82c34cd6c940"),
    "Rackling",
    crate::card::CardArt::new(
        "10f82f2e-a6db-491b-b253-82c34cd6c940",
        "D. Alexander Gregory",
    ),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 137 — Rejuvenation Chamber
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REJUVENATION_CHAMBER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b97f86ce-4758-4ae3-af29-b08a4d771652"),
    "Rejuvenation Chamber",
    crate::card::CardArt::new("b97f86ce-4758-4ae3-af29-b08a4d771652", "Alan Pollack"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 138 — Rusting Golem
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUSTING_GOLEM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c2605448-8e0d-492b-a635-468923c64625"),
    "Rusting Golem",
    crate::card::CardArt::new("c2605448-8e0d-492b-a635-468923c64625", "Greg Staples"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 139 — Tangle Wire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TANGLE_WIRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ad62f313-8a8a-4ffa-ada2-b12b76288729"),
    "Tangle Wire",
    crate::card::CardArt::new("ad62f313-8a8a-4ffa-ada2-b12b76288729", "Glen Angus"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 140 — Viseling
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VISELING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a3eb86c5-d6fe-4dde-ad07-c3109b3a1611"),
    "Viseling",
    crate::card::CardArt::new("a3eb86c5-d6fe-4dde-ad07-c3109b3a1611", "Kev Walker"),
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
                    CostDef::Mana(mana_cost!("{1}{W}")),
                    CostDef::TapSource,
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

// NEM 142 — Rath's Edge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RATH_S_EDGE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("42681dce-5c63-4e56-955e-39f085ea6ae9"),
    "Rath's Edge",
    crate::card::CardArt::new("42681dce-5c63-4e56-955e-39f085ea6ae9", "Ron Spencer"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

// NEM 143 — Terrain Generator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TERRAIN_GENERATOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e6fe66a2-8e70-414f-bedb-8f1f85f1d2d9"),
    "Terrain Generator",
    crate::card::CardArt::new("e6fe66a2-8e70-414f-bedb-8f1f85f1d2d9", "Alan Pollack"),
    crate::card::CardSet::Nemesis,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ANGELIC_FAVOR,
    &AVENGER_EN_DAL,
    &BLINDING_ANGEL,
    &CHIEFTAIN_EN_DAL,
    &DEFENDER_EN_VEC,
    &DEFIANT_FALCON,
    &DEFIANT_VANGUARD,
    &FANATICAL_DEVOTION,
    &LASHKNIFE,
    &LAWBRINGER,
    &LIGHTBRINGER,
    &LIN_SIVVI_DEFIANT_HERO,
    &NETTER_EN_DAL,
    &NOBLE_STAND,
    &OFF_BALANCE,
    &ORACLE_S_ATTENDANTS,
    &PARALLAX_WAVE,
    &SEAL_OF_CLEANSING,
    &SILKENFIST_FIGHTER,
    &SILKENFIST_ORDER,
    &SIVVI_S_RUSE,
    &SIVVI_S_VALOR,
    &SPIRITUAL_ASYLUM,
    &TOPPLE,
    &VOICE_OF_TRUTH,
    &ACCUMULATED_KNOWLEDGE,
    &AETHER_BARRIER,
    &AIR_BLADDER,
    &CLOUDSKATE,
    &DAZE,
    &DOMINATE,
    &ENSNARE,
    &INFILTRATE,
    &JOLTING_MERFOLK,
    &ORAXID,
    &PALE_MOON,
    &PARALLAX_TIDE,
    &RISING_WATERS,
    &ROOTWATER_COMMANDO,
    &ROOTWATER_THIEF,
    &SEAHUNTER,
    &SEAL_OF_REMOVAL,
    &SLIPTIDE_SERPENT,
    &SNEAKY_HOMUNCULUS,
    &STRONGHOLD_BIOLOGIST,
    &STRONGHOLD_MACHINIST,
    &STRONGHOLD_ZEPPELIN,
    &SUBMERGE,
    &TRICKSTER_MAGE,
    &WANDERING_EYE,
    &ASCENDANT_EVINCAR,
    &BATTLEFIELD_PERCHER,
    &BELBE_S_PERCHER,
    &CARRION_WALL,
    &DARK_TRIUMPH,
    &DEATH_PIT_OFFERING,
    &DIVINING_WITCH,
    &MASSACRE,
    &MIND_SLASH,
    &MIND_SWORDS,
    &MURDEROUS_BETRAYAL,
    &PARALLAX_DEMENTIA,
    &PARALLAX_NEXUS,
    &PHYREXIAN_DRIVER,
    &PHYREXIAN_PROWLER,
    &PLAGUE_WITCH,
    &RATHI_ASSASSIN,
    &RATHI_FIEND,
    &RATHI_INTIMIDATOR,
    &SEAL_OF_DOOM,
    &SPINELESS_THUG,
    &SPITEFUL_BULLY,
    &STRONGHOLD_DISCIPLINE,
    &VICIOUS_HUNGER,
    &VOLRATH_THE_FALLEN,
    &ANCIENT_HYDRA,
    &ARC_MAGE,
    &BOLA_WARRIOR,
    &DOWNHILL_CHARGE,
    &FLAME_RIFT,
    &FLOWSTONE_CRUSHER,
    &FLOWSTONE_OVERSEER,
    &FLOWSTONE_SLIDE,
    &FLOWSTONE_STRIKE,
    &FLOWSTONE_SURGE,
    &FLOWSTONE_WALL,
    &LACCOLITH_GRUNT,
    &LACCOLITH_RIG,
    &LACCOLITH_TITAN,
    &LACCOLITH_WARRIOR,
    &LACCOLITH_WHELP,
    &MANA_CACHE,
    &MOGG_ALARM,
    &MOGG_SALVAGE,
    &MOGG_TOADY,
    &MOGGCATCHER,
    &RUPTURE,
    &SEAL_OF_FIRE,
    &SHRIEKING_MOGG,
    &STRONGHOLD_GAMBIT,
    &ANIMATE_LAND,
    &BLASTODERM,
    &COILING_WOODWORM,
    &FOG_PATCH,
    &HARVEST_MAGE,
    &MOSSDOG,
    &NESTING_WURM,
    &OVERLAID_TERRAIN,
    &PACK_HUNT,
    &REFRESHING_RAIN,
    &REVERENT_SILENCE,
    &RHOX,
    &SAPROLING_BURST,
    &SAPROLING_CLUSTER,
    &SEAL_OF_STRENGTH,
    &SKYSHROUD_BEHEMOTH,
    &SKYSHROUD_CLAIM,
    &SKYSHROUD_CUTTER,
    &SKYSHROUD_POACHER,
    &SKYSHROUD_RIDGEBACK,
    &SKYSHROUD_SENTINEL,
    &STAMPEDE_DRIVER,
    &TREETOP_BRACERS,
    &WILD_MAMMOTH,
    &WOODRIPPER,
    &BELBE_S_ARMOR,
    &BELBE_S_PORTAL,
    &COMPLEX_AUTOMATON,
    &EYE_OF_YAWGMOTH,
    &FLINT_GOLEM,
    &FLOWSTONE_ARMOR,
    &FLOWSTONE_THOPTER,
    &KILL_SWITCH,
    &PARALLAX_INHIBITOR,
    &PREDATOR_FLAGSHIP,
    &RACKLING,
    &REJUVENATION_CHAMBER,
    &RUSTING_GOLEM,
    &TANGLE_WIRE,
    &VISELING,
    &KOR_HAVEN,
    &RATH_S_EDGE,
    &TERRAIN_GENERATOR,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
