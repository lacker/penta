//! Mirage cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, CardArt, CardRules, CardSet, CardType, EffectDef, EffectRecipientDef,
    ObjectPredicateDef, PlayerRelation, ZoneKind, ZonePlacement, cards,
};
use crate::mana_cost;

// MIR 14 — Enlightened Tutor
pub(in crate::card::sets) static ENLIGHTENED_TUTOR: CardRecord = CardRecord::new(
    cards::ENLIGHTENED_TUTOR,
    "Enlightened Tutor",
    CardArt::new("cbac1d27-15e2-4e2f-82ab-625a16e096cb", "Dan Frazier"),
    CardSet::Mirage,
    CardRules::new_instant(mana_cost!("{W}")).with_ability(AbilityDef::spell(
        "Search your library for an artifact or enchantment card, reveal it, then shuffle and put that card on top.",
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::AnyOf(&[
                ObjectPredicateDef::HasType(CardType::Artifact),
                ObjectPredicateDef::HasType(CardType::Enchantment),
            ]),
            minimum: 0,
            maximum: 1,
            reveal: true,
            destination: ZoneKind::Library,
            placement: ZonePlacement::Top,
            shuffle: true,
        },
    )),
);

// MIR 245 — Tranquil Domain
pub(in crate::card::sets) static TRANQUIL_DOMAIN: CardRecord = CardRecord::new(
    cards::TRANQUIL_DOMAIN,
    "Tranquil Domain",
    CardArt::new(
        "801f34a6-9f22-43c2-b1e5-194395cc7da1",
        "D. Alexander Gregory",
    ),
    CardSet::Mirage,
    CardRules::new_instant(mana_cost!("{1}{G}")).with_ability(AbilityDef::spell(
        "Destroy all non-Aura enchantments.",
        EffectDef::Destroy {
            object: EffectRecipientDef::MatchingObjects {
                object: ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Enchantment),
                    ObjectPredicateDef::Not(&ObjectPredicateDef::Subtype("Aura")),
                ]),
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::Any,
            },
            can_regenerate: true,
        },
    )),
);

// MIR 255 — Worldly Tutor
pub(in crate::card::sets) static WORLDLY_TUTOR: CardRecord = CardRecord::new(
    cards::WORLDLY_TUTOR,
    "Worldly Tutor",
    CardArt::new("f00115bc-b551-4bf5-a121-bebb37201575", "David O'Connor"),
    CardSet::Mirage,
    CardRules::new_instant(mana_cost!("{G}")).with_ability(AbilityDef::spell(
        "Search your library for a creature card, reveal it, then shuffle and put the card on top.",
        EffectDef::SearchZone {
            player: EffectRecipientDef::Controller,
            source: ZoneKind::Library,
            object: ObjectPredicateDef::HasType(CardType::Creature),
            minimum: 0,
            maximum: 1,
            reveal: true,
            destination: ZoneKind::Library,
            placement: ZonePlacement::Top,
            shuffle: true,
        },
    )),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&ENLIGHTENED_TUTOR, &TRANQUIL_DOMAIN, &WORLDLY_TUTOR];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
