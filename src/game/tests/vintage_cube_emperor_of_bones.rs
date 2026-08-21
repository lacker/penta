//! Emperor of Bones: a graveyard eaten one card a turn, and the best of them
//! rented back for exactly one attack.

use super::*;

/// Player One with an Emperor out since last turn, `theirs` in Player Two's
/// graveyard, and four mana up.
fn staged(theirs: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[1].graveyard.clear();
    for definition in theirs {
        let card = game
            .build_zone(PlayerId::Two, &[*definition])
            .expect("cataloged")
            .into_iter()
            .next()
            .expect("one card");
        game.players[1].graveyard.push(card);
    }
    let emperor = game
        .put_onto_battlefield(PlayerId::One, cards::EMPEROR_OF_BONES)
        .expect("cataloged");
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [1, 1];
    drain_pending(&mut game);
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, emperor)
}

/// Who has a decision waiting, if anyone. `Game::decision_player` answers a
/// different question -- who acts next -- and is always `Some`.
fn deciding(game: &Game) -> Option<PlayerId> {
    game.pending_decisions
        .first()
        .map(|pending| pending.observation.player)
}

/// Passes until somebody is asked something, or the stack is quiet.
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

/// Answers whatever is pending by naming the option whose card is `wanted`,
/// or the first option when nothing is named.
fn answer(game: &mut Game, wanted: Option<CardDefinitionId>) {
    let seat = deciding(game).expect("somebody is being asked");
    let decision = game.observe(seat).decision.expect("just checked");
    let option = match wanted {
        Some(definition) => decision
            .options
            .iter()
            .find(|option| {
                option.card.is_some_and(|(_, characteristics)| {
                    characteristics.card_definition() == Some(definition)
                })
            })
            .unwrap_or_else(|| panic!("{definition:?} is offered")),
        None => decision.options.first().expect("something is offered"),
    }
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

/// Walks to the beginning of combat and lets the Emperor's trigger resolve
/// there, answering its target with `wanted`. Stops in that step rather than
/// walking on: everything else this card does is activated, and only the
/// declare-attackers step refuses to offer an activation.
fn reach_combat(game: &mut Game, wanted: Option<CardDefinitionId>) {
    for _ in 0..24 {
        if game.step == Step::BeginningOfCombat {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            return;
        }
    }
    for _ in 0..8 {
        settle(game);
        if deciding(game).is_some() {
            answer(game, wanted);
            continue;
        }
        return;
    }
}

/// Mana empties at every step change, so the two the adapt costs are raised
/// here rather than staged once at the top.
fn adapt(game: &mut Game, emperor: GameObjectId) {
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == emperor),
        )
        .expect("adapt is activatable");
    game.apply(PlayerId::One, action).expect("it activates");
    settle(game);
}

/// One step of "let the turn get on with it": answer anything pending,
/// finish a combat declaration when one is open, and otherwise pass.
fn advance(game: &mut Game) -> bool {
    if deciding(game).is_some() {
        answer(game, None);
        return true;
    }
    for action in [
        Action::FinishDeclaringAttackers,
        Action::FinishDeclaringBlockers,
    ] {
        for seat in [PlayerId::One, PlayerId::Two] {
            if game.legal_actions(seat).contains(&action) {
                return game.apply(seat, action.clone()).is_ok();
            }
        }
    }
    let player = game.priority;
    game.apply(player, Action::PassPriority).is_ok()
}

fn permanent(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("it is on the battlefield")
}

fn on_battlefield(game: &Game, definition: CardDefinitionId) -> Option<&Permanent> {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == definition)
}

/// The combat trigger takes a card out of a graveyard and keeps it linked.
#[test]
fn it_exiles_a_card_at_the_beginning_of_combat() {
    let (mut game, _emperor) = staged(&[cards::SERRA_ANGEL]);

    reach_combat(&mut game, Some(cards::SERRA_ANGEL));

    assert!(
        game.players[1].graveyard.is_empty(),
        "the Angel left the graveyard",
    );
    assert!(
        game.players[1]
            .exile
            .iter()
            .any(|card| card.definition == cards::SERRA_ANGEL),
        "and is in its owner's exile",
    );
}

/// Adapt puts two counters on, and finds them already there the second time.
#[test]
fn adapt_two_only_works_on_an_empty_creature() {
    let (mut game, emperor) = staged(&[]);
    assert_eq!(
        permanent(&game, emperor).counters(CounterKind::PlusOnePlusOne),
        0,
        "nothing on it yet",
    );

    adapt(&mut game, emperor);
    assert_eq!(
        permanent(&game, emperor).counters(CounterKind::PlusOnePlusOne),
        2,
        "two counters",
    );

    adapt(&mut game, emperor);
    assert_eq!(
        permanent(&game, emperor).counters(CounterKind::PlusOnePlusOne),
        2,
        "and still two: adapt found a counter already there",
    );
}

/// Counters going on reanimate a card the Emperor exiled, with haste and a
/// finality counter, under the Emperor's controller.
#[test]
fn counters_bring_back_a_card_it_exiled() {
    let (mut game, emperor) = staged(&[cards::SERRA_ANGEL]);
    reach_combat(&mut game, Some(cards::SERRA_ANGEL));

    adapt(&mut game, emperor);
    if deciding(&game).is_some() {
        answer(&mut game, Some(cards::SERRA_ANGEL));
    }

    let angel = on_battlefield(&game, cards::SERRA_ANGEL).expect("the Angel came back");
    assert_eq!(
        angel.controller,
        PlayerId::One,
        "under your control, not its owner's",
    );
    assert_eq!(
        angel.counters(CounterKind::Finality),
        1,
        "with a finality counter",
    );
    assert!(
        game.permanent_has_executable_keyword(angel, KeywordAbility::Haste),
        "and haste, which is the whole point of doing it in combat",
    );
}

/// A finality counter is not a marker: the creature is exiled rather than
/// buried when it dies.
#[test]
fn what_comes_back_is_exiled_when_it_dies() {
    let (mut game, emperor) = staged(&[cards::SERRA_ANGEL]);
    reach_combat(&mut game, Some(cards::SERRA_ANGEL));
    adapt(&mut game, emperor);
    if deciding(&game).is_some() {
        answer(&mut game, Some(cards::SERRA_ANGEL));
    }
    let angel = on_battlefield(&game, cards::SERRA_ANGEL)
        .expect("the Angel came back")
        .card
        .id;

    game.move_permanents_to_graveyard(&[angel]);
    settle(&mut game);

    assert!(
        !game.players[1]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::SERRA_ANGEL),
        "it never reached a graveyard",
    );
    assert!(
        game.players[1]
            .exile
            .iter()
            .any(|card| card.definition == cards::SERRA_ANGEL),
        "and went back to exile instead",
    );
}

/// The rental ends: the end step takes it back.
#[test]
fn it_is_sacrificed_at_the_next_end_step() {
    let (mut game, emperor) = staged(&[cards::SERRA_ANGEL]);
    reach_combat(&mut game, Some(cards::SERRA_ANGEL));
    adapt(&mut game, emperor);
    if deciding(&game).is_some() {
        answer(&mut game, Some(cards::SERRA_ANGEL));
    }
    assert!(
        on_battlefield(&game, cards::SERRA_ANGEL).is_some(),
        "it is here for now",
    );

    for _ in 0..40 {
        if on_battlefield(&game, cards::SERRA_ANGEL).is_none() {
            break;
        }
        if !advance(&mut game) {
            break;
        }
    }

    assert!(
        on_battlefield(&game, cards::SERRA_ANGEL).is_none(),
        "the end step took it back",
    );
}

/// "A creature card exiled with this creature": a land it ate is not one,
/// and with nothing else there the trigger finds nothing to bring back.
#[test]
fn a_noncreature_card_it_exiled_is_not_a_candidate() {
    let (mut game, emperor) = staged(&[cards::MOUNTAIN]);
    reach_combat(&mut game, Some(cards::MOUNTAIN));

    adapt(&mut game, emperor);

    assert!(
        deciding(&game).is_none(),
        "there was nothing to choose among",
    );
    assert!(
        on_battlefield(&game, cards::MOUNTAIN).is_none(),
        "and the land stayed in exile",
    );
    assert_eq!(
        permanent(&game, emperor).counters(CounterKind::PlusOnePlusOne),
        2,
        "the counters went on all the same",
    );
}
