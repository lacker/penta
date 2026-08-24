//! Streets of New Capenna Commander cards cataloged for the Vintage Cube
//! pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityCostDef, AbilityDef, CardArt, CardRules, CardSet, CardType, ChoiceVisibilityDef,
    ChooseDef, DiscardSelectionDef, EffectDef, EffectRecipientDef, ManaColor,
    ObjectChoiceBindingDef, ObjectPredicateDef, ObjectRefDef, ObjectSetDef, PlayerRefDef,
    PlayerRelation, TriggerConditionDef, TriggerEventDef, ValueDef, ZoneKind, ZonePlacement,
    tokens,
};
use crate::ids::ObjectBindingIndex;
use crate::mana_cost;

// NCC 81 — Currency Converter
/// "That card", which is the card as it now lies in the graveyard: the
/// discard is over by the time this resolves, so what the trigger points at
/// is the graveyard object rather than the one that was in hand.
static EXILE_THE_DISCARDED_CARD: EffectDef = EffectDef::ExileLinkedToSource {
    object: EffectRecipientDef::TriggeringObject,
};

static CONVERTER_LOOT_COST: [AbilityCostDef; 2] = [
    AbilityCostDef::Mana(mana_cost!("{2}")),
    AbilityCostDef::TapSource,
];

static CONVERTER_LOOT: [EffectDef; 2] = [
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
];

static CONVERTER_CASH_OUT_COST: [AbilityCostDef; 1] = [AbilityCostDef::TapSource];

/// The card the cash-out chose, asked about while the choice still names it.
/// A land pays a Treasure and anything else pays a body, so the two are
/// complementary and exactly one of them runs.
static CHOSEN_IS_A_LAND: TriggerConditionDef = TriggerConditionDef::BoundObjectMatches {
    binding: ObjectBindingIndex::PRIMARY,
    object: ObjectPredicateDef::HasType(CardType::Land),
};

static CHOSEN_IS_NOT_A_LAND: TriggerConditionDef = TriggerConditionDef::Not(&CHOSEN_IS_A_LAND);

/// The card goes back to the graveyard it came from -- its owner's, which is
/// where a card exiled from a graveyard belongs however it got to exile.
static CONVERTER_RETURNS_THE_CARD: EffectDef = EffectDef::MoveToZone {
    object: EffectRecipientDef::object(ObjectRefDef::Binding(ObjectBindingIndex::PRIMARY)),
    from: None,
    zone: ZoneKind::Graveyard,
    placement: ZonePlacement::Top,
    controller: None,
    arrival_effect: None,
    attachment: None,
    counters: None,
    tapped: false,
};

static CONVERTER_PAYS_A_TREASURE: [EffectDef; 2] = [
    CONVERTER_RETURNS_THE_CARD,
    EffectDef::create_token(tokens::treasure()),
];

static CONVERTER_PAYS_A_ROGUE: [EffectDef; 2] = [
    CONVERTER_RETURNS_THE_CARD,
    EffectDef::create_token(tokens::creature(&["Rogue"], &[ManaColor::Black], 2, 2)),
];

static CONVERTER_CASHES_OUT: [EffectDef; 2] = [
    EffectDef::IfCondition {
        condition: &CHOSEN_IS_A_LAND,
        then: &EffectDef::Sequence(&CONVERTER_PAYS_A_TREASURE),
    },
    EffectDef::IfCondition {
        condition: &CHOSEN_IS_NOT_A_LAND,
        then: &EffectDef::Sequence(&CONVERTER_PAYS_A_ROGUE),
    },
];

static CURRENCY_CONVERTER_ABILITIES: [AbilityDef; 3] = [
    AbilityDef::triggered(
        "Whenever you discard a card, you may exile that card from your graveyard.",
        TriggerEventDef::Discarded(PlayerRelation::You),
        EffectDef::May {
            player: EffectRecipientDef::Controller,
            effect: &EXILE_THE_DISCARDED_CARD,
        },
    ),
    AbilityDef::activated(
        "{2}, {T}: Draw a card, then discard a card.",
        &CONVERTER_LOOT_COST,
        EffectDef::Sequence(&CONVERTER_LOOT),
    ),
    AbilityDef::activated(
        "{T}: Put a card exiled with this artifact into its owner's graveyard. If it's a land \
         card, create a Treasure token. If it's a nonland card, create a 2/2 black Rogue creature \
         token.",
        &CONVERTER_CASH_OUT_COST,
        EffectDef::Choose(ChooseDef {
            binding: ObjectChoiceBindingDef::Object(ObjectBindingIndex::PRIMARY),
            unchosen: None,
            chooser: PlayerRefDef::EffectController,
            candidates: ObjectSetDef::LinkedExiles(ObjectPredicateDef::Any),
            exclude: None,
            minimum: 1,
            maximum: 1,
            visibility: ChoiceVisibilityDef::Public,
            then: &EffectDef::Sequence(&CONVERTER_CASHES_OUT),
        }),
    ),
];

pub(in crate::card::sets) static CURRENCY_CONVERTER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("187b6719-e5ed-4615-a00b-3313ceca055b"),
    "Currency Converter",
    CardArt::new("187b6719-e5ed-4615-a00b-3313ceca055b", "Sean Murray"),
    CardSet::StreetsOfNewCapennaCommander,
    // One mana for a bank: every card you throw away is held rather than
    // spent, and later it comes back out as a Treasure or a body.
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&CURRENCY_CONVERTER_ABILITIES),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&CURRENCY_CONVERTER];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
