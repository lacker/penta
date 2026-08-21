//! Four cards that read a permanent's token status.
//!
//! Two want tokens and two want everything but tokens, so the pair of
//! directions is what these tests pin: an anthem that skips card creatures,
//! a sweeper that skips them too, and two draw triggers that a token cannot
//! set off.

use super::*;
use crate::ImplementationStatus;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game
}

fn stats(game: &Game, id: GameObjectId) -> (Option<i16>, Option<i16>) {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still there");
    (game.power(permanent), game.toughness(permanent))
}

/// Answers each waiting decision by taking the last option, which for a
/// "you may draw" is the branch that accepts.
fn drain_accepting(game: &mut Game) {
    for _ in 0..16 {
        if game.stack.is_empty()
            && game.pending_triggers.is_empty()
            && game.pending_decisions.is_empty()
        {
            return;
        }
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let take = decision.minimum.max(1).min(decision.maximum);
            let options = decision
                .options
                .iter()
                .rev()
                .map(|option| option.id)
                .take(take)
                .collect::<Vec<_>>();
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
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            return;
        }
    }
}

/// The General's anthem reaches its tokens and stops at creature cards --
/// including itself.
#[test]
fn phantom_general_pumps_tokens_only() {
    let mut game = ready();
    let general = creature(10_000, cards::PHANTOM_GENERAL, PlayerId::One);
    let general_id = general.card.id;
    game.battlefield.push(general);
    let token = token_permanent(
        10_100,
        tokens::creature(&["Zombie"], &[ManaColor::Black], 2, 2),
        PlayerId::One,
    );
    let token_id = token.card.id;
    game.battlefield.push(token);
    let bear = creature(10_101, cards::GRIZZLY_BEARS, PlayerId::One);
    let bear_id = bear.card.id;
    game.battlefield.push(bear);

    assert_eq!(stats(&game, token_id), (Some(3), Some(3)), "a 2/2 plus one");
    assert_eq!(stats(&game, bear_id), (Some(2), Some(2)), "not a token");
    assert_eq!(
        stats(&game, general_id),
        (Some(2), Some(3)),
        "and the General is a card too",
    );
}

/// Illness in the Ranks shrinks both players' tokens and leaves cards alone.
#[test]
fn illness_in_the_ranks_shrinks_every_players_tokens() {
    let mut game = ready();
    game.battlefield
        .push(creature(10_000, cards::ILLNESS_IN_THE_RANKS, PlayerId::One));
    let mine = token_permanent(
        10_100,
        tokens::creature(&["Zombie"], &[ManaColor::Black], 2, 2),
        PlayerId::One,
    );
    let mine_id = mine.card.id;
    game.battlefield.push(mine);
    let theirs = token_permanent(
        10_101,
        tokens::creature(&["Zombie"], &[ManaColor::Black], 2, 2),
        PlayerId::Two,
    );
    let theirs_id = theirs.card.id;
    game.battlefield.push(theirs);
    let bear = creature(10_102, cards::GRIZZLY_BEARS, PlayerId::Two);
    let bear_id = bear.card.id;
    game.battlefield.push(bear);

    assert_eq!(stats(&game, mine_id), (Some(1), Some(1)));
    assert_eq!(
        stats(&game, theirs_id),
        (Some(1), Some(1)),
        "the opponent's token too",
    );
    assert_eq!(stats(&game, bear_id), (Some(2), Some(2)), "cards are safe");
}

/// A 1/1 token dies to it outright, which is the whole point of the card.
#[test]
fn illness_in_the_ranks_kills_one_toughness_tokens() {
    let mut game = ready();
    game.battlefield
        .push(creature(10_000, cards::ILLNESS_IN_THE_RANKS, PlayerId::Two));
    game.battlefield.push(token_permanent(
        10_100,
        tokens::creature(&["Human"], &[ManaColor::White], 1, 1),
        PlayerId::One,
    ));

    game.check_state_based_actions();
    assert!(
        !game.battlefield.iter().any(|permanent| is_token_with(
            permanent,
            tokens::creature(&["Human"], &[ManaColor::White], 1, 1)
        )),
        "a 1/1 token is a 0/0",
    );
}

/// Destroys the named creature and reports how many cards the controller of
/// the Harvester drew from its death.
fn cards_drawn_when_dying(victim: Permanent) -> usize {
    let mut game = ready();
    game.battlefield
        .push(creature(10_000, cards::HARVESTER_OF_SOULS, PlayerId::One));
    let victim_id = victim.card.id;
    game.battlefield.push(victim);
    for index in 0..5 {
        let filler = card(30_000 + index, cards::GRIZZLY_BEARS, PlayerId::One);
        game.players[PlayerId::One.index()].library.push(filler);
    }

    let before = game.players[PlayerId::One.index()].hand.len();
    game.destroy_permanent(victim_id);
    game.check_state_based_actions();
    drain_accepting(&mut game);
    game.players[PlayerId::One.index()].hand.len() - before
}

#[test]
fn the_harvester_draws_off_creature_cards_only() {
    assert_eq!(
        cards_drawn_when_dying(creature(10_100, cards::GRIZZLY_BEARS, PlayerId::One)),
        1,
    );
    assert_eq!(
        cards_drawn_when_dying(token_permanent(
            10_100,
            tokens::creature(&["Zombie"], &[ManaColor::Black], 2, 2),
            PlayerId::One,
        )),
        0,
        "a token dying is not another nontoken creature",
    );
}

/// And the Soul's entry trigger reads the same status on the way in.
#[test]
fn the_soul_draws_off_creature_cards_entering() {
    let entering = |permanent: Permanent| {
        let mut game = ready();
        game.battlefield
            .push(creature(10_000, cards::SOUL_OF_THE_HARVEST, PlayerId::One));
        for index in 0..5 {
            let filler = card(30_000 + index, cards::GRIZZLY_BEARS, PlayerId::One);
            game.players[PlayerId::One.index()].library.push(filler);
        }

        let before = game.players[PlayerId::One.index()].hand.len();
        game.enqueue_battlefield_entry(PendingBattlefieldEntry {
            permanent,
            from: ZoneKind::Hand,
            completion: EntryCompletion::None,
            redirected_to: None,
        });
        drain_accepting(&mut game);
        game.players[PlayerId::One.index()].hand.len() - before
    };

    assert_eq!(
        entering(creature(10_100, cards::GRIZZLY_BEARS, PlayerId::One)),
        1,
    );
    assert_eq!(
        entering(token_permanent(
            10_100,
            tokens::creature(&["Zombie"], &[ManaColor::Black], 2, 2),
            PlayerId::One,
        )),
        0,
        "a token arriving is not a nontoken creature",
    );
}

#[test]
fn all_four_report_complete_coverage() {
    let catalog = poc::catalog().expect("catalog builds");
    for definition in [
        cards::ILLNESS_IN_THE_RANKS,
        cards::PHANTOM_GENERAL,
        cards::HARVESTER_OF_SOULS,
        cards::SOUL_OF_THE_HARVEST,
    ] {
        let card = catalog.get(definition).expect("the card is cataloged");
        assert_eq!(
            card.rules.implementation_status(),
            ImplementationStatus::Complete,
            "{} should be fully executable",
            card.name,
        );
    }
}
