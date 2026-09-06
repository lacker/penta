//! Two spells whose scope is easy to widen by accident. Morningtide exiles
//! every graveyard, its caster's included; Decompose exiles up to three
//! cards from *one* graveyard, so its offers must never mix the two. And
//! Dominate's target is bounded by the X it was cast for.

use super::*;

fn with_graveyards(mine: usize, theirs: usize) -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].graveyard.clear();
    game.players[1].graveyard.clear();
    for index in 0..mine {
        game.players[0].graveyard.push(card(
            71_000 + u32::try_from(index).expect("small"),
            cards::MOUNTAIN,
            PlayerId::One,
        ));
    }
    for index in 0..theirs {
        game.players[1].graveyard.push(card(
            71_100 + u32::try_from(index).expect("small"),
            cards::FOREST,
            PlayerId::Two,
        ));
    }
    game
}

#[test]
fn morningtide_exiles_the_casters_graveyard_too() {
    let mut game = with_graveyards(2, 2);
    let spell = card(71_200, cards::MORNINGTIDE, PlayerId::One);
    game.players[0].hand.push(spell.clone());
    game.players[0].mana_pool.white = 1;
    game.players[0].mana_pool.colorless = 1;
    let cast = cast_action(spell.id, Vec::new(), Vec::new(), 0);
    assert!(game.legal_actions(PlayerId::One).contains(&cast));
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(
        (
            game.players[0].graveyard.len(),
            game.players[1].graveyard.len()
        ),
        (1, 0),
        "both graveyards emptied, and only Morningtide itself is left behind"
    );
}

#[test]
fn dominate_reaches_only_as_far_as_x() {
    for (x, stealable) in [(1u16, false), (2, true)] {
        let mut game = ready_game();
        game.battlefield.clear();
        game.players[0].hand.clear();
        // Grizzly Bears costs {1}{G}, so mana value two.
        game.battlefield
            .push(creature(71_400, cards::GRIZZLY_BEARS, PlayerId::Two));
        let spell = card(71_410, cards::DOMINATE, PlayerId::One);
        let spell_id = spell.id;
        game.players[0].hand.push(spell);
        game.players[0].mana_pool.blue = 2;
        game.players[0].mana_pool.colorless = 4;

        let offered = game.legal_actions(PlayerId::One).into_iter().any(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if card == spell_id && choices.x() == x)
        });
        assert_eq!(
            offered, stealable,
            "a mana value two creature is reachable at X = {x} only when X is enough"
        );
    }
}
