//! Regression coverage for Foundations's combinations of existing game operations.

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

fn activate(game: &mut Game, id: GameObjectId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a, Action::ActivateAbility {source,..} | Action::ActivateManaAbility {source,..} if *source == id))
        .expect("ability is available");
    game.apply(PlayerId::One, action).unwrap();
    settle(game);
}

#[test]
fn giada_counts_angels_already_present_when_an_angel_enters() {
    let mut game = board(&[]);
    let giada = game
        .put_onto_battlefield(PlayerId::One, cards::GIADA_FONT_OF_HOPE)
        .unwrap();
    let first = game
        .put_onto_battlefield(PlayerId::One, cards::SERRA_ANGEL)
        .unwrap();
    let second = game
        .put_onto_battlefield(PlayerId::One, cards::SERRA_ANGEL)
        .unwrap();
    for (id, expected) in [(giada, 0), (first, 1), (second, 2)] {
        assert_eq!(
            game.battlefield
                .iter()
                .find(|p| p.card.id == id)
                .unwrap()
                .counters(CounterKind::PlusOnePlusOne),
            expected
        );
    }
}

#[test]
fn carnelian_orb_grants_haste_only_to_a_dragon_paid_with_its_mana() {
    for definition in [cards::DRAGON_WHELP, cards::HILL_GIANT] {
        let mut game = board(&[]);
        let orb = game
            .put_onto_battlefield(PlayerId::One, cards::CARNELIAN_ORB_OF_DRAGONKIND)
            .unwrap();
        activate(&mut game, orb);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 3);
        game.players[0].hand = game.build_zone(PlayerId::One, &[definition]).unwrap();
        let id = game.players[0].hand[0].id;
        let action = cast_action(&game, id, PlayOptionId::DEFAULT);
        game.apply(PlayerId::One, action).unwrap();
        settle(&mut game);
        let creature = permanent(&game, definition).card.id;
        game.step = Step::DeclareAttackers;
        game.attackers_declared = false;
        assert_eq!(
            game.legal_actions(PlayerId::One)
                .iter()
                .any(|a| matches!(a,Action::DeclareAttacker{attacker,..} if *attacker==creature)),
            definition == cards::DRAGON_WHELP
        );
    }
}

#[test]
fn kicked_replication_creates_five_copies() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    let id = held(&mut game, cards::RITE_OF_REPLICATION);
    let action=game.legal_actions(PlayerId::One).into_iter().find(|a|matches!(a,Action::CastSpell{card,choices,..} if *card==id && !choices.costs().additional().is_empty())).unwrap();
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| p.card.definition.is_token())
            .count(),
        5
    );
}

#[test]
fn electroduplicate_creates_a_hasty_copy_that_dies_at_the_next_end_step() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    cast(&mut game, cards::ELECTRODUPLICATE);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| p.card.definition.is_token())
            .count(),
        1
    );
    game.players[0].mana_pool = ManaPool::default();
    game.step = Step::PostcombatMain;
    game.advance_step();
    settle(&mut game);
    assert_eq!(game.battlefield.len(), 1);
}

#[test]
fn cemetery_recruitment_draws_for_the_zombie_that_was_returned() {
    for definition in [cards::SCATHE_ZOMBIES, cards::GRIZZLY_BEARS] {
        let mut game = board(&[cards::ISLAND; 3]);
        game.players[0].graveyard = game.build_zone(PlayerId::One, &[definition]).unwrap();
        cast(&mut game, cards::CEMETERY_RECRUITMENT);
        assert!(
            game.players[0]
                .hand
                .iter()
                .any(|c| c.definition == definition)
        );
        assert_eq!(
            game.players[0].library.len(),
            if definition == cards::SCATHE_ZOMBIES {
                2
            } else {
                3
            }
        );
    }
}

#[test]
fn fake_your_own_death_returns_a_new_tapped_object_and_creates_treasure() {
    let mut game = board(&[]);
    let bear = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    cast(&mut game, cards::FAKE_YOUR_OWN_DEATH);
    game.destroy_permanent(bear);
    settle(&mut game);
    let returned = permanent(&game, cards::GRIZZLY_BEARS);
    assert_ne!(returned.card.id, bear);
    assert!(returned.tapped);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| p.card.definition.is_token())
            .count(),
        1
    );
}

#[test]
fn ayli_gains_the_sacrificed_creatures_last_known_toughness() {
    let mut game = board(&[]);
    let ayli = game
        .put_onto_battlefield(PlayerId::One, cards::AYLI_ETERNAL_PILGRIM)
        .unwrap();
    let bear = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == bear)
        .unwrap()
        .add_counters(CounterKind::PlusOnePlusOne, 3);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    let before = game.players[0].life;
    activate(&mut game, ayli);
    assert_eq!(game.players[0].life, before + 5);
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|c| c.definition == cards::GRIZZLY_BEARS)
    );
}

#[test]
fn burning_hart_searches_for_two_tapped_basic_lands() {
    let mut game = board(&[cards::FOREST, cards::ISLAND]);
    let hart = game
        .put_onto_battlefield(PlayerId::One, cards::BURNISHED_HART)
        .unwrap();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a,Action::ActivateAbility{source,..} if *source==hart))
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    while game.pending_decisions.is_empty() {
        game.apply(game.priority, Action::PassPriority).unwrap();
    }
    let decision = game.pending_decisions[0].observation.clone();
    assert_eq!(decision.maximum, 2);
    game.apply(
        decision.player,
        Action::ChooseDecision {
            decision: decision.id,
            options: decision.options.iter().map(|o| o.id).collect(),
        },
    )
    .unwrap();
    settle(&mut game);
    assert_eq!(game.battlefield.len(), 2);
    assert!(game.battlefield.iter().all(|p| p.tapped));
}

#[test]
fn garna_remembers_whether_a_dead_creature_was_attacking() {
    for attacking in [false, true] {
        let mut game = board(&[cards::ISLAND; 3]);
        game.put_onto_battlefield(PlayerId::One, cards::GARNA_BLOODFIST_OF_KELD)
            .unwrap();
        let bear = game
            .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
            .unwrap();
        game.battlefield
            .iter_mut()
            .find(|p| p.card.id == bear)
            .unwrap()
            .attacking = attacking;
        let before = game.players[1].life;
        game.destroy_permanent(bear);
        settle(&mut game);
        assert_eq!(game.players[0].hand.len(), usize::from(attacking));
        assert_eq!(game.players[1].life, before - if attacking { 0 } else { 1 });
    }
}

#[test]
fn foundations_counter_names_round_trip() {
    for name in ["soul", "incubation"] {
        let kind = CounterKind::named(name);
        assert_eq!(CounterKind::from_name(name), Some(kind));
        assert_eq!(kind.name(), name);
    }
}

#[test]
fn dread_summons_counts_creatures_milled_from_both_libraries() {
    let mut game = board(&[cards::GRIZZLY_BEARS]);
    game.players[1].library = game
        .build_zone(PlayerId::Two, &[cards::SCATHE_ZOMBIES])
        .unwrap();
    let id = held(&mut game, cards::DREAD_SUMMONS);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a,Action::CastSpell{card,choices,..} if *card==id && choices.x()==1))
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    assert_eq!(
        game.players[0]
            .graveyard
            .iter()
            .filter(|c| c.definition == cards::GRIZZLY_BEARS)
            .count(),
        1
    );
    assert_eq!(game.players[1].graveyard.len(), 1);
    assert_eq!(game.battlefield.len(), 2);
    assert!(
        game.battlefield
            .iter()
            .all(|p| p.tapped && p.controller == PlayerId::One)
    );
}

#[test]
fn demolition_field_gives_the_destroyed_lands_controller_the_first_search_choice() {
    let mut game = board(&[cards::FOREST]);
    game.players[1].library = game.build_zone(PlayerId::Two, &[cards::ISLAND]).unwrap();
    let field = game
        .put_onto_battlefield(PlayerId::One, cards::DEMOLITION_FIELD)
        .unwrap();
    game.put_onto_battlefield(PlayerId::Two, cards::CITY_OF_BRASS)
        .unwrap();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a,Action::ActivateAbility{source,..} if *source==field))
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    for _ in 0..8 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        game.apply(game.priority, Action::PassPriority).unwrap();
    }
    assert_eq!(game.pending_decisions[0].observation.player, PlayerId::Two);
    settle(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| p.controller == PlayerId::One)
            .count(),
        1
    );
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| p.controller == PlayerId::Two)
            .count(),
        1
    );
}

#[test]
fn hoarding_dragon_returns_only_its_linked_card() {
    let mut game = board(&[cards::SOL_RING]);
    let dragon = game
        .put_onto_battlefield(PlayerId::One, cards::HOARDING_DRAGON)
        .unwrap();
    settle(&mut game);
    assert_eq!(game.players[0].exile.len(), 1);
    game.destroy_permanent(dragon);
    settle(&mut game);
    assert!(game.players[0].exile.is_empty());
    assert_eq!(game.players[0].hand[0].definition, cards::SOL_RING);
}

#[test]
fn venom_connoisseur_counts_resolutions_and_grants_the_second_entry_board_deathtouch() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::VENOM_CONNOISSEUR)
        .unwrap();
    let first = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    settle(&mut game);
    assert!(
        !game.permanent_has_executable_keyword(
            game.battlefield
                .iter()
                .find(|p| p.card.id == first)
                .unwrap(),
            KeywordAbility::Deathtouch
        )
    );
    game.put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    settle(&mut game);
    assert!(
        game.battlefield
            .iter()
            .all(|p| game.permanent_has_executable_keyword(p, KeywordAbility::Deathtouch))
    );
}

#[test]
fn bloodtithe_collector_reads_life_loss_before_it_entered() {
    for lost_life in [false, true] {
        let mut game = board(&[]);
        game.players[1].hand = game.build_zone(PlayerId::Two, &[cards::ISLAND]).unwrap();
        if lost_life {
            game.lose_life(PlayerId::Two, 1);
        }
        game.put_onto_battlefield(PlayerId::One, cards::BLOODTITHE_COLLECTOR)
            .unwrap();
        settle(&mut game);
        assert_eq!(game.players[1].hand.len(), usize::from(!lost_life));
    }
}
