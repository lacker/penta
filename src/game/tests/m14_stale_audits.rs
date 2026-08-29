//! Two Magic 2014 cards whose audit lines named machinery that existed.
//!
//! Congregate wanted a doubled object count, which `Scaled` has always done;
//! Wall of Frost wanted the identity of the creature it blocked, which the
//! block trigger already carries as its own object.

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

fn tapped(game: &Game, id: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still there")
        .tapped
}

/// Two per creature, counting both sides of the table.
#[test]
fn congregate_gains_two_for_each_creature_anywhere() {
    let mut game = ready();
    for index in 0..2 {
        game.battlefield.push(creature(
            10_000 + index,
            cards::GRIZZLY_BEARS,
            PlayerId::One,
        ));
    }
    game.battlefield
        .push(creature(10_100, cards::GRIZZLY_BEARS, PlayerId::Two));

    let spell = card(20_000, cards::CONGREGATE, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.white = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 3;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == spell_id
                    && choices.targets().iter().any(|slot| slot.targets() == [Target::Player(PlayerId::One)]))
        })
        .expect("its own controller is a legal target");
    game.apply(PlayerId::One, action)
        .expect("four mana covers it");
    drain_pending(&mut game);

    assert_eq!(
        game.players[PlayerId::One.index()].life,
        i16::from(rules::STARTING_LIFE) + 6,
        "three creatures at two apiece, theirs included",
    );
}

/// The control: no creatures, no life.
#[test]
fn congregate_gains_nothing_on_an_empty_board() {
    let mut game = ready();
    let spell = card(20_000, cards::CONGREGATE, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.white = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 3;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
        .expect("castable with no creatures out");
    game.apply(PlayerId::One, action)
        .expect("four mana covers it");
    drain_pending(&mut game);

    assert_eq!(
        game.players[PlayerId::One.index()].life,
        i16::from(rules::STARTING_LIFE),
    );
}

/// The Wall holds down the creature it blocked, and only that one.
#[test]
fn wall_of_frost_holds_the_creature_it_blocked() {
    let mut game = ready();
    game.active_player = PlayerId::Two;

    let mut attacker = creature(10_000, cards::GRIZZLY_BEARS, PlayerId::Two);
    attacker.attacking = true;
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);

    let mut other = creature(10_001, cards::GRIZZLY_BEARS, PlayerId::Two);
    other.attacking = true;
    let other_id = other.card.id;
    game.battlefield.push(other);

    let wall = creature(10_100, cards::WALL_OF_FROST, PlayerId::One);
    let wall_id = wall.card.id;
    game.battlefield.push(wall);

    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.blockers_declared = false;
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::DeclareBlocker { blocker, attacker }
                if *blocker == wall_id && *attacker == attacker_id)
        })
        .expect("a Wall may block");
    game.apply(PlayerId::One, action)
        .expect("the block is legal");
    // The trigger fires as the declaration closes, not on the individual
    // block, so the declaration has to be finished rather than the flag set.
    game.finish_declaring_blockers();
    drain_pending(&mut game);

    // Both attackers tapped when they attacked; only the blocked one is held.
    for id in [attacker_id, other_id] {
        game.battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == id)
            .expect("still there")
            .tapped = true;
    }

    game.commit_next_turn(PlayerId::Two, Vec::new());
    drain_pending(&mut game);

    assert!(tapped(&game, attacker_id), "the one it blocked stayed down");
    assert!(!tapped(&game, other_id), "the other untapped as usual");
}
