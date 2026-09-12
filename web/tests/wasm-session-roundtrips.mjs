import assert from "node:assert/strict";
import test from "node:test";
import { initializeWasm, WebGame } from "./wasm-test-support.mjs";
import { SessionClient } from "../../tools/penta-mcp/client.mjs";
import * as support from "./game-room-support.mjs";

const rows = value => Array.isArray(value) ? value : value.rows.map(row => ({ ...value.shared, ...row }));
const exact = source => {
  const action = { ...source };
  delete action.index;
  return { action };
};

test("session API grouped choices need one submission for mulligan cards, attackers and blockers", async t => {
  await initializeWasm();
  const { HostedGame } = await import("../app/wasm/penta_wasm.js");
  const HttpResponse = globalThis.Response;
  let game;
  // Fix only the test's deal, leaving credential/revision randomness intact.
  // Keep the real factory so subsequent room tests and replay see the same seed.
  const random = crypto.getRandomValues.bind(crypto);
  t.mock.method(crypto, "getRandomValues", array => {
    if (array instanceof Uint32Array && array.length === 1) { array[0] = 42; return array; }
    return random(array);
  });
  const createGame = WebGame.withArtPreference;
  t.mock.method(WebGame, "withArtPreference", (...args) => {
    game = createGame(...args);
    return game;
  });
  support.installRoomGlobals({ WebGame, HostedGame });
  try {
    const { GameRoom } = await support.loadGameRoom();
    const room = new GameRoom(support.durableState());
    const started = await room.fetch(support.request("start", { body: {
      format: "old-school-93-94", humanDeck: "White Weenie", botDeck: "White Weenie",
      humanFirst: true, botPolicy: "external", seed: 42, sessionApi: true,
    } }));
    const opened = await started.json();
    assert.equal(started.status, 200);
    const requests = [];
    const client = new SessionClient("http://localhost", async (url, init) => {
      requests.push({ url, body: init.body });
      const response = await room.fetch(new Request(url, init));
      return HttpResponse.json(await response.json(), { status: response.status });
    });
    const connections = {};
    for (const [role, token] of [["human", opened.humanToken], ["bot", opened.botToken]]) {
      connections[role] = (await client.attach({ room: "test", token, presentation: "decision-v1" })).connection;
    }
    const raw = role => JSON.parse(game.sessionObserveJson(role));
    const submit = async (role, choices, useTickets = false) => {
      const view = await client.next({ connection: connections[role], waitMs: 0 });
      if (useTickets) {
        const offered = rows(view.choices);
        choices = choices.map(choice => {
          // A finish command can become legal only after the declarations.
          // Keep that explicit command; the engine checks it at its own step.
          if (choice.action.type.startsWith("Finish")) return choice;
          const action = raw(role).legalActions.find(action => JSON.stringify(exact(action)) === JSON.stringify(choice));
          assert.ok(action);
          return { ticket: offered.find(choice => choice.index === action.index).ticket };
        });
        assert.equal(choices.filter(choice => choice.ticket).length, 2);
      }
      const before = requests.length;
      const result = await client.play({ connection: connections[role], revision: view.revision, choices, waitMs: 0 });
      assert.equal(requests.length - before, 1, "play returns the next view without a separate observation request");
      return result;
    };
    // Setup explicitly makes two mulligans. The resulting two-card selection
    // itself must need just one ticket, not two card-by-card submissions.
    for (const type of ["TakeMulligan", "TakeMulligan", "KeepHand"]) {
      await submit("human", [exact(raw("human").legalActions.find(action => action.type === type))]);
    }
    const bottom = raw("human").legalActions.find(action => action.type === "BottomCards");
    assert.equal(bottom.cards.length, 2);
    const view = await client.next({ connection: connections.human, waitMs: 0 });
    const ticket = rows(view.choices).find(choice => choice.index === bottom.index).ticket;
    const before = requests.length;
    const bottomed = await client.choose({ ticket, waitMs: 0 });
    assert.equal(requests.length - before, 1);
    assert.equal(bottomed.receipt.accepted, 1);
    assert.equal(raw("human").hand.length, 5);
    assert.ok(bottom.cards.every(id => !raw("human").hand.some(card => card.objectId === id)));

    let attacked = false;
    let blocked = false;
    for (let step = 0; step < 500 && !blocked; step++) {
      const role = game.sessionDecisionRole();
      assert.ok(role, "the scripted fixture reaches combat before the game ends");
      const observation = raw(role);
      const actions = observation.legalActions;
      assert.equal(observation.forcedAction, null, "forced continuations do not require model submissions");
      const attackers = actions.filter(action => action.type === "DeclareAttacker");
      const defenders = observation.battlefield.filter(card => card.controller !== observation.seat && !card.tapped && card.power > 0);
      if (!attacked && attackers.length >= 2 && defenders.length >= 2) {
        const chosen = [...new Map(attackers.map(action => [action.attacker, action])).values()].slice(0, 2);
        const result = await submit(role, [...chosen.map(exact), { action: { type: "FinishDeclaringAttackers" } }], true);
        assert.ok(result.receipt.accepted >= 2);
        assert.equal(raw(role).battlefield.filter(card => card.attacking).length, 2);
        assert.ok(!raw(role).legalActions.some(action => action.type === "FinishDeclaringAttackers"));
        attacked = true;
        continue;
      }
      const blocks = actions.filter(action => action.type === "DeclareBlocker");
      if (attacked && blocks.length) {
        const first = blocks[0];
        const second = blocks.find(action => action.blocker !== first.blocker && action.attacker !== first.attacker);
        assert.ok(second, "two distinct blockers can block the two selected attackers");
        const result = await submit(role, [exact(first), exact(second), { action: { type: "FinishDeclaringBlockers" } }], true);
        assert.ok(result.receipt.accepted >= 2);
        assert.ok(!raw(role).legalActions.some(action => action.type === "FinishDeclaringBlockers"));
        blocked = true;
        continue;
      }
      // Deterministic fixture setup only; this is not a production play policy.
      const creature = action => observation.hand.some(card => card.objectId === action.card && /Savannah Lions|White Knight|Icatian Javelineers|Order of Leitbur/.test(card.name));
      const action = actions.find(action => action.type === "KeepHand")
        ?? actions.find(action => action.type === "PlayLand")
        ?? actions.find(action => action.type === "CastSpell" && creature(action))
        ?? actions.find(action => action.type === "FinishDeclaringAttackers")
        ?? actions.find(action => action.type === "PassPriority")
        ?? actions.find(action => action.type === "DiscardCards")
        ?? actions[0];
      await submit(role, [exact(action)]);
    }
    assert.ok(attacked && blocked, JSON.stringify({ attacked, blocked, turn: raw("human").turn, board: raw("human").battlefield.map(card => [card.name, card.controller]), step: raw("human").step }));
  } finally { game?.free(); support.restoreRoomGlobals(); }
});
