use super::*;
use crate::card::abilities;

static CARD_COST_FLASHBACK: AbilityDef = abilities::flashback_for_card_mana_cost();

fn flashback_action(game: &Game, card: GameObjectId, x: u16, targets: &[Target]) -> Action {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell { card: candidate, choices, .. }
                    if *candidate == card
                        && choices.x() == x
                        && choices.iter_targets().copied().eq(targets.iter().copied())
                        && choices.costs().alternative().is_some()
            )
        })
        .expect("the granted flashback cast is legal")
}

#[test]
fn white_suns_zenith_creates_cats_then_returns_and_shuffles() {
    let mut game = ready_game();
    let zenith = card(40_000, cards::WHITE_SUNS_ZENITH, PlayerId::One);
    game.players[0].hand.push(zenith.clone());
    game.players[0].library = vec![
        card(40_001, cards::LIGHTNING_BOLT, PlayerId::One),
        card(40_002, cards::COUNTERSPELL, PlayerId::One),
        card(40_003, cards::MOUNTAIN, PlayerId::One),
    ];
    game.players[0].mana_pool.white = 2;
    game.players[0].mana_pool.colorless = 2;

    game.apply(
        PlayerId::One,
        cast_action(zenith.id, Vec::new(), Vec::new(), 2),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| is_token_with(
                permanent,
                tokens::creature(&["Cat"], &[ManaColor::White], 2, 2)
            ))
            .count(),
        2,
    );
    assert!(
        game.players[0]
            .library
            .iter()
            .any(|card| card.definition == cards::WHITE_SUNS_ZENITH)
    );
    assert!(game.players[0].exile.is_empty());
    assert_ne!(
        game.players[0]
            .library
            .iter()
            .take(3)
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::LIGHTNING_BOLT, cards::COUNTERSPELL, cards::MOUNTAIN],
        "the library is shuffled after Zenith moves into it",
    );
}

#[test]
fn white_suns_zenith_copy_still_shuffles_its_owners_library() {
    let mut game = ready_game();
    let zenith = card(40_100, cards::WHITE_SUNS_ZENITH, PlayerId::One);
    game.players[0].hand.push(zenith.clone());
    game.players[0].library = vec![
        card(40_101, cards::LIGHTNING_BOLT, PlayerId::One),
        card(40_102, cards::COUNTERSPELL, PlayerId::One),
        card(40_103, cards::MOUNTAIN, PlayerId::One),
    ];
    game.players[0].mana_pool.white = 2;
    game.players[0].mana_pool.colorless = 1;
    game.apply(
        PlayerId::One,
        cast_action(zenith.id, Vec::new(), Vec::new(), 1),
    )
    .unwrap();
    game.push_copy(game.stack[0].clone(), PlayerId::One, Vec::new());

    pass_priority_pair(&mut game);

    assert_ne!(
        game.players[0]
            .library
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::LIGHTNING_BOLT, cards::COUNTERSPELL, cards::MOUNTAIN],
        "the resolving copy shuffles even though it has no card to move",
    );
    assert_eq!(game.stack.len(), 1, "the original remains on the stack");
}

#[test]
fn flashback_replaces_white_suns_zeniths_move_but_not_its_shuffle() {
    let mut game = ready_game();
    let zenith = card(40_200, cards::WHITE_SUNS_ZENITH, PlayerId::One);
    game.players[0].graveyard.push(zenith.clone());
    game.players[0].library = vec![
        card(40_201, cards::LIGHTNING_BOLT, PlayerId::One),
        card(40_202, cards::COUNTERSPELL, PlayerId::One),
        card(40_203, cards::MOUNTAIN, PlayerId::One),
    ];
    game.temporary_ability_grants.push(TemporaryAbilityGrant {
        object: zenith.id,
        ability: CARD_COST_FLASHBACK,
    });
    game.players[0].mana_pool.white = 2;
    game.players[0].mana_pool.colorless = 1;

    game.apply(PlayerId::One, flashback_action(&game, zenith.id, 1, &[]))
        .unwrap();
    pass_priority_pair(&mut game);

    assert!(
        game.players[0]
            .exile
            .iter()
            .any(|card| card.definition == cards::WHITE_SUNS_ZENITH)
    );
    assert_ne!(
        game.players[0]
            .library
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::LIGHTNING_BOLT, cards::COUNTERSPELL, cards::MOUNTAIN],
        "Flashback replaces only Zenith's move, leaving its shuffle intact",
    );
}

#[test]
fn reality_strobe_keeps_its_time_counters_when_flashback_resolves() {
    let mut game = ready_game();
    let strobe = card(40_300, cards::REALITY_STROBE, PlayerId::One);
    let target = creature(40_301, cards::SAVANNAH_LIONS, PlayerId::Two);
    let target_id = target.card.id;
    game.players[0].graveyard.push(strobe.clone());
    game.battlefield.push(target);
    game.temporary_ability_grants.push(TemporaryAbilityGrant {
        object: strobe.id,
        ability: CARD_COST_FLASHBACK,
    });
    game.players[0].mana_pool.blue = 2;
    game.players[0].mana_pool.colorless = 4;

    game.apply(
        PlayerId::One,
        flashback_action(&game, strobe.id, 0, &[Target::Permanent(target_id)]),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert!(
        game.battlefield
            .iter()
            .all(|card| card.card.id != target_id)
    );
    assert!(
        game.players[1]
            .hand
            .iter()
            .any(|card| card.definition == cards::SAVANNAH_LIONS)
    );
    let exiled = game.players[0]
        .exile
        .iter()
        .find(|card| card.definition == cards::REALITY_STROBE)
        .expect("Reality Strobe is exiled by its own resolution");
    assert_eq!(exiled.counters(CounterKind::named("time")), 3);
    let observed = game.observe(PlayerId::One);
    assert_eq!(
        observed.card_counters,
        vec![CardCounterObservation {
            object: exiled.id,
            counters: vec![CounterObservation {
                name: "time".to_owned(),
                count: 3,
            }],
        }]
    );
}

#[test]
fn boomerang_returns_a_permanent_to_its_owners_hand() {
    let mut game = ready_game();
    let boomerang = card(40_400, cards::BOOMERANG, PlayerId::One);
    let target = creature(40_401, cards::SAVANNAH_LIONS, PlayerId::Two);
    let target_id = target.card.id;
    game.players[0].hand.push(boomerang.clone());
    game.players[0].mana_pool.blue = 2;
    game.battlefield.push(target);

    game.apply(
        PlayerId::One,
        cast_action(
            boomerang.id,
            vec![Target::Permanent(target_id)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert!(
        game.battlefield
            .iter()
            .all(|card| card.card.id != target_id)
    );
    assert!(
        game.players[1]
            .hand
            .iter()
            .any(|card| card.definition == cards::SAVANNAH_LIONS)
    );
}

#[test]
fn artful_maneuver_exiles_itself_after_its_combat_trick_resolves() {
    let mut game = ready_game();
    let maneuver = card(40_500, cards::ARTFUL_MANEUVER, PlayerId::One);
    let target = creature(40_501, cards::SAVANNAH_LIONS, PlayerId::One);
    let target_id = target.card.id;
    game.players[0].hand.push(maneuver.clone());
    game.players[0].mana_pool.white = 1;
    game.players[0].mana_pool.colorless = 1;
    game.battlefield.push(target);

    game.apply(
        PlayerId::One,
        cast_action(
            maneuver.id,
            vec![Target::Permanent(target_id)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(
        game.battlefield
            .iter()
            .find(|card| card.card.id == target_id)
            .and_then(|card| game.creature_stats(card))
            .map(|stats| (stats.power, stats.toughness)),
        Some((4, 3)),
    );
    assert!(
        game.players[0]
            .exile
            .iter()
            .any(|card| card.definition == cards::ARTFUL_MANEUVER)
    );
}

#[test]
fn a_fizzled_artful_maneuver_does_not_rebound() {
    let mut game = ready_game();
    let maneuver = card(40_600, cards::ARTFUL_MANEUVER, PlayerId::One);
    let target = creature(40_601, cards::SAVANNAH_LIONS, PlayerId::One);
    let target_id = target.card.id;
    game.players[0].hand.push(maneuver.clone());
    game.players[0].mana_pool.white = 1;
    game.players[0].mana_pool.colorless = 1;
    game.battlefield.push(target);

    game.apply(
        PlayerId::One,
        cast_action(
            maneuver.id,
            vec![Target::Permanent(target_id)],
            Vec::new(),
            0,
        ),
    )
    .unwrap();
    game.destroy_permanent(target_id);
    pass_priority_pair(&mut game);

    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::ARTFUL_MANEUVER)
    );
    assert!(game.players[0].exile.is_empty());
}
