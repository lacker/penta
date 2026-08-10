use penta::card::{self, cards};
use penta::game::{PermanentObservation, StackObservation};
use penta::poc;
use penta::{
    AbilityId, AbilityOrigin, Action, AlternativeCostId, BasicLandType, CardInstanceId, CardPartId,
    CastChoices, CastSignature, CostConfiguration, Game, GameResult, HandcraftedPolicy, ManaPool,
    PlayOptionId, PlayerId, PlayerObservation, Policy, RandomPolicy, SpellForm, StackObjectKind,
    Step, Target, TargetSelection, TargetSlotId, play_game,
};

const ACTION_LIMIT: usize = 50_000;
const PRIMARY_PRINTED_ABILITY: AbilityOrigin = AbilityOrigin::Printed {
    definition: penta::CardDefinitionId(0),
    part: CardPartId::PRIMARY,
    ability: AbilityId::PRIMARY,
};

fn activated_targets(target: Target) -> Vec<TargetSelection> {
    vec![TargetSelection::single(TargetSlotId(0), target)]
}

fn policy_observation(
    battlefield: Vec<PermanentObservation>,
    legal_actions: Vec<Action>,
) -> PlayerObservation {
    PlayerObservation {
        viewer: PlayerId::One,
        turn: 3,
        active_turn: 2,
        active_player: PlayerId::One,
        priority: PlayerId::One,
        step: Step::PrecombatMain,
        regular_combat_damage_pending: false,
        life_totals: [20, 20],
        mana_pools: [ManaPool::default(), ManaPool::default()],
        hand: Vec::new(),
        opponent_hand_size: 0,
        last_seen_hand: None,
        library_sizes: [50, 50],
        graveyards: [Vec::new(), Vec::new()],
        exiles: [Vec::new(), Vec::new()],
        battlefield,
        stack: Vec::new(),
        decision: None,
        result: None,
        legal_actions,
    }
}

fn permanent(
    id: u32,
    definition: penta::CardDefinitionId,
    controller: PlayerId,
    power: Option<i16>,
    toughness: Option<i16>,
) -> PermanentObservation {
    PermanentObservation {
        id: CardInstanceId(id),
        definition,
        presented: CardPartId::PRIMARY,
        controller,
        chosen_creature_type: None,
        tapped: false,
        power,
        toughness,
        damage: 0,
        attacking: false,
        blocking: None,
        flying: false,
        can_attack: false,
        entered_this_turn: false,
    }
}

fn stack_object(
    id: u32,
    definition: penta::CardDefinitionId,
    controller: PlayerId,
    kind: StackObjectKind,
    targets: Vec<Target>,
) -> StackObservation {
    StackObservation {
        id: CardInstanceId(id),
        kind,
        source: None,
        ability: None,
        ability_text: None,
        definition,
        controller,
        counterable: true,
        signature: (kind == StackObjectKind::Spell).then(|| {
            CastSignature::from_validated_choices(
                SpellForm::Part(CardPartId::PRIMARY),
                CastChoices::default(),
            )
        }),
        targets,
        chosen_permanents: Vec::new(),
        x: 0,
    }
}

const BLOODRUSH: AbilityOrigin = AbilityOrigin::Printed {
    definition: cards::GHOR_CLAN_RAMPAGER,
    part: CardPartId::PRIMARY,
    ability: AbilityId(1),
};

fn bloodrush_action(source: CardInstanceId, target: CardInstanceId) -> Action {
    Action::ActivateAbility {
        source,
        ability: BLOODRUSH,
        targets: activated_targets(Target::Permanent(target)),
        cost_object: None,
        x: 0,
    }
}

#[test]
fn random_policy_is_seeded_and_avoids_conceding() {
    let catalog = poc::catalog().unwrap();
    let game = Game::new(catalog, [poc::goblins(), poc::goblins()], 17).unwrap();
    let observation = game.observe(PlayerId::One);
    let mut first = RandomPolicy::new(99);
    let mut second = RandomPolicy::new(99);

    for _ in 0..20 {
        let first_action = first.choose_action(&observation);
        let second_action = second.choose_action(&observation);
        assert_eq!(first_action, second_action);
        assert!(!matches!(first_action, Some(Action::Concede)));
    }
}

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
fn handcrafted_does_not_feed_a_creature_to_a_superior_blocker() {
    let catalog = poc::catalog().unwrap();
    let attacker = permanent(
        1,
        poc::cards::GOBLIN_BALLOON_BRIGADE,
        PlayerId::One,
        Some(1),
        Some(1),
    );
    let blocker = permanent(2, poc::cards::SU_CHI, PlayerId::Two, Some(4), Some(4));
    let mut observation = policy_observation(
        vec![attacker, blocker],
        vec![
            Action::FinishDeclaringAttackers,
            Action::DeclareAttacker {
                attacker: CardInstanceId(1),
            },
        ],
    );
    observation.step = Step::DeclareAttackers;
    let mut policy = HandcraftedPolicy::new(catalog);

    assert_eq!(
        policy.choose_action(&observation),
        Some(Action::FinishDeclaringAttackers)
    );
}

#[test]
fn handcrafted_does_not_add_a_redundant_lethal_blocker() {
    let catalog = poc::catalog().unwrap();
    let mut attacker = permanent(
        1,
        poc::cards::GOBLIN_BALLOON_BRIGADE,
        PlayerId::Two,
        Some(1),
        Some(1),
    );
    attacker.attacking = true;
    let mut first_blocker = permanent(
        2,
        poc::cards::GOBLIN_BALLOON_BRIGADE,
        PlayerId::One,
        Some(1),
        Some(1),
    );
    first_blocker.blocking = Some(CardInstanceId(1));
    let second_blocker = permanent(
        3,
        poc::cards::GOBLIN_BALLOON_BRIGADE,
        PlayerId::One,
        Some(1),
        Some(1),
    );
    let mut observation = policy_observation(
        vec![attacker, first_blocker, second_blocker],
        vec![
            Action::FinishDeclaringBlockers,
            Action::DeclareBlocker {
                blocker: CardInstanceId(3),
                attacker: CardInstanceId(1),
            },
        ],
    );
    observation.step = Step::DeclareBlockers;
    let mut policy = HandcraftedPolicy::new(catalog);

    assert_eq!(
        policy.choose_action(&observation),
        Some(Action::FinishDeclaringBlockers)
    );
}

#[test]
fn handcrafted_deploys_a_creature_before_burning_a_nonlethal_player() {
    let catalog = poc::catalog().unwrap();
    let bolt = CardInstanceId(1);
    let goblin = CardInstanceId(2);
    let cast_bolt = Action::CastSpell {
        card: bolt,
        choices: CastChoices::default()
            .with_x(0)
            .with_targets(vec![TargetSelection::new(
                TargetSlotId(0),
                vec![Target::Player(PlayerId::Two)],
            )]),
        sacrifices: Vec::new(),
    };
    let cast_goblin = Action::CastSpell {
        card: goblin,
        choices: CastChoices::default().with_x(0).with_targets(Vec::new()),
        sacrifices: Vec::new(),
    };
    let mut observation = policy_observation(
        Vec::new(),
        vec![Action::PassPriority, cast_bolt, cast_goblin.clone()],
    );
    observation.hand = vec![
        (bolt, poc::cards::LIGHTNING_BOLT),
        (goblin, poc::cards::GOBLIN_BALLOON_BRIGADE),
    ];
    let mut policy = HandcraftedPolicy::new(catalog);

    assert_eq!(policy.choose_action(&observation), Some(cast_goblin));
}

#[test]
fn handcrafted_never_burns_itself_when_the_opponent_is_a_legal_target() {
    let catalog = poc::catalog().unwrap();
    let bolt = CardInstanceId(1);
    let hit_self = Action::CastSpell {
        card: bolt,
        choices: CastChoices::default()
            .with_x(0)
            .with_targets(vec![TargetSelection::new(
                TargetSlotId(0),
                vec![Target::Player(PlayerId::One)],
            )]),
        sacrifices: Vec::new(),
    };
    let hit_opponent = Action::CastSpell {
        card: bolt,
        choices: CastChoices::default()
            .with_x(0)
            .with_targets(vec![TargetSelection::new(
                TargetSlotId(0),
                vec![Target::Player(PlayerId::Two)],
            )]),
        sacrifices: Vec::new(),
    };
    let mut observation = policy_observation(
        Vec::new(),
        vec![Action::PassPriority, hit_opponent.clone(), hit_self],
    );
    observation.hand = vec![(bolt, poc::cards::LIGHTNING_BOLT)];
    let mut policy = HandcraftedPolicy::new(catalog);

    assert_eq!(policy.choose_action(&observation), Some(hit_opponent));
}

#[test]
fn handcrafted_plays_a_mountain_before_a_colorless_land() {
    let catalog = poc::catalog().unwrap();
    let strip = CardInstanceId(1);
    let mountain = CardInstanceId(2);
    let mut observation = policy_observation(
        Vec::new(),
        vec![
            Action::PlayLand {
                card: strip,
                option: PlayOptionId::DEFAULT,
            },
            Action::PlayLand {
                card: mountain,
                option: PlayOptionId::DEFAULT,
            },
        ],
    );
    observation.hand = vec![
        (strip, poc::cards::STRIP_MINE),
        (mountain, poc::cards::MOUNTAIN),
    ];
    let mut policy = HandcraftedPolicy::new(catalog);

    assert_eq!(
        policy.choose_action(&observation),
        Some(Action::PlayLand {
            card: mountain,
            option: PlayOptionId::DEFAULT,
        })
    );
}

#[test]
fn handcrafted_only_uses_orcish_mechanics_on_a_player_for_lethal() {
    let catalog = poc::catalog().unwrap();
    let mechanics = CardInstanceId(1);
    let vise = CardInstanceId(2);
    let observation = policy_observation(
        vec![
            permanent(
                1,
                poc::cards::ORCISH_MECHANICS,
                PlayerId::One,
                Some(1),
                Some(1),
            ),
            permanent(2, poc::cards::BLACK_VISE, PlayerId::One, None, None),
        ],
        vec![
            Action::PassPriority,
            Action::ActivateAbility {
                source: mechanics,
                ability: PRIMARY_PRINTED_ABILITY,
                targets: activated_targets(Target::Player(PlayerId::Two)),
                cost_object: Some(vise),
                x: 0,
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
fn handcrafted_scores_triskelion_from_its_declarative_damage_effect() {
    let catalog = poc::catalog().unwrap();
    let triskelion = CardInstanceId(1);
    let target = CardInstanceId(2);
    let ability = AbilityOrigin::Printed {
        definition: cards::TRISKELION,
        part: CardPartId::PRIMARY,
        ability: AbilityId(1),
    };
    let hit_creature = Action::ActivateAbility {
        source: triskelion,
        ability,
        targets: activated_targets(Target::Permanent(target)),
        cost_object: None,
        x: 0,
    };
    let observation = policy_observation(
        vec![
            permanent(1, cards::TRISKELION, PlayerId::One, Some(2), Some(2)),
            permanent(2, cards::SAVANNAH_LIONS, PlayerId::Two, Some(2), Some(1)),
        ],
        vec![
            Action::PassPriority,
            Action::ActivateAbility {
                source: triskelion,
                ability,
                targets: activated_targets(Target::Player(PlayerId::Two)),
                cost_object: None,
                x: 0,
            },
            hit_creature.clone(),
        ],
    );
    let mut policy = HandcraftedPolicy::new(catalog);

    assert_eq!(policy.choose_action(&observation), Some(hit_creature));
}

#[test]
fn handcrafted_sacrifices_artifacts_to_atog_for_an_unblocked_lethal_attack() {
    let catalog = poc::catalog().unwrap();
    let atog = CardInstanceId(1);
    let vise = CardInstanceId(2);
    let mox = CardInstanceId(3);
    let mut attacking_atog = permanent(1, poc::cards::ATOG, PlayerId::One, Some(1), Some(2));
    attacking_atog.attacking = true;
    let mut observation = policy_observation(
        vec![
            attacking_atog,
            permanent(2, poc::cards::BLACK_VISE, PlayerId::One, None, None),
            permanent(3, poc::cards::MOX_RUBY, PlayerId::One, None, None),
        ],
        vec![
            Action::PassPriority,
            Action::ActivateAbility {
                source: atog,
                ability: PRIMARY_PRINTED_ABILITY,
                targets: Vec::new(),
                cost_object: Some(vise),
                x: 0,
            },
            Action::ActivateAbility {
                source: atog,
                ability: PRIMARY_PRINTED_ABILITY,
                targets: Vec::new(),
                cost_object: Some(mox),
                x: 0,
            },
        ],
    );
    observation.life_totals = [20, 5];
    observation.step = Step::DeclareBlockers;
    let mut policy = HandcraftedPolicy::new(catalog);

    assert_eq!(
        policy.choose_action(&observation),
        Some(Action::ActivateAbility {
            source: atog,
            ability: PRIMARY_PRINTED_ABILITY,
            targets: Vec::new(),
            cost_object: Some(vise),
            x: 0,
        })
    );
}

#[test]
#[ignore = "slow simulation sweep"]
fn handcrafted_policy_decisively_beats_random_across_builtin_decks_and_seats() {
    let catalog = poc::catalog().unwrap();
    let decks = [
        poc::goblins(),
        poc::sligh(),
        poc::artifacts(),
        poc::robots(),
        poc::the_deck(),
    ];
    let mut wins = 0;
    let mut decided_games = 0;

    for deck in decks {
        for seed in 0..10 {
            for handcrafted_seat in [PlayerId::One, PlayerId::Two] {
                let mut game =
                    Game::new(catalog.clone(), [deck.clone(), deck.clone()], seed).unwrap();
                let mut handcrafted = HandcraftedPolicy::new(catalog.clone());
                let mut random = RandomPolicy::new(seed ^ 0xa11c_e5ed);
                let result = match handcrafted_seat {
                    PlayerId::One => {
                        play_game(&mut game, &mut handcrafted, &mut random, ACTION_LIMIT)
                    }
                    PlayerId::Two => {
                        play_game(&mut game, &mut random, &mut handcrafted, ACTION_LIMIT)
                    }
                }
                .unwrap();

                if let GameResult::Winner { winner, .. } = result {
                    decided_games += 1;
                    wins += usize::from(winner == handcrafted_seat);
                }
            }
        }
    }

    assert_eq!(decided_games, 100);
    assert!(
        wins >= 90,
        "handcrafted policy won only {wins} of {decided_games} games"
    );
}

#[test]
fn handcrafted_never_aims_removal_at_its_own_permanents() {
    let catalog = poc::catalog().unwrap();
    let own_angel = permanent(1, poc::cards::SERRA_ANGEL, PlayerId::One, Some(4), Some(4));
    let own_mox = permanent(2, poc::cards::MOX_PEARL, PlayerId::One, None, None);
    let swords = CardInstanceId(10);
    let disenchant = CardInstanceId(11);
    let mut observation = policy_observation(
        vec![own_angel, own_mox],
        vec![
            Action::PassPriority,
            Action::CastSpell {
                card: swords,
                choices: CastChoices::default()
                    .with_x(0)
                    .with_targets(vec![TargetSelection::new(
                        TargetSlotId(0),
                        vec![Target::Permanent(CardInstanceId(1))],
                    )]),
                sacrifices: Vec::new(),
            },
            Action::CastSpell {
                card: disenchant,
                choices: CastChoices::default()
                    .with_x(0)
                    .with_targets(vec![TargetSelection::new(
                        TargetSlotId(0),
                        vec![Target::Permanent(CardInstanceId(2))],
                    )]),
                sacrifices: Vec::new(),
            },
        ],
    );
    observation.hand = vec![
        (swords, poc::cards::SWORDS_TO_PLOWSHARES),
        (disenchant, poc::cards::DISENCHANT),
    ];
    let mut policy = HandcraftedPolicy::new(catalog);

    // Removal carries a large base score, so a merely-unattractive friendly
    // target used to stay far above passing.
    assert_eq!(
        policy.choose_action(&observation),
        Some(Action::PassPriority),
        "the only targets on offer are its own board, so it should hold the removal",
    );
}

#[test]
fn handcrafted_still_spends_removal_on_the_opponent() {
    let catalog = poc::catalog().unwrap();
    let own_angel = permanent(1, poc::cards::SERRA_ANGEL, PlayerId::One, Some(4), Some(4));
    let their_angel = permanent(3, poc::cards::SERRA_ANGEL, PlayerId::Two, Some(4), Some(4));
    let swords = CardInstanceId(10);
    let cast_at_theirs = Action::CastSpell {
        card: swords,
        choices: CastChoices::default()
            .with_x(0)
            .with_targets(vec![TargetSelection::new(
                TargetSlotId(0),
                vec![Target::Permanent(CardInstanceId(3))],
            )]),
        sacrifices: Vec::new(),
    };
    let mut observation = policy_observation(
        vec![own_angel, their_angel],
        vec![
            Action::PassPriority,
            Action::CastSpell {
                card: swords,
                choices: CastChoices::default()
                    .with_x(0)
                    .with_targets(vec![TargetSelection::new(
                        TargetSlotId(0),
                        vec![Target::Permanent(CardInstanceId(1))],
                    )]),
                sacrifices: Vec::new(),
            },
            cast_at_theirs.clone(),
        ],
    );
    observation.hand = vec![(swords, poc::cards::SWORDS_TO_PLOWSHARES)];
    let mut policy = HandcraftedPolicy::new(catalog);

    assert_eq!(policy.choose_action(&observation), Some(cast_at_theirs));
}

#[test]
fn handcrafted_animates_a_factory_once_rather_than_every_priority() {
    let catalog = poc::catalog().unwrap();
    let animate = Action::ActivateAbility {
        source: CardInstanceId(1),
        ability: PRIMARY_PRINTED_ABILITY,
        targets: Vec::new(),
        cost_object: None,
        x: 0,
    };

    let dormant = permanent(1, poc::cards::MISHRA_S_FACTORY, PlayerId::One, None, None);
    let mut observation =
        policy_observation(vec![dormant], vec![Action::PassPriority, animate.clone()]);
    observation.step = Step::BeginningOfCombat;
    let mut policy = HandcraftedPolicy::new(catalog.clone());
    assert_eq!(
        policy.choose_action(&observation),
        Some(animate.clone()),
        "a dormant Factory is still worth animating",
    );

    // Same board, except the Factory is already a 2/2.
    let awake = permanent(
        1,
        poc::cards::MISHRA_S_FACTORY,
        PlayerId::One,
        Some(2),
        Some(2),
    );
    let mut observation = policy_observation(vec![awake], vec![Action::PassPriority, animate]);
    observation.step = Step::BeginningOfCombat;
    let mut policy = HandcraftedPolicy::new(catalog);
    assert_eq!(
        policy.choose_action(&observation),
        Some(Action::PassPriority),
        "animating a Factory that is already a creature only burns mana",
    );
}

#[test]
fn handcrafted_bloodrush_prefers_its_own_attacking_creature() {
    let source = CardInstanceId(100);
    let own_attacker_id = CardInstanceId(101);
    let opposing_attacker_id = CardInstanceId(102);
    let mut own_attacker = permanent(
        own_attacker_id.0,
        cards::SAVANNAH_LIONS,
        PlayerId::One,
        Some(2),
        Some(1),
    );
    own_attacker.attacking = true;
    let mut opposing_attacker = permanent(
        opposing_attacker_id.0,
        cards::SAVANNAH_LIONS,
        PlayerId::Two,
        Some(2),
        Some(1),
    );
    opposing_attacker.attacking = true;
    let own_action = bloodrush_action(source, own_attacker_id);
    let mut observation = policy_observation(
        vec![own_attacker, opposing_attacker],
        vec![
            Action::PassPriority,
            bloodrush_action(source, opposing_attacker_id),
            own_action.clone(),
        ],
    );
    observation.step = Step::DeclareBlockers;
    observation.hand = vec![(source, cards::GHOR_CLAN_RAMPAGER)];
    let mut policy = HandcraftedPolicy::new(card::catalog().unwrap());

    assert_eq!(policy.choose_action(&observation), Some(own_action));
}

#[test]
fn handcrafted_bloodrush_passes_when_only_an_opposing_attacker_is_available() {
    let source = CardInstanceId(100);
    let opposing_attacker_id = CardInstanceId(102);
    let mut opposing_attacker = permanent(
        opposing_attacker_id.0,
        cards::SAVANNAH_LIONS,
        PlayerId::Two,
        Some(2),
        Some(1),
    );
    opposing_attacker.attacking = true;
    let mut observation = policy_observation(
        vec![opposing_attacker],
        vec![
            Action::PassPriority,
            bloodrush_action(source, opposing_attacker_id),
        ],
    );
    observation.step = Step::DeclareBlockers;
    observation.hand = vec![(source, cards::GHOR_CLAN_RAMPAGER)];
    let mut policy = HandcraftedPolicy::new(card::catalog().unwrap());

    assert_eq!(
        policy.choose_action(&observation),
        Some(Action::PassPriority)
    );
}

#[test]
fn handcrafted_bloodrush_waits_for_blockers_and_stops_after_damage() {
    for step in [
        Step::DeclareAttackers,
        Step::CombatDamage,
        Step::EndOfCombat,
    ] {
        let source = CardInstanceId(100);
        let attacker_id = CardInstanceId(101);
        let mut attacker = permanent(
            attacker_id.0,
            cards::SAVANNAH_LIONS,
            PlayerId::One,
            Some(2),
            Some(1),
        );
        attacker.attacking = true;
        let mut observation = policy_observation(
            vec![attacker],
            vec![Action::PassPriority, bloodrush_action(source, attacker_id)],
        );
        observation.step = step;
        observation.hand = vec![(source, cards::GHOR_CLAN_RAMPAGER)];
        let mut policy = HandcraftedPolicy::new(card::catalog().unwrap());

        assert_eq!(
            policy.choose_action(&observation),
            Some(Action::PassPriority),
            "Bloodrush should not be spent during {step:?}",
        );
    }
}

#[test]
fn handcrafted_bloodrush_is_used_between_strike_damage_waves() {
    let source = CardInstanceId(100);
    let attacker_id = CardInstanceId(101);
    let mut attacker = permanent(
        attacker_id.0,
        cards::SAVANNAH_LIONS,
        PlayerId::One,
        Some(2),
        Some(1),
    );
    attacker.attacking = true;
    let bloodrush = bloodrush_action(source, attacker_id);
    let mut observation = policy_observation(
        vec![attacker],
        vec![Action::PassPriority, bloodrush.clone()],
    );
    observation.step = Step::CombatDamage;
    observation.regular_combat_damage_pending = true;
    observation.hand = vec![(source, cards::GHOR_CLAN_RAMPAGER)];
    let mut policy = HandcraftedPolicy::new(card::catalog().unwrap());

    assert_eq!(policy.choose_action(&observation), Some(bloodrush));
}

#[test]
fn handcrafted_does_not_overload_counterflux_into_an_empty_stack() {
    let source = CardInstanceId(100);
    let overload = Action::CastSpell {
        card: source,
        choices: CastChoices::default().with_costs(CostConfiguration::new(
            Some(AlternativeCostId(2)),
            Vec::new(),
        )),
        sacrifices: Vec::new(),
    };
    let mut observation = policy_observation(Vec::new(), vec![Action::PassPriority, overload]);
    observation.hand = vec![(source, cards::COUNTERFLUX)];
    let mut policy = HandcraftedPolicy::new(card::catalog().unwrap());

    assert_eq!(
        policy.choose_action(&observation),
        Some(Action::PassPriority)
    );
}

#[test]
fn handcrafted_does_not_counter_an_observed_uncounterable_spell() {
    let source = CardInstanceId(100);
    let threat = CardInstanceId(200);
    let normal = Action::CastSpell {
        card: source,
        choices: CastChoices::default().with_targets(vec![TargetSelection::single(
            TargetSlotId(0),
            Target::Spell(threat),
        )]),
        sacrifices: Vec::new(),
    };
    let mut observation = policy_observation(Vec::new(), vec![Action::PassPriority, normal]);
    observation.hand = vec![(source, cards::COUNTERFLUX)];
    let mut uncounterable = stack_object(
        threat.0,
        cards::COUNTERFLUX,
        PlayerId::Two,
        StackObjectKind::Spell,
        Vec::new(),
    );
    uncounterable.counterable = false;
    observation.stack = vec![uncounterable];
    let mut policy = HandcraftedPolicy::new(card::catalog().unwrap());

    assert_eq!(
        policy.choose_action(&observation),
        Some(Action::PassPriority)
    );
}

#[test]
fn handcrafted_overload_counts_only_effective_unanswered_spells() {
    let source = CardInstanceId(100);
    let answered = CardInstanceId(200);
    let uncounterable = CardInstanceId(201);
    let ability = CardInstanceId(202);
    let overload = Action::CastSpell {
        card: source,
        choices: CastChoices::default().with_costs(CostConfiguration::new(
            Some(AlternativeCostId(2)),
            Vec::new(),
        )),
        sacrifices: Vec::new(),
    };
    let mut observation =
        policy_observation(Vec::new(), vec![Action::PassPriority, overload.clone()]);
    observation.hand = vec![(source, cards::COUNTERFLUX)];
    let mut uncounterable_spell = stack_object(
        uncounterable.0,
        cards::COUNTERFLUX,
        PlayerId::Two,
        StackObjectKind::Spell,
        Vec::new(),
    );
    uncounterable_spell.counterable = false;
    observation.stack = vec![
        stack_object(
            answered.0,
            cards::SERRA_ANGEL,
            PlayerId::Two,
            StackObjectKind::Spell,
            Vec::new(),
        ),
        uncounterable_spell,
        stack_object(
            ability.0,
            cards::SAVANNAH_LIONS,
            PlayerId::Two,
            StackObjectKind::ActivatedAbility,
            Vec::new(),
        ),
        stack_object(
            203,
            cards::COUNTERSPELL,
            PlayerId::One,
            StackObjectKind::Spell,
            vec![Target::Spell(answered)],
        ),
    ];
    let mut policy = HandcraftedPolicy::new(card::catalog().unwrap());

    assert_eq!(
        policy.choose_action(&observation),
        Some(Action::PassPriority),
        "uncounterable, already-answered, and nonspell objects provide no overload value",
    );

    observation.stack = vec![
        stack_object(
            210,
            cards::SERRA_ANGEL,
            PlayerId::Two,
            StackObjectKind::Spell,
            Vec::new(),
        ),
        stack_object(
            211,
            cards::TRISKELION,
            PlayerId::Two,
            StackObjectKind::Spell,
            Vec::new(),
        ),
    ];
    assert_eq!(policy.choose_action(&observation), Some(overload));
}

#[test]
fn handcrafted_animates_a_manland_only_when_it_can_attack() {
    let catalog = penta::card::catalog().unwrap();
    let animate = Action::ActivateAbility {
        source: CardInstanceId(1),
        ability: AbilityOrigin::Printed {
            definition: cards::MUTAVAULT,
            part: CardPartId::PRIMARY,
            // The mana ability is printed first; the animation follows it.
            ability: AbilityId(1),
        },
        targets: Vec::new(),
        cost_object: None,
        x: 0,
    };
    let vault = || permanent(1, cards::MUTAVAULT, PlayerId::One, None, None);

    let mut main_phase =
        policy_observation(vec![vault()], vec![Action::PassPriority, animate.clone()]);
    main_phase.step = Step::PrecombatMain;
    let mut policy = HandcraftedPolicy::new(catalog.clone());
    assert_eq!(
        policy.choose_action(&main_phase),
        Some(Action::PassPriority),
        "animating outside combat spends mana and risks the land for nothing",
    );

    let mut combat = policy_observation(vec![vault()], vec![Action::PassPriority, animate.clone()]);
    combat.step = Step::BeginningOfCombat;
    let mut policy = HandcraftedPolicy::new(catalog.clone());
    assert_eq!(
        policy.choose_action(&combat),
        Some(animate.clone()),
        "a land that can still attack is worth animating",
    );

    // A tapped land cannot attack, so the animation buys nothing.
    let mut tapped_vault = vault();
    tapped_vault.tapped = true;
    let mut tapped = policy_observation(vec![tapped_vault], vec![Action::PassPriority, animate]);
    tapped.step = Step::BeginningOfCombat;
    let mut policy = HandcraftedPolicy::new(catalog);
    assert_eq!(
        policy.choose_action(&tapped),
        Some(Action::PassPriority),
        "a tapped land cannot attack, so animating it only burns mana",
    );
}
