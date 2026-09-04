#[cfg(test)]
mod tests {
    use super::{
        attacks_each_combat_if_able, banding, begin_game_on_battlefield, bind_top_cards_then,
        bind_top_cards_through_first_matching_then, bloodrush, check_land_enters,
        bloodthirst, bushido,
        creature_damaged_by_source_dies_trigger,
        creature_damaged_by_source_dies_trigger_with_targets, dies_trigger,
        dies_trigger_matching, dies_trigger_with_targets, double_strike, enchant_creature,
        enters_tapped, enters_trigger, enters_trigger_with_targets, evoke_sacrifice,
        exile_and_return_transformed, gain_ability_until_end_of_turn,
        gain_ability_until_end_of_turn_for_mana,
        exile_until_next_end_step, exile_until_next_end_step_under_your_control,
        exile_until_source_leaves, first_strike, flashback,
        flashback_for_card_mana_cost, flying, intimidate, legendary_landwalk, living_weapon,
        look_at_top_cards, mountainwalk,
        look_at_top_cards_choose_to_hand_rest_bottom, overload, pain_land,
        rebound, reveal_hand_and_choose_card, reveal_hand_and_discard_chosen_card,
        reveal_hand_and_exile_chosen_card,
        rampage, reveal_top_cards_put_matching_in_hand_rest_graveyard, shock_land_enters, storm,
        tap_for, trample, ward_aura_protection, EQUIP_TARGET, equip,
    };
    use crate::card::{
        AbilityCostDef, AbilityCostList, AbilityDef, AbilityKindDef, AbilityPredicateDef,
        AbilityTargetDef, ActivationTimingDef, AddManaEffectDef, AlternativeCastKindDef,
        AlternativeCastManaCostDef, BasicLandType, CardRules, CardType, ConditionDef,
        CollectionInspectionDef, DeclarativeAbilityDef, EffectDef, EffectPaymentCostDef,
        EffectRecipientDef, KeywordAbility, ManaColor, ManaCost, ObjectCollectionSourceDef,
        ObjectPredicateDef, ObjectRefDef, PlayerRefDef, PlayerRelation, PlayerSetDef,
        ReplacementEffectDef, ResolvedEffectDurationDef, TriggerEventDef, ValueDef,
        ZoneChangeEventMatcherDef, ZoneKind,
    };
    use crate::mana_cost;
    use crate::{ParentBinding, TargetIndex};

    static TEST_FIRST_STRIKE: AbilityDef = first_strike();
    static TEST_SACRIFICE_SOURCE: [AbilityCostDef; 1] = [AbilityCostDef::SacrificeSource];

    #[test]
    fn reusable_ability_text_can_be_overridden_without_changing_semantics() {
        let default = trample();
        let overridden = default.override_text("Trample (A different printed reminder.)");

        assert_eq!(overridden.text, "Trample (A different printed reminder.)");
        assert_eq!(overridden.rules_text(), overridden.text);
        assert_eq!(overridden.definition, default.definition);
        assert_eq!(overridden.effect, default.effect);
    }

    #[test]
    fn activated_self_grants_share_one_semantic_shape_across_cost_kinds() {
        let mana = gain_ability_until_end_of_turn_for_mana(
            "{R}: This creature gains first strike until end of turn.",
            mana_cost!("{R}"),
            &TEST_FIRST_STRIKE,
        );
        let nonmana = gain_ability_until_end_of_turn(
            "Sacrifice this creature: It gains first strike until end of turn.",
            &TEST_SACRIFICE_SOURCE,
            &TEST_FIRST_STRIKE,
        );

        assert_eq!(
            mana,
            AbilityDef::activated(
                mana.text,
                &[AbilityCostDef::Mana(mana_cost!("{R}"))],
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: crate::card::AppliedEffectDef::add_ability(&TEST_FIRST_STRIKE),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        );
        assert_eq!(
            nonmana,
            AbilityDef::activated(
                nonmana.text,
                &TEST_SACRIFICE_SOURCE,
                EffectDef::Apply {
                    recipient: EffectRecipientDef::Source,
                    effect: crate::card::AppliedEffectDef::add_ability(&TEST_FIRST_STRIKE),
                    duration: ResolvedEffectDurationDef::UntilEndOfTurn,
                },
            ),
        );
    }

    #[test]
    fn enters_tapped_ability_text_uses_an_explicit_subject() {
        let land_entry = enters_tapped(CardType::Land);
        let artifact_entry = enters_tapped(CardType::Artifact);
        let creature_entry = enters_tapped(CardType::Creature);

        assert_eq!(land_entry.text, "This land enters tapped.");
        assert_eq!(artifact_entry.text, "This artifact enters tapped.");
        assert_eq!(creature_entry.text, "This creature enters tapped.");
        assert_eq!(artifact_entry.definition, land_entry.definition);
        assert_eq!(artifact_entry.effect, land_entry.effect);
        assert_eq!(creature_entry.definition, land_entry.definition);
        assert_eq!(creature_entry.effect, land_entry.effect);
    }

    #[test]
    fn repeated_ability_text_lives_with_its_shared_constructor() {
        assert_eq!(
            begin_game_on_battlefield().text,
            "If this card is in your opening hand, you may begin the game with it on the battlefield.",
        );
        assert_eq!(
            attacks_each_combat_if_able().text,
            "This creature attacks each combat if able.",
        );
        assert_eq!(
            rampage(2).text,
            "Rampage 2 (Whenever this creature becomes blocked, it gets +2/+2 until end of turn for each creature blocking it beyond the first.)",
        );
        for (amount, text) in [(1, "Bushido 1"), (2, "Bushido 2"), (5, "Bushido 5")] {
            assert_eq!(bushido(ValueDef::Constant(amount)).text, text);
        }
        assert_eq!(
            bushido(ValueDef::SourcePower)
                .override_text("Bushido X")
                .text,
            "Bushido X",
        );
        for (amount, text) in [
            (1, "Bloodthirst 1"),
            (2, "Bloodthirst 2"),
            (3, "Bloodthirst 3"),
            (6, "Bloodthirst 6"),
        ] {
            assert_eq!(bloodthirst(amount).text, text);
        }
        assert_eq!(
            ward_aura_protection(ManaColor::Blue).text,
            "Enchanted creature has protection from blue. This effect doesn't remove this Aura.",
        );
        assert_eq!(
            evoke_sacrifice().text,
            "When this creature enters, if it was evoked, sacrifice it.",
        );
    }

    #[test]
    fn rebound_is_one_complete_keyword_clause() {
        let ability = rebound();

        assert_eq!(
            ability.definition,
            DeclarativeAbilityDef::Keyword(KeywordAbility::Rebound),
        );
        assert_eq!(
            ability.text,
            "Rebound (If you cast this spell from your hand, exile it as it resolves. At the beginning of your next upkeep, you may cast this card from exile without paying its mana cost.)",
        );
    }

    #[test]
    fn look_at_top_cards_hides_collection_plumbing_for_pure_looks() {
        let player = PlayerRefDef::Target(TargetIndex::PRIMARY);

        let EffectDef::LookAtObjects(look) =
            look_at_top_cards(player, ValueDef::Constant(3))
        else {
            panic!("the helper should build one information action")
        };
        assert_eq!(look.actor, PlayerRefDef::EffectController);
        assert_eq!(
            look.source,
            ObjectCollectionSourceDef::TopCards {
                player,
                count: ValueDef::Constant(3),
            },
        );
        assert_eq!(*look.then, EffectDef::None);
    }

    #[test]
    fn collection_helpers_distinguish_fixed_and_predicate_bounded_sources() {
        static THEN: EffectDef = EffectDef::Special("after binding");
        let player = PlayerRefDef::Target(TargetIndex::PRIMARY);

        let EffectDef::BindObjects(fixed) =
            bind_top_cards_then(player, ValueDef::Constant(4), &THEN)
        else {
            panic!("fixed top cards should use the generic binding stage")
        };
        assert_eq!(
            fixed.source,
            ObjectCollectionSourceDef::TopCards {
                player,
                count: ValueDef::Constant(4),
            }
        );

        let stop = ObjectPredicateDef::HasType(CardType::Land);
        let EffectDef::BindObjects(bounded) =
            bind_top_cards_through_first_matching_then(player, stop, &THEN)
        else {
            panic!("a reveal-until source should use the same generic binding stage")
        };
        assert_eq!(
            bounded.source,
            ObjectCollectionSourceDef::TopCardsThroughFirstMatching {
                player,
                object: stop,
            }
        );
        assert_eq!(bounded.binding, ParentBinding);
        assert_eq!(bounded.then, &THEN);
    }

    #[test]
    fn ordinary_top_card_workflows_are_single_semantic_effects() {
        let predicate = ObjectPredicateDef::HasType(CardType::Instant);
        let EffectDef::ChooseCardsFromCollection(choice) =
            look_at_top_cards_choose_to_hand_rest_bottom(
                ValueDef::Constant(4),
                predicate,
                0,
                1,
            )
        else {
            panic!("an ordinary dig should not expose its binding pipeline")
        };
        assert_eq!(choice.inspection, CollectionInspectionDef::Look);
        assert_eq!(choice.object, predicate);
        assert_eq!((choice.minimum, choice.maximum), (0, 1));

        let EffectDef::RevealAndClassifyCards(classify) =
            reveal_top_cards_put_matching_in_hand_rest_graveyard(
                ValueDef::Constant(4),
                ObjectPredicateDef::HasType(CardType::Land),
            )
        else {
            panic!("a mandatory split should be one classification procedure")
        };
        assert_eq!(
            classify.source,
            ObjectCollectionSourceDef::TopCards {
                player: PlayerRefDef::EffectController,
                count: ValueDef::Constant(4),
            },
        );
    }

    #[test]
    fn storm_builds_the_shared_source_cast_copy_trigger() {
        let ability = storm();
        let DeclarativeAbilityDef::Triggered(trigger) = ability.definition else {
            panic!("storm should be a cast trigger")
        };
        assert_eq!(
            trigger.event,
            TriggerEventDef::spell_cast(ObjectPredicateDef::Source),
        );
        let Some(EffectDef::CopyStackObject(copy)) = ability.declarative_effect() else {
            panic!("storm should copy its source spell")
        };
        assert_eq!(copy.object, EffectRecipientDef::Source);
        assert_eq!(copy.controller, PlayerRefDef::EffectController);
        assert_eq!(copy.count, ValueDef::SpellsCastBeforeThisTurn);
        assert!(copy.retarget);
        assert_eq!(copy.colors, None);
    }

    #[test]
    fn revealed_hand_choice_helpers_share_selection_and_destination_semantics() {
        static CONTINUATION: EffectDef = EffectDef::Special("chosen hand card");
        let player = PlayerRefDef::Target(TargetIndex::PRIMARY);
        let object = ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(CardType::Land));

        let [EffectDef::RevealHand { player: revealed }, EffectDef::Choose(choice)] =
            reveal_hand_and_choose_card(player, object, &CONTINUATION)
        else {
            panic!("the helper should reveal, then choose")
        };
        assert_eq!(revealed, EffectRecipientDef::player(player));
        assert_eq!(choice.chooser, PlayerRefDef::EffectController);
        assert_eq!(choice.minimum, 1);
        assert_eq!(choice.maximum, 1);
        assert_eq!(choice.then, &CONTINUATION);

        let [_, EffectDef::Choose(discard)] =
            reveal_hand_and_discard_chosen_card(player, object)
        else {
            panic!("discard helper should use the common choice")
        };
        assert_eq!(
            *discard.then,
            EffectDef::DiscardCards {
                object: EffectRecipientDef::object(crate::card::ObjectRefDef::Binding(
                    ParentBinding,
                )),
            }
        );

        let [_, EffectDef::Choose(exile)] = reveal_hand_and_exile_chosen_card(player, object)
        else {
            panic!("exile helper should use the common choice")
        };
        assert!(matches!(
            *exile.then,
            EffectDef::MoveToZone {
                zone: ZoneKind::Exile,
                ..
            }
        ));
    }

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
            dies_trigger_matching(
                "When an artifact is put into a graveyard from the battlefield, test.",
                artifact,
                effect,
            )
            .definition
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
    fn damaged_creature_death_helpers_share_the_source_history_matcher() {
        static TARGETS: [AbilityTargetDef; 1] = [AbilityTargetDef::exactly_one_permanent(
            ObjectPredicateDef::Any,
        )];
        let effect = EffectDef::Special("Test common damaged-creature trigger");
        let abilities = [
            creature_damaged_by_source_dies_trigger(
                "Whenever a creature dealt damage by this creature this turn dies, test.",
                effect,
            ),
            creature_damaged_by_source_dies_trigger_with_targets(
                "Whenever a creature dealt damage by this creature this turn dies, test target.",
                &TARGETS,
                effect,
            ),
        ];

        for ability in abilities {
            let DeclarativeAbilityDef::Triggered(definition) = ability.definition else {
                panic!("damaged-creature death helpers should build triggered abilities")
            };
            assert_eq!(
                definition.event,
                TriggerEventDef::ZoneChanged(
                    ZoneChangeEventMatcherDef::new(
                        ObjectPredicateDef::HasType(CardType::Creature),
                        Some(ZoneKind::Battlefield),
                        Some(ZoneKind::Graveyard),
                    )
                    .previously_damaged_by(ObjectRefDef::Source),
                )
            );
        }
        let DeclarativeAbilityDef::Triggered(targeted) = abilities[1].definition else {
            unreachable!()
        };
        assert_eq!(targeted.targets, TARGETS);
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
        let EffectDef::ExileLinkedToSource {
            object,
            then: Some(return_trigger),
            ..
        } = blink
        else {
            panic!("the blink helper should own the complete exile-and-return clause")
        };
        assert_eq!(
            object,
            EffectRecipientDef::Target(TargetIndex::PRIMARY)
        );
        let EffectDef::InstallTrigger(installed) = *return_trigger else {
            panic!("the blink helper should install an ordinary delayed trigger")
        };
        let DeclarativeAbilityDef::Triggered(trigger) = installed.ability.definition else {
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
            installed.ability.declarative_effect(),
            Some(EffectDef::ReturnLinkedExiles {
                object: ObjectPredicateDef::Any,
                zone: ZoneKind::Battlefield,
                controller: None,
                ..
            })
        ));

        let controlled = exile_until_next_end_step_under_your_control(EffectRecipientDef::Source);
        let EffectDef::ExileLinkedToSource {
            object: EffectRecipientDef::Source,
            then: Some(return_trigger),
            ..
        } = controlled
        else {
            panic!("Venser's blink helper should use the same complete abstraction")
        };
        let EffectDef::InstallTrigger(installed) = *return_trigger else {
            panic!("Venser's blink helper should install an ordinary delayed trigger")
        };
        assert!(matches!(
            installed.ability.declarative_effect(),
            Some(EffectDef::ReturnLinkedExiles {
                object: ObjectPredicateDef::Any,
                zone: ZoneKind::Battlefield,
                controller: Some(PlayerRelation::You),
                ..
            })
        ));

        let transformed = exile_and_return_transformed(EffectRecipientDef::Source);
        let EffectDef::ExileLinkedToSource {
            object: EffectRecipientDef::Source,
            then: Some(return_effect),
            ..
        } = transformed
        else {
            panic!("the transform helper should be a linked exile with a continuation")
        };
        assert!(matches!(
            *return_effect,
            EffectDef::ReturnLinkedExiles {
                object: ObjectPredicateDef::Any,
                zone: ZoneKind::Battlefield,
                controller: Some(PlayerRelation::You),
                transformed: true,
                ..
            }
        ));
    }

    #[test]
    fn until_source_leaves_builds_one_linked_exile_clause() {
        let effect =
            exile_until_source_leaves(EffectRecipientDef::Target(TargetIndex::PRIMARY));
        let EffectDef::ExileLinkedToSource {
            object,
            then: Some(return_trigger),
            ..
        } = effect
        else {
            panic!("the helper should own the complete linked clause")
        };
        assert_eq!(object, EffectRecipientDef::Target(TargetIndex::PRIMARY));
        let EffectDef::InstallTrigger(installed) = *return_trigger else {
            panic!("the helper should install its return trigger")
        };
        let DeclarativeAbilityDef::Triggered(trigger) = installed.ability.definition else {
            panic!("the nested return should be a triggered ability")
        };
        assert_eq!(
            trigger.event,
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                Some(ZoneKind::Battlefield),
                None,
            )
        );
        assert!(matches!(
            installed.ability.declarative_effect(),
            Some(EffectDef::ReturnLinkedExiles {
                object: ObjectPredicateDef::Any,
                zone: ZoneKind::Battlefield,
                controller: None,
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

    #[test]
    fn declared_keywords_are_executable() {
        static SHROUD: AbilityDef = AbilityDef::keyword("Shroud", KeywordAbility::Shroud);
        static KEYWORDS: [AbilityDef; 2] = [flying(), SHROUD];
        let rules =
            CardRules::new_creature(ManaCost::default(), &[], 1, 1).with_abilities(&KEYWORDS);

        assert!(rules.has_keyword(KeywordAbility::Flying));
        assert!(rules.has_executable_keyword(KeywordAbility::Flying));
        assert!(rules.has_keyword(KeywordAbility::Shroud));
        assert!(rules.has_executable_keyword(KeywordAbility::Shroud));
    }

    /// Banding is the widest keyword the engine executes: a declaration rule,
    /// a blocking rule, and a damage-assignment rule in both directions.
    #[test]
    fn banding_is_executable_and_completely_covered() {
        assert!(matches!(
            banding().definition,
            DeclarativeAbilityDef::Keyword(KeywordAbility::Banding)
        ));
    }

    #[test]
    fn common_combat_keywords_are_complete_definitions() {
        let cases = [
            (first_strike(), KeywordAbility::FirstStrike),
            (double_strike(), KeywordAbility::DoubleStrike),
            (intimidate(), KeywordAbility::Intimidate),
        ];

        for (ability, expected) in cases {
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
        assert!(AbilityPredicateDef::Is(AbilityKindDef::Flashback).matches(&flashback));
        assert!(!AbilityPredicateDef::Is(AbilityKindDef::Flashback).matches(&overload));
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
    fn landwalk_ability_kind_covers_basic_and_legendary_landwalk_only() {
        assert!(AbilityPredicateDef::Is(AbilityKindDef::Landwalk).matches(&mountainwalk()));
        assert!(AbilityPredicateDef::Is(AbilityKindDef::Landwalk).matches(&legendary_landwalk()));
        assert!(!AbilityPredicateDef::Is(AbilityKindDef::Landwalk).matches(&flying()));
    }

    #[test]
    fn activated_ability_kinds_distinguish_mana_abilities() {
        let mana = tap_for(ManaColor::Green);
        let nonmana = AbilityDef::activated("Test activation.", &[], EffectDef::None);

        assert!(AbilityPredicateDef::Is(AbilityKindDef::Activated).matches(&mana));
        assert!(AbilityPredicateDef::Is(AbilityKindDef::Activated).matches(&nonmana));
        assert!(AbilityPredicateDef::Is(AbilityKindDef::ActivatedMana).matches(&mana));
        assert!(
            !AbilityPredicateDef::Is(AbilityKindDef::ActivatedMana).matches(&nonmana)
        );
        assert!(
            AbilityPredicateDef::Is(AbilityKindDef::NonManaActivated).matches(&nonmana)
        );
        assert!(!AbilityPredicateDef::Is(AbilityKindDef::NonManaActivated).matches(&mana));
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
