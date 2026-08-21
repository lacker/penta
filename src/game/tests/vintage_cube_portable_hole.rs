//! Portable Hole: one white mana that takes a cheap permanent away, and
//! gives it back the moment the Hole itself goes.

use super::*;

/// Player One holding a Hole, with `theirs` on the battlefield under Player
/// Two, and one white mana up.
fn staged(theirs: &[CardDefinitionId]) -> (Game, GameObjectId, Vec<GameObjectId>) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let mut opposing = Vec::new();
    for definition in theirs {
        opposing.push(
            game.put_onto_battlefield(PlayerId::Two, *definition)
                .expect("cataloged"),
        );
    }
    let card = game
        .build_zone(PlayerId::One, &[cards::PORTABLE_HOLE])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let hole = card.id;
    game.players[0].hand.push(card);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [1, 1];
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 1);
    drain_pending(&mut game);
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, hole, opposing)
}

fn settle(game: &mut Game) {
    for _ in 0..24 {
        if game.observe(PlayerId::One).decision.is_some()
            || game.observe(PlayerId::Two).decision.is_some()
        {
            return;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    game.check_state_based_actions();
}

/// Casts the Hole and points its enters trigger at `victim`.
fn cast_at(game: &mut Game, hole: GameObjectId, victim: GameObjectId) {
    cast(game, hole);
    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the trigger asks what to swallow");
    let option = decision
        .options
        .iter()
        .find(|option| option.card.is_some_and(|(id, _)| id == victim))
        .expect("the victim is a legal target")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .expect("the target is legal");
    settle(game);
}

fn cast(game: &mut Game, hole: GameObjectId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == hole))
        .expect("one white mana casts it");
    game.apply(PlayerId::One, action).expect("it casts");
    settle(game);
}

/// Which permanents the trigger could legally point at, by definition.
fn targets_offered(game: &Game) -> Vec<CardDefinitionId> {
    game.observe(PlayerId::One)
        .decision
        .map(|decision| {
            decision
                .options
                .iter()
                .filter_map(|option| {
                    option
                        .card
                        .and_then(|(_, characteristics)| characteristics.card_definition())
                })
                .collect()
        })
        .unwrap_or_default()
}

fn on_battlefield(game: &Game, definition: CardDefinitionId) -> bool {
    game.battlefield
        .iter()
        .any(|permanent| permanent.card.definition == definition)
}

/// The Hole swallows a cheap creature the moment it lands.
#[test]
fn it_exiles_a_cheap_opposing_permanent() {
    let (mut game, hole, theirs) = staged(&[cards::SAVANNAH_LIONS]);

    cast_at(&mut game, hole, theirs[0]);

    assert!(
        !on_battlefield(&game, cards::SAVANNAH_LIONS),
        "the Lions left the battlefield",
    );
    assert_eq!(
        game.players[1]
            .exile
            .iter()
            .filter(|card| card.definition == cards::SAVANNAH_LIONS)
            .count(),
        1,
        "and it is in its owner's exile",
    );
}

/// Destroying the Hole gives the permanent back, under its owner's control.
#[test]
fn breaking_the_hole_gives_the_permanent_back() {
    let (mut game, hole, theirs) = staged(&[cards::SAVANNAH_LIONS]);
    cast_at(&mut game, hole, theirs[0]);
    let hole_permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::PORTABLE_HOLE)
        .expect("the Hole is on the battlefield")
        .card
        .id;

    game.move_permanents_to_graveyard(&[hole_permanent]);
    settle(&mut game);

    let returned = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::SAVANNAH_LIONS)
        .expect("the Lions came back");
    assert_eq!(
        returned.controller,
        PlayerId::Two,
        "under its owner's control, not the Hole controller's",
    );
    assert!(
        game.players[1].exile.is_empty(),
        "and nothing was left in exile",
    );
}

/// "Nonland permanent an opponent controls with mana value 2 or less" is
/// three restrictions, and each one bites.
#[test]
fn it_cannot_point_at_a_land_a_big_creature_or_your_own_things() {
    let (mut game, hole, _theirs) =
        staged(&[cards::MOUNTAIN, cards::SERRA_ANGEL, cards::SAVANNAH_LIONS]);
    let mine = game
        .put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    drain_pending(&mut game);

    cast(&mut game, hole);

    let offered = targets_offered(&game);
    assert!(
        offered.contains(&cards::SAVANNAH_LIONS),
        "the cheap opposing creature is offered: {offered:?}",
    );
    assert!(
        !offered.contains(&cards::MOUNTAIN),
        "a land is not: {offered:?}",
    );
    assert!(
        !offered.contains(&cards::SERRA_ANGEL),
        "and neither is a five-drop: {offered:?}",
    );
    assert!(
        !game
            .observe(PlayerId::One)
            .decision
            .expect("the trigger is asking")
            .options
            .iter()
            .any(|option| option.card.is_some_and(|(id, _)| id == mine)),
        "nor your own creature, however cheap",
    );
}

/// The card that comes back is a new object, so it arrives summoning sick
/// rather than resuming whatever it was doing.
#[test]
fn what_comes_back_is_a_new_permanent() {
    let (mut game, hole, theirs) = staged(&[cards::SAVANNAH_LIONS]);
    let before = theirs[0];
    cast_at(&mut game, hole, before);
    let hole_permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::PORTABLE_HOLE)
        .expect("the Hole is on the battlefield")
        .card
        .id;

    game.move_permanents_to_graveyard(&[hole_permanent]);
    settle(&mut game);

    let returned = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::SAVANNAH_LIONS)
        .expect("the Lions came back");
    assert_ne!(
        returned.card.id, before,
        "a card that changed zones twice is not the permanent that left",
    );
}

/// Leaves, not dies: bouncing the Hole gives the permanent back just as
/// breaking it does.
#[test]
fn bouncing_the_hole_gives_the_permanent_back_too() {
    let (mut game, hole, theirs) = staged(&[cards::SAVANNAH_LIONS]);
    cast_at(&mut game, hole, theirs[0]);
    let hole_permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::PORTABLE_HOLE)
        .expect("the Hole is on the battlefield")
        .card
        .id;

    game.return_permanent_to_hand(hole_permanent);
    settle(&mut game);

    assert!(
        on_battlefield(&game, cards::SAVANNAH_LIONS),
        "the Lions came back from a bounce as readily as from a break",
    );
    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|card| card.definition == cards::PORTABLE_HOLE),
        "and the Hole is in hand to be cast again",
    );
}
