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

/// "Exile all creatures" rather than destroy them, which is the whole reason
/// the deck pays five for it: a Darksteel Myr shrugs off a Wrath and goes to
/// exile all the same, and it counts toward X while it goes.
#[test]
fn indestructible_creatures_go_too_and_still_count() {
    let (mut game, sunfall) = staged(&[cards::DARKSTEEL_MYR], &[cards::SAVANNAH_LIONS]);

    cast(&mut game, sunfall);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.definition == cards::DARKSTEEL_MYR),
        "indestructible is no answer to exile",
    );
    assert_eq!(
        incubator(&game).counters(CounterKind::PlusOnePlusOne),
        2,
        "and it was one of the two counted",
    );
}

/// A token exiled ceases to exist rather than joining anybody's exile zone,
/// and it was still a creature exiled this way: X counts it.
#[test]
fn a_token_ceases_to_exist_and_still_counts_for_x() {
    let (mut game, sunfall) = staged(&[cards::SAVANNAH_LIONS], &[]);
    game.create_token(
        PlayerId::Two,
        crate::card::TokenCharacteristics::creature(&["Bear"], &[ManaColor::Green], 2, 2),
    );
    drain_pending(&mut game);
    game.priority = PlayerId::One;

    cast(&mut game, sunfall);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.definition == ObjectKind::Token
                && permanent.controller == PlayerId::Two),
        "the token is gone",
    );
    assert!(
        game.players[1].exile.is_empty(),
        "and it went nowhere: a token that leaves ceases to exist",
    );
    assert_eq!(
        incubator(&game).counters(CounterKind::PlusOnePlusOne),
        2,
        "but it was a creature exiled this way, so X counted it",
    );
}

/// "All creatures" and nothing else: a planeswalker and an enchantment are
/// left standing, however much the board would rather they were not.
#[test]
fn it_leaves_the_noncreatures_where_they_stand() {
    let (mut game, sunfall) = staged(&[cards::SAVANNAH_LIONS], &[]);
    let walker = game
        .put_onto_battlefield(PlayerId::Two, cards::JACE_THE_MIND_SCULPTOR)
        .expect("cataloged");
    let moat = game
        .put_onto_battlefield(PlayerId::Two, cards::MOAT)
        .expect("cataloged");
    drain_pending(&mut game);
    game.priority = PlayerId::One;

    cast(&mut game, sunfall);

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == walker),
        "the planeswalker stayed",
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == moat),
        "and so did the enchantment",
    );
    assert_eq!(
        incubator(&game).counters(CounterKind::PlusOnePlusOne),
        1,
        "one creature was all there was to exile",
    );
}

/// What it turns into, in full: a Phyrexian artifact creature whose printed
/// body is 0/0 and whose size is entirely the counters it was incubated
/// with. And the transform ability is printed on the front face alone, so
/// once it is over there is nothing left to activate -- it does not turn
/// back.
#[test]
fn the_back_face_is_a_phyrexian_artifact_that_cannot_turn_back() {
    let (mut game, sunfall) = staged(&[cards::SAVANNAH_LIONS], &[cards::SERRA_ANGEL]);
    cast(&mut game, sunfall);
    let token = incubator(&game).card.id;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 4);

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
    let types = game
        .permanent_types(turned)
        .expect("it is on the battlefield");
    assert!(
        types.contains(CardType::Artifact) && types.is_creature(),
        "an artifact creature, not merely a creature: {types:?}",
    );
    assert!(
        game.effective_subtypes(turned)
            .contains(crate::card::Subtype::named("Phyrexian")),
        "and a Phyrexian",
    );
    assert_eq!(
        (game.power(turned), game.toughness(turned)),
        (Some(2), Some(2)),
        "a 0/0 body carrying the two counters it was incubated with",
    );

    assert!(
        !game.legal_actions(PlayerId::One).iter().any(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == token)
        ),
        "the {{2}} belongs to the front face, so there is no turning back",
    );
}
