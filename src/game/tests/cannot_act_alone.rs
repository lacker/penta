//! "This creature can't attack or block alone." Like the minimum-blocker
//! rule this constrains the finished declaration rather than any one attack:
//! declaring the creature is always legal, and it is ending the declaration
//! with nothing beside it that is not. So the symptom is not a missing
//! `DeclareAttacker` action but a missing `FinishDeclaringAttackers` action.

use super::*;

/// Player one in declare-attackers with Mogg Flunkies and `friends` Bears.
fn attacking_side(friends: usize) -> (Game, GameObjectId, Vec<GameObjectId>) {
    let mut game = ready_game();
    game.battlefield.clear();
    let flunkies = creature(48_000, cards::MOGG_FLUNKIES, PlayerId::One);
    let flunkies_id = flunkies.card.id;
    game.battlefield.push(flunkies);
    let mut friend_ids = Vec::new();
    for index in 0..friends {
        let bear = creature(
            48_100 + u32::try_from(index).expect("a small fixture"),
            cards::GRIZZLY_BEARS,
            PlayerId::One,
        );
        friend_ids.push(bear.card.id);
        game.battlefield.push(bear);
    }
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [2, 1];
    game.active_player = PlayerId::One;
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.priority = PlayerId::One;
    (game, flunkies_id, friend_ids)
}

fn declare(game: &mut Game, attacker: GameObjectId) {
    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker,
            defender: AttackDefender::Player(PlayerId::Two),
        },
    )
    .expect("declaring it is always legal");
}

fn can_finish_attacking(game: &Game) -> bool {
    game.legal_actions(PlayerId::One)
        .contains(&Action::FinishDeclaringAttackers)
}

#[test]
fn declaring_it_alone_is_legal_but_finishing_is_not() {
    let (mut game, flunkies, _) = attacking_side(1);
    declare(&mut game, flunkies);
    assert!(
        !can_finish_attacking(&game),
        "the attack cannot be locked in with only the Flunkies in it"
    );
}

#[test]
fn a_second_attacker_lets_the_declaration_close() {
    let (mut game, flunkies, friends) = attacking_side(1);
    declare(&mut game, flunkies);
    declare(&mut game, friends[0]);
    assert!(can_finish_attacking(&game));
}

#[test]
fn leaving_it_at_home_is_always_fine() {
    let (mut game, _, friends) = attacking_side(1);
    declare(&mut game, friends[0]);
    assert!(
        can_finish_attacking(&game),
        "the rule only speaks about declarations the creature is in"
    );
}

#[test]
fn it_cannot_be_the_only_blocker_in_the_combat() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    let mut first = creature(48_200, cards::GRIZZLY_BEARS, PlayerId::One);
    first.attacking = true;
    first.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let first_id = first.card.id;
    game.battlefield.push(first);
    let mut second = creature(48_201, cards::GRIZZLY_BEARS, PlayerId::One);
    second.attacking = true;
    second.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let second_id = second.card.id;
    game.battlefield.push(second);
    let mut flunkies = creature(48_202, cards::MOGG_FLUNKIES, PlayerId::Two);
    flunkies.entered_controller_turn = 0;
    let flunkies_id = flunkies.card.id;
    game.battlefield.push(flunkies);
    let mut bear = creature(48_203, cards::GRIZZLY_BEARS, PlayerId::Two);
    bear.entered_controller_turn = 0;
    let bear_id = bear.card.id;
    game.battlefield.push(bear);
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;

    let block = |game: &mut Game, blocker, attacker| {
        game.apply(PlayerId::Two, Action::DeclareBlocker { blocker, attacker })
            .expect("declaring the block is legal");
    };
    let can_finish_blocking = |game: &Game| {
        game.legal_actions(PlayerId::Two)
            .contains(&Action::FinishDeclaringBlockers)
    };

    block(&mut game, flunkies_id, first_id);
    assert!(
        !can_finish_blocking(&game),
        "one blocker in the whole combat is blocking alone"
    );

    // A second blocker on the *other* attacker is still company: the clause
    // counts the declaration, not the pairing.
    block(&mut game, bear_id, second_id);
    assert!(can_finish_blocking(&game));
}
