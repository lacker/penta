//! Rampage, and the becomes-blocked event underneath it.
//!
//! CR 702.23: whenever this creature becomes blocked, it gets +N/+N until end
//! of turn for each creature blocking it beyond the first. The engine had no
//! becomes-blocked trigger at all, so none of the nine printed rampage cards
//! could be expressed. These tests drive the real declaration path -- declare
//! blockers, finish, and read the attacker's power.

use super::*;

fn rampage_board(walker: CardDefinitionId, blockers: usize) -> Game {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    let mut attacker = creature(10_000, walker, PlayerId::One);
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    game.battlefield.push(attacker);
    for index in 0..blockers {
        let mut blocker = creature(
            11_000 + u32::try_from(index).expect("blocker index fits"),
            cards::SAVANNAH_LIONS,
            PlayerId::Two,
        );
        blocker.blocking = vec![GameObjectId(10_000)];
        game.battlefield.push(blocker);
    }
    game
}

fn attacker_power(game: &Game) -> i16 {
    let attacker = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == GameObjectId(10_000))
        .expect("the attacker is on the battlefield");
    game.power(attacker).expect("the attacker is a creature")
}

fn finish(game: &mut Game) {
    game.finish_declaring_blockers();
    game.finish_rules_procedure();
    for _ in 0..8 {
        if game.stack.is_empty() {
            break;
        }
        let priority = game.priority;
        game.apply(priority, Action::PassPriority)
            .expect("priority passes while the rampage trigger resolves");
    }
}

/// One blocker is not "beyond the first", so rampage pays nothing.
#[test]
fn rampage_pays_nothing_for_a_single_blocker() {
    let mut game = rampage_board(cards::FROST_GIANT, 1);
    let before = attacker_power(&game);
    finish(&mut game);
    assert_eq!(
        attacker_power(&game),
        before,
        "one blocker is the first one"
    );
}

#[test]
fn rampage_pays_its_amount_for_each_blocker_beyond_the_first() {
    // Frost Giant is rampage 2 and a 4/4.
    let mut game = rampage_board(cards::FROST_GIANT, 3);
    finish(&mut game);
    assert_eq!(
        attacker_power(&game),
        8,
        "two blockers beyond the first, at +2/+2 each"
    );
}

/// The printed amounts differ, and each card pays its own rather than the
/// blocker count alone.
#[test]
fn each_printed_rampage_amount_pays_its_own_rate() {
    for (card, base, rate) in [
        (cards::HUNDING_GJORNERSEN, 5, 1),
        (cards::WOLVERINE_PACK, 2, 2),
        (cards::AERATHI_BERSERKER, 2, 3),
    ] {
        let mut game = rampage_board(card, 3);
        finish(&mut game);
        assert_eq!(
            attacker_power(&game),
            base + 2 * rate,
            "two blockers beyond the first at +{rate}/+{rate}",
        );
    }
}

/// Rampage is until end of turn, so it does not follow the creature into the
/// next combat.
#[test]
fn a_rampage_bonus_does_not_survive_cleanup() {
    let mut game = rampage_board(cards::FROST_GIANT, 3);
    finish(&mut game);
    assert_eq!(attacker_power(&game), 8);

    game.finish_cleanup();
    assert_eq!(
        attacker_power(&game),
        4,
        "the until-end-of-turn bonus is gone"
    );
}
