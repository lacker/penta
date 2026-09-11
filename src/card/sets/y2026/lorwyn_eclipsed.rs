//! ECL card records required by supported formats.

use super::CardRecord;
use super::PrintingRecord;
use crate::card::AbilityDef;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::BattlefieldEntryScalarChoiceDef;
use crate::card::CardRules;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaTypeDef;
use crate::card::ObjectPredicateDef;
use crate::card::ReplacementChoiceDef;
use crate::card::ReplacementEffectDef;
use crate::card::TriggerEventDef;
use crate::card::ValueDef;
use crate::card::ZoneKind;
use crate::card::abilities;
use crate::mana_cost;

/// Printed set identity and stable catalog slug.
pub const SET: crate::card::CardSet = crate::card::CardSet::new("ECL", "lorwyn-eclipsed");

pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());

// ECL 128 — Brambleback Brute
// Audit: unsupported — Needs a cost that removes a counter of any kind. RemoveCountersFromSource names one kind, and naming -1/-1 would take away the choice the card gives once anything else has put a counter on it.
pub(in crate::card::sets) static BRAMBLEBACK_BRUTE: CardRecord = CardRecord::new(
    "Brambleback Brute",
    "5ebb8365-c6e1-46e8-a242-6aa27b21e68a",
    "Aaron Miller",
    crate::card::CardRules::unsupported(),
);

// ECL 181 — Lys Alana Informant
pub(in crate::card::sets) static LYS_ALANA_INFORMANT: CardRecord = CardRecord::new(
    "Lys Alana Informant",
    "a79649c4-559e-4306-a102-5fd8750629c7",
    "Sidharth Chaturvedi",
    // A 3/1 that surveils coming and going, so trading it away is still a
    // profitable turn for a deck that wants its graveyard filled.
    CardRules::new_creature(mana_cost!("{1}{G}"), &["Elf", "Scout"], 3, 1).with_ability(
        AbilityDef::triggered(
            "When this creature enters or dies, surveil 1. (Look at the top card of your \
             library. You may put it into your graveyard.)",
            // Entering and dying are two ways for one printed ability to
            // fire, so what it does is written once.
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                ),
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                ),
            ]),
            abilities::surveil(ValueDef::Constant(1)),
        ),
    ),
);

// ECL 194 — Shimmerwilds Growth
pub(in crate::card::sets) static SHIMMERWILDS_GROWTH: CardRecord = CardRecord::new(
    "Shimmerwilds Growth",
    "c122719c-f0d1-4170-a0d1-d62172df1d21",
    "Jorge Jacinto",
CardRules::new_enchantment(mana_cost!("{1}{G}"))
        .with_subtypes(&["Aura"])
        .with_abilities(&[
            abilities::enchant_land(),
            AbilityDef::as_enters(
                "As this Aura enters, choose a color.",
                ReplacementEffectDef::Choose(ReplacementChoiceDef::Scalar(
                    BattlefieldEntryScalarChoiceDef::COLOR,
                )),
            ),
            AbilityDef::static_ability(
                "Enchanted land is the chosen color.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::AttachedPermanent,
                    effect: AppliedEffectDef::set_color(ManaTypeDef::ChosenColor),
                },
            ),
            AbilityDef::triggered_mana(
                "Whenever enchanted land is tapped for mana, its controller adds an additional one mana of the chosen color.",
                TriggerEventDef::tapped_for_mana(ObjectPredicateDef::AttachedToSource),
                EffectDef::AddMana(
                    AddManaEffectDef::one_of_type(ManaTypeDef::ChosenColor)
                        .to_triggering_objects_controller(),
                ),
            ),
        ]),
);

// ECL 251 — Wary Farmer
// Audit: unsupported — Needs an event-tracked "a creature entered under your control this turn" condition. The only available reading is ObjectPredicateDef::EnteredThisTurn over the battlefield, which misses a creature that entered and left before the end step -- the printed intervening-if would still be satisfied. The condition vocabulary already tracks comparable events (CreatureDiedThisTurn, ControllerHadPermanentLeaveThisTurn) but not this one.
pub(in crate::card::sets) static WARY_FARMER: CardRecord = CardRecord::new(
    "Wary Farmer",
    "22d20c0d-176d-49c9-aa0b-2c5778548cc5",
    "Ron Spears",
    crate::card::CardRules::unsupported(),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] = &[
    &BRAMBLEBACK_BRUTE,
    &LYS_ALANA_INFORMANT,
    &SHIMMERWILDS_GROWTH,
    &WARY_FARMER,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
