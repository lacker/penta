//! Nemesis cards used by the staged Premodern deck tranche.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AlternativeCastKindDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::BasicLandType;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::BlockRestrictionDef;
use crate::card::BlockRestrictionMatchDef;
use crate::card::BlockRestrictionSubjectDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CardTypeSet;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseDef;
use crate::card::ComparisonDef;
use crate::card::ControlDurationDef;
use crate::card::CostDef;
use crate::card::CostQuantityDef;
use crate::card::CounterKind;
use crate::card::DamageEventMatcherDef;
use crate::card::DamagePreventionDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::KeywordAbility;
use crate::card::ManaColor;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::PayOrDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::ReplacementEffectDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SacrificedAmountDef;
use crate::card::SubtypeDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::ids::ParentBinding;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "NEM",
    slug: "nemesis",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// NEM 1 — Angelic Favor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANGELIC_FAVOR: CardRecord = CardRecord::new(
    "Angelic Favor",
    "871ad2f3-1dd2-45ea-881d-529aad3b76ec",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// NEM 2 — Avenger en-Dal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AVENGER_EN_DAL: CardRecord = CardRecord::new(
    "Avenger en-Dal",
    "fcf6f711-c0bc-4e12-b9d0-41581924e13c",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// NEM 3 — Blinding Angel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BLINDING_ANGEL: CardRecord = CardRecord::new(
    "Blinding Angel",
    "48c25553-6554-4e31-9012-c50da1f0a171",
    "Todd Lockwood",
    crate::card::CardRules::unsupported(),
);

// NEM 4 — Chieftain en-Dal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CHIEFTAIN_EN_DAL: CardRecord = CardRecord::new(
    "Chieftain en-Dal",
    "0c1f49bc-d144-466f-8795-c0dae7afdc10",
    "Dany Orizio",
    crate::card::CardRules::unsupported(),
);

// NEM 5 — Defender en-Vec
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEFENDER_EN_VEC: CardRecord = CardRecord::new(
    "Defender en-Vec",
    "e7b224b7-d5f7-4515-beca-523f305ee3b8",
    "Bradley Williams",
    crate::card::CardRules::unsupported(),
);

// NEM 6 — Defiant Falcon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEFIANT_FALCON: CardRecord = CardRecord::new(
    "Defiant Falcon",
    "4c80032b-daeb-4661-9a66-61abe9d12ddd",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// NEM 7 — Defiant Vanguard
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEFIANT_VANGUARD: CardRecord = CardRecord::new(
    "Defiant Vanguard",
    "4c0bd267-59ec-41df-b0b7-37f6e6d6b073",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// NEM 8 — Fanatical Devotion
pub(in crate::card::sets) static FANATICAL_DEVOTION: CardRecord = CardRecord::new(
    "Fanatical Devotion",
    "be0ed1fb-d380-4e3e-a43f-c39660a996e9",
    "Massimiliano Frezzato",
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
    "Lashknife",
    "fd7451cc-4126-4518-a103-2558fa81323f",
    "Hannibal King",
    crate::card::CardRules::unsupported(),
);

// NEM 10 — Lawbringer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LAWBRINGER: CardRecord = CardRecord::new(
    "Lawbringer",
    "2d76b7e3-6890-4120-8575-732909c8bdff",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// NEM 11 — Lightbringer
pub(in crate::card::sets) static LIGHTBRINGER: CardRecord = CardRecord::new(
    "Lightbringer",
    "19451993-7a53-4a50-bfca-ddc9cdfbe168",
    "Paolo Parente",
    // A sideboard card on a body, aimed at the one colour whose creatures
    // come back from the graveyard.
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Cleric"], 2, 2).with_ability(
        AbilityDef::activated_with_targets(
            "{T}, Sacrifice this creature: Exile target black creature.",
            &[CostDef::TapSource, CostDef::SacrificeSource],
            &const {
                [AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Color(ManaColor::Black),
                    ]),
                )]
            },
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Exile,
                placement: ZonePlacement::Top,
            },
        ),
    ),
);

// NEM 12 — Lin Sivvi, Defiant Hero
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LIN_SIVVI_DEFIANT_HERO: CardRecord = CardRecord::new(
    "Lin Sivvi, Defiant Hero",
    "e574e522-2632-4cd4-8545-c582ac3b641f",
    "rk post",
    crate::card::CardRules::unsupported(),
);

// NEM 13 — Netter en-Dal
pub(in crate::card::sets) static NETTER_EN_DAL: CardRecord = CardRecord::new(
    "Netter en-Dal",
    "2190649d-f898-4693-8ccf-80e6709d8496",
    "Matt Cavotta",
    // A card to stop one attacker, which is a rate only a deck already
    // holding cards it cannot cast is happy with.
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Spellshaper"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{W}, {T}, Discard a card: Target creature can't attack this turn.",
            &[
                CostDef::Mana(mana_cost!("{W}")),
                CostDef::TapSource,
                CostDef::discard(ObjectPredicateDef::Any),
            ],
            &const {
                [AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )]
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::target_objects(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_ATTACK),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// NEM 14 — Noble Stand
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NOBLE_STAND: CardRecord = CardRecord::new(
    "Noble Stand",
    "5f53ab12-7c16-43b1-b9f9-a5e523cf431b",
    "Greg Hildebrandt & Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// NEM 15 — Off Balance
pub(in crate::card::sets) static OFF_BALANCE: CardRecord = CardRecord::new(
    "Off Balance",
    "adafe5c4-8de0-4d38-919f-de96bc70c21b",
    "Jeff Miracola",
    // A one-mana answer that only lasts the turn, which is enough when the
    // turn in question is the one that would have killed you.
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature can't attack or block this turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        // An attack prohibition is validated only against an object
        // recipient, so the slot's members are named as objects rather than
        // as the target slot itself.
        EffectDef::Apply {
            recipient: EffectRecipientDef::target_objects(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_ATTACK),
                AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// NEM 16 — Oracle's Attendants
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ORACLE_S_ATTENDANTS: CardRecord = CardRecord::new(
    "Oracle's Attendants",
    "e2e0ea3e-9826-408d-835b-18dfecaac8af",
    "Dany Orizio",
    crate::card::CardRules::unsupported(),
);

// NEM 17 — Parallax Wave
pub(in crate::card::sets) static PARALLAX_WAVE: CardRecord = CardRecord::new(
    "Parallax Wave",
    "cef789e8-e4cc-4f61-bc15-debc2487777f",
    "Greg Staples",
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
                otherwise: &EffectDef::sacrifice(EffectRecipientDef::Source),
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
pub(in crate::card::sets) static SEAL_OF_CLEANSING: CardRecord = CardRecord::new(
    "Seal of Cleansing",
    "af6c921e-1b82-412c-9979-adfdf83440f7",
    "Christopher Moeller",
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
    "Silkenfist Fighter",
    "3480efc4-1078-4c63-a94c-d00a7507f6b1",
    "Mark Brill",
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
    "Silkenfist Order",
    "93741517-90ed-46fe-a505-fe6299f188bf",
    "Greg Hildebrandt & Tim Hildebrandt",
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
    "Sivvi's Ruse",
    "132112a0-0fb0-4a80-927d-39d34cf10159",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// NEM 22 — Sivvi's Valor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SIVVI_S_VALOR: CardRecord = CardRecord::new(
    "Sivvi's Valor",
    "9d15f7b5-5070-4742-a05c-623822d874fb",
    "Jeff Miracola",
    crate::card::CardRules::unsupported(),
);

// NEM 23 — Spiritual Asylum
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SPIRITUAL_ASYLUM: CardRecord = CardRecord::new(
    "Spiritual Asylum",
    "e5eea354-2d92-4b57-aec7-25260ab7a70f",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// NEM 24 — Topple
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TOPPLE: CardRecord = CardRecord::new(
    "Topple",
    "a7c25c67-4214-4318-a718-7d351e713f80",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// NEM 25 — Voice of Truth
pub(in crate::card::sets) static VOICE_OF_TRUTH: CardRecord = CardRecord::new(
    "Voice of Truth",
    "40377e3d-77d9-4d86-ac8c-4e27803e48d8",
    "rk post",
    // Protection from its own colour is a mirror-match card, which is what
    // the whole Nemesis cycle was for.
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Angel"], 2, 2).with_abilities(&[
        abilities::flying(),
        abilities::protection_from_color(ManaColor::White),
    ]),
);

// NEM 26 — Accumulated Knowledge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ACCUMULATED_KNOWLEDGE: CardRecord = CardRecord::new(
    "Accumulated Knowledge",
    "ab061406-38f4-40e7-a9ea-e3cbcaabc127",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// NEM 27 — Aether Barrier
pub(in crate::card::sets) static AETHER_BARRIER: CardRecord = CardRecord::new(
    "Aether Barrier",
    "36298f9f-12cc-43bd-adda-ccabd67a9568",
    "David Martin",
// A tax on every creature spell, paid by whoever cast it -- which is why
    // it belongs in the deck with no creatures at all.
    CardRules::new_enchantment(mana_cost!("{2}{U}")).with_ability(AbilityDef::triggered(
    "Whenever a player casts a creature spell, that player sacrifices a permanent of their choice unless they pay {1}.",
    TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
        ObjectPredicateDef::Spell,
        ObjectPredicateDef::HasType(CardType::Creature),
    ])),
    EffectDef::PayOr(
        PayOrDef::unless(
            &[CostDef::Mana(mana_cost!("{1}"))],
            &const {
                EffectDef::SacrificeOfChoice {
                    player: EffectRecipientDef::ControllerOfTriggeringObject,
                    object: ObjectPredicateDef::Any,
                    count: ValueDef::Constant(1),
                    then: None,
                    amount: SacrificedAmountDef::Power,
                    otherwise: None,
                    optional: false,
                }
            },
        )
        .with_payer(PlayerSetDef::One(PlayerRefDef::ControllerOf(
            ObjectRefDef::TriggeringObject,
        ))),
    ),
)),
);

// NEM 28 — Air Bladder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static AIR_BLADDER: CardRecord = CardRecord::new(
    "Air Bladder",
    "a7363c6f-53a3-4f41-b451-8120bc24f1ee",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// NEM 29 — Cloudskate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CLOUDSKATE: CardRecord = CardRecord::new(
    "Cloudskate",
    "e7f97e50-3aeb-4c79-81b1-505a2f32d8ac",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// NEM 30 — Daze
pub(in crate::card::sets) static DAZE: CardRecord = CardRecord::new(
    "Daze",
    "d03bff25-0d5e-4dcf-8d75-6df846afea3b",
    "Matthew D. Wilson",
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
            abilities::counter_target_unless_paid(&[CostDef::GenericMana(ValueDef::Constant(1))]),
        ),
        AbilityDef::alternative_cast(
            &[CostDef::return_to_hand(
                ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Island]),
                CostQuantityDef::Fixed(1),
            )],
            AlternativeCastKindDef::AlternativeCost,
            Some("You may return an Island you control to its owner's hand rather than pay this spell's mana cost."),
            EffectDef::None,
        )
        // One Island back to hand, which is what makes the card free on turn one and
        // a real cost on turn six.
        ,
    ]),
);

// NEM 31 — Dominate
pub(in crate::card::sets) static DOMINATE: CardRecord = CardRecord::new(
    "Dominate",
    "63b2dcb1-8c3e-434c-865a-196d4d799706",
    "Scott Hampton",
    // Control theft sized to the mana, so it answers whatever the format's
    // best creature happens to cost.
    CardRules::new_instant(mana_cost!("{X}{1}{U}{U}")).with_ability(
        AbilityDef::spell_with_targets(
            "Gain control of target creature with mana value X or less.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ManaValueAtMostValue(ValueDef::ChosenX),
                ]),
            )],
            EffectDef::gain_control(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                PlayerRefDef::EffectController,
                ControlDurationDef::Indefinitely,
            ),
        ),
    ),
);

// NEM 32 — Ensnare
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ENSNARE: CardRecord = CardRecord::new(
    "Ensnare",
    "055b344a-4eb1-4579-ac50-973b18e12fad",
    "Gao Yan",
    crate::card::CardRules::unsupported(),
);

// NEM 33 — Infiltrate
pub(in crate::card::sets) static INFILTRATE: CardRecord = CardRecord::new(
    "Infiltrate",
    "c549b817-8ad6-44d0-9761-5e4ff9e62c71",
    "Nelson DeCastro",
    // One mana that turns any creature into the whole clock for a turn,
    // which is what a deck built around one big body wants.
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature can't be blocked this turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// NEM 34 — Jolting Merfolk
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static JOLTING_MERFOLK: CardRecord = CardRecord::new(
    "Jolting Merfolk",
    "8b4d1c74-8b73-445c-b226-349c57a972f6",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// NEM 35 — Oraxid
pub(in crate::card::sets) static ORAXID: CardRecord = CardRecord::new(
    "Oraxid",
    "6c05609a-f32d-4454-af24-a24452997dcb",
    "Dave Dorman",
    // A blue blocker that red cannot burn or get past, printed into a
    // format where that was the whole question.
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Crab", "Beast"], 2, 3)
        .with_ability(abilities::protection_from_color(ManaColor::Red)),
);

// NEM 36 — Pale Moon
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PALE_MOON: CardRecord = CardRecord::new(
    "Pale Moon",
    "aeb282bb-d0b8-4822-8197-ff0523549309",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// NEM 37 — Parallax Tide
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PARALLAX_TIDE: CardRecord = CardRecord::new(
    "Parallax Tide",
    "7fe593eb-df3c-43e5-97a6-418f91e87cb3",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// NEM 38 — Rising Waters
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RISING_WATERS: CardRecord = CardRecord::new(
    "Rising Waters",
    "ec9c84db-cf45-43e1-b38f-8bbf53cf088b",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// NEM 39 — Rootwater Commando
pub(in crate::card::sets) static ROOTWATER_COMMANDO: CardRecord = CardRecord::new(
    "Rootwater Commando",
    "8e86f36d-584d-49b2-8c66-19c262408950",
    "Mark Tedin",
    // The blue mirror-breaker: three mana for two damage a turn that the
    // other blue deck cannot block.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Merfolk"], 2, 2)
        .with_ability(abilities::landwalk(BasicLandType::Island)),
);

// NEM 40 — Rootwater Thief
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ROOTWATER_THIEF: CardRecord = CardRecord::new(
    "Rootwater Thief",
    "38addef3-1dd7-41a1-9706-3be5c86a58c9",
    "Ron Spears",
    crate::card::CardRules::unsupported(),
);

// NEM 41 — Seahunter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SEAHUNTER: CardRecord = CardRecord::new(
    "Seahunter",
    "c375f65a-6d88-4d3c-a7a7-8c7a5cc5807f",
    "Heather Hudson",
    crate::card::CardRules::unsupported(),
);

// NEM 42 — Seal of Removal
pub(in crate::card::sets) static SEAL_OF_REMOVAL: CardRecord = CardRecord::new(
    "Seal of Removal",
    "487becfe-a9b1-4029-a487-2a32561570cb",
    "Christopher Moeller",
    // The bounce is paid for a turn early and kept on the table, which
    // costs a card but takes the tempo loss off the turn it matters.
    CardRules::new_enchantment(mana_cost!("{U}")).with_ability(AbilityDef::activated_with_targets(
        "Sacrifice this enchantment: Return target creature to its owner's hand.",
        &[CostDef::SacrificeSource],
        &const {
            [AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )]
        },
        EffectDef::MoveToZone {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Hand,
            placement: ZonePlacement::Top,
        },
    )),
);

// NEM 43 — Sliptide Serpent
pub(in crate::card::sets) static SLIPTIDE_SERPENT: CardRecord = CardRecord::new(
    "Sliptide Serpent",
    "f8647649-5669-46c4-8840-9ff967fabd99",
    "Daren Bader",
    // Six mana for a 4/4 that dodges removal for four more, which is the
    // deal a control deck takes when it has nothing else to do.
    CardRules::new_creature(mana_cost!("{4}{U}{U}"), &["Serpent"], 4, 4).with_ability(
        AbilityDef::activated(
            "{3}{U}: Return this creature to its owner's hand.",
            &[CostDef::Mana(mana_cost!("{3}{U}"))],
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Source,
                zone: ZoneKind::Hand,
                placement: ZonePlacement::Top,
            },
        ),
    ),
);

// NEM 44 — Sneaky Homunculus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SNEAKY_HOMUNCULUS: CardRecord = CardRecord::new(
    "Sneaky Homunculus",
    "e1b2dadb-4ce3-4f7e-9ca7-79757543f04d",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// NEM 45 — Stronghold Biologist
pub(in crate::card::sets) static STRONGHOLD_BIOLOGIST: CardRecord = CardRecord::new(
    "Stronghold Biologist",
    "6215a5d9-d6d2-4f9f-8a0c-a65d1afd956a",
    "Terese Nielsen",
    // A counterspell that costs a card and a turn, which a deck with no
    // counterspells at all is glad to have on a body.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Spellshaper"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{U}{U}, {T}, Discard a card: Counter target creature spell.",
            &[
                CostDef::Mana(mana_cost!("{U}{U}")),
                CostDef::TapSource,
                CostDef::discard(ObjectPredicateDef::Any),
            ],
            &const {
                [AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Spell,
                            ObjectPredicateDef::HasType(CardType::Creature),
                        ]),
                        zones: &[ZoneKind::Stack],
                        controller: None,
                        owner: None,
                    },
                )]
            },
            EffectDef::Counter {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Graveyard,
                placement: ZonePlacement::Top,
            },
        ),
    ),
);

// NEM 46 — Stronghold Machinist
pub(in crate::card::sets) static STRONGHOLD_MACHINIST: CardRecord = CardRecord::new(
    "Stronghold Machinist",
    "d3567b4e-6e31-40b0-83ea-4a2a58bd637c",
    "Terese Nielsen",
    // The other half of the pair, and between them a deck answers whichever
    // spell it was actually afraid of.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Spellshaper"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{U}{U}, {T}, Discard a card: Counter target noncreature spell.",
            &[
                CostDef::Mana(mana_cost!("{U}{U}")),
                CostDef::TapSource,
                CostDef::discard(ObjectPredicateDef::Any),
            ],
            &const {
                [AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::NoncreatureSpell,
                        zones: &[ZoneKind::Stack],
                        controller: None,
                        owner: None,
                    },
                )]
            },
            EffectDef::Counter {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Graveyard,
                placement: ZonePlacement::Top,
            },
        ),
    ),
);

// NEM 47 — Stronghold Zeppelin
pub(in crate::card::sets) static STRONGHOLD_ZEPPELIN: CardRecord = CardRecord::new(
    "Stronghold Zeppelin",
    "d672110d-b7c4-4233-9c46-73323be7204d",
    "Arnie Swekel",
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
    "Submerge",
    "d2741fe4-37fe-427f-ae85-5107991d4eee",
    "Mark Romanoski",
    crate::card::CardRules::unsupported(),
);

// NEM 49 — Trickster Mage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TRICKSTER_MAGE: CardRecord = CardRecord::new(
    "Trickster Mage",
    "b31b2e24-1a70-48bd-8946-ff29e12c6f3d",
    "Alan Rabinowitz",
    crate::card::CardRules::unsupported(),
);

// NEM 50 — Wandering Eye
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WANDERING_EYE: CardRecord = CardRecord::new(
    "Wandering Eye",
    "2869efd2-060f-4af3-b0dc-b7dc5e1143b8",
    "Sam Wood",
    crate::card::CardRules::unsupported(),
);

// NEM 51 — Ascendant Evincar
pub(in crate::card::sets) static ASCENDANT_EVINCAR: CardRecord = CardRecord::new(
    "Ascendant Evincar",
    "e5c87c93-8cf4-4d1a-9bb8-349600da55bc",
    "Mark Zug",
    // Two anthems pointing opposite ways, which turns a board that was even
    // into one that is two points apart on every body.
    CardRules::new_creature(
        mana_cost!("{4}{B}{B}"),
        &["Phyrexian", "Vampire", "Noble"],
        3,
        3,
    )
    .with_supertype(CardSupertype::Legendary)
    .with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "Other black creatures get +1/+1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Color(ManaColor::Black),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
            },
        ),
        AbilityDef::static_ability(
            "Nonblack creatures get -1/-1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Color(ManaColor::Black)),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-1),
                    ValueDef::Constant(-1),
                ),
            },
        ),
    ]),
);

// NEM 52 — Battlefield Percher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BATTLEFIELD_PERCHER: CardRecord = CardRecord::new(
    "Battlefield Percher",
    "f1ebf021-02a1-4a47-b581-c85a7a76cdec",
    "Edward P. Beard, Jr.",
    crate::card::CardRules::unsupported(),
);

// NEM 53 — Belbe's Percher
pub(in crate::card::sets) static BELBE_S_PERCHER: CardRecord = CardRecord::new(
    "Belbe's Percher",
    "d95dcb2e-8945-47dd-ad40-b5bdcc3ea742",
    "Edward P. Beard, Jr.",
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
pub(in crate::card::sets) static CARRION_WALL: CardRecord = CardRecord::new(
    "Carrion Wall",
    "61d6fa78-3422-4ace-88ab-e985c558cba7",
    "Tony Szczudlo",
    // A wall that survives everything the ground can throw at it for two
    // mana a turn, which is the price of never attacking.
    CardRules::new_creature(mana_cost!("{1}{B}{B}"), &["Wall"], 3, 2).with_abilities(&[
        abilities::defender(),
        abilities::regenerate_self(
            "{1}{B}: Regenerate this creature.",
            &[CostDef::Mana(mana_cost!("{1}{B}"))],
        ),
    ]),
);

// NEM 55 — Dark Triumph
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DARK_TRIUMPH: CardRecord = CardRecord::new(
    "Dark Triumph",
    "794657cd-b292-41d7-a4a6-3f3dd20dc07a",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// NEM 56 — Death Pit Offering
pub(in crate::card::sets) static DEATH_PIT_OFFERING: CardRecord = CardRecord::new(
    "Death Pit Offering",
    "6223e583-8ef6-4d93-8ed0-3ccf4488f166",
    "Pete Venters",
    // It costs you the board and pays you back on everything that follows,
    // which is a deal only a deck already holding creatures can take.
    CardRules::new_enchantment(mana_cost!("{2}{B}{B}")).with_abilities(&[
        abilities::enters_trigger(
            "When this enchantment enters, sacrifice all creatures you control.",
            EffectDef::sacrifice(EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            )),
        ),
        AbilityDef::static_ability(
            "Creatures you control get +2/+2.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(2),
                    ValueDef::Constant(2),
                ),
            },
        ),
    ]),
);

// NEM 57 — Divining Witch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DIVINING_WITCH: CardRecord = CardRecord::new(
    "Divining Witch",
    "be981eef-9dd2-4233-82c9-03f9f2e82c59",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// NEM 58 — Massacre
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MASSACRE: CardRecord = CardRecord::new(
    "Massacre",
    "f05f5d93-50d1-4aa6-af05-383a6808345b",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// NEM 59 — Mind Slash
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIND_SLASH: CardRecord = CardRecord::new(
    "Mind Slash",
    "8bece38b-e09e-4666-95b6-5e5b05867cd5",
    "Adam Rex",
    crate::card::CardRules::unsupported(),
);

// NEM 60 — Mind Swords
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MIND_SWORDS: CardRecord = CardRecord::new(
    "Mind Swords",
    "3d6d91df-008b-48f2-a84f-550702fbcdb3",
    "Daren Bader",
    crate::card::CardRules::unsupported(),
);

// NEM 61 — Murderous Betrayal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MURDEROUS_BETRAYAL: CardRecord = CardRecord::new(
    "Murderous Betrayal",
    "f13a3ed0-aa57-4082-b6b0-b1078c93c0b2",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// NEM 62 — Parallax Dementia
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PARALLAX_DEMENTIA: CardRecord = CardRecord::new(
    "Parallax Dementia",
    "154789ac-bbea-467b-9655-76f378a53f40",
    "Eric Peterson",
    crate::card::CardRules::unsupported(),
);

// NEM 63 — Parallax Nexus
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PARALLAX_NEXUS: CardRecord = CardRecord::new(
    "Parallax Nexus",
    "862c50c7-0840-46e0-a653-5b660fdfd4bd",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// NEM 64 — Phyrexian Driver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_DRIVER: CardRecord = CardRecord::new(
    "Phyrexian Driver",
    "efeef7f3-5b87-440d-a851-95ae2bdb840d",
    "Chippy",
    crate::card::CardRules::unsupported(),
);

// NEM 65 — Phyrexian Prowler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PHYREXIAN_PROWLER: CardRecord = CardRecord::new(
    "Phyrexian Prowler",
    "e26f79b5-780a-4cbb-b49d-01673a411d1f",
    "Mark Zug",
    crate::card::CardRules::unsupported(),
);

// NEM 66 — Plague Witch
pub(in crate::card::sets) static PLAGUE_WITCH: CardRecord = CardRecord::new(
    "Plague Witch",
    "ca3614c2-39b4-4f67-adab-373dcb9e4553",
    "Nelson DeCastro",
    // A card and a tap for one point of shrink, which only matters against
    // a board of one-toughness creatures -- and then matters a lot.
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Spellshaper"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{B}, {T}, Discard a card: Target creature gets -1/-1 until end of turn.",
            &[
                CostDef::Mana(mana_cost!("{B}")),
                CostDef::TapSource,
                CostDef::discard(ObjectPredicateDef::Any),
            ],
            &const {
                [AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )]
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-1),
                    ValueDef::Constant(-1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// NEM 67 — Rathi Assassin
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RATHI_ASSASSIN: CardRecord = CardRecord::new(
    "Rathi Assassin",
    "3e3597c3-3053-49f8-ab7e-a774e2fb082f",
    "Dana Knutson",
    crate::card::CardRules::unsupported(),
);

// NEM 68 — Rathi Fiend
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RATHI_FIEND: CardRecord = CardRecord::new(
    "Rathi Fiend",
    "07ca1184-ade0-4d6d-87f9-ad17f37679b3",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// NEM 69 — Rathi Intimidator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RATHI_INTIMIDATOR: CardRecord = CardRecord::new(
    "Rathi Intimidator",
    "6fc59fa5-144f-49e6-b6bd-2ba6d3f2eff2",
    "Mike Ploog",
    crate::card::CardRules::unsupported(),
);

// NEM 70 — Seal of Doom
pub(in crate::card::sets) static SEAL_OF_DOOM: CardRecord = CardRecord::new(
    "Seal of Doom",
    "396d9f58-a4ca-4197-94be-0f115427224e",
    "Christopher Moeller",
// Removal announced three turns early, which the opponent can play
    // around and can never counter.
    CardRules::new_enchantment(mana_cost!("{2}{B}")).with_ability(
        AbilityDef::activated_with_targets(
            "Sacrifice this enchantment: Destroy target nonblack creature. It can't be regenerated.",
            &[CostDef::SacrificeSource],
            &const {
                [AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Color(ManaColor::Black)),
                ]))]
            },
            EffectDef::WithRule {
                rule: AppliedRuleDef::CannotRegenerate,
                effect: &EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            },
        ),
    ),
);

// NEM 71 — Spineless Thug
pub(in crate::card::sets) static SPINELESS_THUG: CardRecord = CardRecord::new(
    "Spineless Thug",
    "e4a9bb47-3855-425d-924c-09dbde74735b",
    "Matthew D. Wilson",
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
pub(in crate::card::sets) static SPITEFUL_BULLY: CardRecord = CardRecord::new(
    "Spiteful Bully",
    "5535d14a-7126-4a94-96a0-e17ad5c72070",
    "Chippy",
// A 3/3 for two whose upkeep is paid in your own creatures, so it is
    // best in the deck that has nothing else worth keeping.
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Phyrexian", "Zombie", "Mercenary"], 3, 3)
        .with_ability(AbilityDef::triggered_with_targets(
            "At the beginning of your upkeep, this creature deals 3 damage to target creature you control.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            &const {
                [AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]))]
            },
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(3),
            ),
        )),
);

// NEM 73 — Stronghold Discipline
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STRONGHOLD_DISCIPLINE: CardRecord = CardRecord::new(
    "Stronghold Discipline",
    "46fa5472-5341-47cd-884e-fe2fcca12c0d",
    "Li Tie",
    crate::card::CardRules::unsupported(),
);

// NEM 74 — Vicious Hunger
pub(in crate::card::sets) static VICIOUS_HUNGER: CardRecord = CardRecord::new(
    "Vicious Hunger",
    "ccaff6a0-7831-45db-a50f-881c6cb7ce49",
    "Massimiliano Frezzato",
    // Two damage and two life is a four-point swing in a race, which is
    // what a two-mana sorcery buys when it cannot go to the face.
    CardRules::new_sorcery(mana_cost!("{B}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Vicious Hunger deals 2 damage to target creature and you gain 2 life.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Sequence(&[
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(2),
            ),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ]),
    )),
);

// NEM 75 — Volrath the Fallen
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VOLRATH_THE_FALLEN: CardRecord = CardRecord::new(
    "Volrath the Fallen",
    "08bdd66e-9ca1-456e-a61c-7c96cf6f7c56",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// NEM 76 — Ancient Hydra
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANCIENT_HYDRA: CardRecord = CardRecord::new(
    "Ancient Hydra",
    "5de57c84-38b9-4606-b934-0ab270496582",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

// NEM 77 — Arc Mage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ARC_MAGE: CardRecord = CardRecord::new(
    "Arc Mage",
    "62982dab-4c27-45b3-9740-38fec3df7226",
    "Terese Nielsen",
    crate::card::CardRules::unsupported(),
);

// NEM 78 — Bola Warrior
pub(in crate::card::sets) static BOLA_WARRIOR: CardRecord = CardRecord::new(
    "Bola Warrior",
    "dc6e1de6-e7e0-4037-896a-f80c54b8ef5c",
    "Adam Rex",
    // Removing a blocker for a card is the same trade an unblockable spell
    // makes, on a body that can do it again.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Spellshaper"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{R}, {T}, Discard a card: Target creature can't block this turn.",
            &[
                CostDef::Mana(mana_cost!("{R}")),
                CostDef::TapSource,
                CostDef::discard(ObjectPredicateDef::Any),
            ],
            &const {
                [AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )]
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// NEM 79 — Downhill Charge
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DOWNHILL_CHARGE: CardRecord = CardRecord::new(
    "Downhill Charge",
    "2ebfc91c-d764-4e63-a428-82704f8bf1fd",
    "Greg Hildebrandt & Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// NEM 80 — Flame Rift
pub(in crate::card::sets) static FLAME_RIFT: CardRecord = CardRecord::new(
    "Flame Rift",
    "7717eeb9-c457-4a65-93a0-e91c7f6a1970",
    "Ben Thompson",
    // Four to each player for two mana, which is only a bargain for the
    // deck that was going to lose the long game anyway.
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_ability(AbilityDef::spell(
        "Flame Rift deals 4 damage to each player.",
        EffectDef::damage(EffectRecipientDef::EachPlayer, ValueDef::Constant(4)),
    )),
);

// NEM 81 — Flowstone Crusher
pub(in crate::card::sets) static FLOWSTONE_CRUSHER: CardRecord = CardRecord::new(
    "Flowstone Crusher",
    "c93f0066-1ff0-4e52-9959-9eb0def60957",
    "Ben Thompson",
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
pub(in crate::card::sets) static FLOWSTONE_OVERSEER: CardRecord = CardRecord::new(
    "Flowstone Overseer",
    "3e644ab8-3cc3-413d-a918-44fc636087ae",
    "Andrew Goldhawk",
    // It shrinks whatever it points at and grows nothing, so every two mana
    // is a blocker removed rather than a fight won.
    CardRules::new_creature(mana_cost!("{2}{R}{R}{R}"), &["Beast"], 4, 4).with_ability(
        AbilityDef::activated_with_targets(
            "{R}{R}: Target creature gets +1/-1 until end of turn.",
            &[CostDef::Mana(mana_cost!("{R}{R}"))],
            &const {
                [AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )]
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(-1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// NEM 83 — Flowstone Slide
pub(in crate::card::sets) static FLOWSTONE_SLIDE: CardRecord = CardRecord::new(
    "Flowstone Slide",
    "ec7b02e1-0a20-4247-ae2a-056c5356f168",
    "Chippy",
    // Toughness for power on everything, which sweeps small boards and
    // turns large ones lethal.
    CardRules::new_sorcery(mana_cost!("{X}{2}{R}{R}")).with_ability(AbilityDef::spell(
        "All creatures get +X/-X until end of turn.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::ChosenX,
                ValueDef::Negate(&ValueDef::ChosenX),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// NEM 84 — Flowstone Strike
pub(in crate::card::sets) static FLOWSTONE_STRIKE: CardRecord = CardRecord::new(
    "Flowstone Strike",
    "a1203053-0829-4f46-a361-62cb9cd17280",
    "Mike Ploog",
    // The haste is what makes it a trick rather than a pump: it turns a
    // creature that just arrived into damage this turn.
    CardRules::new_instant(mana_cost!("{1}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature gets +1/-1 and gains haste until end of turn.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(-1),
                ),
                AppliedEffectDef::add_ability(&const { abilities::haste() }),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// NEM 85 — Flowstone Surge
pub(in crate::card::sets) static FLOWSTONE_SURGE: CardRecord = CardRecord::new(
    "Flowstone Surge",
    "bc450922-0bbf-46c4-9955-79f4d41ee488",
    "Scott Hampton",
    // Power for toughness across the board, which suits the deck that was
    // attacking anyway and ruins the one that was not.
    CardRules::new_enchantment(mana_cost!("{1}{R}")).with_ability(AbilityDef::static_ability(
        "Creatures you control get +1/-1.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(1),
                ValueDef::Constant(-1),
            ),
        },
    )),
);

// NEM 86 — Flowstone Wall
pub(in crate::card::sets) static FLOWSTONE_WALL: CardRecord = CardRecord::new(
    "Flowstone Wall",
    "89844c2f-f0a4-41c6-ad8c-d559fcaec85c",
    "Jeff Miracola",
    // Six toughness it can spend a point at a time, so it blocks something
    // large and then kills it.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Wall"], 0, 6).with_abilities(&[
        abilities::defender(),
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
    ]),
);

// NEM 87 — Laccolith Grunt
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LACCOLITH_GRUNT: CardRecord = CardRecord::new(
    "Laccolith Grunt",
    "f27fd65a-5631-491f-b158-45012832ccf1",
    "Arnie Swekel",
    crate::card::CardRules::unsupported(),
);

// NEM 88 — Laccolith Rig
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LACCOLITH_RIG: CardRecord = CardRecord::new(
    "Laccolith Rig",
    "4fb92039-03fd-4aee-be74-96997be629d6",
    "Massimiliano Frezzato",
    crate::card::CardRules::unsupported(),
);

// NEM 89 — Laccolith Titan
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LACCOLITH_TITAN: CardRecord = CardRecord::new(
    "Laccolith Titan",
    "e36bc466-0f74-46fd-add2-c1cf3b3fe46b",
    "Tony Szczudlo",
    crate::card::CardRules::unsupported(),
);

// NEM 90 — Laccolith Warrior
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LACCOLITH_WARRIOR: CardRecord = CardRecord::new(
    "Laccolith Warrior",
    "a13b103f-482b-47d5-84a2-3621ba23bd20",
    "Mark Zug",
    crate::card::CardRules::unsupported(),
);

// NEM 91 — Laccolith Whelp
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LACCOLITH_WHELP: CardRecord = CardRecord::new(
    "Laccolith Whelp",
    "86eb5b9e-320f-40de-8668-ee0c08f63ec1",
    "Dave Dorman",
    crate::card::CardRules::unsupported(),
);

// NEM 92 — Mana Cache
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MANA_CACHE: CardRecord = CardRecord::new(
    "Mana Cache",
    "583a33b3-7833-48e5-88c3-849a5771ef6e",
    "rk post",
    crate::card::CardRules::unsupported(),
);

// NEM 93 — Mogg Alarm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOGG_ALARM: CardRecord = CardRecord::new(
    "Mogg Alarm",
    "f246e128-0a43-478a-a232-51020fab76d5",
    "Dave Dorman",
    crate::card::CardRules::unsupported(),
);

// NEM 94 — Mogg Salvage
pub(in crate::card::sets) static MOGG_SALVAGE: CardRecord = CardRecord::new(
    "Mogg Salvage",
    "403aa48c-b684-4c54-8863-460958055a1f",
    "Paolo Parente",
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
            crate::NO_COSTS,
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
    "Mogg Toady",
    "ee8edaf6-d46e-4efb-8bc0-ec11e06eb499",
    "Mike Ploog",
    crate::card::CardRules::unsupported(),
);

// NEM 96 — Moggcatcher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOGGCATCHER: CardRecord = CardRecord::new(
    "Moggcatcher",
    "9ba582d7-1dce-4664-8bd9-6b419596788c",
    "Pete Venters",
    crate::card::CardRules::unsupported(),
);

// NEM 97 — Rupture
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUPTURE: CardRecord = CardRecord::new(
    "Rupture",
    "db53c1fb-3641-44a3-b0b4-b7b2ba993646",
    "Gao Yan",
    crate::card::CardRules::unsupported(),
);

// NEM 98 — Seal of Fire
pub(in crate::card::sets) static SEAL_OF_FIRE: CardRecord = CardRecord::new(
    "Seal of Fire",
    "37eaf1f6-4bdc-4669-9a15-50b65e016ccf",
    "Christopher Moeller",
    CardRules::new_enchantment(mana_cost!("{R}")).with_ability(AbilityDef::activated_with_targets(
        "Sacrifice this enchantment: It deals 2 damage to any target.",
        &[CostDef::SacrificeSource],
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::damage(
            EffectRecipientDef::Target(TargetIndex::PRIMARY),
            ValueDef::Constant(2),
        ),
    )),
);

// NEM 99 — Shrieking Mogg
pub(in crate::card::sets) static SHRIEKING_MOGG: CardRecord = CardRecord::new(
    "Shrieking Mogg",
    "0ce46919-d312-490c-8942-39fbb2d375bf",
    "Dan Frazier",
    // It taps the blockers and attacks the same turn, which is the whole
    // two-card combo in one card.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin"], 1, 1).with_abilities(&[
        abilities::haste(),
        abilities::enters_trigger(
            "When this creature enters, tap all other creatures.",
            EffectDef::Tap {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
            },
        ),
    ]),
);

// NEM 100 — Stronghold Gambit
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STRONGHOLD_GAMBIT: CardRecord = CardRecord::new(
    "Stronghold Gambit",
    "0d18050e-9aad-471e-a6ae-66e5fa2bbb6f",
    "Greg Hildebrandt & Tim Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// NEM 101 — Animate Land
pub(in crate::card::sets) static ANIMATE_LAND: CardRecord = CardRecord::new(
    "Animate Land",
    "20ff4e7d-fa50-48d2-8ab6-6b86e3a05e86",
    "Rebecca Guay",
    // One mana turns a land into a blocker after attackers are declared,
    // which is a combat trick nobody plays around.
    CardRules::new_instant(mana_cost!("{G}")).with_ability(AbilityDef::spell_with_targets(
        "Until end of turn, target land becomes a 3/3 creature that's still a land.",
        &const {
            [AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Land),
            )]
        },
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                AppliedEffectDef::set_base_power_toughness(
                    ValueDef::Constant(3),
                    ValueDef::Constant(3),
                ),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// NEM 102 — Blastoderm
pub(in crate::card::sets) static BLASTODERM: CardRecord = CardRecord::new(
    "Blastoderm",
    "9db5d6c2-b11f-442a-b172-c0c99c9bec07",
    "Eric Peterson",
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
                otherwise: &EffectDef::sacrifice(EffectRecipientDef::Source),
            },
        ),
    ]),
);

// NEM 103 — Coiling Woodworm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COILING_WOODWORM: CardRecord = CardRecord::new(
    "Coiling Woodworm",
    "341a70be-2dbf-4365-9c7a-e52cb62a74fa",
    "David Martin",
    crate::card::CardRules::unsupported(),
);

// NEM 104 — Fog Patch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FOG_PATCH: CardRecord = CardRecord::new(
    "Fog Patch",
    "133f9e4f-2b1b-4a24-ad19-285a2c5845b5",
    "Rebecca Guay",
    crate::card::CardRules::unsupported(),
);

// NEM 105 — Harvest Mage
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HARVEST_MAGE: CardRecord = CardRecord::new(
    "Harvest Mage",
    "95b29329-b9a3-4d59-b0f8-2abc67337760",
    "Dan Frazier",
    crate::card::CardRules::unsupported(),
);

// NEM 106 — Mossdog
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static MOSSDOG: CardRecord = CardRecord::new(
    "Mossdog",
    "2bbe0201-3df3-4d0c-8aa4-8f35f12c322c",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// NEM 107 — Nesting Wurm
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NESTING_WURM: CardRecord = CardRecord::new(
    "Nesting Wurm",
    "5da697da-7026-4dea-b494-8314d789160f",
    "rk post",
    crate::card::CardRules::unsupported(),
);

// NEM 108 — Overlaid Terrain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OVERLAID_TERRAIN: CardRecord = CardRecord::new(
    "Overlaid Terrain",
    "230c7926-9a4b-4ead-b4c8-889f84210545",
    "DiTerlizzi",
    crate::card::CardRules::unsupported(),
);

// NEM 109 — Pack Hunt
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PACK_HUNT: CardRecord = CardRecord::new(
    "Pack Hunt",
    "1c46caa8-efc0-4b72-b122-61e5d86a5b86",
    "Sam Wood",
    crate::card::CardRules::unsupported(),
);

// NEM 110 — Refreshing Rain
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REFRESHING_RAIN: CardRecord = CardRecord::new(
    "Refreshing Rain",
    "c5e24850-bd9b-40ab-878f-b8a554da1956",
    "Don Hazeltine",
    crate::card::CardRules::unsupported(),
);

// NEM 111 — Reverent Silence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REVERENT_SILENCE: CardRecord = CardRecord::new(
    "Reverent Silence",
    "b82d3432-2167-4a65-8221-cb7b338e60d0",
    "Don Hazeltine",
    crate::card::CardRules::unsupported(),
);

// NEM 112 — Rhox
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RHOX: CardRecord = CardRecord::new(
    "Rhox",
    "58388a29-b2a6-4d16-b872-f198563721d9",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// NEM 113 — Saproling Burst
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAPROLING_BURST: CardRecord = CardRecord::new(
    "Saproling Burst",
    "de3a293d-08d8-49e4-b0fa-91afa0a5591d",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// NEM 114 — Saproling Cluster
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SAPROLING_CLUSTER: CardRecord = CardRecord::new(
    "Saproling Cluster",
    "5f50072b-aedd-4074-b1f7-f9ce477c26c2",
    "Matt Cavotta",
    crate::card::CardRules::unsupported(),
);

// NEM 115 — Seal of Strength
pub(in crate::card::sets) static SEAL_OF_STRENGTH: CardRecord = CardRecord::new(
    "Seal of Strength",
    "57650f78-3bf0-485a-bba8-7e7e14e47508",
    "Christopher Moeller",
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
    "Skyshroud Behemoth",
    "1c01d17e-45a2-4b6f-aaa5-2af9c8f26181",
    "Eric Peterson",
    crate::card::CardRules::unsupported(),
);

// NEM 117 — Skyshroud Claim
pub(in crate::card::sets) static SKYSHROUD_CLAIM: CardRecord = CardRecord::new(
    "Skyshroud Claim",
    "cf3e09ff-c917-4c0c-8ddb-e152b4b0b82c",
    "Mark Romanoski",
// Two lands untapped for four mana, which is more mana than it cost on
    // the turn it resolves.
    CardRules::new_sorcery(mana_cost!("{3}{G}")).with_ability(AbilityDef::spell(
        "Search your library for up to two Forest cards, put them onto the battlefield, then shuffle.",
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Forest")),
            minimum: 0,
            maximum: ValueDef::Constant(2),
            reveal: false,
            destination: ZoneKind::Battlefield,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: false,
            attachment: None,
            binding: None,
            then: None,
        },
    )),
);

// NEM 118 — Skyshroud Cutter
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKYSHROUD_CUTTER: CardRecord = CardRecord::new(
    "Skyshroud Cutter",
    "a558c4f5-a716-4e46-9234-5f84f1bd57aa",
    "Tony Szczudlo",
    crate::card::CardRules::unsupported(),
);

// NEM 119 — Skyshroud Poacher
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKYSHROUD_POACHER: CardRecord = CardRecord::new(
    "Skyshroud Poacher",
    "0fb4e44e-656e-4294-a53b-1f7aa96fab31",
    "Ron Spencer",
    crate::card::CardRules::unsupported(),
);

// NEM 120 — Skyshroud Ridgeback
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKYSHROUD_RIDGEBACK: CardRecord = CardRecord::new(
    "Skyshroud Ridgeback",
    "410896ab-d3dc-478c-bfd1-c0cad5b1180a",
    "Carl Critchlow",
    crate::card::CardRules::unsupported(),
);

// NEM 121 — Skyshroud Sentinel
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SKYSHROUD_SENTINEL: CardRecord = CardRecord::new(
    "Skyshroud Sentinel",
    "a35ab55f-f677-45c8-bd32-56788a776b33",
    "Randy Gallegos",
    crate::card::CardRules::unsupported(),
);

// NEM 122 — Stampede Driver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static STAMPEDE_DRIVER: CardRecord = CardRecord::new(
    "Stampede Driver",
    "a295758a-ee46-4ed0-8539-67501a37010d",
    "Ron Spears",
    crate::card::CardRules::unsupported(),
);

// NEM 123 — Treetop Bracers
pub(in crate::card::sets) static TREETOP_BRACERS: CardRecord = CardRecord::new(
    "Treetop Bracers",
    "d6d85032-26e9-44af-ab52-56d9c24e337d",
    "Heather Hudson",
// Evasion for two mana in the colour with the fewest fliers to fear,
    // which is most of what a green creature needed to keep connecting.
    CardRules::new_enchantment(mana_cost!("{1}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +1/+1 and can't be blocked except by creatures with flying.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(1),
                            ValueDef::Constant(1),
                        ),
                        AppliedEffectDef::Rule(AppliedRuleDef::BlockRestriction(
                            BlockRestrictionDef::prohibit(
                                BlockRestrictionSubjectDef::Attacker,
                                BlockRestrictionMatchDef::Except(ObjectPredicateDef::HasKeyword(
                                    KeywordAbility::Flying,
                                )),
                            ),
                        )),
                    ]),
                },
            ),
        ]),
);

// NEM 124 — Wild Mammoth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WILD_MAMMOTH: CardRecord = CardRecord::new(
    "Wild Mammoth",
    "4d927fdb-11c0-42b6-95d4-7af051e45213",
    "Bradley Williams",
    crate::card::CardRules::unsupported(),
);

// NEM 125 — Woodripper
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WOODRIPPER: CardRecord = CardRecord::new(
    "Woodripper",
    "5126b782-d74c-40ca-a9b2-a6c78f94d138",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// NEM 126 — Belbe's Armor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BELBE_S_ARMOR: CardRecord = CardRecord::new(
    "Belbe's Armor",
    "0052158b-58d1-4416-a7ce-7c6a7595263c",
    "D. Alexander Gregory",
    crate::card::CardRules::unsupported(),
);

// NEM 127 — Belbe's Portal
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BELBE_S_PORTAL: CardRecord = CardRecord::new(
    "Belbe's Portal",
    "fb4eeea1-693e-475c-a209-8a0464df8081",
    "Mark Tedin",
    crate::card::CardRules::unsupported(),
);

// NEM 128 — Complex Automaton
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static COMPLEX_AUTOMATON: CardRecord = CardRecord::new(
    "Complex Automaton",
    "5fb3c7af-74e1-4072-953b-b3e9ccd8aa03",
    "Dana Knutson",
    crate::card::CardRules::unsupported(),
);

// NEM 129 — Eye of Yawgmoth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static EYE_OF_YAWGMOTH: CardRecord = CardRecord::new(
    "Eye of Yawgmoth",
    "9c258aa1-cd9f-45e9-b478-d689b78850cd",
    "DiTerlizzi",
    crate::card::CardRules::unsupported(),
);

// NEM 130 — Flint Golem
pub(in crate::card::sets) static FLINT_GOLEM: CardRecord = CardRecord::new(
    "Flint Golem",
    "0e62aa7e-d9f9-42d4-9eed-5f51f88047c6",
    "Lou Harrison",
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
    "Flowstone Armor",
    "1160e476-8a2b-4b90-b4db-f386a80ab067",
    "Paolo Parente",
    crate::card::CardRules::unsupported(),
);

// NEM 132 — Flowstone Thopter
pub(in crate::card::sets) static FLOWSTONE_THOPTER: CardRecord = CardRecord::new(
    "Flowstone Thopter",
    "5bf016ec-2654-4c3e-8e2e-6c70c4604d28",
    "Mike Ploog",
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
    "Kill Switch",
    "a94cfd63-7f2b-4c0a-8dcb-f22cf83e1e27",
    "Brian Snõddy",
    crate::card::CardRules::unsupported(),
);

// NEM 134 — Parallax Inhibitor
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PARALLAX_INHIBITOR: CardRecord = CardRecord::new(
    "Parallax Inhibitor",
    "f758617c-f0e4-43d5-8fb4-e33eb2c5b99f",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// NEM 135 — Predator, Flagship
pub(in crate::card::sets) static PREDATOR_FLAGSHIP: CardRecord = CardRecord::new(
    "Predator, Flagship",
    "28927927-3974-48c3-81c2-518089a10003",
    "Mark Tedin",
    // It hands a creature flying and then kills the fliers, which is one
    // card answering anything the opponent puts on the board.
    CardRules::new_artifact(mana_cost!("{5}"))
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated_with_targets(
                "{2}: Target creature gains flying until end of turn.",
                &[CostDef::Mana(mana_cost!("{2}"))],
                &const {
                    [AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::HasType(CardType::Creature),
                    )]
                },
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&const { abilities::flying() }),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::activated_with_targets(
                "{5}, {T}: Destroy target creature with flying.",
                &[CostDef::Mana(mana_cost!("{5}")), CostDef::TapSource],
                &const {
                    [AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                        ]),
                    )]
                },
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            ),
        ]),
);

// NEM 136 — Rackling
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RACKLING: CardRecord = CardRecord::new(
    "Rackling",
    "10f82f2e-a6db-491b-b253-82c34cd6c940",
    "D. Alexander Gregory",
    crate::card::CardRules::unsupported(),
);

// NEM 137 — Rejuvenation Chamber
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static REJUVENATION_CHAMBER: CardRecord = CardRecord::new(
    "Rejuvenation Chamber",
    "b97f86ce-4758-4ae3-af29-b08a4d771652",
    "Alan Pollack",
    crate::card::CardRules::unsupported(),
);

// NEM 138 — Rusting Golem
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static RUSTING_GOLEM: CardRecord = CardRecord::new(
    "Rusting Golem",
    "c2605448-8e0d-492b-a635-468923c64625",
    "Greg Staples",
    crate::card::CardRules::unsupported(),
);

// NEM 139 — Tangle Wire
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static TANGLE_WIRE: CardRecord = CardRecord::new(
    "Tangle Wire",
    "ad62f313-8a8a-4ffa-ada2-b12b76288729",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// NEM 140 — Viseling
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VISELING: CardRecord = CardRecord::new(
    "Viseling",
    "a3eb86c5-d6fe-4dde-ad07-c3109b3a1611",
    "Kev Walker",
    crate::card::CardRules::unsupported(),
);

// NEM 141 — Kor Haven
pub(in crate::card::sets) static KOR_HAVEN: CardRecord = CardRecord::new(
    "Kor Haven",
    "3d5529ca-5c20-4dfd-8595-96d6dfa6debe",
    "Darrell Riche",
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
pub(in crate::card::sets) static RATH_S_EDGE: CardRecord = CardRecord::new(
    "Rath's Edge",
    "42681dce-5c63-4e56-955e-39f085ea6ae9",
    "Ron Spencer",
    // One damage for five mana and a land is a terrible rate, and the only
    // one available to a deck with no spells left.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets(
            "{4}, {T}, Sacrifice a land: Rath's Edge deals 1 damage to any target.",
            &[
                CostDef::Mana(mana_cost!("{4}")),
                CostDef::TapSource,
                CostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasType(CardType::Land),
                    controller: PlayerRelation::You,
                },
            ],
            &const {
                [AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::AnyTarget,
                )]
            },
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(1),
            ),
        ),
    ]),
);

// NEM 143 — Terrain Generator
pub(in crate::card::sets) static TERRAIN_GENERATOR: CardRecord = CardRecord::new(
    "Terrain Generator",
    "e6fe66a2-8e70-414f-bedb-8f1f85f1d2d9",
    "Alan Pollack",
    // An extra land drop every turn for two mana, which only a deck holding
    // more lands than it can play ever wants.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{2}, {T}: You may put a basic land card from your hand onto the battlefield tapped.",
            &[CostDef::Mana(mana_cost!("{2}")), CostDef::TapSource],
            EffectDef::Choose(ChooseDef {
                binding: ObjectChoiceBindingDef::Object(ParentBinding),
                unchosen: None,
                chooser: PlayerRefDef::EffectController,
                candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Supertype(CardSupertype::Basic),
                    ]),
                    &[ZoneKind::Hand],
                    PlayerRelation::You,
                )),
                exclude: None,
                minimum: 0,
                maximum: 1,
                visibility: ChoiceVisibilityDef::Public,
                then: &const {
                    EffectDef::WithBattlefieldArrival {
                        effect: &const {
                            EffectDef::MoveToZone {
                                object: EffectRecipientDef::object(ObjectRefDef::Binding(
                                    ParentBinding,
                                )),
                                zone: ZoneKind::Battlefield,
                                placement: ZonePlacement::Top,
                            }
                        },
                        arrival: crate::card::BattlefieldArrivalDef {
                            modifications: &const { [BattlefieldEntryModificationDef::Tapped] },
                            ..crate::card::BattlefieldArrivalDef::DEFAULT
                        },
                    }
                },
            }),
        ),
    ]),
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
