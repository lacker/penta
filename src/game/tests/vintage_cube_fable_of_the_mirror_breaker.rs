//! Fable of the Mirror-Breaker: three chapters read one lore counter at a
//! time, and the third one turns the Saga into a creature.

use super::*;

/// The Saga on the battlefield under Player One, with `hand` in hand.
fn staged(hand: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready_game();
    game.battlefield.clear();
    game.players[0].hand.clear();
    game.players[0].library.clear();
    for (index, definition) in [cards::MOUNTAIN, cards::FOREST, cards::PLAINS]
        .into_iter()
        .enumerate()
    {
        let id = 280_000 + u32::try_from(index).expect("three cards");
        game.players[0]
            .library
            .push(card(id, definition, PlayerId::One));
    }
    for (index, definition) in hand.iter().enumerate() {
        let id = 280_100 + u32::try_from(index).expect("a short hand");
        game.players[0]
            .hand
            .push(card(id, *definition, PlayerId::One));
    }
    game.turns_started = [5, 5];
    game.turn = 9;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    let fable = game
        .put_onto_battlefield(PlayerId::One, cards::FABLE_OF_THE_MIRROR_BREAKER)
        .expect("cataloged");
    settle(&mut game);
    (game, fable)
}

fn settle(game: &mut Game) {
    for _ in 0..40 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = decision
                .options
                .iter()
                .map(|option| option.id)
                .take(decision.minimum)
                .collect();
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

fn lore(game: &Game, saga: GameObjectId) -> u16 {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == saga)
        .map_or(0, |permanent| permanent.counters(CounterKind::Lore))
}

fn tokens(game: &Game) -> Vec<GameObjectId> {
    game.battlefield
        .iter()
        .filter(|permanent| permanent.card.definition == ObjectKind::Token)
        .map(|permanent| permanent.card.id)
        .collect()
}

/// Carries the turn round to Player One's next precombat main phase, which
/// is where the next lore counter goes on.
fn next_turn(game: &mut Game) {
    // At least one step first: the caller is standing in the main phase this
    // is meant to leave.
    game.advance_step();
    settle(game);
    for _ in 0..64 {
        if game.step == Step::PrecombatMain && game.active_player == PlayerId::One {
            break;
        }
        game.advance_step();
        settle(game);
    }
}

/// It arrives with one lore counter and reads its first chapter at once.
#[test]
fn it_enters_reading_its_first_chapter() {
    let (game, fable) = staged(&[]);

    assert_eq!(lore(&game, fable), 1, "a lore counter as it enters");
    let made = tokens(&game);
    assert_eq!(made.len(), 1, "and the Goblin it makes");
    let goblin = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == made[0])
        .expect("still there");
    assert_eq!(game.power(goblin), Some(2));
    assert_eq!(game.toughness(goblin), Some(2));
}

/// The Goblin brings a Treasure with every attack.
#[test]
fn the_goblin_makes_treasure_when_it_attacks() {
    let (mut game, _fable) = staged(&[]);
    let goblin = tokens(&game).into_iter().next().expect("one Goblin");
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }

    game.step = Step::DeclareAttackers;
    game.attackers_declared = false;
    game.declare_attacker(goblin, AttackDefender::Player(PlayerId::Two));
    game.finish_declaring_attackers();
    settle(&mut game);

    assert_eq!(tokens(&game).len(), 2, "the Goblin and its Treasure");
}

/// The second chapter comes after the next draw step, and loots.
#[test]
fn the_second_chapter_loots() {
    let (mut game, fable) = staged(&[cards::LIGHTNING_BOLT, cards::SERRA_ANGEL]);
    assert_eq!(game.players[0].hand.len(), 2);

    next_turn(&mut game);

    assert_eq!(lore(&game, fable), 2, "one more lore counter");
    assert_eq!(
        game.players[0].graveyard.len(),
        0,
        "the settle helper declines the discard",
    );
    assert_eq!(game.players[0].hand.len(), 3, "and the turn's draw arrived");
}

/// The third chapter exiles the Saga and brings it back as a creature.
#[test]
fn the_third_chapter_turns_it_into_a_creature() {
    let (mut game, fable) = staged(&[]);
    next_turn(&mut game);
    next_turn(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == fable),
        "the Saga that was there is gone",
    );
    let reflection = game
        .battlefield
        .iter()
        .find(|permanent| {
            permanent.card.definition == ObjectKind::Card(cards::FABLE_OF_THE_MIRROR_BREAKER)
        })
        .expect("and something of it came back");
    assert_eq!(game.power(reflection), Some(2), "as a 2/2");
    assert_eq!(game.toughness(reflection), Some(2));
    assert_eq!(
        reflection.counters(CounterKind::Lore),
        0,
        "a new object, with none of the counters it had",
    );
    assert!(
        game.players[0].graveyard.is_empty(),
        "and it was never sacrificed",
    );
}

/// The Reflection copies another creature, with haste, until end of turn.
#[test]
fn the_reflection_copies_a_creature_for_the_turn() {
    let (mut game, _fable) = staged(&[]);
    next_turn(&mut game);
    next_turn(&mut game);
    let reflection = game
        .battlefield
        .iter()
        .find(|permanent| {
            permanent.card.definition == ObjectKind::Card(cards::FABLE_OF_THE_MIRROR_BREAKER)
        })
        .map(|permanent| permanent.card.id)
        .expect("the Reflection is there");
    let bears = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    settle(&mut game);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    let before = tokens(&game);

    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == reflection
                    && targets
                        .iter()
                        .any(|slot| slot.targets().contains(&Target::Permanent(bears)))
            }
            _ => false,
        })
        .expect("one mana and a tap copies the Bears");
    game.apply(PlayerId::One, activation)
        .expect("it is activated");
    settle(&mut game);

    // The Goblin from the first chapter is still around, so the copy is
    // whichever token was not there before.
    let copy_id = tokens(&game)
        .into_iter()
        .find(|token| !before.contains(token))
        .expect("a copy arrived");
    let copy = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == copy_id)
        .expect("still there");
    assert_eq!(game.power(copy), Some(2), "a copy of the Bears");
    assert!(
        game.permanent_has_executable_keyword(copy, KeywordAbility::Haste),
        "except it has haste",
    );
    game.step = Step::End;
    game.begin_step_triggers();
    settle(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == copy_id),
        "and it is sacrificed at the end step",
    );
}

/// It may not copy itself: "another" leaves the Reflection out.
#[test]
fn the_reflection_cannot_copy_itself() {
    let (mut game, _fable) = staged(&[]);
    next_turn(&mut game);
    next_turn(&mut game);
    let reflection = game
        .battlefield
        .iter()
        .find(|permanent| {
            permanent.card.definition == ObjectKind::Card(cards::FABLE_OF_THE_MIRROR_BREAKER)
        })
        .map(|permanent| permanent.card.id)
        .expect("the Reflection is there");
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);

    assert!(
        !game.legal_actions(PlayerId::One).into_iter().any(|action| {
            matches!(action, Action::ActivateAbility { source, targets, .. }
                if source == reflection
                    && targets
                        .iter()
                        .any(|slot| slot.targets().contains(&Target::Permanent(reflection))))
        }),
        "it is not among its own choices",
    );
}

/// Reaches the Reflection with a mana up and returns its id.
fn reflection_ready(game: &mut Game) -> GameObjectId {
    next_turn(game);
    next_turn(game);
    let reflection = game
        .battlefield
        .iter()
        .find(|permanent| {
            permanent.card.definition == ObjectKind::Card(cards::FABLE_OF_THE_MIRROR_BREAKER)
        })
        .map(|permanent| permanent.card.id)
        .expect("the Reflection is there");
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    reflection
}

/// Every permanent the Reflection is offering to copy right now.
fn copy_targets(game: &Game, reflection: GameObjectId) -> Vec<Target> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } if source == reflection => Some(
                targets
                    .iter()
                    .flat_map(crate::casting::TargetSelection::targets)
                    .copied()
                    .collect::<Vec<_>>(),
            ),
            _ => None,
        })
        .flatten()
        .collect()
}

/// "Another target nonlegendary creature you control" is three restrictions,
/// and only the first had a test. A legend of yours and a creature of theirs
/// are both out of reach.
#[test]
fn it_copies_only_your_own_nonlegendary_creatures() {
    let (mut game, _fable) = staged(&[]);
    let bears = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    let legend = game
        .put_onto_battlefield(PlayerId::One, cards::THALIA_GUARDIAN_OF_THRABEN)
        .expect("cataloged");
    let theirs = game
        .put_onto_battlefield(PlayerId::Two, cards::SERRA_ANGEL)
        .expect("cataloged");
    settle(&mut game);
    let reflection = reflection_ready(&mut game);

    let offered = copy_targets(&game, reflection);
    assert!(
        offered.contains(&Target::Permanent(bears)),
        "the Bears are yours and nonlegendary: {offered:?}",
    );
    assert!(
        !offered.contains(&Target::Permanent(legend)),
        "a legend of your own is not copyable",
    );
    assert!(
        !offered.contains(&Target::Permanent(theirs)),
        "and neither is a creature you do not control",
    );
}

/// "Any enters-the-battlefield abilities of the copied creature will trigger
/// when the token enters." A copied Epicure pings and leaves its own Blood.
#[test]
fn the_copy_brings_the_enters_trigger_with_it() {
    let (mut game, _fable) = staged(&[]);
    let epicure = game
        .put_onto_battlefield(PlayerId::One, cards::VOLDAREN_EPICURE)
        .expect("cataloged");
    settle(&mut game);
    let reflection = reflection_ready(&mut game);
    let life = game.players[1].life;
    let bloods = |game: &Game| {
        game.battlefield
            .iter()
            .filter(|permanent| {
                game.effective_subtypes(permanent)
                    .contains(crate::card::Subtype::named("Blood"))
            })
            .count()
    };
    let before = bloods(&game);

    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == reflection
                    && targets
                        .iter()
                        .any(|slot| slot.targets().contains(&Target::Permanent(epicure)))
            }
            _ => false,
        })
        .expect("the Epicure is a legal thing to copy");
    game.apply(PlayerId::One, activation)
        .expect("it is activated");
    settle(&mut game);

    assert_eq!(
        game.players[1].life,
        life - 1,
        "the copy entered and its trigger went off",
    );
    assert_eq!(bloods(&game), before + 1, "and left a Blood of its own");
}

/// The copy is what was printed: "it doesn't copy whether the creature has
/// any counters on it". A Bears grown by a counter is copied as the 2/2 the
/// card says it is.
#[test]
fn the_copy_leaves_the_counters_behind() {
    let (mut game, _fable) = staged(&[]);
    let bears = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    settle(&mut game);
    if let Some(permanent) = game
        .battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == bears)
    {
        permanent.add_counters(CounterKind::PlusOnePlusOne, 1);
    }
    let reflection = reflection_ready(&mut game);
    assert_eq!(
        game.power(
            game.battlefield
                .iter()
                .find(|permanent| permanent.card.id == bears)
                .expect("the Bears are there")
        ),
        Some(3),
        "the original is a 3/3 while the counter is on it",
    );
    let before = tokens(&game);

    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == reflection
                    && targets
                        .iter()
                        .any(|slot| slot.targets().contains(&Target::Permanent(bears)))
            }
            _ => false,
        })
        .expect("one mana and a tap copies the Bears");
    game.apply(PlayerId::One, activation)
        .expect("it is activated");
    settle(&mut game);

    let copy = tokens(&game)
        .into_iter()
        .find(|token| !before.contains(token))
        .expect("a copy arrived");
    let copy = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == copy)
        .expect("still there");
    assert_eq!(
        (game.power(copy), game.toughness(copy)),
        (Some(2), Some(2)),
        "and the copy is the card, counters and all left behind",
    );
}

/// Copies `victim` with the Reflection and returns the token that arrived.
fn copy_with(game: &mut Game, reflection: GameObjectId, victim: GameObjectId) -> GameObjectId {
    let before = tokens(game);
    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == reflection
                    && targets
                        .iter()
                        .any(|slot| slot.targets().contains(&Target::Permanent(victim)))
            }
            _ => false,
        })
        .expect("one mana and a tap copies it");
    game.apply(PlayerId::One, activation)
        .expect("it is activated");
    settle(game);
    tokens(game)
        .into_iter()
        .find(|token| !before.contains(token))
        .expect("a copy arrived")
}

/// "If the copied creature has {X} in its mana cost, X is 0." A Walking
/// Ballista wearing two counters is a 2/2, and the copy of it is a 0/0: the
/// counters are no part of what is copied and X buys nothing, so what
/// arrives is buried by state-based actions before it can be used.
#[test]
fn a_copied_x_creature_arrives_as_a_nought() {
    let (mut game, _fable) = staged(&[]);
    let reflection = reflection_ready(&mut game);
    let ballista = game
        .put_onto_battlefield(PlayerId::One, cards::WALKING_BALLISTA)
        .expect("cataloged");
    // A Ballista put onto the battlefield arrives with X of nought, so the
    // counters go on before anything checks whether a 0/0 may stay.
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == ballista)
        .expect("it is there")
        .set_counters(CounterKind::PlusOnePlusOne, 2);
    settle(&mut game);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    let before = tokens(&game);

    let activation = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == reflection
                    && targets
                        .iter()
                        .any(|slot| slot.targets().contains(&Target::Permanent(ballista)))
            }
            _ => false,
        })
        .expect("one mana and a tap copies it");
    game.apply(PlayerId::One, activation)
        .expect("it is activated");
    settle(&mut game);
    game.check_state_based_actions();

    assert_eq!(
        tokens(&game),
        before,
        "the copy was a 0/0 and did not survive being made",
    );
    assert_eq!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == ballista)
            .expect("the original is untouched")
            .counters(CounterKind::PlusOnePlusOne),
        2,
        "while the one it copied keeps what it was wearing",
    );
    assert!(
        game.battlefield
            .iter()
            .find(|permanent| permanent.card.id == reflection)
            .is_some_and(|permanent| permanent.tapped),
        "and the Reflection paid its tap for it all the same",
    );
}

/// "Sacrifice it at the beginning of the next end step" is the next one
/// there is: a copy made on their turn is gone by the end of theirs.
#[test]
fn a_copy_made_on_their_turn_dies_at_the_end_of_theirs() {
    let (mut game, _fable) = staged(&[]);
    let reflection = reflection_ready(&mut game);
    let bears = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    settle(&mut game);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.add_unrestricted_mana(PlayerId::One, ManaColor::Red, 1);
    game.active_player = PlayerId::Two;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;

    let copy = copy_with(&mut game, reflection, bears);
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == copy),
        "the copy arrived on their turn",
    );

    game.step = Step::End;
    game.begin_step_triggers();
    settle(&mut game);

    assert!(
        game.battlefield
            .iter()
            .all(|permanent| permanent.card.id != copy),
        "and their end step is the next one, so it is sacrificed there",
    );
}

/// "Sacrifice it at the beginning of the next end step" is read from where
/// the ability resolved. Copying during an end step that has already begun
/// puts the next such beginning on the other side of the table, so the token
/// survives your own end step and stands through the opponent's whole turn
/// -- which is a blocker, and the reason to spend the Reflection late.
#[test]
fn a_copy_made_in_an_end_step_lives_until_the_next_one() {
    let (mut game, _saga) = staged(&[]);
    let reflection = reflection_ready(&mut game);
    let bears = game
        .put_onto_battlefield(PlayerId::One, cards::GRIZZLY_BEARS)
        .expect("cataloged");
    drain_pending(&mut game);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }

    // Your own end step is under way before anything is copied.
    game.step = Step::End;
    game.begin_step_triggers();
    settle(&mut game);
    let copy = copy_with(&mut game, reflection, bears);

    let alive = |game: &Game| {
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == copy)
    };

    // That beginning came and went before the delayed sacrifice existed, so
    // finishing the step it arrived in takes nothing.
    game.check_state_based_actions();
    assert!(alive(&game), "this end step is not the one that claims it");

    // Their whole turn, stepped through by hand: `next_turn` would carry
    // past their end step and the token would already be gone.
    game.turn += 1;
    game.active_player = PlayerId::Two;
    game.turns_started[PlayerId::Two.index()] += 1;
    for step in [
        Step::Upkeep,
        Step::Draw,
        Step::PrecombatMain,
        Step::DeclareAttackers,
    ] {
        game.step = step;
        game.begin_step_triggers();
        settle(&mut game);
        game.check_state_based_actions();
        assert!(alive(&game), "still standing at {step:?}, ready to block");
    }

    game.step = Step::End;
    game.begin_step_triggers();
    settle(&mut game);
    game.check_state_based_actions();

    assert!(
        !alive(&game),
        "and their end step is the next one, which takes it",
    );
}
