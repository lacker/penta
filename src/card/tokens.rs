//! Constructors for standardized token rules.
//!
//! Ordinary creature and artifact-creature tokens are authored directly with
//! [`EffectDef::create_creature_token`] and
//! [`EffectDef::create_artifact_creature_token`]. This module contains only
//! the handful of token rules standardized by the game itself.

use crate::card::{
    AbilityCostDef, AbilityCoverageDef, AbilityDef, AbilityTargetDef, AbilityTargetPredicate,
    ActivationTimingDef, AddManaEffectDef, AppliedEffectDef, AppliedRuleDef, EffectDef,
    EffectRecipientDef, ObjectPredicateDef, ResolvedEffectDurationDef, TokenCharacteristics,
    TokenPart, ValueDef, ZoneKind, abilities,
};
use crate::mana_cost;
use crate::{CardPartId, TargetIndex};

/// Characteristics for an ordinary creature token. The token's name is the
/// subtype list joined with spaces unless its creating effect overrides it.
#[must_use]
pub const fn creature(
    subtypes: &'static [&'static str],
    colors: &'static [crate::card::ManaColor],
    power: i16,
    toughness: i16,
) -> TokenCharacteristics {
    TokenCharacteristics::creature(subtypes, colors, power, toughness)
}

/// Characteristics for an ordinary artifact creature token.
#[must_use]
pub const fn artifact_creature(
    subtypes: &'static [&'static str],
    colors: &'static [crate::card::ManaColor],
    power: i16,
    toughness: i16,
) -> TokenCharacteristics {
    TokenCharacteristics::artifact_creature(subtypes, colors, power, toughness)
}

/// Characteristics for an ordinary noncreature artifact token.
#[must_use]
pub const fn artifact(
    subtypes: &'static [&'static str],
    colors: &'static [crate::card::ManaColor],
) -> TokenCharacteristics {
    TokenCharacteristics::artifact(subtypes, colors)
}

static TREASURE_COST: [AbilityCostDef; 2] =
    [AbilityCostDef::TapSource, AbilityCostDef::SacrificeSource];
static TREASURE_ABILITIES: [AbilityDef; 1] = [AbilityDef::activated_mana(
    "{T}, Sacrifice this artifact: Add one mana of any color.",
    &TREASURE_COST,
    EffectDef::AddMana(AddManaEffectDef::any_color()),
)];

/// The rules-defined Treasure artifact token.
#[must_use]
pub const fn treasure() -> TokenCharacteristics {
    TokenCharacteristics::artifact(&["Treasure"], &[]).with_abilities(&TREASURE_ABILITIES)
}

static FOOD_ABILITIES: [AbilityDef; 1] = [AbilityDef::activated(
    "{2}, {T}, Sacrifice this token: You gain 3 life.",
    &[
        AbilityCostDef::Mana(mana_cost!("{2}")),
        AbilityCostDef::TapSource,
        AbilityCostDef::SacrificeSource,
    ],
    EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(3),
    },
)];

/// The rules-defined Food artifact token.
#[must_use]
pub const fn food() -> TokenCharacteristics {
    TokenCharacteristics::artifact(&["Food"], &[]).with_abilities(&FOOD_ABILITIES)
}

static PEST_ABILITIES: [AbilityDef; 1] = [crate::card::abilities::dies_trigger(
    "When this token dies, you gain 1 life.",
    EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    },
)];

/// The Pest every Witherbloom card makes: a 1/1 in both its colours that
/// pays a life back when it dies. Unlike Food or Blood it is not a
/// rules-defined token, but every card printing it prints the same one.
#[must_use]
pub const fn pest() -> TokenCharacteristics {
    TokenCharacteristics::creature(
        &["Pest"],
        &[crate::card::ManaColor::Black, crate::card::ManaColor::Green],
        1,
        1,
    )
    .with_abilities(&PEST_ABILITIES)
}

static CLUE_ABILITIES: [AbilityDef; 1] = [AbilityDef::activated(
    "{2}, Sacrifice this token: Draw a card.",
    &[
        AbilityCostDef::Mana(mana_cost!("{2}")),
        AbilityCostDef::SacrificeSource,
    ],
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    },
)];

/// The rules-defined Clue artifact token.
#[must_use]
pub const fn clue() -> TokenCharacteristics {
    TokenCharacteristics::artifact(&["Clue"], &[]).with_abilities(&CLUE_ABILITIES)
}

static BLOOD_ABILITIES: [AbilityDef; 1] = [AbilityDef::activated(
    "{1}, {T}, Discard a card, Sacrifice this token: Draw a card.",
    &[
        AbilityCostDef::Mana(mana_cost!("{1}")),
        AbilityCostDef::TapSource,
        AbilityCostDef::DiscardCardMatching(ObjectPredicateDef::Any),
        AbilityCostDef::SacrificeSource,
    ],
    EffectDef::DrawCards {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    },
)];

/// The rules-defined Blood artifact token.
#[must_use]
pub const fn blood() -> TokenCharacteristics {
    TokenCharacteristics::artifact(&["Blood"], &[]).with_abilities(&BLOOD_ABILITIES)
}

static MAP_CREATURE_TARGET: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one(
    AbilityTargetPredicate::Object {
        object: ObjectPredicateDef::HasType(crate::card::CardType::Creature),
        zones: &[ZoneKind::Battlefield],
        controller: Some(crate::card::PlayerRelation::You),
        owner: None,
    },
)];
static MAP_COST: [AbilityCostDef; 3] = [
    AbilityCostDef::Mana(mana_cost!("{1}")),
    AbilityCostDef::TapSource,
    AbilityCostDef::SacrificeSource,
];
static MAP_ABILITIES: [AbilityDef; 1] = [AbilityDef::activated_with_targets(
    "{1}, {T}, Sacrifice this token: Target creature you control explores.",
    &MAP_COST,
    &MAP_CREATURE_TARGET,
    EffectDef::Explore {
        object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
    },
)
.with_activation_timing(ActivationTimingDef::SorcerySpeed)];

/// The rules-defined Map artifact token.
#[must_use]
pub const fn map() -> TokenCharacteristics {
    TokenCharacteristics::artifact(&["Map"], &[]).with_abilities(&MAP_ABILITIES)
}

static INCUBATOR_TRANSFORM_COST: [AbilityCostDef; 1] = [AbilityCostDef::Mana(mana_cost!("{2}"))];
static INCUBATOR_ABILITIES: [AbilityDef; 1] = [AbilityDef::activated(
    "{2}: Transform this token.",
    &INCUBATOR_TRANSFORM_COST,
    EffectDef::Transform {
        object: EffectRecipientDef::Source,
    },
)];
static INCUBATOR_BACK: TokenPart = TokenPart::new(
    CardPartId(1),
    "Phyrexian",
    crate::card::CardRules::new_artifact_creature_without_mana_cost(&["Phyrexian"], 0, 0),
);

/// The rules-defined transforming Incubator token.
#[must_use]
pub const fn incubator() -> TokenCharacteristics {
    TokenCharacteristics::artifact(&["Incubator"], &[])
        .with_abilities(&INCUBATOR_ABILITIES)
        .transforming_into(&INCUBATOR_BACK)
}

static TETRAVITE_ABILITIES: [AbilityDef; 2] = [
    abilities::flying(),
    AbilityDef::static_ability(
        "This token can't be enchanted.",
        EffectDef::StaticApply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::Rule(AppliedRuleDef::CannotBeEnchanted),
        },
    )
    .with_coverage(AbilityCoverageDef::explained_complete(
        "The shared targetability check refuses the token to an Aura spell, and an Aura that arrives some other way still falls off.",
    )),
];

/// Tetravus detaches these, and can exile its own back to rebuild itself.
#[must_use]
pub const fn tetravite() -> TokenCharacteristics {
    TokenCharacteristics::artifact_creature(&["Tetravite"], &[], 1, 1)
        .with_abilities(&TETRAVITE_ABILITIES)
}

/// Shared ability for the uncommon 2/2 red Dragon token that pumps itself.
/// Kept as a rules helper rather than a token identity.
#[must_use]
pub const fn dragon_pump() -> AbilityDef {
    AbilityDef::activated(
        "{R}: This creature gets +1/+0 until end of turn.",
        &[AbilityCostDef::Mana(mana_cost!("{R}"))],
        EffectDef::Apply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::modify_power_toughness(
                ValueDef::Constant(1),
                ValueDef::Constant(0),
            ),
            duration: ResolvedEffectDurationDef::UntilEndOfTurn,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standardized_artifact_tokens_have_their_artifact_subtypes() {
        for (token, expected) in [
            (treasure(), "Treasure"),
            (food(), "Food"),
            (clue(), "Clue"),
            (blood(), "Blood"),
            (map(), "Map"),
        ] {
            assert_eq!(token.name(), expected);
            assert_eq!(token.rules().subtypes(), &[expected]);
        }
    }

    #[test]
    fn dragon_rules_can_retain_both_flying_and_the_pump() {
        static DRAGON_ABILITIES: [AbilityDef; 2] = [abilities::flying(), dragon_pump()];
        let token =
            TokenCharacteristics::creature(&["Dragon"], &[crate::card::ManaColor::Red], 2, 2)
                .with_abilities(&DRAGON_ABILITIES);
        assert_eq!(token.rules().ability_clauses().len(), 2);
    }
}
