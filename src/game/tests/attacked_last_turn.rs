//! "If it attacked during your last turn."
//!
//! History rather than turn state: the answer has to survive the cleanup that
//! clears `attacked_this_turn`, and it has to go stale one turn later. The
//! condition sits on the recipient of each static, so the prohibition is read
//! live -- nothing is installed when the creature swings and nothing has to
//! expire when it stops being true.

use super::*;

/// Puts `definition` under player one, several turns into the game.
fn ready_with(definition: CardDefinitionId) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    let permanent = creature(10_000, definition, PlayerId::One);
    let id = permanent.card.id;
    game.battlefield.push(permanent);
    (game, id)
}

fn swing(game: &mut Game, attacker: GameObjectId) {
    game.step = Step::DeclareAttackers;
    game.declare_attacker(attacker, AttackDefender::Player(PlayerId::Two));
    game.clear_combat();
}

/// Hands the turn to player two and back, which is what makes the last swing
/// belong to "your last turn".
fn next_own_turn(game: &mut Game) {
    game.turns_started[PlayerId::Two.index()] += 1;
    game.turns_started[PlayerId::One.index()] += 1;
}

fn permanent(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("the permanent is on the battlefield")
}

fn untaps(game: &Game, id: GameObjectId) -> bool {
    !game.does_not_untap_during_untap_step(permanent(game, id))
}

#[test]
fn the_turtle_swings_every_other_turn() {
    let (mut game, turtle) = ready_with(cards::GIANT_TURTLE);
    assert!(
        game.can_attack(permanent(&game, turtle)),
        "nothing behind it"
    );

    swing(&mut game, turtle);
    next_own_turn(&mut game);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == turtle)
        .expect("the Turtle is there")
        .tapped = false;
    assert!(
        !game.can_attack(permanent(&game, turtle)),
        "it attacked during the last turn"
    );

    next_own_turn(&mut game);
    assert!(
        game.can_attack(permanent(&game, turtle)),
        "and the turn after that it is rested"
    );
}

/// The answer has to outlive the flag that cleanup clears, which is the whole
/// reason the turn it attacked on is recorded separately.
#[test]
fn the_answer_does_not_depend_on_this_turns_flag() {
    let (mut game, turtle) = ready_with(cards::GIANT_TURTLE);
    swing(&mut game, turtle);
    next_own_turn(&mut game);
    let resting = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == turtle)
        .expect("the Turtle is there");
    resting.tapped = false;
    resting.attacked_this_turn = false;

    assert!(!game.can_attack(permanent(&game, turtle)));
}

#[test]
fn the_rock_sled_stays_tapped_the_turn_after_it_swings() {
    let (mut game, sled) = ready_with(cards::GOBLIN_ROCK_SLED);
    assert!(untaps(&game, sled), "an idle Sled untaps normally");

    swing(&mut game, sled);
    next_own_turn(&mut game);
    assert!(!untaps(&game, sled), "it is still recovering");

    next_own_turn(&mut game);
    assert!(untaps(&game, sled), "and then it untaps again");
}

/// The Kelp holds down whatever it is attached to, read the same way -- and
/// only that creature.
#[test]
fn the_kelp_holds_down_a_host_that_attacked() {
    let (mut game, host) = ready_with(cards::SEDGE_TROLL);
    let mut kelp = creature(10_001, cards::TANGLE_KELP, PlayerId::One);
    kelp.attached_to = Some(host);
    game.battlefield.push(kelp);
    let free = creature(10_002, cards::SEDGE_TROLL, PlayerId::One);
    let free_id = free.card.id;
    game.battlefield.push(free);

    assert!(untaps(&game, host), "an idle host untaps");

    swing(&mut game, host);
    swing(&mut game, free_id);
    next_own_turn(&mut game);

    assert!(!untaps(&game, host), "the Kelp keeps it down");
    assert!(
        untaps(&game, free_id),
        "and the creature it is not on is untouched"
    );
}

/// The Sled's other clause, which the same board makes easy to check: without
/// a Mountain across the table it cannot attack at all.
#[test]
fn the_sled_needs_a_mountain_to_attack() {
    let (mut game, sled) = ready_with(cards::GOBLIN_ROCK_SLED);
    assert!(!game.can_attack(permanent(&game, sled)));

    game.battlefield
        .push(creature(10_001, cards::MOUNTAIN, PlayerId::Two));
    assert!(game.can_attack(permanent(&game, sled)));
}
