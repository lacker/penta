//! SOI card records required by supported formats.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, CardArt, CardRules, CardSet, CardType, CounterKind, EffectDef, EffectRecipientDef,
    ObjectPredicateDef, PlayerRelation, TriggerEventDef, ValueDef, ZoneKind, abilities, tokens,
};
use crate::mana_cost;

// SOI 44 — Thraben Inspector
pub(in crate::card::sets) static THRABEN_INSPECTOR: CardRecord = CardRecord::new(
    CardSet::ShadowsOverInnistrad,
    "Thraben Inspector",
    "d140c3b7-ca78-483d-baeb-307b624fea8b",
    "Matt Stewart",
    // One mana for a body and a card: the Clue is the reason the 1/2 is
    // worth playing, and the body is the reason the card is cheap.
    CardRules::new_creature(mana_cost!("{W}"), &["Human", "Soldier"], 1, 2).with_ability(
        abilities::enters_trigger(
            "When this creature enters, investigate. (Create a Clue token. It's an artifact with \
             \"{2}, Sacrifice this token: Draw a card.\")",
            EffectDef::create_token(tokens::clue()).with_art(CardArt::new(
                "f2c859e1-181e-44d1-afbd-bbd6e52cf42a",
                "John Avon",
            )),
        ),
    ),
);

// SOI 233 — Tireless Tracker
pub(in crate::card::sets) static TIRELESS_TRACKER: CardRecord = CardRecord::new(
    CardSet::ShadowsOverInnistrad,
    "Tireless Tracker",
    "ee8e9928-d9b2-4570-adb8-44b34115decd",
    "Eric Deschamps",
    // Three mana for a 3/2 that turns every land after it into a card, and
    // grows every time one of those cards is cashed in.
    CardRules::new_creature(mana_cost!("{2}{G}"), &["Human", "Scout"], 3, 2).with_abilities(&[
        AbilityDef::triggered(
            "Landfall — Whenever a land you control enters, investigate. (Create a Clue token. \
             It's an artifact with \"{2}, Sacrifice this token: Draw a card.\")",
            // A land you control arriving, which is what landfall is: the Tracker's
            // own arrival is not one, and neither is a land somebody else plays.
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::All(&[
                    ObjectPredicateDef::HasType(CardType::Land),
                    ObjectPredicateDef::ControlledBy(PlayerRelation::You),
                ]),
                None,
                Some(ZoneKind::Battlefield),
            ),
            EffectDef::create_token(tokens::clue()).with_art(CardArt::new(
                "f2c859e1-181e-44d1-afbd-bbd6e52cf42a",
                "John Avon",
            )),
        ),
        AbilityDef::triggered(
            "Whenever you sacrifice a Clue, put a +1/+1 counter on this creature.",
            TriggerEventDef::Sacrificed {
                object: ObjectPredicateDef::Subtype("Clue"),
                player: PlayerRelation::You,
            },
            EffectDef::AddCounters {
                object: EffectRecipientDef::Source,
                kind: CounterKind::PlusOnePlusOne,
                amount: ValueDef::Constant(1),
            },
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&THRABEN_INSPECTOR, &TIRELESS_TRACKER];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
