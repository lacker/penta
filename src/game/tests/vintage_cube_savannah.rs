//! Savannah: a Forest Plains that is not a basic land. The types are what a
//! fetchland and a mana ability read; the missing supertype is what a
//! Wasteland and a basic-land search read.

use super::*;

/// Player One with a Savannah out and a Forest beside it for contrast.
fn staged() -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let savannah = game
        .put_onto_battlefield(PlayerId::One, cards::SAVANNAH)
        .expect("cataloged");
    let forest = game
        .put_onto_battlefield(PlayerId::One, cards::FOREST)
        .expect("cataloged");
    drain_pending(&mut game);
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.empty_mana_pools();
    (game, savannah, forest)
}

fn colors_of(game: &Game, id: GameObjectId) -> Vec<ManaColor> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::ActivateManaAbility { source, color, .. } if source == id => Some(color),
            _ => None,
        })
        .collect()
}

/// "This has the mana abilities associated with both of its basic land
/// types."
#[test]
fn it_taps_for_both_of_its_types() {
    let (game, savannah, _) = staged();

    assert_eq!(
        colors_of(&game, savannah),
        vec![ManaColor::Green, ManaColor::White],
        "green and white, in printed subtype order",
    );
}

/// "Things that affect basic lands don't affect it." A Wasteland answers
/// nonbasic lands, and having Forest and Plains printed on it does not make
/// the Savannah basic.
#[test]
fn a_wasteland_answers_it_and_not_the_forest() {
    let (mut game, savannah, forest) = staged();
    let wasteland = game
        .put_onto_battlefield(PlayerId::Two, cards::WASTELAND)
        .expect("cataloged");
    drain_pending(&mut game);
    game.priority = PlayerId::Two;

    let named = |game: &Game, land: GameObjectId| {
        game.legal_actions(PlayerId::Two).into_iter().any(|action| {
            matches!(
                &action,
                Action::ActivateAbility { source, targets, .. }
                    if *source == wasteland
                        && targets
                            .iter()
                            .any(|selection| selection.targets() == [Target::Permanent(land)])
            )
        })
    };
    assert!(named(&game, savannah), "a nonbasic land, types and all");
    assert!(!named(&game, forest), "and a Forest is not one");

    let destroy = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility { source, targets, .. }
                    if *source == wasteland
                        && targets
                            .iter()
                            .any(|selection| selection.targets() == [Target::Permanent(savannah)])
            )
        })
        .expect("it may be named");
    game.apply(PlayerId::Two, destroy).expect("it activates");
    drain_pending(&mut game);
    game.check_state_based_actions();

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == savannah),
        "and destroyed",
    );
}

/// The other half of the same ruling: a search that names basic land cards
/// passes it over, however many basic land types it has.
#[test]
fn a_basic_land_search_will_not_find_it() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].library.clear();
    for (index, definition) in [cards::SAVANNAH, cards::FOREST].into_iter().enumerate() {
        game.players[0].library.push(card(
            98_000 + u32::try_from(index).expect("two cards"),
            definition,
            PlayerId::One,
        ));
    }
    let growth = game
        .build_zone(PlayerId::One, &[cards::RAMPANT_GROWTH])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let growth_id = growth.id;
    game.players[0].hand.push(growth);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == growth_id))
        .expect("two mana casts it");
    game.apply(PlayerId::One, cast).expect("it is cast");
    pass_priority_pair(&mut game);

    let decision = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
        .expect("the search asks");
    let offered = decision
        .options
        .iter()
        .filter_map(|option| match option.card {
            Some((_, ObjectCharacteristics::Card { definition, .. })) => Some(definition),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        offered,
        vec![cards::FOREST],
        "the Forest is a basic land card and the Savannah is not",
    );
}

/// The other nine of the cycle, each with the pair its basic land types
/// make: one card printed ten ways, and what is worth checking per member
/// is which two colours it answers for.
const ORIGINAL_DUALS: [(CardDefinitionId, [ManaColor; 2]); 10] = [
    (cards::BADLANDS, [ManaColor::Black, ManaColor::Red]),
    (cards::BAYOU, [ManaColor::Black, ManaColor::Green]),
    (cards::PLATEAU, [ManaColor::Red, ManaColor::White]),
    (cards::SAVANNAH, [ManaColor::Green, ManaColor::White]),
    (cards::SCRUBLAND, [ManaColor::White, ManaColor::Black]),
    (cards::TAIGA, [ManaColor::Red, ManaColor::Green]),
    (cards::TROPICAL_ISLAND, [ManaColor::Green, ManaColor::Blue]),
    (cards::TUNDRA, [ManaColor::White, ManaColor::Blue]),
    (cards::UNDERGROUND_SEA, [ManaColor::Blue, ManaColor::Black]),
    (cards::VOLCANIC_ISLAND, [ManaColor::Blue, ManaColor::Red]),
];

/// "This has the mana abilities associated with both of its basic land
/// types" -- and only those two, for every member of the cycle.
#[test]
fn every_original_dual_taps_for_its_own_two() {
    for (definition, colors) in ORIGINAL_DUALS {
        let mut game = ready_game();
        game.battlefield.clear();
        let land = game
            .put_onto_battlefield(PlayerId::One, definition)
            .expect("cataloged");
        drain_pending(&mut game);
        game.active_player = PlayerId::One;
        game.step = Step::PrecombatMain;
        game.priority = PlayerId::One;

        let mut offered = colors_of(&game, land);
        offered.sort_unstable();
        let mut expected = colors.to_vec();
        expected.sort_unstable();
        assert_eq!(offered, expected, "{definition:?} makes its own two");
        assert!(
            !game
                .battlefield
                .iter()
                .find(|permanent| permanent.card.id == land)
                .expect("it entered")
                .tapped,
            "{definition:?} asks nothing on the way in",
        );
    }
}

/// "The mana abilities associated with both of its basic land types" are two
/// abilities on one land, and they share its tap: a Scrubland makes one mana
/// a turn, of whichever colour you asked for, and then offers nothing.
#[test]
fn tapping_a_dual_for_one_colour_spends_the_other() {
    let mut game = ready_game();
    game.battlefield.clear();
    let land = game
        .put_onto_battlefield(PlayerId::One, cards::SCRUBLAND)
        .expect("cataloged");
    drain_pending(&mut game);
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    assert_eq!(colors_of(&game, land).len(), 2, "white and black on offer");

    game.apply(
        PlayerId::One,
        Action::ActivateManaAbility {
            source: land,
            ability: mana_ability_for(&game, land, ManaColor::White),
            color: ManaColor::White,
            counters_removed: None,
            cost_object: None,
            combination: None,
            triggered_mana: None,
        },
    )
    .expect("it taps for white");

    assert_eq!(game.players[0].mana_pool.white, 1);
    assert_eq!(
        game.players[0].mana_pool.black, 0,
        "the other half was never paid for",
    );
    assert!(
        colors_of(&game, land).is_empty(),
        "and a tapped land offers neither colour",
    );
}

/// "Land type changing effects that change a dual land's land type will
/// remove the old land types completely." A Blood Moon makes every nonbasic
/// land a Mountain, and a Badlands keeps neither of its own types.
#[test]
fn a_blood_moon_takes_both_of_a_duals_types() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let badlands = game
        .put_onto_battlefield(PlayerId::One, cards::BADLANDS)
        .expect("cataloged");
    game.battlefield
        .push(creature(64_000, cards::BLOOD_MOON, PlayerId::One));
    drain_pending(&mut game);
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.empty_mana_pools();

    let land = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == badlands)
        .expect("it is on the battlefield");
    assert_eq!(
        game.effective_subtypes(land).as_ref(),
        &["Mountain"],
        "the Swamp went with the rest of what it was",
    );
    assert_eq!(
        colors_of(&game, badlands),
        vec![ManaColor::Red],
        "so it makes red and nothing else",
    );
}

/// "Text-changing effects that just change one of the two land types will
/// leave the other type unaffected." A Magical Hack turning its Swamp into
/// an Island leaves the Mountain where it was.
#[test]
fn magical_hack_changes_one_of_a_duals_types_and_leaves_the_other() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let badlands = game
        .put_onto_battlefield(PlayerId::One, cards::BADLANDS)
        .expect("cataloged");
    drain_pending(&mut game);
    let hack = card(64_100, cards::MAGICAL_HACK, PlayerId::One);
    let hack_id = hack.id;
    game.players[0].hand.push(hack);
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.empty_mana_pools();
    game.players[0].mana_pool.blue = 1;

    game.apply(
        PlayerId::One,
        cast_action(hack_id, vec![Target::Permanent(badlands)], Vec::new(), 0),
    )
    .expect("a Badlands has land types to rewrite");
    pass_priority_pair(&mut game);
    choose_decision_by_label(&mut game, PlayerId::One, "Swamp → Island");

    let land = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == badlands)
        .expect("it is on the battlefield");
    assert_eq!(
        game.effective_subtypes(land).as_ref(),
        &["Island", "Mountain"],
        "one word was rewritten and the other was not",
    );
    let mut colors = colors_of(&game, badlands);
    colors.sort_by_key(|color| format!("{color:?}"));
    assert_eq!(
        colors,
        vec![ManaColor::Blue, ManaColor::Red],
        "and the black went with the Swamp",
    );
}

/// Domain counts basic land types among the lands you control, and a dual
/// carries two of them on one card. Three lands here are five types, which
/// is a Leyline Binding for one white -- where five basics would be five
/// lands.
#[test]
fn a_dual_counts_twice_for_domain() {
    let staged_with = |lands: &[CardDefinitionId]| {
        let mut game = ready_game();
        game.battlefield.clear();
        game.players[0].hand.clear();
        for definition in lands {
            game.put_onto_battlefield(PlayerId::One, *definition)
                .expect("cataloged");
        }
        // Tapped, so what is castable is decided by the mana in the pool
        // rather than by what the lands could still make.
        for permanent in &mut game.battlefield {
            permanent.tapped = true;
        }
        drain_pending(&mut game);
        let binding = game
            .build_zone(PlayerId::One, &[cards::LEYLINE_BINDING])
            .expect("cataloged")
            .into_iter()
            .next()
            .expect("one card");
        let id = binding.id;
        game.players[0].hand.push(binding);
        game.turns_started = [5, 5];
        game.turn = 9;
        game.active_player = PlayerId::One;
        game.step = Step::PrecombatMain;
        game.priority = PlayerId::One;
        (game, id)
    };
    let castable = |game: &Game, card: GameObjectId| {
        game.legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::CastSpell { card: id, .. } if *id == card))
    };

    // One dual is two types, so four of the six mana remain to be paid.
    let (mut game, binding) = staged_with(&[cards::TROPICAL_ISLAND]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 3);
    assert!(!castable(&game, binding), "three mana is not enough");
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 1);
    assert!(
        castable(&game, binding),
        "a Forest Island is two of the five by itself",
    );

    // Two duals and a basic are five types across three lands.
    let (mut game, binding) =
        staged_with(&[cards::TROPICAL_ISLAND, cards::SCRUBLAND, cards::MOUNTAIN]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 1);
    assert!(
        castable(&game, binding),
        "Forest Island, Swamp Plains and a Mountain is the whole domain",
    );
}

/// "This has the mana abilities associated with both of its basic land
/// types" is not the only thing those types are read for: a card that asks
/// whether you control a Swamp is answered by an Underground Sea, which is
/// an Island Swamp and no basic at all.
#[test]
fn a_dual_answers_a_card_that_asks_for_one_of_its_basic_types() {
    let free_offered = |land: Option<CardDefinitionId>| {
        let mut game = ready_game();
        game.battlefield.clear();
        game.players[PlayerId::One.index()].hand.clear();
        if let Some(land) = land {
            game.put_onto_battlefield(PlayerId::One, land)
                .expect("cataloged");
        }
        game.put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
            .expect("cataloged");
        drain_pending(&mut game);
        let snuff = card(97_500, cards::SNUFF_OUT, PlayerId::One);
        let snuff_id = snuff.id;
        game.players[PlayerId::One.index()].hand.push(snuff);
        game.players[PlayerId::One.index()].life = 20;
        game.empty_mana_pools();
        game.active_player = PlayerId::One;
        game.step = Step::PrecombatMain;
        game.priority = PlayerId::One;
        game.legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == snuff_id))
    };

    assert!(
        !free_offered(None),
        "with no land at all the four life buys nothing",
    );
    assert!(
        !free_offered(Some(cards::SAVANNAH)),
        "and a Forest Plains is not a Swamp",
    );
    assert!(
        free_offered(Some(cards::SWAMP)),
        "a basic Swamp is the ordinary way to pay it",
    );
    assert!(
        free_offered(Some(cards::UNDERGROUND_SEA)),
        "and an Underground Sea is a Swamp for exactly as long as it says so",
    );
}

/// The whole cycle checked above is put onto the battlefield rather than
/// played, which skips the one thing a deck actually asks of these lands:
/// a Scrubland played from hand is an ordinary land drop that spends the
/// drop and makes mana the same turn, where the modern duals beside it in
/// the cube arrive tapped and make none.
#[test]
fn a_dual_played_from_hand_makes_mana_the_turn_it_lands() {
    for (definition, untapped) in [(cards::SCRUBLAND, true), (cards::UNDERCITY_SEWERS, false)] {
        let mut game = ready_game();
        game.battlefield.clear();
        game.players[0].hand.clear();
        let land = game
            .build_zone(PlayerId::One, &[definition])
            .expect("cataloged")
            .into_iter()
            .next()
            .expect("one card");
        let id = land.id;
        game.players[0].hand.push(land);
        game.active_player = PlayerId::One;
        game.step = Step::PrecombatMain;
        game.priority = PlayerId::One;
        game.players[0].lands_played_this_turn = 0;

        let play = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| matches!(action, Action::PlayLand { card, .. } if *card == id))
            .unwrap_or_else(|| panic!("{definition:?} is a land drop"));
        game.apply(PlayerId::One, play).expect("it is played");
        drain_pending(&mut game);

        assert_eq!(
            game.players[0].lands_played_this_turn, 1,
            "{definition:?} spent the drop",
        );
        // The card gets a new identity on its way to the battlefield.
        let permanent = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.definition == definition)
            .unwrap_or_else(|| panic!("{definition:?} arrived"))
            .card
            .id;
        assert_eq!(
            colors_of(&game, permanent).is_empty(),
            !untapped,
            "{definition:?} makes mana the turn it lands: {untapped}",
        );
    }
}

/// "This card is a Mountain and a Swamp even while in the graveyard,
/// library, or any other zone." The types are printed on the card rather
/// than granted to the permanent, which is the whole difference between a
/// dual and an Urborg: what Urborg says reaches the battlefield only, and
/// what a Badlands is, it is everywhere.
#[test]
fn a_dual_keeps_its_types_in_every_zone() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].graveyard.clear();
    let source = game
        .put_onto_battlefield(PlayerId::One, cards::SAVANNAH)
        .expect("cataloged");
    drain_pending(&mut game);
    let is = |game: &Game, card: &CardInstance, zone, subtype| {
        game.card_object_matches(ObjectPredicateDef::Subtype(subtype), card, zone, source)
    };

    let badlands = game
        .build_zone(PlayerId::One, &[cards::BADLANDS])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    for subtype in ["Mountain", "Swamp"] {
        assert!(
            is(&game, &badlands, ZoneKind::Hand, subtype),
            "a Badlands in hand is a {subtype}",
        );
    }
    game.players[0].graveyard.push(badlands);
    let buried = game.players[0].graveyard.last().expect("it is there");
    for subtype in ["Mountain", "Swamp"] {
        assert!(
            is(&game, buried, ZoneKind::Graveyard, subtype),
            "and one in a graveyard is a {subtype} too",
        );
    }
    assert!(
        !is(&game, buried, ZoneKind::Graveyard, "Island"),
        "and neither of them is anything else",
    );
}
