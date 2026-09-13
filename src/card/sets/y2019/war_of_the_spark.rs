//! War of the Spark cards cataloged for the Vintage Cube pool.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AlternativeCastKindDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::BasicLandType;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CardTypeSet;
use crate::card::CastTimingPermissionDef;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseDef;
use crate::card::ComparisonDef;
use crate::card::CopyExceptionsDef;
use crate::card::CostAdjustmentDef;
use crate::card::CostAmountDef;
use crate::card::CostDef;
use crate::card::CostModificationDef;
use crate::card::CountConditionDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::CreatureTypeSetDef;
use crate::card::DamageEventMatcherDef;
use crate::card::DamagePreventionDef;
use crate::card::EffectChoiceDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::InstalledTriggerDef;
use crate::card::ManaColor;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::PlayActionMatcherDef;
use crate::card::PlayRestrictionDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::PlayerRuleDef;
use crate::card::PlayerSetDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SpellCostConditionDef;
use crate::card::SpellCostModificationDef;
use crate::card::SumValueDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TopOfLibraryCostDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::TurnStepDef;
use crate::card::ValueComparisonDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "WAR",
    slug: "war-of-the-spark",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// WAR 1 — Karn, the Great Creator
// Audit: unsupported — The engine has no outside-the-game zone or wish-style retrieval operation. Searching exile alone would omit a legal branch of the -2 ability.
pub(in crate::card::sets) static KARN_THE_GREAT_CREATOR_1: CardRecord = CardRecord::new(
    "Karn, the Great Creator",
    "3ec0c0fb-1a4f-45f4-85b7-346a6d3ce2c5",
    "Wisnu Tan",
    crate::card::CardRules::unsupported(),
);

// WAR 2 — Ugin, the Ineffable
// Audit: unsupported — Needs a face-down exile linked to a token whose battlefield exit returns that exact card, while retaining the token and face-down-card identities.
pub(in crate::card::sets) static UGIN_THE_INEFFABLE_2: CardRecord = CardRecord::new(
    "Ugin, the Ineffable",
    "7b003521-3da3-41bf-9765-36630653f902",
    "Daarken",
    crate::card::CardRules::unsupported(),
);

// WAR 51 — Finale of Revelation
pub(in crate::card::sets) static FINALE_OF_REVELATION: CardRecord = CardRecord::new(
    "Finale of Revelation",
    "6630c34a-1a97-4e31-9d2c-1150b0aa903e",
    "Johann Bodin",
    CardRules::new_sorcery(mana_cost!("{X}{U}{U}")).with_abilities(&[AbilityDef::spell(
        "Draw X cards. If X is 10 or more, instead shuffle your \
         graveyard into your library, draw X cards, untap up to five \
         lands, and you have no maximum hand size for the rest of the \
         game.\nExile Finale of Revelation.",
        EffectDef::Sequence(&[
            EffectDef::IfElseCondition {
                condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::ChosenX,
                    comparison: ComparisonDef::GreaterOrEqual,
                    right: ValueDef::Constant(10),
                }),
                then: &EffectDef::Sequence(&[
                    EffectDef::Sequence(&[
                        EffectDef::move_to_zone(
                            EffectRecipientDef::objects(ObjectSetDef::Query(
                                ObjectQueryDef::matching(
                                    ObjectPredicateDef::Any,
                                    &[ZoneKind::Graveyard],
                                    PlayerRelation::You,
                                ),
                            )),
                            ZoneKind::Library,
                            ZonePlacement::Top,
                        ),
                        EffectDef::ShuffleLibrary {
                            player: EffectRecipientDef::Controller,
                        },
                    ]),
                    abilities::draw_cards(ValueDef::ChosenX),
                    EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Objects(crate::Binding!("chosen")),
                        unchosen: None,
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Query(ObjectQueryDef::matching(
                            ObjectPredicateDef::HasType(CardType::Land),
                            &[ZoneKind::Battlefield],
                            PlayerRelation::Any,
                        )),
                        exclude: None,
                        minimum: 0,
                        maximum: 5,
                        visibility: ChoiceVisibilityDef::Public,
                        then: &EffectDef::Untap {
                            object: EffectRecipientDef::objects(ObjectSetDef::Binding(
                                crate::Binding!("chosen"),
                            )),
                        },
                    }),
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Controller,
                        effect: AppliedEffectDef::Rule(AppliedRuleDef::PlayerRule(
                            PlayerRuleDef::NoMaximumHandSize,
                        )),
                        duration: ResolvedEffectDurationDef::Permanent,
                    },
                ]),
                otherwise: &abilities::draw_cards(ValueDef::ChosenX),
            },
            EffectDef::move_to_zone(
                EffectRecipientDef::Source,
                ZoneKind::Exile,
                ZonePlacement::Top,
            ),
        ]),
    )]),
);

// WAR 54 — Jace, Wielder of Mysteries
pub(in crate::card::sets) static JACE_WIELDER_OF_MYSTERIES: CardRecord = CardRecord::new(
    "Jace, Wielder of Mysteries",
    "6adb7d73-4482-4930-8497-cffd169b57e2",
    "Anna Steinbauer",
    CardRules::new_planeswalker(mana_cost!("{1}{U}{U}{U}"), &["Jace"], 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::empty_library_draw_wins(),
            AbilityDef::activated_with_targets(
                "+1: Target player mills two cards. Draw a card.",
                &[CostDef::Loyalty(1)],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Player(PlayerRelation::Any),
                )],
                EffectDef::Sequence(&[
                    EffectDef::Mill {
                        player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        amount: ValueDef::Constant(2),
                    },
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                ]),
            ),
            AbilityDef::activated(
                "−8: Draw seven cards. Then if your library has no cards in it, you win the game.",
                &[CostDef::Loyalty(-8)],
                EffectDef::Sequence(&[
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(7),
                    },
                    EffectDef::IfCondition {
                        // Your own library, empty. Written as a count rather than a dedicated
                        // question so the same shape answers "no cards in it" and any other bound.
                        condition: &TriggerConditionDef::ObjectCount {
                            query: ObjectQueryDef::matching(
                                ObjectPredicateDef::Any,
                                &[ZoneKind::Library],
                                PlayerRelation::You,
                            ),
                            comparison: ComparisonDef::LessOrEqual,
                            amount: 0,
                        },
                        then: &EffectDef::WinTheGame {
                            player: EffectRecipientDef::Controller,
                        },
                    },
                ]),
            ),
        ]),
);

// WAR 61 — Narset, Parter of Veils
pub(in crate::card::sets) static NARSET_PARTER_OF_VEILS: CardRecord = CardRecord::new(
    "Narset, Parter of Veils",
    "8c39f9b4-02b9-4d44-b8d6-4fd02ebbb0c5",
    "Magali Villeneuve",
// Three mana that finds the spell the deck is built around and turns
    // every draw spell the other player has into one card.
    CardRules::new_planeswalker(mana_cost!("{1}{U}{U}"), &["Narset"], 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Each opponent can't draw more than one card each turn.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Opponent,
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotDrawMoreThanEachTurn(1)),
                },
            ),
            AbilityDef::activated(
                "\u{2212}2: Look at the top four cards of your library. You may reveal a noncreature, \
                 nonland card from among them and put it into your hand. Put the rest on the bottom of \
                 your library in a random order.",
                &[CostDef::Loyalty(-2)],
                abilities::look_at_top_cards_reveal_choice_to_hand_rest_random_bottom(
                    ValueDef::Constant(4),
                    // "You may reveal": taking nothing is a legal answer, and what is left
                    // goes to the bottom in a random order rather than in the order it was
                    // seen -- so the four cards are not a free look at the next four draws.
                    ObjectPredicateDef::All(&[
                            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                    ]),
                    0,
                    1,
                ),
            ),
        ]),
);

// WAR 68 — Spark Double
// Audit: unsupported — Needs an entering-copy exception conditional on the copied permanent being a creature or planeswalker, adding the corresponding counter and removing legendary.
pub(in crate::card::sets) static SPARK_DOUBLE_68: CardRecord = CardRecord::new(
    "Spark Double",
    "bb8a103c-b776-4501-9441-a45b90391045",
    "Eric Deschamps",
    crate::card::CardRules::unsupported(),
);

// WAR 79 — Bolas's Citadel
pub(in crate::card::sets) static BOLASS_CITADEL: CardRecord = CardRecord::new(
    "Bolas's Citadel",
    "d2124603-d20e-40eb-97f0-a66323397ac2",
    "Jonas De Ro",
// Six mana to turn a library into a hand and a life total into mana.
    // The ten-permanent ability is the finish, not the plan.
    CardRules::new_artifact(mana_cost!("{3}{B}{B}{B}"))
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "You may look at the top card of your library any time.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::You)),
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::MayLookAtTopOfLibrary),
                },
            ),
            AbilityDef::static_ability(
                "You may play lands and cast spells from the top of your library. If you cast a spell \
                 this way, pay life equal to its mana value rather than pay its mana cost.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::players(PlayerSetDef::Related(PlayerRelation::You)),
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::MayPlayFromTopOfLibrary {
                        // Anything at all, which is what "lands and spells" comes to once the top
                        // of the library is the only place being named.
                        restriction: PlayRestrictionDef::new(PlayActionMatcherDef::Any, ObjectPredicateDef::Any),
                        cost: TopOfLibraryCostDef::LifeEqualToManaValue,
                    }),
                },
            ),
            AbilityDef::activated(
                "{T}, Sacrifice ten nonland permanents: Each opponent loses 10 life.",
                &[
                    CostDef::TapSource,
                    CostDef::SacrificePermanents {
                        object: ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                        controller: PlayerRelation::You,
                        count: 10,
                    },
                ],
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Opponent,
                    amount: ValueDef::Constant(10),
                },
            ),
        ]),
);

// WAR 97 — Liliana, Dreadhorde General
// Audit: unsupported — Needs a choice retaining one controlled permanent per permanent type, allowing one multitype permanent to fill several types, followed by sacrifice of the complement.
pub(in crate::card::sets) static LILIANA_DREADHORDE_GENERAL: CardRecord = CardRecord::new(
    "Liliana, Dreadhorde General",
    "d75ebba8-34ca-47a0-bf13-8318ad73b343",
    "Chris Rallis",
    CardRules::unsupported(),
);

// WAR 99 — Massacre Girl
pub(in crate::card::sets) static MASSACRE_GIRL_99: CardRecord = CardRecord::new(
"Massacre Girl",
"be8ec9e1-2c8e-496d-9111-4d453b75b578",
"Chris Rallis",
CardRules::new_creature(mana_cost!("{3}{B}{B}"), &["Human",
"Assassin"], 4, 4).with_supertype(CardSupertype::Legendary).with_abilities(&[abilities::menace(),
abilities::enters_trigger("When Massacre Girl enters, each other creature gets -1/-1 until end of turn. \
 Whenever a creature dies this turn, each creature other than Massacre Girl \
 gets -1/-1 until end of turn.", EffectDef::Sequence(&[EffectDef::Apply { recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature),
ObjectPredicateDef::Not(&ObjectPredicateDef::Source)]), &[ZoneKind::Battlefield], PlayerRelation::Any), effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(-1), ValueDef::Constant(-1)), duration: ResolvedEffectDurationDef::UntilEndOfTurn },
EffectDef::InstallTrigger(InstalledTriggerDef::this_turn(&AbilityDef::triggered("Whenever a creature dies this turn, each creature other than Massacre Girl \
 gets -1/-1 until end of turn.", TriggerEventDef::zone_changed(ObjectPredicateDef::HasType(CardType::Creature), Some(ZoneKind::Battlefield), Some(ZoneKind::Graveyard)), EffectDef::Apply { recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature),
ObjectPredicateDef::Not(&ObjectPredicateDef::Source)]), &[ZoneKind::Battlefield], PlayerRelation::Any), effect: AppliedEffectDef::modify_power_toughness(ValueDef::Constant(-1), ValueDef::Constant(-1)), duration: ResolvedEffectDurationDef::UntilEndOfTurn })))]))]));

// WAR 115 — Bolt Bend
pub(in crate::card::sets) static BOLT_BEND: CardRecord = CardRecord::new(
    "Bolt Bend",
    "39b35408-3728-4e1b-9f58-b0775df914d6",
    "Svetlin Velinov",
    CardRules::new_instant(mana_cost!("{3}{R}")).with_abilities(&[
        AbilityDef::static_ability(
            "This spell costs {3} less to cast if you control a creature with power 4 or greater.",
            EffectDef::ReduceGenericCostBy(ValueDef::IfMatchingObjectCount(&CountConditionDef {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Creature),
                        ObjectPredicateDef::PowerAtLeast(4),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 1,
                then: ValueDef::Constant(3),
                otherwise: ValueDef::Constant(0),
            })),
        )
        .with_source_zones(&[ZoneKind::Hand]),
        AbilityDef::spell_with_targets(
            "Change the target of target spell or ability with a single target.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::DeclaredTargetCount {
                        minimum: 1,
                        maximum: 1,
                    },
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::ChangeStackTargets(&crate::card::ChangeStackTargetsDef {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                chooser: PlayerRefDef::EffectController,
                change: crate::card::StackTargetChangeDef::ChooseNew {
                    optional: false,
                    restriction: None,
                },
            }),
        ),
    ]),
);

// WAR 125 — Dreadhorde Arcanist
pub(in crate::card::sets) static DREADHORDE_ARCANIST: CardRecord = CardRecord::new(
    "Dreadhorde Arcanist",
    "fd97b3cf-924e-4f77-bb82-0bf19592389f",
    "G-host Lee",
// A 1/3 that only buys back one-mana spells until something makes it
    // bigger, which in the cube is most of what the deck is doing anyway.
    CardRules::new_creature(mana_cost!("{1}{R}"), &["Zombie", "Wizard"], 1, 3)
        .with_abilities(&[
            abilities::trample(),
            AbilityDef::triggered_with_targets(
                "Whenever this creature attacks, you may cast target instant or sorcery card with mana \
                 value less than or equal to this creature's power from your graveyard without paying \
                 its mana cost. If that spell would be put into your graveyard, exile it instead.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                // "Mana value less than or equal to this creature's power" is read live off
                // the Arcanist, so a counter or a pump changes what it can reach.
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::All(&[
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Instant),
                                ObjectPredicateDef::HasType(CardType::Sorcery),
                            ]),
                            ObjectPredicateDef::ManaValueAtMostValue(ValueDef::SourcePower),
                        ]),
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: Some(PlayerRelation::You),
                    },
                )],
                EffectDef::MayCastTargetWithoutPaying {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    // What the card is lent while the offer stands: no mana-payment cost and,
                    // independently, a replacement for a later graveyard move.
                    ability: &AbilityDef::alternative_cast(
                        crate::NO_COSTS,
                        AlternativeCastKindDef::Granted,
                        Some("Cast without paying its mana cost, then exile it."),
                        EffectDef::None,
                    )
                    .with_exile_if_put_into_graveyard(),
                },
            ),
        ]),
);

// WAR 127 — Finale of Promise
// Audit: unsupported — Needs two independently optional graveyard spell targets with mana value bounded by X, free casting, exile-on-resolution replacement, and conditional spell copying.
pub(in crate::card::sets) static FINALE_OF_PROMISE_127: CardRecord = CardRecord::new(
    "Finale of Promise",
    "811b2dda-e1b7-4a46-83cc-5cdc17554836",
    "Jaime Jones",
    crate::card::CardRules::unsupported(),
);

// WAR 130 — Grim Initiate
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static GRIM_INITIATE: CardRecord = CardRecord::new(
    "Grim Initiate",
    "29b6ec9d-3861-48bf-a198-dc7efba5d89c",
    "Jason Felix",
    crate::card::CardRules::unsupported(),
);

// WAR 137 — Krenko, Tin Street Kingpin
pub(in crate::card::sets) static KRENKO_TIN_STREET_KINGPIN_137: CardRecord = CardRecord::new(
    "Krenko, Tin Street Kingpin",
    "37ed04d3-cfa1-4778-aea6-b4c2c29e6e0a",
    "Mark Behm",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin"], 1, 2)
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::triggered(
            "Whenever Krenko attacks, put a +1/+1 counter on it, then create a number of 1/1 red Goblin creature tokens equal to Krenko's power.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            EffectDef::Sequence(&[
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
                EffectDef::CreateToken(crate::card::CreateTokenDef::new(crate::card::TokenDef::Literal(crate::card::TokenCharacteristics::creature(&["Goblin"], &[ManaColor::Red], 1, 1))).with_count(ValueDef::ObjectPower(crate::card::ObjectRefDef::Source))),
            ]),
        )),
);

// WAR 160 — Finale of Devastation
pub(in crate::card::sets) static FINALE_OF_DEVASTATION_160: CardRecord = CardRecord::new(
    "Finale of Devastation",
    "985453e7-997e-4d77-a338-cc0290791ebe",
    "Bayard Wu",
    CardRules::new_sorcery(mana_cost!("{X}{G}{G}")).with_abilities(&[
AbilityDef::spell("Search your library and/or graveyard for a creature card with mana value X or less and put it onto the battlefield. If you search your library this way, shuffle. If X is 10 or more, creatures you control get +X/+X and gain haste until end of turn.", EffectDef::Sequence(&[EffectDef::ChooseEffect { player: EffectRecipientDef::Controller, choices: &[EffectChoiceDef { label: "Search your library.", effect: EffectDef::SearchZone { player: EffectRecipientDef::Controller, source: ZoneKind::Library, object: ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::ManaValueAtMostValue(ValueDef::ChosenX)]), minimum: 0, maximum: ValueDef::Constant(1), reveal: false, destination: ZoneKind::Battlefield, placement: ZonePlacement::Top, shuffle: true, enters_tapped: false, attachment: None, binding: None, then: None } }, EffectChoiceDef { label: "Search your graveyard.", effect: EffectDef::SearchZone { player: EffectRecipientDef::Controller, source: ZoneKind::Graveyard, object: ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::ManaValueAtMostValue(ValueDef::ChosenX)]), minimum: 1, maximum: ValueDef::Constant(1), reveal: false, destination: ZoneKind::Battlefield, placement: ZonePlacement::Top, shuffle: false, enters_tapped: false, attachment: None, binding: None, then: None } }, EffectChoiceDef { label: "Search both zones.", effect: EffectDef::Sequence(&[EffectDef::Choose(ChooseDef { chooser: PlayerRefDef::EffectController, candidates: ObjectSetDef::Union(&[ObjectSetDef::Query(ObjectQueryDef::matching(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::ManaValueAtMostValue(ValueDef::ChosenX)]), &[ZoneKind::Library], PlayerRelation::You)), ObjectSetDef::Query(ObjectQueryDef::matching(ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::ManaValueAtMostValue(ValueDef::ChosenX)]), &[ZoneKind::Graveyard], PlayerRelation::You))]), exclude: None, minimum: 0, maximum: 1, binding: ObjectChoiceBindingDef::Objects(Binding!("finale_found")), unchosen: None, visibility: ChoiceVisibilityDef::Private, then: &EffectDef::move_to_zone(EffectRecipientDef::objects(ObjectSetDef::Binding(Binding!("finale_found"))), ZoneKind::Battlefield, ZonePlacement::Top) }), EffectDef::ShuffleLibrary { player: EffectRecipientDef::Controller }]) }] }, EffectDef::IfCondition { condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef { left: ValueDef::ChosenX, comparison: ComparisonDef::GreaterOrEqual, right: ValueDef::Constant(10) }), then: &EffectDef::Apply { recipient: EffectRecipientDef::matching_objects(ObjectPredicateDef::HasType(CardType::Creature), &[ZoneKind::Battlefield], PlayerRelation::You), effect: AppliedEffectDef::Composite(&[AppliedEffectDef::modify_power_toughness(ValueDef::ChosenX, ValueDef::ChosenX), AppliedEffectDef::add_ability(&abilities::haste())]), duration: ResolvedEffectDurationDef::UntilEndOfTurn } }]))
]),
);

// WAR 169 — Nissa, Who Shakes the World
pub(in crate::card::sets) static NISSA_WHO_SHAKES_THE_WORLD: CardRecord =
    CardRecord::new(
    "Nissa, Who Shakes the World",
    "f857bbe4-5619-4733-a0c7-69700f2ef4f3",
    "Chris Rallis",
// Doubling every Forest is the card: five mana becomes eight the turn
        // after, and the +1 turns the spare land into a 3/3 that attacks at once.
        CardRules::new_planeswalker(mana_cost!("{3}{G}{G}"), &["Nissa"], 5)
            .with_supertype(CardSupertype::Legendary)
            .with_abilities(&[
                AbilityDef::triggered_mana(
                    "Whenever you tap a Forest for mana, add an additional {G}.",
                    // "You tap a Forest for mana" is the tap transition carrying its purpose,
                    // so an ordinary tap does not fire it and a mana tap does.
                    TriggerEventDef::tapped_for_mana(ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ])),
                    EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
                ),
                AbilityDef::activated_with_targets(
                    "+1: Put three +1/+1 counters on up to one target noncreature land you control. Untap it. It becomes a 0/0 Elemental creature with vigilance and haste that's still a land.",
                    &[CostDef::Loyalty(1)],
                    &[AbilityTargetDef::up_to(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::All(&[
                                ObjectPredicateDef::HasType(CardType::Land),
                                ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
                            ]),
                            zones: &[ZoneKind::Battlefield],
                            controller: Some(PlayerRelation::You),
                            owner: None,
                        },
                        1,
                    )],
                    // The counters go on first, while the land is still a noncreature: the
                    // animation then sets a base of 0/0 and the three counters make it a 3/3.
                    EffectDef::Sequence(&[
                        EffectDef::AddCounters {
                            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            kind: CounterKind::PlusOnePlusOne,
                            amount: ValueDef::Constant(3),
                        },
                        EffectDef::Untap {
                            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        },
                        EffectDef::Apply {
                            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                            // "Still a land" is why the types are added rather than set: the animated
                            // permanent keeps tapping for mana, and Nissa's own static doubles it.
                            effect: AppliedEffectDef::Composite(&[
                                AppliedEffectDef::add_card_types(CardTypeSet::single(CardType::Creature)),
                                AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(0), ValueDef::Constant(0)),
                                AppliedEffectDef::add_creature_types(CreatureTypeSetDef::named(&["Elemental"])),
                                AppliedEffectDef::add_ability(&abilities::vigilance()),
                                AppliedEffectDef::add_ability(&abilities::haste()),
                            ]),
                            duration: ResolvedEffectDurationDef::Permanent,
                        },
                    ]),
                ),
                AbilityDef::activated(
                    "−8: You get an emblem with \"Lands you control have indestructible.\" Search your library for any number of Forest cards, put them onto the battlefield tapped, then shuffle.",
                    &[CostDef::Loyalty(-8)],
                    EffectDef::Sequence(&[
                        EffectDef::create_emblem(
                            "Nissa, Who Shakes the World emblem",
                            &[AbilityDef::static_ability(
                                "Lands you control have indestructible.",
                                EffectDef::StaticApply {
                                    recipient: EffectRecipientDef::matching_objects(
                                        ObjectPredicateDef::HasType(CardType::Land),
                                        &[ZoneKind::Battlefield],
                                        PlayerRelation::You,
                                    ),
                                    effect: AppliedEffectDef::add_ability(&abilities::indestructible()),
                                },
                            )],
                        ),
                        EffectDef::SearchZone {
                            player: EffectRecipientDef::Controller,
                            source: ZoneKind::Library,
                            object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
                            minimum: 0,
                            // "Any number" is every one there is, so the bound is how many the library
                            // actually holds rather than a printed number.
                            maximum: ValueDef::CountMatchingObjects(&ObjectQueryDef::matching(
                                ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
                                &[ZoneKind::Library],
                                PlayerRelation::You,
                            )),
                            reveal: false,
                            destination: ZoneKind::Battlefield,
                            placement: ZonePlacement::Top,
                            shuffle: true,
                            enters_tapped: true,
                            attachment: None,
                            binding: None,
                            then: None,
                        },
                    ]),
                ),
            ]),
)
    ;

// WAR 180 — Vivien, Champion of the Wilds
// Audit: unsupported — Arbitrary-card exile permissions cannot combine face-down exile, a creature-only cast restriction, and a while-exiled duration. The timed top-library exile operation cannot grant that permission to just the chosen card.
pub(in crate::card::sets) static VIVIEN_CHAMPION_OF_THE_WILDS_180: CardRecord = CardRecord::new(
    "Vivien, Champion of the Wilds",
    "ff3986d7-9b3d-4082-8d72-ce59c9fcd5d5",
    "Magali Villeneuve",
    crate::card::CardRules::unsupported(),
);

// WAR 193 — Dovin's Veto
pub(in crate::card::sets) static DOVIN_S_VETO_193: CardRecord = CardRecord::new(
    "Dovin's Veto",
    "5d6b5054-2224-4f68-9d82-3ed17c5dacc4",
    "Izzy",
    CardRules::new_instant(mana_cost!("{W}{U}")).with_abilities(&[
        abilities::cannot_be_countered(),
        AbilityDef::spell_with_targets(
            "Counter target noncreature spell.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::NoncreatureSpell,
                    zones: &[ZoneKind::Stack],
                    controller: None,
                    owner: None,
                },
            )],
            EffectDef::counter_target(TargetIndex::PRIMARY),
        ),
    ]),
);

// WAR 204 — Mayhem Devil
pub(in crate::card::sets) static MAYHEM_DEVIL_204: CardRecord = CardRecord::new(
    "Mayhem Devil",
    "17416926-168b-49b3-9231-acbb8f8a1d13",
    "Dmitry Burmak",
    CardRules::new_creature(mana_cost!("{1}{B}{R}"), &["Devil"], 3, 3).with_ability(
        AbilityDef::triggered_with_targets(
            "Whenever a player sacrifices a permanent, Mayhem Devil deals 1 damage to any target.",
            TriggerEventDef::Sacrificed {
                object: ObjectPredicateDef::Any,
                player: PlayerRelation::Any,
            },
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::damage(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ValueDef::Constant(1),
            ),
        ),
    ),
);

// WAR 206 — Neoform
pub(in crate::card::sets) static NEOFORM_206: CardRecord = CardRecord::new(
    "Neoform",
    "92d8f67e-4f2f-4a1f-b190-7c3f39e477e4",
    "Bram Sels",
    CardRules::new_sorcery(mana_cost!("{G}{U}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, sacrifice a creature. Search your library for a creature card with mana value equal to 1 plus the sacrificed creature's mana value, put that card onto the battlefield with an additional +1/+1 counter on it, then shuffle.",
            &[],
            CostDef::SacrificePermanent {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                controller: PlayerRelation::You,
            },
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::ManaValueEqualTo(ValueDef::Sum(&SumValueDef::new(
                        ValueDef::SacrificedManaValue,
                        ValueDef::Constant(1),
                    ))),
                ]),
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: false,
                destination: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: false,
                attachment: None,
                binding: Some(crate::ids::ParentBinding),
                then: Some(&EffectDef::AddCounters {
                    object: EffectRecipientDef::objects(crate::card::ObjectSetDef::Binding(
                        crate::ids::ParentBinding,
                    )),
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                }),
            },
        ),
    ),
);

// WAR 220 — Tamiyo, Collector of Tales
pub(in crate::card::sets) static TAMIYO_COLLECTOR_OF_TALES: CardRecord =
    CardRecord::new(
    "Tamiyo, Collector of Tales",
    "76776b24-a2e1-4590-88e7-8a421baf2fc4",
    "Chase Stone",
// The static is what the card is played for: it turns off every
        // discard-based and sacrifice-based answer an opponent has, and the
        // loyalty abilities are what it does while doing that.
        CardRules::new_planeswalker(mana_cost!("{2}{G}{U}"), &["Tamiyo"], 5)
            .with_supertype(CardSupertype::Legendary)
            .with_abilities(&[
                AbilityDef::static_ability(
                    "Spells and abilities your opponents control can't cause you to discard cards or sacrifice permanents.",
                    // Two prohibitions in one printed sentence, which is why they are a
                    // sequence rather than one effect: other cards state only one of them.
                    EffectDef::Sequence(&[
                        EffectDef::CannotBeForcedToDiscard,
                        EffectDef::CannotBeForcedToSacrifice,
                    ]),
                ),
                AbilityDef::activated(
                    "+1: Choose a nonland card name, then reveal the top four cards of your library. Put all cards with the chosen name from among them into your hand and the rest into your graveyard.",
                    &[CostDef::Loyalty(1)],
                    EffectDef::Sequence(&[
                        EffectDef::BindOutput {
                            binding: Binding!("tamiyo_name"),
                            effect: &EffectDef::ChooseCardName {
                                chooser: PlayerRefDef::EffectController,
                                names: crate::card::CardNameSetDef::NonlandCardNames,
                            },
                        },
                        // The name is chosen before the four cards are seen, so the reveal cannot
                        // be used to pick a name that is already there.
                        abilities::reveal_top_cards_put_matching_in_hand_rest_graveyard(
                            ValueDef::Constant(4),
                            ObjectPredicateDef::NameEquals(crate::card::CardNameDef::Binding(
                                Binding!("tamiyo_name"),
                            )),
                        ),
                    ]),
                ),
                AbilityDef::activated_with_targets(
                    "\u{2212}3: Return target card from your graveyard to your hand.",
                    &[CostDef::Loyalty(-3)],
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::Any,
                            zones: &[ZoneKind::Graveyard],
                            controller: None,
                            owner: Some(PlayerRelation::You),
                        },
                    )],
                    EffectDef::move_to_zone(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ZoneKind::Hand,
                        ZonePlacement::Top,
                    ),
                ),
            ]),
)
    ;

// WAR 221 — Teferi, Time Raveler
pub(in crate::card::sets) static TEFERI_TIME_RAVELER: CardRecord = CardRecord::new(
    "Teferi, Time Raveler",
    "5cb76266-ae50-4bbc-8f96-d98f309b02d3",
    "Chris Rallis",
    // Three mana that takes the other player's instant speed away and hands
    // it to you, with a bounce-and-draw underneath it.
    CardRules::new_planeswalker(mana_cost!("{1}{W}{U}"), &["Teferi"], 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::static_ability(
                "Each opponent can cast spells only any time they could cast a sorcery.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::players(PlayerSetDef::Related(
                        PlayerRelation::Opponent,
                    )),
                    // The restriction bars nothing during an opponent's own
                    // main phase with an empty stack, and every other cast.
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotPlay(
                        PlayRestrictionDef::new(
                            PlayActionMatcherDef::CastSpell,
                            ObjectPredicateDef::Any,
                        )
                        .only_at_sorcery_speed(),
                    )),
                },
            ),
            AbilityDef::activated(
                "+1: Until your next turn, you may cast sorcery spells as though they had flash.",
                &[CostDef::Loyalty(1)],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Controller,
                    // This is a permission rather than a granted keyword, so
                    // the affected cards still do not have flash.
                    effect: AppliedEffectDef::Rule(AppliedRuleDef::MayCastAsThoughItHadFlash(
                        CastTimingPermissionDef::new(ObjectPredicateDef::HasType(
                            CardType::Sorcery,
                        )),
                    )),
                    duration: ResolvedEffectDurationDef::UntilYourNextTurn,
                },
            ),
            AbilityDef::activated_with_targets(
                "\u{2212}3: Return up to one target artifact, creature, or enchantment to its \
                 owner's hand. Draw a card.",
                &[CostDef::Loyalty(-3)],
                &[AbilityTargetDef::up_to(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::HasType(CardType::Enchantment),
                        ]),
                        zones: &[ZoneKind::Battlefield],
                        controller: None,
                        owner: None,
                    },
                    1,
                )],
                // The draw is not conditional on the optional bounce target.
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
            ),
        ]),
);

// WAR 222 — Tenth District Legionnaire
pub(in crate::card::sets) static TENTH_DISTRICT_LEGIONNAIRE: CardRecord = CardRecord::new(
    "Tenth District Legionnaire",
    "44f3090b-917b-4122-b522-27c30dca8e69",
    "Victor Adame Minguez",
    // Haste plus a counter for every trick aimed at it, so the two-drop
    // that ate a pump spell is bigger next turn as well as this one.
    CardRules::new_creature(mana_cost!("{R}{W}"), &["Human", "Soldier"], 2, 2).with_abilities(&[
        abilities::haste(),
        AbilityDef::triggered(
            "Whenever you cast a spell that targets this creature, put a +1/+1 counter on this \
             creature, then scry 1.",
            // Read off what the spell already targets rather than what it
            // could take, and only spells you cast: an opponent's removal
            // aimed at this does not grow it.
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ObjectPredicateDef::TargetsObjectMatching(&ObjectPredicateDef::Source),
            ])),
            EffectDef::Sequence(&[
                EffectDef::AddCounters {
                    object: EffectRecipientDef::Source,
                    kind: CounterKind::PlusOnePlusOne,
                    amount: ValueDef::Constant(1),
                },
                abilities::scry(ValueDef::Constant(1)),
            ]),
        ),
    ]),
);

// WAR 228 — Ashiok, Dream Render
// Audit: unsupported — The search procedure has no continuous restriction that forbids library searches caused by spells and abilities controlled by opponents. The loyalty ability itself is expressible.
pub(in crate::card::sets) static ASHIOK_DREAM_RENDER_228: CardRecord = CardRecord::new(
    "Ashiok, Dream Render",
    "f2df3258-c053-48a8-974f-d80899b2cd93",
    "Cynthia Sheppard",
    crate::card::CardRules::unsupported(),
);

// WAR 229 — Dovin, Hand of Control
pub(in crate::card::sets) static DOVIN_HAND_OF_CONTROL_229: CardRecord = CardRecord::new(
    "Dovin, Hand of Control",
    "bd6ff745-919b-4688-9e9e-ab7835b3b891",
    "Kieran Yanner",
    CardRules::new_planeswalker(mana_cost!("{2}{W/U}"), &["Dovin"], 5).with_supertype(CardSupertype::Legendary).with_abilities(&[AbilityDef::static_ability("Artifact, instant, and sorcery spells your opponents cast cost {1} more to cast.", EffectDef::ModifyCost(CostModificationDef::Spell(SpellCostModificationDef { spell: ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::HasType(CardType::Artifact), ObjectPredicateDef::HasType(CardType::Instant), ObjectPredicateDef::HasType(CardType::Sorcery)]), caster: PlayerRelation::Opponent, condition: SpellCostConditionDef::Always, adjustment: CostAdjustmentDef::Add(CostAmountDef::Generic(ValueDef::Constant(1))) }))), AbilityDef::activated_with_targets("−1: Until your next turn, prevent all damage that would be dealt to and dealt by target permanent an opponent controls.", &[CostDef::Loyalty(-1)], &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object { object: ObjectPredicateDef::Any, zones: &[ZoneKind::Battlefield], controller: Some(PlayerRelation::Opponent), owner: None })], EffectDef::Sequence(&[EffectDef::PreventDamage { prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::to(EffectRecipientDef::Target(TargetIndex::PRIMARY))), duration: ResolvedEffectDurationDef::UntilYourNextTurn }, EffectDef::PreventDamage { prevention: DamagePreventionDef::unlimited(DamageEventMatcherDef::from(ObjectRefDef::Target(TargetIndex::PRIMARY))), duration: ResolvedEffectDurationDef::UntilYourNextTurn }]))]),
);

// WAR 234 — Saheeli, Sublime Artificer
pub(in crate::card::sets) static SAHEELI_SUBLIME_ARTIFICER: CardRecord =
    CardRecord::new(
    "Saheeli, Sublime Artificer",
    "5a10b543-d5d4-42a8-9ee8-dada59a2ad7e",
    "Wesley Burt",
// A planeswalker that never has to be activated: three mana, five
        // loyalty, and a body for every spell the deck was casting anyway.
        CardRules::new_planeswalker(mana_cost!("{1}{U/R}{U/R}"), &["Saheeli"], 5)
            .with_supertype(CardSupertype::Legendary)
            .with_abilities(&[
                AbilityDef::triggered(
                    "Whenever you cast a noncreature spell, create a 1/1 colorless Servo artifact creature \
                     token.",
                    TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                        ObjectPredicateDef::NoncreatureSpell,
                        ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                    ])),
                    EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                        TokenCharacteristics::artifact_creature(&["Servo"], &[], 1, 1).with_art(CardArt::new(
                            "761507d5-d36a-4123-a074-95d7f6ffb4c5",
                            "Victor Adame Minguez",
                        )),
                    ))),
                ),
                AbilityDef::activated_with_targets(
                    "−2: Target artifact you control becomes a copy of another target artifact or creature \
                     you control until end of turn, except it's an artifact in addition to its other types.",
                    &[CostDef::Loyalty(-2)],
                    // "Another target artifact or creature you control": the second slot is a
                    // separate target, so the two cannot be the same permanent.
                    &[
                        AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ])),
                        AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::All(&[
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Artifact),
                                ObjectPredicateDef::HasType(CardType::Creature),
                            ]),
                            ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                        ]))
                        .another(),
                    ],
                    EffectDef::BecomeCopyOf {
                        object: EffectRecipientDef::Target(TargetIndex(1)),
                        copier: Some(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
                        exceptions: CopyExceptionsDef::NONE
                            .with_added_types(CardTypeSet::single(CardType::Artifact)),
                        duration: Some(ResolvedEffectDurationDef::UntilEndOfTurn),
                    },
                ),
            ]),
);

// WAR 238 — God-Pharaoh's Statue
pub(in crate::card::sets) static GOD_PHARAOH_S_STATUE_238: CardRecord = CardRecord::new(
    "God-Pharaoh's Statue",
    "7dce06ba-c1e1-45ec-82a7-fc10b0fa8870",
    "Igor Kieryluk",
    CardRules::new_artifact(mana_cost!("{6}"))
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::spell_cost_increase(
                "Spells your opponents cast cost {2} more to cast.",
                ObjectPredicateDef::Any,
                PlayerRelation::Opponent,
                mana_cost!("{2}"),
            ),
            AbilityDef::triggered(
                "At the beginning of your end step, each opponent loses 1 life.",
                TriggerEventDef::StepBegins {
                    step: TurnStepDef::End,
                    player: PlayerRelation::You,
                },
                EffectDef::LoseLife {
                    recipient: EffectRecipientDef::Opponent,
                    amount: ValueDef::Constant(1),
                },
            ),
        ]),
);

// WAR 242 — Prismite
// Audit: unsupported — The mana-ability planner rejects repeatable activations paid solely with mana; its loop bound currently requires spending a source, permanent, card, life, or activation limit.
pub(in crate::card::sets) static PRISMITE_242: CardRecord = CardRecord::new(
    "Prismite",
    "40475e96-0283-445f-97fb-1da008707399",
    "Alayna Danner",
    crate::card::CardRules::unsupported(),
);

// WAR 244 — Blast Zone
// Audit: unsupported — The activated-cost planner supports a single X symbol only; it cannot enumerate and pay the doubled variable mana cost {X}{X}.
pub(in crate::card::sets) static BLAST_ZONE_244: CardRecord = CardRecord::new(
    "Blast Zone",
    "ea6bc7d5-e8f6-4103-920c-9f7ec5cd6c28",
    "Chris Ostrowski",
    CardRules::unsupported(),
);

// WAR 245 — Emergence Zone
pub(in crate::card::sets) static EMERGENCE_ZONE_245: CardRecord = CardRecord::new(
    "Emergence Zone",
    "ab95f6e7-b806-47fe-a071-6c38b3176d94",
    "Jonas De Ro",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{1}, {T}, Sacrifice this land: You may cast spells this turn as though they had flash.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource, CostDef::SacrificeSource],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MayCastAsThoughItHadFlash(
                    CastTimingPermissionDef::new(ObjectPredicateDef::Any),
                )),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// WAR 275 — Tezzeret, Master of the Bridge
// Audit: unsupported — The cast-cost scanner reads intrinsic payment keywords but cannot grant affinity to candidate creature and planeswalker spells from a battlefield static ability. A generic discount would not actually grant the printed affinity ability.
pub(in crate::card::sets) static TEZZERET_MASTER_OF_THE_BRIDGE_275: CardRecord = CardRecord::new(
    "Tezzeret, Master of the Bridge",
    "9ee1eea2-961f-445d-b035-6baed454f289",
    "Chase Stone",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &KARN_THE_GREAT_CREATOR_1,
    &UGIN_THE_INEFFABLE_2,
    &FINALE_OF_REVELATION,
    &JACE_WIELDER_OF_MYSTERIES,
    &NARSET_PARTER_OF_VEILS,
    &SPARK_DOUBLE_68,
    &BOLASS_CITADEL,
    &LILIANA_DREADHORDE_GENERAL,
    &MASSACRE_GIRL_99,
    &BOLT_BEND,
    &DREADHORDE_ARCANIST,
    &FINALE_OF_PROMISE_127,
    &GRIM_INITIATE,
    &KRENKO_TIN_STREET_KINGPIN_137,
    &FINALE_OF_DEVASTATION_160,
    &NISSA_WHO_SHAKES_THE_WORLD,
    &VIVIEN_CHAMPION_OF_THE_WILDS_180,
    &DOVIN_S_VETO_193,
    &MAYHEM_DEVIL_204,
    &NEOFORM_206,
    &TAMIYO_COLLECTOR_OF_TALES,
    &TEFERI_TIME_RAVELER,
    &TENTH_DISTRICT_LEGIONNAIRE,
    &ASHIOK_DREAM_RENDER_228,
    &DOVIN_HAND_OF_CONTROL_229,
    &SAHEELI_SUBLIME_ARTIFICER,
    &GOD_PHARAOH_S_STATUE_238,
    &PRISMITE_242,
    &BLAST_ZONE_244,
    &EMERGENCE_ZONE_245,
    &TEZZERET_MASTER_OF_THE_BRIDGE_275,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
