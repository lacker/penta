//! Gaea's Liege's all-zone Forest count and source-bound land conversion.
//!
//! Its characteristic-defining ability changes which player it measures as
//! soon as the Liege attacks. Outside the battlefield it is not attacking, so
//! it counts its owner's Forests. The activated ability is a separate
//! continuous effect: it replaces a land's basic types until this exact Liege
//! leaves.

use super::*;
use crate::ImplementationStatus;

fn permanent(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("the permanent remains on the battlefield")
}

fn stats(game: &Game, id: GameObjectId) -> (i16, i16) {
    let permanent = permanent(game, id);
    (
        game.power(permanent).expect("the Liege is a creature"),
        game.toughness(permanent).expect("the Liege is a creature"),
    )
}

fn stats_in_zone(zone: ZoneKind) -> (i16, i16) {
    let mut game = ready_game();
    game.battlefield.clear();
    for index in 0..3 {
        game.battlefield
            .push(creature(10_100 + index, cards::FOREST, PlayerId::One));
    }
    for index in 0..6 {
        game.battlefield
            .push(creature(10_200 + index, cards::FOREST, PlayerId::Two));
    }
    card_stats_in_zone(game, 10_000, cards::GAEA_S_LIEGE, PlayerId::One, zone)
}

fn staged(yours: u32, theirs: u32) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    let liege = creature(10_000, cards::GAEA_S_LIEGE, PlayerId::One);
    let liege_id = liege.card.id;
    game.battlefield.push(liege);
    for index in 0..yours {
        game.battlefield
            .push(creature(10_100 + index, cards::FOREST, PlayerId::One));
    }
    for index in 0..theirs {
        game.battlefield
            .push(creature(10_200 + index, cards::FOREST, PlayerId::Two));
    }
    (game, liege_id)
}

#[test]
fn while_not_attacking_it_counts_only_your_forests() {
    let (mut game, liege) = staged(3, 5);
    game.battlefield
        .push(creature(10_300, cards::ISLAND, PlayerId::One));

    assert_eq!(stats(&game, liege), (3, 3));
}

#[test]
fn attacking_switches_the_count_to_the_defending_player() {
    let (mut game, liege) = staged(2, 5);
    let attacker = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == liege)
        .expect("the Liege is present");
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::Two));

    assert_eq!(stats(&game, liege), (5, 5));

    let attacker = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == liege)
        .expect("the Liege is present");
    attacker.attacking = false;
    attacker.attack_defender = None;
    assert_eq!(stats(&game, liege), (2, 2), "leaving combat switches back");
}

#[test]
fn attacking_a_planeswalker_counts_its_controllers_forests() {
    let (mut game, liege) = staged(1, 4);
    let walker = creature(10_300, cards::JACE_MEMORY_ADEPT, PlayerId::Two);
    let walker_id = walker.card.id;
    game.battlefield.push(walker);
    let attacker = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == liege)
        .expect("the Liege is present");
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Planeswalker(walker_id));

    assert_eq!(stats(&game, liege), (4, 4));
}

#[test]
fn the_characteristic_definition_works_in_every_modeled_card_zone() {
    for zone in [
        ZoneKind::Library,
        ZoneKind::Hand,
        ZoneKind::Battlefield,
        ZoneKind::Graveyard,
        ZoneKind::Stack,
        ZoneKind::Exile,
    ] {
        assert_eq!(stats_in_zone(zone), (3, 3), "wrong stats in {zone:?}");
    }
}

#[test]
fn its_activation_makes_only_a_forest_until_this_liege_leaves() {
    let (mut game, liege) = staged(1, 0);
    game.turn = 5;
    game.turns_started = [5, 5];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    permanent(&game, liege);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == liege)
        .expect("the Liege is present")
        .entered_controller_turn = 0;
    let island = creature(10_300, cards::ISLAND, PlayerId::Two);
    let island_id = island.card.id;
    game.battlefield.push(island);

    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility { source, targets, .. }
                    if *source == liege
                        && targets.iter().flat_map(TargetSelection::targets).any(
                            |target| *target == Target::Permanent(island_id)
                        )
            )
        })
        .expect("the untapped Liege can target the Island");
    game.apply(PlayerId::One, activation)
        .expect("the activation is legal");
    drain_pending(&mut game);

    assert_eq!(
        permanent(&game, island_id)
            .resolved_continuous_effects
            .len(),
        1,
        "resolving the ability leaves one land-type effect on its target",
    );
    assert_eq!(
        permanent(&game, island_id).resolved_continuous_effects[0]
            .source
            .object,
        liege,
        "the continuous effect is bound to the Liege permanent, not its stack ability",
    );
    assert!(
        game.resolved_continuous_effect_is_active(
            &permanent(&game, island_id).resolved_continuous_effects[0]
        ),
        "the effect remains active while this Liege remains",
    );
    let subtypes = game.effective_subtypes(permanent(&game, island_id));
    assert!(subtypes.contains(&"Forest"));
    assert!(!subtypes.contains(&"Island"), "the basic type is replaced");

    game.move_target_to_zone(
        Target::Permanent(liege),
        ZoneKind::Graveyard,
        ZoneMoveCause::Effect {
            controller: PlayerId::One,
        },
        None,
        ZonePlacement::Top,
    );
    drain_pending(&mut game);

    let subtypes = game.effective_subtypes(permanent(&game, island_id));
    assert!(subtypes.contains(&"Island"), "the printed type returns");
    assert!(!subtypes.contains(&"Forest"));
}

#[test]
fn gaeas_liege_reports_complete_declarative_coverage() {
    let catalog = poc::catalog().expect("catalog builds");
    let card = catalog
        .get(cards::GAEA_S_LIEGE)
        .expect("Gaea's Liege is cataloged");
    assert_eq!(
        card.rules.implementation_status(),
        ImplementationStatus::Complete,
    );
    assert!(
        card.rules
            .ability_clauses()
            .iter()
            .all(|ability| ability.declarative_effect().is_some()),
    );
}
