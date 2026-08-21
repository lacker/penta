//! Bountiful Landscape: a land that taps for nothing anyone wants, fetches a
//! tapped basic, and cycles when neither is what the hand needs.

use super::*;

/// Player One with a Landscape on the battlefield since last turn and
/// `library` stacked underneath.
fn staged(library: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].library.clear();
    for definition in library {
        let card = game
            .build_zone(PlayerId::One, &[*definition])
            .expect("cataloged")
            .into_iter()
            .next()
            .expect("one card");
        game.players[0].library.push(card);
    }
    let landscape = game
        .put_onto_battlefield(PlayerId::One, cards::BOUNTIFUL_LANDSCAPE)
        .expect("cataloged");
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [1, 1];
    drain_pending(&mut game);
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, landscape)
}

fn settle(game: &mut Game) {
    for _ in 0..24 {
        if game.observe(PlayerId::One).decision.is_some() {
            return;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    game.check_state_based_actions();
}

/// It taps for one colourless and nothing else.
#[test]
fn it_taps_for_colorless() {
    let (mut game, landscape) = staged(&[]);

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateManaAbility { source, .. } if *source == landscape)
        })
        .expect("the mana ability is offered");
    let Action::ActivateManaAbility { color, .. } = action else {
        unreachable!("the search matched a mana ability")
    };
    assert_eq!(color, ManaColor::Colorless, "colourless, and only that");

    game.apply(PlayerId::One, action).expect("it taps");
    assert_eq!(game.players[0].mana_pool.colorless, 1, "one {{C}}");
}

/// Sacrificing it finds a basic of one of the three named types, tapped.
#[test]
fn it_fetches_a_named_basic_tapped() {
    let (mut game, landscape) = staged(&[cards::PLAINS, cards::ISLAND]);

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == landscape)
        })
        .expect("the fetch is offered");
    game.apply(PlayerId::One, action).expect("it activates");
    settle(&mut game);

    let decision = game
        .observe(PlayerId::One)
        .decision
        .expect("the search asks what to find");
    let offered: Vec<CardDefinitionId> = decision
        .options
        .iter()
        .filter_map(|option| {
            option
                .card
                .and_then(|(_, characteristics)| characteristics.card_definition())
        })
        .collect();
    assert!(
        offered.contains(&cards::ISLAND),
        "an Island is one of the three: {offered:?}",
    );
    assert!(
        !offered.contains(&cards::PLAINS),
        "a Plains is not: {offered:?}",
    );

    let island = decision
        .options
        .iter()
        .find(|option| {
            option.card.is_some_and(|(_, characteristics)| {
                characteristics.card_definition() == Some(cards::ISLAND)
            })
        })
        .expect("the Island is offered")
        .id;
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![island],
        },
    )
    .expect("finding it is legal");
    settle(&mut game);

    let found = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::ISLAND)
        .expect("the Island arrived");
    assert!(found.tapped, "tapped, which is the whole cost of the cycle");
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == landscape),
        "and the Landscape sacrificed itself to do it",
    );
}

/// "Basic" is a real word: a dual with the right type is not found.
#[test]
fn a_nonbasic_with_the_right_type_is_not_found() {
    let (mut game, landscape) = staged(&[cards::SACRED_FOUNDRY, cards::FOREST]);

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == landscape)
        })
        .expect("the fetch is offered");
    game.apply(PlayerId::One, action).expect("it activates");
    settle(&mut game);

    let offered: Vec<CardDefinitionId> = game
        .observe(PlayerId::One)
        .decision
        .expect("the search asks")
        .options
        .iter()
        .filter_map(|option| {
            option
                .card
                .and_then(|(_, characteristics)| characteristics.card_definition())
        })
        .collect();
    assert!(
        offered.contains(&cards::FOREST),
        "the basic Forest is there: {offered:?}",
    );
    assert!(
        !offered.contains(&cards::SACRED_FOUNDRY),
        "a Sacred Foundry is a Mountain, and is not basic: {offered:?}",
    );
}

/// Three coloured mana out of hand draws a card instead.
#[test]
fn it_cycles_from_hand() {
    let (mut game, _landscape) = staged(&[cards::FOREST, cards::ISLAND]);
    let card = game
        .build_zone(PlayerId::One, &[cards::BOUNTIFUL_LANDSCAPE])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let held = card.id;
    game.players[0].hand.push(card);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == held))
        .expect("cycling is offered from hand");
    game.apply(PlayerId::One, action).expect("it cycles");
    settle(&mut game);

    assert_eq!(
        game.players[0].hand.len(),
        1,
        "one card drawn for the one discarded"
    );
    assert!(
        game.players[0]
            .graveyard
            .iter()
            .any(|card| card.definition == cards::BOUNTIFUL_LANDSCAPE),
        "and the Landscape is in the graveyard",
    );
}

/// Cycling costs three specific colours: two of them is not enough.
#[test]
fn cycling_is_not_offered_without_all_three_colors() {
    let (mut game, _landscape) = staged(&[cards::FOREST]);
    let card = game
        .build_zone(PlayerId::One, &[cards::BOUNTIFUL_LANDSCAPE])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let held = card.id;
    game.players[0].hand.push(card);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Green, 2);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);

    assert!(
        !game.legal_actions(PlayerId::One).iter().any(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == held)
        ),
        "green, green, and blue does not pay {{G}}{{U}}{{R}}",
    );
}
