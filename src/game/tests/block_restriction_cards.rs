//! Two turn-long block clauses that point opposite ways.
//!
//! One forbids a creature from blocking at all; the other forces every
//! creature on the other side to block if it can. Both audit lines said no
//! turn-long effect could do either, and both rules were already built.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.turns_started[PlayerId::Two.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game
}

/// The blocks player two's `blocker` is offered against the declared
/// attackers.
fn offered_blocks(game: &mut Game, blocker: GameObjectId) -> Vec<GameObjectId> {
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.blockers_declared = false;
    game.priority = PlayerId::Two;
    game.legal_actions(PlayerId::Two)
        .into_iter()
        .filter_map(|action| match action {
            Action::DeclareBlocker {
                blocker: actual,
                attacker,
            } if actual == blocker => Some(attacker),
            _ => None,
        })
        .collect()
}

/// Casting a red spell takes a creature out of the blocking step.
#[test]
fn the_jester_stops_a_creature_from_blocking() {
    let mut game = ready();
    game.battlefield
        .push(creature(10_000, cards::GOBLIN_BATTLE_JESTER, PlayerId::One));
    let mut attacker = creature(10_001, cards::GRIZZLY_BEARS, PlayerId::One);
    attacker.attacking = true;
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);

    let blocker = creature(10_100, cards::GRIZZLY_BEARS, PlayerId::Two);
    let blocker_id = blocker.card.id;
    game.battlefield.push(blocker);

    assert_eq!(
        offered_blocks(&mut game, blocker_id),
        vec![attacker_id],
        "the block is on offer before the trigger",
    );

    // Lightning Bolt is a red spell, so casting it fires the Jester.
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    let spell = card(20_000, cards::LIGHTNING_BOLT, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.red = 1;
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
        .expect("one red covers a Bolt");
    game.apply(PlayerId::One, action)
        .expect("the cast is legal");
    // The trigger asks for its target, so the choice is made here rather
    // than left to whatever a blind drain would pick.
    let decision = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
        .expect("the trigger asks who cannot block");
    let option = decision
        .options
        .iter()
        .find(|option| option.card.map(|(id, _)| id) == Some(blocker_id))
        .expect("their creature is among the choices")
        .id;
    game.apply(
        decision.player,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .expect("the decision accepts what it offered");
    drain_pending(&mut game);

    assert!(
        offered_blocks(&mut game, blocker_id).is_empty(),
        "the targeted creature cannot block for the rest of the turn",
    );
}

/// The Rampage's two clauses point at opposite boards. A block requirement
/// never removes a legal block -- it removes the option of blocking nothing --
/// so what this checks is that the pump lands on one side only and the
/// requirement leaves the available block intact.
#[test]
fn the_rampage_pumps_yours_and_forces_theirs_to_block() {
    let mut game = ready();
    let mut attacker = creature(10_000, cards::GRIZZLY_BEARS, PlayerId::One);
    attacker.attacking = true;
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);

    let blocker = creature(10_100, cards::GRIZZLY_BEARS, PlayerId::Two);
    let blocker_id = blocker.card.id;
    game.battlefield.push(blocker);

    let spell = card(20_000, cards::PREDATORY_RAMPAGE, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.green = 2;
    game.players[PlayerId::One.index()].mana_pool.colorless = 3;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
        .expect("five mana covers it");
    game.apply(PlayerId::One, action)
        .expect("the cast is legal");
    drain_pending(&mut game);

    let stats = |game: &Game, id: GameObjectId| {
        let permanent = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .expect("still there");
        (game.power(permanent), game.toughness(permanent))
    };
    assert_eq!(
        stats(&game, attacker_id),
        (Some(5), Some(5)),
        "mine grew by three",
    );
    assert_eq!(
        stats(&game, blocker_id),
        (Some(2), Some(2)),
        "and theirs did not",
    );

    assert_eq!(
        offered_blocks(&mut game, blocker_id),
        vec![attacker_id],
        "their creature is still able to block the one attacker",
    );
}
