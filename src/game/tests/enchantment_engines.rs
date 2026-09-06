//! Enchantments that watch or buff the whole table. Two things the catalog
//! cannot check: a trigger that pays "its controller" has to mean the
//! controller of the creature that arrived rather than the enchantment's,
//! and a static grant written for "creatures you control" has to reach a
//! creature that arrives later while leaving the opponent's alone.

use super::*;

/// `enchantments` under player one, plus `spell` in `caster`'s hand with
/// the mana to cast it.
fn staged(
    enchantments: &[CardDefinitionId],
    caster: PlayerId,
    spell: CardDefinitionId,
) -> (Game, CardInstanceId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[1].hand.clear();
    game.turns_started[caster.index()] = 5;
    game.active_player = caster;
    game.priority = caster;
    for (index, definition) in enchantments.iter().enumerate() {
        let mut permanent = creature(
            70_000 + u32::try_from(index).expect("a small fixture"),
            *definition,
            PlayerId::One,
        );
        permanent.entered_controller_turn = 0;
        game.battlefield.push(permanent);
    }
    let held = card(70_100, spell, caster);
    let held_id = held.id;
    game.players[caster.index()].hand.push(held);
    game.add_unrestricted_mana(caster, ManaColor::White, 3);
    game.add_unrestricted_mana(caster, ManaColor::Green, 3);
    game.add_unrestricted_mana(caster, ManaColor::Colorless, 5);
    (game, held_id)
}

/// Resolves `spell` and everything it triggers, returning the hand sizes.
fn resolve(mut game: Game, caster: PlayerId, spell: CardInstanceId) -> (usize, usize) {
    let cast = game
        .legal_actions(caster)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
        .expect("the creature is castable");
    game.apply(caster, cast).expect("the cast is legal");
    for _ in 0..12 {
        drain_pending(&mut game);
        if game.stack.is_empty() {
            break;
        }
        let holder = game.priority;
        if game.apply(holder, Action::PassPriority).is_err() {
            break;
        }
    }
    (game.players[0].hand.len(), game.players[1].hand.len())
}

#[test]
fn the_lair_draws_for_whoever_played_the_creature() {
    let (game, spell) = staged(&[cards::KAVU_LAIR], PlayerId::Two, cards::SERRA_ANGEL);
    assert_eq!(
        resolve(game, PlayerId::Two, spell),
        (0, 1),
        "the Angel is theirs, so the card is theirs -- the Lair is mine"
    );

    let (game, spell) = staged(&[cards::KAVU_LAIR], PlayerId::One, cards::SERRA_ANGEL);
    assert_eq!(
        resolve(game, PlayerId::One, spell),
        (1, 0),
        "and mine draws for me"
    );
}

#[test]
fn the_lair_ignores_a_small_creature() {
    let (game, spell) = staged(&[cards::KAVU_LAIR], PlayerId::One, cards::GRIZZLY_BEARS);
    assert_eq!(
        resolve(game, PlayerId::One, spell),
        (0, 0),
        "a 2/2 is under the threshold"
    );
}

/// A summoning-sick creature each side, with `enchantments` under player one.
fn sick_boards(enchantments: &[CardDefinitionId]) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.turns_started = [5, 5];
    for (index, definition) in enchantments.iter().enumerate() {
        let mut permanent = creature(
            70_200 + u32::try_from(index).expect("a small fixture"),
            *definition,
            PlayerId::One,
        );
        permanent.entered_controller_turn = 0;
        game.battlefield.push(permanent);
    }
    let mut mine = creature(70_300, cards::GRIZZLY_BEARS, PlayerId::One);
    mine.entered_controller_turn = 5;
    let mine_id = mine.card.id;
    game.battlefield.push(mine);
    let mut theirs = creature(70_301, cards::GRIZZLY_BEARS, PlayerId::Two);
    theirs.entered_controller_turn = 5;
    let theirs_id = theirs.card.id;
    game.battlefield.push(theirs);
    (game, mine_id, theirs_id)
}

fn can_attack(game: &mut Game, player: PlayerId, attacker: GameObjectId) -> bool {
    game.active_player = player;
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.priority = player;
    game.legal_actions(player).iter().any(
        |action| matches!(action, Action::DeclareAttacker { attacker: a, .. } if *a == attacker),
    )
}

#[test]
fn the_fires_hasten_only_my_side() {
    let (mut game, mine, _) = sick_boards(&[]);
    assert!(
        !can_attack(&mut game, PlayerId::One, mine),
        "without it a creature that arrived this turn stays home"
    );

    let (mut game, mine, theirs) = sick_boards(&[cards::FIRES_OF_YAVIMAYA]);
    assert!(
        can_attack(&mut game, PlayerId::One, mine),
        "the grant reaches a creature that was already there"
    );
    assert!(
        !can_attack(&mut game, PlayerId::Two, theirs),
        "and stops at the other side of the table"
    );
}
