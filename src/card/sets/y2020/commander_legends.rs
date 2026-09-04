//! Commander Legends card records required by supported formats.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, CardRules, CardSet, EffectDef, PlayerRelation, ReplacementAbilityDef,
    ReplacementEffectDef, ReplacementEventDef, abilities, tokens,
};
use crate::mana_cost;

// CMR 74 — Hullbreacher
pub(in crate::card::sets) static HULLBREACHER: CardRecord = CardRecord::new(
    CardSet::CommanderLegends,
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

// CMR 216 — Annoyed Altisaur
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static ANNOYED_ALTISAUR: CardRecord = CardRecord::new(
    crate::card::CardSet::CommanderLegends,
    "Annoyed Altisaur",
    "7536d618-0c98-45bb-913b-b8117b4acf87",
    "Lars Grant-West",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&HULLBREACHER, &ANNOYED_ALTISAUR];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
