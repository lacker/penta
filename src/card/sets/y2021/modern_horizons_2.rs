//! Modern Horizons 2 cards cataloged as cross-format rules-engine test cases.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    AlternativeCastKindDef, AppliedEffectDef, BasicLandType, CardArt, CardRules, CardSet,
    CardSupertype, CardType, DiscardFollowUpDef, DiscardSelectionDef, DividedTotal, EffectDef,
    EffectRecipientDef, GraveyardTypeConditionDef, ManaColor, ObjectPredicateDef, ObjectQueryDef,
    PlayerRelation, SpellAdditionalCostDef, SpendModeDef, TokenCharacteristics,
    TriggerConditionDef, TriggerEventDef, ValueDef, ZoneKind, ZonePlacement, abilities,
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
    counters: None,
    object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    zone: ZoneKind::Exile,
    placement: ZonePlacement::Top,
    controller: None,
    arrival_effect: None,
    attachment: None,
};

// MH2 25 — Prismatic Ending
pub(in crate::card::sets) static PRISMATIC_ENDING: CardRecord = CardRecord::new_with_legacy_id(
    2193,
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

// MH2 36 — Unbounded Potential
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static UNBOUNDED_POTENTIAL: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9955a344-dcd8-404d-9757-f62ed158ba22"),
    "Unbounded Potential",
    crate::card::CardArt::new("9955a344-dcd8-404d-9757-f62ed158ba22", "Iain McCaig"),
    crate::card::CardSet::ModernHorizons2,
    crate::card::CardRules::unsupported(),
);

// MH2 46 — Hard Evidence
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HARD_EVIDENCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("501599d6-1072-4124-b05d-01f96de153f3"),
    "Hard Evidence",
    crate::card::CardArt::new("501599d6-1072-4124-b05d-01f96de153f3", "Yeong-Hao Han"),
    crate::card::CardSet::ModernHorizons2,
    crate::card::CardRules::unsupported(),
);

// MH2 49 — Lose Focus
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LOSE_FOCUS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("985bdb0c-ce6c-4506-8163-76f3b2fdf5fb"),
    "Lose Focus",
    crate::card::CardArt::new("985bdb0c-ce6c-4506-8163-76f3b2fdf5fb", "Martina Fačková"),
    crate::card::CardSet::ModernHorizons2,
    crate::card::CardRules::unsupported(),
);

// MH2 67 — Subtlety
pub(in crate::card::sets) static SUBTLETY: CardRecord = CardRecord::new_with_legacy_id(
    2236,
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
pub(in crate::card::sets) static BONE_SHARDS: CardRecord = CardRecord::new_with_legacy_id(
    2169,
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
pub(in crate::card::sets) static DAMN: CardRecord = CardRecord::new_with_legacy_id(
    2192,
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

// MH2 91 — Loathsome Curator
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LOATHSOME_CURATOR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("11a59a6f-6ef0-4acc-8358-a4e2cebdb7d5"),
    "Loathsome Curator",
    crate::card::CardArt::new("11a59a6f-6ef0-4acc-8358-a4e2cebdb7d5", "Mila Pesic"),
    crate::card::CardSet::ModernHorizons2,
    crate::card::CardRules::unsupported(),
);

// MH2 95 — Nested Shambler
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static NESTED_SHAMBLER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9851f290-f502-49f8-9b48-67f7966d4e34"),
    "Nested Shambler",
    crate::card::CardArt::new("9851f290-f502-49f8-9b48-67f7966d4e34", "Nicholas Gregory"),
    crate::card::CardSet::ModernHorizons2,
    crate::card::CardRules::unsupported(),
);

// MH2 107 — Vermin Gorger
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static VERMIN_GORGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("d3166b10-5bc3-4db6-bb5b-81045d98e446"),
    "Vermin Gorger",
    crate::card::CardArt::new("d3166b10-5bc3-4db6-bb5b-81045d98e446", "Tobias Kwan"),
    crate::card::CardSet::ModernHorizons2,
    crate::card::CardRules::unsupported(),
);

// MH2 126 — Fury
pub(in crate::card::sets) static FURY: CardRecord = CardRecord::new_with_legacy_id(
    2157,
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
pub(in crate::card::sets) static MINE_COLLAPSE: CardRecord = CardRecord::new_with_legacy_id(
    2261,
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
pub(in crate::card::sets) static UNHOLY_HEAT: CardRecord = CardRecord::new_with_legacy_id(
    2159,
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

static EXILE_A_GREEN_CARD: SpellAdditionalCostDef = SpellAdditionalCostDef::new(
    ObjectPredicateDef::Color(ManaColor::Green),
    ZoneKind::Hand,
    1,
)
.spent(SpendModeDef::Exile);

/// "Up to one target player" includes yourself, which is the mode nobody
/// prints on the card: an Endurance can put your own graveyard back when
/// something else is trying to eat it.
static ENDURANCE_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::up_to(
    AbilityTargetPredicate::Player(PlayerRelation::Any),
    1,
)];

static ENDURANCE_ABILITIES: [AbilityDef; 5] = [
    abilities::flash(),
    abilities::reach(),
    AbilityDef::triggered_with_targets(
        "When this creature enters, up to one target player puts all the cards from their \
         graveyard on the bottom of their library in a random order.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::Source,
            None,
            Some(ZoneKind::Battlefield),
        ),
        &ENDURANCE_TARGET,
        EffectDef::BuryGraveyard {
            player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        },
    ),
    AbilityDef::alternative_cast(
        mana_cost!("{0}"),
        AlternativeCastKindDef::AlternativeCost,
        Some("Evoke—Exile a green card from your hand."),
        EffectDef::None,
    )
    .with_alternative_additional_cost(&EXILE_A_GREEN_CARD),
    abilities::evoke_sacrifice("When this creature enters, if it was evoked, sacrifice it."),
];

// MH2 147 — Abundant Harvest
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ABUNDANT_HARVEST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("16782095-0b7f-4489-8a97-b74f8efef352"),
    "Abundant Harvest",
    crate::card::CardArt::new("5ad86b17-3fed-418a-938c-c49adb409531", "Iris Compiet"),
    crate::card::CardSet::ModernHorizons2,
    crate::card::CardRules::unsupported(),
);

// MH2 149 — Bannerhide Krushok
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BANNERHIDE_KRUSHOK: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1271251b-7d79-4cb4-80bb-98574aa63249"),
    "Bannerhide Krushok",
    crate::card::CardArt::new("1271251b-7d79-4cb4-80bb-98574aa63249", "Joe Slucher"),
    crate::card::CardSet::ModernHorizons2,
    crate::card::CardRules::unsupported(),
);

// MH2 157 — Endurance
pub(in crate::card::sets) static ENDURANCE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("eb0e0404-4846-4891-acfa-bd0951ecf9c6"),
    "Endurance",
    CardArt::new(
        "eb0e0404-4846-4891-acfa-bd0951ecf9c6",
        "Anastasia Ovchinnikova",
    ),
    CardSet::ModernHorizons2,
    // A free answer to a graveyard that leaves a 3/4 blocker behind, or a
    // green card off the top of your hand when the graveyard is the whole
    // reason you are casting it.
    CardRules::new_creature(mana_cost!("{1}{G}{G}"), &["Elemental", "Incarnation"], 3, 4)
        .with_abilities(&ENDURANCE_ABILITIES),
);

// MH2 181 — Urban Daggertooth
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static URBAN_DAGGERTOOTH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4ab83a39-d90d-403e-b74d-fe99c8b2aacd"),
    "Urban Daggertooth",
    crate::card::CardArt::new("4ab83a39-d90d-403e-b74d-fe99c8b2aacd", "Randy Vargas"),
    crate::card::CardSet::ModernHorizons2,
    crate::card::CardRules::unsupported(),
);

// MH2 188 — Captured by Lagacs
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static CAPTURED_BY_LAGACS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7ce1c2a8-688b-4f63-8d58-e325efc6052a"),
    "Captured by Lagacs",
    crate::card::CardArt::new("7ce1c2a8-688b-4f63-8d58-e325efc6052a", "Andrew Mar"),
    crate::card::CardSet::ModernHorizons2,
    crate::card::CardRules::unsupported(),
);

// MH2 202 — Grist, the Hunger Tide
// Audit: blocked — Needs three capabilities at once: a resolution loop that repeats a step while reading what the previous iteration milled, a reflexive triggered ability that chooses its target when the optional sacrifice is actually made rather than on activation, and characteristics that apply in every zone except the battlefield.

/// Domain: how many of the five basic land types are among your lands. A
/// Kavu on a two-colour board is a 2/2, and one behind a full spread of
/// fetched duals is a 5/5.
static KAVU_DOMAIN: AppliedEffectDef = AppliedEffectDef::set_base_power_toughness(
    ValueDef::BasicLandTypesControlled(PlayerRelation::You),
    ValueDef::BasicLandTypesControlled(PlayerRelation::You),
);

/// "If you do": the draw is sized by what the discard actually took, so an
/// empty hand discards nothing and draws nothing.
static KAVU_DRAW_WHAT_WAS_DISCARDED: EffectDef = EffectDef::DrawCards {
    recipient: EffectRecipientDef::Controller,
    amount: ValueDef::MatchedCount,
};

static A_CARD_IN_A_GRAVEYARD: [AbilityTargetDef; 1] = [AbilityTargetDef::up_to(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::Any,
        zones: &[ZoneKind::Graveyard],
        controller: None,
        owner: None,
    },
    1,
)];

static KAVU_MODES: [AbilityDef; 2] = [
    AbilityDef::spell(
        "Discard a card. If you do, draw a card.",
        EffectDef::Discard {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(1),
            selection: DiscardSelectionDef::RecipientChooses,
            then: Some(DiscardFollowUpDef {
                counted: ObjectPredicateDef::Any,
                bound: None,
                effect: &KAVU_DRAW_WHAT_WAS_DISCARDED,
            }),
        },
    ),
    AbilityDef::spell_with_targets(
        "Exile up to one target card from a graveyard.",
        &A_CARD_IN_A_GRAVEYARD,
        EffectDef::MoveToZone {
            counters: None,
            object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            zone: ZoneKind::Exile,
            placement: ZonePlacement::Top,
            arrival_effect: None,
            attachment: None,
            controller: None,
        },
    ),
];

// MH2 216 — Territorial Kavu
pub(in crate::card::sets) static TERRITORIAL_KAVU: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2605df98-0b02-4aab-bc36-01e93c693743"),
    "Territorial Kavu",
    CardArt::new("2605df98-0b02-4aab-bc36-01e93c693743", "E. M. Gist"),
    CardSet::ModernHorizons2,
    // Two mana for as big a body as your mana base is greedy, and an attack
    // trigger that either loots or eats a graveyard.
    CardRules::new_creature(mana_cost!("{R}{G}"), &["Kavu"], 0, 0).with_abilities(&[
        AbilityDef::static_ability(
            "Domain — This creature's power and toughness are each equal to the number of basic \
             land types among lands you control.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::Source,
                effect: KAVU_DOMAIN,
            },
        )
        .with_coverage(AbilityCoverageDef::partial(
            "A characteristic-defining ability sets power and toughness in every zone. This is a \
             battlefield-only continuous effect, so the value is right wherever the card is \
             played and absent for anything reading it in another zone.",
        )),
        AbilityDef::modal_triggered(
            "Whenever this creature attacks, choose one —\n• Discard a card. If you do, draw a \
             card.\n• Exile up to one target card from a graveyard.",
            TriggerEventDef::attacks(ObjectPredicateDef::Source),
            &KAVU_MODES,
        ),
    ]),
);

// MH2 231 — Nettlecyst
pub(in crate::card::sets) static NETTLECYST: CardRecord = CardRecord::new_with_legacy_id(
    2126,
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
pub(in crate::card::sets) static YAVIMAYA_CRADLE_OF_GROWTH: CardRecord =
    CardRecord::new_with_legacy_id(
        262,
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

// MH2 421 — Goblin Anarchomancer
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_ANARCHOMANCER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("633a3423-501d-4b22-95a6-743233be521e"),
    "Goblin Anarchomancer",
    crate::card::CardArt::new("f7f07a80-05b5-4108-9e68-f8da05866acc", "Joe Slucher"),
    crate::card::CardSet::ModernHorizons2,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &PRISMATIC_ENDING,
    &UNBOUNDED_POTENTIAL,
    &HARD_EVIDENCE,
    &LOSE_FOCUS,
    &SUBTLETY,
    &BONE_SHARDS,
    &DAMN,
    &LOATHSOME_CURATOR,
    &NESTED_SHAMBLER,
    &VERMIN_GORGER,
    &FURY,
    &MINE_COLLAPSE,
    &UNHOLY_HEAT,
    &ABUNDANT_HARVEST,
    &BANNERHIDE_KRUSHOK,
    &ENDURANCE,
    &URBAN_DAGGERTOOTH,
    &CAPTURED_BY_LAGACS,
    &TERRITORIAL_KAVU,
    &NETTLECYST,
    &YAVIMAYA_CRADLE_OF_GROWTH,
    &GOBLIN_ANARCHOMANCER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
