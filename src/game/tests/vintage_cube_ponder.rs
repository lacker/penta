//! Ponder: three cards arranged how you like, or washed away, and then a
//! card off whatever is on top.

use super::*;

/// Player One with a Ponder in hand and a library whose top three are known.
/// The library's last element is its top, so the Angel pushed first is the
/// bottom card and never gets looked at.
fn staged() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].library.clear();
    for definition in [
        cards::SERRA_ANGEL,
        cards::GRIZZLY_BEARS,
        cards::LIGHTNING_BOLT,
        cards::MOUNTAIN,
    ] {
        let card = game
            .build_zone(PlayerId::One, &[definition])
            .expect("cataloged")
            .into_iter()
            .next()
            .expect("one card");
        game.players[0].library.push(card);
    }
    let ponder = game
        .build_zone(PlayerId::One, &[cards::PONDER])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let id = ponder.id;
    game.players[0].hand.push(ponder);
    game.players[0].hand.retain(|card| card.id == id);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    game.priority = PlayerId::One;
    (game, id)
}

/// Casts the Ponder, ordering the three cards with `first_on_top` named
/// first when the arrangement is asked for, and shuffling when `shuffle`.
fn ponder(game: &mut Game, spell: GameObjectId, first: CardDefinitionId, shuffle: bool) {
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
        .expect("one blue buys a Ponder");
    game.apply(PlayerId::One, cast).expect("it is castable");

    for _ in 0..16 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = if decision.options.iter().any(|option| option.card.is_some()) {
                // The arrangement: name the wanted card first, then the rest.
                let mut ids = decision
                    .options
                    .iter()
                    .filter(|option| {
                        option.card.is_some_and(|(_, characteristics)| {
                            characteristics.card_definition() == Some(first)
                        })
                    })
                    .map(|option| option.id)
                    .collect::<Vec<_>>();
                ids.extend(
                    decision
                        .options
                        .iter()
                        .filter(|option| {
                            option.card.is_none_or(|(_, characteristics)| {
                                characteristics.card_definition() != Some(first)
                            })
                        })
                        .map(|option| option.id),
                );
                ids.truncate(decision.maximum);
                ids
            } else {
                let wanted = if shuffle { "Do it" } else { "Decline" };
                decision
                    .options
                    .iter()
                    .find(|option| option.label == wanted)
                    .map(|option| vec![option.id])
                    .unwrap_or_default()
            };
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .expect("the decision accepts what it offered");
            continue;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            return;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            return;
        }
    }
}

fn drawn(game: &Game) -> Vec<CardDefinitionId> {
    game.players[0]
        .hand
        .iter()
        .map(|card| card.definition)
        .collect()
}

/// The card named first is the one drawn.
#[test]
fn the_card_you_put_on_top_is_the_one_you_draw() {
    // The deepest of the three, so drawing it means the arrangement moved it
    // rather than that it was already there.
    let (mut game, spell) = staged();
    ponder(&mut game, spell, cards::GRIZZLY_BEARS, false);

    assert_eq!(drawn(&game), vec![cards::GRIZZLY_BEARS]);
}

/// And a different arrangement draws a different card, which is what makes
/// the ordering an ordering.
#[test]
fn a_different_order_draws_a_different_card() {
    let (mut game, spell) = staged();
    ponder(&mut game, spell, cards::LIGHTNING_BOLT, false);

    assert_eq!(drawn(&game), vec![cards::LIGHTNING_BOLT]);
}

/// The fourth card was never looked at and stays where it was.
#[test]
fn only_three_cards_are_looked_at() {
    let (mut game, spell) = staged();
    ponder(&mut game, spell, cards::GRIZZLY_BEARS, false);

    assert_eq!(
        game.players[0].library.first().map(|card| card.definition),
        Some(cards::SERRA_ANGEL),
        "the bottom card never moved",
    );
    assert_eq!(game.players[0].library.len(), 3, "four less the one drawn");
}

/// Shuffling throws the arrangement away, and the draw still happens.
#[test]
fn shuffling_still_draws() {
    let (mut game, spell) = staged();
    ponder(&mut game, spell, cards::GRIZZLY_BEARS, true);

    assert_eq!(game.players[0].hand.len(), 1, "a card either way");
    assert_eq!(game.players[0].library.len(), 3);
}
