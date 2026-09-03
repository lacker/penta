//! Commander Legends: Battle for Baldur's Gate cards cataloged for the
//! Vintage Cube.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AlternateSpellKind, AppliedEffectDef, AppliedRuleDef, CardArt, CardComposition,
    CardEffectStatus, CardNameDef, CardPart, CardRules, CardSet, CardStructure, CardSupertype,
    CardType, ChoiceVisibilityDef, ChooseDef, ComparisonDef, CounterKind, DeckConstructionDef,
    EffectDef, EffectRecipientDef, KeywordAbility, ManaColor, ObjectChoiceBindingDef,
    ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef, ObjectSetFilterDef,
    PlayOptionDef, PlayerRefDef, PlayerRelation, ResolvedEffectDurationDef, SacrificedAmountDef,
    SpellCastQueryDef, SpellForm, SpellResolutionDestinationDef, TokenCharacteristics,
    TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueComparisonDef, ValueDef, ZoneKind,
    ZonePlacement, abilities,
};
use crate::ids::{CardPartId, PlayOptionId};
use crate::{TargetIndex, mana_cost};

// CLB 8 — Banishment
pub(in crate::card::sets) static BANISHMENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a71caadb-31ab-4b7f-b304-e7d3e8f9d132"),
    "Banishment",
    CardArt::new("a71caadb-31ab-4b7f-b304-e7d3e8f9d132", "Darek Zabrocki"),
    CardSet::CommanderLegendsBattleForBaldursGate,
    CardRules::new_enchantment(mana_cost!("{3}{W}")).with_abilities(&[
        abilities::flash(),
        abilities::enters_trigger_with_targets(
            "When this enchantment enters, exile target nonland permanent an opponent controls and all other nonland permanents your opponents control with the same name until this enchantment leaves the battlefield.",
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::Opponent),
                owner: None,
            })],
            abilities::exile_until_source_leaves(EffectRecipientDef::objects(
                ObjectSetDef::Union(&[
                    ObjectSetDef::One(ObjectRefDef::Target(TargetIndex::PRIMARY)),
                    ObjectSetDef::Matching {
                        objects: &ObjectSetDef::Query(ObjectQueryDef::matching(
                            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Opponent,
                        )),
                        object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::NameEquals(
                            CardNameDef::NameOf(ObjectRefDef::Target(TargetIndex::PRIMARY)),
                        )),
                    },
                ]),
            )),
        ),
    ]),
);

// CLB 11 — Blessed Hippogriff
const fn blessed_hippogriff_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{3}{W}"), &const { ["Hippogriff"] }, 2, 3).with_abilities(
        &const {
            [
                abilities::flying(),
                AbilityDef::triggered_with_targets(
                    "Whenever this creature attacks, target attacking creature without flying \
                     gains flying until end of turn.",
                    TriggerEventDef::attacks(ObjectPredicateDef::Source),
                    // "Without flying" excludes the Hippogriff itself, which
                    // already has it, so the trigger only ever helps another
                    // attacker through.
                    &const {
                        [AbilityTargetDef::exactly_one_permanent(
                            ObjectPredicateDef::All(
                                &const {
                                    [
                                        ObjectPredicateDef::HasType(CardType::Creature),
                                        ObjectPredicateDef::Attacking,
                                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasKeyword(
                                            KeywordAbility::Flying,
                                        )),
                                    ]
                                },
                            ),
                        )]
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::add_ability(&const { abilities::flying() }),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ),
            ]
        },
    )
}

fn blessed_hippogriff_composition() -> CardComposition {
    let hippogriff = blessed_hippogriff_rules();
    let blessing = const {
        CardRules::new_instant(mana_cost!("{W}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell_with_targets(
                    "Target creature gains indestructible until end of turn.",
                    &const {
                        [AbilityTargetDef::exactly_one_permanent(
                            ObjectPredicateDef::HasType(CardType::Creature),
                        )]
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::add_ability(
                            &const { abilities::indestructible() },
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            )
    };
    CardComposition {
        parts: vec![
            CardPart::new(CardPartId::PRIMARY, "Blessed Hippogriff", hippogriff),
            CardPart::new(CardPartId(1), "Tyr's Blessing", blessing),
        ],
        structure: CardStructure::AlternateSpell {
            main: CardPartId::PRIMARY,
            alternate: CardPartId(1),
            kind: AlternateSpellKind::Adventure,
        },
        play_options: vec![
            PlayOptionDef::cast(
                PlayOptionId::DEFAULT,
                "Blessed Hippogriff",
                SpellForm::Part(CardPartId::PRIMARY),
                hippogriff
                    .mana_cost()
                    .expect("the Hippogriff has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
            PlayOptionDef::cast(
                PlayOptionId(1),
                "Tyr's Blessing",
                SpellForm::Part(CardPartId(1)),
                blessing
                    .mana_cost()
                    .expect("Tyr's Blessing has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
        ],
    }
    .with_derived_spell_targets()
}

pub(in crate::card::sets) static BLESSED_HIPPOGRIFF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b4590e53-ca8d-4896-a8cf-6af1e4bc456f"),
    "Blessed Hippogriff",
    CardArt::new("b4590e53-ca8d-4896-a8cf-6af1e4bc456f", "Leanna Crossan"),
    CardSet::CommanderLegendsBattleForBaldursGate,
    // A one-mana combat trick early and a flier that carries the team over
    // blockers later: the Adventure is why the body costs four.
    blessed_hippogriff_rules(),
)
.with_composition(blessed_hippogriff_composition);

// CLB 22 — Greatsword of Tyr
pub(in crate::card::sets) static GREATSWORD_OF_TYR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("50088a60-642b-47ed-a289-ef0b617b688f"),
    "Greatsword of Tyr",
    CardArt::new("50088a60-642b-47ed-a289-ef0b617b688f", "Titus Lunter"),
    CardSet::CommanderLegendsBattleForBaldursGate,
    // One white to move it and a counter every swing, so the Equipment is
    // the threat and whichever creature carries it is interchangeable.
    CardRules::new_artifact(mana_cost!("{1}{W}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            AbilityDef::triggered_with_targets(
                "Whenever equipped creature attacks, put a +1/+1 counter on it and tap up to one \
                 target creature defending player controls.",
                TriggerEventDef::attacks(ObjectPredicateDef::AttachedToSource),
                // "Up to one" so the trigger still puts the counter on when
                // the defender has nothing worth tapping, or nothing at all.
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::DefendingPlayer),
                        owner: None,
                    },
                    1,
                )],
                EffectDef::Sequence(&[
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::AttachedPermanent,
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::Tap {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                ]),
            ),
            abilities::equip(
                &[AbilityCostDef::Mana(mana_cost!("{W}"))],
                "Equip {W} ({W}: Attach to target creature you control. Equip only as a \
                 sorcery.)",
            ),
        ]),
);

// CLB 99 — Sword Coast Serpent
const fn sword_coast_serpent_rules() -> CardRules {
    CardRules::new_creature(
        mana_cost!("{5}{U}{U}"),
        &const { ["Serpent", "Dragon"] },
        6,
        6,
    )
    .with_ability(AbilityDef::static_ability(
        "This creature can't be blocked as long as you've cast a noncreature spell this turn.",
        EffectDef::IfCondition {
            // Counted as they are cast rather than read off the stack:
            // the spell that switched this on has usually resolved.
            condition: &const {
                TriggerConditionDef::ValueComparison(
                    &const {
                        ValueComparisonDef {
                            left: ValueDef::CountSpellsCastThisTurn(
                                &const {
                                    SpellCastQueryDef {
                                        player: PlayerRelation::You,
                                        spell: ObjectPredicateDef::NoncreatureSpell,
                                    }
                                },
                            ),
                            comparison: ComparisonDef::GreaterOrEqual,
                            right: ValueDef::Constant(1),
                        }
                    },
                )
            },
            then: &const {
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                }
            },
        },
    ))
}

fn sword_coast_serpent_composition() -> CardComposition {
    let serpent = sword_coast_serpent_rules();
    let wave = const {
        CardRules::new_instant(mana_cost!("{1}{U}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell_with_targets(
                    "Return target creature to its owner's hand.",
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
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            )
    };
    CardComposition {
        parts: vec![
            CardPart::new(CardPartId::PRIMARY, "Sword Coast Serpent", serpent),
            CardPart::new(CardPartId(1), "Capsizing Wave", wave),
        ],
        structure: CardStructure::AlternateSpell {
            main: CardPartId::PRIMARY,
            alternate: CardPartId(1),
            kind: AlternateSpellKind::Adventure,
        },
        play_options: vec![
            PlayOptionDef::cast(
                PlayOptionId::DEFAULT,
                "Sword Coast Serpent",
                SpellForm::Part(CardPartId::PRIMARY),
                serpent
                    .mana_cost()
                    .expect("the Serpent has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
            PlayOptionDef::cast(
                PlayOptionId(1),
                "Capsizing Wave",
                SpellForm::Part(CardPartId(1)),
                wave.mana_cost()
                    .expect("Capsizing Wave has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
        ],
    }
    .with_derived_spell_targets()
}

pub(in crate::card::sets) static SWORD_COAST_SERPENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0bbfb7ae-9a32-428d-903c-99d0d8669b8d"),
    "Sword Coast Serpent",
    CardArt::new("0bbfb7ae-9a32-428d-903c-99d0d8669b8d", "Caio Monteiro"),
    CardSet::CommanderLegendsBattleForBaldursGate,
    // Seven mana for a body that only connects in a deck already casting
    // cheap spells -- which is the deck the Adventure half is for.
    sword_coast_serpent_rules(),
)
.with_composition(sword_coast_serpent_composition);

// CLB 106 — Young Blue Dragon
const fn young_blue_dragon_rules() -> CardRules {
    CardRules::new_creature(mana_cost!("{4}{U}"), &const { ["Dragon"] }, 3, 3)
        .with_ability(abilities::flying())
}

fn young_blue_dragon_composition() -> CardComposition {
    let dragon = young_blue_dragon_rules();
    let augury = const {
        CardRules::new_sorcery(mana_cost!("{1}{U}"))
            .with_subtypes(&const { ["Adventure"] })
            .with_ability(
                AbilityDef::spell(
                    "Scry 1, then draw a card.",
                    // Scry before the draw, unlike Serum Visions: this one
                    // shapes the card it is about to give you.
                    EffectDef::Sequence(
                        &const {
                            [
                                abilities::scry(ValueDef::Constant(1)),
                                EffectDef::DrawCards {
                                    recipient: EffectRecipientDef::Controller,
                                    amount: ValueDef::Constant(1),
                                },
                            ]
                        },
                    ),
                )
                .with_resolution_destination(SpellResolutionDestinationDef::ExileOnAdventure),
            )
    };
    CardComposition {
        parts: vec![
            CardPart::new(CardPartId::PRIMARY, "Young Blue Dragon", dragon),
            CardPart::new(CardPartId(1), "Sand Augury", augury),
        ],
        structure: CardStructure::AlternateSpell {
            main: CardPartId::PRIMARY,
            alternate: CardPartId(1),
            kind: AlternateSpellKind::Adventure,
        },
        play_options: vec![
            PlayOptionDef::cast(
                PlayOptionId::DEFAULT,
                "Young Blue Dragon",
                SpellForm::Part(CardPartId::PRIMARY),
                dragon
                    .mana_cost()
                    .expect("the Dragon has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
            PlayOptionDef::cast(
                PlayOptionId(1),
                "Sand Augury",
                SpellForm::Part(CardPartId(1)),
                augury
                    .mana_cost()
                    .expect("Sand Augury has a printed mana cost"),
                CardEffectStatus::Implemented,
            ),
        ],
    }
    .with_derived_spell_targets()
}

pub(in crate::card::sets) static YOUNG_BLUE_DRAGON: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("56b0f66b-dca9-4a01-9394-20a513c2b225"),
    "Young Blue Dragon",
    CardArt::new("56b0f66b-dca9-4a01-9394-20a513c2b225", "Tuan Duong Chu"),
    CardSet::CommanderLegendsBattleForBaldursGate,
    // A cantrip early and a flier later, which is the deal the whole
    // Adventure cycle offers: two cards' worth of turns from one card.
    young_blue_dragon_rules(),
)
.with_composition(young_blue_dragon_composition);

// CLB 113 — Arms of Hadar
pub(in crate::card::sets) static ARMS_OF_HADAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("db1fd431-8f6d-4ca5-bc0c-53881c500da1"),
    "Arms of Hadar",
    CardArt::new("db1fd431-8f6d-4ca5-bc0c-53881c500da1", "Mirko Failoni"),
    CardSet::CommanderLegendsBattleForBaldursGate,
    // A one-sided sweeper at sorcery speed, and it names a player rather
    // than the creatures, so a board built after it resolves is untouched.
    CardRules::new_sorcery(mana_cost!("{3}{B}")).with_ability(AbilityDef::spell_with_targets(
        "Creatures target player controls get -2/-2 until end of turn.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Player(PlayerRelation::Any),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::objects_controlled_by_target(
                ObjectPredicateDef::HasType(CardType::Creature),
                TargetIndex::PRIMARY,
            ),
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(-2),
                ValueDef::Constant(-2),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )),
);

// CLB 119 — Cast Down
pub(in crate::card::sets) static CAST_DOWN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("116ce944-6871-4f51-a889-d9c4a5d7cff2"),
    "Cast Down",
    CardArt::new("aba79021-39af-4e74-beb5-f2f508c865b2", "Tyler Walpole"),
    CardSet::CommanderLegendsBattleForBaldursGate,
    // Two mana for unconditional removal, priced by the one exception it
    // makes -- which is exactly the thing the opponent built around.
    CardRules::new_instant(mana_cost!("{1}{B}")).with_ability(AbilityDef::destroy_target(
        "Destroy target nonlegendary creature.",
        &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(CardSupertype::Legendary)),
        ])),
        true,
    )),
);

// CLB 130 — Guildsworn Prowler
pub(in crate::card::sets) static GUILDSWORN_PROWLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d7efb10f-c760-431c-8ac6-904965d850dc"),
    "Guildsworn Prowler",
    CardArt::new("d7efb10f-c760-431c-8ac6-904965d850dc", "Fariba Khamseh"),
    CardSet::CommanderLegendsBattleForBaldursGate,
    // Deathtouch makes attacking into it a bad trade and blocking with it a
    // good one, and the card is the reward for choosing the first.
    CardRules::new_creature(
        mana_cost!("{1}{B}"),
        &["Tiefling", "Rogue", "Assassin"],
        2,
        1,
    )
    .with_abilities(&[
        abilities::deathtouch(),
        AbilityDef::triggered_if(
            "When this creature dies, if it wasn't blocking, draw a card.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            ),
            // Read off the creature as it left, so trading it away on
            // your own attack draws and chump-blocking with it does not.
            &const {
                TriggerConditionDef::SourceMatches {
                    object: ObjectPredicateDef::Not(&ObjectPredicateDef::Blocking),
                }
            },
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// CLB 180 — Gut, True Soul Zealot
pub(in crate::card::sets) static GUT_TRUE_SOUL_ZEALOT: CardRecord = CardRecord::new_with_legacy_id(
    2211,
    "Gut, True Soul Zealot",
    CardArt::new("3d8ca18d-9099-4f1e-95c1-f04da58a26bd", "Wayne Reynolds"),
    CardSet::CommanderLegendsBattleForBaldursGate,
    // Every spent artifact and every creature that has done its work turns
    // into four attacking power that two blockers cannot answer alone.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin", "Shaman"], 2, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever you attack, you may sacrifice another creature or an artifact. If you do, create a 4/1 black Skeleton creature token with menace that's tapped and attacking.",
                // "Whenever you attack" is one or more creatures you control attacking,
                // counted once for the declaration rather than once per attacker.
                TriggerEventDef::attack_declared(
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    1,
                    None,
                ),
                EffectDef::SacrificeOfChoice {
                    count: ValueDef::Constant(1),
                    player: EffectRecipientDef::Controller,
                    // "Another creature or an artifact." Gut is neither an artifact nor another
                    // creature, so the exclusion covers both halves without saying so twice.
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Artifact),
                        ]),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    // The token arrives already attacking, which is the whole point: it was
                    // never declared, so nothing that watches a declaration sees it, and it
                    // still connects this combat.
                    then: Some(&EffectDef::create_creature_token(&["Skeleton"], &[ManaColor::Black], 4, 1)
                            .with_abilities(&[abilities::menace()])
                            .with_art(CardArt::new(
                                "cf4c245f-af2f-46a7-81f3-670a04940901",
                                "David Astruga",
                            ))
                            .entering_tapped()
                            .entering_attacking()),
                    amount: SacrificedAmountDef::Power,
                    otherwise: None,
                    optional: true,
                },
            ),
            AbilityDef::deck_construction(
                "Choose a Background (You can have a Background as a second commander.)",
                DeckConstructionDef::ChooseABackground,
                "The parenthesis is the whole sentence: it is a deck-construction \
                 permission, checked where a Commander list is assembled and silent \
                 once the game starts.",
            ),
        ]),
);

// CLB 263 — You Meet in a Tavern
pub(in crate::card::sets) static YOU_MEET_IN_A_TAVERN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("593aa59a-4025-4df8-9f27-188fc7712fde"),
    "You Meet in a Tavern",
    CardArt::new("9fddbd7a-799c-4432-810c-d839c5c354b9", "Zoltan Boros"),
    CardSet::CommanderLegendsBattleForBaldursGate,
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

// CLB 285 — Minsc & Boo, Timeless Heroes
pub(in crate::card::sets) static MINSC_BOO_TIMELESS_HEROES: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("928036c9-11b8-493e-b9f2-8fbd3487cd19"),
    "Minsc & Boo, Timeless Heroes",
    CardArt::new("928036c9-11b8-493e-b9f2-8fbd3487cd19", "Andreas Zafiratos"),
    CardSet::CommanderLegendsBattleForBaldursGate,
    CardRules::new_planeswalker(mana_cost!("{2}{R}{G}"), &["Minsc"], 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered(
                "When Minsc & Boo enters and at the beginning of your upkeep, you may create Boo, a legendary 1/1 red Hamster creature token with trample and haste.",
                // One printed sentence with two ways in, so it is one ability
                // watching both rather than two abilities.
                TriggerEventDef::AnyOf(&[
                    TriggerEventDef::zone_changed(
                        ObjectPredicateDef::Source,
                        None,
                        Some(ZoneKind::Battlefield),
                    ),
                    TriggerEventDef::StepBegins {
                        step: TurnStepDef::Upkeep,
                        player: PlayerRelation::You,
                    },
                ]),
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    // Boo is a particular hamster rather than a kind of one,
                    // which is why the legend rule keeps there being only the
                    // one however many upkeeps go by.
                    effect: &EffectDef::create_token(
                        TokenCharacteristics::creature(&["Hamster"], &[ManaColor::Red], 1, 1)
                            .with_name("Boo")
                            .with_supertype(CardSupertype::Legendary)
                            .with_abilities(&[abilities::trample(), abilities::haste()]),
                    ),
                },
            ),
            AbilityDef::activated_with_targets(
                "+1: Put three +1/+1 counters on up to one target creature with trample or haste.",
                &[AbilityCostDef::Loyalty(1)],
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasKeyword(KeywordAbility::Trample),
                                ObjectPredicateDef::HasKeyword(KeywordAbility::Haste),
                            ]),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    1,
                )],
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(3),
                },
            ),
            AbilityDef::activated(
                "\u{2212}2: Sacrifice a creature.",
                &[AbilityCostDef::Loyalty(-2)],
                EffectDef::SacrificeOfChoice {
                    player: EffectRecipientDef::Controller,
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    count: ValueDef::Constant(1),
                    // The payload is the reflexive half below, which needs to
                    // pick its target after the sacrifice rather than before.
                    then: None,
                    amount: SacrificedAmountDef::Power,
                    otherwise: None,
                    optional: false,
                },
            ),
            AbilityDef::triggered_with_targets(
                "When you do, Minsc & Boo deals X damage to any target, where X is that creature's power. If the sacrificed creature was a Hamster, draw X cards.",
                TriggerEventDef::SacrificePerformed(ObjectPredicateDef::Source),
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::AnyTarget,
                )],
                EffectDef::Sequence(&[
                    EffectDef::DealDamage {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::TriggerEventAmount,
                    },
                    // Throwing Boo is the payoff the card is built around, so
                    // the draw asks what was sacrificed rather than what is
                    // still on the battlefield.
                    EffectDef::IfCondition {
                        condition: &TriggerConditionDef::SacrificedObjectMatches(
                            ObjectPredicateDef::Subtype("Hamster"),
                        ),
                        then: &EffectDef::DrawCards {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::TriggerEventAmount,
                        },
                    },
                ]),
            ),
            AbilityDef::deck_construction(
                "Minsc & Boo, Timeless Heroes can be your commander.",
                DeckConstructionDef::MayBeCommander,
                "A planeswalker rather than a legendary creature, so the deck needs the printed permission.",
            ),
        ]),
);

// CLB 560 — Displacer Kitten
pub(in crate::card::sets) static DISPLACER_KITTEN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9a53e8fc-bfd2-4866-a61c-f3204b0a98bf"),
    "Displacer Kitten",
    CardArt::new("9a53e8fc-bfd2-4866-a61c-f3204b0a98bf", "Campbell White"),
    CardSet::CommanderLegendsBattleForBaldursGate,
    // Four mana for a 2/2 that does nothing on its own and everything in a
    // deck built to cast noncreature spells: every one of them is another
    // enter trigger off whatever is already on the battlefield.
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Cat", "Beast"], 2, 2).with_ability(
        AbilityDef::triggered_with_targets(
            "Avoidance — Whenever you cast a noncreature spell, exile up to one target nonland \
             permanent you control, then return that card to the battlefield under its owner's \
             control.",
            // A noncreature spell you cast. What it does is no part of the condition:
            // the Kitten reads the type line and nothing else.
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            // "Up to one target nonland permanent you control": the trigger goes on the
            // stack whether or not there is anything worth blinking.
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
                1,
            )],
            // Exiling links the permanent to the Kitten, which is what lets the return
            // name the card the exile just made.
            EffectDef::Sequence(&[
                EffectDef::ExileLinkedToSource {
                    until_source_leaves: false,
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    face_down: false,
                    then: None,
                },
                EffectDef::ReturnLinkedExiles {
                    object: ObjectPredicateDef::Any,
                    counters: None,
                    zone: ZoneKind::Battlefield,
                    grant: None,
                    controller: None,
                    transformed: false,
                },
            ]),
        ),
    ),
);

// CLB 630 — Delayed Blast Fireball
pub(in crate::card::sets) static DELAYED_BLAST_FIREBALL: CardRecord =
    CardRecord::new_with_legacy_id(
        2299,
        "Delayed Blast Fireball",
        CardArt::new("400c76c6-f677-4e7e-87ad-2e526d4b498a", "Andreas Zafiratos"),
        CardSet::CommanderLegendsBattleForBaldursGate,
        // A one-sided sweeper that costs a turn of setup, which is the trade the
        // cube's aggressive decks are least able to make and the slow ones most.
        CardRules::new_instant(mana_cost!("{1}{R}{R}")).with_abilities(&[
            AbilityDef::spell(
                "Delayed Blast Fireball deals 2 damage to each opponent and each creature they \
             control. If this spell was cast from exile, it deals 5 damage to each opponent and \
             each creature they control instead.",
                EffectDef::IfElseCondition {
                    condition: &TriggerConditionDef::SourceCastFrom(ZoneKind::Exile),
                    then: &EffectDef::DealDamage {
                        recipient: EffectRecipientDef::EachOpponentAndTheirCreatures,
                        amount: ValueDef::Constant(5),
                    },
                    // Two damage as the baseline and five when it was foretold, which is the
                    // whole of the card: the two mana spent a turn earlier buy three damage and
                    // one mana off the price.
                    otherwise: &EffectDef::DealDamage {
                        recipient: EffectRecipientDef::EachOpponentAndTheirCreatures,
                        amount: ValueDef::Constant(2),
                    },
                },
            ),
            abilities::foretell(mana_cost!("{4}{R}{R}")),
        ]),
    );

// CLB 748 — Dauthi Horror
pub(in crate::card::sets) static DAUTHI_HORROR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c5a8bb3a-3a84-442f-8e31-8af2f04408ab"),
    "Dauthi Horror",
    CardArt::new("7c41afe6-7eed-4cf5-9bbb-ccc9f82cb4fa", "Jeff Laubenstein"),
    CardSet::CommanderLegendsBattleForBaldursGate,
    // Shadow already stops white creatures blocking it, so the second
    // clause only matters against a white creature that also has shadow.
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Dauthi", "Horror"], 2, 1).with_abilities(&[
        abilities::shadow(),
        AbilityDef::static_ability(
            "This creature can't be blocked by white creatures.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::cannot_be_blocked_by(
                    ObjectPredicateDef::Color(ManaColor::White),
                )),
            },
        ),
    ]),
);

// CLB 897 — Izzet Boilerworks
pub(in crate::card::sets) static IZZET_BOILERWORKS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("666f455e-3a3d-475d-b67a-a1fdd74820eb"),
    "Izzet Boilerworks",
    CardArt::new("c86e42c6-342b-443f-9b99-a68cf536ff45", "John Avon"),
    CardSet::CommanderLegendsBattleForBaldursGate,
    // The last of the ten karoos; only the two colours below are its own.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::karoo_bounce(),
        AbilityDef::activated_mana(
            "{T}: Add {U}{R}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one_of_each(
                ManaColor::Blue,
                ManaColor::Red,
            )),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &BANISHMENT,
    &BLESSED_HIPPOGRIFF,
    &GREATSWORD_OF_TYR,
    &SWORD_COAST_SERPENT,
    &YOUNG_BLUE_DRAGON,
    &ARMS_OF_HADAR,
    &CAST_DOWN,
    &GUILDSWORN_PROWLER,
    &GUT_TRUE_SOUL_ZEALOT,
    &YOU_MEET_IN_A_TAVERN,
    &MINSC_BOO_TIMELESS_HEROES,
    &DISPLACER_KITTEN,
    &DELAYED_BLAST_FIREBALL,
    &DAUTHI_HORROR,
    &IZZET_BOILERWORKS,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
