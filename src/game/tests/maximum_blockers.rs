//! A cap on how many creatures may block one attacker. Unlike the minimum,
//! which is checked when the declaration ends, a maximum is checked as each
//! block is offered: the blocker that would exceed it is illegal on its own,
//! so it never appears in the legal-action list at all.

use super::*;

/// Stalking Tiger attacking, with `defenders` untapped creatures opposite.
fn combat(defenders: usize) -> (Game, GameObjectId, Vec<GameObjectId>) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    let mut tiger = creature(38_000, cards::STALKING_TIGER, PlayerId::One);
    tiger.attacking = true;
    tiger.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let tiger_id = tiger.card.id;
    game.battlefield.push(tiger);
    let mut ids = Vec::new();
    for index in 0..defenders {
        let mut bear = creature(
            38_100 + u32::try_from(index).expect("a small fixture"),
            cards::GRIZZLY_BEARS,
            PlayerId::Two,
        );
        bear.entered_controller_turn = 0;
        ids.push(bear.card.id);
        game.battlefield.push(bear);
    }
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    (game, tiger_id, ids)
}

fn blockers_offered(game: &Game, attacker: GameObjectId) -> Vec<GameObjectId> {
    game.legal_actions(PlayerId::Two)
        .into_iter()
        .filter_map(|action| match action {
            Action::DeclareBlocker {
                blocker,
                attacker: a,
            } if a == attacker => Some(blocker),
            _ => None,
        })
        .collect()
}

#[test]
fn the_first_blocker_is_offered_as_usual() {
    let (game, tiger, bears) = combat(2);
    assert_eq!(
        blockers_offered(&game, tiger),
        bears,
        "with nobody blocking yet the cap rules out nothing"
    );
}

#[test]
fn the_second_blocker_is_never_offered() {
    let (mut game, tiger, bears) = combat(2);
    game.apply(
        PlayerId::Two,
        Action::DeclareBlocker {
            blocker: bears[0],
            attacker: tiger,
        },
    )
    .expect("one creature may block it");

    assert!(
        blockers_offered(&game, tiger).is_empty(),
        "the cap is full, so the second block is illegal rather than merely unwise"
    );
}

#[test]
fn the_cap_belongs_to_the_attacker_that_prints_it() {
    let (mut game, tiger, bears) = combat(2);
    let mut bear = creature(38_200, cards::GRIZZLY_BEARS, PlayerId::One);
    bear.attacking = true;
    bear.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let uncapped = bear.card.id;
    game.battlefield.push(bear);
    game.apply(
        PlayerId::Two,
        Action::DeclareBlocker {
            blocker: bears[0],
            attacker: tiger,
        },
    )
    .expect("one creature may block the Tiger");

    assert_eq!(
        blockers_offered(&game, uncapped),
        vec![bears[1]],
        "the other attacker takes as many blockers as ever"
    );
}
