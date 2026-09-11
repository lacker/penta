//! Betrayers of Kamigawa cards cataloged for the Vintage Cube.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CostDef;
use crate::card::CounterKind;
use crate::card::DiscardSelectionDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ObjectPredicateDef;
use crate::card::PlayerRuleDef;
use crate::card::PlayerSetDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::abilities;
use crate::ids::TargetIndex;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "BOK",
    slug: "betrayers_of_kamigawa",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// BOK 44 — Ninja of the Deep Hours
pub(in crate::card::sets) static NINJA_OF_THE_DEEP_HOURS: CardRecord = CardRecord::new(
    "Ninja of the Deep Hours",
    "367a67c7-54db-4336-b55a-3fa27625172a",
    "Dan Murayama Scott",
    // Nobody casts this for four. Two mana off an unblocked one-drop is the
    // card: the attacker that got through goes back to be replayed, and the
    // 2/2 that replaced it is already connecting for a card a turn.
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Human", "Ninja"], 2, 2).with_abilities(&[
        abilities::ninjutsu!(
            "Ninjutsu {1}{U} ({1}{U}, Return an unblocked attacker you control to hand: Put this \
            card onto the battlefield from your hand tapped and attacking.)",
            &[crate::CostDef::Mana(mana_cost!("{1}{U}"))],
        ),
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, you may draw a card.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            },
        ),
    ]),
);

// BOK 76 — Okiba-Gang Shinobi
pub(in crate::card::sets) static OKIBA_GANG_SHINOBI: CardRecord = CardRecord::new(
    "Okiba-Gang Shinobi",
    "5cd9297e-301e-4e70-af9b-3218eacacf8d",
    "Mark Zug",
    // Two cards out of their hand every time it connects, and ninjutsu is
    // what makes it connect: the attacker they chose not to block is traded
    // for the one they would have.
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Rat", "Ninja"], 3, 2).with_abilities(&[
        abilities::ninjutsu!(
            "Ninjutsu {3}{B} ({3}{B}, Return an unblocked attacker you control to hand: Put this \
            card onto the battlefield from your hand tapped and attacking.)",
            &[CostDef::Mana(mana_cost!("{3}{B}"))],
        ),
        AbilityDef::triggered(
            "Whenever this creature deals combat damage to a player, that player discards two \
             cards.",
            TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
            EffectDef::Discard {
                recipient: EffectRecipientDef::EventPlayer,
                amount: ValueDef::Constant(2),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ),
    ]),
);

// BOK 104 — Fumiko the Lowblood
pub(in crate::card::sets) static FUMIKO_THE_LOWBLOOD: CardRecord =
    CardRecord::new(
        "Fumiko the Lowblood",
        "482678b8-bce6-4847-9f43-1761d61645d8",
        "Michael Sutfin",
        CardRules::new_creature(mana_cost!("{2}{R}{R}"), &["Human", "Samurai"], 3, 2)
            .with_supertype(CardSupertype::Legendary)
            .with_abilities(&[
                abilities::bushido(ValueDef::CountMatchingObjects(
                    &const {
                        crate::card::ObjectQueryDef::new(
                            ObjectPredicateDef::Attacking,
                            &[crate::card::ZoneKind::Battlefield],
                        )
                    },
                ))
                .override_text("Bushido X, where X is the number of attacking creatures."),
                AbilityDef::static_ability(
                    "Creatures your opponents control attack each combat if able.",
                    EffectDef::StaticApply {
                        recipient: EffectRecipientDef::matching_objects(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            &[crate::card::ZoneKind::Battlefield],
                            crate::card::PlayerRelation::Opponent,
                        ),
                        effect: AppliedEffectDef::add_ability(
                            &abilities::attacks_each_combat_if_able(),
                        ),
                    },
                ),
            ]),
    );
// BOK 154 — Mirror Gallery
pub(in crate::card::sets) static MIRROR_GALLERY: CardRecord = CardRecord::new(
    "Mirror Gallery",
    "00beba34-54cc-4a30-8424-71a1215647a6",
    "Scott M. Fischer",
    CardRules::new_artifact(mana_cost!("{5}")).with_ability(AbilityDef::static_ability(
        "The \"legend rule\" doesn't apply.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::players(PlayerSetDef::All),
            effect: AppliedEffectDef::Rule(AppliedRuleDef::PlayerRule(
                PlayerRuleDef::LegendRuleDoesNotApplyTo(&ObjectPredicateDef::Any),
            )),
        },
    )),
);

// BOK 163 — Umezawa's Jitte
pub(in crate::card::sets) static UMEZAWAS_JITTE: CardRecord = CardRecord::new(
    "Umezawa's Jitte",
    "3b6e5956-f795-451b-bb24-56462d1ced27",
    "Christopher Moeller",
CardRules::new_artifact(mana_cost!("{2}"))
        .with_supertype(CardSupertype::Legendary)
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever equipped creature deals combat damage, put two charge counters on Umezawa's Jitte.",
                // Damage to a blocker counts as readily as damage to the
                // player: the clause names no recipient at all.
                TriggerEventDef::combat_damage_dealt_by(ObjectPredicateDef::AttachedToSource),
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::named("charge"),
                    amount: ValueDef::Constant(2),
                },
            ),
            AbilityDef::modal_activated(
                "Remove a charge counter from Umezawa's Jitte: Choose one —\n• Equipped creature gets +2/+2 until end of turn.\n• Target creature gets -1/-1 until end of turn.\n• You gain 2 life.",
                &[CostDef::RemoveCountersFromSource {
                    kind: CounterKind::named("charge"),
                    amount: 1,
                }],
                // The three modes of the Jitte's counter-spending ability. Each is a clause
                // of its own with its own targets, chosen as the ability is activated.
                &[
                    AbilityDef::spell(
                        "Equipped creature gets +2/+2 until end of turn",
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::AttachedPermanent,
                            effect: AppliedEffectDef::modify_power_toughness(
                                ValueDef::Constant(2),
                                ValueDef::Constant(2),
                            ),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                    ),
                    AbilityDef::spell_with_targets(
                        "Target creature gets -1/-1 until end of turn",
                        &[AbilityTargetDef::exactly_one_permanent(
                            ObjectPredicateDef::HasType(CardType::Creature),
                        )],
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            effect: AppliedEffectDef::modify_power_toughness(
                                ValueDef::Constant(-1),
                                ValueDef::Constant(-1),
                            ),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                    ),
                    AbilityDef::spell(
                        "You gain 2 life",
                        EffectDef::GainLife {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(2),
                        },
                    ),
                ],
                1,
                1,
                false,
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{2}"))], "Equip {2}"),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &NINJA_OF_THE_DEEP_HOURS,
    &OKIBA_GANG_SHINOBI,
    &FUMIKO_THE_LOWBLOOD,
    &MIRROR_GALLERY,
    &UMEZAWAS_JITTE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
