import assert from "node:assert/strict";
import test from "node:test";

import { HostedGame } from "../app/wasm/penta_wasm.js";
import { initializeWasm, WebGame } from "./wasm-test-support.mjs";

test("the game-over message names whoever actually lost", async () => {
  await initializeWasm();

  const playOut = (game) => {
    for (let turn = 0; turn < 2000; turn++) {
      const state = JSON.parse(game.state_json());
      if (state.result) return state.result;
      if (state.decision) {
        const wanted = Math.max(state.decision.minimum, 1);
        game.choose_decision(
          state.decision.id,
          JSON.stringify(state.decision.options.slice(0, wanted).map((option) => option.id)),
        );
        continue;
      }
      const actions = state.actions.filter((action) => action.kind !== "danger");
      const next =
        actions.find((action) => action.label === "Keep this hand") ??
        actions.find((action) => action.label.startsWith("Bottom ")) ??
        actions.find((action) => action.label.startsWith("Discard ")) ??
        actions.find((action) => action.kind === "pass") ??
        actions[0];
      if (!next) return null;
      game.act(next.index);
    }
    return null;
  };

  const lost = new WebGame("The Deck", "Sligh", "Handcrafted", false, 8);
  const loss = playOut(lost);
  assert.equal(loss.outcome, "loss");
  assert.equal(
    loss.message,
    "You lose — you lost all life",
    "the reason is phrased from the browser player's seat, not the winner's",
  );
  lost.free();
});

test("the game log describes objects that have left visible zones", async () => {
  await initializeWasm();

  // Swords and combat leave the log holding references the observation can
  // no longer resolve from the battlefield or another visible zone.
  const game = new WebGame("White Weenie", "GR Aggro", "Handcrafted", true, 3041712688);
  let sawExileReference = false;
  let sawDestroyed = false;
  let sawExiled = false;
  const reported = new Set();
  for (let turn = 0; turn < 1200; turn++) {
    const state = JSON.parse(game.state_json());
    if (state.result) break;
    for (const line of state.events) {
      assert.ok(
        !/card #\d+|spell #\d+|Unknown card/.test(line),
        `game log leaked a raw instance id: "${line}"`,
      );
      if (/Swords to Plowshares/.test(line)) sawExileReference = true;
      if (/ was destroyed$| was exiled$| returned to hand$/.test(line)) {
        reported.add(line);
        if (line.endsWith("was destroyed")) sawDestroyed = true;
        if (line.endsWith("was exiled")) sawExiled = true;
      }
    }
    for (const action of state.actions) {
      assert.ok(
        !/card #\d+|spell #\d+/.test(action.label),
        `action label leaked a raw instance id: "${action.label}"`,
      );
    }
    if (sawExileReference && sawDestroyed && sawExiled) break;
    if (state.decision) {
      const wanted = Math.max(state.decision.minimum, 1);
      game.choose_decision(
        state.decision.id,
        JSON.stringify(state.decision.options.slice(0, wanted).map((option) => option.id)),
      );
      continue;
    }
    const actions = state.actions.filter((action) => action.kind !== "danger");
    const next =
      actions.find((action) => action.label === "Keep this hand") ??
      actions.find((action) => action.label.startsWith("Bottom ")) ??
      actions.find((action) => action.label.startsWith("Play ")) ??
      actions.find((action) => action.label.startsWith("Attack with ")) ??
      actions.find((action) => action.label.startsWith("Block ")) ??
      actions.find((action) => action.label.startsWith("Assign ")) ??
      actions.find((action) => action.label.startsWith("Discard ")) ??
      actions.find((action) => action.label.startsWith("Cast ")) ??
      actions.find((action) => action.kind === "pass") ??
      actions[0];
    if (!next) break;
    game.act(next.index);
  }
  assert.ok(sawExileReference, "the chosen seed still exercises Swords to Plowshares");

  assert.ok(sawDestroyed, "creatures dying in combat reach the log");
  assert.ok(sawExiled, "Swords to Plowshares exiling a creature reaches the log");
  assert.ok(
    [...reported].every((line) => /^(Your|Opponent’s) /.test(line)),
    `every line names whose permanent it was: ${[...reported].join(" | ")}`,
  );

  game.free();
});

test("first to two fixes decks and replays sideboarding and play/draw", async () => {
  await initializeWasm();
  const game = new WebGame("The Deck", "Sligh", "Handcrafted", true, 41);
  game.enable_match();
  const read = () => JSON.parse(game.state_json());
  const concede = () => {
    let state = read();
    const keep = state.actions.find((action) => action.label === "Keep this hand");
    if (keep) game.act(keep.index);
    state = read();
    const action = state.actions.find((action) => /concede/i.test(action.label));
    assert.ok(action, "concession available after keeping");
    game.act(action.index);
  };
  try {
    assert.deepEqual(read().match.wins, [0, 0]);
    assert.equal(read().match.stage, "play-draw");
    game.choose_decision(read().decision.id, "[0]");
    concede();
    assert.deepEqual(read().match.wins, [0, 1]);
    assert.equal(read().result, null, "one loss does not end the match");
    assert.equal(read().match.stage, "sideboarding");
    const original = read().match;
    const decision = read().decision;
    const main = original.main.map((_,index) => index);
    assert.throws(() => game.choose_decision(decision.id, JSON.stringify(main.slice(1))));
    assert.equal(read().match.game, 1);
    main[0] = original.main.length;
    game.choose_decision(decision.id, JSON.stringify(main));
    assert.equal(read().match.stage, "play-draw");
    const choices = read().opponentActions.filter(action => action.kind === "choice");
    assert.ok(choices.length > 0);
    assert.ok(choices.every(action => action.label === "Opponent made a private choice"));
    game.choose_decision(read().decision.id, "[1]");
    assert.equal(read().match.game, 2);
    assert.deepEqual(read().match.wins, [0, 1]);
    assert.ok(read().match.main.some(card => card.id === original.sideboard[0].id));
    const replay = WebGame.fromReplayJson(game.replayJson());
    try { assert.deepEqual(JSON.parse(replay.state_json()), read()); } finally { replay.free(); }
    concede();
    assert.deepEqual(read().match.wins, [0, 2]);
    assert.equal(read().match.finished, true);
    assert.equal(read().match.stage, "complete");
  } finally { game.free(); }
});


test("hosted first-to-two decisions and the complete match history replay", async () => {
  await initializeWasm();
  const config = JSON.stringify({ p1Deck: "Sligh", p2Deck: "The Deck", opponent: "external", seed: 41, matchMode: "first-to-two-wins" });
  const game = HostedGame.fromConfigJson(config);
  let sawSideboard = false;
  try {
    for (let step = 0; step < 12000 && game.decisionSeat(); step++) {
      const view = JSON.parse(game.observeJson(game.decisionSeat()));
      if (view.decision) {
        sawSideboard ||= view.match.stage === "sideboarding";
        game.chooseDecision(JSON.stringify(view.decision.options.slice(0, view.decision.minimum).map(option => option.id)));
      } else {
        const actions = view.legalActions;
        game.act((actions.find(action => action.type === "KeepHand") ?? actions.find(action => action.type === "PassPriority") ?? actions[0]).index);
      }
    }
    assert.ok(sawSideboard);
    assert.ok(game.resultJson());
    assert.equal(Math.max(...JSON.parse(game.observeJson("p1")).match.wins), 2);
    const replay = HostedGame.replayConfigJson(config, game.historyJson());
    try { assert.equal(replay.observeJson("p1"), game.observeJson("p1")); } finally { replay.free(); }
  } finally { game.free(); }
});

test("external browser opponents choose play/draw through replayable decisions", async () => {
  await initializeWasm();
  const game = new WebGame("The Deck", "Sligh", "External", false, 41);
  try {
    game.enable_match();
    assert.equal(game.opponentIsDeciding(), true);
    const view = JSON.parse(game.opponentObserveJson());
    assert.equal(view.match.stage, "play-draw");
    assert.equal(view.hand.length, 0);
    game.opponentChooseDecision(view.decision.id, "[1]");
    assert.equal(game.opponentIsDeciding(), false);
    const state = JSON.parse(game.state_json());
    assert.equal(state.human.hand.length, 7);
    assert.equal(state.match.stage, "playing");
  } finally { game.free(); }
});
