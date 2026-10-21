import assert from "node:assert/strict";
import test from "node:test";
import { createNativeClient, nativeCommand } from "./native-client.mjs";

const credentials = { server: "http://127.0.0.1:41234", secret: "seat-only", presentation: "pilot-v2" };
const observation = { protocolVersion: 33, seat: "p1", hand: [], battlefield: [], legalActions: [{ index: 7, type: "PassPriority" }], checkpoint: { marker: "private seat checkpoint" } };
const ready = { apiVersion: 1, status: "ready", revision: "revision-one", observation };
function mock(handler) {
  const calls = [];
  return {
    calls,
    fetch: async (url, options) => {
      const body = JSON.parse(options.body);
      calls.push({ url, headers: options.headers, ...body });
      return Response.json(await handler(body.method, body.args));
    },
  };
}

test("fresh invocations reproduce revision tickets and inspect references without a registry", async () => {
  const transport = mock(method => method === "catalog" ? { cards: [] } : ready);
  const first = await nativeCommand("observe", credentials, transport.fetch);
  const second = await nativeCommand("observe", credentials, transport.fetch);
  assert.deepEqual(first, second);
  assert.equal(first.presentation, "pilot-v2");
  assert.equal(first.position.choices.items[0].ticket, "revision-one:a7");
  const details = await nativeCommand("inspect_ref", { ...credentials, reference: first.position.provenance.reference }, transport.fetch);
  assert.deepEqual(details.items[0].checkpoint, observation.checkpoint);
  assert.ok(transport.calls.every(call => call.headers.authorization === "Bearer seat-only"));
  assert.ok(transport.calls.every(call => !Object.hasOwn(call.args, "secret")));
});

test("choose retries send the exact caller request without observing or generating credentials", async () => {
  const transport = mock(() => ({ apiVersion: 1, status: "waiting", receipt: { accepted: 1 } }));
  const input = { ...credentials, ticket: "revision-one:a7", requestId: "move-17" };
  await nativeCommand("choose", input, transport.fetch);
  await nativeCommand("choose", input, transport.fetch);
  assert.equal(transport.calls.length, 2);
  assert.deepEqual(transport.calls[0], transport.calls[1]);
  assert.deepEqual(transport.calls[0].args, { ticket: "revision-one:a7", requestId: "move-17" });
});

test("catalog failure preserves a playable typed packet and committed receipt", async () => {
  const committed = { ...ready, receipt: { requestId: "m", accepted: 1 } };
  const transport = mock(method => {
    if (method === "catalog") throw new Error("catalog unavailable");
    return committed;
  });
  const result = await nativeCommand("choose", { ...credentials, ticket: "old:a7", requestId: "m" }, transport.fetch);
  assert.deepEqual(result.host.receipt, committed.receipt);
  assert.equal(result.presentation, "pilot-v2");
  assert.match(result.host.presentationWarning, /catalog unavailable/);
  assert.deepEqual(result.position.choices.items[0].action, observation.legalActions[0]);
  assert.equal(result.position.choices.items[0].ticket, "revision-one:a7");
});

test("unsupported observations retain exact committed state as a last resort", async () => {
  const committed = { ...ready, observation: { ...observation, protocolVersion: 999 },
    receipt: { requestId: "m", accepted: 1 } };
  const transport = mock(method => method === "catalog" ? { cards: [] } : committed);
  const result = await nativeCommand("choose", { ...credentials, ticket: "old:a7", requestId: "m" }, transport.fetch);
  assert.equal(result.presentation, "exact");
  assert.deepEqual(result.observation, committed.observation);
  assert.deepEqual(result.receipt, committed.receipt);
});

test("exact mode bypasses presentation and catalog lookup", async () => {
  const transport = mock(() => ready);
  assert.deepEqual(await nativeCommand("observe", { ...credentials, presentation: "exact" }, transport.fetch), ready);
  assert.deepEqual(transport.calls.map(call => call.method), ["observe"]);
});

test("legacy pilot presentation and schema remain explicitly selectable", async () => {
  const transport = mock(method => method === "catalog" ? { cards: [] } : ready);
  const legacy = await nativeCommand("observe", { ...credentials, presentation: "pilot-v1" }, transport.fetch);
  assert.equal(legacy.presentation, "pilot-v1");
  const currentSchema = await nativeCommand("schema", {});
  const legacySchema = await nativeCommand("schema", { presentation: "pilot-v1" });
  assert.notDeepEqual(currentSchema, legacySchema);
  await assert.rejects(nativeCommand("schema", { presentation: "exact" }), /schema presentation/);
});

test("organizer status and exports remain raw without presentation warnings", async () => {
  for (const method of ["status", "export"]) {
    const result = { status: "complete", nativeReplayVersion: 1, future: {} };
    const transport = mock(() => result);
    assert.deepEqual(await nativeCommand(method, credentials, transport.fetch), result);
    assert.deepEqual(transport.calls.map(call => call.method), [method]);
  }
});

test("stale references never inspect a new revision", async () => {
  const transport = mock(() => ({ ...ready, revision: "revision-two" }));
  await assert.rejects(nativeCommand("inspect_ref", { ...credentials, reference: "revision-one:r0" }, transport.fetch), /stale reference/);
  assert.deepEqual(transport.calls.map(call => call.method), ["discover"]);
});

test("request IDs, bounded waits and origin-only servers are validated before sending", async () => {
  const transport = mock(() => ready);
  await assert.rejects(nativeCommand("choose", { ...credentials, ticket: "r:a0" }, transport.fetch), /requestId/);
  await assert.rejects(nativeCommand("wait", { ...credentials, waitMs: 25001 }, transport.fetch), /waitMs/);
  await assert.rejects(nativeCommand("observe", { ...credentials, server: "http://user:password@localhost/" }, transport.fetch), /origin/);
  assert.equal(transport.calls.length, 0);
});

test("transport errors propagate without replacing the logical request", async () => {
  const input = { ...credentials, requestId: "retained", ticket: "r:a7" };
  await assert.rejects(nativeCommand("choose", input, async () => { throw new Error("connection lost"); }), /connection lost/);
  const transport = mock(() => ({ status: "waiting", receipt: { requestId: "retained", accepted: 1 } }));
  await nativeCommand("choose", input, transport.fetch);
  assert.equal(transport.calls[0].args.requestId, "retained");
});

test("displayed exact actions round-trip with only top-level action indices removed", async () => {
  const action = { index: 7, type: "ActivateAbility", abilityIndex: 2, target: { index: 4 }, future: "retained" };
  const transport = mock(method => method === "catalog" ? { cards: [] }
    : method === "observe" ? { ...ready, observation: { ...observation, legalActions: [action] } }
      : { status: "waiting" });
  const displayed = await nativeCommand("observe", credentials, transport.fetch);
  const input = { ...credentials, presentation: "exact", revision: ready.revision, requestId: "exact-retry",
    choices: [{ action: displayed.position.choices.items[0].action }, { decision: 42, options: [2, 1] }] };
  const original = structuredClone(input);
  Object.freeze(input.choices[0].action);
  Object.freeze(input.choices[0]);
  Object.freeze(input.choices);
  Object.freeze(input);
  let uncertainBody;
  await assert.rejects(nativeCommand("play", input, async (_url, options) => {
    uncertainBody = JSON.parse(options.body);
    throw new Error("lost response");
  }), /lost response/);
  await nativeCommand("play", input, transport.fetch);
  await nativeCommand("play", input, transport.fetch);
  const calls = transport.calls.filter(call => call.method === "play");
  assert.deepEqual(calls[0], calls[1]);
  assert.deepEqual(uncertainBody, { method: "play", args: calls[0].args });
  assert.deepEqual(calls[0].args.choices, [
    { action: { type: "ActivateAbility", abilityIndex: 2, target: { index: 4 }, future: "retained" } },
    { decision: 42, options: [2, 1] },
  ]);
  assert.deepEqual(input, original);
});

test("invalid exact action containers fail locally with an actionable path", async () => {
  const transport = mock(() => ready);
  for (const choices of [[], [null], [{ action: null }], [{ action: [] }]]) {
    await assert.rejects(nativeCommand("play", { ...credentials, requestId: "m", revision: "r", choices }, transport.fetch), /choices/);
  }
  assert.equal(transport.calls.length, 0);
});

test("native batch schema rejects hosted tickets and bare actions before contacting the host", async () => {
  const transport = mock(() => assert.fail("invalid shape must not contact host"));
  for (const choices of [[{ ticket: "r:a1" }], [{ type: "DeclareAttacker", attacker: 3 }],
    [{ action: { type: "PassPriority" }, index: 0 }], [{ decision: 1 }],
    [{ decision: 1, options: ["wrong"] }], [{ index: 0 }, { index: 1 }]]) {
    await assert.rejects(nativeCommand("play", { ...credentials, requestId: "m", revision: "r", choices }, transport.fetch),
      /choices\[0\].*(action|indices)/);
  }
  assert.equal(transport.calls.length, 0);
});

test("opaque v2 references remain stale across revisions and inspect exact current details", async () => {
  let revision = "revision-one";
  const transport = mock(method => method === "catalog" ? { cards: [] } : { ...ready, revision });
  const view = await nativeCommand("observe", credentials, transport.fetch);
  const reference = view.position.reference;
  assert.match(reference, /^p2r:[A-Za-z0-9_-]{43}$/);
  const details = await nativeCommand("inspect_ref", { ...credentials, reference }, transport.fetch);
  assert.deepEqual(details.items, [observation]);
  revision = "revision-two";
  await assert.rejects(nativeCommand("inspect_ref", { ...credentials, reference }, transport.fetch), /stale reference/);
});


test("persistent clients cache successful public catalogs per client and retry failures", async () => {
  let attempts = 0;
  const transport = mock(method => {
    if (method !== "catalog") return ready;
    if (++attempts === 1) throw new Error("unavailable");
    return { cards: [] };
  });
  const client = createNativeClient(transport.fetch);
  await client("observe", credentials);
  await client("observe", credentials);
  await client("observe", credentials);
  assert.equal(attempts, 2);
  assert.deepEqual(client.stats.callsByMethod, { observe: 3, catalog: 2 });
  assert.ok(client.stats.requestBytes > 0);
  assert.ok(client.stats.responseJsonBytes > 0);
  await createNativeClient(transport.fetch)("observe", credentials);
  assert.equal(attempts, 3);
});

test("interact canonicalizes explicit decision submissions and never retries uncertain writes", async () => {
  const transport = mock(() => ({ status: "waiting" }));
  const input = { ...credentials, requestId: "interaction", revision: "r", choices: [
    { action: { type: "ChooseDecision", decision: 7, options: [2, 3], index: 4 } },
    { action: { type: "PassPriority", index: 9 } },
  ] };
  await nativeCommand("interact", input, transport.fetch);
  assert.deepEqual(transport.calls[0].args.choices, [{ decision: 7, options: [2, 3] }, { action: { type: "PassPriority" } }]);
  assert.equal(input.choices[0].action.index, 4);
  for (const choices of [[], [{ index: 0 }], [{ action: { type: "ChooseDecision", decision: 7 } }]]) {
    await assert.rejects(nativeCommand("interact", { ...input, choices }, transport.fetch));
  }
  let calls = 0;
  await assert.rejects(nativeCommand("interact", input, async () => { calls++; throw new Error("lost"); }), /lost/);
  assert.equal(calls, 1);
});


test("persistent catalog cache resets when its origin changes", async () => {
  const transport = mock(method => method === "catalog" ? { cards: [] } : ready);
  const client = createNativeClient(transport.fetch);
  await client("observe", credentials);
  await client("observe", credentials);
  await client("observe", { ...credentials, server: "http://127.0.0.1:41235" });
  await client("observe", credentials);
  assert.equal(transport.calls.filter(call => call.method === "catalog").length, 3);
});

test("interact rejects decision fields it cannot preserve before sending", async () => {
  const transport = mock(() => ({ status: "ready" }));
  const action = { type: "ChooseDecision", decision: 7, options: [2] };
  for (const choice of [{ action, future: true }, { action: { ...action, future: true } }]) {
    await assert.rejects(nativeCommand("interact", { ...credentials, requestId: "m", revision: "r", choices: [choice] }, transport.fetch), /unsupported fields/);
  }
  assert.equal(transport.calls.length, 0);
});

test("v2 event delivery replaces snapshot updates without losing duplicates or paging", async () => {
  const items = Array.from({ length: 150 }, () => ({ type: "same-event", text: "duplicate occurrences are meaningful" }));
  const transport = mock(method => method === "catalog" ? { cards: [] } : {
    ...ready, observation: { ...observation, updates: [{ type: "old-snapshot" }] }, events: { after: 7, next: 157, items },
  });
  const result = await nativeCommand("observe", { ...credentials, afterEvent: 7 }, transport.fetch);
  assert.deepEqual(result.position.updates, { items, total: 150, reference: null });
  assert.deepEqual(result.host.eventDelivery, { after: 7, next: 157 });
  assert.equal(Object.hasOwn(result.host, "events"), false);
  const canonical = await nativeCommand("inspect_ref", { ...credentials, reference: result.position.reference }, transport.fetch);
  assert.deepEqual(canonical.items[0].updates, [{ type: "old-snapshot" }]);
  assert.deepEqual(result.position.updates.items, items);
});

test("waiting v2 responses retain delivered events without a position", async () => {
  const events = { after: 0, next: 2, items: [{ type: "one" }, { type: "two" }] };
  const transport = mock(() => ({ apiVersion: 1, status: "waiting", events }));
  const result = await nativeCommand("wait", { ...credentials, afterEvent: 0 }, transport.fetch);
  assert.deepEqual(result.host.eventDelivery, events);
  assert.equal(result.position, null);
  assert.equal(Object.hasOwn(result.host, "events"), false);
  const exact = await nativeCommand("wait", { ...credentials, presentation: "exact", afterEvent: 0 }, transport.fetch);
  assert.deepEqual(exact.events, events);
});

test("event cursor does not invalidate interaction contract validation", async () => {
  const transport = mock(() => ({ status: "waiting" }));
  await nativeCommand("interact", { ...credentials, requestId: "m", revision: "r", choices: [{ action: { type: "PassPriority" } }], afterEvent: 0 }, transport.fetch);
  assert.equal(transport.calls[0].args.afterEvent, 0);
  await assert.rejects(nativeCommand("interact", { ...credentials, requestId: "m", revision: "r", choices: [], afterEvent: -1 }, transport.fetch), /afterEvent/);
});
