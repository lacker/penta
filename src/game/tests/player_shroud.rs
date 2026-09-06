//! Shroud on a player, which is not hexproof on a player. Hexproof is
//! controller-relative and stops only the opponent; shroud stops everyone,
//! its own controller included. It is also not protection: no damage is
//! prevented, so an untargeted burn spell still gets through.

use super::*;

/// Ivory Mask under player one, with `caster` holding a Lightning Bolt.
fn staged(caster: PlayerId) -> (Game, CardInstanceId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[1].hand.clear();
    let mut mask = creature(50_000, cards::IVORY_MASK, PlayerId::One);
    mask.entered_controller_turn = 0;
    game.battlefield.push(mask);
    let bolt = card(50_010, cards::LIGHTNING_BOLT, caster);
    let bolt_id = bolt.id;
    game.players[caster.index()].hand.push(bolt);
    game.players[caster.index()].mana_pool.red = 1;
    game.priority = caster;
    (game, bolt_id)
}

fn player_targets(game: &Game, caster: PlayerId, bolt: CardInstanceId) -> Vec<PlayerId> {
    game.legal_actions(caster)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, choices, .. } if card == bolt => Some(choices),
            _ => None,
        })
        .flat_map(|choices| {
            choices
                .targets()
                .iter()
                .flat_map(TargetSelection::targets)
                .filter_map(|target| match target {
                    Target::Player(player) => Some(*player),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

#[test]
fn an_opponent_cannot_target_the_shrouded_player() {
    let (game, bolt) = staged(PlayerId::Two);
    let targets = player_targets(&game, PlayerId::Two, bolt);
    assert!(
        !targets.contains(&PlayerId::One),
        "the shrouded player is off the list"
    );
    assert!(
        targets.contains(&PlayerId::Two),
        "and the caster is still a legal target"
    );
}

/// The difference from hexproof: shroud is not controller-relative, so the
/// player who owns the Mask cannot point a spell at themselves either.
#[test]
fn the_shrouded_player_cannot_target_themselves() {
    let (game, bolt) = staged(PlayerId::One);
    let targets = player_targets(&game, PlayerId::One, bolt);
    assert!(!targets.contains(&PlayerId::One));
    assert!(
        targets.contains(&PlayerId::Two),
        "pointing it at the opponent is unaffected"
    );
}

/// The difference from protection: nothing is prevented, so damage that
/// never targeted anyone still lands.
#[test]
fn untargeted_damage_still_reaches_them() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    // The Mask protects player two this time, so the active player can be the
    // one casting the sweeper.
    let mut mask = creature(50_000, cards::IVORY_MASK, PlayerId::Two);
    mask.entered_controller_turn = 0;
    game.battlefield.push(mask);
    let earthquake = card(50_020, cards::EARTHQUAKE, PlayerId::One);
    game.players[0].hand.push(earthquake.clone());
    game.players[0].mana_pool.red = 4;

    let cast = cast_action(earthquake.id, Vec::new(), Vec::new(), 3);
    assert!(game.legal_actions(PlayerId::One).contains(&cast));
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.players[1].life, 17, "shroud prevents no damage");
}
