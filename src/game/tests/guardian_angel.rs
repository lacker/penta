//! Guardian Angel and activated abilities carried by resolved ongoing effects.

use super::*;

fn resolve_guardian_angel(x: u16, mana_after_cast: u8) -> (Game, GameObjectId) {
    resolve_guardian_angel_target(
        ready_game(),
        Target::Player(PlayerId::One),
        x,
        mana_after_cast,
    )
}

fn resolve_guardian_angel_target(
    mut game: Game,
    target: Target,
    x: u16,
    mana_after_cast: u8,
) -> (Game, GameObjectId) {
    let angel = card(10_000, cards::GUARDIAN_ANGEL, PlayerId::One);
    let angel_id = angel.id;
    game.players[PlayerId::One.index()].hand.push(angel);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 1);
    game.add_unrestricted_mana(
        PlayerId::One,
        ManaColor::Colorless,
        x + u16::from(mana_after_cast),
    );

    game.apply(
        PlayerId::One,
        cast_action(angel_id, vec![target], Vec::new(), x),
    )
    .expect("Guardian Angel can be cast for the chosen X");
    drain_pending(&mut game);

    let source = game
        .ongoing_effects
        .first()
        .expect("the resolving spell created its payment effect")
        .source
        .object;
    (game, source)
}

#[test]
fn later_payments_stay_bound_to_the_original_permanent() {
    let mut game = ready_game();
    let protected = creature(10_010, cards::SAVANNAH_LIONS, PlayerId::One);
    let protected_id = protected.card.id;
    let other = creature(10_011, cards::SAVANNAH_LIONS, PlayerId::One);
    let other_id = other.card.id;
    game.battlefield.extend([protected, other]);
    let (mut game, source) =
        resolve_guardian_angel_target(game, Target::Permanent(protected_id), 0, 1);

    activate_ongoing(&mut game, source);
    drain_pending(&mut game);
    game.damage_target_from(None, Some(Target::Permanent(other_id)), 1);
    game.damage_target_from(None, Some(Target::Permanent(protected_id)), 1);

    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == other_id)
            .expect("the unprotected creature remains present")
            .damage,
        1,
        "the payment cannot be redirected to another recipient"
    );
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == protected_id)
            .expect("the protected creature remains present")
            .damage,
        0,
        "the paid shield stays with Guardian Angel's original target"
    );
}

fn activate_ongoing(game: &mut Game, source: GameObjectId) {
    game.priority = PlayerId::One;
    let activate = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source: candidate, .. } if *candidate == source)
        })
        .expect("one mana offers the ongoing effect's activated ability");
    game.apply(PlayerId::One, activate)
        .expect("the ongoing effect can be activated");
}

#[test]
fn zero_x_still_creates_a_repeatable_payment_effect_until_cleanup() {
    let (mut game, source) = resolve_guardian_angel(0, 2);
    assert!(
        game.damage_preventions.is_empty(),
        "X=0 creates no empty prevention shield"
    );
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != source),
        "the ongoing effect is not a permanent"
    );
    assert!(
        game.emblems.iter().all(|emblem| emblem.card.id != source),
        "and the command-zone approximation is not an emblem"
    );

    activate_ongoing(&mut game, source);
    drain_pending(&mut game);
    activate_ongoing(&mut game, source);
    drain_pending(&mut game);
    assert_eq!(
        game.damage_preventions.len(),
        2,
        "each payment creates its own next-one-damage shield"
    );

    game.damage_target_from(None, Some(Target::Player(PlayerId::One)), 3);
    assert_eq!(
        game.players[PlayerId::One.index()].life,
        i16::from(rules::STARTING_LIFE) - 1,
        "the two paid shields prevent two damage"
    );

    game.finish_cleanup();
    assert!(
        game.ongoing_effects.is_empty(),
        "the payment effect expires"
    );
    game.priority = PlayerId::One;
    assert!(
        !game.legal_actions(PlayerId::One).iter().any(
            |action| matches!(action, Action::ActivateAbility { source: candidate, .. } if *candidate == source)
        ),
        "an expired effect cannot be activated"
    );
}

#[test]
fn stifle_can_counter_the_ongoing_effects_activated_ability() {
    let (mut game, source) = resolve_guardian_angel(2, 1);
    let stifle = card(10_001, cards::STIFLE, PlayerId::Two);
    let stifle_id = stifle.id;
    game.players[PlayerId::Two.index()].hand.push(stifle);
    game.add_unrestricted_mana(PlayerId::Two, ManaColor::Blue, 1);

    activate_ongoing(&mut game, source);
    let ability = game
        .stack
        .last()
        .expect("the activation uses the ordinary stack")
        .id;
    game.priority = PlayerId::Two;
    game.apply(
        PlayerId::Two,
        cast_action(stifle_id, vec![Target::Spell(ability)], Vec::new(), 0),
    )
    .expect("Stifle can target the ongoing effect's activation");
    drain_pending(&mut game);

    assert_eq!(
        game.damage_preventions.len(),
        1,
        "the countered payment did not add another shield"
    );
    game.damage_target_from(None, Some(Target::Player(PlayerId::One)), 3);
    assert_eq!(
        game.players[PlayerId::One.index()].life,
        i16::from(rules::STARTING_LIFE) - 1,
        "only the original X=2 prevention remains"
    );
}

#[test]
fn an_ongoing_effect_checkpoint_round_trip_preserves_its_action() {
    let (game, source) = resolve_guardian_angel(0, 1);
    let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
    assert_eq!(
        wire["checkpoint"]["hasDeferredState"],
        serde_json::Value::Bool(false),
        "the effect has stable catalog semantics"
    );
    let rebuilt =
        Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 4_242)
            .expect("the ongoing effect reconstructs");

    assert_eq!(rebuilt.ongoing_effects, game.ongoing_effects);
    assert!(
        rebuilt.legal_actions(PlayerId::One).iter().any(
            |action| matches!(action, Action::ActivateAbility { source: candidate, .. } if *candidate == source)
        ),
        "the reconstructed effect exposes the same activation"
    );
}
