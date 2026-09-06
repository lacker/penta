//! Countering a spell unless its controller pays, with the source spent as
//! the cost. Both halves of the "unless" matter, and so does the fact that
//! the creature is already gone: the tax is charged and the counter happens
//! from a permanent that left the battlefield to pay for the activation.

use super::*;

/// Player two casting Lightning Bolt into player one's Spiketail Hatchling.
fn staged(payer_mana: u16) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[1].hand.clear();
    let mut hatchling = creature(58_000, cards::SPIKETAIL_HATCHLING, PlayerId::One);
    hatchling.entered_controller_turn = 0;
    let hatchling_id = hatchling.card.id;
    game.battlefield.push(hatchling);

    let bolt = card(58_010, cards::LIGHTNING_BOLT, PlayerId::Two);
    game.players[1].hand.push(bolt.clone());
    game.players[1].mana_pool.red = 1;
    game.players[1].mana_pool.colorless = payer_mana;
    game.priority = PlayerId::Two;
    let cast = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == bolt.id))
        .expect("the Bolt is castable");
    game.apply(PlayerId::Two, cast).expect("the cast is legal");
    // The caster holds priority first; the Hatchling's controller responds.
    game.apply(PlayerId::Two, Action::PassPriority)
        .expect("the caster passes");
    (game, hatchling_id)
}

fn sacrifice_the_hatchling(game: &mut Game, hatchling: GameObjectId) {
    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == hatchling)
        })
        .expect("there is a spell on the stack to point it at");
    game.apply(PlayerId::One, activation)
        .expect("sacrificing pays for it");
}

#[test]
fn paying_the_tax_saves_the_spell_and_the_hatchling_is_still_spent() {
    let (mut game, hatchling) = staged(1);
    sacrifice_the_hatchling(&mut game, hatchling);
    assert!(
        game.battlefield.is_empty(),
        "the Hatchling left to pay for its own activation"
    );

    // The tax is optional, and declining is offered first -- answering with
    // the first option would silently measure the counter branch instead.
    pass_priority_pair(&mut game);
    choose_decision_by_label(&mut game, PlayerId::Two, "Pay the cost");
    drain_pending(&mut game);
    pass_priority_pair(&mut game);
    assert_eq!(
        game.players[0].life, 17,
        "the tax was paid, so the Bolt resolved"
    );
    assert_eq!(game.players[1].mana_pool.colorless, 0, "and it cost a mana");
}

#[test]
fn an_unpayable_tax_counters_the_spell() {
    let (mut game, hatchling) = staged(0);
    sacrifice_the_hatchling(&mut game, hatchling);
    pass_priority_pair(&mut game);

    // With nothing to pay with the choice is not even offered, which is what
    // makes this the unpayable case rather than a declined one.
    assert!(
        !game
            .observe(PlayerId::Two)
            .decision
            .is_some_and(|decision| decision
                .options
                .iter()
                .any(|option| option.label == "Pay the cost")),
        "no payment is on offer"
    );
    drain_pending(&mut game);
    pass_priority_pair(&mut game);

    assert_eq!(game.players[0].life, 20, "so the Bolt was countered");
    assert_eq!(
        game.players[1].graveyard.len(),
        1,
        "and it went to the graveyard"
    );
}
