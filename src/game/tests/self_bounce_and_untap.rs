//! Two abilities a creature points at itself. Returning to hand takes it off
//! the battlefield entirely, so a creature already attacking stops attacking;
//! untapping does not, which is the whole reason a Drake buys vigilance by
//! the turn instead of just untapping later.

use super::*;

/// `definition` attacking player two, with `hand` cards held and mana up.
fn attacking(definition: CardDefinitionId, hand: usize, blue: u16) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    let mut threat = creature(54_000, definition, PlayerId::One);
    threat.attacking = true;
    threat.tapped = true;
    threat.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    threat.entered_controller_turn = 0;
    let threat_id = threat.card.id;
    game.battlefield.push(threat);
    for index in 0..hand {
        game.players[0].hand.push(card(
            54_100 + u32::try_from(index).expect("a small fixture"),
            cards::MOUNTAIN,
            PlayerId::One,
        ));
    }
    game.players[0].mana_pool.blue = blue;
    // Past the declaration windows, where the attacking player has priority
    // and can use an ability at all.
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.blockers_declared = true;
    game.priority = PlayerId::One;
    (game, threat_id)
}

fn activate(game: &mut Game, source: GameObjectId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source: actual, .. } if *actual == source)
        })
        .expect("the ability is available");
    game.apply(PlayerId::One, action)
        .expect("the cost is payable");
    pass_priority_pair(game);
}

#[test]
fn discarding_takes_the_attacker_off_the_battlefield() {
    let (mut game, ephemeron) = attacking(cards::EPHEMERON, 1, 0);
    activate(&mut game, ephemeron);

    assert!(game.battlefield.is_empty(), "it left the battlefield");
    assert_eq!(
        game.players[0].hand.len(),
        1,
        "the discarded card was replaced by the creature itself"
    );
    assert_eq!(
        game.players[0].graveyard.len(),
        1,
        "and the card was discarded"
    );
}

#[test]
fn an_empty_hand_pays_for_no_escape() {
    let (game, ephemeron) = attacking(cards::EPHEMERON, 0, 0);
    assert!(
        !game.legal_actions(PlayerId::One).iter().any(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == ephemeron)
        }),
        "nothing to discard means nothing to pay with"
    );
}

/// Untapping is not leaving: the Drake is still an attacker, which is what
/// makes the ability worth its mana.
#[test]
fn untapping_leaves_the_drake_attacking() {
    let (mut game, drake) = attacking(cards::VIGILANT_DRAKE, 0, 3);
    activate(&mut game, drake);

    let untapped = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == drake)
        .expect("the Drake is still on the battlefield");
    assert!(!untapped.tapped, "it untapped");
    assert!(
        untapped.attacking,
        "and is still attacking, so it will deal its damage and can block later"
    );
}
