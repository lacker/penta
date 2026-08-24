//! Modern Horizons 2 cards cataloged as cross-format rules-engine test cases.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    AlternativeCastKindDef, AppliedEffectDef, BasicLandType, CardArt, CardRules, CardSet,
    CardSupertype, CardType, DamageEventMatcherDef, DamageKindDef, DamageRecipientMatcherDef,
    DamageSourceMatcherDef, DiscardFollowUpDef, DiscardSelectionDef, DividedTotal, EffectDef,
    EffectRecipientDef, ExilePlayDurationDef, GraveyardTypeConditionDef, ManaColor,
    ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, PlayerRelation, SacrificedAmountDef,
    SpellAdditionalCostDef, SpendModeDef, TriggerConditionDef, TriggerEventDef, ValueDef, ZoneKind,
    ZonePlacement, abilities, tokens,
};
use crate::{TargetIndex, mana_cost};

// MH2 25 — Prismatic Ending
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
    from: None,
    zone: ZoneKind::Exile,
    placement: ZonePlacement::Top,
    controller: None,
    arrival_effect: None,
    attachment: None,
    tapped: false,
};

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

// MH2 32 — Solitude
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SOLITUDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b648cc94-7880-456b-82ea-859746d52397"),
    "Solitude",
    crate::card::CardArt::new("47a6234f-309f-4e03-9263-66da48b57153", "Evan Shipard"),
    crate::card::CardSet::ModernHorizons2,
    crate::card::CardRules::unsupported(),
);

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
    abilities::enters_trigger_with_targets(
        "When this creature enters, choose up to one target creature spell or planeswalker \
         spell. Its owner puts it on their choice of the top or bottom of their library.",
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

// MH2 75 — Archon of Cruelty
/// One printed ability with two ways in: he arrives, or he attacks. Two
/// abilities would make him trigger twice on a turn he does both, which the
/// card does not say -- and would count as two triggered abilities where the
/// card has one.
static ARCHON_TRIGGERS: [TriggerEventDef; 2] = [
    TriggerEventDef::zone_changed(
        ObjectPredicateDef::Source,
        None,
        Some(ZoneKind::Battlefield),
    ),
    TriggerEventDef::attacks(ObjectPredicateDef::Source),
];

static AN_OPPONENT: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
)];

static A_CREATURE_OR_PLANESWALKER: ObjectPredicateDef = ObjectPredicateDef::AnyOf(&[
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::HasType(CardType::Planeswalker),
]);

/// Four things in one sentence, in the order they are printed: what the
/// opponent gives up, then what you get. The sacrifice is theirs to choose,
/// which is why it is a procedure rather than a targeted destruction.
static ARCHON_TOLL: [EffectDef; 4] = [
    EffectDef::SacrificeOfChoice {
        player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        object: A_CREATURE_OR_PLANESWALKER,
        count: ValueDef::Constant(1),
        then: None,
        amount: SacrificedAmountDef::Power,
        otherwise: None,
        optional: false,
    },
    EffectDef::Discard {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        amount: ValueDef::Constant(1),
        selection: DiscardSelectionDef::RecipientChooses,
        then: None,
    },
    EffectDef::LoseLife {
        recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        amount: ValueDef::Constant(3),
    },
    EffectDef::Sequence(&ARCHON_REWARD),
];

static ARCHON_REWARD: [EffectDef; 2] = [
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    },
    EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(3),
    },
];

pub(in crate::card::sets) static ARCHON_OF_CRUELTY: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1be9d9a4-d7ee-4854-abc2-85cabf993ec9"),
    "Archon of Cruelty",
    CardArt::new("1be9d9a4-d7ee-4854-abc2-85cabf993ec9", "Andrew Mar"),
    CardSet::ModernHorizons2,
    // Eight mana nobody pays: he is a reanimation target, and the trigger is
    // why -- a six-point swing and two cards the turn he lands, and again
    // every turn he attacks.
    CardRules::new_creature(mana_cost!("{6}{B}{B}"), &["Archon"], 6, 6).with_abilities(&[
        abilities::flying(),
        AbilityDef::triggered_with_targets(
            "Whenever this creature enters or attacks, target opponent sacrifices a creature or \
             planeswalker of their choice, discards a card, and loses 3 life. You draw a card and \
             gain 3 life.",
            TriggerEventDef::AnyOf(&ARCHON_TRIGGERS),
            &AN_OPPONENT,
            EffectDef::Sequence(&ARCHON_TOLL),
        ),
    ]),
);

// MH2 76 — Bone Shards
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
                then: None,
            },
        ),
    ),
);

// MH2 80 — Damn
static DAMN_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::HasType(CardType::Creature),
)];

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
                then: None,
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
                then: None,
            },
        ),
    ]),
);

// MH2 87 — Grief
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GRIEF: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("40d77804-b81f-4e89-8528-1f3970ef3cd6"),
    "Grief",
    crate::card::CardArt::new("e6befbc4-1320-4f26-bd9f-b1814fedda10", "Nicholas Gregory"),
    crate::card::CardSet::ModernHorizons2,
    crate::card::CardRules::unsupported(),
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

// MH2 121 — Dragon's Rage Channeler
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DRAGON_S_RAGE_CHANNELER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4ced112a-e775-4f97-97b3-74877e9dce12"),
    "Dragon's Rage Channeler",
    crate::card::CardArt::new("4ced112a-e775-4f97-97b3-74877e9dce12", "Martina Fačková"),
    crate::card::CardSet::ModernHorizons2,
    crate::card::CardRules::unsupported(),
);

// MH2 126 — Fury
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
    abilities::enters_trigger_with_targets(
        "When this creature enters, it deals 4 damage divided as you choose among any number of target creatures and/or planeswalkers.",
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

pub(in crate::card::sets) static FURY: CardRecord = CardRecord::new_with_legacy_id(
    2157,
    "Fury",
    CardArt::new("bd281158-8180-40b9-a5b7-03cfc712d81a", "Raoul Vitale"),
    CardSet::ModernHorizons2,
    CardRules::new_creature(mana_cost!("{3}{R}{R}"), &["Elemental", "Incarnation"], 3, 3)
        .with_abilities(&FURY_ABILITIES),
);

// MH2 135 — Mine Collapse
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

// MH2 138 — Ragavan, Nimble Pilferer
static RAGAVAN_CONNECTS: [EffectDef; 2] = [
    EffectDef::create_token(tokens::treasure()).with_art(CardArt::new(
        "630c0d1c-9ddb-4e76-a82a-9cdd8a5b487b",
        "Alayna Danner",
    )),
    // "That player's library", and the permission is yours: what the Monkey
    // steals is theirs to lose and yours to cast.
    EffectDef::ExileTopOfLibraryToPlay {
        player: EffectRecipientDef::EventPlayer,
        amount: ValueDef::Constant(1),
        free: false,
        face_down: false,
        duration: ExilePlayDurationDef::ThisTurn,
        spend_any_color: false,
        play_condition: None,
    },
];

static RAGAVAN_ABILITIES: [AbilityDef; 4] = [
    AbilityDef::triggered(
        "Whenever this creature deals combat damage to a player, create a Treasure token and \
         exile the top card of that player's library. Until end of turn, you may cast that card.",
        TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
        EffectDef::Sequence(&RAGAVAN_CONNECTS),
    ),
    abilities::dash(
        mana_cost!("{1}{R}"),
        "Dash {1}{R} (You may cast this spell for its dash cost. If you do, it gains haste, and \
         it's returned from the battlefield to its owner's hand at the beginning of the next end \
         step.)",
    ),
    abilities::dashed_haste(),
    abilities::dashed_return(),
];

pub(in crate::card::sets) static RAGAVAN_NIMBLE_PILFERER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("a9738cda-adb1-47fb-9f4c-ecd930228c4d"),
    "Ragavan, Nimble Pilferer",
    CardArt::new("a9738cda-adb1-47fb-9f4c-ecd930228c4d", "Simon Dominic"),
    CardSet::ModernHorizons2,
    // One mana for a 2/1 that pays for itself the first time it connects,
    // and a dash cost for the turns when leaving it out would only get it
    // killed.
    CardRules::new_creature(mana_cost!("{R}"), &["Monkey", "Pirate"], 2, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&RAGAVAN_ABILITIES),
);

// MH2 145 — Unholy Heat
/// Delirium changes the amount, not the effect, so it is a conditional value
/// rather than a second clause: four card types in your own graveyard, and
/// the same spell deals six.
static UNHOLY_HEAT_AMOUNT: GraveyardTypeConditionDef = GraveyardTypeConditionDef {
    player: PlayerRelation::You,
    minimum: 4,
    then: ValueDef::Constant(6),
    otherwise: ValueDef::Constant(2),
};

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
    abilities::enters_trigger_with_targets(
        "When this creature enters, up to one target player puts all the cards from their \
         graveyard on the bottom of their library in a random order.",
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
// Audit: metadata-only — Needs three capabilities at once: a resolution loop that repeats a step while reading what the previous iteration milled, a reflexive triggered ability that chooses its target when the optional sacrifice is actually made rather than on activation, and characteristics that apply in every zone except the battlefield.
pub(in crate::card::sets) static GRIST_THE_HUNGER_TIDE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8eadbeaf-f01c-4c85-8eaf-6a569a1bdf64"),
    "Grist, the Hunger Tide",
    crate::card::CardArt::new("69af2825-18c2-4463-b6ba-42eaa070ccc1", "Yongjae Choi"),
    crate::card::CardSet::ModernHorizons2,
    crate::card::CardRules::unsupported(),
);

// MH2 216 — Territorial Kavu
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
            from: None,
            zone: ZoneKind::Exile,
            placement: ZonePlacement::Top,
            arrival_effect: None,
            attachment: None,
            controller: None,
            tapped: false,
        },
    ),
];

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

// MH2 227 — Kaldra Compleat
/// The clause the equipped creature gains, not one Kaldra has itself: "that
/// creature" is the one that took the damage, which is a different object
/// from the one that dealt it.
static KALDRA_EXILES_WHAT_IT_HITS: AbilityDef = AbilityDef::triggered(
    "Whenever this creature deals combat damage to a creature, exile that creature.",
    TriggerEventDef::DamageDealt(DamageEventMatcherDef {
        kind: DamageKindDef::Combat,
        source: DamageSourceMatcherDef::Object(ObjectRefDef::Source),
        recipient: DamageRecipientMatcherDef::MatchingObject(ObjectPredicateDef::HasType(
            CardType::Creature,
        )),
    }),
    EffectDef::MoveToZone {
        counters: None,
        object: EffectRecipientDef::object(ObjectRefDef::DamagedObject),
        from: None,
        zone: ZoneKind::Exile,
        placement: ZonePlacement::Top,
        arrival_effect: None,
        attachment: None,
        controller: None,
        tapped: false,
    },
);

static KALDRA_FIRST_STRIKE: AbilityDef = abilities::first_strike();

static KALDRA_TRAMPLE: AbilityDef = abilities::trample();

static KALDRA_INDESTRUCTIBLE: AbilityDef = abilities::indestructible();

static KALDRA_HASTE: AbilityDef = abilities::haste();

static KALDRA_GRANTS: [AppliedEffectDef; 6] = [
    AppliedEffectDef::modify_power_toughness(ValueDef::Constant(5), ValueDef::Constant(5)),
    AppliedEffectDef::add_ability(&KALDRA_FIRST_STRIKE),
    AppliedEffectDef::add_ability(&KALDRA_TRAMPLE),
    AppliedEffectDef::add_ability(&KALDRA_INDESTRUCTIBLE),
    AppliedEffectDef::add_ability(&KALDRA_HASTE),
    AppliedEffectDef::add_ability(&KALDRA_EXILES_WHAT_IT_HITS),
];

static KALDRA_EQUIP_COST: [AbilityCostDef; 1] = [AbilityCostDef::Mana(mana_cost!("{7}"))];

static KALDRA_COMPLEAT_ABILITIES: [AbilityDef; 4] = [
    abilities::living_weapon(),
    abilities::indestructible(),
    AbilityDef::static_ability(
        "Equipped creature gets +5/+5 and has first strike, trample, indestructible, haste, and \
         \"Whenever this creature deals combat damage to a creature, exile that creature.\"",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::AttachedPermanent,
            effect: AppliedEffectDef::Composite(&KALDRA_GRANTS),
        },
    ),
    abilities::equip(&KALDRA_EQUIP_COST, "Equip {7}"),
];

pub(in crate::card::sets) static KALDRA_COMPLEAT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9b6c6ad4-d5fb-4503-8b15-c2104f125990"),
    "Kaldra Compleat",
    CardArt::new("87cc2855-6b14-44dd-a398-7dc2bbae081f", "Vincent Proce"),
    CardSet::ModernHorizons2,
    // Seven mana that arrives as a 5/5 first-striking, trampling,
    // indestructible, hasty creature which exiles whatever blocks it. The
    // Germ is the point: it never needs a creature to equip.
    CardRules::new_artifact(mana_cost!("{7}"))
        .with_supertype(CardSupertype::Legendary)
        .with_subtypes(&["Equipment"])
        .with_abilities(&KALDRA_COMPLEAT_ABILITIES),
);

// MH2 231 — Nettlecyst
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

pub(in crate::card::sets) static NETTLECYST: CardRecord = CardRecord::new_with_legacy_id(
    2126,
    "Nettlecyst",
    CardArt::new("4a0bb5dc-75a6-4bd6-81f8-611197fb0fba", "Vincent Proce"),
    CardSet::ModernHorizons2,
    CardRules::new_artifact(mana_cost!("{3}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::living_weapon(),
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

// MH2 355 — Ignoble Hierarch
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static IGNOBLE_HIERARCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("404f83fb-0090-49d5-a4d0-c963adac2fb2"),
    "Ignoble Hierarch",
    crate::card::CardArt::new("3139cce8-3467-4c50-add2-5b78fb33b90a", "Mark Zug"),
    crate::card::CardSet::ModernHorizons2,
    crate::card::CardRules::unsupported(),
);

// MH2 380 — Urza's Saga
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static URZA_S_SAGA: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("1cf96437-0943-40f9-b175-31a1504028ba"),
    "Urza's Saga",
    crate::card::CardArt::new("2138dfbb-a4e3-49db-b908-95d0b2b7e82f", "Titus Lunter"),
    crate::card::CardSet::ModernHorizons2,
    crate::card::CardRules::unsupported(),
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

// MH2 450 — Dauthi Voidwalker
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DAUTHI_VOIDWALKER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("9b7029b0-cd20-4970-9355-a27611b817bc"),
    "Dauthi Voidwalker",
    crate::card::CardArt::new(
        "29632951-3c3d-478c-8c5a-9a34f30a5c28",
        "Sidharth Chaturvedi",
    ),
    crate::card::CardSet::ModernHorizons2,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &PRISMATIC_ENDING,
    &SOLITUDE,
    &UNBOUNDED_POTENTIAL,
    &HARD_EVIDENCE,
    &LOSE_FOCUS,
    &SUBTLETY,
    &ARCHON_OF_CRUELTY,
    &BONE_SHARDS,
    &DAMN,
    &GRIEF,
    &LOATHSOME_CURATOR,
    &NESTED_SHAMBLER,
    &VERMIN_GORGER,
    &DRAGON_S_RAGE_CHANNELER,
    &FURY,
    &MINE_COLLAPSE,
    &RAGAVAN_NIMBLE_PILFERER,
    &UNHOLY_HEAT,
    &ABUNDANT_HARVEST,
    &BANNERHIDE_KRUSHOK,
    &ENDURANCE,
    &URBAN_DAGGERTOOTH,
    &CAPTURED_BY_LAGACS,
    &GRIST_THE_HUNGER_TIDE,
    &TERRITORIAL_KAVU,
    &KALDRA_COMPLEAT,
    &NETTLECYST,
    &YAVIMAYA_CRADLE_OF_GROWTH,
    &IGNOBLE_HIERARCH,
    &URZA_S_SAGA,
    &GOBLIN_ANARCHOMANCER,
    &DAUTHI_VOIDWALKER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
