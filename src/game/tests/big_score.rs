//! Regression coverage for The Big Score's combinations of existing game operations.

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
fn oltec_matterweaver_creates_an_artifact_gnome_for_a_creature_cast() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::OLTEC_MATTERWEAVER)
        .unwrap();
    cast(&mut game, cards::GRIZZLY_BEARS);
    let token = game
        .battlefield
        .iter()
        .find(|p| p.card.definition.is_token())
        .unwrap();
    assert!(
        game.permanent_types(token)
            .unwrap()
            .contains(CardType::Artifact)
    );
    assert_eq!(game.creature_stats(token).unwrap().power, 1);
}

#[test]
fn synthesizer_construct_counts_itself_and_tracks_artifacts() {
    let mut game = board(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::SIMULACRUM_SYNTHESIZER)
        .unwrap();
    settle(&mut game);
    let dynamo = game
        .put_onto_battlefield(PlayerId::One, cards::THRAN_DYNAMO)
        .unwrap();
    settle(&mut game);
    let token = game
        .battlefield
        .iter()
        .find(|p| p.card.definition.is_token())
        .unwrap()
        .card
        .id;
    assert_eq!(
        game.creature_stats(
            game.battlefield
                .iter()
                .find(|p| p.card.id == token)
                .unwrap()
        )
        .unwrap()
        .power,
        3
    );
    game.destroy_permanent(dynamo);
    settle(&mut game);
    assert_eq!(
        game.creature_stats(
            game.battlefield
                .iter()
                .find(|p| p.card.id == token)
                .unwrap()
        )
        .unwrap()
        .power,
        2
    );
}

#[test]
fn esoteric_duplicator_can_copy_itself_after_being_sacrificed() {
    let mut game = board(&[cards::PLAINS]);
    let id = game
        .put_onto_battlefield(PlayerId::One, cards::ESOTERIC_DUPLICATOR)
        .unwrap();
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 4);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|a| matches!(a,Action::ActivateAbility{source,..} if *source==id))
        .unwrap();
    game.apply(PlayerId::One, action).unwrap();
    for _ in 0..40 {
        if let Some(pending) = game.pending_decisions.first() {
            let d = pending.observation.clone();
            let options = if matches!(pending.continuation, DecisionContinuation::PayOr { .. }) {
                vec![d.options.iter().find(|o| o.label != "Decline").unwrap().id]
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
    assert!(game.battlefield.is_empty());
    game.step = Step::PostcombatMain;
    game.advance_step();
    settle(&mut game);
    assert_eq!(game.battlefield.len(), 1);
    assert!(game.battlefield[0].card.definition.is_token());
}

#[test]
fn molten_duplication_adds_artifact_and_expires_at_the_end_step() {
    let mut game = board(&[]);
    let bear = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    cast_at(
        &mut game,
        cards::MOLTEN_DUPLICATION,
        Target::Permanent(bear),
    );
    let token = game
        .battlefield
        .iter()
        .find(|p| p.card.definition.is_token())
        .unwrap();
    assert!(
        game.permanent_types(token)
            .unwrap()
            .contains(CardType::Artifact)
    );
    assert!(game.permanent_has_executable_keyword(token, KeywordAbility::Haste));
    for p in &mut game.players {
        p.mana_pool = ManaPool::default();
    }
    game.step = Step::PostcombatMain;
    game.advance_step();
    settle(&mut game);
    assert_eq!(game.battlefield.len(), 1);
    assert_eq!(game.battlefield[0].card.id, bear);
}

#[test]
fn territory_forge_borrows_the_exiled_artifacts_mana_ability() {
    let mut game = board(&[]);
    let ring = game
        .put_onto_battlefield(PlayerId::Two, cards::SOL_RING)
        .unwrap();
    cast(&mut game, cards::TERRITORY_FORGE);
    assert!(!game.battlefield.iter().any(|p| p.card.id == ring));
    assert_eq!(game.players[1].exile.len(), 1);
    game.players[0].mana_pool = ManaPool::default();
    let forge = permanent(&game, cards::TERRITORY_FORGE).card.id;
    activate(&mut game, forge);
    assert_eq!(game.players[0].mana_pool.total(), 2);
}

#[test]
fn nexus_creates_a_three_three_artifact_creature_copy() {
    let mut game = board(&[cards::PLAINS]);
    game.players[0].hand = game
        .build_zone(PlayerId::One, &[cards::GRIZZLY_BEARS])
        .unwrap();
    game.put_onto_battlefield(PlayerId::One, cards::NEXUS_OF_BECOMING)
        .unwrap();
    game.step = Step::PrecombatMain;
    game.advance_step();
    settle(&mut game);
    let token = game
        .battlefield
        .iter()
        .find(|p| p.card.definition.is_token())
        .unwrap();
    assert_eq!(game.creature_stats(token).unwrap().power, 3);
    assert_eq!(game.creature_stats(token).unwrap().toughness, 3);
    assert!(
        game.permanent_types(token)
            .unwrap()
            .contains(CardType::Artifact)
    );
    assert_eq!(game.players[0].exile.len(), 1);
}
