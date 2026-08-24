//! Ravnica: City of Guilds cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, CardArt, CardRules, CardSet, EffectDef, EffectRecipientDef, PlayerRelation,
    TopCardSelectionDef, TriggerEventDef, TurnStepDef, ValueDef, ZoneKind, ZonePlacement,
};
use crate::mana_cost;

// RAV 81 — Dark Confidant
/// "You lose life equal to its mana value." The card is in your hand by the
/// time this is asked, so what the reveal hands on is the number rather than
/// the card.
static CONFIDANT_PAYMENT: EffectDef = EffectDef::LoseLife {
    recipient: EffectRecipientDef::Controller,
    amount: ValueDef::MatchedManaValue,
};

/// One card off the top, shown to everybody, into your hand. Nothing is
/// chosen and nothing may be declined: the minimum and the maximum are both
/// the one card the trigger names.
static CONFIDANT_REVEAL: TopCardSelectionDef = TopCardSelectionDef {
    count: ValueDef::Constant(1),
    object: None,
    minimum: 1,
    maximum: 1,
    select_all_matching: true,
    reveal_selected: true,
    selected_zone: ZoneKind::Hand,
    selected_placement: ZonePlacement::Top,
    selected_hidden: false,
    selected_linked_to_source: false,
    selected_face_down: None,
    rest_zone: ZoneKind::Library,
    rest_placement: ZonePlacement::Top,
    rest_random_order: false,
    rest_counters: None,
    selected_order_follows_choice: false,
    then: Some(&CONFIDANT_PAYMENT),
};

pub(in crate::card::sets) static DARK_CONFIDANT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("94f7a441-bf2d-46fb-a7b6-9bd6137f86d9"),
    "Dark Confidant",
    CardArt::new("94f7a441-bf2d-46fb-a7b6-9bd6137f86d9", "Ron Spears"),
    CardSet::RavnicaCityOfGuilds,
    // Two mana for an extra card every turn, at whatever the top of your
    // deck happens to cost -- which is why the decks that play him keep
    // their curve low enough to survive him.
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Wizard"], 2, 1).with_ability(
        AbilityDef::triggered(
            "At the beginning of your upkeep, reveal the top card of your library and put that \
             card into your hand. You lose life equal to its mana value.",
            TriggerEventDef::StepBegins {
                step: TurnStepDef::Upkeep,
                player: PlayerRelation::You,
            },
            EffectDef::LookAtTopAndSelect {
                player: EffectRecipientDef::Controller,
                looker: EffectRecipientDef::Controller,
                selection: &CONFIDANT_REVEAL,
            },
        ),
    ),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&DARK_CONFIDANT];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
