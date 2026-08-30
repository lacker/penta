//! C20 card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, CardArt, CardRules, CardSet, CardType, ChoiceVisibilityDef, ChooseDef, EffectDef,
    EffectRecipientDef, ObjectChoiceBindingDef, ObjectPredicateDef, ObjectRefDef, ObjectSetDef,
    ObjectSetFilterDef, PlayerRefDef, TriggerEventDef, ZoneKind, ZonePlacement, abilities,
};
use crate::{ObjectBindingIndex, mana_cost};

// C20 34 — Ethereal Forager
pub(in crate::card::sets) static ETHEREAL_FORAGER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("97543d69-547e-41f8-9a4f-908e5eb0ee4a"),
    "Ethereal Forager",
    CardArt::new("97543d69-547e-41f8-9a4f-908e5eb0ee4a", "Nicholas Gregory"),
    CardSet::Commander2020,
    CardRules::new_creature(mana_cost!("{4}{U}{U}"), &["Elemental", "Whale"], 3, 3)
        .with_abilities(&[
            abilities::delve(),
            abilities::flying(),
            AbilityDef::triggered(
                "Whenever this creature attacks, you may return an instant or sorcery card exiled with it to its owner's hand.",
                TriggerEventDef::attacks(ObjectPredicateDef::Source),
                EffectDef::May {
                    player: EffectRecipientDef::Controller,
                    effect: &EffectDef::Choose(ChooseDef {
                        binding: ObjectChoiceBindingDef::Object(ObjectBindingIndex::PRIMARY),
                        unchosen: None,
                        chooser: PlayerRefDef::EffectController,
                        candidates: ObjectSetDef::Matching {
                            objects: &ObjectSetDef::LinkedExiles,
                            object: ObjectSetFilterDef::Predicate(&ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::HasType(CardType::Instant),
                                ObjectPredicateDef::HasType(CardType::Sorcery),
                            ])),
                        },
                        exclude: None,
                        minimum: 1,
                        maximum: 1,
                        visibility: ChoiceVisibilityDef::Public,
                        then: &EffectDef::MoveToZone {
                            object: EffectRecipientDef::object(ObjectRefDef::Binding(
                                ObjectBindingIndex::PRIMARY,
                            )),
                            zone: ZoneKind::Hand,
                            placement: ZonePlacement::Top,
                        },
                    }),
                },
            ),
        ]),
);

// C20 67 — Bonder's Ornament
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static BONDER_S_ORNAMENT: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5afe425c-50a7-4d29-ac14-0edb094fc770"),
    "Bonder's Ornament",
    crate::card::CardArt::new("5afe425c-50a7-4d29-ac14-0edb094fc770", "Lindsey Look"),
    crate::card::CardSet::Commander2020,
    crate::card::CardRules::unsupported(),
);

// C20 118 — Murmuring Mystic
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static MURMURING_MYSTIC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5fc6adff-dcb3-456d-a8c2-0e77b784ff89"),
    "Murmuring Mystic",
    crate::card::CardArt::new("ab25853c-29d3-4244-88db-813300a262a5", "Mark Winters"),
    crate::card::CardSet::Commander2020,
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&ETHEREAL_FORAGER, &BONDER_S_ORNAMENT, &MURMURING_MYSTIC];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
