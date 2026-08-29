//! "For as long as this artifact remains tapped."
//!
//! Every other resolving duration has a deadline the effect can be filed
//! under. This one has none: the artifact that tapped to make the bonus
//! decides when it ends by untapping, which may be several turns later or
//! never. So the source is recorded rather than a deadline, and the bonus is
//! read against it -- which is also what lets cleanup leave it alone.

use super::*;

/// Tawnos's Weaponry pointed at a Sedge Troll, both under player one.
fn weaponry_on_a_troll() -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    let weaponry = creature(10_000, cards::TAWNOSS_WEAPONRY, PlayerId::One);
    let weaponry_id = weaponry.card.id;
    game.battlefield.push(weaponry);
    let troll = creature(10_001, cards::SEDGE_TROLL, PlayerId::One);
    let troll_id = troll.card.id;
    game.battlefield.push(troll);
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == weaponry_id
                    && targets
                        .iter()
                        .flat_map(crate::casting::TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(troll_id))
            }
            _ => false,
        })
        .expect("the Weaponry can point at the Troll");
    game.apply(PlayerId::One, action)
        .expect("the ability activates");
    drain_pending(&mut game);
    (game, weaponry_id, troll_id)
}

fn stats(game: &Game, permanent: GameObjectId) -> (Option<i16>, Option<i16>) {
    let permanent = game
        .battlefield
        .iter()
        .find(|candidate| candidate.card.id == permanent)
        .expect("still there");
    (game.power(permanent), game.toughness(permanent))
}

#[test]
fn the_bonus_lasts_while_the_artifact_stays_tapped() {
    let (game, weaponry, troll) = weaponry_on_a_troll();

    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == weaponry)
            .expect("still there")
            .tapped,
        "the tap was the cost"
    );
    // Sedge Troll is a 2/2.
    assert_eq!(stats(&game, troll), (Some(3), Some(3)));
}

/// The half a cleanup-bounded bonus would get wrong: end of turn comes and
/// goes, and the bonus is still there because the artifact still is tapped.
#[test]
fn cleanup_does_not_end_it() {
    let (mut game, _, troll) = weaponry_on_a_troll();

    game.finish_cleanup();

    assert_eq!(stats(&game, troll), (Some(3), Some(3)));
}

/// And the half a permanent bonus would get wrong.
#[test]
fn untapping_the_artifact_ends_it() {
    let (mut game, weaponry, troll) = weaponry_on_a_troll();

    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == weaponry)
        .expect("still there")
        .tapped = false;

    assert_eq!(stats(&game, troll), (Some(2), Some(2)));
}

/// A bonus whose source has untapped is spent, so cleanup drops the record
/// rather than leaving it to accumulate.
#[test]
fn cleanup_drops_a_spent_bonus() {
    let (mut game, weaponry, troll) = weaponry_on_a_troll();
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == weaponry)
        .expect("still there")
        .tapped = false;

    game.finish_cleanup();
    // Tapping it again does not bring the old bonus back.
    let _ = game.tap_permanent(weaponry);

    assert_eq!(stats(&game, troll), (Some(2), Some(2)));
}

/// Ashnod's Battle Gear shrinks toughness, so the same duration has to carry
/// a negative modifier as readily as a positive one.
#[test]
fn a_negative_modifier_rides_the_same_duration() {
    let mut game = ready_game();
    let gear = creature(10_000, cards::ASHNODS_BATTLE_GEAR, PlayerId::One);
    let gear_id = gear.card.id;
    game.battlefield.push(gear);
    // A 5/5, since a 2/2 losing two toughness would simply die.
    let troll = creature(10_001, cards::SHIVAN_DRAGON, PlayerId::One);
    let troll_id = troll.card.id;
    game.battlefield.push(troll);
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == gear_id
                    && targets
                        .iter()
                        .flat_map(crate::casting::TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(troll_id))
            }
            _ => false,
        })
        .expect("the Gear can point at the Troll");
    game.apply(PlayerId::One, action)
        .expect("the ability activates");
    drain_pending(&mut game);

    assert_eq!(stats(&game, troll_id), (Some(7), Some(3)));
}

/// Castle reads the same question the other way round: the condition is on
/// the creature receiving the bonus rather than on the source.
#[test]
fn castle_covers_only_the_untapped_creatures() {
    let mut game = ready_game();
    let troll = creature(10_000, cards::SEDGE_TROLL, PlayerId::One);
    let troll_id = troll.card.id;
    game.battlefield.push(troll);
    let mut tapped = creature(10_001, cards::SEDGE_TROLL, PlayerId::One);
    tapped.tapped = true;
    let tapped_id = tapped.card.id;
    game.battlefield.push(tapped);
    let theirs = creature(10_002, cards::SEDGE_TROLL, PlayerId::Two);
    let theirs_id = theirs.card.id;
    game.battlefield.push(theirs);
    game.battlefield
        .push(creature(10_003, cards::CASTLE, PlayerId::One));

    assert_eq!(stats(&game, troll_id), (Some(2), Some(4)));
    assert_eq!(stats(&game, tapped_id), (Some(2), Some(2)), "tapped");
    assert_eq!(stats(&game, theirs_id), (Some(2), Some(2)), "theirs");
}
