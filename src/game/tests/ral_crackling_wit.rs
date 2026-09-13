//! Static stack ability grants, exercised through Ral's complete emblem.
use super::*;

fn settle(game: &mut Game) {
    for _ in 0..16 {
        drain_pending(game);
        if game.stack.is_empty()
            && game.pending_triggers.is_empty()
            && game.pending_decisions.is_empty()
        {
            return;
        }
    }
    panic!("stack did not settle");
}

fn activate(game: &mut Game, ral: GameObjectId, index: usize) {
    let ability = activated_ability_for(game, ral, index);
    game.apply(PlayerId::One, plain_activation(ral, ability))
        .unwrap();
    settle(game);
}

fn emblem(game: &mut Game) {
    let ral = game
        .put_onto_battlefield(PlayerId::One, cards::RAL_CRACKLING_WIT)
        .unwrap();
    game.battlefield
        .iter_mut()
        .find(|p| p.card.id == ral)
        .unwrap()
        .set_counters(CounterKind::Loyalty, 10);
    let hand = game.players[0].hand.len();
    let count = game.emblems.len();
    activate(game, ral, 2);
    assert_eq!(game.players[0].hand.len(), hand + 3);
    assert_eq!(game.emblems.len(), count + 1);
    assert!(!game.battlefield.iter().any(|p| p.card.id == ral));
}

fn cast(game: &mut Game, definition: CardDefinitionId, player: PlayerId, graveyard: bool) {
    let card = game.build_zone(player, &[definition]).unwrap().remove(0);
    let id = card.id;
    if graveyard {
        game.players[player.index()].graveyard.push(card);
    } else {
        game.players[player.index()].hand.push(card);
    }
    for color in [
        ManaColor::Blue,
        ManaColor::Black,
        ManaColor::Red,
        ManaColor::Colorless,
    ] {
        game.add_unrestricted_mana(player, color, 10);
    }
    game.priority = player;
    let action = game
        .legal_actions(player)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == id))
        .expect("spell is castable");
    game.apply(player, action).unwrap();
    while let Some(decision) = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
        && decision.kind == DecisionKind::TriggerOrder
    {
        game.apply(
            decision.player,
            Action::ChooseDecision {
                decision: decision.id,
                options: decision.options.iter().map(|option| option.id).collect(),
            },
        )
        .unwrap();
    }
}

fn storm_triggers(game: &Game) -> usize {
    game.stack
        .iter()
        .filter(|object| {
            object.kind == StackObjectKind::TriggeredAbility
                && object
                    .ability
                    .as_ref()
                    .and_then(|ability| ability.text)
                    .is_some_and(|text| text.starts_with("Storm"))
        })
        .count()
}

#[test]
fn ral_emblem_grants_storm_from_hand_and_graveyard_in_both_engines() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        // An opponent's earlier spell counts, even though it has no storm.
        cast(&mut game, cards::LIGHTNING_BOLT, PlayerId::Two, false);
        settle(&mut game);
        emblem(&mut game);
        for graveyard in [false, true] {
            let hand = game.players[0].hand.len();
            cast(&mut game, cards::THINK_TWICE, PlayerId::One, graveyard);
            assert_eq!(storm_triggers(&game), 1);
            settle(&mut game);
            // One prior spell, then two; copies themselves are not casts.
            let drawn = if graveyard { 3 } else { 2 };
            assert_eq!(game.players[0].hand.len(), hand + drawn);
        }
        assert!(
            game.players[0]
                .exile
                .iter()
                .any(|c| c.definition == cards::THINK_TWICE)
        );
    }
}

#[test]
fn multiple_emblems_and_printed_storm_trigger_independently() {
    let mut game = ready_game();
    emblem(&mut game);
    emblem(&mut game);
    cast(&mut game, cards::MOX_SAPPHIRE, PlayerId::One, false);
    assert_eq!(storm_triggers(&game), 0, "artifact spells are excluded");
    settle(&mut game);
    cast(&mut game, cards::LIGHTNING_BOLT, PlayerId::Two, false);
    assert_eq!(storm_triggers(&game), 0, "opponent's spells are excluded");
    settle(&mut game);
    cast(&mut game, cards::EMPTY_THE_WARRENS, PlayerId::One, false);
    assert_eq!(
        storm_triggers(&game),
        3,
        "printed plus two granted instances"
    );
    settle(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|p| p.card.definition.is_token())
            .count(),
        14
    );
}

#[test]
fn granted_storm_survives_checkpoint_and_source_removal_after_cast() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        emblem(&mut game);
        cast(&mut game, cards::MOX_SAPPHIRE, PlayerId::One, false);
        settle(&mut game);
        cast(&mut game, cards::THINK_TWICE, PlayerId::One, false);
        let (wire, hidden) = checkpoint_fixture(&game, PlayerId::One);
        let mut rebuilt = Game::from_observation_checkpoint(
            game.catalog.clone(),
            game.format,
            &wire,
            &hidden,
            42,
        )
        .unwrap();
        rebuilt.set_prepared_engine_enabled(prepared);
        assert_eq!(storm_triggers(&rebuilt), 1);
        let hand = rebuilt.players[0].hand.len();
        // The captured resolver owns the granted ability even without its source.
        rebuilt.emblems.clear();
        settle(&mut rebuilt);
        assert_eq!(rebuilt.players[0].hand.len(), hand + 2);
        cast(&mut rebuilt, cards::THINK_TWICE, PlayerId::One, false);
        assert_eq!(storm_triggers(&rebuilt), 0);
    }
}

#[test]
fn ral_loyalty_trigger_and_otter_prowess_use_the_shared_stack() {
    let mut game = ready_game();
    let ral = game
        .put_onto_battlefield(PlayerId::One, cards::RAL_CRACKLING_WIT)
        .unwrap();
    activate(&mut game, ral, 0);
    let otter = game
        .battlefield
        .iter()
        .find(|p| p.card.definition.is_token())
        .unwrap();
    let otter_id = otter.card.id;
    assert_eq!(game.power(otter), Some(1));
    assert_eq!(game.toughness(otter), Some(1));
    assert!(game.effective_subtypes(otter).contains(&"Otter"));
    cast(&mut game, cards::MOX_SAPPHIRE, PlayerId::One, false);
    assert_eq!(
        game.battlefield
            .iter()
            .find(|p| p.card.id == ral)
            .unwrap()
            .counters(CounterKind::Loyalty),
        5
    );
    settle(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .find(|p| p.card.id == ral)
            .unwrap()
            .counters(CounterKind::Loyalty),
        6
    );
    let otter = game
        .battlefield
        .iter()
        .find(|p| p.card.id == otter_id)
        .unwrap();
    assert_eq!(game.power(otter), Some(2));
    assert_eq!(game.toughness(otter), Some(2));
}

#[test]
fn ral_minus_three_draws_before_choosing_two_discards() {
    let mut game = ready_game();
    let ral = game
        .put_onto_battlefield(PlayerId::One, cards::RAL_CRACKLING_WIT)
        .unwrap();
    let ability = activated_ability_for(&game, ral, 1);
    game.apply(PlayerId::One, plain_activation(ral, ability))
        .unwrap();
    pass_priority_pair(&mut game);
    assert_eq!(game.players[0].hand.len(), 3);
    assert!(!game.pending_decisions.is_empty());
    settle(&mut game);
    assert_eq!(game.players[0].hand.len(), 1);
    assert_eq!(game.players[0].graveyard.len(), 2);
}

#[test]
fn emblem_matches_the_selected_adventure_face_and_face_down_characteristics() {
    let mut game = ready_game();
    emblem(&mut game);
    let giant = game
        .build_zone(PlayerId::One, &[cards::BONECRUSHER_GIANT])
        .unwrap()
        .remove(0);
    let id = giant.id;
    game.players[0].hand.push(giant);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 5);
    let stomp = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
            if *card == id && choices.play_option() == PlayOptionId(1))
        })
        .unwrap();
    game.apply(PlayerId::One, stomp).unwrap();
    assert_eq!(
        storm_triggers(&game),
        1,
        "Stomp is an instant despite the card's creature front"
    );
    settle(&mut game);
    let id = game.players[0]
        .exile
        .iter()
        .find(|c| c.definition == cards::BONECRUSHER_GIANT)
        .unwrap()
        .id;
    let giant = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == id))
        .unwrap();
    game.apply(PlayerId::One, giant).unwrap();
    assert_eq!(
        storm_triggers(&game),
        0,
        "the creature face does not have storm"
    );
    settle(&mut game);
    let mut masked = spell(870_301, cards::EMPTY_THE_WARRENS, PlayerId::One, 0);
    masked.face_down = Some(crate::card::face_down::ordinary());
    game.stack.push(masked);
    let mut abilities = Vec::new();
    game.for_each_stack_spell_ability(game.stack.last().unwrap(), |ability| {
        abilities.push(ability);
    });
    assert!(
        abilities.is_empty(),
        "a face-down creature spell has neither printed nor granted storm"
    );
}

#[test]
fn battlefield_grant_uses_spell_controller_and_survives_its_source_leaving() {
    for prepared in [false, true] {
        let mut game = ready_game();
        game.set_prepared_engine_enabled(prepared);
        let fixture = CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000902");
        let definition = CardDefinition::new(
            fixture,
            "Stack storm grant fixture",
            crate::card::sets::bloomburrow::SET,
            CardRules::new_enchantment(mana_cost!("{0}")).with_ability(AbilityDef::static_ability(
                "Instant spells you cast have storm.",
                EffectDef::StaticApply {
                    recipient: EffectRecipientDef::matching_objects(
                        ObjectPredicateDef::HasType(CardType::Instant),
                        &[ZoneKind::Stack],
                        PlayerRelation::You,
                    ),
                    effect: AppliedEffectDef::add_ability(&const { abilities::storm() }),
                },
            )),
        );
        game.catalog = CardCatalog::new(
            game.catalog
                .definitions()
                .into_iter()
                .cloned()
                .chain([definition]),
        )
        .unwrap();
        game.prepared_engine = PreparedEngine::compile(&game.catalog);
        let source = game.put_onto_battlefield(PlayerId::One, fixture).unwrap();
        cast(&mut game, cards::MOX_SAPPHIRE, PlayerId::One, false);
        settle(&mut game);
        // An opponent-owned card cast from exile belongs to the caster on the stack.
        let held = game
            .build_zone(PlayerId::Two, &[cards::THINK_TWICE])
            .unwrap()
            .remove(0);
        let id = held.id;
        game.players[1].exile.push(held);
        game.permit_conditional_cast_while_exiled(id, PlayerId::One);
        let cast = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == id))
            .unwrap();
        game.apply(PlayerId::One, cast).unwrap();
        assert_eq!(storm_triggers(&game), 1);
        game.destroy_permanent(source);
        let hand = game.players[0].hand.len();
        settle(&mut game);
        assert_eq!(game.players[0].hand.len(), hand + 2);
    }
}
