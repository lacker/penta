//! ECL card records required by supported formats.

use super::{CardRecord, PrintingRecord};
use crate::card::{
    AbilityDef, AddManaEffectDef, AppliedEffectDef, BattlefieldEntryScalarChoiceDef, CardRules,
    CardSet, EffectDef, EffectRecipientDef, ManaTypeDef, ObjectPredicateDef, ReplacementChoiceDef,
    ReplacementEffectDef, TriggerEventDef, abilities,
};
use crate::mana_cost;

// ECL 128 — Brambleback Brute
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static BRAMBLEBACK_BRUTE: CardRecord = CardRecord::new(
    crate::card::CardSet::LorwynEclipsed,
    "Brambleback Brute",
    "5ebb8365-c6e1-46e8-a242-6aa27b21e68a",
    "Aaron Miller",
    crate::card::CardRules::unsupported(),
);

// ECL 181 — Lys Alana Informant
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static LYS_ALANA_INFORMANT: CardRecord = CardRecord::new(
    crate::card::CardSet::LorwynEclipsed,
    "Lys Alana Informant",
    "a79649c4-559e-4306-a102-5fd8750629c7",
    "Sidharth Chaturvedi",
    crate::card::CardRules::unsupported(),
);

// ECL 194 — Shimmerwilds Growth
pub(in crate::card::sets) static SHIMMERWILDS_GROWTH: CardRecord = CardRecord::new(
    CardSet::LorwynEclipsed,
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
// Audit: unsupported — Card rules have not been implemented.
pub(in crate::card::sets) static WARY_FARMER: CardRecord = CardRecord::new(
    crate::card::CardSet::LorwynEclipsed,
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
