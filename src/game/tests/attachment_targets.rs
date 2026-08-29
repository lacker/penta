//! Targeting by what a permanent is attached to, in both directions.
//!
//! Ramses Overdark asks the host's question -- is anything on it? -- and the
//! two Aura-breakers ask the Aura's: what is it on? The pair matters because
//! an Aura and its host are both permanents, so a predicate that confused
//! the two would still find something to destroy.

use super::*;

/// Puts `aura` onto `host` and settles attachment legality.
fn attach(
    game: &mut Game,
    id: u32,
    aura: CardDefinitionId,
    host: GameObjectId,
    controller: PlayerId,
) -> GameObjectId {
    let mut permanent = creature(id, aura, controller);
    permanent.attached_to = Some(host);
    let aura_id = permanent.card.id;
    game.battlefield.push(permanent);
    game.check_state_based_actions();
    aura_id
}

/// Which permanents this source's ability may be pointed at.
fn targets_of(game: &Game, source: GameObjectId) -> Vec<GameObjectId> {
    game.legal_actions(PlayerId::One)
        .into_iter()
        .filter_map(|action| match action {
            Action::ActivateAbility {
                source: from,
                targets,
                ..
            } if from == source => targets
                .iter()
                .flat_map(crate::casting::TargetSelection::targets)
                .find_map(|target| match target {
                    Target::Permanent(id) => Some(*id),
                    _ => None,
                }),
            _ => None,
        })
        .collect()
}

#[test]
fn ramses_overdark_names_the_host_and_not_the_aura() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 1;
    let ramses = creature(10_000, cards::RAMSES_OVERDARK, PlayerId::One);
    let ramses_id = ramses.card.id;
    game.battlefield.push(ramses);

    let host = creature(10_001, cards::SEDGE_TROLL, PlayerId::Two);
    let host_id = host.card.id;
    game.battlefield.push(host);
    let bare = creature(10_002, cards::SEDGE_TROLL, PlayerId::Two);
    let bare_id = bare.card.id;
    game.battlefield.push(bare);

    assert!(
        targets_of(&game, ramses_id).is_empty(),
        "nothing is enchanted yet"
    );

    let aura_id = attach(
        &mut game,
        10_003,
        cards::UNHOLY_STRENGTH,
        host_id,
        PlayerId::Two,
    );

    let targets = targets_of(&game, ramses_id);
    assert!(targets.contains(&host_id), "the enchanted creature");
    assert!(!targets.contains(&bare_id), "and not the bare one");
    assert!(!targets.contains(&aura_id), "nor the Aura itself");
}

#[test]
fn savaen_elves_names_the_aura_and_not_its_host() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 1;
    let elves = creature(10_000, cards::SAVAEN_ELVES, PlayerId::One);
    let elves_id = elves.card.id;
    game.battlefield.push(elves);
    game.players[PlayerId::One.index()].mana_pool.green = 2;

    let land = creature(10_001, cards::FOREST, PlayerId::Two);
    let land_id = land.card.id;
    game.battlefield.push(land);
    let creature_host = creature(10_002, cards::SEDGE_TROLL, PlayerId::Two);
    let creature_host_id = creature_host.card.id;
    game.battlefield.push(creature_host);

    let on_a_creature = attach(
        &mut game,
        10_003,
        cards::UNHOLY_STRENGTH,
        creature_host_id,
        PlayerId::Two,
    );
    let on_a_land = attach(
        &mut game,
        10_004,
        cards::PSYCHIC_VENOM,
        land_id,
        PlayerId::Two,
    );

    let targets = targets_of(&game, elves_id);
    assert!(targets.contains(&on_a_land), "the Aura on a land");
    assert!(
        !targets.contains(&on_a_creature),
        "and not the one on a creature"
    );
    assert!(!targets.contains(&land_id), "nor the land under it");
}

/// Miracle Worker cares whose creature the Aura sits on, not whose Aura it
/// is: an opponent's Aura on your creature is exactly what it answers.
#[test]
fn miracle_worker_reads_the_hosts_controller() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 1;
    let worker = creature(10_000, cards::MIRACLE_WORKER, PlayerId::One);
    let worker_id = worker.card.id;
    game.battlefield.push(worker);

    let mine = creature(10_001, cards::SEDGE_TROLL, PlayerId::One);
    let mine_id = mine.card.id;
    game.battlefield.push(mine);
    let theirs = creature(10_002, cards::SEDGE_TROLL, PlayerId::Two);
    let theirs_id = theirs.card.id;
    game.battlefield.push(theirs);

    let on_mine = attach(
        &mut game,
        10_003,
        cards::UNHOLY_STRENGTH,
        mine_id,
        PlayerId::Two,
    );
    let on_theirs = attach(
        &mut game,
        10_004,
        cards::UNHOLY_STRENGTH,
        theirs_id,
        PlayerId::Two,
    );

    let targets = targets_of(&game, worker_id);
    assert!(
        targets.contains(&on_mine),
        "their Aura on my creature is fair game"
    );
    assert!(
        !targets.contains(&on_theirs),
        "and their Aura on their own creature is not"
    );
}

/// And it destroys what it names.
#[test]
fn destroying_the_aura_leaves_the_host_alone() {
    let mut game = ready_game();
    game.turns_started[PlayerId::One.index()] = 1;
    let worker = creature(10_000, cards::MIRACLE_WORKER, PlayerId::One);
    let worker_id = worker.card.id;
    game.battlefield.push(worker);
    let mine = creature(10_001, cards::SEDGE_TROLL, PlayerId::One);
    let mine_id = mine.card.id;
    game.battlefield.push(mine);
    let aura = attach(
        &mut game,
        10_002,
        cards::UNHOLY_STRENGTH,
        mine_id,
        PlayerId::Two,
    );

    let action = game
        .legal_actions(PlayerId::One)
        .into_iter()
        .find(|action| match action {
            Action::ActivateAbility {
                source, targets, ..
            } => {
                *source == worker_id
                    && targets
                        .iter()
                        .flat_map(crate::casting::TargetSelection::targets)
                        .any(|target| *target == Target::Permanent(aura))
            }
            _ => false,
        })
        .expect("the Worker can name that Aura");
    game.apply(PlayerId::One, action)
        .expect("the ability activates");
    drain_pending(&mut game);

    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == aura),
        "the Aura is gone"
    );
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == mine_id),
        "and the creature it was on is not"
    );
}
