//! Adventures in the Forgotten Realms cards cataloged for the Vintage Cube
//! pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, CardArt, CardRules, CardSet, CardType,
    EffectDef, EffectRecipientDef, InstalledTriggerDef, ObjectPredicateDef, PlayerRelation,
    TriggerEventDef, ZoneKind, abilities,
};
use crate::{TargetIndex, mana_cost};

// AFR 33 — Portable Hole
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
        then: None,
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

pub(in crate::card::sets) static PORTABLE_HOLE: CardRecord = CardRecord::new_with_legacy_id(
    2256,
    "Portable Hole",
    CardArt::new("80fca8c0-ae3e-439e-b202-228b9f360e9a", "John Stanko"),
    CardSet::AdventuresInTheForgottenRealms,
    // One white mana answers most of what a fast deck opens on, and it
    // answers it at instant speed on the other player's turn only because
    // somebody flashed it in -- otherwise the Hole is simply the cheapest
    // unconditional removal a white deck gets.
    CardRules::new_artifact(mana_cost!("{W}")).with_ability(
        abilities::enters_trigger_with_targets(
            "When this artifact enters, exile target nonland permanent an opponent controls with \
         mana value 2 or less until this artifact leaves the battlefield.",
            &HOLE_TARGET,
            EffectDef::Sequence(&HOLE_SWALLOWS_IT),
        ),
    ),
);

// AFR 42 — You Hear Something on Watch
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static YOU_HEAR_SOMETHING_ON_WATCH: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("76e939ab-9d0c-4685-805c-c8bc4e6af163"),
    "You Hear Something on Watch",
    crate::card::CardArt::new("76e939ab-9d0c-4685-805c-c8bc4e6af163", "Zezhou Chen"),
    crate::card::CardSet::AdventuresInTheForgottenRealms,
    crate::card::CardRules::unsupported(),
);

// AFR 198 — Owlbear
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static OWLBEAR: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("12b19309-a7f6-44da-b856-d12da11156e8"),
    "Owlbear",
    crate::card::CardArt::new("30e8a00f-8131-470d-8072-4c23b812281a", "Ilse Gort"),
    crate::card::CardSet::AdventuresInTheForgottenRealms,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&PORTABLE_HOLE, &YOU_HEAR_SOMETHING_ON_WATCH, &OWLBEAR];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
