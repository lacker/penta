//! Occult Epiphany: the draw is a wash and the Spirits are the card, one for
//! every card type the discard turned up.

use super::*;

/// Player One holding an Epiphany plus `hand`, with mana for X of `x`.
fn staged(x: u16, hand: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    for definition in hand {
        let card = game
            .build_zone(PlayerId::One, &[*definition])
            .expect("cataloged")
            .into_iter()
            .next()
            .expect("one card");
        game.players[0].hand.push(card);
    }
    let epiphany = game
        .build_zone(PlayerId::One, &[cards::OCCULT_EPIPHANY])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let id = epiphany.id;
    game.players[0].hand.push(epiphany);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1 + x);
    game.priority = PlayerId::One;
    (game, id)
}

/// Casts the Epiphany for `x`, discarding the cards named by `keep_out` --
/// the ones the test staged in hand, which are the ones it wants pitched.
fn cast_and_discard(game: &mut Game, epiphany: GameObjectId, x: u16, pitched: &[GameObjectId]) {
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => *card == epiphany && choices.x() == x,
            _ => false,
        })
        .unwrap_or_else(|| panic!("an Epiphany for X={x} is castable"));
    game.apply(PlayerId::One, cast).expect("it is castable");

    for _ in 0..16 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            // The discard names cards; pick the ones the test staged.
            let options = decision
                .options
                .iter()
                .filter(|option| option.card.is_some_and(|(id, _)| pitched.contains(&id)))
                .map(|option| option.id)
                .collect::<Vec<_>>();
            let options = if options.len() >= decision.minimum {
                options
            } else {
                decision
                    .options
                    .iter()
                    .take(decision.minimum)
                    .map(|option| option.id)
                    .collect()
            };
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .expect("the discard accepts what it offered");
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

fn spirits(game: &Game) -> usize {
    game.battlefield
        .iter()
        .filter(|permanent| {
            is_token_with(
                permanent,
                token_with_flying(tokens::creature(&["Spirit"], &[ManaColor::White], 1, 1)),
            )
        })
        .count()
}

fn ids(game: &Game, definitions: &[CardDefinitionId]) -> Vec<GameObjectId> {
    definitions
        .iter()
        .filter_map(|definition| {
            game.players[0]
                .hand
                .iter()
                .find(|card| card.definition == *definition)
                .map(|card| card.id)
        })
        .collect()
}

/// Three cards of three different types is three Spirits.
#[test]
fn each_distinct_type_is_a_spirit() {
    let staged_hand = [cards::MOUNTAIN, cards::LIGHTNING_BOLT, cards::GRIZZLY_BEARS];
    let (mut game, epiphany) = staged(3, &staged_hand);
    let pitched = ids(&game, &staged_hand);

    cast_and_discard(&mut game, epiphany, 3, &pitched);

    assert_eq!(spirits(&game), 3, "land, instant, and creature");
}

/// Two of the same type is one Spirit: the clause counts types, not cards.
#[test]
fn two_cards_of_one_type_are_one_spirit() {
    let staged_hand = [cards::GRIZZLY_BEARS, cards::SERRA_ANGEL];
    let (mut game, epiphany) = staged(2, &staged_hand);
    let pitched = ids(&game, &staged_hand);

    cast_and_discard(&mut game, epiphany, 2, &pitched);

    assert_eq!(spirits(&game), 1, "two creatures are one card type");
}

/// One card with two types is two Spirits, which is the other half of the
/// same distinction.
#[test]
fn one_card_of_two_types_is_two_spirits() {
    let staged_hand = [cards::DRYAD_ARBOR];
    let (mut game, epiphany) = staged(1, &staged_hand);
    let pitched = ids(&game, &staged_hand);

    cast_and_discard(&mut game, epiphany, 1, &pitched);

    assert_eq!(spirits(&game), 2, "a land creature is both");
}

/// X of zero draws nothing, discards nothing, and makes nothing.
#[test]
fn an_x_of_zero_makes_no_spirits() {
    let (mut game, epiphany) = staged(0, &[]);
    cast_and_discard(&mut game, epiphany, 0, &[]);

    assert_eq!(spirits(&game), 0);
}
