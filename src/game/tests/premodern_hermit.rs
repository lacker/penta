//! The cards the Premodern Angry Hermit list needed.

use super::*;

/// The Druid digs to the first basic land and buries what it passed. A
/// library holding none is emptied instead, which is the deck's whole plan.
#[test]
fn hermit_druid_takes_the_basic_and_buries_the_rest() {
    let dug = |basics: bool| {
        let mut game = ready_game();
        let druid = creature(10_000, cards::HERMIT_DRUID, PlayerId::One);
        game.battlefield.push(druid);
        game.players[PlayerId::One.index()].library.clear();
        // Bottom to top: two spells, then a basic beneath nothing else when
        // the library has one at all.
        if basics {
            game.players[PlayerId::One.index()].library.push(card(
                10_010,
                cards::SWAMP,
                PlayerId::One,
            ));
        }
        for index in 0..2 {
            game.players[PlayerId::One.index()].library.push(card(
                10_020 + index,
                cards::LIGHTNING_BOLT,
                PlayerId::One,
            ));
        }
        game.players[PlayerId::One.index()].mana_pool.green = 1;
        game.priority = PlayerId::One;

        let druid_id = game.battlefield[0].card.id;
        let activate = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| {
                matches!(action, Action::ActivateAbility { source, .. } if *source == druid_id)
            })
            .expect("one green and an untapped Druid");
        game.apply(PlayerId::One, activate).unwrap();
        drain_pending(&mut game);
        let player = &game.players[PlayerId::One.index()];
        (
            player.hand.len(),
            player.graveyard.len(),
            player.library.len(),
        )
    };

    let (hand, graveyard, library) = dug(true);
    assert_eq!(hand, 1, "the Swamp it found");
    assert_eq!(graveyard, 2, "the two Bolts above it");
    assert_eq!(library, 0, "the dig went all the way down");

    let (hand, graveyard, library) = dug(false);
    assert_eq!(hand, 0, "nothing found, so nothing taken");
    assert_eq!(graveyard, 2, "the library empties into the graveyard");
    assert_eq!(library, 0, "and nothing is left");
}

/// Stifle answers an ability and cannot be pointed at a spell.
#[test]
fn stifle_counters_an_ability_but_not_a_spell() {
    let mut game = ready_game();
    let stifle = card(10_000, cards::STIFLE, PlayerId::Two);
    let stifle_id = stifle.id;
    game.players[PlayerId::Two.index()].hand.push(stifle);
    game.players[PlayerId::Two.index()].mana_pool.blue = 1;

    // A spell on the stack alone gives Stifle nothing to name.
    let bolt = card(10_001, cards::LIGHTNING_BOLT, PlayerId::One);
    let bolt_id = bolt.id;
    game.players[PlayerId::One.index()].hand.push(bolt);
    game.players[PlayerId::One.index()].mana_pool.red = 1;
    game.priority = PlayerId::One;
    game.apply(
        PlayerId::One,
        cast_action(bolt_id, vec![Target::Player(PlayerId::Two)], Vec::new(), 0),
    )
    .expect("the Bolt is cast");
    assert!(
        !game
            .legal_actions(PlayerId::Two)
            .iter()
            .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == stifle_id)),
        "a spell is not an activated or triggered ability",
    );
}

/// And Stifle answers the ability itself: the Druid taps, and the dig never
/// happens.
#[test]
fn stifle_counters_the_druids_dig() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::HERMIT_DRUID, PlayerId::One));
    game.players[PlayerId::One.index()].library.clear();
    for index in 0..3 {
        game.players[PlayerId::One.index()].library.push(card(
            10_020 + index,
            cards::LIGHTNING_BOLT,
            PlayerId::One,
        ));
    }
    game.players[PlayerId::One.index()].mana_pool.green = 1;

    let stifle = card(10_001, cards::STIFLE, PlayerId::Two);
    let stifle_id = stifle.id;
    game.players[PlayerId::Two.index()].hand.push(stifle);
    game.players[PlayerId::Two.index()].mana_pool.blue = 1;

    game.priority = PlayerId::One;
    let druid_id = game.battlefield[0].card.id;
    let activate = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == druid_id))
        .expect("one green and an untapped Druid");
    game.apply(PlayerId::One, activate).unwrap();

    let ability = game.stack.last().expect("the dig is on the stack").id;
    game.priority = PlayerId::Two;
    game.apply(
        PlayerId::Two,
        cast_action(stifle_id, vec![Target::Spell(ability)], Vec::new(), 0),
    )
    .expect("the ability can be named");
    drain_pending(&mut game);

    let player = &game.players[PlayerId::One.index()];
    assert_eq!(player.library.len(), 3, "the dig never happened");
    assert_eq!(player.hand.len(), 0, "and nothing was found");
}

/// Shallow Grave takes the newest creature card, not the oldest, and the
/// creature it returns leaves again at the end of the turn.
#[test]
fn shallow_grave_returns_the_top_creature_and_exiles_it_at_end_of_turn() {
    let mut game = ready_game();
    // Oldest to newest: a Bolt between two creatures, so "the top creature
    // card" is the second creature rather than the last card.
    game.players[PlayerId::One.index()].graveyard.push(card(
        10_010,
        cards::GOBLIN_LACKEY,
        PlayerId::One,
    ));
    game.players[PlayerId::One.index()].graveyard.push(card(
        10_011,
        cards::PSYCHATOG,
        PlayerId::One,
    ));
    game.players[PlayerId::One.index()].graveyard.push(card(
        10_012,
        cards::LIGHTNING_BOLT,
        PlayerId::One,
    ));

    let grave = card(10_000, cards::SHALLOW_GRAVE, PlayerId::One);
    let grave_id = grave.id;
    game.players[PlayerId::One.index()].hand.push(grave);
    let pool = &mut game.players[PlayerId::One.index()].mana_pool;
    pool.black = 1;
    pool.colorless = 1;
    game.priority = PlayerId::One;
    game.apply(
        PlayerId::One,
        cast_action(grave_id, Vec::new(), Vec::new(), 0),
    )
    .expect("two mana casts it");
    drain_pending(&mut game);

    let returned = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::PSYCHATOG)
        .expect("the newest creature card came back");
    assert!(
        game.permanent_has_executable_keyword(returned, KeywordAbility::Haste),
        "it can attack the turn it arrives",
    );
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.definition != cards::GOBLIN_LACKEY),
        "the older creature stayed in the graveyard",
    );

    // Reaching the end step is what fires the delayed clause; setting the
    // field would skip the step beginning.
    for _ in 0..8 {
        if game.step == Step::End {
            break;
        }
        game.advance_step();
    }
    assert_eq!(game.step, Step::End, "the turn reached its end step");
    // Driving steps directly skips the procedure that puts captured triggers
    // on the stack, so run it before letting the stack resolve.
    game.finish_rules_procedure();
    pass_until_decision(&mut game);
    drain_pending(&mut game);
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.definition != cards::PSYCHATOG),
        "and it is exiled at the beginning of the end step",
    );
    assert_eq!(
        game.players[PlayerId::One.index()].exile.len(),
        1,
        "exiled rather than buried",
    );
}

/// Reflecting Pool reads the lands you control, not the opponent's, and a
/// type is a type: colourless counts where Fellwar Stone's colours do not.
#[test]
fn reflecting_pool_borrows_from_your_own_lands() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_000, cards::REFLECTING_POOL, PlayerId::One));
    game.battlefield
        .push(creature(10_001, cards::FOREST, PlayerId::One));
    // An opponent's Island is not one of yours.
    game.battlefield
        .push(creature(10_002, cards::ISLAND, PlayerId::Two));
    game.priority = PlayerId::One;

    let pool_id = game.battlefield[0].card.id;
    let colors: Vec<ManaColor> = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::ActivateManaAbility { source, color, .. } if source == pool_id => Some(color),
            _ => None,
        })
        .collect();
    assert_eq!(
        colors,
        vec![ManaColor::Green],
        "the Forest lends green and the opponent's Island lends nothing",
    );

    // Ancient Tomb makes colourless, which "any type" accepts.
    game.battlefield
        .push(creature(10_003, cards::ANCIENT_TOMB, PlayerId::One));
    let mut colors: Vec<ManaColor> = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::ActivateManaAbility { source, color, .. } if source == pool_id => Some(color),
            _ => None,
        })
        .collect();
    colors.sort_unstable();
    colors.dedup();
    assert!(
        colors.contains(&ManaColor::Colorless),
        "a type, not a colour: {colors:?}",
    );
}

/// Krosan Reclamation puts back what its controller picks out of the target
/// player's graveyard, and nothing else moves.
#[test]
fn krosan_reclamation_shuffles_back_the_chosen_cards() {
    let mut game = ready_game();
    game.players[PlayerId::Two.index()].library.clear();
    for index in 0..3 {
        game.players[PlayerId::Two.index()].graveyard.push(card(
            10_010 + index,
            cards::LIGHTNING_BOLT,
            PlayerId::Two,
        ));
    }
    // The controller's own graveyard is not the one being emptied.
    game.players[PlayerId::One.index()].graveyard.push(card(
        10_020,
        cards::COUNTERSPELL,
        PlayerId::One,
    ));

    let reclamation = card(10_000, cards::KROSAN_RECLAMATION, PlayerId::One);
    let reclamation_id = reclamation.id;
    game.players[PlayerId::One.index()].hand.push(reclamation);
    let pool = &mut game.players[PlayerId::One.index()].mana_pool;
    pool.green = 1;
    pool.colorless = 1;
    game.priority = PlayerId::One;
    game.apply(
        PlayerId::One,
        cast_action(
            reclamation_id,
            vec![Target::Player(PlayerId::Two)],
            Vec::new(),
            0,
        ),
    )
    .expect("two mana casts it");
    pass_until_decision(&mut game);
    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the caster picks which cards go back");
    let options = decision
        .options
        .iter()
        .take(2)
        .map(|option| option.id)
        .collect();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options,
        },
    )
    .expect("two of the three is within the limit");
    drain_pending(&mut game);

    assert_eq!(
        game.players[PlayerId::Two.index()].library.len(),
        2,
        "two of the three chosen cards went back",
    );
    assert_eq!(
        game.players[PlayerId::Two.index()].graveyard.len(),
        1,
        "and the third stayed put",
    );
    assert_eq!(
        game.players[PlayerId::One.index()].graveyard.len(),
        2,
        "the caster's own graveyard kept its card and gained the Reclamation",
    );
}

/// Gilded Drake trades itself for the best thing on the other side, and the
/// exchange is not something the turn ending undoes.
#[test]
fn gilded_drake_exchanges_itself_for_a_creature() {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_010, cards::SERRA_ANGEL, PlayerId::Two));
    let angel = game.battlefield[0].card.id;

    let drake = card(10_000, cards::GILDED_DRAKE, PlayerId::One);
    let drake_id = drake.id;
    game.players[PlayerId::One.index()].hand.push(drake);
    let pool = &mut game.players[PlayerId::One.index()].mana_pool;
    pool.blue = 1;
    pool.colorless = 1;
    game.priority = PlayerId::One;
    // The exchange belongs to the enters trigger, so its target is chosen
    // when the trigger goes on the stack rather than as the Drake is cast.
    game.apply(
        PlayerId::One,
        cast_action(drake_id, Vec::new(), Vec::new(), 0),
    )
    .expect("two mana casts it");
    pass_until_decision(&mut game);
    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the trigger asks which creature to take");
    let angel_option = decision
        .options
        .iter()
        .find(|option| option.card.is_some_and(|(card, _)| card == angel))
        .expect("the Angel is the only creature an opponent controls")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![angel_option],
        },
    )
    .expect("naming it is legal");
    pass_until_decision(&mut game);
    drain_pending(&mut game);

    let controller = |game: &Game, definition| {
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.definition == definition)
            .map(|permanent| permanent.controller)
    };
    assert_eq!(
        controller(&game, cards::SERRA_ANGEL),
        Some(PlayerId::One),
        "the Angel changed hands",
    );
    assert_eq!(
        controller(&game, cards::GILDED_DRAKE),
        Some(PlayerId::Two),
        "and the Drake went the other way",
    );

    // An exchange lasts indefinitely, so cleanup gives nothing back.
    for _ in 0..8 {
        if game.step == Step::End {
            break;
        }
        game.advance_step();
    }
    game.finish_rules_procedure();
    pass_until_decision(&mut game);
    drain_pending(&mut game);
    assert_eq!(
        controller(&game, cards::SERRA_ANGEL),
        Some(PlayerId::One),
        "the turn ending does not undo an exchange",
    );
}

/// With nothing to take, the Drake goes instead of staying for free.
#[test]
fn gilded_drake_sacrifices_itself_with_nothing_to_exchange() {
    let mut game = ready_game();
    let drake = card(10_000, cards::GILDED_DRAKE, PlayerId::One);
    let drake_id = drake.id;
    game.players[PlayerId::One.index()].hand.push(drake);
    let pool = &mut game.players[PlayerId::One.index()].mana_pool;
    pool.blue = 1;
    pool.colorless = 1;
    game.priority = PlayerId::One;
    game.apply(
        PlayerId::One,
        cast_action(drake_id, Vec::new(), Vec::new(), 0),
    )
    .expect("an empty board still lets it be cast");
    pass_until_decision(&mut game);
    drain_pending(&mut game);

    assert!(
        game.battlefield.is_empty(),
        "no exchange was made, so it sacrifices itself",
    );
    assert_eq!(
        game.players[PlayerId::One.index()].graveyard.len(),
        1,
        "and it is in its owner's graveyard",
    );
}

fn gilded_drake_with_targeted_trigger() -> (Game, GameObjectId, GameObjectId) {
    let mut game = ready_game();
    game.battlefield
        .push(creature(10_010, cards::SERRA_ANGEL, PlayerId::Two));
    let angel = game.battlefield[0].card.id;
    let drake = card(10_000, cards::GILDED_DRAKE, PlayerId::One);
    let drake_id = drake.id;
    game.players[PlayerId::One.index()].hand.push(drake);
    game.players[PlayerId::One.index()].mana_pool.blue = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;
    game.priority = PlayerId::One;
    game.apply(
        PlayerId::One,
        cast_action(drake_id, Vec::new(), Vec::new(), 0),
    )
    .expect("two mana casts it");
    pass_until_decision(&mut game);
    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the trigger asks which creature to take");
    let target = decision
        .options
        .iter()
        .find(|option| option.card.is_some_and(|(card, _)| card == angel))
        .expect("the Angel is a legal target")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![target],
        },
    )
    .expect("the trigger names the Angel");
    let drake = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::GILDED_DRAKE)
        .expect("the Drake entered before its trigger was placed")
        .card
        .id;
    (game, drake, angel)
}

/// Gilded Drake's Oracle exception keeps the trigger resolving after its only
/// target leaves, so the failed exchange reaches the sacrifice instruction.
#[test]
fn gilded_drake_sacrifices_itself_when_its_target_leaves() {
    let (mut game, drake, angel) = gilded_drake_with_targeted_trigger();
    game.destroy_permanent(angel);
    drain_pending(&mut game);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != drake),
        "the trigger resolved and sacrificed the Drake",
    );
    assert!(game.events.iter().any(|event| matches!(
        event,
        GameEvent::TriggeredAbilityResolved { source, .. } if *source == drake
    )));
    assert!(!game.events.iter().any(|event| matches!(
        event,
        GameEvent::TriggeredAbilityFizzled { source, .. } if *source == drake
    )));
}

/// An illegal target can still exist. The unfizzling trigger must ignore it,
/// fail the exchange, and sacrifice rather than swapping same-seat objects.
#[test]
fn gilded_drake_sacrifices_itself_when_its_target_changes_sides() {
    let (mut game, drake, angel) = gilded_drake_with_targeted_trigger();
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == angel)
        .expect("the Angel is still on the battlefield")
        .controller = PlayerId::One;
    drain_pending(&mut game);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != drake),
        "the impossible exchange sacrificed the Drake",
    );
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == angel)
            .map(|permanent| permanent.controller),
        Some(PlayerId::One),
        "the illegal target was not affected",
    );
}

/// With nothing to feed it, the Dreadnought sacrifices itself and the payer
/// is never asked.
#[test]
fn phyrexian_dreadnought_eats_itself_on_an_empty_board() {
    let mut game = ready_game();
    let dreadnought = card(10_000, cards::PHYREXIAN_DREADNOUGHT, PlayerId::One);
    let dreadnought_id = dreadnought.id;
    game.players[PlayerId::One.index()].hand.push(dreadnought);
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;
    game.priority = PlayerId::One;
    game.apply(
        PlayerId::One,
        cast_action(dreadnought_id, Vec::new(), Vec::new(), 0),
    )
    .expect("one mana casts it");
    pass_until_decision(&mut game);
    drain_pending(&mut game);

    assert!(
        game.battlefield.is_empty(),
        "twelve power is unreachable, so it goes",
    );
    assert_eq!(
        game.players[PlayerId::One.index()].graveyard.len(),
        1,
        "and it is in the graveyard",
    );
}

/// Given enough creatures, the payer feeds it one at a time and keeps it.
#[test]
fn phyrexian_dreadnought_can_be_paid_for_one_creature_at_a_time() {
    let mut game = ready_game();
    // Two Serra Angels are eight power; a third makes twelve.
    for index in 0..3 {
        game.battlefield
            .push(creature(10_010 + index, cards::SERRA_ANGEL, PlayerId::One));
    }
    let dreadnought = card(10_000, cards::PHYREXIAN_DREADNOUGHT, PlayerId::One);
    let dreadnought_id = dreadnought.id;
    game.players[PlayerId::One.index()].hand.push(dreadnought);
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;
    game.priority = PlayerId::One;
    game.apply(
        PlayerId::One,
        cast_action(dreadnought_id, Vec::new(), Vec::new(), 0),
    )
    .expect("one mana casts it");
    pass_until_decision(&mut game);

    // Accept the cost, then feed it Angels until it is satisfied.
    let offer = game
        .observe(PlayerId::One)
        .decision
        .expect("the payer is asked whether to pay");
    let pay = offer
        .options
        .iter()
        .find(|option| option.id != 0)
        .expect("paying is on offer")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: offer.id,
            options: vec![pay],
        },
    )
    .expect("paying is legal");

    for _ in 0..3 {
        let Some(step) = game.observe(PlayerId::One).decision else {
            break;
        };
        let Some(angel) = step.options.iter().find(|option| option.id != 0) else {
            break;
        };
        let angel = angel.id;
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: step.id,
                options: vec![angel],
            },
        )
        .expect("each Angel is a legal way to pay");
    }
    // Twelve is a floor rather than a quota, so once it is met the payer is
    // offered the chance to stop.
    let stop = game
        .observe(PlayerId::One)
        .decision
        .expect("the total is met, and stopping is on offer");
    assert!(
        stop.options.iter().any(|option| option.label == "Stop"),
        "reaching the total offers a way out of paying more",
    );
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: stop.id,
            options: vec![0],
        },
    )
    .expect("stopping is legal once the total is met");
    drain_pending(&mut game);

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::PHYREXIAN_DREADNOUGHT),
        "the cost was paid, so it stays",
    );
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| permanent.card.definition == cards::SERRA_ANGEL)
            .count(),
        0,
        "and three Angels went to pay for it",
    );
}

/// Dragon Breath listens from the graveyard: something enormous arrives and
/// it comes back attached to give it haste.
#[test]
fn dragon_breath_returns_from_the_graveyard_attached() {
    let mut game = ready_game();
    game.players[PlayerId::One.index()].graveyard.push(card(
        10_000,
        cards::DRAGON_BREATH,
        PlayerId::One,
    ));

    // Serra Angel costs five, which is not enough to wake it.
    game.battlefield
        .push(creature(10_010, cards::SERRA_ANGEL, PlayerId::One));
    game.finish_rules_procedure();
    pass_until_decision(&mut game);
    drain_pending(&mut game);
    assert_eq!(
        game.players[PlayerId::One.index()].graveyard.len(),
        1,
        "a five-mana creature is under the threshold",
    );

    // Six is the threshold, and Exalted Angel is exactly six.
    let big = card(10_020, cards::EXALTED_ANGEL, PlayerId::One);
    let big_id = big.id;
    game.players[PlayerId::One.index()].hand.push(big);
    let pool = &mut game.players[PlayerId::One.index()].mana_pool;
    pool.white = 2;
    pool.colorless = 4;
    game.priority = PlayerId::One;
    game.apply(
        PlayerId::One,
        cast_action(big_id, Vec::new(), Vec::new(), 0),
    )
    .expect("six mana casts it face up");
    pass_until_decision(&mut game);
    let offer = game
        .observe(PlayerId::One)
        .decision
        .expect("the trigger asks whether to bring it back");
    let accept = offer
        .options
        .iter()
        .find(|option| option.label != "Decline")
        .expect("accepting is on offer")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: offer.id,
            options: vec![accept],
        },
    )
    .expect("accepting is legal");
    drain_pending(&mut game);

    let breath = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::DRAGON_BREATH)
        .expect("the Breath came back from the graveyard");
    let angel = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::EXALTED_ANGEL)
        .expect("the six-drop is on the battlefield");
    assert_eq!(
        breath.attached_to,
        Some(angel.card.id),
        "and it arrived attached to the creature that woke it",
    );
    assert!(
        game.permanent_has_executable_keyword(angel, KeywordAbility::Haste),
        "which is what the deck wants it for",
    );
}

/// Sutured Ghoul is the pile it ate: the creature cards it exiles as it
/// enters are what its power and toughness are read from.
#[test]
fn sutured_ghoul_is_the_size_of_what_it_exiled() {
    let mut game = ready_game();
    // Serra Angel is 4/4 and Grizzly Bears 2/2, so together 6/6.
    game.players[PlayerId::One.index()].graveyard.push(card(
        10_010,
        cards::SERRA_ANGEL,
        PlayerId::One,
    ));
    game.players[PlayerId::One.index()].graveyard.push(card(
        10_011,
        cards::GRIZZLY_BEARS,
        PlayerId::One,
    ));
    // A noncreature card in the same graveyard is not on offer.
    game.players[PlayerId::One.index()].graveyard.push(card(
        10_012,
        cards::LIGHTNING_BOLT,
        PlayerId::One,
    ));

    let ghoul = card(10_000, cards::SUTURED_GHOUL, PlayerId::One);
    let ghoul_id = ghoul.id;
    game.players[PlayerId::One.index()].hand.push(ghoul);
    let pool = &mut game.players[PlayerId::One.index()].mana_pool;
    pool.black = 3;
    pool.colorless = 4;
    game.priority = PlayerId::One;
    game.apply(
        PlayerId::One,
        cast_action(ghoul_id, Vec::new(), Vec::new(), 0),
    )
    .expect("seven mana casts it");
    pass_until_decision(&mut game);

    let choice = game
        .observe(PlayerId::One)
        .decision
        .expect("the pile is chosen as it enters");
    assert_eq!(
        choice.options.len(),
        2,
        "only the creature cards are on offer",
    );
    let both = choice.options.iter().map(|option| option.id).collect();
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: choice.id,
            options: both,
        },
    )
    .expect("any number includes all of them");
    drain_pending(&mut game);

    let ghoul = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::SUTURED_GHOUL)
        .expect("a six-power body survives the state-based check that a 0/0 does not");
    let stats = game.creature_stats(ghoul).expect("it is a creature");
    assert_eq!(
        (stats.power, stats.toughness),
        (6, 6),
        "four plus two, read off the pile it took",
    );
    assert_eq!(
        game.players[PlayerId::One.index()].exile.len(),
        2,
        "and the pile is in exile",
    );
    assert_eq!(
        game.players[PlayerId::One.index()].graveyard.len(),
        1,
        "the Bolt stayed behind",
    );
}

/// A phased-out permanent is treated as though it does not exist, and comes
/// back before its controller untaps.
#[test]
fn a_phased_out_permanent_is_gone_until_its_controller_untaps() {
    let mut game = ready_game();
    let mut vise = creature(10_000, cards::BLACK_VISE, PlayerId::Two);
    vise.tapped = true;
    game.battlefield.push(vise);
    let vise_id = game.battlefield[0].card.id;

    game.phase_out(vise_id);
    assert!(
        game.battlefield.is_empty(),
        "nothing on the battlefield sees it",
    );
    assert!(
        game.permanent_controller(vise_id).is_none(),
        "and neither does anything asking after it",
    );

    // Its controller's untap step brings it back, before the untap itself.
    game.commit_next_turn(PlayerId::Two, Vec::new());
    let returned = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == vise_id)
        .expect("it phased in");
    assert!(
        !returned.tapped,
        "and it untapped with everything else, having come back first",
    );
}

/// Vision Charm's land mode turns every land of one basic type into
/// another for the turn, which is what strands an opponent's colours.
#[test]
fn vision_charm_turns_one_land_type_into_another() {
    let mut game = ready_game();
    for index in 0..2 {
        game.battlefield
            .push(creature(10_010 + index, cards::ISLAND, PlayerId::Two));
    }
    game.battlefield
        .push(creature(10_020, cards::FOREST, PlayerId::Two));

    let charm = card(10_000, cards::VISION_CHARM, PlayerId::One);
    let charm_id = charm.id;
    game.players[PlayerId::One.index()].hand.push(charm);
    game.players[PlayerId::One.index()].mana_pool.blue = 1;
    game.priority = PlayerId::One;
    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, choices, .. }
                if *card == charm_id && choices.modes() == [ModeId(1)])
        })
        .expect("the land mode is one of the three on offer");
    game.apply(PlayerId::One, cast).unwrap();
    pass_until_decision(&mut game);

    let choice = game
        .observe(PlayerId::One)
        .decision
        .expect("the pair of types is chosen as it resolves");
    let island_to_swamp = choice
        .options
        .iter()
        .find(|option| option.label == "Island → Swamp")
        .expect("every ordered pair is offered")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: choice.id,
            options: vec![island_to_swamp],
        },
    )
    .expect("naming a pair is legal");
    drain_pending(&mut game);

    let swamps = game
        .battlefield
        .iter()
        .filter(|permanent| game.effective_land_types(permanent)[BasicLandType::Swamp.index()])
        .count();
    assert_eq!(swamps, 2, "both Islands are Swamps now");
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| game.effective_land_types(permanent)[BasicLandType::Island.index()])
            .count(),
        0,
        "and nothing is an Island: becoming a type replaces the old one",
    );
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| game.effective_land_types(permanent)[BasicLandType::Forest.index()])
            .count(),
        1,
        "the Forest was never of the first type, so it is untouched",
    );
}
