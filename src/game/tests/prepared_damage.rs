use super::*;

fn bolt_on_stack(prepared: bool, target: Target) -> Game {
    let mut game = ready_game();
    game.set_prepared_engine_enabled(prepared);
    game.players[0].hand.clear();
    let spell = card(98_500, cards::LIGHTNING_BOLT, PlayerId::One);
    let id = spell.id;
    game.players[0].hand.push(spell);
    game.players[0].mana_pool.red = 1;
    game.battlefield
        .push(creature(98_501, cards::SERRA_ANGEL, PlayerId::Two));
    let action = cast_action(id, vec![target], Vec::new(), 0);
    assert!(game.legal_actions(PlayerId::One).contains(&action));
    game.apply(PlayerId::One, action).unwrap();
    assert!(matches!(
        game.stack
            .last()
            .unwrap()
            .ability
            .as_ref()
            .unwrap()
            .resolver,
        StackAbilityResolver::Prepared {
            effect: crate::prepared_engine::PreparedEffect::DealDamage { .. },
            ..
        }
    ));
    game
}

fn assert_same(prepared: &Game, reference: &Game) {
    assert_eq!(prepared.players, reference.players);
    assert_eq!(prepared.battlefield, reference.battlefield);
    assert_eq!(
        prepared.stack.iter().collect::<Vec<_>>(),
        reference.stack.iter().collect::<Vec<_>>()
    );
    assert_eq!(prepared.events, reference.events);
    assert_eq!(prepared.pending_events, reference.pending_events);
    assert_eq!(prepared.pending_procedures, reference.pending_procedures);
    assert_eq!(prepared.damage_preventions, reference.damage_preventions);
    assert_eq!(prepared.result, reference.result);
    for player in [PlayerId::One, PlayerId::Two] {
        assert_eq!(prepared.observe(player), reference.observe(player));
    }
}

#[test]
fn prepared_damage_matches_reference_for_players_permanents_and_illegal_targets() {
    for target in [
        Target::Player(PlayerId::Two),
        Target::Permanent(GameObjectId(98_501)),
    ] {
        for remove_target in [false, true] {
            let mut prepared = bolt_on_stack(true, target);
            let mut reference = bolt_on_stack(false, target);
            if remove_target {
                for game in [&mut prepared, &mut reference] {
                    game.battlefield
                        .retain(|p| p.card.id != GameObjectId(98_501));
                }
            }
            pass_priority_pair(&mut prepared);
            pass_priority_pair(&mut reference);
            assert_same(&prepared, &reference);
            if target == Target::Player(PlayerId::Two) {
                assert_eq!(prepared.players[1].life, 17);
            }
        }
    }
}

#[test]
fn prepared_damage_keeps_prevention() {
    use crate::game::prevention_state::{
        ResolvedDamagePrevention, ResolvedDamagePreventionCapacity, ResolvedDamageRecipientMatcher,
        ResolvedDamageSourceMatcher,
    };
    let mut prepared = bolt_on_stack(true, Target::Player(PlayerId::Two));
    let mut reference = bolt_on_stack(false, Target::Player(PlayerId::Two));
    for game in [&mut prepared, &mut reference] {
        let timestamp = game.allocate_continuous_effect_timestamp();
        game.damage_preventions.push(ResolvedDamagePrevention {
            source: ResolvedDamageSourceMatcher::Any,
            recipient: ResolvedDamageRecipientMatcher::Any,
            combat_only: false,
            capacity: ResolvedDamagePreventionCapacity::Amount(2),
            amount: ValueDef::Constant(2),
            gain_life: None,
            source_ability: AbilitySourceRef {
                object: GameObjectId(98_501),
                ability: primary_ability(cards::SERRA_ANGEL),
            },
            timestamp,
            expiration: ContinuousEffectExpiration::EndOfTurn,
        });
    }
    pass_priority_pair(&mut prepared);
    pass_priority_pair(&mut reference);
    assert_same(&prepared, &reference);
    assert_eq!(prepared.players[1].life, 19);
}

#[test]
fn prepared_damage_recipient_plans_preserve_context_order_and_amount_clamping() {
    for recipient in [
        EffectRecipientDef::Controller,
        EffectRecipientDef::Opponent,
        EffectRecipientDef::EachPlayer,
        EffectRecipientDef::EventPlayer,
        EffectRecipientDef::ControllerOfTriggeringObject,
    ] {
        for amount in [-1, 0, 2, 70_000] {
            let mut prepared = bolt_on_stack(true, Target::Player(PlayerId::Two));
            let mut reference = bolt_on_stack(false, Target::Player(PlayerId::Two));
            // Execute a supported root using a real frozen spell object. The
            // trigger object is absent: the controller is last-known context.
            let object = prepared.stack.last().unwrap().clone();
            let mut context: EffectResolutionContext = TriggerContext::empty().into();
            context.trigger.event_player = Some(PlayerId::Two);
            context.trigger.object_controller = Some(PlayerId::Two);
            let effect = EffectDef::damage(recipient, ValueDef::Constant(amount));
            let scoped = ScopedEffect::primary(effect);
            let plan = crate::prepared_engine::compile_effect(effect).unwrap();
            crate::prepared_engine::execute_effect(
                plan,
                &mut crate::game::prepared_host::PreparedResolution {
                    game: &mut prepared,
                    object: &object,
                    context: &context,
                    reference: scoped,
                },
                object.controller,
                object.source,
                object.ability_origin().unwrap(),
            );
            reference.resolve_effect_def(scoped, &object, context);
            assert_same(&prepared, &reference);
        }
    }
}

#[test]
fn prepared_damage_resolver_cache_matches_uncached_catalog_and_changed_effects() {
    let game = ready_game();
    for definition in game.catalog.definitions() {
        for part in &definition.parts {
            for (index, ability) in part.rules.ability_clauses().iter().enumerate() {
                let origin = AbilityOrigin::Printed {
                    definition: definition.id,
                    part: part.id,
                    ability: AbilityId::from_index(index).unwrap(),
                };
                assert_eq!(
                    game.cached_ability_resolver(origin, ability),
                    Game::ability_resolver(origin, ability)
                );
            }
        }
    }
    let origin = primary_ability(cards::LIGHTNING_BOLT);
    for effect in [
        EffectDef::damage(EffectRecipientDef::EachPlayer, ValueDef::Constant(7)),
        EffectDef::damage(EffectRecipientDef::Controller, ValueDef::ChosenX),
    ] {
        assert_eq!(
            game.prepared_engine.resolving_effect(origin, effect),
            crate::prepared_engine::compile_effect(effect)
        );
    }
}

#[test]
fn prepared_damage_obeys_mode_fallback_and_restores_from_checkpoint() {
    let mut prepared = bolt_on_stack(true, Target::Player(PlayerId::Two));
    let mut reference = bolt_on_stack(false, Target::Player(PlayerId::Two));
    for game in [&mut prepared, &mut reference] {
        game.stack
            .iter_mut()
            .next_back()
            .unwrap()
            .ability
            .as_mut()
            .unwrap()
            .mode_effects
            .push(ScopedEffect::primary(EffectDef::damage(
                EffectRecipientDef::Controller,
                ValueDef::Constant(1),
            )));
    }
    pass_priority_pair(&mut prepared);
    pass_priority_pair(&mut reference);
    assert_same(&prepared, &reference);
    assert_eq!(prepared.players[0].life, 19);

    let game = bolt_on_stack(true, Target::Player(PlayerId::Two));
    let (wire, hidden) = checkpoint_fixture(&game, game.priority);
    let mut restored = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &wire,
        &hidden,
        98_510,
    )
    .unwrap();
    assert!(matches!(
        restored
            .stack
            .last()
            .unwrap()
            .ability
            .as_ref()
            .unwrap()
            .resolver,
        StackAbilityResolver::Prepared {
            effect: crate::prepared_engine::PreparedEffect::DealDamage { .. },
            ..
        }
    ));
    pass_priority_pair(&mut restored);
    assert_eq!(restored.players[1].life, 17);
}

#[test]
fn prepared_damage_preserves_retired_ability_sources_and_protection() {
    for protection in [false, true] {
        let run = |enabled| {
            let mut game = ready_game();
            game.set_prepared_engine_enabled(enabled);
            game.turns_started[0] = 1;
            let mut source = creature(98_520, cards::ICATIAN_JAVELINEERS, PlayerId::One);
            source.counters.set(CounterKind::named("javelin"), 1);
            let source_id = source.card.id;
            let target = creature(98_521, cards::SERRA_ANGEL, PlayerId::Two);
            let target_id = target.card.id;
            game.battlefield = vec![source, target];
            let ability = activated_ability_for(&game, source_id, 0);
            game.apply(
                PlayerId::One,
                Action::ActivateAbility {
                    source: source_id,
                    ability,
                    targets: activated_targets(Target::Permanent(target_id)),
                    cost_objects: Vec::new(),
                    x: 0,
                    modes: Vec::new(),
                    mana_payment: None,
                },
            )
            .unwrap();
            assert!(matches!(
                game.stack
                    .last()
                    .unwrap()
                    .ability
                    .as_ref()
                    .unwrap()
                    .resolver,
                StackAbilityResolver::Prepared {
                    effect: crate::prepared_engine::PreparedEffect::DealDamage { .. },
                    ..
                }
            ));
            game.destroy_permanent_without_regeneration(source_id);
            if protection {
                game.battlefield[0]
                    .temporary_keywords
                    .push(protection_keyword(ManaColor::White));
            }
            pass_priority_pair(&mut game);
            assert_eq!(game.battlefield[0].damage, u16::from(!protection));
            game
        };
        assert_same(&run(true), &run(false));
    }
}

#[test]
fn prepared_damage_uses_the_scoped_target_slot() {
    let mut prepared = bolt_on_stack(true, Target::Player(PlayerId::Two));
    let mut reference = bolt_on_stack(false, Target::Player(PlayerId::Two));
    let mut object = prepared.stack.last().unwrap().clone();
    let ability = object.ability.as_mut().unwrap();
    ability.target_defs.push(ability.target_defs[0]);
    ability.targets.push(TargetSelection::single(
        TargetSlotId(1),
        Target::Player(PlayerId::One),
    ));
    object.signature = Some(CastSignature::from_validated_choices(
        SpellForm::Part(CardPartId::PRIMARY),
        CastChoices::default().with_targets(ability.targets.clone()),
    ));
    let effect = EffectDef::damage(
        EffectRecipientDef::Target(TargetIndex::PRIMARY),
        ValueDef::Constant(3),
    );
    let scoped = ScopedEffect::at(effect, 1);
    let context: EffectResolutionContext = TriggerContext::empty().into();
    let plan = crate::prepared_engine::compile_effect(effect).unwrap();
    crate::prepared_engine::execute_effect(
        plan,
        &mut crate::game::prepared_host::PreparedResolution {
            game: &mut prepared,
            object: &object,
            context: &context,
            reference: scoped,
        },
        object.controller,
        object.source,
        object.ability_origin().unwrap(),
    );
    reference.resolve_effect_def(scoped, &object, context);
    assert_same(&prepared, &reference);
    assert_eq!(prepared.players[0].life, 17);
    assert_eq!(prepared.players[1].life, 20);
}
