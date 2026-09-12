//! Regression coverage for Eldraine's combinations of existing game operations.

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

fn cast_action(game: &Game, id: GameObjectId, option: PlayOptionId) -> Action {
    game.legal_actions(PlayerId::One).into_iter().find(|action| {
        matches!(action, Action::CastSpell {card, choices, ..} if *card == id && choices.play_option() == option)
    }).expect("the requested form is castable")
}

fn cast(game: &mut Game, definition: CardDefinitionId) {
    let id = held(game, definition);
    let action = cast_action(game, id, PlayOptionId::DEFAULT);
    game.apply(PlayerId::One, action).unwrap();
    settle(game);
}

fn permanent(game: &Game, definition: CardDefinitionId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|p| p.card.definition == definition)
        .unwrap()
}

#[test]
fn return_triumphant_attaches_the_role_to_the_new_object() {
    let mut game = board(&[]);
    game.players[0].graveyard = game
        .build_zone(PlayerId::One, &[cards::GRIZZLY_BEARS])
        .unwrap();
    let old = game.players[0].graveyard[0].id;
    cast(&mut game, cards::RETURN_TRIUMPHANT);
    let returned = permanent(&game, cards::GRIZZLY_BEARS).card.id;
    assert_ne!(old, returned);
    assert!(
        game.battlefield
            .iter()
            .any(|p| p.card.definition.is_token() && p.attached_to == Some(returned))
    );
    assert_eq!(game.battlefield.len(), 2);
}

#[test]
fn not_dead_after_all_returns_tapped_and_its_wicked_role_triggers_on_death() {
    let mut game = board(&[]);
    let old = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    cast(&mut game, cards::NOT_DEAD_AFTER_ALL);
    game.move_permanents_to_graveyard(&[old]);
    settle(&mut game);
    let returned = permanent(&game, cards::GRIZZLY_BEARS);
    let id = returned.card.id;
    assert_ne!(id, old);
    assert!(returned.tapped);
    assert_eq!(game.power(returned), Some(3));
    let role = game
        .battlefield
        .iter()
        .find(|p| p.attached_to == Some(id))
        .unwrap()
        .card
        .id;
    let before = game.players[1].life;
    game.move_permanents_to_graveyard(&[role]);
    settle(&mut game);
    assert_eq!(game.players[1].life, before - 1);
}

#[test]
fn picklock_adventure_only_returns_a_card_from_its_own_mill() {
    let mut game = board(&[cards::ISLAND, cards::FOREST, cards::MOUNTAIN, cards::PLAINS]);
    game.players[0].graveyard = game
        .build_zone(PlayerId::One, &[cards::LIGHTNING_BOLT])
        .unwrap();
    let id = held(&mut game, cards::PICKLOCK_PRANKSTER);
    let action = cast_action(&game, id, PlayOptionId(1));
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    assert!(game.players[0].hand.is_empty());
    assert_eq!(game.players[0].graveyard.len(), 5);
    let exiled = game.players[0].exile[0].id;
    assert_eq!(
        game.players[0].exile[0].definition,
        cards::PICKLOCK_PRANKSTER
    );
    assert!(!game.legal_actions(PlayerId::One).iter().any(|a| matches!(a, Action::CastSpell {card,choices,..} if *card==exiled && choices.play_option()==PlayOptionId(1))));
    let action = cast_action(&game, exiled, PlayOptionId::DEFAULT);
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    assert!(game.players[0].exile.is_empty());
    assert_eq!(
        game.power(permanent(&game, cards::PICKLOCK_PRANKSTER)),
        Some(1)
    );
}

#[test]
fn storyteller_pixie_draws_for_the_adventure_but_not_the_creature() {
    let mut game = board(&[cards::ISLAND, cards::FOREST]);
    game.put_onto_battlefield(PlayerId::One, cards::STORYTELLER_PIXIE)
        .unwrap();
    let id = held(&mut game, cards::FROLICKING_FAMILIAR);
    let action = cast_action(&game, id, PlayOptionId(1));
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    // The Adventure cast triggers the Pixie before the spell resolves.
    assert_eq!(game.cards_drawn_this_turn[0], 1);
    let draws = game.cards_drawn_this_turn[0];
    let exiled = game.players[0]
        .exile
        .iter()
        .find(|c| c.definition == cards::FROLICKING_FAMILIAR)
        .unwrap()
        .id;
    let action = cast_action(&game, exiled, PlayOptionId::DEFAULT);
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    assert_eq!(game.cards_drawn_this_turn[0], draws);
}

#[test]
fn eerie_interference_protects_creatures_that_enter_after_it_resolves() {
    let mut game = board(&[]);
    let attacker = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .unwrap();
    cast(&mut game, cards::EERIE_INTERFERENCE);
    let late = game
        .put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .unwrap();
    let before = game.players[0].life;
    game.deal_damage_simultaneously(vec![
        DamageAssignment {
            source: Some(attacker),
            target: Some(Target::Permanent(late)),
            amount: 3,
            combat: false,
        },
        DamageAssignment {
            source: Some(attacker),
            target: Some(Target::Player(PlayerId::One)),
            amount: 3,
            combat: false,
        },
    ]);
    game.check_state_based_actions();
    assert!(game.battlefield.iter().any(|p| p.card.id == late));
    assert_eq!(game.players[0].life, before);
    game.finish_cleanup();
    game.deal_damage_simultaneously(vec![DamageAssignment {
        source: Some(attacker),
        target: Some(Target::Player(PlayerId::One)),
        amount: 3,
        combat: false,
    }]);
    assert_eq!(game.players[0].life, before - 3);
}

#[test]
fn witchkite_counts_a_sacrificed_token_after_it_ceases_to_exist() {
    let mut game = board(&[cards::ISLAND, cards::FOREST]);
    game.players[0].graveyard = game
        .build_zone(PlayerId::One, &[cards::GRIZZLY_BEARS])
        .unwrap();
    cast(&mut game, cards::RETURN_TRIUMPHANT);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| p.card.definition.is_token())
            .count(),
        1
    );
    game.put_onto_battlefield(PlayerId::One, cards::MALEVOLENT_WITCHKITE)
        .unwrap();
    settle(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| p.card.definition.is_token())
            .count(),
        0
    );
    assert_eq!(game.players[0].hand.len(), 1);
    assert_eq!(game.players[0].library.len(), 1);
}

#[test]
fn three_blind_mice_reads_both_shared_chapters_before_its_last_chapter() {
    let mut game = board(&[]);
    let saga = game
        .put_onto_battlefield(PlayerId::One, cards::THREE_BLIND_MICE)
        .unwrap();
    settle(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| p.card.definition.is_token())
            .count(),
        1
    );
    for expected in [2, 3] {
        game.add_counters_to_permanent(saga, CounterKind::Lore, 1);
        settle(&mut game);
        assert_eq!(
            game.battlefield
                .iter()
                .filter(|p| p.card.definition.is_token())
                .count(),
            expected
        );
        assert!(game.battlefield.iter().any(|p| p.card.id == saga));
    }
    game.add_counters_to_permanent(saga, CounterKind::Lore, 1);
    settle(&mut game);
    game.check_state_based_actions();
    assert!(!game.battlefield.iter().any(|p| p.card.id == saga));
}

#[test]
fn bargain_entry_trigger_remembers_payment_after_the_negotiator_leaves() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::ORNITHOPTER)
        .unwrap();
    let id = held(&mut game, cards::HIGH_FAE_NEGOTIATOR);
    let action = game.legal_actions(PlayerId::One).into_iter().find(|action| {
        matches!(action,Action::CastSpell {card,choices,..} if *card==id && !choices.costs().additional().is_empty())
    }).unwrap();
    let before = [game.players[0].life, game.players[1].life];
    game.apply(PlayerId::One, action).unwrap();
    for _ in 0..8 {
        if let Some(source) = game
            .battlefield
            .iter()
            .find(|p| p.card.definition == cards::HIGH_FAE_NEGOTIATOR)
            .map(|p| p.card.id)
        {
            game.move_permanents_to_graveyard(&[source]);
            break;
        }
        game.apply(game.priority, Action::PassPriority).unwrap();
    }
    settle(&mut game);
    assert_eq!(game.players[0].life, before[0] + 3);
    assert_eq!(game.players[1].life, before[1] - 3);
}

#[test]
fn prodigy_cannot_remove_a_counter_from_its_departed_source() {
    for leaves in [false, true] {
        let mut game = board(&[cards::ISLAND]);
        let mut prodigy = creature(96_100, cards::INGENIOUS_PRODIGY, PlayerId::One);
        let id = prodigy.card.id;
        prodigy.add_counters(CounterKind::PlusOnePlusOne, 2);
        game.battlefield.push(prodigy);
        game.step = Step::Upkeep;
        game.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
            step: TurnStepDef::Upkeep,
            player: PlayerId::One,
        });
        if leaves {
            game.move_permanents_to_graveyard(&[id]);
        }
        settle(&mut game);
        assert_eq!(game.players[0].hand.len(), usize::from(!leaves));
        if !leaves {
            assert_eq!(
                permanent(&game, cards::INGENIOUS_PRODIGY).counters(CounterKind::PlusOnePlusOne),
                1
            );
        }
    }
}

#[test]
fn rotisserie_uses_its_last_counter_count_after_sacrificing_itself() {
    let mut game = board(&[
        cards::ISLAND,
        cards::FOREST,
        cards::MOUNTAIN,
        cards::PLAINS,
        cards::SWAMP,
    ]);
    let id = game
        .put_onto_battlefield(PlayerId::One, cards::ROTISSERIE_ELEMENTAL)
        .unwrap();
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == id)
        .unwrap()
        .add_counters(CounterKind::named("skewer"), 3);
    game.deal_damage_simultaneously(vec![DamageAssignment {
        source: Some(id),
        target: Some(Target::Player(PlayerId::Two)),
        amount: 1,
        combat: true,
    }]);
    settle(&mut game);
    assert!(!game.battlefield.iter().any(|p| p.card.id == id));
    assert_eq!(game.players[0].exile.len(), 4);
    assert_eq!(game.players[0].library.len(), 1);
}

#[test]
fn conquest_returns_a_creature_for_a_sacrificed_role() {
    let mut game = board(&[]);
    game.players[0].graveyard = game
        .build_zone(PlayerId::One, &[cards::GRIZZLY_BEARS])
        .unwrap();
    cast(&mut game, cards::RETURN_TRIUMPHANT);
    let angel = game
        .build_zone(PlayerId::One, &[cards::SERRA_ANGEL])
        .unwrap();
    game.players[0].graveyard.extend(angel);
    cast(&mut game, cards::LICH_KNIGHTS_CONQUEST);
    assert!(
        game.battlefield
            .iter()
            .any(|p| p.card.definition == cards::SERRA_ANGEL)
    );
    assert!(
        !game
            .battlefield
            .iter()
            .any(|p| p.card.definition.is_token())
    );
}

#[test]
fn borrowed_adventure_permission_belongs_to_the_caster_instead_of_the_owner() {
    let mut game = board(&[]);
    game.players[1].library = game
        .build_zone(
            PlayerId::Two,
            &[cards::ISLAND, cards::FOREST, cards::MOUNTAIN, cards::PLAINS],
        )
        .unwrap();
    let borrowed = game
        .build_zone(PlayerId::One, &[cards::PICKLOCK_PRANKSTER])
        .unwrap();
    let old = borrowed[0].id;
    game.players[0].exile.extend(borrowed);
    game.permit_free_play_this_turn(old, PlayerId::Two);
    game.priority = PlayerId::Two;
    let cast = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell {card, choices, ..}
            if *card == old && choices.play_option() == PlayOptionId(1))
        })
        .unwrap();
    game.apply(PlayerId::Two, cast).unwrap();
    settle(&mut game);
    let exiled = game.players[0].exile[0].id;
    assert_ne!(old, exiled);
    assert!(game.exile_play_permission(exiled, PlayerId::One).is_none());
    assert!(game.exile_play_permission(exiled, PlayerId::Two).is_some());
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::Two;
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Blue, 2);
    let cast = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell {card, choices, ..}
            if *card == exiled && choices.play_option() == PlayOptionId::DEFAULT)
        })
        .unwrap();
    game.apply(PlayerId::Two, cast).unwrap();
    settle(&mut game);
    assert_eq!(
        permanent(&game, cards::PICKLOCK_PRANKSTER).controller,
        PlayerId::Two
    );
}
