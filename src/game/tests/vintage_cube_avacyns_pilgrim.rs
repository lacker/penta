//! Avacyn's Pilgrim: a green one-drop that makes white, which is the only
//! reason a green deck plays a 1/1 for its first turn.

use super::*;

/// The Pilgrim on the battlefield, out since last turn unless the test says
/// otherwise.
fn staged() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let pilgrim = game
        .put_onto_battlefield(PlayerId::One, cards::AVACYNS_PILGRIM)
        .expect("cataloged");
    drain_pending(&mut game);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [5, 5];
    game.turn = 9;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, pilgrim)
}

/// Every colour this permanent is offering.
fn colors_of(game: &Game, source: GameObjectId) -> Vec<ManaColor> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::ActivateManaAbility {
                source: id, color, ..
            } if id == source => Some(color),
            _ => None,
        })
        .collect()
}

/// White and nothing else: the mana is not the colour that cast him, which
/// is the whole trick of the card.
#[test]
fn he_taps_for_white_alone() {
    let (game, pilgrim) = staged();

    assert_eq!(colors_of(&game, pilgrim), vec![ManaColor::White]);
}

/// A 1/1 Human Monk, which is what answers him and what counts him.
#[test]
fn he_is_a_one_one_human_monk() {
    let (game, pilgrim) = staged();

    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == pilgrim)
        .expect("he is out");
    assert_eq!(
        (game.power(permanent), game.toughness(permanent)),
        (Some(1), Some(1))
    );
    let subtypes = game.effective_subtypes(permanent);
    assert!(
        subtypes.contains(crate::card::Subtype::named("Human")),
        "a Human"
    );
    assert!(
        subtypes.contains(crate::card::Subtype::named("Monk")),
        "and a Monk"
    );
}

/// He has no haste, so the turn he arrives he makes nothing. The white is a
/// turn-two white.
#[test]
fn the_turn_he_arrives_he_makes_no_mana() {
    let (mut game, pilgrim) = staged();
    for permanent in &mut game.battlefield {
        if permanent.card.id == pilgrim {
            permanent.entered_controller_turn = game.turn;
        }
    }

    assert!(
        colors_of(&game, pilgrim).is_empty(),
        "a summoning-sick mana creature taps for nothing",
    );
}

/// The white he makes is white: it pays for a Savannah Lions on its own, and
/// the tap it cost is spent for the turn. (The cast is on offer before he is
/// tapped as well -- a mana ability may be activated while a cost is being
/// paid -- so what this pins is the mana, not the offer.)
#[test]
fn his_white_casts_a_white_one_drop_and_the_tap_is_spent() {
    let (mut game, pilgrim) = staged();
    let lions = card(99_000, cards::SAVANNAH_LIONS, PlayerId::One);
    let lions_id = lions.id;
    game.players[0].hand.push(lions);

    let tap = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateManaAbility { source, .. } if *source == pilgrim
            )
        })
        .expect("he offers his white");
    game.apply(PlayerId::One, tap).expect("it activates");
    drain_pending(&mut game);

    assert_eq!(game.players[0].mana_pool.white, 1, "one white in the pool");
    assert!(
        colors_of(&game, pilgrim).is_empty(),
        "and the tap that paid for it is spent",
    );

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == lions_id))
        .expect("one white casts the Lions");
    game.apply(PlayerId::One, cast).expect("it is cast");
    pass_until_decision(&mut game);
    drain_pending(&mut game);

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::SAVANNAH_LIONS),
        "the white he made was white enough to pay for it",
    );
}
