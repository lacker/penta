//! Preparation is a persistent designation, with a separately castable copy.
use super::woe_hob_completion::{hand, setup};
use super::*;

#[test]
fn woe_hob_preparation_casts_only_the_unbacked_frame_and_reconstructs() {
    for prepared_engine in [false, true] {
        let mut game = setup(prepared_engine);
        game.players[0].library = game
            .build_zone(PlayerId::One, &[cards::FOREST; 10])
            .unwrap();
        let physical = hand(&mut game, cards::EMERITUS_OF_IDEATION);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
        assert!(
            !game.legal_actions(PlayerId::One).iter().any(
                |action| matches!(action, Action::CastSpell { card, .. } if *card == physical)
            )
        );
        let source = game
            .put_onto_battlefield(PlayerId::One, cards::EMERITUS_OF_IDEATION)
            .unwrap();
        let copy = game.players[0].exile[0].id;
        assert!(matches!(
            game.players[0].exile[0].backing,
            ObjectBacking::None
        ));
        let view = game
            .printed_trigger_event_object(
                copy,
                cards::EMERITUS_OF_IDEATION,
                PlayerId::One,
                &CharacteristicContext::Exile,
            )
            .unwrap();
        assert!(view.types.contains(CardType::Instant));
        assert!(!view.types.contains(CardType::Creature));
        assert_eq!(view.mana_value, 1);
        assert_eq!(view.power, None);
        assert_eq!(
            game.object_card_name(copy).as_deref(),
            Some("Ancestral Recall")
        );
        let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
        assert_eq!(wire["exiles"][0][0]["name"], "Ancestral Recall");
        assert_eq!(wire["exiles"][0][0]["isCopy"], true);
        let mut rebuilt =
            Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 0)
                .unwrap();
        for state in [&mut game, &mut rebuilt] {
            assert!(matches!(
                state.players[0].exile[0].backing,
                ObjectBacking::None
            ));
            let action = state.legal_actions(PlayerId::One).into_iter().find(|action| matches!(action,
                Action::CastSpell { card, choices, .. } if *card == copy && choices.targets().iter().any(|target| target.targets() == [Target::Player(PlayerId::One)]))).unwrap();
            state.apply(PlayerId::One, action).unwrap();
            let spell = state.stack.last().unwrap();
            assert!(spell.is_copy);
            assert_eq!(
                state.object_card_name(copy).as_deref(),
                Some("Ancestral Recall")
            );
            assert_eq!(state.object_mana_symbol_count(copy, ManaColor::Blue), 1);
            assert!(spell.cast.as_ref().unwrap().was_cast());
            assert!(
                state
                    .battlefield
                    .iter()
                    .find(|p| p.card.id == source)
                    .unwrap()
                    .designations
                    .is_empty()
            );
            // Keep an ongoing lexical reference to the old exile object;
            // unreferenced last-known objects are deliberately pruned.
            state
                .stack
                .iter_mut()
                .last()
                .unwrap()
                .ability
                .as_mut()
                .unwrap()
                .context
                .bind_object_group(crate::Binding!("original-copy"), vec![Target::Card(copy)]);
            let (wire, hidden) = checkpoint_fixture(state, PlayerId::One);
            let mut from_stack = Game::from_observation_checkpoint(
                state.catalog.clone(),
                state.format,
                &wire,
                &hidden,
                0,
            )
            .unwrap();
            assert_eq!(
                from_stack.object_card_name(copy).as_deref(),
                Some("Ancestral Recall")
            );
            drain_pending(&mut from_stack);
            assert_eq!(from_stack.players[0].hand.len(), 4);
            assert!(from_stack.players[0].graveyard.is_empty());
            assert!(from_stack.players[0].exile.is_empty());
        }
    }
}

#[test]
fn woe_hob_preparation_tracks_controller_and_phasing_without_copying_the_designation() {
    let mut game = setup(true);
    let source = game
        .put_onto_battlefield(PlayerId::One, cards::EMERITUS_OF_IDEATION)
        .unwrap();
    let original_copy = game.players[0].exile[0].id;
    game.set_permanent_designation(source, crate::card::PermanentDesignationDef::Prepared, true);
    assert_eq!(
        game.players[0].exile.len(),
        1,
        "prepared cannot become prepared again"
    );
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == source)
        .unwrap()
        .controller = PlayerId::Two;
    assert!(
        game.exile_play_permission(original_copy, PlayerId::One)
            .is_none()
    );
    assert!(
        game.exile_play_permission(original_copy, PlayerId::Two)
            .is_some()
    );
    game.phase_out(source);
    assert!(game.players[0].exile.is_empty());
    assert_eq!(
        game.phased_out[0].designations,
        [crate::card::PermanentDesignationDef::Prepared]
    );
    let (wire, hidden) = checkpoint_fixture(&game, PlayerId::Two);
    let mut rebuilt =
        Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 0)
            .unwrap();
    for state in [&mut game, &mut rebuilt] {
        state.phase_in_for(PlayerId::Two);
        assert_eq!(state.players[1].exile.len(), 1);
        let copy = state.players[1].exile[0].id;
        assert_ne!(copy, original_copy);
        assert!(state.exile_play_permission(copy, PlayerId::Two).is_some());
        state.sacrifice_permanent(source);
        assert!(state.players[1].exile.is_empty());
        let bear = state
            .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
            .unwrap();
        state.set_permanent_designation(bear, crate::card::PermanentDesignationDef::Prepared, true);
        assert!(
            state
                .battlefield
                .iter()
                .find(|p| p.card.id == bear)
                .unwrap()
                .designations
                .is_empty()
        );
    }
}

#[test]
fn woe_hob_preparation_rejects_inconsistent_public_copy_and_designation_state() {
    let mut game = setup(true);
    game.put_onto_battlefield(PlayerId::One, cards::EMERITUS_OF_IDEATION)
        .unwrap();
    let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
    for kind in 0..4 {
        let mut changed = wire.clone();
        match kind {
            0 => changed["exiles"][0][0]["partId"] = serde_json::json!(0),
            1 => changed["exiles"][0][0]["isCopy"] = serde_json::json!(false),
            2 => changed["battlefield"][0]["designations"] = serde_json::json!([]),
            _ => changed["enduringStory"][0] = serde_json::json!(true),
        }
        assert!(
            Game::from_observation_checkpoint(
                game.catalog.clone(),
                game.format,
                &changed,
                &hidden,
                0
            )
            .is_err(),
            "accepted malformed case {kind}"
        );
    }
}

#[test]
fn woe_hob_emeritus_attack_reprepares_only_after_exiling_eight_cards() {
    for graveyard_size in [7, 8] {
        let mut game = setup(true);
        let source = game
            .put_onto_battlefield(PlayerId::One, cards::EMERITUS_OF_IDEATION)
            .unwrap();
        game.set_permanent_designation(
            source,
            crate::card::PermanentDesignationDef::Prepared,
            false,
        );
        game.players[0].graveyard = game
            .build_zone(PlayerId::One, &vec![cards::FOREST; graveyard_size])
            .unwrap();
        game.step = Step::DeclareAttackers;
        game.declare_attacker(source, AttackDefender::Player(PlayerId::Two));
        game.finish_declaring_attackers();
        for _ in 0..32 {
            if let Some(pending) = game.pending_decisions.first() {
                let decision = pending.observation.clone();
                let options = if matches!(pending.continuation, DecisionContinuation::PayOr { .. })
                {
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
            } else if game.stack.is_empty() && game.pending_triggers.is_empty() {
                break;
            } else {
                game.apply(game.priority, Action::PassPriority).unwrap();
            }
        }
        assert_eq!(
            game.players[0].graveyard.len(),
            if graveyard_size == 8 { 0 } else { 7 }
        );
        assert_eq!(
            game.prepared_spell_copies.len(),
            usize::from(graveyard_size == 8)
        );
    }
}
