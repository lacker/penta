import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test, { after } from "node:test";

import ts from "typescript";

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

class RoomSocket {
  sent = [];
  handlers = new Map();

  accept() {}

  send(message) {
    this.sent.push(message);
  }

  close() {}

  addEventListener(type, handler) {
    this.handlers.set(type, handler);
  }
}

const socketPairs = [];

class TestWebSocketPair {
  constructor() {
    const pair = { 0: new RoomSocket(), 1: new RoomSocket() };
    socketPairs.push(pair);
    return pair;
  }
}

class TestResponse {
  constructor(body = null, init = {}) {
    this.status = init.status ?? 200;
    this.webSocket = init.webSocket;
    this.body = body === null ? "" : String(body);
  }

  static json(value, init = {}) {
    return new TestResponse(JSON.stringify(value), init);
  }

  async json() {
    return JSON.parse(this.body);
  }
}

const SAFE_BEFORE_DRAW = {
  view: "safe-before-draw",
  decision: { actor: "human", kind: "Priority" },
  hand: ["Forest"],
  opponentActions: [],
  result: null,
};

const PRIVATE_MIRACLE_WINDOW = {
  view: "live-private-miracle-window",
  decision: {
    actor: "bot",
    kind: "DrawActionWindow",
    options: [{ id: 1, label: "Reveal Terminus" }],
  },
  hand: ["Forest", "private-draw-must-not-leak"],
  opponentActions: [{ action: "private-draw" }],
  result: null,
};

const SAFE_AFTER_DECLINE = {
  view: "safe-after-decline",
  decision: { actor: "human", kind: "Priority" },
  hand: ["Forest"],
  opponentActions: [],
  result: null,
};

class TestWebGame {
  state = structuredClone(SAFE_BEFORE_DRAW);
  opponentDeciding = false;

  act() {
    this.state = structuredClone(PRIVATE_MIRACLE_WINDOW);
    this.opponentDeciding = true;
  }

  opponentAct() {
    this.state = structuredClone(SAFE_AFTER_DECLINE);
    this.opponentDeciding = false;
  }

  choose_decision() {}
  attack_all() {}
  cancel_attackers() {}
  finalize_blocks() {}
  undo_mana() {}
  set_phase_stop() {}
  set_autopass() {}

  loseOnTime(seat) {
    this.state = { ...this.state, result: { loser: seat } };
    this.opponentDeciding = false;
  }

  opponentIsDeciding() {
    return this.opponentDeciding;
  }

  opponentObserveJson() {
    return JSON.stringify({ privateWindow: this.state.decision });
  }

  state_json() {
    return JSON.stringify(this.state);
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
const originalWebSocketPair = globalThis.WebSocketPair;
globalThis.Response = TestResponse;
globalThis.WebSocketPair = TestWebSocketPair;
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
  globalThis.WebSocketPair = originalWebSocketPair;
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

function assertCredentialsRedacted(record) {
  assert.equal("humanToken" in record, false);
  assert.equal("botToken" in record, false);
}

test("polling and reconnect cannot see an external opponent's private Miracle window", async () => {
  const { GameRoom } = await loadGameRoom();
  const storage = new MemoryStorage();
  const room = new GameRoom(durableState(storage));
  const started = await (
    await room.fetch(
      request("start", {
        body: {
          humanDeck: "human deck",
          botDeck: "bot deck",
          botPolicy: "external",
          humanFirst: true,
          seed: 7,
        },
      }),
    )
  ).json();
  const safeState = structuredClone(started.state);
  assert.equal(safeState.moveClock.seat, "human");

  const commandResponse = await (
    await room.fetch(
      request("command", {
        token: started.humanToken,
        body: { t: "act", index: 0 },
      }),
    )
  ).json();
  assert.deepEqual(commandResponse, safeState);
  assert.equal(storage.values.get("move-clock").seat, "bot");

  const polled = await (
    await room.fetch(request("state", { token: started.humanToken }))
  ).json();
  assert.deepEqual(polled, safeState);
  assert.equal(JSON.stringify(polled).includes("private"), false);
  const privateRecord = await (
    await room.fetch(request("record", { token: started.humanToken }))
  ).json();
  assert.deepEqual(
    privateRecord.commands,
    [],
    "the live replay journal cannot disclose a private opponent command",
  );
  assertCredentialsRedacted(privateRecord);

  // Reconstruct the Durable Object as a real reconnect may. Replay reaches
  // the private window, but the persisted safe projection must win wholesale:
  // even its old human clock remains frozen until the opponent answers.
  const restarted = new GameRoom(durableState(storage));
  const restartedPoll = await (
    await restarted.fetch(request("state", { token: started.humanToken }))
  ).json();
  assert.deepEqual(restartedPoll, safeState);

  const connection = await restarted.fetch(
    request(`ws?role=human&token=${started.humanToken}`),
  );
  assert.equal(connection.status, 101);
  const humanSocket = socketPairs.at(-1)[1];
  const reconnectMessage = JSON.parse(humanSocket.sent.at(-1));
  assert.deepEqual(reconnectMessage, { t: "state", state: safeState });

  await restarted.fetch(
    request("command", {
      token: started.botToken,
      body: { t: "botAct", index: 0 },
    }),
  );
  const resumedMessage = JSON.parse(humanSocket.sent.at(-1));
  assert.equal(resumedMessage.state.view, "safe-after-decline");
  assert.deepEqual(
    resumedMessage.state.opponentActions,
    [],
    "declining the private Miracle window produces no observable choice beat",
  );
  assert.equal(storage.values.get("human-state").commandCount, 2);
  const resumedRecord = await (
    await restarted.fetch(request("record", { token: started.humanToken }))
  ).json();
  assert.deepEqual(
    resumedRecord.commands,
    [],
    "a hidden decline remains absent from the live record after play resumes",
  );
  assertCredentialsRedacted(resumedRecord);

  await restarted.fetch(
    request("lose-on-time", { body: { seat: "human" } }),
  );
  const finishedRecord = await (
    await restarted.fetch(request("record", { token: started.humanToken }))
  ).json();
  assert.ok(finishedRecord.commands.length > 0);
  assertCredentialsRedacted(finishedRecord);
});

test("built-in game records do not expose either seat token", async () => {
  const { GameRoom } = await loadGameRoom();
  const room = new GameRoom(durableState(new MemoryStorage()));
  const started = await (
    await room.fetch(
      request("start", {
        body: {
          humanDeck: "human deck",
          botDeck: "bot deck",
          botPolicy: "random",
          humanFirst: true,
          seed: 7,
        },
      }),
    )
  ).json();
  const record = await (
    await room.fetch(request("record", { token: started.humanToken }))
  ).json();
  assertCredentialsRedacted(record);
});
