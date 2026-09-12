//! Eldritch Moon cards cataloged for the Vintage Cube pool.

use crate::card::BattlefieldEntryModificationDef;
use crate::card::CardSupertype;
use crate::card::ManaColor;
use crate::card::ReplacementEffectDef;
use crate::card::ReplacementEventDef;
use crate::card::SpellResolutionDestinationDef;
use crate::card::SumValueDef;
use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AppliedEffectDef;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseDef;
use crate::card::ControlDurationDef;
use crate::card::CostDef;
use crate::card::CostQuantityDef;
use crate::card::CounterKind;
use crate::card::DiscardFollowUpDef;
use crate::card::DiscardSelectionDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::InstalledTriggerDef;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectSetCountConditionDef;
use crate::card::ObjectSetDef;
use crate::card::ObjectSetPredicateDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SacrificedAmountDef;
use crate::card::SubtypeDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::ids::ParentBinding;
use crate::ids::TargetIndex;
use crate::mana_cost;

pub const ESCALATE: crate::card::MechanicId = crate::card::MechanicId::from_name("mtg:escalate");

/// Choose one or more modes, paying the listed cost for each beyond the first.
///
/// # Panics
///
/// Panics if the mode list is empty or contains more than 255 modes.
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub const fn escalate(
    text: &'static str,
    cost: CostDef,
    modes: &'static [AbilityDef],
) -> AbilityDef {
    assert!(!modes.is_empty() && modes.len() <= u8::MAX as usize);
    AbilityDef::defined(
        text,
        crate::card::DeclarativeAbilityDef::Spell(crate::card::SpellAbilityDef::Modal(
            crate::card::ModalSpellDef::new(modes, 1, modes.len() as u8, false)
                .with_additional_cost(
                    cost,
                    CostQuantityDef::Subtract(
                        &CostQuantityDef::ModeCount,
                        &CostQuantityDef::Fixed(1),
                    ),
                )
                .with_selection_text("Choose one or more —"),
        )),
        EffectDef::None,
    )
    .labeled(ESCALATE)
}

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "EMN",
    slug: "eldritch-moon",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// EMN 6 — Emrakul, the Promised End
// Audit: unsupported — The engine has no control-an-opponent-during-their-next-turn procedure or binding for the extra turn after that controlled turn.
pub(in crate::card::sets) static EMRAKUL_THE_PROMISED_END_6: CardRecord = CardRecord::new(
    "Emrakul, the Promised End",
    "8d74a469-c71d-4773-99d3-5456b31df424",
    "Jaime Jones",
    crate::card::CardRules::unsupported(),
);

// EMN 7 — Eternal Scourge
// Audit: unsupported — There is no intrinsic cast-from-exile permission. Exile-play effects grant permission when they move a card, which cannot authorize this card after any arbitrary path into exile.
pub(in crate::card::sets) static ETERNAL_SCOURGE_7: CardRecord = CardRecord::new(
    "Eternal Scourge",
    "13ce52f5-6d49-4d44-a3d7-925340de8406",
    "Winona Nelson",
    crate::card::CardRules::unsupported(),
);

// EMN 13 — Blessed Alliance
pub(in crate::card::sets) static BLESSED_ALLIANCE: CardRecord = CardRecord::new(
    "Blessed Alliance",
    "b5805eab-9a32-4c0c-9015-7bdb74ad7634",
    "Johann Bodin",
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(escalate(
        "Escalate {2} (Pay this cost for each mode chosen beyond the first.)",
        CostDef::pay_mana(mana_cost!("{2}")),
        &[
            AbilityDef::spell_with_targets(
                "Target player gains 4 life.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(4),
                },
            ),
            AbilityDef::spell_with_targets(
                "Untap up to two target creatures.",
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    2,
                )],
                EffectDef::Untap {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::spell_with_targets(
                "Target opponent sacrifices an attacking creature of their choice.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                )],
                EffectDef::SacrificeOfChoice {
                    player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Attacking,
                    ]),
                    count: ValueDef::Constant(1),
                    then: None,
                    amount: SacrificedAmountDef::Power,
                    otherwise: None,
                    optional: false,
                },
            ),
        ],
    )),
);

// EMN 14 — Borrowed Grace
pub(in crate::card::sets) static BORROWED_GRACE: CardRecord = CardRecord::new(
    "Borrowed Grace",
    "f0067567-3434-4c12-9d4d-04ffc98d012c",
    "Volkan Baǵa",
    CardRules::new_instant(mana_cost!("{2}{W}")).with_ability(escalate(
        "Escalate {1}{W} (Pay this cost for each mode chosen beyond the first.)",
        CostDef::pay_mana(mana_cost!("{1}{W}")),
        &[
            AbilityDef::spell(
                "Creatures you control get +2/+0 until end of turn.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::spell(
                "Creatures you control get +0/+2 until end of turn.",
                EffectDef::Apply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(0),
                        ValueDef::Constant(2),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ],
    )),
);

// EMN 17 — Collective Effort
pub(in crate::card::sets) static COLLECTIVE_EFFORT: CardRecord = CardRecord::new(
    "Collective Effort",
    "d85a6369-c07f-47d5-8448-72d8ec7e7898",
    "Eric Deschamps",
CardRules::new_sorcery(mana_cost!("{1}{W}{W}")).with_ability(
        escalate(
            "Escalate—Tap an untapped creature you control. (Pay this cost for each mode chosen beyond the first.)",
            CostDef::tap(
                ObjectPredicateDef::HasType(CardType::Creature),
                CostQuantityDef::Fixed(1),
            ),
            &[
                AbilityDef::destroy_target(
                    "Destroy target creature with power 4 or greater.",
                    &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::PowerAtLeast(4),
                    ]))
),
                AbilityDef::destroy_target(
                    "Destroy target enchantment.",
                    &AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(
                        CardType::Enchantment,
                    ))
),
                AbilityDef::spell_with_targets(
                    "Put a +1/+1 counter on each creature target player controls.",
                    &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(
                        PlayerRelation::Any,
                    ))],
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::objects_controlled_by_target(
                            ObjectPredicateDef::HasType(CardType::Creature),
                            TargetIndex::PRIMARY,
                        ),
                        kind: CounterKind::PlusOnePlusOne,
                        amount: ValueDef::Constant(1),
                    },
                ),
            ],
        ),
    ),
);

// EMN 37 — Providence
pub(in crate::card::sets) static PROVIDENCE: CardRecord = CardRecord::new(
    "Providence",
    "2e5edd8d-8e10-4414-a326-95a672dfcff7",
    "Zack Stella",
CardRules::new_sorcery(mana_cost!("{5}{W}{W}")).with_abilities(&[
        AbilityDef::opening_hand_reveal(
            "You may reveal this card from your opening hand. If you do, at the beginning of the first upkeep, your life total becomes 26.",
            EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                "At the beginning of the first upkeep, your life total becomes 26.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::Upkeep,
                    player: PlayerRelation::Any,
                },
                EffectDef::SetLifeTotal {
                    recipient: EffectRecipientDef::Controller,
                    total: ValueDef::Constant(26),
                },
            ))),
        ),
        AbilityDef::spell(
            "Your life total becomes 26.",
            EffectDef::SetLifeTotal {
                recipient: EffectRecipientDef::Controller,
                total: ValueDef::Constant(26),
            },
        ),
    ]),
);

// EMN 40 — Selfless Spirit
pub(in crate::card::sets) static SELFLESS_SPIRIT_40: CardRecord = CardRecord::new(
    "Selfless Spirit",
    "a4624976-3773-4a1e-b725-5f6efce147a5",
    "Seb McKinnon",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Spirit", "Cleric"], 2, 1).with_abilities(&[
        abilities::flying(),
        AbilityDef::activated(
            "Sacrifice this creature: Creatures you control gain indestructible until end of turn.",
            &[CostDef::SacrificeSource],
            EffectDef::Apply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// EMN 46 — Thalia, Heretic Cathar
pub(in crate::card::sets) static THALIA_HERETIC_CATHAR_46: CardRecord = CardRecord::new(
    "Thalia, Heretic Cathar",
    "ab0cee38-5e24-49d0-870c-22843ed4e101",
    "Magali Villeneuve",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Soldier"], 3, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::first_strike(),
            AbilityDef::replacement_for(
                "Creatures and nonbasic lands your opponents control enter tapped.",
                ReplacementEventDef::ObjectEntersBattlefield {
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Land),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(
                                CardSupertype::Basic,
                            )),
                        ]),
                    ]),
                    controller: PlayerRelation::Opponent,
                    cast: None,
                },
                ReplacementEffectDef::ModifyBattlefieldEntry(
                    BattlefieldEntryModificationDef::Tapped,
                ),
            ),
        ]),
);

// EMN 55 — Displace
pub(in crate::card::sets) static DISPLACE: CardRecord = CardRecord::new(
    "Displace",
    "8ab850c5-6f5e-41b7-ab52-094579caca12",
    "Clint Cearley",
// Two arrival triggers at instant speed, and the exile also answers
    // whatever is pointed at them in the meantime.
    CardRules::new_instant(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Exile up to two target creatures you control, then return those cards to the battlefield under their owner's control.",
        &[AbilityTargetDef::up_to(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            },
            2,
        )],
        // Exiled linked to this spell and returned by the same resolution,
        // so the cards come back as new objects with their triggers armed.
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
    )),
);

// EMN 65 — Imprisoned in the Moon
// Audit: unsupported — Needs a static effect that replaces the attached permanent's complete card-type set, rather than adding types; full card-type replacement is outside the shared static runtime boundary.
pub(in crate::card::sets) static IMPRISONED_IN_THE_MOON: CardRecord = CardRecord::new(
    "Imprisoned in the Moon",
    "7990ebba-e9f2-4ba4-a352-e26ec81d4bed",
    "Ryan Alexander Lee",
    CardRules::unsupported(),
);

// EMN 82 — Borrowed Malevolence
pub(in crate::card::sets) static BORROWED_MALEVOLENCE: CardRecord = CardRecord::new(
    "Borrowed Malevolence",
    "a71f123e-aad9-4f3e-9f43-1d1be359affb",
    "Volkan Baǵa",
    CardRules::new_instant(mana_cost!("{B}")).with_ability(escalate(
        "Escalate {2} (Pay this cost for each mode chosen beyond the first.)",
        CostDef::pay_mana(mana_cost!("{2}")),
        &[
            AbilityDef::spell_with_targets(
                "Target creature gets +1/+1 until end of turn.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::spell_with_targets(
                "Target creature gets -1/-1 until end of turn.",
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
        ],
    )),
);

// EMN 83 — Cemetery Recruitment
pub(in crate::card::sets) static CEMETERY_RECRUITMENT: CardRecord = CardRecord::new(
    "Cemetery Recruitment",
    "3a23adea-9f4a-409c-a37d-323eee781273",
    "Kieran Yanner",
    CardRules::new_sorcery(mana_cost!("{1}{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Return target creature card from your graveyard to your hand. \
         If it's a Zombie card, draw a card.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                zones: &[ZoneKind::Graveyard],
                controller: None,
                owner: Some(PlayerRelation::You),
            },
        )],
        EffectDef::WithZoneMoveResult {
            effect: &EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
            binding: crate::Binding!("returned"),
            then: &EffectDef::IfCondition {
                condition: &TriggerConditionDef::ObjectSetCount(&ObjectSetCountConditionDef {
                    objects: &ObjectSetDef::ZoneChangeSuccessorsOfBinding(crate::Binding!(
                        "returned"
                    )),
                    predicate: ObjectSetPredicateDef::contains(&ObjectPredicateDef::Subtype(
                        SubtypeDef::Literal("Zombie"),
                    )),
                }),
                then: &abilities::draw_cards(ValueDef::Constant(1)),
            },
        },
    )]),
);

// EMN 85 — Collective Brutality
static AN_OPPONENT: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
)];

pub(in crate::card::sets) static COLLECTIVE_BRUTALITY: CardRecord = CardRecord::new(
    "Collective Brutality",
    "cb94a02f-4660-45b6-8a39-941b710cf8f3",
    "Johann Bodin",
// Two mana that answers three different decks, and the escalate cost is
    // paid in the cards those decks least want you to have anyway.
    CardRules::new_sorcery(mana_cost!("{1}{B}")).with_ability(
        escalate(
            "Escalate—Discard a card. (Pay this cost for each mode chosen beyond the \
             first.)",
            // One mode is free; taking all three costs two discarded cards.
            CostDef::discard(ObjectPredicateDef::Any),
            // Each mode declares its own target slot, so a Brutality that takes two
            // modes points at two things.
            &[
                AbilityDef::spell_with_targets(
                    "Target opponent reveals their hand. You choose an instant or sorcery card from it. That \
                     player discards that card.",
                    &AN_OPPONENT,
                    EffectDef::Sequence(&abilities::reveal_hand_and_discard_chosen_card(
                        PlayerRefDef::Target(TargetIndex::PRIMARY),
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Instant),
                            ObjectPredicateDef::HasType(CardType::Sorcery),
                        ]),
                    )),
                ),
                AbilityDef::spell_with_targets(
                    "Target creature gets -2/-2 until end of turn.",
                    &[AbilityTargetDef::exactly_one_permanent(
                        ObjectPredicateDef::HasType(CardType::Creature),
                    )],
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        effect: AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(-2),
                            ValueDef::Constant(-2),
                        ),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ),
                AbilityDef::spell_with_targets(
                    "Target opponent loses 2 life and you gain 2 life.",
                    &AN_OPPONENT,
                    EffectDef::Sequence(&[
                        EffectDef::LoseLife {
                            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            amount: ValueDef::Constant(2),
                        },
                        EffectDef::GainLife {
                            recipient: EffectRecipientDef::Controller,
                            amount: ValueDef::Constant(2),
                        },
                    ]),
                ),
            ],
        ),
    ),
);

// EMN 111 — Voldaren Pariah // Abolisher of Bloodlines
// Audit: unsupported — Madness requires a discard-to-exile replacement and a linked triggered cast-or-graveyard procedure; no current alternative-cast procedure implements it.
pub(in crate::card::sets) static VOLDAREN_PARIAH_ABOLISHER_OF_BLOODLINES_111: CardRecord =
    CardRecord::new(
        "Voldaren Pariah // Abolisher of Bloodlines",
        "25baac6c-5bb4-4ecc-b1d5-fced52087bd9",
        "James Ryman",
        crate::card::CardRules::unsupported(),
    );

// EMN 116 — Alchemist's Greeting
// Audit: unsupported — Madness requires a discard-to-exile replacement and a linked triggered cast-or-graveyard procedure; no current alternative-cast procedure implements it.
pub(in crate::card::sets) static ALCHEMIST_S_GREETING_116: CardRecord = CardRecord::new(
    "Alchemist's Greeting",
    "8f33aaa1-cbaa-40a9-889e-3eca26b3a549",
    "Jakub Kasper",
    crate::card::CardRules::unsupported(),
);

// EMN 121 — Borrowed Hostility
pub(in crate::card::sets) static BORROWED_HOSTILITY: CardRecord = CardRecord::new(
    "Borrowed Hostility",
    "dd91a194-6043-4c2d-afc8-427c38996ef4",
    "Volkan Baǵa",
    CardRules::new_instant(mana_cost!("{R}")).with_ability(escalate(
        "Escalate {3} (Pay this cost for each mode chosen beyond the first.)",
        CostDef::pay_mana(mana_cost!("{3}")),
        &[
            AbilityDef::spell_with_targets(
                "Target creature gets +3/+0 until end of turn.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(3),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::spell_with_targets(
                "Target creature gains first strike until end of turn.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    effect: AppliedEffectDef::add_ability(&abilities::first_strike()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ],
    )),
);

// EMN 123 — Collective Defiance
pub(in crate::card::sets) static COLLECTIVE_DEFIANCE: CardRecord = CardRecord::new(
    "Collective Defiance",
    "8960883f-3813-412b-9a5b-f8cf8d566fac",
    "Kieran Yanner",
    CardRules::new_sorcery(mana_cost!("{1}{R}{R}")).with_ability(escalate(
        "Escalate {1} (Pay this cost for each mode chosen beyond the first.)",
        CostDef::pay_mana(mana_cost!("{1}")),
        &[
            AbilityDef::spell_with_targets(
                "Target player discards all the cards in their hand, then draws that many cards.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::Discard {
                    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    amount: ValueDef::Constant(i32::MAX),
                    selection: DiscardSelectionDef::RecipientChooses,
                    then: Some(DiscardFollowUpDef {
                        counted: ObjectPredicateDef::Any,
                        bound: Some(ParentBinding),
                        effect: &EffectDef::DrawCards {
                            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            amount: ValueDef::BoundObjectCount(ParentBinding),
                        },
                    }),
                },
            ),
            AbilityDef::spell_with_targets(
                "This spell deals 4 damage to target creature.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(4),
                ),
            ),
            AbilityDef::spell_with_targets(
                "This spell deals 3 damage to target opponent or planeswalker.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Opponent),
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(3),
                ),
            ),
        ],
    )),
);

// EMN 126 — Distemper of the Blood
// Audit: unsupported — Madness requires a discard-to-exile replacement and a linked triggered cast-or-graveyard procedure; no current alternative-cast procedure implements it.
pub(in crate::card::sets) static DISTEMPER_OF_THE_BLOOD_126: CardRecord = CardRecord::new(
    "Distemper of the Blood",
    "d0ad2acb-073b-4a98-be8c-2ea39ca85496",
    "Ben Maier",
    crate::card::CardRules::unsupported(),
);

// EMN 131 — Harmless Offering
pub(in crate::card::sets) static HARMLESS_OFFERING: CardRecord = CardRecord::new(
    "Harmless Offering",
    "f8f3cc4f-7943-4025-b332-b40653b13014",
    "Howard Lyon",
    CardRules::new_sorcery(mana_cost!("{2}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target opponent gains control of target permanent you control.",
        &[
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::Player(PlayerRelation::Opponent)),
            AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::Any,
                zones: &[ZoneKind::Battlefield],
                controller: Some(PlayerRelation::You),
                owner: None,
            }),
        ],
        EffectDef::gain_control(
            EffectRecipientDef::Target(TargetIndex(1)),
            PlayerRefDef::Target(TargetIndex::PRIMARY),
            ControlDurationDef::Indefinitely,
        ),
    )]),
);

// EMN 140 — Savage Alliance
pub(in crate::card::sets) static SAVAGE_ALLIANCE: CardRecord = CardRecord::new(
    "Savage Alliance",
    "b5255da8-8511-48a7-98e5-ba43ca6e8681",
    "Johann Bodin",
    CardRules::new_instant(mana_cost!("{2}{R}")).with_ability(escalate(
        "Escalate {1} (Pay this cost for each mode chosen beyond the first.)",
        CostDef::pay_mana(mana_cost!("{1}")),
        &[
            AbilityDef::spell_with_targets(
                "Creatures target player controls gain trample until end of turn.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::objects_controlled_by_target(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        TargetIndex::PRIMARY,
                    ),
                    effect: AppliedEffectDef::add_ability(&abilities::trample()),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
            AbilityDef::spell_with_targets(
                "This spell deals 2 damage to target creature.",
                &[AbilityTargetDef::exactly_one_permanent(
                    ObjectPredicateDef::HasType(CardType::Creature),
                )],
                EffectDef::damage(
                    EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    ValueDef::Constant(2),
                ),
            ),
            AbilityDef::spell_with_targets(
                "This spell deals 1 damage to each creature target opponent controls.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                )],
                EffectDef::damage(
                    EffectRecipientDef::objects_controlled_by_target(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        TargetIndex::PRIMARY,
                    ),
                    ValueDef::Constant(1),
                ),
            ),
        ],
    )),
);

// EMN 155 — Eldritch Evolution
pub(in crate::card::sets) static ELDRITCH_EVOLUTION_155: CardRecord = CardRecord::new(
    "Eldritch Evolution",
    "efcb00e5-2caa-45c8-ad19-05d45c683d16",
    "Jason Rainville",
    CardRules::new_sorcery(mana_cost!("{1}{G}{G}")).with_abilities(&[
AbilityDef::spell_with_additional_cost("As an additional cost to cast this spell, sacrifice a creature.\nSearch your library for a creature card with mana value X or less, where X is 2 plus the sacrificed creature's mana value. Put that card onto the battlefield, then shuffle. Exile Eldritch Evolution.", &[], CostDef::sacrifice_permanent(ObjectPredicateDef::HasType(CardType::Creature)), EffectDef::SearchZone { player: EffectRecipientDef::Controller, source: ZoneKind::Library, object: ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::ManaValueAtMostValue(ValueDef::Sum(&SumValueDef { left: ValueDef::SacrificedManaValue, right: ValueDef::Constant(2) }))]), minimum: 0, maximum: ValueDef::Constant(1), reveal: true, destination: ZoneKind::Battlefield, placement: ZonePlacement::Top, shuffle: true, enters_tapped: false, attachment: None, binding: None, then: None }).with_resolution_destination(SpellResolutionDestinationDef::Exile)
]),
);

// EMN 160 — Grapple with the Past
pub(in crate::card::sets) static GRAPPLE_WITH_THE_PAST: CardRecord = CardRecord::new(
    "Grapple with the Past",
    "d44a77a6-e8a1-4706-886f-8ab3af56b342",
    "Howard Lyon",
// The mill happens first, so the three cards it just buried are part of
    // what the choice may take back.
    CardRules::new_instant(mana_cost!("{1}{G}")).with_ability(AbilityDef::spell(
        "Mill three cards, then you may return a creature or land card from your graveyard to your hand.",
        EffectDef::Sequence(&[
            EffectDef::Mill {
                player: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
            // Chosen as this resolves rather than targeted, which is why the
            // spell can be cast with an empty graveyard at all.
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &EffectDef::Choose(ChooseDef {
                    binding: ObjectChoiceBindingDef::Objects(ParentBinding),
                    unchosen: None,
                    chooser: PlayerRefDef::EffectController,
                    candidates: ObjectSetDef::Query(ObjectQueryDef::owned_by(
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Land),
                        ]),
                        &[ZoneKind::Graveyard],
                        PlayerSetDef::Related(PlayerRelation::You),
                    )),
                    exclude: None,
                    minimum: 1,
                    maximum: 1,
                    visibility: ChoiceVisibilityDef::Public,
                    then: &EffectDef::move_to_zone(
                        EffectRecipientDef::objects(ObjectSetDef::Binding(
                            ParentBinding,
                        )),
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                }),
            },
        ]),
    )),
);

// EMN 181 — Bloodhall Priest
// Audit: unsupported — Madness requires a discard-to-exile replacement and a linked triggered cast-or-graveyard procedure; no current alternative-cast procedure implements it.
pub(in crate::card::sets) static BLOODHALL_PRIEST_181: CardRecord = CardRecord::new(
    "Bloodhall Priest",
    "c4824cca-0039-4486-be8f-650dac2c8e9f",
    "Mark Winters",
    crate::card::CardRules::unsupported(),
);

// EMN 189 — Spell Queller
// Audit: unsupported — The immediate free-cast operations always offer the cast to the resolving ability controller. They cannot offer the linked exiled card to its owner, who may be a different player.
pub(in crate::card::sets) static SPELL_QUELLER_189: CardRecord = CardRecord::new(
    "Spell Queller",
    "9b76bcd4-580a-4435-afe9-290940b1837f",
    "Adam Paquette",
    crate::card::CardRules::unsupported(),
);

// EMN 203 — Geier Reach Sanitarium
pub(in crate::card::sets) static GEIER_REACH_SANITARIUM_203: CardRecord = CardRecord::new(
    "Geier Reach Sanitarium",
    "96093739-fedc-4d8f-a29d-0e57f571e5a9",
    "Cliff Childs",
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::tap_for(ManaColor::Colorless),
            AbilityDef::activated(
                "{2}, {T}: Each player draws a card, then discards a card.",
                &[CostDef::Mana(mana_cost!("{2}")), CostDef::TapSource],
                EffectDef::Sequence(&[
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::EachPlayer,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::Discard {
                        recipient: EffectRecipientDef::EachPlayer,
                        amount: ValueDef::Constant(1),
                        selection: DiscardSelectionDef::RecipientChooses,
                        then: None,
                    },
                ]),
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &EMRAKUL_THE_PROMISED_END_6,
    &ETERNAL_SCOURGE_7,
    &BLESSED_ALLIANCE,
    &BORROWED_GRACE,
    &COLLECTIVE_EFFORT,
    &PROVIDENCE,
    &SELFLESS_SPIRIT_40,
    &THALIA_HERETIC_CATHAR_46,
    &DISPLACE,
    &IMPRISONED_IN_THE_MOON,
    &BORROWED_MALEVOLENCE,
    &CEMETERY_RECRUITMENT,
    &COLLECTIVE_BRUTALITY,
    &VOLDAREN_PARIAH_ABOLISHER_OF_BLOODLINES_111,
    &ALCHEMIST_S_GREETING_116,
    &BORROWED_HOSTILITY,
    &COLLECTIVE_DEFIANCE,
    &DISTEMPER_OF_THE_BLOOD_126,
    &HARMLESS_OFFERING,
    &SAVAGE_ALLIANCE,
    &ELDRITCH_EVOLUTION_155,
    &GRAPPLE_WITH_THE_PAST,
    &BLOODHALL_PRIEST_181,
    &SPELL_QUELLER_189,
    &GEIER_REACH_SANITARIUM_203,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
