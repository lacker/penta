//! "Defending player" as the recipient of a becomes-blocked trigger. It is
//! read off the attack rather than off the blocker, and it is easy to write
//! as "the opponent" and be right only from the attacker's seat. These drive
//! it from both seats, and check that a creature going unblocked names
//! nobody at all.

use super::*;

/// `attacker` attacking `defender`, blocked by one of the defender's Bears.
/// Both players hold a card, have a library, and control a land.
fn blocked_attack(attacker: CardDefinitionId, attacking_player: PlayerId) -> Game {
    let defending_player = attacking_player.opponent();
    let mut game = ready_game();
    game.battlefield.clear();
    game.turns_started[attacking_player.index()] = 5;
    for player in [PlayerId::One, PlayerId::Two] {
        game.players[player.index()].hand.clear();
        game.players[player.index()].hand.push(card(
            41_010 + u32::try_from(player.index()).expect("two players"),
            cards::MOUNTAIN,
            player,
        ));
        let mut land = creature(
            41_020 + u32::try_from(player.index()).expect("two players"),
            cards::MOUNTAIN,
            player,
        );
        land.entered_controller_turn = 0;
        game.battlefield.push(land);
    }
    let mut threat = creature(41_000, attacker, attacking_player);
    threat.attacking = true;
    threat.attack_defender = Some(AttackDefender::Player(defending_player));
    let threat_id = threat.card.id;
    game.battlefield.push(threat);
    let mut blocker = creature(41_001, cards::GRIZZLY_BEARS, defending_player);
    blocker.entered_controller_turn = 0;
    blocker.blocking = vec![threat_id];
    game.battlefield.push(blocker);
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.finish_declaring_blockers();
    drain_pending(&mut game);
    game
}

fn hands(game: &Game) -> (usize, usize) {
    (game.players[0].hand.len(), game.players[1].hand.len())
}

#[test]
fn the_blocking_player_is_the_one_who_discards() {
    let game = blocked_attack(cards::ALLEY_GRIFTERS, PlayerId::One);
    assert_eq!(
        hands(&game),
        (1, 0),
        "the attacker's controller keeps their card and the defender loses theirs"
    );
}

/// The same card attacking from the other seat, which is where naming the
/// opponent instead of the defender would take the wrong player's card.
#[test]
fn the_recipient_follows_the_attack_not_the_controller() {
    let game = blocked_attack(cards::ALLEY_GRIFTERS, PlayerId::Two);
    assert_eq!(hands(&game), (0, 1));
}

#[test]
fn going_unblocked_costs_the_defender_nothing() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[1].hand.clear();
    game.players[1]
        .hand
        .push(card(41_011, cards::MOUNTAIN, PlayerId::Two));
    game.turns_started[PlayerId::One.index()] = 5;
    let mut grifters = creature(41_000, cards::ALLEY_GRIFTERS, PlayerId::One);
    grifters.attacking = true;
    grifters.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    game.battlefield.push(grifters);
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.finish_declaring_blockers();
    drain_pending(&mut game);

    assert_eq!(
        game.players[1].hand.len(),
        1,
        "nothing blocked, so nothing fired"
    );
}

#[test]
fn the_mill_and_the_sacrifice_name_the_same_player() {
    let golem = blocked_attack(cards::FLINT_GOLEM, PlayerId::One);
    assert_eq!(
        golem.players[1].graveyard.len(),
        3,
        "three cards off the defender's library"
    );
    assert!(
        golem.players[0].graveyard.is_empty(),
        "and none off the attacker's"
    );

    let beast = blocked_attack(cards::THRESHER_BEAST, PlayerId::One);
    let lands = |player: PlayerId| {
        beast
            .battlefield
            .iter()
            .filter(|permanent| {
                permanent.controller == player && permanent.card.id != GameObjectId(41_001)
            })
            .count()
    };
    assert_eq!(
        (lands(PlayerId::One), lands(PlayerId::Two)),
        (2, 0),
        "the defender's land went and the attacker kept both its permanents"
    );
}
