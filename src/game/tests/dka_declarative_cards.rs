//! Dark Ascension declarations whose choices or outcome clauses are easy to
//! put at the wrong rules boundary.

use super::*;

fn choose_card(game: &mut Game, player: PlayerId, card: GameObjectId) {
    let decision = game
        .observe(player)
        .decision
        .expect("the triggered ability is waiting for its target");
    let option = decision
        .options
        .iter()
        .find(|option| option.card.is_some_and(|(object, _)| object == card))
        .expect("the requested permanent is a legal target")
        .id;
    game.apply(
        player,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .expect("the target choice is legal");
}

fn permanent_is_tapped(game: &Game, object: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == object)
        .expect("the permanent remains on the battlefield")
        .tapped
}

#[test]
fn niblis_chooses_tap_or_untap_during_resolution() {
    let mut game = ready_game();
    game.battlefield.clear();
    let niblis = game
        .put_onto_battlefield(PlayerId::One, cards::NIBLIS_OF_THE_BREATH)
        .expect("Niblis of the Breath is cataloged");
    let target = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .expect("Grizzly Bears is cataloged");
    drain_pending(&mut game);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == niblis)
        .expect("Niblis is on the battlefield")
        .entered_controller_turn = 0;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);

    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility {
                    source,
                    targets,
                    modes,
                    ..
                } if *source == niblis
                    && modes.is_empty()
                    && targets.iter().flat_map(TargetSelection::targets)
                        .any(|chosen| *chosen == Target::Permanent(target))
            )
        })
        .expect("Niblis has one nonmodal activation targeting the creature");
    game.apply(PlayerId::One, activation)
        .expect("the Niblis ability activates");

    game.tap_permanent(target);
    pass_priority_pair(&mut game);
    choose_decision_by_label(&mut game, PlayerId::One, "Do it");
    choose_decision_by_label(&mut game, PlayerId::One, "Untap the target creature");

    assert!(
        !permanent_is_tapped(&game, target),
        "the untap branch was chosen only after the target became tapped",
    );
}

fn resolve_ransacker_trigger(game: &mut Game, ransacker: GameObjectId, target: GameObjectId) {
    game.transform_permanent(ransacker);
    game.finish_rules_procedure();
    choose_card(game, PlayerId::One, target);
    pass_priority_pair(game);
    choose_decision_by_label(game, PlayerId::One, "Do it");
    drain_pending(game);
}

fn ransacker_game(artifact: crate::CardDefinitionId) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    let ransacker = game
        .put_onto_battlefield(PlayerId::One, cards::AFFLICTED_DESERTER)
        .expect("Afflicted Deserter is cataloged");
    let target = game
        .put_onto_battlefield(PlayerId::Two, artifact)
        .expect("the target artifact is cataloged");
    drain_pending(&mut game);
    (game, ransacker, target)
}

#[test]
fn ransacker_deals_damage_only_when_the_artifact_reaches_a_graveyard() {
    let (mut ordinary, ransacker, target) = ransacker_game(cards::SOL_RING);
    resolve_ransacker_trigger(&mut ordinary, ransacker, target);
    assert_eq!(ordinary.players[PlayerId::Two.index()].life, 17);
    assert!(
        ordinary.players[PlayerId::Two.index()]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::SOL_RING),
    );

    let (mut indestructible, ransacker, target) = ransacker_game(cards::DARKSTEEL_PLATE);
    resolve_ransacker_trigger(&mut indestructible, ransacker, target);
    assert_eq!(indestructible.players[PlayerId::Two.index()].life, 20);
    assert!(
        indestructible
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == target),
        "an indestructible artifact remains and causes no damage",
    );

    let (mut replaced, ransacker, target) = ransacker_game(cards::SOL_RING);
    replaced
        .put_onto_battlefield(PlayerId::Two, cards::REST_IN_PEACE)
        .expect("Rest in Peace is cataloged");
    drain_pending(&mut replaced);
    resolve_ransacker_trigger(&mut replaced, ransacker, target);
    assert_eq!(replaced.players[PlayerId::Two.index()].life, 20);
    assert!(
        replaced.players[PlayerId::Two.index()]
            .exile
            .iter()
            .any(|card| card.definition == cards::SOL_RING),
        "Rest in Peace replaces the graveyard move and causes no damage",
    );
}
