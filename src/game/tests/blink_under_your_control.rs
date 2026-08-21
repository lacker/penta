//! Blinking a creature back under *your* control.
//!
//! An ordinary return puts a card back under its owner. These two say "under
//! your control", and the two answers differ exactly when the creature was
//! stolen -- which is the case worth testing, because it is also the reason
//! to play a blink in the first place.

use super::*;
use crate::ImplementationStatus;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.battlefield.clear();
    game
}

/// A creature owned by player two but controlled by player one, which is
/// what a stolen creature looks like.
fn stolen_bear(id: u32) -> Permanent {
    let mut permanent = creature(id, cards::GRIZZLY_BEARS, PlayerId::Two);
    permanent.controller = PlayerId::One;
    permanent.entered_controller_turn = 0;
    permanent
}

fn controller_of(game: &Game, definition: CardDefinitionId) -> Option<PlayerId> {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.definition == definition)
        .map(|permanent| permanent.controller)
}

/// Resolves everything, taking the last option of any decision.
fn settle(game: &mut Game) {
    for _ in 0..16 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = decision
                .options
                .last()
                .map(|option| vec![option.id])
                .unwrap_or_default();
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options,
                },
            )
            .expect("the decision accepts what it offered");
            continue;
        }
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
}

/// The Smuggler blinks a stolen bear and keeps it.
#[test]
fn the_smuggler_keeps_what_it_blinks() {
    let mut game = ready();
    let smuggler = creature(10_000, cards::NEPHALIA_SMUGGLER, PlayerId::One);
    let smuggler_id = smuggler.card.id;
    game.battlefield.push(smuggler);
    let bear = stolen_bear(10_001);
    let bear_id = bear.card.id;
    game.battlefield.push(bear);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.players[PlayerId::One.index()].mana_pool.blue = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 3;

    assert_eq!(
        controller_of(&game, cards::GRIZZLY_BEARS),
        Some(PlayerId::One)
    );

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == smuggler_id
                    && targets
                        .iter()
                        .any(|selection| selection.targets().contains(&Target::Permanent(bear_id)))
            }
            _ => false,
        })
        .expect("the bear can be named");
    game.apply(PlayerId::One, action).expect("it is activated");
    settle(&mut game);

    assert_eq!(
        controller_of(&game, cards::GRIZZLY_BEARS),
        Some(PlayerId::One),
        "the bear came back under the blinker, not its owner",
    );
}

/// The Closet does the same on its own end step.
#[test]
fn the_closet_keeps_what_it_blinks() {
    let mut game = ready();
    game.battlefield
        .push(creature(10_000, cards::CONJURERS_CLOSET, PlayerId::One));
    game.battlefield.push(stolen_bear(10_001));
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }

    game.step = Step::End;
    game.capture_battlefield_triggers(&CommittedTriggerEvent::StepBegins {
        step: TurnStepDef::End,
        player: PlayerId::One,
    });
    settle(&mut game);

    assert_eq!(
        controller_of(&game, cards::GRIZZLY_BEARS),
        Some(PlayerId::One),
        "still yours after the blink",
    );
}

/// The control clause is what does it: an ordinary linked return hands the
/// same stolen bear back to its owner.
#[test]
fn an_ordinary_return_gives_it_back_to_its_owner() {
    let mut game = ready();
    // Straight into exile owned by player two, which is where the blink
    // leaves it mid-resolution.
    let bear = stolen_bear(10_001);
    let bear_id = bear.card.id;
    game.players[PlayerId::Two.index()]
        .exile
        .push(bear.card.into_card().expect("the fixture is a card"));

    game.return_exiled_card(bear_id, ZoneKind::Battlefield, None, None, false, None);

    assert_eq!(
        controller_of(&game, cards::GRIZZLY_BEARS),
        Some(PlayerId::Two),
        "without the clause it goes home",
    );
}

#[test]
fn both_blinkers_report_complete_coverage() {
    let catalog = poc::catalog().expect("catalog builds");
    for definition in [cards::NEPHALIA_SMUGGLER, cards::CONJURERS_CLOSET] {
        let card = catalog.get(definition).expect("the card is cataloged");
        assert_eq!(
            card.rules.implementation_status(),
            ImplementationStatus::Complete,
            "{} should be fully executable",
            card.name,
        );
    }
}
