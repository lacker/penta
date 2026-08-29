use super::*;

fn definition(game: &Game, name: &str) -> CardDefinitionId {
    game.catalog
        .find_by_name(name)
        .unwrap_or_else(|| panic!("{name} is cataloged"))
}

fn settle_accepting_may(game: &mut Game) {
    for _ in 0..24 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = decision
                .options
                .iter()
                .find(|option| option.label != "Decline")
                .or_else(|| decision.options.first())
                .map(|option| vec![option.id])
                .unwrap_or_default();
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .expect("the pending choice accepts an offered option");
            continue;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            return;
        }
        let player = game.priority;
        game.apply(player, Action::PassPriority)
            .expect("priority advances while settling");
    }
    panic!("the fight sequence did not settle");
}

#[test]
fn grothama_grants_each_other_creature_its_own_optional_attack_fight() {
    let mut game = ready_game();
    game.battlefield.clear();
    let grothama = game
        .put_onto_battlefield(PlayerId::Two, definition(&game, "Grothama, All-Devouring"))
        .expect("cataloged");
    let drake = game
        .put_onto_battlefield(PlayerId::One, cards::VIRAL_DRAKE)
        .expect("cataloged");
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [2, 1];
    drain_pending(&mut game);
    game.active_player = PlayerId::One;
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.priority = PlayerId::One;

    game.apply(
        PlayerId::One,
        Action::DeclareAttacker {
            attacker: drake,
            defender: AttackDefender::Player(PlayerId::Two),
        },
    )
    .unwrap();
    game.apply(PlayerId::One, Action::FinishDeclaringAttackers)
        .unwrap();
    settle_accepting_may(&mut game);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != drake),
        "the attacking Drake takes Grothama's snapshotted ten power",
    );
    let grothama = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == grothama)
        .expect("Grothama survives");
    assert_eq!(grothama.counters(CounterKind::MinusOneMinusOne), 1);
}

#[test]
fn unnatural_aggression_exiles_the_opposing_creature_that_dies_in_the_fight() {
    let mut game = ready_game();
    let mine = game
        .put_onto_battlefield(PlayerId::One, cards::SERRA_ANGEL)
        .expect("cataloged");
    let theirs = game
        .put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    let aggression = definition(&game, "Unnatural Aggression");
    let spell = card(30_000, aggression, PlayerId::One);
    game.players[0].hand.push(spell.clone());
    game.players[0].mana_pool.green = 1;
    game.players[0].mana_pool.colorless = 2;

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == spell.id
                    && choices.targets().iter().flat_map(TargetSelection::targets).any(|target| *target == Target::Permanent(mine))
                    && choices.targets().iter().flat_map(TargetSelection::targets).any(|target| *target == Target::Permanent(theirs)))
        })
        .expect("the two-target spell is offered");
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);

    assert!(
        game.players[1]
            .exile
            .iter()
            .any(|card| card.definition == cards::SAVANNAH_LIONS),
        "the replacement is installed before state-based actions move the fight casualty",
    );
    assert!(game.players[1].graveyard.is_empty());
}

#[test]
fn the_last_agni_kai_adds_the_exact_excess_from_the_simultaneous_fight() {
    let mut game = ready_game();
    let mine = game
        .put_onto_battlefield(PlayerId::One, cards::SERRA_ANGEL)
        .expect("cataloged");
    let theirs = game
        .put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    let agni_kai = definition(&game, "The Last Agni Kai");
    let spell = card(30_001, agni_kai, PlayerId::One);
    game.players[0].hand.push(spell.clone());
    game.players[0].mana_pool.red = 1;
    game.players[0].mana_pool.colorless = 1;

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == spell.id
                    && choices.targets().iter().flat_map(TargetSelection::targets).any(|target| *target == Target::Permanent(mine))
                    && choices.targets().iter().flat_map(TargetSelection::targets).any(|target| *target == Target::Permanent(theirs)))
        })
        .expect("the two-target spell is offered");
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(
        game.players[0].mana_pool.red, 3,
        "four damage to a creature needing one is three excess",
    );
}

#[test]
fn rhinos_rampage_applies_its_power_bonus_before_the_fight() {
    let mut game = ready_game();
    let mine = game
        .put_onto_battlefield(PlayerId::One, cards::SERRA_ANGEL)
        .expect("cataloged");
    let theirs = game
        .put_onto_battlefield(PlayerId::Two, cards::SEA_SERPENT)
        .expect("cataloged");
    let rampage = definition(&game, "Rhino's Rampage");
    let spell = card(30_002, rampage, PlayerId::One);
    game.players[0].hand.push(spell.clone());
    game.players[0].mana_pool.red = 1;

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == spell.id
                    && choices.targets().iter().flat_map(TargetSelection::targets).any(|target| *target == Target::Permanent(mine))
                    && choices.targets().iter().flat_map(TargetSelection::targets).any(|target| *target == Target::Permanent(theirs)))
        })
        .expect("the two-target spell is offered");
    game.apply(PlayerId::One, cast).unwrap();
    pass_priority_pair(&mut game);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != theirs),
        "the Angel fights with five power and kills the 5/5 Serpent",
    );
}

#[test]
fn a_creature_fighting_itself_deals_one_doubled_damage_assignment() {
    let mut game = ready_game();
    let reckoner = creature(10_000, cards::BOROS_RECKONER, PlayerId::One);
    game.battlefield.push(reckoner);
    let object = installing_object(0, PlayerId::One, Vec::new(), Vec::new(), 0);

    game.resolve_effect_def(
        ScopedEffect::primary(EffectDef::Fight {
            first: ObjectRefDef::Source,
            second: ObjectRefDef::Source,
            excess: None,
        }),
        &object,
        TriggerContext::empty(),
    );

    let reckoner = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == GameObjectId(10_000))
        .expect("state-based actions have not run during effect resolution");
    assert_eq!(reckoner.damage, 6, "self-fight deals twice its power");
    assert_eq!(
        game.pending_triggers.len(),
        1,
        "one doubled assignment creates one damage event for the recipient",
    );
}

#[test]
fn simultaneous_damage_reports_recipient_excess_from_all_sources() {
    let mut game = ready_game();
    let first = creature(10_010, cards::SAVANNAH_LIONS, PlayerId::One);
    let second = creature(10_011, cards::SAVANNAH_LIONS, PlayerId::One);
    let recipient = creature(10_012, cards::SEA_SERPENT, PlayerId::Two);
    let recipient_id = recipient.card.id;
    game.battlefield = vec![first, second, recipient];

    let outcome = game.deal_damage_simultaneously(vec![
        DamageAssignment {
            source: Some(GameObjectId(10_010)),
            target: Some(Target::Permanent(recipient_id)),
            amount: 2,
            combat: false,
        },
        DamageAssignment {
            source: Some(GameObjectId(10_011)),
            target: Some(Target::Permanent(recipient_id)),
            amount: 4,
            combat: false,
        },
    ]);

    assert_eq!(
        outcome.assignments,
        vec![
            DamageAssignmentOutcome {
                source: Some(GameObjectId(10_010)),
                recipient: Target::Permanent(recipient_id),
                amount: 2,
            },
            DamageAssignmentOutcome {
                source: Some(GameObjectId(10_011)),
                recipient: Target::Permanent(recipient_id),
                amount: 4,
            },
        ],
        "the event retains each source-specific damage result",
    );
    assert_eq!(
        outcome.recipients,
        vec![DamageRecipientOutcome {
            recipient: Target::Permanent(recipient_id),
            amount: 6,
            excess: 1,
        }],
        "the recipient took six total damage against a lethal threshold of five",
    );
}
