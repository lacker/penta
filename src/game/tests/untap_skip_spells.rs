//! Two spells that tap and then hold down.
//!
//! Both audit lines asked for a *duration* tied to the affected creature's
//! controller's untap step. A duration is the wrong shape: Frost Breath can
//! reach creatures on both sides, and the two players do not arrive at their
//! untap steps together. Counting a skip on each creature is what makes that
//! come out right.

use super::*;

/// `spell` in player one's hand with plenty of mana, one creature apiece.
fn holding(spell: CardDefinitionId) -> (Game, CardInstanceId, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.turns_started[PlayerId::Two.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;

    let mine = creature(10_000, cards::GRIZZLY_BEARS, PlayerId::One);
    let mine_id = mine.card.id;
    game.battlefield.push(mine);
    let theirs = creature(10_001, cards::GRIZZLY_BEARS, PlayerId::Two);
    let theirs_id = theirs.card.id;
    game.battlefield.push(theirs);

    let card_in_hand = card(20_000, spell, PlayerId::One);
    let spell_id = card_in_hand.id;
    game.players[PlayerId::One.index()].hand.push(card_in_hand);
    game.players[PlayerId::One.index()].mana_pool.blue = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;
    game.priority = PlayerId::One;
    (game, spell_id, mine_id, theirs_id)
}

fn cast_at(game: &mut Game, spell: CardInstanceId, targets: &[GameObjectId]) {
    let wanted = targets
        .iter()
        .map(|id| Target::Permanent(*id))
        .collect::<Vec<_>>();
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == spell
                    && choices.targets().iter().any(|slot| slot.targets() == wanted.as_slice()))
        })
        .expect("the declaration is on offer");
    game.apply(PlayerId::One, action)
        .expect("three mana is enough");
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
fn crippling_chill_taps_holds_and_draws() {
    let (mut game, spell, _mine, theirs) = holding(cards::CRIPPLING_CHILL);
    let before = game.players[PlayerId::One.index()].hand.len();
    cast_at(&mut game, spell, &[theirs]);

    assert!(tapped(&game, theirs), "tapped on resolution");
    assert_eq!(
        game.players[PlayerId::One.index()].hand.len(),
        before - 1 + 1,
        "the spell left hand and a card was drawn",
    );

    game.commit_next_turn(PlayerId::Two, Vec::new());
    drain_pending(&mut game);
    assert!(tapped(&game, theirs), "it skipped their untap step");

    game.commit_next_turn(PlayerId::One, Vec::new());
    game.commit_next_turn(PlayerId::Two, Vec::new());
    drain_pending(&mut game);
    assert!(
        !tapped(&game, theirs),
        "and came back the turn after, having skipped exactly one",
    );
}

/// The point of counting per creature: two creatures under different players
/// each skip their own controller's step.
#[test]
fn frost_breath_holds_both_sides_on_their_own_schedules() {
    let (mut game, spell, mine, theirs) = holding(cards::FROST_BREATH);
    cast_at(&mut game, spell, &[mine, theirs]);
    assert!(tapped(&game, mine) && tapped(&game, theirs));

    // Their untap step comes first and is the one their creature skips.
    game.commit_next_turn(PlayerId::Two, Vec::new());
    drain_pending(&mut game);
    assert!(tapped(&game, theirs), "their creature skipped their step");
    assert!(tapped(&game, mine), "mine has not reached its step yet");

    // Then mine, which skips its own.
    game.commit_next_turn(PlayerId::One, Vec::new());
    drain_pending(&mut game);
    assert!(tapped(&game, mine), "and skips it when it arrives");
    assert!(tapped(&game, theirs), "theirs waits a full cycle to untap");

    // A cycle later each is back, in the same order the steps arrived.
    game.commit_next_turn(PlayerId::Two, Vec::new());
    drain_pending(&mut game);
    assert!(!tapped(&game, theirs), "their step spent the skip");
    assert!(tapped(&game, mine), "mine is still waiting for its own");

    game.commit_next_turn(PlayerId::One, Vec::new());
    drain_pending(&mut game);
    assert!(!tapped(&game, mine));
}

/// "Up to two", so one target is a legal declaration and the other creature
/// is untouched.
#[test]
fn frost_breath_may_take_a_single_target() {
    let (mut game, spell, mine, theirs) = holding(cards::FROST_BREATH);
    cast_at(&mut game, spell, &[theirs]);

    assert!(tapped(&game, theirs));
    assert!(!tapped(&game, mine), "the one left alone stayed up");
}
