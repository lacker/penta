//! Edge of Eternities cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AlternativeCastKindDef, AppliedEffectDef,
    AppliedRuleDef, CardArt, CardRules, CardSet, CardSupertype, CardType, CardTypeSet,
    ChoiceVisibilityDef, ChooseDef, ComparisonDef, CostDef, CounterKind, CreatureTypeSetDef,
    DamageEventMatcherDef, DamageRecipientMatcherDef, DeclarativeAbilityDef, EffectDef,
    EffectRecipientDef, EmblemCharacteristics, GraveyardPlayPermissionDef, HalvedValueDef,
    ManaColor, ModalSpellDef, MoveObjectsDef, ObjectChoiceBindingDef, ObjectPredicateDef,
    ObjectQueryDef, ObjectSetDef, PlayActionMatcherDef, PlayRestrictionDef, PlayerRefDef,
    PlayerRelation, QuantifierDef, RandomizeObjectOrderDef, ReplacementAbilityDef,
    ReplacementConditionDef, ReplacementEffectDef, ReplacementEventDef, ResolvedEffectDurationDef,
    RoundingDef, SumValueDef, TriggerConditionDef, TriggerEventDef, TriggeredAbilityDef,
    TurnStepDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::{ParentBinding, TargetIndex, mana_cost};

// EOE 2 — Tezzeret, Cruel Captain
static AN_ARTIFACT_YOU_CONTROL: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::HasType(CardType::Artifact),
    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
]);

pub(in crate::card::sets) static TEZZERET_CRUEL_CAPTAIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("02e8e540-8aa3-4e6a-9a11-c3949cab5f0f"),
    "Tezzeret, Cruel Captain",
    CardArt::new("02e8e540-8aa3-4e6a-9a11-c3949cab5f0f", "Chris Rahn"),
    CardSet::EdgeOfEternities,
    // Three colourless for a planeswalker that an artifact deck keeps
    // topping up, and whose zero is free every turn.
    CardRules::new_planeswalker(mana_cost!("{3}"), &["Tezzeret"], 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever an artifact you control enters, put a loyalty counter on Tezzeret.",
                TriggerEventDef::zone_changed(AN_ARTIFACT_YOU_CONTROL, None, Some(ZoneKind::Battlefield)),
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::Loyalty,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::activated_with_targets(
                "0: Untap target artifact or creature. If it\'s an artifact creature, put a +1/+1 counter \
                 on it.",
                &[CostDef::Loyalty(0)],
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                )],
                EffectDef::Sequence(&[
                    EffectDef::Untap {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                    EffectDef::IfCondition {
                        // The rider is asked of the target as the ability resolves, so an artifact
                        // animated in response is a legal thing to grow.
                        condition: &TriggerConditionDef::TargetMatches {
                                slot: TargetIndex::PRIMARY,
                                object: ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::HasType(CardType::Artifact),
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                ]),
                            },
                        then: &EffectDef::AddCounters {
                            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::Constant(1),
                        },
                    },
                ]),
            ),
            AbilityDef::activated(
                "−3: Search your library for an artifact card with mana value 1 or less, reveal it, put \
                 it into your hand, then shuffle.",
                &[CostDef::Loyalty(-3)],
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    // A one-mana artifact, which is what the deck this is in is made of.
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::ManaValueAtMost(1),
                    ]),
                    minimum: 0,
                    maximum: ValueDef::Constant(1),
                    reveal: true,
                    destination: ZoneKind::Hand,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: false,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            ),
            AbilityDef::activated(
                "−7: You get an emblem with \"At the beginning of combat on your turn, put three +1/+1 \
                 counters on target artifact you control. If it\'s not a creature, it becomes a 0/0 Robot \
                 artifact creature.\"",
                &[CostDef::Loyalty(-7)],
                EffectDef::CreateEmblem {
                    emblem: EmblemCharacteristics::new("Tezzeret, Cruel Captain emblem", &[AbilityDef::triggered_with_targets(
                            "At the beginning of combat on your turn, put three +1/+1 counters on target artifact you \
                             control. If it's not a creature, it becomes a 0/0 Robot artifact creature.",
                            TriggerEventDef::StepBegins {
                                step: TurnStepDef::BeginningOfCombat,
                                player: PlayerRelation::You,
                            },
                            &[AbilityTargetDef::exactly_one_permanent(
                                    AN_ARTIFACT_YOU_CONTROL,
                                )],
                            EffectDef::Sequence(&[
                                EffectDef::AddCounters {
                                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                    kind: CounterKind::PlusOnePlusOne,
                                    amount: ValueDef::Constant(3),
                                },
                                EffectDef::IfCondition {
                                    condition: &TriggerConditionDef::TargetMatches {
                                            slot: TargetIndex::PRIMARY,
                                            object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                                        },
                                    then: &EffectDef::Apply {
                                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                                        // "If it's not a creature, it becomes a 0/0 Robot artifact creature." The
                                        // counters go on first, so an artifact that was not a creature ends up a
                                        // 3/3: the base is what changes, and the counters sit on top of it.
                                        effect: AppliedEffectDef::Composite(&[
                                            AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                                            AppliedEffectDef::set_creature_types(CreatureTypeSetDef::named(&["Robot"])),
                                            AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(0), ValueDef::Constant(0)),
                                        ]),
                                        duration: ResolvedEffectDurationDef::Permanent,
                                    },
                                },
                            ]),
                        )]),
                },
            ),
        ]),
);

// EOE 9 — Cosmogrand Zenith
pub(in crate::card::sets) static COSMOGRAND_ZENITH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b3c1e5e3-4e6b-456a-958c-7a75c38f8183"),
    "Cosmogrand Zenith",
    CardArt::new("b3c1e5e3-4e6b-456a-958c-7a75c38f8183", "Anna Steinbauer"),
    CardSet::EdgeOfEternities,
    // Three mana for a 2/4 that pays a second time every turn the hand has
    // two spells in it, and the choice is between going wider and going
    // taller.
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Soldier"], 2, 4)
        .with_abilities(&[AbilityDef::defined(
            "Whenever you cast your second spell each turn, choose one —\n• Create two 1/1 white Human \
             Soldier creature tokens.\n• Put a +1/+1 counter on each creature you control.",
            DeclarativeAbilityDef::Triggered(
                TriggeredAbilityDef::new(TriggerEventDef::spell_cast(
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ))
                // Exactly the second, not the second or later: the spell that caused the
                // trigger has already been counted by the time this is read.
                .with_condition(&TriggerConditionDef::SpellsCastThisTurn {
                    quantifier: QuantifierDef::Any,
                    player: PlayerRelation::You,
                    comparison: ComparisonDef::Equal,
                    amount: 2,
                })
                .with_modes(ModalSpellDef::choose_one(&[
                    AbilityDef::spell(
                        "Create two 1/1 white Human Soldier creature tokens.",
                        EffectDef::create_creature_token(&["Human", "Soldier"], &[ManaColor::White], 1, 1)
                            .with_count(ValueDef::Constant(2)),
                    ),
                    // Each creature you control as the trigger resolves, which includes the
                    // tokens the other mode would have made and the Zenith itself.
                    AbilityDef::spell(
                        "Put a +1/+1 counter on each creature you control.",
                        EffectDef::AddCounters {
                            object: EffectRecipientDef::matching_objects(
                                ObjectPredicateDef::HasType(CardType::Creature),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            ),
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::Constant(1),
                        },
                    ),
                ])),
            ),
            EffectDef::None,
        )]),
);

// EOE 18 — Focus Fire
pub(in crate::card::sets) static FOCUS_FIRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a9ddfcbc-0f84-4315-aaa3-ca54ff64d7de"),
    "Focus Fire",
    CardArt::new("a9ddfcbc-0f84-4315-aaa3-ca54ff64d7de", "Borja Pindado"),
    CardSet::EdgeOfEternities,
    // The floor is already two damage in combat, and a board counts twice:
    // each body both survives the trade and raises what this kills.
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell_with_targets(
        "Focus Fire deals X damage to target attacking or blocking creature, where X is 2 plus \
         the number of creatures and/or Spacecraft you control.",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::AttackingOrBlocking,
            ]),
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Sum(&SumValueDef::new(
                ValueDef::Constant(2),
                ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    // A Spacecraft that has stationed up is already a
                    // creature, so the two halves overlap and the query has
                    // to match each permanent once rather than twice.
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype("Spacecraft"),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
            )),
        },
    )),
);

// EOE 51 — Consult the Star Charts
/// "Where X is the number of lands you control", which is the whole reason
/// the card is playable: it looks at more the longer the game goes.
static LANDS_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Land),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

/// One selection differs from the other only in how many it keeps, so the
/// two are the same workflow twice rather than a count the spell could carry.
macro_rules! consult_choice {
    ($cards:expr, $chosen:expr, $rest:expr) => {
        EffectDef::Choose(ChooseDef {
            binding: ObjectChoiceBindingDef::Objects($chosen),
            unchosen: Some($rest),
            chooser: PlayerRefDef::EffectController,
            candidates: ObjectSetDef::Binding(ParentBinding),
            exclude: None,
            minimum: $cards,
            maximum: $cards,
            visibility: ChoiceVisibilityDef::Private,
            then: &const {
                EffectDef::Sequence(&[
                    EffectDef::MoveObjects(MoveObjectsDef {
                        input: ObjectSetDef::Binding($chosen),
                        from: Some(ZoneKind::Library),
                        zone: ZoneKind::Hand,
                        placement: ZonePlacement::Top,
                        moved: None,
                        then: &EffectDef::None,
                    }),
                    EffectDef::RandomizeObjectOrder(RandomizeObjectOrderDef {
                        input: ObjectSetDef::Binding($rest),
                        randomized: ParentBinding,
                        then: &const {
                            EffectDef::MoveObjects(MoveObjectsDef {
                                input: ObjectSetDef::Binding(ParentBinding),
                                from: Some(ZoneKind::Library),
                                zone: ZoneKind::Library,
                                placement: ZonePlacement::Bottom,
                                moved: None,
                                then: &EffectDef::None,
                            })
                        },
                    }),
                ])
            },
        })
    };
}

pub(in crate::card::sets) static CONSULT_THE_STAR_CHARTS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a16a6555-2e3a-4587-aacd-0307d696b26c"),
    "Consult the Star Charts",
    CardArt::new(
        "a16a6555-2e3a-4587-aacd-0307d696b26c",
        "Antonio José Manzanedo",
    ),
    CardSet::EdgeOfEternities,
    // Two mana to dig as deep as your mana base, and four to keep twice as
    // much of what it finds.
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[
        AbilityDef::alternative_cast(
            mana_cost!("{2}{U}{U}"),
            AlternativeCastKindDef::Kicked,
            Some("Kicker {1}{U} (You may pay an additional {1}{U} as you cast this spell.)"),
            EffectDef::None,
        ),
        AbilityDef::spell(
            "Look at the top X cards of your library, where X is the number of lands you \
             control. Put one of those cards into your hand. If this spell was kicked, put two \
             of those cards into your hand instead. Put the rest on the bottom of your library \
             in a random order.",
            EffectDef::IfElseCondition {
                condition: &TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Kicked),
                then: &abilities::bind_top_cards_then(
                    PlayerRefDef::EffectController,
                    ValueDef::CountMatchingObjects(&LANDS_YOU_CONTROL),
                    &consult_choice!(
                        2,
                        Binding!("consult_kicked_chosen"),
                        Binding!("consult_kicked_rest")
                    ),
                ),
                otherwise: &abilities::bind_top_cards_then(
                    PlayerRefDef::EffectController,
                    ValueDef::CountMatchingObjects(&LANDS_YOU_CONTROL),
                    &consult_choice!(
                        1,
                        Binding!("consult_normal_chosen"),
                        Binding!("consult_normal_rest")
                    ),
                ),
            },
        ),
    ]),
);

// EOE 52 — Cryogen Relic
pub(in crate::card::sets) static CRYOGEN_RELIC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7bfb33b6-e2bf-498f-8c58-ae21a840cf75"),
    "Cryogen Relic",
    CardArt::new("7bfb33b6-e2bf-498f-8c58-ae21a840cf75", "Eelis Kyttanen"),
    CardSet::EdgeOfEternities,
    // Sacrificing it draws the second card, so the tap-down costs nothing
    // in cards -- only the two mana and the artifact itself.
    CardRules::new_artifact(mana_cost!("{1}{U}")).with_abilities(&[
        AbilityDef::triggered(
            "When this artifact enters or leaves the battlefield, draw a card.",
            // One printed sentence with two ways in, so it is one ability
            // watching both zone changes.
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    None,
                ),
            ]),
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::activated_with_targets(
            "{1}{U}, Sacrifice this artifact: Put a stun counter on up to one target tapped creature.",
            &[
                CostDef::Mana(mana_cost!("{1}{U}")),
                CostDef::SacrificeSource,
            ],
            // "Up to one", so it can be sacrificed purely for the leave
            // trigger's card when nothing is tapped.
            &[AbilityTargetDef::up_to(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Tapped,
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
                1,
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::Stun,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// EOE 53 — Cryoshatter
pub(in crate::card::sets) static CRYOSHATTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7b62b1e2-9e43-4a66-a647-7e5de2871f2a"),
    "Cryoshatter",
    CardArt::new("7b62b1e2-9e43-4a66-a647-7e5de2871f2a", "Jeremy Wilson"),
    CardSet::EdgeOfEternities,
    // One mana blanks the creature immediately and kills it the moment it
    // is used for anything, which is what makes the -5/-0 half enough.
    CardRules::new_enchantment(mana_cost!("{U}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets -5/-0.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(-5),
                        ValueDef::Constant(0),
                    ),
                },
            ),
            AbilityDef::triggered(
                "When enchanted creature becomes tapped or is dealt damage, destroy it.",
                // Two ways into one printed sentence, both read against the
                // creature this Aura is on rather than the Aura itself.
                TriggerEventDef::AnyOf(&[
                    TriggerEventDef::tapped(ObjectPredicateDef::AttachedToSource),
                    TriggerEventDef::DamageDealt(DamageEventMatcherDef {
                        recipient: DamageRecipientMatcherDef::Recipients(
                            EffectRecipientDef::AttachedPermanent,
                        ),
                        ..DamageEventMatcherDef::ANY
                    }),
                ]),
                EffectDef::Destroy {
                    object: EffectRecipientDef::AttachedPermanent,
                    then: None,
                },
            ),
        ]),
);

// EOE 66 — Mechanozoa
pub(in crate::card::sets) static MECHANOZOA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0cb8d8ce-329a-4a97-b3d8-796703ebcb37"),
    "Mechanozoa",
    CardArt::new("0cb8d8ce-329a-4a97-b3d8-796703ebcb37", "Daarken"),
    CardSet::EdgeOfEternities,
    // Warped on three it is a Frost Lynx that comes back; cast on six it is
    // a 5/7 that does the same thing again. Either way the stun counter is
    // what buys the tempo.
    CardRules::new_artifact_creature(mana_cost!("{4}{U}{U}"), &["Robot", "Jellyfish"], 5, 7)
        .with_abilities(&[
            abilities::enters_trigger_with_targets(
                "When this creature enters, tap target artifact or creature an opponent controls \
                 and put a stun counter on it. (If a permanent with a stun counter would become \
                 untapped, remove one from it instead.)",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Creature),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::Opponent),
                        owner: None,
                    },
                )],
                // Tapping first and stunning second is the printed order, and
                // it matters: a permanent that was already tapped still takes
                // the counter, so the untap it misses is the next one.
                EffectDef::Sequence(&[
                    EffectDef::Tap {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    },
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        kind: CounterKind::Stun,
                        amount: ValueDef::Constant(1),
                    },
                ]),
            ),
            abilities::warp(
                mana_cost!("{2}{U}"),
                "Warp {2}{U} (You may cast this card from your hand for its warp cost. Exile it \
                 at the beginning of the next end step, then you may cast it from exile on a \
                 later turn.)",
            ),
            abilities::warped_exile(),
        ]),
);

// EOE 72 — Quantum Riddler
pub(in crate::card::sets) static QUANTUM_RIDDLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("120be808-ff3b-4fca-96a1-4db6b9825856"),
    "Quantum Riddler",
    CardArt::new("120be808-ff3b-4fca-96a1-4db6b9825856", "Izzy"),
    CardSet::EdgeOfEternities,
    // Five mana for a 4/6 flier that draws a card, or two mana for the same
    // body until the end of turn and the card it comes back with later.
    CardRules::new_creature(mana_cost!("{3}{U}{U}"), &["Sphinx"], 4, 6).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, draw a card.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::defined_replacement(
            "As long as you have one or fewer cards in hand, if you would draw one or more \
             cards, you draw that many cards plus one instead.",
            // "As long as you have one or fewer cards in hand, if you would draw one
            // or more cards, you draw that many cards plus one instead." One
            // replacement of the whole instruction: a draw of three becomes a draw of
            // four rather than a draw of six.
            ReplacementAbilityDef::new()
                .with_event(ReplacementEventDef::WouldDraw {
                    player: PlayerRelation::You,
                    during_own_draw_step: false,
                    except_first_in_draw_step: false,
                })
                .with_condition(ReplacementConditionDef::ControllerHandAtMost(1)),
            ReplacementEffectDef::AddToEventAmount(1),
        ),
        abilities::warp(
            mana_cost!("{1}{U}"),
            "Warp {1}{U} (You may cast this card from your hand for its warp cost. Exile it at \
             the beginning of the next end step, then you may cast it from exile on a later \
             turn.)",
        ),
        abilities::warped_exile(),
    ]),
);

// EOE 77 — Starbreach Whale
pub(in crate::card::sets) static STARBREACH_WHALE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8a1a0476-7145-4493-97e5-4fc05c85e476"),
    "Starbreach Whale",
    CardArt::new("8a1a0476-7145-4493-97e5-4fc05c85e476", "Sam Burley"),
    CardSet::EdgeOfEternities,
    // Warp buys the surveil on turn two and the 3/5 flier later, off one
    // card. The entry trigger is what makes the early half worth a turn:
    // it fires both times.
    CardRules::new_creature(mana_cost!("{4}{U}"), &["Whale"], 3, 5).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, surveil 2. (Look at the top two cards of your library, \
             then put any number of them into your graveyard and the rest on top of your library \
             in any order.)",
            abilities::surveil(ValueDef::Constant(2)),
        ),
        abilities::warp(
            mana_cost!("{1}{U}"),
            "Warp {1}{U} (You may cast this card from your hand for its warp cost. Exile it at \
             the beginning of the next end step, then you may cast it from exile on a later \
             turn.)",
        ),
        abilities::warped_exile(),
    ]),
);

// EOE 152 — Plasma Bolt
// Audit: unsupported — Needs void. Neither half of its condition is available: ControllerHadPermanentLeaveThisTurn is per-player and does not exclude lands, and nothing records that a spell was warped this turn.
pub(in crate::card::sets) static PLASMA_BOLT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a1a1834b-76c2-4496-b8c5-18b69ab34c4c"),
    "Plasma Bolt",
    crate::card::CardArt::new("a1a1834b-76c2-4496-b8c5-18b69ab34c4c", "Viko Menezes"),
    crate::card::CardSet::EdgeOfEternities,
    crate::card::CardRules::unsupported(),
);

// EOE 201 — Ouroboroid
pub(in crate::card::sets) static OUROBOROID: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("209c591a-4ab2-4e89-9523-a7b766cf4e51"),
    "Ouroboroid",
    CardArt::new("209c591a-4ab2-4e89-9523-a7b766cf4e51", "Samuel Perin"),
    CardSet::EdgeOfEternities,
    // A 1/3 that doubles itself every combat and takes the rest of the board
    // with it: one counter each the first turn, two the next, four after
    // that.
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Plant", "Wurm"], 1, 3).with_ability(
        AbilityDef::triggered(
            "At the beginning of combat on your turn, put X +1/+1 counters on each creature you \
             control, where X is this creature's power.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::You,
            },
            // X is read once, as the ability resolves, and every creature
            // gets that many -- including the Wurm, whose own growth does
            // not raise the number partway through.
            EffectDef::AddCounters {
                // "Each creature you control" includes the Wurm itself, so the counters it
                // hands out make the next round of them bigger.
                object: EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ))),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::SourcePower,
            },
        ),
    ),
);

// EOE 244 — Pinnacle Kill-Ship
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static PINNACLE_KILL_SHIP: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("bf784de8-5ae2-4c07-92bb-a5b7f593b773"),
    "Pinnacle Kill-Ship",
    crate::card::CardArt::new("bf784de8-5ae2-4c07-92bb-a5b7f593b773", "Alexandre Honoré"),
    crate::card::CardSet::EdgeOfEternities,
    crate::card::CardRules::unsupported(),
);

// EOE 297 — Mightform Harmonizer
pub(in crate::card::sets) static MIGHTFORM_HARMONIZER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("29bc9be4-4fc3-440a-a851-0c7f8989c9b5"),
    "Mightform Harmonizer",
    CardArt::new("29bc9be4-4fc3-440a-a851-0c7f8989c9b5", "Jessica Fong"),
    CardSet::EdgeOfEternities,
    // Four mana for a 4/4 that makes every land drop a pump spell, or three
    // for one turn of it now and the whole card again later.
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Insect", "Druid"], 4, 4)
        .with_abilities(&[
            AbilityDef::triggered_with_targets(
                "Landfall — Whenever a land you control enters, double the power of target creature you \
                 control until end of turn.",
                TriggerEventDef::zone_changed(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]), None, Some(ZoneKind::Battlefield)),
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                // Doubling is +X/+0 where X is the target's power as the trigger resolves,
                // so two landfalls in a turn compound: the second reads the size the first
                // left behind, and a creature answered in between doubles nothing.
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::TargetPower(TargetIndex::PRIMARY),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            abilities::warp(
                mana_cost!("{2}{G}"),
                "Warp {2}{G} (You may cast this card from your hand for its warp cost. Exile this \
                 creature at the beginning of the next end step, then you may cast it from exile on a \
                 later turn.)",
            ),
            abilities::warped_exile(),
        ]),
);

// EOE 362 — Icetill Explorer
pub(in crate::card::sets) static ICETILL_EXPLORER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("895e5e9b-84dd-4741-8a2c-442165ea9b15"),
    "Icetill Explorer",
    CardArt::new("895e5e9b-84dd-4741-8a2c-442165ea9b15", "Raimaru"),
    CardSet::EdgeOfEternities,
    // Four mana for a 2/4 whose three clauses feed each other: the extra
    // land drop wants lands, the mill finds them, and the graveyard is
    // where the mill puts them.
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Insect", "Scout"], 2, 4).with_abilities(&[
        AbilityDef::static_ability(
            "You may play an additional land on each of your turns.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MayPlayAdditionalLands(1)),
            },
        ),
        AbilityDef::static_ability(
            "You may play lands from your graveyard.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MayPlayFromGraveyard(
                    // Lands only, played the ordinary way: what the permission adds is the
                    // zone, not a way of casting anything out of it.
                    GraveyardPlayPermissionDef::unlimited(PlayRestrictionDef::new(
                        PlayActionMatcherDef::PlayLand,
                        ObjectPredicateDef::HasType(CardType::Land),
                    )),
                )),
            },
        ),
        AbilityDef::triggered(
            "Landfall — Whenever a land you control enters, mill a card.",
            TriggerEventDef::zone_changed(
                // A land you control arriving, which is what landfall is: a land somebody
                // else plays is not one, and the mill is what turns the extra land drop
                // into more lands to play.
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::Mill {
                player: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// EOE 391 — The Endstone
pub(in crate::card::sets) static THE_ENDSTONE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1227eb7f-c2a5-4112-98d0-70275a63c26a"),
    "The Endstone",
    CardArt::new("1227eb7f-c2a5-4112-98d0-70275a63c26a", "Hidetaka Tenjin"),
    CardSet::EdgeOfEternities,
    // Seven mana that draws a card for everything you do and hands the ten
    // life back every end step, which is what makes the seven payable.
    CardRules::new_artifact(mana_cost!("{7}"))
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever you play a land or cast a spell, draw a card.",
                // One ability with two events rather than two abilities: the card prints
                // one, and a turn with a land and a spell in it draws twice either way.
                TriggerEventDef::AnyOf(&[
                    TriggerEventDef::LandPlayed {
                        land: ObjectPredicateDef::Any,
                        player: PlayerRelation::You,
                    },
                    TriggerEventDef::spell_cast(ObjectPredicateDef::ControlledBy(PlayerRelation::You)),
                ]),
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::triggered(
                "At the beginning of your end step, your life total becomes half your starting life \
                 total, rounded up.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::End,
                    player: PlayerRelation::You,
                },
                EffectDef::SetLifeTotal {
                    recipient: EffectRecipientDef::Controller,
                    // Half of what the game began on rather than half of what is left: it sets
                    // the total to the same number every end step, which is a gain from below
                    // it and a loss from above.
                    total: ValueDef::Halved(&HalvedValueDef::new(
                        ValueDef::StartingLifeTotal,
                        RoundingDef::Up,
                    )),
                },
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &TEZZERET_CRUEL_CAPTAIN,
    &COSMOGRAND_ZENITH,
    &FOCUS_FIRE,
    &CONSULT_THE_STAR_CHARTS,
    &CRYOGEN_RELIC,
    &CRYOSHATTER,
    &MECHANOZOA,
    &QUANTUM_RIDDLER,
    &STARBREACH_WHALE,
    &PLASMA_BOLT,
    &OUROBOROID,
    &PINNACLE_KILL_SHIP,
    &MIGHTFORM_HARMONIZER,
    &ICETILL_EXPLORER,
    &THE_ENDSTONE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
