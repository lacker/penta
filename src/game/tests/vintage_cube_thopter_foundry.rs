//! Thopter Foundry: every spent artifact becomes a flier and a life, and it
//! will not eat what it just made.

use super::*;

/// The Foundry on the battlefield under Player One, with `artifacts`
/// beside it.
fn staged(artifacts: &[CardDefinitionId]) -> (Game, GameObjectId, Vec<GameObjectId>) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let foundry = game
        .put_onto_battlefield(PlayerId::One, cards::THOPTER_FOUNDRY)
        .expect("cataloged");
    let mut ids = Vec::new();
    for definition in artifacts {
        ids.push(
            game.put_onto_battlefield(PlayerId::One, *definition)
                .expect("cataloged"),
        );
    }
    drain_pending(&mut game);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.players[0].life = 20;
    game.turns_started = [5, 5];
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, foundry, ids)
}

fn thopters(game: &Game) -> Vec<&Permanent> {
    game.battlefield
        .iter()
        .filter(|permanent| {
            game.effective_subtypes(permanent)
                .contains(crate::card::Subtype::named("Thopter"))
        })
        .collect()
}

/// Every way the Foundry can be activated right now, with what each of them
/// would sacrifice.
fn activations(game: &Game, foundry: GameObjectId) -> Vec<(Action, Vec<GameObjectId>)> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match &action {
            Action::ActivateAbility {
                source,
                cost_objects,
                ..
            } if *source == foundry => {
                let spent = cost_objects.clone();
                Some((action, spent))
            }
            _ => None,
        })
        .collect()
}

/// One mana and an artifact buys a flier and a life.
#[test]
fn it_turns_an_artifact_into_a_flier_and_a_life() {
    let (mut game, foundry, artifacts) = staged(&[cards::HOWLING_MINE]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);

    let offers = activations(&game, foundry);
    assert_eq!(
        offers.len(),
        2,
        "the Mine or the Foundry itself, both being nontoken artifacts",
    );
    let eat_the_mine = offers
        .into_iter()
        .find(|(_, spent)| spent == &vec![artifacts[0]])
        .expect("the Mine is one of them");
    game.apply(PlayerId::One, eat_the_mine.0)
        .expect("it activates");
    drain_pending(&mut game);

    let made = thopters(&game);
    assert_eq!(made.len(), 1, "one Thopter");
    assert_eq!(game.power(made[0]), Some(1));
    assert_eq!(game.toughness(made[0]), Some(1));
    assert!(game.has_flying(made[0]), "and it flies");
    assert!(
        game.permanent_types(made[0]).is_some_and(
            |types| types.contains(CardType::Artifact) && types.contains(CardType::Creature)
        ),
        "an artifact creature",
    );
    assert_eq!(game.players[0].life, 21, "and a life");
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == artifacts[0]),
        "the Mine is gone",
    );
}

/// "A nontoken artifact": the Thopter it just made is not food.
#[test]
fn it_will_not_eat_its_own_thopter() {
    let (mut game, foundry, artifacts) = staged(&[cards::HOWLING_MINE]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 2);
    let eat_the_mine = activations(&game, foundry)
        .into_iter()
        .find(|(_, spent)| spent == &vec![artifacts[0]])
        .expect("the Mine is on offer");
    game.apply(PlayerId::One, eat_the_mine.0)
        .expect("it activates");
    drain_pending(&mut game);
    let thopter = thopters(&game);
    assert_eq!(thopter.len(), 1);
    let thopter = thopter[0].card.id;

    let spent = activations(&game, foundry)
        .into_iter()
        .map(|(_, objects)| objects)
        .collect::<Vec<_>>();

    assert_eq!(
        spent,
        vec![vec![foundry]],
        "the token it made cannot pay for the next one",
    );
    assert!(
        !spent.iter().any(|objects| objects.contains(&thopter)),
        "and it is the token that is refused rather than the artifact type",
    );
}

/// The Foundry itself is a nontoken artifact, so it may eat itself.
#[test]
fn it_may_eat_itself() {
    let (mut game, foundry, _) = staged(&[]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);

    let offers = activations(&game, foundry);
    assert_eq!(offers.len(), 1, "with nothing else, it is its own fuel");
    assert_eq!(offers[0].1, vec![foundry]);
    game.apply(PlayerId::One, offers[0].0.clone())
        .expect("it activates");
    drain_pending(&mut game);

    assert_eq!(thopters(&game).len(), 1, "the Thopter still arrives");
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == foundry),
        "and the Foundry is gone",
    );
}

/// A nonartifact permanent is not food either.
#[test]
fn a_creature_cannot_pay_for_it() {
    let (mut game, foundry, _) = staged(&[cards::GRIZZLY_BEARS]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);

    let spent = activations(&game, foundry)
        .into_iter()
        .map(|(_, objects)| objects)
        .collect::<Vec<_>>();

    assert_eq!(
        spent,
        vec![vec![foundry]],
        "only the Foundry itself is an artifact here",
    );
}

/// Without the mana it cannot be activated at all.
#[test]
fn it_costs_a_mana() {
    let (game, foundry, _) = staged(&[cards::HOWLING_MINE]);

    assert!(
        activations(&game, foundry).is_empty(),
        "an artifact alone does not pay for it",
    );
}

/// "{W/B}{U}": the hybrid pip takes either colour, and the blue one takes
/// only blue.
#[test]
fn the_hybrid_pip_takes_either_half() {
    for (first, castable) in [
        (ManaColor::White, true),
        (ManaColor::Black, true),
        (ManaColor::Green, false),
    ] {
        let mut game = ready_game();
        game.battlefield.clear();
        game.players[0].hand.clear();
        let foundry = game
            .build_zone(PlayerId::One, &[cards::THOPTER_FOUNDRY])
            .expect("cataloged")
            .into_iter()
            .next()
            .expect("one card");
        let foundry_id = foundry.id;
        game.players[0].hand.push(foundry);
        game.turns_started = [5, 5];
        game.active_player = PlayerId::One;
        game.step = Step::PrecombatMain;
        game.priority = PlayerId::One;
        game.add_unrestricted_mana(PlayerId::One, first, 1);
        game.add_unrestricted_mana(PlayerId::One, ManaColor::Blue, 1);

        let offered = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .any(|action| matches!(action, Action::CastSpell { card, .. } if card == foundry_id));
        assert_eq!(
            offered, castable,
            "{first:?} beside a blue mana pays the hybrid half",
        );
    }
}

/// The token is blue, which is the one thing about it the other tests leave
/// out -- and it is what the Foundry is played for in a deck full of
/// colourless artifacts.
#[test]
fn the_thopter_it_makes_is_blue() {
    let (mut game, foundry, artifacts) = staged(&[cards::HOWLING_MINE]);
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);

    let eat_the_mine = activations(&game, foundry)
        .into_iter()
        .find(|(_, spent)| spent == &vec![artifacts[0]])
        .expect("the Mine is food");
    game.apply(PlayerId::One, eat_the_mine.0)
        .expect("it activates");
    drain_pending(&mut game);

    let made = thopters(&game);
    let colors = game.permanent_colors(made[0]);
    let blue = ManaColor::Blue.color_index().expect("blue is a colour");
    assert!(colors[blue], "a blue Thopter");
    assert_eq!(
        colors.iter().filter(|present| **present).count(),
        1,
        "and blue alone, whatever it was made out of",
    );
}

/// An activated ability with no timing restriction: the Foundry answers an
/// attack on the opponent's turn, and the Thopter it makes may block the
/// turn it arrives -- summoning sickness stops attacking and tapping, not
/// blocking (CR 302.6).
#[test]
fn it_makes_a_blocker_on_their_turn() {
    let (mut game, foundry, _) = staged(&[cards::HOWLING_MINE]);
    let bears = game
        .put_onto_battlefield(PlayerId::Two, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    drain_pending(&mut game);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    game.active_player = PlayerId::Two;
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.priority = PlayerId::Two;
    game.apply(
        PlayerId::Two,
        Action::DeclareAttacker {
            attacker: bears,
            defender: AttackDefender::Player(PlayerId::One),
        },
    )
    .expect("the Bears attack");
    game.apply(PlayerId::Two, Action::FinishDeclaringAttackers)
        .expect("the declaration finishes");

    for _ in 0..4 {
        if game.priority == PlayerId::One {
            break;
        }
        let priority = game.priority;
        if game.apply(priority, Action::PassPriority).is_err() {
            break;
        }
    }

    let offers = activations(&game, foundry);
    assert!(
        !offers.is_empty(),
        "nothing about the ability waits for your own main phase",
    );
    game.apply(
        PlayerId::One,
        offers.into_iter().next().expect("one of them").0,
    )
    .expect("it activates mid-combat");
    drain_pending(&mut game);
    let thopter = thopters(&game)
        .first()
        .expect("a Thopter to stand in the way")
        .card
        .id;

    game.step = Step::DeclareBlockers;
    game.blockers_declared = false;
    game.apply(
        PlayerId::One,
        Action::DeclareBlocker {
            blocker: thopter,
            attacker: bears,
        },
    )
    .expect("a creature that arrived this turn may still block");
    game.apply(PlayerId::One, Action::FinishDeclaringBlockers)
        .expect("the declaration finishes");

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == thopter && permanent.blocking.contains(&bears)),
        "the Bears are blocked by a Thopter that did not exist when they were declared",
    );
}
