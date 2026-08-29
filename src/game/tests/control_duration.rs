//! Control changes that last as long as a permanent does.
//!
//! The turn-scoped form is ended by cleanup. This one outlives the turn and
//! ends when its holder does, so what these drive is the difference: the turn
//! passing without giving the permanent back, and the holder leaving giving it
//! back immediately.

use super::*;

fn aladdin_game() -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 1;
    let aladdin = creature(10_000, cards::ALADDIN, PlayerId::One);
    let aladdin_id = aladdin.card.id;
    game.battlefield.push(aladdin);
    let ring = creature(10_001, cards::SOL_RING, PlayerId::Two);
    let ring_id = ring.card.id;
    game.battlefield.push(ring);
    game.players[PlayerId::One.index()].mana_pool.red = 2;
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;
    (game, aladdin_id, ring_id)
}

fn steal(game: &mut Game, source: GameObjectId, victim: GameObjectId) {
    // Thrull Champion is itself a Thrull, so the intended target has to be
    // named rather than taken from whichever activation comes first.
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source: actual,
                targets,
                ..
            } => {
                *actual == source
                    && targets
                        .iter()
                        .flat_map(crate::casting::TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(victim))
            }
            _ => false,
        })
        .expect("the ability is offered against that target");
    game.apply(PlayerId::One, action)
        .expect("the ability activates");
    pass_priority_pair(game);
}

fn controller(game: &Game, id: GameObjectId) -> PlayerId {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("the permanent is on the battlefield")
        .controller
}

#[test]
fn the_stolen_permanent_stays_stolen_across_the_turn() {
    let (mut game, aladdin_id, ring_id) = aladdin_game();
    steal(&mut game, aladdin_id, ring_id);
    assert_eq!(controller(&game, ring_id), PlayerId::One);

    game.finish_cleanup();

    assert_eq!(
        controller(&game, ring_id),
        PlayerId::One,
        "this is not a turn-scoped steal, so cleanup does not give it back"
    );
}

#[test]
fn losing_the_holder_gives_the_permanent_back() {
    let (mut game, aladdin_id, ring_id) = aladdin_game();
    steal(&mut game, aladdin_id, ring_id);
    assert_eq!(controller(&game, ring_id), PlayerId::One);

    game.battlefield
        .retain(|permanent| permanent.card.id != aladdin_id);
    game.check_state_based_actions();

    assert_eq!(
        controller(&game, ring_id),
        PlayerId::Two,
        "the holder left, so the artifact went home"
    );
}

/// "For as long as *you control* this creature" is not the same as "for as
/// long as this creature is on the battlefield": losing the holder to someone
/// else ends the steal too.
#[test]
fn losing_control_of_the_holder_gives_the_permanent_back() {
    let (mut game, aladdin_id, ring_id) = aladdin_game();
    steal(&mut game, aladdin_id, ring_id);

    if let Some(aladdin) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == aladdin_id)
    {
        aladdin.controller = PlayerId::Two;
    }
    game.check_state_based_actions();

    assert_eq!(
        controller(&game, ring_id),
        PlayerId::Two,
        "the holder changed hands, so the steal ended"
    );
}

/// Thrull Champion's own anthem applies to the Thrull it takes, which is the
/// check that the two clauses see the same board.
#[test]
fn thrull_champion_pumps_the_thrull_it_steals() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 1;
    let champion = creature(10_000, cards::THRULL_CHAMPION, PlayerId::One);
    let champion_id = champion.card.id;
    game.battlefield.push(champion);
    let thrull = creature(10_001, cards::BASAL_THRULL, PlayerId::Two);
    let thrull_id = thrull.card.id;
    game.battlefield.push(thrull);

    steal(&mut game, champion_id, thrull_id);

    assert_eq!(controller(&game, thrull_id), PlayerId::One);
    let stolen = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == thrull_id)
        .expect("still there");
    assert_eq!(
        game.power(stolen),
        Some(2),
        "a 1/1 Thrull under the Champion's anthem"
    );
}

/// Rubinia Soulsinger and Willow Satyr pair a control change that lasts while
/// they stay tapped with a choice not to untap. Each half is useless without
/// the other: untapping would hand the creature straight back, and without the
/// choice the untap step would do it every turn.
mod held_while_tapped {
    use super::*;

    fn rubinia_game() -> (Game, GameObjectId, GameObjectId) {
        let mut game = ready_game();
        game.turns_started[PlayerId::One.index()] = 1;
        let rubinia = creature(10_000, cards::RUBINIA_SOULSINGER, PlayerId::One);
        let rubinia_id = rubinia.card.id;
        game.battlefield.push(rubinia);
        let victim = creature(10_001, cards::SEDGE_TROLL, PlayerId::Two);
        let victim_id = victim.card.id;
        game.battlefield.push(victim);
        (game, rubinia_id, victim_id)
    }

    #[test]
    fn untapping_the_holder_hands_the_creature_back() {
        let (mut game, rubinia_id, victim_id) = rubinia_game();
        steal(&mut game, rubinia_id, victim_id);
        assert_eq!(controller(&game, victim_id), PlayerId::One);

        if let Some(rubinia) = game
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == rubinia_id)
        {
            rubinia.tapped = false;
        }
        game.check_state_based_actions();

        assert_eq!(
            controller(&game, victim_id),
            PlayerId::Two,
            "the hold is on the tap, so untapping ends it"
        );
    }

    /// The choice is what makes keeping the creature possible, so the untap
    /// step has to offer leaving the holder tapped as well as untapping it.
    #[test]
    fn the_untap_step_offers_leaving_the_holder_tapped() {
        let (mut game, rubinia_id, victim_id) = rubinia_game();
        steal(&mut game, rubinia_id, victim_id);

        let choices = game.untap_actions(PlayerId::One);
        let untaps_rubinia = |action: &Action| matches!(action, Action::ChooseUntap { permanents } if permanents.contains(&rubinia_id));
        assert!(
            choices.iter().any(untaps_rubinia),
            "untapping is still allowed"
        );
        assert!(
            choices.iter().any(|action| !untaps_rubinia(action)),
            "and so is leaving it tapped, which is the whole point"
        );
    }

    /// An ordinary permanent gets no such choice: every untap declaration the
    /// step offers includes it.
    #[test]
    fn an_ordinary_permanent_is_not_given_the_choice() {
        let mut game = ready_game();
        let mut troll = creature(10_000, cards::SEDGE_TROLL, PlayerId::One);
        troll.tapped = true;
        let troll_id = troll.card.id;
        game.battlefield.push(troll);
        game.turns_started[PlayerId::One.index()] = 1;

        let choices = game.untap_actions(PlayerId::One);
        assert!(!choices.is_empty());
        assert!(
            choices.iter().all(|action| matches!(
                action,
                Action::ChooseUntap { permanents } if permanents.contains(&troll_id)
            )),
            "untapping is mandatory without a card saying otherwise"
        );
    }
}
