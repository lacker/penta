use super::*;
use crate::engine_profiling::{Capture, Report};

fn count(report: &Report, category: &str, operation: &str, path: &str, reason: &str) -> u64 {
    report
        .counters
        .iter()
        .filter(|counter| {
            counter.category == category
                && counter.operation == operation
                && counter.path == path
                && counter.reason == reason
        })
        .map(|counter| counter.count)
        .sum()
}

#[test]
fn engine_profile_draw_dispatch_preserves_events_and_observations() {
    let baseline = resolve_think_twice(true);
    let capture = Capture::start().unwrap();
    let instrumented = resolve_think_twice(true);
    let report = capture.finish();
    assert_eq!(
        baseline.observe(PlayerId::One),
        instrumented.observe(PlayerId::One)
    );
    assert_eq!(baseline.players, instrumented.players);
    assert_eq!(baseline.events, instrumented.events);
    assert_eq!(baseline.pending_procedures, instrumented.pending_procedures);
    assert_eq!(
        count(
            &report,
            "effect_dispatch",
            "DrawCards",
            "prepared",
            "entered"
        ),
        1
    );
    assert_eq!(
        count(
            &report,
            "effect_dispatch",
            "DrawCards",
            "reference",
            "entered"
        ),
        0
    );

    let capture = Capture::start().unwrap();
    let reference = resolve_think_twice(false);
    let report = capture.finish();
    assert_eq!(
        baseline.observe(PlayerId::One),
        reference.observe(PlayerId::One)
    );
    assert_eq!(
        count(
            &report,
            "effect_fallback",
            "DrawCards",
            "reference",
            "engine_disabled"
        ),
        1
    );
    assert_eq!(
        count(
            &report,
            "effect_dispatch",
            "DrawCards",
            "reference",
            "entered"
        ),
        1
    );
    assert_eq!(
        count(
            &report,
            "effect_dispatch",
            "DrawCards",
            "prepared",
            "entered"
        ),
        0
    );
}

#[test]
fn engine_profile_predicate_fallback_reasons_are_exact() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.battlefield
        .push(creature(98_901, cards::SERRA_ANGEL, PlayerId::One));
    let source = game.battlefield[0].card.id;
    let query = ObjectQueryDef::matching(
        ObjectPredicateDef::Any,
        &[ZoneKind::Battlefield],
        PlayerRelation::You,
    );
    let capture = Capture::start().unwrap();
    assert_eq!(
        game.objects_matching_query(query, PlayerId::One, source, TriggerContext::empty())
            .len(),
        1
    );
    game.set_prepared_engine_enabled(false);
    assert_eq!(
        game.objects_matching_query(query, PlayerId::One, source, TriggerContext::empty())
            .len(),
        1
    );
    game.set_prepared_engine_enabled(true);
    let bound =
        ObjectPredicateDef::Subtype(crate::SubtypeDef::Binding(crate::Binding!("chosen_type")));
    assert!(game.prepared_engine.predicate(bound).is_none());
    let prospective = creature(98_902, cards::BLOOD_MOON, PlayerId::One);
    game.objects_matching_query_with_prospective(
        query,
        PlayerId::One,
        source,
        TriggerContext::empty(),
        Some(&prospective),
    );
    let report = capture.finish();
    assert_eq!(
        count(&report, "predicate_plan", "Any", "prepared", "supported"),
        1
    );
    assert_eq!(
        count(
            &report,
            "predicate_plan",
            "Any",
            "reference",
            "engine_disabled"
        ),
        1
    );
    assert_eq!(
        count(
            &report,
            "predicate_plan",
            "Any",
            "reference",
            "prospective_context"
        ),
        1
    );
    assert_eq!(
        count(
            &report,
            "predicate_plan",
            "Subtype",
            "reference",
            "unsupported_predicate"
        ),
        1
    );
}

#[test]
fn engine_profile_reference_short_circuit_counts_only_executed_nodes() {
    let game = ready_game();
    let permanent = creature(98_903, cards::SERRA_ANGEL, PlayerId::One);
    let source = permanent.card.id;
    let object = game.trigger_event_object(&permanent);
    let capture = Capture::start().unwrap();
    assert!(!game.trigger_object_matches(
        ObjectPredicateDef::All(&[ObjectPredicateDef::Spell, ObjectPredicateDef::Tapped,]),
        &object,
        source,
        false
    ));
    let report = capture.finish();
    assert_eq!(
        count(
            &report,
            "predicate_evaluation",
            "All",
            "reference_snapshot",
            "entered"
        ),
        1
    );
    assert_eq!(
        count(
            &report,
            "predicate_evaluation",
            "Spell",
            "reference_snapshot",
            "entered"
        ),
        1
    );
    assert_eq!(
        count(
            &report,
            "predicate_evaluation",
            "Tapped",
            "reference_snapshot",
            "entered"
        ),
        0
    );
}
