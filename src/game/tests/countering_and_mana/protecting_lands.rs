use super::*;

fn rebuild(game: &Game) -> Game {
    let (wire, hidden) = checkpoint_fixture(game, PlayerId::One);
    Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 42)
        .expect("the protection and its source reconstruct")
}

fn village(game: &mut Game) -> GameObjectId {
    game.put_onto_battlefield(PlayerId::One, cards::FOREST)
        .unwrap();
    game.put_onto_battlefield(PlayerId::One, cards::MISTRISE_VILLAGE)
        .unwrap()
}

fn activate_protection(game: &mut Game, source: GameObjectId) {
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source: id, .. } if *id == source),
        )
        .expect("Village offers its stack-using ability");
    game.apply(PlayerId::One, action).unwrap();
}

fn cast_bolt(game: &mut Game, player: PlayerId, id: u32) {
    let bolt = card(id, cards::LIGHTNING_BOLT, player);
    game.players[player.index()].hand.push(bolt.clone());
    game.add_unrestricted_mana(player, ManaColor::Red, 1);
    game.apply(
        player,
        cast_action(bolt.id, vec![Target::Player(player.opponent())], vec![], 0),
    )
    .unwrap();
}

#[test]
fn village_entry_checks_your_existing_land_types() {
    for (land, controller, tapped) in [
        (cards::ISLAND, PlayerId::One, true),
        (cards::MOUNTAIN, PlayerId::One, false),
        (cards::FOREST, PlayerId::One, false),
        (cards::TAIGA, PlayerId::One, false),
        (cards::FOREST, PlayerId::Two, true),
    ] {
        let mut game = ready_game();
        game.battlefield.clear();
        game.put_onto_battlefield(controller, land).unwrap();
        let id = game
            .put_onto_battlefield(PlayerId::One, cards::MISTRISE_VILLAGE)
            .unwrap();
        assert_eq!(
            game.battlefield
                .iter()
                .find(|p| p.card.id == id)
                .unwrap()
                .tapped,
            tapped
        );
        if !tapped {
            let action = game.legal_actions(PlayerId::One).into_iter().find(|action| {
                matches!(action, Action::ActivateManaAbility { source, .. } if *source == id)
            }).unwrap();
            game.apply(PlayerId::One, action).unwrap();
            assert_eq!(game.players[0].mana_pool.blue, 1);
            assert!(game.resolved_player_rules.is_empty());
        }
    }
}

#[test]
fn village_waits_for_resolution_then_protects_exactly_your_next_cast() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        let source = village(&mut game);
        activate_protection(&mut game, source);
        let mut game = rebuild(&game);
        game.set_prepared_engine_enabled(prepared);
        cast_bolt(&mut game, PlayerId::One, 130_001);
        assert!(
            game.can_be_countered(game.stack.last().unwrap()),
            "a response precedes the protection"
        );
        pass_priority_pair(&mut game);
        pass_priority_pair(&mut game);
        assert_eq!(game.resolved_player_rules.len(), 1);
        game.apply(PlayerId::One, Action::PassPriority).unwrap();
        cast_bolt(&mut game, PlayerId::Two, 130_002);
        assert!(game.can_be_countered(game.stack.last().unwrap()));
        assert_eq!(game.resolved_player_rules.len(), 1);
        pass_priority_pair(&mut game);
        game.return_permanent_to_hand(source);
        let mut game = rebuild(&game);
        game.set_prepared_engine_enabled(prepared);
        // Repeated legal-action enumeration is read-only.
        assert_eq!(
            game.legal_actions(PlayerId::One),
            game.legal_actions(PlayerId::One),
        );
        cast_bolt(&mut game, PlayerId::One, 130_003);
        assert!(game.resolved_player_rules.is_empty());
        assert!(!game.can_be_countered(game.stack.last().unwrap()));
        let original = game.stack.last().unwrap().clone();
        let targets = original.signature.as_ref().unwrap().targets().to_vec();
        game.push_copy(original, PlayerId::One, targets);
        assert!(
            game.can_be_countered(game.stack.last().unwrap()),
            "copies do not inherit protection"
        );
        game.stack.pop();
        let mut game = rebuild(&game);
        assert!(!game.can_be_countered(game.stack.last().unwrap()));
        let counter = card(130_004, cards::COUNTERSPELL, PlayerId::Two);
        game.players[1].hand.push(counter.clone());
        game.add_unrestricted_mana(PlayerId::Two, ManaColor::Blue, 2);
        acceptance_attempt_counterspell(&mut game, counter.id);
        assert_eq!(
            game.stack.len(),
            1,
            "Counterspell can target the spell but fails to counter it"
        );
        cast_bolt(&mut game, PlayerId::One, 130_005);
        assert!(game.can_be_countered(game.stack.last().unwrap()));
    }
}

#[test]
fn village_multiple_activations_are_consumed_by_one_spell_and_unused_grants_expire() {
    let mut game = ready_game();
    let first = village(&mut game);
    let second = game
        .put_onto_battlefield(PlayerId::One, cards::MISTRISE_VILLAGE)
        .unwrap();
    for source in [first, second] {
        activate_protection(&mut game, source);
        pass_priority_pair(&mut game);
    }
    assert_eq!(game.resolved_player_rules.len(), 2);
    // A copied spell is not a cast and consumes neither grant.
    let copied = spell(130_010, cards::LIGHTNING_BOLT, PlayerId::One, 0);
    game.push_copy(copied, PlayerId::One, Vec::new());
    assert!(game.can_be_countered(game.stack.last().unwrap()));
    assert_eq!(game.resolved_player_rules.len(), 2);
    game.stack.clear();
    let zero = card(130_011, cards::BLACK_LOTUS, PlayerId::One);
    game.players[0].hand.push(zero.clone());
    game.apply(PlayerId::One, cast_action(zero.id, vec![], vec![], 0))
        .unwrap();
    assert!(
        !game.can_be_countered(game.stack.last().unwrap()),
        "permanent spells also qualify"
    );
    assert!(game.resolved_player_rules.is_empty());
    game.finish_cleanup();
    assert!(
        !game.can_be_countered(game.stack.last().unwrap()),
        "once granted, protection lasts for the spell's stack lifetime"
    );
    game.stack.clear();
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == first)
        .unwrap()
        .tapped = false;
    activate_protection(&mut game, first);
    pass_priority_pair(&mut game);
    game.finish_cleanup();
    assert!(game.resolved_player_rules.is_empty());
    cast_bolt(&mut game, PlayerId::One, 130_012);
    assert!(game.can_be_countered(game.stack.last().unwrap()));
}

#[test]
fn boseiju_enters_tapped_and_requires_two_payable_life() {
    for life in [1, 2, 3] {
        let mut game = ready_game();
        let id = game
            .put_onto_battlefield(PlayerId::One, cards::BOSEIJU_WHO_SHELTERS_ALL_273)
            .unwrap();
        assert!(
            game.battlefield
                .iter()
                .find(|p| p.card.id == id)
                .unwrap()
                .tapped
        );
        assert!(!game.legal_actions(PlayerId::One).iter().any(|action| {
            matches!(action, Action::ActivateManaAbility { source, .. } if *source == id)
        }));
        game.battlefield
            .iter_mut()
            .find(|p| p.card.id == id)
            .unwrap()
            .tapped = false;
        game.players[0].life = life;
        let action = game.legal_actions(PlayerId::One).into_iter().find(
            |action| matches!(action, Action::ActivateManaAbility { source, .. } if *source == id),
        );
        assert_eq!(action.is_some(), life >= 2);
        if let Some(action) = action {
            game.apply(PlayerId::One, action).unwrap();
            assert_eq!(game.players[0].life, life - 2);
        }
    }
}

#[test]
fn boseiju_floating_mana_and_paid_instant_survive_checkpoints_but_copies_are_counterable() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        let source = game
            .put_onto_battlefield(PlayerId::One, cards::BOSEIJU_WHO_SHELTERS_ALL_273)
            .unwrap();
        game.battlefield
            .iter_mut()
            .find(|p| p.card.id == source)
            .unwrap()
            .tapped = false;
        let action = game.legal_actions(PlayerId::One).into_iter().find(|action| {
            matches!(action, Action::ActivateManaAbility { source: id, .. } if *id == source)
        }).unwrap();
        game.apply(PlayerId::One, action).unwrap();
        game.return_permanent_to_hand(source);
        let mut game = rebuild(&game);
        game.set_prepared_engine_enabled(prepared);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
        let blast = card(130_020, cards::PSIONIC_BLAST, PlayerId::One);
        game.players[0].hand.push(blast.clone());
        game.apply(
            PlayerId::One,
            cast_action(blast.id, vec![Target::Player(PlayerId::Two)], vec![], 0),
        )
        .unwrap();
        let mut game = rebuild(&game);
        assert!(!game.can_be_countered(game.stack.last().unwrap()));
        let original = game.stack.last().unwrap().clone();
        let targets = original.signature.as_ref().unwrap().targets().to_vec();
        game.push_copy(original, PlayerId::One, targets);
        assert!(game.can_be_countered(game.stack.last().unwrap()));
        game.stack.pop();
        let counter = card(130_021, cards::COUNTERSPELL, PlayerId::Two);
        game.players[1].hand.push(counter.clone());
        game.add_unrestricted_mana(PlayerId::Two, ManaColor::Blue, 2);
        acceptance_attempt_counterspell(&mut game, counter.id);
        assert_eq!(game.stack.len(), 1);
        pass_priority_pair(&mut game);
        assert_eq!(game.players[1].life, 16);
        assert_eq!(game.players[0].life, 16);
    }
}

#[test]
fn next_spell_rule_filters_cast_characteristics_and_ignores_failed_casts() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        let source = village(&mut game);
        activate_protection(&mut game, source);
        pass_priority_pair(&mut game);
        // Exercise the shared predicate independently of Village's any-spell clause.
        game.resolved_player_rules[0].rule = crate::card::PlayerRuleDef::ApplyToNextSpell {
            object: ObjectPredicateDef::HasType(CardType::Instant),
            effect: &AppliedEffectDef::Rule(AppliedRuleDef::CannotBeCountered),
        };
        let bolt = card(130_030, cards::LIGHTNING_BOLT, PlayerId::One);
        game.players[0].hand.push(bolt.clone());
        assert!(
            game.apply(
                PlayerId::One,
                cast_action(bolt.id, vec![Target::Player(PlayerId::Two)], vec![], 0)
            )
            .is_err()
        );
        assert_eq!(game.resolved_player_rules.len(), 1);
        let lotus = card(130_031, cards::BLACK_LOTUS, PlayerId::One);
        game.players[0].hand.push(lotus.clone());
        game.apply(PlayerId::One, cast_action(lotus.id, vec![], vec![], 0))
            .unwrap();
        assert!(game.can_be_countered(game.stack.last().unwrap()));
        assert_eq!(game.resolved_player_rules.len(), 1);
        cast_bolt(&mut game, PlayerId::One, 130_032);
        assert!(!game.can_be_countered(game.stack.last().unwrap()));
        assert!(game.resolved_player_rules.is_empty());
    }
}

#[test]
fn village_protection_does_not_prevent_a_spell_with_illegal_targets_from_fizzling() {
    let mut game = ready_game();
    let source = village(&mut game);
    activate_protection(&mut game, source);
    pass_priority_pair(&mut game);
    let bear = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .unwrap();
    let bolt = card(130_040, cards::LIGHTNING_BOLT, PlayerId::One);
    game.players[0].hand.push(bolt.clone());
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    game.apply(
        PlayerId::One,
        cast_action(bolt.id, vec![Target::Permanent(bear)], vec![], 0),
    )
    .unwrap();
    assert!(!game.can_be_countered(game.stack.last().unwrap()));
    game.return_permanent_to_hand(bear);
    assert!(game.spell_fizzles(game.stack.last().unwrap()));
    pass_priority_pair(&mut game);
    assert!(game.stack.is_empty());
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::LIGHTNING_BOLT)
    );
}
