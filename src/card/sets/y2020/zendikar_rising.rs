//! Zendikar Rising cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::ParentBinding;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AlternativeCastKindDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::BattlefieldArrivalDef;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CardTypeSet;
use crate::card::CharacteristicOperationDef;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseDef;
use crate::card::ComparisonDef;
use crate::card::ControlDurationDef;
use crate::card::CostDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::CreatureTypeSetDef;
use crate::card::DamageDef;
use crate::card::DamageFollowUpDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::KeywordAbility;
use crate::card::ManaColor;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::ReplacementConditionDef;
use crate::card::ReplacementEffectDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SetOperationDef;
use crate::card::SubtypeDef;
use crate::card::SumValueDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TokenStatsDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "ZNR",
    slug: "zendikar-rising",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// ZNR 4 — Archon of Emeria
pub(in crate::card::sets) static ARCHON_OF_EMERIA_4: CardRecord = CardRecord::new(
    "Archon of Emeria",
    "228c1650-da3c-4099-91b6-18e3873c9cdb",
    "Ryan Pancoast",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Archon"], 2, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "Each player can't cast more than one spell each turn.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::EachPlayer,
                effect: AppliedEffectDef::Rule(crate::card::AppliedRuleDef::CannotPlay(
                    crate::card::PlayRestrictionDef::new(
                        crate::card::PlayActionMatcherDef::CastSpell,
                        ObjectPredicateDef::Any,
                    )
                    .after_spells_cast(1),
                )),
            },
        ),
        AbilityDef::replacement_for(
            "Nonbasic lands your opponents control enter tapped.",
            crate::card::ReplacementEventDef::ObjectEntersBattlefield {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(CardSupertype::Basic)),
                ]),
                controller: PlayerRelation::Opponent,
                cast: None,
            },
            ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped),
        ),
    ]),
);

// ZNR 9 — Dauntless Unity
pub(in crate::card::sets) static DAUNTLESS_UNITY: CardRecord = CardRecord::new(
    "Dauntless Unity",
    "b12a4d17-68e6-4133-99fd-e501e24e6c6b",
    "Josu Hernaiz",
// The kicked mode trades a point of toughness for a point of power, so
    // it is the better combat trick and the worse blocking one.
    CardRules::new_instant(mana_cost!("{1}{W}")).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{2}{W}{W}"))],
            AlternativeCastKindDef::Kicked,
            Some("Kicker {1}{W} (You may pay an additional {1}{W} as you cast this spell.)"),
            EffectDef::None,
        ),
        AbilityDef::spell(
            "Creatures you control get +1/+1 until end of turn. If this spell was kicked, those creatures get +2/+1 until end of turn instead.",
            // "Instead" makes the two exclusive, so one condition picks a
            // branch rather than the kicked mode stacking on the base one.
            EffectDef::IfElseCondition {
                condition: &TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked),
                then: &EffectDef::Apply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(1),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                otherwise: &EffectDef::Apply {
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
            },
        ),
    ]),
);

// ZNR 16 — Felidar Retreat
pub(in crate::card::sets) static FELIDAR_RETREAT: CardRecord = CardRecord::new(
    "Felidar Retreat",
    "45340647-4d3e-4be1-b0e6-e40cc56a438b",
    "Ralph Horsley",
    CardRules::new_enchantment(mana_cost!("{3}{W}")).with_abilities(&[
        AbilityDef::modal_triggered(
            "Landfall — Whenever a land you control enters, choose one —",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            &[
                AbilityDef::spell(
                    "Create a 2/2 white Cat Beast creature token.",
                    EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                        TokenCharacteristics::creature(
                            &["Cat", "Beast"],
                            &[ManaColor::White],
                            2,
                            2,
                        ),
                    ))),
                ),
                AbilityDef::spell(
                    "Put a +1/+1 counter on each creature you control. Those \
                     creatures gain vigilance until end of turn.",
                    EffectDef::Sequence(&[
                        EffectDef::AddCounters {
                            object: EffectRecipientDef::objects(ObjectSetDef::Query(
                                ObjectQueryDef::matching(
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    &[ZoneKind::Battlefield],
                                    PlayerRelation::You,
                                ),
                            )),
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::Constant(1),
                        },
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                                ObjectQueryDef::matching(
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    &[ZoneKind::Battlefield],
                                    PlayerRelation::You,
                                ),
                            )),
                            effect: AppliedEffectDef::add_ability(&abilities::vigilance()),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                    ]),
                ),
            ],
        ),
    ]),
);

// ZNR 39 — Skyclave Apparition
pub(in crate::card::sets) static SKYCLAVE_APPARITION: CardRecord = CardRecord::new(
    "Skyclave Apparition",
    "b83cfbaa-7890-4f6f-878b-4edb45677371",
    "Donato Giancola",
// Three mana for a body and an answer, and the answer is only undone by
    // killing the body -- which hands back an Illusion rather than the card.
    CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Kor", "Spirit"], 2, 2)
        .with_abilities(&[
            abilities::enters_trigger_with_targets(
                "When this creature enters, exile up to one target nonland, nontoken permanent you don't \
                 control with mana value 4 or less.",
                // Everything the exile clause excludes, in one predicate: a land is safe, a
                // token is safe, and anything expensive is safe. "You don't control" is the
                // controller half rather than part of the predicate.
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                            ObjectPredicateDef::ManaValueAtMost(4),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::Opponent),
                        owner: None,
                    },
                    1,
                )],
                EffectDef::ExileLinkedToSource {
                    until_source_leaves: false,
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    face_down: false,
                    then: None,
                },
            ),
            // Leaves, not dies: the card stays in exile whatever happened to the
            // Apparition, and the token is what its owner gets instead.
            AbilityDef::triggered(
                "When this creature leaves the battlefield, the exiled card's owner creates an X/X blue \
                 Illusion creature token, where X is the mana value of the exiled card.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    None,
                ),
                abilities::bind_objects_then(
                    crate::card::ObjectCollectionSourceDef::ObjectSet(
                        ObjectSetDef::LinkedExiles,
                    ),
                    // One token per exiled card, which is one token: the exile clause is "up to
                    // one target". Binding the pile is also what makes the clause do nothing at
                    // all when nothing was exiled -- the Apparition that entered with no legal
                    // target leaves without paying anybody.
                    &EffectDef::ForEachInBinding {
                        objects: ParentBinding,
                        binding: ParentBinding,
                        // The token is the exiled card's owner's, not the Apparition controller's:
                        // what they get back for the permanent that is not coming back.
                        effect: &EffectDef::CreateToken(
                            CreateTokenDef::new(TokenDef::Literal(
                                TokenCharacteristics::creature_with_stats(
                                    &["Illusion"],
                                    &[ManaColor::Blue],
                                    // "Where X is the mana value of the exiled card": both halves read the same
                                    // card, which is the one the leave trigger just bound.
                                    &TokenStatsDef {
                                        power: ValueDef::ObjectManaValue(ObjectRefDef::Binding(ParentBinding)),
                                        toughness: ValueDef::ObjectManaValue(ObjectRefDef::Binding(ParentBinding)),
                                    },
                                ),
                            ))
                            .with_controller(PlayerRefDef::OwnerOf(ObjectRefDef::Binding(ParentBinding))),
                        ),
                    },
                ),
            ),
        ]),
);

// ZNR 60 — Glasspool Mimic // Glasspool Shore
pub(in crate::card::sets) static GLASSPOOL_MIMIC_GLASSPOOL_SHORE_60: CardRecord = CardRecord::new_mdfc(
    "Glasspool Mimic // Glasspool Shore",
    "5adcb500-8c77-4925-8e2c-1243502827d1",
    "Johan Grenier",
    &[("Glasspool Mimic", CardRules::new_creature(mana_cost!("{2}{U}"), &["Shapeshifter", "Rogue"], 0, 0).with_ability(AbilityDef::replacement("You may have this creature enter as a copy of a creature you control, except it's a Shapeshifter Rogue in addition to its other types.", ReplacementEffectDef::CopyEntering { object: ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::ControlledBy(PlayerRelation::You)]), exceptions: crate::card::CopyExceptionsDef::NONE.with_added_creature_types(&["Shapeshifter", "Rogue"]) }))), ("Glasspool Shore", CardRules::new_land(&[]).with_abilities(&[AbilityDef::as_enters("This land enters tapped.", ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped)), abilities::tap_for(ManaColor::Blue)]))],
);

// ZNR 76 — Sea Gate Restoration // Sea Gate, Reborn
pub(in crate::card::sets) static SEA_GATE_RESTORATION_SEA_GATE_REBORN_76: CardRecord =
    CardRecord::new_mdfc(
        "Sea Gate Restoration // Sea Gate, Reborn",
        "193071fe-180b-4d35-ba78-9c16675c29fc",
        "Adam Paquette",
        &[
            (
                "Sea Gate Restoration",
                CardRules::new_sorcery(mana_cost!("{4}{U}{U}{U}")).with_ability(
                    AbilityDef::spell(
                        "Draw cards equal to the number of cards in your hand plus one. You have no maximum hand size for the rest of the game.",
                        EffectDef::Sequence(&[
                            EffectDef::DrawCards {
                                recipient: EffectRecipientDef::Controller,
                                amount: ValueDef::CardsInHandAbove {
                                    player: PlayerRelation::You,
                                    threshold: 0,
                                },
                            },
                            EffectDef::DrawCards {
                                recipient: EffectRecipientDef::Controller,
                                amount: ValueDef::Constant(1),
                            },
                            EffectDef::Apply {
                                recipient: EffectRecipientDef::Controller,
                                effect: AppliedEffectDef::Rule(crate::card::AppliedRuleDef::PlayerRule(
                                    crate::card::PlayerRuleDef::NoMaximumHandSize,
                                )),
                                duration: ResolvedEffectDurationDef::Permanent,
                            },
                        ]),
                    ),
                ),
            ),
            (
                "Sea Gate, Reborn",
                CardRules::new_land(&[]).with_abilities(&[
                    AbilityDef::replacement(
                        "As this land enters, you may pay 3 life. If you don't, it enters tapped.",
                        ReplacementEffectDef::PayOr {
                            payment: crate::card::EffectPaymentDef::new(
                                crate::card::PlayerSetDef::Related(PlayerRelation::You),
                                &[CostDef::PayLife(3)],
                            ),
                            if_paid: &[],
                            if_declined: &[ReplacementEffectDef::ModifyBattlefieldEntry(
                                BattlefieldEntryModificationDef::Tapped,
                            )],
                        },
                    ),
                    abilities::tap_for(ManaColor::Blue),
                ]),
            ),
        ],
    );

// ZNR 80 — Silundi Vision // Silundi Isle
pub(in crate::card::sets) static SILUNDI_VISION_SILUNDI_ISLE_80: CardRecord = CardRecord::new_mdfc(
    "Silundi Vision // Silundi Isle",
    "11568cdf-6148-494c-8b98-f5ca5797d775",
    "Randy Vargas",
    &[("Silundi Vision", CardRules::new_instant(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell("Look at the top six cards of your library. You may reveal an instant or sorcery card from among them and put it into your hand. Put the rest on the bottom of your library in a random order.", abilities::look_at_top_cards_reveal_choice_to_hand_rest_random_bottom(ValueDef::Constant(6), ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::HasType(CardType::Instant), ObjectPredicateDef::HasType(CardType::Sorcery)]), 0, 1)))), ("Silundi Isle", CardRules::new_land(&[]).with_abilities(&[AbilityDef::as_enters("This land enters tapped.", ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped)), abilities::tap_for(ManaColor::Blue)]))],
);

// ZNR 85 — Thieving Skydiver
pub(in crate::card::sets) static THIEVING_SKYDIVER: CardRecord = CardRecord::new(
    "Thieving Skydiver",
    "ff84ea71-e477-44f7-a3f8-77fef708efeb",
    "Kieran Yanner",
    // Two mana for a flier, or two plus X for a flier that takes the best
    // artifact on the board with it -- a Mox on turn three, a Sword on turn
    // five, and the Sword comes down already attached.
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Merfolk", "Rogue"], 2, 1).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{X}{1}{U}"))],
            AlternativeCastKindDef::Kicked,
            Some(
                "Kicker {X}. X can't be 0. (You may pay an additional {X} as you cast this \
                 spell.)",
            ),
            EffectDef::None,
        )
        .with_alternative_minimum_x(1),
        abilities::flying(),
        AbilityDef::triggered_if_with_targets(
            "When this creature enters, if it was kicked, gain control of target artifact with \
             mana value X or less. If that artifact is an Equipment, attach it to this creature.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            // "If it was kicked", asked as the arrival resolves. The kick is what the
            // whole card is: unkicked he is a 2/1 flier and nothing else happens.
            &TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked),
            // "Target artifact with mana value X or less", where X is what his own cast
            // paid: the target is sized by the kick rather than by anything printed.
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::ManaValueAtMostValue(ValueDef::SourceCastX),
                ]),
            )],
            EffectDef::Sequence(&[
                EffectDef::gain_control(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    PlayerRefDef::EffectController,
                    ControlDurationDef::Indefinitely,
                ),
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::TargetMatches {
                        slot: TargetIndex::PRIMARY,
                        object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Equipment")),
                    },
                    then: &EffectDef::AttachToSource {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                },
            ]),
        ),
    ]),
);

// ZNR 94 — Bloodchief's Thirst
pub(in crate::card::sets) static BLOODCHIEFS_THIRST: CardRecord = CardRecord::new(
    "Bloodchief's Thirst",
    "059e8447-6b1c-4651-a734-a8fea2cbf7b2",
    "Jason Rainville",
// One black kills most of what an aggressive deck leads with; four kills
    // whatever is left, which is why the card is played over a cheaper
    // removal spell that can only do the first job.
    CardRules::new_sorcery(mana_cost!("{B}")).with_abilities(&[
        abilities::kicker(
            &[CostDef::Mana(mana_cost!("{2}{B}"))],
        ),
        AbilityDef::spell_with_targets(
            "Destroy target creature or planeswalker with mana value 2 or less. If this spell was kicked, instead destroy target creature or planeswalker.",
            // The mana-value bound is part of what may be targeted rather than something
            // checked on resolution, so an unkicked Thirst never points at anything
            // bigger in the first place.
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::IfAdditionalCostPaid {
                    cost: crate::AdditionalCostIndex::PRIMARY,
                    if_paid: &AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Planeswalker),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    otherwise: &AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::HasType(CardType::Planeswalker),
                            ]),
                            ObjectPredicateDef::ManaValueAtMost(2),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                },
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
    ]),
);

// ZNR 102 — Feed the Swarm
pub(in crate::card::sets) static FEED_THE_SWARM: CardRecord = CardRecord::new(
    "Feed the Swarm",
    "f6b2eba7-862a-4efd-9f65-065fb2070855",
    "Andrey Kuzinskiy",
    CardRules::new_sorcery(mana_cost!("{1}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Destroy target creature or enchantment an opponent controls. \
         You lose life equal to that permanent's mana value.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::Opponent),
                owner: None,
            },
        )],
        EffectDef::Sequence(&[
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::ObjectManaValue(ObjectRefDef::Target(TargetIndex::PRIMARY)),
            },
        ]),
    )]),
);

// ZNR 107 — Highborn Vampire
pub(in crate::card::sets) static HIGHBORN_VAMPIRE: CardRecord = CardRecord::new(
    "Highborn Vampire",
    "24c40082-516e-4381-a4cc-e61c5a9a6cac",
    "Denman Rooke",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Vampire", "Warrior"], 4, 3),
);

// ZNR 111 — Malakir Rebirth // Malakir Mire
pub(in crate::card::sets) static MALAKIR_REBIRTH_MALAKIR_MIRE_111: CardRecord = CardRecord::new_mdfc(
    "Malakir Rebirth // Malakir Mire",
    "609d3ecf-f88d-4268-a8d3-4bf2bcf5df60",
    "Marta Nael",
    &[("Malakir Rebirth", CardRules::new_instant(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets("Choose target creature. You lose 2 life. Until end of turn, that creature gains \"When this creature dies, return it to the battlefield tapped under its owner's control.\"", &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Creature))], EffectDef::Sequence(&[EffectDef::LoseLife { recipient: EffectRecipientDef::Controller, amount: ValueDef::Constant(2) }, EffectDef::Apply { recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY), effect: AppliedEffectDef::add_ability(&AbilityDef::triggered("When this creature dies, return it to the battlefield tapped under its owner's control.", TriggerEventDef::zone_changed(ObjectPredicateDef::Source, Some(ZoneKind::Battlefield), Some(ZoneKind::Graveyard)), EffectDef::WithBattlefieldArrival { effect: &EffectDef::move_to_zone(EffectRecipientDef::object(ObjectRefDef::ZoneChangeResultOfTriggeringObject), ZoneKind::Battlefield, ZonePlacement::Top), arrival: BattlefieldArrivalDef { controller: None, modifications: &[BattlefieldEntryModificationDef::Tapped], attachment: None, counters: None } })), duration: ResolvedEffectDurationDef::UntilEndOfTurn }])))), ("Malakir Mire", CardRules::new_land(&[]).with_abilities(&[abilities::enters_tapped(CardType::Land), abilities::tap_for(ManaColor::Black)]))]);

// ZNR 112 — Marauding Blight-Priest
pub(in crate::card::sets) static MARAUDING_BLIGHT_PRIEST: CardRecord = CardRecord::new(
    "Marauding Blight-Priest",
    "730ddbcd-0814-4e22-85e9-78b0878324b6",
    "Caio Monteiro",
    CardRules::new_creature(mana_cost!("{2}{B}"), &["Vampire", "Cleric"], 3, 2).with_abilities(&[
        AbilityDef::triggered(
            "Whenever you gain life, each opponent loses 1 life.",
            TriggerEventDef::LifeGained(PlayerRelation::You),
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Opponent,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// ZNR 118 — Nullpriest of Oblivion
pub(in crate::card::sets) static NULLPRIEST_OF_OBLIVION: CardRecord = CardRecord::new(
    "Nullpriest of Oblivion",
    "086fc7fb-efcf-4676-8455-39b63edaec6a",
    "Yongjae Choi",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Vampire", "Cleric"], 2, 1).with_abilities(&[
        abilities::kicker(&[CostDef::Mana(mana_cost!("{3}{B}"))]),
        abilities::lifelink(),
        abilities::menace(),
        AbilityDef::triggered_if_with_targets(
            "When this creature enters, if it was kicked, return target \
             creature card from your graveyard to the battlefield.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &TriggerConditionDef::SourcePaidAdditionalCost(crate::AdditionalCostIndex::PRIMARY),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Graveyard],
                    controller: None,
                    owner: Some(PlayerRelation::You),
                },
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Battlefield,
                ZonePlacement::Top,
            ),
        ),
    ]),
);

// ZNR 153 — Relic Robber
pub(in crate::card::sets) static RELIC_ROBBER_153: CardRecord = CardRecord::new(
    "Relic Robber",
    "4540205c-eee8-4db3-8757-710de874b313",
    "Slawomir Maniak",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin", "Rogue"], 2, 2).with_abilities(&[
abilities::haste(),
AbilityDef::triggered("Whenever this creature deals combat damage to a player, that player creates a 0/1 colorless Goblin Construct artifact creature token with \"This token can't block\" and \"At the beginning of your upkeep, this token deals 1 damage to you.\"", TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source), EffectDef::CreateToken(crate::card::CreateTokenDef::new(crate::card::TokenDef::Literal(crate::card::TokenCharacteristics::artifact_creature(&["Goblin", "Construct"], &[], 0, 1).with_abilities(&[AbilityDef::static_ability("This token cannot block.", EffectDef::StaticApply { recipient: EffectRecipientDef::Source, effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BLOCK) }), AbilityDef::triggered("At the beginning of your upkeep, this token deals 1 damage to you.", TriggerEventDef::StepBegins { step: TurnStepDef::Upkeep, player: PlayerRelation::You }, EffectDef::damage(EffectRecipientDef::Controller, ValueDef::Constant(1)))]))).with_controller(PlayerRefDef::EventPlayer)))
]),
);

// ZNR 156 — Roiling Vortex
// Audit: unsupported — Needs an upkeep trigger for each player, a spell-cast mana-spent condition, and a turn-duration opponent life-gain prohibition.
pub(in crate::card::sets) static ROILING_VORTEX_156: CardRecord = CardRecord::new(
    "Roiling Vortex",
    "0b057eb7-8439-4d26-89df-c345ab2773e1",
    "Campbell White",
    crate::card::CardRules::unsupported(),
);

// ZNR 164 — Sneaking Guide
pub(in crate::card::sets) static SNEAKING_GUIDE_164: CardRecord = CardRecord::new(
    "Sneaking Guide",
    "569c9e8c-7808-49d0-82c1-72d5b835f51c",
    "Dan Murayama Scott",
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin", "Rogue"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{2}, {T}: Target creature with power 2 or less can't be blocked this turn.",
            &[CostDef::Mana(mana_cost!("{2}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::PowerGreaterThan(
                        ValueDef::Constant(2),
                    )),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(crate::card::AppliedRuleDef::CANNOT_BE_BLOCKED),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// ZNR 166 — Spikefield Hazard // Spikefield Cave
pub(in crate::card::sets) static SPIKEFIELD_HAZARD_SPIKEFIELD_CAVE_166: CardRecord =
    CardRecord::new_mdfc(
    "Spikefield Hazard // Spikefield Cave",
    "a69541db-3f4e-412f-aa8e-dec1e74f74dc",
    "Tomasz Jedruszek",
    &[("Spikefield Hazard", CardRules::new_instant(mana_cost!("{R}")).with_ability(AbilityDef::spell_with_targets("Spikefield Hazard deals 1 damage to any target. If a permanent dealt damage this way would die this turn, exile it instead.", &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget)], EffectDef::DealDamage(DamageDef::new(EffectRecipientDef::Target(TargetIndex::PRIMARY), ValueDef::Constant(1)).with_follow_up(DamageFollowUpDef::ApplyToDamaged { effect: &AppliedEffectDef::Rule(AppliedRuleDef::ExileInsteadOfDying), duration: ResolvedEffectDurationDef::UntilEndOfTurn }))))), ("Spikefield Cave", CardRules::new_land(&[]).with_abilities(&[abilities::enters_tapped(CardType::Land), abilities::tap_for(ManaColor::Red)]))]);

// ZNR 167 — Spitfire Lagac
pub(in crate::card::sets) static SPITFIRE_LAGAC: CardRecord = CardRecord::new(
    "Spitfire Lagac",
    "47f26493-812f-4c14-91c4-d2ab549a7b8a",
    "Antonio José Manzanedo",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Lizard"], 3, 4).with_abilities(&[
        AbilityDef::triggered(
            "Landfall — Whenever a land you control enters, this creature \
             deals 1 damage to each opponent.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::Constant(1)),
        ),
    ]),
);

// ZNR 174 — Valakut Awakening // Valakut Stoneforge
pub(in crate::card::sets) static VALAKUT_AWAKENING_VALAKUT_STONEFORGE_174: CardRecord =
    CardRecord::new_mdfc(
    "Valakut Awakening // Valakut Stoneforge",
    "228e551e-023a-4c9a-8f32-58dae6ffdf7f",
    "Campbell White",
    &[("Valakut Awakening", CardRules::new_instant(mana_cost!("{2}{R}")).with_ability(AbilityDef::spell("Put any number of cards from your hand on the bottom of your library, then draw that many cards plus one.", EffectDef::Choose(ChooseDef { chooser: PlayerRefDef::EffectController, candidates: ObjectSetDef::Query(ObjectQueryDef::matching(ObjectPredicateDef::Any, &[ZoneKind::Hand], PlayerRelation::You)), exclude: None, minimum: 0, maximum: 255, binding: ObjectChoiceBindingDef::OrderedObjects(Binding!("awakening_hand")), unchosen: None, visibility: ChoiceVisibilityDef::Private, then: &EffectDef::Sequence(&[EffectDef::move_to_zone(EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!("awakening_hand"))), ZoneKind::Library, ZonePlacement::Bottom), abilities::draw_cards(ValueDef::Sum(&SumValueDef { left: ValueDef::BoundObjectCount(Binding!("awakening_hand")), right: ValueDef::Constant(1) }))]) })))), ("Valakut Stoneforge", CardRules::new_land(&[]).with_abilities(&[abilities::enters_tapped(CardType::Land), abilities::tap_for(ManaColor::Red)]))]);

// ZNR 178 — Ancient Greenwarden
pub(in crate::card::sets) static ANCIENT_GREENWARDEN: CardRecord = CardRecord::new(
    "Ancient Greenwarden",
    "dfe08e59-fdc4-436f-b05c-6ad386c46310",
    "Grzegorz Rutkowski",
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Elemental"], 5, 7).with_abilities(&[
        abilities::reach(),
        AbilityDef::static_ability(
            "You may play lands from your graveyard.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MayPlayFromGraveyard(
                    crate::card::GraveyardPlayPermissionDef::unlimited(
                        crate::card::PlayRestrictionDef::new(
                            crate::card::PlayActionMatcherDef::PlayLand,
                            ObjectPredicateDef::HasType(CardType::Land),
                        ),
                    ),
                )),
            },
        ),
        AbilityDef::static_ability(
            "If a land entering causes a triggered ability of a permanent you control to \
                 trigger, that ability triggers an additional time.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::ModifyTriggers(
                    &crate::card::TriggerModificationDef {
                        cause: TriggerEventDef::zone_changed(
                            ObjectPredicateDef::HasType(CardType::Land),
                            None,
                            Some(ZoneKind::Battlefield),
                        ),
                        permanent: Some(ObjectPredicateDef::Any),
                        kind: crate::card::TriggerModificationKindDef::Additional,
                    },
                )),
            },
        ),
    ]),
);

// ZNR 179 — Ashaya, Soul of the Wild
pub(in crate::card::sets) static ASHAYA_SOUL_OF_THE_WILD_179: CardRecord = CardRecord::new(
    "Ashaya, Soul of the Wild",
    "74943390-d25f-47cb-90bb-cbf70c87f4a2",
    "Chase Stone",
    CardRules::new_creature(mana_cost!("{3}{G}{G}"), &["Elemental"], 0, 0).with_supertype(CardSupertype::Legendary).with_abilities(&[AbilityDef::static_ability("Ashaya's power and toughness are each equal to the number of lands you control.", EffectDef::StaticApply { recipient: EffectRecipientDef::Source, effect: AppliedEffectDef::define_power_toughness(ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(ObjectPredicateDef::HasType(CardType::Land), &[ZoneKind::Battlefield], PlayerRelation::You)), ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(ObjectPredicateDef::HasType(CardType::Land), &[ZoneKind::Battlefield], PlayerRelation::You))) }), AbilityDef::static_ability("Nontoken creatures you control are Forest lands in addition to their other types. (They're still affected by summoning sickness.)", EffectDef::StaticApply { recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::Not(&ObjectPredicateDef::Token)]), &[ZoneKind::Battlefield], PlayerRelation::You), effect: AppliedEffectDef::Composite(&[AppliedEffectDef::Characteristic(CharacteristicOperationDef::CardTypes(SetOperationDef::Add(CardTypeSet::single(CardType::Land)))), AppliedEffectDef::add_basic_land_types(&[crate::card::BasicLandType::Forest])]) })]),
);

// ZNR 181 — Broken Wings
pub(in crate::card::sets) static BROKEN_WINGS: CardRecord = CardRecord::new(
    "Broken Wings",
    "c0fc2dfd-85b0-4add-be18-b39549235921",
    "Ekaterina Burmak",
    CardRules::new_instant(mana_cost!("{2}{G}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Destroy target artifact, enchantment, or creature with flying.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Enchantment),
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                ]),
            ]),
        )],
        EffectDef::Destroy {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            then: None,
        },
    )]),
);

// ZNR 185 — Gnarlid Colony
pub(in crate::card::sets) static GNARLID_COLONY: CardRecord = CardRecord::new(
    "Gnarlid Colony",
    "7327289d-eed8-44b1-8495-7172e2b49d5f",
    "Izzy",
    // A two-drop early or a four-power trampler late, and the anthem is what
    // pays a counters deck for playing it at either end.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Beast"], 2, 2).with_abilities(&[
        AbilityDef::alternative_cast(
            &[CostDef::Mana(mana_cost!("{3}{G}{G}"))],
            AlternativeCastKindDef::Kicked,
            Some("Kicker {2}{G} (You may pay an additional {2}{G} as you cast this spell.)"),
            EffectDef::None,
        ),
        AbilityDef::as_enters_if(
            "If this creature was kicked, it enters with two +1/+1 counters on it.",
            ReplacementConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked),
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCounters {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: 2,
                },
            ),
        ),
        AbilityDef::static_ability(
            "Each creature you control with a +1/+1 counter on it has trample. (It can deal \
             excess combat damage to the player or planeswalker it's attacking.)",
            EffectDef::StaticApply {
                // Itself included once it is kicked, which is what makes the
                // kicked mode a trampler rather than just a bigger body.
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasCounter(CounterKind::PlusOnePlusOne),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::trample()),
            },
        ),
    ]),
);

// ZNR 232 — Omnath, Locus of Creation
const fn omnath_resolution(amount: u8) -> TriggerConditionDef {
    TriggerConditionDef::SourceResolutionsThisTurn {
        comparison: ComparisonDef::Equal,
        amount,
    }
}

pub(in crate::card::sets) static OMNATH_LOCUS_OF_CREATION: CardRecord =
    CardRecord::new(
    "Omnath, Locus of Creation",
    "4e4fb50c-a81f-44d3-93c5-fa9a0b37f617",
    "Chris Rahn",
// Four colours for a 4/4 that replaces itself, and a deck full of
        // fetchlands turns the third land of a turn into eight damage.
        CardRules::new_creature(mana_cost!("{R}{G}{W}{U}"), &["Elemental"], 4, 4)
            .with_supertype(CardSupertype::Legendary)
            .with_abilities(&[
                abilities::enters_trigger(
                    "When Omnath enters, draw a card.",
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                ),
                AbilityDef::triggered(
                    "Landfall — Whenever a land you control enters, you gain 4 life if this is the first time \
                     this ability has resolved this turn. If it's the second time, add {R}{G}{W}{U}. If it's \
                     the third time, Omnath deals 4 damage to each opponent and each planeswalker you don't \
                     control.",
                    TriggerEventDef::zone_changed(
                        // A land arriving under its controller. Landfall watches the battlefield
                        // rather than the land drop, so a land put onto the battlefield by a fetch
                        // or a search counts the same way one played from hand does.
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ]),
                        None,
                        Some(ZoneKind::Battlefield),
                    ),
                    // Three exclusive branches on one count, so a fourth land does nothing at
                    // all rather than repeating the third.
                    EffectDef::Sequence(&[
                        EffectDef::IfCondition {
                            // The count includes the resolution asking, so the first time reads one.
                            condition: &omnath_resolution(1),
                            then: &EffectDef::GainLife {
                                recipient: EffectRecipientDef::Controller,
                                amount: ValueDef::Constant(4),
                            },
                        },
                        EffectDef::IfCondition {
                            condition: &omnath_resolution(2),
                            // Four mana of four colours is four separate additions: what the pool ends
                            // up holding is the same either way, and one `AddMana` names a run of like
                            // units plus at most one other.
                            then: &EffectDef::Sequence(&[
                                EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Red)),
                                EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
                                EffectDef::AddMana(AddManaEffectDef::one(ManaColor::White)),
                                EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Blue)),
                            ]),
                        },
                        EffectDef::IfCondition {
                            condition: &omnath_resolution(3),
                            then: &EffectDef::Sequence(&[
                                EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::Constant(4)),
                                EffectDef::damage(
                                    EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                                        ObjectPredicateDef::HasType(CardType::Planeswalker),
                                        &[ZoneKind::Battlefield],
                                        PlayerRelation::NotYou,
                                    ))),
                                    ValueDef::Constant(4),
                                ),
                            ]),
                        },
                    ]),
                ),
            ]),
);

// ZNR 245 — Lithoform Engine
static LITHOFORM_RETARGET_COPY: crate::card::CopyStackObjectDef = crate::card::CopyStackObjectDef {
    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    controller: PlayerRefDef::EffectController,
    count: ValueDef::Constant(1),
    retarget: true,
    colors: None,
};

pub(in crate::card::sets) static LITHOFORM_ENGINE: CardRecord = CardRecord::new(
    "Lithoform Engine",
    "6683416a-5820-4cd0-b28a-60a53239e9ef",
    "Colin Boyer",
CardRules::new_artifact(mana_cost!("{4}"))
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated_with_targets(
                "{2}, {T}: Copy target activated or triggered ability you control. You may choose new targets for the copy.",
                &[
                    CostDef::Mana(mana_cost!("{2}")),
                    CostDef::TapSource,
                ],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Ability,
                        zones: &[ZoneKind::Stack],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::CopyStackObject(&LITHOFORM_RETARGET_COPY),
            ),
            AbilityDef::activated_with_targets(
                "{3}, {T}: Copy target instant or sorcery spell you control. You may choose new targets for the copy.",
                &[
                    CostDef::Mana(mana_cost!("{3}")),
                    CostDef::TapSource,
                ],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Spell,
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Instant),
                                ObjectPredicateDef::HasType(CardType::Sorcery),
                            ]),
                        ]),
                        zones: &[ZoneKind::Stack],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::CopyStackObject(&LITHOFORM_RETARGET_COPY),
            ),
            AbilityDef::activated_with_targets(
                "{4}, {T}: Copy target permanent spell you control. (The copy becomes a token.)",
                &[
                    CostDef::Mana(mana_cost!("{4}")),
                    CostDef::TapSource,
                ],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Spell,
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::HasType(CardType::Enchantment),
                                ObjectPredicateDef::HasType(CardType::Land),
                                ObjectPredicateDef::HasType(CardType::Planeswalker),
                            ]),
                        ]),
                        zones: &[ZoneKind::Stack],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::CopyStackObject(&crate::card::CopyStackObjectDef {
                    retarget: false,
                    ..LITHOFORM_RETARGET_COPY
                }),
            ),
        ]),
);

// ZNR 259 — Brightclimb Pathway // Grimclimb Pathway
pub(in crate::card::sets) static BRIGHTCLIMB_PATHWAY_GRIMCLIMB_PATHWAY_259: CardRecord =
    CardRecord::new_mdfc(
        "Brightclimb Pathway // Grimclimb Pathway",
        "d24c3d51-795d-4c01-a34a-3280fccd2d78",
        "Johannes Voss",
        &[
            (
                "Brightclimb Pathway",
                CardRules::new_land(&[]).with_ability(abilities::tap_for(ManaColor::White)),
            ),
            (
                "Grimclimb Pathway",
                CardRules::new_land(&[]).with_ability(abilities::tap_for(ManaColor::Black)),
            ),
        ],
    );

// ZNR 262 — Crawling Barrens
pub(in crate::card::sets) static CRAWLING_BARRENS: CardRecord = CardRecord::new(
    "Crawling Barrens",
    "7bd0e025-7a75-4641-a51a-27df9dcde05f",
    "Jonas De Ro",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{4}: Put two +1/+1 counters on this land. Then you may have \
             it become a 0/0 Elemental creature until end of turn. It's \
             still a land.",
            &[CostDef::Mana(mana_cost!("{4}"))],
            EffectDef::Sequence(&[
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(2),
                },
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    effect: &EffectDef::Apply {
                        recipient: EffectRecipientDef::Source,
                        effect: AppliedEffectDef::Composite(&[
                            AppliedEffectDef::add_card_types(CardTypeSet::single(
                                CardType::Creature,
                            )),
                            AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&[
                                "Elemental",
                            ])),
                            AppliedEffectDef::set_base_power_toughness(
                                ValueDef::Constant(0),
                                ValueDef::Constant(0),
                            ),
                        ]),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                },
            ]),
        ),
    ]),
);

// ZNR 263 — Needleverge Pathway // Pillarverge Pathway
pub(in crate::card::sets) static NEEDLEVERGE_PATHWAY_PILLARVERGE_PATHWAY_263: CardRecord =
    CardRecord::new_mdfc(
        "Needleverge Pathway // Pillarverge Pathway",
        "6559047e-6ede-4815-a3a0-389062094f9d",
        "Piotr Dura",
        &[
            (
                "Needleverge Pathway",
                CardRules::new_land(&[]).with_ability(abilities::tap_for(ManaColor::Red)),
            ),
            (
                "Pillarverge Pathway",
                CardRules::new_land(&[]).with_ability(abilities::tap_for(ManaColor::White)),
            ),
        ],
    );

// ZNR 264 — Riverglide Pathway // Lavaglide Pathway
pub(in crate::card::sets) static RIVERGLIDE_PATHWAY_LAVAGLIDE_PATHWAY_264: CardRecord =
    CardRecord::new_mdfc(
        "Riverglide Pathway // Lavaglide Pathway",
        "2668ac91-6cda-4f81-a08d-4fc5f9cb35b2",
        "Kieran Yanner",
        &[
            (
                "Riverglide Pathway",
                CardRules::new_land(&[]).with_ability(abilities::tap_for(ManaColor::Blue)),
            ),
            (
                "Lavaglide Pathway",
                CardRules::new_land(&[]).with_ability(abilities::tap_for(ManaColor::Red)),
            ),
        ],
    );

// ZNR 286 — Clearwater Pathway // Murkwater Pathway
pub(in crate::card::sets) static CLEARWATER_PATHWAY_MURKWATER_PATHWAY_286: CardRecord =
    CardRecord::new_mdfc(
        "Clearwater Pathway // Murkwater Pathway",
        "b0fe4b53-18f6-42eb-b03f-cab3e5a7fba6",
        "Johannes Voss",
        &[
            (
                "Clearwater Pathway",
                CardRules::new_land(&[]).with_ability(abilities::tap_for(ManaColor::Blue)),
            ),
            (
                "Murkwater Pathway",
                CardRules::new_land(&[]).with_ability(abilities::tap_for(ManaColor::Black)),
            ),
        ],
    );

// ZNR 287 — Cragcrown Pathway // Timbercrown Pathway
pub(in crate::card::sets) static CRAGCROWN_PATHWAY_TIMBERCROWN_PATHWAY_287: CardRecord =
    CardRecord::new_mdfc(
        "Cragcrown Pathway // Timbercrown Pathway",
        "050602c0-b5b7-4076-af33-0f7a58d0b260",
        "Sam Burley",
        &[
            (
                "Cragcrown Pathway",
                CardRules::new_land(&[]).with_ability(abilities::tap_for(ManaColor::Red)),
            ),
            (
                "Timbercrown Pathway",
                CardRules::new_land(&[]).with_ability(abilities::tap_for(ManaColor::Green)),
            ),
        ],
    );

// ZNR 300 — Moraug, Fury of Akoum
// Audit: unsupported — Needs a per-creature attacks-this-turn value and a landfall trigger that schedules combat plus an opening-combat untap.
pub(in crate::card::sets) static MORAUG_FURY_OF_AKOUM_300: CardRecord = CardRecord::new(
    "Moraug, Fury of Akoum",
    "aecfbd48-7da0-4b44-b9a2-d31412f65eb1",
    "Dominik Mayer",
    crate::card::CardRules::unsupported(),
);

// ZNR 319 — Luminarch Aspirant
pub(in crate::card::sets) static LUMINARCH_ASPIRANT: CardRecord = CardRecord::new(
    "Luminarch Aspirant",
    "ebe9427d-068f-487c-9263-b40366a164bc",
    "Mads Ahm",
    // Two mana that adds a counter every turn it survives, before attackers
    // are declared -- so the counter is already on whatever is about to
    // attack or block.
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Human", "Cleric"], 1, 1).with_ability(
        AbilityDef::triggered_with_targets(
            "At the beginning of combat on your turn, put a +1/+1 counter on target creature you \
             control.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::You,
            },
            // "Target creature you control" -- including herself, which is what makes
            // an unanswered Aspirant a clock rather than a lord.
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// ZNR 335 — Thieving Skydiver (alternate printing)
const THIEVING_SKYDIVER_ALTERNATE_1: PrintingRecord = PrintingRecord::alternate(
    &THIEVING_SKYDIVER,
    1,
    "f8b3eb41-2351-4dca-becb-7ecb1fcfa14b",
    "Kieran Yanner",
);

// ZNR 336 — Agadeem's Awakening // Agadeem, the Undercrypt
// Audit: unsupported — Needs a modal DFC and a variable-card graveyard selection with pairwise distinct mana values.
pub(in crate::card::sets) static AGADEEM_S_AWAKENING_AGADEEM_THE_UNDERCRYPT_336: CardRecord =
    CardRecord::new(
        "Agadeem's Awakening // Agadeem, the Undercrypt",
        "499c2b20-e83e-40ff-919e-1d134ad50c0a",
        "Dmitry Burmak",
        crate::card::CardRules::unsupported(),
    );

// ZNR 354 — Shatterskull Smashing // Shatterskull, the Hammer Pass
// Audit: unsupported — DividedTotal accepts fixed amounts or chosen X, but not the conditional total 2X. Doubling each share from an X-sized division cannot express odd shares such as 1 and 11 when X is six.
pub(in crate::card::sets) static SHATTERSKULL_SMASHING_SHATTERSKULL_354: CardRecord =
    CardRecord::new(
        "Shatterskull Smashing // Shatterskull, the Hammer Pass",
        "243d374f-5b40-4cff-99f5-079ba873d44b",
        "Adam Paquette",
        crate::card::CardRules::unsupported(),
    );

// ZNR 374 — Forsaken Monument
// Audit: unsupported — TapEventMatcherDef can match tapping for mana, but cannot require that the event actually produced colorless mana. A source's possible mana output does not answer which type was produced.
pub(in crate::card::sets) static FORSAKEN_MONUMENT_374: CardRecord = CardRecord::new(
    "Forsaken Monument",
    "0c8f362c-f035-48b3-8e74-ef23240b44f7",
    "Piotr Dura",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ARCHON_OF_EMERIA_4,
    &DAUNTLESS_UNITY,
    &FELIDAR_RETREAT,
    &SKYCLAVE_APPARITION,
    &GLASSPOOL_MIMIC_GLASSPOOL_SHORE_60,
    &SEA_GATE_RESTORATION_SEA_GATE_REBORN_76,
    &SILUNDI_VISION_SILUNDI_ISLE_80,
    &THIEVING_SKYDIVER,
    &BLOODCHIEFS_THIRST,
    &FEED_THE_SWARM,
    &HIGHBORN_VAMPIRE,
    &MALAKIR_REBIRTH_MALAKIR_MIRE_111,
    &MARAUDING_BLIGHT_PRIEST,
    &NULLPRIEST_OF_OBLIVION,
    &RELIC_ROBBER_153,
    &ROILING_VORTEX_156,
    &SNEAKING_GUIDE_164,
    &SPIKEFIELD_HAZARD_SPIKEFIELD_CAVE_166,
    &SPITFIRE_LAGAC,
    &VALAKUT_AWAKENING_VALAKUT_STONEFORGE_174,
    &ANCIENT_GREENWARDEN,
    &ASHAYA_SOUL_OF_THE_WILD_179,
    &BROKEN_WINGS,
    &GNARLID_COLONY,
    &OMNATH_LOCUS_OF_CREATION,
    &LITHOFORM_ENGINE,
    &BRIGHTCLIMB_PATHWAY_GRIMCLIMB_PATHWAY_259,
    &CRAWLING_BARRENS,
    &NEEDLEVERGE_PATHWAY_PILLARVERGE_PATHWAY_263,
    &RIVERGLIDE_PATHWAY_LAVAGLIDE_PATHWAY_264,
    &CLEARWATER_PATHWAY_MURKWATER_PATHWAY_286,
    &CRAGCROWN_PATHWAY_TIMBERCROWN_PATHWAY_287,
    &MORAUG_FURY_OF_AKOUM_300,
    &LUMINARCH_ASPIRANT,
    &AGADEEM_S_AWAKENING_AGADEEM_THE_UNDERCRYPT_336,
    &SHATTERSKULL_SMASHING_SHATTERSKULL_354,
    &FORSAKEN_MONUMENT_374,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] =
    &[THIEVING_SKYDIVER_ALTERNATE_1];
