//! Two cards built on the declined branch of an optional sacrifice.
//!
//! "Unless you sacrifice ..." is one offer with two branches rather than a
//! payment and a separate check, so the toll falls both when the player says
//! no and when there is nothing to say yes with. Elder Spawn names a land
//! type; Curse Artifact names exactly the permanent it is attached to, and
//! asks the permanent's controller rather than its own.

use super::*;

fn ready() -> Game {
    let mut game = ready_game();
    game.turn = 5;
    game.turns_started[PlayerId::One.index()] = 5;
    game.active_player = PlayerId::One;
    game.step = Step::Upkeep;
    game.priority = PlayerId::One;
    game.battlefield.clear();
    game
}

fn spawn_upkeep(islands: usize) -> Game {
    let mut game = ready();
    game.battlefield
        .push(creature(10_000, cards::ELDER_SPAWN, PlayerId::One));
    for _ in 0..islands {
        game.put_onto_battlefield(PlayerId::One, cards::ISLAND)
            .expect("cataloged");
    }
    game.handle_upkeep_triggers();
    game
}

fn spawn_survived(game: &Game) -> bool {
    game.battlefield
        .iter()
        .any(|permanent| permanent.card.definition == cards::ELDER_SPAWN)
}

/// Answers the pending offer, taking the first option to decline and the
/// last to pay.
fn answer(game: &mut Game, pay: bool) {
    for _ in 0..16 {
        if let Some(decision) = game
            .pending_decisions
            .first()
            .map(|pending| pending.observation.clone())
        {
            let options = if pay {
                decision
                    .options
                    .last()
                    .map(|option| vec![option.id])
                    .unwrap_or_default()
            } else {
                Vec::new()
            };
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

/// Paying keeps the Spawn and costs an Island.
#[test]
fn sacrificing_an_island_keeps_the_spawn() {
    let mut game = spawn_upkeep(2);
    answer(&mut game, true);

    assert!(spawn_survived(&game), "the toll was paid");
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| permanent.card.definition == cards::ISLAND)
            .count(),
        1,
        "one Island went",
    );
    assert_eq!(game.players[0].life, 20, "and no damage");
}

/// Declining takes the Spawn and six life.
#[test]
fn declining_costs_the_spawn_and_six_life() {
    let mut game = spawn_upkeep(2);
    answer(&mut game, false);

    assert!(!spawn_survived(&game), "the Spawn went instead");
    assert_eq!(
        game.battlefield
            .iter()
            .filter(|permanent| permanent.card.definition == cards::ISLAND)
            .count(),
        2,
        "both Islands stayed",
    );
    assert_eq!(game.players[0].life, 14, "and six damage landed");
}

/// No Island is a declined offer, not a skipped one -- and it is never asked.
#[test]
fn no_island_pays_the_toll_without_asking() {
    let mut game = spawn_upkeep(0);
    assert!(
        game.pending_decisions.is_empty(),
        "nothing to choose between",
    );
    answer(&mut game, true);

    assert!(!spawn_survived(&game), "the toll fell anyway");
    assert_eq!(game.players[0].life, 14);
}

/// The other clause: a red creature cannot block it.
#[test]
fn red_creatures_cannot_block_the_spawn() {
    let mut game = ready();
    let mut spawn = creature(10_000, cards::ELDER_SPAWN, PlayerId::One);
    spawn.attacking = true;
    let spawn_id = spawn.card.id;
    game.battlefield.push(spawn);
    let red = creature(10_100, cards::GOBLINS_OF_THE_FLARG, PlayerId::Two);
    let red_id = red.card.id;
    game.battlefield.push(red);
    let blue = creature(10_101, cards::AIR_ELEMENTAL, PlayerId::Two);
    let blue_id = blue.card.id;
    game.battlefield.push(blue);

    game.step = Step::DeclareBlockers;
    game.attackers_declared = true;
    game.priority = PlayerId::Two;

    let offered = |game: &Game, blocker: GameObjectId| {
        game.legal_actions(PlayerId::Two)
            .contains(&Action::DeclareBlocker {
                blocker,
                attacker: spawn_id,
            })
    };
    assert!(!offered(&game, red_id), "red cannot block it");
    assert!(offered(&game, blue_id), "blue can");
}

/// Curse Artifact offers the same choice to the artifact's controller, not
/// the Aura's, and names exactly the enchanted artifact.
fn cursed_upkeep() -> (Game, GameObjectId) {
    let mut game = ready();
    let artifact = creature(10_000, cards::ORNITHOPTER, PlayerId::Two);
    let artifact_id = artifact.card.id;
    game.battlefield.push(artifact);
    let mut aura = creature(10_001, cards::CURSE_ARTIFACT, PlayerId::One);
    aura.attached_to = Some(artifact_id);
    game.battlefield.push(aura);
    // A second artifact the same player controls, which the offer must not
    // reach: the clause names "that artifact".
    game.battlefield
        .push(creature(10_002, cards::ORNITHOPTER, PlayerId::Two));

    game.active_player = PlayerId::Two;
    game.handle_upkeep_triggers();
    (game, artifact_id)
}

#[test]
fn the_curse_offers_only_the_artifact_it_is_on() {
    let (mut game, artifact) = cursed_upkeep();
    for _ in 0..8 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        let player = game.priority;
        if game.stack.is_empty() && game.pending_triggers.is_empty() {
            break;
        }
        if game.apply(player, Action::PassPriority).is_err() {
            break;
        }
    }
    let decision = game
        .pending_decisions
        .first()
        .map(|pending| pending.observation.clone())
        .expect("the offer was made");
    assert_eq!(decision.player, PlayerId::Two, "the artifact's controller");
    assert_eq!(
        decision.options.len(),
        1,
        "one artifact named, not every artifact they control",
    );
    assert_eq!(
        decision.options[0].card.map(|(card, _)| card),
        Some(artifact),
    );
}

/// Declining takes two damage; sacrificing takes the artifact instead.
#[test]
fn the_curse_charges_two_unless_the_artifact_goes() {
    let (mut game, artifact) = cursed_upkeep();
    answer(&mut game, false);
    assert_eq!(game.players[1].life, 18, "declined, so two damage");
    assert!(
        game.battlefield
            .iter()
            .any(|permanent| permanent.card.id == artifact),
        "and the artifact stayed",
    );

    let (mut game, artifact) = cursed_upkeep();
    answer(&mut game, true);
    assert_eq!(game.players[1].life, 20, "paid, so no damage");
    assert!(
        !game
            .battlefield
            .iter()
            .any(|permanent| permanent.card.id == artifact),
        "and the artifact went",
    );
}
