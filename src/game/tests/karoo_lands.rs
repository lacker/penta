//! The Karoo cycle: a land that pays for its second mana by handing back an
//! untapped land of the right type. The payment is what needs covering --
//! a tapped land must not count, and a land of the wrong type must not
//! either, or the whole drawback disappears.

use super::*;

/// Karoo played from hand with `untapped` untapped Plains, `tapped` tapped
/// ones, and `forests` Forests standing by as the wrong type.
fn play_karoo(untapped: usize, tapped: usize, forests: usize, pay: bool) -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    for index in 0..(untapped + tapped) {
        let mut plains = creature(
            94_000 + u32::try_from(index).expect("a small fixture"),
            cards::PLAINS,
            PlayerId::One,
        );
        plains.entered_controller_turn = 0;
        plains.tapped = index >= untapped;
        game.battlefield.push(plains);
    }
    for index in 0..forests {
        let mut forest = creature(
            94_100 + u32::try_from(index).expect("a small fixture"),
            cards::FOREST,
            PlayerId::One,
        );
        forest.entered_controller_turn = 0;
        game.battlefield.push(forest);
    }
    let karoo = card(94_200, cards::KAROO, PlayerId::One);
    let karoo_id = karoo.id;
    game.players[0].hand.push(karoo);
    let land_drop = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::PlayLand { card, .. } if *card == karoo_id))
        .expect("the land drop is available");
    game.apply(PlayerId::One, land_drop)
        .expect("the land is played");

    // The payment names what it takes rather than printing a generic
    // "Pay the cost", so the option is the land being handed back.
    let label = if pay { "Return Plains" } else { "Decline" };
    for _ in 0..16 {
        let payer = [PlayerId::One, PlayerId::Two].into_iter().find(|player| {
            game.observe(*player)
                .decision
                .is_some_and(|decision| decision.options.iter().any(|o| o.label == label))
        });
        if let Some(payer) = payer {
            choose_decision_by_label(&mut game, payer, label);
            continue;
        }
        if !game.pending_decisions.is_empty() {
            drain_pending(&mut game);
            continue;
        }
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

fn karoo_survives(game: &Game) -> bool {
    game.battlefield
        .iter()
        .any(|permanent| permanent.card.definition == ObjectKind::Card(cards::KAROO))
}

fn plains_on_board(game: &Game) -> usize {
    game.battlefield
        .iter()
        .filter(|permanent| permanent.card.definition == ObjectKind::Card(cards::PLAINS))
        .count()
}

#[test]
fn an_untapped_plains_pays_for_it() {
    let game = play_karoo(1, 0, 0, true);
    assert!(karoo_survives(&game), "the Karoo stayed");
    assert_eq!(
        plains_on_board(&game),
        0,
        "and the Plains went back to hand"
    );
}

#[test]
fn a_tapped_plains_is_not_enough() {
    let game = play_karoo(0, 2, 0, true);
    assert!(
        !karoo_survives(&game),
        "two tapped Plains cannot pay an untapped cost"
    );
    assert_eq!(plains_on_board(&game), 2, "and neither of them moved");
}

/// The labels the entry payment offers, before anything answers it.
fn offered_payment(untapped: usize, tapped: usize, forests: usize) -> Vec<String> {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    for index in 0..(untapped + tapped) {
        let mut plains = creature(
            94_300 + u32::try_from(index).expect("a small fixture"),
            cards::PLAINS,
            PlayerId::One,
        );
        plains.entered_controller_turn = 0;
        plains.tapped = index >= untapped;
        game.battlefield.push(plains);
    }
    for index in 0..forests {
        let mut forest = creature(
            94_400 + u32::try_from(index).expect("a small fixture"),
            cards::FOREST,
            PlayerId::One,
        );
        forest.entered_controller_turn = 0;
        game.battlefield.push(forest);
    }
    let karoo = card(94_500, cards::KAROO, PlayerId::One);
    let karoo_id = karoo.id;
    game.players[0].hand.push(karoo);
    let land_drop = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::PlayLand { card, .. } if *card == karoo_id))
        .expect("the land drop is available");
    game.apply(PlayerId::One, land_drop)
        .expect("the land is played");
    for _ in 0..8 {
        if game.observe(PlayerId::One).decision.is_some() {
            break;
        }
        let holder = game.priority;
        if game.apply(holder, Action::PassPriority).is_err() {
            break;
        }
    }
    game.observe(PlayerId::One)
        .decision
        .map(|decision| {
            decision
                .options
                .iter()
                .map(|option| option.label.clone())
                .collect()
        })
        .unwrap_or_default()
}

#[test]
fn only_an_untapped_plains_is_ever_offered() {
    assert!(
        offered_payment(1, 0, 0)
            .iter()
            .any(|l| l == "Return Plains"),
        "an untapped Plains may be handed back"
    );
    assert!(
        offered_payment(0, 0, 2)
            .iter()
            .all(|label| label == "Decline"),
        "two untapped Forests offer nothing to return"
    );
    assert!(
        offered_payment(0, 2, 0)
            .iter()
            .all(|label| label == "Decline"),
        "and neither do two tapped Plains"
    );
}

#[test]
fn declining_sacrifices_it_even_with_a_plains_out() {
    let game = play_karoo(1, 0, 0, false);
    assert!(!karoo_survives(&game), "the payment was refused");
    assert_eq!(plains_on_board(&game), 1, "so the Plains stayed put");
}
