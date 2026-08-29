//! Scavenge.
//!
//! An activated ability whose source is a card in a graveyard, which is a
//! zone nothing else activates from. Its cost exiles that card, so by the
//! time the ability resolves the thing whose power it reads has already
//! left -- and it is only offered when a sorcery could be cast.

use super::*;

/// Slitherhead in the graveyard, a creature to grow, and enough mana that
/// affordability is never the reason an activation is missing.
fn scavenge_board(scavenger: CardDefinitionId) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    let dead = card(10_000, scavenger, PlayerId::One);
    let dead_id = dead.id;
    game.players[PlayerId::One.index()].graveyard.push(dead);

    let host = creature(10_001, cards::SEDGE_TROLL, PlayerId::One);
    let host_id = host.card.id;
    game.battlefield.push(host);

    let pool = &mut game.players[PlayerId::One.index()].mana_pool;
    pool.black = 4;
    pool.green = 4;
    pool.colorless = 6;
    (game, dead_id, host_id)
}

fn scavenge_action(game: &Game, source: GameObjectId, host: GameObjectId) -> Option<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source: actual,
                targets,
                ..
            } => {
                *actual == source
                    && targets
                        .iter()
                        .flat_map(crate::casting::TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(host))
            }
            _ => false,
        })
}

#[test]
fn scavenging_moves_the_card_to_exile_and_grows_the_target() {
    let (mut game, dead_id, host_id) = scavenge_board(cards::DEADBRIDGE_GOLIATH);
    let action =
        scavenge_action(&game, dead_id, host_id).expect("scavenge is offered from the graveyard");
    game.apply(PlayerId::One, action)
        .expect("the ability activates");
    drain_pending(&mut game);

    let host = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == host_id)
        .expect("the target is still there");
    assert_eq!(
        host.counters(CounterKind::PlusOnePlusOne),
        5,
        "Deadbridge Goliath is a 5/5, so it leaves five counters behind"
    );
    assert!(
        game.players[PlayerId::One.index()].graveyard.is_empty(),
        "the cost took it out of the graveyard"
    );
    assert_eq!(
        game.players[PlayerId::One.index()].exile.len(),
        1,
        "and put it into exile"
    );
}

/// The power read is the exiled card's own, not the target's or a default.
#[test]
fn the_counter_count_is_the_exiled_cards_power() {
    let (mut game, dead_id, host_id) = scavenge_board(cards::SLITHERHEAD);
    let action = scavenge_action(&game, dead_id, host_id).expect("scavenge is offered");
    game.apply(PlayerId::One, action)
        .expect("the ability activates");
    drain_pending(&mut game);

    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == host_id)
            .expect("the target is still there")
            .counters(CounterKind::PlusOnePlusOne),
        1,
        "Slitherhead is a 1/1, and its scavenge costs nothing at all"
    );
}

/// "Only as a sorcery" is three conditions, and each one alone is enough to
/// close the window.
#[test]
fn scavenge_is_offered_only_when_a_sorcery_could_be_cast() {
    let (mut game, dead_id, host_id) = scavenge_board(cards::DEADBRIDGE_GOLIATH);
    assert!(
        scavenge_action(&game, dead_id, host_id).is_some(),
        "an empty stack in the controller's own main phase is the open window"
    );

    game.step = Step::Upkeep;
    assert!(
        scavenge_action(&game, dead_id, host_id).is_none(),
        "not outside a main phase"
    );

    game.step = Step::PrecombatMain;
    game.active_player = PlayerId::Two;
    assert!(
        scavenge_action(&game, dead_id, host_id).is_none(),
        "not during an opponent's turn"
    );

    game.active_player = PlayerId::One;
    let bolt = card(10_002, cards::LIGHTNING_BOLT, PlayerId::One);
    let bolt_id = bolt.id;
    game.players[PlayerId::One.index()].hand.push(bolt);
    game.players[PlayerId::One.index()].mana_pool.red = 1;
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == bolt_id))
        .expect("the Bolt is castable");
    game.apply(PlayerId::One, cast).expect("the spell is cast");
    // Casting keeps priority, so the next check really is about the stack
    // rather than about whose turn it is to act.
    assert_eq!(game.priority, PlayerId::One);
    assert!(
        scavenge_action(&game, dead_id, host_id).is_none(),
        "and not with anything else on the stack"
    );
}

#[test]
fn an_unaffordable_scavenge_is_not_offered() {
    let (mut game, dead_id, host_id) = scavenge_board(cards::DEADBRIDGE_GOLIATH);
    game.players[PlayerId::One.index()].mana_pool = ManaPool::default();
    assert!(
        scavenge_action(&game, dead_id, host_id).is_none(),
        "Deadbridge Goliath's scavenge is not free"
    );
}
