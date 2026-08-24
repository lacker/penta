//! Umezawa's Jitte: charge counters, and an activated ability with modes.

use super::*;

/// Equips `source` to `host` by finding the printed equip activation.
fn equip_to(game: &mut Game, source: GameObjectId, host: GameObjectId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source: actual,
                targets,
                ..
            } => {
                *actual == source
                    && targets
                        .iter()
                        .flat_map(crate::casting::TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(host))
            }
            _ => false,
        })
        .expect("equip is offered for that creature");
    game.apply(PlayerId::One, action)
        .expect("the ability activates");
    drain_pending(game);
}

/// Activates the Jitte's modal ability, choosing the mode at `index` and
/// whatever target the picked action carries.
fn spend_counter(game: &mut Game, source: GameObjectId, mode: usize, target: Option<GameObjectId>) {
    let wanted = ModeId::from_index(mode).expect("three printed modes");
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source: actual,
                modes,
                targets,
                ..
            } => {
                *actual == source
                    && modes.as_slice() == [wanted]
                    && target.is_none_or(|host| {
                        targets
                            .iter()
                            .flat_map(crate::casting::TargetSelection::targets)
                            .any(|chosen| *chosen == Target::Permanent(host))
                    })
            }
            _ => false,
        })
        .expect("that mode is offered");
    game.apply(PlayerId::One, action)
        .expect("the ability activates");
    drain_pending(game);
}

fn jitte_on_bears(game: &mut Game) -> (GameObjectId, GameObjectId) {
    game.battlefield.clear();
    let jitte = creature(52_000, cards::UMEZAWAS_JITTE, PlayerId::One);
    let jitte_id = jitte.card.id;
    game.battlefield.push(jitte);
    let bears = creature(52_001, cards::GRIZZLY_BEARS, PlayerId::One);
    let bears_id = bears.card.id;
    game.battlefield.push(bears);
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;
    equip_to(game, jitte_id, bears_id);
    (jitte_id, bears_id)
}

fn permanent(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still on the battlefield")
}

/// Combat damage to a blocking creature charges the Jitte just as a hit to
/// the player would: the printed clause names no recipient.
#[test]
fn combat_damage_to_anything_puts_two_charge_counters_on_the_jitte() {
    let mut game = ready_game();
    let (jitte_id, bears_id) = jitte_on_bears(&mut game);
    let wall = creature(52_002, cards::SERRA_ANGEL, PlayerId::Two);
    let wall_id = wall.card.id;
    game.battlefield.push(wall);

    game.damage_target_from_kind(Some(bears_id), Some(Target::Permanent(wall_id)), 2, true);
    drain_pending(&mut game);

    assert_eq!(
        permanent(&game, jitte_id).counters(CounterKind::named("charge")),
        2,
        "two counters per damage event, not per point",
    );
}

/// The first mode pumps whatever the Jitte equips, and spends one counter.
#[test]
fn the_first_mode_pumps_the_equipped_creature() {
    let mut game = ready_game();
    let (jitte_id, bears_id) = jitte_on_bears(&mut game);
    game.damage_target_from_kind(Some(bears_id), Some(Target::Player(PlayerId::Two)), 2, true);
    drain_pending(&mut game);

    spend_counter(&mut game, jitte_id, 0, None);

    let bears = permanent(&game, bears_id);
    assert_eq!(
        (game.power(bears), game.toughness(bears)),
        (Some(4), Some(4)),
        "a 2/2 with +2/+2",
    );
    assert_eq!(
        permanent(&game, jitte_id).counters(CounterKind::named("charge")),
        1,
        "one of the two counters was spent",
    );
}

/// The second mode shrinks a creature the Jitte never touched, which is what
/// makes it removal. A 1/1 dies outright.
#[test]
fn the_second_mode_shrinks_a_target_creature_to_death() {
    let mut game = ready_game();
    let (jitte_id, bears_id) = jitte_on_bears(&mut game);
    let mouse = creature(52_003, cards::SAVANNAH_LIONS, PlayerId::Two);
    let mouse_id = mouse.card.id;
    game.battlefield.push(mouse);
    game.damage_target_from_kind(Some(bears_id), Some(Target::Player(PlayerId::Two)), 2, true);
    drain_pending(&mut game);

    spend_counter(&mut game, jitte_id, 1, Some(mouse_id));

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != mouse_id),
        "a 2/1 loses its last toughness",
    );
    assert_eq!(
        permanent(&game, jitte_id).counters(CounterKind::named("charge")),
        1
    );
}

/// The third mode touches no permanent at all.
#[test]
fn the_third_mode_gains_two_life() {
    let mut game = ready_game();
    let (jitte_id, bears_id) = jitte_on_bears(&mut game);
    game.damage_target_from_kind(Some(bears_id), Some(Target::Player(PlayerId::Two)), 2, true);
    drain_pending(&mut game);
    let life = game.players[PlayerId::One.index()].life;

    spend_counter(&mut game, jitte_id, 2, None);

    assert_eq!(game.players[PlayerId::One.index()].life, life + 2);
    assert_eq!(
        permanent(&game, jitte_id).counters(CounterKind::named("charge")),
        1
    );
}

/// One printed ability, three offers: the mode is chosen as the ability is
/// activated, so each selection is a legal action of its own. Without a
/// counter to remove, none of them is offered.
#[test]
fn each_mode_is_its_own_activation_and_none_is_offered_without_a_counter() {
    let mut game = ready_game();
    let (jitte_id, bears_id) = jitte_on_bears(&mut game);

    let modal = |game: &Game| {
        game.legal_actions(PlayerId::One)
            .into_iter()
            .filter(|action| {
                matches!(
                    action,
                    Action::ActivateAbility { source, modes, .. }
                        if *source == jitte_id && !modes.is_empty()
                )
            })
            .count()
    };
    assert_eq!(modal(&game), 0, "the cost cannot be paid yet");

    game.damage_target_from_kind(Some(bears_id), Some(Target::Player(PlayerId::Two)), 2, true);
    drain_pending(&mut game);

    // Two modes name nothing, and the third names the only creature on the
    // battlefield -- the Jitte itself is an Equipment.
    assert_eq!(modal(&game), 3);
}
