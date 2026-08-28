fn resolve_declarative_spell(game: &mut Game, object: &StackObject, definition: CardDefinitionId) {
    let effect = game
        .catalog
        .get(definition)
        .and_then(|card| card.rules.ability_clauses().first())
        .and_then(|ability| ability.declarative_effect())
        .expect("the spell has a declarative primary effect");
    game.resolve_effect_def(
        ScopedEffect::primary(effect),
        object,
        TriggerContext::empty(),
    );
}

#[test]
fn mulch_keeps_the_lands_and_bins_the_rest() {
    let mut game = ready_game();
    game.players[0].library.clear();
    stack_library(
        &mut game,
        &[
            (10_001, cards::MOUNTAIN),
            (10_002, cards::LIGHTNING_BOLT),
            (10_003, cards::MOUNTAIN),
            (10_004, cards::SAVANNAH_LIONS),
            (10_005, cards::BLACK_LOTUS), // fifth card is untouched
        ],
    );
    let before_hand = game.players[0].hand.len();

    let cast = spell(10_000, cards::MULCH, PlayerId::One, 0);
    resolve_declarative_spell(&mut game, &cast, cards::MULCH);

    assert_eq!(
        game.events
            .iter()
            .filter(|event| matches!(event, GameEvent::CardRevealed { .. }))
            .count(),
        4,
        "the inspected cards are publicly revealed",
    );

    assert_eq!(
        game.players[0].hand.len(),
        before_hand + 2,
        "two lands kept"
    );
    assert_eq!(game.players[0].graveyard.len(), 2, "two nonlands binned");
    assert_eq!(
        game.players[0].library.len(),
        1,
        "only the top four were revealed"
    );
}

#[test]
fn grisly_salvage_may_keep_one_creature_or_land() {
    let mut game = ready_game();
    game.players[0].library.clear();
    game.players[0].library.extend([
        card(10_001, cards::LIGHTNING_BOLT, PlayerId::One), // not eligible
        card(10_002, cards::SAVANNAH_LIONS, PlayerId::One), // creature
        card(10_003, cards::MOUNTAIN, PlayerId::One),       // land
        card(10_004, cards::BLACK_LOTUS, PlayerId::One),    // not eligible
        card(10_005, cards::COUNTERSPELL, PlayerId::One),   // not eligible
    ]);

    let cast = spell(10_000, cards::GRISLY_SALVAGE, PlayerId::One, 0);
    resolve_declarative_spell(&mut game, &cast, cards::GRISLY_SALVAGE);

    assert_eq!(
        game.events
            .iter()
            .filter(|event| matches!(event, GameEvent::CardRevealed { .. }))
            .count(),
        5,
        "the whole inspected group is revealed",
    );

    let decision = game.observe(PlayerId::One).decision.expect("a choice");
    assert_eq!(decision.options.len(), 2, "the creature and the land");
    assert_eq!(
        decision.minimum, 0,
        "'you may' means keeping nothing is legal"
    );

    let keep = decision
        .options
        .iter()
        .find(|option| {
            option
                .card
                .is_some_and(|(id, _)| id == CardInstanceId(10_003))
        })
        .expect("the land is offered")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![keep],
        },
    )
    .unwrap();

    // A zone change mints a new object id, so the card is identified by what
    // it is rather than by the id it had in the library.
    assert_eq!(game.players[0].hand.len(), 1);
    assert_eq!(
        game.players[0].hand[0].definition,
        cards::MOUNTAIN,
        "the chosen land reached hand"
    );
    assert_eq!(
        game.players[0].graveyard.len(),
        4,
        "the other four are binned"
    );
    assert!(game.players[0].library.is_empty());
}

#[test]
fn grisly_salvage_can_decline_and_bin_everything() {
    let mut game = ready_game();
    game.players[0].library.clear();
    game.players[0]
        .library
        .extend((0..5).map(|i| card(10_100 + i, cards::SAVANNAH_LIONS, PlayerId::One)));

    let cast = spell(10_000, cards::GRISLY_SALVAGE, PlayerId::One, 0);
    resolve_declarative_spell(&mut game, &cast, cards::GRISLY_SALVAGE);
    let decision = game.observe(PlayerId::One).decision.expect("a choice");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: Vec::new(),
        },
    )
    .expect("declining is legal");

    assert!(game.players[0].hand.is_empty(), "nothing was kept");
    assert_eq!(
        game.players[0].graveyard.len(),
        5,
        "and no revealed card was lost on the way"
    );
}

#[test]
fn augur_choice_shows_every_card_that_was_looked_at() {
    let mut game = ready_game();
    game.players[0].library.clear();
    stack_library(
        &mut game,
        &[
            (10_101, cards::MOUNTAIN),
            (10_102, cards::COUNTERSPELL),
            (10_103, cards::SAVANNAH_LIONS),
        ],
    );

    let augur = spell(10_100, cards::AUGUR_OF_BOLAS, PlayerId::One, 0);
    resolve_declarative_spell(&mut game, &augur, cards::AUGUR_OF_BOLAS);

    let decision = game.observe(PlayerId::One).decision.expect("a private choice");
    assert_eq!(decision.options.len(), 1, "only the instant is eligible");
    assert_eq!(
        decision.options[0]
            .members
            .iter()
            .filter_map(|(_, characteristics)| characteristics.card_definition())
            .collect::<Vec<_>>(),
        vec![
            cards::MOUNTAIN,
            cards::COUNTERSPELL,
            cards::SAVANNAH_LIONS,
        ],
        "the choice displays the complete inspected group, not only eligible cards",
    );
    assert!(
        game.observe(PlayerId::Two).decision.is_none(),
        "the opponent does not see a private look",
    );
}

#[test]
fn augur_without_a_match_still_shows_the_looked_at_cards() {
    let mut game = ready_game();
    game.players[0].library.clear();
    stack_library(
        &mut game,
        &[
            (10_111, cards::MOUNTAIN),
            (10_112, cards::SAVANNAH_LIONS),
            (10_113, cards::BLACK_LOTUS),
        ],
    );

    let augur = spell(10_110, cards::AUGUR_OF_BOLAS, PlayerId::One, 0);
    resolve_declarative_spell(&mut game, &augur, cards::AUGUR_OF_BOLAS);

    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the private look needs an acknowledgement");
    assert_eq!((decision.minimum, decision.maximum), (0, 0));
    assert_eq!(decision.options.len(), 1);
    assert_eq!(decision.options[0].members.len(), 3);

    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: Vec::new(),
        },
    )
    .expect("the player can finish looking");

    let order = game
        .observe(PlayerId::One)
        .decision
        .expect("all three cards still have to be ordered for the bottom");
    assert_eq!(order.options.len(), 3);
    assert_eq!(
        order.order_semantics,
        Some(DecisionOrderSemantics::Resolution),
    );
}

#[test]
fn terminus_asks_each_owner_in_apnap_order_then_bottoms_the_batches() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].library = vec![card(10_100, cards::MOUNTAIN, PlayerId::One)];
    game.players[1].library = vec![card(10_101, cards::MOUNTAIN, PlayerId::Two)];
    let mut stolen_angel = creature(10_002, cards::SERRA_ANGEL, PlayerId::One);
    stolen_angel.controller = PlayerId::Two;
    game.battlefield.extend([
        creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One),
        stolen_angel,
        creature(10_003, cards::JUZAM_DJINN, PlayerId::Two),
        creature(10_004, cards::VAMPIRE_NIGHTHAWK, PlayerId::Two),
    ]);

    let cast = spell(10_000, cards::TERMINUS, PlayerId::One, 0);
    resolve_declarative_spell(&mut game, &cast, cards::TERMINUS);

    let first = game.observe(PlayerId::One).decision.expect("active owner orders first");
    assert_eq!(first.order_semantics, Some(DecisionOrderSemantics::Resolution));
    assert_eq!(first.visibility, DecisionVisibility::Private);
    assert!(game.observe(PlayerId::Two).decision.is_none());
    let first_order = [cards::SERRA_ANGEL, cards::SAVANNAH_LIONS]
        .iter()
        .map(|definition| {
            first
                .options
                .iter()
                .find(|option| {
                    option.card.is_some_and(|(_, characteristics)| {
                        characteristics.card_definition() == Some(*definition)
                    })
                })
                .expect("owned creature is offered")
                .id
        })
        .collect();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: first.id,
            options: first_order,
        },
    )
    .expect("first owner can arrange their cards");

    let second = game.observe(PlayerId::Two).decision.expect("nonactive owner orders next");
    let second_order = [cards::VAMPIRE_NIGHTHAWK, cards::JUZAM_DJINN]
        .iter()
        .map(|definition| {
            second
                .options
                .iter()
                .find(|option| {
                    option.card.is_some_and(|(_, characteristics)| {
                        characteristics.card_definition() == Some(*definition)
                    })
                })
                .expect("owned creature is offered")
                .id
        })
        .collect();
    game.apply(
        PlayerId::Two,
        Action::ChooseDecision {
            decision: second.id,
            options: second_order,
        },
    )
    .expect("second owner can arrange their cards");

    assert!(game.battlefield.is_empty(), "the simultaneous batch has committed");
    assert_eq!(
        game.players[0]
            .library
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::SERRA_ANGEL, cards::SAVANNAH_LIONS, cards::MOUNTAIN],
    );
    assert_eq!(
        game.players[1]
            .library
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::VAMPIRE_NIGHTHAWK, cards::JUZAM_DJINN, cards::MOUNTAIN],
    );
}

#[test]
fn sphinxs_revelation_scales_life_and_cards_with_x() {
    let mut game = ready_game();
    let before_life = game.players[0].life;
    game.players[0].library.clear();
    game.players[0].library.extend([
        card(10_100, cards::PLAINS, PlayerId::One),
        card(10_101, cards::ISLAND, PlayerId::One),
        card(10_102, cards::FOREST, PlayerId::One),
    ]);
    let revelation = card(10_000, cards::SPHINXS_REVELATION, PlayerId::One);
    game.players[0].hand.push(revelation.clone());
    game.players[0].mana_pool.white = 1;
    game.players[0].mana_pool.blue = 2;
    game.players[0].mana_pool.colorless = 3;

    game.apply(
        PlayerId::One,
        cast_action(revelation.id, Vec::new(), Vec::new(), 3),
    )
    .expect("Sphinx's Revelation can be cast for X equals three");
    pass_priority_pair(&mut game);

    assert_eq!(game.players[0].life, before_life + 3);
    assert_eq!(game.players[0].hand.len(), 3);
    assert!(game.players[0].library.is_empty());
}

#[test]
fn the_mana_creatures_tap_for_their_colour() {
    // Their whole printed text is a mana ability the engine already models,
    // so they are complete rather than staged.
    for (definition, expected) in [
        (cards::AVACYNS_PILGRIM, ManaColor::White),
        (cards::ELVISH_MYSTIC, ManaColor::Green),
    ] {
        let mut game = ready_game();
        game.battlefield
            .push(creature(10_001, definition, PlayerId::One));
        assert!(
            game.legal_actions(PlayerId::One)
                .iter()
                .any(|action| matches!(
                    action,
                    Action::ActivateManaAbility { color, .. } if *color == expected
                )),
            "{definition:?} taps for {expected:?}"
        );
    }
}

#[test]
fn deathtouch_kills_whatever_it_touches_and_lifelink_pays_its_controller() {
    // Vampire Nighthawk is a 2/3 flying deathtouch lifelink. Before these
    // keywords were read, it was a 2/3 flier and nothing more.
    let mut game = ready_game();
    game.step = Step::CombatDamage;
    let mut hawk = creature(10_001, cards::VAMPIRE_NIGHTHAWK, PlayerId::One);
    hawk.attacking = true;
    let mut wall = creature(10_002, cards::SERRA_ANGEL, PlayerId::Two); // 4/4
    wall.blocking = vec![CardInstanceId(10_001)];
    game.battlefield.extend([hawk, wall]);
    let before_life = game.players[0].life;

    game.deal_combat_damage();

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::SERRA_ANGEL),
        "two deathtouch damage is lethal to a 4/4"
    );
    assert_eq!(
        game.players[0].life,
        before_life + 2,
        "and lifelink paid its controller for the damage dealt"
    );
}

#[test]
fn lifelink_pays_for_damage_to_a_player_too() {
    let mut game = ready_game();
    game.step = Step::CombatDamage;
    let mut hawk = creature(10_001, cards::VAMPIRE_NIGHTHAWK, PlayerId::One);
    hawk.attacking = true;
    game.battlefield.push(hawk);
    let before = game.players[0].life;

    game.deal_combat_damage();

    assert_eq!(game.players[1].life, 18, "unblocked, it hits for two");
    assert_eq!(game.players[0].life, before + 2, "and gains that much");
}

#[test]
fn an_ordinary_creature_does_not_gain_life_or_kill_through_toughness() {
    let mut game = ready_game();
    game.step = Step::CombatDamage;
    let mut lions = creature(10_001, cards::SAVANNAH_LIONS, PlayerId::One); // 2/1 vanilla
    lions.attacking = true;
    let mut wall = creature(10_002, cards::SERRA_ANGEL, PlayerId::Two); // 4/4
    wall.blocking = vec![CardInstanceId(10_001)];
    game.battlefield.extend([lions, wall]);
    let before = game.players[0].life;

    game.deal_combat_damage();

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::SERRA_ANGEL),
        "two ordinary damage does not kill a 4/4"
    );
    assert_eq!(game.players[0].life, before, "and gains nobody any life");
}

#[test]
fn reach_blocks_fliers_without_flying() {
    // Ruric Thar has reach; a plain ground creature does not.
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    game.active_player = PlayerId::One;
    let mut flier = creature(10_001, cards::SERRA_ANGEL, PlayerId::One);
    flier.attacking = true;
    game.battlefield.push(flier);
    game.battlefield.push(creature(
        10_002,
        cards::RURIC_THAR_THE_UNBOWED,
        PlayerId::Two,
    ));
    game.battlefield
        .push(creature(10_003, cards::SAVANNAH_LIONS, PlayerId::Two));

    let blockers: Vec<_> = game
        .blocker_actions(PlayerId::Two)
        .into_iter()
        .filter_map(|action| match action {
            Action::DeclareBlocker { blocker, .. } => Some(blocker),
            _ => None,
        })
        .collect();
    assert!(
        blockers.contains(&CardInstanceId(10_002)),
        "reach can block a flier"
    );
    assert!(
        !blockers.contains(&CardInstanceId(10_003)),
        "a ground creature still cannot"
    );
}

#[test]
fn intimidate_only_lets_artifacts_and_matching_colours_block() {
    // Lifebane Zombie is black; only black or artifact creatures may block it.
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    game.active_player = PlayerId::One;
    let mut zombie = creature(10_001, cards::LIFEBANE_ZOMBIE, PlayerId::One);
    zombie.attacking = true;
    game.battlefield.push(zombie);
    game.battlefield
        .push(creature(10_002, cards::JUZAM_DJINN, PlayerId::Two)); // black
    game.battlefield
        .push(creature(10_003, cards::SAVANNAH_LIONS, PlayerId::Two)); // white
    game.battlefield
        .push(creature(10_004, cards::JUGGERNAUT, PlayerId::Two)); // artifact

    let blockers: Vec<_> = game
        .blocker_actions(PlayerId::Two)
        .into_iter()
        .filter_map(|action| match action {
            Action::DeclareBlocker { blocker, .. } => Some(blocker),
            _ => None,
        })
        .collect();
    assert!(
        blockers.contains(&CardInstanceId(10_002)),
        "a black creature shares a colour and may block"
    );
    assert!(
        !blockers.contains(&CardInstanceId(10_003)),
        "a white creature may not"
    );
    assert!(
        blockers.contains(&CardInstanceId(10_004)),
        "an artifact creature may block regardless of colour",
    );
}
