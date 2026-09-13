//! Plotted status belongs to an exile incarnation, independently of card abilities.
use super::*;

fn setup(prepared: bool) -> Game {
    let mut game = ready_game();
    game.set_prepared_engine_enabled(prepared);
    game.battlefield.clear();
    for state in &mut game.players {
        state.hand.clear();
        state.graveyard.clear();
        state.exile.clear();
        state.mana_pool = ManaPool::default();
    }
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.turns_started = [3, 3];
    game
}

fn casts(game: &Game, player: PlayerId, id: GameObjectId) -> Vec<Action> {
    game.legal_actions(player)
        .into_iter()
        .filter(|action| matches!(action, Action::CastSpell { card, .. } if *card == id))
        .collect()
}

fn next_main(game: &mut Game, player: PlayerId) {
    game.commit_next_turn(player, Vec::new());
    game.step = Step::PrecombatMain;
    game.priority = player;
}

fn interrupt(game: &mut Game, definition: CardDefinitionId, copy: bool) -> GameObjectId {
    let mut target = spell(280_000, definition, PlayerId::Two, 4);
    target.card.owner = PlayerId::One;
    target.is_copy = copy;
    let original = target.id;
    game.stack.push(target);
    let aven = game
        .build_zone(PlayerId::One, &[cards::AVEN_INTERRUPTER])
        .unwrap()
        .remove(0);
    let id = aven.id;
    game.players[0].hand.push(aven);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 3);
    let action = casts(game, PlayerId::One, id)
        .into_iter()
        .next()
        .expect("Aven has flash");
    game.apply(PlayerId::One, action).unwrap();
    drain_pending(game);
    assert!(game.stack.is_empty());
    assert!(
        game.battlefield
            .iter()
            .any(|p| p.card.definition == cards::AVEN_INTERRUPTER)
    );
    original
}

#[test]
fn plot_exile_aven_exiles_uncounterable_spells_and_the_owner_gets_a_fresh_cast() {
    for prepared in [false, true] {
        let mut game = setup(prepared);
        let original = interrupt(&mut game, cards::SUPREME_VERDICT, false);
        let exiled = game.players[0].exile[0].id;
        assert_ne!(exiled, original);
        assert_eq!(game.plotted_cards.get(&exiled), Some(&(PlayerId::Two, 3)));
        assert!(casts(&game, PlayerId::One, exiled).is_empty());
        next_main(&mut game, PlayerId::One);
        game.players[0].mana_pool = ManaPool::default();
        assert!(casts(&game, PlayerId::Two, exiled).is_empty());
        let action = casts(&game, PlayerId::One, exiled).remove(0);
        game.apply(PlayerId::One, action).unwrap();
        let recast = game.stack.last().unwrap();
        assert_ne!(recast.id, original);
        assert_ne!(recast.id, exiled);
        assert_eq!(recast.controller, PlayerId::One);
        assert_eq!(recast.signature.as_ref().unwrap().x(), 0);
        assert!(!game.plotted_cards.contains_key(&exiled));
    }
}

#[test]
fn plot_exile_copies_leave_no_castable_card() {
    let mut game = setup(true);
    interrupt(&mut game, cards::LIGHTNING_BOLT, true);
    assert!(game.players.iter().all(|p| p.exile.is_empty()));
    assert!(game.plotted_cards.is_empty());
}

#[test]
fn plot_exile_timing_is_intrinsic_to_the_permission_and_other_permissions_still_work() {
    let mut game = setup(true);
    let original = interrupt(&mut game, cards::LIGHTNING_BOLT, false);
    let exiled = game.players[0].exile[0].id;
    game.players[0].mana_pool = ManaPool::default();
    assert!(
        casts(&game, PlayerId::One, exiled).is_empty(),
        "even an instant must wait"
    );
    next_main(&mut game, PlayerId::One);
    assert!(!casts(&game, PlayerId::One, exiled).is_empty());
    game.put_onto_battlefield(PlayerId::One, cards::LEYLINE_OF_ANTICIPATION)
        .unwrap();
    game.step = Step::End;
    assert!(
        casts(&game, PlayerId::One, exiled).is_empty(),
        "flash cannot widen plot's window"
    );
    game.step = Step::PrecombatMain;
    game.stack
        .push(spell(280_001, cards::GRIZZLY_BEARS, PlayerId::Two, 0));
    assert!(
        casts(&game, PlayerId::One, exiled).is_empty(),
        "plot requires an empty stack"
    );
    game.stack.clear();
    next_main(&mut game, PlayerId::Two);
    game.priority = PlayerId::One;
    assert!(casts(&game, PlayerId::One, exiled).is_empty());
    game.permit_conditional_cast_while_exiled(exiled, PlayerId::One);
    assert!(
        casts(&game, PlayerId::One, exiled).is_empty(),
        "an independent permission owes its own price"
    );
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    assert!(!casts(&game, PlayerId::One, exiled).is_empty());
    game.move_target_to_zone(
        Target::Card(exiled),
        ZoneKind::Hand,
        ZoneMoveCause::Rules,
        None,
        ZonePlacement::Top,
    );
    let returned = game.players[0].hand.last().unwrap().id;
    game.move_target_to_zone(
        Target::Card(returned),
        ZoneKind::Exile,
        ZoneMoveCause::Rules,
        None,
        ZonePlacement::Top,
    );
    next_main(&mut game, PlayerId::One);
    let new_exile = game.players[0].exile.last().unwrap().id;
    assert_ne!(new_exile, original);
    assert!(game.plotted_cards.is_empty());
    assert!(casts(&game, PlayerId::One, new_exile).is_empty());
}

#[test]
fn plot_exile_ordinary_exile_does_not_plot_a_card_with_plot_or_rebound() {
    let mut game = setup(true);
    for definition in [cards::SLICKSHOT_SHOW_OFF, cards::EPHEMERATE] {
        let exiled = game
            .build_zone(PlayerId::One, &[definition])
            .unwrap()
            .remove(0);
        let id = exiled.id;
        game.players[0].exile.push(exiled);
        next_main(&mut game, PlayerId::One);
        assert!(casts(&game, PlayerId::One, id).is_empty());
    }
}

#[test]
fn plot_exile_free_cast_handles_absent_mana_costs_and_zero_x() {
    for definition in [cards::ANCESTRAL_VISION, cards::FIREBALL] {
        let mut game = setup(true);
        let exiled = game
            .build_zone(PlayerId::One, &[definition])
            .unwrap()
            .remove(0);
        let id = exiled.id;
        game.players[0].exile.push(exiled);
        game.make_plotted(id);
        next_main(&mut game, PlayerId::One);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 10);
        let actions = casts(&game, PlayerId::One, id);
        assert!(
            !actions.is_empty(),
            "a free cast can pay an absent mana cost"
        );
        assert!(
            actions.iter().all(
                |action| matches!(action, Action::CastSpell { choices, .. } if choices.x() == 0)
            )
        );
    }
}

#[test]
fn plot_exile_origin_taxes_and_discounts_apply_to_total_cast_cost_only() {
    for prepared in [false, true] {
        let mut game = setup(prepared);
        let aven = game
            .put_onto_battlefield(PlayerId::Two, cards::AVEN_INTERRUPTER)
            .unwrap();
        drain_pending(&mut game);
        for zone in [ZoneKind::Hand, ZoneKind::Graveyard, ZoneKind::Exile] {
            let card = game
                .build_zone(PlayerId::One, &[cards::GRIZZLY_BEARS])
                .unwrap()
                .remove(0);
            let id = card.id;
            match zone {
                ZoneKind::Hand => game.players[0].hand.push(card),
                ZoneKind::Graveyard => game.players[0].graveyard.push(card),
                ZoneKind::Exile => game.players[0].exile.push(card),
                _ => unreachable!(),
            }
            assert_eq!(
                game.spell_cost_increase(PlayerId::One, id, &[]).generic,
                if zone == ZoneKind::Hand { 0 } else { 2 }
            );
        }
        let exiled = game.players[0].exile[0].id;
        game.make_plotted(exiled);
        next_main(&mut game, PlayerId::One);
        assert!(
            casts(&game, PlayerId::One, exiled).is_empty(),
            "free casting still pays Aven's tax"
        );
        game.put_onto_battlefield(PlayerId::One, cards::DOC_AURLOCK_GRIZZLED_GENIUS)
            .unwrap();
        for (zone, id) in [
            (ZoneKind::Hand, game.players[0].hand[0].id),
            (ZoneKind::Graveyard, game.players[0].graveyard[0].id),
            (ZoneKind::Exile, exiled),
        ] {
            assert_eq!(
                game.spell_cost_reduction(cards::GRIZZLY_BEARS, PlayerId::One, id, &[])
                    .generic(),
                if zone == ZoneKind::Hand { 0 } else { 2 }
            );
            assert_eq!(game.spell_cost_increase(PlayerId::Two, id, &[]).generic, 0);
            assert_eq!(
                game.spell_cost_reduction(cards::GRIZZLY_BEARS, PlayerId::Two, id, &[])
                    .generic(),
                0
            );
        }
        assert!(
            !casts(&game, PlayerId::One, exiled).is_empty(),
            "Doc reduces Aven's added cost after the free alternative"
        );
        game.exile_permanent(aven);
        assert!(
            !casts(&game, PlayerId::One, exiled).is_empty(),
            "plot outlives its source"
        );
    }
}

#[test]
fn plot_exile_doc_reduces_only_generic_mana_in_a_complete_plot_payment() {
    let rules = CardRules::new_sorcery(mana_cost!("{9}")).with_ability(abilities::plot(&[
        CostDef::Mana(mana_cost!("{2}{G}")),
        CostDef::PayLife(2),
    ]));
    let (mut game, id) = cost_lists::game_with_cost_rules(&rules);
    game.prepared_engine = PreparedEngine::compile(&game.catalog);
    game.put_onto_battlefield(PlayerId::One, cards::DOC_AURLOCK_GRIZZLED_GENIUS)
        .unwrap();
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .contains(&Action::Plot { card: id }),
        "colored mana is still owed"
    );
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    game.apply(PlayerId::One, Action::Plot { card: id })
        .unwrap();
    drain_pending(&mut game);
    assert_eq!(game.players[0].life, 18);
    assert!(game.players[0].hand.is_empty());
    assert_eq!(game.plotted_cards.len(), 1);
}

#[test]
fn plot_exile_checkpoint_preserves_designation_timing_and_rejects_bad_references() {
    let mut game = setup(true);
    let exiled = game
        .build_zone(PlayerId::One, &[cards::LIGHTNING_BOLT])
        .unwrap()
        .remove(0);
    let id = exiled.id;
    game.players[0].exile.push(exiled);
    game.make_plotted(id);
    for viewer in [PlayerId::One, PlayerId::Two] {
        let (wire, hidden) = checkpoint_fixture(&game, viewer);
        let mut rebuilt = Game::from_observation_checkpoint(
            game.catalog.clone(),
            game.format,
            &wire,
            &hidden,
            55,
        )
        .unwrap();
        assert_eq!(rebuilt.plotted_cards, game.plotted_cards);
        assert!(casts(&rebuilt, PlayerId::One, id).is_empty());
        next_main(&mut rebuilt, PlayerId::One);
        assert!(!casts(&rebuilt, PlayerId::One, id).is_empty());
        let mut bad = wire.clone();
        bad["checkpoint"]["plottedCards"][0][0] = serde_json::json!(999_999);
        assert!(
            Game::from_observation_checkpoint(game.catalog.clone(), game.format, &bad, &hidden, 55)
                .is_err()
        );
    }
}

#[test]
fn plot_exile_doc_discounts_any_exile_but_only_the_casters_graveyard() {
    let mut game = setup(true);
    game.put_onto_battlefield(PlayerId::One, cards::DOC_AURLOCK_GRIZZLED_GENIUS)
        .unwrap();
    let foreign = game
        .build_zone(PlayerId::Two, &[cards::GRIZZLY_BEARS])
        .unwrap()
        .remove(0);
    let id = foreign.id;
    game.players[1].graveyard.push(foreign);
    assert_eq!(
        game.spell_cost_reduction(cards::GRIZZLY_BEARS, PlayerId::One, id, &[])
            .generic(),
        0
    );
    let foreign = game.players[1].graveyard.remove(0);
    game.players[1].exile.push(foreign);
    assert_eq!(
        game.spell_cost_reduction(cards::GRIZZLY_BEARS, PlayerId::One, id, &[])
            .generic(),
        2
    );
}

#[test]
fn plot_exile_does_not_grant_a_land_play_or_hide_an_independent_land_permission() {
    let mut game = setup(true);
    let land = game
        .build_zone(PlayerId::One, &[cards::FOREST])
        .unwrap()
        .remove(0);
    let id = land.id;
    game.players[0].exile.push(land);
    game.make_plotted(id);
    next_main(&mut game, PlayerId::One);
    assert!(
        game.legal_actions(PlayerId::One)
            .iter()
            .all(|action| !matches!(action, Action::PlayLand { card, .. } if *card == id))
    );
    game.permit_conditional_cast_while_exiled(id, PlayerId::One);
    assert!(
        game.legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::PlayLand { card, .. } if *card == id))
    );
}
