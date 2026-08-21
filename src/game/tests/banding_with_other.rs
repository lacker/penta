//! "Bands with other", the banding variant CR 702.21j narrows to a quality.
//!
//! It differs from plain banding at both ends. Forming a band takes no free
//! passenger and instead wants every member to share the named quality, and
//! the damage rule wants two qualifying creatures rather than one. These
//! drive both differences, because a variant that behaved like plain banding
//! would pass any test written only against a band that formed.

use super::*;

/// A declare-attackers step for player one with `definitions` on the
/// battlefield, none of them attacking yet, and `lands` beside them.
fn board(
    definitions: &[CardDefinitionId],
    lands: &[CardDefinitionId],
) -> (Game, Vec<GameObjectId>) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;
    game.step = Step::DeclareAttackers;
    for (index, land) in lands.iter().enumerate() {
        let permanent = creature(
            30_000 + u32::try_from(index).expect("a small index"),
            *land,
            PlayerId::One,
        );
        game.battlefield.push(permanent);
    }
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

fn may_band(game: &Game, first: GameObjectId, second: GameObjectId) -> bool {
    let (first, second) = if first.0 <= second.0 {
        (first, second)
    } else {
        (second, first)
    };
    game.legal_actions(PlayerId::One)
        .contains(&Action::BandAttackers { first, second })
}

/// Two Wolves of the Hunt, made by the Master, band on their shared name.
fn two_wolves() -> (Game, Vec<GameObjectId>) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;
    game.step = Step::DeclareAttackers;
    let ids = (0..2)
        .map(|index| {
            let permanent =
                token_permanent(10_000 + index, wolves_of_the_hunt_token(), PlayerId::One);
            let id = permanent.card.id;
            game.battlefield.push(permanent);
            id
        })
        .collect::<Vec<_>>();
    (game, ids)
}

#[test]
fn wolves_of_the_hunt_band_on_their_own_name() {
    let (mut game, ids) = two_wolves();
    for id in &ids {
        attack(&mut game, *id);
    }
    assert!(may_band(&game, ids[0], ids[1]));
}

/// The control, and the difference from plain banding: a creature outside the
/// quality cannot ride along, because the variant has no free passenger.
#[test]
fn a_creature_outside_the_quality_cannot_join() {
    let (mut game, ids) = two_wolves();
    let outsider = creature(10_500, cards::SAVANNAH_LIONS, PlayerId::One);
    let outsider_id = outsider.card.id;
    game.battlefield.push(outsider);
    for id in ids.iter().chain(std::iter::once(&outsider_id)) {
        attack(&mut game, *id);
    }

    assert!(may_band(&game, ids[0], ids[1]), "the two Wolves still can");
    assert!(
        !may_band(&game, ids[0], outsider_id),
        "the Lions are not named Wolves of the Hunt"
    );
}

/// The lands grant the ability rather than printing it, and only to legendary
/// creatures of their own color.
#[test]
fn the_guildhouse_lets_green_legends_band() {
    let (mut game, ids) = board(
        &[cards::JASMINE_BOREAL, cards::LADY_CALERIA],
        &[cards::ADVENTURERS_GUILDHOUSE],
    );
    for id in &ids {
        attack(&mut game, *id);
    }
    assert!(
        may_band(&game, ids[0], ids[1]),
        "one green legend carries the ability and both are legends"
    );
}

/// Without the land nothing grants it, so the same two legends are just two
/// attackers.
#[test]
fn legends_do_not_band_without_a_land_granting_it() {
    let (mut game, ids) = board(&[cards::JASMINE_BOREAL, cards::LADY_CALERIA], &[]);
    for id in &ids {
        attack(&mut game, *id);
    }
    assert!(!may_band(&game, ids[0], ids[1]));
}

/// Every member has to have the quality, so a nonlegendary creature cannot
/// join a band of legends even beside the land that granted it.
#[test]
fn a_nonlegend_cannot_join_a_band_of_legends() {
    let (mut game, ids) = board(
        &[
            cards::JASMINE_BOREAL,
            cards::LADY_CALERIA,
            cards::SAVANNAH_LIONS,
        ],
        &[cards::ADVENTURERS_GUILDHOUSE],
    );
    for id in &ids {
        attack(&mut game, *id);
    }
    assert!(may_band(&game, ids[0], ids[1]));
    assert!(!may_band(&game, ids[0], ids[2]));
}

/// CR 702.21: the variant's damage rule wants at least two creatures of the
/// quality on the other side of the block, one of which carries the ability.
#[test]
fn two_banded_wolves_take_the_blockers_assignment() {
    let (mut game, ids) = two_wolves();
    for id in &ids {
        attack(&mut game, *id);
    }
    game.apply(
        PlayerId::One,
        Action::BandAttackers {
            first: ids[0],
            second: ids[1],
        },
    )
    .expect("the band is legal");
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .expect("attackers are declared");
    drain_pending(&mut game);

    let blocker = creature(20_000, cards::AIR_ELEMENTAL, PlayerId::Two);
    let blocker_id = blocker.card.id;
    game.battlefield.push(blocker);
    game.step = Step::DeclareBlockers;
    for id in &ids {
        game.declare_blocker(blocker_id, *id);
    }
    game.finish_declaring_blockers();
    game.start_combat_damage();

    assert_eq!(
        game.combat_damage_assigner(blocker_id),
        PlayerId::One,
        "the attacking player divides the blocker's damage"
    );
}

/// One Wolf is not two. The printed rule names a pair, so a lone member of the
/// quality leaves the choice where it started.
#[test]
fn one_wolf_alone_does_not_take_the_assignment() {
    let (mut game, ids) = two_wolves();
    attack(&mut game, ids[0]);
    let lions = creature(10_500, cards::SAVANNAH_LIONS, PlayerId::One);
    let lions_id = lions.card.id;
    game.battlefield.push(lions);
    attack(&mut game, lions_id);
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .expect("attackers are declared");
    drain_pending(&mut game);

    let blocker = creature(20_000, cards::AIR_ELEMENTAL, PlayerId::Two);
    let blocker_id = blocker.card.id;
    game.battlefield.push(blocker);
    game.step = Step::DeclareBlockers;
    game.declare_blocker(blocker_id, ids[0]);
    game.declare_blocker(blocker_id, lions_id);
    game.finish_declaring_blockers();
    game.start_combat_damage();

    assert_eq!(
        game.combat_damage_assigner(blocker_id),
        PlayerId::Two,
        "one Wolf and one Lion is not two Wolves"
    );
}

/// The quality is what every member needs; the ability only has to be on one
/// of them. The Guildhouse reaches green legends alone, and a black one rides
/// along on being legendary.
#[test]
fn one_carrier_takes_the_rest_of_the_legends_with_it() {
    let (mut game, ids) = board(
        &[cards::JASMINE_BOREAL, cards::LADY_ORCA],
        &[cards::ADVENTURERS_GUILDHOUSE],
    );
    for id in &ids {
        attack(&mut game, *id);
    }
    assert!(
        may_band(&game, ids[0], ids[1]),
        "Jasmine Boreal is the green one and carries the ability for both"
    );
}

/// Shelkin Brownie strips the variant, and the band it would have formed goes
/// with it. Taking it off the only carrier is what makes the difference: the
/// other legend never had it.
#[test]
fn the_brownie_takes_the_ability_away() {
    let (mut game, ids) = board(
        &[cards::JASMINE_BOREAL, cards::LADY_ORCA],
        &[cards::ADVENTURERS_GUILDHOUSE],
    );
    // The Brownie has to have been out a turn to pay its tap cost.
    game.turns_started[PlayerId::Two.index()] = 5;
    let brownie = creature(20_000, cards::SHELKIN_BROWNIE, PlayerId::Two);
    let brownie_id = brownie.card.id;
    game.battlefield.push(brownie);

    // The Brownie acts before attackers are declared, which is the window its
    // controller has priority in.
    game.step = Step::BeginningOfCombat;
    game.priority = PlayerId::Two;
    let action = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == brownie_id
                    && targets
                        .iter()
                        .flat_map(crate::casting::TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(ids[0]))
            }
            _ => false,
        })
        .expect("the Brownie can point at the green legend");
    game.apply(PlayerId::Two, action)
        .expect("the ability activates");
    drain_pending(&mut game);

    game.step = Step::DeclareAttackers;
    game.priority = PlayerId::One;
    for id in &ids {
        attack(&mut game, *id);
    }
    assert!(
        !may_band(&game, ids[0], ids[1]),
        "nothing carries the ability now"
    );
}
