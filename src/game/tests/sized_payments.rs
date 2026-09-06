//! Payments whose size is computed rather than printed: the mana value of
//! the spell being countered, and the X its own caster announced. A size
//! that resolves to zero is a spell that always fizzles, and nothing about
//! the board would show it.

use super::*;

/// Resolves the stack, answering any offered payment with `pay`.
fn resolve(game: &mut Game, pay: bool) {
    let label = if pay { "Pay the cost" } else { "Decline" };
    for _ in 0..16 {
        let payer = [PlayerId::One, PlayerId::Two].into_iter().find(|player| {
            game.observe(*player)
                .decision
                .is_some_and(|decision| decision.options.iter().any(|o| o.label == label))
        });
        if let Some(payer) = payer {
            choose_decision_by_label(game, payer, label);
            continue;
        }
        if !game.pending_decisions.is_empty() {
            drain_pending(game);
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
}

/// Player two casts a Bolt at player one, who answers with Rethink. `spare`
/// is how much mana player two has left after casting it.
fn rethinking(spare: u16, pay: bool) -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[1].hand.clear();
    game.turns_started[PlayerId::Two.index()] = 5;
    game.active_player = PlayerId::Two;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::Two;

    let bolt = card(90_000, cards::LIGHTNING_BOLT, PlayerId::Two);
    let bolt_id = bolt.id;
    game.players[1].hand.push(bolt);
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Red, 1 + spare);
    let cast = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == bolt_id
                    && choices
                        .targets()
                        .iter()
                        .flat_map(TargetSelection::targets)
                        .any(|target| *target == Target::Player(PlayerId::One))
            }
            _ => false,
        })
        .expect("the Bolt is castable at me");
    game.apply(PlayerId::Two, cast).expect("the cast is legal");

    let rethink = card(90_100, cards::RETHINK, PlayerId::One);
    let rethink_id = rethink.id;
    game.players[0].hand.push(rethink);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
    game.priority = PlayerId::One;
    let answer = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == rethink_id))
        .expect("the Bolt is a legal target");
    game.apply(PlayerId::One, answer)
        .expect("the cast is legal");
    resolve(&mut game, pay);
    game
}

fn bolt_resolved(game: &Game) -> bool {
    game.players[0].life < 20
}

#[test]
fn a_tapped_out_opponent_loses_the_bolt() {
    let game = rethinking(0, true);
    assert!(
        !bolt_resolved(&game),
        "with no mana left the Bolt cannot pay its own mana value"
    );
}

#[test]
fn one_spare_mana_pays_a_one_drops_tax() {
    let game = rethinking(1, true);
    assert!(
        bolt_resolved(&game),
        "one spare mana covers a mana value of one, so the Bolt resolved"
    );
}

#[test]
fn declining_the_tax_counters_it_anyway() {
    let game = rethinking(1, false);
    assert!(
        !bolt_resolved(&game),
        "the mana was there and simply not spent"
    );
}

/// Excise cast for `x` at an attacking Bears of player two's, who has
/// `their_mana` colourless left over.
fn excising(x: u16, their_mana: u16) -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::Two.index()] = 5;
    let mut bear = creature(90_200, cards::GRIZZLY_BEARS, PlayerId::Two);
    bear.entered_controller_turn = 0;
    bear.attacking = true;
    bear.attack_defender = Some(AttackDefender::Player(PlayerId::One));
    let bear_id = bear.card.id;
    game.battlefield.push(bear);
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Colorless, their_mana);

    let excise = card(90_300, cards::EXCISE, PlayerId::One);
    let excise_id = excise.id;
    game.players[0].hand.push(excise);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, x);
    game.active_player = PlayerId::Two;
    // Instants are only offered once the blocker declaration is finished:
    // the declare-blockers step otherwise offers declarations and nothing
    // else.
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.blockers_declared = true;
    game.priority = PlayerId::One;
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == excise_id
                    && choices.x() == x
                    && choices
                        .targets()
                        .iter()
                        .flat_map(TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(bear_id))
            }
            _ => false,
        })
        .unwrap_or_else(|| panic!("a cast for X of {x} at the Bears is offered"));
    game.apply(PlayerId::One, cast).expect("the cast is legal");
    resolve(&mut game, true);
    game
}

fn bears_alive(game: &Game) -> bool {
    game.battlefield
        .iter()
        .any(|permanent| permanent.card.definition == ObjectKind::Card(cards::GRIZZLY_BEARS))
}

#[test]
fn excise_for_two_exiles_a_tapped_out_attacker() {
    let game = excising(2, 0);
    assert!(!bears_alive(&game), "no mana to pay the {{2}}");
    assert!(
        game.players[1]
            .exile
            .iter()
            .any(|card| card.definition == ObjectKind::Card(cards::GRIZZLY_BEARS)),
        "and exile is where it went"
    );
}

#[test]
fn the_attacker_survives_when_the_x_is_paid() {
    let game = excising(2, 2);
    assert!(
        bears_alive(&game),
        "two mana covers an X of two, so the Bears stayed"
    );
}

#[test]
fn an_x_of_zero_never_exiles_anything() {
    let game = excising(0, 0);
    assert!(bears_alive(&game), "an X of zero is a payment already made");
}
