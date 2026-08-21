//! Paradoxical Outcome: pick up as many of your own permanents as you like,
//! and draw for the ones that came back to your hand.

use super::*;

/// Player One with an Outcome in hand and the mana to cast it.
fn staged() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let outcome = game
        .build_zone(PlayerId::One, &[cards::PARADOXICAL_OUTCOME])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let id = outcome.id;
    game.players[0].hand.push(outcome);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 4);
    game.priority = PlayerId::One;
    (game, id)
}

fn resolve(game: &mut Game) {
    for _ in 0..16 {
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

/// Casts the Outcome naming exactly `wanted`, and resolves it.
fn outcome(game: &mut Game, spell: GameObjectId, wanted: &[GameObjectId]) {
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                if *card != spell {
                    return false;
                }
                let named = choices.iter_targets().copied().collect::<Vec<_>>();
                named.len() == wanted.len()
                    && wanted
                        .iter()
                        .all(|id| named.contains(&Target::Permanent(*id)))
            }
            _ => false,
        })
        .expect("that combination of targets is on offer");
    game.apply(PlayerId::One, cast).expect("it is castable");
    resolve(game);
}

fn on_battlefield(game: &Game, id: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .any(|permanent| permanent.card.id == id)
}

/// Two picked up is two drawn.
#[test]
fn it_draws_for_each_permanent_returned() {
    let (mut game, spell) = staged();
    let first = game
        .put_onto_battlefield(PlayerId::One, cards::MOX_SAPPHIRE)
        .expect("cataloged");
    let second = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    drain_pending(&mut game);
    game.priority = PlayerId::One;

    outcome(&mut game, spell, &[first, second]);

    assert!(!on_battlefield(&game, first) && !on_battlefield(&game, second));
    assert_eq!(game.players[0].hand.len(), 4, "two picked up and two drawn");
}

/// Naming none is legal, and draws none.
#[test]
fn naming_nothing_draws_nothing() {
    let (mut game, spell) = staged();
    game.put_onto_battlefield(PlayerId::One, cards::MOX_SAPPHIRE)
        .expect("cataloged");
    drain_pending(&mut game);
    game.priority = PlayerId::One;

    outcome(&mut game, spell, &[]);

    assert!(
        game.players[0].hand.is_empty(),
        "nothing back and nothing drawn"
    );
    assert_eq!(game.battlefield.len(), 1, "and the Mox stayed put");
}

/// A land and a token are not legal targets, so an Outcome with only those
/// on the battlefield can name nothing at all.
#[test]
fn lands_and_tokens_are_not_legal_targets() {
    let (mut game, spell) = staged();
    game.put_onto_battlefield(PlayerId::One, cards::MOUNTAIN)
        .expect("cataloged");
    game.create_token(
        PlayerId::One,
        token_with_flying(tokens::creature(&["Spirit"], &[ManaColor::White], 1, 1)),
    );
    drain_pending(&mut game);
    game.priority = PlayerId::One;

    let named = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, choices, .. } if card == spell => {
                Some(choices.iter_targets().count())
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(named, vec![0], "the only cast names nothing");
}

/// A permanent you control but do not own goes back to its owner's hand, so
/// it returns without paying you a card.
#[test]
fn a_permanent_you_do_not_own_draws_you_nothing() {
    let (mut game, spell) = staged();
    let theirs = game
        .put_onto_battlefield(PlayerId::Two, cards::MOX_SAPPHIRE)
        .expect("cataloged");
    drain_pending(&mut game);
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == theirs)
    {
        permanent.controller = PlayerId::One;
    }
    game.priority = PlayerId::One;

    outcome(&mut game, spell, &[theirs]);

    assert!(!on_battlefield(&game, theirs), "it left the battlefield");
    assert!(
        game.players[1]
            .hand
            .iter()
            .any(|card| card.definition == cards::MOX_SAPPHIRE),
        "and went to its owner's hand",
    );
    assert!(
        game.players[0].hand.is_empty(),
        "which draws you nothing at all",
    );
}
