//! Oko, Thief of Crowns: a three-mana planeswalker that turns anything into
//! a 3/3 Elk and then trades it for something better.

use super::*;

fn staged(board: &[(CardDefinitionId, PlayerId)]) -> (Game, GameObjectId, Vec<GameObjectId>) {
    let mut game = ready_game();
    game.battlefield.clear();
    let mut ids = Vec::new();
    for (index, (definition, controller)) in board.iter().enumerate() {
        let permanent = creature(
            97_000 + u32::try_from(index).expect("few permanents"),
            *definition,
            *controller,
        );
        ids.push(permanent.card.id);
        game.battlefield.push(permanent);
    }
    let oko = game
        .put_onto_battlefield(PlayerId::One, cards::OKO_THIEF_OF_CROWNS)
        .expect("cataloged");
    drain_pending(&mut game);
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, oko, ids)
}

fn loyalty_action(game: &Game, oko: GameObjectId, wanted: &[GameObjectId]) -> Option<Action> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == oko
                    && wanted.iter().all(|wanted| {
                        targets
                            .iter()
                            .flat_map(crate::casting::TargetSelection::targets)
                            .any(|chosen| *chosen == Target::Permanent(*wanted))
                    })
                    && targets
                        .iter()
                        .flat_map(crate::casting::TargetSelection::targets)
                        .count()
                        == wanted.len()
            }
            _ => false,
        })
}

fn permanent(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("it is on the battlefield")
}

/// The Elk maker: a Mox becomes a 3/3 green creature with nothing printed
/// on it, and stays one.
#[test]
fn it_turns_an_artifact_into_an_elk() {
    let (mut game, oko, ids) = staged(&[(cards::MOX_JET, PlayerId::One)]);
    let mox = ids[0];

    let elkify = loyalty_action(&game, oko, &[mox]).expect("+1 names it");
    game.apply(PlayerId::One, elkify).expect("it activates");
    drain_pending(&mut game);

    let elk = permanent(&game, mox);
    assert_eq!((game.power(elk), game.toughness(elk)), (Some(3), Some(3)));
    assert!(
        game.effective_subtypes(elk)
            .contains(crate::card::Subtype::named("Elk"))
    );
    assert!(
        game.permanent_types(elk)
            .is_some_and(|types| types.contains(CardType::Creature)),
    );
    assert!(
        game.mana_ability_activations(elk).is_empty(),
        "the Mox has lost the ability that made it worth playing",
    );

    // It is not an until-end-of-turn effect: the Elk is still an Elk after
    // cleanup.
    game.cleanup();
    let elk = permanent(&game, mox);
    assert_eq!((game.power(elk), game.toughness(elk)), (Some(3), Some(3)));
}

/// The +2 makes Food.
#[test]
fn it_makes_food() {
    let (mut game, oko, _) = staged(&[]);

    let food = loyalty_action(&game, oko, &[]).expect("+2 needs no target");
    game.apply(PlayerId::One, food).expect("it activates");
    drain_pending(&mut game);

    assert!(game.battlefield.iter().any(|permanent| {
        game.effective_subtypes(permanent)
            .contains(crate::card::Subtype::named("Food"))
    }),);
}

/// The ultimate swaps something of yours for something small of theirs.
#[test]
fn the_ultimate_exchanges_control() {
    let (mut game, oko, ids) = staged(&[
        (cards::MOX_JET, PlayerId::One),
        (cards::GRIZZLY_BEARS, PlayerId::Two),
        (cards::SERRA_ANGEL, PlayerId::Two),
    ]);
    let (mox, bears, angel) = (ids[0], ids[1], ids[2]);
    if let Some(walker) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == oko)
    {
        walker.set_counters(CounterKind::Loyalty, 5);
    }

    assert!(
        loyalty_action(&game, oko, &[mox, angel]).is_none(),
        "a four-power Angel is out of reach",
    );
    let exchange = loyalty_action(&game, oko, &[mox, bears]).expect("the bears are small enough");
    game.apply(PlayerId::One, exchange).expect("it activates");
    drain_pending(&mut game);

    assert_eq!(permanent(&game, mox).controller, PlayerId::Two);
    assert_eq!(permanent(&game, bears).controller, PlayerId::One);
}

/// "It's just a green Elk. The creature keeps any supertypes it has, but
/// loses any other card types." An artifact creature comes out the other
/// side green, no longer an artifact, and no longer a Myr; a legend comes
/// out still legendary.
#[test]
fn the_elk_keeps_its_supertype_and_loses_everything_else() {
    let (mut game, oko, ids) = staged(&[(cards::MYR_BATTLESPHERE, PlayerId::One)]);
    let ballista = ids[0];

    let elkify = loyalty_action(&game, oko, &[ballista]).expect("+1 names it");
    game.apply(PlayerId::One, elkify).expect("it activates");
    drain_pending(&mut game);

    let elk = permanent(&game, ballista);
    let types = game.permanent_types(elk).expect("it has types");
    assert!(types.contains(CardType::Creature), "a creature");
    assert!(
        !types.contains(CardType::Artifact),
        "and not an artifact any more",
    );
    let colors = game.permanent_colors(elk);
    for color in ManaColor::COLORS {
        let index = color.color_index().expect("a colour has an index");
        assert_eq!(
            colors[index],
            color == ManaColor::Green,
            "green and nothing else, but {color:?} is {}",
            colors[index],
        );
    }
    let subtypes = game.effective_subtypes(elk);
    assert!(subtypes.contains(crate::card::Subtype::named("Elk")));
    assert!(
        !subtypes.contains(crate::card::Subtype::named("Myr")),
        "the creature types it had are gone: {subtypes:?}",
    );

    // A legend elked is a legendary Elk.
    let (mut game, oko, ids) = staged(&[(cards::THALIA_GUARDIAN_OF_THRABEN, PlayerId::One)]);
    let thalia = ids[0];
    let elkify = loyalty_action(&game, oko, &[thalia]).expect("+1 names her");
    game.apply(PlayerId::One, elkify).expect("it activates");
    drain_pending(&mut game);

    let elk = permanent(&game, thalia);
    assert_eq!((game.power(elk), game.toughness(elk)), (Some(3), Some(3)));
    assert!(
        game.effective_rules(elk)
            .expect("an Elk has rules")
            .has_supertype(CardSupertype::Legendary),
        "the supertype survives what the types do not",
    );
}

/// "Nonlethal damage dealt to a creature may become lethal if Oko's second
/// ability changes its toughness during that turn." Three damage on a 4/4
/// is a scratch until the 4/4 is an Elk.
#[test]
fn damage_already_marked_can_become_lethal() {
    let (mut game, oko, ids) = staged(&[(cards::SERRA_ANGEL, PlayerId::Two)]);
    let angel = ids[0];
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == angel)
    {
        permanent.damage = 3;
    }
    game.check_state_based_actions();
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == angel),
        "three damage does not kill a 4/4",
    );

    let elkify = loyalty_action(&game, oko, &[angel]).expect("+1 names their Angel");
    game.apply(PlayerId::One, elkify).expect("it activates");
    drain_pending(&mut game);
    game.check_state_based_actions();

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == angel),
        "but it kills a 3/3 that was already carrying it",
    );
}

/// "Any counters that change its power and/or toughness" apply whenever they
/// arrived: a counter put on before the Elk was an Elk still counts
/// afterwards, and so does one put on after.
#[test]
fn counters_still_apply_over_the_elk() {
    let (mut game, oko, ids) = staged(&[(cards::GRIZZLY_BEARS, PlayerId::One)]);
    let bears = ids[0];
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == bears)
    {
        permanent.add_counters(CounterKind::PlusOnePlusOne, 1);
    }

    let elkify = loyalty_action(&game, oko, &[bears]).expect("+1 names it");
    game.apply(PlayerId::One, elkify).expect("it activates");
    drain_pending(&mut game);

    assert_eq!(
        (
            game.power(permanent(&game, bears)),
            game.toughness(permanent(&game, bears))
        ),
        (Some(4), Some(4)),
        "a 3/3 base and the counter it was already carrying",
    );

    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == bears)
    {
        permanent.add_counters(CounterKind::PlusOnePlusOne, 1);
    }
    assert_eq!(
        (
            game.power(permanent(&game, bears)),
            game.toughness(permanent(&game, bears))
        ),
        (Some(5), Some(5)),
        "and one put on afterwards counts the same",
    );
}

/// "Oko's second ability may target a permanent that is only temporarily an
/// artifact or a creature. If this happens, the effect causes that permanent
/// to remain a green Elk creature even after the temporary effect expires."
/// A Colonnade that stood up for one turn is an Elk for the rest of the
/// game: the animation wears off, and the Elk does not.
#[test]
fn a_land_that_stood_up_for_a_turn_stays_an_elk() {
    let (mut game, oko, _) = staged(&[]);
    let colonnade = game
        .put_onto_battlefield(PlayerId::One, cards::CELESTIAL_COLONNADE)
        .expect("cataloged");
    drain_pending(&mut game);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
        permanent.tapped = false;
    }
    for color in ManaColor::COLORS {
        game.add_unrestricted_mana(PlayerId::One, color, 5);
    }
    let animate = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility { source, .. } if *source == colonnade)
        })
        .expect("the Colonnade may stand up");
    game.apply(PlayerId::One, animate).expect("it activates");
    drain_pending(&mut game);
    assert_eq!(
        (
            game.power(permanent(&game, colonnade)),
            game.toughness(permanent(&game, colonnade))
        ),
        (Some(4), Some(4)),
        "a 4/4 for the turn",
    );

    let elkify = loyalty_action(&game, oko, &[colonnade]).expect("+1 names the animated land");
    game.apply(PlayerId::One, elkify).expect("it activates");
    drain_pending(&mut game);

    // Round the turn, which is where the animation stops and the Elk does not.
    let turn = game.turn;
    for _ in 0..80 {
        if game.turn > turn + 1 {
            break;
        }
        game.advance_step();
        drain_pending(&mut game);
    }

    let elk = permanent(&game, colonnade);
    assert_eq!(
        (game.power(elk), game.toughness(elk)),
        (Some(3), Some(3)),
        "still the 3/3 Oko made of it",
    );
    let types = game.permanent_types(elk).expect("it has types");
    assert!(
        types.contains(CardType::Creature),
        "and still a creature, with nothing left to animate it",
    );
    assert!(
        game.effective_subtypes(elk)
            .contains(crate::card::Subtype::named("Elk")),
        "an Elk for good",
    );
}
