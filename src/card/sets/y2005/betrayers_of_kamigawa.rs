//! Betrayers of Kamigawa cards cataloged for the Vintage Cube.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::BasicLandType;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CardTypeSet;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseForEachPlayerDef;
use crate::card::ColorSet;
use crate::card::CostDef;
use crate::card::CounterKind;
use crate::card::CreatureTypeSetDef;
use crate::card::DamageAssignmentDef;
use crate::card::DiscardSelectionDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::HalvedValueDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectSetDef;
use crate::card::PerPlayerSelectionDef;
use crate::card::PlayerRelation;
use crate::card::PlayerRuleDef;
use crate::card::PlayerSetDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::RoundingDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
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

// BOK 33 — Disrupting Shoal
// Audit: unsupported — The casting planner only chooses nonzero X from a variable mana payment or an X-sized additional cost. It cannot derive X from the mana value of the single card exiled for this alternative cost.
pub(in crate::card::sets) static DISRUPTING_SHOAL_33: CardRecord = CardRecord::new(
    "Disrupting Shoal",
    "15589745-4c0a-4edf-ad45-3b7fa45e70c5",
    "Scott M. Fischer",
    crate::card::CardRules::unsupported(),
);

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

// BOK 96 — Blazing Shoal
// Audit: unsupported — The casting planner only chooses nonzero X from a variable mana payment or an X-sized additional cost. It cannot derive X from the mana value of the single card exiled for this alternative cost.
pub(in crate::card::sets) static BLAZING_SHOAL_96: CardRecord = CardRecord::new(
    "Blazing Shoal",
    "8b915daa-d239-4460-bd6b-e1327fdf7f51",
    "Glen Angus",
    crate::card::CardRules::unsupported(),
);

// BOK 98 — Crack the Earth
pub(in crate::card::sets) static CRACK_THE_EARTH_98: CardRecord = CardRecord::new(
    "Crack the Earth",
    "8ab16152-4617-4deb-b995-195e21f8f485",
    "Wayne Reynolds",
    CardRules::new_sorcery(mana_cost!("{R}"))
        .with_subtypes(&["Arcane"])
        .with_abilities(&[AbilityDef::spell(
            "Each player sacrifices a permanent of their choice.",
            EffectDef::ChooseForEachPlayer(ChooseForEachPlayerDef {
                player: EffectRecipientDef::EachPlayer,
                candidates: ObjectPredicateDef::Any,
                zone: ZoneKind::Battlefield,
                selection: PerPlayerSelectionDef::Count(ValueDef::Constant(1)),
                visibility: ChoiceVisibilityDef::Public,
                chosen: Binding!("sacrifice_chosen"),
                unchosen: Binding!("sacrifice_unchosen"),
                then: &EffectDef::sacrifice(EffectRecipientDef::objects(ObjectSetDef::Binding(
                    Binding!("sacrifice_chosen"),
                ))),
            }),
        )]),
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
                    &crate::card::ObjectQueryDef::new(
                        ObjectPredicateDef::Attacking,
                        &[crate::card::ZoneKind::Battlefield],
                    ),
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
// BOK 107 — Heartless Hidetsugu
pub(in crate::card::sets) static HEARTLESS_HIDETSUGU_107: CardRecord = CardRecord::new(
    "Heartless Hidetsugu",
    "4a3ab177-d9ab-46bf-bd92-20a9ecf2d0ad",
    "Carl Critchlow",
    CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Ogre", "Shaman"], 4, 3).with_supertype(CardSupertype::Legendary).with_abilities(&[
AbilityDef::activated("{T}: Heartless Hidetsugu deals damage to each player equal to half that player's life total, rounded down.", &[CostDef::TapSource], EffectDef::damage_simultaneously(&[DamageAssignmentDef::from_effect(EffectRecipientDef::Controller, ValueDef::Halved(&HalvedValueDef { value: ValueDef::LifeTotal(PlayerRelation::You), rounding: RoundingDef::Down })), DamageAssignmentDef::from_effect(EffectRecipientDef::Opponent, ValueDef::Halved(&HalvedValueDef { value: ValueDef::LifeTotal(PlayerRelation::Opponent), rounding: RoundingDef::Down }))]))
]),
);

// BOK 126 — Genju of the Cedars
pub(in crate::card::sets) static GENJU_OF_THE_CEDARS: CardRecord = CardRecord::new(
    "Genju of the Cedars",
    "9d621b82-c863-437b-b42c-31a0872be6d4",
    "Arnie Swekel",
    CardRules::new_enchantment(mana_cost!("{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell(
                "Enchant Forest",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
                )],
            ),
            AbilityDef::activated(
                "{2}: Enchanted Forest becomes a 4/4 green Spirit creature until end of turn. It's still a land.",
                &[CostDef::Mana(mana_cost!("{2}"))],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                        AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&[
                            "Spirit",
                        ])),
                        AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::Green])),
                        AppliedEffectDef::set_base_power_toughness(
                            ValueDef::Constant(4),
                            ValueDef::Constant(4),
                        ),
                    ]),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::triggered(
                "When enchanted Forest is put into a graveyard, you may return this card from your graveyard to your hand.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::AttachedToSource,
                        ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
                    ]),
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    effect: &EffectDef::move_to_zone(
                        EffectRecipientDef::SourceZoneChangeSuccessor,
                        ZoneKind::Hand,
                        ZonePlacement::Top,
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
    &DISRUPTING_SHOAL_33,
    &NINJA_OF_THE_DEEP_HOURS,
    &OKIBA_GANG_SHINOBI,
    &BLAZING_SHOAL_96,
    &CRACK_THE_EARTH_98,
    &FUMIKO_THE_LOWBLOOD,
    &HEARTLESS_HIDETSUGU_107,
    &GENJU_OF_THE_CEDARS,
    &MIRROR_GALLERY,
    &UMEZAWAS_JITTE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
