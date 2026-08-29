//! Focused integration checks for New Phyrexia declarations whose printed
//! costs or controller relations combine multiple shared primitives.

use super::*;

#[test]
fn puresteel_paladin_grants_free_equip_only_during_metalcraft() {
    let mut game = ready_game();
    game.battlefield.clear();

    let paladin = creature(10_000, cards::PURESTEEL_PALADIN, PlayerId::One);
    let paladin_id = paladin.card.id;
    game.battlefield.push(paladin);
    let equipment = creature(10_001, cards::SKYBLINDER_STAFF, PlayerId::One);
    let equipment_id = equipment.card.id;
    game.battlefield.push(equipment);
    let first_relic = creature(10_002, cards::DARKSTEEL_RELIC, PlayerId::One);
    let first_relic_id = first_relic.card.id;
    game.battlefield.push(first_relic);
    game.battlefield
        .push(creature(10_003, cards::DARKSTEEL_RELIC, PlayerId::One));
    let host = creature(10_004, cards::GRIZZLY_BEARS, PlayerId::One);
    let host_id = host.card.id;
    game.battlefield.push(host);

    let free_equip = |game: &Game| {
        game.legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| {
                matches!(
                    action,
                    Action::ActivateAbility {
                        source,
                        ability: AbilityOrigin::Granted { source: granter, .. },
                        targets,
                        ..
                    } if *source == equipment_id
                        && *granter == paladin_id
                        && targets
                            .iter()
                            .flat_map(TargetSelection::targets)
                            .any(|target| *target == Target::Permanent(host_id))
                )
            })
    };

    let action = free_equip(&game).expect("three artifacts grant equip {0}");
    game.apply(PlayerId::One, action)
        .expect("free equip activates");
    drain_pending(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == equipment_id)
            .and_then(|permanent| permanent.attached_to),
        Some(host_id),
    );

    game.battlefield
        .retain(|permanent| permanent.card.id != first_relic_id);
    assert!(
        free_equip(&game).is_none(),
        "dropping below three artifacts removes the granted ability",
    );
}

#[test]
fn shrine_of_boundless_growth_reads_its_charge_counters_after_sacrificing_itself() {
    let mut game = ready_game();
    let mut shrine = creature(10_000, cards::SHRINE_OF_BOUNDLESS_GROWTH, PlayerId::One);
    shrine.counters.set(CounterKind::named("charge"), 3);
    let shrine_id = shrine.card.id;
    game.battlefield.push(shrine);

    let activation = Action::ActivateManaAbility {
        source: shrine_id,
        ability: mana_ability_for(&game, shrine_id, ManaColor::Colorless),
        color: ManaColor::Colorless,
        counters_removed: None,
        cost_object: None,
        combination: None,
        triggered_mana: None,
    };
    assert!(game.legal_actions(PlayerId::One).contains(&activation));

    game.apply(PlayerId::One, activation).unwrap();

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != shrine_id),
        "the Shrine is sacrificed as part of the mana ability's cost",
    );
    assert_eq!(
        game.players[PlayerId::One.index()].mana_pool.colorless,
        3,
        "the resolving mana ability uses the sacrificed Shrine's last-known counters",
    );
}

#[test]
fn parasitic_implant_triggers_on_its_controllers_upkeep_and_sacrifices_the_host() {
    let mut game = ready_game();
    let host = creature(10_000, cards::GRIZZLY_BEARS, PlayerId::Two);
    let host_id = host.card.id;
    game.battlefield.push(host);

    let mut implant = creature(10_001, cards::PARASITIC_IMPLANT, PlayerId::One);
    implant.attached_to = Some(host_id);
    game.battlefield.push(implant);
    game.check_state_based_actions();

    game.commit_next_turn(PlayerId::One, Vec::new());
    drain_pending(&mut game);

    assert!(
        game.players[PlayerId::Two.index()]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::GRIZZLY_BEARS),
        "the enchanted creature's controller sacrifices that creature",
    );
    assert!(game.battlefield.iter().any(|permanent| {
        permanent.controller == PlayerId::One
            && is_token_with(
                permanent,
                tokens::artifact_creature(&["Phyrexian", "Myr"], &[], 1, 1),
            )
    }));
}
