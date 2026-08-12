use super::*;

#[test]
fn reanimate_steals_a_creature_card_and_charges_its_mana_value() {
    let mut game = ready_game();
    let angel = card(10_000, cards::SERRA_ANGEL, PlayerId::Two);
    let target = angel.id;
    game.players[PlayerId::Two.index()].graveyard.push(angel);
    let reanimate = card(10_001, cards::REANIMATE, PlayerId::One);
    game.players[PlayerId::One.index()]
        .hand
        .push(reanimate.clone());
    game.players[PlayerId::One.index()].mana_pool.black = 1;

    game.apply(
        PlayerId::One,
        cast_action(reanimate.id, vec![Target::Card(target)], Vec::new(), 0),
    )
    .unwrap();
    drain_pending(&mut game);

    let angel = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::SERRA_ANGEL)
        .expect("Serra Angel was reanimated");
    assert_eq!(angel.controller, PlayerId::One);
    assert!(game.players[PlayerId::Two.index()].graveyard.is_empty());
    assert_eq!(game.players[PlayerId::One.index()].life, 15);
}

#[test]
fn tormods_crypt_exiles_only_the_target_players_graveyard() {
    let mut game = ready_game();
    let crypt = creature(10_000, cards::TORMODS_CRYPT, PlayerId::One);
    let source = crypt.card.id;
    game.battlefield.push(crypt);
    game.players[PlayerId::One.index()].graveyard.push(card(
        10_001,
        cards::MOUNTAIN,
        PlayerId::One,
    ));
    game.players[PlayerId::Two.index()].graveyard.extend([
        card(10_002, cards::SERRA_ANGEL, PlayerId::Two),
        card(10_003, cards::COUNTERSPELL, PlayerId::Two),
    ]);

    game.apply(
        PlayerId::One,
        Action::ActivateAbility {
            source,
            ability: activated_ability_for(&game, source, 0),
            targets: activated_targets(Target::Player(PlayerId::Two)),
            cost_object: None,
            x: 0,
        },
    )
    .unwrap();
    drain_pending(&mut game);

    assert!(game.players[PlayerId::Two.index()].graveyard.is_empty());
    assert_eq!(game.players[PlayerId::Two.index()].exile.len(), 2);
    assert!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::MOUNTAIN),
        "the other player's graveyard is untouched"
    );
    assert!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::TORMODS_CRYPT),
        "the Crypt was sacrificed as an activation cost"
    );
}

#[test]
fn mana_short_taps_the_target_players_lands_and_clears_their_mana() {
    let mut game = ready_game();
    let island = creature(10_000, cards::ISLAND, PlayerId::Two);
    let mountain = creature(10_001, cards::MOUNTAIN, PlayerId::Two);
    game.battlefield.extend([island, mountain]);
    game.players[PlayerId::Two.index()].mana_pool.red = 2;
    game.players[PlayerId::Two.index()].mana_pool.colorless = 1;
    let mana_short = card(10_002, cards::MANA_SHORT, PlayerId::One);
    game.players[PlayerId::One.index()]
        .hand
        .push(mana_short.clone());
    game.players[PlayerId::One.index()].mana_pool.blue = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;

    game.apply(
        PlayerId::One,
        cast_action(
            mana_short.id,
            vec![Target::Player(PlayerId::Two)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    drain_pending(&mut game);

    assert!(
        game.battlefield
            .iter()
            .filter(|permanent| permanent.controller == PlayerId::Two)
            .all(|permanent| permanent.tapped)
    );
    assert_eq!(
        game.players[PlayerId::Two.index()].mana_pool,
        ManaPool::default()
    );
    assert_eq!(game.players[PlayerId::Two.index()].life, 20);
}

#[test]
fn presence_of_the_master_counters_enchantment_spells() {
    let mut game = ready_game();
    game.battlefield.push(creature(
        10_000,
        cards::PRESENCE_OF_THE_MASTER,
        PlayerId::One,
    ));
    let enchantment = card(10_001, cards::ENERGY_FLUX, PlayerId::Two);
    game.players[PlayerId::Two.index()]
        .hand
        .push(enchantment.clone());
    game.players[PlayerId::Two.index()].mana_pool.blue = 1;
    game.players[PlayerId::Two.index()].mana_pool.colorless = 2;
    game.active_player = PlayerId::Two;
    game.priority = PlayerId::Two;

    game.apply(
        PlayerId::Two,
        cast_action(enchantment.id, Vec::new(), Vec::new(), 0),
    )
    .unwrap();
    drain_pending(&mut game);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.definition != cards::ENERGY_FLUX)
    );
    assert!(
        game.players[PlayerId::Two.index()]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::ENERGY_FLUX)
    );
}

#[test]
fn stasis_stops_both_players_untaps_and_dies_when_its_upkeep_is_unpaid() {
    let mut game = ready_game();
    let stasis = creature(10_000, cards::STASIS, PlayerId::One);
    game.battlefield.push(stasis);
    let mut island = creature(10_001, cards::ISLAND, PlayerId::One);
    island.tapped = true;
    let mut mountain = creature(10_002, cards::MOUNTAIN, PlayerId::Two);
    mountain.tapped = true;
    game.battlefield.extend([island, mountain]);
    game.active_player = PlayerId::Two;
    game.next_regular_player = PlayerId::One;

    game.start_next_turn();
    assert_eq!(game.active_player, PlayerId::One);
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.definition == cards::ISLAND)
            .expect("Island remains")
            .tapped,
        "Stasis prevents its controller from untapping"
    );
    drain_pending(&mut game);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.definition != cards::STASIS),
        "without blue mana the upkeep trigger sacrifices Stasis"
    );

    game.start_next_turn();
    assert_eq!(game.active_player, PlayerId::Two);
    assert!(
        !game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.definition == cards::MOUNTAIN)
            .expect("Mountain remains")
            .tapped,
        "ordinary untapping resumes after Stasis leaves"
    );
}
