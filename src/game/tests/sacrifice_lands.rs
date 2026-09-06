//! Lands that cash themselves in for colour. The sacrifice is a mana
//! ability, so it is never offered as an action -- the only evidence it
//! works is that the planner spends the land to pay for something no other
//! permanent on the battlefield could have paid for.

use super::*;

/// `lands` under player one with `spell` in hand and no mana anywhere.
fn staged(lands: &[CardDefinitionId], spell: CardDefinitionId) -> (Game, CardInstanceId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].mana_pool = ManaPool::default();
    game.turns_started[PlayerId::One.index()] = 5;
    for (index, definition) in lands.iter().enumerate() {
        let mut land = creature(
            71_000 + u32::try_from(index).expect("a small fixture"),
            *definition,
            PlayerId::One,
        );
        land.entered_controller_turn = 0;
        game.battlefield.push(land);
    }
    let held = card(71_100, spell, PlayerId::One);
    let held_id = held.id;
    game.players[0].hand.push(held);
    (game, held_id)
}

fn can_cast(game: &Game, spell: CardInstanceId) -> bool {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .any(|action| matches!(action, Action::CastSpell { card, .. } if card == spell))
}

#[test]
fn the_tap_ability_still_makes_its_own_colour() {
    let (game, spell) = staged(&[cards::ANCIENT_SPRING], cards::CAREFUL_STUDY);
    assert!(can_cast(&game, spell), "the Spring taps for {{U}}");
}

#[test]
fn the_sacrifice_pays_for_colours_the_tap_cannot() {
    let (game, spell) = staged(&[], cards::SACRED_NECTAR);
    assert!(!can_cast(&game, spell), "no lands, no spells");

    let (game, spell) = staged(&[cards::ANCIENT_SPRING], cards::SACRED_NECTAR);
    assert!(
        can_cast(&game, spell),
        "{{W}}{{B}} covers {{1}}{{W}}, which one blue mana never could"
    );
}

#[test]
fn taking_the_cast_eats_the_land() {
    let (mut game, spell) = staged(&[cards::ANCIENT_SPRING], cards::SACRED_NECTAR);
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
        .expect("the Spring covers it");
    game.apply(PlayerId::One, cast).expect("the cast is legal");

    assert!(
        game.battlefield.is_empty(),
        "the Spring was sacrificed for the mana"
    );
    assert_eq!(
        game.players[0].graveyard.len(),
        1,
        "and it is in the graveyard rather than merely gone"
    );
}

#[test]
fn the_dig_reaches_any_colour_at_all() {
    let (game, spell) = staged(&[cards::ARCHAEOLOGICAL_DIG], cards::LIGHTNING_BOLT);
    assert!(
        can_cast(&game, spell),
        "colourless while it lives, any colour when it goes"
    );
}
