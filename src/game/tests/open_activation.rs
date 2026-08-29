//! An ability anybody may activate.
//!
//! The permanent stays the source whoever pays, so what it does is still its
//! doing -- and it does it to everyone, its own controller included. What
//! these check is that the opposing player is offered it at all, that an
//! ordinary ability is not, and that the payer is the one who pays.

use super::*;

/// An Efreet under player one, with `mana` green in each player's pool.
fn efreet_out(mana: u16) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    game.turns_started[PlayerId::Two.index()] = 5;
    let efreet = creature(10_000, cards::IFH_BIFF_EFREET, PlayerId::One);
    let efreet_id = efreet.card.id;
    game.battlefield.push(efreet);
    for player in [PlayerId::One, PlayerId::Two] {
        game.players[player.index()].mana_pool.green = mana;
    }
    (game, efreet_id)
}

fn activation(game: &Game, player: PlayerId, source: GameObjectId) -> Option<Action> {
    game.legal_actions(player)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source: actual, .. } if *actual == source))
}

#[test]
fn the_opposing_player_is_offered_it() {
    let (mut game, efreet) = efreet_out(1);
    game.priority = PlayerId::Two;

    assert!(
        activation(&game, PlayerId::Two, efreet).is_some(),
        "somebody else's permanent, but the ability is open",
    );
}

/// The control: an ordinary activated ability on somebody else's permanent is
/// not offered, which is what makes the flag mean anything.
#[test]
fn an_ordinary_ability_stays_with_its_controller() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    game.turns_started[PlayerId::Two.index()] = 5;
    let bottle = creature(10_000, cards::BOTTLE_OF_SULEIMAN, PlayerId::One);
    let bottle_id = bottle.card.id;
    game.battlefield.push(bottle);
    for player in [PlayerId::One, PlayerId::Two] {
        game.players[player.index()].mana_pool.colorless = 4;
    }

    assert!(activation(&game, PlayerId::One, bottle_id).is_some());
    game.priority = PlayerId::Two;
    assert!(activation(&game, PlayerId::Two, bottle_id).is_none());
}

/// The opponent pays, and the Efreet's controller does not.
#[test]
fn the_activating_player_pays_for_it() {
    let (mut game, efreet) = efreet_out(1);
    game.priority = PlayerId::Two;

    let action = activation(&game, PlayerId::Two, efreet).expect("offered to them");
    game.apply(PlayerId::Two, action)
        .expect("the ability activates");
    drain_pending(&mut game);

    assert_eq!(game.players[PlayerId::Two.index()].mana_pool.green, 0);
    assert_eq!(
        game.players[PlayerId::One.index()].mana_pool.green,
        1,
        "the Efreet's controller kept their mana",
    );
}

/// It hits every flier and every player, its own side included.
#[test]
fn it_catches_everything_that_flies_including_itself() {
    let (mut game, efreet) = efreet_out(1);
    let ground = creature(10_001, cards::SEDGE_TROLL, PlayerId::Two);
    let ground_id = ground.card.id;
    game.battlefield.push(ground);
    game.priority = PlayerId::Two;

    let action = activation(&game, PlayerId::Two, efreet).expect("offered to them");
    game.apply(PlayerId::Two, action)
        .expect("the ability activates");
    drain_pending(&mut game);

    let damage_on = |id: GameObjectId| {
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .map(|permanent| permanent.damage)
    };
    assert_eq!(damage_on(efreet), Some(1), "the Efreet flies, so it is hit");
    assert_eq!(damage_on(ground_id), Some(0), "the Troll does not fly");
    for player in [PlayerId::One, PlayerId::Two] {
        assert_eq!(
            game.players[player.index()].life,
            i16::from(rules::STARTING_LIFE) - 1,
            "each player takes one",
        );
    }
}
