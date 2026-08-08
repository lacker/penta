//! Reusable constructors for common ability clauses.
//!
//! The functions here return identity-free [`AbilityDef`] values. A card part,
//! intrinsic rule, or grant site assigns identity when it attaches the clause.

use super::model::{
    AbilityCostDef, AbilityDef, AbilityImplementationDef, AddManaEffectDef, BasicLandType,
    ColorDef, EffectDef, KeywordAbility, ManaKindDef,
};

/// Why a printed basic-land-type mana clause is not yet fully modeled.
pub const BASIC_LAND_TYPE_MANA_EXPLANATION: &str = concat!(
    "Mana production is implemented, but printed lands model this as an explicit clause rather ",
    "than deriving it intrinsically from their basic land subtype; intrinsic derivation is ",
    "currently limited to Blood Moon.",
);

const fn keyword(text: &'static str, keyword: KeywordAbility) -> AbilityDef {
    AbilityDef::keyword(text, keyword)
}

const fn unsupported_keyword(
    text: &'static str,
    ability: KeywordAbility,
    explanation: &'static str,
) -> AbilityDef {
    keyword(text, ability)
        .with_implementation(AbilityImplementationDef::NotImplemented { explanation })
}

#[must_use]
pub const fn flying() -> AbilityDef {
    keyword("Flying", KeywordAbility::Flying)
}

#[must_use]
pub const fn trample() -> AbilityDef {
    keyword("Trample", KeywordAbility::Trample)
}

#[must_use]
pub const fn haste() -> AbilityDef {
    keyword("Haste", KeywordAbility::Haste)
}

#[must_use]
pub const fn first_strike() -> AbilityDef {
    unsupported_keyword(
        "First strike",
        KeywordAbility::FirstStrike,
        "First-strike combat damage is not implemented.",
    )
}

#[must_use]
pub const fn double_strike() -> AbilityDef {
    unsupported_keyword(
        "Double strike",
        KeywordAbility::DoubleStrike,
        "Double-strike combat damage is not implemented.",
    )
}

#[must_use]
pub const fn banding() -> AbilityDef {
    unsupported_keyword(
        "Banding",
        KeywordAbility::Banding,
        "Band formation and combat damage assignment are not implemented.",
    )
}

#[must_use]
pub const fn vigilance() -> AbilityDef {
    keyword("Vigilance", KeywordAbility::Vigilance)
}

#[must_use]
pub const fn deathtouch() -> AbilityDef {
    keyword("Deathtouch", KeywordAbility::Deathtouch)
}

#[must_use]
pub const fn lifelink() -> AbilityDef {
    keyword("Lifelink", KeywordAbility::Lifelink)
}

#[must_use]
pub const fn reach() -> AbilityDef {
    keyword("Reach", KeywordAbility::Reach)
}

#[must_use]
pub const fn flash() -> AbilityDef {
    keyword("Flash", KeywordAbility::Flash)
}

#[must_use]
pub const fn hexproof() -> AbilityDef {
    keyword("Hexproof", KeywordAbility::Hexproof)
}

#[must_use]
pub const fn intimidate() -> AbilityDef {
    keyword("Intimidate", KeywordAbility::Intimidate)
}

#[must_use]
pub const fn undying() -> AbilityDef {
    keyword("Undying", KeywordAbility::Undying)
}

#[must_use]
pub const fn mountainwalk() -> AbilityDef {
    keyword("Mountainwalk", KeywordAbility::Mountainwalk)
}

#[must_use]
pub const fn protection_from(color: ColorDef) -> AbilityDef {
    let text = match color {
        ColorDef::White => "Protection from white",
        ColorDef::Blue => "Protection from blue",
        ColorDef::Black => "Protection from black",
        ColorDef::Red => "Protection from red",
        ColorDef::Green => "Protection from green",
    };
    keyword(text, KeywordAbility::ProtectionFrom(color))
}

/// A common mana ability that taps its source to add one fixed kind of mana.
#[must_use]
pub const fn tap_for(mana: ManaKindDef) -> AbilityDef {
    let text = match mana {
        ManaKindDef::White => "{T}: Add {W}.",
        ManaKindDef::Blue => "{T}: Add {U}.",
        ManaKindDef::Black => "{T}: Add {B}.",
        ManaKindDef::Red => "{T}: Add {R}.",
        ManaKindDef::Green => "{T}: Add {G}.",
        ManaKindDef::Colorless => "{T}: Add {C}.",
    };
    AbilityDef::activated_mana(
        text,
        &[AbilityCostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::one(mana)),
    )
}

/// The explicit stand-in for the intrinsic mana ability granted by one basic
/// land type. It remains executable, but partial, until the runtime derives it
/// from the object's current types instead of relying on a separate clause.
#[must_use]
pub const fn basic_land_type_mana(land_type: BasicLandType) -> AbilityDef {
    tap_for(land_type.mana_kind()).with_implementation(AbilityImplementationDef::CustomPartial {
        behavior: None,
        explanation: BASIC_LAND_TYPE_MANA_EXPLANATION,
    })
}

#[cfg(test)]
mod tests {
    use super::{
        BASIC_LAND_TYPE_MANA_EXPLANATION, basic_land_type_mana, first_strike, flying, tap_for,
    };
    use crate::card::{
        AbilityCostDef, AbilityDef, AbilityImplementationDef, AddManaEffectDef, BasicLandType,
        CardRules, DeclarativeAbilityDef, EffectDef, KeywordAbility, ManaCost, ManaKindDef,
    };

    #[test]
    fn tap_for_builds_a_complete_executable_mana_ability() {
        let cases = [
            (ManaKindDef::White, "{T}: Add {W}."),
            (ManaKindDef::Blue, "{T}: Add {U}."),
            (ManaKindDef::Black, "{T}: Add {B}."),
            (ManaKindDef::Red, "{T}: Add {R}."),
            (ManaKindDef::Green, "{T}: Add {G}."),
            (ManaKindDef::Colorless, "{T}: Add {C}."),
        ];

        for (mana, text) in cases {
            let ability = tap_for(mana);
            assert_eq!(ability.text, text);
            assert_eq!(ability.implementation, AbilityImplementationDef::Definition);
            assert!(ability.implementation.is_executable());
            assert!(matches!(
                ability.definition,
                DeclarativeAbilityDef::ActivatedMana(definition)
                    if definition.costs == [AbilityCostDef::TapSource]
            ));
            assert_eq!(
                ability.effect,
                EffectDef::AddMana(AddManaEffectDef::one(mana))
            );
        }
    }

    #[test]
    fn basic_land_type_mana_is_an_executable_partial_wrapper() {
        for land_type in BasicLandType::ALL {
            let ability = basic_land_type_mana(land_type);
            let complete = tap_for(land_type.mana_kind());

            assert_eq!(ability.text, complete.text);
            assert_eq!(ability.definition, complete.definition);
            assert_eq!(ability.effect, complete.effect);
            assert!(ability.implementation.is_executable());
            assert!(matches!(
                ability.implementation,
                AbilityImplementationDef::CustomPartial {
                    behavior: None,
                    explanation: BASIC_LAND_TYPE_MANA_EXPLANATION,
                }
            ));
        }
    }

    #[test]
    fn keyword_presence_is_distinct_from_executable_keyword_support() {
        static KEYWORDS: [AbilityDef; 2] = [flying(), first_strike()];
        let rules =
            CardRules::new_creature(ManaCost::default(), &[], 1, 1, "").with_abilities(&KEYWORDS);

        assert!(rules.has_keyword(KeywordAbility::Flying));
        assert!(rules.has_executable_keyword(KeywordAbility::Flying));
        assert!(rules.has_keyword(KeywordAbility::FirstStrike));
        assert!(!rules.has_executable_keyword(KeywordAbility::FirstStrike));
    }
}
