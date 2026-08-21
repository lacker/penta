use super::*;

/// Every permanent a card in hand can legally be aimed at, read off the real
/// cast actions rather than a behavior-keyed target list.
fn castable_targets(game: &Game, player: PlayerId, spell: GameObjectId) -> Vec<GameObjectId> {
    game.legal_actions(player)
        .into_iter()
        .filter_map(|action| match action {
            Action::CastSpell { card, choices, .. } if card == spell => {
                choices.iter_targets().find_map(|target| match target {
                    Target::Permanent(id) => Some(*id),
                    _ => None,
                })
            }
            _ => None,
        })
        .collect()
}

#[test]
fn hexproof_stops_opponents_targeting_but_not_its_controller() {
    let mut game = ready_game();
    game.battlefield.push(creature(
        10_001,
        cards::SIGARDA_HOST_OF_HERONS,
        PlayerId::Two,
    ));

    for player in [PlayerId::One, PlayerId::Two] {
        let terror = card(
            20_000 + u32::from(player == PlayerId::Two),
            cards::TERROR,
            player,
        );
        game.players[player.index()].hand.push(terror.clone());
        game.add_unrestricted_mana(player, ManaColor::Black, 2);
        game.priority = player;
        let targets = castable_targets(&game, player, terror.id);
        if player == PlayerId::One {
            assert!(targets.is_empty(), "an opponent cannot target hexproof");
        } else {
            assert_eq!(
                targets,
                vec![GameObjectId(10_001)],
                "its own controller still can, hexproof only stops opponents"
            );
        }
    }
}

#[test]
fn undying_returns_the_creature_once_with_a_counter() {
    // Strangleroot Geist is a 2/1 with haste and undying.
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_001, cards::STRANGLEROOT_GEIST, PlayerId::One));

    game.destroy_permanent(CardInstanceId(10_001));

    assert_eq!(
        game.battlefield.len(),
        1,
        "it came back rather than staying dead"
    );
    let returned = &game.battlefield[0];
    assert_eq!(
        returned.counters[CounterKind::PlusOnePlusOne.index()],
        1,
        "with a +1/+1 counter"
    );
    assert_ne!(
        returned.card.id,
        CardInstanceId(10_001),
        "and as a new object, because it really did change zones"
    );
    assert!(
        game.players[0].graveyard.is_empty(),
        "the card left the graveyard on its way back"
    );

    // Second death: it has a counter now, so undying does not apply.
    let second = returned.card.id;
    game.destroy_permanent(second);
    assert!(game.battlefield.is_empty(), "it stays dead the second time");
    assert_eq!(game.players[0].graveyard.len(), 1);
}

#[test]
fn undying_returns_it_to_its_owner_not_whoever_killed_it() {
    let mut game = ready_game();
    let mut geist = creature(10_001, cards::STRANGLEROOT_GEIST, PlayerId::One);
    // Someone else has taken control of it.
    geist.controller = PlayerId::Two;
    game.battlefield.push(geist);

    game.destroy_permanent(CardInstanceId(10_001));

    assert_eq!(
        game.battlefield[0].controller,
        PlayerId::One,
        "undying returns it under its owner's control"
    );
}

#[test]
fn undying_return_finishes_entry_replacements_before_publishing_entry_triggers() {
    let mut game = ready_game();
    game.battlefield.extend([
        creature(10_000, cards::BLIND_OBEDIENCE, PlayerId::Two),
        creature(10_001, cards::BLIND_OBEDIENCE, PlayerId::Two),
    ]);
    let mut augur = creature(10_002, cards::AUGUR_OF_BOLAS, PlayerId::One);
    // Granting undying in the fixture lets a real ETB-triggered creature
    // exercise the graveyard-entry origin without adding a card-specific path.
    augur.temporary_keywords.push(KeywordAbility::Undying);
    game.battlefield.push(augur);
    let event_start = game.events().len();

    game.destroy_permanent(CardInstanceId(10_002));

    let order = game
        .observe(PlayerId::One)
        .decision
        .expect("the returning creature's controller orders both Blind Obedience effects");
    assert_eq!(order.options.len(), 2);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.definition != cards::AUGUR_OF_BOLAS),
        "the graveyard return remains prospective during replacement ordering"
    );
    assert!(game.events()[event_start..].iter().all(|event| !matches!(
        event,
        GameEvent::AbilityTriggered {
            presentation: ObjectCharacteristics::Card {
                definition: cards::AUGUR_OF_BOLAS,
                ..
            },
            ..
        }
    )));

    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: order.id,
            options: vec![order.options[0].id],
        },
    )
    .unwrap();

    let returned = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::AUGUR_OF_BOLAS)
        .expect("the replaced graveyard entry commits");
    assert!(returned.tapped);
    assert_eq!(returned.counters(CounterKind::PlusOnePlusOne), 1);
    assert!(game.pending_decisions.is_empty());
    assert!(game.events()[event_start..].iter().any(|event| matches!(
        event,
        GameEvent::AbilityTriggered {
            presentation: ObjectCharacteristics::Card {
                definition: cards::AUGUR_OF_BOLAS,
                ..
            },
            ..
        }
    )));
    assert_eq!(
        game.stack
            .iter()
            .filter(|object| object.kind == StackObjectKind::TriggeredAbility)
            .count(),
        1,
        "the ETB trigger is published once after the final entry commits"
    );
}

#[test]
fn a_plus_one_counter_boosts_stats_whatever_put_it_there() {
    // Strangleroot Geist is a 2/1; undying brings it back as a 3/2. Before
    // +1/+1 counters and javelin counters were separated, the stat bonus was
    // allowlisted to three named cards and this counter did nothing.
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_001, cards::STRANGLEROOT_GEIST, PlayerId::One));
    game.destroy_permanent(CardInstanceId(10_001));

    let returned = &game.battlefield[0];
    assert_eq!(game.power(returned), Some(3), "2/1 plus a counter is 3/2");
    assert_eq!(game.toughness(returned), Some(2));
}

#[test]
fn a_javelin_counter_is_not_a_plus_one_counter() {
    // Icatian Javelineers enters with a javelin counter and stays a 1/1.
    let mut game = ready_game();
    let id = game
        .put_onto_battlefield(PlayerId::One, cards::ICATIAN_JAVELINEERS)
        .expect("Icatian Javelineers is in the catalog");
    let javelineers = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("the entry replacement committed");
    assert_eq!(javelineers.counters(CounterKind::Javelin), 1);
    assert_eq!(
        game.power(javelineers),
        Some(1),
        "its counter is ammunition, not a stat boost"
    );
    assert_eq!(game.toughness(javelineers), Some(1));
}

#[test]
fn protection_reads_the_printed_colours_not_a_list_of_card_names() {
    // Blood Baron of Vizkopa has protection from white and from black. Its
    // data was already in the catalog; the engine simply never looked at it.
    let mut game = ready_game();
    game.battlefield.push(creature(
        10_001,
        cards::BLOOD_BARON_OF_VIZKOPA,
        PlayerId::Two,
    ));

    // Swords to Plowshares is white, Terror is black, Lightning Bolt is red.
    for color in [ManaColor::White, ManaColor::Black, ManaColor::Red] {
        game.add_unrestricted_mana(PlayerId::One, color, 4);
    }
    for (index, (definition, blocked)) in [
        (cards::SWORDS_TO_PLOWSHARES, true),
        (cards::TERROR, true),
        (cards::LIGHTNING_BOLT, false),
    ]
    .into_iter()
    .enumerate()
    {
        let spell = card(
            20_100 + u32::try_from(index).unwrap(),
            definition,
            PlayerId::One,
        );
        game.players[0].hand.push(spell.clone());
        let names_baron =
            castable_targets(&game, PlayerId::One, spell.id).contains(&GameObjectId(10_001));
        assert_eq!(
            names_baron,
            !blocked,
            "{definition:?} targeting Blood Baron should be {}",
            if blocked { "blocked" } else { "allowed" }
        );
    }
}

#[test]
fn the_old_school_knights_keep_their_protection() {
    // These four used to be named in the engine directly. Moving to printed
    // data must not change what they do.
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_001, cards::WHITE_KNIGHT, PlayerId::Two));
    game.battlefield
        .push(creature(10_002, cards::BLACK_KNIGHT, PlayerId::Two));

    for color in [ManaColor::White, ManaColor::Black] {
        game.add_unrestricted_mana(PlayerId::One, color, 4);
    }
    let mut next_id = 20_200;
    let mut hit_by = |game: &mut Game, definition| -> Vec<GameObjectId> {
        let spell = card(next_id, definition, PlayerId::One);
        next_id += 1;
        game.players[0].hand.push(spell.clone());
        castable_targets(game, PlayerId::One, spell.id)
    };

    // Terror is black and cannot touch White Knight. It could not touch Black
    // Knight either, but only because Black Knight is black, so the white
    // Swords to Plowshares is what shows protection working the other way.
    let by_black = hit_by(&mut game, cards::TERROR);
    assert!(
        !by_black.contains(&CardInstanceId(10_001)),
        "White Knight has protection from black"
    );

    let by_white = hit_by(&mut game, cards::SWORDS_TO_PLOWSHARES);
    assert!(
        !by_white.contains(&CardInstanceId(10_002)),
        "Black Knight has protection from white"
    );
    assert!(
        by_white.contains(&CardInstanceId(10_001)),
        "White Knight has no protection from white"
    );
}

#[test]
fn blood_baron_of_vizkopa_ascends_at_thirty_life() {
    let mut game = ready_game();
    game.battlefield.push(creature(
        10_001,
        cards::BLOOD_BARON_OF_VIZKOPA,
        PlayerId::One,
    ));
    let baron = game.battlefield.last().unwrap().clone();

    // Neither half of the condition alone is enough.
    game.players[0].life = 30;
    game.players[1].life = 11;
    assert_eq!(game.power(&baron), Some(4));
    assert!(!game.has_flying(&baron));

    game.players[0].life = 29;
    game.players[1].life = 10;
    assert_eq!(game.power(&baron), Some(4));

    game.players[0].life = 30;
    game.players[1].life = 10;
    assert_eq!(game.power(&baron), Some(10));
    assert_eq!(game.toughness(&baron), Some(10));
    assert!(game.has_flying(&baron));
}

#[test]
fn pillar_of_flame_exiles_what_it_kills() {
    let mut game = ready_game();
    // Savannah Lions is 2/1, so two damage is lethal.
    let lion = creature(10_000, cards::SAVANNAH_LIONS, PlayerId::Two);
    let lion_id = lion.card.id;
    game.battlefield.push(lion);
    let pillar = card(10_001, cards::PILLAR_OF_FLAME, PlayerId::One);
    game.players[0].hand.push(pillar.clone());
    game.players[0].mana_pool.red = 1;

    game.apply(
        PlayerId::One,
        cast_action(pillar.id, vec![Target::Permanent(lion_id)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert!(game.battlefield.is_empty());
    assert!(
        game.players[1].graveyard.is_empty(),
        "the lion never reaches the graveyard"
    );
    assert_eq!(game.players[1].exile[0].definition, cards::SAVANNAH_LIONS);
}

#[test]
fn pillar_of_flame_exiles_a_survivor_that_dies_later_this_turn() {
    let mut game = ready_game();
    // Serra Angel is 4/4: two damage leaves it alive, but the replacement
    // lasts the turn, so a later Lightning Bolt exiles it anyway.
    let angel = creature(10_000, cards::SERRA_ANGEL, PlayerId::Two);
    let angel_id = angel.card.id;
    game.battlefield.push(angel);
    let pillar = card(10_001, cards::PILLAR_OF_FLAME, PlayerId::One);
    let bolt = card(10_002, cards::LIGHTNING_BOLT, PlayerId::One);
    game.players[0].hand.push(pillar.clone());
    game.players[0].hand.push(bolt.clone());
    game.players[0].mana_pool.red = 2;

    game.apply(
        PlayerId::One,
        cast_action(pillar.id, vec![Target::Permanent(angel_id)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    assert_eq!(game.battlefield.len(), 1, "four damage is not lethal");

    game.apply(
        PlayerId::One,
        cast_action(bolt.id, vec![Target::Permanent(angel_id)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert!(game.battlefield.is_empty());
    assert!(game.players[1].graveyard.is_empty());
    assert_eq!(game.players[1].exile[0].definition, cards::SERRA_ANGEL);
}

#[test]
fn pillar_of_flame_can_burn_a_player() {
    let mut game = ready_game();
    let pillar = card(10_001, cards::PILLAR_OF_FLAME, PlayerId::One);
    game.players[0].hand.push(pillar.clone());
    game.players[0].mana_pool.red = 1;

    game.apply(
        PlayerId::One,
        cast_action(
            pillar.id,
            vec![Target::Player(PlayerId::Two)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(game.players[1].life, 18);
}

#[test]
fn declarative_ritual_psionic_blast_and_sign_in_blood_resolve() {
    let mut game = ready_game();
    let ritual = card(10_000, cards::DARK_RITUAL, PlayerId::One);
    game.players[0].hand.push(ritual.clone());
    game.players[0].mana_pool.black = 1;
    game.apply(
        PlayerId::One,
        cast_action(ritual.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    assert_eq!(game.players[0].mana_pool.black, 3);

    let mut game = ready_game();
    let blast = card(10_000, cards::PSIONIC_BLAST, PlayerId::One);
    game.players[0].hand.push(blast.clone());
    game.players[0].mana_pool.blue = 1;
    game.players[0].mana_pool.colorless = 2;
    game.apply(
        PlayerId::One,
        cast_action(blast.id, vec![Target::Player(PlayerId::Two)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    assert_eq!(game.players[0].life, 18);
    assert_eq!(game.players[1].life, 16);

    let mut game = ready_game();
    let sign = card(10_000, cards::SIGN_IN_BLOOD, PlayerId::One);
    game.players[0].hand.push(sign.clone());
    game.players[0].mana_pool.black = 2;
    game.apply(
        PlayerId::One,
        cast_action(sign.id, vec![Target::Player(PlayerId::Two)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);
    assert_eq!(game.players[1].hand.len(), 2);
    assert_eq!(game.players[1].life, 18);
}
