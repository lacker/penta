use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.battlefield.clear();
    for player in &mut game.players {
        player.hand.clear();
        player.graveyard.clear();
        player.exile.clear();
        player.mana_pool = ManaPool::default();
    }
    game
}

fn activation(game: &Game, player: PlayerId, source: GameObjectId) -> Option<Action> {
    game.legal_actions(player).into_iter().find(|action| {
        matches!(
            action, Action::ActivateAbility { source: actual, .. } if *actual == source
        )
    })
}

fn choose(game: &mut Game, index: usize) {
    let decision = game.pending_decisions[0].observation.clone();
    game.apply(
        decision.player,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![decision.options[index].id],
        },
    )
    .unwrap();
}

#[test]
fn cycling_fluctuator_discounts_typecycling_and_only_its_controllers_abilities() {
    for prepared in [false, true] {
        for definition in [cards::ETERNAL_DRAGON, cards::AKROMAS_VENGEANCE] {
            for owner in [PlayerId::One, PlayerId::Two] {
                let mut game = ready();
                game.set_prepared_engine_enabled(prepared);
                game.put_onto_battlefield(owner, cards::FLUCTUATOR).unwrap();
                let held = card(171_010, definition, PlayerId::One);
                let id = held.id;
                game.players[0].hand.push(held);
                let mana = u16::from(definition == cards::AKROMAS_VENGEANCE);
                game.players[0].mana_pool.colorless = mana;
                let offered = activation(&game, PlayerId::One, id);
                assert_eq!(offered.is_some(), owner == PlayerId::One);
                if let Some(action) = offered {
                    game.apply(PlayerId::One, action).unwrap();
                    assert_eq!(game.players[0].mana_pool.total(), 0);
                    assert_eq!(game.players[0].graveyard.len(), 1);
                }
            }
        }
    }
}

#[test]
fn cycling_fluctuator_does_not_remove_colored_mana() {
    let mut game = ready();
    game.put_onto_battlefield(PlayerId::One, cards::FLUCTUATOR)
        .unwrap();
    let held = card(171_020, cards::SECLUDED_STEPPE, PlayerId::One);
    let id = held.id;
    game.players[0].hand.push(held);
    assert!(activation(&game, PlayerId::One, id).is_none());
    game.players[0].mana_pool.white = 1;
    game.apply(PlayerId::One, activation(&game, PlayerId::One, id).unwrap())
        .unwrap();
    assert_eq!(game.players[0].mana_pool.total(), 0);
}

#[test]
fn cycling_rift_targets_on_trigger_placement_but_pays_on_resolution() {
    for prepared in [false, true] {
        for cycler in [PlayerId::One, PlayerId::Two] {
            for pay in [false, true] {
                let mut game = ready();
                game.set_prepared_engine_enabled(prepared);
                let rift = game
                    .put_onto_battlefield(PlayerId::One, cards::LIGHTNING_RIFT)
                    .unwrap();
                // Typecycling has the same occurrence, despite not drawing.
                game.put_onto_battlefield(cycler, cards::FLUCTUATOR)
                    .unwrap();
                let held = card(171_030, cards::ETERNAL_DRAGON, cycler);
                let id = held.id;
                game.players[cycler.index()].hand.push(held);
                game.players[0].mana_pool.colorless = 1;
                game.priority = cycler;
                game.apply(cycler, activation(&game, cycler, id).unwrap())
                    .unwrap();
                assert_eq!(game.players[0].mana_pool.colorless, 1);
                assert_eq!(
                    game.stack.len(),
                    1,
                    "the cycling ability waits below target selection"
                );
                let DecisionContinuation::TriggerPlacement { candidates, .. } =
                    &game.pending_decisions[0].continuation
                else {
                    panic!("Rift chooses its target when the trigger is placed");
                };
                let target = candidates
                    .iter()
                    .position(|target| *target == Target::Player(PlayerId::Two))
                    .unwrap();
                choose(&mut game, target);
                assert_eq!(game.stack.len(), 2);
                assert_eq!(game.stack.last().unwrap().source, Some(rift));
                assert_eq!(
                    game.players[0].mana_pool.colorless, 1,
                    "placement has not paid the optional cost"
                );
                game.resolve_stack_top();
                let last = game.pending_decisions[0].observation.options.len() - 1;
                choose(&mut game, if pay { last } else { 0 });
                assert_eq!(game.players[1].life, if pay { 18 } else { 20 });
                assert_eq!(game.players[0].mana_pool.colorless, u16::from(!pay));
                assert_eq!(
                    game.stack.len(),
                    1,
                    "Rift finished before the typecycling search"
                );
            }
        }
    }
}

#[test]
fn cycling_and_discard_are_two_labels_on_one_occurrence() {
    static WATCHERS: [AbilityDef; 4] = [
        AbilityDef::triggered(
            "Whenever a player discards a card.",
            TriggerEventDef::mechanic_performed(abilities::DISCARD, PlayerRelation::Any),
            EffectDef::None,
        ),
        AbilityDef::triggered(
            "Whenever a player cycles a card.",
            TriggerEventDef::mechanic_performed(abilities::CYCLING, PlayerRelation::Any),
            EffectDef::None,
        ),
        AbilityDef::triggered(
            "Whenever a player cycles or discards a card.",
            TriggerEventDef::AnyOf(&[
                TriggerEventDef::mechanic_performed(abilities::DISCARD, PlayerRelation::Any),
                TriggerEventDef::mechanic_performed(abilities::CYCLING, PlayerRelation::Any),
            ]),
            EffectDef::None,
        ),
        AbilityDef::triggered(
            "Whenever a player discards one or more cards.",
            TriggerEventDef::MechanicPerformed {
                mechanic: abilities::DISCARD,
                player: PlayerRelation::Any,
                object: None,
                one_or_more: true,
            },
            EffectDef::None,
        ),
    ];
    for prepared in [false, true] {
        for cycling in [false, true] {
            let mut game = ready();
            let id = CardDefinitionId::new(171_040);
            let mut watcher = CardDefinition::new(
                id,
                "Named discard watcher",
                CardSet::Magic2014,
                CardRules::new_artifact(ManaCost::default()).with_abilities(&WATCHERS),
            );
            synchronize_single_part_definition(&mut watcher);
            let mut definitions: Vec<_> = game.catalog.definitions().into_iter().cloned().collect();
            definitions.push(watcher);
            game.catalog = CardCatalog::new(definitions).unwrap();
            game.set_prepared_engine_enabled(prepared);
            game.put_onto_battlefield(PlayerId::One, id).unwrap();
            let held = card(171_041, cards::SECLUDED_STEPPE, PlayerId::Two);
            let source = held.id;
            game.players[1].hand.push(held);
            if cycling {
                game.players[1].mana_pool.white = 1;
                game.priority = PlayerId::Two;
                game.apply(
                    PlayerId::Two,
                    activation(&game, PlayerId::Two, source).unwrap(),
                )
                .unwrap();
                // The simultaneously triggered clauses await ordering.
                let DecisionContinuation::TriggerOrder { batch, .. } =
                    &game.pending_decisions[0].continuation
                else {
                    panic!("the observers need an order");
                };
                assert_eq!(
                    batch.triggers.len(),
                    4,
                    "the OR clause must not trigger twice"
                );
            } else {
                game.discard_cards_with_cause(PlayerId::Two, &[source], ZoneMoveCause::Rules);
                assert_eq!(
                    game.pending_triggers.len(),
                    3,
                    "ordinary discard is not cycling"
                );
                game.pending_triggers.clear();
                let batch = [GameObjectId(171_042), GameObjectId(171_043)];
                for id in batch {
                    game.players[1]
                        .hand
                        .push(card(id.0, cards::SECLUDED_STEPPE, PlayerId::Two));
                }
                game.discard_cards_with_cause(PlayerId::Two, &batch, ZoneMoveCause::Rules);
                assert_eq!(
                    game.pending_triggers.len(),
                    5,
                    "two per-card clauses each fire twice, the batch clause once"
                );
            }
        }
    }
}
