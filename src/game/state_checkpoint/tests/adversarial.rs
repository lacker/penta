//! What reconstruction must refuse, and what it must never have said.
//!
//! Two obligations meet here. A checkpoint that has been edited, truncated,
//! or paired with an inconsistent hypothesis must fail with a message rather
//! than construct an approximate world, because a bot that silently searches
//! the wrong position is worse than one that cannot search at all. And a
//! checkpoint must never carry the hidden information the observation around
//! it went to the trouble of redacting.

use super::super::*;
use super::true_hidden_hypothesis;
use serde_json::json;

struct Fixture {
    game: Game,
    catalog: CardCatalog,
    viewer: PlayerId,
    wire: Value,
    hidden: Value,
}

impl Fixture {
    /// A real mid-game position rather than a hand-built one, so the wire
    /// under test is the wire a hosted seat would actually be handed.
    fn played(actions: usize, seed: u64) -> Self {
        Self::first_where(seed, |_, count| count >= actions)
            .unwrap_or_else(|| panic!("seed {seed} never reached {actions} actions"))
    }

    /// The same, but stopping at the first boundary the caller cares about.
    /// Positions holding a particular kind of state are rare enough that
    /// picking an action count and hoping is how a test goes quietly vacuous.
    fn first_where(seed: u64, mut wanted: impl FnMut(&Game, usize) -> bool) -> Option<Self> {
        let catalog = crate::poc::catalog().expect("catalog builds");
        let format = crate::Format::OldSchool9394;
        let first = crate::protocol::deck_by_name_for_format(format, "Sligh")
            .expect("Sligh is a built-in deck");
        let second = crate::protocol::deck_by_name_for_format(format, "The Deck")
            .expect("The Deck is a built-in deck");
        let mut game = Game::new_with_format(format, catalog.clone(), [first, second], seed)
            .expect("game starts");
        let mut policies = [
            crate::RandomPolicy::new(seed ^ 0xfeed),
            crate::RandomPolicy::new(seed ^ 0xbeef),
        ];
        let mut action_count = 0_usize;
        while game.decision_player().is_some() && !wanted(&game, action_count) {
            let player = game.decision_player().expect("the loop proved a seat acts");
            let observation = game.observe(player);
            let Some(action) =
                crate::policy::Policy::choose_action(&mut policies[player.index()], &observation)
            else {
                break;
            };
            if game.apply_observed_action(&observation, action).is_err() {
                break;
            }
            action_count += 1;
        }
        let viewer = game.decision_player()?;
        if !wanted(&game, action_count) {
            return None;
        }
        let observation = game.observe(viewer);
        let actions = crate::protocol::protocol_actions(&observation);
        let wire = crate::protocol::observation_json_for_format(
            &catalog,
            format,
            &observation,
            game.in_pregame(),
            &actions,
        );
        let hidden = true_hidden_hypothesis(&game, viewer);
        Some(Self {
            game,
            catalog,
            viewer,
            wire,
            hidden,
        })
    }

    /// A board holding two simultaneous upkeep triggers that have fired but
    /// not yet been placed on the stack.
    fn with_upkeep_triggers() -> Self {
        let catalog = crate::poc::catalog().expect("catalog builds");
        let mut game = crate::game::tests::ready_game();
        for id in 10_000..10_002 {
            let mut vault =
                crate::game::tests::creature(id, crate::card::cards::MANA_VAULT, PlayerId::One);
            vault.tapped = true;
            game.battlefield.push(vault);
        }
        game.step = crate::Step::Upkeep;
        game.handle_upkeep_triggers();
        let viewer = game
            .decision_player()
            .expect("the position awaits an action");
        let observation = game.observe(viewer);
        let actions = crate::protocol::protocol_actions(&observation);
        let wire = crate::protocol::observation_json_for_format(
            &catalog,
            game.format,
            &observation,
            game.in_pregame(),
            &actions,
        );
        let hidden = true_hidden_hypothesis(&game, viewer);
        Self {
            game,
            catalog,
            viewer,
            wire,
            hidden,
        }
    }

    fn rebuild(&self, wire: &Value, hidden: &Value) -> Result<Game, String> {
        Game::from_observation_checkpoint(
            self.catalog.clone(),
            self.game.format,
            wire,
            hidden,
            9_001,
        )
    }

    /// Reconstructing the fixture untouched must work, so a rejection below is
    /// attributable to the specific corruption and not to the position.
    fn assert_baseline_rebuilds(&self) {
        self.rebuild(&self.wire, &self.hidden)
            .expect("the untouched fixture reconstructs");
    }

    fn wire_with(&self, edit: impl FnOnce(&mut Value)) -> Value {
        let mut wire = self.wire.clone();
        edit(&mut wire);
        wire
    }

    fn hidden_with(&self, edit: impl FnOnce(&mut Value)) -> Value {
        let mut hidden = self.hidden.clone();
        edit(&mut hidden);
        hidden
    }

    fn rejects_wire(&self, description: &str, edit: impl FnOnce(&mut Value)) -> String {
        let wire = self.wire_with(edit);
        match self.rebuild(&wire, &self.hidden) {
            Ok(_) => panic!("{description} was accepted"),
            Err(error) => error,
        }
    }

    fn rejects_hidden(&self, description: &str, edit: impl FnOnce(&mut Value)) -> String {
        let hidden = self.hidden_with(edit);
        match self.rebuild(&self.wire, &hidden) {
            Ok(_) => panic!("{description} was accepted"),
            Err(error) => error,
        }
    }
}

fn checkpoint_mut(wire: &mut Value) -> &mut serde_json::Map<String, Value> {
    wire["checkpoint"]
        .as_object_mut()
        .expect("the checkpoint is an object")
}

#[test]
fn a_checkpoint_missing_any_required_field_is_rejected_by_name() {
    // Additive members carry `#[serde(default)]` deliberately, so that a
    // checkpoint written before they existed still decodes. They are the one
    // kind of field that is not load-bearing, and each is listed rather than
    // inferred so a genuinely required field cannot quietly join them.
    const ADDITIVE: &[&str] = &[
        "allCombatDamagePrevented",
        "preventionShields",
        "relationalDamagePreventions",
    ];

    let fixture = Fixture::played(120, 8_101);
    fixture.assert_baseline_rebuilds();
    let names = fixture.wire["checkpoint"]
        .as_object()
        .expect("the checkpoint is an object")
        .keys()
        .cloned()
        .collect::<Vec<_>>();
    assert!(
        names.len() > 20,
        "the checkpoint carries too little to be worth auditing: {names:?}"
    );
    for name in names {
        // Optional fields legitimately vanish; the rest are load-bearing.
        if fixture.wire["checkpoint"][&name].is_null() || ADDITIVE.contains(&name.as_str()) {
            continue;
        }
        let error = fixture.rejects_wire(&format!("a checkpoint missing {name}"), |wire| {
            checkpoint_mut(wire).remove(&name);
        });
        assert!(
            error.contains("invalid game snapshot"),
            "dropping {name} produced an unhelpful message: {error}"
        );
    }
}

#[test]
fn a_checkpoint_field_of_the_wrong_shape_is_rejected() {
    let fixture = Fixture::played(140, 8_103);
    fixture.assert_baseline_rebuilds();
    for (name, replacement) in [
        ("turnsStarted", json!("two")),
        ("mana", json!(0)),
        ("battlefield", json!({})),
        ("stack", json!("empty")),
        ("viewer", json!(null)),
        ("consecutivePasses", json!(-1)),
        ("extraTurns", json!([9])),
    ] {
        let error = fixture.rejects_wire(&format!("a {name} of the wrong shape"), |wire| {
            checkpoint_mut(wire).insert(name.into(), replacement);
        });
        assert!(
            !error.is_empty(),
            "a malformed {name} must fail with a message"
        );
    }
}

#[test]
fn a_checkpoint_taken_for_the_other_seat_is_rejected() {
    let fixture = Fixture::played(160, 8_107);
    fixture.assert_baseline_rebuilds();
    let error = fixture.rejects_wire("a checkpoint belonging to the other seat", |wire| {
        let viewer = wire["checkpoint"]["viewer"].as_u64().expect("a seat index");
        checkpoint_mut(wire).insert("viewer".into(), json!(1 - viewer));
    });
    assert!(
        error.contains("viewer does not match"),
        "unexpected message: {error}"
    );
}

/// The deferred flag is the engine's own statement that it is holding rules
/// state it cannot address. Honoring it is what makes every other guarantee
/// here conditional rather than aspirational.
#[test]
fn a_checkpoint_that_declares_deferred_state_is_rejected() {
    let fixture = Fixture::played(120, 8_109);
    fixture.assert_baseline_rebuilds();
    let error = fixture.rejects_wire("a checkpoint with deferred state", |wire| {
        checkpoint_mut(wire).insert("hasDeferredState".into(), json!(true));
    });
    assert!(
        error.contains("without stable catalog semantics"),
        "unexpected message: {error}"
    );
}

#[test]
fn a_locator_that_is_absent_from_this_catalog_is_rejected() {
    let fixture = Fixture::played(120, 8_113);
    fixture.assert_baseline_rebuilds();
    let error = fixture.rejects_wire("a grant naming a definition outside the catalog", |wire| {
        checkpoint_mut(wire).insert(
            "temporaryAbilityGrants".into(),
            json!([{
                "object": 1,
                "ability": {
                    "definition": u16::MAX,
                    "partId": 0,
                    "abilityId": 0,
                    "nested": [],
                },
            }]),
        );
    });
    assert!(
        error.contains("absent from this catalog"),
        "unexpected message: {error}"
    );
}

/// Aggregate pools are public and the itemized units are not, so an attacker
/// editing only one of the two is the shape to catch.
#[test]
fn mana_units_that_disagree_with_the_public_pool_are_rejected() {
    let fixture = Fixture::played(120, 8_127);
    fixture.assert_baseline_rebuilds();
    let error = fixture.rejects_wire("mana units disagreeing with the pool", |wire| {
        checkpoint_mut(wire).insert(
            "mana".into(),
            json!([[{"color": "red", "source": null, "payload": null}], []]),
        );
    });
    assert!(
        error.contains("do not match"),
        "unexpected message: {error}"
    );
}

/// A pending decision whose id is not below the next-id counter describes a
/// state the engine could not have produced. Accepting it would hand the bot a
/// world whose next allocation collides with an object already in it.
///
/// The sibling guard on `nextTriggerId` has no reachable fixture: a census of
/// nearly three hundred thousand sampled boundaries never found `pendingTriggers`
/// populated, because triggers waiting to be placed live inside the decision
/// that places them. It stays a check on malformed input rather than on a
/// position a host can emit, so it is asserted directly below.
#[test]
fn a_decision_id_that_precedes_its_own_state_is_rejected() {
    let fixture = (0..24_u64)
        .find_map(|attempt| {
            Fixture::first_where(8_131 + attempt * 29, |game, count| {
                !game.pending_decisions.is_empty() || count >= 400
            })
            .filter(|fixture| !fixture.game.pending_decisions.is_empty())
        })
        .expect("no sampled position held a pending decision to corrupt");
    fixture.assert_baseline_rebuilds();
    let error = fixture.rejects_wire("a next decision id that precedes its own state", |wire| {
        checkpoint_mut(wire).insert("nextDecisionId".into(), json!(0));
    });
    assert!(
        error.contains("does not follow"),
        "unexpected message: {error}"
    );
}

/// The sibling guard, on the triggers a checkpoint says are waiting. Sampled
/// play never populates `pendingTriggers` -- triggers awaiting placement
/// normally live inside the decision that places them -- so the position is
/// built rather than found.
#[test]
fn a_trigger_id_that_precedes_its_own_state_is_rejected() {
    let fixture = Fixture::with_upkeep_triggers();
    assert!(
        !fixture.wire["checkpoint"]["pendingTriggers"]
            .as_array()
            .expect("the checkpoint lists its pending triggers")
            .is_empty(),
        "the fixture must actually be holding triggers"
    );
    fixture.assert_baseline_rebuilds();
    let error = fixture.rejects_wire("a next trigger id that precedes its own state", |wire| {
        checkpoint_mut(wire).insert("nextTriggerId".into(), json!(0));
    });
    assert!(
        error.contains("does not follow"),
        "unexpected message: {error}"
    );
}

#[test]
fn a_hypothesis_with_the_wrong_hidden_zone_sizes_is_rejected() {
    let fixture = Fixture::played(180, 8_137);
    fixture.assert_baseline_rebuilds();
    let opponent = seat_label(fixture.viewer.opponent());
    let error = fixture.rejects_hidden("an oversized opposing hand", |hidden| {
        let hand = hidden["hands"][opponent]
            .as_array_mut()
            .expect("the hypothesis lists the opposing hand");
        hand.push(json!(crate::card::cards::MOUNTAIN.0));
    });
    assert!(
        error.contains("opponentHandSize"),
        "unexpected message: {error}"
    );

    let error = fixture.rejects_hidden("a short library", |hidden| {
        hidden["libraries"]["p1"]
            .as_array_mut()
            .expect("the hypothesis lists p1's library")
            .pop();
    });
    assert!(error.contains("library has"), "unexpected message: {error}");
}

#[test]
fn a_hypothesis_that_omits_the_hidden_zones_entirely_is_rejected() {
    let fixture = Fixture::played(150, 8_147);
    fixture.assert_baseline_rebuilds();
    for hidden in [json!({}), json!({"hands": {}}), json!(null), json!(7)] {
        assert!(
            fixture.rebuild(&fixture.wire, &hidden).is_err(),
            "an incomplete hypothesis was accepted: {hidden}"
        );
    }
}

/// Indices into the hypothesis are how a seat says which hidden card it means.
/// An out-of-range index is the difference between naming a card and naming
/// nothing, so it cannot be quietly dropped.
#[test]
fn hypothesis_indices_outside_the_hidden_hand_are_rejected() {
    let fixture = Fixture::played(200, 8_153);
    fixture.assert_baseline_rebuilds();
    let opponent = seat_label(fixture.viewer.opponent());
    let error = fixture.rejects_hidden("a drawn index past the end of the hand", |hidden| {
        hidden["drawnThisTurn"][opponent] = json!([9_999]);
    });
    assert!(
        error.contains("out of range"),
        "unexpected message: {error}"
    );

    let error = fixture.rejects_hidden("a miracle index past the end of the hand", |hidden| {
        hidden["miracleWindow"] = json!({"seat": opponent, "handIndex": 9_999});
    });
    assert!(
        error.contains("out of range"),
        "unexpected message: {error}"
    );
}

/// The viewer's own miracle window is public to it and already carried by the
/// checkpoint. A hypothesis claiming the viewer's seat is trying to write
/// state it does not own.
#[test]
fn a_hypothesis_that_claims_the_viewers_own_hidden_state_is_rejected() {
    let fixture = Fixture::played(200, 8_161);
    fixture.assert_baseline_rebuilds();
    let viewer = seat_label(fixture.viewer);
    let error = fixture.rejects_hidden("a miracle window on the viewer's seat", |hidden| {
        hidden["miracleWindow"] = json!({"seat": viewer, "handIndex": 0});
    });
    assert!(
        error.contains("must belong to the opposing seat"),
        "unexpected message: {error}"
    );
}

/// The direct statement of redaction: rewrite the opposing library into
/// something else entirely and the seat's wire must not move a byte. Scanning
/// the wire for suspicious numbers would be weaker and noisier -- object ids
/// share a number space with counts and definition ids -- while this asks the
/// question that actually matters, which is whether the seat's view is a
/// function of hidden state at all.
#[test]
fn a_seats_wire_does_not_depend_on_the_opposing_library() {
    let catalog = crate::poc::catalog().expect("catalog builds");
    let format = crate::Format::OldSchool9394;
    let decks = crate::protocol::deck_names_for_format(format);
    let mut checked = 0_usize;
    for game_index in 0..6_usize {
        let first =
            crate::protocol::deck_by_name_for_format(format, decks[game_index % decks.len()])
                .expect("deck exists");
        let second = crate::protocol::deck_by_name_for_format(
            format,
            decks[(game_index * 3 + 1) % decks.len()],
        )
        .expect("deck exists");
        let seed = 8_200 + u64::try_from(game_index).expect("index fits") * 17;
        let mut game = Game::new_with_format(format, catalog.clone(), [first, second], seed)
            .expect("game starts");
        let mut policies = [
            crate::RandomPolicy::new(seed ^ 0x1a1a),
            crate::RandomPolicy::new(seed ^ 0x2b2b),
        ];
        for _ in 0..600 {
            let Some(viewer) = game.decision_player() else {
                break;
            };
            let before = seat_wire(&game, viewer);
            let mut rewritten = game.clone();
            rewrite_library(&mut rewritten, viewer.opponent());
            assert_eq!(
                seat_wire(&rewritten, viewer),
                before,
                "the {} wire moved when only the opposing library changed, at turn {} {:?}",
                seat_label(viewer),
                game.turn,
                game.step,
            );
            checked += 1;
            let observation = game.observe(viewer);
            let Some(action) =
                crate::policy::Policy::choose_action(&mut policies[viewer.index()], &observation)
            else {
                break;
            };
            if game.apply_observed_action(&observation, action).is_err() {
                break;
            }
        }
    }
    assert!(
        checked >= 500,
        "the redaction sweep only reached {checked} boundaries"
    );
}

/// Replaces what a library holds while leaving the object identities and the
/// card count alone, so only genuinely hidden information changes.
fn rewrite_library(game: &mut Game, player: PlayerId) {
    let replacement = crate::card::cards::MOUNTAIN;
    let library = &mut game.players[player.index()].library;
    library.reverse();
    for card in library.iter_mut() {
        card.definition = replacement;
        card.characteristics = CharacteristicSource::Card(replacement);
    }
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
