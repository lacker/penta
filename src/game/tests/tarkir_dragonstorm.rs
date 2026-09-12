//! Regression coverage for Tarkir Dragonstorm combinations of existing game operations.

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
fn renew_exiles_its_source_and_leaves_keyword_counters() {
    let mut game = board(&[]);
    let bear = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    let card = game
        .build_zone(PlayerId::One, &[cards::QARSI_REVENANT])
        .unwrap();
    let source = card[0].id;
    game.players[0].graveyard.extend(card);
    for color in ManaColor::COLORS {
        game.add_unrestricted_mana(PlayerId::One, color, 4);
    }
    activate(&mut game, source);
    assert!(game.players[0].graveyard.is_empty());
    let p = game.battlefield.iter().find(|p| p.card.id == bear).unwrap();
    for counter in [
        CounterKind::Flying,
        CounterKind::Deathtouch,
        CounterKind::Lifelink,
    ] {
        assert_eq!(p.counters(counter), 1);
    }
}

#[test]
fn flurry_keeps_its_trigger_after_a_third_spell_is_cast() {
    for source in [cards::POISED_PRACTITIONER, cards::CORI_STEEL_CUTTER] {
        let mut game = board(&[cards::FOREST]);
        game.put_onto_battlefield(PlayerId::One, source).unwrap();
        game.spells_cast_this_turn[0] = 1;
        let id = held(&mut game, cards::OSCORP_RESEARCH_TEAM);
        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|a| matches!(a, Action::CastSpell {card,..} if *card==id))
            .unwrap();
        game.apply(PlayerId::One, action).unwrap();
        assert!(!game.pending_triggers.is_empty() || game.stack.len() > 1);
        game.spells_cast_this_turn[0] = 3;
        settle(&mut game);
        if source == cards::POISED_PRACTITIONER {
            assert_eq!(
                permanent(&game, source).counters(CounterKind::PlusOnePlusOne),
                1
            );
        } else {
            assert_eq!(
                game.battlefield
                    .iter()
                    .filter(|p| p.card.definition.is_token())
                    .count(),
                1
            );
        }
    }
}

#[test]
fn ainok_wayfarer_grows_when_the_mill_contains_no_lands() {
    let mut game = board(&[cards::GRIZZLY_BEARS; 3]);
    game.put_onto_battlefield(PlayerId::One, cards::AINOK_WAYFARER)
        .unwrap();
    settle(&mut game);
    assert_eq!(
        permanent(&game, cards::AINOK_WAYFARER).counters(CounterKind::PlusOnePlusOne),
        1
    );
    assert_eq!(game.players[0].graveyard.len(), 3);
}

#[test]
fn mobilize_tokens_are_sacrificed_even_after_the_attacker_leaves() {
    let mut game = board(&[]);
    let source = game
        .put_onto_battlefield(PlayerId::One, cards::DALKOVAN_PACKBEASTS)
        .unwrap();
    game.battlefield[0].entered_controller_turn = 0;
    game.step = Step::DeclareAttackers;
    game.declare_attacker(source, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    settle(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| p.card.definition.is_token())
            .count(),
        3
    );
    game.destroy_permanents(&[source], false);
    game.step = Step::PostcombatMain;
    game.advance_step();
    settle(&mut game);
    assert!(game.battlefield.is_empty());
}

#[test]
fn omen_casts_shuffle_the_card_and_can_be_drawn_again() {
    let mut game = board(&[cards::FOREST; 4]);
    let id = held(&mut game, cards::DIRGUR_ISLAND_DRAGON);
    let action = game.legal_actions(PlayerId::One).into_iter().find(|a|matches!(a,Action::CastSpell {card,choices,..} if *card==id && choices.play_option()==PlayOptionId(1))).unwrap();
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    assert_eq!(game.players[0].hand.len(), 1);
    assert!(game.players[0].graveyard.is_empty());
    assert_eq!(
        game.players[0]
            .library
            .iter()
            .filter(|c| c.definition == cards::DIRGUR_ISLAND_DRAGON)
            .count(),
        1
    );
}

#[test]
fn flush_out_requires_a_discard_and_draws_only_when_one_is_discarded() {
    for have_card in [false, true] {
        let mut game = board(&[cards::FOREST; 4]);
        if have_card {
            held(&mut game, cards::GRIZZLY_BEARS);
        }
        let id = held(&mut game, cards::STORMSHRIEK_FERAL);
        let action = game.legal_actions(PlayerId::One).into_iter().find(|a|matches!(a,Action::CastSpell {card,choices,..} if *card==id && choices.play_option()==PlayOptionId(1))).unwrap();
        game.apply(PlayerId::One, action).unwrap();
        settle(&mut game);
        assert_eq!(game.players[0].hand.len(), if have_card { 2 } else { 0 });
        assert_eq!(game.players[0].graveyard.len(), usize::from(have_card));
    }
}

#[test]
fn perennation_adds_both_counters_as_the_creature_enters() {
    let mut game = board(&[]);
    game.players[0].graveyard = game
        .build_zone(PlayerId::One, &[cards::GRIZZLY_BEARS])
        .unwrap();
    cast(&mut game, cards::PERENNATION);
    let p = permanent(&game, cards::GRIZZLY_BEARS);
    assert_eq!(p.counters(CounterKind::Hexproof), 1);
    assert_eq!(p.counters(CounterKind::Indestructible), 1);
}

#[test]
fn sunpearl_kirin_draws_for_the_returned_token() {
    let mut game = board(&[cards::FOREST]);
    game.put_onto_battlefield(PlayerId::One, cards::COMMON_CROOK)
        .unwrap();
    game.destroy_permanents(&[game.battlefield[0].card.id], false);
    settle(&mut game);
    game.put_onto_battlefield(PlayerId::One, cards::SUNPEARL_KIRIN)
        .unwrap();
    settle(&mut game);
    assert_eq!(game.players[0].hand.len(), 1);
    assert!(
        !game
            .battlefield
            .iter()
            .any(|p| p.card.definition.is_token())
    );
}

#[test]
fn traveling_botanist_puts_only_a_land_in_hand() {
    for land in [true, false] {
        let mut game = board(&[if land {
            cards::FOREST
        } else {
            cards::GRIZZLY_BEARS
        }]);
        let source = game
            .put_onto_battlefield(PlayerId::One, cards::TRAVELING_BOTANIST)
            .unwrap();
        game.battlefield[0].entered_controller_turn = 0;
        game.step = Step::DeclareAttackers;
        game.declare_attacker(source, AttackDefender::Player(PlayerId::Two));
        game.finish_declaring_attackers();
        settle(&mut game);
        assert_eq!(game.players[0].hand.len(), usize::from(land));
        assert_eq!(game.players[0].graveyard.len(), usize::from(!land));
    }
}

#[test]
fn magmatic_hellkite_replaces_an_opponents_land_with_a_tapped_stunned_basic() {
    let mut game = board(&[]);
    game.players[1].library = game.build_zone(PlayerId::Two, &[cards::FOREST]).unwrap();
    game.put_onto_battlefield(PlayerId::Two, cards::FRONTIER_BIVOUAC)
        .unwrap();
    game.put_onto_battlefield(PlayerId::One, cards::MAGMATIC_HELLKITE)
        .unwrap();
    settle(&mut game);
    let forest = permanent(&game, cards::FOREST);
    assert_eq!(forest.controller, PlayerId::Two);
    assert!(forest.tapped);
    assert_eq!(forest.counters(CounterKind::Stun), 1);
}

#[test]
fn bloomvine_omen_splits_two_forests_between_hand_and_battlefield() {
    let mut game = board(&[cards::FOREST; 2]);
    let id = held(&mut game, cards::BLOOMVINE_REGENT);
    let action=game.legal_actions(PlayerId::One).into_iter().find(|a|matches!(a,Action::CastSpell {card,choices,..}if *card==id&&choices.play_option()==PlayOptionId(1))).unwrap();
    game.apply(PlayerId::One, action).unwrap();
    // The search offers up to two; select both before choosing which one enters.
    for _ in 0..8 {
        if game
            .pending_decisions
            .first()
            .is_some_and(|p| p.observation.maximum == 2)
        {
            break;
        }
        game.apply(game.priority, Action::PassPriority).unwrap();
    }
    let decision = game.pending_decisions[0].observation.clone();
    game.apply(
        decision.player,
        Action::ChooseDecision {
            decision: decision.id,
            options: decision.options.iter().take(2).map(|o| o.id).collect(),
        },
    )
    .unwrap();
    settle(&mut game);
    assert_eq!(game.players[0].hand.len(), 1);
    assert_eq!(game.battlefield.len(), 1);
    assert!(game.battlefield[0].tapped);
    assert_eq!(game.players[0].library.len(), 1);
}

#[test]
fn sarkhan_can_behold_a_dragon_already_on_the_battlefield() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::BOULDERBORN_DRAGON)
        .unwrap();
    game.put_onto_battlefield(PlayerId::One, cards::SARKHAN_DRAGON_ASCENDANT)
        .unwrap();
    settle(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| p.card.definition.is_token())
            .count(),
        1
    );
    assert_eq!(
        permanent(&game, cards::SARKHAN_DRAGON_ASCENDANT).counters(CounterKind::PlusOnePlusOne),
        0
    );
    game.put_onto_battlefield(PlayerId::One, cards::BOULDERBORN_DRAGON)
        .unwrap();
    settle(&mut game);
    assert_eq!(
        permanent(&game, cards::SARKHAN_DRAGON_ASCENDANT).counters(CounterKind::PlusOnePlusOne),
        1
    );
}

#[test]
fn endure_creates_the_spirit_when_its_source_has_left() {
    let mut game = board(&[]);
    let source = game
        .put_onto_battlefield(PlayerId::One, cards::DUSYUT_EARTHCARVER)
        .unwrap();
    game.destroy_permanents(&[source], false);
    settle(&mut game);
    assert_eq!(game.battlefield.len(), 1);
    assert!(game.battlefield[0].card.definition.is_token());
    assert_eq!(game.power(&game.battlefield[0]), Some(3));
}

#[test]
fn endure_uses_the_creatures_current_controller() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::DUSYUT_EARTHCARVER)
        .unwrap();
    game.battlefield[0].controller = PlayerId::Two;
    for _ in 0..8 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        game.apply(game.priority, Action::PassPriority).unwrap();
    }
    let decision = game.pending_decisions.first().unwrap().observation.clone();
    assert_eq!(decision.player, PlayerId::Two);
}

#[test]
fn endure_uses_last_known_control_when_its_creature_phases_out() {
    let mut game = board(&[]);
    let source = game
        .put_onto_battlefield(PlayerId::One, cards::DUSYUT_EARTHCARVER)
        .unwrap();
    game.battlefield[0].controller = PlayerId::Two;
    game.phase_out(source);
    settle(&mut game);
    assert_eq!(game.battlefield.len(), 1);
    assert!(game.battlefield[0].card.definition.is_token());
    assert_eq!(game.battlefield[0].controller, PlayerId::Two);
    assert_eq!(game.phased_out.len(), 1);
}
