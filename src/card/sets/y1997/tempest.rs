//! Tempest cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityPredicateDef, AbilityTargetDef,
    AbilityTargetPredicate, AddManaEffectDef, AppliedEffectDef, BattlefieldEntryModificationDef,
    CardArt, CardBehavior, CardRules, CardSet, CardSupertype, CardType, ChoiceVisibilityDef,
    ChooseDef, EffectDef, EffectExecutionDef, EffectRecipientDef, ManaColor,
    ObjectChoiceBindingDef, ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef,
    PlayerRefDef, PlayerRelation, PlayerSetDef, ReplacementEffectDef, ReplacementEventDef,
    TriggerConditionDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, ZonePlacement,
    abilities,
};
use crate::ids::{ObjectBindingIndex, ObjectSetBindingIndex};
use crate::{TargetIndex, mana_cost};

/// Everything at once, in one static effect: the abilities go in layer 6 and
/// the stats are set in layer 7b, and a creature that arrives later is caught
/// by the same continuous effect rather than needing its own.
static HUMBLED: [AppliedEffectDef; 2] = [
    AppliedEffectDef::remove_abilities(AbilityPredicateDef::Any),
    AppliedEffectDef::set_base_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)),
];

// TMP 24 — Humility
pub(in crate::card::sets) static HUMILITY: CardRecord = CardRecord::new_with_legacy_id(
    2055,
    "Humility",
    CardArt::new("a2fb7128-806b-4148-80fe-eb967f248021", "Phil Foglio"),
    CardSet::Tempest,
    // Symmetric and total: the control deck playing it has no creatures to
    // lose, which is the whole argument for the card.
    CardRules::new_enchantment(mana_cost!("{2}{W}{W}")).with_ability(AbilityDef::static_ability(
        "All creatures lose all abilities and have base power and toughness 1/1.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::matching_objects(
                ObjectPredicateDef::HasType(CardType::Creature),
                &[ZoneKind::Battlefield],
                PlayerRelation::Any,
            ),
            effect: AppliedEffectDef::Composite(&HUMBLED),
        },
    )),
);

// TMP 51 — Warmth
pub(in crate::card::sets) static WARMTH: CardRecord = CardRecord::new_with_legacy_id(
    286,
    "Warmth",
    CardArt::new("d7dbeea8-06b0-4482-bdae-aa82b9db8856", "Drew Tucker"),
    CardSet::Tempest,
    CardRules::new_enchantment(mana_cost!("{1}{W}")).with_ability(AbilityDef::triggered(
        "Whenever an opponent casts a red spell, you gain 2 life.",
        TriggerEventDef::SpellCast(ObjectPredicateDef::All(&[
            ObjectPredicateDef::Color(ManaColor::Red),
            ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
        ])),
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(2),
        },
    )),
);

// TMP 56 — Chill
pub(in crate::card::sets) static CHILL: CardRecord = CardRecord::new_with_legacy_id(
    2038,
    "Chill",
    CardArt::new("5a7bd777-6f11-441e-887f-9cee1ef96035", "Greg Simanson"),
    CardSet::Tempest,
    // Two extra mana on every burn spell, which is most of what a red deck
    // has to say.
    CardRules::new_enchantment(mana_cost!("{1}{U}")).with_ability(AbilityDef::static_ability(
        "Red spells cost {2} more to cast.",
        EffectDef::IncreaseMatchingSpellCostBy {
            spell: ObjectPredicateDef::Color(ManaColor::Red),
            caster: PlayerRelation::Any,
            amount: mana_cost!("{2}"),
        },
    )),
);

/// The one the opponent hands over, and the two they keep back. Both halves
/// are one partition of the three that were found, which is why the choice
/// names the rest as well as the pick.
static INTUITION_DISTRIBUTE: EffectDef = EffectDef::Sequence(&[
    EffectDef::MoveToZone {
        object: EffectRecipientDef::object(ObjectRefDef::Binding(ObjectBindingIndex::PRIMARY)),
        zone: ZoneKind::Hand,
        placement: ZonePlacement::Top,
        arrival_effect: None,
        attachment: None,
        controller: None,
    },
    EffectDef::MoveToZone {
        object: EffectRecipientDef::objects(ObjectSetDef::Binding(ObjectSetBindingIndex::PRIMARY)),
        zone: ZoneKind::Graveyard,
        placement: ZonePlacement::Top,
        arrival_effect: None,
        attachment: None,
        controller: None,
    },
]);

/// The opponent picks which of the three is worth giving up, out of the
/// cards the search found rather than out of the library it found them in.
static INTUITION_CHOICE: EffectDef = EffectDef::Choose(ChooseDef {
    binding: ObjectChoiceBindingDef::Object(ObjectBindingIndex::PRIMARY),
    unchosen: Some(ObjectSetBindingIndex::PRIMARY),
    chooser: PlayerRefDef::Target(TargetIndex::PRIMARY),
    candidates: ObjectSetDef::Binding(INTUITION_FOUND),
    exclude: None,
    minimum: 1,
    maximum: 1,
    visibility: ChoiceVisibilityDef::Public,
    then: &INTUITION_DISTRIBUTE,
});

/// The three the search turned up, kept apart from the partition bindings so
/// that "the rest" is measured against them rather than against itself.
static INTUITION_FOUND: ObjectSetBindingIndex = ObjectSetBindingIndex::new(1);

static INTUITION_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
)];

// TMP 70 — Intuition
pub(in crate::card::sets) static INTUITION: CardRecord = CardRecord::new_with_legacy_id(
    2084,
    "Intuition",
    CardArt::new("19eae4ac-10a4-4860-bcc2-0c9816f8bcdd", "April Lee"),
    CardSet::Tempest,
    // Naming three copies of one card makes the opponent's choice no choice
    // at all; naming three different ones is how a graveyard deck fills its
    // graveyard and keeps the piece it needs.
    CardRules::new_instant(mana_cost!("{2}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Search your library for three cards and reveal them. Target opponent chooses one. Put that card into your hand and the rest into your graveyard. Then shuffle.",
        &INTUITION_TARGET,
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::Any,
            minimum: 3,
            maximum: ValueDef::Constant(3),
            reveal: true,
            destination: ZoneKind::Library,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: false,
            binding: Some(INTUITION_FOUND),
            then: Some(&INTUITION_CHOICE),
        },
    )),
);

static TIME_WARP_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Any),
)];

// TMP 97 — Time Warp
pub(in crate::card::sets) static TIME_WARP: CardRecord = CardRecord::new_with_legacy_id(
    2109,
    "Time Warp",
    CardArt::new("3447aeaf-3b26-442a-99d4-0a7ee76c8e76", "Pete Venters"),
    CardSet::Tempest,
    CardRules::new_sorcery(mana_cost!("{3}{U}{U}")).with_ability(AbilityDef::spell_with_targets(
        "Target player takes an extra turn after this one.",
        &TIME_WARP_TARGET,
        EffectDef::TakeExtraTurn {
            player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        },
    )),
);

static DANCE_HASTE: AbilityDef = abilities::haste();

/// The creature exiles itself rather than being named by a delayed trigger:
/// it is the object that arrived, and it carries the clause with it.
static DANCE_EXILE_AT_END: AbilityDef = AbilityDef::triggered(
    "At the beginning of the next end step, exile this creature.",
    TriggerEventDef::StepBegins {
        step: TurnStepDef::End,
        player: PlayerRelation::Any,
    },
    EffectDef::MoveToZone {
        object: EffectRecipientDef::Source,
        zone: ZoneKind::Exile,
        placement: ZonePlacement::Top,
        arrival_effect: None,
        attachment: None,
        controller: None,
    },
);

static DANCE_ARRIVAL: AppliedEffectDef = AppliedEffectDef::Composite(&[
    AppliedEffectDef::add_ability(&DANCE_HASTE),
    AppliedEffectDef::add_ability(&DANCE_EXILE_AT_END),
]);

// TMP 116 — Corpse Dance
pub(in crate::card::sets) static CORPSE_DANCE: CardRecord = CardRecord::new_with_legacy_id(
    2187,
    "Corpse Dance",
    CardArt::new("76ae81ea-13e3-4ab8-b956-4c7b139a5e9c", "Brian Snõddy"),
    CardSet::Tempest,
    // Shallow Grave that comes back, which is why five mana a turn is a
    // price worth paying: whatever is on top of the graveyard attacks every
    // turn from here, and the card is never spent.
    CardRules::new_instant(mana_cost!("{2}{B}")).with_abilities(&[
        abilities::buyback(mana_cost!("{2}")),
        AbilityDef::spell(
            "Return the top creature card of your graveyard to the battlefield. That creature gains haste until end of turn. Exile it at the beginning of the next end step.",
            EffectDef::MoveToZone {
                object: EffectRecipientDef::objects(ObjectSetDef::TopOfGraveyardMatching {
                    player: PlayerRefDef::EffectController,
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                }),
                zone: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                arrival_effect: Some(&DANCE_ARRIVAL),
                attachment: None,
                controller: None,
            },
        ),
    ]),
);

// TMP 151 — Reanimate
pub(in crate::card::sets) static REANIMATE: CardRecord = CardRecord::new_with_legacy_id(
    305,
    "Reanimate",
    CardArt::new("fc00f897-988b-4602-969a-c510804ec12a", "Robert Bliss"),
    CardSet::Tempest,
    CardRules::new_sorcery(mana_cost!("{B}")).with_ability(AbilityDef::spell_with_targets(
        "Put target creature card from a graveyard onto the battlefield under your control. You lose life equal to that card's mana value.",
        &[AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
            object: ObjectPredicateDef::HasType(CardType::Creature),
            zones: &[ZoneKind::Graveyard],
            controller: None,
            owner: None,
        })],
        EffectDef::Sequence(&[
            EffectDef::MoveToZone {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                zone: ZoneKind::Battlefield,
                placement: ZonePlacement::Top,
                arrival_effect: None,
                attachment: None,
                controller: Some(PlayerRelation::You),
            },
            EffectDef::LoseLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::TargetManaValue(TargetIndex::PRIMARY),
            },
        ]),
    )),
);

static GOBLIN_BOMBARDMENT_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::AnyTarget,
)];

// TMP 179 — Goblin Bombardment
pub(in crate::card::sets) static GOBLIN_BOMBARDMENT: CardRecord = CardRecord::new_with_legacy_id(
    2110,
    "Goblin Bombardment",
    CardArt::new("179e954f-1d90-4ef4-b800-25845cc338e2", "Brian Snoddy"),
    CardSet::Tempest,
    CardRules::new_enchantment(mana_cost!("{1}{R}")).with_ability(
        AbilityDef::activated_with_targets(
            "Sacrifice a creature: This enchantment deals 1 damage to any target.",
            &[AbilityCostDef::SacrificePermanent {
                object: ObjectPredicateDef::HasType(CardType::Creature),
                controller: PlayerRelation::You,
            }],
            &GOBLIN_BOMBARDMENT_TARGET,
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// TMP 183 — Jackal Pup
pub(in crate::card::sets) static JACKAL_PUP: CardRecord = CardRecord::new_with_legacy_id(
    267,
    "Jackal Pup",
    CardArt::new("3707ab74-9aec-4d30-86e0-ffa5f72d5b4f", "Susan Van Camp"),
    CardSet::Tempest,
    CardRules::new_creature(mana_cost!("{R}"), &["Jackal"], 2, 1).with_ability(
        AbilityDef::triggered(
            "Whenever this creature is dealt damage, it deals that much damage to you.",
            TriggerEventDef::damage_to_source(),
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::TriggerEventAmount,
            },
        ),
    ),
);

// TMP 190 — Mogg Fanatic
pub(in crate::card::sets) static MOGG_FANATIC: CardRecord = CardRecord::new_with_legacy_id(
    268,
    "Mogg Fanatic",
    CardArt::new("ca2ecfd4-c874-4468-8601-87aa110d5a00", "Brom"),
    CardSet::Tempest,
    CardRules::new_creature(mana_cost!("{R}"), &["Goblin"], 1, 1).with_ability(
        AbilityDef::activated_with_targets(
            "Sacrifice this creature: It deals 1 damage to any target.",
            &[AbilityCostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::AnyTarget,
            )],
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
            },
        ),
    ),
);

// TMP 250 — Root Maze
pub(in crate::card::sets) static ROOT_MAZE: CardRecord = CardRecord::new_with_legacy_id(
    287,
    "Root Maze",
    CardArt::new("99a12b74-f191-4362-81ab-77590ae5e68f", "Rebecca Guay"),
    CardSet::Tempest,
    CardRules::new_enchantment(mana_cost!("{G}")).with_ability(AbilityDef::replacement_for(
        "Artifacts and lands enter tapped.",
        ReplacementEventDef::ObjectEntersBattlefield {
            object: ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Land),
            ]),
            controller: PlayerRelation::Any,
            cast: None,
        },
        ReplacementEffectDef::ModifyBattlefieldEntry(BattlefieldEntryModificationDef::Tapped),
    )),
);

/// Naming a card is modelled as picking one of the cards in hand. Every name
/// worth choosing is one of those -- naming something you do not hold can
/// only fail -- and the choice is public either way, so nothing is hidden and
/// nothing achievable is lost.
static NAMED_CARD: ObjectBindingIndex = ObjectBindingIndex::PRIMARY;
static REVEALED_CARD: ObjectBindingIndex = ObjectBindingIndex::new(1);

static SCROLL_NAMES_MATCH: TriggerConditionDef = TriggerConditionDef::BoundObjectsShareName {
    first: NAMED_CARD,
    second: REVEALED_CARD,
};

static SCROLL_SHOT: EffectDef = EffectDef::DealDamage {
    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    amount: ValueDef::Constant(2),
};

static SCROLL_IF_MATCHED: EffectDef = EffectDef::IfCondition {
    condition: &SCROLL_NAMES_MATCH,
    then: &SCROLL_SHOT,
};

static SCROLL_REVEAL: EffectDef = EffectDef::RevealAtRandomFromHand {
    player: EffectRecipientDef::Controller,
    binding: REVEALED_CARD,
    then: &SCROLL_IF_MATCHED,
};

static CARDS_IN_YOUR_HAND: ObjectQueryDef = ObjectQueryDef::owned_by(
    ObjectPredicateDef::Any,
    &[ZoneKind::Hand],
    PlayerSetDef::Related(PlayerRelation::You),
);

// TMP 281 — Cursed Scroll
pub(in crate::card::sets) static CURSED_SCROLL: CardRecord = CardRecord::new_with_legacy_id(
    2037,
    "Cursed Scroll",
    CardArt::new(
        "31415b9b-fb30-4132-a9a3-795b4573a901",
        "D. Alexander Gregory",
    ),
    CardSet::Tempest,
    // An empty hand makes it a certainty, which is why the card belongs in a
    // deck that has already spent everything.
    CardRules::new_artifact(mana_cost!("{1}")).with_ability(AbilityDef::activated_with_targets(
        "{3}, {T}: Choose a card name, then reveal a card at random from your hand. If that card has the chosen name, this artifact deals 2 damage to any target.",
        &[
            AbilityCostDef::Mana(mana_cost!("{3}")),
            AbilityCostDef::TapSource,
        ],
        &SCROLL_TARGET,
        EffectDef::Choose(ChooseDef {
            binding: ObjectChoiceBindingDef::Object(NAMED_CARD),
            unchosen: None,
            chooser: PlayerRefDef::EffectController,
            candidates: ObjectSetDef::Query(CARDS_IN_YOUR_HAND),
            exclude: None,
            minimum: 1,
            maximum: 1,
            visibility: ChoiceVisibilityDef::Public,
            then: &SCROLL_REVEAL,
        }),
    )),
);

static SCROLL_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::AnyTarget,
)];

// TMP 294 — Lotus Petal
pub(in crate::card::sets) static LOTUS_PETAL: CardRecord = CardRecord::new_with_legacy_id(
    271,
    "Lotus Petal",
    CardArt::new("6c877da3-68fa-41d0-8a24-8c79fcd8ecc1", "April Lee"),
    CardSet::Tempest,
    CardRules::new_artifact(mana_cost!("{0}")).with_ability(AbilityDef::activated_mana(
        "{T}, Sacrifice this artifact: Add one mana of any color.",
        &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
        EffectDef::AddMana(AddManaEffectDef::any_color()),
    )),
);

// TMP 315 — Ancient Tomb
pub(in crate::card::sets) static ANCIENT_TOMB: CardRecord = CardRecord::new_with_legacy_id(
    300,
    "Ancient Tomb",
    CardArt::new("30e401e3-282b-4524-87e1-c6cd50cd6d00", "Colin MacNeil"),
    CardSet::Tempest,
    CardRules::new_land(&[]).with_ability(AbilityDef::activated_mana(
        "{T}: Add {C}{C}. This land deals 2 damage to you.",
        &[AbilityCostDef::TapSource],
        EffectDef::AddMana(
            AddManaEffectDef::one(ManaColor::Colorless)
                .with_amount(2)
                .with_damage_to_controller(2),
        ),
    )),
);

// TMP 322 — Reflecting Pool
pub(in crate::card::sets) static REFLECTING_POOL: CardRecord = CardRecord::new_with_legacy_id(
    2073,
    "Reflecting Pool",
    CardArt::new("4fc67298-6610-47d7-971b-baf5728d5349", "Adam Rex"),
    CardSet::Tempest,
    // Worth nothing on its own and everything beside four other lands, which
    // is why a five-color deck plays it and nobody else does.
    CardRules::new_land(&[]).with_ability(
        AbilityDef::activated_mana(
            "{T}: Add one mana of any type that a land you control could produce.",
            &[AbilityCostDef::TapSource],
            EffectDef::Special("Add one mana of a type a land you control could produce"),
        )
        .with_effect_execution(EffectExecutionDef::Custom(CardBehavior::ReflectingPool))
        .with_coverage(AbilityCoverageDef::explained_complete(
            "The available types are computed dynamically from the lands you control.",
        ))
        .with_legacy_procedure(),
    ),
);

// TMP 330 — Wasteland
pub(in crate::card::sets) static WASTELAND: CardRecord = CardRecord::new_with_legacy_id(
    279,
    "Wasteland",
    CardArt::new("99ff731b-8399-40c8-b539-ba6ba5783771", "Una Fricker"),
    CardSet::Tempest,
    CardRules::new_land(&[]).with_abilities(&[
        abilities::tap_for(ManaColor::Colorless),
        AbilityDef::activated_with_targets(
            "{T}, Sacrifice this land: Destroy target nonbasic land.",
            &[AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Supertype(CardSupertype::Basic)),
                ]),
            )],
            EffectDef::destroy_target(TargetIndex::PRIMARY, true),
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &HUMILITY,
    &WARMTH,
    &CHILL,
    &INTUITION,
    &TIME_WARP,
    &CORPSE_DANCE,
    &REANIMATE,
    &GOBLIN_BOMBARDMENT,
    &JACKAL_PUP,
    &MOGG_FANATIC,
    &ROOT_MAZE,
    &CURSED_SCROLL,
    &LOTUS_PETAL,
    &ANCIENT_TOMB,
    &REFLECTING_POOL,
    &WASTELAND,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
