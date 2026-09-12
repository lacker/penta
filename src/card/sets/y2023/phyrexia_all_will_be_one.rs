//! Phyrexia: All Will Be One cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::Binding;
use crate::ParentBinding;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AppliedEffectDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseOneOfEachDef;
use crate::card::ComparisonDef;
use crate::card::CostDef;
use crate::card::CostQuantityDef;
use crate::card::CounterKind;
use crate::card::CreatedTokensDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::MoveObjectsDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::RandomizeObjectOrderDef;
use crate::card::ReplacementEffectDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::RevealObjectsDef;
use crate::card::SacrificedAmountDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::ValueComparisonDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "ONE",
    slug: "phyrexia-all-will-be-one",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// ONE 28 — Planar Disruption
pub(in crate::card::sets) static PLANAR_DISRUPTION: CardRecord = CardRecord::new(
    "Planar Disruption",
    "8ee69a1f-aeed-4eb4-8987-fa720fc99715",
    "Campbell White",
    // Two mana answers a creature, a mana rock, or a planeswalker, which is
    // what a Pacifism that reads wider is worth in a format full of both.
    CardRules::new_enchantment(mana_cost!("{1}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::aura_spell(
                "Enchant artifact, creature, or planeswalker",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Planeswalker),
                    ]),
                )],
            ),
            abilities::enchanted_permanent_subdued(),
        ]),
);

// ONE 80 — Annihilating Glare
pub(in crate::card::sets) static ANNIHILATING_GLARE: CardRecord = CardRecord::new(
    "Annihilating Glare",
    "be5d0b95-ec12-4e8e-99a0-7aca457f9107",
    "Konstantin Porubov",
    CardRules::new_sorcery(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_additional_cost(
        "As an additional cost to cast this spell, pay {4} or sacrifice an artifact or \
             creature.\nDestroy target creature or planeswalker.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::HasType(CardType::Planeswalker),
            ]),
        )],
        CostDef::choice(&[
            CostDef::pay_mana(mana_cost!("{4}")),
            CostDef::sacrifice(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::HasType(CardType::Creature),
                ]),
                CostQuantityDef::Fixed(1),
            ),
        ]),
        EffectDef::destroy_target(crate::TargetIndex::PRIMARY),
    )),
);

// ONE 102 — Offer Immortality
pub(in crate::card::sets) static OFFER_IMMORTALITY: CardRecord = CardRecord::new(
    "Offer Immortality",
    "b0aac10a-6d47-4a6c-8a10-2b7c06f3ff32",
    "A. M. Sartor",
    CardRules::new_instant(mana_cost!("{1}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gains deathtouch and indestructible until end \
         of turn. (Damage and effects that say \"destroy\" don't \
         destroy it.)",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::add_ability(&abilities::deathtouch()),
                AppliedEffectDef::add_ability(&abilities::indestructible()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// ONE 108 — Sheoldred's Edict
/// "Of their choice", which is what makes it an edict: the sacrifice is
/// theirs to make, so hexproof and protection never come into it.
const fn edict(text: &'static str, object: ObjectPredicateDef) -> AbilityDef {
    AbilityDef::spell(
        text,
        EffectDef::SacrificeOfChoice {
            player: EffectRecipientDef::Opponent,
            object,
            count: ValueDef::Constant(1),
            then: None,
            amount: SacrificedAmountDef::Power,
            otherwise: None,
            optional: false,
        },
    )
}

pub(in crate::card::sets) static SHEOLDRED_S_EDICT: CardRecord = CardRecord::new(
    "Sheoldred's Edict",
    "a9225cc3-90f0-448f-a8d9-7c6c2796d077",
    "Helge C. Balzer",
    // Two mana at instant speed for the one creature a protected threat
    // cannot dodge, as long as it is the only one they have.
    CardRules::new_instant(mana_cost!("{1}{B}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            edict(
                "Each opponent sacrifices a nontoken creature of their choice.",
                // Three edicts in one card, and the split is what makes it an answer
                // rather than a gamble: the mode that names tokens leaves the real
                // creature alone, and the mode that names nontokens cannot be paid with a
                // Servo.
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                ]),
            ),
            edict(
                "Each opponent sacrifices a creature token of their choice.",
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Token,
                ]),
            ),
            edict(
                "Each opponent sacrifices a planeswalker of their choice.",
                ObjectPredicateDef::HasType(CardType::Planeswalker),
            ),
        ],
    )),
);

// ONE 121 — Barbed Batterfist
pub(in crate::card::sets) static BARBED_BATTERFIST: CardRecord = CardRecord::new(
    "Barbed Batterfist",
    "de1d02d1-91dc-47d6-bdbe-87602428abfb",
    "Randy Gallegos",
    // A 3/1 for two that leaves the Equipment behind when it trades. The
    // toughness penalty is what pays for that: it makes the Rebel worse at
    // blocking than the 2/2 underneath it.
    CardRules::new_artifact(mana_cost!("{1}{R}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::for_mirrodin(),
            AbilityDef::static_ability(
                "Equipped creature gets +1/-1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(-1),
                    ),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{1}"))], "Equip {1}"),
        ]),
);

// ONE 123 — Blazing Crescendo
// Audit: unsupported — Needs an exile-play permission expiring at cleanup of its controller's next turn; the current turn-count duration also permits plays during the following opponent turn.
pub(in crate::card::sets) static BLAZING_CRESCENDO: CardRecord = CardRecord::new(
    "Blazing Crescendo",
    "d6bfc16a-2871-40a4-b279-636b80491a06",
    "Tiffany Turrill",
    CardRules::unsupported(),
);

// ONE 133 — Furnace Strider
pub(in crate::card::sets) static FURNACE_STRIDER: CardRecord = CardRecord::new(
    "Furnace Strider",
    "aa625ab0-1e79-4497-a5da-98fe1abfd024",
    "Denis Zhbankov",
    // Two free haste grants attached to a body that blocks well, which is
    // what makes five mana acceptable in a deck built to go wide.
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Phyrexian", "Beast"], 4, 5).with_abilities(&[
        AbilityDef::as_enters(
            "This creature enters with two oil counters on it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::named("oil"),
                    amount: 2,
                },
            ),
        ),
        AbilityDef::activated_with_targets(
            "Remove an oil counter from this creature: Target creature you control gains haste \
             until end of turn.",
            // The counter is the whole cost, so this is free twice and then
            // never again.
            &[CostDef::RemoveCountersFromSource {
                kind: CounterKind::named("oil"),
                amount: 1,
            }],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::haste()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// ONE 161 — Cankerbloom
pub(in crate::card::sets) static CANKERBLOOM: CardRecord = CardRecord::new(
    "Cankerbloom",
    "89b39293-6f57-4294-85fc-c718bdbb4d40",
    "Nicholas Gregory",
// A 3/2 for two that is also the artifact removal the deck was going to
    // have to find room for, which is the whole reason it is in a cube.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Phyrexian", "Fungus"], 3, 2).with_ability(
        AbilityDef::modal_activated(
            "{1}, Sacrifice this creature: Choose one —\n• Destroy target artifact.\n• Destroy \
             target enchantment.\n• Proliferate.",
            &[
                CostDef::Mana(mana_cost!("{1}")),
                CostDef::SacrificeSource,
            ],
            // Two of the three answer something and the third answers nothing, which is
            // the point: a mode that only needs a counter on the board is what keeps
            // the card from being dead against a deck with no artifacts.
            &[
                AbilityDef::destroy_target("Destroy target artifact.", &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Artifact),
                )][0]
),
                AbilityDef::destroy_target("Destroy target enchantment.", &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                )][0]
),
                AbilityDef::spell(
                    "Proliferate. (Choose any number of permanents and/or players, then give each another \
                     counter of each kind already there.)",
                    EffectDef::Proliferate,
                ),
            ],
            1,
            1,
            false,
        ),
    ),
);

// ONE 164 — Contagious Vorrac
pub(in crate::card::sets) static CONTAGIOUS_VORRAC: CardRecord = CardRecord::new(
    "Contagious Vorrac",
    "18af2c85-e58f-4043-99d3-e90121348aca",
    "Maxime Minard",
    // Never a blank: a land when the draw is short, and a counter on
    // whatever the oil deck already has going when it is not.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Phyrexian", "Boar", "Beast"], 3, 3)
        .with_ability(abilities::enters_trigger(
            "When this creature enters, look at the top four cards of your library. You may \
             reveal a land card from among them and put it into your hand. Put the rest on the \
             bottom of your library in a random order. If you didn't put a card into your hand \
             this way, proliferate. (Choose any number of permanents and/or players, then give \
             each another counter of each kind already there.)",
            EffectDef::Sequence(&[
                abilities::look_at_top_cards_reveal_choice_to_hand_rest_random_bottom(
                    ValueDef::Constant(4),
                    ObjectPredicateDef::HasType(CardType::Land),
                    0,
                    1,
                ),
                // "If you didn't put a card into your hand this way" is read
                // off what the choice above bound, so declining and finding
                // no land both reach the proliferate.
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                        left: ValueDef::BoundObjectCount(Binding!("top_card_chosen")),
                        comparison: ComparisonDef::LessOrEqual,
                        right: ValueDef::Constant(0),
                    }),
                    then: &EffectDef::Proliferate,
                },
            ]),
        )),
);

// ONE 196 — Atraxa, Grand Unifier
/// Ten cards face up, and one pick per card type from among them: an
/// artifact, a creature, an enchantment, an instant, a land, a planeswalker,
/// and a sorcery, each optional and each from what is left, so an artifact
/// creature taken as the artifact is no longer there to be the creature.
/// The printed reminder counts battle as an eighth type; nothing in this
/// engine is one, so the seven it has are the whole list. The rest go back
/// underneath in a random order, which is why the look is worth so much
/// less to the player who did it than the cards it kept.
const ATRAXA_CHOSEN: Binding = Binding!("atraxa_chosen");
const ATRAXA_REST: Binding = Binding!("atraxa_rest");
pub(in crate::card::sets) static ATRAXA_GRAND_UNIFIER: CardRecord = CardRecord::new(
    "Atraxa, Grand Unifier",
    "4a1f905f-1d55-4d02-9d24-e58070793d3f",
    "Marta Nael",
// Seven mana across four colours for a 7/7 that blocks everything, gains
    // the life back, and refills the hand on the way in.
    CardRules::new_creature(mana_cost!("{3}{G}{W}{U}{B}"), &["Phyrexian", "Angel"], 7, 7)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            abilities::vigilance(),
            abilities::deathtouch(),
            abilities::lifelink(),
            abilities::enters_trigger(
                "When this creature enters, reveal the top ten cards of your library. For each card \
                 type, you may put a card of that type from among the revealed cards into your hand. Put \
                 the rest on the bottom of your library in a random order.",
                abilities::bind_top_cards_then(
                    PlayerRefDef::EffectController,
                    ValueDef::Constant(10),
                    &EffectDef::Sequence(&[
                        EffectDef::RevealObjects(RevealObjectsDef {
                            input: ObjectSetDef::Binding(ParentBinding),
                            then: &EffectDef::None,
                        }),
                        EffectDef::ChooseOneOfEach(ChooseOneOfEachDef {
                                actor: PlayerRefDef::EffectController,
                                input: ObjectSetDef::Binding(ParentBinding),
                                predicates: &[
                                        ObjectPredicateDef::HasType(CardType::Artifact),
                                        ObjectPredicateDef::HasType(CardType::Creature),
                                        ObjectPredicateDef::HasType(CardType::Enchantment),
                                        ObjectPredicateDef::HasType(CardType::Instant),
                                        ObjectPredicateDef::HasType(CardType::Land),
                                        ObjectPredicateDef::HasType(CardType::Planeswalker),
                                        ObjectPredicateDef::HasType(CardType::Sorcery),
                                    ],
                                chosen: ATRAXA_CHOSEN,
                                remainder: ATRAXA_REST,
                                visibility: ChoiceVisibilityDef::Public,
                                then: &EffectDef::Sequence(&[
                                        EffectDef::MoveObjects(MoveObjectsDef {
                                            input: ObjectSetDef::Binding(ATRAXA_CHOSEN),
                                            from: Some(ZoneKind::Library),
                                            zone: ZoneKind::Hand,
                                            placement: ZonePlacement::Top,
                                            moved: None,
                                            then: &EffectDef::None,
                                        }),
                                            EffectDef::RandomizeObjectOrder(
                                                RandomizeObjectOrderDef {
                                                    input: ObjectSetDef::Binding(ATRAXA_REST),
                                                    randomized: ParentBinding,
                                                    then: &EffectDef::MoveObjects(
                                                        MoveObjectsDef {
                                                            input: ObjectSetDef::Binding(
                                                                ParentBinding,
                                                            ),
                                                            from: Some(ZoneKind::Library),
                                                            zone: ZoneKind::Library,
                                                            placement: ZonePlacement::Bottom,
                                                            moved: None,
                                                            then: &EffectDef::None,
                                                        },
                                                    ),
                                                },
                                            )
                                    ]),
                        }),
                    ]),
                ),
            ),
        ]),
);

// ONE 213 — Ovika, Enigma Goliath
pub(in crate::card::sets) static OVIKA_ENIGMA_GOLIATH: CardRecord = CardRecord::new(
    "Ovika, Enigma Goliath",
    "b298cf34-7aa5-4f97-a86c-7f28d2113b87",
    "Antonio José Manzanedo",
    CardRules::new_creature(mana_cost!("{5}{U}{R}"), &["Phyrexian", "Nightmare"], 6, 6)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            abilities::ward(
                &[CostDef::Mana(mana_cost!("{3}")), CostDef::PayLife(3)],
                "Ward—{3}, Pay 3 life.",
            ),
            AbilityDef::triggered(
                "Whenever you cast a noncreature spell, create X 1/1 red \
                 Phyrexian Goblin creature tokens, where X is the mana value \
                 of that spell. They gain haste until end of turn.",
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                EffectDef::create_creature_token(&["Phyrexian", "Goblin"], &[ManaColor::Red], 1, 1)
                    .with_count(ValueDef::ObjectManaValue(ObjectRefDef::TriggeringObject))
                    .with_created_tokens(CreatedTokensDef {
                        binding: crate::Binding!("goblins"),
                        then: &EffectDef::Apply {
                            recipient: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                crate::Binding!("goblins"),
                            )),
                            effect: AppliedEffectDef::add_ability(&abilities::haste()),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                    }),
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &PLANAR_DISRUPTION,
    &ANNIHILATING_GLARE,
    &OFFER_IMMORTALITY,
    &SHEOLDRED_S_EDICT,
    &BARBED_BATTERFIST,
    &BLAZING_CRESCENDO,
    &FURNACE_STRIDER,
    &CANKERBLOOM,
    &CONTAGIOUS_VORRAC,
    &ATRAXA_GRAND_UNIFIER,
    &OVIKA_ENIGMA_GOLIATH,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
