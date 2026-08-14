use super::*;

#[test]
fn domri_ultimate_grants_two_combat_damage_steps() {
    let mut game = ready_game();
    let mut domri = creature(10_000, cards::DOMRI_RADE, PlayerId::One);
    domri.set_counters(CounterKind::Loyalty, 7);
    let domri_id = domri.card.id;
    let attacker = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One);
    let attacker_id = attacker.card.id;
    game.battlefield = vec![domri, attacker];

    let ultimate = Action::ActivateAbility {
        source: domri_id,
        ability: AbilityOrigin::Printed {
            definition: cards::DOMRI_RADE,
            part: CardPartId::PRIMARY,
            ability: AbilityId(2),
        },
        targets: Vec::new(),
        cost_object: None,
        x: 0,
    };
    assert!(game.legal_actions(PlayerId::One).contains(&ultimate));
    game.apply(PlayerId::One, ultimate).unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.emblems.len(), 1);
    let attacker = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == attacker_id)
        .unwrap();
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::Two));

    game.step = Step::DeclareBlockers;
    game.advance_step();

    assert_eq!(game.step, Step::CombatDamage);
    assert!(game.regular_combat_damage_pending());
    assert_eq!(game.players[PlayerId::Two.index()].life, 18);

    pass_priority_pair(&mut game);

    assert_eq!(game.step, Step::CombatDamage);
    assert!(!game.regular_combat_damage_pending());
    assert_eq!(game.players[PlayerId::Two.index()].life, 16);
}

#[test]
fn attacker_controller_assigns_damage_freely_across_multiple_blockers() {
    let mut game = ready_game();
    let mut attacker = creature(10_000, cards::SU_CHI, PlayerId::One);
    attacker.attacking = true;
    let mut first_blocker = creature(10_001, cards::ATOG, PlayerId::Two);
    first_blocker.blocking = Some(attacker.card.id);
    let mut second_blocker = creature(10_002, cards::ATOG, PlayerId::Two);
    second_blocker.blocking = Some(attacker.card.id);
    let attacker_id = attacker.card.id;
    let first_id = first_blocker.card.id;
    let second_id = second_blocker.card.id;
    game.battlefield = vec![attacker, first_blocker, second_blocker];
    game.begin_combat_damage_assignment();

    let assignment = Action::AssignCombatDamage {
        attacker: attacker_id,
        assignments: vec![
            CombatDamageAssignment {
                recipient: Target::Permanent(first_id),
                amount: 1,
            },
            CombatDamageAssignment {
                recipient: Target::Permanent(second_id),
                amount: 3,
            },
        ],
    };
    assert!(game.legal_actions(PlayerId::One).contains(&assignment));
    game.apply(PlayerId::One, assignment).unwrap();

    let first = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == first_id)
        .unwrap();
    assert_eq!(first.damage, 1);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != second_id)
    );
    let attacker = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == attacker_id)
        .unwrap();
    assert_eq!(attacker.damage, 2);
}

#[test]
fn first_strike_kills_a_normal_blocker_before_it_can_hit_back() {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    let mut knight = creature(10_000, cards::BLACK_KNIGHT, PlayerId::One);
    knight.attacking = true;
    let knight_id = knight.card.id;
    let mut blocker = creature(10_001, cards::ATOG, PlayerId::Two);
    blocker.blocking = Some(knight_id);
    let blocker_id = blocker.card.id;
    game.battlefield = vec![knight, blocker];
    let opponent_life = game.players[1].life;

    game.advance_step();

    assert_eq!(game.step, Step::CombatDamage);
    assert!(game.regular_combat_damage_pending());
    assert!(
        game.observe(PlayerId::One).regular_combat_damage_pending,
        "the public observation distinguishes the priority window between damage waves",
    );
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != blocker_id),
        "the 2/2 first striker kills the 1/2 blocker in the strike wave",
    );
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == knight_id)
            .unwrap()
            .damage,
        0,
        "the normal blocker does not hit back in the strike wave",
    );

    pass_priority_pair(&mut game);

    assert_eq!(
        game.step,
        Step::CombatDamage,
        "a second combat-damage step begins after both players get priority",
    );
    assert!(!game.regular_combat_damage_pending());
    assert!(
        !game.observe(PlayerId::One).regular_combat_damage_pending,
        "ordinary priority after regular damage is not an inter-wave window",
    );
    assert_eq!(
        game.players[1].life, opponent_life,
        "killing the blocker does not make the first striker unblocked later",
    );
    assert_eq!(
        game.events()
            .iter()
            .filter(|event| matches!(
                event,
                GameEvent::StepChanged {
                    step: Step::CombatDamage,
                    ..
                }
            ))
            .count(),
        2,
        "both strike waves are observable as CombatDamage steps",
    );
}

#[test]
fn delayed_combat_damage_effect_queued_between_strike_waves_fires_once() {
    const LOSE_ONE: EffectDef = EffectDef::LoseLife {
        recipient: EffectRecipientDef::Controller,
        amount: ValueDef::Constant(1),
    };

    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    let mut attacker = creature(10_000, cards::BLACK_KNIGHT, PlayerId::One);
    attacker.attacking = true;
    game.battlefield.push(attacker);
    game.advance_step();
    assert!(
        game.regular_combat_damage_pending(),
        "the first-strike wave leaves an inter-wave priority window",
    );

    let life_before = game.players[0].life;
    game.delayed_triggers.push(DelayedTrigger {
        object: Box::new(spell(10_001, cards::LIGHTNING_BOLT, PlayerId::One, 0)),
        context: TriggerContext::empty(),
        step: TurnStepDef::CombatDamage,
        player: PlayerRelation::Any,
        effect: ScopedEffect::primary(LOSE_ONE),
    });

    pass_priority_pair(&mut game);

    assert_eq!(game.step, Step::CombatDamage);
    assert!(
        !game.regular_combat_damage_pending(),
        "the regular combat-damage step has begun",
    );
    assert_eq!(game.players[0].life, life_before - 1);
    assert!(game.delayed_triggers.is_empty());
}

#[test]
fn first_strike_blocker_kills_a_normal_attacker_before_it_deals_damage() {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    let mut attacker = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One);
    attacker.attacking = true;
    let attacker_id = attacker.card.id;
    let mut knight = creature(10_001, cards::BLACK_KNIGHT, PlayerId::Two);
    knight.blocking = Some(attacker_id);
    let knight_id = knight.card.id;
    game.battlefield = vec![attacker, knight];

    game.advance_step();

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != attacker_id),
        "the normal attacker dies during the first-strike damage step",
    );
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == knight_id)
            .unwrap()
            .damage,
        0,
        "the normal attacker never deals its combat damage",
    );
}

#[test]
fn double_strike_hits_an_unblocked_player_in_both_damage_steps() {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    let mut attacker = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One);
    attacker.attacking = true;
    attacker
        .temporary_keywords
        .push(KeywordAbility::DoubleStrike);
    game.battlefield.push(attacker);
    let life_before = game.players[1].life;

    game.advance_step();
    assert_eq!(game.players[1].life, life_before - 2);

    pass_priority_pair(&mut game);
    assert_eq!(game.step, Step::CombatDamage);
    assert_eq!(
        game.players[1].life,
        life_before - 4,
        "double strike deals damage once in each combat-damage step",
    );
}

#[test]
fn double_striker_stays_blocked_after_killing_its_only_blocker() {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    let mut attacker = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One);
    attacker.attacking = true;
    attacker
        .temporary_keywords
        .push(KeywordAbility::DoubleStrike);
    let attacker_id = attacker.card.id;
    let mut blocker = creature(10_001, cards::ATOG, PlayerId::Two);
    blocker.blocking = Some(attacker_id);
    let blocker_id = blocker.card.id;
    game.battlefield = vec![attacker, blocker];
    let life_before = game.players[1].life;

    game.advance_step();
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != blocker_id),
        "the blocker dies in the first damage step",
    );
    pass_priority_pair(&mut game);

    assert_eq!(
        game.players[1].life, life_before,
        "a blocked nontrampling attacker cannot redirect its second hit to the player",
    );
}

#[test]
fn double_striker_can_trample_after_killing_its_only_blocker() {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    let mut attacker = creature(10_000, cards::BALL_LIGHTNING, PlayerId::One);
    attacker.attacking = true;
    attacker
        .temporary_keywords
        .push(KeywordAbility::DoubleStrike);
    let attacker_id = attacker.card.id;
    let mut blocker = creature(10_001, cards::ATOG, PlayerId::Two);
    blocker.blocking = Some(attacker_id);
    game.battlefield = vec![attacker, blocker];
    let life_before = game.players[1].life;

    game.advance_step();
    take_default_combat_assignment(&mut game);
    assert_eq!(
        game.players[1].life,
        life_before - 4,
        "the strike wave assigns lethal to the blocker and tramples over",
    );

    pass_priority_pair(&mut game);
    take_default_combat_assignment(&mut game);
    assert_eq!(
        game.players[1].life,
        life_before - 10,
        "trample can assign the whole second hit after every blocker has left",
    );
}

#[test]
fn double_strike_recomputes_multi_blocker_assignment_for_the_second_step() {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    let mut attacker = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One);
    attacker.attacking = true;
    attacker
        .temporary_keywords
        .push(KeywordAbility::DoubleStrike);
    let attacker_id = attacker.card.id;
    let mut first = creature(10_001, cards::SERRA_ANGEL, PlayerId::Two);
    first.blocking = Some(attacker_id);
    let mut second = creature(10_002, cards::SERRA_ANGEL, PlayerId::Two);
    second.blocking = Some(attacker_id);
    let mut blocker_ids = [first.card.id, second.card.id];
    blocker_ids.sort_unstable();
    game.battlefield = vec![attacker, first, second];

    game.advance_step();
    let first_assignment = Action::AssignCombatDamage {
        attacker: attacker_id,
        assignments: blocker_ids
            .iter()
            .copied()
            .zip([2, 0])
            .map(|(recipient, amount)| CombatDamageAssignment {
                recipient: Target::Permanent(recipient),
                amount,
            })
            .collect(),
    };
    assert!(
        game.legal_actions(PlayerId::One)
            .contains(&first_assignment)
    );
    game.apply(PlayerId::One, first_assignment).unwrap();
    assert!(
        !game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == attacker_id)
            .unwrap()
            .combat_damage_assignment
            .is_empty(),
    );

    pass_priority_pair(&mut game);

    assert_eq!(game.pending_combat_attackers, vec![attacker_id]);
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == attacker_id)
            .unwrap()
            .combat_damage_assignment
            .is_empty(),
        "the first wave's assignment cannot leak into the regular wave",
    );
    assert!(
        game.legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::AssignCombatDamage { attacker, .. } if *attacker == attacker_id)),
        "the still-double-striking attacker assigns again against the surviving blockers",
    );
}

#[test]
fn first_strike_step_does_not_prompt_an_ineligible_multi_blocked_attacker() {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    let mut first_striker = creature(10_000, cards::BLACK_KNIGHT, PlayerId::One);
    first_striker.attacking = true;
    let mut normal_attacker = creature(10_001, cards::SU_CHI, PlayerId::One);
    normal_attacker.attacking = true;
    let normal_id = normal_attacker.card.id;
    let mut first_blocker = creature(10_002, cards::SERRA_ANGEL, PlayerId::Two);
    first_blocker.blocking = Some(normal_id);
    let mut second_blocker = creature(10_003, cards::SERRA_ANGEL, PlayerId::Two);
    second_blocker.blocking = Some(normal_id);
    game.battlefield = vec![
        first_striker,
        normal_attacker,
        first_blocker,
        second_blocker,
    ];

    game.advance_step();

    assert!(
        game.pending_combat_attackers.is_empty(),
        "the normal attacker is not asked to assign during the strike wave",
    );
    pass_priority_pair(&mut game);
    assert_eq!(
        game.pending_combat_attackers,
        vec![normal_id],
        "the normal attacker assigns when the regular damage step begins",
    );
}

#[test]
fn losing_double_strike_between_damage_steps_prevents_the_second_hit() {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    let mut attacker = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::One);
    attacker.attacking = true;
    attacker
        .temporary_keywords
        .push(KeywordAbility::DoubleStrike);
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);
    let life_before = game.players[1].life;

    game.advance_step();
    assert_eq!(game.players[1].life, life_before - 2);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == attacker_id)
        .unwrap()
        .temporary_keywords
        .retain(|keyword| *keyword != KeywordAbility::DoubleStrike);

    pass_priority_pair(&mut game);

    assert_eq!(
        game.players[1].life,
        life_before - 2,
        "a combatant in the strike-wave snapshot needs double strike now to hit again",
    );
}

#[test]
fn a_normal_attacker_that_gains_a_strike_keyword_still_hits_in_the_regular_wave() {
    for gained_keyword in [KeywordAbility::FirstStrike, KeywordAbility::DoubleStrike] {
        let mut game = ready_game();
        game.step = Step::DeclareBlockers;
        let mut first_striker = creature(10_000, cards::BLACK_KNIGHT, PlayerId::One);
        first_striker.attacking = true;
        let mut normal_attacker = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One);
        normal_attacker.attacking = true;
        let normal_id = normal_attacker.card.id;
        game.battlefield = vec![first_striker, normal_attacker];

        game.advance_step();
        let life_after_first_wave = game.players[1].life;
        game.battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == normal_id)
            .unwrap()
            .temporary_keywords
            .push(gained_keyword);

        pass_priority_pair(&mut game);

        assert_eq!(
            game.players[1].life,
            life_after_first_wave - 2,
            "a normal combatant that gains {gained_keyword:?} after the strike wave remains eligible for regular damage",
        );
    }
}

#[test]
fn a_first_striker_that_gains_double_strike_hits_in_the_regular_wave() {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    let mut attacker = creature(10_000, cards::BLACK_KNIGHT, PlayerId::One);
    attacker.attacking = true;
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);
    let life_before = game.players[1].life;

    game.advance_step();
    assert_eq!(game.players[1].life, life_before - 2);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == attacker_id)
        .unwrap()
        .temporary_keywords
        .push(KeywordAbility::DoubleStrike);

    pass_priority_pair(&mut game);

    assert_eq!(
        game.players[1].life,
        life_before - 4,
        "gaining double strike makes a first-wave combatant eligible again",
    );
}

#[test]
fn a_single_blocker_without_trample_needs_no_damage_assignment() {
    // Nothing to decide: the blocker takes all of it either way. A trampler
    // in the same spot does get asked, because how much spills past is a real
    // choice -- see a_lone_blocker_still_asks_a_trampler_how_much_spills.
    let mut game = ready_game();
    let mut attacker = creature(10_000, cards::SU_CHI, PlayerId::One);
    attacker.attacking = true;
    let mut blocker = creature(10_001, cards::ATOG, PlayerId::Two);
    blocker.blocking = Some(attacker.card.id);
    let blocker_id = blocker.card.id;
    game.battlefield = vec![attacker, blocker];
    let life_before = game.players[1].life;
    game.begin_combat_damage_assignment();

    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::AssignCombatDamage { .. })),
        "one blocker and no trample leaves nothing worth deciding",
    );
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != blocker_id),
        "the blocker still takes lethal damage",
    );
    assert_eq!(
        game.players[1].life, life_before,
        "and without trample none of it reaches the player",
    );
}

#[test]
fn a_lone_blocker_still_asks_a_trampler_how_much_spills() {
    // 510.1c lets the attacker assign more than lethal to the blocker, so a
    // 6/1 trampler over a 1/2 has a real decision even though only one
    // creature is in the way.
    let mut game = ready_game();
    let mut attacker = creature(10_000, cards::BALL_LIGHTNING, PlayerId::One);
    attacker.attacking = true;
    let attacker_id = attacker.card.id;
    let mut blocker = creature(10_001, cards::ATOG, PlayerId::Two);
    blocker.blocking = Some(attacker_id);
    game.battlefield = vec![attacker, blocker];
    let life_before = game.players[1].life;
    game.begin_combat_damage_assignment();

    let offered: Vec<_> = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter(|action| matches!(action, Action::AssignCombatDamage { .. }))
        .collect();
    assert!(
        offered.len() > 1,
        "how much spills past the blocker is the attacker's call",
    );

    take_default_combat_assignment(&mut game);
    assert_eq!(
        game.players[1].life,
        life_before - 4,
        "the default split still gives the blocker lethal and tramples the rest",
    );
}

#[test]
fn trample_requires_lethal_assignment_before_player_damage() {
    let mut game = ready_game();
    let mut attacker = creature(10_000, cards::BALL_LIGHTNING, PlayerId::One);
    attacker.attacking = true;
    let mut first = creature(10_001, cards::ATOG, PlayerId::Two);
    first.blocking = Some(attacker.card.id);
    let mut second = creature(10_002, cards::GOBLIN_BALLOON_BRIGADE, PlayerId::Two);
    second.blocking = Some(attacker.card.id);
    let attacker_id = attacker.card.id;
    let (first_id, second_id) = (first.card.id, second.card.id);
    game.battlefield = vec![attacker, first, second];
    game.begin_combat_damage_assignment();

    let mut recipients = [Target::Permanent(first_id), Target::Permanent(second_id)];
    recipients.sort_unstable();
    let assignment = |to_first: u16, to_second: u16, to_player: u16| {
        let mut assignments: Vec<_> = recipients
            .iter()
            .copied()
            .zip([to_first, to_second])
            .map(|(recipient, amount)| CombatDamageAssignment { recipient, amount })
            .collect();
        assignments.push(CombatDamageAssignment {
            recipient: Target::Player(PlayerId::Two),
            amount: to_player,
        });
        Action::AssignCombatDamage {
            attacker: attacker_id,
            assignments,
        }
    };
    let actions = game.legal_actions(PlayerId::One);
    let lethal: Vec<u16> = recipients
        .iter()
        .map(|target| match target {
            Target::Permanent(id) => game.lethal_damage(*id),
            _ => 0,
        })
        .collect();
    let spare = 6 - lethal[0] - lethal[1];

    assert!(
        actions.contains(&assignment(lethal[0], lethal[1], spare)),
        "lethal to both blockers then trample over is legal",
    );
    assert!(
        !actions.contains(&assignment(lethal[0] - 1, lethal[1], spare + 1)),
        "trample cannot spill while a blocker is short of lethal",
    );
}

#[test]
fn damage_can_be_divided_freely_across_several_blockers() {
    let mut game = ready_game();
    let mut attacker = creature(10_000, cards::SU_CHI, PlayerId::One);
    attacker.attacking = true;
    let attacker_id = attacker.card.id;
    game.battlefield = vec![attacker];
    let mut ids = Vec::new();
    for index in 0..3 {
        let mut blocker = creature(10_001 + index, cards::ATOG, PlayerId::Two);
        blocker.blocking = Some(attacker_id);
        ids.push(blocker.card.id);
        game.battlefield.push(blocker);
    }
    ids.sort_unstable();
    game.begin_combat_damage_assignment();

    let assignment = |amounts: [u16; 3]| Action::AssignCombatDamage {
        attacker: attacker_id,
        assignments: ids
            .iter()
            .copied()
            .zip(amounts)
            .map(|(id, amount)| CombatDamageAssignment {
                recipient: Target::Permanent(id),
                amount,
            })
            .collect(),
    };
    let actions = game.legal_actions(PlayerId::One);

    // Su-Chi is 4/4 into three 1/2 blockers, so it can kill two of them.
    assert!(
        actions.contains(&assignment([2, 2, 0])),
        "killing two blockers outright is legal",
    );
    let divided = assignment([1, 1, 2]);
    assert!(
        actions.contains(&divided),
        "current CR 510.1c permits any division among several blockers",
    );
    game.apply(PlayerId::One, divided).unwrap();

    for id in &ids[..2] {
        let blocker = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == *id)
            .unwrap();
        assert_eq!(blocker.damage, 1);
    }
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != ids[2]),
        "the blocker assigned lethal damage dies",
    );
}
