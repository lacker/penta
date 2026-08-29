use super::*;

/// Mana alone is not a reason to hold Main 1 open: every untapped land would
/// otherwise force a stop on a turn the player cannot spend it.
#[test]
fn human_main_one_yields_when_only_mana_actions_are_available() {
    let actions = [
        Action::Concede,
        Action::ActivateManaAbility {
            source: CardInstanceId(7),
            ability: penta::AbilityOrigin::IntrinsicBasicLand(penta::BasicLandType::Mountain),
            color: penta::ManaColor::Red,
            counters_removed: None,
            cost_object: None,
            combination: None,
            triggered_mana: None,
        },
        Action::PassPriority,
    ];

    assert_eq!(
        automatic_human_action(
            Step::PrecombatMain,
            true,
            true,
            false,
            false,
            true,
            false,
            false,
            &actions,
        ),
        Some(Action::PassPriority),
        "an actionless main phase on the human's own turn yields too",
    );
    assert_eq!(
        automatic_human_action(
            Step::PrecombatMain,
            false,
            true,
            false,
            false,
            true,
            false,
            false,
            &actions,
        ),
        Some(Action::PassPriority),
        "an actionless opponent main phase can still auto-yield",
    );
}

/// What the player keeps when Main 1 no longer stops on its own: a stop they
/// set, and the attack, which is a decision rather than a window.
#[test]
fn an_empty_main_one_still_honours_a_stop_and_the_attack_decision() {
    let quiet = [Action::Concede, Action::PassPriority];
    assert_eq!(
        automatic_human_action(
            Step::PrecombatMain,
            true,
            true,
            false,
            true,
            true,
            false,
            false,
            &quiet,
        ),
        None,
        "a Main 1 stop still holds the window open",
    );

    let attacks = [
        Action::Concede,
        Action::DeclareAttacker {
            attacker: CardInstanceId(7),
            defender: penta::AttackDefender::Player(PlayerId::Two),
        },
        Action::FinishDeclaringAttackers,
    ];
    assert_eq!(
        automatic_human_action(
            Step::DeclareAttackers,
            true,
            true,
            false,
            false,
            true,
            false,
            false,
            &attacks,
        ),
        None,
        "the step the skipped main phase runs into is the player's call",
    );
}

#[test]
fn a_real_game_action_still_stops_auto_pass() {
    let actions = [
        Action::Concede,
        Action::PlayLand {
            card: CardInstanceId(7),
            option: PlayOptionId::DEFAULT,
        },
        Action::PassPriority,
    ];

    assert_eq!(
        automatic_human_action(
            Step::PrecombatMain,
            true,
            true,
            false,
            false,
            true,
            false,
            false,
            &actions,
        ),
        None
    );
}

#[test]
fn second_main_waits_for_spells_lands_and_non_mana_abilities() {
    let context = AutoPassContext {
        step: Step::PostcombatMain,
        regular_combat_damage_pending: false,
        human_is_active: true,
        stack_is_empty: true,
        has_attacker: false,
        has_blocker: false,
        stop_here: false,
        autopass_enabled: true,
        only_human_objects_on_stack: false,
        human_has_floating_mana: false,
    };
    let useful_actions = [
        Action::PlayLand {
            card: CardInstanceId(7),
            option: PlayOptionId::DEFAULT,
        },
        Action::CastSpell {
            card: CardInstanceId(8),
            choices: choices_targeting(Target::Player(PlayerId::Two)),
            sacrifices: Vec::new(),
        },
        Action::ActivateAbility {
            source: CardInstanceId(9),
            ability: penta::AbilityOrigin::Printed {
                definition: penta::CardDefinitionId::new(1),
                part: penta::CardPartId::PRIMARY,
                ability: penta::AbilityId::PRIMARY,
            },
            targets: Vec::new(),
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
            mana_payment: None,
        },
    ];

    for useful_action in useful_actions {
        let actions = [Action::Concede, useful_action, Action::PassPriority];
        assert_eq!(
            automatic_human_action_for_context(context, &actions),
            None,
            "a legal spell, land play, or non-mana ability must keep second-main priority",
        );
    }

    let actionless = [
        Action::Concede,
        Action::ActivateManaAbility {
            source: CardInstanceId(10),
            ability: penta::AbilityOrigin::IntrinsicBasicLand(penta::BasicLandType::Mountain),
            color: penta::ManaColor::Red,
            counters_removed: None,
            cost_object: None,
            combination: None,
            triggered_mana: None,
        },
        Action::PassPriority,
    ];
    assert_eq!(
        automatic_human_action_for_context(context, &actionless),
        Some(Action::PassPriority),
        "a second main with only mana abilities can still auto-pass",
    );
}

#[test]
fn pregame_choices_do_not_enter_the_animation_queue() {
    assert!(!should_animate_action(&Action::KeepHand));
    assert!(!should_animate_action(&Action::TakeMulligan));
    assert!(!should_animate_action(&Action::BottomCards {
        cards: vec![CardInstanceId(4)],
    }));
    assert!(should_animate_action(&Action::PlayLand {
        card: CardInstanceId(4),
        option: PlayOptionId::DEFAULT,
    }));
}

#[test]
fn routine_beginning_windows_auto_pass_on_either_turn() {
    let actions = [
        Action::Concede,
        Action::CastSpell {
            card: CardInstanceId(7),
            choices: choices_targeting(Target::Player(PlayerId::Two)),
            sacrifices: Vec::new(),
        },
        Action::PassPriority,
    ];

    assert_eq!(
        automatic_human_action(
            Step::Upkeep,
            true,
            true,
            false,
            false,
            true,
            false,
            false,
            &actions,
        ),
        Some(Action::PassPriority)
    );
    assert_eq!(
        automatic_human_action(
            Step::Draw,
            true,
            true,
            false,
            false,
            true,
            false,
            false,
            &actions,
        ),
        Some(Action::PassPriority)
    );
    assert_eq!(
        automatic_human_action(
            Step::End,
            true,
            true,
            false,
            false,
            true,
            false,
            false,
            &actions,
        ),
        Some(Action::PassPriority)
    );
    assert_eq!(
        automatic_human_action(
            Step::Upkeep,
            false,
            true,
            false,
            false,
            true,
            false,
            false,
            &actions,
        ),
        Some(Action::PassPriority),
        "routine opponent upkeep priority is hidden unless the player sets a stop",
    );
    assert_eq!(
        automatic_human_action(
            Step::Draw,
            false,
            true,
            false,
            false,
            true,
            false,
            false,
            &actions,
        ),
        Some(Action::PassPriority),
        "routine opponent draw-step priority is hidden unless the player sets a stop",
    );
    assert_eq!(
        automatic_human_action(
            Step::End,
            true,
            true,
            false,
            false,
            true,
            false,
            true,
            &actions,
        ),
        None,
        "smart priority preserves floating mana in the human's end step",
    );
}

#[test]
fn empty_and_unblocked_combat_steps_auto_pass() {
    let actions = [
        Action::Concede,
        Action::CastSpell {
            card: CardInstanceId(7),
            choices: choices_targeting(Target::Player(PlayerId::Two)),
            sacrifices: Vec::new(),
        },
        Action::PassPriority,
    ];

    assert_eq!(
        automatic_human_action(
            Step::BeginningOfCombat,
            true,
            true,
            false,
            false,
            true,
            false,
            false,
            &actions,
        ),
        Some(Action::PassPriority)
    );
    assert_eq!(
        automatic_human_action(
            Step::CombatDamage,
            true,
            true,
            false,
            false,
            true,
            false,
            false,
            &actions,
        ),
        Some(Action::PassPriority)
    );
    assert_eq!(
        automatic_human_action(
            Step::CombatDamage,
            true,
            true,
            true,
            false,
            true,
            false,
            false,
            &actions,
        ),
        Some(Action::PassPriority),
        "an unblocked attack runs through combat damage without extra clicks",
    );

    assert_eq!(
        automatic_human_action_with_blockers(
            Step::DeclareBlockers,
            true,
            true,
            true,
            true,
            false,
            true,
            false,
            false,
            &actions,
        ),
        None,
        "a declared blocker interrupts smooth combat",
    );
}

#[test]
fn a_pump_ability_holds_combat_open_only_while_it_matters() {
    let actions = [
        Action::Concede,
        Action::ActivateAbility {
            source: CardInstanceId(8),
            ability: penta::AbilityOrigin::Printed {
                definition: penta::CardDefinitionId::new(1),
                part: penta::CardPartId::PRIMARY,
                ability: penta::AbilityId::PRIMARY,
            },
            targets: vec![penta::TargetSelection::single(
                penta::TargetSlotId(0),
                Target::Permanent(CardInstanceId(9)),
            )],
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
            mana_payment: None,
        },
        Action::PassPriority,
    ];
    assert_eq!(
        automatic_human_action(
            Step::DeclareAttackers,
            true,
            true,
            true,
            false,
            true,
            false,
            false,
            &actions,
        ),
        None,
        "a pump ability keeps priority while it can still change the attack",
    );
    assert_eq!(
        automatic_human_action(
            Step::CombatDamage,
            true,
            true,
            true,
            false,
            true,
            false,
            false,
            &actions,
        ),
        Some(Action::PassPriority),
        "but damage is already dealt by the time priority comes back",
    );
}

#[test]
fn first_strike_keeps_the_interwave_priority_window_open() {
    let actions = [
        Action::Concede,
        Action::CastSpell {
            card: CardInstanceId(7),
            choices: choices_targeting(Target::Player(PlayerId::Two)),
            sacrifices: Vec::new(),
        },
        Action::PassPriority,
    ];
    let settled = AutoPassContext {
        step: Step::CombatDamage,
        regular_combat_damage_pending: false,
        human_is_active: true,
        stack_is_empty: true,
        has_attacker: true,
        has_blocker: false,
        stop_here: false,
        autopass_enabled: true,
        only_human_objects_on_stack: false,
        human_has_floating_mana: false,
    };

    assert_eq!(
        automatic_human_action_for_context(settled, &actions),
        Some(Action::PassPriority),
        "ordinary post-damage priority remains routine",
    );
    assert_eq!(
        automatic_human_action_for_context(
            AutoPassContext {
                regular_combat_damage_pending: true,
                ..settled
            },
            &actions,
        ),
        None,
        "the same public step must pause before regular combat damage",
    );
}

#[test]
fn a_combat_stop_interrupts_an_unblocked_attack() {
    let actions = [Action::Concede, Action::PassPriority];
    assert_eq!(
        automatic_human_action_with_blockers(
            Step::CombatDamage,
            true,
            true,
            true,
            false,
            true,
            true,
            false,
            false,
            &actions,
        ),
        None
    );
}

#[test]
fn the_opponents_combat_and_second_main_yield_but_their_end_step_does_not() {
    // A castable instant is exactly what makes these windows "meaningful",
    // and exactly why the end step has to stay.
    let actions = [
        Action::Concede,
        Action::CastSpell {
            card: CardInstanceId(7),
            choices: choices_targeting(Target::Player(PlayerId::Two)),
            sacrifices: Vec::new(),
        },
        Action::PassPriority,
    ];
    let on_their_turn = |step| {
        automatic_human_action(
            step, false, true, false, false, true, false, false, &actions,
        )
    };

    for step in [
        Step::BeginningOfCombat,
        Step::DeclareAttackers,
        Step::DeclareBlockers,
        Step::EndOfCombat,
        Step::PostcombatMain,
    ] {
        assert_eq!(
            on_their_turn(step),
            Some(Action::PassPriority),
            "an unattacked {step:?} on the opponent's turn should yield",
        );
    }
    assert_eq!(
        on_their_turn(Step::End),
        None,
        "the opponent's end step is where instants get cast",
    );
}

#[test]
fn a_declared_attack_still_stops_on_the_opponents_turn() {
    let actions = [
        Action::Concede,
        Action::DeclareBlocker {
            blocker: CardInstanceId(7),
            attacker: CardInstanceId(8),
        },
        Action::PassPriority,
    ];
    assert_eq!(
        automatic_human_action(
            Step::DeclareBlockers,
            false,
            true,
            true,
            false,
            true,
            false,
            false,
            &actions,
        ),
        None,
        "blocks have to be declared against a real attack",
    );
}

#[test]
fn declaring_the_last_attacker_does_not_commit_the_attack() {
    // Running out of creatures to declare is not the same as being done,
    // so the browser still gets to show the confirm and cancel pair.
    let actions = [Action::Concede, Action::FinishDeclaringAttackers];
    assert_eq!(
        automatic_human_action(
            Step::DeclareAttackers,
            true,
            true,
            true,
            false,
            true,
            false,
            false,
            &actions,
        ),
        None,
    );
    assert_eq!(
        automatic_human_action(
            Step::DeclareAttackers,
            true,
            true,
            false,
            false,
            true,
            false,
            false,
            &actions,
        ),
        Some(Action::FinishDeclaringAttackers),
        "with nothing declared there is no attack to confirm",
    );
}

#[test]
fn a_phase_stop_blocks_the_ui_auto_pass() {
    let actions = [Action::Concede, Action::PassPriority];
    assert_eq!(
        automatic_human_action(
            Step::Upkeep,
            true,
            true,
            false,
            true,
            true,
            false,
            false,
            &actions,
        ),
        None
    );
}

#[test]
fn no_attackers_skip_the_rest_of_combat_even_with_a_combat_stop() {
    let actions = [Action::Concede, Action::PassPriority];
    assert_eq!(
        automatic_human_action(
            Step::CombatDamage,
            true,
            true,
            false,
            true,
            true,
            false,
            false,
            &actions,
        ),
        Some(Action::PassPriority)
    );
    assert_eq!(
        automatic_human_action(
            Step::CombatDamage,
            true,
            true,
            false,
            true,
            false,
            false,
            false,
            &actions,
        ),
        None,
        "turning auto-pass off still exposes the empty combat window",
    );
}

#[test]
fn autopass_yields_when_only_human_objects_are_on_the_stack() {
    let actions = [
        Action::Concede,
        Action::CastSpell {
            card: CardInstanceId(7),
            choices: choices_targeting(Target::Player(PlayerId::Two)),
            sacrifices: Vec::new(),
        },
        Action::PassPriority,
    ];
    assert_eq!(
        automatic_human_action(
            Step::PrecombatMain,
            true,
            false,
            false,
            false,
            true,
            true,
            false,
            &actions,
        ),
        Some(Action::PassPriority)
    );
    assert_eq!(
        automatic_human_action(
            Step::PrecombatMain,
            true,
            false,
            false,
            false,
            false,
            true,
            false,
            &actions,
        ),
        None
    );
}
