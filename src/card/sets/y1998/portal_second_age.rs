//! Portal Second Age cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, AbilityTargetPredicate, CardArt, CardRules, CardSet, EffectDef,
    EffectRecipientDef, TopCardSelectionDef, ValueDef, ZoneKind, ZonePlacement,
};
use crate::{TargetIndex, mana_cost};

static SLEIGHT_SELECTION: TopCardSelectionDef = TopCardSelectionDef {
    count: ValueDef::Constant(2),
    object: None,
    minimum: 1,
    maximum: 1,
    select_all_matching: false,
    reveal_selected: false,
    selected_zone: ZoneKind::Hand,
    selected_placement: ZonePlacement::Top,
    rest_zone: ZoneKind::Library,
    rest_placement: ZonePlacement::Bottom,
    selected_order_follows_choice: false,
    then: None,
    selected_face_down: None,
};

// P02 46 — Sleight of Hand
pub(in crate::card::sets) static SLEIGHT_OF_HAND: CardRecord = CardRecord::new_with_legacy_id(
    311,
    "Sleight of Hand",
    CardArt::new("f3405184-dcda-4bb6-ade6-c2a87bc3296d", "Phil Foglio"),
    CardSet::PortalSecondAge,
    CardRules::new_sorcery(mana_cost!("{U}")).with_ability(AbilityDef::spell(
        "Look at the top two cards of your library. Put one of them into your hand and the other on the bottom of your library.",
        EffectDef::LookAtTopAndSelect {
            player: EffectRecipientDef::Controller,
            looker: EffectRecipientDef::Controller,
            selection: &SLEIGHT_SELECTION,
        },
    )),
);

// P02 119 — Volcanic Hammer
pub(in crate::card::sets) static VOLCANIC_HAMMER: CardRecord = CardRecord::new_with_legacy_id(
    273,
    "Volcanic Hammer",
    CardArt::new(
        "58c0489d-b073-4ad4-b044-447fcc865b6c",
        "Edward P. Beard, Jr.",
    ),
    CardSet::PortalSecondAge,
    CardRules::new_sorcery(mana_cost!("{1}{R}")).with_ability(AbilityDef::spell_with_targets(
        "Volcanic Hammer deals 3 damage to any target.",
        &[AbilityTargetDef::exactly_one(
            AbilityTargetPredicate::AnyTarget,
        )],
        EffectDef::DealDamage {
            recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            amount: ValueDef::Constant(3),
        },
    )),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&SLEIGHT_OF_HAND, &VOLCANIC_HAMMER];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
