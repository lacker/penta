import assert from "node:assert/strict";
import { createServer } from "node:http";
import { execFile } from "node:child_process";
import { promisify } from "node:util";
import { mkdtemp, readFile, rm, stat } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { setTimeout as delay } from "node:timers/promises";
import test from "node:test";
import { Backend } from "./backend.mjs";

const exec = promisify(execFile);
const cli = fileURLToPath(new URL("cli.mjs", import.meta.url));
const ready = revision => ({ apiVersion: 1, status: "ready", revision, observation: {
  seat: "p1", phase: "Main1", decision: null,
  legalActions: [{ index: 7, type: "PlayLand", card: 22 }, { index: 9, type: "PassPriority" }],
  checkpoint: { exact: "data" },
} });
const rows = table => Array.isArray(table) ? table : table.rows.map(row => ({ ...table.shared, ...row }));

test("CLI preserves tickets across commands and recovers a committed move after daemon crash", async () => {
  const directory = await mkdtemp(path.join(tmpdir(), "penta-cli-"));
  let revision = "a", fail = false, commits = 0, starts = 0;
  const posts = [], receipts = new Map();
  const host = createServer(async (request, response) => {
    if (request.url === "/_engine/options") return response.end(JSON.stringify({ apiVersion: 1, formats: [{ id: "test", decks: ["A", "B"] }] }));
    if (request.url.endsWith("/start")) {
      starts++;
      let text = ""; for await (const chunk of request) text += chunk;
      if (JSON.parse(text).humanDeck === "Drop") { request.socket.destroy(); return; }
      return response.end(JSON.stringify({ humanToken: "human-secret", botToken: "agent-secret" }));
    }
    assert.equal(request.headers["x-penta-token"], "agent-secret");
    if (request.url.endsWith("/catalog")) return response.end(JSON.stringify({ cards: [] }));
    let view = ready(revision);
    if (request.url.includes("/play?")) {
      let text = ""; for await (const chunk of request) text += chunk;
      const body = JSON.parse(text); posts.push(body);
      if (!receipts.has(body.requestId)) { receipts.set(body.requestId, body); commits++; }
      revision = "b";
      view = { ...ready(revision), receipt: { accepted: body.choices.length, requestId: body.requestId } };
      if (fail) { request.socket.destroy(); return; }
    }
    response.end(JSON.stringify(view));
  });
  await new Promise(resolve => host.listen(0, "127.0.0.1", resolve));
  const env = { ...process.env, PENTA_AGENT_DIR: directory, PENTA_SERVER_URL: `http://127.0.0.1:${host.address().port}` };
  const run = async (command, args = {}) => JSON.parse((await exec(process.execPath, [cli, command, JSON.stringify(args)], { env, cwd: tmpdir() })).stdout);
  try {
    // Racing independent commands must share one session owner.
    const statuses = await Promise.all([run("status"), run("status")]);
    assert.equal(statuses[0].pid, statuses[1].pid);
    const address = JSON.parse(await readFile(path.join(directory, "daemon.json"), "utf8"));
    assert.equal((await fetch(`http://127.0.0.1:${address.port}`, { method: "POST", body: '{}' })).status, 403);
    assert.equal((await stat(path.join(directory, "daemon.json"))).mode & 0o777, 0o600);
    assert.equal((await run("up")).status, "ready");
    assert.equal((await run("options")).formats[0].id, "test");
    const setup = { requestId: "one-game", format: "test", p1Deck: "A", p2Deck: "B", humanSeat: "p2" };
    const racing = await Promise.allSettled([run("start_match", setup), run("start_match", setup)]);
    const created = racing.find(result => result.status === "fulfilled").value;
    assert.deepEqual(await run("start_match", setup), created);
    assert.equal(starts, 1);
    const lost = { ...setup, requestId: "lost-create", p2Deck: "Drop" };
    await assert.rejects(run("start_match", lost), /fetch failed/);
    await assert.rejects(run("start_match", lost), /outcome is uncertain/);
    assert.equal(starts, 2, "an uncertain creation must never automatically redeal");
    await assert.rejects(run("start_match", { ...setup, p1Deck: "B" }), /different arguments/);
    assert.equal(created.seats.p1.token, "agent-secret");
    assert.match(created.humanUrl, /human-secret/);
    const first = await run("attach", created.seats.p1);
    assert.equal(first.presentation, "decision-v1");
    assert.deepEqual((await run("inspect_ref", { reference: first.provenance.reference })).items[0].checkpoint, { exact: "data" });
    await assert.rejects(run("next", { connection: first.connection, waitMs: -1 }), /waitMs/);
    await assert.rejects(run("constructor"), /unknown command/);
    const ticket = rows(first.choices)[0].ticket;
    fail = true;
    await assert.rejects(run("choose", { ticket, waitMs: 0 }), /fetch failed/);
    assert.equal(commits, 1);
    const saved = JSON.parse(await readFile(path.join(directory, "sessions.json"), "utf8"));
    assert.deepEqual(saved.sessions[0][1].pending, posts[0]);
    process.kill(statuses[0].pid, "SIGKILL");
    for (let i = 0; i < 100; i++) {
      try { process.kill(statuses[0].pid, 0); await delay(20); } catch { break; }
    }
    fail = false;
    assert.equal((await run("retry", { connection: first.connection, waitMs: 0 })).receipt.accepted, 1);
    assert.deepEqual(posts[0], posts[1]);
    assert.equal(commits, 1);
    // Recover a response lost between daemon and CLI even after known success.
    await run("retry", { connection: first.connection, waitMs: 0 });
    assert.equal(commits, 1);
    const next = await run("next", { connection: first.connection, full: true, waitMs: 0 });
    const batch = await run("play", { connection: first.connection, revision: next.revision, choices: rows(next.choices).map(({ ticket }) => ({ ticket })), waitMs: 0 });
    assert.equal(batch.receipt.accepted, 2);
    assert.deepEqual(posts.at(-1).choices, [{ action: { type: "PlayLand", card: 22 } }, { action: { type: "PassPriority" } }]);
    assert.deepEqual(await run("start_match", setup), created);
    assert.equal(starts, 2);
  } finally {
    try { await run("stop"); } catch {}
    host.closeAllConnections();
    await new Promise(resolve => host.close(resolve));
    await rm(directory, { recursive: true, force: true });
  }
});

test("backend readiness starts local hosting once and never launches for an explicit URL", async () => {
  let available = false, launches = 0;
  const backend = new Backend("http://unused.test", { remote: false, launch: () => { launches++; } });
  backend.ready = async () => available;
  assert.equal((await backend.up(0)).status, "starting");
  assert.equal((await backend.up(0)).status, "starting");
  assert.equal(launches, 1);
  available = true;
  assert.equal((await backend.up(0)).status, "ready");
  const remote = new Backend("https://unused.test", { remote: true, launch: () => { throw new Error("must not launch"); } });
  remote.ready = async () => false;
  await assert.rejects(remote.up(0), /disables local startup/);
});
