//! Regression coverage for Marvel Spider-Man combinations of existing game operations.

use super::*;

fn board(library: &[CardDefinitionId]) -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    for player in &mut game.players {
        player.hand.clear();
        player.graveyard.clear();
    }
    game.players[0].library = game.build_zone(PlayerId::One, library).unwrap();
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.turns_started = [3, 3];
    game.cards_drawn_this_turn = [0, 0];
    game
}

fn settle(game: &mut Game) {
    for _ in 0..64 {
        if let Some(pending) = game.pending_decisions.first() {
            let decision = pending.observation.clone();
            let options = if matches!(
                pending.continuation,
                DecisionContinuation::OptionalEffect { .. } | DecisionContinuation::PayOr { .. }
            ) {
                vec![decision.options.last().unwrap().id]
            } else {
                decision
                    .options
                    .iter()
                    .map(|o| o.id)
                    .take(decision.minimum.max(1).min(decision.maximum))
                    .collect()
            };
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .unwrap();
        } else if game.stack.is_empty() && game.pending_triggers.is_empty() {
            return;
        } else {
            game.apply(game.priority, Action::PassPriority).unwrap();
        }
    }
    panic!("resolution did not settle");
}

fn held(game: &mut Game, definition: CardDefinitionId) -> GameObjectId {
    let cards = game.build_zone(PlayerId::One, &[definition]).unwrap();
    let id = cards[0].id;
    game.players[0].hand.extend(cards);
    for color in ManaColor::COLORS {
        game.add_unrestricted_mana(PlayerId::One, color, 10);
    }
    id
}

fn permanent(game: &Game, definition: CardDefinitionId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|p| p.card.definition == definition)
        .unwrap()
}

fn activate(game: &mut Game, id: GameObjectId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a, Action::ActivateAbility {source,..} | Action::ActivateManaAbility {source,..} if *source == id))
        .expect("ability is available");
    game.apply(PlayerId::One, action).unwrap();
    settle(game);
}

fn cast(game: &mut Game, definition: CardDefinitionId) {
    let id = held(game, definition);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a,Action::CastSpell{card,..} if *card==id))
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    settle(game);
}

#[test]
fn web_slinging_requires_and_returns_a_tapped_creature_as_a_cost() {
    let mut game = board(&[]);
    let bear = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    let id = held(&mut game, cards::SPIDER_MAN_WEB_SLINGER);
    game.players[0].mana_pool = ManaPool::default();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 1);
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|a| matches!(a,Action::CastSpell {card,..} if *card==id))
    );
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == bear)
        .unwrap()
        .tapped = true;
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a,Action::CastSpell {card,..} if *card==id))
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    assert!(!game.battlefield.iter().any(|p| p.card.id == bear));
    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|c| c.definition == cards::GRIZZLY_BEARS)
    );
    settle(&mut game);
    assert_eq!(
        game.power(permanent(&game, cards::SPIDER_MAN_WEB_SLINGER)),
        Some(3)
    );
    assert_eq!(game.players[0].mana_pool.total(), 0);
}

#[test]
fn spiders_man_rewards_only_the_named_alternative_cost() {
    for alternative in [false, true] {
        let mut game = board(&[]);
        let bear = game
            .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
            .unwrap();
        game.battlefield
            .iter_mut()
            .find(|p| p.card.id == bear)
            .unwrap()
            .tapped = true;
        let id = held(&mut game, cards::SPIDERS_MAN_HEROIC_HORDE);
        let action=game.legal_actions(PlayerId::One).into_iter().find(|a|matches!(a,Action::CastSpell{card,choices,..}if *card==id&&choices.costs().alternative().is_some()==alternative)).unwrap();
        game.apply(PlayerId::One, action).unwrap();
        settle(&mut game);
        assert_eq!(game.players[0].life, if alternative { 23 } else { 20 });
        assert_eq!(
            game.battlefield
                .iter()
                .filter(|p| p.card.definition.is_token())
                .count(),
            if alternative { 2 } else { 0 }
        );
    }
}

#[test]
fn connive_counts_only_nonland_cards_actually_discarded() {
    for discarded in [cards::FOREST, cards::GRIZZLY_BEARS] {
        let mut game = board(&[discarded]);
        game.put_onto_battlefield(PlayerId::One, cards::MOB_LOOKOUT)
            .unwrap();
        settle(&mut game);
        assert_eq!(game.players[0].graveyard[0].definition, discarded);
        assert_eq!(
            permanent(&game, cards::MOB_LOOKOUT).counters(CounterKind::PlusOnePlusOne),
            u16::from(discarded == cards::GRIZZLY_BEARS)
        );
    }
}

#[test]
fn unstable_experiment_without_a_creature_target_does_not_connive() {
    let mut game = board(&[cards::FOREST, cards::ISLAND]);
    let id = held(&mut game, cards::UNSTABLE_EXPERIMENT);
    let action=game.legal_actions(PlayerId::One).into_iter().find(|a|matches!(a,Action::CastSpell{card,choices,..}if *card==id&&choices.iter_targets().eq([&Target::Player(PlayerId::One)]))).unwrap();
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    assert_eq!(game.players[0].hand.len(), 1);
    assert_eq!(game.players[0].library.len(), 1);
    assert_eq!(game.players[0].graveyard.len(), 1);
}

#[test]
fn aunt_mays_spider_counter_survives_her_departure() {
    let mut game = board(&[]);
    let aunt = game
        .put_onto_battlefield(PlayerId::One, cards::AUNT_MAY)
        .unwrap();
    let spider = game
        .put_onto_battlefield(PlayerId::One, cards::SPIDER_MAN_WEB_SLINGER)
        .unwrap();
    game.destroy_permanents(&[aunt], false);
    settle(&mut game);
    assert_eq!(game.players[0].life, 21);
    assert_eq!(
        game.battlefield
            .iter()
            .find(|p| p.card.id == spider)
            .unwrap()
            .counters(CounterKind::PlusOnePlusOne),
        1
    );
}

#[test]
fn police_captain_uses_last_known_counters_after_leaving() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    let captain = game
        .put_onto_battlefield(PlayerId::One, cards::SELFLESS_POLICE_CAPTAIN)
        .unwrap();
    game.add_counters_to_permanent(captain, CounterKind::PlusOnePlusOne, 2);
    game.destroy_permanents(&[captain], false);
    settle(&mut game);
    assert_eq!(
        permanent(&game, cards::GRIZZLY_BEARS).counters(CounterKind::PlusOnePlusOne),
        3
    );
}

#[test]
fn hydro_man_becomes_a_mana_producing_land_until_its_controllers_next_turn() {
    let mut game = board(&[]);
    let hydro = game
        .put_onto_battlefield(PlayerId::One, cards::HYDRO_MAN_FLUID_FELON)
        .unwrap();
    game.step = Step::PostcombatMain;
    game.advance_step();
    settle(&mut game);
    let p = permanent(&game, cards::HYDRO_MAN_FLUID_FELON);
    assert!(game.permanent_types(p).unwrap().contains(CardType::Land));
    assert!(
        !game
            .permanent_types(p)
            .unwrap()
            .contains(CardType::Creature)
    );
    activate(&mut game, hydro);
    assert_eq!(game.players[0].mana_pool.total(), 1);
    game.players[0].mana_pool = ManaPool::default();
    game.commit_next_turn(PlayerId::Two, Vec::new());
    assert!(
        !game
            .permanent_types(permanent(&game, cards::HYDRO_MAN_FLUID_FELON))
            .unwrap()
            .contains(CardType::Creature)
    );
    game.commit_next_turn(PlayerId::One, Vec::new());
    assert!(
        game.permanent_types(permanent(&game, cards::HYDRO_MAN_FLUID_FELON))
            .unwrap()
            .contains(CardType::Creature)
    );
}

#[test]
fn mysterios_delayed_exile_tracks_only_the_tokens_it_created() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::COMMON_CROOK)
        .unwrap();
    let mysterio = game
        .put_onto_battlefield(PlayerId::One, cards::MYSTERIO_MASTER_OF_ILLUSION)
        .unwrap();
    settle(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| p.card.definition.is_token())
            .count(),
        2
    );
    game.destroy_permanents(&[mysterio], false);
    settle(&mut game);
    assert!(
        !game
            .battlefield
            .iter()
            .any(|p| p.card.definition.is_token())
    );
    assert!(
        game.battlefield
            .iter()
            .any(|p| p.card.definition == cards::COMMON_CROOK)
    );
}

#[test]
fn hide_on_the_ceiling_returns_both_owners_creatures_at_the_next_end_step() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    game.put_onto_battlefield(PlayerId::Two, cards::HILL_GIANT)
        .unwrap();
    let id = held(&mut game, cards::HIDE_ON_THE_CEILING);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a,Action::CastSpell{card,choices,..}if *card==id&&choices.x()==2))
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    assert!(game.battlefield.is_empty());
    game.players[0].mana_pool = ManaPool::default();
    game.step = Step::PostcombatMain;
    game.advance_step();
    settle(&mut game);
    assert_eq!(
        permanent(&game, cards::GRIZZLY_BEARS).controller,
        PlayerId::One
    );
    assert_eq!(
        permanent(&game, cards::HILL_GIANT).controller,
        PlayerId::Two
    );
}

#[test]
fn camera_copies_a_targeted_ability_and_spends_a_film_counter() {
    let mut game = board(&[cards::FOREST; 4]);
    let team = game
        .put_onto_battlefield(PlayerId::One, cards::OSCORP_RESEARCH_TEAM)
        .unwrap();
    let camera = game
        .put_onto_battlefield(PlayerId::One, cards::PETER_PARKER_S_CAMERA)
        .unwrap();
    for color in ManaColor::COLORS {
        game.add_unrestricted_mana(PlayerId::One, color, 10);
    }
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a,Action::ActivateAbility{source,..}if *source==team))
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    activate(&mut game, camera);
    assert_eq!(game.players[0].hand.len(), 4);
    assert_eq!(
        permanent(&game, cards::PETER_PARKER_S_CAMERA).counters(CounterKind::named("film")),
        2
    );
}

#[test]
fn rent_is_due_taps_two_creatures_or_sacrifices_itself() {
    for creatures in [1, 2] {
        let mut game = board(&[cards::FOREST]);
        for _ in 0..creatures {
            game.put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
                .unwrap();
        }
        game.put_onto_battlefield(PlayerId::One, cards::RENT_IS_DUE)
            .unwrap();
        game.step = Step::PostcombatMain;
        game.advance_step();
        settle(&mut game);
        assert_eq!(game.players[0].hand.len(), usize::from(creatures == 2));
        assert_eq!(
            game.battlefield
                .iter()
                .any(|p| p.card.definition == cards::RENT_IS_DUE),
            creatures == 2
        );
        assert_eq!(
            game.battlefield.iter().filter(|p| p.tapped).count(),
            if creatures == 2 { 2 } else { 0 }
        );
    }
}

#[test]
fn amazing_acrobatics_can_counter_and_tap_with_both_modes() {
    let mut game = board(&[]);
    let enemy = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .unwrap();
    let spell = held(&mut game, cards::OSCORP_RESEARCH_TEAM);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a,Action::CastSpell{card,..}if *card==spell))
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    let acro = held(&mut game, cards::AMAZING_ACROBATICS);
    let action=game.legal_actions(PlayerId::One).into_iter().find(|a|matches!(a,Action::CastSpell{card,choices,..}if *card==acro&&choices.modes().len()==2)).unwrap();
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    assert!(
        game.battlefield
            .iter()
            .find(|p| p.card.id == enemy)
            .unwrap()
            .tapped
    );
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|c| c.definition == cards::OSCORP_RESEARCH_TEAM)
    );
}

#[test]
fn scout_the_city_gains_life_even_when_nothing_can_be_returned() {
    let mut game = board(&[cards::SCORPION_S_STING; 3]);
    cast(&mut game, cards::SCOUT_THE_CITY);
    assert_eq!(game.players[0].life, 23);
    assert!(game.players[0].hand.is_empty());
    assert_eq!(game.players[0].graveyard.len(), 4);
}

#[test]
fn venom_retains_the_sacrificed_creatures_mana_value_through_drawing() {
    let mut game = board(&[cards::FOREST; 2]);
    let eddie = game
        .put_onto_battlefield(PlayerId::One, cards::EDDIE_BROCK)
        .unwrap();
    settle(&mut game);
    let victim = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    let arrival = held(&mut game, cards::GRIZZLY_BEARS);
    game.priority = PlayerId::One;
    game.step = Step::PrecombatMain;
    activate(&mut game, eddie);
    game.players[0].mana_pool = ManaPool::default();
    game.step = Step::DeclareAttackers;
    game.declare_attacker(eddie, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    settle(&mut game);
    assert_eq!(game.players[0].hand.len(), 2);
    assert!(
        game.players[0]
            .hand
            .iter()
            .all(|c| c.definition == cards::FOREST)
    );
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|c| c.definition == cards::GRIZZLY_BEARS)
    );
    assert!(!game.battlefield.iter().any(|p| p.card.id == victim));
    assert!(!game.players[0].hand.iter().any(|c| c.id == arrival));
    assert_eq!(game.battlefield.len(), 2);
}
