use super::*;

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
fn prepared_engine_is_enabled_by_default_and_can_be_disabled() {
    let mut game = ready_game();
    assert!(game.prepared_engine_enabled());
    game.set_prepared_engine_enabled(false);
    assert!(!game.prepared_engine_enabled());
}
