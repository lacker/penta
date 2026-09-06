//! Final Fantasy cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::TargetIndex;
use crate::card::{
    AbilityDef, AbilityTargetDef, ActivationTimingDef, AddManaEffectDef, AdditionalTriggerDef,
    AppliedEffectDef, AppliedRuleDef, BattlefieldEntryModificationDef, CardArt, CardRules, CardSet,
    CardSupertype, CardType, CharacteristicOperationDef, ConditionDef, CostDef, CounterKind,
    CreatureTypeSetDef, DamageEventMatcherDef, DamageKindDef, DamageRecipientMatcherDef,
    DamageSourceMatcherDef, DrawEventMatcherDef, EffectDef, EffectRecipientDef, ManaColor,
    ObjectPredicateDef, ObjectRefDef, PlayActionMatcherDef, PlayRestrictionDef, PlayerRelation,
    PlayerSetDef, ReplacementEffectDef, ResolvedEffectDurationDef, SetOperationDef,
    TopOfLibraryCostDef, TriggerConditionDef, TriggerEventDef, ValueDef, ZoneKind, ZonePlacement,
    abilities,
};
use crate::mana_cost;

// FIN 91 — Cecil, Dark Knight // Cecil, Redeemed Paladin
pub(in crate::card::sets) static CECIL_DARK_KNIGHT: CardRecord = CardRecord::new_dfc_with_legacy_id(
    2129,
    "Cecil, Dark Knight // Cecil, Redeemed Paladin",
    CardArt::new("026e7167-d665-43d0-a51e-8df2d68cdb5e", "Josu Hernaiz"),
    CardSet::FinalFantasy,
    &[
        (
            "Cecil, Dark Knight",
            const {
                CardRules::new_creature(mana_cost!("{B}"), &const { ["Human", "Knight"] }, 2, 3)
                .with_supertype(CardSupertype::Legendary)
                .with_abilities(&const { [
                    abilities::deathtouch(),
                    AbilityDef::triggered(
                        "Darkness — Whenever Cecil deals damage, you lose that much life. Then if your life total is less than or equal to half your starting life total, untap Cecil and transform it.",
                        TriggerEventDef::DamageDealt(DamageEventMatcherDef {
                            kind: DamageKindDef::Any,
                            source: DamageSourceMatcherDef::Object(ObjectRefDef::Source),
                            recipient: DamageRecipientMatcherDef::Any,
                        }),
                        // "You lose that much life. Then if ..." is one clause resolving in order:
                        // the life is lost first, so the very damage that cost it can be what brings
                        // the total low enough to turn the card over.
                        EffectDef::Sequence(&const { [
                            EffectDef::LoseLife {
                                recipient: EffectRecipientDef::Controller,
                                amount: ValueDef::TriggerEventAmount,
                            },
                            EffectDef::IfCondition {
                                condition: &TriggerConditionDef::ControllerLifeAtMostHalfStartingLife,
                                // The front half's payoff, and the reason the card is played: hitting hard
                                // enough to halve your own life is what turns Cecil over. Untapping is part
                                // of the same clause, so a Cecil that traded its attack for the transform
                                // comes back ready to block.
                                then: &EffectDef::Sequence(&const { [
                                    EffectDef::Untap {
                                        object: EffectRecipientDef::Source,
                                    },
                                    EffectDef::Transform {
                                        object: EffectRecipientDef::Source,
                                    },
                                ] }),
                            },
                        ] }),
                    ),
                ] })
            },
        ),
        // The back face has no printed mana cost and is white, where the front is
        // black: transforming changes the colour it defends in.
        (
            "Cecil, Redeemed Paladin",
            const {
                CardRules::new_creature_without_mana_cost(&const { ["Human", "Knight"] }, 4, 4)
                .printed_colors(&const { [ManaColor::White] })
                .with_supertype(CardSupertype::Legendary)
                .with_abilities(&const { [
                    abilities::lifelink(),
                    AbilityDef::triggered(
                        "Protect — Whenever Cecil attacks, other attacking creatures gain indestructible until end of turn.",
                        TriggerEventDef::attacks(ObjectPredicateDef::Source),
                        EffectDef::Apply {
                            // "Other attacking creatures" excludes Cecil and takes in the opponent's
                            // too, on the rare turn both sides are attacking at once.
                            recipient: EffectRecipientDef::matching_objects(
                                ObjectPredicateDef::All(&const { [
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    ObjectPredicateDef::Attacking,
                                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                                ] }),
                                &const { [ZoneKind::Battlefield] },
                                PlayerRelation::Any,
                            ),
                            effect: AppliedEffectDef::add_ability(&const {
                                abilities::indestructible()
                            }),
                            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                        },
                    ),
                ] })
            },
        ),
    ],
);

// FIN 114 — Resentful Revelation
pub(in crate::card::sets) static RESENTFUL_REVELATION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("945006ea-c6a1-4ee5-abb2-387c2b6d3123"),
    "Resentful Revelation",
    CardArt::new("945006ea-c6a1-4ee5-abb2-387c2b6d3123", "Justyna Dura"),
    CardSet::FinalFantasy,
    // The two cards it buries are the point as often as the one it keeps,
    // and the flashback is what the graveyard deck is really paying for.
    CardRules::new_sorcery(mana_cost!("{1}{B}")).with_abilities(&[
        AbilityDef::spell(
            "Look at the top three cards of your library. Put one of them into your hand and the \
             rest into your graveyard.",
            // Exactly one, not up to one: the card is mandatory, and any of
            // the three qualifies.
            abilities::look_at_top_cards_choose_to_hand_rest_graveyard(
                ValueDef::Constant(3),
                ObjectPredicateDef::Any,
                1,
                1,
            ),
        ),
        abilities::flashback(mana_cost!("{6}{B}")),
    ]),
);

// FIN 164 — Suplex
pub(in crate::card::sets) static SUPLEX: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("f61693a2-7042-44e0-85ba-9bf12ab94e7e"),
    "Suplex",
    CardArt::new("f61693a2-7042-44e0-85ba-9bf12ab94e7e", "Fang Xinyu"),
    CardSet::FinalFantasy,
    // Three damage that answers a recursive creature for good, or the
    // artifact half when there is nothing to throw.
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_ability(AbilityDef::modal_spell(
        "Choose one —",
        &[
            AbilityDef::spell_with_targets(
                "Suplex deals 3 damage to target creature. If that creature would die this turn, exile it \
                 instead.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                // The second sentence is about the creature, not about the damage: it is
                // applied to the target whether or not three damage was enough, or arrived
                // at all, so the two clauses resolve in order rather than as one linked
                // effect. A creature that shrugs the three off is still exiled if
                // something else finishes it before the turn ends.
                EffectDef::Sequence(&[
                    EffectDef::DealDamage {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(3),
                    },
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::Rule(AppliedRuleDef::ExileInsteadOfDying),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            ),
            AbilityDef::spell_with_targets(
                "Exile target artifact.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Artifact),
                )],
                EffectDef::MoveToZone {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    zone: ZoneKind::Exile,
                    placement: ZonePlacement::Top,
                },
            ),
        ],
    )),
);

// FIN 206 — Tifa Lockhart
pub(in crate::card::sets) static TIFA_LOCKHART: CardRecord = CardRecord::new_with_legacy_id(
    2146,
    "Tifa Lockhart",
    CardArt::new("fb781323-2746-405d-a9b2-e778c037a6e9", "Laurel Austin"),
    CardSet::FinalFantasy,
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Human", "Monk"], 1, 2)
        .with_supertype(CardSupertype::Legendary)
        // Doubling is +X/+0 where X is her power as this resolves, so two landfalls
        // in a turn compound: the second reads the size the first left behind.
        .with_abilities(&[
            abilities::trample(),
            AbilityDef::triggered(
                "Landfall — Whenever a land you control enters, double Tifa Lockhart's power until end of turn.",
                // A land you control, not any land: the opponent's fetchland does nothing
                // for her.
                TriggerEventDef::zone_changed(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]), None, Some(ZoneKind::Battlefield)),
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::SourcePower,
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// FIN 248 — Vivi Ornitier
pub(in crate::card::sets) static VIVI_ORNITIER: CardRecord = CardRecord::new_with_legacy_id(
    2162,
    "Vivi Ornitier",
    CardArt::new("ecc1027a-8c07-44a0-bdde-fa2844cff694", "Toni Infante"),
    CardSet::FinalFantasy,
    CardRules::new_creature(mana_cost!("{1}{U}{R}"), &["Wizard"], 0, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated_mana(
                "{0}: Add X mana in any combination of {U} and/or {R}, where X is this creature's power. Activate only during your turn and only once each turn.",
                &[CostDef::Mana(mana_cost!("{0}"))],
                // "Add X mana in any combination of {U} and/or {R}" divides one amount
                // across two types, so the runtime offers the ability once per division.
                // Vivi enters with no power at all, so the first activation worth making
                // comes after a noncreature spell has grown it.
                EffectDef::AddMana(AddManaEffectDef::combination(&[ManaColor::Blue, ManaColor::Red], 0).with_variable_amount(ValueDef::SourcePower)),
            )
            .with_activation_timing(ActivationTimingDef::YourTurn)
            .activations_each_turn(1),
            AbilityDef::triggered(
                "Whenever you cast a noncreature spell, put a +1/+1 counter on this creature and it deals 1 damage to each opponent.",
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::NoncreatureSpell,
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                // The counter and the damage are one clause, and the counter comes first --
                // so a Vivi that has just been cast at is already bigger by the time its own
                // mana ability is next offered.
                EffectDef::Sequence(&[
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::DealDamage {
                        recipient: EffectRecipientDef::Opponent,
                        amount: ValueDef::Constant(1),
                    },
                ]),
            ),
        ]),
);

// FIN 289 — Starting Town
pub(in crate::card::sets) static STARTING_TOWN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("fc7d1912-7e27-49ef-bd98-375d975a42b0"),
    "Starting Town",
    CardArt::new("fc7d1912-7e27-49ef-bd98-375d975a42b0", "Hristo D. Chukov"),
    CardSet::FinalFantasy,
    // A City of Brass for the turns that matter and a tapped land after
    // them, which is the trade a deck makes for fixing it only needs early.
    CardRules::new_land(&["Town"]).with_abilities(&[
        // "Your first, second, or third turn of the game" counts the turns you
        // have taken rather than the turn number: on the draw, your third turn
        // is the game's sixth, and the Town still comes in untapped.
        AbilityDef::as_enters(
            "This land enters tapped unless it's your first, second, or third turn of the game.",
            ReplacementEffectDef::Conditional {
                condition: ConditionDef::ControllerTurnsTakenAtMost(3),
                if_true: &[],
                if_false: &[ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::Tapped,
                )],
            },
        ),
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_mana(
            "{T}, Pay 1 life: Add one mana of any color.",
            &[CostDef::TapSource, CostDef::PayLife(1)],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// FIN 551c — Traveling Chocobo
pub(in crate::card::sets) static TRAVELING_CHOCOBO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("156cfd45-1556-4804-becf-039cfff7de3d"),
    "Traveling Chocobo",
    crate::card::CardArt::new("156cfd45-1556-4804-becf-039cfff7de3d", "Toni Infante"),
    crate::card::CardSet::FinalFantasy,
    // Three mana for a body, a land engine, and a Panharmonicon that only
    // reads lands and its own kind -- which in a deck built for it is most
    // of what enters.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Bird"], 3, 2)
        .with_abilities(&[
            AbilityDef::static_ability(
                "You may look at the top card of your library any time.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::You)),
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::MayLookAtTopOfLibrary),
                },
            ),
            AbilityDef::static_ability(
                "You may play lands and cast Bird spells from the top of your library.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::You)),
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::Rule(AppliedRuleDef::MayPlayFromTopOfLibrary {
                            // Two permissions rather than one: the printed sentence names two kinds of
                            // play, and the restriction each carries is a single action and a single
                            // predicate. Lands cost nothing beyond the land drop; a Bird pays its own
                            // mana cost, since nothing here says otherwise.
                            restriction: PlayRestrictionDef::new(
                                PlayActionMatcherDef::PlayLand,
                                ObjectPredicateDef::HasType(CardType::Land),
                            ),
                            cost: TopOfLibraryCostDef::Printed,
                        }),
                        AppliedEffectDef::Rule(AppliedRuleDef::MayPlayFromTopOfLibrary {
                            restriction: PlayRestrictionDef::new(
                                PlayActionMatcherDef::CastSpell,
                                ObjectPredicateDef::Subtype("Bird"),
                            ),
                            cost: TopOfLibraryCostDef::Printed,
                        }),
                    ]),
                },
            ),
            // The Chocobo itself is a Bird, so a second one doubles the first one's
            // arrival trigger -- and two of them double everything twice.
            AbilityDef::static_ability(
                "If a land or Bird you control entering the battlefield causes a triggered ability of a \
                 permanent you control to trigger, that ability triggers an additional time.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::You)),
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::TriggersAnAdditionalTime(
                        &AdditionalTriggerDef {
                            // A land or a Bird, and yours either way.
                            entering: ObjectPredicateDef::All(&[
                                ObjectPredicateDef::AnyOf(&[
                                    ObjectPredicateDef::HasType(CardType::Land),
                                    ObjectPredicateDef::Subtype("Bird"),
                                ]),
                                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                            ]),
                            permanent: ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        },
                    )),
                },
            ),
        ]),
);

// FIN 581 — Astrologian's Planisphere
pub(in crate::card::sets) static ASTROLOGIAN_S_PLANISPHERE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a0f6e2d7-58b5-4a7d-8c42-e25185cd173f"),
    "Astrologian's Planisphere",
    crate::card::CardArt::new("a0f6e2d7-58b5-4a7d-8c42-e25185cd173f", "Josephine Chang"),
    crate::card::CardSet::FinalFantasy,
    // Two mana for a 1/1 that grows on the turns a blue deck was having
    // anyway, and an Equipment left over when it dies.
    CardRules::new_artifact(mana_cost!("{1}{U}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::job_select(),
            AbilityDef::static_ability(
                "Equipped creature is a Wizard in addition to its other types and has \"Whenever you \
                 cast a noncreature spell and whenever you draw your third card each turn, put a +1/+1 \
                 counter on this creature.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::Characteristic(CharacteristicOperationDef::CreatureTypes(
                            SetOperationDef::Add(CreatureTypeSetDef::named(&["Wizard"])),
                        )),
                        // Granted to the equipped creature, so "this creature" is the creature
                        // rather than the Equipment: the counter goes where the ability lives.
                        AppliedEffectDef::add_ability(&AbilityDef::triggered(
                            "Whenever you cast a noncreature spell and whenever you draw your third card each turn, put \
                             a +1/+1 counter on this creature.",
                            // Two events, one clause, one counter each: a noncreature spell, and the
                            // third card of the turn however it was drawn. The Hero's own draw step
                            // counts toward the third, which is why the card wants a turn with two
                            // cantrips in it rather than a big draw spell.
                            TriggerEventDef::AnyOf(&[
                                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                                    ObjectPredicateDef::NoncreatureSpell,
                                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                                ])),
                                TriggerEventDef::DrewCard(DrawEventMatcherDef::nth_each_turn(PlayerRelation::You, 3)),
                            ]),
                            EffectDef::AddCounters {
                                object: EffectRecipientDef::Source,
                                kind: CounterKind::PlusOnePlusOne,
                                amount: ValueDef::Constant(1),
                            },
                        )),
                    ]),
                },
            ),
            // The flavour name in front of the cost is the whole of what "Diana —"
            // adds: it is an ordinary equip ability underneath.
            abilities::equip(&[CostDef::Mana(mana_cost!("{2}"))], "Diana — Equip {2}"),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &CECIL_DARK_KNIGHT,
    &RESENTFUL_REVELATION,
    &SUPLEX,
    &TIFA_LOCKHART,
    &VIVI_ORNITIER,
    &STARTING_TOWN,
    &TRAVELING_CHOCOBO,
    &ASTROLOGIAN_S_PLANISPHERE,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
