//! Khans of Tarkir cards cataloged as cross-format rules-engine test cases.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, CardArt, CardRules, CardSet, CardSupertype, EffectDef, EffectRecipientDef,
    PlayerRelation, ReplacementEffectDef, ReplacementEventDef, TurnKindDef, ZoneKind,
    ZoneMoveCauseDef,
};
use crate::mana_cost;

static TAKE_EXTRA_TURN_CONTROLLER: EffectDef = EffectDef::TakeExtraTurn {
    player: EffectRecipientDef::Controller,
};

// KTK 227 — Ugin's Nexus
pub(in crate::card::sets) static UGINS_NEXUS: CardRecord = CardRecord::new_with_legacy_id(
    1368,
    "Ugin's Nexus",
    CardArt::new("94002868-a48a-4ea8-bfce-17257078f5db", "Sam Burley"),
    CardSet::KhansOfTarkir,
    CardRules::new_artifact(mana_cost!("{5}"))
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            AbilityDef::replacement_for(
                "If a player would begin an extra turn, that player skips that turn instead.",
                ReplacementEventDef::WouldBeginTurn {
                    player: PlayerRelation::Any,
                    kind: TurnKindDef::Extra,
                },
                ReplacementEffectDef::ReplaceEventWithNothing,
            ),
            AbilityDef::replacement_for(
                "If Ugin's Nexus would be put into a graveyard from the battlefield, instead exile it and take an extra turn after this one.",
                ReplacementEventDef::WouldMove {
                    from: Some(ZoneKind::Battlefield),
                    to: ZoneKind::Graveyard,
                    cause: ZoneMoveCauseDef::Any,
                },
                ReplacementEffectDef::Sequence(&[
                    ReplacementEffectDef::MoveToZone(ZoneKind::Exile),
                    ReplacementEffectDef::Perform(&TAKE_EXTRA_TURN_CONTROLLER),
                ]),
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&UGINS_NEXUS];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
