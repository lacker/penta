//! "One or more target creatures", with no printed ceiling.
//!
//! The count is bounded by the board rather than by a number: a declaration
//! naming every creature there is is as legal as one naming a single
//! creature, and naming none is not.

use super::*;

/// A Glyph in hand and `creatures` creatures to point it at.
fn board(spell: CardDefinitionId, creatures: usize) -> (Game, GameObjectId, Vec<GameObjectId>) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    let mut ids = Vec::new();
    for index in 0..creatures {
        let id = 9_000 + u32::try_from(index).expect("a short list fits");
        let creature = creature(id, cards::SEDGE_TROLL, PlayerId::One);
        ids.push(creature.card.id);
        game.battlefield.push(creature);
    }
    let card = card(10_000, spell, PlayerId::One);
    let card_id = card.id;
    game.players[PlayerId::One.index()].hand.push(card);
    let pool = &mut game.players[PlayerId::One.index()].mana_pool;
    pool.white = 1;
    pool.blue = 1;
    pool.black = 1;
    pool.red = 1;
    pool.green = 1;
    (game, card_id, ids)
}

/// Every distinct target list the Glyph could be cast with.
fn declarations(game: &Game, spell: GameObjectId) -> Vec<Vec<Target>> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, choices, .. } if card == spell => {
                Some(choices.iter_targets().copied().collect())
            }
            _ => None,
        })
        .collect()
}

#[test]
fn the_count_is_bounded_by_the_board() {
    let (game, glyph, creatures) = board(cards::HEAVENS_GATE, 3);
    let sizes: Vec<_> = declarations(&game, glyph)
        .iter()
        .map(Vec::len)
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect();

    assert_eq!(sizes, vec![1, 2, 3], "one, two, or all three -- never none");
    assert_eq!(creatures.len(), 3);
}

/// Every subset of the right size, not just one of each size.
#[test]
fn every_combination_is_offered() {
    let (game, glyph, _) = board(cards::HEAVENS_GATE, 3);
    // Three singles, three pairs, one triple.
    assert_eq!(declarations(&game, glyph).len(), 7);
}

/// With nothing to point at, the spell is not castable at all.
#[test]
fn an_empty_board_offers_nothing() {
    let (game, glyph, _) = board(cards::HEAVENS_GATE, 0);
    assert!(declarations(&game, glyph).is_empty());
}

/// The colour actually lands, and on every creature named.
#[test]
fn the_whole_declaration_changes_colour() {
    let (mut game, glyph, creatures) = board(cards::SYLVAN_PARADISE, 2);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == glyph && choices.iter_targets().count() == 2
            }
            _ => false,
        })
        .expect("both creatures can be named at once");
    game.apply(PlayerId::One, action)
        .expect("the Glyph is cast");
    drain_pending(&mut game);

    for id in creatures {
        assert_eq!(
            game.object_colors(id),
            [false, false, false, false, true],
            "green and nothing else"
        );
    }
}
