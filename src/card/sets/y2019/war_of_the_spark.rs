//! War of the Spark cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AddManaEffectDef,
    AlternativeCastKindDef, AppliedEffectDef, AppliedRuleDef, BasicLandType, CardArt, CardRules,
    CardSet, CardSupertype, CardType, CardTypeSet, CastTimingPermissionDef, ComparisonDef,
    CopyExceptionsDef, CountConditionDef, CounterKind, CreatureTypeSetDef, EffectDef,
    EffectRecipientDef, ManaColor, ObjectPredicateDef, ObjectQueryDef, PlayActionMatcherDef,
    PlayRestrictionDef, PlayerRefDef, PlayerRelation, PlayerSetDef, ResolvedEffectDurationDef,
    TopOfLibraryCostDef, TriggerConditionDef, TriggerEventDef, ValueDef, ZoneKind, ZonePlacement,
    abilities,
};
use crate::ids::ParentBinding;
use crate::{TargetIndex, mana_cost};

// WAR 54 — Jace, Wielder of Mysteries
pub(in crate::card::sets) static JACE_WIELDER_OF_MYSTERIES: CardRecord =
    CardRecord::new_with_legacy_id(
        2160,
        "Jace, Wielder of Mysteries",
        CardArt::new("6adb7d73-4482-4930-8497-cffd169b57e2", "Anna Steinbauer"),
        CardSet::WarOfTheSpark,
        CardRules::new_planeswalker(mana_cost!("{1}{U}{U}{U}"), &["Jace"], 4)
            .with_supertype(CardSupertype::Legendary)
            .with_abilities(&[
                abilities::empty_library_draw_wins(),
                AbilityDef::activated_with_targets(
                    "+1: Target player mills two cards. Draw a card.",
                    &[AbilityCostDef::Loyalty(1)],
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
                    &[AbilityCostDef::Loyalty(-8)],
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
    PrintingAnchor::scryfall("8c39f9b4-02b9-4d44-b8d6-4fd02ebbb0c5"),
    "Narset, Parter of Veils",
    CardArt::new("8c39f9b4-02b9-4d44-b8d6-4fd02ebbb0c5", "Magali Villeneuve"),
    CardSet::WarOfTheSpark,
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
                &[AbilityCostDef::Loyalty(-2)],
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

// WAR 79 — Bolas's Citadel
pub(in crate::card::sets) static BOLASS_CITADEL: CardRecord = CardRecord::new_with_legacy_id(
    2253,
    "Bolas's Citadel",
    CardArt::new("d2124603-d20e-40eb-97f0-a66323397ac2", "Jonas De Ro"),
    CardSet::WarOfTheSpark,
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
                    AbilityCostDef::TapSource,
                    AbilityCostDef::SacrificePermanents {
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

// WAR 115 — Bolt Bend
pub(in crate::card::sets) static BOLT_BEND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("39b35408-3728-4e1b-9f58-b0775df914d6"),
    "Bolt Bend",
    CardArt::new("39b35408-3728-4e1b-9f58-b0775df914d6", "Svetlin Velinov"),
    CardSet::WarOfTheSpark,
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
pub(in crate::card::sets) static DREADHORDE_ARCANIST: CardRecord = CardRecord::new_with_legacy_id(
    2279,
    "Dreadhorde Arcanist",
    CardArt::new("fd97b3cf-924e-4f77-bb82-0bf19592389f", "G-host Lee"),
    CardSet::WarOfTheSpark,
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
                    // What the card is lent while the offer stands. The kind says both halves
                    // of the printed clause at once: the cast costs nothing, and the card is
                    // exiled rather than buried afterwards.
                    ability: &AbilityDef::alternative_cast(
                        mana_cost!("{0}"),
                        AlternativeCastKindDef::WithoutPayingManaCost,
                        Some("Cast without paying its mana cost, then exile it."),
                        EffectDef::None,
                    ),
                },
            ),
        ]),
);

// WAR 130 — Grim Initiate
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GRIM_INITIATE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("29b6ec9d-3861-48bf-a198-dc7efba5d89c"),
    "Grim Initiate",
    crate::card::CardArt::new("29b6ec9d-3861-48bf-a198-dc7efba5d89c", "Jason Felix"),
    crate::card::CardSet::WarOfTheSpark,
    crate::card::CardRules::unsupported(),
);

// WAR 169 — Nissa, Who Shakes the World
pub(in crate::card::sets) static NISSA_WHO_SHAKES_THE_WORLD: CardRecord =
    CardRecord::new_with_legacy_id(
        2172,
        "Nissa, Who Shakes the World",
        CardArt::new("41e108a5-4e2f-42cf-9ea1-87bf3c0a2b7f", "Chris Rallis"),
        CardSet::WarOfTheSpark,
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
                    &[AbilityCostDef::Loyalty(1)],
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
                    &[AbilityCostDef::Loyalty(-8)],
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
    .with_identity_anchor(PrintingAnchor::scryfall(
        "f857bbe4-5619-4733-a0c7-69700f2ef4f3",
    ));

// WAR 220 — Tamiyo, Collector of Tales
pub(in crate::card::sets) static TAMIYO_COLLECTOR_OF_TALES: CardRecord =
    CardRecord::new_with_legacy_id(
        2186,
        "Tamiyo, Collector of Tales",
        CardArt::new("786d89de-da0c-47af-80ae-2734dc0514fc", "Chase Stone"),
        CardSet::WarOfTheSpark,
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
                    &[AbilityCostDef::Loyalty(1)],
                    // The binding the name-choice makes is unused here: what matches is decided
                    // among the four revealed cards rather than across a whole zone, so the
                    // selection reads the name itself.
                    EffectDef::ChooseCardName {
                        chooser: PlayerRefDef::EffectController,
                        nonland_only: true,
                        matched_in: PlayerRefDef::EffectController,
                        zone: ZoneKind::Library,
                        binding: ParentBinding,
                        // The name is chosen before the four cards are seen, so the reveal cannot
                        // be used to pick a name that is already there.
                        then: &abilities::reveal_top_cards_put_matching_in_hand_rest_graveyard(
                            ValueDef::Constant(4),
                            ObjectPredicateDef::HasChosenName,
                        ),
                    },
                ),
                AbilityDef::activated_with_targets(
                    "\u{2212}3: Return target card from your graveyard to your hand.",
                    &[AbilityCostDef::Loyalty(-3)],
                    &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::Any,
                            zones: &[ZoneKind::Graveyard],
                            controller: None,
                            owner: Some(PlayerRelation::You),
                        },
                    )],
                    EffectDef::MoveToZone {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        zone: ZoneKind::Hand,
                        placement: ZonePlacement::Top,
                    },
                ),
            ]),
    )
    .with_identity_anchor(PrintingAnchor::scryfall(
        "76776b24-a2e1-4590-88e7-8a421baf2fc4",
    ));

// WAR 221 — Teferi, Time Raveler
pub(in crate::card::sets) static TEFERI_TIME_RAVELER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5cb76266-ae50-4bbc-8f96-d98f309b02d3"),
    "Teferi, Time Raveler",
    CardArt::new("5cb76266-ae50-4bbc-8f96-d98f309b02d3", "Chris Rallis"),
    CardSet::WarOfTheSpark,
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
                &[AbilityCostDef::Loyalty(1)],
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
                &[AbilityCostDef::Loyalty(-3)],
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
                    EffectDef::MoveToZone {
                        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        zone: ZoneKind::Hand,
                        placement: ZonePlacement::Top,
                    },
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                ]),
            ),
        ]),
);

// WAR 222 — Tenth District Legionnaire
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TENTH_DISTRICT_LEGIONNAIRE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("44f3090b-917b-4122-b522-27c30dca8e69"),
    "Tenth District Legionnaire",
    crate::card::CardArt::new(
        "44f3090b-917b-4122-b522-27c30dca8e69",
        "Victor Adame Minguez",
    ),
    crate::card::CardSet::WarOfTheSpark,
    crate::card::CardRules::unsupported(),
);

// WAR 234 — Saheeli, Sublime Artificer
pub(in crate::card::sets) static SAHEELI_SUBLIME_ARTIFICER: CardRecord =
    CardRecord::new_with_legacy_id(
        2247,
        "Saheeli, Sublime Artificer",
        CardArt::new("5a10b543-d5d4-42a8-9ee8-dada59a2ad7e", "Wesley Burt"),
        CardSet::WarOfTheSpark,
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
                    EffectDef::create_artifact_creature_token(&["Servo"], &[], 1, 1).with_art(CardArt::new(
                        "761507d5-d36a-4123-a074-95d7f6ffb4c5",
                        "Victor Adame Minguez",
                    )),
                ),
                AbilityDef::activated_with_targets(
                    "−2: Target artifact you control becomes a copy of another target artifact or creature \
                     you control until end of turn, except it's an artifact in addition to its other types.",
                    &[AbilityCostDef::Loyalty(-2)],
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

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &JACE_WIELDER_OF_MYSTERIES,
    &NARSET_PARTER_OF_VEILS,
    &BOLASS_CITADEL,
    &BOLT_BEND,
    &DREADHORDE_ARCANIST,
    &GRIM_INITIATE,
    &NISSA_WHO_SHAKES_THE_WORLD,
    &TAMIYO_COLLECTOR_OF_TALES,
    &TEFERI_TIME_RAVELER,
    &TENTH_DISTRICT_LEGIONNAIRE,
    &SAHEELI_SUBLIME_ARTIFICER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
