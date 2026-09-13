//! Reconstruction has to survive being played, not just being inspected.
//!
//! The boundary audit proves a rebuilt game *looks* right at the instant it is
//! built. That is not what a search bot uses it for: it uses it to play
//! forward, and a field that round-trips subtly wrong can produce an identical
//! observation now and a different game five actions later. Drift like that is
//! invisible to a single-boundary check and is exactly what would poison a
//! rollout, so this walks both games forward in lockstep instead.
//!
//! Object identity is deliberately not compared literally. Hypothesized hidden
//! cards are minted with fresh ids by design, so the moment either game draws,
//! the raw ids stop matching and should. What must hold is that the two views
//! agree up to a consistent renaming, which is what canonicalizing ids by
//! first appearance checks.

use super::super::*;
use super::true_hidden_hypothesis;
use crate::policy::Policy;
use std::collections::BTreeMap;

/// Fields whose integer values are object ids. Anything not listed here is
/// compared literally, so a missed field makes this test too strict rather
/// than too lax -- it would fail loudly instead of passing over a real
/// difference.
const ID_FIELDS: &[&str] = &[
    "objectId",
    "instance",
    "stackId",
    "card",
    "source",
    "sourceObjectId",
    "attacker",
    "blocker",
    "costObject",
];

/// The same, for fields holding a list of bare object ids.
const ID_LIST_FIELDS: &[&str] = &[
    "blocking",
    "cards",
    "chosenPermanents",
    "costObjects",
    "permanents",
    "sacrifices",
];

#[test]
fn canonical_blocking_references_preserve_the_attacker_relationship() {
    let host = serde_json::json!({
        "battlefield": [
            { "objectId": 185, "blocking": [] },
            { "objectId": 186, "blocking": [] },
            { "objectId": 187, "blocking": [185] }
        ]
    });
    let mut rebuilt = serde_json::json!({
        "battlefield": [
            { "objectId": 325, "blocking": [] },
            { "objectId": 326, "blocking": [] },
            { "objectId": 327, "blocking": [325] }
        ]
    });
    assert_eq!(canonical(&host, true), canonical(&rebuilt, true));

    rebuilt["battlefield"][2]["blocking"] = serde_json::json!([326]);
    assert_ne!(canonical(&host, true), canonical(&rebuilt, true));
}

#[test]
fn canonical_history_preserves_reveals_without_linking_hypothesized_hand_ids() {
    let host = serde_json::json!({
        "hand": [{ "objectId": 10, "definition": "a" }],
        "publicReveals": [
            { "seat": "p1", "objectId": 10, "definition": "a" },
            { "seat": "p1", "objectId": 10, "definition": "a" }
        ],
        "lastSeenHand": { "seat": "p1", "cards": [{ "objectId": 10, "definition": "a" }] }
    });
    let mut rebuilt = host.clone();
    rebuilt["hand"][0]["objectId"] = serde_json::json!(20);
    assert_eq!(canonical(&host, true), canonical(&rebuilt, true));

    rebuilt["publicReveals"][1]["objectId"] = serde_json::json!(11);
    assert_ne!(canonical(&host, true), canonical(&rebuilt, true));
    rebuilt["publicReveals"] = host["publicReveals"].clone();
    rebuilt["publicReveals"][0]["definition"] = serde_json::json!("b");
    assert_ne!(canonical(&host, true), canonical(&rebuilt, true));
}

#[test]
fn canonical_hand_memory_is_compared_only_when_reconstructible() {
    let host = serde_json::json!({
        "lastSeenHand": { "seat": "p1", "cards": [{ "objectId": 10, "definition": "a" }] }
    });
    let rebuilt = serde_json::json!({ "lastSeenHand": null });
    assert_eq!(canonical(&host, false), canonical(&rebuilt, false));
    assert_ne!(canonical(&host, true), canonical(&rebuilt, true));
}

#[test]
fn canonical_hand_memory_detects_a_missing_new_host_observation() {
    let initial = Some((
        PlayerId::One,
        vec![(GameObjectId(10), crate::card::cards::ISLAND)],
    ));
    let changed = Some((
        PlayerId::One,
        vec![(GameObjectId(11), crate::card::cards::FOREST)],
    ));
    assert!(!hand_memory_is_reconstructible(&initial, &initial, &None));
    assert!(hand_memory_is_reconstructible(&initial, &changed, &None));
    assert!(hand_memory_is_reconstructible(&initial, &initial, &initial));
}

#[test]
#[ignore = "slow decision-boundary reconstruction audit"]
fn sweep_a_reconstructed_game_stays_in_step_with_the_host_as_both_play_forward() {
    let catalog = crate::poc::catalog().expect("catalog builds");
    let mut trajectories = 0_usize;
    let mut steps = 0_usize;

    // Every format with a registered deck, so a format is not left walking
    // only its own card pool's happy path. Premodern carries two lists today
    // and picks up the rest as they are promoted.
    let formats = [
        crate::Format::OldSchool9394,
        crate::Format::IsdM14Standard,
        crate::Format::Premodern,
    ];
    for (format_index, format) in formats.into_iter().enumerate() {
        let decks = crate::protocol::deck_names_for_format(format);
        for (index, name) in decks.iter().enumerate() {
            let opposing = decks[(index * 5 + 2) % decks.len()];
            let seed = 90_000
                + u64::try_from(index).expect("deck index fits") * 173
                + u64::try_from(format_index).expect("format index fits") * 6_101;
            steps += walk_one_trajectory(&catalog, format, [name, opposing], seed);
            trajectories += 1;
        }
    }

    assert!(
        trajectories >= 20,
        "only {trajectories} trajectories were walked"
    );
    assert!(
        steps >= 4_000,
        "the trajectories only advanced {steps} shared actions, which is too \
         few to catch drift"
    );
}

/// Plays a host game to a mid-game boundary, rebuilds it from that seat's
/// observation, and then feeds both games the same `legalActions` index --
/// which is exactly the move a hosted bot makes -- until they run out of
/// deterministic road.
fn walk_one_trajectory(
    catalog: &CardCatalog,
    format: crate::Format,
    decks: [&str; 2],
    seed: u64,
) -> usize {
    let [first, second] = decks.map(|name| {
        crate::protocol::deck_by_name_for_format(format, name)
            .unwrap_or_else(|| panic!("{name} is a built-in {format:?} deck"))
    });
    let mut host =
        Game::new_with_format(format, catalog.clone(), [first, second], seed).expect("game starts");
    let mut policies = [
        crate::RandomPolicy::new(seed ^ 0xa1a1),
        crate::RandomPolicy::new(seed ^ 0xb2b2),
    ];

    // Somewhere past the opening, so the position has real state to preserve.
    for _ in 0..60 {
        let Some(player) = host.decision_player() else {
            return 0;
        };
        let observation = host.observe(player);
        let Some(action) = policies[player.index()].choose_action(&observation) else {
            return 0;
        };
        if host.apply_observed_action(&observation, action).is_err() {
            return 0;
        }
    }

    let Some(viewer) = host.decision_player() else {
        return 0;
    };
    let observation = host.observe(viewer);
    let actions = crate::protocol::protocol_actions(&observation);
    let wire = crate::protocol::observation_json_for_format(
        catalog,
        format,
        &observation,
        host.in_pregame(),
        &actions,
    );
    // The true hypothesis, so that any divergence below is a reconstruction
    // bug rather than the honest consequence of guessing hidden cards wrong.
    let mut rebuilt = Game::from_observation_checkpoint(
        catalog.clone(),
        format,
        &wire,
        &true_hidden_hypothesis(&host, viewer),
        seed ^ 0x7777,
    )
    .unwrap_or_else(|error| {
        panic!("{format:?} seed {seed}: the position did not rebuild: {error}")
    });

    assert_eq!(
        hidden_state(&host),
        hidden_state(&rebuilt),
        "{format:?} seed {seed}: the true hypothesis did not reproduce the \
         host's hidden zones, so nothing below would mean anything",
    );

    let initial_hand_memory = host.last_seen_hands.clone();
    let mut walked = 0_usize;
    for step in 0..400 {
        let Some(acting) = host.decision_player() else {
            return walked;
        };
        assert_eq!(
            rebuilt.decision_player(),
            Some(acting),
            "{format:?} seed {seed} step {step}: the rebuilt game asks a \
             different seat to act",
        );
        let host_view = seat_wire(&host, acting);
        let rebuilt_view = seat_wire(&rebuilt, acting);
        // Only the original viewer supplied private hand memory. Once
        // either game learns a new snapshot for the other seat, both games
        // must agree on that new memory as well. Checking the host too catches
        // a reconstructed game that fails to record a later observation.
        let compare_hand_memory = acting == viewer
            || hand_memory_is_reconstructible(
                &initial_hand_memory[acting.index()],
                &host.last_seen_hands[acting.index()],
                &rebuilt.last_seen_hands[acting.index()],
            );
        assert_eq!(
            canonical(&host_view, compare_hand_memory),
            canonical(&rebuilt_view, compare_hand_memory),
            "{format:?} seed {seed} step {step} (turn {} {:?}): the rebuilt \
             game drifted from the host after {walked} shared actions",
            host.turn,
            host.step,
        );

        let host_actions = crate::protocol::protocol_actions(&host.observe(acting));
        let Some(choice) = policies[acting.index()]
            .choose_action(&host.observe(acting))
            .and_then(|action| {
                host_actions
                    .iter()
                    .position(|candidate| *candidate == action)
            })
        else {
            return walked;
        };

        // A hosted bot answers with an index, so that is what both games get.
        // The lists were just proven equivalent, so the same index is the same
        // action in both.
        apply_index(&mut host, acting, choice);
        apply_index(&mut rebuilt, acting, choice);
        walked += 1;

        // Once local randomness has moved a hidden zone the two games are no
        // longer comparable, and nothing after this point would mean anything.
        // The observation was compared before the action, so a reconstruction
        // bug that reaches the seat's view is still caught on the step it
        // appears; only bugs confined to hidden zones can end a walk early.
        if hidden_state(&host) != hidden_state(&rebuilt) {
            return walked;
        }
    }
    walked
}

fn apply_index(game: &mut Game, player: PlayerId, index: usize) {
    let observation = game.observe(player);
    let actions = crate::protocol::protocol_actions(&observation);
    let action = actions
        .get(index)
        .unwrap_or_else(|| panic!("action index {index} is out of range"))
        .clone();
    game.apply(player, action)
        .expect("the chosen index is legal");
}

fn seat_wire(game: &Game, viewer: PlayerId) -> Value {
    let observation = game.observe(viewer);
    let actions = crate::protocol::protocol_actions(&observation);
    crate::protocol::observation_json_for_format(
        &game.catalog,
        game.format,
        &observation,
        game.in_pregame(),
        &actions,
    )
}

/// Both hands and both libraries, by card identity. This is the state the
/// rollout seed is allowed to move: a shuffle reorders a library, and an
/// effect that discards at random -- Hymn to Tourach -- takes a different card
/// out of a hand. Neither is a reconstruction bug; the rollout seed is
/// deliberately not the host seed.
fn hidden_state(game: &Game) -> Vec<Vec<String>> {
    let definitions =
        |cards: &[CardInstance]| cards.iter().map(|card| card.definition.get()).collect();
    game.players
        .iter()
        .flat_map(|player| [definitions(&player.hand), definitions(&player.library)])
        .collect()
}

fn hand_memory_is_reconstructible(
    initial: &crate::game::LastSeenHand,
    host: &crate::game::LastSeenHand,
    rebuilt: &crate::game::LastSeenHand,
) -> bool {
    host != initial || rebuilt.is_some()
}

/// Rewrites live object ids by first appearance, with separate historical
/// identity maps. Initial private memory for an unobserved seat is omitted;
/// all current state and legal-action relationships remain comparable.
fn canonical(wire: &Value, compare_hand_memory: bool) -> Value {
    let mut ids = BTreeMap::new();
    // The checkpoint is rules bookkeeping rather than the bot's view, and the
    // boundary audit already compares it byte for byte at every position it
    // samples. What this test is asking is whether the *game* drifted, which
    // shows up here in public state and in the legal actions.
    let mut view = wire.clone();
    if let Some(object) = view.as_object_mut() {
        object.remove("checkpoint");
        if !compare_hand_memory {
            object.remove("lastSeenHand");
        }
    }
    // Historical observations retain the ids from when cards were seen.
    // A hypothesized hidden card gets a fresh id, so those historical ids
    // cannot share the live-object renaming map. Preserve identity within
    // each historical record without asserting a link to today's hand.
    let history = ["publicReveals", "lastSeenHand"].map(|field| {
        let value = view.as_object_mut().and_then(|object| object.remove(field));
        (field, value)
    });
    let mut result = rewrite(&view, &mut ids);
    for (field, value) in history {
        if let Some(value) = value {
            result[field] = rewrite(&value, &mut BTreeMap::new());
        }
    }
    result
}

fn rewrite(value: &Value, ids: &mut BTreeMap<u64, u64>) -> Value {
    match value {
        Value::Object(map) => Value::Object(
            map.iter()
                .map(|(key, value)| {
                    let rewritten = if ID_FIELDS.contains(&key.as_str())
                        && let Some(id) = value.as_u64()
                    {
                        Value::from(ordinal(id, ids))
                    } else if ID_LIST_FIELDS.contains(&key.as_str())
                        && let Some(list) = value.as_array()
                    {
                        Value::Array(
                            list.iter()
                                .map(|value| match value.as_u64() {
                                    Some(id) => Value::from(ordinal(id, ids)),
                                    None => rewrite(value, ids),
                                })
                                .collect(),
                        )
                    } else {
                        rewrite(value, ids)
                    };
                    (key.clone(), rewritten)
                })
                .collect(),
        ),
        Value::Array(values) => Value::Array(values.iter().map(|v| rewrite(v, ids)).collect()),
        other => other.clone(),
    }
}

fn ordinal(id: u64, ids: &mut BTreeMap<u64, u64>) -> u64 {
    let next = u64::try_from(ids.len()).expect("id count fits");
    *ids.entry(id).or_insert(next)
}
