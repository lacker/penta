//! Phyrexia: All Will Be One cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::Binding;
use crate::ParentBinding;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::ActivationTimingDef;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CardTypeSet;
use crate::card::CharacteristicOperationDef;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseOneOfEachDef;
use crate::card::ColorSet;
use crate::card::ComparisonDef;
use crate::card::CopyExceptionsDef;
use crate::card::CostDef;
use crate::card::CostQuantityDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::CreatedTokensDef;
use crate::card::DiscardSelectionDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::MoveObjectsDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::RandomizeObjectOrderDef;
use crate::card::ReplacementEffectDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::RevealObjectsDef;
use crate::card::SacrificedAmountDef;
use crate::card::SetOperationDef;
use crate::card::TapEventMatcherDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
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

// ONE 26 — Ossification
// Audit: unsupported — Needs exile-until-source-leaves with an immediate return when the duration
// ends, not a counterable leaves trigger.
pub(in crate::card::sets) static OSSIFICATION_26: CardRecord = CardRecord::new(
    "Ossification",
    "0da03224-c1af-438f-96c2-b0e41e1070b7",
    "Nino Vecia",
    CardRules::unsupported(),
);

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

// ONE 47 — Encroaching Mycosynth
// Audit: unsupported — External static type changes are evaluated for battlefield permanents. Nonbattlefield card and spell types read intrinsic self-characteristic clauses, but do not visit external CardTypes effects, so its permanent-card and permanent-spell clauses would be omitted.
pub(in crate::card::sets) static ENCROACHING_MYCOSYNTH_47: CardRecord = CardRecord::new(
    "Encroaching Mycosynth",
    "65a2fcc9-2317-48a1-a5eb-234fb3300364",
    "Martin de Diego Sádaba",
    crate::card::CardRules::unsupported(),
);

// ONE 64 — Minor Misstep
pub(in crate::card::sets) static MINOR_MISSTEP_64: CardRecord = CardRecord::new(
    "Minor Misstep",
    "360ca37b-5bbd-4923-a493-7674786a36af",
    "Lorenzo Mastroianni",
    CardRules::new_instant(mana_cost!("{U}")).with_ability(AbilityDef::counter_target(
        "Counter target spell with mana value 1 or less.",
        &AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::Spell,
                ObjectPredicateDef::ManaValueAtMost(1),
            ]),
            zones: &[ZoneKind::Stack],
            controller: None,
            owner: None,
        }),
    )),
);

// ONE 75 — Unctus, Grand Metatect
pub(in crate::card::sets) static UNCTUS_GRAND_METATECT_75: CardRecord = CardRecord::new(
    "Unctus, Grand Metatect",
    "164b07e6-48ba-4789-bd8f-7cada1fec8a9",
    "Andrew Mar",
    CardRules::new_artifact_creature(mana_cost!("{1}{U}{U}"), &["Phyrexian", "Vedalken"], 2, 4).with_supertype(CardSupertype::Legendary).with_abilities(&[
AbilityDef::static_ability("Other blue creatures you control have \"Whenever this creature becomes tapped, draw a card, then discard a card.\"", EffectDef::StaticApply { recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::Color(ManaColor::Blue), ObjectPredicateDef::Not(&ObjectPredicateDef::Source)]), &[ZoneKind::Battlefield], PlayerRelation::You), effect: AppliedEffectDef::add_ability(&AbilityDef::triggered("Whenever this creature becomes tapped, draw a card, then discard a card.", TriggerEventDef::Tapped(TapEventMatcherDef::any(ObjectPredicateDef::Source)), EffectDef::Sequence(&[abilities::draw_cards(ValueDef::Constant(1)), EffectDef::Discard { recipient: EffectRecipientDef::Controller, amount: ValueDef::Constant(1), selection: DiscardSelectionDef::RecipientChooses, then: None }]))) }),
AbilityDef::static_ability("Other artifact creatures you control get +1/+1.", EffectDef::StaticApply { recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::HasType(CardType::Artifact), ObjectPredicateDef::Not(&ObjectPredicateDef::Source)]), &[ZoneKind::Battlefield], PlayerRelation::You), effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)) }),
AbilityDef::activated_with_targets("{U/P}: Until end of turn, target creature you control becomes a blue artifact in addition to its other colors and types. Activate only as a sorcery. ({U/P} can be paid with either {U} or 2 life.)", &[CostDef::Mana(mana_cost!("{U/P}"))], &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object { object: ObjectPredicateDef::HasType(CardType::Creature), zones: &[ZoneKind::Battlefield], controller: Some(PlayerRelation::You), owner: None })], EffectDef::Apply { recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY), effect: AppliedEffectDef::Composite(&[AppliedEffectDef::add_colors(ColorSet::from_colors(&[ManaColor::Blue])), AppliedEffectDef::Characteristic(CharacteristicOperationDef::CardTypes(SetOperationDef::Add(CardTypeSet::single(CardType::Artifact))))]), duration: ResolvedEffectDurationDef::UntilEndOfTurn }).with_activation_timing(ActivationTimingDef::SorcerySpeed)
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

// ONE 172 — Infectious Bite
pub(in crate::card::sets) static INFECTIOUS_BITE_172: CardRecord = CardRecord::new(
    "Infectious Bite",
    "83dfb2a5-cd5c-46c6-9bb8-7c5d00f3e003",
    "Campbell White",
    CardRules::new_instant(mana_cost!("{1}{G}")).with_ability(AbilityDef::spell_with_targets(
        "Target creature you control deals damage equal to its power to target creature you don't control. Each opponent gets a poison counter.",
        &[
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            }),
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::NotYou),
                owner: None,
            }),
        ],
        EffectDef::Sequence(&[
            EffectDef::damage_from(
                crate::card::ObjectRefDef::Target(TargetIndex::PRIMARY),
                EffectRecipientDef::Target(TargetIndex(1)),
                ValueDef::TargetPower(TargetIndex::PRIMARY),
            ),
            EffectDef::AddPlayerCounters {
                recipient: EffectRecipientDef::Opponent,
                kind: CounterKind::Poison,
                amount: ValueDef::Constant(1),
            },
        ]),
    )),
);

// ONE 189 — Tyrranax Rex
// Audit: unsupported — Needs the toxic keyword: combat damage to a player must also give its
// fixed poison-counter amount without replacing the combat damage as infect does.
pub(in crate::card::sets) static TYRRANAX_REX_189: CardRecord = CardRecord::new(
    "Tyrranax Rex",
    "0fb52b44-da5f-4f7a-a6c2-7924b855e051",
    "Tuan Duong Chu",
    crate::card::CardRules::unsupported(),
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
                EffectDef::CreateToken(
                    CreateTokenDef::new(TokenDef::Literal(TokenCharacteristics::creature(
                        &["Phyrexian", "Goblin"],
                        &[ManaColor::Red],
                        1,
                        1,
                    )))
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
            ),
        ]),
);

// ONE 218 — Tyvar, Jubilant Brawler
// Audit: unsupported — Needs the static permission to activate creature abilities as though their
// sources had haste. The existing haste keyword does not alter another creature's activation rule.
pub(in crate::card::sets) static TYVAR_JUBILANT_BRAWLER_218: CardRecord = CardRecord::new(
    "Tyvar, Jubilant Brawler",
    "66605fe1-9a20-4c95-b53e-1249cedb978b",
    "Victor Adame Minguez",
    crate::card::CardRules::unsupported(),
);

// ONE 243 — Surgical Skullbomb
pub(in crate::card::sets) static SURGICAL_SKULLBOMB_243: CardRecord = CardRecord::new(
    "Surgical Skullbomb",
    "98c2b2af-739f-413c-8c36-da6f78df0acb",
    "Gaboleps",
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::activated(
            "{1}, Sacrifice this artifact: Draw a card.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::SacrificeSource],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_with_targets(
            "{2}{U}, Sacrifice this artifact: Return target creature to its owner's hand. Draw a card. Activate only as a sorcery.",
            &[CostDef::Mana(mana_cost!("{2}{U}")), CostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Sequence(&[
                EffectDef::move_to_zone(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ZoneKind::Hand,
                    ZonePlacement::Top,
                ),
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ]),
        )
        .with_activation_timing(crate::card::ActivationTimingDef::SorcerySpeed),
    ]),
);

// ONE 246 — Zenith Chronicler
// Audit: unsupported — Needs a cast trigger filtered to each player's first multicolored spell of
// the turn; existing spell-cast predicates do not retain that per-player qualified ordinal.
pub(in crate::card::sets) static ZENITH_CHRONICLER_246: CardRecord = CardRecord::new(
    "Zenith Chronicler",
    "1431fe83-7dc7-4c40-8d66-6525560e4323",
    "Johann Bodin",
    crate::card::CardRules::unsupported(),
);

// ONE 346 — Mondrak, Glory Dominus
pub(in crate::card::sets) static MONDRAK_GLORY_DOMINUS_346: CardRecord = CardRecord::new(
    "Mondrak, Glory Dominus",
    "1ef1b6a8-0151-4e41-a909-3d519dc19f14",
    "rishxxv",
    CardRules::new_creature(mana_cost!("{2}{W}{W}"), &["Phyrexian", "Horror"], 4, 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "If one or more tokens would be created under your control, twice that many of those tokens are created instead.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Controller,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::DoublesTokensCreated),
                },
            ),
            AbilityDef::activated(
                "{1}{W/P}{W/P}, Sacrifice two other artifacts and/or creatures: Put an indestructible counter on this creature.",
                &[
                    CostDef::Mana(mana_cost!("{1}{W/P}{W/P}")),
                    CostDef::SacrificePermanents {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                ObjectPredicateDef::HasType(CardType::Creature),
                            ]),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        controller: PlayerRelation::You,
                        count: 2,
                    },
                ],
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::Indestructible,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// ONE 358 — Staff of Compleation
pub(in crate::card::sets) static STAFF_OF_COMPLEATION_358: CardRecord = CardRecord::new(
    "Staff of Compleation",
    "315490d2-4d1f-4065-9d18-4682f1d7d066",
    "Joshua Alvarado",
    CardRules::new_artifact(mana_cost!("{3}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{T}, Pay 1 life: Destroy target permanent you own.",
            &[CostDef::TapSource, CostDef::PayLife(1)],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Any,
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )],
            EffectDef::destroy_target(TargetIndex::PRIMARY),
        ),
        AbilityDef::activated_mana(
            "{T}, Pay 2 life: Add one mana of any color.",
            &[CostDef::TapSource, CostDef::PayLife(2)],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
        AbilityDef::activated(
            "{T}, Pay 3 life: Proliferate.",
            &[CostDef::TapSource, CostDef::PayLife(3)],
            EffectDef::Proliferate,
        ),
        AbilityDef::activated(
            "{T}, Pay 4 life: Draw a card.",
            &[CostDef::TapSource, CostDef::PayLife(4)],
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated(
            "{5}: Untap this artifact.",
            &[CostDef::Mana(mana_cost!("{5}"))],
            EffectDef::Untap {
                object: EffectRecipientDef::Source,
            },
        ),
    ]),
);

// ONE 388 — Vindictive Flamestoker
pub(in crate::card::sets) static VINDICTIVE_FLAMESTOKER_388: CardRecord = CardRecord::new(
    "Vindictive Flamestoker",
    "6c266012-6374-4870-917a-532fadf917ad",
    "Xavier Ribeiro",
    CardRules::new_creature(mana_cost!("{R}"), &["Phyrexian", "Wizard"], 1, 2).with_abilities(&[
AbilityDef::triggered("Whenever you cast a noncreature spell, put an oil counter on this creature.", TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[ObjectPredicateDef::NoncreatureSpell, ObjectPredicateDef::ControlledBy(PlayerRelation::You)])), EffectDef::AddCounters { object: EffectRecipientDef::Source, kind: CounterKind::named("oil"), amount: ValueDef::Constant(1) }),
AbilityDef::activated("{6}{R}, Sacrifice this creature: Discard your hand, then draw four cards. This ability costs {1} less to activate for each oil counter on this creature.", &[CostDef::Mana(mana_cost!("{6}{R}")), CostDef::SacrificeSource], EffectDef::Sequence(&[EffectDef::Discard { recipient: EffectRecipientDef::Controller, amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(ObjectPredicateDef::Any, &[ZoneKind::Hand], PlayerRelation::You)), selection: DiscardSelectionDef::RecipientChooses, then: None }, abilities::draw_cards(ValueDef::Constant(4))])).with_activation_cost_reduction(ValueDef::CountersOnSource(CounterKind::named("oil")), 0)
]),
);

// ONE 397 — Soulless Jailer
// Audit: unsupported — Needs global zone-change prevention for permanent cards in graveyards and
// a cast restriction limited to noncreature spells cast from graveyard or exile.
pub(in crate::card::sets) static SOULLESS_JAILER_397: CardRecord = CardRecord::new(
    "Soulless Jailer",
    "45354872-4426-445e-8ef0-2df65afdbc53",
    "Donato Giancola",
    crate::card::CardRules::unsupported(),
);

// ONE 400 — Mirrex
// Audit: unsupported — Needs the toxic keyword on its created Mite token; token creation can
// declare the body and can't-block text, but not toxic's combat-damage poison rider.
pub(in crate::card::sets) static MIRREX_400: CardRecord = CardRecord::new(
    "Mirrex",
    "2b7a760f-c9fb-454c-bedd-46a675daf02e",
    "Adam Burn",
    crate::card::CardRules::unsupported(),
);

// ONE 402 — The Mycosynth Gardens
pub(in crate::card::sets) static THE_MYCOSYNTH_GARDENS_402: CardRecord = CardRecord::new(
    "The Mycosynth Gardens",
    "4afcac49-ac80-4561-ba2c-ce9487e9d8fe",
    "Andrew Mar",
    CardRules::new_land(&["Sphere"]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_mana(
            "{1}, {T}: Add one mana of any color.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
        AbilityDef::activated_with_targets(
            "{X}, {T}: This land becomes a copy of target nontoken artifact you control with mana value X.",
            &[CostDef::Mana(mana_cost!("{X}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                    ObjectPredicateDef::ManaValueEqualTo(ValueDef::ChosenX),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            })],
            EffectDef::BecomeCopyOf {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                copier: None,
                exceptions: CopyExceptionsDef::NONE,
                duration: None,
            },
        ),
    ]),
);

// ONE 416 — Elesh Norn, Mother of Machines
// Audit: unsupported — Needs a trigger replacement that suppresses opponents' permanent triggers
// caused by entrants, alongside the existing controller-side additional-trigger rule.
pub(in crate::card::sets) static ELESH_NORN_MOTHER_OF_MACHINES_416: CardRecord = CardRecord::new(
    "Elesh Norn, Mother of Machines",
    "649be99a-fa52-469e-85df-11ecc576ea39",
    "Richard Whitters",
    crate::card::CardRules::unsupported(),
);

// ONE 427 — Skrelv, Defector Mite
// Audit: unsupported — Needs the toxic keyword for the printed and granted toxic 1; the color
// choice and color-scoped combat evasion cannot make the card complete without that rider.
pub(in crate::card::sets) static SKRELV_DEFECTOR_MITE_427: CardRecord = CardRecord::new(
    "Skrelv, Defector Mite",
    "2e55ca48-0fe0-44bd-9453-02cda0b7f5da",
    "Sidharth Chaturvedi",
    crate::card::CardRules::unsupported(),
);

// ONE 479 — Myr Convert
// Audit: unsupported — Needs the toxic keyword, whose combat-damage poison rider differs from
// the engine's supported infect replacement.
pub(in crate::card::sets) static MYR_CONVERT_479: CardRecord = CardRecord::new(
    "Myr Convert",
    "19c7d89a-2b02-4faa-83cf-7dcf7faf6c4a",
    "JungShan",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &OSSIFICATION_26,
    &PLANAR_DISRUPTION,
    &ENCROACHING_MYCOSYNTH_47,
    &MINOR_MISSTEP_64,
    &UNCTUS_GRAND_METATECT_75,
    &ANNIHILATING_GLARE,
    &OFFER_IMMORTALITY,
    &SHEOLDRED_S_EDICT,
    &BARBED_BATTERFIST,
    &BLAZING_CRESCENDO,
    &FURNACE_STRIDER,
    &CANKERBLOOM,
    &CONTAGIOUS_VORRAC,
    &INFECTIOUS_BITE_172,
    &TYRRANAX_REX_189,
    &ATRAXA_GRAND_UNIFIER,
    &OVIKA_ENIGMA_GOLIATH,
    &TYVAR_JUBILANT_BRAWLER_218,
    &SURGICAL_SKULLBOMB_243,
    &ZENITH_CHRONICLER_246,
    &MONDRAK_GLORY_DOMINUS_346,
    &STAFF_OF_COMPLEATION_358,
    &VINDICTIVE_FLAMESTOKER_388,
    &SOULLESS_JAILER_397,
    &MIRREX_400,
    &THE_MYCOSYNTH_GARDENS_402,
    &ELESH_NORN_MOTHER_OF_MACHINES_416,
    &SKRELV_DEFECTOR_MITE_427,
    &MYR_CONVERT_479,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
