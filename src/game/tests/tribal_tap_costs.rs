//! Activations paid by tapping several creatures of one type. The cost is
//! counted and filtered at once, so what needs covering is that it is not
//! offered one short, that creatures of the wrong type do not make up the
//! number, and that paying it really taps that many.

use super::*;

/// Catapult Master with `soldiers` other Soldiers and `others` non-Soldiers
/// under player one, opposite a Grizzly Bears of player two's.
fn staged(soldiers: usize, others: usize) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    let mut master = creature(84_000, cards::CATAPULT_MASTER, PlayerId::One);
    master.entered_controller_turn = 0;
    game.battlefield.push(master);
    for index in 0..soldiers {
        let mut soldier = creature(
            84_100 + u32::try_from(index).expect("a small fixture"),
            cards::INFANTRY_VETERAN,
            PlayerId::One,
        );
        soldier.entered_controller_turn = 0;
        game.battlefield.push(soldier);
    }
    for index in 0..others {
        let mut other = creature(
            84_200 + u32::try_from(index).expect("a small fixture"),
            cards::GRIZZLY_BEARS,
            PlayerId::One,
        );
        other.entered_controller_turn = 0;
        game.battlefield.push(other);
    }
    let mut victim = creature(84_300, cards::SERRA_ANGEL, PlayerId::Two);
    victim.entered_controller_turn = 0;
    let victim_id = victim.card.id;
    game.battlefield.push(victim);
    (game, victim_id)
}

fn activation(game: &Game, victim: GameObjectId) -> Option<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == GameObjectId(84_000)
                    && targets
                        .iter()
                        .flat_map(TargetSelection::targets)
                        .any(|chosen| *chosen == Target::Permanent(victim))
            }
            _ => false,
        })
}

#[test]
fn four_soldiers_is_one_short() {
    let (game, victim) = staged(3, 0);
    assert!(
        activation(&game, victim).is_none(),
        "the Master plus three Soldiers is four, and it wants five"
    );
}

#[test]
fn bears_do_not_make_up_the_number() {
    let (game, victim) = staged(2, 4);
    assert!(
        activation(&game, victim).is_none(),
        "seven creatures, but only three of them are Soldiers"
    );
}

#[test]
fn five_soldiers_exile_the_angel() {
    let (mut game, victim) = staged(4, 0);
    let activation = activation(&game, victim).expect("the Master plus four others is five");
    game.apply(PlayerId::One, activation)
        .expect("the cost is payable");
    for _ in 0..12 {
        drain_pending(&mut game);
        if game.stack.is_empty() {
            break;
        }
        let holder = game.priority;
        if game.apply(holder, Action::PassPriority).is_err() {
            break;
        }
    }

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == victim),
        "the Angel was exiled"
    );
    assert!(
        game.players[1]
            .exile
            .iter()
            .any(|card| card.definition == ObjectKind::Card(cards::SERRA_ANGEL)),
        "and it is in exile rather than the graveyard"
    );
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| permanent.tapped)
            .count(),
        5,
        "five Soldiers paid for it"
    );
}
