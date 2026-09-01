//! Zendikar Rising cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AlternativeCastKindDef, CardArt, CardRules, CardSet, CardSupertype, CardType, ComparisonDef,
    ControlDurationDef, CounterKind, EffectDef, EffectRecipientDef, ManaColor, ObjectPredicateDef,
    ObjectQueryDef, ObjectRefDef, ObjectSetDef, PlayerRefDef, PlayerRelation, TokenStatsDef,
    TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, abilities,
};
use crate::{ParentBinding, TargetIndex, mana_cost};

// ZNR 9 — Dauntless Unity
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DAUNTLESS_UNITY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b12a4d17-68e6-4133-99fd-e501e24e6c6b"),
    "Dauntless Unity",
    crate::card::CardArt::new("b12a4d17-68e6-4133-99fd-e501e24e6c6b", "Josu Hernaiz"),
    crate::card::CardSet::ZendikarRising,
    crate::card::CardRules::unsupported(),
);

// ZNR 39 — Skyclave Apparition
pub(in crate::card::sets) static SKYCLAVE_APPARITION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b83cfbaa-7890-4f6f-878b-4edb45677371"),
    "Skyclave Apparition",
    crate::card::CardArt::new("b83cfbaa-7890-4f6f-878b-4edb45677371", "Donato Giancola"),
    crate::card::CardSet::ZendikarRising,
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
                        effect: &EffectDef::create_creature_token(&["Illusion"], &[ManaColor::Blue], 0, 0)
                                // "Where X is the mana value of the exiled card": both halves read the same
                                // card, which is the one the leave trigger just bound.
                                .with_variable_token_stats(&TokenStatsDef {
                                    power: ValueDef::ObjectManaValue(ObjectRefDef::Binding(ParentBinding)),
                                    toughness: ValueDef::ObjectManaValue(ObjectRefDef::Binding(ParentBinding)),
                                })
                                .with_controller(PlayerRefDef::OwnerOf(ObjectRefDef::Binding(
                                    ParentBinding,
                                ))),
                    },
                ),
            ),
        ]),
);

// ZNR 85 — Thieving Skydiver
pub(in crate::card::sets) static THIEVING_SKYDIVER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ff84ea71-e477-44f7-a3f8-77fef708efeb"),
    "Thieving Skydiver",
    CardArt::new("ff84ea71-e477-44f7-a3f8-77fef708efeb", "Kieran Yanner"),
    CardSet::ZendikarRising,
    // Two mana for a flier, or two plus X for a flier that takes the best
    // artifact on the board with it -- a Mox on turn three, a Sword on turn
    // five, and the Sword comes down already attached.
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Merfolk", "Rogue"], 2, 1).with_abilities(&[
        AbilityDef::alternative_cast(
            mana_cost!("{X}{1}{U}"),
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
                EffectDef::GainControl {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    controller: PlayerRefDef::EffectController,
                    duration: ControlDurationDef::Indefinitely,
                },
                EffectDef::IfCondition {
                    condition: &TriggerConditionDef::TargetMatches {
                        slot: TargetIndex::PRIMARY,
                        object: ObjectPredicateDef::Subtype("Equipment"),
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
pub(in crate::card::sets) static BLOODCHIEFS_THIRST: CardRecord = CardRecord::new_with_legacy_id(
    2165,
    "Bloodchief's Thirst",
    CardArt::new("059e8447-6b1c-4651-a734-a8fea2cbf7b2", "Jason Rainville"),
    CardSet::ZendikarRising,
    // One black kills most of what an aggressive deck leads with; four kills
    // whatever is left, which is why the card is played over a cheaper
    // removal spell that can only do the first job.
    CardRules::new_sorcery(mana_cost!("{B}")).with_abilities(&[
        abilities::kicker(mana_cost!("{2}{B}")),
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
                can_regenerate: true,
                then: None,
            },
        ),
    ]),
);

// ZNR 185 — Gnarlid Colony
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GNARLID_COLONY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7327289d-eed8-44b1-8495-7172e2b49d5f"),
    "Gnarlid Colony",
    crate::card::CardArt::new("7327289d-eed8-44b1-8495-7172e2b49d5f", "Izzy"),
    crate::card::CardSet::ZendikarRising,
    crate::card::CardRules::unsupported(),
);

// ZNR 232 — Omnath, Locus of Creation
const fn omnath_resolution(amount: u8) -> TriggerConditionDef {
    TriggerConditionDef::SourceResolutionsThisTurn {
        comparison: ComparisonDef::Equal,
        amount,
    }
}

pub(in crate::card::sets) static OMNATH_LOCUS_OF_CREATION: CardRecord =
    CardRecord::new_with_legacy_id(
        2264,
        "Omnath, Locus of Creation",
        CardArt::new("4e4fb50c-a81f-44d3-93c5-fa9a0b37f617", "Chris Rahn"),
        CardSet::ZendikarRising,
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
                                EffectDef::DealDamage {
                                    recipient: EffectRecipientDef::Opponent,
                                    amount: ValueDef::Constant(4),
                                },
                                EffectDef::DealDamage {
                                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                                        ObjectQueryDef::matching(
                                            ObjectPredicateDef::HasType(CardType::Planeswalker),
                                            &[ZoneKind::Battlefield],
                                            PlayerRelation::NotYou,
                                        ),
                                    )),
                                    amount: ValueDef::Constant(4),
                                },
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
    PrintingAnchor::scryfall("6683416a-5820-4cd0-b28a-60a53239e9ef"),
    "Lithoform Engine",
    CardArt::new("6683416a-5820-4cd0-b28a-60a53239e9ef", "Colin Boyer"),
    CardSet::ZendikarRising,
    CardRules::new_artifact(mana_cost!("{4}"))
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated_with_targets(
                "{2}, {T}: Copy target activated or triggered ability you control. You may choose new targets for the copy.",
                &[
                    AbilityCostDef::Mana(mana_cost!("{2}")),
                    AbilityCostDef::TapSource,
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
                    AbilityCostDef::Mana(mana_cost!("{3}")),
                    AbilityCostDef::TapSource,
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
                    AbilityCostDef::Mana(mana_cost!("{4}")),
                    AbilityCostDef::TapSource,
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

// ZNR 319 — Luminarch Aspirant
pub(in crate::card::sets) static LUMINARCH_ASPIRANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("ebe9427d-068f-487c-9263-b40366a164bc"),
    "Luminarch Aspirant",
    CardArt::new("ebe9427d-068f-487c-9263-b40366a164bc", "Mads Ahm"),
    CardSet::ZendikarRising,
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

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &DAUNTLESS_UNITY,
    &SKYCLAVE_APPARITION,
    &THIEVING_SKYDIVER,
    &BLOODCHIEFS_THIRST,
    &GNARLID_COLONY,
    &OMNATH_LOCUS_OF_CREATION,
    &LITHOFORM_ENGINE,
    &LUMINARCH_ASPIRANT,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::alternate(&THIEVING_SKYDIVER, 1), // ZNR 335
];
