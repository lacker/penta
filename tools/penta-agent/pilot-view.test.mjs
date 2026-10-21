import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";
import { PilotView } from "./pilot-view.mjs";
import { validateContract, validatePilot, pilotSchema } from "./pilot-contract.mjs";
import { declarations } from "./generate-pilot-types.mjs";
import { nativeCommand } from "./native-client.mjs";

const observation = {
  protocolVersion: 33, seat: "p1", activeSeat: "p2", prioritySeat: "p1", step: "End",
  life: [20, 19], manaPools: [{ white: 0 }, { red: 1 }], hand: [], battlefield: [],
  graveyards: [[], []], legalActions: [], updates: [], decision: null,
};
const envelope = obs => ({ apiVersion: 1, status: "ready", revision: "r1", observation: obs });
const view = obs => new PilotView(envelope({ ...observation, ...obs }), { cards: [] });

test("pilot collections keep one shape at zero, one, many and paged counts", () => {
  for (const count of [0, 1, 2, 101]) {
    const actions = Array.from({ length: count }, (_, index) => ({
      index, type: "CastSpell", card: 9, targets: [{ type: "player", seat: "p2" }],
      modeIds: [index], choices: { manaPayment: { future: false } }, label: "canonical label", ticket: "canonical ticket",
    }));
    const formatted = view({ legalActions: actions });
    const choices = formatted.packet.position.choices;
    assert.deepEqual(Object.keys(choices), ["items", "total", "reference"]);
    assert.equal(choices.total, count);
    const items = [...choices.items];
    if (choices.reference) {
      let offset = 0;
      do {
        const page = formatted.inspect(choices.reference, { offset });
        items.push(...page.items);
        offset = page.nextOffset;
      } while (offset !== null);
    }
    assert.deepEqual(items.map(item => item.action), actions);
    assert.equal(new Set(items.map(item => item.ticket)).size, count);
    if (count) assert.match(items[0].label, /target p2; modeIds: \[0\]; choices.manaPayment:/);
    if (choices.reference) assert.equal(formatted.inspect(choices.reference, { actionType: "CastSpell" }).total, count);
    validatePilot(formatted.packet);
  }
});

test("pilot retains nested zones, unknown fields and exact selection schema", () => {
  const options = Array.from({ length: 101 }, (_, id) => ({ id, future: { art: false } }));
  const schema = { id: 19, minimum: 0, maximum: 101, cancellable: true, future: null };
  const extra = { hand: [{ objectId: 4, name: "A", art: { scryfallId: "x", artist: "Y" } }],
    decision: { ...schema, options }, legalActions: [{ index: 0, type: "ChooseDecision", decision: 19 }],
    future: { null: null, empty: [], false: false, zero: 0 } };
  const formatted = view(extra), position = formatted.packet.position;
  assert.deepEqual(position.zones.hand.items, extra.hand);
  assert.deepEqual(position.zones.graveyards.items, [[], []]);
  assert.equal(position.zones.exiles, null);
  assert.deepEqual(position.facts.future, extra.future);
  assert.deepEqual(position.decision.schema, schema);
  assert.deepEqual(formatted.inspect(position.decision.options.reference).items, options.slice(0, 100));
});

test("native labels distinguish abilities and payment without changing exact actions", () => {
  const legalActions = [0, 1].map(index => ({ index, type: "ActivateAbility", source: 4,
    ability: { kind: "card", ability: index }, modeIds: [], x: null,
    manaPayment: { units: [index] }, targets: [{ type: "player", seat: "p1" }] }));
  const choices = view({ legalActions }).packet.position.choices.items;
  assert.notEqual(choices[0].label, choices[1].label);
  assert.match(choices[0].label, /target p1; ability:/);
  assert.doesNotMatch(choices[0].label, /modeIds|x: null/);
  assert.deepEqual(choices.map(choice => choice.action), legalActions);
});

test("pilot statuses have a discriminant and retain host receipts and yield state", () => {
  for (const status of ["waiting", "complete", "paused"]) {
    const host = { apiVersion: 1, status, receipt: { requestId: "x", accepted: true },
      yield: { requestId: "y", status: "stopped", until: [{ type: "opponentActs" }],
        maxPasses: 64, passes: 2, stopReason: "condition", conditionIndex: 0 } };
    const packet = new PilotView(host).packet;
    assert.equal(packet.status, status);
    assert.equal(packet.revision, null);
    assert.equal(packet.position, null);
    assert.deepEqual(packet.host, host);
  }
  const complete = new PilotView({ ...envelope(observation), status: "complete" }).packet;
  assert.ok(complete.position);
  assert.throws(() => new PilotView({ apiVersion: 1, status: "ready" }), /PilotPacket\.revision: expected string/);
  assert.throws(() => new PilotView({ ...envelope(observation), status: "waiting" }), /PilotPacket\.position: expected null/);
});

test("runtime contract rejects malformed known fields and inconsistent totals", () => {
  assert.throws(() => view({ life: [20] }), /players\.life: invalid item count/);
  assert.throws(() => view({ seat: "p3" }), /seat: expected one of/);
  assert.throws(() => view({ step: "Invented" }), /step: expected one of/);
  const packet = view({}).packet;
  packet.position.choices.total = 1;
  assert.throws(() => validatePilot(packet), /total/);
  assert.throws(() => validateContract({ requestId: "x", revision: "r", until: [] }, "YieldRequest"), /item count/);
  assert.throws(() => validateContract({ requestId: "x", revision: "r", until: [{ type: "nextStep", step: "End" }], maxPasses: 257 }, "YieldRequest"), /bounds/);
});

test("schema is available without credentials and declarations derive from it", async () => {
  assert.deepEqual(await nativeCommand("schema", { presentation: "pilot-v1" }), pilotSchema);
  assert.equal(readFileSync(new URL("./pilot.d.ts", import.meta.url), "utf8"), declarations());
  assert.match(declarations(), /"life"\?: \[number, number\]/);
  assert.match(declarations(), /"manaPools"\?: \[ManaPool, ManaPool\]/);
  validateContract({ requestId: "a", accepted: true, yield: null }, "Receipt");
  validateContract({ requestId: "b", accepted: 2, updates: [], stopped: "waiting" }, "Receipt");
  assert.throws(() => validateContract({ requestId: "a", accepted: -1 }, "Receipt"), /Receipt\.accepted: number out of bounds/);
  assert.throws(() => validateContract({ requestId: "x".repeat(129) }, "CancelYieldRequest"), /too long/);
});

test("native yield and cancellation validate before I/O and retain exact retries", async () => {
  const credentials = { server: "http://localhost:41234", secret: "own-seat" };
  const calls = [];
  const fetcher = async (_url, options) => {
    calls.push(JSON.parse(options.body));
    return Response.json({ apiVersion: 1, status: "waiting" });
  };
  const request = { ...credentials, requestId: "yield-1", revision: "r",
    until: [{ type: "nextStep", step: "End", activeSeat: "opponent" }, { type: "opponentActs" }], maxPasses: 64 };
  await nativeCommand("yield", request, fetcher);
  await nativeCommand("yield", request, fetcher);
  assert.deepEqual(calls[0], calls[1]);
  await nativeCommand("cancelYield", { ...credentials, requestId: "cancel-1" }, fetcher);
  assert.equal(calls[2].method, "cancelYield");
  await assert.rejects(nativeCommand("cancelYield", credentials, fetcher), /requestId/);
  await assert.rejects(nativeCommand("cancelYield", { ...credentials, requestId: "é".repeat(65) }, fetcher), /128 UTF-8 bytes/);
  await assert.rejects(nativeCommand("yield", { ...request, until: [{ type: "nextStep", step: "Invalid" }] }, fetcher), /until\[0\]\.step: expected one of/);
  assert.equal(calls.length, 3);
});

test("catalog outages cannot change paged choice references", () => {
  const observed = { ...observation, hand: [{ definition: "printed", name: "A" }],
    legalActions: Array.from({ length: 101 }, (_, index) => ({ type: "PassPriority", index })) };
  const withCatalog = new PilotView(envelope(observed), { cards: [{ definition: "printed", name: "A" }] });
  const withoutCatalog = new PilotView(envelope(observed), null);
  const reference = withCatalog.packet.position.choices.reference;
  assert.equal(reference, withoutCatalog.packet.position.choices.reference);
  assert.deepEqual(withCatalog.inspect(reference), withoutCatalog.inspect(reference));
  assert.equal(withoutCatalog.packet.position.rules.items[0].available, false);
});

test("partial catalog availability cannot retarget card references", () => {
  const cards = [{ definition: "first", name: "A" }, { definition: "second", name: "B" }];
  const observed = { ...observation, hand: cards };
  const full = new PilotView(envelope(observed), { cards });
  const partial = new PilotView(envelope(observed), { cards: cards.slice(1) });
  const [first, second] = full.packet.position.rules.items;
  assert.equal(partial.inspect(second.reference).items[0].definition, "second");
  assert.throws(() => partial.inspect(first.reference), /unknown or expired/);
  assert.throws(() => partial.inspect("r1:r0"), /unknown or expired/);
});

test("decision-v1 compatibility and exact bypass remain explicit native options", async () => {
  const credentials = { server: "http://localhost:41234", secret: "own-seat" };
  const host = envelope(observation);
  const fetcher = async (_url, options) => Response.json(JSON.parse(options.body).method === "catalog" ? { cards: [] } : host);
  assert.equal((await nativeCommand("observe", { ...credentials, presentation: "decision-v1" }, fetcher)).presentation, "decision-v1");
  assert.deepEqual(await nativeCommand("observe", { ...credentials, presentation: "exact" }, fetcher), host);
});
