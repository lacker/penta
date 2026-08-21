//! Jacked Rabbit: the counters are the body and the body is the token count.

use super::*;

/// Player One holding a Rabbit with the mana for X of `x`.
fn staged(x: u16) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let rabbit = game
        .build_zone(PlayerId::One, &[cards::JACKED_RABBIT])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let id = rabbit.id;
    game.players[0].hand.push(rabbit);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1 + x);
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, id)
}

fn resolve(game: &mut Game) {
    for _ in 0..16 {
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

/// Casts the Rabbit for `x` and lets it resolve.
fn cast(game: &mut Game, rabbit: GameObjectId, x: u16) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => *card == rabbit && choices.x() == x,
            _ => false,
        })
        .unwrap_or_else(|| panic!("a Rabbit for X={x} is castable"));
    game.apply(PlayerId::One, action).expect("it is castable");
    resolve(game);
}

fn on_battlefield(game: &Game) -> Option<&Permanent> {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::JACKED_RABBIT)
}

fn rabbits(game: &Game) -> usize {
    game.battlefield
        .iter()
        .filter(|permanent| {
            is_token_with(
                permanent,
                tokens::creature(&["Rabbit"], &[ManaColor::White], 1, 1),
            )
        })
        .count()
}

/// X counters arrive with it, on top of a printed 1/2.
#[test]
fn it_enters_with_x_counters() {
    let (mut game, rabbit) = staged(3);
    cast(&mut game, rabbit, 3);

    let permanent = on_battlefield(&game).expect("it resolved");
    assert_eq!(permanent.counters(CounterKind::PlusOnePlusOne), 3);
    assert_eq!(game.power(permanent), Some(4), "a 1/2 plus three");
    assert_eq!(game.toughness(permanent), Some(5));
}

/// Under five, ravenous draws nothing.
#[test]
fn a_small_x_draws_nothing() {
    let (mut game, rabbit) = staged(4);
    let before = game.players[0].hand.len();
    cast(&mut game, rabbit, 4);

    assert_eq!(
        game.players[0].hand.len(),
        before - 1,
        "the Rabbit left hand and nothing came back",
    );
}

/// Five is the boundary, and it draws.
#[test]
fn an_x_of_five_draws_a_card() {
    let (mut game, rabbit) = staged(5);
    let before = game.players[0].hand.len();
    cast(&mut game, rabbit, 5);

    assert_eq!(
        game.players[0].hand.len(),
        before,
        "one out and one back in",
    );
}

/// Attacking makes one Rabbit per point of power, counters included.
#[test]
fn attacking_makes_a_rabbit_for_each_point_of_power() {
    let (mut game, rabbit) = staged(2);
    cast(&mut game, rabbit, 2);
    let attacker = on_battlefield(&game).expect("it resolved").card.id;
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == attacker)
    {
        permanent.entered_controller_turn = 0;
    }
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;

    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker,
            defender: AttackDefender::Player(PlayerId::Two),
        },
    )
    .expect("a 3/4 may attack");
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .expect("the declaration is complete");
    resolve(&mut game);

    assert_eq!(rabbits(&game), 3, "power three, three Rabbits");
}

/// The tokens are made, not declared: they arrive after attackers.
#[test]
fn the_tokens_do_not_attack() {
    let (mut game, rabbit) = staged(1);
    cast(&mut game, rabbit, 1);
    let attacker = on_battlefield(&game).expect("it resolved").card.id;
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == attacker)
    {
        permanent.entered_controller_turn = 0;
    }
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;

    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker,
            defender: AttackDefender::Player(PlayerId::Two),
        },
    )
    .expect("a 2/3 may attack");
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .expect("the declaration is complete");
    resolve(&mut game);

    assert_eq!(rabbits(&game), 2);
    assert!(
        game.battlefield
            .iter()
            .filter(|permanent| is_token_with(
                permanent,
                tokens::creature(&["Rabbit"], &[ManaColor::White], 1, 1)
            ))
            .all(|permanent| !permanent.attacking),
        "the declaration was over before they existed",
    );
}
