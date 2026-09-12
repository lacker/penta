//! Jumpstart 2022 card records.

use super::{CardRecord, PrintingRecord};
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::ActivationTimingDef;
use crate::card::AppliedEffectDef;
use crate::card::BattlefieldArrivalDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::CostDef;
use crate::card::CounterKind;
use crate::card::CreatureTypeSetDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::PlayerRelation;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SubtypeDef;
use crate::card::TriggerConditionDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "J22",
    slug: "jumpstart-2022",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// J22 5 — Ingenious Leonin
pub(in crate::card::sets) static INGENIOUS_LEONIN: CardRecord = CardRecord::new(
    "Ingenious Leonin",
    "4c6fd0bf-3f02-46f3-9c9a-931eab190584",
    "Eric Deschamps",
    CardRules::new_creature(mana_cost!("{4}{W}"), &["Cat", "Soldier"], 4, 4).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{3}{W}: Put a +1/+1 counter on another target attacking \
             creature you control. If that creature is a Cat, it gains \
             first strike until end of turn. (It deals combat damage \
             before creatures without first strike.)",
            &[CostDef::Mana(mana_cost!("{3}{W}"))],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ObjectPredicateDef::Attacking,
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::Sequence(&[
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::TargetMatches {
                        slot: TargetIndex::PRIMARY,
                        object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Cat")),
                    },
                    then: &EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                },
            ]),
        ),
    ]),
);

// J22 22 — Deadly Plot
pub(in crate::card::sets) static DEADLY_PLOT: CardRecord = CardRecord::new(
    "Deadly Plot",
    "9c78da23-e7ec-4a3d-9f79-09fb86993b26",
    "Peter Polach",
    CardRules::new_instant(mana_cost!("{3}{B}")).with_abilities(&[AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Destroy target creature or planeswalker.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Planeswalker),
                    ]),
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    then: None,
                },
            ),
            AbilityDef::spell_with_targets(
                "Return target Zombie creature card from your graveyard to the \
                 battlefield tapped.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Zombie")),
                        ]),
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: Some(PlayerRelation::You),
                    },
                )],
                EffectDef::WithBattlefieldArrival {
                    effect: &EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Battlefield,
                        ZonePlacement::Top,
                    ),
                    arrival: BattlefieldArrivalDef {
                        modifications: &[BattlefieldEntryModificationDef::Tapped],
                        ..BattlefieldArrivalDef::DEFAULT
                    },
                },
            ),
        ],
    )]),
);

// J22 27 — Suspicious Shambler
pub(in crate::card::sets) static SUSPICIOUS_SHAMBLER: CardRecord = CardRecord::new(
    "Suspicious Shambler",
    "ff36347c-1fc1-4493-8e45-ef3372ecce45",
    "Javier Charro",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Zombie"], 4, 2).with_abilities(&[
        AbilityDef::activated(
            "{4}{B}{B}, Exile this card from your graveyard: Create two \
             2/2 black Zombie creature tokens. Activate only as a sorcery.",
            &[CostDef::Mana(mana_cost!("{4}{B}{B}")), CostDef::ExileSource],
            EffectDef::create_creature_token(&["Zombie"], &[ManaColor::Black], 2, 2)
                .with_count(ValueDef::Constant(2)),
        )
        .with_activation_timing(ActivationTimingDef::SorcerySpeed)
        .with_source_zones(&[ZoneKind::Graveyard]),
    ]),
);

// J22 41 — Mild-Mannered Librarian
pub(in crate::card::sets) static MILD_MANNERED_LIBRARIAN: CardRecord = CardRecord::new(
    "Mild-Mannered Librarian",
    "3eee6f29-ef06-47f6-99af-fb0ff88f09d0",
    "Justyna Dura",
    CardRules::new_creature(mana_cost!("{G}"), &["Human"], 1, 1).with_abilities(&[
        AbilityDef::activated(
            "{3}{G}: This creature becomes a Werewolf. Put two +1/+1 \
             counters on it and you draw a card. Activate only once.",
            &[CostDef::Mana(mana_cost!("{3}{G}"))],
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&[
                        "Werewolf",
                    ])),
                    duration: ResolvedEffectDurationDef::Permanent,
                },
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(2),
                },
                abilities::draw_cards(ValueDef::Constant(1)),
            ]),
        )
        .once_per_object(),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &INGENIOUS_LEONIN,
    &DEADLY_PLOT,
    &SUSPICIOUS_SHAMBLER,
    &MILD_MANNERED_LIBRARIAN,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
