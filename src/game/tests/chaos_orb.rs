use super::*;

#[test]
fn chaos_orb_chooses_during_resolution_and_uses_a_seeded_success_trial() {
    let mut game = ready_game();
    game.rng = ReplayRng::new(0);
    let orb = creature(10_000, cards::CHAOS_ORB, PlayerId::One);
    let mut target = creature(10_001, cards::BLACK_VISE, PlayerId::Two);
    target.temporary_keywords.push(KeywordAbility::Hexproof);
    target.temporary_keywords.push(KeywordAbility::Shroud);
    let token = token_permanent(
        10_002,
        tokens::creature(&["Beast"], &[ManaColor::Green], 3, 3),
        PlayerId::Two,
    );
    let orb_id = orb.card.id;
    let target_id = target.card.id;
    let token_id = token.card.id;
    game.battlefield = vec![orb, target, token];
    game.players[0].mana_pool.colorless = 1;
    let action = Action::ActivateAbility {
        source: orb_id,
        ability: activated_ability_for(&game, orb_id, 0),
        targets: Vec::new(),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
    };
    assert!(game.legal_actions(PlayerId::One).contains(&action));

    game.apply(PlayerId::One, action).unwrap();

    assert_eq!(game.stack.len(), 1);
    assert_eq!(game.stack[0].kind, StackObjectKind::ActivatedAbility);
    assert!(game.stack[0].chosen_permanents.is_empty());
    assert_eq!(game.stack[0].target_count(), 0);
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == orb_id)
            .is_some_and(|permanent| permanent.tapped)
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == target_id)
    );
    pass_priority_pair(&mut game);
    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("Chaos Orb chooses only once its ability resolves");
    assert_eq!(decision.prompt, "Choose objects");
    assert_eq!(decision.options.len(), 2);
    assert!(
        decision
            .options
            .iter()
            .any(|option| option.card.is_some_and(|(id, _)| id == target_id)),
        "a non-targeting choice ignores hexproof and shroud",
    );
    assert!(
        decision
            .options
            .iter()
            .all(|option| option.card.is_none_or(|(id, _)| id != token_id)),
        "the 93/94 choice excludes tokens",
    );
    choose_decision_by_label(&mut game, PlayerId::One, "Black Vise");
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| ![orb_id, target_id].contains(&permanent.card.id))
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == token_id)
    );
    assert_eq!(game.players[0].mana_pool.total(), 0);
}

#[test]
fn a_failed_chaos_orb_flip_still_destroys_the_orb() {
    let mut game = ready_game();
    game.rng = ReplayRng::new(23);
    let orb = creature(10_000, cards::CHAOS_ORB, PlayerId::One);
    let target = creature(10_001, cards::BLACK_VISE, PlayerId::Two);
    let orb_id = orb.card.id;
    let target_id = target.card.id;
    game.battlefield = vec![orb, target];
    game.players[0].mana_pool.colorless = 1;

    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: orb_id,
            ability: activated_ability_for(&game, orb_id, 0),
            targets: Vec::new(),
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
        },
    )
    .unwrap();
    pass_priority_pair(&mut game);
    choose_decision_by_label(&mut game, PlayerId::One, "Black Vise");

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != orb_id)
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == target_id)
    );
}

#[test]
fn guardian_beast_preserves_chaos_orb_on_success_and_failure() {
    for (seed, target_destroyed) in [(0, true), (23, false)] {
        let mut game = ready_game();
        game.rng = ReplayRng::new(seed);
        let orb = creature(10_000, cards::CHAOS_ORB, PlayerId::One);
        let guardian = creature(10_001, cards::GUARDIAN_BEAST, PlayerId::One);
        let target = creature(10_002, cards::BLACK_VISE, PlayerId::Two);
        let orb_id = orb.card.id;
        let guardian_id = guardian.card.id;
        let target_id = target.card.id;
        game.battlefield = vec![orb, guardian, target];
        game.players[0].mana_pool.colorless = 1;

        game.apply(
            PlayerId::One,
            Action::ActivateAbility {
                source: orb_id,
                ability: activated_ability_for(&game, orb_id, 0),
                targets: Vec::new(),
                cost_objects: Vec::new(),
                x: 0,
                modes: Vec::new(),
            },
        )
        .unwrap();
        pass_priority_pair(&mut game);
        choose_decision_by_label(&mut game, PlayerId::One, "Black Vise");

        assert!(
            game.battlefield
                .iter()
                .any(|permanent| permanent.card.id == orb_id && permanent.tapped),
            "untapped Guardian Beast must save the Orb after seed {seed}",
        );
        assert!(
            game.battlefield
                .iter()
                .any(|permanent| permanent.card.id == guardian_id)
        );
        assert_eq!(
            game.battlefield
                .iter()
                .all(|permanent| permanent.card.id != target_id),
            target_destroyed,
        );
    }
}

#[test]
fn chaos_orb_destroying_its_guardian_removes_its_own_protection() {
    let mut game = ready_game();
    game.rng = ReplayRng::new(0);
    let orb = creature(10_000, cards::CHAOS_ORB, PlayerId::One);
    let guardian = creature(10_001, cards::GUARDIAN_BEAST, PlayerId::One);
    let target = creature(10_002, cards::BLACK_VISE, PlayerId::Two);
    let orb_id = orb.card.id;
    let guardian_id = guardian.card.id;
    let target_id = target.card.id;
    game.battlefield = vec![orb, guardian, target];
    game.players[0].mana_pool.colorless = 1;

    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: orb_id,
            ability: activated_ability_for(&game, orb_id, 0),
            targets: Vec::new(),
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
        },
    )
    .unwrap();
    pass_priority_pair(&mut game);
    choose_decision_by_label(&mut game, PlayerId::One, "Guardian Beast");

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| ![orb_id, guardian_id].contains(&permanent.card.id)),
        "the Beast dies first, so the Orb is no longer indestructible",
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == target_id)
    );
}

#[test]
fn chaos_orb_can_be_activated_the_turn_it_enters_using_untapped_mana() {
    let mut game = ready_game();
    let mut orb = creature(10_000, cards::CHAOS_ORB, PlayerId::One);
    let mut mountain = creature(10_001, cards::MOUNTAIN, PlayerId::One);
    let target = creature(10_002, cards::BLACK_VISE, PlayerId::Two);
    orb.entered_controller_turn = game.turns_started[PlayerId::One.index()];
    mountain.entered_controller_turn = game.turns_started[PlayerId::One.index()];
    let orb_id = orb.card.id;
    let mountain_id = mountain.card.id;
    game.battlefield = vec![orb, mountain, target];
    let action = Action::ActivateAbility {
        source: orb_id,
        ability: activated_ability_for(&game, orb_id, 0),
        targets: Vec::new(),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
    };

    assert!(game.legal_actions(PlayerId::One).contains(&action));
    game.apply(PlayerId::One, action).unwrap();

    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == orb_id)
            .is_some_and(|permanent| permanent.tapped)
    );
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == mountain_id)
            .is_some_and(|permanent| permanent.tapped)
    );
    assert_eq!(game.players[0].mana_pool.total(), 0);
    assert_eq!(game.stack.len(), 1);
}

#[test]
fn removing_chaos_orb_in_response_nullifies_its_flip() {
    let mut game = ready_game();
    let orb = creature(10_000, cards::CHAOS_ORB, PlayerId::One);
    let target = creature(10_001, cards::BLACK_VISE, PlayerId::Two);
    let shatter = card(10_002, cards::SHATTER, PlayerId::Two);
    let orb_id = orb.card.id;
    let target_id = target.card.id;
    game.battlefield = vec![orb, target];
    game.players[0].mana_pool.colorless = 1;
    game.players[1].hand.push(shatter.clone());
    game.players[1].mana_pool.red = 2;

    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: orb_id,
            ability: activated_ability_for(&game, orb_id, 0),
            targets: Vec::new(),
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
        },
    )
    .unwrap();
    game.apply(PlayerId::One, Action::PassPriority).unwrap();
    game.apply(
        PlayerId::Two,
        cast_action(shatter.id, vec![Target::Permanent(orb_id)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    assert_eq!(game.stack.len(), 1);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != orb_id)
    );

    pass_priority_pair(&mut game);

    assert!(game.stack.is_empty());
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == target_id)
    );
}
