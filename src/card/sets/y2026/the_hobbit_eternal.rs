//! The Hobbit Eternal card records required by the cEDH corpus.

use super::{CardRecord, PrintingRecord};
use crate::TargetIndex;
use crate::card::AbilityDef;
use crate::card::AbilityTargetDef;
use crate::card::AddManaEffectDef;
use crate::card::AppliedEffectDef;
use crate::card::CardRules;
use crate::card::CardSupertype;
use crate::card::CardType;
use crate::card::CostDef;
use crate::card::EffectDef;
use crate::card::EffectRecipientDef;
use crate::card::ManaColor;
use crate::card::ManaRestrictionDef;
use crate::card::ObjectPredicateDef;
use crate::card::ResolvedEffectDurationDef;
use crate::card::SubtypeDef;
use crate::card::TriggerEventDef;
use crate::card::abilities;
use crate::mana_cost;

pub const SET: crate::card::CardSet = crate::card::CardSet::new(&crate::card::CardSetMetadata {
    code: "HOC",
    slug: "the-hobbit-eternal",
});
pub(in crate::card::sets) const DEFINITION: crate::card::sets::SetDefinition =
    crate::card::sets::SetDefinition::new(SET, CARDS, ADDITIONAL_PRINTINGS, file!());





// HOC 1 — Fíli and Kíli, Joyous
pub(in crate::card::sets) static FILI_AND_KILI_JOYOUS_1: CardRecord = CardRecord::new(
    "Fíli and Kíli, Joyous",
    "e1d12200-ae0b-4155-9853-3ffaf490c84c",
    "Dmitry Burmak",
    CardRules::new_creature(mana_cost!("{2}{R}"), &["Dwarf", "Bard"], 3, 3)
        .with_supertype(CardSupertype::Legendary)
        .with_abilities(&[
            abilities::haste(),
            AbilityDef::activated_mana(
                "{T}: Add {R}{R}. Spend this mana only to cast Dwarf, Equipment, and Saga spells.",
                &[CostDef::TapSource],
                EffectDef::AddMana(
                    AddManaEffectDef::one(ManaColor::Red)
                        .with_amount(2)
                        .with_restrictions(&[ManaRestrictionDef::CastSpell(
                            ObjectPredicateDef::AnyOf(&[
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Dwarf")),
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Equipment")),
                                ObjectPredicateDef::Subtype(SubtypeDef::Literal("Saga")),
                            ]),
                        )]),
                ),
            ),
        ]),
);

// HOC 8 — Dragon-Cursed Halls
pub(in crate::card::sets) static DRAGON_CURSED_HALLS_8: CardRecord = CardRecord::new(
    "Dragon-Cursed Halls",
    "506b9df7-8236-4c6e-aebc-6b7e6fcd7e88",
    "Marta Nael",
    CardRules::new_land(&[]).with_abilities(&[
abilities::tap_for(ManaColor::Colorless),
AbilityDef::activated_with_targets("{1}, {T}: Until end of turn, target creature gains \"Whenever this creature deals combat damage to a player, create a Treasure token.\"", &[CostDef::Mana(mana_cost!("{1}")), CostDef::TapSource], &[AbilityTargetDef::exactly_one_permanent(ObjectPredicateDef::HasType(CardType::Creature))], EffectDef::Apply { recipient: EffectRecipientDef::Target(TargetIndex::PRIMARY), effect: AppliedEffectDef::add_ability(&AbilityDef::triggered("Whenever this creature deals combat damage to a player, create a Treasure token.", TriggerEventDef::combat_damage_to_player(ObjectPredicateDef::Source), EffectDef::create_token(crate::card::tokens::treasure()))), duration: ResolvedEffectDurationDef::UntilEndOfTurn })
]),
);

pub(in crate::card::sets) static CARDS: &[&CardRecord] =
    &[
    &FILI_AND_KILI_JOYOUS_1,
    &DRAGON_CURSED_HALLS_8,
];

pub(in crate::card::sets) static ADDITIONAL_PRINTINGS: &[PrintingRecord] = &[];
