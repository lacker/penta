//! Two amounts read off the game rather than printed on the card.
//!
//! Death's Presence asks the dead creature how big it was, which is only
//! answerable from last-known information. Ajani's ultimate asks its
//! controller's life total, which is a number rather than the threshold
//! comparisons that already existed.

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

fn counters(game: &Game, id: GameObjectId) -> u16 {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still there")
        .counters(CounterKind::PlusOnePlusOne)
}

/// Answers the target choice by naming `wanted`, then resolves.
fn settle_naming(game: &mut Game, wanted: GameObjectId) {
    for _ in 0..12 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let option = decision
                .options
                .iter()
                .find(|option| option.card.map(|(card, _)| card) == Some(wanted))
                .or_else(|| decision.options.first())
                .map(|option| option.id);
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options: option.into_iter().collect(),
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

/// A 4/4 that dies hands four counters to whatever is left.
#[test]
fn deaths_presence_moves_the_dead_creatures_power() {
    let mut game = ready();
    game.battlefield
        .push(creature(10_000, cards::DEATHS_PRESENCE, PlayerId::One));
    let angel = creature(10_001, cards::SERRA_ANGEL, PlayerId::One);
    let angel_id = angel.card.id;
    game.battlefield.push(angel);
    let survivor = creature(10_002, cards::GRIZZLY_BEARS, PlayerId::One);
    let survivor_id = survivor.card.id;
    game.battlefield.push(survivor);

    game.move_permanents_to_graveyard(&[angel_id]);
    settle_naming(&mut game, survivor_id);

    assert_eq!(
        counters(&game, survivor_id),
        4,
        "the Angel's four power, read after it left",
    );
}

/// A creature the opponent controls dying is somebody else's business.
#[test]
fn deaths_presence_watches_only_your_own_creatures() {
    let mut game = ready();
    game.battlefield
        .push(creature(10_000, cards::DEATHS_PRESENCE, PlayerId::One));
    let theirs = creature(10_001, cards::SERRA_ANGEL, PlayerId::Two);
    let theirs_id = theirs.card.id;
    game.battlefield.push(theirs);
    let survivor = creature(10_002, cards::GRIZZLY_BEARS, PlayerId::One);
    let survivor_id = survivor.card.id;
    game.battlefield.push(survivor);

    game.move_permanents_to_graveyard(&[theirs_id]);
    settle_naming(&mut game, survivor_id);

    assert_eq!(counters(&game, survivor_id), 0, "not your creature");
}

/// Ajani's ultimate makes one Cat per point of life, read at resolution.
#[test]
fn ajanis_ultimate_counts_your_life_total() {
    let mut game = ready();
    let ajani = creature(10_000, cards::AJANI_CALLER_OF_THE_PRIDE, PlayerId::One);
    let ajani_id = ajani.card.id;
    game.battlefield.push(ajani);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == ajani_id)
        .expect("still there")
        .set_counters(CounterKind::Loyalty, 8);
    for permanent in &mut game.battlefield {
        permanent.entered_controller_turn = 0;
    }
    game.players[PlayerId::One.index()].life = 17;
    game.players[PlayerId::Two.index()].life = 3;

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| {
            matches!(action, Action::ActivateAbility {
                source,
                ability: AbilityOrigin::Printed { ability, .. },
                ..
            } if *source == ajani_id && *ability == AbilityId(2))
        })
        .expect("the ultimate is affordable at eight loyalty");
    game.apply(PlayerId::One, action).expect("it is activated");
    drain_pending(&mut game);

    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| is_token_with(
                permanent,
                tokens::creature(&["Cat"], &[ManaColor::White], 2, 2)
            ))
            .count(),
        17,
        "your life total, not the opponent's and not a printed number",
    );
}

#[test]
fn both_cards_report_complete_coverage() {
    let catalog = poc::catalog().expect("catalog builds");
    for definition in [cards::DEATHS_PRESENCE, cards::AJANI_CALLER_OF_THE_PRIDE] {
        let card = catalog.get(definition).expect("the card is cataloged");
        assert_eq!(
            card.rules.implementation_status(),
            ImplementationStatus::Complete,
            "{} should be fully executable",
            card.name,
        );
    }
}
