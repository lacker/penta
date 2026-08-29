//! A counterspell taxed by your board, and a creature that copies itself.
//!
//! Spell Rupture's tax is the greatest power among your creatures, which
//! means the caster's board rather than the victim's; both halves already
//! existed and its audit line had not caught up. Giant Adephage copies the
//! permanent the trigger came from.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::Two;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::Two;
    game.battlefield.clear();
    game
}

/// A spell of player two's on the stack, with Spell Rupture in player one's
/// hand and `power` worth of creatures under player one.
fn stacked(creatures: &[CardDefinitionId]) -> (Game, GameObjectId) {
    let mut game = ready();
    for (index, definition) in creatures.iter().enumerate() {
        game.battlefield.push(creature(
            10_000 + u32::try_from(index).expect("small"),
            *definition,
            PlayerId::One,
        ));
    }
    // The opponent has a bigger creature, which must not be what is counted.
    game.battlefield
        .push(creature(10_900, cards::GHOULTREE, PlayerId::Two));

    let bait = card(20_000, cards::GRIZZLY_BEARS, PlayerId::Two);
    let bait_id = bait.id;
    game.players[PlayerId::Two.index()].hand.push(bait);
    game.players[PlayerId::Two.index()].mana_pool.green = 1;
    game.players[PlayerId::Two.index()].mana_pool.colorless = 1;
    let cast = game
        .legal_actions(PlayerId::Two)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == bait_id))
        .expect("the bear is castable");
    game.apply(PlayerId::Two, cast).expect("it is cast");
    // The caster holds priority first; the Rupture is an answer to what is
    // already waiting on the stack.
    game.apply(PlayerId::Two, Action::PassPriority)
        .expect("priority passes to the opponent");

    let rupture = card(20_001, cards::SPELL_RUPTURE, PlayerId::One);
    let rupture_id = rupture.id;
    game.players[PlayerId::One.index()].hand.push(rupture);
    game.players[PlayerId::One.index()].mana_pool.blue = 1;
    game.players[PlayerId::One.index()].mana_pool.colorless = 1;
    (game, rupture_id)
}

/// Casts the Rupture at the waiting spell and answers the tax.
fn rupture_and_answer(game: &mut Game, rupture: GameObjectId, pay: bool) -> Option<usize> {
    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| matches!(action, Action::CastSpell { card, .. } if *card == rupture))
        .expect("there is a spell to counter");
    game.apply(PlayerId::One, action).expect("it is cast");

    let mut asked = None;
    for _ in 0..16 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            asked = Some(decision.options.len());
            let options = if pay {
                decision
                    .options
                    .last()
                    .map(|option| vec![option.id])
                    .unwrap_or_default()
            } else {
                Vec::new()
            };
            if game
                .apply(
                    decision.player,
                    Action::ChooseDecision {
                        decision: decision.id,
                        options,
                    },
                )
                .is_err()
            {
                // A tax of zero leaves nothing to decline: the only legal
                // answer is the free payment.
                let free = decision
                    .options
                    .last()
                    .map(|option| vec![option.id])
                    .unwrap_or_default();
                game.apply(
                    decision.player,
                    Action::ChooseDecision {
                        decision: decision.id,
                        options: free,
                    },
                )
                .expect("the decision accepts what it offered");
            }
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
    asked
}

/// Declining the tax counters the spell.
#[test]
fn an_unpaid_rupture_counters_the_spell() {
    let (mut game, rupture) = stacked(&[cards::SERRA_ANGEL, cards::GRIZZLY_BEARS]);
    rupture_and_answer(&mut game, rupture, false);

    assert!(
        game.battlefield.iter().all(
            |permanent| permanent.card.definition != cards::GRIZZLY_BEARS
                || permanent.controller == PlayerId::One
        ),
        "the opponent's bear never resolved",
    );
    assert_eq!(
        game.players[PlayerId::Two.index()].graveyard.len(),
        1,
        "it was countered into a graveyard",
    );
}

/// With no creatures of your own the tax is zero, so nothing is countered.
#[test]
fn an_empty_board_taxes_nothing() {
    let (mut game, rupture) = stacked(&[]);
    rupture_and_answer(&mut game, rupture, false);

    assert!(
        game.battlefield.iter().any(
            |permanent| permanent.card.definition == cards::GRIZZLY_BEARS
                && permanent.controller == PlayerId::Two
        ),
        "a tax of zero is paid by doing nothing, so the bear resolved",
    );
}

/// The Adephage copies itself, and the copy is a real 7/7 with trample.
#[test]
fn the_adephage_copies_itself_on_connection() {
    let mut game = ready();
    game.active_player = PlayerId::One;
    game.priority = PlayerId::One;
    let bug = creature(10_000, cards::GIANT_ADEPHAGE, PlayerId::One);
    let bug_id = bug.card.id;
    game.battlefield.push(bug);

    game.deal_combat_damage_to_player(bug_id, PlayerId::Two, 7);
    drain_pending(&mut game);

    let copies = game
        .battlefield
        .iter()
        .filter(|permanent| {
            Game::effective_rules_source(permanent)
                == ObjectCharacteristics::card(cards::GIANT_ADEPHAGE, CardPartId::PRIMARY)
        })
        .collect::<Vec<_>>();
    assert_eq!(copies.len(), 2, "the original and one copy");
    assert_eq!(
        copies
            .iter()
            .filter(|permanent| permanent.card.definition.is_token())
            .count(),
        1,
        "the copy is a token even though it presents Giant Adephage's characteristics",
    );
    for permanent in copies {
        assert_eq!(game.power(permanent), Some(7));
        assert!(game.permanent_has_executable_keyword(permanent, KeywordAbility::Trample));
    }
}
