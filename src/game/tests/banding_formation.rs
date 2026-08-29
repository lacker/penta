//! Declaring an attacking band.
//!
//! CR 702.21b: one or more attacking creatures with banding, plus up to one
//! without. These drive the declaration through the legal-action list rather
//! than setting band indices by hand, because the printed limit lives in what
//! the game is willing to offer.

use super::*;

/// A declare-attackers step with `definitions` on the battlefield for player
/// one, none of them yet attacking.
fn ready_to_attack(definitions: &[CardDefinitionId]) -> (Game, Vec<GameObjectId>) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;
    game.step = Step::DeclareAttackers;
    let ids = definitions
        .iter()
        .enumerate()
        .map(|(index, definition)| {
            let permanent = creature(
                10_000 + u32::try_from(index).expect("a small index"),
                *definition,
                PlayerId::One,
            );
            let id = permanent.card.id;
            game.battlefield.push(permanent);
            id
        })
        .collect();
    (game, ids)
}

fn attack(game: &mut Game, attacker: GameObjectId) {
    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker,
            defender: AttackDefender::Player(PlayerId::Two),
        },
    )
    .expect("the attack is legal");
}

/// The band declarations on offer, as unordered pairs.
fn offered_bands(game: &Game) -> Vec<(GameObjectId, GameObjectId)> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::BandAttackers { first, second } => Some((first, second)),
            _ => None,
        })
        .collect()
}

fn band(game: &mut Game, first: GameObjectId, second: GameObjectId) {
    let (first, second) = if first.0 <= second.0 {
        (first, second)
    } else {
        (second, first)
    };
    game.apply(PlayerId::One, Action::BandAttackers { first, second })
        .expect("the band is legal");
}

fn band_of(game: &Game, attacker: GameObjectId) -> Option<u8> {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == attacker)
        .expect("still on the battlefield")
        .attacking_band
}

#[test]
fn two_creatures_with_banding_may_attack_as_a_band() {
    let (mut game, ids) = ready_to_attack(&[cards::BENALISH_HERO, cards::MESA_PEGASUS]);
    attack(&mut game, ids[0]);
    attack(&mut game, ids[1]);

    assert_eq!(offered_bands(&game).len(), 1, "the one pair, once");
    band(&mut game, ids[0], ids[1]);

    let index = band_of(&game, ids[0]).expect("the Hero is in a band");
    assert_eq!(band_of(&game, ids[1]), Some(index), "and so is the Pegasus");
    assert!(
        offered_bands(&game).is_empty(),
        "they are already banded together"
    );
}

/// The control. Two creatures without banding cannot be a band at all, so the
/// declaration is never offered.
#[test]
fn creatures_without_banding_are_not_offered_a_band() {
    let (mut game, ids) = ready_to_attack(&[cards::SAVANNAH_LIONS, cards::SAVANNAH_LIONS]);
    attack(&mut game, ids[0]);
    attack(&mut game, ids[1]);

    assert!(offered_bands(&game).is_empty());
}

#[test]
fn one_creature_without_banding_may_join_and_a_second_may_not() {
    let (mut game, ids) = ready_to_attack(&[
        cards::BENALISH_HERO,
        cards::SAVANNAH_LIONS,
        cards::SAVANNAH_LIONS,
    ]);
    for id in &ids {
        attack(&mut game, *id);
    }

    // Only the Hero can anchor a band, so both pairings that include it are
    // offered and the pair of Lions is not.
    assert_eq!(offered_bands(&game).len(), 2);

    band(&mut game, ids[0], ids[1]);

    assert!(
        offered_bands(&game).is_empty(),
        "the band already has its one creature without banding"
    );
    assert_eq!(band_of(&game, ids[2]), None);
}

/// Two creatures with banding can carry a third that has none, which is the
/// case the "up to one" limit is actually about.
#[test]
fn a_band_of_two_bandits_takes_one_passenger() {
    let (mut game, ids) = ready_to_attack(&[
        cards::BENALISH_HERO,
        cards::MESA_PEGASUS,
        cards::SAVANNAH_LIONS,
    ]);
    for id in &ids {
        attack(&mut game, *id);
    }
    band(&mut game, ids[0], ids[1]);
    band(&mut game, ids[0], ids[2]);

    let index = band_of(&game, ids[0]).expect("banded");
    assert_eq!(band_of(&game, ids[1]), Some(index));
    assert_eq!(band_of(&game, ids[2]), Some(index));
}

/// A band attacks one defender as a unit, so creatures pointed at different
/// things are never offered one another.
#[test]
fn attackers_on_different_defenders_cannot_band() {
    let (mut game, ids) = ready_to_attack(&[cards::BENALISH_HERO, cards::MESA_PEGASUS]);
    let mut walker = creature(10_500, cards::VRASKA_THE_UNSEEN, PlayerId::Two);
    walker.set_counters(CounterKind::Loyalty, 5);
    let walker_id = walker.card.id;
    game.battlefield.push(walker);

    attack(&mut game, ids[0]);
    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker: ids[1],
            defender: AttackDefender::Planeswalker(walker_id),
        },
    )
    .expect("the planeswalker is a legal defender");

    assert!(
        offered_bands(&game).is_empty(),
        "one is after the player and one after the planeswalker"
    );
}

/// The Skirmishers hand first strike to the rest of their band and to nobody
/// else, which is the whole point of a predicate that reads band membership.
#[test]
fn the_skirmishers_arm_their_band_and_not_the_rest_of_the_attack() {
    let (mut game, ids) = ready_to_attack(&[
        cards::ICATIAN_SKIRMISHERS,
        cards::BENALISH_HERO,
        cards::SAVANNAH_LIONS,
    ]);
    for id in &ids {
        attack(&mut game, *id);
    }
    band(&mut game, ids[0], ids[1]);
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .expect("attackers are declared");
    drain_pending(&mut game);

    let has_first_strike = |id: GameObjectId| {
        let permanent = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .expect("still there");
        game.permanent_has_executable_keyword(permanent, KeywordAbility::FirstStrike)
    };
    assert!(has_first_strike(ids[1]), "the Hero is banded with them");
    assert!(
        !has_first_strike(ids[2]),
        "the Lions attacked alongside, not in the band"
    );
}
