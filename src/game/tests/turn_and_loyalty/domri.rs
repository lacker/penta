fn activate_domri_plus_one(game: &mut Game, domri: GameObjectId) {
    game.turn = 2;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    let plus_one = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, ability, .. }
                if *source == domri
                    && matches!(ability, AbilityOrigin::Printed { ability, .. } if *ability == AbilityId(0)))
        })
        .expect("Domri's +1 is offered");
    game.apply(PlayerId::One, plus_one).unwrap();
    pass_until_decision(game);
}

#[test]
fn domri_plus_one_filters_reveals_and_preserves_a_declined_or_ineligible_top_card() {
    let setup = |top| {
        let mut game = ready_game();
        game.players[0].library.clear();
        stack_library(&mut game, &[(19_100, top), (19_101, cards::LIGHTNING_BOLT)]);
        let domri = game
            .put_onto_battlefield(PlayerId::One, cards::DOMRI_RADE)
            .expect("cataloged");
        (game, domri)
    };

    let (mut taken, domri) = setup(cards::SAVANNAH_LIONS);
    activate_domri_plus_one(&mut taken, domri);
    let decision = taken
        .observe(PlayerId::One)
        .decision
        .expect("a creature top card may be taken");
    assert_eq!(decision.visibility, DecisionVisibility::Private);
    assert_eq!(decision.minimum, 0);
    assert_eq!(decision.maximum, 1);
    assert_eq!(decision.options.len(), 1);
    choose_decision_by_label(&mut taken, PlayerId::One, "Savannah Lions");
    assert!(
        taken.players[0]
            .hand
            .iter()
            .any(|card| { card.definition == cards::SAVANNAH_LIONS })
    );
    assert!(taken.events.iter().any(|event| matches!(
        event,
        GameEvent::CardRevealed {
            player: PlayerId::One,
            definition: cards::SAVANNAH_LIONS,
            ..
        }
    )));

    let (mut declined, domri) = setup(cards::SAVANNAH_LIONS);
    activate_domri_plus_one(&mut declined, domri);
    let decision = declined.observe(PlayerId::One).decision.unwrap();
    declined
        .apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: decision.id,
                options: Vec::new(),
            },
        )
        .unwrap();
    assert_eq!(
        declined.players[0].library.last().unwrap().definition,
        cards::SAVANNAH_LIONS
    );
    assert_eq!(
        declined.players[0].library.last().unwrap().id,
        GameObjectId(19_100),
        "declining does not create a library-to-library zone change"
    );
    assert!(
        !declined
            .events
            .iter()
            .any(|event| matches!(event, GameEvent::CardRevealed { .. }))
    );

    let (mut ineligible, domri) = setup(cards::LIGHTNING_BOLT);
    activate_domri_plus_one(&mut ineligible, domri);
    let decision = ineligible
        .observe(PlayerId::One)
        .decision
        .expect("the private inspection remains observable even with no legal selection");
    assert_eq!((decision.minimum, decision.maximum), (0, 0));
    assert_eq!(decision.options.len(), 1);
    assert_eq!(
        decision.options[0].members,
        vec![(
            GameObjectId(19_100),
            ObjectCharacteristics::card(cards::LIGHTNING_BOLT, CardPartId::PRIMARY),
        )],
        "the chooser sees the ineligible card they looked at",
    );
    assert!(
        ineligible.observe(PlayerId::Two).decision.is_none(),
        "the private inspection is hidden from the opponent",
    );
    ineligible
        .apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: decision.id,
                options: Vec::new(),
            },
        )
        .unwrap();
    assert_eq!(
        ineligible.players[0].library.last().unwrap().definition,
        cards::LIGHTNING_BOLT
    );
    assert!(
        !ineligible
            .events
            .iter()
            .any(|event| matches!(event, GameEvent::CardRevealed { .. }))
    );
}

#[test]
fn domri_fights_and_hands_out_an_emblem() {
    let mut game = ready_game();
    game.battlefield.clear();
    let domri = game
        .put_onto_battlefield(PlayerId::One, cards::DOMRI_RADE)
        .expect("cataloged");
    let mine = game
        .put_onto_battlefield(PlayerId::One, cards::SERRA_ANGEL)
        .expect("cataloged");
    let theirs = game
        .put_onto_battlefield(PlayerId::Two, cards::SAVANNAH_LIONS)
        .expect("cataloged");
    game.turn = 2;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;

    // A 4/4 fighting a 2/1: the Lions die and the Angel takes two.
    let fight = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, ability, targets, .. }
                if *source == domri
                    && matches!(ability, AbilityOrigin::Printed { ability, .. } if *ability == AbilityId(1))
                    && targets.iter().flat_map(TargetSelection::targets).any(|target| *target == Target::Permanent(mine))
                    && targets.iter().flat_map(TargetSelection::targets).any(|target| *target == Target::Permanent(theirs)))
        })
        .expect("the fight is offered");
    game.apply(PlayerId::One, fight).unwrap();
    drain_pending(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == theirs),
        "the smaller creature died"
    );
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == mine)
            .expect("the bigger one lived")
            .damage,
        2,
        "and took the power of what it fought"
    );

    // The emblem grants its keywords without being a permanent.
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == domri)
    {
        permanent.set_counters(CounterKind::Loyalty, 7);
        permanent.activated_loyalty_this_turn = false;
    }
    let ultimate = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, ability, .. }
                if *source == domri
                    && matches!(ability, AbilityOrigin::Printed { ability, .. } if *ability == AbilityId(2)))
        })
        .expect("the emblem ability is offered at seven loyalty");
    game.apply(PlayerId::One, ultimate).unwrap();
    drain_pending(&mut game);

    assert_eq!(game.emblems.len(), 1, "the ultimate creates one emblem");
    assert_eq!(
        game.emblems[0].card.definition,
        ObjectKind::Emblem,
        "an emblem is a creator-owned command-zone object, not a card",
    );
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == domri),
        "and paying the last loyalty left Domri behind"
    );
    let angel = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == mine)
        .expect("still there")
        .clone();
    for keyword in [
        KeywordAbility::DoubleStrike,
        KeywordAbility::Trample,
        KeywordAbility::Hexproof,
        KeywordAbility::Haste,
    ] {
        assert!(
            game.permanent_has_executable_keyword(&angel, keyword),
            "the emblem granted {keyword:?}"
        );
    }
}

#[test]
fn domri_fight_snapshots_both_powers_before_infect_changes_them() {
    let mut game = ready_game();
    game.battlefield.clear();
    let domri = game
        .put_onto_battlefield(PlayerId::One, cards::DOMRI_RADE)
        .expect("cataloged");
    let infect = game
        .put_onto_battlefield(PlayerId::One, cards::VIRAL_DRAKE)
        .expect("cataloged");
    let angel = game
        .put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("cataloged");
    game.turn = 2;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;

    let fight = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, ability, targets, .. }
                if *source == domri
                    && matches!(ability, AbilityOrigin::Printed { ability, .. } if *ability == AbilityId(1))
                    && targets.iter().flat_map(TargetSelection::targets).any(|target| *target == Target::Permanent(infect))
                    && targets.iter().flat_map(TargetSelection::targets).any(|target| *target == Target::Permanent(angel)))
        })
        .expect("the fight is offered");
    game.apply(PlayerId::One, fight).unwrap();
    drain_pending(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == infect),
        "the Drake takes the Angel's pre-fight power of four and dies"
    );
    let angel = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == angel)
        .expect("the Angel survives");
    assert_eq!(angel.counters(CounterKind::MinusOneMinusOne), 1);
    assert_eq!(angel.damage, 0, "infect uses a counter instead of a damage mark");
}
