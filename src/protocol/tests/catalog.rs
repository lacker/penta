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
fn catalog_does_not_publish_creator_owned_virtual_characteristics() {
    let catalog = poc::catalog().expect("catalog builds");
    let value = catalog_json_for_format(&catalog, Format::IsdM14Standard);
    let cards = value["cards"].as_array().expect("cards array");

    let synthetic = cards
        .iter()
        .filter(|card| card["debutSet"] == "token")
        .collect::<Vec<_>>();
    assert!(synthetic.is_empty());
}

#[test]
fn migrated_spells_publish_stable_target_predicates() {
    let catalog = poc::catalog().expect("catalog builds");
    let value = catalog_json_for_format(&catalog, Format::IsdM14Standard);

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
fn format_deck_registries_resolve_without_cross_format_leakage() {
    assert_eq!(deck_names(), deck_names_for_format(Format::OldSchool9394));
    assert_eq!(deck_names_for_format(Format::OldSchool9394).len(), 15);
    assert_eq!(deck_names_for_format(Format::IsdM14Standard).len(), 10);

    for format in [Format::OldSchool9394, Format::IsdM14Standard] {
        for name in deck_names_for_format(format) {
            assert!(
                deck_by_name_for_format(format, name).is_some(),
                "{name} resolves in {format}"
            );
        }
    }

    assert!(deck_by_name_for_format(Format::OldSchool9394, "Briksza Naya Midrange").is_none());
    assert!(deck_by_name_for_format(Format::IsdM14Standard, "Sligh").is_none());
    assert!(
        deck_by_name_for_format(Format::IsdM14Standard, "naya_midrange_rudy_briksza").is_some()
    );
    assert!(
        deck_by_name_for_format(Format::IsdM14Standard, "naya_midrange_jimmie_smith").is_some()
    );
    assert_eq!(
        parse_format_slug("old_school_93_94"),
        Ok(Format::OldSchool9394)
    );
    assert!(parse_format_slug("isd-dgm-standard").is_err());
    assert!(parse_format_slug("isd_dgm_standard").is_err());
    assert!(parse_format_slug("isd-rtr-standard").is_err());
    assert!(parse_format_slug("isd_rtr_standard").is_err());
    assert_eq!(
        parse_format_slug("isd-m14-standard"),
        Ok(Format::IsdM14Standard)
    );
    assert_eq!(
        parse_format_slug("som_m13_standard"),
        Ok(Format::SomM13Standard)
    );
    assert_eq!(parse_format_slug("vintage-cube"), Ok(Format::VintageCube));
    assert_eq!(parse_format_slug("pauper_cube"), Ok(Format::PauperCube));
    assert!(parse_format_slug("vintage").is_err());
}

#[test]
fn bot_game_stores_and_emits_its_format_and_rejects_wrong_decks() {
    let old_school = BotGame::new("Sligh", "Goblins", Opponent::External, PlayerId::Two, 18)
        .expect("compatibility constructor starts Old School");
    assert_eq!(old_school.format(), Format::OldSchool9394);

    let standard = BotGame::new_with_format(
        Format::IsdM14Standard,
        "Briksza Naya Midrange",
        "Greer G/R Aggro",
        Opponent::External,
        PlayerId::Two,
        19,
    )
    .expect("Standard game starts");
    assert_eq!(standard.format(), Format::IsdM14Standard);
    let seat = standard.decision_seat().expect("opening-hand decision");
    let observation: Value =
        serde_json::from_str(&standard.observe_json(seat)).expect("valid observation JSON");
    assert_eq!(observation["protocolVersion"], PROTOCOL_VERSION);
    assert_eq!(observation["format"], "isd-m14-standard");
    assert!(
        observation["legalActions"]
            .as_array()
            .expect("actions")
            .iter()
            .all(|action| action["type"] != "Concede")
    );

    let m14 = BotGame::from_config_json(
        r#"{"format":"isd-m14-standard","p1Deck":"Lorren U/W Flash","p2Deck":"Arch U/W Flash","opponent":"external","seed":5}"#,
    )
    .expect("M14 format accepts M14-dependent decks");
    assert_eq!(m14.format(), Format::IsdM14Standard);
    assert!(
        BotGame::from_config_json(
            r#"{"format":"isd-dgm-standard","p1Deck":"Smith Naya Midrange","p2Deck":"Braun-Duin Naya Midrange"}"#,
        )
        .err()
        .expect("removed format is rejected")
        .contains("unknown format: isd-dgm-standard")
    );
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
            Format::IsdM14Standard,
            "Sligh",
            "Briksza Naya Midrange",
            Opponent::External,
            PlayerId::Two,
            0,
        )
        .err()
        .expect("cross-format deck is rejected")
        .contains("unknown deck for isd-m14-standard")
    );
}

#[test]
fn catalog_json_is_structured_and_legality_is_format_specific() {
    let catalog = poc::catalog().expect("catalog builds");
    let old_school = catalog_json(&catalog);
    let standard = catalog_json_for_format(&catalog, Format::IsdM14Standard);
    assert_eq!(old_school["format"], "old-school-93-94");
    assert_eq!(standard["format"], "isd-m14-standard");

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
    assert_eq!(find("Darksteel Ingot")["legal"], true);
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
