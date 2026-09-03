//! Commander Legends card records required by supported formats.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, CardArt, CardRules, CardSet, EffectDef, PlayerRelation, ReplacementAbilityDef,
    ReplacementEffectDef, ReplacementEventDef, abilities, tokens,
};
use crate::mana_cost;

// CMR 74 — Hullbreacher
pub(in crate::card::sets) static HULLBREACHER: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("4df8aabc-7fcb-4b7b-980b-18f499e6c170"),
    "Hullbreacher",
    CardArt::new(
        "4df8aabc-7fcb-4b7b-980b-18f499e6c170",
        "Sidharth Chaturvedi",
    ),
    CardSet::CommanderLegends,
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
    PrintingAnchor::scryfall("714c3a1f-7b30-4ed8-8f38-6176758741fb"),
    "Sakashima of a Thousand Faces",
    CardArt::new("714c3a1f-7b30-4ed8-8f38-6176758741fb", "Jason A. Engle"),
    CardSet::CommanderLegends,
    CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[&HULLBREACHER, &SAKASHIMA_OF_A_THOUSAND_FACES];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
