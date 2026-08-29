//! An upkeep tax that grows, paid in one colour and counted at resolution.
//!
//! The counter goes on before the payment is asked for, so the very first
//! upkeep already costs {G} rather than nothing. What it buys is symmetric:
//! the damage hits every creature and every player, its controller included.

use super::*;

/// Cyclone under player one with `wind` counters already banked, `green`
/// green mana available, and a creature apiece.
fn blowing(wind: u16, green: u16) -> (Game, GameObjectId, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;

    let mut cyclone = creature(10_000, cards::CYCLONE, PlayerId::One);
    cyclone.set_counters(CounterKind::named("wind"), wind);
    let cyclone_id = cyclone.card.id;
    game.battlefield.push(cyclone);

    let mine = creature(10_100, cards::SEDGE_TROLL, PlayerId::One);
    let mine_id = mine.card.id;
    game.battlefield.push(mine);
    let theirs = creature(10_101, cards::SEDGE_TROLL, PlayerId::Two);
    let theirs_id = theirs.card.id;
    game.battlefield.push(theirs);

    game.players[PlayerId::One.index()].mana_pool.green = green;
    game.priority = PlayerId::One;
    (game, cyclone_id, mine_id, theirs_id)
}

/// Answers each waiting decision by taking the option at `index`, clamped to
/// what is on offer.
fn drain_choosing(game: &mut Game, index: usize) {
    for _ in 0..16 {
        if game.stack.is_empty()
            && game.pending_triggers.is_empty()
            && game.pending_decisions.is_empty()
        {
            return;
        }
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let pick = index.min(decision.options.len().saturating_sub(1));
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options: vec![decision.options[pick].id],
                },
            )
            .expect("the decision accepts what it offered");
            continue;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            return;
        }
    }
}

fn run_upkeep(game: &mut Game, choice: usize) {
    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    drain_choosing(game, choice);
}

fn still_there(game: &Game, id: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .any(|permanent| permanent.card.id == id)
}

fn wind(game: &Game, cyclone: GameObjectId) -> u16 {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == cyclone)
        .expect("still there")
        .counters(CounterKind::named("wind"))
}

/// The counter lands before the bill arrives, so a fresh Cyclone is already
/// asking for one green rather than nothing.
#[test]
fn the_first_upkeep_already_costs_one_green() {
    let (mut game, cyclone, _mine, _theirs) = blowing(0, 1);
    run_upkeep(&mut game, usize::MAX);

    assert!(still_there(&game, cyclone), "one green covered it");
    assert_eq!(wind(&game, cyclone), 1);
    assert_eq!(game.players[PlayerId::One.index()].mana_pool.green, 0);
}

/// The control: not enough green, so the payment is not on offer and the
/// enchantment goes.
#[test]
fn an_unaffordable_upkeep_sacrifices_it() {
    let (mut game, cyclone, _mine, _theirs) = blowing(2, 2);
    run_upkeep(&mut game, usize::MAX);

    assert!(
        !still_there(&game, cyclone),
        "three counters wanted three green",
    );
}

/// Declining is a real choice even when the mana is there.
#[test]
fn declining_sacrifices_it_too() {
    let (mut game, cyclone, _mine, _theirs) = blowing(0, 5);
    run_upkeep(&mut game, 0);

    assert!(!still_there(&game, cyclone));
    assert_eq!(
        game.players[PlayerId::One.index()].mana_pool.green,
        5,
        "and nothing was spent",
    );
}

/// Paying sweeps the board symmetrically: both creatures and both players.
#[test]
fn paying_deals_the_counter_count_to_everything() {
    let (mut game, cyclone, mine, theirs) = blowing(2, 3);
    run_upkeep(&mut game, usize::MAX);

    assert!(still_there(&game, cyclone));
    assert_eq!(wind(&game, cyclone), 3);
    assert!(!still_there(&game, mine), "three killed my own Troll");
    assert!(!still_there(&game, theirs), "and theirs");
    for player in [PlayerId::One, PlayerId::Two] {
        assert_eq!(
            game.players[player.index()].life,
            i16::from(rules::STARTING_LIFE) - 3,
            "{player:?} took the same three",
        );
    }
}

/// The cost is green specifically, so other colours do not pay it.
#[test]
fn other_colours_do_not_pay_for_it() {
    let (mut game, cyclone, _mine, _theirs) = blowing(0, 0);
    game.players[PlayerId::One.index()].mana_pool.red = 5;
    run_upkeep(&mut game, usize::MAX);

    assert!(!still_there(&game, cyclone), "red is not green");
}
