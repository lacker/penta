//! C20 card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, CardArt, CardRules, CardSet, CardType, ChoiceVisibilityDef, ChooseDef, EffectDef,
    EffectRecipientDef, ManaColor, ObjectChoiceBindingDef, ObjectPredicateDef, ObjectRefDef,
    ObjectSetDef, ObjectSetFilterDef, PlayerRefDef, PlayerRelation, TriggerEventDef, ZoneKind,
    ZonePlacement, abilities,
};
use crate::{ParentBinding, mana_cost};

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
    PrintingAnchor::scryfall("5afe425c-50a7-4d29-ac14-0edb094fc770"),
    "Bonder's Ornament",
    crate::card::CardArt::new("5afe425c-50a7-4d29-ac14-0edb094fc770", "Lindsey Look"),
    crate::card::CardSet::Commander2020,
    crate::card::CardRules::unsupported(),
);

// C20 118 — Murmuring Mystic
pub(in crate::card::sets) static MURMURING_MYSTIC: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("5fc6adff-dcb3-456d-a8c2-0e77b784ff89"),
    "Murmuring Mystic",
    CardArt::new("ab25853c-29d3-4244-88db-813300a262a5", "Mark Winters"),
    CardSet::Commander2020,
    // A 1/5 body that turns every cantrip into a blocker, so the deck that
    // was already casting spells stops needing creatures of its own.
    CardRules::new_creature(mana_cost!("{3}{U}"), &["Human", "Wizard"], 1, 5).with_ability(
        AbilityDef::triggered(
            "Whenever you cast an instant or sorcery spell, create a 1/1 blue Bird Illusion creature token with flying.",
            // On the cast rather than the resolution, so a countered spell
            // has already paid for its Bird.
            TriggerEventDef::spell_cast(ObjectPredicateDef::All(&[
                ObjectPredicateDef::AnyOf(&[
                    ObjectPredicateDef::HasType(CardType::Instant),
                    ObjectPredicateDef::HasType(CardType::Sorcery),
                ]),
                ObjectPredicateDef::ControlledBy(PlayerRelation::You),
            ])),
            EffectDef::create_creature_token(&["Bird", "Illusion"], &[ManaColor::Blue], 1, 1)
                .with_abilities(&[abilities::flying()]),
        ),
    ),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&ETHEREAL_FORAGER, &BONDER_S_ORNAMENT, &MURMURING_MYSTIC];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
