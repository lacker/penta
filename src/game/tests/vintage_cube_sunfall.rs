//! Sunfall: a wrath that exiles, and the first two-faced token in the
//! catalog.

use super::*;

/// Player One holding a Sunfall with five mana up, `mine` and `theirs` on
/// the battlefield.
fn staged(mine: &[CardDefinitionId], theirs: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    for definition in mine {
        game.put_onto_battlefield(PlayerId::One, *definition)
            .expect("cataloged");
    }
    for definition in theirs {
        game.put_onto_battlefield(PlayerId::Two, *definition)
            .expect("cataloged");
    }
    let card = game
        .build_zone(PlayerId::One, &[cards::SUNFALL])
        .expect("cataloged")
        .into_iter()
        .next()
        .expect("one card");
    let sunfall = card.id;
    game.players[0].hand.push(card);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [1, 1];
    game.add_unrestricted_mana(PlayerId::One, ManaColor::White, 2);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 3);
    drain_pending(&mut game);
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, sunfall)
}

fn settle(game: &mut Game) {
    for _ in 0..24 {
        if game.observe(PlayerId::One).decision.is_some()
            || game.observe(PlayerId::Two).decision.is_some()
        {
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

fn cast(game: &mut Game, sunfall: GameObjectId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == sunfall))
        .expect("five mana casts it");
    game.apply(PlayerId::One, action).expect("it casts");
    settle(game);
}

fn incubator(game: &Game) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| is_token_with(permanent, tokens::incubator()))
        .expect("an Incubator token was made")
}

/// Every creature goes, whoever controls it, and to exile rather than a
/// graveyard.
#[test]
fn it_exiles_every_creature_on_both_sides() {
    let (mut game, sunfall) = staged(
        &[cards::SAVANNAH_LIONS, cards::SERRA_ANGEL],
        &[cards::SAVANNAH_LIONS],
    );

    cast(&mut game, sunfall);

    assert!(
        !game
            .battlefield
            .iter()
            .any(
                |permanent| permanent.card.definition == cards::SAVANNAH_LIONS
                    || permanent.card.definition == cards::SERRA_ANGEL
            ),
        "no creature is left",
    );
    assert_eq!(
        game.players[0].exile.len() + game.players[1].exile.len(),
        3,
        "all three are in exile",
    );
    assert!(
        game.players[0].graveyard.len() == 1
            && game.players[0].graveyard[0].definition == cards::SUNFALL,
        "and only the Sunfall itself went to a graveyard",
    );
}

/// Incubate X counts what was exiled, and the counters are on the token as
/// it arrives.
#[test]
fn the_incubator_arrives_with_one_counter_per_creature() {
    let (mut game, sunfall) = staged(
        &[cards::SAVANNAH_LIONS, cards::SERRA_ANGEL],
        &[cards::SAVANNAH_LIONS, cards::PLAINS],
    );

    cast(&mut game, sunfall);

    assert_eq!(
        incubator(&game).counters(CounterKind::PlusOnePlusOne),
        3,
        "three creatures, three counters -- the land is not one of them",
    );
    assert_eq!(
        incubator(&game).controller,
        PlayerId::One,
        "and the caster gets it",
    );
}

/// An empty board still makes a token, with nothing on it.
#[test]
fn an_empty_board_still_incubates_zero() {
    let (mut game, sunfall) = staged(&[], &[]);

    cast(&mut game, sunfall);

    assert_eq!(
        incubator(&game).counters(CounterKind::PlusOnePlusOne),
        0,
        "incubate zero is still an Incubator token",
    );
}

/// The front face is an artifact and nothing more: it does not block, and it
/// is not a creature until it turns over.
#[test]
fn the_front_face_is_not_a_creature() {
    let (mut game, sunfall) = staged(&[cards::SAVANNAH_LIONS], &[]);
    cast(&mut game, sunfall);

    let token = incubator(&game);
    assert!(
        game.permanent_types(token)
            .is_some_and(|types| types.contains(CardType::Artifact)),
        "an artifact",
    );
    assert!(
        !game
            .permanent_types(token)
            .is_some_and(CardTypeSet::is_creature),
        "and not yet a creature",
    );
}

/// Two mana turns it over into a body the size of its counters.
#[test]
fn two_mana_transforms_it_into_a_phyrexian() {
    let (mut game, sunfall) = staged(
        &[cards::SAVANNAH_LIONS, cards::SERRA_ANGEL],
        &[cards::SAVANNAH_LIONS],
    );
    cast(&mut game, sunfall);
    let token = incubator(&game).card.id;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == token))
        .expect("two mana transforms it");
    game.apply(PlayerId::One, action).expect("it activates");
    settle(&mut game);

    let turned = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == token)
        .expect("the token is still there");
    assert!(
        game.permanent_types(turned)
            .is_some_and(CardTypeSet::is_creature),
        "now a creature",
    );
    assert_eq!(
        game.power(turned),
        Some(3),
        "a 0/0 with three +1/+1 counters still on it",
    );
}

/// A zero-counter Incubator that turns over is a 0/0 and dies to the rules.
#[test]
fn transforming_an_empty_incubator_kills_it() {
    let (mut game, sunfall) = staged(&[], &[]);
    cast(&mut game, sunfall);
    let token = incubator(&game).card.id;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::ActivateAbility { source, .. } if *source == token))
        .expect("two mana transforms it");
    game.apply(PlayerId::One, action).expect("it activates");
    settle(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == token),
        "a 0/0 with no counters is put into a graveyard by state-based actions",
    );
}
