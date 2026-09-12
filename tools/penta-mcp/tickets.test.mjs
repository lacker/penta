import assert from "node:assert/strict";
import test from "node:test";
import { SessionClient } from "./client.mjs";

const actions = [{ index: 7, type: "PlayLand", card: 22 }, { index: 9, type: "PassPriority" }];
const ready = revision => ({ apiVersion: 1, status: "ready", revision, observation: {
  seat: "p1", decision: null, hand: [{ objectId: 22, name: "Forest" }], legalActions: actions, checkpoint: { data: "exact" },
} });
const unpack = table => Array.isArray(table) ? table : table.rows.map(row => ({ ...table.shared, ...row }));
function fixture() {
  const state = { view: ready("a"), posts: [], reads: 0, fail: null, commits: 0, receipts: new Map() };
  state.client = new SessionClient("http://penta.test", async (url, init) => {
    if (url.endsWith("/catalog")) {
      if (state.catalogFailure) throw new Error("catalog unavailable");
      return Response.json({ cards: [] });
    }
    if (!init.body) { state.reads++; return Response.json(state.view); }
    const body = JSON.parse(init.body); state.posts.push(body);
    if (state.block) await state.block;
    if (state.fail === "stale") return Response.json({ error: "stale revision; refresh" }, { status: 409 });
    if (!state.receipts.has(body.requestId)) { state.commits++; state.receipts.set(body.requestId, body); }
    if (state.fail === "network") throw new Error("lost after commit");
    return Response.json({ ...state.view, receipt: { accepted: 1, requestId: body.requestId } });
  });
  state.attach = () => state.client.attach({ room: "r", token: "own-seat", presentation: "decision-v1" });
  return state;
}

test("tickets submit exact stored indices once and expire after the next submission", async () => {
  const f = fixture(); const first = await f.attach();
  const [land, pass] = unpack(first.choices);
  f.view = ready("b");
  const next = await f.client.choose({ ticket: land.ticket, waitMs: 0 });
  assert.deepEqual(f.posts[0].choices, [{ index: 7 }]);
  assert.equal(f.posts[0].revision, "a");
  await f.client.choose({ ticket: land.ticket, waitMs: 0 });
  assert.deepEqual(f.posts[0], f.posts[1]); assert.equal(f.commits, 1);
  await assert.rejects(f.client.choose({ ticket: pass.ticket }), /expired/);
  await assert.rejects(f.client.inspectReference({ reference: first.provenance.reference }), /expired/);
  f.view = ready("c");
  await f.client.choose({ ticket: unpack(next.choices)[1].ticket, waitMs: 0 });
  await assert.rejects(f.client.choose({ ticket: land.ticket }), /expired/);
  assert.equal(f.commits, 2);
});

test("ticket retries retain request IDs after commit loss and block other selections", async () => {
  const f = fixture(); const first = await f.attach(); const [land, pass] = unpack(first.choices);
  f.fail = "network";
  await assert.rejects(f.client.choose({ ticket: land.ticket, waitMs: 0 }), /lost after commit/);
  await assert.rejects(f.client.choose({ ticket: pass.ticket, waitMs: 0 }), /uncertain outcome/);
  await assert.rejects(f.client.choose({ ticket: land.ticket, options: [] }), /cannot change/);
  f.fail = null; f.view = ready("b");
  await f.client.retry({ connection: first.connection, waitMs: 0 });
  assert.deepEqual(f.posts[0], f.posts[1]); assert.equal(f.commits, 1);
  await f.client.choose({ ticket: land.ticket, waitMs: 0 });
  assert.deepEqual(f.posts[1], f.posts[2]); assert.equal(f.commits, 1);
});

test("definite stale failures need fresh tickets; waiting and raw inspection do not leak or consume views", async () => {
  const f = fixture(); const first = await f.attach(); const [land, pass] = unpack(first.choices);
  f.fail = "stale";
  await assert.rejects(f.client.choose({ ticket: land.ticket, waitMs: 0 }), /stale revision/);
  await assert.rejects(f.client.choose({ ticket: pass.ticket }), /stale ticket/);
  await assert.rejects(f.client.retry({ connection: first.connection }), /no uncertain/);
  f.fail = null;
  f.view = { apiVersion: 1, status: "waiting", role: "human" };
  assert.deepEqual(await f.client.next({ connection: first.connection, waitMs: 0 }), f.view);
  const reads = f.reads;
  assert.deepEqual((await f.client.inspectReference({ reference: first.provenance.reference })).items[0].checkpoint, { data: "exact" });
  assert.equal(f.reads, reads, "frozen inspection never fetches a newer position");
  f.view = ready("b");
  const fresh = await f.client.next({ connection: first.connection, full: true, waitMs: 0 });
  assert.notEqual(fresh.view, first.view);
  const raw = await f.client.inspect({ connection: first.connection, section: "observation" });
  assert.deepEqual(raw.observation.legalActions, actions);
  await f.client.choose({ ticket: unpack(fresh.choices)[1].ticket, waitMs: 0 });
});

test("connections isolate tickets and serialize calls without losing uncertain receipts", async () => {
  const f = fixture(); const first = await f.attach(); const second = await f.attach();
  assert.notEqual(first.view, second.view);
  const other = fixture(); await other.attach();
  await assert.rejects(other.client.choose({ ticket: unpack(first.choices)[0].ticket }), /unknown or expired/);
  let release; f.block = new Promise(resolve => { release = resolve; });
  const pending = f.client.choose({ ticket: unpack(first.choices)[0].ticket, waitMs: 0 });
  await assert.rejects(f.client.next({ connection: first.connection, waitMs: 0 }), /already in progress/);
  await assert.rejects(f.client.choose({ ticket: unpack(first.choices)[0].ticket }), /already in progress/);
  assert.equal((await f.client.next({ connection: second.connection, waitMs: 0 })).view, second.view);
  release(); await pending;
  assert.equal(f.commits, 1);
});

test("optional catalog failure after a committed ticket does not create an uncertain play", async () => {
  const f = fixture(); f.catalogFailure = true;
  const first = await f.attach(); f.view = ready("b");
  const result = await f.client.choose({ ticket: unpack(first.choices)[0].ticket, waitMs: 0 });
  assert.equal(result.receipt.accepted, 1);
  await assert.rejects(f.client.retry({ connection: first.connection }), /no uncertain/);
});

test("decision tickets send selected IDs in model order and reject changed retries", async () => {
  const f = fixture();
  f.view.observation.decision = { id: 10, minimum: 0, maximum: 2, options: [{ id: 8 }, { id: 3 }] };
  f.view.observation.legalActions = [{ index: 99, type: "ChooseDecision", decision: 10 }];
  const first = await f.attach(); const ticket = unpack(first.choices)[0].ticket;
  await assert.rejects(f.client.choose({ ticket }), /explicit options/);
  f.view = ready("b");
  await f.client.choose({ ticket, options: [3, 8], waitMs: 0 });
  assert.deepEqual(f.posts[0].choices, [{ decision: 10, options: [3, 8] }]);
  await assert.rejects(f.client.choose({ ticket, options: [8, 3] }), /cannot change/);
});
