//! Portal Three Kingdoms card records required by supported formats.

use super::CardRecord;
use super::PrintingRecord;
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AbilityTargetPredicate;
use crate::card::CardRules;
use crate::card::DiscardSelectionDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::PlayerRelation;
use crate::card::ValueDef;
use crate::card::abilities;
use crate::card::sets::y1997::visions as catalog_vis;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "PTK",
    slug: "portal-three-kingdoms",
});

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// PTK 71 — Corrupt Court Official
pub(in crate::card::sets) static CORRUPT_COURT_OFFICIAL: CardRecord = CardRecord::new(
    "Corrupt Court Official",
    "9d3ba2e3-e680-47cd-81c5-555deea7d00f",
    "Li Yousong",
    CardRules::new_creature(mana_cost!("{1}{B}"), &["Human", "Advisor"], 1, 1).with_abilities(&[
        abilities::enters_trigger_with_targets(
            "When this creature enters, target opponent discards a card.",
            &[AbilityTargetDef::exactly_one(
                AbilityTargetPredicate::Player(PlayerRelation::Opponent),
            )],
            EffectDef::Discard {
                recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY),
                amount: ValueDef::Constant(1),
                selection: DiscardSelectionDef::RecipientChooses,
                then: None,
            },
        ),
    ]),
);

// PTK 78 — Imperial Seal
pub(in crate::card::sets) static IMPERIAL_SEAL: CardRecord = CardRecord::new(
    "Imperial Seal",
    "822e30db-40c5-4099-868b-185ad9b7c7dc",
    "Li Tie",
    // Vampiric Tutor's clause at sorcery speed, which is the whole of the
    // difference: the card you want is on top of your library, and you wait
    // a turn to draw it.
    CardRules::new_sorcery(mana_cost!("{B}")).with_ability(AbilityDef::spell(
        "Search your library for a card, then shuffle and put that card on top. You lose 2 life.",
        EffectDef::Sequence(&catalog_vis::VAMPIRIC_TUTOR_EFFECT),
    )),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[&CORRUPT_COURT_OFFICIAL, &IMPERIAL_SEAL];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
