//! Regression coverage for Thunder Junction's combinations of existing game operations.

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

#[test]
fn plot_waits_for_a_later_turn_and_casts_without_mana() {
    let mut game = board(&[]);
    let id = held(&mut game, cards::DJINN_OF_FOOL_S_FALL);
    game.apply(PlayerId::One, Action::Plot { card: id })
        .unwrap();
    let exiled = game.players[0].exile[0].id;
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|a| matches!(a, Action::CastSpell {card,..} if *card == exiled))
    );
    for p in &mut game.players {
        p.mana_pool = ManaPool::default();
    }
    game.commit_next_turn(PlayerId::Two, Vec::new());
    game.commit_next_turn(PlayerId::One, Vec::new());
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a,Action::CastSpell {card,..} if *card == exiled))
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    assert!(game.has_flying(permanent(&game, cards::DJINN_OF_FOOL_S_FALL)));
}

#[test]
fn saddle_taps_a_contributor_and_enables_the_attack_trigger() {
    let mut game = board(&[]);
    let mount = game
        .put_onto_battlefield(PlayerId::One, cards::BRIDLED_BIGHORN)
        .unwrap();
    let bear = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    activate(&mut game, mount);
    assert!(permanent(&game, cards::BRIDLED_BIGHORN).saddled);
    assert!(
        game.battlefield
            .iter()
            .find(|p| p.card.id == bear)
            .unwrap()
            .tapped
    );
    game.step = Step::DeclareAttackers;
    game.declare_attacker(mount, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    settle(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| p.card.definition.is_token())
            .count(),
        1
    );
}

#[test]
fn gisa_limits_crimes_to_one_zombie_pair_per_turn() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::GISA_THE_HELLRAISER)
        .unwrap();
    cast_at(
        &mut game,
        cards::LIGHTNING_BOLT,
        Target::Player(PlayerId::Two),
    );
    cast_at(
        &mut game,
        cards::LIGHTNING_BOLT,
        Target::Player(PlayerId::Two),
    );
    let zombies = game
        .battlefield
        .iter()
        .filter(|p| p.card.definition.is_token())
        .collect::<Vec<_>>();
    assert_eq!(zombies.len(), 2);
    for p in zombies {
        assert!(p.tapped);
        assert_eq!(game.creature_stats(p).unwrap().power, 3);
        assert!(game.permanent_has_executable_keyword(p, KeywordAbility::Menace));
    }
}

#[test]
fn forsaken_miner_listens_for_crimes_from_the_graveyard() {
    let mut game = board(&[]);
    game.players[0].graveyard = game
        .build_zone(PlayerId::One, &[cards::FORSAKEN_MINER])
        .unwrap();
    let id = held(&mut game, cards::LIGHTNING_BOLT);
    let action=game.legal_actions(PlayerId::One).into_iter().find(|a|matches!(a,Action::CastSpell{card,choices,..} if *card==id && choices.iter_targets().any(|t|*t==Target::Player(PlayerId::Two)))).unwrap();
    game.apply(PlayerId::One, action).unwrap();
    for _ in 0..20 {
        if let Some(pending) = game.pending_decisions.first() {
            let d = pending.observation.clone();
            let accept = d.options.iter().find(|o| o.label != "Decline").unwrap().id;
            game.apply(
                d.player,
                Action::ChooseDecision {
                    decision: d.id,
                    options: vec![accept],
                },
            )
            .unwrap();
            break;
        }
        game.apply(game.priority, Action::PassPriority).unwrap();
    }
    settle(&mut game);
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .all(|c| c.definition != cards::FORSAKEN_MINER)
    );
    assert!(!game.permanent_has_executable_keyword(
        permanent(&game, cards::FORSAKEN_MINER),
        KeywordAbility::Defender
    ));
}

#[test]
fn dust_animus_replacement_counts_untapped_lands_at_entry() {
    for untapped in [4, 5] {
        let mut game = board(&[]);
        for i in 0..5 {
            let id = game
                .put_onto_battlefield(PlayerId::One, cards::PLAINS)
                .unwrap();
            game.battlefield
                .iter_mut()
                .find(|p| p.card.id == id)
                .unwrap()
                .tapped = i >= untapped;
        }
        game.put_onto_battlefield(PlayerId::One, cards::DUST_ANIMUS)
            .unwrap();
        let p = permanent(&game, cards::DUST_ANIMUS);
        assert_eq!(
            p.counters.count(CounterKind::PlusOnePlusOne),
            if untapped == 5 { 2 } else { 0 }
        );
        assert_eq!(
            game.permanent_has_executable_keyword(p, KeywordAbility::Lifelink),
            untapped == 5
        );
    }
}

#[test]
fn boneyard_desecrator_remembers_the_sacrificed_outlaw() {
    for food in [cards::RECKLESS_LACKEY, cards::GRIZZLY_BEARS] {
        let mut game = board(&[]);
        let source = game
            .put_onto_battlefield(PlayerId::One, cards::BONEYARD_DESECRATOR)
            .unwrap();
        game.put_onto_battlefield(PlayerId::One, food).unwrap();
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 2);
        activate(&mut game, source);
        assert_eq!(
            permanent(&game, cards::BONEYARD_DESECRATOR)
                .counters
                .count(CounterKind::PlusOnePlusOne),
            1
        );
        assert_eq!(
            game.battlefield
                .iter()
                .filter(|p| p.card.definition.is_token())
                .count(),
            usize::from(food == cards::RECKLESS_LACKEY)
        );
    }
}

#[test]
fn goldvein_hydra_uses_its_last_power_for_tapped_treasures() {
    let mut game = board(&[]);
    let id = held(&mut game, cards::GOLDVEIN_HYDRA);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a,Action::CastSpell{card,choices,..} if *card==id && choices.x()==3))
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    let id = permanent(&game, cards::GOLDVEIN_HYDRA).card.id;
    assert_eq!(
        game.creature_stats(permanent(&game, cards::GOLDVEIN_HYDRA))
            .unwrap()
            .power,
        3
    );
    game.destroy_permanent(id);
    settle(&mut game);
    assert_eq!(game.battlefield.len(), 3);
    assert!(
        game.battlefield
            .iter()
            .all(|p| p.tapped && p.card.definition.is_token())
    );
}

#[test]
fn roxanne_adds_the_type_actually_produced_by_its_artifact_token() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::ROXANNE_STARFALL_SAVANT)
        .unwrap();
    settle(&mut game);
    let meteor = game
        .battlefield
        .iter()
        .find(|p| p.card.definition.is_token())
        .unwrap()
        .card
        .id;
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == meteor)
        .unwrap()
        .tapped = false;
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a,Action::ActivateManaAbility{source,..} if *source==meteor))
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    assert_eq!(game.players[0].mana_pool.total(), 2);
    assert!(game.stack.is_empty());
}

#[test]
fn patient_naturalist_returns_a_milled_land_or_creates_treasure() {
    for library in [
        &[cards::PLAINS, cards::GRIZZLY_BEARS][..],
        &[cards::GRIZZLY_BEARS][..],
    ] {
        let mut game = board(library);
        game.put_onto_battlefield(PlayerId::One, cards::PATIENT_NATURALIST)
            .unwrap();
        settle(&mut game);
        assert_eq!(game.players[0].hand.len(), usize::from(library.len() == 2));
        assert_eq!(
            game.battlefield
                .iter()
                .filter(|p| p.card.definition.is_token())
                .count(),
            usize::from(library.len() == 1)
        );
    }
}

#[test]
fn magebane_lizard_counts_the_casting_players_noncreature_spells() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::MAGEBANE_LIZARD)
        .unwrap();
    let before = game.players[0].life;
    cast_at(
        &mut game,
        cards::LIGHTNING_BOLT,
        Target::Player(PlayerId::Two),
    );
    cast_at(
        &mut game,
        cards::LIGHTNING_BOLT,
        Target::Player(PlayerId::Two),
    );
    assert_eq!(game.players[0].life, before - 3);
}

#[test]
fn spree_rush_of_dread_rounds_each_half_up() {
    let mut game = board(&[]);
    for _ in 0..3 {
        game.put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
            .unwrap();
    }
    game.players[1].hand = game.build_zone(PlayerId::Two, &[cards::PLAINS; 3]).unwrap();
    game.players[1].life = 19;
    let id = held(&mut game, cards::RUSH_OF_DREAD);
    let action=game.legal_actions(PlayerId::One).into_iter().find(|a|matches!(a,Action::CastSpell{card,choices,..} if *card==id&&choices.modes().len()==3)).unwrap();
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    assert_eq!(game.battlefield.len(), 1);
    assert_eq!(game.players[1].hand.len(), 1);
    assert_eq!(game.players[1].life, 9);
}

#[test]
fn loot_counter_name_round_trips() {
    assert_eq!(
        CounterKind::from_name("loot"),
        Some(CounterKind::named("loot"))
    );
    assert_eq!(CounterKind::named("loot").name(), "loot");
}

#[test]
fn claim_jumper_can_search_twice_before_its_final_shuffle() {
    let mut game = board(&[cards::PLAINS; 3]);
    for _ in 0..3 {
        game.put_onto_battlefield(PlayerId::Two, cards::PLAINS)
            .unwrap();
    }
    game.put_onto_battlefield(PlayerId::One, cards::CLAIM_JUMPER)
        .unwrap();
    for _ in 0..64 {
        if let Some(pending) = game.pending_decisions.first() {
            let d = pending.observation.clone();
            let options = if matches!(
                pending.continuation,
                DecisionContinuation::OptionalEffect { .. }
            ) {
                vec![d.options.iter().find(|o| o.label == "Do it").unwrap().id]
            } else {
                d.options
                    .iter()
                    .take(d.minimum.max(1).min(d.maximum))
                    .map(|o| o.id)
                    .collect()
            };
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
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| p.controller == PlayerId::One
                && p.card.definition == cards::PLAINS
                && p.tapped)
            .count(),
        2
    );
    assert_eq!(game.players[0].library.len(), 1);
}

#[test]
fn stoic_sphinx_loses_hexproof_after_its_controller_casts_a_spell() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::STOIC_SPHINX)
        .unwrap();
    assert!(game.permanent_has_executable_keyword(
        permanent(&game, cards::STOIC_SPHINX),
        KeywordAbility::Hexproof
    ));
    cast_at(
        &mut game,
        cards::LIGHTNING_BOLT,
        Target::Player(PlayerId::Two),
    );
    assert!(!game.permanent_has_executable_keyword(
        permanent(&game, cards::STOIC_SPHINX),
        KeywordAbility::Hexproof
    ));
}

#[test]
fn thunder_salvo_excludes_itself_from_the_spell_count() {
    let mut game = board(&[]);
    let target = game
        .put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .unwrap();
    cast_at(&mut game, cards::THUNDER_SALVO, Target::Permanent(target));
    assert_eq!(permanent(&game, cards::SERRA_ANGEL).damage, 2);
}
