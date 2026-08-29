//! Two cards whose audit lines named machinery built for other cards.
//!
//! Builder's Blessing wants a recipient narrowed to untapped creatures, which
//! Arcades Sabboth already needed, and Eternal Flame wants a halved count
//! rounded up, which Aspect of Wolf already needed.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.battlefield.clear();
    game
}

fn stats(game: &Game, id: GameObjectId) -> (Option<i16>, Option<i16>) {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still there");
    (game.power(permanent), game.toughness(permanent))
}

/// The toughness comes and goes with the tap, and only reaches your own.
#[test]
fn the_blessing_covers_untapped_creatures_you_control() {
    let mut game = ready();
    game.battlefield
        .push(creature(10_000, cards::BUILDERS_BLESSING, PlayerId::One));
    let mine = creature(10_100, cards::GRIZZLY_BEARS, PlayerId::One);
    let mine_id = mine.card.id;
    game.battlefield.push(mine);
    let theirs = creature(10_101, cards::GRIZZLY_BEARS, PlayerId::Two);
    let theirs_id = theirs.card.id;
    game.battlefield.push(theirs);

    assert_eq!(stats(&game, mine_id), (Some(2), Some(4)), "a 2/2 plus 0/2");
    assert_eq!(
        stats(&game, theirs_id),
        (Some(2), Some(2)),
        "creatures you control, so not theirs",
    );

    let index = game
        .battlefield
        .iter()
        .position(|permanent| permanent.card.id == mine_id)
        .expect("still there");
    game.battlefield[index].tapped = true;
    assert_eq!(
        stats(&game, mine_id),
        (Some(2), Some(2)),
        "tapping gives it straight back",
    );
}

/// Casts Eternal Flame with the given number of Mountains and reports both
/// life totals afterwards.
fn flame_with(mountains: usize) -> [i16; 2] {
    let mut game = ready();
    for _ in 0..mountains {
        game.put_onto_battlefield(PlayerId::One, cards::MOUNTAIN)
            .expect("cataloged");
    }
    let spell = card(20_000, cards::ETERNAL_FLAME, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.red = 2;
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;

    game.apply(
        PlayerId::One,
        cast_action(spell_id, vec![Target::Player(PlayerId::Two)], Vec::new(), 0),
    )
    .expect("the cast is legal");
    drain_pending(&mut game);
    [game.players[0].life, game.players[1].life]
}

/// Rounded up, so an odd Mountain count costs the extra point rather than
/// saving it.
#[test]
fn the_flame_rounds_its_recoil_up() {
    assert_eq!(flame_with(1), [19, 19], "one and one");
    assert_eq!(flame_with(2), [19, 18], "two and one");
    assert_eq!(flame_with(3), [18, 17], "three and two, rounded up");
    assert_eq!(flame_with(4), [18, 16]);
}

/// The Flame cannot be aimed at yourself: "target opponent".
#[test]
fn the_flame_only_aims_at_an_opponent() {
    let mut game = ready();
    game.put_onto_battlefield(PlayerId::One, cards::MOUNTAIN)
        .expect("cataloged");
    let spell = card(20_000, cards::ETERNAL_FLAME, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.red = 2;
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;

    let legal = game.legal_actions(PlayerId::One);
    assert!(legal.contains(&cast_action(
        spell_id,
        vec![Target::Player(PlayerId::Two)],
        Vec::new(),
        0,
    )));
    assert!(
        !legal.contains(&cast_action(
            spell_id,
            vec![Target::Player(PlayerId::One)],
            Vec::new(),
            0,
        )),
        "you take the recoil, not the whole thing",
    );
}
