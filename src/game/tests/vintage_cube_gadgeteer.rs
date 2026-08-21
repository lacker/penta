//! Forensic Gadgeteer: Clues off every artifact, and a discount with a
//! floor under it.

use super::*;

fn settle(game: &mut Game) {
    for _ in 0..16 {
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
}

fn staged() -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.put_onto_battlefield(PlayerId::One, cards::FORENSIC_GADGETEER)
        .expect("cataloged");
    drain_pending(&mut game);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game
}

fn clues(game: &Game) -> usize {
    game.battlefield
        .iter()
        .filter(|permanent| is_token_with(permanent, tokens::clue()))
        .count()
}

/// Casting an artifact investigates; casting anything else does not.
#[test]
fn every_artifact_spell_leaves_a_clue_behind() {
    let mut game = staged();
    let lotus = card(85_000, cards::BLACK_LOTUS, PlayerId::One);
    let lotus_id = lotus.id;
    game.players[0].hand.push(lotus);

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == lotus_id))
        .expect("a free artifact is castable");
    game.apply(PlayerId::One, cast).expect("it is cast");
    settle(&mut game);
    drain_pending(&mut game);

    assert_eq!(clues(&game), 1, "the artifact left a Clue");
}

#[test]
fn a_nonartifact_spell_leaves_nothing() {
    let mut game = staged();
    let bolt = card(85_010, cards::LIGHTNING_BOLT, PlayerId::One);
    let bolt_id = bolt.id;
    game.players[0].hand.push(bolt);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == bolt_id))
        .expect("the Bolt is castable");
    game.apply(PlayerId::One, cast).expect("it is cast");
    settle(&mut game);
    drain_pending(&mut game);

    assert_eq!(clues(&game), 0, "an instant is not an artifact");
}

/// The Clue's own ability costs {2}; with the Gadgeteer out it costs {1}.
#[test]
fn the_discount_takes_a_clue_from_two_mana_to_one() {
    let mut game = staged();
    game.create_token(PlayerId::One, tokens::clue());
    drain_pending(&mut game);
    let clue = game
        .battlefield
        .iter()
        .find(|permanent| is_token_with(permanent, tokens::clue()))
        .expect("the Clue token arrived")
        .card
        .id;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);

    let activate = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == clue))
        .expect("one mana is enough with the discount");
    let before = game.players[0].library.len();
    game.apply(PlayerId::One, activate)
        .expect("the ability activates");
    settle(&mut game);
    drain_pending(&mut game);

    assert_eq!(
        game.players[0].library.len(),
        before - 1,
        "the Clue was cashed in",
    );
    assert_eq!(game.players[0].mana_pool.total(), 0, "for exactly one mana");
}

/// Without the Gadgeteer the same one mana is not enough.
#[test]
fn one_mana_is_not_enough_without_the_discount() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.create_token(PlayerId::One, tokens::clue());
    drain_pending(&mut game);
    let clue = game
        .battlefield
        .iter()
        .find(|permanent| is_token_with(permanent, tokens::clue()))
        .expect("the Clue token arrived")
        .card
        .id;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);

    assert!(
        !game.legal_actions(PlayerId::One).iter().any(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == clue)
        ),
        "the Clue still asks for two",
    );
}

/// The floor holds: an ability already at one mana is not made free.
#[test]
fn the_discount_stops_at_one_mana() {
    let mut game = staged();
    // Zuran Orb's ability costs nothing but a sacrifice, so there is no
    // mana in it to reduce; the Mox's tap is likewise free. A one-mana
    // artifact ability is the case the floor is written for.
    let ingot = game
        .put_onto_battlefield(PlayerId::One, cards::DARKSTEEL_INGOT)
        .expect("cataloged");
    drain_pending(&mut game);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    let cost = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == ingot)
        .map(|permanent| game.ability_mana_cost(permanent, crate::mana_cost!("{1}")))
        .expect("the Ingot is there");

    assert_eq!(
        cost.mana_value(),
        1,
        "a one-mana ability is left where it is",
    );
}
