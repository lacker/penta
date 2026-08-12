use super::*;

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
fn handcrafted_prioritizes_time_vaults_declarative_extra_turn() {
    let source = CardInstanceId(100);
    let tome = CardInstanceId(101);
    let activate = Action::ActivateAbility {
        source,
        ability: printed_ability(cards::TIME_VAULT, 3),
        targets: Vec::new(),
        cost_object: None,
        x: 0,
    };
    let draw = Action::ActivateAbility {
        source: tome,
        ability: printed_ability(cards::JAYEMDAE_TOME, 0),
        targets: Vec::new(),
        cost_object: None,
        x: 0,
    };
    let observation = policy_observation(
        vec![
            permanent(source.0, cards::TIME_VAULT, PlayerId::One, None, None),
            permanent(tome.0, cards::JAYEMDAE_TOME, PlayerId::One, None, None),
        ],
        vec![Action::PassPriority, draw, activate.clone()],
    );
    let mut policy = HandcraftedPolicy::new(card::catalog().unwrap());

    assert_eq!(policy.choose_action(&observation), Some(activate));
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
