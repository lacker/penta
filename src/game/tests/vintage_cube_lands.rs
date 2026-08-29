//! Lands cataloged for the Vintage Cube pool.
//!
//! A triome is three basic land types, a tapped entry, and cycling. Nothing
//! in it is new, so what these check is the composition: that the subtypes
//! really do produce all three colors without a printed mana clause, that the
//! land arrives tapped, and that the cycling ability is the one the card
//! prints rather than the one its subtypes might suggest.

use super::*;

/// Every triome, with the three colors its subtypes grant.
const TRIOMES: [(CardDefinitionId, [ManaColor; 3]); 10] = [
    (
        cards::INDATHA_TRIOME,
        [ManaColor::White, ManaColor::Black, ManaColor::Green],
    ),
    (
        cards::KETRIA_TRIOME,
        [ManaColor::Green, ManaColor::Blue, ManaColor::Red],
    ),
    (
        cards::RAUGRIN_TRIOME,
        [ManaColor::Blue, ManaColor::Red, ManaColor::White],
    ),
    (
        cards::SAVAI_TRIOME,
        [ManaColor::Red, ManaColor::White, ManaColor::Black],
    ),
    (
        cards::ZAGOTH_TRIOME,
        [ManaColor::Black, ManaColor::Green, ManaColor::Blue],
    ),
    (
        cards::JETMIRS_GARDEN,
        [ManaColor::Red, ManaColor::Green, ManaColor::White],
    ),
    (
        cards::RAFFINES_TOWER,
        [ManaColor::White, ManaColor::Blue, ManaColor::Black],
    ),
    (
        cards::SPARAS_HEADQUARTERS,
        [ManaColor::Green, ManaColor::White, ManaColor::Blue],
    ),
    (
        cards::XANDERS_LOUNGE,
        [ManaColor::Blue, ManaColor::Black, ManaColor::Red],
    ),
    (
        cards::ZIATORAS_PROVING_GROUND,
        [ManaColor::Black, ManaColor::Red, ManaColor::Green],
    ),
];

#[test]
fn every_triome_enters_tapped_and_taps_for_each_of_its_three_colors() {
    for (definition, colors) in TRIOMES {
        let mut game = ready_game();
        let land = game
            .put_onto_battlefield(PlayerId::One, definition)
            .expect("cataloged");
        assert!(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == land)
                .expect("the triome entered")
                .tapped,
            "{definition:?} enters tapped",
        );

        for color in colors {
            let mut game = ready_game();
            let land = game
                .put_onto_battlefield(PlayerId::One, definition)
                .expect("cataloged");
            game.battlefield
                .iter_mut()
                .find(|permanent| permanent.card.id == land)
                .expect("the triome entered")
                .tapped = false;
            game.apply(
                PlayerId::One,
                Action::ActivateManaAbility {
                    source: land,
                    ability: mana_ability_for(&game, land, color),
                    color,
                    counters_removed: None,
                    cost_object: None,
                    combination: None,
                    triggered_mana: None,
                },
            )
            .unwrap_or_else(|error| panic!("{definition:?} makes {color:?}: {error}"));
            assert_eq!(
                game.players[PlayerId::One.index()].mana_pool.amount(color),
                1,
                "{definition:?} taps for {color:?}",
            );
        }
    }
}

#[test]
fn a_triome_cycles_from_hand_for_three_generic() {
    let mut game = ready_game();
    game.players[PlayerId::One.index()].hand.clear();
    let triome = card(41_000, cards::RAFFINES_TOWER, PlayerId::One);
    let triome_id = triome.id;
    game.players[PlayerId::One.index()].hand.push(triome);

    assert!(
        !game.legal_actions(PlayerId::One).iter().any(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == triome_id)
        ),
        "cycling is not offered before the three mana is available",
    );

    game.players[PlayerId::One.index()].mana_pool.colorless = 3;
    let library_before = game.players[PlayerId::One.index()].library.len();
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == triome_id),
        )
        .expect("cycling is offered from hand");
    game.apply(PlayerId::One, action).expect("it is activated");

    assert!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::RAFFINES_TOWER),
        "the discard is a cost",
    );
    pass_priority_pair(&mut game);
    assert_eq!(
        game.players[PlayerId::One.index()].library.len(),
        library_before - 1,
        "and the draw is what resolved",
    );
}

/// Every fastland, with the two colours its printed clause names.
const FAST_LANDS: [(CardDefinitionId, [ManaColor; 2]); 10] = [
    (
        cards::BLACKCLEAVE_CLIFFS,
        [ManaColor::Black, ManaColor::Red],
    ),
    (cards::COPPERLINE_GORGE, [ManaColor::Red, ManaColor::Green]),
    (cards::DARKSLICK_SHORES, [ManaColor::Blue, ManaColor::Black]),
    (
        cards::RAZORVERGE_THICKET,
        [ManaColor::Green, ManaColor::White],
    ),
    (cards::SEACHROME_COAST, [ManaColor::White, ManaColor::Blue]),
    (cards::BLOOMING_MARSH, [ManaColor::Black, ManaColor::Green]),
    (
        cards::BOTANICAL_SANCTUM,
        [ManaColor::Green, ManaColor::Blue],
    ),
    (
        cards::CONCEALED_COURTYARD,
        [ManaColor::White, ManaColor::Black],
    ),
    (cards::INSPIRING_VANTAGE, [ManaColor::Red, ManaColor::White]),
    (cards::SPIREBLUFF_CANAL, [ManaColor::Blue, ManaColor::Red]),
];

/// The clause counts the lands already there, so the boundary sits between a
/// second and a third: a fastland is the fourth land you play, and that is
/// the one that arrives tapped.
#[test]
fn a_fastland_enters_untapped_only_while_the_board_is_small() {
    for (definition, _) in FAST_LANDS {
        for (existing, tapped) in [(0, false), (2, false), (3, true)] {
            let mut game = ready_game();
            game.battlefield.clear();
            for index in 0..existing {
                game.battlefield
                    .push(creature(61_000 + index, cards::FOREST, PlayerId::One));
            }
            // Someone else's lands are not lands you control.
            game.battlefield
                .push(creature(61_100, cards::ISLAND, PlayerId::Two));

            let land = game
                .put_onto_battlefield(PlayerId::One, definition)
                .expect("cataloged");
            assert_eq!(
                game.battlefield
                    .iter()
                    .find(|permanent| permanent.card.id == land)
                    .expect("it entered")
                    .tapped,
                tapped,
                "{definition:?} with {existing} other lands",
            );
        }
    }
}

#[test]
fn every_fastland_taps_for_both_of_its_colors() {
    for (definition, colors) in FAST_LANDS {
        for color in colors {
            let mut game = ready_game();
            game.battlefield.clear();
            let land = game
                .put_onto_battlefield(PlayerId::One, definition)
                .expect("cataloged");
            game.apply(
                PlayerId::One,
                Action::ActivateManaAbility {
                    source: land,
                    ability: mana_ability_for(&game, land, color),
                    color,
                    counters_removed: None,
                    cost_object: None,
                    combination: None,
                    triggered_mana: None,
                },
            )
            .unwrap_or_else(|error| panic!("{definition:?} makes {color:?}: {error}"));
            assert_eq!(
                game.players[PlayerId::One.index()].mana_pool.amount(color),
                1,
            );
        }
    }
}

/// A shock land carries its two basic types like any dual, so a fetch land
/// naming those types finds it -- and putting it onto the battlefield is
/// still an entry, so its own replacement asks about the two life.
#[test]
fn a_fetch_land_finds_a_shock_land_and_the_shock_still_asks() {
    let mut game = ready_game();
    game.battlefield.clear();
    let tarn = game
        .put_onto_battlefield(PlayerId::One, cards::SCALDING_TARN)
        .expect("cataloged");
    game.players[0].library.clear();
    game.players[0]
        .library
        .push(card(62_000, cards::STEAM_VENTS, PlayerId::One));
    drain_pending(&mut game);
    let life = game.players[0].life;

    let fetch = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == tarn))
        .expect("the Tarn is ready to crack");
    game.apply(PlayerId::One, fetch).expect("it activates");
    pass_priority_pair(&mut game);

    let search = game
        .observe(PlayerId::One)
        .decision
        .expect("the search offers what it found");
    assert_eq!(
        search
            .options
            .iter()
            .filter_map(|option| option
                .card
                .and_then(|(_, characteristics)| characteristics.card_definition()))
            .collect::<Vec<_>>(),
        vec![cards::STEAM_VENTS],
        "an Island Mountain answers a search for an Island or Mountain card",
    );
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: search.id,
            options: vec![search.options[0].id],
        },
    )
    .expect("taking it is legal");

    let shock = game
        .observe(PlayerId::One)
        .decision
        .expect("the shock land asks about its own two life");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: shock.id,
            options: vec![1],
        },
    )
    .expect("paying is offered");
    drain_pending(&mut game);

    let vents = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::STEAM_VENTS)
        .expect("it arrived");
    assert!(!vents.tapped, "the two life bought it untapped");
    assert_eq!(
        game.players[0].life,
        life - 3,
        "one for the Tarn and two for the Vents",
    );
}

/// A shock land's ruling: "If an effect puts this land onto the battlefield
/// tapped, you may pay 2 life, but it still enters tapped." The Wight fetches
/// tapped, and paying buys nothing but the two life.
#[test]
fn a_shock_land_fetched_tapped_stays_tapped_however_it_is_paid_for() {
    let mut game = ready_game();
    game.battlefield.clear();
    let wight = game
        .put_onto_battlefield(PlayerId::One, cards::WIGHT_OF_THE_RELIQUARY)
        .expect("cataloged");
    let fodder = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    drain_pending(&mut game);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
        permanent.tapped = false;
    }
    game.players[0].library.clear();
    game.players[0]
        .library
        .push(card(62_100, cards::SACRED_FOUNDRY, PlayerId::One));
    game.turns_started = [5, 5];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    let life = game.players[0].life;

    let fetch = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, cost_objects, .. }
                if *source == wight && cost_objects.contains(&fodder))
        })
        .expect("a spare creature pays for the search");
    game.apply(PlayerId::One, fetch).expect("it activates");
    pass_priority_pair(&mut game);

    let search = game
        .observe(PlayerId::One)
        .decision
        .expect("the search offers what it found");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: search.id,
            options: vec![search.options[0].id],
        },
    )
    .expect("taking it is legal");

    let shock = game
        .observe(PlayerId::One)
        .decision
        .expect("the shock land still asks about its two life");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: shock.id,
            options: vec![1],
        },
    )
    .expect("paying is offered");
    drain_pending(&mut game);

    let foundry = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::SACRED_FOUNDRY)
        .expect("it arrived");
    assert!(
        foundry.tapped,
        "what put it there said tapped, and the two life does not answer that",
    );
    assert_eq!(
        game.players[0].life,
        life - 2,
        "but the payment was made all the same",
    );
}

/// "Two or fewer other lands" counts lands and nothing else: a board of
/// artifacts leaves the Vantage untapped, and a Dryad Arbor counts because
/// it is a land, whatever else it also is.
#[test]
fn the_fastland_clause_counts_lands_and_only_lands() {
    let mut game = ready_game();
    game.battlefield.clear();
    for index in 0..3 {
        game.battlefield
            .push(creature(61_200 + index, cards::SOL_RING, PlayerId::One));
    }
    let vantage = game
        .put_onto_battlefield(PlayerId::One, cards::INSPIRING_VANTAGE)
        .expect("cataloged");
    assert!(
        !game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == vantage)
            .expect("it entered")
            .tapped,
        "three artifacts are not three lands",
    );

    let mut game = ready_game();
    game.battlefield.clear();
    for index in 0..3 {
        game.battlefield
            .push(creature(61_300 + index, cards::DRYAD_ARBOR, PlayerId::One));
    }
    let vantage = game
        .put_onto_battlefield(PlayerId::One, cards::INSPIRING_VANTAGE)
        .expect("cataloged");
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == vantage)
            .expect("it entered")
            .tapped,
        "and a Dryad Arbor is a land, creature or not",
    );
}

/// The other half of the shock land's first ruling: it is not basic, but it
/// has the basic land types, which is what a check land reads. An Isolated
/// Chapel wants a Plains or a Swamp, and a Godless Shrine is both.
#[test]
fn a_shock_land_satisfies_a_check_land() {
    // Nothing on the battlefield: the Chapel arrives tapped.
    let mut game = ready_game();
    game.battlefield.clear();
    let chapel = game
        .put_onto_battlefield(PlayerId::One, cards::ISOLATED_CHAPEL)
        .expect("cataloged");
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == chapel)
            .expect("it entered")
            .tapped,
        "an empty board has neither a Plains nor a Swamp",
    );

    // The same Chapel beside a Godless Shrine, which is both of them.
    let mut game = ready_game();
    game.battlefield.clear();
    game.put_onto_battlefield(PlayerId::One, cards::GODLESS_SHRINE)
        .expect("cataloged");
    drain_pending(&mut game);
    let chapel = game
        .put_onto_battlefield(PlayerId::One, cards::ISOLATED_CHAPEL)
        .expect("cataloged");

    assert!(
        !game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == chapel)
            .expect("it entered")
            .tapped,
        "the Shrine is a Plains and a Swamp, whether or not it is basic",
    );
}

/// Cycling is an activated ability with no timing restriction of its own, so
/// a triome held up is a land on your turn and a card on theirs.
#[test]
fn a_triome_cycles_on_the_other_players_turn() {
    let mut game = ready_game();
    game.players[PlayerId::One.index()].hand.clear();
    let triome = card(41_200, cards::SPARAS_HEADQUARTERS, PlayerId::One);
    let triome_id = triome.id;
    game.players[PlayerId::One.index()].hand.push(triome);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);
    game.active_player = PlayerId::Two;
    game.step = Step::End;
    game.priority = PlayerId::One;
    let library_before = game.players[PlayerId::One.index()].library.len();

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == triome_id)
        })
        .expect("their end step is as good a time as any");
    game.apply(PlayerId::One, action).expect("it is activated");
    drain_pending(&mut game);

    assert_eq!(
        game.players[PlayerId::One.index()].library.len(),
        library_before - 1,
        "the card was drawn",
    );
    assert!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::SPARAS_HEADQUARTERS),
        "and the land it was is in the graveyard",
    );
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::PlayLand { card, .. } if *card == triome_id)),
        "a land you cycled is not a land you played",
    );
}

/// Three basic land types answer more fetchlands than two: the same
/// Headquarters is what a Heath is looking for and what a Rainforest is.
#[test]
fn two_different_fetchlands_both_find_the_same_triome() {
    for fetch in [cards::WINDSWEPT_HEATH, cards::MISTY_RAINFOREST] {
        let mut game = ready_game();
        game.battlefield.clear();
        let source = game
            .put_onto_battlefield(PlayerId::One, fetch)
            .expect("cataloged");
        game.players[PlayerId::One.index()].library.clear();
        game.players[PlayerId::One.index()].library.push(card(
            41_300,
            cards::SPARAS_HEADQUARTERS,
            PlayerId::One,
        ));

        let crack = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| {
                matches!(action, Action::ActivateAbility { source: actual, .. } if *actual == source)
            })
            .expect("the fetch is offered");
        game.apply(PlayerId::One, crack).expect("it activates");
        pass_priority_pair(&mut game);
        drain_pending(&mut game);

        let found = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.definition == cards::SPARAS_HEADQUARTERS)
            .unwrap_or_else(|| panic!("{fetch:?} found the Headquarters"));
        assert!(found.tapped, "and it arrives tapped, as it says");
    }
}

/// "Land cards not on the battlefield aren't Forests while Yavimaya is on
/// the battlefield", and "Yavimaya isn't a Forest while it's not on the
/// battlefield" either. A fetchland reads the library, where the Cradle
/// changes nothing at all.
#[test]
fn yavimaya_makes_forests_of_nothing_in_the_library() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.battlefield.push(creature(
        41_400,
        cards::YAVIMAYA_CRADLE_OF_GROWTH,
        PlayerId::One,
    ));
    let heath = game
        .put_onto_battlefield(PlayerId::One, cards::WINDSWEPT_HEATH)
        .expect("cataloged");
    game.players[PlayerId::One.index()].library.clear();
    for (index, definition) in [
        cards::ISLAND,
        cards::YAVIMAYA_CRADLE_OF_GROWTH,
        cards::FOREST,
    ]
    .into_iter()
    .enumerate()
    {
        game.players[PlayerId::One.index()].library.push(card(
            41_410 + u32::try_from(index).expect("three cards"),
            definition,
            PlayerId::One,
        ));
    }

    let crack = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == heath))
        .expect("the fetch is offered");
    game.apply(PlayerId::One, crack).expect("it activates");
    pass_priority_pair(&mut game);

    let offered = game
        .pending_decisions
        .first()
        .expect("the search asks")
        .observation
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
        "the Island is still an Island in the library, and the Cradle is no Forest itself",
    );
}

/// Every shock land, with the two colours its basic land types make. The
/// cycle is one card printed ten ways; what is worth checking per member is
/// which pair it carries.
const SHOCK_LANDS: [(CardDefinitionId, [ManaColor; 2]); 10] = [
    (cards::BLOOD_CRYPT, [ManaColor::Black, ManaColor::Red]),
    (cards::BREEDING_POOL, [ManaColor::Green, ManaColor::Blue]),
    (cards::GODLESS_SHRINE, [ManaColor::White, ManaColor::Black]),
    (
        cards::HALLOWED_FOUNTAIN,
        [ManaColor::White, ManaColor::Blue],
    ),
    (cards::OVERGROWN_TOMB, [ManaColor::Black, ManaColor::Green]),
    (cards::SACRED_FOUNDRY, [ManaColor::Red, ManaColor::White]),
    (cards::STEAM_VENTS, [ManaColor::Blue, ManaColor::Red]),
    (cards::STOMPING_GROUND, [ManaColor::Red, ManaColor::Green]),
    (cards::TEMPLE_GARDEN, [ManaColor::Green, ManaColor::White]),
    (cards::WATERY_GRAVE, [ManaColor::Blue, ManaColor::Black]),
];

/// "This has the mana abilities associated with both of its basic land
/// types" -- and only those two.
#[test]
fn every_shock_land_taps_for_its_own_two_colors() {
    for (definition, colors) in SHOCK_LANDS {
        let mut game = ready_game();
        game.battlefield.clear();
        let land = game
            .put_onto_battlefield(PlayerId::One, definition)
            .expect("cataloged");
        drain_pending(&mut game);
        if let Some(permanent) = game
            .battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == land)
        {
            permanent.tapped = false;
        }

        let mut offered = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .filter_map(|action| match action {
                Action::ActivateManaAbility { source, color, .. } if source == land => Some(color),
                _ => None,
            })
            .collect::<Vec<_>>();
        offered.sort_unstable();
        let mut expected = colors.to_vec();
        expected.sort_unstable();
        assert_eq!(offered, expected, "{definition:?} makes its own two");
    }
}

/// A Blood Moon reads lands on the battlefield and nothing else: the Tower
/// in hand still cycles for its printed three, and the one that is played
/// arrives untapped as a plain Mountain.
#[test]
fn a_blood_moon_leaves_cycling_from_hand_alone() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[PlayerId::One.index()].hand.clear();
    game.put_onto_battlefield(PlayerId::One, cards::BLOOD_MOON)
        .expect("cataloged");
    drain_pending(&mut game);
    let held = card(41_500, cards::RAFFINES_TOWER, PlayerId::One);
    let held_id = held.id;
    game.players[PlayerId::One.index()].hand.push(held);
    let played = card(41_501, cards::RAFFINES_TOWER, PlayerId::One);
    let played_id = played.id;
    game.players[PlayerId::One.index()].hand.push(played);
    game.players[PlayerId::One.index()].lands_played_this_turn = 0;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;

    let play = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::PlayLand { card, .. } if *card == played_id))
        .expect("a land drop is available");
    game.apply(PlayerId::One, play).expect("it is playable");
    drain_pending(&mut game);
    let tower = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::RAFFINES_TOWER)
        .expect("it was played");
    assert!(!tower.tapped, "as a Mountain it has no clause to tap it");
    let tower_id = tower.card.id;
    let offered = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::ActivateManaAbility { source, color, .. } if source == tower_id => Some(color),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(offered, vec![ManaColor::Red], "and a Mountain makes red");

    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);
    assert!(
        game.legal_actions(PlayerId::One).iter().any(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == held_id)
        }),
        "the copy in hand is not a land on the battlefield, so it cycles as printed",
    );
}
