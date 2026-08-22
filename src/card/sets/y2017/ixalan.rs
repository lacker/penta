//! Ixalan cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, CardArt, CardRules, CardSet, CardType,
    ChoiceVisibilityDef, ChooseDef, EffectDef, EffectRecipientDef, InstalledTriggerDef,
    ObjectChoiceBindingDef, ObjectPredicateDef, ObjectQueryDef, ObjectRefDef, ObjectSetDef,
    PlayerRefDef, PlayerRelation, PlayerSetDef, TriggerEventDef, ZoneKind, abilities,
};
use crate::ids::ObjectBindingIndex;
use crate::{TargetIndex, mana_cost};

/// "Until this creature leaves the battlefield" is one printed ability, so
/// the return is a delayed trigger installed by the same resolution rather
/// than a second clause the card does not print.
static FREEBOOTER_RETURNS_IT: AbilityDef = AbilityDef::triggered(
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
);

static FREEBOOTER_EXILE: [EffectDef; 2] = [
    EffectDef::ExileLinkedToSource {
        object: EffectRecipientDef::object(ObjectRefDef::Binding(ObjectBindingIndex::PRIMARY)),
    },
    EffectDef::InstallTrigger(InstalledTriggerDef::once(&FREEBOOTER_RETURNS_IT)),
];

/// Neither a creature nor a land: the Freebooter takes the answer, not the
/// threat, which is what separates it from the Sculler.
static A_NONCREATURE_NONLAND_CARD: ObjectPredicateDef = ObjectPredicateDef::All(&[
    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Creature)),
    ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
]);

static FREEBOOTER_TAKES_A_CARD: [EffectDef; 2] = [
    EffectDef::LookAtHand {
        player: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
    EffectDef::Choose(ChooseDef {
        binding: ObjectChoiceBindingDef::Object(ObjectBindingIndex::PRIMARY),
        unchosen: None,
        chooser: PlayerRefDef::EffectController,
        candidates: ObjectSetDef::Query(ObjectQueryDef::owned_by(
            A_NONCREATURE_NONLAND_CARD,
            &[ZoneKind::Hand],
            PlayerSetDef::One(PlayerRefDef::Target(TargetIndex::PRIMARY)),
        )),
        exclude: None,
        minimum: 1,
        maximum: 1,
        visibility: ChoiceVisibilityDef::Public,
        then: &EffectDef::Sequence(&FREEBOOTER_EXILE),
    }),
];

static FREEBOOTER_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Player(PlayerRelation::Opponent),
)];

static KITESAIL_FREEBOOTER_ABILITIES: [AbilityDef; 2] = [
    abilities::flying(),
    AbilityDef::triggered_with_targets(
        "When this creature enters, target opponent reveals their hand. You choose a noncreature, nonland card from it. Exile that card until this creature leaves the battlefield.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::Source,
            None,
            Some(ZoneKind::Battlefield),
        ),
        &FREEBOOTER_TARGET,
        EffectDef::Sequence(&FREEBOOTER_TAKES_A_CARD),
    ),
];

// XLN 110 — Kitesail Freebooter
pub(in crate::card::sets) static KITESAIL_FREEBOOTER: CardRecord = CardRecord::new_with_legacy_id(
    2149,
    "Kitesail Freebooter",
    CardArt::new("f62fd592-4910-417d-a500-e7029f3d119f", "Dan Murayama Scott"),
    CardSet::Ixalan,
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Pirate"], 1, 2)
        .with_abilities(&KITESAIL_FREEBOOTER_ABILITIES),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&KITESAIL_FREEBOOTER];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
