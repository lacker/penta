//! The horizon lands: two colours that cost a life apiece, and a land that
//! turns into a card once it has nothing left to pay for. Sunbaked Canyon
//! stands for the cycle; the others differ only in which two colours.

use super::*;

fn staged() -> (Game, GameObjectId) {
    staged_land(cards::SUNBAKED_CANYON)
}

/// One horizon land of the cycle, alone on the battlefield.
fn staged_land(definition: CardDefinitionId) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    let land = game
        .put_onto_battlefield(PlayerId::One, definition)
        .expect("cataloged");
    drain_pending(&mut game);
    game.priority = PlayerId::One;
    (game, land)
}

/// Cashes `land` in for a card, which every member of the cycle does the
/// same way. Taken from what the land is offering rather than by index: the
/// mana ability and this one are not numbered in the same sequence.
fn cash_in(game: &mut Game, land: GameObjectId) {
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == land))
        .expect("one mana and a tap buys a card");
    game.apply(PlayerId::One, action)
        .expect("the ability activates");
    drain_pending(game);
}

fn mana_colors(game: &Game, source: GameObjectId) -> Vec<ManaColor> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::ActivateManaAbility {
                source: id, color, ..
            } if id == source => Some(color),
            _ => None,
        })
        .collect()
}

/// Red and white, and nothing else -- a horizon land makes no colourless.
#[test]
fn it_offers_only_its_two_colours() {
    let (game, canyon) = staged();
    let colors = mana_colors(&game, canyon);

    assert!(colors.contains(&ManaColor::Red));
    assert!(colors.contains(&ManaColor::White));
    assert_eq!(colors.len(), 2, "no colourless and no third colour");
}

/// Tapping it costs a life, which is the whole reason it draws a card later.
#[test]
fn making_mana_costs_a_life() {
    let (mut game, canyon) = staged();
    let ability = mana_ability_for(&game, canyon, ManaColor::Red);

    game.apply(
        PlayerId::One,
        Action::ActivateManaAbility {
            source: canyon,
            ability,
            color: ManaColor::Red,
            counters_removed: None,
            cost_object: None,
            combination: None,
            triggered_mana: None,
        },
    )
    .expect("a land with a life to spare taps for red");

    assert_eq!(game.players[0].mana_pool.red, 1);
    assert_eq!(game.players[0].life, 19, "one life for one mana");
}

/// A player at one life may still pay it: life is payable down to zero.
#[test]
fn one_life_is_still_enough() {
    let (mut game, canyon) = staged();
    game.players[0].life = 1;

    assert!(
        !mana_colors(&game, canyon).is_empty(),
        "paying to zero is legal (CR 118.4)",
    );
}

/// A player at zero cannot pay it, so the land makes no mana at all.
#[test]
fn no_life_means_no_mana() {
    let (mut game, canyon) = staged();
    game.players[0].life = 0;

    assert!(
        mana_colors(&game, canyon).is_empty(),
        "there is no life left to spend",
    );
}

/// Cashing it in sacrifices the land and draws.
#[test]
fn it_can_be_cashed_in_for_a_card() {
    let (mut game, canyon) = staged();
    let before = game.players[0].hand.len();

    cash_in(&mut game, canyon);

    assert!(
        game.battlefield.is_empty(),
        "the land sacrificed itself as a cost",
    );
    assert_eq!(game.players[0].hand.len(), before + 1);
    assert_eq!(game.players[0].life, 20, "and cost no life to do it");
}

/// Horizon Canopy is the original of the cycle and the same card: only the
/// pair of colours is different.
#[test]
fn the_canopy_offers_its_own_two_colours() {
    let (game, canopy) = staged_land(cards::HORIZON_CANOPY);
    let colors = mana_colors(&game, canopy);

    assert!(colors.contains(&ManaColor::Green), "Forest half");
    assert!(colors.contains(&ManaColor::White), "Plains half");
    assert_eq!(colors.len(), 2, "no colourless and no third colour");
}

/// And it cashes itself in the same way, which is what makes the shared
/// clause shared rather than copied.
#[test]
fn the_canopy_cashes_itself_in_too() {
    let (mut game, canopy) = staged_land(cards::HORIZON_CANOPY);
    let before = game.players[0].hand.len();

    cash_in(&mut game, canopy);

    assert!(game.battlefield.is_empty(), "the land sacrificed itself");
    assert_eq!(game.players[0].hand.len(), before + 1);
    assert_eq!(game.players[0].life, 20, "and cost no life to do it");
}

/// Waterlogged Grove is the Simic member: green and blue on the same terms,
/// down to the life and the card at the end.
#[test]
fn the_grove_is_the_green_and_blue_one() {
    let (mut game, grove) = staged_land(cards::WATERLOGGED_GROVE);
    let colors = mana_colors(&game, grove);
    assert!(colors.contains(&ManaColor::Green), "Forest half");
    assert!(colors.contains(&ManaColor::Blue), "Island half");
    assert_eq!(colors.len(), 2, "no colourless and no third colour");

    let ability = mana_ability_for(&game, grove, ManaColor::Blue);
    game.apply(
        PlayerId::One,
        Action::ActivateManaAbility {
            source: grove,
            ability,
            color: ManaColor::Blue,
            counters_removed: None,
            cost_object: None,
            combination: None,
            triggered_mana: None,
        },
    )
    .expect("it taps for blue");

    assert_eq!(game.players[0].mana_pool.blue, 1);
    assert_eq!(game.players[0].life, 19, "one life for one mana");
}

/// Once it has been tapped for mana the card is no longer available: the
/// draw wants the same tap the mana already spent.
#[test]
fn the_grove_cannot_both_pay_and_draw() {
    let (mut game, grove) = staged_land(cards::WATERLOGGED_GROVE);
    let before = game.players[0].hand.len();

    cash_in(&mut game, grove);

    assert_eq!(game.players[0].hand.len(), before + 1, "the card came");
    assert_eq!(game.players[0].life, 20, "and no life was spent on it");
    assert!(
        mana_colors(&game, grove).is_empty(),
        "sacrificed for the card, it makes no mana at all",
    );
}

/// Life paid is not damage taken: a prevention shield that would have eaten
/// an Ancient Tomb's two does nothing here, and the player who pays their
/// last life loses with the mana still in their pool.
#[test]
fn the_life_is_paid_rather_than_dealt() {
    let (mut game, canyon) = staged();
    let angel = card(97_900, cards::GUARDIAN_ANGEL, PlayerId::One);
    let angel_id = angel.id;
    game.players[0].hand.push(angel);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
    game.apply(
        PlayerId::One,
        cast_action(angel_id, vec![Target::Player(PlayerId::One)], Vec::new(), 2),
    )
    .expect("a shield of two is castable");
    drain_pending(&mut game);
    game.empty_mana_pools();
    game.players[0].life = 1;

    let ability = mana_ability_for(&game, canyon, ManaColor::White);
    game.apply(
        PlayerId::One,
        Action::ActivateManaAbility {
            source: canyon,
            ability,
            color: ManaColor::White,
            counters_removed: None,
            cost_object: None,
            combination: None,
            triggered_mana: None,
        },
    )
    .expect("one life is enough to pay one life");
    game.check_state_based_actions();

    assert_eq!(
        game.players[0].life, 0,
        "the shield answers damage, and this is a payment",
    );
    assert_eq!(
        game.players[0].mana_pool.white, 1,
        "the mana was made all the same",
    );
    assert!(game.result.is_some(), "and zero life is a loss");
}
