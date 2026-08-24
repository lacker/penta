//! Shards of Alara cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AppliedEffectDef,
    CardArt, CardRules, CardSet, CardSupertype, CardType, ChoiceVisibilityDef, ChooseDef,
    EffectDef, EffectRecipientDef, ManaColor, ObjectChoiceBindingDef, ObjectPredicateDef,
    ObjectQueryDef, ObjectRefDef, ObjectSetDef, PlayerRefDef, PlayerRelation, PlayerSetDef,
    ResolvedEffectDurationDef, TriggerEventDef, ValueDef, ZoneKind, abilities, tokens,
};
use crate::ids::ObjectBindingIndex;
use crate::{TargetIndex, mana_cost};

// ALA 9 — Elspeth, Knight-Errant
/// The four types the emblem names, which between them are every permanent
/// a white deck is likely to control. Written as one alternation because the
/// emblem grants one thing to all of them.
static A_PERMANENT_THE_EMBLEM_PROTECTS: ObjectPredicateDef = ObjectPredicateDef::AnyOf(&[
    ObjectPredicateDef::HasType(CardType::Artifact),
    ObjectPredicateDef::HasType(CardType::Creature),
    ObjectPredicateDef::HasType(CardType::Enchantment),
    ObjectPredicateDef::HasType(CardType::Land),
]);

static ELSPETH_EMBLEM_INDESTRUCTIBLE: AbilityDef = abilities::indestructible();

static ELSPETH_EMBLEM_ABILITIES: [AbilityDef; 1] = [AbilityDef::static_ability(
    "Artifacts, creatures, enchantments, and lands you control have indestructible.",
    EffectDef::StaticApply {
        recipient: EffectRecipientDef::matching_objects(
            A_PERMANENT_THE_EMBLEM_PROTECTS,
            &[ZoneKind::Battlefield],
            PlayerRelation::You,
        ),
        effect: AppliedEffectDef::add_ability(&ELSPETH_EMBLEM_INDESTRUCTIBLE),
    },
)];

static ELSPETH_FLYING: AbilityDef = abilities::flying();

static ELSPETH_PUMP: [AppliedEffectDef; 2] = [
    AppliedEffectDef::modify_power_toughness(ValueDef::Constant(3), ValueDef::Constant(3)),
    AppliedEffectDef::add_ability(&ELSPETH_FLYING),
];

static ELSPETH_PUMP_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: None,
        owner: None,
    },
)];

static ELSPETH_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::activated(
        "+1: Create a 1/1 white Soldier creature token.",
        &[AbilityCostDef::Loyalty(1)],
        EffectDef::CreateToken {
            token: tokens::creature(&["Soldier"], &[ManaColor::White], 1, 1),
            controller: None,
            count: ValueDef::Constant(1),
            tapped: false,
            attacking: false,
            counters: None,
            created: None,
        },
    ),
    // The second plus is what makes her a threat rather than a hedge: any
    // creature, so the token she made last turn is a 4/4 flier this one.
    AbilityDef::activated_with_targets(
        "+1: Target creature gets +3/+3 and gains flying until end of turn.",
        &[AbilityCostDef::Loyalty(1)],
        &ELSPETH_PUMP_TARGET,
        EffectDef::Apply {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            effect: AppliedEffectDef::Composite(&ELSPETH_PUMP),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    ),
    AbilityDef::activated(
        "\u{2212}8: You get an emblem with \"Artifacts, creatures, enchantments, and lands you \
         control have indestructible.\"",
        &[AbilityCostDef::Loyalty(-8)],
        EffectDef::create_emblem("Elspeth, Knight-Errant emblem", &ELSPETH_EMBLEM_ABILITIES),
    ),
];

pub(in crate::card::sets) static ELSPETH_KNIGHT_ERRANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("44c52e52-2b1c-4ca8-ab6d-20d97a342704"),
    "Elspeth, Knight-Errant",
    CardArt::new("44c52e52-2b1c-4ca8-ab6d-20d97a342704", "Volkan Ba\u{11f}a"),
    CardSet::ShardsOfAlara,
    // Four mana, two plus abilities, and neither of them is the safe one:
    // she makes a blocker or she makes an attacker, and the ultimate ends
    // the game against anything that answers permanents.
    CardRules::new_planeswalker(mana_cost!("{2}{W}{W}"), &["Elspeth"], 4)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&ELSPETH_ABILITIES),
);

// ALA 104 — Hissing Iguanar
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static HISSING_IGUANAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4b8b8b90-cb6e-4910-bc40-d96b78b0d70c"),
    "Hissing Iguanar",
    crate::card::CardArt::new("4b8b8b90-cb6e-4910-bc40-d96b78b0d70c", "Brandon Kitkouski"),
    crate::card::CardSet::ShardsOfAlara,
    crate::card::CardRules::unsupported(),
);

// ALA 156 — Blightning
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BLIGHTNING: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3c05e8a2-b7d0-4f24-b2ae-8e4db30e5842"),
    "Blightning",
    crate::card::CardArt::new("3c05e8a2-b7d0-4f24-b2ae-8e4db30e5842", "Thomas M. Baxa"),
    crate::card::CardSet::ShardsOfAlara,
    crate::card::CardRules::unsupported(),
);

// ALA 158 — Branching Bolt
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BRANCHING_BOLT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("e7468876-f401-4a75-81c0-bed09cdda3e1"),
    "Branching Bolt",
    crate::card::CardArt::new("e7468876-f401-4a75-81c0-bed09cdda3e1", "Vance Kovacs"),
    crate::card::CardSet::ShardsOfAlara,
    crate::card::CardRules::unsupported(),
);

// ALA 202 — Tidehollow Sculler
/// Linked to the Sculler rather than exiled outright, which is the whole
/// bargain: the card is gone only for as long as the body survives.
static SCULLER_EXILE: EffectDef = EffectDef::ExileLinkedToSource {
    object: EffectRecipientDef::object(ObjectRefDef::Binding(ObjectBindingIndex::PRIMARY)),
};

static SCULLER_TAKES_A_CARD: [EffectDef; 2] = [
    EffectDef::LookAtHand {
        player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
    EffectDef::Choose(ChooseDef {
        binding: ObjectChoiceBindingDef::Object(ObjectBindingIndex::PRIMARY),
        unchosen: None,
        chooser: PlayerRefDef::EffectController,
        candidates: ObjectSetDef::Query(ObjectQueryDef::owned_by(
            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
            &[ZoneKind::Hand],
            PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
        )),
        exclude: None,
        minimum: 1,
        maximum: 1,
        visibility: ChoiceVisibilityDef::Public,
        then: &SCULLER_EXILE,
    }),
];

static SCULLER_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
)];

static TIDEHOLLOW_SCULLER_ABILITIES: [AbilityDef; 2] = [
    abilities::enters_trigger_with_targets(
        "When this creature enters, target opponent reveals their hand and you choose a nonland card from it. Exile that card.",
        &SCULLER_TARGET,
        EffectDef::Sequence(&SCULLER_TAKES_A_CARD),
    ),
    // Leaves, not dies: bouncing or exiling the Sculler gives the card back
    // just as killing it does.
    AbilityDef::triggered(
        "When this creature leaves the battlefield, return the exiled card to its owner's hand.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::Source,
            Some(ZoneKind::Battlefield),
            None,
        ),
        EffectDef::ReturnLinkedExiles {
            object: ObjectPredicateDef::Any,
            counters: None,
            arrival_effect: None,
            zone: ZoneKind::Hand,
            grant: None,
            controller: None,
            transformed: false,
        },
    ),
];

pub(in crate::card::sets) static TIDEHOLLOW_SCULLER: CardRecord = CardRecord::new_with_legacy_id(
    2145,
    "Tidehollow Sculler",
    CardArt::new("1abecc77-07f2-43e4-8585-0a8199cdcf01", "rk post"),
    CardSet::ShardsOfAlara,
    CardRules::new_artifact_creature(mana_cost!("{W}{B}"), &["Zombie"], 2, 2)
        .with_abilities(&TIDEHOLLOW_SCULLER_ABILITIES),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &ELSPETH_KNIGHT_ERRANT,
    &HISSING_IGUANAR,
    &BLIGHTNING,
    &BRANCHING_BOLT,
    &TIDEHOLLOW_SCULLER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
