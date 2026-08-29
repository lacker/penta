//! A Fog that also costs both players their next untap step.
//!
//! The skip is counted per permanent rather than expressed as a duration,
//! which is what makes it right for a card reaching creatures on both sides:
//! the attacker and the defender do not arrive at their untap steps at the
//! same time, and each creature sits out its own controller's.

use super::*;

/// Player one attacking with two creatures, player two blocking one of them,
/// and Spore Cloud in player two's hand.
fn combat() -> (
    Game,
    CardInstanceId,
    GameObjectId,
    GameObjectId,
    GameObjectId,
) {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.turns_started[PlayerId::Two.index()] = 5;
    game.active_player = PlayerId::One;

    let mut attacker = creature(10_000, cards::SEDGE_TROLL, PlayerId::One);
    attacker.attacking = true;
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);

    let mut unblocked = creature(10_001, cards::SEDGE_TROLL, PlayerId::One);
    unblocked.attacking = true;
    let unblocked_id = unblocked.card.id;
    game.battlefield.push(unblocked);

    let mut blocker = creature(10_002, cards::SEDGE_TROLL, PlayerId::Two);
    blocker.blocking = vec![attacker_id];
    let blocker_id = blocker.card.id;
    game.battlefield.push(blocker);

    let spell = card(20_000, cards::SPORE_CLOUD, PlayerId::Two);
    let spell_id = spell.id;
    game.players[PlayerId::Two.index()].hand.push(spell);
    game.players[PlayerId::Two.index()].mana_pool.green = 2;
    game.players[PlayerId::Two.index()].mana_pool.colorless = 1;

    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.blockers_declared = true;
    game.priority = PlayerId::Two;
    (game, spell_id, attacker_id, unblocked_id, blocker_id)
}

fn cast_it(game: &mut Game, spell: CardInstanceId) {
    let action = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
        .expect("three mana is enough at instant speed");
    game.apply(PlayerId::Two, action)
        .expect("the cast is legal");
    drain_pending(game);
}

fn tapped(game: &Game, id: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still there")
        .tapped
}

#[test]
fn it_taps_the_blockers_and_leaves_the_attackers_alone() {
    let (mut game, spell, attacker, unblocked, blocker) = combat();
    cast_it(&mut game, spell);

    assert!(tapped(&game, blocker), "the blocker went down");
    assert!(
        !tapped(&game, attacker) && !tapped(&game, unblocked),
        "the tap clause names blockers only",
    );
}

#[test]
fn it_prevents_the_combat_damage() {
    let (mut game, spell, _attacker, _unblocked, blocker) = combat();
    cast_it(&mut game, spell);

    game.step = Step::CombatDamage;
    game.deal_combat_damage();
    drain_pending(&mut game);

    assert_eq!(
        game.players[PlayerId::Two.index()].life,
        i16::from(rules::STARTING_LIFE),
        "the unblocked attacker's damage was prevented too",
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == blocker),
        "and nothing traded",
    );
}

/// The defender's own creature misses the defender's next untap step, which
/// arrives before the attacker's does.
#[test]
fn the_blocker_sits_out_its_controllers_next_untap_step() {
    let (mut game, spell, _attacker, _unblocked, blocker) = combat();
    cast_it(&mut game, spell);
    assert!(tapped(&game, blocker));

    game.commit_next_turn(PlayerId::Two, Vec::new());
    drain_pending(&mut game);
    assert!(tapped(&game, blocker), "held down on the turn it skipped");

    game.commit_next_turn(PlayerId::One, Vec::new());
    game.commit_next_turn(PlayerId::Two, Vec::new());
    drain_pending(&mut game);
    assert!(
        !tapped(&game, blocker),
        "and back up the turn after, having skipped exactly one",
    );
}

/// Both attackers skip too, counted against their own controller's step
/// rather than the caster's.
#[test]
fn the_attackers_sit_out_their_controllers_next_untap_step() {
    let (mut game, spell, attacker, unblocked, _blocker) = combat();
    cast_it(&mut game, spell);
    for id in [attacker, unblocked] {
        game.battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == id)
            .expect("still there")
            .tapped = true;
    }

    game.commit_next_turn(PlayerId::Two, Vec::new());
    drain_pending(&mut game);
    game.commit_next_turn(PlayerId::One, Vec::new());
    drain_pending(&mut game);

    assert!(tapped(&game, attacker), "the blocked attacker stayed down");
    assert!(tapped(&game, unblocked), "and so did the unblocked one");
}

/// The control: a creature standing outside the combat is untouched by every
/// clause.
#[test]
fn a_creature_outside_combat_is_not_affected() {
    let (mut game, spell, _attacker, _unblocked, _blocker) = combat();
    let bystander = creature(10_100, cards::SEDGE_TROLL, PlayerId::Two);
    let bystander_id = bystander.card.id;
    game.battlefield.push(bystander);
    cast_it(&mut game, spell);
    assert!(!tapped(&game, bystander_id));

    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == bystander_id)
        .expect("still there")
        .tapped = true;
    game.commit_next_turn(PlayerId::Two, Vec::new());
    drain_pending(&mut game);

    assert!(
        !tapped(&game, bystander_id),
        "it untapped on schedule, skipping nothing",
    );
}
