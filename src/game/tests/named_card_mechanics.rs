//! Cards whose entry choice names another card, and the shared rules that
//! consume that stored name.

use super::*;

const REQUESTED: [CardDefinitionId; 8] = [
    cards::NEVERMORE,
    cards::PITHING_NEEDLE,
    cards::DISRUPTOR_FLUTE,
    cards::SORCEROUS_SPYGLASS,
    cards::ALPINE_MOON,
    cards::ANOINTED_PEACEKEEPER,
    cards::BOOBY_TRAP,
    cards::CABAL_THERAPY,
];

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
fn requested_named_card_definitions_are_complete_and_declarative() {
    let catalog = poc::catalog().expect("the catalog builds");
    for identity in REQUESTED {
        let definition = catalog
            .get(identity)
            .expect("the requested card is cataloged");
        assert_eq!(
            definition.rules.implementation_status(),
            crate::ImplementationStatus::Complete,
            "{} is complete",
            definition.name,
        );
        assert_eq!(
            definition.rules.special_behavior(),
            None,
            "{} uses shared declarative mechanics",
            definition.name,
        );
    }
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
fn spyglass_looks_privately_before_the_public_name_choice() {
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
    let labels = pending_choice(&game, PlayerId::One)
        .options
        .iter()
        .map(|option| option.label.clone())
        .collect::<Vec<_>>();
    assert!(labels.contains(&"Lightning Bolt".to_string()));
    assert!(
        labels.contains(&"Island".to_string()),
        "Spyglass may name a land"
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
