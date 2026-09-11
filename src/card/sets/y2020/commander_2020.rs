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
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "C20",
    slug: "commander-2020",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// C20 26 — Flawless Maneuver
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FLAWLESS_MANEUVER_26: CardRecord = CardRecord::new(
    "Flawless Maneuver",
    "c972abe6-c732-4745-bde4-8b51698f05be",
    "Zoltan Boros",
    crate::card::CardRules::unsupported(),
);

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
                        then: &EffectDef::move_to_zone(
                            EffectRecipientDef::object(ObjectRefDef::Binding(
                                ParentBinding,
                            )),
                            ZoneKind::Hand,
                            ZonePlacement::Top,
                        ),
                    }),
                },
            ),
        ]),
);

// C20 35 — Fierce Guardianship
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static FIERCE_GUARDIANSHIP_35: CardRecord = CardRecord::new(
    "Fierce Guardianship",
    "4c5ffa83-c88d-4f5d-851e-a642b229d596",
    "Randy Vargas",
    crate::card::CardRules::unsupported(),
);

// C20 42 — Deadly Rollick
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEADLY_ROLLICK_42: CardRecord = CardRecord::new(
    "Deadly Rollick",
    "c61fa2c0-63c0-4dc2-9f17-5a00530e3348",
    "Izzy",
    crate::card::CardRules::unsupported(),
);

// C20 50 — Deflecting Swat
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static DEFLECTING_SWAT_50: CardRecord = CardRecord::new(
    "Deflecting Swat",
    "84f035e1-6c89-457b-b05f-85680a50ed91",
    "Izzy",
    crate::card::CardRules::unsupported(),
);

// C20 66 — Slippery Bogbonder
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static SLIPPERY_BOGBONDER_66: CardRecord = CardRecord::new(
    "Slippery Bogbonder",
    "c2f9c4a7-ea53-4da0-9746-2195579f98f6",
    "Mila Pesic",
    crate::card::CardRules::unsupported(),
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

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &FLAWLESS_MANEUVER_26,
    &ETHEREAL_FORAGER,
    &FIERCE_GUARDIANSHIP_35,
    &DEADLY_ROLLICK_42,
    &DEFLECTING_SWAT_50,
    &SLIPPERY_BOGBONDER_66,
    &BONDER_S_ORNAMENT,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] =
    &[MURMURING_MYSTIC_REPRINT];
