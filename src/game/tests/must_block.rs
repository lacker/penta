//! Combat requirements: "all creatures able to block this do so".
//!
//! A requirement never beats a restriction, so "able" is read from the same
//! legality that offers a block in the first place. What the requirement does
//! is take the alternatives away: a creature that could block the lured
//! attacker is offered no other seat, and the defending player cannot finish
//! declaring blockers while one of them is still standing free.

use super::*;

/// Two attackers for player one, with `lured` carrying the requirement.
fn two_attackers(lured: CardDefinitionId, other: CardDefinitionId) -> (Game, [GameObjectId; 2]) {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    let mut ids = [GameObjectId(0); 2];
    for (index, definition) in [lured, other].into_iter().enumerate() {
        let id = 10_000 + u32::try_from(index).expect("two attackers fit");
        let mut attacker = creature(id, definition, PlayerId::One);
        attacker.attacking = true;
        attacker.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
        ids[index] = attacker.card.id;
        game.battlefield.push(attacker);
    }
    (game, ids)
}

fn add_blocker(game: &mut Game, id: u32, definition: CardDefinitionId) -> GameObjectId {
    let blocker = creature(id, definition, PlayerId::Two);
    let blocker_id = blocker.card.id;
    game.battlefield.push(blocker);
    blocker_id
}

/// Which attackers this creature is currently offered as a blocker for.
fn seats(game: &Game, blocker: GameObjectId) -> Vec<GameObjectId> {
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

fn may_finish(game: &Game) -> bool {
    game.legal_actions(PlayerId::Two)
        .iter()
        .any(|action| matches!(action, Action::FinishDeclaringBlockers))
}

/// Marble Priest carries the requirement itself, so nothing has to be
/// attached to it.
#[test]
fn a_wall_is_offered_only_the_creature_it_must_block() {
    let (mut game, [priest, other]) = two_attackers(cards::MARBLE_PRIEST, cards::SEDGE_TROLL);
    let wall = add_blocker(&mut game, 10_010, cards::WALL_OF_STONE);

    assert_eq!(
        seats(&game, wall),
        vec![priest],
        "the requirement takes the other attacker away"
    );
    assert_ne!(priest, other);
}

/// The requirement names Walls, so anything else blocks as it likes.
#[test]
fn a_creature_the_requirement_does_not_name_keeps_every_seat() {
    let (mut game, [priest, other]) = two_attackers(cards::MARBLE_PRIEST, cards::SEDGE_TROLL);
    let lion = add_blocker(&mut game, 10_010, cards::SAVANNAH_LIONS);

    let mut offered = seats(&game, lion);
    offered.sort_unstable();
    let mut expected = vec![priest, other];
    expected.sort_unstable();
    assert_eq!(offered, expected);
}

#[test]
fn the_declaration_cannot_finish_while_a_requirement_is_unmet() {
    let (mut game, [priest, _]) = two_attackers(cards::MARBLE_PRIEST, cards::SEDGE_TROLL);
    let wall = add_blocker(&mut game, 10_010, cards::WALL_OF_STONE);
    assert!(!may_finish(&game), "the Wall is still standing free");

    game.apply(
        PlayerId::Two,
        Action::DeclareBlocker {
            blocker: wall,
            attacker: priest,
        },
    )
    .expect("the required block is legal");

    assert!(may_finish(&game), "the requirement is met");
}

/// A tapped creature is not able to block, so it is not required to, and its
/// controller is free to finish.
#[test]
fn a_creature_that_cannot_block_is_not_required_to() {
    let (mut game, _) = two_attackers(cards::MARBLE_PRIEST, cards::SEDGE_TROLL);
    let wall = add_blocker(&mut game, 10_010, cards::WALL_OF_STONE);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == wall)
        .expect("the Wall is on the battlefield")
        .tapped = true;

    assert!(seats(&game, wall).is_empty());
    assert!(may_finish(&game));
}

/// The Walls that are forced in take nothing for it.
#[test]
fn the_priest_takes_no_combat_damage_from_walls() {
    let (mut game, [priest, _]) = two_attackers(cards::MARBLE_PRIEST, cards::SEDGE_TROLL);
    let wall = add_blocker(&mut game, 10_010, cards::WALL_OF_STONE);
    game.apply(
        PlayerId::Two,
        Action::DeclareBlocker {
            blocker: wall,
            attacker: priest,
        },
    )
    .expect("the required block is legal");
    game.deal_combat_damage();

    let damage = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == priest)
        .map_or(u16::MAX, |permanent| permanent.damage);
    assert_eq!(damage, 0, "a 0/8 Wall deals it nothing");
}

/// Lure puts the same requirement on whatever it enchants, and names every
/// creature rather than only Walls.
#[test]
fn lure_pulls_every_creature_onto_its_host() {
    let (mut game, [host, _]) = two_attackers(cards::GRIZZLY_BEARS, cards::SEDGE_TROLL);
    let lion = add_blocker(&mut game, 10_010, cards::SAVANNAH_LIONS);
    assert_eq!(seats(&game, lion).len(), 2, "free before the Aura arrives");

    let mut lure = creature(10_020, cards::LURE, PlayerId::One);
    lure.attached_to = Some(host);
    game.battlefield.push(lure);

    assert_eq!(seats(&game, lion), vec![host]);
    assert!(!may_finish(&game));
}

/// A ground creature is not able to block a flier, so the Aura's requirement
/// leaves it alone -- the restriction wins.
#[test]
fn a_restriction_beats_the_requirement() {
    let (mut game, [host, other]) = two_attackers(cards::SERRA_ANGEL, cards::SEDGE_TROLL);
    let lion = add_blocker(&mut game, 10_010, cards::SAVANNAH_LIONS);
    let mut lure = creature(10_020, cards::LURE, PlayerId::One);
    lure.attached_to = Some(host);
    game.battlefield.push(lure);

    assert_eq!(
        seats(&game, lion),
        vec![other],
        "it cannot block the flier, so it keeps the seat it can take"
    );
    assert!(may_finish(&game));
}

/// A spell hands the same requirement out for the turn, which is the shape
/// the rule had to reach beyond its two printed statics. Driven through a
/// real cast, because a resolved rule is what is being checked.
#[test]
fn a_spell_can_hand_out_the_requirement_for_the_turn() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    let bear = creature(10_000, cards::GRIZZLY_BEARS, PlayerId::One);
    let bear_id = bear.card.id;
    game.battlefield.push(bear);
    let troll = creature(10_001, cards::SEDGE_TROLL, PlayerId::One);
    let troll_id = troll.card.id;
    game.battlefield.push(troll);
    let lion = creature(10_002, cards::SAVANNAH_LIONS, PlayerId::Two);
    let lion_id = lion.card.id;
    game.battlefield.push(lion);

    let spell = card(10_003, cards::ENLARGE, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    let pool = &mut game.players[PlayerId::One.index()].mana_pool;
    pool.green = 2;
    pool.colorless = 3;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == spell_id
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Permanent(bear_id))
            }
            _ => false,
        })
        .expect("Enlarge can be aimed at the Bears");
    game.apply(PlayerId::One, action).expect("Enlarge is cast");
    drain_pending(&mut game);

    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    for permanent in &mut game.battlefield {
        if permanent.card.id == bear_id || permanent.card.id == troll_id {
            permanent.attacking = true;
            permanent.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
        }
    }

    assert_eq!(
        seats(&game, lion_id),
        vec![bear_id],
        "the enlarged attacker takes the Lions' other seat away"
    );
    assert!(!may_finish(&game));
}
