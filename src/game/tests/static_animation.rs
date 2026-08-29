//! Lands animated by a static ability.
//!
//! Unlike a resolved animation, this one is never written onto the land: it
//! has to keep applying as lands arrive and stop the moment the enchantment
//! leaves. What these drive is that liveness, and the narrowness of the
//! effect -- the lands are still lands, and still tap for mana.

use super::*;

fn stats(game: &Game, permanent: GameObjectId) -> Option<(Option<i16>, Option<i16>)> {
    game.battlefield
        .iter()
        .find(|candidate| candidate.card.id == permanent)
        .map(|candidate| (game.power(candidate), game.toughness(candidate)))
}

fn is_creature(game: &Game, permanent: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .find(|candidate| candidate.card.id == permanent)
        .and_then(|candidate| game.permanent_types(candidate))
        .is_some_and(|types| types.contains(CardType::Creature))
}

#[test]
fn living_lands_animates_the_forests_and_leaves_the_rest() {
    let mut game = ready_game();
    let forest = creature(10_000, cards::FOREST, PlayerId::One);
    let forest_id = forest.card.id;
    game.battlefield.push(forest);
    let island = creature(10_001, cards::ISLAND, PlayerId::Two);
    let island_id = island.card.id;
    game.battlefield.push(island);

    assert!(!is_creature(&game, forest_id), "a Forest is not one yet");

    let enchantment = creature(10_002, cards::LIVING_LANDS, PlayerId::One);
    let enchantment_id = enchantment.card.id;
    game.battlefield.push(enchantment);

    assert!(is_creature(&game, forest_id));
    assert_eq!(stats(&game, forest_id), Some((Some(1), Some(1))));
    assert!(
        !is_creature(&game, island_id),
        "an Island is not a Forest, and belongs to the other player besides"
    );

    // Still a land: it taps for green like any other Forest.
    game.priority = PlayerId::One;
    assert!(
        game.legal_actions(PlayerId::One).iter().any(|action| {
            matches!(action, Action::ActivateManaAbility { source, color, .. }
                if *source == forest_id && *color == ManaColor::Green)
        }),
        "the animation adds the creature type rather than replacing the land"
    );

    game.battlefield
        .retain(|permanent| permanent.card.id != enchantment_id);
    assert!(
        !is_creature(&game, forest_id),
        "and the effect stops with its source"
    );
}

/// A Forest played after the enchantment is animated too, which a
/// materialised animation could not manage.
#[test]
fn a_land_arriving_later_is_animated_as_well() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::LIVING_LANDS, PlayerId::One));

    let forest = creature(10_001, cards::FOREST, PlayerId::One);
    let forest_id = forest.card.id;
    game.battlefield.push(forest);

    assert!(is_creature(&game, forest_id));
}

/// Kormus Bell repaints as well as animates, and the colour is read through
/// the same funnel as protection and Aura legality.
#[test]
fn kormus_bell_makes_the_swamps_black() {
    let mut game = ready_game();
    let swamp = creature(10_000, cards::SWAMP, PlayerId::One);
    let swamp_id = swamp.card.id;
    game.battlefield.push(swamp);
    game.battlefield
        .push(creature(10_001, cards::KORMUS_BELL, PlayerId::Two));

    let swamp = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == swamp_id)
        .expect("there");
    assert!(is_creature(&game, swamp_id));
    assert_eq!(
        game.permanent_colors(swamp),
        [false, false, true, false, false],
        "black, and nothing else"
    );
}

/// Living Plane names no land type at all, so both players' lands go.
#[test]
fn living_plane_animates_every_land() {
    let mut game = ready_game();
    let mine = creature(10_000, cards::FOREST, PlayerId::One);
    let mine_id = mine.card.id;
    game.battlefield.push(mine);
    let theirs = creature(10_001, cards::SWAMP, PlayerId::Two);
    let theirs_id = theirs.card.id;
    game.battlefield.push(theirs);
    game.battlefield
        .push(creature(10_002, cards::LIVING_PLANE, PlayerId::One));

    for land in [mine_id, theirs_id] {
        assert!(is_creature(&game, land));
        assert_eq!(stats(&game, land), Some((Some(1), Some(1))));
    }
}
