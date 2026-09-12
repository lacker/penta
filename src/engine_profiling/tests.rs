use super::*;

#[test]
fn engine_profile_capture_is_scoped_sorted_and_thread_local() {
    record("effect_dispatch", "ignored", "reference", "entered");
    let capture = Capture::start().unwrap();
    record("effect_dispatch", "DrawCards", "reference", "entered");
    assert!(
        Capture::start().is_err(),
        "nested start must preserve counters"
    );
    let other = std::thread::spawn(|| {
        record(
            "effect_dispatch",
            "uncaptured_thread",
            "reference",
            "entered",
        );
        let capture = Capture::start().unwrap();
        record("effect_dispatch", "GainLife", "reference", "entered");
        capture.finish()
    })
    .join()
    .unwrap();
    assert_eq!(other.counters.len(), 1);
    assert_eq!(other.counters[0].operation, "GainLife");
    record("effect_dispatch", "Apply", "prepared", "entered");
    record("effect_dispatch", "DrawCards", "reference", "entered");
    let report = capture.finish();
    assert_eq!(report.counters.len(), 2);
    assert_eq!(report.counters[0].operation, "Apply");
    assert_eq!(report.counters[1].count, 2);
    let wire = serde_json::to_value(report).unwrap();
    assert_eq!(wire["schema_version"], 1);
    assert_eq!(wire["scope"], "current_thread");
    assert!(Capture::start().unwrap().finish().counters.is_empty());
}

#[test]
fn engine_profile_drop_and_unwind_release_capture() {
    let _ = std::panic::catch_unwind(|| {
        let _capture = Capture::start().unwrap();
        record("effect_dispatch", "DrawCards", "reference", "entered");
        panic!("test capture cleanup");
    });
    assert!(Capture::start().unwrap().finish().counters.is_empty());
    drop(Capture::start().unwrap());
    assert!(Capture::start().unwrap().finish().counters.is_empty());
}

#[test]
fn engine_profile_counts_executed_prepared_branches_only() {
    use crate::ObjectPredicateDef;
    let plan = crate::prepared_engine::PreparedPredicate::compile(ObjectPredicateDef::All(&[
        ObjectPredicateDef::Any,
        ObjectPredicateDef::Tapped,
        ObjectPredicateDef::Color(crate::ManaColor::White),
    ]))
    .unwrap();
    let capture = Capture::start().unwrap();
    assert!(!plan.matches(&mut |leaf| matches!(
        leaf,
        crate::prepared_engine::PreparedPredicateLeaf::Constant(true)
    )));
    let report = capture.finish();
    let names: Vec<_> = report
        .counters
        .iter()
        .map(|counter| counter.operation)
        .collect();
    assert_eq!(names, ["All", "Constant", "Tapped"]);
    assert!(report.counters.iter().all(|counter| counter.count == 1));
}
