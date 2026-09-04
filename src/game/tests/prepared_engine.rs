use super::*;
use crate::game::continuous_effects::StaticEffectKind;

fn resolve_think_twice(prepared: bool) -> Game {
    let mut game = ready_game();
    game.set_prepared_engine_enabled(prepared);
    let spell = card(98_000, cards::THINK_TWICE, PlayerId::One);
    let spell_id = spell.id;
    game.players[0].hand.push(spell);
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 1;

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == spell_id))
        .expect("Think Twice is castable");
    game.apply(PlayerId::One, cast).unwrap();
    assert!(matches!(
        game.stack.last().and_then(|object| object.ability.as_ref()),
        Some(StackAbilityPayload {
            resolver: StackAbilityResolver::Prepared { .. },
            ..
        })
    ));
    pass_priority_pair(&mut game);
    game
}

#[test]
fn prepared_draw_and_reference_draw_have_identical_state_changes() {
    let prepared = resolve_think_twice(true);
    let reference = resolve_think_twice(false);

    assert_eq!(prepared.players, reference.players);
    assert!(prepared.stack.is_empty());
    assert!(reference.stack.is_empty());
    assert_eq!(prepared.events, reference.events);
    assert_eq!(prepared.pending_events, reference.pending_events);
    assert_eq!(prepared.pending_procedures, reference.pending_procedures);
    assert_eq!(prepared.result, reference.result);
}

fn resolve_goblin_balloon_brigade_flight(prepared: bool, source_stays: bool) -> Game {
    let mut game = ready_game();
    game.set_prepared_engine_enabled(prepared);
    let goblin = creature(98_050, cards::GOBLIN_BALLOON_BRIGADE, PlayerId::One);
    let goblin_id = goblin.card.id;
    game.battlefield.push(goblin);
    game.players[0].mana_pool.red = 1;

    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == goblin_id)
        })
        .expect("Goblin Balloon Brigade can pay to gain flying");
    game.apply(PlayerId::One, activation).unwrap();
    assert!(matches!(
        game.stack.last().and_then(|object| object.ability.as_ref()),
        Some(StackAbilityPayload {
            resolver: StackAbilityResolver::Prepared { .. },
            ..
        })
    ));
    if !source_stays {
        game.battlefield
            .retain(|permanent| permanent.card.id != goblin_id);
    }
    pass_priority_pair(&mut game);
    if source_stays {
        let goblin = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == goblin_id)
            .expect("the source remains on the battlefield");
        assert!(game.permanent_has_executable_keyword(goblin, KeywordAbility::Flying));
    }
    game
}

#[test]
fn prepared_self_grant_and_reference_self_grant_have_identical_state_changes() {
    for source_stays in [true, false] {
        let prepared = resolve_goblin_balloon_brigade_flight(true, source_stays);
        let reference = resolve_goblin_balloon_brigade_flight(false, source_stays);

        assert_eq!(prepared.players, reference.players);
        assert_eq!(prepared.battlefield, reference.battlefield);
        assert!(prepared.stack.is_empty());
        assert!(reference.stack.is_empty());
        assert_eq!(prepared.events, reference.events);
        assert_eq!(prepared.pending_events, reference.pending_events);
        assert_eq!(prepared.pending_procedures, reference.pending_procedures);
        assert_eq!(prepared.result, reference.result);
        assert_eq!(
            prepared.next_continuous_effect_timestamp, reference.next_continuous_effect_timestamp,
            "the recipient filter still consumes the reference timestamp when the source left",
        );
    }
}

#[test]
fn prepared_source_ability_grant_preserves_live_nonbattlefield_sources() {
    let mut game = ready_game();
    let source = card(98_075, cards::GOBLIN_BALLOON_BRIGADE, PlayerId::One);
    let source_id = source.id;
    game.players[0].hand.push(source);
    let before_timestamp = game.next_continuous_effect_timestamp;
    let effect = crate::prepared_engine::compile_effect(EffectDef::Apply {
        recipient: EffectRecipientDef::Source,
        effect: AppliedEffectDef::add_ability(&TEST_FLYING_ABILITY[0]),
        duration: ResolvedEffectDurationDef::UntilEndOfTurn,
    })
    .expect("the source grant has a prepared lowering");

    crate::prepared_engine::execute_effect(
        effect,
        &mut game,
        PlayerId::One,
        Some(source_id),
        primary_ability(cards::GOBLIN_BALLOON_BRIGADE),
    );

    assert_eq!(game.nonbattlefield_ability_grants.len(), 1);
    assert_eq!(game.next_continuous_effect_timestamp, before_timestamp + 1,);
}

#[test]
fn prepared_static_summary_matches_reference_inspection() {
    let mut game = ready_game();
    for (id, definition) in [(98_100, cards::BLOOD_MOON), (98_101, cards::MOUNTAIN)] {
        let source = creature(id, definition, PlayerId::One);

        game.set_prepared_engine_enabled(false);
        let reference = game.supplies_land_type_effect(&source);
        game.set_prepared_engine_enabled(true);
        let prepared = game.supplies_land_type_effect(&source);

        assert_eq!(prepared, reference, "definition {definition:?}");
    }
}

#[test]
fn prepared_graveyard_source_summary_matches_reference_inspection() {
    let game = ready_game();
    for definition in game.catalog.definitions() {
        let card = card(98_150, definition.id, PlayerId::One);
        let mut reference = false;
        game.for_each_printed_card_ability(&card, &CharacteristicContext::Graveyard, |effective| {
            let ability = effective.ability;
            reference |= matches!(
                ability.definition,
                DeclarativeAbilityDef::Static(definition)
                    if definition.source_zones.contains(&ZoneKind::Graveyard)
            ) && ability.declarative_effect().is_some();
        });
        let prepared = game
            .prepared_supplies_graveyard_static(definition.id)
            .expect("catalog definitions have prepared graveyard summaries");
        assert_eq!(prepared, reference, "definition {:?}", definition.id);
    }
}

#[test]
fn live_structural_conditionals_fall_back_at_the_static_ability_root() {
    let game = ready_game();
    let program = game
        .prepared_static_program(ObjectCharacteristics::card(
            cards::SOULFLAYER,
            CardPartId::PRIMARY,
        ))
        .expect("catalog cards have prepared static programs");

    assert!(!program.abilities().is_empty());
    assert!(
        program
            .abilities()
            .iter()
            .all(|ability| ability.applications.is_none()),
        "live structural branches must retain reference component numbering",
    );
}

#[test]
fn prepared_engine_is_enabled_by_default_and_can_be_disabled() {
    let mut game = ready_game();
    assert!(game.prepared_engine_enabled());
    game.set_prepared_engine_enabled(false);
    assert!(!game.prepared_engine_enabled());
}

fn static_effects(
    game: &mut Game,
    affected: GameObjectId,
    kind: StaticEffectKind,
    prepared: bool,
) -> Vec<StaticAppliedEffect> {
    game.set_prepared_engine_enabled(prepared);
    let affected = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == affected)
        .cloned()
        .expect("the affected permanent is present");
    let mut effects = Vec::new();
    let result = game.visit_static_applied_effects(&affected, kind, |effect| {
        effects.push(effect);
        ControlFlow::Continue(())
    });
    assert!(result.is_continue());
    effects
}

#[test]
fn prepared_static_program_matches_reference_lanes_and_component_identity() {
    let mut game = ready_game();
    game.battlefield.clear();

    let mut kaito = creature(98_200, cards::KAITO_BANE_OF_NIGHTMARES, PlayerId::One);
    kaito.set_counters(CounterKind::Loyalty, 3);
    let kaito_id = kaito.card.id;
    game.battlefield.push(kaito);

    let opalescence = creature(98_201, cards::OPALESCENCE, PlayerId::One);
    let opalescence_id = opalescence.card.id;
    game.battlefield.push(opalescence);

    let plague = creature(98_202, cards::ENGINEERED_PLAGUE, PlayerId::One);
    let plague_id = plague.card.id;
    game.battlefield.push(plague);

    let leyline = creature(98_203, cards::LEYLINE_OF_SINGULARITY, PlayerId::One);
    let leyline_id = leyline.card.id;
    game.battlefield.push(leyline);

    game.active_player = PlayerId::One;
    for affected in [kaito_id, opalescence_id, plague_id, leyline_id] {
        for kind in [
            StaticEffectKind::Any,
            StaticEffectKind::Rules,
            StaticEffectKind::CardTypes,
            StaticEffectKind::Supertypes,
            StaticEffectKind::Colors,
            StaticEffectKind::Abilities,
            StaticEffectKind::Subtypes,
            StaticEffectKind::PowerToughness,
        ] {
            let reference = static_effects(&mut game, affected, kind, false);
            let prepared = static_effects(&mut game, affected, kind, true);
            assert_eq!(prepared, reference, "affected {affected:?}, lane {kind:?}");
        }
    }
}
