//! Brainstorm: three cards for one mana, and two of them back in an order
//! its caster chooses.

use super::*;

/// Player One holding a Brainstorm, with `library` stacked so the last entry
/// is on top.
fn staged(library: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].library.clear();
    for definition in library {
        let card = game
            .build_zone(PlayerId::One, &[*definition])
            .expect("cataloged")
            .into_iter()
            .next()
            .expect("one card");
        game.players[0].library.push(card);
    }
    let card = game
        .build_zone(PlayerId::One, &[cards::BRAINSTORM])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let brainstorm = card.id;
    game.players[0].hand.push(card);
    game.players[0].mana_pool.blue = 1;
    game.turns_started = [1, 1];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, brainstorm)
}

/// Casts and resolves the Brainstorm, stopping at the put-back decision.
fn cast(game: &mut Game, brainstorm: GameObjectId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == brainstorm))
        .expect("one blue mana casts it");
    game.apply(PlayerId::One, action).expect("it casts");
    for _ in 0..8 {
        if game.observe(PlayerId::One).decision.is_some() {
            return;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
}

/// Answers the put-back decision by naming the two cards of `definitions` in
/// that order. The cards that land on the library are new objects, so what a
/// caller can recognise afterwards is what they are, not which object they
/// were in hand.
fn put_back(game: &mut Game, definitions: [CardDefinitionId; 2]) {
    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("two cards go back");
    let pick = |wanted: CardDefinitionId| {
        decision
            .options
            .iter()
            .find(|option| {
                option.card.is_some_and(|(_, characteristics)| {
                    characteristics.card_definition() == Some(wanted)
                })
            })
            .unwrap_or_else(|| panic!("the hand holds a {wanted:?}"))
    };
    let options = vec![pick(definitions[0]).id, pick(definitions[1]).id];
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options,
        },
    )
    .expect("naming two is legal");
}

/// Three cards up, two cards back, and one library net poorer.
#[test]
fn it_draws_three_and_puts_two_back() {
    let (mut game, brainstorm) = staged(&[
        cards::SERRA_ANGEL,
        cards::LIGHTNING_BOLT,
        cards::SAVANNAH_LIONS,
        cards::MOUNTAIN,
        cards::PLAINS,
    ]);

    cast(&mut game, brainstorm);
    put_back(&mut game, [cards::PLAINS, cards::MOUNTAIN]);

    assert_eq!(
        game.players[0].hand.len(),
        1,
        "an empty hand plus three drawn, less the two put back",
    );
    assert_eq!(
        game.players[0].library.len(),
        4,
        "five cards, three drawn, two back",
    );
}

/// "In any order" is the order the two are named in: each goes on top of the
/// last, so the second one named is the next card drawn.
#[test]
fn the_second_card_named_ends_up_on_top() {
    let run = |order: [CardDefinitionId; 2]| {
        let (mut game, brainstorm) = staged(&[
            cards::SERRA_ANGEL,
            cards::SAVANNAH_LIONS,
            cards::LIGHTNING_BOLT,
            cards::MOUNTAIN,
            cards::PLAINS,
        ]);
        cast(&mut game, brainstorm);
        put_back(&mut game, order);
        game.players[0]
            .library
            .last()
            .expect("a library")
            .definition
    };

    assert_eq!(
        run([cards::PLAINS, cards::MOUNTAIN]),
        cards::MOUNTAIN,
        "the Mountain was named second, so it is on top",
    );
    assert_eq!(
        run([cards::MOUNTAIN, cards::PLAINS]),
        cards::PLAINS,
        "and naming them the other way puts the Plains there instead",
    );
}

/// The cards go back where the drawn ones came from, not to the bottom.
#[test]
fn the_cards_go_back_on_top_rather_than_underneath() {
    let (mut game, brainstorm) = staged(&[
        cards::SERRA_ANGEL,
        cards::SAVANNAH_LIONS,
        cards::LIGHTNING_BOLT,
        cards::MOUNTAIN,
        cards::PLAINS,
    ]);
    let bottom = game.players[0].library[0].id;

    cast(&mut game, brainstorm);
    put_back(&mut game, [cards::PLAINS, cards::MOUNTAIN]);

    assert_eq!(
        game.players[0].library[0].id, bottom,
        "the bottom card never moved",
    );
    assert_eq!(
        game.players[0]
            .library
            .iter()
            .rev()
            .take(2)
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::MOUNTAIN, cards::PLAINS],
        "and both named cards are the top two, second-named first",
    );
}

/// It is an instant: castable in an opponent's turn.
#[test]
fn it_is_castable_at_instant_speed() {
    let (mut game, brainstorm) = staged(&[cards::PLAINS, cards::MOUNTAIN, cards::SERRA_ANGEL]);
    game.active_player = PlayerId::Two;
    game.step = Step::Upkeep;
    game.priority = PlayerId::One;

    assert!(
        game.legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == brainstorm)),
        "one blue mana on their upkeep",
    );
}

/// "Two cards from your hand", not two of the three drawn: a card that was
/// there all along goes back just as readily.
#[test]
fn a_card_held_from_before_can_go_back() {
    let (mut game, brainstorm) = staged(&[
        cards::SERRA_ANGEL,
        cards::LIGHTNING_BOLT,
        cards::SAVANNAH_LIONS,
        cards::MOUNTAIN,
        cards::PLAINS,
    ]);
    let held = game
        .build_zone(PlayerId::One, &[cards::ZURAN_ORB])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    game.players[0].hand.push(held);

    cast(&mut game, brainstorm);
    put_back(&mut game, [cards::PLAINS, cards::ZURAN_ORB]);

    assert_eq!(
        game.players[0]
            .library
            .last()
            .expect("a library")
            .definition,
        cards::ZURAN_ORB,
        "the card held from before is on top",
    );
}
