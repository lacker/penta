//! Auras that hand their host a combat restriction. The three prohibitions
//! are one word apart in the text and one constant apart in the code -- and
//! two of them leave the creature able to do the other thing, which is what
//! separates a Cagemail from a Pacifism.

use super::*;

/// `aura` attached to a Grizzly Bears player one controls, opposite a
/// Grizzly Bears player two controls.
fn enchanted(aura: CardDefinitionId, mana: u16) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started[PlayerId::One.index()] = 5;
    let mut bear = creature(65_000, cards::GRIZZLY_BEARS, PlayerId::One);
    bear.entered_controller_turn = 0;
    let bear_id = bear.card.id;
    game.battlefield.push(bear);
    let mut theirs = creature(65_001, cards::GRIZZLY_BEARS, PlayerId::Two);
    theirs.entered_controller_turn = 0;
    let theirs_id = theirs.card.id;
    game.battlefield.push(theirs);

    let spell = card(65_010, aura, PlayerId::One);
    let spell_id = spell.id;
    game.players[0].hand.push(spell);
    game.players[0].mana_pool.colorless = mana;
    game.players[0].mana_pool.white = mana;
    game.players[0].mana_pool.red = mana;
    game.players[0].mana_pool.blue = mana;
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == spell_id
                    && choices
                        .targets()
                        .iter()
                        .flat_map(TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(bear_id))
            }
            _ => false,
        })
        .expect("the Aura is castable onto the Bears");
    game.apply(PlayerId::One, cast).expect("the cast is legal");
    pass_priority_pair(&mut game);
    (game, bear_id, theirs_id)
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

fn can_block(game: &mut Game, blocker: GameObjectId, attacker: GameObjectId) -> bool {
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == attacker)
        .expect("the attacker is on the battlefield")
        .attacking = true;
    game.active_player = PlayerId::Two;
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.blockers_declared = false;
    game.priority = PlayerId::One;
    game.legal_actions(PlayerId::One).into_iter().any(|action| {
        matches!(
            action,
            Action::DeclareBlocker { blocker: b, attacker: a } if b == blocker && a == attacker
        )
    })
}

#[test]
fn cagemail_stops_the_attack_but_not_the_block() {
    let (mut game, bear, theirs) = enchanted(cards::CAGEMAIL, 2);
    assert!(!can_attack(&mut game, bear), "it cannot attack");
    assert!(
        can_block(&mut game, bear, theirs),
        "and blocking is still open to it, which is the whole difference"
    );
}

#[test]
fn maniacal_rage_stops_the_block_but_not_the_attack() {
    let (mut game, bear, theirs) = enchanted(cards::MANIACAL_RAGE, 2);
    assert!(can_attack(&mut game, bear), "it may still attack");
    assert!(!can_block(&mut game, bear, theirs), "and may not block");
}

#[test]
fn cloak_of_mists_stops_the_opponent_from_blocking_it() {
    let (mut game, bear, theirs) = enchanted(cards::CLOAK_OF_MISTS, 2);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == bear)
        .expect("the Bears are on the battlefield")
        .attacking = true;
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    assert!(
        !game.legal_actions(PlayerId::Two).iter().any(|action| {
            matches!(action, Action::DeclareBlocker { attacker, .. } if *attacker == bear)
        }),
        "nothing may be declared against it"
    );
    let _ = theirs;
}
