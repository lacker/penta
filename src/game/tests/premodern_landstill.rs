//! The cards the Premodern Landstill list needed.

use super::*;

fn put_in_graveyard(game: &mut Game, id: u32, definition: CardDefinitionId, owner: PlayerId) {
    let card = card(id, definition, owner);
    game.players[owner.index()].graveyard.push(card);
}

fn graveyard_definitions(game: &Game, player: PlayerId) -> Vec<CardDefinitionId> {
    game.players[player.index()]
        .graveyard
        .iter()
        .map(|card| card.definition)
        .collect()
}

#[test]
fn phyrexian_furnace_eats_a_graveyard_from_the_bottom() {
    let mut game = ready_game();
    let furnace = creature(10_000, cards::PHYREXIAN_FURNACE, PlayerId::One);
    let furnace_id = furnace.card.id;
    game.battlefield.push(furnace);
    // Oldest first, which is the bottom of the pile.
    put_in_graveyard(&mut game, 10_001, cards::LIGHTNING_BOLT, PlayerId::Two);
    put_in_graveyard(&mut game, 10_002, cards::COUNTERSPELL, PlayerId::Two);

    let activation = Action::ActivateAbility {
        source: furnace_id,
        ability: primary_ability(cards::PHYREXIAN_FURNACE),
        targets: activated_targets(Target::Player(PlayerId::Two)),
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
    };
    assert!(game.legal_actions(PlayerId::One).contains(&activation));
    game.apply(PlayerId::One, activation).unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(
        graveyard_definitions(&game, PlayerId::Two),
        vec![cards::COUNTERSPELL],
        "the oldest card went, not the newest",
    );
    assert!(
        game.players[PlayerId::Two.index()]
            .exile
            .iter()
            .any(|card| card.definition == cards::LIGHTNING_BOLT),
        "and it is in exile",
    );
}

/// The other mode picks the card that mattered, and pays for itself.
#[test]
fn sacrificing_the_furnace_exiles_a_chosen_card_and_draws() {
    let mut game = ready_game();
    let furnace = creature(10_000, cards::PHYREXIAN_FURNACE, PlayerId::One);
    let furnace_id = furnace.card.id;
    game.battlefield.push(furnace);
    put_in_graveyard(&mut game, 10_001, cards::LIGHTNING_BOLT, PlayerId::Two);
    put_in_graveyard(&mut game, 10_002, cards::COUNTERSPELL, PlayerId::Two);
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;
    let before = game.players[PlayerId::One.index()].hand.len();

    let newest = game.players[PlayerId::Two.index()]
        .graveyard
        .last()
        .expect("the graveyard has cards")
        .id;
    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility { source, targets, .. }
                    if *source == furnace_id
                        && targets.iter().any(|selection| {
                            selection.targets().contains(&Target::Card(newest))
                        })
            )
        })
        .expect("the newest card is targetable too");
    game.apply(PlayerId::One, activation).unwrap();
    pass_priority_pair(&mut game);

    assert_eq!(
        graveyard_definitions(&game, PlayerId::Two),
        vec![cards::LIGHTNING_BOLT],
        "the chosen card went rather than the bottom one",
    );
    assert_eq!(
        game.players[PlayerId::One.index()].hand.len(),
        before + 1,
        "and the Furnace replaced itself",
    );
}

#[test]
fn powder_keg_destroys_what_its_fuse_counters_name() {
    let mut game = ready_game();
    let keg = creature(10_000, cards::POWDER_KEG, PlayerId::One);
    let keg_id = keg.card.id;
    game.battlefield.push(keg);
    // Two fuse counters, so two-drops die and nothing else does.
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == keg_id)
    {
        permanent.add_counters(CounterKind::named("fuse"), 2);
    }
    let two_drop = creature(10_001, cards::QUIRION_DRYAD, PlayerId::Two);
    let two_drop_id = two_drop.card.id;
    game.battlefield.push(two_drop);
    let one_drop = creature(10_002, cards::MOGG_FANATIC, PlayerId::Two);
    let one_drop_id = one_drop.card.id;
    game.battlefield.push(one_drop);

    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == keg_id),
        )
        .expect("the Keg can be detonated");
    game.apply(PlayerId::One, activation).unwrap();
    pass_priority_pair(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == two_drop_id),
        "a two-drop matched two fuse counters",
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == one_drop_id),
        "a one-drop did not",
    );
}

/// Humility is a layers question: the abilities go in layer 6 and the stats
/// are set in 7b, so a creature it catches keeps neither.
#[test]
fn humility_flattens_every_creature_on_the_board() {
    let mut game = ready_game();
    let flier = creature(10_001, cards::GOBLIN_BALLOON_BRIGADE, PlayerId::One);
    let flier_id = flier.card.id;
    game.battlefield.push(flier);
    let big = creature(10_002, cards::GOBLIN_RINGLEADER, PlayerId::Two);
    let big_id = big.card.id;
    game.battlefield.push(big);

    let humility = creature(10_000, cards::HUMILITY, PlayerId::One);
    game.battlefield.push(humility);
    drain_pending(&mut game);

    for id in [flier_id, big_id] {
        let permanent = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .expect("still on the battlefield");
        assert_eq!(game.power(permanent), Some(1));
        assert_eq!(game.toughness(permanent), Some(1));
    }
    let brigade = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == flier_id)
        .expect("still there");
    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == brigade.card.id)),
        "its activated ability is gone with the rest",
    );
}

/// Thawing Glaciers is the reason cleanup raises a trigger at all: it fetches
/// and then leaves, so it is available again next turn.
#[test]
fn thawing_glaciers_fetches_a_basic_and_returns_at_cleanup() {
    let mut game = ready_game();
    let glaciers = creature(10_000, cards::THAWING_GLACIERS, PlayerId::One);
    let glaciers_id = glaciers.card.id;
    game.battlefield.push(glaciers);
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;
    let lands_before = game
        .battlefield
        .iter()
        .filter(|permanent| permanent.controller == PlayerId::One)
        .count();

    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == glaciers_id)
        })
        .expect("the Glaciers can fetch");
    game.apply(PlayerId::One, activation).unwrap();
    drain_pending(&mut game);

    assert!(
        game.battlefield
            .iter()
            .filter(|permanent| permanent.controller == PlayerId::One)
            .count()
            > lands_before,
        "a basic arrived",
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == glaciers_id),
        "and the Glaciers is still there until cleanup",
    );

    game.step = Step::Cleanup;
    game.complete_cleanup();
    drain_pending(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == glaciers_id),
        "cleanup took it back",
    );
    assert!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .any(|card| card.definition == cards::THAWING_GLACIERS),
        "to its owner's hand",
    );
}

/// Cycling Decree of Justice asks how much to pay, and the answer is the
/// number of Soldiers: X is settled by the payment rather than by a cast.
#[test]
fn cycling_decree_of_justice_buys_soldiers_by_the_mana() {
    let mut game = ready_game();
    let decree = card(10_000, cards::DECREE_OF_JUSTICE, PlayerId::One);
    let decree_id = decree.id;
    game.players[PlayerId::One.index()].hand.push(decree);
    let pool = &mut game.players[PlayerId::One.index()].mana_pool;
    pool.white = 1;
    pool.colorless = 5;

    let cycling = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == decree_id)
        })
        .expect("the Decree can be cycled");
    game.apply(PlayerId::One, cycling).unwrap();
    pass_priority_pair(&mut game);

    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the cycling trigger asks how much to pay");
    let three = decision
        .options
        .iter()
        .find(|option| option.id == 3)
        .expect("three is affordable with three left")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![three],
        },
    )
    .expect("paying three is legal");
    drain_pending(&mut game);

    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| is_token_with(
                permanent,
                tokens::creature(&["Soldier"], &[ManaColor::White], 1, 1)
            ))
            .count(),
        3,
        "three mana bought three Soldiers",
    );
}

/// Declining is always available, and buys nothing.
#[test]
fn declining_the_decrees_trigger_makes_no_soldiers() {
    let mut game = ready_game();
    let decree = card(10_000, cards::DECREE_OF_JUSTICE, PlayerId::One);
    let decree_id = decree.id;
    game.players[PlayerId::One.index()].hand.push(decree);
    let pool = &mut game.players[PlayerId::One.index()].mana_pool;
    pool.white = 1;
    pool.colorless = 5;

    let cycling = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == decree_id)
        })
        .expect("the Decree can be cycled");
    game.apply(PlayerId::One, cycling).unwrap();
    pass_priority_pair(&mut game);

    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the cycling trigger asks how much to pay");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![0],
        },
    )
    .expect("declining is legal");
    drain_pending(&mut game);

    assert!(
        !game.battlefield.iter().any(|permanent| is_token_with(
            permanent,
            tokens::creature(&["Soldier"], &[ManaColor::White], 1, 1)
        )),
        "nothing was paid, so nothing arrived",
    );
    assert_eq!(
        game.players[PlayerId::One.index()].mana_pool.colorless,
        3,
        "and only the two generic the cycling cost left the pool",
    );
}

/// The answer to Wasteland: the land lives, the thing that came for it dies.
#[test]
fn teferis_response_counters_a_land_destruction_ability_and_kills_its_source() {
    let mut game = ready_game();
    // Nonbasic, because that is what Wasteland can aim at.
    let factory = creature(10_001, cards::MISHRA_S_FACTORY, PlayerId::One);
    let factory_id = factory.card.id;
    game.battlefield.push(factory);
    let mut wasteland = creature(10_002, cards::WASTELAND, PlayerId::Two);
    wasteland.tapped = false;
    let wasteland_id = wasteland.card.id;
    game.battlefield.push(wasteland);

    // The opponent aims Wasteland at the Island.
    game.priority = PlayerId::Two;
    let activation = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility { source, targets, .. }
                    if *source == wasteland_id
                        && targets.iter().any(|selection| {
                            selection.targets().contains(&Target::Permanent(factory_id))
                        })
            )
        })
        .expect("Wasteland can aim at the Factory");
    game.apply(PlayerId::Two, activation).unwrap();

    let ability = game.stack.last().expect("the ability is on the stack").id;
    let response = card(10_000, cards::TEFERIS_RESPONSE, PlayerId::One);
    let response_id = response.id;
    game.players[PlayerId::One.index()].hand.push(response);
    let pool = &mut game.players[PlayerId::One.index()].mana_pool;
    pool.blue = 1;
    pool.colorless = 1;
    game.priority = PlayerId::One;
    let before = game.players[PlayerId::One.index()].hand.len();

    game.apply(
        PlayerId::One,
        cast_action(response_id, vec![Target::Spell(ability)], Vec::new(), 0),
    )
    .expect("the Response can answer a land-targeting ability");
    drain_pending(&mut game);

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == factory_id),
        "the Factory survived",
    );
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == wasteland_id),
        "and the Wasteland did not",
    );
    assert_eq!(
        game.players[PlayerId::One.index()].hand.len(),
        before - 1 + 2,
        "two cards drawn, one Response spent",
    );
}

/// The slot is narrow on purpose: an ability aimed elsewhere is not a target.
#[test]
fn teferis_response_ignores_an_ability_that_wants_someone_elses_land() {
    let mut game = ready_game();
    let theirs = creature(10_001, cards::MISHRA_S_FACTORY, PlayerId::Two);
    let theirs_id = theirs.card.id;
    game.battlefield.push(theirs);
    let wasteland = creature(10_002, cards::WASTELAND, PlayerId::Two);
    let wasteland_id = wasteland.card.id;
    game.battlefield.push(wasteland);

    game.priority = PlayerId::Two;
    let activation = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::ActivateAbility { source, targets, .. }
                    if *source == wasteland_id
                        && targets.iter().any(|selection| {
                            selection
                                .targets()
                                .contains(&Target::Permanent(theirs_id))
                        })
            )
        })
        .expect("Wasteland can aim at their own Factory");
    game.apply(PlayerId::Two, activation).unwrap();

    let response = card(10_000, cards::TEFERIS_RESPONSE, PlayerId::One);
    let response_id = response.id;
    game.players[PlayerId::One.index()].hand.push(response);
    let pool = &mut game.players[PlayerId::One.index()].mana_pool;
    pool.blue = 1;
    pool.colorless = 1;
    game.priority = PlayerId::One;

    assert!(
        !game.legal_actions(PlayerId::One).iter().any(|action| {
            matches!(action, Action::CastSpell { card, .. } if *card == response_id)
        }),
        "no land of yours is targeted, so the Response has nothing to answer",
    );
}
