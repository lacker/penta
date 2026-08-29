//! Sleep counters that sit on the creature rather than on the Aura.
//!
//! Two things differ from Cocoon. The counters are on the host, so the static
//! asks what the Aura is attached to rather than what it carries; and there
//! are X of them, which the enters trigger cannot ask the spell for -- it is
//! a separate object -- so it reads the X off the permanent instead.

use super::*;

/// Venarian Gold cast for X on a Sedge Troll player two controls.
fn gilded(x: u16) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    game.turns_started[PlayerId::Two.index()] = 5;
    game.active_player = PlayerId::One;
    let host = creature(10_000, cards::SEDGE_TROLL, PlayerId::Two);
    let host_id = host.card.id;
    game.battlefield.push(host);

    let spell = card(20_000, cards::VENARIAN_GOLD, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.blue = 2;
    game.players[PlayerId::One.index()].mana_pool.colorless = x;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => *card == spell_id && choices.x() == x,
            _ => false,
        })
        .expect("the Troll is a legal host at this X");
    game.apply(PlayerId::One, action)
        .expect("the cast is legal");
    drain_pending(&mut game);
    (game, host_id)
}

fn host_of(game: &Game, host: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == host)
        .expect("still there")
}

/// Runs `player`'s upkeep.
fn take_an_upkeep(game: &mut Game, player: PlayerId) {
    game.commit_next_turn(player, Vec::new());
    drain_pending(game);
}

#[test]
fn it_taps_the_creature_and_puts_x_counters_on_it() {
    let (game, host) = gilded(2);

    let host = host_of(&game, host);
    assert!(host.tapped);
    assert_eq!(
        host.counters(CounterKind::named("sleep")),
        2,
        "the counters are on the creature, and there are X of them",
    );
}

/// A different X puts a different number on, which is what proves the enters
/// trigger is reading the cast rather than a constant.
#[test]
fn a_bigger_x_holds_it_longer() {
    let (game, host) = gilded(4);

    assert_eq!(
        host_of(&game, host).counters(CounterKind::named("sleep")),
        4
    );
}

/// The countdown runs on the creature's controller's upkeep, which is the
/// opponent's -- the Aura controller taking a turn does nothing.
#[test]
fn the_hosts_controller_is_the_one_who_counts_down() {
    let (mut game, host) = gilded(2);

    take_an_upkeep(&mut game, PlayerId::One);
    assert_eq!(
        host_of(&game, host).counters(CounterKind::named("sleep")),
        2,
        "the Aura controller's upkeep is not the one that counts",
    );

    take_an_upkeep(&mut game, PlayerId::Two);
    assert_eq!(
        host_of(&game, host).counters(CounterKind::named("sleep")),
        1
    );
    assert!(
        host_of(&game, host).tapped,
        "and a counter still remains, so it stays down",
    );
}

/// Once the last counter goes the creature untaps normally again.
#[test]
fn it_wakes_up_when_the_counters_run_out() {
    let (mut game, host) = gilded(1);

    take_an_upkeep(&mut game, PlayerId::Two);
    assert_eq!(
        host_of(&game, host).counters(CounterKind::named("sleep")),
        0
    );

    take_an_upkeep(&mut game, PlayerId::Two);
    assert!(!host_of(&game, host).tapped, "nothing is holding it now");
}
