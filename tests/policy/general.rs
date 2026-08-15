use super::*;

#[test]
fn handcrafted_accepts_detention_spheres_optional_exile() {
    let catalog = card::catalog().unwrap();
    let mut game = Game::new(catalog.clone(), [poc::goblins(), poc::goblins()], 17).unwrap();
    game.apply(PlayerId::One, Action::KeepHand).unwrap();
    game.apply(PlayerId::Two, Action::KeepHand).unwrap();
    let lion = game
        .put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
        .unwrap();
    game.put_onto_battlefield(PlayerId::One, cards::DETENTION_SPHERE)
        .unwrap();
    let mut policy = HandcraftedPolicy::new(catalog);

    // Finish the setup procedure so the Sphere's ETB trigger asks for its
    // target. Answer that rules choice directly; the behavior under test is
    // the policy's later answer to the optional effect itself.
    game.apply(PlayerId::One, Action::PassPriority).unwrap();
    let target_observation = game.observe(PlayerId::One);
    let target_decision = target_observation
        .decision
        .as_ref()
        .expect("the ETB trigger asks for a target");
    let lion_option = target_decision
        .options
        .iter()
        .find(|option| option.card == Some((lion, cards::SAVANNAH_LIONS)))
        .expect("the opposing Lion is a legal target")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: target_decision.id,
            options: vec![lion_option],
        },
    )
    .unwrap();

    let mut optional = None;
    for _ in 0..4 {
        let player = game.decision_player().expect("the game is still running");
        let observation = game.observe(player);
        if observation.decision.as_ref().is_some_and(|decision| {
            decision
                .options
                .iter()
                .any(|option| option.label == "Do it")
        }) {
            optional = Some((player, observation));
            break;
        }
        assert!(
            observation.decision.is_none(),
            "only the optional-effect decision remains"
        );
        game.apply(player, Action::PassPriority).unwrap();
    }

    let (player, observation) = optional.expect("the ETB trigger offered its optional effect");
    let decision = observation
        .decision
        .as_ref()
        .expect("the optional effect is pending");
    let action = policy
        .choose_action(&observation)
        .expect("the policy answers the optional effect");
    assert_eq!(
        action,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![1],
        },
    );
    game.apply(player, action).unwrap();

    let resolved = game.observe(PlayerId::One);
    assert!(
        !resolved
            .battlefield
            .iter()
            .any(|permanent| permanent.id == lion),
        "choosing Do it exiles the targeted Lion",
    );
}

#[test]
fn handcrafted_angel_of_serenity_selects_only_helpful_targets() {
    let catalog = card::catalog().unwrap();
    let mut game = Game::new(catalog.clone(), [poc::goblins(), poc::goblins()], 17).unwrap();
    game.apply(PlayerId::One, Action::KeepHand).unwrap();
    game.apply(PlayerId::Two, Action::KeepHand).unwrap();

    let own_battlefield = game
        .put_onto_battlefield(PlayerId::One, cards::SAVANNAH_LIONS)
        .unwrap();
    let opposing_battlefield = game
        .put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
        .unwrap();
    let own_graveyard = game
        .put_into_graveyard(PlayerId::One, cards::SERRA_ANGEL)
        .unwrap();
    let opposing_graveyard = game
        .put_into_graveyard(PlayerId::Two, cards::SERRA_ANGEL)
        .unwrap();
    game.put_onto_battlefield(PlayerId::One, cards::ANGEL_OF_SERENITY)
        .unwrap();
    game.apply(PlayerId::One, Action::PassPriority).unwrap();

    let observation = game.observe(PlayerId::One);
    let decision = observation
        .decision
        .as_ref()
        .expect("Angel's ETB trigger asks for up to three targets");
    let mut policy = HandcraftedPolicy::new(catalog);
    let action = policy
        .choose_action(&observation)
        .expect("the policy places Angel's trigger");
    let Action::ChooseDecision { options, .. } = action else {
        panic!("expected target selection, got {action:?}");
    };
    let mut selected = decision
        .options
        .iter()
        .filter(|option| options.contains(&option.id))
        .filter_map(|option| option.card.map(|(card, _)| card))
        .collect::<Vec<_>>();
    selected.sort_unstable();
    let mut expected = vec![opposing_battlefield, own_graveyard];
    expected.sort_unstable();

    assert_eq!(selected, expected);
    assert!(!selected.contains(&own_battlefield));
    assert!(!selected.contains(&opposing_graveyard));
}

#[test]
fn handcrafted_does_not_float_unneeded_mana_in_its_main_phase() {
    let catalog = poc::catalog().unwrap();
    let mountain = permanent(1, poc::cards::MOUNTAIN, PlayerId::One, None, None);
    let observation = policy_observation(
        vec![mountain],
        vec![
            Action::PassPriority,
            Action::ActivateManaAbility {
                source: CardInstanceId(1),
                ability: AbilityOrigin::IntrinsicBasicLand(BasicLandType::Mountain),
                color: penta::ManaColor::Red,
            },
        ],
    );
    let mut policy = HandcraftedPolicy::new(catalog);

    assert_eq!(
        policy.choose_action(&observation),
        Some(Action::PassPriority)
    );
}

#[test]
fn handcrafted_does_not_counter_a_publicly_uncounterable_spell() {
    let catalog = poc::catalog().unwrap();
    let counterspell = CardInstanceId(1);
    let abrupt_decay = CardInstanceId(2);
    let cast_counterspell = Action::CastSpell {
        card: counterspell,
        choices: CastChoices::default().with_targets(vec![TargetSelection::single(
            TargetSlotId(0),
            Target::Spell(abrupt_decay),
        )]),
        sacrifices: Vec::new(),
    };
    let mut observation =
        policy_observation(Vec::new(), vec![Action::PassPriority, cast_counterspell]);
    observation.hand = vec![(counterspell, poc::cards::COUNTERSPELL)];
    observation.stack.push(StackObservation {
        id: abrupt_decay,
        kind: StackObjectKind::Spell,
        source: None,
        ability: None,
        ability_text: None,
        definition: poc::cards::ABRUPT_DECAY,
        controller: PlayerId::Two,
        counterable: false,
        signature: None,
        targets: Vec::new(),
        chosen_permanents: Vec::new(),
        x: 0,
    });
    let mut policy = HandcraftedPolicy::new(catalog);

    assert_eq!(
        policy.choose_action(&observation),
        Some(Action::PassPriority)
    );
}

#[test]
fn handcrafted_scores_declarative_creature_sweepers_by_the_board_swing() {
    let catalog = poc::catalog().unwrap();
    for definition in [poc::cards::WRATH_OF_GOD, poc::cards::SUPREME_VERDICT] {
        let card = CardInstanceId(10);
        let cast = Action::CastSpell {
            card,
            choices: CastChoices::default(),
            sacrifices: Vec::new(),
        };
        let mut behind = policy_observation(
            vec![
                permanent(
                    1,
                    poc::cards::SAVANNAH_LIONS,
                    PlayerId::One,
                    Some(2),
                    Some(1),
                ),
                permanent(2, poc::cards::ATOG, PlayerId::Two, Some(1), Some(2)),
                permanent(3, poc::cards::SU_CHI, PlayerId::Two, Some(4), Some(4)),
            ],
            vec![Action::PassPriority, cast.clone()],
        );
        behind.hand = vec![(card, definition)];
        let mut policy = HandcraftedPolicy::new(catalog.clone());
        assert_eq!(policy.choose_action(&behind), Some(cast.clone()));

        let mut ahead = policy_observation(
            vec![
                permanent(
                    4,
                    poc::cards::SAVANNAH_LIONS,
                    PlayerId::One,
                    Some(2),
                    Some(1),
                ),
                permanent(5, poc::cards::SU_CHI, PlayerId::One, Some(4), Some(4)),
                permanent(6, poc::cards::ATOG, PlayerId::Two, Some(1), Some(2)),
            ],
            vec![Action::PassPriority, cast],
        );
        ahead.hand = vec![(card, definition)];
        let mut policy = HandcraftedPolicy::new(catalog.clone());
        assert_eq!(policy.choose_action(&ahead), Some(Action::PassPriority));
    }
}

#[test]
fn handcrafted_scores_declarative_nevinyrrals_disk_by_the_board_swing() {
    let catalog = poc::catalog().unwrap();
    let disk = CardInstanceId(10);
    let activate = Action::ActivateAbility {
        source: disk,
        ability: printed_ability(poc::cards::NEVINYRRALS_DISK, 1),
        targets: Vec::new(),
        cost_object: None,
        x: 0,
    };

    let behind = policy_observation(
        vec![
            permanent(
                disk.0,
                poc::cards::NEVINYRRALS_DISK,
                PlayerId::One,
                None,
                None,
            ),
            permanent(
                1,
                poc::cards::SAVANNAH_LIONS,
                PlayerId::One,
                Some(2),
                Some(1),
            ),
            permanent(2, poc::cards::ATOG, PlayerId::Two, Some(1), Some(2)),
            permanent(3, poc::cards::SU_CHI, PlayerId::Two, Some(4), Some(4)),
        ],
        vec![Action::PassPriority, activate.clone()],
    );
    let mut policy = HandcraftedPolicy::new(catalog.clone());
    assert_eq!(policy.choose_action(&behind), Some(activate.clone()));

    let ahead = policy_observation(
        vec![
            permanent(
                disk.0,
                poc::cards::NEVINYRRALS_DISK,
                PlayerId::One,
                None,
                None,
            ),
            permanent(
                4,
                poc::cards::SAVANNAH_LIONS,
                PlayerId::One,
                Some(2),
                Some(1),
            ),
            permanent(5, poc::cards::SU_CHI, PlayerId::One, Some(4), Some(4)),
            permanent(6, poc::cards::ATOG, PlayerId::Two, Some(1), Some(2)),
        ],
        vec![Action::PassPriority, activate],
    );
    let mut policy = HandcraftedPolicy::new(catalog);
    assert_eq!(policy.choose_action(&ahead), Some(Action::PassPriority));
}

#[test]
fn handcrafted_fires_nevinyrrals_disk_for_noncreature_permanents() {
    let catalog = poc::catalog().unwrap();
    let disk = CardInstanceId(10);
    let activate = Action::ActivateAbility {
        source: disk,
        ability: printed_ability(poc::cards::NEVINYRRALS_DISK, 1),
        targets: Vec::new(),
        cost_object: None,
        x: 0,
    };
    let observation = policy_observation(
        vec![
            permanent(
                disk.0,
                poc::cards::NEVINYRRALS_DISK,
                PlayerId::One,
                None,
                None,
            ),
            permanent(1, poc::cards::MOX_RUBY, PlayerId::Two, None, None),
            permanent(2, poc::cards::MOAT, PlayerId::Two, None, None),
        ],
        vec![Action::PassPriority, activate.clone()],
    );
    let mut policy = HandcraftedPolicy::new(catalog);

    assert_eq!(policy.choose_action(&observation), Some(activate));
}

#[test]
fn handcrafted_prioritizes_declarative_time_walk() {
    let time_walk = CardInstanceId(10);
    let cast = Action::CastSpell {
        card: time_walk,
        choices: CastChoices::default(),
        sacrifices: Vec::new(),
    };
    let mut observation = policy_observation(Vec::new(), vec![Action::PassPriority, cast.clone()]);
    observation.hand = vec![(time_walk, cards::TIME_WALK)];
    let mut policy = HandcraftedPolicy::new(card::catalog().unwrap());

    assert_eq!(policy.choose_action(&observation), Some(cast));
}

#[test]
fn handcrafted_counts_intrinsic_basic_land_mana_when_mulliganing() {
    let catalog = poc::catalog().unwrap();
    let mut observation =
        policy_observation(Vec::new(), vec![Action::KeepHand, Action::TakeMulligan]);
    observation.hand = vec![
        (CardInstanceId(1), poc::cards::MOUNTAIN),
        (CardInstanceId(2), poc::cards::MOUNTAIN),
        (CardInstanceId(3), poc::cards::LIGHTNING_BOLT),
        (CardInstanceId(4), poc::cards::LIGHTNING_BOLT),
        (CardInstanceId(5), poc::cards::LIGHTNING_BOLT),
        (CardInstanceId(6), poc::cards::LIGHTNING_BOLT),
        (CardInstanceId(7), poc::cards::LIGHTNING_BOLT),
    ];
    let mut policy = HandcraftedPolicy::new(catalog);

    assert_eq!(policy.choose_action(&observation), Some(Action::KeepHand));
}

#[test]
fn handcrafted_holds_an_x_draw_spell_rather_than_casting_it_for_zero() {
    let catalog = poc::catalog().unwrap();
    let geyser = CardInstanceId(1);
    // With only enough mana for the base UU, the sole legal Braingeyser cast is
    // X=0, which draws nobody any cards. Passing is strictly better.
    let cast_for_zero = Action::CastSpell {
        card: geyser,
        choices: CastChoices::default()
            .with_x(0)
            .with_targets(vec![TargetSelection::new(
                TargetSlotId(0),
                vec![Target::Player(PlayerId::One)],
            )]),
        sacrifices: Vec::new(),
    };
    let mut observation = policy_observation(Vec::new(), vec![Action::PassPriority, cast_for_zero]);
    observation.hand = vec![(geyser, poc::cards::BRAINGEYSER)];
    let mut policy = HandcraftedPolicy::new(catalog);

    assert_eq!(
        policy.choose_action(&observation),
        Some(Action::PassPriority),
        "an X-draw spell for X=0 draws nothing, so the bot should hold it",
    );
}

#[test]
fn handcrafted_still_casts_detonate_for_zero_to_destroy_a_mox() {
    let catalog = poc::catalog().unwrap();
    let detonate = CardInstanceId(1);
    let mox = CardInstanceId(2);
    // Detonate is the counterexample: only its damage scales with X, so an X=0
    // cast still destroys a zero-cost artifact and is worth the card.
    let cast_for_zero = Action::CastSpell {
        card: detonate,
        choices: CastChoices::default()
            .with_x(0)
            .with_targets(vec![TargetSelection::new(
                TargetSlotId(0),
                vec![Target::Permanent(mox)],
            )]),
        sacrifices: Vec::new(),
    };
    let mut observation = policy_observation(
        vec![permanent(
            2,
            poc::cards::MOX_RUBY,
            PlayerId::Two,
            None,
            None,
        )],
        vec![Action::PassPriority, cast_for_zero.clone()],
    );
    observation.hand = vec![(detonate, poc::cards::DETONATE)];
    let mut policy = HandcraftedPolicy::new(catalog);

    assert_eq!(
        policy.choose_action(&observation),
        Some(cast_for_zero),
        "Detonate for X=0 still destroys a zero-cost artifact",
    );
}
