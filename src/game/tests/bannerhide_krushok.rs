//! Scavenge reads "this card's power" after the card has already been exiled
//! to pay for it, so the value has to come from last-known information rather
//! than from anything still on the battlefield.

use super::*;

#[test]
fn scavenge_puts_counters_equal_to_the_exiled_cards_printed_power() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].graveyard.clear();

    let mut target = creature(22_000, cards::SAVANNAH_LIONS, PlayerId::One);
    target.entered_controller_turn = 0;
    let target_id = target.card.id;
    game.battlefield.push(target);

    let krushok = card(22_001, cards::BANNERHIDE_KRUSHOK, PlayerId::One);
    let krushok_id = krushok.id;
    game.players[0].graveyard.push(krushok);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 2);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 5);

    let scavenge = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == krushok_id)
        })
        .expect("scavenge is activatable from the graveyard");
    game.apply(PlayerId::One, scavenge)
        .expect("the scavenge cost is payable");
    for _ in 0..8 {
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let priority = game.priority;
        if game.apply(priority, Action::PassPriority).is_err() {
            break;
        }
    }

    let counters = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == target_id)
        .expect("the target is still on the battlefield")
        .counters(CounterKind::PlusOnePlusOne);
    assert_eq!(
        counters, 4,
        "the Krushok's printed 4 power survives being exiled to pay"
    );
}
