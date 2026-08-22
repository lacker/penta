//! Conspiracy: Take the Crown cards cataloged for the Vintage Cube pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, CardArt, CardRules, CardSet, CardType, EffectDef,
    EffectRecipientDef, InstalledTriggerDef, ObjectPredicateDef, PlayerRefDef, PlayerRelation,
    TriggerEventDef, ZoneKind,
};
use crate::{TargetIndex, mana_cost};

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
    },
    EffectDef::InstallTrigger(InstalledTriggerDef::once(&JAILER_RELEASE)),
];

static JAILER_ABILITIES: [AbilityDef; 2] = [
    AbilityDef::triggered(
        "When this creature enters, you become the monarch.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::Source,
            None,
            Some(ZoneKind::Battlefield),
        ),
        EffectDef::BecomeMonarch {
            player: PlayerRefDef::EffectController,
        },
    ),
    AbilityDef::triggered_with_targets(
        "When this creature enters, exile target creature an opponent controls until an opponent becomes the monarch.",
        TriggerEventDef::zone_changed(
            ObjectPredicateDef::Source,
            None,
            Some(ZoneKind::Battlefield),
        ),
        &JAILER_TARGET,
        EffectDef::Sequence(&JAILER_JAILS),
    ),
];

// CN2 18 — Palace Jailer
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

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&PALACE_JAILER];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
