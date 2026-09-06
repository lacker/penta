use super::*;

#[test]
fn painlands_offer_safe_colorless_or_colored_mana_with_immediate_damage() {
    for (definition, colors) in [
        (cards::ADARKAR_WASTES, [ManaColor::White, ManaColor::Blue]),
        (cards::KARPLUSAN_FOREST, [ManaColor::Red, ManaColor::Green]),
        (
            cards::UNDERGROUND_RIVER,
            [ManaColor::Blue, ManaColor::Black],
        ),
        (cards::CAVES_OF_KOILOS, [ManaColor::White, ManaColor::Black]),
        (cards::LLANOWAR_WASTES, [ManaColor::Black, ManaColor::Green]),
        (cards::YAVIMAYA_COAST, [ManaColor::Green, ManaColor::Blue]),
        (cards::BATTLEFIELD_FORGE, [ManaColor::Red, ManaColor::White]),
        (cards::SHIVAN_REEF, [ManaColor::Blue, ManaColor::Red]),
        // The Tempest cycle prints the same two abilities and adds a turn of
        // delay, which does not change what either ability costs.
        (cards::CALDERA_LAKE, [ManaColor::Blue, ManaColor::Red]),
        (cards::PINE_BARRENS, [ManaColor::Black, ManaColor::Green]),
        (cards::SALT_FLATS, [ManaColor::White, ManaColor::Black]),
        (cards::SCABLAND, [ManaColor::Red, ManaColor::White]),
        (cards::SKYSHROUD_FOREST, [ManaColor::Green, ManaColor::Blue]),
    ] {
        let mut game = ready_game();
        let land = creature(10_000, definition, PlayerId::One);
        let source = land.card.id;
        game.battlefield.push(land);
        game.apply(
            PlayerId::One,
            Action::ActivateManaAbility {
                source,
                ability: mana_ability_for(&game, source, ManaColor::Colorless),
                color: ManaColor::Colorless,
                counters_removed: None,
                cost_object: None,
                combination: None,
                triggered_mana: None,
            },
        )
        .unwrap();
        assert_eq!(game.players[PlayerId::One.index()].life, 20);
        assert_eq!(game.players[PlayerId::One.index()].mana_pool.colorless, 1);

        for color in colors {
            let mut game = ready_game();
            let land = creature(10_000, definition, PlayerId::One);
            let source = land.card.id;
            game.battlefield.push(land);
            game.apply(
                PlayerId::One,
                Action::ActivateManaAbility {
                    source,
                    ability: mana_ability_for(&game, source, color),
                    color,
                    counters_removed: None,
                    cost_object: None,
                    combination: None,
                    triggered_mana: None,
                },
            )
            .unwrap();
            assert_eq!(game.players[PlayerId::One.index()].life, 19);
            assert_eq!(
                game.players[PlayerId::One.index()].mana_pool.amount(color),
                1
            );
            assert!(game.stack.is_empty(), "mana abilities do not use the stack");
        }
    }
}

#[test]
fn ancient_tomb_adds_two_colorless_and_deals_two_damage_immediately() {
    let mut game = ready_game();
    let tomb = creature(10_000, cards::ANCIENT_TOMB, PlayerId::One);
    let source = tomb.card.id;
    game.battlefield.push(tomb);

    game.apply(
        PlayerId::One,
        Action::ActivateManaAbility {
            source,
            ability: mana_ability_for(&game, source, ManaColor::Colorless),
            color: ManaColor::Colorless,
            counters_removed: None,
            cost_object: None,
            combination: None,
            triggered_mana: None,
        },
    )
    .unwrap();

    assert_eq!(game.players[PlayerId::One.index()].mana_pool.colorless, 2);
    assert_eq!(game.players[PlayerId::One.index()].life, 18);
    assert!(game.stack.is_empty());
}

#[test]
fn wasteland_sacrifices_to_destroy_a_nonbasic_but_cannot_target_a_basic() {
    let mut game = ready_game();
    let wasteland = creature(10_000, cards::WASTELAND, PlayerId::One);
    let source = wasteland.card.id;
    let basic = creature(10_001, cards::MOUNTAIN, PlayerId::Two);
    let basic_id = basic.card.id;
    let nonbasic = creature(10_002, cards::COASTAL_TOWER, PlayerId::Two);
    let nonbasic_id = nonbasic.card.id;
    game.battlefield.extend([wasteland, basic, nonbasic]);
    let ability = activated_ability_for(&game, source, 0);
    let target_basic = Action::ActivateAbility {
        source,
        ability,
        targets: activated_targets(Target::Permanent(basic_id)),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
        mana_payment: None,
    };
    let target_nonbasic = Action::ActivateAbility {
        source,
        ability,
        targets: activated_targets(Target::Permanent(nonbasic_id)),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
        mana_payment: None,
    };

    assert!(!game.legal_actions(PlayerId::One).contains(&target_basic));
    assert!(game.legal_actions(PlayerId::One).contains(&target_nonbasic));
    game.apply(PlayerId::One, target_nonbasic).unwrap();
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != source),
        "Wasteland is sacrificed as a cost"
    );
    pass_priority_pair(&mut game);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != nonbasic_id)
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == basic_id)
    );
}

#[test]
fn dust_bowl_sacrifices_a_land_and_rishadan_port_taps_one() {
    let mut game = ready_game();
    let bowl = creature(10_000, cards::DUST_BOWL, PlayerId::One);
    let bowl_id = bowl.card.id;
    let fodder = creature(10_001, cards::MOUNTAIN, PlayerId::One);
    let fodder_id = fodder.card.id;
    let target = creature(10_002, cards::WASTELAND, PlayerId::Two);
    let target_id = target.card.id;
    game.battlefield.extend([bowl, fodder, target]);
    game.players[PlayerId::One.index()].mana_pool.colorless = 3;
    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: bowl_id,
            ability: activated_ability_for(&game, bowl_id, 0),
            targets: activated_targets(Target::Permanent(target_id)),
            cost_objects: vec![fodder_id],
            x: 0,
            modes: Vec::new(),
            mana_payment: None,
        },
    )
    .unwrap();
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != fodder_id),
        "the chosen land is sacrificed before anyone can respond"
    );
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == bowl_id)
            .expect("Dust Bowl remains")
            .tapped
    );
    pass_priority_pair(&mut game);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != target_id)
    );

    let port = creature(10_003, cards::RISHADAN_PORT, PlayerId::One);
    let port_id = port.card.id;
    let land = creature(10_004, cards::ISLAND, PlayerId::Two);
    let land_id = land.card.id;
    game.battlefield.extend([port, land]);
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;
    game.priority = PlayerId::One;
    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: port_id,
            ability: activated_ability_for(&game, port_id, 0),
            targets: activated_targets(Target::Permanent(land_id)),
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
            mana_payment: None,
        },
    )
    .unwrap();
    pass_priority_pair(&mut game);
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == land_id)
            .expect("the Island remains")
            .tapped
    );
}

#[test]
fn coastal_tower_enters_tapped_and_fetchlands_find_only_their_land_types() {
    let mut game = ready_game();
    let tower = game
        .put_onto_battlefield(PlayerId::One, cards::COASTAL_TOWER)
        .expect("cataloged");
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == tower)
            .expect("Coastal Tower entered")
            .tapped
    );

    for (fetch, eligible, ineligible) in [
        (cards::FLOODED_STRAND, cards::PLAINS, cards::MOUNTAIN),
        (cards::WOODED_FOOTHILLS, cards::FOREST, cards::ISLAND),
    ] {
        let mut game = ready_game();
        game.players[PlayerId::One.index()].library.clear();
        stack_library(
            &mut game,
            &[
                (10_001, eligible),
                (10_002, ineligible),
                (10_003, cards::SWAMP),
            ],
        );
        let fetch = creature(10_000, fetch, PlayerId::One);
        let source = fetch.card.id;
        game.battlefield.push(fetch);
        game.apply(
            PlayerId::One,
            Action::ActivateAbility {
                source,
                ability: activated_ability_for(&game, source, 0),
                targets: Vec::new(),
                cost_objects: Vec::new(),
                x: 0,
                modes: Vec::new(),
                mana_payment: None,
            },
        )
        .unwrap();
        assert_eq!(game.players[PlayerId::One.index()].life, 19);
        assert!(
            game.battlefield
                .iter()
                .all(|permanent| permanent.card.id != source)
        );
        pass_priority_pair(&mut game);

        let decision = game
            .observe(PlayerId::One)
            .decision
            .expect("the fetchland offers its matching library cards");
        let offered = decision
            .options
            .iter()
            .filter_map(|option| {
                option
                    .card
                    .and_then(|(_, characteristics)| characteristics.card_definition())
            })
            .collect::<Vec<_>>();
        assert!(offered.contains(&eligible));
        assert!(!offered.contains(&ineligible));
        let chosen = decision
            .options
            .iter()
            .find(|option| {
                option.card.is_some_and(|(_, characteristics)| {
                    characteristics.card_definition() == Some(eligible)
                })
            })
            .expect("the matching land is selectable")
            .id;
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: decision.id,
                options: vec![chosen],
            },
        )
        .unwrap();
        assert!(
            game.battlefield
                .iter()
                .any(|permanent| permanent.card.definition == eligible)
        );
    }
}

/// The Tempest painlands print a third ability the Apocalypse cycle does
/// not, and Grand Coliseum prints all three plus a colour choice. What is
/// worth separating is that the delay is a replacement on entry, not a cost
/// of either mana ability: neither is more expensive for it.
#[test]
fn the_slow_painlands_arrive_tapped() {
    for definition in [
        cards::CALDERA_LAKE,
        cards::PINE_BARRENS,
        cards::SALT_FLATS,
        cards::SCABLAND,
        cards::SKYSHROUD_FOREST,
        cards::GRAND_COLISEUM,
    ] {
        let mut game = ready_game();
        game.battlefield.clear();
        game.players[0].hand.clear();
        game.players[0]
            .hand
            .push(card(10_900, definition, PlayerId::One));
        game.apply(
            PlayerId::One,
            Action::PlayLand {
                card: CardInstanceId(10_900),
                option: PlayOptionId::DEFAULT,
            },
        )
        .expect("a land drop is available");

        let land = game
            .battlefield
            .last()
            .expect("the land is on the battlefield");
        assert!(land.tapped, "it arrived tapped");
        assert_eq!(game.players[0].life, 20, "and cost no life to play");
    }
}

/// Grand Coliseum's coloured ability makes any colour rather than one of
/// two, and still costs exactly one life.
#[test]
fn grand_coliseum_pays_one_life_for_any_color() {
    for color in [
        ManaColor::White,
        ManaColor::Blue,
        ManaColor::Black,
        ManaColor::Red,
        ManaColor::Green,
    ] {
        let mut game = ready_game();
        let coliseum = creature(10_910, cards::GRAND_COLISEUM, PlayerId::One);
        let source = coliseum.card.id;
        game.battlefield.push(coliseum);
        game.apply(
            PlayerId::One,
            Action::ActivateManaAbility {
                source,
                ability: mana_ability_for(&game, source, color),
                color,
                counters_removed: None,
                cost_object: None,
                combination: None,
                triggered_mana: None,
            },
        )
        .unwrap();
        assert_eq!(game.players[PlayerId::One.index()].life, 19);
        assert_eq!(
            game.players[PlayerId::One.index()].mana_pool.amount(color),
            1
        );
    }
}
