//! FDN card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, AppliedEffectDef, CardArt, CardRules, CardSet, CardSupertype,
    CharacteristicOperationDef, CreatureTypeSetDef, EffectDef, EffectRecipientDef,
    ExilePlayDurationDef, ObjectPredicateDef, PowerToughnessOperationDef,
    ResolvedEffectDurationDef, SetOperationDef, TriggerConditionDef, TriggerEventDef, ValueDef,
    abilities,
};
use crate::mana_cost;

// FDN 18 — Inspiring Paladin
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static INSPIRING_PALADIN: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0763be06-25b2-4d6b-ab33-a1af85aeb443"),
    "Inspiring Paladin",
    crate::card::CardArt::new("0763be06-25b2-4d6b-ab33-a1af85aeb443", "Valera Lutfullina"),
    crate::card::CardSet::MagicFoundations,
    crate::card::CardRules::unsupported(),
);

// FDN 114 — Treetop Snarespinner
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static TREETOP_SNARESPINNER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("88e68fa3-159d-49a6-8ac6-afc9bd6f1718"),
    "Treetop Snarespinner",
    crate::card::CardArt::new("88e68fa3-159d-49a6-8ac6-afc9bd6f1718", "Steve Ellis"),
    crate::card::CardSet::MagicFoundations,
    crate::card::CardRules::unsupported(),
);

static LEYLINE_AXE_BONUS: [AppliedEffectDef; 3] = [
    AppliedEffectDef::modify_power_toughness(ValueDef::Constant(1), ValueDef::Constant(1)),
    AppliedEffectDef::add_ability(&abilities::double_strike()),
    AppliedEffectDef::add_ability(&abilities::trample()),
];

// FDN 129 — Leyline Axe
pub(in crate::card::sets) static LEYLINE_AXE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("b9c03336-a321-4c06-94d1-809f328fabd8"),
    "Leyline Axe",
    CardArt::new("b9c03336-a321-4c06-94d1-809f328fabd8", "Edgar Sánchez Hidalgo"),
    CardSet::MagicFoundations,
    CardRules::new_artifact(mana_cost!("{4}"))
        .with_subtypes(&["Equipment"])
        .with_abilities(&[
            abilities::begin_game_on_battlefield("If this card is in your opening hand, you may begin the game with it on the battlefield."),
            AbilityDef::static_ability(
                "Equipped creature gets +1/+1 and has double strike and trample.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::Composite(&LEYLINE_AXE_BONUS),
                },
            ),
            abilities::equip(&[AbilityCostDef::Mana(mana_cost!("{3}"))], "Equip {3}"),
        ]),
);

// FDN 195 — Fanatical Firebrand
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static FANATICAL_FIREBRAND: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5e5565de-028c-4799-a9f6-4dcd685639eb"),
    "Fanatical Firebrand",
    crate::card::CardArt::new("d1296316-7781-4e98-95e6-7020648be6a5", "Wayne Reynolds"),
    crate::card::CardSet::MagicFoundations,
    crate::card::CardRules::unsupported(),
);

// FDN 200 — Goblin Surprise
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static GOBLIN_SURPRISE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("527dd5d4-5f72-40bb-8a9d-1f5ac3f81e2e"),
    "Goblin Surprise",
    crate::card::CardArt::new("527dd5d4-5f72-40bb-8a9d-1f5ac3f81e2e", "Kevin Sidharta"),
    crate::card::CardSet::MagicFoundations,
    crate::card::CardRules::unsupported(),
);

// FDN 330 — Kellan, Planar Trailblazer
/// The Detective's own clause, granted rather than printed: a card exiled
/// off the top and playable for the turn, which is what the second
/// activation is paying to turn on.
static KELLAN_INVESTIGATES: AbilityDef = AbilityDef::triggered(
    "Whenever Kellan deals combat damage to a player, exile the top card of your library. You may \
     play that card this turn.",
    TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source),
    EffectDef::ExileTopOfLibraryToPlay {
        player: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
        free: false,
        face_down: false,
        duration: ExilePlayDurationDef::ThisTurn,
        spend_any_color: false,
        play_condition: None,
    },
);

/// "It becomes a Human Faerie Detective": a set rather than an addition, so
/// the Scout it was is gone and the second activation has something to ask
/// about.
static KELLAN_BECOMES_A_DETECTIVE: [AppliedEffectDef; 2] = [
    AppliedEffectDef::Characteristic(CharacteristicOperationDef::CreatureTypes(
        SetOperationDef::Set(CreatureTypeSetDef::named(&["Human", "Faerie", "Detective"])),
    )),
    AppliedEffectDef::add_ability(&KELLAN_INVESTIGATES),
];

static KELLAN_DOUBLE_STRIKE: AbilityDef = abilities::double_strike();

static KELLAN_BECOMES_A_ROGUE: [AppliedEffectDef; 3] = [
    AppliedEffectDef::Characteristic(CharacteristicOperationDef::PowerToughness(
        PowerToughnessOperationDef::SetBase {
            power: ValueDef::Constant(3),
            toughness: ValueDef::Constant(2),
        },
    )),
    AppliedEffectDef::Characteristic(CharacteristicOperationDef::CreatureTypes(
        SetOperationDef::Set(CreatureTypeSetDef::named(&["Human", "Faerie", "Rogue"])),
    )),
    AppliedEffectDef::add_ability(&KELLAN_DOUBLE_STRIKE),
];

/// Each activation asks what Kellan is now, so the two have to be paid in
/// order and neither does anything twice.
static KELLAN_IS_A_SCOUT: TriggerConditionDef = TriggerConditionDef::SourceMatches {
    object: ObjectPredicateDef::Subtype("Scout"),
};

static KELLAN_IS_A_DETECTIVE: TriggerConditionDef = TriggerConditionDef::SourceMatches {
    object: ObjectPredicateDef::Subtype("Detective"),
};

static KELLAN_GROWS_UP: EffectDef = EffectDef::IfCondition {
    condition: &KELLAN_IS_A_SCOUT,
    then: &EffectDef::Apply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::Composite(&KELLAN_BECOMES_A_DETECTIVE),
        duration: ResolvedEffectDurationDef::Permanent,
    },
};

static KELLAN_GROWS_UP_AGAIN: EffectDef = EffectDef::IfCondition {
    condition: &KELLAN_IS_A_DETECTIVE,
    then: &EffectDef::Apply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::Composite(&KELLAN_BECOMES_A_ROGUE),
        duration: ResolvedEffectDurationDef::Permanent,
    },
};

static KELLAN_FIRST_STEP: [AbilityCostDef; 1] = [AbilityCostDef::Mana(mana_cost!("{1}{R}"))];
static KELLAN_SECOND_STEP: [AbilityCostDef; 1] = [AbilityCostDef::Mana(mana_cost!("{2}{R}"))];

static KELLAN_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::activated(
        "{1}{R}: If Kellan is a Scout, it becomes a Human Faerie Detective and gains \"Whenever \
         Kellan deals combat damage to a player, exile the top card of your library. You may play \
         that card this turn.\"",
        &KELLAN_FIRST_STEP,
        KELLAN_GROWS_UP,
    ),
    AbilityDef::activated(
        "{2}{R}: If Kellan is a Detective, it becomes a 3/2 Human Faerie Rogue and gains double \
         strike.",
        &KELLAN_SECOND_STEP,
        KELLAN_GROWS_UP_AGAIN,
    ),
];

pub(in crate::card::sets) static KELLAN_PLANAR_TRAILBLAZER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("0e413f37-b59a-4302-86d3-2abce81edc78"),
    "Kellan, Planar Trailblazer",
    CardArt::new("0e413f37-b59a-4302-86d3-2abce81edc78", "Aaron J. Riley"),
    CardSet::MagicFoundations,
    // One mana for a 2/1 that grows into what the rest of the turn's mana
    // has nothing better to do with.
    CardRules::new_creature(mana_cost!("{R}"), &["Human", "Faerie", "Scout"], 2, 1)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&KELLAN_ABILITIES),
);

// FDN 528 — Undying Malice
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static UNDYING_MALICE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("8eb38041-043a-4b18-9d9a-f1283684e8f1"),
    "Undying Malice",
    crate::card::CardArt::new("97b3cf11-e352-4ee1-8c03-13898f576ef9", "Igor Kieryluk"),
    crate::card::CardSet::MagicFoundations,
    crate::card::CardRules::unsupported(),
);

// FDN 596 — Shipwreck Dowser
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static SHIPWRECK_DOWSER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("59d38ef7-5017-4ea3-b97f-a8fe12d03e98"),
    "Shipwreck Dowser",
    crate::card::CardArt::new("1f20fe3d-792a-4030-a25c-e81b48b2bcb4", "Caroline Gariba"),
    crate::card::CardSet::MagicFoundations,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &INSPIRING_PALADIN,
    &TREETOP_SNARESPINNER,
    &LEYLINE_AXE,
    &FANATICAL_FIREBRAND,
    &GOBLIN_SURPRISE,
    &KELLAN_PLANAR_TRAILBLAZER,
    &UNDYING_MALICE,
    &SHIPWRECK_DOWSER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
