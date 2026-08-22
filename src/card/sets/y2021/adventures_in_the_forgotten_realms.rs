//! Adventures in the Forgotten Realms cards cataloged for the Vintage Cube
//! pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, CardArt, CardRules, CardSet, CardType,
    EffectDef, EffectRecipientDef, InstalledTriggerDef, ObjectPredicateDef, PlayerRelation,
    TriggerEventDef, ZoneKind,
};
use crate::{TargetIndex, mana_cost};

/// "Until this artifact leaves the battlefield" is one printed ability, so
/// the return rides on the same resolution as a delayed trigger rather than
/// appearing as a second clause the card does not print. Leaves, not dies:
/// bouncing or exiling the Hole gives the permanent back just as destroying
/// it does.
static HOLE_GIVES_IT_BACK: AbilityDef = AbilityDef::triggered(
    "When this artifact leaves the battlefield, return the exiled card to the battlefield under \
     its owner's control.",
    TriggerEventDef::zone_changed(
        ObjectPredicateDef::Source,
        Some(ZoneKind::Battlefield),
        None,
    ),
    EffectDef::ReturnLinkedExiles {
        object: ObjectPredicateDef::Any,
        counters: None,
        arrival_effect: None,
        zone: ZoneKind::Battlefield,
        grant: None,
        controller: None,
        transformed: false,
    },
);

static HOLE_SWALLOWS_IT: [EffectDef; 2] = [
    EffectDef::ExileLinkedToSource {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
    EffectDef::InstallTrigger(InstalledTriggerDef::once(&HOLE_GIVES_IT_BACK)),
];

/// A cheap nonland permanent across the table. Mana value is read off the
/// card, so a token is a zero and qualifies.
static HOLE_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::All(&[
            ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
            ObjectPredicateDef::ManaValueAtMost(2),
        ]),
        zones: &[ZoneKind::Battlefield],
        controller: Some(PlayerRelation::Opponent),
        owner: None,
    },
)];

// AFR 33 — Portable Hole
pub(in crate::card::sets) static PORTABLE_HOLE: CardRecord = CardRecord::new_with_legacy_id(
    2256,
    "Portable Hole",
    CardArt::new("80fca8c0-ae3e-439e-b202-228b9f360e9a", "John Stanko"),
    CardSet::AdventuresInTheForgottenRealms,
    // One white mana answers most of what a fast deck opens on, and it
    // answers it at instant speed on the other player's turn only because
    // somebody flashed it in -- otherwise the Hole is simply the cheapest
    // unconditional removal a white deck gets.
    CardRules::new_artifact(mana_cost!("{W}")).with_ability(AbilityDef::triggered_with_targets(
        "When this artifact enters, exile target nonland permanent an opponent controls with \
         mana value 2 or less until this artifact leaves the battlefield.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::Source,
            None,
            Some(ZoneKind::Battlefield),
        ),
        &HOLE_TARGET,
        EffectDef::Sequence(&HOLE_SWALLOWS_IT),
    )),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&PORTABLE_HOLE];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
