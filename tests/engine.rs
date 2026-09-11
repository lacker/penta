use penta::card::sets;
use penta::card::{CardCatalog, CardDefinition, CardPrinting, CardRules, CardSupertype};
use penta::deck::{Deck, DeckError};
use penta::game::{GameResult, WinReason};
use penta::poc;
use penta::{
    Action, CardDefinitionId, CastChoices, Format, Game, GameError, GameEvent, PlayOptionId,
    PlayerId, Step, Target, TargetSelection, TargetSlotId,
};

fn lightning_bolt(id: CardDefinitionId) -> CardDefinition {
    let catalog = poc::catalog().expect("the built-in catalog is valid");
    let mut definition = catalog
        .get(poc::cards::LIGHTNING_BOLT)
        .expect("the built-in catalog contains Lightning Bolt")
        .clone();
    definition.id = id;
    definition.printings = vec![CardPrinting::new(id, sets::alpha::SET)];
    definition
}

fn catalog() -> CardCatalog {
    CardCatalog::new([
        CardDefinition::new(
            CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000001"),
            "Mountain",
            sets::alpha::SET,
            CardRules::new_land(&["Mountain"]).with_supertype(CardSupertype::Basic),
        ),
        lightning_bolt(CardDefinitionId::from_uuid(
            "00000000-0000-0000-0000-000000000002",
        )),
        CardDefinition::new(
            CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000003"),
            "Black Lotus",
            sets::alpha::SET,
            CardRules::unsupported(),
        ),
        CardDefinition::new(
            CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000004"),
            "Contract from Below",
            sets::alpha::SET,
            CardRules::unsupported(),
        ),
        CardDefinition::new(
            CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000005"),
            "Standard Test Spell",
            sets::innistrad::SET,
            CardRules::unsupported(),
        ),
    ])
    .unwrap()
}

fn valid_deck() -> Deck {
    let mut main = vec![CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000001"); 55];
    main.extend([CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000002"); 4]);
    main.push(CardDefinitionId::from_uuid(
        "00000000-0000-0000-0000-000000000003",
    ));
    Deck {
        main,
        sideboard: Vec::new(),
    }
}

fn game_with_mountain_and_bolt() -> Game {
    let catalog = catalog();
    for seed in 0..1_000 {
        let mut game = Game::new(catalog.clone(), [valid_deck(), valid_deck()], seed).unwrap();
        let hand = &game.observe(PlayerId::One).hand;
        let has_mountain = hand.iter().any(|(_, definition)| {
            *definition == CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000001")
        });
        let has_bolt = hand.iter().any(|(_, definition)| {
            *definition == CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000002")
        });
        if has_mountain && has_bolt {
            keep_both(&mut game);
            return game;
        }
    }
    panic!("expected to find a deterministic seed with Mountain and Lightning Bolt");
}

fn keep_both(game: &mut Game) {
    game.apply(PlayerId::One, Action::KeepHand).unwrap();
    game.apply(PlayerId::Two, Action::KeepHand).unwrap();
}

#[test]
fn decision_player_tracks_pregame_priority_and_turn_based_choices() {
    let mut game = Game::new(catalog(), [valid_deck(), valid_deck()], 0).unwrap();
    assert_eq!(game.decision_player(), Some(PlayerId::One));

    game.apply(PlayerId::One, Action::KeepHand).unwrap();
    assert_eq!(game.decision_player(), Some(PlayerId::Two));
    game.apply(PlayerId::Two, Action::KeepHand).unwrap();
    assert_eq!(game.decision_player(), Some(PlayerId::One));

    pass_priority_pair(&mut game);
    pass_priority_pair(&mut game);
    pass_priority_pair(&mut game);
    assert_eq!(game.observe(PlayerId::One).step, Step::DeclareAttackers);
    assert_eq!(game.decision_player(), Some(PlayerId::One));

    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .unwrap();
    pass_priority_pair(&mut game);
    assert_eq!(game.observe(PlayerId::One).step, Step::EndOfCombat);
    assert_eq!(game.decision_player(), Some(PlayerId::One));

    game.apply(PlayerId::Two, Action::Concede).unwrap();
    assert_eq!(game.decision_player(), None);
}

fn pass_priority_pair(game: &mut Game) {
    let first = game.observe(PlayerId::One).priority;
    game.apply(first, Action::PassPriority).unwrap();
    game.apply(first.opponent(), Action::PassPriority).unwrap();
}

fn activate_red_mana(game: &mut Game, player: PlayerId, source: penta::GameObjectId) {
    let action = game
        .observe(player)
        .legal_actions
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateManaAbility {
                    source: candidate,
                    color: penta::ManaColor::Red,
                    ..
                } if *candidate == source
            )
        })
        .expect("the Mountain exposes its printed red mana ability");
    game.apply(player, action).unwrap();
}

fn advance_to_first_main(game: &mut Game) {
    assert_eq!(game.observe(PlayerId::One).step, Step::Upkeep);
    pass_priority_pair(game);
    assert_eq!(game.observe(PlayerId::One).step, Step::PrecombatMain);
}

#[test]
fn restricted_cards_are_limited_across_deck_and_sideboard() {
    let catalog = catalog();
    let mut main = vec![CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000001"); 58];
    main.extend([CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000003"); 2]);
    let error = Deck {
        main,
        sideboard: Vec::new(),
    }
    .validate(&catalog)
    .unwrap_err();

    assert_eq!(
        error,
        DeckError::TooManyCopies {
            card: "Black Lotus".into(),
            count: 2,
            limit: 1,
        }
    );
}

#[test]
fn banned_cards_are_rejected() {
    let catalog = catalog();
    let mut main = vec![CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000001"); 59];
    main.push(CardDefinitionId::from_uuid(
        "00000000-0000-0000-0000-000000000004",
    ));

    assert_eq!(
        Deck {
            main,
            sideboard: Vec::new(),
        }
        .validate(&catalog)
        .unwrap_err(),
        DeckError::BannedCard("Contract from Below".into())
    );
}

#[test]
fn deck_validation_uses_the_selected_formats_card_pool() {
    let catalog = catalog();
    let mut main = vec![CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000001"); 59];
    main.push(CardDefinitionId::from_uuid(
        "00000000-0000-0000-0000-000000000005",
    ));
    let standard_deck = Deck {
        main,
        sideboard: Vec::new(),
    };

    assert_eq!(
        standard_deck.clone().validate(&catalog).unwrap_err(),
        DeckError::CardNotAllowed {
            card: "Standard Test Spell".into(),
            format: Format::OldSchool9394,
        }
    );
    standard_deck
        .validate_for_format(&catalog, Format::IsdM14Standard)
        .unwrap();

    let mut main = vec![CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000001"); 59];
    main.push(CardDefinitionId::from_uuid(
        "00000000-0000-0000-0000-000000000002",
    ));
    assert_eq!(
        Deck {
            main,
            sideboard: Vec::new(),
        }
        .validate_for_format(&catalog, Format::IsdM14Standard)
        .unwrap_err(),
        DeckError::CardNotAllowed {
            card: "Lightning Bolt".into(),
            format: Format::IsdM14Standard,
        }
    );
}

#[test]
fn deck_validation_uses_reprints_without_splitting_copy_identity() {
    let mountain = CardDefinition::new(
        CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000001"),
        "Mountain",
        sets::alpha::SET,
        CardRules::new_land(&["Mountain"]).with_supertype(CardSupertype::Basic),
    );
    let bolt = lightning_bolt(CardDefinitionId::from_uuid(
        "00000000-0000-0000-0000-000000000002",
    ));
    let catalog = CardCatalog::with_additional_printings(
        [mountain, bolt],
        [
            CardPrinting::new(
                CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000002"),
                sets::magic_2014::SET,
            ),
            CardPrinting::with_variant(
                CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000002"),
                sets::magic_2014::SET,
                1,
            ),
        ],
    )
    .unwrap();

    let legal = Deck {
        main: [
            vec![CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000001"); 56],
            vec![CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000002"); 4],
        ]
        .concat(),
        sideboard: Vec::new(),
    };
    legal
        .validate_for_format(&catalog, Format::IsdM14Standard)
        .unwrap();

    let too_many = Deck {
        main: [
            vec![CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000001"); 55],
            vec![CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000002"); 5],
        ]
        .concat(),
        sideboard: Vec::new(),
    };
    assert_eq!(
        too_many
            .validate_for_format(&catalog, Format::IsdM14Standard)
            .unwrap_err(),
        DeckError::TooManyCopies {
            card: "Lightning Bolt".into(),
            count: 5,
            limit: 4,
        }
    );
    assert_eq!(
        catalog.find_by_name("lightning bolt"),
        Some(CardDefinitionId::from_uuid(
            "00000000-0000-0000-0000-000000000002"
        ))
    );
}

#[test]
fn setup_is_deterministic_and_hides_the_opponents_hand() {
    let catalog = catalog();
    let game_a = Game::new(catalog.clone(), [valid_deck(), valid_deck()], 0xdeca_fbad).unwrap();
    let game_b = Game::new(catalog.clone(), [valid_deck(), valid_deck()], 0xdeca_fbad).unwrap();

    assert_eq!(game_a.format(), Format::OldSchool9394);
    assert_eq!(game_a.observe(PlayerId::One), game_b.observe(PlayerId::One));
    let observation = game_a.observe(PlayerId::One);
    assert_eq!(observation.hand.len(), 7);
    assert_eq!(observation.opponent_hand_size, 7);
    assert_eq!(observation.library_sizes, [53, 53]);
}

#[test]
fn concession_ends_the_game() {
    let catalog = catalog();
    let mut game = Game::new(catalog.clone(), [valid_deck(), valid_deck()], 123).unwrap();

    game.apply(PlayerId::One, Action::Concede).unwrap();

    let observation = game.observe(PlayerId::Two);
    assert_eq!(
        observation.result,
        Some(GameResult::Winner {
            winner: PlayerId::Two,
            reason: WinReason::OpponentConceded,
        })
    );
    assert!(observation.legal_actions.is_empty());
}

#[test]
fn only_the_priority_player_can_take_game_actions() {
    let game = game_with_mountain_and_bolt();

    assert_eq!(game.legal_actions(PlayerId::Two), vec![Action::Concede]);
    assert!(
        game.legal_actions(PlayerId::One)
            .contains(&Action::PassPriority)
    );
}

#[test]
fn first_player_skips_the_entire_first_draw_step() {
    let mut game = game_with_mountain_and_bolt();
    let initial_hand_size = game.observe(PlayerId::One).hand.len();

    pass_priority_pair(&mut game);

    let observation = game.observe(PlayerId::One);
    assert_eq!(observation.step, Step::PrecombatMain);
    assert_eq!(observation.hand.len(), initial_hand_size);
}

#[test]
fn london_mulligan_redraws_seven_then_bottoms_one() {
    let catalog = catalog();
    let mut game = Game::new(catalog, [valid_deck(), valid_deck()], 77).unwrap();

    game.apply(PlayerId::One, Action::TakeMulligan).unwrap();
    assert_eq!(game.observe(PlayerId::One).hand.len(), 7);
    game.apply(PlayerId::One, Action::KeepHand).unwrap();

    let bottom = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::BottomCards { .. }))
        .unwrap();
    game.apply(PlayerId::One, bottom).unwrap();
    assert_eq!(game.observe(PlayerId::One).hand.len(), 6);

    game.apply(PlayerId::Two, Action::KeepHand).unwrap();
    assert!(
        game.legal_actions(PlayerId::One)
            .contains(&Action::PassPriority)
    );
}

#[test]
fn mountain_casts_and_resolves_lightning_bolt() {
    let mut game = game_with_mountain_and_bolt();
    advance_to_first_main(&mut game);

    let observation = game.observe(PlayerId::One);
    let mountain = observation
        .hand
        .iter()
        .find(|(_, definition)| {
            *definition == CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000001")
        })
        .unwrap()
        .0;
    let bolt = observation
        .hand
        .iter()
        .find(|(_, definition)| {
            *definition == CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000002")
        })
        .unwrap()
        .0;

    game.apply(
        PlayerId::One,
        Action::PlayLand {
            card: mountain,
            option: PlayOptionId::DEFAULT,
        },
    )
    .unwrap();
    let mountain = game
        .observe(PlayerId::One)
        .battlefield
        .iter()
        .find(|permanent| {
            permanent.characteristics.card_definition()
                == Some(CardDefinitionId::from_uuid(
                    "00000000-0000-0000-0000-000000000001",
                ))
        })
        .unwrap()
        .id;
    activate_red_mana(&mut game, PlayerId::One, mountain);
    game.apply(
        PlayerId::One,
        Action::CastSpell {
            card: bolt,
            choices: CastChoices::default()
                .with_x(0)
                .with_targets(vec![TargetSelection::new(
                    TargetSlotId(0),
                    vec![Target::Player(PlayerId::Two)],
                )]),
            sacrifices: Vec::new(),
        },
    )
    .unwrap();

    let on_stack = game.observe(PlayerId::One);
    assert_eq!(on_stack.stack.len(), 1);
    let spell_id = on_stack.stack[0].id;
    assert_ne!(spell_id, bolt);
    assert_eq!(on_stack.life_totals, [20, 20]);
    assert_eq!(on_stack.mana_pools[0].red, 0);

    pass_priority_pair(&mut game);

    let resolved = game.observe(PlayerId::One);
    assert!(resolved.stack.is_empty());
    assert_eq!(resolved.life_totals, [20, 17]);
    assert_eq!(
        resolved.graveyards[0][0].1,
        CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000002")
    );
    assert_ne!(resolved.graveyards[0][0].0, spell_id);
    assert!(game.events().contains(&GameEvent::DamageDealt {
        player: PlayerId::Two,
        amount: 3,
    }));
}

#[test]
fn nonbasic_lands_cannot_be_cast_as_spells() {
    use poc::cards::{MISHRA_S_FACTORY, STRIP_MINE};

    let catalog = poc::catalog().unwrap();
    for definition in [MISHRA_S_FACTORY, STRIP_MINE] {
        let mut checked = false;
        for seed in 0..1_000 {
            let deck = poc::artifacts();
            let mut game = Game::new(catalog.clone(), [deck.clone(), deck], seed).unwrap();
            if !game
                .observe(PlayerId::One)
                .hand
                .iter()
                .any(|(_, card)| *card == definition)
            {
                continue;
            }

            keep_both(&mut game);
            advance_to_first_main(&mut game);
            let observation = game.observe(PlayerId::One);
            let card = observation
                .hand
                .iter()
                .find(|(_, card)| *card == definition)
                .unwrap()
                .0;
            let actions = game.legal_actions(PlayerId::One);

            assert!(actions.contains(&Action::PlayLand {
                card,
                option: PlayOptionId::DEFAULT,
            }));
            assert!(!actions.iter().any(
                |action| matches!(action, Action::CastSpell { card: cast, .. } if *cast == card)
            ));
            checked = true;
            break;
        }
        assert!(checked, "expected a seed with nonbasic land {definition:?}");
    }
}

#[test]
fn unspent_mana_burns_at_the_end_of_a_phase() {
    let mut game = game_with_mountain_and_bolt();
    advance_to_first_main(&mut game);
    let mountain = game
        .observe(PlayerId::One)
        .hand
        .iter()
        .find(|(_, definition)| {
            *definition == CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000001")
        })
        .unwrap()
        .0;

    game.apply(
        PlayerId::One,
        Action::PlayLand {
            card: mountain,
            option: PlayOptionId::DEFAULT,
        },
    )
    .unwrap();
    let mountain = game
        .observe(PlayerId::One)
        .battlefield
        .iter()
        .find(|permanent| {
            permanent.characteristics.card_definition()
                == Some(CardDefinitionId::from_uuid(
                    "00000000-0000-0000-0000-000000000001",
                ))
        })
        .unwrap()
        .id;
    activate_red_mana(&mut game, PlayerId::One, mountain);
    pass_priority_pair(&mut game);

    let observation = game.observe(PlayerId::One);
    assert_eq!(observation.step, Step::BeginningOfCombat);
    assert_eq!(observation.life_totals, [19, 20]);
    assert_eq!(observation.mana_pools[0].red, 0);
    assert!(game.events().contains(&GameEvent::ManaBurn {
        player: PlayerId::One,
        amount: 1,
    }));
}

#[test]
fn mana_emptying_and_burn_follow_the_games_format() {
    let catalog = catalog();
    let deck = Deck {
        main: vec![CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000001"); 60],
        sideboard: Vec::new(),
    };

    for (format, expected_mana) in [(Format::OldSchool9394, 1), (Format::IsdM14Standard, 0)] {
        let mut game =
            Game::new_with_format(format, catalog.clone(), [deck.clone(), deck.clone()], 0)
                .unwrap();
        assert_eq!(game.format(), format);
        keep_both(&mut game);
        advance_to_first_main(&mut game);

        let mountain = game.observe(PlayerId::One).hand[0].0;
        game.apply(
            PlayerId::One,
            Action::PlayLand {
                card: mountain,
                option: PlayOptionId::DEFAULT,
            },
        )
        .unwrap();
        let mountain = game.observe(PlayerId::One).battlefield.first().unwrap().id;
        pass_priority_pair(&mut game);
        assert_eq!(game.observe(PlayerId::One).step, Step::BeginningOfCombat);

        activate_red_mana(&mut game, PlayerId::One, mountain);
        pass_priority_pair(&mut game);

        let observation = game.observe(PlayerId::One);
        assert_eq!(observation.step, Step::DeclareAttackers);
        assert_eq!(observation.mana_pools[0].red, expected_mana);
        assert_eq!(observation.life_totals, [20, 20]);
        assert!(
            !game
                .events()
                .iter()
                .any(|event| matches!(event, GameEvent::ManaBurn { .. }))
        );
    }
}

#[test]
fn game_validates_decks_against_its_own_catalog() {
    let catalog = catalog();
    let short_deck = Deck {
        main: vec![CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000001"); 59],
        sideboard: Vec::new(),
    };

    assert!(matches!(
        Game::new(catalog, [short_deck, valid_deck()], 0),
        Err(GameError::InvalidDeck {
            player: PlayerId::One,
            error: DeckError::MainDeckTooSmall {
                actual: 59,
                minimum: 60,
            },
        })
    ));
}

#[test]
fn drawing_from_an_empty_library_loses_the_game() {
    let catalog = catalog();
    let mut game = Game::new(catalog, [valid_deck(), valid_deck()], 0).unwrap();
    keep_both(&mut game);

    for _ in 0..3_000 {
        if game.result().is_some() {
            break;
        }
        let (actor, action) = [PlayerId::One, PlayerId::Two]
            .into_iter()
            .find_map(|actor| {
                game.legal_actions(actor)
                    .into_iter()
                    .find(|action| !matches!(action, Action::Concede))
                    .map(|action| (actor, action))
            })
            .unwrap();
        game.apply(actor, action).unwrap();
    }

    assert_eq!(
        game.result(),
        Some(GameResult::Winner {
            winner: PlayerId::One,
            reason: WinReason::OpponentTriedToDrawFromEmptyLibrary,
        })
    );
}

#[test]
#[ignore = "slow simulation sweep"]
fn sweep_all_builtin_deck_matchups_complete_under_deterministic_greedy_bots() {
    let decks = [
        ("Goblins", penta::poc::goblins()),
        ("Sligh", penta::poc::sligh()),
        ("Artifacts", penta::poc::artifacts()),
        ("Robots", penta::poc::robots()),
        ("The Deck", penta::poc::the_deck()),
        ("Mono Black", penta::poc::mono_black()),
        ("White Weenie", penta::poc::white_weenie()),
        ("Erhnamgeddon", penta::poc::erhnamgeddon()),
        ("Counterburn", penta::poc::counterburn()),
        ("Lions/Dib", penta::poc::lions_dib()),
    ];
    // Every pairing plays, but only in one seating: the reversed game is the
    // redundant half of a quadratic grid, and this is a termination check
    // rather than a matchup evaluation. Which deck leads alternates with the
    // pair's parity, so no deck is stuck on the play or on the draw.
    for (left, (left_name, left_deck)) in decks.iter().enumerate() {
        for (offset, (right_name, right_deck)) in decks.iter().skip(left).enumerate() {
            let [(first_name, first_deck), (second_name, second_deck)] = if (left + offset) % 2 == 0
            {
                [(left_name, left_deck), (right_name, right_deck)]
            } else {
                [(right_name, right_deck), (left_name, left_deck)]
            };
            let mut game = Game::new(
                penta::poc::catalog().unwrap(),
                [first_deck.clone(), second_deck.clone()],
                2_026,
            )
            .unwrap();
            for _ in 0..50_000 {
                let Some(player) = game.decision_player() else {
                    break;
                };
                let action = choose_greedy_action(&game, player).unwrap();
                game.apply(player, action).unwrap();
            }
            assert!(
                game.result().is_some(),
                "{first_name} versus {second_name} did not terminate"
            );
        }
    }
}

#[test]
#[ignore = "slow simulation sweep"]
fn sweep_every_registered_premodern_matchup_completes_under_deterministic_greedy_bots() {
    // Read from the registry rather than named here, so a list promoted out of
    // `decks/premodern/` is swept the day it is registered instead of the day
    // somebody remembers to add it.
    let names = penta::protocol::deck_names_for_format(Format::Premodern);
    assert!(
        !names.is_empty(),
        "the Premodern registry is empty, so this sweeps nothing"
    );
    let catalog = poc::catalog().unwrap();

    // Both seatings, and the mirror: two decks make too small a grid to give
    // up half of it, and a mirror is the one pairing where a deck has to beat
    // its own clock.
    for (left, left_name) in names.iter().enumerate() {
        for right_name in names.iter().skip(left) {
            for (offset, [first, second]) in [[left_name, right_name], [right_name, left_name]]
                .into_iter()
                .enumerate()
            {
                let build = |name: &str| {
                    penta::protocol::deck_by_name_for_format(Format::Premodern, name)
                        .unwrap_or_else(|| panic!("{name} is a registered Premodern deck"))
                };
                let mut game = Game::new_with_format(
                    Format::Premodern,
                    catalog.clone(),
                    [build(first), build(second)],
                    2_026 + u64::try_from(offset).expect("seating index fits"),
                )
                .unwrap();
                for _ in 0..50_000 {
                    let Some(player) = game.decision_player() else {
                        break;
                    };
                    let action = choose_greedy_action(&game, player).unwrap();
                    game.apply(player, action).unwrap();
                }
                assert!(
                    game.result().is_some(),
                    "{first} versus {second} did not terminate"
                );
            }
        }
    }
}

fn choose_greedy_action(game: &Game, player: PlayerId) -> Option<Action> {
    if let Some(decision) = game.observe(player).decision {
        return Some(Action::ChooseDecision {
            decision: decision.id,
            options: decision
                .options
                .iter()
                .take(decision.minimum)
                .map(|option| option.id)
                .collect(),
        });
    }
    let actions = game.legal_actions(player);
    let choose = |predicate: &dyn Fn(&Action) -> bool| {
        actions.iter().find(|action| predicate(action)).cloned()
    };

    choose(&|action| matches!(action, Action::KeepHand))
        .or_else(|| choose(&|action| matches!(action, Action::BottomCards { .. })))
        .or_else(|| choose(&|action| matches!(action, Action::DiscardCards { .. })))
        .or_else(|| {
            actions
                .iter()
                .filter_map(|action| match action {
                    Action::ChooseUntap { permanents } => Some((permanents.len(), action.clone())),
                    _ => None,
                })
                .max_by_key(|(count, _)| *count)
                .map(|(_, action)| action)
        })
        .or_else(|| choose(&|action| matches!(action, Action::PlayLand { .. })))
        .or_else(|| choose(&|action| matches!(action, Action::ActivateManaAbility { .. })))
        .or_else(|| {
            actions
                .iter()
                .filter_map(|action| match action {
                    Action::CastSpell { choices, .. } => {
                        let attacks_opponent = choices
                            .iter_targets()
                            .any(|target| *target == Target::Player(player.opponent()));
                        Some((attacks_opponent, choices.x(), action.clone()))
                    }
                    _ => None,
                })
                .max_by_key(|(attacks_opponent, x, _)| (*attacks_opponent, *x))
                .map(|(_, _, action)| action)
        })
        .or_else(|| choose(&|action| matches!(action, Action::ActivateAbility { .. })))
        .or_else(|| choose(&|action| matches!(action, Action::DeclareAttacker { .. })))
        .or_else(|| choose(&|action| matches!(action, Action::FinishDeclaringAttackers)))
        .or_else(|| choose(&|action| matches!(action, Action::DeclareBlocker { .. })))
        .or_else(|| choose(&|action| matches!(action, Action::FinishDeclaringBlockers)))
        .or_else(|| choose(&|action| matches!(action, Action::AssignCombatDamage { .. })))
        .or_else(|| choose(&|action| matches!(action, Action::PassPriority)))
}

#[test]
#[allow(clippy::too_many_lines)]
fn aura_sequence_attaches_to_its_indexed_semantic_target() {
    use penta::{
        AbilityDef, AbilityTargetDef, AbilityTargetPredicate, AppliedEffectDef, CardComposition,
        CardRules, CardType, EffectDef, EffectRecipientDef, ManaCost, ObjectPredicateDef,
        TargetIndex, ZoneKind,
    };

    static ATTACH_SEQUENCE: [EffectDef; 1] = [EffectDef::Attach {
        object: EffectRecipientDef::Target(TargetIndex(1)),
    }];
    static FLYING: AbilityDef = penta::card::abilities::flying();
    static AURA_ABILITIES: [AbilityDef; 2] = [
        AbilityDef::spell_with_targets(
            "Enchant creature",
            &[
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Land),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                }),
                AbilityTargetDef::exactly_one(AbilityTargetPredicate::Object {
                    object: ObjectPredicateDef::HasType(CardType::Creature),
                    zones: &[ZoneKind::Battlefield],
                    controller: None,
                    owner: None,
                }),
            ],
            EffectDef::Sequence(&ATTACH_SEQUENCE),
        ),
        AbilityDef::static_ability(
            "Enchanted creature has flying.",
            EffectDef::StaticApply {
                recipient: EffectRecipientDef::AttachedPermanent,
                effect: AppliedEffectDef::add_ability(&FLYING),
            },
        ),
    ];

    fn definition_with_rules(
        id: CardDefinitionId,
        name: &str,
        rules: &CardRules,
    ) -> CardDefinition {
        let composition = CardComposition::single(name, *rules);
        CardDefinition {
            id,
            name: name.into(),
            art: None,
            debut_set: sets::alpha::SET,
            printings: vec![CardPrinting::new(id, sets::alpha::SET)],
            rules: *rules,
            parts: composition.parts,
            structure: composition.structure,
            play_options: composition.play_options,
        }
    }

    let mountain_definition: CardDefinitionId =
        CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000001");
    let creature_definition: CardDefinitionId =
        CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000006");
    let aura_definition: CardDefinitionId =
        CardDefinitionId::from_uuid("00000000-0000-0000-0000-000000000007");

    let creature = definition_with_rules(
        creature_definition,
        "Semantic Aura Host",
        &CardRules::new_creature(ManaCost::new(0, 0), &["Bear"], 2, 2),
    );
    let aura = definition_with_rules(
        aura_definition,
        "Indexed Test Aura",
        &CardRules::new_enchantment(ManaCost::new(0, 0))
            .with_subtypes(&["Aura"])
            .with_abilities(&AURA_ABILITIES),
    );
    let catalog = CardCatalog::new([
        CardDefinition::new(
            mountain_definition,
            "Mountain",
            sets::alpha::SET,
            CardRules::new_land(&["Mountain"]).with_supertype(CardSupertype::Basic),
        ),
        creature,
        aura,
    ])
    .unwrap();
    let deck = Deck {
        main: vec![mountain_definition; 60],
        sideboard: Vec::new(),
    };
    let mut game = Game::new(catalog, [deck.clone(), deck], 0).unwrap();
    keep_both(&mut game);
    advance_to_first_main(&mut game);
    game.set_hand(PlayerId::One, &[aura_definition]).unwrap();

    let land = game
        .put_onto_battlefield(PlayerId::One, mountain_definition)
        .unwrap();
    let creature = game
        .put_onto_battlefield(PlayerId::One, creature_definition)
        .unwrap();
    let aura = game.hand(PlayerId::One)[0].object;
    game.apply(
        PlayerId::One,
        Action::CastSpell {
            card: aura,
            choices: CastChoices::default().with_targets(vec![
                TargetSelection::new(TargetSlotId(0), vec![Target::Permanent(land)]),
                TargetSelection::new(TargetSlotId(1), vec![Target::Permanent(creature)]),
            ]),
            sacrifices: Vec::new(),
        },
    )
    .unwrap();
    pass_priority_pair(&mut game);

    let battlefield = game.observe(PlayerId::One).battlefield;
    assert!(
        battlefield
            .iter()
            .any(|permanent| permanent.characteristics.card_definition() == Some(aura_definition)),
        "the Aura remains attached because target 1 satisfies its enchant restriction"
    );
    assert!(
        battlefield
            .iter()
            .find(|permanent| permanent.id == creature)
            .is_some_and(|permanent| permanent.flying),
        "the granted ability follows the indexed Attach target, not the first target"
    );
}

/// The simulation surface: a caller who owns the process can read hidden state
/// and state what it might have been instead. That is what determinized search
/// needs, and the engine supplies no distribution for it.
#[path = "engine/hidden_state.rs"]
mod hidden_state;
