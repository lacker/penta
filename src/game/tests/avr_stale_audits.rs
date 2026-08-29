//! Three Avacyn Restored cards resting on machinery that existed.
//!
//! Swampwalk, a doubled count and a dynamic generic payment were all built.
//! What is worth pinning is that two of the three say "other creatures you
//! control", which is the clause easiest to get wrong in either direction.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.turns_started[PlayerId::Two.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::PrecombatMain;
    game.priority = PlayerId::One;
    game
}

/// Answers each waiting decision by taking the option at `index`, clamped to
/// what is on offer.
fn drain_choosing(game: &mut Game, index: usize) {
    for _ in 0..16 {
        if game.stack.is_empty()
            && game.pending_triggers.is_empty()
            && game.pending_decisions.is_empty()
        {
            return;
        }
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let pick = index.min(decision.options.len().saturating_sub(1));
            game.apply(
                decision.player,
                Action::ChooseDecision {
                    decision: decision.id,
                    options: vec![decision.options[pick].id],
                },
            )
            .expect("the decision accepts what it offered");
            continue;
        }
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            return;
        }
    }
}

#[test]
fn the_explorer_has_swampwalk() {
    let mut game = ready();
    let explorer = creature(10_000, cards::FARBOG_EXPLORER, PlayerId::One);
    let explorer_id = explorer.card.id;
    game.battlefield.push(explorer);

    let permanent = game
        .battlefield
        .iter()
        .find(|permanent| permanent.card.id == explorer_id)
        .expect("still there");
    assert!(game.permanent_has_executable_keyword(
        permanent,
        KeywordAbility::Landwalk(BasicLandType::Swamp),
    ));
}

/// Two per *other* creature, so the Redeemer's own arrival is not counted.
#[test]
fn the_redeemer_gains_two_for_each_other_creature() {
    let mut game = ready();
    for index in 0..2 {
        game.battlefield.push(creature(
            10_000 + index,
            cards::GRIZZLY_BEARS,
            PlayerId::One,
        ));
    }
    game.battlefield
        .push(creature(10_100, cards::GRIZZLY_BEARS, PlayerId::Two));
    let before = game.players[PlayerId::One.index()].life;

    game.enqueue_battlefield_entry(PendingBattlefieldEntry {
        permanent: creature(10_200, cards::GOLDNIGHT_REDEEMER, PlayerId::One),
        from: ZoneKind::Hand,
        completion: EntryCompletion::None,
        redirected_to: None,
    });
    drain_pending(&mut game);

    assert_eq!(
        game.players[PlayerId::One.index()].life,
        before + 4,
        "two of mine at two apiece; theirs and itself excluded",
    );
}

/// The Fettergeist's tax counts other creatures, so alone the payment is
/// zero. The choice is still offered -- "unless you pay {0}" is a real
/// decision -- but taking it costs nothing.
#[test]
fn a_lone_fettergeist_pays_nothing() {
    let mut game = ready();
    let geist = creature(10_000, cards::FETTERGEIST, PlayerId::One);
    let geist_id = geist.card.id;
    game.battlefield.push(geist);

    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    drain_choosing(&mut game, usize::MAX);

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == geist_id),
        "an empty pool still covered an empty cost",
    );
    assert_eq!(game.players[PlayerId::One.index()].mana_pool.colorless, 0);
}

/// With company it costs one per other creature, and declining sacrifices it.
#[test]
fn a_crowded_fettergeist_costs_one_each() {
    let mut game = ready();
    let geist = creature(10_000, cards::FETTERGEIST, PlayerId::One);
    let geist_id = geist.card.id;
    game.battlefield.push(geist);
    for index in 0..2 {
        game.battlefield.push(creature(
            10_100 + index,
            cards::GRIZZLY_BEARS,
            PlayerId::One,
        ));
    }
    game.players[PlayerId::One.index()].mana_pool.colorless = 2;

    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    drain_choosing(&mut game, usize::MAX);

    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == geist_id),
        "two mana covered two other creatures",
    );
    assert_eq!(
        game.players[PlayerId::One.index()].mana_pool.colorless,
        0,
        "and both were spent",
    );
}

/// The control: declining lets it go.
#[test]
fn declining_the_fettergeist_tax_sacrifices_it() {
    let mut game = ready();
    let geist = creature(10_000, cards::FETTERGEIST, PlayerId::One);
    let geist_id = geist.card.id;
    game.battlefield.push(geist);
    game.battlefield
        .push(creature(10_100, cards::GRIZZLY_BEARS, PlayerId::One));
    game.players[PlayerId::One.index()].mana_pool.colorless = 5;

    game.step = Step::Upkeep;
    game.handle_upkeep_triggers();
    drain_choosing(&mut game, 0);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == geist_id),
    );
    assert_eq!(
        game.players[PlayerId::One.index()].mana_pool.colorless,
        5,
        "and nothing was spent",
    );
}
