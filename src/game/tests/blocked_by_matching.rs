//! "Can't be blocked by <kind>" is the mirror of fear's "except by": one
//! names who is turned away, the other who is let through. Getting the two
//! the wrong way round produces a card that is nearly unblockable instead of
//! barely evasive, so each is checked against a blocker inside the predicate
//! and one outside it.

use super::*;

fn combat(
    attacker: CardDefinitionId,
    blocker: CardDefinitionId,
) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    let mut threat = creature(76_000, attacker, PlayerId::One);
    threat.attacking = true;
    threat.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let threat_id = threat.card.id;
    game.battlefield.push(threat);
    let mut defender = creature(76_001, blocker, PlayerId::Two);
    defender.entered_controller_turn = 0;
    let defender_id = defender.card.id;
    game.battlefield.push(defender);
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    (game, threat_id, defender_id)
}

fn can_block(game: &Game, blocker: GameObjectId, attacker: GameObjectId) -> bool {
    game.legal_actions(PlayerId::Two).into_iter().any(|action| {
        matches!(
            action,
            Action::DeclareBlocker { blocker: b, attacker: a } if b == blocker && a == attacker
        )
    })
}

#[test]
fn stone_spirit_turns_away_fliers_and_nothing_else() {
    let (game, spirit, angel) = combat(cards::STONE_SPIRIT, cards::SERRA_ANGEL);
    assert!(!can_block(&game, angel, spirit), "a flier may not block it");

    let (game, spirit, bear) = combat(cards::STONE_SPIRIT, cards::GRIZZLY_BEARS);
    assert!(
        can_block(&game, bear, spirit),
        "and a ground creature still may, which is the half a mirrored \
         predicate would lose"
    );
}

#[test]
fn rampart_crawler_turns_away_walls_and_nothing_else() {
    let (game, crawler, wall) = combat(cards::RAMPART_CRAWLER, cards::WALL_OF_STONE);
    assert!(!can_block(&game, wall, crawler), "a Wall may not block it");

    let (game, crawler, bear) = combat(cards::RAMPART_CRAWLER, cards::GRIZZLY_BEARS);
    assert!(can_block(&game, bear, crawler), "anything else may");
}
