import assert from "node:assert/strict";
import test, { after } from "node:test";

import {
  MemoryStorage,
  durableState,
  installRoomGlobals,
  loadGameRoom,
  request,
  restoreRoomGlobals,
  socketPairs,
} from "./game-room-support.mjs";

/**
 * What a reconnecting or polling human seat is allowed to see, and what the
 * room may not let slip while the opponent seat is mid-decision.
 */

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

const constructorArgs = [];

class TestWebGame {
  state = structuredClone(SAFE_BEFORE_DRAW);
  tournament = false;
  enable_match() { this.tournament = true; }
  matchStage() { return this.tournament ? "sideboarding" : "playing"; }
  opponentChooseDecision(decision, options) {
    assert.equal(decision, 42);
    assert.deepEqual(JSON.parse(options), [0, 2]);
    this.opponentAct();
  }
  opponentDeciding = false;

  constructor(...args) {
    constructorArgs.push(args);
  }

  static withArtPreference(...args) {
    return new TestWebGame(...args);
  }

  act() {
    this.state = structuredClone(PRIVATE_MIRACLE_WINDOW);
    this.opponentDeciding = true;
  }

  opponentConcede() {
    this.state = { ...this.state, result: { loser: "bot", reason: "OpponentConceded" } };
    this.opponentDeciding = false;
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

  isFinished() {
    return Boolean(this.state.result);
  }

  resultJson() {
    return this.state.result ? JSON.stringify(this.state.result) : undefined;
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

installRoomGlobals({
  WebGame: TestWebGame,
  presence: { FINISHED_ROOM_MS: 60_000, moveBudgetMs: (seat) => (seat === "bot" ? 1_000 : 10_000) },
});
after(restoreRoomGlobals);

let GameRoom;
test.before(async () => {
  ({ GameRoom } = await loadGameRoom());
});

function assertCredentialsRedacted(record) {
  assert.equal("humanToken" in record, false);
  assert.equal("botToken" in record, false);
}

test("polling and reconnect cannot see an external opponent's private Miracle window", async () => {
  constructorArgs.length = 0;
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
          artPreference: "format-matching",
        },
      }),
    )
  ).json();
  const safeState = structuredClone(started.state);
  assert.equal(constructorArgs.at(-1)[6], "format-matching");
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
  assert.equal(constructorArgs.length, 2, "the restarted room rebuilt its engine");
  assert.equal(constructorArgs.at(-1)[6], "format-matching");
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


test("tournament rooms persist bot sideboard selections without a move clock", async () => {
  const storage = new MemoryStorage();
  const room = new GameRoom(durableState(storage));
  const started = await (await room.fetch(request("start", { body: {
    humanDeck: "human deck", botDeck: "bot deck", botPolicy: "external",
    humanFirst: true, seed: 7, matchMode: "first-to-two-wins",
  } }))).json();
  assert.equal(started.state.moveClock, undefined);
  await room.fetch(request("command", { token: started.humanToken, body: { t: "act", index: 0 } }));
  assert.equal(storage.values.has("move-clock"), false);
  assert.equal(storage.alarm, null);
  const selected = { t: "botChoose", decision: 42, options: [0, 2] };
  const applied = await room.fetch(request("command", { token: started.botToken, body: selected }));
  assert.equal(applied.status, 200);
  const stored = [...storage.values.values()].find(value => value?.commands);
  assert.deepEqual(stored.commands.at(-1), selected);
  const reloaded = new GameRoom(durableState(storage));
  const state = await (await reloaded.fetch(request("state", { token: started.humanToken }))).json();
  assert.equal(state.view, "safe-after-decline");
});


test("only the authenticated bot can submit a replayed bot concession", async () => {
  const storage = new MemoryStorage();
  const room = new GameRoom(durableState(storage));
  const started = await (await room.fetch(request("start", { body: {
    humanDeck: "human deck", botDeck: "bot deck", botPolicy: "external", humanFirst: true, seed: 7,
  } }))).json();
  const refused = await room.fetch(request("command", {
    token: started.humanToken, body: { t: "botConcede" },
  }));
  assert.equal(refused.status, 403);
  const accepted = await room.fetch(request("command", {
    token: started.botToken, body: { t: "botConcede" },
  }));
  assert.equal(accepted.status, 200);
  const record = await (await room.fetch(request("record", { token: started.humanToken }))).json();
  assert.deepEqual(record.commands, [{ t: "botConcede" }]);
  const restored = new GameRoom(durableState(storage));
  const view = await (await restored.fetch(request("state", { token: started.humanToken }))).json();
  assert.equal(view.result.loser, "bot");
});
