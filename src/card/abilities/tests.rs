#[cfg(test)]
mod tests {
    use super::{
        banding, bloodrush, check_land_enters, double_strike, first_strike, flashback,
        flashback_for_card_mana_cost, flying, intimidate, overload, pain_land, shock_land_enters,
        tap_for, EQUIP_TARGET, equip,
    };
    use crate::card::{
        AbilityCostDef, AbilityCostList, AbilityCoverageDef, AbilityDef, ActivationTimingDef,
        AddManaEffectDef, AlternativeCastKindDef, AlternativeCastManaCostDef, BasicLandType,
        CardRules, ConditionDef, DeclarativeAbilityDef, EffectDef, EffectPaymentCostDef,
        EffectRecipientDef, KeywordAbility, ManaColor, ManaCost, ObjectPredicateDef,
        PlayerRelation, PlayerSetDef, ReplacementEffectDef, ZoneKind,
    };
    use crate::TargetIndex;
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
    fn pain_land_keeps_damage_on_only_the_colored_ability() {
        let abilities = pain_land(
            "{T}: Add {W} or {U}. This land deals 1 damage to you.",
            &[ManaColor::White, ManaColor::Blue],
        );

        assert_eq!(
            abilities[0].declarative_effect(),
            Some(EffectDef::AddMana(AddManaEffectDef::one(
                ManaColor::Colorless
            )))
        );
        assert_eq!(
            abilities[1].declarative_effect(),
            Some(EffectDef::AddMana(
                AddManaEffectDef::choice(&[ManaColor::White, ManaColor::Blue])
                    .with_damage_to_controller(1)
            ))
        );
    }

    #[test]
    fn common_land_entry_abilities_use_shared_conditions_and_costs() {
        let shock = shock_land_enters();
        assert!(matches!(
            shock.declarative_replacement(),
            Some(ReplacementEffectDef::PayOr {
                payment,
                if_declined: [_],
                ..
            }) if payment.payer == PlayerSetDef::Related(PlayerRelation::You)
                && payment.cost == EffectPaymentCostDef::Life(2)
        ));

        let check = check_land_enters(
            "This land enters tapped unless you control a Mountain or a Plains.",
            &[BasicLandType::Mountain, BasicLandType::Plains],
        );
        assert!(matches!(
            check.declarative_replacement(),
            Some(ReplacementEffectDef::Conditional {
                condition: ConditionDef::Exists(query),
                ..
            }) if query.related_player
                == Some(PlayerSetDef::Related(PlayerRelation::You))
                && matches!(
                    query.object,
                    ObjectPredicateDef::HasAnyBasicLandType(types)
                        if types == [BasicLandType::Mountain, BasicLandType::Plains]
                )
        ));
    }

    /// A card can print a keyword the engine only records. The distinction is
    /// a property of the coverage model rather than of any particular keyword,
    /// so the metadata-only case is built here instead of borrowing whichever
    /// keyword happens to be unimplemented today.
    #[test]
    fn keyword_presence_is_distinct_from_executable_keyword_support() {
        static RECORDED_ONLY: AbilityDef = AbilityDef::keyword("Shroud", KeywordAbility::Shroud)
            .with_coverage(AbilityCoverageDef::metadata_only(
                "Recorded for this test, not executed.",
            ));
        static KEYWORDS: [AbilityDef; 2] = [flying(), RECORDED_ONLY];
        let rules =
            CardRules::new_creature(ManaCost::default(), &[], 1, 1).with_abilities(&KEYWORDS);

        assert!(rules.has_keyword(KeywordAbility::Flying));
        assert!(rules.has_executable_keyword(KeywordAbility::Flying));
        assert!(rules.has_keyword(KeywordAbility::Shroud));
        assert!(!rules.has_executable_keyword(KeywordAbility::Shroud));
    }

    /// Banding is the widest keyword the engine executes: a declaration rule,
    /// a blocking rule, and a damage-assignment rule in both directions.
    #[test]
    fn banding_is_executable_and_completely_covered() {
        assert!(banding().is_executable());
        assert_eq!(banding().coverage, AbilityCoverageDef::complete());
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

    #[test]
    fn equip_preserves_a_mixed_ordered_cost_list_and_shared_procedure() {
        static COSTS: [AbilityCostDef; 3] = [
            AbilityCostDef::Mana(mana_cost!("{2}")),
            AbilityCostDef::TapSource,
            AbilityCostDef::PayLife(1),
        ];
        let ability = equip(&COSTS, "{2}, {T}, Pay 1 life: Equip test creature.");
        let DeclarativeAbilityDef::Activated(definition) = ability.definition else {
            panic!("Equip should be an activated ability")
        };

        assert_eq!(
            definition.costs.as_slice(),
            COSTS,
            "mana and distinct nonmana costs retain their printed order",
        );
        assert_eq!(definition.targets, EQUIP_TARGET);
        assert_eq!(definition.timing, ActivationTimingDef::SorcerySpeed);
        assert_eq!(
            ability.declarative_effect(),
            Some(EffectDef::Attach {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            }),
        );
    }
}
