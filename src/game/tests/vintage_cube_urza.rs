//! Urza, Lord High Artificer: a Construct that grows with the board, a blue
//! mana out of every artifact, and a mana sink that buys a free card.

use super::*;

fn staged(artifacts: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[PlayerId::One.index()].hand.clear();
    game.players[PlayerId::One.index()].library.clear();
    for index in 0..4 {
        game.players[PlayerId::One.index()].library.push(card(
            96_000 + index,
            cards::LIGHTNING_BOLT,
            PlayerId::One,
        ));
    }
    for (index, definition) in artifacts.iter().enumerate() {
        game.battlefield.push(creature(
            96_100 + u32::try_from(index).expect("few artifacts"),
            *definition,
            PlayerId::One,
        ));
    }
    let urza = game
        .put_onto_battlefield(PlayerId::One, cards::URZA_LORD_HIGH_ARTIFICER)
        .expect("cataloged");
    drain_pending(&mut game);
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, urza)
}

fn construct(game: &Game) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| {
            game.effective_subtypes(permanent)
                .contains(crate::card::Subtype::named("Construct"))
        })
        .expect("the Construct is there")
}

/// The Construct counts every artifact, itself included.
#[test]
fn the_construct_counts_the_artifacts() {
    let (game, _) = staged(&[]);
    assert_eq!(
        (
            game.power(construct(&game)),
            game.toughness(construct(&game))
        ),
        (Some(1), Some(1)),
        "a lone Construct counts only itself",
    );

    let (game, _) = staged(&[cards::MOX_JET, cards::BLACK_LOTUS]);
    assert_eq!(
        (
            game.power(construct(&game)),
            game.toughness(construct(&game))
        ),
        (Some(3), Some(3)),
    );
}

/// Every untapped artifact is a blue mana.
#[test]
fn artifacts_tap_for_blue() {
    let (mut game, _) = staged(&[cards::MOX_JET]);
    let mox = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::MOX_JET)
        .expect("it is there")
        .card
        .id;

    // Urza's is an activation that names the artifact it taps, rather than
    // the tap-yourself shape an ordinary mana rock offers.
    let tap = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility { cost_objects, .. } => cost_objects.contains(&mox),
            _ => false,
        })
        .expect("the Mox is one of the artifacts he can tap");
    game.apply(PlayerId::One, tap).expect("it activates");
    drain_pending(&mut game);

    assert_eq!(
        game.players[PlayerId::One.index()].mana_pool.blue,
        1,
        "an artifact tapped for blue rather than for what it prints",
    );
}

/// Five mana shuffles and exiles the top card, and that card may be played
/// for nothing this turn.
#[test]
fn five_mana_buys_a_free_card() {
    let (mut game, urza) = staged(&[]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 5);

    let dig = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, cost_objects, .. }
            if *source == urza && cost_objects.is_empty())
        })
        .expect("five mana buys it");
    game.apply(PlayerId::One, dig).expect("it activates");
    drain_pending(&mut game);

    let exiled = game.players[PlayerId::One.index()]
        .exile
        .iter()
        .map(|card| card.definition)
        .collect::<Vec<_>>();
    assert_eq!(exiled, vec![cards::LIGHTNING_BOLT], "one card is exiled");
    assert!(
        game.legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::CastSpell { .. })),
        "and it may be cast with no mana left at all",
    );
}
