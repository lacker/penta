//! Shards of Alara cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, CardArt, CardRules, CardSet, CardType,
    ChoiceVisibilityDef, ChooseDef, EffectDef, EffectRecipientDef, ObjectChoiceBindingDef,
    ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef, PlayerRefDef, PlayerRelation,
    PlayerSetDef, TriggerEventDef, ZoneKind,
};
use crate::ids::ObjectBindingIndex;
use crate::{TargetIndex, mana_cost};

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
    AbilityDef::triggered_with_targets(
        "When this creature enters, target opponent reveals their hand and you choose a nonland card from it. Exile that card.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::Source,
            None,
            Some(ZoneKind::Battlefield),
        ),
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

// ALA 202 — Tidehollow Sculler
pub(in crate::card::sets) static TIDEHOLLOW_SCULLER: CardRecord = CardRecord::new_with_legacy_id(
    2145,
    "Tidehollow Sculler",
    CardArt::new("1abecc77-07f2-43e4-8585-0a8199cdcf01", "rk post"),
    CardSet::ShardsOfAlara,
    CardRules::new_artifact_creature(mana_cost!("{W}{B}"), &["Zombie"], 2, 2)
        .with_abilities(&TIDEHOLLOW_SCULLER_ABILITIES),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&TIDEHOLLOW_SCULLER];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
