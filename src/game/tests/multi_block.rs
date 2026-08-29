//! Blocking more than one attacker.
//!
//! One block is what every creature gets; the printed permission on the
//! Two-Headed Giant is what raises that to two. What these check is the
//! difference between the two, since a creature that could always block
//! twice would pass a test written only against the Giant.

use super::*;

/// Two 1/1 attackers on player two, plus one blocker of `definition`.
fn two_attackers(definition: CardDefinitionId) -> (Game, [GameObjectId; 2], GameObjectId) {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    let mut attackers = [GameObjectId(0); 2];
    for (index, slot) in attackers.iter_mut().enumerate() {
        let mut attacker = creature(
            10_000 + u32::try_from(index).expect("a small index"),
            cards::MERFOLK_OF_THE_PEARL_TRIDENT,
            PlayerId::One,
        );
        attacker.attacking = true;
        attacker.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
        *slot = attacker.card.id;
        game.battlefield.push(attacker);
    }
    let blocker = creature(10_002, definition, PlayerId::Two);
    let blocker_id = blocker.card.id;
    game.battlefield.push(blocker);
    (game, attackers, blocker_id)
}

/// Which attackers this creature is still offered a block against.
fn offered_against(game: &Game, blocker: GameObjectId) -> Vec<GameObjectId> {
    game.legal_actions(PlayerId::Two)
        .into_iter()
        .filter_map(|action| match action {
            Action::DeclareBlocker {
                blocker: actual,
                attacker,
            } if actual == blocker => Some(attacker),
            _ => None,
        })
        .collect()
}

fn block(game: &mut Game, blocker: GameObjectId, attacker: GameObjectId) {
    game.apply(PlayerId::Two, Action::DeclareBlocker { blocker, attacker })
        .expect("the block is legal");
}

/// The control: an ordinary creature is offered both attackers to begin with,
/// and neither once it has committed to one.
#[test]
fn an_ordinary_creature_blocks_once() {
    let (mut game, attackers, blocker_id) = two_attackers(cards::SAVANNAH_LIONS);
    assert_eq!(
        offered_against(&game, blocker_id).len(),
        2,
        "either attacker, before it picks one"
    );

    block(&mut game, blocker_id, attackers[0]);

    assert!(
        offered_against(&game, blocker_id).is_empty(),
        "one block is all an ordinary creature gets"
    );
}

#[test]
fn the_giant_blocks_a_second_attacker() {
    let (mut game, attackers, giant_id) = two_attackers(cards::TWO_HEADED_GIANT_OF_FORIYS);

    block(&mut game, giant_id, attackers[0]);

    assert_eq!(
        offered_against(&game, giant_id),
        vec![attackers[1]],
        "the additional block is still on offer, against the other attacker"
    );

    block(&mut game, giant_id, attackers[1]);

    let giant = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == giant_id)
        .expect("still there");
    assert_eq!(giant.blocking, attackers.to_vec());
    assert!(
        offered_against(&game, giant_id).is_empty(),
        "an additional creature is one more, not any number"
    );
}

/// Both attackers are blocked, so neither connects, and the Giant takes both
/// their damage at once. Blocking twice would be hollow if the damage step
/// only ran one of the pairings.
#[test]
fn both_blocked_attackers_deal_their_damage_to_the_giant() {
    let (mut game, attackers, giant_id) = two_attackers(cards::TWO_HEADED_GIANT_OF_FORIYS);
    block(&mut game, giant_id, attackers[0]);
    block(&mut game, giant_id, attackers[1]);

    let before = game.players[PlayerId::Two.index()].life;
    game.deal_combat_damage();

    assert_eq!(
        game.players[PlayerId::Two.index()].life,
        before,
        "both attackers are blocked"
    );
    let giant = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == giant_id)
        .expect("a 4/4 survives two 1/1s");
    assert_eq!(giant.damage, 2, "one from each attacker it blocked");
}
