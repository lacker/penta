#[cfg(test)]
mod tests {
    use super::{
        banding, bloodrush, check_land_enters, dies_trigger, dies_trigger_matching,
        dies_trigger_with_targets, double_strike, enchant_creature, enters_trigger,
        enters_trigger_with_targets, exile_until_next_end_step,
        exile_until_next_end_step_under_your_control, first_strike, flashback,
        flashback_for_card_mana_cost, flying, intimidate, living_weapon, overload, pain_land,
        shock_land_enters, tap_for, EQUIP_TARGET, equip,
    };
    use crate::card::{
        AbilityCostDef, AbilityCostList, AbilityCoverageDef, AbilityDef, AbilityPredicateDef,
        AbilityTargetDef, ActivationTimingDef, AddManaEffectDef, AlternativeCastKindDef,
        AlternativeCastManaCostDef, BasicLandType, CardRules, CardType, ConditionDef,
        DeclarativeAbilityDef, EffectDef, EffectPaymentCostDef, EffectRecipientDef, KeywordAbility,
        ManaColor, ManaCost, ObjectPredicateDef, PlayerRelation, PlayerSetDef,
        ReplacementEffectDef, TriggerEventDef, ZoneKind,
    };
    use crate::TargetIndex;
    use crate::mana_cost;

    #[test]
    fn living_weapon_owns_its_rules_defined_germ() {
        let Some(EffectDef::CreateAttachedToken { token, .. }) =
            living_weapon().declarative_effect()
        else {
            panic!("living weapon should create and attach its Germ")
        };
        let rules = token.rules();

        assert_eq!(token.name(), "Phyrexian Germ");
        assert!(rules.has_type(CardType::Creature));
        assert_eq!(rules.subtypes(), &["Phyrexian", "Germ"]);
        assert_eq!(rules.colors(), [false, false, true, false, false]);
        assert_eq!(rules.creature_stats().map(|stats| (stats.power, stats.toughness)), Some((0, 0)));
    }

    #[test]
    fn common_source_zone_triggers_use_shared_events_and_preserve_targets() {
        static TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::Any,
        )];
        let effect = EffectDef::Special("Test common source trigger");

        for ability in [
            enters_trigger("When this enters, test.", effect),
            enters_trigger_with_targets("When this enters, test target.", &TARGETS, effect),
        ] {
            let DeclarativeAbilityDef::Triggered(definition) = ability.definition else {
                panic!("enters helpers should build triggered abilities")
            };
            assert_eq!(
                definition.event,
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    None,
                    Some(ZoneKind::Battlefield),
                )
            );
        }

        for ability in [
            dies_trigger("When this dies, test.", effect),
            dies_trigger_with_targets("When this dies, test target.", &TARGETS, effect),
        ] {
            let DeclarativeAbilityDef::Triggered(definition) = ability.definition else {
                panic!("dies helpers should build triggered abilities")
            };
            assert_eq!(
                definition.event,
                TriggerEventDef::zone_changed(
                    ObjectPredicateDef::Source,
                    Some(ZoneKind::Battlefield),
                    Some(ZoneKind::Graveyard),
                )
            );
        }

        let DeclarativeAbilityDef::Triggered(enters) =
            enters_trigger("When this enters, test.", effect).definition
        else {
            unreachable!()
        };
        assert!(enters.targets.is_empty());
        let DeclarativeAbilityDef::Triggered(targeted_enters) =
            enters_trigger_with_targets("When this enters, test target.", &TARGETS, effect)
                .definition
        else {
            unreachable!()
        };
        assert_eq!(targeted_enters.targets, TARGETS);
        assert!(!targeted_enters.resolves_with_illegal_targets);
        let DeclarativeAbilityDef::Triggered(unfizzling_enters) =
            enters_trigger_with_targets("When this enters, test target.", &TARGETS, effect)
                .resolves_with_illegal_targets()
                .definition
        else {
            unreachable!()
        };
        assert!(unfizzling_enters.resolves_with_illegal_targets);

        let DeclarativeAbilityDef::Triggered(dies) =
            dies_trigger("When this dies, test.", effect).definition
        else {
            unreachable!()
        };
        assert!(dies.targets.is_empty());
        let DeclarativeAbilityDef::Triggered(targeted_dies) =
            dies_trigger_with_targets("When this dies, test target.", &TARGETS, effect).definition
        else {
            unreachable!()
        };
        assert_eq!(targeted_dies.targets, TARGETS);

        let artifact = ObjectPredicateDef::HasType(CardType::Artifact);
        let DeclarativeAbilityDef::Triggered(matching_dies) =
            dies_trigger_matching("When an artifact dies, test.", artifact, effect).definition
        else {
            unreachable!()
        };
        assert_eq!(
            matching_dies.event,
            TriggerEventDef::zone_changed(
                artifact,
                Some(ZoneKind::Battlefield),
                Some(ZoneKind::Graveyard),
            )
        );
    }

    #[test]
    fn common_aura_and_delayed_return_helpers_build_the_complete_clauses() {
        let aura = enchant_creature();
        let DeclarativeAbilityDef::Spell(spell) = aura.definition else {
            panic!("enchant creature should build an Aura spell clause")
        };
        assert_eq!(spell.targets(), super::ENCHANT_CREATURE_TARGET);
        assert_eq!(
            aura.declarative_effect(),
            Some(EffectDef::Attach {
                object: EffectRecipientDef::Target(TargetIndex::PRIMARY),
            })
        );

        let blink = exile_until_next_end_step(EffectRecipientDef::Target(TargetIndex::PRIMARY));
        let EffectDef::ExileUntilNextEndStep {
            object,
            return_ability,
        } = blink
        else {
            panic!("the blink helper should own the complete exile-and-return clause")
        };
        assert_eq!(
            object,
            EffectRecipientDef::Target(TargetIndex::PRIMARY)
        );
        let DeclarativeAbilityDef::Triggered(trigger) = return_ability.definition else {
            panic!("the nested return should be a triggered ability")
        };
        assert_eq!(
            trigger.event,
            TriggerEventDef::StepBegins {
                step: crate::card::TurnStepDef::End,
                player: PlayerRelation::Any,
            }
        );
        assert!(matches!(
            return_ability.declarative_effect(),
            Some(EffectDef::ReturnLinkedExiles {
                object: ObjectPredicateDef::Any,
                zone: ZoneKind::Battlefield,
                controller: None,
                ..
            })
        ));

        let controlled = exile_until_next_end_step_under_your_control(EffectRecipientDef::Source);
        let EffectDef::ExileUntilNextEndStep {
            object: EffectRecipientDef::Source,
            return_ability,
        } = controlled
        else {
            panic!("Venser's blink helper should use the same complete abstraction")
        };
        assert!(matches!(
            return_ability.declarative_effect(),
            Some(EffectDef::ReturnLinkedExiles {
                object: ObjectPredicateDef::Any,
                zone: ZoneKind::Battlefield,
                controller: Some(PlayerRelation::You),
                ..
            })
        ));
    }

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
        assert!(AbilityPredicateDef::Flashback.matches(&flashback));
        assert!(!AbilityPredicateDef::Flashback.matches(&overload));
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
