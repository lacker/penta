//! Guide of Souls: every other creature is a life and an energy, and three
//! energy turns an attacker into a flying Angel for good.

use super::*;

/// The Guide and a bear on the battlefield since last turn, in Player One's
/// precombat main phase with `energy` already banked.
fn staged(energy: u16) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let guide = game
        .put_onto_battlefield(PlayerId::One, cards::GUIDE_OF_SOULS)
        .expect("cataloged");
    let bears = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [2, 1];
    drain_pending(&mut game);
    game.players[0].life = 20;
    game.players[0]
        .counters
        .set(CounterKind::named("energy"), energy);
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, guide, bears)
}

fn energy(game: &Game) -> u16 {
    game.players[0].counters.count(CounterKind::named("energy"))
}

fn permanent(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("it is on the battlefield")
}

/// Attacks with `attackers`, names the first legal target for the trigger,
/// and answers the offer to pay the energy.
fn attack_and_answer(game: &mut Game, attackers: &[GameObjectId], pay: bool) {
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    for attacker in attackers {
        game.apply(
            PlayerId::One,
            Action::DeclareAttacker {
                attacker: *attacker,
                defender: AttackDefender::Player(PlayerId::Two),
            },
        )
        .expect("it attacks");
    }
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .expect("the declaration finishes");
    pass_until_decision(game);
    let targeting = game
        .observe(PlayerId::One)
        .decision
        .expect("the trigger asks for its target");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: targeting.id,
            options: vec![targeting.options[0].id],
        },
    )
    .expect("naming an attacker is legal");
    pass_until_decision(game);
    let offer = game
        .observe(PlayerId::One)
        .decision
        .expect("the attack trigger offers the energy");
    let chosen = if pay {
        vec![
            offer
                .options
                .iter()
                .find(|option| option.label != "Decline")
                .expect("paying is on offer")
                .id,
        ]
    } else {
        vec![0]
    };
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: offer.id,
            options: chosen,
        },
    )
    .expect("answering the offer is legal");
    drain_pending(game);
}

/// A creature entering pays a life and an energy.
#[test]
fn another_creature_entering_pays_both() {
    let (mut game, _guide, _bears) = staged(0);

    game.put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    drain_pending(&mut game);

    assert_eq!(game.players[0].life, 21, "one life");
    assert_eq!(energy(&game), 1, "and one energy");
}

/// "Another": the Guide arriving does not trigger itself, and neither does
/// a creature an opponent plays.
#[test]
fn neither_himself_nor_theirs_counts() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].life = 20;
    game.players[0]
        .counters
        .set(CounterKind::named("energy"), 0);
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.put_onto_battlefield(PlayerId::One, cards::GUIDE_OF_SOULS)
        .expect("cataloged");
    drain_pending(&mut game);

    assert_eq!(game.players[0].life, 20, "he is not another creature");
    assert_eq!(energy(&game), 0);

    game.put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    drain_pending(&mut game);

    assert_eq!(game.players[0].life, 20, "and it is not one you control");
    assert_eq!(energy(&game), 0);
}

/// Three energy makes an attacker a 4/4 flying Angel, and the energy is
/// gone.
#[test]
fn three_energy_makes_an_angel() {
    let (mut game, _guide, bears) = staged(3);

    attack_and_answer(&mut game, &[bears], true);

    let attacker = permanent(&game, bears);
    assert_eq!(game.power(attacker), Some(4), "two +1/+1 counters");
    assert_eq!(game.toughness(attacker), Some(4));
    assert_eq!(
        attacker.counters(CounterKind::Flying),
        1,
        "and a flying counter",
    );
    assert!(game.has_flying(attacker), "which is what grants flying");
    assert!(
        game.effective_subtypes(attacker)
            .contains(crate::card::Subtype::named("Angel")),
        "an Angel in addition to Bear",
    );
    assert!(
        game.effective_subtypes(attacker)
            .contains(crate::card::Subtype::named("Bear")),
        "in addition to, not instead of",
    );
    assert_eq!(energy(&game), 0, "all three were spent");
}

/// Declining leaves the energy banked and the attacker as it was.
#[test]
fn declining_spends_nothing() {
    let (mut game, _guide, bears) = staged(3);

    attack_and_answer(&mut game, &[bears], false);

    let attacker = permanent(&game, bears);
    assert_eq!(game.power(attacker), Some(2));
    assert_eq!(energy(&game), 3, "the energy is still there");
}

/// Two energy cannot pay for it: energy is spent all at once or not at all,
/// so paying is not even offered.
#[test]
fn two_energy_is_not_enough() {
    let (mut game, _guide, bears) = staged(2);
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;

    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker: bears,
            defender: AttackDefender::Player(PlayerId::Two),
        },
    )
    .expect("it attacks");
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .expect("the declaration finishes");
    pass_until_decision(&mut game);
    let targeting = game
        .observe(PlayerId::One)
        .decision
        .expect("the trigger asks for its target");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: targeting.id,
            options: vec![targeting.options[0].id],
        },
    )
    .expect("naming an attacker is legal");
    pass_until_decision(&mut game);

    let offer = game
        .observe(PlayerId::One)
        .decision
        .expect("the trigger still resolves and still asks");
    assert!(
        offer.options.iter().all(|option| option.label == "Decline"),
        "two energy buys nothing, so only declining is on offer",
    );
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: offer.id,
            options: vec![0],
        },
    )
    .expect("declining is legal");
    drain_pending(&mut game);

    let attacker = permanent(&game, bears);
    assert_eq!(game.power(attacker), Some(2), "nothing happened");
    assert_eq!(energy(&game), 2, "and nothing was spent");
}

/// The Angel keeps its wings and its counters after the turn ends.
#[test]
fn the_angel_is_permanent() {
    let (mut game, _guide, bears) = staged(3);
    attack_and_answer(&mut game, &[bears], true);

    let turn = game.turn;
    for _ in 0..60 {
        if game.turn > turn {
            break;
        }
        game.advance_step();
        drain_pending(&mut game);
    }

    let attacker = permanent(&game, bears);
    assert_eq!(game.power(attacker), Some(4), "counters do not wear off");
    assert!(
        game.has_flying(attacker),
        "and neither does the counter's word"
    );
    assert!(
        game.effective_subtypes(attacker)
            .contains(crate::card::Subtype::named("Angel"))
    );
}
