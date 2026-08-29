//! "Can't be blocked except by two or more creatures."
//!
//! A constraint on the finished declaration rather than on any one block. The
//! first blocker is perfectly legal and only becomes illegal by being the
//! last, so what menace does is refuse to let the declaration end there --
//! the defending player commits a second blocker or takes none back.

use super::*;

/// A menacing attacker for player one, with `blockers` untapped creatures
/// under player two.
fn menacing_attack(attacker: CardDefinitionId, blockers: usize) -> (Game, Vec<GameObjectId>) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    let mut attacking = creature(10_000, attacker, PlayerId::One);
    attacking.attacking = true;
    attacking.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    game.battlefield.push(attacking);

    let mut ids = Vec::new();
    for index in 0..blockers {
        let id = 10_100 + u32::try_from(index).expect("a short list fits");
        let blocker = creature(id, cards::SEDGE_TROLL, PlayerId::Two);
        ids.push(blocker.card.id);
        game.battlefield.push(blocker);
    }
    (game, ids)
}

fn attacker_of(game: &Game) -> GameObjectId {
    game.battlefield
        .iter()
        .find(|permanent| permanent.attacking)
        .expect("something is attacking")
        .card
        .id
}

fn may_finish(game: &Game) -> bool {
    game.legal_actions(PlayerId::Two)
        .iter()
        .any(|action| matches!(action, Action::FinishDeclaringBlockers))
}

fn block(game: &mut Game, blocker: GameObjectId) {
    let attacker = attacker_of(game);
    game.apply(PlayerId::Two, Action::DeclareBlocker { blocker, attacker })
        .expect("the block is legal");
}

/// Taking none is always allowed: the constraint is on blocking with exactly
/// one, not on declining.
#[test]
fn letting_it_through_is_always_legal() {
    let (game, _) = menacing_attack(cards::RIPSCALE_PREDATOR, 2);
    assert!(may_finish(&game));
}

/// The first blocker is offered and accepted, and then the declaration
/// cannot end.
#[test]
fn one_blocker_cannot_finish_the_declaration() {
    let (mut game, blockers) = menacing_attack(cards::RIPSCALE_PREDATOR, 2);
    block(&mut game, blockers[0]);

    assert!(
        !may_finish(&game),
        "one is not two, and the rules say two or none"
    );
}

#[test]
fn two_blockers_finish_it() {
    let (mut game, blockers) = menacing_attack(cards::RIPSCALE_PREDATOR, 2);
    block(&mut game, blockers[0]);
    block(&mut game, blockers[1]);

    assert!(may_finish(&game));
}

/// With only one creature available, the defending player has no second
/// blocker to commit -- and so cannot block at all.
#[test]
fn a_lone_blocker_is_stuck_once_it_commits() {
    let (mut game, blockers) = menacing_attack(cards::RIPSCALE_PREDATOR, 1);
    assert!(may_finish(&game), "before committing, letting it through");

    block(&mut game, blockers[0]);
    assert!(!may_finish(&game), "and afterwards, nothing legal is left");
}

/// An ordinary attacker takes one blocker happily, which is what says the
/// constraint above comes from menace rather than the harness.
#[test]
fn an_ordinary_attacker_is_content_with_one() {
    let (mut game, blockers) = menacing_attack(cards::SEDGE_TROLL, 2);
    block(&mut game, blockers[0]);

    assert!(may_finish(&game));
}

/// Granted rather than printed, and read the same way.
#[test]
fn an_aura_can_hand_menace_out() {
    let (mut game, blockers) = menacing_attack(cards::SEDGE_TROLL, 2);
    let attacker = attacker_of(&game);
    let mut aura = creature(10_200, cards::MADCAP_SKILLS, PlayerId::One);
    aura.attached_to = Some(attacker);
    game.battlefield.push(aura);

    block(&mut game, blockers[0]);
    assert!(!may_finish(&game), "the Aura brought the constraint");
    assert_eq!(
        game.power(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == attacker)
                .expect("the attacker is there")
        ),
        Some(5),
        "and three power with it"
    );
}

/// Handed out to a whole side rather than to one creature, and to the
/// enchantment's controller's creatures alone.
#[test]
fn the_war_drums_menace_your_creatures_and_not_theirs() {
    let (mut game, blockers) = menacing_attack(cards::SEDGE_TROLL, 2);
    game.battlefield
        .push(creature(10_200, cards::GOBLIN_WAR_DRUMS, PlayerId::One));

    block(&mut game, blockers[0]);
    assert!(!may_finish(&game), "the Drums brought the constraint");

    let blocker = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == blockers[1])
        .expect("still there");
    assert!(
        !game.permanent_has_executable_keyword(blocker, KeywordAbility::Menace),
        "the other side's creatures are not the Drums' creatures"
    );
}
