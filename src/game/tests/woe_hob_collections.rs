//! Hidden information and aggregate selection constraints in the deck corpus.
use super::woe_hob_completion::{hand, setup};
use super::*;

#[test]
fn woe_hob_robbery_hides_exile_from_its_owner_and_spends_colorless_as_any_type() {
    for floating in [true, false] {
        let mut game = setup(true);
        game.players[0].library = game.build_zone(PlayerId::One, &[cards::FOREST; 8]).unwrap();
        game.players[1].library = game
            .build_zone(PlayerId::Two, &[cards::FOREST, cards::ANCESTRAL_RECALL])
            .unwrap();
        let robbery = hand(&mut game, cards::OUTRAGEOUS_ROBBERY);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 4);
        let cast = game.legal_actions(PlayerId::One).into_iter().find(|action|
        matches!(action, Action::CastSpell { card, choices, .. } if *card == robbery && choices.x() == 2
            && choices.targets().iter().any(|target| target.targets() == [Target::Player(PlayerId::Two)]))).unwrap();
        game.apply(PlayerId::One, cast).unwrap();
        drain_pending(&mut game);
        assert_eq!(game.observe(PlayerId::One).exiles[1].len(), 2);
        assert!(game.observe(PlayerId::Two).exiles[1].is_empty());
        assert_eq!(game.observe(PlayerId::Two).face_down_exile_sizes[1], 2);
        let recall = game.players[1]
            .exile
            .iter()
            .find(|c| c.definition == cards::ANCESTRAL_RECALL)
            .unwrap()
            .id;
        if floating {
            game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
        } else {
            game.put_onto_battlefield(PlayerId::One, cards::FOREST)
                .unwrap();
        }
        let cast = game.legal_actions(PlayerId::One).into_iter().find(|action|
        matches!(action, Action::CastSpell { card, choices, .. } if *card == recall
            && choices.targets().iter().any(|target| target.targets() == [Target::Player(PlayerId::One)]))).unwrap();
        game.apply(PlayerId::One, cast).unwrap();
        drain_pending(&mut game);
        assert_eq!(game.players[0].hand.len(), 3);
        assert!(
            game.players[1]
                .graveyard
                .iter()
                .any(|c| c.definition == cards::ANCESTRAL_RECALL)
        );
    }
}

#[test]
fn woe_hob_michelangelo_caps_the_combined_mana_value_and_keeps_choices_private() {
    let mut game = setup(true);
    game.players[0].library = game
        .build_zone(
            PlayerId::One,
            &[
                cards::FOREST,
                cards::FOREST,
                cards::FOREST,
                cards::FOREST,
                cards::SERRA_ANGEL,
                cards::GRIZZLY_BEARS,
                cards::SAVANNAH_LIONS,
                cards::SAVANNAH_LIONS,
            ],
        )
        .unwrap();
    let technique = hand(&mut game, cards::MICHELANGELO_S_TECHNIQUE);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 5);
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == technique))
        .unwrap();
    game.apply(PlayerId::One, cast).unwrap();
    game.pass_priority(PlayerId::One);
    game.pass_priority(PlayerId::Two);
    let first = game.observe(PlayerId::One).decision.unwrap();
    let angel = first
        .options
        .iter()
        .find(|o| o.label.contains("Serra Angel"))
        .unwrap()
        .id;
    assert!(
        game.observe(PlayerId::Two)
            .decision
            .as_ref()
            .is_none_or(|d| d.options.is_empty())
    );
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: first.id,
            options: vec![angel],
        },
    )
    .unwrap();
    let second = game.observe(PlayerId::One).decision.unwrap();
    assert!(
        !second
            .options
            .iter()
            .any(|o| o.label.contains("Grizzly Bears"))
    );
    let lion = second
        .options
        .iter()
        .find(|o| o.label.contains("Savannah Lions"))
        .unwrap()
        .id;
    assert!(
        game.battlefield.is_empty(),
        "both choices precede the simultaneous entry"
    );
    let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
    let mut rebuilt =
        Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 0)
            .unwrap();
    for state in [&mut game, &mut rebuilt] {
        state
            .apply(
                PlayerId::One,
                Action::ChooseDecision {
                    decision: second.id,
                    options: vec![lion],
                },
            )
            .unwrap();
        drain_pending(state);
        assert_eq!(state.battlefield.len(), 2);
        assert_eq!(state.players[0].library.len(), 6);
        assert_eq!(
            state
                .battlefield
                .iter()
                .map(|p| state.permanent_mana_value(p))
                .sum::<u16>(),
            6
        );
    }
}

#[test]
fn woe_hob_political_triumph_counts_crossing_four_even_after_its_source_leaves() {
    for placed in [1, 2] {
        let mut game = setup(true);
        game.players[0].library = game.build_zone(PlayerId::One, &[cards::FOREST; 8]).unwrap();
        let bear = game
            .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
            .unwrap();
        let plan = game
            .put_onto_battlefield(PlayerId::One, cards::POLITICAL_TRIUMPH)
            .unwrap();
        game.add_counters_to_permanent(plan, CounterKind::named("plan"), 3);
        assert!(game.pending_triggers.is_empty());
        game.add_counters_to_permanent(plan, CounterKind::named("plan"), placed);
        game.sacrifice_permanent(plan);
        drain_pending(&mut game);
        assert_eq!(game.players[0].hand.len(), 1);
        assert_eq!(
            game.battlefield
                .iter()
                .find(|p| p.card.id == bear)
                .unwrap()
                .counters(CounterKind::PlusOnePlusOne),
            1
        );
    }
}

#[test]
fn woe_hob_she_hulk_uses_the_turn_limit_only_when_the_damage_is_accepted() {
    for prepared in [false, true] {
        let mut game = setup(prepared);
        let hulk = game
            .put_onto_battlefield(PlayerId::One, cards::JENNIFER_WALTERS)
            .unwrap();
        game.transform_permanent(hulk);
        let starting_life = game.players[1].life;
        for attempt in 0..3 {
            game.deal_damage_simultaneously(vec![DamageAssignment {
                source: None,
                target: Some(Target::Permanent(hulk)),
                amount: 1,
                combat: false,
            }]);
            game.finish_rules_procedure();
            let pending = game.pending_decisions.first().unwrap();
            let DecisionContinuation::TriggerPlacement { candidates, .. } = &pending.continuation
            else {
                panic!("damage trigger must choose a target");
            };
            let index = candidates
                .iter()
                .position(|target| *target == Target::Player(PlayerId::Two))
                .unwrap();
            let decision = pending.observation.clone();
            game.apply(
                PlayerId::One,
                Action::ChooseDecision {
                    decision: decision.id,
                    options: vec![decision.options[index].id],
                },
            )
            .unwrap();
            if attempt == 2 {
                game.sacrifice_permanent(hulk);
            }
            game.pass_priority(PlayerId::One);
            game.pass_priority(PlayerId::Two);
            let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
            game = Game::from_observation_checkpoint(
                game.catalog.clone(),
                game.format,
                &wire,
                &hidden,
                0,
            )
            .unwrap();
            let offer = game.observe(PlayerId::One).decision.unwrap();
            assert_eq!(
                offer.options.iter().any(|o| o.label == "Do it"),
                attempt < 2
            );
            choose_decision_by_label(
                &mut game,
                PlayerId::One,
                if attempt == 1 { "Do it" } else { "Decline" },
            );
            assert_eq!(game.effect_uses_this_turn.len(), usize::from(attempt > 0));
            assert_eq!(game.players[1].life, starting_life - i16::from(attempt > 0));
        }
    }
}

#[test]
fn woe_hob_any_type_preserves_nongeneric_mana_and_contribution_restrictions() {
    use crate::game::payment::allocation::{
        PaymentPool, contribution_remainder, symbol_payment_remainder,
    };
    let mut pool = PaymentPool {
        any_type: true,
        ..PaymentPool::default()
    };
    pool.mana.add_color(ManaColor::Green, 1);
    pool.non_generic.add_color(ManaColor::Green, 1);
    let can_pay = |cost| symbol_payment_remainder(pool, PaymentPool::default(), cost, 0).is_some();
    assert!(can_pay(mana_cost!("{U}")));
    assert!(can_pay(mana_cost!("{C}")));
    assert!(!can_pay(mana_cost!("{1}")));
    let (restricted, _) = fold_restricted_x(mana_cost!("{X}"), 1, ManaColor::Black);
    assert!(!can_pay(restricted));
    pool.non_generic = ManaPool::default();
    assert!(symbol_payment_remainder(pool, PaymentPool::default(), restricted, 0).is_none());
    pool.direct.add_color(ManaColor::Green, 1);
    assert_eq!(
        contribution_remainder(pool, mana_cost!("{1}{C}"), 0),
        Some(mana_cost!("{C}")),
    );
    assert!(contribution_remainder(pool, mana_cost!("{C}"), 0).is_none());
    assert!(contribution_remainder(pool, mana_cost!("{U}"), 0).is_none());
}
