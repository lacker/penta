//! Rooftop Storm's battlefield-granted alternative cost and its ordinary
//! casting-cost interactions.

use super::*;

fn casts_of(game: &Game, player: PlayerId, spell: GameObjectId) -> Vec<Action> {
    game.legal_actions(player)
        .into_iter()
        .filter(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell))
        .collect()
}

fn alternative_cast(game: &Game, spell: GameObjectId) -> Action {
    casts_of(game, PlayerId::One, spell)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell { choices, .. }
                    if choices.costs().alternative().is_some()
            )
        })
        .expect("Rooftop Storm offers its alternative cost")
}

#[test]
fn rooftop_storm_offers_one_free_cast_only_for_its_controllers_zombie_creatures() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(120_000, cards::ROOFTOP_STORM, PlayerId::One));
    // A second copy offers the same cost, not a duplicate game action.
    game.battlefield
        .push(creature(120_001, cards::ROOFTOP_STORM, PlayerId::One));
    let zombie = card(120_002, cards::ARMORED_SKAAB, PlayerId::One);
    let zombie_id = zombie.id;
    let human = card(120_003, cards::SELHOFF_OCCULTIST, PlayerId::One);
    let human_id = human.id;
    game.players[PlayerId::One.index()]
        .hand
        .extend([zombie, human]);

    let zombie_casts = casts_of(&game, PlayerId::One, zombie_id);
    assert_eq!(zombie_casts.len(), 1, "equivalent grants are one choice");
    assert!(matches!(
        &zombie_casts[0],
        Action::CastSpell { choices, .. } if choices.costs().alternative().is_some()
    ));
    assert!(
        casts_of(&game, PlayerId::One, human_id).is_empty(),
        "a non-Zombie creature still needs its printed mana"
    );

    game.active_player = PlayerId::Two;
    game.priority = PlayerId::Two;
    let opposing_zombie = card(120_004, cards::ARMORED_SKAAB, PlayerId::Two);
    let opposing_zombie_id = opposing_zombie.id;
    game.players[PlayerId::Two.index()]
        .hand
        .push(opposing_zombie);
    assert!(
        casts_of(&game, PlayerId::Two, opposing_zombie_id).is_empty(),
        "Rooftop Storm does not change an opponent's costs"
    );
}

#[test]
fn rooftop_storm_replaces_only_the_mana_cost_and_keeps_mandatory_additional_costs() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(120_010, cards::ROOFTOP_STORM, PlayerId::One));
    let mauler = card(120_011, cards::MAKESHIFT_MAULER, PlayerId::One);
    let mauler_id = mauler.id;
    game.players[PlayerId::One.index()].hand.push(mauler);

    assert!(
        casts_of(&game, PlayerId::One, mauler_id).is_empty(),
        "the free alternative cannot waive Makeshift Mauler's exile cost"
    );

    let fodder = card(120_012, cards::MONSS_GOBLIN_RAIDERS, PlayerId::One);
    let fodder_id = fodder.id;
    game.players[PlayerId::One.index()].graveyard.push(fodder);
    let action = alternative_cast(&game, mauler_id);
    let Action::CastSpell { sacrifices, .. } = &action else {
        unreachable!("the helper returns a spell cast")
    };
    assert_eq!(sacrifices, &[fodder_id]);

    game.apply(PlayerId::One, action)
        .expect("the alternative cast is payable without mana");
    assert!(
        game.players[PlayerId::One.index()]
            .exile
            .iter()
            .any(|card| card.definition == cards::MONSS_GOBLIN_RAIDERS),
        "the mandatory additional cost was paid"
    );
    assert_eq!(
        game.stack.last().unwrap().card.definition,
        ObjectKind::Card(cards::MAKESHIFT_MAULER)
    );
}

#[test]
fn cost_increases_are_added_after_rooftop_storms_alternative_cost() {
    let mut game = ready_game();
    game.battlefield.extend([
        creature(120_020, cards::ROOFTOP_STORM, PlayerId::One),
        creature(120_021, cards::DERELOR, PlayerId::One),
    ]);
    let zombie = card(120_022, cards::WALKING_CORPSE, PlayerId::One);
    let zombie_id = zombie.id;
    game.players[PlayerId::One.index()].hand.push(zombie);

    assert!(
        casts_of(&game, PlayerId::One, zombie_id).is_empty(),
        "Derelor's colored increase is still owed"
    );
    game.players[PlayerId::One.index()].mana_pool.black = 1;
    let action = alternative_cast(&game, zombie_id);
    game.apply(PlayerId::One, action)
        .expect("one black mana pays the increase on the zero alternative");
    assert_eq!(game.players[PlayerId::One.index()].mana_pool.black, 0);
    assert_eq!(
        game.stack.last().unwrap().card.definition,
        ObjectKind::Card(cards::WALKING_CORPSE)
    );
}

#[test]
fn a_stale_rooftop_storm_action_is_revalidated_after_the_enchantment_leaves() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(120_030, cards::ROOFTOP_STORM, PlayerId::One));
    let zombie = card(120_031, cards::ARMORED_SKAAB, PlayerId::One);
    let zombie_id = zombie.id;
    game.players[PlayerId::One.index()].hand.push(zombie);
    let action = alternative_cast(&game, zombie_id);

    game.battlefield.clear();
    assert!(
        game.apply(PlayerId::One, action).is_err(),
        "a vanished static source cannot authorize its old action"
    );
}
