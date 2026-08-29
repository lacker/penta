//! Mana added off somebody else's tap.
//!
//! "Its controller adds an additional {G}" points at the player who tapped the
//! land, not at the player who owns the thing watching. Wild Growth can sit on
//! an opponent's land and the Gauntlet watches every Mountain on the table, so
//! both cards read wrong if the mana lands in the wrong pool.

use super::*;

fn tap_for(game: &mut Game, player: PlayerId, land: GameObjectId, color: ManaColor) {
    // A mana ability still wants its controller to hold priority.
    game.priority = player;
    let action = Action::ActivateManaAbility {
        source: land,
        ability: mana_ability_for(game, land, color),
        color,
        counters_removed: None,
        cost_object: None,
        combination: None,
    };
    game.apply(player, action).expect("the land taps for mana");
}

fn pool(game: &Game, player: PlayerId) -> ManaPool {
    game.players[player.index()].mana_pool
}

/// Wild Growth on a land its own controller owns: the ordinary case, and the
/// one that would pass even if the recipient were read from the Aura.
#[test]
fn wild_growth_adds_to_its_own_controllers_pool() {
    let mut game = ready_game();
    let forest = creature(10_000, cards::FOREST, PlayerId::One);
    let forest_id = forest.card.id;
    game.battlefield.push(forest);
    let mut aura = creature(10_001, cards::WILD_GROWTH, PlayerId::One);
    aura.attached_to = Some(forest_id);
    game.battlefield.push(aura);

    tap_for(&mut game, PlayerId::One, forest_id, ManaColor::Green);

    assert_eq!(
        pool(&game, PlayerId::One).green,
        2,
        "the Forest's own green plus the Aura's"
    );
}

/// The case that decides it: the Aura is player one's, the land is player
/// two's, and the extra mana belongs to player two.
#[test]
fn wild_growth_pays_the_land_controller_not_the_aura_controller() {
    let mut game = ready_game();
    let forest = creature(10_000, cards::FOREST, PlayerId::Two);
    let forest_id = forest.card.id;
    game.battlefield.push(forest);
    let mut aura = creature(10_001, cards::WILD_GROWTH, PlayerId::One);
    aura.attached_to = Some(forest_id);
    game.battlefield.push(aura);

    tap_for(&mut game, PlayerId::Two, forest_id, ManaColor::Green);

    assert_eq!(
        pool(&game, PlayerId::Two).green,
        2,
        "the land's controller gets both"
    );
    assert_eq!(
        pool(&game, PlayerId::One).green,
        0,
        "and the Aura's controller gets none of it"
    );
}

/// The Gauntlet watches every Mountain, including the ones across the table.
#[test]
fn the_gauntlet_pays_whoever_tapped_the_mountain() {
    let mut game = ready_game();
    let gauntlet = creature(10_000, cards::GAUNTLET_OF_MIGHT, PlayerId::One);
    game.battlefield.push(gauntlet);
    let mountain = creature(10_001, cards::MOUNTAIN, PlayerId::Two);
    let mountain_id = mountain.card.id;
    game.battlefield.push(mountain);

    tap_for(&mut game, PlayerId::Two, mountain_id, ManaColor::Red);

    assert_eq!(
        pool(&game, PlayerId::Two).red,
        2,
        "an opponent's Mountain still doubles, for them"
    );
    assert_eq!(pool(&game, PlayerId::One).red, 0);
}

/// The other half of the Gauntlet, and the control for the trigger's
/// predicate: it names Mountains, so nothing else pays out.
#[test]
fn the_gauntlet_pumps_red_creatures_and_ignores_other_lands() {
    let mut game = ready_game();
    let gauntlet = creature(10_000, cards::GAUNTLET_OF_MIGHT, PlayerId::One);
    game.battlefield.push(gauntlet);
    let goblin = creature(10_001, cards::MONSS_GOBLIN_RAIDERS, PlayerId::One);
    let goblin_id = goblin.card.id;
    game.battlefield.push(goblin);
    let plains = creature(10_002, cards::PLAINS, PlayerId::One);
    let plains_id = plains.card.id;
    game.battlefield.push(plains);

    let goblin = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == goblin_id)
        .expect("still there");
    assert_eq!(game.power(goblin), Some(2), "a 1/1 Goblin is red");

    tap_for(&mut game, PlayerId::One, plains_id, ManaColor::White);
    assert_eq!(
        pool(&game, PlayerId::One).white,
        1,
        "a Plains is not a Mountain"
    );
}
