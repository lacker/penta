use super::*;

#[test]
fn handcrafted_begins_the_turn_by_default_for_time_vaults_choice() {
    let catalog = poc::catalog().unwrap();
    let mut observation = policy_observation(Vec::new(), Vec::new());
    observation.decision = Some(DecisionObservation {
        id: 40,
        player: PlayerId::One,
        kind: DecisionKind::Choice,
        order_semantics: None,
        prompt: "A turn would begin".to_owned(),
        visibility: DecisionVisibility::Public,
        preference: DecisionPreference::PreferOption(0),
        minimum: 1,
        maximum: 1,
        cancellable: false,
        options: vec![
            DecisionOption {
                id: 0,
                label: "Begin the turn".to_owned(),
                card: None,
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::None,
            },
            DecisionOption {
                id: 1,
                label: "Apply Time Vault's replacement effect".to_owned(),
                card: Some((CardInstanceId(10), cards::TIME_VAULT)),
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::Battlefield,
            },
        ],
    });
    let mut policy = HandcraftedPolicy::new(catalog);

    assert_eq!(
        policy.choose_action(&observation),
        Some(Action::ChooseDecision {
            decision: 40,
            options: vec![0],
        })
    );
}

#[test]
fn handcrafted_balanced_partition_is_deterministic_nonempty_and_near_balanced() {
    let catalog = poc::catalog().unwrap();
    let decision = DecisionObservation {
        id: 41,
        player: PlayerId::One,
        kind: DecisionKind::Choice,
        order_semantics: None,
        prompt: "Separate these permanents into two piles".to_owned(),
        visibility: DecisionVisibility::Public,
        preference: DecisionPreference::BalancedPartition,
        minimum: 0,
        maximum: 4,
        cancellable: false,
        options: vec![
            DecisionOption {
                id: 0,
                label: "Mountain".to_owned(),
                card: Some((CardInstanceId(10), poc::cards::MOUNTAIN)),
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::Battlefield,
            },
            DecisionOption {
                id: 1,
                label: "Lightning Bolt".to_owned(),
                card: Some((CardInstanceId(11), poc::cards::LIGHTNING_BOLT)),
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::Battlefield,
            },
            DecisionOption {
                id: 2,
                label: "Goblin Balloon Brigade".to_owned(),
                card: Some((CardInstanceId(12), poc::cards::GOBLIN_BALLOON_BRIGADE)),
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::Battlefield,
            },
            DecisionOption {
                id: 3,
                label: "Black Vise".to_owned(),
                card: Some((CardInstanceId(13), poc::cards::BLACK_VISE)),
                members: Vec::new(),
                ability_text: None,
                zone: DecisionZone::Battlefield,
            },
        ],
    };
    let mut observation = policy_observation(Vec::new(), Vec::new());
    observation.decision = Some(decision);
    let mut policy = HandcraftedPolicy::new(catalog);

    let first = policy.choose_action(&observation);
    assert_eq!(first, policy.choose_action(&observation));
    let Some(Action::ChooseDecision { options, .. }) = first else {
        panic!("the partition has a policy choice");
    };
    // Mountain 80, Lightning Bolt 55, Goblin Balloon Brigade 65, Black Vise
    // 55. Bolt and the Vise are worth the same, so more than one split is
    // equally even; what matters is that the pile is a real one and as close
    // to half as the values allow.
    let value = |id: u32| match id {
        0 => 80,
        // Lightning Bolt and Black Vise are worth the same, which is what
        // makes more than one split equally even here.
        1 | 3 => 55,
        2 => 65,
        other => panic!("unexpected option {other}"),
    };
    assert!(!options.is_empty(), "the pile has to be a real one");
    assert!(options.len() < 4, "and so does the pile left behind");
    let taken: i32 = options.iter().copied().map(value).sum();
    assert_eq!(
        (255 - 2 * taken).abs(),
        15,
        "no split of 80/55/65/55 comes closer to even than fifteen points",
    );

    for (minimum, maximum, expected_count) in [(0, 1, 1), (3, 4, 3)] {
        let mut bounded = observation.clone();
        let pending = bounded.decision.as_mut().expect("decision exists");
        pending.minimum = minimum;
        pending.maximum = maximum;
        let Some(Action::ChooseDecision { options, .. }) = policy.choose_action(&bounded) else {
            panic!("a valid bounded partition has a policy choice");
        };
        assert_eq!(options.len(), expected_count);
    }
}

#[test]
fn handcrafted_lower_card_value_uses_members_to_choose_the_cheaper_pile() {
    let catalog = poc::catalog().unwrap();
    let mut observation = policy_observation(Vec::new(), Vec::new());
    observation.decision = Some(DecisionObservation {
        id: 42,
        player: PlayerId::One,
        kind: DecisionKind::Choice,
        order_semantics: None,
        prompt: "Choose a pile to sacrifice".to_owned(),
        visibility: DecisionVisibility::Public,
        preference: DecisionPreference::LowerCardValue,
        minimum: 1,
        maximum: 1,
        cancellable: false,
        options: vec![
            DecisionOption {
                id: 0,
                label: "Mountain, Lightning Bolt".to_owned(),
                card: None,
                members: vec![
                    (CardInstanceId(20), poc::cards::MOUNTAIN),
                    (CardInstanceId(21), poc::cards::LIGHTNING_BOLT),
                ],
                ability_text: None,
                zone: DecisionZone::None,
            },
            DecisionOption {
                id: 1,
                label: "Goblin Balloon Brigade".to_owned(),
                card: None,
                members: vec![(CardInstanceId(22), poc::cards::GOBLIN_BALLOON_BRIGADE)],
                ability_text: None,
                zone: DecisionZone::None,
            },
        ],
    });
    let mut policy = HandcraftedPolicy::new(catalog);

    assert_eq!(
        policy.choose_action(&observation),
        Some(Action::ChooseDecision {
            decision: 42,
            options: vec![1],
        })
    );
}
