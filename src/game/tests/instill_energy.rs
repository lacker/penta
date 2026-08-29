//! Attacking as though hasty, which is less than haste.
//!
//! The Aura buys the attack and nothing else: the enchanted creature is still
//! summoning sick for its own {T} ability. The free untap alongside it is
//! rationed by the printed "only once each turn" rather than by its cost.

use super::*;

/// A Ley Druid that came down this turn, an Island for it to point at, and
/// `enchanted` deciding whether Instill Energy is on it.
fn summoning_sick(enchanted: bool) -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;
    game.battlefield
        .push(creature(10_000, cards::ISLAND, PlayerId::One));

    let mut druid = creature(10_001, cards::LEY_DRUID, PlayerId::One);
    druid.entered_controller_turn = 5;
    let druid_id = druid.card.id;
    game.battlefield.push(druid);

    let mut aura_id = druid_id;
    if enchanted {
        let mut aura = creature(10_002, cards::INSTILL_ENERGY, PlayerId::One);
        aura.attached_to = Some(druid_id);
        aura_id = aura.card.id;
        game.battlefield.push(aura);
    }
    (game, druid_id, aura_id)
}

fn may_attack(game: &mut Game, attacker: GameObjectId) -> bool {
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.legal_actions(PlayerId::One)
        .iter()
        .any(|action| matches!(action, Action::DeclareAttacker { attacker: actual, .. } if *actual == attacker))
}

fn offers(game: &Game, source: GameObjectId) -> bool {
    game.legal_actions(PlayerId::One).iter().any(
        |action| matches!(action, Action::ActivateAbility { source: actual, .. } if *actual == source),
    )
}

fn tapped(game: &Game, id: GameObjectId) -> bool {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still there")
        .tapped
}

/// The control: without the Aura the same creature is stuck.
#[test]
fn a_creature_that_just_arrived_cannot_attack() {
    let (mut game, druid, _) = summoning_sick(false);
    assert!(!may_attack(&mut game, druid));
}

#[test]
fn the_aura_lets_it_attack_the_turn_it_arrives() {
    let (mut game, druid, _) = summoning_sick(true);
    assert!(may_attack(&mut game, druid));
}

/// Where "as though it had haste" stops. The Druid's own ability costs {T},
/// and summoning sickness still forbids that -- so the Aura is not haste
/// under another name.
#[test]
fn the_aura_does_not_unlock_the_creatures_own_tap_ability() {
    let (mut game, druid, _) = summoning_sick(true);
    game.step = Step::PrecombatMain;
    assert!(
        !offers(&game, druid),
        "the tap ability is still summoning sick",
    );

    // And the ability is otherwise live, so the silence above is sickness
    // rather than a missing target or an unbuilt ability.
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == druid)
        .expect("still there")
        .entered_controller_turn = 4;
    assert!(offers(&game, druid), "a turn older, it is on offer");
}

#[test]
fn the_free_untap_untaps_the_enchanted_creature() {
    let (mut game, druid, aura) = summoning_sick(true);
    game.step = Step::PrecombatMain;
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == druid)
        .expect("still there")
        .tapped = true;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == aura))
        .expect("the Aura offers its ability");
    game.apply(PlayerId::One, action)
        .expect("nothing to pay makes it legal");
    drain_pending(&mut game);

    assert!(!tapped(&game, druid), "the Druid came back up");
    assert!(
        !offers(&game, aura),
        "and the once-each-turn clause closes the ability for the rest of the turn",
    );
}

/// The window is whose turn it is, not which step.
#[test]
fn the_free_untap_is_not_offered_on_the_opponents_turn() {
    let (mut game, _druid, aura) = summoning_sick(true);
    game.step = Step::PrecombatMain;
    assert!(offers(&game, aura), "your own turn opens it");

    game.active_player = PlayerId::Two;
    assert!(!offers(&game, aura), "theirs does not");
}
