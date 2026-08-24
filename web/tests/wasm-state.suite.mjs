import assert from "node:assert/strict";
import test from "node:test";

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
