//! One-shot combat prohibitions cast from the stack. The restriction has to
//! reach the creature that was targeted rather than the spell that named
//! it, and "can't attack or block" is two prohibitions that a card can get
//! half right -- so each half is checked against a baseline where the same
//! creature may do it.

use super::*;

/// A Grizzly Bears each side, with `spell` -- if any -- resolved onto
/// player one's.
fn staged(spell: Option<CardDefinitionId>) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    let mut mine = creature(67_000, cards::GRIZZLY_BEARS, PlayerId::One);
    mine.entered_controller_turn = 0;
    let mine_id = mine.card.id;
    game.battlefield.push(mine);
    let mut theirs = creature(67_001, cards::GRIZZLY_BEARS, PlayerId::Two);
    theirs.entered_controller_turn = 0;
    let theirs_id = theirs.card.id;
    game.battlefield.push(theirs);

    if let Some(definition) = spell {
        let held = card(67_010, definition, PlayerId::One);
        let held_id = held.id;
        game.players[0].hand.push(held);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 2);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 2);
        let cast = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| match action {
                Action::CastSpell { card, choices, .. } => {
                    *card == held_id
                        && choices
                            .targets()
                            .iter()
                            .flat_map(TargetSelection::targets)
                            .any(|target| *target == Target::Permanent(mine_id))
                }
                _ => false,
            })
            .expect("the spell is castable onto my Bears");
        game.apply(PlayerId::One, cast).expect("the cast is legal");
        pass_priority_pair(&mut game);
    }
    (game, mine_id, theirs_id)
}

fn can_attack(game: &mut Game, attacker: GameObjectId) -> bool {
    game.active_player = PlayerId::One;
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.priority = PlayerId::One;
    game.legal_actions(PlayerId::One).iter().any(
        |action| matches!(action, Action::DeclareAttacker { attacker: a, .. } if *a == attacker),
    )
}

/// Whether `blocker`, controlled by `defender`, may block `attacker`.
fn can_block(
    game: &mut Game,
    defender: PlayerId,
    blocker: GameObjectId,
    attacker: GameObjectId,
) -> bool {
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == attacker)
        .expect("the attacker is on the battlefield")
        .attacking = true;
    game.active_player = defender.opponent();
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.blockers_declared = false;
    game.priority = defender;
    game.legal_actions(defender).into_iter().any(|action| {
        matches!(
            action,
            Action::DeclareBlocker { blocker: b, attacker: a } if b == blocker && a == attacker
        )
    })
}

#[test]
fn an_untouched_bears_may_do_both() {
    let (mut game, mine, _) = staged(None);
    assert!(can_attack(&mut game, mine), "it may attack");
    let (mut game, mine, theirs) = staged(None);
    assert!(
        can_block(&mut game, PlayerId::One, mine, theirs),
        "and it may block"
    );
}

#[test]
fn off_balance_stops_the_attack() {
    let (mut game, mine, _) = staged(Some(cards::OFF_BALANCE));
    assert!(
        !can_attack(&mut game, mine),
        "the creature it named cannot be declared as an attacker"
    );
}

#[test]
fn off_balance_stops_the_block_as_well() {
    let (mut game, mine, theirs) = staged(Some(cards::OFF_BALANCE));
    assert!(
        !can_block(&mut game, PlayerId::One, mine, theirs),
        "and it cannot be declared as a blocker either"
    );
}

#[test]
fn infiltrate_stops_the_other_side_from_blocking() {
    let (mut game, mine, theirs) = staged(None);
    assert!(
        can_block(&mut game, PlayerId::Two, theirs, mine),
        "their Bears would ordinarily block mine"
    );

    let (mut game, mine, theirs) = staged(Some(cards::INFILTRATE));
    assert!(
        !can_block(&mut game, PlayerId::Two, theirs, mine),
        "but not once mine cannot be blocked this turn"
    );
}
