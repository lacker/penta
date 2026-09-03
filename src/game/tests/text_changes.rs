use super::*;

fn text_change(word: TextWordChange, expiration: ContinuousEffectExpiration) -> TextChange {
    TextChange { word, expiration }
}

#[test]
fn text_changes_apply_in_timestamp_order() {
    let mut game = ready_game();
    let mut source = creature(10_000, cards::BOG_WRAITH, PlayerId::One);
    source.text_changes.extend([
        text_change(
            TextWordChange::BasicLandType {
                from: BasicLandType::Swamp,
                to: BasicLandType::Island,
            },
            ContinuousEffectExpiration::Never,
        ),
        text_change(
            TextWordChange::BasicLandType {
                from: BasicLandType::Island,
                to: BasicLandType::Forest,
            },
            ContinuousEffectExpiration::Never,
        ),
    ]);
    let source_id = source.card.id;
    game.battlefield.push(source);

    assert!(game.permanent_has_executable_keyword(
        &game.battlefield[0],
        KeywordAbility::Landwalk(BasicLandType::Forest),
    ));
    assert_eq!(
        game.text_changed_basic_land_type(source_id, BasicLandType::Swamp),
        BasicLandType::Forest,
    );
}

#[test]
fn sleight_of_mind_changes_color_words_in_printed_protection() {
    let mut game = ready_game();
    let knight_id = CardInstanceId(10_000);
    game.battlefield
        .push(creature(knight_id.0, cards::WHITE_KNIGHT, PlayerId::One));
    let red = creature(10_001, cards::HILL_GIANT, PlayerId::Two);
    let black = creature(10_002, cards::BLACK_KNIGHT, PlayerId::Two);
    let sleight = card(10_003, cards::SLEIGHT_OF_MIND, PlayerId::One);
    game.players[0].hand.push(sleight.clone());
    game.players[0].mana_pool.blue = 1;

    game.apply(
        PlayerId::One,
        cast_action(
            sleight.id,
            vec![Target::Permanent(knight_id)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    choose_decision_by_label(&mut game, PlayerId::One, "Black → Red");

    let knight = &game.battlefield[0];
    assert!(game.is_protected_from_characteristics(
        knight,
        &game.trigger_event_object(&red),
        false,
    ));
    assert!(!game.is_protected_from_characteristics(
        knight,
        &game.trigger_event_object(&black),
        false,
    ));
}

#[test]
fn text_changes_follow_the_source_of_a_granted_ability() {
    let mut game = ready_game();
    let host_id = CardInstanceId(10_000);
    let mut ward = creature(10_001, cards::BLACK_WARD, PlayerId::One);
    ward.attached_to = Some(host_id);
    ward.text_changes.push(text_change(
        TextWordChange::Color {
            from: ManaColor::Black,
            to: ManaColor::Red,
        },
        ContinuousEffectExpiration::Never,
    ));
    let red = creature(10_002, cards::HILL_GIANT, PlayerId::Two);
    let black = creature(10_003, cards::BLACK_KNIGHT, PlayerId::Two);
    game.battlefield
        .extend([creature(host_id.0, cards::SERRA_ANGEL, PlayerId::One), ward]);

    let host = &game.battlefield[0];
    assert!(game.is_protected_from_characteristics(host, &game.trigger_event_object(&red), false,));
    assert!(!game.is_protected_from_characteristics(
        host,
        &game.trigger_event_object(&black),
        false,
    ));
}

#[test]
fn crystal_spray_resumes_with_its_draw_and_expires_at_cleanup() {
    let mut game = ready_game();
    let target_id = CardInstanceId(10_000);
    game.battlefield
        .push(creature(target_id.0, cards::WHITE_KNIGHT, PlayerId::Two));
    let drawn = card(10_001, cards::BLACK_LOTUS, PlayerId::One);
    game.players[0].library.push(drawn.clone());
    let spray = card(10_002, cards::CRYSTAL_SPRAY, PlayerId::One);
    game.players[0].hand.push(spray.clone());
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 2;

    game.apply(
        PlayerId::One,
        cast_action(spray.id, vec![Target::Permanent(target_id)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    assert!(
        game.players[0].hand.is_empty(),
        "the draw waits on the word choice"
    );
    choose_decision_by_label(&mut game, PlayerId::One, "Black → Red");
    assert_eq!(game.players[0].hand.len(), 1);
    assert_eq!(game.players[0].hand[0].definition, drawn.definition);
    assert_eq!(game.battlefield[0].text_changes.len(), 1);

    game.finish_cleanup();
    assert!(game.battlefield[0].text_changes.is_empty());
}

#[test]
fn mind_bend_offers_both_word_domains_but_only_one_replacement() {
    let mut game = ready_game();
    let target_id = CardInstanceId(10_000);
    game.battlefield
        .push(creature(target_id.0, cards::WHITE_KNIGHT, PlayerId::Two));
    let bend = card(10_001, cards::MIND_BEND, PlayerId::One);
    game.players[0].hand.push(bend.clone());
    game.players[0].mana_pool.blue = 1;

    game.apply(
        PlayerId::One,
        cast_action(bend.id, vec![Target::Permanent(target_id)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    let decision = game.observe(PlayerId::One).decision.expect("word choice");
    assert_eq!(decision.minimum, 1);
    assert_eq!(decision.maximum, 1);
    assert!(
        decision
            .options
            .iter()
            .any(|option| option.label == "Swamp → Island")
    );
    assert!(
        decision
            .options
            .iter()
            .any(|option| option.label == "Black → Red")
    );
}

#[test]
fn a_changed_token_instruction_bakes_the_word_into_copiable_values() {
    let mut game = ready_game();
    let moan = card(10_000, cards::MOAN_OF_THE_UNHALLOWED, PlayerId::One);
    game.players[0].hand.push(moan.clone());
    game.players[0].mana_pool.black = 2;
    game.players[0].mana_pool.colorless = 2;
    game.apply(
        PlayerId::One,
        cast_action(moan.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    game.stack
        .iter_mut()
        .next_back()
        .expect("Moan is on the stack")
        .text_changes
        .push(text_change(
            TextWordChange::Color {
                from: ManaColor::Black,
                to: ManaColor::Red,
            },
            ContinuousEffectExpiration::Never,
        ));

    pass_priority_pair(&mut game);
    let tokens = game
        .battlefield
        .iter()
        .filter(|permanent| permanent.card.definition == ObjectKind::Token)
        .collect::<Vec<_>>();
    assert_eq!(tokens.len(), 2);
    for token in tokens {
        assert_eq!(
            game.permanent_colors(token),
            [false, false, false, true, false]
        );
        let CharacteristicSource::Token(characteristics) = token.card.characteristics else {
            panic!("the created permanent is backed by token characteristics");
        };
        assert_eq!(characteristics.color_word(ManaColor::Black), ManaColor::Red);
    }
}

#[test]
fn magical_hack_changes_land_type_words_inside_static_effects() {
    let mut game = ready_game();
    let mut urborg = creature(10_000, cards::URBORG_TOMB_OF_YAWGMOTH, PlayerId::One);
    urborg.text_changes.push(text_change(
        TextWordChange::BasicLandType {
            from: BasicLandType::Swamp,
            to: BasicLandType::Island,
        },
        ContinuousEffectExpiration::Never,
    ));
    game.battlefield
        .extend([urborg, creature(10_001, cards::PLAINS, PlayerId::Two)]);

    let plains = &game.battlefield[1];
    assert!(game.effective_subtypes(plains).contains(&"Plains"));
    assert!(game.effective_subtypes(plains).contains(&"Island"));
    assert!(!game.effective_subtypes(plains).contains(&"Swamp"));
}

#[test]
fn changing_a_spell_revalidates_its_targets_from_the_changed_text() {
    let mut game = ready_game();
    let knight_id = CardInstanceId(10_000);
    game.battlefield
        .push(creature(knight_id.0, cards::BOG_WRAITH, PlayerId::Two));
    let purge = card(10_001, cards::CELESTIAL_PURGE, PlayerId::One);
    let sleight = card(10_002, cards::SLEIGHT_OF_MIND, PlayerId::One);
    game.players[0]
        .hand
        .extend([purge.clone(), sleight.clone()]);
    game.players[0].mana_pool.white = 1;
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 1;

    game.apply(
        PlayerId::One,
        cast_action(purge.id, vec![Target::Permanent(knight_id)], Vec::new(), 0),
    )
    .unwrap();
    let purge_id = game
        .stack
        .iter()
        .find(|object| object.card.definition == cards::CELESTIAL_PURGE)
        .expect("Purge is on the stack")
        .id;
    game.apply(
        PlayerId::One,
        cast_action(sleight.id, vec![Target::Spell(purge_id)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    choose_decision_by_label(&mut game, PlayerId::One, "Black → Red");
    pass_priority_pair(&mut game);

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == knight_id)
    );
}
