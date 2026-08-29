//! Two Magic 2013 cards whose audit lines named primitives that existed.
//!
//! Islandwalk has been printed on cards since Alpha, and the tap-plus-skip
//! pair was built for Spore Cloud. What is worth pinning is the scoping each
//! card puts around them: the Master reaches only your own other Merfolk, and
//! Sleep only the targeted player's creatures.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.turns_started[PlayerId::Two.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game
}

fn stats(game: &Game, id: GameObjectId) -> (Option<i16>, Option<i16>) {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still there");
    (game.power(permanent), game.toughness(permanent))
}

fn has_islandwalk(game: &Game, id: GameObjectId) -> bool {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still there");
    game.permanent_has_executable_keyword(
        permanent,
        KeywordAbility::Landwalk(BasicLandType::Island),
    )
}

fn tapped(game: &Game, id: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still there")
        .tapped
}

#[test]
fn the_master_pumps_your_other_merfolk_and_gives_them_islandwalk() {
    let mut game = ready();
    let master = creature(10_000, cards::MASTER_OF_THE_PEARL_TRIDENT, PlayerId::One);
    let master_id = master.card.id;
    game.battlefield.push(master);
    let ally = creature(10_001, cards::MERFOLK_OF_THE_PEARL_TRIDENT, PlayerId::One);
    let ally_id = ally.card.id;
    game.battlefield.push(ally);

    assert_eq!(stats(&game, ally_id), (Some(2), Some(2)), "a 1/1 plus one");
    assert!(has_islandwalk(&game, ally_id));

    assert_eq!(
        stats(&game, master_id),
        (Some(2), Some(2)),
        "\"other\", so the Master does not pump itself",
    );
    assert!(
        !has_islandwalk(&game, master_id),
        "and does not give itself islandwalk either",
    );
}

/// "You control", which is where the Master differs from Lord of Atlantis.
#[test]
fn the_master_leaves_the_opponents_merfolk_alone() {
    let mut game = ready();
    game.battlefield.push(creature(
        10_000,
        cards::MASTER_OF_THE_PEARL_TRIDENT,
        PlayerId::One,
    ));
    let theirs = creature(10_001, cards::MERFOLK_OF_THE_PEARL_TRIDENT, PlayerId::Two);
    let theirs_id = theirs.card.id;
    game.battlefield.push(theirs);

    assert_eq!(stats(&game, theirs_id), (Some(1), Some(1)));
    assert!(!has_islandwalk(&game, theirs_id));
}

/// Sleep taps only the targeted player's creatures, and holds exactly those
/// through their next untap step.
#[test]
fn sleep_taps_and_holds_only_the_targeted_players_creatures() {
    let mut game = ready();
    let mine = creature(10_000, cards::GRIZZLY_BEARS, PlayerId::One);
    let mine_id = mine.card.id;
    game.battlefield.push(mine);
    let theirs = creature(10_001, cards::GRIZZLY_BEARS, PlayerId::Two);
    let theirs_id = theirs.card.id;
    game.battlefield.push(theirs);

    let spell = card(20_000, cards::SLEEP, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.blue = 2;
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == spell_id
                    && choices.targets().iter().any(|slot| slot.targets() == [Target::Player(PlayerId::Two)]))
        })
        .expect("their seat is a legal target");
    game.apply(PlayerId::One, action)
        .expect("four mana covers it");
    drain_pending(&mut game);

    assert!(tapped(&game, theirs_id), "theirs went down");
    assert!(!tapped(&game, mine_id), "mine did not");

    game.commit_next_turn(PlayerId::Two, Vec::new());
    drain_pending(&mut game);
    assert!(tapped(&game, theirs_id), "and skipped their untap step");

    game.commit_next_turn(PlayerId::One, Vec::new());
    game.commit_next_turn(PlayerId::Two, Vec::new());
    drain_pending(&mut game);
    assert!(!tapped(&game, theirs_id), "back a cycle later");
}
