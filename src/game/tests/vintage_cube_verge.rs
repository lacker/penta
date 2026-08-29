//! The verges: lands whose second colour is conditional. Wastewood
//! Verge stands for the cycle; the others differ only in which two colours.

use super::*;

/// Which colours the Verge is currently offering.
fn offered_colors(game: &Game, verge: GameObjectId) -> Vec<ManaColor> {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == verge)
        .expect("the Verge is on the battlefield");
    let mut colors = game
        .mana_ability_activations(permanent)
        .into_iter()
        .map(|activation| activation.color)
        .collect::<Vec<_>>();
    colors.sort_unstable();
    colors.dedup();
    colors
}

fn staged(companions: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    let verge = game
        .put_onto_battlefield(PlayerId::One, cards::WASTEWOOD_VERGE)
        .expect("cataloged");
    for definition in companions {
        game.put_onto_battlefield(PlayerId::One, *definition)
            .expect("cataloged");
    }
    drain_pending(&mut game);
    (game, verge)
}

/// Alone it is a green source and nothing else.
#[test]
fn the_verge_alone_makes_only_green() {
    let (game, verge) = staged(&[]);

    assert_eq!(offered_colors(&game, verge), vec![ManaColor::Green]);
}

/// A Forest turns the black half on, and so does a Swamp: either type
/// answers the condition.
#[test]
fn either_named_land_type_turns_the_black_half_on() {
    for companion in [cards::FOREST, cards::SWAMP] {
        let (game, verge) = staged(&[companion]);

        let mut expected = vec![ManaColor::Black, ManaColor::Green];
        expected.sort_unstable();
        let mut offered = offered_colors(&game, verge);
        offered.sort_unstable();
        assert_eq!(offered, expected, "{companion:?} should switch it on");
    }
}

/// A land with neither type does not, however many of them there are.
#[test]
fn an_unrelated_land_does_not_turn_it_on() {
    let (game, verge) = staged(&[cards::MOUNTAIN, cards::ISLAND, cards::PLAINS]);

    assert_eq!(offered_colors(&game, verge), vec![ManaColor::Green]);
}

/// The Verge is not a Forest itself, so a second copy does not switch the
/// first one on.
#[test]
fn two_verges_do_not_switch_each_other_on() {
    let (game, verge) = staged(&[cards::WASTEWOOD_VERGE]);

    assert_eq!(offered_colors(&game, verge), vec![ManaColor::Green]);
}

/// With the condition met, the black half actually produces black.
#[test]
fn the_black_half_adds_black_when_it_is_offered() {
    let (mut game, verge) = staged(&[cards::SWAMP]);
    let ability = mana_ability_for(&game, verge, ManaColor::Black);

    game.apply(
        PlayerId::One,
        Action::ActivateManaAbility {
            source: verge,
            ability,
            color: ManaColor::Black,
            counters_removed: None,
            cost_object: None,
            combination: None,
            triggered_mana: None,
        },
    )
    .expect("the ability activates");

    assert_eq!(game.players[0].mana_pool.black, 1);
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == verge)
            .is_some_and(|permanent| permanent.tapped),
        "and it tapped for it",
    );
}

/// Riverpyre Verge is the same land in the cycle's other pair of colours:
/// red unconditionally, blue once the deck has supplied the land type.
#[test]
fn the_riverpyre_verge_offers_its_own_two_colours() {
    let mut game = ready_game();
    game.battlefield.clear();
    let verge = game
        .put_onto_battlefield(PlayerId::One, cards::RIVERPYRE_VERGE)
        .expect("cataloged");
    drain_pending(&mut game);
    assert_eq!(offered_colors(&game, verge), vec![ManaColor::Red]);

    game.put_onto_battlefield(PlayerId::One, cards::ISLAND)
        .expect("cataloged");
    drain_pending(&mut game);

    let mut offered = offered_colors(&game, verge);
    offered.sort_unstable();
    let mut expected = vec![ManaColor::Blue, ManaColor::Red];
    expected.sort_unstable();
    assert_eq!(offered, expected, "an Island switches the blue half on");
}

/// And a land with neither of *its* types leaves it alone, even one that
/// would switch the other verge on.
#[test]
fn the_riverpyre_verge_ignores_the_other_cycle_members_types() {
    let mut game = ready_game();
    game.battlefield.clear();
    let verge = game
        .put_onto_battlefield(PlayerId::One, cards::RIVERPYRE_VERGE)
        .expect("cataloged");
    game.put_onto_battlefield(PlayerId::One, cards::SWAMP)
        .expect("cataloged");
    drain_pending(&mut game);

    assert_eq!(offered_colors(&game, verge), vec![ManaColor::Red]);
}

/// Thornspire Verge is the Gruul member: red unconditionally, green once a
/// Mountain or a Forest is out.
#[test]
fn the_thornspire_verge_offers_its_own_two_colours() {
    let mut game = ready_game();
    game.battlefield.clear();
    let verge = game
        .put_onto_battlefield(PlayerId::One, cards::THORNSPIRE_VERGE)
        .expect("cataloged");
    drain_pending(&mut game);
    assert_eq!(offered_colors(&game, verge), vec![ManaColor::Red]);

    // Either of its two types answers, and a Swamp is neither.
    game.put_onto_battlefield(PlayerId::One, cards::SWAMP)
        .expect("cataloged");
    drain_pending(&mut game);
    assert_eq!(offered_colors(&game, verge), vec![ManaColor::Red]);

    game.put_onto_battlefield(PlayerId::One, cards::FOREST)
        .expect("cataloged");
    drain_pending(&mut game);
    let mut offered = offered_colors(&game, verge);
    offered.sort_unstable();
    let mut expected = vec![ManaColor::Red, ManaColor::Green];
    expected.sort_unstable();
    assert_eq!(offered, expected, "a Forest switches the green half on");
}

/// Blazemire Verge is the Rakdos member: black unconditionally, red once a
/// Swamp or a Mountain is out.
#[test]
fn the_blazemire_verge_offers_its_own_two_colours() {
    let mut game = ready_game();
    game.battlefield.clear();
    let verge = game
        .put_onto_battlefield(PlayerId::One, cards::BLAZEMIRE_VERGE)
        .expect("cataloged");
    drain_pending(&mut game);
    assert_eq!(offered_colors(&game, verge), vec![ManaColor::Black]);

    // A Forest is neither of its types.
    game.put_onto_battlefield(PlayerId::One, cards::FOREST)
        .expect("cataloged");
    drain_pending(&mut game);
    assert_eq!(offered_colors(&game, verge), vec![ManaColor::Black]);

    game.put_onto_battlefield(PlayerId::One, cards::MOUNTAIN)
        .expect("cataloged");
    drain_pending(&mut game);
    let mut offered = offered_colors(&game, verge);
    offered.sort_unstable();
    let mut expected = vec![ManaColor::Black, ManaColor::Red];
    expected.sort_unstable();
    assert_eq!(offered, expected, "a Mountain switches the red half on");
}

/// Bleachbone Verge is the Orzhov member: black unconditionally, white once
/// a Plains or a Swamp is out.
#[test]
fn the_bleachbone_verge_offers_its_own_two_colours() {
    let mut game = ready_game();
    game.battlefield.clear();
    let verge = game
        .put_onto_battlefield(PlayerId::One, cards::BLEACHBONE_VERGE)
        .expect("cataloged");
    drain_pending(&mut game);
    assert_eq!(offered_colors(&game, verge), vec![ManaColor::Black]);

    // An Island is neither of its types.
    game.put_onto_battlefield(PlayerId::One, cards::ISLAND)
        .expect("cataloged");
    drain_pending(&mut game);
    assert_eq!(offered_colors(&game, verge), vec![ManaColor::Black]);

    game.put_onto_battlefield(PlayerId::One, cards::PLAINS)
        .expect("cataloged");
    drain_pending(&mut game);
    let mut offered = offered_colors(&game, verge);
    offered.sort_unstable();
    let mut expected = vec![ManaColor::Black, ManaColor::White];
    expected.sort_unstable();
    assert_eq!(offered, expected, "a Plains switches the white half on");
}

/// A Swamp answers the same condition, and it is the land the Verge's own
/// unconditional half already wants.
#[test]
fn a_swamp_switches_the_bleachbone_verge_on_too() {
    let mut game = ready_game();
    game.battlefield.clear();
    let verge = game
        .put_onto_battlefield(PlayerId::One, cards::BLEACHBONE_VERGE)
        .expect("cataloged");
    game.put_onto_battlefield(PlayerId::One, cards::SWAMP)
        .expect("cataloged");
    drain_pending(&mut game);

    let mut offered = offered_colors(&game, verge);
    offered.sort_unstable();
    let mut expected = vec![ManaColor::Black, ManaColor::White];
    expected.sort_unstable();
    assert_eq!(offered, expected);
}

/// Sunbillow Verge is the Boros member: white unconditionally, red once a
/// Mountain or a Plains is out.
#[test]
fn the_sunbillow_verge_offers_its_own_two_colours() {
    let mut game = ready_game();
    game.battlefield.clear();
    let verge = game
        .put_onto_battlefield(PlayerId::One, cards::SUNBILLOW_VERGE)
        .expect("cataloged");
    drain_pending(&mut game);
    assert_eq!(offered_colors(&game, verge), vec![ManaColor::White]);

    // An Island is neither of its types.
    game.put_onto_battlefield(PlayerId::One, cards::ISLAND)
        .expect("cataloged");
    drain_pending(&mut game);
    assert_eq!(offered_colors(&game, verge), vec![ManaColor::White]);

    game.put_onto_battlefield(PlayerId::One, cards::MOUNTAIN)
        .expect("cataloged");
    drain_pending(&mut game);
    let mut offered = offered_colors(&game, verge);
    offered.sort_unstable();
    let mut expected = vec![ManaColor::White, ManaColor::Red];
    expected.sort_unstable();
    assert_eq!(offered, expected, "a Mountain switches the red half on");
}

/// "A Swamp or a Forest" is about types rather than about basics: a Bayou is
/// both of them at once, and one land answering the condition twice over is
/// still one land.
#[test]
fn a_dual_land_carrying_the_type_answers_the_condition() {
    let (game, verge) = staged(&[cards::BAYOU]);

    let mut offered = offered_colors(&game, verge);
    offered.sort_unstable();
    let mut expected = vec![ManaColor::Black, ManaColor::Green];
    expected.sort_unstable();
    assert_eq!(offered, expected, "a Swamp Forest is a Swamp and a Forest");
}

/// And the types are read as they stand rather than as they were printed. A
/// Blood Moon takes the Bayou's two types away, and takes the Verge's own
/// printed halves with them: what is left of both is a Mountain.
#[test]
fn a_blood_moon_leaves_neither_the_type_nor_the_verge() {
    let (mut game, verge) = staged(&[cards::BAYOU]);
    game.put_onto_battlefield(PlayerId::One, cards::BLOOD_MOON)
        .expect("cataloged");
    drain_pending(&mut game);

    assert_eq!(
        offered_colors(&game, verge),
        vec![ManaColor::Red],
        "a nonbasic land that is a Mountain taps for red and nothing else",
    );
}
