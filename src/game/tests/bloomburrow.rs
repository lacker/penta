//! Regression coverage for Bloomburrow's combinations of existing game operations.

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
fn offspring_is_payable_alongside_omnisciences_free_cast() {
    let mut game = board(&[cards::ISLAND; 6]);
    game.put_onto_battlefield(PlayerId::One, cards::OMNISCIENCE)
        .unwrap();
    let card = game
        .build_zone(PlayerId::One, &[cards::THUNDERTRAP_TRAINER])
        .unwrap()
        .remove(0);
    let id = card.id;
    game.players[0].hand.push(card);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 4);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| {
            matches!(a,
                Action::CastSpell {card, choices, ..} if *card == id
                    && choices.costs().alternative().is_some()
                    && choices.costs().additional().len() == 1
            )
        })
        .expect("free mana cost and independent offspring payment coexist");
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| game.effective_subtypes(p).contains(&"Otter"))
            .count(),
        2
    );
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| p.card.definition.is_token())
            .count(),
        1
    );
}

#[test]
fn beza_evaluates_each_comparison_in_sequence() {
    let mut game = board(&[cards::FOREST; 6]);
    game.players[0].life = 10;
    game.players[1].life = 12;
    game.players[1].hand = game.build_zone(PlayerId::Two, &[cards::FOREST; 3]).unwrap();
    game.put_onto_battlefield(PlayerId::Two, cards::FOREST)
        .unwrap();
    for _ in 0..2 {
        game.put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
            .unwrap();
    }
    game.put_onto_battlefield(PlayerId::One, cards::BEZA_THE_BOUNDING_SPRING)
        .unwrap();
    settle(&mut game);
    assert_eq!(game.players[0].life, 14);
    assert_eq!(game.players[0].hand.len(), 1);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| p.controller == PlayerId::One && p.card.definition.is_token())
            .count(),
        3
    );
}

#[test]
fn splash_portal_checks_the_returned_creatures_type() {
    let mut game = board(&[cards::FOREST; 6]);
    let old = game
        .put_onto_battlefield(PlayerId::One, cards::POND_PROPHET)
        .unwrap();
    settle(&mut game);
    let before = game.players[0].hand.len();
    cast(&mut game, cards::SPLASH_PORTAL);
    assert_ne!(permanent(&game, cards::POND_PROPHET).card.id, old);
    assert_eq!(
        game.players[0].hand.len(),
        before + 2,
        "the Portal and the returned Prophet both draw"
    );
}

#[test]
fn cache_grab_makes_food_for_the_squirrel_returned_from_the_mill() {
    let mut game = board(&[
        cards::CORPSEBERRY_CULTIVATOR,
        cards::FELL,
        cards::DIRESIGHT,
        cards::SAVOR,
    ]);
    cast(&mut game, cards::CACHE_GRAB);
    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|c| c.definition == cards::CORPSEBERRY_CULTIVATOR)
    );
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| game.effective_subtypes(p).contains(&"Food"))
            .count(),
        1
    );
    assert_eq!(game.players[0].graveyard.len(), 4);
}

#[test]
fn clifftop_lookout_reveals_through_the_land_and_returns_other_cards() {
    let mut game = board(&[cards::FOREST, cards::GRIZZLY_BEARS, cards::FELL]);
    game.put_onto_battlefield(PlayerId::One, cards::CLIFFTOP_LOOKOUT)
        .unwrap();
    settle(&mut game);
    assert!(permanent(&game, cards::FOREST).tapped);
    assert_eq!(game.players[0].library.len(), 2);
    assert!(game.players[0].graveyard.is_empty());
}

#[test]
fn stargaze_selects_exactly_x_cards_and_moves_the_rest() {
    let mut game = board(&[cards::ISLAND; 6]);
    let id = held(&mut game, cards::STARGAZE);
    let life = game.players[0].life;
    game.apply(PlayerId::One, super::cast_action(id, vec![], vec![], 2))
        .unwrap();
    settle(&mut game);
    assert_eq!(game.players[0].hand.len(), 2);
    assert_eq!(game.players[0].library.len(), 2);
    assert_eq!(game.players[0].graveyard.len(), 3);
    assert_eq!(game.players[0].life, life - 2);
}

#[test]
fn cruelclaw_offers_a_discard_cast_without_any_mana() {
    let mut game = board(&[cards::GRIZZLY_BEARS, cards::FOREST, cards::ISLAND]);
    game.players[0].hand = game.build_zone(PlayerId::One, &[cards::SWAMP]).unwrap();
    let source = game
        .put_onto_battlefield(PlayerId::One, cards::THE_INFAMOUS_CRUELCLAW)
        .unwrap();
    game.deal_combat_damage_to_player(source, PlayerId::Two, 3);
    for _ in 0..12 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        game.apply(game.priority, Action::PassPriority).unwrap();
    }
    let spell = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a, Action::CastSpell { .. }))
        .expect("discard is the alternative payment");
    game.apply(PlayerId::One, spell).unwrap();
    settle(&mut game);
    assert!(
        game.battlefield
            .iter()
            .any(|p| p.card.definition == cards::GRIZZLY_BEARS)
    );
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|c| c.definition == cards::SWAMP)
    );
    assert_eq!(game.players[0].exile.len(), 2);
}

#[test]
fn pearl_of_wisdom_discounts_its_own_cast_from_hand() {
    let mut game = board(&[cards::ISLAND; 4]);
    game.put_onto_battlefield(PlayerId::One, cards::CORUSCATION_MAGE)
        .unwrap();
    settle(&mut game);
    game.players[0].hand = game
        .build_zone(PlayerId::One, &[cards::PEARL_OF_WISDOM])
        .unwrap();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 2);
    let id = game.players[0].hand[0].id;
    let action = cast_action(&game, id, PlayOptionId::DEFAULT);
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    assert_eq!(game.players[0].hand.len(), 2);
}

#[test]
fn portent_requires_four_exiled_cards_instead_of_four_card_types() {
    for (library, expected_offer) in [
        (vec![cards::ORNITHOPTER, cards::FOREST, cards::FELL], false),
        (
            vec![
                cards::ORNITHOPTER,
                cards::FOREST,
                cards::FELL,
                cards::FEATHER_OF_FLIGHT,
            ],
            true,
        ),
    ] {
        let mut game = board(&library);
        let id = held(&mut game, cards::PORTENT_OF_CALAMITY);
        game.apply(
            PlayerId::One,
            super::cast_action(id, vec![], vec![], u16::try_from(library.len()).unwrap()),
        )
        .unwrap();
        let mut offered = false;
        for _ in 0..64 {
            if let Some(pending) = game.pending_decisions.first() {
                offered |= matches!(
                    pending.continuation,
                    DecisionContinuation::MayCastGranted { .. }
                );
                let d = pending.observation.clone();
                let options = d
                    .options
                    .iter()
                    .take(d.minimum.max(1).min(d.maximum))
                    .map(|o| o.id)
                    .collect();
                game.apply(
                    d.player,
                    Action::ChooseDecision {
                        decision: d.id,
                        options,
                    },
                )
                .unwrap();
            } else if game.stack.is_empty() && game.pending_triggers.is_empty() {
                break;
            } else {
                game.apply(game.priority, Action::PassPriority).unwrap();
            }
        }
        assert_eq!(offered, expected_offer);
        assert_eq!(
            game.players[0].hand.len(),
            library.len(),
            "uncast selected cards go to hand"
        );
        assert!(game.players[0].exile.is_empty());
    }
}
