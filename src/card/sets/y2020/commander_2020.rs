//! C20 card records required by supported formats.

use super::CardRecord;
use super::PrintingRecord;
use crate::ParentBinding;
use crate::card::AbilityDef;
use crate::card::CardRules;
use crate::card::CardType;
use crate::card::ChoiceVisibilityDef;
use crate::card::ChooseDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ObjectChoiceBindingDef;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectRefDef;
use crate::card::ObjectSetDef;
use crate::card::ObjectSetFilterDef;
use crate::card::PlayerRefDef;
use crate::card::TriggerEventDef;
use crate::card::ZoneKind;
use crate::card::ZonePlacement;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new("C20", "commander-2020");

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// C20 34 — Ethereal Forager
pub(in crate::card::sets) static ETHEREAL_FORAGER: CardRecord = CardRecord::new(
    "Ethereal Forager",
    "97543d69-547e-41f8-9a4f-908e5eb0ee4a",
    "Nicholas Gregory",
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
                        binding: ObjectChoiceBindingDef::Object(ParentBinding),
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
                                ParentBinding,
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
// Audit: unsupported — Needs a player set filtered by what its members control. PlayerSetDef offers All, One, Related and LegalTargets, none of which can say "each player who controls a permanent named Bonder's Ornament"; drawing for every player instead would hand cards to opponents who control none.
pub(in crate::card::sets) static BONDER_S_ORNAMENT: CardRecord = CardRecord::new(
    "Bonder's Ornament",
    "5afe425c-50a7-4d29-ac14-0edb094fc770",
    "Lindsey Look",
    crate::card::CardRules::unsupported(),
);

// C20 118 — Murmuring Mystic (reprint)
const MURMURING_MYSTIC_REPRINT: PrintingRecord = PrintingRecord::reprint(
    &crate::card::sets::y2018::guilds_of_ravnica::MURMURING_MYSTIC,
    "ab25853c-29d3-4244-88db-813300a262a5",
    "Mark Winters",
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&ETHEREAL_FORAGER, &BONDER_S_ORNAMENT];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] =
    &[MURMURING_MYSTIC_REPRINT];
