//! Smuggler's Copter: a 3/3 flier any one creature can turn on, which fixes
//! every draw it connects with.

use super::*;

/// The Copter on the battlefield, with `crew` creatures beside it.
fn staged(crew: &[CardDefinitionId]) -> (Game, GameObjectId, Vec<GameObjectId>) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    let copter = game
        .put_onto_battlefield(PlayerId::One, cards::SMUGGLER_S_COPTER)
        .expect("cataloged");
    let mut ids = Vec::new();
    for definition in crew {
        ids.push(
            game.put_onto_battlefield(PlayerId::One, *definition)
                .expect("cataloged"),
        );
    }
    drain_pending(&mut game);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
        permanent.tapped = false;
    }
    game.turns_started = [5, 5];
    game.turn = 9;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    (game, copter, ids)
}

/// Answers whatever is asked, taking a "you may" rather than declining it:
/// the loot is the half of the card worth watching.
fn settle(game: &mut Game) {
    for _ in 0..24 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = decision
                .options
                .iter()
                .find(|option| option.label == "Do it")
                .map_or_else(
                    || {
                        decision
                            .options
                            .iter()
                            .map(|option| option.id)
                            .take(decision.minimum.max(1))
                            .collect()
                    },
                    |option| vec![option.id],
                );
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .expect("the offered choice is legal");
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
}

fn permanent(game: &Game, id: GameObjectId) -> &Permanent {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("it is on the battlefield")
}

fn is_creature(game: &Game, id: GameObjectId) -> bool {
    game.permanent_types(permanent(game, id))
        .is_some_and(|types| types.contains(CardType::Creature))
}

/// Crews it by tapping whatever is offered.
fn crew(game: &mut Game, copter: GameObjectId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == copter),
        )
        .expect("crew is activatable");
    game.apply(PlayerId::One, action).expect("it crews");
    settle(game);
}

/// Answers the pending decision with the one option whose label contains
/// `needle`, which the blanket `settle` would not pick for itself.
fn answer_containing(game: &mut Game, needle: &str) {
    let decision = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
        .expect("a decision is waiting");
    let option = decision
        .options
        .iter()
        .find(|option| option.label.contains(needle))
        .unwrap_or_else(|| panic!("no option named {needle:?} in {:?}", decision.options));
    game.apply(
        decision.player,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option.id],
        },
    )
    .expect("the offered choice is legal");
}

/// Uncrewed it is an artifact and nothing else: no power, no toughness, and
/// nothing to attack with.
#[test]
fn uncrewed_it_is_not_a_creature() {
    let (game, copter, _) = staged(&[cards::GRIZZLY_BEARS]);

    assert!(!is_creature(&game, copter));
    assert_eq!(game.power(permanent(&game, copter)), None);
}

/// Crewing makes it a 3/3 artifact creature with flying.
#[test]
fn crewing_makes_it_a_flying_three_three() {
    let (mut game, copter, crewers) = staged(&[cards::GRIZZLY_BEARS]);

    crew(&mut game, copter);

    assert!(is_creature(&game, copter));
    assert_eq!(game.power(permanent(&game, copter)), Some(3));
    assert_eq!(game.toughness(permanent(&game, copter)), Some(3));
    assert!(game.has_flying(permanent(&game, copter)));
    assert!(
        permanent(&game, crewers[0]).tapped,
        "and the crew is tapped for it",
    );
}

/// Crew 1: one power is enough, and a 1/1 is one power.
#[test]
fn one_power_is_enough() {
    let (mut game, copter, _) = staged(&[cards::SAVANNAH_LIONS]);

    crew(&mut game, copter);

    assert!(is_creature(&game, copter));
}

/// With nothing to tap it stays an artifact.
#[test]
fn it_cannot_crew_itself() {
    let (game, copter, _) = staged(&[]);

    assert!(
        !game.legal_actions(PlayerId::One).into_iter().any(
            |action| matches!(action, Action::ActivateAbility { source, .. } if source == copter)
        ),
        "a Vehicle is not a creature and cannot pay its own crew cost",
    );
}

/// Attacking loots: a card drawn and a card discarded.
#[test]
fn attacking_loots() {
    let (mut game, copter, _) = staged(&[cards::GRIZZLY_BEARS]);
    crew(&mut game, copter);
    game.players[0]
        .hand
        .push(card(99_000, cards::MOUNTAIN, PlayerId::One));
    let hand = game.players[0].hand.len();
    let library = game.players[0].library.len();

    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.declare_attacker(copter, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    settle(&mut game);

    assert_eq!(game.players[0].library.len(), library - 1, "one drawn");
    assert_eq!(game.players[0].hand.len(), hand, "and one discarded");
    assert_eq!(game.players[0].graveyard.len(), 1);
}

/// Blocking loots the same way, which is the other half of one clause.
#[test]
fn blocking_loots_too() {
    let (mut game, copter, _) = staged(&[cards::GRIZZLY_BEARS]);
    crew(&mut game, copter);
    game.players[0]
        .hand
        .push(card(99_100, cards::MOUNTAIN, PlayerId::One));
    let library = game.players[0].library.len();
    let attacker = creature(99_200, cards::SERRA_ANGEL, PlayerId::Two);
    let attacker_id = attacker.card.id;
    game.battlefield.push(attacker);
    drain_pending(&mut game);

    game.active_player = PlayerId::Two;
    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == attacker_id)
    {
        permanent.entered_controller_turn = 0;
    }
    game.declare_attacker(attacker_id, AttackDefender::Player(PlayerId::One));
    game.finish_declaring_attackers();
    settle(&mut game);
    game.step = Step::DeclareBlockers;
    game.declare_blocker(copter, attacker_id);
    game.finish_declaring_blockers();
    settle(&mut game);

    assert_eq!(game.players[0].library.len(), library - 1, "blocking loots");
}

/// Crewing lasts until end of turn: next turn it is an artifact again.
#[test]
fn the_crew_wears_off() {
    let (mut game, copter, _) = staged(&[cards::GRIZZLY_BEARS]);
    crew(&mut game, copter);
    assert!(is_creature(&game, copter));

    for _ in 0..40 {
        if game.turn > 9 {
            break;
        }
        game.advance_step();
        settle(&mut game);
    }

    assert!(!is_creature(&game, copter), "back to being an artifact");
}

/// "Any untapped creature you control can be tapped to pay a crew cost, even
/// one that just came under your control." A bear that cannot attack is a
/// bear that can still crew.
#[test]
fn a_creature_that_arrived_this_turn_may_still_crew() {
    let (mut game, copter, crewers) = staged(&[cards::GRIZZLY_BEARS]);
    let bear = crewers[0];
    let arrived = game.turns_started[PlayerId::One.index()];
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == bear)
        .expect("it is there")
        .entered_controller_turn = arrived;

    assert!(
        !game.legal_actions(PlayerId::One).iter().any(|action| {
            matches!(action, Action::DeclareAttacker { attacker, .. } if *attacker == bear)
        }),
        "the bear itself is too new to attack",
    );

    crew(&mut game, copter);

    assert!(is_creature(&game, copter), "and new or not, it crewed");
    assert!(
        permanent(&game, bear).tapped,
        "the crew cost taps whoever pays it",
    );
    assert_eq!(
        (
            game.power(permanent(&game, copter)),
            game.toughness(permanent(&game, copter))
        ),
        (Some(3), Some(3)),
        "the Vehicle is the printed 3/3, not the bear that crewed it",
    );
}

/// The loot is a "you may": declining draws nothing and discards nothing,
/// which is what you want with a hand you would rather keep.
#[test]
fn the_loot_may_be_declined() {
    let (mut game, copter, _) = staged(&[cards::GRIZZLY_BEARS]);
    crew(&mut game, copter);
    game.players[PlayerId::One.index()]
        .hand
        .push(card(99_100, cards::MOUNTAIN, PlayerId::One));
    let hand = game.players[PlayerId::One.index()].hand.len();
    let library = game.players[PlayerId::One.index()].library.len();

    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.declare_attacker(copter, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();

    let decision = loop {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            break decision;
        }
        let priority = game.priority;
        assert!(
            game.apply(priority, Action::PassPriority).is_ok(),
            "the attack trigger is waiting to ask",
        );
    };
    let decline = decision
        .options
        .iter()
        .find(|option| option.label != "Do it")
        .expect("declining is one of the answers");
    game.apply(
        PlayerId::One,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![decline.id],
        },
    )
    .expect("a may is a may");
    drain_pending(&mut game);

    assert_eq!(
        game.players[PlayerId::One.index()].library.len(),
        library,
        "nothing was drawn",
    );
    assert_eq!(
        game.players[PlayerId::One.index()].hand.len(),
        hand,
        "so nothing had to be discarded",
    );
    assert!(game.players[PlayerId::One.index()].graveyard.is_empty());
}

/// "Vehicle is an artifact type, not a creature type. A Vehicle that's
/// crewed won't normally have any creature type." It is a 3/3 artifact
/// creature and a Vehicle, and nothing a tribal card would recognise.
#[test]
fn a_crewed_copter_has_no_creature_type() {
    let (mut game, copter, _) = staged(&[cards::GRIZZLY_BEARS]);

    crew(&mut game, copter);

    let subtypes = game.effective_subtypes(permanent(&game, copter));
    assert_eq!(
        subtypes,
        crate::card::SubtypeSet::from_names(&["Vehicle"]),
        "the artifact type it was printed with, and no creature type at all",
    );
    assert!(
        game.permanent_types(permanent(&game, copter))
            .is_some_and(
                |types| types.contains(CardType::Creature) && types.contains(CardType::Artifact)
            ),
        "while being both an artifact and a creature",
    );
}

/// "When a Vehicle becomes a creature, that doesn't count as having a
/// creature enter the battlefield." A Champion of Lambholt watching for
/// arrivals sees nothing: the Copter was already there.
#[test]
fn crewing_is_not_a_creature_entering() {
    let (mut game, copter, _) = staged(&[cards::GRIZZLY_BEARS]);
    let champion = game
        .put_onto_battlefield(PlayerId::One, cards::CHAMPION_OF_LAMBHOLT)
        .expect("cataloged");
    drain_pending(&mut game);
    game.priority = PlayerId::One;
    let before = permanent(&game, champion).counters(CounterKind::PlusOnePlusOne);

    crew(&mut game, copter);

    assert_eq!(
        permanent(&game, champion).counters(CounterKind::PlusOnePlusOne),
        before,
        "it only changed what it is",
    );
}

/// "Creatures that crew a Vehicle aren't attached to it or related in any
/// other way. Effects that affect the Vehicle don't affect the creatures
/// that crewed it." Destroying the Copter leaves the bear that flew it
/// standing, tapped and otherwise untouched.
#[test]
fn destroying_the_copter_leaves_its_crew_alone() {
    let (mut game, copter, crewers) = staged(&[cards::GRIZZLY_BEARS]);
    let bears = crewers[0];
    crew(&mut game, copter);

    game.destroy_permanent(copter);
    game.check_state_based_actions();
    drain_pending(&mut game);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != copter),
        "the Vehicle is gone",
    );
    let bear = permanent(&game, bears);
    assert_eq!(
        (game.power(bear), game.toughness(bear)),
        (Some(2), Some(2)),
        "and the bear that crewed it is a bear still",
    );
    assert!(bear.tapped, "tapped, which is all the crewing cost it");
}

/// "You may tap more creatures than necessary to activate a crew ability."
/// The cost is asked one creature at a time, and the seat that has already
/// paid crew 1 is still offered the second creature: tapping both is legal,
/// and both end up tapped for it.
#[test]
fn more_crew_than_needed_may_be_tapped() {
    let (mut game, copter, crewers) = staged(&[cards::GRIZZLY_BEARS, cards::SAVANNAH_LIONS]);

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == copter),
        )
        .expect("crew is activatable");
    game.apply(PlayerId::One, action).expect("it crews");
    // The cost is paid one creature at a time, so both are tapped by
    // answering twice rather than by naming a pair.
    for crewer in &crewers {
        let decision = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
            .expect("it asks who crews");
        let option = decision
            .options
            .iter()
            .find(|option| option.card.is_some_and(|(object, _)| object == *crewer))
            .unwrap_or_else(|| panic!("{crewer:?} is still on offer"))
            .id;
        game.apply(
            PlayerId::One,
            Action::ChooseDecision {
                decision: decision.id,
                options: vec![option],
            },
        )
        .expect("tapping it is legal");
    }
    settle(&mut game);

    assert!(
        is_creature(&game, copter),
        "one power was enough twice over"
    );
    assert!(
        crewers
            .iter()
            .all(|crewer| permanent(&game, *crewer).tapped),
        "and both of them paid for it",
    );
}

/// "Once a Vehicle becomes a creature, it behaves exactly like any other
/// artifact creature. It can't attack unless you've controlled it
/// continuously since your turn began." Crewing is not haste: a Copter cast
/// this turn flies nowhere, though it still blocks and still crews.
#[test]
fn a_copter_that_arrived_this_turn_cannot_attack() {
    let (mut game, copter, _) = staged(&[cards::GRIZZLY_BEARS]);
    let arrived = game.turns_started[PlayerId::One.index()];
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == copter)
        .expect("it is there")
        .entered_controller_turn = arrived;

    crew(&mut game, copter);

    assert!(is_creature(&game, copter), "it crewed regardless");
    assert!(
        !game.can_attack(permanent(&game, copter)),
        "but it has not been here since the turn began",
    );
    assert!(
        !game.legal_actions(PlayerId::One).iter().any(|action| {
            matches!(action, Action::DeclareAttacker { attacker, .. } if *attacker == copter)
        }),
        "so it is never offered as an attacker",
    );
}

/// "You may activate a crew ability of a Vehicle even if it's already an
/// artifact creature. Doing so has no effect on the Vehicle. It doesn't
/// change its power and toughness." The second crew costs a second creature
/// and buys nothing.
#[test]
fn crewing_a_crewed_copter_changes_nothing() {
    let (mut game, copter, crewers) = staged(&[cards::GRIZZLY_BEARS, cards::SAVANNAH_LIONS]);
    crew(&mut game, copter);
    let after_first = (
        game.power(permanent(&game, copter)),
        game.toughness(permanent(&game, copter)),
    );
    assert_eq!(after_first, (Some(3), Some(3)));
    let untapped = crewers
        .iter()
        .copied()
        .find(|id| !permanent(&game, *id).tapped)
        .expect("only one of the two paid for the first crew");

    crew(&mut game, copter);

    assert!(
        permanent(&game, untapped).tapped,
        "the second crew really was paid for",
    );
    assert!(is_creature(&game, copter), "and it is still a creature");
    assert_eq!(
        (
            game.power(permanent(&game, copter)),
            game.toughness(permanent(&game, copter))
        ),
        after_first,
        "still the printed 3/3, not a 6/6",
    );
}

/// "If a permanent becomes a copy of a Vehicle, the copy won't be a
/// creature, even if the Vehicle it's copying has become an artifact
/// creature." Crewing is an effect on that permanent, not part of what a
/// copy reads: the Copy Artifact arrives as a Vehicle waiting for a crew of
/// its own.
#[test]
fn a_copy_of_a_crewed_copter_is_not_a_creature() {
    let (mut game, copter, _) = staged(&[cards::GRIZZLY_BEARS]);
    crew(&mut game, copter);
    assert!(is_creature(&game, copter), "the original is crewed");

    let copy = game
        .put_onto_battlefield(PlayerId::One, cards::COPY_ARTIFACT)
        .expect("cataloged");
    answer_containing(&mut game, "copy of");
    settle(&mut game);

    assert_ne!(copy, copter, "a second permanent, not the same one");
    let types = game
        .permanent_types(permanent(&game, copy))
        .expect("it is on the battlefield");
    assert!(
        types.contains(CardType::Artifact),
        "it copied the Copter: {types:?}",
    );
    assert!(
        game.effective_subtypes(permanent(&game, copy))
            .contains(crate::card::Subtype::named("Vehicle")),
        "a Vehicle like the one it copied",
    );
    assert!(
        !is_creature(&game, copy),
        "but the crewing did not come with it",
    );
    assert!(
        is_creature(&game, copter),
        "and the original is still crewed"
    );
}
