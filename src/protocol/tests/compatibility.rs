use super::*;

#[test]
fn bot_compatibility_checks_both_required_capability_subsets() {
    assert!(
        check_bot_compatibility(
            PROTOCOL_VERSION,
            REQUIRED_BOT_CAPABILITIES,
            PROTOCOL_CAPABILITIES,
            Some(SIMULATION_FINGERPRINT),
        )
        .is_ok()
    );
    let mut with_unknown_extra = REQUIRED_BOT_CAPABILITIES.to_vec();
    with_unknown_extra.push("future.optional.v9");
    assert!(check_bot_compatibility(PROTOCOL_VERSION, &with_unknown_extra, &[], None).is_ok());

    let mismatch =
        check_bot_compatibility(PROTOCOL_VERSION - 1, &[], &[], None).expect_err("wrong epoch");
    assert!(mismatch.contains("does not match"));

    let missing_server = check_bot_compatibility(
        PROTOCOL_VERSION,
        REQUIRED_BOT_CAPABILITIES,
        &["future.unavailable.v1"],
        None,
    )
    .expect_err("bot requirement is unavailable");
    assert!(missing_server.contains("server is missing required capability"));

    let wrong_simulation = check_bot_compatibility(
        PROTOCOL_VERSION,
        REQUIRED_BOT_CAPABILITIES,
        &[],
        Some("sha256-wrong"),
    )
    .expect_err("trained bot requires another simulation");
    assert!(wrong_simulation.contains("simulation fingerprint"));
}

#[test]
fn capability_manifests_are_sorted_unique_and_required_is_advertised() {
    for capabilities in [PROTOCOL_CAPABILITIES, REQUIRED_BOT_CAPABILITIES] {
        assert!(
            capabilities.windows(2).all(|pair| pair[0] < pair[1]),
            "capabilities must be sorted and unique: {capabilities:?}"
        );
    }
    assert!(
        REQUIRED_BOT_CAPABILITIES
            .iter()
            .all(|required| PROTOCOL_CAPABILITIES.contains(required))
    );
}

#[test]
fn catalog_and_observation_publish_one_compatibility_manifest() {
    let catalog = poc::catalog().expect("catalog builds");
    let catalog_json = catalog_json_for_format(&catalog, Format::OldSchool9394);
    let game = BotGame::new("Sligh", "Goblins", Opponent::External, PlayerId::Two, 42)
        .expect("game starts");
    let seat = game.decision_seat().expect("opening decision");
    let observation: Value =
        serde_json::from_str(&game.observe_json(seat)).expect("observation is JSON");

    for document in [&catalog_json, &observation] {
        assert_eq!(document["protocolVersion"], PROTOCOL_VERSION);
        assert_eq!(
            document["protocolCapabilities"],
            json!(PROTOCOL_CAPABILITIES)
        );
        assert_eq!(document["engineVersion"], ENGINE_VERSION);
        assert_eq!(document["simulationFingerprint"], SIMULATION_FINGERPRINT);
    }
    assert!(SIMULATION_FINGERPRINT.starts_with("sha256-"));
    assert_eq!(SIMULATION_FINGERPRINT.len(), "sha256-".len() + 64);
}
