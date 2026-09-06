//! "It gets +N/+N for each creature blocking it": the count is taken as the
//! trigger resolves, so a second blocker doubles the bonus rather than
//! firing the trigger twice.

use super::*;

/// Rabid Elephant attacking into `blockers` creatures, all blocking it.
fn staged(blockers: usize) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    let mut elephant = creature(28_000, cards::RABID_ELEPHANT, PlayerId::One);
    elephant.attacking = true;
    elephant.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let elephant_id = elephant.card.id;
    game.battlefield.push(elephant);
    for index in 0..blockers {
        let mut blocker = creature(
            28_100 + u32::try_from(index).expect("a small fixture"),
            cards::GRIZZLY_BEARS,
            PlayerId::Two,
        );
        blocker.entered_controller_turn = 0;
        blocker.blocking = vec![elephant_id];
        game.battlefield.push(blocker);
    }
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.finish_declaring_blockers();
    drain_pending(&mut game);
    (game, elephant_id)
}

fn stats(game: &Game, id: GameObjectId) -> (i16, i16) {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("the Elephant is on the battlefield");
    (
        game.power(permanent).expect("power"),
        game.toughness(permanent).expect("toughness"),
    )
}

#[test]
fn one_blocker_is_one_bonus() {
    let (game, elephant) = staged(1);
    assert_eq!(stats(&game, elephant), (5, 6), "3/4 plus one lot of +2/+2");
}

#[test]
fn two_blockers_double_the_bonus() {
    let (game, elephant) = staged(2);
    assert_eq!(stats(&game, elephant), (7, 8), "3/4 plus two lots of +2/+2");
}

#[test]
fn going_unblocked_leaves_it_alone() {
    let (game, elephant) = staged(0);
    assert_eq!(stats(&game, elephant), (3, 4), "the trigger never fired");
}
