//! Descendant of Storms, and endure: a choice between a counter and a body,
//! made as the attack trigger resolves.

use super::*;

/// A declare-attackers step with the Descendant able to attack and enough
/// mana to pay for the trigger.
fn staged(mana: u16) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    let descendant = game
        .put_onto_battlefield(PlayerId::One, cards::DESCENDANT_OF_STORMS)
        .expect("cataloged");
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [2, 1];
    drain_pending(&mut game);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, mana);
    game.active_player = PlayerId::One;
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.priority = PlayerId::One;
    (game, descendant)
}

/// Attacks, finishes the declaration, and answers the "you may pay" offer.
fn attack_and_answer_payment(game: &mut Game, descendant: GameObjectId, pay: bool) {
    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker: descendant,
            defender: AttackDefender::Player(PlayerId::Two),
        },
    )
    .expect("it attacks");
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .expect("the declaration finishes");
    pass_until_decision(game);
    let offer = game
        .observe(PlayerId::One)
        .decision
        .expect("the attack trigger offers the payment");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: offer.id,
            options: vec![u32::from(pay)],
        },
    )
    .expect("answering the offer is legal");
}

fn spirits(game: &Game) -> usize {
    game.battlefield
        .iter()
        .filter(|permanent| {
            permanent.controller == PlayerId::One
                && game
                    .effective_subtypes(permanent)
                    .contains(crate::card::Subtype::named("Spirit"))
        })
        .count()
}

fn counters_on(game: &Game, id: GameObjectId) -> u16 {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .map_or(0, |permanent| {
            permanent.counters(CounterKind::PlusOnePlusOne)
        })
}

/// Declining the payment ends the clause: no counter and no Spirit.
#[test]
fn declining_the_payment_does_nothing() {
    let (mut game, descendant) = staged(2);

    attack_and_answer_payment(&mut game, descendant, false);
    drain_pending(&mut game);

    assert_eq!(counters_on(&game, descendant), 0);
    assert_eq!(spirits(&game), 0);
    assert_eq!(
        game.players[0].mana_pool.total(),
        2,
        "and the mana is still there",
    );
}

/// Paying offers the two halves of endure, and nothing else.
#[test]
fn paying_offers_the_counter_or_the_spirit() {
    let (mut game, descendant) = staged(2);

    attack_and_answer_payment(&mut game, descendant, true);

    let endure = game
        .observe(PlayerId::One)
        .decision
        .expect("endure asks which half");
    assert_eq!(endure.options.len(), 2, "{:?}", endure.options);
    assert_eq!(
        game.players[0].mana_pool.total(),
        0,
        "two mana bought the choice",
    );
}

/// The counters half puts a +1/+1 counter on the attacker.
#[test]
fn endure_can_put_the_counter_on_it() {
    let (mut game, descendant) = staged(2);
    attack_and_answer_payment(&mut game, descendant, true);

    let endure = game
        .observe(PlayerId::One)
        .decision
        .expect("endure asks which half");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: endure.id,
            options: vec![0],
        },
    )
    .expect("the counter is a legal choice");
    drain_pending(&mut game);

    assert_eq!(counters_on(&game, descendant), 1);
    assert_eq!(spirits(&game), 0, "one half or the other, never both");
}

/// The token half makes a 1/1 white Spirit instead.
#[test]
fn endure_can_make_the_spirit_instead() {
    let (mut game, descendant) = staged(2);
    attack_and_answer_payment(&mut game, descendant, true);

    let endure = game
        .observe(PlayerId::One)
        .decision
        .expect("endure asks which half");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: endure.id,
            options: vec![1],
        },
    )
    .expect("the Spirit is a legal choice");
    drain_pending(&mut game);

    assert_eq!(counters_on(&game, descendant), 0);
    assert_eq!(spirits(&game), 1);
    let spirit = game
        .battlefield
        .iter()
        .find(|permanent| {
            game.effective_subtypes(permanent)
                .contains(crate::card::Subtype::named("Spirit"))
        })
        .expect("the Spirit is there");
    assert_eq!(game.power(spirit), Some(1));
    assert_eq!(game.toughness(spirit), Some(1));
}

/// Nothing is offered without the mana to pay for it.
#[test]
fn the_offer_needs_the_mana() {
    let (mut game, descendant) = staged(0);
    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker: descendant,
            defender: AttackDefender::Player(PlayerId::Two),
        },
    )
    .expect("it attacks");
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .expect("the declaration finishes");
    drain_pending(&mut game);

    assert_eq!(counters_on(&game, descendant), 0);
    assert_eq!(spirits(&game), 0);
}
