//! Commander Legends card records required by supported formats.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::CardRules;
use crate::card::EffectDef;
use crate::card::PlayerRelation;
use crate::card::ReplacementAbilityDef;
use crate::card::ReplacementEffectDef;
use crate::card::ReplacementEventDef;
use crate::card::abilities;
use crate::card::tokens;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "CMR",
    slug: "commander-legends",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// CMR 74 — Hullbreacher
pub(in crate::card::sets) static HULLBREACHER: CardRecord = CardRecord::new(
    "Hullbreacher",
    "4df8aabc-7fcb-4b7b-980b-18f499e6c170",
    "Sidharth Chaturvedi",
// Three mana at instant speed that turns their draw spell into your
    // mana, and a 3/2 body attached to it.
    CardRules::new_creature(mana_cost!("{2}{U}"), &["Merfolk", "Pirate"], 3, 2)
        .with_abilities(&[
            abilities::flash(),
            AbilityDef::defined_replacement(
                "If an opponent would draw a card except the first one they draw in each of their draw \
                 steps, instead you create a Treasure token. (It\'s an artifact with \"{T}, Sacrifice \
                 this token: Add one mana of any color.\")",
                // "Except the first one they draw in each of their draw steps": their
                // turn-based draw still happens, and everything after it does not.
                ReplacementAbilityDef::new()
                    .with_event(ReplacementEventDef::WouldDraw {
                        player: PlayerRelation::Opponent,
                        during_own_draw_step: false,
                        except_first_in_draw_step: true,
                    }),
                // The draw is replaced outright and the Treasure is the effect's
                // controller's, which is what makes this a tax on them rather than a gift:
                // the card they would have drawn stays in their library.
                ReplacementEffectDef::Sequence(&[
                    ReplacementEffectDef::ReplaceEventWithNothing,
                    ReplacementEffectDef::Perform(&EffectDef::create_token(tokens::treasure())),
                ]),
            ),
        ]),
);

// CMR 89 — Sakashima of a Thousand Faces
// Audit: unsupported — Needs the Partner deck-construction permission and an entry-copy exception that retains every other ability printed on the source.
pub(in crate::card::sets) static SAKASHIMA_OF_A_THOUSAND_FACES: CardRecord = CardRecord::new(
    "Sakashima of a Thousand Faces",
    "714c3a1f-7b30-4ed8-8f38-6176758741fb",
    "Jason A. Engle",
    CardRules::unsupported(),
);

// CMR 216 — Annoyed Altisaur
pub(in crate::card::sets) static ANNOYED_ALTISAUR: CardRecord = CardRecord::new(
    "Annoyed Altisaur",
    "7536d618-0c98-45bb-913b-b8117b4acf87",
    "Lars Grant-West",
    // Seven mana with cascade attached, which is why a limited deck plays it
    // as two cards rather than as an expensive one.
    CardRules::new_creature(mana_cost!("{5}{G}{G}"), &["Dinosaur"], 6, 5).with_abilities(&[
        abilities::reach(),
        abilities::trample(),
        abilities::cascade(),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &HULLBREACHER,
    &SAKASHIMA_OF_A_THOUSAND_FACES,
    &ANNOYED_ALTISAUR,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
