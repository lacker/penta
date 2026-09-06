//! Cycling, which lives on the card in hand rather than on a permanent. A
//! land with it is the shape most likely to be missed: the only way to use
//! a land from hand is normally to play it, so the cycling ability has to
//! be offered as an action in its own right.

use super::*;

/// Player one holding `held`, with `mana` colourless available.
fn holding(held: CardDefinitionId, mana: u16) -> (Game, CardInstanceId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    let card = card(77_000, held, PlayerId::One);
    let card_id = card.id;
    game.players[0].hand.push(card);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, mana);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, mana);
    (game, card_id)
}

fn cycling_of(game: &Game, held: CardInstanceId) -> Option<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == held))
}

#[test]
fn a_cycling_land_may_be_cycled_from_hand() {
    let (game, moor) = holding(cards::BARREN_MOOR, 0);
    assert!(
        cycling_of(&game, moor).is_none(),
        "with no mana there is nothing to pay the {{B}} with"
    );

    let (game, moor) = holding(cards::BARREN_MOOR, 1);
    assert!(
        cycling_of(&game, moor).is_some(),
        "one black mana buys the cycling"
    );
}

#[test]
fn cycling_it_draws_and_discards_it() {
    let (mut game, moor) = holding(cards::BARREN_MOOR, 1);
    let hand_before = game.players[0].hand.len();
    let library_before = game.players[0].library.len();
    let cycle = cycling_of(&game, moor).expect("the cycling is offered");
    game.apply(PlayerId::One, cycle)
        .expect("the cost is payable");
    pass_priority_pair(&mut game);

    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == ObjectKind::Card(cards::BARREN_MOOR)),
        "the land was discarded as the cost"
    );
    assert_eq!(
        game.players[0].library.len(),
        library_before - 1,
        "and a card was drawn for it"
    );
    assert_eq!(
        game.players[0].hand.len(),
        hand_before,
        "one card in, one card out"
    );
}

#[test]
fn the_land_can_still_simply_be_played() {
    let (game, moor) = holding(cards::BARREN_MOOR, 1);
    assert!(
        game.legal_actions(PlayerId::One)
            .into_iter()
            .any(|action| matches!(action, Action::PlayLand { card, .. } if card == moor)),
        "cycling does not replace the land drop"
    );
}

/// Gilded Light resolved for player one, with `caster` holding a Bolt.
fn gilded(caster: PlayerId) -> (Game, CardInstanceId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[1].hand.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    let light = card(77_100, cards::GILDED_LIGHT, PlayerId::One);
    let light_id = light.id;
    game.players[0].hand.push(light);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == light_id))
        .expect("Gilded Light is castable");
    game.apply(PlayerId::One, cast).expect("the cast is legal");
    pass_priority_pair(&mut game);

    let bolt = card(77_110, cards::LIGHTNING_BOLT, caster);
    let bolt_id = bolt.id;
    game.players[caster.index()].hand.push(bolt);
    game.add_unrestricted_mana(caster, ManaColor::Red, 1);
    game.priority = caster;
    (game, bolt_id)
}

fn player_targets(game: &Game, caster: PlayerId, bolt: CardInstanceId) -> Vec<PlayerId> {
    game.legal_actions(caster)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, choices, .. } if card == bolt => Some(choices),
            _ => None,
        })
        .flat_map(|choices| {
            choices
                .targets()
                .iter()
                .flat_map(TargetSelection::targets)
                .filter_map(|target| match target {
                    Target::Player(player) => Some(*player),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

#[test]
fn gilded_light_hides_its_controller_from_everybody() {
    let (game, bolt) = gilded(PlayerId::Two);
    assert!(
        !player_targets(&game, PlayerId::Two, bolt).contains(&PlayerId::One),
        "the opponent cannot aim at the player who cast it"
    );

    let (game, bolt) = gilded(PlayerId::One);
    assert!(
        !player_targets(&game, PlayerId::One, bolt).contains(&PlayerId::One),
        "and neither can they -- shroud stops everyone"
    );
    assert!(
        player_targets(&game, PlayerId::One, bolt).contains(&PlayerId::Two),
        "while the opponent is still a legal target"
    );
}
