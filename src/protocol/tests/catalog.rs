use super::*;

#[test]
fn the_catalog_lists_every_card_with_names_and_costs() {
    let catalog = poc::catalog().expect("catalog builds");
    let value = catalog_json(&catalog);
    let cards = value["cards"].as_array().expect("cards array");
    assert!(cards.len() > 100, "the pool is substantial");
    assert!(cards.iter().all(|card| card["name"].is_string()));
    assert!(
        cards
            .iter()
            .any(|card| card["name"] == "Lightning Bolt" && card["manaCost"]["red"] == 1)
    );
}

#[test]
fn catalog_mana_cost_distinguishes_no_cost_from_printed_zero() {
    let catalog = poc::catalog().expect("catalog builds");
    let value = catalog_json(&catalog);
    let cards = value["cards"].as_array().expect("cards array");
    let find = |name: &str| {
        cards
            .iter()
            .find(|card| card["name"] == name)
            .unwrap_or_else(|| panic!("{name} is cataloged"))
    };

    let mountain = find("Mountain");
    assert!(mountain["manaCost"].is_null());
    assert!(mountain["parts"][0]["manaCost"].is_null());

    let mox = find("Mox Ruby");
    assert!(mox["manaCost"].is_object());
    assert_eq!(mox["manaCost"]["generic"], 0);
    assert_eq!(mox["parts"][0]["manaCost"]["generic"], 0);
}

#[test]
fn a_token_is_cataloged_for_lookup_but_never_legal_and_carries_no_art() {
    let catalog = poc::catalog().expect("catalog builds");
    let value = catalog_json_for_format(&catalog, Format::IsdRtrStandard);
    let cards = value["cards"].as_array().expect("cards array");
    let beast = cards
        .iter()
        .find(|card| card["definition"] == crate::card::cards::BEAST_TOKEN_3_3_GREEN.0)
        .expect("a client can resolve a token by definition");

    assert_eq!(beast["name"], "Beast");
    assert_eq!(beast["power"], 3);
    assert_eq!(beast["toughness"], 3);
    assert_eq!(beast["allowed"], false, "a token is in no card pool");
    assert_eq!(beast["legal"], false);
    assert!(
        beast["manaCost"].is_null(),
        "a token has no printed mana cost"
    );

    // The browser renders art only for a Scryfall UUID and otherwise falls
    // back to the card-type glyph, so an empty identifier is what keeps a
    // token from requesting an image that does not exist.
    let art = beast["art"]["scryfallId"].as_str().unwrap_or_default();
    assert!(
        art.is_empty(),
        "a token names no printing, so it has no Scryfall identifier"
    );
}

#[test]
fn catalog_exposes_derived_implementation_coverage_not_the_play_gate() {
    let catalog = poc::catalog().expect("catalog builds");
    let value = catalog_json_for_format(&catalog, Format::IsdRtrStandard);
    let cards = value["cards"].as_array().expect("cards array");
    let find = |name: &str| {
        cards
            .iter()
            .find(|card| card["name"] == name)
            .unwrap_or_else(|| panic!("{name} is cataloged"))
    };

    let pilgrim = find("Avacyn's Pilgrim");
    assert_eq!(pilgrim["implementationStatus"], "complete");
    assert_eq!(pilgrim["parts"][0]["implementationStatus"], "complete");
    assert!(pilgrim.get("effectStatus").is_none());
    assert!(pilgrim["parts"][0].get("effectStatus").is_none());

    // Any card with a mix of executable and pending clauses will do here.
    // Chaos Orb's tap ability works, but the flip itself is a
    // deterministic approximation; repoint this if that changes.
    let partial = find("Chaos Orb");
    assert_eq!(partial["implementationStatus"], "partial");
    assert_eq!(partial["parts"][0]["implementationStatus"], "partial");

    // A card whose gap is a whole clause rather than a detail reports the
    // same way: Jace's ultimate is cataloged and does nothing, and his
    // other two play. Vraska used to be the example here, until attack
    // defenders made her retaliation reachable and she went complete. No
    // card in the catalog is metadata-only through and through any more,
    // so that status has no example left to name.
    let jace = find("Jace, Architect of Thought");
    assert_eq!(jace["implementationStatus"], "partial");
    assert_eq!(jace["parts"][0]["implementationStatus"], "partial");
    let vraska = find("Vraska the Unseen");
    assert_eq!(vraska["implementationStatus"], "complete");
    assert!(
        cards
            .iter()
            .all(|card| card["implementationStatus"] != "metadataOnly"),
        "every card in this format executes at least one clause"
    );
    // Pithing Needle is now complete; this keeps the coverage assertion
    // aligned with its newly executable card-name choice.
    let needle = find("Pithing Needle");
    assert_eq!(needle["implementationStatus"], "complete");
    assert_eq!(needle["parts"][0]["implementationStatus"], "complete");
    let blood_moon = find("Blood Moon");
    assert_eq!(blood_moon["implementationStatus"], "complete");
    assert_eq!(blood_moon["parts"][0]["implementationStatus"], "complete");
    let boros_charm = find("Boros Charm");
    assert_eq!(boros_charm["implementationStatus"], "complete");
    assert_eq!(boros_charm["parts"][0]["implementationStatus"], "complete");

    assert!(cards.iter().all(|card| {
        card["playOptions"].as_array().is_some_and(|options| {
            options
                .iter()
                .all(|option| option.get("effectStatus").is_none())
        })
    }));
}

#[test]
fn migrated_spells_do_not_require_an_additional_protocol_bump() {
    let catalog = poc::catalog().expect("catalog builds");
    let value = catalog_json_for_format(&catalog, Format::IsdRtrStandard);
    assert_eq!(
        PROTOCOL_VERSION, 17,
        "Mana Vault's corrected upkeep decision owns this branch's bump"
    );
    assert_eq!(value["protocolVersion"], PROTOCOL_VERSION);

    let cards = value["cards"].as_array().expect("cards array");
    let expected = [
        ("Doom Blade", "CreaturePermanent"),
        ("Swords to Plowshares", "CreaturePermanent"),
        ("Divine Offering", "Permanent"),
        ("Dispel", "Spell"),
        ("Dissipate", "Spell"),
        ("Putrefy", "Permanent"),
        ("Ultimate Price", "CreaturePermanent"),
        ("Warleader's Helix", "AnyTarget"),
    ];

    for (name, predicate) in expected {
        let card = cards
            .iter()
            .find(|card| card["name"] == name)
            .unwrap_or_else(|| panic!("{name} is cataloged"));
        let targets = card["playOptions"][0]["targets"]
            .as_array()
            .unwrap_or_else(|| panic!("{name} exposes target metadata"));
        assert_eq!(targets.len(), 1, "{name} has one target slot");
        assert_eq!(targets[0]["id"], 0, "{name} uses the primary slot");
        assert_eq!(
            targets[0]["predicate"], predicate,
            "{name} exposes its simplified target kind",
        );
        assert_eq!(targets[0]["minimum"], 1, "{name} requires its target");
        assert_eq!(targets[0]["maximum"], 1, "{name} takes one target");
        assert!(
            targets[0]["label"]
                .as_str()
                .is_some_and(|label| !label.is_empty()),
            "{name} exposes a presentation label",
        );
    }
}

#[test]
fn deck_names_all_resolve() {
    for name in deck_names() {
        assert!(deck_by_name(name).is_some(), "{name} resolves");
    }
    assert!(deck_by_name("Not A Deck").is_none());
}

#[test]
fn both_format_deck_registries_resolve_without_cross_format_leakage() {
    assert_eq!(deck_names(), deck_names_for_format(Format::OldSchool9394));
    assert_eq!(deck_names_for_format(Format::OldSchool9394).len(), 15);
    assert_eq!(deck_names_for_format(Format::IsdRtrStandard).len(), 8);

    for format in [Format::OldSchool9394, Format::IsdRtrStandard] {
        for name in deck_names_for_format(format) {
            assert!(
                deck_by_name_for_format(format, name).is_some(),
                "{name} resolves in {format}"
            );
        }
    }

    assert!(deck_by_name_for_format(Format::OldSchool9394, "Briksza Naya Midrange").is_none());
    assert!(deck_by_name_for_format(Format::IsdRtrStandard, "Sligh").is_none());
    assert!(
        deck_by_name_for_format(Format::IsdRtrStandard, "naya_midrange_rudy_briksza").is_some()
    );
    assert_eq!(
        parse_format_slug("old_school_93_94"),
        Ok(Format::OldSchool9394)
    );
    assert_eq!(
        parse_format_slug("isd-rtr-standard"),
        Ok(Format::IsdRtrStandard)
    );
    assert!(parse_format_slug("vintage").is_err());
}

#[test]
fn bot_game_stores_and_emits_its_format_and_rejects_wrong_decks() {
    let old_school = BotGame::new("Sligh", "Goblins", Opponent::External, PlayerId::Two, 18)
        .expect("compatibility constructor starts Old School");
    assert_eq!(old_school.format(), Format::OldSchool9394);

    let standard = BotGame::new_with_format(
        Format::IsdRtrStandard,
        "Briksza Naya Midrange",
        "Greer G/R Aggro",
        Opponent::External,
        PlayerId::Two,
        19,
    )
    .expect("Standard game starts");
    assert_eq!(standard.format(), Format::IsdRtrStandard);
    let seat = standard.decision_seat().expect("opening-hand decision");
    let observation: Value =
        serde_json::from_str(&standard.observe_json(seat)).expect("valid observation JSON");
    assert_eq!(observation["protocolVersion"], PROTOCOL_VERSION);
    assert_eq!(observation["format"], "isd-rtr-standard");
    assert!(
        observation["legalActions"]
            .as_array()
            .expect("actions")
            .iter()
            .all(|action| action["type"] != "Concede")
    );

    let configured = BotGame::from_config_json(
        r#"{"format":"isd-rtr-standard","p1Deck":"Lorren U/W Flash","p2Deck":"Arch U/W Flash","opponent":"external","seed":4}"#,
    )
    .expect("format slug selects Standard");
    assert_eq!(configured.format(), Format::IsdRtrStandard);
    assert!(
        BotGame::from_config_json(r#"{"format":2,"p1Deck":"Sligh","p2Deck":"Goblins"}"#)
            .err()
            .expect("non-string format is rejected")
            .contains("format must be a string")
    );

    assert!(
        BotGame::new_with_format(
            Format::OldSchool9394,
            "Briksza Naya Midrange",
            "Sligh",
            Opponent::External,
            PlayerId::Two,
            0,
        )
        .err()
        .expect("cross-format deck is rejected")
        .contains("unknown deck for old-school-93-94")
    );
    assert!(
        BotGame::new_with_format(
            Format::IsdRtrStandard,
            "Sligh",
            "Briksza Naya Midrange",
            Opponent::External,
            PlayerId::Two,
            0,
        )
        .err()
        .expect("cross-format deck is rejected")
        .contains("unknown deck for isd-rtr-standard")
    );
}

#[test]
fn catalog_json_is_structured_and_legality_is_format_specific() {
    let catalog = poc::catalog().expect("catalog builds");
    let old_school = catalog_json(&catalog);
    let standard = catalog_json_for_format(&catalog, Format::IsdRtrStandard);
    assert_eq!(old_school["format"], "old-school-93-94");
    assert_eq!(standard["format"], "isd-rtr-standard");

    let cards = standard["cards"].as_array().expect("cards array");
    assert!(cards.windows(2).all(|pair| {
        pair[0]["definition"].as_u64().expect("id") < pair[1]["definition"].as_u64().expect("id")
    }));
    let find = |name: &str| {
        cards
            .iter()
            .find(|card| card["name"] == name)
            .unwrap_or_else(|| panic!("{name} is cataloged"))
    };
    let turn_burn = find("Turn // Burn");
    assert_eq!(turn_burn["legal"], true);
    assert_eq!(turn_burn["structure"]["kind"], "split");
    assert_eq!(turn_burn["parts"].as_array().expect("parts").len(), 2);
    assert_eq!(
        turn_burn["playOptions"]
            .as_array()
            .expect("play options")
            .len(),
        3
    );
    assert!(
        !turn_burn["printings"]
            .as_array()
            .expect("printings")
            .is_empty()
    );

    let charm = find("Izzet Charm");
    assert_eq!(
        charm["playOptions"][0]["modes"]["choices"]
            .as_array()
            .expect("modes")
            .len(),
        3
    );
    assert_eq!(find("Lightning Bolt")["legal"], false);
    assert_eq!(find("Thespian's Stage")["debutSet"], "gatecrash");
    assert_eq!(find("Thespian's Stage")["legal"], true);
    assert_eq!(find("Darksteel Ingot")["debutSet"], "darksteel");
    assert_eq!(find("Darksteel Ingot")["legal"], false);
    assert_eq!(find("Dryad Arbor")["debutSet"], "future-sight");
    assert_eq!(find("Dryad Arbor")["legal"], false);
    assert_eq!(find("Nylea's Presence")["debutSet"], "theros");
    assert_eq!(find("Nylea's Presence")["legal"], false);
    assert_eq!(find("Urborg, Tomb of Yawgmoth")["debutSet"], "planar-chaos");
    assert_eq!(find("Urborg, Tomb of Yawgmoth")["legal"], false);
    assert_eq!(
        find("Yavimaya, Cradle of Growth")["debutSet"],
        "modern-horizons-2"
    );
    assert_eq!(find("Yavimaya, Cradle of Growth")["legal"], false);
    let old_bolt = old_school["cards"]
        .as_array()
        .expect("cards")
        .iter()
        .find(|card| card["name"] == "Lightning Bolt")
        .expect("bolt");
    assert_eq!(old_bolt["legal"], true);

    let juggernaut = old_school["cards"]
        .as_array()
        .expect("cards")
        .iter()
        .find(|card| card["name"] == "Juggernaut")
        .expect("Juggernaut is cataloged");
    assert_eq!(juggernaut["kind"], "ArtifactCreature");
    assert_eq!(juggernaut["parts"][0]["kind"], "ArtifactCreature");
    assert_eq!(
        juggernaut["parts"][0]["typeLine"],
        "Artifact Creature — Juggernaut"
    );
    assert_eq!(
        juggernaut["parts"][0]["colors"],
        json!([false, false, false, false, false])
    );
    assert_eq!(juggernaut["debutSet"], "alpha");
}
