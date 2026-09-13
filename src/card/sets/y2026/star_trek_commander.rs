//! Star Trek Commander cards.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::ObjectPredicateDef;
use crate::card::ObjectQueryDef;
use crate::card::PlayActionMatcherDef;
use crate::card::PlayRestrictionDef;
use crate::card::PlayerRelation;
use crate::card::PlayerSetDef;
use crate::card::ZoneKind;
use crate::card::ZonePositionDef;
use crate::card::abilities;
use crate::mana_cost;
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "TRC",
    slug: "star-trek-commander",
});
pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());
// TRC 200 — Benjamin Sisko, Besieged
pub(in crate::card::sets) static BENJAMIN_SISKO_BESIEGED: CardRecord = CardRecord::new(
    "Benjamin Sisko, Besieged",
    "6cf853b8-4479-424d-998a-4fc53456c38b",
    "Josu Hernaiz",
    CardRules::new_creature(mana_cost!("{4}{G}{G}"), &["Human", "Officer"], 5, 5)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::cards_known_to(
                "Play with the top card of your library revealed.",
                ObjectQueryDef::matching(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Library],
                    PlayerRelation::You,
                )
                .at(ZonePositionDef::FromTop(0)),
                PlayerSetDef::Related(PlayerRelation::Any),
            ),
            abilities::play_from_zone(
                ObjectQueryDef::matching(
                    ObjectPredicateDef::Any,
                    &[ZoneKind::Library],
                    PlayerRelation::You,
                )
                .at(ZonePositionDef::FromTop(0)),
                "You may play lands and cast creature spells from the top of your library.",
                PlayRestrictionDef::new(
                    PlayActionMatcherDef::Any,
                    ObjectPredicateDef::AnyOf(&[
                        ObjectPredicateDef::HasType(CardType::Land),
                        ObjectPredicateDef::HasType(CardType::Creature),
                    ]),
                ),
            ),
        ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&BENJAMIN_SISKO_BESIEGED];
pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
