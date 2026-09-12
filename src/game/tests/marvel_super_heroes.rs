//! Marvel Super Heroes regressions for compositions of existing operations.

use super::*;

fn board(library: &[CardDefinitionId]) -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    for player in &mut game.players {
        player.hand.clear();
        player.graveyard.clear();
    }
    game.players[0].library = game.build_zone(PlayerId::One, library).unwrap();
    game.players[0].library.reverse();
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
fn iron_lad_reveals_only_the_top_card_and_draws_it_only_if_artifact() {
    let mut game = board(&[cards::MIND_STONE, cards::FOREST]);
    let lad = game
        .put_onto_battlefield(PlayerId::One, cards::IRON_LAD_DIVERGING_DESTINY)
        .unwrap();
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == lad)
        .unwrap()
        .entered_controller_turn = 0;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 2);
    activate(&mut game, lad);
    assert_eq!(game.players[0].hand.len(), 1);
    assert_eq!(game.players[0].hand[0].definition, cards::MIND_STONE);
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == lad)
        .unwrap()
        .tapped = false;
    activate(&mut game, lad);
    assert_eq!(game.players[0].hand.len(), 1);
    assert_eq!(game.players[0].library.len(), 1);
}

#[test]
fn super_adaptoid_adds_missing_keywords_without_duplicate_counters() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::THOR_ODINSON)
        .unwrap();
    let adaptoid = game
        .put_onto_battlefield(PlayerId::One, cards::SUPER_ADAPTOID)
        .unwrap();
    settle(&mut game);
    let p = permanent(&game, cards::SUPER_ADAPTOID);
    assert_eq!(game.power(p), Some(2));
    assert_eq!(p.counters(CounterKind::Flying), 1);
    assert_eq!(p.counters(CounterKind::Vigilance), 1);
    assert_eq!(p.counters(CounterKind::FirstStrike), 0);
    game.step = Step::DeclareAttackers;
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == adaptoid)
        .unwrap()
        .entered_controller_turn = 0;
    game.declare_attacker(adaptoid, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    settle(&mut game);
    assert_eq!(
        permanent(&game, cards::SUPER_ADAPTOID).counters(CounterKind::Flying),
        1
    );
}

#[test]
fn shared_noncreature_abilities_keep_both_instances_of_prowess() {
    let mut game = board(&[cards::FOREST; 3]);
    game.put_onto_battlefield(PlayerId::One, cards::THOR_ODINSON)
        .unwrap();
    cast(&mut game, cards::PYM_PARTICLES);
    assert_eq!(game.power(permanent(&game, cards::THOR_ODINSON)), Some(6));
    assert_eq!(game.players[0].hand.len(), 1);
}

#[test]
fn spell_draws_for_the_named_target_player() {
    let mut game = board(&[]);
    game.players[1].library = game.build_zone(PlayerId::Two, &[cards::FOREST; 4]).unwrap();
    let skrull = game
        .put_onto_battlefield(PlayerId::One, cards::SUPER_SKRULL)
        .unwrap();
    for c in ManaColor::COLORS {
        game.add_unrestricted_mana(PlayerId::One, c, 10);
    }
    let action=game.legal_actions(PlayerId::One).into_iter().find(|a|matches!(a,Action::ActivateAbility{source,targets,..} if *source==skrull&&targets.iter().any(|t|t.targets().contains(&Target::Player(PlayerId::Two))))).unwrap();
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    assert_eq!(game.players[1].hand.len(), 4);
    assert!(game.players[0].hand.is_empty());
}

#[test]
fn alien_invasion_counts_existing_invasion_counters_before_adding_the_next() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::ALIEN_INVASION)
        .unwrap();
    for _ in 0..2 {
        game.step = Step::PrecombatMain;
        game.advance_step();
        settle(&mut game);
    }
    let mut sizes = game
        .battlefield
        .iter()
        .filter(|p| p.card.definition.is_token())
        .map(|p| game.power(p).unwrap())
        .collect::<Vec<_>>();
    sizes.sort();
    assert_eq!(sizes, [1, 2]);
    assert_eq!(
        permanent(&game, cards::ALIEN_INVASION).counters(CounterKind::named("invasion")),
        2
    );
}

#[test]
fn us_agent_creates_and_attaches_a_shield_with_its_own_equip_ability() {
    let mut game = board(&[]);
    let agent = game
        .put_onto_battlefield(PlayerId::One, cards::U_S_AGENT_JOHN_WALKER)
        .unwrap();
    settle(&mut game);
    let shield = game
        .battlefield
        .iter()
        .find(|p| p.card.definition.is_token())
        .unwrap();
    assert_eq!(shield.attached_to, Some(agent));
    assert_eq!(
        game.power(permanent(&game, cards::U_S_AGENT_JOHN_WALKER)),
        Some(4)
    );
    let shield_id = shield.card.id;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 2);
    assert!(
        game.legal_actions(PlayerId::One)
            .iter()
            .any(|a| matches!(a,Action::ActivateAbility{source,..}if *source==shield_id))
    );
}

#[test]
fn daredevil_tracks_the_exiled_top_card_and_grants_permission_only_to_it() {
    let mut game = board(&[cards::HERO_IN_TRAINING, cards::FOREST]);
    let hero = game
        .put_onto_battlefield(PlayerId::One, cards::DAREDEVIL_MAN_WITHOUT_FEAR)
        .unwrap();
    let before = game
        .power(permanent(&game, cards::DAREDEVIL_MAN_WITHOUT_FEAR))
        .unwrap();
    game.step = Step::DeclareAttackers;
    game.declare_attacker(hero, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    settle(&mut game);
    assert_eq!(game.players[0].library.len(), 1);
    assert_eq!(game.players[0].library[0].definition, cards::FOREST);
    assert_eq!(
        game.power(permanent(&game, cards::DAREDEVIL_MAN_WITHOUT_FEAR)),
        Some(before + 2)
    );
    game.step = Step::PostcombatMain;
    for c in ManaColor::COLORS {
        game.add_unrestricted_mana(PlayerId::One, c, 5);
    }
    assert!(
        game.legal_actions(PlayerId::One)
            .iter()
            .any(|a| matches!(a, Action::CastSpell { .. }))
    );
}

#[test]
fn connive_counts_nonlands_but_not_lands() {
    for (drawn, expected) in [(cards::MIND_STONE, 1), (cards::FOREST, 0)] {
        let mut game = board(&[drawn]);
        game.put_onto_battlefield(PlayerId::One, cards::RED_ROOM_RECRUIT)
            .unwrap();
        settle(&mut game);
        assert_eq!(
            permanent(&game, cards::RED_ROOM_RECRUIT).counters(CounterKind::PlusOnePlusOne),
            expected
        );
        assert!(game.players[0].hand.is_empty());
        assert_eq!(game.players[0].graveyard[0].definition, drawn);
    }
}

#[test]
fn declined_discard_or_sacrifice_does_not_draw() {
    let mut game = board(&[cards::FOREST; 3]);
    cast(&mut game, cards::VISION_OF_LOVE);
    assert!(game.players[0].hand.is_empty());
    assert_eq!(game.players[0].library.len(), 3);
}

#[test]
fn darklands_offer_colored_mana_only_on_entry_turn_or_with_a_basic() {
    let mut game = board(&[]);
    let land = game
        .put_onto_battlefield(PlayerId::One, cards::DARK_FORTRESS)
        .unwrap();
    let choices = |game: &Game| {
        game.legal_actions(PlayerId::One)
            .into_iter()
            .filter(|a| matches!(a,Action::ActivateManaAbility{source,..}if *source==land))
            .count()
    };
    assert_eq!(choices(&game), 3);
    game.turn += 1;
    assert_eq!(choices(&game), 1);
    game.put_onto_battlefield(PlayerId::One, cards::FOREST)
        .unwrap();
    assert_eq!(choices(&game), 3);
}

#[test]
fn conditional_flash_uses_the_opponents_spell_history() {
    let mut game = board(&[cards::FOREST; 3]);
    game.put_onto_battlefield(PlayerId::One, cards::CAPTAIN_MAR_VELL_SPACE_BORN)
        .unwrap();
    let creature = held(&mut game, cards::HERO_IN_TRAINING);
    game.step = Step::BeginningOfCombat;
    let can_cast = |game: &Game| {
        game.legal_actions(PlayerId::One)
            .iter()
            .any(|a| matches!(a,Action::CastSpell{card,..}if *card==creature))
    };
    assert!(!can_cast(&game));
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::Two;
    let instant = game.build_zone(PlayerId::Two, &[cards::CONSIDER]).unwrap();
    let instant_id = instant[0].id;
    game.players[1].hand.extend(instant);
    game.players[1].library = game.build_zone(PlayerId::Two, &[cards::FOREST; 3]).unwrap();
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Blue, 1);
    let action = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|a| matches!(a,Action::CastSpell{card,..}if *card==instant_id))
        .unwrap();
    game.apply(PlayerId::Two, action).unwrap();
    settle(&mut game);
    game.priority = PlayerId::One;
    assert!(can_cast(&game));
}

#[test]
fn hulkling_checks_the_size_comparison_again_when_its_trigger_resolves() {
    let mut game = board(&[]);
    let hulkling = game
        .put_onto_battlefield(PlayerId::One, cards::HULKLING_BURGEONING_BRUISER)
        .unwrap();
    let before = game
        .power(permanent(&game, cards::HULKLING_BURGEONING_BRUISER))
        .unwrap();
    game.put_onto_battlefield(PlayerId::One, cards::CRAW_WURM)
        .unwrap();
    assert!(!game.pending_triggers.is_empty() || !game.stack.is_empty());
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == hulkling)
        .unwrap()
        .add_counters(CounterKind::PlusOnePlusOne, 10);
    settle(&mut game);
    assert_eq!(
        game.power(permanent(&game, cards::HULKLING_BURGEONING_BRUISER)),
        Some(before + 10)
    );
}

#[test]
fn loki_draws_for_an_ability_target_but_not_a_spell_target() {
    let mut game = board(&[cards::FOREST; 4]);
    game.put_onto_battlefield(PlayerId::One, cards::LOKI_GOD_OF_MISCHIEF)
        .unwrap();
    let patriot = game
        .put_onto_battlefield(PlayerId::One, cards::PATRIOT_SHIELD_WIELDER)
        .unwrap();
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == patriot)
        .unwrap()
        .entered_controller_turn = 0;
    cast(&mut game, cards::PYM_PARTICLES);
    assert_eq!(game.players[0].hand.len(), 1);
    activate(&mut game, patriot);
    assert_eq!(game.players[0].hand.len(), 2);
}

#[test]
fn cast_target_predicates_support_hero_triggers_and_a_temporary_activation() {
    let mut game = board(&[cards::FOREST; 6]);
    game.put_onto_battlefield(PlayerId::One, cards::COLLEEN_WING_STREET_SAMURAI)
        .unwrap();
    game.put_onto_battlefield(PlayerId::One, cards::MOCKINGBIRD_ACE_AGENT)
        .unwrap();
    let fist = game
        .put_onto_battlefield(PlayerId::One, cards::IRON_FIST_LIVING_WEAPON)
        .unwrap();
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == fist)
        .unwrap()
        .entered_controller_turn = 0;
    cast(&mut game, cards::PYM_PARTICLES);
    assert_eq!(
        permanent(&game, cards::COLLEEN_WING_STREET_SAMURAI).counters(CounterKind::PlusOnePlusOne),
        1
    );
    assert_eq!(
        permanent(&game, cards::MOCKINGBIRD_ACE_AGENT).counters(CounterKind::PlusOnePlusOne),
        1
    );
    let action=game.legal_actions(PlayerId::One).into_iter().find(|a|matches!(a,Action::ActivateAbility{source,targets,..}if *source==fist&&targets.iter().any(|s|s.targets().contains(&Target::Player(PlayerId::Two))))).unwrap();
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    assert_eq!(game.players[1].life, 18);
    assert_eq!(
        permanent(&game, cards::COLLEEN_WING_STREET_SAMURAI).counters(CounterKind::PlusOnePlusOne),
        1
    );
}

#[test]
fn fin_fang_foom_copies_a_spell_targeting_an_artifact() {
    let mut game = board(&[cards::FOREST; 4]);
    game.put_onto_battlefield(PlayerId::One, cards::FIN_FANG_FOOM)
        .unwrap();
    let stone = game
        .put_onto_battlefield(PlayerId::One, cards::MIND_STONE)
        .unwrap();
    let id = held(&mut game, cards::I_AM_IRON_MAN);
    let action=game.legal_actions(PlayerId::One).into_iter().find(|a|matches!(a,Action::CastSpell{card,choices,..}if *card==id&&choices.iter_targets().any(|t|*t==Target::Permanent(stone)))).unwrap();
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    assert_eq!(game.players[0].hand.len(), 2);
    assert_eq!(
        permanent(&game, cards::FIN_FANG_FOOM).counters(CounterKind::PlusOnePlusOne),
        2
    );
}
