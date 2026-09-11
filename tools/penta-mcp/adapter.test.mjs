import assert from "node:assert/strict";
import { createServer as httpServer } from "node:http";
import { fileURLToPath } from "node:url";
import test from "node:test";
import { Client } from "@modelcontextprotocol/client";
import { StdioClientTransport } from "@modelcontextprotocol/client/stdio";
import { SessionClient } from "./client.mjs";
import { changes, page, playingObservation, present } from "./views.mjs";

const observation = {
  seat: "p1", phase: "Main1", decision: null, hand: [{ objectId: 9, name: "Mountain" }],
  legalActions: [{ index: 0, type: "PassPriority" }, { index: 1, type: "ActivateManaAbility", source: 7 }],
  checkpoint: { longReconstruction: "x".repeat(10000) },
};
const ready = { apiVersion: 1, status: "ready", role: "human", revision: "a", observation };

function patch(value, edits) {
  value = structuredClone(value);
  for (const edit of edits) {
    if (!edit.path.length) { value = structuredClone(edit.value); continue; }
    let target = value;
    for (const key of edit.path.slice(0, -1)) target = target[key];
    const key = edit.path.at(-1);
    if (edit.remove) delete target[key]; else target[key] = structuredClone(edit.value);
  }
  return value;
}

test("presentation changes roundtrip exact fields and ordered choices, including removal and null", () => {
  const states = [observation, { ...observation, phase: "Main2" },
    { seat: "p1", phase: "Main2", hand: [], decision: { id: 8, options: [{ id: 3 }, { id: 1 }] }, legalActions: [] },
    { ...observation, result: { winner: "p1" } }];
  for (const before of states) for (const after of states) {
    assert.deepEqual(patch(before, changes(before, after)), after);
  }
  const first = present(ready);
  assert.deepEqual(first.result.observation.legalActions, observation.legalActions);
  assert.equal("checkpoint" in first.result.observation, false);
  const waiting = present({ status: "waiting" }, first.previous);
  const next = present({ ...ready, revision: "b", observation: states[1] }, waiting.previous);
  assert.equal(next.result.baseRevision, "a");
  assert.deepEqual(patch(first.result.observation, next.result.changes), playingObservation(states[1]));
  assert.deepEqual(present(ready, next.previous, true).result.observation, playingObservation(observation));
});

test("array entries change compactly while preserving every value and position", () => {
  const before = Array.from({ length: 8 }, (_, i) => ({
    objectId: i + 20, definition: i + 200, name: `Permanent ${i}`,
    controller: i % 2 ? "p1" : "p2", tapped: false,
    power: 3, toughness: 3, counters: { charge: 1 },
  }));
  const tapped = structuredClone(before);
  tapped[3].tapped = true;
  const edits = changes({ battlefield: before }, { battlefield: tapped });
  assert.deepEqual(edits, [{ path: ["battlefield", "3", "tapped"], value: true }]);
  assert.deepEqual(patch({ battlefield: before }, edits), { battlefield: tapped });
  assert.ok(JSON.stringify(edits).length < JSON.stringify(tapped).length / 4);

  const removedField = structuredClone(tapped);
  delete removedField[5].counters;
  const states = [before, tapped, removedField, [...before].reverse(),
    [...before, { objectId: 99 }], before.slice(1), [], null,
    { selected: before[0] }, before.map(card => card.objectId)];
  for (const previous of states) for (const next of states) {
    assert.deepEqual(patch(previous, changes(previous, next)), next);
  }
  // Resizing must replace the array, never delete an index and leave a hole.
  assert.deepEqual(changes(before, before.slice(1)), [{ path: [], value: before.slice(1) }]);
});

test("large menus are explicit references and every option remains inspectable without renumbering", async () => {
  const legalActions = Array.from({ length: 250 }, (_, index) => ({ index, type: index % 2 ? "ActivateManaAbility" : "PlayLand", card: index }));
  const decision = { id: 3, minimum: 1, options: Array.from({ length: 250 }, (_, id) => ({ id, label: `Card ${id}` })) };
  const view = { ...ready, observation: { ...observation, legalActions, decision } };
  const client = new SessionClient("http://penta.test", async () => Response.json(view));
  const attached = await client.attach({ room: "r", token: "own-seat" });
  assert.equal(attached.observation.legalActions.count, 250);
  assert.equal(attached.observation.legalActions.types.ActivateManaAbility, 125);
  assert.equal(attached.observation.decision.options.count, 250);
  const reconstructed = [];
  for (let offset = 0; offset < 250; offset += 100) {
    reconstructed.push(...(await client.inspect({ connection: attached.connection, section: "legalActions", offset })).items);
  }
  assert.deepEqual(reconstructed, legalActions);
  const search = await client.inspect({ connection: attached.connection, section: "decision", query: "Card 249" });
  assert.deepEqual(search.items, [{ id: 249, label: "Card 249" }]);
  assert.equal(search.decision.id, 3);
  assert.deepEqual((await client.inspect({ connection: attached.connection, section: "checkpoint" })).checkpoint, observation.checkpoint);
});

test("inspection sizes its pages by payload as well as count and always advances", () => {
  const items = Array.from({ length: 10 }, (_, id) => ({ id, text: "x".repeat(10_000) }));
  const first = page(items, 0, 100);
  assert.equal(first.items.length, 2);
  assert.equal(first.nextOffset, 2);
  assert.deepEqual(page(items, first.nextOffset, 100).items, items.slice(2, 4));
  assert.equal(page([{ text: "x".repeat(30_000) }], 0, 100).nextOffset, null);
  assert.equal(playingObservation({ legalActions: items }).legalActions.count, 10);
});

test("public events and forced inspection information remain exact and pageable", async () => {
  const updates = Array.from({ length: 105 }, (_, id) => ({ type: "AutomaticDecision", decision: { id, prompt: "Look", options: [{ id: 0, members: [{ name: `Card ${id}` }] }] } }));
  const client = new SessionClient("http://penta.test", async () => Response.json({ ...ready, observation: { ...ready.observation, updates } }));
  const attached = await client.attach({ room: "r", token: "own-seat" });
  assert.deepEqual(attached.observation.updates, { count: 105, inspect: "updates" });
  const first = await client.inspect({ connection: attached.connection, section: "updates" });
  const rest = await client.inspect({ connection: attached.connection, section: "updates", offset: first.nextOffset });
  assert.deepEqual([...first.items, ...rest.items], updates);
});

test("uncertain plays retain the identical receipt; definite stale errors allow resynchronization", async () => {
  const posts = [];
  let failure = "network";
  const client = new SessionClient("http://penta.test", async (_url, init) => {
    if (!init.body) return Response.json(ready);
    posts.push(JSON.parse(init.body));
    if (failure === "network") throw new Error("connection lost after commit");
    if (failure === "stale") return Response.json({ error: "stale revision" }, { status: 409 });
    return Response.json({ ...ready, receipt: { requestId: posts.at(-1).requestId, accepted: 1 } });
  });
  const { connection } = await client.attach({ room: "r", token: "seat" });
  const move = { connection, revision: "a", choices: [{ index: 0 }], waitMs: 0 };
  await assert.rejects(client.play(move), /connection lost/);
  await assert.rejects(client.play(move), /uncertain outcome/);
  failure = null;
  await client.retry({ connection, waitMs: 0 });
  assert.deepEqual(posts[0], posts[1]);
  failure = "stale";
  await assert.rejects(client.play(move), /stale revision/);
  failure = null;
  assert.equal((await client.play(move)).receipt.accepted, 1);
});

test("human invitation maps either engine seat and contains only the human credential", async () => {
  for (const humanSeat of ["p1", "p2"]) {
    let config;
    const client = new SessionClient("http://penta.test", async (_url, init) => {
      config = JSON.parse(init.body);
      return Response.json({ humanToken: "human-secret", botToken: "bot-secret" });
    });
    const opened = await client.create({ format: "old-school-93-94", p1Deck: "Sligh", p2Deck: "The Deck", matchMode: "first-to-two-wins", humanSeat });
    assert.equal(config.humanDeck, humanSeat === "p1" ? "Sligh" : "The Deck");
    assert.equal(config.sessionApi, true);
    assert.equal(opened.seats[humanSeat].token, "human-secret");
    const url = new URL(opened.humanUrl);
    assert.equal(new URLSearchParams(url.hash.slice(1)).get("seatToken"), "human-secret");
    assert.equal(opened.humanUrl.includes("bot-secret"), false);
  }
});

test("stdio MCP handshake exposes usable schemas and a single compact tool result", async () => {
  const host = httpServer((request, response) => {
    assert.equal(request.url, "/_engine/options");
    response.setHeader("content-type", "application/json");
    response.end(JSON.stringify({ apiVersion: 1, formats: [{ id: "old-school-93-94", decks: ["Sligh"] }] }));
  });
  await new Promise(resolve => host.listen(0, "127.0.0.1", resolve));
  const transport = new StdioClientTransport({
    command: process.execPath, args: [fileURLToPath(new URL("server.mjs", import.meta.url))],
    env: { PENTA_SERVER_URL: `http://127.0.0.1:${host.address().port}` }, stderr: "pipe",
  });
  const client = new Client({ name: "penta-test", version: "1" });
  try {
    await client.connect(transport);
    const listed = await client.listTools();
    assert.deepEqual(listed.tools.map(tool => tool.name).sort(), ["attach", "inspect", "next", "options", "play", "retry", "start_match"]);
    const result = await client.callTool({ name: "options", arguments: {} });
    assert.equal(result.isError, undefined);
    assert.equal(result.content.length, 1);
    assert.equal(result.structuredContent, undefined, "do not duplicate a large observation in two result fields");
    assert.equal(JSON.parse(result.content[0].text).formats[0].decks[0], "Sligh");
    const bad = await client.callTool({ name: "next", arguments: { connection: "missing", waitMs: 0 } });
    assert.equal(bad.isError, true);
    assert.match(bad.content[0].text, /unknown connection/);
  } finally {
    await client.close();
    await new Promise(resolve => host.close(resolve));
  }
});
