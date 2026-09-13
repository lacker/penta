use super::*;

#[test]
fn cedh_opening_snapshot_keeps_command_zones_public_and_out_of_hand() {
    let decks = penta::protocol::deck_names_for_format(penta::Format::Cedh);
    assert_eq!(decks.len(), 16, "the top 16 cEDH lists are registered");
    let deck = decks.first().expect("cEDH has an imported deck");
    let game = WebGame::new(deck, deck, "Handcrafted", true, 9_031, Some("cedh".into()))
        .expect("cEDH game starts");
    let snapshot = game.snapshot_value(false);

    assert_eq!(snapshot["format"], "cedh");
    assert_eq!(snapshot["human"]["life"], 40);
    assert_eq!(snapshot["opponent"]["life"], 40);
    let commanders = snapshot["commanders"]
        .as_array()
        .expect("commander history is public");
    assert!(!commanders.is_empty());

    for owner in ["human", "opponent"] {
        let command_zone = snapshot[owner]["commandZone"]
            .as_array()
            .expect("command zone is a card array");
        let owned = commanders
            .iter()
            .filter(|commander| commander["owner"] == owner)
            .collect::<Vec<_>>();
        assert_eq!(command_zone.len(), owned.len(), "{owner} command zone");
        for commander in owned {
            let card = command_zone
                .iter()
                .find(|card| card["definition"] == commander["definition"])
                .expect("designated commander remains a command-zone card");
            assert_eq!(commander["object"], card["id"]);
            assert_eq!(commander["commandZoneCasts"], 0);
            assert_eq!(
                commander["combatDamage"],
                json!({ "human": 0, "opponent": 0 })
            );
            assert!(
                commander["name"]
                    .as_str()
                    .is_some_and(|name| !name.is_empty())
            );
        }
    }

    let human_hand = snapshot["human"]["hand"]
        .as_array()
        .expect("the browser player's hand is visible");
    assert!(
        snapshot["human"]["commandZone"]
            .as_array()
            .expect("human command zone")
            .iter()
            .all(|commander| !human_hand.iter().any(|card| card["id"] == commander["id"]))
    );
}

#[test]
fn standard_opening_snapshot_has_empty_commander_contract_fields() {
    let game = WebGame::new(
        "Briksza Naya Midrange",
        "Greer G/R Aggro",
        "Handcrafted",
        true,
        2_013,
        Some("isd-m14-standard".into()),
    )
    .expect("Standard game starts");
    let snapshot = game.snapshot_value(false);

    assert_eq!(snapshot["human"]["commandZone"], json!([]));
    assert_eq!(snapshot["opponent"]["commandZone"], json!([]));
    assert_eq!(snapshot["commanders"], json!([]));
}
