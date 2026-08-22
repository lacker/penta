//! Reconstruction at the boundaries sampled play almost never reaches.
//!
//! The broad audit walks hundreds of thousands of decision boundaries, but a
//! census of what it actually touched shows the tail it cannot reach: a
//! seven-mana pile-splitter, a planeswalker ultimate, an entering copy. Those
//! are exactly the continuations most likely to carry state a snapshot cannot
//! express, so each one is built here on purpose rather than waited for.

use super::super::*;
use super::{determinized, true_hidden_hypothesis};
use crate::game::tests::{card, creature, ready_game};
use crate::game::{DecisionContinuation, ResolvedEffectPayment};
use crate::{Action, CardDefinitionId, CounterKind, ManaColor, TargetSelection};
use serde_json::Value;

/// The whole reconstruction contract at one boundary: the engine claims the
/// state is representable, the true hidden zones rebuild it, and so does a
/// hypothesis that disagrees with them. The last part is the point of the
/// exercise -- a search bot never knows the truth, so a reconstruction that
/// only works when handed the real hidden zones is not a reconstruction.
pub(super) fn assert_reconstructs(game: &Game, label: &str) {
    let viewer = game
        .decision_player()
        .unwrap_or_else(|| panic!("{label}: the position must await an action"));
    let observation = game.observe(viewer);
    let actions = crate::protocol::protocol_actions(&observation);
    let wire = crate::protocol::observation_json_for_format(
        &game.catalog,
        game.format,
        &observation,
        game.in_pregame(),
        &actions,
    );
    assert_eq!(
        wire["checkpoint"]["hasDeferredState"],
        Value::Bool(false),
        "{label}: the engine deferred a state it is expected to represent",
    );

    let truth = true_hidden_hypothesis(game, viewer);
    for (kind, hidden) in [
        ("the true hidden zones", truth.clone()),
        (
            "a hypothesis that disagrees with the host",
            determinized(&truth, viewer),
        ),
    ] {
        let rebuilt = Game::from_observation_checkpoint(
            game.catalog.clone(),
            game.format,
            &wire,
            &hidden,
            4_242,
        )
        .unwrap_or_else(|error| panic!("{label}: {kind} failed to rebuild: {error}"));
        let rebuilt_observation = rebuilt.observe(viewer);
        let rebuilt_actions = crate::protocol::protocol_actions(&rebuilt_observation);
        assert_eq!(
            rebuilt_actions, actions,
            "{label}: {kind} changed the actions"
        );
        assert_eq!(
            crate::protocol::observation_json_for_format(
                &rebuilt.catalog,
                rebuilt.format,
                &rebuilt_observation,
                rebuilt.in_pregame(),
                &rebuilt_actions,
            ),
            wire,
            "{label}: {kind} changed the public observation",
        );
    }
}

fn rebuild_from_truth(game: &Game, seed: u64) -> Game {
    let viewer = game
        .decision_player()
        .expect("the position must await an action");
    rebuild_from_truth_for_viewer(game, viewer, seed)
}

fn rebuild_from_truth_for_viewer(game: &Game, viewer: PlayerId, seed: u64) -> Game {
    let observation = game.observe(viewer);
    let actions = crate::protocol::protocol_actions(&observation);
    let wire = crate::protocol::observation_json_for_format(
        &game.catalog,
        game.format,
        &observation,
        game.in_pregame(),
        &actions,
    );
    Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &wire,
        &true_hidden_hypothesis(game, viewer),
        seed,
    )
    .expect("the typed decision state reconstructs")
}

/// `ready_game` clears the board and hands but keeps both libraries stocked,
/// which is what these positions want: a known board and real hidden zones to
/// hypothesize about.
pub(super) fn staged_game() -> Game {
    ready_game()
}

/// The same board under the modern format, for the cards that only exist
/// there. Only the rules profile differs; reconstruction reads the format
/// from the same observation it reads everything else from.
pub(super) fn staged_modern_game() -> Game {
    let mut game = ready_game();
    game.format = crate::Format::IsdDgmStandard;
    game
}

#[test]
fn a_begin_turn_replacement_with_a_deferred_effect_reconstructs_and_resumes() {
    let mut game = staged_game();
    game.step = crate::Step::Cleanup;
    game.extra_turns = vec![PlayerId::Two, PlayerId::Two];
    let mut vault = creature(10_000, crate::card::cards::TIME_VAULT, PlayerId::Two);
    vault.tapped = true;
    let vault_id = vault.card.id;
    game.battlefield.push(vault);

    game.start_next_turn();
    let pending = game
        .pending_decisions
        .first()
        .expect("Time Vault asks whether to replace the first extra turn");
    let replacement = pending
        .observation
        .options
        .iter()
        .find(|option| option.card.is_some_and(|(object, _)| object == vault_id))
        .expect("the Vault replacement is offered")
        .id;
    game.apply(
        PlayerId::Two,
        Action::ChooseDecision {
            decision: pending.observation.id,
            options: vec![replacement],
        },
    )
    .expect("the first extra turn is skipped");

    assert!(matches!(
        game.pending_decisions
            .first()
            .map(|pending| &pending.continuation),
        Some(DecisionContinuation::BeginTurn { deferred, .. }) if deferred.len() == 1
    ));
    assert_reconstructs(
        &game,
        "a prospective turn carrying Time Vault's deferred untap",
    );
    let mut rebuilt = rebuild_from_truth(&game, 4_243);

    for candidate in [&mut game, &mut rebuilt] {
        let pending = candidate
            .pending_decisions
            .first()
            .expect("the reconstructed turn choice remains pending");
        candidate
            .apply(
                PlayerId::Two,
                Action::ChooseDecision {
                    decision: pending.observation.id,
                    options: vec![0],
                },
            )
            .expect("the next extra turn begins");
    }

    assert_eq!(rebuilt.observe(PlayerId::One), game.observe(PlayerId::One));
    assert_eq!(rebuilt.observe(PlayerId::Two), game.observe(PlayerId::Two));
    assert_eq!(game.active_player, PlayerId::Two);
    assert!(
        !game
            .battlefield
            .iter()
            .find(|permanent| permanent.card.id == vault_id)
            .expect("Time Vault remains on the battlefield")
            .tapped,
        "the carried effect untaps Time Vault before the accepted turn begins",
    );
}

#[test]
fn a_battlefield_exit_replacement_choice_explicitly_fails_closed() {
    let mut game = staged_game();
    game.battlefield.clear();
    game.put_onto_battlefield(PlayerId::Two, crate::card::cards::REST_IN_PEACE)
        .expect("Rest in Peace is cataloged");
    let nexus = game
        .put_onto_battlefield(PlayerId::One, crate::card::cards::UGINS_NEXUS)
        .expect("Ugin's Nexus is cataloged");
    game.pending_triggers.clear();
    game.move_permanents_to_graveyard(&[nexus]);

    assert!(matches!(
        game.pending_decisions
            .first()
            .map(|pending| &pending.continuation),
        Some(DecisionContinuation::BattlefieldExitReplacement { .. })
    ));
    let viewer = game
        .decision_player()
        .expect("the affected player orders the replacements");
    let observation = game.observe(viewer);
    let actions = crate::protocol::protocol_actions(&observation);
    let wire = crate::protocol::observation_json_for_format(
        &game.catalog,
        game.format,
        &observation,
        game.in_pregame(),
        &actions,
    );
    assert_eq!(wire["checkpoint"]["hasDeferredState"], Value::Bool(true));
    let error = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &wire,
        &true_hidden_hypothesis(&game, viewer),
        4_244,
    )
    .expect_err("the unencoded battlefield-exit completion graph must fail closed");
    assert!(
        error.contains("without stable catalog semantics"),
        "unexpected reconstruction error: {error}",
    );
}

/// Adds mana the way the engine does, so the itemized units and the public
/// aggregate pool stay in agreement. Writing `mana_pool` alone builds a game
/// the engine could never have reached, and reconstruction says so.
pub(super) fn fill_mana(game: &mut Game, player: PlayerId, amount: u16) {
    for color in [
        ManaColor::White,
        ManaColor::Blue,
        ManaColor::Black,
        ManaColor::Red,
        ManaColor::Green,
        ManaColor::Colorless,
    ] {
        game.add_unrestricted_mana(player, color, amount);
    }
}

fn loyalty_action(
    source: GameObjectId,
    definition: CardDefinitionId,
    ability: u8,
    targets: Vec<TargetSelection>,
) -> Action {
    Action::ActivateAbility {
        source,
        ability: AbilityOrigin::Printed {
            definition,
            part: CardPartId::PRIMARY,
            ability: AbilityId(ability),
        },
        targets,
        cost_objects: Vec::new(),
        x: 0,
        modes: Vec::new(),
    }
}

pub(super) fn resolve_top_of_stack(game: &mut Game) {
    for _ in 0..8 {
        if !game.pending_decisions.is_empty() || game.stack.is_empty() {
            return;
        }
        let priority = game.priority;
        game.apply(priority, Action::PassPriority)
            .expect("priority passes while the stack resolves");
    }
}

pub(super) fn answer_with_first_option(game: &mut Game) {
    let pending = game
        .pending_decisions
        .first()
        .expect("a decision is pending");
    let player = pending.observation.player;
    let decision = pending.observation.id;
    let option = pending.observation.options[0].id;
    game.apply(
        player,
        Action::ChooseDecision {
            decision,
            options: vec![option],
        },
    )
    .expect("the first option is answerable");
}

fn answer_with_label(game: &mut Game, label: &str) {
    let pending = game
        .pending_decisions
        .first()
        .expect("a decision is pending");
    let player = pending.observation.player;
    let decision = pending.observation.id;
    let option = pending
        .observation
        .options
        .iter()
        .find(|option| option.label == label)
        .unwrap_or_else(|| panic!("the decision does not offer {label}"))
        .id;
    game.apply(
        player,
        Action::ChooseDecision {
            decision,
            options: vec![option],
        },
    )
    .unwrap_or_else(|error| panic!("the {label} option is answerable: {error}"));
}

/// Casts `source` at `target` by finding the action the seat is actually
/// offered, so target-slot bookkeeping stays the engine's business.
pub(super) fn cast_targeting(
    game: &mut Game,
    player: PlayerId,
    source: GameObjectId,
    target: Target,
) {
    let action = game
        .legal_actions(player)
        .into_iter()
        .find(|action| {
            matches!(
                action,
                Action::CastSpell { card, choices, .. }
                    if *card == source
                        && choices.iter_targets().copied().eq([target])
            )
        })
        .unwrap_or_else(|| panic!("{source:?} cannot be cast at {target:?}"));
    game.apply(player, action).expect("the cast is legal");
}

/// Answers the pending decision with the option standing for `object`. Entry
/// choices usually offer "as itself" first, and taking that default would walk
/// straight past the state the test exists to build.
fn answer_with_option_naming(game: &mut Game, object: GameObjectId) {
    let pending = game
        .pending_decisions
        .first()
        .expect("a decision is pending");
    let player = pending.observation.player;
    let decision = pending.observation.id;
    let option = pending
        .observation
        .options
        .iter()
        .find(|option| {
            option
                .card
                .is_some_and(|(candidate, _)| candidate == object)
        })
        .unwrap_or_else(|| panic!("{object:?} is not one of the offered options"))
        .id;
    game.apply(
        player,
        Action::ChooseDecision {
            decision,
            options: vec![option],
        },
    )
    .expect("the named option is answerable");
}

include!("rare_states/decisions.rs");

#[test]
fn battlefield_entry_payment_freezes_and_authenticates_its_payer_and_cost() {
    let mut game = staged_modern_game();
    let land = card(11_200, crate::card::cards::HALLOWED_FOUNTAIN, PlayerId::One);
    let land_id = land.id;
    game.players[PlayerId::One.index()].hand.push(land);
    game.apply(
        PlayerId::One,
        Action::PlayLand {
            card: land_id,
            option: crate::PlayOptionId::DEFAULT,
        },
    )
    .expect("the shock land is playable");

    assert!(matches!(
        game.pending_decisions
            .first()
            .map(|pending| &pending.continuation),
        Some(DecisionContinuation::BattlefieldEntryPayment {
            player: PlayerId::One,
            payment: ResolvedEffectPayment::Life(2),
            definition: crate::card::ReplacementEffectDef::PayOr { .. },
            ..
        })
    ));
    assert_reconstructs(&game, "a frozen battlefield-entry payment");

    let observation = game.observe(PlayerId::One);
    let actions = crate::protocol::protocol_actions(&observation);
    let wire = crate::protocol::observation_json_for_format(
        &game.catalog,
        game.format,
        &observation,
        game.in_pregame(),
        &actions,
    );
    let hidden = true_hidden_hypothesis(&game, PlayerId::One);
    for (label, mut edited) in [
        ("payer", wire.clone()),
        ("payment", wire.clone()),
        ("source", wire.clone()),
        ("schema", wire.clone()),
    ] {
        if label == "payer" {
            edited["checkpoint"]["decisionState"]["continuation"]["player"] =
                Value::from(PlayerId::Two.index());
        } else if label == "payment" {
            edited["checkpoint"]["decisionState"]["continuation"]["payment"]["value"] =
                Value::from(3);
        } else if label == "source" {
            edited["checkpoint"]["decisionState"]["continuation"]["effect"]["ability"]["definition"] =
                Value::from(crate::card::cards::STEAM_VENTS.get());
        } else {
            edited["decision"]["orderSemantics"] = Value::String("resolution".into());
        }
        let error = Game::from_observation_checkpoint(
            game.catalog.clone(),
            game.format,
            &edited,
            &hidden,
            4_244,
        )
        .unwrap_err();
        assert!(
            error.contains("payer or payment disagrees")
                || error.contains("locator disagrees with its replacement source")
                || error.contains("decision kind disagrees"),
            "unexpected {label} error: {error}",
        );
    }
}

#[test]
fn ordinary_pay_or_rejects_payer_and_payment_splices() {
    let mut game = staged_game();
    let mut vault = creature(11_300, crate::card::cards::MANA_VAULT, PlayerId::One);
    vault.tapped = true;
    game.battlefield.push(vault);
    for id in 11_301..11_305 {
        game.battlefield
            .push(creature(id, crate::card::cards::MOUNTAIN, PlayerId::One));
    }
    game.step = crate::Step::Upkeep;
    game.handle_upkeep_triggers();
    game.finish_rules_procedure();
    for _ in 0..8 {
        if !game.pending_decisions.is_empty() {
            break;
        }
        let priority = game.priority;
        game.apply(priority, Action::PassPriority)
            .expect("priority passes while Mana Vault resolves");
    }
    assert!(matches!(
        game.pending_decisions
            .first()
            .map(|pending| &pending.continuation),
        Some(DecisionContinuation::PayOr {
            player: PlayerId::One,
            payment: ResolvedEffectPayment::Mana(_),
            ..
        })
    ));
    assert_reconstructs(&game, "an ordinary frozen pay-or choice");

    let observation = game.observe(PlayerId::One);
    let actions = crate::protocol::protocol_actions(&observation);
    let wire = crate::protocol::observation_json_for_format(
        &game.catalog,
        game.format,
        &observation,
        game.in_pregame(),
        &actions,
    );
    let hidden = true_hidden_hypothesis(&game, PlayerId::One);
    for (label, mut edited) in [
        ("payer", wire.clone()),
        ("payment", wire.clone()),
        ("definition", wire.clone()),
        ("schema", wire.clone()),
    ] {
        if label == "payer" {
            edited["checkpoint"]["decisionState"]["continuation"]["player"] =
                Value::from(PlayerId::Two.index());
        } else if label == "payment" {
            edited["checkpoint"]["decisionState"]["continuation"]["payment"]["value"]["generic"] =
                Value::from(5);
        } else if label == "definition" {
            edited["checkpoint"]["decisionState"]["continuation"]["definition"]["path"] =
                serde_json::json!([999]);
        } else {
            edited["decision"]["orderSemantics"] = Value::String("resolution".into());
        }
        let error = Game::from_observation_checkpoint(
            game.catalog.clone(),
            game.format,
            &edited,
            &hidden,
            4_245,
        )
        .unwrap_err();
        assert!(
            error.contains("payer")
                || error.contains("payment")
                || error.contains("locator is absent")
                || error.contains("decision kind disagrees"),
            "unexpected {label} error: {error}",
        );
    }
}

#[test]
#[allow(clippy::too_many_lines)]
fn duress_choice_checkpoint_rejects_ineligible_hand_card_splices() {
    let mut game = staged_modern_game();
    let duress = card(11_400, crate::card::cards::DURESS, PlayerId::One);
    let duress_id = duress.id;
    game.players[0].hand.push(duress);
    game.players[1].hand.extend([
        card(11_401, crate::card::cards::SAVANNAH_LIONS, PlayerId::Two),
        card(11_402, crate::card::cards::MOUNTAIN, PlayerId::Two),
        card(11_403, crate::card::cards::LIGHTNING_BOLT, PlayerId::Two),
        card(11_404, crate::card::cards::BLACK_LOTUS, PlayerId::Two),
    ]);
    fill_mana(&mut game, PlayerId::One, 1);
    game.apply(
        PlayerId::One,
        Action::CastSpell {
            card: duress_id,
            choices: crate::CastChoices::default().with_targets(vec![TargetSelection::single(
                crate::TargetSlotId(0),
                Target::Player(PlayerId::Two),
            )]),
            sacrifices: Vec::new(),
        },
    )
    .expect("Duress is castable");
    resolve_top_of_stack(&mut game);
    assert!(matches!(
        game.pending_decisions
            .first()
            .map(|pending| &pending.continuation),
        Some(DecisionContinuation::ChooseForEffect { .. })
    ));
    assert_reconstructs(&game, "Duress's authored object choice");

    let viewer = PlayerId::One;
    let observation = game.observe(viewer);
    let actions = crate::protocol::protocol_actions(&observation);
    let wire = crate::protocol::observation_json_for_format(
        &game.catalog,
        game.format,
        &observation,
        game.in_pregame(),
        &actions,
    );
    let hidden = true_hidden_hypothesis(&game, viewer);

    let mut missing_origin = wire.clone();
    missing_origin["checkpoint"]["decisionState"]["cardOrigins"] = serde_json::json!([]);
    let error = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &missing_origin,
        &hidden,
        4_246,
    )
    .expect_err("a disclosed hidden-hand option requires its origin");
    assert!(
        error.contains("lacks a card origin"),
        "unexpected error: {error}"
    );

    let mut wrong_origin = wire.clone();
    wrong_origin["checkpoint"]["decisionState"]["cardOrigins"][0]["zone"] =
        Value::String("library".into());
    let error = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &wrong_origin,
        &hidden,
        4_247,
    )
    .expect_err("a disclosed hand option rejects a library origin");
    assert!(
        error.contains("disagrees with its option zone"),
        "unexpected error: {error}",
    );

    let savannah = game.players[1]
        .hand
        .iter()
        .find(|card| card.definition == crate::card::cards::SAVANNAH_LIONS)
        .expect("the excluded creature remains in hand");
    let mut spliced = wire.clone();
    let old_object = spliced["decision"]["options"][0]["card"]["objectId"]
        .as_u64()
        .expect("the eligible option has an object id");
    spliced["decision"]["options"][0]["label"] = Value::String("Savannah Lions".into());
    spliced["decision"]["options"][0]["card"]["objectId"] = Value::from(savannah.id.0);
    spliced["decision"]["options"][0]["card"]["definition"] =
        Value::from(savannah.definition.get());
    spliced["decision"]["options"][0]["card"]["characteristics"]["definition"] =
        Value::from(savannah.definition.get());
    spliced["checkpoint"]["decisionState"]["options"][0]["card"]["objectId"] =
        Value::from(savannah.id.0);
    spliced["checkpoint"]["decisionState"]["options"][0]["card"]["characteristics"]["definition"] =
        Value::from(savannah.definition.get());
    let origins = spliced["checkpoint"]["decisionState"]["cardOrigins"]
        .as_array_mut()
        .expect("card origins are an array");
    let origin = origins
        .iter_mut()
        .find(|origin| origin["objectId"].as_u64() == Some(old_object))
        .expect("the edited option has an origin");
    origin["objectId"] = Value::from(savannah.id.0);
    let error = Game::from_observation_checkpoint(
        game.catalog.clone(),
        game.format,
        &spliced,
        &hidden,
        4_248,
    )
    .expect_err("Duress cannot be edited to choose a creature");
    assert!(
        error.contains("object choice decision options disagree"),
        "unexpected error: {error}",
    );
}
