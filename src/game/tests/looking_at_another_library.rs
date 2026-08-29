//! Looking at the top of a library that is not yours.
//!
//! Digging through your own library names one player twice, so the decision
//! and the library were the same thing. A spy separates them: the cards come
//! off the target's library and the choice is shown to the spy's controller.
//! Nothing else about the look changes -- the cards go back where they were,
//! in the order they were in.

use super::*;

/// The definitions on top of player two's library, top-first.
fn stack_library(game: &mut Game, definitions: &[CardDefinitionId]) -> Vec<CardDefinitionId> {
    game.players[PlayerId::Two.index()].library.clear();
    // The library's last element is its top, so the caller's top-first list
    // goes in backwards.
    for (index, definition) in definitions.iter().rev().enumerate() {
        let id = 20_000 + u32::try_from(index).expect("a short library fits");
        game.players[PlayerId::Two.index()]
            .library
            .push(card(id, *definition, PlayerId::Two));
    }
    definitions.to_vec()
}

fn library_from_top(game: &Game) -> Vec<CardDefinitionId> {
    game.players[PlayerId::Two.index()]
        .library
        .iter()
        .rev()
        .map(|card| card.definition)
        .collect()
}

/// The pending decision, or nothing if none is waiting.
fn pending(game: &Game) -> Option<DecisionObservation> {
    game.pending_decisions
        .first()
        .map(|decision| decision.observation.clone())
}

/// Answers with `label`, or with nothing at all when the decision permits
/// nothing -- which is how a pure look is acknowledged.
fn answer(game: &mut Game, label: &str) {
    let observation = pending(game).expect("a decision is waiting");
    let options = if observation.maximum == 0 {
        Vec::new()
    } else {
        vec![
            observation
                .options
                .iter()
                .find(|option| option.label == label)
                .unwrap_or_else(|| panic!("no option labelled {label}"))
                .id,
        ]
    };
    game.apply(
        observation.player,
        Action::ChooseDecision {
            decision: observation.id,
            options,
        },
    )
    .expect("the choice is submitted");
}

fn spy_board() -> (Game, GameObjectId, Vec<CardDefinitionId>) {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    let spy = creature(10_000, cards::ORCISH_SPY, PlayerId::One);
    let spy_id = spy.card.id;
    game.battlefield.push(spy);
    let library = stack_library(
        &mut game,
        &[
            cards::SEDGE_TROLL,
            cards::SAVANNAH_LIONS,
            cards::SOL_RING,
            cards::MOUNTAIN,
        ],
    );
    (game, spy_id, library)
}

fn activate(game: &mut Game, source: GameObjectId) {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source: activated,
                targets,
                ..
            } => {
                *activated == source
                    && targets
                        .iter()
                        .any(|selection| selection.targets() == [Target::Player(PlayerId::Two)])
            }
            _ => false,
        })
        .expect("the ability can be aimed at the other player");
    game.apply(PlayerId::One, action)
        .expect("the ability is activated");
    settle(game);
}

/// Passes priority until something is waiting on a decision, stopping there
/// rather than answering it.
fn settle(game: &mut Game) {
    for _ in 0..8 {
        if !game.pending_decisions.is_empty()
            || (game.stack.is_empty() && game.pending_triggers.is_empty())
        {
            return;
        }
        let holder = game.priority;
        if game.apply(holder, Action::PassPriority).is_err() {
            return;
        }
    }
}

#[test]
fn the_spy_looks_and_its_controller_decides() {
    let (mut game, spy, _) = spy_board();
    activate(&mut game, spy);

    let observation = pending(&game).expect("the spy is looking");
    assert_eq!(
        observation.player,
        PlayerId::One,
        "the library is player two's, but the looking is player one's"
    );
    assert_eq!(
        observation.options.len(),
        1,
        "nothing may be taken, so there is nothing to choose between"
    );
    assert_eq!(observation.options[0].members.len(), 3, "the top three");
}

/// Looking is all it does: the same cards stay on top, in the same order.
#[test]
fn the_library_is_left_exactly_as_it_was() {
    let (mut game, spy, library) = spy_board();
    activate(&mut game, spy);
    answer(&mut game, "Put them back");
    settle(&mut game);

    assert_eq!(library_from_top(&game), library);
}

/// Visions looks deeper and then hands the caster a choice the Spy does not
/// have.
#[test]
fn visions_may_leave_the_library_alone() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 5;
    let library = stack_library(
        &mut game,
        &[
            cards::SEDGE_TROLL,
            cards::SAVANNAH_LIONS,
            cards::SOL_RING,
            cards::MOUNTAIN,
            cards::GRIZZLY_BEARS,
            cards::SERRA_ANGEL,
        ],
    );
    let spell = card(10_000, cards::VISIONS, PlayerId::One);
    let spell_id = spell.id;
    game.players[PlayerId::One.index()].hand.push(spell);
    game.players[PlayerId::One.index()].mana_pool.white = 1;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::CastSpell { card, choices, .. } => {
                *card == spell_id
                    && choices
                        .iter_targets()
                        .any(|target| *target == Target::Player(PlayerId::Two))
            }
            _ => false,
        })
        .expect("Visions can be aimed at the other player");
    game.apply(PlayerId::One, action).expect("Visions is cast");
    settle(&mut game);

    let observation = pending(&game).expect("the caster is looking");
    assert_eq!(observation.player, PlayerId::One);
    assert_eq!(observation.options[0].members.len(), 5, "the top five");

    answer(&mut game, "Put them back");
    settle(&mut game);
    answer(&mut game, "Decline");
    settle(&mut game);

    assert_eq!(
        library_from_top(&game),
        library,
        "declining the shuffle leaves what was seen where it was"
    );
}
