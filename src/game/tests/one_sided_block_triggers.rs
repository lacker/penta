//! The halves of "blocks or becomes blocked by".
//!
//! One ordered pair of events is emitted per blocking relationship, and each
//! side names the other creature. The union was already expressible; these
//! are the cards that print only one direction, and telling the sides apart
//! is what the halves add.

use super::*;
use crate::ImplementationStatus;

/// Player one attacks with `attacker`; player two blocks with `blocker`.
fn blocked_by(attacker: CardDefinitionId, blocker: CardDefinitionId) -> (Game, [GameObjectId; 2]) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    let mut attacking = creature(10_000, attacker, PlayerId::One);
    attacking.attacking = true;
    attacking.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let attacker_id = attacking.card.id;
    game.battlefield.push(attacking);
    let mut defending = creature(10_001, blocker, PlayerId::Two);
    defending.blocking = vec![attacker_id];
    let blocker_id = defending.card.id;
    game.battlefield.push(defending);
    (game, [attacker_id, blocker_id])
}

/// Commits the block, begins the end-of-combat step, and resolves the delayed
/// triggers created by the blocking abilities.
fn resolve_blocks(game: &mut Game) {
    game.finish_declaring_blockers();
    drain_pending(game);
    game.step = Step::EndOfCombat;
    game.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
        step: TurnStepDef::EndOfCombat,
        player: game.active_player,
    });
    game.finish_rules_procedure();
    drain_pending(game);
}

fn survives(game: &Game, id: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .any(|permanent| permanent.card.id == id)
}

/// The Medusa attacking: the blocker dies, and a Wall would not have.
#[test]
fn the_medusa_kills_what_blocks_it() {
    let (mut game, [medusa, blocker]) = blocked_by(cards::INFERNAL_MEDUSA, cards::SAVANNAH_LIONS);
    resolve_blocks(&mut game);

    assert!(!survives(&game, blocker), "the blocker is destroyed");
    assert!(survives(&game, medusa));
}

/// The half that names Walls is the attacking one, so a Wall that blocks the
/// Medusa walks away.
#[test]
fn a_wall_that_blocks_the_medusa_lives() {
    let (mut game, [_, wall]) = blocked_by(cards::INFERNAL_MEDUSA, cards::WALL_OF_STONE);
    resolve_blocks(&mut game);

    assert!(survives(&game, wall));
}

/// The Medusa blocking: the other half of the card, which spares nothing --
/// a Wall cannot attack, so the exemption never applies on this side.
#[test]
fn the_medusa_kills_what_it_blocks() {
    let (mut game, [attacker, _]) = blocked_by(cards::SEDGE_TROLL, cards::INFERNAL_MEDUSA);
    resolve_blocks(&mut game);

    assert!(!survives(&game, attacker));
}

/// Venom hands the union of both halves to whatever it enchants, so the
/// trigger reads the host's blocking pair rather than the Aura's.
#[test]
fn venom_kills_from_either_side() {
    for (attacker, blocker, doomed) in [
        (cards::GRIZZLY_BEARS, cards::SAVANNAH_LIONS, 1),
        (cards::SEDGE_TROLL, cards::GRIZZLY_BEARS, 0),
    ] {
        let (mut game, ids) = blocked_by(attacker, blocker);
        // The Aura goes on the Bears, whichever side of the block they are.
        let host = ids[usize::from(doomed == 0)];
        let mut aura = creature(10_002, cards::VENOM, PlayerId::One);
        aura.attached_to = Some(host);
        game.battlefield.push(aura);

        resolve_blocks(&mut game);

        assert!(
            !survives(&game, ids[doomed]),
            "the creature on the other side of the block dies"
        );
        assert!(survives(&game, host), "and the enchanted one does not");
    }
}

/// The blocking half on its own: the Cyclops grows when it blocks and not
/// when it is blocked.
#[test]
fn the_cyclops_grows_on_the_side_it_is_on() {
    let (mut game, [_, cyclops]) = blocked_by(cards::SEDGE_TROLL, cards::FORTRESS_CYCLOPS);
    game.finish_declaring_blockers();
    drain_pending(&mut game);

    let blocking = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == cyclops)
        .expect("the Cyclops is there");
    // Its other trigger uses the long-standing attacks event; this one is
    // the half that had no event until the halves were split.
    assert_eq!(game.power(blocking), Some(3), "blocking is not attacking");
    assert_eq!(game.toughness(blocking), Some(6));
}

/// The Vigilante names each blocker rather than the fact of being blocked,
/// so the damage goes to the creature that stopped it.
#[test]
fn the_vigilante_burns_the_creature_that_blocked_it() {
    let (mut game, [_, blocker]) = blocked_by(cards::SOMBERWALD_VIGILANTE, cards::SEDGE_TROLL);
    game.finish_declaring_blockers();
    drain_pending(&mut game);

    let damaged = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == blocker)
        .expect("the blocker is there");
    assert_eq!(damaged.damage, 1);
}

#[test]
fn every_one_sided_block_identity_reports_complete_coverage() {
    let catalog = poc::catalog().expect("catalog builds");
    for definition in [
        cards::FORTRESS_CYCLOPS,
        cards::SOMBERWALD_VIGILANTE,
        cards::HAMLET_CAPTAIN,
        cards::INFERNAL_MEDUSA,
        cards::VENOM,
    ] {
        let card = catalog.get(definition).expect("the card is cataloged");
        assert_eq!(
            card.rules.implementation_status(),
            ImplementationStatus::Complete,
            "{} should be fully executable",
            card.name,
        );
    }
}
