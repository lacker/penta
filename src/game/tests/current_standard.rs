//! Rules exercised by the Worlds 32 card additions.
use super::*;

fn hold(game: &mut Game, definition: CardDefinitionId) -> GameObjectId {
    let card = game
        .build_zone(PlayerId::One, &[definition])
        .unwrap()
        .remove(0);
    let id = card.id;
    game.players[0].hand.push(card);
    id
}

fn cast(game: &mut Game, card_id: GameObjectId, target: Option<Target>, x: u16) {
    game.priority = PlayerId::One;
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. } if *card == card_id
            && choices.x() == x
            && target.is_none_or(|wanted| choices.iter_targets().any(|target| *target == wanted)))
        })
        .expect("a legal cast");
    game.apply(PlayerId::One, action).unwrap();
}

fn activate(game: &mut Game, source_id: GameObjectId) {
    game.priority = PlayerId::One;
    let action = game.legal_actions(PlayerId::One).into_iter().find(|action| {
        matches!(action, Action::ActivateAbility { source, .. } if *source == source_id)
    }).expect("a legal activation");
    game.apply(PlayerId::One, action).unwrap();
}

fn count_tokens(game: &Game, subtype: &str) -> usize {
    game.battlefield
        .iter()
        .filter(|p| {
            p.card.definition.is_token() && game.object_subtypes(p.card.id).contains(&subtype)
        })
        .count()
}

#[test]
fn mission_crosses_its_threshold_and_reads_last_known_counters() {
    let mut game = ready_game();
    let mission = game
        .put_onto_battlefield(PlayerId::One, cards::ANTHROPOLOGICAL_MISSION)
        .unwrap();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 10);
    for expected in 1..=3 {
        let held = hold(&mut game, cards::SPELLBOOK);
        let before = game.players[0].library.len();
        cast(&mut game, held, None, 0);
        drain_pending(&mut game);
        assert_eq!(game.players[0].library.len(), before - 1);
        assert_eq!(
            game.current_or_last_known_counters(mission, CounterKind::named("quest")),
            expected
        );
        assert_eq!(game.players[0].hand.len(), usize::from(expected == 3));
    }
    let held = hold(&mut game, cards::SPELLBOOK);
    cast(&mut game, held, None, 0);
    game.destroy_permanent(mission);
    drain_pending(&mut game);
    assert_eq!(
        game.players[0].hand.len(),
        2,
        "three last-known counters still prevent discarding"
    );
}

#[test]
fn essence_burn_restricts_targets_and_exiles_lethal_damage() {
    let mut game = ready_game();
    let green = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .unwrap();
    let white = game
        .put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .unwrap();
    let spell = hold(&mut game, cards::ESSENCE_BURN);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 2);
    let actions = game.legal_actions(PlayerId::One);
    assert!(!actions.iter().any(
        |action| matches!(action, Action::CastSpell { card, choices, .. }
        if *card == spell && choices.iter_targets().any(|t| *t == Target::Permanent(white)))
    ));
    cast(&mut game, spell, Some(Target::Permanent(green)), 0);
    drain_pending(&mut game);
    assert!(
        game.players[1]
            .exile
            .iter()
            .any(|c| c.definition == cards::GRIZZLY_BEARS)
    );
    assert!(game.players[1].graveyard.is_empty());
}

#[test]
fn cosmozoan_counts_only_noncreature_nonland_graveyard_cards() {
    let mut game = ready_game();
    game.players[0].graveyard = game
        .build_zone(
            PlayerId::One,
            &[
                cards::MOUNTAIN,
                cards::GRIZZLY_BEARS,
                cards::SPELLBOOK,
                cards::LIGHTNING_BOLT,
                cards::COUNTERSPELL,
                cards::DARK_RITUAL,
                cards::DISENCHANT,
            ],
        )
        .unwrap();
    let fish = hold(&mut game, cards::RAVENOUS_COSMOZOAN);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    cast(&mut game, fish, None, 0);
    drain_pending(&mut game);
    let fish = game
        .battlefield
        .iter()
        .find(|p| p.card.definition == cards::RAVENOUS_COSMOZOAN)
        .unwrap();
    assert!(game.has_flying(fish));
    assert!(game.permanent_has_executable_keyword(fish, KeywordAbility::Haste));
}

#[test]
fn sphinx_dies_into_one_copy_and_its_token_does_not_copy_itself() {
    let mut game = ready_game();
    let sphinx = game
        .put_onto_battlefield(PlayerId::One, cards::SPHINX_OF_FALSE_CONCLUSIONS)
        .unwrap();
    game.destroy_permanent(sphinx);
    drain_pending(&mut game);
    assert_eq!(count_tokens(&game, "Sphinx"), 1);
    let copy = game
        .battlefield
        .iter()
        .find(|p| p.card.definition.is_token())
        .unwrap()
        .card
        .id;
    game.destroy_permanent(copy);
    drain_pending(&mut game);
    assert_eq!(count_tokens(&game, "Sphinx"), 0);
}

#[test]
fn karn_suppresses_artifact_and_creature_entries_until_it_leaves() {
    let mut game = ready_game();
    let karn = game
        .put_onto_battlefield(PlayerId::One, cards::KARN_ARGENT_DEFENDER)
        .unwrap();
    let life = game.players[0].life;
    game.put_onto_battlefield(PlayerId::One, cards::THRAGTUSK)
        .unwrap();
    drain_pending(&mut game);
    assert_eq!(game.players[0].life, life);
    game.destroy_permanent(karn);
    game.put_onto_battlefield(PlayerId::One, cards::THRAGTUSK)
        .unwrap();
    drain_pending(&mut game);
    assert_eq!(game.players[0].life, life + 5);
}

#[test]
fn ooze_counts_departing_counters_after_the_source_also_leaves() {
    let mut game = ready_game();
    let ooze = game
        .put_onto_battlefield(PlayerId::One, cards::THE_OOZE)
        .unwrap();
    let bear = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == bear)
        .unwrap()
        .set_counters(CounterKind::PlusOnePlusOne, 3);
    game.destroy_permanent(bear);
    game.destroy_permanent(ooze);
    drain_pending(&mut game);
    assert_eq!(count_tokens(&game, "Mutagen"), 3);
}

#[test]
fn starcage_returns_exiles_immediately_when_it_leaves() {
    let mut game = ready_game();
    game.put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .unwrap();
    let starcage = game
        .put_onto_battlefield(PlayerId::One, cards::PINNACLE_STARCAGE)
        .unwrap();
    drain_pending(&mut game);
    assert_eq!(game.players[1].exile.len(), 1);
    game.destroy_permanent(starcage);
    assert!(
        game.players[1].exile.is_empty(),
        "return is a duration, not a leaves trigger"
    );
    assert!(
        game.battlefield
            .iter()
            .any(|p| p.card.definition == cards::GRIZZLY_BEARS)
    );
}

#[test]
fn starcage_activation_counts_only_cards_that_reach_graveyards() {
    let mut game = ready_game();
    game.put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .unwrap();
    game.put_onto_battlefield(PlayerId::One, cards::SPELLBOOK)
        .unwrap();
    let starcage = game
        .put_onto_battlefield(PlayerId::One, cards::PINNACLE_STARCAGE)
        .unwrap();
    drain_pending(&mut game);
    assert_eq!(game.linked_exile_ids(starcage).len(), 2);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 8);
    activate(&mut game, starcage);
    drain_pending(&mut game);
    assert_eq!(count_tokens(&game, "Robot"), 2);
    assert!(game.players.iter().all(|p| p.exile.is_empty()));
    assert!(!game.battlefield.iter().any(|p| p.card.id == starcage));
    assert!(
        game.players[1]
            .graveyard
            .iter()
            .any(|c| c.definition == cards::GRIZZLY_BEARS)
    );
}

#[test]
fn starcage_does_not_make_robots_for_replaced_graveyard_moves() {
    let mut game = ready_game();
    game.put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .unwrap();
    let starcage = game
        .put_onto_battlefield(PlayerId::One, cards::PINNACLE_STARCAGE)
        .unwrap();
    drain_pending(&mut game);
    game.put_onto_battlefield(PlayerId::One, cards::REST_IN_PEACE)
        .unwrap();
    drain_pending(&mut game);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 8);
    activate(&mut game, starcage);
    drain_pending(&mut game);
    assert_eq!(count_tokens(&game, "Robot"), 0);
}

fn rebuild(game: &Game) -> Game {
    let (wire, hidden) = checkpoint_fixture(game, PlayerId::One);
    Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 0)
        .expect("current Standard checkpoint")
}

fn accept_pending(game: &mut Game) {
    for _ in 0..64 {
        if game.stack.is_empty()
            && game.pending_triggers.is_empty()
            && game.pending_decisions.is_empty()
        {
            return;
        }
        if let Some(pending) = game.pending_decisions.first() {
            let decision = pending.observation.clone();
            let optional = matches!(
                pending.continuation,
                DecisionContinuation::PayOr { .. } | DecisionContinuation::OptionalEffect { .. }
            );
            let options = if optional {
                vec![decision.options.last().unwrap().id]
            } else {
                decision
                    .options
                    .iter()
                    .take(decision.minimum.max(1).min(decision.maximum))
                    .map(|o| o.id)
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
        } else {
            game.apply(game.priority, Action::PassPriority).unwrap();
        }
    }
    panic!("effect did not finish");
}

#[test]
fn prepared_frames_cast_once_and_survive_checkpoints() {
    for (definition, color, name) in [
        (
            cards::POMPOUS_BATTLEMAGE_IMPROVISED_ACT,
            ManaColor::Red,
            "Improvised Act",
        ),
        (
            cards::CARNIVOROUS_CULTIVATOR_ENROOT,
            ManaColor::Green,
            "Enroot",
        ),
    ] {
        let mut game = ready_game();
        game.players[0].library = game.build_zone(PlayerId::One, &[cards::FOREST; 5]).unwrap();
        let source = game
            .put_onto_battlefield(PlayerId::One, definition)
            .unwrap();
        let physical = hold(&mut game, definition);
        hold(&mut game, cards::MOUNTAIN);
        game.add_unrestricted_mana(PlayerId::One, color, 1);
        assert!(
            game.legal_actions(PlayerId::One)
                .iter()
                .filter_map(|action| {
                    if let Action::CastSpell { card, choices, .. } = action {
                        (*card == physical).then_some(choices.play_option())
                    } else {
                        None
                    }
                })
                .all(|option| option == crate::PlayOptionId::DEFAULT)
        );
        let copy = game.players[0].exile[0].id;
        assert_eq!(game.object_card_name(copy).as_deref(), Some(name));
        let mut restored = rebuild(&game);
        for state in [&mut game, &mut restored] {
            cast(state, copy, None, 0);
            assert!(state.stack.iter().any(|spell| spell.is_copy));
            assert!(
                state
                    .battlefield
                    .iter()
                    .find(|p| p.card.id == source)
                    .unwrap()
                    .designations
                    .is_empty()
            );
            accept_pending(state);
            assert!(state.players[0].exile.is_empty());
            assert!(
                state.players[0]
                    .graveyard
                    .iter()
                    .all(|c| c.backing != ObjectBacking::None)
            );
            if color == ManaColor::Green {
                assert!(
                    state.players[0]
                        .graveyard
                        .iter()
                        .any(|c| c.definition == cards::FOREST)
                );
                assert_eq!(state.players[0].library.len(), 4);
            } else {
                assert_eq!(
                    state.players[0].library.len(),
                    4,
                    "discarding draws one card"
                );
            }
        }
    }
}

#[test]
fn hydra_uses_cast_x_and_distributes_only_after_paying_a_counter() {
    let mut game = ready_game();
    let bear = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    let hydra = hold(&mut game, cards::GUIDING_HYDRA);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 4);
    cast(&mut game, hydra, None, 3);
    accept_pending(&mut game);
    let hydra = game
        .battlefield
        .iter()
        .find(|p| p.card.definition == cards::GUIDING_HYDRA)
        .unwrap()
        .card
        .id;
    assert_eq!(
        game.current_or_last_known_counters(hydra, CounterKind::PlusOnePlusOne),
        3
    );
    game.step = Step::BeginningOfCombat;
    game.begin_step_triggers();
    accept_pending(&mut game);
    assert_eq!(
        game.current_or_last_known_counters(hydra, CounterKind::PlusOnePlusOne),
        2
    );
    assert_eq!(
        game.current_or_last_known_counters(bear, CounterKind::PlusOnePlusOne),
        1
    );
}

#[test]
fn leatherhead_removes_a_counter_before_choosing_a_reflexive_target() {
    for remove_source in [false, true] {
        let mut game = ready_game();
        let leatherhead = game
            .put_onto_battlefield(PlayerId::One, cards::LEATHERHEAD_SWAMP_STALKER)
            .unwrap();
        let own = game
            .put_onto_battlefield(PlayerId::One, cards::SPELLBOOK)
            .unwrap();
        let enemy = game
            .put_onto_battlefield(PlayerId::Two, cards::SPELLBOOK)
            .unwrap();
        let attacker = game
            .battlefield
            .iter_mut()
            .find(|p| p.card.id == leatherhead)
            .unwrap();
        assert_eq!(attacker.counters(CounterKind::Hexproof), 1);
        attacker.attacking = true;
        attacker.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
        game.deal_combat_damage();
        if remove_source {
            game.destroy_permanent(leatherhead);
        }
        accept_pending(&mut game);
        assert!(game.battlefield.iter().any(|p| p.card.id == own));
        assert_eq!(
            game.battlefield.iter().any(|p| p.card.id == enemy),
            remove_source
        );
        if !remove_source {
            assert_eq!(
                game.current_or_last_known_counters(leatherhead, CounterKind::Hexproof),
                0
            );
        }
    }
}

#[test]
fn starcage_linked_duration_exiles_reconstruct_before_activation() {
    let mut game = ready_game();
    game.put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .unwrap();
    let cage = game
        .put_onto_battlefield(PlayerId::One, cards::PINNACLE_STARCAGE)
        .unwrap();
    accept_pending(&mut game);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 8);
    let mut restored = rebuild(&game);
    for state in [&mut game, &mut restored] {
        activate(state, cage);
        accept_pending(state);
        assert_eq!(count_tokens(state, "Robot"), 1);
        assert!(state.players[1].exile.is_empty());
    }
}

#[test]
fn naked_time_searches_installs_a_turn_trigger_and_sacrifices_a_land() {
    let mut game = ready_game();
    game.players[0].library = game
        .build_zone(PlayerId::One, &[cards::MOUNTAIN, cards::FOREST])
        .unwrap();
    let saga = game
        .put_onto_battlefield(PlayerId::One, cards::THE_NAKED_TIME)
        .unwrap();
    accept_pending(&mut game);
    assert!(
        game.battlefield
            .iter()
            .any(|p| p.card.definition == cards::MOUNTAIN && p.tapped)
    );
    game.place_draw_step_lore_counters(PlayerId::One);
    accept_pending(&mut game);
    let mut restored = rebuild(&game);
    for state in [&mut game, &mut restored] {
        let bear = state
            .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
            .unwrap();
        accept_pending(state);
        let bear = state
            .battlefield
            .iter()
            .find(|p| p.card.id == bear)
            .unwrap();
        assert_eq!(bear.counters(CounterKind::PlusOnePlusOne), 1);
        assert!(state.permanent_has_executable_keyword(bear, KeywordAbility::Haste));
        state.place_draw_step_lore_counters(PlayerId::One);
        accept_pending(state);
        assert!(
            !state
                .battlefield
                .iter()
                .any(|p| p.card.definition == cards::MOUNTAIN)
        );
        assert!(!state.battlefield.iter().any(|p| p.card.id == saga));
    }
}

#[test]
fn cultivator_combat_damage_returns_a_land_card() {
    let mut game = ready_game();
    game.players[0].graveyard = game.build_zone(PlayerId::One, &[cards::FOREST]).unwrap();
    let source = game
        .put_onto_battlefield(PlayerId::One, cards::CARNIVOROUS_CULTIVATOR_ENROOT)
        .unwrap();
    game.deal_combat_damage_to_player(source, PlayerId::Two, 2);
    accept_pending(&mut game);
    assert_eq!(game.players[0].hand[0].definition, cards::FOREST);
    assert!(game.players[0].graveyard.is_empty());
}

#[test]
fn theorist_draws_on_opponents_draw_step_and_creates_an_illusion() {
    let mut game = ready_game();
    let jace = game
        .put_onto_battlefield(PlayerId::One, cards::THE_THEORIST_JACE_BELEREN)
        .unwrap();
    game.active_player = PlayerId::Two;
    game.step = Step::Draw;
    game.begin_step_triggers();
    accept_pending(&mut game);
    assert_eq!(game.players[0].hand.len(), 1);
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    activate(&mut game, jace);
    accept_pending(&mut game);
    assert_eq!(count_tokens(&game, "Illusion"), 1);
}
