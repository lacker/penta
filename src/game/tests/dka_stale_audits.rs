//! Two Dark Ascension cards resting on machinery built for other cards.
//!
//! Thalia is the spell-cost increase Derelor introduced, widened to both
//! seats; Archangel's Light is one doubled count, gained once rather than
//! twice, and counted before the shuffle empties what it counts.

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

fn castable(game: &Game, spell: CardInstanceId) -> bool {
    game.legal_actions(PlayerId::One)
        .iter()
        .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
}

/// Lightning Bolt is {R}; under Thalia it wants {1}{R}.
#[test]
fn thalia_taxes_a_noncreature_spell() {
    let mut game = ready();
    game.battlefield.push(creature(
        10_000,
        cards::THALIA_GUARDIAN_OF_THRABEN,
        PlayerId::One,
    ));
    let spell = card(20_000, cards::LIGHTNING_BOLT, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.red = 1;
    assert!(!castable(&game, spell_id), "one red no longer covers it");

    game.players[PlayerId::One.index()].mana_pool.colorless = 1;
    assert!(castable(&game, spell_id));
}

/// A creature spell is untouched.
#[test]
fn thalia_leaves_creature_spells_alone() {
    let mut game = ready();
    game.battlefield.push(creature(
        10_000,
        cards::THALIA_GUARDIAN_OF_THRABEN,
        PlayerId::One,
    ));
    let spell = card(20_000, cards::GRIZZLY_BEARS, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.green = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;
    assert!(castable(&game, spell_id), "printed price");
}

/// The control: no Thalia, no tax.
#[test]
fn without_thalia_the_bolt_costs_one() {
    let mut game = ready();
    let spell = card(20_000, cards::LIGHTNING_BOLT, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.red = 1;
    assert!(castable(&game, spell_id));
}

/// Two per card, counted before the graveyard is shuffled away.
#[test]
fn archangels_light_gains_twice_the_graveyard_and_empties_it() {
    let mut game = ready();
    for index in 0..3 {
        game.players[PlayerId::One.index()].graveyard.push(card(
            30_000 + index,
            cards::SEDGE_TROLL,
            PlayerId::One,
        ));
    }
    let before = game.players[PlayerId::One.index()].life;

    let spell = card(20_000, cards::ARCHANGELS_LIGHT, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.white = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 7;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
        .expect("eight mana covers it");
    game.apply(PlayerId::One, action)
        .expect("the cast is legal");
    drain_pending(&mut game);

    assert_eq!(
        game.players[PlayerId::One.index()].life,
        before + 6,
        "three cards at two apiece, counted before the shuffle",
    );
    // The sweep happens while the spell is still on the stack, so what is
    // left afterwards is the Light itself and nothing else.
    let graveyard = &game.players[PlayerId::One.index()].graveyard;
    assert_eq!(graveyard.len(), 1);
    assert_eq!(
        graveyard[0].definition,
        cards::ARCHANGELS_LIGHT,
        "the three it counted went into the library",
    );
}
