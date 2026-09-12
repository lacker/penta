//! Teenage Mutant Ninja Turtles compositions and casting regressions.

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
    game.format = Format::IsdM14Standard;
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
        .find(|a| matches!(a, Action::ActivateAbility {source,..} if *source == id))
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
fn mutagen_tokens_activate_at_sorcery_speed_and_put_counters_on_creatures() {
    let mut game = board(&[]);
    let crab = game
        .put_onto_battlefield(PlayerId::One, cards::CRUSTACEAN_COMMANDO)
        .unwrap();
    settle(&mut game);
    let mutagen = game
        .battlefield
        .iter()
        .find(|p| p.card.definition.is_token())
        .unwrap()
        .card
        .id;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    activate(&mut game, mutagen);
    assert_eq!(game.battlefield.len(), 1);
    assert_eq!(
        game.battlefield
            .iter()
            .find(|p| p.card.id == crab)
            .unwrap()
            .counters(CounterKind::PlusOnePlusOne),
        1
    );
}

#[test]
fn alliance_triggers_for_other_creatures_only() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::EPF_POINT_SQUAD)
        .unwrap();
    settle(&mut game);
    assert_eq!(
        permanent(&game, cards::EPF_POINT_SQUAD).counters(CounterKind::PlusOnePlusOne),
        0
    );
    game.put_onto_battlefield(PlayerId::One, cards::SQUIRRELANOIDS)
        .unwrap();
    settle(&mut game);
    assert_eq!(
        permanent(&game, cards::EPF_POINT_SQUAD).counters(CounterKind::PlusOnePlusOne),
        1
    );
}

#[test]
fn ooze_spill_counters_a_spell_and_creates_a_mutagen() {
    let mut game = board(&[]);
    let spell = held(&mut game, cards::SQUIRRELANOIDS);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a,Action::CastSpell{card,..} if *card==spell))
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    cast(&mut game, cards::OOZE_SPILL);
    assert_eq!(game.battlefield.len(), 1);
    assert!(game.battlefield[0].card.definition.is_token());
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|c| c.definition == cards::SQUIRRELANOIDS)
    );
}

#[test]
fn broadcast_takeover_freezes_opposing_artifacts_before_changing_control() {
    let mut game = board(&[]);
    let owned = game
        .put_onto_battlefield(PlayerId::One, cards::BUZZ_BOTS)
        .unwrap();
    let stolen = game
        .put_onto_battlefield(PlayerId::Two, cards::BUZZ_BOTS)
        .unwrap();
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == stolen)
        .unwrap()
        .tapped = true;
    cast(&mut game, cards::BROADCAST_TAKEOVER);
    let stolen = game
        .battlefield
        .iter()
        .find(|p| p.card.id == stolen)
        .unwrap();
    assert_eq!(stolen.controller, PlayerId::One);
    assert!(!stolen.tapped);
    assert!(game.permanent_has_executable_keyword(stolen, KeywordAbility::Haste));
    let owned = game
        .battlefield
        .iter()
        .find(|p| p.card.id == owned)
        .unwrap();
    assert!(!game.permanent_has_executable_keyword(owned, KeywordAbility::Haste));
}

#[test]
fn hacktivist_counts_distinct_spell_types_including_artifact_creatures() {
    let mut game = board(&[cards::FOREST; 6]);
    cast(&mut game, cards::BUZZ_BOTS);
    for _ in 0..2 {
        cast(&mut game, cards::HAMATO_GUARDIAN_STANCE);
    }
    game.put_onto_battlefield(PlayerId::One, cards::APRIL_O_NEIL_HACKTIVIST)
        .unwrap();
    game.step = Step::PostcombatMain;
    game.advance_step();
    settle(&mut game);
    assert_eq!(game.players[0].hand.len(), 3);
}

#[test]
fn cloning_saga_copies_its_exiled_creature_on_successive_chapters() {
    let mut game = board(&[]);
    game.players[0].graveyard = game
        .build_zone(PlayerId::One, &[cards::SQUIRRELANOIDS])
        .unwrap();
    cast(&mut game, cards::THE_CLONING_OF_SHREDDER);
    assert_eq!(game.players[0].exile.len(), 1);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| p.card.definition.is_token())
            .count(),
        1
    );
    game.step = Step::Draw;
    game.advance_step();
    settle(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| p.card.definition.is_token())
            .count(),
        2
    );
}

#[test]
fn northampton_returns_one_exiled_creature_and_puts_the_other_in_hand() {
    let mut game = board(&[]);
    let farm = game
        .put_onto_battlefield(PlayerId::One, cards::NORTHAMPTON_FARM)
        .unwrap();
    game.put_onto_battlefield(PlayerId::One, cards::SQUIRRELANOIDS)
        .unwrap();
    game.put_onto_battlefield(PlayerId::One, cards::BUZZ_BOTS)
        .unwrap();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 4);
    for _ in 0..2 {
        game.battlefield
            .iter_mut()
            .find(|p| p.card.id == farm)
            .unwrap()
            .tapped = false;
        activate(&mut game, farm);
    }
    assert_eq!(game.players[0].exile.len(), 2);
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == farm)
        .unwrap()
        .tapped = false;
    activate(&mut game, farm);
    assert!(game.players[0].exile.is_empty());
    assert_eq!(game.players[0].hand.len(), 1);
    assert_eq!(game.battlefield.len(), 1);
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|c| c.definition == cards::NORTHAMPTON_FARM)
    );
}

#[test]
fn koyas_delayed_return_uses_the_new_exiled_card_identity() {
    let mut game = board(&[]);
    let original = game
        .put_onto_battlefield(PlayerId::Two, cards::SQUIRRELANOIDS)
        .unwrap();
    game.put_onto_battlefield(PlayerId::One, cards::KOYA_DEATH_FROM_ABOVE)
        .unwrap();
    settle(&mut game);
    assert_eq!(game.players[1].exile.len(), 1);
    game.step = Step::PostcombatMain;
    game.advance_step();
    settle(&mut game);
    let returned = permanent(&game, cards::SQUIRRELANOIDS);
    assert_ne!(returned.card.id, original);
    assert_eq!(returned.controller, PlayerId::Two);
}

#[test]
fn retro_mutation_removes_abilities_and_changes_base_stats() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::BUZZ_BOTS)
        .unwrap();
    cast(&mut game, cards::RETRO_MUTATION);
    let creature = permanent(&game, cards::BUZZ_BOTS);
    assert_eq!(game.power(creature), Some(0));
    assert_eq!(game.toughness(creature), Some(1));
    assert!(!game.permanent_has_executable_keyword(creature, KeywordAbility::Flying));
    assert!(!game.permanent_has_executable_keyword(creature, KeywordAbility::Vigilance));
}

#[test]
fn katana_attaches_to_an_entering_ninja_and_untaps_it_after_combat_damage() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::QUINTESSENTIAL_KATANA)
        .unwrap();
    let ninja = game
        .put_onto_battlefield(PlayerId::One, cards::FOOT_ELITE)
        .unwrap();
    settle(&mut game);
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == ninja)
        .unwrap()
        .tapped = true;
    game.deal_combat_damage_to_player(ninja, PlayerId::Two, 3);
    settle(&mut game);
    assert_eq!(game.players[0].life, 22);
    assert!(
        !game
            .battlefield
            .iter()
            .find(|p| p.card.id == ninja)
            .unwrap()
            .tapped
    );
}

#[test]
fn old_hobs_token_has_haste_and_is_destroyed_at_the_next_end_step() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::OLD_HOB_ALLEYCAT_BLUES)
        .unwrap();
    game.step = Step::PrecombatMain;
    game.advance_step();
    settle(&mut game);
    let token = game
        .battlefield
        .iter()
        .find(|p| p.card.definition.is_token())
        .unwrap();
    assert!(game.permanent_has_executable_keyword(token, KeywordAbility::Haste));
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
fn shredders_revenge_draw_mode_affects_the_targeted_opponent() {
    let mut game = board(&[]);
    let id = held(&mut game, cards::SHREDDER_S_REVENGE);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| {
            matches!(a,
                Action::CastSpell {card,choices,..} if *card == id
                    && choices.modes().iter().any(|mode| mode.index() == 1)
                    && choices.iter_targets().any(|t| *t == Target::Player(PlayerId::Two))
            )
        })
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    assert_eq!(game.players[1].hand.len(), 2);
    assert_eq!(game.players[1].life, 18);
    assert!(game.players[0].hand.is_empty());
}
