//! Pulling a creature out of the fight it is already in.
//!
//! Both cards are shields with durations, and neither needed anything new:
//! prevention names one side of a damage event, so "to and dealt by it" is
//! two rules rather than one, and the three clauses of a Glyph end at three
//! different times.

use super::*;

/// Player one attacks with `attacker`; player two blocks with `blocker`.
fn combat(attacker: CardDefinitionId, blocker: CardDefinitionId) -> (Game, [GameObjectId; 2]) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    let mut attacking = creature(10_000, attacker, PlayerId::One);
    attacking.attacking = true;
    attacking.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    attacking.tapped = true;
    let attacker_id = attacking.card.id;
    game.battlefield.push(attacking);
    let mut defending = creature(10_001, blocker, PlayerId::Two);
    defending.blocking = vec![attacker_id];
    let blocker_id = defending.card.id;
    game.battlefield.push(defending);
    // Blockers have to be committed before anyone holds priority again,
    // which is the window both of these cards are cast in.
    game.finish_declaring_blockers();
    drain_pending(&mut game);
    (game, [attacker_id, blocker_id])
}

fn permanent(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("the permanent is on the battlefield")
}

fn survives(game: &Game, id: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .any(|permanent| permanent.card.id == id)
}

/// The Scout unties its attacker and takes it out of the damage entirely --
/// in both directions.
#[test]
fn the_scout_pulls_its_attacker_out_of_the_exchange() {
    let (mut game, [attacker, blocker]) = combat(cards::GRIZZLY_BEARS, cards::SEDGE_TROLL);
    let scout = creature(10_002, cards::ELVISH_SCOUT, PlayerId::One);
    let scout_id = scout.card.id;
    game.battlefield.push(scout);
    game.players[PlayerId::One.index()].mana_pool.green = 1;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == scout_id))
        .expect("the Scout can be aimed at the attacker");
    game.apply(PlayerId::One, action)
        .expect("the ability is activated");
    drain_pending(&mut game);

    assert!(!permanent(&game, attacker).tapped, "it is untapped again");

    game.deal_combat_damage();
    game.check_state_based_actions();

    assert!(survives(&game, attacker), "nothing was dealt to it");
    assert_eq!(permanent(&game, blocker).damage, 0, "and none by it either");
}

/// The Glyph makes a Wall lethal and untouchable, then collects it.
#[test]
fn the_glyph_trades_a_wall_for_whatever_it_blocked() {
    let (mut game, [attacker, wall]) = combat(cards::SEDGE_TROLL, cards::WALL_OF_STONE);
    let glyph = card(10_002, cards::GLYPH_OF_DESTRUCTION, PlayerId::One);
    let glyph_id = glyph.id;
    game.players[PlayerId::Two.index()].hand.push(glyph);
    game.players[PlayerId::Two.index()].mana_pool.red = 1;
    game.priority = PlayerId::Two;

    let action = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == glyph_id))
        .expect("the Glyph can be aimed at the blocking Wall");
    game.apply(PlayerId::Two, action)
        .expect("the Glyph is cast");
    drain_pending(&mut game);

    assert_eq!(game.power(permanent(&game, wall)), Some(10));

    game.deal_combat_damage();
    game.check_state_based_actions();

    assert!(!survives(&game, attacker), "ten kills what it blocked");
    assert_eq!(
        permanent(&game, wall).damage,
        0,
        "and the Wall took nothing back"
    );

    game.clear_combat();
    // The trigger fires as the end step begins, so the step has to be
    // entered rather than set.
    game.step = Step::PostcombatMain;
    game.advance_step();
    drain_pending(&mut game);

    assert!(!survives(&game, wall), "the Glyph collects it at end step");
}
