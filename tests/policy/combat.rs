use super::*;

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
                defender: AttackDefender::Player(PlayerId::Two),
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
fn handcrafted_attacks_a_killable_opposing_planeswalker() {
    let catalog = card::catalog().unwrap();
    let attacker = permanent(1, cards::GOBLIN_KING, PlayerId::One, Some(3), Some(3));
    let mut domri = permanent(2, cards::DOMRI_RADE, PlayerId::Two, None, None);
    domri.loyalty = Some(3);
    let attack_domri = Action::DeclareAttacker {
        attacker: CardInstanceId(1),
        defender: AttackDefender::Planeswalker(CardInstanceId(2)),
    };
    let mut observation = policy_observation(
        vec![attacker, domri],
        vec![
            Action::FinishDeclaringAttackers,
            Action::DeclareAttacker {
                attacker: CardInstanceId(1),
                defender: AttackDefender::Player(PlayerId::Two),
            },
            attack_domri.clone(),
        ],
    );
    observation.step = Step::DeclareAttackers;
    let mut policy = HandcraftedPolicy::new(catalog);

    assert_eq!(policy.choose_action(&observation), Some(attack_domri));
}

#[test]
fn handcrafted_attacks_the_player_when_combat_damage_is_lethal() {
    let catalog = card::catalog().unwrap();
    let attacker = permanent(1, cards::GOBLIN_KING, PlayerId::One, Some(3), Some(3));
    let mut domri = permanent(2, cards::DOMRI_RADE, PlayerId::Two, None, None);
    domri.loyalty = Some(3);
    let attack_player = Action::DeclareAttacker {
        attacker: CardInstanceId(1),
        defender: AttackDefender::Player(PlayerId::Two),
    };
    let mut observation = policy_observation(
        vec![attacker, domri],
        vec![
            Action::FinishDeclaringAttackers,
            attack_player.clone(),
            Action::DeclareAttacker {
                attacker: CardInstanceId(1),
                defender: AttackDefender::Planeswalker(CardInstanceId(2)),
            },
        ],
    );
    observation.life_totals = [20, 3];
    observation.step = Step::DeclareAttackers;
    let mut policy = HandcraftedPolicy::new(catalog);

    assert_eq!(policy.choose_action(&observation), Some(attack_player));
}

#[test]
fn handcrafted_uses_domri_to_win_a_favorable_fight() {
    let catalog = card::catalog().unwrap();
    let mut domri = permanent(1, cards::DOMRI_RADE, PlayerId::One, None, None);
    domri.loyalty = Some(3);
    let fighter = permanent(2, cards::SU_CHI, PlayerId::One, Some(4), Some(4));
    let victim = permanent(
        3,
        cards::GOBLIN_BALLOON_BRIGADE,
        PlayerId::Two,
        Some(1),
        Some(1),
    );
    let fight = Action::ActivateAbility {
        source: CardInstanceId(1),
        ability: printed_ability(cards::DOMRI_RADE, 1),
        targets: vec![
            TargetSelection::single(TargetSlotId(0), Target::Permanent(CardInstanceId(2))),
            TargetSelection::single(TargetSlotId(1), Target::Permanent(CardInstanceId(3))),
        ],
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
        mana_payment: None,
    };
    let observation = policy_observation(
        vec![domri, fighter, victim],
        vec![
            Action::PassPriority,
            Action::ActivateAbility {
                source: CardInstanceId(1),
                ability: printed_ability(cards::DOMRI_RADE, 0),
                targets: Vec::new(),
                cost_objects: Vec::new(),
                x: 0,
                modes: Vec::new(),
                mana_payment: None,
            },
            fight.clone(),
        ],
    );
    let mut policy = HandcraftedPolicy::new(catalog);

    assert_eq!(policy.choose_action(&observation), Some(fight));
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
    first_blocker.blocking = vec![CardInstanceId(1)];
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
                ability: primary_printed_ability(),
                targets: activated_targets(Target::Player(PlayerId::Two)),
                cost_objects: vec![vise],
                x: 0,
                modes: Vec::new(),
                mana_payment: None,
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
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
        mana_payment: None,
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
                cost_objects: Vec::new(),
                x: 0,
                modes: Vec::new(),
                mana_payment: None,
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
    // Scoring reads the real ability now, so a stand-in origin would find no
    // sacrifice cost and no pump.
    let atog_ability = AbilityOrigin::Printed {
        definition: poc::cards::ATOG,
        part: CardPartId::PRIMARY,
        ability: AbilityId::PRIMARY,
    };
    let mut attacking_atog = permanent(1, poc::cards::ATOG, PlayerId::One, Some(1), Some(2));
    attacking_atog.attacking = true;
    attacking_atog.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
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
                ability: atog_ability,
                targets: Vec::new(),
                cost_objects: vec![vise],
                x: 0,
                modes: Vec::new(),
                mana_payment: None,
            },
            Action::ActivateAbility {
                source: atog,
                ability: atog_ability,
                targets: Vec::new(),
                cost_objects: vec![mox],
                x: 0,
                modes: Vec::new(),
                mana_payment: None,
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
            ability: atog_ability,
            targets: Vec::new(),
            cost_objects: vec![vise],
            x: 0,
            modes: Vec::new(),
            mana_payment: None,
        })
    );
}
