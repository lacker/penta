//! Khans of Tarkir cards cataloged as cross-format rules-engine test cases.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, CardArt, CardRules, CardSet, CardSupertype, EffectDef, EffectRecipientDef,
    PlayerRelation, ReplacementEffectDef, ReplacementEventDef, TurnKindDef, ValueDef, ZoneKind,
    ZoneMoveCauseDef, abilities,
};
use crate::mana_cost;

static TAKE_EXTRA_TURN_CONTROLLER: EffectDef = EffectDef::TakeExtraTurn {
    player: EffectRecipientDef::Controller,
};

// KTK 59 — Treasure Cruise
pub(in crate::card::sets) static TREASURE_CRUISE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("7a59d4b1-6cf4-44ec-8a96-1bb7094fea21"),
    "Treasure Cruise",
    CardArt::new("7a59d4b1-6cf4-44ec-8a96-1bb7094fea21", "Cynthia Sheppard"),
    CardSet::KhansOfTarkir,
    CardRules::new_sorcery(mana_cost!("{7}{U}")).with_abilities(&[
        abilities::delve(),
        AbilityDef::spell(
            "Draw three cards.",
            EffectDef::DrawCards {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(3),
            },
        ),
    ]),
);

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

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&TREASURE_CRUISE, &UGINS_NEXUS];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
