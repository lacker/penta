//! Portal Second Age declarations whose composition adds a boundary beyond
//! the shared primitive tests: owner-relative follow-up, scoped untap skips,
//! an exact multi-target slot, and a multiplayer sacrifice continuation.

use super::*;

fn cast_action(game: &Game, spell: CardInstanceId, targets: &[Target]) -> Action {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            let Action::CastSpell { card, choices, .. } = action else {
                return false;
            };
            *card == spell
                && choices.targets().iter().any(|slot| {
                    slot.targets().len() == targets.len()
                        && targets.iter().all(|target| slot.targets().contains(target))
                })
        })
        .expect("the spell has the requested legal targets")
}

#[test]
fn path_of_peace_gives_life_to_the_destroyed_creatures_owner() {
    let mut game = ready_game();
    let mut stolen = creature(31_000, cards::GRIZZLY_BEARS, PlayerId::Two);
    stolen.controller = PlayerId::One;
    let stolen_id = stolen.card.id;
    game.battlefield.push(stolen);

    let path = card(31_001, cards::PATH_OF_PEACE, PlayerId::One);
    let path_id = path.id;
    game.players[PlayerId::One.index()].hand.push(path);
    game.players[PlayerId::One.index()].mana_pool.white = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 3;

    let action = cast_action(&game, path_id, &[Target::Permanent(stolen_id)]);
    game.apply(PlayerId::One, action)
        .expect("four mana covers it");
    drain_pending(&mut game);

    assert_eq!(game.players[PlayerId::One.index()].life, 20);
    assert_eq!(game.players[PlayerId::Two.index()].life, 24);
    assert!(
        game.players[PlayerId::Two.index()]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::GRIZZLY_BEARS),
        "the physical owner receives both the dead card and the life",
    );
}

#[test]
fn exhaustion_holds_the_opponents_creatures_and_lands_only() {
    let mut game = ready_game();
    let mut bear = creature(31_010, cards::GRIZZLY_BEARS, PlayerId::Two);
    bear.tapped = true;
    let creature_id = bear.card.id;
    let mut land = creature(31_011, cards::MOUNTAIN, PlayerId::Two);
    land.tapped = true;
    let land_id = land.card.id;
    let mut artifact = creature(31_012, cards::MOX_RUBY, PlayerId::Two);
    artifact.tapped = true;
    let artifact_id = artifact.card.id;
    game.battlefield.extend([bear, land, artifact]);

    let exhaustion = card(31_013, cards::EXHAUSTION, PlayerId::One);
    let exhaustion_id = exhaustion.id;
    game.players[PlayerId::One.index()].hand.push(exhaustion);
    game.players[PlayerId::One.index()].mana_pool.blue = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;

    let action = cast_action(&game, exhaustion_id, &[Target::Player(PlayerId::Two)]);
    game.apply(PlayerId::One, action)
        .expect("three mana covers it");
    drain_pending(&mut game);

    game.commit_next_turn(PlayerId::Two, Vec::new());
    drain_pending(&mut game);

    let tapped = |id| {
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .expect("the permanent remains")
            .tapped
    };
    assert!(tapped(creature_id), "the creature misses the untap");
    assert!(tapped(land_id), "the land misses the untap");
    assert!(!tapped(artifact_id), "an ordinary artifact still untaps");
}

#[test]
fn jagged_lightning_requires_and_damages_two_creatures() {
    let mut game = ready_game();
    let first = creature(31_020, cards::WALL_OF_STONE, PlayerId::One);
    let first_id = first.card.id;
    game.battlefield.push(first);

    let jagged = card(31_021, cards::JAGGED_LIGHTNING, PlayerId::One);
    let jagged_id = jagged.id;
    game.players[PlayerId::One.index()].hand.push(jagged);
    game.players[PlayerId::One.index()].mana_pool.red = 2;
    game.players[PlayerId::One.index()].mana_pool.colorless = 3;

    assert!(
        game.legal_actions(PlayerId::One)
            .iter()
            .all(|action| !matches!(action, Action::CastSpell { card, .. } if *card == jagged_id)),
        "one creature cannot fill a two-creature target slot",
    );

    let second = creature(31_022, cards::WALL_OF_STONE, PlayerId::Two);
    let second_id = second.card.id;
    game.battlefield.push(second);
    let targets = [Target::Permanent(first_id), Target::Permanent(second_id)];
    let action = cast_action(&game, jagged_id, &targets);
    game.apply(PlayerId::One, action)
        .expect("five mana covers it");
    drain_pending(&mut game);

    for id in [first_id, second_id] {
        let damage = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .expect("the Wall survives")
            .damage;
        assert_eq!(damage, 3);
    }
}

#[test]
fn wildfire_finishes_after_both_players_choose_lands() {
    let mut game = ready_game();
    for index in 0..5 {
        game.battlefield
            .push(creature(31_030 + index, cards::MOUNTAIN, PlayerId::One));
    }
    for index in 0..3 {
        game.battlefield
            .push(creature(31_040 + index, cards::MOUNTAIN, PlayerId::Two));
    }
    let first_wall = creature(31_050, cards::WALL_OF_STONE, PlayerId::One);
    let first_wall_id = first_wall.card.id;
    let second_wall = creature(31_051, cards::WALL_OF_STONE, PlayerId::Two);
    let second_wall_id = second_wall.card.id;
    game.battlefield.extend([first_wall, second_wall]);

    let wildfire = card(31_052, cards::WILDFIRE, PlayerId::One);
    let wildfire_id = wildfire.id;
    game.players[PlayerId::One.index()].hand.push(wildfire);
    game.players[PlayerId::One.index()].mana_pool.red = 2;
    game.players[PlayerId::One.index()].mana_pool.colorless = 4;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == wildfire_id))
        .expect("Wildfire is castable");
    game.apply(PlayerId::One, action)
        .expect("six mana covers it");
    drain_pending(&mut game);

    let land_count = |player| {
        game.battlefield
            .iter()
            .filter(|permanent| permanent.controller == player)
            .filter(|permanent| permanent.card.definition == cards::MOUNTAIN)
            .count()
    };
    assert_eq!(land_count(PlayerId::One), 1);
    assert_eq!(
        land_count(PlayerId::Two),
        0,
        "three means sacrificing all three"
    );
    for id in [first_wall_id, second_wall_id] {
        let damage = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .expect("the Wall survives")
            .damage;
        assert_eq!(damage, 4, "damage follows the multiplayer choices");
    }
}
