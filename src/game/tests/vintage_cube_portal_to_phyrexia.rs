//! Portal to Phyrexia: nine mana that empties their board on the way in and
//! refills yours every upkeep afterwards.

use super::*;

/// The Portal about to arrive, with `theirs` creatures under player two.
fn staged(theirs: &[CardDefinitionId]) -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[PlayerId::One.index()].graveyard.clear();
    game.players[PlayerId::Two.index()].graveyard.clear();
    for (index, definition) in theirs.iter().enumerate() {
        game.battlefield.push(creature(
            89_000 + u32::try_from(index).expect("few creatures"),
            *definition,
            PlayerId::Two,
        ));
    }
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game
}

/// Answers whatever is waiting by taking the first legal answer.
fn settle(game: &mut Game) {
    for _ in 0..16 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = decision
                .options
                .iter()
                .map(|option| option.id)
                .take(decision.minimum.max(1).min(decision.maximum))
                .collect();
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .expect("the offered choice is legal");
            continue;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let priority = game.priority;
        if game.apply(priority, Action::PassPriority).is_err() {
            break;
        }
    }
}

fn their_creatures(game: &Game) -> usize {
    game.battlefield
        .iter()
        .filter(|permanent| permanent.controller == PlayerId::Two)
        .filter(|permanent| {
            game.permanent_types(permanent)
                .is_some_and(|types| types.contains(CardType::Creature))
        })
        .count()
}

/// Three of theirs die on the way in, and they pick which.
#[test]
fn arriving_takes_three_of_theirs() {
    let mut game = staged(&[
        cards::GRIZZLY_BEARS,
        cards::GRIZZLY_BEARS,
        cards::SAVANNAH_LIONS,
        cards::SERRA_ANGEL,
    ]);
    game.put_onto_battlefield(PlayerId::One, cards::PORTAL_TO_PHYREXIA)
        .expect("cataloged");

    settle(&mut game);

    assert_eq!(their_creatures(&game), 1, "four go in, one comes out");
    assert_eq!(game.players[PlayerId::Two.index()].graveyard.len(), 3);
}

/// A player holding fewer than three gives up every one they have.
#[test]
fn two_creatures_are_both_taken() {
    let mut game = staged(&[cards::GRIZZLY_BEARS, cards::SAVANNAH_LIONS]);
    game.put_onto_battlefield(PlayerId::One, cards::PORTAL_TO_PHYREXIA)
        .expect("cataloged");

    settle(&mut game);

    assert_eq!(their_creatures(&game), 0);
    assert_eq!(game.players[PlayerId::Two.index()].graveyard.len(), 2);
}

/// Your own creatures are not theirs, so the Portal leaves them alone.
#[test]
fn it_does_not_take_your_own() {
    let mut game = staged(&[cards::GRIZZLY_BEARS]);
    let yours = game
        .put_onto_battlefield(PlayerId::One, cards::SERRA_ANGEL)
        .expect("cataloged");
    game.put_onto_battlefield(PlayerId::One, cards::PORTAL_TO_PHYREXIA)
        .expect("cataloged");

    settle(&mut game);

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == yours),
    );
}

/// The upkeep trigger takes a creature out of a graveyard -- either
/// graveyard -- and it arrives yours, and Phyrexian.
#[test]
fn the_upkeep_reanimates_from_any_graveyard() {
    let mut game = staged(&[]);
    game.players[PlayerId::Two.index()].graveyard.push(card(
        89_500,
        cards::SERRA_ANGEL,
        PlayerId::Two,
    ));
    game.put_onto_battlefield(PlayerId::One, cards::PORTAL_TO_PHYREXIA)
        .expect("cataloged");
    settle(&mut game);

    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    settle(&mut game);

    let angel = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::SERRA_ANGEL)
        .expect("the Angel came back");
    assert_eq!(
        angel.controller,
        PlayerId::One,
        "under your control, out of their graveyard",
    );
    assert!(
        game.effective_subtypes(angel)
            .contains(crate::card::Subtype::named("Phyrexian")),
        "and a Phyrexian in addition to being an Angel",
    );
    assert!(
        game.effective_subtypes(angel)
            .contains(crate::card::Subtype::named("Angel"))
    );
}

/// "*Each opponent sacrifices*": a sacrifice is not a destruction, so
/// indestructible is no protection from it. A Darksteel Myr goes with the
/// rest.
#[test]
fn indestructible_creatures_are_sacrificed_like_any_other() {
    let mut game = staged(&[cards::DARKSTEEL_MYR, cards::DARKSTEEL_MYR]);
    game.put_onto_battlefield(PlayerId::One, cards::PORTAL_TO_PHYREXIA)
        .expect("cataloged");

    settle(&mut game);

    assert_eq!(
        their_creatures(&game),
        0,
        "what cannot be destroyed can still be given up",
    );
}

/// "At the beginning of *your* upkeep": theirs comes and goes with the
/// graveyard untouched.
#[test]
fn their_upkeep_reanimates_nothing() {
    let mut game = staged(&[]);
    game.players[PlayerId::Two.index()].graveyard.push(card(
        89_600,
        cards::SERRA_ANGEL,
        PlayerId::Two,
    ));
    game.put_onto_battlefield(PlayerId::One, cards::PORTAL_TO_PHYREXIA)
        .expect("cataloged");
    settle(&mut game);

    game.active_player = PlayerId::Two;
    game.step = Step::Upkeep;
    game.priority = PlayerId::Two;
    game.handle_upkeep_triggers();
    settle(&mut game);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.definition != cards::SERRA_ANGEL),
        "the Angel stayed where it was lying",
    );
    assert_eq!(
        game.players[PlayerId::Two.index()].graveyard.len(),
        1,
        "their graveyard is as it was",
    );
}

/// "Target creature card from a graveyard": with none in either graveyard
/// the upkeep trigger has nothing to name and does nothing at all.
#[test]
fn an_upkeep_with_no_creature_card_anywhere_does_nothing() {
    let mut game = staged(&[]);
    game.players[PlayerId::One.index()].graveyard.push(card(
        89_700,
        cards::LIGHTNING_BOLT,
        PlayerId::One,
    ));
    game.players[PlayerId::Two.index()]
        .graveyard
        .push(card(89_701, cards::FOREST, PlayerId::Two));
    let portal = game
        .put_onto_battlefield(PlayerId::One, cards::PORTAL_TO_PHYREXIA)
        .expect("cataloged");
    settle(&mut game);

    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    settle(&mut game);

    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| permanent.card.id != portal)
            .count(),
        0,
        "a Bolt and a land are no creature cards, so nothing came back",
    );
    assert_eq!(
        game.players[PlayerId::One.index()].graveyard.len()
            + game.players[PlayerId::Two.index()].graveyard.len(),
        2,
        "and both graveyards are untouched",
    );
}
