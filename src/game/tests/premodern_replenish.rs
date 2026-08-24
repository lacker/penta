//! The cards the Premodern Replenish list needed.

use super::*;

/// Replenish empties the graveyard of enchantments and leaves everything
/// else in it, including an enchantment that belongs to the opponent.
#[test]
fn replenish_returns_every_enchantment_you_own() {
    let mut game = ready_game();
    game.players[PlayerId::One.index()].graveyard.push(card(
        10_010,
        cards::ENGINEERED_PLAGUE,
        PlayerId::One,
    ));
    game.players[PlayerId::One.index()].graveyard.push(card(
        10_011,
        cards::SEAL_OF_CLEANSING,
        PlayerId::One,
    ));
    game.players[PlayerId::One.index()].graveyard.push(card(
        10_012,
        cards::LIGHTNING_BOLT,
        PlayerId::One,
    ));
    game.players[PlayerId::Two.index()].graveyard.push(card(
        10_013,
        cards::ENGINEERED_PLAGUE,
        PlayerId::Two,
    ));

    let replenish = card(10_000, cards::REPLENISH, PlayerId::One);
    let replenish_id = replenish.id;
    game.players[PlayerId::One.index()].hand.push(replenish);
    let pool = &mut game.players[PlayerId::One.index()].mana_pool;
    pool.white = 1;
    pool.colorless = 3;
    game.priority = PlayerId::One;
    game.apply(
        PlayerId::One,
        cast_action(replenish_id, Vec::new(), Vec::new(), 0),
    )
    .expect("four mana casts it");
    drain_pending(&mut game);

    assert_eq!(
        game.battlefield.len(),
        2,
        "both of your enchantments came back",
    );
    assert_eq!(
        game.players[PlayerId::One.index()].graveyard.len(),
        2,
        "the Bolt stayed, and the Replenish joined it",
    );
    assert_eq!(
        game.players[PlayerId::Two.index()].graveyard.len(),
        1,
        "the opponent's enchantment is not yours to return",
    );
}

/// Frantic Search draws, discards, and hands the mana back: three lands
/// untap, which is what makes it free.
#[test]
fn frantic_search_untaps_three_of_the_lands_that_paid_for_it() {
    let mut game = ready_game();
    for index in 0..4 {
        let mut land = creature(10_010 + index, cards::ISLAND, PlayerId::One);
        land.tapped = true;
        game.battlefield.push(land);
    }
    for index in 0..3 {
        game.players[PlayerId::One.index()].library.push(card(
            10_030 + index,
            cards::COUNTERSPELL,
            PlayerId::One,
        ));
    }

    let search = card(10_000, cards::FRANTIC_SEARCH, PlayerId::One);
    let search_id = search.id;
    game.players[PlayerId::One.index()].hand.push(search);
    let pool = &mut game.players[PlayerId::One.index()].mana_pool;
    pool.blue = 1;
    pool.colorless = 2;
    game.priority = PlayerId::One;
    game.apply(
        PlayerId::One,
        cast_action(search_id, Vec::new(), Vec::new(), 0),
    )
    .expect("three mana casts it");

    // Two cards drawn and two discarded -- a hand of exactly two has no
    // choice to make -- and then the lands are chosen.
    pass_until_decision(&mut game);
    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("which lands to untap");
    let options = decision
        .options
        .iter()
        .take(3)
        .map(|option| option.id)
        .collect();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options,
        },
    )
    .expect("three is the printed maximum");
    drain_pending(&mut game);

    let untapped = game
        .battlefield
        .iter()
        .filter(|permanent| !permanent.tapped)
        .count();
    assert_eq!(untapped, 3, "three of the four Islands came back up");
}

/// Attunement pays with itself and gets itself back: the enchantment leaves
/// the battlefield for its owner's hand, and the graveyard takes the four.
#[test]
fn attunement_returns_itself_and_fills_the_graveyard() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::ATTUNEMENT, PlayerId::One));
    for index in 0..3 {
        game.players[PlayerId::One.index()].hand.push(card(
            10_010 + index,
            cards::COUNTERSPELL,
            PlayerId::One,
        ));
    }
    for index in 0..5 {
        game.players[PlayerId::One.index()].library.push(card(
            10_020 + index,
            cards::ISLAND,
            PlayerId::One,
        ));
    }
    game.priority = PlayerId::One;

    let source = game.battlefield[0].card.id;
    let activate = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source: id, .. } if *id == source),
        )
        .expect("the cost is the enchantment itself, so it is always payable");
    game.apply(PlayerId::One, activate).unwrap();
    pass_until_decision(&mut game);
    // Three drawn onto a hand of three, then four of those six discarded.
    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("which four cards go");
    // The Attunement is in hand by now and could be discarded like anything
    // else; keeping it is what shows it came back.
    let options: Vec<u32> = decision
        .options
        .iter()
        .filter(|option| option.label != "Attunement")
        .take(4)
        .map(|option| option.id)
        .collect();
    assert_eq!(options.len(), 4, "four cards other than the Attunement");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options,
        },
    )
    .expect("four is what the card asks for");
    drain_pending(&mut game);

    let player = &game.players[PlayerId::One.index()];
    assert_eq!(player.graveyard.len(), 4, "four cards fed the graveyard");
    assert!(
        game.battlefield.is_empty(),
        "the enchantment left the battlefield to pay",
    );
    assert!(
        player
            .hand
            .iter()
            .any(|card| card.definition == cards::ATTUNEMENT),
        "and it is back in hand to be cast again",
    );
}

/// Opalescence stands the other enchantments up at the size of their own
/// costs, leaves itself and any Aura alone, and stops when it leaves.
#[test]
fn opalescence_animates_each_other_non_aura_enchantment() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::OPALESCENCE, PlayerId::One));
    // Engineered Plague costs {2}{B}, so three.
    game.battlefield
        .push(creature(10_001, cards::ENGINEERED_PLAGUE, PlayerId::One));
    // An Aura is left out of it, and so is a plain artifact.
    game.battlefield
        .push(creature(10_002, cards::BLACK_VISE, PlayerId::One));

    let stats = |game: &Game, definition| {
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.definition == definition)
            .and_then(|permanent| game.creature_stats(permanent))
            .map(|stats| (stats.power, stats.toughness))
    };

    assert_eq!(
        stats(&game, cards::ENGINEERED_PLAGUE),
        Some((3, 3)),
        "a three-mana enchantment is a 3/3",
    );
    assert_eq!(
        stats(&game, cards::OPALESCENCE),
        None,
        "\"each other\" leaves the Opalescence itself out",
    );
    assert_eq!(
        stats(&game, cards::BLACK_VISE),
        None,
        "and an artifact is not an enchantment",
    );

    // The animation is the Opalescence's, so it ends with the Opalescence.
    let opalescence = game.battlefield[0].card.id;
    game.destroy_permanent(opalescence);
    drain_pending(&mut game);
    assert_eq!(
        stats(&game, cards::ENGINEERED_PLAGUE),
        None,
        "the Plague is an enchantment again and nothing more",
    );
}

/// Parallax Wave spends its own fading to answer creatures, and everything
/// it took comes back the moment it goes.
#[test]
fn parallax_wave_exiles_with_its_fade_counters_and_gives_them_back() {
    let mut game = ready_game();
    let wave = card(10_000, cards::PARALLAX_WAVE, PlayerId::One);
    let wave_id = wave.id;
    game.players[PlayerId::One.index()].hand.push(wave);
    let pool = &mut game.players[PlayerId::One.index()].mana_pool;
    pool.white = 2;
    pool.colorless = 2;
    game.battlefield
        .push(creature(10_010, cards::GRIZZLY_BEARS, PlayerId::Two));
    game.priority = PlayerId::One;
    game.apply(
        PlayerId::One,
        cast_action(wave_id, Vec::new(), Vec::new(), 0),
    )
    .expect("four mana casts it");
    drain_pending(&mut game);

    let wave_id = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::PARALLAX_WAVE)
        .expect("it resolved")
        .card
        .id;
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == wave_id)
            .map(|permanent| permanent.counters(CounterKind::named("fade"))),
        Some(5),
        "fading 5 means five counters on arrival",
    );

    // Spend one to exile the Bears.
    let bears = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::GRIZZLY_BEARS)
        .expect("the opponent's creature is there")
        .card
        .id;
    let activate = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == wave_id),
        )
        .expect("a fade counter pays for it");
    game.apply(PlayerId::One, activate).unwrap();
    pass_until_decision(&mut game);
    drain_pending(&mut game);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != bears),
        "the creature is exiled",
    );
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == wave_id)
            .map(|permanent| permanent.counters(CounterKind::named("fade"))),
        Some(4),
        "and the counter is spent",
    );

    // The Wave leaving is what gives it back.
    game.destroy_permanent(wave_id);
    drain_pending(&mut game);
    pass_until_decision(&mut game);
    drain_pending(&mut game);
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::GRIZZLY_BEARS),
        "everything it exiled comes back when it goes",
    );
}

/// Fading is a countdown: the upkeep that cannot pay a counter is the one
/// that ends the permanent, so fading N lasts N of its controller's turns.
#[test]
fn a_faded_parallax_wave_sacrifices_itself() {
    let survives_upkeep = |counters: u16| {
        let mut game = ready_game();
        let mut wave = creature(10_000, cards::PARALLAX_WAVE, PlayerId::One);
        wave.set_counters(CounterKind::named("fade"), counters);
        game.battlefield.push(wave);
        let wave_id = game.battlefield[0].card.id;

        game.turn += 1;
        game.step = Step::Upkeep;
        game.handle_upkeep_triggers();
        pass_until_decision(&mut game);
        drain_pending(&mut game);
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == wave_id)
            .map(|permanent| permanent.counters(CounterKind::named("fade")))
    };

    assert_eq!(
        survives_upkeep(1),
        Some(0),
        "the last counter is spent, and the Wave stays for this turn",
    );
    assert_eq!(
        survives_upkeep(0),
        None,
        "the upkeep it cannot pay is the one it goes on",
    );
}

/// Decree of Silence answers each opponent spell and marks itself for it,
/// and the third mark is the one it goes on.
#[test]
fn decree_of_silence_counters_until_its_third_depletion_counter() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::DECREE_OF_SILENCE, PlayerId::One));
    let decree = game.battlefield[0].card.id;

    let cast_a_bolt = |game: &mut Game, id: u32| {
        let bolt = card(id, cards::LIGHTNING_BOLT, PlayerId::Two);
        let bolt_id = bolt.id;
        game.players[PlayerId::Two.index()].hand.push(bolt);
        game.players[PlayerId::Two.index()].mana_pool.red = 1;
        game.priority = PlayerId::Two;
        game.apply(
            PlayerId::Two,
            cast_action(bolt_id, vec![Target::Player(PlayerId::One)], Vec::new(), 0),
        )
        .expect("one red casts it");
        pass_until_decision(game);
        drain_pending(game);
    };

    for (index, expected) in [(0_u32, 1_u16), (1, 2)] {
        cast_a_bolt(&mut game, 10_010 + index);
        assert_eq!(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == decree)
                .map(|permanent| permanent.counters(CounterKind::named("depletion"))),
            Some(expected),
            "the spell is answered and the enchantment marked",
        );
        assert_eq!(
            game.players[PlayerId::One.index()].life,
            20,
            "no damage got through"
        );
    }

    // The third counter is the one it goes on.
    cast_a_bolt(&mut game, 10_012);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != decree),
        "three depletion counters is as many as it gets",
    );
    assert_eq!(
        game.players[PlayerId::One.index()].life,
        20,
        "and the third spell was still countered",
    );
}

/// Intuition hands the opponent the choice: one of the three the search
/// found goes to hand, and the other two go to the graveyard.
#[test]
fn intuition_gives_the_opponent_the_pick_of_three() {
    let mut game = ready_game();
    game.players[PlayerId::One.index()].library.clear();
    for index in 0..3 {
        game.players[PlayerId::One.index()].library.push(card(
            10_010 + index,
            cards::COUNTERSPELL,
            PlayerId::One,
        ));
    }
    game.players[PlayerId::One.index()]
        .library
        .push(card(10_020, cards::ISLAND, PlayerId::One));

    let intuition = card(10_000, cards::INTUITION, PlayerId::One);
    let intuition_id = intuition.id;
    game.players[PlayerId::One.index()].hand.push(intuition);
    let pool = &mut game.players[PlayerId::One.index()].mana_pool;
    pool.blue = 1;
    pool.colorless = 2;
    game.priority = PlayerId::One;
    game.apply(
        PlayerId::One,
        cast_action(
            intuition_id,
            vec![Target::Player(PlayerId::Two)],
            Vec::new(),
            0,
        ),
    )
    .expect("three mana casts it");

    // The caster searches out three, and then the opponent picks.
    pass_until_decision(&mut game);
    let search = game
        .observe(PlayerId::One)
        .decision
        .expect("the caster searches");
    let three = search
        .options
        .iter()
        .filter(|option| option.label == "Counterspell")
        .take(3)
        .map(|option| option.id)
        .collect();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: search.id,
            options: three,
        },
    )
    .expect("three is what the card asks for");
    pass_until_decision(&mut game);

    let pick = game
        .observe(PlayerId::Two)
        .decision
        .expect("the opponent chooses one of them");
    assert_eq!(
        pick.options.len(),
        3,
        "out of the three found, not out of the library",
    );
    game.apply(
        PlayerId::Two,
        Action::ChooseDecision {
            decision: pick.id,
            options: vec![pick.options[0].id],
        },
    )
    .expect("one of the three");
    drain_pending(&mut game);

    let player = &game.players[PlayerId::One.index()];
    assert_eq!(
        player
            .hand
            .iter()
            .filter(|card| card.definition == cards::COUNTERSPELL)
            .count(),
        1,
        "the one the opponent gave up",
    );
    assert_eq!(
        player
            .graveyard
            .iter()
            .filter(|card| card.definition == cards::COUNTERSPELL)
            .count(),
        2,
        "and the rest went to the graveyard",
    );
    assert_eq!(player.library.len(), 1, "the Island stayed where it was");
}

/// Abeyance shuts the target out of instants, sorceries, and activations for
/// the turn, and leaves their lands and creatures alone.
#[test]
fn abeyance_locks_a_player_out_for_the_turn() {
    let mut game = ready_game();
    // Something to activate, and something to cast.
    game.battlefield
        .push(creature(10_010, cards::CURSED_SCROLL, PlayerId::Two));
    game.players[PlayerId::Two.index()].hand.push(card(
        10_011,
        cards::LIGHTNING_BOLT,
        PlayerId::Two,
    ));
    // A land, to show which activations survive: mana abilities are the
    // exemption the card names.
    game.battlefield
        .push(creature(10_012, cards::ISLAND, PlayerId::Two));
    let pool = &mut game.players[PlayerId::Two.index()].mana_pool;
    pool.red = 1;
    pool.green = 1;
    pool.colorless = 4;

    let abeyance = card(10_000, cards::ABEYANCE, PlayerId::One);
    let abeyance_id = abeyance.id;
    game.players[PlayerId::One.index()].hand.push(abeyance);
    let pool = &mut game.players[PlayerId::One.index()].mana_pool;
    pool.white = 1;
    pool.colorless = 1;
    // Legal actions are enumerated for whoever holds priority, so each side
    // is asked while it does.
    game.priority = PlayerId::Two;
    let before = game.legal_actions(PlayerId::Two);
    assert!(
        before
            .iter()
            .any(|action| matches!(action, Action::ActivateAbility { .. })),
        "the Scroll is activatable to begin with",
    );

    game.priority = PlayerId::One;
    game.apply(
        PlayerId::One,
        cast_action(
            abeyance_id,
            vec![Target::Player(PlayerId::Two)],
            Vec::new(),
            0,
        ),
    )
    .expect("two mana casts it");
    drain_pending(&mut game);

    game.priority = PlayerId::Two;
    let after = game.legal_actions(PlayerId::Two);
    assert!(
        !after
            .iter()
            .any(|action| matches!(action, Action::ActivateAbility { .. })),
        "no activation but a mana ability",
    );
    assert!(
        !after
            .iter()
            .any(|action| matches!(action, Action::CastSpell { .. })),
        "and the Bolt in hand is shut out too",
    );
    assert!(
        after
            .iter()
            .any(|action| matches!(action, Action::ActivateManaAbility { .. })),
        "but a mana ability is the exemption the card names",
    );
    assert_eq!(
        game.players[PlayerId::One.index()].hand.len(),
        1,
        "and it replaced itself",
    );
}

/// Skycloud Expanse makes two unlike mana from one activation, and the
/// planner will spend it on a cost that needs both.
#[test]
fn skycloud_expanse_makes_one_of_each_colour() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::SKYCLOUD_EXPANSE, PlayerId::One));
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;
    game.priority = PlayerId::One;

    let expanse = game.battlefield[0].card.id;
    let activate = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateManaAbility { source, .. } if *source == expanse)
        })
        .expect("one generic pays for it");
    game.apply(PlayerId::One, activate).unwrap();
    drain_pending(&mut game);

    let pool = game.players[PlayerId::One.index()].mana_pool;
    assert_eq!(pool.white, 1, "one white");
    assert_eq!(pool.blue, 1, "and one blue, from the same activation");
    assert_eq!(pool.colorless, 0, "the generic went to pay for it");
}

/// And the planner can find both halves when a cost needs them, which is
/// the whole reason the land is played.
#[test]
fn skycloud_expanse_pays_a_two_colour_cost() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::SKYCLOUD_EXPANSE, PlayerId::One));
    let expanse = game.battlefield[0].card.id;
    // The Island pays the Expanse's own {1}, and the Expanse supplies both
    // halves of a {W}{U} cost -- a plan the planner has to find in two steps.
    game.battlefield
        .push(creature(10_010, cards::ISLAND, PlayerId::One));
    let mage = card(10_020, cards::MEDDLING_MAGE, PlayerId::One);
    let mage_id = mage.id;
    game.players[PlayerId::One.index()].hand.push(mage);
    game.priority = PlayerId::One;

    // The Expanse's own {1} has to be on the table before its ability is
    // offered: mana-ability enumeration reads the pool as it is rather than
    // planning a chain of activations. So the Island goes first.
    let island = game.battlefield[1].card.id;
    let tap_island = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateManaAbility { source, .. } if *source == island)
        })
        .expect("the Island taps for blue");
    game.apply(PlayerId::One, tap_island).unwrap();
    let tap_expanse = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateManaAbility { source, .. } if *source == expanse)
        })
        .expect("with one blue on the table the Expanse is affordable");
    game.apply(PlayerId::One, tap_expanse).unwrap();
    drain_pending(&mut game);

    assert!(
        game.legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == mage_id)),
        "and its two colours cast a spell that needs both",
    );
}
