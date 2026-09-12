//! Ixalan cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::CostDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::InstalledTriggerDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SubtypeDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::ids::ParentBinding;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "XLN",
    slug: "ixalan",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

const TREASURE_TOKEN: TokenCharacteristics =
    crate::card::tokens::treasure().with_art(CardArt::new(
        "720f3e68-84c0-462e-a0d1-90236ccc494a",
        "Florian de Gesincourt",
    ));

// XLN 6 — Bishop's Soldier
pub(in crate::card::sets) static BISHOP_S_SOLDIER: CardRecord = CardRecord::new(
    "Bishop's Soldier",
    "d954677c-2de6-440a-90d0-bab2e0c8b4af",
    "Scott Murphy",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Vampire", "Soldier"], 2, 2)
        .with_abilities(&[abilities::lifelink()]),
);

// XLN 34 — Settle the Wreckage
// Audit: unsupported — Needs a completed exile result that counts only attackers actually moved to exile, including replacement effects. WithZoneMoveResult binds attempted recipients and cannot filter their successors by destination for the search maximum.
pub(in crate::card::sets) static SETTLE_THE_WRECKAGE: CardRecord = CardRecord::new(
    "Settle the Wreckage",
    "9cbd346e-098a-4cf6-a72f-468376fd2e8f",
    "Dimitar Marinski",
    CardRules::unsupported(),
);

// XLN 41 — Territorial Hammerskull
pub(in crate::card::sets) static TERRITORIAL_HAMMERSKULL: CardRecord = CardRecord::new(
    "Territorial Hammerskull",
    "af5a237a-31e7-43ee-8d47-3eb12dd1a60c",
    "Lars Grant-West",
    // The tap happens on the declaration, so it clears a blocker before
    // blockers are chosen: a 2/3 that attacks as if it were much larger.
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Dinosaur"], 2, 3).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever this creature attacks, tap target creature an opponent controls.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Tap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
    ),
);

// XLN 48 — Chart a Course
// Audit: unsupported — Needs per-player attack history retained after attacking creatures leave or change controllers; current object predicates only identify individual permanents that attacked.
pub(in crate::card::sets) static CHART_A_COURSE: CardRecord = CardRecord::new(
    "Chart a Course",
    "98291778-2ec2-47e2-ac99-5f8cfbb3cf24",
    "James Ryman",
    CardRules::unsupported(),
);

// XLN 53 — Dive Down
pub(in crate::card::sets) static DIVE_DOWN: CardRecord = CardRecord::new(
    "Dive Down",
    "b33e493e-1aef-43b3-9716-52158b002430",
    "Magali Villeneuve",
    CardRules::new_instant(mana_cost!("{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature you control gets +0/+3 and gains hexproof \
         until end of turn. (It can't be the target of spells or \
         abilities your opponents control.)",
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
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(0),
                    ValueDef::Constant(3),
                ),
                AppliedEffectDef::add_ability(&abilities::hexproof()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// XLN 71 — River's Rebuke
pub(in crate::card::sets) static RIVER_S_REBUKE: CardRecord = CardRecord::new(
    "River's Rebuke",
    "fda8ef30-bbfa-4857-9750-0dd0def8b13f",
    "Raymond Swanland",
    CardRules::new_sorcery(mana_cost!("{4}{U}{U}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Return all nonland permanents target player controls to their \
             owner's hand.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Any),
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::objects(ObjectSetDef::Query(ObjectQueryDef::controlled_by(
                    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    &[ZoneKind::Battlefield],
                    PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
                ))),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
        ),
    ]),
);

// XLN 84 — Storm Fleet Spy
// Audit: unsupported — Needs per-player attack history retained after attacking creatures leave or change controllers; current object predicates only identify individual permanents that attacked.
pub(in crate::card::sets) static STORM_FLEET_SPY: CardRecord = CardRecord::new(
    "Storm Fleet Spy",
    "f7c33ef4-60bb-4f95-92a5-7abedaac6767",
    "Scott Murphy",
    CardRules::unsupported(),
);

// XLN 110 — Kitesail Freebooter
pub(in crate::card::sets) static KITESAIL_FREEBOOTER: CardRecord = CardRecord::new(
    "Kitesail Freebooter",
    "f62fd592-4910-417d-a500-e7029f3d119f",
    "Dan Murayama Scott",
CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Pirate"], 1, 2)
        .with_abilities(&[
            abilities::flying(),
            abilities::enters_trigger_with_targets(
                "When this creature enters, target opponent reveals their hand. You choose a noncreature, nonland card from it. Exile that card until this creature leaves the battlefield.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                )],
                EffectDef::Sequence(&abilities::reveal_hand_and_choose_card(
                    PlayerRefDef::Target(TargetIndex::PRIMARY),
                    // Neither a creature nor a land: the Freebooter takes the answer, not the
                    // threat, which is what separates it from the Sculler.
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    ]),
                    &EffectDef::Sequence(&[
                        EffectDef::ExileLinkedToSource {
                            until_source_leaves: true,
                            object: EffectRecipientDef::object(ObjectRefDef::Binding(ParentBinding)),
                            face_down: false,
                            then: None,
                        },
                        // "Until this creature leaves the battlefield" is one printed ability, so
                        // the return is a delayed trigger installed by the same resolution rather
                        // than a second clause the card does not print.
                        EffectDef::InstallTrigger(InstalledTriggerDef::once(&AbilityDef::triggered(
                            "When this creature leaves the battlefield, return the exiled card to its owner's hand.",
                            TriggerEventDef::zone_changed(
                                ObjectPredicateDef::Source,
                                Some(ZoneKind::Battlefield),
                                None,
                            ),
                            EffectDef::ReturnLinkedExiles {
                                object: ObjectPredicateDef::Any,
                                counters: None,
                                zone: ZoneKind::Hand,
                                grant: None,
                                controller: None,
                                transformed: false,
                            },
                        ))),
                    ]),
                )),
            ),
        ]),
);

// XLN 123 — Skulduggery
pub(in crate::card::sets) static SKULDUGGERY: CardRecord = CardRecord::new(
    "Skulduggery",
    "ba30343b-1637-490f-810e-d614219789e3",
    "Deruchenko Alexander",
    CardRules::new_instant(mana_cost!("{B}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Until end of turn, target creature you control gets +1/+1 and \
         target creature an opponent controls gets -1/-1.",
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
                controller: Some(PlayerRelation::Opponent),
                owner: None,
            }),
        ],
        EffectDef::Sequence(&[
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex(1)),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(-1),
                    ValueDef::Constant(-1),
                ),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )]),
);

// XLN 191 — Growing Rites of Itlimoc // Itlimoc, Cradle of the Sun
pub(in crate::card::sets) static GROWING_RITES_OF_ITLIMOC: CardRecord = CardRecord::new_dfc(
    "Growing Rites of Itlimoc // Itlimoc, Cradle of the Sun",
    "b3b87bfc-f97f-4734-94f6-e3e2f335fc4d",
    "Grzegorz Rutkowski",
    &[
        (
            "Growing Rites of Itlimoc",
            CardRules::new_enchantment(mana_cost!("{2}{G}"))
                .with_supertype(crate::card::CardSupertype::Legendary)
                .with_abilities(&[
                    abilities::enters_trigger(
                        "When Growing Rites of Itlimoc enters, look at the top four \
                         cards of your library. You may reveal a creature card from \
                         among them and put it into your hand. Put the rest on the \
                         bottom of your library in any order.",
                        abilities::look_at_top_cards_reveal_choice_to_hand_rest_bottom(
                            ValueDef::Constant(4),
                            ObjectPredicateDef::HasType(CardType::Creature),
                            0,
                            1,
                        ),
                    ),
                    AbilityDef::triggered_if(
                        "At the beginning of your end step, if you control four or \
                         more creatures, transform Growing Rites of Itlimoc.",
                        TriggerEventDef::StepBegins {
                            step: crate::card::TurnStepDef::End,
                            player: PlayerRelation::You,
                        },
                        &crate::card::TriggerConditionDef::ObjectCount {
                            query: crate::card::ObjectQueryDef::matching(
                                ObjectPredicateDef::HasType(CardType::Creature),
                                &[ZoneKind::Battlefield],
                                PlayerRelation::You,
                            ),
                            comparison: crate::card::ComparisonDef::GreaterOrEqual,
                            amount: 4,
                        },
                        EffectDef::Transform {
                            object: EffectRecipientDef::Source,
                        },
                    ),
                ]),
        ),
        (
            "Itlimoc, Cradle of the Sun",
            CardRules::new_land(&[])
                .with_supertype(crate::card::CardSupertype::Legendary)
                .with_abilities(&[
                    abilities::tap_for(crate::card::ManaColor::Green),
                    abilities::tap_for_mana(
                        "{T}: crate::card::Add {G} for each creature you control.",
                        crate::card::AddManaEffectDef::one(crate::card::ManaColor::Green)
                            .with_variable_amount(ValueDef::CountMatchingObjects(
                                &crate::card::ObjectQueryDef::matching(
                                    ObjectPredicateDef::HasType(CardType::Creature),
                                    &[ZoneKind::Battlefield],
                                    PlayerRelation::You,
                                ),
                            )),
                    ),
                ]),
        ),
    ],
);

// XLN 194 — Jade Guardian
pub(in crate::card::sets) static JADE_GUARDIAN: CardRecord = CardRecord::new(
    "Jade Guardian",
    "aca83e48-6e32-477f-8714-6103e77c06df",
    "Chris Seaman",
    // Hexproof is what makes the counter safe to spend on itself: a 3/3 the
    // opponent cannot answer with a spell.
    CardRules::new_creature(mana_cost!("{3}{G}"), &["Merfolk", "Shaman"], 2, 2).with_abilities(&[
        abilities::hexproof(),
        abilities::enters_trigger_with_targets(
            "When this creature enters, put a +1/+1 counter on target Merfolk you control.",
            // It is itself a Merfolk, so a board with no other one still has
            // a legal target.
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Merfolk")),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::You),
                    owner: None,
                },
            )],
            EffectDef::AddCounters {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// XLN 198 — New Horizons
pub(in crate::card::sets) static NEW_HORIZONS: CardRecord = CardRecord::new(
    "New Horizons",
    "15b12c75-1248-4c81-90cf-28e341a885cf",
    "Noah Bradley",
    CardRules::new_enchantment(mana_cost!("{2}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_land(),
            abilities::enters_trigger_with_targets(
                "When this Aura enters, put a +1/+1 counter on target creature \
                 you control.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::HasType(CardType::Creature),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted land has \"{T}: Add two mana of any one color.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&AbilityDef::activated_mana(
                        "{T}: Add two mana of any one color.",
                        &[CostDef::TapSource],
                        EffectDef::AddMana(AddManaEffectDef::any_color().with_amount(2)),
                    )),
                },
            ),
        ]),
);

// XLN 222 — Gishath, Sun's Avatar
pub(in crate::card::sets) static GISHATH_SUN_S_AVATAR: CardRecord = CardRecord::new(
    "Gishath, Sun's Avatar",
    "7335e500-342d-476d-975c-817512e6e3d6",
    "Zack Stella",
    CardRules::new_creature(mana_cost!("{5}{R}{G}{W}"), &["Dinosaur", "Avatar"], 7, 6)
        .with_supertype(crate::card::CardSupertype::Legendary)
        .with_abilities(&[
            abilities::vigilance(),
            abilities::trample(),
            abilities::haste(),
            AbilityDef::triggered(
                "Whenever Gishath deals combat damage to a player, reveal that \
                 many cards from the top of your library. Put any number of \
                 Dinosaur creature cards from among them onto the battlefield \
                 and the rest on the bottom of your library in a random order.",
                TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
                EffectDef::ChooseCardsFromCollection(crate::card::ChooseCardsFromCollectionDef {
                    source: crate::card::ObjectCollectionSourceDef::TopCards {
                        player: PlayerRefDef::EffectController,
                        count: ValueDef::TriggerEventAmount,
                    },
                    actor: PlayerRefDef::EffectController,
                    inspection: crate::card::CollectionInspectionDef::Reveal,
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Dinosaur")),
                    ]),
                    minimum: 0,
                    maximum: 65535,
                    chosen: crate::Binding!("chosen"),
                    remainder: crate::Binding!("rest"),
                    then: &EffectDef::Sequence(&[
                        EffectDef::move_to_zone(
                            EffectRecipientDef::objects(crate::card::ObjectSetDef::Binding(
                                crate::Binding!("chosen"),
                            )),
                            ZoneKind::Battlefield,
                            crate::card::ZonePlacement::Top,
                        ),
                        EffectDef::RandomizeObjectOrder(crate::card::RandomizeObjectOrderDef {
                            input: crate::card::ObjectSetDef::Binding(crate::Binding!("rest")),
                            randomized: crate::Binding!("randomized"),
                            then: &EffectDef::MoveObjects(crate::card::MoveObjectsDef {
                                input: crate::card::ObjectSetDef::Binding(crate::Binding!(
                                    "randomized"
                                )),
                                from: Some(ZoneKind::Library),
                                zone: ZoneKind::Library,
                                placement: crate::card::ZonePlacement::Bottom,
                                moved: None,
                                then: &EffectDef::None,
                            }),
                        }),
                    ]),
                }),
            ),
        ]),
);

// XLN 242 — Pirate's Cutlass
pub(in crate::card::sets) static PIRATE_S_CUTLASS: CardRecord = CardRecord::new(
    "Pirate's Cutlass",
    "b3e7e871-19cf-486d-bacb-1499fe066974",
    "John Stanko",
    CardRules::new_artifact(mana_cost!("{3}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::enters_trigger_with_targets(
                "When this Equipment enters, attach it to target Pirate you \
                 control.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Subtype(SubtypeDef::Literal("Pirate")),
                        zones: &[ZoneKind::Battlefield],
                        controller: Some(PlayerRelation::You),
                        owner: None,
                    },
                )],
                EffectDef::Attach {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                },
            ),
            AbilityDef::static_ability(
                "Equipped creature gets +2/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(1),
                    ),
                },
            ),
            abilities::equip(&[CostDef::Mana(mana_cost!("{2}"))], "Equip {2}"),
        ]),
);

// XLN 248 — Sorcerous Spyglass
pub(in crate::card::sets) static SORCEROUS_SPYGLASS: CardRecord = CardRecord::new(
    "Sorcerous Spyglass",
    "85506a24-8d60-475c-9f43-65994caca7d4",
    "Kieran Yanner",
CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[
        AbilityDef::as_enters(
            "As this artifact enters, look at an opponent's hand, then choose any card name.",
            crate::card::ReplacementEffectDef::Sequence(&[
                crate::card::ReplacementEffectDef::LookAtHand(PlayerRelation::Opponent),
                crate::card::ReplacementEffectDef::BindOutput {
                    binding: crate::Binding!("sorcerous_spyglass_name"),
                    effect: &abilities::choose_card_name_as_enters(
                        crate::card::CardNameSetDef::AllCardNames,
                    ),
                },
            ]),
        ),
        abilities::cannot_activate_nonmana_abilities_with_name(
            "Activated abilities of sources with the chosen name can't be activated unless they're mana abilities.",
            crate::card::CardNameDef::Binding(crate::Binding!("sorcerous_spyglass_name")),
        ),
    ]),
);

// XLN 250 — Treasure Map // Treasure Cove
pub(in crate::card::sets) static TREASURE_MAP: CardRecord = CardRecord::new_dfc(
    "Treasure Map // Treasure Cove",
    "c0f9c733-0818-4a03-8f0c-a163d09e0fff",
    "Cliff Childs",
    &[
        (
            "Treasure Map",
            CardRules::new_artifact(mana_cost!("{2}")).with_abilities(&[AbilityDef::activated(
                "{1}, {T}: Scry 1. Put a landmark counter on this artifact. \
                 Then if there are three or more landmark counters on it, \
                 remove those counters, transform this artifact, and create \
                 three Treasure tokens. (They're artifacts with \"{T}, \
                 Sacrifice this token: Add one mana of any color.\")",
                &[
                    crate::card::CostDef::Mana(mana_cost!("{1}")),
                    crate::card::CostDef::TapSource,
                ],
                EffectDef::Sequence(&[
                    abilities::scry(ValueDef::Constant(1)),
                    EffectDef::AddCounters {
                        object: EffectRecipientDef::Source,
                        kind: CounterKind::named("landmark"),
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::IfCondition {
                        condition: &crate::card::TriggerConditionDef::SourceCounters {
                            kind: CounterKind::named("landmark"),
                            comparison: crate::card::ComparisonDef::GreaterOrEqual,
                            amount: 3,
                        },
                        then: &EffectDef::Sequence(&[
                            EffectDef::RemoveCounters {
                                object: EffectRecipientDef::Source,
                                kind: CounterKind::named("landmark"),
                                amount: ValueDef::CountersOnSource(CounterKind::named("landmark")),
                            },
                            EffectDef::Transform {
                                object: EffectRecipientDef::Source,
                            },
                            EffectDef::CreateToken(
                                CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                                    .with_count(ValueDef::Constant(3)),
                            ),
                        ]),
                    },
                ]),
            )]),
        ),
        (
            "Treasure Cove",
            CardRules::new_land(&[]).with_abilities(&[
                abilities::tap_for(crate::card::ManaColor::Colorless),
                AbilityDef::activated(
                    "{T}, Sacrifice a Treasure: Draw a card.",
                    &[
                        crate::card::CostDef::TapSource,
                        crate::card::CostDef::sacrifice_permanent(ObjectPredicateDef::Subtype(
                            SubtypeDef::Literal("Treasure"),
                        )),
                    ],
                    abilities::draw_cards(ValueDef::Constant(1)),
                ),
            ]),
        ),
    ],
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &BISHOP_S_SOLDIER,
    &SETTLE_THE_WRECKAGE,
    &TERRITORIAL_HAMMERSKULL,
    &CHART_A_COURSE,
    &DIVE_DOWN,
    &RIVER_S_REBUKE,
    &STORM_FLEET_SPY,
    &KITESAIL_FREEBOOTER,
    &SKULDUGGERY,
    &GROWING_RITES_OF_ITLIMOC,
    &JADE_GUARDIAN,
    &NEW_HORIZONS,
    &GISHATH_SUN_S_AVATAR,
    &PIRATE_S_CUTLASS,
    &SORCEROUS_SPYGLASS,
    &TREASURE_MAP,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
