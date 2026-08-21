//! Brightglass Gearhulk: a 4/4 that finds the two one-drops the deck is
//! built around.

use super::*;

/// Player One holding a Gearhulk with four mana up and `library` stacked in
/// their own library.
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
        .build_zone(PlayerId::One, &[cards::BRIGHTGLASS_GEARHULK])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let gearhulk = card.id;
    game.players[0].hand.push(card);
    game.turns_started = [1, 1];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 2);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 2);
    (game, gearhulk)
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

/// Casts it and takes the "you may", which is the decision before the
/// search. Answering `false` declines instead.
fn cast(game: &mut Game, gearhulk: GameObjectId, search: bool) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == gearhulk))
        .expect("four mana casts it");
    game.apply(PlayerId::One, action).expect("it casts");
    settle(game);
    let seat = deciding(game).expect("the may is offered");
    let decision = game.observe(seat).decision.expect("just checked");
    let wanted = if search { "Do it" } else { "Decline" };
    let option = decision
        .options
        .iter()
        .find(|option| option.label == wanted)
        .unwrap_or_else(|| panic!("{wanted} is one of the two"))
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

/// Answers whatever is pending by naming `wanted`, or by taking none.
fn answer(game: &mut Game, wanted: &[CardDefinitionId]) {
    let seat = deciding(game).expect("somebody is being asked");
    let decision = game.observe(seat).decision.expect("just checked");
    let options = wanted
        .iter()
        .map(|definition| {
            decision
                .options
                .iter()
                .find(|option| {
                    option
                        .card
                        .is_some_and(|(_, found)| found.card_definition() == Some(*definition))
                })
                .unwrap_or_else(|| panic!("{definition:?} is offered"))
                .id
        })
        .collect();
    game.apply(
        seat,
        Action::ChooseDecision {
            decision: decision.id,
            options,
        },
    )
    .expect("the answer is legal");
    settle(game);
}

/// What the search is offering, as card definitions.
fn offered(game: &Game) -> Vec<CardDefinitionId> {
    let seat = deciding(game).expect("something is being asked");
    game.observe(seat)
        .decision
        .expect("just checked")
        .options
        .iter()
        .filter_map(|option| {
            option
                .card
                .and_then(|(_, characteristics)| characteristics.card_definition())
        })
        .collect()
}

fn in_hand(game: &Game, definition: CardDefinitionId) -> bool {
    game.players[0]
        .hand
        .iter()
        .any(|card| card.definition == definition)
}

/// It finds two one-mana permanents and puts them in hand.
#[test]
fn it_finds_two_cheap_permanent_cards() {
    let (mut game, gearhulk) = staged(&[
        cards::SOL_RING,
        cards::SAVANNAH_LIONS,
        cards::MOUNTAIN,
        cards::ISLAND,
    ]);
    let library = game.players[0].library.len();

    cast(&mut game, gearhulk, true);
    answer(&mut game, &[cards::SOL_RING, cards::SAVANNAH_LIONS]);

    assert!(in_hand(&game, cards::SOL_RING), "the artifact");
    assert!(in_hand(&game, cards::SAVANNAH_LIONS), "and the creature");
    assert_eq!(
        game.players[0].library.len(),
        library - 2,
        "both came out of the library",
    );
}

/// "Mana value 1 or less" and those three types: a Serra Angel is a
/// creature and too expensive, a Mountain is a land, and neither is on the
/// menu.
#[test]
fn it_only_offers_cheap_permanents_of_the_three_types() {
    let (mut game, gearhulk) = staged(&[
        cards::SOL_RING,
        cards::SERRA_ANGEL,
        cards::MOUNTAIN,
        cards::LIGHTNING_BOLT,
    ]);

    cast(&mut game, gearhulk, true);

    let offered = offered(&game);
    assert_eq!(
        offered,
        vec![cards::SOL_RING],
        "a one-mana artifact and nothing else: {offered:?}",
    );
}

/// "Up to two": one is a legal answer, and so is none.
#[test]
fn taking_fewer_than_two_is_allowed() {
    let (mut game, gearhulk) = staged(&[cards::SOL_RING, cards::SAVANNAH_LIONS, cards::MOUNTAIN]);
    cast(&mut game, gearhulk, true);
    let seat = deciding(&game).expect("the search asks");
    assert_eq!(
        game.observe(seat).decision.expect("just checked").minimum,
        0,
        "with no obligation to take any",
    );

    answer(&mut game, &[cards::SOL_RING]);

    assert!(in_hand(&game, cards::SOL_RING), "one taken");
    assert!(
        !in_hand(&game, cards::SAVANNAH_LIONS),
        "and the other left where it was",
    );
}

/// A 4/4 with first strike and trample besides.
#[test]
fn it_is_a_four_four_with_first_strike_and_trample() {
    let (mut game, gearhulk) = staged(&[cards::MOUNTAIN]);
    cast(&mut game, gearhulk, false);

    let body = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::BRIGHTGLASS_GEARHULK)
        .expect("it resolved");
    assert_eq!((game.power(body), game.toughness(body)), (Some(4), Some(4)));
    assert!(game.permanent_has_executable_keyword(body, KeywordAbility::FirstStrike));
    assert!(game.permanent_has_executable_keyword(body, KeywordAbility::Trample));
    assert!(
        game.permanent_types(body)
            .is_some_and(|types| types.contains(CardType::Artifact)),
        "and an artifact creature",
    );
}
