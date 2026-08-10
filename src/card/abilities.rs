//! Reusable constructors for common ability clauses.
//!
//! The functions here return identity-free [`AbilityDef`] values. A card part,
//! intrinsic rule, or grant site assigns identity when it attaches the clause.

use super::model::{
    AbilityCostDef, AbilityCostList, AbilityCoverageDef, AbilityDef, AbilityTargetDef,
    AddManaEffectDef, AlternativeCastKindDef, AnimationDef, AppliedEffectDef, BasicLandType,
    BattlefieldEntryModificationDef, CardType, CardTypeSet, ConditionDef, CostDef, EffectDef,
    EffectDurationDef, EffectRecipientDef, KeywordAbility, ManaColor, ManaCost, ObjectPredicateDef,
    ObjectQueryDef, PaymentDef, PlayerRelation, ReplacementEffectDef, ZoneKind,
};

/// Mishra's Factory's 2/2 Assembly-Worker artifact creature. The card's
/// animation still resolves through its legacy immediate path, which reads
/// this definition rather than restating the creature it becomes.
pub static MISHRAS_FACTORY_ANIMATION: AnimationDef = AnimationDef::new(2, 2)
    .with_types(CardTypeSet::single(CardType::Creature).with(CardType::Artifact))
    .with_subtypes(&["Assembly-Worker"]);

/// "Attacks each combat if able." Cards state this in their own words rather
/// than as a printed keyword, so the text is supplied by the caller.
#[must_use]
pub const fn attacks_each_combat_if_able(text: &'static str) -> AbilityDef {
    keyword(text, KeywordAbility::AttacksEachCombatIfAble)
}
const ENTER_TAPPED: [ReplacementEffectDef; 1] = [ReplacementEffectDef::ModifyBattlefieldEntry(
    BattlefieldEntryModificationDef::Tapped,
)];
const PAY_TWO_LIFE: [CostDef; 1] = [CostDef::PayLife(2)];

const fn keyword(text: &'static str, keyword: KeywordAbility) -> AbilityDef {
    AbilityDef::keyword(text, keyword)
}

const fn unsupported_keyword(
    text: &'static str,
    ability: KeywordAbility,
    explanation: &'static str,
) -> AbilityDef {
    keyword(text, ability).with_coverage(AbilityCoverageDef::metadata_only(explanation))
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
    keyword("First strike", KeywordAbility::FirstStrike)
}

#[must_use]
pub const fn defender() -> AbilityDef {
    keyword("Defender", KeywordAbility::Defender)
}

#[must_use]
pub const fn double_strike() -> AbilityDef {
    keyword("Double strike", KeywordAbility::DoubleStrike)
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
pub const fn protection_from(color: ManaColor) -> AbilityDef {
    let text = match color {
        ManaColor::White => "Protection from white",
        ManaColor::Blue => "Protection from blue",
        ManaColor::Black => "Protection from black",
        ManaColor::Red => "Protection from red",
        ManaColor::Green => "Protection from green",
        ManaColor::Colorless => "Protection from colorless",
    };
    keyword(text, KeywordAbility::ProtectionFrom(color))
}

/// A printed flashback clause. Its attached ability identity becomes the
/// spell play option's alternative-cost identity.
#[must_use]
pub const fn flashback(mana_cost: ManaCost) -> AbilityDef {
    AbilityDef::alternative_cast(
        mana_cost,
        AlternativeCastKindDef::Flashback,
        None,
        EffectDef::None,
    )
}

/// Miracle, the permission to cast a card from hand for a different cost in
/// the window opened by drawing it.
#[must_use]
pub const fn miracle(mana_cost: ManaCost) -> AbilityDef {
    AbilityDef::alternative_cast(
        mana_cost,
        AlternativeCastKindDef::Miracle,
        None,
        EffectDef::None,
    )
}

/// A flashback ability whose cost is the mana cost of the card carrying it.
/// This is the form granted by Snapcaster Mage.
#[must_use]
pub const fn flashback_for_card_mana_cost() -> AbilityDef {
    AbilityDef::alternative_cast_for_card_mana_cost(
        AlternativeCastKindDef::Flashback,
        None,
        EffectDef::None,
    )
}

/// A printed overload clause. `effect` is the spell after every instance of
/// "target" has been changed to "each."
#[must_use]
pub const fn overload(
    mana_cost: ManaCost,
    stack_text: &'static str,
    effect: EffectDef,
) -> AbilityDef {
    AbilityDef::alternative_cast(
        mana_cost,
        AlternativeCastKindDef::Overload,
        Some(stack_text),
        effect,
    )
}

/// A Bloodrush ability activated from the card carrying it in hand. The
/// mechanic always discards that card in addition to paying its mana cost;
/// the card supplies its exact rules text, target declaration, and effect.
#[must_use]
pub const fn bloodrush(
    mana_cost: ManaCost,
    text: &'static str,
    targets: &'static [AbilityTargetDef],
    effect: EffectDef,
) -> AbilityDef {
    AbilityDef::activated_with_cost_list_and_targets(
        text,
        AbilityCostList::two(
            AbilityCostDef::Mana(mana_cost),
            AbilityCostDef::DiscardSource,
        ),
        targets,
        effect,
    )
    .with_source_zones(&[ZoneKind::Hand])
}

/// The intrinsic stack-zone rule carried by spells that cannot be countered.
#[must_use]
pub const fn cannot_be_countered() -> AbilityDef {
    AbilityDef::static_ability(
        "This spell can't be countered.",
        EffectDef::Apply {
            recipient: EffectRecipientDef::Source,
            effect: AppliedEffectDef::CannotBeCountered,
            duration: EffectDurationDef::WhileSourceRemainsInZone,
        },
    )
    .with_source_zones(&[ZoneKind::Stack])
}

/// A common mana ability that taps its source to add one fixed kind of mana.
#[must_use]
pub const fn tap_for(mana: ManaColor) -> AbilityDef {
    let text = match mana {
        ManaColor::White => "{T}: Add {W}.",
        ManaColor::Blue => "{T}: Add {U}.",
        ManaColor::Black => "{T}: Add {B}.",
        ManaColor::Red => "{T}: Add {R}.",
        ManaColor::Green => "{T}: Add {G}.",
        ManaColor::Colorless => "{T}: Add {C}.",
    };
    AbilityDef::activated_mana(
        text,
        &[AbilityCostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::one(mana)),
    )
}

/// The shared replacement clause printed on shock lands.
#[must_use]
pub const fn shock_land_enters() -> AbilityDef {
    AbilityDef::as_enters(
        "As this land enters, you may pay 2 life. If you don't, it enters tapped.",
        ReplacementEffectDef::OptionalPayment {
            payment: PaymentDef::new(PlayerRelation::You, &PAY_TWO_LIFE),
            if_paid: &[],
            if_declined: &ENTER_TAPPED,
        },
    )
}

/// An unconditional battlefield-entry replacement shared by permanents that
/// enter tapped.
#[must_use]
pub const fn enters_tapped(text: &'static str) -> AbilityDef {
    AbilityDef::as_enters(text, ENTER_TAPPED[0])
}

/// A shared checkland-style entry clause backed by the general object-query
/// condition vocabulary.
#[must_use]
pub const fn check_land_enters(
    text: &'static str,
    land_types: &'static [BasicLandType],
) -> AbilityDef {
    enters_tapped_unless_you_control(text, ObjectPredicateDef::HasAnyBasicLandType(land_types))
}

/// An as-enters clause whose untapped branch depends on any controlled
/// battlefield object matching `object`.
#[must_use]
pub const fn enters_tapped_unless_you_control(
    text: &'static str,
    object: ObjectPredicateDef,
) -> AbilityDef {
    AbilityDef::as_enters(
        text,
        ReplacementEffectDef::Conditional {
            condition: ConditionDef::Exists(ObjectQueryDef {
                object,
                zones: &[ZoneKind::Battlefield],
                controller: PlayerRelation::You,
            }),
            if_true: &[],
            if_false: &ENTER_TAPPED,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::{
        banding, bloodrush, check_land_enters, double_strike, first_strike, flashback,
        flashback_for_card_mana_cost, flying, intimidate, overload, shock_land_enters, tap_for,
    };
    use crate::card::{
        AbilityCostDef, AbilityCostList, AbilityCoverageDef, AbilityDef, AddManaEffectDef,
        AlternativeCastKindDef, AlternativeCastManaCostDef, BasicLandType, CardRules, ConditionDef,
        CostDef, DeclarativeAbilityDef, EffectDef, KeywordAbility, ManaColor, ManaCost,
        ObjectPredicateDef, PlayerRelation, ReplacementEffectDef, ZoneKind,
    };
    use crate::mana_cost;

    #[test]
    fn tap_for_builds_a_complete_executable_mana_ability() {
        let cases = [
            (ManaColor::White, "{T}: Add {W}."),
            (ManaColor::Blue, "{T}: Add {U}."),
            (ManaColor::Black, "{T}: Add {B}."),
            (ManaColor::Red, "{T}: Add {R}."),
            (ManaColor::Green, "{T}: Add {G}."),
            (ManaColor::Colorless, "{T}: Add {C}."),
        ];

        for (mana, text) in cases {
            let ability = tap_for(mana);
            assert_eq!(ability.text, text);
            assert_eq!(ability.coverage, AbilityCoverageDef::complete());
            assert!(ability.is_executable());
            assert!(matches!(
                ability.definition,
                DeclarativeAbilityDef::ActivatedMana(definition)
                    if definition.costs.as_slice() == [AbilityCostDef::TapSource]
            ));
            assert_eq!(
                ability.declarative_effect(),
                Some(EffectDef::AddMana(AddManaEffectDef::one(mana)))
            );
        }
    }

    #[test]
    fn common_land_entry_abilities_use_shared_conditions_and_costs() {
        let shock = shock_land_enters();
        assert!(matches!(
            shock.declarative_effect(),
            Some(EffectDef::Replacement(ReplacementEffectDef::OptionalPayment {
                payment,
                if_declined: [_],
                ..
            })) if payment.payer == PlayerRelation::You
                && payment.costs == [CostDef::PayLife(2)]
        ));

        let check = check_land_enters(
            "This land enters tapped unless you control a Mountain or a Plains.",
            &[BasicLandType::Mountain, BasicLandType::Plains],
        );
        assert!(matches!(
            check.declarative_effect(),
            Some(EffectDef::Replacement(ReplacementEffectDef::Conditional {
                condition: ConditionDef::Exists(query),
                ..
            })) if query.controller == PlayerRelation::You
                && matches!(
                    query.object,
                    ObjectPredicateDef::HasAnyBasicLandType(types)
                        if types == [BasicLandType::Mountain, BasicLandType::Plains]
                )
        ));
    }

    #[test]
    fn keyword_presence_is_distinct_from_executable_keyword_support() {
        static KEYWORDS: [AbilityDef; 2] = [flying(), banding()];
        let rules =
            CardRules::new_creature(ManaCost::default(), &[], 1, 1).with_abilities(&KEYWORDS);

        assert!(rules.has_keyword(KeywordAbility::Flying));
        assert!(rules.has_executable_keyword(KeywordAbility::Flying));
        assert!(rules.has_keyword(KeywordAbility::Banding));
        assert!(!rules.has_executable_keyword(KeywordAbility::Banding));
    }

    #[test]
    fn common_combat_keywords_are_complete_definitions() {
        let cases = [
            (first_strike(), KeywordAbility::FirstStrike),
            (double_strike(), KeywordAbility::DoubleStrike),
            (intimidate(), KeywordAbility::Intimidate),
        ];

        for (ability, expected) in cases {
            assert_eq!(ability.coverage, AbilityCoverageDef::complete());
            assert!(ability.is_executable());
            assert_eq!(ability.definition, DeclarativeAbilityDef::Keyword(expected));
        }
        assert_eq!(intimidate().text, "Intimidate");
    }

    #[test]
    fn alternative_cast_helpers_own_costs_and_render_canonical_text() {
        let flashback = flashback(mana_cost!("{2}{U}"));
        let overload = overload(
            mana_cost!("{3}{R}{R}{R}"),
            "Deal 4 damage to each creature you don't control.",
            EffectDef::None,
        );

        assert!(matches!(
            flashback.definition,
            DeclarativeAbilityDef::AlternativeCast(definition)
                if definition.kind == AlternativeCastKindDef::Flashback
                    && definition.mana_cost
                        == AlternativeCastManaCostDef::Fixed(mana_cost!("{2}{U}"))
        ));
        assert_eq!(
            flashback.rules_text(),
            "Flashback {2}{U} (You may cast this card from your graveyard for its flashback cost. Then exile it.)",
        );
        assert!(matches!(
            overload.definition,
            DeclarativeAbilityDef::AlternativeCast(definition)
                if definition.kind == AlternativeCastKindDef::Overload
                    && definition.mana_cost
                        == AlternativeCastManaCostDef::Fixed(mana_cost!("{3}{R}{R}{R}"))
                    && definition.stack_text
                        == Some("Deal 4 damage to each creature you don't control.")
        ));
        assert_eq!(
            overload.rules_text(),
            "Overload {3}{R}{R}{R} (You may cast this spell for its overload cost. If you do, change \"target\" in its text to \"each.\")",
        );

        let granted = flashback_for_card_mana_cost();
        assert!(matches!(
            granted.definition,
            DeclarativeAbilityDef::AlternativeCast(definition)
                if definition.kind == AlternativeCastKindDef::Flashback
                    && definition.mana_cost == AlternativeCastManaCostDef::ThisCardManaCost
                    && definition.mana_cost.resolve(Some(mana_cost!("{1}{U}")))
                        == Some(mana_cost!("{1}{U}"))
        ));
        let DeclarativeAbilityDef::AlternativeCast(definition) = granted.definition else {
            unreachable!("the helper always builds an alternative-cast ability")
        };
        assert_eq!(definition.mana_cost.resolve(None), None);
    }

    #[test]
    fn bloodrush_owns_its_hand_zone_and_discard_procedure() {
        let effect = EffectDef::Special("Test Bloodrush effect");
        let text = "Bloodrush — {R}{G}, Discard this card: Test Bloodrush effect.";
        let ability = bloodrush(mana_cost!("{R}{G}"), text, &[], effect);
        let DeclarativeAbilityDef::Activated(definition) = ability.definition else {
            panic!("Bloodrush should be an activated ability")
        };

        assert_eq!(ability.text, text);
        assert_eq!(definition.source_zones, [ZoneKind::Hand]);
        assert_eq!(
            definition.costs,
            AbilityCostList::borrowed(&[
                AbilityCostDef::Mana(mana_cost!("{R}{G}")),
                AbilityCostDef::DiscardSource,
            ]),
            "inline and borrowed cost storage should compare by their costs",
        );
        assert_eq!(
            definition.costs.as_slice(),
            [
                AbilityCostDef::Mana(mana_cost!("{R}{G}")),
                AbilityCostDef::DiscardSource,
            ],
        );
        assert_eq!(ability.declarative_effect(), Some(effect));
    }
}
