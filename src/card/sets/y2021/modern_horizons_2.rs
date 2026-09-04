//! Modern Horizons 2 cards cataloged as cross-format rules-engine test cases.

use super::{CardRecord, PrintingRecord};
use crate::card::CostQuantityDef;
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AlternativeCastKindDef, AppliedEffectDef, BasicLandType, BattlefieldEntryModificationDef,
    CardArt, CardRules, CardSet, CardSupertype, CardType, CardTypeSet, CharacteristicOperationDef,
    ChoiceVisibilityDef, ChooseDef, ComparisonDef, CounterKind, DamageEventMatcherDef,
    DamageKindDef, DamageRecipientMatcherDef, DamageSourceMatcherDef, DiscardFollowUpDef,
    DiscardSelectionDef, DividedTotal, EffectDef, EffectPaymentCostDef, EffectPaymentDef,
    EffectRecipientDef, ExilePlayDurationDef, FreePlayDef, FreePlayDurationDef,
    GraveyardTypeConditionDef, ManaColor, MillLoopDef, ObjectChoiceBindingDef, ObjectPredicateDef,
    ObjectQueryDef, ObjectRefDef, ObjectSetDef, ObjectSetFilterDef, PayOrDef, PlayerRefDef,
    PlayerRelation, PlayerSetDef, PowerToughnessOperationDef, ReplacementEffectDef,
    ReplacementEventDef, ResolvedEffectDurationDef, SacrificedAmountDef, SetOperationDef,
    SpellAdditionalCostDef, TargetChooserDef, TriggerConditionDef, TriggerEventDef,
    ValueComparisonDef, ValueDef, ZoneKind, ZonePlacement, abilities, tokens,
};
use crate::{AdditionalCostIndex, ParentBinding, TargetIndex, mana_cost};

// MH2 25 — Prismatic Ending
pub(in crate::card::sets) static PRISMATIC_ENDING: CardRecord = CardRecord::new(
    CardSet::ModernHorizons2,
    "Prismatic Ending",
    "825969b9-3c70-4fca-8cab-696e9ca7cdb2",
    "John Stanko",
    // X buys nothing by itself: it is a sink for the extra colours, and how
    // many different ones went in is the only thing the spell reads.
    CardRules::new_sorcery(mana_cost!("{X}{W}")).with_ability(AbilityDef::spell_with_targets(
            "Converge — Exile target nonland permanent if its mana value is less than or equal to the number of colors of mana spent to cast this spell.",
            // A nonland permanent of any size may be targeted; whether it is actually
            // exiled is settled on resolution, against what paid for the spell.
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
            )],
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::TargetMatches {
                    slot: TargetIndex::PRIMARY,
                    object: ObjectPredicateDef::ManaValueAtMostValue(ValueDef::ColorsOfManaSpent),
                },
                then: &EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Exile,
                    placement: ZonePlacement::Top,
                },
            },
        )),
);

// MH2 32 — Solitude
pub(in crate::card::sets) static SOLITUDE: CardRecord = CardRecord::new(
    CardSet::ModernHorizons2,
    "Solitude",
    "47a6234f-309f-4e03-9263-66da48b57153",
    "Evan Shipard",
    // Two white cards for a free Swords to Plowshares at instant speed, and
    // a lifelinking 3/2 on the turns five mana is available instead.
    CardRules::new_creature(mana_cost!("{3}{W}{W}"), &["Elemental", "Incarnation"], 3, 2)
        .with_abilities(&[
            abilities::flash(),
            abilities::lifelink(),
            abilities::enters_trigger_with_targets(
                "When this creature enters, exile up to one other target creature. That creature's \
                 controller gains life equal to its power.",
                // "Up to one other": declining is a legal choice, and Solitude herself is
                // never one of the options.
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    1,
                )],
                // Swords to Plowshares' pair, in the same order: the power the life is
                // read from is the one the creature had as it left the battlefield.
                EffectDef::Sequence(&[
                    EffectDef::MoveToZone {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        zone: ZoneKind::Exile,
                        placement: ZonePlacement::Top,
                    },
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::ControllerOfTarget(TargetIndex::PRIMARY),
                        amount: ValueDef::TargetPower(TargetIndex::PRIMARY),
                    },
                ]),
            ),
            AbilityDef::alternative_cast(
                mana_cost!("{0}"),
                AlternativeCastKindDef::AlternativeCost,
                Some("Evoke—Exile a white card from your hand."),
                EffectDef::None,
            )
            .with_alternative_additional_cost(&SpellAdditionalCostDef::exile(
                ObjectPredicateDef::Color(ManaColor::White),
                ZoneKind::Hand,
                CostQuantityDef::Fixed(1),
            )),
            abilities::evoke_sacrifice(
                "When this creature enters, if it was evoked, sacrifice it.",
            ),
        ]),
);

// MH2 36 — Unbounded Potential
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static UNBOUNDED_POTENTIAL: CardRecord = CardRecord::new(
    crate::card::CardSet::ModernHorizons2,
    "Unbounded Potential",
    "9955a344-dcd8-404d-9757-f62ed158ba22",
    "Iain McCaig",
    crate::card::CardRules::unsupported(),
);

// MH2 46 — Hard Evidence
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static HARD_EVIDENCE: CardRecord = CardRecord::new(
    crate::card::CardSet::ModernHorizons2,
    "Hard Evidence",
    "501599d6-1072-4124-b05d-01f96de153f3",
    "Yeong-Hao Han",
    crate::card::CardRules::unsupported(),
);

// MH2 49 — Lose Focus
pub(in crate::card::sets) static LOSE_FOCUS: CardRecord = CardRecord::new(
    CardSet::ModernHorizons2,
    "Lose Focus",
    "985bdb0c-ce6c-4506-8163-76f3b2fdf5fb",
    "Martina Fačková",
    // A soft counter that stops being soft once there is spare mana: each
    // replicate is another {2} the other player has to find.
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[
        abilities::replicate(mana_cost!("{U}")),
        AbilityDef::spell_with_targets(
            "Counter target spell unless its controller pays {2}.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Spell,
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                },
            )],
            abilities::counter_target_unless_paid(ValueDef::Constant(2)),
        ),
        AbilityDef::triggered(
            "Replicate {U} (When you cast this spell, copy it for each time you paid its \
             replicate cost. You may choose new targets for the copies.)",
            TriggerEventDef::spell_cast(ObjectPredicateDef::Source),
            // The copies are a cast trigger rather than part of the spell's own clause,
            // exactly as storm is: what differs is only where the count comes from, and
            // replicate counts what was paid rather than what was cast.
            EffectDef::CopyStackObject(&crate::card::CopyStackObjectDef {
                object: EffectRecipientDef::Source,
                controller: PlayerRefDef::EffectController,
                count: ValueDef::AdditionalCostPayments(AdditionalCostIndex::PRIMARY),
                retarget: true,
                colors: None,
            }),
        ),
    ]),
);

// MH2 52 — Murktide Regent
pub(in crate::card::sets) static MURKTIDE_REGENT: CardRecord = CardRecord::new(
    CardSet::ModernHorizons2,
    "Murktide Regent",
    "20c4aae1-7665-4df7-bd51-a1d95bf8a17d",
    "Lucas Graciano",
    CardRules::new_creature(mana_cost!("{5}{U}{U}"), &["Dragon"], 3, 3).with_abilities(&[
        abilities::delve(),
        abilities::flying(),
        AbilityDef::as_enters(
            "This creature enters with a +1/+1 counter on it for each instant and sorcery card exiled with it.",
            ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::AddCountersValue {
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::CountObjects(&ObjectSetDef::Matching {
                        objects: &ObjectSetDef::LinkedExiles,
                        object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Instant),
                            ObjectPredicateDef::HasType(CardType::Sorcery),
                        ])),
                    }),
                },
            ),
        ),
        AbilityDef::triggered(
            "Whenever an instant or sorcery card leaves your graveyard, put a +1/+1 counter on this creature.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::OwnedBy(PlayerRelation::You),
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                    ]),
                ]),
                Some(ZoneKind::Graveyard),
                None,
            ),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// MH2 67 — Subtlety
pub(in crate::card::sets) static SUBTLETY: CardRecord = CardRecord::new(
    CardSet::ModernHorizons2,
    "Subtlety",
    "701256d5-1389-48b7-9581-d6037209bd06",
    "Anastasia Ovchinnikova",
    // Free interaction that leaves a body when you have the mana, and a
    // blue card off the top of your hand when you do not.
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Elemental", "Incarnation"], 3, 3)
        .with_abilities(&[
            abilities::flash(),
            abilities::flying(),
            abilities::enters_trigger_with_targets(
                "When this creature enters, choose up to one target creature spell or planeswalker \
                 spell. Its owner puts it on their choice of the top or bottom of their library.",
                // A creature or planeswalker spell on the stack, anybody's. "Up to one"
                // means a Subtlety with nothing worth answering still enters and still
                // leaves a 3/3 behind.
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Spell,
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Creature),
                                ObjectPredicateDef::HasType(CardType::Planeswalker),
                            ]),
                        ]),
                        zones: &[ZoneKind::Stack],
                        controller: None,
                        owner: None,
                    },
                    1,
                )],
                EffectDef::PutSpellIntoOwnersLibrary {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::alternative_cast(
                mana_cost!("{0}"),
                AlternativeCastKindDef::AlternativeCost,
                Some("Evoke—Exile a blue card from your hand."),
                EffectDef::None,
            )
            .with_alternative_additional_cost(&SpellAdditionalCostDef::exile(
                ObjectPredicateDef::Color(ManaColor::Blue),
                ZoneKind::Hand,
                CostQuantityDef::Fixed(1),
            )),
            abilities::evoke_sacrifice(
                "When this creature enters, if it was evoked, sacrifice it.",
            ),
        ]),
);

// MH2 75 — Archon of Cruelty
static A_CREATURE_OR_PLANESWALKER: ObjectPredicateDef = ObjectPredicateDef::AnyOf(&[
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::HasType(CardType::Planeswalker),
]);

pub(in crate::card::sets) static ARCHON_OF_CRUELTY: CardRecord = CardRecord::new(
    CardSet::ModernHorizons2,
    "Archon of Cruelty",
    "1be9d9a4-d7ee-4854-abc2-85cabf993ec9",
    "Andrew Mar",
    // Eight mana nobody pays: he is a reanimation target, and the trigger is
    // why -- a six-point swing and two cards the turn he lands, and again
    // every turn he attacks.
    CardRules::new_creature(mana_cost!("{6}{B}{B}"), &["Archon"], 6, 6).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "Whenever this creature enters or attacks, target opponent sacrifices a creature or \
             planeswalker of their choice, discards a card, and loses 3 life. You draw a card and \
             gain 3 life.",
            // One printed ability with two ways in: he arrives, or he attacks. Two
            // abilities would make him trigger twice on a turn he does both, which the
            // card does not say -- and would count as two triggered abilities where the
            // card has one.
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
            ]),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
            )],
            // Four things in one sentence, in the order they are printed: what the
            // opponent gives up, then what you get. The sacrifice is theirs to choose,
            // which is why it is a procedure rather than a targeted destruction.
            EffectDef::Sequence(&[
                EffectDef::SacrificeOfChoice {
                    player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    object: A_CREATURE_OR_PLANESWALKER,
                    count: ValueDef::Constant(1),
                    then: None,
                    amount: SacrificedAmountDef::Power,
                    otherwise: None,
                    optional: false,
                },
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(1),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: None,
                },
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(3),
                },
                EffectDef::Sequence(&[
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(3),
                    },
                ]),
            ]),
        ),
    ]),
);

// MH2 76 — Bone Shards
pub(in crate::card::sets) static BONE_SHARDS: CardRecord = CardRecord::new(
    CardSet::ModernHorizons2,
    "Bone Shards",
    "1ee98955-4c47-4d45-9377-608dfa755337",
    "Tommy Arnold",
    // One black kills anything, and the second card is the price. A deck
    // full of things it wants in the graveyard pays it gladly.
    CardRules::new_sorcery(mana_cost!("{B}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, sacrifice a creature or discard a card.\nDestroy target creature or planeswalker.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                ]),
            )],
            // The second half of "sacrifice a creature or discard a card". Which half
            // is paid is settled as the spell is cast: both spend a card the caster
            // already had, and the enumeration offers every one of them.
            SpellAdditionalCostDef::choice(&[
                SpellAdditionalCostDef::sacrifice(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    CostQuantityDef::Fixed(1),
                ),
                SpellAdditionalCostDef::discard(
                    ObjectPredicateDef::Any,
                    CostQuantityDef::Fixed(1),
                ),
            ]),
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
                then: None,
            },
        ),
    ),
);

// MH2 80 — Damn
pub(in crate::card::sets) static DAMN: CardRecord = CardRecord::new(
    CardSet::ModernHorizons2,
    "Damn",
    "efeae088-9ac5-4d2f-a15c-d8675a471ac5",
    "Lucas Graciano",
    // Two black is removal and four with two white is a Wrath, off one card
    // -- and neither half leaves anything to regenerate, which is what puts
    // it ahead of the sorceries it is otherwise a copy of.
    CardRules::new_sorcery(mana_cost!("{B}{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Destroy target creature. A creature destroyed this way can't be regenerated.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: false,
                then: None,
            },
        ),
        AbilityDef::alternative_cast(
            mana_cost!("{2}{W}{W}"),
            AlternativeCastKindDef::Overload,
            Some("Destroy each creature. A creature destroyed this way can't be regenerated."),
            EffectDef::Destroy {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                can_regenerate: false,
                then: None,
            },
        ),
    ]),
);

// MH2 87 — Grief
pub(in crate::card::sets) static GRIEF: CardRecord = CardRecord::new(
    CardSet::ModernHorizons2,
    "Grief",
    "e6befbc4-1320-4f26-bd9f-b1814fedda10",
    "Nicholas Gregory",
    // Two black cards for a Thoughtseize on turn one, and a 3/2 that is
    // hard to block on the turns you have four mana instead.
    CardRules::new_creature(mana_cost!("{2}{B}{B}"), &["Elemental", "Incarnation"], 3, 2)
        // Thoughtseize's clause without the life, and aimed at an opponent rather
        // than any player: revealed rather than looked at, so the choice is one
        // both players can check.
        .with_abilities(&[
            abilities::menace(),
            abilities::enters_trigger_with_targets(
                "When this creature enters, target opponent reveals their hand. You choose a nonland \
                 card from it. That player discards that card.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                )],
                EffectDef::Sequence(&abilities::reveal_hand_and_discard_chosen_card(
                    PlayerRefDef::Target(TargetIndex::PRIMARY),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                )),
            ),
            AbilityDef::alternative_cast(
                mana_cost!("{0}"),
                AlternativeCastKindDef::AlternativeCost,
                Some("Evoke—Exile a black card from your hand."),
                EffectDef::None,
            )
            .with_alternative_additional_cost(&SpellAdditionalCostDef::exile(
                ObjectPredicateDef::Color(ManaColor::Black),
                ZoneKind::Hand,
                CostQuantityDef::Fixed(1),
            )),
            abilities::evoke_sacrifice("When this creature enters, if it was evoked, sacrifice it."),
        ]),
);

// MH2 91 — Loathsome Curator
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LOATHSOME_CURATOR: CardRecord = CardRecord::new(
    crate::card::CardSet::ModernHorizons2,
    "Loathsome Curator",
    "11a59a6f-6ef0-4acc-8358-a4e2cebdb7d5",
    "Mila Pesic",
    crate::card::CardRules::unsupported(),
);

// MH2 95 — Nested Shambler
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static NESTED_SHAMBLER: CardRecord = CardRecord::new(
    crate::card::CardSet::ModernHorizons2,
    "Nested Shambler",
    "9851f290-f502-49f8-9b48-67f7966d4e34",
    "Nicholas Gregory",
    crate::card::CardRules::unsupported(),
);

// MH2 107 — Vermin Gorger
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static VERMIN_GORGER: CardRecord = CardRecord::new(
    crate::card::CardSet::ModernHorizons2,
    "Vermin Gorger",
    "d3166b10-5bc3-4db6-bb5b-81045d98e446",
    "Tobias Kwan",
    crate::card::CardRules::unsupported(),
);

// MH2 121 — Dragon's Rage Channeler
pub(in crate::card::sets) static DRAGON_S_RAGE_CHANNELER: CardRecord = CardRecord::new(
    CardSet::ModernHorizons2,
    "Dragon's Rage Channeler",
    "4ced112a-e775-4f97-97b3-74877e9dce12",
    "Martina Fačková",
    // One mana for a 1/1 that fills its own graveyard and turns into a 3/3
    // flier for doing what the deck was going to do anyway. The compulsion
    // to attack is the price, and it is rarely one.
    CardRules::new_creature(mana_cost!("{R}"), &["Human", "Shaman"], 1, 1)
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever you cast a noncreature spell, surveil 1. (Look at the top card of your library. \
                 You may put that card into your graveyard.)",
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                abilities::surveil(ValueDef::Constant(1)),
            ),
            // "As long as", so it is asked live rather than once: the 3/3 flier is
            // a 1/1 again the moment the fourth card type leaves the graveyard.
            AbilityDef::static_ability(
                "Delirium — As long as there are four or more card types among cards in your graveyard, \
                 this creature gets +2/+2, has flying, and attacks each combat if able.",
                EffectDef::IfCondition {
                    // Four card types among the cards in your own graveyard, which the surveil
                    // above is what fills: the look is the cost of nothing and the delirium is
                    // what it buys.
                    condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                        left: ValueDef::CardTypesAmongGraveyards(PlayerRelation::You),
                        comparison: ComparisonDef::GreaterOrEqual,
                        right: ValueDef::Constant(4),
                    }),
                    // Three grants under one condition, so they arrive and leave together: a
                    // graveyard that falls back under four takes the flying and the compulsion
                    // with it.
                    then: &EffectDef::Sequence(&[
                        EffectDef::StaticApply {
                            recipient: EffectRecipientDef::Source,
                            effect: AppliedEffectDef::modify_power_toughness(
                                ValueDef::Constant(2),
                                ValueDef::Constant(2),
                            ),
                        },
                        EffectDef::StaticApply {
                            recipient: EffectRecipientDef::Source,
                            effect: AppliedEffectDef::add_ability(&abilities::flying()),
                        },
                        EffectDef::StaticApply {
                            recipient: EffectRecipientDef::Source,
                            effect: AppliedEffectDef::add_ability(&abilities::attacks_each_combat_if_able("This creature attacks each combat if able.")),
                        },
                    ]),
                },
            ),
        ]),
);

// MH2 126 — Fury
pub(in crate::card::sets) static FURY: CardRecord = CardRecord::new(
    CardSet::ModernHorizons2,
    "Fury",
    "bd281158-8180-40b9-a5b7-03cfc712d81a",
    "Raoul Vitale",
    CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Elemental", "Incarnation"], 3, 3)
        .with_abilities(&[
            abilities::double_strike(),
            abilities::enters_trigger_with_targets(
                "When this creature enters, it deals 4 damage divided as you choose among any number of target creatures and/or planeswalkers.",
                // Four damage split however the caster likes, over creatures and
                // planeswalkers alike. Every target must be assigned at least one, so four
                // is the most it can ever cover.
                &[AbilityTargetDef {
                    predicate: AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Planeswalker),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    minimum: 1,
                    maximum: AbilityTargetDef::UNLIMITED,
                    exact_count: None,
                    divided_total: Some(DividedTotal::Fixed(4)),
                    another: false,
                    excludes_source: false,
                    chooser: TargetChooserDef::Controller,
                }],
                EffectDef::DealDamage {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::DividedAmongTargets,
                },
            ),
            AbilityDef::alternative_cast(
                mana_cost!("{0}"),
                AlternativeCastKindDef::AlternativeCost,
                Some("Evoke—Exile a red card from your hand."),
                EffectDef::None,
            )
            .with_alternative_additional_cost(&SpellAdditionalCostDef::exile(
                ObjectPredicateDef::Color(ManaColor::Red),
                ZoneKind::Hand,
                CostQuantityDef::Fixed(1),
            )),
            // Evoke's own sacrifice. It is a separate trigger because it happens
            // after the Elemental has arrived, alongside the damage trigger rather
            // than instead of it -- which is why an evoked Fury still burns.
            abilities::evoke_sacrifice("When this creature enters, if it was evoked, sacrifice it."),
        ]),
);

// MH2 135 — Mine Collapse
pub(in crate::card::sets) static MINE_COLLAPSE: CardRecord = CardRecord::new(
    CardSet::ModernHorizons2,
    "Mine Collapse",
    "56e2e8b5-660d-4469-a4fe-2367dfadb709",
    "Bud Cook",
    // Nobody pays four mana for this. What it is worth is a land off an
    // already-flooded board on your own turn, which is why the free half is
    // the half that reads "if it's your turn".
    CardRules::new_instant(mana_cost!("{3}{R}")).with_abilities(&[
        AbilityDef::alternative_cast(
            mana_cost!("{0}"),
            AlternativeCastKindDef::AlternativeCost,
            Some(
                "If it's your turn, you may sacrifice a Mountain rather than pay this spell's \
                 mana cost.",
            ),
            EffectDef::None,
        )
        // A Mountain, not a red source: what the cost names is the land type, so a
        // Sacred Foundry pays it and a Mountain that has stopped being one does not.
        .with_alternative_additional_cost(&SpellAdditionalCostDef::sacrifice(
            ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Mountain]),
            CostQuantityDef::Fixed(1),
        ))
        // "If it's your turn" gates only the free cast. The printed cost is always
        // available, which is why this is a condition on the alternative rather
        // than a restriction on the card.
        .with_alternative_condition(&TriggerConditionDef::ActivePlayer(PlayerRelation::You)),
        AbilityDef::spell_with_targets(
            "Mine Collapse deals 5 damage to target creature or planeswalker.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Planeswalker),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(5),
            },
        ),
    ]),
);

// MH2 138 — Ragavan, Nimble Pilferer
pub(in crate::card::sets) static RAGAVAN_NIMBLE_PILFERER: CardRecord = CardRecord::new(
    CardSet::ModernHorizons2,
    "Ragavan, Nimble Pilferer",
    "a9738cda-adb1-47fb-9f4c-ecd930228c4d",
    "Simon Dominic",
    // One mana for a 2/1 that pays for itself the first time it connects,
    // and a dash cost for the turns when leaving it out would only get it
    // killed.
    CardRules::new_creature(mana_cost!("{R}"), &["Monkey", "Pirate"], 2, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::triggered(
                "Whenever this creature deals combat damage to a player, create a Treasure token and \
                 exile the top card of that player's library. Until end of turn, you may cast that card.",
                TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
                EffectDef::Sequence(&[
                    EffectDef::create_token(tokens::treasure()).with_art(CardArt::new(
                        "630c0d1c-9ddb-4e76-a82a-9cdd8a5b487b",
                        "Alayna Danner",
                    )),
                    // "That player's library", and the permission is yours: what the Monkey
                    // steals is theirs to lose and yours to cast.
                    EffectDef::ExileTopOfLibraryToPlay {
                        player: EffectRecipientDef::EventPlayer,
                        amount: ValueDef::Constant(1),
                        free: false,
                        face_down: false,
                        duration: ExilePlayDurationDef::ThisTurn,
                        spend_any_color: false,
                        play_condition: None,
                        cast_only: true,
                    },
                ]),
            ),
            abilities::dash(
                mana_cost!("{1}{R}"),
                "Dash {1}{R} (You may cast this spell for its dash cost. If you do, it gains haste, and \
                 it's returned from the battlefield to its owner's hand at the beginning of the next end \
                 step.)",
            ),
            abilities::dashed_haste(),
            abilities::dashed_return(),
        ]),
);

// MH2 145 — Unholy Heat
pub(in crate::card::sets) static UNHOLY_HEAT: CardRecord = CardRecord::new(
    CardSet::ModernHorizons2,
    "Unholy Heat",
    "2b73d294-6ab1-4051-9b0f-d8e335d37674",
    "Kari Christensen",
    CardRules::new_instant(mana_cost!("{R}")).with_ability(AbilityDef::spell_with_targets(
        "Unholy Heat deals 2 damage to target creature or planeswalker.\nDelirium — Unholy Heat deals 6 damage instead if there are four or more card types among cards in your graveyard.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            // Delirium changes the amount, not the effect, so it is a conditional value
            // rather than a second clause: four card types in your own graveyard, and
            // the same spell deals six.
            amount: ValueDef::IfCardTypesAmongGraveyards(&GraveyardTypeConditionDef {
                player: PlayerRelation::You,
                minimum: 4,
                then: ValueDef::Constant(6),
                otherwise: ValueDef::Constant(2),
            }),
        },
    )),
);

// MH2 147 — Abundant Harvest
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ABUNDANT_HARVEST: CardRecord = CardRecord::new(
    crate::card::CardSet::ModernHorizons2,
    "Abundant Harvest",
    "5ad86b17-3fed-418a-938c-c49adb409531",
    "Iris Compiet",
    crate::card::CardRules::unsupported(),
);

// MH2 149 — Bannerhide Krushok
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BANNERHIDE_KRUSHOK: CardRecord = CardRecord::new(
    crate::card::CardSet::ModernHorizons2,
    "Bannerhide Krushok",
    "1271251b-7d79-4cb4-80bb-98574aa63249",
    "Joe Slucher",
    crate::card::CardRules::unsupported(),
);

// MH2 157 — Endurance
pub(in crate::card::sets) static ENDURANCE: CardRecord = CardRecord::new(
    CardSet::ModernHorizons2,
    "Endurance",
    "eb0e0404-4846-4891-acfa-bd0951ecf9c6",
    "Anastasia Ovchinnikova",
    // A free answer to a graveyard that leaves a 3/4 blocker behind, or a
    // green card off the top of your hand when the graveyard is the whole
    // reason you are casting it.
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Elemental", "Incarnation"], 3, 4)
        .with_abilities(&[
            abilities::flash(),
            abilities::reach(),
            abilities::enters_trigger_with_targets(
                "When this creature enters, up to one target player puts all the cards from their \
                 graveyard on the bottom of their library in a random order.",
                // "Up to one target player" includes yourself, which is the mode nobody
                // prints on the card: an Endurance can put your own graveyard back when
                // something else is trying to eat it.
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                    1,
                )],
                EffectDef::BuryGraveyard {
                    player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::alternative_cast(
                mana_cost!("{0}"),
                AlternativeCastKindDef::AlternativeCost,
                Some("Evoke—Exile a green card from your hand."),
                EffectDef::None,
            )
            .with_alternative_additional_cost(&SpellAdditionalCostDef::exile(
                ObjectPredicateDef::Color(ManaColor::Green),
                ZoneKind::Hand,
                CostQuantityDef::Fixed(1),
            )),
            abilities::evoke_sacrifice(
                "When this creature enters, if it was evoked, sacrifice it.",
            ),
        ]),
);

// MH2 181 — Urban Daggertooth
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static URBAN_DAGGERTOOTH: CardRecord = CardRecord::new(
    crate::card::CardSet::ModernHorizons2,
    "Urban Daggertooth",
    "4ab83a39-d90d-403e-b74d-fe99c8b2aacd",
    "Randy Vargas",
    crate::card::CardRules::unsupported(),
);

// MH2 188 — Captured by Lagacs
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CAPTURED_BY_LAGACS: CardRecord = CardRecord::new(
    crate::card::CardSet::ModernHorizons2,
    "Captured by Lagacs",
    "7ce1c2a8-688b-4f63-8d58-e325efc6052a",
    "Andrew Mar",
    crate::card::CardRules::unsupported(),
);

// MH2 202 — Grist, the Hunger Tide
pub(in crate::card::sets) static GRIST_THE_HUNGER_TIDE: CardRecord = CardRecord::new(
    CardSet::ModernHorizons2,
    "Grist, the Hunger Tide",
    "69af2825-18c2-4463-b6ba-42eaa070ccc1",
    "Yongjae Choi",
    // Three mana that makes a body every turn and answers one on the turn
    // it lands, which is why it is played over the planeswalkers that only
    // do one of those.
    CardRules::new_planeswalker(mana_cost!("{1}{B}{G}"), &["Grist"], 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "As long as Grist isn't on the battlefield, it's a 1/1 Insect creature in addition to its \
                 other types.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: // "A 1/1 Insect creature in addition to its other types": a creature card
                        // with an Insect subtype and a body, added to what the card already is
                        // rather than replacing it.
                        AppliedEffectDef::Composite(&[
                            AppliedEffectDef::Characteristic(CharacteristicOperationDef::CardTypes(SetOperationDef::Add(
                                CardTypeSet::single(CardType::Creature),
                            ))),
                            AppliedEffectDef::Characteristic(CharacteristicOperationDef::Subtypes(SetOperationDef::Add(
                                &["Insect"],
                            ))),
                            AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
                                PowerToughnessOperationDef::SetBase {
                                    power: ValueDef::Constant(1),
                                    toughness: ValueDef::Constant(1),
                                },
                            )),
                        ]),
                },
            )
            // "As long as Grist isn't on the battlefield": every zone but that one,
            // which is a list of source zones rather than a condition to recheck.
            // The stack is one of them, so the spell on its way in is a creature
            // spell -- Essence Scatter counters it and Negate does not.
            .with_source_zones(&[
                ZoneKind::Library,
                ZoneKind::Hand,
                ZoneKind::Graveyard,
                ZoneKind::Stack,
                ZoneKind::Exile,
                ZoneKind::Command,
            ]),
            AbilityDef::activated(
                "+1: Create a 1/1 black and green Insect creature token, then mill a card. If an Insect \
                 card was milled this way, put a loyalty counter on Grist and repeat this process.",
                &[AbilityCostDef::Loyalty(1)],
                // The library is what bounds this in practice; the limit is only there so
                // a process with nothing to stop it still stops.
                EffectDef::MillWhileMatching(&MillLoopDef {
                    player: EffectRecipientDef::Controller,
                    body: &EffectDef::create_creature_token(&["Insect"], &[ManaColor::Black, ManaColor::Green], 1, 1),
                    // An Insect card in the library keeps the process going -- and a Grist on
                    // top is one, which is what his own first clause is for.
                    object: ObjectPredicateDef::Subtype("Insect"),
                    on_match: &EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::Loyalty,
                        amount: ValueDef::Constant(1),
                    },
                    limit: 512,
                }),
            ),
            AbilityDef::activated(
                "\u{2212}2: You may sacrifice a creature.",
                &[AbilityCostDef::Loyalty(-2)],
                EffectDef::PayOr(PayOrDef::optional(
                    EffectPaymentDef {
                        payer: PlayerSetDef::Related(PlayerRelation::You),
                        cost: EffectPaymentCostDef::SacrificePermanentMatching(ObjectPredicateDef::HasType(
                                CardType::Creature,
                            )),
                    },
                    &EffectDef::None,
                )),
            ),
            AbilityDef::triggered_with_targets(
                "When you do, destroy target creature or planeswalker.",
                TriggerEventDef::OptionalEffectTaken(ObjectPredicateDef::Source),
                &[AbilityTargetDef::exactly_one_permanent(
                    A_CREATURE_OR_PLANESWALKER,
                )],
                EffectDef::Destroy {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    can_regenerate: true,
                    then: None,
                },
            ),
            AbilityDef::activated(
                "\u{2212}5: Each opponent loses life equal to the number of creature cards in your \
                 graveyard.",
                &[AbilityCostDef::Loyalty(-5)],
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Opponent,
                    amount: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Graveyard],
                        PlayerRelation::You,
                    )),
                },
            ),
        ]),
);

// MH2 216 — Territorial Kavu
pub(in crate::card::sets) static TERRITORIAL_KAVU: CardRecord = CardRecord::new(
    CardSet::ModernHorizons2,
    "Territorial Kavu",
    "2605df98-0b02-4aab-bc36-01e93c693743",
    "E. M. Gist",
    // Two mana for as big a body as your mana base is greedy, and an attack
    // trigger that either loots or eats a graveyard.
    CardRules::new_creature(mana_cost!("{R}{G}"), &["Kavu"], 0, 0).with_abilities(&[
        AbilityDef::static_ability(
            "Domain — This creature's power and toughness are each equal to the number of basic \
             land types among lands you control.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                // Domain: how many of the five basic land types are among your lands. A
                // Kavu on a two-colour board is a 2/2, and one behind a full spread of
                // fetched duals is a 5/5.
                effect: AppliedEffectDef::define_power_toughness(
                    ValueDef::BasicLandTypesControlled(PlayerRelation::You),
                    ValueDef::BasicLandTypesControlled(PlayerRelation::You),
                ),
            },
        ),
        AbilityDef::modal_triggered(
            "Whenever this creature attacks, choose one —\n• Discard a card. If you do, draw a \
             card.\n• Exile up to one target card from a graveyard.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &[
                AbilityDef::spell(
                    "Discard a card. If you do, draw a card.",
                    EffectDef::Discard {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                        selection: DiscardSelectionDef::RecipientChooses,
                        then: Some(DiscardFollowUpDef {
                            counted: ObjectPredicateDef::Any,
                            bound: None,
                            // "If you do": the draw is sized by what the discard actually took, so an
                            // empty hand discards nothing and draws nothing.
                            effect: &EffectDef::DrawCards {
                                recipient: EffectRecipientDef::Controller,
                                amount: ValueDef::MatchedCount,
                            },
                        }),
                    },
                ),
                AbilityDef::spell_with_targets(
                    "Exile up to one target card from a graveyard.",
                    &[AbilityTargetDef::up_to(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::Any,
                            zones: &[ZoneKind::Graveyard],
                            controller: None,
                            owner: None,
                        },
                        1,
                    )],
                    EffectDef::MoveToZone {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        zone: ZoneKind::Exile,
                        placement: ZonePlacement::Top,
                    },
                ),
            ],
        ),
    ]),
);

// MH2 227 — Kaldra Compleat
pub(in crate::card::sets) static KALDRA_COMPLEAT: CardRecord = CardRecord::new(
    CardSet::ModernHorizons2,
    "Kaldra Compleat",
    "87cc2855-6b14-44dd-a398-7dc2bbae081f",
    "Vincent Proce",
    // Seven mana that arrives as a 5/5 first-striking, trampling,
    // indestructible, hasty creature which exiles whatever blocks it. The
    // Germ is the point: it never needs a creature to equip.
    CardRules::new_artifact(mana_cost!("{7}"))
        .with_supertype(CardSupertype::Legendary)
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::living_weapon(),
            abilities::indestructible(),
            AbilityDef::static_ability(
                "Equipped creature gets +5/+5 and has first strike, trample, indestructible, haste, and \
                 \"Whenever this creature deals combat damage to a creature, exile that creature.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(ValueDef::Constant(5), ValueDef::Constant(5)),
                        AppliedEffectDef::add_ability(&abilities::first_strike()),
                        AppliedEffectDef::add_ability(&abilities::trample()),
                        AppliedEffectDef::add_ability(&abilities::indestructible()),
                        AppliedEffectDef::add_ability(&abilities::haste()),
                        // The clause the equipped creature gains, not one Kaldra has itself: "that
                        // creature" is the one that took the damage, which is a different object
                        // from the one that dealt it.
                        AppliedEffectDef::add_ability(&AbilityDef::triggered(
                            "Whenever this creature deals combat damage to a creature, exile that creature.",
                            TriggerEventDef::DamageDealt(DamageEventMatcherDef {
                                kind: DamageKindDef::Combat,
                                source: DamageSourceMatcherDef::Object(ObjectRefDef::Source),
                                recipient: DamageRecipientMatcherDef::MatchingObject(ObjectPredicateDef::HasType(
                                    CardType::Creature,
                                )),
                            }),
                            EffectDef::MoveToZone {
                                object: EffectRecipientDef::DamagedObject,
                                zone: ZoneKind::Exile,
                                placement: ZonePlacement::Top,
                            },
                        )),
                    ]),
                },
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{7}"))], "Equip {7}"),
        ]),
);

// MH2 231 — Nettlecyst
/// "Artifact and/or enchantment" is one query rather than two sums: a
/// permanent that is both is counted once, and Nettlecyst counts itself.
static ARTIFACTS_AND_ENCHANTMENTS_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::AnyOf(&[
        ObjectPredicateDef::HasType(CardType::Artifact),
        ObjectPredicateDef::HasType(CardType::Enchantment),
    ]),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

pub(in crate::card::sets) static NETTLECYST: CardRecord = CardRecord::new(
    CardSet::ModernHorizons2,
    "Nettlecyst",
    "4a0bb5dc-75a6-4bd6-81f8-611197fb0fba",
    "Vincent Proce",
    CardRules::new_artifact(mana_cost!("{3}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::living_weapon(),
            AbilityDef::static_ability(
                "Equipped creature gets +1/+1 for each artifact and/or enchantment you control.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::CountMatchingObjects(&ARTIFACTS_AND_ENCHANTMENTS_YOU_CONTROL),
                        ValueDef::CountMatchingObjects(&ARTIFACTS_AND_ENCHANTMENTS_YOU_CONTROL),
                    ),
                },
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{2}"))], "Equip {2}"),
        ]),
);

// MH2 261 — Yavimaya, Cradle of Growth
pub(in crate::card::sets) static YAVIMAYA_CRADLE_OF_GROWTH: CardRecord = CardRecord::new(
    CardSet::ModernHorizons2,
    "Yavimaya, Cradle of Growth",
    "4e4b6e22-93b2-4896-bba5-0ceaa5d8ea3c",
    "Sarah Finnigan",
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::static_ability(
            "Each land is a Forest in addition to its other land types.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Land),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::add_basic_land_types(&[BasicLandType::Forest]),
            },
        )),
);

// MH2 355 — Ignoble Hierarch
pub(in crate::card::sets) static IGNOBLE_HIERARCH: CardRecord = CardRecord::new(
    CardSet::ModernHorizons2,
    "Ignoble Hierarch",
    "3139cce8-3467-4c50-add2-5b78fb33b90a",
    "Mark Zug",
    // Noble Hierarch in the other three colours: the same one-mana
    // accelerant, and the same 0/1 that exalted turns into a real
    // dividend on a turn when only one creature attacks.
    CardRules::new_creature(mana_cost!("{G}"), &["Goblin", "Shaman"], 0, 1).with_abilities(&[
        abilities::exalted(),
        AbilityDef::activated_mana(
            "{T}: Add {B}, {R}, or {G}.",
            &[AbilityCostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::choice(&[
                ManaColor::Black,
                ManaColor::Red,
                ManaColor::Green,
            ])),
        ),
    ]),
);

// MH2 380 — Urza's Saga
static ARTIFACTS_YOU_CONTROL_SAGA: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::HasType(CardType::Artifact),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

pub(in crate::card::sets) static URZA_S_SAGA: CardRecord = CardRecord::new(
    CardSet::ModernHorizons2,
    "Urza's Saga",
    "2138dfbb-a4e3-49db-b908-95d0b2b7e82f",
    "Titus Lunter",
    // A land that costs nothing, taps for one turn's mana, spends the next
    // two turns making Constructs, and fetches the artifact that makes them
    // bigger on its way out.
    // Two subtypes rather than one name: the land type "Urza's" that the
    // Urzatron cares about, and the enchantment type "Saga" that the lore
    // counters read.
    CardRules::new_land(&["Urza's", "Saga"])
        .with_type(CardType::Enchantment)
        .with_abilities(&[
            abilities::saga_chapter(
                1,
                "I — This Saga gains \"{T}: Add {C}.\"",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    // What chapter I hands the land. It is granted for good rather than for the
                    // turn: the Saga taps for mana from the moment the first chapter resolves
                    // until it sacrifices itself after the third.
                    effect: AppliedEffectDef::add_ability(&AbilityDef::activated_mana(
                        "{T}: Add {C}.",
                        &[AbilityCostDef::TapSource],
                        EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
                    )),
                    duration: ResolvedEffectDurationDef::Permanent,
                },
            ),
            abilities::saga_chapter(
                2,
                "II — This Saga gains \"{2}, {T}: Create a 0/0 colorless Construct artifact creature \
                 token with 'This token gets +1/+1 for each artifact you control.'\"",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&AbilityDef::activated(
                        "{2}, {T}: Create a 0/0 colorless Construct artifact creature token with \"This token gets \
                         +1/+1 for each artifact you control.\"",
                        &[
                            AbilityCostDef::Mana(mana_cost!("{2}")),
                            AbilityCostDef::TapSource,
                        ],
                        EffectDef::create_artifact_creature_token(&["Construct"], &[], 0, 0)
                            // The token's own clause, printed on the token rather than on the Saga:
                            // it counts itself, so the first one is a 1/1 on an otherwise empty board.
                            .with_abilities(&[AbilityDef::static_ability(
                                "This token gets +1/+1 for each artifact you control.",
                                EffectDef::StaticApply {
                                    recipient: EffectRecipientDef::Source,
                                    effect: AppliedEffectDef::modify_power_toughness(
                                        ValueDef::CountMatchingObjects(&ARTIFACTS_YOU_CONTROL_SAGA),
                                        ValueDef::CountMatchingObjects(&ARTIFACTS_YOU_CONTROL_SAGA),
                                    ),
                                },
                            )]),
                    )),
                    duration: ResolvedEffectDurationDef::Permanent,
                },
            ),
            abilities::saga_chapter(
                3,
                "III — Search your library for an artifact card with mana cost {0} or {1}, put it onto \
                 the battlefield, then shuffle.",
                EffectDef::SearchZone {
                    player: EffectRecipientDef::Controller,
                    source: ZoneKind::Library,
                    // "Mana cost {0} or {1}" is the printed cost, not the mana
                    // value: Portable Hole costs {W} and Walking Ballista costs
                    // {X}{X}, and neither is findable even though both have a
                    // mana value of at most one.
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Artifact),
                        ObjectPredicateDef::GenericManaCostAtMost(1),
                    ]),
                    minimum: 0,
                    maximum: ValueDef::Constant(1),
                    reveal: true,
                    destination: ZoneKind::Battlefield,
                    placement: ZonePlacement::Top,
                    shuffle: true,
                    enters_tapped: false,
                    attachment: None,
                    binding: None,
                    then: None,
                },
            ),
        ]),
);

// MH2 421 — Goblin Anarchomancer
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_ANARCHOMANCER: CardRecord = CardRecord::new(
    crate::card::CardSet::ModernHorizons2,
    "Goblin Anarchomancer",
    "f7f07a80-05b5-4108-9e68-f8da05866acc",
    "Joe Slucher",
    crate::card::CardRules::unsupported(),
);

// MH2 450 — Dauthi Voidwalker
pub(in crate::card::sets) static DAUTHI_VOIDWALKER: CardRecord = CardRecord::new(
    CardSet::ModernHorizons2,
    "Dauthi Voidwalker",
    "29632951-3c3d-478c-8c5a-9a34f30a5c28",
    "Sidharth Chaturvedi",
    // Two mana for a body nothing ordinary can block, a graveyard nobody
    // else gets to use, and one card off the top of that pile.
    CardRules::new_creature(mana_cost!("{B}{B}"), &["Dauthi", "Rogue"], 3, 2)
        .with_abilities(&[
            abilities::shadow(),
            AbilityDef::replacement_for(
                "If a card would be put into an opponent\'s graveyard from anywhere, instead exile it \
                 with a void counter on it.",
                // Their cards, not yours, and cards rather than tokens: a token that would
                // die still dies, and ceases to exist as it always would.
                ReplacementEventDef::AnyObjectWouldMove {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::OwnedBy(PlayerRelation::Opponent),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                    ]),
                    to: ZoneKind::Graveyard,
                },
                // The counter is the whole point: it marks the pile this creature is
                // allowed to reach back into, which is what separates it from the
                // graveyard hate that only takes things away.
                ReplacementEffectDef::Sequence(&[
                    ReplacementEffectDef::MoveToZone(ZoneKind::Exile),
                    ReplacementEffectDef::PlaceCountersOnMovedObject {
                        kind: CounterKind::named("void"),
                        amount: 1,
                    },
                ]),
            ),
            AbilityDef::activated(
                "{T}, Sacrifice this creature: Choose an exiled card an opponent owns with a void counter \
                 on it. You may play it this turn without paying its mana cost.",
                &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
                // One card, chosen as the ability resolves and cast then or not at all.
                // What it costs is nothing at all, which is why the creature has to die to
                // ask.
                EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                        ObjectPredicateDef::HasCounter(CounterKind::named("void")),
                        &[ZoneKind::Exile],
                        PlayerRelation::Opponent,
                    )),
                    exclude: None,
                    minimum: 1,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::MayPlayWithoutPaying(FreePlayDef {
                        objects: ObjectSetDef::Binding(ParentBinding),
                        duration: FreePlayDurationDef::WhileResolving,
                        mandatory: false,
                        grants_haste: false,
                    }),
                }),
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &PRISMATIC_ENDING,
    &SOLITUDE,
    &UNBOUNDED_POTENTIAL,
    &HARD_EVIDENCE,
    &LOSE_FOCUS,
    &MURKTIDE_REGENT,
    &SUBTLETY,
    &ARCHON_OF_CRUELTY,
    &BONE_SHARDS,
    &DAMN,
    &GRIEF,
    &LOATHSOME_CURATOR,
    &NESTED_SHAMBLER,
    &VERMIN_GORGER,
    &DRAGON_S_RAGE_CHANNELER,
    &FURY,
    &MINE_COLLAPSE,
    &RAGAVAN_NIMBLE_PILFERER,
    &UNHOLY_HEAT,
    &ABUNDANT_HARVEST,
    &BANNERHIDE_KRUSHOK,
    &ENDURANCE,
    &URBAN_DAGGERTOOTH,
    &CAPTURED_BY_LAGACS,
    &GRIST_THE_HUNGER_TIDE,
    &TERRITORIAL_KAVU,
    &KALDRA_COMPLEAT,
    &NETTLECYST,
    &YAVIMAYA_CRADLE_OF_GROWTH,
    &IGNOBLE_HIERARCH,
    &URZA_S_SAGA,
    &GOBLIN_ANARCHOMANCER,
    &DAUTHI_VOIDWALKER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
