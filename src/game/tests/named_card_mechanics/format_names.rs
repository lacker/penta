use super::*;
use crate::card::CardNameSetDef;
use std::collections::BTreeSet;

#[test]
fn needle_names_only_the_format_pool_including_reprints_and_both_faces() {
    let mut game = ready_game();
    game.format = Format::IsdM14Standard;
    game.put_onto_battlefield(PlayerId::One, cards::PITHING_NEEDLE)
        .unwrap();
    let decision = pending_choice(&game, PlayerId::One);
    let names: BTreeSet<_> = decision
        .options
        .iter()
        .map(|option| option.label.as_str())
        .collect();
    for name in [
        "Pithing Needle",
        "Island",
        "Delver of Secrets",
        "Insectile Aberration",
    ] {
        assert!(
            names.contains(name),
            "{name} is nameable through a legal printing"
        );
    }
    for name in [
        "Lightning Bolt",
        "Badlands",
        "Delver of Secrets // Insectile Aberration",
    ] {
        assert!(
            !names.contains(name),
            "{name} is not a nameable part in this format"
        );
    }
    choose_label(&mut game, PlayerId::One, "Insectile Aberration");
    drain_pending(&mut game);
    assert!(game.battlefield.iter().any(|permanent| {
        permanent.chosen_card_name.as_deref() == Some("Insectile Aberration")
    }));
}

#[test]
fn every_catalog_name_set_applies_format_and_part_restrictions() {
    use CardNameSetDef::*;
    let mut game = ready_game();
    game.format = Format::IsdM14Standard;
    // A nonland, nonbasic land, and basic land that all belong to this format.
    for (set, expected) in [
        (AllCardNames, [true, true, true]),
        (NonlandCardNames, [true, false, false]),
        (LandCardNames, [false, true, true]),
        (NonbasicLandCardNames, [false, true, false]),
        (CardNamesOtherThanBasicLands, [true, true, false]),
        (BasicLandNames, [false, false, true]),
        (
            Union(&[NonlandCardNames, NonbasicLandCardNames]),
            [true, true, false],
        ),
    ] {
        let names = game.catalog_card_names(set).unwrap();
        for (name, present) in ["Pithing Needle", "Rootbound Crag", "Island"]
            .into_iter()
            .zip(expected)
        {
            assert_eq!(names.contains(name), present, "{set:?}: {name}");
        }
        assert!(!names.contains("Lightning Bolt"), "{set:?}");
        assert!(!names.contains("Badlands"), "{set:?}");
    }
}

#[test]
fn name_pools_follow_bans_restrictions_cube_membership_and_commander_policy() {
    let mut game = ready_game();
    for (format, included, excluded) in [
        (Format::Legacy, "Lightning Bolt", "Black Lotus"),
        (Format::Vintage, "Black Lotus", "Contract from Below"),
        (Format::VintageCube, "Ancestral Recall", "Sorrow's Path"),
        (Format::Cedh, "Sol Ring", "Mana Crypt"),
        (Format::DuelCommander, "Lightning Bolt", "Sol Ring"),
    ] {
        game.format = format;
        let names = game
            .catalog_card_names(CardNameSetDef::AllCardNames)
            .unwrap();
        assert!(names.contains(included), "{format}: {included}");
        assert!(!names.contains(excluded), "{format}: {excluded}");
        assert!(
            names.contains("Island"),
            "{format}: basic lands remain nameable"
        );
    }
}

#[test]
fn resolving_name_choices_use_the_format_even_when_other_names_are_in_hand() {
    let mut game = ready_game();
    game.format = Format::Premodern;
    game.players[PlayerId::Two.index()].hand.push(card(
        10_100,
        cards::PITHING_NEEDLE,
        PlayerId::Two,
    ));
    let therapy = card(10_101, cards::CABAL_THERAPY, PlayerId::One);
    let therapy_id = therapy.id;
    game.players[PlayerId::One.index()].hand.push(therapy);
    game.players[PlayerId::One.index()].mana_pool.black = 1;
    game.apply(
        PlayerId::One,
        cast_action(
            therapy_id,
            vec![Target::Player(PlayerId::Two)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    pass_until_decision(&mut game);
    let decision = pending_choice(&game, PlayerId::One);
    assert!(
        decision
            .options
            .iter()
            .any(|option| option.label == "Lightning Bolt")
    );
    for excluded in ["Pithing Needle", "Island", "Necropotence"] {
        assert!(
            !decision
                .options
                .iter()
                .any(|option| option.label == excluded)
        );
    }
    choose_label(&mut game, PlayerId::One, "Lightning Bolt");
    drain_pending(&mut game);
    assert_eq!(game.players[PlayerId::Two.index()].hand.len(), 1);
}

#[test]
fn object_derived_names_are_not_restricted_to_the_format_pool() {
    const OBJECTS: ObjectSetDef = ObjectSetDef::Query(crate::card::ObjectQueryDef::new(
        ObjectPredicateDef::Source,
        &[ZoneKind::Battlefield],
    ));
    let mut game = ready_game();
    let source = game
        .put_onto_battlefield(PlayerId::One, cards::DEATHRITE_SHAMAN)
        .unwrap();
    assert!(
        !game
            .catalog_card_names(CardNameSetDef::AllCardNames)
            .unwrap()
            .contains("Deathrite Shaman")
    );
    for set in [
        CardNameSetDef::NamesOf(&OBJECTS),
        CardNameSetDef::NamesAppearingAtLeast {
            objects: &OBJECTS,
            count: 1,
        },
    ] {
        assert_eq!(
            game.source_card_name_set(set, source),
            BTreeSet::from(["Deathrite Shaman".to_owned()])
        );
    }
}
