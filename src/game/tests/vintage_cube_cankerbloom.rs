//! Cankerbloom: a 3/2 for two that is also the artifact removal the deck was
//! going to have to find room for.

use super::*;

/// Player One with a Cankerbloom out since last turn and one mana up, plus
/// `theirs` on the battlefield under Player Two.
fn staged(theirs: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    for definition in theirs {
        game.put_onto_battlefield(PlayerId::Two, *definition)
            .expect("cataloged");
    }
    let bloom = game
        .put_onto_battlefield(PlayerId::One, cards::CANKERBLOOM)
        .expect("cataloged");
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.turns_started = [2, 1];
    drain_pending(&mut game);
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Colorless, 1);
    (game, bloom)
}

fn deciding(game: &Game) -> Option<PlayerId> {
    game.pending_decisions
        .first()
        .map(|pending| pending.observation.player)
}

fn settle(game: &mut Game) {
    for _ in 0..24 {
        if deciding(game).is_some() {
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

/// Activates the ability with mode `index` chosen, aimed at `target` when it
/// takes one.
fn activate(game: &mut Game, bloom: GameObjectId, index: usize, target: Option<Target>) {
    let mode = ModeId::from_index(index).expect("one of the three");
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source,
                modes,
                targets,
                ..
            } => {
                *source == bloom
                    && modes == &[mode]
                    && target.is_none_or(|wanted| {
                        targets
                            .iter()
                            .any(|selection| selection.targets().contains(&wanted))
                    })
            }
            _ => false,
        })
        .unwrap_or_else(|| panic!("mode {index} is activatable"));
    game.apply(PlayerId::One, action).expect("it activates");
    settle(game);
}

/// Which modes the Cankerbloom is offering right now.
fn offered_modes(game: &Game, bloom: GameObjectId) -> Vec<ModeId> {
    let mut modes = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::ActivateAbility { source, modes, .. } if source == bloom => {
                modes.first().copied()
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    modes.sort_unstable();
    modes.dedup();
    modes
}

fn on_battlefield(game: &Game, definition: CardDefinitionId) -> bool {
    game.battlefield
        .iter()
        .any(|permanent| permanent.card.definition == definition)
}

/// One mana and the body destroys an artifact.
#[test]
fn the_first_mode_destroys_an_artifact() {
    let (mut game, bloom) = staged(&[cards::SOL_RING]);
    let ring = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::SOL_RING)
        .expect("it is here")
        .card
        .id;

    activate(&mut game, bloom, 0, Some(Target::Permanent(ring)));

    assert!(!on_battlefield(&game, cards::SOL_RING), "the Ring is gone");
    assert!(
        !on_battlefield(&game, cards::CANKERBLOOM),
        "and the Cankerbloom sacrificed itself to do it",
    );
}

/// The second mode does the same for an enchantment, and only an
/// enchantment.
#[test]
fn the_second_mode_destroys_an_enchantment() {
    let (mut game, bloom) = staged(&[cards::CIRCLE_OF_PROTECTION_BLUE]);
    let circle = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.definition == cards::CIRCLE_OF_PROTECTION_BLUE)
        .expect("it is here")
        .card
        .id;

    activate(&mut game, bloom, 1, Some(Target::Permanent(circle)));

    assert!(
        !on_battlefield(&game, cards::CIRCLE_OF_PROTECTION_BLUE),
        "the Circle is gone",
    );
}

/// A mode with nothing to point at is not on offer, which is what makes the
/// third one worth having.
#[test]
fn a_targeting_mode_with_nothing_to_hit_is_not_offered() {
    let (game, bloom) = staged(&[]);

    assert_eq!(
        offered_modes(&game, bloom),
        vec![ModeId::from_index(2).expect("the third mode")],
        "an empty board leaves only the proliferate",
    );
}

/// Proliferate gives another counter of each kind already there.
#[test]
fn proliferate_adds_one_of_each_kind_already_there() {
    let (mut game, bloom) = staged(&[]);
    let bears = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    drain_pending(&mut game);
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == bears)
    {
        permanent.set_counters(CounterKind::PlusOnePlusOne, 2);
        permanent.set_counters(CounterKind::named("charge"), 1);
    }
    game.priority = PlayerId::One;

    activate(&mut game, bloom, 2, None);
    let seat = deciding(&game).expect("it asks what to proliferate");
    let decision = game.observe(seat).decision.expect("just checked");
    let option = decision
        .options
        .iter()
        .find(|option| option.card.is_some_and(|(found, _)| found == bears))
        .expect("the Bears carries counters, so it is on the menu")
        .id;
    game.apply(
        seat,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .expect("the answer is legal");
    settle(&mut game);

    let bears = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == bears)
        .expect("it is still here");
    assert_eq!(
        bears.counters(CounterKind::PlusOnePlusOne),
        3,
        "one more of the kind that was there",
    );
    assert_eq!(
        bears.counters(CounterKind::named("charge")),
        2,
        "and one more of the other kind too",
    );
}

/// A permanent with no counters is not something another counter could be
/// given to, so it is not on the menu.
#[test]
fn a_permanent_with_no_counters_is_not_a_candidate() {
    let (mut game, bloom) = staged(&[]);
    game.put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    drain_pending(&mut game);
    game.priority = PlayerId::One;

    activate(&mut game, bloom, 2, None);

    assert!(
        deciding(&game).is_none(),
        "nothing on the board has a counter, so nothing is asked",
    );
}

/// A poisoned player is a candidate, and proliferating them adds one.
#[test]
fn proliferate_reaches_a_player_with_counters() {
    let (mut game, bloom) = staged(&[]);
    game.players[1].counters.set(CounterKind::Poison, 3);

    activate(&mut game, bloom, 2, None);
    let seat = deciding(&game).expect("it asks");
    let decision = game.observe(seat).decision.expect("just checked");
    let option = decision
        .options
        .iter()
        .find(|option| option.card.is_none())
        .expect("the poisoned player is on the menu")
        .id;
    game.apply(
        seat,
        Action::ChooseDecision {
            decision: decision.id,
            options: vec![option],
        },
    )
    .expect("the answer is legal");
    settle(&mut game);

    assert_eq!(
        game.players[1].counters.count(CounterKind::Poison),
        4,
        "one more poison counter"
    );
}

/// "Any number" includes none.
#[test]
fn proliferating_nothing_is_allowed() {
    let (mut game, bloom) = staged(&[]);
    game.players[1].counters.set(CounterKind::Poison, 3);

    activate(&mut game, bloom, 2, None);
    let seat = deciding(&game).expect("it asks");
    let decision = game.observe(seat).decision.expect("just checked");
    assert_eq!(decision.minimum, 0, "with no obligation to choose any");
    game.apply(
        seat,
        Action::ChooseDecision {
            decision: decision.id,
            options: Vec::new(),
        },
    )
    .expect("the empty answer is legal");
    settle(&mut game);

    assert_eq!(
        game.players[1].counters.count(CounterKind::Poison),
        3,
        "and nothing was added"
    );
}
