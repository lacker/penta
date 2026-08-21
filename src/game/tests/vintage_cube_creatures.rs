//! Creatures cataloged for the Vintage Cube pool.

use super::search_and_reveal::stack_library;
use super::*;

/// The Ent as a spell: six mana for a body that brings a Food with it.
#[test]
fn the_ent_arrives_with_reach_and_a_food_token() {
    let mut game = ready_game();
    game.battlefield.clear();
    let ent = game
        .put_onto_battlefield(PlayerId::One, cards::GENEROUS_ENT)
        .expect("cataloged");
    drain_pending(&mut game);

    let ent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == ent)
        .expect("it entered");
    assert_eq!((game.power(ent), game.toughness(ent)), (Some(5), Some(7)));
    assert!(
        game.permanent_has_executable_keyword(ent, KeywordAbility::Reach),
        "a Treefolk this size blocks fliers",
    );

    let food = game
        .battlefield
        .iter()
        .find(|permanent| is_token_with(permanent, tokens::food()))
        .expect("the enters trigger made a Food");
    let rules = game.effective_rules(food).expect("the token has rules");
    assert!(
        rules.has_subtype("Food"),
        "Food is an artifact type, not a creature type",
    );
    assert!(rules.has_type(crate::card::CardType::Artifact));
    assert!(!rules.has_type(crate::card::CardType::Creature));
}

/// The Food it left behind: three life for two mana and itself.
#[test]
fn the_food_token_is_eaten_for_three_life() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.create_token(PlayerId::One, tokens::food());
    drain_pending(&mut game);
    let food = game
        .battlefield
        .iter()
        .find(|permanent| is_token_with(permanent, tokens::food()))
        .expect("the Food token arrived")
        .card
        .id;
    game.players[PlayerId::One.index()].life = 10;
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == food))
        .expect("the Food can be eaten");
    game.apply(PlayerId::One, action).expect("it is activated");
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != food),
        "sacrificing it is a cost",
    );
    drain_pending(&mut game);
    assert_eq!(game.players[PlayerId::One.index()].life, 13);
}

/// The Ent as a land: one mana from hand, and it fetches a Forest instead of
/// drawing. Anything with the Forest subtype counts, not just the basic.
#[test]
fn forestcycling_finds_a_forest_rather_than_drawing() {
    let mut game = ready_game();
    game.players[PlayerId::One.index()].hand.clear();
    game.players[PlayerId::One.index()].library.clear();
    stack_library(
        &mut game,
        &[
            (52_000, cards::LIGHTNING_BOLT),
            (52_001, cards::TAIGA),
            (52_002, cards::ISLAND),
        ],
    );
    let ent = card(52_003, cards::GENEROUS_ENT, PlayerId::One);
    let ent_id = ent.id;
    game.players[PlayerId::One.index()].hand.push(ent);
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == ent_id),
        )
        .expect("forestcycling is offered from hand");
    game.apply(PlayerId::One, action).expect("it is activated");
    assert!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::GENEROUS_ENT),
        "the discard is a cost",
    );
    pass_priority_pair(&mut game);

    let decision = game.observe(PlayerId::One).decision.expect("a search");
    assert_eq!(
        decision
            .options
            .iter()
            .filter_map(|option| option
                .card
                .and_then(|(_, characteristics)| characteristics.card_definition()))
            .collect::<Vec<_>>(),
        vec![cards::TAIGA],
        "a dual land is a Forest; the Island and the Bolt are not",
    );
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![decision.options[0].id],
        },
    )
    .expect("the search is answered");

    assert!(
        game.players[PlayerId::One.index()]
            .hand
            .iter()
            .any(|card| card.definition == cards::TAIGA),
        "the found land goes to hand rather than the battlefield",
    );
}

/// The Titan's ability is one ability with two ways in. Both paths reach the
/// same search, and the search takes any land rather than only basics.
#[test]
fn the_titan_fetches_on_entering_and_again_on_attacking() {
    for attack_instead in [false, true] {
        let mut game = ready_game();
        game.battlefield.clear();
        game.players[PlayerId::One.index()].library.clear();
        stack_library(
            &mut game,
            &[
                (58_000, cards::TAIGA),
                (58_001, cards::FOREST),
                (58_002, cards::LIGHTNING_BOLT),
            ],
        );

        let titan = if attack_instead {
            // Already here since last turn, so it can attack.
            let titan = creature(58_100, cards::PRIMEVAL_TITAN, PlayerId::One);
            let id = titan.card.id;
            game.battlefield.push(titan);
            id
        } else {
            game.put_onto_battlefield(PlayerId::One, cards::PRIMEVAL_TITAN)
                .expect("cataloged")
        };

        if attack_instead {
            game.step = Step::DeclareAttackers;
            game.declare_attacker(titan, AttackDefender::Player(PlayerId::Two));
            game.finish_declaring_attackers();
        }

        // The search is optional, so answering it is what takes the lands.
        let decision = loop {
            if let Some(decision) = game.observe(PlayerId::One).decision {
                break decision;
            }
            let player = game.priority;
            game.apply(player, Action::PassPriority)
                .expect("the trigger is on the stack");
        };
        let accept = decision
            .options
            .last()
            .expect("the optional search offers accepting it")
            .id;
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: decision.id,
                options: vec![accept],
            },
        )
        .expect("the search is accepted");

        let search = game
            .observe(PlayerId::One)
            .decision
            .expect("the library search follows");
        let mut offered = search
            .options
            .iter()
            .filter_map(|option| {
                option
                    .card
                    .and_then(|(_, characteristics)| characteristics.card_definition())
            })
            .collect::<Vec<_>>();
        offered.sort_unstable();
        let mut lands = vec![cards::TAIGA, cards::FOREST];
        lands.sort_unstable();
        assert_eq!(
            offered, lands,
            "any land card, and nothing that is not a land (attacking: {attack_instead})",
        );
        let chosen = search
            .options
            .iter()
            .map(|option| option.id)
            .collect::<Vec<_>>();
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: search.id,
                options: chosen,
            },
        )
        .expect("both lands are taken");

        for land in [cards::TAIGA, cards::FOREST] {
            let found = game
                .battlefield
                .iter()
                .find(|permanent| permanent.card.definition == land)
                .unwrap_or_else(|| panic!("{land:?} arrived"));
            assert!(found.tapped, "the lands arrive tapped");
        }
    }
}

/// Cecil's front half turns the damage it deals into life loss, and the same
/// clause checks afterwards whether that loss has taken its controller low
/// enough to turn the card over.
#[test]
fn cecil_transforms_once_his_own_damage_has_halved_your_life() {
    for (starting_life, transforms) in [(20, false), (13, true)] {
        let mut game = ready_game();
        game.battlefield.clear();
        game.players[PlayerId::One.index()].life = starting_life;
        let cecil = creature(59_000, cards::CECIL_DARK_KNIGHT, PlayerId::One);
        let cecil_id = cecil.card.id;
        game.battlefield.push(cecil);
        game.tap_permanent(cecil_id);

        game.damage_target_from(Some(cecil_id), Some(Target::Player(PlayerId::Two)), 3);
        drain_pending(&mut game);

        assert_eq!(
            game.players[PlayerId::One.index()].life,
            starting_life - 3,
            "the damage Cecil dealt is repaid in life",
        );
        let permanent = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == cecil_id)
            .expect("he is still here");
        assert_eq!(
            permanent.presented == CardPartId(1),
            transforms,
            "at {starting_life} life, three damage should{} turn him over",
            if transforms { "" } else { " not" },
        );
        if transforms {
            assert!(!permanent.tapped, "and untap him on the way");
            assert_eq!(
                (game.power(permanent), game.toughness(permanent)),
                (Some(4), Some(4)),
                "the back half is the bigger one",
            );
        }
    }
}

/// The back half protects the rest of the attack, and not itself: "other
/// attacking creatures" is the whole clause.
#[test]
fn the_redeemed_paladin_covers_the_other_attackers() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[PlayerId::One.index()].life = 5;
    let cecil = creature(59_100, cards::CECIL_DARK_KNIGHT, PlayerId::One);
    let cecil_id = cecil.card.id;
    game.battlefield.push(cecil);
    // Halve his controller's life with his own damage to get the back face.
    game.damage_target_from(Some(cecil_id), Some(Target::Player(PlayerId::Two)), 1);
    drain_pending(&mut game);
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == cecil_id)
            .expect("still here")
            .presented,
        CardPartId(1),
        "the Paladin side is up",
    );

    let friend = creature(59_101, cards::GRIZZLY_BEARS, PlayerId::One);
    let friend_id = friend.card.id;
    game.battlefield.push(friend);
    let bystander = creature(59_102, cards::SAVANNAH_LIONS, PlayerId::One);
    let bystander_id = bystander.card.id;
    game.battlefield.push(bystander);

    game.step = Step::DeclareAttackers;
    game.declare_attacker(cecil_id, AttackDefender::Player(PlayerId::Two));
    game.declare_attacker(friend_id, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    drain_pending(&mut game);

    let indestructible = |id| {
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .is_some_and(|permanent| game.has_indestructible(permanent))
    };
    assert!(indestructible(friend_id), "the other attacker is covered");
    assert!(
        !indestructible(bystander_id),
        "a creature that stayed home is not attacking",
    );
    assert!(
        !indestructible(cecil_id),
        "and \"other\" excludes Cecil himself",
    );
}

/// A Lhurgoyf counts card types, not cards: a graveyard of ten creatures is
/// worth the same as a graveyard of one.
#[test]
fn pyrogoyf_grows_with_the_types_in_every_graveyard_not_the_cards() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[PlayerId::One.index()].graveyard.clear();
    game.players[PlayerId::Two.index()].graveyard.clear();
    let goyf = game
        .put_onto_battlefield(PlayerId::One, cards::PYROGOYF)
        .expect("cataloged");
    let stats = |game: &Game| {
        let permanent = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == goyf)
            .expect("still there");
        (game.power(permanent), game.toughness(permanent))
    };
    assert_eq!(
        stats(&game),
        (Some(0), Some(1)),
        "an empty graveyard is 0/1"
    );

    // Three creature cards are still one type.
    for instance in 0..3 {
        game.players[PlayerId::One.index()].graveyard.push(card(
            62_000 + instance,
            cards::GRIZZLY_BEARS,
            PlayerId::One,
        ));
    }
    assert_eq!(
        stats(&game),
        (Some(1), Some(2)),
        "one type among three cards"
    );

    // An instant in the same graveyard is a second type.
    game.players[PlayerId::One.index()].graveyard.push(card(
        62_100,
        cards::LIGHTNING_BOLT,
        PlayerId::One,
    ));
    assert_eq!(stats(&game), (Some(2), Some(3)));

    // "All graveyards" reaches across the table.
    game.players[PlayerId::Two.index()]
        .graveyard
        .push(card(62_200, cards::FOREST, PlayerId::Two));
    assert_eq!(
        stats(&game),
        (Some(3), Some(4)),
        "the opponent's land counts"
    );

    // A second instant adds nothing, because the type is already there.
    game.players[PlayerId::Two.index()].graveyard.push(card(
        62_300,
        cards::ANCESTRAL_RECALL,
        PlayerId::Two,
    ));
    assert_eq!(stats(&game), (Some(3), Some(4)));
}

/// Its own arrival is what usually triggers it, and the damage is read from
/// the creature that entered.
#[test]
fn pyrogoyf_burns_on_arrival_for_as_much_as_it_is_worth() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[PlayerId::One.index()].graveyard.clear();
    game.players[PlayerId::Two.index()].graveyard.clear();
    // Two types in the graveyard: a 2/3 arriving.
    game.players[PlayerId::One.index()].graveyard.push(card(
        62_400,
        cards::GRIZZLY_BEARS,
        PlayerId::One,
    ));
    game.players[PlayerId::One.index()].graveyard.push(card(
        62_401,
        cards::LIGHTNING_BOLT,
        PlayerId::One,
    ));
    let before = game.players[PlayerId::Two.index()].life;

    game.put_onto_battlefield(PlayerId::One, cards::PYROGOYF)
        .expect("cataloged");

    let decision = loop {
        if let Some(decision) = game.observe(PlayerId::One).decision {
            break decision;
        }
        let player = game.priority;
        assert!(
            game.apply(player, Action::PassPriority).is_ok(),
            "the enters trigger should be waiting on a target",
        );
    };
    let opponent = decision
        .options
        .iter()
        .find(|option| option.label == "your opponent")
        .expect("the opponent is one of the offered targets")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![opponent],
        },
    )
    .expect("a target is chosen");
    drain_pending(&mut game);

    assert_eq!(
        before - game.players[PlayerId::Two.index()].life,
        2,
        "a 2/3 Lhurgoyf burns for two",
    );
}

/// The Krasis lends its own body: a 1/1 becomes a 4/4 while the Krasis is a
/// 4/4, and a 7/7 while it is a 7/7. Setting a base rather than adding to one
/// is what makes the small creature big rather than bigger.
#[test]
fn the_krasis_sets_another_creature_to_its_own_size() {
    for (adapted, expected) in [(false, 4), (true, 7)] {
        let mut game = ready_game();
        game.battlefield.clear();
        let krasis = creature(64_000, cards::UNRULY_KRASIS, PlayerId::One);
        let krasis_id = krasis.card.id;
        game.battlefield.push(krasis);
        let lions = creature(64_001, cards::SAVANNAH_LIONS, PlayerId::One);
        let lions_id = lions.card.id;
        game.battlefield.push(lions);

        if adapted {
            game.players[PlayerId::One.index()].mana_pool.green = 1;
            game.players[PlayerId::One.index()].mana_pool.blue = 1;
            game.players[PlayerId::One.index()].mana_pool.colorless = 3;
            let adapt = game
                .legal_actions(PlayerId::One)
                .into_iter()
                .find(|action| {
                    matches!(action, Action::ActivateAbility { source, .. } if *source == krasis_id)
                })
                .expect("adapt is offered");
            game.apply(PlayerId::One, adapt).expect("it activates");
            drain_pending(&mut game);
        }

        game.step = Step::DeclareAttackers;
        game.declare_attacker(krasis_id, AttackDefender::Player(PlayerId::Two));
        game.finish_declaring_attackers();

        // Two answers follow in order: which creature the trigger targets,
        // and then whether to take the optional effect at all. The last
        // option is the affirmative one in both.
        for _ in 0..16 {
            if let Some(decision) = game.observe(PlayerId::One).decision {
                let accept = decision.options.last().expect("an option is offered").id;
                game.apply(
                    PlayerId::One,
                    Action::ChooseDecision {
                        decision: decision.id,
                        options: vec![accept],
                    },
                )
                .expect("the offered option is legal");
                continue;
            }
            if game.stack.is_empty() && game.pending_triggers.is_empty() {
                break;
            }
            let player = game.priority;
            assert!(
                game.apply(player, Action::PassPriority).is_ok(),
                "the attack trigger is waiting",
            );
        }

        let lions = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == lions_id)
            .expect("still there");
        assert_eq!(
            (game.power(lions), game.toughness(lions)),
            (Some(expected), Some(expected)),
            "a 2/1 takes the Krasis's size (adapted: {adapted})",
        );
    }
}

/// Adapt is a conditional rather than a cost: the second activation resolves
/// and simply finds counters already there.
#[test]
fn the_krasis_adapts_only_while_it_has_no_counters() {
    let mut game = ready_game();
    game.battlefield.clear();
    let krasis = creature(64_100, cards::UNRULY_KRASIS, PlayerId::One);
    let krasis_id = krasis.card.id;
    game.battlefield.push(krasis);

    let activate = |game: &mut Game| {
        game.players[PlayerId::One.index()].mana_pool.green = 1;
        game.players[PlayerId::One.index()].mana_pool.blue = 1;
        game.players[PlayerId::One.index()].mana_pool.colorless = 3;
        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| {
                matches!(action, Action::ActivateAbility { source, .. } if *source == krasis_id)
            })
            .expect("adapt is always offered");
        game.apply(PlayerId::One, action).expect("it activates");
        drain_pending(game);
    };

    activate(&mut game);
    let size = |game: &Game| {
        let krasis = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == krasis_id)
            .expect("still there");
        (game.power(krasis), game.toughness(krasis))
    };
    assert_eq!(size(&game), (Some(7), Some(7)), "three counters arrive");

    activate(&mut game);
    assert_eq!(
        size(&game),
        (Some(7), Some(7)),
        "and a second adapt adds nothing while they are still on it",
    );
}

/// The Sculler holds a card rather than taking it: the exile is linked to
/// the body, so answering the body gives the card back. And it is "leaves",
/// not "dies", so bouncing the Sculler returns the card too.
#[test]
fn the_sculler_holds_a_nonland_card_until_it_leaves() {
    for bounce_instead in [false, true] {
        let mut game = ready_game();
        game.battlefield.clear();
        game.players[PlayerId::Two.index()].hand.clear();
        // Two nonlands, so the choice is a real one, and a land that must
        // never be among the options.
        game.players[PlayerId::Two.index()].hand.push(card(
            66_000,
            cards::LIGHTNING_BOLT,
            PlayerId::Two,
        ));
        game.players[PlayerId::Two.index()].hand.push(card(
            66_001,
            cards::SERRA_ANGEL,
            PlayerId::Two,
        ));
        game.players[PlayerId::Two.index()]
            .hand
            .push(card(66_002, cards::FOREST, PlayerId::Two));

        let sculler = game
            .put_onto_battlefield(PlayerId::One, cards::TIDEHOLLOW_SCULLER)
            .expect("cataloged");

        // Two answers: which opponent the trigger targets, and then which
        // card to take out of the hand it revealed.
        let mut offered_cards = Vec::new();
        for _ in 0..8 {
            if let Some(decision) = game.observe(PlayerId::One).decision {
                let cards = decision
                    .options
                    .iter()
                    .filter_map(|option| {
                        option
                            .card
                            .and_then(|(_, characteristics)| characteristics.card_definition())
                    })
                    .collect::<Vec<_>>();
                if !cards.is_empty() {
                    offered_cards = cards;
                }
                game.apply(
                    PlayerId::One,
                    Action::ChooseDecision {
                        decision: decision.id,
                        options: vec![decision.options[0].id],
                    },
                )
                .expect("the offered choice is legal");
                continue;
            }
            if game.stack.is_empty() && game.pending_triggers.is_empty() {
                break;
            }
            let player = game.priority;
            assert!(
                game.apply(player, Action::PassPriority).is_ok(),
                "the enters trigger is waiting",
            );
        }
        offered_cards.sort_unstable();
        let mut nonlands = vec![cards::LIGHTNING_BOLT, cards::SERRA_ANGEL];
        nonlands.sort_unstable();
        assert_eq!(
            offered_cards, nonlands,
            "the nonland cards, and the Forest is not one",
        );

        assert_eq!(
            game.players[PlayerId::Two.index()].exile.len(),
            1,
            "one card is held while the Sculler stands",
        );
        let held = game.players[PlayerId::Two.index()].exile[0].definition;
        assert!(
            game.players[PlayerId::Two.index()]
                .hand
                .iter()
                .all(|card| card.definition != held),
        );

        if bounce_instead {
            game.return_permanent_to_hand(sculler);
        } else {
            game.move_permanents_to_graveyard(&[sculler]);
        }
        drain_pending(&mut game);

        assert!(
            game.players[PlayerId::Two.index()]
                .hand
                .iter()
                .any(|card| card.definition == held),
            "and comes back to its owner's hand once the body leaves \
             (bounced: {bounce_instead})",
        );
        assert!(game.players[PlayerId::Two.index()].exile.is_empty());
    }
}

/// Doubling compounds within a turn, because each trigger reads the size the
/// one before it left behind. A 1/2 that sees three lands is an 8/2.
#[test]
fn tifa_doubles_her_power_once_per_land_and_compounds() {
    let mut game = ready_game();
    game.battlefield.clear();
    let tifa = creature(68_000, cards::TIFA_LOCKHART, PlayerId::One);
    let tifa_id = tifa.card.id;
    game.battlefield.push(tifa);

    let size = |game: &Game| {
        let tifa = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == tifa_id)
            .expect("she is still there");
        (game.power(tifa), game.toughness(tifa))
    };
    assert_eq!(size(&game), (Some(1), Some(2)));

    for (index, expected) in [2, 4, 8].into_iter().enumerate() {
        game.put_onto_battlefield(
            PlayerId::One,
            if index == 0 {
                cards::FOREST
            } else {
                cards::ISLAND
            },
        )
        .expect("a land arrives");
        drain_pending(&mut game);
        assert_eq!(
            size(&game),
            (Some(expected), Some(2)),
            "land {index} doubles her power and leaves toughness alone",
        );
    }
}

/// "A land you control" is the whole restriction: the opponent's land does
/// nothing for her.
#[test]
fn tifa_ignores_lands_the_other_player_plays() {
    let mut game = ready_game();
    game.battlefield.clear();
    let tifa = creature(68_100, cards::TIFA_LOCKHART, PlayerId::One);
    let tifa_id = tifa.card.id;
    game.battlefield.push(tifa);

    game.put_onto_battlefield(PlayerId::Two, cards::FOREST)
        .expect("a land arrives for the opponent");
    drain_pending(&mut game);

    let tifa = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == tifa_id)
        .expect("she is still there");
    assert_eq!((game.power(tifa), game.toughness(tifa)), (Some(1), Some(2)));
}

/// A land arriving puts a counter on a creature the trigger targets, and it
/// need not be Bill himself.
#[test]
fn bristly_bill_grows_a_creature_when_a_land_arrives() {
    let mut game = ready_game();
    game.battlefield.clear();
    let bill = creature(89_000, cards::BRISTLY_BILL_SPINE_SOWER, PlayerId::One);
    let bill_id = bill.card.id;
    game.battlefield.push(bill);
    let bears = creature(89_001, cards::GRIZZLY_BEARS, PlayerId::One);
    let bears_id = bears.card.id;
    game.battlefield.push(bears);
    game.players[0]
        .hand
        .push(card(89_002, cards::FOREST, PlayerId::One));

    let play = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::PlayLand { .. }))
        .expect("a land is playable");
    game.apply(PlayerId::One, play).expect("it is played");
    // The trigger asks which creature; take the Bears rather than Bill.
    let decision = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
        .expect("landfall asks for a target");
    let option = decision
        .options
        .iter()
        .find(|option| option.card.is_some_and(|(card, _)| card == bears_id))
        .expect("the Bears are a legal target");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option.id],
        },
    )
    .expect("the target is chosen");
    drain_pending(&mut game);

    let bears = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == bears_id)
        .expect("the Bears are still there");
    assert_eq!(bears.counters(CounterKind::PlusOnePlusOne), 1);
    let bill = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == bill_id)
        .expect("Bill is still there");
    assert_eq!(
        bill.counters(CounterKind::PlusOnePlusOne),
        0,
        "the counter went where it was pointed",
    );
}

/// Doubling reads each creature's own count, so a creature with none gains
/// none and every other one gains what it already had.
#[test]
fn bristly_bill_doubles_each_creatures_own_counters() {
    let mut game = ready_game();
    game.battlefield.clear();
    let mut bill = creature(89_010, cards::BRISTLY_BILL_SPINE_SOWER, PlayerId::One);
    bill.add_counters(CounterKind::PlusOnePlusOne, 1);
    let bill_id = bill.card.id;
    game.battlefield.push(bill);
    let mut bears = creature(89_011, cards::GRIZZLY_BEARS, PlayerId::One);
    bears.add_counters(CounterKind::PlusOnePlusOne, 3);
    let bears_id = bears.card.id;
    game.battlefield.push(bears);
    let plain = creature(89_012, cards::SAVANNAH_LIONS, PlayerId::One);
    let plain_id = plain.card.id;
    game.battlefield.push(plain);
    let mut theirs = creature(89_013, cards::GRIZZLY_BEARS, PlayerId::Two);
    theirs.add_counters(CounterKind::PlusOnePlusOne, 2);
    let theirs_id = theirs.card.id;
    game.battlefield.push(theirs);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 2);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == bill_id),
        )
        .expect("five mana pays for the doubling");
    game.apply(PlayerId::One, action).expect("it is activated");
    drain_pending(&mut game);

    let counters = |game: &Game, id| {
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .map(|permanent| permanent.counters(CounterKind::PlusOnePlusOne))
    };
    assert_eq!(counters(&game, bill_id), Some(2), "one becomes two");
    assert_eq!(counters(&game, bears_id), Some(6), "three becomes six");
    assert_eq!(counters(&game, plain_id), Some(0), "none stays none");
    assert_eq!(
        counters(&game, theirs_id),
        Some(2),
        "and a creature you do not control is untouched",
    );
}

/// Each draw is its own trigger, so a spell that draws three fires three
/// times -- and the two clauses point in opposite directions.
#[test]
fn sheoldred_pays_you_and_charges_them_per_card() {
    let mut game = ready_game();
    game.battlefield.clear();
    let sheoldred = creature(92_000, cards::SHEOLDRED_THE_APOCALYPSE, PlayerId::One);
    game.battlefield.push(sheoldred);
    for (seat, player) in [PlayerId::One, PlayerId::Two].into_iter().enumerate() {
        game.players[player.index()].library.clear();
        for id in 0..4 {
            let seat = u32::try_from(seat).expect("two seats fit in a u32");
            game.players[player.index()].library.push(card(
                92_010 + id + 10 * seat,
                cards::GRIZZLY_BEARS,
                player,
            ));
        }
    }
    let mine = game.players[0].life;
    let theirs = game.players[1].life;

    game.draw_cards(PlayerId::One, 3);
    drain_pending(&mut game);
    assert_eq!(
        game.players[0].life,
        mine + 6,
        "three of your draws is six life",
    );
    assert_eq!(game.players[1].life, theirs, "and none of theirs");

    game.draw_cards(PlayerId::Two, 2);
    drain_pending(&mut game);
    assert_eq!(
        game.players[1].life,
        theirs - 4,
        "two of their draws is four life the other way",
    );
    assert_eq!(game.players[0].life, mine + 6, "which pays you nothing");
}

/// A draw that was replaced never happened, so nothing fires for it.
#[test]
fn sheoldred_ignores_a_draw_that_never_lands() {
    let mut game = ready_game();
    game.battlefield.clear();
    let sheoldred = creature(92_030, cards::SHEOLDRED_THE_APOCALYPSE, PlayerId::One);
    game.battlefield.push(sheoldred);
    game.players[0].library.clear();
    let before = game.players[0].life;

    // An empty library draws nothing at all.
    game.draw_cards(PlayerId::One, 1);
    drain_pending(&mut game);

    assert_eq!(
        game.players[0].life, before,
        "no card reached the hand, so nothing triggered",
    );
}
