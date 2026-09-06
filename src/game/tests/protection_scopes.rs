//! Protection whose predicate is a card type or a tribe rather than a
//! colour. Each is a different predicate behind the same keyword, and the
//! two extremes are worth separating: protection from creatures stops every
//! block, while protection from artifacts stops only the artifact ones.

use super::*;

/// `attacker` attacking, with `blocker` opposite it.
fn combat(
    attacker: CardDefinitionId,
    blocker: CardDefinitionId,
) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    let mut threat = creature(74_000, attacker, PlayerId::One);
    threat.attacking = true;
    threat.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let threat_id = threat.card.id;
    game.battlefield.push(threat);
    let mut defender = creature(74_001, blocker, PlayerId::Two);
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
fn protection_from_creatures_stops_every_blocker() {
    for blocker in [cards::GRIZZLY_BEARS, cards::ORNITHOPTER, cards::SERRA_ANGEL] {
        let (game, eesha, defender) = combat(cards::COMMANDER_EESHA, blocker);
        assert!(
            !can_block(&game, defender, eesha),
            "nothing that is a creature may block her"
        );
    }
}

/// Protection from artifacts is narrower: only the artifact creature is
/// turned away.
#[test]
fn protection_from_artifacts_stops_only_the_artifact_ones() {
    let (game, curator, thopter) = combat(cards::ANGELIC_CURATOR, cards::ORNITHOPTER);
    assert!(
        !can_block(&game, thopter, curator),
        "an artifact creature cannot block it"
    );

    // Serra Angel flies, so it can reach the Curator on the ordinary rules.
    let (game, curator, angel) = combat(cards::ANGELIC_CURATOR, cards::SERRA_ANGEL);
    assert!(
        can_block(&game, angel, curator),
        "and a nonartifact flier still can"
    );
}

#[test]
fn protection_from_a_tribe_reads_the_subtype() {
    let (game, stalker, dragon) = combat(cards::DRAGONSTALKER, cards::FURYBORN_HELLKITE);
    assert!(
        !can_block(&game, dragon, stalker),
        "a Dragon cannot block it"
    );

    let (game, stalker, angel) = combat(cards::DRAGONSTALKER, cards::SERRA_ANGEL);
    assert!(
        can_block(&game, angel, stalker),
        "and another flier that is not a Dragon still can"
    );
}
