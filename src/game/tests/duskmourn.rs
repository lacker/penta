//! Regression coverage for Duskmourn's combinations of existing game operations.

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
        matches!(action, Action::CastSpell {card, choices, ..} if *card == id && choices.play_option() == option && choices.costs().alternative().is_none())
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
fn room_cast_and_unlock_keep_both_halves_and_pay_the_printed_cost() {
    let mut game = board(&[cards::ISLAND; 8]);
    let bear = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    let room = held(&mut game, cards::GRAND_ENTRYWAY);
    let cast = cast_action(&game, room, PlayOptionId::DEFAULT);
    game.apply(PlayerId::One, cast).unwrap();
    settle(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| p.card.definition.is_token())
            .count(),
        1
    );
    let before = game.players[0].mana_pool.total();
    let unlock = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a, Action::UnlockDoor { .. }))
        .unwrap();
    game.apply(PlayerId::One, unlock).unwrap();
    settle(&mut game);
    assert_eq!(before - game.players[0].mana_pool.total(), 3);
    assert_eq!(
        permanent(&game, cards::GRAND_ENTRYWAY).presented,
        CardPartId(2)
    );
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|a| matches!(a, Action::UnlockDoor { .. }))
    );
    assert_eq!(
        game.battlefield
            .iter()
            .find(|p| p.card.id == bear)
            .unwrap()
            .counters(CounterKind::PlusOnePlusOne),
        1
    );
}

#[test]
fn a_room_put_onto_the_battlefield_has_two_locked_doors() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::GRAND_ENTRYWAY)
        .unwrap();
    for color in ManaColor::COLORS {
        game.add_unrestricted_mana(PlayerId::One, color, 8);
    }
    settle(&mut game);
    assert_eq!(game.battlefield.len(), 1);
    assert_eq!(
        game.legal_actions(PlayerId::One)
            .iter()
            .filter(|a| matches!(a, Action::UnlockDoor { .. }))
            .count(),
        2
    );
    game.step = Step::Upkeep;
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|a| matches!(a, Action::UnlockDoor { .. }))
    );
}

#[test]
fn manifest_equipment_attaches_to_the_created_permanent() {
    let mut game = board(&[cards::FOREST, cards::GRIZZLY_BEARS]);
    cast(&mut game, cards::CURSED_WINDBREAKER);
    let windbreaker = permanent(&game, cards::CURSED_WINDBREAKER);
    let host = windbreaker.attached_to.expect("attached to its manifest");
    let manifested = game.battlefield.iter().find(|p| p.card.id == host).unwrap();
    assert!(manifested.face_down.is_some());
    assert_eq!(manifested.controller, PlayerId::One);
    assert_eq!(game.players[0].graveyard.len(), 1);
}

#[test]
fn manifest_dread_for_an_opponent_uses_their_library_and_control() {
    let mut game = board(&[cards::ISLAND; 3]);
    game.players[1].library = game
        .build_zone(PlayerId::Two, &[cards::FOREST, cards::MOUNTAIN])
        .unwrap();
    let bear = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .unwrap();
    let remake = held(&mut game, cards::UNWANTED_REMAKE);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| match a {
            Action::CastSpell { card, choices, .. } => {
                *card == remake
                    && choices
                        .targets()
                        .iter()
                        .any(|s| s.targets().contains(&Target::Permanent(bear)))
            }
            _ => false,
        })
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    assert_eq!(game.players[0].library.len(), 3);
    assert!(game.players[1].library.is_empty());
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| p.face_down.is_some() && p.controller == PlayerId::Two)
            .count(),
        1
    );
    assert_eq!(game.players[1].graveyard.len(), 2);
}

#[test]
fn come_back_wrong_returns_the_destroyed_card_and_sacrifices_that_incarnation() {
    let mut game = board(&[cards::ISLAND; 4]);
    let bear = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .unwrap();
    let spell = held(&mut game, cards::COME_BACK_WRONG);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| match a {
            Action::CastSpell { card, choices, .. } => {
                *card == spell
                    && choices
                        .targets()
                        .iter()
                        .any(|s| s.targets().contains(&Target::Permanent(bear)))
            }
            _ => false,
        })
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    let returned = permanent(&game, cards::GRIZZLY_BEARS).card.id;
    assert_ne!(returned, bear);
    assert_eq!(
        permanent(&game, cards::GRIZZLY_BEARS).controller,
        PlayerId::One
    );
    game.players[0].mana_pool = ManaPool::default();
    game.step = Step::PostcombatMain;
    game.advance_step();
    assert_eq!(game.step, Step::End);
    assert!(game.result.is_none());
    settle(&mut game);
    assert!(!game.battlefield.iter().any(|p| p.card.id == returned));
    assert!(
        game.players[1]
            .graveyard
            .iter()
            .any(|c| c.definition == cards::GRIZZLY_BEARS)
    );
}

#[test]
fn a_normal_overlord_does_not_count_down_time_counters() {
    let mut game = board(&[cards::ISLAND; 5]);
    cast(&mut game, cards::OVERLORD_OF_THE_FLOODPITS);
    let id = permanent(&game, cards::OVERLORD_OF_THE_FLOODPITS).card.id;
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == id)
        .unwrap()
        .add_counters(CounterKind::named("time"), 2);
    game.players[0].mana_pool = ManaPool::default();
    game.step = Step::PostcombatMain;
    game.advance_step();
    assert_eq!(game.step, Step::End);
    assert!(game.result.is_none());
    settle(&mut game);
    assert_eq!(
        permanent(&game, cards::OVERLORD_OF_THE_FLOODPITS).counters(CounterKind::named("time")),
        2
    );
}

#[test]
fn fanatic_draws_only_when_its_controller_discarded() {
    for have_card in [false, true] {
        let mut game = board(&[cards::ISLAND; 5]);
        if have_card {
            game.players[0].hand = game.build_zone(PlayerId::One, &[cards::FOREST]).unwrap();
        }
        game.players[1].hand = game.build_zone(PlayerId::Two, &[cards::MOUNTAIN]).unwrap();
        game.put_onto_battlefield(PlayerId::One, cards::FANATIC_OF_THE_HARROWING)
            .unwrap();
        settle(&mut game);
        assert_eq!(game.players[0].hand.len(), usize::from(have_card));
        assert!(game.players[1].hand.is_empty());
        assert_eq!(game.players[0].library.len(), if have_card { 4 } else { 5 });
    }
}
#[test]
fn coordinated_clobbering_can_use_one_or_two_tapped_damage_sources() {
    for two in [false, true] {
        let mut game = board(&[]);
        let a = game
            .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
            .unwrap();
        let b = game
            .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
            .unwrap();
        let enemy = game
            .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
            .unwrap();
        let spell = held(&mut game, cards::COORDINATED_CLOBBERING);
        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|a0| match a0 {
                Action::CastSpell { card, choices, .. } if *card == spell => {
                    let targets = choices.targets();
                    targets[0].targets() == [Target::Permanent(a)]
                        && targets[1].targets()
                            == if two {
                                vec![Target::Permanent(b)]
                            } else {
                                vec![]
                            }
                        && targets[2].targets() == [Target::Permanent(enemy)]
                }
                _ => false,
            })
            .expect("one or two allied creatures can be selected");
        game.apply(PlayerId::One, action).unwrap();
        settle(&mut game);
        assert!(
            game.battlefield
                .iter()
                .find(|p| p.card.id == a)
                .unwrap()
                .tapped
        );
        assert_eq!(
            game.battlefield
                .iter()
                .find(|p| p.card.id == b)
                .unwrap()
                .tapped,
            two
        );
        assert!(!game.battlefield.iter().any(|p| p.card.id == enemy));
    }
}

#[test]
fn scorching_dragonfire_exiles_a_creature_destroyed_later_that_turn() {
    let mut game = board(&[]);
    let spider = game
        .put_onto_battlefield(PlayerId::Two, cards::GIANT_SPIDER)
        .unwrap();
    cast(&mut game, cards::SCORCHING_DRAGONFIRE);
    assert!(game.battlefield.iter().any(|p| p.card.id == spider));
    game.destroy_permanent(spider);
    settle(&mut game);
    assert!(
        !game.players[1]
            .graveyard
            .iter()
            .any(|c| c.definition == cards::GIANT_SPIDER)
    );
    assert!(
        game.players[1]
            .exile
            .iter()
            .any(|c| c.definition == cards::GIANT_SPIDER)
    );
}

#[test]
fn hovership_leaving_makes_the_exiled_cards_owner_manifest() {
    let mut game = board(&[cards::ISLAND; 4]);
    game.players[1].library = game
        .build_zone(PlayerId::Two, &[cards::MOUNTAIN, cards::FOREST])
        .unwrap();
    game.put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .unwrap();
    cast(&mut game, cards::UNIDENTIFIED_HOVERSHIP);
    assert!(
        game.players[1]
            .exile
            .iter()
            .any(|c| c.definition == cards::GRIZZLY_BEARS)
    );
    let vehicle = permanent(&game, cards::UNIDENTIFIED_HOVERSHIP).card.id;
    game.destroy_permanent(vehicle);
    settle(&mut game);
    assert!(game.players[1].library.is_empty());
    assert_eq!(game.players[0].library.len(), 4);
    assert!(
        game.battlefield
            .iter()
            .any(|p| p.face_down.is_some() && p.controller == PlayerId::Two)
    );
}
