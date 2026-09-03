//! Land-type words are live everywhere they matter.
//!
//! Each board crosses a different engine boundary that a text or type change
//! has to reach: state triggers, combat, Aura legality, dynamic values, costs,
//! animation, triggered mana, and characteristic-setting effects.

use super::*;

fn change_land_word(permanent: &mut Permanent, from: BasicLandType, to: BasicLandType) {
    permanent.text_changes.push(TextChange {
        word: TextWordChange::BasicLandType { from, to },
        expiration: ContinuousEffectExpiration::Never,
    });
}

fn permanent(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("the permanent is still on the battlefield")
}

fn can_be_blocked(game: &Game, attacker: GameObjectId) -> bool {
    game.legal_actions(PlayerId::Two).iter().any(
        |action| matches!(action, Action::DeclareBlocker { attacker: id, .. } if *id == attacker),
    )
}

#[test]
fn illusionary_terrain_turning_islands_into_forests_kills_dandan() {
    let mut game = ready_game();
    let dandan = creature(10_000, cards::DANDAN, PlayerId::One);
    let dandan_id = dandan.card.id;
    let island = creature(10_001, cards::ISLAND, PlayerId::One);
    let mut terrain = creature(10_002, cards::ILLUSIONARY_TERRAIN, PlayerId::One);
    terrain.chosen_basic_land_type_substitution =
        Some((BasicLandType::Island, BasicLandType::Forest));
    game.battlefield.extend([dandan, island, terrain]);

    assert!(
        game.effective_subtypes(&game.battlefield[1])
            .contains(&"Forest")
    );
    assert!(
        !game
            .effective_subtypes(&game.battlefield[1])
            .contains(&"Island")
    );

    game.check_state_based_actions();
    drain_pending(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == dandan_id),
        "Dandân's state trigger sees that its controller now controls no Islands",
    );
}

#[test]
fn illusionary_terrain_removes_the_land_type_that_turns_on_landwalk() {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    game.priority = PlayerId::Two;
    let mut attacker = creature(10_000, cards::SEGOVIAN_LEVIATHAN, PlayerId::One);
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let attacker_id = attacker.card.id;
    game.battlefield.extend([
        attacker,
        creature(10_001, cards::SAVANNAH_LIONS, PlayerId::Two),
        creature(10_002, cards::ISLAND, PlayerId::Two),
    ]);

    assert!(
        !can_be_blocked(&game, attacker_id),
        "the defending Island initially turns on islandwalk",
    );

    let mut terrain = creature(10_003, cards::ILLUSIONARY_TERRAIN, PlayerId::One);
    terrain.chosen_basic_land_type_substitution =
        Some((BasicLandType::Island, BasicLandType::Forest));
    game.battlefield.push(terrain);

    assert!(
        can_be_blocked(&game, attacker_id),
        "once the basic Island is a Forest, islandwalk no longer applies",
    );
}

#[test]
fn an_unmatched_illusionary_terrain_substitution_does_not_remove_rules_text() {
    let mut game = ready_game();
    let island = creature(10_000, cards::ISLAND, PlayerId::One);
    let island_id = island.card.id;
    let plains = creature(10_001, cards::PLAINS, PlayerId::One);
    let plains_id = plains.card.id;
    let mut terrain = creature(10_002, cards::ILLUSIONARY_TERRAIN, PlayerId::One);
    terrain.chosen_basic_land_type_substitution =
        Some((BasicLandType::Island, BasicLandType::Forest));
    game.battlefield.extend([island, plains, terrain]);

    assert!(game.rules_text_abilities_removed(permanent(&game, island_id)));
    assert!(
        !game.rules_text_abilities_removed(permanent(&game, plains_id)),
        "a conditional Set that does not find its first type does nothing",
    );
}

#[test]
fn a_hacked_enchant_forest_aura_stays_on_taiga_but_falls_off_forest() {
    let mut game = ready_game();
    let forest = creature(10_000, cards::FOREST, PlayerId::One);
    let forest_id = forest.card.id;
    let taiga = creature(10_001, cards::TAIGA, PlayerId::One);
    let taiga_id = taiga.card.id;
    let mut on_forest = creature(10_002, cards::UTOPIA_SPRAWL, PlayerId::One);
    on_forest.attached_to = Some(forest_id);
    let on_forest_id = on_forest.card.id;
    let mut on_taiga = creature(10_003, cards::UTOPIA_SPRAWL, PlayerId::One);
    on_taiga.attached_to = Some(taiga_id);
    let on_taiga_id = on_taiga.card.id;
    game.battlefield
        .extend([forest, taiga, on_forest, on_taiga]);
    game.check_state_based_actions();
    assert!(game.players[0].graveyard.is_empty());

    for aura in &mut game.battlefield {
        if [on_forest_id, on_taiga_id].contains(&aura.card.id) {
            change_land_word(aura, BasicLandType::Forest, BasicLandType::Mountain);
        }
    }
    game.check_state_based_actions();

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == on_taiga_id),
        "Taiga is still a Mountain under the rewritten enchant restriction",
    );
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == on_forest_id),
        "a basic Forest no longer satisfies Enchant Mountain",
    );
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::UTOPIA_SPRAWL),
    );
}

#[test]
fn hacked_forest_count_auras_count_the_replacement_land_type() {
    let mut game = ready_game();
    let first_host = creature(10_000, cards::GRIZZLY_BEARS, PlayerId::One);
    let first_host_id = first_host.card.id;
    let second_host = creature(10_001, cards::GRIZZLY_BEARS, PlayerId::One);
    let second_host_id = second_host.card.id;
    let mut aspect = creature(10_002, cards::ASPECT_OF_WOLF, PlayerId::One);
    aspect.attached_to = Some(first_host_id);
    change_land_word(&mut aspect, BasicLandType::Forest, BasicLandType::Island);
    let mut armor = creature(10_003, cards::BLANCHWOOD_ARMOR, PlayerId::One);
    armor.attached_to = Some(second_host_id);
    change_land_word(&mut armor, BasicLandType::Forest, BasicLandType::Island);
    game.battlefield
        .extend([first_host, second_host, aspect, armor]);
    for id in 10_010..10_013 {
        game.battlefield
            .push(creature(id, cards::ISLAND, PlayerId::One));
    }
    game.battlefield
        .push(creature(10_020, cards::FOREST, PlayerId::One));

    assert_eq!(game.power(permanent(&game, first_host_id)), Some(3));
    assert_eq!(game.toughness(permanent(&game, first_host_id)), Some(4));
    assert_eq!(game.power(permanent(&game, second_host_id)), Some(5));
    assert_eq!(game.toughness(permanent(&game, second_host_id)), Some(5));
}

#[test]
fn dryads_favor_grants_the_hacked_landwalk_word() {
    let mut game = ready_game();
    game.step = Step::DeclareBlockers;
    game.priority = PlayerId::Two;
    let mut attacker = creature(10_000, cards::GRIZZLY_BEARS, PlayerId::One);
    attacker.attacking = true;
    attacker.attack_defender = Some(AttackDefender::Player(PlayerId::Two));
    let attacker_id = attacker.card.id;
    let mut favor = creature(10_001, cards::DRYADS_FAVOR, PlayerId::One);
    favor.attached_to = Some(attacker_id);
    change_land_word(&mut favor, BasicLandType::Forest, BasicLandType::Mountain);
    game.battlefield.extend([
        attacker,
        favor,
        creature(10_002, cards::SAVANNAH_LIONS, PlayerId::Two),
        creature(10_003, cards::FOREST, PlayerId::Two),
    ]);

    assert!(
        can_be_blocked(&game, attacker_id),
        "the printed Forest no longer turns on the granted walk",
    );
    game.battlefield
        .push(creature(10_004, cards::MOUNTAIN, PlayerId::Two));
    assert!(
        !can_be_blocked(&game, attacker_id),
        "the Aura now grants mountainwalk",
    );
}

#[test]
fn fortitude_hacked_to_mountain_sacrifices_only_mountains() {
    let mut game = ready_game();
    let host = creature(10_000, cards::GRIZZLY_BEARS, PlayerId::One);
    let host_id = host.card.id;
    let forest = creature(10_001, cards::FOREST, PlayerId::One);
    let forest_id = forest.card.id;
    let mountain = creature(10_002, cards::MOUNTAIN, PlayerId::One);
    let mountain_id = mountain.card.id;
    let mut fortitude = creature(10_003, cards::FORTITUDE, PlayerId::One);
    fortitude.attached_to = Some(host_id);
    let fortitude_id = fortitude.card.id;
    change_land_word(
        &mut fortitude,
        BasicLandType::Forest,
        BasicLandType::Mountain,
    );
    game.battlefield.extend([host, forest, mountain, fortitude]);

    let payments = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::ActivateAbility {
                source,
                cost_objects,
                ..
            } if source == fortitude_id => cost_objects.first().copied(),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert!(payments.contains(&mountain_id));
    assert!(!payments.contains(&forest_id));
}

#[test]
fn genju_animates_its_land_without_removing_land_characteristics() {
    let mut game = ready_game();
    game.players[0].mana_pool.colorless = 2;
    let taiga = creature(10_000, cards::TAIGA, PlayerId::One);
    let taiga_id = taiga.card.id;
    let mut genju = creature(10_001, cards::GENJU_OF_THE_CEDARS, PlayerId::One);
    genju.attached_to = Some(taiga_id);
    change_land_word(&mut genju, BasicLandType::Forest, BasicLandType::Mountain);
    let genju_id = genju.card.id;
    game.battlefield.extend([taiga, genju]);

    let ability = activated_ability_for(&game, genju_id, 0);
    game.apply(PlayerId::One, plain_activation(genju_id, ability))
        .expect("the Genju activation is legal");
    pass_priority_pair(&mut game);

    let animated = permanent(&game, taiga_id);
    let types = game.permanent_types(animated).expect("a permanent");
    assert!(types.contains(CardType::Land));
    assert!(types.contains(CardType::Creature));
    assert_eq!(game.power(animated), Some(4));
    assert_eq!(game.toughness(animated), Some(4));
    assert_eq!(
        game.permanent_colors(animated),
        [false, false, false, false, true]
    );
    assert!(game.effective_subtypes(animated).contains(&"Spirit"));
    assert!(game.effective_subtypes(animated).contains(&"Forest"));
    assert!(game.effective_subtypes(animated).contains(&"Mountain"));
}

#[test]
fn genju_follows_its_hacked_land_to_the_graveyard_and_can_return() {
    let mut game = ready_game();
    let taiga = creature(10_000, cards::TAIGA, PlayerId::One);
    let taiga_id = taiga.card.id;
    let mut genju = creature(10_001, cards::GENJU_OF_THE_CEDARS, PlayerId::One);
    genju.attached_to = Some(taiga_id);
    change_land_word(&mut genju, BasicLandType::Forest, BasicLandType::Mountain);
    game.battlefield.extend([taiga, genju]);

    game.move_permanents_to_graveyard(&[taiga_id]);
    assert_eq!(game.pending_triggers.len(), 1, "Genju sees its host leave");
    game.check_state_based_actions();
    for _ in 0..8 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        let player = game.priority;
        game.apply(player, Action::PassPriority)
            .expect("priority advances to the optional return");
    }
    choose_decision_by_label(&mut game, PlayerId::One, "Do it");
    drain_pending(&mut game);

    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|card| card.definition == cards::GENJU_OF_THE_CEDARS),
        "the trigger follows the Aura's new graveyard object",
    );
}

#[test]
fn fortitude_returns_to_hand_after_it_dies() {
    let mut game = ready_game();
    let host = creature(10_000, cards::GRIZZLY_BEARS, PlayerId::One);
    let host_id = host.card.id;
    let mut fortitude = creature(10_001, cards::FORTITUDE, PlayerId::One);
    fortitude.attached_to = Some(host_id);
    let fortitude_id = fortitude.card.id;
    game.battlefield.extend([host, fortitude]);

    game.move_permanents_to_graveyard(&[fortitude_id]);
    drain_pending(&mut game);

    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|card| card.definition == cards::FORTITUDE),
    );
}

#[test]
fn utopia_sprawl_adds_the_chosen_color_when_its_forest_makes_mana() {
    let mut game = ready_game();
    let forest = creature(10_000, cards::FOREST, PlayerId::One);
    let forest_id = forest.card.id;
    let mut sprawl = creature(10_001, cards::UTOPIA_SPRAWL, PlayerId::One);
    sprawl.attached_to = Some(forest_id);
    sprawl.chosen_color = Some(ManaColor::Blue);
    game.battlefield.extend([forest, sprawl]);

    game.apply(
        PlayerId::One,
        Action::ActivateManaAbility {
            source: forest_id,
            ability: mana_ability_for(&game, forest_id, ManaColor::Green),
            color: ManaColor::Green,
            counters_removed: None,
            cost_object: None,
            combination: None,
            triggered_mana: None,
        },
    )
    .expect("the Forest taps for mana");

    assert_eq!(game.players[0].mana_pool.green, 1);
    assert_eq!(game.players[0].mana_pool.blue, 1);
}

#[test]
fn song_of_the_dryads_sets_every_named_characteristic_and_follows_text_changes() {
    let mut game = ready_game();
    let angel = creature(10_000, cards::SERRA_ANGEL, PlayerId::One);
    let angel_id = angel.card.id;
    let mut song = creature(10_001, cards::SONG_OF_THE_DRYADS, PlayerId::Two);
    song.attached_to = Some(angel_id);
    let song_id = song.card.id;
    game.battlefield.extend([angel, song]);

    let enchanted = permanent(&game, angel_id);
    assert_eq!(
        game.permanent_types(enchanted),
        Some(CardTypeSet::single(CardType::Land))
    );
    assert_eq!(game.permanent_colors(enchanted), [false; 5]);
    assert_eq!(game.effective_subtypes(enchanted).as_ref(), &["Forest"]);
    assert!(!game.permanent_has_executable_keyword(enchanted, KeywordAbility::Flying));
    assert!(
        game.mana_ability_activations(enchanted)
            .iter()
            .any(|activation| activation.color == ManaColor::Green)
    );

    change_land_word(
        game.battlefield
            .iter_mut()
            .find(|permanent| permanent.card.id == song_id)
            .expect("Song is attached"),
        BasicLandType::Forest,
        BasicLandType::Mountain,
    );
    let enchanted = permanent(&game, angel_id);
    assert_eq!(game.effective_subtypes(enchanted).as_ref(), &["Mountain"]);
    assert!(
        game.mana_ability_activations(enchanted)
            .iter()
            .any(|activation| activation.color == ManaColor::Red)
    );
}

#[test]
fn song_of_the_dryads_removes_subtypes_of_the_card_types_it_replaces() {
    let mut game = ready_game();
    let jitte = creature(10_000, cards::UMEZAWAS_JITTE, PlayerId::One);
    let jitte_id = jitte.card.id;
    let mut song = creature(10_001, cards::SONG_OF_THE_DRYADS, PlayerId::Two);
    song.attached_to = Some(jitte_id);
    game.battlefield.extend([jitte, song]);

    assert_eq!(
        game.effective_subtypes(permanent(&game, jitte_id)).as_ref(),
        &["Forest"]
    );
}

#[test]
fn illusionary_terrain_chooses_and_remembers_an_ordered_type_pair_as_it_enters() {
    let mut game = ready_game();
    let terrain = card(10_000, cards::ILLUSIONARY_TERRAIN, PlayerId::One);
    game.players[0].hand.push(terrain.clone());
    game.players[0].mana_pool.blue = 2;
    game.battlefield
        .push(creature(10_001, cards::ISLAND, PlayerId::Two));

    game.apply(
        PlayerId::One,
        cast_action(terrain.id, Vec::new(), Vec::new(), 0),
    )
    .expect("Illusionary Terrain can be cast");
    pass_priority_pair(&mut game);
    choose_decision_by_label(&mut game, PlayerId::One, "Island → Forest");

    let terrain = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::ILLUSIONARY_TERRAIN)
        .expect("the chosen enchantment enters");
    assert_eq!(
        terrain.chosen_basic_land_type_substitution,
        Some((BasicLandType::Island, BasicLandType::Forest))
    );
    let island = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::ISLAND)
        .expect("the basic land remains");
    assert_eq!(game.effective_subtypes(island).as_ref(), &["Forest"]);
}
