//! Two enters triggers that reach "each other creature". The exclusion is
//! the whole card in both cases: a Hellion that swept itself would die to its
//! own trigger, and a Mogg that tapped itself could not attack with the
//! opening it just made.

use super::*;

/// `definition` cast from hand, with `others` creatures already out.
fn arriving(definition: CardDefinitionId, others: &[(CardDefinitionId, PlayerId)]) -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    for (index, (card_definition, controller)) in others.iter().enumerate() {
        let mut other = creature(
            59_100 + u32::try_from(index).expect("a small fixture"),
            *card_definition,
            *controller,
        );
        other.entered_controller_turn = 0;
        game.battlefield.push(other);
    }
    let spell = card(59_000, definition, PlayerId::One);
    let spell_id = spell.id;
    game.players[0].hand.push(spell);
    game.players[0].mana_pool.red = 6;
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
        .expect("it is castable");
    game.apply(PlayerId::One, cast).expect("the cast is legal");
    pass_priority_pair(&mut game);
    drain_pending(&mut game);
    game.check_state_based_actions();
    game
}

fn surviving(game: &Game, definition: CardDefinitionId) -> Option<&Permanent> {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == ObjectKind::Card(definition))
}

#[test]
fn the_hellion_sweeps_everything_except_itself() {
    let game = arriving(
        cards::CRATER_HELLION,
        &[
            (cards::GRIZZLY_BEARS, PlayerId::One),
            (cards::SERRA_ANGEL, PlayerId::Two),
        ],
    );

    assert_eq!(
        game.battlefield.len(),
        1,
        "both other creatures died, its own controller's included"
    );
    assert_eq!(
        surviving(&game, cards::CRATER_HELLION)
            .expect("the Hellion is there")
            .damage,
        0,
        "and it took none of the four itself"
    );
}

#[test]
fn the_mogg_taps_everything_except_itself() {
    let game = arriving(
        cards::SHRIEKING_MOGG,
        &[
            (cards::GRIZZLY_BEARS, PlayerId::One),
            (cards::GRIZZLY_BEARS, PlayerId::Two),
        ],
    );

    let mogg = surviving(&game, cards::SHRIEKING_MOGG).expect("the Mogg arrived");
    assert!(!mogg.tapped, "it left itself untapped, which is the point");
    assert!(
        game.battlefield
            .iter()
            .filter(|permanent| permanent.card.definition == ObjectKind::Card(cards::GRIZZLY_BEARS))
            .all(|permanent| permanent.tapped),
        "and tapped both of the others, whoever controls them"
    );
}
