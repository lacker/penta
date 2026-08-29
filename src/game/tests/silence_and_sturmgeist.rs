//! A prohibition that reaches mana abilities, and a body defined by the hand.
//!
//! Stony Silence shuts off every artifact's activations including the ones
//! that make mana -- which are enumerated on their own path, so the rule has
//! to be read in two places. Sturmgeist reads its own controller's hand live.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.battlefield.clear();
    game.players[PlayerId::One.index()].hand.clear();
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

/// The mana ability of an artifact goes too, which a prohibition read only on
/// the ordinary activation path would miss.
#[test]
fn stony_silence_reaches_an_artifacts_mana_ability() {
    let mut game = ready();
    game.put_onto_battlefield(PlayerId::Two, cards::MOX_SAPPHIRE)
        .expect("cataloged");
    let mox = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::MOX_SAPPHIRE)
        .expect("it is there")
        .card
        .id;
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }

    let taps_for_mana = |game: &Game| {
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == mox)
            .is_some_and(|permanent| !game.mana_ability_activations(permanent).is_empty())
    };
    assert!(taps_for_mana(&game), "the Mox works to begin with");

    game.battlefield
        .push(creature(10_000, cards::STONY_SILENCE, PlayerId::One));
    assert!(
        !taps_for_mana(&game),
        "and the Silence stops it, whoever controls it",
    );
}

/// A land is not an artifact, so its mana ability is untouched.
#[test]
fn stony_silence_leaves_lands_alone() {
    let mut game = ready();
    game.put_onto_battlefield(PlayerId::One, cards::ISLAND)
        .expect("cataloged");
    game.battlefield
        .push(creature(10_000, cards::STONY_SILENCE, PlayerId::One));
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }

    let island = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::ISLAND)
        .expect("it is there");
    assert!(
        !game.mana_ability_activations(island).is_empty(),
        "the clause names artifacts",
    );
}

/// The Sturmgeist grows and shrinks with the hand it reads.
#[test]
fn the_sturmgeist_is_as_big_as_your_hand() {
    let mut game = ready();
    let geist = creature(10_000, cards::STURMGEIST, PlayerId::One);
    let geist_id = geist.card.id;
    game.battlefield.push(geist);

    assert_eq!(stats(&game, geist_id), (Some(0), Some(0)), "an empty hand");

    for index in 0..3 {
        game.players[PlayerId::One.index()].hand.push(card(
            30_000 + index,
            cards::GRIZZLY_BEARS,
            PlayerId::One,
        ));
    }
    assert_eq!(stats(&game, geist_id), (Some(3), Some(3)));

    // The opponent's hand is not yours.
    game.players[PlayerId::Two.index()].hand.push(card(
        31_000,
        cards::GRIZZLY_BEARS,
        PlayerId::Two,
    ));
    assert_eq!(stats(&game, geist_id), (Some(3), Some(3)), "still three");

    game.players[PlayerId::One.index()].hand.pop();
    assert_eq!(stats(&game, geist_id), (Some(2), Some(2)), "and it shrinks");
}

/// The Authority reads the enchanted creature's controller's hand, not the
/// Aura controller's -- the two are different players here.
#[test]
fn the_authority_reads_the_creatures_controllers_hand() {
    let mut game = ready();
    let bear = creature(10_000, cards::GRIZZLY_BEARS, PlayerId::Two);
    let bear_id = bear.card.id;
    game.battlefield.push(bear);
    let mut aura = creature(10_001, cards::RIGHTEOUS_AUTHORITY, PlayerId::One);
    aura.attached_to = Some(bear_id);
    game.battlefield.push(aura);

    assert_eq!(
        stats(&game, bear_id),
        (Some(2), Some(2)),
        "both hands empty"
    );

    // The Aura's controller draws: the creature is unmoved.
    for index in 0..3 {
        game.players[PlayerId::One.index()].hand.push(card(
            30_000 + index,
            cards::GRIZZLY_BEARS,
            PlayerId::One,
        ));
    }
    assert_eq!(
        stats(&game, bear_id),
        (Some(2), Some(2)),
        "the Aura controller's hand is the wrong one",
    );

    for index in 0..2 {
        game.players[PlayerId::Two.index()].hand.push(card(
            31_000 + index,
            cards::GRIZZLY_BEARS,
            PlayerId::Two,
        ));
    }
    assert_eq!(
        stats(&game, bear_id),
        (Some(4), Some(4)),
        "and the creature controller's hand is the right one",
    );
}

/// The extra draw follows the same player, on their draw step.
#[test]
fn the_authority_draws_for_the_creatures_controller() {
    let mut game = ready();
    let bear = creature(10_000, cards::GRIZZLY_BEARS, PlayerId::Two);
    let bear_id = bear.card.id;
    game.battlefield.push(bear);
    let mut aura = creature(10_001, cards::RIGHTEOUS_AUTHORITY, PlayerId::One);
    aura.attached_to = Some(bear_id);
    game.battlefield.push(aura);
    for index in 0..4 {
        game.players[PlayerId::Two.index()].library.push(card(
            31_000 + index,
            cards::GRIZZLY_BEARS,
            PlayerId::Two,
        ));
    }

    let hands = |game: &Game| [game.players[0].hand.len(), game.players[1].hand.len()];
    let before = hands(&game);

    game.active_player = PlayerId::Two;
    game.step = Step::Draw;
    game.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
        step: TurnStepDef::Draw,
        player: PlayerId::Two,
    });
    drain_pending(&mut game);

    assert_eq!(
        hands(&game),
        [before[0], before[1] + 1],
        "their draw step, their extra card",
    );
}
