//! Metamorphosis Fanatic: a body that reanimates, and gives what it brings
//! back lifelink by putting a counter on it rather than by lending it one.

use super::*;

/// Player One with the Fanatic in hand, a Grizzly Bears in the graveyard,
/// and enough mana for either way of casting it.
fn staged() -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].graveyard.clear();
    game.players[0]
        .graveyard
        .push(card(89_000, cards::GRIZZLY_BEARS, PlayerId::One));
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 2);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 4);
    game
}

fn permanent(game: &Game, definition: CardDefinitionId) -> Option<&Permanent> {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == definition)
}

/// Puts the Fanatic onto the battlefield and answers whatever its trigger
/// asks, choosing `targets` from the offered options.
fn arrive_and_target(game: &mut Game, targets: &[u32]) {
    game.put_onto_battlefield(PlayerId::One, cards::METAMORPHOSIS_FANATIC)
        .expect("cataloged");
    // The entry is a rules procedure; its trigger only reaches the stack,
    // and asks for a target, once that procedure finishes.
    game.finish_rules_procedure();
    // A graveyard with nothing in it offers nothing to choose, so the "up to
    // one" trigger resolves without asking.
    let Some(decision) = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
    else {
        drain_pending(game);
        return;
    };
    let options = decision
        .options
        .iter()
        .enumerate()
        .filter(|(index, _)| targets.contains(&u32::try_from(*index).expect("small")))
        .map(|(_, option)| option.id)
        .collect::<Vec<_>>();
    game.apply(
        decision.player,
        Action::ChooseDecision {
            decision: decision.id,
            options,
        },
    )
    .expect("the target choice is legal");
    drain_pending(game);
}

/// The printed body has lifelink of its own.
#[test]
fn the_fanatic_has_lifelink() {
    let mut game = staged();
    game.players[0].graveyard.clear();
    arrive_and_target(&mut game, &[]);
    let fanatic = permanent(&game, cards::METAMORPHOSIS_FANATIC).expect("it entered");

    assert!(game.permanent_has_executable_keyword(fanatic, KeywordAbility::Lifelink));
}

/// The trigger brings a creature card back out of your graveyard.
#[test]
fn it_returns_a_creature_from_your_graveyard() {
    let mut game = staged();

    arrive_and_target(&mut game, &[0]);

    assert!(
        permanent(&game, cards::GRIZZLY_BEARS).is_some(),
        "the bear is on the battlefield",
    );
    assert!(
        game.players[0].graveyard.is_empty(),
        "and out of the graveyard",
    );
}

/// It arrives carrying a lifelink counter, and therefore lifelink -- which
/// is a thing about the permanent rather than a grant with a duration.
#[test]
fn what_comes_back_has_a_lifelink_counter() {
    let mut game = staged();

    arrive_and_target(&mut game, &[0]);
    let bear = permanent(&game, cards::GRIZZLY_BEARS).expect("the bear came back");

    assert_eq!(bear.counters(CounterKind::Lifelink), 1);
    assert!(
        game.permanent_has_executable_keyword(bear, KeywordAbility::Lifelink),
        "the counter is what gives it the keyword (CR 122.1b)",
    );
}

/// The counter is not a duration, so nothing about a new turn takes it away.
#[test]
fn the_counter_outlives_the_turn() {
    let mut game = staged();
    arrive_and_target(&mut game, &[0]);
    let bear = permanent(&game, cards::GRIZZLY_BEARS)
        .map(|permanent| permanent.card.id)
        .expect("the bear came back");

    game.complete_cleanup();
    drain_pending(&mut game);
    let bear = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == bear)
        .expect("the bear is still there");

    assert_eq!(bear.counters(CounterKind::Lifelink), 1);
    assert!(game.permanent_has_executable_keyword(bear, KeywordAbility::Lifelink));
}

/// "Up to one": choosing nothing is a legal declaration, and nothing comes
/// back.
#[test]
fn it_can_choose_to_return_nothing() {
    let mut game = staged();

    arrive_and_target(&mut game, &[]);

    assert!(permanent(&game, cards::GRIZZLY_BEARS).is_none());
    assert_eq!(
        game.players[0].graveyard.len(),
        1,
        "the bear stayed where it was",
    );
}

/// "Your graveyard": a creature card in theirs is not on the menu.
#[test]
fn it_does_not_reach_the_opponents_graveyard() {
    let mut game = staged();
    game.players[0].graveyard.clear();
    game.players[1]
        .graveyard
        .push(card(89_010, cards::GRIZZLY_BEARS, PlayerId::Two));

    game.put_onto_battlefield(PlayerId::One, cards::METAMORPHOSIS_FANATIC)
        .expect("cataloged");
    drain_pending(&mut game);

    assert!(
        permanent(&game, cards::GRIZZLY_BEARS).is_none(),
        "their bear stayed in their graveyard",
    );
    assert_eq!(game.players[1].graveyard.len(), 1);
}

/// A noncreature card in your own graveyard is not a legal target either.
#[test]
fn it_names_creature_cards_only() {
    let mut game = staged();
    game.players[0].graveyard.clear();
    game.players[0]
        .graveyard
        .push(card(89_020, cards::LIGHTNING_BOLT, PlayerId::One));

    game.put_onto_battlefield(PlayerId::One, cards::METAMORPHOSIS_FANATIC)
        .expect("cataloged");
    drain_pending(&mut game);

    assert_eq!(
        game.players[0].graveyard.len(),
        1,
        "the Bolt is not a creature card",
    );
}

/// Miracle: drawn as the first card of the turn, it can be cast for {1}{B}
/// instead of six mana.
#[test]
fn it_can_be_cast_for_its_miracle_cost() {
    let mut game = staged();
    game.players[0].library = vec![card(89_030, cards::METAMORPHOSIS_FANATIC, PlayerId::One)];
    game.turn = 2;
    game.step = Step::Draw;
    game.priority = PlayerId::One;
    game.cards_drawn_this_turn = [0; 2];
    game.drawn_this_turn = [Vec::new(), Vec::new()];
    let drawn = game.draw_card(PlayerId::One).expect("the Fanatic is drawn");

    // Reveal it, which is the option the draw-action window offers.
    let reveal = game
        .observe(PlayerId::One)
        .decision
        .expect("the drawing player is offered the reveal");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: reveal.id,
            options: vec![1],
        },
    )
    .expect("revealing is legal");
    pass_until_decision(&mut game);

    let offer = game
        .observe(PlayerId::One)
        .decision
        .expect("the linked trigger resolves into a cast offer");
    assert!(
        offer.options.iter().any(|option| option.card
            == Some((
                drawn,
                ObjectCharacteristics::card(cards::METAMORPHOSIS_FANATIC, CardPartId::PRIMARY),
            ))),
        "the offer is about the card that was just drawn",
    );

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == drawn))
        .expect("the Fanatic is castable for its miracle cost");
    let before = game.players[0].mana_pool.total();
    game.apply(PlayerId::One, cast).expect("the cast is legal");

    assert_eq!(
        before - game.players[0].mana_pool.total(),
        2,
        "two mana, not six",
    );
}
