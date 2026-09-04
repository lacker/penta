//! Adventures in the Forgotten Realms cards cataloged for the Vintage Cube
//! pool.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityTargetDef, AbilityTargetPredicate, CardRules, CardSet, CardType, EffectRecipientDef,
    ObjectPredicateDef, PlayerRelation, ZoneKind, abilities,
};
use crate::{TargetIndex, mana_cost};

// AFR 33 — Portable Hole
pub(in crate::card::sets) static PORTABLE_HOLE: CardRecord = CardRecord::new(
    CardSet::AdventuresInTheForgottenRealms,
    "Portable Hole",
    "80fca8c0-ae3e-439e-b202-228b9f360e9a",
    "John Stanko",
    // One white mana answers most of what a fast deck opens on, and it
    // answers it at instant speed on the other player's turn only because
    // somebody flashed it in -- otherwise the Hole is simply the cheapest
    // unconditional removal a white deck gets.
    CardRules::new_artifact(mana_cost!("{W}")).with_ability(
        abilities::enters_trigger_with_targets(
            "When this artifact enters, exile target nonland permanent an opponent controls with \
         mana value 2 or less until this artifact leaves the battlefield.",
            // A cheap nonland permanent across the table. Mana value is read off the
            // card, so a token is a zero and qualifies.
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::All(&[
                        ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land)),
                        ObjectPredicateDef::ManaValueAtMost(2),
                    ]),
                    zones: &[ZoneKind::Battlefield],
                    controller: Some(PlayerRelation::Opponent),
                    owner: None,
                },
            )],
            abilities::exile_until_source_leaves(EffectRecipientDef::Target(TargetIndex::PRIMARY)),
        ),
    ),
);

// AFR 42 — You Hear Something on Watch
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static YOU_HEAR_SOMETHING_ON_WATCH: CardRecord = CardRecord::new(
    crate::card::CardSet::AdventuresInTheForgottenRealms,
    "You Hear Something on Watch",
    "76e939ab-9d0c-4685-805c-c8bc4e6af163",
    "Zezhou Chen",
    crate::card::CardRules::unsupported(),
);

// AFR 198 — Owlbear
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static OWLBEAR: CardRecord = CardRecord::new(
    crate::card::CardSet::AdventuresInTheForgottenRealms,
    "Owlbear",
    "30e8a00f-8131-470d-8072-4c23b812281a",
    "Ilse Gort",
    crate::card::CardRules::unsupported(),
);

// AFR 215 — You Meet in a Tavern
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static YOU_MEET_IN_A_TAVERN: CardRecord = CardRecord::new(
    crate::card::CardSet::AdventuresInTheForgottenRealms,
    "You Meet in a Tavern",
    "593aa59a-4025-4df8-9f27-188fc7712fde",
    "Zoltan Boros",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &PORTABLE_HOLE,
    &YOU_HEAR_SOMETHING_ON_WATCH,
    &OWLBEAR,
    &YOU_MEET_IN_A_TAVERN,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
