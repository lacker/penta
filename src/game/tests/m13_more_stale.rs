//! Three more Magic 2013 cards whose audit lines had gone stale.
//!
//! Exalted and regeneration were both built; the entering creature's power
//! is a trigger value that exists; and a mana ability can now read a
//! battlefield count as well as a counter count, since either is knowable
//! before the ability is activated.

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

/// Queues an arrival without answering anything, so the caller decides how
/// any resulting "you may" is answered.
fn arrive(game: &mut Game, id: u32, definition: CardDefinitionId, controller: PlayerId) {
    game.enqueue_battlefield_entry(PendingBattlefieldEntry {
        permanent: creature(id, definition, controller),
        from: ZoneKind::Hand,
        completion: EntryCompletion::None,
        redirected_to: None,
    });
}

/// Answers each waiting decision by taking the last option, which for a
/// "you may" is the branch that accepts.
fn drain_accepting(game: &mut Game) {
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
            let take = decision.minimum.max(1).min(decision.maximum);
            let options = decision
                .options
                .iter()
                .rev()
                .map(|option| option.id)
                .take(take)
                .collect::<Vec<_>>();
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
        let player = game.priority;
        if game.apply(player, Action::PassPriority).is_err() {
            return;
        }
    }
}

fn counters(game: &Game, id: GameObjectId) -> u16 {
    game.battlefield
        .iter()
        .find(|permanent| permanent.card.id == id)
        .expect("still there")
        .counters(CounterKind::PlusOnePlusOne)
}

#[test]
fn duty_bound_dead_has_exalted_and_can_regenerate() {
    let mut game = ready();
    let dead = creature(10_000, cards::DUTY_BOUND_DEAD, PlayerId::One);
    let dead_id = dead.card.id;
    game.battlefield.push(dead);

    // Exalted is a triggered ability rather than a keyword, and has its own
    // test module; what this one checks is the regeneration beside it.
    game.players[PlayerId::One.index()].mana_pool.black = 1;
    assert!(
        !game.legal_actions(PlayerId::One).iter().any(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == dead_id)
        ),
        "one black is three short",
    );

    game.players[PlayerId::One.index()].mana_pool.colorless = 3;
    assert!(
        game.legal_actions(PlayerId::One).iter().any(
            |action| matches!(action, Action::ActivateAbility { source, .. } if *source == dead_id)
        ),
        "four mana buys the regeneration",
    );
}

/// The Goliath reads the entering creature's power, not a constant, and
/// takes arrivals from either side.
#[test]
fn the_goliath_counts_the_entering_creatures_power() {
    let mut game = ready();
    let goliath = creature(10_000, cards::HAMLETBACK_GOLIATH, PlayerId::One);
    let goliath_id = goliath.card.id;
    game.battlefield.push(goliath);

    // Air Elemental is a 4/4.
    arrive(&mut game, 10_100, cards::AIR_ELEMENTAL, PlayerId::One);
    drain_accepting(&mut game);
    assert_eq!(counters(&game, goliath_id), 4);

    // Grizzly Bears is a 2/2, and theirs feeds it just the same.
    arrive(&mut game, 10_101, cards::GRIZZLY_BEARS, PlayerId::Two);
    drain_accepting(&mut game);
    assert_eq!(counters(&game, goliath_id), 6, "four then two");
}

/// The Archdruid counts itself, so a lone one taps for a single green.
#[test]
fn the_archdruid_taps_for_one_green_per_elf() {
    let mut game = ready();
    let druid = creature(10_000, cards::ELVISH_ARCHDRUID, PlayerId::One);
    let druid_id = druid.card.id;
    game.battlefield.push(druid);

    let tap = |game: &mut Game| {
        let before = game.players[PlayerId::One.index()].mana_pool.green;
        let action = game
            .legal_actions(PlayerId::One)
            .into_iter()
            .find(|action| {
                matches!(action, Action::ActivateManaAbility { source, .. } if *source == druid_id)
            })
            .expect("an untapped Archdruid offers its mana ability");
        game.apply(PlayerId::One, action).expect("tapping is free");
        drain_pending(game);
        game.players[PlayerId::One.index()].mana_pool.green - before
    };

    assert_eq!(tap(&mut game), 1, "itself and nothing else");

    game.battlefield
        .iter_mut()
        .find(|permanent| permanent.card.id == druid_id)
        .expect("still there")
        .tapped = false;
    for index in 0..2 {
        game.battlefield.push(creature(
            10_100 + index,
            cards::LLANOWAR_ELVES,
            PlayerId::One,
        ));
    }
    game.priority = PlayerId::One;

    assert_eq!(tap(&mut game), 3, "itself and two more");
}

/// "Other Elf creatures you control", so the Archdruid does not pump itself.
#[test]
fn the_archdruid_pumps_other_elves_only() {
    let mut game = ready();
    let druid = creature(10_000, cards::ELVISH_ARCHDRUID, PlayerId::One);
    let druid_id = druid.card.id;
    game.battlefield.push(druid);
    let elf = creature(10_100, cards::LLANOWAR_ELVES, PlayerId::One);
    let elf_id = elf.card.id;
    game.battlefield.push(elf);

    let stats = |game: &Game, id: GameObjectId| {
        let permanent = game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == id)
            .expect("still there");
        (game.power(permanent), game.toughness(permanent))
    };
    assert_eq!(stats(&game, elf_id), (Some(2), Some(2)), "a 1/1 plus one");
    assert_eq!(stats(&game, druid_id), (Some(2), Some(2)), "printed size");
}
