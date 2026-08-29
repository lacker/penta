//! "Target player reveals their hand."
//!
//! The reveal is its own step. Nothing moves, and what follows reads the hand
//! afresh rather than the reveal's result -- so a hand with nothing to take
//! and nothing to count is still revealed, and the table still knows it.

use super::*;

/// Casts `definition` at player two with a stocked hand for them, and returns
/// what they were holding.
fn cast_at_hand(
    definition: CardDefinitionId,
    hand: &[CardDefinitionId],
) -> (Game, Vec<CardDefinitionId>) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    for (index, held) in hand.iter().enumerate() {
        let id = 20_000 + u32::try_from(index).expect("a small hand fits");
        game.players[PlayerId::Two.index()]
            .hand
            .push(card(id, *held, PlayerId::Two));
    }
    let spell = card(10_000, definition, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    let pool = &mut game.players[PlayerId::One.index()].mana_pool;
    pool.blue = 3;
    pool.black = 1;
    pool.colorless = 3;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == spell_id
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Player(PlayerId::Two))
            }
            _ => false,
        })
        .expect("the spell can be aimed at the other player");
    game.apply(PlayerId::One, action)
        .expect("the spell is cast");
    drain_pending(&mut game);
    (game, hand.to_vec())
}

fn revealed(game: &Game) -> Vec<CardDefinitionId> {
    game.events()
        .iter()
        .filter_map(|event| match event {
            GameEvent::CardRevealed {
                player, definition, ..
            } if *player == PlayerId::Two => Some(*definition),
            _ => None,
        })
        .collect()
}

fn hand(game: &Game) -> Vec<CardDefinitionId> {
    game.players[PlayerId::Two.index()]
        .hand
        .iter()
        .map(|card| card.definition)
        .collect()
}

#[test]
fn amnesia_reveals_the_hand_and_leaves_only_the_lands() {
    let (game, held) = cast_at_hand(
        cards::AMNESIA,
        &[
            cards::MOUNTAIN,
            cards::SEDGE_TROLL,
            cards::LIGHTNING_BOLT,
            cards::MOUNTAIN,
        ],
    );

    assert_eq!(revealed(&game), held, "every card was shown, in hand order");
    assert_eq!(hand(&game), vec![cards::MOUNTAIN, cards::MOUNTAIN]);
    assert_eq!(
        game.players[PlayerId::Two.index()].graveyard.len(),
        2,
        "the two nonlands were discarded"
    );
}

/// An all-land hand loses nothing, but is still revealed: the two halves of
/// the card are separate steps.
#[test]
fn a_hand_with_nothing_to_lose_is_still_revealed() {
    let (game, held) = cast_at_hand(cards::AMNESIA, &[cards::MOUNTAIN, cards::MOUNTAIN]);

    assert_eq!(revealed(&game), held);
    assert_eq!(hand(&game), held, "nothing was discarded");
}

#[test]
fn inquisition_counts_the_white_cards_it_revealed() {
    let (game, held) = cast_at_hand(
        cards::INQUISITION,
        &[
            cards::SAVANNAH_LIONS,
            cards::SEDGE_TROLL,
            cards::SERRA_ANGEL,
            cards::MOUNTAIN,
        ],
    );
    let life = i16::from(crate::rules::STARTING_LIFE);

    assert_eq!(revealed(&game), held);
    assert_eq!(
        game.players[PlayerId::Two.index()].life,
        life - 2,
        "two white creatures, and neither the red one nor the land"
    );
    assert_eq!(hand(&game), held, "nothing is taken, only counted");
}

/// No white cards is no damage, and the hand is revealed either way.
#[test]
fn a_hand_with_no_white_cards_costs_nothing() {
    let (game, held) = cast_at_hand(
        cards::INQUISITION,
        &[cards::SEDGE_TROLL, cards::LIGHTNING_BOLT],
    );

    assert_eq!(revealed(&game), held);
    assert_eq!(
        game.players[PlayerId::Two.index()].life,
        i16::from(crate::rules::STARTING_LIFE)
    );
}
