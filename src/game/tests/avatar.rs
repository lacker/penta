//! Regression coverage for Avatar's combinations of existing game operations.

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

fn cast_at(game: &mut Game, definition: CardDefinitionId, target: Target) {
    let id = held(game, definition);
    let action = game.legal_actions(PlayerId::One).into_iter().find(|a| matches!(a,
        Action::CastSpell {card, choices,..} if *card == id && choices.iter_targets().any(|t| *t == target))).unwrap();
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
fn shrine_entry_counts_all_shrines_and_other_entries_trigger_once() {
    let mut game = board(&[cards::PLAINS; 8]);
    game.put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    game.put_onto_battlefield(PlayerId::One, cards::SOUTHERN_AIR_TEMPLE)
        .unwrap();
    settle(&mut game);
    assert_eq!(
        permanent(&game, cards::GRIZZLY_BEARS)
            .counters
            .count(CounterKind::PlusOnePlusOne),
        1
    );
    game.put_onto_battlefield(PlayerId::One, cards::THE_SPIRIT_OASIS)
        .unwrap();
    settle(&mut game);
    assert_eq!(game.players[0].hand.len(), 2);
    assert_eq!(
        permanent(&game, cards::GRIZZLY_BEARS)
            .counters
            .count(CounterKind::PlusOnePlusOne),
        2
    );
}
#[test]
fn path_to_redemption_retains_its_host_after_sacrificing_the_aura() {
    let mut game = board(&[]);
    let bear = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .unwrap();
    cast_at(
        &mut game,
        cards::PATH_TO_REDEMPTION,
        Target::Permanent(bear),
    );
    let aura = permanent(&game, cards::PATH_TO_REDEMPTION).card.id;
    activate(&mut game, aura);
    assert!(
        game.players[1]
            .exile
            .iter()
            .any(|c| c.definition == cards::GRIZZLY_BEARS)
    );
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|c| c.definition == cards::PATH_TO_REDEMPTION)
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
fn avatar_destiny_returns_its_aura_and_a_creature_actually_milled() {
    let mut game = board(&[cards::PLAINS, cards::GRIZZLY_BEARS, cards::ISLAND]);
    let bear = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    cast_at(&mut game, cards::AVATAR_DESTINY, Target::Permanent(bear));
    game.destroy_permanents(&[bear], false);
    settle(&mut game);
    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|c| c.definition == cards::AVATAR_DESTINY)
    );
    assert!(
        game.battlefield
            .iter()
            .any(|p| p.card.definition == cards::GRIZZLY_BEARS && p.card.id != bear)
    );
    assert_eq!(game.players[0].library.len(), 1);
}
#[test]
fn draw_thresholds_update_live_and_reset_with_the_turn() {
    let mut game = board(&[cards::PLAINS; 5]);
    game.put_onto_battlefield(PlayerId::One, cards::FOGGY_SWAMP_HUNTERS)
        .unwrap();
    game.put_onto_battlefield(PlayerId::One, cards::JUNE_BOUNTY_HUNTER)
        .unwrap();
    game.put_onto_battlefield(PlayerId::One, cards::MESSENGER_HAWK)
        .unwrap();
    settle(&mut game);
    assert!(!game.permanent_has_executable_keyword(
        permanent(&game, cards::FOGGY_SWAMP_HUNTERS),
        KeywordAbility::Lifelink
    ));
    assert!(!game.has_applied_rule(
        permanent(&game, cards::JUNE_BOUNTY_HUNTER),
        crate::card::AppliedRuleDef::CANNOT_BE_BLOCKED
    ));
    game.draw_cards(PlayerId::One, 2);
    settle(&mut game);
    assert!(game.permanent_has_executable_keyword(
        permanent(&game, cards::FOGGY_SWAMP_HUNTERS),
        KeywordAbility::Lifelink
    ));
    assert!(game.permanent_has_executable_keyword(
        permanent(&game, cards::FOGGY_SWAMP_HUNTERS),
        KeywordAbility::Menace
    ));
    assert!(game.has_applied_rule(
        permanent(&game, cards::JUNE_BOUNTY_HUNTER),
        crate::card::AppliedRuleDef::CANNOT_BE_BLOCKED
    ));
    assert_eq!(
        game.creature_stats(permanent(&game, cards::MESSENGER_HAWK))
            .unwrap()
            .power,
        3
    );
    game.commit_next_turn(PlayerId::Two, Vec::new());
    assert!(!game.permanent_has_executable_keyword(
        permanent(&game, cards::FOGGY_SWAMP_HUNTERS),
        KeywordAbility::Lifelink
    ));
    assert_eq!(
        game.creature_stats(permanent(&game, cards::MESSENGER_HAWK))
            .unwrap()
            .power,
        1
    );
}
#[test]
fn accumulate_wisdom_moves_cards_without_drawing() {
    let mut game = board(&[cards::PLAINS, cards::ISLAND, cards::SWAMP]);
    game.players[0].graveyard = game
        .build_zone(
            PlayerId::One,
            &[
                cards::SHARED_ROOTS,
                cards::OZAI_S_CRUELTY,
                cards::OCTOPUS_FORM,
            ],
        )
        .unwrap();
    cast(&mut game, cards::ACCUMULATE_WISDOM);
    assert_eq!(game.players[0].hand.len(), 3);
    assert_eq!(game.cards_drawn_this_turn[0], 0);
    assert!(game.players[0].library.is_empty());
}
#[test]
fn ostrich_horse_gets_a_counter_when_the_mill_contains_no_land() {
    let mut game = board(&[cards::GRIZZLY_BEARS; 3]);
    game.put_onto_battlefield(PlayerId::One, cards::OSTRICH_HORSE)
        .unwrap();
    settle(&mut game);
    assert_eq!(
        permanent(&game, cards::OSTRICH_HORSE)
            .counters
            .count(CounterKind::PlusOnePlusOne),
        1
    );
    assert_eq!(game.players[0].graveyard.len(), 3);
}
#[test]
fn sparring_dummy_gains_life_for_a_milled_lesson() {
    let mut game = board(&[cards::SHARED_ROOTS]);
    let dummy = game
        .put_onto_battlefield(PlayerId::One, cards::SPARRING_DUMMY)
        .unwrap();
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == dummy)
        .unwrap()
        .entered_controller_turn = 0;
    let life = game.players[0].life;
    activate(&mut game, dummy);
    assert_eq!(game.players[0].life, life + 2);
    assert_eq!(game.players[0].graveyard.len(), 1);
}
#[test]
fn war_balloon_stays_a_creature_with_three_fire_counters() {
    let mut game = board(&[]);
    let balloon = game
        .put_onto_battlefield(PlayerId::One, cards::WAR_BALLOON)
        .unwrap();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 3);
    for _ in 0..3 {
        activate(&mut game, balloon);
    }
    assert_eq!(
        game.creature_stats(permanent(&game, cards::WAR_BALLOON))
            .unwrap()
            .power,
        4
    );
    game.commit_next_turn(PlayerId::Two, Vec::new());
    assert_eq!(
        game.creature_stats(permanent(&game, cards::WAR_BALLOON))
            .unwrap()
            .power,
        4
    );
}
#[test]
fn avatar_lands_check_for_a_basic_land_before_entry() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::ABANDONED_AIR_TEMPLE)
        .unwrap();
    assert!(permanent(&game, cards::ABANDONED_AIR_TEMPLE).tapped);
    game.put_onto_battlefield(PlayerId::One, cards::PLAINS)
        .unwrap();
    game.put_onto_battlefield(PlayerId::One, cards::AGNA_QEL_A)
        .unwrap();
    assert!(!permanent(&game, cards::AGNA_QEL_A).tapped);
}
#[test]
fn zhao_sets_nonbasic_land_types_only_after_its_counter() {
    let mut game = board(&[]);
    let zhao = game
        .put_onto_battlefield(PlayerId::One, cards::ZHAO_THE_MOON_SLAYER)
        .unwrap();
    game.put_onto_battlefield(PlayerId::Two, cards::TROPICAL_ISLAND)
        .unwrap();
    assert!(permanent(&game, cards::TROPICAL_ISLAND).tapped);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 7);
    activate(&mut game, zhao);
    let land = permanent(&game, cards::TROPICAL_ISLAND);
    assert!(game.effective_subtypes(land).contains(&"Mountain"));
    assert!(!game.effective_subtypes(land).contains(&"Island"));
}
#[test]
fn cabbage_merchant_returns_even_when_its_search_finds_nothing() {
    let mut game = board(&[cards::GRIZZLY_BEARS]);
    game.put_onto_battlefield(PlayerId::One, cards::UNLUCKY_CABBAGE_MERCHANT)
        .unwrap();
    settle(&mut game);
    let food = game
        .battlefield
        .iter()
        .find(|p| p.card.definition.is_token())
        .unwrap()
        .card
        .id;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 2);
    activate(&mut game, food);
    assert!(
        !game
            .battlefield
            .iter()
            .any(|p| p.card.definition == cards::UNLUCKY_CABBAGE_MERCHANT)
    );
    assert!(
        game.players[0]
            .library
            .iter()
            .any(|c| c.definition == cards::UNLUCKY_CABBAGE_MERCHANT)
    );
}
#[test]
fn allies_at_last_deals_both_creatures_damage() {
    let mut game = board(&[]);
    let first = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    let second = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    let enemy = game
        .put_onto_battlefield(PlayerId::Two, cards::SHIVAN_DRAGON)
        .unwrap();
    let id = held(&mut game, cards::ALLIES_AT_LAST);
    let action=game.legal_actions(PlayerId::One).into_iter().find(|a|matches!(a,Action::CastSpell{card,choices,..} if *card==id && [first,second,enemy].iter().all(|id|choices.iter_targets().any(|t|*t==Target::Permanent(*id))))).unwrap();
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    assert_eq!(permanent(&game, cards::SHIVAN_DRAGON).damage, 4);
}
#[test]
fn trebuchets_attacking_token_is_sacrificed_at_end_step() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::FIRE_NAVY_TREBUCHET)
        .unwrap();
    let bear = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == bear)
        .unwrap()
        .entered_controller_turn = 0;
    game.step = Step::DeclareAttackers;
    game.declare_attacker(bear, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    settle(&mut game);
    let token = game
        .battlefield
        .iter()
        .find(|p| p.card.definition.is_token())
        .unwrap();
    assert!(token.tapped && token.attacking);
    assert_eq!(game.creature_stats(token).unwrap().power, 2);
    game.step = Step::PostcombatMain;
    game.advance_step();
    settle(&mut game);
    assert!(
        !game
            .battlefield
            .iter()
            .any(|p| p.card.definition.is_token())
    );
}

#[test]
fn gran_gran_discount_tracks_the_current_lesson_count() {
    let mut game = board(&[cards::PLAINS, cards::ISLAND]);
    game.put_onto_battlefield(PlayerId::One, cards::GRAN_GRAN)
        .unwrap();
    game.players[0].graveyard = game
        .build_zone(PlayerId::One, &[cards::SHARED_ROOTS, cards::OZAI_S_CRUELTY])
        .unwrap();
    let hand = game
        .build_zone(PlayerId::One, &[cards::DIVINATION])
        .unwrap();
    let id = hand[0].id;
    game.players[0].hand = hand;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|a| matches!(a, Action::CastSpell {card, ..} if *card == id))
    );
    let lesson = game
        .build_zone(PlayerId::One, &[cards::OCTOPUS_FORM])
        .unwrap();
    game.players[0].graveyard.extend(lesson);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a, Action::CastSpell {card, ..} if *card == id))
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    assert_eq!(game.players[0].hand.len(), 2);
    assert_eq!(game.players[0].mana_pool.total(), 0);
}
