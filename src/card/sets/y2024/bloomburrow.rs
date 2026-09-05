//! Bloomburrow cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, ActivationTimingDef,
    AddManaEffectDef, AlternativeCastKindDef, AppliedEffectDef, CardArt, CardRules, CardSet,
    CardSupertype, CardType, ComparisonDef, CopyExceptionsDef, CopyStackObjectDef, CounterKind,
    DiscardSelectionDef, EffectDef, EffectRecipientDef, ManaColor, ObjectPredicateDef,
    ObjectSetDef, PlayerRefDef, PlayerRelation, SpellAdditionalCostDef, TriggerConditionDef,
    TriggerEventDef, ValueComparisonDef, ValueDef, ZoneKind, ZonePlacement, abilities,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

// BLB 54 — Kitsa, Otterball Elite
pub(in crate::card::sets) static KITSA_OTTERBALL_ELITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("c8ff751a-ec64-41d5-b22c-2a483ad9a9b2"),
    "Kitsa, Otterball Elite",
    CardArt::new("c8ff751a-ec64-41d5-b22c-2a483ad9a9b2", "Zoltan Boros"),
    CardSet::Bloomburrow,
    // Two mana for a body that loots every turn it has nothing better to
    // do, and copies the spell that made it big enough on the turns it
    // does. Vigilance is why the tap is not a real cost.
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Otter", "Wizard"], 1, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::vigilance(),
            abilities::prowess(),
            AbilityDef::activated(
                "{T}: Draw a card, then discard a card.",
                &[AbilityCostDef::TapSource],
                EffectDef::Sequence(&[
                    EffectDef::DrawCards {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                    },
                    EffectDef::Discard {
                        recipient: EffectRecipientDef::Controller,
                        amount: ValueDef::Constant(1),
                        selection: DiscardSelectionDef::RecipientChooses,
                        then: None,
                    },
                ]),
            ),
            AbilityDef::activated_with_targets(
                "{2}, {T}: Copy target instant or sorcery spell you control. You may choose new targets \
                 for the copy. Activate only if Kitsa's power is 3 or greater.",
                &[
                    AbilityCostDef::Mana(mana_cost!("{2}")),
                    AbilityCostDef::TapSource,
                ],
                // Yours rather than anybody's: Kitsa copies what you are casting, not what
                // is being cast at you.
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
                EffectDef::CopyStackObject(&CopyStackObjectDef {
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    controller: PlayerRefDef::EffectController,
                    count: ValueDef::Constant(1),
                    retarget: true,
                    colors: None,
                }),
            )
            // Read live where the activation is offered, so the prowess trigger from
            // the spell being copied is what turns the ability on: a 1/3 that has cast
            // two noncreature spells this turn is a 3/5.
            .with_activation_condition(&TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                    left: ValueDef::SourcePower,
                    comparison: ComparisonDef::GreaterOrEqual,
                    right: ValueDef::Constant(3),
                })),
        ]),
);

// BLB 75 — Stormchaser's Talent
static MAKE_AN_OTTER: EffectDef =
    EffectDef::create_creature_token(&["Otter"], &[ManaColor::Blue, ManaColor::Red], 1, 1)
        .with_abilities(&[abilities::prowess()])
        .with_art(CardArt::new(
            "e6b2c465-c446-4dee-9101-763105dcf813",
            "Julia Griffin",
        ));

pub(in crate::card::sets) static STORMCHASERS_TALENT: CardRecord = CardRecord::new_with_legacy_id(
    2232,
    "Stormchaser's Talent",
    CardArt::new("a36e682d-b43d-4e08-bf5b-70d7e924dbe5", "Christina Kraus"),
    CardSet::Bloomburrow,
    // One mana for a body, and a mana sink that buys back a spell and then
    // turns every cantrip afterwards into another creature.
    CardRules::new_enchantment(mana_cost!("{U}"))
        .with_subtypes(&["Class"])
        .with_abilities(&[
            abilities::enters_trigger(
                "When this Class enters, create a 1/1 blue and red Otter creature token with prowess.",
                MAKE_AN_OTTER,
            ),
            AbilityDef::activated(
                "{3}{U}: Level 2",
                &[AbilityCostDef::Mana(mana_cost!("{3}{U}"))],
                EffectDef::GainClassLevel { level: 2 },
            )
            .with_activation_timing(ActivationTimingDef::SorcerySpeed)
            // A Class is level 1 with no counters on it, so climbing to two takes one
            // counter and to three takes two. Each level is bought separately, only at
            // sorcery speed (CR 717.2b), and only from the level directly below it:
            // "you can't activate the first level ability of a Class unless that Class
            // is level 1."
            .with_activation_condition(&TriggerConditionDef::SourceCounters {
                kind: CounterKind::named("level"),
                comparison: ComparisonDef::Equal,
                amount: 0,
            }),
            AbilityDef::triggered_with_targets(
                "When this Class becomes level 2, return target instant or sorcery card from your \
                 graveyard to your hand.",
                TriggerEventDef::BecomesLevel(2),
                &[AbilityTargetDef::exactly_one(
                        AbilityTargetPredicate::Object {
                            object: ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Instant),
                                ObjectPredicateDef::HasType(CardType::Sorcery),
                            ]),
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
            AbilityDef::activated(
                "{5}{U}: Level 3",
                &[AbilityCostDef::Mana(mana_cost!("{5}{U}"))],
                EffectDef::GainClassLevel { level: 3 },
            )
            .with_activation_timing(ActivationTimingDef::SorcerySpeed)
            // Level 2 and no further: a Class at level 1 cannot buy its way straight
            // to three, and one already at three has nothing left to buy.
            .with_activation_condition(&TriggerConditionDef::SourceCounters {
                kind: CounterKind::named("level"),
                comparison: ComparisonDef::Equal,
                amount: 1,
            }),
            AbilityDef::triggered_if(
                "Whenever you cast an instant or sorcery spell, create a 1/1 blue and red Otter creature \
                 token with prowess.",
                TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Instant),
                        ObjectPredicateDef::HasType(CardType::Sorcery),
                    ]),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ])),
                // The level-3 clause only functions once the Class is there. Written as an
                // intervening-if, which is checked when it would trigger and again as it
                // resolves -- so a Class knocked back down between the two does not make
                // the Otter.
                &TriggerConditionDef::SourceCounters {
                    kind: CounterKind::named("level"),
                    comparison: ComparisonDef::GreaterOrEqual,
                    amount: 2,
                },
                MAKE_AN_OTTER,
            ),
        ]),
);

// BLB 78 — Thundertrap Trainer
static TRAINER_ARRIVES: TriggerEventDef = TriggerEventDef::zone_changed(
    ObjectPredicateDef::Source,
    None,
    Some(ZoneKind::Battlefield),
);

pub(in crate::card::sets) static THUNDERTRAP_TRAINER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9cf3af94-b7c8-415c-a5a1-d89967fd0bba"),
    "Thundertrap Trainer",
    CardArt::new("9cf3af94-b7c8-415c-a5a1-d89967fd0bba", "Matt Stewart"),
    CardSet::Bloomburrow,
    // Two mana to dig four cards deep for the spell you want, or six for two
    // bodies and two looks.
    CardRules::new_creature(mana_cost!("{1}{U}"), &["Otter", "Wizard"], 1, 2).with_abilities(&[
        AbilityDef::alternative_cast(
            mana_cost!("{5}{U}"),
            AlternativeCastKindDef::Offspring,
            Some(
                "Offspring {4} (You may pay an additional {4} as you cast this spell. If you do, \
                 when this creature enters, create a 1/1 token copy of it.)",
            ),
            EffectDef::None,
        ),
        AbilityDef::triggered_if(
            "When this creature enters, create a 1/1 token copy of it.",
            TRAINER_ARRIVES,
            // "If you do, when this creature enters": the arrival asks what the cast
            // paid, which the permanent recorded as it arrived.
            &TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Offspring),
            // A 1/1 copy of himself, which arrives with his own look at four attached
            // to it -- the whole reason the extra four mana is worth paying.
            EffectDef::create_token_from_copy(&crate::card::TokenCopyDef {
                object: &EffectRecipientDef::Source,
                exceptions: CopyExceptionsDef::power_toughness(1, 1),
            }),
        ),
        AbilityDef::triggered(
            "When this creature enters, look at the top four cards of your library. You may \
             reveal a noncreature, nonland card from among them and put it into your hand. Put \
             the rest on the bottom of your library in a random order.",
            TRAINER_ARRIVES,
            abilities::look_at_top_cards_reveal_choice_to_hand_rest_random_bottom(
                ValueDef::Constant(4),
                // A noncreature, nonland card among the four, which is what the deck
                // playing him is digging for.
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

// BLB 94 — Feed the Cycle
pub(in crate::card::sets) static FEED_THE_CYCLE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7e017ff8-2936-4a1b-bece-00004cfbad06"),
    "Feed the Cycle",
    CardArt::new("7e017ff8-2936-4a1b-bece-00004cfbad06", "Donato Giancola"),
    CardSet::Bloomburrow,
    CardRules::new_instant(mana_cost!("{1}{B}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, forage or pay {B}. (To forage, exile \
             three cards from your graveyard or sacrifice a Food.)\nDestroy target creature or \
             planeswalker.",
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Creature),
                    ObjectPredicateDef::HasType(CardType::Planeswalker),
                ]),
            )],
            SpellAdditionalCostDef::choice(&[
                SpellAdditionalCostDef::forage(),
                SpellAdditionalCostDef::pay_mana(mana_cost!("{B}")),
            ]),
            EffectDef::destroy_target(TargetIndex::PRIMARY, true),
        ),
    ),
);

// BLB 208 — Cindering Cutthroat
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static CINDERING_CUTTHROAT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b2ea10dd-21ea-4622-be27-79d03a802b85"),
    "Cindering Cutthroat",
    crate::card::CardArt::new("b2ea10dd-21ea-4622-be27-79d03a802b85", "Wayne Reynolds"),
    crate::card::CardSet::Bloomburrow,
    crate::card::CardRules::unsupported(),
);

// BLB 235 — Tempest Angler
pub(in crate::card::sets) static TEMPEST_ANGLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("850daae4-f0b7-4604-95e7-ad044ec165c3"),
    "Tempest Angler",
    CardArt::new("850daae4-f0b7-4604-95e7-ad044ec165c3", "Raluca Marinescu"),
    CardSet::Bloomburrow,
    // Counters rather than prowess: what it grows it keeps, so a slow turn
    // of cheap spells leaves a threat rather than a one-turn swing.
    CardRules::new_creature(mana_cost!("{1}{U/R}{U/R}"), &["Otter", "Wizard"], 2, 2).with_ability(
        AbilityDef::triggered(
            "Whenever you cast a noncreature spell, put a +1/+1 counter on this creature.",
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::NoncreatureSpell,
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// BLB 254 — Hidden Grotto
pub(in crate::card::sets) static HIDDEN_GROTTO: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4ba8f2e7-8357-4862-97dc-1942d066023a"),
    "Hidden Grotto",
    CardArt::new("4ba8f2e7-8357-4862-97dc-1942d066023a", "Fiona Hsieh"),
    CardSet::Bloomburrow,
    // Untapped and colourless by default, so the fixing costs a mana rather
    // than a turn -- and the surveil pays for playing it over a basic.
    CardRules::new_land(&[]).with_abilities(&[
        abilities::enters_trigger(
            "When this land enters, surveil 1. (Look at the top card of your library. You may \
             put it into your graveyard.)",
            abilities::surveil(ValueDef::Constant(1)),
        ),
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_mana(
            "{1}, {T}: Add one mana of any color.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::TapSource,
            ],
            EffectDef::AddMana(AddManaEffectDef::any_color()),
        ),
    ]),
);

// BLB 307 — Thundertrap Trainer (alternate printing)

// BLB 322 — Keen-Eyed Curator
pub(in crate::card::sets) static KEEN_EYED_CURATOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("004a67ce-60ef-4cc2-9f4d-f30e3029d80a"),
    "Keen-Eyed Curator",
    CardArt::new("004a67ce-60ef-4cc2-9f4d-f30e3029d80a", "Mariah Tekulve"),
    CardSet::Bloomburrow,
    // Two mana for a 3/3 that answers a graveyard a card at a time, and
    // turns into a 7/7 trampler for having done it four kinds of times.
    CardRules::new_creature(mana_cost!("{G}{G}"), &["Raccoon", "Scout"], 3, 3)
        .with_abilities(&[
            // "As long as", so the 7/7 comes and goes with the pile rather than
            // being settled once.
            AbilityDef::static_ability(
                "As long as there are four or more card types among cards exiled with this creature, it \
                 gets +4/+4 and has trample.",
                EffectDef::IfCondition {
                    // Four card types among the cards he took, counted over the pile rather
                    // than over any zone: he keeps them, so a card that leaves exile stops
                    // counting and the rest still do.
                    condition: &TriggerConditionDef::ValueComparison(&ValueComparisonDef {
                            left: ValueDef::CardTypesAmongObjects(&ObjectSetDef::LinkedExiles),
                            comparison: ComparisonDef::GreaterOrEqual,
                            right: ValueDef::Constant(4),
                        }),
                    then: &EffectDef::Sequence(&[
                        EffectDef::StaticApply {
                            recipient: EffectRecipientDef::Source,
                            effect: AppliedEffectDef::modify_power_toughness(
                                ValueDef::Constant(4),
                                ValueDef::Constant(4),
                            ),
                        },
                        EffectDef::StaticApply {
                            recipient: EffectRecipientDef::Source,
                            effect: AppliedEffectDef::add_ability(&abilities::trample()),
                        },
                    ]),
                },
            ),
            // Either graveyard: what he is played for is emptying theirs, and the
            // card types he needs come from wherever they are.
            AbilityDef::activated_with_targets(
                "{1}: Exile target card from a graveyard.",
                &[AbilityCostDef::Mana(mana_cost!("{1}"))],
                &[AbilityTargetDef::exactly_one(
                    AbilityTargetPredicate::Object {
                        object: ObjectPredicateDef::Any,
                        zones: &[ZoneKind::Graveyard],
                        controller: None,
                        owner: None,
                    },
                )],
                EffectDef::ExileLinkedToSource {
                    until_source_leaves: false,
                    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                    face_down: false,
                    then: None,
                },
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &KITSA_OTTERBALL_ELITE,
    &STORMCHASERS_TALENT,
    &THUNDERTRAP_TRAINER,
    &FEED_THE_CYCLE,
    &CINDERING_CUTTHROAT,
    &TEMPEST_ANGLER,
    &HIDDEN_GROTTO,
    &KEEN_EYED_CURATOR,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[
    PrintingRecord::alternate(&THUNDERTRAP_TRAINER, 1), // BLB 307
];
