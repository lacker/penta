//! Cards whose entry choice names another card, and the shared rules that
//! consume that stored name.

use super::*;

fn pending_choice(game: &Game, player: PlayerId) -> DecisionObservation {
    game.observe(player)
        .decision
        .expect("an entry choice is pending")
}

fn choose_label(game: &mut Game, player: PlayerId, label: &str) {
    let decision = pending_choice(game, player);
    let option = decision
        .options
        .iter()
        .find(|option| option.label == label)
        .unwrap_or_else(|| panic!("{label} is offered"))
        .id;
    game.apply(
        player,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .expect("the named entry choice is legal");
}

fn put_named(
    game: &mut Game,
    player: PlayerId,
    definition: CardDefinitionId,
    name: &str,
) -> GameObjectId {
    let id = game
        .put_onto_battlefield(player, definition)
        .expect("the naming permanent is cataloged");
    choose_label(game, player, name);
    drain_pending(game);
    id
}

fn can_cast(game: &Game, player: PlayerId, card: GameObjectId) -> bool {
    game.legal_actions(player).iter().any(
        |action| matches!(action, Action::CastSpell { card: candidate, .. } if *candidate == card),
    )
}

fn has_ordinary_activation(game: &Game, player: PlayerId, source: GameObjectId) -> bool {
    game.legal_actions(player).iter().any(
        |action| matches!(action, Action::ActivateAbility { source: candidate, .. } if *candidate == source),
    )
}

fn has_mana_activation(game: &Game, player: PlayerId, source: GameObjectId) -> bool {
    game.legal_actions(player).iter().any(
        |action| matches!(action, Action::ActivateManaAbility { source: candidate, .. } if *candidate == source),
    )
}

#[test]
fn nevermore_prohibits_casting_but_not_activating_the_named_card() {
    let mut game = ready_game();
    game.turns_started = [1, 1];
    let disk = game
        .put_onto_battlefield(PlayerId::Two, cards::NEVINYRRALS_DISK)
        .expect("Nevinyrral's Disk is cataloged");
    put_named(
        &mut game,
        PlayerId::One,
        cards::NEVERMORE,
        "Nevinyrral's Disk",
    );
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == disk)
        .expect("the Disk remains on the battlefield")
        .tapped = false;

    game.priority = PlayerId::Two;
    game.players[PlayerId::Two.index()].mana_pool.colorless = 1;
    assert!(
        has_ordinary_activation(&game, PlayerId::Two, disk),
        "Nevermore's cast restriction does not inherit Pithing Needle's activation lock",
    );
    let second_disk = card(10_001, cards::NEVINYRRALS_DISK, PlayerId::Two);
    let second_disk_id = second_disk.id;
    game.players[PlayerId::Two.index()].hand.push(second_disk);
    game.players[PlayerId::Two.index()].mana_pool.colorless = 4;
    game.priority = PlayerId::Two;
    assert!(!can_cast(&game, PlayerId::Two, second_disk_id));
}

#[test]
fn spyglass_looks_privately_but_its_public_name_catalog_is_hidden_zone_independent() {
    let mut game = ready_game();
    game.players[PlayerId::Two.index()].hand.push(card(
        10_010,
        cards::LIGHTNING_BOLT,
        PlayerId::Two,
    ));
    game.put_onto_battlefield(PlayerId::One, cards::SORCEROUS_SPYGLASS)
        .expect("Spyglass is cataloged");

    assert_eq!(
        game.last_seen_hands[PlayerId::One.index()],
        Some((
            PlayerId::Two,
            vec![(GameObjectId(10_010), cards::LIGHTNING_BOLT)],
        )),
    );
    assert_eq!(game.last_seen_hands[PlayerId::Two.index()], None);
    let chooser_decision = pending_choice(&game, PlayerId::One);
    let labels = chooser_decision
        .options
        .iter()
        .map(|option| option.label.clone())
        .collect::<Vec<_>>();
    assert!(labels.contains(&"Lightning Bolt".to_string()));
    assert!(
        labels.contains(&"Island".to_string()),
        "Spyglass may name a land"
    );
    assert!(chooser_decision.options.iter().all(|option| {
        option.card.is_none() && option.members.is_empty() && option.zone == DecisionZone::None
    }));
    assert_eq!(
        pending_choice(&game, PlayerId::Two).options,
        chooser_decision.options,
        "the public naming choice discloses only the shared catalog"
    );

    let mut different_hidden_cards = ready_game();
    different_hidden_cards.players[PlayerId::Two.index()]
        .hand
        .push(card(10_011, cards::SERRA_ANGEL, PlayerId::Two));
    different_hidden_cards
        .put_onto_battlefield(PlayerId::One, cards::SORCEROUS_SPYGLASS)
        .expect("Spyglass is cataloged");
    let other_labels = pending_choice(&different_hidden_cards, PlayerId::One)
        .options
        .into_iter()
        .map(|option| option.label)
        .collect::<Vec<_>>();
    assert_eq!(
        labels, other_labels,
        "hidden cards cannot shape name options"
    );
}

#[test]
fn flute_and_peacekeeper_apply_their_distinct_named_cost_rules() {
    let mut flute_game = ready_game();
    let bolt = card(10_020, cards::LIGHTNING_BOLT, PlayerId::Two);
    let bolt_id = bolt.id;
    flute_game.players[PlayerId::Two.index()].hand.push(bolt);
    put_named(
        &mut flute_game,
        PlayerId::One,
        cards::DISRUPTOR_FLUTE,
        "Lightning Bolt",
    );
    flute_game.priority = PlayerId::Two;
    flute_game.players[PlayerId::Two.index()].mana_pool.red = 1;
    flute_game.players[PlayerId::Two.index()]
        .mana_pool
        .colorless = 2;
    assert!(!can_cast(&flute_game, PlayerId::Two, bolt_id));
    flute_game.players[PlayerId::Two.index()]
        .mana_pool
        .colorless = 3;
    assert!(can_cast(&flute_game, PlayerId::Two, bolt_id));

    let mut peacekeeper_game = ready_game();
    peacekeeper_game.turns_started = [1, 1];
    let factory = peacekeeper_game
        .put_onto_battlefield(PlayerId::Two, cards::MISHRA_S_FACTORY)
        .expect("Factory is cataloged");
    peacekeeper_game.players[PlayerId::Two.index()]
        .hand
        .push(card(10_021, cards::LIGHTNING_BOLT, PlayerId::Two));
    peacekeeper_game
        .put_onto_battlefield(PlayerId::One, cards::ANOINTED_PEACEKEEPER)
        .expect("Peacekeeper is cataloged");
    choose_label(&mut peacekeeper_game, PlayerId::One, "Mishra's Factory");
    drain_pending(&mut peacekeeper_game);
    peacekeeper_game.priority = PlayerId::Two;
    peacekeeper_game.players[PlayerId::Two.index()]
        .mana_pool
        .colorless = 1;
    assert!(!has_ordinary_activation(
        &peacekeeper_game,
        PlayerId::Two,
        factory,
    ));
    assert!(has_mana_activation(
        &peacekeeper_game,
        PlayerId::Two,
        factory,
    ));
    peacekeeper_game.players[PlayerId::Two.index()]
        .mana_pool
        .colorless = 3;
    assert!(has_ordinary_activation(
        &peacekeeper_game,
        PlayerId::Two,
        factory,
    ));
}

#[test]
fn named_activation_rules_reach_sources_in_hand() {
    let mut needle_game = ready_game();
    let decree = card(10_022, cards::DECREE_OF_JUSTICE, PlayerId::Two);
    let decree_id = decree.id;
    needle_game.players[PlayerId::Two.index()].hand.push(decree);
    put_named(
        &mut needle_game,
        PlayerId::One,
        cards::PITHING_NEEDLE,
        "Decree of Justice",
    );
    needle_game.priority = PlayerId::Two;
    needle_game.players[PlayerId::Two.index()].mana_pool.white = 1;
    needle_game.players[PlayerId::Two.index()]
        .mana_pool
        .colorless = 10;
    assert!(
        !has_ordinary_activation(&needle_game, PlayerId::Two, decree_id),
        "Pithing Needle stops cycling from hand",
    );

    let mut peacekeeper_game = ready_game();
    let decree = card(10_023, cards::DECREE_OF_JUSTICE, PlayerId::Two);
    let decree_id = decree.id;
    peacekeeper_game.players[PlayerId::Two.index()]
        .hand
        .push(decree);
    peacekeeper_game
        .put_onto_battlefield(PlayerId::One, cards::ANOINTED_PEACEKEEPER)
        .expect("Peacekeeper is cataloged");
    choose_label(&mut peacekeeper_game, PlayerId::One, "Decree of Justice");
    drain_pending(&mut peacekeeper_game);
    peacekeeper_game.priority = PlayerId::Two;
    peacekeeper_game.players[PlayerId::Two.index()]
        .mana_pool
        .white = 1;
    peacekeeper_game.players[PlayerId::Two.index()]
        .mana_pool
        .colorless = 2;
    assert!(!has_ordinary_activation(
        &peacekeeper_game,
        PlayerId::Two,
        decree_id,
    ));
    peacekeeper_game.players[PlayerId::Two.index()]
        .mana_pool
        .colorless = 4;
    assert!(
        has_ordinary_activation(&peacekeeper_game, PlayerId::Two, decree_id),
        "Peacekeeper adds two mana to the cycling cost",
    );
    let cycling = peacekeeper_game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == decree_id)
        })
        .expect("the taxed cycling activation is offered");
    peacekeeper_game
        .apply(PlayerId::Two, cycling)
        .expect("the taxed cycling activation is legal");
    assert_eq!(
        peacekeeper_game.players[PlayerId::Two.index()]
            .mana_pool
            .total(),
        0,
        "the activation pays the full increased cost",
    );
}

#[test]
fn alpine_moon_names_only_nonbasic_lands_and_rewrites_matching_opponent_lands() {
    let mut game = ready_game();
    let tower = game
        .put_onto_battlefield(PlayerId::Two, cards::URZA_S_TOWER)
        .expect("Urza's Tower is cataloged");
    game.put_onto_battlefield(PlayerId::One, cards::ALPINE_MOON)
        .expect("Alpine Moon is cataloged");
    let labels = pending_choice(&game, PlayerId::One)
        .options
        .iter()
        .map(|option| option.label.clone())
        .collect::<Vec<_>>();
    assert!(labels.contains(&"Urza's Tower".to_string()));
    assert!(!labels.contains(&"Mountain".to_string()));
    assert!(!labels.contains(&"Lightning Bolt".to_string()));
    choose_label(&mut game, PlayerId::One, "Urza's Tower");
    drain_pending(&mut game);

    let affected = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == tower)
        .expect("Urza's Tower remains on the battlefield");
    let subtypes = game.effective_subtypes(affected);
    assert!(!subtypes.contains(&"Urza's") && !subtypes.contains(&"Tower"));
    assert_eq!(
        game.mana_ability_activations(affected).len(),
        5,
        "the land has exactly the granted five-color mana ability choices",
    );
}

#[test]
fn booby_trap_reveals_the_chosen_players_matching_draw_then_sacrifices_and_hits() {
    let mut game = ready_game();
    game.players[PlayerId::Two.index()].library.push(card(
        10_030,
        cards::LIGHTNING_BOLT,
        PlayerId::Two,
    ));
    let trap = game
        .put_onto_battlefield(PlayerId::One, cards::BOOBY_TRAP)
        .expect("Booby Trap is cataloged");
    let labels = pending_choice(&game, PlayerId::One)
        .options
        .iter()
        .map(|option| option.label.clone())
        .collect::<Vec<_>>();
    assert!(labels.contains(&"Lightning Bolt".to_string()));
    assert!(labels.contains(&"Badlands".to_string()));
    assert!(!labels.contains(&"Mountain".to_string()));
    choose_label(&mut game, PlayerId::One, "Lightning Bolt");
    drain_pending(&mut game);

    game.cards_drawn_this_turn[PlayerId::Two.index()] = 1;
    let event_start = game.events().len();
    game.draw_cards(PlayerId::Two, 1);
    drain_pending(&mut game);

    assert!(game.events()[event_start..].iter().any(|event| matches!(
        event,
        GameEvent::CardRevealed {
            player: PlayerId::Two,
            definition,
            ..
        } if *definition == cards::LIGHTNING_BOLT
    )));
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != trap),
        "the Trap sacrificed itself",
    );
    assert_eq!(game.players[PlayerId::Two.index()].life, 10);
}

#[test]
fn revoker_stops_mana_abilities_and_offers_only_nonland_names() {
    let mut game = ready_game();
    let bird = game
        .put_onto_battlefield(PlayerId::Two, cards::BIRDS_OF_PARADISE)
        .expect("Birds of Paradise is cataloged");
    game.put_onto_battlefield(PlayerId::One, cards::PHYREXIAN_REVOKER)
        .expect("Phyrexian Revoker is cataloged");

    let labels = pending_choice(&game, PlayerId::One)
        .options
        .iter()
        .map(|option| option.label.clone())
        .collect::<Vec<_>>();
    assert!(labels.contains(&"Birds of Paradise".to_string()));
    assert!(!labels.contains(&"Island".to_string()));
    choose_label(&mut game, PlayerId::One, "Birds of Paradise");
    drain_pending(&mut game);

    game.priority = PlayerId::Two;
    assert!(!has_mana_activation(&game, PlayerId::Two, bird));
}

#[test]
fn petrified_hamlet_grants_mana_while_suppressing_the_named_lands_other_abilities() {
    let mut game = ready_game();
    game.turns_started = [1, 1];
    let maze = game
        .put_onto_battlefield(PlayerId::Two, cards::MAZE_OF_ITH)
        .expect("Maze of Ith is cataloged");
    game.put_onto_battlefield(PlayerId::One, cards::PETRIFIED_HAMLET)
        .expect("Petrified Hamlet is cataloged");

    let labels = pending_choice(&game, PlayerId::One)
        .options
        .iter()
        .map(|option| option.label.clone())
        .collect::<Vec<_>>();
    assert!(labels.contains(&"Maze of Ith".to_string()));
    assert!(labels.contains(&"Island".to_string()));
    assert!(!labels.contains(&"Lightning Bolt".to_string()));
    choose_label(&mut game, PlayerId::One, "Maze of Ith");
    drain_pending(&mut game);

    game.priority = PlayerId::Two;
    assert!(!has_ordinary_activation(&game, PlayerId::Two, maze));
    assert!(has_mana_activation(&game, PlayerId::Two, maze));
}

#[test]
fn assembly_hall_reveals_its_cost_card_and_searches_for_its_name() {
    let mut game = ready_game();
    let hall = game
        .put_onto_battlefield(PlayerId::One, cards::ASSEMBLY_HALL)
        .expect("Assembly Hall is cataloged");
    let shown = card(10_040, cards::GRIZZLY_BEARS, PlayerId::One);
    game.players[PlayerId::One.index()].hand.push(shown.clone());
    game.players[PlayerId::One.index()].library = vec![
        card(10_041, cards::LIGHTNING_BOLT, PlayerId::One),
        card(10_042, cards::GRIZZLY_BEARS, PlayerId::One),
    ];
    game.players[PlayerId::One.index()].mana_pool.colorless = 4;

    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility {
                    source,
                    cost_objects,
                    ..
                } if *source == hall && cost_objects.as_slice() == [shown.id]
            )
        })
        .expect("the Hall offers the revealed creature as its cost");
    game.apply(PlayerId::One, activation).unwrap();
    assert!(game.events().iter().any(|event| matches!(
        event,
        GameEvent::CardRevealed { card, definition, .. }
            if *card == shown.id && *definition == cards::GRIZZLY_BEARS
    )));

    pass_priority_pair(&mut game);
    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the matching library card is offered");
    assert_eq!(decision.minimum, 0);
    let found = decision
        .options
        .iter()
        .find(|option| option.label == "Grizzly Bears")
        .expect("only the matching name is searchable")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![found],
        },
    )
    .unwrap();
    assert_eq!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .filter(|card| card.definition == cards::GRIZZLY_BEARS)
            .count(),
        2,
    );
}

#[test]
fn echoing_truth_moves_every_matching_permanent_and_no_others() {
    let mut game = ready_game();
    let first = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    game.put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .unwrap();
    let ring = game
        .put_onto_battlefield(PlayerId::Two, cards::SOL_RING)
        .unwrap();
    let truth = card(10_050, cards::ECHOING_TRUTH, PlayerId::One);
    game.players[PlayerId::One.index()].hand.push(truth.clone());
    game.players[PlayerId::One.index()].mana_pool.blue = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;

    game.apply(
        PlayerId::One,
        cast_action(truth.id, vec![Target::Permanent(first)], Vec::new(), 0),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.definition != cards::GRIZZLY_BEARS)
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == ring)
    );
}

#[test]
fn eye_of_singularity_ignores_basic_land_names_and_removes_only_older_duplicates() {
    let mut game = ready_game();
    game.put_onto_battlefield(PlayerId::One, cards::EYE_OF_SINGULARITY)
        .unwrap();
    drain_pending(&mut game);

    let first = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .unwrap();
    drain_pending(&mut game);
    let second = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .unwrap();
    drain_pending(&mut game);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != first)
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == second)
    );

    game.put_onto_battlefield(PlayerId::One, cards::ISLAND)
        .unwrap();
    game.put_onto_battlefield(PlayerId::Two, cards::ISLAND)
        .unwrap();
    drain_pending(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| permanent.card.definition == cards::ISLAND)
            .count(),
        2,
    );
}

#[test]
fn extirpate_searches_another_players_zones_without_giving_them_the_choices() {
    let mut game = ready_game();
    let graveyard_bolt = card(10_060, cards::LIGHTNING_BOLT, PlayerId::Two);
    game.players[PlayerId::Two.index()]
        .graveyard
        .push(graveyard_bolt.clone());
    game.players[PlayerId::Two.index()].hand.push(card(
        10_061,
        cards::LIGHTNING_BOLT,
        PlayerId::Two,
    ));
    game.players[PlayerId::Two.index()].library.push(card(
        10_062,
        cards::LIGHTNING_BOLT,
        PlayerId::Two,
    ));
    let matching_before = [
        &game.players[PlayerId::Two.index()].graveyard,
        &game.players[PlayerId::Two.index()].hand,
        &game.players[PlayerId::Two.index()].library,
    ]
    .into_iter()
    .flatten()
    .filter(|card| card.definition == cards::LIGHTNING_BOLT)
    .count();
    let exiled_before = game.players[PlayerId::Two.index()]
        .exile
        .iter()
        .filter(|card| card.definition == cards::LIGHTNING_BOLT)
        .count();
    let extirpate = card(10_063, cards::EXTIRPATE, PlayerId::One);
    game.players[PlayerId::One.index()]
        .hand
        .push(extirpate.clone());
    game.players[PlayerId::One.index()].mana_pool.black = 1;

    game.apply(
        PlayerId::One,
        cast_action(
            extirpate.id,
            vec![Target::Card(graveyard_bolt.id)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    for zone_index in 0..3 {
        let decision = game
            .observe(PlayerId::One)
            .decision
            .unwrap_or_else(|| panic!("Extirpate offers searched zone {zone_index} in order"));
        assert_eq!(decision.player, PlayerId::One);
        let options = decision.options.iter().map(|option| option.id).collect();
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: decision.id,
                options,
            },
        )
        .unwrap();
    }

    assert_eq!(
        game.players[PlayerId::Two.index()]
            .exile
            .iter()
            .filter(|card| card.definition == cards::LIGHTNING_BOLT)
            .count(),
        exiled_before + matching_before,
    );
}

#[test]
fn counterbore_keeps_the_countered_spells_name_and_controller_for_its_searches() {
    let mut game = ready_game();
    game.players[PlayerId::Two.index()].graveyard.push(card(
        10_070,
        cards::LIGHTNING_BOLT,
        PlayerId::Two,
    ));
    let bolt = card(10_071, cards::LIGHTNING_BOLT, PlayerId::Two);
    game.players[PlayerId::Two.index()].hand.push(bolt.clone());
    game.players[PlayerId::Two.index()].hand.push(card(
        10_072,
        cards::LIGHTNING_BOLT,
        PlayerId::Two,
    ));
    game.players[PlayerId::Two.index()].library.push(card(
        10_073,
        cards::LIGHTNING_BOLT,
        PlayerId::Two,
    ));
    let matching_before = [
        &game.players[PlayerId::Two.index()].graveyard,
        &game.players[PlayerId::Two.index()].hand,
        &game.players[PlayerId::Two.index()].library,
    ]
    .into_iter()
    .flatten()
    .filter(|card| card.definition == cards::LIGHTNING_BOLT)
    .count();
    let exiled_before = game.players[PlayerId::Two.index()]
        .exile
        .iter()
        .filter(|card| card.definition == cards::LIGHTNING_BOLT)
        .count();

    let counterbore = card(10_074, cards::COUNTERBORE, PlayerId::One);
    game.players[PlayerId::One.index()]
        .hand
        .push(counterbore.clone());
    game.players[PlayerId::Two.index()].mana_pool.red = 1;
    game.players[PlayerId::One.index()].mana_pool.blue = 5;
    game.priority = PlayerId::Two;

    game.apply(
        PlayerId::Two,
        cast_action(bolt.id, vec![Target::Player(PlayerId::One)], Vec::new(), 0),
    )
    .expect("the named spell is cast");
    let bolt_on_stack = game.stack.last().expect("the Bolt is on the stack").id;
    game.apply(PlayerId::Two, Action::PassPriority)
        .expect("the opponent receives priority");
    game.apply(
        PlayerId::One,
        cast_action(
            counterbore.id,
            vec![Target::Spell(bolt_on_stack)],
            Vec::new(),
            0,
        ),
    )
    .expect("Counterbore targets the spell");
    pass_priority_pair(&mut game);

    for zone_index in 0..3 {
        let decision = game
            .observe(PlayerId::One)
            .decision
            .unwrap_or_else(|| panic!("Counterbore offers searched zone {zone_index} in order"));
        assert_eq!(decision.player, PlayerId::One);
        let options = decision.options.iter().map(|option| option.id).collect();
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: decision.id,
                options,
            },
        )
        .unwrap();
    }

    assert_eq!(
        game.players[PlayerId::Two.index()]
            .exile
            .iter()
            .filter(|card| card.definition == cards::LIGHTNING_BOLT)
            .count(),
        exiled_before + matching_before,
    );
}
