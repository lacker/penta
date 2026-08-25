//! Conspiracy: Take the Crown cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, CardArt, CardRules, CardSet, CardType, EffectDef,
    EffectRecipientDef, InstalledTriggerDef, ObjectPredicateDef, PlayerRefDef, PlayerRelation,
    TriggerEventDef, ZoneKind, abilities,
};
use crate::{TargetIndex, mana_cost};

// CN2 18 — Palace Jailer
static JAILER_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
    ObjectPredicateDef::All(&[
        ObjectPredicateDef::HasType(CardType::Creature),
        ObjectPredicateDef::ControlledBy(PlayerRelation::Opponent),
    ]),
)];

/// The release. It listens from outside every zone, so a Jailer that has
/// already died still gives the creature back the moment the crown changes
/// hands -- and if it never does, the creature never comes back.
static JAILER_RELEASE: AbilityDef = AbilityDef::triggered(
    "When an opponent becomes the monarch, return the exiled card to the battlefield.",
    TriggerEventDef::BecomesMonarch(PlayerRelation::Opponent),
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

/// Exiling and arming the release are one clause: the card is linked to the
/// Jailer, and the delayed trigger is what "until" means.
static JAILER_JAILS: [EffectDef; 2] = [
    EffectDef::ExileLinkedToSource {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
        then: None,
    },
    EffectDef::InstallTrigger(InstalledTriggerDef::once(&JAILER_RELEASE)),
];

static JAILER_ABILITIES: [AbilityDef; 2] = [
    abilities::enters_trigger(
        "When this creature enters, you become the monarch.",
        EffectDef::BecomeMonarch {
            player: PlayerRefDef::EffectController,
        },
    ),
    abilities::enters_trigger_with_targets(
        "When this creature enters, exile target creature an opponent controls until an opponent becomes the monarch.",
        &JAILER_TARGET,
        EffectDef::Sequence(&JAILER_JAILS),
    ),
];

pub(in crate::card::sets) static PALACE_JAILER: CardRecord = CardRecord::new_with_legacy_id(
    2171,
    "Palace Jailer",
    CardArt::new("78cef262-c753-4658-b3ec-fec8db47f944", "David Palumbo"),
    CardSet::ConspiracyTakeTheCrown,
    // The crown is the card: a removal spell that also draws every turn, for
    // as long as nobody can get through to take it back.
    CardRules::new_creature(mana_cost!("{2}{W}{W}"), &["Human", "Soldier"], 2, 2)
        .with_abilities(&JAILER_ABILITIES),
);

// CN2 19 — Palace Sentinels
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static PALACE_SENTINELS: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3e002a99-eb2b-4cc3-992e-f3ee42245dba"),
    "Palace Sentinels",
    crate::card::CardArt::new("3e002a99-eb2b-4cc3-992e-f3ee42245dba", "Aaron Miller"),
    crate::card::CardSet::ConspiracyTakeTheCrown,
    crate::card::CardRules::unsupported(),
);

// CN2 48 — Thorn of the Black Rose
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static THORN_OF_THE_BLACK_ROSE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("2e4829c6-50d4-4602-af78-59249486a97c"),
    "Thorn of the Black Rose",
    crate::card::CardArt::new("2e4829c6-50d4-4602-af78-59249486a97c", "David Gaillet"),
    crate::card::CardSet::ConspiracyTakeTheCrown,
    crate::card::CardRules::unsupported(),
);

// CN2 64 — Entourage of Trest
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static ENTOURAGE_OF_TREST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("3d7ee777-6113-43f8-883e-c7569eefb872"),
    "Entourage of Trest",
    crate::card::CardArt::new("3d7ee777-6113-43f8-883e-c7569eefb872", "Anthony Palumbo"),
    crate::card::CardSet::ConspiracyTakeTheCrown,
    crate::card::CardRules::unsupported(),
);

// CN2 77 — Leovold, Emissary of Trest
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static LEOVOLD_EMISSARY_OF_TREST: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("49bb0ad3-1082-41f1-82a4-52a4006cc9b6"),
    "Leovold, Emissary of Trest",
    crate::card::CardArt::new("49bb0ad3-1082-41f1-82a4-52a4006cc9b6", "Magali Villeneuve"),
    crate::card::CardSet::ConspiracyTakeTheCrown,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &PALACE_JAILER,
    &PALACE_SENTINELS,
    &THORN_OF_THE_BLACK_ROSE,
    &ENTOURAGE_OF_TREST,
    &LEOVOLD_EMISSARY_OF_TREST,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
