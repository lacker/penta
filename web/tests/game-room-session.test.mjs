import assert from "node:assert/strict";
import { after, before, test } from "node:test";
import { MemoryStorage, durableState, installRoomGlobals, loadGameRoom, request, restoreRoomGlobals, socketPairs } from "./game-room-support.mjs";

class Engine {
  turn = 0;
  stage = "playing";
  exact = false;
  constructor(_a, _b, _policy, first) { this.first = first; }
  enableSessionApi() { this.exact = true; }
  enable_match() {}
  sessionDecisionRole() { return this.turn >= 6 ? undefined : this.turn % 2 ? "bot" : "human"; }
  isFinished() { return this.turn >= 6; }
  opponentIsDeciding() { return this.sessionDecisionRole() === "bot"; }
  matchStage() { return this.stage; }
  state_json() { return JSON.stringify({ human: { hand: ["human-only"] }, turn: this.turn, result: this.isFinished() ? { winner: "human" } : null, opponentActions: [] }); }
  sessionObserveJson(role) {
    return JSON.stringify({ seat: role === "human" ? "p1" : "p2", hand: [role + "-only"],
      decision: null, legalActions: this.sessionDecisionRole() === role ? [{ index: 0, type: "PassPriority" }] : [],
      checkpoint: { visible: role }, result: this.isFinished() ? { winner: "p1" } : null });
  }
  opponentObserveJson() { return this.sessionObserveJson("bot"); }
  sessionCatalogJson() { return JSON.stringify({ cards: [{ definition: 1, name: "Mountain" }] }); }
  sessionAct(role, index) {
    assert.equal(role, this.sessionDecisionRole());
    assert.equal(index, 0);
    this.turn++;
  }
  act(index) { this.sessionAct("human", index); }
  opponentAct(index) { this.sessionAct("bot", index); }
  resultJson() { return this.isFinished() ? '{"winner":"human"}' : undefined; }
}

let GameRoom;
before(async () => {
  installRoomGlobals({ WebGame: Engine });
  ({ GameRoom } = await loadGameRoom());
});
after(restoreRoomGlobals);

async function setup(storage = new MemoryStorage()) {
  const room = new GameRoom(durableState(storage));
  const opened = await (await room.fetch(request("start", { body: {
    humanDeck: "A", botDeck: "B", botPolicy: "external", humanFirst: true,
    seed: 1, sessionApi: true, matchMode: "first-to-two-wins",
  } }))).json();
  assert.ok(opened.humanToken);
  return { storage, room, ...opened };
}
async function view(room, token) { return (await room.fetch(request("session", { token }))).json(); }

test("session API serves both seats, waits without disclosing private windows, and survives reconnect", async () => {
  const { room, storage, humanToken, botToken } = await setup();
  assert.equal((await room.fetch(request("session"))).status, 403);
  assert.deepEqual(await view(room, botToken), { apiVersion: 1, status: "waiting", role: "bot" });
  const first = await view(room, humanToken);
  assert.deepEqual(first.observation.hand, ["human-only"]);
  assert.equal(typeof first.revision, "string");
  const waiting = room.fetch(request("session?wait=1000", { token: botToken }));
  const body = { revision: first.revision, requestId: "first", choices: [{ index: 0 }] };
  const moved = await (await room.fetch(request("play", { token: humanToken, body }))).json();
  assert.equal(moved.receipt.accepted, 1);
  assert.equal(moved.status, "waiting");
  assert.equal("revision" in moved, false);
  const next = await (await waiting).json();
  assert.deepEqual(next.observation.hand, ["bot-only"]);
  assert.notEqual(next.revision, first.revision);
  assert.equal(storage.alarm, null, "external session decisions have no imposed move clock");
  const reconnected = new GameRoom(durableState(storage));
  assert.deepEqual(await view(reconnected, botToken), next);
  const retried = await (await reconnected.fetch(request("play", { token: humanToken, body }))).json();
  assert.equal(retried.receipt.accepted, 1);
  assert.equal(storage.values.get("hosted-game").commands.length, 1);
  assert.equal((await reconnected.fetch(request("play", { token: humanToken,
    body: { ...body, choices: [{ index: 1 }] } }))).status, 409);
});

test("session API serializes competing requests and rejects stale browser commands", async () => {
  const { room, storage, humanToken } = await setup();
  const first = await view(room, humanToken);
  const responses = await Promise.all(["a", "b"].map(requestId => room.fetch(request("play", {
    token: humanToken, body: { revision: first.revision, requestId, choices: [{ index: 0 }] },
  }))));
  assert.deepEqual(responses.map(response => response.status).sort(), [200, 409]);
  assert.equal(storage.values.get("hosted-game").commands.length, 1);
  const stale = await room.fetch(request("command", { token: humanToken, body: { t: "act", index: 0 } }));
  assert.equal(stale.status, 409);
});

test("session API reloads durable receipts after an uncertain storage commit", async () => {
  for (const committed of [false, true]) {
    class Storage extends MemoryStorage {
      fail = false;
      async put(key, value) {
        if (key === "hosted-game" && this.fail) {
          this.fail = false;
          if (committed) await super.put(key, value);
          throw new Error("storage connection lost");
        }
        return super.put(key, value);
      }
    }
    const { room, storage, humanToken } = await setup(new Storage());
    const first = await view(room, humanToken);
    const body = { revision: first.revision, requestId: "uncertain", choices: [{ index: 0 }] };
    storage.fail = true;
    assert.equal((await room.fetch(request("play", { token: humanToken, body }))).status, 503);
    const retry = await (await room.fetch(request("play", { token: humanToken, body }))).json();
    assert.equal(retry.receipt.accepted, 1);
    assert.equal(storage.values.get("hosted-game").commands.length, 1);
  }
});

test("session API preserves retry semantics when notification fails after the play commits", async () => {
  class Storage extends MemoryStorage {
    fail = false;
    async delete(key) {
      if (this.fail) { this.fail = false; throw new Error("clock update failed after commit"); }
      return super.delete(key);
    }
  }
  const { room, storage, humanToken, botToken } = await setup(new Storage());
  const first = await view(room, humanToken);
  const body = { revision: first.revision, requestId: "committed", choices: [{ index: 0 }] };
  storage.fail = true;
  assert.equal((await room.fetch(request("play", { token: humanToken, body }))).status, 503);
  const retry = await (await room.fetch(request("play", { token: humanToken, body }))).json();
  assert.equal(retry.receipt.accepted, 1);
  assert.equal(storage.values.get("hosted-game").commands.length, 1);
  const bot = await view(room, botToken);
  const reply = { revision: bot.revision, requestId: "bot-committed", choices: [{ index: 0 }] };
  storage.fail = true;
  assert.equal((await room.fetch(request("play", { token: botToken, body: reply }))).status, 503);
  assert.equal((await room.fetch(request("play", { token: botToken, body: reply }))).status, 200);
  assert.equal((await (await room.fetch(request("state", { token: humanToken }))).json()).turn, 2);
});

test("session API prevents a human socket from controlling the other seat or forcing a timeout", async () => {
  const { room, humanToken, storage } = await setup();
  const first = await view(room, humanToken);
  await room.fetch(request("ws?role=human", { token: humanToken }));
  const socket = socketPairs.at(-1)[1];
  for (const command of [{ t: "sessionAct", role: "bot", index: 0 }, { t: "loseOnTime", seat: "bot" }]) {
    socket.handlers.get("message")({ data: JSON.stringify({ ...command, revision: first.revision }) });
    assert.equal(JSON.parse(socket.sent.at(-1)).t, "error");
  }
  assert.equal(storage.values.get("hosted-game").commands.length, 0);
});

test("session API batches explicit actions, reports an accepted prefix, and never reuses indices", async () => {
  const { room, storage, humanToken } = await setup();
  const first = await view(room, humanToken);
  const move = await (await room.fetch(request("play", { token: humanToken, body: {
    revision: first.revision, requestId: "batch", choices: [
      { action: { type: "PassPriority" } }, { action: { type: "PassPriority" } },
    ],
  } }))).json();
  assert.equal(move.receipt.accepted, 1);
  assert.match(move.receipt.stopped, /does not hold/);
  assert.equal(storage.values.get("hosted-game").commands.length, 1);
  assert.equal((await room.fetch(request("play", { token: humanToken, body: {
    revision: first.revision, requestId: "bad", choices: [{ index: 0 }, { index: 0 }],
  } }))).status, 400);
});

test("session API keeps every live replay private and exposes completed replay provenance", async () => {
  const { room, humanToken, botToken } = await setup();
  for (let i = 0; i < 6; i++) {
    const token = i % 2 ? botToken : humanToken;
    const before = await view(room, token);
    for (const credential of [humanToken, botToken]) {
      assert.equal((await room.fetch(request("record", { token: credential }))).status, 403,
        "live records must not disclose either registered deck to a playing seat");
    }
    await room.fetch(request("play", { token, body: {
      revision: before.revision, requestId: `move-${i}`, choices: [{ index: 0 }],
    } }));
  }
  const done = await view(room, botToken);
  assert.equal(done.status, "complete");
  const record = await (await room.fetch(request("record", { token: humanToken }))).json();
  assert.deepEqual(await (await room.fetch(request("record", { token: botToken }))).json(), record);
  assert.equal(record.commands.length, 6);
  assert.equal("humanToken" in record, false);
  assert.equal("receipts" in record, false);
});
