//! SOK card records required by supported formats.

use super::{CardRecord, PrintingRecord};
use crate::AbilityDef;
use crate::CardRules;
use crate::CardSet;
use crate::CardSupertype;
use crate::CardType;
use crate::EffectDef;
use crate::EffectRecipientDef;
use crate::ObjectPredicateDef;
use crate::ValueDef;
use crate::ZoneKind;
use crate::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

// SOK 63 — Death Denied
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEATH_DENIED: CardRecord = CardRecord::new(
    crate::card::CardSet::SaviorsOfKamigawa,
    "Death Denied",
    "8f66ddc5-f5e6-44de-8189-87b6521d1fea",
    "Greg Hildebrandt",
    crate::card::CardRules::unsupported(),
);

// SOK 147 — Seek the Horizon
pub(in crate::card::sets) static SEEK_THE_HORIZON: CardRecord = CardRecord::new(
    CardSet::SaviorsOfKamigawa,
    "Seek the Horizon",
    "49f8a9e7-f505-4fc5-b820-0af1ee1960c7",
    "Eric Polak",
    CardRules::new_sorcery(mana_cost!("{3}{G}")).with_ability(AbilityDef::spell(
        "Search your library for up to three basic land reveal them, put them into your hand, then shuffle.",
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::All(&[
                ObjectPredicateDef::HasType(CardType::Land),
                ObjectPredicateDef::Supertype(CardSupertype::Basic),
            ]),
            minimum: 0,
            maximum: ValueDef::Constant(3),
            reveal: true,
            destination: ZoneKind::Hand,
            placement: ZonePlacement::Top,
            shuffle: true,
            enters_tapped: false,
            attachment: None,
            binding: None,
            then: None,
        },
    )),
);

// SOK 158 — Pithing Needle
pub(in crate::card::sets) static PITHING_NEEDLE: CardRecord = CardRecord::new(
    CardSet::SaviorsOfKamigawa,
    "Pithing Needle",
    "78eb9e1d-113e-45ff-8435-32ee42fa5631",
    "Pete Venters",
    CardRules::new_artifact(mana_cost!("{1}")).with_abilities(&[
        abilities::choose_card_name_as_enters(
            "As this artifact enters, choose a card name.",
            crate::card::BattlefieldEntryScalarChoiceDef::CARD_NAME,
        ),
        abilities::cannot_activate_nonmana_abilities_with_chosen_name(
            "Activated abilities of sources with the chosen name can't be activated unless they're mana abilities.",
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&DEATH_DENIED, &SEEK_THE_HORIZON, &PITHING_NEEDLE];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
