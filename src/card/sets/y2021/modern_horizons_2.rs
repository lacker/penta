//! Modern Horizons 2 cards cataloged as cross-format rules-engine test cases.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AlternativeCastKindDef,
    AppliedEffectDef, BasicLandType, CardArt, CardRules, CardSet, CardSupertype, CardType,
    DividedTotal, EffectDef, EffectRecipientDef, GraveyardTypeConditionDef, ManaColor,
    ObjectPredicateDef, ObjectQueryDef, PlayerRelation, SpellAdditionalCostDef, SpendModeDef,
    TokenCharacteristics, TriggerConditionDef, TriggerEventDef, ValueDef, ZoneKind, ZonePlacement,
    abilities, cards,
};
use crate::{TargetIndex, mana_cost};

/// "Artifact and/or enchantment" is one query rather than two sums: a
/// permanent that is both is counted once, and Nettlecyst counts itself.
static ARTIFACTS_AND_ENCHANTMENTS_YOU_CONTROL: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::AnyOf(&[
        ObjectPredicateDef::HasType(CardType::Artifact),
        ObjectPredicateDef::HasType(CardType::Enchantment),
    ]),
    &[ZoneKind::Battlefield],
    PlayerRelation::You,
);

/// Four damage split however the caster likes, over creatures and
/// planeswalkers alike. Every target must be assigned at least one, so four
/// is the most it can ever cover.
static FURY_TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef {
    predicate: AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::HasType(CardType::Planeswalker),
        ]),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
    minimum: 1,
    maximum: AbilityTargetDef::UNLIMITED,
    divided_total: Some(DividedTotal::Fixed(4)),
    another: false,
}];

static EXILE_A_RED_CARD: SpellAdditionalCostDef =
    SpellAdditionalCostDef::new(ObjectPredicateDef::Color(ManaColor::Red), ZoneKind::Hand, 1)
        .spent(SpendModeDef::Exile);

static FURY_ABILITIES: [AbilityDef; 4] = [
    abilities::double_strike(),
    AbilityDef::triggered_with_targets(
        "When this creature enters, it deals 4 damage divided as you choose among any number of target creatures and/or planeswalkers.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::Source,
            None,
            Some(ZoneKind::Battlefield),
        ),
        &FURY_TARGETS,
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::DividedAmongTargets,
        },
    ),
    AbilityDef::alternative_cast(
        mana_cost!("{0}"),
        AlternativeCastKindDef::AlternativeCost,
        Some("Evoke—Exile a red card from your hand."),
        EffectDef::None,
    )
    .with_alternative_additional_cost(&EXILE_A_RED_CARD),
    // Evoke's own sacrifice. It is a separate trigger because it happens
    // after the Elemental has arrived, alongside the damage trigger rather
    // than instead of it -- which is why an evoked Fury still burns.
    abilities::evoke_sacrifice("When this creature enters, if it was evoked, sacrifice it."),
];

/// The second half of "sacrifice a creature or discard a card". Which half
/// is paid is settled as the spell is cast: both spend a card the caster
/// already had, and the enumeration offers every one of them.
static BONE_SHARDS_DISCARD: SpellAdditionalCostDef =
    SpellAdditionalCostDef::new(ObjectPredicateDef::Any, ZoneKind::Hand, 1);

static BONE_SHARDS_COST: SpellAdditionalCostDef = SpellAdditionalCostDef::new(
    ObjectPredicateDef::HasType(CardType::Creature),
    ZoneKind::Battlefield,
    1,
)
.or(&BONE_SHARDS_DISCARD);

static BONE_SHARDS_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::AnyOf(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::HasType(CardType::Planeswalker),
    ]),
)];

/// A nonland permanent of any size may be targeted; whether it is actually
/// exiled is settled on resolution, against what paid for the spell.
static ENDING_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
)];

static ENDING_SMALL_ENOUGH: TriggerConditionDef = TriggerConditionDef::TargetMatches {
    slot: TargetIndex::PRIMARY,
    object: ObjectPredicateDef::ManaValueAtMostValue(ValueDef::ColorsOfManaSpent),
};

static ENDING_EXILE: EffectDef = EffectDef::MoveToZone {
    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    zone: ZoneKind::Exile,
    placement: ZonePlacement::Top,
    controller: None,
    arrival_effect: None,
    attachment: None,
};

// MH2 25 — Prismatic Ending
pub(in crate::card::sets) static PRISMATIC_ENDING: CardRecord = CardRecord::new(
    cards::PRISMATIC_ENDING,
    "Prismatic Ending",
    CardArt::new("825969b9-3c70-4fca-8cab-696e9ca7cdb2", "John Stanko"),
    CardSet::ModernHorizons2,
    // X buys nothing by itself: it is a sink for the extra colours, and how
    // many different ones went in is the only thing the spell reads.
    CardRules::new_sorcery(mana_cost!("{X}{W}"))
        .with_converge()
        .with_ability(AbilityDef::spell_with_targets(
            "Converge — Exile target nonland permanent if its mana value is less than or equal to the number of colors of mana spent to cast this spell.",
            &ENDING_TARGET,
            EffectDef::IfCondition {
                condition: &ENDING_SMALL_ENOUGH,
                then: &ENDING_EXILE,
            },
        )),
);

static DAMN_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Creature),
)];

static EXILE_A_BLUE_CARD: SpellAdditionalCostDef = SpellAdditionalCostDef::new(
    ObjectPredicateDef::Color(ManaColor::Blue),
    ZoneKind::Hand,
    1,
)
.spent(SpendModeDef::Exile);

/// A creature or planeswalker spell on the stack, anybody's. "Up to one"
/// means a Subtlety with nothing worth answering still enters and still
/// leaves a 3/3 behind.
static SUBTLETY_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::up_to(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::HasType(CardType::Planeswalker),
        ]),
        zones: &[ZoneKind::Stack],
        controller: None,
        owner: None,
    },
    1,
)];

static SUBTLETY_ABILITIES: [AbilityDef; 5] = [
    abilities::flash(),
    abilities::flying(),
    AbilityDef::triggered_with_targets(
        "When this creature enters, choose up to one target creature spell or planeswalker \
         spell. Its owner puts it on their choice of the top or bottom of their library.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::Source,
            None,
            Some(ZoneKind::Battlefield),
        ),
        &SUBTLETY_TARGET,
        EffectDef::PutSpellIntoOwnersLibrary {
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        },
    ),
    AbilityDef::alternative_cast(
        mana_cost!("{0}"),
        AlternativeCastKindDef::AlternativeCost,
        Some("Evoke—Exile a blue card from your hand."),
        EffectDef::None,
    )
    .with_alternative_additional_cost(&EXILE_A_BLUE_CARD),
    abilities::evoke_sacrifice("When this creature enters, if it was evoked, sacrifice it."),
];

// MH2 67 — Subtlety
pub(in crate::card::sets) static SUBTLETY: CardRecord = CardRecord::new(
    cards::SUBTLETY,
    "Subtlety",
    CardArt::new(
        "701256d5-1389-48b7-9581-d6037209bd06",
        "Anastasia Ovchinnikova",
    ),
    CardSet::ModernHorizons2,
    // Free interaction that leaves a body when you have the mana, and a
    // blue card off the top of your hand when you do not.
    CardRules::new_creature(mana_cost!("{2}{U}{U}"), &["Elemental", "Incarnation"], 3, 3)
        .with_abilities(&SUBTLETY_ABILITIES),
);

// MH2 76 — Bone Shards
pub(in crate::card::sets) static BONE_SHARDS: CardRecord = CardRecord::new(
    cards::BONE_SHARDS,
    "Bone Shards",
    CardArt::new("1ee98955-4c47-4d45-9377-608dfa755337", "Tommy Arnold"),
    CardSet::ModernHorizons2,
    // One black kills anything, and the second card is the price. A deck
    // full of things it wants in the graveyard pays it gladly.
    CardRules::new_sorcery(mana_cost!("{B}")).with_ability(
        AbilityDef::spell_with_additional_cost(
            "As an additional cost to cast this spell, sacrifice a creature or discard a card.\nDestroy target creature or planeswalker.",
            &BONE_SHARDS_TARGET,
            BONE_SHARDS_COST,
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: true,
            },
        ),
    ),
);

// MH2 80 — Damn
pub(in crate::card::sets) static DAMN: CardRecord = CardRecord::new(
    cards::DAMN,
    "Damn",
    CardArt::new("efeae088-9ac5-4d2f-a15c-d8675a471ac5", "Lucas Graciano"),
    CardSet::ModernHorizons2,
    // Two black is removal and four with two white is a Wrath, off one card
    // -- and neither half leaves anything to regenerate, which is what puts
    // it ahead of the sorceries it is otherwise a copy of.
    CardRules::new_sorcery(mana_cost!("{B}{B}")).with_abilities(&[
        AbilityDef::spell_with_targets(
            "Destroy target creature. A creature destroyed this way can't be regenerated.",
            &DAMN_TARGET,
            EffectDef::Destroy {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                can_regenerate: false,
            },
        ),
        AbilityDef::alternative_cast(
            mana_cost!("{2}{W}{W}"),
            AlternativeCastKindDef::Overload,
            Some("Destroy each creature. A creature destroyed this way can't be regenerated."),
            EffectDef::Destroy {
                object: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Creature),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                can_regenerate: false,
            },
        ),
    ]),
);

// MH2 126 — Fury
pub(in crate::card::sets) static FURY: CardRecord = CardRecord::new(
    cards::FURY,
    "Fury",
    CardArt::new("bd281158-8180-40b9-a5b7-03cfc712d81a", "Raoul Vitale"),
    CardSet::ModernHorizons2,
    CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Elemental", "Incarnation"], 3, 3)
        .with_abilities(&FURY_ABILITIES),
);

/// Delirium changes the amount, not the effect, so it is a conditional value
/// rather than a second clause: four card types in your own graveyard, and
/// the same spell deals six.
static UNHOLY_HEAT_AMOUNT: GraveyardTypeConditionDef = GraveyardTypeConditionDef {
    player: PlayerRelation::You,
    minimum: 4,
    then: ValueDef::Constant(6),
    otherwise: ValueDef::Constant(2),
};

/// A Mountain, not a red source: what the cost names is the land type, so a
/// Sacred Foundry pays it and a Mountain that has stopped being one does not.
static SACRIFICE_A_MOUNTAIN: SpellAdditionalCostDef = SpellAdditionalCostDef::new(
    ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Mountain]),
    ZoneKind::Battlefield,
    1,
);

/// "If it's your turn" gates only the free cast. The printed cost is always
/// available, which is why this is a condition on the alternative rather
/// than a restriction on the card.
static YOUR_TURN: TriggerConditionDef = TriggerConditionDef::ActivePlayer(PlayerRelation::You);

static MINE_COLLAPSE_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::HasType(CardType::Planeswalker),
        ]),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
)];

// MH2 135 — Mine Collapse
pub(in crate::card::sets) static MINE_COLLAPSE: CardRecord = CardRecord::new(
    cards::MINE_COLLAPSE,
    "Mine Collapse",
    CardArt::new("56e2e8b5-660d-4469-a4fe-2367dfadb709", "Bud Cook"),
    CardSet::ModernHorizons2,
    // Nobody pays four mana for this. What it is worth is a land off an
    // already-flooded board on your own turn, which is why the free half is
    // the half that reads "if it's your turn".
    CardRules::new_instant(mana_cost!("{3}{R}")).with_abilities(&[
        AbilityDef::alternative_cast(
            mana_cost!("{0}"),
            AlternativeCastKindDef::AlternativeCost,
            Some(
                "If it's your turn, you may sacrifice a Mountain rather than pay this spell's \
                 mana cost.",
            ),
            EffectDef::None,
        )
        .with_alternative_additional_cost(&SACRIFICE_A_MOUNTAIN)
        .with_alternative_condition(&YOUR_TURN),
        AbilityDef::spell_with_targets(
            "Mine Collapse deals 5 damage to target creature or planeswalker.",
            &MINE_COLLAPSE_TARGET,
            EffectDef::DealDamage {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(5),
            },
        ),
    ]),
);

static UNHOLY_HEAT_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::AnyOf(&[
            ObjectPredicateDef::HasType(CardType::Creature),
            ObjectPredicateDef::HasType(CardType::Planeswalker),
        ]),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
)];

// MH2 145 — Unholy Heat
pub(in crate::card::sets) static UNHOLY_HEAT: CardRecord = CardRecord::new(
    cards::UNHOLY_HEAT,
    "Unholy Heat",
    CardArt::new("2b73d294-6ab1-4051-9b0f-d8e335d37674", "Kari Christensen"),
    CardSet::ModernHorizons2,
    CardRules::new_instant(mana_cost!("{R}")).with_ability(AbilityDef::spell_with_targets(
        "Unholy Heat deals 2 damage to target creature or planeswalker.\nDelirium — Unholy Heat deals 6 damage instead if there are four or more card types among cards in your graveyard.",
        &UNHOLY_HEAT_TARGET,
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::IfCardTypesAmongGraveyards(&UNHOLY_HEAT_AMOUNT),
        },
    )),
);

// MH2 202 — Grist, the Hunger Tide
// Audit: blocked — Needs three capabilities at once: a resolution loop that repeats a step while reading what the previous iteration milled, a reflexive triggered ability that chooses its target when the optional sacrifice is actually made rather than on activation, and characteristics that apply in every zone except the battlefield.

// MH2 231 — Nettlecyst
pub(in crate::card::sets) static NETTLECYST: CardRecord = CardRecord::new(
    cards::NETTLECYST,
    "Nettlecyst",
    CardArt::new("4a0bb5dc-75a6-4bd6-81f8-611197fb0fba", "Vincent Proce"),
    CardSet::ModernHorizons2,
    CardRules::new_artifact(mana_cost!("{3}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::living_weapon(
                TokenCharacteristics::creature(&["Phyrexian", "Germ"], &[ManaColor::Black], 0, 0)
                    .with_art(CardArt::new(
                        "b53e0681-603e-4180-bc86-3dadf214e61a",
                        "Igor Kieryluk",
                    )),
            ),
            AbilityDef::static_ability(
                "Equipped creature gets +1/+1 for each artifact and/or enchantment you control.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::modify_power_toughness(
                        ValueDef::CountMatchingObjects(&ARTIFACTS_AND_ENCHANTMENTS_YOU_CONTROL),
                        ValueDef::CountMatchingObjects(&ARTIFACTS_AND_ENCHANTMENTS_YOU_CONTROL),
                    ),
                },
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{2}"))], "Equip {2}"),
        ]),
);

// MH2 261 — Yavimaya, Cradle of Growth
pub(in crate::card::sets) static YAVIMAYA_CRADLE_OF_GROWTH: CardRecord = CardRecord::new(
    cards::YAVIMAYA_CRADLE_OF_GROWTH,
    "Yavimaya, Cradle of Growth",
    CardArt::new("4e4b6e22-93b2-4896-bba5-0ceaa5d8ea3c", "Sarah Finnigan"),
    CardSet::ModernHorizons2,
    CardRules::new_land(&[])
        .with_supertype(CardSupertype::Legendary)
        .with_ability(AbilityDef::static_ability(
            "Each land is a Forest in addition to its other land types.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::matching_objects(
                    ObjectPredicateDef::HasType(CardType::Land),
                    &[ZoneKind::Battlefield],
                    PlayerRelation::Any,
                ),
                effect: AppliedEffectDef::add_basic_land_types(&[BasicLandType::Forest]),
            },
        )),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &PRISMATIC_ENDING,
    &SUBTLETY,
    &BONE_SHARDS,
    &DAMN,
    &FURY,
    &MINE_COLLAPSE,
    &UNHOLY_HEAT,
    &NETTLECYST,
    &YAVIMAYA_CRADLE_OF_GROWTH,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
