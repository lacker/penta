//! Optional additional costs that stay distinct through casting and
//! resolution: ordinary kicker, multikicker, and two repeatable surcharges.

use super::*;
use crate::card::{CostQuantityDef, SpellAdditionalCostDef};

fn settle(game: &mut Game) {
    for _ in 0..32 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = decision
                .options
                .iter()
                .take(decision.minimum.max(1))
                .map(|option| option.id)
                .collect();
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .expect("the first offered replacement ordering is legal");
            continue;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let priority = game.priority;
        if game.apply(priority, Action::PassPriority).is_err() {
            break;
        }
    }
    game.check_state_based_actions();
}

fn card_in_hand(game: &mut Game, definition: CardDefinitionId) -> GameObjectId {
    game.players[0].hand.clear();
    let card = game
        .build_zone(PlayerId::One, &[definition])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let id = card.id;
    game.players[0].hand.push(card);
    id
}

fn cast_with_costs(game: &mut Game, card: GameObjectId, wanted: &[AdditionalCostId]) {
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell {
                card: candidate,
                choices,
                ..
            } => *candidate == card && choices.costs().additional() == wanted,
            _ => false,
        })
        .expect("the selected optional costs are payable");
    game.apply(PlayerId::One, cast).expect("the spell is cast");
    settle(game);
}

#[test]
#[allow(clippy::too_many_lines)]
fn escape_context_and_kicker_payment_coexist_through_entry() {
    static ABILITIES: [AbilityDef; 3] = [
        AbilityDef::triggered_if(
            "When this creature enters, if it escaped, you gain 1 life.",
            TriggerEventDef::zone_changed(
                ObjectPredicateDef::Source,
                None,
                Some(ZoneKind::Battlefield),
            ),
            &TriggerConditionDef::All(&[
                TriggerConditionDef::SourceWasCast,
                TriggerConditionDef::SourceCastFrom(ZoneKind::Graveyard),
                TriggerConditionDef::SourceCastWith(AlternativeCastKindDef::Escape),
            ]),
            EffectDef::GainLife {
                recipient: EffectRecipientDef::Controller,
                amount: ValueDef::Constant(1),
            },
        ),
        abilities::kicker(mana_cost!("{1}")),
        AbilityDef::alternative_cast_with_additional_cost(
            AlternativeCastManaCostDef::Fixed(mana_cost!("{B}")),
            AlternativeCastKindDef::Escape,
            None,
            SpellAdditionalCostDef::exile(
                ObjectPredicateDef::Any,
                ZoneKind::Graveyard,
                CostQuantityDef::Fixed(1),
            ),
            EffectDef::None,
        ),
    ];
    let definition_id = CardDefinitionId::new(20_102);
    let mut definition = CardDefinition::new(
        definition_id,
        "Escaped and Kicked",
        CardSet::TherosBeyondDeath,
        crate::card::CardRules::unsupported(),
    );
    definition.rules =
        CardRules::new_creature(mana_cost!("{2}{B}"), &["Giant"], 3, 3).with_abilities(&ABILITIES);
    synchronize_single_part_definition(&mut definition);
    let mut game = ready_game();
    game.catalog = CardCatalog::new([definition]).expect("the fixture is valid");
    game.battlefield.clear();
    for player in &mut game.players {
        player.hand.clear();
        player.graveyard.clear();
        player.library.clear();
        player.exile.clear();
    }
    let escaped = card(20_102, definition_id, PlayerId::One);
    let escaped_id = escaped.id;
    let fodder = card(20_103, definition_id, PlayerId::One);
    let fodder_id = fodder.id;
    game.players[0].graveyard.extend([escaped, fodder]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell {
                card,
                choices,
                sacrifices,
            } => {
                *card == escaped_id
                    && choices.costs().alternative().is_some()
                    && choices.costs().additional() == [AdditionalCostId(1)]
                    && sacrifices == &[fodder_id]
            }
            _ => false,
        })
        .expect("escape and kicker are selectable together");
    game.apply(PlayerId::One, cast).expect("the cast is legal");
    let spell = game.stack.last().expect("the spell is on the stack");
    let cast = spell.cast.as_ref().expect("the cast context is retained");
    assert_eq!(cast.source_zone, Some(CastSourceZone::Graveyard));
    assert_eq!(cast.alternative, Some(AlternativeCastKindDef::Escape));
    assert_eq!(cast.additional_costs, [1]);
    assert_eq!(cast.exiled_payment_cards.len(), 1);
    let additional = spell
        .signature
        .as_ref()
        .expect("a spell has a signature")
        .costs()
        .additional();
    assert_eq!(
        additional,
        [AdditionalCostId(1)],
        "the independent kicker selection remains in the cast context",
    );
    let life = game.players[0].life;
    settle(&mut game);
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == definition_id)
        .expect("the escaped creature entered");
    let cast = permanent
        .cast
        .as_ref()
        .expect("entry kept the cast context");
    assert_eq!(cast.source_zone, Some(CastSourceZone::Graveyard));
    assert_eq!(cast.alternative, Some(AlternativeCastKindDef::Escape));
    assert_eq!(cast.additional_costs, [1]);
    assert_eq!(game.players[0].life, life + 1, "the escape condition fired");
}

fn staged_anavolver() -> (Game, GameObjectId) {
    let mut game = ready_game();
    let card = card_in_hand(&mut game, cards::ANAVOLVER);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 4);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);
    (game, card)
}

fn anavolver(game: &Game) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::ANAVOLVER)
        .expect("Anavolver resolved")
}

#[test]
fn kicker_declarations_keep_printed_and_additional_costs_separate() {
    let game = ready_game();
    let cases = [
        (
            cards::PROHIBIT,
            mana_cost!("{1}{U}"),
            mana_cost!("{2}"),
            "Kicker {2} (You may pay an additional {2} as you cast this spell.)\nCounter target spell if its mana value is 2 or less. If this spell was kicked, counter that spell if its mana value is 4 or less instead.",
        ),
        (
            cards::OVERLOAD,
            mana_cost!("{R}"),
            mana_cost!("{2}"),
            "Kicker {2} (You may pay an additional {2} as you cast this spell.)\nDestroy target artifact if its mana value is 2 or less. If this spell was kicked, destroy that artifact if its mana value is 5 or less instead.",
        ),
        (
            cards::BURST_LIGHTNING,
            mana_cost!("{R}"),
            mana_cost!("{4}"),
            "Kicker {4} (You may pay an additional {4} as you cast this spell.)\nBurst Lightning deals 2 damage to any target. If this spell was kicked, it deals 4 damage instead.",
        ),
        (
            cards::BLOODCHIEFS_THIRST,
            mana_cost!("{B}"),
            mana_cost!("{2}{B}"),
            "Kicker {2}{B} (You may pay an additional {2}{B} as you cast this spell.)\nDestroy target creature or planeswalker with mana value 2 or less. If this spell was kicked, instead destroy target creature or planeswalker.",
        ),
        (
            cards::TEAR_ASUNDER,
            mana_cost!("{1}{G}"),
            mana_cost!("{1}{B}"),
            "Kicker {1}{B} (You may pay an additional {1}{B} as you cast this spell.)\nExile target artifact or enchantment. If this spell was kicked, exile target nonland permanent instead.",
        ),
    ];

    for (card, printed, kicker, oracle) in cases {
        let definition = game.catalog.get(card).expect("cataloged");
        let option = &definition.play_options[0];
        assert_eq!(option.mana_cost, Some(printed), "{}", definition.name);
        assert!(option.alternative_costs.is_empty(), "{}", definition.name);
        assert_eq!(option.additional_costs.len(), 1, "{}", definition.name);
        assert_eq!(
            option.additional_costs[0].mana_cost,
            Some(kicker),
            "{}",
            definition.name,
        );
        assert_eq!(definition.rules.rules_text(), oracle, "{}", definition.name);
    }
}

#[test]
fn anavolver_offers_two_independent_kickers_and_remembers_both() {
    let (mut game, card) = staged_anavolver();
    let mut offered = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell {
                card: candidate,
                choices,
                ..
            } if candidate == card => Some(choices.costs().additional().to_vec()),
            _ => None,
        })
        .collect::<Vec<_>>();
    offered.sort_by_key(|costs| (costs.len(), costs.first().copied()));
    offered.dedup();
    assert_eq!(
        offered,
        vec![
            vec![],
            vec![AdditionalCostId(0)],
            vec![AdditionalCostId(1)],
            vec![AdditionalCostId(0), AdditionalCostId(1)],
        ],
    );

    cast_with_costs(&mut game, card, &[AdditionalCostId(0), AdditionalCostId(1)]);

    let permanent = anavolver(&game);
    assert_eq!(
        permanent.counters(CounterKind::PlusOnePlusOne),
        3,
        "the two kickers add their counters independently",
    );
    assert!(game.permanent_has_executable_keyword(permanent, KeywordAbility::Flying));
    let source = permanent.card.id;
    assert!(game.legal_actions(PlayerId::One).into_iter().any(
        |action| matches!(action, Action::ActivateAbility { source: candidate, .. } if candidate == source)
    ));

    let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
    let rebuilt = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &wire,
        &hidden,
        30_010,
    )
    .expect("the distinct kicker counts reconstruct");
    let permanent = anavolver(&rebuilt);
    assert_eq!(permanent.counters(CounterKind::PlusOnePlusOne), 3);
    assert!(rebuilt.permanent_has_executable_keyword(permanent, KeywordAbility::Flying));
    let source = permanent.card.id;
    assert!(rebuilt.legal_actions(PlayerId::One).into_iter().any(
        |action| matches!(action, Action::ActivateAbility { source: candidate, .. } if candidate == source)
    ));
}

#[test]
fn anavolver_kickers_grant_only_their_own_half() {
    let (mut blue_game, blue_card) = staged_anavolver();
    cast_with_costs(&mut blue_game, blue_card, &[AdditionalCostId(0)]);
    let blue = anavolver(&blue_game);
    assert_eq!(blue.counters(CounterKind::PlusOnePlusOne), 2);
    assert!(blue_game.permanent_has_executable_keyword(blue, KeywordAbility::Flying));
    assert!(!blue_game.legal_actions(PlayerId::One).into_iter().any(
        |action| matches!(action, Action::ActivateAbility { source, .. } if source == blue.card.id)
    ));

    let (mut black_game, black_card) = staged_anavolver();
    cast_with_costs(&mut black_game, black_card, &[AdditionalCostId(1)]);
    let black = anavolver(&black_game);
    assert_eq!(black.counters(CounterKind::PlusOnePlusOne), 1);
    assert!(!black_game.permanent_has_executable_keyword(black, KeywordAbility::Flying));
    assert!(black_game.legal_actions(PlayerId::One).into_iter().any(
        |action| matches!(action, Action::ActivateAbility { source, .. } if source == black.card.id)
    ));
}

#[test]
fn wolfbriar_makes_one_wolf_for_each_multikicker_payment() {
    let mut game = ready_game();
    let card = card_in_hand(&mut game, cards::WOLFBRIAR_ELEMENTAL);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 7);

    cast_with_costs(
        &mut game,
        card,
        &[
            AdditionalCostId(0),
            AdditionalCostId(0),
            AdditionalCostId(0),
        ],
    );

    assert_eq!(
        game.battlefield.len(),
        4,
        "the Elemental and three Wolf tokens",
    );
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| permanent.card.definition.is_token())
            .count(),
        3,
    );
}

#[test]
fn primitive_justice_scales_targets_and_counts_only_green_for_life() {
    let mut game = ready_game();
    let card = card_in_hand(&mut game, cards::PRIMITIVE_JUSTICE);
    let artifacts = (0..4)
        .map(|index| {
            let permanent = token_permanent(
                30_000 + index,
                TokenCharacteristics::artifact(&["Clue"], &[]),
                PlayerId::Two,
            );
            let id = permanent.card.id;
            game.battlefield.push(permanent);
            id
        })
        .collect::<Vec<_>>();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 2);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    let before_life = game.players[0].life;

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell {
                card: candidate,
                choices,
                ..
            } => {
                *candidate == card
                    && choices.costs().additional() == [AdditionalCostId(0), AdditionalCostId(1)]
                    && matches!(choices.targets(), [selection]
                    if selection.targets().iter().copied().eq(
                        artifacts[..3].iter().copied().map(Target::Permanent)
                    ))
            }
            _ => false,
        })
        .expect("one red and one green surcharge require exactly three targets");
    game.apply(PlayerId::One, cast).expect("the spell is cast");
    settle(&mut game);

    assert_eq!(game.players[0].life, before_life + 1);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| artifacts.contains(&permanent.card.id))
            .map(|permanent| permanent.card.id)
            .collect::<Vec<_>>(),
        vec![artifacts[3]],
    );
}

/// Bargain is kicker's shape with a sacrifice instead of mana: paid once,
/// read back as a yes. Candy Grapple is the first card to print it, and the
/// two sizes are one effect reading the payment rather than two clauses.
#[test]
fn bargain_is_optional_and_the_payment_picks_which_number_applies() {
    for bargained in [false, true] {
        let mut game = ready_game();
        game.battlefield.clear();
        let mut victim = creature(21_000, cards::SERRA_ANGEL, PlayerId::Two);
        victim.entered_controller_turn = 0;
        let victim_id = victim.card.id;
        game.battlefield.push(victim);
        let food = creature(21_001, cards::ORNITHOPTER, PlayerId::One);
        game.battlefield.push(food);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);

        let grapple = card_in_hand(&mut game, cards::CANDY_GRAPPLE);
        let cast = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| match action {
                Action::CastSpell {
                    card: candidate,
                    choices,
                    ..
                } => {
                    *candidate == grapple
                        && choices.costs().additional().is_empty() != bargained
                        && choices
                            .targets()
                            .iter()
                            .flat_map(TargetSelection::targets)
                            .copied()
                            .eq([Target::Permanent(victim_id)])
                }
                _ => false,
            })
            .expect("both the bargained and unbargained casts are offered");
        game.apply(PlayerId::One, cast).expect("the spell is cast");
        settle(&mut game);

        // A 4/4 survives -3/-3 as a 1/1 and does not survive -5/-5, which is
        // the whole of what the sacrifice buys.
        let toughness = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == victim_id)
            .and_then(|permanent| game.toughness(permanent));
        if bargained {
            assert_eq!(toughness, None, "the bargained -5/-5 kills a 4/4");
        } else {
            assert_eq!(toughness, Some(1), "the unbargained -3/-3 leaves a 1/1");
        }
    }
}
