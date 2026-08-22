import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test, { after } from "node:test";

import ts from "typescript";

/**
 * Open decklists is a mutual, opt-in-only disclosure: the bot seat's
 * observation gains `opponentDeck` -- naming the human seat's deck -- only
 * once both the human seat (declared at `start`) and the bot seat (declared
 * by the registry, via `disclose-bot-deck`, once a challenge actually
 * succeeds) have opted in. Neither side opting in, by itself, is enough.
 *
 * This harness mirrors `game-room-cache.test.mjs`: `GameRoom` is transpiled
 * from source with its worker-only imports swapped for injected globals, so
 * these tests run under plain `node --test` without a Workers runtime.
 */

class MemoryStorage {
  values = new Map();
  alarm = null;

  async get(key) {
    const value = this.values.get(key);
    return value === undefined ? undefined : structuredClone(value);
  }

  async put(key, value) {
    this.values.set(key, structuredClone(value));
  }

  async delete(key) {
    return this.values.delete(key);
  }

  async setAlarm(time) {
    this.alarm = time;
  }

  async deleteAlarm() {
    this.alarm = null;
  }
}

class TestResponse {
  constructor(body = null, init = {}) {
    this.status = init.status ?? 200;
    this.body = body === null ? "" : String(body);
  }

  static json(value, init = {}) {
    return new TestResponse(JSON.stringify(value), init);
  }

  async json() {
    return JSON.parse(this.body);
  }
}

/** Just enough of `WebGame` for the bot seat to hold a pending decision. */
class TestWebGame {
  opponentObserveJson() {
    return JSON.stringify({ decision: { actor: "bot", kind: "Priority" } });
  }

  opponentIsDeciding() {
    return true;
  }

  state_json() {
    return JSON.stringify({ result: null });
  }
}

class TestHostedGame {
  static replayVersion() {
    return 1;
  }

  static simulationFingerprint() {
    return "test-fingerprint";
  }

  static engineVersion() {
    return "test-engine";
  }

  static protocolVersion() {
    return 1;
  }
}

const originalResponse = globalThis.Response;
globalThis.Response = TestResponse;
globalThis.__gameRoomEngine = async () => ({
  WebGame: TestWebGame,
  HostedGame: TestHostedGame,
});
globalThis.__replayCompatibilityError = () => null;
globalThis.__botPresence = {
  FINISHED_ROOM_MS: 60_000,
  moveBudgetMs: (seat) => (seat === "bot" ? 1_000 : 10_000),
};

after(() => {
  globalThis.Response = originalResponse;
  delete globalThis.__gameRoomEngine;
  delete globalThis.__replayCompatibilityError;
  delete globalThis.__botPresence;
});

async function loadGameRoom() {
  let source = await readFile(
    new URL("../worker/game-room.ts", import.meta.url),
    "utf8",
  );
  const replacements = [
    [
      'import { type EngineModule, engine } from "./engine";',
      "const engine = globalThis.__gameRoomEngine;",
    ],
    [
      'import { replayCompatibilityError } from "./replay-compatibility.mjs";',
      "const replayCompatibilityError = globalThis.__replayCompatibilityError;",
    ],
    [
      'import { FINISHED_ROOM_MS, moveBudgetMs } from "./bot-presence.mjs";',
      "const { FINISHED_ROOM_MS, moveBudgetMs } = globalThis.__botPresence;",
    ],
  ];
  for (const [from, to] of replacements) {
    assert.ok(source.includes(from), `test loader expected ${from}`);
    source = source.replace(from, to);
  }
  const javascript = ts.transpileModule(source, {
    compilerOptions: {
      module: ts.ModuleKind.ESNext,
      target: ts.ScriptTarget.ES2022,
    },
  }).outputText;
  const encoded = Buffer.from(javascript).toString("base64");
  return import(`data:text/javascript;base64,${encoded}`);
}

function durableState(storage) {
  return {
    storage,
    blockConcurrencyWhile: (callback) => callback(),
  };
}

function request(route, { token, body } = {}) {
  const headers = token ? { "x-penta-token": token } : undefined;
  return new Request(`https://room.test/${route}`, {
    method: body === undefined ? "GET" : "POST",
    headers,
    body: body === undefined ? undefined : JSON.stringify(body),
  });
}

/** Starts a room and returns its tokens, optionally declaring the human seat's opt-in. */
async function startRoom(room, { humanDiscloseDeck } = {}) {
  const response = await room.fetch(
    request("start", {
      body: {
        humanDeck: "The Deck",
        botDeck: "Sligh",
        botPolicy: "external",
        humanFirst: true,
        seed: 7,
        ...(humanDiscloseDeck === undefined ? {} : { humanDiscloseDeck }),
      },
    }),
  );
  return response.json();
}

async function opponentObservation(room, botToken) {
  const response = await room.fetch(request("opponent", { token: botToken }));
  const view = await response.json();
  assert.equal(view.deciding, true, "the bot seat should hold the decision in this fixture");
  return view.observation;
}

test("neither side opting in: the bot's observation is unchanged", async () => {
  const { GameRoom } = await loadGameRoom();
  const room = new GameRoom(durableState(new MemoryStorage()));
  const { botToken } = await startRoom(room);
  const observation = await opponentObservation(room, botToken);
  assert.equal("opponentDeck" in observation, false);
});

test("both sides opted in: the bot's observation names the human seat's deck", async () => {
  const { GameRoom } = await loadGameRoom();
  const room = new GameRoom(durableState(new MemoryStorage()));
  const { botToken } = await startRoom(room, { humanDiscloseDeck: true });
  await room.fetch(
    request("disclose-bot-deck", { body: { discloseDeck: true } }),
  );
  const observation = await opponentObservation(room, botToken);
  assert.equal(observation.opponentDeck, "The Deck");
});

test("only the human seat opted in: nothing is disclosed", async () => {
  const { GameRoom } = await loadGameRoom();
  const room = new GameRoom(durableState(new MemoryStorage()));
  const { botToken } = await startRoom(room, { humanDiscloseDeck: true });
  // No disclose-bot-deck call: the bot that filled the seat never opted in.
  const observation = await opponentObservation(room, botToken);
  assert.equal("opponentDeck" in observation, false);
});

test("only the bot seat opted in: nothing is disclosed", async () => {
  const { GameRoom } = await loadGameRoom();
  const room = new GameRoom(durableState(new MemoryStorage()));
  const { botToken } = await startRoom(room);
  await room.fetch(
    request("disclose-bot-deck", { body: { discloseDeck: true } }),
  );
  const observation = await opponentObservation(room, botToken);
  assert.equal("opponentDeck" in observation, false);
});

test("an explicit false from either side still withholds disclosure", async () => {
  const { GameRoom } = await loadGameRoom();
  const room = new GameRoom(durableState(new MemoryStorage()));
  const { botToken } = await startRoom(room, { humanDiscloseDeck: true });
  await room.fetch(
    request("disclose-bot-deck", { body: { discloseDeck: false } }),
  );
  const observation = await opponentObservation(room, botToken);
  assert.equal("opponentDeck" in observation, false);
});
