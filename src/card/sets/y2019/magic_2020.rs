//! Core Set 2020 cards cataloged for the Vintage Cube.

use super::CardRecord;
use super::PrintingRecord;
use crate::ParentBinding;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::ComparisonDef;
use crate::card::CostDef;
use crate::card::CounterKind;
use crate::card::CreateTokenDef;
use crate::card::DamageAssignmentDef;
use crate::card::DamageDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::KeywordAbility;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::PlayActionMatcherDef;
use crate::card::PlayRestrictionDef;
use crate::card::PlayerRefDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::ReplacementEffectDef;
use crate::card::ReplacementEventDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::StackTargetAggregationDef;
use crate::card::StackTargetFilterDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::TopOfLibraryCostDef;
use crate::card::TriggerConditionDef;
use crate::card::TriggerEventDef;
use crate::card::ValueComparisonDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "M20",
    slug: "magic-2020",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

const TREASURE_TOKEN: TokenCharacteristics = crate::card::tokens::treasure().with_art(
    CardArt::new("42e54aad-ec80-4914-9ba8-91bd53924778", "Alayna Danner"),
);

// M20 3 — Ancestral Blade
pub(in crate::card::sets) static ANCESTRAL_BLADE: CardRecord = CardRecord::new(
    "Ancestral Blade",
    "2ba18114-af6c-48cd-82c9-eb6541d566bf",
    "Scott Murphy",
    // Two mana buys a 2/2 that leaves an Equipment behind, which is what
    // makes it playable in a deck with no other artifacts to care about.
    CardRules::new_artifact(mana_cost!("{1}{W}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::enters_trigger(
                "When this Equipment enters, create a 1/1 white Soldier creature token, then \
                 attach this Equipment to it.",
                EffectDef::Sequence(&[
                    EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                        TokenCharacteristics::creature(&["Soldier"], &[ManaColor::White], 1, 1),
                    ))),
                    // The attach names the token this resolution just made
                    // rather than any Soldier, so an existing one is never
                    // picked up instead.
                    EffectDef::AttachToSource {
                        object: EffectRecipientDef::objects(ObjectSetDef::TokensCreatedBy(
                            ObjectRefDef::Source,
                        )),
                    },
                ]),
            ),
            AbilityDef::static_ability(
                "Equipped creature gets +1/+1.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(1),
                        ValueDef::Constant(1),
                    ),
                },
            ),
            abilities::equip(
                &[CostDef::Mana(mana_cost!("{1}"))],
                "Equip {1} ({1}: Attach to target creature you control. Equip only as a sorcery.)",
            ),
        ]),
);

// M20 4 — Angel of Vitality
pub(in crate::card::sets) static ANGEL_OF_VITALITY: CardRecord = CardRecord::new(
    "Angel of Vitality",
    "e2f39777-b80a-4618-9310-a9e5b91bb2a2",
    "Johannes Voss",
    CardRules::new_creature(mana_cost!("{2}{W}"), &["Angel"], 2, 2).with_abilities(&[
        abilities::flying(),
        AbilityDef::replacement_for(
            "If you would gain life, you gain that much life plus 1 instead.",
            ReplacementEventDef::WouldGainLife(PlayerRelation::You),
            ReplacementEffectDef::AddToEventAmount(1),
        ),
        AbilityDef::static_ability(
            "This creature gets +2/+2 as long as you have 25 or more life.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::Not(&TriggerConditionDef::ControllerLifeAtMost(
                    24,
                )),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                },
            },
        ),
    ]),
);

// M20 13 — Devout Decree
pub(in crate::card::sets) static DEVOUT_DECREE: CardRecord = CardRecord::new(
    "Devout Decree",
    "2dcde8fe-d4a4-4c6e-926e-c4a1b45045e4",
    "Zoltan Boros",
    CardRules::new_sorcery(mana_cost!("{1}{W}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Exile target creature or planeswalker that's black or red. \
         Scry 1. (Look at the top card of your library. You may put \
         that card on the bottom.)",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::All(&[
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                ]),
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Color(ManaColor::Black),
                    ObjectPredicateDef::Color(ManaColor::Red),
                ]),
            ]),
        )],
        EffectDef::Sequence(&[
            EffectDef::move_to_zone(
                EffectRecipientDef::Target(TargetIndex::PRIMARY),
                ZoneKind::Exile,
                ZonePlacement::Top,
            ),
            abilities::scry(ValueDef::Constant(1)),
        ]),
    )]),
);

// M20 17 — Gauntlets of Light
pub(in crate::card::sets) static GAUNTLETS_OF_LIGHT_17: CardRecord = CardRecord::new(
    "Gauntlets of Light",
    "da0d5436-b881-45ac-b8ec-248d88714021",
    "Ekaterina Burmak",
    CardRules::new_enchantment(mana_cost!("{2}{W}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_creature(),
            AbilityDef::static_ability(
                "Enchanted creature gets +0/+2 and assigns combat damage equal to its toughness rather than its power.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&[
                        AppliedEffectDef::modify_power_toughness(
                            ValueDef::Constant(0),
                            ValueDef::Constant(2),
                        ),
                        AppliedEffectDef::Rule(
                            AppliedRuleDef::AssignsCombatDamageEqualToToughness,
                        ),
                    ]),
                },
            ),
            AbilityDef::static_ability(
                "Enchanted creature has \"{2}{W}: Untap this creature.\"",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::add_ability(&AbilityDef::activated(
                        "{2}{W}: Untap this creature.",
                        &[CostDef::Mana(mana_cost!("{2}{W}"))],
                        EffectDef::Untap {
                            object: EffectRecipientDef::Source,
                        },
                    )),
                },
            ),
        ]),
);

// M20 34 — Raise the Alarm (reprint)
const RAISE_THE_ALARM_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2003::mirrodin::RAISE_THE_ALARM,
    "764a7a53-314e-4b1f-aa33-0f312d06df71",
    "Zoltan Boros",
);

// M20 50 — Brineborn Cutthroat
pub(in crate::card::sets) static BRINEBORN_CUTTHROAT: CardRecord = CardRecord::new(
    "Brineborn Cutthroat",
    "0857765f-afd7-418a-a93b-c0bd1b1f037e",
    "Caio Monteiro",
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Merfolk", "Pirate"], 2, 1).with_abilities(&[
        abilities::flash(),
        AbilityDef::triggered_if(
            "Whenever you cast a spell during an opponent's turn, put a \
             +1/+1 counter on this creature.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::Any,
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            &TriggerConditionDef::ActivePlayer(PlayerRelation::Opponent),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

// M20 54 — Cloudkin Seer
pub(in crate::card::sets) static CLOUDKIN_SEER: CardRecord = CardRecord::new(
    "Cloudkin Seer",
    "e2111753-a930-403f-9d94-a86dfcb069da",
    "Anastasia Ovchinnikova",
    // A flier that replaces itself, which is the rate every blue common
    // three-drop is measured against.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Elemental", "Wizard"], 2, 1).with_abilities(
        &[
            abilities::flying(),
            abilities::enters_trigger(
                "When this creature enters, draw a card.",
                EffectDef::DrawCards {
                    recipient: EffectRecipientDef::Controller,
                    amount: ValueDef::Constant(1),
                },
            ),
        ],
    ),
);

// M20 74 — Scholar of the Ages
pub(in crate::card::sets) static SCHOLAR_OF_THE_AGES_74: CardRecord = CardRecord::new(
    "Scholar of the Ages",
    "80137c9a-ea56-4dc7-a503-43fe192c8fce",
    "Micah Epstein",
    CardRules::new_creature(mana_cost!("{5}{U}{U}"), &["Human", "Wizard"], 3, 3).with_abilities(&[
abilities::enters_trigger_with_targets("When this creature enters, return up to two target instant and/or sorcery cards from your graveyard to your hand.", &[AbilityTargetDef::up_to(AbilityTargetPredicate::Object { object: ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::HasType(CardType::Instant), ObjectPredicateDef::HasType(CardType::Sorcery)]), zones: &[ZoneKind::Graveyard], controller: None, owner: Some(PlayerRelation::You) }, 2)], EffectDef::move_to_zone(EffectRecipientDef::Target(TargetIndex::PRIMARY), ZoneKind::Hand, ZonePlacement::Top))
]),
);

// M20 76 — Spectral Sailor
pub(in crate::card::sets) static SPECTRAL_SAILOR: CardRecord = CardRecord::new(
    "Spectral Sailor",
    "67483891-36d1-46f2-8b4f-b8b7bd54bdcc",
    "Cristi Balanescu",
    CardRules::new_creature(mana_cost!("{U}"), &["Spirit", "Pirate"], 1, 1).with_abilities(&[
        abilities::flash(),
        abilities::flying(),
        AbilityDef::activated(
            "{3}{U}: Draw a card.",
            &[CostDef::Mana(mana_cost!("{3}{U}"))],
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
    ]),
);

// M20 77 — Tale's End
pub(in crate::card::sets) static TALE_S_END_77: CardRecord = CardRecord::new(
    "Tale's End",
    "1421115b-9a98-4ab2-bcb2-7d8899ce12db",
    "Randy Vargas",
    CardRules::new_instant(mana_cost!("{1}{U}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Counter target activated ability, triggered ability, or legendary spell.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::Object {
                object: ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::Ability,
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Spell,
                        ObjectPredicateDef::Supertype(CardSupertype::Legendary),
                    ]),
                ]),
                zones: &[ZoneKind::Stack],
                controller: None,
                owner: None,
            },
        )],
        EffectDef::counter_target(TargetIndex::PRIMARY),
    )]),
);

// M20 113 — Scheming Symmetry
pub(in crate::card::sets) static SCHEMING_SYMMETRY_113: CardRecord = CardRecord::new(
    "Scheming Symmetry",
    "01acc50b-856d-442d-9880-1a892b40643b",
    "Seb McKinnon",
    CardRules::new_sorcery(mana_cost!("{B}")).with_abilities(&[
AbilityDef::spell_with_targets("Choose two target players. Each of them searches their library for a card, then shuffles and puts that card on top.", &[AbilityTargetDef::exactly_value(AbilityTargetPredicate::Player(PlayerRelation::Any), ValueDef::Constant(2))], EffectDef::SearchZone { player: EffectRecipientDef::Target(TargetIndex::PRIMARY), source: ZoneKind::Library, object: ObjectPredicateDef::Any, minimum: 1, maximum: ValueDef::Constant(1), reveal: false, destination: ZoneKind::Library, placement: ZonePlacement::Top, shuffle: true, enters_tapped: false, attachment: None, binding: None, then: None })
]),
);

// M20 122 — Vilis, Broker of Blood
// Audit: unsupported — Needs a life-loss event carrying its amount into a draw effect; existing damage and payment events do not provide a general loss-of-life trigger value.
pub(in crate::card::sets) static VILIS_BROKER_OF_BLOOD_122: CardRecord = CardRecord::new(
    "Vilis, Broker of Blood",
    "ecdf2bd9-87b9-470a-ad2e-0ebf98560f87",
    "Tyler Jacobson",
    crate::card::CardRules::unsupported(),
);

// M20 136 — Drakuseth, Maw of Flames
pub(in crate::card::sets) static DRAKUSETH_MAW_OF_FLAMES: CardRecord = CardRecord::new(
    "Drakuseth, Maw of Flames",
    "d09af78f-efde-4107-8406-cb12fd11c686",
    "Grzegorz Rutkowski",
    CardRules::new_creature(mana_cost!("{4}{R}{R}{R}"), &["Dragon"], 7, 7)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::flying(),
            AbilityDef::triggered_with_targets(
                "Whenever Drakuseth attacks, it deals 4 damage to any target \
                 and 3 damage to each of up to two other targets.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                &[
                    AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget),
                    AbilityTargetDef {
                        minimum: 0,
                        maximum: 2,
                        another: true,
                        ..AbilityTargetDef::exactly_one(AbilityTargetPredicate::AnyTarget)
                    },
                ],
                EffectDef::DealDamage(DamageDef::simultaneous(&[
                    DamageAssignmentDef::from_effect(
                        EffectRecipientDef::Target(TargetIndex::PRIMARY),
                        ValueDef::Constant(4),
                    ),
                    DamageAssignmentDef::from_effect(
                        EffectRecipientDef::Target(TargetIndex(1)),
                        ValueDef::Constant(3),
                    ),
                ])),
            ),
        ]),
);

// M20 141 — Glint-Horn Buccaneer
pub(in crate::card::sets) static GLINT_HORN_BUCCANEER_141: CardRecord = CardRecord::new(
    "Glint-Horn Buccaneer",
    "df2df9cb-14f5-470f-b438-20f4ae8d0d59",
    "Zack Stella",
    CardRules::new_creature(mana_cost!("{1}{R}{R}"), &["Minotaur", "Pirate"], 2, 4).with_abilities(
        &[
            abilities::haste(),
            AbilityDef::triggered(
                "Whenever you discard a card, this creature deals 1 damage to each opponent.",
                TriggerEventDef::Discarded(PlayerRelation::You),
                EffectDef::damage(EffectRecipientDef::Opponent, ValueDef::Constant(1)),
            ),
            AbilityDef::activated(
                "{1}{R}, Discard a card: Draw a card. Activate only if this creature is attacking.",
                &[
                    CostDef::Mana(mana_cost!("{1}{R}")),
                    CostDef::discard(ObjectPredicateDef::Any),
                ],
                abilities::draw_cards(ValueDef::Constant(1)),
            )
            .with_activation_condition(&TriggerConditionDef::SourceMatches {
                object: ObjectPredicateDef::Attacking,
            }),
        ],
    ),
);

// M20 144 — Goblin Smuggler
pub(in crate::card::sets) static GOBLIN_SMUGGLER: CardRecord = CardRecord::new(
    "Goblin Smuggler",
    "95dc1a65-271c-455a-ae0c-f652444a53ac",
    "Dan Murayama Scott",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin", "Rogue"], 2, 2).with_abilities(&[
        abilities::haste(),
        AbilityDef::activated_with_targets(
            "{T}: Another target creature with power 2 or less can't be \
             blocked this turn.",
            &[CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ObjectPredicateDef::PowerLessThan(ValueDef::Constant(3)),
                ]),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// M20 148 — Leyline of Combustion
pub(in crate::card::sets) static LEYLINE_OF_COMBUSTION: CardRecord = CardRecord::new(
    "Leyline of Combustion",
    "3a93c8e2-fb27-43af-83a7-2bd4d40e0eff",
    "Noah Bradley",
CardRules::new_enchantment(mana_cost!("{2}{R}{R}")).with_abilities(&[
        abilities::begin_game_on_battlefield(),
        AbilityDef::triggered(
            "Whenever you and/or at least one permanent you control becomes the target of a spell or ability an opponent controls, this enchantment deals 2 damage to that player.",
            TriggerEventDef::targets_selected(
                ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
                StackTargetFilterDef::AnyOf(&[
                    StackTargetFilterDef::Player(PlayerRelation::You),
                    StackTargetFilterDef::Permanent(ObjectPredicateDef::ControlledBy(
                        PlayerRelation::You,
                    )),
                ]),
                StackTargetAggregationDef::OneOrMoreMatchingTargets,
            ),
            EffectDef::damage(EffectRecipientDef::EventPlayer, ValueDef::Constant(2)),
        ),
    ]),
);

// M20 153 — Rapacious Dragon
pub(in crate::card::sets) static RAPACIOUS_DRAGON: CardRecord = CardRecord::new(
    "Rapacious Dragon",
    "2c9bf6d8-ebf6-40ff-858a-3483d19bb584",
    "Johan Grenier",
    CardRules::new_creature(mana_cost!("{4}{R}"), &["Dragon"], 3, 3).with_abilities(&[
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, create two Treasure tokens. \
             (They're artifacts with \"{T}, Sacrifice this token: Add one \
             mana of any color.\")",
            EffectDef::CreateToken(
                CreateTokenDef::new(TokenDef::Literal(TREASURE_TOKEN))
                    .with_count(ValueDef::Constant(2)),
            ),
        ),
    ]),
);

// M20 169 — Elvish Reclaimer
pub(in crate::card::sets) static ELVISH_RECLAIMER: CardRecord = CardRecord::new(
    "Elvish Reclaimer",
    "39c431d7-d94b-46c4-bb89-f3db56214ab4",
    "Victor Adame Minguez",
    // One mana for a body that turns a spent fetchland into whatever land
    // the deck is built around, and is a 3/4 by the time it has done it
    // twice.
    CardRules::new_creature(mana_cost!("{G}"), &["Elf", "Warrior"], 1, 2).with_abilities(&[
        AbilityDef::static_ability(
            "This creature gets +2/+2 as long as there are three or more land cards in your \
             graveyard.",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::ObjectCount {
                    // "Three or more land cards in your graveyard": the fetchlands that made
                    // him a 3/4 are the same ones his own ability puts there, which is why he
                    // grows on the turn he is used.
                    query: ObjectQueryDef::owned_by(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Graveyard],
                        PlayerSetDef::Related(PlayerRelation::You),
                    ),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 3,
                },
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::Constant(2),
                        ValueDef::Constant(2),
                    ),
                },
            },
        ),
        AbilityDef::activated(
            "{2}, {T}, Sacrifice a land: Search your library for a land card, put it onto the \
             battlefield tapped, then shuffle.",
            &[
                CostDef::Mana(mana_cost!("{2}")),
                CostDef::TapSource,
                CostDef::SacrificePermanent {
                    object: ObjectPredicateDef::HasType(CardType::Land),
                    controller: PlayerRelation::You,
                },
            ],
            EffectDef::SearchZone {
                player: EffectRecipientDef::Controller,
                source: ZoneKind::Library,
                object: ObjectPredicateDef::HasType(CardType::Land),
                minimum: 0,
                maximum: ValueDef::Constant(1),
                reveal: false,
                destination: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                shuffle: true,
                enters_tapped: true,
                attachment: None,
                binding: None,
                then: None,
            },
        ),
    ]),
);

// M20 179 — Leyline of Abundance
pub(in crate::card::sets) static LEYLINE_OF_ABUNDANCE: CardRecord = CardRecord::new(
    "Leyline of Abundance",
    "c68e8342-78d2-4826-a287-64c371b97d19",
    "Noah Bradley",
    CardRules::new_enchantment(mana_cost!("{2}{G}{G}")).with_abilities(&[
        abilities::begin_game_on_battlefield(),
        AbilityDef::triggered_mana(
            "Whenever you tap a creature for mana, add an additional {G}.",
            TriggerEventDef::tapped_for_mana(ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Creature),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Green)),
        ),
        AbilityDef::activated(
            "{6}{G}{G}: Put a +1/+1 counter on each creature you control.",
            &[CostDef::Mana(mana_cost!("{6}{G}{G}"))],
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
    ]),
);

// M20 198 — Veil of Summer
// Audit: unsupported — Hexproof has no color-specific player or permanent rule. Ordinary hexproof would also stop opposing red, green, white, and colorless sources.
pub(in crate::card::sets) static VEIL_OF_SUMMER_198: CardRecord = CardRecord::new(
    "Veil of Summer",
    "aa686c34-1c11-469f-93c2-f9891aea521f",
    "Lake Hurwitz",
    crate::card::CardRules::unsupported(),
);

// M20 208 — Empyrean Eagle
pub(in crate::card::sets) static EMPYREAN_EAGLE: CardRecord = CardRecord::new(
    "Empyrean Eagle",
    "ac555709-c7cc-4c64-8a6f-8fe2bc149fcd",
    "Jason A. Engle",
    CardRules::new_creature(mana_cost!("{1}{W}{U}"), &["Bird", "Spirit"], 2, 3).with_abilities(&[
        abilities::flying(),
        AbilityDef::static_ability(
            "Other creatures you control with flying get +1/+1.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::objects(ObjectSetDef::Query(
                    ObjectQueryDef::matching(
                        ObjectPredicateDef::All(&[
                            ObjectPredicateDef::HasType(CardType::Creature),
                            ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                            ObjectPredicateDef::HasKeyword(KeywordAbility::Flying),
                        ]),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    ),
                )),
                effect: AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(1),
                    ValueDef::Constant(1),
                ),
            },
        ),
    ]),
);

// M20 222 — Bag of Holding
// Audit: unsupported — Discard-trigger snapshots are inconsistent across payment paths: resolving discards identify the new card, while some casting costs retain the retired hand identity without a zone-change result. The linked exile cannot reliably select exactly the discarded graveyard incarnation across those paths.
pub(in crate::card::sets) static BAG_OF_HOLDING_222: CardRecord = CardRecord::new(
    "Bag of Holding",
    "49283832-54f2-4619-b4a9-750493c93292",
    "Dmitry Burmak",
    crate::card::CardRules::unsupported(),
);

// M20 230 — Manifold Key
pub(in crate::card::sets) static MANIFOLD_KEY: CardRecord = CardRecord::new(
    "Manifold Key",
    "715e637a-dfd8-45a0-b1ea-53e4abd29307",
    "Lake Hurwitz",
    // One mana that untaps a Mox for profit and, when there is nothing to
    // untap, pushes a creature through instead.
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{1}, {T}: Untap another target artifact.",
            &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource],
            // "Another" excludes the Key itself, which is what stops it untapping
            // itself for free every turn.
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Artifact),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                ]),
            )],
            EffectDef::Untap {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            },
        ),
        AbilityDef::activated_with_targets(
            "{3}, {T}: Target creature can't be blocked this turn.",
            &[CostDef::Mana(mana_cost!("{3}")), CostDef::TapSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::Apply {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                effect: AppliedEffectDef::Rule(AppliedRuleDef::CANNOT_BE_BLOCKED),
                duration: ResolvedEffectDurationDef::UntilEndOfTurn,
            },
        ),
    ]),
);

// M20 233 — Mystic Forge
pub(in crate::card::sets) static MYSTIC_FORGE_233: CardRecord = CardRecord::new(
    "Mystic Forge",
    "924a24e7-91b8-4ceb-a136-7a765d98c994",
    "Titus Lunter",
    CardRules::new_artifact(mana_cost!("{4}")).with_abilities(&[
        AbilityDef::static_ability(
            "You may look at the top card of your library any time.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MayLookAtTopOfLibrary),
            },
        ),
        AbilityDef::static_ability(
            "You may cast artifact spells and colorless spells from the top of your library.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Controller,
                effect: AppliedEffectDef::Rule(AppliedRuleDef::MayPlayFromTopOfLibrary {
                    restriction: PlayRestrictionDef::new(
                        PlayActionMatcherDef::CastSpell,
                        ObjectPredicateDef::AnyOf(&[
                            ObjectPredicateDef::HasType(CardType::Artifact),
                            ObjectPredicateDef::ColorCount(0),
                        ]),
                    ),
                    cost: TopOfLibraryCostDef::Printed,
                }),
            },
        ),
        AbilityDef::activated(
            "{T}, Pay 1 life: Exile the top card of your library.",
            &[CostDef::TapSource, CostDef::PayLife(1)],
            abilities::bind_top_cards_then(
                PlayerRefDef::EffectController,
                ValueDef::Constant(1),
                &EffectDef::move_to_zone(
                    EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding)),
                    ZoneKind::Exile,
                    ZonePlacement::Top,
                ),
            ),
        ),
    ]),
);

// M20 244 — Cryptic Caves
pub(in crate::card::sets) static CRYPTIC_CAVES: CardRecord = CardRecord::new(
    "Cryptic Caves",
    "fde9e9cb-68ab-4856-8ad6-30f66666dd93",
    "Sung Choi",
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated(
            "{1}, {T}, Sacrifice this land: Draw a card. Activate only if \
             you control five or more lands.",
            &[
                CostDef::Mana(mana_cost!("{1}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            abilities::draw_cards(ValueDef::Constant(1)),
        )
        .with_activation_condition(&TriggerConditionDef::ObjectCount {
            query: ObjectQueryDef::matching(
                ObjectPredicateDef::HasType(CardType::Land),
                &[ZoneKind::Battlefield],
                PlayerRelation::You,
            ),
            comparison: ComparisonDef::GreaterOrEqual,
            amount: 5,
        }),
    ]),
);

// M20 247 — Field of the Dead
pub(in crate::card::sets) static FIELD_OF_THE_DEAD: CardRecord = CardRecord::new(
    "Field of the Dead",
    "470ca3f4-29aa-4c4c-8ff2-8cdd70c69943",
    "Kev Walker",
    // A land that makes colourless and comes in tapped, which is what a deck
    // pays for turning every land drop after the seventh into a 2/2.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_tapped(CardType::Land),
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::triggered_if(
            "Whenever this land or another land you control enters, if you control seven or more \
             lands with different names, create a 2/2 black Zombie creature token.",
            // "This land or another land you control": the Field's own arrival counts,
            // which is what makes the seventh land the one that starts it.
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            &// The Field itself is one of the seven, and so is every other land you
                // control -- what is counted is names rather than lands, which is why a
                // deck built for this plays one of each dual rather than four of one.
                TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::DistinctNamesAmong(&ObjectQueryDef::matching(
                        ObjectPredicateDef::HasType(CardType::Land),
                        &[ZoneKind::Battlefield],
                        PlayerRelation::You,
                    )),
                    comparison: ComparisonDef::GreaterOrEqual,
                    right: ValueDef::Constant(7),
                }),
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(
                TokenCharacteristics::creature(&["Zombie"], &[ManaColor::Black], 2, 2).with_art(
                    CardArt::new("18f0436e-9328-4266-9cf8-80b557a0c17c", "Anna Steinbauer"),
                ),
            ))),
        ),
    ]),
);

// M20 285 — Twinblade Paladin
pub(in crate::card::sets) static TWINBLADE_PALADIN: CardRecord = CardRecord::new(
    "Twinblade Paladin",
    "6397d426-00e0-44da-b23c-44ccea65f5aa",
    "Jana Schirmer",
    CardRules::new_creature(mana_cost!("{3}{W}"), &["Human", "Knight"], 3, 3).with_abilities(&[
        AbilityDef::triggered(
            "Whenever you gain life, put a +1/+1 counter on this creature.",
            TriggerEventDef::LifeGained(PlayerRelation::You),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
        AbilityDef::static_ability(
            "As long as you have 25 or more life, this creature has double \
             strike. (It deals both first-strike and regular combat \
             damage.)",
            EffectDef::IfCondition {
                condition: &TriggerConditionDef::Not(&TriggerConditionDef::ControllerLifeAtMost(
                    24,
                )),
                then: &EffectDef::StaticApply {
                    recipient: EffectRecipientDef::Source,
                    effect: AppliedEffectDef::add_ability(&abilities::double_strike()),
                },
            },
        ),
    ]),
);

// M20 297 — Wildfire Elemental
// Audit: unsupported — Needs a noncombat damage matcher. DamageKindDef offers only Any and Combat, so "whenever an opponent is dealt noncombat damage" cannot be said; using Any would also fire on every attack, which is the opposite of what the card rewards.
pub(in crate::card::sets) static WILDFIRE_ELEMENTAL: CardRecord = CardRecord::new(
    "Wildfire Elemental",
    "272e317c-55c4-43b2-91aa-3e0009cfd7d5",
    "Svetlin Velinov",
    crate::card::CardRules::unsupported(),
);

// M20 300 — Gnarlback Rhino
pub(in crate::card::sets) static GNARLBACK_RHINO: CardRecord = CardRecord::new(
    "Gnarlback Rhino",
    "68a69558-aca0-413d-9762-2fa115b44abd",
    "YW Tang",
    CardRules::new_creature(mana_cost!("{2}{G}{G}"), &["Rhino"], 4, 4).with_abilities(&[
        abilities::trample(),
        AbilityDef::triggered(
            "Whenever you cast a spell that targets this creature, draw a \
             card.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::TargetsObjectMatching(&ObjectPredicateDef::Source),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            abilities::draw_cards(ValueDef::Constant(1)),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ANCESTRAL_BLADE,
    &ANGEL_OF_VITALITY,
    &DEVOUT_DECREE,
    &GAUNTLETS_OF_LIGHT_17,
    &BRINEBORN_CUTTHROAT,
    &CLOUDKIN_SEER,
    &SCHOLAR_OF_THE_AGES_74,
    &SPECTRAL_SAILOR,
    &TALE_S_END_77,
    &SCHEMING_SYMMETRY_113,
    &VILIS_BROKER_OF_BLOOD_122,
    &DRAKUSETH_MAW_OF_FLAMES,
    &GLINT_HORN_BUCCANEER_141,
    &GOBLIN_SMUGGLER,
    &LEYLINE_OF_COMBUSTION,
    &RAPACIOUS_DRAGON,
    &ELVISH_RECLAIMER,
    &LEYLINE_OF_ABUNDANCE,
    &VEIL_OF_SUMMER_198,
    &EMPYREAN_EAGLE,
    &BAG_OF_HOLDING_222,
    &MANIFOLD_KEY,
    &MYSTIC_FORGE_233,
    &CRYPTIC_CAVES,
    &FIELD_OF_THE_DEAD,
    &TWINBLADE_PALADIN,
    &WILDFIRE_ELEMENTAL,
    &GNARLBACK_RHINO,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] =
    &[RAISE_THE_ALARM_REPRINT];
