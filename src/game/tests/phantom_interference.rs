//! Spree's first card in the catalog. Each chosen mode adds its own mana to
//! the cost, so which casts are legal is the thing worth pinning: one mode,
//! the other, or both, and never none.

use super::*;

/// Player One holding Phantom Interference with `mana` colorless available
/// beside the blue, and an opponent's spell on the stack to answer.
fn staged(colorless: u16) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.stack
        .push(spell(23_000, cards::LIGHTNING_BOLT, PlayerId::Two, 0));
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    for _ in 0..colorless {
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    }
    let interference = card(23_001, cards::PHANTOM_INTERFERENCE, PlayerId::One);
    let id = interference.id;
    game.players[0].hand.push(interference);
    (game, id)
}

/// How many distinct mode selections Phantom Interference can be cast with
/// on this board.
fn castable_mode_counts(game: &Game, card: GameObjectId) -> Vec<usize> {
    let mut counts = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell {
                card: candidate,
                choices,
                ..
            } if candidate == card => Some(choices.modes().len()),
            _ => None,
        })
        .collect::<Vec<_>>();
    counts.sort_unstable();
    counts.dedup();
    counts
}

#[test]
fn two_mana_buys_only_the_cheaper_mode() {
    let (game, interference) = staged(1);
    assert_eq!(
        castable_mode_counts(&game, interference),
        vec![1],
        "one blue plus one generic pays for the cheaper mode alone"
    );
}

#[test]
fn five_mana_buys_either_mode_or_both_together() {
    let (game, interference) = staged(4);
    assert_eq!(
        castable_mode_counts(&game, interference),
        vec![1, 2],
        "with everything available both single modes and the pair are offered"
    );
}

#[test]
fn one_mana_cannot_cast_it_at_all() {
    let (game, interference) = staged(0);
    assert!(
        castable_mode_counts(&game, interference).is_empty(),
        "spree has no free mode, so one blue mana never casts it"
    );
}
