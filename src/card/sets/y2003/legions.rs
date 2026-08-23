//! Legions cards used by the staged Premodern deck tranche.

use super::{CardRecord, PrintingAnchor, PrintingRecord};
use crate::card::{
    AbilityDef, AbilityTargetDef, CardArt, CardRules, CardSet, CardType, EffectDef,
    EffectRecipientDef, ObjectPredicateDef, ObjectQueryDef, PlayerRelation, TriggerEventDef,
    ValueDef, ZoneKind, abilities,
};
use crate::ids::TargetIndex;
use crate::mana_cost;

/// Every Goblin on the battlefield, whoever controls it -- the count is of
/// the board, not of your side of it.
static GOBLINS_ON_THE_BATTLEFIELD: ObjectQueryDef = ObjectQueryDef::matching(
    ObjectPredicateDef::Subtype("Goblin"),
    &[ZoneKind::Battlefield],
    PlayerRelation::Any,
);

static INCINERATE_FOR_EACH_GOBLIN: EffectDef = EffectDef::DealDamage {
    recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    amount: ValueDef::CountMatchingObjects(&GOBLINS_ON_THE_BATTLEFIELD),
};

// LGN 12 — Deftblade Elite
// Audit: metadata-only — Card rules have not been implemented.
pub(in crate::card::sets) static DEFTBLADE_ELITE: CardRecord = CardRecord::new(
    PrintingAnchor::scryfall("76ffbae4-7aad-493c-86a0-c6e6425da8fd"),
    "Deftblade Elite",
    crate::card::CardArt::new("76ffbae4-7aad-493c-86a0-c6e6425da8fd", "Alan Pollack"),
    crate::card::CardSet::Legions,
    crate::card::CardRules::unsupported(),
);

// LGN 94 — Gempalm Incinerator
pub(in crate::card::sets) static GEMPALM_INCINERATOR: CardRecord = CardRecord::new_with_legacy_id(
    2026,
    "Gempalm Incinerator",
    CardArt::new("2687c311-fd0c-4fe0-bce8-e3f412216796", "Luca Zontini"),
    CardSet::Legions,
    // The card is played as removal far more often than as a creature, and
    // the Incinerator itself is not on the battlefield when it counts -- it
    // is in the graveyard, so it never counts itself.
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Goblin"], 2, 1).with_abilities(&[
        abilities::cycling(
            "Cycling {1}{R} ({1}{R}, Discard this card: Draw a card.)",
            mana_cost!("{1}{R}"),
        ),
        AbilityDef::triggered_with_targets(
            "When you cycle this card, you may have it deal X damage to target creature, where X is the number of Goblins on the battlefield.",
            TriggerEventDef::Cycled,
            &[AbilityTargetDef::exactly_one_permanent(
                ObjectPredicateDef::HasType(CardType::Creature),
            )],
            EffectDef::May {
                player: EffectRecipientDef::Controller,
                effect: &INCINERATE_FOR_EACH_GOBLIN,
            },
        ),
    ]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&DEFTBLADE_ELITE, &GEMPALM_INCINERATOR];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
