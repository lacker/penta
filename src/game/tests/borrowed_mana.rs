//! Mana types selected from what referenced permanents could produce.

use super::*;

fn offered_types(game: &Game, player: PlayerId, source: GameObjectId) -> Vec<ManaColor> {
    let mut types = game
        .legal_actions(player)
        .into_iter()
        .filter_map(|action| match action {
            Action::ActivateManaAbility {
                source: candidate,
                color,
                ..
            } if candidate == source => Some(color),
            _ => None,
        })
        .collect::<Vec<_>>();
    types.sort_unstable();
    types.dedup();
    types
}

fn activate_for(game: &mut Game, player: PlayerId, source: GameObjectId, color: ManaColor) {
    game.priority = player;
    let action = game
        .legal_actions(player)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateManaAbility {
                    source: candidate,
                    color: candidate_color,
                    ..
                } if *candidate == source && *candidate_color == color
            )
        })
        .expect("the requested mana type is offered");
    game.apply(player, action)
        .expect("the mana ability resolves");
}

#[test]
fn reflecting_pool_includes_colorless_but_only_reads_your_lands() {
    let mut game = ready_game();
    let pool = creature(10_000, cards::REFLECTING_POOL, PlayerId::One);
    let pool_id = pool.card.id;
    game.battlefield.extend([
        pool,
        creature(10_001, cards::FOREST, PlayerId::One),
        creature(10_002, cards::ANCIENT_TOMB, PlayerId::One),
        creature(10_003, cards::ISLAND, PlayerId::Two),
    ]);

    assert_eq!(
        offered_types(&game, PlayerId::One, pool_id),
        vec![ManaColor::Green, ManaColor::Colorless],
    );
}

#[test]
fn fellwar_stone_and_exotic_orchard_offer_opponents_land_colors_not_colorless() {
    for definition in [cards::FELLWAR_STONE, cards::EXOTIC_ORCHARD] {
        let mut game = ready_game();
        let source = creature(10_000, definition, PlayerId::One);
        let source_id = source.card.id;
        let mut island = creature(10_002, cards::ISLAND, PlayerId::Two);
        island.tapped = true;
        game.battlefield.extend([
            source,
            creature(10_001, cards::FOREST, PlayerId::One),
            island,
            creature(10_003, cards::ANCIENT_TOMB, PlayerId::Two),
        ]);

        assert_eq!(
            offered_types(&game, PlayerId::One, source_id),
            vec![ManaColor::Blue],
            "{definition:?} reads the opponent and filters colorless",
        );
    }
}

#[test]
fn could_produce_ignores_whether_the_lenders_activation_cost_can_be_paid() {
    let mut game = ready_game();
    let stone = creature(10_000, cards::FELLWAR_STONE, PlayerId::One);
    let stone_id = stone.card.id;
    // Constructed directly on the battlefield, the Mine has no mining
    // counters and cannot pay its own mana ability's removal cost.
    let empty_mine = creature(10_001, cards::GEMSTONE_MINE, PlayerId::Two);
    game.battlefield.extend([stone, empty_mine]);

    assert_eq!(
        offered_types(&game, PlayerId::One, stone_id),
        ManaColor::COLORS,
    );
}

#[test]
fn could_produce_ignores_cavern_restrictions_and_spend_rider() {
    for definition in [
        cards::REFLECTING_POOL,
        cards::FELLWAR_STONE,
        cards::EXOTIC_ORCHARD,
    ] {
        let mut game = ready_game();
        let controller = PlayerId::One;
        let lender = if definition == cards::REFLECTING_POOL {
            controller
        } else {
            PlayerId::Two
        };
        let source = creature(10_000, definition, controller);
        let source_id = source.card.id;
        let mut cavern = creature(10_001, cards::CAVERN_OF_SOULS, lender);
        cavern.chosen_creature_type = Some("Soldier".into());
        game.battlefield.extend([source, cavern]);

        let expected = if definition == cards::REFLECTING_POOL {
            ManaColor::ALL.to_vec()
        } else {
            ManaColor::COLORS.to_vec()
        };
        assert_eq!(
            offered_types(&game, controller, source_id),
            expected,
            "Cavern can produce every color regardless of its spending clause",
        );
        activate_for(&mut game, controller, source_id, ManaColor::Blue);

        let mana = game.players[controller.index()]
            .mana
            .iter()
            .find(|mana| mana.source.is_some_and(|source| source.object == source_id))
            .expect("the borrowing permanent produced mana");
        assert!(mana.restrictions.is_empty());
        assert!(mana.spend_effects.is_empty());
    }
}

#[test]
fn mana_flare_copies_the_type_cavern_produced_without_its_riders() {
    let mut game = ready_game();
    let mut cavern = creature(10_000, cards::CAVERN_OF_SOULS, PlayerId::One);
    cavern.chosen_creature_type = Some("Soldier".into());
    let cavern_id = cavern.card.id;
    let flare = creature(10_001, cards::MANA_FLARE, PlayerId::Two);
    let flare_id = flare.card.id;
    game.battlefield.extend([cavern, flare]);

    activate_for(&mut game, PlayerId::One, cavern_id, ManaColor::Blue);

    let mana = &game.players[PlayerId::One.index()].mana;
    assert_eq!(mana.len(), 2);
    let cavern_mana = mana
        .iter()
        .find(|mana| mana.source.is_some_and(|source| source.object == cavern_id))
        .expect("Cavern produced one mana");
    assert!(!cavern_mana.restrictions.is_empty());
    assert!(!cavern_mana.spend_effects.is_empty());
    let flare_mana = mana
        .iter()
        .find(|mana| mana.source.is_some_and(|source| source.object == flare_id))
        .expect("Mana Flare produced the additional mana");
    assert_eq!(flare_mana.color, ManaColor::Blue);
    assert!(flare_mana.restrictions.is_empty());
    assert!(flare_mana.spend_effects.is_empty());
}

#[test]
fn mana_flare_uses_the_type_actually_produced_not_every_type_cavern_could_produce() {
    let mut game = ready_game();
    let cavern = creature(10_000, cards::CAVERN_OF_SOULS, PlayerId::One);
    let cavern_id = cavern.card.id;
    game.battlefield
        .extend([cavern, creature(10_001, cards::MANA_FLARE, PlayerId::One)]);

    activate_for(&mut game, PlayerId::One, cavern_id, ManaColor::Colorless);

    assert_eq!(game.players[PlayerId::One.index()].mana_pool.colorless, 2);
    assert_eq!(game.players[PlayerId::One.index()].mana_pool.total(), 2);
}

#[test]
fn could_produce_cycles_only_propagate_a_type_from_a_real_producer() {
    let mut game = ready_game();
    let first = creature(10_000, cards::EXOTIC_ORCHARD, PlayerId::One);
    let first_id = first.card.id;
    let second = creature(10_001, cards::EXOTIC_ORCHARD, PlayerId::Two);
    let second_id = second.card.id;
    game.battlefield.extend([first, second]);

    assert!(offered_types(&game, PlayerId::One, first_id).is_empty());
    game.priority = PlayerId::Two;
    assert!(offered_types(&game, PlayerId::Two, second_id).is_empty());

    game.battlefield
        .push(creature(10_002, cards::FOREST, PlayerId::Two));
    game.priority = PlayerId::One;
    assert_eq!(
        offered_types(&game, PlayerId::One, first_id),
        vec![ManaColor::Green],
    );
    game.priority = PlayerId::Two;
    assert_eq!(
        offered_types(&game, PlayerId::Two, second_id),
        vec![ManaColor::Green],
    );
}
