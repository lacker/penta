//! Unleash.
//!
//! One keyword doing two things: an optional counter as the permanent enters,
//! and no blocking while it carries one. The choice is the point -- a 1/1 that
//! can block and a 2/2 that cannot are both reasonable, so the entry has to
//! ask rather than assume.

use super::*;

fn enter(accept: bool) -> (Game, GameObjectId) {
    let mut game = ready_game();
    let cackler = card(10_000, cards::RAKDOS_CACKLER, PlayerId::One);
    let cackler_id = cackler.id;
    game.players[PlayerId::One.index()].hand.push(cackler);
    game.players[PlayerId::One.index()].mana_pool.black = 1;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == cackler_id))
        .expect("the Cackler is castable");
    game.apply(PlayerId::One, action)
        .expect("the spell is cast");
    pass_priority_pair(&mut game);

    let decision = game
        .pending_decisions
        .first()
        .expect("entering asks whether to take the counter");
    let option = decision
        .observation
        .options
        .iter()
        .find(|option| option.label == if accept { "Accept" } else { "Decline" })
        .expect("both answers are offered")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.observation.id,
            options: vec![option],
        },
    )
    .expect("the choice is submitted");
    drain_pending(&mut game);

    let entered = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::RAKDOS_CACKLER)
        .expect("the Cackler entered")
        .card
        .id;
    (game, entered)
}

fn can_block_for_owner(game: &Game, blocker: GameObjectId) -> bool {
    game.legal_actions(PlayerId::One).iter().any(
        |action| matches!(action, Action::DeclareBlocker { blocker: actual, .. } if *actual == blocker),
    )
}

fn attacked_by_opponent(game: &mut Game) {
    game.step = Step::DeclareBlockers;
    game.active_player = PlayerId::Two;
    game.attackers_declared = true;
    game.priority = PlayerId::One;
    let mut attacker = creature(11_000, cards::SEDGE_TROLL, PlayerId::Two);
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::One));
    game.battlefield.push(attacker);
}

#[test]
fn accepting_the_counter_makes_it_bigger() {
    let (game, cackler_id) = enter(true);
    let cackler = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == cackler_id)
        .expect("on the battlefield");
    assert_eq!(cackler.counters(CounterKind::PlusOnePlusOne), 1);
    assert_eq!(game.power(cackler), Some(2), "a 1/1 with a counter");
}

#[test]
fn declining_leaves_it_as_printed() {
    let (game, cackler_id) = enter(false);
    let cackler = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == cackler_id)
        .expect("on the battlefield");
    assert_eq!(cackler.counters(CounterKind::PlusOnePlusOne), 0);
    assert_eq!(game.power(cackler), Some(1));
}

/// The counter is what takes its blocking away, so the two answers are a
/// trade rather than one being strictly better.
#[test]
fn the_counter_is_what_stops_it_blocking() {
    let (mut unleashed, unleashed_id) = enter(true);
    attacked_by_opponent(&mut unleashed);
    assert!(
        !can_block_for_owner(&unleashed, unleashed_id),
        "it took the counter, so it cannot block"
    );

    let (mut restrained, restrained_id) = enter(false);
    attacked_by_opponent(&mut restrained);
    assert!(
        can_block_for_owner(&restrained, restrained_id),
        "and declining keeps its blocking"
    );
}
