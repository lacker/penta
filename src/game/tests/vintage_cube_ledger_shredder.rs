//! Ledger Shredder: a two-drop that filters a hand and grows for it, on
//! anybody's turn.

use super::*;

/// Player One with a Shredder out since last turn, `mine` in hand, and both
/// players holding a Bolt to cast.
fn staged(mine: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    for seat in [PlayerId::One, PlayerId::Two] {
        game.players[seat.index()].hand.clear();
    }
    for definition in mine {
        let card = game
            .build_zone(PlayerId::One, &[*definition])
            .expect("cataloged")
            .into_iter()
            .next()
            .expect("one card");
        game.players[0].hand.push(card);
    }
    let shredder = game
        .put_onto_battlefield(PlayerId::One, cards::LEDGER_SHREDDER)
        .expect("cataloged");
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [2, 1];
    drain_pending(&mut game);
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, shredder)
}

fn deciding(game: &Game) -> Option<PlayerId> {
    game.pending_decisions
        .first()
        .map(|pending| pending.observation.player)
}

fn settle(game: &mut Game) {
    for _ in 0..24 {
        if deciding(game).is_some() {
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
    game.check_state_based_actions();
}

/// Puts a Bolt in `seat`'s hand, casts it at the other player, and lets the
/// stack settle.
fn cast_a_bolt(game: &mut Game, seat: PlayerId) {
    let card = game
        .build_zone(seat, &[cards::LIGHTNING_BOLT])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let id = card.id;
    game.players[seat.index()].hand.push(card);
    game.add_unrestricted_mana(seat, ManaColor::Red, 1);
    game.priority = seat;
    let cast = game
        .legal_actions(seat)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == id
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Player(seat.opponent()))
            }
            _ => false,
        })
        .expect("the Bolt is castable");
    game.apply(seat, cast).expect("it casts");
    settle(game);
}

/// Answers the connive discard by naming `wanted`.
fn discard(game: &mut Game, wanted: CardDefinitionId) {
    let seat = deciding(game).expect("connive asks what to discard");
    let decision = game.observe(seat).decision.expect("just checked");
    let option = decision
        .options
        .iter()
        .find(|option| {
            option
                .card
                .is_some_and(|(_, found)| found.card_definition() == Some(wanted))
        })
        .unwrap_or_else(|| panic!("{wanted:?} is in hand"))
        .id;
    game.apply(
        seat,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .expect("the answer is legal");
    settle(game);
}

fn the_shredder(game: &Game) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::LEDGER_SHREDDER)
        .expect("it is on the battlefield")
}

/// The first spell does nothing; the second connives.
#[test]
fn the_second_spell_of_the_turn_connives() {
    let (mut game, _shredder) = staged(&[cards::SERRA_ANGEL]);

    cast_a_bolt(&mut game, PlayerId::One);
    assert!(deciding(&game).is_none(), "one spell is not two");

    cast_a_bolt(&mut game, PlayerId::One);

    assert!(
        deciding(&game).is_some(),
        "the second spell connived, and it is asking what to discard",
    );
}

/// Discarding a nonland card puts a counter on.
#[test]
fn discarding_a_nonland_card_grows_it() {
    let (mut game, _shredder) = staged(&[cards::SERRA_ANGEL]);
    assert_eq!(game.power(the_shredder(&game)), Some(1), "a 1/3 so far");

    cast_a_bolt(&mut game, PlayerId::One);
    cast_a_bolt(&mut game, PlayerId::One);
    discard(&mut game, cards::SERRA_ANGEL);

    assert_eq!(
        the_shredder(&game).counters(CounterKind::PlusOnePlusOne),
        1,
        "one counter for the nonland card",
    );
    assert_eq!(game.power(the_shredder(&game)), Some(2), "a 2/4 now");
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::SERRA_ANGEL),
        "and the Angel was discarded",
    );
}

/// Discarding a land does not.
#[test]
fn discarding_a_land_does_not() {
    let (mut game, _shredder) = staged(&[cards::MOUNTAIN]);

    cast_a_bolt(&mut game, PlayerId::One);
    cast_a_bolt(&mut game, PlayerId::One);
    discard(&mut game, cards::MOUNTAIN);

    assert_eq!(
        the_shredder(&game).counters(CounterKind::PlusOnePlusOne),
        0,
        "a land buys no counter",
    );
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::MOUNTAIN),
        "though the land still went",
    );
}

/// Connive draws first and discards second, so the card drawn is one of the
/// cards that may be discarded.
#[test]
fn it_draws_before_it_discards() {
    let (mut game, _shredder) = staged(&[cards::MOUNTAIN]);
    let drawn = game
        .build_zone(PlayerId::One, &[cards::SERRA_ANGEL])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    game.players[0].library.push(drawn);

    cast_a_bolt(&mut game, PlayerId::One);
    cast_a_bolt(&mut game, PlayerId::One);

    let seat = deciding(&game).expect("connive is asking");
    let offered = game
        .observe(seat)
        .decision
        .expect("just checked")
        .options
        .iter()
        .filter_map(|option| {
            option
                .card
                .and_then(|(_, characteristics)| characteristics.card_definition())
        })
        .collect::<Vec<_>>();
    assert!(
        offered.contains(&cards::SERRA_ANGEL),
        "the card connive just drew is one of the cards it may discard: {offered:?}",
    );
    assert!(offered.contains(&cards::MOUNTAIN), "beside the hand it had");
}

/// "A player", not "you": their second spell connives your Shredder.
#[test]
fn their_second_spell_connives_it_too() {
    let (mut game, _shredder) = staged(&[cards::SERRA_ANGEL]);

    cast_a_bolt(&mut game, PlayerId::Two);
    assert!(deciding(&game).is_none(), "one of theirs is not two");
    cast_a_bolt(&mut game, PlayerId::Two);

    let seat = deciding(&game).expect("their second spell connived it");
    assert_eq!(
        seat,
        PlayerId::One,
        "and it is the Shredder's controller who draws and discards",
    );
}

/// The counts are per player: one spell each is not a second spell for
/// either of them.
#[test]
fn one_spell_each_is_not_a_second_spell() {
    let (mut game, _shredder) = staged(&[cards::SERRA_ANGEL]);

    cast_a_bolt(&mut game, PlayerId::One);
    cast_a_bolt(&mut game, PlayerId::Two);

    assert!(deciding(&game).is_none(), "their first is not your second");
}

/// "Each turn": the third spell is not the second, so it connives once.
#[test]
fn a_third_spell_does_not_connive_again() {
    let (mut game, _shredder) = staged(&[cards::SERRA_ANGEL, cards::MOUNTAIN]);
    cast_a_bolt(&mut game, PlayerId::One);
    cast_a_bolt(&mut game, PlayerId::One);
    discard(&mut game, cards::SERRA_ANGEL);

    cast_a_bolt(&mut game, PlayerId::One);

    assert!(
        deciding(&game).is_none(),
        "a third spell is not a second one",
    );
    assert_eq!(
        the_shredder(&game).counters(CounterKind::PlusOnePlusOne),
        1,
        "still the one counter",
    );
}

/// A 1/3 flier besides.
#[test]
fn it_flies() {
    let (game, _shredder) = staged(&[]);

    assert!(game.permanent_has_executable_keyword(the_shredder(&game), KeywordAbility::Flying));
    assert_eq!(game.toughness(the_shredder(&game)), Some(3));
}
