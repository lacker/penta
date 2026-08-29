//! Detain.
//!
//! Three restrictions that always travel together and end at one moment: the
//! detaining player's next turn. What these check is each restriction
//! separately, and that the end really is the *next* turn rather than the
//! nearest cleanup.

use super::*;

fn detained_board(victim: CardDefinitionId) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 1;
    game.turns_started[PlayerId::Two.index()] = 1;
    let victim = creature(10_001, victim, PlayerId::Two);
    let victim_id = victim.card.id;
    game.battlefield.push(victim);

    let arrester = card(10_000, cards::AZORIUS_ARRESTER, PlayerId::One);
    let arrester_id = arrester.id;
    game.players[PlayerId::One.index()].hand.push(arrester);
    game.players[PlayerId::One.index()].mana_pool.white = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == arrester_id))
        .expect("the Arrester is castable");
    game.apply(PlayerId::One, action)
        .expect("the spell is cast");
    drain_pending(&mut game);
    (game, victim_id)
}

fn can_activate(game: &Game, source: GameObjectId) -> bool {
    game.legal_actions(PlayerId::Two).iter().any(
        |action| matches!(action, Action::ActivateAbility { source: actual, .. } if *actual == source),
    )
}

#[test]
fn a_detained_permanent_cannot_activate_its_abilities() {
    let (game, victim_id) = detained_board(cards::MISHRA_S_FACTORY);
    assert!(
        !can_activate(&game, victim_id),
        "Mishra's Factory cannot animate while detained"
    );
}

#[test]
fn a_detained_creature_cannot_attack() {
    let (mut game, victim_id) = detained_board(cards::SEDGE_TROLL);
    game.active_player = PlayerId::Two;
    game.step = Step::DeclareAttackers;
    assert!(
        !game.legal_actions(PlayerId::Two).iter().any(|action| {
            matches!(action, Action::DeclareAttacker { attacker, .. } if *attacker == victim_id)
        }),
        "a detained creature is not offered as an attacker"
    );
}

#[test]
fn a_detained_creature_cannot_block() {
    let (mut game, victim_id) = detained_board(cards::SEDGE_TROLL);
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    let mut attacker = creature(10_002, cards::SEDGE_TROLL, PlayerId::One);
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    game.battlefield.push(attacker);

    assert!(
        !game.legal_actions(PlayerId::Two).iter().any(|action| {
            matches!(action, Action::DeclareBlocker { blocker, .. } if *blocker == victim_id)
        }),
        "a detained creature is not offered as a blocker"
    );
}

/// "Until your next turn" outlasts the turn it resolved on, which a
/// cleanup-scoped restriction would not.
#[test]
fn detain_outlasts_the_turn_it_landed_on() {
    let (mut game, victim_id) = detained_board(cards::MISHRA_S_FACTORY);
    game.finish_cleanup();
    assert!(
        !can_activate(&game, victim_id),
        "the detaining player's turn has not come round yet"
    );

    // The detaining player's next turn begins, which is what commits the
    // expiry rather than any cleanup in between. Whose priority it is then is
    // a different question, so this reads the restriction itself.
    game.commit_next_turn(PlayerId::One, Vec::new());

    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == victim_id)
            .expect("still on the battlefield")
            .detained_until_turn_of
            .is_none(),
        "and it ends when that turn arrives"
    );
}

/// Detain from an activated ability rather than an entry trigger, and from a
/// spell -- the restriction is the same wherever it comes from.
#[test]
fn detain_works_from_an_activated_ability() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 1;
    let mage = creature(10_000, cards::NEW_PRAHV_GUILDMAGE, PlayerId::One);
    let mage_id = mage.card.id;
    game.battlefield.push(mage);
    // A nonland permanent, since that is what this ability names, and one
    // with an activated ability so the lock is observable.
    let victim = creature(10_001, cards::SEDGE_TROLL, PlayerId::Two);
    let victim_id = victim.card.id;
    game.battlefield.push(victim);
    game.players[PlayerId::Two.index()].mana_pool.black = 1;
    game.players[PlayerId::One.index()].mana_pool.white = 1;
    game.players[PlayerId::One.index()].mana_pool.blue = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 3;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == mage_id
                    && targets
                        .iter()
                        .flat_map(crate::casting::TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(victim_id))
            }
            _ => false,
        })
        .expect("the Guildmage can detain that permanent");
    game.apply(PlayerId::One, action)
        .expect("the ability activates");
    drain_pending(&mut game);

    assert!(
        !can_activate(&game, victim_id),
        "detained by an activated ability just the same"
    );
}
