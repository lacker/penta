//! Ancient Tomb: two colourless from an untapped land, paid for in damage
//! rather than in life -- which is a different thing when something is
//! watching, or preventing.

use super::*;

/// The Tomb on the battlefield with an empty pool and a full life total.
fn staged() -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let tomb = game
        .put_onto_battlefield(PlayerId::One, cards::ANCIENT_TOMB)
        .expect("cataloged");
    drain_pending(&mut game);
    game.players[0].life = 20;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.empty_mana_pools();
    (game, tomb)
}

fn tap_the_tomb(game: &mut Game, tomb: GameObjectId) {
    let mana = Action::ActivateManaAbility {
        source: tomb,
        ability: mana_ability_for(game, tomb, ManaColor::Colorless),
        color: ManaColor::Colorless,
        counters_removed: None,
        cost_object: None,
        combination: None,
        triggered_mana: None,
    };
    game.apply(PlayerId::One, mana).expect("it taps for two");
    game.check_state_based_actions();
}

/// The land is a land: it pays on the turn it arrives, which is what makes
/// it a fast mana source rather than a slow one.
#[test]
fn it_pays_the_turn_it_arrives() {
    let (mut game, tomb) = staged();
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == tomb)
    {
        permanent.entered_controller_turn = game.turns_started[0];
    }

    tap_the_tomb(&mut game, tomb);

    assert_eq!(game.players[0].mana_pool.colorless, 2);
    assert_eq!(game.players[0].life, 18, "and two damage on the way");
}

/// Two damage, not two life lost: a prevention shield eats it whole and the
/// mana arrives anyway.
#[test]
fn the_two_is_damage_and_can_be_prevented() {
    let (mut game, tomb) = staged();
    let angel = card(97_000, cards::GUARDIAN_ANGEL, PlayerId::One);
    let angel_id = angel.id;
    game.players[0].hand.push(angel);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
    game.apply(
        PlayerId::One,
        cast_action(angel_id, vec![Target::Player(PlayerId::One)], Vec::new(), 2),
    )
    .expect("two mana and a white buys a shield of two");
    drain_pending(&mut game);
    game.empty_mana_pools();

    tap_the_tomb(&mut game, tomb);

    assert_eq!(
        game.players[0].life, 20,
        "the shield answered the land, which life loss would have walked past",
    );
    assert_eq!(
        game.players[0].mana_pool.colorless, 2,
        "and the mana was added regardless: the damage is not a cost",
    );
}

/// Nothing checks whether you can afford it. At two life the Tomb is a way
/// to lose the game with mana still in your pool.
#[test]
fn it_will_kill_its_own_controller() {
    let (mut game, tomb) = staged();
    game.players[0].life = 2;

    tap_the_tomb(&mut game, tomb);

    assert_eq!(game.players[0].life, 0);
    assert_eq!(
        game.players[0].mana_pool.colorless, 2,
        "the mana is there to spend, and there is nobody left to spend it",
    );
    assert!(game.result.is_some(), "zero life is a loss");
}
