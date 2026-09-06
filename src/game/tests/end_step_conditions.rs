//! An intervening-if counted at the end step, and an optional payment
//! offered by somebody else's spell. Neither leaves a mark when the
//! condition is false, so each is checked in both directions.

use super::*;

/// Well of Life with `untapped` untapped Forests and `tapped` tapped ones,
/// run through player one's end step.
fn end_step(untapped: usize, tapped: usize) -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let well = creature(88_000, cards::WELL_OF_LIFE, PlayerId::One);
    game.battlefield.push(well);
    for index in 0..(untapped + tapped) {
        let mut forest = creature(
            88_100 + u32::try_from(index).expect("a small fixture"),
            cards::FOREST,
            PlayerId::One,
        );
        forest.entered_controller_turn = 0;
        forest.tapped = index >= untapped;
        game.battlefield.push(forest);
    }
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;
    game.step = Step::PostcombatMain;
    game.advance_step();
    game.finish_rules_procedure();
    for _ in 0..8 {
        drain_pending(&mut game);
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let holder = game.priority;
        if game.apply(holder, Action::PassPriority).is_err() {
            break;
        }
    }
    game
}

#[test]
fn an_untapped_land_switches_the_well_off() {
    assert_eq!(
        end_step(1, 2).players[0].life,
        20,
        "one land still untapped, so no life"
    );
}

#[test]
fn spending_every_land_pays_two_life() {
    assert_eq!(
        end_step(0, 3).players[0].life,
        22,
        "every land tapped is the condition the Well wants"
    );
}

#[test]
fn no_lands_at_all_also_counts_as_none_untapped() {
    assert_eq!(
        end_step(0, 0).players[0].life,
        22,
        "zero untapped lands is zero either way"
    );
}

/// The Onyx Talisman on player one's side with a black spell of player
/// two's just cast, and a tapped Forest of player one's to untap.
fn talisman_trigger(mana: u16) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[1].hand.clear();
    let talisman = creature(88_200, cards::ONYX_TALISMAN, PlayerId::One);
    game.battlefield.push(talisman);
    let mut forest = creature(88_201, cards::FOREST, PlayerId::One);
    forest.entered_controller_turn = 0;
    forest.tapped = true;
    let forest_id = forest.card.id;
    game.battlefield.push(forest);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, mana);

    let spell = card(88_300, cards::DARK_RITUAL, PlayerId::Two);
    let spell_id = spell.id;
    game.players[1].hand.push(spell);
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Black, 1);
    game.active_player = PlayerId::Two;
    game.turns_started[PlayerId::Two.index()] = 5;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::Two;
    let cast = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
        .expect("the Ritual is castable");
    game.apply(PlayerId::Two, cast).expect("the cast is legal");
    (game, forest_id)
}

#[test]
fn the_talisman_offers_its_payment_on_an_opponents_spell() {
    let (mut game, forest) = talisman_trigger(3);
    for _ in 0..8 {
        if game.observe(PlayerId::One).decision.is_some() {
            break;
        }
        let holder = game.priority;
        if game.apply(holder, Action::PassPriority).is_err() {
            break;
        }
    }
    // The target is chosen as the trigger goes on the stack; the payment is
    // offered only once it resolves.
    choose_decision_by_label(&mut game, PlayerId::One, "Forest");
    for _ in 0..8 {
        if game.observe(PlayerId::One).decision.is_some() {
            break;
        }
        let holder = game.priority;
        if game.apply(holder, Action::PassPriority).is_err() {
            break;
        }
    }
    choose_decision_by_label(&mut game, PlayerId::One, "Pay the cost");
    drain_pending(&mut game);

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == forest && !permanent.tapped),
        "paying the {{3}} untapped the Forest"
    );
}

#[test]
fn with_no_mana_there_is_nothing_to_offer() {
    let (mut game, forest) = talisman_trigger(0);
    for _ in 0..8 {
        drain_pending(&mut game);
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let holder = game.priority;
        if game.apply(holder, Action::PassPriority).is_err() {
            break;
        }
    }

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == forest && permanent.tapped),
        "nothing to pay with, so the Forest stays tapped"
    );
}
