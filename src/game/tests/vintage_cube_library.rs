//! Searching, scrying, and the cards that rearrange a library.
//!
//! What these have in common is not a colour or a cost but a question: which
//! half of a partition the card lets you name, and where each half lands.

use super::search_and_reveal::stack_library;
use super::*;

/// Resolves whatever is on the stack, answering nothing.
fn resolve(game: &mut Game) {
    for _ in 0..8 {
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
}

#[test]
fn entomb_puts_the_found_card_into_the_graveyard() {
    let mut game = ready_game();
    game.players[PlayerId::One.index()].library.clear();
    stack_library(
        &mut game,
        &[(50_400, cards::SERRA_ANGEL), (50_401, cards::GRIZZLY_BEARS)],
    );
    let entomb = card(50_402, cards::ENTOMB, PlayerId::One);
    let entomb_id = entomb.id;
    game.players[PlayerId::One.index()].hand.push(entomb);
    game.players[PlayerId::One.index()].mana_pool.black = 1;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == entomb_id))
        .expect("Entomb is castable");
    game.apply(PlayerId::One, action).expect("it is cast");
    pass_priority_pair(&mut game);

    let decision = game.observe(PlayerId::One).decision.expect("a search");
    let angel = decision
        .options
        .iter()
        .find(|option| {
            option.card.is_some_and(|(_, characteristics)| {
                characteristics.card_definition() == Some(cards::SERRA_ANGEL)
            })
        })
        .expect("every card in the library is eligible")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![angel],
        },
    )
    .expect("the search is answered");

    assert!(
        game.players[PlayerId::One.index()]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::SERRA_ANGEL),
        "the found card goes to the graveyard rather than the hand",
    );
}

#[test]
fn vampiric_tutor_leaves_the_card_on_top_and_costs_two_life() {
    let mut game = ready_game();
    game.players[PlayerId::One.index()].library.clear();
    stack_library(
        &mut game,
        &[
            (50_500, cards::GRIZZLY_BEARS),
            (50_501, cards::SERRA_ANGEL),
            (50_502, cards::LIGHTNING_BOLT),
        ],
    );
    let tutor = card(50_503, cards::VAMPIRIC_TUTOR, PlayerId::One);
    let tutor_id = tutor.id;
    game.players[PlayerId::One.index()].hand.push(tutor);
    game.players[PlayerId::One.index()].mana_pool.black = 1;
    let life = game.players[PlayerId::One.index()].life;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == tutor_id))
        .expect("Vampiric Tutor is castable");
    game.apply(PlayerId::One, action).expect("it is cast");
    pass_priority_pair(&mut game);

    let decision = game.observe(PlayerId::One).decision.expect("a search");
    let angel = decision
        .options
        .iter()
        .find(|option| {
            option.card.is_some_and(|(_, characteristics)| {
                characteristics.card_definition() == Some(cards::SERRA_ANGEL)
            })
        })
        .expect("every card in the library is eligible")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![angel],
        },
    )
    .expect("the search is answered");
    resolve(&mut game);

    assert_eq!(
        game.players[PlayerId::One.index()]
            .library
            .last()
            .map(|card| card.definition),
        Some(cards::SERRA_ANGEL),
        "the found card survives the shuffle on top",
    );
    assert_eq!(game.players[PlayerId::One.index()].life, life - 2);
}

#[test]
fn mystical_tutor_offers_only_instants_and_sorceries() {
    let mut game = ready_game();
    game.players[PlayerId::One.index()].library.clear();
    stack_library(
        &mut game,
        &[
            (50_600, cards::GRIZZLY_BEARS),
            (50_601, cards::LIGHTNING_BOLT),
            (50_602, cards::ANCESTRAL_RECALL),
        ],
    );
    let tutor = card(50_603, cards::MYSTICAL_TUTOR, PlayerId::One);
    let tutor_id = tutor.id;
    game.players[PlayerId::One.index()].hand.push(tutor);
    game.players[PlayerId::One.index()].mana_pool.blue = 1;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == tutor_id))
        .expect("Mystical Tutor is castable");
    game.apply(PlayerId::One, action).expect("it is cast");
    pass_priority_pair(&mut game);

    let decision = game.observe(PlayerId::One).decision.expect("a search");
    let mut offered = decision
        .options
        .iter()
        .filter_map(|option| {
            option
                .card
                .and_then(|(_, characteristics)| characteristics.card_definition())
        })
        .collect::<Vec<_>>();
    offered.sort_unstable();
    let mut expected = vec![cards::LIGHTNING_BOLT, cards::ANCESTRAL_RECALL];
    expected.sort_unstable();
    assert_eq!(offered, expected, "the creature is not an eligible card");
}

/// Scry 2 with both cards kept is an arrangement, not just a filter: the two
/// go back on top in the order they were chosen, and the draw that follows
/// takes whichever was put there first.
#[test]
fn preordain_scries_two_and_lets_you_order_what_stays() {
    let mut game = ready_game();
    game.players[0].library.clear();
    stack_library(
        &mut game,
        &[
            (60_000, cards::LIGHTNING_BOLT),
            (60_001, cards::SERRA_ANGEL),
            (60_002, cards::SAVANNAH_LIONS),
        ],
    );
    let preordain = card(60_003, cards::PREORDAIN, PlayerId::One);
    let preordain_id = preordain.id;
    game.players[0].hand.push(preordain);
    game.players[0].mana_pool.blue = 1;
    game.apply(
        PlayerId::One,
        cast_action(preordain_id, Vec::new(), Vec::new(), 0),
    )
    .expect("it is cast");
    pass_priority_pair(&mut game);

    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the scry looks at two");
    assert_eq!(
        decision
            .options
            .iter()
            .map(|option| option.label.clone())
            .collect::<Vec<_>>(),
        vec!["Lightning Bolt".to_owned(), "Serra Angel".to_owned()],
        "only the top two are looked at",
    );

    // Keep both, naming the Angel first so it ends up on top of the Bolt.
    let angel = decision
        .options
        .iter()
        .find(|option| option.label == "Serra Angel")
        .expect("offered")
        .id;
    let bolt = decision
        .options
        .iter()
        .find(|option| option.label == "Lightning Bolt")
        .expect("offered")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![angel, bolt],
        },
    )
    .expect("both may stay on top");

    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|card| card.definition == cards::SERRA_ANGEL),
        "the draw takes the card put on top first",
    );
    assert_eq!(
        game.players[0]
            .library
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::SAVANNAH_LIONS, cards::LIGHTNING_BOLT],
        "and the other stays above what was never looked at",
    );
}

/// Sending both to the bottom is the other end of the same choice.
#[test]
fn preordain_can_bury_both_cards_it_looked_at() {
    let mut game = ready_game();
    game.players[0].library.clear();
    stack_library(
        &mut game,
        &[
            (60_100, cards::LIGHTNING_BOLT),
            (60_101, cards::SERRA_ANGEL),
            (60_102, cards::SAVANNAH_LIONS),
        ],
    );
    let preordain = card(60_103, cards::PREORDAIN, PlayerId::One);
    let preordain_id = preordain.id;
    game.players[0].hand.push(preordain);
    game.players[0].mana_pool.blue = 1;
    game.apply(
        PlayerId::One,
        cast_action(preordain_id, Vec::new(), Vec::new(), 0),
    )
    .expect("it is cast");
    pass_priority_pair(&mut game);

    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the scry looks at two");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: Vec::new(),
        },
    )
    .expect("keeping nothing is allowed");

    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|card| card.definition == cards::SAVANNAH_LIONS),
        "the draw reaches past both buried cards",
    );
    assert_eq!(
        game.players[0].library.len(),
        2,
        "and both are still in the library, at the bottom",
    );
}

/// The sacrifice is a cost, so it happens on casting and cannot be dodged by
/// answering the spell. What comes back is any land, which is the point: a
/// Forest becomes whatever the deck actually wanted.
#[test]
fn crop_rotation_trades_a_land_on_the_battlefield_for_any_land_in_the_library() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].library.clear();
    stack_library(
        &mut game,
        &[
            (63_000, cards::LIGHTNING_BOLT),
            (63_001, cards::GAEAS_CRADLE),
            (63_002, cards::TAIGA),
        ],
    );
    let forest = creature(63_003, cards::FOREST, PlayerId::One);
    let forest_id = forest.card.id;
    game.battlefield.push(forest);

    let rotation = card(63_004, cards::CROP_ROTATION, PlayerId::One);
    let rotation_id = rotation.id;
    game.players[0].hand.push(rotation);
    game.players[0].mana_pool.green = 1;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::CastSpell { card, sacrifices, .. }
                if *card == rotation_id && sacrifices.contains(&forest_id))
        })
        .expect("the land on the battlefield pays for it");
    game.apply(PlayerId::One, action).expect("it is cast");
    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != forest_id),
        "the sacrifice is a cost, paid on casting",
    );
    pass_priority_pair(&mut game);

    let decision = game.observe(PlayerId::One).decision.expect("a search");
    let mut offered = decision
        .options
        .iter()
        .filter_map(|option| {
            option
                .card
                .and_then(|(_, characteristics)| characteristics.card_definition())
        })
        .collect::<Vec<_>>();
    offered.sort_unstable();
    let mut lands = vec![cards::GAEAS_CRADLE, cards::TAIGA];
    lands.sort_unstable();
    assert_eq!(offered, lands, "any land card, and only lands");

    let cradle = decision
        .options
        .iter()
        .find(|option| {
            option.card.is_some_and(|(_, characteristics)| {
                characteristics.card_definition() == Some(cards::GAEAS_CRADLE)
            })
        })
        .expect("offered")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![cradle],
        },
    )
    .expect("the search is answered");

    let found = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::GAEAS_CRADLE)
        .expect("it arrived on the battlefield");
    assert!(!found.tapped, "and it arrives untapped");
}

/// With no land to sacrifice there is no way to pay, so the spell is not
/// castable at all.
#[test]
fn crop_rotation_needs_a_land_to_give_up() {
    let mut game = ready_game();
    game.battlefield.clear();
    let rotation = card(63_100, cards::CROP_ROTATION, PlayerId::One);
    let rotation_id = rotation.id;
    game.players[0].hand.push(rotation);
    game.players[0].mana_pool.green = 1;

    assert!(
        !game
            .legal_actions(PlayerId::One)
            .iter()
            .any(|action| matches!(action, Action::CastSpell { card, .. } if *card == rotation_id)),
        "the mana alone does not pay for it",
    );
}

/// Two bounds at once: an instant or a sorcery, and cheap. Demonic Tutor sits
/// exactly on the line at two and is eligible; Wrath of God is the same card
/// type and one too expensive.
#[test]
fn spellseeker_finds_a_cheap_instant_or_sorcery_and_nothing_else() {
    let mut game = ready_game();
    game.players[0].library.clear();
    stack_library(
        &mut game,
        &[
            (72_000, cards::LIGHTNING_BOLT),
            (72_001, cards::DEMONIC_TUTOR),
            (72_002, cards::WRATH_OF_GOD),
            (72_003, cards::SERRA_ANGEL),
            (72_004, cards::FOREST),
        ],
    );

    game.put_onto_battlefield(PlayerId::One, cards::SPELLSEEKER)
        .expect("cataloged");

    // The search is optional; the last option accepts it.
    let offer = loop {
        if let Some(decision) = game.observe(PlayerId::One).decision {
            break decision;
        }
        let player = game.priority;
        assert!(
            game.apply(player, Action::PassPriority).is_ok(),
            "the enters trigger is waiting",
        );
    };
    let accept = offer.options.last().expect("accepting is offered").id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: offer.id,
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
    let mut expected = vec![cards::LIGHTNING_BOLT, cards::DEMONIC_TUTOR];
    expected.sort_unstable();
    assert_eq!(
        offered, expected,
        "a four-mana sorcery, a creature and a land are all out",
    );

    let tutor = search
        .options
        .iter()
        .find(|option| {
            option.card.is_some_and(|(_, characteristics)| {
                characteristics.card_definition() == Some(cards::DEMONIC_TUTOR)
            })
        })
        .expect("offered")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: search.id,
            options: vec![tutor],
        },
    )
    .expect("the search is answered");

    assert!(
        game.players[0]
            .hand
            .iter()
            .any(|card| card.definition == cards::DEMONIC_TUTOR),
        "the found card goes to hand",
    );
}

/// Five cards seen, exactly two taken, and the other three go under the
/// library rather than to the graveyard.
#[test]
fn stock_up_takes_two_of_five_and_buries_the_rest() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].library.clear();
    // Pushed last is drawn first, so this is the top five in reverse.
    let bottom = card(91_000, cards::BLACK_LOTUS, PlayerId::One);
    let bottom_id = bottom.id;
    game.players[0].library.push(bottom);
    for id in 91_001..91_006 {
        game.players[0]
            .library
            .push(card(id, cards::GRIZZLY_BEARS, PlayerId::One));
    }
    let stock_up = card(91_010, cards::STOCK_UP, PlayerId::One);
    let stock_up_id = stock_up.id;
    game.players[0].hand.push(stock_up);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
    let held = game.players[0].hand.len();

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == stock_up_id))
        .expect("three mana casts it");
    game.apply(PlayerId::One, cast).expect("it is cast");
    pass_until_decision(&mut game);

    let look = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
        .expect("the dig asks which cards to take");
    assert_eq!(look.options.len(), 5, "five cards are seen");
    assert_eq!(
        (look.minimum, look.maximum),
        (2, 2),
        "and exactly two are taken",
    );
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: look.id,
            options: look
                .options
                .iter()
                .take(2)
                .map(|option| option.id)
                .collect(),
        },
    )
    .expect("taking two is the only legal answer");
    drain_pending(&mut game);

    // One card left the hand for Stock Up and two came back.
    assert_eq!(game.players[0].hand.len(), held + 1);
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .all(|card| card.definition != cards::GRIZZLY_BEARS),
        "the rest are not milled",
    );
    assert_eq!(
        game.players[0].library.len(),
        4,
        "one card was already under them, and three went back beneath it",
    );
    // The library is stored bottom-first, so the deepest card is the one at
    // the front: the three that went back are now beneath the card that was
    // already at the bottom, and it will be drawn before any of them.
    assert!(
        game.players[0]
            .library
            .iter()
            .position(|card| card.id == bottom_id)
            .is_some_and(|position| position == 3),
        "the rest went under what was already at the bottom",
    );
}

/// X=0 is the mode that matters: the permanent lands on top of its owner's
/// library, where they will draw it instead of something else.
#[test]
fn unexpectedly_absent_for_zero_puts_it_on_top() {
    let mut game = ready_game();
    game.battlefield.clear();
    let bears = creature(94_000, cards::GRIZZLY_BEARS, PlayerId::Two);
    let bears_id = bears.card.id;
    game.battlefield.push(bears);
    game.players[1].library.clear();
    for id in 94_001..94_004 {
        game.players[1]
            .library
            .push(card(id, cards::FOREST, PlayerId::Two));
    }
    let absent = card(94_010, cards::UNEXPECTEDLY_ABSENT, PlayerId::One);
    let absent_id = absent.id;
    game.players[0].hand.push(absent);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 2);

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => *card == absent_id && choices.x() == 0,
            _ => false,
        })
        .expect("two mana casts it for X=0");
    game.apply(PlayerId::One, cast).expect("it is cast");
    pass_until_decision(&mut game);
    drain_pending(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == bears_id),
        "the permanent leaves the battlefield",
    );
    // The library is stored bottom-first, so the top is the last entry.
    assert_eq!(
        game.players[1].library.last().map(|card| card.definition),
        Some(cards::GRIZZLY_BEARS),
        "and lands on top, costing its controller their next draw",
    );
}

/// A larger X buries it that many cards down, and an X past the end of the
/// library puts it on the bottom.
#[test]
fn unexpectedly_absent_buries_it_x_cards_deep() {
    for (x, expected_from_top) in [(2_u16, 2_usize), (9, 3)] {
        let mut game = ready_game();
        game.battlefield.clear();
        let bears = creature(94_020, cards::GRIZZLY_BEARS, PlayerId::Two);
        game.battlefield.push(bears);
        game.players[1].library.clear();
        for id in 94_021..94_024 {
            game.players[1]
                .library
                .push(card(id, cards::FOREST, PlayerId::Two));
        }
        let absent = card(94_030, cards::UNEXPECTEDLY_ABSENT, PlayerId::One);
        let absent_id = absent.id;
        game.players[0].hand.push(absent);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 2);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, x);

        let cast = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| match action {
                Action::CastSpell { card, choices, .. } => *card == absent_id && choices.x() == x,
                _ => false,
            })
            .unwrap_or_else(|| panic!("X={x} is affordable"));
        game.apply(PlayerId::One, cast).expect("it is cast");
        pass_until_decision(&mut game);
        drain_pending(&mut game);

        let library = &game.players[1].library;
        let position = library
            .iter()
            .position(|card| card.definition == cards::GRIZZLY_BEARS)
            .expect("the permanent is in the library");
        assert_eq!(
            library.len() - 1 - position,
            expected_from_top,
            "X={x} should bury it {expected_from_top} cards from the top",
        );
    }
}

/// Five cards kept from library and graveyard together, everything else
/// exiled, and the order they were picked is the order they are drawn.
#[test]
fn doomsday_stacks_five_cards_and_exiles_everything_else() {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].library.clear();
    game.players[0].graveyard.clear();
    for id in 97_000..97_010 {
        game.players[0]
            .library
            .push(card(id, cards::GRIZZLY_BEARS, PlayerId::One));
    }
    // One card in the graveyard, to prove the search reaches both zones.
    game.players[0]
        .graveyard
        .push(card(97_020, cards::BLACK_LOTUS, PlayerId::One));
    let doomsday = card(97_030, cards::DOOMSDAY, PlayerId::One);
    let doomsday_id = doomsday.id;
    game.players[0].hand.push(doomsday);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Black, 3);
    game.players[0].life = 20;

    let cast = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == doomsday_id))
        .expect("three black mana casts it");
    game.apply(PlayerId::One, cast).expect("it is cast");
    pass_until_decision(&mut game);

    let search = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
        .expect("the search asks which five to keep");
    assert_eq!(
        search.options.len(),
        11,
        "ten cards in the library and one in the graveyard, offered together",
    );
    assert_eq!((search.minimum, search.maximum), (5, 5));
    // Pick the graveyard card first, so it must end up on top.
    let lotus = search
        .options
        .iter()
        .find(|option| option.label.contains("Lotus"))
        .expect("the graveyard card is on offer");
    let mut chosen = vec![lotus.id];
    chosen.extend(
        search
            .options
            .iter()
            .filter(|option| option.id != lotus.id)
            .take(4)
            .map(|option| option.id),
    );
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: search.id,
            options: chosen,
        },
    )
    .expect("keeping five is the only legal answer");
    drain_pending(&mut game);

    assert_eq!(game.players[0].library.len(), 5, "a five-card library");
    // Doomsday itself is still on the stack while it resolves, so it was
    // never searched and is not exiled -- it goes to the graveyard after.
    assert_eq!(
        game.players[0]
            .graveyard
            .iter()
            .map(|card| card.definition)
            .collect::<Vec<_>>(),
        vec![cards::DOOMSDAY],
    );
    assert_eq!(
        game.players[0].exile.len(),
        6,
        "the other six cards are exiled",
    );
    // The library is stored bottom-first, so the last entry is drawn next.
    assert_eq!(
        game.players[0].library.last().map(|card| card.definition),
        Some(cards::BLACK_LOTUS),
        "the card chosen first is the card drawn first",
    );
    assert_eq!(game.players[0].life, 10, "and half the life is gone");
}
