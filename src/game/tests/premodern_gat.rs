//! The cards Daniel Sondike's GAT list needed.
//!
//! Each of these turns on a piece of engine vocabulary the format had not
//! asked for before: a scalar the source chose on entry and then reads back,
//! a land that spends itself, and a replacement paid from hand.

use super::*;

/// Casts `definition` from `player`'s hand with the mana already available,
/// letting the spell resolve.
fn cast_permanent(game: &mut Game, id: u32, definition: CardDefinitionId, player: PlayerId) {
    let spell = card(id, definition, player);
    let spell_id = spell.id;
    game.players[player.index()].hand.push(spell);
    let pool = &mut game.players[player.index()].mana_pool;
    pool.black = 3;
    pool.colorless = 3;
    game.priority = player;
    game.apply(player, cast_action(spell_id, Vec::new(), Vec::new(), 0))
        .expect("the spell is cast");
    pass_priority_pair(game);
}

/// Answers the as-enters scalar choice with `label`.
fn choose_scalar(game: &mut Game, player: PlayerId, label: &str) {
    let decision = game
        .observe(player)
        .decision
        .expect("the permanent asks for its entry choice");
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
    .expect("the choice is legal");
    drain_pending(game);
}

fn stats(game: &Game, id: GameObjectId) -> Option<(i16, i16)> {
    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)?;
    Some((
        game.power(permanent).expect("a creature has power"),
        game.toughness(permanent).expect("a creature has toughness"),
    ))
}

#[test]
fn engineered_plague_shrinks_only_the_type_it_named() {
    let mut game = ready_game();
    let ringleader = creature(10_001, cards::GOBLIN_RINGLEADER, PlayerId::Two);
    let ringleader_id = ringleader.card.id;
    game.battlefield.push(ringleader);
    let dryad = creature(10_002, cards::QUIRION_DRYAD, PlayerId::Two);
    let dryad_id = dryad.card.id;
    game.battlefield.push(dryad);

    cast_permanent(&mut game, 10_000, cards::ENGINEERED_PLAGUE, PlayerId::One);
    choose_scalar(&mut game, PlayerId::One, "Goblin");

    assert_eq!(
        stats(&game, ringleader_id),
        Some((1, 1)),
        "the Goblin shrank"
    );
    assert_eq!(
        stats(&game, dryad_id),
        Some((1, 1)),
        "the Dryad is untouched, and its own 1/1 is not the Plague's doing",
    );
}

/// The half that decides games: a one-toughness creature of the named type is
/// not shrunk but killed, by state-based actions rather than by the Plague.
#[test]
fn engineered_plague_kills_the_one_toughness_creatures_it_names() {
    let mut game = ready_game();
    let fanatic = creature(10_001, cards::MOGG_FANATIC, PlayerId::Two);
    let fanatic_id = fanatic.card.id;
    game.battlefield.push(fanatic);

    cast_permanent(&mut game, 10_000, cards::ENGINEERED_PLAGUE, PlayerId::One);
    choose_scalar(&mut game, PlayerId::One, "Goblin");

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == fanatic_id),
        "a 1/1 Goblin is a 0/0 and dies",
    );
}

/// A Plague that never got its choice -- and one that named something else --
/// leaves everything alone. The predicate reads a specific chosen type, not
/// "some type was chosen".
#[test]
fn a_plague_naming_another_type_shrinks_nothing() {
    let mut game = ready_game();
    let ringleader = creature(10_001, cards::GOBLIN_RINGLEADER, PlayerId::Two);
    let ringleader_id = ringleader.card.id;
    game.battlefield.push(ringleader);

    cast_permanent(&mut game, 10_000, cards::ENGINEERED_PLAGUE, PlayerId::One);
    choose_scalar(&mut game, PlayerId::One, "Dryad");

    assert_eq!(stats(&game, ringleader_id), Some((2, 2)));
}

/// Plays `definition` as a land from `player`'s hand and returns its id.
fn play_land(
    game: &mut Game,
    id: u32,
    definition: CardDefinitionId,
    player: PlayerId,
) -> GameObjectId {
    let land = card(id, definition, player);
    let land_id = land.id;
    game.players[player.index()].hand.push(land);
    game.priority = player;
    game.apply(
        player,
        Action::PlayLand {
            card: land_id,
            option: PlayOptionId::DEFAULT,
        },
    )
    .expect("the land is played");
    drain_pending(game);
    // A permanent entering the battlefield is a new object, so the card's id
    // in hand is not the one to follow afterwards.
    let _ = land_id;
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == definition)
        .expect("the land entered the battlefield")
        .card
        .id
}

fn mining_counters(game: &Game, id: GameObjectId) -> Option<u16> {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .map(|permanent| permanent.counters(CounterKind::named("mining")))
}

/// Taps the Mine for one mana of `color`, then untaps it so the next
/// activation is about the counters rather than about the tap.
fn mine_for(game: &mut Game, source: GameObjectId, color: ManaColor) {
    game.apply(
        PlayerId::One,
        Action::ActivateManaAbility {
            source,
            ability: mana_ability_for(game, source, color),
            color,
            counters_removed: None,
            cost_object: None,
            combination: None,
            triggered_mana: None,
        },
    )
    .expect("the Mine still has a counter to spend");
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == source)
    {
        permanent.tapped = false;
    }
}

#[test]
fn gemstone_mine_enters_with_three_counters_and_spends_itself_dry() {
    let mut game = ready_game();
    let mine = play_land(&mut game, 10_000, cards::GEMSTONE_MINE, PlayerId::One);
    assert_eq!(mining_counters(&game, mine), Some(3));

    // Every colour, because that is the whole reason the deck plays it.
    mine_for(&mut game, mine, ManaColor::Blue);
    assert_eq!(mining_counters(&game, mine), Some(2));
    mine_for(&mut game, mine, ManaColor::White);
    assert_eq!(mining_counters(&game, mine), Some(1));
    assert_eq!(game.players[PlayerId::One.index()].mana_pool.blue, 1);
    assert_eq!(game.players[PlayerId::One.index()].mana_pool.white, 1);

    mine_for(&mut game, mine, ManaColor::Green);
    assert_eq!(
        mining_counters(&game, mine),
        None,
        "the last counter takes the land with it",
    );
    assert!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::GEMSTONE_MINE),
        "and it goes to the graveyard, not into exile or nowhere",
    );
    assert_eq!(
        game.players[PlayerId::One.index()].mana_pool.green,
        1,
        "the mana it was sacrificed for is still in the pool",
    );
}

/// The Mine is offered only while it can pay its own cost.
#[test]
fn a_spent_mine_offers_nothing() {
    let mut game = ready_game();
    let mine = play_land(&mut game, 10_000, cards::GEMSTONE_MINE, PlayerId::One);
    for color in [ManaColor::Blue, ManaColor::White, ManaColor::Green] {
        mine_for(&mut game, mine, color);
    }

    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(
                action,
                Action::ActivateManaAbility { source, .. } if *source == mine
            )),
        "a land in the graveyard is not a mana source",
    );
}

/// Casts Meddling Mage and has it name `named`, returning the Mage.
fn meddling_mage_naming(game: &mut Game, named: &str) -> GameObjectId {
    let mage = card(10_000, cards::MEDDLING_MAGE, PlayerId::One);
    let mage_id = mage.id;
    game.players[PlayerId::One.index()].hand.push(mage);
    let pool = &mut game.players[PlayerId::One.index()].mana_pool;
    pool.white = 1;
    pool.blue = 1;
    game.priority = PlayerId::One;
    game.apply(
        PlayerId::One,
        cast_action(mage_id, Vec::new(), Vec::new(), 0),
    )
    .expect("the Mage is cast");
    pass_priority_pair(game);
    choose_scalar(game, PlayerId::One, named);
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::MEDDLING_MAGE)
        .expect("the Mage entered once its name was chosen")
        .card
        .id
}

/// Whether `player` is offered a cast of the card object `card`.
fn can_cast(game: &Game, player: PlayerId, card: GameObjectId) -> bool {
    game.legal_actions(player).iter().any(
        |action| matches!(action, Action::CastSpell { card: candidate, .. } if *candidate == card),
    )
}

#[test]
fn meddling_mage_locks_out_the_name_it_chose() {
    let mut game = ready_game();
    let bolt = card(10_001, cards::LIGHTNING_BOLT, PlayerId::Two);
    let bolt_id = bolt.id;
    game.players[PlayerId::Two.index()].hand.push(bolt);
    let opt = card(10_002, cards::OPT, PlayerId::Two);
    let opt_id = opt.id;
    game.players[PlayerId::Two.index()].hand.push(opt);
    let pool = &mut game.players[PlayerId::Two.index()].mana_pool;
    pool.red = 1;
    pool.blue = 1;

    meddling_mage_naming(&mut game, "Lightning Bolt");
    game.priority = PlayerId::Two;

    assert!(
        !can_cast(&game, PlayerId::Two, bolt_id),
        "the named spell cannot be cast",
    );
    assert!(
        can_cast(&game, PlayerId::Two, opt_id),
        "everything else still can",
    );
}

/// The lock is symmetric, and it dies with the Mage.
#[test]
fn the_lock_binds_its_own_controller_and_leaves_with_the_mage() {
    let mut game = ready_game();
    let bolt = card(10_001, cards::LIGHTNING_BOLT, PlayerId::One);
    let bolt_id = bolt.id;
    game.players[PlayerId::One.index()].hand.push(bolt);

    let mage = meddling_mage_naming(&mut game, "Lightning Bolt");
    game.players[PlayerId::One.index()].mana_pool.red = 1;
    assert!(
        !can_cast(&game, PlayerId::One, bolt_id),
        "the Mage does not care who was going to cast it",
    );

    game.move_permanents_to_graveyard(&[mage]);
    drain_pending(&mut game);
    assert!(
        can_cast(&game, PlayerId::One, bolt_id),
        "and the lock leaves with it",
    );
}

/// "Choose a nonland card name" is a restriction, not flavor.
#[test]
fn the_mage_is_not_offered_a_land_to_name() {
    let mut game = ready_game();
    let mage = card(10_000, cards::MEDDLING_MAGE, PlayerId::One);
    let mage_id = mage.id;
    game.players[PlayerId::One.index()].hand.push(mage);
    let pool = &mut game.players[PlayerId::One.index()].mana_pool;
    pool.white = 1;
    pool.blue = 1;
    game.apply(
        PlayerId::One,
        cast_action(mage_id, Vec::new(), Vec::new(), 0),
    )
    .expect("the Mage is cast");
    pass_priority_pair(&mut game);

    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the Mage asks for a name");
    let labels: Vec<&str> = decision
        .options
        .iter()
        .map(|option| option.label.as_str())
        .collect();
    assert!(labels.contains(&"Lightning Bolt"), "a spell is nameable");
    assert!(
        !labels.contains(&"Island") && !labels.contains(&"Gemstone Mine"),
        "no land is, basic or otherwise",
    );
}

/// Casts Portent at `target` and returns the three cards it is looking at, in
/// the order the decision offers them.
fn portent_at(game: &mut Game, target: PlayerId) -> Vec<GameObjectId> {
    let spell = card(10_000, cards::PORTENT, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.blue = 1;
    game.priority = PlayerId::One;
    game.apply(
        PlayerId::One,
        cast_action(spell_id, vec![Target::Player(target)], Vec::new(), 0),
    )
    .expect("Portent is cast");
    pass_priority_pair(game);
    game.observe(PlayerId::One)
        .decision
        .expect("Portent asks for an arrangement")
        .options
        .iter()
        .filter_map(|option| option.card.map(|(card, _)| card))
        .collect()
}

/// Names the looked-at cards in the given top-first order.
fn arrange(game: &mut Game, order: &[GameObjectId]) {
    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("Portent asks for an arrangement");
    let options: Vec<u32> = order
        .iter()
        .map(|wanted| {
            decision
                .options
                .iter()
                .find(|option| option.card.is_some_and(|(card, _)| card == *wanted))
                .expect("every looked-at card is offered")
                .id
        })
        .collect();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options,
        },
    )
    .expect("naming all three is a legal arrangement");
    drain_pending(game);
}

#[test]
fn portent_puts_the_three_back_in_the_order_they_were_named() {
    let mut game = ready_game();
    let seen = portent_at(&mut game, PlayerId::One);
    assert_eq!(seen.len(), 3);

    // Reversed, so an implementation that ignores the order fails rather than
    // passing by accident on a library that came back the way it went in.
    let wanted: Vec<GameObjectId> = seen.iter().rev().copied().collect();
    arrange(&mut game, &wanted);

    let top: Vec<GameObjectId> = game.players[PlayerId::One.index()]
        .library
        .iter()
        .rev()
        .take(3)
        .map(|card| card.id)
        .collect();
    assert_eq!(top, wanted, "the arrangement is the order they were named");
}

/// The delayed draw is the rest of the card: it arrives an upkeep later, not
/// on resolution.
#[test]
fn portent_draws_a_card_at_the_next_upkeep() {
    let mut game = ready_game();
    let seen = portent_at(&mut game, PlayerId::One);
    let before = game.players[PlayerId::One.index()].hand.len();
    arrange(&mut game, &seen);
    assert_eq!(
        game.players[PlayerId::One.index()].hand.len(),
        before,
        "nothing is drawn while it resolves",
    );

    // The next upkeep, reached the way the delayed-trigger tests reach one.
    game.turn += 1;
    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    drain_pending(&mut game);
    assert_eq!(
        game.players[PlayerId::One.index()].hand.len(),
        before + 1,
        "the delayed draw arrives at the next upkeep",
    );
}

/// Casts Mox Diamond and returns the entry-payment decision it stops on.
fn cast_mox_diamond(game: &mut Game) -> DecisionObservation {
    let mox = card(10_000, cards::MOX_DIAMOND, PlayerId::One);
    let mox_id = mox.id;
    game.players[PlayerId::One.index()].hand.push(mox);
    game.priority = PlayerId::One;
    game.apply(
        PlayerId::One,
        cast_action(mox_id, Vec::new(), Vec::new(), 0),
    )
    .expect("a free artifact is castable");
    pass_priority_pair(game);
    game.observe(PlayerId::One)
        .decision
        .expect("Mox Diamond asks for its land")
}

fn on_battlefield(game: &Game, definition: CardDefinitionId) -> bool {
    game.battlefield
        .iter()
        .any(|permanent| permanent.card.definition == definition)
}

#[test]
fn mox_diamond_offers_each_land_in_hand_and_nothing_else() {
    let mut game = ready_game();
    game.players[PlayerId::One.index()].hand.clear();
    let island = card(10_001, cards::ISLAND, PlayerId::One);
    let island_id = island.id;
    game.players[PlayerId::One.index()].hand.push(island);
    game.players[PlayerId::One.index()].hand.push(card(
        10_002,
        cards::LIGHTNING_BOLT,
        PlayerId::One,
    ));

    let decision = cast_mox_diamond(&mut game);
    let paying: Vec<GameObjectId> = decision
        .options
        .iter()
        .filter_map(|option| option.card.map(|(card, _)| card))
        .collect();
    assert_eq!(
        paying,
        vec![island_id],
        "the land can pay and the burn spell cannot",
    );
}

#[test]
fn discarding_a_land_keeps_the_mox() {
    let mut game = ready_game();
    game.players[PlayerId::One.index()].hand.clear();
    let island = card(10_001, cards::ISLAND, PlayerId::One);
    let island_id = island.id;
    game.players[PlayerId::One.index()].hand.push(island);

    let decision = cast_mox_diamond(&mut game);
    let pay = decision
        .options
        .iter()
        .find(|option| option.card.is_some_and(|(card, _)| card == island_id))
        .expect("the Island is offered")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![pay],
        },
    )
    .expect("discarding the Island pays");
    drain_pending(&mut game);

    assert!(on_battlefield(&game, cards::MOX_DIAMOND), "the Mox entered");
    assert!(
        game.players[PlayerId::One.index()].hand.is_empty(),
        "and the Island is gone",
    );
    assert!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::ISLAND),
        "discarded to the graveyard",
    );
}

/// Declining sends the Mox itself to the graveyard, which is the half that
/// needs the entry to be replaced rather than undone.
#[test]
fn declining_puts_the_mox_in_the_graveyard() {
    let mut game = ready_game();
    game.players[PlayerId::One.index()].hand.clear();
    game.players[PlayerId::One.index()]
        .hand
        .push(card(10_001, cards::ISLAND, PlayerId::One));

    let decision = cast_mox_diamond(&mut game);
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![0],
        },
    )
    .expect("declining is always available");
    drain_pending(&mut game);

    assert!(
        !on_battlefield(&game, cards::MOX_DIAMOND),
        "an unpaid Mox never enters",
    );
    assert!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::MOX_DIAMOND),
        "it is in its owner's graveyard",
    );
    assert_eq!(
        game.players[PlayerId::One.index()].hand.len(),
        1,
        "and the Island stayed in hand",
    );
}

/// A hand with no land cannot pay at all, so the Mox is not even asked about.
#[test]
fn a_landless_hand_is_never_offered_the_choice() {
    let mut game = ready_game();
    game.players[PlayerId::One.index()].hand.clear();
    game.players[PlayerId::One.index()].hand.push(card(
        10_001,
        cards::LIGHTNING_BOLT,
        PlayerId::One,
    ));

    let mox = card(10_000, cards::MOX_DIAMOND, PlayerId::One);
    let mox_id = mox.id;
    game.players[PlayerId::One.index()].hand.push(mox);
    game.priority = PlayerId::One;
    game.apply(
        PlayerId::One,
        cast_action(mox_id, Vec::new(), Vec::new(), 0),
    )
    .expect("a free artifact is castable");
    pass_priority_pair(&mut game);
    drain_pending(&mut game);

    assert!(
        game.observe(PlayerId::One).decision.is_none(),
        "there is nothing to decide",
    );
    assert!(
        !on_battlefield(&game, cards::MOX_DIAMOND),
        "and the Mox goes straight to the graveyard",
    );
    assert!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::MOX_DIAMOND),
    );
}

/// The half the deck is actually paying for.
#[test]
fn a_mox_that_entered_taps_for_any_color() {
    let mut game = ready_game();
    game.players[PlayerId::One.index()].hand.clear();
    let island = card(10_001, cards::ISLAND, PlayerId::One);
    let island_id = island.id;
    game.players[PlayerId::One.index()].hand.push(island);

    let decision = cast_mox_diamond(&mut game);
    let pay = decision
        .options
        .iter()
        .find(|option| option.card.is_some_and(|(card, _)| card == island_id))
        .expect("the Island is offered")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![pay],
        },
    )
    .expect("discarding the Island pays");
    drain_pending(&mut game);

    let mox = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::MOX_DIAMOND)
        .expect("the Mox entered")
        .card
        .id;
    game.apply(
        PlayerId::One,
        Action::ActivateManaAbility {
            source: mox,
            ability: mana_ability_for(&game, mox, ManaColor::Green),
            color: ManaColor::Green,
            counters_removed: None,
            cost_object: None,
            combination: None,
            triggered_mana: None,
        },
    )
    .expect("the Mox taps for any color");
    assert_eq!(game.players[PlayerId::One.index()].mana_pool.green, 1);
}

/// "If you don't discard a land card, Mox Diamond never enters. It won't
/// trigger abilities that look for something entering." A Mirrorworks
/// watching for artifacts sees the Mox that paid and never sees the one
/// that did not.
#[test]
fn a_declined_mox_never_enters_and_triggers_nothing() {
    for pay in [true, false] {
        let mut game = ready_game();
        game.battlefield.clear();
        game.players[PlayerId::One.index()].hand.clear();
        let island = card(10_001, cards::ISLAND, PlayerId::One);
        let island_id = island.id;
        game.players[PlayerId::One.index()].hand.push(island);
        game.put_onto_battlefield(PlayerId::One, cards::MIRRORWORKS)
            .expect("cataloged");
        drain_pending(&mut game);

        let decision = cast_mox_diamond(&mut game);
        let options = if pay {
            vec![
                decision
                    .options
                    .iter()
                    .find(|option| option.card.is_some_and(|(card, _)| card == island_id))
                    .expect("the Island is offered")
                    .id,
            ]
        } else {
            // Option zero is the standing "decline" the replacement offers.
            vec![0]
        };
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: decision.id,
                options,
            },
        )
        .expect("either answer is allowed");

        // Anything the Mirrorworks has to say arrives now.
        let mut watched = false;
        for _ in 0..8 {
            if let Some(pending) = game
                .pending_decisions
                .first()
                .map(|pending| pending.observation.clone())
            {
                watched |= pending
                    .prompt
                    .starts_with("Whenever another nontoken artifact");
                let options = pending
                    .options
                    .iter()
                    .find(|option| option.label == "Decline")
                    .map(|option| vec![option.id])
                    .unwrap_or_default();
                game.apply(
                    pending.player,
                    Action::ChooseDecision {
                        decision: pending.id,
                        options,
                    },
                )
                .expect("declining is allowed");
                continue;
            }
            if game.stack.is_empty() && game.pending_triggers.is_empty() {
                break;
            }
            let priority = game.priority;
            if game.apply(priority, Action::PassPriority).is_err() {
                break;
            }
        }
        game.check_state_based_actions();

        assert_eq!(
            on_battlefield(&game, cards::MOX_DIAMOND),
            pay,
            "the Mox enters only when a land pays for it",
        );
        assert_eq!(
            watched, pay,
            "and only an entry something watched for is one the Mirrorworks saw",
        );
        if !pay {
            assert!(
                game.players[PlayerId::One.index()]
                    .graveyard
                    .iter()
                    .any(|card| card.definition == cards::MOX_DIAMOND),
                "the Mox went to the graveyard on the way",
            );
            assert!(
                game.players[PlayerId::One.index()]
                    .hand
                    .iter()
                    .any(|card| card.definition == cards::ISLAND),
                "and the land it did not discard is still in hand",
            );
        }
    }
}
