use super::*;

#[test]
fn a_physical_card_gets_new_object_identity_in_each_cast_zone() {
    let mut game = ready_game();
    let card = card(10_000, cards::TRISKELION, PlayerId::One);
    let hand_id = card.id;
    let physical = backing_cards(&card.backing);
    game.players[0].hand.push(card);
    game.players[0].mana_pool.colorless = 6;

    game.apply(
        PlayerId::One,
        cast_action(hand_id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    let spell_id = game.stack[0].id;
    assert_ne!(spell_id, hand_id);
    assert_eq!(backing_cards(&game.stack[0].card.backing), physical);

    pass_priority_pair(&mut game);
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::TRISKELION)
        .unwrap();
    assert_ne!(permanent.card.id, spell_id);
    assert_ne!(permanent.card.id, hand_id);
    assert_eq!(backing_cards(&permanent.card.backing), physical);
}

#[test]
fn a_forked_spell_has_new_identity_and_no_physical_backing() {
    let mut game = ready_game();
    let original = spell(77, cards::LIGHTNING_BOLT, PlayerId::Two, 0);
    let original_id = original.id;

    game.push_copy(original, PlayerId::One, Vec::new());

    let copied = game.stack.last().unwrap();
    assert_ne!(copied.id, original_id);
    assert_eq!(copied.card.backing, ObjectBacking::None);
    assert_eq!(
        copied.card.characteristics,
        CharacteristicSource::Copy(cards::LIGHTNING_BOLT)
    );
    assert_eq!(copied.card.owner, PlayerId::One);
    assert!(copied.is_copy);
}

#[test]
fn physical_card_metadata_is_separate_from_live_objects() {
    let game = ready_game();
    let physical = game.physical_cards[0].clone();
    assert_eq!(
        game.physical_card_definition(physical.id),
        Some(physical.definition)
    );
    assert_eq!(game.physical_card_owner(physical.id), Some(physical.owner));
}

#[test]
fn validated_sideboards_are_retained_outside_the_game_without_perturbing_main_ids() {
    let deck_with_sideboard = poc::mono_red_atog();
    assert!(!deck_with_sideboard.sideboard.is_empty());
    let sideboard_definitions = deck_with_sideboard.sideboard.clone();
    let mut deck_without_sideboard = deck_with_sideboard.clone();
    deck_without_sideboard.sideboard.clear();

    let with_sideboards = Game::new(
        poc::catalog().unwrap(),
        [deck_with_sideboard.clone(), deck_with_sideboard],
        17,
    )
    .unwrap();
    let without_sideboards = Game::new(
        poc::catalog().unwrap(),
        [deck_without_sideboard.clone(), deck_without_sideboard],
        17,
    )
    .unwrap();

    for player in [PlayerId::One, PlayerId::Two] {
        let main_objects = |game: &Game| {
            game.players[player.index()]
                .library
                .iter()
                .chain(&game.players[player.index()].hand)
                .map(|card| (card.id, card.definition, card.backing.clone()))
                .collect::<Vec<_>>()
        };
        assert_eq!(
            main_objects(&with_sideboards),
            main_objects(&without_sideboards)
        );

        let outside = &with_sideboards.players[player.index()].outside_game;
        assert_eq!(
            outside
                .iter()
                .map(|card| card.definition)
                .collect::<Vec<_>>(),
            sideboard_definitions
        );
        for card in outside {
            assert_eq!(card.owner, player);
            let backing = backing_cards(&card.backing);
            let [physical] = backing.as_slice() else {
                panic!("an outside-game card must have exactly one physical backing card")
            };
            assert_eq!(
                with_sideboards.physical_card_definition(*physical),
                Some(card.definition)
            );
            assert_eq!(with_sideboards.physical_card_owner(*physical), Some(player));
        }
    }
}

#[test]
fn spell_events_keep_stack_identity_and_definition_after_the_card_moves() {
    let mut game = ready_game();
    let bolt = card(10_000, cards::LIGHTNING_BOLT, PlayerId::One);
    let hand_id = bolt.id;
    game.players[0].hand.push(bolt);
    game.players[0].mana_pool.red = 1;
    let event_start = game.events.len();

    game.apply(
        PlayerId::One,
        cast_action(hand_id, vec![Target::Player(PlayerId::Two)], Vec::new(), 0),
    )
    .unwrap();
    let stack_id = game.stack[0].id;
    assert_ne!(stack_id, hand_id);
    assert!(game.events[event_start..].contains(&GameEvent::SpellCast {
        player: PlayerId::One,
        card: stack_id,
        definition: cards::LIGHTNING_BOLT,
        targets: vec![Target::Player(PlayerId::Two)],
    }));

    pass_priority_pair(&mut game);
    assert!(
        game.events[event_start..].contains(&GameEvent::SpellResolved {
            card: stack_id,
            definition: cards::LIGHTNING_BOLT,
        })
    );
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::LIGHTNING_BOLT && card.id != stack_id),
        "the event still names the former stack object after the card became a new object",
    );
}

#[test]
fn ability_events_distinguish_the_stack_object_from_a_source_that_left_play() {
    let mut game = ready_game();
    let strip = creature(10_000, cards::STRIP_MINE, PlayerId::One);
    let target = creature(10_001, cards::MOUNTAIN, PlayerId::Two);
    let source_id = strip.card.id;
    let target_id = target.card.id;
    game.battlefield = vec![strip, target];
    let event_start = game.events.len();

    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source: source_id,
            ability: activated_ability_for(&game, source_id, 0),
            targets: activated_targets(Target::Permanent(target_id)),
            cost_objects: Vec::new(),
            x: 0,
            modes: Vec::new(),
            mana_payment: None,
        },
    )
    .unwrap();
    let ability_id = game.stack[0].id;
    assert_eq!(
        game.stack[0].ability_origin(),
        Some(AbilityOrigin::Printed {
            definition: cards::STRIP_MINE,
            part: CardPartId::PRIMARY,
            ability: crate::AbilityId(1),
        })
    );
    assert_eq!(
        game.stack[0].ability_text(),
        Some("{T}, Sacrifice this land: Destroy target land.")
    );
    assert_ne!(ability_id, source_id);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != source_id),
        "the source has already left play when its activation is logged",
    );
    assert!(
        game.events[event_start..].contains(&GameEvent::AbilityActivated {
            player: PlayerId::One,
            object: ability_id,
            source: source_id,
            presentation: ObjectCharacteristics::card(cards::STRIP_MINE, CardPartId::PRIMARY),
            chosen_permanents: vec![target_id],
        })
    );

    pass_priority_pair(&mut game);
    assert!(
        game.events[event_start..].contains(&GameEvent::AbilityResolved {
            object: ability_id,
            source: source_id,
            presentation: ObjectCharacteristics::card(cards::STRIP_MINE, CardPartId::PRIMARY),
        })
    );
}

#[test]
fn recall_charges_two_generic_mana_for_each_x() {
    let game = ready_game();
    let cost = game
        .catalog
        .get(cards::RECALL)
        .and_then(|definition| definition.rules.mana_cost())
        .expect("Recall has a printed mana cost");
    assert!(can_pay(
        ManaPool {
            blue: 1,
            colorless: 6,
            ..ManaPool::default()
        },
        cost,
        3,
    ));
    assert!(!can_pay(
        ManaPool {
            blue: 1,
            colorless: 5,
            ..ManaPool::default()
        },
        cost,
        3,
    ));
}

#[test]
fn white_red_hybrid_symbols_accept_either_color_but_not_colorless() {
    let cost = ManaCost::hybrid_pair(HybridPair::WhiteRed, 3);
    assert!(can_pay(
        ManaPool {
            white: 2,
            red: 1,
            ..ManaPool::default()
        },
        cost,
        0,
    ));
    assert!(can_pay(
        ManaPool {
            red: 3,
            ..ManaPool::default()
        },
        cost,
        0,
    ));
    assert!(!can_pay(
        ManaPool {
            colorless: 3,
            ..ManaPool::default()
        },
        cost,
        0,
    ));

    let mut pool = ManaPool {
        white: 2,
        red: 1,
        ..ManaPool::default()
    };
    pay_cost(&mut pool, cost, 0);
    assert_eq!(pool, ManaPool::default());
}

#[test]
fn overlapping_hybrid_pairs_share_each_colors_capacity() {
    let mut cost = ManaCost::default();
    cost.hybrid[HybridPair::WhiteBlue.index()] = 1;
    cost.hybrid[HybridPair::WhiteBlack.index()] = 1;

    assert!(
        !can_pay(
            ManaPool {
                white: 1,
                green: 1,
                ..ManaPool::default()
            },
            cost,
            0,
        ),
        "one white cannot satisfy both overlapping hybrid symbols",
    );

    let mut pool = ManaPool {
        white: 1,
        blue: 1,
        ..ManaPool::default()
    };
    assert!(can_pay(pool, cost, 0));
    pay_cost(&mut pool, cost, 0);
    assert_eq!(pool, ManaPool::default());

    cost.generic = 1;
    assert_eq!(
        Game::generic_shortfall(
            ManaPool {
                white: 1,
                blue: 1,
                ..ManaPool::default()
            },
            cost,
            0,
        ),
        1,
        "both colored units are globally committed to the hybrid symbols",
    );
}

#[test]
fn declarative_mana_production_drives_generic_mana_sources() {
    static ABILITIES: [AbilityDef; 1] = [AbilityDef::activated_mana(
        "{T}: Add {U} or {R}.",
        &[AbilityCostDef::TapSource],
        EffectDef::AddMana(AddManaEffectDef::choice(&[ManaColor::Blue, ManaColor::Red])),
    )];
    let definition_id = CardDefinitionId::new(10_000);
    let mut definition = CardDefinition::new(
        definition_id,
        "Test dual land",
        CardSet::Magic2014,
        false,
        CardBehavior::Unsupported,
    );
    definition.rules = CardRules::new_land(&[]).with_abilities(&ABILITIES);
    synchronize_single_part_definition(&mut definition);

    let mut game = ready_game();
    game.catalog = CardCatalog::new([definition]).unwrap();
    game.battlefield
        .push(creature(10_000, definition_id, PlayerId::One));

    let activations = game.mana_ability_activations(&game.battlefield[0]);
    assert_eq!(
        activations
            .iter()
            .map(|activation| activation.color)
            .collect::<Vec<_>>(),
        vec![ManaColor::Blue, ManaColor::Red]
    );
    let ability = mana_ability_for(&game, CardInstanceId(10_000), ManaColor::Blue);
    game.activate_mana_source(
        PlayerId::One,
        CardInstanceId(10_000),
        ability,
        ManaColor::Blue,
        &ManaActivationChoices::default(),
    );
    assert_eq!(game.players[0].mana_pool.blue, 1);
    assert!(game.battlefield[0].tapped);
}

#[test]
fn observations_keep_permanents_with_object_specific_state_out_of_shared_piles() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_100, cards::MANA_VAULT, PlayerId::One));
    game.battlefield
        .push(creature(10_101, cards::MANA_VAULT, PlayerId::One));

    let pristine = game.observe(PlayerId::One);
    assert!(
        pristine
            .battlefield
            .iter()
            .all(|permanent| !permanent.has_individual_state),
        "a permanent's own printed static ability is shared card state",
    );

    game.battlefield[0].skipped_untap_steps = 1;
    let affected = game.observe(PlayerId::One);
    assert!(affected.battlefield[0].has_individual_state);
    assert!(!affected.battlefield[1].has_individual_state);

    let first = game.battlefield[0].card.id;
    game.battlefield[1].attached_to = Some(first);
    let attached = game.observe(PlayerId::One);
    assert!(attached.battlefield[0].has_individual_state);
    assert!(attached.battlefield[1].has_individual_state);
}
