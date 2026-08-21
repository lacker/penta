//! Voice of Victory: two extra power on every attack, and a Silence for the
//! turn you attack.

use super::*;

/// Player One with a Voice out since last turn, ready to declare attackers.
fn staged() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[1].hand.clear();
    let voice = game
        .put_onto_battlefield(PlayerId::One, cards::VOICE_OF_VICTORY)
        .expect("cataloged");
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [2, 1];
    drain_pending(&mut game);
    game.active_player = PlayerId::One;
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.priority = PlayerId::One;
    (game, voice)
}

fn settle(game: &mut Game) {
    for _ in 0..24 {
        if !game.pending_decisions.is_empty() {
            return;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    game.check_state_based_actions();
}

fn attack_with(game: &mut Game, attacker: GameObjectId) {
    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker,
            defender: AttackDefender::Player(PlayerId::Two),
        },
    )
    .expect("it attacks");
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .expect("the declaration finishes");
    settle(game);
    drain_pending(game);
}

fn warriors(game: &Game) -> Vec<&Permanent> {
    game.battlefield
        .iter()
        .filter(|permanent| {
            is_token_with(
                permanent,
                tokens::creature(&["Warrior"], &[ManaColor::Red], 1, 1),
            )
        })
        .collect()
}

/// One step of "let the turn get on with it".
fn advance(game: &mut Game) -> bool {
    if let Some(seat) = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.player)
    {
        let decision = game.observe(seat).decision.expect("just checked");
        let options = decision
            .options
            .iter()
            .take(decision.minimum)
            .map(|option| option.id)
            .collect();
        return game
            .apply(
                seat,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .is_ok();
    }
    for action in [
        Action::FinishDeclaringAttackers,
        Action::FinishDeclaringBlockers,
    ] {
        for seat in [PlayerId::One, PlayerId::Two] {
            if game.legal_actions(seat).contains(&action) {
                return game.apply(seat, action.clone()).is_ok();
            }
        }
    }
    let player = game.priority;
    game.apply(player, Action::PassPriority).is_ok()
}

/// Puts a Bolt in Player Two's hand with the mana to cast it, and reports
/// whether they can.
fn they_can_cast_a_bolt(game: &mut Game) -> bool {
    let bolt = game
        .build_zone(PlayerId::Two, &[cards::LIGHTNING_BOLT])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let id = bolt.id;
    game.players[1].hand.push(bolt);
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Red, 1);
    let castable = game
        .legal_actions(PlayerId::Two)
        .iter()
        .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == id));
    game.players[1].hand.retain(|card| card.id != id);
    castable
}

/// Mobilize 2: attacking makes two Warriors, tapped and attacking.
#[test]
fn attacking_makes_two_tapped_attacking_warriors() {
    let (mut game, voice) = staged();

    attack_with(&mut game, voice);

    let warriors = warriors(&game);
    assert_eq!(warriors.len(), 2, "mobilize 2 makes two");
    for warrior in warriors {
        assert!(warrior.tapped, "tapped");
        assert!(warrior.attacking, "and attacking");
        assert_eq!(
            warrior.attack_defender,
            Some(AttackDefender::Player(PlayerId::Two)),
            "at the one player there is to attack",
        );
        assert_eq!(
            (game.power(warrior), game.toughness(warrior)),
            (Some(1), Some(1)),
        );
    }
}

/// "Sacrifice them at the beginning of the next end step" -- and the Voice
/// itself is not one of them.
#[test]
fn the_warriors_are_sacrificed_at_the_end_step() {
    let (mut game, voice) = staged();
    attack_with(&mut game, voice);
    assert_eq!(warriors(&game).len(), 2, "they are here for now");

    for _ in 0..60 {
        if warriors(&game).is_empty() {
            break;
        }
        if !advance(&mut game) {
            break;
        }
    }

    assert!(warriors(&game).is_empty(), "the end step took them back");
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == voice),
        "and left the Voice alone",
    );
}

/// A Warrior that was on the battlefield before the attack is not one of the
/// tokens this attack made, so the delayed clause does not take it.
#[test]
fn it_sacrifices_only_the_tokens_it_made() {
    let (mut game, voice) = staged();
    game.create_token(
        PlayerId::One,
        tokens::creature(&["Warrior"], &[ManaColor::Red], 1, 1),
    );
    drain_pending(&mut game);
    let bystander = game
        .battlefield
        .iter()
        .find(|permanent| {
            is_token_with(
                permanent,
                tokens::creature(&["Warrior"], &[ManaColor::Red], 1, 1),
            )
        })
        .expect("the bystander entered")
        .card
        .id;

    attack_with(&mut game, voice);
    assert_eq!(warriors(&game).len(), 3, "two new ones and the bystander");

    for _ in 0..60 {
        if warriors(&game).len() <= 1 {
            break;
        }
        if !advance(&mut game) {
            break;
        }
    }

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == bystander),
        "the Warrior that was already here is still here",
    );
    assert_eq!(warriors(&game).len(), 1, "and only it");
}

/// "Your opponents can't cast spells during your turn."
#[test]
fn opponents_cannot_cast_during_your_turn() {
    let (mut game, voice) = staged();
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::Two;

    assert!(
        !they_can_cast_a_bolt(&mut game),
        "it is your turn, so they may not",
    );

    // The same moment with the Voice gone, so the refusal above is the
    // clause rather than the step.
    game.battlefield
        .retain(|permanent| permanent.card.id != voice);
    assert!(
        they_can_cast_a_bolt(&mut game),
        "and it was the Voice stopping them",
    );
}

/// On their own turn the same opponents may cast whatever they like.
#[test]
fn they_may_cast_on_their_own_turn() {
    let (mut game, _voice) = staged();
    game.active_player = PlayerId::Two;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::Two;

    assert!(
        they_can_cast_a_bolt(&mut game),
        "the clause names your turn and no other",
    );
}

/// It is your opponents who are silenced, not you.
#[test]
fn you_may_still_cast_on_your_own_turn() {
    let (mut game, _voice) = staged();
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    let bolt = game
        .build_zone(PlayerId::One, &[cards::LIGHTNING_BOLT])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let id = bolt.id;
    game.players[0].hand.push(bolt);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);

    assert!(
        game.legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == id)),
        "your own spells are untouched",
    );
}

/// The Voice leaving lifts the restriction: it is a static ability of a
/// permanent, not a lasting effect.
#[test]
fn killing_it_gives_them_their_spells_back() {
    let (mut game, voice) = staged();
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::Two;
    assert!(!they_can_cast_a_bolt(&mut game), "silenced for now");

    game.move_permanents_to_graveyard(&[voice]);
    settle(&mut game);
    game.priority = PlayerId::Two;

    assert!(
        they_can_cast_a_bolt(&mut game),
        "with the Voice gone they may cast again",
    );
}
