//! Core Set 2020 cards cataloged for the Vintage Cube.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AddManaEffectDef, AppliedEffectDef,
    AppliedRuleDef, CardArt, CardRules, CardSet, CardType, ComparisonDef, CounterKind, EffectDef,
    EffectRecipientDef, ManaColor, ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef,
    PlayerRelation, PlayerSetDef, ResolvedEffectDurationDef, StackTargetAggregationDef,
    StackTargetFilterDef, TriggerConditionDef, TriggerEventDef, ValueComparisonDef, ValueDef,
    ZoneKind, ZonePlacement, abilities,
};
use crate::{TargetIndex, mana_cost};

// M20 3 — Ancestral Blade
pub(in crate::card::sets) static ANCESTRAL_BLADE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2ba18114-af6c-48cd-82c9-eb6541d566bf"),
    "Ancestral Blade",
    CardArt::new("2ba18114-af6c-48cd-82c9-eb6541d566bf", "Scott Murphy"),
    CardSet::Magic2020,
    // Two mana buys a 2/2 that leaves an Equipment behind, which is what
    // makes it playable in a deck with no other artifacts to care about.
    CardRules::new_artifact(mana_cost!("{1}{W}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::enters_trigger(
                "When this Equipment enters, create a 1/1 white Soldier creature token, then \
                 attach this Equipment to it.",
                EffectDef::Sequence(&[
                    EffectDef::create_creature_token(&["Soldier"], &[ManaColor::White], 1, 1),
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
                &[AbilityCostDef::Mana(mana_cost!("{1}"))],
                "Equip {1} ({1}: Attach to target creature you control. Equip only as a sorcery.)",
            ),
        ]),
);

// M20 34 — Raise the Alarm
pub(in crate::card::sets) static RAISE_THE_ALARM: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4be510c8-fc01-4374-ac04-7968d24480fe"),
    "Raise the Alarm",
    CardArt::new("764a7a53-314e-4b1f-aa33-0f312d06df71", "Zoltan Boros"),
    CardSet::Magic2020,
    // Two bodies at instant speed, which is what a token deck pays the extra
    // mana for: it holds up the trick and still develops the board.
    CardRules::new_instant(mana_cost!("{1}{W}")).with_ability(AbilityDef::spell(
        "Create two 1/1 white Soldier creature tokens.",
        EffectDef::create_creature_token(&["Soldier"], &[ManaColor::White], 1, 1).with_amount(2),
    )),
);

// M20 54 — Cloudkin Seer
pub(in crate::card::sets) static CLOUDKIN_SEER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e2111753-a930-403f-9d94-a86dfcb069da"),
    "Cloudkin Seer",
    CardArt::new(
        "e2111753-a930-403f-9d94-a86dfcb069da",
        "Anastasia Ovchinnikova",
    ),
    CardSet::Magic2020,
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

// M20 148 — Leyline of Combustion
pub(in crate::card::sets) static LEYLINE_OF_COMBUSTION: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3a93c8e2-fb27-43af-83a7-2bd4d40e0eff"),
    "Leyline of Combustion",
    CardArt::new("3a93c8e2-fb27-43af-83a7-2bd4d40e0eff", "Noah Bradley"),
    CardSet::Magic2020,
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
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::EventPlayer,
                amount: ValueDef::Constant(2),
            },
        ),
    ]),
);

// M20 169 — Elvish Reclaimer
pub(in crate::card::sets) static ELVISH_RECLAIMER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("39c431d7-d94b-46c4-bb89-f3db56214ab4"),
    "Elvish Reclaimer",
    CardArt::new(
        "39c431d7-d94b-46c4-bb89-f3db56214ab4",
        "Victor Adame Minguez",
    ),
    CardSet::Magic2020,
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
                AbilityCostDef::Mana(mana_cost!("{2}")),
                AbilityCostDef::TapSource,
                AbilityCostDef::SacrificePermanent {
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
    PrintingAnchor::scryfall("c68e8342-78d2-4826-a287-64c371b97d19"),
    "Leyline of Abundance",
    CardArt::new("c68e8342-78d2-4826-a287-64c371b97d19", "Noah Bradley"),
    CardSet::Magic2020,
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
            &[AbilityCostDef::Mana(mana_cost!("{6}{G}{G}"))],
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

// M20 230 — Manifold Key
pub(in crate::card::sets) static MANIFOLD_KEY: CardRecord = CardRecord::new_with_legacy_id(
    2207,
    "Manifold Key",
    CardArt::new("715e637a-dfd8-45a0-b1ea-53e4abd29307", "Lake Hurwitz"),
    CardSet::Magic2020,
    // One mana that untaps a Mox for profit and, when there is nothing to
    // untap, pushes a creature through instead.
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        AbilityDef::activated_with_targets(
            "{1}, {T}: Untap another target artifact.",
            &[
                AbilityCostDef::Mana(mana_cost!("{1}")),
                AbilityCostDef::TapSource,
            ],
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
            &[
                AbilityCostDef::Mana(mana_cost!("{3}")),
                AbilityCostDef::TapSource,
            ],
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

// M20 247 — Field of the Dead
pub(in crate::card::sets) static FIELD_OF_THE_DEAD: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("470ca3f4-29aa-4c4c-8ff2-8cdd70c69943"),
    "Field of the Dead",
    CardArt::new("470ca3f4-29aa-4c4c-8ff2-8cdd70c69943", "Kev Walker"),
    CardSet::Magic2020,
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
            EffectDef::create_creature_token(&["Zombie"], &[ManaColor::Black], 2, 2).with_art(
                CardArt::new("18f0436e-9328-4266-9cf8-80b557a0c17c", "Anna Steinbauer"),
            ),
        ),
    ]),
);

// M20 297 — Wildfire Elemental
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WILDFIRE_ELEMENTAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("272e317c-55c4-43b2-91aa-3e0009cfd7d5"),
    "Wildfire Elemental",
    crate::card::CardArt::new("272e317c-55c4-43b2-91aa-3e0009cfd7d5", "Svetlin Velinov"),
    crate::card::CardSet::Magic2020,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ANCESTRAL_BLADE,
    &RAISE_THE_ALARM,
    &CLOUDKIN_SEER,
    &LEYLINE_OF_COMBUSTION,
    &ELVISH_RECLAIMER,
    &LEYLINE_OF_ABUNDANCE,
    &MANIFOLD_KEY,
    &FIELD_OF_THE_DEAD,
    &WILDFIRE_ELEMENTAL,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
