//! Turning a creature sideways for mana.
//!
//! The amount is not known until the spell resolves, so it is read off the
//! creature that was tapped rather than printed. The second instruction still
//! finds that creature after the first has tapped it, which is the part worth
//! checking: the target predicate names an untapped creature, and by then it
//! is not one.

use super::*;

/// Energy Tap in player one's hand with `creature` on their battlefield.
fn ready_to_tap(creature_definition: CardDefinitionId) -> (Game, GameObjectId, CardInstanceId) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    let subject = creature(10_000, creature_definition, PlayerId::One);
    let subject_id = subject.card.id;
    game.battlefield.push(subject);

    let spell = card(20_000, cards::ENERGY_TAP, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.blue = 1;
    (game, subject_id, spell_id)
}

fn cast_it(game: &mut Game, spell: CardInstanceId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
        .expect("an untapped creature to tap");
    game.apply(PlayerId::One, action)
        .expect("the cast is legal");
    drain_pending(game);
}

#[test]
fn it_taps_the_creature_and_pays_out_its_mana_value() {
    // Sedge Troll costs {2}{R}, so three.
    let (mut game, subject, spell) = ready_to_tap(cards::SEDGE_TROLL);
    cast_it(&mut game, spell);

    let subject = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == subject)
        .expect("still there");
    assert!(subject.tapped);
    assert_eq!(
        game.players[PlayerId::One.index()].mana_pool.colorless,
        3,
        "the mana followed the tap, read off the creature",
    );
}

/// A different cost pays differently, which is what tells a read-off value
/// from a printed one.
#[test]
fn a_costlier_creature_pays_more() {
    // Serra Angel costs {3}{W}{W}, so five.
    let (mut game, _subject, spell) = ready_to_tap(cards::SERRA_ANGEL);
    cast_it(&mut game, spell);

    assert_eq!(game.players[PlayerId::One.index()].mana_pool.colorless, 5);
}

/// The control: it names an untapped creature, so a tapped one is not a legal
/// target and the spell is not castable at all.
#[test]
fn a_tapped_creature_is_not_a_legal_target() {
    let (mut game, subject, spell) = ready_to_tap(cards::SEDGE_TROLL);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == subject)
        .expect("still there")
        .tapped = true;

    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell)),
    );
}
