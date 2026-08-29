//! "Players can't untap more than one ... during their untap steps."
//!
//! A cap on the turn-based action rather than a prohibition: the player still
//! picks which one, and permanents the cap does not name untap as usual. Two
//! caps compose, each narrowing its own group -- and a permanent covered by
//! both cannot satisfy one and let a second through the other.

use super::*;

/// Player one, several turns in, with `sources` already on the battlefield.
fn board(sources: &[CardDefinitionId]) -> Game {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    for (index, definition) in sources.iter().enumerate() {
        let id = 9_000 + u32::try_from(index).expect("a short list fits");
        game.battlefield
            .push(creature(id, *definition, PlayerId::One));
    }
    game
}

/// Adds a tapped permanent under player one and returns its id.
fn tapped(game: &mut Game, id: u32, definition: CardDefinitionId) -> GameObjectId {
    let mut permanent = creature(id, definition, PlayerId::One);
    permanent.tapped = true;
    let permanent_id = permanent.card.id;
    game.battlefield.push(permanent);
    permanent_id
}

/// How many of `group` the largest offered untap declaration takes.
fn most_untapped(game: &Game, group: &[GameObjectId]) -> usize {
    game.untap_actions(PlayerId::One)
        .iter()
        .filter_map(|action| match action {
            Action::ChooseUntap { permanents } => {
                Some(group.iter().filter(|id| permanents.contains(id)).count())
            }
            _ => None,
        })
        .max()
        .unwrap_or(0)
}

#[test]
fn without_a_cap_everything_untaps_together() {
    let mut game = board(&[]);
    let lands = [
        tapped(&mut game, 10_000, cards::MOUNTAIN),
        tapped(&mut game, 10_001, cards::MOUNTAIN),
    ];

    assert_eq!(most_untapped(&game, &lands), 2);
}

#[test]
fn winter_orb_holds_the_lands_to_one() {
    let mut game = board(&[cards::WINTER_ORB]);
    let lands = [
        tapped(&mut game, 10_000, cards::MOUNTAIN),
        tapped(&mut game, 10_001, cards::MOUNTAIN),
    ];

    assert_eq!(most_untapped(&game, &lands), 1);
}

/// The Orb's own condition sits outside the cap, so tapping it lifts the cap
/// without touching anyone's lands.
#[test]
fn a_tapped_orb_caps_nothing() {
    let mut game = board(&[cards::WINTER_ORB]);
    game.battlefield[0].tapped = true;
    let lands = [
        tapped(&mut game, 10_000, cards::MOUNTAIN),
        tapped(&mut game, 10_001, cards::MOUNTAIN),
    ];

    assert_eq!(most_untapped(&game, &lands), 2);
}

/// The cap names lands, so creatures are untouched by it.
#[test]
fn the_orb_leaves_creatures_alone() {
    let mut game = board(&[cards::WINTER_ORB]);
    let creatures = [
        tapped(&mut game, 10_000, cards::SEDGE_TROLL),
        tapped(&mut game, 10_001, cards::SEDGE_TROLL),
    ];

    assert_eq!(most_untapped(&game, &creatures), 2);
}

#[test]
fn smoke_holds_the_creatures_to_one() {
    let mut game = board(&[cards::SMOKE]);
    let creatures = [
        tapped(&mut game, 10_000, cards::SEDGE_TROLL),
        tapped(&mut game, 10_001, cards::SEDGE_TROLL),
    ];

    assert_eq!(most_untapped(&game, &creatures), 1);
}

#[test]
fn damping_field_holds_the_artifacts_to_one() {
    let mut game = board(&[cards::DAMPING_FIELD]);
    let artifacts = [
        tapped(&mut game, 10_000, cards::SOL_RING),
        tapped(&mut game, 10_001, cards::SOL_RING),
    ];

    assert_eq!(most_untapped(&game, &artifacts), 1);
}

/// Two caps at once, each still holding its own group to one.
#[test]
fn two_caps_each_narrow_their_own_group() {
    let mut game = board(&[cards::WINTER_ORB, cards::DAMPING_FIELD]);
    let lands = [
        tapped(&mut game, 10_000, cards::MOUNTAIN),
        tapped(&mut game, 10_001, cards::MOUNTAIN),
    ];
    let artifacts = [
        tapped(&mut game, 10_002, cards::SOL_RING),
        tapped(&mut game, 10_003, cards::SOL_RING),
    ];

    assert_eq!(most_untapped(&game, &lands), 1);
    assert_eq!(most_untapped(&game, &artifacts), 1);
    let both: Vec<_> = lands.iter().chain(artifacts.iter()).copied().collect();
    assert_eq!(most_untapped(&game, &both), 2, "one from each group");
}
