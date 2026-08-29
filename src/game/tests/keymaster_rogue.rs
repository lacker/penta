//! A bounce that is chosen rather than targeted, and mandatory.
//!
//! No target slot: nothing can be done in response to protect what it will
//! name, and the choice is made as the trigger resolves. A minimum of one
//! means it cannot be answered with nothing -- with no other creature out,
//! the Rogue returns itself.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game.battlefield.clear();
    game.players[PlayerId::One.index()].hand.clear();
    game
}

/// Resolves the entry trigger, naming `wanted` when it is on offer.
fn settle_naming(game: &mut Game, wanted: Option<CardDefinitionId>) -> Option<Vec<String>> {
    let mut offered = None;
    for _ in 0..12 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            offered = Some(
                decision
                    .options
                    .iter()
                    .map(|option| option.label.clone())
                    .collect::<Vec<_>>(),
            );
            let chosen = wanted
                .and_then(|definition| {
                    decision.options.iter().find(|option| {
                        option.card.and_then(|(_, actual)| actual.card_definition())
                            == Some(definition)
                    })
                })
                .or_else(|| decision.options.first())
                .map(|option| option.id);
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options: chosen.into_iter().collect(),
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
    offered
}

fn on_battlefield(game: &Game, definition: CardDefinitionId) -> bool {
    game.battlefield
        .iter()
        .any(|permanent| permanent.card.definition == definition)
}

/// The bounce reaches another creature you control, and only yours.
#[test]
fn it_returns_a_creature_you_control() {
    let mut game = ready();
    game.battlefield
        .push(creature(10_000, cards::GRIZZLY_BEARS, PlayerId::One));
    game.battlefield
        .push(creature(10_001, cards::SERRA_ANGEL, PlayerId::Two));
    game.put_onto_battlefield(PlayerId::One, cards::KEYMASTER_ROGUE)
        .expect("cataloged");

    let offered = settle_naming(&mut game, Some(cards::GRIZZLY_BEARS));

    assert_eq!(
        offered.map(|labels| labels.len()),
        Some(2),
        "your bear and the Rogue itself, never the opponent's Angel",
    );
    assert!(
        !on_battlefield(&game, cards::GRIZZLY_BEARS),
        "the bear went"
    );
    assert!(
        on_battlefield(&game, cards::KEYMASTER_ROGUE),
        "the Rogue stayed"
    );
    assert!(
        on_battlefield(&game, cards::SERRA_ANGEL),
        "and the opponent's creature was never on offer",
    );
}

/// With nothing else out it has to name itself, which is what makes the
/// bounce a cost rather than a bonus.
#[test]
fn alone_it_returns_itself() {
    let mut game = ready();
    game.put_onto_battlefield(PlayerId::One, cards::KEYMASTER_ROGUE)
        .expect("cataloged");

    // Forced, so nothing is asked: one candidate and a minimum of one.
    assert_eq!(settle_naming(&mut game, None), None);

    assert!(!on_battlefield(&game, cards::KEYMASTER_ROGUE));
    assert_eq!(
        game.players[PlayerId::One.index()].hand.len(),
        1,
        "and it is back in hand",
    );
}

/// The evasion is a real static, not a combat special case.
#[test]
fn it_cannot_be_blocked() {
    let mut game = ready();
    let rogue = creature(10_000, cards::KEYMASTER_ROGUE, PlayerId::One);
    let rogue_id = rogue.card.id;
    game.battlefield.push(rogue);
    // A flier, so it could block a flying attacker too: only "can't be
    // blocked" explains the refusal below.
    let blocker = creature(10_001, cards::SERRA_ANGEL, PlayerId::Two);
    let blocker_id = blocker.card.id;
    game.battlefield.push(blocker);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == rogue_id)
        .expect("still there")
        .attacking = true;
    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.priority = PlayerId::Two;

    // A second attacker the same blocker may take, so the refusal above is
    // about the Rogue rather than about the blocker or the step.
    let ordinary = creature(10_002, cards::GHOULTREE, PlayerId::One);
    let ordinary_id = ordinary.card.id;
    game.battlefield.push(ordinary);
    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == ordinary_id)
        .expect("still there")
        .attacking = true;

    let offered = |attacker| {
        game.legal_actions(PlayerId::Two)
            .contains(&Action::DeclareBlocker {
                blocker: blocker_id,
                attacker,
            })
    };
    assert!(!offered(rogue_id), "nothing may block it");
    assert!(offered(ordinary_id), "though that blocker can block");
}
