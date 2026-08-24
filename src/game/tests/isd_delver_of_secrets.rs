//! Delver's private top-card look, optional public reveal, and transform.

use super::*;
use crate::ImplementationStatus;

fn upkeep_with(top: CardDefinitionId) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[PlayerId::One.index()].library.clear();
    let top_id = GameObjectId(91_001);
    game.players[PlayerId::One.index()]
        .library
        .push(card(top_id.0, top, PlayerId::One));
    let delver = game
        .put_onto_battlefield(PlayerId::One, cards::DELVER_OF_SECRETS)
        .expect("Delver is cataloged");
    drain_pending(&mut game);
    game.active_player = PlayerId::One;
    game.step = Step::Upkeep;
    game.priority = PlayerId::One;
    game.handle_upkeep_triggers();
    for _ in 0..16 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        let priority = game.priority;
        game.apply(priority, Action::PassPriority)
            .expect("priority advances Delver's upkeep trigger");
    }
    (game, delver, top_id)
}

fn choose_reveal(game: &mut Game, player: PlayerId) {
    let decision = game
        .observe(player)
        .decision
        .expect("Delver offers its controller the top card");
    let option = decision.options[0].id;
    game.apply(
        player,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .expect("revealing the inspected card is legal");
    drain_pending(game);
}

fn presented_part(game: &Game, delver: GameObjectId) -> CardPartId {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == delver)
        .expect("Delver remains on the battlefield")
        .presented
}

#[test]
fn instant_or_sorcery_reveal_transforms_delver_without_moving_the_card() {
    for top in [cards::LIGHTNING_BOLT, cards::DEMONIC_TUTOR] {
        let (mut game, delver, top_id) = upkeep_with(top);
        let decision = game
            .observe(PlayerId::One)
            .decision
            .expect("the controller privately looks at the top card");
        assert_eq!((decision.minimum, decision.maximum), (0, 1));
        assert_eq!(decision.options[0].card.map(|card| card.0), Some(top_id));
        assert!(game.observe(PlayerId::Two).decision.is_none());

        choose_reveal(&mut game, PlayerId::One);

        assert_eq!(presented_part(&game, delver), CardPartId(1));
        assert_eq!(game.players[0].library.last().unwrap().id, top_id);
        assert!(game.events.iter().any(|event| matches!(
            event,
            GameEvent::CardRevealed {
                player: PlayerId::One,
                card,
                definition,
            } if *card == top_id && *definition == top
        )));
    }
}

#[test]
fn declining_or_revealing_another_card_leaves_delver_front_face_up() {
    let (mut declined, delver, top_id) = upkeep_with(cards::LIGHTNING_BOLT);
    let decision = declined.observe(PlayerId::One).decision.unwrap();
    declined
        .apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: decision.id,
                options: Vec::new(),
            },
        )
        .unwrap();
    drain_pending(&mut declined);
    assert_eq!(presented_part(&declined, delver), CardPartId::PRIMARY);
    assert_eq!(declined.players[0].library.last().unwrap().id, top_id);
    assert!(
        !declined
            .events
            .iter()
            .any(|event| matches!(event, GameEvent::CardRevealed { .. }))
    );

    let (mut land, delver, top_id) = upkeep_with(cards::FOREST);
    choose_reveal(&mut land, PlayerId::One);
    assert_eq!(presented_part(&land, delver), CardPartId::PRIMARY);
    assert_eq!(land.players[0].library.last().unwrap().id, top_id);
    assert!(land.events.iter().any(|event| matches!(
        event,
        GameEvent::CardRevealed {
            definition: cards::FOREST,
            ..
        }
    )));
}

#[test]
fn back_face_is_a_blue_flying_three_two_and_coverage_is_complete() {
    let (mut game, delver, _) = upkeep_with(cards::LIGHTNING_BOLT);
    choose_reveal(&mut game, PlayerId::One);
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == delver)
        .unwrap();

    assert_eq!(
        (game.power(permanent), game.toughness(permanent)),
        (Some(3), Some(2)),
    );
    assert!(game.permanent_has_executable_keyword(permanent, KeywordAbility::Flying));
    assert_eq!(
        game.effective_colors(permanent, &game.effective_rules(permanent).unwrap()),
        [false, true, false, false, false],
    );
    assert_eq!(
        game.catalog
            .get(cards::DELVER_OF_SECRETS)
            .unwrap()
            .rules
            .implementation_status(),
        ImplementationStatus::Complete,
    );
}
