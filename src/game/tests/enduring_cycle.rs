//! Enduring returns establish a noncopiable type effect before anything sees entry.
use super::*;

fn put(game: &mut Game, definition: CardDefinitionId) -> GameObjectId {
    game.put_onto_battlefield(PlayerId::One, definition)
        .unwrap()
}

fn body(game: &Game, definition: CardDefinitionId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|p| p.card.definition == definition)
        .unwrap()
}

fn settle(game: &mut Game) {
    game.finish_rules_procedure();
    drain_pending(game);
    assert!(game.pending_decisions.is_empty());
    assert!(game.stack.is_empty());
    assert!(game.pending_triggers.is_empty());
}

fn restore(game: &Game) -> Game {
    let (wire, hidden) = checkpoint_fixture(game, PlayerId::One);
    assert_eq!(wire["checkpoint"]["hasDeferredState"], false);
    Game::from_observation_checkpoint(game.catalog.clone(), game.format, &wire, &hidden, 42)
        .expect("entry type effects and their source reconstruct")
}

#[test]
fn return_is_an_enchantment_before_replacements_and_triggers_in_both_engines() {
    let run = |prepared| {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        game.put_onto_battlefield(PlayerId::Two, cards::BLIND_OBEDIENCE)
            .unwrap();
        put(&mut game, cards::SOUL_WARDEN);
        let enduring = put(&mut game, cards::ENDURING_CURIOSITY);
        settle(&mut game);
        let life = game.players[0].life;
        game.destroy_permanent(enduring);
        game.finish_rules_procedure();
        assert_eq!(game.stack.len(), 1, "the death trigger is respondable");
        game = restore(&game);
        game.set_prepared_engine_enabled(prepared);
        settle(&mut game);
        let returned = body(&game, cards::ENDURING_CURIOSITY);
        assert_ne!(returned.card.id, enduring);
        assert_eq!(
            game.permanent_types(returned),
            Some(CardTypeSet::single(CardType::Enchantment))
        );
        assert!(game.effective_subtypes(returned).is_empty());
        assert_eq!(game.power(returned), None);
        assert!(
            !returned.tapped,
            "Blind Obedience never sees a creature entry"
        );
        assert_eq!(
            game.players[0].life, life,
            "Soul Warden never sees a creature entry"
        );
        game = restore(&game);
        let returned = body(&game, cards::ENDURING_CURIOSITY);
        assert_eq!(
            game.permanent_types(returned),
            Some(CardTypeSet::single(CardType::Enchantment))
        );
        game
    };
    let reference = run(false);
    let prepared = run(true);
    assert_eq!(reference.players, prepared.players);
    assert_eq!(reference.events, prepared.events);
}

#[test]
fn return_uses_owner_and_flicker_discards_the_type_effect() {
    let mut game = ready_game();
    let enduring = put(&mut game, cards::ENDURING_VITALITY);
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == enduring)
        .unwrap()
        .controller = PlayerId::Two;
    game.destroy_permanent(enduring);
    settle(&mut game);
    let returned = body(&game, cards::ENDURING_VITALITY);
    assert_eq!(returned.controller, PlayerId::One);
    let id = returned.card.id;
    game.move_target_to_zone(
        Target::Permanent(id),
        ZoneKind::Exile,
        ZoneMoveCause::Effect {
            controller: PlayerId::One,
        },
        None,
        ZonePlacement::Top,
    );
    let exiled = game.players[0].exile.last().unwrap().id;
    game.move_target_to_zone(
        Target::Card(exiled),
        ZoneKind::Battlefield,
        ZoneMoveCause::Effect {
            controller: PlayerId::One,
        },
        None,
        ZonePlacement::Top,
    );
    settle(&mut game);
    let returned = body(&game, cards::ENDURING_VITALITY);
    assert!(game.permanent_types(returned).unwrap().is_creature());
    assert_eq!(game.power(returned), Some(3));
}

#[test]
fn an_enchantment_stays_dead_but_a_later_animation_can_return_again() {
    for animate in [false, true] {
        let mut game = ready_game();
        let enduring = put(&mut game, cards::ENDURING_COURAGE);
        game.destroy_permanent(enduring);
        settle(&mut game);
        let id = body(&game, cards::ENDURING_COURAGE).card.id;
        if animate {
            attach_constant_resolved_characteristics(
                &mut game,
                id,
                &[AppliedEffectDef::add_card_types(CardTypeSet::single(
                    CardType::Creature,
                ))],
                ContinuousEffectExpiration::Never,
            );
            assert!(
                game.effective_subtypes(body(&game, cards::ENDURING_COURAGE))
                    .is_empty(),
                "a later animation does not recover creature subtypes removed by the return"
            );
        }
        game.destroy_permanent(id);
        settle(&mut game);
        assert_eq!(game.battlefield.len(), usize::from(animate));
        if animate {
            assert!(
                !game
                    .permanent_types(body(&game, cards::ENDURING_COURAGE))
                    .unwrap()
                    .is_creature()
            );
        }
    }
}

#[test]
fn a_graveyard_zone_change_breaks_the_return_reference() {
    let mut game = ready_game();
    let enduring = put(&mut game, cards::ENDURING_CURIOSITY);
    game.destroy_permanent(enduring);
    game.finish_rules_procedure();
    let dead = game.players[0].graveyard.last().unwrap().id;
    game.move_target_to_zone(
        Target::Card(dead),
        ZoneKind::Exile,
        ZoneMoveCause::Effect {
            controller: PlayerId::One,
        },
        None,
        ZonePlacement::Top,
    );
    let exiled = game.players[0].exile.last().unwrap().id;
    game.move_target_to_zone(
        Target::Card(exiled),
        ZoneKind::Graveyard,
        ZoneMoveCause::Effect {
            controller: PlayerId::One,
        },
        None,
        ZonePlacement::Top,
    );
    settle(&mut game);
    assert!(game.battlefield.is_empty());
    assert_eq!(game.players[0].graveyard.len(), 1);
}

#[test]
fn copies_return_only_physical_cards_and_the_type_change_is_not_copiable() {
    let mut game = ready_game();
    let mut copier = creature(90_000, cards::SOL_RING, PlayerId::One);
    copier.copy_effect = Some(copied_characteristics(cards::ENDURING_CURIOSITY));
    game.battlefield.push(copier);
    game.create_token_copy(
        PlayerId::One,
        copied_characteristics(cards::ENDURING_CURIOSITY),
        None,
        CardPartId::PRIMARY,
    );
    settle(&mut game);
    let ids = game
        .battlefield
        .iter()
        .map(|p| p.card.id)
        .collect::<Vec<_>>();
    game.destroy_permanents(&ids, false);
    settle(&mut game);
    assert_eq!(game.battlefield.len(), 1);
    let ring = body(&game, cards::SOL_RING);
    assert_eq!(
        game.permanent_types(ring),
        Some(CardTypeSet::single(CardType::Enchantment))
    );
    game = restore(&game);
    assert_eq!(
        game.permanent_types(body(&game, cards::SOL_RING)),
        Some(CardTypeSet::single(CardType::Enchantment))
    );

    let enduring = put(&mut game, cards::ENDURING_CURIOSITY);
    game.destroy_permanent(enduring);
    settle(&mut game);
    let returned = body(&game, cards::ENDURING_CURIOSITY);
    let values = Game::copiable_characteristics(returned);
    game.create_token_copy(PlayerId::One, values, None, CardPartId::PRIMARY);
    settle(&mut game);
    let token = game
        .battlefield
        .iter()
        .find(|p| p.card.definition.is_token())
        .unwrap();
    assert!(
        game.permanent_types(token).unwrap().is_creature(),
        "copying the returned enchantment restores printed types"
    );
}

#[test]
fn innocence_checks_power_at_entry_and_shares_one_allowance_for_the_turn() {
    let mut game = ready_game();
    put(&mut game, cards::ENDURING_INNOCENCE);
    assert!(game.pending_triggers.is_empty(), "not its own entry");
    put(&mut game, cards::SERRA_ANGEL);
    game.put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .unwrap();
    assert!(
        game.pending_triggers.is_empty(),
        "only your small creatures"
    );
    let hand = game.players[0].hand.len();
    let mut bear = None;
    game.entering_together(|game| {
        bear = Some(put(game, cards::GRIZZLY_BEARS));
        put(game, cards::GRIZZLY_BEARS);
    });
    assert_eq!(game.pending_triggers.len(), 1);
    attach_constant_resolved_characteristics(
        &mut game,
        bear.unwrap(),
        &[AppliedEffectDef::modify_power_toughness(
            ValueDef::Constant(5),
            ValueDef::Constant(5),
        )],
        ContinuousEffectExpiration::EndOfTurn,
    );
    settle(&mut game);
    assert_eq!(
        game.players[0].hand.len(),
        hand + 1,
        "power is not rechecked on resolution"
    );
    put(&mut game, cards::GRIZZLY_BEARS);
    assert!(game.pending_triggers.is_empty());

    // A new incarnation has its own allowance, even in the same turn.
    let id = body(&game, cards::ENDURING_INNOCENCE).card.id;
    game.destroy_permanent(id);
    settle(&mut game);
    put(&mut game, cards::GRIZZLY_BEARS);
    settle(&mut game);
    assert_eq!(game.players[0].hand.len(), hand + 2);
}

#[test]
fn innocence_includes_entry_counters_in_its_power_test() {
    let mut game = ready_game();
    put(&mut game, cards::ENDURING_INNOCENCE);
    // Arrival counters already contribute to power when the trigger checks it.
    let walker = card(80_000, cards::HANGARBACK_WALKER_229, PlayerId::One);
    game.players[0].graveyard.push(walker.clone());
    game.move_target_to_zone(
        Target::Card(walker.id),
        ZoneKind::Battlefield,
        ZoneMoveCause::Effect {
            controller: PlayerId::One,
        },
        Some(
            BattlefieldArrival::under(PlayerId::One)
                .with_counters(Some((CounterKind::PlusOnePlusOne, 3))),
        ),
        ZonePlacement::Top,
    );
    assert!(
        game.pending_triggers.is_empty(),
        "a 3/3 does not qualify despite printed 0 power"
    );
}

#[test]
fn courage_and_vitality_keep_their_abilities_as_enchantments() {
    let mut game = ready_game();
    let courage = put(&mut game, cards::ENDURING_COURAGE);
    let vitality = put(&mut game, cards::ENDURING_VITALITY);
    settle(&mut game);
    game.destroy_permanents(&[courage, vitality], false);
    settle(&mut game);
    let bear = put(&mut game, cards::GRIZZLY_BEARS);
    settle(&mut game);
    let creature = game.battlefield.iter().find(|p| p.card.id == bear).unwrap();
    assert_eq!(game.power(creature), Some(4));
    assert!(game.permanent_has_executable_keyword(creature, KeywordAbility::Haste));
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateManaAbility { source, .. } if *source == bear))
        .expect("Courage's haste lets the new creature tap for Vitality's mana");
    game.apply(PlayerId::One, action).unwrap();
    settle(&mut game);
    assert_eq!(game.players[0].mana_pool.total(), 1);
    game.cleanup();
    let creature = game.battlefield.iter().find(|p| p.card.id == bear).unwrap();
    assert_eq!(game.power(creature), Some(2));
    assert!(!game.permanent_has_executable_keyword(creature, KeywordAbility::Haste));
}

#[test]
fn curiosity_counts_each_combat_source_and_tenacity_targets_the_opponent() {
    let mut game = ready_game();
    let curiosity = put(&mut game, cards::ENDURING_CURIOSITY);
    let tenacity = put(&mut game, cards::ENDURING_TENACITY);
    let bear = put(&mut game, cards::GRIZZLY_BEARS);
    let hand = game.players[0].hand.len();
    game.deal_damage_simultaneously(vec![
        DamageAssignment {
            source: Some(curiosity),
            target: Some(Target::Player(PlayerId::Two)),
            amount: 4,
            combat: true,
        },
        DamageAssignment {
            source: Some(bear),
            target: Some(Target::Player(PlayerId::Two)),
            amount: 2,
            combat: true,
        },
    ]);
    settle(&mut game);
    assert_eq!(game.players[0].hand.len(), hand + 2);
    game.destroy_permanents(&[curiosity, tenacity], false);
    settle(&mut game);
    game.deal_damage_simultaneously(vec![DamageAssignment {
        source: Some(bear),
        target: Some(Target::Player(PlayerId::Two)),
        amount: 1,
        combat: false,
    }]);
    settle(&mut game);
    assert_eq!(
        game.players[0].hand.len(),
        hand + 2,
        "noncombat damage does not draw"
    );
    let life = game.players[1].life;
    game.gain_life(PlayerId::One, 3);
    game.finish_rules_procedure();
    assert!(
        !game.pending_decisions.is_empty(),
        "Tenacity chooses an opponent target"
    );
    game = restore(&game);
    settle(&mut game);
    assert_eq!(game.players[1].life, life - 3);
}

#[test]
fn returning_a_copy_preserves_the_type_effect_through_an_entry_choice_checkpoint() {
    let mut game = ready_game();
    let mut copier = creature(90_000, cards::CAVERN_OF_SOULS, PlayerId::One);
    copier.copy_effect = Some(copied_characteristics(cards::ENDURING_CURIOSITY));
    game.battlefield.push(copier);
    game.destroy_permanent(GameObjectId(90_000));
    game.finish_rules_procedure();
    pass_until_decision(&mut game);
    assert!(
        !game.pending_decisions.is_empty(),
        "Cavern still chooses a creature type"
    );
    game = restore(&game);
    settle(&mut game);
    let returned = body(&game, cards::CAVERN_OF_SOULS);
    assert_eq!(
        game.permanent_types(returned),
        Some(CardTypeSet::single(CardType::Enchantment))
    );
    assert!(returned.chosen_creature_type.is_some());
}

#[test]
fn returning_copies_drop_subtypes_of_card_types_they_no_longer_have() {
    for definition in [
        cards::PLAINS,
        cards::BONESPLITTER,
        cards::JACE_THE_MIND_SCULPTOR,
    ] {
        let mut game = ready_game();
        let mut copier = creature(90_000, definition, PlayerId::One);
        copier.copy_effect = Some(copied_characteristics(cards::ENDURING_CURIOSITY));
        game.battlefield.push(copier);
        game.destroy_permanent(GameObjectId(90_000));
        settle(&mut game);
        let returned = body(&game, definition);
        assert_eq!(
            game.permanent_types(returned),
            Some(CardTypeSet::single(CardType::Enchantment))
        );
        assert!(game.effective_subtypes(returned).is_empty());
        assert_eq!(
            returned.counters(CounterKind::Loyalty),
            0,
            "it never enters as a planeswalker"
        );
    }
}
