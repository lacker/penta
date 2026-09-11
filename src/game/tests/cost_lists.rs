use super::*;
use crate::card::sets;

#[test]
fn echo_accepts_a_discard_cost_and_sacrifices_when_it_is_not_paid() {
    for (has_card, pay) in [(false, false), (true, false), (true, true)] {
        let mut game = ready_game();
        game.battlefield.clear();
        game.players[0].hand.clear();
        game.players[0].graveyard.clear();
        game.players[0].mana_pool = ManaPool::default();
        let mut imp = creature(230_001, cards::DEEPCAVERN_IMP, PlayerId::One);
        let id = imp.card.id;
        imp.entered_controller_turn = 5;
        game.battlefield.push(imp);
        if has_card {
            game.players[0]
                .hand
                .push(card(230_002, cards::SWAMP, PlayerId::One));
        }
        game.turns_started[0] = 6;
        game.turn = 6;
        game.active_player = PlayerId::One;
        game.step = Step::Upkeep;
        game.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerId::One,
        });
        for _ in 0..24 {
            if let Some(decision) = game
                .pending_decisions
                .first()
                .map(|p| p.observation.clone())
            {
                let option = if pay {
                    decision.options.last()
                } else {
                    decision.options.first()
                }
                .expect("a payment decision has an option");
                game.apply(
                    decision.player,
                    Action::ChooseDecision {
                        decision: decision.id,
                        options: vec![option.id],
                    },
                )
                .unwrap();
            } else if game.stack.is_empty() && game.pending_triggers.is_empty() {
                break;
            } else {
                game.apply(game.priority, Action::PassPriority).unwrap();
            }
        }
        assert_eq!(game.battlefield.iter().any(|p| p.card.id == id), pay);
        assert_eq!(game.players[0].hand.len(), usize::from(has_card && !pay));
        assert_eq!(game.players[0].mana_pool, ManaPool::default());
    }
}

#[test]
fn resolving_cost_lists_pay_mana_life_and_discard_as_one_payment() {
    static COSTS: [CostDef; 3] = [
        CostDef::Mana(mana_cost!("{1}")),
        CostDef::PayLife(2),
        CostDef::DiscardCards(1),
    ];
    static PAID: EffectDef = EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(5),
    };
    for has_card in [false, true] {
        let mut game = ready_game();
        game.players[0].hand.clear();
        game.players[0].graveyard.clear();
        game.players[0].mana_pool = ManaPool::default();
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
        if has_card {
            game.players[0]
                .hand
                .push(card(230_010, cards::SWAMP, PlayerId::One));
        }
        let source = spell(230_011, cards::LIGHTNING_BOLT, PlayerId::One, 0);
        game.resolve_effect_def(
            ScopedEffect::primary(EffectDef::PayOr(PayOrDef::optional(&COSTS, &PAID))),
            &source,
            TriggerContext::empty(),
        );
        let decision = game.observe(PlayerId::One).decision.unwrap();
        assert_eq!(decision.options.len(), if has_card { 2 } else { 1 });
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: decision.id,
                options: vec![u32::from(has_card)],
            },
        )
        .unwrap();
        assert_eq!(game.players[0].life, if has_card { 23 } else { 20 });
        assert!(game.players[0].hand.is_empty());
        assert_eq!(game.players[0].graveyard.len(), usize::from(has_card));
        assert_eq!(game.players[0].mana_pool.colorless, u16::from(!has_card));
    }
}

#[test]
fn resolving_cost_lists_cannot_discard_the_same_card_twice() {
    let mut game = ready_game();
    game.players[0].hand.clear();
    game.players[0]
        .hand
        .push(card(230_020, cards::SWAMP, PlayerId::One));
    let payment = ResolvedEffectPayment::all(vec![
        ResolvedEffectPayment::Discard(1),
        ResolvedEffectPayment::Discard(1),
    ]);
    assert!(!game.can_pay_effect_payment(PlayerId::One, payment.clone()));
    assert!(!game.pay_effect_payment(PlayerId::One, payment));
    assert_eq!(game.players[0].hand.len(), 1);
}

#[test]
fn resolving_cost_lists_require_distinct_permanents_for_repeated_returns() {
    static COSTS: [CostDef; 2] = [
        CostDef::MovePermanentMatching {
            object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
            zone: ZoneKind::Hand,
        },
        CostDef::MovePermanentMatching {
            object: ObjectPredicateDef::HasAnyBasicLandType(&[BasicLandType::Forest]),
            zone: ZoneKind::Hand,
        },
    ];
    static PAID: EffectDef = EffectDef::GainLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    };

    for forests in [1, 2] {
        let mut game = ready_game();
        game.battlefield.clear();
        game.players[0].hand.clear();
        for index in 0..forests {
            game.battlefield
                .push(creature(230_030 + index, cards::FOREST, PlayerId::One));
        }
        let source = spell(230_032, cards::LIGHTNING_BOLT, PlayerId::One, 0);
        game.resolve_effect_def(
            ScopedEffect::primary(EffectDef::PayOr(PayOrDef::optional(&COSTS, &PAID))),
            &source,
            TriggerContext::empty(),
        );
        let decision = game.observe(PlayerId::One).decision.unwrap();

        if forests == 1 {
            assert_eq!(decision.options.len(), 1, "one Forest cannot pay twice");
            continue;
        }

        let payment = decision.options.last().expect("two Forests can pay");
        assert_eq!(payment.members.len(), 2);
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: decision.id,
                options: vec![payment.id],
            },
        )
        .unwrap();
        assert!(game.battlefield.is_empty());
        assert_eq!(game.players[0].hand.len(), 2);
        assert_eq!(game.players[0].life, 21);
    }
}

#[test]
fn free_static_alternative_and_zero_mana_alternative_remain_distinct() {
    let costs_of = |id| {
        ready_game()
            .catalog
            .get(id)
            .unwrap()
            .rules
            .ability_clauses()
            .iter()
            .find_map(|ability| match ability.declarative_effect() {
                Some(EffectDef::ModifyCost(CostModificationDef::SpellAlternative {
                    costs,
                    ..
                })) => Some(costs),
                _ => None,
            })
            .unwrap()
    };
    let free = costs_of(cards::OMNISCIENCE);
    let zero = costs_of(cards::ROOFTOP_STORM);
    assert!(free.is_empty());
    assert!(!crate::card::costs::includes_mana_payment(free));
    assert!(crate::card::costs::includes_mana_payment(zero));
    assert_ne!(free, zero);
    assert_eq!(
        crate::card::costs::mana_cost(free, None),
        crate::card::costs::mana_cost(zero, None)
    );
}

pub(in crate::game) fn game_with_cost_rules(rules: &CardRules) -> (Game, GameObjectId) {
    let definition_id = CardDefinitionId::from_uuid("00000000-0000-0000-0000-0000000186a1");
    let mut definition = CardDefinition::new(
        definition_id,
        "Cost list fixture",
        sets::magic_2014::SET,
        CardRules::unsupported(),
    );
    definition.rules = *rules;
    synchronize_single_part_definition(&mut definition);
    let mut game = ready_game();
    let mut definitions = game
        .catalog
        .definitions()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    definitions.push(definition);
    game.catalog = CardCatalog::new(definitions).unwrap();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].graveyard.clear();
    game.players[0].mana_pool = ManaPool::default();
    game.players[0]
        .hand
        .push(card(230_100, definition_id, PlayerId::One));
    (game, GameObjectId(230_100))
}

fn cast_mana_presence(game: &Game, action: &Action) -> bool {
    let Action::CastSpell {
        card,
        choices,
        sacrifices,
    } = action
    else {
        panic!("expected cast")
    };
    let (signature, _, source_zone) = game
        .validated_cast_signature(PlayerId::One, *card, choices, sacrifices)
        .unwrap();
    game.cast_object_payments_and_life(
        PlayerId::One,
        *card,
        &signature,
        CastCostContext {
            source_zone,
            offer: None,
        },
        sacrifices,
    )
    .2
}

#[test]
fn omniscience_mana_window_depends_on_the_complete_selected_payment() {
    for (cost, expected_mana, expected_life) in [
        (CostDef::PayLife(1), false, 19),
        (CostDef::Mana(mana_cost!("{0}")), true, 20),
        (CostDef::Mana(mana_cost!("{1}")), true, 20),
    ] {
        let (mut game, id) =
            game_with_cost_rules(&CardRules::new_sorcery(mana_cost!("{9}")).with_ability(
                AbilityDef::spell_with_additional_cost(
                    "Pay an additional cost.",
                    &[],
                    cost,
                    EffectDef::None,
                ),
            ));
        game.battlefield
            .push(creature(230_101, cards::OMNISCIENCE, PlayerId::One));
        game.battlefield
            .push(creature(230_102, cards::SWAMP, PlayerId::One));
        let action = game.legal_actions(PlayerId::One).into_iter().find(|action| matches!(action, Action::CastSpell { card, choices, .. } if *card == id && choices.costs().alternative().is_some())).unwrap();
        assert_eq!(cast_mana_presence(&game, &action), expected_mana);
        game.apply(PlayerId::One, action).unwrap();
        assert_eq!(game.players[0].life, expected_life);
        assert_eq!(
            game.battlefield
                .iter()
                .find(|p| p.card.id == GameObjectId(230_102))
                .unwrap()
                .tapped,
            matches!(cost, CostDef::Mana(mana) if mana.generic == 1)
        );
    }
}

#[test]
fn a_tax_reduced_to_zero_still_includes_a_mana_payment() {
    let (mut game, id) = game_with_cost_rules(
        &CardRules::new_sorcery(mana_cost!("{9}"))
            .with_ability(AbilityDef::spell("Do nothing.", EffectDef::None)),
    );
    game.battlefield
        .push(creature(230_101, cards::OMNISCIENCE, PlayerId::One));
    let free_cast = |game: &Game| {
        game.legal_actions(PlayerId::One).into_iter().find(|action| matches!(action, Action::CastSpell { card, choices, .. } if *card == id && choices.costs().alternative().is_some())).unwrap()
    };
    assert!(!cast_mana_presence(&game, &free_cast(&game)));
    game.battlefield.push(creature(
        230_102,
        cards::SPHERE_OF_RESISTANCE,
        PlayerId::One,
    ));
    game.battlefield
        .push(creature(230_103, cards::HELM_OF_AWAKENING, PlayerId::One));
    let action = free_cast(&game);
    assert!(cast_mana_presence(&game, &action));
    game.apply(PlayerId::One, action).unwrap();
    assert_eq!(game.players[0].mana_pool, ManaPool::default());
}

#[test]
fn repeatable_nonmana_costs_are_bounded_by_their_resources() {
    static ABILITIES: [AbilityDef; 2] = [
        AbilityDef::spell("Do nothing.", EffectDef::None),
        abilities::multikicker(&[CostDef::DiscardCards(1)]),
    ];
    let (mut game, id) =
        game_with_cost_rules(&CardRules::new_sorcery(mana_cost!("{0}")).with_abilities(&ABILITIES));
    game.players[0]
        .hand
        .push(card(230_101, cards::SWAMP, PlayerId::One));
    game.players[0]
        .hand
        .push(card(230_102, cards::FOREST, PlayerId::One));
    let casts = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| matches!(action, Action::CastSpell { card, .. } if *card == id))
        .collect::<Vec<_>>();
    assert!(casts.iter().all(|action| matches!(action, Action::CastSpell { choices, .. } if choices.costs().additional().len() <= 2)));
    let twice = casts.into_iter().find(|action| matches!(action, Action::CastSpell { choices, .. } if choices.costs().additional().len() == 2)).expect("two cards pay two repetitions without any mana sources");
    game.apply(PlayerId::One, twice).unwrap();
    assert!(game.players[0].hand.is_empty());
    assert_eq!(game.players[0].graveyard.len(), 2);
}

#[test]
fn spree_mode_costs_can_be_nonmana() {
    static MODES: [(&[crate::CostDef], AbilityDef); 1] = [(
        &[CostDef::PayLife(2)],
        AbilityDef::spell("Do nothing.", EffectDef::None),
    )];
    let (mut game, id) =
        game_with_cost_rules(&CardRules::new_sorcery(mana_cost!("{0}")).with_ability(
            crate::card::sets::outlaws_of_thunder_junction::spree(&MODES),
        ));
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == id))
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    assert_eq!(game.players[0].life, 18);
}

#[test]
fn modal_additional_cost_repetitions_are_authored_without_a_mechanic_label() {
    use crate::card::{CostQuantityDef, ModalSpellDef, SpellAbilityDef};
    static MODES: [AbilityDef; 2] = [
        AbilityDef::spell("First mode.", EffectDef::None),
        AbilityDef::spell("Second mode.", EffectDef::None),
    ];
    for (repetitions, expected_life) in [
        (CostQuantityDef::Fixed(0), 20),
        (CostQuantityDef::Fixed(3), 14),
        (CostQuantityDef::ModeCount, 16),
        (
            CostQuantityDef::Subtract(&CostQuantityDef::ModeCount, &CostQuantityDef::Fixed(1)),
            18,
        ),
    ] {
        let ability = AbilityDef::defined(
            "Choose both —",
            DeclarativeAbilityDef::Spell(SpellAbilityDef::Modal(
                ModalSpellDef::new(&MODES, 2, 2, false)
                    .with_additional_cost(CostDef::PayLife(2), repetitions),
            )),
            EffectDef::None,
        );
        assert_eq!(ability.label, None);
        let (mut game, id) =
            game_with_cost_rules(&CardRules::new_sorcery(mana_cost!("{0}")).with_ability(ability));
        let cast = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == id))
            .expect("the authored payment is affordable");
        game.apply(PlayerId::One, cast).unwrap();
        assert_eq!(game.players[0].life, expected_life);
    }
}

#[test]
fn once_per_object_restriction_does_not_require_an_exhaust_label() {
    const ABILITY: AbilityDef = AbilityDef::activated(
        "Pay 1 life: Gain 2 life. Activate only once.",
        &[CostDef::PayLife(1)],
        EffectDef::GainLife {
            recipient: EffectRecipientDef::Controller,
            amount: ValueDef::Constant(2),
        },
    )
    .once_per_object();
    assert_eq!(ABILITY.label, None);
    let (mut game, _) = game_with_cost_rules(
        &CardRules::new_creature(mana_cost!("{0}"), &["Test"], 1, 1).with_ability(ABILITY),
    );
    game.players[0].hand.clear();
    let source = game
        .put_onto_battlefield(PlayerId::One, CardDefinitionId::new(100_001))
        .unwrap();
    drain_pending(&mut game);
    let action = game.legal_actions(PlayerId::One).into_iter().find(|action| {
        matches!(action, Action::ActivateAbility { source: actual, .. } if *actual == source)
    }).expect("an unlabeled once-only ability is initially available");
    game.apply(PlayerId::One, action).unwrap();
    drain_pending(&mut game);
    assert_eq!(game.players[0].life, 21);
    game.cleanup();
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    assert!(game.legal_actions(PlayerId::One).iter().all(|action| {
        !matches!(action, Action::ActivateAbility { source: actual, .. } if *actual == source)
    }));
}

#[test]
fn special_actions_pay_nonmana_lists_and_keep_the_source_out_of_discard_choices() {
    static COSTS: [CostDef; 2] = [
        CostDef::PayLife(2),
        CostDef::discard(ObjectPredicateDef::Not(&ObjectPredicateDef::HasType(
            CardType::Creature,
        ))),
    ];
    static SUSPEND: [AbilityDef; 1] = [abilities::suspend(
        "Suspend 2—Pay 2 life, discard a noncreature card.",
        &crate::card::SuspendAbilityDef::fixed(2, &COSTS),
    )];
    for plot in [false, true] {
        let rules = CardRules::new_sorcery(mana_cost!("{9}"));
        let rules = if plot {
            rules.with_ability(abilities::plot(&COSTS))
        } else {
            rules.with_abilities(&SUSPEND)
        };
        let (mut game, id) = game_with_cost_rules(&rules);
        let find = |game: &Game| {
            game.legal_actions(PlayerId::One)
                .into_iter()
                .find(|action| match action {
                    Action::Plot { card } if plot => *card == id,
                    Action::Suspend { card, .. } if !plot => *card == id,
                    _ => false,
                })
        };
        assert!(
            find(&game).is_none(),
            "the card taking the special action cannot discard itself"
        );
        game.players[0]
            .hand
            .push(card(230_103, cards::GRIZZLY_BEARS, PlayerId::One));
        assert!(
            find(&game).is_none(),
            "a creature cannot pay the discard cost"
        );
        game.players[0]
            .hand
            .push(card(230_101, cards::SWAMP, PlayerId::One));
        game.players[0]
            .hand
            .push(card(230_102, cards::FOREST, PlayerId::One));
        game.apply(PlayerId::One, find(&game).unwrap()).unwrap();
        let decision = game.observe(PlayerId::One).decision.unwrap();
        assert_eq!(decision.options.len(), 2);
        assert_eq!(
            game.players[0].life, 20,
            "no component is spent before the complete choice"
        );
        assert!(
            game.observe(PlayerId::Two)
                .decision
                .is_none_or(|decision| decision.options.is_empty())
        );
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: decision.id,
                options: vec![decision.options[0].id],
            },
        )
        .unwrap();
        assert_eq!(game.players[0].life, 18);
        assert_eq!(game.players[0].hand.len(), 2);
        assert_eq!(game.players[0].graveyard.len(), 1);
        assert_eq!(game.players[0].exile.len(), 1);
        assert_eq!(
            game.players[0].exile[0]
                .counters
                .count(CounterKind::named("time")),
            if plot { 0 } else { 2 }
        );
    }
}

#[test]
fn morph_can_be_paid_with_life_instead_of_mana() {
    let (mut game, id) = game_with_cost_rules(
        &CardRules::new_creature(mana_cost!("{9}"), &["Beast"], 3, 3)
            .with_morph(&[CostDef::PayLife(2)])
            .with_ability(AbilityDef::alternative_cast(
                &[CostDef::Mana(mana_cost!("{3}"))],
                crate::card::face_down::morph_cast(),
                None,
                EffectDef::None,
            )),
    );
    let definition = game.players[0].hand.remove(0).definition;
    let mut permanent = creature(id.0, definition, PlayerId::One);
    permanent.face_down = Some(crate::card::face_down::morph());
    game.battlefield.push(permanent);
    let action = Action::TurnFaceUp { permanent: id };
    assert!(game.legal_actions(PlayerId::One).contains(&action));
    game.apply(PlayerId::One, action).unwrap();
    assert_eq!(game.players[0].life, 18);
    assert!(game.battlefield[0].face_down.is_none());
}

pub(in crate::game) fn mixed_echo_pending_game() -> Game {
    static COSTS: [CostDef; 2] = [CostDef::Mana(mana_cost!("{1}")), CostDef::DiscardCards(1)];
    let (mut game, id) = game_with_cost_rules(
        &CardRules::new_creature(mana_cost!("{1}"), &["Imp"], 1, 1)
            .with_ability(abilities::echo("Echo—{1}, discard a card.", &COSTS)),
    );
    let definition = game.players[0].hand.remove(0).definition;
    let mut permanent = creature(id.0, definition, PlayerId::One);
    permanent.entered_controller_turn = 5;
    game.battlefield.push(permanent);
    game.players[0]
        .hand
        .push(card(230_101, cards::SWAMP, PlayerId::One));
    game.players[0]
        .hand
        .push(card(230_102, cards::FOREST, PlayerId::One));
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    game.turns_started[0] = 6;
    game.turn = 6;
    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    game.finish_rules_procedure();
    game.resolve_stack_top();
    game
}

#[test]
fn mixed_alternative_casts_pay_every_component_without_synthetic_mana() {
    static ABILITIES: [AbilityDef; 2] = [
        AbilityDef::spell("Do nothing.", EffectDef::None),
        AbilityDef::alternative_cast(
            &[CostDef::PayLife(2), CostDef::DiscardCards(1)],
            AlternativeCastKindDef::AlternativeCost,
            Some("Pay 2 life and discard a card rather than pay this spell's mana cost."),
            EffectDef::None,
        ),
    ];
    let (mut game, id) =
        game_with_cost_rules(&CardRules::new_sorcery(mana_cost!("{9}")).with_abilities(&ABILITIES));
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == id))
    );
    game.players[0]
        .hand
        .push(card(230_101, cards::SWAMP, PlayerId::One));
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == id))
        .unwrap();
    assert!(!cast_mana_presence(&game, &action));
    game.apply(PlayerId::One, action).unwrap();
    assert_eq!(game.players[0].life, 18);
    assert_eq!(game.players[0].graveyard.len(), 1);
    assert!(game.players[0].hand.is_empty());
}

#[test]
fn zero_repetitions_do_not_introduce_a_mana_payment() {
    static ABILITIES: [AbilityDef; 2] = [
        AbilityDef::spell("Do nothing.", EffectDef::None),
        AbilityDef::alternative_cast(
            &[CostDef::ManaTimes {
                cost: mana_cost!("{1}"),
                quantity: crate::card::CostQuantityDef::Fixed(0),
            }],
            AlternativeCastKindDef::AlternativeCost,
            Some("Pay no cost rather than pay this spell's mana cost."),
            EffectDef::None,
        ),
    ];
    let (mut game, id) =
        game_with_cost_rules(&CardRules::new_sorcery(mana_cost!("{9}")).with_abilities(&ABILITIES));
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == id))
        .unwrap();
    assert!(!cast_mana_presence(&game, &action));
    game.apply(PlayerId::One, action).unwrap();
    assert_eq!(game.stack.len(), 1);
}
