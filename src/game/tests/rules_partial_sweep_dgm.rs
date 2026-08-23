//! Stale rules-gap audits retired by declaration, choice, and continuation primitives.

use super::*;
use crate::ImplementationStatus;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.turns_started[PlayerId::Two.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.battlefield.clear();
    game
}

fn settle_required_choices(game: &mut Game) {
    for _ in 0..32 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = decision
                .options
                .iter()
                .take(decision.minimum)
                .map(|option| option.id)
                .collect();
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .expect("the required choices are legal");
            continue;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            return;
        }
        let player = game.priority;
        game.apply(player, Action::PassPriority)
            .expect("priority advances");
    }
    panic!("the game did not settle");
}

#[test]
fn battalion_uses_the_committed_attack_declaration() {
    let mut game = ready();
    let mastiff = creature(10_000, cards::BOROS_MASTIFF, PlayerId::One);
    let mastiff_id = mastiff.card.id;
    let tajic = creature(10_001, cards::TAJIC_BLADE_OF_THE_LEGION, PlayerId::One);
    let tajic_id = tajic.card.id;
    let first = creature(10_002, cards::GRIZZLY_BEARS, PlayerId::One);
    let first_id = first.card.id;
    let second = creature(10_003, cards::SAVANNAH_LIONS, PlayerId::One);
    let second_id = second.card.id;
    game.battlefield.extend([mastiff, tajic, first, second]);
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;

    for attacker in [mastiff_id, tajic_id, first_id, second_id] {
        game.apply(
            PlayerId::One,
            Action::DeclareAttacker {
                attacker,
                defender: AttackDefender::Player(PlayerId::Two),
            },
        )
        .expect("the creature can attack");
    }
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .expect("the declaration completes");

    game.remove_permanent_from_combat(first_id);
    game.remove_permanent_from_combat(second_id);
    drain_pending(&mut game);

    let mastiff = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == mastiff_id)
        .expect("the Mastiff remains");
    assert!(game.permanent_has_executable_keyword(mastiff, KeywordAbility::Lifelink));
    let tajic = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == tajic_id)
        .expect("Tajic remains");
    assert_eq!(
        (game.power(tajic), game.toughness(tajic)),
        (Some(7), Some(7))
    );
}

#[test]
fn species_gorger_makes_a_non_targeted_creature_choice() {
    let mut game = ready();
    let gorger = creature(10_000, cards::SPECIES_GORGER, PlayerId::One);
    let bear = creature(10_001, cards::GRIZZLY_BEARS, PlayerId::One);
    let bear_id = bear.card.id;
    game.battlefield.extend([gorger, bear]);
    game.battlefield
        .push(creature(10_002, cards::SERRA_ANGEL, PlayerId::Two));
    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();

    for _ in 0..12 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let option = decision
                .options
                .iter()
                .find(|option| option.card.as_ref().is_some_and(|(id, _)| *id == bear_id))
                .expect("the Bear is offered");
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options: vec![option.id],
                },
            )
            .expect("the Bear can be chosen");
            break;
        }
        let player = game.priority;
        game.apply(player, Action::PassPriority)
            .expect("priority advances");
    }
    drain_pending(&mut game);

    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|card| card.definition == cards::GRIZZLY_BEARS)
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::SPECIES_GORGER)
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::SERRA_ANGEL)
    );
}

#[test]
fn ral_zarek_requires_different_targets() {
    let mut game = ready();
    let ral = game
        .put_onto_battlefield(PlayerId::One, cards::RAL_ZAREK)
        .expect("cataloged");
    game.put_onto_battlefield(PlayerId::One, cards::MOUNTAIN)
        .expect("cataloged");
    game.put_onto_battlefield(PlayerId::Two, cards::ISLAND)
        .expect("cataloged");

    let actions = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::ActivateAbility {
                source,
                ability: AbilityOrigin::Printed { ability, .. },
                targets,
                ..
            } if source == ral && ability == AbilityId(0) => Some(targets),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert!(!actions.is_empty(), "Ral's +1 is offered");
    for targets in actions {
        let chosen = targets
            .iter()
            .flat_map(TargetSelection::targets)
            .collect::<Vec<_>>();
        assert_eq!(chosen.len(), 2);
        assert_ne!(chosen[0], chosen[1]);
    }
}

#[test]
fn ral_zarek_flips_five_coins_and_queues_one_turn_per_win() {
    let mut observed = std::collections::BTreeSet::new();
    for seed in 0..40 {
        let mut game = ready_game_with_seed(seed);
        game.battlefield.clear();
        game.turn = 5;
        game.turns_started[PlayerId::One.index()] = 5;
        game.step = Step::PrecombatMain;
        game.priority = PlayerId::One;
        let ral = game
            .put_onto_battlefield(PlayerId::One, cards::RAL_ZAREK)
            .expect("cataloged");
        game.battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == ral)
            .expect("Ral remains")
            .set_counters(CounterKind::Loyalty, 7);

        let ultimate = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| {
                matches!(action, Action::ActivateAbility {
                    source,
                    ability: AbilityOrigin::Printed { ability, .. },
                    ..
                } if *source == ral && *ability == AbilityId(2))
            })
            .expect("Ral's ultimate is offered");
        game.apply(PlayerId::One, ultimate)
            .expect("the ultimate activates");
        drain_pending(&mut game);
        assert!(game.extra_turns.len() <= 5);
        observed.insert(game.extra_turns.len());
    }
    assert!(
        observed.len() > 1,
        "different seeds produce different win counts"
    );
}

#[test]
fn nicol_bolas_steals_indefinitely() {
    let mut game = ready();
    let bolas = game
        .put_onto_battlefield(PlayerId::One, cards::NICOL_BOLAS_PLANESWALKER)
        .expect("cataloged");
    let bear = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    let steal = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility {
                source,
                ability: AbilityOrigin::Printed { ability, .. },
                targets,
                ..
            } if *source == bolas
                && *ability == AbilityId(1)
                && targets.iter().flat_map(TargetSelection::targets)
                    .any(|target| *target == Target::Permanent(bear)))
        })
        .expect("the theft is offered");
    game.apply(PlayerId::One, steal)
        .expect("the ability activates");
    drain_pending(&mut game);

    assert_eq!(game.controller_of_object(bear), Some(PlayerId::One));
}

#[test]
fn nicol_bolas_discards_before_the_sacrifice_choice() {
    let mut game = ready();
    let bolas = game
        .put_onto_battlefield(PlayerId::One, cards::NICOL_BOLAS_PLANESWALKER)
        .expect("cataloged");
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == bolas)
        .expect("Bolas remains")
        .set_counters(CounterKind::Loyalty, 9);
    for offset in 0..8 {
        game.players[1]
            .hand
            .push(card(11_000 + offset, cards::MOUNTAIN, PlayerId::Two));
        game.battlefield
            .push(creature(12_000 + offset, cards::FOREST, PlayerId::Two));
    }

    let ultimate = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility {
                source,
                ability: AbilityOrigin::Printed { ability, .. },
                targets,
                ..
            } if *source == bolas
                && *ability == AbilityId(2)
                && targets.iter().flat_map(TargetSelection::targets)
                    .any(|target| *target == Target::Player(PlayerId::Two)))
        })
        .expect("the ultimate can target the opponent");
    game.apply(PlayerId::One, ultimate)
        .expect("the ultimate activates");
    settle_required_choices(&mut game);

    assert_eq!(game.players[1].life, 13);
    assert_eq!(game.players[1].hand.len(), 1);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| permanent.controller == PlayerId::Two)
            .count(),
        1,
    );
}

#[test]
fn every_additional_sweep_card_reports_complete_coverage() {
    let catalog = poc::catalog().expect("catalog builds");
    for definition in [
        cards::BOROS_MASTIFF,
        cards::RAL_ZAREK,
        cards::NICOL_BOLAS_PLANESWALKER,
        cards::SPECIES_GORGER,
        cards::TAJIC_BLADE_OF_THE_LEGION,
    ] {
        let card = catalog.get(definition).expect("the card is cataloged");
        assert_eq!(
            card.rules.implementation_status(),
            ImplementationStatus::Complete,
            "{} should be fully executable",
            card.name,
        );
    }
}
