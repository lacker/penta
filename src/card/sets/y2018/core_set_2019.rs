//! M19 card records required by supported formats.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityPredicateDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::BattlefieldEntryChoiceDestinationDef;
use crate::card::BattlefieldEntryScalarChoiceDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CharacteristicOperationDef;
use crate::card::ChooseCardsFromCollectionDef;
use crate::card::CollectionInspectionDef;
use crate::card::ComparisonDef;
use crate::card::CostDef;
use crate::card::CreateTokenDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::EmblemCharacteristics;
use crate::card::ExilePlayDurationDef;
use crate::card::KeywordAbility;
use crate::card::LAND_SUBTYPES;
use crate::card::ManaColor;
use crate::card::ObjectCollectionSourceDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectSetDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::RandomizeObjectOrderDef;
use crate::card::ReplacementChoiceDef;
use crate::card::ReplacementEffectDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::RevealObjectsDef;
use crate::card::SetOperationDef;
use crate::card::SubtypeDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
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
    code: "M19",
    slug: "core-set-2019",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// M19 13 — Herald of Faith
pub(in crate::card::sets) static HERALD_OF_FAITH: CardRecord = CardRecord::new(
    "Herald of Faith",
    "452591ca-7273-4e47-820b-3ff89697a036",
    "Tommy Arnold",
    CardRules::new_creature(mana_cost!("{3}{W}{W}"), &["Angel"], 4, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered(
            "Whenever this creature attacks, you gain 2 life.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(2),
            },
        ),
    ]),
);

// M19 22 — Leonin Vanguard
pub(in crate::card::sets) static LEONIN_VANGUARD: CardRecord = CardRecord::new(
    "Leonin Vanguard",
    "724738ad-6a9b-4ef6-b637-558645cd8151",
    "Aaron Miller",
    CardRules::new_creature(mana_cost!("{W}"), &["Cat", "Soldier"], 1, 1).with_abilities(&[
        AbilityDef::triggered_if(
            "At the beginning of combat on your turn, if you control three \
             or more creatures, this creature gets +1/+1 until end of turn \
             and you gain 1 life.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::BeginningOfCombat,
                player: PlayerRelation::You,
            },
            &TriggerConditionDef::ObjectCount {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 3,
            },
            EffectDef::Sequence(&[
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
    ]),
);

// M19 29 — Militia Bugler
pub(in crate::card::sets) static MILITIA_BUGLER: CardRecord = CardRecord::new(
    "Militia Bugler",
    "43c5bf25-937c-4e17-9ed4-b4c4579fa9dc",
    "David Gaillet",
    // The power restriction is what keeps this honest: it finds the small
    // creatures a white deck is already full of, and none of the payoffs.
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Human", "Soldier"], 2, 3).with_abilities(&[
        abilities::vigilance(),
        abilities::enters_trigger(
            "When this creature enters, look at the top four cards of your library. You may \
             reveal a creature card with power 2 or less from among them and put it into your \
             hand. Put the rest on the bottom of your library in a random order.",
            abilities::look_at_top_cards_reveal_choice_to_hand_rest_random_bottom(
                ValueDef::Constant(4),
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    // "Power 2 or less" has to be written as a strict
                    // comparison because power only reads upward here.
                    ObjectPredicateDef::PowerLessThan(ValueDef::Constant(3)),
                ]),
                0,
                1,
            ),
        ),
    ]),
);

// M19 34 — Resplendent Angel
pub(in crate::card::sets) static RESPLENDENT_ANGEL: CardRecord = CardRecord::new(
    "Resplendent Angel",
    "586854d1-edfd-4c66-873d-df459324dbfd",
    "Volkan Baǵa",
    CardRules::new_creature(mana_cost!("{1}{W}{W}"), &["Angel"], 3, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_if(
            "At the beginning of each end step, if you gained 5 or more \
             life this turn, create a 4/4 white Angel creature token with \
             flying and vigilance.",
            crate::card::TriggerEventDef::StepBegins {
                step: crate::card::TurnStepDef::End,
                player: PlayerRelation::Any,
            },
            &crate::card::TriggerConditionDef::ValueComparison(&crate::card::ValueComparisonDef {
                left: ValueDef::LifeGainedThisTurn(PlayerRelation::You),
                comparison: crate::card::ComparisonDef::GreaterOrEqual,
                right: ValueDef::Constant(5),
            }),
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::creature(&["Angel"], &[crate::card::ManaColor::White], 4, 4)
                    .with_abilities(&[abilities::flying(), abilities::vigilance()]),
            ))),
        ),
        AbilityDef::activated(
            "{3}{W}{W}{W}: Until end of turn, this creature gets +2/+2 and \
             gains lifelink.",
            &[CostDef::Mana(mana_cost!("{3}{W}{W}{W}"))],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Source,
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                    AppliedEffectDef::add_ability(&abilities::lifelink()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// M19 55 — Exclusion Mage
pub(in crate::card::sets) static EXCLUSION_MAGE: CardRecord = CardRecord::new(
    "Exclusion Mage",
    "ccad82f5-5c5c-42ad-b66e-942f0d9631ca",
    "Chris Seaman",
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Human", "Wizard"], 2, 2).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this creature enters, return target creature an opponent \
             controls to its owner's hand.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Hand,
                ZonePlacement::Top,
            ),
        ),
    ]),
);

// M19 63 — Mystic Archaeologist
pub(in crate::card::sets) static MYSTIC_ARCHAEOLOGIST: CardRecord = CardRecord::new(
    "Mystic Archaeologist",
    "1b19cad0-5754-4625-8303-c8310bc7cbd5",
    "Eric Deschamps",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Human", "Wizard"], 2, 1).with_abilities(&[
        AbilityDef::activated(
            "{3}{U}{U}: Draw two cards.",
            &[CostDef::Mana(mana_cost!("{3}{U}{U}"))],
            abilities::draw_cards(ValueDef::Constant(2)),
        ),
    ]),
);

// M19 118 — Skeleton Archer
pub(in crate::card::sets) static SKELETON_ARCHER: CardRecord = CardRecord::new(
    "Skeleton Archer",
    "8fee5cc4-a686-4ce6-aa6b-1b8a88e6dea3",
    "Randy Vargas",
    CardRules::new_creature(mana_cost!("{3}{B}"), &["Skeleton", "Archer"], 3, 3).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this creature enters, it deals 1 damage to any target.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(1),
            ),
        ),
    ]),
);

// M19 124 — Vampire Neonate
pub(in crate::card::sets) static VAMPIRE_NEONATE: CardRecord = CardRecord::new(
    "Vampire Neonate",
    "167822a5-2ab5-42f5-afa4-562fe2d7501b",
    "Daarken",
    CardRules::new_creature(mana_cost!("{B}"), &["Vampire"], 0, 3).with_abilities(&[
        AbilityDef::activated(
            "{2}, {T}: Each opponent loses 1 life and you gain 1 life.",
            &[CostDef::Mana(mana_cost!("{2}")), CostDef::TapSource],
            EffectDef::Sequence(&[
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Opponent,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::GainLife {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ]),
        ),
    ]),
);

// M19 125 — Vampire Sovereign
pub(in crate::card::sets) static VAMPIRE_SOVEREIGN: CardRecord = CardRecord::new(
    "Vampire Sovereign",
    "ee338221-ead9-4b89-8b0c-12745c4ca13d",
    "Volkan Baǵa",
    // A six-point swing attached to a flier, which is what makes five mana
    // a fair price in a format where the race is the game.
    CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Vampire", "Noble"], 3, 4).with_abilities(
        &[
            abilities::flying(),
            abilities::enters_trigger_with_targets(
                "When this creature enters, target opponent loses 3 life and you gain 3 life.",
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
                )],
                EffectDef::Sequence(&[
                    EffectDef::LoseLife {
                        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(3),
                    },
                    EffectDef::GainLife {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(3),
                    },
                ]),
            ),
        ],
    ),
);

// M19 128 — Alpine Moon
pub(in crate::card::sets) static ALPINE_MOON: CardRecord = CardRecord::new(
    "Alpine Moon",
    "2435c810-2baf-4e3b-80ce-542b94694901",
    "Alayna Danner",
CardRules::new_enchantment(mana_cost!("{R}")).with_abilities(&[
        AbilityDef::as_enters(
            "As this enchantment enters, choose a nonbasic land card name.",
            crate::card::ReplacementEffectDef::BindOutput {
                binding: Binding!("alpine_moon_name"),
                effect: &abilities::choose_card_name_as_enters(
                    crate::card::CardNameSetDef::NonbasicLandCardNames,
                ),
            },
        ),
        AbilityDef::static_ability(
            "Lands your opponents control with the chosen name lose all land types and abilities, and they gain \"{T}: Add one mana of any color.\"",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::NameEquals(
                            crate::card::CardNameDef::Binding(Binding!("alpine_moon_name")),
                        ),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Opponent,
                ),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::Characteristic(CharacteristicOperationDef::Subtypes(
                        SetOperationDef::Remove(LAND_SUBTYPES),
                    )),
                    AppliedEffectDef::remove_abilities(AbilityPredicateDef::Any),
                    AppliedEffectDef::add_ability(&AbilityDef::activated_mana(
                        "{T}: Add one mana of any color.",
                        &[CostDef::TapSource],
                        EffectDef::AddMana(AddManaEffectDef::any_color()),
                    )),
                ]),
            },
        ),
    ]),
);

// M19 134 — Dark-Dweller Oracle
pub(in crate::card::sets) static DARK_DWELLER_ORACLE: CardRecord = CardRecord::new(
    "Dark-Dweller Oracle",
    "69a57bfc-1de2-4b3a-84bc-19ec41087f0d",
    "Deruchenko Alexander",
// A sacrifice outlet that turns each body into a look at the top card,
    // and it can eat itself once the board is empty.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Goblin", "Shaman"], 2, 2).with_ability(
        AbilityDef::activated(
            "{1}, Sacrifice a creature: Exile the top card of your library. You may play that card this turn.",
            &[
                CostDef::Mana(mana_cost!("{1}")),
                CostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    controller: PlayerRelation::You,
                },
            ],
            EffectDef::ExileTopOfLibraryToPlay {
                player: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
                // "You may play that card", not play it for free: the Oracle
                // still charges for whatever it turns up.
                free: false,
                face_down: false,
                duration: ExilePlayDurationDef::ThisTurn,
                spend_any_color: false,
                play_condition: None,
                cast_only: false,
            },
        ),
    ),
);

// M19 143 — Goblin Motivator
pub(in crate::card::sets) static GOBLIN_MOTIVATOR: CardRecord = CardRecord::new(
    "Goblin Motivator",
    "94b3a4fb-9024-45ef-a54b-cf3a9fa5b9c2",
    "Johann Bodin",
    // Any creature, not only yours, though the haste is only worth giving
    // to something that just arrived on your own side.
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin", "Warrior"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "{T}: Target creature gains haste until end of turn.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::add_ability(&abilities::haste()),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ),
);

// M19 149 — Lathliss, Dragon Queen
pub(in crate::card::sets) static LATHLISS_DRAGON_QUEEN: CardRecord = CardRecord::new(
    "Lathliss, Dragon Queen",
    "54a4c37d-5eeb-42c9-9688-c2ed0d5044cd",
    "Alex Konstad",
    CardRules::new_creature(mana_cost!("{4}{R}{R}"), &["Dragon"], 6, 6)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::triggered(
                "Whenever another nontoken Dragon you control enters, create a \
                 5/5 red Dragon creature token with flying.",
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Dragon")),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Token),
                        ]),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ]),
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                    TokenCharacteristics::creature(&["Dragon"], &[ManaColor::Red], 5, 5)
                        .with_abilities(&[abilities::flying()]),
                ))),
            ),
            AbilityDef::activated(
                "{1}{R}: Dragons you control get +1/+0 until end of turn.",
                &[CostDef::Mana(mana_cost!("{1}{R}"))],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                        ObjectQueryDef::matching(
                            ObjectPredicateDef::Subtype(SubtypeDef::Literal("Dragon")),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::You,
                        ),
                    )),
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(0),
                    ),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        ]),
);

// M19 166 — Viashino Pyromancer
pub(in crate::card::sets) static VIASHINO_PYROMANCER: CardRecord = CardRecord::new(
    "Viashino Pyromancer",
    "a82fdb2f-b199-44ff-9615-90fc074cb8b0",
    "Jesper Ejsing",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Lizard", "Wizard"], 2, 1).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this creature enters, it deals 2 damage to target player \
             or planeswalker.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::PlayerOrPlaneswalker(PlayerRelation::Any),
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(2),
            ),
        ),
    ]),
);

// M19 168 — Volley Veteran
pub(in crate::card::sets) static VOLLEY_VETERAN: CardRecord = CardRecord::new(
    "Volley Veteran",
    "164960fc-6e80-4a53-90d7-5a18c0a28083",
    "Craig J Spearing",
    CardRules::new_creature(mana_cost!("{3}{R}"), &["Goblin", "Warrior"], 4, 2).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this creature enters, it deals damage to target creature \
             an opponent controls equal to the number of Goblins you \
             control.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                    ObjectPredicateDef::Subtype(SubtypeDef::Literal("Goblin")),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                )),
            ),
        ),
    ]),
);

// M19 185 — Gigantosaurus
pub(in crate::card::sets) static GIGANTOSAURUS: CardRecord = CardRecord::new(
    "Gigantosaurus",
    "c1db84d8-d426-4c0d-b44e-5be7b0f5f5bf",
    "Jonathan Kuo",
    CardRules::new_creature(mana_cost!("{G}{G}{G}{G}{G}"), &["Dinosaur"], 10, 10),
);

// M19 208 — Vivien Reid
pub(in crate::card::sets) static VIVIEN_REID: CardRecord = CardRecord::new(
    "Vivien Reid",
    "681fbd66-b622-4f20-a860-f101aff21109",
    "Anna Steinbauer",
    CardRules::new_planeswalker(mana_cost!("{3}{G}{G}"), &["Vivien"], 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::activated(
                "+1: Look at the top four cards of your library. You may \
                 reveal a creature or land card from among them and put it \
                 into your hand. Put the rest on the bottom of your library in \
                 a random order.",
                &[CostDef::Loyalty(1)],
                EffectDef::ChooseCardsFromCollection(ChooseCardsFromCollectionDef {
                    source: ObjectCollectionSourceDef::TopCards {
                        player: PlayerRefDef::EffectController,
                        count: ValueDef::Constant(4),
                    },
                    actor: PlayerRefDef::EffectController,
                    inspection: CollectionInspectionDef::Look,
                    object: ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::HasType(CardType::Land),
                    ]),
                    minimum: 0,
                    maximum: 1,
                    chosen: crate::Binding!("chosen"),
                    remainder: crate::Binding!("rest"),
                    then: &EffectDef::Sequence(&[
                        EffectDef::RevealObjects(RevealObjectsDef {
                            input: ObjectSetDef::Binding(crate::Binding!("chosen")),
                            then: &EffectDef::None,
                        }),
                        EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Binding(crate::Binding!(
                                "chosen"
                            ))),
                            ZoneKind::Hand,
                            ZonePlacement::Top,
                        ),
                        EffectDef::RandomizeObjectOrder(RandomizeObjectOrderDef {
                            input: ObjectSetDef::Binding(crate::Binding!("rest")),
                            randomized: crate::Binding!("random_bottom"),
                            then: &EffectDef::move_to_zone(
                                EffectRecipientDef::objects(ObjectSetDef::Binding(
                                    crate::Binding!("random_bottom"),
                                )),
                                ZoneKind::Library,
                                ZonePlacement::Bottom,
                            ),
                        }),
                    ]),
                }),
            ),
            AbilityDef::activated_with_targets(
                "−3: Destroy target artifact, enchantment, or creature with \
                 flying.",
                &[CostDef::Loyalty(-3)],
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
            ),
            AbilityDef::activated(
                "−8: You get an emblem with \"Creatures you control get +2/+2 \
                 and have vigilance, trample, and indestructible.\"",
                &[CostDef::Loyalty(-8)],
                EffectDef::CreateEmblem {
                    emblem: EmblemCharacteristics::new(
                        "Vivien Emblem",
                        &[AbilityDef::static_ability(
                            "Creatures you control get +2/+2 and have vigilance, trample, \
                             and indestructible.",
                            EffectDef::StaticApply {
                                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                                    ObjectQueryDef::matching(
                                        ObjectPredicateDef::HasType(CardType::Creature),
                                        &[ZoneKind::Battlefield],
                                        PlayerRelation::You,
                                    ),
                                )),
                                effect: AppliedEffectDef::Composite(&[
                                    AppliedEffectDef::modify_power_toughness(
                                        ValueDef::Constant(2),
                                        ValueDef::Constant(2),
                                    ),
                                    AppliedEffectDef::add_ability(&abilities::vigilance()),
                                    AppliedEffectDef::add_ability(&abilities::trample()),
                                    AppliedEffectDef::add_ability(&abilities::indestructible()),
                                ]),
                            },
                        )],
                    ),
                },
            ),
        ]),
);

// M19 217 — Heroic Reinforcements
pub(in crate::card::sets) static HEROIC_REINFORCEMENTS: CardRecord = CardRecord::new(
    "Heroic Reinforcements",
    "33c35bf8-ae43-41aa-aae9-4d7513f9058c",
    "Scott Murphy",
    CardRules::new_sorcery(mana_cost!("{2}{R}{W}")).with_abilities(&[AbilityDef::spell(
        "Create two 1/1 white Soldier creature tokens. Until end of \
         turn, creatures you control get +1/+1 and gain haste. (They \
         can attack and {T} this turn.)",
        EffectDef::Sequence(&[
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(TokenCharacteristics::creature(
                    &["Soldier"],
                    &[ManaColor::White],
                    1,
                    1,
                )))
                .with_count(ValueDef::Constant(2)),
            ),
            EffectDef::Apply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::Composite(&[
                    AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                    AppliedEffectDef::add_ability(&abilities::haste()),
                ]),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ]),
    )]),
);

// M19 231 — Diamond Mare
pub(in crate::card::sets) static DIAMOND_MARE: CardRecord = CardRecord::new(
    "Diamond Mare",
    "ca600b3f-2c70-489b-b218-6e3245b90114",
    "Alayna Danner",
    CardRules::new_artifact_creature(mana_cost!("{2}"), &["Horse"], 1, 3).with_abilities(&[
        AbilityDef::as_enters(
            "As this permanent enters, choose a color.",
            ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(
                BattlefieldEntryScalarChoiceDef::COLOR,
            )),
        ),
        AbilityDef::triggered(
            "Whenever you cast a spell of the chosen color, you gain 1 life.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasSourcesChosenScalar(
                    BattlefieldEntryChoiceDestinationDef::Color,
                ),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// M19 241 — Meteor Golem
pub(in crate::card::sets) static METEOR_GOLEM: CardRecord = CardRecord::new(
    "Meteor Golem",
    "1bdb0b15-d651-4730-8be9-d0e01145311b",
    "Lake Hurwitz",
    CardRules::new_artifact_creature(mana_cost!("{7}"), &["Golem"], 3, 3).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this creature enters, destroy target nonland permanent \
             an opponent controls.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
    ]),
);

// M19 297 — Kargan Dragonrider
pub(in crate::card::sets) static KARGAN_DRAGONRIDER: CardRecord = CardRecord::new(
    "Kargan Dragonrider",
    "34750304-c536-47d4-922d-a3654c37ffbc",
    "Greg Opalinski",
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Human", "Warrior"], 2, 2).with_abilities(&[
        AbilityDef::static_ability(
            "As long as you control a Dragon, this creature has flying.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ObjectCount {
                    query: ObjectQueryDef::matching(
                        ObjectPredicateDef::Subtype(SubtypeDef::Literal("Dragon")),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 1,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::flying()),
                },
            },
        ),
    ]),
);

// M19 302 — Aggressive Mammoth
pub(in crate::card::sets) static AGGRESSIVE_MAMMOTH: CardRecord = CardRecord::new(
    "Aggressive Mammoth",
    "323f3c76-5e79-43e6-ae78-f555810edbc3",
    "Filip Burburan",
    CardRules::new_creature(mana_cost!("{3}{G}{G}{G}"), &["Elephant"], 8, 8).with_abilities(&[
        abilities::trample(),
        AbilityDef::static_ability(
            "Other creatures you control have trample.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::add_ability(&abilities::trample()),
            },
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &HERALD_OF_FAITH,
    &LEONIN_VANGUARD,
    &MILITIA_BUGLER,
    &RESPLENDENT_ANGEL,
    &EXCLUSION_MAGE,
    &MYSTIC_ARCHAEOLOGIST,
    &SKELETON_ARCHER,
    &VAMPIRE_NEONATE,
    &VAMPIRE_SOVEREIGN,
    &ALPINE_MOON,
    &DARK_DWELLER_ORACLE,
    &GOBLIN_MOTIVATOR,
    &LATHLISS_DRAGON_QUEEN,
    &VIASHINO_PYROMANCER,
    &VOLLEY_VETERAN,
    &GIGANTOSAURUS,
    &VIVIEN_REID,
    &HEROIC_REINFORCEMENTS,
    &DIAMOND_MARE,
    &METEOR_GOLEM,
    &KARGAN_DRAGONRIDER,
    &AGGRESSIVE_MAMMOTH,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
