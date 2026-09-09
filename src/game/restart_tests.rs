use super::*;
use crate::card::{self, CostDef, CounterKind, DeclarativeAbilityDef, cards};
use crate::game::tests::{checkpoint_fixture, drain_pending};
use crate::match_play::MatchMode;
use crate::{Action, Format, GameResult};

fn staged(mode: MatchMode) -> (Game, GameObjectId) {
    let mut main = vec![cards::FOREST; 53];
    main.extend([
        cards::KARN_LIBERATED,
        cards::RUNECLAW_BEAR,
        cards::PACIFISM,
        cards::SUTURE_PRIEST,
        cards::SUTURE_PRIEST,
        cards::CAVERN_OF_SOULS,
        cards::DROWNED_CATACOMB,
    ]);
    let decks = [0, 1].map(|_| Deck {
        main: main.clone(),
        sideboard: vec![cards::MOUNTAIN; 15],
    });
    let mut game =
        Game::new_with_format(Format::SomM13Standard, card::catalog().unwrap(), decks, 12).unwrap();
    game.set_match_mode(mode).unwrap();
    if mode == MatchMode::FirstToTwoWins {
        let decision = game.match_decision().unwrap();
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: decision.id,
                options: vec![0],
            },
        )
        .unwrap();
    }
    game.pregame = None;
    game.step = crate::game::Step::PrecombatMain;
    let karn = take(&mut game, PlayerId::One, cards::KARN_LIBERATED);
    let id = karn.id;
    game.players[0].hand.push(karn);
    let outcome = game.move_target_to_zone(
        Target::Card(id),
        ZoneKind::Battlefield,
        ZoneMoveCause::Effect {
            controller: PlayerId::One,
        },
        Some(BattlefieldArrival::under(PlayerId::One)),
        ZonePlacement::Top,
    );
    let _ = outcome;
    drain_pending(&mut game);
    let id = game
        .battlefield
        .iter()
        .find(|p| p.card.definition == cards::KARN_LIBERATED)
        .unwrap()
        .card
        .id;
    (game, id)
}

fn take(
    game: &mut Game,
    seat: PlayerId,
    definition: crate::CardDefinitionId,
) -> crate::game::CardInstance {
    let player = &mut game.players[seat.index()];
    for zone in [&mut player.hand, &mut player.library] {
        if let Some(index) = zone.iter().position(|c| c.definition == definition) {
            return zone.remove(index);
        }
    }
    panic!("card in deck");
}

fn activate(game: &mut Game, karn: GameObjectId, change: i8, target: Option<Target>) {
    let action = game.legal_actions(PlayerId::One).into_iter().find(|action| {
        let Action::ActivateAbility { source, ability, targets, .. } = action else { return false; };
        if *source != karn || !target.is_none_or(|wanted| targets.iter().any(|s| s.targets().contains(&wanted))) { return false; }
        let permanent = game.battlefield.iter().find(|p| p.card.id == karn).unwrap();
        game.find_effective_ability(permanent, |effective| effective.origin == *ability).is_some_and(|effective| {
            matches!(effective.ability.definition, DeclarativeAbilityDef::Activated(def) if def.costs.contains(&CostDef::Loyalty(change)))
        })
    }).expect("Karn's loyalty ability is legal");
    game.apply(PlayerId::One, action).unwrap();
    drain_pending(game);
}

fn ultimate(game: &mut Game, karn: GameObjectId) {
    let permanent = game
        .battlefield
        .iter_mut()
        .find(|p| p.card.id == karn)
        .unwrap();
    permanent.activated_loyalty_this_turn = false;
    permanent.set_counters(CounterKind::Loyalty, 14);
    activate(game, karn, -14, None);
}

fn keep(game: &mut Game) {
    for _ in 0..2 {
        let player = game.decision_player().unwrap();
        game.apply(player, Action::KeepHand).unwrap();
    }
}

#[test]
fn karn_restart_is_not_a_conclusion_and_retained_cards_wait_for_pregame() {
    for mode in [MatchMode::OneConclusion, MatchMode::FirstToTwoWins] {
        let (mut game, karn) = staged(mode);
        game.set_prepared_engine_enabled(false);
        let bear = take(&mut game, PlayerId::Two, cards::RUNECLAW_BEAR);
        let bear_id = bear.id;
        game.players[1].exile.push(bear);
        game.linked_exiles.push((karn, bear_id));
        let aura = take(&mut game, PlayerId::Two, cards::PACIFISM);
        let aura_id = aura.id;
        game.players[1].exile.push(aura);
        game.linked_exiles.push((karn, aura_id));
        game.players[0].life = 3;
        ultimate(&mut game, karn);
        assert_eq!(game.restart_count, 1);
        assert_eq!(game.result(), None);
        assert_eq!(game.current_game_result(), None);
        assert_eq!(game.match_json(PlayerId::One)["draws"], 0);
        assert_eq!(game.match_json(PlayerId::One)["stage"], "playing");
        assert!(!game.prepared_engine_enabled());
        assert!(game.battlefield.is_empty());
        assert_eq!(game.players[0].life, 20);
        assert_eq!(game.players[1].exile.len(), 1);
        assert_eq!(game.players[1].exile[0].definition, cards::RUNECLAW_BEAR);
        assert_eq!(game.players[1].outside_game.len(), 15);
        keep(&mut game);
        let bear = game
            .battlefield
            .iter()
            .find(|p| p.card.definition == cards::RUNECLAW_BEAR)
            .unwrap();
        assert_eq!(bear.controller, PlayerId::One);
        assert_eq!(bear.card.owner, PlayerId::Two);
        assert!(bear.entered_controller_turn < game.turns_started[0]);
        assert!(!bear.tapped);
        assert_eq!(game.turn, 1);
        game.finish(GameResult::Draw);
        assert_eq!(game.match_json(PlayerId::One)["draws"], 1);
        assert_eq!(game.result().is_some(), mode == MatchMode::OneConclusion);
    }
}

#[test]
fn karn_restart_checkpoint_preserves_participating_cards_and_startup() {
    let (mut game, karn) = staged(MatchMode::OneConclusion);
    let bear = take(&mut game, PlayerId::Two, cards::RUNECLAW_BEAR);
    game.linked_exiles.push((karn, bear.id));
    game.players[1].exile.push(bear);
    let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
    let mut rebuilt =
        Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 42)
            .unwrap();
    ultimate(&mut rebuilt, karn);
    assert_eq!(rebuilt.physical_cards.len(), 150);
    let (wire, hidden) = checkpoint_fixture(&rebuilt, PlayerId::One);
    let mut resumed = Game::from_observation_checkpoint(
        rebuilt.catalog.clone(),
        rebuilt.format,
        &wire,
        &hidden,
        43,
    )
    .unwrap();
    keep(&mut resumed);
    assert_eq!(resumed.battlefield.len(), 1);
    assert_eq!(resumed.battlefield[0].card.definition, cards::RUNECLAW_BEAR);
}

#[test]
fn karn_plus_four_exiles_the_target_players_choice_and_minus_three_links_permanents() {
    let (mut game, karn) = staged(MatchMode::OneConclusion);
    activate(&mut game, karn, 4, Some(Target::Player(PlayerId::Two)));
    assert_eq!(game.players[1].exile.len(), 1);
    assert_eq!(game.linked_exiles.len(), 1);
    let permanent = game
        .battlefield
        .iter_mut()
        .find(|p| p.card.id == karn)
        .unwrap();
    permanent.activated_loyalty_this_turn = false;
    activate(&mut game, karn, -3, Some(Target::Permanent(karn)));
    assert!(game.battlefield.is_empty());
    assert_eq!(game.players[0].exile[0].definition, cards::KARN_LIBERATED);
}

#[test]
fn karn_restart_simultaneous_entries_survive_a_replacement_checkpoint() {
    let (mut game, karn) = staged(MatchMode::OneConclusion);
    for definition in [
        cards::SUTURE_PRIEST,
        cards::CAVERN_OF_SOULS,
        cards::SUTURE_PRIEST,
        cards::DROWNED_CATACOMB,
    ] {
        let card = take(&mut game, PlayerId::Two, definition);
        game.linked_exiles.push((karn, card.id));
        game.players[1].exile.push(card);
    }
    ultimate(&mut game, karn);
    keep(&mut game);
    assert!(
        game.restart_arrivals
            .as_ref()
            .is_some_and(|state| !state.ready.is_empty())
    );
    assert!(
        game.battlefield.is_empty(),
        "entry replacements precede every arrival"
    );
    let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
    let mut resumed =
        Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 43)
            .unwrap();
    drain_pending(&mut resumed);
    assert_eq!(resumed.battlefield.len(), 4);
    assert_eq!(
        resumed
            .events
            .iter()
            .filter(|event| matches!(event, crate::GameEvent::AbilityTriggered { .. }))
            .count(),
        2,
        "each Priest sees the other arrive"
    );
    let land = resumed
        .battlefield
        .iter()
        .find(|p| p.card.definition == cards::DROWNED_CATACOMB)
        .unwrap();
    assert!(
        !land.tapped,
        "a retained tapped land untaps in the first untap step"
    );
    assert_eq!(resumed.physical_cards.len(), 150);
    resumed.pending_restart = Some(RestartRequest {
        controller: PlayerId::Two,
        retained: vec![],
    });
    resumed.perform_restart();
    assert_eq!(resumed.decision_player(), Some(PlayerId::Two));
    assert_eq!(
        resumed.players[1].library.len() + resumed.players[1].hand.len(),
        60
    );
    assert_eq!(
        resumed.match_json(PlayerId::One)["wins"],
        serde_json::json!([0, 0])
    );
}

#[test]
fn karn_restart_uses_hypothesized_cards_and_scores_a_short_opening_draw_as_a_loss() {
    let (mut game, karn) = staged(MatchMode::OneConclusion);
    game.set_hand(PlayerId::One, &[]).unwrap();
    game.set_library(PlayerId::One, &[cards::FOREST; 5])
        .unwrap();
    ultimate(&mut game, karn);
    assert_eq!(
        game.players[0].hand.len(),
        6,
        "the hypothesized library plus Karn return"
    );
    assert!(
        game.result().is_none(),
        "no state-based loss during pregame"
    );
    keep(&mut game);
    assert_eq!(
        game.result(),
        Some(GameResult::Winner {
            winner: PlayerId::Two,
            reason: crate::WinReason::OpponentTriedToDrawFromEmptyLibrary
        })
    );
}
