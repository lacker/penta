//! Spells whose second line is "Draw a card." The rider is a later step in
//! one sequence, so what needs covering is that it still runs after a first
//! step that waits on a decision -- a sacrifice of choice stops the
//! resolution mid-way, and a step dropped there would be invisible.

use super::*;

/// Dredge in hand with `lands` Forests of mine on the battlefield.
fn dredging(lands: usize) -> (Game, CardInstanceId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    for index in 0..lands {
        let mut forest = creature(
            82_000 + u32::try_from(index).expect("a small fixture"),
            cards::FOREST,
            PlayerId::One,
        );
        forest.entered_controller_turn = 0;
        game.battlefield.push(forest);
    }
    let dredge = card(82_100, cards::DREDGE, PlayerId::One);
    let dredge_id = dredge.id;
    game.players[0].hand.push(dredge);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);
    (game, dredge_id)
}

fn resolve(game: &mut Game) {
    for _ in 0..12 {
        drain_pending(game);
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let holder = game.priority;
        if game.apply(holder, Action::PassPriority).is_err() {
            break;
        }
    }
}

#[test]
fn the_draw_survives_the_sacrifice_decision() {
    let (mut game, dredge) = dredging(2);
    let library_before = game.players[0].library.len();
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == dredge))
        .expect("one black mana pays for it");
    game.apply(PlayerId::One, cast).expect("the cast is legal");
    resolve(&mut game);

    assert_eq!(
        game.battlefield.len(),
        1,
        "one of the two Forests was sacrificed"
    );
    assert_eq!(
        game.players[0].library.len(),
        library_before - 1,
        "and the rider still drew a card afterwards"
    );
}

/// Execute in hand with `board` creatures under player two.
fn execute(board: &[CardDefinitionId]) -> (Game, CardInstanceId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    for (index, definition) in board.iter().enumerate() {
        let mut permanent = creature(
            82_200 + u32::try_from(index).expect("a small fixture"),
            *definition,
            PlayerId::Two,
        );
        permanent.entered_controller_turn = 0;
        game.battlefield.push(permanent);
    }
    let execute = card(82_300, cards::EXECUTE, PlayerId::One);
    let execute_id = execute.id;
    game.players[0].hand.push(execute);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
    (game, execute_id)
}

fn castable(game: &Game, spell: CardInstanceId) -> bool {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .any(|action| matches!(action, Action::CastSpell { card, .. } if card == spell))
}

#[test]
fn execute_only_names_a_white_creature() {
    let (game, spell) = execute(&[cards::GRIZZLY_BEARS]);
    assert!(
        !castable(&game, spell),
        "a green Bears is not a legal target, so there is nothing to cast at"
    );

    let (game, spell) = execute(&[cards::SERRA_ANGEL]);
    assert!(
        castable(&game, spell),
        "a white Angel is, and the card is drawn as part of the same spell"
    );
}

#[test]
fn execute_kills_the_angel_and_replaces_itself() {
    let (mut game, spell) = execute(&[cards::SERRA_ANGEL]);
    let library_before = game.players[0].library.len();
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
        .expect("the Angel is a legal target");
    game.apply(PlayerId::One, cast).expect("the cast is legal");
    resolve(&mut game);

    assert!(game.battlefield.is_empty(), "the Angel was destroyed");
    assert_eq!(
        game.players[0].library.len(),
        library_before - 1,
        "and the spell replaced itself"
    );
}
