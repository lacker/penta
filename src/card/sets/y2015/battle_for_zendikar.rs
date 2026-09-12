//! BFZ card records required by supported formats.

use crate::ParentBinding;
use crate::card::BattlefieldEntryModificationDef;
use crate::card::BindObjectsDef;
use crate::card::CardSupertype;
use crate::card::ComparisonDef;
use crate::card::ConditionDef;
use crate::card::FreePlayDef;
use crate::card::FreePlayDurationDef;
use crate::card::ObjectCollectionSourceDef;
use crate::card::ObjectCountConditionDef;
use crate::card::ObjectQueryDef;
use crate::card::ObjectSetDef;
use crate::card::PayOrDef;
use crate::card::PlayerRefDef;
use crate::card::ReplacementEffectDef;
use crate::card::TriggerEventDef;
use crate::card::ZonePlacement;
use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::AppliedRuleDef;
use crate::card::CardArt;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::CostDef;
use crate::card::CreateTokenDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectRefDef;
use crate::card::PlayerRelation;
use crate::card::ResolvedEffectDurationDef;
use crate::card::TokenCharacteristics;
use crate::card::TokenDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::abilities;
use crate::ids::TargetIndex;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "BFZ",
    slug: "battle-for-zendikar",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

const ELDRAZI_SCION_TOKEN: TokenCharacteristics =
    TokenCharacteristics::creature(&["Eldrazi", "Scion"], &[], 1, 1)
        .with_abilities(&[AbilityDef::activated_mana(
            "Sacrifice this creature: Add {C}.",
            &[CostDef::SacrificeSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless)),
        )])
        .with_art(CardArt::new("b999a0fe-d2d0-4367-9abb-6ce5f3764f19", "Izzy"));

// BFZ 15 — Ulamog, the Ceaseless Hunger
pub(in crate::card::sets) static ULAMOG_THE_CEASELESS_HUNGER_15: CardRecord = CardRecord::new(
    "Ulamog, the Ceaseless Hunger",
    "1192f7a9-102e-4b3a-b154-18c8eb332217",
    "Michael Komarck",
    CardRules::new_creature(mana_cost!("{10}"), &["Eldrazi"], 10, 10).with_supertype(CardSupertype::Legendary).with_abilities(&[
AbilityDef::triggered_with_targets("When you cast this spell, exile two target permanents.", TriggerEventDef::spell_cast(ObjectPredicateDef::Source), &[AbilityTargetDef { minimum: 2, maximum: 2, ..AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::Any) }], EffectDef::move_to_zone(EffectRecipientDef::Target(TargetIndex::PRIMARY), ZoneKind::Exile, ZonePlacement::Top)),
abilities::indestructible(),
AbilityDef::triggered("Whenever Ulamog attacks, defending player exiles the top twenty cards of their library.", TriggerEventDef::attacks(ObjectPredicateDef::Source), EffectDef::BindObjects(BindObjectsDef { source: ObjectCollectionSourceDef::TopCards { player: PlayerRefDef::Opponent, count: ValueDef::Constant(20) }, binding: ParentBinding, then: &EffectDef::move_to_zone(EffectRecipientDef::objects(ObjectSetDef::Binding(ParentBinding)), ZoneKind::Exile, ZonePlacement::Top) }))
]),
);

// BFZ 17 — Void Winnower
// Audit: unsupported — Spell and blocker predicates have mana-value comparisons but no parity predicate for all even mana values, including arbitrary chosen X values.
pub(in crate::card::sets) static VOID_WINNOWER_17: CardRecord = CardRecord::new(
    "Void Winnower",
    "8cbedb0a-34ca-4d42-bb43-cbea0f3c6d02",
    "Chase Stone",
    crate::card::CardRules::unsupported(),
);

// BFZ 25 — Felidar Cub
pub(in crate::card::sets) static FELIDAR_CUB: CardRecord = CardRecord::new(
    "Felidar Cub",
    "ea76a183-e15c-4968-b29d-91c074aa8681",
    "Steve Prescott",
    CardRules::new_creature(mana_cost!("{1}{W}"), &["Cat", "Beast"], 2, 2).with_abilities(&[
        AbilityDef::activated_with_targets(
            "Sacrifice this creature: Destroy target enchantment.",
            &[CostDef::SacrificeSource],
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Enchantment),
            )],
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                then: None,
            },
        ),
    ]),
);

// BFZ 50 — Stasis Snare
// Audit: unsupported — Needs an exile-until-source-leaves duration with immediate return when that duration ends (CR 610.3); an ordinary leaves trigger returns the card later through the stack.
pub(in crate::card::sets) static STASIS_SNARE: CardRecord = CardRecord::new(
    "Stasis Snare",
    "ff820544-f4a3-40c4-a48e-84b5e2d06caa",
    "Jason Felix",
    CardRules::unsupported(),
);

// BFZ 58 — Eldrazi Skyspawner
pub(in crate::card::sets) static ELDRAZI_SKYSPAWNER: CardRecord = CardRecord::new(
    "Eldrazi Skyspawner",
    "9c9c1a10-446e-492a-95cc-a459dc6c08a0",
    "Chase Stone",
// Three mana for two bodies and a ritual: the Scion is what turns the
    // flier into a fourth-turn six-drop.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Eldrazi", "Drone"], 2, 1).with_abilities(&[
        abilities::devoid(),
        abilities::flying(),
        abilities::enters_trigger(
            "When this creature enters, create a 1/1 colorless Eldrazi Scion creature token. It has \"Sacrifice this token: Add {C}.\"",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(ELDRAZI_SCION_TOKEN))),
        ),
    ]),
);

// BFZ 106 — Carrier Thrall
pub(in crate::card::sets) static CARRIER_THRALL: CardRecord = CardRecord::new(
    "Carrier Thrall",
    "bd2ab895-9225-4eba-90c3-4023db4f8b70",
    "Lius Lasahido",
// Two mana that trades and still leaves a mana behind, which is why the
    // body is aggressive and the death trigger is not.
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Vampire"], 2, 1).with_abilities(&[
        abilities::devoid(),
        abilities::dies_trigger(
            "When this creature dies, create a 1/1 colorless Eldrazi Scion creature token. It has \"Sacrifice this token: Add {C}.\"",
            EffectDef::CreateToken(CreateTokenDef::new(TokenDef::Literal(ELDRAZI_SCION_TOKEN))),
        ),
    ]),
);

// BFZ 124 — Vampiric Rites
pub(in crate::card::sets) static VAMPIRIC_RITES: CardRecord = CardRecord::new(
    "Vampiric Rites",
    "3416309a-5824-48f5-876e-00e0f180acf9",
    "Anastasia Ovchinnikova",
    CardRules::new_enchantment(mana_cost!("{B}")).with_abilities(&[AbilityDef::activated(
        "{1}{B}, Sacrifice a creature: You gain 1 life and draw a card.",
        &[
            CostDef::Mana(mana_cost!("{1}{B}")),
            CostDef::sacrifice_permanent(ObjectPredicateDef::HasType(CardType::Creature)),
        ],
        EffectDef::Sequence(&[
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
            abilities::draw_cards(ValueDef::Constant(1)),
        ]),
    )]),
);

// BFZ 157 — Sure Strike
pub(in crate::card::sets) static SURE_STRIKE: CardRecord = CardRecord::new(
    "Sure Strike",
    "074dd176-4608-42ca-8fc3-c7040cd7b32e",
    "Jakub Kasper",
    CardRules::new_instant(mana_cost!("{1}{R}")).with_abilities(&[AbilityDef::spell_with_targets(
        "Target creature gets +3/+0 and gains first strike until end \
         of turn. (It deals combat damage before creatures without \
         first strike.)",
        &[AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::HasType(CardType::Creature),
        )],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&[
                AppliedEffectDef::modify_power_toughness(
                    ValueDef::Constant(3),
                    ValueDef::Constant(0),
                ),
                AppliedEffectDef::add_ability(&abilities::first_strike()),
            ]),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )]),
);

// BFZ 168 — Unnatural Aggression
pub(in crate::card::sets) static UNNATURAL_AGGRESSION: CardRecord = CardRecord::new(
    "Unnatural Aggression",
    "8293c66d-9a9b-4817-9bc3-ffd57fda290c",
    "James Ryman",
CardRules::new_instant(mana_cost!("{2}{G}"))
        .printed_colors(&[])
        .with_abilities(&[
            abilities::devoid(),
            AbilityDef::spell_with_targets(
                "Target creature you control fights target creature an opponent controls. If the creature an opponent controls would die this turn, exile it instead.",
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
                    EffectDef::Fight {
                        first: ObjectRefDef::Target(TargetIndex::PRIMARY),
                        second: ObjectRefDef::Target(TargetIndex(1)),
                        excess: None,
                    },
                    // This sentence is independent of whether the fight dealt damage. If the
                    // opponent's creature remains a legal target, any way it would die later
                    // this turn is replaced with exile.
                    EffectDef::Apply {
                        recipient: EffectRecipientDef::Target(TargetIndex(1)),
                        effect: AppliedEffectDef::Rule(AppliedRuleDef::ExileInsteadOfDying),
                        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                    },
                ]),
            ),
        ]),
);

// BFZ 209 — Bring to Light
pub(in crate::card::sets) static BRING_TO_LIGHT_209: CardRecord = CardRecord::new(
    "Bring to Light",
    "d25b13a4-6282-4426-8b01-9550f7d52d16",
    "Jonas De Ro",
    CardRules::new_sorcery(mana_cost!("{3}{G}{U}")).with_abilities(&[
AbilityDef::spell("Converge — Search your library for a creature, instant, or sorcery card with mana value less than or equal to the number of colors of mana spent to cast this spell, exile that card, then shuffle. You may cast that card without paying its mana cost.", EffectDef::SearchZone { player: EffectRecipientDef::Controller, source: ZoneKind::Library, object: ObjectPredicateDef::All(&[ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::AnyOf(&[ObjectPredicateDef::HasType(CardType::Instant), ObjectPredicateDef::HasType(CardType::Sorcery)])]), ObjectPredicateDef::ManaValueAtMostValue(ValueDef::ColorsOfManaSpent)]), minimum: 0, maximum: ValueDef::Constant(1), reveal: true, destination: ZoneKind::Exile, placement: ZonePlacement::Top, shuffle: true, enters_tapped: false, attachment: None, binding: Some(Binding!("bring_card")), then: Some(&EffectDef::MayPlayWithoutPaying(FreePlayDef { objects: ObjectSetDef::Binding(Binding!("bring_card")), duration: FreePlayDurationDef::WhileResolving, mandatory: false, grants_haste: false })) })
]),
);

// BFZ 223 — Hedron Archive
pub(in crate::card::sets) static HEDRON_ARCHIVE: CardRecord = CardRecord::new(
    "Hedron Archive",
    "2519149c-f8a3-413f-b7b2-cd596970be4c",
    "Craig J Spearing",
    CardRules::new_artifact(mana_cost!("{4}")).with_abilities(&[
        AbilityDef::activated_mana(
            "{T}: Add {C}{C}.",
            &[CostDef::TapSource],
            EffectDef::AddMana(AddManaEffectDef::one(ManaColor::Colorless).with_amount(2)),
        ),
        AbilityDef::activated(
            "{2}, {T}, Sacrifice this artifact: Draw two cards.",
            &[
                CostDef::Mana(mana_cost!("{2}")),
                CostDef::TapSource,
                CostDef::SacrificeSource,
            ],
            abilities::draw_cards(ValueDef::Constant(2)),
        ),
    ]),
);

// BFZ 242 — Sanctum of Ugin
pub(in crate::card::sets) static SANCTUM_OF_UGIN_242: CardRecord = CardRecord::new(
    "Sanctum of Ugin",
    "86798d03-9f2d-46bd-a660-13c8dd5535ce",
    "James Paick",
    CardRules::new_land(&[]).with_abilities(&[
abilities::tap_for(ManaColor::Colorless),
AbilityDef::triggered("Whenever you cast a colorless spell with mana value 7 or greater, you may sacrifice this land. If you do, search your library for a colorless creature card, reveal it, put it into your hand, then shuffle.", TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[ObjectPredicateDef::ControlledBy(PlayerRelation::You), ObjectPredicateDef::ColorCount(0), ObjectPredicateDef::Not(&ObjectPredicateDef::ManaValueAtMost(6))])), EffectDef::PayOr(PayOrDef::optional(&[CostDef::sacrifice_permanent(ObjectPredicateDef::Source)], &EffectDef::SearchZone { player: EffectRecipientDef::Controller, source: ZoneKind::Library, object: ObjectPredicateDef::All(&[ObjectPredicateDef::HasType(CardType::Creature), ObjectPredicateDef::ColorCount(0)]), minimum: 0, maximum: ValueDef::Constant(1), reveal: true, destination: ZoneKind::Hand, placement: ZonePlacement::Top, shuffle: true, enters_tapped: false, attachment: None, binding: None, then: None })))
]),
);

// BFZ 249 — Sunken Hollow
pub(in crate::card::sets) static SUNKEN_HOLLOW_249: CardRecord = CardRecord::new(
    "Sunken Hollow",
    "0dd1726f-b899-491a-8b0e-8e3d25f17d3d",
    "Adam Paquette",
    CardRules::new_land(&["Island", "Swamp"]).with_abilities(&[AbilityDef::as_enters(
        "This land enters tapped unless you control two or more basic lands.",
        ReplacementEffectDef::Conditional {
            condition: ConditionDef::ObjectCount(&ObjectCountConditionDef {
                query: ObjectQueryDef::matching(
                    ObjectPredicateDef::All(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::Supertype(CardSupertype::Basic),
                        ObjectPredicateDef::Not(&ObjectPredicateDef::Source),
                    ]),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::You,
                ),
                comparison: ComparisonDef::GreaterOrEqual,
                amount: 2,
            }),
            if_true: &[],
            if_false: &[ReplacementEffectDef::ModifyBattlefieldEntry(
                BattlefieldEntryModificationDef::Tapped,
            )],
        },
    )]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ULAMOG_THE_CEASELESS_HUNGER_15,
    &VOID_WINNOWER_17,
    &FELIDAR_CUB,
    &STASIS_SNARE,
    &ELDRAZI_SKYSPAWNER,
    &CARRIER_THRALL,
    &VAMPIRIC_RITES,
    &SURE_STRIKE,
    &UNNATURAL_AGGRESSION,
    &BRING_TO_LIGHT_209,
    &HEDRON_ARCHIVE,
    &SANCTUM_OF_UGIN_242,
    &SUNKEN_HOLLOW_249,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
