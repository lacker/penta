//! Adventures in the Forgotten Realms cards cataloged for the Vintage Cube
//! pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::ActivationTimingDef;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CardTypeSet;
use crate::card::CharacteristicOperationDef;
use crate::card::ColorSet;
use crate::card::ComparisonDef;
use crate::card::ConditionDef;
use crate::card::CostDef;
use crate::card::CounterKind;
use crate::card::CreatureTypeSetDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectCountConditionDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::PlayerRelation;
use crate::card::ReplacementEffectDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SetOperationDef;
use crate::card::SubtypeDef;
use crate::card::SumValueDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::ValueComparisonDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::card::tokens;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "AFR",
    slug: "adventures-in-the-forgotten-realms",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

/// CR 716.2a: each bar is an activation plus a conditional ability grant.
/// The level belongs to the permanent, including when copied abilities change.
macro_rules! class_level {
    ($text:literal, $cost:literal, $level:literal, [$($ability:expr),+ $(,)?] $(,)?) => {
        [
            $crate::card::AbilityDef::activated(
                $text,
                &[$crate::card::CostDef::Mana($crate::mana_cost!($cost))],
                $crate::card::EffectDef::GainClassLevel { level: $level },
            ).with_activation_timing($crate::card::ActivationTimingDef::SorcerySpeed)
             .with_activation_condition(&$crate::card::TriggerConditionDef::SourceClassLevel {
                 comparison: $crate::card::ComparisonDef::Equal, level: $level - 1,
             }),
            $($crate::card::AbilityDef::static_ability(
                ($ability).text,
                $crate::card::EffectDef::IfCondition {
                    condition: &$crate::card::TriggerConditionDef::SourceClassLevel {
                        comparison: $crate::card::ComparisonDef::GreaterOrEqual, level: $level,
                    },
                    then: &$crate::card::EffectDef::StaticApply {
                        recipient: $crate::card::EffectRecipientDef::Source,
                        effect: $crate::card::AppliedEffectDef::add_ability(&$ability),
                    },
                },
            )),+
        ]
    };
}
pub(crate) use class_level;

// AFR 6 — Cleric Class
pub(in crate::card::sets) static CLERIC_CLASS: CardRecord = CardRecord::new(
    "Cleric Class",
    "47ce8b7e-d8e1-489a-a69e-99089eeb8739",
    "Alayna Danner",
    CardRules::new_enchantment(mana_cost!("{W}"))
        .with_subtypes(&["Class"])
        .with_abilities(&crate::ability_list![
            [AbilityDef::replacement_for(
                "If you would gain life, you gain that much life plus 1 instead.",
                crate::card::ReplacementEventDef::WouldGainLife(PlayerRelation::You),
                ReplacementEffectDef::AddToEventAmount(1),
            )],
            class_level!(
                "{3}{W}: Level 2",
                "{3}{W}",
                2,
                [AbilityDef::triggered_with_targets(
                    "Whenever you gain life, put a +1/+1 counter on target creature you control.",
                    TriggerEventDef::LifeGained(PlayerRelation::You),
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::HasType(CardType::Creature),
                            zones: &[ZoneKind::Battlefield],
                            controller: Some(PlayerRelation::You),
                            owner: None,
                        }
                    )],
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                )]
            ),
            class_level!(
                "{4}{W}: Level 3",
                "{4}{W}",
                3,
                [AbilityDef::triggered_with_targets(
                    "When this Class becomes level 3, return target creature card \
                     from your graveyard to the battlefield. You gain life equal \
                     to that creature's toughness.",
                    TriggerEventDef::BecomesLevel(3),
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::HasType(CardType::Creature),
                            zones: &[ZoneKind::Graveyard],
                            controller: None,
                            owner: Some(PlayerRelation::You),
                        }
                    )],
                    EffectDef::WithZoneMoveResult {
                        effect: &EffectDef::move_to_zone(
                            EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            ZoneKind::Battlefield,
                            ZonePlacement::Top,
                        ),
                        binding: crate::Binding!("returned"),
                        then: &EffectDef::GainLife {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::AggregateObjectValues(
                                &crate::card::ObjectValueAggregateDef {
                                    objects:
                                        crate::card::ObjectSetDef::ZoneChangeSuccessorsOfBinding(
                                            crate::Binding!("returned")
                                        ),
                                    select: crate::card::ObjectValueDef::Toughness,
                                    operation: crate::card::AggregateOperationDef::Sum,
                                }
                            ),
                        },
                    },
                )]
            ),
        ]),
);

// AFR 29 — Paladin Class
// Audit: unsupported — Needs level-granted static anthem evaluation and opponent spell taxation during your turn.
pub(in crate::card::sets) static PALADIN_CLASS: CardRecord = CardRecord::new(
    "Paladin Class",
    "5bf81fb1-7992-4ae9-b1a8-80c31579a2bf",
    "Campbell White",
    CardRules::unsupported(),
);

// AFR 33 — Portable Hole
pub(in crate::card::sets) static PORTABLE_HOLE: CardRecord = CardRecord::new(
    "Portable Hole",
    "80fca8c0-ae3e-439e-b202-228b9f360e9a",
    "John Stanko",
    // One white mana answers most of what a fast deck opens on, and it
    // answers it at instant speed on the other player's turn only because
    // somebody flashed it in -- otherwise the Hole is simply the cheapest
    // unconditional removal a white deck gets.
    CardRules::new_artifact(mana_cost!("{W}")).with_ability(
        abilities::enters_trigger_with_targets(
            "When this artifact enters, exile target nonland permanent an opponent controls with \
         mana value 2 or less until this artifact leaves the battlefield.",
            // A cheap nonland permanent across the table. Mana value is read off the
            // card, so a token is a zero and qualifies.
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                        ObjectPredicateDef::ManaValueAtMost(2),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            abilities::exile_until_source_leaves(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
        ),
    ),
);

// AFR 42 — You Hear Something on Watch
pub(in crate::card::sets) static YOU_HEAR_SOMETHING_ON_WATCH: CardRecord = CardRecord::new(
    "You Hear Something on Watch",
    "76e939ab-9d0c-4685-805c-c8bc4e6af163",
    "Zezhou Chen",
    // A combat trick or a removal spell for the same two mana, chosen after
    // blockers, which is what makes holding it up rarely wrong.
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell(
                "Rouse the Party — Creatures you control get +1/+1 until end of turn.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::spell_with_targets(
                "Set Off Traps — This spell deals 5 damage to target attacking creature.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Attacking,
                    ]),
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(5),
                ),
            ),
        ],
    )),
);

// AFR 81 — Wizard Class
pub(in crate::card::sets) static WIZARD_CLASS: CardRecord = CardRecord::new(
    "Wizard Class",
    "d1f629fb-b097-4240-8560-ef47f5678f48",
    "Marta Nael",
    CardRules::new_enchantment(mana_cost!("{U}"))
        .with_subtypes(&["Class"])
        .with_abilities(&crate::ability_list![
            [AbilityDef::static_ability(
                "You have no maximum hand size.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Controller,
                    effect: AppliedEffectDef::Rule(crate::card::AppliedRuleDef::PlayerRule(
                        crate::card::PlayerRuleDef::NoMaximumHandSize,
                    )),
                },
            )],
            class_level!(
                "{2}{U}: Level 2",
                "{2}{U}",
                2,
                [AbilityDef::triggered(
                    "When this Class becomes level 2, draw two cards.",
                    TriggerEventDef::BecomesLevel(2),
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(2),
                    },
                )],
            ),
            class_level!(
                "{4}{U}: Level 3",
                "{4}{U}",
                3,
                [AbilityDef::triggered_with_targets(
                    "Whenever you draw a card, put a +1/+1 counter on target \
                     creature you control.",
                    TriggerEventDef::DrewCard(crate::card::DrawEventMatcherDef::any(
                        PlayerRelation::You,
                    )),
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::HasType(CardType::Creature),
                            zones: &[ZoneKind::Battlefield],
                            controller: Some(PlayerRelation::You),
                            owner: None,
                        },
                    )],
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Target(TargetIndex(0)),
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                )],
            ),
        ]),
);

// AFR 119 — Shambling Ghast
pub(in crate::card::sets) static SHAMBLING_GHAST_119: CardRecord = CardRecord::new(
    "Shambling Ghast",
    "d96198a7-dd19-4940-bf8f-23135011fc84",
    "Dave Kendall",
    CardRules::new_creature(mana_cost!("{B}"), &["Zombie"], 1, 1).with_ability(
        AbilityDef::modal_triggered(
            "When this creature dies, choose one —
• Brave the Stench — Target creature an opponent controls gets -1/-1 until end of turn.
• Search the Body — Create a Treasure token.",
            crate::card::TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            &[
                AbilityDef::spell_with_targets(
                    "Target creature an opponent controls gets -1/-1 until end of turn.",
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::HasType(CardType::Creature),
                            zones: &[ZoneKind::Battlefield],
                            controller: Some(PlayerRelation::Opponent),
                            owner: None,
                        },
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
                    "Create a Treasure token.",
                    EffectDef::CreateToken(crate::card::CreateTokenDef::new(
                        crate::card::TokenDef::Literal(tokens::treasure()),
                    )),
                ),
            ],
        ),
    ),
);

// AFR 123 — Vampire Spawn
pub(in crate::card::sets) static VAMPIRE_SPAWN: CardRecord = CardRecord::new(
    "Vampire Spawn",
    "b8975c72-b2ec-4c5f-86a4-4e1e3bb41c15",
    "Alex Brock",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Vampire"], 2, 3).with_abilities(&[
        abilities::enters_trigger(
            "When this creature enters, each opponent loses 2 life and you \
             gain 2 life.",
            EffectDef::Sequence(&[
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Opponent,
                    amount: ValueDef::Constant(2),
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(2),
                },
            ]),
        ),
    ]),
);

// AFR 125 — Warlock Class
// Audit: unsupported — Needs each opponent's total life lost this turn, rather than net life change.
pub(in crate::card::sets) static WARLOCK_CLASS: CardRecord = CardRecord::new(
    "Warlock Class",
    "b7faf899-96b7-454e-b634-6684c2d72f26",
    "Kieran Yanner",
    CardRules::unsupported(),
);

// AFR 131 — Barbarian Class
// Audit: unsupported — Needs dice-roll replacement and dice-roll trigger events.
pub(in crate::card::sets) static BARBARIAN_CLASS: CardRecord = CardRecord::new(
    "Barbarian Class",
    "647c2269-bdc7-4455-9158-73abbff6e50e",
    "Campbell White",
    CardRules::unsupported(),
);

// AFR 132 — Battle Cry Goblin
// Audit: unsupported — Attack history cannot sum the power of creatures as they were declared this combat. A resolution-time query of attacking creatures changes after removal and pump effects and cannot implement the intervening-if pack-tactics check.
pub(in crate::card::sets) static BATTLE_CRY_GOBLIN_132: CardRecord = CardRecord::new(
    "Battle Cry Goblin",
    "9766a427-2bb3-4028-a502-d1194cdc93aa",
    "April Prime",
    crate::card::CardRules::unsupported(),
);

// AFR 147 — Hobgoblin Bandit Lord
// Audit: unsupported — The turn-history vocabulary does not count creatures of a specified type entering under a player's control. A live EnteredThisTurn query would lose Goblins that have since left the battlefield.
pub(in crate::card::sets) static HOBGOBLIN_BANDIT_LORD_147: CardRecord = CardRecord::new(
    "Hobgoblin Bandit Lord",
    "09e9dc36-f2d8-4384-98cb-e44c00b02433",
    "Mark Zug",
    crate::card::CardRules::unsupported(),
);

// AFR 158 — Plundering Barbarian
pub(in crate::card::sets) static PLUNDERING_BARBARIAN_158: CardRecord = CardRecord::new(
    "Plundering Barbarian",
    "d875881c-c285-47f1-8a37-e4a0239d47ec",
    "Andrew Mar",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Dwarf", "Barbarian"], 2, 2).with_ability(
        AbilityDef::modal_triggered(
            "When this creature enters, choose one —\n• Smash the Chest — Destroy target artifact.\n• Pry It Open — Create a Treasure token.",
            crate::card::TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[
                AbilityDef::spell_with_targets(
                    "Destroy target artifact.",
                    &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Artifact))],
                    EffectDef::Destroy { object: EffectRecipientDef::Target(TargetIndex::PRIMARY), then: None },
                ),
                AbilityDef::spell("Create a Treasure token.", EffectDef::CreateToken(crate::card::CreateTokenDef::new(crate::card::TokenDef::Literal(tokens::treasure())))),
            ],
        ),
    ),
);

// AFR 164 — Unexpected Windfall
pub(in crate::card::sets) static UNEXPECTED_WINDFALL_164: CardRecord = CardRecord::new(
    "Unexpected Windfall",
    "bae6a5fb-39f5-4cf8-85f7-661cb4570507",
    "Alayna Danner",
    CardRules::new_instant(mana_cost!("{2}{R}{R}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, discard a card. Draw two cards and create two Treasure tokens.",
            &[],
            CostDef::DiscardCards(1),
            EffectDef::Sequence(&[
                EffectDef::DrawCards { recipient: EffectRecipientDef::Controller, amount: ValueDef::Constant(2) },
                EffectDef::CreateToken(crate::card::CreateTokenDef::new(crate::card::TokenDef::Literal(tokens::treasure())).with_count(ValueDef::Constant(2))),
            ]),
        ),
    ),
);

// AFR 176 — Circle of Dreams Druid
pub(in crate::card::sets) static CIRCLE_OF_DREAMS_DRUID_176: CardRecord = CardRecord::new(
    "Circle of Dreams Druid",
    "be6fdec0-a2c4-4da2-ae14-961185eaee66",
    "Sam Guay",
    CardRules::new_creature(mana_cost!("{G}{G}{G}"), &["Elf", "Druid"], 2, 1).with_ability(
        AbilityDef::activated_mana(
            "{T}: Add {G} for each creature you control.",
            &[CostDef::TapSource],
            EffectDef::AddMana(
                AddManaEffectDef::one(ManaColor::Green).with_variable_amount(
                    ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                ),
            ),
        ),
    ),
);

// AFR 180 — Druid Class
// Audit: unsupported — Needs level-granted additional-land permission and durable animation with a granted characteristic-defining ability.
pub(in crate::card::sets) static DRUID_CLASS: CardRecord = CardRecord::new(
    "Druid Class",
    "09278e95-eaae-4cd4-a0d8-a2d15b0abb58",
    "Svetlin Velinov",
    CardRules::unsupported(),
);

// AFR 198 — Owlbear
pub(in crate::card::sets) static OWLBEAR: CardRecord = CardRecord::new(
    "Owlbear",
    "30e8a00f-8131-470d-8072-4c23b812281a",
    "Ilse Gort",
    // "Keen Senses" is an ability word: flavour on the front of the clause
    // that changes nothing about how it works.
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Bird", "Bear"], 4, 4).with_abilities(&[
        abilities::trample(),
        abilities::enters_trigger(
            "Keen Senses — When this creature enters, draw a card.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// AFR 202 — Ranger Class
// Audit: unsupported — Needs level-granted top-library visibility and creature-casting permissions.
pub(in crate::card::sets) static RANGER_CLASS: CardRecord = CardRecord::new(
    "Ranger Class",
    "7ca392ca-3219-4694-9a74-aa079c76b91e",
    "Suzanne Helmigh",
    CardRules::unsupported(),
);

// AFR 207 — The Tarrasque
pub(in crate::card::sets) static THE_TARRASQUE_207: CardRecord = CardRecord::new(
    "The Tarrasque",
    "8a26fa15-d81f-4152-ae33-e91aa276b3fc",
    "Filip Burburan",
    CardRules::new_creature(mana_cost!("{6}{G}{G}{G}"), &["Dinosaur"], 10, 10).with_supertype(CardSupertype::Legendary).with_abilities(&[
AbilityDef::static_ability("The Tarrasque has haste and ward {10} as long as it was cast.", EffectDef::IfCondition { condition: &TriggerConditionDef::SourceWasCast, then: &EffectDef::StaticApply { recipient: EffectRecipientDef::Source, effect: AppliedEffectDef::Composite(&[AppliedEffectDef::add_ability(&abilities::haste()), AppliedEffectDef::add_ability(&abilities::ward(&[CostDef::Mana(mana_cost!("{10}"))], "Ward {10}"))]) } }),
AbilityDef::triggered_with_targets("Whenever The Tarrasque attacks, it fights target creature defending player controls.", TriggerEventDef::attacks(ObjectPredicateDef::Source), &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object { object: ObjectPredicateDef::HasType(CardType::Creature), zones: &[ZoneKind::Battlefield], controller: Some(PlayerRelation::Opponent), owner: None })], EffectDef::Fight { first: crate::card::ObjectRefDef::Source, second: crate::card::ObjectRefDef::Target(TargetIndex::PRIMARY), excess: None })
]),
);

// AFR 215 — You Meet in a Tavern
pub(in crate::card::sets) static YOU_MEET_IN_A_TAVERN: CardRecord = CardRecord::new(
    "You Meet in a Tavern",
    "593aa59a-4025-4df8-9f27-188fc7712fde",
    "Zoltan Boros",
    // Refuel or finish, chosen on the turn it is cast, which is what four
    // mana buys in a deck that is sometimes ahead and sometimes empty.
    CardRules::new_sorcery(mana_cost!("{2}{G}{G}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell(
                "Form a Party — Look at the top five cards of your library. You may reveal any \
                 number of creature cards from among them and put them into your hand. Put the \
                 rest on the bottom of your library in a random order.",
                // "Any number" is nought through five, so a whiff takes
                // nothing and still buries the five.
                abilities::look_at_top_cards_reveal_choice_to_hand_rest_random_bottom(
                    ValueDef::Constant(5),
                    ObjectPredicateDef::HasType(CardType::Creature),
                    0,
                    5,
                ),
            ),
            AbilityDef::spell(
                "Start a Brawl — Creatures you control get +2/+2 until end of turn.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ],
    )),
);

// AFR 217 — Bard Class
// Audit: unsupported — Needs level-granted static colored-mana cost modifiers; the existing colored-symbol reduction is read only from intrinsic clauses.
pub(in crate::card::sets) static BARD_CLASS: CardRecord = CardRecord::new(
    "Bard Class",
    "37d6343a-c514-4ca6-a415-62d1a473ae20",
    "Andrew Mar",
    CardRules::unsupported(),
);

// AFR 222 — Fighter Class
// Audit: unsupported — Needs equip-only activation cost reduction and mandatory blocking of a particular attacker.
pub(in crate::card::sets) static FIGHTER_CLASS: CardRecord = CardRecord::new(
    "Fighter Class",
    "d54a8329-b940-42c9-8ace-1d74407d14cb",
    "Volkan Baǵa",
    CardRules::unsupported(),
);

// AFR 228 — Monk Class
// Audit: unsupported — Needs card-bound exile casting permission conditional on another spell cast this turn.
pub(in crate::card::sets) static MONK_CLASS: CardRecord = CardRecord::new(
    "Monk Class",
    "b2edd708-46ee-4963-b7e6-b631616d78fe",
    "Randy Vargas",
    CardRules::unsupported(),
);

// AFR 230 — Rogue Class
// Audit: unsupported — Needs source-linked face-down exile with persistent look permission and level-gated play permission.
pub(in crate::card::sets) static ROGUE_CLASS: CardRecord = CardRecord::new(
    "Rogue Class",
    "0727f65b-cfbe-47d5-87c6-239cf8d93ca6",
    "Véronique Meignaud",
    CardRules::unsupported(),
);

// AFR 233 — Sorcerer Class
// Audit: unsupported — Needs mana restricted specifically to Class-level bar activations, alongside instant or sorcery casting.
pub(in crate::card::sets) static SORCERER_CLASS: CardRecord = CardRecord::new(
    "Sorcerer Class",
    "f754b385-a28d-48de-a91f-2b4f33cc47f7",
    "Alexander Mokhov",
    CardRules::unsupported(),
);

// AFR 258 — Hive of the Eye Tyrant
pub(in crate::card::sets) static HIVE_OF_THE_EYE_TYRANT_258: CardRecord = CardRecord::new(
"Hive of the Eye Tyrant",
"9eb391dc-0378-4793-a5de-899b09792a4b",
"Johannes Voss",
CardRules::new_land(&[]).with_abilities(&[AbilityDef::as_enters("If you control two or more other lands, this land enters tapped.", ReplacementEffectDef::Conditional { condition: ConditionDef::ObjectCount(&ObjectCountConditionDef { query: ObjectQueryDef::matching(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Land),
ObjectPredicateDef::Not(&ObjectPredicateDef::Source)]), &[ZoneKind::Battlefield], PlayerRelation::You), comparison: ComparisonDef::GreaterOrEqual, amount: 2 }), if_true: &[ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped)], if_false: &[] }),
AbilityDef::activated_mana("{T}: Add {B}.", &[CostDef::TapSource], EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Black))),
AbilityDef::activated("{3}{B}: Until end of turn, this land becomes a 3/3 black Beholder creature \
 with menace and \"Whenever this creature attacks, exile target card from \
 defending player's graveyard.\" It's still a land.", &[CostDef::Mana(mana_cost!("{3}{B}"))], EffectDef::Apply { recipient: EffectRecipientDef::Source, effect: AppliedEffectDef::Composite(&[AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)), AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(3), ValueDef::Constant(3)), AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::Black])), AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&["Beholder"])), AppliedEffectDef::add_ability(&abilities::menace()), AppliedEffectDef::add_ability(&AbilityDef::triggered_with_targets("Whenever this creature attacks, exile target card from defending player's graveyard.", TriggerEventDef::attacks(ObjectPredicateDef::Source), &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object { object: ObjectPredicateDef::Any, zones: &[ZoneKind::Graveyard], controller: None, owner: Some(PlayerRelation::Opponent) })], EffectDef::move_to_zone(EffectRecipientDef::Target(TargetIndex::PRIMARY), ZoneKind::Exile, ZonePlacement::Top)))]), duration: ResolvedEffectDurationDef::UntilEndOfTurn })]));

// AFR 296 — Old Gnawbone
pub(in crate::card::sets) static OLD_GNAWBONE_296: CardRecord = CardRecord::new(
    "Old Gnawbone",
    "16ead969-8ba4-4587-a68d-47730943605e",
    "Randy Vargas",
    CardRules::new_creature(mana_cost!("{5}{G}{G}"), &["Dragon"], 7, 7).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever a creature you control deals combat damage to a player, create that many Treasure tokens.",
            crate::card::TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::CreateToken(crate::card::CreateTokenDef::new(crate::card::TokenDef::Literal(tokens::treasure())).with_count(ValueDef::TriggerEventAmount)),
        ),
    ]).with_supertype(crate::card::CardSupertype::Legendary),
);

// AFR 304 — Oswald Fiddlebender
pub(in crate::card::sets) static OSWALD_FIDDLEBENDER_304: CardRecord = CardRecord::new(
    "Oswald Fiddlebender",
    "853c9db7-504e-4dfb-8067-abd2a36f6a1a",
    "Phil Stone",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Gnome", "Artificer"], 2, 2).with_supertype(CardSupertype::Legendary).with_abilities(&[
AbilityDef::activated("Magical Tinkering — {W}, {T}, Sacrifice an artifact: Search your library for an artifact card with mana value equal to 1 plus the sacrificed artifact's mana value, put it onto the battlefield, then shuffle. Activate only as a sorcery.", &[CostDef::Mana(mana_cost!("{W}")), CostDef::TapSource, CostDef::sacrifice_permanent(ObjectPredicateDef::HasType(CardType::Artifact))], EffectDef::SearchZone { player: EffectRecipientDef::Controller, source: ZoneKind::Library, object: ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Artifact), ObjectPredicateDef::ManaValueEqualTo(ValueDef::Sum(&SumValueDef { left: ValueDef::Constant(1), right: ValueDef::SacrificedManaValue }))]), minimum: 0, maximum: ValueDef::Constant(1), reveal: false, destination: ZoneKind::Battlefield, placement: ZonePlacement::Top, shuffle: true, enters_tapped: false, attachment: None, binding: None, then: None }).with_activation_timing(ActivationTimingDef::SorcerySpeed)
]),
);

// AFR 317 — Delina, Wild Mage
// Audit: unsupported — The repeating d20 roll with an optional reroll has no declarative dice procedure.
pub(in crate::card::sets) static DELINA_WILD_MAGE_317: CardRecord = CardRecord::new(
    "Delina, Wild Mage",
    "c4f1dc68-868b-4f3f-b10a-62842d88edc4",
    "Justine Mara Andersen",
    crate::card::CardRules::unsupported(),
);

// AFR 322 — Xorn
// Audit: unsupported — Its replacement needs to recognize Treasure-token creation and add one to that event.
pub(in crate::card::sets) static XORN_322: CardRecord = CardRecord::new(
    "Xorn",
    "713a6502-5239-449a-b04a-c82d525f9916",
    "Justine Jones",
    crate::card::CardRules::unsupported(),
);

// AFR 351 — Den of the Bugbear
pub(in crate::card::sets) static DEN_OF_THE_BUGBEAR_351: CardRecord = CardRecord::new(
    "Den of the Bugbear",
    "565be37a-4c14-420b-a07c-18e21f7fd731",
    "Jeff Easley",
    CardRules::new_land(&[]).with_abilities(&[
AbilityDef::as_enters("If you control two or more other lands, this land enters tapped.", ReplacementEffectDef::Conditional { condition: ConditionDef::ObjectCount(&ObjectCountConditionDef { query: ObjectQueryDef::matching(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Land), ObjectPredicateDef::Not(&ObjectPredicateDef::Source)]), &[ZoneKind::Battlefield], PlayerRelation::You), comparison: ComparisonDef::GreaterOrEqual, amount: 2 }), if_true: &[ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped)], if_false: &[] }),
abilities::tap_for(ManaColor::Red),
AbilityDef::activated("{3}{R}: Until end of turn, this land becomes a 3/2 red Goblin creature with \"Whenever this creature attacks, create a 1/1 red Goblin creature token that's tapped and attacking.\" It's still a land.", &[CostDef::Mana(mana_cost!("{3}{R}"))], EffectDef::Apply { recipient: EffectRecipientDef::Source, effect: AppliedEffectDef::Composite(&[AppliedEffectDef::Characteristic(CharacteristicOperationDef::CardTypes(SetOperationDef::Add(CardTypeSet::single(CardType::Creature)))), AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(3), ValueDef::Constant(2)), AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&["Goblin"])), AppliedEffectDef::set_colors(ColorSet::from_colors(&[ManaColor::Red])), AppliedEffectDef::add_ability(&AbilityDef::triggered("Whenever this creature attacks, create a 1/1 red Goblin creature token that is tapped and attacking.", TriggerEventDef::attacks(ObjectPredicateDef::Source), EffectDef::CreateToken(crate::card::CreateTokenDef::new(crate::card::TokenDef::Literal(crate::card::TokenCharacteristics::creature(&["Goblin"], &[ManaColor::Red], 1, 1))).entering_tapped().entering_attacking())))]), duration: ResolvedEffectDurationDef::UntilEndOfTurn })
]),
);

// AFR 358 — Treasure Vault
// Audit: unsupported — The activated-cost planner supports a single X symbol only; it cannot enumerate and pay the doubled variable mana cost {X}{X}.
pub(in crate::card::sets) static TREASURE_VAULT_358: CardRecord = CardRecord::new(
    "Treasure Vault",
    "8d75f53d-d105-4973-ab2e-58cdf3a41eed",
    "Erol Otus",
    CardRules::unsupported(),
);

// AFR 363 — Loyal Warhound
pub(in crate::card::sets) static LOYAL_WARHOUND_363: CardRecord = CardRecord::new(
    "Loyal Warhound",
    "bce58575-7607-49a4-a7c9-b23b715c3bf5",
    "Dmitry Burmak",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Dog"], 3, 1).with_abilities(&[
abilities::vigilance(),
AbilityDef::triggered_if("When this creature enters, if an opponent controls more lands than you, search your library for a basic Plains card, put it onto the battlefield tapped, then shuffle.", TriggerEventDef::zone_changed(ObjectPredicateDef::Source, None, Some(ZoneKind::Battlefield)), &TriggerConditionDef::ValueComparison(&ValueComparisonDef { left: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(ObjectPredicateDef::HasType(CardType::Land), &[ZoneKind::Battlefield], PlayerRelation::Opponent)), comparison: ComparisonDef::Greater, right: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(ObjectPredicateDef::HasType(CardType::Land), &[ZoneKind::Battlefield], PlayerRelation::You)) }), EffectDef::SearchZone { player: EffectRecipientDef::Controller, source: ZoneKind::Library, object: ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Land), ObjectPredicateDef::Supertype(CardSupertype::Basic), ObjectPredicateDef::Subtype(SubtypeDef::Literal("Plains"))]), minimum: 0, maximum: ValueDef::Constant(1), reveal: false, destination: ZoneKind::Battlefield, placement: ZonePlacement::Top, shuffle: true, enters_tapped: true, attachment: None, binding: None, then: None })
]),
);

// AFR 373 — Asmodeus the Archfiend
// Audit: unsupported — Its draw replacement needs a face-down exile pile tied to the later life-payment action.
pub(in crate::card::sets) static ASMODEUS_THE_ARCHFIEND_373: CardRecord = CardRecord::new(
    "Asmodeus the Archfiend",
    "1a0bbab6-b9ad-456d-ab4a-485dd8d89b35",
    "Aleksi Briclot",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &CLERIC_CLASS,
    &PALADIN_CLASS,
    &PORTABLE_HOLE,
    &YOU_HEAR_SOMETHING_ON_WATCH,
    &WIZARD_CLASS,
    &SHAMBLING_GHAST_119,
    &VAMPIRE_SPAWN,
    &WARLOCK_CLASS,
    &BARBARIAN_CLASS,
    &BATTLE_CRY_GOBLIN_132,
    &HOBGOBLIN_BANDIT_LORD_147,
    &PLUNDERING_BARBARIAN_158,
    &UNEXPECTED_WINDFALL_164,
    &CIRCLE_OF_DREAMS_DRUID_176,
    &DRUID_CLASS,
    &OWLBEAR,
    &RANGER_CLASS,
    &THE_TARRASQUE_207,
    &YOU_MEET_IN_A_TAVERN,
    &BARD_CLASS,
    &FIGHTER_CLASS,
    &MONK_CLASS,
    &ROGUE_CLASS,
    &SORCERER_CLASS,
    &HIVE_OF_THE_EYE_TYRANT_258,
    &OLD_GNAWBONE_296,
    &OSWALD_FIDDLEBENDER_304,
    &DELINA_WILD_MAGE_317,
    &XORN_322,
    &DEN_OF_THE_BUGBEAR_351,
    &TREASURE_VAULT_358,
    &LOYAL_WARHOUND_363,
    &ASMODEUS_THE_ARCHFIEND_373,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
